extern crate clap;
extern crate colored;

use autoclap::autoclap;
use clap::{App, Arg};

#[cfg(not(tarpaulin_include))]
fn main() {
    let args = autoclap!().try_get_matches().unwrap_or_else(|e| e.exit());

    musage::driver::Driver::drive(args);
}
