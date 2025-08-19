use perseus::prelude::*;
use sycamore::prelude::*;

#[engine_only_fn]
pub fn head(cx: Scope, title: String) -> View<SsrNode> {
    view! { cx,
        script { r#"(function(){
            var t=localStorage.getItem('theme');
            if(!t) t = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
            document.documentElement.setAttribute('data-theme', t);
        })();"# }
        style { r#"
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
        link(rel="preload", href=".perseus/static/styles.css", as="style")
        link(rel="stylesheet", href=".perseus/static/styles.css")
        link(
            rel="icon",
            href=".perseus/static/assets/favicon_white.ico",
            type="image/x-icon",
            sizes="32x32"
        )
    }
}

