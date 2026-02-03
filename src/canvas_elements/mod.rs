use iced::{Point, Size, mouse};
use iced::widget::canvas;
use iced::{Color, Rectangle, Renderer, Theme};

use crate::colors::HSV;
pub struct ColorWheel {
    pub radius: f32,
    // selected_color: HSV,
}

impl<Message> canvas::Program<Message> for ColorWheel {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        
        let size = self.radius * 2.0;
        let center = (frame.center().x, frame.center().y);
        let start_x = center.0 - self.radius;
        let start_y = center.1 - self.radius;
        
        frame.fill_rectangle(Point { x: 0.0, y: 0.0 }, Size {width: frame.width(), height: frame.height()}, Color::BLACK);
        // * For drawing square color picker

        // for col in (start_y as usize)..((start_y + size) as usize) {
            // for row in (start_x as usize)..((start_x + size) as usize) {

        // * For filling whole canvas
        for col in 0..frame.height() as usize {
            for row in 0..frame.width() as usize {
                let col = col as f32;
                let row = row as f32;
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
                let s = (dist_from(center.0, center.1, row, col) / self.radius).clamp(0.0, 1.0);
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

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (f32, f32, f32) {
    let scaled_h = if hue >= 300.0 {
        (hue - 360.0) / 60.0
    } else {
        hue / 60.0
    };


    let chroma = saturation * value;
    let min = value - chroma;
    let max = value;

    let (r, g, b) = match scaled_h as i8{
        -1..1 => {
            if scaled_h - 0.0 < 0.0 {
                (max, min, min - scaled_h * chroma)
            } else {
                (max, min + scaled_h * chroma, min)
            }
        }
        1..3 => {
            if scaled_h - 2.0 < 0.0 {
                (min - (scaled_h - 2.0) * chroma, max, min)
            } else {
                (min, max, min + (scaled_h - 2.0) * chroma)
            }
        }
        3..5 => {
            if scaled_h - 4.0 < 0.0 {
                (min, min - (scaled_h - 4.0) * chroma, max)
            } else {
                (min + (scaled_h - 4.0) * chroma, min, max)
            }
        }
        _=> {
            (0.5, 0.5, 0.5)
        }
    };

    let (r, g, b) = (
        r.clamp(0.0, 1.0),
        g.clamp(0.0, 1.0),
        b.clamp(0.0, 1.0)
    );

    (r, g, b)

}