use serde::Deserialize;
use serde_json;
use std::ffi::CStr;
use std::os::raw::c_char;

fn str_slice<'a>() -> &'a str {
    extern "C" {
        fn cgo_ptr_char() -> *mut c_char;
    }
    unsafe { CStr::from_ptr(cgo_ptr_char()).to_str().unwrap_or_default() }
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Doggo {
    Name: String,
    Age: i32,
    Friend: String,
    Friend_Name: String,
    Friend_Age: i32,
}

impl Doggo {
    pub fn initialize() -> Self {
        serde_json::from_str(str_slice()).unwrap()
    }
    pub fn get_name(&self) -> &str {
        &self.Name
    }
    pub fn get_age(&self) -> &i32 {
        &self.Age
    }
    pub fn get_friend(&self) -> &str {
        &self.Friend
    }
    pub fn get_friend_name(&self) -> &str {
        &self.Friend_Name
    }
    pub fn get_friend_age(&self) -> &i32 {
        &self.Friend_Age
    }
}
