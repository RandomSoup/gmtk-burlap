use crate::parser::NodeType;

/*
// Helper for ops
macro_rules! do_op {
    ($left:expr, $right:expr, $op:tt, $errval:expr) => {
        match $left {
            // Floats
            NodeType::Float => {
                if let NodeType::Float = $right {
                    // Two floats
                    Ok(NodeType::Float, format!("lhs {} rhs", $op))
                } else if let NodeType::Int(i_right) = $right {
                    // A float and an int
                    Ok(NodeType::Float, format!("lhs {} (float) rhs", $op))
                } else {
                    $errval
                }
            },
            // Ints
            NodeType::Int(i) => {
                if let NodeType::Float(f_right) = $right {
                    // Int and float
                    Ok(NodeType::Float, format!("(float) lhs {} rhs", $op))
                } else if let NodeType::Int(i_right) = $right {
                    // Two ints
                    Ok(NodeType::Int, format!("lhs {} rhs", $op))
                } else {
                    $errval
                }
            },
            // Anything else
            _ => {
                $errval
            },
        }
    }
}

// Add
impl_op_ex!(+ |left: &NodeType, right: &NodeType| -> Result<NodeType, String> {
    // Lists
    if let NodeType::List(inner) = left {
        if let Some(vals) = right.infer(NodeType::List(inner.clone())) {
            // Concat
            for val in vals.clone() {
                list.push((list.len().to_string(), val));
            }
        } else {
            // Append
            list.push((list.len().to_string(), right.clone()));
        }
        return Ok(NodeType::List);
    } else if let NodeType::FastList(_) = left {
        let NodeType::FastList(mut list) = left.clone() else {
            panic!("impossible");
        };
        if let Some(mut vals) = right.values() {
            // Concat
            list.append(&mut vals);
        } else {
            // Append
            list.push(right.clone());
        }
        return Ok(NodeType::FastList(list));
    };
    // Strings
    if let NodeType::Str(s) = right {
        return Ok(NodeType::Str(left.to_string()? + &s));
    } else if let NodeType::Str(s) = left {
        return Ok(NodeType::Str(s.to_owned() + &right.to_string()?));
    }
    // Anything else
    return do_op!(left, right, +, Err(
        format!("Cannot add {} and {}", left.get_type(), right.get_type())
    ))
});

// Subtract
impl_op_ex!(- |left: &NodeType, right: &NodeType| -> Result<NodeType, String> {
    do_op!(left, right, -,
        Err(format!("Cannot subtract {} and {}", left.get_type(), right.get_type()))
    )
});

// Multiply
impl_op_ex!(* |left: &NodeType, right: &NodeType| -> Result<NodeType, String> {
    return match left {
        // str * number is valid
        NodeType::Str(s) => {
            if let NodeType::Int(i_right) = right {
                Ok(if *i_right > 0 {
                    NodeType::Str((*s).repeat((*i_right).try_into().unwrap()))
                } else {
                    NodeType::Str("".to_string())
                })
            } else {
                Err(format!(
                    "Cannot multiply {} and {}", left.get_type(), right.get_type()
                ))
            }
        },
        _ => do_op!(left, right, *,
            Err(format!(
                "Cannot multiply {} and {}", left.get_type(), right.get_type()
            ))
        ),
    }
});

// Div
impl_op_ex!(/ |left: &NodeType, right: &NodeType| -> Result<NodeType, String> {
    if let NodeType::Int(i) = left {
        return &NodeType::Float(*i as f32) / right;
    }
    do_op!(
        left, right, /,
        Err(format!(
            "Cannot modulo {} and {}",
            left.get_type(),
            right.get_type()
        ))
    )
});

// Modulo
impl_op_ex!(% |left: &NodeType, right: &NodeType| -> Result<NodeType, String> {
    do_op!(
        left, right, %,
        Err(format!(
            "Cannot modulo {} and {}",
            left.get_type(),
            right.get_type()
        ))
    )
});
*/