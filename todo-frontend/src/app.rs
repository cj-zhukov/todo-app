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

#[derive(Debug, Serialize)]
struct ApiTodoCreateRequest {
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct ApiTodoUpdateRequest {
    pub body: String,
    pub completed: bool,
}

#[component]
pub fn App() -> impl IntoView {
    let (is_loading, set_is_loading) = signal(false); // spinner
    let (error, set_error) = signal(None::<String>); // error msg
    let (result, set_result) = signal(None::<Value>); // todos response
    let (todo, set_todo) = signal("Todo body".to_string()); // create or update todo 
    let (mode, set_mode) = signal(Mode::ListTodo); // mode
    let (todo_id, set_todo_id) = signal(String::new()); // todo id
    let (todo_completed, set_todo_completed) = signal(false); // todo completed

    let send_request = move |_| {
        spawn_local(async move {
            let current_mode = mode.get_untracked();
            let current_todo = todo.get_untracked();
            let current_id = todo_id.get_untracked();
            let current_completed = todo_completed.get_untracked();
            let endpoint = format!("{URL}todos");
            set_is_loading.set(true);
            set_error.set(None);
            log!(
                "Sending request to endpoint: {:?} with mode: {}",
                endpoint,
                current_mode.as_ref()
            );
            
            let request = match build_request(
                current_mode,
                &endpoint,
                current_todo,
                current_id,
                current_completed,
            ) {
                Ok(req) => req,
                Err(e) => {
                    set_error.set(Some(e));
                    set_is_loading.set(false);
                    return;
                }
            };

            let response = match send(request).await {
                Ok(res) => res,
                Err(e) => {
                    set_error.set(Some(e));
                    set_result.set(None);
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
                completed=todo_completed
                set_completed=set_todo_completed
            />

            // error msg
            <ErrorMessage error=error />

            // operation type
            <OperationPanel
                mode=mode
                set_mode=set_mode
                send_request=send_request
                is_loading=is_loading
                modes=vec![Mode::ListTodo, Mode::AddTodo, Mode::GetTodo, Mode::DeleteTodo, Mode::UpdateTodo]
            />

            // todos response result
            <Show when=move || mode.get() == Mode::ListTodo || mode.get() == Mode::GetTodo || mode.get() == Mode::DeleteTodo || mode.get() == Mode::UpdateTodo>
                <TodoResult result=result />
            </Show>

        </div>
    }
}

fn build_request(
    mode: Mode,
    endpoint: &str,
    todo: String,
    id: String,
    completed: bool,
) -> Result<Request, String> {
    match mode {
        Mode::ListTodo => {
            Request::get(endpoint)
                .build()
                .map_err(|e| e.to_string())
        }

        Mode::AddTodo => {
            let payload = ApiTodoCreateRequest { body: todo };

            Request::post(endpoint)
                .header("Content-Type", "application/json")
                .json(&payload)
                .map_err(|e| e.to_string())
        }

        Mode::GetTodo => {
            let id = id.parse::<i64>().map_err(|_| "Invalid ID")?;
            let url = format!("{}/{}", endpoint, id);

            Request::get(&url)
                .build()
                .map_err(|e| e.to_string())
        }

        Mode::DeleteTodo => {
            let id = id.parse::<i64>().map_err(|_| "Invalid ID")?;
            let url = format!("{}/{}", endpoint, id);

            Request::delete(&url)
                .build()
                .map_err(|e| e.to_string())
        }
        
        Mode::UpdateTodo => {
            let id = id.parse::<i64>().map_err(|_| "Invalid ID")?;
            let url = format!("{}/{}", endpoint, id);
            let payload = ApiTodoUpdateRequest {
                body: todo,
                completed,
            };

            Request::put(&url)
                .header("Content-Type", "application/json")
                .json(&payload)
                .map_err(|e| e.to_string())
        }
    }
}

async fn send(req: Request) -> Result<gloo_net::http::Response, String> {
    req.send()
        .await
        .map_err(|e| format!("Network error: {e}"))
}
