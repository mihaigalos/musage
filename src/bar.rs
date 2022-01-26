use crate::colorizer::Colorizer;

pub struct Bar;

impl Bar {
    pub fn new(percent_usage: f64) -> String {
        let bar_length = 20;
        let bar_unit = "■";
        let count_units = Bar::compute_bar_units(percent_usage, bar_length);
        Colorizer::colorize_bar(bar_length, bar_unit, count_units, percent_usage)
    }
    fn compute_bar_units(mut percent: f64, total_chars: usize) -> usize {
        if percent.is_nan() {
            percent = 0.0;
        }
        let parts_used = (percent.round() * total_chars as f64 / 100.0).ceil() as usize;
        parts_used
    }
}
