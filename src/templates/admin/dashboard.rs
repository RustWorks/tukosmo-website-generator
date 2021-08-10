use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;


markup::define! {
    Dashboard<'a>(
        title: &'a str,
        lang_code: &'a str,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: AdminPanel {
                content: Content {},
                current_page: "dashboard",
            },
        }
    }

    Content() {
        section[class = "hero is-info welcome is-small"] {
            div[class = "hero-body"] {
                div[class = "container"] {
                    h1[class = "title"] {
                        "Hello, Lajto."
                    }
                    h2[class = "subtitle"] {
                        "I hope you are having a great day!"
                    }
                }
            }
        }

        section[class = "info-tiles"] {
            div[class = "tile is-ancestor has-text-centered"] {
                div[class = "tile is-parent"] {
                    article[class = "tile is-child box"] {
                        p[class = "title"] { "12" }
                        p[class = "subtitle"] { "users" }
                    }
                }
                div[class = "tile is-parent"] {
                    article[class = "tile is-child box"] {
                        p[class = "title"] { "34" }
                        p[class = "subtitle"] { "posts" }
                    }
                }
                div[class = "tile is-parent"] {
                    article[class = "tile is-child box"] {
                        p[class = "title"] { "4" }
                        p[class = "subtitle"] { "pages" }
                    }
                }
                div[class = "tile is-parent"] {
                    article[class = "tile is-child box"] {
                        p[class = "title"] { "19" }
                        p[class = "subtitle"] { "files" }
                    }
                }
            }
        }

        /*
        div[class = "columns"] {
            div[class = "column is-6"] {
                div[class = "card events-card"] {
                    header[class = "card-header"] {
                        p[class = "card-header-title"] {
                            "Events"
                        }
                        a[
                            href = "#",
                            class = "card-header-icon",
                        ] {
                            span[class = "icon"] {
                                i[class = "eos-icons"] { "keyboard_arrow_down" }
                            }
                        }
                    }
                    div[class = "card-table"] {
                        div[class = "content"] {
                            table[
                                class = "table is-fullwidth is-striped",
                            ] {
                                tbody {
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                    tr {
                                        td[width = "5%"] {
                                            i[class = "eos-icons"] { "notifications_none" }
                                        }
                                        td { "Lorum ipsum dolem aire" }
                                        td[class = "level-right"] {
                                            a[
                                                class = "button is-small is-primary",
                                                href = "#",
                                            ] { "Action" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    footer[class = "card-footer"] {
                        a[
                            href = "#",
                            class = "card-footer-item",
                        ] {
                            "View all"
                        }
                    }
                }
            }
            div[class = "column is-6"] {
                div[class = "card"] {
                    header[class = "card-header"] {
                        p[class = "card-header-title"] {
                            "Inventory Search"
                        }
                        a[
                            href = "#",
                            class = "card-header-icon",
                        ] {
                            span[class = "icon"] {
                                i[class = "eos-icons"] { "keyboard_arrow_down" }
                            }
                        }
                    }
                    div[class = "card-content"] {
                        div[class = "content"] {
                            div[class = "control has-icons-left has-icons-right"] {
                                input[
                                    class = "input is-large",
                                    type = "text",
                                    placeholder = "",
                                ];
                                span[class = "icon is-medium is-left"] {
                                    i[class = "eos-icons"] { "search" }
                                }
                                span[class = "icon is-medium is-right"] {
                                    i[class = "eos-icons"] { "check" }
                                }
                            }
                        }
                    }
                }
                div[class = "card"] {
                    header[class = "card-header"] {
                        p[class = "card-header-title"] {
                            "User Search"
                        }
                        a[
                            href = "#",
                            class = "card-header-icon",
                        ] {
                            span[class = "icon"] {
                                i[class = "eos-icons"] { "keyboard_arrow_down" }
                            }
                        }
                    }
                    div[class = "card-content"] {
                        div[class = "content"] {
                            div[class = "control has-icons-left has-icons-right"] {
                                input[
                                    class = "input is-large",
                                    type = "text",
                                    placeholder = "",
                                ];
                                span[class = "icon is-medium is-left"] {
                                    i[class = "eos-icons"] { "search" }
                                }
                                span[class = "icon is-medium is-right"] {
                                    i[class = "eos-icons"] { "check" }
                                }
                            }
                        }
                    }
                }
            }
        }
        */
    }
}

