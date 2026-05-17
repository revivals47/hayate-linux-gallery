use hayate_platform::widget::core::Widget;
use hayate_kit::widget::menu_bar::{Menu, MenuBarWidget};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct MenuBarDemo;

impl Demo for MenuBarDemo {
    fn id(&self) -> &'static str { "menu_bar" }
    fn category(&self) -> Category { Category::Navigation }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "メニューバー", Lang::En => "Menu Bar" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        // `&X` marks the mnemonic; under --win95 the char gets an
        // underline, modern themes just strip the `&`.
        let (file, edit, new, open, save, undo, redo) = match ctx.lang {
            Lang::Ja => ("ファイル(&F)", "編集(&E)", "新規(&N)", "開く(&O)",
                         "保存(&S)", "元に戻す(&U)", "やり直し(&R)"),
            Lang::En => ("&File", "&Edit", "&New", "&Open", "&Save", "&Undo", "&Redo"),
        };
        let file_menu = Menu::new(file)
            .add_action(new,  || println!("menu: new"))
            .add_action(open, || println!("menu: open"))
            .add_separator()
            .add_action(save, || println!("menu: save"));
        let edit_menu = Menu::new(edit)
            .add_action(undo, || println!("menu: undo"))
            .add_action(redo, || println!("menu: redo"));
        let bar = MenuBarWidget::new().add_menu(file_menu).add_menu(edit_menu);
        // MenuBarWidget is painted without an engine by default; no setup here
        // since widget inject_engine injects via the tree.
        let _ = ctx;
        Box::new(bar)
    }
}

inventory::submit!(DemoEntry { demo: &MenuBarDemo });
