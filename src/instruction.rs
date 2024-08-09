//! Define instructions data structure.

mod a_extension;
mod base_i;
mod c_extension;
mod m_extension;
mod opcode;
mod priv_extension;
mod zicsr_extension;

use std::fmt::{self, Display, Formatter};

use a_extension::AOpcode;
use base_i::BaseIOpcode;
use c_extension::COpcode;
use m_extension::MOpcode;
use priv_extension::PrivOpcode;
use zicsr_extension::ZicsrOpcode;

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
            InstFormat::Rformat | InstFormat::Mformat => {
                write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                    reg2str(self.rs2.unwrap())
                )
            }
            InstFormat::Aformat => match self.opc {
                OpcodeKind::LR_W | OpcodeKind::LR_D => write!(
                    f,
                    "{} {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                ),
                _ => write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                    reg2str(self.rs2.unwrap())
                ),
            },
            InstFormat::R_SHAMTformat => {
                write!(
                    f,
                    "{} {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                )
            }
            InstFormat::CLformat | InstFormat::A_LRformat | InstFormat::Iformat => write!(
                f,
                "{} {}, {}, {}",
                self.opc.to_string(),
                reg2str(self.rd.unwrap()),
                reg2str(self.rs1.unwrap()),
                self.imm.unwrap()
            ),
            InstFormat::CSformat | InstFormat::Sformat | InstFormat::Bformat => write!(
                f,
                "{} {}, {}({})",
                self.opc.to_string(),
                reg2str(self.rs1.unwrap()),
                self.imm.unwrap(),
                reg2str(self.rs2.unwrap()),
            ),
            InstFormat::CIWformat => {
                write!(
                    f,
                    "{} {}, sp, {:x}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::CSSformat => {
                write!(
                    f,
                    "{} {}, {}(sp)",
                    self.opc.to_string(),
                    reg2str(self.rs2.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::Uformat | InstFormat::Jformat => {
                write!(
                    f,
                    "{} {}, {:#x}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::CJformat => {
                write!(f, "{} {}", self.opc.to_string(), self.imm.unwrap())
            }
            InstFormat::CIformat => {
                write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rd.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::CRformat => match self.opc {
                OpcodeKind::C_JR => {
                    write!(
                        f,
                        "{} zero, 0({})",
                        self.opc.to_string(),
                        reg2str(self.rs1.unwrap()),
                    )
                }
                OpcodeKind::C_JALR => {
                    write!(
                        f,
                        "{} ra, 0({})",
                        self.opc.to_string(),
                        reg2str(self.rs1.unwrap()),
                    )
                }
                OpcodeKind::C_MV => write!(
                    f,
                    "{} {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs2.unwrap())
                ),
                OpcodeKind::C_ADD => write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs2.unwrap())
                ),
                _ => unreachable!(),
            },
            InstFormat::CAformat => {
                write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs2.unwrap())
                )
            }
            InstFormat::CBformat => {
                write!(
                    f,
                    "{} {}, {}",
                    self.opc.to_string(),
                    self.rs1.unwrap(),
                    self.imm.unwrap(),
                )
            }
            InstFormat::CSRformat => {
                write!(
                    f,
                    "{} {}, {:#x}, {}",
                    self.opc.to_string(),
                    reg2str(self.rd.unwrap()),
                    self.rs2.unwrap(),
                    reg2str(self.rs1.unwrap()),
                )
            }
            InstFormat::CSRuiformat => {
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
    Rformat,

    /// Regular format with shamt
    /// ```ignore
    /// srai rd, rs1
    /// ```
    R_SHAMTformat,

    /// Immediate format
    /// ```ignore
    /// lw rd, imm(rs1)
    /// ```
    Iformat,

    /// Store format
    /// ```ignore
    /// sw rs2, imm(rs1)
    /// ```
    Sformat,

    /// Branch format
    /// ```ignore
    /// beq rs1, rs2, imm
    /// ```
    Bformat,

    /// Upper immediate format
    /// ```ignore
    /// lui rd, imm
    /// ```
    Uformat,

    /// Jump format
    /// ```ignore
    /// jal rd, imm
    /// ```
    Jformat,

    /// Compressed Register format
    /// ```ignore
    /// c.mv rd, rs2
    /// c.add rd, rd, rs2
    /// ```
    CRformat,

    /// Compressed Immediate format
    /// ```ignore
    /// c.nop
    /// c.addi rd, rd, imm
    /// c.addi16sp x2, x2, nzimm
    /// ```
    CIformat,

    /// Compressed Stack-relative Store format
    /// ```ignore
    /// c.swsp rs2, imm
    /// -> sw rs2, imm[8:3](x2)
    /// ```
    CSSformat,

    /// Compressed Wide Immediate Store format
    /// ```ignore
    /// c.addi4spn rd, x2, nzuimm
    /// ```
    CIWformat,

    /// Compressed Load format
    /// ```ignore
    /// c.lw rd imm(rs1)
    /// ```
    CLformat,

    /// Compressed Store format
    /// ```ignore
    /// c.sw rs2, imm(rs1)
    /// ```
    CSformat,

    /// Compressed Arithmetic format
    /// ```ignore
    /// c.and rd, rd, rs2
    /// ```
    CAformat,

    /// Compressed Branch format
    /// ```ignore
    /// c.beqz rs1, imm
    /// c.srai rd, rd, shamt
    /// ```
    CBformat,

    /// Compressed Jump format
    /// ```ignore
    /// c.j imm
    /// ```
    CJformat,

    /// Compressed Csr format
    /// ```ignore
    /// csrrw rd, csr, rs1
    /// csrrwi rd, csr, imm
    /// ```
    CSRformat,

    /// Csr with uimm format
    /// ```ignore
    /// csrrwi rd, csr, imm
    /// ```
    CSRuiformat,

    /// M-extension instruction format
    /// ```ignore
    /// mul rd, rs1, rs2
    /// ```
    Mformat,

    /// A-extension instruction format
    /// ```ignore
    /// sc.w rd, rs2, (rs1)
    /// ```
    Aformat,

    /// lr.w instruction format in A-extension
    /// ```ignore
    /// lr.w rd, (rs1)
    /// ```
    A_LRformat,

    /// Uncategorized format
    /// ```ignore
    /// ecall
    /// wfi
    /// mret
    /// c.ebreak
    /// ```
    Uncategorized,
}

/// Trait for `OpcodeKind`
trait Opcode<Fmt> {
    fn get_format(&self) -> InstFormat;
}

/// Opcode
pub enum NewOpcodeKind {
    /// Base Integer Instruction Set
    BaseI(BaseIOpcode),
    /// Integer Multiplication and Division
    M(MOpcode),
    /// Atomic Instructions
    A(AOpcode),
    /// Compressed Instructions
    C(COpcode),
    /// Control and Status Register Instructions
    Zicsr(ZicsrOpcode),
    /// Privileged Instructions
    Priv(PrivOpcode),
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
