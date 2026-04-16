//! Navigation widgets: TabView, TreeView, Breadcrumb, NavigationList.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, TextWidget, TabViewWidget, TabEntry,
    TreeViewWidget, TreeNode,
    BreadcrumbWidget, NavigationListWidget, NavItem,
    HStack, VStack, Padding,
};

use crate::i18n::L;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L, theme: Option<&AppTheme>) -> Box<dyn Widget> {
    let themed = theme.is_some();
    let (sec_size, lbl_size) = if themed { (14.0, 11.0) } else { (18.0, 13.0) };
    let section = |text: &str| -> Box<dyn Widget> {
        let mut w = TextWidget::new(text, sec_size).with_engine(engine.clone());
        if themed { w = w.with_color(0, 0, 0); }
        Box::new(w)
    };
    let label = |text: &str| -> Box<dyn Widget> {
        let mut w = TextWidget::new(text, lbl_size).with_engine(engine.clone());
        if themed { w = w.with_color(0, 0, 0); }
        Box::new(w)
    };

    // Breadcrumb
    let breadcrumb = BreadcrumbWidget::new()
        .with_segments(&["Home", "Documents", "Projects", "hayate-ui"])
        .with_engine(engine.clone());

    // TabView + TreeView row. Nested TabView picks up the theme so the
    // sub-tabs match the chrome.
    let tab_state: std::rc::Rc<std::cell::RefCell<usize>> = std::rc::Rc::new(std::cell::RefCell::new(0));
    let tab_state_cb = tab_state.clone();
    let mut inner_tabs = TabViewWidget::new()
        .add_tab(TabEntry::new("A", Box::new(Padding::all(12.0, label("Content A")))))
        .add_tab(TabEntry::new("B", Box::new(Padding::all(12.0, label("Content B")))))
        .add_tab(TabEntry::new("C", Box::new(Padding::all(12.0, label("Content C")))))
        .on_change(move |i| *tab_state_cb.borrow_mut() = i);
    if let Some(t) = theme { inner_tabs = inner_tabs.theme(t.tab.clone()); }
    let tab_label = ["A", "B", "C"];
    let tab_live = crate::live::LiveText::new(
        tab_state.clone(),
        move |i: &usize| format!("Active: {}", tab_label.get(*i).unwrap_or(&"?")),
        lbl_size,
    )
        .with_engine(engine.clone())
        .with_color(if themed {0} else {220}, if themed {0} else {220}, if themed {0} else {220})
        .with_width(120.0);

    let tree = TreeViewWidget::new(vec![
        TreeNode::new("src").with_children(vec![
            TreeNode::new("main.rs"),
            TreeNode::new("lib.rs"),
            TreeNode::new("widget").with_children(vec![
                TreeNode::new("core.rs"),
                TreeNode::new("basic.rs"),
            ]),
        ]),
        TreeNode::new("Cargo.toml"),
    ]);

    // NavigationList
    let nav = NavigationListWidget::new()
        .with_items(vec![
            NavItem::section("Favorites"),
            NavItem::entry_with_icon("H", "Home"),
            NavItem::entry_with_icon("D", "Documents"),
            NavItem::entry_with_icon("P", "Downloads"),
            NavItem::section("Devices"),
            NavItem::entry_with_icon("S", "SSD 500GB"),
        ]);

    let row1 = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0)
            .add(label(l.tab_view_nested()))
            .add(Box::new(inner_tabs))
            .add(Box::new(tab_live))))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.tree_view()))
            .add(Box::new(tree))))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.nav_list()))
            .add(Box::new(nav))));

    let content = VStack::new(12.0)
        .add(section(l.breadcrumb()))
        .add(Box::new(breadcrumb))
        .add(section(l.nav_widgets()))
        .add(Box::new(row1));

    Box::new(Padding::all(16.0, Box::new(content)))
}
