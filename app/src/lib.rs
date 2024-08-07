use leptos::*;
use leptos::leptos_dom::logging::console_log;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;

use request_domain::login::Token;

use crate::components::navbar::Navbar;
use crate::pages::build_squad::BuildSquad;
use crate::pages::hello_world::HelloWorld;
use crate::pages::select_squad::SelectSquad;
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

    view! {
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"/>
        <Script
            src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"
            integrity="sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz"
            crossorigin="anonymous"
        />

        <Router>
            <Navbar/>
            <main align="center">
                <Routes>
                    <Route path=Page::Home.path() view=Home/>
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

                    <ProtectedRoute
                        path=Page::HelloWorld.path()
                        redirect_path="/login"
                        condition=move || { is_logged.get() }
                        view=move || {
                            view! { <HelloWorld/> }
                        }
                    />

                    <ProtectedRoute
                        path=Page::BuildSquad.path()
                        redirect_path="/login"
                        condition=move || { is_logged.get() }
                        view=move || {
                            view! { <BuildSquad/> }
                        }
                    />

                    <ProtectedRoute
                        path=Page::SelectSquad.path()
                        redirect_path="/login"
                        condition=move || { is_logged.get() }
                        view=move || {
                            view! { <SelectSquad/> }
                        }
                    />

                    <ProtectedRoute
                        path="my-squad"
                        redirect_path="/login"
                        condition=move || { is_logged.get() }
                        view=|| view! { My Squad }
                    />
                    <Route path="*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}
