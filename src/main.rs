#[macro_use]
extern crate lalrpop_util;

mod ast;
mod parse;

use parse::parse;
use std::error::Error;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buffer = String::with_capacity(1024);

    std::io::stdin().read_to_string(&mut buffer)?;

    println!(
        "{:?}",
        parse(&buffer).map_err(|err| std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            err.description()
        ))?
    );

    Ok(())
}
