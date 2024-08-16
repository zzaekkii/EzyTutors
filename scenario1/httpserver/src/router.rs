use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{http_request, http_request::HttpRequest, http_response::HttpResponse};
use std::io::prelude::*;
pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // GET 요청이면,
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(s) => {
                    // 해당 URI를 파싱한다.
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        // 경로가 /api로 시작하면 Web 서비스 호출.
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        // 다른 리소스에 대한 요청이면 정적 페이지 핸들러 호출.
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            // GET 요청이 아니면 404 반환.
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}