use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{TreeNode, TreeViewWidget};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct TreeViewDemo;

impl Demo for TreeViewDemo {
    fn id(&self) -> &'static str { "tree_view" }
    fn category(&self) -> Category { Category::Display }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "ツリービュー", Lang::En => "Tree View" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (root_lbl, src, tests, toml, main, lib, itest) = match ctx.lang {
            Lang::Ja => ("プロジェクト", "src", "tests", "Cargo.toml", "main.rs", "lib.rs", "integration.rs"),
            Lang::En => ("Project", "src", "tests", "Cargo.toml", "main.rs", "lib.rs", "integration.rs"),
        };
        let root = TreeNode::new(root_lbl).with_children(vec![
            TreeNode::new(src).with_children(vec![
                TreeNode::new(main),
                TreeNode::new(lib),
            ]),
            TreeNode::new(tests).with_children(vec![TreeNode::new(itest)]),
            TreeNode::new(toml),
        ]);
        Box::new(
            TreeViewWidget::new(vec![root])
                .on_select(|p| println!("tree select: {p:?}")),
        )
    }
}

inventory::submit!(DemoEntry { demo: &TreeViewDemo });
