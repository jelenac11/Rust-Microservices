FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin search
WORKDIR /search
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/search_microservice*
RUN cargo build

FROM debian:buster-slim

RUN mkdir search
WORKDIR /search

# install libpq and libsqlite
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /search/target/debug/search-microservice .
COPY --from=build /search/.env .
COPY --from=build /search/diesel.toml .

# expose port
EXPOSE 8083

# run the binary
CMD ["/search/search-microservice"]
