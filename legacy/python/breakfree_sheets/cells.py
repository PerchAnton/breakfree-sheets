"""Stored content and calculated values are deliberately separate types."""

from dataclasses import dataclass
from decimal import Context, Decimal, DecimalException, ROUND_HALF_EVEN, Underflow
from enum import Enum
import re

ROWS = 100
COLS = 26
MAX_SOURCE = 4096
NUMBER = re.compile(r"[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)\Z", re.ASCII)
ADDRESS = re.compile(r"[A-Z](?:[1-9][0-9]?|100)\Z", re.ASCII)
# Create a fresh context for every calculation; caller/global context cannot alter it.
NUMERIC_CONTEXT = Context(prec=28, rounding=ROUND_HALF_EVEN, Emin=-999, Emax=999)
NUMERIC_CONTEXT.traps[Underflow] = True


class Kind(str, Enum):
  TEXT = "text"
  NUMBER = "number"
  FORMULA = "formula"


@dataclass(frozen=True)
class Error:
  code: str
  message: str


class CalculationError(ValueError):
  def __init__(self, code: str, message: str):
    super().__init__(message)
    self.error = Error(code, message)


def address(value: str) -> str:
  if not isinstance(value, str) or not value.isascii():
    raise ValueError("Cell address must be an ASCII string.")
  value = value.upper()
  if not ADDRESS.fullmatch(value):
    raise ValueError("Cell reference must be A1 through Z100.")
  return value


def number(source: str) -> Decimal:
  if not isinstance(source, str) or len(source) > MAX_SOURCE or not NUMBER.fullmatch(source):
    raise ValueError("Use a decimal number with a dot; no separators or exponent notation.")
  try:
    value = Decimal(source)
    if len(value.as_tuple().digits) > 28 or value.adjusted() < -999 or value.adjusted() > 999:
      raise ValueError("Numbers allow at most 28 significant digits and exponents -999 to 999.")
    return value
  except DecimalException as exc:
    raise ValueError("Invalid decimal number.") from exc


@dataclass(frozen=True)
class Cell:
  kind: Kind
  source: str

  def __post_init__(self):
    if not isinstance(self.kind, Kind):
      raise ValueError("Cell kind must be text, number or formula.")
    if not isinstance(self.source, str) or len(self.source) > MAX_SOURCE:
      raise ValueError(f"Cell content must be a string of at most {MAX_SOURCE} characters.")
    try:
      self.source.encode("utf-8")
    except UnicodeError as exc:
      raise ValueError("Cell content must contain valid Unicode text.") from exc
    if self.kind == Kind.NUMBER:
      number(self.source)
    # Invalid formulas remain editable and persist with a visible calculation error.


Value = Decimal | str | Error | None


def display(value: Value) -> str:
  if value is None:
    return ""
  if isinstance(value, Error):
    return value.code
  if isinstance(value, Decimal):
    if value.is_zero():
      return "0"
    # Display never changes source or rounds a stored value.
    text = format(value, "f") if -6 <= value.adjusted() < 28 else str(value)
    return text.rstrip("0").rstrip(".") if "." in text and "E" not in text else text
  return value
