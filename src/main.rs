extern crate clap;
extern crate colored;

use autoclap::autoclap;
use clap::Command;
use clap::{Arg, ArgAction};
use std::env;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app: clap::Command = autoclap!()
        .arg(
            Arg::new("version")
                .long("version")
                .short('v')
                .action(ArgAction::SetTrue)
                .help("Display version information.")
                .required(false),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('D')
                .action(ArgAction::SetTrue)
                .help("Print raw data used internally.")
                .required(false),
        );
    let args = app.clone().try_get_matches().unwrap_or_else(|e| e.exit());

    if args.get_flag("version") {
        println!("{}", app.get_about().unwrap());
    } else {
        musage::driver::Driver::drive(args);
    }
}
