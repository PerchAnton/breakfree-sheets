"""Opt-in tests require a real desktop: BREAKFREE_GUI_TESTS=1."""

from decimal import Decimal
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from breakfree_sheets.cells import Kind
from breakfree_sheets.persistence import FileError, load
from tools.build import build


@unittest.skipUnless(os.environ.get("BREAKFREE_GUI_TESTS") == "1", "Set BREAKFREE_GUI_TESTS=1 on a desktop to run Tk tests.")
class GuiTests(unittest.TestCase):
  def setUp(self):
    import tkinter as tk
    from breakfree_sheets.gui import Application
    self.root = tk.Tk()
    self.addCleanup(self.root.destroy)
    self.app = Application(self.root)
    self.root.update()
    self.temporary = tempfile.TemporaryDirectory()
    self.addCleanup(self.temporary.cleanup)
    self.path = Path(self.temporary.name) / "Workbook.bfsheet"

  def enter(self, name, kind, source):
    self.assertTrue(self.app.select(name))
    self.app.kind.set(kind.value)
    self.app.entry.delete(0, "end")
    self.app.entry.insert(0, source)
    self.assertTrue(self.app.apply())
    self.root.update()

  def test_ui_end_to_end_calculation_save_open_undo(self):
    self.enter("A1", Kind.NUMBER, "2")
    self.enter("B1", Kind.NUMBER, "3")
    self.enter("C1", Kind.FORMULA, "=A1+B1")
    self.assertEqual(self.app.editor.values["C1"], Decimal(5))
    self.assertEqual(self.app.canvas.itemcget(self.app.text_items["C1"], "text"), "5")
    self.enter("A1", Kind.NUMBER, "4")
    self.assertEqual(self.app.canvas.itemcget(self.app.text_items["C1"], "text"), "7")
    self.app.undo()
    self.assertEqual(self.app.editor.values["C1"], Decimal(5))
    self.app.redo()
    self.assertEqual(self.app.editor.values["C1"], Decimal(7))
    with patch("breakfree_sheets.gui.filedialog.asksaveasfilename", return_value=str(self.path)):
      self.assertTrue(self.app.save())
    self.assertFalse(self.app.editor.dirty)
    self.app.new()
    with patch("breakfree_sheets.gui.filedialog.askopenfilename", return_value=str(self.path)):
      self.assertTrue(self.app.open())
    self.assertEqual(self.app.editor.book.get("C1").source, "=A1+B1")
    self.enter("A1", Kind.NUMBER, "6")
    self.assertEqual(self.app.editor.values["C1"], Decimal(9))

  def test_selection_keyboard_and_visible_errors(self):
    self.app.canvas.focus_force()
    self.root.update()
    self.app.canvas.event_generate("<KeyPress-Right>")
    self.root.update()
    self.assertEqual(self.app.selected, "B1")
    self.enter("B1", Kind.FORMULA, "=1/0")
    self.assertIn("#DIV/0!", self.app.result.get())
    self.assertEqual(self.app.canvas.itemcget(self.app.text_items["B1"], "text"), "#DIV/0!")
    self.enter("B1", Kind.FORMULA, "=B1")
    self.assertIn("#CYCLE!", self.app.result.get())
    self.enter("B1", Kind.FORMULA, "=1+")
    self.assertIn("#SYNTAX!", self.app.result.get())

  def test_default_text_preserves_leading_zeros_and_pending_draft(self):
    self.app.entry.insert(0, "00123")
    self.assertTrue(self.app.draft_dirty)
    self.assertIn("*", self.root.title())
    self.app.apply()
    self.assertEqual(self.app.editor.values["A1"], "00123")
    self.assertEqual(self.app.editor.book.get("A1").kind, Kind.TEXT)

  def test_cancel_discard_and_save_choices_include_unapplied_input(self):
    self.app.source.set("pending")
    with patch("breakfree_sheets.gui.messagebox.askyesnocancel", return_value=None):
      self.app.new()
    self.assertEqual(self.app.source.get(), "pending")
    with patch("breakfree_sheets.gui.messagebox.askyesnocancel", return_value=True), \
         patch("breakfree_sheets.gui.filedialog.asksaveasfilename", return_value=""):
      self.app.new()
    self.assertEqual(self.app.source.get(), "pending")
    with patch("breakfree_sheets.gui.messagebox.askyesnocancel", return_value=True), \
         patch("breakfree_sheets.gui.filedialog.asksaveasfilename", return_value=str(self.path)):
      self.app.new()
    self.assertEqual(load(self.path).get("A1").source, "pending")
    self.assertIsNone(self.app.editor.book.get("A1"))
    self.app.source.set("discard this")
    with patch("breakfree_sheets.gui.messagebox.askyesnocancel", return_value=False):
      self.app.new()
    self.assertEqual(self.app.source.get(), "")

  def test_failed_open_preserves_current_document_and_draft(self):
    self.enter("A1", Kind.NUMBER, "2")
    self.app.source.set("3")
    self.path.write_bytes(b"corrupt")
    with patch("breakfree_sheets.gui.filedialog.askopenfilename", return_value=str(self.path)), \
         patch("breakfree_sheets.gui.messagebox.showerror") as error:
      self.assertFalse(self.app.open())
    error.assert_called_once()
    self.assertEqual(self.app.editor.values["A1"], Decimal(2))
    self.assertEqual(self.app.source.get(), "3")
    self.assertEqual(self.path.read_bytes(), b"corrupt")

  def test_open_same_file_after_save_prompt_uses_newly_saved_content(self):
    self.enter("A1", Kind.NUMBER, "2")
    with patch("breakfree_sheets.gui.filedialog.asksaveasfilename", return_value=str(self.path)):
      self.assertTrue(self.app.save())
    self.enter("A1", Kind.NUMBER, "4")
    with patch("breakfree_sheets.gui.filedialog.askopenfilename", return_value=str(self.path)), \
         patch("breakfree_sheets.gui.messagebox.askyesnocancel", return_value=True):
      self.assertTrue(self.app.open())
    self.assertEqual(self.app.editor.values["A1"], Decimal(4))
    self.assertFalse(self.app.editor.dirty)

  def test_failed_save_keeps_dirty_state_and_current_path(self):
    self.enter("A1", Kind.NUMBER, "2")
    with patch("breakfree_sheets.gui.filedialog.asksaveasfilename", return_value=str(self.path)), \
         patch("breakfree_sheets.gui.save", side_effect=FileError("disk full")), \
         patch("breakfree_sheets.gui.messagebox.showerror"):
      self.assertFalse(self.app.save())
    self.assertTrue(self.app.editor.dirty)
    self.assertIsNone(self.app.path)

  def test_invalid_number_can_be_discarded_and_does_not_change_model(self):
    self.app.kind.set(Kind.NUMBER.value)
    self.app.source.set("not a number")
    with patch("breakfree_sheets.gui.messagebox.showerror"):
      self.assertFalse(self.app.apply())
    self.assertIsNone(self.app.editor.book.get("A1"))
    with patch("breakfree_sheets.gui.messagebox.askyesnocancel", return_value=False):
      self.app.new()
    self.assertFalse(self.app.draft_dirty)

  def test_built_gui_initializes_outside_checkout_with_embedded_logo(self):
    isolated = Path(self.temporary.name)
    launcher = build(isolated)
    code = """
import runpy, sys, tkinter as tk
def checked_loop(root):
  root.update()
  assert 'BreakFree Sheets' in root.title()
  assert len(root.winfo_children()) >= 5
  assert len(root.tk.call('image', 'names')) >= 2
  root.destroy()
tk.Tk.mainloop = checked_loop
sys.argv = [sys.argv[1], '--new']
runpy.run_path(sys.argv[0], run_name='__main__')
"""
    result = subprocess.run([sys.executable, "-c", code, str(launcher)], cwd=isolated,
                            env={**os.environ, "PYTHONPATH": ""}, capture_output=True, text=True, timeout=20)
    self.assertEqual(result.returncode, 0, result.stderr)
