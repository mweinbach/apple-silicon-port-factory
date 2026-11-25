//! Build the offline mnemonic index from the dispatch matches that actually lead to
//! report_unimplemented. The op-family catalog describes target coverage and includes
//! mnemonics with no handler. Comments, tests and nested arithmetic matches are not handlers.

use std::collections::BTreeSet;
use std::{env, fs, path::PathBuf};
use syn::{visit::Visit, Block, Expr, ExprMatch, ImplItem, Item, Pat};

#[derive(Default)]
struct Dispatch {
    matches: usize,
    mnemonics: BTreeSet<String>,
}

impl Dispatch {
    fn pattern(&mut self, pat: &Pat) {
        match pat {
            Pat::Or(or) => or.cases.iter().for_each(|case| self.pattern(case)),
            Pat::Path(p)
                if p.path.segments.len() == 2 && p.path.segments[0].ident == "Mnemonic" =>
            {
                self.mnemonics.insert(p.path.segments[1].ident.to_string());
            }
            Pat::Wild(_) => {} // delegates to the next handler or returns false
            _ => panic!("unexpected interpreter dispatch pattern; update readiness"),
        }
    }
}

impl<'ast> Visit<'ast> for Dispatch {
    fn visit_expr_match(&mut self, m: &'ast ExprMatch) {
        if matches!(&*m.expr, Expr::Path(p) if p.path.is_ident("mn")) {
            self.matches += 1;
            for arm in &m.arms {
                assert!(
                    arm.guard.is_none(),
                    "guarded mnemonic requires a form check in readiness"
                );
                self.pattern(&arm.pat);
            }
        } else {
            syn::visit::visit_expr_match(self, m);
        }
    }
}

fn handler<'a>(items: &'a [Item], name: &str) -> &'a Block {
    items
        .iter()
        .find_map(|item| match item {
            Item::Fn(f) if f.sig.ident == name => Some(f.block.as_ref()),
            Item::Impl(i) => i.items.iter().find_map(|m| match m {
                ImplItem::Fn(f) if f.sig.ident == name => Some(&f.block),
                _ => None,
            }),
            _ => None,
        })
        .unwrap_or_else(|| panic!("readiness could not find runtime dispatcher {name}"))
}

fn main() {
    let mut names = BTreeSet::new();
    for (file, name) in [
        ("interp", "emu_run_region"),
        ("generic_ops", "exec_generic_ins"),
        ("simd_mov", "exec_simd_mov_int"),
        ("simd_fp", "exec_simd_fp_ext"),
        ("regfile_ext", "exec_x87_ins"),
    ] {
        let path = format!("../x86-runtime/src/emu/{file}.rs");
        println!("cargo:rerun-if-changed={path}");
        let ast = syn::parse_file(&fs::read_to_string(&path).expect(&path)).expect(&path);
        let mut dispatch = Dispatch::default();
        dispatch.visit_block(handler(&ast.items, name));
        assert_eq!(
            dispatch.matches, 1,
            "expected a mnemonic dispatch in {name}"
        );
        names.extend(dispatch.mnemonics);
    }
    let cases = names
        .into_iter()
        .map(|n| format!("Mnemonic::{n}"))
        .collect::<Vec<_>>()
        .join(" | ");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("generic_handlers.rs");
    fs::write(
        out,
        format!("fn has_handler(mn: Mnemonic) -> bool {{ matches!(mn, {cases}) }}"),
    )
    .unwrap();
}
