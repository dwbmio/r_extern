pub mod http;

#[no_mangle]
pub extern "C" fn add(a: i32) -> i32 {
    a * 2
}
