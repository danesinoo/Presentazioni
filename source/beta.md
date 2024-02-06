# Presentazioni

clicking on "c" make the comment section appear!

---

# Slide navigation:

Navigation can occur via the vertical bars or via Vim motions (h, j, k, l).

---

## How does it work?

- .md into .hmtl
- headers (#, ##, ...) into slides

[[how_does_it_work.md]]

## Slides structure

- title
- content

---

## Slide Structure:

Each slide has a predefined space for the title and one for the content.
The content is resized using CSS scale to fit the dedicated space.
Slides are indicated by thin vertical bars at the edges of the HTML page, which 
appear or disappear depending on the presence of slides in the indicated 
directions.

---

## Comments

---

## Content/Comments Window:

- ``c`` toggles the content/comments window.
- This section can contain detailed information, references, or additional 
explanations.
- Inline comments start with ``---`` and end with ``---``.

---

## Import

- you can even import some file
- I advice to use this to split the presentation and the comments/documentation
- ``[[path/to/import/file.md]]``

[[import.md]]

# About this program

- obsidian slides
- my professors program

[[inspiration.md]]

# Math formula

Are math formula supported this easily?  
- inline $ f: \mathbb{R} \rightarrow \mathbb{R} $
- on a separeta line
$$ f: \mathbb{R} \rightarrow \mathbb{R} $$
