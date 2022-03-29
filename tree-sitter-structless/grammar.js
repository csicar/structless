

module.exports = grammar({
  name: 'structless',

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => repeat1($.top),

    top: $ => choice($.delimited, $.just_text, $.string),

    delimited: $ => choice(
      seq(field('delim_start', $.paren_start), field('content', repeat($.top)), field('delim_end', $.paren_end)),
      seq(field('delim_start', $.bracket_start), field('content', repeat($.top)), field('delim_end', $.bracket_end)),
      seq(field('delim_start', $.brace_start), field('content', repeat($.top)), field('delim_end', $.brace_end))
    ),
    paren_start: $ => token("("),
    paren_end: $ => token(")"),
    bracket_start: $ => token("["),
    bracket_end: $ => token("]"),
    brace_start: $ => token("{"),
    brace_end: $ => token("}"),

    just_text: $ => /([^\[\]\(\)\{\}\"])+/,

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
