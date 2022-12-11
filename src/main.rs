use iced::widget::{container, row, text, column, vertical_space, vertical_rule, self};
use iced::{executor, Application, Command, Length, Settings, Theme, Element, Color, theme, Background};
fn main() {
    App::run(Settings {
        window:
            iced::window::Settings {
                min_size: Some((800, 400)),
                ..iced::window::Settings::default()
            },
            ..Settings::default() 
    }).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    EventOccured(iced_native::Event),
}

struct App {
    theme: Theme,
} 

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
       (App {
           theme: Theme::Dark,
       }, Command::none()) 
    }

    fn title(&self) -> String {
        "Streamers Control".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccured(event) => {},
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        iced_lazy::responsive(|_size| {
            let button_area = container(
                column![
                    row![ 
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                    ].spacing(10),
                    vertical_space(Length::Units(10)), 
                    row![ 
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                    ].spacing(10),
                    vertical_space(Length::Units(10)), 
                    row![ 
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                        container(row![]).width(Length::Units(75)).height(Length::Units(75)).style(theme::Container::Custom(Box::new(CustomContainerStyle {}))),
                    ].spacing(10),
                ]
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();
            let plugin_area = widget::column![text("hello").size(50)].height(Length::Fill).width(Length::Units(256 + ((_size.width / 256.0) * 10.0) as u16));
            row![
                button_area,
                vertical_rule(2),
                plugin_area,
            ].width(Length::Fill).height(Length::Fill).into()

        }).into()


    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

}
struct CustomContainerStyle {}

impl container::StyleSheet for CustomContainerStyle {
    type Style = iced::Theme;
    

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance { background: Some(Background::Color(Color::from_rgb(0.11, 0.11, 0.11))), border_radius: 15.0, ..Default::default()}
    }
}
