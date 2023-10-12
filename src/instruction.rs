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
            InstFormat::C_Q1_Rtype
            | InstFormat::C_Q2_Rtype
            | InstFormat::Rtype
            | InstFormat::Mtype
            | InstFormat::Atype => {
                write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                    reg2str(self.rs2.unwrap())
                )
            }
            InstFormat::A_LRtype
            | InstFormat::Itype
            | InstFormat::C_Q0_Itype
            | InstFormat::C_Q1_Itype
            | InstFormat::C_Q2_Itype => write!(
                f,
                "{} {}, {}, {}",
                self.opc.to_string(),
                reg2str(self.rd.unwrap()),
                reg2str(self.rs1.unwrap()),
                self.imm.unwrap()
            ),
            InstFormat::Stype | InstFormat::C_Stype | InstFormat::Btype => write!(
                f,
                "{} {}, {:#x}({})",
                self.opc.to_string(),
                reg2str(self.rs1.unwrap()),
                self.imm.unwrap(),
                reg2str(self.rs2.unwrap()),
            ),
            InstFormat::Utype | InstFormat::Jtype | InstFormat::C_Utype => {
                write!(
                    f,
                    "{} {}, {:#x}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::C_Q1_Jtype | InstFormat::C_Q1_NoRDtype => {
                write!(f, "{} ({:#x})", self.opc.to_string(), self.imm.unwrap())
            }
            InstFormat::C_Q2_Jtype => {
                write!(f, "{} ({:#x})", self.opc.to_string(), self.rs1.unwrap())
            }
            InstFormat::C_Btype => {
                write!(
                    f,
                    "{} {}, {}",
                    self.opc.to_string(),
                    self.rs1.unwrap(),
                    self.imm.unwrap(),
                )
            }
            InstFormat::C_Q2_SPtype => {
                write!(
                    f,
                    "{} {:#x}, {}",
                    self.opc.to_string(),
                    self.rs2.unwrap(),
                    reg2str(self.rs1.unwrap()),
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
                    "{} {}, {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    self.rs2.unwrap(),
                    self.imm.unwrap(),
                )
            }
            InstFormat::Uncategorized => {
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
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum InstFormat {
    /// Regular format
    /// ```ignore
    /// add rd, rs1, rs2
    /// ```
    Rtype,

    /// Immediate format
    /// ```ignore
    /// lw rd, imm(rs1)
    /// ```
    Itype,

    /// Store format
    /// ```ignore
    /// sw rs2, imm(rs1)
    /// ```
    Stype,

    /// Branch format
    /// ```ignore
    /// beq rs1, rs2, imm
    /// ```
    Btype,

    /// Upper immediate format
    /// ```ignore
    /// lui rd, imm
    /// ```
    Utype,

    /// Jump format
    /// ```ignore
    /// jal rd, imm
    /// ```
    Jtype,

    /// Compressed Immediate format in Quadrant 0
    /// ```ignore
    /// c.addi4spn rd, nzuimm
    /// ```
    C_Q0_Itype,

    /// Compressed Store format
    /// ```ignore
    /// c.lw rd, imm(rs1)
    /// ```
    C_Stype,

    /// Compressed Immediate format in Quadrant 1
    /// ```ignore
    /// c.addi rd, rd, nzuimm
    /// ```
    C_Q1_Itype,

    /// Compressed Jump format in Quadrant 1
    /// ```ignore
    /// c.j imm
    /// ```
    C_Q1_Jtype,

    /// Compressed instruction does not have rd in Quadrant 1
    /// ```ignore
    /// c.addi16sp nzuimm
    /// ```
    C_Q1_NoRDtype,

    /// Compressed Upper immediate format (Quadrant 1)
    /// ```ignore
    /// c.lui rd, nzuimm
    /// ```
    C_Utype,

    /// Compressed Regular format in Quadrant 1
    /// ```ignore
    /// c.and rd, rd, rs2
    /// ```
    C_Q1_Rtype,

    /// Compressed Branch format (Quadrant 1)
    /// ```ignore
    /// c.beqz rs1, imm
    /// ```
    C_Btype,

    /// Compressed Immediate format in Quadrant 2
    /// ```ignore
    /// c.slli rd, rd, nzuimm
    /// ```
    C_Q2_Itype,

    /// Compressed Jump format in Quadrant 2
    /// ```ignore
    /// c.jr rs1
    /// ```
    C_Q2_Jtype,

    /// Compressed load/store stack pointer instruction in Quadrant 2
    /// ```ignore
    /// c.swsp rs2, uimm
    /// ```
    C_Q2_SPtype,

    /// Compressed Regular format in Quadrant 2
    /// ```ignore
    /// c.add rd, rd, rs2
    /// c.mv rd, rs2
    /// ```
    C_Q2_Rtype,

    /// Compressed Csr format
    /// ```ignore
    /// csrrw rd, csr, rs1
    /// ```
    CSRtype,

    /// Csr with uimm format
    /// ```ignore
    /// csrrwi rd, csr, imm
    /// ```
    CSRuitype,

    /// M-extension instruction format
    /// ```ignore
    /// mul rd, rs1, rs2
    /// ```
    Mtype,

    /// A-extension instruction format
    /// ```ignore
    /// sc.w rd, rs2, (rs1)
    /// ```
    Atype,

    /// lr.w instruction format in A-extension
    /// ```ignore
    /// lr.w rd, (rs1)
    /// ```
    A_LRtype,

    /// Uncategorized format
    /// ```ignore
    /// ecall
    /// wfi
    /// mret
    /// c.ebreak
    /// ```
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
