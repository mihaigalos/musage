use byte_unit::Byte;
use clap::ArgMatches;
use colored::*;

use crate::bar::Bar;
use crate::stats::StatsMem;
use crate::stats::StatsSwap;
pub struct Writer;

impl Writer {
    fn iec_representation(input: u64) -> String {
        Byte::from_bytes(input as u128)
            .get_appropriate_unit(true)
            .format(1)
            .replace(' ', "")
            .replace('i', "")
            .replace('B', "")
    }

    pub fn write((stats_mem, stats_swap): (StatsMem, StatsSwap), _: ArgMatches) {
        Writer::write_disks(stats_mem, stats_swap);
    }
    pub fn write_disks(stats_mem: StatsMem, stats_swap: StatsSwap) {
        println!(
            "{:width$} {:>11} {:>11} {:>11} {:>11} {:>11} {:>11} {:>5}",
            "".yellow().bold(),
            "Total".yellow().bold(),
            "Used".yellow().bold(),
            "Free".yellow().bold(),
            "Shared".yellow().bold(),
            "Buff/Cache".yellow().bold(),
            "Available".yellow().bold(),
            "Use%".yellow().bold(),
            width = 3
        );
        Writer::write_stat_memory(stats_mem);
        Writer::write_stat_swap(stats_swap);
    }

    fn write_stat_memory(stat: StatsMem) {
        println!(
            "{:width$} {:>11} {:>11} {:>11} {:>11} {:>11} {:>11} {:>5.4} {}",
            stat.name.yellow().bold(),
            Writer::iec_representation(stat.total),
            Writer::iec_representation(stat.used),
            Writer::iec_representation(stat.free),
            Writer::iec_representation(stat.shared),
            Writer::iec_representation(stat.buff_cache),
            Writer::iec_representation(stat.available),
            stat.percent_usage,
            Bar::create(stat.percent_usage, stat.percent_cache),
            width = 3
        );
    }
    fn write_stat_swap(stat: StatsSwap) {
        println!(
            "{:width$} {:>10} {:>11} {:>11} {:>11} {:>11} {:>11} {:>5.4} {}",
            stat.name.yellow().bold(),
            Writer::iec_representation(stat.total),
            Writer::iec_representation(stat.used),
            Writer::iec_representation(stat.free),
            "-",
            "-",
            "-",
            stat.percent_usage,
            Bar::create(stat.percent_usage, stat.percent_usage),
            width = 4
        );
    }
}
