use rosin::prelude::*;
use rosin::widgets::*;

#[cfg_attr(
    debug_assertions,
    derive(serde::Deserialize, serde::Serialize, TypeHash, Expose),
    serde(default)
)]
#[derive(Debug)]
pub struct State {
    style: Stylesheet,
    count: Var<i32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            style: stylesheet!("src/style.css"),
            count: Var::new(0),
        }
    }
}

#[unsafe(no_mangle)]
pub fn main_view(state: &State, ui: &mut Ui<State, WindowHandle>) {
    ui.node()
        .id(id!())
        .style_sheet(&state.style)
        .classes("root")
        .children(|ui| {
            label(ui, id!(), *state.count).classes("number");
            button(ui, id!(), "Count", |s, _| {
                *s.count.write() += 1;
            });
        });
}
