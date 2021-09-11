FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin users
WORKDIR /users
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/user_microservice*
RUN cargo build

FROM debian:buster-slim

RUN mkdir users
WORKDIR /users


RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /users/target/debug/user-microservice .
COPY --from=build /users/.env .
COPY --from=build /users/diesel.toml .

EXPOSE 8081

CMD ["/users/user-microservice"]
