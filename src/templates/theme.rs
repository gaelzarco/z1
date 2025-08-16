use sycamore::prelude::*;
#[cfg(client)]
use wasm_bindgen::prelude::*;

#[cfg(client)]
#[wasm_bindgen(module = "/src/scripts/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = initTheme)]
    fn init_theme() -> String;
    #[wasm_bindgen(js_name = toggleTheme)]
    fn toggle_theme() -> String;
}

#[component]
pub fn toggle_button<G: Html>(cx: Scope) -> View<G> {
    let theme = create_signal(cx, String::from("light"));
    #[cfg(client)]
    on_mount(cx, || theme.set(init_theme()));

    view! { cx,
        button(
            class="theme_toggle",
            aria-label="Toggle theme",
            on:click=move |_| {
                #[cfg(client)]
                theme.set(toggle_theme());
            }
        ) {
            (if theme.get().as_str() == "dark" { "☀️ Light" } else { "🌙 Dark" })
        }
    }
}
