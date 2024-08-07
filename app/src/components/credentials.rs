use leptos::{Action, component, create_signal, event_target_value, IntoView, SignalGet, view};
use leptos::ev::SubmitEvent;

#[component]
pub fn CredentialsForm
(
    title: &'static str,
    action_label: &'static str,
    action: Action<(String, String), ()>,
) -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (error, _) = create_signal("".to_string());


    let dispatch_action =
        move || action.dispatch((email.get(), password.get()));
    
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        dispatch_action();
    };

    view! {
        <form on:submit=on_submit>
            <h2>{title}</h2>
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
                    <button class="btn btn-primary row" style="width: 100%" type="submit">
                        {action_label}
                    </button>
                </div>
                {error}
            </div>
        </form>
    }
}