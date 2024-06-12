use leptos::*;
use leptos::tracing::info;
use leptos_meta::*;
use leptos_router::*;

use crate::components::navbar::Navbar;
use crate::services::api::Api;

use self::pages::*;

mod components;
mod services;
mod pages;

#[component]
pub fn App() -> impl IntoView {
    let api = Api::new("http://0.0.0.0:3000");

    provide_context(api);

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
                        redirect_path="login"
                        condition=|| true
                        view=|| view! { My Squad }
                    />
                    <Route
                        path=Page::Login.path()
                        view=move || {
                            view! {
                                <Login
                                    api=api
                                    on_success=move |_| {
                                        info!("on_success");
                                        let navigate = use_navigate();
                                        navigate(Page::Home.path(), Default::default());
                                    }
                                />
                            }
                        }
                    />

                    <Route
                        path=Page::Sighup.path()
                        view=move || {
                            view! {
                                <Sighup
                                    api=api
                                    on_success=move |_| {
                                        info!("on_success");
                                        let navigate = use_navigate();
                                        navigate(Page::Home.path(), Default::default());
                                    }
                                />
                            }
                        }
                    />
                    <Route path="*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}
