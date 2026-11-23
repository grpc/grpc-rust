#!/bin/bash
set -e

# Update VERSION then execute this script.
# Imports grpc/grpc-proto protos into grpc-xds/proto/third_party/grpc-proto.

source "$(cd "$(dirname "$0")" && pwd)/../import_common.sh"

VERSION=a0515b76877ff1298b0f430198043705a216541d
DOWNLOAD_URL="https://github.com/grpc/grpc-proto/archive/${VERSION}.tar.gz"
DOWNLOAD_BASE_DIR="grpc-proto-${VERSION}"
SOURCE_PROTO_BASE_DIR="${DOWNLOAD_BASE_DIR}"
# Sorted alphabetically.
FILES=(
grpc/lookup/v1/rls.proto
grpc/lookup/v1/rls_config.proto
)

import_protos
