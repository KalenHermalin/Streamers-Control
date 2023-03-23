use iced::mouse::Button;
use iced_native::layout::{self, Layout};
use iced_native::renderer;
//use iced_graphics::renderer;
use iced_native::widget::{self, Widget};
use iced_native::{Color, Element, Length, Point, Rectangle, Size};

pub struct ControlButton{
    size: f32,
    radius: f32,
    border_width: f32,
    offset: [f32; 2],
    name: String,
}

pub struct PluginButton<'a, Message>{
    width: f32,
    height: f32,
    data: String,
    on_drag: Option<Box<dyn Fn(DragEvent) -> Message +'a>>,
}



#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct PluginButtonState {
    pub is_pressed: bool,
    pub offset: [f32; 2],
}

#[derive(Debug, Clone)]
pub enum DragEvent {

    Dropped {
        data: String,
        targetButton: Point
    },

}



impl PluginButtonState {
    pub fn new() -> PluginButtonState {
        PluginButtonState::default()
    }
}


impl<'a, Message> PluginButton<'a, Message> {
    pub fn new(width: f32, height: f32, data: String) -> Self {
        Self {
            width,
            height,
            on_drag: None,
            data
        }
    }

    pub fn on_drag<F>(mut self, f:F) -> Self
    where
        F: 'a + Fn(DragEvent) -> Message,
    {
        self.on_drag = Some(Box::new(f));
        self
    }
    /*pub fn on_drag(mut self, msg: Message) -> Self {
        self.on_drag = Some(msg);
        self
    }*/

    
}

impl ControlButton{
    pub fn new(size: f32, radius: f32, border_width: f32, on_drop: impl Fn() -> String + 'static) -> Self {
        Self {
            size,
            radius,
            border_width,
            offset: [0.0, 0.0],
            name: "".to_string(),

        }
    }

}


fn font_best_size(content: &str ) -> f32 {
    if content.len() < 11{
        20.0 - 2.0 * (content.len() as f32 - 7.0)
    } else {
        12.0
    }
}
impl<Message, Renderer> Widget<Message, Renderer> for ControlButton 
where
    Renderer: renderer::Renderer + iced_native::text::Renderer,
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
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let draw = |renderer: &mut Renderer |  { 
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: self.radius.into(),
                    border_width: self.border_width,
                    border_color: Color::from_rgb(1.0, 0.0, 0.0),
                },
                Color::from_rgb(0.11, 0.11, 0.11), 
            );

            iced_native::text::Renderer::fill_text(
                renderer, 
                iced_native::text::Text{
                    content: &self.name,
                    bounds: Rectangle {
                        x: layout.bounds().center_x(),
                        y: layout.bounds().center_y(),
                        width: layout.bounds().width, 
                        height: layout.bounds().height},
                        size: font_best_size(&self.name),
                    color: Color::BLACK,
                    font: Default::default(),
                    horizontal_alignment: iced_native::alignment::Horizontal::Center,
                    vertical_alignment: iced_native::alignment::Vertical::Center,
                });

        };

        draw(renderer);
    }

    fn on_event(
        &mut self,_state: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        _shell: &mut iced_native::Shell<'_, Message>,
        ) -> iced::event::Status {
        match event {
            _ => {}
        }
        iced::event::Status::Captured 
    } 
}

impl<'a, Message, Renderer> From<ControlButton> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + iced_native::text::Renderer,
{
    fn from(circle: ControlButton) -> Self {
        Self::new(circle)
    }
}


impl<'a, Message, Renderer> Widget<Message, Renderer> for PluginButton<'a, Message>
where
    Renderer: renderer::Renderer + iced_native::text::Renderer,
    Message: Clone 
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<PluginButtonState>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(PluginButtonState::new())
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        draw_plugin_button(renderer, layout.bounds(), cursor_position, || tree.state.downcast_ref::<PluginButtonState>())
    }
    fn on_event(
        &mut self, 
        tree: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
        ) -> iced::event::Status {
        update(event, layout, cursor_position, shell, &self.on_drag, || tree.state.downcast_mut::<PluginButtonState>())
    }
}

pub fn update<'a, Message: Clone>(
    event: iced::Event,
    layout: Layout<'_>,
    cursor_position: Point,
    shell: &mut iced_native::Shell<'_, Message>,
    on_drag: &Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
    state: impl FnOnce() -> &'a mut PluginButtonState,
    ) -> iced::event::Status {
    match event {
        iced::Event::Mouse(iced::mouse::Event::ButtonPressed(Button::Left)) => {
            if layout.bounds().contains(cursor_position) {
                if layout.bounds().contains(cursor_position) {
                    let state = state();
                    state.is_pressed = true;
                }
            }
        },
        iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
            let state = state();
            if state.is_pressed {
                state.offset = [position.x - layout.bounds().center_x(), position.y - layout.bounds().center_y()];
            }
        },
        iced::Event::Mouse(iced::mouse::Event::ButtonReleased(Button::Left)) => {
                let state = state();
                if state.is_pressed {
                    println!("Hell");
                    state.is_pressed = false;
                    if on_drag.is_some() {
                        if let Some(on_drag) = on_drag.clone() {
                            shell.publish(on_drag(DragEvent::Dropped { data: "hell".to_string(), targetButton: cursor_position }));
                        }
                    }
                }
        },
        _ => {},
    }
    iced::event::Status::Captured
}
fn draw_plugin_button<'a, Renderer: renderer::Renderer + iced_native::text::Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    _cursor_position: Point,
    state: impl FnOnce() -> &'a PluginButtonState,
    ) {
    let state = state();
    let draw = |renderer: &mut Renderer |  { 
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: 2.0.into(),
                border_width: 1.0,
                border_color: Color::WHITE,
            },
            Color::from_rgb(0.09, 0.09, 0.09), 
        );
    };
    if state.is_pressed {
        renderer.with_translation(state.offset.into(), |renderer| {
            draw(renderer);
        });
    } else {
        draw(renderer);
    }
}

impl<'a, Message, Renderer> From<PluginButton<'a, Message>> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + iced_native::text::Renderer,
    Message: Clone + 'a,
{
    fn from(circle: PluginButton<'a, Message>) -> Self {
use iced::mouse::Button;
use iced_native::layout::{self, Layout};
use iced_native::renderer;
//use iced_graphics::renderer;
use iced_native::widget::{self, Widget};
use iced_native::{Color, Element, Length, Point, Rectangle, Size};

pub struct ControlButton{
    size: f32,
    radius: f32,
    border_width: f32,
    offset: [f32; 2],
    name: String,
}

pub struct PluginButton<'a, Message>{
    width: f32,
    height: f32,
    data: String,
    on_drag: Option<Box<dyn Fn(DragEvent) -> Message +'a>>,
}



#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct PluginButtonState {
    pub is_pressed: bool,
    pub offset: [f32; 2],
}

#[derive(Debug, Clone)]
pub enum DragEvent {

    Dropped {
        data: String,
        targetButton: Point
    },

}



impl PluginButtonState {
    pub fn new() -> PluginButtonState {
        PluginButtonState::default()
    }
}


impl<'a, Message> PluginButton<'a, Message> {
    pub fn new(width: f32, height: f32, data: String) -> Self {
        Self {
            width,
            height,
            on_drag: None,
            data
        }
    }

    pub fn on_drag<F>(mut self, f:F) -> Self
    where
        F: 'a + Fn(DragEvent) -> Message,
    {
        self.on_drag = Some(Box::new(f));
        self
    }
    /*pub fn on_drag(mut self, msg: Message) -> Self {
        self.on_drag = Some(msg);
        self
    }*/

    
}

impl ControlButton{
    pub fn new(size: f32, radius: f32, border_width: f32, on_drop: impl Fn() -> String + 'static) -> Self {
        Self {
            size,
            radius,
            border_width,
            offset: [0.0, 0.0],
            name: "".to_string(),

        }
    }

}


fn font_best_size(content: &str ) -> f32 {
    if content.len() < 11{
        20.0 - 2.0 * (content.len() as f32 - 7.0)
    } else {
        12.0
    }
}
impl<Message, Renderer> Widget<Message, Renderer> for ControlButton 
where
    Renderer: renderer::Renderer + iced_native::text::Renderer,
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
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let draw = |renderer: &mut Renderer |  { 
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: self.radius.into(),
                    border_width: self.border_width,
                    border_color: Color::from_rgb(1.0, 0.0, 0.0),
                },
                Color::from_rgb(0.11, 0.11, 0.11), 
            );

            iced_native::text::Renderer::fill_text(
                renderer, 
                iced_native::text::Text{
                    content: &self.name,
                    bounds: Rectangle {
                        x: layout.bounds().center_x(),
                        y: layout.bounds().center_y(),
                        width: layout.bounds().width, 
                        height: layout.bounds().height},
                        size: font_best_size(&self.name),
                    color: Color::BLACK,
                    font: Default::default(),
                    horizontal_alignment: iced_native::alignment::Horizontal::Center,
                    vertical_alignment: iced_native::alignment::Vertical::Center,
                });

        };

        draw(renderer);
    }

    fn on_event(
        &mut self,_state: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        _shell: &mut iced_native::Shell<'_, Message>,
        ) -> iced::event::Status {
        match event {
            _ => {}
        }
        iced::event::Status::Captured 
    } 
}

impl<'a, Message, Renderer> From<ControlButton> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + iced_native::text::Renderer,
{
    fn from(circle: ControlButton) -> Self {
        Self::new(circle)
    }
}


impl<'a, Message, Renderer> Widget<Message, Renderer> for PluginButton<'a, Message>
where
    Renderer: renderer::Renderer + iced_native::text::Renderer,
    Message: Clone 
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<PluginButtonState>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(PluginButtonState::new())
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        draw_plugin_button(renderer, layout.bounds(), cursor_position, || tree.state.downcast_ref::<PluginButtonState>())
    }
    fn on_event(
        &mut self, 
        tree: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
        ) -> iced::event::Status {
        update(event, layout, cursor_position, shell, &self.on_drag, || tree.state.downcast_mut::<PluginButtonState>())
    }
}

pub fn update<'a, Message: Clone>(
    event: iced::Event,
    layout: Layout<'_>,
    cursor_position: Point,
    shell: &mut iced_native::Shell<'_, Message>,
    on_drag: &Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
    state: impl FnOnce() -> &'a mut PluginButtonState,
    ) -> iced::event::Status {
    match event {
        iced::Event::Mouse(iced::mouse::Event::ButtonPressed(Button::Left)) => {
            if layout.bounds().contains(cursor_position) {
                if on_drag.is_some() {
                    if let Some(on_drag) = on_drag.clone() {
                        println!("HELLO THERE");
                        if layout.bounds().contains(cursor_position) {
                            let state = state();
                            state.is_pressed = true;
                            shell.publish(on_drag(DragEvent::Dropped { data: "hell".to_string(), targetButton: cursor_position }));

                        }
                    }
                }
            }
        },
        iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
            let state = state();
            if state.is_pressed {
                state.offset = [position.x - layout.bounds().center_x(), position.y - layout.bounds().center_y()];
            }
        }
        iced::Event::Mouse(iced::mouse::Event::ButtonReleased(Button::Left)) => {
            let state = state();
            if state.is_pressed {
                state.is_pressed = false;
            }
        }
        _ => {},
    }
    iced::event::Status::Captured
}
fn draw_plugin_button<'a, Renderer: renderer::Renderer + iced_native::text::Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    _cursor_position: Point,
    state: impl FnOnce() -> &'a PluginButtonState,
    ) {
    let state = state();
    let draw = |renderer: &mut Renderer |  { 
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: 2.0.into(),
                border_width: 1.0,
                border_color: Color::WHITE,
            },
            Color::from_rgb(0.09, 0.09, 0.09), 
        );
    };
    if state.is_pressed {
        renderer.with_translation(state.offset.into(), |renderer| {
            draw(renderer);
        });
    } else {
        draw(renderer);
    }
}

impl<'a, Message, Renderer> From<PluginButton<'a, Message>> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + iced_native::text::Renderer,
    Message: Clone + 'a,
{
    fn from(circle: PluginButton<'a, Message>) -> Self {
        Self::new(circle)
    }
}
        Self::new(circle)
    }
}
