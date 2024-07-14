use leptos::ev::DragEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::{component, create_signal, view, IntoView};
use serde_json::from_str;

use request_domain::player::Player;

#[component]
pub fn PlayerDropzone() -> impl IntoView {
    let (data, set_data) = create_signal(None::<Player>);

    let handle_drag_over = move |event: DragEvent| {
        console_log(&format!("Drag over event: {:?}", event));
        event.prevent_default();
    };

    let handle_drop = move |event: DragEvent| {
        console_log(&format!("Drop event: {:?}", event));
        event.prevent_default();
        let data_transfer = event.data_transfer().unwrap();
        if let Ok(json_data) = data_transfer.get_data("application/json") {
            if let Ok(data) = from_str::<Player>(&json_data) {
                set_data(Some(data));
            }
        }
    };

    view! {
        <div
            class="dropbox"
            on:dragover=handle_drag_over
            on:drop=handle_drop
        >
            {move ||
                match data() {
                    Some(data) => view! { <p>{format!("Dropped: {:?}", data.name)}</p> },
                    None => view! {  <p>{"Drop here"}</p> },
                }
            }
        </div>
    }
}
