use std::cmp::Ordering;
use std::path::PathBuf;

use crate::Arguments;
use crate::common::IMPOSSIBLE_STATE;
use crate::lexer::TokenType;
use crate::parser::{ASTNode, ASTNode::*};
use crate::value::Value;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        return Compiler{};
    }
}

fn compile_unary(
    compiler: &mut Compiler,
    op: &TokenType, val: &Box<ASTNode>
) -> bool {
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
    return true;
}

fn compile_binop(
    compiler: &mut Compiler,
    lhs: &Box<ASTNode>, op: &TokenType, rhs: &Box<ASTNode>
) -> bool {
    true
}

fn compile_expr(compiler: &mut Compiler, node: &ASTNode) -> bool {
    match node {
        // Values
        VarExpr(val) => {
        },
        StringExpr(val) => {
        },
        NumberExpr(val) => {
        },
        DecimalExpr(val) => {
        },
        BoolExpr(val) => {
        },
        NoneExpr => {
        },
        ByteExpr(val) => {
        },
        // Binop/unary
        BinopExpr(lhs, op, rhs) => {
            return compile_binop(compiler, lhs, op, rhs);
        }
        UnaryExpr(op, val) => {
            return compile_unary(compiler, op, val);
        },
        // Calls
        CallExpr(expr, args) => {
        },
        // List
        ListExpr(keys, values, fast) => {
        },
        // Indexes
        IndexExpr(val, index) => {
        },
        _ => {
            panic!("Unknown token! {:?}", node);
        }
    };
    return true;
}

fn _compile_body(
    compiler: &mut Compiler, nodes: &Vec<ASTNode>
) -> bool {
    // Compile all nodes
    for node in nodes {
        if !compile_stmt(compiler, node) {
            // Pass it down
            return false;
        }
    }
    return true;
}
fn compile_body(
    compiler: &mut Compiler, node: &ASTNode
) -> bool {
    let BodyStmt(nodes) = node else {
        if *node == Nop {
            return true;
        }
        panic!("compile_body got non-body node!");
    };
    return _compile_body(compiler, nodes);
}

fn compile_stmt(
    compiler: &mut Compiler, node: &ASTNode
) -> bool {
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
    return true;
}

pub fn compile(ast: Vec<ASTNode>) -> bool {
    if ast.is_empty() {
        return true;
    }
    // Compile
    let mut compiler = Compiler::new();
    for node in &ast {
        if !compile_stmt(&mut compiler, node) {
            return false;
        }
    }
    return true;
}
