#[repr(u8)]
#[derive(Debug, Clone)]
pub enum OptionKind {
    BufferSize = 0,
    SegmentNumber = 1,
}

impl From<u8> for OptionKind {
    fn from(kind: u8) -> Self {
        match kind {
            0 => Self::BufferSize,
            1 => Self::SegmentNumber,
            _ => panic!("Unknown option kind {}", kind)
        }
    }
}


pub struct FrameOption<'a> {
    pub kind: OptionKind,
    pub data: &'a[u8],
}

impl<'a> FrameOption<'a> {
    pub fn new(kind: OptionKind, data: &'a [u8]) -> Self {
        Self {
            kind,
            data,
        }
    }
}