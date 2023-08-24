FROM busybox:glibc
COPY ./target/x86_64-unknown-linux-gnu/release/pork-ddns .

ENV RUST_LOG WARN
CMD ./pork-ddns