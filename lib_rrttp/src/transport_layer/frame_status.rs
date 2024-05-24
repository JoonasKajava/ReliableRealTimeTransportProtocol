use std::time::Instant;

#[derive(Clone, Copy, Default)]
pub enum FrameStatus {
    #[default]
    NotSent,
    Acknowledged,
    Sent(Instant),
}
