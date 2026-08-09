#!/usr/bin/env python3
from __future__ import annotations

import pathlib
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def load(path: pathlib.Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def main() -> int:
    errors: list[str] = []
    manifest = load(ROOT / ".zpkg.toml")
    lock = load(ROOT / ".zpkg.lock")
    package = manifest.get("package", {})
    if package.get("org") != "apostille-me" or package.get("name") != "apme-interfaces":
        errors.append("package identity must be apostille-me/apme-interfaces")
    if package.get("language") != "universal":
        errors.append("package.language must use the supported universal variant")
    if package.get("repository", {}).get("url") != "https://github.com/apostille-me/apme-interfaces":
        errors.append("package.repository.url must match the canonical repository")
    dependencies = manifest.get("dependencies", {})
    if dependencies not in ({}, None):
        errors.append("interfaces must remain the dependency root")
    if lock.get("version") != 1:
        errors.append(".zpkg.lock must use version = 1")
    if manifest.get("targets", {}).get("repository", {}).get("dir") != ".":
        errors.append("[targets.repository] must publish the repository root")
    for error in errors:
        print(f"error: {error}", file=sys.stderr)
    if errors:
        return 1
    print("validated apostille-me/apme-interfaces Zed package boundary")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
