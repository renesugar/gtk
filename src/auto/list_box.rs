// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_10", feature = "dox"))]
use Adjustment;
use Buildable;
use Container;
use ListBoxRow;
use MovementStep;
use SelectionMode;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ListBox(Object<ffi::GtkListBox, ffi::GtkListBoxClass, ListBoxClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_list_box_get_type(),
    }
}

impl ListBox {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new() -> ListBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_new()).unsafe_cast()
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
impl Default for ListBox {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_LIST_BOX: Option<&ListBox> = None;

pub trait ListBoxExt: 'static {
    //#[cfg(any(feature = "v3_16", feature = "dox"))]
    //fn bind_model<P: Fn(&glib::Object) -> Widget + 'static, Q: Into<Option<P>>>(&self, model: /*Ignored*/Option<&gio::ListModel>, create_widget_func: Q);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn drag_highlight_row<P: IsA<ListBoxRow>>(&self, row: &P);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn drag_unhighlight_row(&self);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_activate_on_single_click(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_adjustment(&self) -> Option<Adjustment>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_selected_row(&self) -> Option<ListBoxRow>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_selected_rows(&self) -> Vec<ListBoxRow>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_selection_mode(&self) -> SelectionMode;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn invalidate_filter(&self);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn invalidate_headers(&self);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn invalidate_sort(&self);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn prepend<P: IsA<Widget>>(&self, child: &P);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn select_all(&self);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn select_row<'a, P: IsA<ListBoxRow> + 'a, Q: Into<Option<&'a P>>>(&self, row: Q);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn selected_foreach<P: FnMut(&ListBox, &ListBoxRow)>(&self, func: P);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_activate_on_single_click(&self, single: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_adjustment<'a, P: IsA<Adjustment> + 'a, Q: Into<Option<&'a P>>>(&self, adjustment: Q);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_filter_func<P: Fn(&ListBoxRow) -> bool + 'static, Q: Into<Option<P>>>(&self, filter_func: Q);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_header_func<P: Fn(&ListBoxRow, &ListBoxRow) + 'static, Q: Into<Option<P>>>(&self, update_header: Q);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_placeholder<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, placeholder: Q);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_selection_mode(&self, mode: SelectionMode);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_sort_func<P: Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static, Q: Into<Option<P>>>(&self, sort_func: Q);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn unselect_all(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn unselect_row<P: IsA<ListBoxRow>>(&self, row: &P);

    fn get_property_activate_on_single_click(&self) -> bool;

    fn set_property_activate_on_single_click(&self, activate_on_single_click: bool);

    fn get_property_selection_mode(&self) -> SelectionMode;

    fn set_property_selection_mode(&self, selection_mode: SelectionMode);

    fn connect_activate_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_cursor_row(&self);

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_cursor(&self, object: MovementStep, p0: i32);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_row_activated<F: Fn(&Self, &ListBoxRow) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_row_selected<F: Fn(&Self, &Option<ListBoxRow>) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn emit_select_all(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_selected_rows_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_toggle_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_cursor_row(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn emit_unselect_all(&self);

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ListBox>> ListBoxExt for O {
    //#[cfg(any(feature = "v3_16", feature = "dox"))]
    //fn bind_model<P: Fn(&glib::Object) -> Widget + 'static, Q: Into<Option<P>>>(&self, model: /*Ignored*/Option<&gio::ListModel>, create_widget_func: Q) {
    //    unsafe { TODO: call ffi::gtk_list_box_bind_model() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn drag_highlight_row<P: IsA<ListBoxRow>>(&self, row: &P) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(self.as_ref().to_glib_none().0, row.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn drag_unhighlight_row(&self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_get_activate_on_single_click(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_adjustment(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_index(self.as_ref().to_glib_none().0, index_))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_y(self.as_ref().to_glib_none().0, y))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_selected_row(&self) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_selected_row(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_selected_rows(&self) -> Vec<ListBoxRow> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_list_box_get_selected_rows(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_list_box_get_selection_mode(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_filter(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn invalidate_headers(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_sort(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn prepend<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_list_box_prepend(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn select_all(&self) {
        unsafe {
            ffi::gtk_list_box_select_all(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn select_row<'a, P: IsA<ListBoxRow> + 'a, Q: Into<Option<&'a P>>>(&self, row: Q) {
        let row = row.into();
        unsafe {
            ffi::gtk_list_box_select_row(self.as_ref().to_glib_none().0, row.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn selected_foreach<P: FnMut(&ListBox, &ListBoxRow)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&ListBox, &ListBoxRow)>(box_: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, user_data: glib_ffi::gpointer) {
            let box_ = from_glib_borrow(box_);
            let row = from_glib_borrow(row);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&box_, &row);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_list_box_selected_foreach(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(self.as_ref().to_glib_none().0, single.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_adjustment<'a, P: IsA<Adjustment> + 'a, Q: Into<Option<&'a P>>>(&self, adjustment: Q) {
        let adjustment = adjustment.into();
        unsafe {
            ffi::gtk_list_box_set_adjustment(self.as_ref().to_glib_none().0, adjustment.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_filter_func<P: Fn(&ListBoxRow) -> bool + 'static, Q: Into<Option<P>>>(&self, filter_func: Q) {
        let filter_func = filter_func.into();
        let filter_func_data: Box_<Option<P>> = Box::new(filter_func.into());
        unsafe extern "C" fn filter_func_func<P: Fn(&ListBoxRow) -> bool + 'static>(row: *mut ffi::GtkListBoxRow, user_data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let row = from_glib_borrow(row);
            let callback: &Option<P> = &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&row)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let filter_func = if filter_func_data.is_some() { Some(filter_func_func::<P> as _) } else { None };
        unsafe extern "C" fn destroy_func<P: Fn(&ListBoxRow) -> bool + 'static>(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<P>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<Option<P>> = filter_func_data;
        unsafe {
            ffi::gtk_list_box_set_filter_func(self.as_ref().to_glib_none().0, filter_func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_header_func<P: Fn(&ListBoxRow, &ListBoxRow) + 'static, Q: Into<Option<P>>>(&self, update_header: Q) {
        let update_header = update_header.into();
        let update_header_data: Box_<Option<P>> = Box::new(update_header.into());
        unsafe extern "C" fn update_header_func<P: Fn(&ListBoxRow, &ListBoxRow) + 'static>(row: *mut ffi::GtkListBoxRow, before: *mut ffi::GtkListBoxRow, user_data: glib_ffi::gpointer) {
            let row = from_glib_borrow(row);
            let before = from_glib_borrow(before);
            let callback: &Option<P> = &*(user_data as *mut _);
            if let Some(ref callback) = *callback {
                callback(&row, &before)
            } else {
                panic!("cannot get closure...")
            };
        }
        let update_header = if update_header_data.is_some() { Some(update_header_func::<P> as _) } else { None };
        unsafe extern "C" fn destroy_func<P: Fn(&ListBoxRow, &ListBoxRow) + 'static>(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<P>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<Option<P>> = update_header_data;
        unsafe {
            ffi::gtk_list_box_set_header_func(self.as_ref().to_glib_none().0, update_header, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_placeholder<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, placeholder: Q) {
        let placeholder = placeholder.into();
        unsafe {
            ffi::gtk_list_box_set_placeholder(self.as_ref().to_glib_none().0, placeholder.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_sort_func<P: Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static, Q: Into<Option<P>>>(&self, sort_func: Q) {
        let sort_func = sort_func.into();
        let sort_func_data: Box_<Option<P>> = Box::new(sort_func.into());
        unsafe extern "C" fn sort_func_func<P: Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>(row1: *mut ffi::GtkListBoxRow, row2: *mut ffi::GtkListBoxRow, user_data: glib_ffi::gpointer) -> libc::c_int {
            let row1 = from_glib_borrow(row1);
            let row2 = from_glib_borrow(row2);
            let callback: &Option<P> = &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&row1, &row2)
            } else {
                panic!("cannot get closure...")
            };
            res
        }
        let sort_func = if sort_func_data.is_some() { Some(sort_func_func::<P> as _) } else { None };
        unsafe extern "C" fn destroy_func<P: Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<P>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<Option<P>> = sort_func_data;
        unsafe {
            ffi::gtk_list_box_set_sort_func(self.as_ref().to_glib_none().0, sort_func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_list_box_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn unselect_row<P: IsA<ListBoxRow>>(&self, row: &P) {
        unsafe {
            ffi::gtk_list_box_unselect_row(self.as_ref().to_glib_none().0, row.as_ref().to_glib_none().0);
        }
    }

    fn get_property_activate_on_single_click(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"activate-on-single-click\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_activate_on_single_click(&self, activate_on_single_click: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"activate-on-single-click\0".as_ptr() as *const _, Value::from(&activate_on_single_click).to_glib_none().0);
        }
    }

    fn get_property_selection_mode(&self) -> SelectionMode {
        unsafe {
            let mut value = Value::from_type(<SelectionMode as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"selection-mode\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_selection_mode(&self, selection_mode: SelectionMode) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"selection-mode\0".as_ptr() as *const _, Value::from(&selection_mode).to_glib_none().0);
        }
    }

    fn connect_activate_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"activate-cursor-row\0".as_ptr() as *const _,
                transmute(activate_cursor_row_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_activate_cursor_row(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("activate-cursor-row", &[]).unwrap() };
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, MovementStep, i32) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"move-cursor\0".as_ptr() as *const _,
                transmute(move_cursor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_cursor(&self, object: MovementStep, p0: i32) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("move-cursor", &[&object, &p0]).unwrap() };
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_row_activated<F: Fn(&Self, &ListBoxRow) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &ListBoxRow) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"row-activated\0".as_ptr() as *const _,
                transmute(row_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_row_selected<F: Fn(&Self, &Option<ListBoxRow>) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Option<ListBoxRow>) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"row-selected\0".as_ptr() as *const _,
                transmute(row_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"select-all\0".as_ptr() as *const _,
                transmute(select_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn emit_select_all(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("select-all", &[]).unwrap() };
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_selected_rows_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"selected-rows-changed\0".as_ptr() as *const _,
                transmute(selected_rows_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_toggle_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"toggle-cursor-row\0".as_ptr() as *const _,
                transmute(toggle_cursor_row_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_toggle_cursor_row(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("toggle-cursor-row", &[]).unwrap() };
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"unselect-all\0".as_ptr() as *const _,
                transmute(unselect_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn emit_unselect_all(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("unselect-all", &[]).unwrap() };
    }

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::activate-on-single-click\0".as_ptr() as *const _,
                transmute(notify_activate_on_single_click_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::selection-mode\0".as_ptr() as *const _,
                transmute(notify_selection_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_cursor_row_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn move_cursor_trampoline<P>(this: *mut ffi::GtkListBox, object: ffi::GtkMovementStep, p0: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P, MovementStep, i32) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast(), from_glib(object), p0)
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn row_activated_trampoline<P>(this: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P, &ListBoxRow) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(row))
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn row_selected_trampoline<P>(this: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P, &Option<ListBoxRow>) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(row))
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn select_all_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn selected_rows_changed_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn toggle_cursor_row_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn unselect_all_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_activate_on_single_click_trampoline<P>(this: *mut ffi::GtkListBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selection_mode_trampoline<P>(this: *mut ffi::GtkListBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ListBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListBox")
    }
}
