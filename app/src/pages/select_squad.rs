use leptos::{component, create_resource, IntoView, SignalGet, use_context, view};

use crate::components::input::checkbox::CheckboxInput;
use crate::services::api::Api;

#[component]
pub fn SelectSquad() -> impl IntoView {
    let api = use_context::<Api>().expect("required api service");

    let players = create_resource(|| (), move |_| async move {
        api.get_players().await
    });


    let players_view = move || match players.get() {
        None => {
            view! { <div>Loading...</div> }
        }
        Some(data) => {
            view! {
                <div class="overflow-auto col-md-6" style="height=800px">
                    <div class="form-outline mb-4">
                        <input type="text" class="form-control" id="datatable-search-input"/>
                        <label class="form-label" for="datatable-search-input">
                            Search
                        </label>
                    </div>
                    <table class="table table-hover">
                        <thead>
                            <th scope="col">#</th>
                            <th scope="col">Name</th>
                            <th scope="col">Team</th>
                            <th scope="col">Position</th>
                        </thead>
                        <tbody>
                            {data
                                .iter()
                                .map(|x| {
                                    view! {
                                        <tr>
                                            <th scope="row">{x.id}</th>
                                            <td>{&x.name}</td>
                                            <td>{&x.team}</td>
                                            <td>{&x.position.to_string()}</td>
                                        </tr>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </tbody>
                    </table>
                </div>
            }
        }
    };

    view! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-md-6">
                    <CheckboxInput display_text="Checkbox".to_string()/>
                </div>
                {players_view}
            </div>
        </div>
    }
}
