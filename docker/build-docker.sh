#!/usr/bin/env bash

set -e

# Version control
commit_hash=$(git rev-parse HEAD)
commit_hash_short=$(git rev-parse --short HEAD)
commit_timestamp=$(git show -s --format="%ci" ${commit_hash})
version_tag=$(git describe --tags)
build_date=$(date -u +'%Y-%m-%d %H:%M:%S')

# Set ARCH variable
architecture=$(uname -m)
if [ -z "$architecture" ]; then
    export ARCH="x86_64" # Linux, Windows (default)
else
    export ARCH=${architecture} # Mac
fi

docker build \
       --build-arg ARCH="$ARCH" \
       --build-arg BUILD_DATE="$build_date"\
       --build-arg SERVICE=psql-ledger-rst \
       --build-arg GIT_COMMIT_DATE="$commit_timestamp" \
       --build-arg GIT_VERSION_TAG="$version_tag" \
       --build-arg GIT_COMMIT="$commit_hash" \
       -t "${ECR}"psql-ledger-rst:latest  \
       -t "${ECR}"psql-ledger-rst:"$commit_hash_short" \
       -t "${ECR}"psql-ledger-rst:"$version_tag" \
       -f Dockerfile ..
# Remove intermediate Docker layers
docker image prune -f