use std::net::TcpListener;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for steam in listener.incoming() {
        let steam = steam.unwrap();

        println!("Connection established!");
    }

}
