#!/bin/bash
set -e

# Re-imports every vendored xDS proto dependency by running each
# subdirectory's import.sh.
#
# Each dependency is pinned to a specific upstream version (kept in sync with
# grpc-java's xds/third_party import scripts) so the whole set forms a
# coherent, mutually-compatible snapshot. To bump a single dependency, edit
# its VERSION in the corresponding <name>/import.sh and run that script
# directly instead.

cd "$(dirname "$0")"

for script in */import.sh; do
  name="$(dirname "${script}")"
  echo "==> Importing ${name}"
  ./"${script}"
done

echo "All xDS protos imported."
