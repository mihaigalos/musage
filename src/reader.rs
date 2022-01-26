use crate::stats::StatsMem;
use crate::stats::StatsSwap;
use clap::ArgMatches;
use procfs::Meminfo;

pub struct Reader;

impl Reader {
    pub fn read(args: &ArgMatches) -> (StatsMem, StatsSwap) {
        let meminfoA = Meminfo::new();
        let meminfoB = Meminfo::new();
        match meminfoA {
            Ok(meminfoA) => {
                return (
                    StatsMem::new("Mem", meminfoA, args),
                    StatsSwap::new("Swap", meminfoB.unwrap(), args),
                );
            }
            _ => {
                println!("Unexpected error.");
                return (StatsMem::new_empty(), StatsSwap::new_empty());
            }
        }
    }
}
