"""One bounded worksheet; no UI, history, disk or suite dependencies."""

from types import MappingProxyType
from collections.abc import Mapping

from .cells import Cell, address


class Workbook:
  def __init__(self, cells: Mapping[str, Cell] | None = None):
    self._cells = {}
    for name, cell in (cells or {}).items():
      self.set(name, cell)

  @property
  def cells(self) -> Mapping[str, Cell]:
    return MappingProxyType(self._cells)

  def get(self, name: str) -> Cell | None:
    return self._cells.get(address(name))

  def set(self, name: str, cell: Cell | None):
    name = address(name)
    if cell is None:
      self._cells.pop(name, None)
    elif not isinstance(cell, Cell):
      raise ValueError("Expected a typed Cell.")
    else:
      self._cells[name] = cell
