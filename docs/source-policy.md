# Source Policy

MOBY Atlas prefers transparent uncertainty over polished but unsupported claims. Every dossier should make it clear where a fact came from, how recently it was checked, and how much confidence the project has in the modeled value.

## Preferred Sources

Use official public sources whenever possible:

- cannabis regulator websites
- tax agency guidance
- state statutes
- administrative rules
- official open-data portals
- official license search tools
- official annual reports, bulletins, PDFs, and dashboards

Vendor pages may be useful for track-and-trace confirmation, but the source report flags `official_vendor` because regulator, statute, rule, or tax agency sources are stronger when available.

Secondary sources should be rare. When used, mark `source_quality: secondary_source`, keep confidence conservative, and add a note explaining what official source should replace it.

## Source Quality Labels

Current dossiers use labels such as:

| Label | Meaning |
|---|---|
| `official_regulator` | State cannabis regulator or equivalent agency. |
| `official_tax_agency` | State tax/revenue agency source. |
| `official_statute_or_regulation` | Statute, rule, code, or official regulation source. |
| `official_open_data` | Official public data portal or downloadable dataset. |
| `official_vendor` | Official vendor page, usually for track-and-trace context. |
| `secondary_source` | Non-official source used as a temporary reference. |

## Confidence Labels

Use confidence to communicate data quality, not optimism.

| Label | Use When |
|---|---|
| `high` | The value is directly supported by a strong current source. |
| `medium` | The value is sourced but needs better extraction, a stronger source, or extra review. |
| `low` | The source exists but the modeled value is incomplete, unresolved, or needs verification. |

Avoid using `unknown` as a final confidence value. If the value itself is unknown, keep the value unknown and set confidence conservatively with a note.

## Last Checked

`last_checked` records when the source was reviewed for the dossier. It is not a guarantee that the linked page has remained unchanged.

Use ISO-style dates:

```yaml
last_checked: 2026-05-12
```

Future freshness checks should be able to flag stale dates without changing the underlying schema.

## Unknowns And Notes

Unknowns are acceptable. They should be obvious.

Good patterns:

- `total: null` for unextracted active license totals
- `rate: "unknown"` when a tax exists but the current rate needs official verification
- `started_year: null` when the start year is not confidently modeled
- notes that name what needs review instead of implying certainty

Avoid patterns:

- guessing counts from a search result page
- copying a tax rate from a secondary source without marking it
- replacing a blank with a confident value because it looks cleaner

## Source Completeness Command

Run:

```bash
cargo run -- sources
```

The report flags missing receipt fields, medium/low confidence, secondary sources, vendor-only source support, unknown active license totals, and empty by-type active license counts. Warnings are not automatically errors; they are a public to-do list for data quality.
