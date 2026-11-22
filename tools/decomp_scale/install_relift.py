#!/usr/bin/env python3
"""Install fresh review-mode lifts (from gen_subsystems.py --only-va) into the shards.

For every target/review/<dir>/fn_<va>.rs:
  * if a shard file already defines `fn fn_<va>(`, that body is replaced in place;
  * otherwise the body is appended to shards/subsystems/relift.rs (created on demand
    and registered in subsystems/mod.rs);
  * the registry entry for <va> is pointed at fn_<va> (keeping its Throws mode).

Usage: install_relift.py <review_out_dir>
"""
import glob
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
SHARDS = f"{ROOT}/games/alan-wake-2/crates/aw2-game/src/shards"
HEADER = ("#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, "
          "dead_code, clippy::all, unreachable_code, unused_parens)]\n\n")


def body_span(text, va):
    m = re.search(rf"(#\[inline\(never\)\]\n)?pub unsafe extern \"win64\" fn fn_{va}\(", text)
    if not m:
        return None
    end = text.find("\n}\n", m.start())
    return m.start(), end + 3


def main(src_dir):
    fresh = {}
    for p in glob.glob(f"{src_dir}/fn_*.rs"):
        va = os.path.basename(p)[3:-3]
        fresh[va] = open(p).read().replace("\r\n", "\n").rstrip("\n") + "\n"
    t_files = [f for f in glob.glob(f"{SHARDS}/**/*.rs", recursive=True)
               if not os.path.basename(f).startswith("e") and not f.endswith("mod.rs")]
    remaining = dict(fresh)
    replaced = 0
    for f in t_files:
        text = open(f).read()
        if "fn fn_" not in text:
            continue
        changed = False
        for va in [v for v in remaining if f"fn fn_{v}(" in text]:
            span = body_span(text, va)
            if not span:
                continue
            text = text[:span[0]] + remaining.pop(va) + text[span[1]:]
            changed = True
            replaced += 1
        if changed:
            open(f, "w").write(text)
    appended = 0
    if remaining:
        rl = f"{SHARDS}/subsystems/relift.rs"
        existing = open(rl).read() if os.path.exists(rl) else HEADER
        for va, body in sorted(remaining.items()):
            span = body_span(existing, va)
            if span:
                existing = existing[:span[0]] + body + existing[span[1]:]
            else:
                existing += "\n" + body
            appended += 1
        open(rl, "w").write(existing)
        mod = f"{SHARDS}/subsystems/mod.rs"
        mtext = open(mod).read()
        if "pub mod relift;" not in mtext:
            mtext = mtext.rstrip("\n") + "\npub mod relift;\npub use relift::*;\n"
            open(mod, "w").write(mtext)
    # registry repoint
    ent = re.compile(r'(Replacement::[a-z_]+\((0x[0-9a-f]+), "[^"]*", )([^\n]*?)( as \*const \(\))')
    repointed = 0
    for f in glob.glob(f"{SHARDS}/**/e*.rs", recursive=True):
        text = open(f).read()
        in_subsystems = os.sep + "subsystems" + os.sep in f

        def rep(m):
            nonlocal repointed
            va = f"{int(m.group(2), 16):09x}"
            if va not in fresh:
                return m.group(0)
            target = f"fn_{va}"
            if va in remaining and not in_subsystems:
                target = f"crate::shards::subsystems::fn_{va}"
            if m.group(3) == target:
                return m.group(0)
            repointed += 1
            return f"{m.group(1)}{target}{m.group(4)}"

        new = ent.sub(rep, text)
        if new != text:
            open(f, "w").write(new)
    print(f"fresh={len(fresh)} replaced_in_place={replaced} appended={appended} registry_repointed={repointed}")


if __name__ == "__main__":
    main(sys.argv[1])
