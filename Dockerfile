FROM ekidd/rust-musl-builder
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir /app
WORKDIR /app
COPY . /app
RUN cargo build --release
EXPOSE 8000
WORKDIR ./target/release
RUN ./rustly
