use super::{Extensions, InstFormat, OpcodeKind};

impl OpcodeKind {
    /// Get instruction format from `OpcodeKind`
    pub fn get_format(&self) -> InstFormat {
        match self {
            // Base Integer
            OpcodeKind::LUI => InstFormat::Utype,
            OpcodeKind::AUIPC => InstFormat::Utype,
            OpcodeKind::JAL => InstFormat::Jtype,
            OpcodeKind::JALR => InstFormat::Itype,
            OpcodeKind::BEQ => InstFormat::Btype,
            OpcodeKind::BNE => InstFormat::Btype,
            OpcodeKind::BLT => InstFormat::Btype,
            OpcodeKind::BGE => InstFormat::Btype,
            OpcodeKind::BLTU => InstFormat::Btype,
            OpcodeKind::BGEU => InstFormat::Btype,
            OpcodeKind::LB => InstFormat::Itype,
            OpcodeKind::LH => InstFormat::Itype,
            OpcodeKind::LW => InstFormat::Itype,
            OpcodeKind::LBU => InstFormat::Itype,
            OpcodeKind::LHU => InstFormat::Itype,
            OpcodeKind::SB => InstFormat::Stype,
            OpcodeKind::SH => InstFormat::Stype,
            OpcodeKind::SW => InstFormat::Stype,
            OpcodeKind::ADDI => InstFormat::Itype,
            OpcodeKind::SLTI => InstFormat::Itype,
            OpcodeKind::SLTIU => InstFormat::Itype,
            OpcodeKind::XORI => InstFormat::Itype,
            OpcodeKind::ORI => InstFormat::Itype,
            OpcodeKind::ANDI => InstFormat::Itype,
            OpcodeKind::SLLI => InstFormat::Rtype,
            OpcodeKind::SRLI => InstFormat::Rtype,
            OpcodeKind::SRAI => InstFormat::Rtype,
            OpcodeKind::ADD => InstFormat::Rtype,
            OpcodeKind::SUB => InstFormat::Rtype,
            OpcodeKind::SLL => InstFormat::Rtype,
            OpcodeKind::SLT => InstFormat::Rtype,
            OpcodeKind::SLTU => InstFormat::Rtype,
            OpcodeKind::XOR => InstFormat::Rtype,
            OpcodeKind::SRL => InstFormat::Rtype,
            OpcodeKind::SRA => InstFormat::Rtype,
            OpcodeKind::OR => InstFormat::Rtype,
            OpcodeKind::AND => InstFormat::Rtype,
            OpcodeKind::FENCE => InstFormat::Rtype,
            OpcodeKind::ECALL => InstFormat::Uncategorized,
            OpcodeKind::EBREAK => InstFormat::Uncategorized,
            OpcodeKind::LWU => InstFormat::Itype,
            OpcodeKind::LD => InstFormat::Itype,
            OpcodeKind::SD => InstFormat::Stype,
            OpcodeKind::ADDIW => InstFormat::Itype,
            OpcodeKind::SLLIW => InstFormat::Rtype,
            OpcodeKind::SRLIW => InstFormat::Rtype,
            OpcodeKind::SRAIW => InstFormat::Rtype,
            OpcodeKind::ADDW => InstFormat::Rtype,
            OpcodeKind::SUBW => InstFormat::Rtype,
            OpcodeKind::SLLW => InstFormat::Rtype,
            OpcodeKind::SRLW => InstFormat::Rtype,
            OpcodeKind::SRAW => InstFormat::Rtype,
            // Zicsr
            OpcodeKind::CSRRW => InstFormat::CSRtype,
            OpcodeKind::CSRRS => InstFormat::CSRtype,
            OpcodeKind::CSRRC => InstFormat::CSRtype,
            OpcodeKind::CSRRWI => InstFormat::CSRuitype,
            OpcodeKind::CSRRSI => InstFormat::CSRuitype,
            OpcodeKind::CSRRCI => InstFormat::CSRuitype,
            // Privileged
            OpcodeKind::SRET => InstFormat::Uncategorized,
            OpcodeKind::MRET => InstFormat::Uncategorized,
            OpcodeKind::WFI => InstFormat::Uncategorized,
            OpcodeKind::SFENCE_VMA => InstFormat::Uncategorized,
            // Multiplication and Division
            OpcodeKind::MUL => InstFormat::Mtype,
            OpcodeKind::MULH => InstFormat::Mtype,
            OpcodeKind::MULHSU => InstFormat::Mtype,
            OpcodeKind::MULHU => InstFormat::Mtype,
            OpcodeKind::DIV => InstFormat::Mtype,
            OpcodeKind::DIVU => InstFormat::Mtype,
            OpcodeKind::REM => InstFormat::Mtype,
            OpcodeKind::REMU => InstFormat::Mtype,
            OpcodeKind::MULW => InstFormat::Mtype,
            OpcodeKind::DIVW => InstFormat::Mtype,
            OpcodeKind::DIVUW => InstFormat::Mtype,
            OpcodeKind::REMW => InstFormat::Mtype,
            OpcodeKind::REMUW => InstFormat::Mtype,
            // Atomic
            OpcodeKind::LR_W => InstFormat::Atype,
            OpcodeKind::SC_W => InstFormat::Atype,
            OpcodeKind::AMOSWAP_W => InstFormat::Atype,
            OpcodeKind::AMOADD_W => InstFormat::Atype,
            OpcodeKind::AMOXOR_W => InstFormat::Atype,
            OpcodeKind::AMOAND_W => InstFormat::Atype,
            OpcodeKind::AMOOR_W => InstFormat::Atype,
            OpcodeKind::AMOMIN_W => InstFormat::Atype,
            OpcodeKind::AMOMAX_W => InstFormat::Atype,
            OpcodeKind::AMOMINU_W => InstFormat::Atype,
            OpcodeKind::AMOMAXU_W => InstFormat::Atype,
            OpcodeKind::LR_D => InstFormat::Atype,
            OpcodeKind::SC_D => InstFormat::Atype,
            OpcodeKind::AMOSWAP_D => InstFormat::Atype,
            OpcodeKind::AMOADD_D => InstFormat::Atype,
            OpcodeKind::AMOXOR_D => InstFormat::Atype,
            OpcodeKind::AMOAND_D => InstFormat::Atype,
            OpcodeKind::AMOOR_D => InstFormat::Atype,
            OpcodeKind::AMOMIN_D => InstFormat::Atype,
            OpcodeKind::AMOMAX_D => InstFormat::Atype,
            OpcodeKind::AMOMINU_D => InstFormat::Atype,
            OpcodeKind::AMOMAXU_D => InstFormat::Atype,
            // Compressed
            OpcodeKind::C_ADDI4SPN
            | OpcodeKind::C_LW
            | OpcodeKind::C_SW
            | OpcodeKind::C_NOP
            | OpcodeKind::C_ADDI
            | OpcodeKind::C_JAL
            | OpcodeKind::C_LI
            | OpcodeKind::C_ADDI16SP
            | OpcodeKind::C_LUI
            | OpcodeKind::C_SRLI
            | OpcodeKind::C_SRAI
            | OpcodeKind::C_ANDI
            | OpcodeKind::C_SUB
            | OpcodeKind::C_XOR
            | OpcodeKind::C_OR
            | OpcodeKind::C_AND
            | OpcodeKind::C_J
            | OpcodeKind::C_BEQZ
            | OpcodeKind::C_BNEZ
            | OpcodeKind::C_SLLI
            | OpcodeKind::C_LWSP
            | OpcodeKind::C_JR
            | OpcodeKind::C_MV
            | OpcodeKind::C_EBREAK
            | OpcodeKind::C_JALR
            | OpcodeKind::C_ADD
            | OpcodeKind::C_SWSP
            | OpcodeKind::C_LD
            | OpcodeKind::C_SD
            | OpcodeKind::C_ADDIW
            | OpcodeKind::C_SUBW
            | OpcodeKind::C_ADDW
            | OpcodeKind::C_LDSP
            | OpcodeKind::C_SDSP => InstFormat::Ctype,
        }
    }

    /// Get instruction extension from `OpcodeKind`
    pub fn get_extension(&self) -> Extensions {
        match self {
            // Base Integer
            OpcodeKind::LUI
            | OpcodeKind::AUIPC
            | OpcodeKind::JAL
            | OpcodeKind::JALR
            | OpcodeKind::BEQ
            | OpcodeKind::BNE
            | OpcodeKind::BLT
            | OpcodeKind::BGE
            | OpcodeKind::BLTU
            | OpcodeKind::BGEU
            | OpcodeKind::LB
            | OpcodeKind::LH
            | OpcodeKind::LW
            | OpcodeKind::LBU
            | OpcodeKind::LHU
            | OpcodeKind::SB
            | OpcodeKind::SH
            | OpcodeKind::SW
            | OpcodeKind::ADDI
            | OpcodeKind::SLTI
            | OpcodeKind::SLTIU
            | OpcodeKind::XORI
            | OpcodeKind::ORI
            | OpcodeKind::ANDI
            | OpcodeKind::SLLI
            | OpcodeKind::SRLI
            | OpcodeKind::SRAI
            | OpcodeKind::ADD
            | OpcodeKind::SUB
            | OpcodeKind::SLL
            | OpcodeKind::SLT
            | OpcodeKind::SLTU
            | OpcodeKind::XOR
            | OpcodeKind::SRL
            | OpcodeKind::SRA
            | OpcodeKind::OR
            | OpcodeKind::AND
            | OpcodeKind::FENCE
            | OpcodeKind::ECALL
            | OpcodeKind::EBREAK
            | OpcodeKind::LWU
            | OpcodeKind::LD
            | OpcodeKind::SD
            | OpcodeKind::ADDIW
            | OpcodeKind::SLLIW
            | OpcodeKind::SRLIW
            | OpcodeKind::SRAIW
            | OpcodeKind::ADDW
            | OpcodeKind::SUBW
            | OpcodeKind::SLLW
            | OpcodeKind::SRLW
            | OpcodeKind::SRAW => Extensions::BaseI,
            // Control and Status Register Instruction
            OpcodeKind::CSRRW
            | OpcodeKind::CSRRS
            | OpcodeKind::CSRRC
            | OpcodeKind::CSRRWI
            | OpcodeKind::CSRRSI
            | OpcodeKind::CSRRCI => Extensions::Zicsr,
            // Privileged
            OpcodeKind::SRET | OpcodeKind::MRET | OpcodeKind::WFI | OpcodeKind::SFENCE_VMA => {
                Extensions::Priv
            }
            // Multiplication and Division
            OpcodeKind::MUL
            | OpcodeKind::MULH
            | OpcodeKind::MULHSU
            | OpcodeKind::MULHU
            | OpcodeKind::DIV
            | OpcodeKind::DIVU
            | OpcodeKind::REM
            | OpcodeKind::REMU
            | OpcodeKind::MULW
            | OpcodeKind::DIVW
            | OpcodeKind::DIVUW
            | OpcodeKind::REMW
            | OpcodeKind::REMUW => Extensions::M,
            // Atomic
            OpcodeKind::LR_W
            | OpcodeKind::SC_W
            | OpcodeKind::AMOSWAP_W
            | OpcodeKind::AMOADD_W
            | OpcodeKind::AMOXOR_W
            | OpcodeKind::AMOAND_W
            | OpcodeKind::AMOOR_W
            | OpcodeKind::AMOMIN_W
            | OpcodeKind::AMOMAX_W
            | OpcodeKind::AMOMINU_W
            | OpcodeKind::AMOMAXU_W
            | OpcodeKind::LR_D
            | OpcodeKind::SC_D
            | OpcodeKind::AMOSWAP_D
            | OpcodeKind::AMOADD_D
            | OpcodeKind::AMOXOR_D
            | OpcodeKind::AMOAND_D
            | OpcodeKind::AMOOR_D
            | OpcodeKind::AMOMIN_D
            | OpcodeKind::AMOMAX_D
            | OpcodeKind::AMOMINU_D
            | OpcodeKind::AMOMAXU_D => Extensions::A,
            // Compressed
            OpcodeKind::C_ADDI4SPN
            | OpcodeKind::C_LW
            | OpcodeKind::C_SW
            | OpcodeKind::C_NOP
            | OpcodeKind::C_ADDI
            | OpcodeKind::C_JAL
            | OpcodeKind::C_LI
            | OpcodeKind::C_ADDI16SP
            | OpcodeKind::C_LUI
            | OpcodeKind::C_SRLI
            | OpcodeKind::C_SRAI
            | OpcodeKind::C_ANDI
            | OpcodeKind::C_SUB
            | OpcodeKind::C_XOR
            | OpcodeKind::C_OR
            | OpcodeKind::C_AND
            | OpcodeKind::C_J
            | OpcodeKind::C_BEQZ
            | OpcodeKind::C_BNEZ
            | OpcodeKind::C_SLLI
            | OpcodeKind::C_LWSP
            | OpcodeKind::C_JR
            | OpcodeKind::C_MV
            | OpcodeKind::C_EBREAK
            | OpcodeKind::C_JALR
            | OpcodeKind::C_ADD
            | OpcodeKind::C_SWSP
            | OpcodeKind::C_LD
            | OpcodeKind::C_SD
            | OpcodeKind::C_ADDIW
            | OpcodeKind::C_SUBW
            | OpcodeKind::C_ADDW
            | OpcodeKind::C_LDSP
            | OpcodeKind::C_SDSP => Extensions::C,
        }
    }

    /// `OpcodeKind` to string
    pub fn to_string(&self) -> &'static str {
        match self {
            OpcodeKind::LUI => "lui",
            OpcodeKind::AUIPC => "auipc",
            OpcodeKind::JAL => "jal",
            OpcodeKind::JALR => "jalr",
            OpcodeKind::BEQ => "beq",
            OpcodeKind::BNE => "bne",
            OpcodeKind::BLT => "blt",
            OpcodeKind::BGE => "bge",
            OpcodeKind::BLTU => "bltu",
            OpcodeKind::BGEU => "bgeu",
            OpcodeKind::LB => "lb",
            OpcodeKind::LH => "lh",
            OpcodeKind::LW => "lw",
            OpcodeKind::LBU => "lbu",
            OpcodeKind::LHU => "lhu",
            OpcodeKind::SB => "sb",
            OpcodeKind::SH => "sh",
            OpcodeKind::SW => "sw",
            OpcodeKind::ADDI => "addi",
            OpcodeKind::SLTI => "slti",
            OpcodeKind::SLTIU => "sltiu",
            OpcodeKind::XORI => "xori",
            OpcodeKind::ORI => "ori",
            OpcodeKind::ANDI => "andi",
            OpcodeKind::SLLI => "slli",
            OpcodeKind::SRLI => "srli",
            OpcodeKind::SRAI => "srai",
            OpcodeKind::ADD => "add",
            OpcodeKind::SUB => "sub",
            OpcodeKind::SLL => "sll",
            OpcodeKind::SLT => "slt",
            OpcodeKind::SLTU => "sltu",
            OpcodeKind::XOR => "xor",
            OpcodeKind::SRL => "srl",
            OpcodeKind::SRA => "sra",
            OpcodeKind::OR => "or",
            OpcodeKind::AND => "and",
            OpcodeKind::FENCE => "fence",
            OpcodeKind::ECALL => "ecall",
            OpcodeKind::EBREAK => "ebreak",
            OpcodeKind::LWU => "lwu",
            OpcodeKind::LD => "ld",
            OpcodeKind::SD => "sd",
            OpcodeKind::ADDIW => "addiw",
            OpcodeKind::SLLIW => "slliw",
            OpcodeKind::SRLIW => "srliw",
            OpcodeKind::SRAIW => "sraiw",
            OpcodeKind::ADDW => "addw",
            OpcodeKind::SUBW => "subw",
            OpcodeKind::SLLW => "sllw",
            OpcodeKind::SRLW => "srlw",
            OpcodeKind::SRAW => "sraw",
            OpcodeKind::CSRRW => "csrrw",
            OpcodeKind::CSRRS => "csrrs",
            OpcodeKind::CSRRC => "csrrc",
            OpcodeKind::CSRRWI => "csrrwi",
            OpcodeKind::CSRRSI => "csrrsi",
            OpcodeKind::CSRRCI => "csrrci",
            OpcodeKind::SRET => "sret",
            OpcodeKind::MRET => "mret",
            OpcodeKind::WFI => "wfi",
            OpcodeKind::SFENCE_VMA => "sfence.vma",
            OpcodeKind::MUL => "mul",
            OpcodeKind::MULH => "mulh",
            OpcodeKind::MULHSU => "mulhsu,",
            OpcodeKind::MULHU => "mulhu",
            OpcodeKind::DIV => "div",
            OpcodeKind::DIVU => "divu",
            OpcodeKind::REM => "rem",
            OpcodeKind::REMU => "remu",
            OpcodeKind::MULW => "mulw",
            OpcodeKind::DIVW => "divw",
            OpcodeKind::DIVUW => "divuw",
            OpcodeKind::REMW => "remw",
            OpcodeKind::REMUW => "remuw",
            OpcodeKind::LR_W => "lr.w",
            OpcodeKind::SC_W => "sc.w",
            OpcodeKind::AMOSWAP_W => "amoswap.w",
            OpcodeKind::AMOADD_W => "amoadd.w",
            OpcodeKind::AMOXOR_W => "amoxor.w",
            OpcodeKind::AMOAND_W => "amoand.w",
            OpcodeKind::AMOOR_W => "amoor.w",
            OpcodeKind::AMOMIN_W => "amomin.w",
            OpcodeKind::AMOMAX_W => "amomax.w",
            OpcodeKind::AMOMINU_W => "amominu.w",
            OpcodeKind::AMOMAXU_W => "amomaxu.w",
            OpcodeKind::LR_D => "lr.d",
            OpcodeKind::SC_D => "sc.d",
            OpcodeKind::AMOSWAP_D => "amoswap.d",
            OpcodeKind::AMOADD_D => "amoadd.d",
            OpcodeKind::AMOXOR_D => "amoxor.d",
            OpcodeKind::AMOAND_D => "amoand.d",
            OpcodeKind::AMOOR_D => "amoor.d",
            OpcodeKind::AMOMIN_D => "amomin.d",
            OpcodeKind::AMOMAX_D => "amomax.d",
            OpcodeKind::AMOMINU_D => "amominu.d",
            OpcodeKind::AMOMAXU_D => "amomaxu.d",
            OpcodeKind::C_ADDI4SPN => "C.addi4spn",
            OpcodeKind::C_LW => "C.lw",
            OpcodeKind::C_SW => "C.sw",
            OpcodeKind::C_NOP => "C.nop",
            OpcodeKind::C_ADDI => "C.addi",
            OpcodeKind::C_JAL => "C.jal",
            OpcodeKind::C_LI => "C.li",
            OpcodeKind::C_ADDI16SP => "C.addi16sp",
            OpcodeKind::C_LUI => "C.lui",
            OpcodeKind::C_SRLI => "C.srli",
            OpcodeKind::C_SRAI => "C.srai",
            OpcodeKind::C_ANDI => "C.andi",
            OpcodeKind::C_SUB => "C.sub",
            OpcodeKind::C_XOR => "C.xor",
            OpcodeKind::C_OR => "C.or",
            OpcodeKind::C_AND => "C.and",
            OpcodeKind::C_J => "C.j",
            OpcodeKind::C_BEQZ => "C.beqz",
            OpcodeKind::C_BNEZ => "C.bnez",
            OpcodeKind::C_SLLI => "C.slli",
            OpcodeKind::C_LWSP => "C.lwsp",
            OpcodeKind::C_JR => "C.jr",
            OpcodeKind::C_MV => "C.mv",
            OpcodeKind::C_EBREAK => "C.ebreak",
            OpcodeKind::C_JALR => "C.jalr",
            OpcodeKind::C_ADD => "C.add",
            OpcodeKind::C_SWSP => "C.swsp",
            OpcodeKind::C_LD => "C.ld",
            OpcodeKind::C_SD => "C.sd",
            OpcodeKind::C_ADDIW => "C.addiw",
            OpcodeKind::C_SUBW => "C.subw",
            OpcodeKind::C_ADDW => "C.addw",
            OpcodeKind::C_LDSP => "C.ldsp",
            OpcodeKind::C_SDSP => "C.sdsp",
        }
    }
}
