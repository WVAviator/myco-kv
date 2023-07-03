use clap::Parser;
use logger::Logger;
use myco_kv::{eventbroker, kvmap::KVMap};
use std::{
    sync::{Arc, Mutex},
    thread,
};

mod logger;
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
    let event_broker = Arc::new(Mutex::new(event_broker));

    let logger = Logger::new();
    let mut kvmap = KVMap::new(Arc::clone(&event_broker));
    logger.restore(&mut kvmap);

    {
        let mut event_broker = event_broker.lock().unwrap();
        event_broker.subscribe(Box::new(logger));
    }

    let kvmap = Arc::new(Mutex::new(kvmap));

    let server_kvmap = Arc::clone(&kvmap);
    let server_thread = thread::spawn(move || {
        server::start(port, server_kvmap);
    });
    let repl_thread = thread::spawn(move || repl::start(port));

    server_thread.join().unwrap();
    repl_thread.join().unwrap();
}
