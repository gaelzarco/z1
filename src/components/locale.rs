use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn select_dropdown<G: Html>(cx: Scope) -> View<G> {
    let value = create_signal(cx, Reactor::<G>::from_cx(cx).get_translator().get_locale());

    view! { cx,
        div(class="locale_wrapper") {
            select(
                class="locale_dropdown",
                aria-label="Toggle locale",
                bind:value=value,
                on:change=move |_| {
                    #[cfg(client)]
                    Reactor::<G>::from_cx(cx).switch_locale(value.get().as_str());
                }
            ) {
                option(
                    aria-label="Toggle es-ES locale",
                    value="es-ES",
                    selected=if value.get().as_str() == "es-ES" { true } else { false },
                    on:click=move |_| {
                        #[cfg(client)]
                        {
                            value.set("es-ES".to_string()); 
                        }
                    }
                ) { "ES" }
                option(
                    aria-label="Toggle en-US locale",
                    value="en-US",
                    selected=if value.get().as_str() == "en-US" { true } else { false },
                    on:click=move |_| {
                        #[cfg(client)] 
                        {
                            value.set("en-US".to_string()); 
                        }
                    }
                ) { "EN" }
            }
            img(
                class="arrow",
                src=".perseus/static/icons/caret-up.svg",
                alt=(t!(cx, "alt_caret"))
            )
        }
    }
}
