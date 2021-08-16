use gtk::{Label, Notebook, prelude::*};
use futures::executor::block_on;
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
    let future = fetch_path("https://d.store.deepinos.org.cn//store/chat/applist.json".to_string());
    let test: String = block_on(future).unwrap();
    println!("{}",test);
    for name in names.into_iter() {
        create_tab(&notebook, name);
    }
    notebook
}
fn create_tab(notebook: &Notebook, title: &str) {
    let lable = Label::new(Some(title));
    //let lable2 = Label::new(Some(title));
    let flowbox = gtk::FlowBox::new();
    flowbox.set_valign(gtk::Align::Start);
    flowbox.set_max_children_per_line(30);
    flowbox.set_selection_mode(gtk::SelectionMode::None);
    let scrolled = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    scrolled.add(&flowbox);
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

    let future = fetch_path("https://examine-spark.oss-cn-shanghai.aliyuncs.com/icons/2020/11/19/7f3f6130-2a5c-11eb-848c-e55ce84765f9.svg".to_string());
    let image = block_on(future).unwrap();
    //let stream = gtk::gio::MemoryInputStream::from_bytes(&gtk::glib::Bytes::from(image.as_bytes()));
    let loader = gtk::gdk_pixbuf::PixbufLoader::new();
    loader.write(image.as_bytes()).unwrap();
    loader.close().unwrap();
    let pixbuf = loader.pixbuf().unwrap();
    //let icon = gtk::Image::from_gicon(&pixbuf, gtk::IconSize::Button);
    for name in names.into_iter() {
        let boxs = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let button = gtk::Button::new();
        //let image  =gtk::gio::Icon::for_string("edit-find-symbolic").unwrap();
        let image2 = gtk::Image::from_gicon(&pixbuf.clone(), gtk::IconSize::Button);

        //let image2 = gtk::Image::from_gicon(&image, gtk::IconSize::Button);
        
        button.add(&image2);
        let label = Label::new(Some(name));
        boxs.pack_start(&button, true, true, 0);
        boxs.pack_start(&label, true, true, 0);
        flowbox.add(&boxs);
    }
    notebook.append_page(&scrolled, Some(&lable));
}
async fn fetch_path(path: String) -> surf::Result<String> {
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
