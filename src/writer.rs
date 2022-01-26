use byte_unit::Byte;
use clap::ArgMatches;
use colored::*;

use crate::bar::Bar;
use crate::stats::Stats;
pub struct Writer;

impl Writer {
    fn iec_representation(input: u64) -> String {
        Byte::from_bytes(input as u128)
            .get_appropriate_unit(true)
            .format(1)
            .replace(" ", "")
            .replace("i", "")
            .replace("B", "")
    }

    pub fn write(stats: Vec<Stats>, _: ArgMatches) {
        Writer::write_disks(stats);
    }
    pub fn write_disks(stats: Vec<Stats>) {
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
        for stat in stats {
            Writer::write_stat(stat);
        }
    }

    fn write_stat(stat: Stats) {
        println!(
            "{:width$} {:>11} {:>11} {:>11} {:>11} {:>11} {:>11} {:>5.4} {}",
            stat.name.yellow().bold(),
            Writer::iec_representation(stat.total),
            Writer::iec_representation(stat.used),
            Writer::iec_representation(stat.free),
            Writer::iec_representation(stat.shared),
            Writer::iec_representation(stat.buff_cache),
            Writer::iec_representation(stat.available),
            stat.percent_usage.to_string() + "%",
            Bar::new(stat.percent_usage),
            width = 3
        );
    }
}
