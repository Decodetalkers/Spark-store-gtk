use gtk::{Label, Notebook, prelude::{FlowBoxExt, NotebookExt, NotebookExtManual, ScrolledWindowExt, WidgetExt,ContainerExt}};
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
    for name in names.into_iter() {
        let label = Label::new(Some(name));
        flowbox.add(&label);
    }
    notebook.append_page(&scrolled, Some(&lable));
}
