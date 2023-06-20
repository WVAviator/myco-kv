use std::thread;

mod server;
mod repl;

fn main() {
    let server_thread = thread::spawn(server::start);
    let repl_thread = thread::spawn(repl::start);
    
    server_thread.join().unwrap();
    repl_thread.join().unwrap();
}

