extern crate libc;
use libc::size_t;


#[no_mangle]
pub extern fn Hello() {
        println!("Hello World");
    
}
