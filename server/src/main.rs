use std::{
    // borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    sync::{
        // atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
    // time::Duration,
    net::SocketAddr,
    error::Error
};
use tokio::{
    // io::{AsyncReadExt, AsyncWriteExt},
    net::{UdpSocket},
    // time::timeout,
};

// struct Client {
//     socket: UdpSocket,
//     to_send: Option<(usize, SocketAddr)>,
// }

#[tokio::main]
async fn main(){
    let client_map: HashMap<SocketAddr, VecDeque<String>> = HashMap::new();
    let client_map_lock = Arc::new(RwLock::new(client_map));

    let socket = UdpSocket::bind("127.0.0.1:7878").await.unwrap();
    let mut socket_lock = Arc::new(socket);
    
    let mut buffer = [1; 500];
    let mut is_new_client =false;
    loop{
        let mut c_socket_lock = Arc::clone(&socket_lock);
        let c_client_map_lock = Arc::clone(&client_map_lock);
        match socket_lock.recv_from(&mut buffer).await{
            
            Ok((size, addr)) =>{
                let message = String::from_utf8_lossy(&mut buffer[..size]);
                {

                    let mut map = c_client_map_lock.write().unwrap();
                    is_new_client = map.contains_key(&addr);

                    if !is_new_client {
                        map.insert( addr, VecDeque::new() );
                    }

                    for (key, message_deque) in &mut *map {
                        println!(
                            "clinet: {}",
                            key
                        );
                        if size > 3 {
                            message_deque.push_back(message.to_string());
                        } 
                    }
                }

                if !is_new_client{
                    println!("Connection Established");

                    tokio::spawn(async move {
                        handle_connection(&mut c_socket_lock.clone(), addr, &mut c_client_map_lock.clone()).await;
                    });
                }
            },
            Err(_)=>{
                println!("Error",);
            }
        }
    }
    
}

async fn handle_connection(
    socket_lock: &mut Arc<UdpSocket>,
    client_address: SocketAddr,
    client_lock: &mut Arc<RwLock<HashMap<SocketAddr, VecDeque<String>>>>,
){

    let mut buffer = [1; 500];
    let mut _len = 0;
    println!("Listening: {}", client_address);
    
    loop {
        let mut boradcast_vec = VecDeque::new();
        // println!("client {}", client_address);
        {
            let client_map = &mut *client_lock.write().unwrap();
            if client_map.len() != 0 {
                boradcast_vec = client_map.get_mut(&client_address).unwrap().clone();
                client_map.get_mut(&client_address).unwrap().clear();
            }
            else{
                println!("Array Empty")
            }
        }
        
        loop{
            if boradcast_vec.len() == 0 {
                break;
            }

            let message = boradcast_vec.pop_front().unwrap();
            socket_lock.send_to(message.as_bytes(), client_address).await;
        } 
    }
}
