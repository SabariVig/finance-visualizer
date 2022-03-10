FROM rust:1.59 AS builder

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo new --bin financereports 

WORKDIR /financereports

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --target=x86_64-unknown-linux-musl

RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/financereports*

RUN cargo build --release --target=x86_64-unknown-linux-musl


CMD ["target/x86_64-unknown-linux-musl/release/financereports"]

### Scratch 

FROM alpine

WORKDIR /app

RUN apk --no-cache add ca-certificates

COPY --from=builder /financereports/target/x86_64-unknown-linux-musl/release/ /usr/local/bin/

COPY ledger.dock .

RUN ls && pwd && cat ledger.dock

EXPOSE 8080

CMD ["financereports"]

