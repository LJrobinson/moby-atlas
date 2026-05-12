# MOBY Atlas

**MOBY Atlas** is a source-cited cannabis state reference project for mapping regulators, license types, taxes, track-and-trace systems, official documents, active licenses, and program history across U.S. cannabis markets.

MOBY stands for:

**Marijuana Operator Business Yardstick**

The goal is to create a standardized reference layer for comparing cannabis markets state by state.

## Purpose

The cannabis industry is fragmented. Each state has its own regulators, license structures, taxes, compliance systems, reporting rules, and source-of-truth documents.

MOBY Atlas aims to normalize that chaos into a consistent, source-cited format.

## Initial State Coverage Goals

The first coverage target includes:

- PA - Pennsylvania
- MA - Massachusetts
- MD - Maryland
- ME - Maine
- MI - Michigan
- WV - West Virginia
- OR - Oregon
- NV - Nevada
- CA - California
- NY - New York
- CO - Colorado

Additional states can be added later by creating new state dossier files in `data/states`.

## Current Features

- Rust CLI project
- YAML-based state dossier files
- List available states
- Show a state dossier as JSON
- Validate required initial state coverage
- Compare two state dossiers with `compare <state> <state>`
- Canonical tax category normalization for state-specific cannabis taxes
- Tax category validation during `validate`
- Active license count reporting with source/as-of/confidence fields
- JSON export for full state dossiers
- CSV export for flattened state summary data

## Planned Features

- State comparison command
- Source document validation
- License taxonomy normalization
- Active license count tracking
- Cannabis tax summaries by state
- Track-and-trace system mapping
- JSON export
- CSV export
- Future API/data ingestion from official state sources

## Project Structure

```text
moby-atlas
├── data
│   └── states
├── docs
├── src
│   └── main.rs
├── Cargo.toml
└── README.md
```

## CLI Usage

List available state dossiers:

```bash
cargo run -- list
```

Show a state dossier:

```bash
cargo run -- show nv
```

Validate required state coverage:

```bash
cargo run -- validate
```

Compare two states: `compare <state> <state>`

```bash
cargo run -- compare nv ma
```

Generate a coverage report:

```bash
cargo run -- coverage
```

Show cannabis taxes grouped by canonical tax category:

```bash
cargo run -- tax-categories
```

Show active license count sources and known counts:

```bash
cargo run -- licenses
```

Export all dossiers as JSON:

```bash
cargo run -- export-json
```

Export flattened state summary CSV:

```bash
cargo run -- export-csv
```

Custom output paths:

```bash
cargo run -- export-json --out exports/moby-atlas.json
cargo run -- export-csv --out exports/moby-atlas-states.csv
```

## Data Philosophy

MOBY Atlas prioritizes:

Official sources
Source-cited facts
State-by-state comparison
Unknown values over guessed values
Normalized categories with state-specific labels preserved
Disclaimer

*MOBY Atlas is a reference and research project. It is not legal advice.*