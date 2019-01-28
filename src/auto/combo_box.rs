// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use CellArea;
use CellEditable;
use CellLayout;
use Container;
use ScrollType;
use SensitivityType;
use TreeIter;
use TreeModel;
use Widget;
use atk;
use ffi;
use gdk;
use glib;
use glib::GString;
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
    pub struct ComboBox(Object<ffi::GtkComboBox, ffi::GtkComboBoxClass, ComboBoxClass>) @extends Bin, Container, Widget, @implements Buildable, CellEditable, CellLayout;

    match fn {
        get_type => || ffi::gtk_combo_box_get_type(),
    }
}

impl ComboBox {
    pub fn new() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new()).unsafe_cast()
        }
    }

    pub fn new_with_area<P: IsA<CellArea>>(area: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_area(area.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_area_and_entry<P: IsA<CellArea>>(area: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_area_and_entry(area.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_entry() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_entry()).unsafe_cast()
        }
    }

    pub fn new_with_model<P: IsA<TreeModel>>(model: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model(model.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_model_and_entry<P: IsA<TreeModel>>(model: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model_and_entry(model.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for ComboBox {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_COMBO_BOX: Option<&ComboBox> = None;

pub trait ComboBoxExt: 'static {
    fn get_active_id(&self) -> Option<GString>;

    fn get_active_iter(&self) -> Option<TreeIter>;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_add_tearoffs(&self) -> bool;

    fn get_button_sensitivity(&self) -> SensitivityType;

    fn get_column_span_column(&self) -> i32;

    fn get_entry_text_column(&self) -> i32;

    #[cfg_attr(feature = "v3_20", deprecated)]
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool;

    fn get_has_entry(&self) -> bool;

    fn get_id_column(&self) -> i32;

    fn get_model(&self) -> Option<TreeModel>;

    fn get_popup_accessible(&self) -> Option<atk::Object>;

    fn get_popup_fixed_width(&self) -> bool;

    //fn get_row_separator_func(&self) -> Fn(&TreeModel, &TreeIter) -> bool + 'static;

    fn get_row_span_column(&self) -> i32;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_title(&self) -> Option<GString>;

    fn get_wrap_width(&self) -> i32;

    fn popdown(&self);

    fn popup(&self);

    fn popup_for_device(&self, device: &gdk::Device);

    fn set_active_id<'a, P: Into<Option<&'a str>>>(&self, active_id: P) -> bool;

    fn set_active_iter<'a, P: Into<Option<&'a TreeIter>>>(&self, iter: P);

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_add_tearoffs(&self, add_tearoffs: bool);

    fn set_button_sensitivity(&self, sensitivity: SensitivityType);

    fn set_column_span_column(&self, column_span: i32);

    fn set_entry_text_column(&self, text_column: i32);

    #[cfg_attr(feature = "v3_20", deprecated)]
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool);

    fn set_id_column(&self, id_column: i32);

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q);

    fn set_popup_fixed_width(&self, fixed: bool);

    fn set_row_separator_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P);

    fn set_row_span_column(&self, row_span: i32);

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_title(&self, title: &str);

    fn set_wrap_width(&self, width: i32);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn get_property_has_frame(&self) -> bool;

    fn set_property_has_frame(&self, has_frame: bool);

    fn get_property_popup_shown(&self) -> bool;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_property_tearoff_title(&self) -> Option<GString>;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_property_tearoff_title<'a, P: Into<Option<&'a str>>>(&self, tearoff_title: P);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_active(&self, scroll_type: ScrollType);

    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popdown(&self) -> bool;

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popup(&self);

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn connect_property_add_tearoffs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_button_sensitivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_column_span_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_entry_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_id_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_shown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_span_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn connect_property_tearoff_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ComboBox>> ComboBoxExt for O {
    fn get_active_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_active_id(self.as_ref().to_glib_none().0))
        }
    }

    fn get_active_iter(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_combo_box_get_active_iter(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_add_tearoffs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_add_tearoffs(self.as_ref().to_glib_none().0))
        }
    }

    fn get_button_sensitivity(&self) -> SensitivityType {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_button_sensitivity(self.as_ref().to_glib_none().0))
        }
    }

    fn get_column_span_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_column_span_column(self.as_ref().to_glib_none().0)
        }
    }

    fn get_entry_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_entry_text_column(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_focus_on_click(self.as_ref().to_glib_none().0))
        }
    }

    fn get_has_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_has_entry(self.as_ref().to_glib_none().0))
        }
    }

    fn get_id_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_id_column(self.as_ref().to_glib_none().0)
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_model(self.as_ref().to_glib_none().0))
        }
    }

    fn get_popup_accessible(&self) -> Option<atk::Object> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_popup_accessible(self.as_ref().to_glib_none().0))
        }
    }

    fn get_popup_fixed_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_popup_fixed_width(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_row_separator_func(&self) -> Fn(&TreeModel, &TreeIter) -> bool + 'static {
    //    unsafe { TODO: call ffi::gtk_combo_box_get_row_separator_func() }
    //}

    fn get_row_span_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_row_span_column(self.as_ref().to_glib_none().0)
        }
    }

    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_title(self.as_ref().to_glib_none().0))
        }
    }

    fn get_wrap_width(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_wrap_width(self.as_ref().to_glib_none().0)
        }
    }

    fn popdown(&self) {
        unsafe {
            ffi::gtk_combo_box_popdown(self.as_ref().to_glib_none().0);
        }
    }

    fn popup(&self) {
        unsafe {
            ffi::gtk_combo_box_popup(self.as_ref().to_glib_none().0);
        }
    }

    fn popup_for_device(&self, device: &gdk::Device) {
        unsafe {
            ffi::gtk_combo_box_popup_for_device(self.as_ref().to_glib_none().0, device.to_glib_none().0);
        }
    }

    fn set_active_id<'a, P: Into<Option<&'a str>>>(&self, active_id: P) -> bool {
        let active_id = active_id.into();
        unsafe {
            from_glib(ffi::gtk_combo_box_set_active_id(self.as_ref().to_glib_none().0, active_id.to_glib_none().0))
        }
    }

    fn set_active_iter<'a, P: Into<Option<&'a TreeIter>>>(&self, iter: P) {
        let iter = iter.into();
        unsafe {
            ffi::gtk_combo_box_set_active_iter(self.as_ref().to_glib_none().0, mut_override(iter.to_glib_none().0));
        }
    }

    fn set_add_tearoffs(&self, add_tearoffs: bool) {
        unsafe {
            ffi::gtk_combo_box_set_add_tearoffs(self.as_ref().to_glib_none().0, add_tearoffs.to_glib());
        }
    }

    fn set_button_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_combo_box_set_button_sensitivity(self.as_ref().to_glib_none().0, sensitivity.to_glib());
        }
    }

    fn set_column_span_column(&self, column_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_column_span_column(self.as_ref().to_glib_none().0, column_span);
        }
    }

    fn set_entry_text_column(&self, text_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_entry_text_column(self.as_ref().to_glib_none().0, text_column);
        }
    }

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_combo_box_set_focus_on_click(self.as_ref().to_glib_none().0, focus_on_click.to_glib());
        }
    }

    fn set_id_column(&self, id_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_id_column(self.as_ref().to_glib_none().0, id_column);
        }
    }

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q) {
        let model = model.into();
        unsafe {
            ffi::gtk_combo_box_set_model(self.as_ref().to_glib_none().0, model.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe {
            ffi::gtk_combo_box_set_popup_fixed_width(self.as_ref().to_glib_none().0, fixed.to_glib());
        }
    }

    fn set_row_separator_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box::new(func);
        unsafe extern "C" fn func_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(model: *mut ffi::GtkTreeModel, iter: *mut ffi::GtkTreeIter, data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let model = from_glib_borrow(model);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(data as *mut _);
            let res = (*callback)(&model, &iter);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(data: glib_ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_combo_box_set_row_separator_func(self.as_ref().to_glib_none().0, func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn set_row_span_column(&self, row_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_row_span_column(self.as_ref().to_glib_none().0, row_span);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_combo_box_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_wrap_width(&self, width: i32) {
        unsafe {
            ffi::gtk_combo_box_set_wrap_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = Value::from_type(<CellArea as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"cell-area\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_has_frame(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"has-frame\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_has_frame(&self, has_frame: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"has-frame\0".as_ptr() as *const _, Value::from(&has_frame).to_glib_none().0);
        }
    }

    fn get_property_popup_shown(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"popup-shown\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_tearoff_title(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"tearoff-title\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_tearoff_title<'a, P: Into<Option<&'a str>>>(&self, tearoff_title: P) {
        let tearoff_title = tearoff_title.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"tearoff-title\0".as_ptr() as *const _, Value::from(tearoff_title).to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) -> String + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"format-entry-text\0".as_ptr() as *const _,
                transmute(format_entry_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ScrollType) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"move-active\0".as_ptr() as *const _,
                transmute(move_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_active(&self, scroll_type: ScrollType) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("move-active", &[&scroll_type]).unwrap() };
    }

    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"popdown\0".as_ptr() as *const _,
                transmute(popdown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_popdown(&self) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("popdown", &[]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"popup\0".as_ptr() as *const _,
                transmute(popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_popup(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("popup", &[]).unwrap() };
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_active_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::active-id\0".as_ptr() as *const _,
                transmute(notify_active_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_add_tearoffs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::add-tearoffs\0".as_ptr() as *const _,
                transmute(notify_add_tearoffs_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_button_sensitivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::button-sensitivity\0".as_ptr() as *const _,
                transmute(notify_button_sensitivity_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_column_span_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::column-span-column\0".as_ptr() as *const _,
                transmute(notify_column_span_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_entry_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::entry-text-column\0".as_ptr() as *const _,
                transmute(notify_entry_text_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::has-frame\0".as_ptr() as *const _,
                transmute(notify_has_frame_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_id_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::id-column\0".as_ptr() as *const _,
                transmute(notify_id_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::model\0".as_ptr() as *const _,
                transmute(notify_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_popup_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::popup-fixed-width\0".as_ptr() as *const _,
                transmute(notify_popup_fixed_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_popup_shown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::popup-shown\0".as_ptr() as *const _,
                transmute(notify_popup_shown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_row_span_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::row-span-column\0".as_ptr() as *const _,
                transmute(notify_row_span_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tearoff_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::tearoff-title\0".as_ptr() as *const _,
                transmute(notify_tearoff_title_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wrap_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::wrap-width\0".as_ptr() as *const _,
                transmute(notify_wrap_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn format_entry_text_trampoline<P>(this: *mut ffi::GtkComboBox, path: *mut libc::c_char, f: glib_ffi::gpointer) -> *mut libc::c_char
where P: IsA<ComboBox> {
    let f: &&(Fn(&P, &str) -> String + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(path)).to_glib_full()
}

unsafe extern "C" fn move_active_trampoline<P>(this: *mut ffi::GtkComboBox, scroll_type: ffi::GtkScrollType, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P, ScrollType) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast(), from_glib(scroll_type))
}

unsafe extern "C" fn popdown_trampoline<P>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) -> bool + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast()).to_glib()
}

unsafe extern "C" fn popup_trampoline<P>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_active_id_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_add_tearoffs_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_button_sensitivity_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_column_span_column_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_entry_text_column_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_has_frame_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_id_column_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_model_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_popup_fixed_width_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_popup_shown_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_row_span_column_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_tearoff_title_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_wrap_width_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ComboBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComboBox")
    }
}
