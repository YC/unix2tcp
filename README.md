# unix2tcp
This Docker image relays data between a Unix Domain Socket source and an address (hostname:port) destination.

## Example
Create and accept connections on /tmp/http.sock and relay them to 127.0.0.1 port 80:
```
./unix2tcp.sh /tmp/http.sock 127.0.0.1:80
```

This is a wrapper around the following socat command:
```
socat UNIX-LISTEN:/tmp/http.sock,fork TCP-CONNECT:127.0.0.1:80
```

## Why?
Some applications only support listening over IP ports, and it may be non-trivial to add support for Unix Domain Sockets.

My use case:
- Application and unix2tcp/socat are in Docker containers, on same network
- Application listens over port, unix2tcp/socat listens over unix domain socket and relays data
- Result: port is not exposed to host

### Downsides/Issues of this approach
- Source IPs are not retained
- Not as clean as ip2unix

## Notable projects in this area
- ip2unix
