use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::tukosmo::TukosmoAResponse;


markup::define! {
    Tukosmo<'a>(
        title: &'a str,
        q: &'a TukosmoAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "tukosmo",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

