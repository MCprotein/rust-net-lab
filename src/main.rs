use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("새로운 클라이언트 연결됨: {:#?}", stream.peer_addr()?);
    let mut buffer = [0_u8; 1024];
    let n = stream.read(&mut buffer)?;
    println!("받은 데이터: {:#?}", &buffer[..n]);
    stream.write_all(&buffer[..n])?;
    stream.flush()?;

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
