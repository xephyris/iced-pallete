use std::f32::consts::PI;

use iced::{Element, Task, widget::{Column, text}};

mod colors;

#[derive(Default, Debug, Copy, Clone)]
struct IcedPallete {
    
}

impl IcedPallete {
    
    fn update(&mut self, message: Message) {

    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(
            text("Iced"))
            // .push(
            // iced::widget::canvas::gradient::Linear::new(0.0, (2*PI))
            // )
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
