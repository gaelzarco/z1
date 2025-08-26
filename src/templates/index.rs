use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::controls;
use crate::components::head;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    #[cfg(client)]
    {
        let reactor = Reactor::<G>::from_cx(cx);
        reactor.preload(cx, "/project/rgx");
        reactor.preload(cx, "/project/moxie");
        reactor.preload(cx, "/project/space");
    }

    let name_t           = t!(cx, "name");
    let subtitle_t       = t!(cx, "subtitle");

    let exp1_title       = t!(cx, "exp_one_title");
    let exp1_desc        = t!(cx, "exp_one_desc");
    let exp2_title       = t!(cx, "exp_two_title");
    let exp2_desc        = t!(cx, "exp_two_desc");
    let exp3_title       = t!(cx, "exp_three_title");
    let exp3_desc        = t!(cx, "exp_three_desc");

    let proj_one_url     = t!(cx, "proj_one_url");
    let proj_one_title   = t!(cx, "proj_one_title");
    let proj_one_desc    = t!(cx, "proj_one_desc");

    let proj_two_url     = t!(cx, "proj_two_url");
    let proj_two_title   = t!(cx, "proj_two_title");
    let proj_two_desc    = t!(cx, "proj_two_desc");

    let proj_three_url   = t!(cx, "proj_three_url");
    let proj_three_title = t!(cx, "proj_three_title");
    let proj_three_desc  = t!(cx, "proj_three_desc");

    view! {
        cx,
        main {
            controls::all()
            section(class="content") {

                section(class="profile_wrapper") {
                    div(class="profile_left") {
                        img(
                            src=".perseus/static/assets/dot-circle-white.webp",
                            alt=(t!(cx, "alt_dot_circle")),
                            loading="lazy",
                            height="95",
                            width="150"
                        )
                    }
                    div(class="profile_right") {
                        h1 { (name_t) }
                        p  { (subtitle_t) }
                    }
                }

                section(class="about_wrapper") {
                    h2 { (t!(cx, "profile_header")) }
                    p  { (t!(cx, "profile_desc")) }
                }

                section(class="generic_wrapper") {
                    h2 { (t!(cx, "experience_header")) }
                    section(class="xp_item_wrapper") {
                        div(class="xp_item") {
                            div(class="xp_left") { p { "2024" } }
                            div(class="xp_right") {
                                a(
                                    href="https://702pros.com/",
                                    target="_blank",
                                    rel="noopener noreferrer"
                                ) {
                                    (exp1_title)
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt=(t!(cx, "alt_arrow"))
                                    )
                                }
                                p { (exp1_desc) }
                            }
                        }

                        div(class="xp_item") {
                            div(class="xp_left") { p { "2023" } }
                            div(class="xp_right") {
                                a(
                                    href="https://teamtechsolution.com/",
                                    target="_blank",
                                    rel="noopener noreferrer"
                                ) {
                                    (exp2_title)
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt=(t!(cx, "alt_arrow"))
                                    )
                                }
                                p { (exp2_desc) }
                            }
                        }

                        div(class="xp_item") {
                            div(class="xp_left") { p { "2023" } }
                            div(class="xp_right") {
                                a(
                                    href="https://productos-ai.com/en/",
                                    target="_blank",
                                    rel="noopener noreferrer"
                                ) {
                                    (exp3_title)
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt=(t!(cx, "alt_arrow"))
                                    )
                                }
                                p { (exp3_desc) }
                            }
                        }
                    }
                }

                section(class="generic_wrapper") {
                    h2 { (t!(cx, "projects_header")) }

                    section(class="xp_item_wrapper") {
                        div(class="xp_item") {
                            div(class="xp_left") { p { "2025" } }
                            div(class="xp_right") {
                                a( href=proj_one_url ) {
                                    (proj_one_title)
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt=(t!(cx, "alt_arrow"))
                                    )
                                }
                                p { (proj_one_desc) }
                            }
                        }

                        div(class="xp_item") {
                            div(class="xp_left") { p { "2024" } }
                            div(class="xp_right") {
                                a( href=proj_two_url ) {
                                    (proj_two_title)
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt=(t!(cx, "alt_arrow"))
                                    )
                                }
                                p { (proj_two_desc) }
                            }
                        }

                        div(class="xp_item") {
                            div(class="xp_left") { p { "2023" } }
                            div(class="xp_right") {
                                a( href=proj_three_url ) {
                                    (proj_three_title)
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt=(t!(cx, "alt_arrow"))
                                    )
                                }
                                p { (proj_three_desc) }
                            }
                        }
                    }
                }

                section(class="generic_wrapper") {
                    h2 { (t!(cx, "links_header")) }

                    section(class="xp_item_wrapper") {
                        div(class="xp_item") {
                            div(class="xp_left") { p { "LinkedIn" } }
                            div(class="xp_right") {
                                a(
                                    href="https://linkedin.com/in/gaelzarco",
                                    target="_blank"
                                ) {
                                    "@gaelzarco"
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt=(t!(cx, "alt_arrow"))
                                    )
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    head::builder(cx, String::from("Software Engineer"))
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
