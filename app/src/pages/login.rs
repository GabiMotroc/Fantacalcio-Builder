use leptos::{Callable, Callback, component, create_action, create_signal, IntoView, SignalGet, use_context, view};

use request_domain::login::LoginRequest;

use crate::components::credentials::CredentialsForm;
use crate::services::api::Api;

#[component]
pub fn Login(
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
        <CredentialsForm
            title = "Login"
            action_label = "Login"
            action = register_action
        ></CredentialsForm>
    }
}
