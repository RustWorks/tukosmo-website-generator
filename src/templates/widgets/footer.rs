use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use chrono::Datelike;


markup::define! {
    Footer<'a>(
        t: &'a TranslateI18N,
    ) {
        div[
            class = "site-footer",
        ] {
            div[
                class = "container",
            ] {
                div[
                    class = "site-credits",
                ] {
                    @t.name_year_copyright
                    .replace("{name}", "Lajto")  // TODO: This shouldn't be
                                                 // the website's name, but
                                                 // the company/author/owner's
                    .replace("{year}", &chrono::Utc::now().year().to_string())
                }
            }
        }
    }
}

