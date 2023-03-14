use std::time::Duration;
use std::time::SystemTime;

pub fn now_millis() -> String {
    let ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    ms.as_millis().to_string()
}
