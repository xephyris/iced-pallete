use iced::{Point, Size, mouse};
use iced::{Color, Rectangle, Renderer, Theme};
use iced_graphics::geometry::{LineCap, Path, Stroke};
use iced::widget::canvas;

use crate::colors::{HSV, hsv_to_rgb};
pub struct ColorWheel
{
    pub radius: f32,
    // selected_color: HSV
}

impl<Message> canvas::Program<Message> for ColorWheel {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        
        let size = self.radius * 2.0;
        let center = (frame.center().x, frame.center().y);
        let start_x = center.0 - self.radius;
        let start_y = center.1 - self.radius;
        
        // frame.fill_rectangle(Point { x: 0.0, y: 0.0 }, Size {width: frame.width(), height: frame.height()}, Color::BLACK);
        // * For drawing square color picker

        // for col in (start_y as usize)..((start_y + size) as usize) {
            // for row in (start_x as usize)..((start_x + size) as usize) {

        // * For filling whole canvas
        for col in 0..frame.height() as usize {
            for row in 0..frame.width() as usize {
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

        // Circle around to make image look less jagged
        // Canvas doesn't do anti-aliasing for direct pixel manipulation

        let center = frame.center();

        let line_width = (5.0 * self.radius / 512.0).clamp(1.0, 5.0);

        let circle = Path::circle(center, self.radius);

        let stroke = Stroke {
            width: line_width,
            style: canvas::Style::Solid(theme.palette().background), 
            line_cap: LineCap::Round,
            ..Default::default()
        };

        frame.stroke(&circle, stroke);

        vec![frame.into_geometry()]
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


