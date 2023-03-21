####################################################################################################
## Builder
####################################################################################################
FROM docker.io/library/rust:1.68 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=torad-user-info
ENV UID=10001
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /torad-user-info

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /torad-user-info

# Copy our build
COPY --from=builder /torad-user-info/target/x86_64-unknown-linux-musl/release/torad-user-info ./

# Use an unprivileged user.
USER torad-user-info:torad-user-info

EXPOSE 8080

CMD ["/torad-user-info/torad-user-info"]