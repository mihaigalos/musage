extern crate clap;
extern crate colored;

use autoclap::autoclap;
use clap::Arg;
use clap::Command;
use std::env;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app: clap::Command = autoclap!();
    let args = app.clone().try_get_matches().unwrap_or_else(|e| e.exit());
    musage::driver::Driver::drive(args);
}
