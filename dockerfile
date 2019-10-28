# select build image
FROM rust:1.38 as build

# create a new empty shell project
RUN USER=root cargo new --bin my_project
WORKDIR /my_project

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# RUN cargo clean

# build for release
RUN rm ./target/release/deps/act*
RUN cargo build --release

# our final base
FROM rust:1.38-slim

RUN mkdir /static
RUN mkdir /templates
COPY static/* /static/
COPY templates/* /templates/ 
# copy the build artifact from the build stage
COPY --from=build /my_project/target/release/act .

# set the startup command to run your binary
CMD ["./act"]