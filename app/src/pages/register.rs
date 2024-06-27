use leptos::{component, create_action, use_context, view, Callback, IntoView};

use request_domain::login::LoginRequest;

use crate::components::credentials::CredentialsForm;
use crate::services::api::Api;
use crate::services::auth::AuthSession;

#[component]
pub fn Sighup(#[prop(into)] on_success: Callback<i32>) -> impl IntoView {
    let api = use_context::<Api>().unwrap();

    let register_action = create_action(move |(email, password): &(String, String)| {
        let email = email.to_string();
        let password = password.to_string();
        async move {
            let result = api.register(LoginRequest { email, password }).await;

            if let Ok(res) = result {
                AuthSession::save_token(&res.token);
                on_success(0);
            }
        }
    });

    view! {
        <CredentialsForm
            title="Register"
            action_label="Register as a new user"
            action=register_action
        />
    }
}
