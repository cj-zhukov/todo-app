use leptos::prelude::*;

use crate::components::Mode;

#[component]
pub fn InputPanel(
    mode: ReadSignal<Mode>,
    todo: ReadSignal<String>,
    set_todo: WriteSignal<String>,
    todo_id: ReadSignal<String>,
    set_todo_id: WriteSignal<String>,
    completed: ReadSignal<bool>,
    set_completed: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <>
            // ID input
            <Show when=move || mode.get().needs_id()>
                <input
                    type="text"
                    placeholder="Todo ID"
                    on:input=move |ev| set_todo_id.set(event_target_value(&ev))
                    prop:value=move || todo_id.get()
                />
            </Show>

            // TEXT input
            <Show when=move || mode.get().needs_text()>
                <textarea
                    placeholder="Todo body"
                    on:input=move |ev| {
                        set_todo.set(event_target_value(&ev));
                    }

                >
                    { move || todo.get() }
                </textarea>
            </Show>

            // COMPLETED checkbox (only for update)
            <Show when=move || mode.get().needs_completed()>
                <label>
                    <input
                        type="checkbox"
                        on:change=move |ev| {
                            set_completed.set(event_target_checked(&ev));
                        }
                        prop:checked=move || completed.get()
                    />
                    "Completed"
                </label>
            </Show>
        </>
    }
}
