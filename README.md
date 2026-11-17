# planche

The Facile canvas document: one format for a Toile board, a Diapo deck and a Clap video.
Shapes on a canvas in a Yjs document, with frames as slides or scenes, and encre rich text.

## What it does

- Defines the document in [`spec/FORMAT.md`](spec/FORMAT.md): roots, types, keys, limits
- Validates a live Yjs document under a profile (`board` for Toile, `full` for the rest)
- Reads and writes the document as plain values
- Projects the document as text and reads it back losslessly, for the CLI and agents
- Plans agent operations that touch only the shapes they name
- Gives browser apps the same schema, the Yjs helpers and the canvas engine in TypeScript

## Install

```toml
planche = { git = "https://github.com/FacileStudio/planche", tag = "v0.3.1" }
```

```json
"@facile/planche": "github:FacileStudio/planche#v0.3.1"
```

## Develop

```sh
mise run check
```

## License

MIT
