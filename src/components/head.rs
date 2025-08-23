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
        meta(charset="UTF-8")
        meta(name="viewport", content="width=device-width, initial-scale=1.0")
        meta(http-equiv="X-UA-Compatible", content="ie=edge")
        title { (format!("Zarco - {}", title)) }
        meta(name="description", content="Gael Zarco: software engineer and CS student.")
        link(
            rel="preload",
            href=".perseus/static/fonts/Dots.ttf",
            as="font",
            type="font/ttf",
            crossorigin=""
        )
        link(
            rel="preload",
            href=".perseus/static/fonts/Geist.ttf",
            as="font",
            type="font/ttf",
            crossorigin=""
        )
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
            href=".perseus/static/assets/favicon.ico",
            type="image/x-icon",
            sizes="32x32"
        )
        style { r#"
            html { color-scheme: light; }
            @media (prefers-color-scheme: dark) {
              html { color-scheme: dark; }
            }
            @media (prefers-color-scheme: light) {
              :root {
                --bg:#FbFdFf; --bgt:#FbFdFF77; --h:#0d0b0f; --p:#7F7F7F; --text-a:#A3A3A3;
                --li-focus:#7F7F7F; --xp-left-p:#A3A3A3; --xp-right-a:#0d0b0f;
                --profile-img-filter:invert(1); --arrow-filter:invert(0);
                --bc-item:black; --border:#7F7F7F; --label:"Dark"; --icon:url(".perseus/static/icons/moon.svg");
              }
            }
            @media (prefers-color-scheme: dark) {
              :root {
                --bg:#0d0b0f; --bgt:#0d0b0f77; --h:#FbFdFf; --p:#A3A3A3; --text-a:#7F7F7F;
                --li-focus:#A3A3A3; --xp-left-p:#7F7F7F; --xp-right-a:#FbFdFf;
                --profile-img-filter:invert(0); --arrow-filter:invert(1);
                --bc-item:white; --border:#A3A3A3; --label:"Light"; --icon:url(".perseus/static/icons/sun.svg");
              }
            }
            html { background: var(--bg); }
            body { background: var(--bg); }
            .content {
              animation: fade_in 250ms ease-in forwards;
            }
            .theme_toggle {
              background: var(--bgt); 
              color: var(--border);
              border: 1px solid var(--border);
            }
            .theme_toggle::before{
              content:"";
              display: flex;
              align-items:center;
              width: 1.2em;
              height:1.2em;
              filter: var(--arrow-filter);
              background-color: black;
              -webkit-mask-image: var(--icon);
              -webkit-mask-repeat: no-repeat;
              -webkit-mask-position: center;
              -webkit-mask-size: 100% 100%;
                      mask-image: var(--icon);
                      mask-repeat: no-repeat;
                      mask-position: center;
                      mask-size: 100% 100%;
            }

            .theme_toggle::after{
              content: var(--label);
            }
            @media (prefers-reduced-motion: reduce) {
              .content {
                animation: none !important;
              }
            }
            @keyframes fade_in {
              from { opacity: 0; }
              to   { opacity: 1; }
            }
        "#}
    }
}
