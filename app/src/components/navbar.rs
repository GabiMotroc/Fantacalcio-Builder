use leptos::{component, view, IntoView};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar navbar-collapse bg-body-tertiary">
            <div class="container-fluid">
                <a class="navbar-brand" href="/">
                    Fantacalcio Builder
                </a>
                <div class="collapse navbar-collapse" id="navBar">
                    <ul class="navbar-nav">
                        <li class="nav-item">
                            <a class="nav-link" href="/test">
                                Test
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
