use leptos::{component, create_action, create_signal, For, IntoView, SignalGetUntracked, SignalUpdate, use_context, view};
use leptos::leptos_dom::logging::console_log;

use request_domain::player::{Player, Position};

use crate::components::input::enum_dropdown::EnumDropdown;
use crate::components::input::form_input::TextInput;
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
    let (selected_position, set_selected_position) = create_signal(None);


    let (search, set_search) = create_signal("".to_string());

    let displayed_players = move || {
        let all_players = players();

        let search_value = search();
        let mut result = apply_search_filter(search_value, all_players);

        let selected = selected_players();
        result = remove_selected_players(result, selected);

        let position = selected_position();
        filter_position(result, position)
    };

    view! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-md-6">
                    <For each=selected_players key=|p| p.id let:player>
                        <div>{player.name}</div>
                    </For>

                </div>
                <div class="overflow-auto col-md-6" style="height=800px">
                    <TextInput
                        id="search".to_string()
                        placeholder="Search".to_string()
                        getter=search
                        setter=set_search
                    />
                    <table class="table table-hover">
                        <thead>
                            <th scope="col">#</th>
                            <th scope="col">Name</th>
                            <th scope="col">Team</th>
                            <th scope="col">
                                <EnumDropdown
                                    placeholder="Position".to_string()
                                    enum_values=Position::all()
                                    setter=set_selected_position
                                />
                            </th>
                        </thead>
                        <tbody>
                            <For each=displayed_players key=|player| player.id let:player>

                                {{
                                    let cloned_player = player.clone();
                                    view! {
                                        <tr on:click=move |_| {
                                            let cloned_player = cloned_player.clone();
                                            console_log(&format!("clicked player {} with all players {:?}", &cloned_player.name, selected_players.get_untracked()));
                                            set_selected_players
                                                .update(move |list| {
                                                    list.push(cloned_player);
                                                });
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

fn apply_search_filter(search_value: String, initial: Vec<Player>) -> Vec<Player> {
    match search_value.is_empty() {
        true => { initial }
        false => {
            initial.into_iter()
                .filter(|p|
                p.name.to_lowercase().contains(&search_value.to_lowercase()) ||
                    p.team.to_lowercase().contains(&search_value.to_lowercase())
                )
                .collect::<Vec<_>>()
        }
    }
}

fn remove_selected_players(initial: Vec<Player>, selected: Vec<Player>) -> Vec<Player> {
    initial.into_iter()
        .filter(|p| {
            !selected.clone().into_iter().any(|s| s.id == p.id)
        })
        .collect::<Vec<_>>()
}

fn filter_position(initial: Vec<Player>, position: Option<Position>) -> Vec<Player> {
    match position {
        None => { initial }
        Some(pos) => {
            initial.into_iter()
                .filter(|p| p.position == pos)
                .collect::<Vec<_>>()
        }
    }
}