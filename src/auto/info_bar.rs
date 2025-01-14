// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use BaselinePosition;
use Box;
use Buildable;
use Button;
use Container;
use MessageType;
use Orientable;
use Orientation;
use ResizeMode;
use ResponseType;
use Widget;

glib_wrapper! {
    pub struct InfoBar(Object<gtk_sys::GtkInfoBar, gtk_sys::GtkInfoBarClass, InfoBarClass>) @extends Box, Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_info_bar_get_type(),
    }
}

impl InfoBar {
    pub fn new() -> InfoBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_info_bar_new()).unsafe_cast() }
    }

    //pub fn new_with_buttons(first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> InfoBar {
    //    unsafe { TODO: call gtk_sys:gtk_info_bar_new_with_buttons() }
    //}
}

impl Default for InfoBar {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InfoBarBuilder {
    message_type: Option<MessageType>,
    #[cfg(any(feature = "v3_22_29", feature = "dox"))]
    revealed: Option<bool>,
    show_close_button: Option<bool>,
    baseline_position: Option<BaselinePosition>,
    homogeneous: Option<bool>,
    spacing: Option<i32>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
}

impl InfoBarBuilder {
    pub fn new() -> Self {
        Self {
            message_type: None,
            #[cfg(any(feature = "v3_22_29", feature = "dox"))]
            revealed: None,
            show_close_button: None,
            baseline_position: None,
            homogeneous: None,
            spacing: None,
            border_width: None,
            child: None,
            resize_mode: None,
            app_paintable: None,
            can_default: None,
            can_focus: None,
            events: None,
            expand: None,
            #[cfg(any(feature = "v3_20", feature = "dox"))]
            focus_on_click: None,
            halign: None,
            has_default: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            no_show_all: None,
            opacity: None,
            parent: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
            orientation: None,
        }
    }

    pub fn build(self) -> InfoBar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref message_type) = self.message_type {
            properties.push(("message-type", message_type));
        }
        #[cfg(any(feature = "v3_22_29", feature = "dox"))]
        {
            if let Some(ref revealed) = self.revealed {
                properties.push(("revealed", revealed));
            }
        }
        if let Some(ref show_close_button) = self.show_close_button {
            properties.push(("show-close-button", show_close_button));
        }
        if let Some(ref baseline_position) = self.baseline_position {
            properties.push(("baseline-position", baseline_position));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new(InfoBar::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = Some(message_type);
        self
    }

    #[cfg(any(feature = "v3_22_29", feature = "dox"))]
    pub fn revealed(mut self, revealed: bool) -> Self {
        self.revealed = Some(revealed);
        self
    }

    pub fn show_close_button(mut self, show_close_button: bool) -> Self {
        self.show_close_button = Some(show_close_button);
        self
    }

    pub fn baseline_position(mut self, baseline_position: BaselinePosition) -> Self {
        self.baseline_position = Some(baseline_position);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &Widget) -> Self {
        self.child = Some(child.clone());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &Container) -> Self {
        self.parent = Some(parent.clone());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_INFO_BAR: Option<&InfoBar> = None;

pub trait InfoBarExt: 'static {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: ResponseType);

    fn add_button(&self, button_text: &str, response_id: ResponseType) -> Option<Button>;

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_action_area(&self) -> Option<Widget>;

    fn get_content_area(&self) -> Option<Widget>;

    fn get_message_type(&self) -> MessageType;

    #[cfg(any(feature = "v3_22_29", feature = "dox"))]
    fn get_revealed(&self) -> bool;

    fn get_show_close_button(&self) -> bool;

    fn response(&self, response_id: ResponseType);

    fn set_default_response(&self, response_id: ResponseType);

    fn set_message_type(&self, message_type: MessageType);

    fn set_response_sensitive(&self, response_id: ResponseType, setting: bool);

    #[cfg(any(feature = "v3_22_29", feature = "dox"))]
    fn set_revealed(&self, revealed: bool);

    fn set_show_close_button(&self, setting: bool);

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_close(&self);

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_message_type_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[cfg(any(feature = "v3_22_29", feature = "dox"))]
    fn connect_property_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<InfoBar>> InfoBarExt for O {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: ResponseType) {
        unsafe {
            gtk_sys::gtk_info_bar_add_action_widget(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                response_id.to_glib(),
            );
        }
    }

    fn add_button(&self, button_text: &str, response_id: ResponseType) -> Option<Button> {
        unsafe {
            from_glib_none(gtk_sys::gtk_info_bar_add_button(
                self.as_ref().to_glib_none().0,
                button_text.to_glib_none().0,
                response_id.to_glib(),
            ))
        }
    }

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_info_bar_add_buttons() }
    //}

    fn get_action_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_info_bar_get_action_area(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_content_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_info_bar_get_content_area(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_message_type(&self) -> MessageType {
        unsafe {
            from_glib(gtk_sys::gtk_info_bar_get_message_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22_29", feature = "dox"))]
    fn get_revealed(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_info_bar_get_revealed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_info_bar_get_show_close_button(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn response(&self, response_id: ResponseType) {
        unsafe {
            gtk_sys::gtk_info_bar_response(self.as_ref().to_glib_none().0, response_id.to_glib());
        }
    }

    fn set_default_response(&self, response_id: ResponseType) {
        unsafe {
            gtk_sys::gtk_info_bar_set_default_response(
                self.as_ref().to_glib_none().0,
                response_id.to_glib(),
            );
        }
    }

    fn set_message_type(&self, message_type: MessageType) {
        unsafe {
            gtk_sys::gtk_info_bar_set_message_type(
                self.as_ref().to_glib_none().0,
                message_type.to_glib(),
            );
        }
    }

    fn set_response_sensitive(&self, response_id: ResponseType, setting: bool) {
        unsafe {
            gtk_sys::gtk_info_bar_set_response_sensitive(
                self.as_ref().to_glib_none().0,
                response_id.to_glib(),
                setting.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_22_29", feature = "dox"))]
    fn set_revealed(&self, revealed: bool) {
        unsafe {
            gtk_sys::gtk_info_bar_set_revealed(self.as_ref().to_glib_none().0, revealed.to_glib());
        }
    }

    fn set_show_close_button(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_info_bar_set_show_close_button(
                self.as_ref().to_glib_none().0,
                setting.to_glib(),
            );
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkInfoBar,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InfoBar>,
        {
            let f: &F = &*(f as *const F);
            f(&InfoBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute(close_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_close(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("close", &[])
                .unwrap()
        };
    }

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn response_trampoline<P, F: Fn(&P, ResponseType) + 'static>(
            this: *mut gtk_sys::GtkInfoBar,
            response_id: gtk_sys::GtkResponseType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InfoBar>,
        {
            let f: &F = &*(f as *const F);
            f(
                &InfoBar::from_glib_borrow(this).unsafe_cast(),
                from_glib(response_id),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"response\0".as_ptr() as *const _,
                Some(transmute(response_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_message_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_message_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkInfoBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InfoBar>,
        {
            let f: &F = &*(f as *const F);
            f(&InfoBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::message-type\0".as_ptr() as *const _,
                Some(transmute(
                    notify_message_type_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22_29", feature = "dox"))]
    fn connect_property_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_revealed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkInfoBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InfoBar>,
        {
            let f: &F = &*(f as *const F);
            f(&InfoBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::revealed\0".as_ptr() as *const _,
                Some(transmute(notify_revealed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_close_button_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkInfoBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InfoBar>,
        {
            let f: &F = &*(f as *const F);
            f(&InfoBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-close-button\0".as_ptr() as *const _,
                Some(transmute(
                    notify_show_close_button_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for InfoBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InfoBar")
    }
}
