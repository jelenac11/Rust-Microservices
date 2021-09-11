FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin posts
WORKDIR /posts
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/actix_example*
RUN cargo build

FROM debian:buster-slim

RUN mkdir posts
WORKDIR /posts

# install libpq and libsqlite
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /posts/target/debug/actix-example .
COPY --from=build /posts/.env .
COPY --from=build /posts/diesel.toml .

# expose port
EXPOSE 8082

# run the binary
CMD ["/posts/actix-example"]
