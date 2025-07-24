#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use super::*;

    #[test]
    fn test_papilo_basic() {
        let problem_name = CStr::from_bytes_until_nul(b"papilo\0").unwrap();
        unsafe {
           let problem = papilo_problem_create(f64::INFINITY, problem_name.as_ptr(), 1000, 10, 10);
            assert!(!problem.is_null());
            let solver = papilo_solver_create();
            assert!(!solver.is_null());
            papilo_solver_load_problem(solver, problem);
            papilo_solver_start(solver);
            papilo_problem_free(problem);
            papilo_solver_free(solver);
        }
    }
}