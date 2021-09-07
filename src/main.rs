//use gdtk::add_corner;
use gtk::{prelude::*, HeaderBar, Stack, StackSwitcher};
use serde_json::Value;
use std::thread;
mod config;
mod page1;
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
    stack.add_titled(&page1, "lable", "商店");
    let lable2 = gtk::Box::new(gtk::Orientation::Vertical, 0);
    stack.add_titled(&lable2, "lable2", "下载信息");
    lable2.set_valign(gtk::Align::Start);
    GLOBAL_DOWNLOAD.with(move |global| {
        *global.borrow_mut() = Some(lable2);
    });

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
    search_entry.connect_event(|entry, event| {
        if let Some(key) = event.keycode() {
            if key == 36 {
                GLOBAL_OVERLAY.with(move |global| {
                    if let Some(ref overlay_box) = *global.borrow_mut() {
                        let children = overlay_box.children();
                        for child in children {
                            overlay_box.remove(&child);
                        }
                        let table = search_widget(entry.text().to_string());
                        overlay_box.pack_start(&table, true, true, 0);
                        overlay_box.show_all();
                        GLOBAL_TITLE.with(move |global| {
                            if let Some(ref title) = *global.borrow_mut() {
                                title.switch_title(&entry.text());
                            }
                        });
                    }
                });
            }
        };
        gtk::Inhibit(false)
    });
}
fn search_widget(tag_name: String) -> gtk::Widget {
    let urls: Vec<&str> = vec![
        "https://d.store.deepinos.org.cn//store/network/",
        "https://d.store.deepinos.org.cn//store/chat/",
        "https://d.store.deepinos.org.cn//store/music/",
        "https://d.store.deepinos.org.cn//store/video/",
        "https://d.store.deepinos.org.cn//store/image_graphics/",
        "https://d.store.deepinos.org.cn//store/games/",
        "https://d.store.deepinos.org.cn//store/office/",
        "https://d.store.deepinos.org.cn//store/reading/",
        "https://d.store.deepinos.org.cn//store/development/",
        "https://d.store.deepinos.org.cn//store/tools/",
        "https://d.store.deepinos.org.cn//store/others/",
    ];
    let flowbox = gtk::FlowBox::new();
    flowbox.set_valign(gtk::Align::Start);
    flowbox.set_max_children_per_line(30);
    flowbox.set_selection_mode(gtk::SelectionMode::None);
    let scrolled = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    let boxs = gtk::Box::new(gtk::Orientation::Vertical, 0);
    boxs.set_valign(gtk::Align::Start);
    boxs.pack_end(&flowbox, true, true, 0);
    scrolled.add(&boxs);
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    thread::spawn(move || {
        for url in urls.into_iter() {
            let input = fetch_message(url.to_string() + "applist.json");
            let source: Value = match serde_json::from_str(&input) {
                Err(_) => Value::Null,
                Ok(output) => output,
            };
            let mut index = 0;
            while source[index] != Value::Null {
                let name = remove_quotation(source[index]["Name"].to_string());
                if name.contains(&tag_name) {
                    let input = remove_quotation(source[index]["Pkgname"].to_string());
                    let input = url.to_string() + &input + "/icon.png";
                    let path = fetch_pic(&input);
                    let source0 = source[index].clone();
                    tx.send(Some((source0, path, url.to_string())))
                        .expect("error");
                }
                index += 1;
            }
        }
        tx.send(None).expect("error");
    });
    rx.attach(None, move |value| match value {
        Some(source) => {
            let (value, path, url2) = source;
            let pixbuf = {
                if value["icons"] != Value::Null {
                    let pixbuf = get_pixbuf(path);
                    pixbuf
                        .scale_simple(160, 160, gtk::gdk_pixbuf::InterpType::Hyper)
                        .unwrap()
                } else {
                    let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_resource("/ygo/akalin.png").unwrap();
                    pixbuf
                        .scale_simple(160, 160, gtk::gdk_pixbuf::InterpType::Hyper)
                        .unwrap()
                }
            };
            let image = gtk::Image::from_gicon(&pixbuf, gtk::IconSize::Button);
            let boxs = gtk::Box::new(gtk::Orientation::Vertical, 1);
            let button = gtk::Button::new();

            button.add(&image);
            let label = gtk::Label::new(Some(&remove_quotation(value["Name"].to_string())));
            label.set_max_width_chars(15);
            label.set_line_wrap(true);
            boxs.pack_start(&button, true, true, 0);
            boxs.pack_start(&label, true, true, 0);
            flowbox.add(&boxs);
            flowbox.show_all();

            button.connect_clicked(move |_| {
                let pixbuf2 = pixbuf.clone();
                let url2 = url2.clone();
                let value2 = value.clone();
                let the_title = remove_quotation(value["Name"].to_string());
                let more = remove_quotation(value["More"].to_string()).replace("\\n", "\n");
                let intorduction = gtk::Label::new(Some(&more));
                intorduction.set_line_wrap(true);
                let image = gtk::Image::from_gicon(&pixbuf, gtk::IconSize::Button);
                let overlay_inside_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                let overlay_left_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
                let download_button = gtk::Button::with_label("下载");

                overlay_left_box.pack_start(&image, true, false, 0);
                overlay_left_box.pack_start(&download_button, true, false, 0);

                overlay_inside_box.set_valign(gtk::Align::Start);
                overlay_inside_box.pack_start(&overlay_left_box, true, false, 0);
                overlay_inside_box.pack_start(&intorduction, true, false, 0);
                let scrolled = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
                scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
                scrolled.add(&overlay_inside_box);

                download_button.connect_clicked(move |_button| {
                    let pixbuf2 = pixbuf2.clone();
                    let url2 = url2.clone();
                    let value2 = value2.clone();
                    GLOBAL_OVERLAY.with(move |global| {
                        if let Some(ref overlay_box) = *global.borrow_mut() {
                            for child in overlay_box.children() {
                                overlay_box.remove(&child);
                            }
                            overlay_box.hide();
                        }
                    });
                    GLOBAL_TITLE.with(move |global| {
                        if let Some(ref title) = *global.borrow_mut() {
                            title.switch_stack();
                        }
                    });
                    GLOBAL_DOWNLOAD.with(move |global| {
                        let url2 = url2.clone();
                        if let Some(ref download) = *global.borrow_mut() {
                            let start = gtkdownloadbar::DownloadProgressBar::new(
                                format!(
                                    "{}{}/{}",
                                    url2,
                                    remove_quotation(value2["Pkgname"].to_string()),
                                    remove_quotation(value2["Filename"].to_string())
                                ),
                                Some(remove_quotation(value2["Name"].to_string())),
                                Some(pixbuf2),
                            )
                            .unwrap();

                            println!(
                                "{}",
                                format!(
                                    "{}{}/{}",
                                    url2,
                                    remove_quotation(value2["Pkgname"].to_string()),
                                    remove_quotation(value2["Filename"].to_string())
                                )
                            );
                            start.add_progress_bar_to(download);
                            download.show_all();
                        }
                    });
                });

                GLOBAL_OVERLAY.with(move |global| {
                    if let Some(ref overlay_box) = *global.borrow_mut() {
                        let children = overlay_box.children();
                        for child in children {
                            overlay_box.remove(&child);
                        }
                        overlay_box.pack_start(&scrolled, true, true, 0);
                        overlay_box.show_all();
                        GLOBAL_TITLE.with(move |global| {
                            if let Some(ref title) = *global.borrow_mut() {
                                title.switch_title(&the_title);
                            }
                        });
                    }
                });
            });

            glib::Continue(true)
        }
        None => {
            let underline_label = gtk::Label::new(Some("没有更多了"));
            boxs.pack_start(&underline_label, true, false, 0);
            boxs.show_all();
            glib::Continue(false)
        }
    });

    scrolled.upcast()
}
