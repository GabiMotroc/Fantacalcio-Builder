use leptos::{Callable, Callback, component, create_action, create_signal, event_target_value, IntoView, SignalGet, use_context, view};

use request_domain::login::LoginRequest;

use crate::services::api::Api;

#[component]
pub fn Sighup(
    #[prop(into)] on_success: Callback<i32>,
) -> impl IntoView {
    let api = use_context::<Api>().unwrap();

    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (error, set_error) = create_signal("".to_string());

    let register_action = create_action(move |(email, password): &(String, String)| {
        let email = email.to_string();
        let password = password.to_string();
        async move {
            let result = api.sighup(LoginRequest {
                email: email,
                password: password,
            }).await;

            match result {
                Ok(res) => {
                    on_success.call(0);
                }
                Err(err) => {
                    set_error("failed".to_string());
                }
            }
        }
    });

    let dispatch_action =
        move || register_action.dispatch((email.get(), password.get()));

    view! {
        <div>
            <div class="m-5">
                <label for="email_input" class="form-label">
                    Username:
                </label>
                <input
                    class="form-control"
                    id="email_input"
                    type="email"
                    placeholder="name@email.com"
                    on:input=move |x| {
                        set_email(event_target_value(&x));
                    }

                    prop:value=email
                />
                <label for="password" class="form-label pt-2">
                    Password:
                </label>
                <input
                    class="form-control"
                    id="password_input"
                    type="password"
                    placeholder="****"
                    on:input=move |x| {
                        set_password(event_target_value(&x));
                    }
                />

                <div class=" d-flex align-items-center justify-content-center pt-4">
                    <button
                        class="btn btn-primary row"
                        style="width: 100%"
                        on:click=move |_| dispatch_action()
                    >
                        Login
                    </button>
                </div>
                {error}
            </div>
        </div>
    }
}
