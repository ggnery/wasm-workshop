extern "C" {
    pub fn alert(index: i32);
}

#[no_mangle] 
pub fn greet(index: i32) {
    println!("Hello from guest function, received {index}");
    unsafe {
        alert(64);
    }
}