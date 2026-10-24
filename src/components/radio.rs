use super::Icon;
use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};
use std::rc::Rc;

pub fn radio_icon(checked: bool, cx: &App) -> Div {
    let brand = Theme::get(cx).brand;
    div().size(px(24.)).flex_shrink_0().when(checked, |el| {
        el.child(Icon::new("checked").size(24.).color(brand))
    })
}

type SelectHandler = Rc<dyn Fn(&usize, &mut Window, &mut App)>;

pub struct RadioGroup {
    id: ElementId,
    options: Vec<(SharedString, bool)>,
    selected: Option<usize>,
    on_change: Option<SelectHandler>,
}

impl RadioGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            options: vec![],
            selected: None,
            on_change: None,
        }
    }
    pub fn option(mut self, label: impl Into<SharedString>) -> Self {
        self.options.push((label.into(), false));
        self
    }
    pub fn disabled_option(mut self, label: impl Into<SharedString>) -> Self {
        self.options.push((label.into(), true));
        self
    }
    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn rows(self, window: &mut Window, cx: &mut App) -> Vec<AnyElement> {
        let t = Theme::get(cx).clone();
        let n = self.options.len();
        let handles = window.use_keyed_state(self.id.clone(), cx, |_, _| Vec::<FocusHandle>::new());
        let handles = handles.update(cx, |v, cx| {
            while v.len() < n {
                v.push(cx.focus_handle());
            }
            v.clone()
        });
        let enabled: Vec<bool> = self.options.iter().map(|(_, d)| !d).collect();
        let tab_index = self
            .selected
            .filter(|i| enabled.get(*i) == Some(&true))
            .or_else(|| enabled.iter().position(|e| *e));
        let pressed = t.fg_3;
        let brand = t.brand;
        self.options
            .into_iter()
            .enumerate()
            .map(|(i, (label, disabled))| {
                let checked = self.selected == Some(i);
                let on_change = self.on_change.clone();
                let on_key = self.on_change.clone();
                let handles = handles.clone();
                let enabled = enabled.clone();
                let mut el = super::form::cell_frame(ElementId::NamedInteger(
                    "radio-option".into(),
                    i as u64,
                ))
                .role(Role::RadioButton)
                .aria_label(label.clone())
                .aria_toggled(if checked {
                    Toggled::True
                } else {
                    Toggled::False
                })
                .aria_position_in_set(i + 1)
                .aria_size_of_set(n)
                .text_color(if disabled { t.fg_2 } else { t.fg_0 })
                .child(div().flex_1().min_w_0().child(label))
                .child(
                    div()
                        .pl(px(16.))
                        .child(radio_icon(checked, cx).when(disabled, |el| el.opacity(0.3))),
                );
                if !disabled {
                    el = el
                        .track_focus(&handles[i].clone().tab_stop(tab_index == Some(i)))
                        .focusable()
                        .cursor_pointer()
                        .active(move |s| s.bg(pressed).rounded(px(8.)))
                        .focus_visible(move |s| s.border_color(brand))
                        .on_click(move |_, window, cx| {
                            if let Some(h) = on_change.as_ref() {
                                h(&i, window, cx);
                            }
                        })
                        .on_key_down(move |event, window, cx| {
                            let step: isize = match event.keystroke.key.as_str() {
                                "down" | "right" => 1,
                                "up" | "left" => -1,
                                _ => return,
                            };
                            cx.stop_propagation();
                            let mut j = i as isize;
                            for _ in 0..n {
                                j = (j + step).rem_euclid(n as isize);
                                if enabled[j as usize] {
                                    break;
                                }
                            }
                            let j = j as usize;
                            window.focus(&handles[j], cx);
                            if let Some(h) = on_key.as_ref() {
                                h(&j, window, cx);
                            }
                        });
                }
                Semantics::new(el).disabled(disabled).into_any_element()
            })
            .collect()
    }
}
