use leptos::{component, For, IntoView, view, WriteSignal};

use request_domain::player::{Player, Position};

use crate::components::input::enum_dropdown::EnumDropdown;

#[component]
pub fn PlayerTable(players: impl Fn() -> Vec<Player> + 'static,
                   #[prop(optional)] set_selected_position: Option<WriteSignal<Option<Position>>>,
                   on_row_click: impl Fn(Player) + 'static + Copy) -> impl IntoView {
    let position_col = match set_selected_position {
        None => { view! { <th scope="col">Position</th> } }
        Some(setter) => {
            view! {
                <th scope="col">
                    <EnumDropdown
                        placeholder="Position".to_string()
                        enum_values=Position::all()
                        setter=setter
                    />
                </th>
            }
        }
    };

    view! {
        <table class="table table-hover">
            <thead>
                <th scope="col">#</th>
                <th scope="col">Name</th>
                <th scope="col">Team</th>
                {position_col}
            </thead>
            <tbody>
                <For each=players key=|player| player.id let:player>

                    {
                        let cloned_player = player.clone();
                        view! {
                            <tr on:click=move |_| {
                                let cloned_player = cloned_player.clone();
                                on_row_click(cloned_player);
                            }>
                                <th scope="row">{player.id}</th>
                                <td>{&player.name}</td>
                                <td>{&player.team}</td>
                                <td>{&player.position.to_string()}</td>
                            </tr>
                        }
                    }

                </For>
            </tbody>
        </table>
    }
}