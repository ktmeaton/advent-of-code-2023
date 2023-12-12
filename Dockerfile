FROM alpine:3

COPY target/x86_64-unknown-linux-musl/release/advent-of-code-2023 /usr/bin/advent-of-code-2023

WORKDIR "/advent-of-code-2023"

RUN set -eux
