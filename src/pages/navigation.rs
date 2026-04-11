//! Navigation widgets page: TabView, TreeView.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    TextWidget, TabViewWidget, TabEntry,
    TreeViewWidget, TreeNode,
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

    let nav_row = HStack::new(24.0)
        .add(Box::new(VStack::new(4.0)
            .add(label(l.tab_view_nested()))
            .add(Box::new(inner_tabs))))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.tree_view()))
            .add(Box::new(tree))));

    let content = VStack::new(16.0)
        .add(section(l.nav_widgets()))
        .add(Box::new(nav_row));

    Box::new(Padding::all(20.0, Box::new(content)))
}
