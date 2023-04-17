use std::collections::HashMap;

use crate::common::IMPOSSIBLE_STATE;
use crate::lexer::TokenType;
use crate::parser::{ASTNode, ASTNode::*};
use crate::value::Value;
use crate::vm::Opcode;

#[derive(Debug)]
pub struct Program {
    // Opcodes and constants
    pub ops: Vec<u8>,
    pub consts: Vec<Value>,
    // Function locations (name : (byte pos, arg num))
    pub functis: HashMap<String, (usize, i32)>,
    // Line numbers (run-length encoded)
    pub lines: Vec<usize>,
}

impl Program {
    // Init
    pub fn new() -> Program {
        Program{
            ops: Vec::new(), consts: Vec::new(),
            functis: HashMap::new(), lines: Vec::new()
        }
    }

    pub fn write_num(&mut self, val: u32, ops: u8, start: usize) {
        for i in 0..ops {
            self.ops[start + i as usize] = ((val >> (i * 8)) & 255) as u8;
        }
    }

    pub fn push(&mut self, val: Value) {
        // Get the index, or append
        let index = self.consts.iter().position(|i| i.clone() == val)
            .unwrap_or_else(|| {
            self.consts.push(val);
            self.consts.len() - 1
        });
        // Push the instruction
        if index > 2usize.pow(24)-1 {
            panic!("Too many different constants!");
        } else if index > u8::MAX.into() {
            // The len is too big for one byte, so push 3
            self.ops.push(Opcode::PUSH3 as u8);
            self.ops.push((index & 255) as u8);
            self.ops.push(((index >> 8) & 255) as u8);
            self.ops.push(((index >> 16) & 255) as u8);
        } else {
            self.ops.push(Opcode::PUSH as u8);
            self.ops.push(index as u8);
        }
    }
}

fn compile_unary(
    program: &mut Program,
    op: &TokenType, val: &Box<ASTNode>
) -> bool {
    match op {
        // -/!
        TokenType::Minus => {
            program.push(Value::Int(0));
            if !compile_expr(program, val) {
                return false;
            }
            program.ops.push(Opcode::SUB as u8);
        },
        TokenType::Not => {
            if !compile_expr(program, val) {
                return false;
            }
            program.ops.push(Opcode::NOT as u8);
        },
        // ++/--
        TokenType::PlusPlus => {
            if !compile_expr(program, val) {
                return false;
            }
            program.push(Value::Int(1));
            program.ops.push(Opcode::ADD as u8);
            program.ops.push(Opcode::DUP as u8);
            if let VarExpr(s) = *val.clone() {
                program.push(Value::Str(s));
                program.ops.push(Opcode::SV as u8);
            }
        },
        TokenType::MinusMinus => {
            if !compile_expr(program, val) {
                return false;
            }
            program.push(Value::Int(1));
            program.ops.push(Opcode::SUB as u8);
            program.ops.push(Opcode::DUP as u8);
            if let VarExpr(s) = *val.clone() {
                program.push(Value::Str(s));
                program.ops.push(Opcode::SV as u8);
            }
        },
        _ => panic!("{}", IMPOSSIBLE_STATE),
    }
    return true;
}

fn compile_binop(
    program: &mut Program,
    lhs: &Box<ASTNode>, op: &TokenType, rhs: &Box<ASTNode>,
    clean: bool
) -> bool {
    // Compile sides
    if let TokenType::Equals = op {} else {
        // No need to compile the value if it will just be reassigned
        if !compile_expr(program, lhs) {
            return false;
        }
    }
    if !compile_expr(program, rhs) {
        return false;
    }
    // Compile op
    match op {
        // Simple single instructions
        TokenType::Plus | TokenType::PlusEquals => {
            program.ops.push(Opcode::ADD as u8);
        },
        TokenType::Minus | TokenType::MinusEquals => {
            program.ops.push(Opcode::SUB as u8);
        },
        TokenType::Times | TokenType::TimesEquals => {
            program.ops.push(Opcode::MUL as u8);
        },
        TokenType::Div | TokenType::DivEquals => {
            program.ops.push(Opcode::DIV as u8);
        },
        TokenType::Modulo => {
            program.ops.push(Opcode::MOD as u8);
        },
        TokenType::And => {
            program.ops.push(Opcode::AND as u8);
        },
        TokenType::Or => {
            program.ops.push(Opcode::OR as u8);
        },
        TokenType::Xor => {
            program.ops.push(Opcode::XOR as u8);
        },
        TokenType::Gt => {
            program.ops.push(Opcode::GT as u8);
        },
        TokenType::Lt => {
            program.ops.push(Opcode::LT as u8);
        },
        TokenType::EqualsEquals => {
            program.ops.push(Opcode::EQ as u8);
        },
        // Harder ones that don't have a single instruction
        TokenType::NotEquals => {
            program.ops.push(Opcode::EQ as u8);
            program.ops.push(Opcode::NOT as u8);
        },
        TokenType::LtEquals => {
            program.ops.push(Opcode::GT as u8);
            program.ops.push(Opcode::NOT as u8);
        },
        TokenType::GtEquals => {
            program.ops.push(Opcode::LT as u8);
            program.ops.push(Opcode::NOT as u8);
        },
        // Handled later
        TokenType::Equals => {},
        _ => panic!("That operator isn't implemented!"),
    };
    // Set the variable
    if let TokenType::PlusEquals | TokenType::MinusEquals
        | TokenType::TimesEquals | TokenType::DivEquals
        | TokenType::Equals = op.clone()
    {
        if let VarExpr(s) = *lhs.clone() {
            program.push(Value::Str(s));
            program.ops.push(Opcode::SV as u8);
        }
    } else if clean {
        // Clean up the stack
        program.ops.push(Opcode::DEL as u8);
    }
    return true;
}

fn compile_expr(program: &mut Program, node: &ASTNode) -> bool {
    match node {
        // Values
        VarExpr(val) => {
            program.push(Value::Str(val.clone()));
            program.ops.push(Opcode::PV as u8);
        },
        StringExpr(val) => {
            program.push(Value::Str(val.clone()));
        },
        NumberExpr(val) => {
            program.push(Value::Int(*val));
        },
        DecimalExpr(val) => {
            program.push(Value::Float(*val));
        },
        BoolExpr(val) => {
            program.push(Value::Bool(*val));
        },
        NoneExpr => {
            program.push(Value::None);
        },
        // Binop/unary
        BinopExpr(lhs, op, rhs) => {
            return compile_binop(program, lhs, op, rhs, false);
        }
        UnaryExpr(op, val) => {
            return compile_unary(program, op, val);
        },
        // Calls
        CallExpr(name, args) => {
            // Push the call. Name, arg count, args (in reverse order)
            for arg in args {
                if !compile_expr(program, arg) {
                    return false;
                }
            }
            program.push(Value::Int(args.len() as i32));
            program.push(Value::Str(name.clone()));
            program.ops.push(Opcode::CALL as u8);
        },
        // List
        ListExpr(keys, values) => {
            // Build the list
            let mut at = values.len();
            while at > 0 {
                at -= 1;
                if !compile_expr(program, &values[at]) {
                    return false;
                }
                program.push(Value::Str(keys[at].clone()));
            }
            // Push
            program.push(Value::Int(values.len() as i32));
            program.ops.push(Opcode::LL as u8);
        },
        // Indexes
        IndexExpr(val, index) => {
            // Push
            if !compile_expr(program, val) {
                return false;
            }
            if !compile_expr(program, index) {
                return false;
            }
            program.ops.push(Opcode::INX as u8);
        },
        // TODO
        ImportStmt(_) => {},
        _ => {
            panic!("Unknown token! {:?}", node);
        }
    };
    return true;
}

fn _compile_body(program: &mut Program, nodes: &Vec<ASTNode>) -> bool {
    // Compile all nodes
    for node in nodes {
        if !compile_stmt(program, node) {
            // Pass it down
            return false;
        }
    }
    return true;
}
fn compile_body(program: &mut Program, node: &ASTNode) -> bool {
    let BodyStmt(nodes) = node else {
        if *node == Nop {
            return true;
        }
        panic!("compile_body got non-body node!");
    };
    return _compile_body(program, nodes);
}

fn compile_stmt(program: &mut Program, node: &ASTNode) -> bool {
    match node {
        // Statements
        LetStmt(name, val) => {
            compile_expr(program, val);
            program.push(Value::Str(name.to_string()));
            program.ops.push(Opcode::DV as u8);
        },
        IfStmt(cond, body, else_part) => {
            // The condition must be a expr, so no need to match against stmts
            compile_expr(program, cond);

            // This is for when boolean not is forgotten
            if **body == Nop {
                program.ops.push(Opcode::NOT as u8);
                // Push the jump offset (which will be filled later)
                program.ops.push(Opcode::JMPNT as u8);
                program.ops.push(0);
                // Compile body
                let pos = program.ops.len();
                compile_stmt(program, else_part);
                program.ops[pos - 1] = (program.ops.len() - pos + 1)
                    as u8;
                return true
            }

            // Push the jump offset (which will be filled later)
            program.ops.push(Opcode::JMPNT as u8);
            program.ops.push(0);
            let pos = program.ops.len();
            // Compile true part
            compile_body(program, body);
            program.ops[pos - 1] = (program.ops.len() - pos + 1) as u8;

            // The else
            if **else_part != Nop {
                // Prep exit offset
                program.ops[pos - 1] += 2;
                program.ops.push(Opcode::JMPU as u8);
                program.ops.push(0);
                let pos = program.ops.len();
                // Compile else part
                compile_stmt(program, else_part);
                program.ops[pos - 1] = (program.ops.len() - pos + 1) as u8;
            }
        },
        LoopStmt(var, iter, body) => {
            // Load iter
            compile_expr(program, iter);
            program.ops.push(Opcode::TITR as u8);
            let pos = program.ops.len();
            program.ops.push(Opcode::NXT as u8);

            // Exit jump
            program.ops.push(Opcode::JMPNT as u8);
            program.ops.push(0);
            let offpos = program.ops.len();

            // Set the loop var
            program.push(Value::Str(var.to_string()));
            program.ops.push(Opcode::DV as u8);

            // Body
            compile_body(program, body);

            // Backwards jump
            program.ops.push(Opcode::JMPB as u8);
            program.ops.push((program.ops.len() - pos) as u8);
            // Clean up the iter
            program.ops.push(Opcode::DEL as u8);
            program.ops[offpos - 1] = (program.ops.len() - offpos) as u8;
        },
        WhileStmt(cond, body) => {
            // Start, exit jump + cond
            let pos = program.ops.len();
            compile_expr(program, cond);
            program.ops.push(Opcode::JMPNT as u8);
            program.ops.push(0);
            let offpos = program.ops.len();

            // Compile body
            compile_body(program, body);

            // Backwards jump
            program.ops.push(Opcode::JMPB as u8);
            program.ops.push((program.ops.len() - pos) as u8);
            program.ops[offpos - 1] = (program.ops.len() - offpos + 1) as u8;
        },
        BodyStmt(nodes) => return _compile_body(program, nodes),
        FunctiStmt(name, args, body) => {
            // Declare function
            program.push(Value::Int(args.len() as i32));
            program.push(Value::Str(name.to_string()));
            program.ops.push(Opcode::FN as u8);
            // Jump around function
            program.ops.push(Opcode::JMPU as u8);
            program.ops.push(0);
            let pos = program.ops.len();
            // Load args
            for arg in args {
                program.push(Value::Str(arg.to_string()));
                program.ops.push(Opcode::DV as u8);
            }
            // Compile body
            compile_body(program, body);
            // Return
            program.push(Value::None);
            program.ops.push(Opcode::RET as u8);
            // Fill jump
            program.ops[pos - 1] = (program.ops.len() - pos + 1) as u8;
        },
        ReturnStmt(ret) => {
            // Compile return value
            compile_expr(program, ret);
            // Return return value
            program.ops.push(Opcode::RET as u8);
        }
        Nop => {
            // Nop isn't turned into the NOP instruction because it's useless
        },
        // Expressions
        // Binops don't always return, so let them clean up the stack themselves
        BinopExpr(lhs, op, rhs) => {
            return compile_binop(program, lhs, op, rhs, true);
        }
        _ => {
            let ret = compile_expr(program, node);
            // Remove unused values from the stack
            program.ops.push(Opcode::DEL as u8);
            return ret;
        }
    };
    return true;
}

pub fn compile(ast: Vec<ASTNode>) -> Option<Program> {
    let mut program = Program::new();
    // Compile
    for node in ast {
        if !compile_stmt(&mut program, &node) {
            return None;
        }
    }
    // Jumps go onto the next instruction, so a nop is needed at the end
    program.ops.push(Opcode::NOP as u8);
    //println!("CC: {:?}", program);
    return Some(program);
}
