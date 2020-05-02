use std::thread;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket!");
    //let socket = UdpSocket::bind("0.0.0.0:8888")?;
    let mut buf = [0u8; 1500];
    /*
    Ok((amt, src)) = socket.recv_from(&mut buf);


    let buf = &mut buf[..amt];

    buf.reverse();
    socket.send_to(buf, &src);
    drop(socket);
    */


    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to close socket");
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    //println!("Handing connection from {}", src);
                    let buf = &mut buf[..amt];
                    //buf.reverse();
                    println!("{:?}", buf);
                    sock.send_to(buf, &src).expect("Failed to send a response");
                });
            },
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}
