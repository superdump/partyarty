use color::Colorf32;
use timers::Timers;

#[derive(Debug, Default)]
pub struct PerfTimers(pub Timers);

#[derive(Debug, Default)]
pub struct Samples(pub usize);

#[derive(Debug, Default)]
pub struct Width(pub usize);

#[derive(Debug, Default)]
pub struct Height(pub usize);

#[derive(Debug, Default)]
pub struct FrameCount(pub u32);

#[derive(Debug, Default)]
pub struct BufferTotals(pub Vec<Colorf32>);

#[derive(Debug, Default)]
pub struct BufferOutput(pub Vec<u32>);

#[derive(Debug, Default)]
pub struct ImageFilePrefix(pub String);
