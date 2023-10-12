//! Define instructions data structure.

mod opcode;

use std::fmt::{self, Display, Formatter};

/// Instruction
#[derive(Debug)]
pub struct Instruction {
    /// Opcode
    pub opc: OpcodeKind,
    /// Register Destination
    pub rd: Option<usize>,
    /// Register Source 1
    pub rs1: Option<usize>,
    /// Register Source 2
    pub rs2: Option<usize>,
    /// Immediate
    pub imm: Option<i32>,
    /// Instruction extension
    pub extension: Extensions,
    /// Instruction format
    pub inst_format: InstFormat,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.inst_format {
            InstFormat::Rtype | InstFormat::Mtype | InstFormat::Atype => write!(
                f,
                "{} {}, {}, {}",
                self.opc.to_string(),
                reg2str(self.rd.unwrap()),
                reg2str(self.rs1.unwrap()),
                reg2str(self.rs2.unwrap())
            ),
            InstFormat::Alrtype | InstFormat::Itype => write!(
                f,
                "{} {}, {}, {:#x}",
                self.opc.to_string(),
                reg2str(self.rd.unwrap()),
                reg2str(self.rs1.unwrap()),
                self.imm.unwrap()
            ),
            InstFormat::Stype | InstFormat::Btype => write!(
                f,
                "{} {}, {}, {:#x}({})",
                self.opc.to_string(),
                reg2str(self.rd.unwrap()),
                reg2str(self.rs1.unwrap()),
                self.imm.unwrap(),
                reg2str(self.rs2.unwrap()),
            ),
            InstFormat::Utype | InstFormat::Jtype => {
                write!(
                    f,
                    "{} {}, {:#x}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::CSRtype => {
                write!(
                    f,
                    "{} {}, {:#x}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    self.rs2.unwrap(),
                    reg2str(self.rs1.unwrap()),
                )
            }
            InstFormat::CSRuitype => {
                write!(
                    f,
                    "{} {}, {:#x}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    self.rs2.unwrap(),
                    self.imm.unwrap(),
                )
            }
            InstFormat::Ctype | InstFormat::Uncategorized => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    self.opc.to_string(),
                    match self.rd {
                        Some(rd) => format!(" {}", reg2str(rd)),
                        None => String::new(),
                    },
                    match self.rs1 {
                        Some(rs1) => format!(" {rs1}"),
                        None => String::new(),
                    },
                    match self.rs2 {
                        Some(rs2) => format!(" {rs2}"),
                        None => String::new(),
                    },
                    match self.imm {
                        Some(imm) => format!(" {imm}"),
                        None => String::new(),
                    },
                )
            }
        }
    }
}

fn reg2str(rd_value: usize) -> &'static str {
    match rd_value {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",
        5 => "t0",
        6 => "t1",
        7 => "t2",
        8 => "s0", // fp
        9 => "s1",
        10 => "a0",
        11 => "a1",
        12 => "a2",
        13 => "a3",
        14 => "a4",
        15 => "a5",
        16 => "a6",
        17 => "a7",
        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "s8",
        25 => "s9",
        26 => "s10",
        27 => "s11",
        28 => "t3",
        29 => "t4",
        30 => "t5",
        31 => "t6",
        _ => panic!("unknown register"),
    }
}

/// RISC-V extensions
#[derive(Debug)]
pub enum Extensions {
    /// Base Integer Instruction Set
    BaseI,
    /// Integer Multiplication and Division
    M,
    /// Atomic Instructions
    A,
    /// Compressed Instructions
    C,
    /// Control and Status Register Instructions
    Zicsr,
    /// Privileged Instructions
    Priv,
}

/// Instruction format
#[derive(Debug)]
pub enum InstFormat {
    /// Regular format
    Rtype,
    /// Immediate format
    Itype,
    /// Store format
    Stype,
    /// Branch format
    Btype,
    /// Upper immediate format
    Utype,
    /// Jump format
    Jtype,
    /// Compressed instruction
    /// C-type may possibly be further divided (Q0, Q1, Q2...).
    Ctype,
    /// Csr format
    CSRtype,
    /// Csr with uimm format
    CSRuitype,
    /// M-extension instruction format
    Mtype,
    /// A-extension instruction format
    Atype,
    /// lr.w instruction format in A-extension
    Alrtype,
    /// Uncategorized format
    Uncategorized,
}

/// Opcode
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpcodeKind {
    //== Base Integer Instruction ==
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    FENCE,
    ECALL,
    EBREAK,
    //-- rv64 --
    LWU,
    LD,
    SD,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,

    //== Zicsr Extension ==
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,

    //== privileged Instruction ==
    SRET,
    MRET,
    WFI,
    SFENCE_VMA,

    //== M Extension ==
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,
    //-- rv64 --
    MULW,
    DIVW,
    DIVUW,
    REMW,
    REMUW,

    //== A Extension ==
    LR_W,
    SC_W,
    AMOSWAP_W,
    AMOADD_W,
    AMOXOR_W,
    AMOAND_W,
    AMOOR_W,
    AMOMIN_W,
    AMOMAX_W,
    AMOMINU_W,
    AMOMAXU_W,
    //-- rv64 --
    LR_D,
    SC_D,
    AMOSWAP_D,
    AMOADD_D,
    AMOXOR_D,
    AMOAND_D,
    AMOOR_D,
    AMOMIN_D,
    AMOMAX_D,
    AMOMINU_D,
    AMOMAXU_D,

    //== C Extension ==
    C_ADDI4SPN,
    C_LW,
    C_SW,
    C_NOP,
    C_ADDI,
    C_JAL,
    C_LI,
    C_ADDI16SP,
    C_LUI,
    C_SRLI,
    C_SRAI,
    C_ANDI,
    C_SUB,
    C_XOR,
    C_OR,
    C_AND,
    C_J,
    C_BEQZ,
    C_BNEZ,
    C_SLLI,
    C_LWSP,
    C_JR,
    C_MV,
    C_EBREAK,
    C_JALR,
    C_ADD,
    C_SWSP,
    //-- rv64 --
    C_LD,
    C_SD,
    C_ADDIW,
    C_SUBW,
    C_ADDW,
    C_LDSP,
    C_SDSP,
}
