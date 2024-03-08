# Use a  rust alpine as the base image
FROM rust:1.67 as builder

# Setup

# Read arguments
ARG BUILD_DATE
ARG GIT_SHA
ARG SERVICE

# Set env variables
ENV build_date=$BUILD_DATE
ENV commit_hash=$GIT_SHA
ENV service_name=$SERVICE
RUN echo "building service: ${service_name}, build date: ${build_date}, commit hash: ${commit_hash}"

RUN cargo new --bin psql-ledger-rst
# Set the working directory
WORKDIR /psql-ledger-rst

# Copy essential files only
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./benches ./benches

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/psql_ledger_rst*
RUN cargo build --release

############################################################################################################

# Use a minimal container for the executable
FROM debian:bullseye-slim as slim_builder
WORKDIR /
# Copy binary over
COPY --from=builder /psql-ledger-rst/target/release/psql-ledger-rst /out/psql-ledger-rst

# Create linux svcuser
RUN mkdir /out/etc && \
    echo "svcuser:x:1010:1010::/sbin/nologin:/bin/false" > /out/etc/passwd && \
    echo "macuser:x:501:20::/sbin/nologin:/bin/false" >> /out/etc/passwd && \
    echo "linuxuser:x:1000:1000::/sbin/nologin:/bin/false" >> /out/etc/passwd && \
    echo "root:x:0:0:root:/sbin/nologin:/bin/false" >> /out/etc/passwd && \
    echo "svcgroup:x:1010:svcuser" > /out/etc/group && \
    echo "macgroup:x:20:macuser" >> /out/etc/group && \
    echo "linuxgroup:x:1000:linuxuser" >> /out/etc/group && \
    echo "root:x:0:root" >> /out/etc/group && \
    mkdir /out/config && \
    chown -R 1010:1010 /out/config


############################################################################################################

#SSL certs
FROM alpine as certs
RUN apk add --no-cache ca-certificates

############################################################################################################


# Copy binary to a scratch container. Let's keep our images nice and small!
FROM scratch
COPY --from=slim_builder /out .
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
# Set User
USER svcuser
# Expose the port your application will run on
EXPOSE 8080

# Run the binary
CMD ["/psql-ledger-rst"]

