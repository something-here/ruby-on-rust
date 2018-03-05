// BASED ON https://github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/test/test_parser.rb

extern crate ruby_on_rust;

use ruby_on_rust::lexer::Lexer;
use ruby_on_rust::parser::parser::Parser;
use ruby_on_rust::ast::node::Node;


// helpers
// TODO macros


//   #
//   # Literals
//   #

//   def test_empty_stmt
//     assert_parses(
//       nil,
//       %q{})
//   end
// TODO FAILING
// #[test]
// fn empty_stmt() {
//     let content = String::from("");
//     let mut parser = Parser::new(content);
//     let node = parser.parse();
//     assert_eq!(node, Node::None);
// }

//   def test_nil
//     assert_parses(
//       s(:nil),
//       %q{nil},
//       %q{~~~ expression})
//   end
#[test]
fn nil() {
    let content = String::from("nil");
    let mut parser = Parser::new(content);
    let node = parser.parse();
    assert_eq!(node, Node::Nil);
}

//   def test_nil_expression
//     assert_parses(
//       s(:begin),
//       %q{()},
//       %q{^ begin
//         | ^ end
//         |~~ expression})

//     assert_parses(
//       s(:kwbegin),
//       %q{begin end},
//       %q{~~~~~ begin
//         |      ~~~ end
//         |~~~~~~~~~ expression})
//   end

//   def test_true
//     assert_parses(
//       s(:true),
//       %q{true},
//       %q{~~~~ expression})
//   end

//   def test_false
//     assert_parses(
//       s(:false),
//       %q{false},
//       %q{~~~~~ expression})
//   end

//   def test_int
//     assert_parses(
//       s(:int, 42),
//       %q{42},
//       %q{~~ expression})

//     assert_parses(
//       s(:int, 42),
//       %q{+42},
//       %q{^ operator
//         |~~~ expression})

//     assert_parses(
//       s(:int, -42),
//       %q{-42},
//       %q{^ operator
//         |~~~ expression})
//   end

//   def test_int___LINE__
//     assert_parses(
//       s(:int, 1),
//       %q{__LINE__},
//       %q{~~~~~~~~ expression})
//   end

//   def test_float
//     assert_parses(
//       s(:float, 1.33),
//       %q{1.33},
//       %q{~~~~ expression})

//     assert_parses(
//       s(:float, -1.33),
//       %q{-1.33},
//       %q{^ operator
//         |~~~~~ expression})
//   end

//   def test_rational
//     assert_parses(
//       s(:rational, Rational(42)),
//       %q{42r},
//       %q{~~~ expression},
//       SINCE_2_1)

//     assert_parses(
//       s(:rational, Rational(421, 10)),
//       %q{42.1r},
//       %q{~~~~~ expression},
//       SINCE_2_1)
//   end

//   def test_complex
//     assert_parses(
//       s(:complex, Complex(0, 42)),
//       %q{42i},
//       %q{~~~ expression},
//       SINCE_2_1)

//     assert_parses(
//       s(:complex, Complex(0, Rational(42))),
//       %q{42ri},
//       %q{~~~~ expression},
//       SINCE_2_1)

//     assert_parses(
//       s(:complex, Complex(0, 42.1)),
//       %q{42.1i},
//       %q{~~~~~ expression},
//       SINCE_2_1)

//     assert_parses(
//       s(:complex, Complex(0, Rational(421, 10))),
//       %q{42.1ri},
//       %q{~~~~~~ expression},
//       SINCE_2_1)
//   end

// TODO INCOMPLETE
#[test]
fn test_int() {
    let content = String::from("42");
    let mut parser = Parser::new(content);
    let node = parser.parse();
    assert_eq!(node, Node::Int(42));
}
