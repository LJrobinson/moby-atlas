use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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

    let expected_states = ["PA", "MA", "MD", "MI", "WV", "OR", "NV", "CA", "NY", "CO"];

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
        for state in missing {
            println!("- {state}");
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