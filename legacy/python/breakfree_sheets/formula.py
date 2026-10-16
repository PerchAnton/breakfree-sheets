"""Bounded shunting-yard parser; output is a small postfix language, never Python."""

from dataclasses import dataclass
from decimal import Decimal
import re

from .cells import CalculationError, address, number

TOKEN = re.compile(r"\s*(?:([0-9]+(?:\.[0-9]*)?|\.[0-9]+)|([A-Za-z]+[0-9]+)|([+*/()\-]))", re.ASCII)
PRECEDENCE = {"+": 1, "-": 1, "*": 2, "/": 2, "neg": 3}
MAX_FORMULA = 1024
MAX_TOKENS = 512


@dataclass(frozen=True)
class Reference:
  address: str


@dataclass(frozen=True)
class Expression:
  instructions: tuple
  references: frozenset[str]


def parse(source: str) -> Expression:
  if len(source) > MAX_FORMULA:
    raise CalculationError("#LIMIT!", "Formula exceeds 1024 characters.")
  if not source.startswith("="):
    raise CalculationError("#SYNTAX!", "A formula must start with =.")
  text = source[1:].strip()
  output, operators, references = [], [], set()
  position, count, needs_value = 0, 0, True
  while position < len(text):
    match = TOKEN.match(text, position)
    if match is None:
      raise CalculationError("#SYNTAX!", f"Unsupported syntax at character {position + 2}.")
    position = match.end()
    count += 1
    if count > MAX_TOKENS:
      raise CalculationError("#LIMIT!", "Formula exceeds 512 tokens.")
    numeric, ref, operator = match.groups()
    if numeric is not None or ref is not None:
      if not needs_value:
        raise CalculationError("#SYNTAX!", "An operator is required between values.")
      if numeric is not None:
        try:
          output.append(number(numeric))
        except ValueError as exc:
          raise CalculationError("#NUM!", str(exc)) from exc
      else:
        try:
          target = address(ref)
        except ValueError as exc:
          raise CalculationError("#REF!", str(exc)) from exc
        references.add(target)
        output.append(Reference(target))
      needs_value = False
    elif operator == "(":
      if not needs_value:
        raise CalculationError("#SYNTAX!", "Implicit multiplication is unsupported.")
      operators.append(operator)
    elif operator == ")":
      if needs_value:
        raise CalculationError("#SYNTAX!", "A value is required before ).")
      while operators and operators[-1] != "(":
        output.append(operators.pop())
      if not operators:
        raise CalculationError("#SYNTAX!", "Unmatched closing parenthesis.")
      operators.pop()
    else:
      if needs_value:
        if operator != "-":
          raise CalculationError("#SYNTAX!", "Expected a number, reference, ( or unary minus.")
        operators.append("neg")
        continue
      while operators and operators[-1] != "(" and PRECEDENCE[operators[-1]] >= PRECEDENCE[operator]:
        output.append(operators.pop())
      operators.append(operator)
      needs_value = True
  if needs_value:
    raise CalculationError("#SYNTAX!", "Formula is empty or ends with an operator.")
  while operators:
    operator = operators.pop()
    if operator == "(":
      raise CalculationError("#SYNTAX!", "Unmatched opening parenthesis.")
    output.append(operator)
  return Expression(tuple(output), frozenset(references))
