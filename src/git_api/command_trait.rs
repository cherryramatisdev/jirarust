use std::process::Output;

#[cfg_attr(test, mockall::automock)]
pub trait Command {
    fn output(&self) -> Result<Output, std::io::Error>;
}
