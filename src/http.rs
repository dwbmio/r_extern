pub mod http {
    use std::string;

    use lazy_static::lazy_static;

    use reqwest::Client;

    lazy_static! {
        static ref host: String = String::from("http://l27.0.0.1");
    }

    pub async extern fn SetHost(h: String){
        host = h;
    }

    #[no_mangle]
    pub async extern fn HttpGet() -> Client{
        reqwest::get()
    }
}