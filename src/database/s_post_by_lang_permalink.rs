use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;


pub struct PostDB {
    pub id: i64,
    pub lang: i64,
    pub title: String,
    pub body: String,
    pub permalink: String,
    pub author: i64,
    pub date: String,
    pub date_trans: String,
}

pub fn s_post_by_lang_permalink(
    lang_code: String,
    permalink_value: String
) -> Option<PostDB> {
    let mut post_struct: Option<PostDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_post_by_lang_permalink(s_language_id_by_code($1), $2)",
            &[&lang_code, &permalink_value]
        ) {
            for row in rows {
                let post_id: i64 = row.get("tp_id");
                let post_lang: i64 = row.get("tp_lang");
                let post_title: String = row.get("tp_title");
                let post_body: String = row.get("tp_body");
                let post_permalink: String = row.get("tp_permalink");
                let post_author: i64 = row.get("tp_author");
                let post_date: String = row.get("tp_date");
                let post_date_trans: String = row.get("tp_date_trans");

                post_struct = Some(
                    PostDB {
                        id: post_id,
                        lang: post_lang,
                        title: post_title,
                        body: post_body,
                        permalink: post_permalink,
                        author: post_author,
                        date: post_date,
                        date_trans: post_date_trans,
                    }
                );
            }
        }
        // TODO: Control the error!
    }

    post_struct
}

