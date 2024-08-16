// 서버 모듈.
use super::router::Router;
use http::http_request::HttpRequest;
use std::io::prelude::*; // most 잘 나가는 국밥 함수 모음 라이브러리.
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        // 소켓 주소를 리스닝하는 서버 가동(소켓에 바인딩).
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        // 리스닝 투 유입되는 커넥션 인 루프.
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 90];
            stream.read(&mut read_buffer).unwrap();

            // HTTP 요청을 Rust 구조체 인스턴스로 변환.
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            // 요청을 적절한 handler로 routing(라우터로 전달).
            Router::route(req, &mut stream);
        }
    }
}