use iced::{button, text_input, Background, Color};

pub struct DiscordRedButton;
impl button::StyleSheet for DiscordRedButton {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.930, 0.266, 0.289,
            ))),
            border_color: Color::TRANSPARENT,
            border_radius: 2.0,
            ..button::Style::default()
        }
    }
    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                0.930, 0.266, 0.289, 0.3,
            ))),
            ..self.active()
        }
    }
}

pub struct DiscordGreenButton;
impl button::StyleSheet for DiscordGreenButton {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.25, 0.641, 0.275,
            ))),
            border_color: Color::TRANSPARENT,
            border_radius: 2.0,
            ..button::Style::default()
        }
    }
    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                0.25, 0.641, 0.275, 0.3,
            ))),
            ..self.active()
        }
    }
}

pub struct DiscordDefaultButton;
impl button::StyleSheet for DiscordDefaultButton {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.25, 0.414, 0.669,
            ))),
            border_color: Color::TRANSPARENT,
            border_radius: 2.0,
            ..button::Style::default()
        }
    }
    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                0.25, 0.414, 0.669, 0.3,
            ))),
            ..self.active()
        }
    }
}

pub struct DiscordTextInput;
impl text_input::StyleSheet for DiscordTextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.1)),
            border_width: 2.0,
            border_radius: 3.0,
            border_color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            ..text_input::Style::default()
        }
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgba(1.0, 1.0, 1.0, 0.3)
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            border_color: Color::from_rgb(0.113, 0.688, 0.941),
            ..self.active()
        }
    }
    fn selection_color(&self) -> iced::Color {
        Color::from_rgb(0.3, 0.445, 0.602)
    }
}
