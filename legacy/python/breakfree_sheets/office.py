"""Temporary v1 adapter; no Office process, repository or network is needed."""

from . import __version__
from .persistence import APP_ID, FORMAT_ID


def capabilities() -> dict:
  return {
    "protocol_version": 1,
    "app_id": APP_ID,
    "app_version": __version__,
    "display_name": "BreakFree Sheets",
    "capabilities": ["new", "open"],
    "formats": [{"id": FORMAT_ID, "extensions": ["bfsheet"], "read": True, "write": True}],
  }
