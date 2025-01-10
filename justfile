# Lists all targets
[private]
default:
    @just --list

# Build the application
build:
    cargo build --verbose

# Fixes Rust syntax
[group('format')]
[group('lint')]
cargo-format:
    cargo fmt --check

# Run `hadolint` on all `Dockerfile`s
[group('lint')]
hadolint:
    #!/usr/bin/env bash
    set -eou pipefail
    find . -type f -name "Dockerfile*" | while read -r file; do
        echo -n "Running \`hadolint\` on ${file}..."
        hadolint ${file}
        echo "{{ BOLD + GREEN }}OK{{ NORMAL }}"
    done

# Check `just` syntax
[group('lint')]
just-check:
    #!/usr/bin/env bash
    set -eou pipefail
    find . -type f -name "justfile" | while read -r file; do
        echo -n "Running \`just --fmt --check\` on ${file}..."
        just --unstable --fmt --check -f ${file}
        echo "{{ BOLD + GREEN }}OK{{ NORMAL }}"
    done

# Fixes `just` syntax
[group('format')]
[group('lint')]
just-format:
    #!/usr/bin/env bash
    set -eou pipefail
    find . -type f -name "justfile" | while read -r file; do
        echo "Running \`just --fmt\` on ${file}..."
        just --unstable --fmt -f ${file}
    done

# Runs all linters
lint: cargo-format hadolint just-check just-format renovate-validate yamllint

# Validate `renovate.json` file
[group('lint')]
renovate-validate:
    renovate-config-validator

# Test the application
test:
    cargo test --verbose

# Check YAML files with yamllint
[group('lint')]
yamllint:
    yamllint .
