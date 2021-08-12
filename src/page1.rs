use gtk::{
    prelude::{NotebookExt, NotebookExtManual},
    Label, Notebook,
};
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
    let lable2 = Label::new(Some(title));
    notebook.append_page(&lable2, Some(&lable));
}
