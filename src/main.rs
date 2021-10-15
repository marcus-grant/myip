extern crate structopt;
// use std::net::TcpListener;
// use std::env;
use structopt::clap::crate_version;
use structopt::StructOpt;


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
    // let listener = TcpListener::bind("127.0.kkk")
    // let args: Vec<String> = env::args().collect();
    let opts = Command::from_args();

    if opts.debug {
        println!("\n=== Debug Mode! ===\n");
        println!("Options: {:?}\n", opts);
    }

    match opts.subcommand {
        Some(SubCommand::Server(props)) => println!(
            "Server started, listening on port {:?}", props.port),
        None => println!("Client mode not yet implemented!"),
    }
}

