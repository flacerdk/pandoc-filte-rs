# pandoc_filters

A Rust module for writing pandoc filters, in the spirit of the
[Python module](https://github.com/jgm/pandocfilters) and the
[Haskell module](http://hackage.haskell.org/package/pandoc-types-1.16.1.1).

## Usage

A filter is a function `T -> T`, where `T` is one of the types that implement
the `Walkable` trait in `types.rs`. Typically `T` will be either `Block` or
`Inline`.

After writing the function, simply pass it to `filter`, defined in `json.rs`. It
takes a Pandoc AST and returns the transformed AST. `main.rs` shows an example
of this usage with makes every inline string uppercase.

## Why?

Most people would probably do better by just using the Python module, but I
wanted to have an interface to manipulate the Pandoc AST in the context
of a larger program (specifically
[safe_blog](https://github.com/flacerdk/safe_blog)). A type-safe tool for
writing filters provides just that.

## Issues

`types.rs` has a lot of custom serializer code that can probably be
simplified. I wrote this code because Serde's automatically generated serializer
isn't fully compatible with Pandoc: for example, it serializes `X("a","b")` as
`["a", "b"]`, whereas Pandoc wants something of the form `[{"a": []}, {"b":
[]}]`.
