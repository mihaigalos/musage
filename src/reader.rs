use crate::stats::StatsMem;
use crate::stats::StatsSwap;
use clap::ArgMatches;
use procfs::Meminfo;

pub struct Reader;

impl Reader {
    pub fn read(args: &ArgMatches) -> (StatsMem, StatsSwap) {
        let meminfo = Meminfo::new();
        match meminfo {
            Ok(meminfo) => {
                (
                    StatsMem::new("Mem", meminfo.clone(), args),
                    StatsSwap::new("Swap", meminfo, args),
                )
            }
            Err(err) => {
                println!("Error: {}", err);
                (StatsMem::new_empty(), StatsSwap::new_empty())
            }
        }
    }
}
