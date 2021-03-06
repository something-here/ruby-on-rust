// https://raw.githubusercontent.com/whitequark/parser/2a73841d6da04a5ab9bd270561165fd766722d43/lib/parser/builders/default.rb

use parser::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    // for rules which doesnot need to return a real node
    Dummy,
    // for rules which returns a vec of nodes
    Nodes(Vec<Node>),
    // for rules which may returns a result being `nil`, when the rule is applied so we cannot return a None, i guess. still not sure about this.
    Null,

    Nil,

    True,
    False,

    Int(isize),

    Str(String),
    DStr(Vec<Node>),

    Sym(String),

    Array(Vec<Node>),

    Pair { key: Box<Node>, value: Box<Node> },
    Hash(Vec<Node>), // TODO Hash(Vec<Node::Pair>) after enum variants become types

    NSelf,
    LVar(String),

    Ident(String),
    Assign(Box<Node>, Token, Box<Node>), // TODO a dummy NodeType for builder.assign

    // assignable
    LVasgn(String, Vec<Node>),

    Begin(Vec<Node>),
}


// def unary_num(unary_t, numeric)
//   value, = *numeric
//   operator_loc = loc(unary_t)

//   case value(unary_t)
//   when '+'
//     value = +value
//   when '-'
//     value = -value
//   end

//   numeric.updated(nil, [ value ],
//     :location =>
//       Source::Map::Operator.new(
//         operator_loc,
//         operator_loc.join(numeric.loc.expression)))
// end
// TODO INCOMPLETE
// DUMMY ALWAYS Node::Int
pub fn unary_num(t_unary: Token, n_simple_numeric: Node) -> Node {
    let mut numeric_value = if let Node::Int(int_value) = n_simple_numeric { int_value } else { panic!(); };

    if let Token::T_UNARY_NUM(polarty) = t_unary {
        match polarty.as_ref() {
            "+" => (),
            "-" => { numeric_value = 0 - numeric_value; },
            _ => { panic!(); }
        }
    } else { panic!(); }

    return Node::Int(numeric_value);
}

// # Strings

// def string(string_t)
//   n(:str, [ string_value(string_t) ],
//     delimited_string_map(string_t))
// end

// def string_internal(string_t)
//   n(:str, [ string_value(string_t) ],
//     unquoted_map(string_t))
// end

// def string_compose(begin_t, parts, end_t)
//   if collapse_string_parts?(parts)
//     if begin_t.nil? && end_t.nil?
//       parts.first
//     else
//       n(:str, parts.first.children,
//         string_map(begin_t, parts, end_t))
//     end
//   else
//     n(:dstr, [ *parts ],
//       string_map(begin_t, parts, end_t))
//   end
// end
// TODO INCOMPLETE
pub fn string_compose(parts: Node) -> Node {
    if is_collapse_string_parts(&parts) {}

    // TODO DUMMY
    if let Node::Str(string_value) = parts { return Node::Str(string_value); }
    if let Node::Nodes(str_nodes) = parts {
        if let Node::Str(str_value) = (*str_nodes.get(0).unwrap()).clone() {
            return Node::Str(str_value);
        }
    }

    panic!("string_compose UNIMPL");
}

// def character(char_t)
//   n(:str, [ string_value(char_t) ],
//     prefix_string_map(char_t))
// end

// def __FILE__(__FILE__t)
//   n0(:__FILE__,
//     token_map(__FILE__t))
// end

// # Symbols

// def symbol(symbol_t)
//   n(:sym, [ string_value(symbol_t).to_sym ],
//     prefix_string_map(symbol_t))
// end

// def symbol_internal(symbol_t)
//   n(:sym, [ string_value(symbol_t).to_sym ],
//     unquoted_map(symbol_t))
// end

// def symbol_compose(begin_t, parts, end_t)
//   if collapse_string_parts?(parts)
//     str = parts.first
// 
//     n(:sym, [ str.children.first.to_sym ],
//       collection_map(begin_t, str.loc.expression, end_t))
//   elsif @parser.version == 18 && parts.empty?
//     diagnostic :error, :empty_symbol, nil, loc(begin_t).join(loc(end_t))
//   else
//     n(:dsym, [ *parts ],
//       collection_map(begin_t, parts, end_t))
//   end
// end

// # Executable strings

// def xstring_compose(begin_t, parts, end_t)
//   n(:xstr, [ *parts ],
//     string_map(begin_t, parts, end_t))
// end

// # Indented (interpolated, noninterpolated, executable) strings

// def dedent_string(node, dedent_level)
//   if !dedent_level.nil?
//     dedenter = Lexer::Dedenter.new(dedent_level)
// 
//     if node.type == :str
//       str = node.children.first
//       dedenter.dedent(str)
//     elsif node.type == :dstr || node.type == :xstr
//       node.children.each do |str_node|
//         if str_node.type == :str
//           str = str_node.children.first
//           dedenter.dedent(str)
//         else
//           dedenter.interrupt
//         end
//       end
//     end
//   end

//   node
// end

// # Regular expressions

// def regexp_options(regopt_t)
//   options = value(regopt_t).
//     each_char.sort.uniq.
//     map(&:to_sym)
// 
//   n(:regopt, options,
//     token_map(regopt_t))
// end

// def regexp_compose(begin_t, parts, end_t, options)
//   begin
//     static_regexp(parts, options)
//   rescue RegexpError => e
//     diagnostic :error, :invalid_regexp, { :message => e.message },
//                loc(begin_t).join(loc(end_t))
//   end
// 
//   n(:regexp, (parts << options),
//     regexp_map(begin_t, end_t, options))
// end

// # Arrays

// def array(begin_t, elements, end_t)
//   n(:array, elements,
//     collection_map(begin_t, elements, end_t))
// end

// def splat(star_t, arg=nil)
//   if arg.nil?
//     n0(:splat,
//       unary_op_map(star_t))
//   else
//     n(:splat, [ arg ],
//       unary_op_map(star_t, arg))
//   end
// end

// def word(parts)
//   if collapse_string_parts?(parts)
//     parts.first
//   else
//     n(:dstr, [ *parts ],
//       collection_map(nil, parts, nil))
//   end
// end

// def words_compose(begin_t, parts, end_t)
//   n(:array, [ *parts ],
//     collection_map(begin_t, parts, end_t))
// end

// def symbols_compose(begin_t, parts, end_t)
//   parts = parts.map do |part|
//     case part.type
//     when :str
//       value, = *part
//       part.updated(:sym, [ value.to_sym ])
//     when :dstr
//       part.updated(:dsym)
//     else
//       part
//     end
//   end

//   n(:array, [ *parts ],
//     collection_map(begin_t, parts, end_t))
// end

// # Hashes

// def pair(key, assoc_t, value)
//   n(:pair, [ key, value ],
//     binary_op_map(key, assoc_t, value))
// end

// def pair_list_18(list)
//   if list.size % 2 != 0
//     diagnostic :error, :odd_hash, nil, list.last.loc.expression
//   else
//     list.
//       each_slice(2).map do |key, value|
//         n(:pair, [ key, value ],
//           binary_op_map(key, nil, value))
//       end
//   end
// end

// def pair_keyword(key_t, value)
//   key_map, pair_map = pair_keyword_map(key_t, value)

//   key = n(:sym, [ value(key_t).to_sym ], key_map)

//   n(:pair, [ key, value ], pair_map)
// end

// def pair_quoted(begin_t, parts, end_t, value)
//   end_t, pair_map = pair_quoted_map(begin_t, end_t, value)

//   key = symbol_compose(begin_t, parts, end_t)

//   n(:pair, [ key, value ], pair_map)
// end

// def kwsplat(dstar_t, arg)
//   n(:kwsplat, [ arg ],
//     unary_op_map(dstar_t, arg))
// end

// def associate(begin_t, pairs, end_t)
//   n(:hash, [ *pairs ],
//     collection_map(begin_t, pairs, end_t))
// end

// # Ranges

// def range_inclusive(lhs, dot2_t, rhs)
//   n(:irange, [ lhs, rhs ],
//     binary_op_map(lhs, dot2_t, rhs))
// end

// def range_exclusive(lhs, dot3_t, rhs)
//   n(:erange, [ lhs, rhs ],
//     binary_op_map(lhs, dot3_t, rhs))
// end

// #
// # Access
// #

// def self(token)
//   n0(:self,
//     token_map(token))
// end

// def ident(token)
//   n(:ident, [ value(token).to_sym ],
//     variable_map(token))
// end

// def ivar(token)
//   n(:ivar, [ value(token).to_sym ],
//     variable_map(token))
// end

// def gvar(token)
//   n(:gvar, [ value(token).to_sym ],
//     variable_map(token))
// end

// def cvar(token)
//   n(:cvar, [ value(token).to_sym ],
//     variable_map(token))
// end

// def back_ref(token)
//   n(:back_ref, [ value(token).to_sym ],
//     token_map(token))
// end

// def nth_ref(token)
//   n(:nth_ref, [ value(token) ],
//     token_map(token))
// end

// def accessible(node)
//   case node.type
//   when :__FILE__
//     if @emit_file_line_as_literals
//       n(:str, [ node.loc.expression.source_buffer.name ],
//         node.loc.dup)
//     else
//       node
//     end
// 
//   when :__LINE__
//     if @emit_file_line_as_literals
//       n(:int, [ node.loc.expression.line ],
//         node.loc.dup)
//     else
//       node
//     end
// 
//   when :__ENCODING__
//     n(:const, [ n(:const, [ nil, :Encoding], nil), :UTF_8 ],
//       node.loc.dup)
// 
//   when :ident
//     name, = *node
// 
//     if @parser.static_env.declared?(name)
//       node.updated(:lvar)
//     else
//       name, = *node
//       n(:send, [ nil, name ],
//         var_send_map(node))
//     end
// 
//   else
//     node
//   end
// end
// TODO INCOMPLETE
pub fn accessible(node: Node) -> Node {
    println!("building node:accessible, node: {:?}", node);
    return match node {
        Node::Ident(n_ident_value) => {
            // TODO DUMMY
            // TODO handle static_env
            let node = Node::LVar(n_ident_value);
            return node;
        }
        _ => node
    };
}

// def const(name_t)
//   n(:const, [ nil, value(name_t).to_sym ],
//     constant_map(nil, nil, name_t))
// end

// def const_global(t_colon3, name_t)
//   cbase = n0(:cbase, token_map(t_colon3))

//   n(:const, [ cbase, value(name_t).to_sym ],
//     constant_map(cbase, t_colon3, name_t))
// end

// def const_fetch(scope, t_colon2, name_t)
//   n(:const, [ scope, value(name_t).to_sym ],
//     constant_map(scope, t_colon2, name_t))
// end

// def __ENCODING__(__ENCODING__t)
//   n0(:__ENCODING__,
//     token_map(__ENCODING__t))
// end

// #
// # Assignment
// #

// def assignable(node)
//   case node.type
//   when :cvar
//     node.updated(:cvasgn)
// 
//   when :ivar
//     node.updated(:ivasgn)
// 
//   when :gvar
//     node.updated(:gvasgn)
// 
//   when :const
//     if @parser.in_def?
//       diagnostic :error, :dynamic_const, nil, node.loc.expression
//     end
// 
//     node.updated(:casgn)
// 
//   when :ident
//     name, = *node
//     @parser.static_env.declare(name)
// 
//     node.updated(:lVasgn)
// 
//   when :nil, :self, :true, :false,
//        :__FILE__, :__LINE__, :__ENCODING__
//     diagnostic :error, :invalid_assignment, nil, node.loc.expression
// 
//   when :back_ref, :nth_ref
//     diagnostic :error, :backref_assignment, nil, node.loc.expression
//   end
// end
// TODO INCOMPLETE
pub fn assignable(node: Node) -> Node {
    println!("DEBUGGING node::assignable: {:?}", node);
    match node {
        Node::Ident(ident) => {
            //     name, = *node
            //     @parser.static_env.declare(name)
            // 
            //     node.updated(:lVasgn)

            // TODO handle appending nodes list
            // TODO handle static_env
            return Node::LVasgn(ident, vec![]);
        },
        _ => { panic!("node::assignable: UNIMPL branch"); }
    }
}

// def const_op_assignable(node)
//   node.updated(:casgn)
// end

// def assign(lhs, eql_t, rhs)
//   (lhs << rhs).updated(nil, nil,
//     :location => lhs.loc.
//       with_operator(loc(eql_t)).
//       with_expression(join_exprs(lhs, rhs)))
// end
// TODO INCOMPLETE
pub fn assign(lhs_node: Node, token: Token, rhs_node: Node) -> Node {
    match lhs_node {
        Node::LVasgn(var_str, mut nodes) => {
            nodes.push(rhs_node);
            return Node::LVasgn(var_str, nodes);
        },
        _ => { panic!("node::assign UNIMPL"); }
    }
}

// def op_assign(lhs, op_t, rhs)
//   case lhs.type
//   when :gvasgn, :ivasgn, :lVasgn, :cvasgn, :casgn, :send, :csend
//     operator   = value(op_t)[0..-1].to_sym
//     source_map = lhs.loc.
//                     with_operator(loc(op_t)).
//                     with_expression(join_exprs(lhs, rhs))
// 
//     case operator
//     when :'&&'
//       n(:and_asgn, [ lhs, rhs ], source_map)
//     when :'||'
//       n(:or_asgn, [ lhs, rhs ], source_map)
//     else
//       n(:op_asgn, [ lhs, operator, rhs ], source_map)
//     end
// 
//   when :back_ref, :nth_ref
//     diagnostic :error, :backref_assignment, nil, lhs.loc.expression
//   end
// end

// def multi_lhs(begin_t, items, end_t)
//   n(:mlhs, [ *items ],
//     collection_map(begin_t, items, end_t))
// end

// def multi_assign(lhs, eql_t, rhs)
//   n(:masgn, [ lhs, rhs ],
//     binary_op_map(lhs, eql_t, rhs))
// end

// #
// # Class and module definition
// #

// def def_class(class_t, name,
//               lt_t, superclass,
//               body, end_t)
//   n(:class, [ name, superclass, body ],
//     module_definition_map(class_t, name, lt_t, end_t))
// end

// def def_sclass(class_t, lshft_t, expr,
//                body, end_t)
//   n(:sclass, [ expr, body ],
//     module_definition_map(class_t, nil, lshft_t, end_t))
// end

// def def_module(module_t, name,
//                body, end_t)
//   n(:module, [ name, body ],
//     module_definition_map(module_t, name, nil, end_t))
// end

// #
// # Method (un)definition
// #

// def def_method(def_t, name_t, args,
//                body, end_t)
//   n(:def, [ value(name_t).to_sym, args, body ],
//     definition_map(def_t, nil, name_t, end_t))
// end

// def def_singleton(def_t, definee, dot_t,
//                   name_t, args,
//                   body, end_t)
//   case definee.type
//   when :int, :str, :dstr, :sym, :dsym,
//        :regexp, :array, :hash

//     diagnostic :error, :singleton_literal, nil, definee.loc.expression

//   else
//     n(:defs, [ definee, value(name_t).to_sym, args, body ],
//       definition_map(def_t, dot_t, name_t, end_t))
//   end
// end

// def undef_method(undef_t, names)
//   n(:undef, [ *names ],
//     keyword_map(undef_t, nil, names, nil))
// end

// def alias(alias_t, to, from)
//   n(:alias, [ to, from ],
//     keyword_map(alias_t, nil, [to, from], nil))
// end

// #
// # Formal arguments
// #

// def args(begin_t, args, end_t, check_args=true)
//   args = check_duplicate_args(args) if check_args
//   n(:args, args,
//     collection_map(begin_t, args, end_t))
// end

// def arg(name_t)
//   n(:arg, [ value(name_t).to_sym ],
//     variable_map(name_t))
// end

// def optarg(name_t, eql_t, value)
//   n(:optarg, [ value(name_t).to_sym, value ],
//     variable_map(name_t).
//       with_operator(loc(eql_t)).
//       with_expression(loc(name_t).join(value.loc.expression)))
// end

// def restarg(star_t, name_t=nil)
//   if name_t
//     n(:restarg, [ value(name_t).to_sym ],
//       arg_prefix_map(star_t, name_t))
//   else
//     n0(:restarg,
//       arg_prefix_map(star_t))
//   end
// end

// def kwarg(name_t)
//   n(:kwarg, [ value(name_t).to_sym ],
//     kwarg_map(name_t))
// end

// def kwoptarg(name_t, value)
//   n(:kwoptarg, [ value(name_t).to_sym, value ],
//     kwarg_map(name_t, value))
// end

// def kwrestarg(dstar_t, name_t=nil)
//   if name_t
//     n(:kwrestarg, [ value(name_t).to_sym ],
//       arg_prefix_map(dstar_t, name_t))
//   else
//     n0(:kwrestarg,
//       arg_prefix_map(dstar_t))
//   end
// end

// def shadowarg(name_t)
//   n(:shadowarg, [ value(name_t).to_sym ],
//     variable_map(name_t))
// end

// def blockarg(amper_t, name_t)
//   n(:blockarg, [ value(name_t).to_sym ],
//     arg_prefix_map(amper_t, name_t))
// end

// def procarg0(arg)
//   if self.class.emit_procarg0
//     arg.updated(:procarg0)
//   else
//     arg
//   end
// end

// # Ruby 1.8 block arguments

// def arg_expr(expr)
//   if expr.type == :lVasgn
//     expr.updated(:arg)
//   else
//     n(:arg_expr, [ expr ],
//       expr.loc.dup)
//   end
// end

// def restarg_expr(star_t, expr=nil)
//   if expr.nil?
//     n0(:restarg, token_map(star_t))
//   elsif expr.type == :lVasgn
//     expr.updated(:restarg)
//   else
//     n(:restarg_expr, [ expr ],
//       expr.loc.dup)
//   end
// end

// def blockarg_expr(amper_t, expr)
//   if expr.type == :lVasgn
//     expr.updated(:blockarg)
//   else
//     n(:blockarg_expr, [ expr ],
//       expr.loc.dup)
//   end
// end

// # MacRuby Objective-C arguments

// def objc_kwarg(kwname_t, assoc_t, name_t)
//   kwname_l = loc(kwname_t)
//   if assoc_t.nil? # a: b, not a => b
//     kwname_l   = kwname_l.resize(kwname_l.size - 1)
//     operator_l = kwname_l.end.resize(1)
//   else
//     operator_l = loc(assoc_t)
//   end

//   n(:objc_kwarg, [ value(kwname_t).to_sym, value(name_t).to_sym ],
//     Source::Map::ObjcKwarg.new(kwname_l, operator_l, loc(name_t),
//                                kwname_l.join(loc(name_t))))
// end

// def objc_restarg(star_t, name=nil)
//   if name.nil?
//     n0(:restarg, arg_prefix_map(star_t))
//   elsif name.type == :arg # regular restarg
//     name.updated(:restarg, nil,
//       { :location => name.loc.with_operator(loc(star_t)) })
//   else # restarg with objc_kwarg inside
//     n(:objc_restarg, [ name ],
//       unary_op_map(star_t, name))
//   end
// end

// #
// # Method calls
// #

// def call_type_for_dot(dot_t)
//   if !dot_t.nil? && value(dot_t) == :anddot
//     :csend
//   else
//     # This case is a bit tricky. ruby23.y returns the token tDOT with
//     # the value :dot, and the token :tANDDOT with the value :anddot.
//     #
//     # But, ruby{18..22}.y (which unconditionally expect tDOT) just
//     # return "." there, since they are to be kept close to the corresponding
//     # Ruby MRI grammars.
//     #
//     # Thankfully, we don't have to care.
//     :send
//   end
// end

// def call_method(receiver, dot_t, selector_t,
//                 lparen_t=nil, args=[], rparen_t=nil)
//   type = call_type_for_dot(dot_t)
//   if selector_t.nil?
//     n(type, [ receiver, :call, *args ],
//       send_map(receiver, dot_t, nil, lparen_t, args, rparen_t))
//   else
//     n(type, [ receiver, value(selector_t).to_sym, *args ],
//       send_map(receiver, dot_t, selector_t, lparen_t, args, rparen_t))
//   end
// end

// def call_lambda(lambda_t)
//   if self.class.emit_lambda
//     n0(:lambda, expr_map(loc(lambda_t)))
//   else
//     n(:send, [ nil, :lambda ],
//       send_map(nil, nil, lambda_t))
//   end
// end

// def block(method_call, begin_t, args, body, end_t)
//   _receiver, _selector, *call_args = *method_call

//   if method_call.type == :yield
//     diagnostic :error, :block_given_to_yield, nil, method_call.loc.keyword, [loc(begin_t)]
//   end

//   last_arg = call_args.last
//   if last_arg && last_arg.type == :block_pass
//     diagnostic :error, :block_and_blockarg, nil, last_arg.loc.expression, [loc(begin_t)]
//   end

//   if [:send, :csend, :super, :zsuper, :lambda].include?(method_call.type)
//     n(:block, [ method_call, args, body ],
//       block_map(method_call.loc.expression, begin_t, end_t))
//   else
//     # Code like "return foo 1 do end" is reduced in a weird sequence.
//     # Here, method_call is actually (return).
//     actual_send, = *method_call
//     block =
//       n(:block, [ actual_send, args, body ],
//         block_map(actual_send.loc.expression, begin_t, end_t))

//     n(method_call.type, [ block ],
//       method_call.loc.with_expression(join_exprs(method_call, block)))
//   end
// end

// def block_pass(amper_t, arg)
//   n(:block_pass, [ arg ],
//     unary_op_map(amper_t, arg))
// end

// def objc_varargs(pair, rest_of_varargs)
//   value, first_vararg = *pair
//   vararg_array = array(nil, [ first_vararg, *rest_of_varargs ], nil).
//     updated(:objc_varargs)
//   pair.updated(nil, [ value, vararg_array ],
//     { :location => pair.loc.with_expression(
//           pair.loc.expression.join(vararg_array.loc.expression)) })
// end

// def attr_asgn(receiver, dot_t, selector_t)
//   method_name = (value(selector_t) + '=').to_sym
//   type = call_type_for_dot(dot_t)

//   # Incomplete method call.
//   n(type, [ receiver, method_name ],
//     send_map(receiver, dot_t, selector_t))
// end

// def index(receiver, lbrack_t, indexes, rbrack_t)
//   n(:send, [ receiver, :[], *indexes ],
//     send_index_map(receiver, lbrack_t, rbrack_t))
// end

// def index_asgn(receiver, lbrack_t, indexes, rbrack_t)
//   # Incomplete method call.
//   n(:send, [ receiver, :[]=, *indexes ],
//     send_index_map(receiver, lbrack_t, rbrack_t))
// end

// def binary_op(receiver, operator_t, arg)
//   source_map = send_binary_op_map(receiver, operator_t, arg)

//   if @parser.version == 18
//     operator = value(operator_t)

//     if operator == '!='
//       method_call = n(:send, [ receiver, :==, arg ], source_map)
//     elsif operator == '!~'
//       method_call = n(:send, [ receiver, :=~, arg ], source_map)
//     end

//     if %w(!= !~).include?(operator)
//       return n(:not, [ method_call ],
//                expr_map(source_map.expression))
//     end
//   end

//   n(:send, [ receiver, value(operator_t).to_sym, arg ],
//     source_map)
// end

// def match_op(receiver, match_t, arg)
//   source_map = send_binary_op_map(receiver, match_t, arg)

//   if (regexp = static_regexp_node(receiver))
//     regexp.names.each do |name|
//       @parser.static_env.declare(name)
//     end

//     n(:match_with_lVasgn, [ receiver, arg ],
//       source_map)
//   else
//     n(:send, [ receiver, :=~, arg ],
//       source_map)
//   end
// end

// def unary_op(op_t, receiver)
//   case value(op_t)
//   when '+', '-'
//     method = value(op_t) + '@'
//   else
//     method = value(op_t)
//   end

//   n(:send, [ receiver, method.to_sym ],
//     send_unary_op_map(op_t, receiver))
// end

// def not_op(not_t, begin_t=nil, receiver=nil, end_t=nil)
//   if @parser.version == 18
//     n(:not, [ check_condition(receiver) ],
//       unary_op_map(not_t, receiver))
//   else
//     if receiver.nil?
//       nil_node = n0(:begin, collection_map(begin_t, nil, end_t))

//       n(:send, [
//         nil_node, :'!'
//       ], send_unary_op_map(not_t, nil_node))
//     else
//       n(:send, [ check_condition(receiver), :'!' ],
//         send_map(nil, nil, not_t, begin_t, [receiver], end_t))
//     end
//   end
// end

// #
// # Control flow
// #

// # Logical operations: and, or

// def logical_op(type, lhs, op_t, rhs)
//   n(type, [ lhs, rhs ],
//     binary_op_map(lhs, op_t, rhs))
// end

// # Conditionals

// def condition(cond_t, cond, then_t,
//               if_true, else_t, if_false, end_t)
//   n(:if, [ check_condition(cond), if_true, if_false ],
//     condition_map(cond_t, cond, then_t, if_true, else_t, if_false, end_t))
// end

// def condition_mod(if_true, if_false, cond_t, cond)
//   n(:if, [ check_condition(cond), if_true, if_false ],
//     keyword_mod_map(if_true || if_false, cond_t, cond))
// end

// def ternary(cond, question_t, if_true, colon_t, if_false)
//   n(:if, [ check_condition(cond), if_true, if_false ],
//     ternary_map(cond, question_t, if_true, colon_t, if_false))
// end

// # Case matching

// def when(when_t, patterns, then_t, body)
//   children = patterns << body
//   n(:when, children,
//     keyword_map(when_t, then_t, children, nil))
// end

// def case(case_t, expr, when_bodies, else_t, else_body, end_t)
//   n(:case, [ expr, *(when_bodies << else_body)],
//     condition_map(case_t, expr, nil, nil, else_t, else_body, end_t))
// end

// # Loops

// def loop(type, keyword_t, cond, do_t, body, end_t)
//   n(type, [ check_condition(cond), body ],
//     keyword_map(keyword_t, do_t, nil, end_t))
// end

// def loop_mod(type, body, keyword_t, cond)
//   if body.type == :kwbegin
//     type = :"#{type}_post"
//   end

//   n(type, [ check_condition(cond), body ],
//     keyword_mod_map(body, keyword_t, cond))
// end

// def for(for_t, iterator, in_t, iteratee,
//         do_t, body, end_t)
//   n(:for, [ iterator, iteratee, body ],
//     for_map(for_t, in_t, do_t, end_t))
// end

// # Keywords

// def keyword_cmd(type, keyword_t, lparen_t=nil, args=[], rparen_t=nil)
//   if type == :yield && args.count > 0
//     last_arg = args.last
//     if last_arg.type == :block_pass
//       diagnostic :error, :block_given_to_yield, nil, loc(keyword_t), [last_arg.loc.expression]
//     end
//   end

//   n(type, args,
//     keyword_map(keyword_t, lparen_t, args, rparen_t))
// end

// # BEGIN, END

// def preexe(preexe_t, lbrace_t, compstmt, rbrace_t)
//   n(:preexe, [ compstmt ],
//     keyword_map(preexe_t, lbrace_t, [], rbrace_t))
// end

// def postexe(postexe_t, lbrace_t, compstmt, rbrace_t)
//   n(:postexe, [ compstmt ],
//     keyword_map(postexe_t, lbrace_t, [], rbrace_t))
// end

// # Exception handling

// def rescue_body(rescue_t,
//                 exc_list, assoc_t, exc_var,
//                 then_t, compound_stmt)
//   n(:resbody, [ exc_list, exc_var, compound_stmt ],
//     rescue_body_map(rescue_t, exc_list, assoc_t,
//                     exc_var, then_t, compound_stmt))
// end

// def begin_body(compound_stmt, rescue_bodies=[],
//                else_t=nil,    else_=nil,
//                ensure_t=nil,  ensure_=nil)
//   if rescue_bodies.any?
//     if else_t
//       compound_stmt =
//         n(:rescue,
//           [ compound_stmt, *(rescue_bodies + [ else_ ]) ],
//           eh_keyword_map(compound_stmt, nil, rescue_bodies, else_t, else_))
//     else
//       compound_stmt =
//         n(:rescue,
//           [ compound_stmt, *(rescue_bodies + [ nil ]) ],
//           eh_keyword_map(compound_stmt, nil, rescue_bodies, nil, nil))
//     end
//   elsif else_t
//     statements = []
//     if !compound_stmt.nil?
//       if compound_stmt.type == :begin
//         statements += compound_stmt.children
//       else
//         statements.push(compound_stmt)
//       end
//     end
//     statements.push(
//       n(:begin, [ else_ ],
//         collection_map(else_t, [ else_ ], nil)))
//     compound_stmt =
//       n(:begin, statements,
//         collection_map(nil, statements, nil))
//   end

//   if ensure_t
//     compound_stmt =
//       n(:ensure,
//         [ compound_stmt, ensure_ ],
//         eh_keyword_map(compound_stmt, ensure_t, [ ensure_ ], nil, nil))
//   end

//   compound_stmt
// end

// #
// # Expression grouping
// #

// def compstmt(statements)
//   case
//   when statements.none?
//     nil
//   when statements.one?
//     statements.first
//   else
//     n(:begin, statements,
//       collection_map(nil, statements, nil))
//   end
// end
pub fn compstmt(nodes: Node) -> Node {
    if let Node::Nodes(extracted_nodes) = nodes {
        match extracted_nodes.len() {
            0 => { return Node::Null; }
            1 => { return extracted_nodes.get(0).unwrap().clone(); }
            // TODO collection_map
            _ => { return Node::Begin(extracted_nodes); }
        }
    } else {
        panic!("compstmt: should pass in a Node::Nodes")
    }
}

// def begin(begin_t, body, end_t)
//   if body.nil?
//     # A nil expression: `()'.
//     n0(:begin,
//       collection_map(begin_t, nil, end_t))
//   elsif body.type == :mlhs  ||
//        (body.type == :begin &&
//         body.loc.begin.nil? && body.loc.end.nil?)
//     # Synthesized (begin) from compstmt "a; b" or (mlhs)
//     # from multi_lhs "(a, b) = *foo".
//     n(body.type, body.children,
//       collection_map(begin_t, body.children, end_t))
//   else
//     n(:begin, [ body ],
//       collection_map(begin_t, [ body ], end_t))
//   end
// end

// def begin_keyword(begin_t, body, end_t)
//   if body.nil?
//     # A nil expression: `begin end'.
//     n0(:kwbegin,
//       collection_map(begin_t, nil, end_t))
//   elsif (body.type == :begin &&
//          body.loc.begin.nil? && body.loc.end.nil?)
//     # Synthesized (begin) from compstmt "a; b".
//     n(:kwbegin, body.children,
//       collection_map(begin_t, body.children, end_t))
//   else
//     n(:kwbegin, [ body ],
//       collection_map(begin_t, [ body ], end_t))
//   end
// end

//     #
//     # HELPERS
//     #

//     # Extract a static string from e.g. a regular expression,
//     # honoring the fact that MRI expands interpolations like #{""}
//     # at parse time.
//     def static_string(nodes)
//       nodes.map do |node|
//         case node.type
//         when :str
//           node.children[0]
//         when :begin
//           if (string = static_string(node.children))
//             string
//           else
//             return nil
//           end
//         else
//           return nil
//         end
//       end.join
//     end

//     def static_regexp(parts, options)
//       source = static_string(parts)
//       return nil if source.nil?

//       source = case
//       when options.children.include?(:u)
//         source.encode(Encoding::UTF_8)
//       when options.children.include?(:e)
//         source.encode(Encoding::EUC_JP)
//       when options.children.include?(:s)
//         source.encode(Encoding::WINDOWS_31J)
//       when options.children.include?(:n)
//         source.encode(Encoding::BINARY)
//       else
//         source
//       end

//       Regexp.new(source, (Regexp::EXTENDED if options.children.include?(:x)))
//     end

//     def static_regexp_node(node)
//       if node.type == :regexp
//         parts, options = node.children[0..-2], node.children[-1]
//         static_regexp(parts, options)
//       end
//     end

//     def collapse_string_parts?(parts)
//       parts.one? &&
//           [:str, :dstr].include?(parts.first.type)
//     end
fn is_collapse_string_parts(parts: &Node) -> bool {
    true
}

//     def value(token)
//       token[0]
//     end

//     def string_value(token)
//       unless token[0].valid_encoding?
//         diagnostic(:error, :invalid_encoding, nil, token[1])
//       end

//       token[0]
//     end

//     def loc(token)
//       # Pass through `nil`s and return nil for tNL.
//       token[1] if token && token[0]
//     end

//     def diagnostic(type, reason, arguments, location, highlights=[])
//       @parser.diagnostics.process(
//           Diagnostic.new(type, reason, arguments, location, highlights))

//       if type == :error
//         @parser.send :yyerror
//       end
//     end
//   end
