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

## Data Philosophy

MOBY Atlas prioritizes:

Official sources
Source-cited facts
State-by-state comparison
Unknown values over guessed values
Normalized categories with state-specific labels preserved
Disclaimer

*MOBY Atlas is a reference and research project. It is not legal advice.*