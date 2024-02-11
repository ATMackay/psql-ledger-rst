#!/usr/bin/env bash

set -e

# Version control
commit_hash=$(git rev-parse HEAD)
commit_hash_short=$(git rev-parse --short HEAD)

# Set ARCH variable
architecture=$(uname -m)
if [ -z "$architecture" ]; then
    export ARCH="x86_64" # Linux, Windows (default)
else
    export ARCH=${architecture} # Mac
fi

docker build \
       --build-arg ARCH="$ARCH" \
       --build-arg BUILD_DATE="$(git show -s --format=%ci "$commit_hash")"\
       --build-arg SERVICE=psqlledger-rst \
       --build-arg GIT_SHA="$commit_hash" \
       -t "${ECR}"psqlledger-rst:latest  \
       -t "${ECR}"psqlledger-rst:"$commit_hash_short"  \
       -f Dockerfile ..