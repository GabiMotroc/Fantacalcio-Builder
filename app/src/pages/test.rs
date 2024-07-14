use leptos::{component, IntoView, view};
use leptos::ev::DragEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use request_domain::player::{Player, Position};
use crate::components::draggable_player::DraggablePlayer;
use crate::components::player_dropzone::{PlayerDropzone};

#[component]
pub fn test() -> impl IntoView {
    let barella = Player{
        id: 21,
        fantacalcio_id: 1298,
        position: Position::Midfielder,
        name: "Barella".to_string(),
        team: "Inter".to_string(),
        is_active: true,
    };
    let lookman = Player{
        id: 21,
        fantacalcio_id: 1298,
        position: Position::Forward,
        name: "Lookman".to_string(),
        team: "Atalanta".to_string(),
        is_active: true,
    };

    view! {
        <div class="container">
            <DraggablePlayer player=barella></DraggablePlayer>
            <DraggablePlayer player=lookman></DraggablePlayer>
            <PlayerDropzone/>
        </div>
    }
}
fn handle_drag_start(event: DragEvent) {
    console_log(&format!("Drag start event: {:?}", event));
    if let Some(data_transfer) = event.data_transfer() {
        if let Some(target) = event.target()
        {
            let html_el = target.dyn_into::<HtmlElement>().unwrap();
            data_transfer.set_data("text/plain", &html_el.inner_text()).unwrap();
        }
    }
}

fn handle_drag_over(event: DragEvent) {
    console_log(&format!("Drag over event: {:?}", event));
    event.prevent_default();
}

fn handle_drop(event: DragEvent) {
    console_log(&format!("Drop event: {:?}", event));
    event.prevent_default();
    if let Some(data_transfer) = event.data_transfer() {
        if let Ok(text) = data_transfer.get_data("text/plain") {
            if let Some(target) = event.target() {
                let dropbox = target.dyn_into::<HtmlElement>().unwrap();
                dropbox.set_inner_text(&text);
            }
        }
    }
}
