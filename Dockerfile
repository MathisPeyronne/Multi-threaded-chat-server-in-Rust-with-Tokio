#inspiration from: https://youtu.be/xuqolj01D7M?t=2192

#stage 1: generate a recipe file for dependencies
FROM rust:latest as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

#stage 2 - build dependencies
FROM rust:latest as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

#stage 3 
FROM rust:latest as builder

#Create a user
ENV USER=web
ENV UID=1001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY . /app

WORKDIR /app

COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release


FROM debian:stable-slim
#rust:1.66

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/public /app/public

COPY --from=builder /app/target/release/server_practice /app/server_practice
WORKDIR /app

USER web:web

#CMD /app
CMD ["./server_practice"]

###################### For production, you use a distroless, but for debugging with distro ######################

# FROM gcr.io/distroless/cc-debian11

# COPY --from=builder /etc/passwd /etc/passwd
# COPY --from=builder /etc/group /etc/group
# COPY --from=builder /app/public /public

# COPY --from=builder /app/target/release/server_practice /app/server_practice
# WORKDIR /app

# USER web:web

# #CMD /app
# CMD ["./server_practice"]