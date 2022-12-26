FROM alpine:3
LABEL org.opencontainers.image.source=https://github.com/YC/unix2tcp
RUN apk add socat --no-cache
COPY ./unix2tcp.sh /
ENTRYPOINT ["/unix2tcp.sh"]
