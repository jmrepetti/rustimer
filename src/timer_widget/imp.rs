use std::cell::{Cell};

use gtk::{
    glib::{self, clone},
    cairo, prelude::*, subclass::prelude::*};

use core::f64::consts::PI;

#[derive(Debug)]
pub struct TimerWidget {
    pub seconds: Cell<usize>,
    pub running: Cell<bool>
}

impl Default for TimerWidget {
    fn default() -> Self {
        Self {
            seconds: Cell::new(1500),
            running: Cell::new(false),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TimerWidget {
    const NAME: &'static str = "TimerWidget";
    type Type = super::TimerWidget;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for TimerWidget {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_size_request(500, 500);

        // Handle Click gesture for start/stop the timer
        let gesture = gtk::GestureClick::new();
        gesture.connect_released(clone!(@weak obj => move |_gesture, _n_press, _x, _y| {
            obj.start_stop();
        }));
        obj.add_controller(gesture);
    }

}

impl WidgetImpl for TimerWidget {

    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        let widget = self.obj();
        let rad = PI / 180.0;

        let width = widget.width() as f64;
        let height = widget.height() as f64;

        let cr = snapshot.append_cairo(&gtk::graphene::Rect::new(
            0.0,
            0.0,
            width as f32,
            height as f32,
        ));
      
        let time_left_angle = self.seconds.get() as f64 * 2.0 * PI / 3600.0;

        let xc: f64 = width / 2.0;
        let yc: f64 = height  / 2.0;
        let radius: f64 = width / 3.0;
        let angle1: f64 = -90.0 * rad; 
        let angle2: f64 = time_left_angle - 90.0 * rad;  
        
        cr.set_source_rgba(1.0,1.0,1.0, 1.0);

        let div_radio = (width - 30.0) / 2.0;

        let n = 12;
        let step = 30.0;
        cr.set_line_width(30.0);
        for i in 0..n {
            cr.arc (xc, yc, div_radio, (i as f64 * step - 1.0) as f64 * rad, (i as f64 * step + 1.0) as f64 * rad);
            cr.stroke().expect("stroke error");
        }
        
        let n = 60;
        let step = 6.0;
        cr.set_line_width(10.0);
        for i in 0..n {
            cr.arc (xc, yc, div_radio + 10.0, (i as f64 * step - 0.5) as f64 * rad, (i as f64 * step + 0.5) as f64 * rad);
            cr.stroke().expect("stroke error");
        }

        cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
        cr.set_line_width(100.0);
        cr.arc(xc, yc, radius, angle1, angle2);
        cr.stroke().expect("stroke error");

        cr.set_source_rgba(1.0,1.0,1.0, 1.0);

        let minutes_left = self.seconds.get() as i32 / 60;
        let seconds_left = self.seconds.get() % 60;
        let text = format!("{:02}.{:02}", minutes_left, seconds_left);

        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        cr.set_font_size(90.0);

        let text_x = xc - (275.0/2.0);
        let text_y = yc + (69.0/2.0);
        
        cr.move_to(text_x, text_y);
        cr.show_text(&text).expect("show_text error");

        cr.save().unwrap();

    }
}
