# Taxonomies

MOBY Atlas uses canonical categories so state-specific cannabis terminology can be compared without erasing the original state labels.

The canonical lists below match the current CLI validation lists.

## License Categories

| Category | Intended Meaning |
|---|---|
| `consumption` | On-site consumption, hospitality, or lounge license types. |
| `cultivation` | Grower, producer, nursery, cooperative cultivation, or cultivation facility licenses. |
| `delivery` | Cannabis delivery license types. |
| `distribution` | Distributor, transporter, wholesaler, or secure transport license types. |
| `event` | Temporary event or event organizer licenses. |
| `incubator` | Incubator or shared space license types. |
| `manufacturing` | Processor, product manufacturer, production, or extraction license types. |
| `medical_to_adult_use_vertical` | Vertically integrated medical operators authorized for adult-use activity. |
| `medical_vertical` | Vertically integrated medical cannabis organization or registered organization license types. |
| `microbusiness` | Small integrated or limited-scope microbusiness license types. |
| `operator` | Business operator category that does not fit a narrower activity category. |
| `research` | Research, academic, clinical, or R&D license types. |
| `retail` | Retailer, dispensary, store, or provisioning center license types. |
| `sample_collection` | Sample collector license types. |
| `testing_lab` | Laboratory, testing facility, or safety compliance facility license types. |
| `worker_permit` | Worker permits or individual occupational credentials. |

Example:

| State Label | Canonical Category |
|---|---|
| Retail Dispensary | `retail` |
| Marijuana Retailer | `retail` |
| Provisioning Center | `retail` |
| Processor | `manufacturing` |
| Independent Testing Laboratory | `testing_lab` |

Run:

```bash
cargo run -- categories
```

## Tax Categories

| Category | Intended Meaning |
|---|---|
| `adult_use_tax` | Adult-use cannabis tax category when a more specific category is not yet modeled. |
| `cannabis_excise_tax` | General cannabis excise tax. |
| `cultivation_tax` | Cannabis cultivation tax. |
| `distributor_tax` | Distributor-level cannabis tax. |
| `gross_receipts_tax` | Gross receipts tax specific to cannabis activity. |
| `local_option_tax` | Local optional cannabis taxes. |
| `medical_cannabis_tax` | Medical cannabis-specific tax treatment. |
| `retail_excise_tax` | Retail-level cannabis excise tax. |
| `sales_tax` | General sales tax category when state/local specificity is not modeled. |
| `state_sales_tax` | State sales tax treatment. |
| `wholesale_tax` | Wholesale-level cannabis tax. |

Example:

| State Tax Label | Canonical Category |
|---|---|
| State Excise Tax | `retail_excise_tax` |
| Retail Cannabis Excise Tax | `retail_excise_tax` |
| Local Option Tax | `local_option_tax` |
| Medical Marijuana Gross Receipts Tax | `gross_receipts_tax` |

Run:

```bash
cargo run -- tax-categories
```

## Taxonomy Rules

- Preserve the state-specific label in `name`.
- Use the canonical `category` for comparison.
- Keep unclear rates as strings such as `unknown`, `varies`, `exempt`, or `ended`.
- Add notes for unusual mappings, secondary-source support, or unresolved tax treatment.
- Do not add a new canonical category unless the existing categories cannot represent the state label cleanly.
