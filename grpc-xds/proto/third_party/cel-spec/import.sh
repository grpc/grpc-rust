#!/bin/bash
set -e

# Update VERSION then execute this script.
# Imports google/cel-spec CEL expression protos into src/main/proto.
# Modeled after grpc-java's xds/third_party/cel-spec/import.sh.

source "$(cd "$(dirname "$0")" && pwd)/../import_common.sh"

VERSION="v0.15.0"
DOWNLOAD_URL="https://github.com/google/cel-spec/archive/refs/tags/${VERSION}.tar.gz"
DOWNLOAD_BASE_DIR="cel-spec-${VERSION#v}"
SOURCE_PROTO_BASE_DIR="${DOWNLOAD_BASE_DIR}/proto"
# Sorted alphabetically.
FILES=(
cel/expr/checked.proto
cel/expr/syntax.proto
)

import_protos
