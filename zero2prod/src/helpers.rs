use std::error::Error;
use std::fmt::{Formatter, Result};

pub fn error_chain_fmt(e: &impl Error, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "{}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by: \n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}
