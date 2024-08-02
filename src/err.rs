#[macro_export]
macro_rules! err_line {
    ($msg:expr, $index:expr) => {{
        // $index + 1 because sbg starts on line 1
        // and enumerate() on line 0
        eprintln!(" error: line {} {}", $index + 1, $msg);
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! err_exit {
    ($title:expr, $msg:expr) => {{
        eprintln!(" error: {}, {}", $title, $msg);
        std::process::exit(1);
    }};
}

pub use err_exit;
pub use err_line;
