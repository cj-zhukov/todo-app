use leptos::prelude::*;

use crate::components::Mode;

#[component]
pub fn InputPanel(
    mode: ReadSignal<Mode>,
    todo: ReadSignal<String>,
    set_todo: WriteSignal<String>,
    todo_id: ReadSignal<String>,
    set_todo_id: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <>
            // Add todo input
            <Show when=move || mode.get() == Mode::AddTodo>
                <textarea
                    style="width: 800px; height: 150px; font-size: 1rem;"
                    on:input=move |ev| set_todo.set(event_target_value(&ev))
                >
                    { move || todo.get() }
                </textarea>
            </Show>

            // Get todo by ID input
            <Show when=move || mode.get() == Mode::GetTodo>
                <input
                    type="text"
                    placeholder="Enter todo ID"
                    on:input=move |ev| set_todo_id.set(event_target_value(&ev))
                    prop:value=move || todo_id.get()
                />
            </Show>
        </>
    }
}
