use iced::{Background, Color, Element, Theme, widget::{Column, button, text}};
use iced_widget::Row;

use crate::colors::HSV;

mod colors;
mod canvas_elements; 

#[derive(Default, Debug, Copy, Clone)]
struct IcedPallete {
    current_color: HSV,
}

impl IcedPallete {
    
    fn update(&mut self, message: Message) {
        match message {
            Message::ColorUpdated(hsv) => {
                self.current_color = hsv
            },
            Message::_None => {},
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let color = self.current_color.to_rgb_u8();
        let color_f32 = self.current_color.to_rgb();
        Column::new()
            .push(
            text("Iced")
            )
            .push(
                text(format!("Current Color: R: {} G: {} B: {}", color.0, color.1, color.2))
            )
            .push(
                text(format!("HEX: #{:02X}{:02X}{:02X}", color.0, color.1, color.2))
            )
            .push(text(format!("HSV: H: {} S: {} V: {}", 
                (self.current_color.hue / 360.0 * 100.0).round() / 100.0, 
                (self.current_color.saturation * 100.0).round() / 100.0, 
                (self.current_color.value * 100.0).round() / 100.0))
            )
            .push(Row::new()
                .push(
                    button("     ").style(
                        move |_: &Theme, _status| {
                            button::Style { 
                                background: Some(Background::Color(Color::from_rgb(color_f32.0, color_f32.1, color_f32.2))), 
                                text_color: Color::from_rgb(color_f32.0, color_f32.1, color_f32.2), 
                                border: iced::Border::default().color(Color::from_rgb(color_f32.0, color_f32.1, color_f32.2)), 
                                snap: true,
                                ..Default::default()
                            }
                        })
                        .width(iced::Length::Fixed(100.0))
                        .height(iced::Length::Fixed(100.0))
                ).push(
                    button("     ").style(
                        move |_: &Theme, _status| {
                            button::Style { 
                                background: Some(Background::Color(Color::from_rgb(color_f32.0, color_f32.1, color_f32.2))), 
                                text_color: Color::from_rgb(color_f32.0, color_f32.1, color_f32.2), 
                                border: iced::Border::default().color(Color::from_rgb(color_f32.0, color_f32.1, color_f32.2)), 
                                snap: true,
                                ..Default::default()
                            }
                        })
                        .width(iced::Length::Fixed(100.0))
                        .height(iced::Length::Fixed(100.0))
                )

            )
            .push(
                canvas_elements::ColorWheel::new(256.0, Message::ColorUpdated),
                // Canvas::new(
                //     canvas_elements::ColorWheel{
                //         radius: 256.0, 
                //         selected_color: None,
                //     }
                // ).width(1024_u32).height(1024_u32)
            )
        .into()
    }

}

#[derive(Debug, Clone, Copy)]
enum Message {
    ColorUpdated(HSV),
    _None
}

fn main() -> iced::Result {
    // iced 0.14.0 boot is now init parameters not name
    iced::application(IcedPallete::default, IcedPallete::update, IcedPallete::view).run()
}

