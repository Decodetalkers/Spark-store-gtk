use futures::executor::block_on;
use gtk::{prelude::*, Label, Notebook};
//use gtk::Image;
use serde_json::Value;
use std::thread;
use crate::config::*;
pub fn mainpage() -> Notebook {
    let notebook = Notebook::new();
    notebook.set_tab_pos(gtk::PositionType::Left);
    notebook.set_scrollable(true);
    let names = vec![
        "网络应用",
        "社交沟通",
        "音乐欣赏",
        "视频播放",
        "图形图像",
        "游戏娱乐",
        "办公学习",
        "阅读翻译",
        "编程开发",
        "系统管理",
        "其他",
    ];
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
    //let future = fetch_path("https://d.store.deepinos.org.cn//store/chat/applist.json".to_string());
    //let test: String = block_on(future).unwrap();
    //println!("{}",test);
    for (name, url) in names.into_iter().zip(urls) {
        create_tab(&notebook, name, url.to_string());
    }
    notebook
}
fn create_tab(notebook: &Notebook, title: &str, url: String) {
    let lable = Label::new(Some(title));
    let flowbox = gtk::FlowBox::new();
    flowbox.set_valign(gtk::Align::Start);
    flowbox.set_max_children_per_line(30);
    flowbox.set_selection_mode(gtk::SelectionMode::None);
    let scrolled = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    scrolled.add(&flowbox);
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    thread::spawn(move || {
        let future = fetch_message(url.clone() + "applist.json");
        let input2 = block_on(future).unwrap();
        let source: Value = match serde_json::from_str(&input2) {
            Err(_) => Value::Null,
            Ok(output) => output,
        };
        let mut index = 0;
        let mut threads = vec![];
        while source[index] != Value::Null {
            //let mut image = gtk::Image::new();

            let input = remove_quotation(source[index]["Pkgname"].to_string());
            let input = url.clone() + &input + "/icon.png";
            let tx0 = tx.clone();
            let source0 = source[index].clone();
            let t = thread::spawn(move || {
                let future = fetch_path(&input);
                let path = block_on(future).unwrap();
                tx0.send(Some((source0, path))).expect("error");
                drop(tx0);
                drop(input);
            });
            threads.push(t);
            index += 1;
        }
        for thread in threads {
            thread.join().unwrap();
        }
        tx.send(None).expect("error");
        drop(tx);
    });

    rx.attach(None, move |value| match value {
        Some(source) => {
            let (value, path) = source;
            let image = {
                if value["icons"] != Value::Null {
                    let pixbuf = get_pixbuf(path);
                    let pixbuf = pixbuf
                        .scale_simple(160, 160, gtk::gdk_pixbuf::InterpType::Hyper)
                        .unwrap();
                    gtk::Image::from_gicon(&pixbuf, gtk::IconSize::Button)
                } else {
                    let pixbuf =
                        gtk::gdk_pixbuf::Pixbuf::from_resource("/ygo/youxie.jpeg").unwrap();
                    let pixbuf = pixbuf
                        .scale_simple(160, 160, gtk::gdk_pixbuf::InterpType::Hyper)
                        .unwrap();
                    gtk::Image::from_gicon(&pixbuf, gtk::IconSize::Button)
                }
            };
            let boxs = gtk::Box::new(gtk::Orientation::Vertical, 1);
            let button = gtk::Button::new();

            button.add(&image);
            let label = Label::new(Some(&remove_quotation(value["Name"].to_string())));
            label.set_max_width_chars(15);
            label.set_line_wrap(true);
            boxs.pack_start(&button, true, true, 0);
            boxs.pack_start(&label, true, true, 0);
            flowbox.add(&boxs);
            flowbox.show_all();

            button.connect_clicked(|_|{
                GLOBAL_OVERLAY.with(move |global|{
                    if let Some(ref overlay_box) = *global.borrow_mut(){
                        if overlay_box.children().is_empty(){
                            let table = gtk::Label::new(Some("MM"));
                            overlay_box.pack_start(&table,true,true,0);
                            overlay_box.show_all();
                            GLOBAL_TITLE.with(move |global|{
                                if let Some(ref title) = *global.borrow_mut(){
                                    title.switch_title("sss");
                                }
                            });
                        }

                    }
                });
            });

            glib::Continue(true)
        }
        None => glib::Continue(false),
    });

    notebook.append_page(&scrolled, Some(&lable));
}
fn get_pixbuf(bytes: Vec<u8>) -> gtk::gdk_pixbuf::Pixbuf {
    //let bytes = fetch_path(path).await.unwrap();
    let bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = gtk::gio::MemoryInputStream::from_bytes(&bytes);
    let output: gtk::gdk_pixbuf::Pixbuf = match gtk::gdk_pixbuf::Pixbuf::from_stream::<
        gtk::gio::MemoryInputStream,
        gtk::gio::Cancellable,
    >(&stream, None)
    {
        Err(_) => gtk::gdk_pixbuf::Pixbuf::from_resource("/ygo/youxie.jpeg").unwrap(),
        Ok(output) => output,
    };
    output
}
async fn fetch_path(path: &str) -> surf::Result<Vec<u8>> {
    let mut back_string = vec![];
    let url = surf::http::Url::parse(path);
    match url {
        Ok(_) => {
            match surf::get(&path).await {
                Ok(mut response) => {
                    match response.body_bytes().await {
                        Ok(text) => back_string = text.to_vec(),
                        Err(_) => {
                            println!("Read response text Error!")
                        }
                    };
                }
                Err(_) => {
                    println!("reqwest get Error!")
                }
            }
            Ok(back_string)
        }
        Err(_) => Ok(vec![]),
    }
}
async fn fetch_message(path: String) -> surf::Result<String> {
    let mut back_string = String::new();
    let url = surf::http::Url::parse(&path);
    match url {
        Ok(_) => {
            match surf::get(&path).await {
                Ok(mut response) => {
                    match response.body_string().await {
                        Ok(text) => back_string = text,
                        Err(_) => {
                            println!("Read response text Error!")
                        }
                    };
                }
                Err(_) => {
                    println!("reqwest get Error!")
                }
            }
            Ok(back_string)
        }
        Err(_) => Ok(String::new()),
    }
}
fn remove_quotation(input: String) -> String {
    let length = input.len();
    (&input[1..length - 1]).to_string()
}
