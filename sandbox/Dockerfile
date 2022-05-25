# Alpine does not have libc++ support
FROM debian:bullseye-slim AS playground
RUN apt-get update \
    && apt-get install --no-install-recommends -y clang libc-dev libc++-dev \
    && rm -rf /var/lib/apt/lists/*
ADD ./entrypoint.sh /entrypoint.sh
RUN mkdir -p /playground \
    && chown -R nobody:nogroup /playground \
    && chmod +x /entrypoint.sh
USER nobody
WORKDIR /playground
ENTRYPOINT [ "/entrypoint.sh" ]