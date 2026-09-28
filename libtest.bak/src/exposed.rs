
//use core::slice::from_raw_parts;


/// `OutputPipe` works as a output bridge between a `#![no_std]`
/// library and the rust testing environment
///
/// # Usage
/// ```rust
/// use core::fmt::Write;
///
/// let mut out = OutputPipe;
///
/// _ = write!(&mut out, "Hello {}", "World!");
///
/// ```
pub struct OutputPipe;


impl core::fmt::Write for OutputPipe {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        std::print!("{c}");
        Ok(())
    }
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        std::print!("{s}");
        Ok(())
    }

}

/// Formats output into the `stdout`
#[macro_export]
macro_rules! print {
    ($guard:ident: $($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($guard, $($arg)*);
    }};
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use $crate::exposed::OutputPipe;
        let mut pipe = OutputPipe;
        let _ = write!(&mut pipe, $($arg)*);
    }};
}

/// Formats output into the `stdout`
#[macro_export]
macro_rules! println {
    ($guard:ident: $($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = writeln!($guard, $($arg)*);
    }};
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use $crate::exposed::OutputPipe;
        let mut pipe = OutputPipe;
        let _ = writeln!(&mut pipe, $($arg)*);
    }};
}


/// Formats output into the `stdout`
#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}:{}]", core::file!(), core::line!(), core::column!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}:{}] {} = {:#?}",
                    core::file!(),
                    core::line!(),
                    core::column!(),
                    core::stringify!($val),
                    // The `&T: Debug` check happens here (not in the format literal desugaring)
                    // to avoid format literal related messages and suggestions.
                    &&tmp as &dyn core::fmt::Debug,
                );
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
