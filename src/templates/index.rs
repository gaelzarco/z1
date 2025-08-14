use perseus::prelude::*;
use sycamore::prelude::*;
#[cfg(client)]
use wasm_bindgen::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    #[cfg(client)]
    on_mount(cx, || fade_content());

    view! {
        cx,
        main {
            section(class="content fade_in") {

                section(class="profile_wrapper") {
                    div(class="profile_left") {
                        img(
                            src=".perseus/static/assets/dot-circle-white.webp",
                            alt="Dot Matrix Circle",
                            loading="lazy",
                            height="95",
                            width="150"
                        )
                    }
                    div(class="profile_right") {
                        h1 { "Gael Zarco" }
                        p { "Software Engineer - CS Student" }
                        a(
                            href=".perseus/static/GAEL_ZARCO.pdf",
                            target="_blank", rel="noopener noreferrer"
                        ) { "Resume" }
                    }
                }

                section(class="about_wrapper") {
                    h2 { "About" }
                    p {
                        "Experienced in web development and IT support. Seeking part-time and "
                        "internship opportunities while pursuing a bachelor's in computer "
                        "science."
                    }
                }

                section(class="generic_wrapper") {
                    h2 { "Experience" }

                    section(class="xp_item_wrapper") {
                        div(class="xp_item") {
                            div(class="xp_left") { p { "2024" } }
                            div(class="xp_right") {
                                a(
                                    href="https://702pros.com/",
                                    target="_blank",
                                    rel="noopener noreferrer"
                                ) {
                                    "Web Developer - 702 Pros"
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt="Arrow"
                                    )
                                }
                                p {
                                    "Built full-stack applications using WordPress, PHP, and Apache "
                                    "with a focus on reducing external dependencies. Met tight "
                                    "deadlines and project milestones while simultaneously "
                                    "incorporating client feedback."
                                }
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
                                    "IT Technician - Team Tech Solutions"
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt="Arrow"
                                    )
                                }
                                p {
                                    "Provided IT support, meeting Service Level Agreement "
                                    "requirements, to large health clinics and consumer markets "
                                    "across the US. Reported directly to Director of Operations "
                                    "to track initiatives and discuss objectives to improve "
                                    "client feedback and support efficiency."
                                }
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
                                    "Software Engineer Intern - Productos-AI"
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg", 
                                        alt="Arrow"
                                    )
                                }
                                p {
                                    "Increased front-end functionality, security, and performance "
                                    "using React.js and AWS Lambda. Improved API response-times "
                                    "and participated in an Agile team environment."
                                }
                            }
                        } }
                }

                section(class="generic_wrapper") {
                    h2 { "Projects" }

                    section(class="xp_item_wrapper") {
                        div(class="xp_item") {
                            div(class="xp_left") { p { "2025" } }
                            div(class="xp_right") {
                                a(href="/project/rgx") {
                                    "RGX"
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt="Arrow"
                                    )
                                }
                                p {
                                    "Software rasterizer that parses .obj files and translates "
                                    "triangle and texture coordinates into a fully rendered raster."
                                }
                            }
                        }

                        div(class="xp_item") {
                            div(class="xp_left") { p { "2024" } }
                            div(class="xp_right") {
                                a(href="project/space") {
                                    "Space"
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt="Arrow"
                                    )
                                }
                                p { "Web application to chat in real-time." }
                            }
                        }

                        div(class="xp_item") {
                            div(class="xp_left") { p { "2023" } }
                            div(class="xp_right") {
                                a(href="/project/moxie") {
                                    "Moxie"
                                    img(
                                        class="arrow",
                                        src=".perseus/static/icons/arrow-top-right.svg",
                                        alt="Arrow"
                                    )
                                }
                                p { "Social media web application." }
                            }
                        }
                    }
                }

                section(class="generic_wrapper") {
                    h2 { "Links" }

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
                                        alt="Arrow"
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
    view! { cx,
        meta(charset="UTF-8")
        meta(name="viewport", content="width=device-width, initial-scale=1.0")
        meta(http-equiv="X-UA-Compatible", content="ie=edge")
        title { "Zarco - Software Engineer" }
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

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}

#[cfg(client)]
#[wasm_bindgen(module = "/src/scripts/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = fadeContent)]
    fn fade_content();
}
