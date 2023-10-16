#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rst(pub u8);
impl Rst {
    #[doc = "Perform I2C software reset"]
    pub const RESET: Self = Self(0x07);
}
impl Rst {
    pub const fn from_bits(val: u8) -> Rst {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Status {
    #[doc = "Bus Error (master mode only)"]
    BUS_ERROR = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "START condition transmitted"]
    START = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Repeated START condition transmitted"]
    RSTART = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "Address and Write bit transmitted, ACK received"]
    SLAW_TX_ACK = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "Address and Write bit transmitted, NACK received"]
    SLAW_TX_NACK = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    #[doc = "Data byte transmitted in master mode, ACK received"]
    MDATA_TX_ACK = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "Data byte transmitted in master mode, NACK received"]
    MDATA_TX_NACK = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    #[doc = "Arbitration lost in address or data byte"]
    ARB_LOST = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Address and read bit transmitted, ACK received"]
    SLAR_TX_ACK = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    #[doc = "Address and read bit transmitted, NACK received"]
    SLAR_TX_NACK = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "Data byte received in master mode, ACK transmitted"]
    MDATA_RX_ACK = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    #[doc = "Data byte received in master mode, NACK transmitted"]
    MDATA_RX_NACK = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    #[doc = "Slave address and write bit received, ACK transmitted"]
    SLAW_RX_ACK = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    #[doc = "Arbitration lost in address as master, slave address and write bit received, ACK transmitted"]
    ARB_LOST_SLAW_RX_ACK = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    #[doc = "General call received, ACK returned"]
    GC_ACK = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    #[doc = "Data byte received after slave address received, ACK transmitted"]
    SDATA_RX_ACK = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    #[doc = "Data byte received after slave address received, NACK transmitted"]
    SDATA_RX_NACK = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    #[doc = "Data byte received after general call address received, ACK transmitted"]
    DATA_RX_GC_ACK = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    #[doc = "STOP or repeated START condition received in slave mode"]
    STOP_RSTART_RX = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    #[doc = "Address and read bit received, ACK transmitted"]
    SLAR_RX_ACK = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    #[doc = "Arbitration lost in address as master, address and read bit received, ACK transmitted"]
    ARB_LOST_SLAR_RX_ACK = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    #[doc = "Data byte transmitted in slave mode, ACK received"]
    SDATA_TX_ACK = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    #[doc = "Data byte transmitted in slave mode, NACK received"]
    SDATA_TX_NACK = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    #[doc = "Last data byte transmitted in slave mode, ACK received"]
    SDATA_LAST_TX_ACK = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    #[doc = "Last data byte transmitted in slave mode, NACK received"]
    SDATA_LAST_TX_NACK = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    #[doc = "Second address byte received in slave mode, ACK transmitted"]
    SECOND_ADDR_TX_ACK = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    #[doc = "Second address byte received in slave mode, NACK transmitted"]
    SECOND_ADDR_TX_NACK = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    #[doc = "No relevant status information"]
    NO_INFO = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status {
    #[inline(always)]
    fn from(val: u8) -> Status {
        Status::from_bits(val)
    }
}
impl From<Status> for u8 {
    #[inline(always)]
    fn from(val: Status) -> u8 {
        Status::to_bits(val)
    }
}
