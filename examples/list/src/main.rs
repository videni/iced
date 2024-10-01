use iced::widget::{
    button, center, column, container, horizontal_space, list, row, scrollable,
    text,
};
use iced::{settings, Alignment, Color, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    iced::application("List - Iced", List::update, List::view)
        .settings( Settings {
            // debug: true, 
            ..settings::Settings::default()
        } )
        .theme(|_| Theme::TokyoNight)
        .run()
}

struct List {
    content: list::Content<(usize, State)>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Update(usize),
    Remove(usize),
}

impl List {
    fn update(&mut self, message: Message) {
        match message {
            Message::Update(index) => {
                if let Some((_id, state)) = self.content.get_mut(index) {
                    *state = State::Updated;
                }
            }
            Message::Remove(index) => {
                let _ = self.content.remove(index);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // return center(square(400, 100)).into();

        center(
            scrollable(
                list(&self.content, |index, (id, state)| {
                    row![
                        square(400, index as u32),
                        // match state {
                        //     State::Idle =>
                        //         Element::from(text(format!("I am item {id}!"))),
                        //     State::Updated => center(
                        //         column![
                        //             text(format!("I am item {id}!")),
                        //             text("... but different!")
                        //         ]
                        //         .spacing(20)
                        //     )
                        //     .height(300)
                        //     .into(),
                        // },
                        // horizontal_space(),
                        // button("Update").on_press_maybe(
                        //     matches!(state, State::Idle)
                        //         .then_some(Message::Update(index))
                        // ),
                        // button("Remove")
                        //     .on_press(Message::Remove(index))
                        //     .style(button::danger)
                    ]
                    .spacing(10)
                    .padding(5)
                    // .align_items(Alignment::Center)
                    .into()
                    // square(50.0*(index as f32 + 1.0), index as u32)
                    // .explain(Color::new(1.0, 0.0,0.0, 0.0))
                }).spacing(10)
            )
            .width(Length::Fill)
            ,
        )
        .padding(10)
        .into()
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            content: list::Content::from_iter(
                (0..3).map(|id| (id, State::Idle)),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Updated,
}


use iced::widget::canvas::{Canvas, Geometry, Frame, Program, Text};
use iced::{
     Point, Rectangle, Renderer,
};
use iced::mouse;

fn square<'a>(size: impl Into<Length> + Copy, text: u32) -> Element<'a, Message> {
    struct Square {
        text: String
    }

    impl Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<Geometry> {
            let mut frame = Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

    
            frame.fill_text(self.text.as_str());

            vec![frame.into_geometry()]
        }
    }

    Canvas::new(Square{ text: text.to_string()}).width(size).height(size).into()
}