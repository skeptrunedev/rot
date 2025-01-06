#![allow(clippy::multiple_crate_versions)]

use clap::Parser;
use utils::utilities::{RotArgs, RotUtils};

pub mod utils;

pub fn main() {
    let args = RotArgs::parse();
    match args.cmd {
        RotUtils::Init(u) => {
            if let Err(e) = u.run() {
                eprintln!("{}", e.message);
            }
        }
    }
}
