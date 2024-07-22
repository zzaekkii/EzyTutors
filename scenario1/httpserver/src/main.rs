mod handler;
mod server;
mod router;
use server::Server;

fn main() {
    // 서버 가동.
    let server = Server::new("localhost:3000");

    // 서버 실행.
    server.run();
}
