use sycamore::prelude::*;
#[cfg(client)]
use wasm_bindgen::prelude::*;

#[component]
pub fn toggle_button<G: Html>(cx: Scope) -> View<G> {
    let theme = create_signal(cx, {
        #[cfg(client)] { init_theme() }
        #[cfg(not(client))] { String::from("") }
    });

    view! { cx,
        div(class="theme_toggle_container") {
            button(class="theme_toggle", aria-label="Toggle theme",
                on:click=move |_| { #[cfg(client)] theme.set(toggle_theme()); }
            ) {
                img(class="arrow", src="", alt="")
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
