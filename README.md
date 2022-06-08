# scaph2cc

A small utility to filter and convert ⚡ [Scaphandre](https://github.com/hubblo-org/scaphandre) json exporter output to a format suitable for 🌳 CarbonCrush API.

## Usage

Invoke scaph2CC through command line. It expects the following parameters:

- the scaphandre json analysis file as input
- the name of the file to be generated
- the process name that you want to use to filter the input file
- additional carbon crush metadata:
  - application id
  - ci branch measured
  - url of the current ci pipeline
  - commit id

Scaph2CC returns a json summary file that can be ingested by CarbonCrush API and a junit test report.

```bash
cargo run -- -i tests/scaphandre-full-report.json -o tests/carboncrush-data.json --junit-report-file tests/carboncrush-report.xml --app-id myapp-123 --branch master --ci-pipeline-url "http://ci.com/master/123456" --process-name stress-ng --commit-sha d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33
```

```bash
scaph2cc 0.0.5
Olivier de Meringo <demeringo@gmail.com>
The arguments of scap2cc CLi

USAGE:
    scaph2cc [OPTIONS] --input-file <INPUT_FILE> --output-file <OUTPUT_FILE> --process-name <PROCESS_NAME> --app-id <APP_ID> --branch <BRANCH> --ci-pipeline-url <CI_PIPELINE_URL> --commit-sha <COMMIT_SHA>

OPTIONS:
    -a, --app-id <APP_ID>
            Carbon crush app id

    -b, --branch <BRANCH>
            Name of the current branch

    -c, --commit-sha <COMMIT_SHA>
            commit identifier (sha)

    -h, --help
            Print help information

    -i, --input-file <INPUT_FILE>
            Name of the scaphandre result file to use as input

    -j, --junit-report-file <JUNIT_REPORT_FILE>
            Name of the junit report to generate [default: carboncrush-report.xml]

    -o, --output-file <OUTPUT_FILE>
            Name of the carbon crush file to generate

    -p, --process-name <PROCESS_NAME>
            The name of the process to filter

    -u, --ci-pipeline-url <CI_PIPELINE_URL>
            ci pipeline URL

    -V, --version
```

### Carbon Crush input Format

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
## Usage in CI

scaph2cc is included in the [scaphandre-node-ci docker image](https://github.com/demeringo/scaphandre-node-ci/)

## Thanks

🌳 _All credits go to the Scaphandre project and its great communitity_: <https://github.com/hubblo-org/scaphandre>
