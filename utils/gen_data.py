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
UNICODE_REPLACEMENT_CHAR = "\uFFFD"


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
        f'pub const BRAND: &\'static str = "{brand}";',
        f"pub const THREADS: [ThreadRef; {len(threads)}] = [",
    ]
    body = [
        "\n".join(
            [
                "    ThreadRef::new(",
                f"        BRAND,",
                f'        "{code}",',
                f'        "{name}",',
                f"        &{list(color)},",
                "    ),",
            ]
        )
        for code, (name, color) in threads.items()
    ]
    tail = ["];"]

    file = "\n".join(PRELUDE + head + body + tail)
    assert UNICODE_REPLACEMENT_CHAR not in file, file
    return file


def simplify_name(name):
    pass


def key_to_filename(key):
    brand, uniqueness = key
    name = brand if uniqueness is None else f"{brand}_{uniqueness}"
    clean = "".join(c if c in VALID_FILENAME_CHARS else "_" for c in name.lower())
    clean = clean.strip("_")
    while "__" in clean:
        clean = clean.replace("__", "_")
    return clean


class ThreadData:
    def __init__(self):
        self.data = collections.defaultdict(lambda: collections.defaultdict(dict))
        self.normalised_data = {}
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
                # Try decoding without it, as it usually increases the confidence
                result = self.decode_string(data.replace(b"\xA0", b" "))
                if result is not None:
                    return result
        decoded = data.decode(self.detector.result["encoding"]).replace("\xA0", " ")
        if UNICODE_REPLACEMENT_CHAR in decoded or "\xA0" in decoded:
            print(f"Cannot decode data: {data}")
        return decoded

    def parse_thread_csv(self, file):
        try:
            data = file.read_bytes()
            file_text = self.decode_string(data)
            if file_text is None:
                return
            threads = []
            for line in file_text.splitlines():
                if not line.strip():
                    continue
                splits = [*map(str.strip, line.split(","))]
                if len(splits) != 6:
                    break
                code, brand, name, r, g, b = splits
                color = (int(r), int(g), int(b))
                if all(0 <= c <= 255 for c in color):
                    threads.append((brand, code, name, color))
            for brand, code, name, color in threads:
                self.data[brand][file][code] = (name, color)
        except IOError:
            pass
        except IndexError:
            pass

    def prepare_to_write(self):
        if not self.data:
            return False
        self.normalise_data()
        self.generate_filenames()
        return True

    def normalise_data(self):
        self.normalised_data = {}
        for brand, thread_by_file in sorted(self.data.items()):
            print(brand, list(thread_by_file.keys()))
            first = next(iter(thread_by_file.values()))
            if all(threads == first for threads in thread_by_file.values()):
                print(f"    Only data-set")
                self.normalised_data[(brand, None)] = next(
                    iter(thread_by_file.values())
                )
            else:
                all_threads = [
                    (key, color)
                    for threads in thread_by_file.values()
                    for key, color in threads.items()
                ]
                print(f"   Thread counts{len(all_threads)} // {len(dict(all_threads))}")
                if len(all_threads) == len(dict(all_threads)):
                    self.normalised_data[(brand, None)] = dict(all_threads)
                else:
                    # Try to find a unique way of extracting the filename.
                    filenames = {
                        (file.stem, *reversed(file.parent.parts)): threads
                        for file, threads in thread_by_file.items()
                    }
                    parts = 1
                    while len(filenames) != len([name[:parts] for name in filenames]):
                        parts += 1
                    for name, threads in filenames.items():
                        self.normalised_data[
                            (brand, "/".join(map(str.lower, name[:parts])))
                        ] = threads

    def generate_filenames(self):
        self.brand_filenames = {
            key: key_to_filename(key) for key in self.normalised_data
        }
        references = collections.defaultdict(list)
        for key, name in self.brand_filenames.items():
            references[name].append(key)
        invalid_names = {
            name: keys for name, keys in references.items() if len(keys) > 1
        }
        if invalid_names:
            print("Invalid counts:", invalid_names)
        assert not invalid_names

    def generate_rust(self):
        if not self.prepare_to_write():
            return
        folder = ROOT_FOLDER / "src/generated/"
        folder.mkdir(parents=True, exist_ok=True)
        for key, threads in self.normalised_data.items():
            brand, _ = key
            filename = self.brand_filenames[key]
            (folder / f"{filename}.rs").write_text(generate_rust_module(brand, threads))
        mod = folder.with_suffix(".rs")
        modules = {f"pub mod {name};" for name in self.brand_filenames.values()}
        if mod.is_file():
            modules = modules | {
                line
                for line in mod.read_text().splitlines()
                if line.startswith("pub mod ")
            }
        mod.write_text("\n".join(sorted(modules)))

    def generate_json(self):
        if not self.prepare_to_write():
            return
        folder = ROOT_FOLDER / "data/brands/"
        folder.mkdir(parents=True, exist_ok=True)
        rollup_data = {}
        for brand, threads in self.normalised_data.items():
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
            files += file.resolve().iterdir()
        elif file.is_file():
            data.parse_thread_csv(file)
    if data.prepare_to_write():
        data.generate_rust()
        data.generate_json()


if __name__ == "__main__":
    main(sys.argv[1:])
