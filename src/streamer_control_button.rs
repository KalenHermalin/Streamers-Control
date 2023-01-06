use iced::mouse::Button;
use iced_native::layout::{self, Layout};
use iced_native::renderer;
    use iced_native::widget::{self, Widget};
    use iced_native::{Color, Element, Length, Point, Rectangle, Size};

    pub struct StreamerButton{
        size: f32,
        radius: f32,
        border_width: f32,
        mouse_state: MouseState,
        on_drop: Option<Box<dyn Fn() -> String>>,
        
    }

    #[derive(Default, Clone, Copy, PartialEq, Debug)]
    pub struct MouseState {
        pub pressed: bool,
    }
    impl StreamerButton{
        pub fn new(size: f32, radius: f32, border_width: f32, on_drop: impl Fn() -> String + 'static) -> Self {
            Self {
                size,
                radius,
                border_width,
                mouse_state: Default::default(),
                on_drop: Some(Box::new(on_drop)),

            }
        }

    }

    impl<Message, Renderer> Widget<Message, Renderer> for StreamerButton 
    where
        Renderer: renderer::Renderer,
    {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(
            &self,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(self.size, self.size))
        }


        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Renderer::Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            cursor_position: Point,
            _viewport: &Rectangle,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: self.radius.into(),
                    border_width: self.border_width,
                    border_color: Color::from_rgb(1.0, 0.0, 0.0),
                },
               Color::from_rgb(0.11, 0.11, 0.11), 
            );
        }


       fn on_event(
               &mut self,
               _state: &mut widget::Tree,
               event: iced::Event,
               layout: Layout<'_>,
               cursor_position: Point,
               _renderer: &Renderer,
               _clipboard: &mut dyn iced_native::Clipboard,
               _shell: &mut iced_native::Shell<'_, Message>,
           ) -> iced::event::Status {
           match event {
               iced::Event::Mouse(iced::mouse::Event::ButtonPressed(Button::Left)) => {
                   if layout.bounds().contains(cursor_position) {
                        println!("Pressed On StreamerButton");
                        self.mouse_state.pressed = true;
                   }
               },
               iced::Event::Mouse(iced::mouse::Event::ButtonReleased(Button::Left)) => {
                   if layout.bounds().contains(cursor_position) {
                       match &self.on_drop {
                           Some(callback) => { 
                               let data = callback();
                               println!("{}", data);
                           },
                           None => {},
                           
                       }
                       self.mouse_state.pressed = false;
                   }
               },
               _ => {}
           }
          iced::event::Status::Captured 
       } 
    }

    impl<'a, Message, Renderer> From<StreamerButton> for Element<'a, Message, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(circle: StreamerButton) -> Self {
            Self::new(circle)
        }
    }
