FROM ekidd/rust-musl-builder
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
ADD . /home/rust/src
#RUN sudo chmod -R /home/rust/src
RUN sudo chown -R rust:rust /home/rust/src
WORKDIR /home/rust/src
RUN cargo build --release
EXPOSE 8000
ENTRYPOINT ["/bin/bash", "-c", "/home/rust/src/target/release/rustly"]
