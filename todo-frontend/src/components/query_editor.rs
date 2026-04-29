use leptos::prelude::*;

#[component]
pub fn AddTodo(todo: ReadSignal<String>, set_todo: WriteSignal<String>) -> impl IntoView {
    view! {
        <textarea
            style="width: 800px; height: 150px; font-size: 1rem;"
            on:input=move |ev| set_todo.set(event_target_value(&ev))
        >
            { move || todo.get() }
        </textarea>
    }
}
