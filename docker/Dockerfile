# Use a  rust alpine as the base image
FROM rust:1.85 AS builder

# Install dependencies for static linking
RUN apt-get update && apt-get install -y musl-tools
# Setup

# Read arguments
ARG ARCH
ARG SERVICE
ARG BUILD_DATE
ARG GIT_COMMIT
ARG GIT_VERSION_TAG

# Set env variables
ENV ARCH=$ARCH
ENV SERVICE_NAME=$SERVICE
ENV BUILD_DATE=$BUILD_DATE
ENV COMMIT_HASH=$GIT_COMMIT
ENV GIT_VERSION_TAG=$GIT_VERSION_TAG

RUN echo "building service: ${SERVICE_NAME}"
RUN echo "version: ${GIT_VERSION_TAG}"
RUN echo "build date: ${BUILD_DATE}"
RUN echo "commit hash: ${COMMIT_HASH}"
RUN echo "architecture: ${ARCH}"

# Set the working directory
WORKDIR /usr/src/psql-ledger-rst
# Copy entire directory (minus ignored files)
COPY ./ ./


# Enable Rust target for cross-compilation
RUN rustup target add aarch64-unknown-linux-musl x86_64-unknown-linux-musl

# Select target based on ARCH
RUN if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then \
        cargo build --release --target aarch64-unknown-linux-musl; \
    else \
        cargo build --release --target x86_64-unknown-linux-musl; \
    fi

# Final minimal image: scratch (empty base image)
FROM scratch

# Set the working directory
WORKDIR /usr/src/psql-ledger-rst

# Copy CA certificates (needed for TLS/HTTPS)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy compiled binary based on target architecture
ARG ARCH
COPY --from=builder /usr/src/psql-ledger-rst/target/${ARCH}-unknown-linux-musl/release/psql-ledger-rst /psql-ledger-rst

EXPOSE 8080

# Run the binary
CMD ["/psql-ledger-rst", "run"]