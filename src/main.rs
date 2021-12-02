use mio::{
    net::{TcpStream, UnixStream},
    Events, Interest, Poll, Token,
};
use std::{
    env, fs,
    io::{ErrorKind, Read, Write},
    net::ToSocketAddrs,
    os::unix::net::UnixListener,
    path::Path,
    process, thread,
};

fn main() -> std::io::Result<()> {
    // Parse commandline arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <path> <addr>", &args[0]);
        process::exit(1);
    }
    let path = Path::new(&args[1]);

    // Parse addr
    // TODO: Should try next() in turn, connect may fail otherwise
    // e.g. localhost resolves to ::1, but app not listening on ipv6
    let addr = &args[2];
    let addr = addr.to_socket_addrs().unwrap().next().unwrap();
    println!(
        "Initialising with path: {}, addr: {}",
        path.to_str().unwrap(),
        addr,
    );

    // Check socket file existance, remove if exists
    if path.exists() {
        fs::remove_file(path).expect("cannot remove existing socket file");
    }
    // Open Unix Socket
    let listener = UnixListener::bind(path)?;

    // Accept new connections
    loop {
        match listener.accept() {
            Ok((unix_stream, _addr)) => {
                if let (Ok(mut addr_stream), Ok(())) =
                    (TcpStream::connect(addr), unix_stream.set_nonblocking(true))
                {
                    let mut socket = mio::net::UnixStream::from_std(unix_stream);
                    thread::spawn(move || redirect_data(&mut socket, &mut addr_stream));
                }
            }
            Err(e) => {
                println!("accept function failed: {:?}", e);
            }
        }
    }
}

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn redirect_data(
    unix_stream: &mut UnixStream,
    addr_stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a poll instance
    let mut poll = Poll::new()?;
    // Create storage for events
    let mut events = Events::with_capacity(1024);

    // Start listening for reads
    poll.registry()
        .register(addr_stream, SERVER, Interest::READABLE)?;
    poll.registry()
        .register(unix_stream, CLIENT, Interest::READABLE)?;

    loop {
        // Poll Mio for events, blocking until we get an event
        poll.poll(&mut events, None)?;

        for event in &events {
            match event.token() {
                // Read from server, addr to unix socket
                SERVER => redirect_data_sockets(addr_stream, unix_stream)?,
                // Read from client, unix socket to addr
                CLIENT => redirect_data_sockets(unix_stream, addr_stream)?,
                _ => unreachable!(),
            }
        }
    }
}

fn redirect_data_sockets(
    reading_socket: &mut dyn Read,
    writing_socket: &mut dyn Write,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buf = [0_u8; 256];

    loop {
        // Read into buffer
        match reading_socket.read(&mut buf) {
            Ok(read_len) => {
                if read_len == 0 {
                    return Ok(());
                }
                // Write until all written
                let mut written = 0;
                while read_len != written {
                    written += writing_socket.write(&buf[written..read_len])?;
                }
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    // kind: WouldBlock, message: "Resource temporarily unavailable"
                    return Ok(());
                } else {
                    return Err(Box::new(e));
                }
            }
        }
    }
}
