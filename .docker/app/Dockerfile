FROM clux/muslrust:stable as builder
WORKDIR /home/rust/src
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/ma_sonpike /
EXPOSE 3000
CMD ["/ma_sonpike"]
