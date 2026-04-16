use hayate_ui::style::theme::Color;
use hayate_ui::widget::ColorPickerWidget;
use hayate_ui::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ColorPickerDemo;

impl Demo for ColorPickerDemo {
    fn id(&self) -> &'static str { "color_picker" }
    fn category(&self) -> Category { Category::Input }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "カラーピッカー", Lang::En => "Color Picker" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        Box::new(
            ColorPickerWidget::new(Color::rgb(80, 140, 220))
                .on_change(|c| println!("color: #{:02X}{:02X}{:02X}", c.r, c.g, c.b)),
        )
    }
}

inventory::submit!(DemoEntry { demo: &ColorPickerDemo });
