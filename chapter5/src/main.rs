use std::fmt::Write;

const WIDTH: usize = 34;

pub trait Widget {
    /// Natural WIDTH of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.capacity()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_fmt(format_args!("|{:^WIDTH$}|\n", self.label)).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        // TODO JSP self.label.width()
        13
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        // idk callback ???
        let btn_width = self.width();
        let diff = WIDTH - btn_width - 4;   // TODO find right way
        buffer.write_str("| ").unwrap();
        buffer.write_fmt(format_args!("+{:-^btn_width$}+", "")).unwrap();
        buffer.write_fmt(format_args!("{:>diff$} |\n", "")).unwrap();

        buffer.write_str("| ").unwrap();
        buffer.write_fmt(format_args!("|{:^btn_width$}|", self.label.label)).unwrap();
        buffer.write_fmt(format_args!("{:>diff$} |\n", "")).unwrap();

        buffer.write_str("| ").unwrap();
        buffer.write_fmt(format_args!("+{:-^btn_width$}+", "")).unwrap();
        buffer.write_fmt(format_args!("{:>diff$} |\n", "")).unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        WIDTH   // idk how to find 34..
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_fmt(format_args!("+{:->WIDTH$}+\n", "")).unwrap();
        buffer.write_fmt(format_args!("|{:^WIDTH$}|\n", self.title)).unwrap();
        buffer.write_fmt(format_args!("+{:=>WIDTH$}+\n", "")).unwrap();
        for widget in &self.widgets {
            widget.draw_into(buffer);
        }
        buffer.write_fmt(format_args!("+{:->WIDTH$}+\n", "")).unwrap();
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
    /*
    +--------------------------------+
    |       Rust GUI Demo 1.23       |
    +================================+
    | This is a small text GUI demo. |
    | +-----------+                  |
    | | Click me! |                  |
    | +-----------+                  |
    +--------------------------------+
    */
}