use std::array;

use static_init::dynamic;

use crate::utils::extend_sign;

#[derive(Debug)]
pub(crate) struct CInstructor {
  pub(crate) opcode: u8,
  pub(crate) funct3: u8,
  pub(crate) decompress: fn(inst: u16) -> Option<u32>,
}

#[dynamic]
//                               opcode and funct3
static INSTRUCTORS: [Option<CInstructor>; 1024] = {
  let mut res: [Option<CInstructor>; 1024] = array::from_fn(|_| None);
  for instructor in instructors() {
    let funct3 = instructor.funct3 as usize;
    let opcode = instructor.opcode as usize;
    res[(funct3 << 7) | opcode] = Some(instructor);
  }
  res
};

pub(crate) fn decompress(inst: u16) -> Option<u32> {
  let instructor = INSTRUCTORS.get(((inst >> 6) & 0b1110000000 | (inst & 0b11)) as usize)?.as_ref()?;
  (instructor.decompress)(inst)
}

fn instructors() -> Vec<CInstructor> {
  Vec::from([
    CInstructor {
      opcode: 0b00,
      funct3: 0b000,
      decompress: |inst| {
        if inst == 0 {
          None
        } else {
          // C.ADDI4SPN
          let imm = (inst as u32 >> 1) & 0x3c0
            | (inst as u32 >> 7) & 0x30
            | (inst as u32 >> 2) &0x8
            | (inst as u32 >> 4) & 0x4;
          let imm = imm << 20;
          let rs1 = 2u32 << 15;
          let rd = (inst as u32 >> 2) & 0x7;
          let rd = (rd + 8) << 7;
          Some(imm | rs1 | rd | 0b0010011)
        }
      },
    },

    CInstructor {
      opcode: 0b00,
      funct3: 0b001,
      decompress: |inst| {
        // C.FLD
        let imm = (inst as u32 >> 7) & 0x38
          | ((inst as u32) << 1) & 0xc0;
        let imm = imm << 20;
        let rs1 = (inst as u32 >> 7) & 0x7;
        let rs1 = (rs1 + 8) << 15;
        let rd = (inst as u32 >> 2) & 0x7;
        let rd = (rd + 8) << 7;
        Some(imm | rs1 | 0b011 << 12 | rd | 0b0000111)
      },
    },

    CInstructor {
      opcode: 0b00,
      funct3: 0b010,
      decompress: |inst| {
        // C.LW
        let imm = (inst as u32 >> 7) & 0x38
          | (inst as u32 >> 4) & 0x4
          | ((inst as u32) << 1) & 0x40;
        let imm = imm << 20;
        let rs1 = (inst as u32 >> 7) & 0x7;
        let rs1 = (rs1 + 8) << 15;
        let rd = (inst as u32 >> 2) & 0x7;
        let rd = (rd + 8) << 7;
        Some(imm | rs1 | 0b010 << 12 | rd | 0b0000011)
      },
    },

    CInstructor {
      opcode: 0b00,
      funct3: 0b011,
      decompress: |inst| {
        // C.LD
        let imm = (inst as u32 >> 7) & 0x38
          | ((inst as u32) << 1) & 0xc0;
        let imm = imm << 20;
        let rs1 = (inst as u32 >> 7) & 0x7;
        let rs1 = (rs1 + 8) << 15;
        let rd = (inst as u32 >> 2) & 0x7;
        let rd = (rd + 8) << 7;
        Some(imm | rs1 | 0b011 << 12 | rd | 0b0000011)
      }
    },

    CInstructor {
      opcode: 0b00,
      funct3: 0b101,
      decompress: |inst| {
        // C.FSD
        let imm = (inst as u32 >> 7) & 0x38
          | ((inst as u32) << 1) & 0xc0;
        let imm2 = ((imm >> 5) & 0x7f) << 25;
        let imm1 = (imm & 0x1f) << 7;
        let rs2 = (inst as u32 >> 2) & 0x7;
        let rs2 = (rs2 + 8) << 20;
        let rs1 = (inst as u32 >> 7) & 0x7;
        let rs1 = (rs1 + 8) << 15;
        Some(imm2 | rs2 | rs1 | 0b011 << 12 | imm1 | 0b0100111)
      },
    },

    CInstructor {
      opcode: 0b00,
      funct3: 0b110,
      decompress: |inst| {
        // C.SW
        let imm = (inst as u32 >> 7) & 0x38
          | (inst as u32 >> 4) & 0x4
          | ((inst as u32) << 1) & 0x40;
        let imm2 = ((imm >> 5) & 0x7f) << 25;
        let imm1 = (imm & 0x1f) << 7;
        let rs2 = (inst as u32 >> 2) & 0x7;
        let rs2 = (rs2 + 8) << 20;
        let rs1 = (inst as u32 >> 7) & 0x7;
        let rs1 = (rs1 + 8) << 15;
        Some(imm2 | rs2 | rs1 | 0b010 << 12 | imm1 | 0b0100011)
      },
    },

    CInstructor {
      opcode: 0b00,
      funct3: 0b111,
      decompress: |inst| {
        // C.SD
        let imm = (inst as u32 >> 7) & 0x38
          | ((inst as u32) << 1) & 0xc0;
        let imm2 = ((imm >> 5) & 0x7f) << 25;
        let imm1 = (imm & 0x1f) << 7;
        let rs2 = (inst as u32 >> 2) & 0x7;
        let rs2 = (rs2 + 8) << 20;
        let rs1 = (inst as u32 >> 7) & 0x7;
        let rs1 = (rs1 + 8) << 15;
        Some(imm2 | rs2 | rs1 | 0b011 << 12 | imm1 | 0b0100011)
      },
    },
  
    CInstructor {
      opcode: 0b01,
      funct3: 0b000,
      decompress: |inst| {
        let rs1rd = (inst as u32 >> 7) & 0x1f;
        if rs1rd == 0 {
          // C.NOP
          Some(0b00000000000000000000000000010011)
        } else {
          // C.ADDI
          let imm = (inst as u64 >> 7) & 0x20
            | (inst as u64 >> 2) & 0x1f;
          let imm = (extend_sign(imm, 6) as i32 as u32) << 20;
          let rs1 = rs1rd << 15;
          let rd = rs1rd << 7;
          Some(imm | rs1 | rd | 0b0010011)
        }
      }
    },

    CInstructor {
      opcode: 0b01,
      funct3: 0b001,
      decompress: |inst| {
        // C.ADDIW
        let rs1rd = (inst as u32 >> 7) & 0x1f;
        if rs1rd == 0 { return None; }
        let imm = (inst as u64 >> 7) & 0x20
          | (inst as u64 >> 2) & 0x1f;
        let imm = (extend_sign(imm, 6) as i32 as u32) << 20;
        let rs1 = rs1rd << 15;
        let rd = rs1rd << 7;
        Some(imm | rs1 | rd | 0b0011011)
      },
    },

    CInstructor {
      opcode: 0b01,
      funct3: 0b010,
      decompress: |inst| {
        // C.LI
        let imm = (inst as u64 >> 7) & 0x20
          | (inst as u64 >> 2) & 0x1f;
        let imm = (extend_sign(imm, 6) as i32 as u32) << 20;
        let rd = (inst as u32 >> 7) & 0x1f;
        let rd = rd << 7;
        Some(imm | rd | 0b0010011)
      },
    },

    CInstructor {
      opcode: 0b01,
      funct3: 0b011,
      decompress: |inst| {
        let rd = (inst as u32 >> 7) & 0x1f;
        if rd == 2 {
          // C.ADDI16SP
          let imm = (inst as u64 >> 3) & 0x200
            | (inst as u64 >> 2) & 0x10
            | ((inst as u64) << 1) & 0x40
            | ((inst as u64) << 4) & 0x180
            | ((inst as u64) << 3) & 0x20;
          let imm = (extend_sign(imm, 10) as i32 as u32) << 20;
          let rs1 = 2 << 15;
          let rd = 2 << 7;
          Some(imm | rs1 | rd | 0b0010011)
        } else {
          // C.LUI
          let imm = ((inst as u64) << 5) & 0x20000
            | ((inst as u64) << 10) & 0x1f000;
          let imm = extend_sign(imm, 18) as i32 as u32;
          let rd = rd << 7;
          Some(imm | rd | 0b0110111)
        }
      },
    },

    CInstructor {
      opcode: 0b01,
      funct3: 0b100,
      decompress: |inst| {
        let rs1rd = (inst as u32 >> 7) & 0x7;
        let funct3 = (inst >> 10) & 0b11;
        if funct3 == 0b00 {
          // C.SRLI
          let imm = (inst as u32 >> 7) & 0x20
            | (inst as u32 >> 2) & 0x1f;
          let imm = imm << 20;
          let rd = (inst as u32 >> 7) & 0b111;
          let rs1 = (rd + 8) << 15;
          let rd = (rd + 8) << 7;
          Some(imm | rs1 | 0b101 << 12 | rd | 0b0010011)
        } else if funct3 == 0b01 {
          // C.SRAI
          let imm = (inst as u32 >> 7) & 0x20
            | (inst as u32 >> 2) & 0x1f;
          let imm = imm << 20;
          let rd = (inst as u32 >> 7) & 0b111;
          let rs1 = (rd + 8) << 15;
          let rd = (rd + 8) << 7;
          Some(0b0100000 << 25 | imm | rs1 | 0b101 << 12 | rd | 0b0010011)
        } else if funct3 == 0b10 {
          // C.ANDI
          let imm = (inst as u64 >> 7) & 0x20
            | (inst as u64 >> 2) & 0x1f;
          let imm = (extend_sign(imm, 6) as i32 as u32) << 20;
          let rs1 = (rs1rd + 8) << 15;
          let rd = (rs1rd + 8) << 7;
          Some(imm | rs1 | 0b111 << 12 | rd | 0b0010011)
        } else if funct3 == 0b11 {
          let rs2 = (inst as u32 >> 2) & 0b111;
          let rs2 = (rs2 + 8) << 20;
          let rs1 = (rs1rd + 8) << 15;
          let rd = (rs1rd + 8) << 7;
          let funct1 = (inst >> 12) & 0b1;
          let funct2 = (inst >> 5) & 0b11;
          match (funct1, funct2) {
            (0b0, 0b00) => {
              // C.SUB
              Some(0b0100000 << 25 | rs2 | rs1 | rd | 0b0110011)
            },
            (0b0, 0b01) => {
              // C.XOR
              Some(rs2 | rs1 | 0b100 << 12 | rd | 0b0110011)
            },
            (0b0, 0b10) => {
              // C.OR
              Some(rs2 | rs1 | 0b110 << 12 | rd | 0b0110011)
            },
            (0b0, 0b11) => {
              // C.AND
              Some(rs2 | rs1 | 0b111 << 12 | rd | 0b0110011)
            },
            (0b1, 0b00) => {
              // C.SUBW
              Some(0b0100000 << 25 | rs2 | rs1 | rd | 0b0111011)
            },
            (0b1, 0b01) => {
              // C.ADDW
              Some(rs2 | rs1 | rd | 0b0111011)

            },
            _ => None,
          }
        } else {
          None
        }
      },
    },

    CInstructor {
      opcode: 0b01,
      funct3: 0b101,
      decompress: |inst| {
        // C.J
        let inst = inst as u64;
        let imm = (inst >> 1) & 0x800
          | (inst >> 7) & 0x10
          | (inst >> 1) & 0x300
          | (inst << 2) & 0x400
          | (inst >> 1) & 0x40
          | (inst << 1) & 0x80
          | (inst >> 2) & 0xe
          | (inst << 3) & 0x20;
        let imm = extend_sign(imm, 12) as i32 as u32;
        let imm = (imm >> 1) & 0x80000
          | (imm << 8) & 0x7fe00
          | (imm >> 3) & 0x100
          | (imm >> 12) & 0xff;
        let imm = imm << 12;
        Some(imm | 0b1101111)
      },
    },

    CInstructor {
      opcode: 0b01,
      funct3: 0b110,
      decompress: |inst| {
        // C.BEQZ
        let inst = inst as u64;
        let imm = (inst >> 4) & 0x100
          | (inst >> 7) & 0x18
          | (inst << 1) & 0xc0
          | (inst >> 2) & 0x6
          | (inst << 3) & 0x20;
        let imm = extend_sign(imm, 9) as i32 as u32;
        let imm2 = (imm >> 6) & 0x40
          | (imm >> 5) & 0x3f;
        let imm2 = imm2 << 25;
        let imm1 = imm & 0x1e
          | (imm >> 11) & 0x1;
        let imm1 = imm1 << 7;
        let rs1 = (inst as u32 >> 7) & 0x7;
        let rs2 = (rs1 + 8) << 20;
        Some(imm2 | rs2 | imm1 | 0b1100011)
      },
    },

    CInstructor {
      opcode: 0b01,
      funct3: 0b111,
      decompress: |inst| {
        // C.BNEZ
        let inst = inst as u64;
        let imm = (inst >> 4) & 0x100
          | (inst >> 7) & 0x18
          | (inst << 1) & 0xc0
          | (inst >> 2) & 0x6
          | (inst << 3) & 0x20;
        let imm = extend_sign(imm, 9) as i32 as u32;
        let imm2 = (imm >> 6) & 0x40
          | (imm >> 5) & 0x3f;
        let imm2 = imm2 << 25;
        let imm1 = imm & 0x1e
          | (imm >> 11) & 0x1;
        let imm1 = imm1 << 7;
        let rs1 = (inst as u32 >> 7) & 0x7;
        let rs2 = (rs1 + 8) << 20;
        Some(imm2 | rs2 | 0b001 << 12 | imm1 | 0b1100011)
      },
    },

    CInstructor {
      opcode: 0b10,
      funct3: 0b000,
      decompress: |inst| {
        let rs1rd = (inst as u32 >> 7) & 0x1f;
        if rs1rd == 0 { return None; }
        // C.SLLI
        let imm = (inst as u32 >> 7) & 0x20
          | (inst as u32 >> 2) & 0x1f;
        let shamt = imm << 20;
        let rs1 = rs1rd << 15;
        let rd = rs1rd << 7;
        Some(shamt | rs1 | 0b001 << 12 | rd | 0b0010011)
      },
    },

    CInstructor {
      opcode: 0b10,
      funct3: 0b001,
      decompress: |inst| {
        let rd = (inst as u32 >> 7) & 0x1f;
        if rd == 0 { return None; }
        // C.FLDSP
        let imm = (inst as u32 >> 7) & 0x20
          | (inst as u32 >> 2) & 0x18
          | ((inst as u32) << 4) & 0x1c0;
        let imm = imm << 20;
        let rs1 = 2 << 15;
        let rd = rd << 7;
        Some(imm | rs1 | 0b011 << 12 | rd | 0b0000111)
      },
    },
    
    CInstructor {
      opcode: 0b10,
      funct3: 0b010,
      decompress: |inst| {
        // C.LWSP
        let rd = (inst as u32 >> 7) & 0x1f;
        if rd == 0 { return None; }
        let imm = (inst as u32 >> 7) & 0x20
          | (inst as u32 >> 2) & 0x1c
          | ((inst as u32) << 4) & 0xc0;
        let imm = imm << 20;
        let rs1 = 2 << 15;
        let rd = rd << 7;
        Some(imm | rs1 | 0b010 << 12 | rd | 0b0000011)
      },
    },

    CInstructor {
      opcode: 0b10,
      funct3: 0b011,
      decompress: |inst| {
        // C.LDSP
        let rd = (inst as u32 >> 7) & 0x1f;
        if rd == 0 { return None; }
        let imm = (inst as u32 >> 7) & 0x20
          | (inst as u32 >> 2) & 0x18
          | ((inst as u32) << 4) & 0x1c0;
        let imm = imm << 20;
        let rs1 = 2 << 15;
        let rd = rd << 7;
        Some(imm | rs1 | 0b011 << 12 | rd | 0b0000011)
      },
    },

    CInstructor {
      opcode: 0b10,
      funct3: 0b100,
      decompress: |inst| {
        let rs1 = (inst as u32 >> 7) & 0x1f;
        let rs2 = (inst as u32 >> 2) & 0x1f;
        let funct1 = (inst >> 12) & 0b1;
        if funct1 == 0 {
          if rs2 == 0 {
            // C.JR
            if rs1 == 0 { return None; }
            let rs1 = rs1 << 15;
            Some(rs1 | 0b1100111)
          } else {
            // C.MV
            let rs2 = rs2 << 20;
            let rd = rs1 << 7;
            Some(rs2 | rd | 0b0110011)
          }
        } else {
          #[allow(clippy::collapsible_else_if)]
          if rs1 == 0 && rs2 == 0 {
            // C.EBREAK
            Some(0b00000000000100000000000001110011)
          } else if rs1 !=0 && rs2 == 0 {
            // C.JALR
            let rs1 = rs1 << 15;
            let rd = 1 << 7;
            Some(rs1 | rd | 0b1100111)
          } else {
            // C.ADD
            let rs2 = rs2 << 20;
            let rs1s = rs1 << 15;
            let rd = rs1 << 7;
            Some(rs2 | rs1s | rd | 0b0110011)
          }
        }
      },
    },

    CInstructor {
      opcode: 0b10,
      funct3: 0b101,
      decompress: |inst| {
        // C.FSDSP
        let imm = (inst as u32 >> 7) & 0x38
          | (inst as u32 >> 1) & 0x1c0;
        let imm2 = ((imm >> 5) & 0x3f) << 25;
        let imm1 = (imm & 0x1f) << 7;
        let rs2 = (inst as u32 >> 2) & 0x1f;
        let rs1 = 2 << 15;
        Some(imm2 | rs2 | rs1 | 0b011 << 12 | imm1 | 0b0100111)
      },
    },

    CInstructor {
      opcode: 0b10,
      funct3: 0b110,
      decompress: |inst| {
        // C.SWSP
        let imm = (inst as u32 >> 7) & 0x3c
          | (inst as u32 >> 1) & 0xc0;
        let imm2 = ((imm >> 5) & 0x3f) << 25;
        let imm1 = (imm & 0x1f) << 7;
        let rs2 = (inst as u32 >> 2) & 0x1f;
        let rs2 = rs2 << 20;
        let rs1 = 2 << 15;
        Some(imm2 | rs2 | rs1 | 0b010 << 12 | imm1 | 0b0100011)
      },
    },

    CInstructor {
      opcode: 0b10,
      funct3: 0b111,
      decompress: |inst| {
        // C.SDSP
        let imm = (inst as u32 >> 7) & 0x38
          | (inst as u32 >> 1) & 0x1c0;
        let imm2 = ((imm >> 5) & 0x3f) << 25;
        let imm1 = (imm & 0x1f) << 7;
        let rs2 = ((inst as u32 >> 2) & 0x1f) << 20;
        let rs1 = 2 << 15;
        Some(imm2 | rs2 | rs1 | 0b011 << 12 | imm1 | 0b0100011)
      },
    },
  ])
}
