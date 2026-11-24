//! Join real JIT output to the encoding forms on each case's declared Arm operation page.

use super::{assemble, verify_assembly, Cases, Instruction, Pattern};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::Path;

const CATALOG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../db/a64_encodings.tsv"
));
const HEADER: &str = "page\tencoding\tmnemonics\tpattern\texclude\tfields\tcondition\tpreferred";

struct Form<'a> {
    id: &'a str,
    mnemonics: &'a str,
    pattern: Pattern,
    exclude: Vec<Pattern>,
}

impl Form<'_> {
    fn matches(&self, word: u32) -> bool {
        self.pattern.matches(word) && !self.exclude.iter().any(|p| p.matches(word))
    }
}

type Catalog<'a> = HashMap<&'a str, Vec<Form<'a>>>;

fn load_catalog(text: &str) -> Result<Catalog<'_>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("unexpected A64 catalog header".into());
    }
    let mut pages = Catalog::new();
    let mut ids = HashSet::new();
    for (index, line) in lines.enumerate() {
        let at = || format!("A64 catalog line {}", index + 2);
        let fields: Vec<_> = line.split('\t').collect();
        let &[page, id, mnemonics, pattern, exclude, _, _, _] = fields.as_slice() else {
            return Err(format!("{}: expected eight fields", at()));
        };
        if fields.contains(&"") || !ids.insert(id) {
            return Err(format!("{}: empty field or duplicate encoding {id}", at()));
        }
        let parse =
            |bits| Pattern::parse(bits).ok_or_else(|| format!("{}: invalid pattern {bits}", at()));
        let pattern = parse(pattern)?;
        let exclude = if exclude == "-" {
            Vec::new()
        } else {
            exclude.split(',').map(parse).collect::<Result<_, _>>()?
        };
        pages.entry(page).or_default().push(Form {
            id,
            mnemonics,
            pattern,
            exclude,
        });
    }
    for id in &ids {
        if let Some((base, _)) = id.split_once('/') {
            if !ids.contains(base) {
                return Err(format!("alias {id} refers to missing encoding {base}"));
            }
        }
    }
    Ok(pages)
}

fn emitter_matches(catalog: &Catalog<'_>, words: &[Instruction]) -> Result<String, String> {
    let mut output = String::from("case_id\tcase_word\tword\tassembly\tpage\tencodings\n");
    for word in words {
        let mnemonic = word
            .asm
            .split_ascii_whitespace()
            .next()
            .ok_or("empty reference assembly")?;
        let forms = catalog
            .get(word.page.as_str())
            .ok_or_else(|| format!("{}: missing A64 operation page {}", word.case, word.page))?;
        let matches: Vec<_> = forms
            .iter()
            .filter(|form| form.matches(word.emitted))
            .filter(|form| {
                form.mnemonics
                    .split(',')
                    .any(|name| name.eq_ignore_ascii_case(mnemonic))
            })
            .map(|f| f.id)
            .collect();
        if matches.is_empty() {
            return Err(format!(
                "{}: {:#010x} ({}) matches no encoding on {}",
                word.case, word.emitted, word.asm, word.page
            ));
        }
        writeln!(
            output,
            "{}\t{}\t{:08x}\t{}\t{}\t{}",
            word.case_id,
            word.case,
            word.emitted,
            word.asm,
            word.page,
            matches.join(",")
        )
        .unwrap();
    }
    Ok(output)
}

#[test]
fn a64_emitter_matches() {
    let catalog = load_catalog(CATALOG).unwrap();
    // Every named instruction and alias in the pinned 2026-09 release, including unused forms.
    let forms = catalog.values().flatten();
    assert_eq!(
        forms.clone().filter(|form| form.id.contains('/')).count(),
        314
    );
    assert_eq!(forms.count(), 4429 + 314);
    assert_eq!(catalog.len(), 2351);
    let cases = Cases::registered();
    let output = emitter_matches(&catalog, &cases.words).unwrap();
    if let Some(path) = std::env::var_os("PORT_A64_MATCHES_OUT") {
        // Only export independently assembled words. A fresh path protects tracked fixtures.
        verify_assembly(&cases.words, &assemble(&cases.words)).unwrap();
        let mut file = fs::File::create_new(&path).unwrap_or_else(|e| panic!("{path:?}: {e}"));
        file.write_all(output.as_bytes()).unwrap();
    } else {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../db/a64_emitter_matches.tsv");
        let expected = fs::read_to_string(path).expect("read checked-in A64 emitter matches");
        let mut checked_in = expected.split_inclusive('\n');
        for (index, row) in output.split_inclusive('\n').enumerate() {
            assert_eq!(
                Some(row),
                checked_in.next(),
                "A64 emitter matches line {}",
                index + 1
            );
        }
        assert_eq!(
            checked_in.next(),
            None,
            "extra checked-in A64 emitter match"
        );
    }
    eprintln!(
        "a64_emitter_matches: {} cases, {} emitted words",
        cases.count,
        cases.words.len()
    );
}

#[test]
fn catalog_resolves_extension_forms_and_alias_constraints() {
    let catalog = load_catalog(CATALOG).unwrap();
    let matches = |page: &str, word| {
        catalog[page]
            .iter()
            .filter(|form| form.matches(word))
            .map(|form| form.id)
            .collect::<Vec<_>>()
    };
    // Clang-assembled SVE, SME and MOPS instructions outside the current Asm corpus.
    assert_eq!(matches("add_z_p_zz", 0x0480_08e3), ["add_z_p_zz_"]); // add z3.s, p2/m, z3.s, z7.s
    assert_eq!(
        matches("smopa_za_pp_zz", 0xa085_6881),
        ["smopa_za_pp_zz_32"]
    ); // smopa za1.s, p2/m, p3/m, z4.b, z5.b
    assert_eq!(matches("cpyfp", 0x1901_0440), ["CPYFP_CPY_memcms"]); // cpyfp [x0]!, [x1]!, x2!

    assert_eq!(
        matches("cset_csinc", 0x1a9f_17e3),
        ["CSINC_32_condsel/CSET"]
    ); // cset w3, eq
       // AL is valid in the base CSINC, but the CSET alias excludes cond = 111x.
    assert!(matches("cset_csinc", 0x1a9f_e7e3).is_empty());
    assert_eq!(matches("csinc", 0x1a9f_e7e3), ["CSINC_32_condsel"]);

    // This ORR is valid assembly for the same bytes as MOV. Labeling it as a MOV
    // case would give the checked-in match the wrong source mnemonic.
    let wrong_page = Instruction {
        case_id: 1,
        case: "orr[0]".into(),
        page: "mov_orr_advsimd_reg".into(),
        asm: "orr v27.16b, v19.16b, v19.16b".into(),
        emitted: 0x4eb3_1e7b,
    };
    assert!(emitter_matches(&catalog, &[wrong_page])
        .unwrap_err()
        .contains("matches no encoding"));
}
