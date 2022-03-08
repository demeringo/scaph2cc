# scaph2cc

A small utility to filter and convert Scaphandre json output to a format suitable for CarbonCrush.

## Usage

```bash
cargo run -- -i tests/scaphandre-full-report.json -o tests/carbon-crush-data.json --app-id 123 --branch master --ci-pipeline-url "http://ci.com/master/123456" --process-name stress-ng
```

```bash

scaph2cc --help (or cargo run -- --help)

Filter Scaphandre json report on a specific process name and convert it to carbon crush json format

USAGE:
    scaph2cc --input-file <INPUT_FILE> --output-file <OUTPUT_FILE> --process-name <PROCESS_NAME> --app-id <APP_ID> --branch <BRANCH> --ci-pipeline-url <CI_PIPELINE_URL>

OPTIONS:
    -a, --app-id <APP_ID>                      Carbon crush app id
    -b, --branch <BRANCH>                      Name of the carbon crush file to generate
    -c, --ci-pipeline-url <CI_PIPELINE_URL>    ci pipeline URL
    -h, --help                                 Print help information
    -i, --input-file <INPUT_FILE>              Name of the scaphandre input file
    -o, --output-file <OUTPUT_FILE>            Name of the carbon crush file to generate
    -p, --process-name <PROCESS_NAME>          The process name to filter
    -V, --version                              Print version information
```
