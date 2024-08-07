use leptos::{component, event_target_value, IntoView, ReadSignal, view, WriteSignal};

#[component]
pub fn TextInput(id: String, placeholder: String, getter: ReadSignal<String>, setter: WriteSignal<String>) -> impl IntoView {
    view! {
        <div class="form-outline mb-4">
            <label for=id.clone() class="form-label">
                {placeholder.clone()}
            </label>
            <input
                class="form-control"
                id=id
                type="text"
                placeholder=placeholder
                on:input=move |x| {
                    setter(event_target_value(&x));
                }

                prop:value=getter
            />
        </div>
    }
}
