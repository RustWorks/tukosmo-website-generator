use markup;

use crate::i18n::t::t;
use crate::database::s_post_by_lang_permalink::PostDB;
use crate::markdown::render_html::render_html;


markup::define! {
    Post<'a>(
        lang_code: &'a str,
        post: &'a PostDB,
    ) {
        article[
            class = "post",
        ] {
            div[
                class = "post-image",
            ] {
                figure {
                    img[
                        src = "https://www.azamara.com/sites/default/files/heros/reykjavik-iceland-1800x1000.jpg",
                        alt = "reykjavik",
                    ];
                }
            }

            div[
                class = "post-content",
            ] {
                div[
                    class = "post-header",
                ] {
                    div[
                        class = "post-title",
                    ] {
                        h1[
                            class = "post-title",
                        ] {
                            @post.title
                        }
                    }

                    div[
                        class = "post-meta",
                    ] {
                        span[
                            class = "post-meta-author",
                        ] {
                            "Lajto Mekadimon"
                        }
                        
                        span[
                            class = "post-meta-date",
                        ] {
                            //"August 27, 2021"
                            @post.date
                        }
                        
                        span[
                            class = "post-meta-edit",
                        ] {
                            //"August 27, 2021"
                            {&t("Edit", lang_code)}
                        }
                    }
                }

                div[
                    class = "post-body",
                ] {
                    {markup::raw(&render_html(&post.body))}
                }
            }
        }
    }
}

