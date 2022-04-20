//! # scaph2cc
//!
//! `scaph2cc` a small CLI that allows converting Scaphandre output to CarbonCrush API input format.
//!
//! `scaph2cc` filters the Scaphandre JSON output to aggregate measures on specific processes
//! and add some context information (extracted from CLI options) about the current build
//! context.  

mod carboncrush_exporter;
use crate::carboncrush_exporter::*;

mod scaphandre_reader;

use clap::Parser;

/// The arguments of scap2cc CLi
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the scaphandre input file
    #[clap(short, long)]
    input_file: String,

    /// Name of the carbon crush file to generate
    #[clap(short, long)]
    output_file: String,

    /// The process name to filter
    #[clap(short, long)]
    process_name: String,

    /// Carbon crush app id
    #[clap(short, long)]
    app_id: String,

    /// Name of the carbon crush file to generate
    #[clap(short, long)]
    branch: String,

    /// ci pipeline URL
    #[clap(short = 'u', long)]
    ci_pipeline_url: String,

    /// commit identifier (sha)
    #[clap(short = 'c', long)]
    commit_sha: String,
}

/// scaph2cc CLI, used to filter scaphandre data and upload it to CarbonCrush API.
fn main() {
    let args = Args::parse();
    let app_id = args.app_id.as_str();
    let branch = args.branch.as_str();
    let pipeline_url = args.ci_pipeline_url.as_str();
    let scaphandre_json_file = args.input_file.as_str();
    let carboncrush_json_file = args.output_file.as_str();
    let process_name = args.process_name.as_str();
    let commit_sha = args.commit_sha.as_str();

    let average_consumption = scaphandre_reader::average_consumption(scaphandre_json_file, process_name);
    let duration = scaphandre_reader::process_duration_seconds(scaphandre_json_file, process_name);

    let total_energy = average_consumption * duration;
    println!(
        "Done. Average consumption: {}, Duration: {}, Total energy: {}",
        average_consumption, duration, total_energy
    );
    let carbon_crush_result = build_carboncrush_result(
        average_consumption,
        app_id,
        branch,
        commit_sha,
        pipeline_url,
        total_energy,
        duration,
    );
    save_carboncrush_file(carbon_crush_result, carboncrush_json_file);
}
