/// Launches the process specified via |command_line|. Returns true upon
/// success. Must be called on the browser process TID_PROCESS_LAUNCHER thread.
/// 
/// Unix-specific notes:
/// - All file descriptors open in the parent process will be closed in the
/// child process except for stdin, stdout, and stderr.
/// - If the first argument on the command line does not contain a slash,
/// PATH will be searched. (See man execvp.)
#[allow(non_snake_case)]
pub fn cef_launch_process(command_line: crate::include::CefCommandLine, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_launch_process(crate::include::CefCommandLine::to_cef_own(command_line),);
    if ret == 0 { false } else { true }
  }
}
