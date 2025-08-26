use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::head;

pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, _err_pos| {
        match err {
            ClientError::ServerError { status, message: _ } => match status {
                404 => (
                    head::builder(cx, status.to_string()),
                    view! { cx,
                        main {
                            section(class="content") {
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
                                    p { (t!(cx, "404")) }
                                }

                            }
                        }
                    },
                ),
                // 4xx is a client error
                _ if (400..500).contains(&status) => (
                    head::builder(cx, status.to_string()),
                    view! { cx,
                        main {
                            section(class="content") {
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
                                    p { (t!(cx, "400")) }
                                }

                            }
                        }
                    },
                ),
                // 5xx is a server error
                _ => (
                    head::builder(cx, status.to_string()),
                    view! { cx,
                        main {
                            section(class="content") {
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
                                    p { (t!(cx, "500")) }
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
                        section(class="modal") {
                            section {
                                h1 { "4XX" }
                                p { (t!(cx, "400")) }
                            }
                        }
                    }
                },
            ),
            ClientError::FetchError(_) => (
                view! { cx, },
                view! { cx,
                    main {
                        section(class="modal") {
                            section {
                                h1 { "4XX" }
                                p { (t!(cx, "network")) }
                            }
                        }
                    }
                },
            ),
            _ => (
                view! { cx, },
                view! { cx,
                    main {
                        section(class="modal") {
                            section {
                                h1 { "5XX" }
                                p { (t!(cx, "500")) }
                            }
                        }
                    }
                },
            ),
        }
    })
}
