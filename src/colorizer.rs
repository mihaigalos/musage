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
    pub fn colorize_bar(
        bar_length: usize,
        bar_unit: &str,
        count_units: usize,
        percent_usage: f64,
    ) -> String {
        let mut result = "".to_string();
        for i in 0..bar_length {
            if i < count_units {
                result = format!(
                    "{}{}",
                    result,
                    Colorizer::colorize_used(bar_unit.to_string(), percent_usage)
                );
            } else {
                result = format!("{}{}", result, bar_unit.white().dimmed());
            }
        }
        result
    }
}
