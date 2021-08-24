use std::cell::RefCell;
use gtk::StackSwitcher;
use gtk::prelude::*;
pub struct Title {
    pub title_label: gtk::Label,
    pub stackswitcher: StackSwitcher,
    pub button_back: gtk::Button,
}

// Download and search page
// Title widget
thread_local! {
    pub static GLOBAL_OVERLAY: RefCell<Option<gtk::Box>> = RefCell::new(None);
    pub static GLOBAL_TITLE: RefCell<Option<Title>> = RefCell::new(None);
}
impl Title {
    pub fn switch_title(&self,title: &str){
        self.title_label.set_text(title);
        self.title_label.show();
        self.stackswitcher.hide();
        self.button_back.show();
    }
    pub fn switch_stack(&self){
        self.title_label.hide();
        self.stackswitcher.show();
    }
}
