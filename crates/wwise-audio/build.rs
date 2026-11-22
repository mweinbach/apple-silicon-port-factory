//! Generates the Wwise Vorbis packed-codebook tables from vgmstream's
//! `src/coding/libs/vorbis_codebooks_wwise.h` (optional `third_party/vgmstream` checkout) at
//! build time, so the codebook data itself is never copied into this repository.
//!
//! The header contains `static const uint8_t vcb_<lib>_<n>[] = { ... };` arrays plus two index
//! lists `vcb_list_standard[]` / `vcb_list_aotuv603[]` of `{id, size, array}` entries. Those are the
//! Wwise SDK codebook libraries ("packed_codebooks.bin" / "packed_codebooks_aoTuV_603.bin" as
//! extracted by hcs's ww2ogg). `wwise-audio decode` validates the result against the game's data.
//!
//! If `third_party/vgmstream` has not been fetched yet (via `./scripts/third_party.sh`), empty
//! tables are generated and Vorbis decoding reports a clear error at runtime instead of failing
//! the build.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let header = manifest
        .join("../../third_party/vgmstream/src/coding/libs/vorbis_codebooks_wwise.h");
    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed=build.rs");

    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("wwise_codebooks.rs");
    let src = std::fs::read_to_string(&header).unwrap_or_default();
    let mut code = String::new();
    if src.is_empty() {
        println!(
            "cargo:warning=wwise-audio: {} not found (run scripts/third_party.sh); Wwise Vorbis decoding disabled",
            header.display()
        );
        code.push_str("pub const AVAILABLE: bool = false;\n");
        code.push_str("pub static STANDARD: &[(u32, &[u8])] = &[];\n");
        code.push_str("pub static AOTUV603: &[(u32, &[u8])] = &[];\n");
        std::fs::write(out, code).unwrap();
        return;
    }

    // 1) byte arrays
    let mut arrays: HashMap<String, Vec<u8>> = HashMap::new();
    let mut rest = src.as_str();
    while let Some(p) = rest.find("static const uint8_t ") {
        rest = &rest[p + "static const uint8_t ".len()..];
        let name_end = rest.find('[').expect("array name");
        let name = rest[..name_end].trim().to_string();
        let open = rest.find('{').expect("array open");
        let close = rest.find("};").expect("array close");
        let body = &rest[open + 1..close];
        let bytes: Vec<u8> = body
            .split(',')
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| u8::from_str_radix(t.trim_start_matches("0x"), 16).expect("hex byte"))
            .collect();
        arrays.insert(name, bytes);
        rest = &rest[close + 2..];
    }

    // 2) index lists
    let parse_list = |list: &str| -> Vec<(u32, u32, String)> {
        let key = format!("vcb_info_t {}[] = {{", list);
        let start = src.find(&key).unwrap_or_else(|| panic!("{} not found", list)) + key.len();
        let end = start + src[start..].find("};").unwrap();
        src[start..end]
            .lines()
            .filter_map(|l| {
                let l = l.trim().trim_end_matches(',');
                let l = l.strip_prefix('{')?.strip_suffix('}')?;
                let mut it = l.split(',').map(|s| s.trim());
                let id = u32::from_str_radix(it.next()?.trim_start_matches("0x"), 16).ok()?;
                let size = u32::from_str_radix(it.next()?.trim_start_matches("0x"), 16).ok()?;
                let name = it.next()?.to_string();
                Some((id, size, name))
            })
            .collect()
    };

    code.push_str("pub const AVAILABLE: bool = true;\n");
    let mut emitted = std::collections::BTreeSet::new();
    for list in ["vcb_list_standard", "vcb_list_aotuv603"] {
        for (_, size, name) in parse_list(list) {
            let bytes = arrays.get(&name).unwrap_or_else(|| panic!("missing array {}", name));
            assert_eq!(bytes.len() as u32, size, "size mismatch for {}", name);
            if emitted.insert(name.clone()) {
                write!(code, "static {}: [u8; {}] = {:?};\n", name.to_uppercase(), bytes.len(), bytes)
                    .unwrap();
            }
        }
    }
    for (list, rust) in [("vcb_list_standard", "STANDARD"), ("vcb_list_aotuv603", "AOTUV603")] {
        write!(code, "pub static {}: &[(u32, &[u8])] = &[\n", rust).unwrap();
        for (id, _, name) in parse_list(list) {
            write!(code, "    ({:#06x}, &{}),\n", id, name.to_uppercase()).unwrap();
        }
        code.push_str("];\n");
    }
    std::fs::write(out, code).unwrap();
}
