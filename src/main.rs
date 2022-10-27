use std::fmt::Error;
use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::time::Duration;
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // threads.ush()
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static
    {

    }

}

pub struct  Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id: id, thread: thread }
    }
}

pub enum PoolError {
    PoolCreationError
}


    // pub fn trynew(size: usize) -> Result<ThreadPool, Error> {

    //     Ok(ThreadPool)
    // }




fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(8);

    for (tid,stream) in listener.incoming().enumerate() {
        let stream = stream.unwrap();

        pool.execute(move|| {
            handle_connection(stream);
            println!("Thread {tid} Connection established!");
        });

        // thread::spawn(move|| {
        //     handle_connection(stream);
        //     println!("Thread {tid} Connection established!");
        // });

        

    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
