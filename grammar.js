module.exports = grammar({
  name: 'structless',

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => repeat1($.top),

    top : $ => choice($.delimited, $.just_text, $.string),

    delimited: $ => choice(seq("(", repeat($.top), ")"), seq("[", repeat($.top), "]")),

    just_text: $ => /([^\[\]\(\)\"])+/,

    string: $ => choice(
      seq('"', '"'),
      seq('"', $.string_content, '"')
    ),

    string_content: $ => repeat1(choice(
      token.immediate(/[^\\"\n]+/),
      $.escape_sequence
    )),

    escape_sequence: $ => token.immediate(seq(
      '\\',
      /(\"|\\|\/|b|f|n|r|t|u)/
    )),
  }
});
