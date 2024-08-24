use std::sync::Mutex;

// 애플리케이션 상태 정의.
pub struct AppState {
    pub health_check_response: String, // 공유된 이뮤터블.
    pub visit_count: Mutex<u32>, // 공유된 뮤터블.
}