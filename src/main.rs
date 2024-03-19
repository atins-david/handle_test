use gio::prelude::*;
use gtk::prelude::*;

use gdk::gdk_pixbuf::Pixbuf;
use std::path::Path;

use glib::ObjectType;
use std::ffi::c_void;

fn main() {
    let application = gtk::Application::new(Some("com.example.image_viewer"), Default::default());

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Image Viewer");
        window.set_default_size(1800, 1200);

        let drawing_area = gtk::DrawingArea::new();
        window.add(&drawing_area);

        drawing_area.connect_draw(|drawing_area, cr| {
            // any image file
            if let Some(pixbuf) = load_image("D:/b.png") {
                cr.set_source_pixbuf(&pixbuf, 0.0, 0.0);
                let _ = cr.paint();

                let handle = cast_native_window_handle(&drawing_area.window().unwrap());
                println!("handle?? {}", handle);

                if let Some(display) = gdk::Display::default() {
                    println!("display?? {:?}", display.name());                    

                    let display_reload = gdk::Display::open(&display.name()).unwrap();
                    let window_reload = cast_native_handle_window(display_reload, handle);
                    println!("wd?? {:?}", window_reload);

                    let pix = window_reload
                        .pixbuf(0, 0, window_reload.width(), window_reload.height())
                        .unwrap();

                    let file_path = "captured_image.png";
                    let format = "png";
                    let options = vec![];
                    pix.savev(file_path, format, &options)
                        .expect("Failed to save GdkPixbuf to file");
                }
            }
            Inhibit(false)
        });

        window.show_all();
    });

    application.run();
}

fn load_image(filename: &str) -> Option<Pixbuf> {
    if let Ok(pixbuf) = Pixbuf::from_file(Path::new(filename)) {
        Some(pixbuf)
    } else {
        println!("Failed to load image");
        None
    }
}

pub fn cast_native_window_handle(window: &gtk::gdk::Window) -> usize {
    #[cfg(target_os = "windows")]
    {
        extern "C" {
            pub fn gdk_win32_window_get_impl_hwnd(
                window: *mut glib::object::GObject,
            ) -> *mut c_void;
        }

        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            gdk_win32_window_get_impl_hwnd(window.as_ptr() as *mut _) as usize
        }
    }
}

pub fn cast_native_handle_window(display: gdk::Display, handle: usize) -> gtk::gdk::Window {
    #[cfg(target_os = "windows")]
    {
        extern "C" {
            pub fn gdk_win32_window_lookup_for_display(
                display: *mut glib::object::GObject,
                handle: usize,
            ) -> *mut c_void;
        }

        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let wd =
                gdk_win32_window_lookup_for_display(display.as_ptr() as *mut _, handle as usize)
                    as *mut gtk::gdk::Window;

            let wd = std::mem::transmute::<*mut gtk::gdk::Window, gtk::gdk::Window>(wd);

            wd
        }
    }
}
