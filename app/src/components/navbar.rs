use leptos::{component, expect_context, IntoView, Show, Signal, SignalGet, view};

#[component]
pub fn Navbar() -> impl IntoView {
    let is_logged_in = expect_context::<Signal<bool>>();

    view! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container-fluid">
                <a class="navbar-brand" href="/">
                    Fantacalcio Builder
                </a>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarNav"
                    aria-controls="navbarNav"
                    aria-expanded="false"
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navBar">
                    <ul class="navbar-nav">
                        <Show
                            when=move || { !is_logged_in.get() }
                            fallback=|| {
                                view! {
                                    <li class="nav-item">
                                        <a class="nav-link" href="/build-squad">
                                            Build Squad
                                        </a>
                                    </li>
                                }
                            }
                        >

                            <li class="nav-item">
                                <a class="nav-link" href="/login">
                                    Login
                                </a>
                            </li>
                            <li class="nav-item">
                                <a class="nav-link" href="/register">
                                    Register
                                </a>
                            </li>
                        </Show>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
