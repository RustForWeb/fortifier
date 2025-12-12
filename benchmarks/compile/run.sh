#!/usr/bin/env bash

set -euxo pipefail

suites=(
    structs-1-fields-10
    structs-10-fields-10
)

crates=(
    fortifier
    validator
)

hyperfine \
    --setup 'cargo build -p fortifier-benchmark-compile --features={suite},{crate}' \
    --prepare 'cargo clean -p fortifier-benchmark-compile' \
    --shell=none \
    --export-markdown results.md \
    --parameter-list suite "$(IFS=, ; echo "${suites[*]}")" \
    --parameter-list crate "$(IFS=, ; echo "${crates[*]}")" \
    --command-name '{suite} {crate}' \
    'cargo build -p fortifier-benchmark-compile --features={suite},{crate}'
