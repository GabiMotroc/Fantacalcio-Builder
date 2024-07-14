use leptos::{component, IntoView};
use stylers::style_str;
use crate::components::player_dropzone::PlayerDropzone;

#[component]
pub fn Field() -> impl IntoView {
    let (class_name, style_val) = style_str! {
        .body {
            margin: 0;
            background-color: #2b2b2b;
        }

        .field {
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            width: 75vh;
            max-height: 1200px;
            height: 100%;
            background-color: #4CAF50;
            border: 5px solid white;
            border-radius: 10px;
            transform-origin: center;
        }

        .goal-area {
            display: flex;
            justify-content: center;
            height: 15%;
        }

        .goal-box {
            width: 50%;
            height: 90%;
            border: 2px solid white;
            display: flex;
            justify-content: center;
            position: relative;
        }

        .penalty-spot {
            width: 6px;
            height: 6px;
            background-color: white;
            border-radius: 50%;
        }

        .field-inner {
            flex: 1;
            position: relative;
            display: flex;
            justify-content: center;
            align-items: center;
        }

        .center-circle {
            width: 20%;
            height: 20%;
            border: 2px solid white;
            border-radius: 50%;
        }

        .half-line {
            position: absolute;
            width: 100%;
            height: 2px;
            background-color: white;
            top: 50%;
            transform: translateY(-50%);
        }

        .top-goal {
            align-items: start;
        }

        .bottom-goal {
            align-items: end;
        }

        .six-yard-box {
            width: 50%;
            height: 40%;
            border: 2px solid white;
        }

        .bottom-goal-box {
            align-items: end;
        }

        .top-goal-box {
            align-items: start;
        }
        
        .box{
            position: absolute;
            height: 100px;
            width: 100px;
            background-color: red;
        }
    };

    leptos::view! {
        class = class_name,
        <style>{style_val}</style>
            <div class="field">
                <div class="box">
                    <PlayerDropzone/>
                </div>
                <div class="top-goal goal-area">
                    <div class="goal-box top-goal-box">
                        <div class="six-yard-box"></div>
                    </div>
                </div>
                <div class="field-inner">
                    <div class="half-line"></div>
                    <div class="center-circle"></div>
                </div>
                <div class="bottom-goal goal-area">
                    <div class="goal-box bottom-goal-box">
                        <div class="six-yard-box"></div>
                    </div>
                </div>
            </div>
    }
}