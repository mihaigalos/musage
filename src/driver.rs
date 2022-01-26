use clap::ArgMatches;

use crate::reader::Reader;
use crate::writer::Writer;
pub struct Driver;

impl Driver {
    pub fn drive(args: ArgMatches) {
        Writer::write(Reader::read(&args), args);
    }
}
