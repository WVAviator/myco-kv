use clap::Parser;
use directories::ProjectDirs;
use myco_kv::{kvmap::KVMap, wal::WriteAheadLog};
use std::{
    fs,
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

    let system_data_directory = ProjectDirs::from("com", "WVAviator", "MycoKV")
        .expect("Could not access system data directory.");
    let system_data_directory = system_data_directory.data_dir();

    fs::create_dir_all(system_data_directory).expect("Failed to create data directory");

    let wal_directory = system_data_directory.join("wal.mkv");
    let wal_directory = wal_directory
        .to_str()
        .expect("Invalid directory path for write-ahead log");

    let wal = WriteAheadLog::new(wal_directory).expect("Could not open database log.");
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
