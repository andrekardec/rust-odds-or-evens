FROM rust:1.65 as builder
WORKDIR /usr/src/odds-or-evens
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/odds-or-evens /usr/local/bin/odds-or-evens
CMD ["odds-or-evens"]