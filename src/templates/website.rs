use markup;


markup::define! {
    Home<'a>(title: &'a str) {
        @markup::doctype()
        html[
            lang = "en"
        ] {
            head {
                meta[charset = "utf-8"];
                meta[
                    name = "viewport",
                    content = "width=device-width, initial-scale=1.0",
                ];

                title { @title }

                meta[
                    name = "description",
                    content = "",
                ];
                meta[
                    name = "author",
                    content = "",
                ];

                // These are many
                link[
                    rel = "apple-touch-icon",
                    sizes = "",
                    href = "",
                ];

                // These are many
                link[
                    rel = "icon",
                    type = "",
                    sizes = "",
                    href = "",
                ];

                link[
                    rel = "manifest",
                    href = "",
                ];

                // Styles
                link[
                    rel = "stylesheet",
                    href = "",
                ];

                style {
                    "body { background: #fafbfc; }"
                    "#main { padding: 2rem; }"
                }
            }
            body {
                //@Content { param1 param2 }
                ""

                script[
                    src = "",
                ] {}
            }
        }
    }
}
