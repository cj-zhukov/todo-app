use gloo_net::http::Request;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::components::*;
use crate::utils::constraints::*;

#[derive(Debug, Deserialize)]
struct ApiTodoResponse {
    pub message: String,
    pub content: Value,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiTodoCreateRequest {
    pub body: String,
}

#[component]
pub fn App() -> impl IntoView {
    let (is_loading, set_is_loading) = signal(false); // spinner
    let (error, set_error) = signal(None::<String>); // error msg
    let (result, set_result) = signal(None::<Value>); // todos response
    let (todo, set_todo) = signal("Todo body".to_string()); // add todo form
    let (mode, set_mode) = signal(Mode::ListTodo); // mode
    let (todo_id, set_todo_id) = signal(String::new());

    let send_request = move |_| {
        spawn_local(async move {
            let current_mode = mode.get_untracked();
            let current_todo = todo.get_untracked();
            let current_id = todo_id.get_untracked();
            let endpoint = format!("{URL}todos");
            set_is_loading.set(true);
            set_error.set(None);
            log!(
                "Sending request to endpoint: {:?} with mode: {}",
                endpoint,
                current_mode.as_ref()
            );

            let response = match current_mode {
                Mode::ListTodo => match Request::get(&endpoint).build() {
                    Ok(req) => match req.send().await {
                        Ok(res) => res,
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
                }
                Mode::AddTodo => {
                    let payload = ApiTodoCreateRequest {
                        body: current_todo,
                    };
                    match Request::post(&endpoint)
                        .header("Content-Type", "application/json")
                        .json(&payload)
                    {    
                        Ok(req) => match req.send().await {
                            Ok(res) => res,
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
                    
                    }
                }
                Mode::GetTodo => {
                    let current_id = match current_id.parse::<i64>() {
                        Ok(id) => id,
                        Err(_) => {
                            set_error.set(Some("Invalid ID".to_string()));
                            set_is_loading.set(false);
                            return;
                        }
                    };
                    let url = format!("{}/{}", endpoint, current_id);
                    match Request::get(&url).send().await {
                        Ok(res) => res,
                        Err(e) => {
                            set_result.set(None);
                            set_error.set(Some(format!("Network error: {e}")));
                            set_is_loading.set(false);
                            return;
                        }
                    }                    
                }
                Mode::DeleteTodo => {
                    let current_id = match current_id.parse::<i64>() {
                        Ok(id) => id,
                        Err(_) => {
                            set_error.set(Some("Invalid ID".to_string()));
                            set_is_loading.set(false);
                            return;
                        }
                    };
                    let url = format!("{}/{}", endpoint, current_id);
                    match Request::delete(&url).send().await {
                        Ok(res) => res,
                        Err(e) => {
                            set_result.set(None);
                            set_error.set(Some(format!("Network error: {e}")));
                            set_is_loading.set(false);
                            return;
                        }
                    }                           
                }
                _ => todo!()
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
            
            match response.json::<ApiTodoResponse>().await {
                    Ok(resp) => {
                        log!("Recieved response msg: {}, content: {}", resp.message, resp.content);
                        set_result.set(Some(resp.content))
                    }
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

            // Input generic panel
            <InputPanel
                mode=mode
                todo=todo
                set_todo=set_todo
                todo_id=todo_id
                set_todo_id=set_todo_id
            />

            // error msg
            <ErrorMessage error=error />

            // operation type
            <OperationPanel
                mode=mode
                set_mode=set_mode
                send_request=send_request
                is_loading=is_loading
                modes=vec![Mode::ListTodo, Mode::AddTodo, Mode::GetTodo, Mode::DeleteTodo]
            />

            // todos response result
            <Show when=move || mode.get() == Mode::ListTodo || mode.get() == Mode::GetTodo || mode.get() == Mode::DeleteTodo>
                <ListTodoResult result=result />
            </Show>

        </div>
    }
}

