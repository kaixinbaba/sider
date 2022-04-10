#![forbid(unsafe_code)]
mod cli;

use sider::SDResult;

fn main() -> SDResult<()> {
    cli::parse_cli()?;

    Ok(())
}
