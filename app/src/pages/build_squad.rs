use crate::components::draggable_player::DraggablePlayer;
use leptos::{component, view, IntoView};
use request_domain::player::{Player, Position};

use crate::components::field::Field;

#[component]
pub fn build_squad() -> impl IntoView {

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
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-md-6 d-flex flex-column justify-content-center align-items-center bg-light">
                    <ul class="list-unstyled">
                        <li>
                            <DraggablePlayer player=barella/>
                        </li>
                        <li>
                            <DraggablePlayer player=lookman/>
                        </li>
                    </ul>
                </div>
                <div class="col-md-6 d-flex justify-content-center align-items-center">
                    <Field/>
                </div>
            </div>
        </div>
    }
}
