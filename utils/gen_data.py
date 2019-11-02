#!/usr/bin/env python3

# Requires `chardet`; install using `pip install chardet`

import pathlib
import sys
import re
import string
import collections
import json
import chardet

ROOT_FOLDER = pathlib.Path(__file__).parent.parent
VALID_FILENAME_CHARS = string.ascii_lowercase + string.digits + "_"
UNICODE_REPLACEMENT_CHAR = "�"


def generate_rust_module(brand, threads):
    PRELUDE = [
        "#![allow(unknown_lints)]",
        "#![allow(clippy::all)]",
        "" "#![cfg_attr(rustfmt, rustfmt_skip)]",
        "",
        "use crate::ThreadRef;",
        "",
    ]

    head = [
        f'const BRAND: &\'static str = "{brand}";',
        f"const THREADS: [ThreadRef; {len(threads)}] = [",
    ]
    body = [
        "\n".join(
            [
                "    ThreadRef {",
                f"        brand: BRAND,",
                f'        code: "{code}",',
                f'        name: "{name}",',
                f"        color: &{list(color)},",
                "    },",
            ]
        )
        for (code, name, color) in threads
    ]
    tail = ["];"]

    return "\n".join(PRELUDE + head + body + tail)


class ThreadData:
    def __init__(self):
        self.data = collections.defaultdict(set)
        self.brand_filenames = {}
        self.detector = chardet.UniversalDetector()

    def decode_string(self, data, count=0):
        self.detector.reset()
        for line in data.splitlines(keepends=True):
            self.detector.feed(line)
            if self.detector.done:
                break
        self.detector.close()
        if self.detector.result["encoding"] is None:
            return None
        if self.detector.result["confidence"] < 1.0:
            if (
                count == 0
                and self.detector.result["encoding"] == "ISO-8859-1"
                and b"\xA0" in data
            ):
                # Some files seem to use 0xA0 value as a space!?
                # Try decoding with that instead
                result = self.decode_string(data.replace(b"\xA0", b" "))
                if result is not None:
                    return result
        return data.decode(self.detector.result["encoding"])

    def parse_thread_csv(self, file):
        try:
            data = file.read_bytes()
            file_text = self.decode_string(data)
            if file_text is None:
                return
            for line in file_text.splitlines():
                if not line.strip():
                    continue
                line = line.replace("\xA0", " ")
                if any(ord(c) > 128 for c in line):
                    print(f"Non-ascii character detected in {line!r}")
                splits = line.split(",")
                if len(splits) != 6:
                    break
                code, brand, name, r, g, b = splits
                self.data[brand].add(
                    (code.strip(), name.strip(), (int(r), int(g), int(b)))
                )
        except IOError:
            pass
        except IndexError:
            pass

    def prepare_to_write(self):
        if not self.data:
            return False
        if any(isinstance(v, set) for v in self.data.values()):
            self.normalise_data()
        if set(self.brand_filenames) != set(self.data):
            self.generate_filenames()
        return True

    def normalise_data(self):
        self.data = {
            brand: sorted(threads) for brand, threads in sorted(self.data.items())
        }

    def generate_filenames(self):
        self.brand_filenames = {
            brand: "".join(
                c if c in VALID_FILENAME_CHARS else "_" for c in brand.lower()
            )
            for brand in self.data
        }
        assert len(set(self.brand_filenames.keys())) == len(
            set(self.brand_filenames.values())
        )

    def generate_rust(self):
        if not self.prepare_to_write():
            return
        folder = ROOT_FOLDER / "src/generated/"
        folder.mkdir(parents=True, exist_ok=True)
        for brand, threads in self.data.items():
            filename = self.brand_filenames[brand]
            (folder / f"{filename}.rs").write_text(generate_rust_module(brand, threads))
        mod = folder.with_suffix(".rs")
        modules = {f"mod {name};" for name in self.brand_filenames.values()}
        if mod.is_file():
            modules = modules | {
                line for line in mod.read_text().splitlines() if line.startswith("mod ")
            }
        mod.write_text("\n".join(sorted(modules)))

    def generate_json(self):
        if not self.prepare_to_write():
            return
        folder = ROOT_FOLDER / "data/brands/"
        folder.mkdir(parents=True, exist_ok=True)
        rollup_data = {}
        for brand, threads in self.data.items():
            filename = self.brand_filenames[brand]
            thread_data = [
                {"code": code, "name": name, "color": color}
                for (code, name, color) in threads
            ]
            rollup_data[brand] = thread_data
            (folder / f"{filename}.json").write_text(
                json.dumps(
                    {"brand": brand, "threads": thread_data},
                    sort_keys=True,
                    indent=1,
                    ensure_ascii=True,
                )
            )
        folder.with_suffix(".json").write_text(
            json.dumps(
                {
                    brand: f"brands/{filename}.json"
                    for brand, filename in self.brand_filenames.items()
                },
                sort_keys=True,
                indent=1,
                ensure_ascii=True,
            )
        )
        (folder.parent / "rollup.json").write_text(
            json.dumps(rollup_data, sort_keys=True, indent=1, ensure_ascii=True)
        )


def main(args):
    files = [*map(pathlib.Path, args)]
    data = ThreadData()

    while files:
        file = files.pop(0)
        if file.is_dir():
            files += file.iterdir()
        elif file.is_file():
            data.parse_thread_csv(file)
    if data.prepare_to_write():
        data.generate_rust()
        data.generate_json()


if __name__ == "__main__":
    main(sys.argv[1:])
