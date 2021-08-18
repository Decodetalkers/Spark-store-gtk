use gtk::{Label, Notebook, prelude::*};
use futures::executor::block_on;
use serde_json::Value;
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
    //let future = fetch_path("https://d.store.deepinos.org.cn//store/chat/applist.json".to_string());
    //let test: String = block_on(future).unwrap();
    //println!("{}",test);
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

    //let future = fetch_path("https://examine-spark.oss-cn-shanghai.aliyuncs.com/images/2021/01/24/4822c780-5dfd-11eb-a7e8-39f472e1f1d8.png".to_string());
    //let image = block_on(future).unwrap();
    //let stream = gtk::gio::MemoryInputStream::from_bytes(&gtk::glib::Bytes::from(image.as_bytes()));
    //let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_stream::<gtk::gio::MemoryInputStream,gtk::gio::Cancellable>(&stream,None).unwrap();
    let future = get_pixbuf("https://examine-spark.oss-cn-shanghai.aliyuncs.com/icons/2021/01/24/4529abc0-5dfd-11eb-a7e8-39f472e1f1d8.png");
    let pixbuf = block_on(future);


    //let loader = gtk::gdk_pixbuf::PixbufLoader::new();
    //loader.write(image.as_bytes()).unwrap();
    //loader.close().unwrap();
    //let pixbuf = loader.pixbuf().unwrap();
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
async fn get_pixbuf(path:&str) -> gtk::gdk_pixbuf::Pixbuf {
    let bytes = fetch_path(path).await.unwrap();
    let bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = gtk::gio::MemoryInputStream::from_bytes(&bytes);
    gtk::gdk_pixbuf::Pixbuf::from_stream::<gtk::gio::MemoryInputStream,gtk::gio::Cancellable>(&stream,None).unwrap()

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
