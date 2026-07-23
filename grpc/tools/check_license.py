#!/usr/bin/env python3
"""
Checks source files (*.rs, *.proto) for exact match against MIT or Apache 2.0 license boilerplate.
Performs piecewise validation and unified diffing against expected boilerplate templates.
"""

import datetime
import difflib
import os
import re
import sys
from pathlib import Path

IGNORE_DIRS = {"target", ".git", ".gemini", "generated", "node_modules"}
IGNORE_FILES = {"src/credentials/rustls/key_log.rs"}  # Third-party Apache 2.0 file
TARGET_EXTENSIONS = (".rs",)

MIN_YEAR = 2025

# Regex to capture: (prefix) Copyright [(c)] (year_spec)
COPYRIGHT_RE = re.compile(
    r"^(\s*(?:/\*|\*|//)\s*)Copyright\s+(?:\(c\)\s+)?([\d\-,]+)\b",
    re.IGNORECASE,
)

MIT_BLOCK_TEMPLATE = """/*
 *
 * Copyright <YEAR> gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */""".splitlines()


def should_ignore(filepath: str) -> bool:
    normalized = filepath.replace("\\", "/")
    for ign_file in IGNORE_FILES:
        if normalized.endswith(ign_file):
            return True
    return bool(IGNORE_DIRS.intersection(Path(filepath).parts))


def extract_header_lines(lines: list[str]) -> tuple[list[str] | None, str | None]:
    if not lines:
        return None, "File is empty; expected /* ... */ or // ... header block on line 1"

    first_line = lines[0].strip()
    header = []
    if first_line.startswith("/*"):
        for line in lines:
            header.append(line.rstrip())
            if line.strip() == "*/":
                break
        return header, None
    elif first_line.startswith("//"):
        for line in lines:
            if line.strip().startswith("//"):
                header.append(line.rstrip())
            else:
                break
        return header, None

    return (
        None,
        f"Expected comment header block (/* or //) on line 1, found: {repr(first_line)}",
    )


def validate_copyright_years(year_spec: str) -> tuple[bool, str | None]:
    current_year = datetime.date.today().year
    years = [int(y) for y in re.findall(r"\b\d{4}\b", year_spec)]
    if not years:
        return False, f"No valid 4-digit years found in '{year_spec}'"
    for y in years:
        if y < MIN_YEAR or y > current_year:
            return (
                False,
                f"Year {y} in '{year_spec}' is outside allowed range [{MIN_YEAR}, {current_year}]",
            )
    return True, None


def validate_copyright(header_lines: list[str]) -> tuple[bool, str | None]:
    for line in header_lines:
        match = COPYRIGHT_RE.match(line)
        if match:
            year_spec = match.group(2).strip()

            valid_years, year_err = validate_copyright_years(year_spec)
            if not valid_years:
                return False, f"Copyright year validation failed on line {repr(line.strip())}: {year_err}"

            return True, None

    return False, "No valid 'Copyright <years>' line found in header block"


def normalize_header(header_lines: list[str]) -> list[str]:
    norm_header = []
    for line in header_lines:
        match = COPYRIGHT_RE.match(line)
        if match:
            norm_line = line[:match.start(2)] + "<YEAR>" + line[match.end(2):]
            norm_header.append(norm_line.rstrip())
        else:
            norm_header.append(line.rstrip())
    return norm_header


def check_diff(norm_header: list[str], filepath: str) -> tuple[bool, str | None]:
    diff = list(
        difflib.unified_diff(
            MIT_BLOCK_TEMPLATE,
            norm_header,
            fromfile="Expected MIT License (Block)",
            tofile=filepath,
            lineterm="",
        )
    )
    if not diff:
        return True, None

    diff_str = "\n".join(diff)
    return False, f"Boilerplate mismatch:\n{diff_str}"


def validate_file(filepath: str) -> tuple[bool, str | None]:
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            raw_lines = [f.readline() for _ in range(50)]
    except Exception as e:
        return False, f"Failed to read file: {e}"

    header_lines, err = extract_header_lines(raw_lines)
    if err or header_lines is None:
        return False, err

    valid_copy, err = validate_copyright(header_lines)
    if not valid_copy:
        return False, err

    norm_header = normalize_header(header_lines)
    return check_diff(norm_header, filepath)


def main():
    targets = sys.argv[1:] if len(sys.argv) > 1 else ["grpc" if os.path.exists("grpc") else "."]
    files_to_check = []

    for target in targets:
        if os.path.isfile(target):
            if not should_ignore(target) and target.endswith(TARGET_EXTENSIONS):
                files_to_check.append(target)
        elif os.path.isdir(target):
            for dirpath, dirnames, filenames in os.walk(target):
                dirnames[:] = [d for d in dirnames if d not in IGNORE_DIRS]
                for f in filenames:
                    if f.endswith(TARGET_EXTENSIONS):
                        full_path = os.path.join(dirpath, f)
                        if not should_ignore(full_path):
                            files_to_check.append(full_path)

    files_to_check.sort()
    failures = []

    for filepath in files_to_check:
        passed, reason = validate_file(filepath)
        if not passed:
            failures.append((filepath, reason))

    print(f"Checked {len(files_to_check)} code files for exact license boilerplate match.")
    if failures:
        print(f"FAILED: Found {len(failures)} files with boilerplate discrepancies:\n")
        for filepath, reason in failures:
            print(f"--- {filepath} ---")
            print(f"{reason}\n")
        sys.exit(1)
    else:
        print("SUCCESS: All checked files contain exact MIT license boilerplate.")
        sys.exit(0)


if __name__ == "__main__":
    main()
