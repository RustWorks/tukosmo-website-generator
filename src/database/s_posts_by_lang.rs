use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;


pub struct PostDB {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub body: String,
    pub permalink: String,
    pub author: i64,
    pub author_name: String,
    pub original_author: i64,
    pub original_author_name: String,
    pub date: String,
    pub date_trans: String,
}

pub fn s_posts_by_lang(
    lang_code: String
) -> Vec<PostDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_posts_by_lang(s_language_id_by_code($1))",
            &[&lang_code,]
        ) {
            for row in rows {
                let post_id: i64 = row.get("tp_id");
                let post_title: String = row.get("tp_title");
                let post_description: String = row.get("tp_description");
                let post_body: String = row.get("tp_body");
                let post_permalink: String = row.get("tp_permalink");
                let post_author: i64 = row.get("tp_author");
                let post_author_name: String = row.get("tp_author_name");
                let post_original_author: i64 = row.get("tp_original_author");
                let post_original_author_name: String = row.get("tp_original_author_name");
                let post_date: String = row.get("tp_date");
                let post_date_trans: String = row.get("tp_date_trans");

                vec.push(
                    PostDB {
                        id: post_id,
                        title: post_title,
                        description: post_description,
                        body: post_body,
                        permalink: post_permalink,
                        author: post_author,
                        author_name: post_author_name,
                        original_author: post_original_author,
                        original_author_name: post_original_author_name,
                        date: post_date,
                        date_trans: post_date_trans,
                    }
                );                
            }
        }
        // TODO: Control the error!
    }

    vec
}
