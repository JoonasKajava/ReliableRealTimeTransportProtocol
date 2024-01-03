use iced::{Alignment, Color, Element, Sandbox, Theme};
use iced::widget::{button, column, container, row, text, text_input};

use crate::ui::connection_status::ConnectionStatus;

#[derive(Default)]
pub struct RRTPApplication {
    local_address: String,
    remote_address: String,
    connection_status: ConnectionStatus,
}


#[derive(Debug, Clone)]
pub enum Message {
    LocalAddressChanged(String),
    RemoteAddressChanged(String),
    StopLocalSocket,
    Connect,
    Disconnect,
    Send,
    Receive,
}

impl Sandbox for RRTPApplication {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        "RRTTP".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Connect => {}
            Message::Disconnect => {}
            Message::Send => {}
            Message::Receive => {}
            Message::LocalAddressChanged(value) => {
                self.local_address = value;
            }
            Message::RemoteAddressChanged(value) => {
                self.remote_address = value;
            }
            Message::StopLocalSocket => {}
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let local_socket_address_input = row![
            text("Local address"),
            text_input("127.0.0.1:12345", self.local_address.as_str()).on_input(Message::LocalAddressChanged).width(iced::Length::Fixed(200f32)),
            button("Start local socket").on_press(Message::Connect),
        ]
            .align_items(Alignment::Center)
            .spacing(10);
        let remote_address_input = row![
            text("Remote address"),
            text_input("127.0.0.1:12345", self.remote_address.as_str()).on_input(Message::RemoteAddressChanged).width(iced::Length::Fixed(200f32)),
            button("Connect to remote address").on_press(Message::Connect),
        ]
            .align_items(Alignment::Center)
            .spacing(10);


        let local_socket_status = row![
            text("Local socket: "),
            text("haloo"),
            button("Stop").on_press(Message::StopLocalSocket),
        ].align_items(Alignment::Center)
            .spacing(10);

        let connection_status = row![
            text("Connection status: "),
            text("haloo"),
            button("Disconnect").on_press(Message::Disconnect),
        ].align_items(Alignment::Center)
            .spacing(10);

        let status_row = column![local_socket_status, connection_status]
            .width(iced::Length::Fill);


        let content = column![
               status_row,
            local_socket_address_input,
            remote_address_input
        ].spacing(10);

        let container_content: Element<_> = container(
            content
        ).padding(10).width(iced::Length::Fill)
            .into();
        container_content.explain(Color::BLACK)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}