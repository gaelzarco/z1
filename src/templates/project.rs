use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sycamore::prelude::*;
use crate::templates::theme;
use crate::templates::header;
#[cfg(client)]
use wasm_bindgen::prelude::*;

const PROJECTS_JSON: &str = include_str!("../../static/projects.json");

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "ProjectPageStateRx")]
pub struct ProjectPageState {
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
    on_mount(cx, || scroll_top());

    let tools  = create_memo(cx, || state.tools.get().to_vec());
    let skills = create_memo(cx, || state.skills.get().to_vec());

    // Flatten screenshots HashMap -> Vec<(src, alt)>
    let shots = create_memo(cx, || {
        (*state.screenshots.get())
            .clone()
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<(String, String)>>()
    });

    // Flatten links Vec<Vec<String>> -> Vec<(label, text, href)>
    let links = create_memo(cx, || {
        state.links.get().to_vec().into_iter().map(|row| {
            let label = row.get(0).cloned().unwrap_or_default();
            let text  = row.get(1).cloned().unwrap_or_default();
            let href  = row.get(2).cloned().unwrap_or_default();
            (label, text, href)
        }).collect::<Vec<_>>()
    });

    let screenshots_view = move || {
        let shots_vec = shots.get();
        if !shots_vec.is_empty() {
            let frag = View::new_fragment(
                shots_vec
                    .iter()
                    .cloned()
                    .map(|(src, alt)| view! { cx,
                        img(class="res_img", src=src, alt=alt, loading="lazy")
                    })
                    .collect()
            );
            view! { cx,
                section(class="generic_wrapper") {
                    h2 { "Screenshots" }
                    (frag)
                }
            }
        } else {
            View::empty()
        }
    };

    view! { cx,
        main {
            section(class="content") {
                theme::toggle_button()

                // Breadcrumb
                nav(class="bc_wrapper") {
                    a(class="bc_item", href="/") {
                        img(class="bc_arrow", src=".perseus/static/icons/arrow-left.svg", alt="arrow")
                        "Home"
                    }
                }

                // Title
                section(class="about_wrapper") {
                    h1 { (state.title.get()) }
                    p  { (state.desc.get()) }
                }

                // Abstract
                section(class="about_wrapper") {
                    h2 { "Abstract" }
                    p  { (state.abstr.get()) }
                }

                // Methods
                section(class="generic_wrapper") {
                    h2 { "Methods" }
                    section(class="xp_item_wrapper") {
                        // Goal
                        div(class="xp_item") {
                            div(class="xp_left")  { p { "Goal" } }
                            div(class="xp_right") { p { (state.goal.get()) } }
                        }
                        // Tools
                        div(class="xp_item") {
                            div(class="xp_left")  { p { "Tools" } }
                            div(class="xp_right") {
                                ul(class="focus") {
                                    (View::new_fragment(
                                        tools.get().iter().cloned()
                                            .map(|t| view! { cx, li(class="focus") { (t) } })
                                            .collect()
                                    ))
                                }
                            }
                        }
                        // Skills
                        div(class="xp_item") {
                            div(class="xp_left")  { p { "Skills" } }
                            div(class="xp_right") {
                                ul(class="focus") {
                                    (View::new_fragment(
                                        skills.get().iter().cloned()
                                            .map(|s| view! { cx, li(class="focus") { (s) } })
                                            .collect()
                                    ))
                                }
                            }
                        }
                    }
                }

                // Screenshots
                (screenshots_view())

                // Results
                section(class="about_wrapper") {
                    h2 { "Results" }
                    p  { (state.result.get()) }
                }

                // Links
                section(class="generic_wrapper") {
                    h2 { "Links" }
                    section(class="xp_item_wrapper") {
                        (View::new_fragment(
                            links.get().iter().cloned().map(|(label, text, href)| view! { cx,
                                div(class="xp_item") {
                                    div(class="xp_left")  { p { (label) } }
                                    div(class="xp_right") {
                                        a(href=href, target="_blank", rel="noopener noreferrer") {
                                            (text)
                                            img(class="arrow", src=".perseus/static/icons/arrow-top-right.svg", alt="Arrow")
                                        }
                                    }
                                }
                            }).collect()
                        ))
                    }
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, props: ProjectPageState) -> View<SsrNode> {
    header::head(cx, props.title)
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
    let slug = info.path;
    let data: HashMap<String, ProjectPageState> =
        serde_json::from_str(PROJECTS_JSON).unwrap();
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
    #[wasm_bindgen(js_name = scrollTop)]
    fn scroll_top();
}
