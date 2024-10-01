mod simple_shader_program;

use simple_shader_program::SimpleShaderProgram;

use iced::time::Instant;
use iced::widget::shader::wgpu;
use iced::widget::{
    center, checkbox, column, row, scrollable, shader, slider, text,
};
use iced::window;
use iced::{Center, Color, Element, Fill, Subscription};

fn main() -> iced::Result {
    iced::application(
        "Scrollable Custom Shader - Iced",
        Application::update,
        Application::view,
    )
    .run()
}

#[derive(Debug, Clone)]
enum Message {}

struct Application {}

impl Application {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let t0 = Element::from(
            shader(SimpleShaderProgram {image: "ice_cube_normal_map".to_owned()})
                .width(800)
                .height(800),
        );
        let t1 = Element::from(
            shader(SimpleShaderProgram {image: "tiger".to_owned()})
                .width(800)
                .height(800),
        );
        let t2 = Element::from(
            shader(SimpleShaderProgram {image: "ice_cube_normal_map".to_owned()})
                .width(800)
                .height(800),
        );
        let t3 = Element::from(
            shader(SimpleShaderProgram {image: "tiger".to_owned()})
                .width(800)
                .height(800),
        );

        let list = column![t0, t1, t2, t3].spacing(10);
        scrollable(list).width(iced::Length::Fill).into()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
