
FROM alpine:latest AS build

RUN apk add --no-cache curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

RUN cargo install

RUN cargo build --release --features production

FROM alpine:latest AS release

COPY --from=build /target/release/linzer-system /usr/local/bin/linzer-system

CMD ["linzer-system"]
