"""Small Tk editor. All calculated results come from history.Editor's engine."""

import base64
from importlib.resources import files
from pathlib import Path
import tkinter as tk
from tkinter import filedialog, messagebox, ttk

from . import __version__
from .cells import COLS, ROWS, Cell, Error, Kind, display
from .history import Editor
from .persistence import FileError, load, save

TEAL = "#248A83"
AUBERGINE = "#241337"
PAPER = "#FFFDF8"
WIDTH, HEIGHT, GUTTER = 116, 28, 48


class Application:
  def __init__(self, root: tk.Tk, book=None, path=None):
    self.root = root
    self.editor = Editor(book)
    self.path = Path(path) if path else None
    self.selected = "A1"
    self.baseline = (Kind.TEXT.value, "")
    self.kind = tk.StringVar(value=Kind.TEXT.value)
    self.source = tk.StringVar()
    self.location = tk.StringVar(value="A1")
    self.status = tk.StringVar()
    self.result = tk.StringVar()
    root.geometry("1120x760")
    root.minsize(760, 480)
    style = ttk.Style(root)
    style.theme_use("clam")
    style.configure("TButton", padding=(10, 7))
    style.configure("Accent.TButton", background=TEAL, foreground="white")
    style.map("Accent.TButton", background=[("active", "#1B6863")])

    header = tk.Frame(root, background=AUBERGINE, padx=16, pady=10)
    header.pack(fill="x")
    data = files("breakfree_sheets").joinpath("assets/breakfree-icon.png").read_bytes()
    self.logo = tk.PhotoImage(data=base64.b64encode(data)).subsample(4)
    root.iconphoto(True, self.logo)
    tk.Label(header, image=self.logo, background=AUBERGINE).pack(side="left", padx=(0, 12))
    tk.Label(header, text="BreakFree Sheets", font=("TkDefaultFont", 19, "bold"),
             background=AUBERGINE, foreground=PAPER).pack(side="left")
    tk.Label(header, text=f"Your numbers. Your freedom.   •   Prototype {__version__}",
             background=AUBERGINE, foreground=PAPER).pack(side="right")

    toolbar = ttk.Frame(root, padding=10)
    toolbar.pack(fill="x")
    for label, command in [("New", self.new), ("Open…", self.open), ("Save", self.save),
                           ("Save As…", self.save_as), ("Undo", self.undo),
                           ("Redo", self.redo), ("Clear cell", self.clear)]:
      ttk.Button(toolbar, text=label, command=command,
                 style="Accent.TButton" if label == "Save" else "TButton").pack(side="left", padx=(0, 6))

    editbar = ttk.Frame(root, padding=(10, 0, 10, 8))
    editbar.pack(fill="x")
    ttk.Label(editbar, textvariable=self.location, width=6, font=("TkDefaultFont", 12, "bold")).pack(side="left")
    self.type_picker = ttk.Combobox(editbar, textvariable=self.kind, values=[kind.value for kind in Kind],
                                    state="readonly", width=9)
    self.type_picker.pack(side="left", padx=(0, 8))
    self.entry = ttk.Entry(editbar, textvariable=self.source)
    self.entry.pack(side="left", fill="x", expand=True, padx=(0, 8))
    ttk.Button(editbar, text="Apply ↵", command=self.apply, style="Accent.TButton").pack(side="left")
    ttk.Label(root, text="Choose Text for identifiers such as 00123. Number uses a decimal dot. Formula starts with =.",
              padding=(10, 0, 10, 8)).pack(anchor="w")

    frame = ttk.Frame(root)
    frame.pack(fill="both", expand=True, padx=10)
    frame.rowconfigure(0, weight=1)
    frame.columnconfigure(0, weight=1)
    self.canvas = tk.Canvas(frame, background=PAPER, highlightthickness=0, takefocus=True,
                            scrollregion=(0, 0, GUTTER + COLS * WIDTH, (ROWS + 1) * HEIGHT))
    self.canvas.grid(row=0, column=0, sticky="nsew")
    horizontal = ttk.Scrollbar(frame, orient="horizontal", command=self.canvas.xview)
    horizontal.grid(row=1, column=0, sticky="ew")
    vertical = ttk.Scrollbar(frame, orient="vertical", command=self.canvas.yview)
    vertical.grid(row=0, column=1, sticky="ns")
    self.canvas.configure(xscrollcommand=horizontal.set, yscrollcommand=vertical.set)
    self.text_items = {}
    for column in range(COLS):
      x = GUTTER + column * WIDTH
      self.canvas.create_rectangle(x, 0, x + WIDTH, HEIGHT, fill="#E4EFEC", outline="#C8D6D2")
      self.canvas.create_text(x + WIDTH / 2, HEIGHT / 2, text=chr(65 + column), fill=AUBERGINE)
    for row in range(1, ROWS + 1):
      y = row * HEIGHT
      self.canvas.create_rectangle(0, y, GUTTER, y + HEIGHT, fill="#E4EFEC", outline="#C8D6D2")
      self.canvas.create_text(GUTTER / 2, y + HEIGHT / 2, text=str(row), fill=AUBERGINE)
      for column in range(COLS):
        x = GUTTER + column * WIDTH
        self.canvas.create_rectangle(x, y, x + WIDTH, y + HEIGHT, fill=PAPER, outline="#E0E4DF")
        name = f"{chr(65 + column)}{row}"
        self.text_items[name] = self.canvas.create_text(x + 7, y + HEIGHT / 2, anchor="w", fill=AUBERGINE)
    self.selection = self.canvas.create_rectangle(0, 0, 0, 0, outline=TEAL, width=3)
    ttk.Label(root, textvariable=self.result, padding=(10, 8), wraplength=1050).pack(fill="x")
    ttk.Label(root, textvariable=self.status, padding=(10, 0, 10, 8)).pack(fill="x")

    self.canvas.bind("<Button-1>", self.click)
    self.canvas.bind("<Double-Button-1>", lambda event: self.entry.focus_set())
    for key, delta in {"Left": (-1, 0), "Right": (1, 0), "Up": (0, -1), "Down": (0, 1),
                       "Tab": (1, 0), "Return": (0, 1)}.items():
      self.canvas.bind(f"<{key}>", lambda event, d=delta: self.move(*d))
    self.canvas.bind("<Delete>", lambda event: self.clear())
    self.canvas.bind("<BackSpace>", lambda event: self.clear())
    self.canvas.bind("<F2>", lambda event: self.entry.focus_set())
    self.canvas.bind("<MouseWheel>", self.scroll)
    self.canvas.bind("<Button-4>", lambda event: self.canvas.yview_scroll(-3, "units"))
    self.canvas.bind("<Button-5>", lambda event: self.canvas.yview_scroll(3, "units"))
    self.entry.bind("<Return>", lambda event: self.apply())
    self.entry.bind("<Escape>", lambda event: self.cancel_draft())
    modifier = "Command" if root.tk.call("tk", "windowingsystem") == "aqua" else "Control"
    for key, action in {"s": self.save, "o": self.open, "n": self.new, "z": self.undo, "y": self.redo}.items():
      root.bind(f"<{modifier}-{key}>", lambda event, fn=action: self.shortcut(fn))
    self.kind.trace_add("write", lambda *_: self.update_status())
    self.source.trace_add("write", lambda *_: self.update_status())
    root.protocol("WM_DELETE_WINDOW", self.close)
    self.refresh()
    self.show_selected()
    self.canvas.focus_set()

  @property
  def draft_dirty(self):
    return (self.kind.get(), self.source.get()) != self.baseline

  def update_status(self):
    dirty = self.editor.dirty or self.draft_dirty
    filename = self.path.name if self.path else "Untitled"
    self.root.title(f"{'* ' if dirty else ''}{filename} — BreakFree Sheets")
    self.status.set(f"Sheet1  •  A1–Z100  •  {'Unsaved changes' if dirty else 'No unsaved changes'}"
                    + ("  •  Entry pending: Apply or Escape" if self.draft_dirty else ""))

  def refresh(self):
    for name, item in self.text_items.items():
      value = self.editor.values.get(name)
      text = display(value).replace("\n", "↵").replace("\r", "")
      self.canvas.itemconfigure(item, text=text[:14] + "…" if len(text) > 15 else text,
                                 fill="#A32626" if isinstance(value, Error) else AUBERGINE)
    self.update_status()
    self.update_result()

  def update_result(self):
    value = self.editor.values.get(self.selected)
    text = f"{self.selected}: {display(value)}"
    if isinstance(value, Error):
      text += f" — {value.message}"
    self.result.set(text[:600])

  def show_selected(self):
    cell = self.editor.book.get(self.selected)
    self.baseline = (cell.kind.value, cell.source) if cell else (Kind.TEXT.value, "")
    self.kind.set(self.baseline[0])
    self.source.set(self.baseline[1])
    self.location.set(self.selected)
    column, row = ord(self.selected[0]) - 65, int(self.selected[1:])
    x, y = GUTTER + column * WIDTH, row * HEIGHT
    self.canvas.coords(self.selection, x + 1, y + 1, x + WIDTH - 1, y + HEIGHT - 1)
    self.canvas.tag_raise(self.selection)
    self.update_status()
    self.update_result()

  def apply(self):
    if not self.draft_dirty:
      return True
    try:
      cell = Cell(Kind(self.kind.get()), self.source.get())
      self.editor.set(self.selected, cell)
    except ValueError as exc:
      messagebox.showerror("Cannot apply cell", str(exc), parent=self.root)
      return False
    self.baseline = (cell.kind.value, cell.source)
    self.refresh()
    return True

  def select(self, name):
    if not self.apply():
      return False
    self.selected = name
    self.show_selected()
    self.canvas.focus_set()
    return True

  def click(self, event):
    column = int((self.canvas.canvasx(event.x) - GUTTER) // WIDTH)
    row = int(self.canvas.canvasy(event.y) // HEIGHT)
    if 0 <= column < COLS and 1 <= row <= ROWS:
      self.select(f"{chr(65 + column)}{row}")

  def move(self, dx, dy):
    column = min(COLS - 1, max(0, ord(self.selected[0]) - 65 + dx))
    row = min(ROWS, max(1, int(self.selected[1:]) + dy))
    if self.select(f"{chr(65 + column)}{row}"):
      x, y = GUTTER + column * WIDTH, row * HEIGHT
      width, height = self.canvas.winfo_width(), self.canvas.winfo_height()
      left, top = self.canvas.canvasx(0), self.canvas.canvasy(0)
      if x < left or x + WIDTH > left + width:
        self.canvas.xview_moveto(max(0, x + WIDTH - width) / (GUTTER + COLS * WIDTH))
      if y < top or y + HEIGHT > top + height:
        self.canvas.yview_moveto(max(0, y + HEIGHT - height) / ((ROWS + 1) * HEIGHT))
    return "break"

  def scroll(self, event):
    self.canvas.yview_scroll(-1 if event.delta > 0 else 1, "units")

  def shortcut(self, action):
    action()
    return "break"

  def cancel_draft(self):
    self.show_selected()
    self.canvas.focus_set()
    return "break"

  def clear(self):
    self.editor.set(self.selected, None)
    self.show_selected()
    self.refresh()

  def undo(self):
    if self.draft_dirty:
      self.cancel_draft()
    else:
      self.editor.undo()
      self.show_selected()
      self.refresh()

  def redo(self):
    if self.draft_dirty:
      return
    self.editor.redo()
    self.show_selected()
    self.refresh()

  def confirm_replace(self):
    if not (self.editor.dirty or self.draft_dirty):
      return True
    choice = messagebox.askyesnocancel("Unsaved changes", "Save changes before continuing?", parent=self.root)
    if choice is None:
      return False
    return self.save() if choice else True

  def replace(self, book=None, path=None):
    self.editor = Editor(book)
    self.path = Path(path) if path else None
    self.selected = "A1"
    self.canvas.xview_moveto(0)
    self.canvas.yview_moveto(0)
    self.show_selected()
    self.refresh()

  def new(self):
    if self.confirm_replace():
      self.replace()

  def open(self):
    selected = filedialog.askopenfilename(parent=self.root, title="Open BreakFree Sheets workbook",
                                          filetypes=[("BreakFree Sheets", "*.bfsheet"), ("All files", "*")])
    if not selected:
      return False
    try:
      book = load(selected)
    except FileError as exc:
      messagebox.showerror("Cannot open workbook", str(exc), parent=self.root)
      return False
    if self.confirm_replace():
      # Saving the current book may have replaced the selected file. Reload it
      # after the prompt so an earlier disk snapshot never becomes a clean book.
      try:
        book = load(selected)
      except FileError as exc:
        messagebox.showerror("Cannot open workbook", str(exc), parent=self.root)
        return False
      self.replace(book, selected)
      return True
    return False

  def save_as(self):
    return self.save(choose=True)

  def save(self, choose=False):
    if not self.apply():
      return False
    path = self.path
    if choose or path is None:
      selected = filedialog.asksaveasfilename(parent=self.root, title="Save BreakFree Sheets workbook",
                                              defaultextension=".bfsheet", initialfile=path.name if path else "Untitled.bfsheet",
                                              filetypes=[("BreakFree Sheets", "*.bfsheet")])
      if not selected:
        return False
      path = Path(selected)
    try:
      save(self.editor.book, path)
    except FileError as exc:
      messagebox.showerror("Cannot save workbook", str(exc), parent=self.root)
      return False
    self.path = path
    self.editor.mark_saved()
    self.update_status()
    return True

  def close(self):
    if self.confirm_replace():
      self.root.destroy()


def run(book=None, path=None):
  try:
    root = tk.Tk()
  except tk.TclError as exc:
    raise RuntimeError(str(exc)) from exc
  Application(root, book, path)
  root.mainloop()
