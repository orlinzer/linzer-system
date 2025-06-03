use linzer_system::interfaces::cli;
use std::io::Result;

/// Main function of the Linzer system
fn main() -> Result<()> {
    let mut broker = cli::run_cli_setup()?;
    cli::run_cli_loop(&mut broker)?;
    Ok(())
}
