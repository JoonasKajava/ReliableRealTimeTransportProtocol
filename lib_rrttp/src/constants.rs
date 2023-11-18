pub const BUFFER_SIZE: u32 = 1024;

/// Timeout in milliseconds, when to stop waiting for a response.
pub const TIMEOUT: usize = 1000;

/// In bytes
pub const MAX_DATA_SIZE: u32 = 128;
/// In bytes
const SEQ_NUM_SIZE: u32 = 4;
/// In bytes
const ACK_NUM_SIZE: u32 = 4;
/// In bytes
const CONTROL_BITS_SIZE: u32 = 1;
/// In bytes
const RESERVED_SIZE: u32 = 3;
/// In bytes
const OPTION_KIND_SIZE: u32 = 1;
/// In bytes
const OPTION_LENGTH_SIZE: u32 = 1;
/// In bytes
const OPTION_DATA_SIZE: u32 = 4;
const MAX_OPTION_COUNT: u32 = 2;

pub const MIN_FRAME_SIZE: u32 = SEQ_NUM_SIZE + ACK_NUM_SIZE + CONTROL_BITS_SIZE + RESERVED_SIZE;
pub const MAX_FRAME_SIZE: u32 = SEQ_NUM_SIZE + ACK_NUM_SIZE + CONTROL_BITS_SIZE + RESERVED_SIZE + ((OPTION_KIND_SIZE + OPTION_LENGTH_SIZE + OPTION_DATA_SIZE) * MAX_OPTION_COUNT) + MAX_DATA_SIZE;


pub const WINDOW_SIZE: u32 = 64;