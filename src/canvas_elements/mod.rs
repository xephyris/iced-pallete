// use iced::{Point, Size, mouse};
// use iced::{Color, Rectangle, Renderer, Theme};
use iced_core::widget::{Tree, Widget, tree};
use iced_core::{Color, Element, Length, Point, Rectangle, Size, layout, mouse};
use iced_graphics::geometry::{self, Frame, LineCap, Path, Stroke};

use crate::colors::{HSV, hsv_to_rgb, position_to_hsv};
pub struct ColorWheel<'a, Message>
{
    radius: f32,
    width: Length,
    height: Length,
    selected_color: Option<Selector>,
    on_select: Box<dyn Fn(HSV) -> Message + 'a>,

}

impl<'a, Message> ColorWheel<'a, Message> {
    pub fn new(radius: f32, on_select: impl Fn(HSV) -> Message + 'a) -> Self {
        ColorWheel { 
            radius, 
            width: Length::Fill, 
            height: Length::Fill, 
            selected_color: None, 
            on_select: Box::new(on_select),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> 
    for ColorWheel<'a, Message> 
    where 
        Renderer: geometry::Renderer + 'static,
        Theme: iced_core::theme::Base + 'a,
    {
    
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Renderer>::default())
    }

    fn mouse_interaction(
            &self,
            _tree: &Tree,
            layout: layout::Layout<'_>,
            cursor: mouse::Cursor,
            _viewport: &Rectangle,
            _renderer: &Renderer,
        ) -> mouse::Interaction {
        if let Some(position) = cursor.position_from(layout.bounds().center()) {
            // dbg!(f32::sqrt(position.x.powi(2) + position.y.powi(2)));
            if f32::sqrt(position.x.powi(2) + position.y.powi(2)) < self.radius {
                return mouse::Interaction::Crosshair;
            }
        }
        Default::default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &iced_core::renderer::Style,
        layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let State {
            wheel_cache,
            selector_cache,
        }: &State<Renderer> = tree.state.downcast_ref();

        let bounds = layout.bounds();
        let size = bounds.size();

        renderer.with_layer(bounds, |renderer| {
            let color_wheel = wheel_cache.draw(renderer, size, |frame| {
                let size = self.radius * 2.0;
                let center = (layout.bounds().center().x, layout.bounds().center().y);
                let start_x = center.0 - self.radius;
                let start_y = center.1 - self.radius;
                
                // frame.fill_rectangle(Point { x: 0.0, y: 0.0 }, Size {width: frame.width(), height: frame.height()}, Color::BLACK);
                // * For filling whole canvas
                // for col in 0..frame.height() as usize {
                    // for row in 0..frame.width() as usize {
                
                // * For drawing square color selector

                for col in (start_y as usize)..((start_y + size) as usize) {
                    for row in (start_x as usize)..((start_x + size) as usize) {

                        let col = col as f32;
                        let row = row as f32;
                        let dist = dist_from(center.0, center.1, row, col);
                        if dist < self.radius {
                            let mut angle = f32::atan2(col - center.1,row - center.0).to_degrees();
                            if angle < 0.0 {
                                angle += 360.0;
                            }
                            if angle < 90.0 {
                                angle += 270.0;
                            } else {
                                angle -= 90.0;
                            }

                            // dbg!(format!("H: {col} W: {row}, ANGLE: {angle}"));
                            let s = (dist / self.radius).clamp(0.0, 1.0);
                            let h = angle;
                            let v = 1.0;

                            let (r, g, b) = hsv_to_rgb(h, s, v);

                            frame.fill_rectangle(
                                Point::new(row, col),
                                Size::new(1.0, 1.0),
                                Color::from_rgb(r, g, b),
                            );
                        }
                    }
                }

                // frame.fill_rectangle(
                //     Point::new(center.0 - self.radius / 10.0, center.1 - self.radius / 10.0),
                //     Size::new(self.radius / 5.0, self.radius / 5.0),
                //     Color::from_rgb(0.0, 0.0, 0.0),
                // );

                // Circle around to make image look less jagged
                // Canvas doesn't do anti-aliasing for direct pixel manipulation

                let center = layout.bounds().center();

                let line_width = (5.0 * self.radius / 512.0).clamp(1.0, 5.0);

                let circle = Path::circle(center, self.radius);

                let stroke = Stroke {
                    width: line_width,
                    style: geometry::Style::Solid(theme.palette().unwrap().background), 
                    line_cap: LineCap::Round,
                    ..Default::default()
                };

                

                frame.stroke(&circle, stroke);
            });
            dbg!("Drawing Selector {}", &self.selected_color);
            let selector = selector_cache.draw(renderer, size, |frame| {
                if let Some(selector) = &self.selected_color {
                    selector.draw(frame, theme);
                }
            });
            renderer.draw_geometry(color_wheel);
            renderer.draw_geometry(selector);
        })
    }

    fn update(
            &mut self,
            tree: &mut Tree,
            event: &iced::Event,
            layout: layout::Layout<'_>,
            cursor: mouse::Cursor,
            _renderer: &Renderer,
            _clipboard: &mut dyn iced_core::Clipboard,
            shell: &mut iced_core::Shell<'_, Message>,
            _viewport: &Rectangle,
        ) {
        let State {
            wheel_cache,
            selector_cache,
        }: &mut State<Renderer> = tree.state.downcast_mut();

        match event {
            iced_core::Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(_button) => {
                        if let Some(cursor) = cursor.position_from(layout.bounds().center()) {
                            if dist_from(cursor.x, cursor.y, 0.0, 0.0) < self.radius {
                                let color = position_to_hsv(cursor.x, cursor.y, self.radius);
                                self.selected_color.replace(Selector::new(cursor.x, cursor.y, 2.0, color));
                                dbg!("Updating Drawing Selector {}", &self.selected_color);
                                selector_cache.clear();
                                let _ = shell.redraw_request();
                                shell.publish((self.on_select)(color));
                            } else {
                                ()
                            }
                        } else {
                            ()
                        }
                    }
                    _ => ()
                }
            }
            _ => ()
        }

    }
}

impl <'a, Message, Theme, Renderer> From<ColorWheel<'a, Message>> 
    for Element<'a, Message, Theme, Renderer>
    where 
        Message: 'a,
        Renderer: geometry::Renderer + 'static,
        Theme: iced_core::theme::Base + 'a,

{
    fn from(widget: ColorWheel<'a, Message>) -> Self {
        Element::new(widget)
    }
}

pub struct State<Renderer: geometry::Renderer> {
    wheel_cache: geometry::Cache<Renderer>,
    selector_cache: geometry::Cache<Renderer>,
}

impl<Renderer: geometry::Renderer> Default for State<Renderer> {
    fn default() -> Self {
        State {
            wheel_cache: Default::default(),
            selector_cache: Default::default(),
        }
    }
}

fn dist_from(x1: f32, y1: f32, x2: f32, y2: f32) -> f32{
    let dist = f32::sqrt((x1 - x2).powi(2) + (y1 - y2).powi(2));
    if dist.is_nan() {
        0.0
    } else {
        dist
    }
}


#[derive(Debug)]
pub struct Selector {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: HSV,
}

impl Selector {
    pub fn new(x: f32, y: f32, radius: f32, hsv: HSV) -> Self {
        Selector { x, y, radius, color: hsv }
    }

    pub fn draw<Renderer: geometry::Renderer>(&self, frame: &mut Frame<Renderer>, theme: &impl iced_core::theme::Base) {
        let circle = Path::circle(Point{x: self.x, y: self.y}, self.radius);
        let line_width = (3.0 * self.radius / 64.0).clamp(1.0, 3.0);

        for col in ((self.y - self.radius) as usize)..((self.y + self.radius) as usize) {
            for row in ((self.x - self.radius) as usize)..((self.y + self.radius) as usize) {
                let dist = dist_from(col as f32, row as f32, self.x, self.y);
                if dist < self.radius {
                    // dbg!(format!("H: {col} W: {row}, ANGLE: {angle}"));

                    let (r, g, b) = hsv_to_rgb(self.color.hue, self.color.saturation, self.color.value);

                    frame.fill_rectangle(
                        Point::new(row as f32, col as f32),
                        Size::new(1.0, 1.0),
                        Color::from_rgb(r, g, b),
                    );
                }
            }
        }

        let stroke = Stroke {
            width: line_width,
            style: geometry::Style::Solid(theme.palette().unwrap().background), 
            line_cap: LineCap::Round,
            ..Default::default()
        };

        frame.stroke(&circle, stroke);
    }
}




