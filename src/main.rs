use std::{
    io::{Error, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

mod network {
    pub mod buffer;
}
use network::buffer::NetworkBuffer;

/// 실제로는 Client 프로그램이 별도로 존재
/// 네트워크 통신을 통해 서버에 메세지 전달 -> 바이트로 해야함
/// Header:
///     Protocol ID: 2 Bytes, u16
///     Message ID: 4 Bytes, u32
///     Message Type: 2 Bytes, u16 - 0x01: 메세지
///     Message Total Size: 4 Bytes, u32
///     Chunk Index: 4 Bytes, u32
/// Max Packet Size: 10 * 1024 * 1024
/// Min Packet Size: 0 Bytes 불허
#[derive(Default)]
struct Client {
    content_length: [u8; 4],
    message: Vec<u8>,
}

impl Client {
    pub fn new(raw_message: Vec<u8>) -> Self {
        let body_length: usize = raw_message.len();
        let body_length_u32: u32 = body_length
            .try_into()
            .expect("메세지 크기가 4GB를 초과하여 전송할 수 없습니다.");

        Self {
            content_length: body_length_u32.to_be_bytes(),
            message: raw_message,
        }
    }

    /// 헤더(4바이트) + 본문을 하나의 바이트 배열로 합쳐서 반환
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(4 + self.message.len());

        packet.extend_from_slice(&self.content_length);
        packet.extend_from_slice(&self.message);

        packet
    }
}

struct Header {
    content_length: usize,
}

struct Message {
    header: Header,
    body: Vec<u8>,
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let header_bytes: [u8; 4] = bytes[..4].try_into().expect("헤더 바이트가 부족합니다.");

        let content_length: usize = u32::from_be_bytes(header_bytes)
            .try_into()
            .expect("헤더가 이상해");

        let body = bytes[4..].to_vec();

        Self {
            header: Header { content_length },
            body,
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("새로운 클라이언트 연결됨: {:#?}", stream.peer_addr()?);

    let mut network_buffer = NetworkBuffer::new();

    loop {
        match client(&mut stream, &mut network_buffer) {
            Ok(byte_packets) => {
                for packet in byte_packets {
                    server(&packet);
                }
            }
            Err(e) => {
                println!("연결 종료: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn client(
    stream: &mut TcpStream,
    network_buffer: &mut NetworkBuffer,
) -> Result<Vec<Vec<u8>>, Error> {
    let mut temp_buffer = [0_u8; 1024];

    let n = stream.read(&mut temp_buffer)?;

    if n == 0 {
        stream.flush()?;
        return Err(Error::new(
            ErrorKind::ConnectionAborted,
            "클라이언트가 연결을 끊었습니다.",
        ));
    }

    let raw_message = temp_buffer[..n].to_vec();
    let client = Client::new(raw_message);
    let packet_bytes = client.to_be_bytes();

    network_buffer.push_data(&packet_bytes);

    /// 1. 클라이언트가 사용자의 메세지를 가공하여 메세지 길이를 맨앞 4바이트 헤더로 붙여서 반환
    /// 2. 네트워크버퍼에 계속 쌓아 버퍼링하면서 길이마다 자르고, 자른걸 다시 벡터에 넣는다.
    /// 3. 하나씩 서버에 보냄
    let mut completed_packets = Vec::new();
    while let Some(frame) = network_buffer.pop_frame() {
        completed_packets.push(frame);
    }

    Ok(completed_packets)
}

fn server(packet: &[u8]) {
    let message = Message::from_bytes(packet);

    println!("1. 수신된 전체 패킷 바이트: {:?}", packet);
    println!(
        "2. 파싱된 헤더 (길이): {} 바이트",
        message.header.content_length
    );
    println!(
        "3. 파싱된 본문: {}",
        String::from_utf8_lossy(&message.body).trim_end()
    );
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
