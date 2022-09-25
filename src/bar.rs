use crate::colorizer::Colorizer;

pub struct Bar;

impl Bar {
    pub fn create(percent_usage: f64, percent_cache: f64) -> String {
        let bar_length = 20;
        let bar_unit = "■";
        let bar_units_usage = Bar::compute_bar_units(percent_usage, bar_length);
        let bar_units_cache = Bar::compute_bar_units(percent_cache, bar_length);
        Colorizer::colorize_bar(
            bar_length,
            bar_unit,
            bar_units_usage,
            bar_units_cache,
            percent_usage,
            percent_cache,
        )
    }
    fn compute_bar_units(mut percent: f64, total_chars: usize) -> usize {
        if percent.is_nan() {
            percent = 0.0;
        }
        (percent.round() * total_chars as f64 / 100.0).ceil() as usize
    }
}
