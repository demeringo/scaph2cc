# scaph2cc

A small utility to filter and convert Scaphandre json output to a format suitable for CarbonCrush.

## Usage

Provide the relevant process name that you want to use to filter the results.

```bash
cargo run -- -i tests/scaphandre-full-report.json -o tests/carbon-crush-data.json --app-id myapp-123 --branch master --ci-pipeline-url "http://ci.com/master/123456" --process-name stress-ng --commit-sha d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33
```

```bash

scaph2cc --help (or cargo run -- --help)


Filter Scaphandre json report on a specific process name and convert it to carbon crush json format

USAGE:
    scaph2cc --input-file <INPUT_FILE> --output-file <OUTPUT_FILE> --process-name <PROCESS_NAME> --app-id <APP_ID> --branch <BRANCH> --ci-pipeline-url <CI_PIPELINE_URL> --commit-sha <COMMIT_SHA>

OPTIONS:
    -a, --app-id <APP_ID>                      Carbon crush app id
    -b, --branch <BRANCH>                      Name of the carbon crush file to generate
    -c, --commit-sha <COMMIT_SHA>              commit identifier (sha)
    -h, --help                                 Print help information
    -i, --input-file <INPUT_FILE>              Name of the scaphandre input file
    -o, --output-file <OUTPUT_FILE>            Name of the carbon crush file to generate
    -p, --process-name <PROCESS_NAME>          The process name to filter
    -u, --ci-pipeline-url <CI_PIPELINE_URL>    ci pipeline URL
    -V, --version                              Print version information
 olivier@pad  ~/atelier/scaph/scaph2cc 
```

## Carbon Crush Format

```json
{
  "consumption": "7269278.142857143",
  "duration" : "10.00",
  "appId": "123",
  "branch": "master",
  "ciPipelineUrl": "http://ci.com/master/123456",
  "commitSha":"d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33",
  "energy" : "72692781.42857143"
}
```
