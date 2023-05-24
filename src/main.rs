use std::{pin::Pin, future::Future};

use tower::{Service};

struct Socks5;

type Request = String;

impl Service<Request> for Socks5 {
    type Response = String;
    type Error = String;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: Request) -> Self::Future {
        todo!()
    }
}
fn main() {

}
