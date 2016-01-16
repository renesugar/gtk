// This file was generated by gir (463de47) from gir-files (11e0e6d)
// DO NOT EDIT

use Error;
use FileChooserAction;
use FileFilter;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct FileChooser(Object<ffi::GtkFileChooser>): Widget;

    match fn {
        get_type => || ffi::gtk_file_chooser_get_type(),
    }
}

pub trait FileChooserExt {
    fn add_filter(&self, filter: &FileFilter);

    fn add_shortcut_folder(&self, folder: &str) -> Result<(), Error>;

    fn add_shortcut_folder_uri(&self, uri: &str) -> Result<(), Error>;

    fn get_action(&self) -> FileChooserAction;

    fn get_create_folders(&self) -> bool;

    fn get_current_folder(&self) -> Option<String>;

    //fn get_current_folder_file(&self) -> /*Ignored*/Option<gio::File>;

    fn get_current_folder_uri(&self) -> Option<String>;

    #[cfg(gtk_3_10)]
    fn get_current_name(&self) -> Option<String>;

    fn get_do_overwrite_confirmation(&self) -> bool;

    fn get_extra_widget(&self) -> Option<Widget>;

    //fn get_file(&self) -> /*Ignored*/Option<gio::File>;

    fn get_filename(&self) -> Option<String>;

    fn get_filenames(&self) -> Vec<String>;

    //fn get_files(&self) -> Vec<gio::File>;

    fn get_filter(&self) -> Option<FileFilter>;

    fn get_local_only(&self) -> bool;

    //fn get_preview_file(&self) -> /*Ignored*/Option<gio::File>;

    fn get_preview_filename(&self) -> Option<String>;

    fn get_preview_uri(&self) -> Option<String>;

    fn get_preview_widget(&self) -> Option<Widget>;

    fn get_preview_widget_active(&self) -> bool;

    fn get_select_multiple(&self) -> bool;

    fn get_show_hidden(&self) -> bool;

    fn get_uri(&self) -> Option<String>;

    fn get_uris(&self) -> Vec<String>;

    fn get_use_preview_label(&self) -> bool;

    fn list_filters(&self) -> Vec<FileFilter>;

    fn list_shortcut_folder_uris(&self) -> Vec<String>;

    fn list_shortcut_folders(&self) -> Vec<String>;

    fn remove_filter(&self, filter: &FileFilter);

    fn remove_shortcut_folder(&self, folder: &str) -> Result<(), Error>;

    fn remove_shortcut_folder_uri(&self, uri: &str) -> Result<(), Error>;

    fn select_all(&self);

    //fn select_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T) -> Result<(), Error>;

    fn select_filename(&self, filename: &str) -> bool;

    fn select_uri(&self, uri: &str) -> bool;

    fn set_action(&self, action: FileChooserAction);

    fn set_create_folders(&self, create_folders: bool);

    fn set_current_folder(&self, filename: &str) -> bool;

    //fn set_current_folder_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T) -> Result<(), Error>;

    fn set_current_folder_uri(&self, uri: &str) -> bool;

    fn set_current_name(&self, name: &str);

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool);

    fn set_extra_widget<T: IsA<Widget>>(&self, extra_widget: &T);

    //fn set_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T) -> Result<(), Error>;

    fn set_filename(&self, filename: &str) -> bool;

    fn set_filter(&self, filter: &FileFilter);

    fn set_local_only(&self, local_only: bool);

    fn set_preview_widget<T: IsA<Widget>>(&self, preview_widget: &T);

    fn set_preview_widget_active(&self, active: bool);

    fn set_select_multiple(&self, select_multiple: bool);

    fn set_show_hidden(&self, show_hidden: bool);

    fn set_uri(&self, uri: &str) -> bool;

    fn set_use_preview_label(&self, use_label: bool);

    fn unselect_all(&self);

    //fn unselect_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T);

    fn unselect_filename(&self, filename: &str);

    fn unselect_uri(&self, uri: &str);
}

impl<O: IsA<FileChooser>> FileChooserExt for O {
    fn add_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_add_filter(self.to_glib_none().0, filter.to_glib_full());
        }
    }

    fn add_shortcut_folder(&self, folder: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_add_shortcut_folder(self.to_glib_none().0, folder.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_shortcut_folder_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_add_shortcut_folder_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_action(&self) -> FileChooserAction {
        unsafe {
            ffi::gtk_file_chooser_get_action(self.to_glib_none().0)
        }
    }

    fn get_create_folders(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_create_folders(self.to_glib_none().0))
        }
    }

    fn get_current_folder(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_folder(self.to_glib_none().0))
        }
    }

    //fn get_current_folder_file(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_file_chooser_get_current_folder_file() }
    //}

    fn get_current_folder_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_folder_uri(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    fn get_current_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_name(self.to_glib_none().0))
        }
    }

    fn get_do_overwrite_confirmation(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_do_overwrite_confirmation(self.to_glib_none().0))
        }
    }

    fn get_extra_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_extra_widget(self.to_glib_none().0))
        }
    }

    //fn get_file(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_file_chooser_get_file() }
    //}

    fn get_filename(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_filename(self.to_glib_none().0))
        }
    }

    fn get_filenames(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_get_filenames(self.to_glib_none().0))
        }
    }

    //fn get_files(&self) -> Vec<gio::File> {
    //    unsafe { TODO: call ffi::gtk_file_chooser_get_files() }
    //}

    fn get_filter(&self) -> Option<FileFilter> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_filter(self.to_glib_none().0))
        }
    }

    fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_local_only(self.to_glib_none().0))
        }
    }

    //fn get_preview_file(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_file_chooser_get_preview_file() }
    //}

    fn get_preview_filename(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_preview_filename(self.to_glib_none().0))
        }
    }

    fn get_preview_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_preview_uri(self.to_glib_none().0))
        }
    }

    fn get_preview_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_preview_widget(self.to_glib_none().0))
        }
    }

    fn get_preview_widget_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_preview_widget_active(self.to_glib_none().0))
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_select_multiple(self.to_glib_none().0))
        }
    }

    fn get_show_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_show_hidden(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_uri(self.to_glib_none().0))
        }
    }

    fn get_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_get_uris(self.to_glib_none().0))
        }
    }

    fn get_use_preview_label(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_use_preview_label(self.to_glib_none().0))
        }
    }

    fn list_filters(&self) -> Vec<FileFilter> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_file_chooser_list_filters(self.to_glib_none().0))
        }
    }

    fn list_shortcut_folder_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_list_shortcut_folder_uris(self.to_glib_none().0))
        }
    }

    fn list_shortcut_folders(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_list_shortcut_folders(self.to_glib_none().0))
        }
    }

    fn remove_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_remove_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn remove_shortcut_folder(&self, folder: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_remove_shortcut_folder(self.to_glib_none().0, folder.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_shortcut_folder_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_remove_shortcut_folder_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_file_chooser_select_all(self.to_glib_none().0);
        }
    }

    //fn select_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_file_chooser_select_file() }
    //}

    fn select_filename(&self, filename: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_select_filename(self.to_glib_none().0, filename.to_glib_none().0))
        }
    }

    fn select_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_select_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn set_action(&self, action: FileChooserAction) {
        unsafe {
            ffi::gtk_file_chooser_set_action(self.to_glib_none().0, action);
        }
    }

    fn set_create_folders(&self, create_folders: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_create_folders(self.to_glib_none().0, create_folders.to_glib());
        }
    }

    fn set_current_folder(&self, filename: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_set_current_folder(self.to_glib_none().0, filename.to_glib_none().0))
        }
    }

    //fn set_current_folder_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_file_chooser_set_current_folder_file() }
    //}

    fn set_current_folder_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_set_current_folder_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn set_current_name(&self, name: &str) {
        unsafe {
            ffi::gtk_file_chooser_set_current_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_do_overwrite_confirmation(self.to_glib_none().0, do_overwrite_confirmation.to_glib());
        }
    }

    fn set_extra_widget<T: IsA<Widget>>(&self, extra_widget: &T) {
        unsafe {
            ffi::gtk_file_chooser_set_extra_widget(self.to_glib_none().0, extra_widget.to_glib_none().0);
        }
    }

    //fn set_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_file_chooser_set_file() }
    //}

    fn set_filename(&self, filename: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_set_filename(self.to_glib_none().0, filename.to_glib_none().0))
        }
    }

    fn set_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_set_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    fn set_preview_widget<T: IsA<Widget>>(&self, preview_widget: &T) {
        unsafe {
            ffi::gtk_file_chooser_set_preview_widget(self.to_glib_none().0, preview_widget.to_glib_none().0);
        }
    }

    fn set_preview_widget_active(&self, active: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_preview_widget_active(self.to_glib_none().0, active.to_glib());
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_select_multiple(self.to_glib_none().0, select_multiple.to_glib());
        }
    }

    fn set_show_hidden(&self, show_hidden: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_show_hidden(self.to_glib_none().0, show_hidden.to_glib());
        }
    }

    fn set_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_set_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn set_use_preview_label(&self, use_label: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_use_preview_label(self.to_glib_none().0, use_label.to_glib());
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_file_chooser_unselect_all(self.to_glib_none().0);
        }
    }

    //fn unselect_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T) {
    //    unsafe { TODO: call ffi::gtk_file_chooser_unselect_file() }
    //}

    fn unselect_filename(&self, filename: &str) {
        unsafe {
            ffi::gtk_file_chooser_unselect_filename(self.to_glib_none().0, filename.to_glib_none().0);
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_file_chooser_unselect_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }
}
