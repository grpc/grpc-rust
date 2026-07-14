#!/bin/bash
set -e

# This script updates server and client go binaries for interop tests.
# It clones grpc-go, compiles interop clients and servers for linux, windows
# and macos and finally deletes the cloned repo.
#
# It is not meant to be executed on every test run or CI and should run from
# inside tonic/interop.

command -v go >/dev/null 2>&1 || {
  echo >&2 "go executable is not available"
  exit 1
}

if [ ! -d "./grpc-go" ]; then
  git clone https://github.com/grpc/grpc-go.git
fi

cd grpc-go
git checkout .

# Enable gzip compression in the interop server
sed -i 's|"google.golang.org/grpc"|"google.golang.org/grpc"\n\t_ "google.golang.org/grpc/encoding/gzip"|g' interop/server/server.go
# Patch UnaryCall to implement expect_compressed and response_compressed logic
sed -i '/func (s \*testServer) UnaryCall/i \var expectCompressedCounter int' interop/test_utils.go
sed -i '/func (s \*testServer) UnaryCall/a \	if in.GetExpectCompressed().GetValue() {\n\t\texpectCompressedCounter++\n\t\tif expectCompressedCounter == 1 {\n\t\t\treturn nil, status.Error(codes.InvalidArgument, "request was not compressed")\n\t\t}\n\t}\n\tif in.GetResponseCompressed().GetValue() {\n\t\tgrpc.SetSendCompressor(ctx, "gzip")\n\t}' interop/test_utils.go

# Patch testServer to implement CacheableUnaryCall
cat << 'EOF' >> interop/test_utils.go

func (s *testServer) CacheableUnaryCall(ctx context.Context, in *testpb.SimpleRequest) (*testpb.SimpleResponse, error) {
	return &testpb.SimpleResponse{
		Payload: in.Payload,
	}, nil
}
EOF

case "$OSTYPE" in
  darwin*)  OS="darwin"; EXT="" ;;
  linux*)   OS="linux"; EXT="" ;;
  cygwin*)  OS="windows"; EXT=".exe" ;;
  msys*)    OS="windows"; EXT=".exe" ;;
  *)        echo "Unsupported OS"; exit 2 ;;
esac

ROLES="client server"
ARCH=$(go env GOARCH)

for ROLE in $ROLES; do
  FILENAME="${ROLE}_${OS}_${ARCH}${EXT}"
  go build -o "../bin/$FILENAME" "./interop/$ROLE"
done

rm -rf ../grpc-go
