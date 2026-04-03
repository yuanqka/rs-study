use std::vec;

use mini_redis::{Connection, Frame};
use mini_redis::{Result, client};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
pub async fn main() {
    tokio::spawn(async {
        loop {
            let test: Vec<i32> = vec![1, 2, 3, 4, 5];
            let _test = &test;
            match client().await {
                Ok(_) => println!("任务成功完成！"),
                Err(e) => eprintln!("发生错误, 错误信息: {}", e),
            }
            tokio::task::yield_now().await;
            println!("{:?}", _test);
            /*不可编译通过, Rc没有实现Send trait, 在线程间传递是不安全的
            let rc = std::rc::Rc::new("hello");
            // `rc` is used after `.await`. It must be persisted to
            // the task's state.
            tokio::task::yield_now().await;
            println!("{}", rc);
            */
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    match TcpListener::bind("127.0.0.1:6379").await {
        Ok(listener) => loop {
            // The second item contains the IP and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();
            // A new task is spawned for each inbound socket. The socket is
            // moved to the new task and processed there.
            tokio::spawn(async move {
                process(socket).await;
            });
        },
        Err(e) => eprintln!("发生错误, 错误信息: {}", e),
    }
}

async fn client() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from the server: {:?}", result);
    Ok(())
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // A hashmap is used to store data
    let mut db = HashMap::new();

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
