use leptos::*;
use leptos::ev::DragEvent;
use leptos::leptos_dom::logging::console_log;
use request_domain::player::Player;

#[component]
pub fn DraggablePlayer(player: Player) -> impl IntoView {
    let player_copy = player.clone();
    
    let handle_drag_start = move |event: DragEvent| {
        console_log(&format!("Drag start event: {:?}", event));
        let data_transfer = event.data_transfer().unwrap();
        let data = serde_json::to_string(&player.clone()).unwrap();
        data_transfer.set_data("application/json", &data).unwrap();
    };

    view! {
        <div class="draggable" draggable="true" on:dragstart=handle_drag_start>
            {player_copy.name.clone()}
        </div>
    }
}