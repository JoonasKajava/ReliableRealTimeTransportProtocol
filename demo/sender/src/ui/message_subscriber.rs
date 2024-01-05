use std::sync::mpsc::Receiver;
use iced::{Subscription, subscription};
use iced::futures::SinkExt;

pub fn subscribe_to_messages(receiver: Receiver<Vec<u8>>) -> Subscription<Event> {
    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        100,
        |mut output| async move {
            loop {
                let message = receiver.recv().unwrap();
                let message_content = String::from_utf8(message).unwrap();
                let _ = output.send(Event::MessageReceived(message_content)).await;
            }
        }
    )

}



#[derive(Debug, Clone)]
pub enum Event {
    MessageReceived(String)
}