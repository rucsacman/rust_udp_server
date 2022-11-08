use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
    time::Duration,
    net::SocketAddr,
    error::Error
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UdpSocket},
    time::timeout,
};
#[tokio::main]
async fn main(){
    let client_object: HashMap<usize, VecDeque<String>> = HashMap::new();
    let client_object_lock = Arc::new(RwLock::new(client_object));

    let socket = UdpSocket::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let mut buffer = [1; 500];
        let mut _len = 0;
        // let (size, addr) = socket.recv_from(&mut buffer).await.expect("ERROR");
        
        println!("Listening..");
        match socket.recv_from(&mut buffer).await{
            Ok(_) =>{
                println!("Success");
            },
            Err(_)=>{
                println!("Error",);
            }
        }
        println!("Size:");

        // match timeout(Duration::from_millis(50), socket.send_to(&mut buffer[..size], addr)).await {
        //     Ok(result) => {
        //         let result = match result {
        //             Ok(bytes) => {
        //                 println!("received bytes {}", bytes);
        //                 _len = bytes
        //             }
        //             Err(e) => {
        //                 println!("unable to read the data from the socket error: {}", e);
        //                 _len = 0;
        //             }
        //         };
        //         result
        //     }
        //     Err(_) => {
        //         _len = 0;
        //     }
        // };
    }
}

async fn handle_connection(
    mut socket: UdpSocket,
    // client_lock: &mut Arc<RwLock<HashMap<usize, VecDeque<String>>>>,
) -> Result<(), std::io::Error> {
    let client_id = get_id() - 1;
    // {
    //     let boradcast_map = &mut *client_lock.write().unwrap();
    //     boradcast_map.insert(client_id, VecDeque::new());
    //     println!("size: {}", boradcast_map.len());
    // }

    loop {
        let mut buffer = [1; 500];
        let mut _len = 0;
        let (size, addr) = socket.recv_from(&mut buffer).await?;

        match timeout(Duration::from_millis(50), socket.send_to(&mut buffer[..size], addr)).await {
            Ok(result) => {
                let result = match result {
                    Ok(bytes) => {
                        // println!("received bytes {}", bytes);
                        _len = bytes
                    }
                    Err(e) => {
                        println!("unable to read the data from the socket error: {}", e);
                        _len = 0;
                    }
                };
                result
            }
            Err(_) => {
                _len = 0;
            }
        };

        // let mut boradcast_vec = VecDeque::new();

        // {
        //     let client_map = &mut *client_lock.write().unwrap();
        //     if _len > 3 {
        //         let message = String::from_utf8_lossy(&mut buffer[.._len]);

        //         for (key, message_deque) in &mut *client_map {
        //             println!(
        //                 "clinet: {} | Push back :  key: {}  value : {} ",
        //                 client_id, key, message
        //             );
        //             message_deque.push_back(message.to_string());
        //         }
        //     }
        //     boradcast_vec = client_map.get_mut(&client_id).unwrap().clone();
        //     client_map.get_mut(&client_id).unwrap().clear();
        // }

        // let mut borrow_vec = boradcast_vec.borrow_mut();
        // let mut message: String = String::new();

        // loop {
        //     if borrow_vec.len() == 0 {
        //         break;
        //     }

        //     let value = borrow_vec.pop_front().unwrap();
        //     message.push_str(&value);
        // }

        // if message.len() > 0 {
        //     println!("clinet: {}  write message : {} ", client_id, message);
            
        //     match timeout(Duration::from_millis(50), socket.send(message.as_bytes())).await {
        //         Ok(result) => {
        //             let result = match result {
        //                 Ok(bytes) => {
        //                     let _ = socket.flush().await.unwrap();
        //                     _len = bytes
        //                 }
        //                 Err(e) => {
        //                     println!("unable to write the data from the socket error: {}", e);
        //                     _len = 0;
        //                 }
        //             };
        //             result
        //         }
        //         Err(_) => {
        //             _len = 0;
        //         }
        //     };
        // }
    }
}

fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
