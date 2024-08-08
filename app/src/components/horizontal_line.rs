use leptos::{component, view, IntoView};
use stylers::style_str;

#[component]
pub fn HorizontalLine(placeholder: String) -> impl IntoView {
    let (class_name, style_val) = style_str! {
        .line-with-text {
            display: flex;
            align-items: center;
            text-align: center;
        }

        .line-with-text hr {
            flex-grow: 1;
            border: none;
            border-top: 1px solid #000;
            margin: 0 10px;
        }

        .line-with-text span {
            padding: 0 10px;
            white-space: nowrap;
        }
    };
    view! { class=class_name,
        <style>{style_val}</style>
        <div class="line-with-text">
            <hr/>
            <span>{placeholder}</span>
            <hr/>
        </div>
    }
}