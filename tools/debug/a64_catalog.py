#!/usr/bin/env python3
"""Generate the A64 encoding catalog from Arm's BSD-licensed AARCHMRS archive."""

import argparse
import hashlib
import json
import sys
import tarfile
from pathlib import Path


RELEASE = "2026-09_rel"
ARCHIVE_SHA256 = "d5bf3f538663fb18be081ba8e9f08b188ed06f81dd19d4aa2da5b2795103a4ec"
CATALOG = Path(__file__).resolve().parents[2] / "db/a64_encodings.tsv"
HEADER = ("page", "encoding", "mnemonics", "pattern", "exclude", "fields", "condition", "preferred")
ANY = (0, 0)  # (fixed-bit mask, required value)


def bits(value):
    text = value["value"]
    if not (text.startswith("'") and text.endswith("'") and text[1:-1]
            and set(text[1:-1]) <= set("01x")):
        raise ValueError(f"invalid Arm bit string {text!r}")
    return text[1:-1]


def cube(text, low):
    mask = int("".join("0" if b == "x" else "1" for b in text), 2)
    return mask << low, int(text.replace("x", "0"), 2) << low


def intersect(a, b):
    mask, value = a
    if (value ^ b[1]) & mask & b[0]:
        return None
    return mask | b[0], value | b[1]


def diagram(node, inherited):
    fixed = inherited
    fields = {}
    occupied = 0
    for field in node["encoding"]["values"]:
        low, width = field["range"]["start"], field["range"]["width"]
        text = bits(field["value"])
        if not (0 <= low < low + width <= 32 and len(text) == width):
            raise ValueError(f"invalid bit range in {node['name']}")
        span = ((1 << width) - 1) << low
        fixed = intersect(fixed, cube(text, low))
        if fixed is None or span & occupied:
            raise ValueError(f"conflicting bit ranges in {node['name']}")
        occupied |= span
        if "name" in field:
            fields[field["name"]] = (low, width)
    if node["_type"] == "Instruction.Instruction" and occupied != 0xffffffff:
        raise ValueError(f"incomplete instruction diagram in {node['name']}")
    return fixed, fields


def overlap(left, right):
    return {common for a in left for b in right if (common := intersect(a, b)) is not None}


def field_test(field, literal, equal):
    low, width = field
    text = bits(literal)
    if len(text) != width:
        raise ValueError(f"field width {width} disagrees with {text!r}")
    mask, value = cube(text, low)
    if equal:
        return {(mask, value)}
    # The complement of a pattern is the union of its individual bit mismatches.
    return {(1 << i, ~value & (1 << i)) for i in range(low, low + width) if mask & (1 << i)}


def condition_cubes(expr, fields, truth):
    """Word patterns making an Arm condition true or false with all features available."""
    kind = expr["_type"]
    if kind == "AST.Bool":
        return {ANY} if expr["value"] == truth else set()
    if kind == "AST.Function" and expr["name"] == "IsFeatureImplemented":
        return {ANY} if truth else set()
    if kind == "AST.UnaryOp" and expr["op"] == "!":
        return condition_cubes(expr["expr"], fields, not truth)
    if kind != "AST.BinaryOp":
        raise ValueError(f"unknown encoding condition {expr}")
    op, left, right = expr["op"], expr["left"], expr["right"]
    if op in ("&&", "||"):
        a = condition_cubes(left, fields, truth)
        b = condition_cubes(right, fields, truth)
        return overlap(a, b) if (op == "&&") == truth else a | b
    if left["_type"] != "AST.Identifier":
        raise ValueError(f"expected named field in {expr}")
    field = fields[left["value"]]
    if op in ("==", "!="):
        return field_test(field, right, (op == "==") == truth)
    if op == "IN" and right["_type"] == "AST.Set":
        matches = set() if truth else {ANY}
        for literal in right["values"]:
            part = field_test(field, literal, truth)
            matches = matches | part if truth else overlap(matches, part)
        return matches
    raise ValueError(f"unknown field test {expr}")


def expression(expr):
    """Keep feature conditions and disassembly preferences legible in the catalog."""
    kind = expr["_type"]
    if kind == "AST.Bool":
        return str(expr["value"]).upper()
    if kind in ("AST.Identifier", "AST.Integer", "Values.Value"):
        return str(expr["value"])
    if kind == "AST.UnaryOp":
        return expr["op"] + expression(expr["expr"])
    if kind == "AST.BinaryOp":
        return f"({expression(expr['left'])} {expr['op']} {expression(expr['right'])})"
    if kind == "AST.Set":
        return "{" + ", ".join(map(expression, expr["values"])) + "}"
    if kind in ("AST.Function", "AST.SquareOp"):
        args = ", ".join(map(expression, expr["arguments"]))
        if kind == "AST.SquareOp":
            return f"{expression(expr['var'])}[{args}]"
        if expr["parameters"]:
            raise ValueError(f"unexpected function parameters in {expr}")
        return args if expr["name"] == "IsFeatureImplemented" else f"{expr['name']}({args})"
    raise ValueError(f"unknown Arm expression {expr}")


def pattern(c):
    mask, value = c
    return "".join(str((value >> i) & 1) if mask & (1 << i) else "x" for i in reversed(range(32)))


def mnemonics(symbols, rules, prefix=""):
    """Expand the assembly grammar only up to the first space (e.g. SXTL/SXTL2 or B.cond)."""
    if not symbols:
        yield prefix
        return
    symbol, *rest = symbols
    if symbol["_type"] == "Instruction.Symbols.Literal":
        yield from mnemonics(rest, rules, prefix + symbol["value"].lower())
    elif symbol["rule_id"] == "SPACE":
        yield prefix
    else:
        rule = rules[symbol["rule_id"]]
        if rule["_type"] == "Instruction.Rules.Rule":
            choices = [rule["symbols"]]
        elif rule["_type"] == "Instruction.Rules.Choice":
            choices = rule["choices"]
        else:
            raise ValueError(f"unexpected token in mnemonic: {symbol}")
        for choice in choices:
            suffix = choice["symbols"] if choice else []
            yield from mnemonics(suffix + rest, rules, prefix)


def word_patterns(fixed, exclusions):
    # Fold one-bit exclusions into the diagram (e.g. CMP requires Rd == 31).
    while True:
        remaining = {(m & ~fixed[0], v & ~fixed[0]) for m, v in exclusions
                     if intersect(fixed, (m, v)) is not None}
        if ANY in remaining:
            raise ValueError("encoding has no eligible words")
        forced = [(m, v ^ m) for m, v in remaining if m & (m - 1) == 0]
        if not forced:
            return pattern(fixed), ",".join(sorted(map(pattern, remaining))) or "-"
        for bit in forced:
            fixed = intersect(fixed, bit)
            if fixed is None:
                raise ValueError("unsatisfiable field constraints")


def catalog(root, rules):
    rows = []
    names = set()

    def walk(node, fixed, fields, excluded, conditions, parent=""):
        kind = node["_type"].removeprefix("Instruction.")
        if kind not in ("InstructionSet", "InstructionGroup", "Instruction", "InstructionAlias"):
            raise ValueError(f"unexpected A64 node {kind}")
        if kind != "InstructionAlias":
            fixed, own_fields = diagram(node, fixed)
            fields = {**fields, **own_fields}
        excluded = excluded | condition_cubes(node["condition"], fields, False)
        condition = expression(node["condition"])
        if condition != "TRUE" and condition not in conditions:
            conditions = (*conditions, condition)
        if kind in ("Instruction", "InstructionAlias"):
            name = node["name"]
            if kind == "Instruction":
                parent, layout = name, own_fields
            else:
                name = f"{parent}/{name}"
                layout = fields
            if name in names:
                raise ValueError(f"duplicate encoding {name}")
            names.add(name)
            # Variable named fields of the instruction itself, in diagram order.
            operands = ",".join(f"{k}[{lo + w - 1}:{lo}]" for k, (lo, w) in layout.items()
                                if ((fixed[0] >> lo) & ((1 << w) - 1)) != (1 << w) - 1)
            match, forbidden = word_patterns(fixed, excluded)
            spellings = ",".join(sorted(set(mnemonics(node["assembly"]["symbols"], rules))))
            row = (node["operation_id"].lower(), name, spellings, match, forbidden, operands or "-",
                   " && ".join(conditions) or "TRUE",
                   expression(node["preferred"]) if node.get("preferred") else "-")
            rows.append(row)
            fields = layout  # Aliases name the containing instruction's fields.
        for child in node.get("children", []):
            walk(child, fixed, fields, excluded, conditions, parent)

    walk(root, ANY, {}, set(), ())
    if not rows:
        raise ValueError("empty A64 instruction set")
    return "\n".join("\t".join(row) for row in [HEADER, *sorted(rows)]) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archive", type=Path, help="AARCHMRS_OPENSOURCE_A_profile_FAT-2026-09.tar.gz")
    parser.add_argument("--check", action="store_true", help="compare with db/a64_encodings.tsv")
    args = parser.parse_args()
    with args.archive.open("rb") as archive:
        digest = hashlib.sha256(archive.read()).hexdigest()
        if digest != ARCHIVE_SHA256:
            sys.exit(f"unexpected Arm archive SHA-256: {digest}")
        archive.seek(0)
        with tarfile.open(fileobj=archive, mode="r:gz") as release:
            source = json.load(release.extractfile("Instructions.json"))
    if source["_meta"]["version"]["ref"] != RELEASE:
        sys.exit("unexpected Arm release")
    root, = (n for n in source["instructions"] if n["name"] == "A64")
    if root["read_width"] != 32:
        sys.exit("expected a 32-bit A64 instruction set")
    text = catalog(root, source["assembly_rules"])
    if args.check:
        if CATALOG.read_bytes() != text.encode("utf-8"):
            sys.exit(f"{CATALOG.name} differs from Arm {RELEASE}")
        print(f"{CATALOG.name} matches Arm {RELEASE} ({len(text.splitlines()) - 1} encodings)")
    else:
        sys.stdout.buffer.write(text.encode("utf-8"))


if __name__ == "__main__":
    main()
