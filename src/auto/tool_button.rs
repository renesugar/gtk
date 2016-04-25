// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Container;
use Object;
use ToolItem;
use Widget;
use ffi;
use ffi::GtkToolButton;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi::gpointer;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ToolButton(Object<ffi::GtkToolButton>): ToolItem, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_tool_button_get_type(),
    }
}

impl ToolButton {
    pub fn new<T: IsA<Widget>>(icon_widget: Option<&T>, label: Option<&str>) -> ToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new(icon_widget.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> ToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ToolButtonExt {
    fn get_icon_name(&self) -> Option<String>;

    fn get_icon_widget(&self) -> Option<Widget>;

    fn get_label(&self) -> Option<String>;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_stock_id(&self) -> Option<String>;

    fn get_use_underline(&self) -> bool;

    fn set_icon_name(&self, icon_name: Option<&str>);

    fn set_icon_widget<T: IsA<Widget>>(&self, icon_widget: Option<&T>);

    fn set_label(&self, label: Option<&str>);

    fn set_label_widget<T: IsA<Widget>>(&self, label_widget: Option<&T>);

    fn set_stock_id(&self, stock_id: Option<&str>);

    fn set_use_underline(&self, use_underline: bool);

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ToolButton> + IsA<Object>> ToolButtonExt for O {
    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_icon_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_widget(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label(self.to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label_widget(self.to_glib_none().0))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_stock_id(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_button_get_use_underline(self.to_glib_none().0))
        }
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_tool_button_set_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn set_icon_widget<T: IsA<Widget>>(&self, icon_widget: Option<&T>) {
        unsafe {
            ffi::gtk_tool_button_set_icon_widget(self.to_glib_none().0, icon_widget.to_glib_none().0);
        }
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::gtk_tool_button_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_widget<T: IsA<Widget>>(&self, label_widget: Option<&T>) {
        unsafe {
            ffi::gtk_tool_button_set_label_widget(self.to_glib_none().0, label_widget.to_glib_none().0);
        }
    }

    fn set_stock_id(&self, stock_id: Option<&str>) {
        unsafe {
            ffi::gtk_tool_button_set_stock_id(self.to_glib_none().0, stock_id.to_glib_none().0);
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_tool_button_set_use_underline(self.to_glib_none().0, use_underline.to_glib());
        }
    }

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "clicked",
                transmute(clicked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn clicked_trampoline<T>(this: *mut GtkToolButton, f: gpointer)
where T: IsA<ToolButton> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&ToolButton::from_glib_none(this).downcast_unchecked())
}
