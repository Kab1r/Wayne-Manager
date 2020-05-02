ARG BINARY_NAME_DEFAULT=wayne-manager
ARG BINARY_NAME_DEFAULT_ALT=wayne_manager

FROM clux/muslrust:stable as builder
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser
ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT
ENV BINARY_NAME_ALT=$BINARY_NAME_DEFAUKT_ALT
# Build the project with target x86_64-unknown-linux-musl

# Build dummy main with the project's Cargo lock and toml
# This is a docker trick in order to avoid downloading and building 
# dependencies when lock and toml not is modified.
COPY Cargo.lock .
COPY Cargo.toml .
COPY src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME /build-out/

# Create a minimal docker image 
FROM alpine

ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT

ENV RUST_LOG="error,$BINARY_NAME=info"
COPY --from=builder /build-out/$BINARY_NAME /

# Start with an execution list (there is no sh in a scratch image)
# No shell => no variable expansion, |, <, >, etc 
# Hard coded start command
CMD ["/wayne-manager"]