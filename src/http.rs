pub mod http {
    use std::{
        ffi::{c_char, CStr, CString},
        sync::Mutex
    };

    use lazy_static::lazy_static;
    use reqwest::{Url, header::{CONTENT_TYPE, HeaderMap, HeaderValue}};
    

    struct ReqInfo {
        host: String,
        con_timeout: u16,
        req_timeout: u16,
    }

    lazy_static! {
        static ref BASEINFO: Mutex<ReqInfo> = Mutex::new(ReqInfo {
            host: String::from("http://127.0.0.1"),
            con_timeout: 30,
            req_timeout: 30
        });
    }

    fn get_url(path: *const c_char) -> Option<Url>{
        let path = unsafe { CStr::from_ptr(path) }
        .to_str()
        .expect("unsafe raise error");
        let host = &BASEINFO.lock().unwrap().host;
    
        let ret = Url::parse(host.as_str()).and_then(|u| u.join(path));
        if let Ok(url) = ret {
            return Some(url);
        }
        None
    }

    #[no_mangle]
    pub extern "C" fn http_set_host(host: *const c_char) {
        BASEINFO.lock().unwrap().host = unsafe { CStr::from_ptr(host) }
            .to_str()
            .unwrap()
            .to_string();
        println!("set host suc!");
    }

    #[no_mangle]
    pub extern "C" fn http_set_timeout(t: u16) {
        BASEINFO.lock().unwrap().con_timeout = t;
        BASEINFO.lock().unwrap().req_timeout = t;
        println!("set timeout suc!");
    }

    #[no_mangle]
    pub extern "C" fn http_get(path: *const c_char) -> *mut c_char {
        // todo: return struct 
        let resp:String;
        let url = get_url(path);
        if let Some(url) = url {
            println!("req-->>{}", url.to_string());
            match reqwest::blocking::get(url) {
                Ok(t) => {
                    let r = t.text();
                    if r.is_ok() {
                        resp = format!("true|{}", &r.unwrap()).to_string();
                        println!("rep<<--{}", resp);
                    } else {
                        resp = format!("false|rep fmt error!<{}>", &r.unwrap()).to_string();
                    }
                }
                Err(e) => resp = format!("false|http get failed!<{}>", e.to_string()).to_string(),
            }
        } else {
            resp = format!("false|host parse failed!").to_string();
        };
        let ret = CString::new(resp).unwrap();
        ret.into_raw()
    }

    #[no_mangle]
    pub extern "C" fn http_post(path: *const c_char, body: *const c_char) -> *mut c_char {
        println!("start post...");
        // todo: return struct 

        fn construct_headers() -> HeaderMap {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers
        }
     
     
        let mut resp:String;
        let url = get_url(path);
        if let Some(url_) = url {
            println!("req-->>{}", url_.to_string());
            let r = match reqwest::blocking::Client::builder().build() {
                Ok(t) => {
                    let body_ = unsafe { CStr::from_ptr(body) }.to_str().unwrap();
                    let re = t.post(url_)
                    .headers(construct_headers())
                    .body(body_)
                    .send();
                    resp = match re {
                        Ok(resp) => {
                            match resp.text(){
                                Ok(resp_str) => format!("true|{}", resp_str).to_string(),
                                Err(e) =>  format!("false|rep fmt error!<{}>", e.to_string()).to_string()
                            }
                        },
                        Err(e) => format!("false|host parse failed!{}", e.to_string()).to_string()
                    };
                    resp
                }
                Err(e) => format!("false|http get failed!<{}>", e.to_string()).to_string()
            };
            resp = r;
        } else {
            resp = format!("false|host parse failed!").to_string();
        };
        let ret = CString::new(resp).unwrap();
        ret.into_raw()
    }
}
