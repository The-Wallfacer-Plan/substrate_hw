use std::io::{Read, Write};
use std::net::{AddrParseError, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::str;

use clap::Parser;

/// buffer size for reading
const SIZE: usize = u8::MAX as usize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// IP address to connect to
    #[clap(short, value_parser, default_value = "127.0.0.1")]
    ip: String,
    /// port to to connect
    #[clap(short, value_parser, default_value_t = 8888)]
    port: u16,
    /// message to be sent
    #[clap(short, value_parser, default_value = "")]
    msg: String,
}

fn main() {
    // parse args with Clap
    let args: Args = Args::parse();

    // parse to get the ipv4 address
    let ipv4addr: Result<Ipv4Addr, AddrParseError> = args.ip.parse();
    // get the SocketAddr from ipv4 address and port
    let socket_addr: SocketAddr = match ipv4addr {
        // good, let's generate the address with ipv4 and port
        Ok(ipv4) => {
            SocketAddr::V4(SocketAddrV4::new(ipv4, args.port))
        }
        // bad format
        Err(_) => {
            // use the default ip address and port
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8888))
        }
    };

    // handle the connection
    match TcpStream::connect(socket_addr) {
        // connected
        Ok(mut stream) => {
            // print ok message
            println!("successfully connected to {}:{}, msg=\"{}\"", socket_addr.ip(), socket_addr.port(), args.msg);

            // get the client message in bytes format
            let msg = args.msg.as_bytes();
            // write the bytes
            stream.write(msg).unwrap();

            // generate the slice
            let mut data = [0u8; SIZE];
            // read the message replied from the server
            match stream.read(&mut data) {
                // good state
                Ok(_) => {
                    // get the replied data in str format
                    let text = str::from_utf8(&data).unwrap();
                    // message should end with the one sent from the client
                    if !&data.ends_with(msg) {
                        // just print the replied message
                        println!("reply: {}", text);
                    }
                        // unmatched
                    else {
                        // println the reply, as well as the original message
                        eprintln!("reply(unexpected): {}, original={}", text, args.msg);
                    }
                }
                // error state
                Err(e) => {
                    eprintln!("failed to receive data: {}", e);
                }
            }
        }
        // error connection
        Err(e) => {
            // just print the error message
            eprintln!("failed to connect: {}", e);
        }
    }
}
