# Tool for generating the Fenl documentation

Currently, this only generates the catalog.

The catalog is maintained in `sparrow-catalog/catalog/*.toml`.
Each `toml` file defines the documentation for a Fenl function.

## Generating Documentation

Documentation may be generated using (from `sparrow-rs` directory) the following command.
This uses the `catalog.md` template from the template-dir to render the catalog.

```sh
cargo run -p sparrow-catalog -- --input-dir sparrow-catalog/catalog generate --template-dir sparrow-catalog/templates
```

## Updating Examples / Signatures

The following updates the signature and example output in the `toml` files.

```sh
cargo run -p sparrow-catalog -- --input-dir sparrow-catalog/catalog update
```

## Checking Examlpes / Signatures

The following ensures that the signature and example output in the `toml` files are up to date.

```sh
cargo run -p sparrow-catalog -- --input-dir sparrow-catalog/catalog check
```

## Function Documentation Style Guide

1. Each function should have a `short_doc` which consists of a single sentence
   and ends with a period. This is used in the tables listing all functions, so
   it should be short and concise.
2. Each function should have a `long_doc` describing how it behaves and how each
   argument is used.
3. Each function should have one or more examples. Each example should have a `name`
   and `description`. The `input_csv` is required, but the `output_csv` may be
   updated automatically.

Currently, all of the examples use a single `input_csv` that gets registered as a
table with the following properties:

- The name of the table is `Input`.
- The time column is `time`.
- The key column is `key`.
- The grouping is `grouping`.

Note that no `subsort` column is used. Examples should not contain one.