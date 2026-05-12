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
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LicenseType {
    name: String,
    category: String,
    source_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaxRule {
    name: String,
    applies_to: String,
    rate: String,
    source_url: String,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ActiveLicenses {
    total: Option<u32>,
    as_of: String,
    source_url: String,
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
    notes: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { data_dir } => list_states(&data_dir),
        Commands::Show { state, data_dir } => show_state(&state, &data_dir),
        Commands::Validate { data_dir } => validate_states(&data_dir),
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