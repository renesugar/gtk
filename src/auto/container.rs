// This file was generated by gir (463de47) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use ResizeMode;
use Widget;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Container(Object<ffi::GtkContainer>): Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_container_get_type(),
    }
}

pub trait ContainerExt {
    fn add<T: IsA<Widget>>(&self, widget: &T);

    //fn add_with_properties<T: IsA<Widget>>(&self, widget: &T, first_prop_name: &str, : /*Unknown conversion*/Fundamental: VarArgs);

    fn check_resize(&self);

    //fn child_get<T: IsA<Widget>>(&self, child: &T, first_prop_name: &str, : /*Unknown conversion*/Fundamental: VarArgs);

    //fn child_get_property<T: IsA<Widget>>(&self, child: &T, property_name: &str, value: /*Ignored*/&mut gobject::Value);

    //fn child_get_valist<T: IsA<Widget>>(&self, child: &T, first_property_name: &str, var_args: /*Unknown conversion*/Unsupported);

    fn child_notify<T: IsA<Widget>>(&self, child: &T, child_property: &str);

    //fn child_set<T: IsA<Widget>>(&self, child: &T, first_prop_name: &str, : /*Unknown conversion*/Fundamental: VarArgs);

    //fn child_set_property<T: IsA<Widget>>(&self, child: &T, property_name: &str, value: /*Ignored*/&gobject::Value);

    //fn child_set_valist<T: IsA<Widget>>(&self, child: &T, first_property_name: &str, var_args: /*Unknown conversion*/Unsupported);

    fn child_type(&self) -> glib::types::Type;

    //fn forall(&self, callback: /*Unknown conversion*/Unknown rust type: "Callback", callback_data: Fundamental: Pointer);

    //fn foreach(&self, callback: /*Unknown conversion*/Unknown rust type: "Callback", callback_data: Fundamental: Pointer);

    fn get_border_width(&self) -> u32;

    fn get_children(&self) -> Vec<Widget>;

    //fn get_focus_chain(&self, focusable_widgets: /*Unimplemented*/Vec<Widget>) -> bool;

    fn get_focus_child(&self) -> Option<Widget>;

    fn get_focus_hadjustment(&self) -> Option<Adjustment>;

    fn get_focus_vadjustment(&self) -> Option<Adjustment>;

    //fn get_path_for_child<T: IsA<Widget>>(&self, child: &T) -> /*Ignored*/WidgetPath;

    fn get_resize_mode(&self) -> ResizeMode;

    //fn propagate_draw<T: IsA<Widget>>(&self, child: &T, cr: /*Ignored*/&mut cairo::Context);

    fn remove<T: IsA<Widget>>(&self, widget: &T);

    fn resize_children(&self);

    fn set_border_width(&self, border_width: u32);

    fn set_focus_chain(&self, focusable_widgets: &[Widget]);

    fn set_focus_child<T: IsA<Widget>>(&self, child: Option<&T>);

    fn set_focus_hadjustment(&self, adjustment: &Adjustment);

    fn set_focus_vadjustment(&self, adjustment: &Adjustment);

    fn set_reallocate_redraws(&self, needs_redraws: bool);

    fn set_resize_mode(&self, resize_mode: ResizeMode);

    fn unset_focus_chain(&self);
}

impl<O: IsA<Container>> ContainerExt for O {
    fn add<T: IsA<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_container_add(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    //fn add_with_properties<T: IsA<Widget>>(&self, widget: &T, first_prop_name: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_container_add_with_properties() }
    //}

    fn check_resize(&self) {
        unsafe {
            ffi::gtk_container_check_resize(self.to_glib_none().0);
        }
    }

    //fn child_get<T: IsA<Widget>>(&self, child: &T, first_prop_name: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_container_child_get() }
    //}

    //fn child_get_property<T: IsA<Widget>>(&self, child: &T, property_name: &str, value: /*Ignored*/&mut gobject::Value) {
    //    unsafe { TODO: call ffi::gtk_container_child_get_property() }
    //}

    //fn child_get_valist<T: IsA<Widget>>(&self, child: &T, first_property_name: &str, var_args: /*Unknown conversion*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_container_child_get_valist() }
    //}

    fn child_notify<T: IsA<Widget>>(&self, child: &T, child_property: &str) {
        unsafe {
            ffi::gtk_container_child_notify(self.to_glib_none().0, child.to_glib_none().0, child_property.to_glib_none().0);
        }
    }

    //fn child_set<T: IsA<Widget>>(&self, child: &T, first_prop_name: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_container_child_set() }
    //}

    //fn child_set_property<T: IsA<Widget>>(&self, child: &T, property_name: &str, value: /*Ignored*/&gobject::Value) {
    //    unsafe { TODO: call ffi::gtk_container_child_set_property() }
    //}

    //fn child_set_valist<T: IsA<Widget>>(&self, child: &T, first_property_name: &str, var_args: /*Unknown conversion*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_container_child_set_valist() }
    //}

    fn child_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_container_child_type(self.to_glib_none().0))
        }
    }

    //fn forall(&self, callback: /*Unknown conversion*/Unknown rust type: "Callback", callback_data: Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_container_forall() }
    //}

    //fn foreach(&self, callback: /*Unknown conversion*/Unknown rust type: "Callback", callback_data: Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_container_foreach() }
    //}

    fn get_border_width(&self) -> u32 {
        unsafe {
            ffi::gtk_container_get_border_width(self.to_glib_none().0)
        }
    }

    fn get_children(&self) -> Vec<Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_container_get_children(self.to_glib_none().0))
        }
    }

    //fn get_focus_chain(&self, focusable_widgets: /*Unimplemented*/Vec<Widget>) -> bool {
    //    unsafe { TODO: call ffi::gtk_container_get_focus_chain() }
    //}

    fn get_focus_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_container_get_focus_child(self.to_glib_none().0))
        }
    }

    fn get_focus_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_container_get_focus_hadjustment(self.to_glib_none().0))
        }
    }

    fn get_focus_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_container_get_focus_vadjustment(self.to_glib_none().0))
        }
    }

    //fn get_path_for_child<T: IsA<Widget>>(&self, child: &T) -> /*Ignored*/WidgetPath {
    //    unsafe { TODO: call ffi::gtk_container_get_path_for_child() }
    //}

    fn get_resize_mode(&self) -> ResizeMode {
        unsafe {
            ffi::gtk_container_get_resize_mode(self.to_glib_none().0)
        }
    }

    //fn propagate_draw<T: IsA<Widget>>(&self, child: &T, cr: /*Ignored*/&mut cairo::Context) {
    //    unsafe { TODO: call ffi::gtk_container_propagate_draw() }
    //}

    fn remove<T: IsA<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_container_remove(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    fn resize_children(&self) {
        unsafe {
            ffi::gtk_container_resize_children(self.to_glib_none().0);
        }
    }

    fn set_border_width(&self, border_width: u32) {
        unsafe {
            ffi::gtk_container_set_border_width(self.to_glib_none().0, border_width);
        }
    }

    fn set_focus_chain(&self, focusable_widgets: &[Widget]) {
        unsafe {
            ffi::gtk_container_set_focus_chain(self.to_glib_none().0, focusable_widgets.to_glib_none().0);
        }
    }

    fn set_focus_child<T: IsA<Widget>>(&self, child: Option<&T>) {
        unsafe {
            ffi::gtk_container_set_focus_child(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn set_focus_hadjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_container_set_focus_hadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    fn set_focus_vadjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_container_set_focus_vadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    fn set_reallocate_redraws(&self, needs_redraws: bool) {
        unsafe {
            ffi::gtk_container_set_reallocate_redraws(self.to_glib_none().0, needs_redraws.to_glib());
        }
    }

    fn set_resize_mode(&self, resize_mode: ResizeMode) {
        unsafe {
            ffi::gtk_container_set_resize_mode(self.to_glib_none().0, resize_mode);
        }
    }

    fn unset_focus_chain(&self) {
        unsafe {
            ffi::gtk_container_unset_focus_chain(self.to_glib_none().0);
        }
    }
}
