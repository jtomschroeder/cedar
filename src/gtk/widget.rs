
use gtk;

// pub trait Widget<M> {
//     fn add(&self, container: &gtk::Box);

//     fn update(&mut self, model: &M);
// }

use dom::Attributes;
use super::{Window, Button, Label, Stack};

pub trait Widget<S> {
    // fn id(&self) -> &Id;

    fn update(&mut self, Attributes<S>) {}

    // fn add(&self, container: &gtk::Box);

    // fn add(&self, &Box<Widget<S>>) {}
    // fn add_in(&self, &Widget<S>) {}
}

// #[derive(Debug)]
pub enum NWidget<S> {
    Button(Button<S>),
    Stack(Stack),
    Label(Label),
}

impl<S> NWidget<S> {
    pub fn add(&self, widget: &NWidget<S>) {
        match self {
            &NWidget::Button(_) => panic!(""),
            &NWidget::Stack(ref stack) => stack.add(widget),
            &NWidget::Label(_) => panic!(""),
        }
    }

    // pub fn widget<W: gtk::IsA<gtk::Widget>>(&self) -> &W {
    //     match self {
    //         &NWidget::Button(button) => return &button.button as &W,
    //         &NWidget::Stack(_) => {}
    //         &NWidget::Label(_) => {}
    //     }

    //     unimplemented!()
    // }

    pub fn update(&mut self, _: Attributes<S>) {}
}