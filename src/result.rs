use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct APIResult<T: Serialize> {
    pub ok: bool,
    pub err: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> APIResult<T> {
    pub fn from_result(result: Result<T>) -> APIResult<T> {
        match result {
            Ok(r) => {
                APIResult {
                    ok: true,
                    err: None,
                    data: Some(r),
                }
            }
            Err(x) => {
                APIResult {
                    ok: false,
                    err: Some(x.to_string()),
                    data: None,
                }
            }
        }
    }
}
