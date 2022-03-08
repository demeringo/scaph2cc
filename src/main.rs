mod cc_format;
use crate::cc_format::*;

mod scaph_reader;

use clap::Parser;

/// Filter Scaphandre json report on a specific process name and convert it to carbon crush json format
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

fn main() {
    let args = Args::parse();
    let app_id = args.app_id.as_str();
    let branch = args.branch.as_str();
    let pipeline_url = args.ci_pipeline_url.as_str();
    let scaph_filename = args.input_file.as_str();
    let cc_filename = args.output_file.as_str();
    let process_name = args.process_name.as_str();
    let commit_sha = args.commit_sha.as_str();

    let average_consumption = scaph_reader::average_consumption(scaph_filename, process_name);
    let duration = scaph_reader::process_duration_seconds(scaph_filename, process_name);

    let total_energy = average_consumption * duration;
    println!(
        "Done. Average consumption: {}, Duration: {}, Total energy: {}",
        average_consumption, duration, total_energy
    );
    let carbon_crush_results = build_cc_result(
        average_consumption,
        app_id,
        branch,
        commit_sha,
        pipeline_url,
        total_energy,
        duration,
    );
    save_cc_file(carbon_crush_results, cc_filename);
}
