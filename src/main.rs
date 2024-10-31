use glib::ExitCode;
use gui::creator;
use sheet::Catalog;

pub mod archive;
mod gui;
mod sheet;

pub const APP_ID: &str = "org.computingfun.cybernetic-trpg";

fn main() -> ExitCode {
    let r = creator();
    println!("{:?}", r);
    ExitCode::SUCCESS
}

/// Fetches the first command-line argument and attempts to return a `Catalog` entry.
/// Returns [`None`] if the argument or matching catalog entry is not found.
pub fn arg() -> Option<Catalog> {
    std::env::args_os().nth(1).and_then(Catalog::lookup)
}
