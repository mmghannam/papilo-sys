#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use super::papilo_problem_create;

    #[test]
    fn test_papilo_basic() {
        let problem_name = CStr::from_bytes_until_nul(b"papilo\0").unwrap();
        unsafe {
           let res = papilo_problem_create(f64::INFINITY, problem_name.as_ptr(), 1000, 10, 10);
            assert!(!res.is_null());
        }
    }
}