use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let action = create_server_action::<ActionHandler>(cx);

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=move |cx| view! {
                        cx,
                        <HomePage action=action />
                    }/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(ActionHandler, "/api")]
async fn action_handler(cx: Scope, full_name: String, email: String, password: String, remember: Option<String>) -> Result<(), ServerFnError> {
    Ok(())
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope, action: Action<ActionHandler, Result<(), ServerFnError>>) -> impl IntoView {
    // Creates a reactive value to update the button
    //let (count, set_count) = create_signal(cx, 0);
    //let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <ActionForm action=action>
            <label>
                "Full Name"
                <input type="text" placeholder="Full Name" maxlength="32" name="full_name" class="auth-input" />
            </label>
            <label>
                "Email"
                <input type="text" placeholder="Email" name="email" class="auth-input" />
            </label>
            <label>
                "Password"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <label>
                <input type="checkbox" name="remember" class="auth-input" />
                "Remember me?"
            </label>
            <button type="submit" class="button">"Log In"</button>
        </ActionForm>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
