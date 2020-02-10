FROM rust:1.39
COPY ./ ./
RUN cargo build
CMD cargo run


EXPOSE 6443/tcp
