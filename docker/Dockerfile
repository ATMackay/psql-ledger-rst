# Use a  golang alpine as the base image
FROM rust:latest as builder

# Setup

# Read arguments
ARG ARCH
ARG BUILD_DATE
ARG GIT_SHA
ARG SERVICE

# Set env variables
ENV arch=$ARCH
ENV build_date=$BUILD_DATE
ENV commit_hash=$GIT_SHA
ENV service_name=$SERVICE
RUN echo "building service: ${service_name}, build date: ${build_date}, commit hash: ${commit_hash}, architecture: ${arch}"

# Set the working directory
WORKDIR /
COPY . .

RUN cargo build --release

# Use a minimal container for the executable
FROM debian:buster-slim


WORKDIR /usr/src/psql-ledger-rst

# Copy binary over
COPY --from=builder /usr/src/psql-ledger-rst/target/release/psql-ledger-rst /usr/src/psql-ledger-rst

# Expose the port our application is bound to
EXPOSE 8080

# Run the binary
ENTRYPOINT ["/psql-ledger-rst"]

