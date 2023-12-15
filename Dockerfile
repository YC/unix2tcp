FROM alpine:3.19
LABEL org.opencontainers.image.source=https://github.com/YC/unix2tcp
RUN apk add socat=1.8.0.0-r0 --no-cache
COPY ./unix2tcp.sh /
ENTRYPOINT ["/unix2tcp.sh"]
