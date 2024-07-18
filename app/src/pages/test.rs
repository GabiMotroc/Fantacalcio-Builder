use leptos::{component, IntoView, view};

use request_domain::player::{Player, Position};

use crate::components::draggable_player::DraggablePlayer;
use crate::components::player_dropzone::PlayerDropzone;

#[component]
pub fn test() -> impl IntoView {
    let barella = Player {
        id: 21,
        fantacalcio_id: 1298,
        position: Position::Midfielder,
        name: "Barella".to_string(),
        team: "Inter".to_string(),
        is_active: true,
    };
    let lookman = Player {
        id: 21,
        fantacalcio_id: 1298,
        position: Position::Forward,
        name: "Lookman".to_string(),
        team: "Atalanta".to_string(),
        is_active: true,
    };

    view! {
        <div class="container">
            <DraggablePlayer player=barella/>
            <DraggablePlayer player=lookman/>
            <PlayerDropzone/>
        </div>
    }
}