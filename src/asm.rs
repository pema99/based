use crate::compile::*;
use crate::ast::*;
use std::collections::HashMap;

pub fn print_asm(blob: Blob) {
    let mut rev_label_map = HashMap::new();
    for (label, val) in blob.labels {
        rev_label_map.insert(val, label);
    }

    for (idx, instr) in blob.instructions.iter().enumerate() {
        if rev_label_map.contains_key(&idx) {
            println!("{}:", rev_label_map.get(&idx).unwrap());
        }
        print!("\t");
        print_instr(instr);
    }
}

fn print_instr(instr: &Instr) {
    match instr {
        Instr::BinOp(lhs, op, rhs) => match op {
            Op::Add => println!("addq {}, {}", translate_loc(lhs), translate_loc(rhs)),
            Op::Sub => println!("subq {}, {}", translate_loc(lhs), translate_loc(rhs)),
            Op::Mul => println!("mulq {}, {}", translate_loc(lhs), translate_loc(rhs)),
            Op::Div => println!("divq {}, {}", translate_loc(lhs), translate_loc(rhs)),
        }, 
        Instr::UnOp(op, rhs) => {},
        
        Instr::Mov(lhs, rhs) => println!("movq {}, {}", translate_loc(lhs), translate_loc(rhs)),
        
        Instr::Cmp(lhs, rhs) => println!("cmp {}, {}", translate_loc(lhs), translate_loc(rhs)),
    
        Instr::Jmp(loc) => println!("jmp {}", translate_loc(loc)),
        Instr::Jez(loc) => println!("jez {}", translate_loc(loc)),
        Instr::Call(loc) => println!("call {}", translate_loc(loc)),
        Instr::Ret => println!("ret"),
    }
}

fn translate_loc(loc: &Loc) -> String {
    match loc {
        Loc::Imm(v) => format!("${}", v),
        Loc::Reg(v) => format!("%{}", v),
        Loc::RegDeref(v) => format!("(%{})", v),
        Loc::AddrStack(v) => format!("OADDR"),
        Loc::Label(v) => format!("{}", v)
    }
}