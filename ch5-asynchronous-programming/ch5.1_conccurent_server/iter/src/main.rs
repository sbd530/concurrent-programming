use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

//* 반복 서버 구현 : 읽은 데이터에 대해 응답만 echo 서버
fn main() {
    // TCP 10001번 포트 리스닝
    let listner = TcpListener::bind("127.0.0.1:10001").unwrap();

    // 커넥션 요청을 수락
    while let Ok((stream, _)) = listner.accept() {
        // 읽기, 쓰기 객체 생성
        let stream0 = stream.try_clone().unwrap();
        let mut reader = BufReader::new(stream0);
        let mut writer = BufWriter::new(stream);

        // 1 라인씩 읽어서 동일한 것 을 쓴다.
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        writer.write(buf.as_bytes()).unwrap();
        writer.flush().unwrap();
    } 
    
}