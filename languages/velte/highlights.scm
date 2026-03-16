; inherits: html

(comment) @comment

(raw_text) @none

; Tag + attribute names
(tag_name) @tag
(erroneous_end_tag_name) @tag
(attribute_name) @attribute

; Attribute values
[
  (attribute_value)
  (quoted_attribute_value)
] @string

; Velte block/expression punctuation
[
  "{"
  "}"
] @punctuation.bracket

[
  "#"
  ":"
  "/"
  "@"
] @tag.delimiter

; Velte keywords/tags
[
  "as"
  "key"
  "html"
  "snippet"
  "render"
] @keyword

"const" @type.qualifier

[
  "if"
  "else"
  "then"
] @keyword.conditional

"each" @keyword.repeat

[
  "await"
  "then"
] @keyword.coroutine

"catch" @keyword.exception
"debug" @keyword.debug

(snippet_name) @function
