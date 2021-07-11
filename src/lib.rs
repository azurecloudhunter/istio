use log::trace;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;
use serde_json::{Map, Value};
use std::{error::Error};
#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpHeadersRoot) });
}

struct HttpHeadersRoot;

impl Context for HttpHeadersRoot {
}

impl RootContext for HttpHeadersRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpHeaders { context_id }))
    }

}

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {
    fn on_http_call_response(&mut self, _: u32, _: usize, _body_size: usize, _: usize) {
        let myvec = self.get_http_call_response_headers(); 
        if myvec.is_empty(){
            self.send_http_response(500, vec![], Some(b"no headers!"));
        }
        let mut stuff = Vec::new();
        for (name, value) in myvec {
            if name.to_lowercase().starts_with("x-"){
               stuff.push(format!("\"{}\":\"{}\"",&name,&value));
            }
        }
        let joined = format!("{{{}}}",stuff.join(","));
        self.set_shared_data("perfdata",Some(joined.as_bytes()),None).unwrap();
        self.resume_http_request();
        return;
    }
}

fn parse_sjson(res: &str) -> Result<Map<String, serde_json::Value>, Box<dyn Error>> {
    Ok(serde_json::from_str::<Value>(&res)?
        .as_object()
        .unwrap()
        .clone())
}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        let res = self.dispatch_http_call(
            "healthcluster",
            vec![
                (":method", "GET"),
                (":path", "/pymetric"),
                (":authority", "172.19.0.2"),
            ],
            None,
            vec![],
            Duration::from_secs(5),
        );
        if res.is_err() {
            let ss = format!("Error dispatch json: {:?}", res);
            self.send_http_response(500, vec![], Some(ss.as_bytes()));
        }
        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        match self.get_shared_data("perfdata") {
            (Some(cache), _) => {
                let mystr =  String::from_utf8(cache.clone()).unwrap();
                let headermap = parse_sjson(&mystr).unwrap();
                for (n, v) in headermap {
                    self.set_http_response_header(&n,Some(v.as_str().unwrap()));
                }
            }
            (None, _) => {
                self.send_http_response(
                    500,
                    vec![("Powered-By", "example proxy")],
                    Some(b"shared headers missing"),
                );

            }
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        trace!("#{} completed.", self.context_id);
    }
}
