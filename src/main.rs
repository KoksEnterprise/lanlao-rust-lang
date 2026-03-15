fn main() {
    // Main program logic here.
    println!("Hello, World!");
}

// Custom panic handler
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}