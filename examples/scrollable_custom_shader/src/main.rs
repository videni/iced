mod simple_shader_program;

use iced::Length::Shrink;
use simple_shader_program::SimpleShaderProgram;

use iced::time::Instant;
use iced::widget::shader::wgpu;
use iced::widget::{
    center, checkbox, column, list, image, row, scrollable, shader, slider, text
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

struct Application {
    content: list::Content<String>
}

impl Application {
    fn new() -> Self {
        let content =list::Content::<String>::with_items(vec!["ice_cube_normal_map".to_owned(), "tiger".to_owned()]); 

        Self {
            content
        }
    }

    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let t0 = 
            shader(SimpleShaderProgram {image: "ice_cube_normal_map".to_owned()})
                .width(800)
                .height(700);
        let t1 = 
            shader(SimpleShaderProgram {image: "tiger".to_owned()})
                .width(800)
                .height(700);
        // let t2 = 
        //     shader(SimpleShaderProgram {image: "ice_cube_normal_map".to_owned()})
        //         .width(800)
        //         .height(800);
        // let t3 = 
        //     shader(SimpleShaderProgram {image: "tiger".to_owned()})
        //         .width(800)
        //         .height(800);
        // let list = column![t0, t1].spacing(10);
        // let list = list::List::new(&self.content, move|_index, image_name| {
        //         shader(SimpleShaderProgram {image: image_name.clone()})
        //         .width(400)
        //         .height(400)
        //         .into()
        //     }
        // )
        // .spacing(10);

        // dbg!("tiger");
        // let t = image(format!(
        //         "{}/resources/tiger.png",
        //         env!("CARGO_MANIFEST_DIR")
        //     )).opacity(0.9);
        
        // dbg!("ice_cube_normal_map");

        // let t1 = 
        //     image(format!(
        //         "{}/resources/ice_cube_normal_map.png",
        //         env!("CARGO_MANIFEST_DIR")
        //     )
        // );
        
        let list = scrollable(column![t0, t1])
            .width(iced::Length::Fill);

        column![text("I am header"), list].into()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
