use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

#[derive(Parser, Debug)]
#[command(name = "moby-atlas")]
#[command(about = "MOBY Atlas: source-cited cannabis state reference CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all available state dossiers
    List {
        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Show one state dossier as JSON
    Show {
        /// State abbreviation, such as NV, MA, CA
        state: String,

        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Validate all state dossier files
    Validate {
        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Compare two state dossiers
    Compare {
        /// First state abbreviation, such as NV
        left: String,

        /// Second state abbreviation, such as MA
        right: String,

        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Report dossier coverage status for all states
    Coverage {
        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Show license labels grouped by canonical category
    Categories {
        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Show cannabis taxes grouped by canonical tax category
    TaxCategories {
        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Show active license count sources and known counts
    Licenses {
        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Export all state dossiers as JSON
    ExportJson {
        /// Output JSON file path
        #[arg(short, long, default_value = "exports/moby-atlas.json")]
        out: PathBuf,

        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },

    /// Export flattened state summary data as CSV
    ExportCsv {
        /// Output CSV file path
        #[arg(short, long, default_value = "exports/moby-atlas-states.csv")]
        out: PathBuf,

        /// Directory containing state YAML files
        #[arg(short, long, default_value = "data/states")]
        data_dir: PathBuf,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct StateDossier {
    state: StateIdentity,
    programs: Programs,
    regulatory_bodies: Vec<RegulatoryBody>,
    track_and_trace: TrackAndTrace,
    license_types: Vec<LicenseType>,
    taxes: Vec<TaxRule>,
    active_licenses: ActiveLicenses,
    official_sources: Vec<OfficialSource>,
    notes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StateIdentity {
    name: String,
    abbreviation: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Programs {
    medical: ProgramStatus,
    adult_use: ProgramStatus,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProgramStatus {
    status: String,
    started_year: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegulatoryBody {
    name: String,
    #[serde(rename = "type")]
    body_type: String,
    website: String,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrackAndTrace {
    system: String,
    status: String,
    source_url: String,
    source_quality: String,
    last_checked: String,
    confidence: String,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LicenseType {
    name: String,
    category: String,
    source_url: String,
    source_quality: String,
    last_checked: String,
    confidence: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaxRule {
    name: String,
    category: String,
    applies_to: String,
    rate: String,
    source_url: String,
    source_quality: String,
    last_checked: String,
    confidence: String,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ActiveLicenses {
    total: Option<u32>,
    as_of: String,
    source_url: String,
    source_quality: String,
    last_checked: String,
    confidence: String,
    by_type: Vec<ActiveLicenseCount>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ActiveLicenseCount {
    license_type: String,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OfficialSource {
    title: String,
    #[serde(rename = "type")]
    source_type: String,
    url: String,
    source_quality: String,
    last_checked: String,
    confidence: String,
    notes: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoverageStatus {
    Complete,
    Partial,
    Incomplete,
}

#[derive(Debug)]
struct InvalidTaxCategory {
    state: String,
    tax_name: String,
    category: String,
}

#[derive(Debug)]
struct ActiveLicenseIssue {
    state: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct StateSummaryRow {
    state: String,
    abbreviation: String,
    medical_status: String,
    medical_started_year: String,
    adult_use_status: String,
    adult_use_started_year: String,
    regulators: String,
    track_and_trace_system: String,
    track_and_trace_status: String,
    license_type_count: usize,
    tax_count: usize,
    active_license_total: String,
    active_license_as_of: String,
    active_license_source: String,
    active_license_confidence: String,
}

fn find_active_license_issues(dossiers: &[StateDossier]) -> Vec<ActiveLicenseIssue> {
    let mut issues = Vec::new();

    for dossier in dossiers {
        let state = dossier.state.abbreviation.to_uppercase();
        let active = &dossier.active_licenses;

        let has_total = active.total.is_some();
        let has_by_type = !active.by_type.is_empty();
        let has_any_count = has_total || has_by_type;

        if has_any_count && is_unknown_or_empty(&active.source_url) {
            issues.push(ActiveLicenseIssue {
                state: state.clone(),
                message: "has active license counts but no source_url".to_string(),
            });
        }

        if has_any_count && is_unknown_or_empty(&active.as_of) {
            issues.push(ActiveLicenseIssue {
                state: state.clone(),
                message: "has active license counts but no as_of date".to_string(),
            });
        }

        if has_any_count && is_unknown_or_empty(&active.confidence) {
            issues.push(ActiveLicenseIssue {
                state: state.clone(),
                message: "has active license counts but no confidence".to_string(),
            });
        }

        if has_any_count && is_unknown_or_empty(&active.source_quality) {
            issues.push(ActiveLicenseIssue {
                state: state.clone(),
                message: "has active license counts but no source_quality".to_string(),
            });
        }
    }

    issues
}

fn find_invalid_tax_categories(dossiers: &[StateDossier]) -> Vec<InvalidTaxCategory> {
    let valid_categories = canonical_tax_categories();
    let mut invalid = Vec::new();

    for dossier in dossiers {
        let state = dossier.state.abbreviation.to_uppercase();

        for tax in &dossier.taxes {
            if is_unknown_or_empty(&tax.name) && is_unknown_or_empty(&tax.category) {
                continue;
            }

            if is_unknown_or_empty(&tax.category) {
                invalid.push(InvalidTaxCategory {
                    state: state.clone(),
                    tax_name: format_empty_as_unknown(&tax.name),
                    category: "unknown".to_string(),
                });

                continue;
            }

            if !valid_categories.contains(&tax.category.as_str()) {
                invalid.push(InvalidTaxCategory {
                    state: state.clone(),
                    tax_name: format_empty_as_unknown(&tax.name),
                    category: tax.category.clone(),
                });
            }
        }
    }

    invalid
}

impl CoverageStatus {
    fn as_str(self) -> &'static str {
        match self {
            CoverageStatus::Complete => "complete",
            CoverageStatus::Partial => "partial",
            CoverageStatus::Incomplete => "incomplete",
        }
    }
}

fn report_coverage(data_dir: &Path) -> Result<()> {
    let dossiers = load_all_dossiers(data_dir)?;

    let mut complete_count = 0;
    let mut partial_count = 0;
    let mut incomplete_count = 0;

    println!("MOBY Atlas Coverage Report");
    println!();

    for dossier in &dossiers {
        let status = coverage_status(dossier);

        match status {
            CoverageStatus::Complete => complete_count += 1,
            CoverageStatus::Partial => partial_count += 1,
            CoverageStatus::Incomplete => incomplete_count += 1,
        }

        println!(
            "{}: {}",
            dossier.state.abbreviation.to_uppercase(),
            status.as_str()
        );
    }

    println!();
    println!("Summary:");
    println!("Complete: {complete_count}");
    println!("Partial: {partial_count}");
    println!("Incomplete: {incomplete_count}");

    Ok(())
}

fn coverage_status(dossier: &StateDossier) -> CoverageStatus {
    let score = coverage_score(dossier);

    if score >= 8 {
        CoverageStatus::Complete
    } else if score >= 4 {
        CoverageStatus::Partial
    } else {
        CoverageStatus::Incomplete
    }
}

fn coverage_score(dossier: &StateDossier) -> u8 {
    let mut score = 0;

    if has_program_status(dossier) {
        score += 1;
    }

    if has_regulatory_body(dossier) {
        score += 1;
    }

    if has_track_and_trace(dossier) {
        score += 1;
    }

    if has_license_types(dossier) {
        score += 1;
    }

    if has_taxes(dossier) {
        score += 1;
    }

    if has_active_license_source(dossier) {
        score += 1;
    }

    if has_official_sources(dossier) {
        score += 1;
    }

    if has_source_receipts(dossier) {
        score += 1;
    }

    score
}

fn has_program_status(dossier: &StateDossier) -> bool {
    !is_unknown_or_empty(&dossier.programs.medical.status)
        || !is_unknown_or_empty(&dossier.programs.adult_use.status)
}

fn has_regulatory_body(dossier: &StateDossier) -> bool {
    dossier
        .regulatory_bodies
        .iter()
        .any(|body| !is_unknown_or_empty(&body.name) && !is_unknown_or_empty(&body.website))
}

fn has_track_and_trace(dossier: &StateDossier) -> bool {
    !is_unknown_or_empty(&dossier.track_and_trace.system)
        && !is_unknown_or_empty(&dossier.track_and_trace.source_url)
}

fn has_license_types(dossier: &StateDossier) -> bool {
    dossier.license_types.iter().any(|license| {
        !is_unknown_or_empty(&license.name)
            && !is_unknown_or_empty(&license.category)
            && !is_unknown_or_empty(&license.source_url)
    })
}

fn has_taxes(dossier: &StateDossier) -> bool {
    dossier.taxes.iter().any(|tax| {
        !is_unknown_or_empty(&tax.name)
            && !is_unknown_or_empty(&tax.applies_to)
            && !is_unknown_or_empty(&tax.source_url)
    })
}

fn has_active_license_source(dossier: &StateDossier) -> bool {
    !is_unknown_or_empty(&dossier.active_licenses.source_url)
}

fn has_official_sources(dossier: &StateDossier) -> bool {
    dossier.official_sources.iter().any(|source| {
        !is_unknown_or_empty(&source.title)
            && !is_unknown_or_empty(&source.source_type)
            && !is_unknown_or_empty(&source.url)
    })
}

fn has_source_receipts(dossier: &StateDossier) -> bool {
    let track_has_receipt = has_receipt_fields(
        &dossier.track_and_trace.source_quality,
        &dossier.track_and_trace.last_checked,
        &dossier.track_and_trace.confidence,
    );

    let license_has_receipt = dossier.license_types.iter().any(|license| {
        has_receipt_fields(
            &license.source_quality,
            &license.last_checked,
            &license.confidence,
        )
    });

    let tax_has_receipt = dossier.taxes.iter().any(|tax| {
        has_receipt_fields(&tax.source_quality, &tax.last_checked, &tax.confidence)
    });

    let official_source_has_receipt = dossier.official_sources.iter().any(|source| {
        has_receipt_fields(
            &source.source_quality,
            &source.last_checked,
            &source.confidence,
        )
    });

    track_has_receipt || license_has_receipt || tax_has_receipt || official_source_has_receipt
}

fn has_receipt_fields(source_quality: &str, last_checked: &str, confidence: &str) -> bool {
    !is_unknown_or_empty(source_quality)
        && !is_unknown_or_empty(last_checked)
        && !is_unknown_or_empty(confidence)
}

fn is_unknown_or_empty(value: &str) -> bool {
    let normalized = value.trim().to_lowercase();

    normalized.is_empty() || normalized == "unknown"
}

fn main() -> Result<()> {
    let cli = Cli::parse();

        match cli.command {
            Commands::List { data_dir } => list_states(&data_dir),
            Commands::Show { state, data_dir } => show_state(&state, &data_dir),
            Commands::Validate { data_dir } => validate_states(&data_dir),
            Commands::Compare {
                left,
                right,
                data_dir,
            } => compare_states(&left, &right, &data_dir),
            Commands::Coverage { data_dir } => report_coverage(&data_dir),
            Commands::Categories { data_dir } => show_license_categories(&data_dir),
            Commands::TaxCategories { data_dir } => show_tax_categories(&data_dir),
            Commands::Licenses { data_dir } => show_active_licenses(&data_dir),
            Commands::ExportJson { out, data_dir } => export_json(&data_dir, &out),
            Commands::ExportCsv { out, data_dir } => export_csv(&data_dir, &out),
        }
}

fn list_states(data_dir: &Path) -> Result<()> {
    let dossiers = load_all_dossiers(data_dir)?;

    for dossier in dossiers {
        println!(
            "{} - {}",
            dossier.state.abbreviation.to_uppercase(),
            dossier.state.name
        );
    }

    Ok(())
}

fn show_state(state: &str, data_dir: &Path) -> Result<()> {
    let path = data_dir.join(format!("{}.yaml", state.to_lowercase()));
    let dossier = load_dossier(&path)?;

    let json = serde_json::to_string_pretty(&dossier)?;
    println!("{json}");

    Ok(())
}

fn validate_states(data_dir: &Path) -> Result<()> {
    let dossiers = load_all_dossiers(data_dir)?;

    let expected_states = ["PA", "MA", "MD", "MI", "WV", "OR", "NV", "CA", "NY", "CO", "ME"];

    let found: Vec<String> = dossiers
        .iter()
        .map(|dossier| dossier.state.abbreviation.to_uppercase())
        .collect();

    let mut missing = Vec::new();

    for expected in expected_states {
        if !found.iter().any(|actual| actual == expected) {
            missing.push(expected);
        }
    }

    if missing.is_empty() {
        println!("OK: all required initial MOBY Atlas states are present.");
    } else {
        println!("Missing required states:");
        for state in &missing {
            println!("- {state}");
        }
    }

    let invalid_categories = find_invalid_license_categories(&dossiers);

    if invalid_categories.is_empty() {
        println!("OK: all license categories are canonical.");
    } else {
        println!("Invalid license categories:");

        for issue in invalid_categories {
            println!(
                "- {}: '{}' uses invalid category '{}'",
                issue.state, issue.license_name, issue.category
            );
        }
    }

    let invalid_tax_categories = find_invalid_tax_categories(&dossiers);

    if invalid_tax_categories.is_empty() {
        println!("OK: all tax categories are canonical.");
    } else {
        println!("Invalid tax categories:");

        for issue in invalid_tax_categories {
            println!(
                "- {}: '{}' uses invalid category '{}'",
                issue.state, issue.tax_name, issue.category
            );
        }
    }

    let active_license_issues = find_active_license_issues(&dossiers);

    if active_license_issues.is_empty() {
        println!("OK: active license counts have required source fields.");
    } else {
        println!("Active license count issues:");

        for issue in active_license_issues {
            println!("- {}: {}", issue.state, issue.message);
        }
    }

    println!("Validated {} state dossier file(s).", dossiers.len());

    Ok(())
}

fn compare_states(left: &str, right: &str, data_dir: &Path) -> Result<()> {
    let left_path = data_dir.join(format!("{}.yaml", left.to_lowercase()));
    let right_path = data_dir.join(format!("{}.yaml", right.to_lowercase()));

    let left_dossier = load_dossier(&left_path)?;
    let right_dossier = load_dossier(&right_path)?;

    let left_abbr = left_dossier.state.abbreviation.to_uppercase();
    let right_abbr = right_dossier.state.abbreviation.to_uppercase();

    println!("MOBY Atlas Compare: {left_abbr} vs {right_abbr}");
    println!();

    print_program_comparison(&left_dossier, &right_dossier);
    print_regulator_comparison(&left_dossier, &right_dossier);
    print_track_and_trace_comparison(&left_dossier, &right_dossier);
    print_license_type_comparison(&left_dossier, &right_dossier);
    print_tax_comparison(&left_dossier, &right_dossier);
    print_active_license_comparison(&left_dossier, &right_dossier);

    Ok(())
}

fn print_program_comparison(left: &StateDossier, right: &StateDossier) {
    println!("Program Status:");

    println!("  Medical:");
    println!(
        "    {}: {}, started {}",
        left.state.abbreviation,
        left.programs.medical.status,
        format_optional_year(left.programs.medical.started_year)
    );
    println!(
        "    {}: {}, started {}",
        right.state.abbreviation,
        right.programs.medical.status,
        format_optional_year(right.programs.medical.started_year)
    );

    println!();

    println!("  Adult Use:");
    println!(
        "    {}: {}, started {}",
        left.state.abbreviation,
        left.programs.adult_use.status,
        format_optional_year(left.programs.adult_use.started_year)
    );
    println!(
        "    {}: {}, started {}",
        right.state.abbreviation,
        right.programs.adult_use.status,
        format_optional_year(right.programs.adult_use.started_year)
    );

    println!();
}

fn print_regulator_comparison(left: &StateDossier, right: &StateDossier) {
    println!("Regulators:");

    println!("  {}:", left.state.abbreviation);
    for body in &left.regulatory_bodies {
        println!("    - {}", format_empty_as_unknown(&body.name));
    }

    println!("  {}:", right.state.abbreviation);
    for body in &right.regulatory_bodies {
        println!("    - {}", format_empty_as_unknown(&body.name));
    }

    println!();
}

fn print_track_and_trace_comparison(left: &StateDossier, right: &StateDossier) {
    println!("Track and Trace:");

    println!(
        "  {}: {} ({})",
        left.state.abbreviation,
        format_empty_as_unknown(&left.track_and_trace.system),
        left.track_and_trace.status
    );

    println!(
        "  {}: {} ({})",
        right.state.abbreviation,
        format_empty_as_unknown(&right.track_and_trace.system),
        right.track_and_trace.status
    );

    println!();
}

fn print_license_type_comparison(left: &StateDossier, right: &StateDossier) {
    println!("License Types:");

    println!("  {}:", left.state.abbreviation);
    for license in &left.license_types {
        println!(
            "    - {} [{}]",
            format_empty_as_unknown(&license.name),
            format_empty_as_unknown(&license.category)
        );
    }

    println!();

    println!("  {}:", right.state.abbreviation);
    for license in &right.license_types {
        println!(
            "    - {} [{}]",
            format_empty_as_unknown(&license.name),
            format_empty_as_unknown(&license.category)
        );
    }

    println!();
}

fn print_tax_comparison(left: &StateDossier, right: &StateDossier) {
    println!("Taxes:");

    println!("  {}:", left.state.abbreviation);
    for tax in &left.taxes {
        println!(
            "    - {}: {}",
            format_empty_as_unknown(&tax.name),
            format_empty_as_unknown(&tax.rate)
        );
    }

    println!();

    println!("  {}:", right.state.abbreviation);
    for tax in &right.taxes {
        println!(
            "    - {}: {}",
            format_empty_as_unknown(&tax.name),
            format_empty_as_unknown(&tax.rate)
        );
    }

    println!();
}

fn print_active_license_comparison(left: &StateDossier, right: &StateDossier) {
    println!("Active Licenses:");

    println!(
        "  {}: total {}, as of {}",
        left.state.abbreviation,
        format_optional_count(left.active_licenses.total),
        format_empty_as_unknown(&left.active_licenses.as_of)
    );

    println!(
        "  {}: total {}, as of {}",
        right.state.abbreviation,
        format_optional_count(right.active_licenses.total),
        format_empty_as_unknown(&right.active_licenses.as_of)
    );

    println!();
}

fn format_optional_year(value: Option<u16>) -> String {
    match value {
        Some(year) => year.to_string(),
        None => "unknown".to_string(),
    }
}

fn format_optional_count(value: Option<u32>) -> String {
    match value {
        Some(count) => count.to_string(),
        None => "unknown".to_string(),
    }
}

fn format_empty_as_unknown(value: &str) -> String {
    if value.trim().is_empty() {
        "unknown".to_string()
    } else {
        value.to_string()
    }
}

fn load_all_dossiers(data_dir: &Path) -> Result<Vec<StateDossier>> {
    let mut dossiers = Vec::new();

    let entries = fs::read_dir(data_dir)
        .with_context(|| format!("Could not read data directory: {}", data_dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|value| value.to_str()) == Some("yaml") {
            let dossier = load_dossier(&path)?;
            dossiers.push(dossier);
        }
    }

    dossiers.sort_by(|a, b| a.state.abbreviation.cmp(&b.state.abbreviation));

    Ok(dossiers)
}

fn load_dossier(path: &Path) -> Result<StateDossier> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("Could not read dossier file: {}", path.display()))?;

    let dossier: StateDossier = serde_yaml::from_str(&raw)
        .with_context(|| format!("Could not parse dossier file: {}", path.display()))?;

    Ok(dossier)
}

fn show_license_categories(data_dir: &Path) -> Result<()> {
    let dossiers = load_all_dossiers(data_dir)?;
    let mut categories: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    for dossier in dossiers {
        let state_abbr = dossier.state.abbreviation.to_uppercase();

        for license in dossier.license_types {
            if is_unknown_or_empty(&license.name) || is_unknown_or_empty(&license.category) {
                continue;
            }

            categories
                .entry(license.category)
                .or_default()
                .entry(state_abbr.clone())
                .or_default()
                .push(license.name);
        }
    }

    println!("MOBY Atlas License Categories");
    println!();

    for (category, states) in categories {
        println!("{category}:");

        for (state, mut labels) in states {
            labels.sort();
            labels.dedup();

            println!("  {}: {}", state, labels.join(", "));
        }

        println!();
    }

    Ok(())
}

fn show_tax_categories(data_dir: &Path) -> Result<()> {
    let dossiers = load_all_dossiers(data_dir)?;
    let mut categories: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    for dossier in dossiers {
        let state_abbr = dossier.state.abbreviation.to_uppercase();

        for tax in dossier.taxes {
            if is_unknown_or_empty(&tax.name) || is_unknown_or_empty(&tax.category) {
                continue;
            }

            let label = format!(
                "{} - {}",
                tax.name,
                format_empty_as_unknown(&tax.rate)
            );

            categories
                .entry(tax.category)
                .or_default()
                .entry(state_abbr.clone())
                .or_default()
                .push(label);
        }
    }

    println!("MOBY Atlas Tax Categories");
    println!();

    for (category, states) in categories {
        println!("{category}:");

        for (state, mut labels) in states {
            labels.sort();
            labels.dedup();

            println!("  {}: {}", state, labels.join(", "));
        }

        println!();
    }

    Ok(())
}

fn show_active_licenses(data_dir: &Path) -> Result<()> {
    let dossiers = load_all_dossiers(data_dir)?;

    println!("MOBY Atlas Active License Counts");
    println!();

    for dossier in dossiers {
        println!("{}:", dossier.state.abbreviation.to_uppercase());

        println!(
            "  Total: {}",
            format_optional_count(dossier.active_licenses.total)
        );

        println!(
            "  As of: {}",
            format_empty_as_unknown(&dossier.active_licenses.as_of)
        );

        println!(
            "  Source: {}",
            format_empty_as_unknown(&dossier.active_licenses.source_url)
        );

        println!(
            "  Source Quality: {}",
            format_empty_as_unknown(&dossier.active_licenses.source_quality)
        );

        println!(
            "  Confidence: {}",
            format_empty_as_unknown(&dossier.active_licenses.confidence)
        );

        if dossier.active_licenses.by_type.is_empty() {
            println!("  By Type: unknown");
        } else {
            println!("  By Type:");

            for count in dossier.active_licenses.by_type {
                println!("    - {}: {}", count.license_type, count.count);
            }
        }

        println!();
    }

    Ok(())
}

#[derive(Debug)]
struct InvalidLicenseCategory {
    state: String,
    license_name: String,
    category: String,
}

fn find_invalid_license_categories(dossiers: &[StateDossier]) -> Vec<InvalidLicenseCategory> {
    let valid_categories = canonical_license_categories();
    let mut invalid = Vec::new();

    for dossier in dossiers {
        let state = dossier.state.abbreviation.to_uppercase();

        for license in &dossier.license_types {
            if is_unknown_or_empty(&license.name) && is_unknown_or_empty(&license.category) {
                continue;
            }

            if is_unknown_or_empty(&license.category) {
                invalid.push(InvalidLicenseCategory {
                    state: state.clone(),
                    license_name: format_empty_as_unknown(&license.name),
                    category: "unknown".to_string(),
                });

                continue;
            }

            if !valid_categories.contains(&license.category.as_str()) {
                invalid.push(InvalidLicenseCategory {
                    state: state.clone(),
                    license_name: format_empty_as_unknown(&license.name),
                    category: license.category.clone(),
                });
            }
        }
    }

    invalid
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Could not create output directory: {}", parent.display()))?;
        }
    }

    Ok(())
}

fn export_json(data_dir: &Path, out: &Path) -> Result<()> {
    let dossiers = load_all_dossiers(data_dir)?;
    let json = serde_json::to_string_pretty(&dossiers)?;

    ensure_parent_dir(out)?;

    fs::write(out, json)
        .with_context(|| format!("Could not write JSON export: {}", out.display()))?;

    println!("Exported {} state dossier(s) to {}", dossiers.len(), out.display());

    Ok(())
}

fn export_csv(data_dir: &Path, out: &Path) -> Result<()> {
    let dossiers = load_all_dossiers(data_dir)?;

    ensure_parent_dir(out)?;

    let mut writer = csv::Writer::from_path(out)
        .with_context(|| format!("Could not create CSV export: {}", out.display()))?;

    for dossier in &dossiers {
        let row = StateSummaryRow {
            state: dossier.state.name.clone(),
            abbreviation: dossier.state.abbreviation.to_uppercase(),
            medical_status: dossier.programs.medical.status.clone(),
            medical_started_year: format_optional_year(dossier.programs.medical.started_year),
            adult_use_status: dossier.programs.adult_use.status.clone(),
            adult_use_started_year: format_optional_year(dossier.programs.adult_use.started_year),
            regulators: dossier
                .regulatory_bodies
                .iter()
                .map(|body| body.name.clone())
                .filter(|name| !is_unknown_or_empty(name))
                .collect::<Vec<_>>()
                .join("; "),
            track_and_trace_system: format_empty_as_unknown(&dossier.track_and_trace.system),
            track_and_trace_status: format_empty_as_unknown(&dossier.track_and_trace.status),
            license_type_count: dossier.license_types.len(),
            tax_count: dossier.taxes.len(),
            active_license_total: format_optional_count(dossier.active_licenses.total),
            active_license_as_of: format_empty_as_unknown(&dossier.active_licenses.as_of),
            active_license_source: format_empty_as_unknown(&dossier.active_licenses.source_url),
            active_license_confidence: format_empty_as_unknown(&dossier.active_licenses.confidence),
        };

        writer.serialize(row)?;
    }

    writer.flush()?;

    println!("Exported {} state summary row(s) to {}", dossiers.len(), out.display());

    Ok(())
}

fn canonical_license_categories() -> &'static [&'static str] {
    &[
        "consumption",
        "cultivation",
        "delivery",
        "distribution",
        "event",
        "incubator",
        "manufacturing",
        "medical_to_adult_use_vertical",
        "medical_vertical",
        "microbusiness",
        "operator",
        "research",
        "retail",
        "sample_collection",
        "testing_lab",
        "worker_permit",
    ]
}

fn canonical_tax_categories() -> &'static [&'static str] {
    &[
        "adult_use_tax",
        "cannabis_excise_tax",
        "cultivation_tax",
        "distributor_tax",
        "gross_receipts_tax",
        "local_option_tax",
        "medical_cannabis_tax",
        "retail_excise_tax",
        "sales_tax",
        "state_sales_tax",
        "wholesale_tax",
    ]
}