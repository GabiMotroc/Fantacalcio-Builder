use leptos::{component, create_node_ref, create_signal, For, IntoView, view};
use leptos::html::Div;
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
use leptos_use::docs::BooleanDisplay;

#[component]
pub fn PlayerDropZone() -> impl IntoView {
    let (dropped, set_dropped) = create_signal(false);

    let box_ref = create_node_ref::<Div>();
    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(
        box_ref,
        UseDropZoneOptions::default()
            .on_drop(move |node| {
                set_dropped(true);
            })
            .on_enter(move |node| set_dropped(false))
        ,
    );
    view! {
        <div class="box" node_ref=box_ref>
            dropped:
            <BooleanDisplay value=dropped/>
              <For each=files key=|f| f.name() let:file>
                            <div class="w-200px bg-black-200/10 ma-2 pa-6">
                                <p>Name: {file.name()}</p>
                                <p>Size: {file.size()}</p>
                                <p>Type: {file.type_()}</p>
                                <p>Last modified: {file.last_modified()}</p>
                            </div>
                        </For>
        </div>
    }
}