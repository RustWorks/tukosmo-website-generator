use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;


markup::define! {
    Languages<'a>(
        title: &'a str,
        lang_code: &'a str,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: AdminPanel {
                content: Content {},
                current_page: "languages",
                lang_code: lang_code,
            },
        }
    }

    Content() {
        "Bla"
    }
}

