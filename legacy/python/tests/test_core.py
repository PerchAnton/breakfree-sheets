from decimal import Decimal, localcontext
import unittest

from breakfree_sheets.cells import Cell, Error, Kind, address, display
from breakfree_sheets.formula import parse
from breakfree_sheets.cells import CalculationError
from breakfree_sheets.history import Editor
from breakfree_sheets.workbook import Workbook


def numeric(source):
  return Cell(Kind.NUMBER, str(source))


def formula(source):
  return Cell(Kind.FORMULA, source)


class ParserTests(unittest.TestCase):
  def calculate(self, source):
    editor = Editor()
    editor.set("A1", formula(source))
    return editor.values["A1"]

  def test_precedence_associativity_parentheses_and_negation(self):
    cases = {"=2+3*4": "14", "=(2+3)*4": "20", "=12/3/2": "2", "=8-3-2": "3",
             "=-2*3": "-6", "=-(2+3)": "-5", "=--2": "2", "=2*-3": "-6",
             "=2--3": "5", "= 1 + .5 ": "1.5", "=1/-2": "-0.5"}
    for source, expected in cases.items():
      with self.subTest(source=source):
        self.assertEqual(self.calculate(source), Decimal(expected))

  def test_parser_extracts_case_insensitive_references(self):
    self.assertEqual(parse("= a1 + B100 * z99").references, {"A1", "B100", "Z99"})

  def test_invalid_syntax_is_explicit(self):
    for source in ["", "1+2", "=", "=1+", "=()", "=(1", "=1)", "=1 2", "=2(3)",
                   "=1/**2", "=+2", "=SUM(A1)", "=__import__('os')",
                   "=[book]A1", "=https://example.org", "=WEBSERVICE(A1)", "=1e3", "=1,5"]:
      with self.subTest(source=source):
        result = self.calculate(source)
        self.assertIsInstance(result, Error)
        self.assertEqual(result.code, "#SYNTAX!")

  def test_bad_reference_is_not_a_blank(self):
    for source in ["=A0", "=A101", "=AA1", "=A01", "=Sheet2!A1"]:
      self.assertEqual(self.calculate(source).code, "#REF!")

  def test_parser_limits_are_visible_and_iterative(self):
    self.assertEqual(self.calculate("=" + "1+" * 300 + "1").code, "#LIMIT!")
    self.assertEqual(self.calculate("=" + "1" * 1025).code, "#LIMIT!")
    self.assertEqual(self.calculate("=" + "(" * 200 + "2" + ")" * 200), Decimal(2))
    with self.assertRaises(CalculationError):
      parse("=SUM(A1)")


class CalculationTests(unittest.TestCase):
  def setUp(self):
    self.editor = Editor()

  def test_required_workflow_and_undo_input(self):
    editor = self.editor
    editor.set("A1", numeric(2))
    editor.set("B1", numeric(3))
    editor.set("C1", formula("=A1+B1"))
    self.assertEqual(editor.values["C1"], Decimal(5))
    editor.set("A1", numeric(4))
    self.assertEqual(editor.values["C1"], Decimal(7))
    editor.undo()
    self.assertEqual(editor.values["C1"], Decimal(5))
    editor.redo()
    self.assertEqual(editor.values["C1"], Decimal(7))

  def test_diamond_dependency_recalculation(self):
    editor = self.editor
    for name, cell in [("D1", formula("=B1+C1")), ("C1", formula("=A1*3")),
                       ("B1", formula("=A1*2")), ("A1", numeric(2))]:
      editor.set(name, cell)
    self.assertEqual(editor.values["D1"], Decimal(10))
    editor.set("A1", numeric(4))
    self.assertEqual(editor.values["D1"], Decimal(20))
    self.assertEqual(editor.calculator.graph.dependencies["D1"], {"B1", "C1"})

  def test_long_chain_does_not_use_python_recursion(self):
    names = [f"{chr(65 + column)}{row}" for column in range(26) for row in range(1, 101)]
    cells = {names[0]: numeric(1)}
    cells.update({name: formula(f"={previous}+1") for previous, name in zip(names, names[1:])})
    editor = Editor(Workbook(cells))
    self.assertEqual(editor.values[names[-1]], Decimal(2600))
    editor.set(names[0], numeric(2))
    self.assertEqual(editor.values[names[-1]], Decimal(2601))

  def test_direct_cycle_and_recovery(self):
    editor = self.editor
    editor.set("A1", formula("=A1+1"))
    self.assertEqual(editor.values["A1"].code, "#CYCLE!")
    editor.set("A1", numeric(2))
    self.assertEqual(editor.values["A1"], Decimal(2))
    editor.undo()
    self.assertEqual(editor.values["A1"].code, "#CYCLE!")

  def test_indirect_cycle_descendant_and_unrelated_cell(self):
    editor = self.editor
    for name, source in {"A1": "=B1+1", "B1": "=C1", "C1": "=A1", "D1": "=B1*2", "E1": "=2+3"}.items():
      editor.set(name, formula(source))
    for name in ["A1", "B1", "C1", "D1"]:
      self.assertEqual(editor.values[name].code, "#CYCLE!")
    self.assertEqual(editor.values["E1"], Decimal(5))
    editor.set("C1", numeric(3))
    self.assertEqual(editor.values["D1"], Decimal(6))

  def test_blank_zero_and_text_never_coerced(self):
    editor = self.editor
    editor.set("B1", formula("=A1+1"))
    self.assertEqual(editor.values["B1"], Decimal(1))
    editor.set("A1", Cell(Kind.TEXT, "00123"))
    self.assertEqual(editor.values["A1"], "00123")
    self.assertEqual(editor.values["B1"].code, "#VALUE!")
    editor.set("A1", Cell(Kind.TEXT, ""))
    self.assertEqual(editor.values["B1"].code, "#VALUE!")
    editor.set("A1", None)
    self.assertEqual(editor.values["B1"], Decimal(1))

  def test_error_propagation_and_recovery(self):
    editor = self.editor
    editor.set("A1", formula("=1/0"))
    editor.set("B1", formula("=A1+1"))
    self.assertEqual(editor.values["B1"].code, "#DIV/0!")
    editor.set("A1", formula("=1+"))
    self.assertEqual(editor.values["B1"].code, "#SYNTAX!")
    editor.set("A1", numeric(2))
    self.assertEqual(editor.values["B1"], Decimal(3))

  def test_zero_divided_by_zero_and_negative_zero(self):
    self.editor.set("A1", formula("=0/0"))
    self.assertEqual(self.editor.values["A1"].code, "#DIV/0!")
    self.editor.set("A1", formula("=1/-0"))
    self.assertEqual(self.editor.values["A1"].code, "#DIV/0!")

  def test_decimal_precision_and_caller_context_independence(self):
    with localcontext() as context:
      context.prec = 2
      self.editor.set("A1", formula("=0.1+0.2"))
      self.editor.set("B1", formula("=1/3"))
      self.editor.set("C1", formula("=1234+1"))
    self.assertEqual(self.editor.values["A1"], Decimal("0.3"))
    self.assertEqual(self.editor.values["B1"], Decimal("0.3333333333333333333333333333"))
    self.assertEqual(self.editor.values["C1"], Decimal(1235))

  def test_numeric_bounds_and_half_even(self):
    for value in ["NaN", "Infinity", "1,5", "1e4", "1" * 29, " 2 "]:
      with self.subTest(value=value), self.assertRaises(ValueError):
        numeric(value)
    self.editor.set("A1", formula("=0.1234567890123456789012345678+0.00000000000000000000000000005"))
    self.assertEqual(self.editor.values["A1"], Decimal("0.1234567890123456789012345678"))
    self.editor.set("A1", formula("=" + "1" * 29))
    self.assertEqual(self.editor.values["A1"].code, "#NUM!")

  def test_numeric_overflow_and_underflow_are_errors(self):
    cells = {"A1": numeric(1)}
    # Reach the exponent bounds through a chain of representable operations.
    for row in range(2, 50):
      cells[f"A{row}"] = formula(f"=A{row-1}*1000000000000000000000000000")
      cells[f"B{row}"] = formula("=1/1000000000000000000000000000" if row == 2
                                  else f"=B{row-1}/1000000000000000000000000000")
    editor = Editor(Workbook(cells))
    self.assertEqual(editor.values["A49"].code, "#NUM!")
    self.assertEqual(editor.values["B49"].code, "#NUM!")

  def test_display_does_not_change_source(self):
    cell = numeric("1.2300")
    self.editor.set("A1", cell)
    self.assertEqual(display(self.editor.values["A1"]), "1.23")
    self.assertEqual(self.editor.book.get("A1").source, "1.2300")
    self.assertEqual(display(Decimal("-0")), "0")

  def test_formula_replacement_rebuilds_dependencies(self):
    self.editor.set("A1", formula("=B1"))
    self.editor.set("A1", formula("=C1"))
    self.assertEqual(self.editor.calculator.graph.dependencies["A1"], {"C1"})
    self.editor.set("C1", numeric(7))
    self.assertEqual(self.editor.values["A1"], Decimal(7))


class HistoryAndModelTests(unittest.TestCase):
  def test_undo_redo_branch_and_saved_state(self):
    editor = Editor()
    self.assertFalse(editor.dirty)
    editor.set("A1", numeric(2))
    editor.mark_saved()
    editor.set("A1", formula("=3*2"))
    self.assertTrue(editor.dirty)
    editor.undo()
    self.assertFalse(editor.dirty)
    self.assertEqual(editor.book.get("A1"), numeric(2))
    editor.redo()
    self.assertEqual(editor.values["A1"], Decimal(6))
    editor.undo()
    editor.set("B1", Cell(Kind.TEXT, "new branch"))
    editor.redo()
    self.assertEqual(editor.book.get("A1"), numeric(2))
    self.assertFalse(editor.redo_stack)

  def test_delete_and_empty_history(self):
    editor = Editor()
    editor.undo()
    editor.redo()
    editor.set("A1", numeric(2))
    editor.set("A1", None)
    self.assertFalse(editor.dirty)
    editor.undo()
    self.assertEqual(editor.values["A1"], Decimal(2))
    editor.redo()
    self.assertNotIn("A1", editor.values)

  def test_addresses_types_and_mapping_protection(self):
    self.assertEqual(address("z100"), "Z100")
    for invalid in ["A0", "AA1", "A101", " A1", "A01", "ſ1", 42]:
      with self.subTest(invalid=invalid), self.assertRaises(ValueError):
        address(invalid)
    with self.assertRaises(ValueError):
      Cell("text", "content")
    with self.assertRaises(ValueError):
      Cell(Kind.TEXT, "\ud800")
    with self.assertRaises(ValueError):
      Cell(Kind.TEXT, "x" * 4097)
    with self.assertRaises(TypeError):
      Workbook().cells["A1"] = numeric(2)
