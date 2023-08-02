### Leptos Actix Action Form bug minimal reproducible version
When reloading the page multiple times the error happens, on my computer, it errors specifically on the third time.
This is with actix, and the actions are passed down from the main `App` function to the `HomePage` function, which
causes the error, although this seems to be done the same way in [the Axum session auth example](https://github.com/leptos-rs/leptos/tree/main/examples/session_auth_axum). The way I solved the error is by using `create_server_action` directly in `HomePage` rather than in
`App`, which again is not the way it was done in the axum example.
