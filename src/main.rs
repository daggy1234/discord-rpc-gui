mod styles;

use discord_rich_presence::{activity, new_client, DiscordIpc};
use iced::{
    button, text_input, Align, Button, Column, Element, Sandbox, Settings, Space, Text, TextInput,
};

pub fn main() -> iced::Result {
    std::thread::spawn(|| {
        match new_client("731140044970262590") {
            Ok(mut c) => {
                let o = match c.connect() {
                    Ok(_o) => {
                        println!("Client Connec");
                        true
                    }
                    Err(_e) => {
                        println!("Errpr");
                        false
                    }
                };
                if o {
                    match c.set_activity(activity::Activity::new().state("foo").details("bar")) {
                        Ok(_a) => println!("Set Activity"),
                        Err(_eaa) => println!("Error setting activity"),
                    };
                    std::thread::sleep_ms(100000);
                    match c.close() {
                        Ok(_aaaaa) => println!("Closed Conn"),
                        Err(_aaaaaaaaaa) => println!("Failed close"),
                    };
                };
            }
            Err(_e) => {
                println!("E");
            }
        };
    });
    Counter::run(Settings::default())
}

use styles::{DiscordDefaultButton, DiscordGreenButton, DiscordRedButton, DiscordTextInput};

#[derive(Default)]
struct ClientIdState {
    st: text_input::State,
    val: String,
    valid: bool,
}

#[derive(Default)]
struct ClientIdValidState {
    st: button::State,
    status: Option<bool>,
}

#[derive(Default)]
struct Counter {
    client_id: ClientIdState,
    client_id_valid: ClientIdValidState,
}

#[derive(Debug, Clone)]
enum Message {
    ClientIdInputChanged(String),
    ValidateClientId,
}

// fn get_cvv_styl(stat: Option<bool>) -> button::StyleSheet {
//     match stat {
//         Some(v) => {
//             if v {
//                 return DiscordRedButton;
//             }
//             return DiscordGreenButton;
//         }
//         None => return DiscordDefaultButton,
//     }
// }

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Discord-RPC-GUI")
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb(0.172, 0.184, 0.2)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ValidateClientId => {
                let tp = match self.client_id.val.parse::<u64>() {
                    Ok(_v) => true,
                    Err(_e) => false,
                };
                self.client_id.valid = tp;
                self.client_id_valid.status = Some(tp);
                println!("Help");
            }
            Message::ClientIdInputChanged(v) => {
                self.client_id.val = v;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let cbs = &mut self.client_id_valid;
        Column::new()
            .padding(20)
            .align_items(Align::Start)
            .push(
                Text::new("Discord-RPC-GUI")
                    .color(iced::Color::WHITE)
                    .size(50),
            )
            .push(Space::with_height(iced::Length::Units(10)))
            .push(
                TextInput::new(
                    &mut self.client_id.st,
                    "Enter Client ID",
                    &self.client_id.val,
                    Message::ClientIdInputChanged,
                )
                .style(DiscordTextInput)
                .padding(20),
            )
            .push(Space::with_height(iced::Length::Units(20)))
            .push(
                Button::new(&mut cbs.st, Text::new("Validate Client Id"))
                    .style(DiscordDefaultButton)
                    .padding(10)
                    .on_press(Message::ValidateClientId),
            )
            .into()
    }
}
