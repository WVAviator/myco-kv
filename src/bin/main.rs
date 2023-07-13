use clap::Parser;
use myco_kv::{kvmap::KVMap, wal::WriteAheadLog};
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

    let wal = WriteAheadLog::new().expect("Could not open database log.");
    let wal = Arc::new(Mutex::new(wal));

    let mut kvmap = KVMap::new(wal);
    kvmap
        .restore()
        .expect("Could not restore database from log.");

    let kvmap = Arc::new(Mutex::new(kvmap));

    let server_kvmap = Arc::clone(&kvmap);
    let server_thread = thread::spawn(move || {
        server::start(port, server_kvmap);
    });
    let repl_thread = thread::spawn(move || repl::start(port));

    server_thread.join().unwrap();
    repl_thread.join().unwrap();
}
