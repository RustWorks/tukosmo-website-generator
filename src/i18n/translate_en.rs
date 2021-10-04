

pub fn translate_en(text_value: &str) -> &str {
    match text_value {
        "(website_name)" => "MyExample",
        "{n} posts" => "{n} posts",
        "{n} results of {m}" => "{n} results of {m}",
        "{name} © {year} [copyright]" => "{name} © {year}",
        "Account" => "Account",
        "Add language" => "Add language",
        "Author" => "Author",
        "Blog" => "Blog",
        "Cancel" => "Cancel",
        "Code" => "Code",
        "Dashboard" => "Dashboard",
        "Data" => "Data",
        "Deleted [post]" => "Deleted",
        "Description" => "Description",
        "Documentation" => "Documentation",
        "Draft" => "Draft",
        "Drafts" => "Drafts",
        "Edit" => "Edit",
        "Edit language: {name}" => "Edit language: {name}",
        "Edit post: '{title}'" => "Edit post: '{title}'",
        "Example: en" => "Example: en",
        "Files" => "Files",
        "Forgotten password?" => "Forgotten password?",
        "General" => "General",
        "Hello, {name}." => "Hello, {name}.",
        "Help [noun]" => "Help",
        "I hope you are having a great day!" =>
            "I hope you are having a great day!",
        "Language" => "Language",
        "Language name" => "Language name",
        "Languages" => "Languages",
        "Last update" => "Last update",
        "Login [noun]" => "Login",
        "Login [verb]" => "Login",
        "Logout [verb]" => "Logout",
        "New post" => "New post",
        "Next [page]" => "Next",
        "Original author" => "Original author",
        "Page {n}" => "Page {n}",
        "Pages" => "Pages",
        "Permalink" => "Permalink",
        "Posts" => "Posts",
        "Post's body" => "Post's body",
        "Previous [page]" => "Previous",
        "Published" => "Published",
        "Published [posts]" => "Published",
        "Read more" => "Read more",
        "Scheduled [posts]" => "Scheduled",
        "Server" => "Server",
        "Sessions" => "Sessions",
        "Settings" => "Settings",
        "Sign up [verb]" => "Sign up",
        "Statistics" => "Statistics",
        "Status" => "Status",
        "Submit" => "Submit",
        "Title" => "Title",
        "translated by {name}" => "translated by {name}",
        "Trash" => "Trash",
        "Tukosmo" => "Tukosmo",
        "Tukosmo Admin Panel" => "Tukosmo Admin Panel",
        "Untranslated" => "Untranslated",
        "Untranslated [posts]" => "Untranslated",
        "untranslated" => "untranslated",
        "Users" => "Users",
        "Visit website" => "Visit website",
        "Website" => "Website",
        "Your email" => "Your email",
        "Your password" => "Your password",

        //--------------//

        _ => "[unstranslated]"
    }
}

