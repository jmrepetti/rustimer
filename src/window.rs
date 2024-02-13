/* MIT License
 *
 * Copyright (c) 2024 Matias
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::timer_widget::TimerWidget;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/jmrepetti/rustimer/window.ui")]
    pub struct RustimerWindow {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub timer: TemplateChild<TimerWidget>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RustimerWindow {
        const NAME: &'static str = "RustimerWindow";
        type Type = super::RustimerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("win.add-5-minutes", None, |window, _, _| {
                window.increase_timer_by(5*60);
            });
            klass.install_action("win.add-1-minute", None, |window, _, _| {
                window.increase_timer_by(1*60);
            });
            klass.install_action("win.reset-timer", None, |window, _, _| {
                window.reset_timer();
            });
            klass.install_action("win.start-stop-timer", None, |window, _, _| {
                window.start_stop_timer();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RustimerWindow {}
    impl WidgetImpl for RustimerWindow {}
    impl WindowImpl for RustimerWindow {}
    impl ApplicationWindowImpl for RustimerWindow {}
    impl AdwApplicationWindowImpl for RustimerWindow {}
}

glib::wrapper! {
    pub struct RustimerWindow(ObjectSubclass<imp::RustimerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks(functions)]
impl RustimerWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
    pub fn increase_timer_by(&self, seconds: usize) {
        self.imp().timer.increase_by(seconds);
    }
    pub fn reset_timer(&self) {
        self.imp().timer.reset(0);
    }
    pub fn start_stop_timer(&self) {
        self.imp().timer.start_stop();
    }
}
