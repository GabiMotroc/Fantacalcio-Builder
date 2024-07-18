use leptos::{component, IntoView, view};
use stylers::style_str;
use crate::components::player_dropzone::PlayerDropzone;

#[component]
pub fn PlayerFormation() -> impl IntoView{
    let (class_name, style_val) = style_str! {
        .player {
            width: 100px;
            height: 100px;
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 2;
        }
    };
    view! {
        class = class_name,
        <style>{style_val}</style>
        <div class="vh-100 d-flex align-items-center justify-content-center">
            <div class="container h-100 d-flex flex-column justify-content-around">
                <div class="row justify-content-around">
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                </div>
                <div class="row justify-content-around">
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                </div>
                <div class="row justify-content-around">
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                </div>
                <div class="row justify-content-around">
                    <div class="player">
                        <PlayerDropzone/>
                    </div>
                </div>
            </div>
        </div>
    }
}