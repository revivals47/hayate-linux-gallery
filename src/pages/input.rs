//! Input widgets page with live value readouts.
//!
//! Every control below binds its callback into a shared `Rc<RefCell<…>>`,
//! and the corresponding `LiveText` echoes the value on repaint — so
//! dragging, typing, or clicking visibly moves the readout.

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
use crate::live::LiveText;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L, theme: Option<&AppTheme>) -> Box<dyn Widget> {
    let themed = theme.is_some();
    let (sec_size, lbl_size) = if themed { (14.0, 11.0) } else { (18.0, 13.0) };
    let label_rgb = if themed { (0, 0, 0) } else { (220, 220, 220) };
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
    // Inline helper macro — closure can't be generic so each `live` call
    // needs its own concrete `LiveText<T>`.
    macro_rules! live {
        ($state:expr, $fmt:expr, $width:expr) => {{
            Box::new(LiveText::new($state, $fmt, lbl_size)
                .with_engine(engine.clone())
                .with_color(label_rgb.0, label_rgb.1, label_rgb.2)
                .with_width($width))
        }};
    }

    // TextInput (no callback API — just render; text is captured inside widget).
    let mut ti1 = TextInputWidget::new(engine.clone())
        .with_placeholder(l.type_here()).with_width(250.0);
    let mut ti2 = TextInputWidget::new(engine.clone())
        .with_placeholder("Input 2").with_width(250.0);
    if let Some(t) = theme {
        ti1 = ti1.theme(t.input.clone());
        ti2 = ti2.theme(t.input.clone());
    }
    let input_row = HStack::new(16.0)
        .add(Box::new(ti1))
        .add(Box::new(ti2));

    // TextArea with char-count readout.
    let ta_state: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let textarea = {
        let s = ta_state.clone();
        TextAreaWidget::new()
            .with_placeholder(l.textarea_placeholder())
            .with_size(500.0, 80.0)
            .on_change(move |text: &str| { *s.borrow_mut() = text.chars().count(); })
    };
    let ta_live = live!(ta_state.clone(), |n: &usize| format!("Chars: {n}"), 160.0);

    // Slider with live value.
    let s1_state: Rc<RefCell<f32>> = Rc::new(RefCell::new(50.0));
    let s2_state: Rc<RefCell<f32>> = Rc::new(RefCell::new(0.3));
    let mut s1 = {
        let s = s1_state.clone();
        SliderWidget::new(0.0, 100.0, 50.0).on_change(move |v| *s.borrow_mut() = v)
    };
    let mut s2 = {
        let s = s2_state.clone();
        SliderWidget::new(0.0, 1.0, 0.3).with_step(0.1).on_change(move |v| *s.borrow_mut() = v)
    };
    if let Some(t) = theme { s1 = s1.theme(t.slider.clone()); s2 = s2.theme(t.slider.clone()); }
    let s1_live = live!(s1_state.clone(), |v: &f32| format!("{:.1} / 100", v), 120.0);
    let s2_live = live!(s2_state.clone(), |v: &f32| format!("{:.2} / 1.00", v), 120.0);
    let slider_section = HStack::new(24.0)
        .add(Box::new(VStack::new(4.0)
            .add(label(l.continuous()))
            .add(Box::new(s1))
            .add(s1_live)))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.stepped()))
            .add(Box::new(s2))
            .add(s2_live)));

    // SpinButton with live value.
    let sp_state: Rc<RefCell<f32>> = Rc::new(RefCell::new(25.0));
    let mut sp = {
        let s = sp_state.clone();
        SpinButtonWidget::new(0.0, 100.0, 25.0, 1.0).on_change(move |v| *s.borrow_mut() = v)
    };
    if let Some(t) = theme { sp = sp.theme(t.spin.clone()); }
    let sp_live = live!(sp_state.clone(), |v: &f32| format!("{:.0}", v), 100.0);

    // Dropdown with live selection.
    let dd_state: Rc<RefCell<String>> = Rc::new(RefCell::new("Tokyo".to_string()));
    let cities = vec![
        "Tokyo".to_string(), "Osaka".to_string(), "Kyoto".to_string(),
        "Nagoya".to_string(), "Fukuoka".to_string(),
    ];
    let mut dd = {
        let s = dd_state.clone();
        DropdownWidget::new(cities.clone())
            .with_selected(0)
            .on_select(move |_i, name| { *s.borrow_mut() = name.to_string(); })
    };
    if let Some(t) = theme { dd = dd.theme(t.dropdown.clone()); }
    let dd_live = live!(dd_state.clone(), |s: &String| format!("City: {s}"), 160.0);

    // RadioGroup with live selection.
    let rg_options = ["Small", "Medium", "Large"];
    let rg_state: Rc<RefCell<usize>> = Rc::new(RefCell::new(1));
    let rg = {
        let s = rg_state.clone();
        RadioGroupWidget::new(&rg_options)
            .with_selected(1)
            .with_engine(engine.clone())
            .on_change(move |i| *s.borrow_mut() = i)
    };
    let rg_live = live!(rg_state.clone(),
        move |i: &usize| format!("Size: {}", rg_options.get(*i).unwrap_or(&"?")),
        160.0);

    let controls_row = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0)
            .add(label(l.spin_button()))
            .add(Box::new(sp))
            .add(sp_live)))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.dropdown()))
            .add(Box::new(dd))
            .add(dd_live)))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.radio_group()))
            .add(Box::new(rg))
            .add(rg_live)));

    // ColorPicker with live RGB readout.
    let cp_state: Rc<RefCell<(u8, u8, u8)>> = Rc::new(RefCell::new((100, 150, 200)));
    let picker = {
        let s = cp_state.clone();
        ColorPickerWidget::new(Color::rgba(100, 150, 200, 255))
            .on_change(move |c| *s.borrow_mut() = (c.r, c.g, c.b))
    };
    let cp_live = live!(cp_state.clone(),
        |c: &(u8, u8, u8)| format!("RGB({}, {}, {})", c.0, c.1, c.2),
        200.0);

    let content = VStack::new(12.0)
        .add(section(l.text_input()))
        .add(Box::new(input_row))
        .add(section(l.text_area()))
        .add(Box::new(HStack::new(16.0).add(Box::new(textarea)).add(ta_live)))
        .add(section(l.slider()))
        .add(Box::new(slider_section))
        .add(Box::new(controls_row))
        .add(section(l.color_picker()))
        .add(Box::new(HStack::new(16.0).add(Box::new(picker)).add(cp_live)));

    Box::new(Padding::all(16.0, Box::new(content)))
}
