
DO

LANGUAGE PLPGSQL

$$

DECLARE

    lang_en BIGINT;
    lang_es BIGINT;

    post_id BIGINT;
    post_title TEXT;
    post_description TEXT;
    permalink_value TEXT;
    post_body TEXT;

BEGIN

    post_id := i_post(
        1  -- user ID
    );

    post_title := 'Example post';
    post_description :=
        'In this post, we will test MarkDown (CommonMark).';
    permalink_value := 'example-post';
    


    post_body :=
'
# h1 Heading
## h2 Heading
### h3 Heading
#### h4 Heading
##### h5 Heading
###### h6 Heading


## Paragraphs

This is a paragraph. You can use bold text like
**this** or __this__. You can also use italic text
like *this* or _this_. You can even use automatic
quotations[^a]. It''s so cool!

[^a]: Random text.

## Horizontal Rules

___

---

***


## Blockquotes

> Blockquotes can also be nested...
>> ...by using additional greater-than signs right next to each other...
> > > ...or with spaces between arrows.


## Lists

Unordered:

+ Create a list by starting a line with `+`, `-`, or `*`
+ Sub-lists are made by indenting 2 spaces:
  - Marker character change forces new list start:
    * Ac tristique libero volutpat at
    + Facilisis in pretium nisl aliquet
    - Nulla volutpat aliquam velit
+ Very easy!

Ordered:

1. Lorem ipsum dolor sit amet
2. Consectetur adipiscing elit
3. Integer molestie lorem at massa

1. You can use sequential numbers...
1. ...or keep all the numbers as `1.`

Start numbering with offset:

57. foo
1. bar

Task list:

- [ ] Bla
- [x] Ble
- [x] Bli
- [ ] Blo
- [x] Blu


## Tables

Left aligned columns:

| Option | Description |
| ------ | ----------- |
| data   | bla bla bla |
| data   | bla bla bla |
| data   | bla bla bla |

Right aligned columns:

| Option | Description |
| ------:| -----------:|
| data   | bla bla bla |
| data   | bla bla bla |
| data   | bla bla bla |


## Code

Inline `code`.

Indented code:

    // Some comments
    line 1 of code
    line 2 of code
    line 3 of code


Block code "fences":

```
Sample text here...
```

Syntax highlighting

```js
var foo = function (bar) {
    return bar++;
};

console.log(foo(5));
```

## Links

[link text](https://wikipedia.org)

[link with title](https://wikipedia.org "title text!")

Autoconverted link: https://wikipedia.org


## Images

![Tukosmo](/static/img/tukosmo-logo-128.png)
![Tukosmo](/static/img/tukosmo-logo-128.png "Title")

Like links, Images also have a footnote style syntax

![Alt text][id]

With a reference later in the document defining the URL location:

[id]: /static/img/tukosmo-logo-128.png  "Tukosmo"
';



    PERFORM i_post_translation(
        post_id,
        s_language_id_by_code('en'),
        post_title,
        post_description,
        post_body,
        permalink_value,
        1,  -- user ID
        FALSE
    );

END;

$$;
