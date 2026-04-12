//! Input widgets: TextInput, TextArea, Slider, SpinButton, Dropdown,
//! RadioGroup, ColorPicker.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::style::theme::Color;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, TextWidget, TextInputWidget, TextAreaWidget, SliderWidget, SpinButtonWidget,
    DropdownWidget, RadioGroupWidget, ColorPickerWidget,
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

    // TextInput
    let input_row = HStack::new(16.0)
        .add(Box::new(TextInputWidget::new(engine.clone())
            .with_placeholder(l.type_here()).with_width(250.0)))
        .add(Box::new(TextInputWidget::new(engine.clone())
            .with_placeholder("Input 2").with_width(250.0)));

    // TextArea
    let textarea = TextAreaWidget::new()
        .with_placeholder(l.textarea_placeholder())
        .with_size(500.0, 80.0);

    // Slider
    let mut s1 = SliderWidget::new(0.0, 100.0, 50.0);
    let mut s2 = SliderWidget::new(0.0, 1.0, 0.3).with_step(0.1);
    if let Some(t) = theme { s1 = s1.theme(t.slider.clone()); s2 = s2.theme(t.slider.clone()); }
    let slider_section = HStack::new(24.0)
        .add(Box::new(VStack::new(4.0).add(label(l.continuous())).add(Box::new(s1))))
        .add(Box::new(VStack::new(4.0).add(label(l.stepped())).add(Box::new(s2))));

    // SpinButton + Dropdown + RadioGroup row
    let mut sp = SpinButtonWidget::new(0.0, 100.0, 25.0, 1.0);
    let mut dd = DropdownWidget::new(vec![
        "Tokyo".into(), "Osaka".into(), "Kyoto".into(),
        "Nagoya".into(), "Fukuoka".into(),
    ]).with_selected(0);
    if let Some(t) = theme { sp = sp.theme(t.spin.clone()); dd = dd.theme(t.dropdown.clone()); }
    let controls_row = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0).add(label(l.spin_button())).add(Box::new(sp))))
        .add(Box::new(VStack::new(4.0).add(label(l.dropdown())).add(Box::new(dd))))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.radio_group()))
            .add(Box::new(RadioGroupWidget::new(&["Small", "Medium", "Large"])
                .with_selected(1)
                .with_engine(engine.clone())))));

    // ColorPicker
    let picker = ColorPickerWidget::new(Color::rgba(100, 150, 200, 255));

    let content = VStack::new(12.0)
        .add(section(l.text_input()))
        .add(Box::new(input_row))
        .add(section(l.text_area()))
        .add(Box::new(textarea))
        .add(section(l.slider()))
        .add(Box::new(slider_section))
        .add(Box::new(controls_row))
        .add(section(l.color_picker()))
        .add(Box::new(picker));

    Box::new(Padding::all(16.0, Box::new(content)))
}
