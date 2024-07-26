use leptos::{component, IntoView, view};

#[component]
pub fn CheckboxInput(display_text: String) -> impl IntoView {
    view! {
        <div class="form-check">
            <input class="form-check-input" type="checkbox" value="" id="checkbox"/>
            <label class="form-check-label" for="checkbox">
                Default checkbox
            </label>
        </div>
    }
}