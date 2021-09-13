use markup;

use crate::database::data::DataDB;


markup::define! {
    AdminLangDropdown<'a>(
        lang_code: &'a str,
        route: &'a str,
        data: &'a DataDB,
    ) {
        div[class = "dropdown is-hoverable"] {
            div[class = "dropdown-trigger"] {
                button[
                    class = "button",
                    "aria-haspopup" = "true",
                    "aria-controls" = "dropdown-menu",
                ] {
                    span {
                        @for lang in &data.languages {  // TODO: Add to DataDB the current language or something
                            @if &lang.code == lang_code {
                                @lang.name
                            }
                        }
                    }
                    span[class = "icon is-small"] {
                        i[class = "eos-icons"] {
                            "arrow_drop_down"
                        }
                    }
                }
            }
            div[
                class = "dropdown-menu",
                id = "dropdown-menu",
                role = "menu",
            ] {
                div[class = "dropdown-content"] {
                    @for lang in &data.languages {
                        a[
                            href = "/{lang}{route}"
                                .replace("{lang}", &lang.code)
                                .replace("{route}", route),
                            class = if &lang.code != lang_code {
                                "dropdown-item"
                            } else {
                                "dropdown-item is-active"
                            },
                        ] {
                            @lang.name
                        }
                    }
                }
            }
        }
    }
}

