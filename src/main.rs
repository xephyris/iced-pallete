use std::f32::consts::PI;

use iced::{Element, Length, Task, widget::{Canvas, Column, canvas, text}};

mod colors;
mod canvas_elements; 

#[derive(Default, Debug, Copy, Clone)]
struct IcedPallete {
    
}

impl IcedPallete {
    
    fn update(&mut self, message: Message) {

    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(
            text("Iced")
            )
            // .push(
            // iced::widget::canvas::gradient::Linear::new(0.0, (2*PI))
            // )
            .push(
                Canvas::new(
                    canvas_elements::ColorWheel{
                        radius: 512.0 
                    }
                ).width(1024_u32).height(1024_u32)
            )
        .into()
    }

}

#[derive(Debug, Copy, Clone)]
enum Message {

}

fn main() -> iced::Result {
    // iced 0.14.0 boot is now init parameters not name
    iced::application(IcedPallete::default, IcedPallete::update, IcedPallete::view).run()
}

