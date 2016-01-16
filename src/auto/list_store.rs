// This file was generated by gir (463de47) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use TreeIter;
use TreeModel;
use TreeSortable;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct ListStore(Object<ffi::GtkListStore>): Buildable, TreeModel, TreeSortable;

    match fn {
        get_type => || ffi::gtk_list_store_get_type(),
    }
}

impl ListStore {
    //pub fn new(n_columns: i32, : /*Unknown conversion*/Fundamental: VarArgs) -> ListStore {
    //    unsafe { TODO: call ffi::gtk_list_store_new() }
    //}

    //pub fn newv(n_columns: i32, types: &Unknown rust type: "CArray TypeId { ns_id: 0, id: 30 }") -> ListStore {
    //    unsafe { TODO: call ffi::gtk_list_store_newv() }
    //}

    pub fn append(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_append(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gtk_list_store_clear(self.to_glib_none().0);
        }
    }

    pub fn insert(&self, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert(self.to_glib_none().0, iter.to_glib_none_mut().0, position);
            iter
        }
    }

    pub fn insert_after(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_after(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    pub fn insert_before(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_before(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    //pub fn insert_with_values(&self, position: i32, : /*Unknown conversion*/Fundamental: VarArgs) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_list_store_insert_with_values() }
    //}

    //pub fn insert_with_valuesv(&self, position: i32, columns: &Unknown rust type: "CArray TypeId { ns_id: 0, id: 14 }", values: &[&gobject::Value], n_values: i32) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_list_store_insert_with_valuesv() }
    //}

    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_iter_is_valid(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    pub fn move_after(&self, iter: &mut TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_after(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(position.to_glib_none().0));
        }
    }

    pub fn move_before(&self, iter: &mut TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_before(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(position.to_glib_none().0));
        }
    }

    pub fn prepend(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_prepend(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn remove(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_remove(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    //pub fn reorder(&self, new_order: &Unknown rust type: "CArray TypeId { ns_id: 0, id: 14 }") {
    //    unsafe { TODO: call ffi::gtk_list_store_reorder() }
    //}

    //pub fn set(&self, iter: &mut TreeIter, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_list_store_set() }
    //}

    //pub fn set_column_types(&self, n_columns: i32, types: &Unknown rust type: "CArray TypeId { ns_id: 0, id: 30 }") {
    //    unsafe { TODO: call ffi::gtk_list_store_set_column_types() }
    //}

    //pub fn set_valist(&self, iter: &mut TreeIter, var_args: /*Unknown conversion*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_list_store_set_valist() }
    //}

    //pub fn set_value(&self, iter: &mut TreeIter, column: i32, value: /*Ignored*/&mut gobject::Value) {
    //    unsafe { TODO: call ffi::gtk_list_store_set_value() }
    //}

    //pub fn set_valuesv(&self, iter: &mut TreeIter, columns: &Unknown rust type: "CArray TypeId { ns_id: 0, id: 14 }", values: &[&gobject::Value], n_values: i32) {
    //    unsafe { TODO: call ffi::gtk_list_store_set_valuesv() }
    //}

    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_list_store_swap(self.to_glib_none().0, mut_override(a.to_glib_none().0), mut_override(b.to_glib_none().0));
        }
    }
}
