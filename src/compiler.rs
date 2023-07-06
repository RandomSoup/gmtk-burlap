#![allow(unused_imports, unused_variables, dead_code)]
use std::cmp::Ordering;
use std::path::PathBuf;

use crate::Arguments;
use crate::common::IMPOSSIBLE_STATE;
use crate::lexer::TokenType;
use crate::parser::{ASTNode, ASTNode::*};
//use crate::value::Value;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        return Compiler{};
    }
}

fn compile_unary(
    compiler: &mut Compiler,
    op: &TokenType, val: &Box<ASTNode>
) -> Option<String> {
    match op {
        // -/!
        TokenType::Minus => {
        },
        TokenType::Not => {
        },
        // ++/--
        TokenType::PlusPlus => {
        },
        TokenType::MinusMinus => {
        },
        _ => panic!("{}", IMPOSSIBLE_STATE),
    }
    return None;
}

fn compile_binop(
    compiler: &mut Compiler,
    lhs: &Box<ASTNode>, op: &TokenType, rhs: &Box<ASTNode>
) -> Option<String> {
    None
}

fn op_to_char(op: TokenType) -> char {
    match op {
        TokenType::Times => '*',
        TokenType::Plus => '+',
        TokenType::Minus => '-',
        TokenType::Div => '/',
        TokenType::Mod => '%',
        _ => panic!("inconceivable!"),
    }
}

fn compile_expr(compiler: &mut Compiler, node: &ASTNode) -> Option<String> {
    Some(match node {
        // Values
        VarExpr(val) => {
            format!("{}", val)
        },
        StringExpr(val) => {
            format!("std::string(\"{}\")", val)
        },
        NumberExpr(val) => {
            format!("{}", val)
        },
        DecimalExpr(val) => {
            format!("{}", val)
        },
        BoolExpr(val) => {
            format!("{}", val)
        },
        NoneExpr => {
            format!("None{{}}")
        },
        ByteExpr(val) => {
            format!("(char) {}", val)
        },
        // Binop/unary
        BinopExpr(lhs, op, rhs) => {
            let ret = "(".to_owned() + &compile_expr(compiler, lhs)?;
            ret += " " + op_to_char(op) + " ";
            ret += compile_expr(compiler, rhs)? + ")";
            ret
        }
        UnaryExpr(op, val) => {
            return compile_unary(compiler, op, val);
        },
        // Calls
        //CallExpr(expr, args) => {
        //},
        // List
        //ListExpr(keys, values, fast) => {
        //},
        // Indexes
        //IndexExpr(val, index) => {
        //},
        _ => {
            panic!("Unknown token! {:?}", node);
        }
    })
}

fn _compile_body(
    compiler: &mut Compiler, nodes: &Vec<ASTNode>
) -> Option<String> {
    // Compile all nodes
    let mut ret = "".to_string();
    for node in nodes {
        ret += &compile_stmt(compiler, node)?;
    }
    return Some(ret);
}
fn compile_body(
    compiler: &mut Compiler, node: &ASTNode
) -> Option<String> {
    let BodyStmt(nodes) = node else {
        if *node == Nop {
            return Some("{}".to_string());
        }
        panic!("compile_body got non-body node!");
    };
    return _compile_body(compiler, nodes);
}

fn compile_stmt(
    compiler: &mut Compiler, node: &ASTNode
) -> Option<String> {
    match node {
        // Statements
        LetStmt(name, ltype, val) => {
        },
        IfStmt(cond, body, else_part) => {
        },
        LoopStmt(var, iter, body) => {
        },
        WhileStmt(cond, body) => {
        },
        BodyStmt(nodes) => return _compile_body(compiler, nodes),
        FunctiStmt(name, fargs, rettype, body) => {
        },
        ForwardStmt(name, fargs, rettype) => {
        },
        ReturnStmt(ret) => {
        },
        ImportStmt() => {
        },
        EndImportStmt(file) => {
        },

        Nop => {},
        // Expressions
        _ => return compile_expr(compiler, node)
    };
    return None;
}

pub fn compile(ast: Vec<ASTNode>) -> Option<String> {
    if ast.is_empty() {
        return Some("".to_string());
    }
    // Compile
    let mut compiler = Compiler::new();
    let mut ret = "".to_string();
    for node in &ast {
        ret += &compile_stmt(&mut compiler, node)?;
    }
    return Some(ret);
}
