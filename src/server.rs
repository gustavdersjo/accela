use std::{io, thread};
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_client(mut stream: &TcpStream) {
    stream.write("Test\n".as_ref()).unwrap();
}

pub(crate) fn run() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3301")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream= stream?;
        thread::Builder::new()
            .name(format!("client handler for {}", stream.peer_addr()?.to_string()))
            .spawn(move || {
                handle_client(&stream);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }).expect("thread failed");
    }
    Ok(())
}
