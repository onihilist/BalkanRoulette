FROM alpine:latest
COPY target/x86_64-unknown-linux-musl/release/material .
CMD ["./material"]