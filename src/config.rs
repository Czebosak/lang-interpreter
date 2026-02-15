#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationLevel {
    None,
    Standard,
    Aggressive,
}

pub struct Config {
    pub optimization: OptimizationLevel,
}
