use std::fmt;

#[derive(Copy,Clone,Debug,Hash)]
#[repr(u8)]
pub enum Residue{
    A = 0b0001,
    T = 0b0010,
    C = 0b0100,
    G = 0b1000,
    W = 0b0011,
    S = 0b1100,
    R = 0b1001,
    Y = 0b0110,
    K = 0b1010,
    M = 0b0101,
    B = 0b1110,
    D = 0b1011,
    H = 0b0111,
    V = 0b1101,
    N = 0b1111,
    Gap = 0b0000,
}

impl Residue{
    pub fn from_int(value:u8)->Residue{
        match value{
            0b0001 => Residue::A,
            0b0010 => Residue::T,
            0b0100 => Residue::C,
            0b1000 => Residue::G,
            0b0011 => Residue::W,
            0b1100 => Residue::S,
            0b1001 => Residue::R,
            0b0110 => Residue::Y,
            0b1010 => Residue::K,
            0b0101 => Residue::M,
            0b1110 => Residue::B,
            0b1011 => Residue::D,
            0b0111 => Residue::H,
            0b1101 => Residue::V,
            0b1111 => Residue::N,
            _=> Residue::Gap
        }
    }
    pub fn from_char(ch:char)->Residue{
        match ch{
            'A' => Residue::A,
            'T'|'U' => Residue::T,
            'C' => Residue::C,
            'G' => Residue::G,
            'W' => Residue::W,
            'S' => Residue::S,
            'R' => Residue::R,
            'Y' => Residue::Y,
            'K' => Residue::K,
            'M' => Residue::M,
            'B' => Residue::B,
            'D' => Residue::D,
            'H' => Residue::V,
            'V' => Residue::V,
            'N'|'X' => Residue::N,
            '-'|'.' => Residue::Gap,
            _=>unreachable!()
        }
    }
    pub fn complement(&self)->Residue{
        match self{
            Residue::A => Residue::T,
            Residue::T => Residue::A,
            Residue::C => Residue::G,
            Residue::G => Residue::C,
            Residue::Y => Residue::R,
            Residue::R => Residue::Y,
            Residue::W => Residue::W,
            Residue::S => Residue::S,
            Residue::K => Residue::M,
            Residue::M => Residue::K,
            Residue::D => Residue::H,
            Residue::V => Residue::B,
            Residue::H => Residue::D,
            Residue::B => Residue::V,
            Residue::N => Residue::N,
            Residue::Gap => Residue::Gap,
        }
    }
    pub fn one_way_match(&self, rhs:Residue)->bool{
        match *self{
            Residue::A | Residue::T| Residue::C | Residue::G=>{
                return *self as u8 ^ rhs as u8 == 0;
            }
            Residue::Y => if let Residue::Y | Residue::C | Residue::T = rhs{ true }else{ false }
            Residue::R => if let Residue::R | Residue::A | Residue::G = rhs{ true }else{ false }
            Residue::W => if let Residue::W | Residue::A | Residue::T = rhs{ true }else{ false }
            Residue::S => if let Residue::S | Residue::G | Residue::C = rhs{ true }else{ false }
            Residue::K => if let Residue::K | Residue::T | Residue::G = rhs{ true }else{ false }
            Residue::M => if let Residue::M | Residue::C | Residue::A = rhs{ true }else{ false }
            Residue::D => if let Residue::D | Residue::A | Residue::G | Residue::T = rhs{ true }else{ false }
            Residue::V => if let Residue::V | Residue::A | Residue::C | Residue::G = rhs{ true }else{ false }
            Residue::H => if let Residue::H | Residue::A | Residue::C | Residue::T = rhs{ true }else{ false }
            Residue::B => if let Residue::B | Residue::C | Residue::G | Residue::T = rhs{ true }else{ false }
            Residue::N => if let Residue::Gap = rhs{ false }else{ true }
            Residue::Gap=> if let Residue::Gap = rhs{ true }else{ false }
        }
    }
    pub fn vec_from_str(input: &str)->Vec<Self>{
        let mut result =  Vec::new();
        for c in input.chars(){
            result.push(match c.to_ascii_uppercase(){
                'A' => Residue::A,
                'T'|'U' => Residue::T,
                'C' => Residue::C,
                'G' => Residue::G,
                'W' => Residue::W,
                'S' => Residue::S,
                'R' => Residue::R,
                'Y' => Residue::Y,
                'K' => Residue::K,
                'M' => Residue::M,
                'B' => Residue::B,
                'D' => Residue::D,
                'H' => Residue::V,
                'V' => Residue::V,
                'N'|'X' => Residue::N,
                '-' => Residue::Gap,
                _=>panic!("Invalid character in PAM sequence")
            })
        }
        result
    }
}

impl std::ops::BitXor for Residue{
    type Output = Residue;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Residue::from_int(self as u8 ^ rhs as u8)
    }
}
impl std::ops::BitAnd for Residue{
    type Output = Residue;
    fn bitand(self, rhs: Self) -> Self::Output {
        Residue::from_int(self as u8 & rhs as u8)
    }
}
impl std::ops::BitOr for Residue{
    type Output = Residue;
    fn bitor(self, rhs: Self) -> Self::Output {
        if self==Residue::Gap || rhs==Residue::Gap{
            return Residue::Gap
        }
        Residue::from_int(self as u8 | rhs as u8)
    }
}

impl Eq for Residue{}

impl PartialEq for Residue{
    fn eq(&self, other: &Self) -> bool {
        if *self as u8 ==0  ||  *other as u8 ==0{
            return *self as u8 == *other as u8
        }
        if (*self as u8 & *other as u8 )!=0{
            true
        }else {
            false
        }
    }
}

impl fmt::Display for Residue{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        if *self==Residue::Gap{
            write!(f, "-")
        }else{
            write!(f, "{:?}",self)
        }
    }
}