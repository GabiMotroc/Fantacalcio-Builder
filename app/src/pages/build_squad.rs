use leptos::{component, IntoView, view};

use crate::components::field::Field;

#[component]
pub fn build_squad() -> impl IntoView {
    view! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-md-6 d-flex flex-column justify-content-center align-items-center bg-light">
                    <ul class="list-unstyled">
                        <li>Player 1</li>
                        <li>Player 2</li>
                        <li>Player 3</li>
                        <li>Player 4</li>
                        <li>Player 5</li>
                        <li>Player 6</li>
                        <li>Player 7</li>
                        <li>Player 8</li>
                        <li>Player 9</li>
                        <li>Player 10</li>
                    </ul>
                </div>
                <div class="col-md-6 d-flex justify-content-center align-items-center">
                    <Field/>
                </div>
            </div>
        </div>
    }
}