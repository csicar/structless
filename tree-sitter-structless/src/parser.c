#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 20
#define LARGE_STATE_COUNT 10
#define SYMBOL_COUNT 18
#define ALIAS_COUNT 0
#define TOKEN_COUNT 11
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 3
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 3

enum {
  sym_paren_start = 1,
  sym_paren_end = 2,
  sym_bracket_start = 3,
  sym_bracket_end = 4,
  sym_brace_start = 5,
  sym_brace_end = 6,
  sym_just_text = 7,
  anon_sym_DQUOTE = 8,
  aux_sym_string_content_token1 = 9,
  sym_escape_sequence = 10,
  sym_source_file = 11,
  sym_top = 12,
  sym_delimited = 13,
  sym_string = 14,
  sym_string_content = 15,
  aux_sym_source_file_repeat1 = 16,
  aux_sym_string_content_repeat1 = 17,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_paren_start] = "paren_start",
  [sym_paren_end] = "paren_end",
  [sym_bracket_start] = "bracket_start",
  [sym_bracket_end] = "bracket_end",
  [sym_brace_start] = "brace_start",
  [sym_brace_end] = "brace_end",
  [sym_just_text] = "just_text",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_content_token1] = "string_content_token1",
  [sym_escape_sequence] = "escape_sequence",
  [sym_source_file] = "source_file",
  [sym_top] = "top",
  [sym_delimited] = "delimited",
  [sym_string] = "string",
  [sym_string_content] = "string_content",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_string_content_repeat1] = "string_content_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_paren_start] = sym_paren_start,
  [sym_paren_end] = sym_paren_end,
  [sym_bracket_start] = sym_bracket_start,
  [sym_bracket_end] = sym_bracket_end,
  [sym_brace_start] = sym_brace_start,
  [sym_brace_end] = sym_brace_end,
  [sym_just_text] = sym_just_text,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_content_token1] = aux_sym_string_content_token1,
  [sym_escape_sequence] = sym_escape_sequence,
  [sym_source_file] = sym_source_file,
  [sym_top] = sym_top,
  [sym_delimited] = sym_delimited,
  [sym_string] = sym_string,
  [sym_string_content] = sym_string_content,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_string_content_repeat1] = aux_sym_string_content_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_paren_start] = {
    .visible = true,
    .named = true,
  },
  [sym_paren_end] = {
    .visible = true,
    .named = true,
  },
  [sym_bracket_start] = {
    .visible = true,
    .named = true,
  },
  [sym_bracket_end] = {
    .visible = true,
    .named = true,
  },
  [sym_brace_start] = {
    .visible = true,
    .named = true,
  },
  [sym_brace_end] = {
    .visible = true,
    .named = true,
  },
  [sym_just_text] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_content_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_escape_sequence] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_top] = {
    .visible = true,
    .named = true,
  },
  [sym_delimited] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_string_content] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_content_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_content = 1,
  field_delim_end = 2,
  field_delim_start = 3,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_content] = "content",
  [field_delim_end] = "delim_end",
  [field_delim_start] = "delim_start",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_delim_end, 1},
    {field_delim_start, 0},
  [2] =
    {field_content, 1},
    {field_delim_end, 2},
    {field_delim_start, 0},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(5);
      if (lookahead == '"') ADVANCE(15);
      if (lookahead == '(') ADVANCE(6);
      if (lookahead == ')') ADVANCE(7);
      if (lookahead == '[') ADVANCE(8);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead == ']') ADVANCE(9);
      if (lookahead == '{') ADVANCE(10);
      if (lookahead == '}') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(2)
      if (lookahead == '"') ADVANCE(15);
      if (lookahead == '\\') ADVANCE(3);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(16);
      if (lookahead != 0) ADVANCE(17);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(15);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      END_STATE();
    case 3:
      if (lookahead == '"' ||
          lookahead == '/' ||
          lookahead == '\\' ||
          lookahead == 'b' ||
          lookahead == 'f' ||
          lookahead == 'n' ||
          lookahead == 'r' ||
          lookahead == 't' ||
          lookahead == 'u') ADVANCE(18);
      END_STATE();
    case 4:
      if (eof) ADVANCE(5);
      if (lookahead == '"') ADVANCE(15);
      if (lookahead == '(') ADVANCE(6);
      if (lookahead == ')') ADVANCE(7);
      if (lookahead == '[') ADVANCE(8);
      if (lookahead == ']') ADVANCE(9);
      if (lookahead == '{') ADVANCE(10);
      if (lookahead == '}') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(sym_paren_start);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(sym_paren_end);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_bracket_start);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_bracket_end);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_brace_start);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_brace_end);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_just_text);
      if (lookahead == '"') ADVANCE(18);
      if (lookahead == '/' ||
          lookahead == '\\' ||
          lookahead == 'b' ||
          lookahead == 'f' ||
          lookahead == 'n' ||
          lookahead == 'r' ||
          lookahead == 't' ||
          lookahead == 'u') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != '(' &&
          lookahead != ')' &&
          (lookahead < '[' || ']' < lookahead) &&
          lookahead != '{' &&
          lookahead != '}') ADVANCE(14);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_just_text);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '(' &&
          lookahead != ')' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '{' &&
          lookahead != '}') ADVANCE(14);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_just_text);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '(' &&
          lookahead != ')' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '{' &&
          lookahead != '}') ADVANCE(14);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(aux_sym_string_content_token1);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(16);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(17);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(aux_sym_string_content_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(17);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_escape_sequence);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '(' &&
          lookahead != ')' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '{' &&
          lookahead != '}') ADVANCE(14);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 4},
  [2] = {.lex_state = 4},
  [3] = {.lex_state = 4},
  [4] = {.lex_state = 4},
  [5] = {.lex_state = 4},
  [6] = {.lex_state = 4},
  [7] = {.lex_state = 4},
  [8] = {.lex_state = 4},
  [9] = {.lex_state = 4},
  [10] = {.lex_state = 4},
  [11] = {.lex_state = 4},
  [12] = {.lex_state = 4},
  [13] = {.lex_state = 4},
  [14] = {.lex_state = 4},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 1},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 2},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_paren_start] = ACTIONS(1),
    [sym_paren_end] = ACTIONS(1),
    [sym_bracket_start] = ACTIONS(1),
    [sym_bracket_end] = ACTIONS(1),
    [sym_brace_start] = ACTIONS(1),
    [sym_brace_end] = ACTIONS(1),
    [sym_just_text] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [sym_escape_sequence] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(18),
    [sym_top] = STATE(6),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(6),
    [sym_paren_start] = ACTIONS(3),
    [sym_bracket_start] = ACTIONS(5),
    [sym_brace_start] = ACTIONS(7),
    [sym_just_text] = ACTIONS(9),
    [anon_sym_DQUOTE] = ACTIONS(11),
  },
  [2] = {
    [sym_top] = STATE(2),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_paren_start] = ACTIONS(15),
    [sym_paren_end] = ACTIONS(18),
    [sym_bracket_start] = ACTIONS(20),
    [sym_bracket_end] = ACTIONS(18),
    [sym_brace_start] = ACTIONS(23),
    [sym_brace_end] = ACTIONS(18),
    [sym_just_text] = ACTIONS(26),
    [anon_sym_DQUOTE] = ACTIONS(29),
  },
  [3] = {
    [sym_top] = STATE(7),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(7),
    [sym_paren_start] = ACTIONS(3),
    [sym_paren_end] = ACTIONS(32),
    [sym_bracket_start] = ACTIONS(5),
    [sym_brace_start] = ACTIONS(7),
    [sym_just_text] = ACTIONS(9),
    [anon_sym_DQUOTE] = ACTIONS(11),
  },
  [4] = {
    [sym_top] = STATE(8),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(8),
    [sym_paren_start] = ACTIONS(3),
    [sym_bracket_start] = ACTIONS(5),
    [sym_bracket_end] = ACTIONS(32),
    [sym_brace_start] = ACTIONS(7),
    [sym_just_text] = ACTIONS(9),
    [anon_sym_DQUOTE] = ACTIONS(11),
  },
  [5] = {
    [sym_top] = STATE(9),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(9),
    [sym_paren_start] = ACTIONS(3),
    [sym_bracket_start] = ACTIONS(5),
    [sym_brace_start] = ACTIONS(7),
    [sym_brace_end] = ACTIONS(32),
    [sym_just_text] = ACTIONS(9),
    [anon_sym_DQUOTE] = ACTIONS(11),
  },
  [6] = {
    [sym_top] = STATE(2),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(34),
    [sym_paren_start] = ACTIONS(3),
    [sym_bracket_start] = ACTIONS(5),
    [sym_brace_start] = ACTIONS(7),
    [sym_just_text] = ACTIONS(9),
    [anon_sym_DQUOTE] = ACTIONS(11),
  },
  [7] = {
    [sym_top] = STATE(2),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(2),
    [sym_paren_start] = ACTIONS(3),
    [sym_paren_end] = ACTIONS(36),
    [sym_bracket_start] = ACTIONS(5),
    [sym_brace_start] = ACTIONS(7),
    [sym_just_text] = ACTIONS(9),
    [anon_sym_DQUOTE] = ACTIONS(11),
  },
  [8] = {
    [sym_top] = STATE(2),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(2),
    [sym_paren_start] = ACTIONS(3),
    [sym_bracket_start] = ACTIONS(5),
    [sym_bracket_end] = ACTIONS(36),
    [sym_brace_start] = ACTIONS(7),
    [sym_just_text] = ACTIONS(9),
    [anon_sym_DQUOTE] = ACTIONS(11),
  },
  [9] = {
    [sym_top] = STATE(2),
    [sym_delimited] = STATE(10),
    [sym_string] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(2),
    [sym_paren_start] = ACTIONS(3),
    [sym_bracket_start] = ACTIONS(5),
    [sym_brace_start] = ACTIONS(7),
    [sym_brace_end] = ACTIONS(36),
    [sym_just_text] = ACTIONS(9),
    [anon_sym_DQUOTE] = ACTIONS(11),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(38), 2,
      ts_builtin_sym_end,
      sym_just_text,
    ACTIONS(40), 7,
      sym_paren_start,
      sym_paren_end,
      sym_bracket_start,
      sym_bracket_end,
      sym_brace_start,
      sym_brace_end,
      anon_sym_DQUOTE,
  [14] = 2,
    ACTIONS(42), 2,
      ts_builtin_sym_end,
      sym_just_text,
    ACTIONS(44), 7,
      sym_paren_start,
      sym_paren_end,
      sym_bracket_start,
      sym_bracket_end,
      sym_brace_start,
      sym_brace_end,
      anon_sym_DQUOTE,
  [28] = 2,
    ACTIONS(46), 2,
      ts_builtin_sym_end,
      sym_just_text,
    ACTIONS(48), 7,
      sym_paren_start,
      sym_paren_end,
      sym_bracket_start,
      sym_bracket_end,
      sym_brace_start,
      sym_brace_end,
      anon_sym_DQUOTE,
  [42] = 2,
    ACTIONS(50), 2,
      ts_builtin_sym_end,
      sym_just_text,
    ACTIONS(52), 7,
      sym_paren_start,
      sym_paren_end,
      sym_bracket_start,
      sym_bracket_end,
      sym_brace_start,
      sym_brace_end,
      anon_sym_DQUOTE,
  [56] = 2,
    ACTIONS(54), 2,
      ts_builtin_sym_end,
      sym_just_text,
    ACTIONS(56), 7,
      sym_paren_start,
      sym_paren_end,
      sym_bracket_start,
      sym_bracket_end,
      sym_brace_start,
      sym_brace_end,
      anon_sym_DQUOTE,
  [70] = 4,
    ACTIONS(58), 1,
      anon_sym_DQUOTE,
    STATE(16), 1,
      aux_sym_string_content_repeat1,
    STATE(19), 1,
      sym_string_content,
    ACTIONS(60), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [84] = 3,
    ACTIONS(62), 1,
      anon_sym_DQUOTE,
    STATE(17), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(64), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [95] = 3,
    ACTIONS(66), 1,
      anon_sym_DQUOTE,
    STATE(17), 1,
      aux_sym_string_content_repeat1,
    ACTIONS(68), 2,
      aux_sym_string_content_token1,
      sym_escape_sequence,
  [106] = 1,
    ACTIONS(71), 1,
      ts_builtin_sym_end,
  [110] = 1,
    ACTIONS(73), 1,
      anon_sym_DQUOTE,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(10)] = 0,
  [SMALL_STATE(11)] = 14,
  [SMALL_STATE(12)] = 28,
  [SMALL_STATE(13)] = 42,
  [SMALL_STATE(14)] = 56,
  [SMALL_STATE(15)] = 70,
  [SMALL_STATE(16)] = 84,
  [SMALL_STATE(17)] = 95,
  [SMALL_STATE(18)] = 106,
  [SMALL_STATE(19)] = 110,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [15] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(3),
  [18] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [20] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(4),
  [23] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(5),
  [26] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(10),
  [29] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(15),
  [32] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [36] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_top, 1),
  [40] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_top, 1),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_delimited, 2, .production_id = 1),
  [44] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_delimited, 2, .production_id = 1),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2),
  [48] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_delimited, 3, .production_id = 2),
  [52] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_delimited, 3, .production_id = 2),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3),
  [56] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3),
  [58] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [60] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [62] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_content, 1),
  [64] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [66] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_content_repeat1, 2),
  [68] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_content_repeat1, 2), SHIFT_REPEAT(17),
  [71] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_structless(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
