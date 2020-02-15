FROM fedora:31
RUN yum install -y rust cargo
COPY ./ ./
RUN cargo build --release
CMD /target/release/kuvert


EXPOSE 8443/tcp
