use fltk::{enums::*, prelude::*, *};
use fltk_theme::colors::html::*;

macro_rules! col {
    ($e: expr) => {
        Color::from_rgb($e.0, $e.1, $e.2)
    }
}

fn main() {
    let bg = *HTML_MAP.get("SeaShell").unwrap();

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(bg.0, bg.1, bg.2);

    let mut win = window::Window::default().with_size(800, 600);
    {
        let scroll = group::Scroll::default_fill().with_type(group::ScrollType::VerticalAlways);
        {
            let mut scrollbar = scroll.scrollbar();
            scrollbar.set_type(valuator::ScrollbarType::VerticalNice);
            let mut pack = group::Pack::default()
                .with_size(700, 600)
                .center_of_parent();
            pack.set_spacing(5);
            {
                for i in HTML_MAP.iter() {
                    let mut frame = frame::Frame::default().with_size(0, 40).with_label(i.0);
                    frame.set_frame(FrameType::RFlatBox);
                    frame.set_color(col!(*i.1));
                }
            }
            pack.end();
        }
        scroll.end();
    }
    win.end();
    win.show();
    a.run().unwrap();
}