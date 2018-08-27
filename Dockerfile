FROM clux/muslrust:nightly-2018-08-23 as build
WORKDIR /usr/src
COPY . .
RUN cargo build --release

FROM scratch
EXPOSE 80
COPY --from=build /usr/src/target/x86_64-unknown-linux-musl/release/alphabet_icon /
CMD ["/alphabet_icon"]