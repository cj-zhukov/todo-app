use leptos::prelude::*;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
struct TodosTable {
    id: Option<i64>,
    body: Option<String>,
    completed: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[component]
pub fn ListTodoResult(result: ReadSignal<Option<Value>>) -> impl IntoView {
    view! {
        <div style="width: 100%; text-align: center;">
            {move || {
                result
                    .get()
                    .and_then(|data| serde_json::from_value::<Vec<TodosTable>>(data).ok())
                    .map(|rows| {
                        let columns: Vec<(&str, Box<dyn Fn(&TodosTable) -> Option<String>>)> = vec![
                            ("id", Box::new(|r| r.id.map(|v| v.to_string()))),
                            ("body", Box::new(|r| r.body.clone())),
                            ("completed", Box::new(|r| r.completed.map(|v| v.to_string()))),
                            ("created_at", Box::new(|r| r.created_at.clone())),
                            ("updated_at", Box::new(|r| r.updated_at.clone())),
                        ];

                        let active_columns: Vec<_> = columns
                            .iter()
                            .filter(|(_, getter)| rows.iter().any(|r| getter(r).is_some()))
                            .collect();

                        view! {
                            <table style="width: 100%; border-collapse: collapse;">
                                <thead>
                                    <tr>
                                        {active_columns.iter().map(|(name, _)| view! {
                                            <th style="border: 1px solid black; padding: 8px;">{name.to_string()}</th>
                                        }).collect_view()}
                                    </tr>
                                </thead>
                                <tbody>
                                    {rows.iter().map(|row| {
                                        view! {
                                            <tr>
                                                {active_columns.iter().map(|(_, getter)| {
                                                    let value = getter(row).unwrap_or_default();
                                                    view! {
                                                        <td style="border: 1px solid black; padding: 8px;">{value}</td>
                                                    }
                                                }).collect_view()}
                                            </tr>
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>
                        }
                    })
            }}
        </div>
    }
}
