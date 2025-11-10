[
  "use"
  "intrinsic"
  "data"
  "trait"
  "instance"
  "where"
  "def"
  "let"
  "in"
  "compose"
  "bind"
  "if"
  "then"
  "else"
] @keyword

(comment) @comment

(string_literal) @string
(integer_literal) @number
(bool_literal) @boolean
(list_literal) @list
(type_name) @type

(function_declaration
  name: (identifier) @function)