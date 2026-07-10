#!/bin/bash
set -e

# Update VERSION then execute this script.
# Imports cncf/xds protos into src/main/proto.
# Modeled after grpc-java's xds/third_party/xds/import.sh.

source "$(cd "$(dirname "$0")" && pwd)/../import_common.sh"

# import VERSION from one of the google internal CLs
VERSION=2ac532fd44436293585084f8d94c6bdb17835af0
DOWNLOAD_URL="https://github.com/cncf/xds/archive/${VERSION}.tar.gz"
DOWNLOAD_BASE_DIR="xds-${VERSION}"
SOURCE_PROTO_BASE_DIR="${DOWNLOAD_BASE_DIR}"
# Sorted alphabetically.
FILES=(
udpa/annotations/migrate.proto
udpa/annotations/security.proto
udpa/annotations/sensitive.proto
udpa/annotations/status.proto
udpa/annotations/versioning.proto
udpa/type/v1/typed_struct.proto
xds/annotations/v3/migrate.proto
xds/annotations/v3/security.proto
xds/annotations/v3/sensitive.proto
xds/annotations/v3/status.proto
xds/annotations/v3/versioning.proto
xds/core/v3/authority.proto
xds/core/v3/collection_entry.proto
xds/core/v3/context_params.proto
xds/core/v3/cidr.proto
xds/core/v3/extension.proto
xds/core/v3/resource_locator.proto
xds/core/v3/resource_name.proto
xds/data/orca/v3/orca_load_report.proto
xds/service/orca/v3/orca.proto
xds/type/matcher/v3/cel.proto
xds/type/matcher/v3/matcher.proto
xds/type/matcher/v3/regex.proto
xds/type/matcher/v3/string.proto
xds/type/v3/cel.proto
xds/type/matcher/v3/http_inputs.proto
xds/type/v3/typed_struct.proto
)

import_protos
