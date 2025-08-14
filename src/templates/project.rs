use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sycamore::prelude::*;
#[cfg(client)]
use wasm_bindgen::prelude::*;

const PROJECTS_JSON: &str = include_str!("../../static/projects.json");

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "ProjectPageStateRx")]
struct ProjectPageState {
    title: String,
    desc: String,
    abstr: String,
    goal: String,
    tools: Vec<String>,
    skills: Vec<String>,
    screenshots: Option<HashMap<String, String>>,
    result: String,
    links: Vec<Vec<String>>,
}

#[auto_scope]
fn project_page<G: Html>(cx: Scope, state: &ProjectPageStateRx) -> View<G> {
    #[cfg(client)]
    on_mount(cx, || fade_content());

    let shots = (*state.screenshots.get()).clone().unwrap_or_default();
    let screenshots_section = if shots.is_empty() {
        View::empty()
    } else {
        let frag = View::new_fragment(
            shots.into_iter().map(|(src, alt)| view! { cx,
                img(class="res_img", src=src, alt=alt, loading="lazy")
            }).collect()
        );
        view! { cx,
            section(class="generic_wrapper") {
                h2 { "Screenshots" }
                (frag)
            }
        }
    };

    view! { cx,
        main {
            section(class="content fade_in") {

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

                // Title
                section(class="about_wrapper") {
                    h1 { (state.title.get()) }
                    p { (state.desc.get()) }
                }

                // Abstract
                section(class="about_wrapper") {
                    h2 { "Abstract" }
                    p { (state.abstr.get()) }
                }

                // Methods
                section(class="generic_wrapper") {
                    h2 { "Methods" }
                    section(class="xp_item_wrapper") {

                        div(class="xp_item") {
                            div(class="xp_left") { p { "Goal" } }
                            div(class="xp_right") {
                                p { (state.goal.get()) }
                            }
                        }

                        div(class="xp_item") {
                            div(class="xp_left") { p { "Tools" } }
                            div(class="xp_right") {
                                ul(class="focus") {
                                    (View::new_fragment({
                                        let tools: Vec<String> = (*state.tools.get()).to_vec();
                                        tools.into_iter()
                                            .map(|i| view! {
                                                cx, li(class="focus") { (i) }
                                            })
                                            .collect()
                                    }))
                                }
                            }
                        }

                        div(class="xp_item") {
                            div(class="xp_left") { p { "Skills" } }
                            div(class="xp_right") {
                                ul(class="focus") {
                                    (View::new_fragment({
                                        let skills: Vec<String> = (*state.skills.get()).to_vec();
                                        skills.into_iter()
                                            .map(|i| view! {
                                                cx, li(class="focus") { (i) }
                                            })
                                            .collect()
                                    }))
                                }
                            }
                        }
                    }
                }

                // Screenshots
                (screenshots_section)

                // Results
                section(class="about_wrapper") {
                    h2 { "Results" }
                    p {
                        ( (state.result.get()) )
                    }
                }

                // Links
                section(class="generic_wrapper") {
                    h2 { "Links" }
                    section(class="xp_item_wrapper") {
                        (View::new_fragment({
                            let links: Vec<Vec<String>> = state.links.get().clone().to_vec();
                            links.into_iter().map(|row| {
                                let label = row.get(0).cloned().unwrap_or_default();
                                let text  = row.get(1).cloned().unwrap_or_default();
                                let href  = row.get(2).cloned().unwrap_or_default();

                                view! { cx,
                                    div(class="xp_item") {
                                        div(class="xp_left") { p { (label) } }
                                        div(class="xp_right") {
                                            a(
                                                href=href,
                                                target="_blank",
                                                rel="noopener noreferrer"
                                            )
                                            {
                                                (text)
                                                img(
                                                    class="arrow",
                                                    src=".perseus/static/icons/arrow-top-right.svg",
                                                    alt="Arrow"
                                                )
                                            }
                                        }
                                    }
                                }
                            }).collect()
                        }))
                    }
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, _props: ProjectPageState) -> View<SsrNode> {
    view! {
        cx,
        meta(charset="UTF-8")
        meta(name="viewport", content="width=device-width, initial-scale=1.0")
        meta(http-equiv="X-UA-Compatible", content="ie=edge")
        title { "Zarco - RGX" }
        link(rel="preload", href=".perseus/static/styles.css", as="style")
        link(rel="stylesheet", href=".perseus/static/styles.css")
        link(
            rel="icon",
            href=".perseus/static/assets/favicon_white.ico",
            type="image/x-icon", sizes="32x32"
        )
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["rgx".into(), "space".into(), "moxie".into()],
        extra: ().into(),
    }
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>)
    -> Result<ProjectPageState, BlamedError<anyhow::Error>> {
    let data: HashMap<String, ProjectPageState> =
        serde_json::from_str(PROJECTS_JSON).unwrap();

    let slug = info.path;
    data.get(&slug)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("unknown project: {}", slug).into())
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("project")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .view_with_state(project_page)
        .head_with_state(head)
        .build()
}

#[cfg(client)]
#[wasm_bindgen(module = "/src/scripts/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = fadeContent)]
    fn fade_content();
}
