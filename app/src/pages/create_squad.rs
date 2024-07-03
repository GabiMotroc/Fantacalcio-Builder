use leptos::{component, create_signal, For, IntoView, SignalGet, view};
use crate::components::player::Player;

#[component]
pub fn choose_squad() -> impl IntoView{
    let (players_grid, _) =create_signal::<Vec<Vec<i32>>>(vec![
        vec![0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
    ]);

    view! {
        <div>
            <For each=move || players_grid.get()
        key=|n| *n>
                <Player/>
            </For>
        // <img src="field.png" height="100%"/>
        </div>
    }
}