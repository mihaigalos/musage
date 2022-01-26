use clap::ArgMatches;
use procfs::Meminfo;
use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

use crate::procfields::ProcFields;
use crate::stats::Stats;

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

        // match sysinfo {
        //     Ok(sysinfo) => {
        //    let fields: Vec<&str> = line.split_whitespace().collect();
        //    let statvfs = match statvfs(fields[ProcFields::Mountpoint.upcast()]) {
        //        Ok(s) => s,
        //        Err(_) => continue, // i.e.: no permissions to read
        //    };
        //
        //    let s = Stats::new(
        //        fields[ProcFields::Filesystem.upcast()],
        //        fields[ProcFields::Mountpoint.upcast()],
        //        statvfs,
        //        args,
        //    );
        //
        //    max_width = cmp::max(max_width, s.filesystem.len());
        //    stats.push(s);
        // }
        // Err(err) => println!("Error: {}", err),
        // }

        stats
    }
}
