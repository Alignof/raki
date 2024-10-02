//! Define instructions data structure.

pub mod a_extension;
pub mod base_i;
pub mod c_extension;
pub mod m_extension;
pub mod priv_extension;
pub mod zicfiss_extension;
pub mod zicntr_extension;
pub mod zicsr_extension;
pub mod zifencei_extension;

use core::fmt::{self, Display, Formatter};

use a_extension::AOpcode;
use base_i::BaseIOpcode;
use c_extension::COpcode;
use m_extension::MOpcode;
use priv_extension::PrivOpcode;
use zicfiss_extension::ZicfissOpcode;
use zicntr_extension::ZicntrOpcode;
use zicsr_extension::ZicsrOpcode;
use zifencei_extension::ZifenceiOpcode;

/// Instruction
#[derive(Debug, PartialEq)]
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
    /// Instruction format
    pub inst_format: InstFormat,
}

impl Display for Instruction {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.inst_format {
            InstFormat::RFormat | InstFormat::MFormat => {
                write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                    reg2str(self.rs2.unwrap())
                )
            }
            InstFormat::AFormat => match self.opc {
                OpcodeKind::A(AOpcode::LR_W | AOpcode::LR_D) => write!(
                    f,
                    "{} {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                ),
                _ => write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                    reg2str(self.rs2.unwrap())
                ),
            },
            InstFormat::RShamtFormat => {
                write!(
                    f,
                    "{} {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs1.unwrap()),
                )
            }
            InstFormat::ClFormat | InstFormat::ALrFormat | InstFormat::IFormat => write!(
                f,
                "{} {}, {}, {}",
                self.opc,
                reg2str(self.rd.unwrap()),
                reg2str(self.rs1.unwrap()),
                self.imm.unwrap()
            ),
            InstFormat::CsFormat | InstFormat::SFormat | InstFormat::BFormat => write!(
                f,
                "{} {}, {}({})",
                self.opc,
                reg2str(self.rs1.unwrap()),
                self.imm.unwrap(),
                reg2str(self.rs2.unwrap()),
            ),
            InstFormat::CiwFormat => {
                write!(
                    f,
                    "{} {}, sp, {:x}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::CssFormat => {
                write!(
                    f,
                    "{} {}, {}(sp)",
                    self.opc,
                    reg2str(self.rs2.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::UFormat | InstFormat::JFormat => {
                write!(
                    f,
                    "{} {}, {:#x}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::CjFormat => {
                write!(f, "{} {}", self.opc, self.imm.unwrap())
            }
            InstFormat::CiFormat => {
                write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rd.unwrap()),
                    self.imm.unwrap()
                )
            }
            InstFormat::CrFormat => match self.opc {
                OpcodeKind::C(COpcode::JR) => {
                    write!(f, "{} zero, 0({})", self.opc, reg2str(self.rs1.unwrap()),)
                }
                OpcodeKind::C(COpcode::JALR) => {
                    write!(f, "{} ra, 0({})", self.opc, reg2str(self.rs1.unwrap()),)
                }
                OpcodeKind::C(COpcode::MV) => write!(
                    f,
                    "{} {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs2.unwrap())
                ),
                OpcodeKind::C(COpcode::ADD) => write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs2.unwrap())
                ),
                _ => unreachable!(),
            },
            InstFormat::CaFormat => {
                write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rd.unwrap()),
                    reg2str(self.rs2.unwrap())
                )
            }
            InstFormat::CbFormat => {
                write!(
                    f,
                    "{} {}, {}",
                    self.opc,
                    self.rs1.unwrap(),
                    self.imm.unwrap(),
                )
            }
            InstFormat::CsrFormat => {
                write!(
                    f,
                    "{} {}, {:#x}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    self.rs2.unwrap(),
                    reg2str(self.rs1.unwrap()),
                )
            }
            InstFormat::CsrUiFormat => {
                write!(
                    f,
                    "{} {}, {}, {}",
                    self.opc,
                    reg2str(self.rd.unwrap()),
                    self.rs2.unwrap(),
                    self.imm.unwrap(),
                )
            }
            InstFormat::OnlyRd => {
                write!(f, "{} {}", self.opc, reg2str(self.rd.unwrap()),)
            }
            InstFormat::OnlyRs1 => {
                write!(f, "{} {}", self.opc, reg2str(self.rs1.unwrap()),)
            }
            InstFormat::OnlyRs2 => {
                write!(f, "{} {}", self.opc, reg2str(self.rs2.unwrap()),)
            }
            InstFormat::NoOperand => match self.opc {
                OpcodeKind::BaseI(BaseIOpcode::ECALL | BaseIOpcode::EBREAK)
                | OpcodeKind::Zifencei(ZifenceiOpcode::FENCE)
                | OpcodeKind::C(COpcode::NOP | COpcode::EBREAK)
                | OpcodeKind::Priv(
                    PrivOpcode::MRET | PrivOpcode::SRET | PrivOpcode::WFI | PrivOpcode::SFENCE_VMA,
                ) => write!(f, "{}", self.opc),
                _ => unreachable!(),
            },
        }
    }
}

/// Convert register number to string.
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

/// Instruction format
/// See: [The RISC-V Instruction Set Manual: Volume II Version 20240411](https://github.com/riscv/riscv-isa-manual/releases/download/20240411/priv-isa-asciidoc.pdf) p.23,141
#[derive(Debug, PartialEq)]
pub enum InstFormat {
    /// Regular format
    /// ```ignore
    /// add rd, rs1, rs2
    /// ```
    RFormat,

    /// Regular format with shamt
    /// ```ignore
    /// srai rd, rs1
    /// ```
    RShamtFormat,

    /// Immediate format
    /// ```ignore
    /// lw rd, imm(rs1)
    /// ```
    IFormat,

    /// Store format
    /// ```ignore
    /// sw rs2, imm(rs1)
    /// ```
    SFormat,

    /// Branch format
    /// ```ignore
    /// beq rs1, rs2, imm
    /// ```
    BFormat,

    /// Upper immediate format
    /// ```ignore
    /// lui rd, imm
    /// ```
    UFormat,

    /// Jump format
    /// ```ignore
    /// jal rd, imm
    /// ```
    JFormat,

    /// Compressed Register format
    /// ```ignore
    /// c.mv rd, rs2
    /// c.add rd, rd, rs2
    /// ```
    CrFormat,

    /// Compressed Immediate format
    /// ```ignore
    /// c.nop
    /// c.addi rd, rd, imm
    /// c.addi16sp x2, x2, nzimm
    /// ```
    CiFormat,

    /// Compressed Stack-relative Store format
    /// ```ignore
    /// c.swsp rs2, imm
    /// -> sw rs2, imm[8:3](x2)
    /// ```
    CssFormat,

    /// Compressed Wide Immediate Store format
    /// ```ignore
    /// c.addi4spn rd, x2, nzuimm
    /// ```
    CiwFormat,

    /// Compressed Load format
    /// ```ignore
    /// c.lw rd imm(rs1)
    /// ```
    ClFormat,

    /// Compressed Store format
    /// ```ignore
    /// c.sw rs2, imm(rs1)
    /// ```
    CsFormat,

    /// Compressed Arithmetic format
    /// ```ignore
    /// c.and rd, rd, rs2
    /// ```
    CaFormat,

    /// Compressed Branch format
    /// ```ignore
    /// c.beqz rs1, imm
    /// c.srai rd, rd, shamt
    /// ```
    CbFormat,

    /// Compressed Jump format
    /// ```ignore
    /// c.j imm
    /// ```
    CjFormat,

    /// Compressed Csr format
    /// ```ignore
    /// csrrw rd, csr, rs1
    /// csrrwi rd, csr, imm
    /// ```
    CsrFormat,

    /// Csr with uimm format
    /// ```ignore
    /// csrrwi rd, csr, imm
    /// ```
    CsrUiFormat,

    /// M-extension instruction format
    /// ```ignore
    /// mul rd, rs1, rs2
    /// ```
    MFormat,

    /// A-extension instruction format
    /// ```ignore
    /// sc.w rd, rs2, (rs1)
    /// ```
    AFormat,

    /// lr.w instruction format in A-extension
    /// ```ignore
    /// lr.w rd, (rs1)
    /// ```
    ALrFormat,

    /// No operand
    /// ```ignore
    /// ecall
    /// wfi
    /// mret
    /// c.ebreak
    /// ```
    NoOperand,

    /// Only rd name
    /// ```ignore
    /// rdtime rd
    /// ssrdp rd
    /// ```
    OnlyRd,

    /// Only rs1 name
    /// ```ignore
    /// sspush ra
    /// ```
    OnlyRs1,

    /// Only rs2 name
    /// ```ignore
    /// sspopchk t0
    /// ```
    OnlyRs2,
}

/// Trait for `OpcodeKind`
pub trait Opcode {
    /// Get Instruction format (e.g. R-type, I-type, S-type, etc...)
    /// See: Chapter 34. RV32/64G Instruction Set Listings, p.554
    fn get_format(&self) -> InstFormat;
}

/// Extension type and Instruction name.
#[derive(Debug, PartialEq)]
pub enum OpcodeKind {
    /// Base Integer Instruction Set
    BaseI(BaseIOpcode),
    /// Integer Multiplication and Division
    M(MOpcode),
    /// Atomic Instructions
    A(AOpcode),
    /// Compressed Instructions
    C(COpcode),
    /// Instruction-Fetch Fence,
    Zifencei(ZifenceiOpcode),
    /// Control and Status Register Instructions
    Zicsr(ZicsrOpcode),
    /// CFI Shadow Stack
    Zicfiss(ZicfissOpcode),
    /// Base Counters and Timers
    Zicntr(ZicntrOpcode),
    /// Privileged Instructions
    Priv(PrivOpcode),
}

impl Display for OpcodeKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::BaseI(opc) => write!(f, "{opc}"),
            Self::M(opc) => write!(f, "{opc}"),
            Self::A(opc) => write!(f, "{opc}"),
            Self::C(opc) => write!(f, "{opc}"),
            Self::Zifencei(opc) => write!(f, "{opc}"),
            Self::Zicsr(opc) => write!(f, "{opc}"),
            Self::Zicfiss(opc) => write!(f, "{opc}"),
            Self::Zicntr(opc) => write!(f, "{opc}"),
            Self::Priv(opc) => write!(f, "{opc}"),
        }
    }
}

impl OpcodeKind {
    #[must_use]
    pub fn get_format(&self) -> InstFormat {
        match &self {
            Self::BaseI(opc) => opc.get_format(),
            Self::M(opc) => opc.get_format(),
            Self::A(opc) => opc.get_format(),
            Self::C(opc) => opc.get_format(),
            Self::Zifencei(opc) => opc.get_format(),
            Self::Zicsr(opc) => opc.get_format(),
            Self::Zicfiss(opc) => opc.get_format(),
            Self::Zicntr(opc) => opc.get_format(),
            Self::Priv(opc) => opc.get_format(),
        }
    }
}
