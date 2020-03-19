//#![windows_subsystem = "windows"]

use lib::{JavascriptCallback, PluginGui};
use std::os::raw::{c_void, c_char};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::rc::Rc;
use std::str;
use webview_sys::*;

#[no_mangle]
extern "C" fn listener(this: *const c_void, result: *const c_char) -> *mut c_char {
    unsafe {
        let that = this as *const Gui;
        let res = CStr::from_ptr(result).to_string_lossy().into_owned();
        let str = (*(*that).js_callback)(res);
        let c_str = CString::new(str).unwrap();
        c_str.into_raw()
    }
}

struct Gui {
    html_content: String,
    window_size: Option<(i32, i32)>,
    is_open: bool,
    web_view: Option<*const c_void>,
    js_callback: Rc<JavascriptCallback>,
}

impl PluginGui for Gui {
    fn size(&self) -> (i32, i32) {
        match self.window_size {
            Some(inner) => inner,
            None => (0, 0)
        }
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn close(&mut self) {
        unsafe {
            match self.web_view {
                Some(inner) => {
                    destroy_webview(inner);
                    self.web_view = None;
                },
                None => ()
            }
        }
        self.is_open = false;
    }

    fn open(&mut self, parent_handle: *mut c_void) -> bool {
        unsafe {
            let html = CString::new(self.html_content.as_str()).expect("");
            let ptr: *const Gui = self;
            let (width, height) = self.window_size.unwrap();
            self.web_view = Some(create_webview(parent_handle, width, height, html.as_ptr(), ptr as *const c_void, listener as *const c_void));
        }
        true
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn execute(&self, javascript_code: &str) -> Result<(), Box<dyn Error>> {
        unsafe {
            let js = CString::new(javascript_code)?;
            match self.web_view {
                Some(inner) => {
                    eval_webview(inner, js.as_ptr());
                },
                None => ()
            }
        }
        Ok(())
    }
}

pub fn new_plugin_gui(
    html_document: String,
    js_callback: JavascriptCallback,
    window_size: Option<(i32, i32)>) -> Box<dyn PluginGui>
{
    Box::new(
        Gui {
            html_content: html_document,
            is_open: false,
            js_callback: Rc::new(js_callback),
            window_size: window_size,
            web_view: None,
        })
}

