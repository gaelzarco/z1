use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, _err_pos| {
        match err {
            ClientError::ServerError { status, message: _ } => match status {
                404 => (
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
                        title { (format!("Zarco - {}", status)) }
                        link(rel="preload", href=".perseus/static/styles.css", as="style")
                        link(rel="stylesheet", href=".perseus/static/styles.css")
                        link(
                            rel="icon",
                            href=".perseus/static/assets/favicon_white.ico",
                            type="image/x-icon",
                            sizes="32x32"
                        )
                    },
                    view! { cx,
                        main {
                            section(class="content") {

                                // Breadcrumb
                                nav(class="bc_wrapper") {
                                    a(class="bc_item", href="/") {
                                        img(
                                            class="bc_arrow",
                                            src=".perseus/static/icons/arrow-left.svg",
                                            alt="arrow"
                                        )
                                        "Home"
                                    }
                                }

                                section(class="about_wrapper") {
                                    h1 { (status) }
                                    p { "This page could not be found. Please head to the home page." }
                                }

                            }
                        }
                    },
                ),
                // 4xx is a client error
                _ if (400..500).contains(&status) => (
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
                        title { (format!("Zarco - {}", status)) }
                        link(rel="preload", href=".perseus/static/styles.css", as="style")
                        link(rel="stylesheet", href=".perseus/static/styles.css")
                        link(
                            rel="icon",
                            href=".perseus/static/assets/favicon_white.ico",
                            type="image/x-icon",
                            sizes="32x32"
                        )
                    },
                    view! { cx,
                        main {
                            section(class="content") {

                                // Breadcrumb
                                nav(class="bc_wrapper") {
                                    a(class="bc_item", href="/") {
                                        img(
                                            class="bc_arrow",
                                            src=".perseus/static/icons/arrow-left.svg",
                                            alt="arrow"
                                        )
                                        "Home"
                                    }
                                }

                                section(class="about_wrapper") {
                                    h1 { (status) }
                                    p { "Something went wrong. Please try again." }
                                }

                            }
                        }
                    },
                ),
                // 5xx is a server error
                _ => (
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
                        title { (format!("Zarco - {}", status)) }
                        link(rel="preload", href=".perseus/static/styles.css", as="style")
                        link(rel="stylesheet", href=".perseus/static/styles.css")
                        link(
                            rel="icon",
                            href=".perseus/static/assets/favicon_white.ico",
                            type="image/x-icon",
                            sizes="32x32"
                        )
                    },
                    view! { cx,
                        main {
                            section(class="content") {

                                // Breadcrumb
                                nav(class="bc_wrapper") {
                                    a(class="bc_item", href="/") {
                                        img(
                                            class="bc_arrow",
                                            src=".perseus/static/icons/arrow-left.svg",
                                            alt="arrow"
                                        )
                                        "Home"
                                    }
                                }

                                section(class="about_wrapper") {
                                    h1 { (status) }
                                    p { "Sorry, this one is on me. Please try again." }
                                }

                            }
                        }
                    },
                ),
            },
            ClientError::Panic(_) => (
                view! { cx, },
                view! { cx,
                    main {
                        section(class="content") {

                            // Breadcrumb
                            nav(class="bc_wrapper") {
                                a(class="bc_item", href="/") {
                                    img(
                                        class="bc_arrow",
                                        src=".perseus/static/icons/arrow-left.svg",
                                        alt="arrow"
                                    )
                                    "Home"
                                }
                            }

                            section(class="about_wrapper") {
                                h1 { "5XX" }
                                p { "A critical error occured. Please head to the home page." }
                            }

                        }
                    }
                },
            ),
            ClientError::FetchError(_) => (
                view! { cx, },
                view! { cx,
                    main {
                        section(class="content") {

                            // Breadcrumb
                            nav(class="bc_wrapper") {
                                a(class="bc_item", href="/") {
                                    img(
                                        class="bc_arrow",
                                        src=".perseus/static/icons/arrow-left.svg",
                                        alt="arrow"
                                    )
                                    "Home"
                                }
                            }

                            section(class="about_wrapper") {
                                h1 { "4XX" }
                                p { "Network error occured. Verify you have an internet connection and try again." }
                            }

                        }
                    }
                },
            ),
            _ => (
                view! { cx, },
                view! { cx,
                    main {
                        section(class="content") {

                            // Breadcrumb
                            nav(class="bc_wrapper") {
                                a(class="bc_item", href="/") {
                                    img(
                                        class="bc_arrow",
                                        src=".perseus/static/icons/arrow-left.svg",
                                        alt="arrow"
                                    )
                                    "Home"
                                }
                            }

                            section(class="about_wrapper") {
                                h1 { "?XX" }
                                p { "An internal error occured. Please try again." }
                            }

                        }
                    }
                },
            ),
        }
    })
}
