extern crate cocoa;

use std::os::raw::*;

extern "C" {
    pub fn create_webview(
        parent: *const c_void,
        width: c_int,
        height: c_int,
        html: *const c_char,
        data: *const c_void,
        callback: *const c_void,
    ) -> *const c_void;
    pub fn eval_webview(webview: *const c_void, js: *const c_char);
    pub fn destroy_webview(webview: *const c_void);
}
