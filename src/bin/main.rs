use clap::Parser;
use myco_kv::{eventbroker, kvmap::KVMap};
use std::{
    sync::{Arc, Mutex},
    thread,
};

mod repl;
mod server;

#[derive(Parser, Debug)]
#[command(name = "MycoKV", version = "0.1.0", author = "WVAviator")]
struct Args {
    #[arg(short, long, default_value = "6922")]
    port: Option<u16>,
}

fn main() {
    let args = Args::parse();
    let port = args.port.unwrap();

    let event_broker = eventbroker::EventBroker::new();

    let kvmap = KVMap::new(event_broker);
    let kvmap = Arc::new(Mutex::new(kvmap));

    let server_kvmap = Arc::clone(&kvmap);
    let server_thread = thread::spawn(move || {
        server::start(port, server_kvmap);
    });
    let repl_thread = thread::spawn(move || repl::start(port));

    server_thread.join().unwrap();
    repl_thread.join().unwrap();
}
