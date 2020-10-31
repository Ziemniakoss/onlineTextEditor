FROM opensuse/tumbleweed

RUN zypper in -y redis rust cargo gcc
COPY docker-entrypoint.sh docker-entrypoint.sh
RUN chmod 700 docker-entrypoint.sh
COPY editor-server/Cargo.toml editor-server/Cargo.toml
COPY editor-server/src/main.rs editor-server/src/main.rs
WORKDIR editor-server
RUN echo $PWD
RUN cargo fetch -q

COPY editor-server/src src/
RUN cargo b --release -q
EXPOSE 5000
ENTRYPOINT ["/docker-entrypoint.sh"]