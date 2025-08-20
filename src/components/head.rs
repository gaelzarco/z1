use sycamore::prelude::*;

#[component]
pub fn builder(cx: Scope, title: String) -> View<SsrNode> {
    const VER: &str = env!("CARGO_PKG_VERSION");

    view! {cx,
        script { r#"
            (() => {
                let t = localStorage.getItem('theme');
                if (!t) {
                    t = window.matchMedia('(prefers-color-scheme: dark)')
                        .matches ? 'dark' : 'light';
                };
                document.documentElement.setAttribute('data-theme', t);
            })();
            "# }
        style { r#"
            .content {
              animation: fade_in 250ms ease-in forwards;
            }

            @keyframes fade_in {
              from { opacity: 0; }
              to   { opacity: 1; }
            }

            @media (prefers-reduced-motion: reduce) {
              .content {
                animation: none !important;
              }
            }
        "#}

        meta(charset="UTF-8")
        meta(name="viewport", content="width=device-width, initial-scale=1.0")
        meta(http-equiv="X-UA-Compatible", content="ie=edge")
        title { (format!("Zarco - {}", title)) }
        link(
            rel="preload",
            href=format!(".perseus/static/styles.css?={}", VER),
            as="style"
        )
        link(
            rel="stylesheet",
            href=format!(".perseus/static/styles.css?={}", VER),
        )
        link(
            rel="icon",
            href=".perseus/static/assets/favicon_white.ico",
            type="image/x-icon",
            sizes="32x32"
        )
    }
}
