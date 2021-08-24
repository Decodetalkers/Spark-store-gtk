//use gdtk::add_corner;
use gtk::{prelude::*, HeaderBar, Stack, StackSwitcher};
mod page1;
mod config;
use config::*;
fn main() {
    gio::resources_register_include!("compiled.gresource").unwrap();
    let application = gtk::Application::new(Some("come.test.add"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &gtk::Application) {
    let provider = gtk::CssProvider::new();
    // Load the CSS file
    let style = include_bytes!("gtk.css");
    provider.load_from_data(style).expect("Failed to load CSS");
    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let window = gtk::ApplicationWindow::new(application);
    if let Ok(icon) = &gtk::gdk_pixbuf::Pixbuf::from_resource("/ygo/youxie.jpeg") {
        window.set_icon(Some(icon));
    }
    // 绘制圆角
    //add_corner(&window, 0.02);
    //window.set_title("test");
    window.set_default_size(650, 200);
    //let vbox = gtk::Box::new(gtk::Orientation::Vertical,10);
    let title = HeaderBar::new();
    title.set_show_close_button(true);
    window.set_titlebar(Some(&title));
    //vbox.pack_start(&title, false, false, 0);

    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(1000);
    let page1 = page1::mainpage();
    stack.add_titled(&page1, "lable", "label");
    let lable2 = gtk::Label::new(Some("BB"));
    stack.add_titled(&lable2, "lable2", "label2");

    let stackswitcher = StackSwitcher::new();
    stackswitcher.set_stack(Some(&stack));
    let title_label = gtk::Label::new(Some("Shop"));
    let titel_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    titel_box.pack_start(&stackswitcher, true, true, 0);
    titel_box.pack_start(&title_label, true, true, 0);
    let search_image =
        gtk::Image::from_icon_name(Some("edit-find-symbolic"), gtk::IconSize::Button);
    let button_search = gtk::ToggleButton::new();
    button_search.add(&search_image);
    let search_bar = gtk::SearchBar::new();
    let search_entry = gtk::SearchEntry::new();
    //这样就让搜索出现了
    //search_bar.set_search_mode(true);
    search_bar.set_child(Some(&search_entry));
    button_search.connect_toggled(glib::clone!(@weak search_bar => move |button|{
        if button.is_active() {
            search_bar.set_search_mode(true);
        }else {
            search_bar.set_search_mode(false);
        }
    }));
    let boxs = gtk::Box::new(gtk::Orientation::Vertical, 0);
    boxs.pack_start(&search_bar, false, true, 0);

    let overlay = gtk::Overlay::new();
    overlay.add(&stack);
    boxs.pack_start(&overlay, true, true, 0);
    //button_search.conn
    title.pack_end(&button_search);
    //title.add(&stackswitcher);
    //title.set_child(Some(&stackswitcher));
    // pack_start是左边
    // set_custom_title 设置标题控件
    // pack_end 设置右起
    let back_image =
        gtk::Image::from_icon_name(Some("go-previous-symbolic"), gtk::IconSize::Button);
    let button_back = gtk::Button::new();
    button_back.add(&back_image);

    title.pack_start(&button_back);

    title.set_custom_title(Some(&titel_box));

    //vbox.pack_start(&lable, true, true, 0);
    //布局完成
    window.add(&boxs);
    window.show_all();
    title_label.hide();
    button_back.hide();

    let overlay_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    overlay_box.set_widget_name("overlaybox");
    overlay_box.hide();
    overlay.add_overlay(&overlay_box);

    GLOBAL_OVERLAY.with(move |global| {
        *global.borrow_mut() = Some(overlay_box);
    });
    button_back.connect_clicked(|button| {
        GLOBAL_OVERLAY.with(move |global| {
            if let Some(ref overlay_box) = *global.borrow_mut() {
                let children = overlay_box.children();
                for child in children {
                    overlay_box.remove(&child);
                }
                overlay_box.hide();
                GLOBAL_TITLE.with(move |global| {
                    if let Some(ref title) = *global.borrow_mut() {
                        title.switch_stack();
                    }
                });
            }
        });
        button.hide();
    });
    GLOBAL_TITLE.with(move |global| {
        *global.borrow_mut() = Some(Title {
            title_label,
            stackswitcher,
            button_back,
        })
    });
    search_entry.connect_event(|entry,event|{
        if let Some(key) = event.keycode() {
            if key == 36{
                GLOBAL_OVERLAY.with(move |global|{
                    if let Some(ref overlay_box) = *global.borrow_mut(){
                        if overlay_box.children().is_empty(){
                            let table = gtk::Label::new(Some("MM"));
                            overlay_box.pack_start(&table,true,true,0);
                            overlay_box.show_all();
                            GLOBAL_TITLE.with(move |global|{
                                if let Some(ref title) = *global.borrow_mut(){
                                    title.switch_title(&entry.text());
                                }
                            });
                        }

                    }
                });
            }
        };
        gtk::Inhibit(false)
    });
}
