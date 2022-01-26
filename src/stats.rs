use clap::ArgMatches;
use procfs::Meminfo;

#[derive(Debug)]
pub struct Stats {
    pub name: String,
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub shared: u64,
    pub buff_cache: u64,
    pub available: u64,
    pub percent_usage: f64,
}

impl Stats {
    pub fn new(name: &str, meminfo: Meminfo, _: &ArgMatches) -> Stats {
        let used = meminfo.mem_total - meminfo.mem_available.unwrap();
        let percent_usage = 100.0 * (used as f64 / meminfo.mem_total as f64);
        Stats {
            name: name.to_string(),
            total: meminfo.mem_total,
            used: used,
            free: meminfo.mem_free,
            shared: 0,
            buff_cache: 0,
            available: meminfo.mem_available.unwrap(),
            percent_usage: percent_usage,
        }
    }
}
