//! Check each finished A64 word against its intended Arm page and independently assembled operands.

use super::super::emit::Asm;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

// Keep reference assembly beside the invocation. Cases with patching or multiword
// expansions use Cases::check directly so the whole sequence is verified.
macro_rules! check_words {
    ($cases:ident, $method:ident($($arg:expr),*) => $page:expr, $assembly:expr) => {
        check_words!($cases, stringify!($method); $method($($arg),*) => $page, $assembly;)
    };
    ($cases:ident, $name:expr; $($method:ident($($arg:expr),*) => $page:expr, $assembly:expr;)+) => {
        $cases.check($name, &[$(($page, $assembly)),+], |a| {
            $(a.$method($($arg),*);)+
        })
    };
}

mod catalog;
mod core_cases;
mod direct_cases;
mod extension_cases;
mod vector_cases;

#[derive(Default)]
struct Cases {
    count: usize,
    words: Vec<Instruction>,
}

struct Instruction {
    case_id: usize,
    case: String,
    page: String,
    asm: String,
    emitted: u32,
}

impl Cases {
    fn registered() -> Self {
        let mut cases = Self::default();
        core_cases::register(&mut cases);
        vector_cases::register(&mut cases);
        extension_cases::register(&mut cases);
        direct_cases::register(&mut cases);
        assert!(!cases.words.is_empty(), "empty opcode corpus");
        cases
    }

    fn check(&mut self, name: &str, expected: &[(&str, &str)], emit: impl FnOnce(&mut Asm)) {
        // Virtual addresses used for patching are independent of the ordinary RW storage.
        let mut storage = vec![0_u32; expected.len() + 1];
        let mut a = Asm::new(storage.as_mut_ptr(), 0x1000, storage.len());
        emit(&mut a);
        assert!(!a.overflow, "{name}: Asm overflow");
        self.record(name, expected, &storage[..a.pos]);
    }

    fn record(&mut self, name: &str, expected: &[(&str, &str)], words: &[u32]) {
        assert_eq!(words.len(), expected.len(), "{name}: emitted word count");
        for (index, (&(page, asm), &emitted)) in expected.iter().zip(words).enumerate() {
            self.words.push(Instruction {
                case_id: self.count + 1,
                case: format!("{name}[{index}]"),
                page: page.to_owned(),
                asm: asm.to_owned(),
                emitted,
            });
        }
        self.count += 1;
    }
}

#[derive(Clone, Copy)]
struct Pattern {
    mask: u32,
    value: u32,
}

impl Pattern {
    fn parse(text: &str) -> Option<Self> {
        if text.len() != 32 {
            return None;
        }
        let mut mask = 0;
        let mut value = 0;
        for bit in text.bytes() {
            mask <<= 1;
            value <<= 1;
            match bit {
                b'0' => mask |= 1,
                b'1' => {
                    mask |= 1;
                    value |= 1;
                }
                b'x' => {}
                _ => return None,
            }
        }
        (mask != 0).then_some(Self { mask, value })
    }

    fn matches(self, word: u32) -> bool {
        word & self.mask == self.value
    }
}

type Table = HashMap<String, Vec<Pattern>>;

fn page_stem(field: &str) -> Option<String> {
    let name = field.rsplit(['/', '\\']).next()?.to_ascii_lowercase();
    let stem = [".md", ".html", ".htm", ".xml"]
        .iter()
        .find_map(|ext| name.strip_suffix(ext))
        .unwrap_or(&name);
    let valid = !stem.is_empty()
        && stem
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    valid.then(|| stem.to_owned())
}

fn parse_table(text: &str) -> Result<Table, String> {
    let mut lines = text
        .trim_start_matches('\u{feff}')
        .lines()
        .enumerate()
        .peekable();
    let (_, first) = lines.peek().ok_or("empty encoding table")?;
    let fields: Vec<_> = first.split('\t').collect();
    let named = |names: &[&str]| {
        names
            .iter()
            .find_map(|name| fields.iter().position(|v| v.eq_ignore_ascii_case(name)))
    };
    let columns = match (
        named(&["page", "page_stem", "path", "file", "filename"]),
        named(&["pattern", "bit_pattern", "bits", "encoding"]),
    ) {
        (Some(page), Some(pattern)) => {
            lines.next();
            (page, pattern)
        }
        // Existing mirrors may use a headerless table with an instruction title and a page path.
        _ => {
            let candidates: Vec<_> = fields
                .iter()
                .enumerate()
                .filter(|(_, v)| Pattern::parse(v).is_some())
                .map(|(i, _)| i)
                .collect();
            let &[pattern] = candidates.as_slice() else {
                return Err("expected TSV columns for an Arm page and a 32-bit pattern".into());
            };
            let page = if fields.len() == 2 {
                1 - pattern
            } else {
                let paths: Vec<_> = fields
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.contains('/') || v.ends_with(".md") || v.ends_with(".html"))
                    .map(|(i, _)| i)
                    .collect();
                let &[page] = paths.as_slice() else {
                    return Err("expected one Arm page path in the headerless TSV".into());
                };
                page
            };
            (page, pattern)
        }
    };
    let mut table = Table::new();
    for (index, line) in lines {
        let fields_on_line: Vec<_> = line.split('\t').collect();
        let line_no = index + 1;
        if fields_on_line.len() != fields.len() {
            return Err(format!(
                "line {line_no}: expected {} TSV columns",
                fields.len()
            ));
        }
        let page = page_stem(fields_on_line[columns.0])
            .ok_or_else(|| format!("line {line_no}: invalid Arm page"))?;
        let pattern = Pattern::parse(fields_on_line[columns.1])
            .ok_or_else(|| format!("line {line_no}: invalid 32-bit [01x] pattern"))?;
        table.entry(page).or_default().push(pattern);
    }
    if table.is_empty() {
        return Err("no encoding patterns in table".into());
    }
    Ok(table)
}

fn assemble(words: &[Instruction]) -> String {
    // Labels associate each source instruction with exactly one output word, including branches.
    let mut source = String::new();
    for (index, word) in words.iter().enumerate() {
        writeln!(source, ".Lcase_{index}:\n\t{}", word.asm).unwrap();
    }
    let mut command = if cfg!(target_os = "macos") {
        let mut cmd = Command::new("xcrun");
        cmd.arg("clang");
        cmd
    } else {
        Command::new("clang")
    };
    let mut child = command
        .args([
            "-cc1as",
            "-triple",
            "aarch64-unknown-linux-gnu",
            "-target-cpu",
            "apple-m1",
            "-filetype",
            "asm",
            "-show-encoding",
            "-o",
            "-",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start clang integrated assembler");
    let mut stdin = child.stdin.take().unwrap();
    // Drain both output pipes while feeding the batch, which can exceed a pipe buffer.
    let (output, written) = std::thread::scope(|scope| {
        let writer = scope.spawn(move || stdin.write_all(source.as_bytes()));
        (
            child.wait_with_output().expect("wait for clang"),
            writer.join().unwrap(),
        )
    });
    assert!(
        output.status.success(),
        "clang {}:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    written.expect("send assembly to clang");
    String::from_utf8(output.stdout).expect("UTF-8 assembly listing")
}

fn clang_word(line: &str) -> Option<u32> {
    let (_, encoding) = line.split_once("// encoding:")?;
    let bytes = encoding.trim().strip_prefix('[')?.strip_suffix(']')?;
    let mut parts = bytes.split(',');
    let mut word = [0; 4];
    for byte in &mut word {
        let hex = parts.next()?.trim().strip_prefix("0x")?;
        if hex.len() != 2 {
            return None;
        }
        *byte = u8::from_str_radix(hex, 16).ok()?;
    }
    parts.next().is_none().then(|| u32::from_le_bytes(word))
}

fn verify_assembly(words: &[Instruction], listing: &str) -> Result<(), String> {
    let mut lines = listing.lines().map(str::trim).filter(|l| {
        l.starts_with(".Lcase_") || (!l.starts_with("//") && l.contains("// encoding:"))
    });
    for (index, word) in words.iter().enumerate() {
        let context = || format!("{} ({} / {:?})", word.case, word.page, word.asm);
        if lines.next() != Some(&format!(".Lcase_{index}:")) {
            return Err(format!(
                "{}: expected one labeled instruction from clang",
                context()
            ));
        }
        let expected = lines.next().and_then(clang_word).ok_or_else(|| {
            format!(
                "{}: expected four fully resolved bytes from clang",
                context()
            )
        })?;
        if word.emitted != expected {
            return Err(format!(
                "{}: emitted {:#010x}, clang assembled {expected:#010x}",
                context(),
                word.emitted
            ));
        }
    }
    if lines.next().is_some() {
        return Err("extra instruction in clang listing".into());
    }
    Ok(())
}

fn verify(table: &Table, words: &[Instruction], listing: &str) -> Result<(), String> {
    for word in words {
        let patterns = table
            .get(&word.page)
            .ok_or_else(|| format!("{}: missing Arm page {}", word.case, word.page))?;
        if !patterns.iter().any(|pattern| pattern.matches(word.emitted)) {
            return Err(format!(
                "{} ({:?}): emitted {:#010x} does not match the Arm page {}",
                word.case, word.asm, word.emitted, word.page
            ));
        }
    }
    verify_assembly(words, listing)
}

#[test]
fn a64_opcode_encodings() {
    let Some(path) = std::env::var_os("PORT_A64_ENCODINGS") else {
        eprintln!("skipping a64_opcode_encodings: PORT_A64_ENCODINGS is unset");
        return;
    };
    let path = Path::new(&path);
    let tsv = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("PORT_A64_ENCODINGS={path:?}: {e}"));
    let table = parse_table(&tsv).unwrap_or_else(|e| panic!("PORT_A64_ENCODINGS={path:?}: {e}"));
    let cases = Cases::registered();
    verify(&table, &cases.words, &assemble(&cases.words)).unwrap_or_else(|e| panic!("{e}"));
    eprintln!(
        "a64_opcode_encodings: verified {} cases ({} emitted words, {} pages in reference)",
        cases.count,
        cases.words.len(),
        table.len()
    );
}

#[test]
fn table_accepts_page_paths_and_rejects_malformed_rows() {
    let bits = format!("10{}", "x".repeat(30));
    for input in [
        format!("page\tpattern\nsample\t{bits}"),
        format!("path\ttitle\tpattern\nlatest_aarch64/sample.md\tSample\t{bits}"),
        format!("Sample\t{bits}\tlatest_aarch64/sample.html"),
    ] {
        let table = parse_table(&input).unwrap();
        assert!(table["sample"][0].matches(0x8000_0000));
        assert!(!table["sample"][0].matches(0x4000_0000));
    }
    for input in [
        "",
        "page\tpattern",
        "page\tpattern\nsample\t01x",
        "page\tpattern\nsample",
        "page\tpattern\nsample\txxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    ] {
        assert!(parse_table(input).is_err(), "{input:?}");
    }
}

#[test]
fn verifier_distinguishes_opcode_operand_and_instruction_count() {
    let tsv = format!(
        "page\tpattern\nsample\t1{}\nalias\t10{}",
        "x".repeat(31),
        "x".repeat(30)
    );
    let table = parse_table(&tsv).unwrap();
    let mut word = Instruction {
        case_id: 1,
        case: "synthetic".into(),
        page: "alias".into(),
        asm: "sample".into(),
        emitted: 0x8000_0001,
    };
    let listing = ".Lcase_0:\n sample // encoding: [0x01,0x00,0x00,0x80]";
    let check = |word: &Instruction, text: &str| verify(&table, std::slice::from_ref(word), text);
    assert!(check(&word, listing).is_ok());
    assert!(check(
        &word,
        &format!("{listing}\n extra // encoding: [0x01,0x00,0x00,0x80]")
    )
    .is_err());
    assert!(check(&word, ".Lcase_0:\n b // encoding: [A,A,A,0x14]").is_err());
    word.emitted ^= 1 << 30; // Valid for 'sample', outside the requested narrower 'alias' page.
    assert!(check(&word, listing)
        .unwrap_err()
        .contains("does not match the Arm page"));
    word.emitted = 0x8000_0002; // Same opcode with a different operand.
    assert!(check(&word, listing)
        .unwrap_err()
        .contains("clang assembled"));
    word.page = "missing".into();
    assert!(check(&word, listing)
        .unwrap_err()
        .contains("missing Arm page"));
}
