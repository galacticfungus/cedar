
extern crate cedar;

type Model = String;

enum Message {
    NewContent(String),
}

fn update(_: Model, message: Message) -> Model {
    match message {
        Message::NewContent(content) => content,
    }
}

fn view() -> cedar::View<Model, Message> {
    cedar::View::new()
        .field(|field| {
            field.placeholder("Text to reverse")
                .change(|s| Message::NewContent(s.chars().rev().collect()))
        })
        .label(|label| label.text(Model::clone))
}

fn main() {
    cedar::Application::new("--".into(), update, view).run()
}
