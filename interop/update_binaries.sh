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

PLATFORMS="darwin linux windows"
ROLES="client server"
ARCH=amd64

for ROLE in $ROLES; do
  for OS in $PLATFORMS; do
    FILENAME="${ROLE}_${OS}_${ARCH}"
    if [[ "${OS}" == "windows" ]]; then FILENAME="${FILENAME}.exe"; fi
    GOOS=$OS GOARCH=$ARCH go build -o "../bin/$FILENAME" "./interop/$ROLE"
  done
done

rm -rf ../grpc-go
