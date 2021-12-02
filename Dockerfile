FROM rust:alpine3.14 AS build
COPY . .
RUN cargo build --release

FROM alpine:3
LABEL org.opencontainers.image.source=https://github.com/YC/unix2tcp
COPY --from=build target/release/unix2tcp /unix2tcp
ENTRYPOINT ["/unix2tcp"]
