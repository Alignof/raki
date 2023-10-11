use super::{Extensions, InstFormat, OpcodeKind};

impl OpcodeKind {
    /// Get instruction format from `OpcodeKind`
    pub fn get_format(&self) -> InstFormat {
        match self {
            /*
            // Base Integer
            OpcodeKind::LUI => InstFormat::Uncategorized,
            OpcodeKind::AUIPC => InstFormat::Uncategorized,
            OpcodeKind::JAL => InstFormat::Uncategorized,
            OpcodeKind::JALR => InstFormat::Uncategorized,
            OpcodeKind::BEQ => InstFormat::Uncategorized,
            OpcodeKind::BNE => InstFormat::Uncategorized,
            OpcodeKind::BLT => InstFormat::Uncategorized,
            OpcodeKind::BGE => InstFormat::Uncategorized,
            OpcodeKind::BLTU => InstFormat::Uncategorized,
            OpcodeKind::BGEU => InstFormat::Uncategorized,
            OpcodeKind::LB => InstFormat::Uncategorized,
            OpcodeKind::LH => InstFormat::Uncategorized,
            OpcodeKind::LW => InstFormat::Uncategorized,
            OpcodeKind::LBU => InstFormat::Uncategorized,
            OpcodeKind::LHU => InstFormat::Uncategorized,
            OpcodeKind::SB => InstFormat::Uncategorized,
            OpcodeKind::SH => InstFormat::Uncategorized,
            OpcodeKind::SW => InstFormat::Uncategorized,
            OpcodeKind::ADDI => InstFormat::Uncategorized,
            OpcodeKind::SLTI => InstFormat::Uncategorized,
            OpcodeKind::SLTIU => InstFormat::Uncategorized,
            OpcodeKind::XORI => InstFormat::Uncategorized,
            OpcodeKind::ORI => InstFormat::Uncategorized,
            OpcodeKind::ANDI => InstFormat::Uncategorized,
            OpcodeKind::SLLI => InstFormat::Uncategorized,
            OpcodeKind::SRLI => InstFormat::Uncategorized,
            OpcodeKind::SRAI => InstFormat::Uncategorized,
            OpcodeKind::ADD => InstFormat::Uncategorized,
            OpcodeKind::SUB => InstFormat::Uncategorized,
            OpcodeKind::SLL => InstFormat::Uncategorized,
            OpcodeKind::SLT => InstFormat::Uncategorized,
            OpcodeKind::SLTU => InstFormat::Uncategorized,
            OpcodeKind::XOR => InstFormat::Uncategorized,
            OpcodeKind::SRL => InstFormat::Uncategorized,
            OpcodeKind::SRA => InstFormat::Uncategorized,
            OpcodeKind::OR => InstFormat::Uncategorized,
            OpcodeKind::AND => InstFormat::Uncategorized,
            OpcodeKind::FENCE => InstFormat::Uncategorized,
            OpcodeKind::ECALL => InstFormat::Uncategorized,
            OpcodeKind::EBREAK => InstFormat::Uncategorized,
            OpcodeKind::LWU => InstFormat::Uncategorized,
            OpcodeKind::LD => InstFormat::Uncategorized,
            OpcodeKind::SD => InstFormat::Uncategorized,
            OpcodeKind::ADDIW => InstFormat::Uncategorized,
            OpcodeKind::SLLIW => InstFormat::Uncategorized,
            OpcodeKind::SRLIW => InstFormat::Uncategorized,
            OpcodeKind::SRAIW => InstFormat::Uncategorized,
            OpcodeKind::ADDW => InstFormat::Uncategorized,
            OpcodeKind::SUBW => InstFormat::Uncategorized,
            OpcodeKind::SLLW => InstFormat::Uncategorized,
            OpcodeKind::SRLW => InstFormat::Uncategorized,
            OpcodeKind::SRAW => InstFormat::Uncategorized,
            OpcodeKind::CSRRW => InstFormat::Uncategorized,
            OpcodeKind::CSRRS => InstFormat::Uncategorized,
            OpcodeKind::CSRRC => InstFormat::Uncategorized,
            OpcodeKind::CSRRWI => InstFormat::Uncategorized,
            OpcodeKind::CSRRSI => InstFormat::Uncategorized,
            OpcodeKind::CSRRCI => InstFormat::Uncategorized,
            // Privileged
            OpcodeKind::SRET => InstFormat::Uncategorized,
            OpcodeKind::MRET => InstFormat::Uncategorized,
            OpcodeKind::WFI => InstFormat::Uncategorized,
            OpcodeKind::SFENCE_VMA => InstFormat::Uncategorized,
            // Multiplication and Division
            OpcodeKind::MUL => InstFormat::Uncategorized,
            OpcodeKind::MULH => InstFormat::Uncategorized,
            OpcodeKind::MULHSU => InstFormat::Uncategorized,
            OpcodeKind::MULHU => InstFormat::Uncategorized,
            OpcodeKind::DIV => InstFormat::Uncategorized,
            OpcodeKind::DIVU => InstFormat::Uncategorized,
            OpcodeKind::REM => InstFormat::Uncategorized,
            OpcodeKind::REMU => InstFormat::Uncategorized,
            OpcodeKind::MULW => InstFormat::Uncategorized,
            OpcodeKind::DIVW => InstFormat::Uncategorized,
            OpcodeKind::DIVUW => InstFormat::Uncategorized,
            OpcodeKind::REMW => InstFormat::Uncategorized,
            OpcodeKind::REMUW => InstFormat::Uncategorized,
            // Atomic
            OpcodeKind::LR_W => InstFormat::Uncategorized,
            OpcodeKind::SC_W => InstFormat::Uncategorized,
            OpcodeKind::AMOSWAP_W => InstFormat::Uncategorized,
            OpcodeKind::AMOADD_W => InstFormat::Uncategorized,
            OpcodeKind::AMOXOR_W => InstFormat::Uncategorized,
            OpcodeKind::AMOAND_W => InstFormat::Uncategorized,
            OpcodeKind::AMOOR_W => InstFormat::Uncategorized,
            OpcodeKind::AMOMIN_W => InstFormat::Uncategorized,
            OpcodeKind::AMOMAX_W => InstFormat::Uncategorized,
            OpcodeKind::AMOMINU_W => InstFormat::Uncategorized,
            OpcodeKind::AMOMAXU_W => InstFormat::Uncategorized,
            OpcodeKind::LR_D => InstFormat::Uncategorized,
            OpcodeKind::SC_D => InstFormat::Uncategorized,
            OpcodeKind::AMOSWAP_D => InstFormat::Uncategorized,
            OpcodeKind::AMOADD_D => InstFormat::Uncategorized,
            OpcodeKind::AMOXOR_D => InstFormat::Uncategorized,
            OpcodeKind::AMOAND_D => InstFormat::Uncategorized,
            OpcodeKind::AMOOR_D => InstFormat::Uncategorized,
            OpcodeKind::AMOMIN_D => InstFormat::Uncategorized,
            OpcodeKind::AMOMAX_D => InstFormat::Uncategorized,
            OpcodeKind::AMOMINU_D => InstFormat::Uncategorized,
            OpcodeKind::AMOMAXU_D => InstFormat::Uncategorized,
            // Compressed
            OpcodeKind::C_ADDI4SPN => InstFormat::Uncategorized,
            OpcodeKind::C_LW => InstFormat::Uncategorized,
            OpcodeKind::C_SW => InstFormat::Uncategorized,
            OpcodeKind::C_NOP => InstFormat::Uncategorized,
            OpcodeKind::C_ADDI => InstFormat::Uncategorized,
            OpcodeKind::C_JAL => InstFormat::Uncategorized,
            OpcodeKind::C_LI => InstFormat::Uncategorized,
            OpcodeKind::C_ADDI16SP => InstFormat::Uncategorized,
            OpcodeKind::C_LUI => InstFormat::Uncategorized,
            OpcodeKind::C_SRLI => InstFormat::Uncategorized,
            OpcodeKind::C_SRAI => InstFormat::Uncategorized,
            OpcodeKind::C_ANDI => InstFormat::Uncategorized,
            OpcodeKind::C_SUB => InstFormat::Uncategorized,
            OpcodeKind::C_XOR => InstFormat::Uncategorized,
            OpcodeKind::C_OR => InstFormat::Uncategorized,
            OpcodeKind::C_AND => InstFormat::Uncategorized,
            OpcodeKind::C_J => InstFormat::Uncategorized,
            OpcodeKind::C_BEQZ => InstFormat::Uncategorized,
            OpcodeKind::C_BNEZ => InstFormat::Uncategorized,
            OpcodeKind::C_SLLI => InstFormat::Uncategorized,
            OpcodeKind::C_LWSP => InstFormat::Uncategorized,
            OpcodeKind::C_JR => InstFormat::Uncategorized,
            OpcodeKind::C_MV => InstFormat::Uncategorized,
            OpcodeKind::C_EBREAK => InstFormat::Uncategorized,
            OpcodeKind::C_JALR => InstFormat::Uncategorized,
            OpcodeKind::C_ADD => InstFormat::Uncategorized,
            OpcodeKind::C_SWSP => InstFormat::Uncategorized,
            OpcodeKind::C_LD => InstFormat::Uncategorized,
            OpcodeKind::C_SD => InstFormat::Uncategorized,
            OpcodeKind::C_ADDIW => InstFormat::Uncategorized,
            OpcodeKind::C_SUBW => InstFormat::Uncategorized,
            OpcodeKind::C_ADDW => InstFormat::Uncategorized,
            OpcodeKind::C_LDSP => InstFormat::Uncategorized,
            OpcodeKind::C_SDSP => InstFormat::Uncategorized,
            */
            _ => InstFormat::Uncategorized,
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
