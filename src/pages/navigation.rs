//! Navigation widgets: TabView, TreeView, Breadcrumb, NavigationList.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    TextWidget, TabViewWidget, TabEntry,
    TreeViewWidget, TreeNode,
    BreadcrumbWidget, NavigationListWidget, NavItem,
    HStack, VStack, Padding,
};

use crate::i18n::L;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L) -> Box<dyn Widget> {
    let section = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 18.0).with_engine(engine.clone()))
    };
    let label = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 13.0).with_engine(engine.clone()))
    };

    // Breadcrumb
    let breadcrumb = BreadcrumbWidget::new()
        .with_segments(&["Home", "Documents", "Projects", "hayate-ui"])
        .with_engine(engine.clone());

    // TabView + TreeView row
    let inner_tabs = TabViewWidget::new()
        .add_tab(TabEntry::new("A", Box::new(Padding::all(12.0, label("Content A")))))
        .add_tab(TabEntry::new("B", Box::new(Padding::all(12.0, label("Content B")))))
        .add_tab(TabEntry::new("C", Box::new(Padding::all(12.0, label("Content C")))));

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
            .add(Box::new(inner_tabs))))
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
