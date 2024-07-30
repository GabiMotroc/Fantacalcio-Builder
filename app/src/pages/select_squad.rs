use leptos::{component, create_action, create_signal, For, IntoView, SignalUpdate, use_context, view};
use leptos::leptos_dom::logging::console_log;

use request_domain::player::Player;

use crate::services::api::Api;

#[component]
pub fn SelectSquad() -> impl IntoView {
    let api = use_context::<Api>().expect("required api service");

    let (players, set_players) = create_signal(Vec::<Player>::new());
    let a = create_action(move |_: &()| {
        async move {
            console_log("got players");
            set_players(api.get_players().await);
        }
    });
    a.dispatch(());


    let (selected_players, set_selected_players) = create_signal(Vec::<Player>::new());

    view! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-md-6">
                    <For each=selected_players key=|p| p.id let:player>
                        <div>{player.name}</div>
                    </For>

                </div>
                <div class="overflow-auto col-md-6" style="height=800px">
                    <div class="form-outline mb-4">
                        <label class="form-label" for="datatable-search-input">
                            Search
                        </label>
                        <input type="text" class="form-control" id="datatable-search-input"/>
                    </div>
                    <table class="table table-hover">
                        <thead>
                            <th scope="col">#</th>
                            <th scope="col">Name</th>
                            <th scope="col">Team</th>
                            <th scope="col">Position</th>
                        </thead>
                        <tbody>
                            <For each=players key=|player| player.id let:player>

                                {{
                                    let cloned_player = player.clone();
                                    view! {
                                        <tr on:click=move |_| {
                                            let cloned_player = cloned_player.clone();
                                            set_selected_players
                                                .update(move |list| {
                                                    list.push(cloned_player);
                                                });
                                            console_log("clicked player");
                                        }>
                                            <th scope="row">{player.id}</th>
                                            <td>{&player.name}</td>
                                            <td>{&player.team}</td>
                                            <td>{&player.position.to_string()}</td>
                                        </tr>
                                    }
                                }}

                            </For>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
