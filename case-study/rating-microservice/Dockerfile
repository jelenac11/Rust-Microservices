FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin rating
WORKDIR /rating
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/rating_microservice*
RUN cargo build

FROM debian:buster-slim

RUN mkdir rating
WORKDIR /rating

# install libpq and libsqlite
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /rating/target/debug/rating-microservice .
COPY --from=build /rating/.env .
COPY --from=build /rating/diesel.toml .

# expose port
EXPOSE 8084

# run the binary
CMD ["/rating/rating-microservice"]
