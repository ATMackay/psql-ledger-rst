#!/usr/bin/env bash

set -e

# Version control
commit_hash=$(git rev-parse HEAD)
commit_hash_short=$(git rev-parse --short HEAD)

docker build \
       --build-arg BUILD_DATE="$(git show -s --format=%ci "$commit_hash")"\
       --build-arg SERVICE=psql-ledger-rst \
       --build-arg GIT_SHA="$commit_hash" \
       -t "${ECR}"psql-ledger-rst:latest  \
       -t "${ECR}"psql-ledger-rst:"$commit_hash_short"  \
       -f Dockerfile ..