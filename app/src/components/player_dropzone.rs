use leptos::ev::DragEvent;
use leptos::{component, create_signal, view, IntoView};
use leptos::leptos_dom::logging::console_log;
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
        <div on:dragover=handle_drag_over on:drop=handle_drop class="test">
            <img src="player_icon.png" style="width: 100%; height: 100%"/>
            {move || match data() {
                Some(data) => view! { <div>{data.name}</div> },
                None => view! { <div></div> },
            }}

        </div>
    }
}
