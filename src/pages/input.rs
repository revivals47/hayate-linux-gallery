//! Input widgets page: TextInput, Slider, SpinButton, Dropdown, ColorPicker.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::style::theme::Color;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    TextWidget, TextInputWidget, SliderWidget, SpinButtonWidget,
    DropdownWidget, ColorPickerWidget,
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

    let input_row = HStack::new(16.0)
        .add(Box::new(TextInputWidget::new(engine.clone())
            .with_placeholder(l.type_here()).with_width(250.0)))
        .add(Box::new(TextInputWidget::new(engine.clone())
            .with_placeholder("Input 2").with_width(250.0)));

    let slider_section = VStack::new(8.0)
        .add(label(l.continuous()))
        .add(Box::new(SliderWidget::new(0.0, 100.0, 50.0)))
        .add(label(l.stepped()))
        .add(Box::new(SliderWidget::new(0.0, 1.0, 0.3).with_step(0.1)));

    let spin_row = HStack::new(16.0)
        .add(label(l.integer()))
        .add(Box::new(SpinButtonWidget::new(0.0, 100.0, 25.0, 1.0)))
        .add(label(l.float_val()))
        .add(Box::new(SpinButtonWidget::new(0.0, 1.0, 0.5, 0.05)));

    let dropdown_row = HStack::new(16.0)
        .add(label(l.city()))
        .add(Box::new(DropdownWidget::new(vec![
            "Tokyo".into(), "Osaka".into(), "Kyoto".into(),
            "Nagoya".into(), "Fukuoka".into(),
        ]).with_selected(0)));

    let content = VStack::new(16.0)
        .add(section(l.text_input()))
        .add(Box::new(input_row))
        .add(section(l.slider()))
        .add(Box::new(slider_section))
        .add(section(l.spin_button()))
        .add(Box::new(spin_row))
        .add(section(l.dropdown()))
        .add(Box::new(dropdown_row))
        .add(section(l.color_picker()))
        .add(Box::new(ColorPickerWidget::new(Color::rgba(100, 150, 200, 255))));

    Box::new(Padding::all(20.0, Box::new(content)))
}
