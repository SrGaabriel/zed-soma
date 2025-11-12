[
  (if)
  (then)
  (else)
  (let)
  (in)
  (compose)
  (bind)
  (where)
  (def)
  (intrinsic)
  (data)
  (trait)
  (instance)
  (import)
] @keyword

(comment) @comment

(string_literal) @string
(integer_literal) @number
(bool_literal) @boolean
(list_literal) @list

(function_declaration
  name: (_) @function
)

(function_signature
  name: (_) @function
)

(data_type_declaration
  name: (type_name) @type
)

(data_type_declaration
  type_parameters: (identifier) @type
)

(intrinsic_data_type
  name: (type_name) @type
)

(trait_constraint
  parameter: (identifier) @type
  trait: (type_name) @type
)

(constructor_pattern
  constructor: (type_name) @constructor
)

(import_path
  (identifier) @module)

(type) @type

[
  (arrow)
  (equal)
  (colon)
  (colon_colon)
  (bar)
  (double_arrow)
  (operator)
  (reverse_arrow)
  "<-"
] @operator

(constructor_name) @constructor

(app_expression
  function: (identifier) @function)

[
    (lbraces)
    (rbraces)
    "("
    ")"
] @punctuation.bracket

[
    "."
    "/"
    ","
] @punctuation.delimiter