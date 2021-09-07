use futures::executor::block_on;
use gtk::prelude::*;
use gtk::StackSwitcher;
use std::cell::RefCell;
pub struct Title {
    pub title_label: gtk::Label,
    pub stackswitcher: StackSwitcher,
    pub button_back: gtk::Button,
}

// Download and search page
// Title widget
thread_local! {
    pub static GLOBAL_OVERLAY: RefCell<Option<gtk::Box>> = RefCell::new(None);
    pub static GLOBAL_DOWNLOAD: RefCell<Option<gtk::Box>> = RefCell::new(None);
    pub static GLOBAL_TITLE: RefCell<Option<Title>> = RefCell::new(None);
}
impl Title {
    pub fn switch_title(&self, title: &str) {
        self.title_label.set_text(title);
        self.title_label.show();
        self.stackswitcher.hide();
        self.button_back.show();
    }
    pub fn switch_stack(&self) {
        self.button_back.hide();
        self.title_label.hide();
        self.stackswitcher.show();
    }
}
pub fn fetch_pic(input: &str) -> Vec<u8> {
    let future = fetch_path(input);
    block_on(future).unwrap()
}
pub fn fetch_message(input: String) -> String {
    let future = fetch_message_before(input);
    block_on(future).unwrap()
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
async fn fetch_message_before(path: String) -> surf::Result<String> {
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
pub fn remove_quotation(input: String) -> String {
    let length = input.len();
    (&input[1..length - 1]).to_string()
}
pub fn get_pixbuf(bytes: Vec<u8>) -> gtk::gdk_pixbuf::Pixbuf {
    let bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = gtk::gio::MemoryInputStream::from_bytes(&bytes);
    let output: gtk::gdk_pixbuf::Pixbuf = match gtk::gdk_pixbuf::Pixbuf::from_stream::<
        gtk::gio::MemoryInputStream,
        gtk::gio::Cancellable,
    >(&stream, None)
    {
        Err(_) => gtk::gdk_pixbuf::Pixbuf::from_resource("/ygo/akalin.png").unwrap(),
        Ok(output) => output,
    };
    output
}
