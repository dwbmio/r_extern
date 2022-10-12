pub mod http {
    use std::{
        ffi::{c_char, CStr, CString},
        sync::Mutex,
    };

    use lazy_static::lazy_static;
    use reqwest::Url;

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
        let path = unsafe { CStr::from_ptr(path) }
            .to_str()
            .expect("unsafe raise error");
        let host = &BASEINFO.lock().unwrap().host;
        let mut resp: String = String::new();
        let ret = Url::parse(host.as_str()).and_then(|u| u.join(path));
        if let Ok(url) = ret {
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
}
