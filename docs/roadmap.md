# MOBY Atlas Roadmap

MOBY Atlas is a source-cited cannabis state reference project for comparing state cannabis markets, regulators, license types, taxes, track-and-trace systems, official sources, and active license data.

## v0.1.0 - Framework

- Rust CLI initialized
- YAML state dossier files
- Initial state coverage placeholders:
  - PA
  - MA
  - MD
  - MI
  - WV
  - OR
  - NV
  - CA
  - NY
  - CO
- CLI commands:
  - `list`
  - `show <state>`
  - `validate`
  - `compare <state> <state>`

Status: complete

## v0.2.0 - Source Receipts

Add stronger source tracking fields to state dossiers:

- source quality
- last checked date
- confidence level
- official source type
- notes for unresolved/unknown facts

Status: complete

## v0.3.0 - Compare Command

Add state comparison support:

```bash
cargo run -- compare nv ma
```

Initial comparison targets:

- program status
- regulatory bodies
- track-and-trace systems
- license types
- taxes
- active license availability

status: complete

## v0.4.0 - Canonical License Taxonomy

Normalize state-specific license names into canonical categories.

Examples:

| State Label | Canonical Category |
|---|---|
| Retail Dispensary | retail |
| Marijuana Retailer | retail |
| Provisioning Center | retail |
| Processor | manufacturing |
| Testing Laboratory | testing_lab |

Planned CLI support:

```bash
cargo run -- categories
```

status: complete

## v0.5.0 - Tax Model Normalization

Track cannabis-specific taxes by state.

Canonical tax categories include:

- cannabis_excise_tax
- retail_excise_tax
- wholesale_tax
- distributor_tax
- cultivation_tax
- local_option_tax
- medical_cannabis_tax
- adult_use_tax
- state_sales_tax
- gross_receipts_tax

CLI support:

```bash
cargo run -- tax-categories
```

Validation support:

- verifies tax categories use approved canonical values
- warns on unknown or invalid tax category values
- shows state-specific tax names grouped by canonical tax category

status: complete

## v0.6.0 - Active License Counts

Track active license counts by state and license type where official public sources are available.

Fields:

- total active licenses
- active licenses by type
- as-of date
- official source URL
- source confidence

CLI support:

```bash
cargo run -- licenses
```

Validation support:

- warns when active license counts exist without a source URL
- warns when active license counts exist without an as-of date
- warns when active license counts exist without source quality/confidence

status: complete

## v0.7.0 - Export Commands

Add export support:

```bash
cargo run -- export-json
cargo run -- export-csv
```

Export formats:

JSON export preserves full nested state dossiers
CSV export provides one flattened summary row per state

status: complete

## v0.8.0 - Source Completeness Report

Add a command to identify missing source links or incomplete dossiers.

```bash
cargo run -- sources
```

status: complete

## v0.9.0 - Finalize First Atlas

Push to release:

- updated docs/
- updated README.md
- updated github description
- updated github topics

## v1.0.0 - First Complete Atlas

Definition of done:

- All 10 initial states have completed dossiers
- Each state has official regulator source
- Each state has program status
- Each state has track-and-trace system
- Each state has license taxonomy
- Each state has cannabis tax summary
- Each state has active license source if publicly available
- State comparison command works
- Validation command passes
- README explains scope and disclaimer