FROM rust:1.61-alpine as builder
RUN apk add --no-cache git
WORKDIR /build
RUN git clone https://github.com/SerenityOS/jakt
RUN cd jakt && cargo install --path .

# Alpine does not have libc++ support
FROM debian:bullseye as playground
RUN apt-get update && apt-get install -y clang libc-dev libc++-dev
COPY --from=builder /usr/local/cargo/bin/jakt /usr/local/bin/jakt
COPY --from=builder /build/jakt/runtime/ /usr/local/include/runtime/
ADD ./entrypoint.sh /entrypoint.sh
RUN mkdir -p /playground \
    && chown -R nobody:nogroup /playground \
    && chmod +x /entrypoint.sh
USER nobody
WORKDIR /playground
ENTRYPOINT [ "/entrypoint.sh" ]