// %type expr { Expression }
// %type T_INTEGER { i64 }
// // %left OP_PLUS OP_MINUS.

%include {
/* extra include */

// #[derive(Debug)]
// pub enum Operator {
//     Addition,
//     Substraction,
// }

#[derive(Debug)]
pub enum Expression {
    Number(i64),
    // Binary(Operator, Box<Expression>, Box<Expression>),
    Primary_bool(bool)
}

}

%derive_token { Debug }

%extra_argument { Option<Expression> }
%syntax_error { println!("syntax error"); }

%parse_accept {
    println!("parse_accept");
}

%parse_failure {
    println!("parse_failure!");
}

program ::= expr(A). {
    self.extra = Some(A);
}

// expr(A) ::= expr(B) OP_PLUS expr(C). {
//     A = Expression::Binary(Operator::Addition, Box::new(B), Box::new(C));
// }
// expr(A) ::= expr(B) OP_MINUS expr(C). {
//     A = Expression::Binary(Operator::Substraction, Box::new(B), Box::new(C));
// }
// expr(A) ::= LPAREN expr(B) RPAREN. {
//     A = B;
// }
expr(A) ::= T_INTEGER(B). { A = Expression::Number(B); }
expr(A) ::= KW_TRUE. { A = Expression:: }
