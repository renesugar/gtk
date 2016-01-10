// This file was generated by gir (15fe1aa) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use PositionType;
use ReliefStyle;
use Widget;
use ffi;
use gdk;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Button(Object<ffi::GtkButton>): Widget, Container, Bin, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_button_get_type(),
    }
}

impl Button {
    pub fn new() -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_icon_name(icon_name: &str, size: i32) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_from_icon_name(icon_name.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }

}

pub trait ButtonExt {
    fn clicked(&self);
    fn enter(&self);
    fn get_alignment(&self) -> (f32, f32);
    #[cfg(gtk_3_6)]
    fn get_always_show_image(&self) -> bool;
    fn get_event_window(&self) -> Option<gdk::Window>;
    fn get_focus_on_click(&self) -> bool;
    fn get_image(&self) -> Option<Widget>;
    fn get_image_position(&self) -> PositionType;
    fn get_label(&self) -> Option<String>;
    fn get_relief(&self) -> ReliefStyle;
    fn get_use_stock(&self) -> bool;
    fn get_use_underline(&self) -> bool;
    fn leave(&self);
    fn pressed(&self);
    fn released(&self);
    fn set_alignment(&self, xalign: f32, yalign: f32);
    #[cfg(gtk_3_6)]
    fn set_always_show_image(&self, always_show: bool);
    fn set_focus_on_click(&self, focus_on_click: bool);
    fn set_image<T: Upcast<Widget>>(&self, image: &T);
    fn set_image_position(&self, position: PositionType);
    fn set_label(&self, label: &str);
    fn set_relief(&self, relief: ReliefStyle);
    fn set_use_stock(&self, use_stock: bool);
    fn set_use_underline(&self, use_underline: bool);
}

impl<O: Upcast<Button>> ButtonExt for O {
    fn clicked(&self) {
        unsafe {
            ffi::gtk_button_clicked(self.to_glib_none().0);
        }
    }

    fn enter(&self) {
        unsafe {
            ffi::gtk_button_enter(self.to_glib_none().0);
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_button_get_alignment(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    #[cfg(gtk_3_6)]
    fn get_always_show_image(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_always_show_image(self.to_glib_none().0))
        }
    }

    fn get_event_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_event_window(self.to_glib_none().0))
        }
    }

    fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_focus_on_click(self.to_glib_none().0))
        }
    }

    fn get_image(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_image(self.to_glib_none().0))
        }
    }

    fn get_image_position(&self) -> PositionType {
        unsafe {
            ffi::gtk_button_get_image_position(self.to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_label(self.to_glib_none().0))
        }
    }

    fn get_relief(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_button_get_relief(self.to_glib_none().0)
        }
    }

    fn get_use_stock(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_use_stock(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_use_underline(self.to_glib_none().0))
        }
    }

    fn leave(&self) {
        unsafe {
            ffi::gtk_button_leave(self.to_glib_none().0);
        }
    }

    fn pressed(&self) {
        unsafe {
            ffi::gtk_button_pressed(self.to_glib_none().0);
        }
    }

    fn released(&self) {
        unsafe {
            ffi::gtk_button_released(self.to_glib_none().0);
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_button_set_alignment(self.to_glib_none().0, xalign, yalign);
        }
    }

    #[cfg(gtk_3_6)]
    fn set_always_show_image(&self, always_show: bool) {
        unsafe {
            ffi::gtk_button_set_always_show_image(self.to_glib_none().0, always_show.to_glib());
        }
    }

    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_button_set_focus_on_click(self.to_glib_none().0, focus_on_click.to_glib());
        }
    }

    fn set_image<T: Upcast<Widget>>(&self, image: &T) {
        unsafe {
            ffi::gtk_button_set_image(self.to_glib_none().0, image.to_glib_none().0);
        }
    }

    fn set_image_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_button_set_image_position(self.to_glib_none().0, position);
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_button_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_relief(&self, relief: ReliefStyle) {
        unsafe {
            ffi::gtk_button_set_relief(self.to_glib_none().0, relief);
        }
    }

    fn set_use_stock(&self, use_stock: bool) {
        unsafe {
            ffi::gtk_button_set_use_stock(self.to_glib_none().0, use_stock.to_glib());
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_button_set_use_underline(self.to_glib_none().0, use_underline.to_glib());
        }
    }

}
