"""Check local Markdown/image references without following remote links."""

from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
LINK = re.compile(r"!?\[[^\]]*\]\(([^\s)]+)(?:\s+[^)]*)?\)|(?:src|href)=[\"']([^\"']+)[\"']")


def check() -> list[str]:
  errors = []
  for document in sorted(ROOT.rglob("*.md")):
    if any(part in {".git", ".venv", "dist"} for part in document.relative_to(ROOT).parts):
      continue
    for match in LINK.finditer(document.read_text(encoding="utf-8")):
      target = match[1] or match[2]
      parsed = urlsplit(target)
      if parsed.scheme or parsed.netloc or not parsed.path:
        continue
      path = (document.parent / unquote(parsed.path)).resolve()
      if not path.is_relative_to(ROOT) or not path.exists():
        errors.append(f"{document.relative_to(ROOT)}: missing local target {target}")
  return errors


if __name__ == "__main__":
  errors = check()
  if errors:
    raise SystemExit("\n".join(errors))
  print("Local Markdown links and image paths: OK")
