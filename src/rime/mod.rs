mod rime;

use libc::{c_char, c_int};
use std::ffi::CStr;
use std::io::{Error, ErrorKind};
use std::str;

pub struct RimeApi {
    api: *const rime::RimeApi,
    session_id: rime::RimeSessionId,

    first_run: bool,
}

pub fn new_api() -> RimeApi {
    unsafe {
        let api = rime::rime_get_api();
        RimeApi {
            api,
            session_id: 0 as rime::RimeSessionId,
            first_run: true,
        }
    }
}

impl RimeApi {
    pub fn start(&mut self) -> Result<(), Error> {
        unsafe {
            let mut ptr: *const c_char = std::mem::uninitialized();

            let cfg = &mut rime::rime_traits_t {
                data_size: 0 as c_int,
                shared_data_dir: "/tmp/Rime\0".as_ptr() as *const c_char,
                user_data_dir: "/tmp/Rime\0".as_ptr() as *const c_char,
                distribution_name: "RimeRS\0".as_ptr() as *const c_char,
                distribution_code_name: "rime-rs\0".as_ptr() as *const c_char,
                distribution_version: "0.1.0\0".as_ptr() as *const c_char,
                app_name: "rime.rs\0".as_ptr() as *const c_char,
                modules: &mut ptr,
                log_dir: "/tmp/log\0".as_ptr() as *const c_char,
                min_log_level: 0 as c_int,
            };
            cfg.data_size = std::mem::size_of::<rime::rime_traits_t>() as i32;

            if self.first_run {
                println!("first run: {:?}", get_string_from_c(cfg.user_data_dir));

                match (*self.api).setup {
                    None => return Err(Error::new(ErrorKind::Other, "oh no!")),
                    Some(f) => {
                        f(cfg);
                    }
                }

                (*self).first_run = false;
            }
            if let Some(f) = (*self.api).initialize {
                f(cfg);
            }
            if let Some(f) = (*self.api).deployer_initialize {
                f(cfg);
            }
            if let Some(dir) = self.get_user_data_dir() {
                println!("done init {}", dir);
            }
            if let Some(f) = (*self.api).start_maintenance {
                f(1 as i32);
            }
            println!("done maintenance");
            if let Some(f) = (*self.api).join_maintenance_thread {
                f();
            }
            println!("done join");

            if let Some(f) = (*self.api).create_session {
                (*self).session_id = f();
            }
            println!("created session");
        }
        Err(Error::new(ErrorKind::Other, "oh yes!"))
    }

    pub fn get_version(&self) -> Option<&str> {
        unsafe {
            if let Some(f) = (*self.api).get_version {
                return Some(get_string_from_c(f()));
            }
        }
        None
    }

    pub fn get_user_data_dir(&self) -> Option<&str> {
        unsafe {
            if let Some(f) = (*self.api).get_user_data_dir {
                return Some(get_string_from_c(f()));
            }
        }
        None
    }
}

impl Drop for RimeApi {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn get_string_from_c(cs: *const c_char) -> &'static str {
    let c_str: &CStr = unsafe { CStr::from_ptr(cs) };
    let str_slice: &str = c_str.to_str().unwrap();
    // let str_buf: String = str_slice.to_owned();
    str_slice
}

#[test]
fn test_get_version() {
    let api = new_api();
    match api.get_version() {
        Some(v) => assert_eq!("1.5.3", v),
        None => assert!(false),
    }
}

#[test]
fn test_get_user_data_dir() {
    let api = new_api();
    match api.get_user_data_dir() {
        Some(v) => assert_eq!(".", v),
        None => assert!(false),
    }
}

#[test]
fn test_start() {
    let mut api = new_api();
    api.start().unwrap();
    assert_eq!(api.first_run, false);
}
