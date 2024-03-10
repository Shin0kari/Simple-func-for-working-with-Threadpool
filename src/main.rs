use std::{
    net::{TcpListener, TcpStream},
    thread::sleep,
    time,
};

use simple_threadpool_func_bio::simple_threadpool_func::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let pool = ThreadPool::new(6);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(_stream: TcpStream) {
    println!("New pool activated");
    sleep(time::Duration::from_secs(10))
}
