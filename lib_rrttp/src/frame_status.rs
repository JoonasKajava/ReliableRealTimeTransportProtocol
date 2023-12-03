use std::time::Instant;

#[derive(Clone, Copy)]
pub enum FrameStatus {
    NotSent,
    Acknowledged,
    Sent(Instant)
}