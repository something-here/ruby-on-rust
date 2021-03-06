use std::collections::HashMap;

use regex::Regex;

// TODO should these be 'static ?
type TMatchingPatternLiterals = HashMap<&'static str, String>;
type TMatchingPatternRegexs = HashMap<&'static str, Regex>;
pub type TMatchingPatterns = ( TMatchingPatternLiterals, TMatchingPatternRegexs );

pub fn construct() -> TMatchingPatterns {
    let mut pattern_literals: TMatchingPatternLiterals = HashMap::new();
    let mut patterns: TMatchingPatternRegexs = HashMap::new();

    // NOTE
    // pattern!
    //   1. insert regex into pattern_literals, with wrapper ()
    //   2. insert regex plus prefix ^ into patterns, with wrapper ()
    // 
    // use patterns.insert(^regex) directly to avoid unnecessary pattern_literal
    // 

    macro_rules! pattern {
        ($name:expr, $pattern_lit:expr) => {
            let pattern_lit = format!(r"({})", $pattern_lit);
            let pattern_lit_with_prefix = format!(r"^({})", $pattern_lit);

            pattern_literals.insert($name, pattern_lit );
            patterns.insert($name, Regex::new( &pattern_lit_with_prefix ).unwrap());
        };
    }

    // 
    // NATIVE
    // 

    patterns.insert("any", Regex::new(r"(?s)^").unwrap()); // TODO NOT SURE
    patterns.insert("zlen", Regex::new(r"^$").unwrap()); // TODO REALLY?

    // 
    // CHARACTER CLASSES
    // 

    //   c_nl       = '\n' $ do_nl;
    pattern!("c_nl", r"\n"); // WITH EMBEDDED ACTION
    //   c_space    = [ \t\r\f\v];
    pattern!("c_space", r"[ \t\r\f\v]");
    //   c_space_nl = c_space | c_nl;
    pattern!("c_space_nl", r"[ \n\t\r\f\v]"); // TODO NOT CORRESPONDING
    //   c_eof      = 0x04 | 0x1a | 0 | zlen; # ^D, ^Z, \0, EOF
    pattern!("c_eof", r"$"); // TODO NOT CORRESPONDING

    //   c_eol      = c_nl | c_eof;
    pattern!("c_eol", r"(\n|\z)"); // TODO NOT CORRESPONDING
    //   c_any      = any - c_eof;
    pattern!("c_any", r"(?s)."); // TODO NOT CORRESPONDING

    //   c_nl_zlen  = c_nl | zlen;
    pattern!("c_nl_zlen", r"\n"); // TODO NOT CORRESPONDING

    //   c_line     = any - c_nl_zlen;
    pattern!("c_line", r"[^\n]"); // TODO NOT CORRESPONDING

    // TODO
    //   c_unicode  = c_any - 0x00..0x7f;
    //   c_upper    = [A-Z];
    //   c_lower    = [a-z_]  | c_unicode;
    //   c_alpha    = c_lower | c_upper;
    //   c_alnum    = c_alpha | [0-9];

    // 
    // TOKEN DEFINITIONS
    // 

    // # All operators are punctuation. There is more to punctuation
    // # than just operators. Operators can be overridden by user;
    // # punctuation can not.

    // # A list of operators which are valid in the function name context, but
    // # have different semantics in others.
    // operator_fname      = '[]' | '[]=' | '`'  | '-@' | '+@' | '~@'  | '!@' ;
    pattern!("operator_fname", r"(\[\])|(\[\]=)|`|(-@)|(\+@)|(~@)|(!@)");

    // # A list of operators which can occur within an assignment shortcut (+ → +=).
    // operator_arithmetic = '&'  | '|'   | '&&' | '||' | '^'  | '+'   | '-'  |
    //                       '*'  | '/'   | '**' | '~'  | '<<' | '>>'  | '%'  ;
    pattern!("operator_arithmetic", r"(&)|(\|)|(&&)|(\|\|)|(\^)|(\+)|(-)|(\*)|(/)|(\*\*)|(~)|(<<)|(>>)|(%)");

    // # A list of all user-definable operators not covered by groups above.
    // operator_rest       = '=~' | '!~' | '==' | '!=' | '!'   | '===' |
    //                       '<'  | '<=' | '>'  | '>=' | '<=>' | '=>'  ;
    pattern!("operator_rest", "(=~)|(!~)|(==)|(!=)|(!)|(===)|(<)|(<=)|(>)|(>=)|(<=>)|(=>)");

    //   # Note that `{` and `}` need to be referred to as e_lbrace and e_rbrace,
    //   # as they are ambiguous with interpolation `#{}` and should be counted.
    //   # These braces are not present in punctuation lists.

    //   # A list of punctuation which has different meaning when used at the
    //   # beginning of expression.
    //   punctuation_begin   = '-'  | '+'  | '::' | '('  | '['  |
    //                         '*'  | '**' | '&'  ;
    pattern!("punctuation_begin", r"(-)|(\+)|(::)|(\()|(\[)|(\*)|(\*\*)|(&)");

    //   # A list of all punctuation except punctuation_begin.
    //   punctuation_end     = ','  | '='  | '->' | '('  | '['  | ']'   |
    //                         '::' | '?'  | ':'  | '.'  | '..' | '...' ;
    pattern!("punctuation_end", r"(,)|(=)|(->)|(\()|(\[)|(\])|(::)|(\?)|(:)|(\.)|(\.\.)|(\.\.\.)");

    // # A list of keywords which have different meaning at the beginning of expression.
    // keyword_modifier    = 'if'     | 'unless' | 'while'  | 'until' | 'rescue' ;
    pattern!("keyword_modifier", "(if)|(unless)|(while)|(until)|(rescue)");

    // # A list of keywords which accept an argument-like expression, i.e. have the
    // # same post-processing as method calls or commands. Example: `yield 1`,
    // # `yield (1)`, `yield(1)`, are interpreted as if `yield` was a function.
    // keyword_with_arg    = 'yield'  | 'super'  | 'not'    | 'defined?' ;
    pattern!("keyword_with_arg", "(yield)|(super)|(not)|(defined?)");

    // # A list of keywords which accept a literal function name as an argument.
    // keyword_with_fname  = 'def'    | 'undef'  | 'alias'  ;
    pattern!("keyword_with_fname", "(def)|(undef)|(alias)");

    // # A list of keywords which accept an expression after them.
    // keyword_with_value  = 'else'   | 'case'   | 'ensure' | 'module' | 'elsif' | 'then'  |
    //                       'for'    | 'in'     | 'do'     | 'when'   | 'begin' | 'class' |
    //                       'and'    | 'or'     ;
    pattern!("keyword_with_value", "(else)|(case)|(ensure)|(module)|(elsif)|(then)|(for)|(in)|(do)|(when)|(begin)|(class)|(and)|(or)");

    // # A list of keywords which accept a value, and treat the keywords from
    // # `keyword_modifier` list as modifiers.
    // keyword_with_mid    = 'rescue' | 'return' | 'break'  | 'next'   ;
    pattern!("keyword_with_mid", "(rescue)|(return)|(break)|(next)");

    // # A list of keywords which do not accept an expression after them.
    // keyword_with_end    = 'end'    | 'self'   | 'true'   | 'false'  | 'retry'    |
    //                       'redo'   | 'nil'    | 'BEGIN'  | 'END'    | '__FILE__' |
    //                       '__LINE__' | '__ENCODING__';
    pattern!("keyword_with_end", "(end)|(self)|(true)|(false)|(retry)|(redo)|(nil)|(BEGIN)|(END)|(__FILE__)|(__LINE__)|(__ENCODING__)");

    // # All keywords.
    // keyword             = keyword_with_value | keyword_with_mid |
    //                       keyword_with_end   | keyword_with_arg |
    //                       keyword_with_fname | keyword_modifier ;
    // TODO simplify after NLL is online
    // let _keyword_pattern = format!(r"{}|{}|{}|{}|{}|{}",
    //     pattern_literals.get("keyword_with_value").unwrap(), pattern_literals.get("keyword_with_mid").unwrap(),
    //     pattern_literals.get("keyword_with_arg").unwrap(), pattern_literals.get("keyword_with_end").unwrap(),
    //     pattern_literals.get("keyword_with_fname").unwrap(), pattern_literals.get("keyword_modifier").unwrap()
    // );
    pattern!("keyword", "(else)|(case)|(ensure)|(module)|(elsif)|(then)|(for)|(in)|(do)|(when)|(begin)|(class)|(and)|(or)|(rescue)|(return)|(break)|(next)|(end)|(self)|(true)|(false)|(retry)|(redo)|(nil)|(BEGIN)|(END)|(__FILE__)|(__LINE__)|(__ENCODING__)|(yield)|(super)|(not)|(defined?)|(def)|(undef)|(alias)|(if)|(unless)|(while)|(until)|(rescue)");

    //   constant       = c_upper c_alnum*;
    pattern!("constant", "[[:upper:]][[:alnum:]]*");
    //   bareword       = c_alpha c_alnum*;
    pattern!("bareword", "[[:alpha:]][[:alnum:]]*");

    //   call_or_var    = c_lower c_alnum*;
    pattern!("call_or_var", "[[:lower:]][[:alnum:]]*");
    //   class_var      = '@@' bareword;
    pattern!("class_var", "@@[[:alpha:]][[:alnum:]]*");
    //   instance_var   = '@' bareword;
    pattern!("instance_var", "@[[:alpha:]][[:alnum:]]*");
    //   global_var     = '$'
    //       ( bareword | digit+
    //       | [`'+~*$&?!@/\\;,.=:<>"] # `
    //       | '-' c_alnum
    //       )
    //   ;
    // TODO use macro to combine complex pattern
    pattern!("global_var",
        format!(r"\$(({})|({})|({})|({}))",
            r"[[:alpha:]][[:alnum:]]*",
            r"[[:digit:]]+",
            r#"[`'\+~\*$&\?!@/\\;,\.=:<>"]"#,
            r"-[[:alnum:]]"
        )
    );

    //   # Ruby accepts (and fails on) variables with leading digit
    //   # in literal context, but not in unquoted symbol body.
    //   class_var_v    = '@@' c_alnum+;
    pattern!("class_var_v", "@@[:alnum:]+");
    //   instance_var_v = '@' c_alnum+;
    pattern!("instance_var_v", "@[:alnum:]+");

    //   label          = bareword [?!]? ':';
    pattern!("label", r"[[:alpha:]][[:alnum:]]*[\?!]?:");

    //   #
    //   # === NUMERIC PARSING ===
    //   #

    //   int_hex  = ( xdigit+ '_' )* xdigit* '_'? ;
    //   int_dec  = ( digit+ '_' )* digit* '_'? ;
    pattern!("int_dec", "([[:digit:]]+_)*[[:digit:]]*_?");
    //   int_bin  = ( [01]+ '_' )* [01]* '_'? ;

    //   flo_int  = [1-9] [0-9]* ( '_' digit+ )* | '0';
    //   flo_frac = '.' ( digit+ '_' )* digit+;
    //   flo_pow  = [eE] [+\-]? ( digit+ '_' )* digit+;

    //   int_suffix =
    //     ''   % { @num_xfrm = lambda { |chars| emit(:tINTEGER,   chars) } }
    //   | 'r'  % { @num_xfrm = lambda { |chars| emit(:tRATIONAL,  Rational(chars)) } }
    //   | 'i'  % { @num_xfrm = lambda { |chars| emit(:tIMAGINARY, Complex(0, chars)) } }
    //   | 'ri' % { @num_xfrm = lambda { |chars| emit(:tIMAGINARY, Complex(0, Rational(chars))) } };

    //   flo_pow_suffix =
    //     ''   % { @num_xfrm = lambda { |chars| emit(:tFLOAT,     Float(chars)) } }
    //   | 'i'  % { @num_xfrm = lambda { |chars| emit(:tIMAGINARY, Complex(0, Float(chars))) } };

    //   flo_suffix =
    //     flo_pow_suffix
    //   | 'r'  % { @num_xfrm = lambda { |chars| emit(:tRATIONAL,  Rational(chars)) } }
    //   | 'ri' % { @num_xfrm = lambda { |chars| emit(:tIMAGINARY, Complex(0, Rational(chars))) } };


    //   #
    //   # === ESCAPE SEQUENCE PARSING ===
    //   #

    //   # Escape parsing code is a Ragel pattern, not a scanner, and therefore
    //   # it shouldn't directly raise errors or perform other actions with side effects.
    //   # In reality this would probably just mess up error reporting in pathological
    //   # cases, through.

    //   # The amount of code required to parse \M\C stuff correctly is ridiculous.

    //   escaped_nl = "\\" c_nl;

    //   action unicode_points {
    //     @escape = ""

    //     codepoints  = tok(@escape_s + 2, p - 1)
    //     codepoint_s = @escape_s + 2

    //     if @version < 24
    //       if codepoints.start_with?(" ") || codepoints.start_with?("\t")
    //         diagnostic :fatal, :invalid_unicode_escape, nil,
    //           range(@escape_s + 2, @escape_s + 3)
    //       end

    //       if spaces_p = codepoints.index(/[ \t]{2}/)
    //         diagnostic :fatal, :invalid_unicode_escape, nil,
    //           range(codepoint_s + spaces_p + 1, codepoint_s + spaces_p + 2)
    //       end

    //       if codepoints.end_with?(" ") || codepoints.end_with?("\t")
    //         diagnostic :fatal, :invalid_unicode_escape, nil, range(p - 1, p)
    //       end
    //     end

    //     codepoints.scan(/([0-9a-fA-F]+)|([ \t]+)/).each do |(codepoint_str, spaces)|
    //       if spaces
    //         codepoint_s += spaces.length
    //       else
    //         codepoint = codepoint_str.to_i(16)

    //         if codepoint >= 0x110000
    //           diagnostic :error, :unicode_point_too_large, nil,
    //                      range(codepoint_s, codepoint_s + codepoint_str.length)
    //           break
    //         end

    //         @escape     += codepoint.chr(Encoding::UTF_8)
    //         codepoint_s += codepoint_str.length
    //       end
    //     end
    //   }

    //   action unescape_char {
    //     codepoint = @source_pts[p - 1]
    //     if (@escape = ESCAPES[codepoint]).nil?
    //       @escape = encode_escape(@source_buffer.slice(p - 1))
    //     end
    //   }

    //   action invalid_complex_escape {
    //     diagnostic :fatal, :invalid_escape
    //   }

    //   action slash_c_char {
    //     @escape = encode_escape(@escape[0].ord & 0x9f)
    //   }

    //   action slash_m_char {
    //     @escape = encode_escape(@escape[0].ord | 0x80)
    //   }

    //   maybe_escaped_char = (
    //         '\\' c_any      %unescape_char
    //     | ( c_any - [\\] )  % { @escape = @source_buffer.slice(p - 1).chr }
    //   );

    //   maybe_escaped_ctrl_char = ( # why?!
    //         '\\' c_any      %unescape_char %slash_c_char
    //     |   '?'             % { @escape = "\x7f" }
    //     | ( c_any - [\\?] ) % { @escape = @source_buffer.slice(p - 1).chr } %slash_c_char
    //   );

    //   escape = (
    //       # \377
    //       [0-7]{1,3}
    //       % { @escape = encode_escape(tok(@escape_s, p).to_i(8) % 0x100) }
    // 
    //       # \xff
    //     | 'x' xdigit{1,2}
    //         % { @escape = encode_escape(tok(@escape_s + 1, p).to_i(16)) }
    // 
    //       # %q[\x]
    //     | 'x' ( c_any - xdigit )
    //       % {
    //         diagnostic :fatal, :invalid_hex_escape, nil, range(@escape_s - 1, p + 2)
    //       }
    // 
    //       # \u263a
    //     | 'u' xdigit{4}
    //       % { @escape = tok(@escape_s + 1, p).to_i(16).chr(Encoding::UTF_8) }
    // 
    //       # \u123
    //     | 'u' xdigit{0,3}
    //       % {
    //         diagnostic :fatal, :invalid_unicode_escape, nil, range(@escape_s - 1, p)
    //       }
    // 
    //       # u{not hex} or u{}
    //     | 'u{' ( c_any - xdigit - [ \t}] )* '}'
    //       % {
    //         diagnostic :fatal, :invalid_unicode_escape, nil, range(@escape_s - 1, p)
    //       }
    // 
    //       # \u{  \t  123  \t 456   \t\t }
    //     | 'u{' [ \t]* ( xdigit{1,6} [ \t]+ )*
    //       (
    //         ( xdigit{1,6} [ \t]* '}'
    //           %unicode_points
    //         )
    //         |
    //         ( xdigit* ( c_any - xdigit - [ \t}] )+ '}'
    //           | ( c_any - [ \t}] )* c_eof
    //           | xdigit{7,}
    //         ) % {
    //           diagnostic :fatal, :unterminated_unicode, nil, range(p - 1, p)
    //         }
    //       )
    // 
    //       # \C-\a \cx
    //     | ( 'C-' | 'c' ) escaped_nl?
    //       maybe_escaped_ctrl_char
    // 
    //       # \M-a
    //     | 'M-' escaped_nl?
    //       maybe_escaped_char
    //       %slash_m_char
    // 
    //       # \C-\M-f \M-\cf \c\M-f
    //     | ( ( 'C-'   | 'c' ) escaped_nl?   '\\M-'
    //       |   'M-\\'         escaped_nl? ( 'C-'   | 'c' ) ) escaped_nl?
    //       maybe_escaped_ctrl_char
    //       %slash_m_char
    // 
    //     | 'C' c_any %invalid_complex_escape
    //     | 'M' c_any %invalid_complex_escape
    //     | ( 'M-\\C' | 'C-\\M' ) c_any %invalid_complex_escape
    // 
    //     | ( c_any - [0-7xuCMc] ) %unescape_char
    // 
    //     | c_eof % {
    //       diagnostic :fatal, :escape_eof, nil, range(p - 1, p)
    //     }
    //   );
    // TODO

    //   # Use rules in form of `e_bs escape' when you need to parse a sequence.
    //   e_bs = '\\' % {
    //     @escape_s = p
    //     @escape   = nil
    //   };
    pattern!("e_bs", r"\\");

    // #
    // # === STRING AND HEREDOC PARSING ===
    // #

    // # Heredoc parsing is quite a complex topic. First, consider that heredocs
    // # can be arbitrarily nested. For example:
    // #
    // #     puts <<CODE
    // #     the result is: #{<<RESULT.inspect
    // #       i am a heredoc
    // #     RESULT
    // #     }
    // #     CODE
    // #
    // # which, incidentally, evaluates to:
    // #
    // #     the result is: "  i am a heredoc\n"
    // #
    // # To parse them, lexer refers to two kinds (remember, nested heredocs)
    // # of positions in the input stream, namely heredoc_e
    // # (HEREDOC declaration End) and @herebody_s (HEREdoc BODY line Start).
    // #
    // # heredoc_e is simply contained inside the corresponding Literal, and
    // # when the heredoc is closed, the lexing is restarted from that position.
    // #
    // # @herebody_s is quite more complex. First, @herebody_s changes after each
    // # heredoc line is lexed. This way, at '\n' tok(@herebody_s, @te) always
    // # contains the current line, and also when a heredoc is started, @herebody_s
    // # contains the position from which the heredoc will be lexed.
    // #
    // # Second, as (insanity) there are nested heredocs, we need to maintain a
    // # stack of these positions. Each time #push_literal is called, it saves current
    // # @heredoc_s to literal.saved_herebody_s, and after an interpolation (possibly
    // # containing another heredocs) is closed, the previous value is restored.

    // e_heredoc_nl = c_nl % {
    // # After every heredoc was parsed, @herebody_s contains the
    // # position of next token after all heredocs.
    // if @herebody_s
    //     p = @herebody_s
    //     @herebody_s = nil
    // end
    // };
    // TODO INCOMPLETE
    //     e_heredoc_nl embedded proc
    pattern!("e_heredoc_nl", r"\n");


    //   #
    //   # === INTERPOLATION PARSING ===
    //   #

    //   # Interpolations with immediate variable names simply call into
    //   # the corresponding machine.

    //   interp_var = '#' ( global_var | class_var_v | instance_var_v );
    patterns.insert(
        "interp_var",
        Regex::new(
            &format!(r"^#({}|{}|{})",
            pattern_literals.get("global_var").unwrap(),
            pattern_literals.get("class_var_v").unwrap(),
            pattern_literals.get("instance_var_v").unwrap())
        ).unwrap()
    );

    //   # Interpolations with code blocks must match nested curly braces, as
    //   # interpolation ending is ambiguous with a block ending. So, every
    //   # opening and closing brace should be matched with e_[lr]brace rules,
    //   # which automatically perform the counting.
    //   #
    //   # Note that interpolations can themselves be nested, so brace balance
    //   # is tied to the innermost literal.
    //   #
    //   # Also note that literals themselves should not use e_[lr]brace rules
    //   # when matching their opening and closing delimiters, as the amount of
    //   # braces inside the characters of a string literal is independent.

    //   interp_code = '#{';
    pattern!("interp_code", r"#\{");

    //   e_lbrace = '{' % {
    //       NOTE embedded action moved to shared_actions
    //   };
    pattern!("e_lbrace", r"\{");

    //   e_rbrace = '}' % {
    //       NOTE embedded action moved to shared_actions
    //   };
    pattern!("e_rbrace", r"\}");

    // #
    // # === WHITESPACE HANDLING ===
    // #

    // # Various contexts in Ruby allow various kinds of whitespace
    // # to be used. They are grouped to clarify the lexing machines
    // # and ease collection of comments.

    // # A line of code with inline #comment at end is always equivalent
    // # to a line of code ending with just a newline, so an inline
    // # comment is deemed equivalent to non-newline whitespace
    // # (c_space character class).

    // w_space =
    //     c_space+
    //     | '\\' e_heredoc_nl
    //     ;
    pattern!("w_space", r"([ \t\r\f\v]+)"); // TODO INCOMPLETE

    // w_comment =
    //     '#'     %{ @sharp_s = p - 1 }
    //     # The (p == pe) condition compensates for added "\0" and
    //     # the way Ragel handles EOF.
    //     c_line* %{ emit_comment(@sharp_s, p == pe ? p - 2 : p) }
    //     ;
    // TODO INCOMPLETE
    // patterns.insert("w_comment", r"^#.*");

    // w_space_comment =
    //     w_space
    //     | w_comment
    //     ;
    // TODO INCOMPLETE
    pattern!("w_space_comment", r"[ \t\r\f\v]+");

    // # A newline in non-literal context always interoperates with
    // # here document logic and can always be escaped by a backslash,
    // # still interoperating with here document logic in the same way,
    // # yet being invisible to anything else.
    // #
    // # To demonstrate:
    // #
    // #     foo = <<FOO \
    // #     bar
    // #     FOO
    // #      + 2
    // #
    // # is equivalent to `foo = "bar\n" + 2`.

    // w_newline =
    //     e_heredoc_nl;
    pattern!("w_newline", r"\n"); // TODO NOT CORRESPONDING

    // w_any =
    //     w_space
    //     | w_comment
    //     | w_newline
    //     ;
    pattern!("w_any", r"[ \t\r\f\v]+"); // TODO INCOMPLETE

    //   #
    //   # === EXPRESSION PARSING ===
    //   #

    //   # These rules implement a form of manually defined lookahead.
    //   # The default longest-match scanning does not work here due
    //   # to sheer ambiguity.

    //   ambiguous_fid_suffix =         # actual    parsed
    //       [?!]    %{ tm = p }      | # a?        a?
    //       [?!]'=' %{ tm = p - 2 }    # a!=b      a != b
    //   ;
    // NOTE embedded action is `ambiguous_suffix`
    pattern!("ambiguous_ident_suffix", r"[\?!]=?");

    //   ambiguous_ident_suffix =       # actual    parsed
    //       ambiguous_fid_suffix     |
    //       '='     %{ tm = p }      | # a=        a=
    //       '=='    %{ tm = p - 2 }  | # a==b      a == b
    //       '=~'    %{ tm = p - 2 }  | # a=~b      a =~ b
    //       '=>'    %{ tm = p - 2 }  | # a=>b      a => b
    //       '==='   %{ tm = p - 3 }    # a===b     a === b
    //   ;
    // NOTE embedded action is `ambiguous_suffix`
    pattern!("ambiguous_ident_suffix", r"([\?!]=?)|=|(==)|(=~)|(=>)|(===)");

    //   ambiguous_symbol_suffix =      # actual    parsed
    //       ambiguous_ident_suffix |
    //       '==>'   %{ tm = p - 2 }    # :a==>b    :a= => b
    //   ;
    // NOTE embedded action is `ambiguous_suffix`
    pattern!("ambiguous_symbol_suffix", r"([\?!]=?)|=|(==)|(=~)|(=>)|(===)|(==>)");

    //   # Ambiguous with 1.9 hash labels.
    //   ambiguous_const_suffix =       # actual    parsed
    //       '::'    %{ tm = p - 2 }    # A::B      A :: B
    //   ;
    // NOTE embedded action is `ambiguous_suffix`
    pattern!("ambiguous_const_suffix", r"::");


    // # Resolving kDO/kDO_COND/kDO_BLOCK ambiguity requires embedding
    // # @cond/@cmdarg-related code to e_lbrack, e_lparen and e_lbrace.

    // e_lbrack = '[' % {
    //     @cond.push(false); @cmdarg.push(false)
    // };
    // NOTE embedded action moved to shared_actions
    pattern!("e_lbrack", r"\[");

    // # Ruby 1.9 lambdas require parentheses counting in order to
    // # emit correct opening kDO/tLBRACE.

    // e_lparen = '(' % {
    //     @cond.push(false); @cmdarg.push(false)

    //     @paren_nest += 1
    // };
    // NOTE embedded action moved to shared_actions
    pattern!("e_lparen", r"\(");

    // e_rparen = ')' % {
    //     @paren_nest -= 1
    // };
    // NOTE embedded action moved to shared_actions
    pattern!("e_rparen", r"\)");

    // ===
    // additional
    // ===

    // w_space+
    pattern!("w_space+", r"[ \t\r\f\v]+");

    // TODO NOT CORRESPONDING
    // NOTE cant combine current `w_any` with `*`, since it ends with `+`
    pattern!("w_any*", r"[ \t\r\f\v]*");

    // call_or_var - keyword
    // TODO simplify, dont rewrite pattern_lits
    pattern!("call_or_var - keyword",
        r"[[:lower:][:alnum:]*&&[^(else)|(case)|(ensure)|(module)|(elsif)|(then)|(for)|(in)|(do)|(when)|(begin)|(class)|(and)|(or)|(rescue)|(return)|(break)|(next)|(end)|(self)|(true)|(false)|(retry)|(redo)|(nil)|(BEGIN)|(END)|(__FILE__)|(__LINE__)|(__ENCODING__)|(yield)|(super)|(not)|(defined?)|(def)|(undef)|(alias)|(if)|(unless)|(while)|(until)|(rescue)]]"
    );

    (pattern_literals, patterns)
}
