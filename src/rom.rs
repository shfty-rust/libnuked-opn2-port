// logsin table
pub const LOGSIN_ROM: [u16; 256] = [
    0x859, 0x6c3, 0x607, 0x58b, 0x52e, 0x4e4, 0x4a6, 0x471, 0x443, 0x41a, 0x3f5, 0x3d3, 0x3b5,
    0x398, 0x37e, 0x365, 0x34e, 0x339, 0x324, 0x311, 0x2ff, 0x2ed, 0x2dc, 0x2cd, 0x2bd, 0x2af,
    0x2a0, 0x293, 0x286, 0x279, 0x26d, 0x261, 0x256, 0x24b, 0x240, 0x236, 0x22c, 0x222, 0x218,
    0x20f, 0x206, 0x1fd, 0x1f5, 0x1ec, 0x1e4, 0x1dc, 0x1d4, 0x1cd, 0x1c5, 0x1be, 0x1b7, 0x1b0,
    0x1a9, 0x1a2, 0x19b, 0x195, 0x18f, 0x188, 0x182, 0x17c, 0x177, 0x171, 0x16b, 0x166, 0x160,
    0x15b, 0x155, 0x150, 0x14b, 0x146, 0x141, 0x13c, 0x137, 0x133, 0x12e, 0x129, 0x125, 0x121,
    0x11c, 0x118, 0x114, 0x10f, 0x10b, 0x107, 0x103, 0xff, 0xfb, 0xf8, 0xf4, 0xf0, 0xec, 0xe9,
    0xe5, 0xe2, 0xde, 0xdb, 0xd7, 0xd4, 0xd1, 0xcd, 0xca, 0xc7, 0xc4, 0xc1, 0xbe, 0xbb, 0xb8, 0xb5,
    0xb2, 0xaf, 0xac, 0xa9, 0xa7, 0xa4, 0xa1, 0x9f, 0x9c, 0x99, 0x97, 0x94, 0x92, 0x8f, 0x8d, 0x8a,
    0x88, 0x86, 0x83, 0x81, 0x7f, 0x7d, 0x7a, 0x78, 0x76, 0x74, 0x72, 0x70, 0x6e, 0x6c, 0x6a, 0x68,
    0x66, 0x64, 0x62, 0x60, 0x5e, 0x5c, 0x5b, 0x59, 0x57, 0x55, 0x53, 0x52, 0x50, 0x4e, 0x4d, 0x4b,
    0x4a, 0x48, 0x46, 0x45, 0x43, 0x42, 0x40, 0x3f, 0x3e, 0x3c, 0x3b, 0x39, 0x38, 0x37, 0x35, 0x34,
    0x33, 0x31, 0x30, 0x2f, 0x2e, 0x2d, 0x2b, 0x2a, 0x29, 0x28, 0x27, 0x26, 0x25, 0x24, 0x23, 0x22,
    0x21, 0x20, 0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x17, 0x16, 0x15, 0x14, 0x14,
    0x13, 0x12, 0x11, 0x11, 0x10, 0xf, 0xf, 0xe, 0xd, 0xd, 0xc, 0xc, 0xb, 0xa, 0xa, 0x9, 0x9, 0x8,
    0x8, 0x7, 0x7, 0x7, 0x6, 0x6, 0x5, 0x5, 0x5, 0x4, 0x4, 0x4, 0x3, 0x3, 0x3, 0x2, 0x2, 0x2, 0x2,
    0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0, 0, 0, 0, 0, 0, 0, 0,
];

// exp table
pub const EXP_ROM: [u16; 256] = [
    0, 0x3, 0x6, 0x8, 0xb, 0xe, 0x11, 0x14, 0x16, 0x19, 0x1c, 0x1f, 0x22, 0x25, 0x28, 0x2a, 0x2d,
    0x30, 0x33, 0x36, 0x39, 0x3c, 0x3f, 0x42, 0x45, 0x48, 0x4b, 0x4e, 0x51, 0x54, 0x57, 0x5a, 0x5d,
    0x60, 0x63, 0x66, 0x69, 0x6c, 0x6f, 0x72, 0x75, 0x78, 0x7b, 0x7e, 0x82, 0x85, 0x88, 0x8b, 0x8e,
    0x91, 0x94, 0x98, 0x9b, 0x9e, 0xa1, 0xa4, 0xa8, 0xab, 0xae, 0xb1, 0xb5, 0xb8, 0xbb, 0xbe, 0xc2,
    0xc5, 0xc8, 0xcc, 0xcf, 0xd2, 0xd6, 0xd9, 0xdc, 0xe0, 0xe3, 0xe7, 0xea, 0xed, 0xf1, 0xf4, 0xf8,
    0xfb, 0xff, 0x102, 0x106, 0x109, 0x10c, 0x110, 0x114, 0x117, 0x11b, 0x11e, 0x122, 0x125, 0x129,
    0x12c, 0x130, 0x134, 0x137, 0x13b, 0x13e, 0x142, 0x146, 0x149, 0x14d, 0x151, 0x154, 0x158,
    0x15c, 0x160, 0x163, 0x167, 0x16b, 0x16f, 0x172, 0x176, 0x17a, 0x17e, 0x181, 0x185, 0x189,
    0x18d, 0x191, 0x195, 0x199, 0x19c, 0x1a0, 0x1a4, 0x1a8, 0x1ac, 0x1b0, 0x1b4, 0x1b8, 0x1bc,
    0x1c0, 0x1c4, 0x1c8, 0x1cc, 0x1d0, 0x1d4, 0x1d8, 0x1dc, 0x1e0, 0x1e4, 0x1e8, 0x1ec, 0x1f0,
    0x1f5, 0x1f9, 0x1fd, 0x201, 0x205, 0x209, 0x20e, 0x212, 0x216, 0x21a, 0x21e, 0x223, 0x227,
    0x22b, 0x230, 0x234, 0x238, 0x23c, 0x241, 0x245, 0x249, 0x24e, 0x252, 0x257, 0x25b, 0x25f,
    0x264, 0x268, 0x26d, 0x271, 0x276, 0x27a, 0x27f, 0x283, 0x288, 0x28c, 0x291, 0x295, 0x29a,
    0x29e, 0x2a3, 0x2a8, 0x2ac, 0x2b1, 0x2b5, 0x2ba, 0x2bf, 0x2c4, 0x2c8, 0x2cd, 0x2d2, 0x2d6,
    0x2db, 0x2e0, 0x2e5, 0x2e9, 0x2ee, 0x2f3, 0x2f8, 0x2fd, 0x302, 0x306, 0x30b, 0x310, 0x315,
    0x31a, 0x31f, 0x324, 0x329, 0x32e, 0x333, 0x338, 0x33d, 0x342, 0x347, 0x34c, 0x351, 0x356,
    0x35b, 0x360, 0x365, 0x36a, 0x370, 0x375, 0x37a, 0x37f, 0x384, 0x38a, 0x38f, 0x394, 0x399,
    0x39f, 0x3a4, 0x3a9, 0x3ae, 0x3b4, 0x3b9, 0x3bf, 0x3c4, 0x3c9, 0x3cf, 0x3d4, 0x3da, 0x3df,
    0x3e4, 0x3ea, 0x3ef, 0x3f5, 0x3fa,
];

// Note table
pub const FN_NOTE: [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 3, 3, 3, 3, 3];

// Envelope generator
pub const EG_STEP_HI: [[u32; 4]; 4] = [[0, 0, 0, 0], [1, 0, 0, 0], [1, 0, 1, 0], [1, 1, 1, 0]];
pub const EG_AM_SHIFT: [u8; 4] = [7, 3, 1, 0];

// Phase generator
pub const PG_DETUNE: [u32; 8] = [16, 17, 19, 20, 22, 24, 27, 29];

pub const PG_LFO_SH1: [[u32; 8]; 8] = [
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 7, 7, 1, 1],
    [7, 7, 7, 7, 1, 1, 1, 1],
    [7, 7, 7, 1, 1, 1, 1, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
];

pub const PG_LFO_SH2: [[u32; 8]; 8] = [
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 2, 2, 2, 2],
    [7, 7, 7, 2, 2, 2, 7, 7],
    [7, 7, 2, 2, 7, 7, 2, 2],
    [7, 7, 2, 7, 7, 7, 2, 7],
    [7, 7, 7, 2, 7, 7, 2, 1],
    [7, 7, 7, 2, 7, 7, 2, 1],
    [7, 7, 7, 2, 7, 7, 2, 1],
];

// Address decoder
pub const OP_OFFSET: [u32; 12] = [
    0,     // Ch1 OP1/OP2
    0x1,   // Ch2 OP1/OP2
    0x2,   // Ch3 OP1/OP2
    0x100, // Ch4 OP1/OP2
    0x101, // Ch5 OP1/OP2
    0x102, // Ch6 OP1/OP2
    0x4,   // Ch1 OP3/OP4
    0x5,   // Ch2 OP3/OP4
    0x6,   // Ch3 OP3/OP4
    0x104, // Ch4 OP3/OP4
    0x105, // Ch5 OP3/OP4
    0x106, // Ch6 OP3/OP4
];
pub const CH_OFFSET: [u32; 6] = [
    0,     // Ch1
    0x1,   // Ch2
    0x2,   // Ch3
    0x100, // Ch4
    0x101, // Ch5
    0x102, // Ch6
];

// LFO
pub const LFO_CYCLES: [u32; 8] = [108, 77, 71, 67, 62, 44, 8, 5];

// FM algorithm
pub const FM_ALGORITHM: [[[u32; 8]; 6]; 4] = [
    [
        [1, 1, 1, 1, 1, 1, 1, 1], // OP1_0
        [1, 1, 1, 1, 1, 1, 1, 1], // OP1_1
        [0, 0, 0, 0, 0, 0, 0, 0], // OP2
        [0, 0, 0, 0, 0, 0, 0, 0], // Last operator
        [0, 0, 0, 0, 0, 0, 0, 0], // Last operator
        [0, 0, 0, 0, 0, 0, 0, 1], // Out
    ],
    [
        [0, 1, 0, 0, 0, 1, 0, 0], // OP1_0
        [0, 0, 0, 0, 0, 0, 0, 0], // OP1_1
        [1, 1, 1, 0, 0, 0, 0, 0], // OP2
        [0, 0, 0, 0, 0, 0, 0, 0], // Last operator
        [0, 0, 0, 0, 0, 0, 0, 0], // Last operator
        [0, 0, 0, 0, 0, 1, 1, 1], // Out
    ],
    [
        [0, 0, 0, 0, 0, 0, 0, 0], // OP1_0
        [0, 0, 0, 0, 0, 0, 0, 0], // OP1_1
        [0, 0, 0, 0, 0, 0, 0, 0], // OP2
        [1, 0, 0, 1, 1, 1, 1, 0], // Last operator
        [0, 0, 0, 0, 0, 0, 0, 0], // Last operator
        [0, 0, 0, 0, 1, 1, 1, 1], // Out
    ],
    [
        [0, 0, 1, 0, 0, 1, 0, 0], // OP1_0
        [0, 0, 0, 0, 0, 0, 0, 0], // OP1_1
        [0, 0, 0, 1, 0, 0, 0, 0], // OP2
        [1, 1, 0, 1, 1, 0, 0, 0], // Last operator
        [0, 0, 1, 0, 0, 0, 0, 0], // Last operator
        [1, 1, 1, 1, 1, 1, 1, 1], // Out
    ],
];
