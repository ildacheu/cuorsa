# syntax=docker/dockerfile:1

ARG APP_NAME=cuorsa

################################################################################
# Create a stage for building the application.
FROM rust:1-slim-buster AS build
ARG APP_NAME
WORKDIR /app

COPY ./src/ /app/src/
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./templates/ /app/templates/
COPY ./sqlx-data.json /app/sqlx-data.json

RUN cargo build --locked --release
RUN ls && echo sfjdfjdf

FROM debian:buster-slim AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /app/target/release/cuorsa /bin/
COPY /assets ./assets
COPY /templates ./templates

# Expose the port that the application listens on.
EXPOSE 3000 

# What the container should run when it is started.
CMD ["/bin/cuorsa"]
