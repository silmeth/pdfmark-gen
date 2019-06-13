# PdfMark-Gen

A simple command-line utility for generating pdfmark file
(like [described here](https://www.physics.drexel.edu/~wking/unfolding-disasters-old/posts/PDF_bookmarks_with_Ghostscript/))
with metadata (title, author, table of contents) for a pdf file.

It takes two parameters:
* a path to a simple json input file specifying `title`, a list of `authors`,
  and `toc` being a list of Table of Contents items, each having its own `title`, `page` number and optionally
  `children` (list of sub-sections of the given ToC item),
* and a path to the output file to which the pdfmark code will be output.

All string (titles, authors) are converted to UTF-16BE with BOM before being written to pdfmark, so it
correctly handles non-ASCII unicode characters.

For example given a json file like this as input:

```json
{
  "title": "A Glorious PDF Example",
  "authors": ["Author no. 1", "Author no. 2", "et al."],
  "toc": [
    { "title": "Réamhrá – Preface", "page": 1 },
    {
      "title": "Chapter 1", "page": 4,
      "children": [
        { "title": "Subsection 1.1", "page": 4 },
        { "title": "Subsection 1.2", "page": 5 }
      ]
    },
    { "title": "Posłowie – Afterword", "page":  10 }
  ]
}
```

it will generate the following pdfmark file:

```
[ /Title <FEFF004100200047006C006F00720069006F0075007300200050004400460020004500780061006D0070006C0065> /Author <FEFF0041007500740068006F00720020006E006F002E00200031002C00200041007500740068006F00720020006E006F002E00200032002C00200065007400200061006C002E> /DOCINFO pdfmark
[ /Title <FEFF005200E90061006D0068007200E10020201300200050007200650066006100630065> /Page 1 /OUT pdfmark
[ /Title <FEFF004300680061007000740065007200200031> /Page 4 /Count 2 /OUT pdfmark
[ /Title <FEFF00530075006200730065006300740069006F006E00200031002E0031> /Page 4 /OUT pdfmark
[ /Title <FEFF00530075006200730065006300740069006F006E00200031002E0032> /Page 5 /OUT pdfmark
[ /Title <FEFF0050006F00730142006F007700690065002020130020004100660074006500720077006F00720064> /Page 10 /OUT pdfmark
```

## What is it useful for?

It’s useful if you want to add an easily-navigable list of content to a pdf file using Ghostscript, especially
if you want to use non-ASCII characters in it.

I’ve written it because I didn’t want to write a pdfmark code by hand, especially since it required the conversion
of all the non-ASCII characters to UTF-16BE BOM hex-encoded strings.

## How to build it?

Like any other simple Rust program. Ensure you have cargo and rustc installed
(see [installing Rust](https://www.rust-lang.org/tools/install)), then:

```
cargo build --release
```

or (to get a debug executable):

```
cargo build
```

## How to use it?

Create a json like the example above, save it to an UTF-8-encoded file, then invoke the program:

```
pdfmark-gen path/to/the/json path/to/output/pdfmark
```

then you can use the generated `pdfmark` with Ghostscript:

```
gs -sOutputFile=path/to/output.pdf -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 \
  -dNOPAUSE -dBATCH input.pdf path/to/pdfmark
```
