# MOBY Atlas Roadmap

MOBY Atlas is a source-cited cannabis state reference project for comparing regulators, license types, taxes, track-and-trace systems, official sources, active license data, and source completeness across state dossier YAML files.

## Status Summary

| Version | Milestone | Status |
|---|---|---|
| v0.1.0 | Framework | Complete |
| v0.2.0 | Source Receipts | Complete |
| v0.3.0 | Compare Command | Complete |
| v0.4.0 | Canonical License Taxonomy | Complete |
| v0.5.0 | Tax Model Normalization | Complete |
| v0.6.0 | Active License Counts | Complete |
| v0.7.0 | Export Commands | Complete |
| v0.8.0 | Source Completeness Report | Complete |
| v0.9.0 | Finalize First Atlas / GitHub Polish | Complete |
| v1.0.0 | First Complete Atlas | Ready for user review |

## v0.1.0 - Framework

Initial project foundation.

- Rust CLI initialized.
- YAML state dossier files added.
- First-atlas state coverage started for PA, MA, MD, ME, MI, WV, OR, NV, CA, NY, and CO.
- Initial commands added:
  - `list`
  - `show <state>`
  - `validate`
  - `compare <state> <state>`

Status: complete

## v0.2.0 - Source Receipts

Added source tracking fields to state dossiers.

- `source_quality`
- `last_checked`
- `confidence`
- official source type
- notes for unresolved, partial, or unknown facts

Status: complete

## v0.3.0 - Compare Command

Added state comparison support.

```bash
cargo run -- compare nv ma
```

Comparison output covers:

- program status
- regulatory bodies
- track-and-trace systems
- license types
- taxes
- active license availability

Status: complete

## v0.4.0 - Canonical License Taxonomy

Normalized state-specific license names into canonical categories while preserving state labels.

Examples:

| State Label | Canonical Category |
|---|---|
| Retail Dispensary | `retail` |
| Marijuana Retailer | `retail` |
| Provisioning Center | `retail` |
| Processor | `manufacturing` |
| Testing Laboratory | `testing_lab` |

CLI support:

```bash
cargo run -- categories
```

Status: complete

## v0.5.0 - Tax Model Normalization

Tracked cannabis-specific taxes by state and mapped them to canonical tax categories.

Canonical tax categories include:

- `adult_use_tax`
- `cannabis_excise_tax`
- `cultivation_tax`
- `distributor_tax`
- `gross_receipts_tax`
- `local_option_tax`
- `medical_cannabis_tax`
- `retail_excise_tax`
- `sales_tax`
- `state_sales_tax`
- `wholesale_tax`

CLI support:

```bash
cargo run -- tax-categories
```

Validation support:

- verifies tax categories use approved canonical values
- warns on unknown or invalid tax category values
- shows state-specific tax names grouped by canonical tax category

Status: complete

## v0.6.0 - Active License Counts

Tracked active license count fields by state and license type where official public sources are available.

Fields:

- total active licenses, when extracted
- active licenses by type, when extracted
- as-of date or live lookup status
- official source URL
- source quality and confidence

CLI support:

```bash
cargo run -- licenses
```

Validation support:

- warns when active license counts exist without a source URL
- warns when active license counts exist without an as-of date
- warns when active license counts exist without source quality or confidence

Status: complete

## v0.7.0 - Export Commands

Added export support.

```bash
cargo run -- export-json
cargo run -- export-csv
```

Export formats:

- JSON export preserves full nested state dossiers.
- CSV export provides one flattened summary row per state.
- Generated exports are written under `exports/` by default and are ignored by Git.

Status: complete

## v0.8.0 - Source Completeness Report

Added a command to identify missing source links, incomplete source receipt fields, low-confidence values, secondary sources, and active-license extraction gaps.

```bash
cargo run -- sources
```

Status: complete

## v0.9.0 - Finalize First Atlas / GitHub Polish

Prepared the repository as a polished GitHub portfolio project.

- README rewritten to explain the project, problem, usage, examples, philosophy, model, exports, and disclaimer.
- Roadmap updated to reflect completed milestones.
- Focused docs added for usage, data model, source policy, and taxonomies.
- CLI help wording polished for readers landing on the repo cold.
- Obvious metadata cleanup applied without changing the YAML schema.
- GitHub repository description and topics documented for maintainers.

Status: complete

## v1.0.0 - First Complete Atlas

Definition of done:

- All 11 current first-atlas state dossiers remain parseable and validation passes.
- Each state has an official regulator source.
- Each state has medical and adult-use program status with uncertainty preserved where needed.
- Each state has track-and-trace system status or an explicit unknown/uncertain note.
- Each state has license types mapped to canonical categories.
- Each state has cannabis tax entries mapped to canonical tax categories.
- Each state has active license source coverage, or a clear note explaining why official count extraction is still pending.
- Source completeness report has no critical missing receipt fields.
- Weak source warnings are either reduced or intentionally retained with notes that explain the uncertainty.
- Comparison, category, license, source, and export commands work from a clean checkout.
- README and docs describe v1.0.0 as a usable source-cited first atlas, not a complete legal compliance database.

Status: ready for user review before tagging.

## Future Roadmap

- Add more states beyond the current first-atlas coverage.
- Extract official active license counts from public state datasets, dashboards, or license search tools where available.
- Add source freshness checks for stale `last_checked` dates.
- Publish an optional website or static atlas view.
- Add richer exports for category-level, source-level, and comparison-level analysis.
- Automate source ingestion where official public data exists and licensing/format terms allow it.
- Produce API-ready JSON artifacts for downstream tools.
- Generate state comparison reports for selected state pairs or regions.

Future work should keep the same core rule: do not invent regulatory facts. If a value is uncertain, preserve the uncertainty in the dossier.
