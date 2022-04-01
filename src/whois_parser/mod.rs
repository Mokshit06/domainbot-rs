use ffi::{GoString, WhoIsParser, WhoIsParser_return};
use std::ffi::{CStr, CString};

mod ffi;

#[derive(Debug)]
pub struct WhoIsInfo {
    pub taken: bool,
    pub tld_supported: bool,
    pub registrar_name: String,
    pub expiration_date: String,
}

impl WhoIsInfo {
    pub fn parse(raw: &str) -> WhoIsInfo {
        let c_raw = CString::new(raw).expect("CString::new failed");
        let ptr = c_raw.as_ptr();
        let go_string = GoString {
            p: ptr,
            n: c_raw.as_bytes().len() as isize,
        };
        let WhoIsParser_return {
            r0: taken,
            r1: tld_supported,
            r2: registar,
            r3: expiration_date,
        } = unsafe { WhoIsParser(go_string) };

        WhoIsInfo {
            taken: taken != 0,
            tld_supported: tld_supported != 0,
            registrar_name: unsafe { CStr::from_ptr(registar).to_string_lossy() }.to_string(),
            expiration_date: unsafe { CStr::from_ptr(expiration_date).to_string_lossy() }
                .to_string(),
        }
    }
}
