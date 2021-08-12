use gdtk::add_corner;
use gtk::{prelude::*, HeaderBar, Stack, StackSwitcher};
mod page1;
fn main() {
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
    // 绘制圆角
    add_corner(&window, 0.02);
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
    let page1 = page1::mainpage();
    stack.add_titled(&page1, "lable", "label");
    let lable2 = gtk::Label::new(Some("BB"));
    stack.add_titled(&lable2, "lable2", "label2");

    let stackswitcher = StackSwitcher::new();
    stackswitcher.set_stack(Some(&stack));
    //title.add(&stackswitcher);
    //title.set_child(Some(&stackswitcher));
    // pack_start是左边
    // set_custom_title 设置标题控件
    // pack_end 设置右起
    title.set_custom_title(Some(&stackswitcher));

    //vbox.pack_start(&lable, true, true, 0);
    window.add(&stack);
    window.show_all();
}
