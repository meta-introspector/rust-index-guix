#!/bin/bash

set -euo pipefail

# Script to update the index-guix crate from crates.io

CRATE_NAME="index-guix"
CRATE_VERSION="1.0.3" # Hardcode for now, or fetch dynamically if needed
CRATE_FILE="${CRATE_NAME}-${CRATE_VERSION}.crate"
DOWNLOAD_URL="https://crates.io/api/v1/crates/${CRATE_NAME}/${CRATE_VERSION}/download"

# Navigate to the root of the submodule
cd "$(dirname "$0")"

echo "Updating ${CRATE_NAME} v${CRATE_VERSION} from crates.io..."

# Download the crate file
curl -L "${DOWNLOAD_URL}" -o "${CRATE_FILE}"

# Extract contents, replacing existing files
# Use --strip-components=1 to extract contents directly into the current directory
tar -xzf "${CRATE_FILE}" --strip-components=1

# Clean up the downloaded crate file
rm "${CRATE_FILE}"

echo "${CRATE_NAME} updated successfully!"
