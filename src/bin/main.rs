use clap::Parser;
use directories::ProjectDirs;
use myco_kv::{kvmap::KVMap, wal::WriteAheadLog, worker::Worker};
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

    #[clap(long, action)]
    purge: bool,
}

fn main() {
    let args = Args::parse();
    let port = args.port.unwrap();
    let purge = args.purge;

    let system_data_directory = ProjectDirs::from("com", "WVAviator", "MycoKV")
        .expect("Could not access system data directory.");
    let system_data_directory = system_data_directory.data_dir();

    fs::create_dir_all(system_data_directory).expect("Failed to create data directory");

    let wal_directory = system_data_directory.join("wal.mkv");
    let wal_directory = wal_directory
        .to_str()
        .expect("Invalid directory path for write-ahead log");

    let mut wal = WriteAheadLog::new(wal_directory).expect("Could not open database log.");
    if purge {
        wal.clear().expect("Could not purge logs.");
    }

    let wal = Arc::new(Mutex::new(wal));

    let mut kvmap = KVMap::new(wal);
    kvmap
        .restore()
        .expect("Could not restore database from log.");

    let kvmap = Arc::new(Mutex::new(kvmap));

    let worker_kvmap = Arc::clone(&kvmap);
    let expiration_worker = move || {
        let mut kvmap = worker_kvmap.lock().unwrap();
        kvmap.process_expirations().unwrap_or(());
    };
    let expiration_worker = Worker::new(5000, expiration_worker);
    let expiration_worker_thread = expiration_worker.start();

    let server_kvmap = Arc::clone(&kvmap);
    let server_thread = thread::spawn(move || {
        server::start(port, server_kvmap);
    });
    let repl_thread = thread::spawn(move || repl::start(port));

    server_thread.join().unwrap();
    repl_thread.join().unwrap();
    expiration_worker_thread.join().unwrap();
}
