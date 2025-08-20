mod templates;
mod components;
mod error_views;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::project::get_template())
        .error_views(crate::error_views::get_error_views())
        .static_alias("/resume", "./static/GAEL_ZARCO.pdf")
}

