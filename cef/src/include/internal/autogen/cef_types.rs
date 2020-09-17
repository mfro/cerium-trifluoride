/// Log severity levels.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefLogSeverity(cef_sys::cef_log_severity_t);
impl CefLogSeverity {
  /// Default logging (currently INFO logging).
  pub const DEFAULT: CefLogSeverity = CefLogSeverity(cef_sys::cef_log_severity_t_LOGSEVERITY_DEFAULT);
  /// Verbose logging.
  pub const VERBOSE: CefLogSeverity = CefLogSeverity(cef_sys::cef_log_severity_t_LOGSEVERITY_VERBOSE);
  /// DEBUG logging.
  pub const DEBUG: CefLogSeverity = CefLogSeverity(cef_sys::cef_log_severity_t_LOGSEVERITY_DEBUG);
  /// INFO logging.
  pub const INFO: CefLogSeverity = CefLogSeverity(cef_sys::cef_log_severity_t_LOGSEVERITY_INFO);
  /// WARNING logging.
  pub const WARNING: CefLogSeverity = CefLogSeverity(cef_sys::cef_log_severity_t_LOGSEVERITY_WARNING);
  /// ERROR logging.
  pub const ERROR: CefLogSeverity = CefLogSeverity(cef_sys::cef_log_severity_t_LOGSEVERITY_ERROR);
  /// FATAL logging.
  pub const FATAL: CefLogSeverity = CefLogSeverity(cef_sys::cef_log_severity_t_LOGSEVERITY_FATAL);
  /// Disable logging to file for all messages, and to stderr for messages with
  /// severity less than FATAL.
  pub const DISABLE: CefLogSeverity = CefLogSeverity(cef_sys::cef_log_severity_t_LOGSEVERITY_DISABLE);
}
impl From<cef_sys::cef_log_severity_t> for CefLogSeverity {
  fn from(raw: cef_sys::cef_log_severity_t) -> CefLogSeverity {
    CefLogSeverity(raw)
  }
}
impl From<CefLogSeverity> for cef_sys::cef_log_severity_t {
  fn from(src: CefLogSeverity) -> cef_sys::cef_log_severity_t {
    src.0
  }
}
impl std::ops::BitOr for CefLogSeverity {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Represents the state of a setting.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefState(cef_sys::cef_state_t);
impl CefState {
  /// Use the default state for the setting.
  pub const DEFAULT: CefState = CefState(cef_sys::cef_state_t_STATE_DEFAULT);
  /// Enable or allow the setting.
  pub const ENABLED: CefState = CefState(cef_sys::cef_state_t_STATE_ENABLED);
  /// Disable or disallow the setting.
  pub const DISABLED: CefState = CefState(cef_sys::cef_state_t_STATE_DISABLED);
}
impl From<cef_sys::cef_state_t> for CefState {
  fn from(raw: cef_sys::cef_state_t) -> CefState {
    CefState(raw)
  }
}
impl From<CefState> for cef_sys::cef_state_t {
  fn from(src: CefState) -> cef_sys::cef_state_t {
    src.0
  }
}
impl std::ops::BitOr for CefState {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Initialization settings. Specify NULL or 0 to get the recommended default
/// values. Many of these and other settings can also configured using command-
/// line switches.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefSettings { pub raw: cef_sys::cef_settings_t }
impl Default for CefSettings {
  fn default() -> CefSettings {
    let /* mut */ raw = cef_sys::cef_settings_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_settings_t>() as u64;
    CefSettings { raw }
  }
}
impl From<CefSettings> for cef_sys::cef_settings_t {
  fn from(src: CefSettings) -> cef_sys::cef_settings_t {
    src.raw
  }
}
impl From<cef_sys::cef_settings_t> for CefSettings {
  fn from(raw: cef_sys::cef_settings_t) -> CefSettings {
    CefSettings { raw }
  }
}
#[allow(non_snake_case)]
impl CefSettings {
  /// Size of this structure.
  pub fn size(&self) -> u64 { self.raw.size }
  /// Size of this structure.
  pub fn set_size(&mut self, v: u64) -> &mut Self { self.raw.size = v; self }
  /// Set to true (1) to disable the sandbox for sub-processes. See
  /// cef_sandbox_win.h for requirements to enable the sandbox on Windows. Also
  /// configurable using the "no-sandbox" command-line switch.
  pub fn no_sandbox(&self) -> i32 { self.raw.no_sandbox }
  /// Set to true (1) to disable the sandbox for sub-processes. See
  /// cef_sandbox_win.h for requirements to enable the sandbox on Windows. Also
  /// configurable using the "no-sandbox" command-line switch.
  pub fn set_no_sandbox(&mut self, v: i32) -> &mut Self { self.raw.no_sandbox = v; self }
  /// The path to a separate executable that will be launched for sub-processes.
  /// If this value is empty on Windows or Linux then the main process executable
  /// will be used. If this value is empty on macOS then a helper executable must
  /// exist at "Contents/Frameworks/<app> Helper.app/Contents/MacOS/<app> Helper"
  /// in the top-level app bundle. See the comments on CefExecuteProcess() for
  /// details. If this value is non-empty then it must be an absolute path. Also
  /// configurable using the "browser-subprocess-path" command-line switch.
  pub fn browser_subprocess_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.browser_subprocess_path) }
  /// The path to a separate executable that will be launched for sub-processes.
  /// If this value is empty on Windows or Linux then the main process executable
  /// will be used. If this value is empty on macOS then a helper executable must
  /// exist at "Contents/Frameworks/<app> Helper.app/Contents/MacOS/<app> Helper"
  /// in the top-level app bundle. See the comments on CefExecuteProcess() for
  /// details. If this value is non-empty then it must be an absolute path. Also
  /// configurable using the "browser-subprocess-path" command-line switch.
  pub fn set_browser_subprocess_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.browser_subprocess_path, v); self }
  /// The path to the CEF framework directory on macOS. If this value is empty
  /// then the framework must exist at "Contents/Frameworks/Chromium Embedded
  /// Framework.framework" in the top-level app bundle. If this value is
  /// non-empty then it must be an absolute path. Also configurable using the
  /// "framework-dir-path" command-line switch.
  pub fn framework_dir_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.framework_dir_path) }
  /// The path to the CEF framework directory on macOS. If this value is empty
  /// then the framework must exist at "Contents/Frameworks/Chromium Embedded
  /// Framework.framework" in the top-level app bundle. If this value is
  /// non-empty then it must be an absolute path. Also configurable using the
  /// "framework-dir-path" command-line switch.
  pub fn set_framework_dir_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.framework_dir_path, v); self }
  /// The path to the main bundle on macOS. If this value is empty then it
  /// defaults to the top-level app bundle. If this value is non-empty then it
  /// must be an absolute path. Also configurable using the "main-bundle-path"
  /// command-line switch.
  pub fn main_bundle_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.main_bundle_path) }
  /// The path to the main bundle on macOS. If this value is empty then it
  /// defaults to the top-level app bundle. If this value is non-empty then it
  /// must be an absolute path. Also configurable using the "main-bundle-path"
  /// command-line switch.
  pub fn set_main_bundle_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.main_bundle_path, v); self }
  /// Set to true (1) to enable use of the Chrome runtime in CEF. This feature is
  /// considered experimental and is not recommended for most users at this time.
  /// See issue #2969 for details.
  pub fn chrome_runtime(&self) -> i32 { self.raw.chrome_runtime }
  /// Set to true (1) to enable use of the Chrome runtime in CEF. This feature is
  /// considered experimental and is not recommended for most users at this time.
  /// See issue #2969 for details.
  pub fn set_chrome_runtime(&mut self, v: i32) -> &mut Self { self.raw.chrome_runtime = v; self }
  /// Set to true (1) to have the browser process message loop run in a separate
  /// thread. If false (0) than the CefDoMessageLoopWork() function must be
  /// called from your application message loop. This option is only supported on
  /// Windows and Linux.
  pub fn multi_threaded_message_loop(&self) -> i32 { self.raw.multi_threaded_message_loop }
  /// Set to true (1) to have the browser process message loop run in a separate
  /// thread. If false (0) than the CefDoMessageLoopWork() function must be
  /// called from your application message loop. This option is only supported on
  /// Windows and Linux.
  pub fn set_multi_threaded_message_loop(&mut self, v: i32) -> &mut Self { self.raw.multi_threaded_message_loop = v; self }
  /// Set to true (1) to control browser process main (UI) thread message pump
  /// scheduling via the CefBrowserProcessHandler::OnScheduleMessagePumpWork()
  /// callback. This option is recommended for use in combination with the
  /// CefDoMessageLoopWork() function in cases where the CEF message loop must be
  /// integrated into an existing application message loop (see additional
  /// comments and warnings on CefDoMessageLoopWork). Enabling this option is not
  /// recommended for most users; leave this option disabled and use either the
  /// CefRunMessageLoop() function or multi_threaded_message_loop if possible.
  pub fn external_message_pump(&self) -> i32 { self.raw.external_message_pump }
  /// Set to true (1) to control browser process main (UI) thread message pump
  /// scheduling via the CefBrowserProcessHandler::OnScheduleMessagePumpWork()
  /// callback. This option is recommended for use in combination with the
  /// CefDoMessageLoopWork() function in cases where the CEF message loop must be
  /// integrated into an existing application message loop (see additional
  /// comments and warnings on CefDoMessageLoopWork). Enabling this option is not
  /// recommended for most users; leave this option disabled and use either the
  /// CefRunMessageLoop() function or multi_threaded_message_loop if possible.
  pub fn set_external_message_pump(&mut self, v: i32) -> &mut Self { self.raw.external_message_pump = v; self }
  /// Set to true (1) to enable windowless (off-screen) rendering support. Do not
  /// enable this value if the application does not use windowless rendering as
  /// it may reduce rendering performance on some systems.
  pub fn windowless_rendering_enabled(&self) -> i32 { self.raw.windowless_rendering_enabled }
  /// Set to true (1) to enable windowless (off-screen) rendering support. Do not
  /// enable this value if the application does not use windowless rendering as
  /// it may reduce rendering performance on some systems.
  pub fn set_windowless_rendering_enabled(&mut self, v: i32) -> &mut Self { self.raw.windowless_rendering_enabled = v; self }
  /// Set to true (1) to disable configuration of browser process features using
  /// standard CEF and Chromium command-line arguments. Configuration can still
  /// be specified using CEF data structures or via the
  /// CefApp::OnBeforeCommandLineProcessing() method.
  pub fn command_line_args_disabled(&self) -> i32 { self.raw.command_line_args_disabled }
  /// Set to true (1) to disable configuration of browser process features using
  /// standard CEF and Chromium command-line arguments. Configuration can still
  /// be specified using CEF data structures or via the
  /// CefApp::OnBeforeCommandLineProcessing() method.
  pub fn set_command_line_args_disabled(&mut self, v: i32) -> &mut Self { self.raw.command_line_args_disabled = v; self }
  /// The location where data for the global browser cache will be stored on
  /// disk. If this value is non-empty then it must be an absolute path that is
  /// either equal to or a child directory of CefSettings.root_cache_path. If
  /// this value is empty then browsers will be created in "incognito mode" where
  /// in-memory caches are used for storage and no data is persisted to disk.
  /// HTML5 databases such as localStorage will only persist across sessions if a
  /// cache path is specified. Can be overridden for individual CefRequestContext
  /// instances via the CefRequestContextSettings.cache_path value.
  pub fn cache_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.cache_path) }
  /// The location where data for the global browser cache will be stored on
  /// disk. If this value is non-empty then it must be an absolute path that is
  /// either equal to or a child directory of CefSettings.root_cache_path. If
  /// this value is empty then browsers will be created in "incognito mode" where
  /// in-memory caches are used for storage and no data is persisted to disk.
  /// HTML5 databases such as localStorage will only persist across sessions if a
  /// cache path is specified. Can be overridden for individual CefRequestContext
  /// instances via the CefRequestContextSettings.cache_path value.
  pub fn set_cache_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.cache_path, v); self }
  /// The root directory that all CefSettings.cache_path and
  /// CefRequestContextSettings.cache_path values must have in common. If this
  /// value is empty and CefSettings.cache_path is non-empty then it will
  /// default to the CefSettings.cache_path value. If this value is non-empty
  /// then it must be an absolute path. Failure to set this value correctly may
  /// result in the sandbox blocking read/write access to the cache_path
  /// directory.
  pub fn root_cache_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.root_cache_path) }
  /// The root directory that all CefSettings.cache_path and
  /// CefRequestContextSettings.cache_path values must have in common. If this
  /// value is empty and CefSettings.cache_path is non-empty then it will
  /// default to the CefSettings.cache_path value. If this value is non-empty
  /// then it must be an absolute path. Failure to set this value correctly may
  /// result in the sandbox blocking read/write access to the cache_path
  /// directory.
  pub fn set_root_cache_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.root_cache_path, v); self }
  /// The location where user data such as spell checking dictionary files will
  /// be stored on disk. If this value is empty then the default
  /// platform-specific user data directory will be used ("~/.cef_user_data"
  /// directory on Linux, "~/Library/Application Support/CEF/User Data" directory
  /// on Mac OS X, "Local Settings\Application Data\CEF\User Data" directory
  /// under the user profile directory on Windows). If this value is non-empty
  /// then it must be an absolute path.
  pub fn user_data_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.user_data_path) }
  /// The location where user data such as spell checking dictionary files will
  /// be stored on disk. If this value is empty then the default
  /// platform-specific user data directory will be used ("~/.cef_user_data"
  /// directory on Linux, "~/Library/Application Support/CEF/User Data" directory
  /// on Mac OS X, "Local Settings\Application Data\CEF\User Data" directory
  /// under the user profile directory on Windows). If this value is non-empty
  /// then it must be an absolute path.
  pub fn set_user_data_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.user_data_path, v); self }
  /// To persist session cookies (cookies without an expiry date or validity
  /// interval) by default when using the global cookie manager set this value to
  /// true (1). Session cookies are generally intended to be transient and most
  /// Web browsers do not persist them. A |cache_path| value must also be
  /// specified to enable this feature. Also configurable using the
  /// "persist-session-cookies" command-line switch. Can be overridden for
  /// individual CefRequestContext instances via the
  /// CefRequestContextSettings.persist_session_cookies value.
  pub fn persist_session_cookies(&self) -> i32 { self.raw.persist_session_cookies }
  /// To persist session cookies (cookies without an expiry date or validity
  /// interval) by default when using the global cookie manager set this value to
  /// true (1). Session cookies are generally intended to be transient and most
  /// Web browsers do not persist them. A |cache_path| value must also be
  /// specified to enable this feature. Also configurable using the
  /// "persist-session-cookies" command-line switch. Can be overridden for
  /// individual CefRequestContext instances via the
  /// CefRequestContextSettings.persist_session_cookies value.
  pub fn set_persist_session_cookies(&mut self, v: i32) -> &mut Self { self.raw.persist_session_cookies = v; self }
  /// To persist user preferences as a JSON file in the cache path directory set
  /// this value to true (1). A |cache_path| value must also be specified
  /// to enable this feature. Also configurable using the
  /// "persist-user-preferences" command-line switch. Can be overridden for
  /// individual CefRequestContext instances via the
  /// CefRequestContextSettings.persist_user_preferences value.
  pub fn persist_user_preferences(&self) -> i32 { self.raw.persist_user_preferences }
  /// To persist user preferences as a JSON file in the cache path directory set
  /// this value to true (1). A |cache_path| value must also be specified
  /// to enable this feature. Also configurable using the
  /// "persist-user-preferences" command-line switch. Can be overridden for
  /// individual CefRequestContext instances via the
  /// CefRequestContextSettings.persist_user_preferences value.
  pub fn set_persist_user_preferences(&mut self, v: i32) -> &mut Self { self.raw.persist_user_preferences = v; self }
  /// Value that will be returned as the User-Agent HTTP header. If empty the
  /// default User-Agent string will be used. Also configurable using the
  /// "user-agent" command-line switch.
  pub fn user_agent(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.user_agent) }
  /// Value that will be returned as the User-Agent HTTP header. If empty the
  /// default User-Agent string will be used. Also configurable using the
  /// "user-agent" command-line switch.
  pub fn set_user_agent(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.user_agent, v); self }
  /// Value that will be inserted as the product portion of the default
  /// User-Agent string. If empty the Chromium product version will be used. If
  /// |userAgent| is specified this value will be ignored. Also configurable
  /// using the "product-version" command-line switch.
  pub fn product_version(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.product_version) }
  /// Value that will be inserted as the product portion of the default
  /// User-Agent string. If empty the Chromium product version will be used. If
  /// |userAgent| is specified this value will be ignored. Also configurable
  /// using the "product-version" command-line switch.
  pub fn set_product_version(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.product_version, v); self }
  /// The locale string that will be passed to WebKit. If empty the default
  /// locale of "en-US" will be used. This value is ignored on Linux where locale
  /// is determined using environment variable parsing with the precedence order:
  /// LANGUAGE, LC_ALL, LC_MESSAGES and LANG. Also configurable using the "lang"
  /// command-line switch.
  pub fn locale(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.locale) }
  /// The locale string that will be passed to WebKit. If empty the default
  /// locale of "en-US" will be used. This value is ignored on Linux where locale
  /// is determined using environment variable parsing with the precedence order:
  /// LANGUAGE, LC_ALL, LC_MESSAGES and LANG. Also configurable using the "lang"
  /// command-line switch.
  pub fn set_locale(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.locale, v); self }
  /// The directory and file name to use for the debug log. If empty a default
  /// log file name and location will be used. On Windows and Linux a "debug.log"
  /// file will be written in the main executable directory. On Mac OS X a
  /// "~/Library/Logs/<app name>_debug.log" file will be written where <app name>
  /// is the name of the main app executable. Also configurable using the
  /// "log-file" command-line switch.
  pub fn log_file(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.log_file) }
  /// The directory and file name to use for the debug log. If empty a default
  /// log file name and location will be used. On Windows and Linux a "debug.log"
  /// file will be written in the main executable directory. On Mac OS X a
  /// "~/Library/Logs/<app name>_debug.log" file will be written where <app name>
  /// is the name of the main app executable. Also configurable using the
  /// "log-file" command-line switch.
  pub fn set_log_file(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.log_file, v); self }
  /// The log severity. Only messages of this severity level or higher will be
  /// logged. When set to DISABLE no messages will be written to the log file,
  /// but FATAL messages will still be output to stderr. Also configurable using
  /// the "log-severity" command-line switch with a value of "verbose", "info",
  /// "warning", "error", "fatal" or "disable".
  pub fn log_severity(&self) -> &crate::include::internal::CefLogSeverity { unsafe { &*(self as *const _ as *const _) } }
  /// The log severity. Only messages of this severity level or higher will be
  /// logged. When set to DISABLE no messages will be written to the log file,
  /// but FATAL messages will still be output to stderr. Also configurable using
  /// the "log-severity" command-line switch with a value of "verbose", "info",
  /// "warning", "error", "fatal" or "disable".
  pub fn set_log_severity(&mut self, v: crate::include::internal::CefLogSeverity) -> &mut Self { self.raw.log_severity = v.into(); self }
  /// Custom flags that will be used when initializing the V8 JavaScript engine.
  /// The consequences of using custom flags may not be well tested. Also
  /// configurable using the "js-flags" command-line switch.
  pub fn javascript_flags(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.javascript_flags) }
  /// Custom flags that will be used when initializing the V8 JavaScript engine.
  /// The consequences of using custom flags may not be well tested. Also
  /// configurable using the "js-flags" command-line switch.
  pub fn set_javascript_flags(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.javascript_flags, v); self }
  /// The fully qualified path for the resources directory. If this value is
  /// empty the cef.pak and/or devtools_resources.pak files must be located in
  /// the module directory on Windows/Linux or the app bundle Resources directory
  /// on Mac OS X. If this value is non-empty then it must be an absolute path.
  /// Also configurable using the "resources-dir-path" command-line switch.
  pub fn resources_dir_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.resources_dir_path) }
  /// The fully qualified path for the resources directory. If this value is
  /// empty the cef.pak and/or devtools_resources.pak files must be located in
  /// the module directory on Windows/Linux or the app bundle Resources directory
  /// on Mac OS X. If this value is non-empty then it must be an absolute path.
  /// Also configurable using the "resources-dir-path" command-line switch.
  pub fn set_resources_dir_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.resources_dir_path, v); self }
  /// The fully qualified path for the locales directory. If this value is empty
  /// the locales directory must be located in the module directory. If this
  /// value is non-empty then it must be an absolute path. This value is ignored
  /// on Mac OS X where pack files are always loaded from the app bundle
  /// Resources directory. Also configurable using the "locales-dir-path"
  /// command-line switch.
  pub fn locales_dir_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.locales_dir_path) }
  /// The fully qualified path for the locales directory. If this value is empty
  /// the locales directory must be located in the module directory. If this
  /// value is non-empty then it must be an absolute path. This value is ignored
  /// on Mac OS X where pack files are always loaded from the app bundle
  /// Resources directory. Also configurable using the "locales-dir-path"
  /// command-line switch.
  pub fn set_locales_dir_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.locales_dir_path, v); self }
  /// Set to true (1) to disable loading of pack files for resources and locales.
  /// A resource bundle handler must be provided for the browser and render
  /// processes via CefApp::GetResourceBundleHandler() if loading of pack files
  /// is disabled. Also configurable using the "disable-pack-loading" command-
  /// line switch.
  pub fn pack_loading_disabled(&self) -> i32 { self.raw.pack_loading_disabled }
  /// Set to true (1) to disable loading of pack files for resources and locales.
  /// A resource bundle handler must be provided for the browser and render
  /// processes via CefApp::GetResourceBundleHandler() if loading of pack files
  /// is disabled. Also configurable using the "disable-pack-loading" command-
  /// line switch.
  pub fn set_pack_loading_disabled(&mut self, v: i32) -> &mut Self { self.raw.pack_loading_disabled = v; self }
  /// Set to a value between 1024 and 65535 to enable remote debugging on the
  /// specified port. For example, if 8080 is specified the remote debugging URL
  /// will be http://localhost:8080. CEF can be remotely debugged from any CEF or
  /// Chrome browser window. Also configurable using the "remote-debugging-port"
  /// command-line switch.
  pub fn remote_debugging_port(&self) -> i32 { self.raw.remote_debugging_port }
  /// Set to a value between 1024 and 65535 to enable remote debugging on the
  /// specified port. For example, if 8080 is specified the remote debugging URL
  /// will be http://localhost:8080. CEF can be remotely debugged from any CEF or
  /// Chrome browser window. Also configurable using the "remote-debugging-port"
  /// command-line switch.
  pub fn set_remote_debugging_port(&mut self, v: i32) -> &mut Self { self.raw.remote_debugging_port = v; self }
  /// The number of stack trace frames to capture for uncaught exceptions.
  /// Specify a positive value to enable the CefRenderProcessHandler::
  /// OnUncaughtException() callback. Specify 0 (default value) and
  /// OnUncaughtException() will not be called. Also configurable using the
  /// "uncaught-exception-stack-size" command-line switch.
  pub fn uncaught_exception_stack_size(&self) -> i32 { self.raw.uncaught_exception_stack_size }
  /// The number of stack trace frames to capture for uncaught exceptions.
  /// Specify a positive value to enable the CefRenderProcessHandler::
  /// OnUncaughtException() callback. Specify 0 (default value) and
  /// OnUncaughtException() will not be called. Also configurable using the
  /// "uncaught-exception-stack-size" command-line switch.
  pub fn set_uncaught_exception_stack_size(&mut self, v: i32) -> &mut Self { self.raw.uncaught_exception_stack_size = v; self }
  /// Set to true (1) to ignore errors related to invalid SSL certificates.
  /// Enabling this setting can lead to potential security vulnerabilities like
  /// "man in the middle" attacks. Applications that load content from the
  /// internet should not enable this setting. Also configurable using the
  /// "ignore-certificate-errors" command-line switch. Can be overridden for
  /// individual CefRequestContext instances via the
  /// CefRequestContextSettings.ignore_certificate_errors value.
  pub fn ignore_certificate_errors(&self) -> i32 { self.raw.ignore_certificate_errors }
  /// Set to true (1) to ignore errors related to invalid SSL certificates.
  /// Enabling this setting can lead to potential security vulnerabilities like
  /// "man in the middle" attacks. Applications that load content from the
  /// internet should not enable this setting. Also configurable using the
  /// "ignore-certificate-errors" command-line switch. Can be overridden for
  /// individual CefRequestContext instances via the
  /// CefRequestContextSettings.ignore_certificate_errors value.
  pub fn set_ignore_certificate_errors(&mut self, v: i32) -> &mut Self { self.raw.ignore_certificate_errors = v; self }
  /// Background color used for the browser before a document is loaded and when
  /// no document color is specified. The alpha component must be either fully
  /// opaque (0xFF) or fully transparent (0x00). If the alpha component is fully
  /// opaque then the RGB components will be used as the background color. If the
  /// alpha component is fully transparent for a windowed browser then the
  /// default value of opaque white be used. If the alpha component is fully
  /// transparent for a windowless (off-screen) browser then transparent painting
  /// will be enabled.
  pub fn background_color(&self) -> &crate::include::internal::CefColor { unsafe { &*(self as *const _ as *const _) } }
  /// Background color used for the browser before a document is loaded and when
  /// no document color is specified. The alpha component must be either fully
  /// opaque (0xFF) or fully transparent (0x00). If the alpha component is fully
  /// opaque then the RGB components will be used as the background color. If the
  /// alpha component is fully transparent for a windowed browser then the
  /// default value of opaque white be used. If the alpha component is fully
  /// transparent for a windowless (off-screen) browser then transparent painting
  /// will be enabled.
  pub fn set_background_color(&mut self, v: crate::include::internal::CefColor) -> &mut Self { self.raw.background_color = v.into(); self }
  /// Comma delimited ordered list of language codes without any whitespace that
  /// will be used in the "Accept-Language" HTTP header. May be overridden on a
  /// per-browser basis using the CefBrowserSettings.accept_language_list value.
  /// If both values are empty then "en-US,en" will be used. Can be overridden
  /// for individual CefRequestContext instances via the
  /// CefRequestContextSettings.accept_language_list value.
  pub fn accept_language_list(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.accept_language_list) }
  /// Comma delimited ordered list of language codes without any whitespace that
  /// will be used in the "Accept-Language" HTTP header. May be overridden on a
  /// per-browser basis using the CefBrowserSettings.accept_language_list value.
  /// If both values are empty then "en-US,en" will be used. Can be overridden
  /// for individual CefRequestContext instances via the
  /// CefRequestContextSettings.accept_language_list value.
  pub fn set_accept_language_list(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.accept_language_list, v); self }
  /// GUID string used for identifying the application. This is passed to the
  /// system AV function for scanning downloaded files. By default, the GUID
  /// will be an empty string and the file will be treated as an untrusted
  /// file when the GUID is empty.
  pub fn application_client_id_for_file_scanning(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.application_client_id_for_file_scanning) }
  /// GUID string used for identifying the application. This is passed to the
  /// system AV function for scanning downloaded files. By default, the GUID
  /// will be an empty string and the file will be treated as an untrusted
  /// file when the GUID is empty.
  pub fn set_application_client_id_for_file_scanning(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.application_client_id_for_file_scanning, v); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Request context initialization settings. Specify NULL or 0 to get the
/// recommended default values.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefRequestContextSettings { pub raw: cef_sys::cef_request_context_settings_t }
impl Default for CefRequestContextSettings {
  fn default() -> CefRequestContextSettings {
    let /* mut */ raw = cef_sys::cef_request_context_settings_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_request_context_settings_t>() as u64;
    CefRequestContextSettings { raw }
  }
}
impl From<CefRequestContextSettings> for cef_sys::cef_request_context_settings_t {
  fn from(src: CefRequestContextSettings) -> cef_sys::cef_request_context_settings_t {
    src.raw
  }
}
impl From<cef_sys::cef_request_context_settings_t> for CefRequestContextSettings {
  fn from(raw: cef_sys::cef_request_context_settings_t) -> CefRequestContextSettings {
    CefRequestContextSettings { raw }
  }
}
#[allow(non_snake_case)]
impl CefRequestContextSettings {
  /// Size of this structure.
  pub fn size(&self) -> u64 { self.raw.size }
  /// Size of this structure.
  pub fn set_size(&mut self, v: u64) -> &mut Self { self.raw.size = v; self }
  /// The location where cache data for this request context will be stored on
  /// disk. If this value is non-empty then it must be an absolute path that is
  /// either equal to or a child directory of CefSettings.root_cache_path. If
  /// this value is empty then browsers will be created in "incognito mode" where
  /// in-memory caches are used for storage and no data is persisted to disk.
  /// HTML5 databases such as localStorage will only persist across sessions if a
  /// cache path is specified. To share the global browser cache and related
  /// configuration set this value to match the CefSettings.cache_path value.
  pub fn cache_path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.cache_path) }
  /// The location where cache data for this request context will be stored on
  /// disk. If this value is non-empty then it must be an absolute path that is
  /// either equal to or a child directory of CefSettings.root_cache_path. If
  /// this value is empty then browsers will be created in "incognito mode" where
  /// in-memory caches are used for storage and no data is persisted to disk.
  /// HTML5 databases such as localStorage will only persist across sessions if a
  /// cache path is specified. To share the global browser cache and related
  /// configuration set this value to match the CefSettings.cache_path value.
  pub fn set_cache_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.cache_path, v); self }
  /// To persist session cookies (cookies without an expiry date or validity
  /// interval) by default when using the global cookie manager set this value to
  /// true (1). Session cookies are generally intended to be transient and most
  /// Web browsers do not persist them. Can be set globally using the
  /// CefSettings.persist_session_cookies value. This value will be ignored if
  /// |cache_path| is empty or if it matches the CefSettings.cache_path value.
  pub fn persist_session_cookies(&self) -> i32 { self.raw.persist_session_cookies }
  /// To persist session cookies (cookies without an expiry date or validity
  /// interval) by default when using the global cookie manager set this value to
  /// true (1). Session cookies are generally intended to be transient and most
  /// Web browsers do not persist them. Can be set globally using the
  /// CefSettings.persist_session_cookies value. This value will be ignored if
  /// |cache_path| is empty or if it matches the CefSettings.cache_path value.
  pub fn set_persist_session_cookies(&mut self, v: i32) -> &mut Self { self.raw.persist_session_cookies = v; self }
  /// To persist user preferences as a JSON file in the cache path directory set
  /// this value to true (1). Can be set globally using the
  /// CefSettings.persist_user_preferences value. This value will be ignored if
  /// |cache_path| is empty or if it matches the CefSettings.cache_path value.
  pub fn persist_user_preferences(&self) -> i32 { self.raw.persist_user_preferences }
  /// To persist user preferences as a JSON file in the cache path directory set
  /// this value to true (1). Can be set globally using the
  /// CefSettings.persist_user_preferences value. This value will be ignored if
  /// |cache_path| is empty or if it matches the CefSettings.cache_path value.
  pub fn set_persist_user_preferences(&mut self, v: i32) -> &mut Self { self.raw.persist_user_preferences = v; self }
  /// Set to true (1) to ignore errors related to invalid SSL certificates.
  /// Enabling this setting can lead to potential security vulnerabilities like
  /// "man in the middle" attacks. Applications that load content from the
  /// internet should not enable this setting. Can be set globally using the
  /// CefSettings.ignore_certificate_errors value. This value will be ignored if
  /// |cache_path| matches the CefSettings.cache_path value.
  pub fn ignore_certificate_errors(&self) -> i32 { self.raw.ignore_certificate_errors }
  /// Set to true (1) to ignore errors related to invalid SSL certificates.
  /// Enabling this setting can lead to potential security vulnerabilities like
  /// "man in the middle" attacks. Applications that load content from the
  /// internet should not enable this setting. Can be set globally using the
  /// CefSettings.ignore_certificate_errors value. This value will be ignored if
  /// |cache_path| matches the CefSettings.cache_path value.
  pub fn set_ignore_certificate_errors(&mut self, v: i32) -> &mut Self { self.raw.ignore_certificate_errors = v; self }
  /// Comma delimited ordered list of language codes without any whitespace that
  /// will be used in the "Accept-Language" HTTP header. Can be set globally
  /// using the CefSettings.accept_language_list value or overridden on a per-
  /// browser basis using the CefBrowserSettings.accept_language_list value. If
  /// all values are empty then "en-US,en" will be used. This value will be
  /// ignored if |cache_path| matches the CefSettings.cache_path value.
  pub fn accept_language_list(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.accept_language_list) }
  /// Comma delimited ordered list of language codes without any whitespace that
  /// will be used in the "Accept-Language" HTTP header. Can be set globally
  /// using the CefSettings.accept_language_list value or overridden on a per-
  /// browser basis using the CefBrowserSettings.accept_language_list value. If
  /// all values are empty then "en-US,en" will be used. This value will be
  /// ignored if |cache_path| matches the CefSettings.cache_path value.
  pub fn set_accept_language_list(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.accept_language_list, v); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Browser initialization settings. Specify NULL or 0 to get the recommended
/// default values. The consequences of using custom values may not be well
/// tested. Many of these and other settings can also configured using command-
/// line switches.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefBrowserSettings { pub raw: cef_sys::cef_browser_settings_t }
impl Default for CefBrowserSettings {
  fn default() -> CefBrowserSettings {
    let /* mut */ raw = cef_sys::cef_browser_settings_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_browser_settings_t>() as u64;
    CefBrowserSettings { raw }
  }
}
impl From<CefBrowserSettings> for cef_sys::cef_browser_settings_t {
  fn from(src: CefBrowserSettings) -> cef_sys::cef_browser_settings_t {
    src.raw
  }
}
impl From<cef_sys::cef_browser_settings_t> for CefBrowserSettings {
  fn from(raw: cef_sys::cef_browser_settings_t) -> CefBrowserSettings {
    CefBrowserSettings { raw }
  }
}
#[allow(non_snake_case)]
impl CefBrowserSettings {
  /// Size of this structure.
  pub fn size(&self) -> u64 { self.raw.size }
  /// Size of this structure.
  pub fn set_size(&mut self, v: u64) -> &mut Self { self.raw.size = v; self }
  /// The maximum rate in frames per second (fps) that CefRenderHandler::OnPaint
  /// will be called for a windowless browser. The actual fps may be lower if
  /// the browser cannot generate frames at the requested rate. The minimum
  /// value is 1 and the maximum value is 60 (default 30). This value can also be
  /// changed dynamically via CefBrowserHost::SetWindowlessFrameRate.
  pub fn windowless_frame_rate(&self) -> i32 { self.raw.windowless_frame_rate }
  /// The maximum rate in frames per second (fps) that CefRenderHandler::OnPaint
  /// will be called for a windowless browser. The actual fps may be lower if
  /// the browser cannot generate frames at the requested rate. The minimum
  /// value is 1 and the maximum value is 60 (default 30). This value can also be
  /// changed dynamically via CefBrowserHost::SetWindowlessFrameRate.
  pub fn set_windowless_frame_rate(&mut self, v: i32) -> &mut Self { self.raw.windowless_frame_rate = v; self }
  /// Font settings.
  pub fn standard_font_family(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.standard_font_family) }
  /// Font settings.
  pub fn set_standard_font_family(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.standard_font_family, v); self }
  pub fn fixed_font_family(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.fixed_font_family) }
  pub fn set_fixed_font_family(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.fixed_font_family, v); self }
  pub fn serif_font_family(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.serif_font_family) }
  pub fn set_serif_font_family(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.serif_font_family, v); self }
  pub fn sans_serif_font_family(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.sans_serif_font_family) }
  pub fn set_sans_serif_font_family(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.sans_serif_font_family, v); self }
  pub fn cursive_font_family(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.cursive_font_family) }
  pub fn set_cursive_font_family(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.cursive_font_family, v); self }
  pub fn fantasy_font_family(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.fantasy_font_family) }
  pub fn set_fantasy_font_family(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.fantasy_font_family, v); self }
  pub fn default_font_size(&self) -> i32 { self.raw.default_font_size }
  pub fn set_default_font_size(&mut self, v: i32) -> &mut Self { self.raw.default_font_size = v; self }
  pub fn default_fixed_font_size(&self) -> i32 { self.raw.default_fixed_font_size }
  pub fn set_default_fixed_font_size(&mut self, v: i32) -> &mut Self { self.raw.default_fixed_font_size = v; self }
  pub fn minimum_font_size(&self) -> i32 { self.raw.minimum_font_size }
  pub fn set_minimum_font_size(&mut self, v: i32) -> &mut Self { self.raw.minimum_font_size = v; self }
  pub fn minimum_logical_font_size(&self) -> i32 { self.raw.minimum_logical_font_size }
  pub fn set_minimum_logical_font_size(&mut self, v: i32) -> &mut Self { self.raw.minimum_logical_font_size = v; self }
  /// Default encoding for Web content. If empty "ISO-8859-1" will be used. Also
  /// configurable using the "default-encoding" command-line switch.
  pub fn default_encoding(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.default_encoding) }
  /// Default encoding for Web content. If empty "ISO-8859-1" will be used. Also
  /// configurable using the "default-encoding" command-line switch.
  pub fn set_default_encoding(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.default_encoding, v); self }
  /// Controls the loading of fonts from remote sources. Also configurable using
  /// the "disable-remote-fonts" command-line switch.
  pub fn remote_fonts(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls the loading of fonts from remote sources. Also configurable using
  /// the "disable-remote-fonts" command-line switch.
  pub fn set_remote_fonts(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.remote_fonts = v.into(); self }
  /// Controls whether JavaScript can be executed. Also configurable using the
  /// "disable-javascript" command-line switch.
  pub fn javascript(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether JavaScript can be executed. Also configurable using the
  /// "disable-javascript" command-line switch.
  pub fn set_javascript(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.javascript = v.into(); self }
  /// Controls whether JavaScript can be used to close windows that were not
  /// opened via JavaScript. JavaScript can still be used to close windows that
  /// were opened via JavaScript or that have no back/forward history. Also
  /// configurable using the "disable-javascript-close-windows" command-line
  /// switch.
  pub fn javascript_close_windows(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether JavaScript can be used to close windows that were not
  /// opened via JavaScript. JavaScript can still be used to close windows that
  /// were opened via JavaScript or that have no back/forward history. Also
  /// configurable using the "disable-javascript-close-windows" command-line
  /// switch.
  pub fn set_javascript_close_windows(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.javascript_close_windows = v.into(); self }
  /// Controls whether JavaScript can access the clipboard. Also configurable
  /// using the "disable-javascript-access-clipboard" command-line switch.
  pub fn javascript_access_clipboard(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether JavaScript can access the clipboard. Also configurable
  /// using the "disable-javascript-access-clipboard" command-line switch.
  pub fn set_javascript_access_clipboard(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.javascript_access_clipboard = v.into(); self }
  /// Controls whether DOM pasting is supported in the editor via
  /// execCommand("paste"). The |javascript_access_clipboard| setting must also
  /// be enabled. Also configurable using the "disable-javascript-dom-paste"
  /// command-line switch.
  pub fn javascript_dom_paste(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether DOM pasting is supported in the editor via
  /// execCommand("paste"). The |javascript_access_clipboard| setting must also
  /// be enabled. Also configurable using the "disable-javascript-dom-paste"
  /// command-line switch.
  pub fn set_javascript_dom_paste(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.javascript_dom_paste = v.into(); self }
  /// Controls whether any plugins will be loaded. Also configurable using the
  /// "disable-plugins" command-line switch.
  pub fn plugins(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether any plugins will be loaded. Also configurable using the
  /// "disable-plugins" command-line switch.
  pub fn set_plugins(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.plugins = v.into(); self }
  /// Controls whether file URLs will have access to all URLs. Also configurable
  /// using the "allow-universal-access-from-files" command-line switch.
  pub fn universal_access_from_file_urls(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether file URLs will have access to all URLs. Also configurable
  /// using the "allow-universal-access-from-files" command-line switch.
  pub fn set_universal_access_from_file_urls(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.universal_access_from_file_urls = v.into(); self }
  /// Controls whether file URLs will have access to other file URLs. Also
  /// configurable using the "allow-access-from-files" command-line switch.
  pub fn file_access_from_file_urls(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether file URLs will have access to other file URLs. Also
  /// configurable using the "allow-access-from-files" command-line switch.
  pub fn set_file_access_from_file_urls(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.file_access_from_file_urls = v.into(); self }
  /// Controls whether web security restrictions (same-origin policy) will be
  /// enforced. Disabling this setting is not recommend as it will allow risky
  /// security behavior such as cross-site scripting (XSS). Also configurable
  /// using the "disable-web-security" command-line switch.
  pub fn web_security(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether web security restrictions (same-origin policy) will be
  /// enforced. Disabling this setting is not recommend as it will allow risky
  /// security behavior such as cross-site scripting (XSS). Also configurable
  /// using the "disable-web-security" command-line switch.
  pub fn set_web_security(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.web_security = v.into(); self }
  /// Controls whether image URLs will be loaded from the network. A cached image
  /// will still be rendered if requested. Also configurable using the
  /// "disable-image-loading" command-line switch.
  pub fn image_loading(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether image URLs will be loaded from the network. A cached image
  /// will still be rendered if requested. Also configurable using the
  /// "disable-image-loading" command-line switch.
  pub fn set_image_loading(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.image_loading = v.into(); self }
  /// Controls whether standalone images will be shrunk to fit the page. Also
  /// configurable using the "image-shrink-standalone-to-fit" command-line
  /// switch.
  pub fn image_shrink_standalone_to_fit(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether standalone images will be shrunk to fit the page. Also
  /// configurable using the "image-shrink-standalone-to-fit" command-line
  /// switch.
  pub fn set_image_shrink_standalone_to_fit(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.image_shrink_standalone_to_fit = v.into(); self }
  /// Controls whether text areas can be resized. Also configurable using the
  /// "disable-text-area-resize" command-line switch.
  pub fn text_area_resize(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether text areas can be resized. Also configurable using the
  /// "disable-text-area-resize" command-line switch.
  pub fn set_text_area_resize(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.text_area_resize = v.into(); self }
  /// Controls whether the tab key can advance focus to links. Also configurable
  /// using the "disable-tab-to-links" command-line switch.
  pub fn tab_to_links(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether the tab key can advance focus to links. Also configurable
  /// using the "disable-tab-to-links" command-line switch.
  pub fn set_tab_to_links(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.tab_to_links = v.into(); self }
  /// Controls whether local storage can be used. Also configurable using the
  /// "disable-local-storage" command-line switch.
  pub fn local_storage(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether local storage can be used. Also configurable using the
  /// "disable-local-storage" command-line switch.
  pub fn set_local_storage(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.local_storage = v.into(); self }
  /// Controls whether databases can be used. Also configurable using the
  /// "disable-databases" command-line switch.
  pub fn databases(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether databases can be used. Also configurable using the
  /// "disable-databases" command-line switch.
  pub fn set_databases(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.databases = v.into(); self }
  /// Controls whether the application cache can be used. Also configurable using
  /// the "disable-application-cache" command-line switch.
  pub fn application_cache(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether the application cache can be used. Also configurable using
  /// the "disable-application-cache" command-line switch.
  pub fn set_application_cache(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.application_cache = v.into(); self }
  /// Controls whether WebGL can be used. Note that WebGL requires hardware
  /// support and may not work on all systems even when enabled. Also
  /// configurable using the "disable-webgl" command-line switch.
  pub fn webgl(&self) -> &crate::include::internal::CefState { unsafe { &*(self as *const _ as *const _) } }
  /// Controls whether WebGL can be used. Note that WebGL requires hardware
  /// support and may not work on all systems even when enabled. Also
  /// configurable using the "disable-webgl" command-line switch.
  pub fn set_webgl(&mut self, v: crate::include::internal::CefState) -> &mut Self { self.raw.webgl = v.into(); self }
  /// Background color used for the browser before a document is loaded and when
  /// no document color is specified. The alpha component must be either fully
  /// opaque (0xFF) or fully transparent (0x00). If the alpha component is fully
  /// opaque then the RGB components will be used as the background color. If the
  /// alpha component is fully transparent for a windowed browser then the
  /// CefSettings.background_color value will be used. If the alpha component is
  /// fully transparent for a windowless (off-screen) browser then transparent
  /// painting will be enabled.
  pub fn background_color(&self) -> &crate::include::internal::CefColor { unsafe { &*(self as *const _ as *const _) } }
  /// Background color used for the browser before a document is loaded and when
  /// no document color is specified. The alpha component must be either fully
  /// opaque (0xFF) or fully transparent (0x00). If the alpha component is fully
  /// opaque then the RGB components will be used as the background color. If the
  /// alpha component is fully transparent for a windowed browser then the
  /// CefSettings.background_color value will be used. If the alpha component is
  /// fully transparent for a windowless (off-screen) browser then transparent
  /// painting will be enabled.
  pub fn set_background_color(&mut self, v: crate::include::internal::CefColor) -> &mut Self { self.raw.background_color = v.into(); self }
  /// Comma delimited ordered list of language codes without any whitespace that
  /// will be used in the "Accept-Language" HTTP header. May be set globally
  /// using the CefBrowserSettings.accept_language_list value. If both values are
  /// empty then "en-US,en" will be used.
  pub fn accept_language_list(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.accept_language_list) }
  /// Comma delimited ordered list of language codes without any whitespace that
  /// will be used in the "Accept-Language" HTTP header. May be set globally
  /// using the CefBrowserSettings.accept_language_list value. If both values are
  /// empty then "en-US,en" will be used.
  pub fn set_accept_language_list(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.accept_language_list, v); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Return value types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefReturnValue(cef_sys::cef_return_value_t);
impl CefReturnValue {
  /// Cancel immediately.
  pub const CANCEL: CefReturnValue = CefReturnValue(cef_sys::cef_return_value_t_RV_CANCEL);
  /// Continue immediately.
  pub const CONTINUE: CefReturnValue = CefReturnValue(cef_sys::cef_return_value_t_RV_CONTINUE);
  /// Continue asynchronously (usually via a callback).
  pub const CONTINUE_ASYNC: CefReturnValue = CefReturnValue(cef_sys::cef_return_value_t_RV_CONTINUE_ASYNC);
}
impl From<cef_sys::cef_return_value_t> for CefReturnValue {
  fn from(raw: cef_sys::cef_return_value_t) -> CefReturnValue {
    CefReturnValue(raw)
  }
}
impl From<CefReturnValue> for cef_sys::cef_return_value_t {
  fn from(src: CefReturnValue) -> cef_sys::cef_return_value_t {
    src.0
  }
}
impl std::ops::BitOr for CefReturnValue {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// URL component parts.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefUrlparts { pub raw: cef_sys::cef_urlparts_t }
impl Default for CefUrlparts {
  fn default() -> CefUrlparts {
    let /* mut */ raw = cef_sys::cef_urlparts_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_urlparts_t>() as u64;
    CefUrlparts { raw }
  }
}
impl From<CefUrlparts> for cef_sys::cef_urlparts_t {
  fn from(src: CefUrlparts) -> cef_sys::cef_urlparts_t {
    src.raw
  }
}
impl From<cef_sys::cef_urlparts_t> for CefUrlparts {
  fn from(raw: cef_sys::cef_urlparts_t) -> CefUrlparts {
    CefUrlparts { raw }
  }
}
#[allow(non_snake_case)]
impl CefUrlparts {
  /// The complete URL specification.
  pub fn spec(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.spec) }
  /// The complete URL specification.
  pub fn set_spec(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.spec, v); self }
  /// Scheme component not including the colon (e.g., "http").
  pub fn scheme(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.scheme) }
  /// Scheme component not including the colon (e.g., "http").
  pub fn set_scheme(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.scheme, v); self }
  /// User name component.
  pub fn username(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.username) }
  /// User name component.
  pub fn set_username(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.username, v); self }
  /// Password component.
  pub fn password(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.password) }
  /// Password component.
  pub fn set_password(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.password, v); self }
  /// Host component. This may be a hostname, an IPv4 address or an IPv6 literal
  /// surrounded by square brackets (e.g., "[2001:db8::1]").
  pub fn host(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.host) }
  /// Host component. This may be a hostname, an IPv4 address or an IPv6 literal
  /// surrounded by square brackets (e.g., "[2001:db8::1]").
  pub fn set_host(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.host, v); self }
  /// Port number component.
  pub fn port(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.port) }
  /// Port number component.
  pub fn set_port(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.port, v); self }
  /// Origin contains just the scheme, host, and port from a URL. Equivalent to
  /// clearing any username and password, replacing the path with a slash, and
  /// clearing everything after that. This value will be empty for non-standard
  /// URLs.
  pub fn origin(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.origin) }
  /// Origin contains just the scheme, host, and port from a URL. Equivalent to
  /// clearing any username and password, replacing the path with a slash, and
  /// clearing everything after that. This value will be empty for non-standard
  /// URLs.
  pub fn set_origin(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.origin, v); self }
  /// Path component including the first slash following the host.
  pub fn path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.path) }
  /// Path component including the first slash following the host.
  pub fn set_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.path, v); self }
  /// Query string component (i.e., everything following the '?').
  pub fn query(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.query) }
  /// Query string component (i.e., everything following the '?').
  pub fn set_query(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.query, v); self }
  /// Fragment (hash) identifier component (i.e., the string following the '#').
  pub fn fragment(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.fragment) }
  /// Fragment (hash) identifier component (i.e., the string following the '#').
  pub fn set_fragment(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.fragment, v); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Cookie priority values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefCookiePriority(cef_sys::cef_cookie_priority_t);
impl CefCookiePriority {
  pub const LOW: CefCookiePriority = CefCookiePriority(cef_sys::cef_cookie_priority_t_CEF_COOKIE_PRIORITY_LOW);
  pub const MEDIUM: CefCookiePriority = CefCookiePriority(cef_sys::cef_cookie_priority_t_CEF_COOKIE_PRIORITY_MEDIUM);
  pub const HIGH: CefCookiePriority = CefCookiePriority(cef_sys::cef_cookie_priority_t_CEF_COOKIE_PRIORITY_HIGH);
}
impl From<cef_sys::cef_cookie_priority_t> for CefCookiePriority {
  fn from(raw: cef_sys::cef_cookie_priority_t) -> CefCookiePriority {
    CefCookiePriority(raw)
  }
}
impl From<CefCookiePriority> for cef_sys::cef_cookie_priority_t {
  fn from(src: CefCookiePriority) -> cef_sys::cef_cookie_priority_t {
    src.0
  }
}
impl std::ops::BitOr for CefCookiePriority {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Cookie same site values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefCookieSameSite(cef_sys::cef_cookie_same_site_t);
impl CefCookieSameSite {
  pub const UNSPECIFIED: CefCookieSameSite = CefCookieSameSite(cef_sys::cef_cookie_same_site_t_CEF_COOKIE_SAME_SITE_UNSPECIFIED);
  pub const NO_RESTRICTION: CefCookieSameSite = CefCookieSameSite(cef_sys::cef_cookie_same_site_t_CEF_COOKIE_SAME_SITE_NO_RESTRICTION);
  pub const LAX_MODE: CefCookieSameSite = CefCookieSameSite(cef_sys::cef_cookie_same_site_t_CEF_COOKIE_SAME_SITE_LAX_MODE);
  pub const STRICT_MODE: CefCookieSameSite = CefCookieSameSite(cef_sys::cef_cookie_same_site_t_CEF_COOKIE_SAME_SITE_STRICT_MODE);
}
impl From<cef_sys::cef_cookie_same_site_t> for CefCookieSameSite {
  fn from(raw: cef_sys::cef_cookie_same_site_t) -> CefCookieSameSite {
    CefCookieSameSite(raw)
  }
}
impl From<CefCookieSameSite> for cef_sys::cef_cookie_same_site_t {
  fn from(src: CefCookieSameSite) -> cef_sys::cef_cookie_same_site_t {
    src.0
  }
}
impl std::ops::BitOr for CefCookieSameSite {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Cookie information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefCookie { pub raw: cef_sys::cef_cookie_t }
impl Default for CefCookie {
  fn default() -> CefCookie {
    let /* mut */ raw = cef_sys::cef_cookie_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_cookie_t>() as u64;
    CefCookie { raw }
  }
}
impl From<CefCookie> for cef_sys::cef_cookie_t {
  fn from(src: CefCookie) -> cef_sys::cef_cookie_t {
    src.raw
  }
}
impl From<cef_sys::cef_cookie_t> for CefCookie {
  fn from(raw: cef_sys::cef_cookie_t) -> CefCookie {
    CefCookie { raw }
  }
}
#[allow(non_snake_case)]
impl CefCookie {
  /// The cookie name.
  pub fn name(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.name) }
  /// The cookie name.
  pub fn set_name(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.name, v); self }
  /// The cookie value.
  pub fn value(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.value) }
  /// The cookie value.
  pub fn set_value(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.value, v); self }
  /// If |domain| is empty a host cookie will be created instead of a domain
  /// cookie. Domain cookies are stored with a leading "." and are visible to
  /// sub-domains whereas host cookies are not.
  pub fn domain(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.domain) }
  /// If |domain| is empty a host cookie will be created instead of a domain
  /// cookie. Domain cookies are stored with a leading "." and are visible to
  /// sub-domains whereas host cookies are not.
  pub fn set_domain(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.domain, v); self }
  /// If |path| is non-empty only URLs at or below the path will get the cookie
  /// value.
  pub fn path(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.path) }
  /// If |path| is non-empty only URLs at or below the path will get the cookie
  /// value.
  pub fn set_path(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.path, v); self }
  /// If |secure| is true the cookie will only be sent for HTTPS requests.
  pub fn secure(&self) -> i32 { self.raw.secure }
  /// If |secure| is true the cookie will only be sent for HTTPS requests.
  pub fn set_secure(&mut self, v: i32) -> &mut Self { self.raw.secure = v; self }
  /// If |httponly| is true the cookie will only be sent for HTTP requests.
  pub fn httponly(&self) -> i32 { self.raw.httponly }
  /// If |httponly| is true the cookie will only be sent for HTTP requests.
  pub fn set_httponly(&mut self, v: i32) -> &mut Self { self.raw.httponly = v; self }
  /// The cookie creation date. This is automatically populated by the system on
  /// cookie creation.
  pub fn creation(&self) -> &crate::include::internal::CefTime { unsafe { &*(self as *const _ as *const _) } }
  /// The cookie creation date. This is automatically populated by the system on
  /// cookie creation.
  pub fn set_creation(&mut self, v: crate::include::internal::CefTime) -> &mut Self { self.raw.creation = v.into(); self }
  /// The cookie last access date. This is automatically populated by the system
  /// on access.
  pub fn last_access(&self) -> &crate::include::internal::CefTime { unsafe { &*(self as *const _ as *const _) } }
  /// The cookie last access date. This is automatically populated by the system
  /// on access.
  pub fn set_last_access(&mut self, v: crate::include::internal::CefTime) -> &mut Self { self.raw.last_access = v.into(); self }
  /// The cookie expiration date is only valid if |has_expires| is true.
  pub fn has_expires(&self) -> i32 { self.raw.has_expires }
  /// The cookie expiration date is only valid if |has_expires| is true.
  pub fn set_has_expires(&mut self, v: i32) -> &mut Self { self.raw.has_expires = v; self }
  pub fn expires(&self) -> &crate::include::internal::CefTime { unsafe { &*(self as *const _ as *const _) } }
  pub fn set_expires(&mut self, v: crate::include::internal::CefTime) -> &mut Self { self.raw.expires = v.into(); self }
  /// Same site.
  pub fn same_site(&self) -> &crate::include::internal::CefCookieSameSite { unsafe { &*(self as *const _ as *const _) } }
  /// Same site.
  pub fn set_same_site(&mut self, v: crate::include::internal::CefCookieSameSite) -> &mut Self { self.raw.same_site = v.into(); self }
  /// Priority.
  pub fn priority(&self) -> &crate::include::internal::CefCookiePriority { unsafe { &*(self as *const _ as *const _) } }
  /// Priority.
  pub fn set_priority(&mut self, v: crate::include::internal::CefCookiePriority) -> &mut Self { self.raw.priority = v.into(); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Process termination status values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefTerminationStatus(cef_sys::cef_termination_status_t);
impl CefTerminationStatus {
  /// Non-zero exit status.
  pub const ABNORMAL_TERMINATION: CefTerminationStatus = CefTerminationStatus(cef_sys::cef_termination_status_t_TS_ABNORMAL_TERMINATION);
  /// SIGKILL or task manager kill.
  pub const PROCESS_WAS_KILLED: CefTerminationStatus = CefTerminationStatus(cef_sys::cef_termination_status_t_TS_PROCESS_WAS_KILLED);
  /// Segmentation fault.
  pub const PROCESS_CRASHED: CefTerminationStatus = CefTerminationStatus(cef_sys::cef_termination_status_t_TS_PROCESS_CRASHED);
  /// Out of memory. Some platforms may use TS_PROCESS_CRASHED instead.
  pub const PROCESS_OOM: CefTerminationStatus = CefTerminationStatus(cef_sys::cef_termination_status_t_TS_PROCESS_OOM);
}
impl From<cef_sys::cef_termination_status_t> for CefTerminationStatus {
  fn from(raw: cef_sys::cef_termination_status_t) -> CefTerminationStatus {
    CefTerminationStatus(raw)
  }
}
impl From<CefTerminationStatus> for cef_sys::cef_termination_status_t {
  fn from(src: CefTerminationStatus) -> cef_sys::cef_termination_status_t {
    src.0
  }
}
impl std::ops::BitOr for CefTerminationStatus {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Path key values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefPathKey(cef_sys::cef_path_key_t);
impl CefPathKey {
  /// Current directory.
  pub const DIR_CURRENT: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_DIR_CURRENT);
  /// Directory containing PK_FILE_EXE.
  pub const DIR_EXE: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_DIR_EXE);
  /// Directory containing PK_FILE_MODULE.
  pub const DIR_MODULE: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_DIR_MODULE);
  /// Temporary directory.
  pub const DIR_TEMP: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_DIR_TEMP);
  /// Path and filename of the current executable.
  pub const FILE_EXE: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_FILE_EXE);
  /// Path and filename of the module containing the CEF code (usually the libcef
  /// module).
  pub const FILE_MODULE: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_FILE_MODULE);
  /// "Local Settings\Application Data" directory under the user profile
  /// directory on Windows.
  pub const LOCAL_APP_DATA: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_LOCAL_APP_DATA);
  /// "Application Data" directory under the user profile directory on Windows
  /// and "~/Library/Application Support" directory on Mac OS X.
  pub const USER_DATA: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_USER_DATA);
  /// Directory containing application resources. Can be configured via
  /// CefSettings.resources_dir_path.
  pub const DIR_RESOURCES: CefPathKey = CefPathKey(cef_sys::cef_path_key_t_PK_DIR_RESOURCES);
}
impl From<cef_sys::cef_path_key_t> for CefPathKey {
  fn from(raw: cef_sys::cef_path_key_t) -> CefPathKey {
    CefPathKey(raw)
  }
}
impl From<CefPathKey> for cef_sys::cef_path_key_t {
  fn from(src: CefPathKey) -> cef_sys::cef_path_key_t {
    src.0
  }
}
impl std::ops::BitOr for CefPathKey {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Storage types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefStorageType(cef_sys::cef_storage_type_t);
impl CefStorageType {
  pub const LOCALSTORAGE: CefStorageType = CefStorageType(cef_sys::cef_storage_type_t_ST_LOCALSTORAGE);
  pub const SESSIONSTORAGE: CefStorageType = CefStorageType(cef_sys::cef_storage_type_t_ST_SESSIONSTORAGE);
}
impl From<cef_sys::cef_storage_type_t> for CefStorageType {
  fn from(raw: cef_sys::cef_storage_type_t) -> CefStorageType {
    CefStorageType(raw)
  }
}
impl From<CefStorageType> for cef_sys::cef_storage_type_t {
  fn from(src: CefStorageType) -> cef_sys::cef_storage_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefStorageType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported error code values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefErrorcode(cef_sys::cef_errorcode_t);
impl CefErrorcode {
  /// No error.
  pub const NONE: CefErrorcode = CefErrorcode(cef_sys::cef_errorcode_t_ERR_NONE);
}
impl From<cef_sys::cef_errorcode_t> for CefErrorcode {
  fn from(raw: cef_sys::cef_errorcode_t) -> CefErrorcode {
    CefErrorcode(raw)
  }
}
impl From<CefErrorcode> for cef_sys::cef_errorcode_t {
  fn from(src: CefErrorcode) -> cef_sys::cef_errorcode_t {
    src.0
  }
}
impl std::ops::BitOr for CefErrorcode {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported certificate status code values. See net\cert\cert_status_flags.h
/// for more information. CERT_STATUS_NONE is new in CEF because we use an
/// enum while cert_status_flags.h uses a typedef and static const variables.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefCertStatus(cef_sys::cef_cert_status_t);
impl CefCertStatus {
  pub const NONE: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_NONE);
  pub const COMMON_NAME_INVALID: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_COMMON_NAME_INVALID);
  pub const DATE_INVALID: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_DATE_INVALID);
  pub const AUTHORITY_INVALID: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_AUTHORITY_INVALID);
  /// 1 << 3 is reserved for ERR_CERT_CONTAINS_ERRORS (not useful with WinHTTP).
  pub const NO_REVOCATION_MECHANISM: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_NO_REVOCATION_MECHANISM);
  pub const UNABLE_TO_CHECK_REVOCATION: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_UNABLE_TO_CHECK_REVOCATION);
  pub const REVOKED: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_REVOKED);
  pub const INVALID: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_INVALID);
  pub const WEAK_SIGNATURE_ALGORITHM: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_WEAK_SIGNATURE_ALGORITHM);
  /// 1 << 9 was used for CERT_STATUS_NOT_IN_DNS
  pub const NON_UNIQUE_NAME: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_NON_UNIQUE_NAME);
  pub const WEAK_KEY: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_WEAK_KEY);
  /// 1 << 12 was used for CERT_STATUS_WEAK_DH_KEY
  pub const PINNED_KEY_MISSING: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_PINNED_KEY_MISSING);
  pub const NAME_CONSTRAINT_VIOLATION: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_NAME_CONSTRAINT_VIOLATION);
  pub const VALIDITY_TOO_LONG: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_VALIDITY_TOO_LONG);
  /// Bits 16 to 31 are for non-error statuses.
  pub const IS_EV: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_IS_EV);
  pub const REV_CHECKING_ENABLED: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_REV_CHECKING_ENABLED);
  /// Bit 18 was CERT_STATUS_IS_DNSSEC
  pub const SHA1_SIGNATURE_PRESENT: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_SHA1_SIGNATURE_PRESENT);
  pub const CT_COMPLIANCE_FAILED: CefCertStatus = CefCertStatus(cef_sys::cef_cert_status_t_CERT_STATUS_CT_COMPLIANCE_FAILED);
}
impl From<cef_sys::cef_cert_status_t> for CefCertStatus {
  fn from(raw: cef_sys::cef_cert_status_t) -> CefCertStatus {
    CefCertStatus(raw)
  }
}
impl From<CefCertStatus> for cef_sys::cef_cert_status_t {
  fn from(src: CefCertStatus) -> cef_sys::cef_cert_status_t {
    src.0
  }
}
impl std::ops::BitOr for CefCertStatus {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// The manner in which a link click should be opened. These constants match
/// their equivalents in Chromium's window_open_disposition.h and should not be
/// renumbered.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t);
impl CefWindowOpenDisposition {
  pub const UNKNOWN: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_UNKNOWN);
  pub const CURRENT_TAB: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_CURRENT_TAB);
  pub const SINGLETON_TAB: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_SINGLETON_TAB);
  pub const NEW_FOREGROUND_TAB: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_NEW_FOREGROUND_TAB);
  pub const NEW_BACKGROUND_TAB: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_NEW_BACKGROUND_TAB);
  pub const NEW_POPUP: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_NEW_POPUP);
  pub const NEW_WINDOW: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_NEW_WINDOW);
  pub const SAVE_TO_DISK: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_SAVE_TO_DISK);
  pub const OFF_THE_RECORD: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_OFF_THE_RECORD);
  pub const IGNORE_ACTION: CefWindowOpenDisposition = CefWindowOpenDisposition(cef_sys::cef_window_open_disposition_t_WOD_IGNORE_ACTION);
}
impl From<cef_sys::cef_window_open_disposition_t> for CefWindowOpenDisposition {
  fn from(raw: cef_sys::cef_window_open_disposition_t) -> CefWindowOpenDisposition {
    CefWindowOpenDisposition(raw)
  }
}
impl From<CefWindowOpenDisposition> for cef_sys::cef_window_open_disposition_t {
  fn from(src: CefWindowOpenDisposition) -> cef_sys::cef_window_open_disposition_t {
    src.0
  }
}
impl std::ops::BitOr for CefWindowOpenDisposition {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// "Verb" of a drag-and-drop operation as negotiated between the source and
/// destination. These constants match their equivalents in WebCore's
/// DragActions.h and should not be renumbered.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t);
impl CefDragOperationsMask {
  pub const NONE: CefDragOperationsMask = CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t_DRAG_OPERATION_NONE);
  pub const COPY: CefDragOperationsMask = CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t_DRAG_OPERATION_COPY);
  pub const LINK: CefDragOperationsMask = CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t_DRAG_OPERATION_LINK);
  pub const GENERIC: CefDragOperationsMask = CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t_DRAG_OPERATION_GENERIC);
  pub const PRIVATE: CefDragOperationsMask = CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t_DRAG_OPERATION_PRIVATE);
  pub const MOVE: CefDragOperationsMask = CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t_DRAG_OPERATION_MOVE);
  pub const DELETE: CefDragOperationsMask = CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t_DRAG_OPERATION_DELETE);
  pub const EVERY: CefDragOperationsMask = CefDragOperationsMask(cef_sys::cef_drag_operations_mask_t_DRAG_OPERATION_EVERY);
}
impl From<cef_sys::cef_drag_operations_mask_t> for CefDragOperationsMask {
  fn from(raw: cef_sys::cef_drag_operations_mask_t) -> CefDragOperationsMask {
    CefDragOperationsMask(raw)
  }
}
impl From<CefDragOperationsMask> for cef_sys::cef_drag_operations_mask_t {
  fn from(src: CefDragOperationsMask) -> cef_sys::cef_drag_operations_mask_t {
    src.0
  }
}
impl std::ops::BitOr for CefDragOperationsMask {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Input mode of a virtual keyboard. These constants match their equivalents
/// in Chromium's text_input_mode.h and should not be renumbered.
/// See https://html.spec.whatwg.org/#input-modalities:-the-inputmode-attribute
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefTextInputMode(cef_sys::cef_text_input_mode_t);
impl CefTextInputMode {
  pub const DEFAULT: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_DEFAULT);
  pub const NONE: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_NONE);
  pub const TEXT: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_TEXT);
  pub const TEL: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_TEL);
  pub const URL: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_URL);
  pub const EMAIL: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_EMAIL);
  pub const NUMERIC: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_NUMERIC);
  pub const DECIMAL: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_DECIMAL);
  pub const SEARCH: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_SEARCH);
  pub const MAX: CefTextInputMode = CefTextInputMode(cef_sys::cef_text_input_mode_t_CEF_TEXT_INPUT_MODE_MAX);
}
impl From<cef_sys::cef_text_input_mode_t> for CefTextInputMode {
  fn from(raw: cef_sys::cef_text_input_mode_t) -> CefTextInputMode {
    CefTextInputMode(raw)
  }
}
impl From<CefTextInputMode> for cef_sys::cef_text_input_mode_t {
  fn from(src: CefTextInputMode) -> cef_sys::cef_text_input_mode_t {
    src.0
  }
}
impl std::ops::BitOr for CefTextInputMode {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// V8 access control values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefV8Accesscontrol(cef_sys::cef_v8_accesscontrol_t);
impl CefV8Accesscontrol {
  pub const DEFAULT: CefV8Accesscontrol = CefV8Accesscontrol(cef_sys::cef_v8_accesscontrol_t_V8_ACCESS_CONTROL_DEFAULT);
  pub const ALL_CAN_READ: CefV8Accesscontrol = CefV8Accesscontrol(cef_sys::cef_v8_accesscontrol_t_V8_ACCESS_CONTROL_ALL_CAN_READ);
  pub const ALL_CAN_WRITE: CefV8Accesscontrol = CefV8Accesscontrol(cef_sys::cef_v8_accesscontrol_t_V8_ACCESS_CONTROL_ALL_CAN_WRITE);
  pub const PROHIBITS_OVERWRITING: CefV8Accesscontrol = CefV8Accesscontrol(cef_sys::cef_v8_accesscontrol_t_V8_ACCESS_CONTROL_PROHIBITS_OVERWRITING);
}
impl From<cef_sys::cef_v8_accesscontrol_t> for CefV8Accesscontrol {
  fn from(raw: cef_sys::cef_v8_accesscontrol_t) -> CefV8Accesscontrol {
    CefV8Accesscontrol(raw)
  }
}
impl From<CefV8Accesscontrol> for cef_sys::cef_v8_accesscontrol_t {
  fn from(src: CefV8Accesscontrol) -> cef_sys::cef_v8_accesscontrol_t {
    src.0
  }
}
impl std::ops::BitOr for CefV8Accesscontrol {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// V8 property attribute values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefV8Propertyattribute(cef_sys::cef_v8_propertyattribute_t);
impl CefV8Propertyattribute {
  pub const NONE: CefV8Propertyattribute = CefV8Propertyattribute(cef_sys::cef_v8_propertyattribute_t_V8_PROPERTY_ATTRIBUTE_NONE);
  /// Writeable, Enumerable,
  /// Configurable
  pub const READONLY: CefV8Propertyattribute = CefV8Propertyattribute(cef_sys::cef_v8_propertyattribute_t_V8_PROPERTY_ATTRIBUTE_READONLY);
  /// Not writeable
  pub const DONTENUM: CefV8Propertyattribute = CefV8Propertyattribute(cef_sys::cef_v8_propertyattribute_t_V8_PROPERTY_ATTRIBUTE_DONTENUM);
  /// Not enumerable
  pub const DONTDELETE: CefV8Propertyattribute = CefV8Propertyattribute(cef_sys::cef_v8_propertyattribute_t_V8_PROPERTY_ATTRIBUTE_DONTDELETE);
}
impl From<cef_sys::cef_v8_propertyattribute_t> for CefV8Propertyattribute {
  fn from(raw: cef_sys::cef_v8_propertyattribute_t) -> CefV8Propertyattribute {
    CefV8Propertyattribute(raw)
  }
}
impl From<CefV8Propertyattribute> for cef_sys::cef_v8_propertyattribute_t {
  fn from(src: CefV8Propertyattribute) -> cef_sys::cef_v8_propertyattribute_t {
    src.0
  }
}
impl std::ops::BitOr for CefV8Propertyattribute {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Post data elements may represent either bytes or files.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefPostdataelementType(cef_sys::cef_postdataelement_type_t);
impl CefPostdataelementType {
  pub const EMPTY: CefPostdataelementType = CefPostdataelementType(cef_sys::cef_postdataelement_type_t_PDE_TYPE_EMPTY);
  pub const BYTES: CefPostdataelementType = CefPostdataelementType(cef_sys::cef_postdataelement_type_t_PDE_TYPE_BYTES);
  pub const FILE: CefPostdataelementType = CefPostdataelementType(cef_sys::cef_postdataelement_type_t_PDE_TYPE_FILE);
}
impl From<cef_sys::cef_postdataelement_type_t> for CefPostdataelementType {
  fn from(raw: cef_sys::cef_postdataelement_type_t) -> CefPostdataelementType {
    CefPostdataelementType(raw)
  }
}
impl From<CefPostdataelementType> for cef_sys::cef_postdataelement_type_t {
  fn from(src: CefPostdataelementType) -> cef_sys::cef_postdataelement_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefPostdataelementType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Resource type for a request.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefResourceType(cef_sys::cef_resource_type_t);
impl CefResourceType {
  /// Top level page.
  pub const MAIN_FRAME: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_MAIN_FRAME);
  /// Frame or iframe.
  pub const SUB_FRAME: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_SUB_FRAME);
  /// CSS stylesheet.
  pub const STYLESHEET: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_STYLESHEET);
  /// External script.
  pub const SCRIPT: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_SCRIPT);
  /// Image (jpg/gif/png/etc).
  pub const IMAGE: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_IMAGE);
  /// Font.
  pub const FONT_RESOURCE: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_FONT_RESOURCE);
  /// Some other subresource. This is the default type if the actual type is
  /// unknown.
  pub const SUB_RESOURCE: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_SUB_RESOURCE);
  /// Object (or embed) tag for a plugin, or a resource that a plugin requested.
  pub const OBJECT: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_OBJECT);
  /// Media resource.
  pub const MEDIA: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_MEDIA);
  /// Main resource of a dedicated worker.
  pub const WORKER: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_WORKER);
  /// Main resource of a shared worker.
  pub const SHARED_WORKER: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_SHARED_WORKER);
  /// Explicitly requested prefetch.
  pub const PREFETCH: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_PREFETCH);
  /// Favicon.
  pub const FAVICON: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_FAVICON);
  /// XMLHttpRequest.
  pub const XHR: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_XHR);
  /// A request for a <ping>
  pub const PING: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_PING);
  /// Main resource of a service worker.
  pub const SERVICE_WORKER: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_SERVICE_WORKER);
  /// A report of Content Security Policy violations.
  pub const CSP_REPORT: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_CSP_REPORT);
  /// A resource that a plugin requested.
  pub const PLUGIN_RESOURCE: CefResourceType = CefResourceType(cef_sys::cef_resource_type_t_RT_PLUGIN_RESOURCE);
}
impl From<cef_sys::cef_resource_type_t> for CefResourceType {
  fn from(raw: cef_sys::cef_resource_type_t) -> CefResourceType {
    CefResourceType(raw)
  }
}
impl From<CefResourceType> for cef_sys::cef_resource_type_t {
  fn from(src: CefResourceType) -> cef_sys::cef_resource_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefResourceType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Transition type for a request. Made up of one source value and 0 or more
/// qualifiers.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefTransitionType(cef_sys::cef_transition_type_t);
impl CefTransitionType {
  /// Source is a link click or the JavaScript window.open function. This is
  /// also the default value for requests like sub-resource loads that are not
  /// navigations.
  pub const LINK: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_LINK);
  /// Source is some other "explicit" navigation. This is the default value for
  /// navigations where the actual type is unknown. See also TT_DIRECT_LOAD_FLAG.
  pub const EXPLICIT: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_EXPLICIT);
  /// Source is a subframe navigation. This is any content that is automatically
  /// loaded in a non-toplevel frame. For example, if a page consists of several
  /// frames containing ads, those ad URLs will have this transition type.
  /// The user may not even realize the content in these pages is a separate
  /// frame, so may not care about the URL.
  pub const AUTO_SUBFRAME: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_AUTO_SUBFRAME);
  /// Source is a subframe navigation explicitly requested by the user that will
  /// generate new navigation entries in the back/forward list. These are
  /// probably more important than frames that were automatically loaded in
  /// the background because the user probably cares about the fact that this
  /// link was loaded.
  pub const MANUAL_SUBFRAME: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_MANUAL_SUBFRAME);
  /// Source is a form submission by the user. NOTE: In some situations
  /// submitting a form does not result in this transition type. This can happen
  /// if the form uses a script to submit the contents.
  pub const FORM_SUBMIT: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_FORM_SUBMIT);
  /// Source is a "reload" of the page via the Reload function or by re-visiting
  /// the same URL. NOTE: This is distinct from the concept of whether a
  /// particular load uses "reload semantics" (i.e. bypasses cached data).
  pub const RELOAD: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_RELOAD);
  /// General mask defining the bits used for the source values.
  pub const SOURCE_MASK: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_SOURCE_MASK);
  /// Attempted to visit a URL but was blocked.
  pub const BLOCKED_FLAG: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_BLOCKED_FLAG);
  /// Used the Forward or Back function to navigate among browsing history.
  /// Will be ORed to the transition type for the original load.
  pub const FORWARD_BACK_FLAG: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_FORWARD_BACK_FLAG);
  /// Loaded a URL directly via CreateBrowser, LoadURL or LoadRequest.
  pub const DIRECT_LOAD_FLAG: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_DIRECT_LOAD_FLAG);
  /// The beginning of a navigation chain.
  pub const CHAIN_START_FLAG: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_CHAIN_START_FLAG);
  /// The last transition in a redirect chain.
  pub const CHAIN_END_FLAG: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_CHAIN_END_FLAG);
  /// Redirects caused by JavaScript or a meta refresh tag on the page.
  pub const CLIENT_REDIRECT_FLAG: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_CLIENT_REDIRECT_FLAG);
  /// Redirects sent from the server by HTTP headers.
  pub const SERVER_REDIRECT_FLAG: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_SERVER_REDIRECT_FLAG);
  /// Used to test whether a transition involves a redirect.
  pub const IS_REDIRECT_MASK: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_IS_REDIRECT_MASK);
  /// General mask defining the bits used for the qualifiers.
  pub const QUALIFIER_MASK: CefTransitionType = CefTransitionType(cef_sys::cef_transition_type_t_TT_QUALIFIER_MASK);
}
impl From<cef_sys::cef_transition_type_t> for CefTransitionType {
  fn from(raw: cef_sys::cef_transition_type_t) -> CefTransitionType {
    CefTransitionType(raw)
  }
}
impl From<CefTransitionType> for cef_sys::cef_transition_type_t {
  fn from(src: CefTransitionType) -> cef_sys::cef_transition_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefTransitionType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Flags used to customize the behavior of CefURLRequest.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t);
impl CefUrlrequestFlags {
  /// Default behavior.
  pub const NONE: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_NONE);
  /// If set the cache will be skipped when handling the request. Setting this
  /// value is equivalent to specifying the "Cache-Control: no-cache" request
  /// header. Setting this value in combination with UR_FLAG_ONLY_FROM_CACHE will
  /// cause the request to fail.
  pub const SKIP_CACHE: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_SKIP_CACHE);
  /// If set the request will fail if it cannot be served from the cache (or some
  /// equivalent local store). Setting this value is equivalent to specifying the
  /// "Cache-Control: only-if-cached" request header. Setting this value in
  /// combination with UR_FLAG_SKIP_CACHE or UR_FLAG_DISABLE_CACHE will cause the
  /// request to fail.
  pub const ONLY_FROM_CACHE: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_ONLY_FROM_CACHE);
  /// If set the cache will not be used at all. Setting this value is equivalent
  /// to specifying the "Cache-Control: no-store" request header. Setting this
  /// value in combination with UR_FLAG_ONLY_FROM_CACHE will cause the request to
  /// fail.
  pub const DISABLE_CACHE: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_DISABLE_CACHE);
  /// If set user name, password, and cookies may be sent with the request, and
  /// cookies may be saved from the response.
  pub const ALLOW_STORED_CREDENTIALS: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_ALLOW_STORED_CREDENTIALS);
  /// If set upload progress events will be generated when a request has a body.
  pub const REPORT_UPLOAD_PROGRESS: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_REPORT_UPLOAD_PROGRESS);
  /// If set the CefURLRequestClient::OnDownloadData method will not be called.
  pub const NO_DOWNLOAD_DATA: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_NO_DOWNLOAD_DATA);
  /// If set 5XX redirect errors will be propagated to the observer instead of
  /// automatically re-tried. This currently only applies for requests
  /// originated in the browser process.
  pub const NO_RETRY_ON_5XX: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_NO_RETRY_ON_5XX);
  /// If set 3XX responses will cause the fetch to halt immediately rather than
  /// continue through the redirect.
  pub const STOP_ON_REDIRECT: CefUrlrequestFlags = CefUrlrequestFlags(cef_sys::cef_urlrequest_flags_t_UR_FLAG_STOP_ON_REDIRECT);
}
impl From<cef_sys::cef_urlrequest_flags_t> for CefUrlrequestFlags {
  fn from(raw: cef_sys::cef_urlrequest_flags_t) -> CefUrlrequestFlags {
    CefUrlrequestFlags(raw)
  }
}
impl From<CefUrlrequestFlags> for cef_sys::cef_urlrequest_flags_t {
  fn from(src: CefUrlrequestFlags) -> cef_sys::cef_urlrequest_flags_t {
    src.0
  }
}
impl std::ops::BitOr for CefUrlrequestFlags {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Flags that represent CefURLRequest status.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefUrlrequestStatus(cef_sys::cef_urlrequest_status_t);
impl CefUrlrequestStatus {
  /// Unknown status.
  pub const UNKNOWN: CefUrlrequestStatus = CefUrlrequestStatus(cef_sys::cef_urlrequest_status_t_UR_UNKNOWN);
  /// Request succeeded.
  pub const SUCCESS: CefUrlrequestStatus = CefUrlrequestStatus(cef_sys::cef_urlrequest_status_t_UR_SUCCESS);
  /// An IO request is pending, and the caller will be informed when it is
  /// completed.
  pub const IO_PENDING: CefUrlrequestStatus = CefUrlrequestStatus(cef_sys::cef_urlrequest_status_t_UR_IO_PENDING);
  /// Request was canceled programatically.
  pub const CANCELED: CefUrlrequestStatus = CefUrlrequestStatus(cef_sys::cef_urlrequest_status_t_UR_CANCELED);
  /// Request failed for some reason.
  pub const FAILED: CefUrlrequestStatus = CefUrlrequestStatus(cef_sys::cef_urlrequest_status_t_UR_FAILED);
}
impl From<cef_sys::cef_urlrequest_status_t> for CefUrlrequestStatus {
  fn from(raw: cef_sys::cef_urlrequest_status_t) -> CefUrlrequestStatus {
    CefUrlrequestStatus(raw)
  }
}
impl From<CefUrlrequestStatus> for cef_sys::cef_urlrequest_status_t {
  fn from(src: CefUrlrequestStatus) -> cef_sys::cef_urlrequest_status_t {
    src.0
  }
}
impl std::ops::BitOr for CefUrlrequestStatus {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Structure representing a point.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefPoint { pub raw: cef_sys::cef_point_t }
impl Default for CefPoint {
  fn default() -> CefPoint {
    let /* mut */ raw = cef_sys::cef_point_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_point_t>() as u64;
    CefPoint { raw }
  }
}
impl From<CefPoint> for cef_sys::cef_point_t {
  fn from(src: CefPoint) -> cef_sys::cef_point_t {
    src.raw
  }
}
impl From<cef_sys::cef_point_t> for CefPoint {
  fn from(raw: cef_sys::cef_point_t) -> CefPoint {
    CefPoint { raw }
  }
}
#[allow(non_snake_case)]
impl CefPoint {
  pub fn x(&self) -> i32 { self.raw.x }
  pub fn set_x(&mut self, v: i32) -> &mut Self { self.raw.x = v; self }
  pub fn y(&self) -> i32 { self.raw.y }
  pub fn set_y(&mut self, v: i32) -> &mut Self { self.raw.y = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Structure representing a rectangle.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefRect { pub raw: cef_sys::cef_rect_t }
impl Default for CefRect {
  fn default() -> CefRect {
    let /* mut */ raw = cef_sys::cef_rect_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_rect_t>() as u64;
    CefRect { raw }
  }
}
impl From<CefRect> for cef_sys::cef_rect_t {
  fn from(src: CefRect) -> cef_sys::cef_rect_t {
    src.raw
  }
}
impl From<cef_sys::cef_rect_t> for CefRect {
  fn from(raw: cef_sys::cef_rect_t) -> CefRect {
    CefRect { raw }
  }
}
#[allow(non_snake_case)]
impl CefRect {
  pub fn x(&self) -> i32 { self.raw.x }
  pub fn set_x(&mut self, v: i32) -> &mut Self { self.raw.x = v; self }
  pub fn y(&self) -> i32 { self.raw.y }
  pub fn set_y(&mut self, v: i32) -> &mut Self { self.raw.y = v; self }
  pub fn width(&self) -> i32 { self.raw.width }
  pub fn set_width(&mut self, v: i32) -> &mut Self { self.raw.width = v; self }
  pub fn height(&self) -> i32 { self.raw.height }
  pub fn set_height(&mut self, v: i32) -> &mut Self { self.raw.height = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Structure representing a size.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefSize { pub raw: cef_sys::cef_size_t }
impl Default for CefSize {
  fn default() -> CefSize {
    let /* mut */ raw = cef_sys::cef_size_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_size_t>() as u64;
    CefSize { raw }
  }
}
impl From<CefSize> for cef_sys::cef_size_t {
  fn from(src: CefSize) -> cef_sys::cef_size_t {
    src.raw
  }
}
impl From<cef_sys::cef_size_t> for CefSize {
  fn from(raw: cef_sys::cef_size_t) -> CefSize {
    CefSize { raw }
  }
}
#[allow(non_snake_case)]
impl CefSize {
  pub fn width(&self) -> i32 { self.raw.width }
  pub fn set_width(&mut self, v: i32) -> &mut Self { self.raw.width = v; self }
  pub fn height(&self) -> i32 { self.raw.height }
  pub fn set_height(&mut self, v: i32) -> &mut Self { self.raw.height = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Structure representing a range.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefRange { pub raw: cef_sys::cef_range_t }
impl Default for CefRange {
  fn default() -> CefRange {
    let /* mut */ raw = cef_sys::cef_range_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_range_t>() as u64;
    CefRange { raw }
  }
}
impl From<CefRange> for cef_sys::cef_range_t {
  fn from(src: CefRange) -> cef_sys::cef_range_t {
    src.raw
  }
}
impl From<cef_sys::cef_range_t> for CefRange {
  fn from(raw: cef_sys::cef_range_t) -> CefRange {
    CefRange { raw }
  }
}
#[allow(non_snake_case)]
impl CefRange {
  pub fn from(&self) -> i32 { self.raw.from }
  pub fn set_from(&mut self, v: i32) -> &mut Self { self.raw.from = v; self }
  pub fn to(&self) -> i32 { self.raw.to }
  pub fn set_to(&mut self, v: i32) -> &mut Self { self.raw.to = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Structure representing insets.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefInsets { pub raw: cef_sys::cef_insets_t }
impl Default for CefInsets {
  fn default() -> CefInsets {
    let /* mut */ raw = cef_sys::cef_insets_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_insets_t>() as u64;
    CefInsets { raw }
  }
}
impl From<CefInsets> for cef_sys::cef_insets_t {
  fn from(src: CefInsets) -> cef_sys::cef_insets_t {
    src.raw
  }
}
impl From<cef_sys::cef_insets_t> for CefInsets {
  fn from(raw: cef_sys::cef_insets_t) -> CefInsets {
    CefInsets { raw }
  }
}
#[allow(non_snake_case)]
impl CefInsets {
  pub fn top(&self) -> i32 { self.raw.top }
  pub fn set_top(&mut self, v: i32) -> &mut Self { self.raw.top = v; self }
  pub fn left(&self) -> i32 { self.raw.left }
  pub fn set_left(&mut self, v: i32) -> &mut Self { self.raw.left = v; self }
  pub fn bottom(&self) -> i32 { self.raw.bottom }
  pub fn set_bottom(&mut self, v: i32) -> &mut Self { self.raw.bottom = v; self }
  pub fn right(&self) -> i32 { self.raw.right }
  pub fn set_right(&mut self, v: i32) -> &mut Self { self.raw.right = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Structure representing a draggable region.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefDraggableRegion { pub raw: cef_sys::cef_draggable_region_t }
impl Default for CefDraggableRegion {
  fn default() -> CefDraggableRegion {
    let /* mut */ raw = cef_sys::cef_draggable_region_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_draggable_region_t>() as u64;
    CefDraggableRegion { raw }
  }
}
impl From<CefDraggableRegion> for cef_sys::cef_draggable_region_t {
  fn from(src: CefDraggableRegion) -> cef_sys::cef_draggable_region_t {
    src.raw
  }
}
impl From<cef_sys::cef_draggable_region_t> for CefDraggableRegion {
  fn from(raw: cef_sys::cef_draggable_region_t) -> CefDraggableRegion {
    CefDraggableRegion { raw }
  }
}
#[allow(non_snake_case)]
impl CefDraggableRegion {
  /// Bounds of the region.
  pub fn bounds(&self) -> &crate::include::internal::CefRect { unsafe { &*(self as *const _ as *const _) } }
  /// Bounds of the region.
  pub fn set_bounds(&mut self, v: crate::include::internal::CefRect) -> &mut Self { self.raw.bounds = v.into(); self }
  /// True (1) this this region is draggable and false (0) otherwise.
  pub fn draggable(&self) -> i32 { self.raw.draggable }
  /// True (1) this this region is draggable and false (0) otherwise.
  pub fn set_draggable(&mut self, v: i32) -> &mut Self { self.raw.draggable = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Existing process IDs.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefProcessId(cef_sys::cef_process_id_t);
impl CefProcessId {
  /// Browser process.
  pub const BROWSER: CefProcessId = CefProcessId(cef_sys::cef_process_id_t_PID_BROWSER);
  /// Renderer process.
  pub const RENDERER: CefProcessId = CefProcessId(cef_sys::cef_process_id_t_PID_RENDERER);
}
impl From<cef_sys::cef_process_id_t> for CefProcessId {
  fn from(raw: cef_sys::cef_process_id_t) -> CefProcessId {
    CefProcessId(raw)
  }
}
impl From<CefProcessId> for cef_sys::cef_process_id_t {
  fn from(src: CefProcessId) -> cef_sys::cef_process_id_t {
    src.0
  }
}
impl std::ops::BitOr for CefProcessId {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Existing thread IDs.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefThreadId(cef_sys::cef_thread_id_t);
impl CefThreadId {
  /// The main thread in the browser. This will be the same as the main
  /// application thread if CefInitialize() is called with a
  /// CefSettings.multi_threaded_message_loop value of false. Do not perform
  /// blocking tasks on this thread. All tasks posted after
  /// CefBrowserProcessHandler::OnContextInitialized() and before CefShutdown()
  /// are guaranteed to run. This thread will outlive all other CEF threads.
  pub const UI: CefThreadId = CefThreadId(cef_sys::cef_thread_id_t_TID_UI);
  /// Used for blocking tasks (e.g. file system access) where the user won't
  /// notice if the task takes an arbitrarily long time to complete. All tasks
  /// posted after CefBrowserProcessHandler::OnContextInitialized() and before
  /// CefShutdown() are guaranteed to run.
  pub const FILE_BACKGROUND: CefThreadId = CefThreadId(cef_sys::cef_thread_id_t_TID_FILE_BACKGROUND);
  pub const FILE: CefThreadId = CefThreadId(cef_sys::cef_thread_id_t_TID_FILE);
  /// Used for blocking tasks (e.g. file system access) that affect UI or
  /// responsiveness of future user interactions. Do not use if an immediate
  /// response to a user interaction is expected. All tasks posted after
  /// CefBrowserProcessHandler::OnContextInitialized() and before CefShutdown()
  /// are guaranteed to run.
  /// Examples:
  /// - Updating the UI to reflect progress on a long task.
  /// - Loading data that might be shown in the UI after a future user
  /// interaction.
  pub const FILE_USER_VISIBLE: CefThreadId = CefThreadId(cef_sys::cef_thread_id_t_TID_FILE_USER_VISIBLE);
  /// Used for blocking tasks (e.g. file system access) that affect UI
  /// immediately after a user interaction. All tasks posted after
  /// CefBrowserProcessHandler::OnContextInitialized() and before CefShutdown()
  /// are guaranteed to run.
  /// Example: Generating data shown in the UI immediately after a click.
  pub const FILE_USER_BLOCKING: CefThreadId = CefThreadId(cef_sys::cef_thread_id_t_TID_FILE_USER_BLOCKING);
  /// Used to launch and terminate browser processes.
  pub const PROCESS_LAUNCHER: CefThreadId = CefThreadId(cef_sys::cef_thread_id_t_TID_PROCESS_LAUNCHER);
  /// Used to process IPC and network messages. Do not perform blocking tasks on
  /// this thread. All tasks posted after
  /// CefBrowserProcessHandler::OnContextInitialized() and before CefShutdown()
  /// are guaranteed to run.
  pub const IO: CefThreadId = CefThreadId(cef_sys::cef_thread_id_t_TID_IO);
  /// The main thread in the renderer. Used for all WebKit and V8 interaction.
  /// Tasks may be posted to this thread after
  /// CefRenderProcessHandler::OnWebKitInitialized but are not guaranteed to
  /// run before sub-process termination (sub-processes may be killed at any time
  /// without warning).
  pub const RENDERER: CefThreadId = CefThreadId(cef_sys::cef_thread_id_t_TID_RENDERER);
}
impl From<cef_sys::cef_thread_id_t> for CefThreadId {
  fn from(raw: cef_sys::cef_thread_id_t) -> CefThreadId {
    CefThreadId(raw)
  }
}
impl From<CefThreadId> for cef_sys::cef_thread_id_t {
  fn from(src: CefThreadId) -> cef_sys::cef_thread_id_t {
    src.0
  }
}
impl std::ops::BitOr for CefThreadId {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Thread priority values listed in increasing order of importance.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefThreadPriority(cef_sys::cef_thread_priority_t);
impl CefThreadPriority {
  /// Suitable for threads that shouldn't disrupt high priority work.
  pub const BACKGROUND: CefThreadPriority = CefThreadPriority(cef_sys::cef_thread_priority_t_TP_BACKGROUND);
  /// Default priority level.
  pub const NORMAL: CefThreadPriority = CefThreadPriority(cef_sys::cef_thread_priority_t_TP_NORMAL);
  /// Suitable for threads which generate data for the display (at ~60Hz).
  pub const DISPLAY: CefThreadPriority = CefThreadPriority(cef_sys::cef_thread_priority_t_TP_DISPLAY);
  /// Suitable for low-latency, glitch-resistant audio.
  pub const REALTIME_AUDIO: CefThreadPriority = CefThreadPriority(cef_sys::cef_thread_priority_t_TP_REALTIME_AUDIO);
}
impl From<cef_sys::cef_thread_priority_t> for CefThreadPriority {
  fn from(raw: cef_sys::cef_thread_priority_t) -> CefThreadPriority {
    CefThreadPriority(raw)
  }
}
impl From<CefThreadPriority> for cef_sys::cef_thread_priority_t {
  fn from(src: CefThreadPriority) -> cef_sys::cef_thread_priority_t {
    src.0
  }
}
impl std::ops::BitOr for CefThreadPriority {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Message loop types. Indicates the set of asynchronous events that a message
/// loop can process.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMessageLoopType(cef_sys::cef_message_loop_type_t);
impl CefMessageLoopType {
  /// Supports tasks and timers.
  pub const DEFAULT: CefMessageLoopType = CefMessageLoopType(cef_sys::cef_message_loop_type_t_ML_TYPE_DEFAULT);
  /// Supports tasks, timers and native UI events (e.g. Windows messages).
  pub const UI: CefMessageLoopType = CefMessageLoopType(cef_sys::cef_message_loop_type_t_ML_TYPE_UI);
  /// Supports tasks, timers and asynchronous IO events.
  pub const IO: CefMessageLoopType = CefMessageLoopType(cef_sys::cef_message_loop_type_t_ML_TYPE_IO);
}
impl From<cef_sys::cef_message_loop_type_t> for CefMessageLoopType {
  fn from(raw: cef_sys::cef_message_loop_type_t) -> CefMessageLoopType {
    CefMessageLoopType(raw)
  }
}
impl From<CefMessageLoopType> for cef_sys::cef_message_loop_type_t {
  fn from(src: CefMessageLoopType) -> cef_sys::cef_message_loop_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefMessageLoopType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Windows COM initialization mode. Specifies how COM will be initialized for a
/// new thread.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefComInitMode(cef_sys::cef_com_init_mode_t);
impl CefComInitMode {
  /// No COM initialization.
  pub const NONE: CefComInitMode = CefComInitMode(cef_sys::cef_com_init_mode_t_COM_INIT_MODE_NONE);
  /// Initialize COM using single-threaded apartments.
  pub const STA: CefComInitMode = CefComInitMode(cef_sys::cef_com_init_mode_t_COM_INIT_MODE_STA);
  /// Initialize COM using multi-threaded apartments.
  pub const MTA: CefComInitMode = CefComInitMode(cef_sys::cef_com_init_mode_t_COM_INIT_MODE_MTA);
}
impl From<cef_sys::cef_com_init_mode_t> for CefComInitMode {
  fn from(raw: cef_sys::cef_com_init_mode_t) -> CefComInitMode {
    CefComInitMode(raw)
  }
}
impl From<CefComInitMode> for cef_sys::cef_com_init_mode_t {
  fn from(src: CefComInitMode) -> cef_sys::cef_com_init_mode_t {
    src.0
  }
}
impl std::ops::BitOr for CefComInitMode {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported value types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefValueType(cef_sys::cef_value_type_t);
impl CefValueType {
  pub const INVALID: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_INVALID);
  pub const NULL: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_NULL);
  pub const BOOL: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_BOOL);
  pub const INT: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_INT);
  pub const DOUBLE: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_DOUBLE);
  pub const STRING: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_STRING);
  pub const BINARY: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_BINARY);
  pub const DICTIONARY: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_DICTIONARY);
  pub const LIST: CefValueType = CefValueType(cef_sys::cef_value_type_t_VTYPE_LIST);
}
impl From<cef_sys::cef_value_type_t> for CefValueType {
  fn from(raw: cef_sys::cef_value_type_t) -> CefValueType {
    CefValueType(raw)
  }
}
impl From<CefValueType> for cef_sys::cef_value_type_t {
  fn from(src: CefValueType) -> cef_sys::cef_value_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefValueType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported JavaScript dialog types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefJsdialogType(cef_sys::cef_jsdialog_type_t);
impl CefJsdialogType {
  pub const ALERT: CefJsdialogType = CefJsdialogType(cef_sys::cef_jsdialog_type_t_JSDIALOGTYPE_ALERT);
  pub const CONFIRM: CefJsdialogType = CefJsdialogType(cef_sys::cef_jsdialog_type_t_JSDIALOGTYPE_CONFIRM);
  pub const PROMPT: CefJsdialogType = CefJsdialogType(cef_sys::cef_jsdialog_type_t_JSDIALOGTYPE_PROMPT);
}
impl From<cef_sys::cef_jsdialog_type_t> for CefJsdialogType {
  fn from(raw: cef_sys::cef_jsdialog_type_t) -> CefJsdialogType {
    CefJsdialogType(raw)
  }
}
impl From<CefJsdialogType> for cef_sys::cef_jsdialog_type_t {
  fn from(src: CefJsdialogType) -> cef_sys::cef_jsdialog_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefJsdialogType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Screen information used when window rendering is disabled. This structure is
/// passed as a parameter to CefRenderHandler::GetScreenInfo and should be filled
/// in by the client.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefScreenInfo { pub raw: cef_sys::cef_screen_info_t }
impl Default for CefScreenInfo {
  fn default() -> CefScreenInfo {
    let /* mut */ raw = cef_sys::cef_screen_info_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_screen_info_t>() as u64;
    CefScreenInfo { raw }
  }
}
impl From<CefScreenInfo> for cef_sys::cef_screen_info_t {
  fn from(src: CefScreenInfo) -> cef_sys::cef_screen_info_t {
    src.raw
  }
}
impl From<cef_sys::cef_screen_info_t> for CefScreenInfo {
  fn from(raw: cef_sys::cef_screen_info_t) -> CefScreenInfo {
    CefScreenInfo { raw }
  }
}
#[allow(non_snake_case)]
impl CefScreenInfo {
  /// Device scale factor. Specifies the ratio between physical and logical
  /// pixels.
  pub fn device_scale_factor(&self) -> f32 { self.raw.device_scale_factor }
  /// Device scale factor. Specifies the ratio between physical and logical
  /// pixels.
  pub fn set_device_scale_factor(&mut self, v: f32) -> &mut Self { self.raw.device_scale_factor = v; self }
  /// The screen depth in bits per pixel.
  pub fn depth(&self) -> i32 { self.raw.depth }
  /// The screen depth in bits per pixel.
  pub fn set_depth(&mut self, v: i32) -> &mut Self { self.raw.depth = v; self }
  /// The bits per color component. This assumes that the colors are balanced
  /// equally.
  pub fn depth_per_component(&self) -> i32 { self.raw.depth_per_component }
  /// The bits per color component. This assumes that the colors are balanced
  /// equally.
  pub fn set_depth_per_component(&mut self, v: i32) -> &mut Self { self.raw.depth_per_component = v; self }
  /// This can be true for black and white printers.
  pub fn is_monochrome(&self) -> i32 { self.raw.is_monochrome }
  /// This can be true for black and white printers.
  pub fn set_is_monochrome(&mut self, v: i32) -> &mut Self { self.raw.is_monochrome = v; self }
  /// This is set from the rcMonitor member of MONITORINFOEX, to whit:
  /// "A RECT structure that specifies the display monitor rectangle,
  /// expressed in virtual-screen coordinates. Note that if the monitor
  /// is not the primary display monitor, some of the rectangle's
  /// coordinates may be negative values."
  /// 
  /// The |rect| and |available_rect| properties are used to determine the
  /// available surface for rendering popup views.
  pub fn rect(&self) -> &crate::include::internal::CefRect { unsafe { &*(self as *const _ as *const _) } }
  /// This is set from the rcMonitor member of MONITORINFOEX, to whit:
  /// "A RECT structure that specifies the display monitor rectangle,
  /// expressed in virtual-screen coordinates. Note that if the monitor
  /// is not the primary display monitor, some of the rectangle's
  /// coordinates may be negative values."
  /// 
  /// The |rect| and |available_rect| properties are used to determine the
  /// available surface for rendering popup views.
  pub fn set_rect(&mut self, v: crate::include::internal::CefRect) -> &mut Self { self.raw.rect = v.into(); self }
  /// This is set from the rcWork member of MONITORINFOEX, to whit:
  /// "A RECT structure that specifies the work area rectangle of the
  /// display monitor that can be used by applications, expressed in
  /// virtual-screen coordinates. Windows uses this rectangle to
  /// maximize an application on the monitor. The rest of the area in
  /// rcMonitor contains system windows such as the task bar and side
  /// bars. Note that if the monitor is not the primary display monitor,
  /// some of the rectangle's coordinates may be negative values".
  /// 
  /// The |rect| and |available_rect| properties are used to determine the
  /// available surface for rendering popup views.
  pub fn available_rect(&self) -> &crate::include::internal::CefRect { unsafe { &*(self as *const _ as *const _) } }
  /// This is set from the rcWork member of MONITORINFOEX, to whit:
  /// "A RECT structure that specifies the work area rectangle of the
  /// display monitor that can be used by applications, expressed in
  /// virtual-screen coordinates. Windows uses this rectangle to
  /// maximize an application on the monitor. The rest of the area in
  /// rcMonitor contains system windows such as the task bar and side
  /// bars. Note that if the monitor is not the primary display monitor,
  /// some of the rectangle's coordinates may be negative values".
  /// 
  /// The |rect| and |available_rect| properties are used to determine the
  /// available surface for rendering popup views.
  pub fn set_available_rect(&mut self, v: crate::include::internal::CefRect) -> &mut Self { self.raw.available_rect = v.into(); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Supported menu IDs. Non-English translations can be provided for the
/// IDS_MENU_* strings in CefResourceBundleHandler::GetLocalizedString().
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMenuId(cef_sys::cef_menu_id_t);
impl CefMenuId {
  /// Navigation.
  pub const BACK: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_BACK);
  pub const FORWARD: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_FORWARD);
  pub const RELOAD: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_RELOAD);
  pub const RELOAD_NOCACHE: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_RELOAD_NOCACHE);
  pub const STOPLOAD: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_STOPLOAD);
  /// Editing.
  pub const UNDO: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_UNDO);
  pub const REDO: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_REDO);
  pub const CUT: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_CUT);
  pub const COPY: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_COPY);
  pub const PASTE: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_PASTE);
  pub const DELETE: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_DELETE);
  pub const SELECT_ALL: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_SELECT_ALL);
  /// Miscellaneous.
  pub const FIND: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_FIND);
  pub const PRINT: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_PRINT);
  pub const VIEW_SOURCE: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_VIEW_SOURCE);
  /// Spell checking word correction suggestions.
  pub const SPELLCHECK_SUGGESTION_0: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_SPELLCHECK_SUGGESTION_0);
  pub const SPELLCHECK_SUGGESTION_1: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_SPELLCHECK_SUGGESTION_1);
  pub const SPELLCHECK_SUGGESTION_2: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_SPELLCHECK_SUGGESTION_2);
  pub const SPELLCHECK_SUGGESTION_3: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_SPELLCHECK_SUGGESTION_3);
  pub const SPELLCHECK_SUGGESTION_4: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_SPELLCHECK_SUGGESTION_4);
  pub const SPELLCHECK_SUGGESTION_LAST: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_SPELLCHECK_SUGGESTION_LAST);
  pub const NO_SPELLING_SUGGESTIONS: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_NO_SPELLING_SUGGESTIONS);
  pub const ADD_TO_DICTIONARY: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_ADD_TO_DICTIONARY);
  /// Custom menu items originating from the renderer process. For example,
  /// plugin placeholder menu items or Flash menu items.
  pub const CUSTOM_FIRST: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_CUSTOM_FIRST);
  pub const CUSTOM_LAST: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_CUSTOM_LAST);
  /// All user-defined menu IDs should come between MENU_ID_USER_FIRST and
  /// MENU_ID_USER_LAST to avoid overlapping the Chromium and CEF ID ranges
  /// defined in the tools/gritsettings/resource_ids file.
  pub const USER_FIRST: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_USER_FIRST);
  pub const USER_LAST: CefMenuId = CefMenuId(cef_sys::cef_menu_id_t_MENU_ID_USER_LAST);
}
impl From<cef_sys::cef_menu_id_t> for CefMenuId {
  fn from(raw: cef_sys::cef_menu_id_t) -> CefMenuId {
    CefMenuId(raw)
  }
}
impl From<CefMenuId> for cef_sys::cef_menu_id_t {
  fn from(src: CefMenuId) -> cef_sys::cef_menu_id_t {
    src.0
  }
}
impl std::ops::BitOr for CefMenuId {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Mouse button types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMouseButtonType(cef_sys::cef_mouse_button_type_t);
impl CefMouseButtonType {
  pub const LEFT: CefMouseButtonType = CefMouseButtonType(cef_sys::cef_mouse_button_type_t_MBT_LEFT);
  pub const MIDDLE: CefMouseButtonType = CefMouseButtonType(cef_sys::cef_mouse_button_type_t_MBT_MIDDLE);
  pub const RIGHT: CefMouseButtonType = CefMouseButtonType(cef_sys::cef_mouse_button_type_t_MBT_RIGHT);
}
impl From<cef_sys::cef_mouse_button_type_t> for CefMouseButtonType {
  fn from(raw: cef_sys::cef_mouse_button_type_t) -> CefMouseButtonType {
    CefMouseButtonType(raw)
  }
}
impl From<CefMouseButtonType> for cef_sys::cef_mouse_button_type_t {
  fn from(src: CefMouseButtonType) -> cef_sys::cef_mouse_button_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefMouseButtonType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Structure representing mouse event information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefMouseEvent { pub raw: cef_sys::cef_mouse_event_t }
impl Default for CefMouseEvent {
  fn default() -> CefMouseEvent {
    let /* mut */ raw = cef_sys::cef_mouse_event_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_mouse_event_t>() as u64;
    CefMouseEvent { raw }
  }
}
impl From<CefMouseEvent> for cef_sys::cef_mouse_event_t {
  fn from(src: CefMouseEvent) -> cef_sys::cef_mouse_event_t {
    src.raw
  }
}
impl From<cef_sys::cef_mouse_event_t> for CefMouseEvent {
  fn from(raw: cef_sys::cef_mouse_event_t) -> CefMouseEvent {
    CefMouseEvent { raw }
  }
}
#[allow(non_snake_case)]
impl CefMouseEvent {
  /// X coordinate relative to the left side of the view.
  pub fn x(&self) -> i32 { self.raw.x }
  /// X coordinate relative to the left side of the view.
  pub fn set_x(&mut self, v: i32) -> &mut Self { self.raw.x = v; self }
  /// Y coordinate relative to the top side of the view.
  pub fn y(&self) -> i32 { self.raw.y }
  /// Y coordinate relative to the top side of the view.
  pub fn set_y(&mut self, v: i32) -> &mut Self { self.raw.y = v; self }
  /// Bit flags describing any pressed modifier keys. See
  /// cef_event_flags_t for values.
  pub fn modifiers(&self) -> u32 { self.raw.modifiers }
  /// Bit flags describing any pressed modifier keys. See
  /// cef_event_flags_t for values.
  pub fn set_modifiers(&mut self, v: u32) -> &mut Self { self.raw.modifiers = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Touch points states types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefTouchEventType(cef_sys::cef_touch_event_type_t);
impl CefTouchEventType {
  pub const RELEASED: CefTouchEventType = CefTouchEventType(cef_sys::cef_touch_event_type_t_CEF_TET_RELEASED);
  pub const PRESSED: CefTouchEventType = CefTouchEventType(cef_sys::cef_touch_event_type_t_CEF_TET_PRESSED);
  pub const MOVED: CefTouchEventType = CefTouchEventType(cef_sys::cef_touch_event_type_t_CEF_TET_MOVED);
  pub const CANCELLED: CefTouchEventType = CefTouchEventType(cef_sys::cef_touch_event_type_t_CEF_TET_CANCELLED);
}
impl From<cef_sys::cef_touch_event_type_t> for CefTouchEventType {
  fn from(raw: cef_sys::cef_touch_event_type_t) -> CefTouchEventType {
    CefTouchEventType(raw)
  }
}
impl From<CefTouchEventType> for cef_sys::cef_touch_event_type_t {
  fn from(src: CefTouchEventType) -> cef_sys::cef_touch_event_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefTouchEventType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// The device type that caused the event.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefPointerType(cef_sys::cef_pointer_type_t);
impl CefPointerType {
  pub const TOUCH: CefPointerType = CefPointerType(cef_sys::cef_pointer_type_t_CEF_POINTER_TYPE_TOUCH);
  pub const MOUSE: CefPointerType = CefPointerType(cef_sys::cef_pointer_type_t_CEF_POINTER_TYPE_MOUSE);
  pub const PEN: CefPointerType = CefPointerType(cef_sys::cef_pointer_type_t_CEF_POINTER_TYPE_PEN);
  pub const ERASER: CefPointerType = CefPointerType(cef_sys::cef_pointer_type_t_CEF_POINTER_TYPE_ERASER);
  pub const UNKNOWN: CefPointerType = CefPointerType(cef_sys::cef_pointer_type_t_CEF_POINTER_TYPE_UNKNOWN);
}
impl From<cef_sys::cef_pointer_type_t> for CefPointerType {
  fn from(raw: cef_sys::cef_pointer_type_t) -> CefPointerType {
    CefPointerType(raw)
  }
}
impl From<CefPointerType> for cef_sys::cef_pointer_type_t {
  fn from(src: CefPointerType) -> cef_sys::cef_pointer_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefPointerType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Structure representing touch event information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefTouchEvent { pub raw: cef_sys::cef_touch_event_t }
impl Default for CefTouchEvent {
  fn default() -> CefTouchEvent {
    let /* mut */ raw = cef_sys::cef_touch_event_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_touch_event_t>() as u64;
    CefTouchEvent { raw }
  }
}
impl From<CefTouchEvent> for cef_sys::cef_touch_event_t {
  fn from(src: CefTouchEvent) -> cef_sys::cef_touch_event_t {
    src.raw
  }
}
impl From<cef_sys::cef_touch_event_t> for CefTouchEvent {
  fn from(raw: cef_sys::cef_touch_event_t) -> CefTouchEvent {
    CefTouchEvent { raw }
  }
}
#[allow(non_snake_case)]
impl CefTouchEvent {
  /// Id of a touch point. Must be unique per touch, can be any number except -1.
  /// Note that a maximum of 16 concurrent touches will be tracked; touches
  /// beyond that will be ignored.
  pub fn id(&self) -> i32 { self.raw.id }
  /// Id of a touch point. Must be unique per touch, can be any number except -1.
  /// Note that a maximum of 16 concurrent touches will be tracked; touches
  /// beyond that will be ignored.
  pub fn set_id(&mut self, v: i32) -> &mut Self { self.raw.id = v; self }
  /// X coordinate relative to the left side of the view.
  pub fn x(&self) -> f32 { self.raw.x }
  /// X coordinate relative to the left side of the view.
  pub fn set_x(&mut self, v: f32) -> &mut Self { self.raw.x = v; self }
  /// Y coordinate relative to the top side of the view.
  pub fn y(&self) -> f32 { self.raw.y }
  /// Y coordinate relative to the top side of the view.
  pub fn set_y(&mut self, v: f32) -> &mut Self { self.raw.y = v; self }
  /// X radius in pixels. Set to 0 if not applicable.
  pub fn radius_x(&self) -> f32 { self.raw.radius_x }
  /// X radius in pixels. Set to 0 if not applicable.
  pub fn set_radius_x(&mut self, v: f32) -> &mut Self { self.raw.radius_x = v; self }
  /// Y radius in pixels. Set to 0 if not applicable.
  pub fn radius_y(&self) -> f32 { self.raw.radius_y }
  /// Y radius in pixels. Set to 0 if not applicable.
  pub fn set_radius_y(&mut self, v: f32) -> &mut Self { self.raw.radius_y = v; self }
  /// Rotation angle in radians. Set to 0 if not applicable.
  pub fn rotation_angle(&self) -> f32 { self.raw.rotation_angle }
  /// Rotation angle in radians. Set to 0 if not applicable.
  pub fn set_rotation_angle(&mut self, v: f32) -> &mut Self { self.raw.rotation_angle = v; self }
  /// The normalized pressure of the pointer input in the range of [0,1].
  /// Set to 0 if not applicable.
  pub fn pressure(&self) -> f32 { self.raw.pressure }
  /// The normalized pressure of the pointer input in the range of [0,1].
  /// Set to 0 if not applicable.
  pub fn set_pressure(&mut self, v: f32) -> &mut Self { self.raw.pressure = v; self }
  /// The state of the touch point. Touches begin with one CEF_TET_PRESSED event
  /// followed by zero or more CEF_TET_MOVED events and finally one
  /// CEF_TET_RELEASED or CEF_TET_CANCELLED event. Events not respecting this
  /// order will be ignored.
  pub fn type_(&self) -> &crate::include::internal::CefTouchEventType { unsafe { &*(self as *const _ as *const _) } }
  /// The state of the touch point. Touches begin with one CEF_TET_PRESSED event
  /// followed by zero or more CEF_TET_MOVED events and finally one
  /// CEF_TET_RELEASED or CEF_TET_CANCELLED event. Events not respecting this
  /// order will be ignored.
  pub fn set_type_(&mut self, v: crate::include::internal::CefTouchEventType) -> &mut Self { self.raw.type_ = v.into(); self }
  /// Bit flags describing any pressed modifier keys. See
  /// cef_event_flags_t for values.
  pub fn modifiers(&self) -> u32 { self.raw.modifiers }
  /// Bit flags describing any pressed modifier keys. See
  /// cef_event_flags_t for values.
  pub fn set_modifiers(&mut self, v: u32) -> &mut Self { self.raw.modifiers = v; self }
  /// The device type that caused the event.
  pub fn pointer_type(&self) -> &crate::include::internal::CefPointerType { unsafe { &*(self as *const _ as *const _) } }
  /// The device type that caused the event.
  pub fn set_pointer_type(&mut self, v: crate::include::internal::CefPointerType) -> &mut Self { self.raw.pointer_type = v.into(); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Paint element types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefPaintElementType(cef_sys::cef_paint_element_type_t);
impl CefPaintElementType {
  pub const VIEW: CefPaintElementType = CefPaintElementType(cef_sys::cef_paint_element_type_t_PET_VIEW);
  pub const POPUP: CefPaintElementType = CefPaintElementType(cef_sys::cef_paint_element_type_t_PET_POPUP);
}
impl From<cef_sys::cef_paint_element_type_t> for CefPaintElementType {
  fn from(raw: cef_sys::cef_paint_element_type_t) -> CefPaintElementType {
    CefPaintElementType(raw)
  }
}
impl From<CefPaintElementType> for cef_sys::cef_paint_element_type_t {
  fn from(src: CefPaintElementType) -> cef_sys::cef_paint_element_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefPaintElementType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported event bit flags.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefEventFlags(cef_sys::cef_event_flags_t);
impl CefEventFlags {
  pub const NONE: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_NONE);
  pub const CAPS_LOCK_ON: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_CAPS_LOCK_ON);
  pub const SHIFT_DOWN: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_SHIFT_DOWN);
  pub const CONTROL_DOWN: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_CONTROL_DOWN);
  pub const ALT_DOWN: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_ALT_DOWN);
  pub const LEFT_MOUSE_BUTTON: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_LEFT_MOUSE_BUTTON);
  pub const MIDDLE_MOUSE_BUTTON: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_MIDDLE_MOUSE_BUTTON);
  pub const RIGHT_MOUSE_BUTTON: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_RIGHT_MOUSE_BUTTON);
  /// Mac OS-X command key.
  pub const COMMAND_DOWN: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_COMMAND_DOWN);
  pub const NUM_LOCK_ON: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_NUM_LOCK_ON);
  pub const IS_KEY_PAD: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_IS_KEY_PAD);
  pub const IS_LEFT: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_IS_LEFT);
  pub const IS_RIGHT: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_IS_RIGHT);
  pub const ALTGR_DOWN: CefEventFlags = CefEventFlags(cef_sys::cef_event_flags_t_EVENTFLAG_ALTGR_DOWN);
}
impl From<cef_sys::cef_event_flags_t> for CefEventFlags {
  fn from(raw: cef_sys::cef_event_flags_t) -> CefEventFlags {
    CefEventFlags(raw)
  }
}
impl From<CefEventFlags> for cef_sys::cef_event_flags_t {
  fn from(src: CefEventFlags) -> cef_sys::cef_event_flags_t {
    src.0
  }
}
impl std::ops::BitOr for CefEventFlags {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported menu item types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMenuItemType(cef_sys::cef_menu_item_type_t);
impl CefMenuItemType {
  pub const NONE: CefMenuItemType = CefMenuItemType(cef_sys::cef_menu_item_type_t_MENUITEMTYPE_NONE);
  pub const COMMAND: CefMenuItemType = CefMenuItemType(cef_sys::cef_menu_item_type_t_MENUITEMTYPE_COMMAND);
  pub const CHECK: CefMenuItemType = CefMenuItemType(cef_sys::cef_menu_item_type_t_MENUITEMTYPE_CHECK);
  pub const RADIO: CefMenuItemType = CefMenuItemType(cef_sys::cef_menu_item_type_t_MENUITEMTYPE_RADIO);
  pub const SEPARATOR: CefMenuItemType = CefMenuItemType(cef_sys::cef_menu_item_type_t_MENUITEMTYPE_SEPARATOR);
  pub const SUBMENU: CefMenuItemType = CefMenuItemType(cef_sys::cef_menu_item_type_t_MENUITEMTYPE_SUBMENU);
}
impl From<cef_sys::cef_menu_item_type_t> for CefMenuItemType {
  fn from(raw: cef_sys::cef_menu_item_type_t) -> CefMenuItemType {
    CefMenuItemType(raw)
  }
}
impl From<CefMenuItemType> for cef_sys::cef_menu_item_type_t {
  fn from(src: CefMenuItemType) -> cef_sys::cef_menu_item_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefMenuItemType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported context menu type flags.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefContextMenuTypeFlags(cef_sys::cef_context_menu_type_flags_t);
impl CefContextMenuTypeFlags {
  /// No node is selected.
  pub const NONE: CefContextMenuTypeFlags = CefContextMenuTypeFlags(cef_sys::cef_context_menu_type_flags_t_CM_TYPEFLAG_NONE);
  /// The top page is selected.
  pub const PAGE: CefContextMenuTypeFlags = CefContextMenuTypeFlags(cef_sys::cef_context_menu_type_flags_t_CM_TYPEFLAG_PAGE);
  /// A subframe page is selected.
  pub const FRAME: CefContextMenuTypeFlags = CefContextMenuTypeFlags(cef_sys::cef_context_menu_type_flags_t_CM_TYPEFLAG_FRAME);
  /// A link is selected.
  pub const LINK: CefContextMenuTypeFlags = CefContextMenuTypeFlags(cef_sys::cef_context_menu_type_flags_t_CM_TYPEFLAG_LINK);
  /// A media node is selected.
  pub const MEDIA: CefContextMenuTypeFlags = CefContextMenuTypeFlags(cef_sys::cef_context_menu_type_flags_t_CM_TYPEFLAG_MEDIA);
  /// There is a textual or mixed selection that is selected.
  pub const SELECTION: CefContextMenuTypeFlags = CefContextMenuTypeFlags(cef_sys::cef_context_menu_type_flags_t_CM_TYPEFLAG_SELECTION);
  /// An editable element is selected.
  pub const EDITABLE: CefContextMenuTypeFlags = CefContextMenuTypeFlags(cef_sys::cef_context_menu_type_flags_t_CM_TYPEFLAG_EDITABLE);
}
impl From<cef_sys::cef_context_menu_type_flags_t> for CefContextMenuTypeFlags {
  fn from(raw: cef_sys::cef_context_menu_type_flags_t) -> CefContextMenuTypeFlags {
    CefContextMenuTypeFlags(raw)
  }
}
impl From<CefContextMenuTypeFlags> for cef_sys::cef_context_menu_type_flags_t {
  fn from(src: CefContextMenuTypeFlags) -> cef_sys::cef_context_menu_type_flags_t {
    src.0
  }
}
impl std::ops::BitOr for CefContextMenuTypeFlags {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported context menu media types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefContextMenuMediaType(cef_sys::cef_context_menu_media_type_t);
impl CefContextMenuMediaType {
  /// No special node is in context.
  pub const NONE: CefContextMenuMediaType = CefContextMenuMediaType(cef_sys::cef_context_menu_media_type_t_CM_MEDIATYPE_NONE);
  /// An image node is selected.
  pub const IMAGE: CefContextMenuMediaType = CefContextMenuMediaType(cef_sys::cef_context_menu_media_type_t_CM_MEDIATYPE_IMAGE);
  /// A video node is selected.
  pub const VIDEO: CefContextMenuMediaType = CefContextMenuMediaType(cef_sys::cef_context_menu_media_type_t_CM_MEDIATYPE_VIDEO);
  /// An audio node is selected.
  pub const AUDIO: CefContextMenuMediaType = CefContextMenuMediaType(cef_sys::cef_context_menu_media_type_t_CM_MEDIATYPE_AUDIO);
  /// A file node is selected.
  pub const FILE: CefContextMenuMediaType = CefContextMenuMediaType(cef_sys::cef_context_menu_media_type_t_CM_MEDIATYPE_FILE);
  /// A plugin node is selected.
  pub const PLUGIN: CefContextMenuMediaType = CefContextMenuMediaType(cef_sys::cef_context_menu_media_type_t_CM_MEDIATYPE_PLUGIN);
}
impl From<cef_sys::cef_context_menu_media_type_t> for CefContextMenuMediaType {
  fn from(raw: cef_sys::cef_context_menu_media_type_t) -> CefContextMenuMediaType {
    CefContextMenuMediaType(raw)
  }
}
impl From<CefContextMenuMediaType> for cef_sys::cef_context_menu_media_type_t {
  fn from(src: CefContextMenuMediaType) -> cef_sys::cef_context_menu_media_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefContextMenuMediaType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported context menu media state bit flags.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t);
impl CefContextMenuMediaStateFlags {
  pub const NONE: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_NONE);
  pub const ERROR: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_ERROR);
  pub const PAUSED: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_PAUSED);
  pub const MUTED: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_MUTED);
  pub const LOOP: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_LOOP);
  pub const CAN_SAVE: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_SAVE);
  pub const HAS_AUDIO: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_HAS_AUDIO);
  pub const HAS_VIDEO: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_HAS_VIDEO);
  pub const CONTROL_ROOT_ELEMENT: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CONTROL_ROOT_ELEMENT);
  pub const CAN_PRINT: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_PRINT);
  pub const CAN_ROTATE: CefContextMenuMediaStateFlags = CefContextMenuMediaStateFlags(cef_sys::cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_ROTATE);
}
impl From<cef_sys::cef_context_menu_media_state_flags_t> for CefContextMenuMediaStateFlags {
  fn from(raw: cef_sys::cef_context_menu_media_state_flags_t) -> CefContextMenuMediaStateFlags {
    CefContextMenuMediaStateFlags(raw)
  }
}
impl From<CefContextMenuMediaStateFlags> for cef_sys::cef_context_menu_media_state_flags_t {
  fn from(src: CefContextMenuMediaStateFlags) -> cef_sys::cef_context_menu_media_state_flags_t {
    src.0
  }
}
impl std::ops::BitOr for CefContextMenuMediaStateFlags {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported context menu edit state bit flags.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t);
impl CefContextMenuEditStateFlags {
  pub const NONE: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_NONE);
  pub const CAN_UNDO: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_UNDO);
  pub const CAN_REDO: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_REDO);
  pub const CAN_CUT: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_CUT);
  pub const CAN_COPY: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_COPY);
  pub const CAN_PASTE: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_PASTE);
  pub const CAN_DELETE: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_DELETE);
  pub const CAN_SELECT_ALL: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_SELECT_ALL);
  pub const CAN_TRANSLATE: CefContextMenuEditStateFlags = CefContextMenuEditStateFlags(cef_sys::cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_TRANSLATE);
}
impl From<cef_sys::cef_context_menu_edit_state_flags_t> for CefContextMenuEditStateFlags {
  fn from(raw: cef_sys::cef_context_menu_edit_state_flags_t) -> CefContextMenuEditStateFlags {
    CefContextMenuEditStateFlags(raw)
  }
}
impl From<CefContextMenuEditStateFlags> for cef_sys::cef_context_menu_edit_state_flags_t {
  fn from(src: CefContextMenuEditStateFlags) -> cef_sys::cef_context_menu_edit_state_flags_t {
    src.0
  }
}
impl std::ops::BitOr for CefContextMenuEditStateFlags {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Key event types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefKeyEventType(cef_sys::cef_key_event_type_t);
impl CefKeyEventType {
  /// Notification that a key transitioned from "up" to "down".
  pub const RAWKEYDOWN: CefKeyEventType = CefKeyEventType(cef_sys::cef_key_event_type_t_KEYEVENT_RAWKEYDOWN);
  /// Notification that a key was pressed. This does not necessarily correspond
  /// to a character depending on the key and language. Use KEYEVENT_CHAR for
  /// character input.
  pub const KEYDOWN: CefKeyEventType = CefKeyEventType(cef_sys::cef_key_event_type_t_KEYEVENT_KEYDOWN);
  /// Notification that a key was released.
  pub const KEYUP: CefKeyEventType = CefKeyEventType(cef_sys::cef_key_event_type_t_KEYEVENT_KEYUP);
  /// Notification that a character was typed. Use this for text input. Key
  /// down events may generate 0, 1, or more than one character event depending
  /// on the key, locale, and operating system.
  pub const CHAR: CefKeyEventType = CefKeyEventType(cef_sys::cef_key_event_type_t_KEYEVENT_CHAR);
}
impl From<cef_sys::cef_key_event_type_t> for CefKeyEventType {
  fn from(raw: cef_sys::cef_key_event_type_t) -> CefKeyEventType {
    CefKeyEventType(raw)
  }
}
impl From<CefKeyEventType> for cef_sys::cef_key_event_type_t {
  fn from(src: CefKeyEventType) -> cef_sys::cef_key_event_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefKeyEventType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Structure representing keyboard event information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefKeyEvent { pub raw: cef_sys::cef_key_event_t }
impl Default for CefKeyEvent {
  fn default() -> CefKeyEvent {
    let /* mut */ raw = cef_sys::cef_key_event_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_key_event_t>() as u64;
    CefKeyEvent { raw }
  }
}
impl From<CefKeyEvent> for cef_sys::cef_key_event_t {
  fn from(src: CefKeyEvent) -> cef_sys::cef_key_event_t {
    src.raw
  }
}
impl From<cef_sys::cef_key_event_t> for CefKeyEvent {
  fn from(raw: cef_sys::cef_key_event_t) -> CefKeyEvent {
    CefKeyEvent { raw }
  }
}
#[allow(non_snake_case)]
impl CefKeyEvent {
  /// The type of keyboard event.
  pub fn type_(&self) -> &crate::include::internal::CefKeyEventType { unsafe { &*(self as *const _ as *const _) } }
  /// The type of keyboard event.
  pub fn set_type_(&mut self, v: crate::include::internal::CefKeyEventType) -> &mut Self { self.raw.type_ = v.into(); self }
  /// Bit flags describing any pressed modifier keys. See
  /// cef_event_flags_t for values.
  pub fn modifiers(&self) -> u32 { self.raw.modifiers }
  /// Bit flags describing any pressed modifier keys. See
  /// cef_event_flags_t for values.
  pub fn set_modifiers(&mut self, v: u32) -> &mut Self { self.raw.modifiers = v; self }
  /// The Windows key code for the key event. This value is used by the DOM
  /// specification. Sometimes it comes directly from the event (i.e. on
  /// Windows) and sometimes it's determined using a mapping function. See
  /// WebCore/platform/chromium/KeyboardCodes.h for the list of values.
  pub fn windows_key_code(&self) -> i32 { self.raw.windows_key_code }
  /// The Windows key code for the key event. This value is used by the DOM
  /// specification. Sometimes it comes directly from the event (i.e. on
  /// Windows) and sometimes it's determined using a mapping function. See
  /// WebCore/platform/chromium/KeyboardCodes.h for the list of values.
  pub fn set_windows_key_code(&mut self, v: i32) -> &mut Self { self.raw.windows_key_code = v; self }
  /// The actual key code genenerated by the platform.
  pub fn native_key_code(&self) -> i32 { self.raw.native_key_code }
  /// The actual key code genenerated by the platform.
  pub fn set_native_key_code(&mut self, v: i32) -> &mut Self { self.raw.native_key_code = v; self }
  /// Indicates whether the event is considered a "system key" event (see
  /// http://msdn.microsoft.com/en-us/library/ms646286(VS.85).aspx for details).
  /// This value will always be false on non-Windows platforms.
  pub fn is_system_key(&self) -> i32 { self.raw.is_system_key }
  /// Indicates whether the event is considered a "system key" event (see
  /// http://msdn.microsoft.com/en-us/library/ms646286(VS.85).aspx for details).
  /// This value will always be false on non-Windows platforms.
  pub fn set_is_system_key(&mut self, v: i32) -> &mut Self { self.raw.is_system_key = v; self }
  /// The character generated by the keystroke.
  pub fn character(&self) -> u16 { self.raw.character }
  /// The character generated by the keystroke.
  pub fn set_character(&mut self, v: u16) -> &mut Self { self.raw.character = v; self }
  /// Same as |character| but unmodified by any concurrently-held modifiers
  /// (except shift). This is useful for working out shortcut keys.
  pub fn unmodified_character(&self) -> u16 { self.raw.unmodified_character }
  /// Same as |character| but unmodified by any concurrently-held modifiers
  /// (except shift). This is useful for working out shortcut keys.
  pub fn set_unmodified_character(&mut self, v: u16) -> &mut Self { self.raw.unmodified_character = v; self }
  /// True if the focus is currently on an editable field on the page. This is
  /// useful for determining if standard key events should be intercepted.
  pub fn focus_on_editable_field(&self) -> i32 { self.raw.focus_on_editable_field }
  /// True if the focus is currently on an editable field on the page. This is
  /// useful for determining if standard key events should be intercepted.
  pub fn set_focus_on_editable_field(&mut self, v: i32) -> &mut Self { self.raw.focus_on_editable_field = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Focus sources.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefFocusSource(cef_sys::cef_focus_source_t);
impl CefFocusSource {
  /// The source is explicit navigation via the API (LoadURL(), etc).
  pub const NAVIGATION: CefFocusSource = CefFocusSource(cef_sys::cef_focus_source_t_FOCUS_SOURCE_NAVIGATION);
  /// The source is a system-generated focus event.
  pub const SYSTEM: CefFocusSource = CefFocusSource(cef_sys::cef_focus_source_t_FOCUS_SOURCE_SYSTEM);
}
impl From<cef_sys::cef_focus_source_t> for CefFocusSource {
  fn from(raw: cef_sys::cef_focus_source_t) -> CefFocusSource {
    CefFocusSource(raw)
  }
}
impl From<CefFocusSource> for cef_sys::cef_focus_source_t {
  fn from(src: CefFocusSource) -> cef_sys::cef_focus_source_t {
    src.0
  }
}
impl std::ops::BitOr for CefFocusSource {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Navigation types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefNavigationType(cef_sys::cef_navigation_type_t);
impl CefNavigationType {
  pub const LINK_CLICKED: CefNavigationType = CefNavigationType(cef_sys::cef_navigation_type_t_NAVIGATION_LINK_CLICKED);
  pub const FORM_SUBMITTED: CefNavigationType = CefNavigationType(cef_sys::cef_navigation_type_t_NAVIGATION_FORM_SUBMITTED);
  pub const BACK_FORWARD: CefNavigationType = CefNavigationType(cef_sys::cef_navigation_type_t_NAVIGATION_BACK_FORWARD);
  pub const RELOAD: CefNavigationType = CefNavigationType(cef_sys::cef_navigation_type_t_NAVIGATION_RELOAD);
  pub const FORM_RESUBMITTED: CefNavigationType = CefNavigationType(cef_sys::cef_navigation_type_t_NAVIGATION_FORM_RESUBMITTED);
  pub const OTHER: CefNavigationType = CefNavigationType(cef_sys::cef_navigation_type_t_NAVIGATION_OTHER);
}
impl From<cef_sys::cef_navigation_type_t> for CefNavigationType {
  fn from(raw: cef_sys::cef_navigation_type_t) -> CefNavigationType {
    CefNavigationType(raw)
  }
}
impl From<CefNavigationType> for cef_sys::cef_navigation_type_t {
  fn from(src: CefNavigationType) -> cef_sys::cef_navigation_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefNavigationType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported XML encoding types. The parser supports ASCII, ISO-8859-1, and
/// UTF16 (LE and BE) by default. All other types must be translated to UTF8
/// before being passed to the parser. If a BOM is detected and the correct
/// decoder is available then that decoder will be used automatically.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefXmlEncodingType(cef_sys::cef_xml_encoding_type_t);
impl CefXmlEncodingType {
  pub const NONE: CefXmlEncodingType = CefXmlEncodingType(cef_sys::cef_xml_encoding_type_t_XML_ENCODING_NONE);
  pub const UTF8: CefXmlEncodingType = CefXmlEncodingType(cef_sys::cef_xml_encoding_type_t_XML_ENCODING_UTF8);
  pub const UTF16LE: CefXmlEncodingType = CefXmlEncodingType(cef_sys::cef_xml_encoding_type_t_XML_ENCODING_UTF16LE);
  pub const UTF16BE: CefXmlEncodingType = CefXmlEncodingType(cef_sys::cef_xml_encoding_type_t_XML_ENCODING_UTF16BE);
  pub const ASCII: CefXmlEncodingType = CefXmlEncodingType(cef_sys::cef_xml_encoding_type_t_XML_ENCODING_ASCII);
}
impl From<cef_sys::cef_xml_encoding_type_t> for CefXmlEncodingType {
  fn from(raw: cef_sys::cef_xml_encoding_type_t) -> CefXmlEncodingType {
    CefXmlEncodingType(raw)
  }
}
impl From<CefXmlEncodingType> for cef_sys::cef_xml_encoding_type_t {
  fn from(src: CefXmlEncodingType) -> cef_sys::cef_xml_encoding_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefXmlEncodingType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// XML node types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefXmlNodeType(cef_sys::cef_xml_node_type_t);
impl CefXmlNodeType {
  pub const UNSUPPORTED: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_UNSUPPORTED);
  pub const PROCESSING_INSTRUCTION: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_PROCESSING_INSTRUCTION);
  pub const DOCUMENT_TYPE: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_DOCUMENT_TYPE);
  pub const ELEMENT_START: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_ELEMENT_START);
  pub const ELEMENT_END: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_ELEMENT_END);
  pub const ATTRIBUTE: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_ATTRIBUTE);
  pub const TEXT: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_TEXT);
  pub const CDATA: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_CDATA);
  pub const ENTITY_REFERENCE: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_ENTITY_REFERENCE);
  pub const WHITESPACE: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_WHITESPACE);
  pub const COMMENT: CefXmlNodeType = CefXmlNodeType(cef_sys::cef_xml_node_type_t_XML_NODE_COMMENT);
}
impl From<cef_sys::cef_xml_node_type_t> for CefXmlNodeType {
  fn from(raw: cef_sys::cef_xml_node_type_t) -> CefXmlNodeType {
    CefXmlNodeType(raw)
  }
}
impl From<CefXmlNodeType> for cef_sys::cef_xml_node_type_t {
  fn from(src: CefXmlNodeType) -> cef_sys::cef_xml_node_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefXmlNodeType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Popup window features.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefPopupFeatures { pub raw: cef_sys::cef_popup_features_t }
impl Default for CefPopupFeatures {
  fn default() -> CefPopupFeatures {
    let /* mut */ raw = cef_sys::cef_popup_features_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_popup_features_t>() as u64;
    CefPopupFeatures { raw }
  }
}
impl From<CefPopupFeatures> for cef_sys::cef_popup_features_t {
  fn from(src: CefPopupFeatures) -> cef_sys::cef_popup_features_t {
    src.raw
  }
}
impl From<cef_sys::cef_popup_features_t> for CefPopupFeatures {
  fn from(raw: cef_sys::cef_popup_features_t) -> CefPopupFeatures {
    CefPopupFeatures { raw }
  }
}
#[allow(non_snake_case)]
impl CefPopupFeatures {
  pub fn x(&self) -> i32 { self.raw.x }
  pub fn set_x(&mut self, v: i32) -> &mut Self { self.raw.x = v; self }
  pub fn xSet(&self) -> i32 { self.raw.xSet }
  pub fn set_xSet(&mut self, v: i32) -> &mut Self { self.raw.xSet = v; self }
  pub fn y(&self) -> i32 { self.raw.y }
  pub fn set_y(&mut self, v: i32) -> &mut Self { self.raw.y = v; self }
  pub fn ySet(&self) -> i32 { self.raw.ySet }
  pub fn set_ySet(&mut self, v: i32) -> &mut Self { self.raw.ySet = v; self }
  pub fn width(&self) -> i32 { self.raw.width }
  pub fn set_width(&mut self, v: i32) -> &mut Self { self.raw.width = v; self }
  pub fn widthSet(&self) -> i32 { self.raw.widthSet }
  pub fn set_widthSet(&mut self, v: i32) -> &mut Self { self.raw.widthSet = v; self }
  pub fn height(&self) -> i32 { self.raw.height }
  pub fn set_height(&mut self, v: i32) -> &mut Self { self.raw.height = v; self }
  pub fn heightSet(&self) -> i32 { self.raw.heightSet }
  pub fn set_heightSet(&mut self, v: i32) -> &mut Self { self.raw.heightSet = v; self }
  pub fn menuBarVisible(&self) -> i32 { self.raw.menuBarVisible }
  pub fn set_menuBarVisible(&mut self, v: i32) -> &mut Self { self.raw.menuBarVisible = v; self }
  pub fn statusBarVisible(&self) -> i32 { self.raw.statusBarVisible }
  pub fn set_statusBarVisible(&mut self, v: i32) -> &mut Self { self.raw.statusBarVisible = v; self }
  pub fn toolBarVisible(&self) -> i32 { self.raw.toolBarVisible }
  pub fn set_toolBarVisible(&mut self, v: i32) -> &mut Self { self.raw.toolBarVisible = v; self }
  pub fn scrollbarsVisible(&self) -> i32 { self.raw.scrollbarsVisible }
  pub fn set_scrollbarsVisible(&mut self, v: i32) -> &mut Self { self.raw.scrollbarsVisible = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// DOM document types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefDomDocumentType(cef_sys::cef_dom_document_type_t);
impl CefDomDocumentType {
  pub const UNKNOWN: CefDomDocumentType = CefDomDocumentType(cef_sys::cef_dom_document_type_t_DOM_DOCUMENT_TYPE_UNKNOWN);
  pub const HTML: CefDomDocumentType = CefDomDocumentType(cef_sys::cef_dom_document_type_t_DOM_DOCUMENT_TYPE_HTML);
  pub const XHTML: CefDomDocumentType = CefDomDocumentType(cef_sys::cef_dom_document_type_t_DOM_DOCUMENT_TYPE_XHTML);
  pub const PLUGIN: CefDomDocumentType = CefDomDocumentType(cef_sys::cef_dom_document_type_t_DOM_DOCUMENT_TYPE_PLUGIN);
}
impl From<cef_sys::cef_dom_document_type_t> for CefDomDocumentType {
  fn from(raw: cef_sys::cef_dom_document_type_t) -> CefDomDocumentType {
    CefDomDocumentType(raw)
  }
}
impl From<CefDomDocumentType> for cef_sys::cef_dom_document_type_t {
  fn from(src: CefDomDocumentType) -> cef_sys::cef_dom_document_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefDomDocumentType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// DOM event category flags.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefDomEventCategory(cef_sys::cef_dom_event_category_t);
impl CefDomEventCategory {
  pub const UNKNOWN: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_UNKNOWN);
  pub const UI: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_UI);
  pub const MOUSE: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_MOUSE);
  pub const MUTATION: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_MUTATION);
  pub const KEYBOARD: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_KEYBOARD);
  pub const TEXT: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_TEXT);
  pub const COMPOSITION: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_COMPOSITION);
  pub const DRAG: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_DRAG);
  pub const CLIPBOARD: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_CLIPBOARD);
  pub const MESSAGE: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_MESSAGE);
  pub const WHEEL: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_WHEEL);
  pub const BEFORE_TEXT_INSERTED: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_BEFORE_TEXT_INSERTED);
  pub const OVERFLOW: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_OVERFLOW);
  pub const PAGE_TRANSITION: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_PAGE_TRANSITION);
  pub const POPSTATE: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_POPSTATE);
  pub const PROGRESS: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_PROGRESS);
  pub const XMLHTTPREQUEST_PROGRESS: CefDomEventCategory = CefDomEventCategory(cef_sys::cef_dom_event_category_t_DOM_EVENT_CATEGORY_XMLHTTPREQUEST_PROGRESS);
}
impl From<cef_sys::cef_dom_event_category_t> for CefDomEventCategory {
  fn from(raw: cef_sys::cef_dom_event_category_t) -> CefDomEventCategory {
    CefDomEventCategory(raw)
  }
}
impl From<CefDomEventCategory> for cef_sys::cef_dom_event_category_t {
  fn from(src: CefDomEventCategory) -> cef_sys::cef_dom_event_category_t {
    src.0
  }
}
impl std::ops::BitOr for CefDomEventCategory {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// DOM event processing phases.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefDomEventPhase(cef_sys::cef_dom_event_phase_t);
impl CefDomEventPhase {
  pub const UNKNOWN: CefDomEventPhase = CefDomEventPhase(cef_sys::cef_dom_event_phase_t_DOM_EVENT_PHASE_UNKNOWN);
  pub const CAPTURING: CefDomEventPhase = CefDomEventPhase(cef_sys::cef_dom_event_phase_t_DOM_EVENT_PHASE_CAPTURING);
  pub const AT_TARGET: CefDomEventPhase = CefDomEventPhase(cef_sys::cef_dom_event_phase_t_DOM_EVENT_PHASE_AT_TARGET);
  pub const BUBBLING: CefDomEventPhase = CefDomEventPhase(cef_sys::cef_dom_event_phase_t_DOM_EVENT_PHASE_BUBBLING);
}
impl From<cef_sys::cef_dom_event_phase_t> for CefDomEventPhase {
  fn from(raw: cef_sys::cef_dom_event_phase_t) -> CefDomEventPhase {
    CefDomEventPhase(raw)
  }
}
impl From<CefDomEventPhase> for cef_sys::cef_dom_event_phase_t {
  fn from(src: CefDomEventPhase) -> cef_sys::cef_dom_event_phase_t {
    src.0
  }
}
impl std::ops::BitOr for CefDomEventPhase {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// DOM node types.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefDomNodeType(cef_sys::cef_dom_node_type_t);
impl CefDomNodeType {
  pub const UNSUPPORTED: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_UNSUPPORTED);
  pub const ELEMENT: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_ELEMENT);
  pub const ATTRIBUTE: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_ATTRIBUTE);
  pub const TEXT: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_TEXT);
  pub const CDATA_SECTION: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_CDATA_SECTION);
  pub const PROCESSING_INSTRUCTIONS: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_PROCESSING_INSTRUCTIONS);
  pub const COMMENT: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_COMMENT);
  pub const DOCUMENT: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_DOCUMENT);
  pub const DOCUMENT_TYPE: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_DOCUMENT_TYPE);
  pub const DOCUMENT_FRAGMENT: CefDomNodeType = CefDomNodeType(cef_sys::cef_dom_node_type_t_DOM_NODE_TYPE_DOCUMENT_FRAGMENT);
}
impl From<cef_sys::cef_dom_node_type_t> for CefDomNodeType {
  fn from(raw: cef_sys::cef_dom_node_type_t) -> CefDomNodeType {
    CefDomNodeType(raw)
  }
}
impl From<CefDomNodeType> for cef_sys::cef_dom_node_type_t {
  fn from(src: CefDomNodeType) -> cef_sys::cef_dom_node_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefDomNodeType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported file dialog modes.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefFileDialogMode(cef_sys::cef_file_dialog_mode_t);
impl CefFileDialogMode {
  /// Requires that the file exists before allowing the user to pick it.
  pub const OPEN: CefFileDialogMode = CefFileDialogMode(cef_sys::cef_file_dialog_mode_t_FILE_DIALOG_OPEN);
  /// Like Open, but allows picking multiple files to open.
  pub const OPEN_MULTIPLE: CefFileDialogMode = CefFileDialogMode(cef_sys::cef_file_dialog_mode_t_FILE_DIALOG_OPEN_MULTIPLE);
  /// Like Open, but selects a folder to open.
  pub const OPEN_FOLDER: CefFileDialogMode = CefFileDialogMode(cef_sys::cef_file_dialog_mode_t_FILE_DIALOG_OPEN_FOLDER);
  /// Allows picking a nonexistent file, and prompts to overwrite if the file
  /// already exists.
  pub const SAVE: CefFileDialogMode = CefFileDialogMode(cef_sys::cef_file_dialog_mode_t_FILE_DIALOG_SAVE);
  /// General mask defining the bits used for the type values.
  pub const TYPE_MASK: CefFileDialogMode = CefFileDialogMode(cef_sys::cef_file_dialog_mode_t_FILE_DIALOG_TYPE_MASK);
  /// Prompt to overwrite if the user selects an existing file with the Save
  /// dialog.
  pub const OVERWRITEPROMPT_FLAG: CefFileDialogMode = CefFileDialogMode(cef_sys::cef_file_dialog_mode_t_FILE_DIALOG_OVERWRITEPROMPT_FLAG);
  /// Do not display read-only files.
  pub const HIDEREADONLY_FLAG: CefFileDialogMode = CefFileDialogMode(cef_sys::cef_file_dialog_mode_t_FILE_DIALOG_HIDEREADONLY_FLAG);
}
impl From<cef_sys::cef_file_dialog_mode_t> for CefFileDialogMode {
  fn from(raw: cef_sys::cef_file_dialog_mode_t) -> CefFileDialogMode {
    CefFileDialogMode(raw)
  }
}
impl From<CefFileDialogMode> for cef_sys::cef_file_dialog_mode_t {
  fn from(src: CefFileDialogMode) -> cef_sys::cef_file_dialog_mode_t {
    src.0
  }
}
impl std::ops::BitOr for CefFileDialogMode {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Print job color mode values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefColorModel(cef_sys::cef_color_model_t);
impl CefColorModel {
  pub const UNKNOWN: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_UNKNOWN);
  pub const GRAY: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_GRAY);
  pub const COLOR: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_COLOR);
  pub const CMYK: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_CMYK);
  pub const CMY: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_CMY);
  pub const KCMY: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_KCMY);
  pub const CMY_K: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_CMY_K);
  /// CMY_K represents CMY+K.
  pub const BLACK: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_BLACK);
  pub const GRAYSCALE: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_GRAYSCALE);
  pub const RGB: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_RGB);
  pub const RGB16: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_RGB16);
  pub const RGBA: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_RGBA);
  pub const COLORMODE_COLOR: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_COLORMODE_COLOR);
  /// Used in samsung printer ppds.
  pub const COLORMODE_MONOCHROME: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_COLORMODE_MONOCHROME);
  /// Used in samsung printer ppds.
  pub const HP_COLOR_COLOR: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_HP_COLOR_COLOR);
  /// Used in HP color printer ppds.
  pub const HP_COLOR_BLACK: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_HP_COLOR_BLACK);
  /// Used in HP color printer ppds.
  pub const PRINTOUTMODE_NORMAL: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_PRINTOUTMODE_NORMAL);
  /// Used in foomatic ppds.
  pub const PRINTOUTMODE_NORMAL_GRAY: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_PRINTOUTMODE_NORMAL_GRAY);
  /// Used in foomatic ppds.
  pub const PROCESSCOLORMODEL_CMYK: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_PROCESSCOLORMODEL_CMYK);
  /// Used in canon printer ppds.
  pub const PROCESSCOLORMODEL_GREYSCALE: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_PROCESSCOLORMODEL_GREYSCALE);
  /// Used in canon printer ppds.
  pub const PROCESSCOLORMODEL_RGB: CefColorModel = CefColorModel(cef_sys::cef_color_model_t_COLOR_MODEL_PROCESSCOLORMODEL_RGB);
}
impl From<cef_sys::cef_color_model_t> for CefColorModel {
  fn from(raw: cef_sys::cef_color_model_t) -> CefColorModel {
    CefColorModel(raw)
  }
}
impl From<CefColorModel> for cef_sys::cef_color_model_t {
  fn from(src: CefColorModel) -> cef_sys::cef_color_model_t {
    src.0
  }
}
impl std::ops::BitOr for CefColorModel {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Print job duplex mode values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefDuplexMode(cef_sys::cef_duplex_mode_t);
impl CefDuplexMode {
  pub const UNKNOWN: CefDuplexMode = CefDuplexMode(cef_sys::cef_duplex_mode_t_DUPLEX_MODE_UNKNOWN);
  pub const SIMPLEX: CefDuplexMode = CefDuplexMode(cef_sys::cef_duplex_mode_t_DUPLEX_MODE_SIMPLEX);
  pub const LONG_EDGE: CefDuplexMode = CefDuplexMode(cef_sys::cef_duplex_mode_t_DUPLEX_MODE_LONG_EDGE);
  pub const SHORT_EDGE: CefDuplexMode = CefDuplexMode(cef_sys::cef_duplex_mode_t_DUPLEX_MODE_SHORT_EDGE);
}
impl From<cef_sys::cef_duplex_mode_t> for CefDuplexMode {
  fn from(raw: cef_sys::cef_duplex_mode_t) -> CefDuplexMode {
    CefDuplexMode(raw)
  }
}
impl From<CefDuplexMode> for cef_sys::cef_duplex_mode_t {
  fn from(src: CefDuplexMode) -> cef_sys::cef_duplex_mode_t {
    src.0
  }
}
impl std::ops::BitOr for CefDuplexMode {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Cursor type values.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefCursorType(cef_sys::cef_cursor_type_t);
impl CefCursorType {
  pub const POINTER: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_POINTER);
  pub const CROSS: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_CROSS);
  pub const HAND: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_HAND);
  pub const IBEAM: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_IBEAM);
  pub const WAIT: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_WAIT);
  pub const HELP: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_HELP);
  pub const EASTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_EASTRESIZE);
  pub const NORTHRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHRESIZE);
  pub const NORTHEASTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHEASTRESIZE);
  pub const NORTHWESTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHWESTRESIZE);
  pub const SOUTHRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_SOUTHRESIZE);
  pub const SOUTHEASTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_SOUTHEASTRESIZE);
  pub const SOUTHWESTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_SOUTHWESTRESIZE);
  pub const WESTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_WESTRESIZE);
  pub const NORTHSOUTHRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHSOUTHRESIZE);
  pub const EASTWESTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_EASTWESTRESIZE);
  pub const NORTHEASTSOUTHWESTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHEASTSOUTHWESTRESIZE);
  pub const NORTHWESTSOUTHEASTRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHWESTSOUTHEASTRESIZE);
  pub const COLUMNRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_COLUMNRESIZE);
  pub const ROWRESIZE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_ROWRESIZE);
  pub const MIDDLEPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_MIDDLEPANNING);
  pub const EASTPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_EASTPANNING);
  pub const NORTHPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHPANNING);
  pub const NORTHEASTPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHEASTPANNING);
  pub const NORTHWESTPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NORTHWESTPANNING);
  pub const SOUTHPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_SOUTHPANNING);
  pub const SOUTHEASTPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_SOUTHEASTPANNING);
  pub const SOUTHWESTPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_SOUTHWESTPANNING);
  pub const WESTPANNING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_WESTPANNING);
  pub const MOVE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_MOVE);
  pub const VERTICALTEXT: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_VERTICALTEXT);
  pub const CELL: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_CELL);
  pub const CONTEXTMENU: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_CONTEXTMENU);
  pub const ALIAS: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_ALIAS);
  pub const PROGRESS: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_PROGRESS);
  pub const NODROP: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NODROP);
  pub const COPY: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_COPY);
  pub const NONE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NONE);
  pub const NOTALLOWED: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_NOTALLOWED);
  pub const ZOOMIN: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_ZOOMIN);
  pub const ZOOMOUT: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_ZOOMOUT);
  pub const GRAB: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_GRAB);
  pub const GRABBING: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_GRABBING);
  pub const MIDDLE_PANNING_VERTICAL: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_MIDDLE_PANNING_VERTICAL);
  pub const MIDDLE_PANNING_HORIZONTAL: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_MIDDLE_PANNING_HORIZONTAL);
  pub const CUSTOM: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_CUSTOM);
  pub const DND_NONE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_DND_NONE);
  pub const DND_MOVE: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_DND_MOVE);
  pub const DND_COPY: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_DND_COPY);
  pub const DND_LINK: CefCursorType = CefCursorType(cef_sys::cef_cursor_type_t_CT_DND_LINK);
}
impl From<cef_sys::cef_cursor_type_t> for CefCursorType {
  fn from(raw: cef_sys::cef_cursor_type_t) -> CefCursorType {
    CefCursorType(raw)
  }
}
impl From<CefCursorType> for cef_sys::cef_cursor_type_t {
  fn from(src: CefCursorType) -> cef_sys::cef_cursor_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefCursorType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Structure representing cursor information. |buffer| will be
/// |size.width|*|size.height|*4 bytes in size and represents a BGRA image with
/// an upper-left origin.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefCursorInfo { pub raw: cef_sys::cef_cursor_info_t }
impl Default for CefCursorInfo {
  fn default() -> CefCursorInfo {
    let /* mut */ raw = cef_sys::cef_cursor_info_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_cursor_info_t>() as u64;
    CefCursorInfo { raw }
  }
}
impl From<CefCursorInfo> for cef_sys::cef_cursor_info_t {
  fn from(src: CefCursorInfo) -> cef_sys::cef_cursor_info_t {
    src.raw
  }
}
impl From<cef_sys::cef_cursor_info_t> for CefCursorInfo {
  fn from(raw: cef_sys::cef_cursor_info_t) -> CefCursorInfo {
    CefCursorInfo { raw }
  }
}
#[allow(non_snake_case)]
impl CefCursorInfo {
  pub fn hotspot(&self) -> &crate::include::internal::CefPoint { unsafe { &*(self as *const _ as *const _) } }
  pub fn set_hotspot(&mut self, v: crate::include::internal::CefPoint) -> &mut Self { self.raw.hotspot = v.into(); self }
  pub fn image_scale_factor(&self) -> f32 { self.raw.image_scale_factor }
  pub fn set_image_scale_factor(&mut self, v: f32) -> &mut Self { self.raw.image_scale_factor = v; self }
  pub fn size(&self) -> &crate::include::internal::CefSize { unsafe { &*(self as *const _ as *const _) } }
  pub fn set_size(&mut self, v: crate::include::internal::CefSize) -> &mut Self { self.raw.size = v.into(); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// URI unescape rules passed to CefURIDecode().
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefUriUnescapeRule(cef_sys::cef_uri_unescape_rule_t);
impl CefUriUnescapeRule {
  /// Don't unescape anything at all.
  pub const NONE: CefUriUnescapeRule = CefUriUnescapeRule(cef_sys::cef_uri_unescape_rule_t_UU_NONE);
  /// Don't unescape anything special, but all normal unescaping will happen.
  /// This is a placeholder and can't be combined with other flags (since it's
  /// just the absence of them). All other unescape rules imply "normal" in
  /// addition to their special meaning. Things like escaped letters, digits,
  /// and most symbols will get unescaped with this mode.
  pub const NORMAL: CefUriUnescapeRule = CefUriUnescapeRule(cef_sys::cef_uri_unescape_rule_t_UU_NORMAL);
  /// Convert %20 to spaces. In some places where we're showing URLs, we may
  /// want this. In places where the URL may be copied and pasted out, then
  /// you wouldn't want this since it might not be interpreted in one piece
  /// by other applications.
  pub const SPACES: CefUriUnescapeRule = CefUriUnescapeRule(cef_sys::cef_uri_unescape_rule_t_UU_SPACES);
  /// Unescapes '/' and '\\'. If these characters were unescaped, the resulting
  /// URL won't be the same as the source one. Moreover, they are dangerous to
  /// unescape in strings that will be used as file paths or names. This value
  /// should only be used when slashes don't have special meaning, like data
  /// URLs.
  pub const PATH_SEPARATORS: CefUriUnescapeRule = CefUriUnescapeRule(cef_sys::cef_uri_unescape_rule_t_UU_PATH_SEPARATORS);
  /// Unescapes various characters that will change the meaning of URLs,
  /// including '%', '+', '&', '#'. Does not unescape path separators.
  /// If these characters were unescaped, the resulting URL won't be the same
  /// as the source one. This flag is used when generating final output like
  /// filenames for URLs where we won't be interpreting as a URL and want to do
  /// as much unescaping as possible.
  pub const URL_SPECIAL_CHARS_EXCEPT_PATH_SEPARATORS: CefUriUnescapeRule = CefUriUnescapeRule(cef_sys::cef_uri_unescape_rule_t_UU_URL_SPECIAL_CHARS_EXCEPT_PATH_SEPARATORS);
  /// URL queries use "+" for space. This flag controls that replacement.
  pub const REPLACE_PLUS_WITH_SPACE: CefUriUnescapeRule = CefUriUnescapeRule(cef_sys::cef_uri_unescape_rule_t_UU_REPLACE_PLUS_WITH_SPACE);
}
impl From<cef_sys::cef_uri_unescape_rule_t> for CefUriUnescapeRule {
  fn from(raw: cef_sys::cef_uri_unescape_rule_t) -> CefUriUnescapeRule {
    CefUriUnescapeRule(raw)
  }
}
impl From<CefUriUnescapeRule> for cef_sys::cef_uri_unescape_rule_t {
  fn from(src: CefUriUnescapeRule) -> cef_sys::cef_uri_unescape_rule_t {
    src.0
  }
}
impl std::ops::BitOr for CefUriUnescapeRule {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Options that can be passed to CefParseJSON.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefJsonParserOptions(cef_sys::cef_json_parser_options_t);
impl CefJsonParserOptions {
  /// Parses the input strictly according to RFC 4627. See comments in Chromium's
  /// base/json/json_reader.h file for known limitations/deviations from the RFC.
  pub const RFC: CefJsonParserOptions = CefJsonParserOptions(cef_sys::cef_json_parser_options_t_JSON_PARSER_RFC);
  /// Allows commas to exist after the last element in structures.
  pub const ALLOW_TRAILING_COMMAS: CefJsonParserOptions = CefJsonParserOptions(cef_sys::cef_json_parser_options_t_JSON_PARSER_ALLOW_TRAILING_COMMAS);
}
impl From<cef_sys::cef_json_parser_options_t> for CefJsonParserOptions {
  fn from(raw: cef_sys::cef_json_parser_options_t) -> CefJsonParserOptions {
    CefJsonParserOptions(raw)
  }
}
impl From<CefJsonParserOptions> for cef_sys::cef_json_parser_options_t {
  fn from(src: CefJsonParserOptions) -> cef_sys::cef_json_parser_options_t {
    src.0
  }
}
impl std::ops::BitOr for CefJsonParserOptions {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Error codes that can be returned from CefParseJSONAndReturnError.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefJsonParserError(cef_sys::cef_json_parser_error_t);
impl CefJsonParserError {
  pub const NO_ERROR: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_NO_ERROR);
  pub const INVALID_ESCAPE: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_INVALID_ESCAPE);
  pub const SYNTAX_ERROR: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_SYNTAX_ERROR);
  pub const UNEXPECTED_TOKEN: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_UNEXPECTED_TOKEN);
  pub const TRAILING_COMMA: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_TRAILING_COMMA);
  pub const TOO_MUCH_NESTING: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_TOO_MUCH_NESTING);
  pub const UNEXPECTED_DATA_AFTER_ROOT: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_UNEXPECTED_DATA_AFTER_ROOT);
  pub const UNSUPPORTED_ENCODING: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_UNSUPPORTED_ENCODING);
  pub const UNQUOTED_DICTIONARY_KEY: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_UNQUOTED_DICTIONARY_KEY);
  pub const PARSE_ERROR_COUNT: CefJsonParserError = CefJsonParserError(cef_sys::cef_json_parser_error_t_JSON_PARSE_ERROR_COUNT);
}
impl From<cef_sys::cef_json_parser_error_t> for CefJsonParserError {
  fn from(raw: cef_sys::cef_json_parser_error_t) -> CefJsonParserError {
    CefJsonParserError(raw)
  }
}
impl From<CefJsonParserError> for cef_sys::cef_json_parser_error_t {
  fn from(src: CefJsonParserError) -> cef_sys::cef_json_parser_error_t {
    src.0
  }
}
impl std::ops::BitOr for CefJsonParserError {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Options that can be passed to CefWriteJSON.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefJsonWriterOptions(cef_sys::cef_json_writer_options_t);
impl CefJsonWriterOptions {
  /// Default behavior.
  pub const DEFAULT: CefJsonWriterOptions = CefJsonWriterOptions(cef_sys::cef_json_writer_options_t_JSON_WRITER_DEFAULT);
  /// This option instructs the writer that if a Binary value is encountered,
  /// the value (and key if within a dictionary) will be omitted from the
  /// output, and success will be returned. Otherwise, if a binary value is
  /// encountered, failure will be returned.
  pub const OMIT_BINARY_VALUES: CefJsonWriterOptions = CefJsonWriterOptions(cef_sys::cef_json_writer_options_t_JSON_WRITER_OMIT_BINARY_VALUES);
  /// This option instructs the writer to write doubles that have no fractional
  /// part as a normal integer (i.e., without using exponential notation
  /// or appending a '.0') as long as the value is within the range of a
  /// 64-bit int.
  pub const OMIT_DOUBLE_TYPE_PRESERVATION: CefJsonWriterOptions = CefJsonWriterOptions(cef_sys::cef_json_writer_options_t_JSON_WRITER_OMIT_DOUBLE_TYPE_PRESERVATION);
  /// Return a slightly nicer formatted json string (pads with whitespace to
  /// help with readability).
  pub const PRETTY_PRINT: CefJsonWriterOptions = CefJsonWriterOptions(cef_sys::cef_json_writer_options_t_JSON_WRITER_PRETTY_PRINT);
}
impl From<cef_sys::cef_json_writer_options_t> for CefJsonWriterOptions {
  fn from(raw: cef_sys::cef_json_writer_options_t) -> CefJsonWriterOptions {
    CefJsonWriterOptions(raw)
  }
}
impl From<CefJsonWriterOptions> for cef_sys::cef_json_writer_options_t {
  fn from(src: CefJsonWriterOptions) -> cef_sys::cef_json_writer_options_t {
    src.0
  }
}
impl std::ops::BitOr for CefJsonWriterOptions {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Margin type for PDF printing.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefPdfPrintMarginType(cef_sys::cef_pdf_print_margin_type_t);
impl CefPdfPrintMarginType {
  /// Default margins.
  pub const DEFAULT: CefPdfPrintMarginType = CefPdfPrintMarginType(cef_sys::cef_pdf_print_margin_type_t_PDF_PRINT_MARGIN_DEFAULT);
  /// No margins.
  pub const NONE: CefPdfPrintMarginType = CefPdfPrintMarginType(cef_sys::cef_pdf_print_margin_type_t_PDF_PRINT_MARGIN_NONE);
  /// Minimum margins.
  pub const MINIMUM: CefPdfPrintMarginType = CefPdfPrintMarginType(cef_sys::cef_pdf_print_margin_type_t_PDF_PRINT_MARGIN_MINIMUM);
  /// Custom margins using the |margin_*| values from cef_pdf_print_settings_t.
  pub const CUSTOM: CefPdfPrintMarginType = CefPdfPrintMarginType(cef_sys::cef_pdf_print_margin_type_t_PDF_PRINT_MARGIN_CUSTOM);
}
impl From<cef_sys::cef_pdf_print_margin_type_t> for CefPdfPrintMarginType {
  fn from(raw: cef_sys::cef_pdf_print_margin_type_t) -> CefPdfPrintMarginType {
    CefPdfPrintMarginType(raw)
  }
}
impl From<CefPdfPrintMarginType> for cef_sys::cef_pdf_print_margin_type_t {
  fn from(src: CefPdfPrintMarginType) -> cef_sys::cef_pdf_print_margin_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefPdfPrintMarginType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Structure representing PDF print settings.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefPdfPrintSettings { pub raw: cef_sys::cef_pdf_print_settings_t }
impl Default for CefPdfPrintSettings {
  fn default() -> CefPdfPrintSettings {
    let /* mut */ raw = cef_sys::cef_pdf_print_settings_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_pdf_print_settings_t>() as u64;
    CefPdfPrintSettings { raw }
  }
}
impl From<CefPdfPrintSettings> for cef_sys::cef_pdf_print_settings_t {
  fn from(src: CefPdfPrintSettings) -> cef_sys::cef_pdf_print_settings_t {
    src.raw
  }
}
impl From<cef_sys::cef_pdf_print_settings_t> for CefPdfPrintSettings {
  fn from(raw: cef_sys::cef_pdf_print_settings_t) -> CefPdfPrintSettings {
    CefPdfPrintSettings { raw }
  }
}
#[allow(non_snake_case)]
impl CefPdfPrintSettings {
  /// Page title to display in the header. Only used if |header_footer_enabled|
  /// is set to true (1).
  pub fn header_footer_title(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.header_footer_title) }
  /// Page title to display in the header. Only used if |header_footer_enabled|
  /// is set to true (1).
  pub fn set_header_footer_title(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.header_footer_title, v); self }
  /// URL to display in the footer. Only used if |header_footer_enabled| is set
  /// to true (1).
  pub fn header_footer_url(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.header_footer_url) }
  /// URL to display in the footer. Only used if |header_footer_enabled| is set
  /// to true (1).
  pub fn set_header_footer_url(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.header_footer_url, v); self }
  /// Output page size in microns. If either of these values is less than or
  /// equal to zero then the default paper size (A4) will be used.
  pub fn page_width(&self) -> i32 { self.raw.page_width }
  /// Output page size in microns. If either of these values is less than or
  /// equal to zero then the default paper size (A4) will be used.
  pub fn set_page_width(&mut self, v: i32) -> &mut Self { self.raw.page_width = v; self }
  pub fn page_height(&self) -> i32 { self.raw.page_height }
  pub fn set_page_height(&mut self, v: i32) -> &mut Self { self.raw.page_height = v; self }
  /// The percentage to scale the PDF by before printing (e.g. 50 is 50%).
  /// If this value is less than or equal to zero the default value of 100
  /// will be used.
  pub fn scale_factor(&self) -> i32 { self.raw.scale_factor }
  /// The percentage to scale the PDF by before printing (e.g. 50 is 50%).
  /// If this value is less than or equal to zero the default value of 100
  /// will be used.
  pub fn set_scale_factor(&mut self, v: i32) -> &mut Self { self.raw.scale_factor = v; self }
  /// Margins in points. Only used if |margin_type| is set to
  /// PDF_PRINT_MARGIN_CUSTOM.
  pub fn margin_top(&self) -> i32 { self.raw.margin_top }
  /// Margins in points. Only used if |margin_type| is set to
  /// PDF_PRINT_MARGIN_CUSTOM.
  pub fn set_margin_top(&mut self, v: i32) -> &mut Self { self.raw.margin_top = v; self }
  pub fn margin_right(&self) -> i32 { self.raw.margin_right }
  pub fn set_margin_right(&mut self, v: i32) -> &mut Self { self.raw.margin_right = v; self }
  pub fn margin_bottom(&self) -> i32 { self.raw.margin_bottom }
  pub fn set_margin_bottom(&mut self, v: i32) -> &mut Self { self.raw.margin_bottom = v; self }
  pub fn margin_left(&self) -> i32 { self.raw.margin_left }
  pub fn set_margin_left(&mut self, v: i32) -> &mut Self { self.raw.margin_left = v; self }
  /// Margin type.
  pub fn margin_type(&self) -> &crate::include::internal::CefPdfPrintMarginType { unsafe { &*(self as *const _ as *const _) } }
  /// Margin type.
  pub fn set_margin_type(&mut self, v: crate::include::internal::CefPdfPrintMarginType) -> &mut Self { self.raw.margin_type = v.into(); self }
  /// Set to true (1) to print headers and footers or false (0) to not print
  /// headers and footers.
  pub fn header_footer_enabled(&self) -> i32 { self.raw.header_footer_enabled }
  /// Set to true (1) to print headers and footers or false (0) to not print
  /// headers and footers.
  pub fn set_header_footer_enabled(&mut self, v: i32) -> &mut Self { self.raw.header_footer_enabled = v; self }
  /// Set to true (1) to print the selection only or false (0) to print all.
  pub fn selection_only(&self) -> i32 { self.raw.selection_only }
  /// Set to true (1) to print the selection only or false (0) to print all.
  pub fn set_selection_only(&mut self, v: i32) -> &mut Self { self.raw.selection_only = v; self }
  /// Set to true (1) for landscape mode or false (0) for portrait mode.
  pub fn landscape(&self) -> i32 { self.raw.landscape }
  /// Set to true (1) for landscape mode or false (0) for portrait mode.
  pub fn set_landscape(&mut self, v: i32) -> &mut Self { self.raw.landscape = v; self }
  /// Set to true (1) to print background graphics or false (0) to not print
  /// background graphics.
  pub fn backgrounds_enabled(&self) -> i32 { self.raw.backgrounds_enabled }
  /// Set to true (1) to print background graphics or false (0) to not print
  /// background graphics.
  pub fn set_backgrounds_enabled(&mut self, v: i32) -> &mut Self { self.raw.backgrounds_enabled = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Supported UI scale factors for the platform. SCALE_FACTOR_NONE is used for
/// density independent resources such as string, html/js files or an image that
/// can be used for any scale factors (such as wallpapers).
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefScaleFactor(cef_sys::cef_scale_factor_t);
impl CefScaleFactor {
  pub const NONE: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_NONE);
  pub const X100P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_100P);
  pub const X125P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_125P);
  pub const X133P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_133P);
  pub const X140P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_140P);
  pub const X150P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_150P);
  pub const X180P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_180P);
  pub const X200P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_200P);
  pub const X250P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_250P);
  pub const X300P: CefScaleFactor = CefScaleFactor(cef_sys::cef_scale_factor_t_SCALE_FACTOR_300P);
}
impl From<cef_sys::cef_scale_factor_t> for CefScaleFactor {
  fn from(raw: cef_sys::cef_scale_factor_t) -> CefScaleFactor {
    CefScaleFactor(raw)
  }
}
impl From<CefScaleFactor> for cef_sys::cef_scale_factor_t {
  fn from(src: CefScaleFactor) -> cef_sys::cef_scale_factor_t {
    src.0
  }
}
impl std::ops::BitOr for CefScaleFactor {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Plugin policies supported by CefRequestContextHandler::OnBeforePluginLoad.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefPluginPolicy(cef_sys::cef_plugin_policy_t);
impl CefPluginPolicy {
  /// Allow the content.
  pub const ALLOW: CefPluginPolicy = CefPluginPolicy(cef_sys::cef_plugin_policy_t_PLUGIN_POLICY_ALLOW);
  /// Allow important content and block unimportant content based on heuristics.
  /// The user can manually load blocked content.
  pub const DETECT_IMPORTANT: CefPluginPolicy = CefPluginPolicy(cef_sys::cef_plugin_policy_t_PLUGIN_POLICY_DETECT_IMPORTANT);
  /// Block the content. The user can manually load blocked content.
  pub const BLOCK: CefPluginPolicy = CefPluginPolicy(cef_sys::cef_plugin_policy_t_PLUGIN_POLICY_BLOCK);
  /// Disable the content. The user cannot load disabled content.
  pub const DISABLE: CefPluginPolicy = CefPluginPolicy(cef_sys::cef_plugin_policy_t_PLUGIN_POLICY_DISABLE);
}
impl From<cef_sys::cef_plugin_policy_t> for CefPluginPolicy {
  fn from(raw: cef_sys::cef_plugin_policy_t) -> CefPluginPolicy {
    CefPluginPolicy(raw)
  }
}
impl From<CefPluginPolicy> for cef_sys::cef_plugin_policy_t {
  fn from(src: CefPluginPolicy) -> cef_sys::cef_plugin_policy_t {
    src.0
  }
}
impl std::ops::BitOr for CefPluginPolicy {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Policy for how the Referrer HTTP header value will be sent during navigation.
/// If the `--no-referrers` command-line flag is specified then the policy value
/// will be ignored and the Referrer value will never be sent.
/// Must be kept synchronized with net::URLRequest::ReferrerPolicy from Chromium.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefReferrerPolicy(cef_sys::cef_referrer_policy_t);
impl CefReferrerPolicy {
  /// Clear the referrer header if the header value is HTTPS but the request
  /// destination is HTTP. This is the default behavior.
  pub const CLEAR_REFERRER_ON_TRANSITION_FROM_SECURE_TO_INSECURE: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_CLEAR_REFERRER_ON_TRANSITION_FROM_SECURE_TO_INSECURE);
  pub const DEFAULT: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_DEFAULT);
  /// A slight variant on CLEAR_REFERRER_ON_TRANSITION_FROM_SECURE_TO_INSECURE:
  /// If the request destination is HTTP, an HTTPS referrer will be cleared. If
  /// the request's destination is cross-origin with the referrer (but does not
  /// downgrade), the referrer's granularity will be stripped down to an origin
  /// rather than a full URL. Same-origin requests will send the full referrer.
  pub const REDUCE_REFERRER_GRANULARITY_ON_TRANSITION_CROSS_ORIGIN: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_REDUCE_REFERRER_GRANULARITY_ON_TRANSITION_CROSS_ORIGIN);
  /// Strip the referrer down to an origin when the origin of the referrer is
  /// different from the destination's origin.
  pub const ORIGIN_ONLY_ON_TRANSITION_CROSS_ORIGIN: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_ORIGIN_ONLY_ON_TRANSITION_CROSS_ORIGIN);
  /// Never change the referrer.
  pub const NEVER_CLEAR_REFERRER: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_NEVER_CLEAR_REFERRER);
  /// Strip the referrer down to the origin regardless of the redirect location.
  pub const ORIGIN: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_ORIGIN);
  /// Clear the referrer when the request's referrer is cross-origin with the
  /// request's destination.
  pub const CLEAR_REFERRER_ON_TRANSITION_CROSS_ORIGIN: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_CLEAR_REFERRER_ON_TRANSITION_CROSS_ORIGIN);
  /// Strip the referrer down to the origin, but clear it entirely if the
  /// referrer value is HTTPS and the destination is HTTP.
  pub const ORIGIN_CLEAR_ON_TRANSITION_FROM_SECURE_TO_INSECURE: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_ORIGIN_CLEAR_ON_TRANSITION_FROM_SECURE_TO_INSECURE);
  /// Always clear the referrer regardless of the request destination.
  pub const NO_REFERRER: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_NO_REFERRER);
  /// Always the last value in this enumeration.
  pub const LAST_VALUE: CefReferrerPolicy = CefReferrerPolicy(cef_sys::cef_referrer_policy_t_REFERRER_POLICY_LAST_VALUE);
}
impl From<cef_sys::cef_referrer_policy_t> for CefReferrerPolicy {
  fn from(raw: cef_sys::cef_referrer_policy_t) -> CefReferrerPolicy {
    CefReferrerPolicy(raw)
  }
}
impl From<CefReferrerPolicy> for cef_sys::cef_referrer_policy_t {
  fn from(src: CefReferrerPolicy) -> cef_sys::cef_referrer_policy_t {
    src.0
  }
}
impl std::ops::BitOr for CefReferrerPolicy {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Return values for CefResponseFilter::Filter().
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefResponseFilterStatus(cef_sys::cef_response_filter_status_t);
impl CefResponseFilterStatus {
  /// Some or all of the pre-filter data was read successfully but more data is
  /// needed in order to continue filtering (filtered output is pending).
  pub const NEED_MORE_DATA: CefResponseFilterStatus = CefResponseFilterStatus(cef_sys::cef_response_filter_status_t_RESPONSE_FILTER_NEED_MORE_DATA);
  /// Some or all of the pre-filter data was read successfully and all available
  /// filtered output has been written.
  pub const DONE: CefResponseFilterStatus = CefResponseFilterStatus(cef_sys::cef_response_filter_status_t_RESPONSE_FILTER_DONE);
  /// An error occurred during filtering.
  pub const ERROR: CefResponseFilterStatus = CefResponseFilterStatus(cef_sys::cef_response_filter_status_t_RESPONSE_FILTER_ERROR);
}
impl From<cef_sys::cef_response_filter_status_t> for CefResponseFilterStatus {
  fn from(raw: cef_sys::cef_response_filter_status_t) -> CefResponseFilterStatus {
    CefResponseFilterStatus(raw)
  }
}
impl From<CefResponseFilterStatus> for cef_sys::cef_response_filter_status_t {
  fn from(src: CefResponseFilterStatus) -> cef_sys::cef_response_filter_status_t {
    src.0
  }
}
impl std::ops::BitOr for CefResponseFilterStatus {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Describes how to interpret the components of a pixel.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefColorType(cef_sys::cef_color_type_t);
impl CefColorType {
  /// RGBA with 8 bits per pixel (32bits total).
  pub const RGBA_8888: CefColorType = CefColorType(cef_sys::cef_color_type_t_CEF_COLOR_TYPE_RGBA_8888);
  /// BGRA with 8 bits per pixel (32bits total).
  pub const BGRA_8888: CefColorType = CefColorType(cef_sys::cef_color_type_t_CEF_COLOR_TYPE_BGRA_8888);
}
impl From<cef_sys::cef_color_type_t> for CefColorType {
  fn from(raw: cef_sys::cef_color_type_t) -> CefColorType {
    CefColorType(raw)
  }
}
impl From<CefColorType> for cef_sys::cef_color_type_t {
  fn from(src: CefColorType) -> cef_sys::cef_color_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefColorType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Describes how to interpret the alpha component of a pixel.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefAlphaType(cef_sys::cef_alpha_type_t);
impl CefAlphaType {
  /// No transparency. The alpha component is ignored.
  pub const OPAQUE: CefAlphaType = CefAlphaType(cef_sys::cef_alpha_type_t_CEF_ALPHA_TYPE_OPAQUE);
  /// Transparency with pre-multiplied alpha component.
  pub const PREMULTIPLIED: CefAlphaType = CefAlphaType(cef_sys::cef_alpha_type_t_CEF_ALPHA_TYPE_PREMULTIPLIED);
  /// Transparency with post-multiplied alpha component.
  pub const POSTMULTIPLIED: CefAlphaType = CefAlphaType(cef_sys::cef_alpha_type_t_CEF_ALPHA_TYPE_POSTMULTIPLIED);
}
impl From<cef_sys::cef_alpha_type_t> for CefAlphaType {
  fn from(raw: cef_sys::cef_alpha_type_t) -> CefAlphaType {
    CefAlphaType(raw)
  }
}
impl From<CefAlphaType> for cef_sys::cef_alpha_type_t {
  fn from(src: CefAlphaType) -> cef_sys::cef_alpha_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefAlphaType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Text style types. Should be kepy in sync with gfx::TextStyle.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefTextStyle(cef_sys::cef_text_style_t);
impl CefTextStyle {
  pub const BOLD: CefTextStyle = CefTextStyle(cef_sys::cef_text_style_t_CEF_TEXT_STYLE_BOLD);
  pub const ITALIC: CefTextStyle = CefTextStyle(cef_sys::cef_text_style_t_CEF_TEXT_STYLE_ITALIC);
  pub const STRIKE: CefTextStyle = CefTextStyle(cef_sys::cef_text_style_t_CEF_TEXT_STYLE_STRIKE);
  pub const DIAGONAL_STRIKE: CefTextStyle = CefTextStyle(cef_sys::cef_text_style_t_CEF_TEXT_STYLE_DIAGONAL_STRIKE);
  pub const UNDERLINE: CefTextStyle = CefTextStyle(cef_sys::cef_text_style_t_CEF_TEXT_STYLE_UNDERLINE);
}
impl From<cef_sys::cef_text_style_t> for CefTextStyle {
  fn from(raw: cef_sys::cef_text_style_t) -> CefTextStyle {
    CefTextStyle(raw)
  }
}
impl From<CefTextStyle> for cef_sys::cef_text_style_t {
  fn from(src: CefTextStyle) -> cef_sys::cef_text_style_t {
    src.0
  }
}
impl std::ops::BitOr for CefTextStyle {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Specifies where along the main axis the CefBoxLayout child views should be
/// laid out.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMainAxisAlignment(cef_sys::cef_main_axis_alignment_t);
impl CefMainAxisAlignment {
  /// Child views will be left-aligned.
  pub const START: CefMainAxisAlignment = CefMainAxisAlignment(cef_sys::cef_main_axis_alignment_t_CEF_MAIN_AXIS_ALIGNMENT_START);
  /// Child views will be center-aligned.
  pub const CENTER: CefMainAxisAlignment = CefMainAxisAlignment(cef_sys::cef_main_axis_alignment_t_CEF_MAIN_AXIS_ALIGNMENT_CENTER);
  /// Child views will be right-aligned.
  pub const END: CefMainAxisAlignment = CefMainAxisAlignment(cef_sys::cef_main_axis_alignment_t_CEF_MAIN_AXIS_ALIGNMENT_END);
}
impl From<cef_sys::cef_main_axis_alignment_t> for CefMainAxisAlignment {
  fn from(raw: cef_sys::cef_main_axis_alignment_t) -> CefMainAxisAlignment {
    CefMainAxisAlignment(raw)
  }
}
impl From<CefMainAxisAlignment> for cef_sys::cef_main_axis_alignment_t {
  fn from(src: CefMainAxisAlignment) -> cef_sys::cef_main_axis_alignment_t {
    src.0
  }
}
impl std::ops::BitOr for CefMainAxisAlignment {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Specifies where along the cross axis the CefBoxLayout child views should be
/// laid out.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefCrossAxisAlignment(cef_sys::cef_cross_axis_alignment_t);
impl CefCrossAxisAlignment {
  /// Child views will be stretched to fit.
  pub const STRETCH: CefCrossAxisAlignment = CefCrossAxisAlignment(cef_sys::cef_cross_axis_alignment_t_CEF_CROSS_AXIS_ALIGNMENT_STRETCH);
  /// Child views will be left-aligned.
  pub const START: CefCrossAxisAlignment = CefCrossAxisAlignment(cef_sys::cef_cross_axis_alignment_t_CEF_CROSS_AXIS_ALIGNMENT_START);
  /// Child views will be center-aligned.
  pub const CENTER: CefCrossAxisAlignment = CefCrossAxisAlignment(cef_sys::cef_cross_axis_alignment_t_CEF_CROSS_AXIS_ALIGNMENT_CENTER);
  /// Child views will be right-aligned.
  pub const END: CefCrossAxisAlignment = CefCrossAxisAlignment(cef_sys::cef_cross_axis_alignment_t_CEF_CROSS_AXIS_ALIGNMENT_END);
}
impl From<cef_sys::cef_cross_axis_alignment_t> for CefCrossAxisAlignment {
  fn from(raw: cef_sys::cef_cross_axis_alignment_t) -> CefCrossAxisAlignment {
    CefCrossAxisAlignment(raw)
  }
}
impl From<CefCrossAxisAlignment> for cef_sys::cef_cross_axis_alignment_t {
  fn from(src: CefCrossAxisAlignment) -> cef_sys::cef_cross_axis_alignment_t {
    src.0
  }
}
impl std::ops::BitOr for CefCrossAxisAlignment {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Settings used when initializing a CefBoxLayout.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefBoxLayoutSettings { pub raw: cef_sys::cef_box_layout_settings_t }
impl Default for CefBoxLayoutSettings {
  fn default() -> CefBoxLayoutSettings {
    let /* mut */ raw = cef_sys::cef_box_layout_settings_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_box_layout_settings_t>() as u64;
    CefBoxLayoutSettings { raw }
  }
}
impl From<CefBoxLayoutSettings> for cef_sys::cef_box_layout_settings_t {
  fn from(src: CefBoxLayoutSettings) -> cef_sys::cef_box_layout_settings_t {
    src.raw
  }
}
impl From<cef_sys::cef_box_layout_settings_t> for CefBoxLayoutSettings {
  fn from(raw: cef_sys::cef_box_layout_settings_t) -> CefBoxLayoutSettings {
    CefBoxLayoutSettings { raw }
  }
}
#[allow(non_snake_case)]
impl CefBoxLayoutSettings {
  /// If true (1) the layout will be horizontal, otherwise the layout will be
  /// vertical.
  pub fn horizontal(&self) -> i32 { self.raw.horizontal }
  /// If true (1) the layout will be horizontal, otherwise the layout will be
  /// vertical.
  pub fn set_horizontal(&mut self, v: i32) -> &mut Self { self.raw.horizontal = v; self }
  /// Adds additional horizontal space between the child view area and the host
  /// view border.
  pub fn inside_border_horizontal_spacing(&self) -> i32 { self.raw.inside_border_horizontal_spacing }
  /// Adds additional horizontal space between the child view area and the host
  /// view border.
  pub fn set_inside_border_horizontal_spacing(&mut self, v: i32) -> &mut Self { self.raw.inside_border_horizontal_spacing = v; self }
  /// Adds additional vertical space between the child view area and the host
  /// view border.
  pub fn inside_border_vertical_spacing(&self) -> i32 { self.raw.inside_border_vertical_spacing }
  /// Adds additional vertical space between the child view area and the host
  /// view border.
  pub fn set_inside_border_vertical_spacing(&mut self, v: i32) -> &mut Self { self.raw.inside_border_vertical_spacing = v; self }
  /// Adds additional space around the child view area.
  pub fn inside_border_insets(&self) -> &crate::include::internal::CefInsets { unsafe { &*(self as *const _ as *const _) } }
  /// Adds additional space around the child view area.
  pub fn set_inside_border_insets(&mut self, v: crate::include::internal::CefInsets) -> &mut Self { self.raw.inside_border_insets = v.into(); self }
  /// Adds additional space between child views.
  pub fn between_child_spacing(&self) -> i32 { self.raw.between_child_spacing }
  /// Adds additional space between child views.
  pub fn set_between_child_spacing(&mut self, v: i32) -> &mut Self { self.raw.between_child_spacing = v; self }
  /// Specifies where along the main axis the child views should be laid out.
  pub fn main_axis_alignment(&self) -> &crate::include::internal::CefMainAxisAlignment { unsafe { &*(self as *const _ as *const _) } }
  /// Specifies where along the main axis the child views should be laid out.
  pub fn set_main_axis_alignment(&mut self, v: crate::include::internal::CefMainAxisAlignment) -> &mut Self { self.raw.main_axis_alignment = v.into(); self }
  /// Specifies where along the cross axis the child views should be laid out.
  pub fn cross_axis_alignment(&self) -> &crate::include::internal::CefCrossAxisAlignment { unsafe { &*(self as *const _ as *const _) } }
  /// Specifies where along the cross axis the child views should be laid out.
  pub fn set_cross_axis_alignment(&mut self, v: crate::include::internal::CefCrossAxisAlignment) -> &mut Self { self.raw.cross_axis_alignment = v.into(); self }
  /// Minimum cross axis size.
  pub fn minimum_cross_axis_size(&self) -> i32 { self.raw.minimum_cross_axis_size }
  /// Minimum cross axis size.
  pub fn set_minimum_cross_axis_size(&mut self, v: i32) -> &mut Self { self.raw.minimum_cross_axis_size = v; self }
  /// Default flex for views when none is specified via CefBoxLayout methods.
  /// Using the preferred size as the basis, free space along the main axis is
  /// distributed to views in the ratio of their flex weights. Similarly, if the
  /// views will overflow the parent, space is subtracted in these ratios. A flex
  /// of 0 means this view is not resized. Flex values must not be negative.
  pub fn default_flex(&self) -> i32 { self.raw.default_flex }
  /// Default flex for views when none is specified via CefBoxLayout methods.
  /// Using the preferred size as the basis, free space along the main axis is
  /// distributed to views in the ratio of their flex weights. Similarly, if the
  /// views will overflow the parent, space is subtracted in these ratios. A flex
  /// of 0 means this view is not resized. Flex values must not be negative.
  pub fn set_default_flex(&mut self, v: i32) -> &mut Self { self.raw.default_flex = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Specifies the button display state.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefButtonState(cef_sys::cef_button_state_t);
impl CefButtonState {
  pub const NORMAL: CefButtonState = CefButtonState(cef_sys::cef_button_state_t_CEF_BUTTON_STATE_NORMAL);
  pub const HOVERED: CefButtonState = CefButtonState(cef_sys::cef_button_state_t_CEF_BUTTON_STATE_HOVERED);
  pub const PRESSED: CefButtonState = CefButtonState(cef_sys::cef_button_state_t_CEF_BUTTON_STATE_PRESSED);
  pub const DISABLED: CefButtonState = CefButtonState(cef_sys::cef_button_state_t_CEF_BUTTON_STATE_DISABLED);
}
impl From<cef_sys::cef_button_state_t> for CefButtonState {
  fn from(raw: cef_sys::cef_button_state_t) -> CefButtonState {
    CefButtonState(raw)
  }
}
impl From<CefButtonState> for cef_sys::cef_button_state_t {
  fn from(src: CefButtonState) -> cef_sys::cef_button_state_t {
    src.0
  }
}
impl std::ops::BitOr for CefButtonState {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Specifies the horizontal text alignment mode.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefHorizontalAlignment(cef_sys::cef_horizontal_alignment_t);
impl CefHorizontalAlignment {
  /// Align the text's left edge with that of its display area.
  pub const LEFT: CefHorizontalAlignment = CefHorizontalAlignment(cef_sys::cef_horizontal_alignment_t_CEF_HORIZONTAL_ALIGNMENT_LEFT);
  /// Align the text's center with that of its display area.
  pub const CENTER: CefHorizontalAlignment = CefHorizontalAlignment(cef_sys::cef_horizontal_alignment_t_CEF_HORIZONTAL_ALIGNMENT_CENTER);
  /// Align the text's right edge with that of its display area.
  pub const RIGHT: CefHorizontalAlignment = CefHorizontalAlignment(cef_sys::cef_horizontal_alignment_t_CEF_HORIZONTAL_ALIGNMENT_RIGHT);
}
impl From<cef_sys::cef_horizontal_alignment_t> for CefHorizontalAlignment {
  fn from(raw: cef_sys::cef_horizontal_alignment_t) -> CefHorizontalAlignment {
    CefHorizontalAlignment(raw)
  }
}
impl From<CefHorizontalAlignment> for cef_sys::cef_horizontal_alignment_t {
  fn from(src: CefHorizontalAlignment) -> cef_sys::cef_horizontal_alignment_t {
    src.0
  }
}
impl std::ops::BitOr for CefHorizontalAlignment {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Specifies how a menu will be anchored for non-RTL languages. The opposite
/// position will be used for RTL languages.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMenuAnchorPosition(cef_sys::cef_menu_anchor_position_t);
impl CefMenuAnchorPosition {
  pub const TOPLEFT: CefMenuAnchorPosition = CefMenuAnchorPosition(cef_sys::cef_menu_anchor_position_t_CEF_MENU_ANCHOR_TOPLEFT);
  pub const TOPRIGHT: CefMenuAnchorPosition = CefMenuAnchorPosition(cef_sys::cef_menu_anchor_position_t_CEF_MENU_ANCHOR_TOPRIGHT);
  pub const BOTTOMCENTER: CefMenuAnchorPosition = CefMenuAnchorPosition(cef_sys::cef_menu_anchor_position_t_CEF_MENU_ANCHOR_BOTTOMCENTER);
}
impl From<cef_sys::cef_menu_anchor_position_t> for CefMenuAnchorPosition {
  fn from(raw: cef_sys::cef_menu_anchor_position_t) -> CefMenuAnchorPosition {
    CefMenuAnchorPosition(raw)
  }
}
impl From<CefMenuAnchorPosition> for cef_sys::cef_menu_anchor_position_t {
  fn from(src: CefMenuAnchorPosition) -> cef_sys::cef_menu_anchor_position_t {
    src.0
  }
}
impl std::ops::BitOr for CefMenuAnchorPosition {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported color types for menu items.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMenuColorType(cef_sys::cef_menu_color_type_t);
impl CefMenuColorType {
  pub const TEXT: CefMenuColorType = CefMenuColorType(cef_sys::cef_menu_color_type_t_CEF_MENU_COLOR_TEXT);
  pub const TEXT_HOVERED: CefMenuColorType = CefMenuColorType(cef_sys::cef_menu_color_type_t_CEF_MENU_COLOR_TEXT_HOVERED);
  pub const TEXT_ACCELERATOR: CefMenuColorType = CefMenuColorType(cef_sys::cef_menu_color_type_t_CEF_MENU_COLOR_TEXT_ACCELERATOR);
  pub const TEXT_ACCELERATOR_HOVERED: CefMenuColorType = CefMenuColorType(cef_sys::cef_menu_color_type_t_CEF_MENU_COLOR_TEXT_ACCELERATOR_HOVERED);
  pub const BACKGROUND: CefMenuColorType = CefMenuColorType(cef_sys::cef_menu_color_type_t_CEF_MENU_COLOR_BACKGROUND);
  pub const BACKGROUND_HOVERED: CefMenuColorType = CefMenuColorType(cef_sys::cef_menu_color_type_t_CEF_MENU_COLOR_BACKGROUND_HOVERED);
  pub const COUNT: CefMenuColorType = CefMenuColorType(cef_sys::cef_menu_color_type_t_CEF_MENU_COLOR_COUNT);
}
impl From<cef_sys::cef_menu_color_type_t> for CefMenuColorType {
  fn from(raw: cef_sys::cef_menu_color_type_t) -> CefMenuColorType {
    CefMenuColorType(raw)
  }
}
impl From<CefMenuColorType> for cef_sys::cef_menu_color_type_t {
  fn from(src: CefMenuColorType) -> cef_sys::cef_menu_color_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefMenuColorType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported SSL version values. See net/ssl/ssl_connection_status_flags.h
/// for more information.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefSslVersion(cef_sys::cef_ssl_version_t);
impl CefSslVersion {
  pub const UNKNOWN: CefSslVersion = CefSslVersion(cef_sys::cef_ssl_version_t_SSL_CONNECTION_VERSION_UNKNOWN);
  /// Unknown SSL version.
  pub const SSL2: CefSslVersion = CefSslVersion(cef_sys::cef_ssl_version_t_SSL_CONNECTION_VERSION_SSL2);
  pub const SSL3: CefSslVersion = CefSslVersion(cef_sys::cef_ssl_version_t_SSL_CONNECTION_VERSION_SSL3);
  pub const TLS1: CefSslVersion = CefSslVersion(cef_sys::cef_ssl_version_t_SSL_CONNECTION_VERSION_TLS1);
  pub const TLS1_1: CefSslVersion = CefSslVersion(cef_sys::cef_ssl_version_t_SSL_CONNECTION_VERSION_TLS1_1);
  pub const TLS1_2: CefSslVersion = CefSslVersion(cef_sys::cef_ssl_version_t_SSL_CONNECTION_VERSION_TLS1_2);
  pub const TLS1_3: CefSslVersion = CefSslVersion(cef_sys::cef_ssl_version_t_SSL_CONNECTION_VERSION_TLS1_3);
  pub const QUIC: CefSslVersion = CefSslVersion(cef_sys::cef_ssl_version_t_SSL_CONNECTION_VERSION_QUIC);
}
impl From<cef_sys::cef_ssl_version_t> for CefSslVersion {
  fn from(raw: cef_sys::cef_ssl_version_t) -> CefSslVersion {
    CefSslVersion(raw)
  }
}
impl From<CefSslVersion> for cef_sys::cef_ssl_version_t {
  fn from(src: CefSslVersion) -> cef_sys::cef_ssl_version_t {
    src.0
  }
}
impl std::ops::BitOr for CefSslVersion {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Supported SSL content status flags. See content/public/common/ssl_status.h
/// for more information.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefSslContentStatus(cef_sys::cef_ssl_content_status_t);
impl CefSslContentStatus {
  pub const NORMAL_CONTENT: CefSslContentStatus = CefSslContentStatus(cef_sys::cef_ssl_content_status_t_SSL_CONTENT_NORMAL_CONTENT);
  pub const DISPLAYED_INSECURE_CONTENT: CefSslContentStatus = CefSslContentStatus(cef_sys::cef_ssl_content_status_t_SSL_CONTENT_DISPLAYED_INSECURE_CONTENT);
  pub const RAN_INSECURE_CONTENT: CefSslContentStatus = CefSslContentStatus(cef_sys::cef_ssl_content_status_t_SSL_CONTENT_RAN_INSECURE_CONTENT);
}
impl From<cef_sys::cef_ssl_content_status_t> for CefSslContentStatus {
  fn from(raw: cef_sys::cef_ssl_content_status_t) -> CefSslContentStatus {
    CefSslContentStatus(raw)
  }
}
impl From<CefSslContentStatus> for cef_sys::cef_ssl_content_status_t {
  fn from(src: CefSslContentStatus) -> cef_sys::cef_ssl_content_status_t {
    src.0
  }
}
impl std::ops::BitOr for CefSslContentStatus {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// 
/// Configuration options for registering a custom scheme.
/// These values are used when calling AddCustomScheme.
/// 
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefSchemeOptions(cef_sys::cef_scheme_options_t);
impl CefSchemeOptions {
  pub const NONE: CefSchemeOptions = CefSchemeOptions(cef_sys::cef_scheme_options_t_CEF_SCHEME_OPTION_NONE);
  /// If CEF_SCHEME_OPTION_STANDARD is set the scheme will be treated as a
  /// standard scheme. Standard schemes are subject to URL canonicalization and
  /// parsing rules as defined in the Common Internet Scheme Syntax RFC 1738
  /// Section 3.1 available at http://www.ietf.org/rfc/rfc1738.txt
  /// 
  /// In particular, the syntax for standard scheme URLs must be of the form:
  /// <pre>
  /// [scheme]://[username]:[password]@[host]:[port]/[url-path]
  /// </pre> Standard scheme URLs must have a host component that is a fully
  /// qualified domain name as defined in Section 3.5 of RFC 1034 [13] and
  /// Section 2.1 of RFC 1123. These URLs will be canonicalized to
  /// "scheme://host/path" in the simplest case and
  /// "scheme://username:password@host:port/path" in the most explicit case. For
  /// example, "scheme:host/path" and "scheme:///host/path" will both be
  /// canonicalized to "scheme://host/path". The origin of a standard scheme URL
  /// is the combination of scheme, host and port (i.e., "scheme://host:port" in
  /// the most explicit case).
  /// 
  /// For non-standard scheme URLs only the "scheme:" component is parsed and
  /// canonicalized. The remainder of the URL will be passed to the handler as-
  /// is. For example, "scheme:///some%20text" will remain the same. Non-standard
  /// scheme URLs cannot be used as a target for form submission.
  pub const STANDARD: CefSchemeOptions = CefSchemeOptions(cef_sys::cef_scheme_options_t_CEF_SCHEME_OPTION_STANDARD);
  /// If CEF_SCHEME_OPTION_LOCAL is set the scheme will be treated with the same
  /// security rules as those applied to "file" URLs. Normal pages cannot link to
  /// or access local URLs. Also, by default, local URLs can only perform
  /// XMLHttpRequest calls to the same URL (origin + path) that originated the
  /// request. To allow XMLHttpRequest calls from a local URL to other URLs with
  /// the same origin set the CefSettings.file_access_from_file_urls_allowed
  /// value to true (1). To allow XMLHttpRequest calls from a local URL to all
  /// origins set the CefSettings.universal_access_from_file_urls_allowed value
  /// to true (1).
  pub const LOCAL: CefSchemeOptions = CefSchemeOptions(cef_sys::cef_scheme_options_t_CEF_SCHEME_OPTION_LOCAL);
  /// If CEF_SCHEME_OPTION_DISPLAY_ISOLATED is set the scheme can only be
  /// displayed from other content hosted with the same scheme. For example,
  /// pages in other origins cannot create iframes or hyperlinks to URLs with the
  /// scheme. For schemes that must be accessible from other schemes don't set
  /// this, set CEF_SCHEME_OPTION_CORS_ENABLED, and use CORS
  /// "Access-Control-Allow-Origin" headers to further restrict access.
  pub const DISPLAY_ISOLATED: CefSchemeOptions = CefSchemeOptions(cef_sys::cef_scheme_options_t_CEF_SCHEME_OPTION_DISPLAY_ISOLATED);
  /// If CEF_SCHEME_OPTION_SECURE is set the scheme will be treated with the same
  /// security rules as those applied to "https" URLs. For example, loading this
  /// scheme from other secure schemes will not trigger mixed content warnings.
  pub const SECURE: CefSchemeOptions = CefSchemeOptions(cef_sys::cef_scheme_options_t_CEF_SCHEME_OPTION_SECURE);
  /// If CEF_SCHEME_OPTION_CORS_ENABLED is set the scheme can be sent CORS
  /// requests. This value should be set in most cases where
  /// CEF_SCHEME_OPTION_STANDARD is set.
  pub const CORS_ENABLED: CefSchemeOptions = CefSchemeOptions(cef_sys::cef_scheme_options_t_CEF_SCHEME_OPTION_CORS_ENABLED);
  /// If CEF_SCHEME_OPTION_CSP_BYPASSING is set the scheme can bypass Content-
  /// Security-Policy (CSP) checks. This value should not be set in most cases
  /// where CEF_SCHEME_OPTION_STANDARD is set.
  pub const CSP_BYPASSING: CefSchemeOptions = CefSchemeOptions(cef_sys::cef_scheme_options_t_CEF_SCHEME_OPTION_CSP_BYPASSING);
  /// If CEF_SCHEME_OPTION_FETCH_ENABLED is set the scheme can perform Fetch API
  /// requests.
  pub const FETCH_ENABLED: CefSchemeOptions = CefSchemeOptions(cef_sys::cef_scheme_options_t_CEF_SCHEME_OPTION_FETCH_ENABLED);
}
impl From<cef_sys::cef_scheme_options_t> for CefSchemeOptions {
  fn from(raw: cef_sys::cef_scheme_options_t) -> CefSchemeOptions {
    CefSchemeOptions(raw)
  }
}
impl From<CefSchemeOptions> for cef_sys::cef_scheme_options_t {
  fn from(src: CefSchemeOptions) -> cef_sys::cef_scheme_options_t {
    src.0
  }
}
impl std::ops::BitOr for CefSchemeOptions {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Error codes for CDM registration. See cef_web_plugin.h for details.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefCdmRegistrationError(cef_sys::cef_cdm_registration_error_t);
impl CefCdmRegistrationError {
  /// No error. Registration completed successfully.
  pub const NONE: CefCdmRegistrationError = CefCdmRegistrationError(cef_sys::cef_cdm_registration_error_t_CEF_CDM_REGISTRATION_ERROR_NONE);
  /// Required files or manifest contents are missing.
  pub const INCORRECT_CONTENTS: CefCdmRegistrationError = CefCdmRegistrationError(cef_sys::cef_cdm_registration_error_t_CEF_CDM_REGISTRATION_ERROR_INCORRECT_CONTENTS);
  /// The CDM is incompatible with the current Chromium version.
  pub const INCOMPATIBLE: CefCdmRegistrationError = CefCdmRegistrationError(cef_sys::cef_cdm_registration_error_t_CEF_CDM_REGISTRATION_ERROR_INCOMPATIBLE);
  /// CDM registration is not supported at this time.
  pub const NOT_SUPPORTED: CefCdmRegistrationError = CefCdmRegistrationError(cef_sys::cef_cdm_registration_error_t_CEF_CDM_REGISTRATION_ERROR_NOT_SUPPORTED);
}
impl From<cef_sys::cef_cdm_registration_error_t> for CefCdmRegistrationError {
  fn from(raw: cef_sys::cef_cdm_registration_error_t) -> CefCdmRegistrationError {
    CefCdmRegistrationError(raw)
  }
}
impl From<CefCdmRegistrationError> for cef_sys::cef_cdm_registration_error_t {
  fn from(src: CefCdmRegistrationError) -> cef_sys::cef_cdm_registration_error_t {
    src.0
  }
}
impl std::ops::BitOr for CefCdmRegistrationError {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Composition underline style.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefCompositionUnderlineStyle(cef_sys::cef_composition_underline_style_t);
impl CefCompositionUnderlineStyle {
  pub const SOLID: CefCompositionUnderlineStyle = CefCompositionUnderlineStyle(cef_sys::cef_composition_underline_style_t_CEF_CUS_SOLID);
  pub const DOT: CefCompositionUnderlineStyle = CefCompositionUnderlineStyle(cef_sys::cef_composition_underline_style_t_CEF_CUS_DOT);
  pub const DASH: CefCompositionUnderlineStyle = CefCompositionUnderlineStyle(cef_sys::cef_composition_underline_style_t_CEF_CUS_DASH);
  pub const NONE: CefCompositionUnderlineStyle = CefCompositionUnderlineStyle(cef_sys::cef_composition_underline_style_t_CEF_CUS_NONE);
}
impl From<cef_sys::cef_composition_underline_style_t> for CefCompositionUnderlineStyle {
  fn from(raw: cef_sys::cef_composition_underline_style_t) -> CefCompositionUnderlineStyle {
    CefCompositionUnderlineStyle(raw)
  }
}
impl From<CefCompositionUnderlineStyle> for cef_sys::cef_composition_underline_style_t {
  fn from(src: CefCompositionUnderlineStyle) -> cef_sys::cef_composition_underline_style_t {
    src.0
  }
}
impl std::ops::BitOr for CefCompositionUnderlineStyle {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Structure representing IME composition underline information. This is a thin
/// wrapper around Blink's WebCompositionUnderline class and should be kept in
/// sync with that.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefCompositionUnderline { pub raw: cef_sys::cef_composition_underline_t }
impl Default for CefCompositionUnderline {
  fn default() -> CefCompositionUnderline {
    let /* mut */ raw = cef_sys::cef_composition_underline_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_composition_underline_t>() as u64;
    CefCompositionUnderline { raw }
  }
}
impl From<CefCompositionUnderline> for cef_sys::cef_composition_underline_t {
  fn from(src: CefCompositionUnderline) -> cef_sys::cef_composition_underline_t {
    src.raw
  }
}
impl From<cef_sys::cef_composition_underline_t> for CefCompositionUnderline {
  fn from(raw: cef_sys::cef_composition_underline_t) -> CefCompositionUnderline {
    CefCompositionUnderline { raw }
  }
}
#[allow(non_snake_case)]
impl CefCompositionUnderline {
  /// Underline character range.
  pub fn range(&self) -> &crate::include::internal::CefRange { unsafe { &*(self as *const _ as *const _) } }
  /// Underline character range.
  pub fn set_range(&mut self, v: crate::include::internal::CefRange) -> &mut Self { self.raw.range = v.into(); self }
  /// Text color.
  pub fn color(&self) -> &crate::include::internal::CefColor { unsafe { &*(self as *const _ as *const _) } }
  /// Text color.
  pub fn set_color(&mut self, v: crate::include::internal::CefColor) -> &mut Self { self.raw.color = v.into(); self }
  /// Background color.
  pub fn background_color(&self) -> &crate::include::internal::CefColor { unsafe { &*(self as *const _ as *const _) } }
  /// Background color.
  pub fn set_background_color(&mut self, v: crate::include::internal::CefColor) -> &mut Self { self.raw.background_color = v.into(); self }
  /// Set to true (1) for thick underline.
  pub fn thick(&self) -> i32 { self.raw.thick }
  /// Set to true (1) for thick underline.
  pub fn set_thick(&mut self, v: i32) -> &mut Self { self.raw.thick = v; self }
  /// Style.
  pub fn style(&self) -> &crate::include::internal::CefCompositionUnderlineStyle { unsafe { &*(self as *const _ as *const _) } }
  /// Style.
  pub fn set_style(&mut self, v: crate::include::internal::CefCompositionUnderlineStyle) -> &mut Self { self.raw.style = v.into(); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Enumerates the various representations of the ordering of audio channels.
/// Must be kept synchronized with media::ChannelLayout from Chromium.
/// See media\base\channel_layout.h
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefChannelLayout(cef_sys::cef_channel_layout_t);
impl CefChannelLayout {
  pub const NONE: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_NONE);
  pub const UNSUPPORTED: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_UNSUPPORTED);
  /// Front C
  pub const MONO: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_MONO);
  /// Front L, Front R
  pub const STEREO: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_STEREO);
  /// Front L, Front R, Back C
  pub const X2_1: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_2_1);
  /// Front L, Front R, Front C
  pub const SURROUND: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_SURROUND);
  /// Front L, Front R, Front C, Back C
  pub const X4_0: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_4_0);
  /// Front L, Front R, Side L, Side R
  pub const X2_2: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_2_2);
  /// Front L, Front R, Back L, Back R
  pub const QUAD: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_QUAD);
  /// Front L, Front R, Front C, Side L, Side R
  pub const X5_0: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_5_0);
  /// Front L, Front R, Front C, LFE, Side L, Side R
  pub const X5_1: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_5_1);
  /// Front L, Front R, Front C, Back L, Back R
  pub const X5_0_BACK: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_5_0_BACK);
  /// Front L, Front R, Front C, LFE, Back L, Back R
  pub const X5_1_BACK: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_5_1_BACK);
  /// Front L, Front R, Front C, Side L, Side R, Back L, Back R
  pub const X7_0: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_7_0);
  /// Front L, Front R, Front C, LFE, Side L, Side R, Back L, Back R
  pub const X7_1: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_7_1);
  /// Front L, Front R, Front C, LFE, Side L, Side R, Front LofC, Front RofC
  pub const X7_1_WIDE: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_7_1_WIDE);
  /// Stereo L, Stereo R
  pub const STEREO_DOWNMIX: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_STEREO_DOWNMIX);
  /// Stereo L, Stereo R, LFE
  pub const X2POINT1: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_2POINT1);
  /// Stereo L, Stereo R, Front C, LFE
  pub const X3_1: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_3_1);
  /// Stereo L, Stereo R, Front C, Rear C, LFE
  pub const X4_1: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_4_1);
  /// Stereo L, Stereo R, Front C, Side L, Side R, Back C
  pub const X6_0: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_6_0);
  /// Stereo L, Stereo R, Side L, Side R, Front LofC, Front RofC
  pub const X6_0_FRONT: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_6_0_FRONT);
  /// Stereo L, Stereo R, Front C, Rear L, Rear R, Rear C
  pub const HEXAGONAL: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_HEXAGONAL);
  /// Stereo L, Stereo R, Front C, LFE, Side L, Side R, Rear Center
  pub const X6_1: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_6_1);
  /// Stereo L, Stereo R, Front C, LFE, Back L, Back R, Rear Center
  pub const X6_1_BACK: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_6_1_BACK);
  /// Stereo L, Stereo R, Side L, Side R, Front LofC, Front RofC, LFE
  pub const X6_1_FRONT: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_6_1_FRONT);
  /// Front L, Front R, Front C, Side L, Side R, Front LofC, Front RofC
  pub const X7_0_FRONT: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_7_0_FRONT);
  /// Front L, Front R, Front C, LFE, Back L, Back R, Front LofC, Front RofC
  pub const X7_1_WIDE_BACK: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_7_1_WIDE_BACK);
  /// Front L, Front R, Front C, Side L, Side R, Rear L, Back R, Back C.
  pub const OCTAGONAL: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_OCTAGONAL);
  /// Channels are not explicitly mapped to speakers.
  pub const DISCRETE: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_DISCRETE);
  /// Front L, Front R, Front C. Front C contains the keyboard mic audio. This
  /// layout is only intended for input for WebRTC. The Front C channel
  /// is stripped away in the WebRTC audio input pipeline and never seen outside
  /// of that.
  pub const STEREO_AND_KEYBOARD_MIC: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_STEREO_AND_KEYBOARD_MIC);
  /// Front L, Front R, Side L, Side R, LFE
  pub const X4_1_QUAD_SIDE: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_4_1_QUAD_SIDE);
  /// Actual channel layout is specified in the bitstream and the actual channel
  /// count is unknown at Chromium media pipeline level (useful for audio
  /// pass-through mode).
  pub const BITSTREAM: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_BITSTREAM);
  /// Max value, must always equal the largest entry ever logged.
  pub const MAX: CefChannelLayout = CefChannelLayout(cef_sys::cef_channel_layout_t_CEF_CHANNEL_LAYOUT_MAX);
}
impl From<cef_sys::cef_channel_layout_t> for CefChannelLayout {
  fn from(raw: cef_sys::cef_channel_layout_t) -> CefChannelLayout {
    CefChannelLayout(raw)
  }
}
impl From<CefChannelLayout> for cef_sys::cef_channel_layout_t {
  fn from(src: CefChannelLayout) -> cef_sys::cef_channel_layout_t {
    src.0
  }
}
impl std::ops::BitOr for CefChannelLayout {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Structure representing the audio parameters for setting up the audio handler.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefAudioParameters { pub raw: cef_sys::cef_audio_parameters_t }
impl Default for CefAudioParameters {
  fn default() -> CefAudioParameters {
    let /* mut */ raw = cef_sys::cef_audio_parameters_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_audio_parameters_t>() as u64;
    CefAudioParameters { raw }
  }
}
impl From<CefAudioParameters> for cef_sys::cef_audio_parameters_t {
  fn from(src: CefAudioParameters) -> cef_sys::cef_audio_parameters_t {
    src.raw
  }
}
impl From<cef_sys::cef_audio_parameters_t> for CefAudioParameters {
  fn from(raw: cef_sys::cef_audio_parameters_t) -> CefAudioParameters {
    CefAudioParameters { raw }
  }
}
#[allow(non_snake_case)]
impl CefAudioParameters {
  /// Layout of the audio channels
  pub fn channel_layout(&self) -> &crate::include::internal::CefChannelLayout { unsafe { &*(self as *const _ as *const _) } }
  /// Layout of the audio channels
  pub fn set_channel_layout(&mut self, v: crate::include::internal::CefChannelLayout) -> &mut Self { self.raw.channel_layout = v.into(); self }
  /// Sample rate
  /// 
  pub fn sample_rate(&self) -> i32 { self.raw.sample_rate }
  /// Sample rate
  /// 
  pub fn set_sample_rate(&mut self, v: i32) -> &mut Self { self.raw.sample_rate = v; self }
  /// Number of frames per buffer
  pub fn frames_per_buffer(&self) -> i32 { self.raw.frames_per_buffer }
  /// Number of frames per buffer
  pub fn set_frames_per_buffer(&mut self, v: i32) -> &mut Self { self.raw.frames_per_buffer = v; self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Result codes for CefMediaRouter::CreateRoute. Should be kept in sync with
/// Chromium's media_router::RouteRequestResult::ResultCode type.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t);
impl CefMediaRouteCreateResult {
  pub const UNKNOWN_ERROR: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_UNKNOWN_ERROR);
  pub const OK: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_OK);
  pub const TIMED_OUT: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_TIMED_OUT);
  pub const ROUTE_NOT_FOUND: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_ROUTE_NOT_FOUND);
  pub const SINK_NOT_FOUND: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_SINK_NOT_FOUND);
  pub const INVALID_ORIGIN: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_INVALID_ORIGIN);
  pub const NO_SUPPORTED_PROVIDER: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_NO_SUPPORTED_PROVIDER);
  pub const CANCELLED: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_CANCELLED);
  pub const ROUTE_ALREADY_EXISTS: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_ROUTE_ALREADY_EXISTS);
  pub const TOTAL_COUNT: CefMediaRouteCreateResult = CefMediaRouteCreateResult(cef_sys::cef_media_route_create_result_t_CEF_MRCR_TOTAL_COUNT);
}
impl From<cef_sys::cef_media_route_create_result_t> for CefMediaRouteCreateResult {
  fn from(raw: cef_sys::cef_media_route_create_result_t) -> CefMediaRouteCreateResult {
    CefMediaRouteCreateResult(raw)
  }
}
impl From<CefMediaRouteCreateResult> for cef_sys::cef_media_route_create_result_t {
  fn from(src: CefMediaRouteCreateResult) -> cef_sys::cef_media_route_create_result_t {
    src.0
  }
}
impl std::ops::BitOr for CefMediaRouteCreateResult {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Connection state for a MediaRoute object.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMediaRouteConnectionState(cef_sys::cef_media_route_connection_state_t);
impl CefMediaRouteConnectionState {
  pub const UNKNOWN: CefMediaRouteConnectionState = CefMediaRouteConnectionState(cef_sys::cef_media_route_connection_state_t_CEF_MRCS_UNKNOWN);
  pub const CONNECTING: CefMediaRouteConnectionState = CefMediaRouteConnectionState(cef_sys::cef_media_route_connection_state_t_CEF_MRCS_CONNECTING);
  pub const CONNECTED: CefMediaRouteConnectionState = CefMediaRouteConnectionState(cef_sys::cef_media_route_connection_state_t_CEF_MRCS_CONNECTED);
  pub const CLOSED: CefMediaRouteConnectionState = CefMediaRouteConnectionState(cef_sys::cef_media_route_connection_state_t_CEF_MRCS_CLOSED);
  pub const TERMINATED: CefMediaRouteConnectionState = CefMediaRouteConnectionState(cef_sys::cef_media_route_connection_state_t_CEF_MRCS_TERMINATED);
}
impl From<cef_sys::cef_media_route_connection_state_t> for CefMediaRouteConnectionState {
  fn from(raw: cef_sys::cef_media_route_connection_state_t) -> CefMediaRouteConnectionState {
    CefMediaRouteConnectionState(raw)
  }
}
impl From<CefMediaRouteConnectionState> for cef_sys::cef_media_route_connection_state_t {
  fn from(src: CefMediaRouteConnectionState) -> cef_sys::cef_media_route_connection_state_t {
    src.0
  }
}
impl std::ops::BitOr for CefMediaRouteConnectionState {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Icon types for a MediaSink object. Should be kept in sync with Chromium's
/// media_router::SinkIconType type.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t);
impl CefMediaSinkIconType {
  pub const CAST: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_CAST);
  pub const CAST_AUDIO_GROUP: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_CAST_AUDIO_GROUP);
  pub const CAST_AUDIO: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_CAST_AUDIO);
  pub const MEETING: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_MEETING);
  pub const HANGOUT: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_HANGOUT);
  pub const EDUCATION: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_EDUCATION);
  pub const WIRED_DISPLAY: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_WIRED_DISPLAY);
  pub const GENERIC: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_GENERIC);
  pub const TOTAL_COUNT: CefMediaSinkIconType = CefMediaSinkIconType(cef_sys::cef_media_sink_icon_type_t_CEF_MSIT_TOTAL_COUNT);
}
impl From<cef_sys::cef_media_sink_icon_type_t> for CefMediaSinkIconType {
  fn from(raw: cef_sys::cef_media_sink_icon_type_t) -> CefMediaSinkIconType {
    CefMediaSinkIconType(raw)
  }
}
impl From<CefMediaSinkIconType> for cef_sys::cef_media_sink_icon_type_t {
  fn from(src: CefMediaSinkIconType) -> cef_sys::cef_media_sink_icon_type_t {
    src.0
  }
}
impl std::ops::BitOr for CefMediaSinkIconType {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
/// Device information for a MediaSink object.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefMediaSinkDeviceInfo { pub raw: cef_sys::cef_media_sink_device_info_t }
impl Default for CefMediaSinkDeviceInfo {
  fn default() -> CefMediaSinkDeviceInfo {
    let /* mut */ raw = cef_sys::cef_media_sink_device_info_t::default();
    // raw.size = std::mem::size_of::<cef_sys::cef_media_sink_device_info_t>() as u64;
    CefMediaSinkDeviceInfo { raw }
  }
}
impl From<CefMediaSinkDeviceInfo> for cef_sys::cef_media_sink_device_info_t {
  fn from(src: CefMediaSinkDeviceInfo) -> cef_sys::cef_media_sink_device_info_t {
    src.raw
  }
}
impl From<cef_sys::cef_media_sink_device_info_t> for CefMediaSinkDeviceInfo {
  fn from(raw: cef_sys::cef_media_sink_device_info_t) -> CefMediaSinkDeviceInfo {
    CefMediaSinkDeviceInfo { raw }
  }
}
#[allow(non_snake_case)]
impl CefMediaSinkDeviceInfo {
  pub fn ip_address(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.ip_address) }
  pub fn set_ip_address(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.ip_address, v); self }
  pub fn port(&self) -> i32 { self.raw.port }
  pub fn set_port(&mut self, v: i32) -> &mut Self { self.raw.port = v; self }
  pub fn model_name(&self) -> String { crate::include::helpers::cef_string_to_string(&self.raw.model_name) }
  pub fn set_model_name(&mut self, v: &str) -> &mut Self { crate::include::helpers::str_to_cef_string(&mut self.raw.model_name, v); self }
  pub fn build(&self) -> Self { Self { raw: self.raw } }
}
/// Represents commands available to TextField.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefTextFieldCommands(cef_sys::cef_text_field_commands_t);
impl CefTextFieldCommands {
  pub const CUT: CefTextFieldCommands = CefTextFieldCommands(cef_sys::cef_text_field_commands_t_CEF_TFC_CUT);
  pub const COPY: CefTextFieldCommands = CefTextFieldCommands(cef_sys::cef_text_field_commands_t_CEF_TFC_COPY);
  pub const PASTE: CefTextFieldCommands = CefTextFieldCommands(cef_sys::cef_text_field_commands_t_CEF_TFC_PASTE);
  pub const UNDO: CefTextFieldCommands = CefTextFieldCommands(cef_sys::cef_text_field_commands_t_CEF_TFC_UNDO);
  pub const DELETE: CefTextFieldCommands = CefTextFieldCommands(cef_sys::cef_text_field_commands_t_CEF_TFC_DELETE);
  pub const SELECT_ALL: CefTextFieldCommands = CefTextFieldCommands(cef_sys::cef_text_field_commands_t_CEF_TFC_SELECT_ALL);
}
impl From<cef_sys::cef_text_field_commands_t> for CefTextFieldCommands {
  fn from(raw: cef_sys::cef_text_field_commands_t) -> CefTextFieldCommands {
    CefTextFieldCommands(raw)
  }
}
impl From<CefTextFieldCommands> for cef_sys::cef_text_field_commands_t {
  fn from(src: CefTextFieldCommands) -> cef_sys::cef_text_field_commands_t {
    src.0
  }
}
impl std::ops::BitOr for CefTextFieldCommands {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
