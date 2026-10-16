import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from breakfree_sheets import __version__
from breakfree_sheets.cli import main
from breakfree_sheets.office import capabilities
from breakfree_sheets.persistence import save
from breakfree_sheets.workbook import Workbook
from tools.build import build

ROOT = Path(__file__).resolve().parents[1]


class ContractTests(unittest.TestCase):
  def command(self, *args, launcher=None, cwd=None):
    return subprocess.run([sys.executable, str(launcher or ROOT / "breakfree-sheets"), *args],
                          cwd=cwd or ROOT, capture_output=True, text=True, timeout=15,
                          env={**os.environ, "PYTHONPATH": "", "DISPLAY": ""})

  def test_capabilities_exact_contract_one_json_and_no_gui_import(self):
    result = self.command("--capabilities")
    self.assertEqual(result.returncode, 0, result.stderr)
    self.assertEqual(result.stderr, "")
    self.assertEqual(len(result.stdout.splitlines()), 1)
    document = json.loads(result.stdout)
    self.assertEqual(document, capabilities())
    fixture = json.loads((ROOT / "contracts/v1/capabilities.example.json").read_text())
    self.assertEqual(document, fixture)
    self.assertEqual(document["app_version"], __version__)
    code = "from breakfree_sheets.cli import main; main(['--capabilities']); import sys; assert 'tkinter' not in sys.modules"
    subprocess.run([sys.executable, "-c", code], cwd=ROOT, capture_output=True, check=True, timeout=15)

  def test_version_is_actual_build_version(self):
    result = self.command("--version")
    self.assertEqual(result.returncode, 0)
    self.assertEqual(result.stdout.strip(), f"BreakFree Sheets {__version__}")
    self.assertEqual(result.stderr, "")

  def test_new_and_absolute_open_route_to_same_application(self):
    calls = []
    fake_gui = SimpleNamespace(run=lambda book, path: calls.append((book, path)))
    with tempfile.TemporaryDirectory() as temporary:
      path = Path(temporary) / "Unicode Україна with spaces.bfsheet"
      save(Workbook(), path)
      with patch.dict(sys.modules, {"breakfree_sheets.gui": fake_gui}):
        self.assertEqual(main(["--new"]), 0)
        self.assertEqual(main(["--open", str(path)]), 0)
    self.assertEqual(calls[0], (None, None))
    self.assertIsInstance(calls[1][0], Workbook)
    self.assertEqual(calls[1][1], path)

  def test_relative_path_and_conflicting_actions_fail_before_gui(self):
    for args in [("--open", "relative.bfsheet"), ("--new", "--capabilities"), ("--version", "--new")]:
      result = self.command(*args)
      self.assertNotEqual(result.returncode, 0, args)
      self.assertEqual(result.stdout, "")

  def test_invalid_file_and_missing_file_have_no_gui_or_stdout(self):
    with tempfile.TemporaryDirectory() as temporary:
      path = Path(temporary) / "bad.bfsheet"
      for data in [None, b"invalid", b'{"file_version":999}']:
        if data is not None:
          path.write_bytes(data)
        result = self.command("--open", str(path))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stdout, "")
        self.assertTrue(result.stderr)

  def test_build_is_deterministic_and_runs_outside_checkout(self):
    with tempfile.TemporaryDirectory() as temporary:
      isolated = Path(temporary)
      launcher = build(isolated)
      previous = launcher.read_bytes()
      build(isolated)
      self.assertEqual(launcher.read_bytes(), previous)
      result = self.command("--capabilities", launcher=launcher, cwd=isolated)
      self.assertEqual(result.returncode, 0, result.stderr)
      self.assertEqual(json.loads(result.stdout), capabilities())
      self.assertEqual(self.command("--version", launcher=launcher, cwd=isolated).returncode, 0)
      self.assertTrue((isolated / "breakfree-sheets.cmd").is_file())
