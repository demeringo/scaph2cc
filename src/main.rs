//! # scaph2cc
//!
//! `scaph2cc` a small CLI that allows converting Scaphandre output to CarbonCrush API input format.
//!
//! `scaph2cc` filters the Scaphandre JSON output to aggregate measures on specific processes
//! and add some context information (extracted from CLI options) about the current build
//! context.  

mod carboncrush_exporter;
use std::path::PathBuf;

use crate::carboncrush_exporter::*;

mod scaphandre_reader;

use clap::Parser;

/// The arguments of scaph2cc CLi
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the scaphandre result file to use as input
    #[clap(short, long, parse(from_os_str))]
    input_file: PathBuf,

    /// Name of the carbon crush file to generate
    #[clap(short, long, parse(from_os_str))]
    output_file: PathBuf,

    /// Name of the junit report to generate
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "carboncrush-report.xml"
    )]
    junit_report_file: PathBuf,

    /// The name of the process to filter
    #[clap(short = 'n', long)]
    process_name: String,

      /// The pid of the process to filter
      #[clap(short = 'p', long)]
      process_id: String,

    /// Carbon crush app id
    #[clap(short, long)]
    app_id: String,

    /// Name of the current branch
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
    let scaphandre_json_file = args.input_file;
    let carboncrush_json_file = args.output_file;
    let junit_report = args.junit_report_file;
    let process_name = args.process_name.as_str();
    let commit_sha = args.commit_sha.as_str();

    let average_power_microwatts =
        scaphandre_reader::average_consumption_microwatt(&scaphandre_json_file, process_name);
    let duration_s =
        scaphandre_reader::process_duration_seconds(&scaphandre_json_file, process_name);

    let carbon_crush_result = build_carboncrush_result(
        average_power_microwatts,
        app_id,
        branch,
        commit_sha,
        pipeline_url,
        duration_s,
    );

    println!(
        "Done. Average consumption (uW): {}, Duration (s): {}, Total energy (uWs): {}, Total energy (Wh): {}",
        average_power_microwatts, duration_s, carbon_crush_result.energy, carbon_crush_result.energy_watthours
    );

    save_carboncrush_file(&carbon_crush_result, carboncrush_json_file);

    save_as_junit_report(&carbon_crush_result, junit_report);
}
