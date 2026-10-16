"""The GUI and headless clients use this same calculation engine."""

from decimal import Decimal, DecimalException, DivisionByZero, localcontext
from collections.abc import Mapping

from .cells import Cell, Kind, Error, Value, CalculationError, NUMERIC_CONTEXT, number
from .dependencies import DependencyGraph
from .formula import Expression, Reference, parse


def evaluate(expression: Expression, values: Mapping[str, Value]) -> Decimal | Error:
  stack = []
  with localcontext(NUMERIC_CONTEXT):
    for instruction in expression.instructions:
      if isinstance(instruction, Decimal):
        stack.append(instruction)
      elif isinstance(instruction, Reference):
        value = values.get(instruction.address)
        if value is None:
          value = Decimal(0)
        elif isinstance(value, str):
          value = Error("#VALUE!", f"{instruction.address} contains text, not a number.")
        stack.append(value)
      else:
        right = stack.pop()
        left = None if instruction == "neg" else stack.pop()
        if isinstance(left, Error) or isinstance(right, Error):
          stack.append(left if isinstance(left, Error) else right)
          continue
        try:
          if instruction == "neg":
            result = -right
          elif instruction == "+":
            result = left + right
          elif instruction == "-":
            result = left - right
          elif instruction == "*":
            result = left * right
          elif instruction == "/":
            if right.is_zero():
              raise DivisionByZero
            result = left / right
          else:
            raise AssertionError("Unknown compiled instruction.")
          stack.append(result)
        except DivisionByZero:
          stack.append(Error("#DIV/0!", "Division by zero."))
        except DecimalException:
          stack.append(Error("#NUM!", "Numeric overflow, underflow or invalid operation."))
  return stack[0]


class Calculator:
  def __init__(self):
    self.graph = DependencyGraph({})

  def calculate(self, cells: Mapping[str, Cell]) -> dict[str, Value]:
    compiled = {}
    dependencies = {}
    for name, cell in cells.items():
      if cell.kind == Kind.FORMULA:
        try:
          compiled[name] = parse(cell.source)
          dependencies[name] = compiled[name].references
        except CalculationError as exc:
          compiled[name] = exc.error
          dependencies[name] = frozenset()
      else:
        dependencies[name] = frozenset()
    self.graph = DependencyGraph(dependencies)
    ordered, blocked = self.graph.order()
    values = {}
    for name in ordered:
      cell = cells[name]
      if cell.kind == Kind.TEXT:
        values[name] = cell.source
      elif cell.kind == Kind.NUMBER:
        values[name] = number(cell.source)
      elif isinstance(compiled[name], Error):
        values[name] = compiled[name]
      else:
        values[name] = evaluate(compiled[name], values)
    for name in blocked:
      values[name] = Error("#CYCLE!", "Cell participates in or depends on a circular reference.")
    return values
