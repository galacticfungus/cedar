
use dom;
use std::fmt::Debug;

use super::{Window, Label, Stack, Button, TextField};
use super::widget::Widget;

use stream::Stream;
use atomic_box::AtomicBox;

struct Vertex<S> {
    widget: AtomicBox<Box<Widget<S>>>,
    children: Vec<Vertex<S>>,
}

type Tree<S> = Vec<Vertex<S>>;

fn create<S: Clone + 'static>(stream: Stream<S>, node: dom::Object<S>) -> Vertex<S> {
    let (kind, attributes) = (node.value.0, node.value.1);
    let mut widget: Box<Widget<S>> = match kind {
        dom::Kind::Label => Box::new(Label::new()),
        dom::Kind::Button => Box::new(Button::new(stream.clone())), 
        dom::Kind::Stack => Box::new(Stack::new()),
        dom::Kind::Field => Box::new(TextField::new(stream.clone())),
    };

    widget.update(attributes);

    let children = node.children
        .into_iter()
        .map(|child| {
            let child = create(stream.clone(), child);
            widget.add(&child.widget);
            child
        })
        .collect();

    Vertex {
        widget: AtomicBox::new(widget),
        children,
    }
}

// TODO: use `removeFromSuperview()` to 'delete' nodes

fn patch<S: Debug>(tree: &mut Tree<S>, (mut path, op): dom::Change<S>) {
    if path.is_empty() {
        return;
    }

    let location = path.remove(0);
    if path.is_empty() {
        let widget = &mut tree[location.index].widget;

        use tree::Operation::*;
        match op {
            Update((_, attrs)) => widget.update(attrs),
            op => panic!("Not yet implemented! {:?}", op),
        }
    } else {
        patch(&mut tree[location.index].children, (path, op));
    }
}

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub fn program<S, M>(model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq + Debug,
    M: Send + 'static + Debug,
{
    let app = super::Application::new(); // TODO: enforce `app` created first

    let stream = Stream::new();

    let (_window, mut stack) = Window::new("cedar");

    let node = view(&model);

    let vertex = create(stream.clone(), node.clone());
    stack.add(&vertex.widget);

    let mut tree = vec![vertex];

    // Use `Option` to allow for move/mutation in FnMut `run`
    let mut model = Some(model);
    let mut node = Some(node);

    app.run(move || loop {
        let message = stream.pop();

        // println!("msg: {:?}", message);

        let m = update(model.take().unwrap(), message);

        let new = view(&m);

        // println!("node: {:?}", new);

        let old = node.take().unwrap();
        let changeset = dom::diff(old, new.clone());

        // println!("diff: {:?}", changeset);

        for change in changeset.into_iter() {
            patch(&mut tree, change);
        }

        node = Some(new);
        model = Some(m);
    })

}
