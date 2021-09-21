use crate::syscall::*;
use core::fmt::{Error, Write};

pub struct Writer;

impl Write for Writer {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        unsafe {
            sys_write(0, out.as_bytes().as_ptr(), out.len());
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {{
        use core::fmt::Write;
        let _ = write!(Writer, $($args)+);
    }};
}

#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}
