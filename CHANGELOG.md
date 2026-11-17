# Changelog

## [Unreleased]

## [0.2.0] - 2026-11-17

### Added

- The TypeScript package `@facile/planche`, used from a git tag: the schema with the `board` and `full` profiles, the Yjs document helpers, fractional indices, text splicing and undo, taken from Toile's client.
- The canvas engine in TypeScript: camera, geometry, bounds, anchors, spatial index, snapping, transforms, hit testing, the scene read model and the tool contract, taken from Toile's client.
- `spec/fixtures/keys.json`, the key table of both profiles, checked by the Rust and the TypeScript tests so the two cannot drift.

## [0.1.0] - 2026-11-16

### Added

- The schema 1 spec, `spec/FORMAT.md`: Toile's board schema plus frames as slides and scenes, timed shapes, `video`, `audio`, `html` and encre rich text.
- Two profiles: `board` accepts and refuses exactly what Toile's server did; `full` allows the whole schema.
- The validator over a live Yjs document, checked per transaction on the shapes it touched (`schema::watch`, `check`, `check_all`), ported from Toile.
- The plain-value model and the Yjs bridge both ways, plus `Edit`s applied inside one transaction.
- The text projection: a whole document as markdown-like text and back, losslessly, with a strict reader that names the line it stopped at.
- The agent operations: `read`, `read_shapes`, `add`, `update`, `delete`, `reorder`, `replace_all`, each checked whole before it answers its edits.
