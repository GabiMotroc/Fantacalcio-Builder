use leptos::{component, create_action, expect_context, view, Callback, IntoView};

use request_domain::login::LoginRequest;

use crate::components::credentials::CredentialsForm;
use crate::services::api::Api;
use crate::services::auth::AuthSession;

#[component]
pub fn Login(#[prop(into)] on_success: Callback<i32>) -> impl IntoView {
    let api = expect_context::<Api>();

    let login_action = create_action(move |(email, password): &(String, String)| {
        let email = email.to_string();
        let password = password.to_string();
        async move {
            let result = api.login(LoginRequest { email, password }).await;

            if let Ok(res) = result {
                AuthSession::save_token(&res.token);
                on_success(0);
            }
        }
    });

    view! { <CredentialsForm title="Login" action_label="Login" action=login_action/> }
}
