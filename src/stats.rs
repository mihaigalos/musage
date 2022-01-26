use clap::ArgMatches;
use procfs::Meminfo;

#[derive(Debug)]
pub struct StatsMem {
    pub name: String,
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub shared: u64,
    pub buff_cache: u64,
    pub available: u64,
    pub percent_usage: f64,
    pub percent_cache: f64,
}

#[derive(Debug)]
pub struct StatsSwap {
    pub name: String,
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub percent_usage: f64,
}
impl StatsMem {
    pub fn new(name: &str, meminfo: Meminfo, args: &ArgMatches) -> StatsMem {
        let used = meminfo.mem_total - meminfo.mem_available.unwrap();
        let buff_cache = meminfo.cached + meminfo.buffers;
        let percent_usage = 100.0 * (used as f64 / meminfo.mem_total as f64);
        let percent_cache = 100.0 * (buff_cache as f64 / meminfo.mem_total as f64);

        let result = StatsMem {
            name: name.to_string(),
            total: meminfo.mem_total,
            used: used,
            free: meminfo.mem_free,
            shared: meminfo.shmem.unwrap(),
            buff_cache: buff_cache,
            available: meminfo.mem_available.unwrap(),
            percent_usage: percent_usage,
            percent_cache: percent_cache,
        };
        if args.is_present("debug") {
            println!("{:?}", result);
        }
        result
    }
    pub fn new_empty() -> StatsMem {
        StatsMem {
            name: "unknown".to_string(),
            total: 0,
            used: 0,
            free: 0,
            shared: 0,
            buff_cache: 0,
            available: 0,
            percent_usage: 0.0,
            percent_cache: 0.0,
        }
    }
}
impl StatsSwap {
    pub fn new(name: &str, meminfo: Meminfo, args: &ArgMatches) -> StatsSwap {
        let percent_usage = 100.0 * (meminfo.swap_cached as f64 / meminfo.swap_total as f64);
        let _percent_cache = 81.0;
        let result = StatsSwap {
            name: name.to_string(),
            total: meminfo.swap_total,
            used: meminfo.swap_cached,
            free: meminfo.swap_free,
            percent_usage: percent_usage,
        };
        if args.is_present("debug") {
            println!("{:?}", result);
        }
        result
    }
    pub fn new_empty() -> StatsSwap {
        StatsSwap {
            name: "unknown".to_string(),
            total: 0,
            used: 0,
            free: 0,
            percent_usage: 0.0,
        }
    }
}
