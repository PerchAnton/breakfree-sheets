use crate::persistence::{APP_ID, FORMAT_ID};
use serde_json::{Value, json};
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn capabilities() -> Value {
    json!({"protocol_version":1,"app_id":APP_ID,"app_version":VERSION,"display_name":"BreakFree Sheets",
           "capabilities":["new","open"],"formats":[{"id":FORMAT_ID,"extensions":["bfsheet"],"read":true,"write":true}]})
}
