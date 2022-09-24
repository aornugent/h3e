FROM rust

RUN apt-get update && apt-get -y install cmake
RUN cargo install cargo-watch

EXPOSE 3001

WORKDIR /usr/src/app

VOLUME ["/usr/local/cargo"]
