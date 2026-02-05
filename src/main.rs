use iced::{Element, widget::{Column, text}};

use crate::colors::HSV;

mod colors;
mod canvas_elements; 

#[derive(Default, Debug, Copy, Clone)]
struct IcedPallete {

}

impl IcedPallete {
    
    fn update(&mut self, _message: Message) {

    }

    fn view(&self) -> Element<'_, Message> {
        Column::new()
            .push(
            text("Iced")
            )
            // .push(
            // iced::widget::canvas::gradient::Linear::new(0.0, (2*PI))
            // )
            .push(
                canvas_elements::ColorWheel::new(50.0, Message::Some),
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
    Some(HSV),
    _None
}

fn main() -> iced::Result {
    // iced 0.14.0 boot is now init parameters not name
    iced::application(IcedPallete::default, IcedPallete::update, IcedPallete::view).run()
}

