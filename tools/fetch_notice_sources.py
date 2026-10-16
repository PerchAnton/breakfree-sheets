"""Explicit maintenance helper: fetch missing notice texts at crate-pinned Git SHAs.

Never called by Cargo/build/test/runtime. Requires previously saved Cargo metadata.
"""
import concurrent.futures
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
packages = json.loads((ROOT / 'artifacts/cargo-metadata.json').read_text())['packages']
queries = {}
for package in packages:
    if package['source'] is None:
        continue
    base = Path(package['manifest_path']).parent
    candidates = [p for p in base.iterdir() if p.is_file() and p.name.upper().startswith(('LICENSE','COPYING','NOTICE','COPYRIGHT'))]
    if candidates:
        continue
    vcs = base / '.cargo_vcs_info.json'
    sha = json.loads(vcs.read_text()).get('git', {}).get('sha1') if vcs.exists() else None
    repository = package.get('repository') or ''
    match = re.match(r'https?://github.com/([^/]+)/([^/]+)', repository)
    if match and sha and re.fullmatch('[0-9a-f]{40}', sha):
        owner, repo = match.groups()
        queries.setdefault((owner, repo.removesuffix('.git'), sha), []).append(package['name']+'@'+package['version'])

def fetch(item):
    (owner, repo, sha), names = item
    results = []
    for filename in ['LICENSE-MIT','LICENSE-APACHE','LICENSE','LICENSE.txt','LICENSE.md','COPYING']:
        url = f'https://raw.githubusercontent.com/{owner}/{repo}/{sha}/{filename}'
        try:
            data = subprocess.run(
                ['curl', '--fail', '--silent', '--show-error', '--max-time', '8', url],
                check=True, capture_output=True, timeout=12,
            ).stdout
            if not data or len(data) >= 200000:
                continue
            data.decode('utf-8')
            name = hashlib.sha256(data).hexdigest()[:20]+'.txt'
            results.append((name, data, url))
        except (OSError, UnicodeError, subprocess.SubprocessError):
            continue
    return names, results

directory = ROOT/'third_party/licenses'
directory.mkdir(parents=True, exist_ok=True)
index = {}
with concurrent.futures.ThreadPoolExecutor(max_workers=12) as pool:
    for names, results in pool.map(fetch, queries.items()):
        links = []
        for filename, data, url in results:
            (directory/filename).write_bytes(data)
            links.append({'file':filename, 'source':url})
        for name in names:
            if links:
                index[name]=links
(ROOT/'third_party/upstream_sources.json').write_text(json.dumps(index,indent=2,sort_keys=True)+'\n')
print(f'Pinned upstream license sources retrieved for {len(index)} package versions')
