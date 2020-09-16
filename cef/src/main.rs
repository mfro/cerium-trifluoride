#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_variables)]

extern crate cef_sys;
extern crate winapi;

use cef_sys::*;
use std::ptr;

use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::{
    CW_USEDEFAULT, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

mod include;
pub use include::*;

pub fn execute_process(main_args: &MainArgs, app: CefApp) -> i32 {
    unsafe { cef_execute_process(&main_args.raw, CefApp::to_cef_own(app), ptr::null_mut()) }
}

pub fn initialize(main_args: &MainArgs, settings: &CefSettings, app: CefApp) -> i32 {
    unsafe {
        cef_initialize(
            &main_args.raw,
            &settings.raw,
            CefApp::to_cef_own(app),
            ptr::null_mut(),
        )
    }
}

struct MyApp;
impl App for MyApp {
    fn on_before_command_line_processing(
        &mut self,
        process_type: Option<&CefString>,
        command_line: CefCommandLine,
    ) {
        let mut command_line = command_line;
        println!("on_before_command_line_processing {:?}", process_type);
        println!("{}", command_line.get_program());
    }
}

struct MyClient {
    life_span_handler: CefLifeSpanHandler,
}

impl Client for MyClient {
    fn get_life_span_handler(&mut self) -> Option<CefLifeSpanHandler> {
        Some(self.life_span_handler.clone())
    }
}

struct MyLifeSpanHandler;
impl LifeSpanHandler for MyLifeSpanHandler {
    fn on_before_close(&mut self, browser: CefBrowser) {
        unsafe { cef_quit_message_loop() };
    }
}

#[repr(C)]
struct CefSettings2 {
    raw: cef_sys::cef_settings_t,
}

impl CefSettings2 {}

fn main() {
    // for x in std::env::args() {
    //     print!("{} ", x);
    // }
    // println!();

    let main_args = unsafe { MainArgs::new(GetModuleHandleA(std::ptr::null()) as _) };

    let app = CefApp::new(MyApp);
    // app.lock().on_before_command_line_processing(None, "");

    // println!("{:?} {:?}", &app as *const _, &app.raw as *const _);

    if execute_process(&main_args, app.clone()) >= 0 {
        return;
    }

    let settings = CefSettings::default()
        .set_no_sandbox(1)
        .set_log_severity(CefLogSeverity::WARNING)
        .build();

    initialize(&main_args, &settings, app.clone());

    let window_info = CefWindowInfo::default()
        .set_style(WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | WS_VISIBLE)
        .set_x(CW_USEDEFAULT)
        .set_y(CW_USEDEFAULT)
        .set_width(1280)
        .set_height(720)
        .set_window_name("hello, world!")
        .build();

    let client = CefClient::new(MyClient {
        life_span_handler: MyLifeSpanHandler.into(),
    });

    let url = CefString::new("https://www.google.com/");

    let mut browser_settings = cef_browser_settings_t::default();
    browser_settings.size = std::mem::size_of::<cef_browser_settings_t>() as u64;

    unsafe {
        cef_browser_host_create_browser(
            &window_info.raw,
            CefClient::to_cef_own(client),
            url.into_cef(),
            &browser_settings,
            ptr::null_mut(),
            ptr::null_mut(),
        );

        cef_run_message_loop();

        cef_shutdown();
    }
}
