use colored::*;
pub struct Colorizer;

impl Colorizer {
    pub fn colorize_used(input: String, percent_disk: f64) -> ColoredString {
        if percent_disk > 90.0 {
            input.red()
        } else if percent_disk > 75.0 {
            input.yellow()
        } else {
            input.green()
        }
    }

    pub fn colorize_cached(input: String, percent_disk: f64) -> ColoredString {
        if percent_disk > 80.0 {
            input.on_magenta()
        } else {
            input.on_blue()
        }
    }

    pub fn colorize_bar(
        bar_length: usize,
        bar_unit: &str,
        bar_units_usage: usize,
        bar_units_cache: usize,
        percent_usage: f64,
        percent_cache: f64,
    ) -> String {
        let mut result = "".to_string();
        for i in 0..bar_length {
            if i < bar_units_usage {
                result = format!(
                    "{}{}",
                    result,
                    Colorizer::colorize_used(bar_unit.to_string(), percent_usage)
                );
            } else {
                result = format!("{}{}", result, bar_unit.white().dimmed());
            }
            if i < bar_units_cache {
                result = format!("{}", Colorizer::colorize_cached(result, percent_cache));
            }
        }
        result
    }
}
