pub const BUFFER_SIZE: usize = 1024;

/// Timeout in milliseconds, when to stop waiting for a response.
pub const TIMEOUT: usize = 1000;

pub const MAX_DATA_SIZE: usize = 1024;


const SEQ_NUM_SIZE: usize = 32;
const ACK_NUM_SIZE: usize = 32;
const CONTROL_BITS_SIZE: usize = 8;
const RESERVED_SIZE: usize = 2;
const OPTION_KIND_SIZE: usize = 8;
const OPTION_LENGTH_SIZE: usize = 8;
const OPTION_DATA_SIZE: usize = 32;
const MAX_OPTION_COUNT: usize = 2;

pub const MAX_FRAME_SIZE: usize = SEQ_NUM_SIZE + ACK_NUM_SIZE + CONTROL_BITS_SIZE + RESERVED_SIZE + ((OPTION_KIND_SIZE + OPTION_LENGTH_SIZE + OPTION_DATA_SIZE) * MAX_OPTION_COUNT) + MAX_DATA_SIZE;


pub const WINDOW_SIZE: usize = 64;