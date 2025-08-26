use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::theme;
use crate::components::locale;

#[component]
pub fn all<G: Html>(cx: Scope) -> View<G> {
    view!{ cx,
        section(class="controls_container") {
            locale::select_dropdown()
            theme::toggle_button()
        }
    }
}
