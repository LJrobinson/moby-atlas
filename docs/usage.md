# CLI Usage

MOBY Atlas is a Rust CLI. Commands read YAML dossiers from `data/states/` by default.

## Help

```bash
cargo run -- --help
```

To inspect a subcommand:

```bash
cargo run -- compare --help
```

## List States

```bash
cargo run -- list
```

Prints all available state dossiers by abbreviation and state name.

## Show One Dossier

```bash
cargo run -- show nv
```

Prints the selected YAML dossier as pretty JSON. State abbreviations are case-insensitive because file lookup lowercases the argument.

## Validate

```bash
cargo run -- validate
```

Checks:

- all required first-atlas state files are present
- license categories are canonical
- tax categories are canonical
- active license counts with values have required source fields

## Compare

```bash
cargo run -- compare nv ma
```

Compares:

- program status
- regulators
- track-and-trace systems
- license types and canonical categories
- taxes and rate strings
- active license total and as-of fields

## Coverage

```bash
cargo run -- coverage
```

Scores each dossier as `complete`, `partial`, or `incomplete` based on major modeled fields and source receipt coverage.

## License Categories

```bash
cargo run -- categories
```

Groups state license labels by canonical license category.

## Tax Categories

```bash
cargo run -- tax-categories
```

Groups state tax labels and rates by canonical tax category.

## Active Licenses

```bash
cargo run -- licenses
```

Reports active license count fields, by-type counts, as-of dates, source URLs, source quality, and confidence. Unknown counts are expected when official source extraction is still pending.

## Source Completeness

```bash
cargo run -- sources
```

Reports missing or weak source receipt fields. Warnings are data-quality signals, not legal conclusions.

## Exports

```bash
cargo run -- export-json
cargo run -- export-csv
```

Defaults:

- JSON: `exports/moby-atlas.json`
- CSV: `exports/moby-atlas-states.csv`

Custom paths:

```bash
cargo run -- export-json --out exports/moby-atlas.json
cargo run -- export-csv --out exports/moby-atlas-states.csv
```

The `exports/` directory is ignored by Git. Treat generated files as local artifacts unless a future release intentionally tracks samples.

## Alternate Data Directory

Most commands accept `--data-dir`:

```bash
cargo run -- list --data-dir data/states
```

This is useful for smoke-testing a copied or experimental dossier directory without changing the default dataset.
