extern crate structopt;
use structopt::clap::crate_version;
use structopt::StructOpt;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


// Good learning source for rust to build CLI programs below:
// https://rust-cli.github.io/book/tutorial/cli-args.html
// Used as template

const USAGE_DESCRIPTION: &str =
"A program that can operate both as a server to report the IP address of any HTTP GET request and a client to make that request.";


#[derive(StructOpt, Debug)]
#[structopt(
    name = "MyIP",
    version = crate_version!(),
    about = USAGE_DESCRIPTION,
    author = "Marcus Grant",
    )]
struct Command {

    #[structopt(subcommand)]
    subcommand: Option<SubCommand>,

    #[structopt(short, long, global = true)]
    debug: bool,

    /* TODO Implement proper logging
    /// Activate logging
    #[structopt(short, long, global = true)]
    log: bool,
    */
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    /// Run as server
    Server(ServerOpts),
}

#[derive(Debug, StructOpt)]
struct ServerOpts {
    /// Port to listen to
    #[structopt(default_value = "80")]
    port: u16,
}

fn main() {
    let opts = Command::from_args();

    // if opts.debug {
    //     println!("\n=== Debug Mode! ===\n");
    //     println!("Options: {:?}\n", opts);
    // }

    #[cfg(debug_assertions)]
    println!("\n=== Debug Mode! ===\n");
    println!("Options: {:?}\n", opts);


    match opts.subcommand {
        Some(SubCommand::Server(p)) => start_server(p.port),
        None => println!("Client mode not yet implemented!"),
    }
}

fn start_server(port: u16) {
    let listener = TcpListener::bind(
        format!("{}:{}", "0.0.0.0", port)).unwrap();
    println!("Server started, listening on port {:?}", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
        println!("{:?}", stream);
        get_ip_from_headers(&stream);
        respond_ok_with_ip(&stream, "69.420.13.11");
    }
}

fn print_header(buffer: [u8; 1024]) {
    println!("===Request(Start):\n{}===Request(End)", String::from_utf8_lossy(&buffer[..]));
}

fn get_ip_from_headers(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    #[cfg(debug_assertions)]
    print_header(buffer);
}

fn respond_ok_with_ip(mut stream: &TcpStream, ip: &str) {
    // let response =
    //     format!("{} 200 OK\r\n{}", HTTP_VERS, ip);
    let response = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Type: 'text/plain'; encoding=utf8"
        ip,
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
