extern crate libc;

use std::ffi::CString;
use std::fmt;
use std::mem;
use std::str;

// 定义C函数的外部声明
#[link(name = "cs50")]
extern "C" {
    fn get_char(format: *const libc::c_char) -> libc::c_char;
    fn get_double(format: *const libc::c_char) -> libc::c_double;
    fn get_float(format: *const libc::c_char) -> libc::c_float;
    fn get_int(format: *const libc::c_char) -> libc::c_int;
    fn get_long(format: *const libc::c_char) -> libc::c_long;
    fn get_long_long(format: *const libc::c_char) -> libc::c_longlong;
    fn get_string(format: *const libc::c_char) -> *mut libc::c_char;
    fn free_string(s: *mut libc::c_char) -> ();
}

// 为get_string返回的字符串创建包装器类型
#[derive(Debug)]
pub struct Cs50String {
    ptr: *mut libc::c_char,
}

impl Cs50String {
    // 创建Cs50String实例
    fn new(ptr: *mut libc::c_char) -> Result<Self, EOFError> {
        if ptr.is_null() {
            Err(EOFError)
        } else {
            Ok(Cs50String { ptr })
        }
    }

    // 将Cs50String转换为Rust字符串
    pub fn to_string(&self) -> Result<String, str::Utf8Error> {
        unsafe {
            str::from_utf8(CString::from_raw(self.ptr).to_bytes()).map(|s| s.to_owned())
        }
    }
}

// 实现Drop trait以自动释放内存
impl Drop for Cs50String {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                free_string(self.ptr);
                self.ptr = std::ptr::null_mut();
            }
        }
    }
}

// 禁止克隆Cs50String，避免双重释放
impl Clone for Cs50String {
    fn clone(&self) -> Self {
        panic!("Cs50String cannot be cloned to avoid double free");
    }
}

// 实现Display trait以方便打印
impl fmt::Display for Cs50String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_string() {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "<invalid UTF-8>")
        }
    }
}

// 定义EOF错误类型
#[derive(Debug, PartialEq)]
pub struct EOFError;

impl fmt::Display for EOFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "输入错误或EOF")
    }
}

impl std::error::Error for EOFError {
    fn description(&self) -> &str {
        "输入错误或EOF"
    }
}

// 定义get_char函数
pub fn get_char(prompt: &str) -> Result<char, Box<dyn std::error::Error>> {
    let c_prompt = CString::new(prompt)?;
    let result = unsafe { get_char(c_prompt.as_ptr()) };
    
    if result == libc::c_char::max_value() {
        Err(Box::new(EOFError))
    } else {
        Ok(result as u8 as char)
    }
}

// 定义get_double函数
pub fn get_double(prompt: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let c_prompt = CString::new(prompt)?;
    let result = unsafe { get_double(c_prompt.as_ptr()) };
    
    if result == libc::c_double::max_value() {
        Err(Box::new(EOFError))
    } else {
        Ok(result as f64)
    }
}

// 定义get_float函数
pub fn get_float(prompt: &str) -> Result<f32, Box<dyn std::error::Error>> {
    let c_prompt = CString::new(prompt)?;
    let result = unsafe { get_float(c_prompt.as_ptr()) };
    
    if result == libc::c_float::max_value() {
        Err(Box::new(EOFError))
    } else {
        Ok(result as f32)
    }
}

// 定义get_int函数
pub fn get_int(prompt: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let c_prompt = CString::new(prompt)?;
    let result = unsafe { get_int(c_prompt.as_ptr()) };
    
    if result == libc::c_int::max_value() {
        Err(Box::new(EOFError))
    } else {
        Ok(result as i32)
    }
}

// 定义get_long函数
pub fn get_long(prompt: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let c_prompt = CString::new(prompt)?;
    let result = unsafe { get_long(c_prompt.as_ptr()) };
    
    if result == libc::c_long::max_value() {
        Err(Box::new(EOFError))
    } else {
        Ok(result as i64)
    }
}

// 定义get_long_long函数
pub fn get_long_long(prompt: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let c_prompt = CString::new(prompt)?;
    let result = unsafe { get_long_long(c_prompt.as_ptr()) };
    
    if result == libc::c_longlong::max_value() {
        Err(Box::new(EOFError))
    } else {
        Ok(result as i64)
    }
}

// 定义get_string函数
pub fn get_string(prompt: &str) -> Result<Cs50String, Box<dyn std::error::Error>> {
    let c_prompt = CString::new(prompt)?;
    let ptr = unsafe { get_string(c_prompt.as_ptr()) };
    Cs50String::new(ptr).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_int() {
        // 这个测试需要用户输入，所以在自动化测试中可能无法运行
        // let result = get_int("请输入一个整数: ");
        // assert!(result.is_ok());
    }
}