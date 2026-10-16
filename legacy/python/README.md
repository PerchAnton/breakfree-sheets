# Historical Python/Tk prototype

Preserved as the v1 calculation/persistence conformance reference. Owner decision
ADR 0003 supersedes this implementation: the active application, including GUI,
model, calculation, history, files and Office adapter, is entirely Rust.
Nothing in this directory is built, embedded, imported or executed by Cargo or
the native executable. The Python prototype is not the current product launcher.

The frozen original core can be checked from the repository root with:

```sh
PYTHONPATH=legacy/python python3 -m unittest discover -s legacy/python/tests -p test_core.py
PYTHONPATH=legacy/python python3 -m unittest discover -s legacy/python/tests -p test_persistence.py
```

`tests_rust/fixtures/python-v1-conformance.json` records 35 synthetic scenarios
from this engine, with canonical numeric displays, so Rust tests need no Python.
Historical notices, integration fixture and build sources retain their original
version and provenance rather than being rewritten as new Rust claims.
