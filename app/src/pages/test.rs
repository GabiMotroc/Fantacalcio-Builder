use leptos::{component, create_signal, IntoView, view};
use leptos::ev::DragEvent;
use leptos_use::docs::BooleanDisplay;

#[component]
pub fn test() -> impl IntoView {
    let (dragged, set_dragged) = create_signal(false);
    // let drop_handle = use_d`
    view! {
        <div>
            <div draggable="true" on:drop=move |ev| {drop_handle(ev)}>Barella</div>
            <div draggable="true">Frattesi</div>
            <div on:dragover=move |x| {set_dragged(true)}>selected player: </div>
            <BooleanDisplay value=dragged/>
        </div>
    }
}

fn drop_handle(drag_event: DragEvent) {
    let a = drag_event.data_transfer().unwrap();
}
