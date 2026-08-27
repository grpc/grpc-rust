#!/bin/bash
set -e

# Update VERSION then execute this script.
# Imports googleapis protos into src/main/proto.
# Modeled after grpc-java's xds/third_party/googleapis/import.sh.

source "$(cd "$(dirname "$0")" && pwd)/../import_common.sh"

VERSION=114a745b2841a044e98cdbb19358ed29fcf4a5f1
DOWNLOAD_URL="https://github.com/googleapis/googleapis/archive/${VERSION}.tar.gz"
DOWNLOAD_BASE_DIR="googleapis-${VERSION}"
SOURCE_PROTO_BASE_DIR="${DOWNLOAD_BASE_DIR}"
# Sorted alphabetically.
# annotations.proto/http.proto/status.proto are not needed by grpc-java (it
# resolves them from external Bazel/Maven deps), but we vendor them so protoc
# can resolve every non-well-known import from the committed tree.
FILES=(
google/api/annotations.proto
google/api/expr/v1alpha1/checked.proto
google/api/expr/v1alpha1/syntax.proto
google/api/http.proto
google/rpc/status.proto
)

import_protos
