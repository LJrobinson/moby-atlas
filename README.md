# MOBY Atlas

**MOBY Atlas** is a Rust CLI and source-cited cannabis state reference project for comparing state regulators, license types, tax models, track-and-trace systems, active license sources, and official documents.

MOBY stands for **Marijuana Operator Business Yardstick**.

Cannabis regulation in the United States is fragmented by design. Every state has its own regulator, license vocabulary, tax model, track-and-trace system, public records, and source documents. That makes it hard for operators, researchers, builders, analysts, and compliance teams to answer a basic question: "How does this state compare to that state?"

MOBY Atlas turns that state-by-state sprawl into a structured, source-cited comparison layer.

## What It Does

- Stores each state as a YAML dossier in `data/states/`.
- Preserves state-specific license and tax labels while making them easier to compare.
- Maps those labels into canonical categories so states can be compared.
- Tracks official source URLs, source quality, last-checked dates, confidence, and notes.
- Reports active license count sources, even when counts still need extraction.
- Exports the atlas as full JSON or flattened CSV for downstream work.
- Surfaces source gaps and weak receipts instead of hiding uncertainty.

MOBY Atlas is not trying to be a final legal database. It is a careful first atlas: transparent about what is known, what is sourced, and what still needs review.

## Current Coverage

The v1.0.0 first-atlas scope covers 11 state dossiers:

| State | Code |
|---|---|
| California | CA |
| Colorado | CO |
| Massachusetts | MA |
| Maryland | MD |
| Maine | ME |
| Michigan | MI |
| Nevada | NV |
| New York | NY |
| Oregon | OR |
| Pennsylvania | PA |
| West Virginia | WV |

Additional states can be added by creating another dossier YAML file in `data/states/` that follows the same model.

## CLI Features

| Command | Purpose |
|---|---|
| `list` | List available state dossiers. |
| `show <state>` | Print one state dossier as JSON. |
| `validate` | Check required first-atlas state coverage and canonical categories. |
| `compare <state> <state>` | Compare two states across programs, regulators, tracking, licenses, taxes, and active license availability. |
| `coverage` | Score dossier completeness across major modeled fields and source receipts. |
| `categories` | Group state license labels by canonical license category. |
| `tax-categories` | Group cannabis taxes by canonical tax category. |
| `licenses` | Report active license count sources, as-of dates, confidence, and known counts. |
| `sources` | Report missing or weak source receipt metadata. |
| `export-json` | Export all full dossiers as JSON. |
| `export-csv` | Export one flattened summary row per state. |

## Quick Start

```bash
cargo run -- list
cargo run -- validate
cargo run -- compare nv ma
```

Show a dossier:

```bash
cargo run -- show nv
```

Export generated artifacts:

```bash
cargo run -- export-json
cargo run -- export-csv
```

Custom export paths:

```bash
cargo run -- export-json --out exports/moby-atlas.json
cargo run -- export-csv --out exports/moby-atlas-states.csv
```

The `exports/` directory is ignored so generated data does not get accidentally committed.

## Example Output

Compare Nevada and Massachusetts:

```bash
cargo run -- compare nv ma
```

```text
MOBY Atlas Compare: NV vs MA

Program Status:
  Medical:
    NV: active, started 2015
    MA: active, started unknown

Track and Trace:
  NV: Metrc (active)
  MA: Metrc (active)

License Types:
  NV:
    - Cultivation [cultivation]
    - Retail Dispensary [retail]

  MA:
    - Marijuana Cultivator [cultivation]
    - Marijuana Retailer [retail]

Taxes:
  NV:
    - Retail Cannabis Excise Tax: 10%

  MA:
    - State Sales Tax: 6.25%
    - State Excise Tax: 10.75%
```

Group state license labels by canonical category:

```bash
cargo run -- categories
```

```text
MOBY Atlas License Categories

cultivation:
  CA: Cultivation
  MA: Craft Marijuana Cooperative, Marijuana Cultivator
  NV: Cultivation

retail:
  CA: Retail
  MA: Marijuana Retailer
  NV: Retail Dispensary
```

Group cannabis taxes by canonical category:

```bash
cargo run -- tax-categories
```

```text
MOBY Atlas Tax Categories

retail_excise_tax:
  MA: State Excise Tax - 10.75%
  NV: Retail Cannabis Excise Tax - 10%
  OR: State Recreational Marijuana Tax - 17%

state_sales_tax:
  CA: State Sales and Use Tax - varies
  MA: State Sales Tax - 6.25%
```

Inspect active license count sources:

```bash
cargo run -- licenses
```

```text
MOBY Atlas Active License Counts

NV:
  Total: unknown
  As of: 2026-04-01
  Source: https://ccb.nv.gov/list-of-licensees/
  Source Quality: official_regulator
  Confidence: high
  By Type: unknown
```

Audit source completeness:

```bash
cargo run -- sources
```

```text
MOBY Atlas Source Completeness Report

NV:
  WARN: tax 'Wholesale Cannabis Excise Tax' missing/unknown rate
  WARN: active_licenses total unknown
  WARN: active_licenses by_type empty
```

Export data:

```bash
cargo run -- export-json --out exports/moby-atlas.json
cargo run -- export-csv --out exports/moby-atlas-states.csv
```

```text
Exported 11 state dossier(s) to exports/moby-atlas.json
Exported 11 state summary row(s) to exports/moby-atlas-states.csv
```

## Data Philosophy

MOBY Atlas is built around a few deliberate rules:

- Source-cited facts beat broad summaries.
- Unknown is better than fake certainty.
- Official regulator, tax agency, statute, rule, and open-data sources are preferred.
- State-specific labels are preserved because local language matters.
- Canonical categories normalize terminology for comparison.
- Medium or low confidence is acceptable when the note explains the uncertainty.
- Secondary sources are marked as secondary and should be replaced with official sources when available.
- This project is a reference and research tool, not legal advice.

## Data Model

Each YAML dossier includes:

| Section | Description |
|---|---|
| `state` | State name and abbreviation. |
| `programs` | Medical and adult-use status plus known start year. |
| `regulatory_bodies` | Regulator names, types, websites, and notes. |
| `track_and_trace` | System name, status, source receipt, and notes. |
| `license_types` | State license labels mapped to canonical license categories. |
| `taxes` | Cannabis tax labels mapped to canonical tax categories. |
| `active_licenses` | Known or pending totals, by-type counts, as-of date, source, and confidence. |
| `official_sources` | Regulator pages, tax references, statute/rule links, open-data pages, PDFs, and other source receipts. |
| `notes` | Human-readable context and unresolved items. |

Source-backed records generally carry:

- `source_url` or `url`
- `source_quality`
- `last_checked`
- `confidence`
- `notes` where the fact needs context

More detail lives in [docs/data-model.md](docs/data-model.md) and [docs/source-policy.md](docs/source-policy.md).

## Canonical License Taxonomy

Cannabis states often use different labels for similar commercial functions. MOBY Atlas keeps the state label while adding a normalized category.

Examples:

| State Label | Canonical Category |
|---|---|
| Retail Dispensary | `retail` |
| Marijuana Retailer | `retail` |
| Provisioning Center | `retail` |
| Processor | `manufacturing` |
| Independent Testing Laboratory | `testing_lab` |
| Consumption Lounge | `consumption` |

The current canonical license categories are documented in [docs/taxonomies.md](docs/taxonomies.md).

## Canonical Tax Taxonomy

Cannabis taxes vary by state: retail excise, wholesale, distributor, cultivation, medical-only, local option, sales tax treatment, and other models. MOBY Atlas stores the state-specific tax name and rate string, then maps it to a canonical category such as:

- `retail_excise_tax`
- `cannabis_excise_tax`
- `wholesale_tax`
- `distributor_tax`
- `cultivation_tax`
- `local_option_tax`
- `medical_cannabis_tax`
- `state_sales_tax`

The tax model is intentionally conservative. If a rate is unclear or state treatment needs review, the dossier keeps that uncertainty in `rate`, `confidence`, and `notes`.

## Active License Counts

Active license data is treated differently from static regulatory facts. Many states publish searchable license portals or dashboards, but not all of them expose a simple official count by license type.

MOBY Atlas tracks:

- total active license count, when extracted
- by-type counts, when extracted
- `as_of` date or live lookup status
- official source URL
- source quality and confidence

When an official source exists but counts have not been extracted, the dossier keeps `total: null` and an explanatory confidence level rather than inventing a number.

## Source Confidence Model

The source model is simple on purpose:

| Field | Meaning |
|---|---|
| `source_quality` | Kind of source, such as `official_regulator`, `official_tax_agency`, `official_statute_or_regulation`, `official_open_data`, `official_vendor`, or `secondary_source`. |
| `last_checked` | Date the source was reviewed for the dossier. |
| `confidence` | Current confidence in the captured value: usually `high`, `medium`, or `low`. |
| `notes` | Why the value is modeled this way, especially if incomplete or uncertain. |

Run `cargo run -- sources` to see missing fields, low-confidence areas, secondary sources, and active-license extraction gaps.

## Project Structure

```text
moby-atlas/
|-- data/
|   `-- states/
|       |-- ca.yaml
|       |-- co.yaml
|       `-- ...
|-- docs/
|   |-- data-model.md
|   |-- roadmap.md
|   |-- source-policy.md
|   |-- taxonomies.md
|   `-- usage.md
|-- src/
|   `-- main.rs
|-- Cargo.toml
|-- Cargo.lock
`-- README.md
```

## GitHub Metadata Suggestions

Suggested repository description:

> Rust CLI and source-cited cannabis state atlas for comparing regulators, license types, taxes, track-and-trace systems, and active license sources.

Suggested topics:

`rust`, `cli`, `cannabis`, `cannabis-data`, `regulatory-data`, `compliance`, `open-data`, `yaml`, `serde`, `data-normalization`, `state-by-state`, `taxonomies`

## Roadmap

Milestone status:

- v0.1.0 Framework
- v0.2.0 Source receipts
- v0.3.0 Compare command
- v0.4.0 Canonical license taxonomy
- v0.5.0 Tax model normalization
- v0.6.0 Active license counts
- v0.7.0 Export commands
- v0.8.0 Source completeness report
- v0.9.0 GitHub polish and first-atlas documentation pass
- v1.0.0 Data completeness and source-quality audit for the 11-state first atlas: ready for user review

The v1.0.0 target is a usable source-cited first atlas, not a complete legal compliance database. It keeps uncertainty visible, validates the current 11-state scope, and reports no missing source receipt fields before tagging. See [docs/roadmap.md](docs/roadmap.md).

## Why This Matters

Cannabis market data is often either too legalistic for builders, too informal for compliance work, or too state-specific for comparison. MOBY Atlas is a portfolio-grade attempt to model the messy middle: a practical CLI, transparent source receipts, normalized categories, and enough humility to mark uncertainty instead of smoothing it away.

That combination matters for regulated-market research, internal tooling, open-data workflows, compliance prototypes, and any product that needs to compare state cannabis markets without pretending the states all speak the same language.

## Disclaimer

MOBY Atlas is a reference and research project. It is not legal, tax, compliance, financial, or business advice. Cannabis laws, rules, tax rates, licensing systems, and agency guidance change often. Always verify against current official sources before relying on any value.
