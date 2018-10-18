#[derive(Debug, Default)]
pub struct Samples(pub usize);

#[derive(Debug, Default)]
pub struct Width(pub usize);

#[derive(Debug, Default)]
pub struct Height(pub usize);

#[derive(Debug, Default)]
pub struct FrameCount(pub u32);

#[derive(Debug, Default)]
pub struct Buffer(pub Vec<u32>);
