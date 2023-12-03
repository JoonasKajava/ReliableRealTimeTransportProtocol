pub const BUFFER_SIZE: usize = 1024;

/// Timeout in milliseconds, when to stop waiting for a response.
pub const TIMEOUT: u128 = 200;

/// In bytes
pub const MAX_DATA_SIZE: usize = 128;
/// In bytes
const SEQ_NUM_SIZE: usize = 4;
/// In bytes
const ACK_NUM_SIZE: usize = 4;
/// In bytes
const CONTROL_BITS_SIZE: usize = 1;
/// In bytes
const RESERVED_SIZE: usize = 3;
/// In bytes
const OPTION_KIND_SIZE: usize = 1;
/// In bytes
const OPTION_LENGTH_SIZE: usize = 1;
/// In bytes
const OPTION_DATA_SIZE: usize = 4;
const MAX_OPTION_COUNT: usize = 2;

/// In bytes
pub const MIN_FRAME_SIZE: usize = SEQ_NUM_SIZE + ACK_NUM_SIZE + CONTROL_BITS_SIZE + RESERVED_SIZE;
/// In bytes
pub const MAX_FRAME_SIZE: usize = SEQ_NUM_SIZE + ACK_NUM_SIZE + CONTROL_BITS_SIZE + RESERVED_SIZE + ((OPTION_KIND_SIZE + OPTION_LENGTH_SIZE + OPTION_DATA_SIZE) * MAX_OPTION_COUNT) + MAX_DATA_SIZE;


pub const WINDOW_SIZE: usize = 64;