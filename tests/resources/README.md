# Test Resources

This directory holds test fixture assets for the Optima test suite.

## Directory Structure

```
tests/resources/
├── README.md          # This file
├── sample.html        # Sample HTML document for loading tests
├── sample.css         # Sample CSS stylesheet for injection tests
├── fonts/             # (future) Font files for font-face testing
├── images/            # (future) Image assets for image-loading tests
└── scripts/           # (future) JavaScript snippets for JS-enabled tests
```

## Adding Fixtures

When adding test fixtures, follow these guidelines:

1. **Keep files small.** Test HTML should be minimal — just enough to exercise
   the code path you are testing.
2. **Use descriptive names.** E.g. `minimal.html`, `complex-layout.html`,
   `font-face.css`.
3. **No external resources.** Fixtures should be self-contained.  Do not
   reference CDN URLs or external assets in test HTML; the test environment
   may not have network access.
4. **Document non-obvious files.** If a fixture exists for an edge case,
   add a comment at the top of the file or a README note.

## Sample Files

- **`sample.html`** — A basic HTML5 document with a heading, paragraph, and
  a linked stylesheet reference.  Useful for testing `load_html` and document
  structure inspection.
- **`sample.css`** — A small CSS file with rules for common elements.
  Useful for testing `inject_css` and style parsing.
