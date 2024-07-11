use leptos::{component, create_signal, For, IntoView, view};

use crate::components::player::Player;

#[component]
pub fn select_squad() -> impl IntoView {
    let (goalkeppers, _) = create_signal(vec![0, 1, 2]);
    let (backs, _) = create_signal(vec![0, 0, 0, 0, 0, 0, 0, 0]);
    let mids = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let initial_forwards = (1..6).map(|id| (id, create_signal(id))).collect::<Vec<_>>();
    let (forwards, _) = create_signal(initial_forwards);

    let (players_grid, _) = create_signal(
        vec![
            create_signal(vec![0, 1, 2, 3, 4, 5]),
            create_signal(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            create_signal(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            create_signal(vec![0, 1, 2]),
        ]
    );

    view! {
        <div class="container-fluid">
            <For
                each=players_grid
                key=|n| 1
                children=move |row| {
                    view! {
                        <div class="row">
                            <For
                                each=row.0
                                key=|m| *m
                                children=move |player| {
                                    view! {
                                        <div class="col align-self-center">
                                            <Player col=player row=1/>
                                        </div>
                                    }
                                }
                            />

                        </div>
                    }
                }
            />

        // {mids.into_iter().map(|n| view! { <Player/> }).collect_view()}

        // <img src="field.png" height="100%"/>
        </div>
    }
}