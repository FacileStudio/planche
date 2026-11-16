# planche format, schema 1

Normative. planche is the Facile canvas document: a Toile board, a Diapo deck, a Clap
video. When an implementation and this page disagree, the implementation is wrong. A change
to this page that changes what a valid document is, is a schema version bump (section 1.8).

The key words MUST, MUST NOT and MAY are used as in RFC 2119.

## 1. Document model

### 1.1 Roots

A document is a Yjs document with exactly two roots, both `Y.Map`:

- `meta`: document settings.
- `shapes`: every shape, keyed by shape id.

Any other root, or list content in either map, makes the document invalid.

### 1.2 Profiles

An app declares one profile, and its server refuses what the profile leaves out.

| Profile | For | Allows |
|---|---|---|
| `board` | Toile | The schema 1 board, exactly as Toile shipped it: the types and keys marked *board* below, `text` as `Y.Text` only, `meta.schema` only |
| `full` | Diapo, Clap, a future design app | Everything on this page |

A `board` document is a valid `full` document. The `board` profile exists so that Toile's
server keeps refusing what it refused before planche.

### 1.3 `meta`

| Key | Profile | Value |
|---|---|---|
| `schema` | board | The number `1`. Required |
| `size` | full | `"<w>x<h>"`, two integers in 1..16384, the frame size in pixels (`"1920x1080"`) |

### 1.4 Common keys

Every shape is a `Y.Map` with a string `type` and numbers `x` and `y`. An `audio` shape is the
only one that MAY leave out `x` and `y`. Every key a shape holds MUST be allowed for its type by
its profile.

| Key | Profile | Value |
|---|---|---|
| `type` | board | One of the types in 1.5 |
| `x`, `y`, `w`, `h` | board | Number. Board coordinates, or frame pixels for a shape inside a fixed-size frame |
| `rot` | board | Number, radians around the box centre |
| `z` | board | String, a fractional index; paint order among siblings, lowest first |
| `parent` | board | String, the id of the frame that holds this shape. It MAY name a missing shape |
| `style` | board | Plain object with any of `fill`, `stroke`, `font`, `align` (strings) and `width`, `size` (numbers) |
| `locked` | board | Boolean |
| `step` | full | Integer in 0..1000, the reveal step inside a frame; 0 is shown from the start |
| `enter` | full | String of at most 64 characters, an animation preset name |
| `start`, `duration` | full | Number in 0..86400, seconds from the frame's start |
| `track` | full | Integer in 0..64. The operations refuse two timed shapes of one frame that overlap in time on one track; the validator does not, so a live drag may pass through |
| `fit` | full | `"cover"`, `"contain"` or `"fill"` |
| `opacity` | full | Number in 0..1 |

### 1.5 Types

| Type | Profile | Keys beyond the common ones |
|---|---|---|
| `sticky`, `text`, `rect`, `ellipse`, `diamond` | board | `text` |
| `line`, `arrow` | board | `points` (stride 2), `from`, `to` (bindings), `head` (string) |
| `ink` | board | `points` (stride 3: `x, y, pressure`) |
| `image` | board | `asset` (string) |
| `card` | board | `url`, `title`, `kind`, `image`, `description`, `favicon`, `site` (strings) |
| `frame` | board | `name` (string) |
| `frame` | full | also `order` (string, fractional index, the slide or scene order), `notes` (rich text), `layout` (string of at most 64 characters), `fixed_duration` (boolean) |
| `video` | full | `asset` (string), `media_in` (number in 0..86400), `volume` (number in 0..1) |
| `audio` | full | `asset`, `media_in`, `volume`, as for `video`. Never drawn |
| `html` | full | `html` (string of at most 100,000 characters, kept verbatim) |

For Diapo and Clap, `asset` is a relative asset path such as `assets/x.mp4`. For Toile it is a
Toile asset id.

A frame's `duration` is its length in seconds. When `fixed_duration` is false or missing, the
frame lasts until its last timed shape ends.

### 1.6 Values

- **Number**: finite, absolute value at most 1e7, unless its key gives a narrower range.
  Integer keys (`step`, `track`) MUST hold whole numbers.
- **String**: at most 10,000 characters, unless its key says otherwise.
- **Points**: a flat number array whose length is a multiple of the stride, at most 10,000
  points, relative to the shape's `x`, `y`.
- **Binding**: `{ shape, anchor }`: `shape` a string, `anchor` two numbers in 0..1, and no other
  key.
- **Text** (`text`): a `Y.Text` of at most 10,000 characters (*plain text*), or, in `full` only,
  a `Y.XmlFragment` holding an encre document in encre's collaborative layout (encre FORMAT
  1.6) that passes encre validation (*rich text*).
- **Rich text** (`notes`): a `Y.XmlFragment` as above. Always rich.

### 1.7 Limits

A server checks these on every accepted update:

| Limit | Value |
|---|---|
| Shapes per document | 20,000 |
| Points per stroke | 10,000 |
| Characters per string or plain text | 10,000 |
| Characters per `html` | 100,000 |
| Bytes per update | 1 MiB |
| Absolute value of a number | 1e7 |

Unknown roots, types and keys are refused.

### 1.8 Changing the schema

`meta.schema` names the version. A later version ships a migration function from the one
before, run on load, never a silent reinterpretation. Toile board schema 1 is planche schema 1
with the `board` profile, so moving a Toile board to planche is the identity.

## 2. Text projection

The projection is a plain-text view of a whole document, for the CLI and for agents. It reads
like markdown and round-trips losslessly. It is never the storage.

### 2.1 Shape

```text
---
planche: 1
size: 1920x1080
---

````frame id="f1" x="0" y="0" w="1920" h="1080" z="a0" name="Intro" order="a0"
```notes
Say hello first.
```

```text id="t1" x="120" y="80" w="900" h="200" z="a0" style.size="64"
# Hello **everyone**
```
````

```sticky id="s9" x="2400" y="0" z="a1" text="plain"
buy milk
```
```

- **Front matter** opens the file: a `---` line, `planche: <schema>`, then `size: <w>x<h>` when
  `meta.size` is set, then `---`. Nothing else goes in it.
- **One fence per shape.** The info string is the type, then `id="…"`, then the shape's keys
  as attributes. Blocks are separated by one blank line.
- **Children are nested.** A shape whose `parent` names a frame present in the document is
  written inside that frame's fence, without a `parent` attribute. Any other shape is written at
  the top level, with `parent` when it has one. Only one level nests: a frame inside a frame is
  written at the top level with its `parent`.
- **A frame's notes** are a `notes` fence, first inside the frame fence, holding Facile Markdown.
- **The fence** is one backtick longer than the longest backtick run inside it, and at least
  three, as encre FORMAT 2.1.

### 2.2 Attributes

Attributes are `key="value"`, separated by one space, with `\"` and `\\` as the only escapes, as
encre FORMAT 2.1. The key alone says how the value reads:

| Value | Written as |
|---|---|
| Number | Shortest decimal that reads back to the same 64-bit float, no exponent, `-0` written `0` |
| String | Verbatim, escaped |
| Boolean | `true` or `false` |
| Style | One attribute per field: `style.fill="#fff"`, `style.width="2"` |
| Points | Numbers separated by one space |
| Binding | `<shape id> <anchor x> <anchor y>`, separated by one space |

A writer writes `id` first, then the keys in the order of section 1.4 then section 1.5, then
`style.*` in the order `fill stroke width font size align`, and leaves out `type`, `text`,
`notes` and `html`.

### 2.3 Bodies

| Shape | Body |
|---|---|
| A type with `text`, holding rich text | Facile Markdown, encre's canonical serialization. No `text` attribute |
| A type with `text`, holding plain text | The text verbatim, with `text="plain"` |
| A type with `text`, holding none | Empty, with `text="none"` |
| `frame` | The `notes` fence, then the child shapes |
| `html` | The `html` value verbatim |
| Any other type | Empty |

A reader treats a missing `text` attribute as rich, so an empty body with no `text` attribute is
an empty rich text.

### 2.4 Order

A writer writes frames first, by `order` then `id` (a frame without `order` sorts after those
with one), then the other top-level shapes, by `z` then `id` (a shape without `z` sorts first).
Children inside a frame sort by `z` then `id` the same way.

### 2.5 Reading

The reader is strict, because the projection is written by tools and agents, not typed into a
rich editor. It refuses, with the line number:

- anything outside a fence other than blank lines;
- an unknown type, an unknown or duplicate attribute, or a value its key cannot read;
- a missing or duplicate `id`;
- a `notes` fence outside a frame, or a frame inside a frame fence;
- a document that does not then pass section 1 under the reader's profile.

### 2.6 Round-trip laws

For every valid document `d` and every projection text `p` the reader accepts:

1. `read(write(d)) == d`
2. `write(read(p))` is a fixed point: `write(read(write(read(p)))) == write(read(p))`

Equality compares meta, shape ids, and every key's value; rich text compares as encre
documents without their block ids, which the projection does not carry. A reader gives rich
text fresh block ids.

## 3. Agent operations

Agents read the projection and write through operations. Each operation is validated whole
before anything changes, and, when the document is open, applied as one Yjs transaction in the
room. The host keeps the document version, a number that increases on every accepted change.

| Operation | Arguments | Effect |
|---|---|---|
| `read` | | The projection |
| `read_shapes` | `frame` (optional id) | `[{id, type, parent, projection}]` for top-level shapes, or for the children of `frame`; `projection` is that one shape's block |
| `add` | `projection` (one or more blocks), `parent` (optional frame id) | Adds the shapes. A block without `id` gets a fresh one; an `id` already present is refused. Answers the ids |
| `update` | `id`, `set` (object), `unset` (key list) | Sets and removes keys of one shape. `set` holds values as JSON (numbers, strings, booleans, the style object, the points array, the binding object), and `text` or `notes` as Facile Markdown. `type` and `id` cannot change |
| `delete` | `ids` | Removes those shapes. Removing a frame also removes the shapes whose `parent` is that frame |
| `reorder` | `id`, one of `before`, `after` (a sibling id), `front`, `back` | A frame with `order` moves in slide order; any other shape moves in paint order (`z`) among its siblings |
| `replace_all` | `projection`, `expect_version` | Replaces the whole document only while the version still matches |

### 3.1 Rules

1. A refused operation changes nothing. The result of an accepted one MUST pass section 1 under
   the host's profile; otherwise it is refused.
2. Fresh ids are 16 characters from `A-Z a-z 0-9 - _`, like Toile's client.
3. A new fractional index sorts strictly between its neighbours and is at most 64 characters.
4. Each operation touches only the shapes it names, plus a removed frame's children, so other
   clients' edits to other shapes merge untouched. Only `replace_all` rewrites every shape.
5. Every failure is a typed error whose message says what to change, for example
   `update: shape t1 has no key fit for type text; allowed: …`.

## 4. Compiling to HTML

Reserved. Diapo's scene compiler takes a `full` document as its second input in phase 4 and
keeps the `window.__scene` contract.
