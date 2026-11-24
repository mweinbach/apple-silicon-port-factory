#!/usr/bin/env python3
"""Write page names and fixed-bit patterns from an extracted Arm A64 ISA XML release."""

import argparse
import sys
import xml.etree.ElementTree as ET
from pathlib import Path


def apply_boxes(bits, boxes):
    """Overlay the fixed cells of a register diagram or one of its encoding variants."""
    for box in boxes:
        start = 31 - int(box.attrib["hibit"])
        end = start + int(box.get("width", 1))
        offset = start
        for cell in box.findall("c"):
            width = int(cell.get("colspan", 1))
            value = (cell.text or "").strip()
            if not (0 <= start <= offset < offset + width <= end <= 32):
                raise ValueError("invalid bit range in encoding diagram")
            if value in ("0", "1", "(0)", "(1)"):
                bit = value.strip("()")
                if width != 1 or bits[offset] not in ("x", bit):
                    raise ValueError("inconsistent fixed bit in encoding diagram")
                bits[offset] = bit
            elif value not in ("", "x", "N", "Z") and not value.startswith("!= "):
                raise ValueError(f"unknown encoding cell {value!r}")
            # Field exclusions (N/Z and !=) do not specify individual fixed bits.
            offset += width
        if offset != end:
            raise ValueError("incomplete encoding box")


def page_patterns(xml_file):
    root = ET.parse(xml_file).getroot()
    if root.tag != "instructionsection" or root.get("type") not in ("instruction", "alias"):
        return
    for iclass in root.findall("./classes/iclass"):
        diagram = iclass.find("regdiagram")
        if diagram is None or diagram.get("form") != "32":
            raise ValueError("missing 32-bit register diagram")
        common = ["x"] * 32
        apply_boxes(common, diagram.findall("box"))
        for encoding in iclass.findall("encoding"):
            bits = common.copy()
            apply_boxes(bits, encoding.findall("box"))
            yield "".join(bits)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("xml_dir", type=Path, help="the release directory containing the instruction XML files")
    args = parser.parse_args()
    rows = []
    for xml_file in sorted(args.xml_dir.glob("*.xml")):
        try:
            rows.extend((xml_file.stem, pattern) for pattern in sorted(set(page_patterns(xml_file))))
        except (ET.ParseError, ValueError) as error:
            sys.exit(f"{xml_file}: {error}")
    if not rows:
        sys.exit(f"{args.xml_dir}: no A64 instruction encoding diagrams found")
    print("page\tpattern")
    for page, pattern in rows:
        print(f"{page}\t{pattern}")


if __name__ == "__main__":
    main()
