#!/bin/bash
set -e

# Update VERSION then execute this script.
# Imports envoyproxy/protoc-gen-validate protos into src/main/proto.
# Modeled after grpc-java's xds/third_party/protoc-gen-validate/import.sh.

source "$(cd "$(dirname "$0")" && pwd)/../import_common.sh"

# import VERSION from one of the google internal CLs
VERSION=dfcdc5ea103dda467963fb7079e4df28debcfd28
DOWNLOAD_URL="https://github.com/envoyproxy/protoc-gen-validate/archive/${VERSION}.tar.gz"
DOWNLOAD_BASE_DIR="protoc-gen-validate-${VERSION}"
SOURCE_PROTO_BASE_DIR="${DOWNLOAD_BASE_DIR}"
# Sorted alphabetically.
FILES=(
validate/validate.proto
)

import_protos
