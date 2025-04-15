#!/usr/bin/env bash
set -euo pipefail

# Script to verify that the crate version hasn't been published yet
# and is greater than the latest published version on crates.io

# Get the local version from Cargo.toml
LOCAL_VERSION=$(cargo metadata --format-version=1 --no-deps | \
                jq -r '.packages[] | select(.name == "fib-rs") | .version')

echo "Local version: $LOCAL_VERSION"

# Get the latest published version from crates.io
LATEST_VERSION=$(cargo info fib-rs --registry=crates-io | awk '/^version:/ {print $2}')

if [ -z "$LATEST_VERSION" ]; then
    echo "No published version found. This is a new crate."
    exit 0
fi

echo "Latest published version: $LATEST_VERSION"

# Compare the versions using sort -V for correct semantic version comparison
if [ "$(echo -e "$LOCAL_VERSION\n$LATEST_VERSION" | sort -V | tail -n1)" != "$LOCAL_VERSION" ]; then
    echo "ERROR: Local version $LOCAL_VERSION is not greater than the latest published version $LATEST_VERSION"
    echo "Please update the version in Cargo.toml"
    exit 1
fi

echo "Version check passed: $LOCAL_VERSION > $LATEST_VERSION"

# Check if the package would pass validation for publishing
echo "Running cargo package to verify the crate..."
cargo package --workspace --all-features 

echo "All checks passed! Version $LOCAL_VERSION is valid for publishing."
exit 0