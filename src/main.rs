#![no_std]
#![no_main]
#![allow(nonstandard_style)]
#![allow(unused_parens)]
// make sure to point rust to the nightly library to enable development
// need to create git repo with my main account as laptop connects to school :(

use core::panic::PanicInfo;

// to run: cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
// defined in .cargo/config.toml as well

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vgaBuffer: *mut u8 = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vgaBuffer.offset(i as isize * 2) = byte;
            *vgaBuffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    return loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    return loop {};
}



