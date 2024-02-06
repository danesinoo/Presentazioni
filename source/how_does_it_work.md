## General operation:

The program takes as input a folder containing Markdown files and transforms 
them into HTML pages. Each Markdown section of the header type (indicated by #, 
##, ####, etc.) becomes a slide in the resulting HTML page. The content under 
each header constitutes the content of the slide.

## Program Startup:
The program starts by executing its executable followed by two flags:
- ``-s path/to/source/file.md`` specifies the input markdown.
- ``-o path/to/output/directory`` specifies the output folder.

---

Just in case: you can run a rust program as this one with:
```bash
cargo run # compiles and run
cargo run -- -s path/to/source_file.md -o output/dir # for this specific program
```

or else

```bash
cargo build # compiles
./target/debug/Presentazioni -s path/to/source_file.md -o output/dir # run
```

or else, if you want to build a more optimized version:

```bash
cargo build --release
./target/release/Presentaizoni -s path/to/source_file.md -o output/dir
```
