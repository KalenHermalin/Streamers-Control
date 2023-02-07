use iced::widget::{container, row, column, vertical_space, vertical_rule, rule, self};
use iced::{executor, Application, Command, Length, Settings, Theme, Element, Color, theme};
mod streamer_control_button;
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
    NativeEventOccured(iced_native::Event),
    DragStarted,
}

struct App {
    theme: Theme,
    currently_dragged: Option<String>,
} 

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
       (App {
           theme: Theme::Dark,
           currently_dragged: None,
       }, Command::none()) 
    }

    fn title(&self) -> String {
        "Streamers Control".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NativeEventOccured(_event) => {},
            Message::DragStarted => {
                println!("DragStarted");
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        iced_lazy::responsive(|size| {
            let button_area = container(
                column![
                    row![ 
                    ].spacing(10),
                    vertical_space(Length::Units(10)), 
                    row![ 
                    ].spacing(10),
                    vertical_space(Length::Units(10)), 
                    row![ 
                    ].spacing(10),
                    
                ]
            )
            .width(Length::FillPortion(4))
            .height(Length::Fill)
            .center_x()
            .center_y();
            let plugin_area = widget::column![
                streamer_control_button::PluginButton::new(256.0 + (size.width / 256.0) * 10.0, 50.0, "Hello".to_string()).on_drag(Message::DragStarted),
                ]
                .height(Length::Fill).width(Length::Units(256 + ((size.width / 256.0) * 10.0) as u16)).spacing(2);
            row![
                button_area,
                vertical_rule(2).style(theme::Rule::Custom(Box::new(CustomStyle {}))),
                plugin_area,
            ].width(Length::Fill).height(Length::Fill)
        }.into()).into()
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced_native::subscription::events().map(Message::NativeEventOccured)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

}
struct CustomStyle {}


impl rule::StyleSheet for CustomStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {color: Color::WHITE, width: 1, radius: 0.0, fill_mode: rule::FillMode::Full }
    }
}
