FROM rust:latest
WORKDIR /app
COPY . /app
LABEL maintainer="Léopold Koprivnik Ibghy <skwal.net@gmail.com>"
RUN cargo build --release
RUN cp target/release/rust-drawing /usr/local/bin/
RUN chmod +x /usr/local/bin/rust-drawing
ENTRYPOINT ["bash"]