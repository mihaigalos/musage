use crate::stats::Stats;
use clap::ArgMatches;
use procfs::Meminfo;

pub struct Reader;

impl Reader {
    pub fn read(args: &ArgMatches) -> Vec<Stats> {
        let mut stats: Vec<Stats> = Vec::new();

        let meminfo = Meminfo::new();
        match meminfo {
            Ok(meminfo) => {
                let s = Stats::new("Mem", meminfo, args);
                stats.push(s);
            }
            Err(err) => println!("Error: {}", err),
        }

        stats
    }
}
