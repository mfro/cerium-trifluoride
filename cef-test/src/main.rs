extern crate cef;
extern crate winapi;

use cef::*;

use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::{
    CW_USEDEFAULT, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

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

    fn get_render_process_handler(&mut self) -> Option<CefRenderProcessHandler> {
        Some(CefRenderProcessHandler::new(MyRenderProcessHandler))
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
    fn on_before_close(&mut self, _browser: CefBrowser) {
        cef_quit_message_loop();
    }
}

struct MyRenderProcessHandler;
impl RenderProcessHandler for MyRenderProcessHandler {
    fn on_context_created(
        &mut self,
        _browser: CefBrowser,
        _frame: CefFrame,
        context: CefV8Context,
    ) -> () {
        let mut context = context;
        let mut globals = context.get_global().unwrap();

        let string = CefV8Value::create_string(Some(&CefString::new("test"))).unwrap();

        globals.set_value_bykey(
            Some(&CefString::new("test")),
            string,
            CefV8Propertyattribute::NONE,
        );
    }
}

fn main() {
    let main_args = unsafe { CefMainArgs::new(GetModuleHandleA(std::ptr::null()) as _) };

    let app = CefApp::new(MyApp);

    if cef_execute_process(&main_args, Some(app.clone()), None) >= 0 {
        return;
    }

    let settings = CefSettings::default()
        .set_no_sandbox(1)
        .set_log_severity(CefLogSeverity::ERROR)
        .build();

    cef_initialize(&main_args, &settings, Some(app.clone()), None);

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

    let browser_settings = CefBrowserSettings::default();

    let mut browser = CefBrowserHost::create_browser_sync(
        &window_info,
        Some(client.clone()),
        Some(&url),
        &browser_settings,
        None,
        None,
    )
    .unwrap();

    let mut host = browser.get_host().unwrap();

    host.show_dev_tools(
        Some(&window_info),
        Some(client.clone()),
        Some(&browser_settings),
        None,
    );

    cef_run_message_loop();

    cef_shutdown();
}
