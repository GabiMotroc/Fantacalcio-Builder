use std::fmt::Display;

use leptos::*;

#[component]
pub fn EnumDropdown<T: Clone + Display + 'static>(placeholder: String, enum_values: Vec<T>, setter: WriteSignal<Option<T>>) -> impl IntoView {
    view! {
        <div class="dropdown">
            <button
                class="btn btn-primary dropdown-toggle"
                type="button"
                id="dropdownMenuButton"
                data-bs-toggle="dropdown"
                aria-expanded="false"
            >
                {placeholder}
            </button>
            <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton">

                {enum_values
                    .into_iter()
                    .map(|enum_val| {
                        let val = enum_val.clone();
                        view! {
                            <li>
                                <a
                                    class="dropdown-item"
                                    href="#"
                                    on:click=move |_| {
                                        let val = val.clone();
                                        setter.set(Some(val.clone()))
                                    }
                                >

                                    {enum_val.to_string()}
                                </a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}