use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::theme;
use crate::components::head;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        main {
            theme::toggle_button()
            section(class="content") {

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
                                    "Built full-stack web applications using PHP "
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
                                    "Provided IT support, meeting Service Level Agreements"
                                    ", to health clinics and other businesses "
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
                                    "Developed front-end functionality with a focus on security "
                                    "using React.js. Improved application responsiveness and user "
                                    " experience in a collaborative team environment."
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
                                    "triangle coordinates into a render."
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
    head::builder(cx, String::from("Software Engineer"))
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
