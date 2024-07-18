use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)] // 유도 트레이트.
pub struct HttpResponse<'a> { // 모두 같은 life time을 가짐.
    // 프로토콜 버전.
    version: &'a str,
    // 상태 코드.
    status_code: &'a str,
    // 상태 설명.
    status_text: &'a str,
    // (선택적) 헤더 리스트.
    headers: Option<HashMap<&'a str, &'a str>>,
    // (선택적) 본문.
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self{
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}