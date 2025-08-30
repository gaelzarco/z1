mod templates;
mod components;
mod error_views;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .locales_and_translations_manager("en-US", &["es-ES"])
        .template(crate::templates::index::get_template())
        .template(crate::templates::project::get_template())
        .error_views(crate::error_views::get_error_views())
        .static_alias("/resume", "./static/GAEL_ZARCO.pdf")
}
