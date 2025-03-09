# Use a  rust alpine as the base image
FROM rust:1.67 as builder

# Setup

# Read arguments
ARG ARCH
ARG BUILD_DATE
ARG GIT_SHA
ARG SERVICE
ARG VERSION

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

RUN cargo build -r

# Use a minimal container for the executable
FROM debian:bullseye-slim
WORKDIR /usr/src/psql-ledger-rst
# Copy binary over
COPY --from=builder /usr/src/psql-ledger-rst/target/release/psql-ledger-rst /usr/src/psql-ledger-rst

# Expose the port our application is bound to
EXPOSE 8080

# Run the binary
CMD ["/psql-ledger-rst"]

