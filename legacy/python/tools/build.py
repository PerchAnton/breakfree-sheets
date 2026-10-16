"""Build a deterministic standard-library zip application, without downloads."""

from pathlib import Path
import zipfile

ROOT = Path(__file__).resolve().parents[1]


def build(output: Path | None = None) -> Path:
  output = output or ROOT / "dist"
  output.mkdir(parents=True, exist_ok=True)
  target = output / "breakfree-sheets"
  with target.open("wb") as stream:
    stream.write(b"#!/usr/bin/env python3\n")
    with zipfile.ZipFile(stream, "w", compression=zipfile.ZIP_STORED) as archive:
      sources = {path.relative_to(ROOT).as_posix(): path.read_bytes()
                 for path in sorted((ROOT / "breakfree_sheets").rglob("*"))
                 if path.is_file() and (path.suffix == ".py" or path.parent.name == "assets")}
      sources["__main__.py"] = b"from breakfree_sheets.cli import main\nraise SystemExit(main())\n"
      for name in ["NOTICES.md", "docs/BRAND_PROVENANCE.md", "contracts/v1/SNAPSHOT.md"]:
        sources[name] = (ROOT / name).read_bytes()
      for name, data in sorted(sources.items()):
        info = zipfile.ZipInfo(name, date_time=(2026, 1, 1, 0, 0, 0))
        info.external_attr = 0o644 << 16
        archive.writestr(info, data)
  target.chmod(0o755)
  (output / "breakfree-sheets.cmd").write_bytes((ROOT / "breakfree-sheets.cmd").read_bytes())
  return target


if __name__ == "__main__":
  print(build())
