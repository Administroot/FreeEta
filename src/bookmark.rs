use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::border;
use iced::mouse;
use iced::{Color, Element, Length, Rectangle, Size};

pub struct Bookmark {
    color: Color,
}

impl Bookmark {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

pub fn custom_bookmark(color: Color) -> Bookmark {
    Bookmark::new(color)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Bookmark
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        // widget{x: 120.0, y: 55.0} when screen{x: 1535.2, y: 191.2}
        // TODO: pub fn get_size(id: Id) -> Task<Size>
        layout::Node::new(Size::new(400., 400.))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(200.),
                ..renderer::Quad::default()
            },
            self.color,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Bookmark>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(circle: Bookmark) -> Self {
        Self::new(circle)
    }
}