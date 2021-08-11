use gtk::{
   HeaderBar,
   prelude::*
};
fn main() {
    let application = gtk::Application::new(
        Some("come.test.add"), 
        Default::default()
    );
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &gtk::Application){
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("test");
    window.set_default_size(350, 70);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical,10);
    let title = HeaderBar::new();
    title.set_show_close_button(true);
    //vbox.pack_start(&title, false, false, 0);
    let lable = gtk::Label::new(Some("MM"));
    window.set_titlebar(Some(&title));
    vbox.pack_start(&lable, true, true, 0);
    window.add(&vbox);
    window.show_all();
}
