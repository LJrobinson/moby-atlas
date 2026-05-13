# Data Model

MOBY Atlas stores each state as a YAML dossier under `data/states/`. The Rust CLI deserializes those files directly, so the docs below describe the current working model rather than a separate aspirational schema.

## Dossier Sections

| Section | Required Shape | Purpose |
|---|---|---|
| `state` | object | State name and two-letter abbreviation. |
| `programs` | object | Medical and adult-use program status plus known start year. |
| `regulatory_bodies` | list | Cannabis regulator names, source websites, types, and notes. |
| `track_and_trace` | object | Seed-to-sale or track-and-trace system information. |
| `license_types` | list | State-specific license labels mapped to canonical license categories. |
| `taxes` | list | State-specific tax labels mapped to canonical tax categories. |
| `active_licenses` | object | Known or pending active license totals, by-type counts, as-of date, source, and confidence. |
| `official_sources` | list | Source receipts used to support the dossier. |
| `notes` | list | Human-readable context, modeling decisions, and unresolved items. |

## Source Receipt Fields

Most source-backed objects include:

| Field | Meaning |
|---|---|
| `source_url` or `url` | Link to the supporting source. |
| `source_quality` | Source type, such as `official_regulator` or `official_tax_agency`. |
| `last_checked` | Date the source was reviewed. |
| `confidence` | Confidence in the captured value. |
| `notes` | Context, caveats, or extraction notes. |

Regulatory bodies currently use `website` instead of a full source receipt block. The `official_sources` section should include the more complete regulator receipt when available.

## Programs

`programs` separates medical and adult-use status.

```yaml
programs:
  medical:
    status: active
    started_year: 2015
  adult_use:
    status: active
    started_year: 2017
```

If a start year is not confidently modeled, use `null` and explain the uncertainty in `notes`.

## License Types

Each license entry preserves the state label in `name` and maps it to a canonical `category`.

```yaml
license_types:
  - name: Retail Dispensary
    category: retail
    source_url: "https://example.gov/license-types"
    source_quality: official_regulator
    last_checked: 2026-05-12
    confidence: high
```

The CLI validates categories against the canonical list in `src/main.rs` and documents them in [taxonomies.md](taxonomies.md).

## Taxes

Each tax entry preserves the state tax label and rate text. Rates are strings because state tax models often include exemptions, variable local rates, dates, or unresolved treatment.

```yaml
taxes:
  - name: Retail Cannabis Excise Tax
    category: retail_excise_tax
    applies_to: "Adult-use retail cannabis sales"
    rate: "10%"
    source_url: "https://example.gov/cannabis-tax"
    source_quality: official_tax_agency
    last_checked: 2026-05-12
    confidence: high
    notes: "Modeled from official tax agency guidance."
```

Use conservative text such as `unknown`, `varies`, `exempt`, or `ended` when that is more accurate than forcing a numeric rate.

## Active Licenses

Active license counts are intentionally cautious.

```yaml
active_licenses:
  total: null
  as_of: "live lookup"
  source_url: "https://example.gov/license-search"
  source_quality: official_regulator
  last_checked: 2026-05-12
  confidence: medium
  by_type: []
```

Use `total: null` and an empty `by_type` list when an official source exists but a count has not been extracted. Do not estimate counts from informal lists.

## Notes

Use `notes` to explain modeling decisions, uncertain years, secondary-source risk, official-source limitations, and follow-up work. Notes are part of the data quality model, not a dumping ground for unsupported facts.
