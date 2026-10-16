"""Collect locked-package and previously retrieved, source-pinned notices."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
metadata = json.loads((ROOT / 'artifacts/cargo-metadata.json').read_text())
destination = ROOT / 'third_party/licenses'
destination.mkdir(parents=True, exist_ok=True)
rows, missing = [], []
upstream_path = ROOT / 'third_party/upstream_sources.json'
upstream = json.loads(upstream_path.read_text()) if upstream_path.exists() else {}
for package in sorted(metadata['packages'], key=lambda p: (p['name'], p['version'])):
    if package['source'] is None:
        continue
    base = Path(package['manifest_path']).parent
    candidates = [p for p in base.iterdir() if p.is_file() and p.name.upper().startswith(('LICENSE', 'COPYING', 'NOTICE', 'COPYRIGHT'))]
    if package['name'] == 'epaint_default_fonts':
        candidates += list((base / 'fonts').glob('*.txt'))
    links = []
    for source in sorted(candidates):
        data = source.read_bytes()
        filename = hashlib.sha256(data).hexdigest()[:20] + '.txt'
        (destination / filename).write_bytes(data)
        links.append(f'[{source.name}](licenses/{filename})')
    for notice in upstream.get(package['name'] + '@' + package['version'], []):
        if not (destination / notice['file']).is_file():
            raise FileNotFoundError(notice['file'])
        links.append(f"[Pinned upstream text](licenses/{notice['file']}) ([source]({notice['source']}))")
    if not links:
        missing.append(package['name'])
    rows.append(f"| {package['name']} | {package['version']} | `{package['license'] or 'unspecified'}` | {'; '.join(links) or 'No notice text shipped in this package; see upstream license'} |")
header = '''# Locked dependency notice inventory

Generated from Cargo metadata and the source packages selected by Cargo.lock.
When a crate omits its license text, retrieved upstream texts are pinned to the
exact source Git SHA from that crate's .cargo_vcs_info.json; provenance is in
upstream_sources.json. Retrieval is a separate maintenance task, never a build,
test or runtime step.
This lists all resolved platforms/features, including packages not linked into
the current native target. It does not choose a license for BreakFree Sheets.
Copied notice/license texts retain their original bytes; identical files are
stored once. Default-font notices are included explicitly. Packages without a
shipped or pinned upstream license text need review before a redistributable release;
this inventory alone is not a complete redistribution/license audit.

| Package | Version | Declared license | Notice texts and provenance |
| :--- | :--- | :--- | :--- |
'''
(ROOT / 'third_party/NOTICES.md').write_text(header + '\n'.join(rows) + '\n')
print(f'{len(rows)} resolved packages; {len(list(destination.iterdir()))} distinct notice texts')
print('No packaged notice text:', ', '.join(missing))
