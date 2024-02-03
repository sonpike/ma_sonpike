FROM rust:alpine AS chef
RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# No idea why this second copy is needed
COPY . .
RUN cargo build --release --bin ma_sonpike

FROM alpine:latest AS runtime
COPY assets /assets
COPY --from=builder /app/target/release/ma_sonpike .
ENTRYPOINT ["/ma_sonpike"]
