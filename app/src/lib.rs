use leptos::leptos_dom::logging::console_log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use request_domain::login::Token;

use crate::components::navbar::Navbar;
use crate::pages::create_squad::ChooseSquad;
use crate::pages::hello_world::HelloWorld;
use crate::pages::test::Test;
use crate::services::api::Api;
use crate::services::auth::AuthSession;

use self::pages::*;


mod components;
mod pages;
mod services;

#[component]
pub fn App() -> impl IntoView {
    let api = Api::new("http://localhost:3000/api");
    provide_context(api);

    let (token, _, _) = use_local_storage::<Token, JsonCodec>("token");
    provide_context(token);

    let is_logged = Signal::derive(move || match AuthSession::validate_token(&token().token) {
        Ok(_) => true,
        Err(err) => {
            console_log(&format!("{}", err));
            false
        }
    });

    provide_context(is_logged);

    create_effect(move |_| console_log(&is_logged().to_string()));

    view! {
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"/>
        <Script
            src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.min.js"
            integrity="sha384-BBtl+eGJRgqQAUMxJ7pMwbEyER4l1g+O15P+16Ep7Q9Q+zqX6gSbd85u4mG4QzX+"
            crossorigin="anonymous"
        />

        <Router>
            <Navbar/>
            <main align="center">
                <Routes>
                    <Route path=Page::Home.path() view=Home/>
                    <ProtectedRoute
                        path="my-squad"
                        redirect_path="/login"
                        condition=move || { is_logged.get() }
                        view=|| view! { My Squad }
                    />
                    <Route
                        path=Page::Login.path()
                        view=move || {
                            view! { <Login on_success=move |_| {}/> }
                        }
                    />

                    <Route
                        path=Page::Register.path()
                        view=move || {
                            view! {
                                <Sighup on_success=move |_| {
                                    let navigate = use_navigate();
                                    navigate(Page::Home.path(), Default::default());
                                }/>
                            }
                        }
                    />

                    <Route
                        path=Page::Test.path()
                        view=move || {
                            view! { <Test/> }
                        }
                    />

                    <Route
                        path=Page::HelloWorld.path()
                        view=move || {
                            view! { <HelloWorld/> }
                        }
                    />

                    <Route
                        path=Page::CreateSquad.path()
                        view=move || {
                            view! { <ChooseSquad/> }
                        }
                    />

                    <Route path="*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}
