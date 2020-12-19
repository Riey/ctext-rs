//! This library is wrapper for xcb-imdkit with dynamic linkage

use std::os::raw::c_char;
use std::sync::Once;

fn init() {
    const ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe { crate::ffi::xcb_compound_text_init() });
}

pub fn utf8_to_compound_text(text: &str) -> Result<Vec<u8>, ()> {
    init();

    let mut len = 0;

    unsafe {
        let ptr: *mut c_char = crate::ffi::xcb_utf8_to_compound_text(
            text.as_ptr() as *const _,
            text.len() as _,
            &mut len,
        );

        if ptr.is_null() {
            Err(())
        } else {
            let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, len as usize);
            let out = slice.to_vec();
            let _ = slice;
            libc::free(ptr as *mut _);

            Ok(out)
        }
    }
}

pub fn compound_text_to_utf8(bytes: &[u8]) -> Result<String, ()> {
    init();

    let mut len = 0;

    unsafe {
        let ptr: *mut c_char = crate::ffi::xcb_compound_text_to_utf8(
            bytes.as_ptr() as *const _,
            bytes.len() as _,
            &mut len,
        );

        if ptr.is_null() {
            Err(())
        } else {
            let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, len as usize);
            let out = slice.to_vec();
            let _ = slice;
            libc::free(ptr as *mut _);

            String::from_utf8(out).map_err(|_| ())
        }
    }
}

mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn korean() {
        const UTF8: &str = "가나다";
        const COMP: &[u8] = &[
            27, 37, 71, 234, 176, 128, 235, 130, 152, 235, 139, 164, 27, 37, 64,
        ];
        assert_eq!(crate::utf8_to_compound_text(UTF8).unwrap(), COMP);
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }
}
