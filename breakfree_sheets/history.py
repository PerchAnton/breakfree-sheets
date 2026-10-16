"""Edits and history own calculation and the saved-state comparison."""

from dataclasses import dataclass

from .cells import Cell, address
from .calculation import Calculator
from .workbook import Workbook

MAX_HISTORY = 500


@dataclass(frozen=True)
class Edit:
  address: str
  before: Cell | None
  after: Cell | None


class Editor:
  def __init__(self, book: Workbook | None = None):
    self.book = book if book is not None else Workbook()
    self.calculator = Calculator()
    self.undo_stack: list[Edit] = []
    self.redo_stack: list[Edit] = []
    self._saved = dict(self.book.cells)
    self.recalculate()

  @property
  def dirty(self) -> bool:
    return self.book.cells != self._saved

  def mark_saved(self):
    self._saved = dict(self.book.cells)

  def recalculate(self):
    self.values = self.calculator.calculate(self.book.cells)

  def set(self, name: str, cell: Cell | None):
    name = address(name)
    before = self.book.get(name)
    if before == cell:
      return
    self.book.set(name, cell)
    self.undo_stack.append(Edit(name, before, cell))
    self.undo_stack = self.undo_stack[-MAX_HISTORY:]
    self.redo_stack.clear()
    self.recalculate()

  def undo(self):
    if self.undo_stack:
      edit = self.undo_stack.pop()
      self.book.set(edit.address, edit.before)
      self.redo_stack.append(edit)
      self.recalculate()

  def redo(self):
    if self.redo_stack:
      edit = self.redo_stack.pop()
      self.book.set(edit.address, edit.after)
      self.undo_stack.append(edit)
      self.recalculate()
