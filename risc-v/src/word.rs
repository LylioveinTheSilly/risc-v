use std::ops::{ 
    Deref, DerefMut, 
    BitAnd, BitAndAssign, 
    BitOr,  BitOrAssign, 
    BitXor, BitXorAssign,

    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,

    Shl, ShlAssign,
    Shr, ShrAssign,

    Not,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Word(pub u32);
impl Word {
    pub const MAX: Word = Word(u32::MAX);

    pub fn opcode(&self) -> Word { 
        self.0.bitand(0b_1111111).into()
    }
    pub fn funct3(&self) -> Word { 
        (self.0.bitand(0b_00000000_00000000_01110000_00000000) >> 12).into()
    }
    pub fn funct7(&self) -> Word { 
        (self.0.bitand(0b_11111110_00000000_00000000_00000000) >> 25).into()
    }
    pub fn rs1(&self) -> Word { 
        (self.0.bitand(0b_00000000_00001111_10000000_00000000) >> 15).into()
    }
    pub fn rs2(&self) -> Word { 
        (self.0.bitand(0b_00000001_11110000_00000000_00000000) >> 20).into()
    }
    pub fn rd(&self) -> Word { 
        (self.0.bitand(0b_00000000_00000000_00001111_10000000) >> 7).into()
    }

    pub fn i_type_immediate(&self) -> Word { 
        let mut value = 0;
        let imm = self.0 & 0b_11111111_11110000_00000000_00000000;
        let sign = (self.0 & 0b_10000000_00000000_00000000_00000000) >> 31;

        value |= imm >> 20;

        match sign {
            0 => (),
            1 => value |= 0b_11111111_11111111_11111000_00000000,
            _ => unreachable!("sign can only be 0 or 1"),
        }

        value.into()
    }
    pub fn s_type_immediate(&self) -> Word { 
        let upper_imm =  self.0 & 0b_11111110_00000000_00000000_00000000;
        let lower_imm =  self.0 & 0b_00000000_00000000_00001111_10000000;

        let mut value = 0;
        let sign = (upper_imm & 0b_10000000_00000000_00000000_00000000) >> 31;

        value |= lower_imm >> 7;
        value |= upper_imm >> 20;

        match sign {
            0 => (),
            1 => value |= 0b_11111111_11111111_11111000_00000000,
            _ => unreachable!("sign can only be 0 or 1"),
        }

        value.into()
    }
    pub fn b_type_immediate(&self) -> Word { 
        let upper_imm =  self.0 & 0b_11111110_00000000_00000000_00000000;
        let lower_imm =  self.0 & 0b_00000000_00000000_00001111_10000000;

        let mut value = 0;
        let sign = (upper_imm & 0b_10000000_00000000_00000000_00000000) >> 31;

        value |= (lower_imm & 0b_00000000_00000000_00001111_00000000) >> 7;
        value |= (upper_imm & 0b_01111110_00000000_00000000_00000000) >> 20;
        value |= (lower_imm & 0b_00000000_00000000_00000000_10000000) << 4;

        match sign {
            0 => (),
            1 => value |= 0b_11111111_11111111_11111000_00000000,
            _ => unreachable!("sign can only be 0 or 1"),
        }

        value.into()
    }
    pub fn u_type_immediate(&self) -> Word { 
        let imm = self.0 & 0b_11111111_11111111_11110000_00000000;
        imm.into()
    }
    pub fn j_type_immediate(&self) -> Word { 
        let imm = self.0 & 0b_11111111_11111111_11110000_00000000;

        let mut value = 0;
        let sign = (imm & 0b_10000000_00000000_00000000_00000000) >> 31;

        value |= (imm & 0b_01111111_11100000_00000000_00000000) >> 20;
        value |= (imm & 0b_00000000_00010000_00000000_00000000) >> 9;
        value |= imm & 0b_00000000_00001111_11110000_00000000;

        match sign {
            0 => (),
            1 => value |= 0b_11111111_11111000_00000000_00000000,
            _ => unreachable!("sign can only be 0 or 1"),
        }

        value.into()
    }
    pub fn shift_imm_amount(&self) -> Word {
        let imm = self.0 & 0b_11111111_11110000_00000000_00000000;
        let shamt_i = (imm >> 20) & 0b_11111;
        shamt_i.into()
    }

    pub fn from_le_bytes(bytes: [u8; 4]) -> Word {
        Word(u32::from_le_bytes(bytes))
    }

    pub fn signed(&self) -> i32 {
        self.0 as i32
    }

    pub fn unsigned(&self) -> u32 {
        self.0
    }
}

macro_rules! impl_operations_for_type {
    ($impl_type:ty) => {
        impl From<$impl_type> for Word {
            fn from(value: $impl_type) -> Word {
                Word(value as u32)
            } 
        }
        impl From<Word> for $impl_type {
            fn from(value: Word) -> $impl_type {
                value.0 as $impl_type
            } 
        }
    }
}

impl_operations_for_type!(u8);
impl_operations_for_type!(i8);
impl_operations_for_type!(u16);
impl_operations_for_type!(i16);
impl_operations_for_type!(u32);
impl_operations_for_type!(i32);
impl_operations_for_type!(u64);
impl_operations_for_type!(i64);

impl Deref for Word {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Word {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BitAnd for Word {
    type Output = Word;
    fn bitand(self, rhs: Self) -> Self::Output {
        (self.0 & rhs.0).into()
    }
}
impl BitAndAssign for Word {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for Word {
    type Output = Word;
    fn bitor(self, rhs: Self) -> Self::Output {
        (self.0 | rhs.0).into()
    }
}
impl BitOrAssign for Word {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for Word {
    type Output = Word;
    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}
impl BitXorAssign for Word {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Add for Word {
    type Output = Word;
    fn add(self, rhs: Self) -> Self::Output {
        (self.0.wrapping_add(rhs.0)).into()
    }
}
impl AddAssign for Word {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0)
    }
}

impl Sub for Word {
    type Output = Word;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.0.wrapping_sub(rhs.0)).into()
    }
}
impl SubAssign for Word {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_sub(rhs.0)
    }
}

impl Mul for Word {
    type Output = Word;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.0.wrapping_mul(rhs.0)).into()
    }
}
impl MulAssign for Word {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_mul(rhs.0)
    }
}

impl Div for Word {
    type Output = Word;
    fn div(self, rhs: Self) -> Self::Output {
        (self.0.wrapping_div(rhs.0)).into()
    }
}
impl DivAssign for Word {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_div(rhs.0)
    }
}

impl Shl for Word {
    type Output = Word;
    fn shl(self, rhs: Self) -> Self::Output {
        (self.0.wrapping_shl(rhs.0)).into()
    }
}
impl ShlAssign for Word {
    fn shl_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_shl(rhs.0)
    }
}

impl Shr for Word {
    type Output = Word;
    fn shr(self, rhs: Self) -> Self::Output {
        (self.0.wrapping_shr(rhs.0)).into()
    }
}
impl ShrAssign for Word {
    fn shr_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_shr(rhs.0)
    }
}

impl Not for Word {
    type Output = Word;
    fn not(self) -> Self::Output {
        Word(!self.0)
    }
}