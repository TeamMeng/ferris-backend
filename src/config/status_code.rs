use std::collections::HashMap;
use tokio::sync::OnceCell;

pub static STATUS_CODE: OnceCell<HashMap<usize, &str>> = OnceCell::const_new();

pub async fn code_init() {
    STATUS_CODE
        .get_or_init(|| async { HashMap::from([(0, "Success"), (1, "Fail"), (200, "Ok")]) })
        .await;
}
