use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

// HTTP request 구조 정의.
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // 요청 행을 공백으로 구분된 개별 덩어리로 파싱.
    let mut words = s.split_whitespace();

    // 요청 첫 번째 행에서 HTTP 메서드 추출.
    let method = words.next().unwrap();

    // 요청 두 번째 행에서 요청 URI 추출.
    let resource = words.next().unwrap();

    // 요청 세 번째 행에서 HTTP 버전 추출.
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    // 구분자 ':'로 나눠진 헤더 행 파싱.
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    // 헤더에서 key 추출.
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    // 헤더에서 value 추출.
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        // 유입된 요청에서 각 행을 읽어냄.
        for line in req.lines() {
            // 읽은 행이 request 행이면 process_req_line()을 호출.
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;

            // 읽은 행이 header면 process_header_line()을 호출.
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);

            // 읽은 행이 빈 행이면 아무것도 안 함.
            } else if line.len() == 0 {

                // 위 조건을 모두 불만족할 경우 메시지 바디로 취급.
            } else {
                parsed_msg_body = line;
            }
        }
        // 유입되는 HTTP 요청을 HttpRequest 구조체로 파싱.
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

// HTTP 버전 확인 후 값 지정.
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

// HTTP 메서드 확인 후 값 지정.
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

// 테스트 코드.
#[cfg(test)]
mod tests {
    use super::*;
    #[test] // HTTP 메서드 테스트.
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test] // HTTP 버전 테스트.
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected: HashMap<String, String> = HashMap::new();

        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());

        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
