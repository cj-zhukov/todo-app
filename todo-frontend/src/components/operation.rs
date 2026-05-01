use std::str::FromStr;

use leptos::prelude::*;
use web_sys::MouseEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    ListTodo,   // list all todos
    GetTodo,    // get existing todo using id
    AddTodo,    // create new todo
    UpdateTodo, // update existing todo using id
    DeleteTodo, // delete existing todo ising id
}

impl Mode {
    pub const ALL: &'static [Mode] = &[
        Mode::ListTodo,
        Mode::AddTodo,
        Mode::GetTodo,
        Mode::DeleteTodo,
        Mode::UpdateTodo,
    ];

    fn variants() -> &'static [(Mode, &'static str)] {
        &[
            (Mode::ListTodo, "list"),
            (Mode::GetTodo, "get"),
            (Mode::AddTodo, "add"),
            (Mode::UpdateTodo, "update"),
            (Mode::DeleteTodo, "delete"),
        ]
    }

    pub fn as_str(&self) -> &'static str {
        Self::variants()
            .iter()
            .find(|(m, _)| m == self)
            .map(|(_, s)| *s)
            .unwrap()
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Mode::variants()
            .iter()
            .find(|(_, name)| *name == s)
            .map(|(m, _)| *m)
            .ok_or(())
    }
}

impl Mode {
    pub fn shows_result(&self) -> bool {
        matches!(
            self,
            Mode::ListTodo | Mode::GetTodo | Mode::DeleteTodo | Mode::UpdateTodo
        )
    }

    pub fn needs_id(&self) -> bool {
        matches!(self, Mode::GetTodo | Mode::DeleteTodo | Mode::UpdateTodo)
    }

    pub fn needs_text(&self) -> bool {
        matches!(self, Mode::AddTodo | Mode::UpdateTodo)
    }

    pub fn needs_completed(&self) -> bool {
        matches!(self, Mode::UpdateTodo)
    }
}

#[component]
pub fn OperationPanel(
    mode: ReadSignal<Mode>,
    set_mode: WriteSignal<Mode>,
    send_request: impl Fn(MouseEvent) + 'static + Clone,
    is_loading: ReadSignal<bool>,
    modes: Vec<Mode>, // which one button to show
) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: space-between; align-items: center; width: 600px;">
            <select
                style="font-size: 1rem; padding: 0.5rem;"
                on:change=move |ev| {
                    let selected = event_target_value(&ev);
                    if let Ok(mode) = selected.parse::<Mode>() {
                        set_mode.set(mode);
                    }
                }
            >
                <For
                    each=move || modes.clone()
                    key=|m| *m as i32
                    children=move |m| {
                        let value = m.as_str().to_string();
                        view! {
                            <option
                                value=value.clone()
                                selected={move || mode.get() == m}
                            >
                                { value.clone() }
                            </option>
                        }
                    }
                />
            </select>

            <button
                style="font-size: 1rem; padding: 0.5rem 1rem;"
                on:click=send_request
                disabled=move || is_loading.get()
            >
                "Run"
            </button>

        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_mode() {
        assert_eq!(Mode::ListTodo.as_str(), "list");

        let (mode, set_mode) = signal(Mode::ListTodo);
        set_mode.set(Mode::ListTodo);
        assert_eq!(mode.get(), Mode::ListTodo);
    }
}
