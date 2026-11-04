#!/bin/bash

# Synchronize protos from grpc-proto. For all folders in the 'for' loop,
# check their proto/ directory and overwrite .proto files that also exist in
# grpc-proto.

set -e

cd "$(dirname "$0")/.."

tmpdir="$(mktemp -d)"
curl -Ls https://github.com/grpc/grpc-proto/archive/master.tar.gz | tar xz -C "$tmpdir"
base="$tmpdir/grpc-proto-master"

# Copy protos in 'proto' from grpc-proto for these projects
for project in interop grpc-benchmark; do
  while read -r proto; do
    [ -f "$base/$proto" ] && cp "$base/$proto" "$project/third_party/grpc-proto/$proto"
    echo "$proto"
  done < <(cd "$project/third_party/grpc-proto" && find . -name "*.proto" && echo ./LICENSE)
done | sort -u > "$tmpdir/grpc-rust.lst"

(cd "$base" && find . -name "*.proto" && echo ./LICENSE) | sort > "$tmpdir/base.lst"
echo "Files synced:"
comm -12 "$tmpdir/grpc-rust.lst" "$tmpdir/base.lst"

echo
echo "Files in grpc-proto that were not used by grpc-rust:"
comm -13 "$tmpdir/grpc-rust.lst" "$tmpdir/base.lst"

echo
echo "Files in grpc-rust that don't exist in grpc-proto:"
comm -23 "$tmpdir/grpc-rust.lst" "$tmpdir/base.lst"

rm -r "$tmpdir"
