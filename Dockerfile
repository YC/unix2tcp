FROM alpine:3.18
LABEL org.opencontainers.image.source=https://github.com/YC/unix2tcp
RUN apk add socat=1.7.4.4-r1 --no-cache
COPY ./unix2tcp.sh /
ENTRYPOINT ["/unix2tcp.sh"]
