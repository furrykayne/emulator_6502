use crate::nes::opcodes::AddrMode::*;
use crate::nes::opcodes::OpCode::*;

#[derive(Debug, Copy, Clone)]
pub(crate) enum AddrMode {
    Abs,
    Implicit,
    Immediate,
    Absolute,
    AbsoluteX,
    Relative,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum OpCode {
    Sei,
    Cld,
    Txs,
    Cpx(AddrMode),
    Bne,
    Bpl(AddrMode),
    Inx,
    Dex,
    Dey,
    Ldx(AddrMode),
    Ldy(AddrMode),
    Lda(AddrMode),
    Sta(AddrMode),
    Stx(AddrMode),
}

pub static OP_CODES: [Option<(OpCode)>; 256] = [
    None, // 0x00
    None, // 0x01
    None, // 0x02
    None, // 0x03
    None, // 0x04
    None, // 0x05
    None, // 0x06
    None, // 0x07
    None, // 0x08
    None, // 0x09
    None, // 0x0a
    None, // 0x0b
    None, // 0x0c
    None, // 0x0d
    None, // 0x0e
    None, // 0x0f
    Some(Bpl(Relative)), // 0x10
    None, // 0x11
    None, // 0x12
    None, // 0x13
    None, // 0x14
    None, // 0x15
    None, // 0x16
    None, // 0x17
    None, // 0x18
    None, // 0x19
    None, // 0x1a
    None, // 0x1b
    None, // 0x1c
    None, // 0x1d
    None, // 0x1e
    None, // 0x1f
    None, // 0x20
    None, // 0x21
    None, // 0x22
    None, // 0x23
    None, // 0x24
    None, // 0x25
    None, // 0x26
    None, // 0x27
    None, // 0x28
    None, // 0x29
    None, // 0x2a
    None, // 0x2b
    None, // 0x2c
    None, // 0x2d
    None, // 0x2e
    None, // 0x2f
    None, // 0x30
    None, // 0x31
    None, // 0x32
    None, // 0x33
    None, // 0x34
    None, // 0x35
    None, // 0x36
    None, // 0x37
    None, // 0x38
    None, // 0x39
    None, // 0x3a
    None, // 0x3b
    None, // 0x3c
    None, // 0x3d
    None, // 0x3e
    None, // 0x3f
    None, // 0x40
    None, // 0x41
    None, // 0x42
    None, // 0x43
    None, // 0x44
    None, // 0x45
    None, // 0x46
    None, // 0x47
    None, // 0x48
    None, // 0x49
    None, // 0x4a
    None, // 0x4b
    None, // 0x4c
    None, // 0x4d
    None, // 0x4e
    None, // 0x4f
    None, // 0x50
    None, // 0x51
    None, // 0x52
    None, // 0x53
    None, // 0x54
    None, // 0x55
    None, // 0x56
    None, // 0x57
    None, // 0x58
    None, // 0x59
    None, // 0x5a
    None, // 0x5b
    None, // 0x5c
    None, // 0x5d
    None, // 0x5e
    None, // 0x5f
    None, // 0x60
    None, // 0x61
    None, // 0x62
    None, // 0x63
    None, // 0x64
    None, // 0x65
    None, // 0x66
    None, // 0x67
    None, // 0x68
    None, // 0x69
    None, // 0x6a
    None, // 0x6b
    None, // 0x6c
    None, // 0x6d
    None, // 0x6e
    None, // 0x6f
    None, // 0x70
    None, // 0x71
    None, // 0x72
    None, // 0x73
    None, // 0x74
    None, // 0x75
    None, // 0x76
    None, // 0x77
    Some(Sei), // 0x78
    None, // 0x79
    None, // 0x7a
    None, // 0x7b
    None, // 0x7c
    None, // 0x7d
    None, // 0x7e
    None, // 0x7f
    None, // 0x80
    None, // 0x81
    None, // 0x82
    None, // 0x83
    None, // 0x84
    None, // 0x85
    None, // 0x86
    None, // 0x87
    Some(Dey), // 0x88
    None, // 0x89
    None, // 0x8a
    None, // 0x8b
    None, // 0x8c
    Some(Sta(Absolute)), // 0x8d
    Some(Stx(Absolute)), // 0x8e
    None, // 0x8f
    None, // 0x90
    None, // 0x91
    None, // 0x92
    None, // 0x93
    None, // 0x94
    None, // 0x95
    None, // 0x96
    None, // 0x97
    None, // 0x98
    None, // 0x99
    Some(Txs), // 0x9a
    None, // 0x9b
    None, // 0x9c
    None, // 0x9d
    None, // 0x9e
    None, // 0x9f
    Some(Ldy(Immediate)), // 0xa0
    None, // 0xa1
    Some(Ldx(Immediate)), // 0xa2
    None, // 0xa3
    None, // 0xa4
    None, // 0xa5
    None, // 0xa6
    None, // 0xa7
    None, // 0xa8
    Some(Lda(Immediate)), // 0xa9
    None, // 0xaa
    None, // 0xab
    None, // 0xac
    Some(Lda(Absolute)), // 0xad
    None, // 0xae
    None, // 0xaf
    None, // 0xb0
    None, // 0xb1
    None, // 0xb2
    None, // 0xb3
    None, // 0xb4
    None, // 0xb5
    None, // 0xb6
    None, // 0xb7
    None, // 0xb8
    None, // 0xb9
    None, // 0xba
    None, // 0xbb
    None, // 0xbc
    Some(Lda(AbsoluteX)), // 0xbd
    None, // 0xbe
    None, // 0xbf
    None, // 0xc0
    None, // 0xc1
    None, // 0xc2
    None, // 0xc3
    None, // 0xc4
    None, // 0xc5
    None, // 0xc6
    None, // 0xc7
    None, // 0xc8
    None, // 0xc9
    Some(Dex), // 0xca
    None, // 0xcb
    None, // 0xcc
    None, // 0xcd
    None, // 0xce
    None, // 0xcf
    Some(Bne), // 0xd0
    None, // 0xd1
    None, // 0xd2
    None, // 0xd3
    None, // 0xd4
    None, // 0xd5
    None, // 0xd6
    None, // 0xd7
    Some(Cld), // 0xd8
    None, // 0xd9
    None, // 0xda
    None, // 0xdb
    None, // 0xdc
    None, // 0xdd
    None, // 0xde
    None, // 0xdf
    Some(Cpx(Immediate)), // 0xe0
    None, // 0xe1
    None, // 0xe2
    None, // 0xe3
    None, // 0xe4
    None, // 0xe5
    None, // 0xe6
    None, // 0xe7
    Some(Inx), // 0xe8
    None, // 0xe9
    None, // 0xea
    None, // 0xeb
    None, // 0xec
    None, // 0xed
    None, // 0xee
    None, // 0xef
    None, // 0xf0
    None, // 0xf1
    None, // 0xf2
    None, // 0xf3
    None, // 0xf4
    None, // 0xf5
    None, // 0xf6
    None, // 0xf7
    None, // 0xf8
    None, // 0xf9
    None, // 0xfa
    None, // 0xfb
    None, // 0xfc
    None, // 0xfd
    None, // 0xfe
    None, // 0xff
];