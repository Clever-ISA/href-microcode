#[derive(Clone, Debug, Hash, PartialEq, Eq)]

pub struct Instruction {
    pub opcode: u16,
    pub decode: Vec<DecodeOp>,
    pub deps: Vec<DepOp>,
    pub execute: Vec<ExecuteOp>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum DecodeOp {
    Operands(u32),
    HValidMask(u8),
    Prefix(u16),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
    ReadAddr,
    ReadWrite,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ExecutionUnit {
    Alu,
    Fpu,
    IoBus,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum DepOp {
    Operand(u32, Access),
    Reg(u8, Access),
    Regh(Access),
    Mem(Access),
    Xu(ExecutionUnit, u8),
    Serialize {
        regs: bool,
        addr: bool,
        instr: bool,
        mem: bool,
    },
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ExecuteOp {
    Rxc(u8),
    Lda { op: u8, input: u8, unit: u8 },
    Ldas { op: u8, unit: u8 },
    Ldash { mask: u8, shift: u8, disp: u8 },
    Xalu { func: u8, unit: u8 },
    Xalu2 { func: u8, unit: u8, vnit: u8 },
    Sta { op: u8, output: u8, unit: u8 },
    Staf { pos: u8, unit: u8, invmask: u8 },
    Stafr { reg: u8, unit: u8 },
    Lxa { op: u8 },
    Uxa { op: u8 },
    Wfl { flags: u8 },
    Cma { op: u8, input: u8, unit: u8 },
    Rdreg { reg: u8, input: u8, unit: u8 },
    Wrreg { reg: u8, output: u8, unit: u8 },
    Wrip { output: u8, unit: u8 },
    Wrmd { output: u8, unit: u8 },
}
