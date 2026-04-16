//! Basic widgets page: Button (with click counter), Checkbox, Switch, Text.
//!
//! Each interactive widget drives a live state display so you can tell at a
//! glance whether the widget is firing its callbacks — this is the difference
//! between "widgets rendered" and "widgets validated".

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, ButtonWidget, ButtonTheme, CheckboxWidget, CheckStyle, SwitchWidget, TextWidget,
    HStack, VStack, Padding,
};

use crate::i18n::L;
use crate::live::LiveText;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L, theme: Option<&AppTheme>) -> Box<dyn Widget> {
    let themed = theme.is_some();
    let (sec_size, lbl_size) = if themed { (14.0, 11.0) } else { (18.0, 12.0) };
    let label_color = if themed { (0, 0, 0) } else { (220, 220, 220) };
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
    // ── Button with click counter ──
    let click_state: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let click_live = {
        let s = click_state.clone();
        Box::new(LiveText::new(s, |n: &u32| format!("Clicked: {n}"), lbl_size)
            .with_engine(engine.clone())
            .with_color(label_color.0, label_color.1, label_color.2)
            .with_width(140.0))
    };

    let mk_counted_button = |label: &str, theme_override: Option<ButtonTheme>, state: Rc<RefCell<u32>>| -> Box<dyn Widget> {
        let mut b = ButtonWidget::new(label);
        if let Some(t) = theme_override { b = b.theme(t); }
        b = b.on_click(move || { *state.borrow_mut() += 1; });
        Box::new(b)
    };

    let (button_grid, showcase_rows): (Box<dyn Widget>, Vec<Box<dyn Widget>>) = if let Some(t) = theme {
        // Themed mode: show only themed buttons, with click counters.
        let row1 = HStack::new(6.0)
            .add(mk_counted_button("OK", Some(t.button.clone()), click_state.clone()))
            .add(mk_counted_button("Cancel", Some(t.button.clone()), click_state.clone()))
            .add(mk_counted_button("Apply", Some(t.button.clone()), click_state.clone()));
        (Box::new(row1), vec![])
    } else {
        let r1 = HStack::new(6.0)
            .add(mk_counted_button("Filled", None, click_state.clone()))
            .add(mk_counted_button("Outlined", Some(ButtonTheme::outlined()), click_state.clone()))
            .add(mk_counted_button("Ghost", Some(ButtonTheme::ghost()), click_state.clone()))
            .add(mk_counted_button("Pill", Some(ButtonTheme::pill()), click_state.clone()))
            .add(mk_counted_button("Elevated", Some(ButtonTheme::elevated()), click_state.clone()));
        let r2 = HStack::new(6.0)
            .add(Box::new(ButtonWidget::new("Gradient").theme(ButtonTheme::gradient_accent())))
            .add(Box::new(ButtonWidget::new("Glass").theme(ButtonTheme::glass())))
            .add(Box::new(ButtonWidget::new("Neon").theme(ButtonTheme::neon())))
            .add(Box::new(ButtonWidget::new("Soft").theme(ButtonTheme::soft())));
        let r3 = HStack::new(6.0)
            .add(Box::new(ButtonWidget::new("Win11").theme(ButtonTheme::win11())))
            .add(Box::new(ButtonWidget::new("Win11 Sec").theme(ButtonTheme::win11_secondary())))
            .add(Box::new(ButtonWidget::new("macOS").theme(ButtonTheme::macos())))
            .add(Box::new(ButtonWidget::new("macOS Sec").theme(ButtonTheme::macos_secondary())));
        let r4 = HStack::new(6.0)
            .add(Box::new(ButtonWidget::new("Material 3").theme(ButtonTheme::material3())))
            .add(Box::new(ButtonWidget::new("Fluent").theme(ButtonTheme::fluent())))
            .add(Box::new(ButtonWidget::new("Adwaita").theme(ButtonTheme::adwaita_suggested())))
            .add(Box::new(ButtonWidget::new("Win95").theme(ButtonTheme::win95())))
            .add(Box::new(ButtonWidget::new("Mac OS 9").theme(ButtonTheme::mac_classic())));
        (Box::new(r1), vec![Box::new(r2), Box::new(r3), Box::new(r4)])
    };

    let buttons_with_counter = HStack::new(16.0)
        .add(button_grid)
        .add(click_live);

    // ── Checkbox with live state ──
    let cb_state: Rc<RefCell<(bool, bool, bool)>> = Rc::new(RefCell::new((false, true, false)));
    let mk_cb = |lbl: String, start_checked: bool, setter: Box<dyn Fn(&mut (bool, bool, bool), bool)>| -> Box<dyn Widget> {
        let s = cb_state.clone();
        let mut cb = CheckboxWidget::new(lbl).checked(start_checked)
            .with_engine(engine.clone());
        if let Some(t) = theme {
            cb = cb.theme(t.checkbox.clone());
        }
        cb = cb.on_toggle(move |v| setter(&mut *s.borrow_mut(), v));
        Box::new(cb)
    };

    let cb_row = if theme.is_some() {
        HStack::new(16.0)
            .add(mk_cb(l.option_a().to_string(), false, Box::new(|s, v| s.0 = v)))
            .add(mk_cb(l.option_b_checked().to_string(), true, Box::new(|s, v| s.1 = v)))
            .add(mk_cb(l.option_c().to_string(), false, Box::new(|s, v| s.2 = v)))
    } else {
        HStack::new(16.0)
            .add(Box::new(VStack::new(4.0)
                .add(label("Filled"))
                .add(mk_cb(l.option_a().to_string(), false, Box::new(|s, v| s.0 = v)))
                .add(mk_cb(l.option_b_checked().to_string(), true, Box::new(|s, v| s.1 = v)))))
            .add(Box::new(VStack::new(4.0)
                .add(label("Classic"))
                .add(Box::new(CheckboxWidget::new(l.option_a()).style(CheckStyle::Classic)))
                .add(Box::new(CheckboxWidget::new(l.option_b_checked()).style(CheckStyle::Classic).checked(true)))))
            .add(Box::new(VStack::new(4.0)
                .add(label("Minimal"))
                .add(Box::new(CheckboxWidget::new(l.option_a()).style(CheckStyle::Minimal)))
                .add(Box::new(CheckboxWidget::new(l.option_b_checked()).style(CheckStyle::Minimal).checked(true)))))
    };

    let cb_live = Box::new(LiveText::new(cb_state.clone(),
        |s: &(bool, bool, bool)| format!("A={} B={} C={}",
            if s.0 {"on"} else {"off"},
            if s.1 {"on"} else {"off"},
            if s.2 {"on"} else {"off"}),
        lbl_size)
        .with_engine(engine.clone())
        .with_color(label_color.0, label_color.1, label_color.2)
        .with_width(240.0));

    // ── Switch with live state ──
    let sw_state: Rc<RefCell<(bool, bool)>> = Rc::new(RefCell::new((false, true)));
    let mk_sw = |start: bool, setter: Box<dyn Fn(&mut (bool, bool), bool)>| -> Box<dyn Widget> {
        let s = sw_state.clone();
        let mut w = SwitchWidget::new(start);
        if let Some(t) = theme { w = w.theme(t.switch.clone()); }
        w = w.on_toggle(move |v| setter(&mut *s.borrow_mut(), v));
        Box::new(w)
    };

    let switch_row = if theme.is_some() {
        HStack::new(16.0)
            .add(label("A"))
            .add(mk_sw(false, Box::new(|s, v| s.0 = v)))
            .add(label("B"))
            .add(mk_sw(true, Box::new(|s, v| s.1 = v)))
    } else {
        HStack::new(16.0)
            .add(label(l.dark_mode()))
            .add(mk_sw(false, Box::new(|s, v| s.0 = v)))
            .add(label(l.notifications()))
            .add(mk_sw(true, Box::new(|s, v| s.1 = v)))
    };

    let sw_live = Box::new(LiveText::new(sw_state.clone(),
        |s: &(bool, bool)| format!("A={} / B={}",
            if s.0 {"ON"} else {"OFF"},
            if s.1 {"ON"} else {"OFF"}),
        lbl_size)
        .with_engine(engine.clone())
        .with_color(label_color.0, label_color.1, label_color.2)
        .with_width(200.0));

    // ── CJK text ──
    let cjk = VStack::new(4.0)
        .add(label(l.text_sample()))
        .add(Box::new(TextWidget::new("한국어 렌더링", 14.0).with_engine(engine.clone())))
        .add(Box::new(TextWidget::new("简体中文渲染", 14.0).with_engine(engine.clone())))
        .add(Box::new(TextWidget::new("繁體中文渲染", 14.0).with_engine(engine.clone())));

    // ── Assemble ──
    let mut content = VStack::new(10.0)
        .add(section(l.button()))
        .add(Box::new(buttons_with_counter));
    for row in showcase_rows {
        content = content.add(row);
    }
    content = content
        .add(section(l.checkbox()))
        .add(Box::new(HStack::new(16.0).add(Box::new(cb_row)).add(cb_live)))
        .add(section(l.switch()))
        .add(Box::new(HStack::new(16.0).add(Box::new(switch_row)).add(sw_live)))
        .add(section(l.text()))
        .add(Box::new(cjk));

    Box::new(Padding::all(14.0, Box::new(content)))
}
