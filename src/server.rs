use futures::SinkExt;
use iced::stream;
use iced::futures::Stream;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use crate::Message;

pub fn run() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        dbg!("Starting server...");
        let listener = TcpListener::bind("0.0.0.0:12345").await.unwrap();
        dbg!("Started server...");

        loop {
            dbg!("Accepting...");
            let (mut socket, addr) = listener.accept().await.unwrap();
            println!("New connection from: {}", addr);

            let mut text_message = String::new();
            let mut buf = [0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("Connection closed");
                        break
                    },
                    Ok(n) => {
                        let chunk = String::from_utf8(buf[..n].to_vec()).unwrap();
                        text_message.push_str(&chunk);
                        break
                    }
                    Err(e) => {
                        println!("Failed to read from socket: {}", e);
                        break;
                    }
                }
            }
            if !text_message.is_empty() {
                println!("Received: {}", &text_message);
                if let Err(e) =  output.send(Message::BuddySent(text_message)).await {
                    println!("Failed to send message: {}", e);
                }
            }
        }
    })
}
