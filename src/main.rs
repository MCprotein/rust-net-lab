use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

/// 실제로는 Client 프로그램이 별도로 존재
/// 네트워크 통신을 통해 서버에 메세지 전달 -> 바이트로 해야함
struct Client {
    content_length: [u8; 4],
    message: Vec<u8>,
}

impl Client {
    pub fn new(raw_message: Vec<u8>) -> Self {
        let body_length: usize = raw_message.len() - 1;
        let body_length_u32: u32 = body_length
            .try_into()
            .expect("메세지 크기가 4GB를 초과하여 전송할 수 없습니다.");

        Self {
            content_length: body_length_u32.to_be_bytes(),
            message: raw_message,
        }
    }
}

struct Header {
    content_length: usize,
}

impl Header {
    pub fn new(content_length: usize) -> Self {
        Self { content_length }
    }
}

struct Message {
    header: Header,
    body: Vec<u8>,
}

impl Message {
    pub fn new(client: Client) -> Self {
        let content_length_u32 = u32::from_be_bytes(client.content_length);
        let content_length: usize = content_length_u32
            .try_into()
            .expect("usize 범위를 초과했습니다.");
        Self {
            header: Header { content_length },
            body: client.message,
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("새로운 클라이언트 연결됨: {:#?}", stream.peer_addr()?);
    let mut buffer = [0_u8; 1024];

    loop {
        let n = stream.read(&mut buffer)?;

        if n == 0 {
            println!("클라이언트 연결 종료");
            stream.flush()?;
            break;
        }

        println!(
            "받은 데이터: {:#?}, {:#?}",
            &buffer[..n],
            String::from_utf8_lossy(&buffer[..n]).trim_end()
        );
        stream.write_all(&buffer[..n])?;
        stream.flush()?;
    }

    Ok(())
}

fn handle_error(e: Error) -> Result<(), Error> {
    println!("오류 발생: {}", e);

    Err(e)
}

fn main() -> std::io::Result<()> {
    println!("서버 온");

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("서버 주소: {:?}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)?;
            }
            Err(e) => {
                handle_error(e)?;
            }
        }
    }
    Ok(())
}
