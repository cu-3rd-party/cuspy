from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LOCALES_DIR = ROOT / "frontend" / "messages"
BOLD = "\033[1m"
RED = "\033[31m"
YELLOW = "\033[33m"
GREEN = "\033[32m"
RESET = "\033[0m"


def read_locale(path: Path) -> dict[str, str]:
    with path.open("r", encoding="utf-8") as file:
        data = json.load(file)

    if not isinstance(data, dict):
        raise ValueError(f"Locale file must contain a JSON object: {path}")

    return data


def write_locale(path: Path, data: dict[str, str]) -> None:
    sorted_data = dict(sorted(data.items()))
    with path.open("w", encoding="utf-8") as file:
        json.dump(sorted_data, file, ensure_ascii=False, indent=2)
        file.write("\n")


def main() -> int:
    locale_paths = sorted(LOCALES_DIR.glob("*.json"))
    if not locale_paths:
        raise SystemExit(f"No locale files found in {LOCALES_DIR}")

    locales = {path.name: read_locale(path) for path in locale_paths}
    all_keys = sorted({key for locale in locales.values() for key in locale})

    for key in all_keys:
        existing = {name: data[key] for name, data in locales.items() if key in data}
        if len(existing) == len(locales):
            continue

        source_name, source_value = next(iter(existing.items()))
        missing_locales = sorted(name for name in locales if name not in existing)
        for locale_name in missing_locales:
            print(
                f"Missing key {BOLD}{YELLOW}'{key}'{RESET} in {BOLD}{RED}{locale_name}{RESET}; "
                f"copied from {BOLD}{GREEN}{source_name}{RESET}"
            )
            locales[locale_name][key] = source_value

    for path in locale_paths:
        write_locale(path, locales[path.name])

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
