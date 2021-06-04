# Paper threading

[![crates.io](https://img.shields.io/crates/v/pap.svg)](https://crates.io/crates/pap)

pap is a paper threading tool for the command line. It offers a simple interface for adding metadata and querying reading status. The workflow is opinionated, dividing into four different stages and tailored to the author's use.

<img src="example.png"></img>

## Installation

You can install `pap` with

```
cargo install pap
```

## How-to use

Currently my reading consists of four stages:
 * I  - added but auxiliary sources have to be read first
 * II - read and compressed on a piece of paper or whiteboard
 * III - recalled or explained to somebody else, looked into open-review
 * IV - feed-back from publication venue, textual results or reversed-citation graph

To setup the project create a subfolder with an empty `.pap.toml` file, e.g.
```bash
$ mkdir papers/
$ touch papers/.pap.toml
```
the tool will look through all sub-directories and select the first found workspace.

You can add a new paper with `pap add`. This will open a template for you in vim and let you fill in title, abstract etc. An associated hash is generated from the title and the metadata written to a folder with this hash, e.g. `papers/<hash>/index.toml`. If you want to use a hash, you only have to specify as many symbols as necessary to disambiguate it from other paper's hashes. (similar to git)

You can view your history with `pap view <pattern>`, this prints progress and title with corresponding hash. Only a sequential view is available at the moment, graph views may be added later.

A new stage can be marked with `pap mark <hash fragment> <progress>` with progress one of `(I | II | III | IV)`. After updating the stage the paper should appear at the bottom of the sequential list.

I'm adding the full-text paper to `papers/<hash>/index.pdf`, but no prior path is defined in the tool and you can use arbitrary file names for primary source, artifacts etc.

And of course I encourage you to manage everything with git as this will simplify your working flow and let you share your reading list.

# License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license http://opensource.org/licenses/MIT, at your option. This file may not be copied, modified, or distributed except according to those terms.

