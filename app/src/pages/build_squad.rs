use crate::components::draggable_player::DraggablePlayer;
use leptos::{component, create_action, create_signal, use_context, view, For, IntoView, SignalGet};
use request_domain::player::{Player, Position};

use crate::components::field::Field;
use crate::components::horizontal_line::HorizontalLine;
use crate::services::api::Api;

#[component]
pub fn build_squad() -> impl IntoView {
    let api = use_context::<Api>().expect("required api service");

    let (players, set_players) = create_signal(Vec::<Player>::new());
    let a = create_action(move |_: &()| {
        async move {
            set_players(api.get_selected_players().await);
        }
    });
    a.dispatch(());

    let goalkeepers = move || players.get()
        .into_iter()
        .filter(|p| p.position == Position::Goalkeeper)
        .collect::<Vec<_>>();

    let defenders = move || players.get()
        .into_iter()
        .filter(|p| p.position == Position::Defender)
        .collect::<Vec<_>>();

    let midfielders = move || players.get()
        .into_iter()
        .filter(|p| p.position == Position::Midfielder)
        .collect::<Vec<_>>();

    let forwards = move || players.get()
        .into_iter()
        .filter(|p| p.position == Position::Forward)
        .collect::<Vec<_>>();

    view! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-md-6 flex-column justify-content-center align-items-center bg-light">
                    <div class="row">
                        <HorizontalLine placeholder="Goalkeeper".to_string()/>
                        <For each=goalkeepers key=|player| player.id let:player>
                                <DraggablePlayer player=player/>
                        </For>
                    </div>
                    <div class="row">
                        <HorizontalLine placeholder="Defender".to_string()/>
                        <For each=defenders key=|player| player.id let:player>
                                <DraggablePlayer player=player/>
                        </For>
                    </div>
                    <div class="row">
                        <HorizontalLine placeholder="Midfielder".to_string()/>
                        <For each=midfielders key=|player| player.id let:player>
                                <DraggablePlayer player=player/>
                        </For>
                    </div>
                    <div class="row">
                        <HorizontalLine placeholder="Forward".to_string()/>
                        <For each=forwards key=|player| player.id let:player>
                                <DraggablePlayer player=player/>
                        </For>
                    </div>
                </div>
                <div class="col-md-6 d-flex justify-content-center align-items-center">
                    <Field/>
                </div>
            </div>
        </div>
    }
}
