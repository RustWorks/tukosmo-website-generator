use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::markdown::render_html::render_html;
use crate::database::types::{WebsiteDataDB, PostDB};


markup::define! {
    Post<'a>(
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
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
                        src = "https://www.azamara.com/sites/default/\
                               files/heros/reykjavik-iceland-1800x1000.jpg",
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
                            a[
                                href = "/{lang}/blog"
                                    .replace("{lang}", &data.lang.code)
                                ,
                            ] {
                                i[class = "eos-icons"] {
                                    "person"
                                }

                                " "

                                @post.author_name
                            }
                        }
                        
                        span[
                            class = "post-meta-date",
                        ] {
                            a[
                                href = "/{lang}/blog"
                                    .replace("{lang}", &data.lang.code)
                                ,
                            ] {
                                i[class = "eos-icons"] {
                                    "calendar_today"
                                }

                                " "

                                {t_date(&post.date, &data.lang.code)}
                            }
                        }
                        
                        @if let Some(_user) = &data.userd {
                            span[
                                class = "post-meta-edit",
                            ] {
                                a[
                                    href = "/{lang}/admin/edit_post?id={id}"
                                        .replace("{lang}", &data.lang.code)
                                        .replace("{id}", &post.id.to_string())
                                    ,
                                ] {
                                    i[class = "eos-icons"] {
                                        "mode_edit"
                                    }

                                    " "

                                    @t.edit
                                }
                            }
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

