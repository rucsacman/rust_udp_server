use std::{
    net::SocketAddr, 
    borrow::BorrowMut,
    sync::{
        Arc,
    },
    io
};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    //Udp Server adress
    let remote_addr: SocketAddr = "127.0.0.1:7878".parse().expect("Error server adress");
    // We use port 0 to let the operating system allocate an available port for us. This is our client adress
    let local_addr: SocketAddr = "0.0.0.0:0".parse().expect("Error client adress");

    let socket = UdpSocket::bind(local_addr).await.expect("Error in binding");

    const MAX_DATAGRAM_SIZE: usize = 65_507;

    match socket.connect(&remote_addr).await {
        Ok(_n) => {
            let socket_lock = Arc::new(socket);
            let mut c_socket_lock = Arc::clone(&socket_lock);
            println!("Connected.. ");
            let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
            tokio::spawn( async move {
                loop {
                    //Get the data from server
                    // let l_socket = c_socket_lock.write().unwrap();
                    let len = c_socket_lock.recv(&mut data).await.expect("Data reciving failed");
                    
                    println!(
                        "Received {} bytes:\n{}",
                        len,
                        String::from_utf8_lossy(&data[..len])
                    );
                }
            });
            let mut user_buffer = String::new();
            // let mut borrow_vec = c_socket_lock.clone();
            loop {
                io::stdin().read_line(&mut user_buffer).unwrap();
                socket_lock.send(user_buffer.as_bytes()).await;
            }

        }
        Err(error) => {
            println!("Connection Failed {}", error);
        }
    }
    
    
    //Send data to the server
    

    
}