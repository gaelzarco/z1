use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sycamore::prelude::*;
use crate::components::theme;
use crate::components::lang;
use crate::components::head;

const PROJECTS_JSON: &str = include_str!("../../static/projects.json");

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "ProjectPageStateRx")]
struct ProjectPageState {
    slug: String,
    tools: Vec<String>,
    skills: Vec<String>,
    skills_spanish: Vec<String>,
    screenshots: Option<HashMap<String, String>>,
    links: Vec<Vec<String>>,
}

#[auto_scope]
fn project_page<G: Html>(cx: Scope, state: &ProjectPageStateRx) -> View<G> {
    let tools  = create_memo(cx, || state.tools.get().to_vec());

    let home_url = t!(cx, "home_url");
    let home_label = t!(cx, "home_label");
    let is_es = home_label != "Home";

    let skills = create_memo(cx, move || {
        if is_es {
            state.skills_spanish.get().to_vec()
        } else {
            state.skills.get().to_vec()
        }
    });

    let shots = create_memo(cx, || {
        (*state.screenshots.get())
            .clone()
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<(String, String)>>()
    });

    let links = create_memo(cx, || {
        state.links.get().to_vec().into_iter().map(|row| {
            let label = row.get(0).cloned().unwrap_or_default();
            let text  = row.get(1).cloned().unwrap_or_default();
            let href  = row.get(2).cloned().unwrap_or_default();
            (label, text, href)
        }).collect::<Vec<_>>()
    });

    let slug = state.slug.get();
    let (title_t, desc_t, abstr_t, goal_t, result_t) = match slug.as_str() {
        "rgx" => (
            t!(cx, "proj_one_title"),
            t!(cx, "proj_one_desc"),
            t!(cx, "proj_one_abstr"),
            t!(cx, "proj_one_goal"),
            t!(cx, "proj_one_result"),
        ),
        "space" => (
            t!(cx, "proj_two_title"),
            t!(cx, "proj_two_desc"),
            t!(cx, "proj_two_abstr"),
            t!(cx, "proj_two_goal"),
            t!(cx, "proj_two_result"),
        ),
        "moxie" => (
            t!(cx, "proj_three_title"),
            t!(cx, "proj_three_desc"),
            t!(cx, "proj_three_abstr"),
            t!(cx, "proj_three_goal"),
            t!(cx, "proj_three_result"),
        ),
        _ => (
            t!(cx, "projects_header"),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ),
    };

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
                    h2 { (t!(cx, "screenshots_header")) }
                    (frag)
                }
            }
        } else {
            View::empty()
        }
    };

    view! { cx,
        main {
            lang::toggle_button()
            theme::toggle_button()
            section(class="content") {

                nav(class="bc_wrapper") {
                    a(class="bc_item", href=(home_url)) {
                        img(class="bc_arrow", src=".perseus/static/icons/arrow-left.svg", alt=(t!(cx, "alt_arrow")))
                        (t!(cx, "home_label"))
                    }
                }

                section(class="about_wrapper") {
                    h1 { (title_t.clone()) }
                    p  { (desc_t.clone()) }
                }

                section(class="about_wrapper") {
                    h2 { (t!(cx, "abstract_header")) }
                    p  { (abstr_t.clone()) }
                }

                section(class="generic_wrapper") {
                    h2 { (t!(cx, "methods_header")) }
                    section(class="xp_item_wrapper") {

                        div(class="xp_item") {
                            div(class="xp_left")  { p { (t!(cx, "goal_label")) } }
                            div(class="xp_right") { p { (goal_t.clone()) } }
                        }

                        div(class="xp_item") {
                            div(class="xp_left")  { p { (t!(cx, "tools_label")) } }
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

                        div(class="xp_item") {
                            div(class="xp_left")  { p { (t!(cx, "skills_label")) } }
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

                (screenshots_view())

                section(class="about_wrapper") {
                    h2 { (t!(cx, "results_header")) }
                    p  { (result_t.clone()) }
                }

                section(class="generic_wrapper") {
                    h2 { (t!(cx, "links_header")) }
                    section(class="xp_item_wrapper") {
                        (View::new_fragment(
                            links.get().iter().cloned().map(|(label, text, href)| view! { cx,
                                div(class="xp_item") {
                                    div(class="xp_left")  { p { (label) } }
                                    div(class="xp_right") {
                                        a(href=href, target="_blank", rel="noopener noreferrer") {
                                            (text)
                                            img(class="arrow", src=".perseus/static/icons/arrow-top-right.svg", alt=(t!(cx, "alt_arrow")))
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
    let title = match props.slug.as_str() {
        "rgx"   => t!(cx, "proj_one_title"),
        "space" => t!(cx, "proj_two_title"),
        "moxie" => t!(cx, "proj_three_title"),
        _       => t!(cx, "projects_header"),
    };
    head::builder(cx, title)
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["rgx".into(), "space".into(), "moxie".into()],
        extra: ().into(),
    }
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> ProjectPageState {
    let slug = info.path;
    let data: HashMap<String, ProjectPageState> = serde_json::from_str(PROJECTS_JSON).unwrap();
    let mut state = data.get(&slug).cloned().unwrap_or_else(|| ProjectPageState {
        slug: String::new(),
        tools: vec![],
        skills: vec![],
        skills_spanish: vec![],
        screenshots: None,
        links: vec![],
    });
    state.slug = slug;
    state
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("project")
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
        .view_with_state(project_page)
        .head_with_state(head)
        .build()
}
