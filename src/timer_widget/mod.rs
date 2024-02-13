mod imp;

use gtk::{
    glib::{self},
    prelude::*,
    subclass::prelude::*,
};

use std::time::Duration;

glib::wrapper! {
    pub struct TimerWidget(ObjectSubclass<imp::TimerWidget>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Scrollable;
}


impl TimerWidget {
    pub fn tick(&self) {
        if self.is_running() {
            let update_next_frame_callback = glib::clone!(@weak self as widget => move || {
                if widget.is_running() {
                    if widget.imp().seconds.get() > 0 {
                        widget.imp().seconds.replace(widget.imp().seconds.get() - 1);
                    } else {
                        widget.stop();
                    }
                    widget.queue_draw();
                    widget.tick();
                }
            });
            glib::timeout_add_local_once(Duration::from_millis(1000), update_next_frame_callback);
        }
    }
    
    pub fn increase_by(&self, seconds: usize) {
        let imp = self.imp();
        if (imp.seconds.get() + seconds) < 3600 {   
            imp.seconds.replace(imp.seconds.get() + seconds);
        } else {
            imp.seconds.replace(3600);
        }
        self.queue_draw();
    }
    
    pub fn reset(&self, seconds: usize) {
        let imp = self.imp();
        imp.seconds.replace(seconds);
        if imp.seconds.get() == 0 {
            self.stop();
        }
        self.queue_draw();
    }

    pub fn stop(&self) {
        let imp = self.imp();
        imp.running.replace(false);
        self.queue_draw();

    }

    pub fn start(&self) {
        let imp = self.imp();
        imp.running.replace(true);
        self.tick();
        self.queue_draw();
    }

    pub fn is_running(&self) -> bool {
        let imp = self.imp();
        imp.running.get() 
    }

    pub fn start_stop(&self) {
        let _imp = self.imp();
        if self.is_running() {
            self.stop();
        } else {
            self.start();
        }
    }

}