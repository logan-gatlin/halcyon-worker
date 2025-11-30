use std::{fmt::Debug, rc::Rc};

use halcyon_lib::compile;
use worker::*;

fn error(msg: impl Into<String>) -> Response {
    Response::builder()
        .with_status(400)
        .body(ResponseBody::Body(msg.into().into_bytes()))
}

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let data = req.form_data().await?;
    let source_code = match data.get("source") {
        Some(FormEntry::File(f)) => {
            let bytes = f.bytes().await?;
            String::from_utf8_lossy(&bytes).to_string()
        }
        _ => {
            return Ok(error(
                "Incorrect usage. Try: `curl -vfF source=@file.hc https://build.halcyon-lang.dev/ -o file.wasm`",
            ))
        }
    };
    match compile(&source_code) {
        Ok(binary) => Ok(Response::builder()
            .with_status(200)
            .body(ResponseBody::Body(binary))),
        Err(err) => Ok(error(err)),
    }
}
