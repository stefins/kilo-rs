use std::env;
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn initEditor();
    fn editorSelectSyntaxHighlight(s: *const c_char);
    fn editorOpen(filename: *const c_char) -> i32;
    fn enableRawMode(fd: i32) -> i32;
    fn editorSetStatusMessage(fmt: *const c_char);
    fn editorRefreshScreen();
    fn editorProcessKeypress(fd: i32);
}

pub type KiloResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run_kilo(args: Vec<String>) -> KiloResult<()> {
    if args.len() != 2 {
        println!("Usage: kilo <filename>");
    }
    let file_name = CString::new(args[1].as_str())?;
    unsafe {
        initEditor();
        editorSelectSyntaxHighlight(file_name.as_ptr());
        editorOpen(file_name.as_ptr());
        enableRawMode(0);
        editorSetStatusMessage(
            CString::new("HELP: Ctrl-S = save | Ctrl-Q = quit | Ctrl-F = find")?.as_ptr(),
        );
        loop {
            editorRefreshScreen();
            editorProcessKeypress(1);
        }
    }
}
fn main() -> KiloResult<()> {
    let args: Vec<String> = env::args().collect();
    run_kilo(args)
}
