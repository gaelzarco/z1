use sycamore::prelude::*;
#[cfg(client)]
use wasm_bindgen::prelude::*;

#[component]
pub fn toggle_button<G: Html>(cx: Scope) -> View<G> {
    let theme = create_signal(cx, {
        #[cfg(client)] { init_theme() }
        #[cfg(not(client))] { String::from("dark") }
    });

    let icon  = create_memo(cx, ||
        if theme.get().as_str() == "dark" { ".perseus/static/icons/sun.svg" }
        else { ".perseus/static/icons/moon.svg" });
    let label = create_memo(cx, ||
        if theme.get().as_str() == "dark" { "Light" }
        else { "Dark" });
    let alt   = create_memo(cx, ||
        if theme.get().as_str() == "dark" { "Sun" }
        else { "Moon" });

    view! { cx,
        div(class="theme_toggle_container") {
            button(class="theme_toggle", aria-label="Toggle theme",
                on:click=move |_| { #[cfg(client)] theme.set(toggle_theme()); }
            ) {
                img(class="arrow", src=icon.get(), alt=alt.get())
                (label.get())
            }
        }
    }
}

#[cfg(client)]
#[wasm_bindgen(module = "/src/scripts/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = initTheme)]
    fn init_theme() -> String;
    #[wasm_bindgen(js_name = toggleTheme)]
    fn toggle_theme() -> String;
}
