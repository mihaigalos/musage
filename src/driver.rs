use clap::ArgMatches;

use crate::reader::Reader;
use crate::writer::Writer;
pub struct Driver;

impl Driver {
    pub fn drive(args: ArgMatches) {
        let stats = Reader::read(&args);
        Writer::write(stats, args);
    }
}
