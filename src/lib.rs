use std::{fmt::Display, time::Duration};

pub const B: usize = 1;
pub const KB: usize = 1024 * B;
pub const MB: usize = 1024 * KB;
pub const GB: usize = 1024 * MB;

pub const NS: u128 = 1;
pub const US: u128 = 1000 * NS;
pub const MS: u128 = 1000 * US;
pub const S: u128 = 1000 * MS;
pub const M: u128 = 60 * S;
// pub const H: u128 = 60 * M;

pub struct Stats<'a> {
    name: &'a str,
    time: u128,
    size: usize,
    rate: f64,
}

impl<'a> Stats<'a> {
    pub fn new(name: &'a str, time: Duration, size: usize) -> Stats<'a> {
        let time = time.as_nanos();
        let rate = size as f64 / time as f64;
        Stats {
            name,
            time,
            size,
            rate,
        }
    }
}

impl Display for Stats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (size, size_units) = match self.size {
            ..=KB => (self.size / B, "B"),
            ..=MB => (self.size / KB, "KB"),
            ..=GB => (self.size / MB, "MB"),
            _ => (self.size / GB, "GB"),
        };

        let (time, time_units) = match self.time {
            ..=US => (self.time as f64 / NS as f64, "ns"),
            ..=MS => (self.time as f64 / US as f64, "us"),
            ..=S => (self.time as f64 / MS as f64, "ms"),
            ..=M => (self.time as f64 / S as f64, "s"),
            _ => (self.time as f64 / M as f64, "m"),
        };

        let rate_in_bps = (self.rate as f64 * S as f64) as usize;
        let (rate, rate_units) = match rate_in_bps {
            ..=KB => (rate_in_bps as f64 / B as f64, "b/s"),
            ..=MB => (rate_in_bps as f64 / KB as f64, "kb/s"),
            ..=GB => (rate_in_bps as f64 / MB as f64, "mb/s"),
            _ => (rate_in_bps as f64 / GB as f64, "gb/s"),
        };

        let name = self.name;
        write!(f, "'{name}': processed {size:.2} {size_units} in {time:.2} {time_units} at {rate:.2} {rate_units}")
    }
}
