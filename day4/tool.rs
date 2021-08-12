use gtk::cairo::{Context, Format, ImageSurface};
use gtk::prelude::*;
use std::f64::consts::PI;
//gtk 添加圆角
pub fn add_corner(window: &gtk::ApplicationWindow, radius: f64) {
    window.connect_configure_event(move |window, _| {
        //清除绘制
        gdk::Window::invalidate_rect(&window.window().unwrap(), Some(&window.allocation()), false);
        let (width, height) = window.size();
        let surface = ImageSurface::create(Format::ARgb32, width + 5, height + 53)
            .expect("Can't create surface");
        let cr = Context::new(&surface).expect("Can't create a Cairo context");
        // Examples are in 1.0 x 1.0 coordinate space

        //cr.scale(width as f64, height as f64+ 50.0);
        //// Drawing code goes here
        //cr.set_line_width(0.0);
        //cr.set_source_rgb(0.0, 0.0, 0.0);
        //cr.arc(0.5, 0.5, 0.5, 0.0, 360.0);
        ////cr.rectangle(0.25, 0.25, 0.5, 0.5);
        //cr.fill().expect("sss");
        //cr.stroke().expect("Invalid cairo surface state");

        //
        //cr.arc(xc, yc, radius, angle1, angle2)
        cr.scale(width as f64 + 5.0, height as f64 + 53.0);
        // Drawing code goes here
        cr.set_line_width(0.1);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(radius, 0.0);
        cr.line_to(1.0 - radius, 0.0);
        //cr.stroke().expect("Invalid cairo surface state");
        //cr.arc(0.1, 0.1, 0.1, PI, PI*3.0/2.0);
        cr.move_to(1.0, radius);
        cr.line_to(1.0, 1.0 - radius);
        //cr.stroke().expect("Invalid cairo surface state");
        //cr.arc(0.1, 0.9, 0.1, PI/2.0, PI);
        cr.move_to(1.0 - radius, 1.0);
        cr.line_to(radius, 1.0);
        //cr.stroke().expect("Invalid cairo surface state");
        //cr.arc(0.9, 0.9, 0.1, 0.0, PI/2.0);
        cr.move_to(0.0, 1.0 - radius);
        cr.line_to(0.0, radius);
        //cr.stroke().expect("Invalid cairo surface state");

        cr.arc(radius, radius, radius, PI, PI * 3.0 / 2.0);
        cr.arc(1.0 - radius, radius, radius, -PI / 2.0, 0.0);

        cr.arc(1.0 - radius, 1.0 - radius, radius, 0.0, PI / 2.0);
        cr.arc(radius, 1.0 - radius, radius, PI / 2.0, PI);

        //cr.stroke_preserve().unwrap();
        cr.fill().expect("sss");
        cr.stroke().expect("Invalid cairo surface state");

        let temp = surface.create_region();
        window.shape_combine_region(Some(&temp.unwrap()));
        false
    });
    window.set_opacity(0.0);
}
