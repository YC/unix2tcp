# unix2tcp
This program relays data between a Unix Domain Socket source and an address (hostname:port) destination.

## Example
Create and accept connections on /tmp/http.sock and relay them to 127.0.0.1 port 80:
```
./unix2tcp /tmp/http.sock 127.0.0.1:80
```

For reference, this can also be achieved with the following socat command:
```
socat UNIX-CONNECT:/tmp/http.sock TCP-LISTEN:80
```

## Why?
Some applications only support listening over IP ports, and it may be non-trivial to add support for Unix Domain Sockets.

My use case:
- Application and unix2tcp are in Docker containers, on same network
- Application listens over port, unix2tcp listens over unix domain socket and relays data
- Result: port is not exposed to host

However, note that this is easily achievable with socat in a Docker container.

## Architecture Overview
- Main thread accepts connections from Unix Socket
- Create new thread for every new connection
- Use poll to detect readable socket, read data from socket, and write this data to the other socket

### Downsides/Issues of this approach
- Source IPs are not retained
- Not as clean as ip2unix

## Notable projects in this area
- socat
- ip2unix
