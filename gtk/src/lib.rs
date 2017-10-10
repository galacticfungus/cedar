
extern crate gtk;
extern crate crossbeam;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

use std::sync::Arc;
use crossbeam::sync::MsQueue;

pub struct Renderer {
    pub incoming: Arc<MsQueue<String>>,
    pub outgoing: Arc<MsQueue<String>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            incoming: Arc::new(MsQueue::new()),
            outgoing: Arc::new(MsQueue::new()),
        }
    }
}

pub fn run(interconnect: Renderer) {
    // let interconnect = Box::new(interconnect);
    // unsafe { bindings::run(Box::into_raw(interconnect) as *mut c_void) }

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("** cedar **");
    window.set_default_size(500, 500);

    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        println!("Quit!");

        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    gtk::timeout_add(16, move || {
        if let Some(command) = interconnect.incoming.try_pop() {
            println!("Command: {:?}", command);
        }

        gtk::Continue(true)
    });

    gtk::main();
}
