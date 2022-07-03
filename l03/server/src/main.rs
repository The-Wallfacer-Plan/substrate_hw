use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::net::{AddrParseError, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::thread;

use clap::Parser;

/// buffer size for reading
const SIZE: usize = u8::MAX as usize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// IP address to listen on
    #[clap(short, value_parser, default_value = "127.0.0.1")]
    ip: String,
    /// port to listen on
    #[clap(short, value_parser, default_value_t = 8888)]
    port: u16,
}

/// handle client messages, reply format: [ip:port] message
/// # Arguments:
/// * `stream` - tcp stream containing the messages from the client
/// * `socket_addr` - peer connection address for the current connection, it provides `ip:port`
fn handle_msg(mut stream: TcpStream, socket_addr: SocketAddr) {
    // buffer data
    let mut data = [0u8; SIZE];
    // to indicate whether the connection is lost
    let mut lost = false;
    // keep reading the data/message
    while match stream.read(&mut data) {
        // deal with the data
        Ok(size) => {
            // if reading data is ok
            if size != 0 {
                // write with the reply message, part1 [ip:port]
                stream.write(format!("[{}:{}] ", socket_addr.ip(), socket_addr.port()).as_bytes()).unwrap();
                // indiate the data is empty, tell that no message is sent by the client
                if size <= 2 {
                    // no message
                    stream.write("[NO MESSAGE]\n".as_ref()).unwrap();
                } else {
                    // echo client's message
                    stream.write(&data[0..size]).unwrap();
                }
            }
                // data size is 0, indicating the connection is lost
            else {
                // print the error message
                eprintln!("connection lost for {}:{}", socket_addr.ip(), socket_addr.port());
                // mark connection lost
                lost = true;
            }
            // good state
            true
        }
        // with an error during connection
        Err(e) => {
            // print with the error message
            println!("connection error: {}, terminating connection", e);
            // shutdown
            let res = stream.shutdown(Shutdown::Both);
            // match with the
            match res {
                Ok(_) => {
                    // print termination message
                    println!("successfully terminated");
                }
                Err(error) => {
                    // just tell the error
                    eprintln!("shutting down error: {}", error);
                }
            }
            // error state
            false
        }
    } {
        // if connection is lost, break out(exit)
        if lost {
            break;
        }
    }
}

fn main() {

    // part with clap
    let args = Args::parse();

    // parse to get the ipv4 address
    let ipv4addr: Result<Ipv4Addr, AddrParseError> = args.ip.parse();

    // get the address in `SocketAddr` form
    let socket_addr: SocketAddr = match ipv4addr {
        // good, no problem
        Ok(ipv4) => {
            // add the port to form SocketAddr
            SocketAddr::V4(SocketAddrV4::new(ipv4, args.port))
        }
        // error, use the default
        Err(_) => {
            // use the default
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8888))
        }
    };

    // get the tcp listener
    let listener = TcpListener::bind(socket_addr).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("server listening on {}:{}", socket_addr.ip(), socket_addr.port());
    // keep dealing with the incoming connection
    for stream in listener.incoming() {
        // deal with the stream from the client
        match stream {
            // good connection
            Ok(stream) => {
                // get the peer connection address (auto-generated)
                let addr: SocketAddr = stream.peer_addr().unwrap();
                println!("new peer connection at {}:{}", addr.ip(), addr.port());
                // spawn the thread and deal with it, to keep the main thread always available
                thread::spawn(move || {
                    // connection succeeded
                    handle_msg(stream, addr)
                });
            }
            // bad connection
            Err(e) => {
                // connection failed
                eprintln!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
