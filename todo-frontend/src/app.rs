use gloo_net::http::Request;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use serde::Deserialize;
use serde_json::Value;

use crate::components::*;
use crate::utils::constraints::*;

#[derive(Debug, Deserialize)]
struct ApiTodoListResponse {
    pub message: String,
    pub content: Value,
}

#[component]
pub fn App() -> impl IntoView {
    let (is_loading, set_is_loading) = signal(false); // spinner
    let (error, set_error) = signal(None::<String>); // error msg
    let (result, set_result) = signal(None::<Value>); // list todos
    let (todo, set_todo) = signal("Todo body".to_string()); // add todo form
    let (mode, set_mode) = signal(Mode::ListTodo); // mode

    let send_request = move |_| {
        spawn_local(async move {
            let current_mode = mode.get_untracked();
            let endpoint = format!("{URL}todos");
            set_is_loading.set(true);
            set_error.set(None);
            log!(
                "Sending request to endpoint: {:?} with mode: {}",
                endpoint,
                current_mode.as_ref()
            );

            let response = match Request::get(&endpoint).build()
            {
                Ok(req) => match req.send().await {
                    Ok(req) => req,
                    Err(e) => {
                        set_result.set(None);
                        set_error.set(Some(format!("Network error: {e}")));
                        set_is_loading.set(false);
                        return;
                    }
                },
                Err(e) => {
                    set_result.set(None);
                    set_error.set(Some(format!("Failed to build request: {e}")));
                    set_is_loading.set(false);
                    return;
                }
            };

            if !response.ok() {
                let msg = match response.status() {
                    400 => "Invalid user input".to_string(),
                    404 => "No data found".to_string(),
                    500 => "Internal server error".to_string(),
                    _ => format!("Error {} occurred", response.status()),
                };
                set_result.set(None);
                set_error.set(Some(msg));
                set_is_loading.set(false);
                return;
            }
            
            log!("Recieved response: {:?}", response);

            match response.json::<ApiTodoListResponse>().await {
                    Ok(resp) => set_result.set(Some(resp.content)),
                    Err(e) => {
                        set_result.set(None);
                        set_error.set(Some(format!("Failed to parse response: {e}")));
                    }
                }
                set_is_loading.set(false);
            });
    };

    view! {
        // spinner
        <Spinner visible=is_loading />

        <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem;">
            <h1>"Todo App"</h1>

            // add todo form
            <AddTodo todo=todo set_todo=set_todo />

            // error msg
            <ErrorMessage error=error />

            // operation type
            <OperationPanel
                mode=mode
                set_mode=set_mode
                send_request=send_request
                is_loading=is_loading
                modes=vec![Mode::ListTodo]
            />

            // List todo result
            <Show when=move || mode.get() == Mode::ListTodo>
                <ListTodoResult result=result />
            </Show>

        </div>
    }
}

pub fn it_works() -> impl IntoView {
    view! { <p>"Here’s some text"</p> }
}
