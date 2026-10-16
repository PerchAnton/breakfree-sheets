# Brand asset provenance

The active Rust application embeds the approved **dedicated Sheets mark** from
[Sheets commit e7de5a9](https://github.com/PerchAnton/breakfree-sheets/commit/e7de5a9745c5b29527e38c9022b58d6b53a9b686),
not the generic Office icon. The exact approved files and guidance are preserved
under [docs/brand](brand/README.md). No logo was invented or redrawn.

| Asset | Origin and processing |
| :--- | :--- |
| `assets/breakfree-sheets.webp` | Exact bytes of `docs/brand/breakfree-sheets-transparent.webp` from e7de5a9, embedded with Rust include_bytes and decoded to RGBA at runtime |
| `docs/brand/breakfree-sheets-transparent.webp` | Unmodified approved 192 × 192 WebP, transparent exterior around the purple tile |
| `legacy/python/breakfree_sheets/assets/` | Historical shared Office mark, preserved only for the superseded prototype |

Active asset SHA-256: `b8da96ad1623ef1bb647907b39fabe93c3c30502c5bed738e4e3bcc667fed6bb`.

Tests verify zero alpha at all four corners and opaque interior artwork. Actual
light-background native captures are linked from [UI_SPEC](UI_SPEC.md). The
transparent image is not flattened onto black or white. The small raster remains
a display derivative, not a high-resolution/vector platform master.

The shared palette is pinned to Office BRAND.md at
[cfaaf419](https://github.com/PerchAnton/breakfree-office/blob/cfaaf419e84d9981acd421d130113ca2265061fd/BRAND.md):
teal `#248A83`, aubergine `#241337`, light neutral chrome and a white worksheet.
The owner has not selected a software/brand redistribution license; this file
grants no unrelated reuse rights.
