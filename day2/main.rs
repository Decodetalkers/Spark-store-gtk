use gtk::{
   HeaderBar,
   Stack,
   StackSwitcher,
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
    //window.set_title("test");
    window.set_default_size(350, 70);
    //let vbox = gtk::Box::new(gtk::Orientation::Vertical,10);
    let title = HeaderBar::new();
    title.set_show_close_button(true);
    window.set_titlebar(Some(&title));
    //vbox.pack_start(&title, false, false, 0);
    
    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(1000);
    let lable = gtk::Label::new(Some("MM"));
    stack.add_titled(&lable, "lable", "label");
    let lable2 = gtk::Label::new(Some("BB"));
    stack.add_titled(&lable2, "lable2", "label2");

    let stackswitcher = StackSwitcher::new();
    stackswitcher.set_stack(Some(&stack));
    title.add(&stackswitcher);
    

    //vbox.pack_start(&lable, true, true, 0);
    window.add(&stack);
    window.show_all();
}
