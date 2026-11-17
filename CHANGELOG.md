# Changelog

## [Unreleased]

## [0.3.1] - 2026-11-17

### Fixed

- TypeScript `allowedKey` allows `head` on a `line`, as the spec and the Rust validator always did.
- The spec gives the `valign` default per type: `middle` for `rect`, `ellipse` and `diamond`, `top` for the rest, as Diapo draws them.
- The spec says when a schema change needs a new version: only when it refuses or reinterprets a document valid before.

## [0.3.0] - 2026-11-17

### Added

- `style.valign` under the `full` profile: `top`, `middle` or `bottom`, where a shape's text sits in its box. The validator refuses any other value, and `board` still refuses the key.
- `style.align: "justify"` under the `full` profile. The TypeScript `cleanStyle(value, profile)` keeps `valign` and `justify` only for `full`; its default stays `board`.
- The projection writes `style.valign` after `style.align`.

### Changed

- `Rule::Style` now carries the `Profile` whose style fields it allows (`Rule::Style(Profile)`), and `Style` gains a `valign` field.

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
