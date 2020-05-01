FROM rust:1.43 as builder
WORKDIR /usr/src/wayne-manager
COPY . .
RUN cargo install --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/wayne-manager /

CMD ["/wayne-manager"]