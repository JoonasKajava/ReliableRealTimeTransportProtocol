

#[derive(Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connected,
}

#[derive(Default, PartialOrd, PartialEq)]
pub enum LocalSocketStatus {
    #[default]
    Stopped,
    Started,
}