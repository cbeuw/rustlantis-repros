#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics, const_hash)]
    #![allow(unused_parens, unused_assignments, overflowing_literals)]
    extern crate core;
    use core::intrinsics::mir::*;

    use std::fmt::Debug;

    #[inline(never)]
    pub fn dump_var<T: Debug, U: Debug, V: Debug, W: Debug>(f: usize,
        var0: usize, val0: T,
        var1: usize, val1: U,
        var2: usize, val2: V,
        var3: usize, val3: W,
    ) {
        println!("fn{f}:_{var0} = {val0:?}\n_{var1} = {val1:?}\n_{var2} = {val2:?}\n_{var3} = {val3:?}");
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: u128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64) -> i64 {
mir! {
type RET = i64;
let _14: i32;
let _15: (bool, i128, *const i64, (usize,));
let _16: [i128; 8];
let _17: bool;
let _18: bool;
let _19: f32;
let _20: Adt59;
let _21: bool;
let _22: Adt63;
let _23: Adt60;
let _24: Adt51;
let _25: f32;
let _26: [i16; 8];
let _27: Adt64;
let _28: ([u16; 5], (bool, i16), [u16; 5], [i128; 8], [u32; 7]);
let _29: isize;
let _30: [isize; 8];
let _31: Adt56;
let _32: [usize; 2];
let _33: [char; 2];
let _34: bool;
let _35: char;
let _36: ([u16; 5], (bool, i16), [u16; 5], [i128; 8], [u32; 7]);
let _37: [isize; 5];
let _38: f64;
let _39: [char; 5];
let _40: char;
let _41: char;
let _42: f32;
let _43: u64;
let _44: Adt50;
let _45: (usize,);
let _46: f64;
let _47: isize;
let _48: f32;
let _49: f64;
let _50: u32;
let _51: Adt56;
let _52: i8;
let _53: ();
let _54: ();
{
_1 = true;
_6 = (-142872833_i32) - (-1263464607_i32);
_5 = (-737466805745828137_i64) as i16;
_16 = [(-158141002429875867046941817664239038251_i128),77868434494471101237799406964112104289_i128,(-3955995758504024773023591801028797735_i128),117059504018504641445306057308593941115_i128,(-61534578831263403609018983008219218878_i128),13451498964128564052369848149992653013_i128,65693353499988834704579460803687944915_i128,(-8187100472087307859412719023575701088_i128)];
Goto(bb1)
}
bb1 = {
_10 = 21_u8;
_3 = (-10_isize) ^ (-32_isize);
_15.2 = core::ptr::addr_of!(_7);
_18 = !_1;
_11 = 7775_u16 - 34842_u16;
_15.0 = !_1;
_13 = _6 as u64;
_1 = !_18;
_12 = (-110860887473031947385384677389902648338_i128) as u32;
_15.3 = (6_usize,);
_9 = !_15.3.0;
_15.2 = core::ptr::addr_of!(_7);
_16 = [(-21834154251866516009932435589349851870_i128),140937040125400646031740536250386500314_i128,(-52341516074262015520164840613887216582_i128),109499687997920432149241417563304835439_i128,(-123231286304291558476635938557793992644_i128),(-62797088173487706693546984658492093154_i128),(-166026779097715359467413054346934517398_i128),(-147907861328747430935685220512929049192_i128)];
RET = 1729550039640942828_i64 + 3219344265162530347_i64;
_3 = (-34_isize) << _12;
_15.3.0 = _9 - _9;
_19 = _3 as f32;
_15.0 = _18 ^ _18;
_15.1 = 48319427335297714311744883374386007163_i128;
_19 = _6 as f32;
_4 = !(-101_i8);
_11 = 13631_u16 / 61213_u16;
_6 = (-413957678_i32);
_19 = (-3496165354645002410_i64) as f32;
_11 = 51008_u16;
Goto(bb2)
}
bb2 = {
_19 = _13 as f32;
RET = (-3421301194967018742_i64);
Goto(bb3)
}
bb3 = {
_15.3.0 = _10 as usize;
_9 = !_15.3.0;
_7 = !(-5726772795099695140_i64);
_17 = _18;
_5 = 27986_i16;
_22.fld4.fld0.0 = !_18;
_22.fld4.fld4.fld6.0.0 = !_5;
_22.fld4.fld4.fld6.3 = !_15.1;
match _5 {
27986 => bb5,
_ => bb4
}
}
bb4 = {
_19 = _13 as f32;
RET = (-3421301194967018742_i64);
Goto(bb3)
}
bb5 = {
_15.2 = core::ptr::addr_of!(_24.fld6);
_22.fld4.fld4.fld4.1 = _5;
_22.fld2.fld0.1.0 = !_22.fld4.fld0.0;
_24.fld1.0 = _15.0 | _15.0;
_22.fld4.fld4.fld4.1 = _22.fld4.fld4.fld6.0.0 << _3;
_22.fld4.fld4.fld4 = (_17, _5);
_23.fld4.fld6.1 = _9;
_21 = _9 < _15.3.0;
_22.fld2.fld0.1.1 = _22.fld4.fld4.fld4.1;
_23.fld4.fld2 = _3;
_22.fld4.fld0.1 = _6 as i16;
_23.fld4.fld6.2 = !84664680441754988696262579946907043469_u128;
_14 = -_6;
_24.fld1.3 = (_15.3.0,);
_22.fld4.fld4.fld3 = [_21];
_27.fld1.fld4 = (_9,);
_3 = _7 as isize;
_22.fld3 = _11 as u64;
_22.fld2.fld1 = _22.fld4.fld4.fld6.3 as f32;
_2 = '\u{6cc8b}';
_15.0 = _22.fld4.fld4.fld4.0;
Goto(bb6)
}
bb6 = {
_23.fld0.0 = _22.fld4.fld4.fld4.0;
_27.fld1.fld5 = _6 << _22.fld3;
_27.fld1.fld3.0 = (_22.fld4.fld0.1, _15.1);
_23.fld4.fld4 = (_22.fld2.fld0.1.0, _22.fld4.fld4.fld6.0.0);
_24.fld6 = -_7;
_23.fld1.0 = !_24.fld1.3.0;
Call(_22.fld4.fld4.fld0 = fn1(_2, _16, _22.fld4.fld0, _2, _6, _15.0, _17, _3, _22.fld4.fld4.fld6.3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_23.fld4.fld6.3 = _15.1;
_27.fld1.fld3.3 = _22.fld2.fld0.1.1 as i128;
_23.fld0 = _22.fld2.fld0.1;
_24.fld1 = (_21, _22.fld4.fld4.fld6.3, _15.2, _23.fld1);
_26 = [_27.fld1.fld3.0.0,_27.fld1.fld3.0.0,_23.fld0.1,_22.fld2.fld0.1.1,_5,_23.fld0.1,_22.fld4.fld4.fld4.1,_22.fld4.fld0.1];
_15 = _24.fld1;
_22.fld2.fld0.0 = [_11,_11,_11,_11,_11];
_23.fld1 = _24.fld1.3;
_27.fld1.fld1.fld0 = core::ptr::addr_of!(_23.fld4.fld6.2);
_22.fld4.fld4.fld5.1 = [_2,_2];
_22.fld4.fld4.fld5.1 = [_2,_2];
_22.fld4.fld4.fld6 = (_27.fld1.fld3.0, _9, _23.fld4.fld6.2, _24.fld1.1, _2);
_28.1.0 = !_17;
RET = _7;
_24.fld1 = (_21, _23.fld4.fld6.3, _15.2, _27.fld1.fld4);
_31.fld7.fld5.fld3.4 = _2;
_20.fld0 = !_22.fld4.fld4.fld6.2;
_23.fld4.fld6.0 = _22.fld4.fld4.fld6.0;
Goto(bb8)
}
bb8 = {
_27.fld1.fld1.fld1 = _11 as i128;
_31.fld1 = core::ptr::addr_of_mut!(_28.2);
_28.0 = _22.fld2.fld0.0;
_3 = _23.fld4.fld2 ^ _23.fld4.fld2;
_11 = _12 as u16;
_31.fld7.fld7 = (_22.fld4.fld4.fld6.0.0, _23.fld4.fld6.0.1);
_28.1 = (_22.fld4.fld4.fld4.0, _31.fld7.fld7.0);
_23.fld4.fld5 = (_6, _22.fld4.fld4.fld5.1);
_23.fld4.fld4 = (_23.fld0.0, _31.fld7.fld7.0);
_22.fld4.fld4.fld5.0 = _27.fld1.fld5 << _27.fld1.fld3.0.0;
_23.fld4.fld2 = !_3;
_17 = _15.0;
_27.fld1.fld0 = [_12,_12,_12,_12,_12,_12,_12];
Goto(bb9)
}
bb9 = {
_33 = _23.fld4.fld5.1;
_24.fld1 = (_21, _27.fld1.fld3.0.1, _15.2, _15.3);
_23.fld4.fld6 = (_27.fld1.fld3.0, _22.fld4.fld4.fld6.1, _22.fld4.fld4.fld6.2, _27.fld1.fld3.3, _22.fld4.fld4.fld6.4);
_22.fld4.fld4.fld5.0 = -_23.fld4.fld5.0;
_23.fld4.fld6.0.0 = !_27.fld1.fld3.0.0;
_23.fld0.1 = _22.fld2.fld0.1.1 >> _22.fld4.fld4.fld4.1;
_36.2 = [_11,_11,_11,_11,_11];
_31.fld3 = (_23.fld4.fld6.0, _23.fld1.0, _22.fld4.fld4.fld6.2, _15.1, _23.fld4.fld6.4);
_36.1 = _22.fld4.fld0;
_31.fld7.fld0.fld1.fld1 = _23.fld4.fld6.0.1;
_8 = !_22.fld4.fld4.fld6.2;
_31.fld7.fld5.fld1.fld0 = core::ptr::addr_of!(_31.fld7.fld5.fld3.2);
_24.fld1.3 = (_15.3.0,);
_27.fld1.fld5 = _22.fld4.fld4.fld5.0;
_24.fld6 = _7;
_24.fld3 = [_24.fld1.0];
_38 = _12 as f64;
_31.fld7.fld7 = _27.fld1.fld3.0;
_31.fld7.fld5.fld3.0 = (_31.fld3.0.0, _27.fld1.fld3.3);
_31.fld7.fld3 = _31.fld3.2 as i8;
_31.fld7.fld2 = _38 as u128;
_36.1 = _22.fld4.fld4.fld4;
_10 = 75_u8 + 126_u8;
_24.fld0.0 = [_18,_21];
_31.fld7.fld1 = _19 as usize;
_31.fld7.fld0.fld5 = [_10,_10,_10,_10,_10,_10,_10,_10];
_22.fld4.fld4.fld2 = _23.fld4.fld2 & _23.fld4.fld2;
match _36.1.1 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
27986 => bb10,
_ => bb6
}
}
bb10 = {
_22.fld4.fld4.fld4.1 = _22.fld4.fld4.fld6.0.0 & _27.fld1.fld3.0.0;
_36.1 = (_18, _22.fld4.fld4.fld4.1);
_31.fld7.fld7 = (_5, _22.fld4.fld4.fld6.3);
_24.fld2 = core::ptr::addr_of!(_39);
_28.1.1 = _7 as i16;
_22.fld4.fld4.fld4 = (_15.0, _23.fld0.1);
_24.fld5 = _10;
_16 = [_15.1,_31.fld7.fld0.fld1.fld1,_31.fld3.3,_22.fld4.fld4.fld6.3,_31.fld7.fld7.1,_24.fld1.1,_27.fld1.fld3.3,_31.fld7.fld7.1];
_10 = _31.fld3.2 as u8;
_27.fld1.fld3.4 = _31.fld7.fld5.fld3.4;
_22.fld4.fld4.fld6 = (_31.fld7.fld7, _9, _31.fld3.2, _31.fld3.3, _31.fld3.4);
_22.fld4.fld4.fld6.0.1 = _24.fld1.1 & _31.fld7.fld7.1;
_36 = (_28.0, _22.fld2.fld0.1, _28.0, _16, _27.fld1.fld0);
Goto(bb11)
}
bb11 = {
_27.fld0 = _31.fld7.fld2 as i128;
_31.fld7.fld5.fld5 = _6;
_24.fld3 = [_22.fld4.fld4.fld4.0];
_31.fld7.fld5.fld1.fld1 = -_31.fld3.3;
_22.fld3 = _12 as u64;
_36.1.0 = _15.0;
_27.fld1.fld3.0 = (_31.fld7.fld5.fld3.0.0, _27.fld1.fld3.3);
_31.fld7.fld7 = (_22.fld2.fld0.1.1, _15.1);
_31.fld3 = (_23.fld4.fld6.0, _9, _31.fld7.fld2, _15.1, _22.fld4.fld4.fld6.4);
_22.fld2.fld0.3 = [_22.fld4.fld4.fld6.0.1,_27.fld1.fld3.0.1,_15.1,_23.fld4.fld6.0.1,_23.fld4.fld6.3,_27.fld1.fld3.3,_31.fld7.fld7.1,_23.fld4.fld6.0.1];
_22.fld4.fld4.fld6.1 = _31.fld7.fld1 % 5_usize;
_31.fld7.fld4 = _36.4;
_20 = Adt59 { fld0: _22.fld4.fld4.fld6.2 };
_31.fld7.fld5.fld3.3 = _31.fld7.fld0.fld1.fld1 & _22.fld4.fld4.fld6.0.1;
Goto(bb12)
}
bb12 = {
_17 = _36.1.0;
_22.fld6 = (_24.fld0.0,);
_24.fld1.2 = core::ptr::addr_of!(_7);
_23.fld4.fld4.1 = _23.fld0.1 ^ _22.fld2.fld0.1.1;
_31.fld7.fld5.fld4.0 = _31.fld7.fld1 * _31.fld3.1;
_8 = _31.fld7.fld2 | _31.fld3.2;
RET = _7;
_27.fld2 = [_22.fld4.fld4.fld4.0,_21];
_31.fld5 = _8 << _23.fld4.fld4.1;
_31.fld7.fld5.fld2 = [_31.fld7.fld5.fld3.4,_23.fld4.fld6.4,_31.fld7.fld5.fld3.4,_23.fld4.fld6.4,_22.fld4.fld4.fld6.4];
_28.4 = _36.4;
_22.fld0 = !_1;
_24.fld5 = _23.fld4.fld4.0 as u8;
match _22.fld4.fld4.fld6.0.0 {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb5,
4 => bb13,
27986 => bb15,
_ => bb14
}
}
bb13 = {
_27.fld1.fld1.fld1 = _11 as i128;
_31.fld1 = core::ptr::addr_of_mut!(_28.2);
_28.0 = _22.fld2.fld0.0;
_3 = _23.fld4.fld2 ^ _23.fld4.fld2;
_11 = _12 as u16;
_31.fld7.fld7 = (_22.fld4.fld4.fld6.0.0, _23.fld4.fld6.0.1);
_28.1 = (_22.fld4.fld4.fld4.0, _31.fld7.fld7.0);
_23.fld4.fld5 = (_6, _22.fld4.fld4.fld5.1);
_23.fld4.fld4 = (_23.fld0.0, _31.fld7.fld7.0);
_22.fld4.fld4.fld5.0 = _27.fld1.fld5 << _27.fld1.fld3.0.0;
_23.fld4.fld2 = !_3;
_17 = _15.0;
_27.fld1.fld0 = [_12,_12,_12,_12,_12,_12,_12];
Goto(bb9)
}
bb14 = {
_15.2 = core::ptr::addr_of!(_24.fld6);
_22.fld4.fld4.fld4.1 = _5;
_22.fld2.fld0.1.0 = !_22.fld4.fld0.0;
_24.fld1.0 = _15.0 | _15.0;
_22.fld4.fld4.fld4.1 = _22.fld4.fld4.fld6.0.0 << _3;
_22.fld4.fld4.fld4 = (_17, _5);
_23.fld4.fld6.1 = _9;
_21 = _9 < _15.3.0;
_22.fld2.fld0.1.1 = _22.fld4.fld4.fld4.1;
_23.fld4.fld2 = _3;
_22.fld4.fld0.1 = _6 as i16;
_23.fld4.fld6.2 = !84664680441754988696262579946907043469_u128;
_14 = -_6;
_24.fld1.3 = (_15.3.0,);
_22.fld4.fld4.fld3 = [_21];
_27.fld1.fld4 = (_9,);
_3 = _7 as isize;
_22.fld3 = _11 as u64;
_22.fld2.fld1 = _22.fld4.fld4.fld6.3 as f32;
_2 = '\u{6cc8b}';
_15.0 = _22.fld4.fld4.fld4.0;
Goto(bb6)
}
bb15 = {
_31.fld3.0.1 = _31.fld7.fld5.fld3.3;
_22.fld2.fld0.2 = [_11,_11,_11,_11,_11];
_23.fld4.fld6.0 = (_31.fld3.0.0, _27.fld1.fld3.3);
_22.fld2.fld3 = !_10;
_22.fld4.fld4.fld2 = _3 + _23.fld4.fld2;
_22.fld3 = _13 >> _23.fld4.fld4.1;
_23.fld0.1 = _5 | _31.fld7.fld7.0;
_51.fld7.fld4 = _28.4;
_51.fld6 = _12 & _12;
_22.fld4.fld4.fld5 = (_27.fld1.fld5, _23.fld4.fld5.1);
_21 = !_17;
_31.fld7.fld5.fld1 = Adt52 { fld0: _27.fld1.fld1.fld0,fld1: _15.1 };
_32 = [_9,_31.fld7.fld5.fld4.0];
_27.fld1 = Adt54 { fld0: _51.fld7.fld4,fld1: Move(_31.fld7.fld5.fld1),fld2: _31.fld7.fld5.fld2,fld3: _22.fld4.fld4.fld6,fld4: _15.3,fld5: _31.fld7.fld5.fld5 };
_44.fld0 = [_11,_11,_11,_11,_11];
_22.fld4.fld4.fld6.0 = (_22.fld4.fld0.1, _31.fld7.fld5.fld3.3);
_23.fld4.fld3 = [_28.1.0];
_15.3 = (_27.fld1.fld3.1,);
_27.fld1.fld3.0.0 = _31.fld7.fld5.fld5 as i16;
_31.fld7.fld0.fld5 = [_22.fld2.fld3,_10,_24.fld5,_24.fld5,_22.fld2.fld3,_22.fld2.fld3,_24.fld5,_24.fld5];
_31.fld7.fld5.fld3.0.1 = _27.fld0 * _23.fld4.fld6.0.1;
_23.fld4.fld2 = _31.fld7.fld1 as isize;
_44.fld2 = [_3,_3,_22.fld4.fld4.fld2,_22.fld4.fld4.fld2,_22.fld4.fld4.fld2];
_40 = _31.fld7.fld5.fld3.4;
_31.fld7.fld0.fld4 = _23.fld0.1;
_23.fld4.fld6.4 = _27.fld1.fld3.4;
Goto(bb16)
}
bb16 = {
Call(_53 = dump_var(0_usize, 6_usize, Move(_6), 40_usize, Move(_40), 10_usize, Move(_10), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(0_usize, 13_usize, Move(_13), 14_usize, Move(_14), 8_usize, Move(_8), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(0_usize, 21_usize, Move(_21), 11_usize, Move(_11), 32_usize, Move(_32), 54_usize, _54), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: char,mut _2: [i128; 8],mut _3: (bool, i16),mut _4: char,mut _5: i32,mut _6: bool,mut _7: bool,mut _8: isize,mut _9: i128) -> [isize; 8] {
mir! {
type RET = [isize; 8];
let _10: char;
let _11: (bool, i16);
let _12: [usize; 2];
let _13: [u32; 7];
let _14: u64;
let _15: Adt48;
let _16: f64;
let _17: *const u128;
let _18: (i16, i128);
let _19: i128;
let _20: [u16; 4];
let _21: Adt54;
let _22: u64;
let _23: i32;
let _24: (i16, i128);
let _25: isize;
let _26: (bool, i128, *const i64, (usize,));
let _27: char;
let _28: f64;
let _29: [u16; 5];
let _30: ([bool; 2],);
let _31: [bool; 1];
let _32: u8;
let _33: i128;
let _34: *const i64;
let _35: [u16; 1];
let _36: [u16; 4];
let _37: char;
let _38: [usize; 2];
let _39: Adt57;
let _40: u128;
let _41: f32;
let _42: Adt56;
let _43: usize;
let _44: isize;
let _45: f32;
let _46: bool;
let _47: [u32; 7];
let _48: Adt55;
let _49: i64;
let _50: *const u128;
let _51: bool;
let _52: [u16; 4];
let _53: [isize; 8];
let _54: usize;
let _55: [u16; 4];
let _56: i32;
let _57: isize;
let _58: [i16; 8];
let _59: f64;
let _60: ();
let _61: ();
{
_9 = 136658053822780018952843232560029311005_i128 & 71711064081263556835181012415048818847_i128;
_1 = _4;
_11.0 = _3.0;
_3.1 = 30727_i16 & (-16030_i16);
_2 = [_9,_9,_9,_9,_9,_9,_9,_9];
_11.1 = _7 as i16;
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_12 = [15069766403405097687_usize,3_usize];
_11 = (_6, _3.1);
_3.1 = _9 as i16;
_9 = (-151697521271050354528854091232947150925_i128);
_13 = [1633079201_u32,1959771046_u32,3416225368_u32,1127175230_u32,3820154392_u32,413306247_u32,3355974470_u32];
_13 = [4161039135_u32,624036219_u32,1328690841_u32,3280223908_u32,4199370451_u32,2481945937_u32,2900555860_u32];
_10 = _4;
_7 = !_11.0;
_4 = _1;
_3 = (_11.0, _11.1);
_11.0 = _6;
_15.fld5.0 = _5 << _11.1;
match _9 {
0 => bb1,
188584845649888108934520516198821060531 => bb3,
_ => bb2
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
_15.fld6.3 = _9 << _11.1;
_15.fld5.1 = [_1,_10];
_11 = (_6, _3.1);
_16 = 49797398182272871_u64 as f64;
_15.fld4.0 = !_6;
_15.fld6.3 = 13356001324368970088_u64 as i128;
_5 = _15.fld5.0 - _15.fld5.0;
_14 = 3154679250997503139_u64 << _5;
_15.fld6.0 = (_3.1, _9);
_15.fld6.4 = _1;
_15.fld6.2 = 17920505156123370630_usize as u128;
_21.fld0 = [3725533202_u32,1943085444_u32,3348997490_u32,4255617058_u32,3179587365_u32,698506700_u32,919932464_u32];
_10 = _15.fld6.4;
_15.fld0 = [_8,_8,_8,_8,_8,_8,_8,_8];
_18.1 = _6 as i128;
_5 = _15.fld5.0 & _15.fld5.0;
_15.fld4.1 = _3.1 ^ _15.fld6.0.0;
_15.fld4.0 = !_3.0;
_4 = _10;
_3.0 = _5 >= _5;
Goto(bb4)
}
bb4 = {
_23 = -_5;
_21.fld5 = _23;
Goto(bb5)
}
bb5 = {
_3.1 = _14 as i16;
_21.fld0 = [1099215463_u32,440715298_u32,2742105568_u32,778347149_u32,2039605443_u32,3345681465_u32,4181363361_u32];
Call(RET = fn2(_15.fld4.1, _15.fld0, _16, _7, _16, _5, _15.fld4.1, _1, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_21.fld3.0.0 = _15.fld4.1 >> _9;
_21.fld3 = (_15.fld6.0, 10275302263044101005_usize, _15.fld6.2, _15.fld6.0.1, _15.fld6.4);
_21.fld3.0.0 = _3.0 as i16;
_15.fld6.3 = -_15.fld6.0.1;
_14 = 17983926622845116062_u64;
_15.fld6.2 = _21.fld3.2 ^ _21.fld3.2;
_17 = core::ptr::addr_of!(_15.fld6.2);
_4 = _15.fld6.4;
_15.fld6 = _21.fld3;
_21.fld1.fld1 = _15.fld6.3;
_15.fld4.1 = _21.fld3.0.0;
_15.fld6.3 = (-16_i8) as i128;
_21.fld3.1 = _3.1 as usize;
_22 = !_14;
_20 = [44676_u16,20983_u16,57962_u16,44960_u16];
_11.1 = _8 as i16;
_15.fld6.0.1 = 9390_u16 as i128;
_21.fld1.fld0 = _17;
_15.fld1 = core::ptr::addr_of_mut!(_29);
Goto(bb7)
}
bb7 = {
_15.fld6.0 = (_15.fld4.1, _21.fld1.fld1);
_15.fld5.0 = -_21.fld5;
_21.fld5 = _5;
_30.0 = [_6,_11.0];
_21.fld2 = [_10,_15.fld6.4,_1,_21.fld3.4,_10];
_11.1 = _16 as i16;
_21.fld4 = (_21.fld3.1,);
_25 = 233_u8 as isize;
_3 = _15.fld4;
_7 = _21.fld3.0.0 != _3.1;
_21.fld3.0 = (_3.1, _21.fld3.3);
RET = _15.fld0;
_26.3 = (_21.fld4.0,);
_21.fld3.0.0 = _3.1;
_1 = _21.fld3.4;
_28 = 162780393_u32 as f64;
_32 = 153_u8 << _3.1;
_11.0 = _7;
_15.fld4 = (_11.0, _15.fld6.0.0);
Goto(bb8)
}
bb8 = {
_20 = [57288_u16,16042_u16,12330_u16,34009_u16];
_27 = _21.fld3.4;
_22 = !_14;
_1 = _27;
_23 = _4 as i32;
_21.fld1.fld1 = -_21.fld3.0.1;
_18.0 = -_15.fld4.1;
_21.fld0 = [581437096_u32,1887023621_u32,278027433_u32,1889852526_u32,3585692383_u32,1537305329_u32,352471721_u32];
_21.fld4.0 = _15.fld6.1 * _26.3.0;
_15.fld6.1 = _26.3.0 % 12281266797993969902_usize;
_14 = !_22;
_39.fld0.4 = _21.fld0;
_33 = _18.1;
Goto(bb9)
}
bb9 = {
_15.fld2 = 105_i8 as isize;
_21.fld2 = [_10,_4,_1,_1,_4];
_18.0 = _21.fld3.1 as i16;
_26.3 = _21.fld4;
_21.fld3.0.0 = _15.fld6.0.0 << _3.1;
_41 = (*_17) as f32;
_39.fld0.3 = [_15.fld6.3,_21.fld1.fld1,_21.fld3.0.1,_18.1,_9,_18.1,_15.fld6.3,_21.fld3.3];
_24.0 = !_15.fld4.1;
_21.fld3.2 = !_15.fld6.2;
_15.fld3 = [_15.fld4.0];
_20 = [37881_u16,20009_u16,51082_u16,41405_u16];
_15.fld6.0.0 = _15.fld4.1 ^ _21.fld3.0.0;
_39.fld0.0 = [43022_u16,39251_u16,60416_u16,21090_u16,14047_u16];
_42.fld7.fld6 = _15.fld0;
_36 = [31646_u16,30067_u16,33389_u16,39571_u16];
_42.fld2 = core::ptr::addr_of_mut!(_28);
_3.0 = !_15.fld4.0;
_42.fld7.fld4 = [3578407688_u32,3537463801_u32,2137909384_u32,3228574964_u32,2798642586_u32,3085322316_u32,4209605586_u32];
_17 = core::ptr::addr_of!(_42.fld7.fld2);
_15.fld6.4 = _21.fld3.4;
RET = [_15.fld2,_25,_25,_8,_8,_15.fld2,_8,_8];
_26.3.0 = !_21.fld4.0;
_42.fld7.fld5.fld3.0.0 = _21.fld4.0 as i16;
_36 = [23085_u16,5155_u16,43893_u16,48677_u16];
_42.fld7.fld5.fld2 = [_10,_27,_4,_21.fld3.4,_1];
match _21.fld3.0.1 {
0 => bb1,
1 => bb8,
188584845649888108934520516198821060531 => bb10,
_ => bb3
}
}
bb10 = {
_39.fld0.1.1 = _15.fld4.1 << _24.0;
Call(_42.fld5 = core::intrinsics::transmute(_21.fld3.0.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15.fld6 = (_21.fld3.0, _26.3.0, _42.fld5, _21.fld3.0.1, _27);
_42.fld7.fld0.fld1 = Adt52 { fld0: _21.fld1.fld0,fld1: _21.fld1.fld1 };
_42.fld7.fld0.fld1 = Adt52 { fld0: _21.fld1.fld0,fld1: _9 };
_21.fld3.0.0 = _15.fld6.0.0;
_42.fld7.fld5.fld3.0.1 = _32 as i128;
_42.fld7.fld5 = Move(_21);
_42.fld7.fld4 = _42.fld7.fld5.fld0;
_19 = 53_i8 as i128;
_42.fld7.fld0.fld6 = [_9,_15.fld6.3,_9,_42.fld7.fld5.fld3.3];
_26.0 = _15.fld4.1 < _15.fld6.0.0;
_9 = _42.fld7.fld5.fld1.fld1;
_42.fld7.fld5.fld3.0.1 = !_15.fld6.3;
_45 = -_41;
_9 = _42.fld7.fld0.fld1.fld1 - _42.fld7.fld0.fld1.fld1;
_47 = _39.fld0.4;
_48.fld0.fld0 = !_11.0;
_26.0 = _32 > _32;
match _42.fld7.fld5.fld3.3 {
0 => bb4,
188584845649888108934520516198821060531 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_15.fld5.1 = [_15.fld6.4,_10];
_42.fld0 = !_32;
_30.0 = [_48.fld0.fld0,_11.0];
_42.fld0 = _32 + _32;
_48.fld5.fld3 = _15.fld6;
_42.fld7.fld6 = [_25,_25,_25,_25,_25,_25,_15.fld2,_8];
_48.fld5.fld4.0 = _15.fld5.0 as usize;
_48.fld0.fld1 = Adt52 { fld0: _42.fld7.fld5.fld1.fld0,fld1: _33 };
_24.0 = -_3.1;
_24 = (_48.fld5.fld3.0.0, _48.fld5.fld3.3);
_48.fld7 = (_15.fld4.1, _9);
_39.fld0.1 = (_11.0, _48.fld5.fld3.0.0);
_42.fld7.fld5.fld4 = (_48.fld5.fld3.1,);
_48.fld5.fld3.1 = _42.fld7.fld5.fld3.1 / 6975842368068017019_usize;
_9 = _33;
_18.0 = _48.fld7.0;
_33 = _19;
_42.fld7.fld5.fld3 = _15.fld6;
_42.fld7.fld0.fld1.fld0 = core::ptr::addr_of!(_15.fld6.2);
_54 = _26.3.0;
Goto(bb14)
}
bb14 = {
_42.fld7.fld1 = _42.fld7.fld5.fld3.2 as usize;
_42.fld4 = _14 ^ _14;
_15.fld4.0 = _7 & _11.0;
_3.1 = -_24.0;
_3.0 = _15.fld4.0;
_48.fld0.fld6 = _42.fld7.fld0.fld6;
_13 = _39.fld0.4;
Goto(bb15)
}
bb15 = {
Call(_60 = dump_var(1_usize, 19_usize, Move(_19), 6_usize, Move(_6), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_60 = dump_var(1_usize, 3_usize, Move(_3), 9_usize, Move(_9), 23_usize, Move(_23), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_60 = dump_var(1_usize, 33_usize, Move(_33), 25_usize, Move(_25), 47_usize, Move(_47), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_60 = dump_var(1_usize, 2_usize, Move(_2), 22_usize, Move(_22), 61_usize, _61, 61_usize, _61), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i16,mut _2: [isize; 8],mut _3: f64,mut _4: bool,mut _5: f64,mut _6: i32,mut _7: i16,mut _8: char,mut _9: [u32; 7]) -> [isize; 8] {
mir! {
type RET = [isize; 8];
let _10: [u32; 7];
let _11: Adt51;
let _12: bool;
let _13: isize;
let _14: isize;
let _15: i16;
let _16: isize;
let _17: char;
let _18: [u32; 7];
let _19: [isize; 8];
let _20: f32;
let _21: *const f32;
let _22: *mut [u16; 5];
let _23: Adt54;
let _24: ([bool; 2],);
let _25: Adt55;
let _26: char;
let _27: [isize; 5];
let _28: [bool; 2];
let _29: u64;
let _30: u8;
let _31: u128;
let _32: f64;
let _33: i16;
let _34: [u16; 4];
let _35: [i128; 8];
let _36: [usize; 2];
let _37: Adt53;
let _38: *const f32;
let _39: i32;
let _40: isize;
let _41: f64;
let _42: *mut [u8; 8];
let _43: (bool, i16);
let _44: char;
let _45: [isize; 5];
let _46: Adt63;
let _47: i8;
let _48: [bool; 2];
let _49: char;
let _50: [u8; 8];
let _51: ();
let _52: ();
{
RET = [9223372036854775807_isize,(-43_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-29_isize),(-54_isize),(-9223372036854775808_isize)];
_5 = _3 * _3;
_7 = 2542472533276254186_u64 as i16;
_2 = [84_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-119_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = _2;
_1 = !_7;
_5 = _3;
_1 = 1307050050517245177_i64 as i16;
_8 = '\u{c9d2f}';
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),(-69_isize),116_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,95_isize,(-63_isize),9223372036854775807_isize];
_8 = '\u{5b08}';
_2 = [9223372036854775807_isize,117_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [(-42_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,93_isize,(-49_isize)];
_2 = [9223372036854775807_isize,9223372036854775807_isize,86_isize,9223372036854775807_isize,48_isize,9223372036854775807_isize,(-106_isize),(-79_isize)];
Call(_6 = fn3(_3, _3, _9, _8, _7, _3, _2, _8, _5, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _2;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,32_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-52_isize),(-9223372036854775808_isize)];
_5 = _3;
_9 = [2746521484_u32,3468703356_u32,3117042171_u32,1363520899_u32,639866905_u32,1367713570_u32,3845373051_u32];
_2 = [12_isize,(-9223372036854775808_isize),88_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,42_isize,(-43_isize)];
RET = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),33_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-100_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-4_isize),9223372036854775807_isize];
_4 = true;
_3 = _5;
Goto(bb2)
}
bb2 = {
_8 = '\u{71733}';
_11.fld1.3 = (206846473460988414_usize,);
_7 = _1;
_3 = 64032_u16 as f64;
_13 = (-34_isize) & 9223372036854775807_isize;
_5 = -_3;
_11.fld1.3.0 = !5_usize;
Goto(bb3)
}
bb3 = {
_9 = [1315510787_u32,2719686720_u32,13916678_u32,2825163405_u32,1413806800_u32,2704198092_u32,326533711_u32];
_14 = -_13;
_1 = !_7;
_11.fld3 = [_4];
Call(_11.fld0.0 = fn5(_9, _6, _14, _11.fld1.3.0, _14, _4, _2, _14, _3, _5, _6, _14, _9), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = _5 + _5;
_12 = !_4;
_11.fld5 = 74_u8 & 118_u8;
_9 = [87221760_u32,160319688_u32,683666268_u32,580328818_u32,2685006956_u32,2356149355_u32,2044519641_u32];
_12 = _4;
_15 = -_1;
_11.fld1.2 = core::ptr::addr_of!(_11.fld6);
_11.fld1.1 = _11.fld1.3.0 as i128;
_11.fld1.0 = !_4;
_16 = _14 << _13;
_7 = _15;
_18 = [1270541650_u32,3658579949_u32,1094634327_u32,2382295255_u32,20771489_u32,509125756_u32,1653913240_u32];
_11.fld1.2 = core::ptr::addr_of!(_11.fld6);
_19 = _2;
_9 = [4165909732_u32,1917785062_u32,1938500964_u32,3881835561_u32,1556173895_u32,3691350133_u32,1712158590_u32];
_11.fld1.0 = _11.fld5 > _11.fld5;
_11.fld1.2 = core::ptr::addr_of!(_11.fld6);
RET = [_13,_16,_14,_16,_14,_13,_14,_16];
_8 = '\u{55650}';
_1 = _7 >> _14;
_11.fld6 = 1996879428903295456_i64 + (-5814948848055171422_i64);
_11.fld1.0 = !_4;
_13 = _16 & _14;
_2 = [_16,_13,_13,_13,_14,_16,_16,_13];
RET = [_16,_13,_14,_16,_13,_16,_14,_13];
_17 = _8;
_16 = 41643257983218909960098020187895642655_u128 as isize;
Goto(bb5)
}
bb5 = {
_20 = _11.fld5 as f32;
_10 = [3076003614_u32,2554960338_u32,2623680664_u32,2630717159_u32,2531181722_u32,201625828_u32,3291527034_u32];
_7 = _11.fld6 as i16;
_8 = _17;
_11.fld1.3.0 = !5_usize;
_9 = [2606595704_u32,929929079_u32,199593356_u32,1975444701_u32,2760627402_u32,2094829155_u32,1696990350_u32];
_9 = [362323413_u32,2494066885_u32,1401706310_u32,2439079058_u32,909945823_u32,3743973648_u32,2922885062_u32];
_11.fld3 = [_4];
_21 = core::ptr::addr_of!(_20);
_11.fld1.1 = (-16302160558428219351423933457558984408_i128) - (-76880188194002751851038783150040518539_i128);
_21 = core::ptr::addr_of!((*_21));
_23.fld3.0.1 = !_11.fld1.1;
_1 = 97970900891801098458661949264001062312_u128 as i16;
_23.fld3.0.0 = _1;
_24 = (_11.fld0.0,);
_25.fld5.fld2 = [_8,_8,_8,_8,_8];
_23.fld4.0 = _11.fld1.3.0 >> _23.fld3.0.1;
_23.fld3.1 = _23.fld4.0;
_11.fld2 = core::ptr::addr_of!(_25.fld5.fld2);
_21 = core::ptr::addr_of!(_20);
Call(_23.fld3.3 = fn18(_21, _2, _20, _23.fld3.0, _8, _11.fld0, _11.fld6, _24.0, _2, _11.fld1.3.0, _8, _14), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_25.fld0.fld2 = [_1,_15,_15,_15,_7,_1,_7,_1];
_25.fld0.fld1.fld0 = core::ptr::addr_of!(_25.fld2);
_25.fld5.fld3.0.0 = _11.fld6 as i16;
_26 = _17;
_25.fld0.fld5 = [_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5];
_25.fld5.fld5 = !_6;
_11.fld4 = core::ptr::addr_of_mut!(_25.fld0.fld5);
_11.fld4 = core::ptr::addr_of_mut!(_25.fld0.fld5);
_25.fld5.fld3.0 = (_15, _23.fld3.0.1);
_25.fld1 = _23.fld3.1;
_25.fld5.fld0 = [1959521295_u32,3777714326_u32,125193546_u32,2264291786_u32,699512633_u32,2110313689_u32,1623095751_u32];
_25.fld0.fld3 = core::ptr::addr_of_mut!(_3);
_21 = core::ptr::addr_of!((*_21));
_11.fld1.2 = core::ptr::addr_of!(_11.fld6);
_25.fld7.1 = _25.fld5.fld3.0.1;
_25.fld5.fld1.fld1 = 1948219939367211623850802335594264967_u128 as i128;
_3 = _5;
_14 = _23.fld3.0.1 as isize;
_18 = [4181454561_u32,2229226126_u32,970385652_u32,4072547376_u32,2766652559_u32,2737454646_u32,1830058136_u32];
_27 = [_13,_13,_14,_13,_13];
_1 = _7;
_25.fld5.fld3.3 = _23.fld3.3 << _23.fld3.3;
_25.fld7.0 = _25.fld5.fld3.0.0 - _15;
Goto(bb7)
}
bb7 = {
_28 = [_12,_11.fld1.0];
(*_21) = (-111_i8) as f32;
_9 = [931508380_u32,3637185779_u32,1247856711_u32,1612169652_u32,1093070238_u32,455309602_u32,1182429064_u32];
_26 = _17;
_25.fld0.fld0 = _11.fld1.0;
_24 = (_28,);
_25.fld0.fld2 = [_7,_25.fld7.0,_1,_25.fld7.0,_7,_25.fld7.0,_7,_7];
_11.fld1.0 = _25.fld5.fld3.3 > _25.fld5.fld3.3;
_25.fld6 = _19;
_25.fld3 = _23.fld3.3 as i8;
_25.fld5.fld2 = [_26,_26,_17,_17,_26];
_2 = _19;
_25.fld0.fld3 = core::ptr::addr_of_mut!(_5);
Call(_37.fld6 = core::intrinsics::transmute(_2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_36 = [_23.fld4.0,_23.fld3.1];
_25.fld7.0 = _5 as i16;
_34 = [26289_u16,23950_u16,36520_u16,28263_u16];
_11.fld3 = [_11.fld1.0];
_37.fld5 = _25.fld0.fld5;
_23.fld1.fld1 = _25.fld5.fld3.3;
_25.fld0.fld6 = [_25.fld5.fld3.0.1,_23.fld3.0.1,_23.fld1.fld1,_23.fld3.3];
_37.fld1.fld1 = _25.fld5.fld3.0.1;
_25.fld5.fld4.0 = _23.fld3.1;
_23.fld0 = [369111470_u32,2503361908_u32,3230549957_u32,2550767106_u32,783773019_u32,1333111659_u32,2373968900_u32];
_23.fld3.0.1 = !_25.fld5.fld3.3;
_23.fld3 = (_25.fld7, _25.fld1, 275788459117491294208145053046034331644_u128, _25.fld5.fld3.3, _26);
_25.fld0.fld0 = _11.fld1.0 | _12;
_11.fld5 = !105_u8;
Goto(bb9)
}
bb9 = {
_38 = _21;
_8 = _17;
_23.fld3.0.0 = -_7;
_23.fld5 = _6 >> _13;
_4 = _11.fld1.0 ^ _11.fld1.0;
(*_21) = 1940922877_u32 as f32;
RET = [_13,_13,_13,_13,_13,_13,_14,_14];
_29 = 17411137153533565895_u64;
_2 = _19;
_25.fld5.fld3.0.1 = _23.fld1.fld1 | _23.fld1.fld1;
_25.fld5.fld3.0.1 = -_23.fld1.fld1;
match _29 {
0 => bb7,
17411137153533565895 => bb10,
_ => bb4
}
}
bb10 = {
_40 = _25.fld5.fld3.3 as isize;
_10 = _18;
_23.fld3.0.0 = _11.fld6 as i16;
_18 = _10;
_46.fld4.fld4.fld6.3 = 46761_u16 as i128;
match _23.fld3.2 {
0 => bb9,
1 => bb2,
2 => bb6,
3 => bb5,
4 => bb11,
275788459117491294208145053046034331644 => bb13,
_ => bb12
}
}
bb11 = {
_38 = _21;
_8 = _17;
_23.fld3.0.0 = -_7;
_23.fld5 = _6 >> _13;
_4 = _11.fld1.0 ^ _11.fld1.0;
(*_21) = 1940922877_u32 as f32;
RET = [_13,_13,_13,_13,_13,_13,_14,_14];
_29 = 17411137153533565895_u64;
_2 = _19;
_25.fld5.fld3.0.1 = _23.fld1.fld1 | _23.fld1.fld1;
_25.fld5.fld3.0.1 = -_23.fld1.fld1;
match _29 {
0 => bb7,
17411137153533565895 => bb10,
_ => bb4
}
}
bb12 = {
_36 = [_23.fld4.0,_23.fld3.1];
_25.fld7.0 = _5 as i16;
_34 = [26289_u16,23950_u16,36520_u16,28263_u16];
_11.fld3 = [_11.fld1.0];
_37.fld5 = _25.fld0.fld5;
_23.fld1.fld1 = _25.fld5.fld3.3;
_25.fld0.fld6 = [_25.fld5.fld3.0.1,_23.fld3.0.1,_23.fld1.fld1,_23.fld3.3];
_37.fld1.fld1 = _25.fld5.fld3.0.1;
_25.fld5.fld4.0 = _23.fld3.1;
_23.fld0 = [369111470_u32,2503361908_u32,3230549957_u32,2550767106_u32,783773019_u32,1333111659_u32,2373968900_u32];
_23.fld3.0.1 = !_25.fld5.fld3.3;
_23.fld3 = (_25.fld7, _25.fld1, 275788459117491294208145053046034331644_u128, _25.fld5.fld3.3, _26);
_25.fld0.fld0 = _11.fld1.0 | _12;
_11.fld5 = !105_u8;
Goto(bb9)
}
bb13 = {
_46.fld5 = _23.fld5 << _23.fld3.0.1;
_27 = [_40,_40,_13,_13,_13];
_46.fld4.fld1.0 = _25.fld3 as usize;
_46.fld2.fld0.2 = [16385_u16,39891_u16,10525_u16,63556_u16,30610_u16];
_25.fld5.fld4 = _46.fld4.fld1;
_46.fld4.fld1 = _25.fld5.fld4;
_25.fld0.fld1.fld1 = _23.fld1.fld1 << _23.fld1.fld1;
_11.fld6 = 3451180829683365465_i64;
_12 = !_11.fld1.0;
_36 = [_23.fld3.1,_23.fld4.0];
_35 = [_23.fld1.fld1,_25.fld0.fld1.fld1,_23.fld3.3,_25.fld0.fld1.fld1,_37.fld1.fld1,_25.fld0.fld1.fld1,_25.fld0.fld1.fld1,_25.fld0.fld1.fld1];
_46.fld2.fld0.4 = [3187053457_u32,2074302712_u32,3807780664_u32,676389784_u32,710828296_u32,4180655907_u32,991277673_u32];
_43 = (_12, _7);
_25.fld5.fld3.1 = 1774289801_u32 as usize;
_16 = !_40;
_25.fld0.fld0 = _4;
_25.fld5.fld1.fld0 = core::ptr::addr_of!(_46.fld4.fld4.fld6.2);
Goto(bb14)
}
bb14 = {
_46.fld4.fld4.fld6.0.1 = _25.fld5.fld1.fld1;
_39 = 19715_u16 as i32;
_1 = _8 as i16;
_37.fld0 = _43.0 ^ _12;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(2_usize, 13_usize, Move(_13), 24_usize, Move(_24), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(2_usize, 2_usize, Move(_2), 14_usize, Move(_14), 39_usize, Move(_39), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(2_usize, 26_usize, Move(_26), 29_usize, Move(_29), 36_usize, Move(_36), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(2_usize, 6_usize, Move(_6), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: f64,mut _2: f64,mut _3: [u32; 7],mut _4: char,mut _5: i16,mut _6: f64,mut _7: [isize; 8],mut _8: char,mut _9: f64,mut _10: f64) -> i32 {
mir! {
type RET = i32;
let _11: i64;
let _12: isize;
let _13: Adt63;
let _14: Adt49;
let _15: i64;
let _16: i64;
let _17: isize;
let _18: Adt48;
let _19: i128;
let _20: u32;
let _21: Adt52;
let _22: Adt50;
let _23: [isize; 5];
let _24: f32;
let _25: f64;
let _26: [i16; 8];
let _27: Adt60;
let _28: isize;
let _29: u32;
let _30: Adt55;
let _31: i128;
let _32: isize;
let _33: Adt58;
let _34: [isize; 5];
let _35: ();
let _36: ();
{
_2 = _10 / 1_f64;
RET = 462448767_i32;
Call(_9 = fn4(_2, _7, _1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = -25429_i16;
_2 = _9;
Goto(bb2)
}
bb2 = {
_4 = _8;
_6 = _2 - _9;
_5 = 21408_i16 - 12024_i16;
_4 = _8;
RET = 195540422_i32;
_3 = [761358946_u32,976773791_u32,2274489985_u32,1389414890_u32,2355043234_u32,1835505131_u32,2246531503_u32];
_5 = !(-31106_i16);
Goto(bb3)
}
bb3 = {
_8 = _4;
_9 = -_6;
_7 = [(-30_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),113_isize,(-89_isize),122_isize,12_isize];
_3 = [153779666_u32,2618894815_u32,2476452212_u32,4059266878_u32,3145905485_u32,574234431_u32,3697549502_u32];
_13.fld2.fld0.4 = [1088478440_u32,3706675125_u32,2218164064_u32,1636728694_u32,948221012_u32,756208797_u32,2237638963_u32];
_14.fld0.fld5.0 = !152985908_i32;
_3 = _13.fld2.fld0.4;
_13.fld4.fld1 = (3_usize,);
_14.fld0.fld4.0 = true;
_4 = _8;
_13.fld2.fld0.3 = [29134367386004844278126436395449458963_i128,(-82724447202224081043438536558437623369_i128),(-85316817890433670692991818418484913898_i128),(-36412949429154550821883084034688449015_i128),27559401303586994085425452732279040762_i128,32727279566937032647979595137827226273_i128,(-29508384695213309007473877975274356363_i128),(-146110879352467263578575374600926195131_i128)];
_13.fld4.fld4.fld4.1 = (-20_i8) as i16;
_3 = [2684961475_u32,2698961949_u32,2067332178_u32,1087378498_u32,3408326526_u32,124313470_u32,3189679061_u32];
_13.fld6.0 = [_14.fld0.fld4.0,_14.fld0.fld4.0];
_13.fld4.fld4.fld2 = (-9223372036854775808_isize) | 9223372036854775807_isize;
_4 = _8;
_12 = _13.fld4.fld4.fld2 + _13.fld4.fld4.fld2;
_13.fld4.fld4.fld5.1 = [_4,_8];
_14.fld0.fld6.0 = (_13.fld4.fld4.fld4.1, 152702433890278738898191545300668893581_i128);
_13.fld4.fld4.fld6.2 = 53_u8 as u128;
_4 = _8;
_13.fld4.fld4.fld5.0 = 78_i8 as i32;
_13.fld4.fld4.fld6.0.0 = _13.fld4.fld4.fld4.1;
match _14.fld0.fld6.0.1 {
0 => bb4,
1 => bb5,
152702433890278738898191545300668893581 => bb7,
_ => bb6
}
}
bb4 = {
_4 = _8;
_6 = _2 - _9;
_5 = 21408_i16 - 12024_i16;
_4 = _8;
RET = 195540422_i32;
_3 = [761358946_u32,976773791_u32,2274489985_u32,1389414890_u32,2355043234_u32,1835505131_u32,2246531503_u32];
_5 = !(-31106_i16);
Goto(bb3)
}
bb5 = {
_5 = -25429_i16;
_2 = _9;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_13.fld2.fld1 = 214_u8 as f32;
_13.fld4.fld4.fld6.3 = -_14.fld0.fld6.0.1;
_13.fld5 = _13.fld4.fld4.fld5.0;
_13.fld4.fld4.fld3 = [_14.fld0.fld4.0];
_14.fld0.fld6.0.0 = !_13.fld4.fld4.fld4.1;
_3 = _13.fld2.fld0.4;
_12 = _13.fld4.fld4.fld2 ^ _13.fld4.fld4.fld2;
match _13.fld4.fld1.0 {
0 => bb1,
1 => bb3,
3 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_13.fld2.fld0.0 = [12455_u16,23239_u16,17448_u16,28751_u16,37914_u16];
_13.fld4.fld4.fld4 = (_14.fld0.fld4.0, _5);
_13.fld2.fld0.1.0 = _6 == _6;
_13.fld2.fld3 = 67_u8 >> _12;
_13.fld2.fld2 = !1106142138_u32;
_14.fld0.fld2 = -_13.fld4.fld4.fld2;
_17 = -_12;
_14.fld0.fld0 = [_12,_17,_12,_13.fld4.fld4.fld2,_12,_13.fld4.fld4.fld2,_17,_12];
_13.fld4.fld2 = core::ptr::addr_of_mut!(_13.fld2.fld0.2);
_16 = _13.fld4.fld4.fld2 as i64;
_13.fld4.fld0.0 = !_13.fld2.fld0.1.0;
_13.fld2.fld0.0 = [48753_u16,48454_u16,26010_u16,42707_u16,64076_u16];
_18.fld4.0 = _9 != _9;
_14.fld0.fld6.0.1 = _13.fld4.fld4.fld6.2 as i128;
_13.fld4.fld4.fld6.2 = _6 as u128;
_13.fld4.fld4.fld2 = 48833_u16 as isize;
_18.fld1 = core::ptr::addr_of_mut!(_13.fld2.fld0.2);
_14.fld1 = [_12,_14.fld0.fld2,_13.fld4.fld4.fld2,_12,_12];
_13.fld4.fld4.fld6.1 = _13.fld4.fld1.0 + _13.fld4.fld1.0;
_14.fld0.fld6.0 = (_13.fld4.fld4.fld6.0.0, _13.fld4.fld4.fld6.3);
_13.fld4.fld4.fld6.0.1 = _13.fld4.fld4.fld6.3 & _13.fld4.fld4.fld6.3;
_14.fld0.fld3 = [_18.fld4.0];
_18.fld6.2 = _13.fld4.fld4.fld6.2 | _13.fld4.fld4.fld6.2;
_13.fld4.fld4.fld6.1 = _13.fld4.fld4.fld4.1 as usize;
Goto(bb10)
}
bb10 = {
_4 = _8;
_1 = _6 + _9;
_13.fld4.fld4.fld6.3 = _14.fld0.fld6.0.1;
_8 = _4;
_13.fld4.fld4.fld6.3 = _13.fld2.fld1 as i128;
_13.fld2.fld2 = 388067808_u32;
_13.fld4.fld0.0 = _13.fld4.fld4.fld4.1 > _13.fld4.fld4.fld6.0.0;
_14.fld0.fld5.1 = [_4,_4];
_20 = 44741_u16 as u32;
_13.fld3 = 17800361987163994136_u64 / 4727849087124654668_u64;
_14.fld0.fld6.0 = (_13.fld4.fld4.fld6.0.0, _13.fld4.fld4.fld6.0.1);
_13.fld4.fld4.fld4 = (_18.fld4.0, _14.fld0.fld6.0.0);
_13.fld2.fld0.1 = (_18.fld4.0, _13.fld4.fld4.fld4.1);
_18.fld0 = [_17,_12,_12,_12,_12,_14.fld0.fld2,_12,_12];
_14.fld0.fld6.2 = _18.fld6.2;
_14.fld0.fld4 = (_13.fld2.fld0.1.0, _13.fld4.fld4.fld6.0.0);
_18.fld6.0.0 = !_14.fld0.fld6.0.0;
_18.fld4.0 = _13.fld4.fld4.fld4.0;
RET = _13.fld4.fld4.fld5.0;
_8 = _4;
_12 = _14.fld0.fld5.0 as isize;
_21.fld0 = core::ptr::addr_of!(_13.fld4.fld4.fld6.2);
_13.fld2.fld0.2 = [8305_u16,26366_u16,44079_u16,24136_u16,1223_u16];
_13.fld6.0 = [_13.fld4.fld4.fld4.0,_13.fld4.fld4.fld4.0];
_14.fld2 = _13.fld3;
_13.fld5 = _13.fld4.fld4.fld5.0;
_14.fld0.fld6.2 = _13.fld4.fld4.fld6.2 * _18.fld6.2;
match _13.fld2.fld2 {
388067808 => bb12,
_ => bb11
}
}
bb11 = {
_4 = _8;
_6 = _2 - _9;
_5 = 21408_i16 - 12024_i16;
_4 = _8;
RET = 195540422_i32;
_3 = [761358946_u32,976773791_u32,2274489985_u32,1389414890_u32,2355043234_u32,1835505131_u32,2246531503_u32];
_5 = !(-31106_i16);
Goto(bb3)
}
bb12 = {
_22.fld4 = [_8,_4,_8,_8,_8];
_18.fld5.0 = _13.fld5;
_18.fld6.0 = _13.fld4.fld4.fld6.0;
_18.fld2 = _12;
_2 = _1;
_19 = _16 as i128;
Call(_14.fld0.fld5.0 = core::intrinsics::bswap(_18.fld5.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_19 = !_13.fld4.fld4.fld6.3;
_13.fld4.fld4.fld4.1 = _4 as i16;
_14.fld0.fld6.0.0 = !_13.fld4.fld4.fld6.0.0;
_18.fld6.0.1 = _13.fld4.fld4.fld6.3 + _13.fld4.fld4.fld6.0.1;
_14.fld0.fld6.3 = !_14.fld0.fld6.0.1;
_22.fld3 = (_13.fld6.0,);
_15 = _14.fld0.fld6.3 as i64;
_13.fld4.fld4.fld6.3 = _14.fld0.fld6.3;
_13.fld5 = _14.fld0.fld5.0 << _18.fld6.2;
_13.fld0 = _9 <= _9;
_14.fld0.fld5.0 = !_13.fld5;
_23 = [_18.fld2,_17,_17,_17,_14.fld0.fld2];
_22.fld3 = _13.fld6;
_15 = !_16;
_18.fld5.1 = _14.fld0.fld5.1;
_22.fld2 = _23;
_27.fld2 = core::ptr::addr_of_mut!(_13.fld2.fld0.0);
_13.fld4.fld0.1 = _13.fld2.fld0.1.1 - _5;
_27.fld1 = (_13.fld4.fld1.0,);
_18.fld5.1 = [_8,_8];
_27.fld4.fld6.0.0 = !_13.fld4.fld4.fld4.1;
_27.fld4.fld5 = _14.fld0.fld5;
match _13.fld4.fld1.0 {
3 => bb15,
_ => bb14
}
}
bb14 = {
_4 = _8;
_6 = _2 - _9;
_5 = 21408_i16 - 12024_i16;
_4 = _8;
RET = 195540422_i32;
_3 = [761358946_u32,976773791_u32,2274489985_u32,1389414890_u32,2355043234_u32,1835505131_u32,2246531503_u32];
_5 = !(-31106_i16);
Goto(bb3)
}
bb15 = {
_21.fld1 = _13.fld4.fld4.fld6.3;
_21.fld0 = core::ptr::addr_of!(_18.fld6.2);
_30.fld4 = [_13.fld2.fld2,_20,_20,_20,_20,_13.fld2.fld2,_20];
_33.fld0.fld0 = (_13.fld2.fld0.2, _13.fld4.fld4.fld4, _13.fld2.fld0.2, _13.fld2.fld0.3, _30.fld4);
_22.fld1 = _13.fld2.fld3 ^ _13.fld2.fld3;
_13.fld4.fld2 = core::ptr::addr_of_mut!(_22.fld0);
_13.fld1 = [_12,_12,_12,_18.fld2,_17,_17,_17,_12];
_30.fld4 = [_13.fld2.fld2,_13.fld2.fld2,_20,_20,_13.fld2.fld2,_13.fld2.fld2,_13.fld2.fld2];
_13.fld0 = _18.fld4.0;
Goto(bb16)
}
bb16 = {
Call(_35 = dump_var(3_usize, 5_usize, Move(_5), 20_usize, Move(_20), 15_usize, Move(_15), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(3_usize, 23_usize, Move(_23), 3_usize, Move(_3), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: f64,mut _2: [isize; 8],mut _3: f64,mut _4: char) -> f64 {
mir! {
type RET = f64;
let _5: *const i64;
let _6: f32;
let _7: ();
let _8: ();
{
RET = 170178442780916984430015446363170170880_u128 as f64;
RET = _1;
_3 = _1 / 1_f64;
_1 = _3 * _3;
_6 = _1 as f32;
_4 = '\u{66ba1}';
_4 = '\u{c1e0}';
_3 = -_1;
_3 = _1 * _1;
_2 = [(-9223372036854775808_isize),35_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-127_isize),21_isize,(-98_isize)];
_1 = -_3;
RET = _3 / f64::NAN;
RET = _3;
RET = -_3;
_3 = -_1;
_6 = 114798271328763855671588535514642024952_u128 as f32;
_4 = '\u{650f1}';
_6 = 4_usize as f32;
_4 = '\u{94c28}';
_6 = 54913_u16 as f32;
_6 = 1307807819_i32 as f32;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(4_usize, 4_usize, Move(_4), 8_usize, _8, 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [u32; 7],mut _2: i32,mut _3: isize,mut _4: usize,mut _5: isize,mut _6: bool,mut _7: [isize; 8],mut _8: isize,mut _9: f64,mut _10: f64,mut _11: i32,mut _12: isize,mut _13: [u32; 7]) -> [bool; 2] {
mir! {
type RET = [bool; 2];
let _14: isize;
let _15: Adt56;
let _16: [usize; 2];
let _17: [u16; 1];
let _18: Adt50;
let _19: f64;
let _20: [char; 2];
let _21: i32;
let _22: Adt64;
let _23: Adt53;
let _24: f64;
let _25: f32;
let _26: *const [char; 5];
let _27: i128;
let _28: [isize; 5];
let _29: Adt60;
let _30: [usize; 2];
let _31: isize;
let _32: ();
let _33: ();
{
_15.fld7.fld5.fld3.1 = _6 as usize;
_15.fld7.fld5.fld3.0 = ((-14540_i16), (-57405645566841038884444589715846772695_i128));
_15.fld7.fld5.fld3.0 = ((-21983_i16), 43455926409780245612125181517449252937_i128);
_15.fld7.fld5.fld3.2 = _9 as u128;
_15.fld7.fld5.fld3.3 = _15.fld7.fld5.fld3.0.1 << _5;
_10 = -_9;
_15.fld3 = (_15.fld7.fld5.fld3.0, _4, _15.fld7.fld5.fld3.2, _15.fld7.fld5.fld3.0.1, '\u{ff436}');
_15.fld2 = core::ptr::addr_of_mut!(_10);
match _15.fld7.fld5.fld3.0.1 {
0 => bb1,
1 => bb2,
43455926409780245612125181517449252937 => bb4,
_ => bb3
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
_15.fld2 = core::ptr::addr_of_mut!(_18.fld5);
_15.fld5 = _15.fld3.2;
_18.fld4 = [_15.fld3.4,_15.fld3.4,_15.fld3.4,_15.fld3.4,_15.fld3.4];
_8 = _12 * _5;
_18.fld1 = 26_u8 + 101_u8;
Goto(bb5)
}
bb5 = {
_15.fld7.fld7.1 = _15.fld3.3 >> _15.fld3.2;
Call(_15.fld3.0.0 = fn6(_10, _15.fld3.3, _15.fld2, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_15.fld7.fld1 = _15.fld7.fld5.fld3.1 | _4;
_15.fld7.fld2 = _15.fld5 | _15.fld3.2;
_16 = [_15.fld3.1,_15.fld7.fld1];
_15.fld7.fld5.fld5 = _2 - _2;
_4 = _15.fld7.fld1 / 5522464369314596423_usize;
_6 = !false;
_15.fld7.fld5.fld3.2 = _6 as u128;
_15.fld7.fld5.fld4 = (_4,);
_15.fld3.2 = !_15.fld7.fld5.fld3.2;
_15.fld6 = 3644017493_u32 ^ 973844506_u32;
RET = [_6,_6];
_15.fld4 = 6006359809625903857_u64;
_3 = -_8;
_15.fld3.1 = _4 - _15.fld7.fld1;
_22.fld1.fld4.0 = !_4;
_22.fld1.fld2 = [_15.fld3.4,_15.fld3.4,_15.fld3.4,_15.fld3.4,_15.fld3.4];
_15.fld7.fld5.fld1.fld1 = _15.fld7.fld5.fld3.3 + _15.fld7.fld7.1;
_15.fld1 = core::ptr::addr_of_mut!(_18.fld0);
_23.fld3 = core::ptr::addr_of_mut!(_19);
_2 = -_11;
_15.fld7.fld7 = _15.fld7.fld5.fld3.0;
_23.fld6 = [_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld1.fld1,_15.fld7.fld5.fld1.fld1];
_12 = !_5;
_22.fld2 = [_6,_6];
_22.fld1.fld3.3 = -_15.fld7.fld7.1;
_15.fld2 = core::ptr::addr_of_mut!(_19);
_22.fld1.fld3.0.1 = _15.fld3.3;
match _15.fld7.fld7.1 {
0 => bb3,
43455926409780245612125181517449252937 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_22.fld1.fld3.0 = _15.fld3.0;
_12 = _8;
_22.fld1.fld2 = _18.fld4;
_15.fld7.fld7.1 = _15.fld7.fld5.fld1.fld1 | _15.fld7.fld5.fld1.fld1;
_15.fld7.fld5.fld1.fld0 = core::ptr::addr_of!(_15.fld3.2);
_1 = _13;
_22.fld1.fld3.1 = !_15.fld3.1;
_25 = _15.fld7.fld5.fld5 as f32;
_15.fld7.fld0.fld1.fld0 = core::ptr::addr_of!(_22.fld1.fld3.2);
_15.fld3.0.0 = _15.fld7.fld5.fld3.0.0 - _15.fld7.fld7.0;
_18.fld2 = [_8,_8,_3,_3,_3];
_23.fld1.fld0 = _15.fld7.fld5.fld1.fld0;
_22.fld1 = Adt54 { fld0: _13,fld1: Move(_15.fld7.fld5.fld1),fld2: _18.fld4,fld3: _15.fld3,fld4: _15.fld7.fld5.fld4,fld5: _2 };
Call(_12 = fn11(_15.fld7.fld5.fld3.1, _15.fld4, _15.fld7.fld5.fld3.2, _15.fld5, _18.fld4, _15.fld7.fld5.fld3.0, _22.fld1.fld3.1, _23.fld1.fld0, _22.fld1.fld4.0, _18.fld5, _22.fld1.fld1.fld0, _5, _1, _22.fld1.fld3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_15.fld3.0.0 = _15.fld7.fld7.0 << _4;
Goto(bb10)
}
bb10 = {
Goto(bb11)
}
bb11 = {
_23.fld2 = [_22.fld1.fld3.0.0,_15.fld3.0.0,_15.fld7.fld5.fld3.0.0,_15.fld3.0.0,_15.fld3.0.0,_15.fld7.fld7.0,_15.fld3.0.0,_15.fld7.fld7.0];
_22.fld1.fld5 = _2;
_18.fld3 = (_22.fld2,);
_15.fld7.fld6 = _7;
_2 = -_15.fld7.fld5.fld5;
_24 = _18.fld5;
_15.fld7.fld0.fld1 = Adt52 { fld0: _23.fld1.fld0,fld1: _15.fld7.fld7.1 };
_15.fld5 = _15.fld4 as u128;
_15.fld7.fld0.fld1 = Adt52 { fld0: _22.fld1.fld1.fld0,fld1: _22.fld1.fld1.fld1 };
_15.fld3.0.0 = -_22.fld1.fld3.0.0;
_11 = _15.fld7.fld5.fld5;
_22.fld1.fld1.fld0 = core::ptr::addr_of!(_15.fld3.2);
_22.fld1 = Adt54 { fld0: _1,fld1: Move(_15.fld7.fld0.fld1),fld2: _18.fld4,fld3: _15.fld3,fld4: _15.fld7.fld5.fld4,fld5: _11 };
_3 = _8 >> _15.fld7.fld7.1;
_23.fld1 = Move(_22.fld1.fld1);
_15.fld7.fld0.fld4 = _22.fld1.fld3.0.0;
_23.fld4 = _15.fld7.fld5.fld3.0.0;
_22.fld1.fld3.2 = _22.fld1.fld4.0 as u128;
_23.fld5 = [_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1];
_15.fld7.fld5.fld5 = _25 as i32;
match _23.fld4 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
340282366920938463463374607431768189473 => bb19,
_ => bb18
}
}
bb12 = {
Goto(bb11)
}
bb13 = {
_15.fld3.0.0 = _15.fld7.fld7.0 << _4;
Goto(bb10)
}
bb14 = {
_22.fld1.fld3.0 = _15.fld3.0;
_12 = _8;
_22.fld1.fld2 = _18.fld4;
_15.fld7.fld7.1 = _15.fld7.fld5.fld1.fld1 | _15.fld7.fld5.fld1.fld1;
_15.fld7.fld5.fld1.fld0 = core::ptr::addr_of!(_15.fld3.2);
_1 = _13;
_22.fld1.fld3.1 = !_15.fld3.1;
_25 = _15.fld7.fld5.fld5 as f32;
_15.fld7.fld0.fld1.fld0 = core::ptr::addr_of!(_22.fld1.fld3.2);
_15.fld3.0.0 = _15.fld7.fld5.fld3.0.0 - _15.fld7.fld7.0;
_18.fld2 = [_8,_8,_3,_3,_3];
_23.fld1.fld0 = _15.fld7.fld5.fld1.fld0;
_22.fld1 = Adt54 { fld0: _13,fld1: Move(_15.fld7.fld5.fld1),fld2: _18.fld4,fld3: _15.fld3,fld4: _15.fld7.fld5.fld4,fld5: _2 };
Call(_12 = fn11(_15.fld7.fld5.fld3.1, _15.fld4, _15.fld7.fld5.fld3.2, _15.fld5, _18.fld4, _15.fld7.fld5.fld3.0, _22.fld1.fld3.1, _23.fld1.fld0, _22.fld1.fld4.0, _18.fld5, _22.fld1.fld1.fld0, _5, _1, _22.fld1.fld3), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_15.fld7.fld1 = _15.fld7.fld5.fld3.1 | _4;
_15.fld7.fld2 = _15.fld5 | _15.fld3.2;
_16 = [_15.fld3.1,_15.fld7.fld1];
_15.fld7.fld5.fld5 = _2 - _2;
_4 = _15.fld7.fld1 / 5522464369314596423_usize;
_6 = !false;
_15.fld7.fld5.fld3.2 = _6 as u128;
_15.fld7.fld5.fld4 = (_4,);
_15.fld3.2 = !_15.fld7.fld5.fld3.2;
_15.fld6 = 3644017493_u32 ^ 973844506_u32;
RET = [_6,_6];
_15.fld4 = 6006359809625903857_u64;
_3 = -_8;
_15.fld3.1 = _4 - _15.fld7.fld1;
_22.fld1.fld4.0 = !_4;
_22.fld1.fld2 = [_15.fld3.4,_15.fld3.4,_15.fld3.4,_15.fld3.4,_15.fld3.4];
_15.fld7.fld5.fld1.fld1 = _15.fld7.fld5.fld3.3 + _15.fld7.fld7.1;
_15.fld1 = core::ptr::addr_of_mut!(_18.fld0);
_23.fld3 = core::ptr::addr_of_mut!(_19);
_2 = -_11;
_15.fld7.fld7 = _15.fld7.fld5.fld3.0;
_23.fld6 = [_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld1.fld1,_15.fld7.fld5.fld1.fld1];
_12 = !_5;
_22.fld2 = [_6,_6];
_22.fld1.fld3.3 = -_15.fld7.fld7.1;
_15.fld2 = core::ptr::addr_of_mut!(_19);
_22.fld1.fld3.0.1 = _15.fld3.3;
match _15.fld7.fld7.1 {
0 => bb3,
43455926409780245612125181517449252937 => bb8,
_ => bb7
}
}
bb17 = {
_15.fld7.fld7.1 = _15.fld3.3 >> _15.fld3.2;
Call(_15.fld3.0.0 = fn6(_10, _15.fld3.3, _15.fld2, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb18 = {
_15.fld2 = core::ptr::addr_of_mut!(_18.fld5);
_15.fld5 = _15.fld3.2;
_18.fld4 = [_15.fld3.4,_15.fld3.4,_15.fld3.4,_15.fld3.4,_15.fld3.4];
_8 = _12 * _5;
_18.fld1 = 26_u8 + 101_u8;
Goto(bb5)
}
bb19 = {
_9 = _15.fld6 as f64;
_15.fld3.3 = _15.fld7.fld7.1 & _15.fld7.fld5.fld3.3;
_15.fld7.fld5.fld5 = _11;
_29.fld0 = (_6, _23.fld4);
_15.fld0 = _18.fld1 >> _15.fld3.3;
_5 = _3;
_15.fld7.fld5.fld3.4 = _22.fld1.fld3.4;
_31 = _3 * _3;
_29.fld1 = (_22.fld1.fld3.1,);
_18.fld3 = (_22.fld2,);
_29.fld4.fld2 = _15.fld3.1 as isize;
Goto(bb20)
}
bb20 = {
Call(_32 = dump_var(5_usize, 12_usize, Move(_12), 6_usize, Move(_6), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_32 = dump_var(5_usize, 13_usize, Move(_13), 2_usize, Move(_2), 33_usize, _33, 33_usize, _33), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: f64,mut _2: i128,mut _3: *mut f64,mut _4: [isize; 8]) -> i16 {
mir! {
type RET = i16;
let _5: (i32, [char; 2]);
let _6: [u32; 7];
let _7: [bool; 2];
let _8: Adt63;
let _9: i8;
let _10: isize;
let _11: Adt52;
let _12: Adt52;
let _13: i8;
let _14: f64;
let _15: i16;
let _16: Adt59;
let _17: u8;
let _18: (i16, i128);
let _19: *const [char; 5];
let _20: [i16; 8];
let _21: Adt63;
let _22: ((i16, i128), usize, u128, i128, char);
let _23: *const f32;
let _24: ([u16; 5], (bool, i16), [u16; 5], [i128; 8], [u32; 7]);
let _25: [i128; 4];
let _26: [u32; 7];
let _27: [i128; 8];
let _28: ();
let _29: ();
{
(*_3) = _1;
RET = !(-15002_i16);
RET = 4068872266_u32 as i16;
RET = (-25574_i16) | (-28775_i16);
_1 = 13351_i16 as f64;
_5.1 = ['\u{64a31}','\u{a4641}'];
_4 = [50_isize,9223372036854775807_isize,9223372036854775807_isize,10_isize,9223372036854775807_isize,(-9223372036854775808_isize),90_isize,9223372036854775807_isize];
_1 = (*_3);
Goto(bb1)
}
bb1 = {
_3 = core::ptr::addr_of_mut!((*_3));
RET = (-12809_i16) + (-27933_i16);
_2 = -121133966480748443114454195091394500468_i128;
(*_3) = _1;
RET = 238_u8 as i16;
RET = -(-10816_i16);
_4 = [(-9223372036854775808_isize),(-34_isize),(-9223372036854775808_isize),(-30_isize),(-76_isize),9223372036854775807_isize,(-86_isize),(-9223372036854775808_isize)];
RET = (-22721_i16) | 22238_i16;
_2 = 24620322487444884549386309601871154879_i128 << 146496844482837884280751640573560083924_i128;
(*_3) = 268145953_u32 as f64;
_2 = (-140217630486410802949740409016432904487_i128) + 34693600542552491629512461599033204340_i128;
_5.1 = ['\u{d9b4e}','\u{7789b}'];
_5.0 = 1115465444_i32 << _2;
_3 = core::ptr::addr_of_mut!(_1);
_5.1 = ['\u{eb2cf}','\u{5e6d1}'];
(*_3) = 8131061078046565536_u64 as f64;
RET = _5.0 as i16;
_5.1 = ['\u{4c759}','\u{d4fa9}'];
(*_3) = 1809637885_u32 as f64;
_4 = [(-9223372036854775808_isize),(-34_isize),(-9223372036854775808_isize),(-111_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1 as i128;
Goto(bb2)
}
bb2 = {
(*_3) = 862741256_u32 as f64;
_8.fld4.fld4.fld5.0 = _5.0;
(*_3) = 20415_u16 as f64;
Goto(bb3)
}
bb3 = {
_8.fld2.fld0.1.0 = _8.fld4.fld4.fld5.0 <= _5.0;
_8.fld2.fld2 = 496300576_u32 ^ 1259827291_u32;
_8.fld3 = 5067246160447144974_u64;
_8.fld4.fld4.fld5.1 = ['\u{92f89}','\u{1054b2}'];
_3 = core::ptr::addr_of_mut!((*_3));
_11.fld0 = core::ptr::addr_of!(_8.fld4.fld4.fld6.2);
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-47_isize),(-124_isize)];
_11.fld1 = _2 & _2;
_8.fld4.fld4.fld6.4 = '\u{8759e}';
_8.fld4.fld4.fld4.0 = _8.fld2.fld0.1.0 | _8.fld2.fld0.1.0;
_8.fld4.fld4.fld3 = [_8.fld4.fld4.fld4.0];
_8.fld4.fld4.fld6.3 = !_11.fld1;
_2 = !_8.fld4.fld4.fld6.3;
_5.1 = _8.fld4.fld4.fld5.1;
_11.fld0 = core::ptr::addr_of!(_8.fld4.fld4.fld6.2);
_14 = -_1;
_8.fld4.fld4.fld6.4 = '\u{c66b0}';
_8.fld4.fld4.fld6.3 = !_11.fld1;
_9 = -(-55_i8);
Goto(bb4)
}
bb4 = {
_8.fld2.fld0.4 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_2 = !_11.fld1;
_12.fld0 = core::ptr::addr_of!(_8.fld4.fld4.fld6.2);
_8.fld4.fld0.1 = (-2708_i16) | (-24403_i16);
_3 = core::ptr::addr_of_mut!(_14);
_8.fld2.fld0.1.0 = _8.fld4.fld4.fld4.0 > _8.fld4.fld4.fld4.0;
Goto(bb5)
}
bb5 = {
_8.fld4.fld4.fld4 = (_8.fld2.fld0.1.0, _8.fld4.fld0.1);
_8.fld2.fld0.0 = [52961_u16,63474_u16,52258_u16,24937_u16,6704_u16];
_6 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_11.fld0 = core::ptr::addr_of!(_8.fld4.fld4.fld6.2);
_8.fld3 = 11189815092837876652_u64 - 7357752243428662893_u64;
_4 = [(-46_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),43_isize,18_isize];
_8.fld4.fld4.fld2 = 9223372036854775807_isize;
_7 = [_8.fld4.fld4.fld4.0,_8.fld2.fld0.1.0];
_8.fld2.fld0.3 = [_11.fld1,_11.fld1,_11.fld1,_11.fld1,_8.fld4.fld4.fld6.3,_8.fld4.fld4.fld6.3,_8.fld4.fld4.fld6.3,_11.fld1];
_8.fld1 = _4;
_8.fld4.fld4.fld4.1 = _8.fld4.fld4.fld2 as i16;
_13 = _8.fld3 as i8;
_6 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_16.fld0 = !314181729724298820135671473590439348730_u128;
_9 = _13;
_12.fld1 = _2;
_8.fld4.fld2 = core::ptr::addr_of_mut!(_8.fld2.fld0.2);
_9 = _13 + _13;
_8.fld4.fld4.fld6.1 = 3_usize + 4_usize;
_8.fld2.fld1 = _8.fld4.fld4.fld6.1 as f32;
_8.fld4.fld0.1 = _16.fld0 as i16;
_21.fld1 = [_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2];
_8.fld6.0 = _7;
_8.fld4.fld1 = (_8.fld4.fld4.fld6.1,);
Goto(bb6)
}
bb6 = {
_16.fld0 = 209399217313075898303213931440045855591_u128;
_21.fld4.fld3 = -_8.fld2.fld1;
_8.fld0 = _9 <= _9;
Call(_13 = fn7(_1, _8.fld1, _8.fld4.fld4.fld2, _8.fld2.fld2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8.fld4.fld4.fld6.2 = _16.fld0;
RET = _8.fld4.fld4.fld4.1 * _8.fld4.fld4.fld4.1;
_3 = core::ptr::addr_of_mut!(_14);
_8.fld4.fld4.fld0 = _4;
_8.fld2.fld0.1.1 = _8.fld4.fld0.1;
_15 = _8.fld4.fld0.1 | _8.fld2.fld0.1.1;
_8.fld4.fld4.fld4 = (_8.fld2.fld0.1.0, _15);
_21.fld4.fld2 = core::ptr::addr_of_mut!(_8.fld2.fld0.0);
_22.0.0 = (-2058388155575000834_i64) as i16;
_22.2 = _8.fld4.fld1.0 as u128;
_21.fld4.fld4.fld6.4 = _8.fld4.fld4.fld6.4;
_21.fld3 = !_8.fld3;
_21.fld2.fld2 = _8.fld2.fld2;
_21.fld4.fld2 = core::ptr::addr_of_mut!(_24.0);
_12 = Move(_11);
Call(_19 = fn9(_8.fld4.fld2, _4, _8.fld2.fld0.3, _8.fld4.fld4.fld2, _8.fld1, _8.fld4.fld1, _13), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_22.2 = _16.fld0;
match _16.fld0 {
0 => bb1,
1 => bb2,
2 => bb7,
209399217313075898303213931440045855591 => bb9,
_ => bb5
}
}
bb9 = {
_7 = [_8.fld2.fld0.1.0,_8.fld2.fld0.1.0];
_21.fld4.fld4.fld5.1 = [_21.fld4.fld4.fld6.4,_8.fld4.fld4.fld6.4];
_18.1 = -_12.fld1;
Goto(bb10)
}
bb10 = {
_24.2 = [49488_u16,9611_u16,2104_u16,17575_u16,23863_u16];
_24.1 = _8.fld2.fld0.1;
_21.fld3 = _18.1 as u64;
match _16.fld0 {
0 => bb11,
1 => bb12,
209399217313075898303213931440045855591 => bb14,
_ => bb13
}
}
bb11 = {
_7 = [_8.fld2.fld0.1.0,_8.fld2.fld0.1.0];
_21.fld4.fld4.fld5.1 = [_21.fld4.fld4.fld6.4,_8.fld4.fld4.fld6.4];
_18.1 = -_12.fld1;
Goto(bb10)
}
bb12 = {
_8.fld4.fld4.fld4 = (_8.fld2.fld0.1.0, _8.fld4.fld0.1);
_8.fld2.fld0.0 = [52961_u16,63474_u16,52258_u16,24937_u16,6704_u16];
_6 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_11.fld0 = core::ptr::addr_of!(_8.fld4.fld4.fld6.2);
_8.fld3 = 11189815092837876652_u64 - 7357752243428662893_u64;
_4 = [(-46_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),43_isize,18_isize];
_8.fld4.fld4.fld2 = 9223372036854775807_isize;
_7 = [_8.fld4.fld4.fld4.0,_8.fld2.fld0.1.0];
_8.fld2.fld0.3 = [_11.fld1,_11.fld1,_11.fld1,_11.fld1,_8.fld4.fld4.fld6.3,_8.fld4.fld4.fld6.3,_8.fld4.fld4.fld6.3,_11.fld1];
_8.fld1 = _4;
_8.fld4.fld4.fld4.1 = _8.fld4.fld4.fld2 as i16;
_13 = _8.fld3 as i8;
_6 = [_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2,_8.fld2.fld2];
_16.fld0 = !314181729724298820135671473590439348730_u128;
_9 = _13;
_12.fld1 = _2;
_8.fld4.fld2 = core::ptr::addr_of_mut!(_8.fld2.fld0.2);
_9 = _13 + _13;
_8.fld4.fld4.fld6.1 = 3_usize + 4_usize;
_8.fld2.fld1 = _8.fld4.fld4.fld6.1 as f32;
_8.fld4.fld0.1 = _16.fld0 as i16;
_21.fld1 = [_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2];
_8.fld6.0 = _7;
_8.fld4.fld1 = (_8.fld4.fld4.fld6.1,);
Goto(bb6)
}
bb13 = {
_8.fld2.fld0.1.0 = _8.fld4.fld4.fld5.0 <= _5.0;
_8.fld2.fld2 = 496300576_u32 ^ 1259827291_u32;
_8.fld3 = 5067246160447144974_u64;
_8.fld4.fld4.fld5.1 = ['\u{92f89}','\u{1054b2}'];
_3 = core::ptr::addr_of_mut!((*_3));
_11.fld0 = core::ptr::addr_of!(_8.fld4.fld4.fld6.2);
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-47_isize),(-124_isize)];
_11.fld1 = _2 & _2;
_8.fld4.fld4.fld6.4 = '\u{8759e}';
_8.fld4.fld4.fld4.0 = _8.fld2.fld0.1.0 | _8.fld2.fld0.1.0;
_8.fld4.fld4.fld3 = [_8.fld4.fld4.fld4.0];
_8.fld4.fld4.fld6.3 = !_11.fld1;
_2 = !_8.fld4.fld4.fld6.3;
_5.1 = _8.fld4.fld4.fld5.1;
_11.fld0 = core::ptr::addr_of!(_8.fld4.fld4.fld6.2);
_14 = -_1;
_8.fld4.fld4.fld6.4 = '\u{c66b0}';
_8.fld4.fld4.fld6.3 = !_11.fld1;
_9 = -(-55_i8);
Goto(bb4)
}
bb14 = {
_8.fld2.fld0.1.1 = _9 as i16;
_21.fld2.fld0 = (_8.fld2.fld0.2, _8.fld4.fld4.fld4, _8.fld2.fld0.0, _8.fld2.fld0.3, _6);
_22.0.1 = _8.fld4.fld4.fld6.3 << _2;
_8.fld4.fld2 = core::ptr::addr_of_mut!(_24.0);
_8.fld4.fld4.fld6.0.0 = _16.fld0 as i16;
_22.3 = _18.1 - _2;
_21.fld4.fld4.fld0 = [_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2];
_21.fld4.fld4.fld5 = _5;
_22.1 = !_8.fld4.fld1.0;
_8.fld4.fld0.1 = (-191897899518863943_i64) as i16;
_8.fld2.fld0.1 = _24.1;
_8.fld2 = Adt57 { fld0: _21.fld2.fld0,fld1: _21.fld4.fld3,fld2: _21.fld2.fld2,fld3: 119_u8 };
_15 = _8.fld2.fld0.1.1;
_8.fld5 = _8.fld4.fld1.0 as i32;
_18.0 = 5576995098944845129_i64 as i16;
RET = 27144_u16 as i16;
_8.fld4.fld4.fld6.0 = (_21.fld2.fld0.1.1, _8.fld4.fld4.fld6.3);
_21.fld4.fld0.0 = _21.fld2.fld0.1.0;
_8.fld2.fld1 = _21.fld4.fld3;
_22.4 = _21.fld4.fld4.fld6.4;
_8.fld2.fld0.1.1 = _15 | _8.fld4.fld0.1;
_21.fld1 = [_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2,_8.fld4.fld4.fld2];
_21.fld2.fld3 = !_8.fld2.fld3;
_21.fld4.fld4.fld6.0.0 = _8.fld2.fld0.1.1;
_8.fld4.fld4.fld2 = (*_3) as isize;
_21.fld4.fld4.fld1 = core::ptr::addr_of_mut!(_21.fld2.fld0.2);
_21.fld0 = _8.fld0;
_21.fld4.fld4.fld6.4 = _22.4;
_17 = !_21.fld2.fld3;
_21.fld2.fld0.0 = [63265_u16,64759_u16,22960_u16,63959_u16,51303_u16];
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(6_usize, 22_usize, Move(_22), 6_usize, Move(_6), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(6_usize, 4_usize, Move(_4), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: f64,mut _2: [isize; 8],mut _3: isize,mut _4: u32) -> i8 {
mir! {
type RET = i8;
let _5: (i16, i128);
let _6: [char; 2];
let _7: [i16; 8];
let _8: f64;
let _9: u64;
let _10: [i128; 4];
let _11: f64;
let _12: char;
let _13: f32;
let _14: (usize,);
let _15: *const u128;
let _16: [bool; 2];
let _17: [isize; 8];
let _18: (i32, [char; 2]);
let _19: i32;
let _20: f64;
let _21: f64;
let _22: Adt49;
let _23: ();
let _24: ();
{
_4 = 1963293956_u32;
_5.1 = 53757269105323564063737583828095387230_i128;
_5.0 = (-20953_i16);
RET = _3 as i8;
_5.1 = 133079511862569709657337966590069612323_i128 ^ (-21611386886676049374888991346114157194_i128);
_3 = -9223372036854775807_isize;
_5.0 = '\u{4b7fd}' as i16;
_1 = 47294_u16 as f64;
_1 = _3 as f64;
_5.0 = 3130_i16 ^ (-14646_i16);
_3 = _4 as isize;
_4 = 3638277009_u32 >> _3;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_5 = ((-6271_i16), (-160152358689463084386435721289318328642_i128));
_4 = _5.0 as u32;
_4 = 690248594_u32 - 2305477859_u32;
_5 = (17488_i16, (-69906908286743163216058852399852304501_i128));
RET = 7551941358742385097_i64 as i8;
RET = 13314446862141818865_u64 as i8;
_5.1 = 168917768954413053455543603798295030724_i128 | 85274442189080748621072211429410607086_i128;
RET = (-17_i8);
_5.0 = 19271_i16;
_3 = (-9223372036854775808_isize);
Call(_5.0 = fn8(_1, _3, _3, _3, _2, _4, _2, _4, _3, _3, _2, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-8_i8) + 62_i8;
_4 = 1218282690_u32 >> _3;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_4 = 983136599_u32 << _5.1;
Call(_1 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = (10466_i16, (-36436245941118365472742290001471846013_i128));
_4 = _5.1 as u32;
_3 = 4541_u16 as isize;
_5.0 = !25012_i16;
RET = (-27_i8) | 81_i8;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = _5.0 as isize;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
match _5.1 {
0 => bb1,
1 => bb3,
2 => bb4,
303846120979820097990632317430296365443 => bb6,
_ => bb5
}
}
bb3 = {
RET = (-8_i8) + 62_i8;
_4 = 1218282690_u32 >> _3;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_4 = 983136599_u32 << _5.1;
Call(_1 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_5.0 = -(-4746_i16);
_3 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_3 = 33_isize * (-9223372036854775808_isize);
_3 = 9223372036854775807_isize << _4;
_3 = -(-9223372036854775808_isize);
_5.0 = (-12066_i16);
_6 = ['\u{f2d55}','\u{38548}'];
match _5.1 {
0 => bb1,
1 => bb5,
2 => bb3,
303846120979820097990632317430296365443 => bb8,
_ => bb7
}
}
bb7 = {
_5 = (10466_i16, (-36436245941118365472742290001471846013_i128));
_4 = _5.1 as u32;
_3 = 4541_u16 as isize;
_5.0 = !25012_i16;
RET = (-27_i8) | 81_i8;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = _5.0 as isize;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
match _5.1 {
0 => bb1,
1 => bb3,
2 => bb4,
303846120979820097990632317430296365443 => bb6,
_ => bb5
}
}
bb8 = {
_4 = !3319598781_u32;
_5.0 = 20502_i16;
_3 = (-9223372036854775808_isize);
RET = (-34_i8) * (-117_i8);
RET = !65_i8;
_3 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_5.0 = 7502_i16 ^ 1499_i16;
RET = !(-56_i8);
_5 = (24503_i16, (-97707352719075306796572172492894330430_i128));
_5.0 = !(-29389_i16);
_6 = ['\u{27a33}','\u{3de05}'];
RET = 84_i8 + (-103_i8);
_3 = (-9223372036854775808_isize) + 23_isize;
_1 = 7_usize as f64;
_5.0 = 17103_i16 * 8643_i16;
_6 = ['\u{69300}','\u{29559}'];
_3 = (-9223372036854775808_isize);
Goto(bb9)
}
bb9 = {
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_9 = 209_u8 as u64;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
RET = 61_i8 * (-79_i8);
_5 = ((-25237_i16), 150134987700243791618607477919239309741_i128);
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_1 = 25_i8 as f64;
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_4 = true as u32;
_5.0 = 30102_i16;
RET = (-92_i8);
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_10 = [_5.1,_5.1,_5.1,_5.1];
_6 = ['\u{1015dc}','\u{cb3d3}'];
_5.1 = 137024715594734680020696451790057503600_i128;
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_3 = (-107_isize);
RET = !124_i8;
Call(_8 = core::intrinsics::transmute(_9), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_8 = _1;
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
RET = (-83_i8);
_4 = 1985192302_u32 - 3103031425_u32;
_10 = [_5.1,_5.1,_5.1,_5.1];
Goto(bb11)
}
bb11 = {
RET = !40_i8;
_11 = _1;
_6 = ['\u{8cddb}','\u{80eb7}'];
_1 = _8;
_5.1 = _8 as i128;
_3 = 15_isize;
_8 = -_1;
_5 = (14264_i16, 1996988583917959980321371754508358594_i128);
_5 = (7819_i16, 133777424691233590079496082561230950648_i128);
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_1 = _8 / f64::NAN;
match _5.1 {
0 => bb9,
1 => bb10,
133777424691233590079496082561230950648 => bb13,
_ => bb12
}
}
bb12 = {
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_8 = _1;
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
RET = (-83_i8);
_4 = 1985192302_u32 - 3103031425_u32;
_10 = [_5.1,_5.1,_5.1,_5.1];
Goto(bb11)
}
bb13 = {
_12 = '\u{bbbd1}';
_9 = !10810003155652982060_u64;
RET = 46_i8 * (-100_i8);
_9 = !4499390639001362494_u64;
_9 = 53277_u16 as u64;
_14 = (5_usize,);
_5.0 = 166_u8 as i16;
_13 = _4 as f32;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_13 = _5.0 as f32;
_9 = 8270914730420182790_u64 % 17104997872831977416_u64;
_13 = _1 as f32;
_5.1 = !(-144905421129860032787028089764682404639_i128);
_13 = _1 as f32;
_4 = !2701314361_u32;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_4 = !3593274131_u32;
match _14.0 {
0 => bb1,
1 => bb8,
2 => bb9,
3 => bb7,
5 => bb15,
_ => bb14
}
}
bb14 = {
_5 = (10466_i16, (-36436245941118365472742290001471846013_i128));
_4 = _5.1 as u32;
_3 = 4541_u16 as isize;
_5.0 = !25012_i16;
RET = (-27_i8) | 81_i8;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = _5.0 as isize;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
match _5.1 {
0 => bb1,
1 => bb3,
2 => bb4,
303846120979820097990632317430296365443 => bb6,
_ => bb5
}
}
bb15 = {
_5.1 = (-123667055587924704018451359470796663810_i128) << _5.0;
_11 = 48435_u16 as f64;
RET = (-56_i8);
_18.1 = _6;
_18.0 = _13 as i32;
_5.1 = (-165889447009292861279126454131861910715_i128) + 32420538162493284153036789816277468517_i128;
RET = 112_i8;
_19 = -_18.0;
_16 = [true,true];
RET = (-43_i8) | (-122_i8);
_11 = _1 * _1;
_5 = ((-30662_i16), 110179332059758033042066614656309046722_i128);
RET = 100_i8;
_8 = -_11;
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(7_usize, 7_usize, Move(_7), 19_usize, Move(_19), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(7_usize, 4_usize, Move(_4), 2_usize, Move(_2), 24_usize, _24, 24_usize, _24), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: f64,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [isize; 8],mut _6: u32,mut _7: [isize; 8],mut _8: u32,mut _9: isize,mut _10: isize,mut _11: [isize; 8],mut _12: u32,mut _13: u32) -> i16 {
mir! {
type RET = i16;
let _14: Adt48;
let _15: Adt48;
let _16: [char; 5];
let _17: f32;
let _18: f32;
let _19: ();
let _20: ();
{
RET = 15949_i16;
_10 = !_2;
_1 = (-1668877147_i32) as f64;
_13 = _10 as u32;
_2 = _3;
_1 = 180759047342631790138339105290333922074_u128 as f64;
_14.fld5.1 = ['\u{828fe}','\u{2ba32}'];
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_14.fld4 = (true, (-5955_i16));
_14.fld5.0 = !(-695935589_i32);
Goto(bb8)
}
bb8 = {
_13 = _12;
_14.fld3 = [_14.fld4.0];
_8 = _12 - _13;
_7 = [_9,_10,_9,_4,_10,_3,_3,_10];
_10 = 51000_u16 as isize;
_15.fld5.0 = _1 as i32;
_1 = 40_u16 as f64;
_15.fld2 = _10;
_15.fld6.0.1 = 112125603605710465270928912253682874814_i128;
_13 = !_12;
_15.fld6.1 = !936858285991073538_usize;
_14.fld5.0 = _15.fld5.0;
_15.fld6.2 = 44_i8 as u128;
_15.fld4.1 = _15.fld6.1 as i16;
_11 = _5;
_14.fld6.3 = _15.fld6.0.1 ^ _15.fld6.0.1;
_15.fld0 = [_10,_9,_2,_15.fld2,_2,_2,_2,_3];
_6 = _8 | _12;
_2 = _15.fld2;
_14.fld5.0 = -_15.fld5.0;
_5 = _7;
Goto(bb9)
}
bb9 = {
_13 = 51_u8 as u32;
_14.fld3 = [_14.fld4.0];
_15.fld4.1 = _14.fld4.1;
_14.fld0 = _7;
_2 = _4 - _9;
match _3 {
0 => bb4,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463454151235394913435648 => bb16,
_ => bb15
}
}
bb10 = {
_13 = _12;
_14.fld3 = [_14.fld4.0];
_8 = _12 - _13;
_7 = [_9,_10,_9,_4,_10,_3,_3,_10];
_10 = 51000_u16 as isize;
_15.fld5.0 = _1 as i32;
_1 = 40_u16 as f64;
_15.fld2 = _10;
_15.fld6.0.1 = 112125603605710465270928912253682874814_i128;
_13 = !_12;
_15.fld6.1 = !936858285991073538_usize;
_14.fld5.0 = _15.fld5.0;
_15.fld6.2 = 44_i8 as u128;
_15.fld4.1 = _15.fld6.1 as i16;
_11 = _5;
_14.fld6.3 = _15.fld6.0.1 ^ _15.fld6.0.1;
_15.fld0 = [_10,_9,_2,_15.fld2,_2,_2,_2,_3];
_6 = _8 | _12;
_2 = _15.fld2;
_14.fld5.0 = -_15.fld5.0;
_5 = _7;
Goto(bb9)
}
bb11 = {
_14.fld4 = (true, (-5955_i16));
_14.fld5.0 = !(-695935589_i32);
Goto(bb8)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_15.fld5.1 = ['\u{e552c}','\u{3ea64}'];
_15.fld3 = [_14.fld4.0];
_14.fld6.0.1 = _14.fld6.3;
_14.fld3 = _15.fld3;
_14.fld6.3 = !_14.fld6.0.1;
_14.fld6.3 = _14.fld6.0.1 - _14.fld6.0.1;
_15.fld4 = _14.fld4;
_14.fld6.0.0 = _15.fld4.1 - _15.fld4.1;
_15.fld4 = (_14.fld4.0, _14.fld4.1);
Goto(bb17)
}
bb17 = {
Call(_19 = dump_var(8_usize, 12_usize, Move(_12), 5_usize, Move(_5), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_19 = dump_var(8_usize, 13_usize, Move(_13), 10_usize, Move(_10), 20_usize, _20, 20_usize, _20), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: *mut [u16; 5],mut _2: [isize; 8],mut _3: [i128; 8],mut _4: isize,mut _5: [isize; 8],mut _6: (usize,),mut _7: i8) -> *const [char; 5] {
mir! {
type RET = *const [char; 5];
let _8: Adt51;
let _9: [u32; 7];
let _10: f64;
let _11: [bool; 2];
let _12: isize;
let _13: [u16; 5];
let _14: (bool, i16);
let _15: Adt61;
let _16: u32;
let _17: char;
let _18: char;
let _19: (usize,);
let _20: i64;
let _21: bool;
let _22: isize;
let _23: u128;
let _24: f64;
let _25: Adt55;
let _26: isize;
let _27: i16;
let _28: f64;
let _29: Adt64;
let _30: Adt54;
let _31: [i16; 8];
let _32: [i128; 8];
let _33: isize;
let _34: char;
let _35: f64;
let _36: char;
let _37: Adt56;
let _38: Adt48;
let _39: [u16; 4];
let _40: ();
let _41: ();
{
(*_1) = [23696_u16,59313_u16,5682_u16,63769_u16,15691_u16];
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = [6349_u16,40539_u16,34725_u16,45322_u16,44754_u16];
_2 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = [(-122151069688266595940710267867630997567_i128),74546862745501860632685979770515015235_i128,(-146202675194765101396266192073970783905_i128),(-98103649953532393908749571609157715206_i128),135148582662859437275645375479762560160_i128,137489915659104230717007120828375926302_i128,48204626416412039511119622381230632773_i128,130578144478119310592878977732369608277_i128];
(*_1) = [22582_u16,58904_u16,17115_u16,28653_u16,57801_u16];
(*_1) = [16623_u16,44571_u16,58700_u16,24728_u16,43370_u16];
_8.fld1.3 = (_6.0,);
_8.fld1.1 = !(-10139337310736671637881715808026676964_i128);
_7 = (-44_i8);
_9 = [2415632308_u32,1966066982_u32,2983566239_u32,226641472_u32,2777216068_u32,2325595793_u32,3840019152_u32];
_2 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = [_8.fld1.1,_8.fld1.1,_8.fld1.1,_8.fld1.1,_8.fld1.1,_8.fld1.1,_8.fld1.1,_8.fld1.1];
_8.fld1.0 = true;
_8.fld1.2 = core::ptr::addr_of!(_8.fld6);
_8.fld3 = [_8.fld1.0];
_8.fld5 = _8.fld1.1 as u8;
_8.fld1.1 = 108209284033839047342092405295478913352_i128;
_8.fld1.1 = !58508347716568404804084229452963434856_i128;
_6 = _8.fld1.3;
_8.fld5 = !232_u8;
_2 = [_4,_4,_4,_4,_4,_4,_4,_4];
_8.fld1.3.0 = !_6.0;
_9 = [3504901417_u32,2460774781_u32,1736067266_u32,3177210583_u32,2598402306_u32,3745674490_u32,2315022690_u32];
_8.fld0.0 = [_8.fld1.0,_8.fld1.0];
Goto(bb1)
}
bb1 = {
_6.0 = _8.fld1.3.0;
_12 = _4;
_7 = -71_i8;
Goto(bb2)
}
bb2 = {
_14 = (_8.fld1.0, 2180_i16);
_13 = (*_1);
_8.fld0.0 = [_8.fld1.0,_14.0];
_8.fld1.0 = _14.0;
_8.fld3 = [_8.fld1.0];
_9 = [3069082822_u32,4084335939_u32,1155242997_u32,3788426695_u32,719836158_u32,2042028441_u32,3630542007_u32];
_12 = _4;
_10 = _8.fld1.3.0 as f64;
_9 = [1684162479_u32,2435882286_u32,3595201717_u32,2631394410_u32,4041987273_u32,3536423282_u32,2550257879_u32];
_9 = [350247697_u32,168496273_u32,2753838515_u32,3165293388_u32,2226848748_u32,3337032578_u32,1304206975_u32];
_8.fld3 = [_8.fld1.0];
_6.0 = !_8.fld1.3.0;
_15.fld1.fld1 = (-2483737548584574320_i64) as i128;
_17 = '\u{f8c5}';
_17 = '\u{79627}';
_19.0 = !_8.fld1.3.0;
_8.fld1.3 = (_19.0,);
_18 = _17;
match _14.1 {
0 => bb3,
1 => bb4,
2 => bb5,
2180 => bb7,
_ => bb6
}
}
bb3 = {
_6.0 = _8.fld1.3.0;
_12 = _4;
_7 = -71_i8;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
(*_1) = _13;
_10 = _7 as f64;
_19 = (_8.fld1.3.0,);
(*_1) = _13;
_10 = _14.1 as f64;
_19 = _8.fld1.3;
_3 = [_8.fld1.1,_15.fld1.fld1,_15.fld1.fld1,_8.fld1.1,_8.fld1.1,_15.fld1.fld1,_15.fld1.fld1,_8.fld1.1];
_8.fld1.1 = 2909949686_u32 as i128;
_16 = 1996159884_u32 ^ 3892790698_u32;
_11 = [_8.fld1.0,_8.fld1.0];
_6.0 = _8.fld1.3.0 & _19.0;
_15.fld1.fld1 = -_8.fld1.1;
_8.fld6 = (-7938831082221494592_i64) << _8.fld1.3.0;
_8.fld1.2 = core::ptr::addr_of!(_8.fld6);
_13 = [61712_u16,59749_u16,33100_u16,53935_u16,18999_u16];
_15.fld1.fld1 = 16925259737850360850_u64 as i128;
_15.fld0.0 = _8.fld1.0 as usize;
_1 = core::ptr::addr_of_mut!((*_1));
_6.0 = _8.fld1.0 as usize;
_15.fld0.0 = _19.0 ^ _8.fld1.3.0;
_6.0 = !_19.0;
_19.0 = _12 as usize;
_4 = -_12;
_23 = !42857845035607309685164626777795388958_u128;
_15.fld1.fld0 = core::ptr::addr_of!(_23);
Call(_8.fld1.3.0 = fn10(_8.fld6, _4, _16, _12, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = core::ptr::addr_of_mut!((*_1));
_10 = _14.1 as f64;
_17 = _18;
_8.fld2 = core::ptr::addr_of!(_25.fld5.fld2);
_25.fld5.fld2 = [_17,_17,_18,_18,_17];
_24 = _10 / f64::INFINITY;
_10 = _12 as f64;
_25.fld5.fld3.3 = _15.fld1.fld1 ^ _8.fld1.1;
_8.fld1.2 = core::ptr::addr_of!(_8.fld6);
RET = core::ptr::addr_of!(_25.fld5.fld2);
_22 = _12 >> _15.fld0.0;
_24 = _10;
_25.fld0.fld1.fld1 = -_25.fld5.fld3.3;
_21 = _8.fld1.0 | _14.0;
_29.fld1.fld1.fld0 = _15.fld1.fld0;
_23 = !199745305499089966730323718191672461985_u128;
_15.fld1.fld0 = core::ptr::addr_of!(_25.fld2);
_25.fld5.fld1.fld0 = core::ptr::addr_of!(_29.fld1.fld3.2);
(*RET) = [_17,_18,_18,_18,_18];
_25.fld0.fld1 = Move(_15.fld1);
_13 = [65324_u16,600_u16,20459_u16,50322_u16,51284_u16];
_30.fld1.fld0 = core::ptr::addr_of!(_23);
_25.fld5.fld3.4 = _17;
_25.fld7 = (_14.1, _25.fld0.fld1.fld1);
_25.fld0.fld3 = core::ptr::addr_of_mut!(_28);
Goto(bb9)
}
bb9 = {
_29.fld1.fld3.1 = !_6.0;
_28 = -_24;
_25.fld0.fld1.fld0 = core::ptr::addr_of!(_30.fld3.2);
_31 = [_25.fld7.0,_25.fld7.0,_14.1,_14.1,_25.fld7.0,_25.fld7.0,_14.1,_14.1];
_6 = _15.fld0;
_8.fld2 = core::ptr::addr_of!(_25.fld5.fld2);
_25.fld1 = _22 as usize;
_25.fld0.fld3 = core::ptr::addr_of_mut!(_28);
_25.fld5.fld3.3 = !_25.fld7.1;
_25.fld0.fld5 = [_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5];
_20 = _8.fld6 - _8.fld6;
RET = _8.fld2;
_15.fld0.0 = !_19.0;
_25.fld5.fld5 = !1869715847_i32;
_29.fld1.fld3 = (_25.fld7, _8.fld1.3.0, _23, _25.fld0.fld1.fld1, _25.fld5.fld3.4);
_30 = Adt54 { fld0: _9,fld1: Move(_25.fld0.fld1),fld2: (*RET),fld3: _29.fld1.fld3,fld4: _6,fld5: _25.fld5.fld5 };
_30.fld3.3 = -_30.fld1.fld1;
Goto(bb10)
}
bb10 = {
_29.fld1.fld3.3 = _30.fld3.3;
_5 = [_12,_22,_22,_22,_22,_4,_22,_22];
_29.fld1.fld1.fld1 = !_30.fld3.3;
_25.fld5.fld3.0.0 = _25.fld7.0 ^ _29.fld1.fld3.0.0;
_26 = _22 << _4;
_25.fld5.fld3.0 = (_30.fld3.0.0, _29.fld1.fld3.3);
_25.fld0.fld0 = _21;
_19 = (_25.fld1,);
_30.fld3.0.1 = _19.0 as i128;
_25.fld5.fld4.0 = _19.0 + _25.fld1;
_25.fld5.fld3.0 = (_30.fld3.0.0, _8.fld1.1);
_25.fld7.0 = _25.fld5.fld3.0.0;
_18 = _30.fld3.4;
Goto(bb11)
}
bb11 = {
_22 = _26 | _12;
_18 = _25.fld5.fld3.4;
_29.fld0 = _30.fld3.3 * _30.fld3.0.1;
_25.fld5.fld3.2 = _29.fld0 as u128;
_27 = 27871_u16 as i16;
_9 = [_16,_16,_16,_16,_16,_16,_16];
_24 = _29.fld0 as f64;
_8.fld2 = core::ptr::addr_of!(_29.fld1.fld2);
_16 = !826372268_u32;
_12 = _7 as isize;
_25.fld5.fld3.1 = _25.fld5.fld4.0 * _19.0;
_25.fld2 = _25.fld5.fld3.2;
_36 = _17;
_25.fld5.fld3.1 = !_15.fld0.0;
_37.fld7.fld5.fld4 = (_19.0,);
_37.fld7.fld7 = _29.fld1.fld3.0;
_25.fld3 = -_7;
match _14.1 {
0 => bb1,
1 => bb7,
2 => bb12,
3 => bb13,
4 => bb14,
2180 => bb16,
_ => bb15
}
}
bb12 = {
Return()
}
bb13 = {
_14 = (_8.fld1.0, 2180_i16);
_13 = (*_1);
_8.fld0.0 = [_8.fld1.0,_14.0];
_8.fld1.0 = _14.0;
_8.fld3 = [_8.fld1.0];
_9 = [3069082822_u32,4084335939_u32,1155242997_u32,3788426695_u32,719836158_u32,2042028441_u32,3630542007_u32];
_12 = _4;
_10 = _8.fld1.3.0 as f64;
_9 = [1684162479_u32,2435882286_u32,3595201717_u32,2631394410_u32,4041987273_u32,3536423282_u32,2550257879_u32];
_9 = [350247697_u32,168496273_u32,2753838515_u32,3165293388_u32,2226848748_u32,3337032578_u32,1304206975_u32];
_8.fld3 = [_8.fld1.0];
_6.0 = !_8.fld1.3.0;
_15.fld1.fld1 = (-2483737548584574320_i64) as i128;
_17 = '\u{f8c5}';
_17 = '\u{79627}';
_19.0 = !_8.fld1.3.0;
_8.fld1.3 = (_19.0,);
_18 = _17;
match _14.1 {
0 => bb3,
1 => bb4,
2 => bb5,
2180 => bb7,
_ => bb6
}
}
bb14 = {
_6.0 = _8.fld1.3.0;
_12 = _4;
_7 = -71_i8;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_38.fld6.3 = -_25.fld7.1;
_37.fld4 = !16451294129699118812_u64;
_37.fld3.4 = _17;
_37.fld3.2 = !_25.fld2;
_29.fld1 = Adt54 { fld0: _30.fld0,fld1: Move(_30.fld1),fld2: (*RET),fld3: _30.fld3,fld4: _25.fld5.fld4,fld5: _30.fld5 };
_37.fld7.fld5.fld3.3 = _37.fld7.fld7.1;
_38.fld2 = _24 as isize;
_20 = _37.fld4 as i64;
_37.fld7.fld0.fld6 = [_29.fld0,_29.fld1.fld3.0.1,_29.fld1.fld3.0.1,_29.fld1.fld3.0.1];
_38.fld5.0 = -_30.fld5;
(*RET) = [_37.fld3.4,_18,_25.fld5.fld3.4,_36,_30.fld3.4];
_24 = -_28;
_30.fld4 = _25.fld5.fld4;
_31 = [_25.fld7.0,_29.fld1.fld3.0.0,_14.1,_37.fld7.fld7.0,_25.fld7.0,_25.fld5.fld3.0.0,_25.fld7.0,_30.fld3.0.0];
_15 = Adt61 { fld0: _37.fld7.fld5.fld4,fld1: Move(_29.fld1.fld1) };
_8.fld3 = [_25.fld0.fld0];
_37.fld3.3 = _29.fld0 | _29.fld1.fld3.0.1;
_37.fld3.0.0 = _8.fld5 as i16;
_37.fld3.0.1 = _8.fld6 as i128;
_30.fld3.2 = !_37.fld3.2;
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(9_usize, 11_usize, Move(_11), 19_usize, Move(_19), 12_usize, Move(_12), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(9_usize, 3_usize, Move(_3), 6_usize, Move(_6), 18_usize, Move(_18), 17_usize, Move(_17)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(9_usize, 7_usize, Move(_7), 26_usize, Move(_26), 14_usize, Move(_14), 41_usize, _41), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i64,mut _2: isize,mut _3: u32,mut _4: isize,mut _5: isize) -> usize {
mir! {
type RET = usize;
let _6: i64;
let _7: [u8; 8];
let _8: [isize; 8];
let _9: Adt59;
let _10: *const i64;
let _11: i32;
let _12: f64;
let _13: (i32, [char; 2]);
let _14: isize;
let _15: u128;
let _16: *const f32;
let _17: [bool; 1];
let _18: [isize; 8];
let _19: [i16; 8];
let _20: i64;
let _21: [char; 2];
let _22: ();
let _23: ();
{
_1 = !1579440064959232154_i64;
RET = !14847843328273169538_usize;
_4 = _2;
RET = !1_usize;
Call(_1 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 7_usize;
_5 = -_4;
_5 = _4 - _4;
_6 = _1 - _1;
_1 = 640876612_i32 as i64;
Goto(bb2)
}
bb2 = {
_5 = _2 >> _6;
_2 = 251768992_i32 as isize;
_3 = 31951_i16 as u32;
_1 = '\u{82a39}' as i64;
RET = !1228764919432490217_usize;
_5 = -_2;
_6 = 781065884139152577_u64 as i64;
_2 = -_4;
_2 = -_5;
RET = !12169691020892366783_usize;
_5 = _2;
_5 = _4 ^ _2;
_1 = _6;
_3 = !482051731_u32;
_3 = !2619513095_u32;
_6 = _1;
_3 = 3560545498_u32 - 1375904252_u32;
_2 = _5 | _5;
_6 = (-1642591188_i32) as i64;
RET = 6248666216140829070_usize << _1;
_2 = _4 >> _3;
_4 = !_5;
_6 = !_1;
Goto(bb3)
}
bb3 = {
_1 = 14971_u16 as i64;
_4 = 33481_u16 as isize;
RET = (-2_i8) as usize;
_1 = _6;
_7 = [91_u8,132_u8,10_u8,119_u8,184_u8,133_u8,250_u8,197_u8];
RET = 335974375085506627685920020346569688867_u128 as usize;
RET = 2_usize - 4024505174751050804_usize;
_6 = -_1;
RET = !13915528624420110408_usize;
_4 = _5 | _2;
RET = 7_usize >> _4;
_8 = [_4,_2,_4,_4,_4,_4,_2,_2];
_9 = Adt59 { fld0: 131396207251302702403317178350728900089_u128 };
RET = _1 as usize;
_1 = _5 as i64;
_5 = _4 * _4;
_12 = _4 as f64;
_11 = _12 as i32;
_9 = Adt59 { fld0: 53757657323250876407054648307940815125_u128 };
_8 = [_4,_4,_2,_2,_5,_5,_5,_5];
_2 = _11 as isize;
match _9.fld0 {
53757657323250876407054648307940815125 => bb4,
_ => bb2
}
}
bb4 = {
_13.0 = _11;
match _9.fld0 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
53757657323250876407054648307940815125 => bb8,
_ => bb7
}
}
bb5 = {
_1 = 14971_u16 as i64;
_4 = 33481_u16 as isize;
RET = (-2_i8) as usize;
_1 = _6;
_7 = [91_u8,132_u8,10_u8,119_u8,184_u8,133_u8,250_u8,197_u8];
RET = 335974375085506627685920020346569688867_u128 as usize;
RET = 2_usize - 4024505174751050804_usize;
_6 = -_1;
RET = !13915528624420110408_usize;
_4 = _5 | _2;
RET = 7_usize >> _4;
_8 = [_4,_2,_4,_4,_4,_4,_2,_2];
_9 = Adt59 { fld0: 131396207251302702403317178350728900089_u128 };
RET = _1 as usize;
_1 = _5 as i64;
_5 = _4 * _4;
_12 = _4 as f64;
_11 = _12 as i32;
_9 = Adt59 { fld0: 53757657323250876407054648307940815125_u128 };
_8 = [_4,_4,_2,_2,_5,_5,_5,_5];
_2 = _11 as isize;
match _9.fld0 {
53757657323250876407054648307940815125 => bb4,
_ => bb2
}
}
bb6 = {
_5 = _2 >> _6;
_2 = 251768992_i32 as isize;
_3 = 31951_i16 as u32;
_1 = '\u{82a39}' as i64;
RET = !1228764919432490217_usize;
_5 = -_2;
_6 = 781065884139152577_u64 as i64;
_2 = -_4;
_2 = -_5;
RET = !12169691020892366783_usize;
_5 = _2;
_5 = _4 ^ _2;
_1 = _6;
_3 = !482051731_u32;
_3 = !2619513095_u32;
_6 = _1;
_3 = 3560545498_u32 - 1375904252_u32;
_2 = _5 | _5;
_6 = (-1642591188_i32) as i64;
RET = 6248666216140829070_usize << _1;
_2 = _4 >> _3;
_4 = !_5;
_6 = !_1;
Goto(bb3)
}
bb7 = {
RET = 7_usize;
_5 = -_4;
_5 = _4 - _4;
_6 = _1 - _1;
_1 = 640876612_i32 as i64;
Goto(bb2)
}
bb8 = {
_9 = Adt59 { fld0: 81215117552020864450796724497830358895_u128 };
_2 = (-76_i8) as isize;
_4 = _2;
_14 = -_5;
_10 = core::ptr::addr_of!(_1);
_3 = 31314_i16 as u32;
_12 = _3 as f64;
_14 = -_5;
(*_10) = _6;
(*_10) = -_6;
_3 = 3371756615_u32 | 3137075514_u32;
_5 = -_14;
_2 = _14 >> _11;
_9 = Adt59 { fld0: 229015152690540347153643155079117345067_u128 };
_15 = _3 as u128;
_15 = !_9.fld0;
_12 = (*_10) as f64;
RET = !7218071430445595740_usize;
(*_10) = _6 >> _13.0;
_13.1 = ['\u{ba100}','\u{14613}'];
_3 = '\u{3edc6}' as u32;
_13.0 = _11 << _1;
_11 = _13.0;
match _9.fld0 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
229015152690540347153643155079117345067 => bb14,
_ => bb13
}
}
bb9 = {
RET = 7_usize;
_5 = -_4;
_5 = _4 - _4;
_6 = _1 - _1;
_1 = 640876612_i32 as i64;
Goto(bb2)
}
bb10 = {
_5 = _2 >> _6;
_2 = 251768992_i32 as isize;
_3 = 31951_i16 as u32;
_1 = '\u{82a39}' as i64;
RET = !1228764919432490217_usize;
_5 = -_2;
_6 = 781065884139152577_u64 as i64;
_2 = -_4;
_2 = -_5;
RET = !12169691020892366783_usize;
_5 = _2;
_5 = _4 ^ _2;
_1 = _6;
_3 = !482051731_u32;
_3 = !2619513095_u32;
_6 = _1;
_3 = 3560545498_u32 - 1375904252_u32;
_2 = _5 | _5;
_6 = (-1642591188_i32) as i64;
RET = 6248666216140829070_usize << _1;
_2 = _4 >> _3;
_4 = !_5;
_6 = !_1;
Goto(bb3)
}
bb11 = {
_1 = 14971_u16 as i64;
_4 = 33481_u16 as isize;
RET = (-2_i8) as usize;
_1 = _6;
_7 = [91_u8,132_u8,10_u8,119_u8,184_u8,133_u8,250_u8,197_u8];
RET = 335974375085506627685920020346569688867_u128 as usize;
RET = 2_usize - 4024505174751050804_usize;
_6 = -_1;
RET = !13915528624420110408_usize;
_4 = _5 | _2;
RET = 7_usize >> _4;
_8 = [_4,_2,_4,_4,_4,_4,_2,_2];
_9 = Adt59 { fld0: 131396207251302702403317178350728900089_u128 };
RET = _1 as usize;
_1 = _5 as i64;
_5 = _4 * _4;
_12 = _4 as f64;
_11 = _12 as i32;
_9 = Adt59 { fld0: 53757657323250876407054648307940815125_u128 };
_8 = [_4,_4,_2,_2,_5,_5,_5,_5];
_2 = _11 as isize;
match _9.fld0 {
53757657323250876407054648307940815125 => bb4,
_ => bb2
}
}
bb12 = {
_5 = _2 >> _6;
_2 = 251768992_i32 as isize;
_3 = 31951_i16 as u32;
_1 = '\u{82a39}' as i64;
RET = !1228764919432490217_usize;
_5 = -_2;
_6 = 781065884139152577_u64 as i64;
_2 = -_4;
_2 = -_5;
RET = !12169691020892366783_usize;
_5 = _2;
_5 = _4 ^ _2;
_1 = _6;
_3 = !482051731_u32;
_3 = !2619513095_u32;
_6 = _1;
_3 = 3560545498_u32 - 1375904252_u32;
_2 = _5 | _5;
_6 = (-1642591188_i32) as i64;
RET = 6248666216140829070_usize << _1;
_2 = _4 >> _3;
_4 = !_5;
_6 = !_1;
Goto(bb3)
}
bb13 = {
_1 = 14971_u16 as i64;
_4 = 33481_u16 as isize;
RET = (-2_i8) as usize;
_1 = _6;
_7 = [91_u8,132_u8,10_u8,119_u8,184_u8,133_u8,250_u8,197_u8];
RET = 335974375085506627685920020346569688867_u128 as usize;
RET = 2_usize - 4024505174751050804_usize;
_6 = -_1;
RET = !13915528624420110408_usize;
_4 = _5 | _2;
RET = 7_usize >> _4;
_8 = [_4,_2,_4,_4,_4,_4,_2,_2];
_9 = Adt59 { fld0: 131396207251302702403317178350728900089_u128 };
RET = _1 as usize;
_1 = _5 as i64;
_5 = _4 * _4;
_12 = _4 as f64;
_11 = _12 as i32;
_9 = Adt59 { fld0: 53757657323250876407054648307940815125_u128 };
_8 = [_4,_4,_2,_2,_5,_5,_5,_5];
_2 = _11 as isize;
match _9.fld0 {
53757657323250876407054648307940815125 => bb4,
_ => bb2
}
}
bb14 = {
_13.1 = ['\u{fe147}','\u{3b077}'];
_1 = _6;
_15 = _9.fld0;
RET = 30793_u16 as usize;
_14 = _12 as isize;
_9.fld0 = !_15;
_3 = 15669_u16 as u32;
_15 = _9.fld0 | _9.fld0;
_9.fld0 = _15 % 53123720885606283646861943149386566422_u128;
_20 = -(*_10);
_8 = [_2,_2,_5,_2,_2,_2,_2,_5];
_10 = core::ptr::addr_of!(_20);
_18 = _8;
_4 = _2;
_15 = _9.fld0 & _9.fld0;
_8 = [_4,_4,_4,_2,_2,_5,_4,_4];
_14 = -_2;
_15 = _9.fld0 - _9.fld0;
(*_10) = _1;
(*_10) = _6 ^ _1;
_13.1 = ['\u{8354a}','\u{30400}'];
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(10_usize, 5_usize, Move(_5), 14_usize, Move(_14), 2_usize, Move(_2), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(10_usize, 20_usize, Move(_20), 8_usize, Move(_8), 7_usize, Move(_7), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: usize,mut _2: u64,mut _3: u128,mut _4: u128,mut _5: [char; 5],mut _6: (i16, i128),mut _7: usize,mut _8: *const u128,mut _9: usize,mut _10: f64,mut _11: *const u128,mut _12: isize,mut _13: [u32; 7],mut _14: ((i16, i128), usize, u128, i128, char)) -> isize {
mir! {
type RET = isize;
let _15: f32;
let _16: [u32; 7];
let _17: Adt58;
let _18: bool;
let _19: Adt48;
let _20: Adt63;
let _21: u32;
let _22: ([bool; 2],);
let _23: (bool, i16);
let _24: i64;
let _25: i8;
let _26: isize;
let _27: bool;
let _28: [u8; 8];
let _29: bool;
let _30: char;
let _31: char;
let _32: u8;
let _33: [i128; 4];
let _34: [char; 5];
let _35: Adt61;
let _36: Adt56;
let _37: [i16; 8];
let _38: (bool, i128, *const i64, (usize,));
let _39: [i128; 4];
let _40: Adt49;
let _41: Adt56;
let _42: isize;
let _43: i128;
let _44: ();
let _45: ();
{
_12 = -53_isize;
_14.0.1 = false as i128;
_14.4 = '\u{2c771}';
_5 = [_14.4,_14.4,_14.4,_14.4,_14.4];
_14.0.0 = 15_u8 as i16;
_14.0.1 = _6.1 << _12;
_2 = _7 as u64;
_14.2 = (*_11) % 79595435038344096566848884620752470335_u128;
_9 = _14.0.1 as usize;
RET = !_12;
_2 = 14180560175095952019_u64;
Goto(bb1)
}
bb1 = {
(*_8) = _4;
_16 = _13;
_17.fld2 = Adt52 { fld0: _8,fld1: _14.0.1 };
_17.fld0.fld1 = _2 as f32;
_19.fld6.3 = _6.1 + _14.0.1;
RET = _6.0 as isize;
(*_8) = _3;
_20.fld4.fld0.1 = !_6.0;
_14.0.1 = _19.fld6.3;
_20.fld2.fld0.1 = (false, _20.fld4.fld0.1);
_6.1 = _19.fld6.3;
_2 = 6503690621130719018_u64;
RET = -_12;
_20.fld4.fld1.0 = _7;
_19.fld2 = _9 as isize;
_19.fld3 = [_20.fld2.fld0.1.0];
_19.fld4.1 = !_14.0.0;
_19.fld4 = _20.fld2.fld0.1;
_19.fld5.1 = [_14.4,_14.4];
Call(_17.fld0.fld1 = fn12(_19.fld2, _2, _12, _7, _10, _14.1, _2, _12, (*_11), _2, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14.0.0 = _19.fld4.1 << _14.0.1;
Goto(bb3)
}
bb3 = {
_20.fld2.fld3 = 160_u8 ^ 72_u8;
_20.fld4.fld1 = (_9,);
_20.fld4.fld4.fld2 = -_19.fld2;
_8 = core::ptr::addr_of!((*_11));
_23.1 = -_14.0.0;
_19.fld0 = [_19.fld2,_19.fld2,_19.fld2,_19.fld2,_20.fld4.fld4.fld2,_19.fld2,_12,_20.fld4.fld4.fld2];
_17.fld0.fld2 = _20.fld4.fld0.1 as u32;
_8 = core::ptr::addr_of!((*_8));
_20.fld4.fld4.fld6.0 = (_6.0, _6.1);
_17.fld1 = _19.fld5.1;
_20.fld4.fld4.fld4.0 = !_20.fld2.fld0.1.0;
_19.fld6.4 = _14.4;
_19.fld1 = core::ptr::addr_of_mut!(_20.fld2.fld0.2);
_19.fld0 = [_19.fld2,_19.fld2,_20.fld4.fld4.fld2,_19.fld2,_12,_19.fld2,_20.fld4.fld4.fld2,_19.fld2];
RET = _19.fld2;
RET = !_19.fld2;
Goto(bb4)
}
bb4 = {
_19.fld2 = !_20.fld4.fld4.fld2;
_21 = !_17.fld0.fld2;
_5 = [_19.fld6.4,_19.fld6.4,_14.4,_14.4,_19.fld6.4];
_20.fld2.fld0.1.1 = 453521440_i32 as i16;
_23.0 = _19.fld4.0 ^ _20.fld2.fld0.1.0;
_20.fld2.fld0.2 = [50707_u16,37614_u16,34917_u16,32372_u16,37318_u16];
_9 = _14.0.1 as usize;
_19.fld6 = (_6, _20.fld4.fld1.0, _14.2, _17.fld2.fld1, _14.4);
_20.fld4.fld4.fld1 = core::ptr::addr_of_mut!(_17.fld0.fld0.0);
_20.fld4.fld4.fld6 = (_6, _9, _19.fld6.2, _17.fld2.fld1, _14.4);
_14.0.0 = (*_11) as i16;
_27 = !_23.0;
_12 = -_20.fld4.fld4.fld2;
_28 = [_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3];
_20.fld4.fld4.fld5 = (1375907633_i32, _17.fld1);
_14.4 = _19.fld6.4;
_19.fld5 = (_20.fld4.fld4.fld5.0, _20.fld4.fld4.fld5.1);
_19.fld5.0 = !_20.fld4.fld4.fld5.0;
_25 = 57_i8;
_19.fld4 = (_20.fld4.fld4.fld4.0, _23.1);
_20.fld4.fld4.fld1 = _19.fld1;
_20.fld4.fld4.fld6.0.0 = !_23.1;
_20.fld4.fld3 = -_17.fld0.fld1;
_20.fld4.fld4.fld0 = [_20.fld4.fld4.fld2,_12,_20.fld4.fld4.fld2,_12,_20.fld4.fld4.fld2,_19.fld2,_20.fld4.fld4.fld2,_19.fld2];
_14.2 = _20.fld4.fld4.fld6.2 | (*_8);
_19.fld0 = [_20.fld4.fld4.fld2,_20.fld4.fld4.fld2,_12,_12,_20.fld4.fld4.fld2,_20.fld4.fld4.fld2,_12,_19.fld2];
_20.fld2.fld1 = -_20.fld4.fld3;
Call(_17.fld0.fld0.4 = core::intrinsics::transmute(_13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17.fld0.fld0.3 = [_14.0.1,_20.fld4.fld4.fld6.3,_19.fld6.0.1,_17.fld2.fld1,_20.fld4.fld4.fld6.0.1,_20.fld4.fld4.fld6.3,_17.fld2.fld1,_6.1];
_19.fld4.0 = _27;
_20.fld4.fld0.0 = !_19.fld4.0;
_7 = _20.fld4.fld4.fld6.1 - _14.1;
_20.fld4.fld4.fld6.0 = _14.0;
_28 = [_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3];
Call(_20.fld2.fld2 = core::intrinsics::bswap(_21), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_17.fld4 = [_19.fld6.3,_17.fld2.fld1,_6.1,_20.fld4.fld4.fld6.0.1];
_26 = _19.fld6.4 as isize;
_17.fld0.fld3 = !_20.fld2.fld3;
_20.fld4.fld4.fld4 = (_20.fld4.fld0.0, _23.1);
_20.fld4.fld4.fld4.0 = !_20.fld4.fld0.0;
_17.fld0.fld0.0 = [17809_u16,230_u16,40936_u16,51564_u16,16002_u16];
_29 = _20.fld4.fld4.fld4.0;
_20.fld4.fld0 = (_27, _19.fld4.1);
_20.fld4.fld4.fld6.1 = _19.fld6.0.1 as usize;
_20.fld4.fld4.fld3 = [_29];
_3 = _19.fld6.3 as u128;
_25 = _2 as i8;
(*_11) = _19.fld6.2 << _14.0.1;
_17.fld0.fld3 = _19.fld5.0 as u8;
_33 = _17.fld4;
_20.fld6.0 = [_19.fld4.0,_27];
_17.fld4 = _33;
_33 = [_14.0.1,_19.fld6.3,_19.fld6.3,_20.fld4.fld4.fld6.3];
Goto(bb7)
}
bb7 = {
_19.fld6 = _20.fld4.fld4.fld6;
RET = _26 + _12;
_3 = !(*_11);
_24 = (-296787995593553800_i64);
_20.fld4.fld4.fld4.0 = !_23.0;
_20.fld4.fld2 = _20.fld4.fld4.fld1;
_14.0.0 = _20.fld2.fld1 as i16;
Call(_35.fld0.0 = core::intrinsics::transmute(_19.fld6.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20.fld3 = _2 | _2;
_20.fld4.fld4.fld6.2 = (*_8) ^ (*_11);
Goto(bb9)
}
bb9 = {
_36.fld7.fld5 = Adt54 { fld0: _16,fld1: Move(_17.fld2),fld2: _5,fld3: _14,fld4: _35.fld0,fld5: _19.fld5.0 };
_20.fld4.fld4.fld6.1 = !_7;
_36.fld7.fld4 = _17.fld0.fld0.4;
_20.fld4.fld4.fld3 = [_27];
_36.fld3.2 = _20.fld4.fld4.fld6.2;
_20.fld4.fld4.fld6.0.1 = _12 as i128;
_35.fld0.0 = !_20.fld4.fld4.fld6.1;
_36.fld0 = _17.fld0.fld3;
_2 = !_20.fld3;
_35.fld0 = (_20.fld4.fld4.fld6.1,);
_4 = _17.fld0.fld2 as u128;
(*_8) = !_20.fld4.fld4.fld6.2;
_19.fld5.0 = _36.fld7.fld5.fld5 & _36.fld7.fld5.fld5;
_9 = !_36.fld7.fld5.fld4.0;
_36.fld3.3 = !_36.fld7.fld5.fld3.0.1;
_38.3 = (_20.fld4.fld4.fld6.1,);
_14.1 = _20.fld4.fld4.fld6.1 / 291323141294831226_usize;
Call(_20.fld2.fld0.4 = fn14(_20.fld4.fld4.fld5.1, _26, _20.fld4.fld0.1, _12, _14.0.1, _20.fld4.fld4.fld1, _20.fld2.fld2, _1, _24, _14.3, _11, _19.fld2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_36.fld7.fld2 = _24 as u128;
_38.0 = !_20.fld4.fld0.0;
_36.fld7.fld0.fld1.fld1 = -_36.fld7.fld5.fld3.0.1;
_20.fld2.fld0.1.1 = -_19.fld4.1;
_36.fld7.fld6 = [_12,_19.fld2,_19.fld2,_12,_12,_20.fld4.fld4.fld2,_20.fld4.fld4.fld2,_12];
_4 = !_14.2;
_6.1 = !_36.fld7.fld5.fld3.0.1;
_36.fld7.fld0.fld4 = _23.1 << _26;
_36.fld4 = !_20.fld3;
_19.fld4.1 = _19.fld6.0.0 & _20.fld4.fld0.1;
_20.fld2.fld0.1 = (_23.0, _20.fld4.fld4.fld4.1);
_35 = Adt61 { fld0: _36.fld7.fld5.fld4,fld1: Move(_36.fld7.fld5.fld1) };
_40.fld0.fld5 = (_19.fld5.0, _17.fld1);
_40.fld0.fld5.1 = _17.fld1;
_36.fld6 = _17.fld0.fld2 - _17.fld0.fld2;
(*_8) = _17.fld0.fld2 as u128;
_36.fld7.fld1 = _17.fld0.fld1 as usize;
_36.fld7.fld1 = _20.fld4.fld4.fld6.3 as usize;
_19.fld6.4 = _20.fld4.fld4.fld6.4;
(*_11) = !_20.fld4.fld4.fld6.2;
_36.fld4 = _2;
_20.fld4.fld4.fld4.0 = !_20.fld2.fld0.1.0;
_20.fld2.fld0.4 = [_17.fld0.fld2,_17.fld0.fld2,_17.fld0.fld2,_36.fld6,_36.fld6,_21,_36.fld6];
_23 = _20.fld4.fld4.fld4;
_20.fld4.fld4.fld6.4 = _36.fld7.fld5.fld3.4;
_19.fld6.0.0 = _19.fld4.1;
Call(_32 = fn15(_20.fld4.fld0.0, _19.fld1, _3, _40.fld0.fld5, _13, _20.fld2.fld1, _20.fld4.fld4.fld5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_35.fld1.fld0 = _11;
_40.fld0 = Adt48 { fld0: _20.fld4.fld4.fld0,fld1: _20.fld4.fld4.fld1,fld2: _20.fld4.fld4.fld2,fld3: _20.fld4.fld4.fld3,fld4: _19.fld4,fld5: _19.fld5,fld6: _19.fld6 };
_20.fld4.fld0.1 = 65293_u16 as i16;
_41.fld7.fld7 = (_14.0.0, _40.fld0.fld6.0.1);
_20.fld4.fld2 = core::ptr::addr_of_mut!(_20.fld2.fld0.2);
_41.fld7.fld5.fld3 = _14;
_40.fld1 = [_12,_19.fld2,_20.fld4.fld4.fld2,_20.fld4.fld4.fld2,_19.fld2];
(*_8) = _36.fld7.fld5.fld3.2;
_40.fld0.fld4.1 = -_40.fld0.fld6.0.0;
_17.fld0.fld0.4 = [_36.fld6,_17.fld0.fld2,_17.fld0.fld2,_36.fld6,_21,_17.fld0.fld2,_36.fld6];
match _24 {
340282366920938463463077819436174657656 => bb13,
_ => bb12
}
}
bb12 = {
_19.fld2 = !_20.fld4.fld4.fld2;
_21 = !_17.fld0.fld2;
_5 = [_19.fld6.4,_19.fld6.4,_14.4,_14.4,_19.fld6.4];
_20.fld2.fld0.1.1 = 453521440_i32 as i16;
_23.0 = _19.fld4.0 ^ _20.fld2.fld0.1.0;
_20.fld2.fld0.2 = [50707_u16,37614_u16,34917_u16,32372_u16,37318_u16];
_9 = _14.0.1 as usize;
_19.fld6 = (_6, _20.fld4.fld1.0, _14.2, _17.fld2.fld1, _14.4);
_20.fld4.fld4.fld1 = core::ptr::addr_of_mut!(_17.fld0.fld0.0);
_20.fld4.fld4.fld6 = (_6, _9, _19.fld6.2, _17.fld2.fld1, _14.4);
_14.0.0 = (*_11) as i16;
_27 = !_23.0;
_12 = -_20.fld4.fld4.fld2;
_28 = [_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3,_20.fld2.fld3];
_20.fld4.fld4.fld5 = (1375907633_i32, _17.fld1);
_14.4 = _19.fld6.4;
_19.fld5 = (_20.fld4.fld4.fld5.0, _20.fld4.fld4.fld5.1);
_19.fld5.0 = !_20.fld4.fld4.fld5.0;
_25 = 57_i8;
_19.fld4 = (_20.fld4.fld4.fld4.0, _23.1);
_20.fld4.fld4.fld1 = _19.fld1;
_20.fld4.fld4.fld6.0.0 = !_23.1;
_20.fld4.fld3 = -_17.fld0.fld1;
_20.fld4.fld4.fld0 = [_20.fld4.fld4.fld2,_12,_20.fld4.fld4.fld2,_12,_20.fld4.fld4.fld2,_19.fld2,_20.fld4.fld4.fld2,_19.fld2];
_14.2 = _20.fld4.fld4.fld6.2 | (*_8);
_19.fld0 = [_20.fld4.fld4.fld2,_20.fld4.fld4.fld2,_12,_12,_20.fld4.fld4.fld2,_20.fld4.fld4.fld2,_12,_19.fld2];
_20.fld2.fld1 = -_20.fld4.fld3;
Call(_17.fld0.fld0.4 = core::intrinsics::transmute(_13), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_14.0 = (_20.fld4.fld4.fld4.1, _41.fld7.fld7.1);
_17.fld0.fld3 = !_32;
_37 = [_20.fld4.fld4.fld6.0.0,_40.fld0.fld4.1,_20.fld4.fld4.fld4.1,_19.fld6.0.0,_23.1,_20.fld4.fld4.fld4.1,_40.fld0.fld6.0.0,_20.fld2.fld0.1.1];
_18 = _20.fld4.fld4.fld6.1 != _7;
_7 = _24 as usize;
Goto(bb14)
}
bb14 = {
_41.fld7.fld5.fld3.3 = _19.fld6.0.1 * _6.1;
_26 = _10 as isize;
(*_8) = !_36.fld3.2;
_36.fld1 = core::ptr::addr_of_mut!(_20.fld2.fld0.2);
_40.fld0.fld6.4 = _20.fld4.fld4.fld6.4;
_41.fld7.fld5.fld3.0 = _41.fld7.fld7;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(11_usize, 3_usize, Move(_3), 1_usize, Move(_1), 27_usize, Move(_27), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(11_usize, 16_usize, Move(_16), 24_usize, Move(_24), 33_usize, Move(_33), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(11_usize, 21_usize, Move(_21), 9_usize, Move(_9), 32_usize, Move(_32), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: u64,mut _3: isize,mut _4: usize,mut _5: f64,mut _6: usize,mut _7: u64,mut _8: isize,mut _9: u128,mut _10: u64,mut _11: ((i16, i128), usize, u128, i128, char)) -> f32 {
mir! {
type RET = f32;
let _12: isize;
let _13: u64;
let _14: Adt57;
let _15: isize;
let _16: char;
let _17: [bool; 2];
let _18: Adt57;
let _19: Adt50;
let _20: isize;
let _21: bool;
let _22: ();
let _23: ();
{
_11.0.0 = 43634_u16 as i16;
RET = 120_u8 as f32;
RET = _11.2 as f32;
_11.4 = '\u{1481}';
_11.2 = _9 & _9;
RET = 107_i8 as f32;
_6 = _11.1;
_14.fld0.3 = [_11.0.1,_11.3,_11.0.1,_11.0.1,_11.0.1,_11.0.1,_11.0.1,_11.0.1];
RET = 222_u8 as f32;
_14.fld0.3 = [_11.0.1,_11.0.1,_11.3,_11.0.1,_11.0.1,_11.0.1,_11.0.1,_11.3];
_6 = _11.1;
_14.fld0.4 = [1292629500_u32,1693105247_u32,612599915_u32,1228835181_u32,1138028433_u32,37676273_u32,3533870701_u32];
_12 = _1 >> _6;
_14.fld2 = 1164920529_u32 * 2000960753_u32;
_11.0 = (21308_i16, _11.3);
_10 = _7;
_11.4 = '\u{c5629}';
_11.0.0 = 215_u8 as i16;
_15 = -_1;
_18.fld1 = _11.0.0 as f32;
_6 = _11.1 ^ _11.1;
_18.fld0.2 = [29118_u16,10094_u16,13364_u16,49799_u16,27880_u16];
match _11.0.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
43455926409780245612125181517449252937 => bb8,
_ => bb7
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_10 = 5689753770652120667_i64 as u64;
_14.fld0.4 = [_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2];
_13 = !_7;
_11.0.1 = _11.3;
_14.fld3 = 15647_u16 as u8;
_18.fld1 = _5 as f32;
_14.fld0.1 = (true, _11.0.0);
_11.0.1 = _11.3 ^ _11.3;
_18.fld3 = _5 as u8;
_7 = !_2;
_5 = _18.fld1 as f64;
_18.fld0.1 = (_14.fld0.1.0, _11.0.0);
_11.3 = _11.0.1 >> _3;
_14.fld0.0 = [16622_u16,49168_u16,54117_u16,49610_u16,49216_u16];
_8 = _15;
_11.0 = (_18.fld0.1.1, _11.3);
_18.fld2 = !_14.fld2;
_14.fld0.1 = (_18.fld0.1.0, _11.0.0);
_9 = _11.2 & _11.2;
_14.fld3 = _18.fld3 & _18.fld3;
Call(_11.0 = fn13(_11.4, _11.2, _18.fld0.1.1, _18.fld0.1.0, _3, _13, _18.fld3, _11.2, _18.fld0.1.0, _2, _8, _14.fld2, _10, _10, _18.fld0.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14.fld2 = _18.fld2 & _18.fld2;
_8 = (-668541132_i32) as isize;
match _2 {
0 => bb10,
1 => bb11,
2 => bb12,
6503690621130719018 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_8 = _14.fld0.1.0 as isize;
_19.fld2 = [_8,_15,_12,_12,_12];
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(12_usize, 2_usize, Move(_2), 11_usize, Move(_11), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(12_usize, 8_usize, Move(_8), 3_usize, Move(_3), 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: char,mut _2: u128,mut _3: i16,mut _4: bool,mut _5: isize,mut _6: u64,mut _7: u8,mut _8: u128,mut _9: bool,mut _10: u64,mut _11: isize,mut _12: u32,mut _13: u64,mut _14: u64,mut _15: (bool, i16)) -> (i16, i128) {
mir! {
type RET = (i16, i128);
let _16: bool;
let _17: (bool, i128, *const i64, (usize,));
let _18: bool;
let _19: Adt57;
let _20: f64;
let _21: u128;
let _22: isize;
let _23: [i16; 8];
let _24: Adt64;
let _25: i32;
let _26: u16;
let _27: bool;
let _28: isize;
let _29: *const [char; 5];
let _30: ([u16; 5], (bool, i16), [u16; 5], [i128; 8], [u32; 7]);
let _31: Adt59;
let _32: (i32, [char; 2]);
let _33: char;
let _34: (bool, i128, *const i64, (usize,));
let _35: isize;
let _36: ();
let _37: ();
{
RET.0 = _1 as i16;
RET = (_15.1, (-5193404040779513597686569197970309035_i128));
RET.0 = _3 - _15.1;
_5 = _11 & _11;
_2 = _8;
_2 = _8 ^ _8;
_14 = !_6;
_5 = _11;
_1 = '\u{35659}';
_2 = _1 as u128;
_4 = _9 ^ _15.0;
_2 = _15.1 as u128;
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6503690621130719018 => bb8,
_ => bb7
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
RET.0 = !_3;
_16 = !_15.0;
match _10 {
0 => bb1,
1 => bb4,
6503690621130719018 => bb9,
_ => bb6
}
}
bb9 = {
_19.fld0.4 = [_12,_12,_12,_12,_12,_12,_12];
RET.0 = _15.1;
RET.1 = _1 as i128;
RET.0 = (-153727155228643766789637644802680594114_i128) as i16;
_17.1 = _8 as i128;
_15 = (_4, _3);
_20 = _17.1 as f64;
_19.fld0.1.1 = _3;
_19.fld0.1.1 = _3;
_15.1 = _3 - _19.fld0.1.1;
_17.3.0 = (-1566491201960923057_i64) as usize;
_21 = (-260112527_i32) as u128;
_14 = !_13;
RET.1 = _10 as i128;
RET.0 = _15.1 * _15.1;
_19.fld0.1 = (_9, _15.1);
RET = (_15.1, _17.1);
Call(_24.fld1.fld3.3 = core::intrinsics::transmute(_2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19.fld0.2 = [43630_u16,33198_u16,53685_u16,53948_u16,31425_u16];
_19.fld0.0 = _19.fld0.2;
_24.fld1.fld0 = [_12,_12,_12,_12,_12,_12,_12];
_7 = !116_u8;
_9 = _4 == _15.0;
_4 = _9;
_24.fld1.fld4 = _17.3;
_22 = _12 as isize;
_24.fld1.fld0 = [_12,_12,_12,_12,_12,_12,_12];
_17.0 = _9;
_10 = _14 % 4556460301345772373_u64;
_24.fld0 = _21 as i128;
_20 = (-629834986_i32) as f64;
_18 = _17.0;
_19.fld0.1.1 = _20 as i16;
_24.fld0 = 8143423137785725962_i64 as i128;
_24.fld1.fld4.0 = _17.3.0 + _17.3.0;
_5 = !_22;
Goto(bb11)
}
bb11 = {
_5 = _24.fld1.fld4.0 as isize;
_15.1 = 30226_u16 as i16;
_12 = _8 as u32;
_24.fld1.fld3.0.0 = -_3;
_24.fld2 = [_17.0,_18];
RET = (_3, _24.fld0);
_20 = _19.fld0.1.1 as f64;
RET = (_24.fld1.fld3.0.0, _17.1);
_24.fld1.fld2 = [_1,_1,_1,_1,_1];
_24.fld1.fld3.3 = _17.1;
_25 = 1436037464_i32 | 488307350_i32;
_3 = _15.1 * _24.fld1.fld3.0.0;
_19.fld0.1.0 = _16;
_16 = _10 < _6;
_3 = !_19.fld0.1.1;
_24.fld1.fld3.0 = (_3, _24.fld1.fld3.3);
_15.0 = !_17.0;
_19.fld0.1.0 = _4 <= _17.0;
Goto(bb12)
}
bb12 = {
_17.3 = _24.fld1.fld4;
Goto(bb13)
}
bb13 = {
_7 = 11_u8;
RET = (_3, _24.fld1.fld3.3);
_19.fld0.1 = (_9, _15.1);
_22 = _5 * _5;
_19.fld0.3 = [_24.fld1.fld3.3,_24.fld1.fld3.3,_24.fld1.fld3.3,_24.fld1.fld3.3,_17.1,_24.fld1.fld3.3,_17.1,_24.fld1.fld3.0.1];
_24.fld1.fld2 = [_1,_1,_1,_1,_1];
_2 = _24.fld0 as u128;
_15.1 = _3 + _19.fld0.1.1;
_24.fld1.fld3.4 = _1;
_11 = -_22;
_24.fld1.fld1.fld1 = _17.1 ^ _24.fld1.fld3.3;
_24.fld2 = [_18,_19.fld0.1.0];
_24.fld1.fld1.fld0 = core::ptr::addr_of!(_21);
_1 = _24.fld1.fld3.4;
_24.fld1.fld3.0.1 = _24.fld1.fld1.fld1 & _24.fld1.fld1.fld1;
_24.fld1.fld3.2 = _21;
_24.fld1.fld3.3 = _24.fld1.fld3.0.1;
_19.fld2 = _25 as u32;
_19.fld3 = _24.fld1.fld3.2 as u8;
RET.1 = _24.fld1.fld3.3 - _24.fld1.fld3.3;
_19.fld0.1 = (_9, _15.1);
Goto(bb14)
}
bb14 = {
Call(_36 = dump_var(13_usize, 25_usize, Move(_25), 15_usize, Move(_15), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_36 = dump_var(13_usize, 9_usize, Move(_9), 8_usize, Move(_8), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(13_usize, 14_usize, Move(_14), 22_usize, Move(_22), 37_usize, _37, 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [char; 2],mut _2: isize,mut _3: i16,mut _4: isize,mut _5: i128,mut _6: *mut [u16; 5],mut _7: u32,mut _8: usize,mut _9: i64,mut _10: i128,mut _11: *const u128,mut _12: isize) -> [u32; 7] {
mir! {
type RET = [u32; 7];
let _13: *const u128;
let _14: [u32; 7];
let _15: Adt62;
let _16: i128;
let _17: *const f32;
let _18: f64;
let _19: Adt49;
let _20: isize;
let _21: ();
let _22: ();
{
_12 = _4 & _4;
(*_6) = [45541_u16,9943_u16,30137_u16,31414_u16,58883_u16];
Goto(bb1)
}
bb1 = {
_6 = core::ptr::addr_of_mut!((*_6));
_9 = -4342478707736411462_i64;
_15.fld0.fld3 = [951_u16];
_15.fld2.fld0 = (_8,);
RET = [2725771624_u32,3689311015_u32,3450513786_u32,897513155_u32,2017130918_u32,494425002_u32,720683194_u32];
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
43455926409780245612125181517449252937 => bb6,
_ => bb5
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_15.fld0.fld4 = [_5,_5,_5,_5];
_15.fld2.fld1.fld1 = -_5;
_15.fld0.fld3 = [62592_u16];
_15.fld0.fld2.fld0 = core::ptr::addr_of!((*_11));
_13 = core::ptr::addr_of!((*_11));
_15.fld2.fld1.fld0 = core::ptr::addr_of!((*_11));
_15.fld0.fld0.fld0.0 = (*_6);
(*_11) = !178530845601296389263742723471455067247_u128;
_15.fld0.fld2 = Adt52 { fld0: _13,fld1: _10 };
_15.fld0.fld2.fld0 = core::ptr::addr_of!((*_13));
_17 = core::ptr::addr_of!(_15.fld0.fld0.fld1);
(*_11) = !70897151717745392437459349275992888038_u128;
_17 = core::ptr::addr_of!((*_17));
Goto(bb7)
}
bb7 = {
(*_17) = 543843835875032035_u64 as f32;
(*_13) = !123767285976823218603639815458461877248_u128;
_5 = !_15.fld2.fld1.fld1;
_15.fld0.fld0.fld0.1 = (true, _3);
Goto(bb8)
}
bb8 = {
_15.fld0.fld1 = ['\u{37150}','\u{215a2}'];
_10 = -_15.fld2.fld1.fld1;
_16 = _9 as i128;
(*_11) = 139777162180347663196381711031326058058_u128 - 58304462551124413838863567100802133885_u128;
_15.fld0.fld0.fld0.2 = [13895_u16,46651_u16,45020_u16,34293_u16,62746_u16];
_19.fld0.fld5 = (899862074_i32, _15.fld0.fld1);
_19.fld0.fld6.1 = _8;
_19.fld0.fld4 = (_15.fld0.fld0.fld0.1.0, _15.fld0.fld0.fld0.1.1);
match _19.fld0.fld5.0 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
899862074 => bb15,
_ => bb14
}
}
bb9 = {
(*_17) = 543843835875032035_u64 as f32;
(*_13) = !123767285976823218603639815458461877248_u128;
_5 = !_15.fld2.fld1.fld1;
_15.fld0.fld0.fld0.1 = (true, _3);
Goto(bb8)
}
bb10 = {
_15.fld0.fld4 = [_5,_5,_5,_5];
_15.fld2.fld1.fld1 = -_5;
_15.fld0.fld3 = [62592_u16];
_15.fld0.fld2.fld0 = core::ptr::addr_of!((*_11));
_13 = core::ptr::addr_of!((*_11));
_15.fld2.fld1.fld0 = core::ptr::addr_of!((*_11));
_15.fld0.fld0.fld0.0 = (*_6);
(*_11) = !178530845601296389263742723471455067247_u128;
_15.fld0.fld2 = Adt52 { fld0: _13,fld1: _10 };
_15.fld0.fld2.fld0 = core::ptr::addr_of!((*_13));
_17 = core::ptr::addr_of!(_15.fld0.fld0.fld1);
(*_11) = !70897151717745392437459349275992888038_u128;
_17 = core::ptr::addr_of!((*_17));
Goto(bb7)
}
bb11 = {
_6 = core::ptr::addr_of_mut!((*_6));
_9 = -4342478707736411462_i64;
_15.fld0.fld3 = [951_u16];
_15.fld2.fld0 = (_8,);
RET = [2725771624_u32,3689311015_u32,3450513786_u32,897513155_u32,2017130918_u32,494425002_u32,720683194_u32];
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
43455926409780245612125181517449252937 => bb6,
_ => bb5
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_15.fld2.fld1.fld0 = core::ptr::addr_of!((*_11));
_15.fld1 = -_19.fld0.fld5.0;
_19.fld0.fld0 = [_4,_4,_12,_4,_4,_4,_2,_12];
_19.fld0.fld5 = (_15.fld1, _15.fld0.fld1);
_15.fld2.fld1 = Move(_15.fld0.fld2);
(*_11) = 40821414751478531834248155619000342578_u128;
_19.fld0.fld2 = !_12;
_19.fld0.fld1 = core::ptr::addr_of_mut!(_15.fld0.fld0.fld0.2);
Goto(bb16)
}
bb16 = {
Call(_21 = dump_var(14_usize, 5_usize, Move(_5), 10_usize, Move(_10), 16_usize, Move(_16), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_21 = dump_var(14_usize, 3_usize, Move(_3), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: bool,mut _2: *mut [u16; 5],mut _3: u128,mut _4: (i32, [char; 2]),mut _5: [u32; 7],mut _6: f32,mut _7: (i32, [char; 2])) -> u8 {
mir! {
type RET = u8;
let _8: [i128; 4];
let _9: u64;
let _10: isize;
let _11: u64;
let _12: i8;
let _13: Adt52;
let _14: f64;
let _15: Adt55;
let _16: [i128; 8];
let _17: f32;
let _18: Adt63;
let _19: isize;
let _20: f64;
let _21: [i16; 8];
let _22: ();
let _23: ();
{
_7 = (_4.0, _4.1);
_6 = _4.0 as f32;
_7.0 = 21_i8 as i32;
RET = !94_u8;
_7.0 = _6 as i32;
RET = 62338_u16 as u8;
_3 = 108123247799676306660695193757465868197_u128;
_6 = 42_u8 as f32;
_9 = !6521320816666795163_u64;
_7 = (_4.0, _4.1);
(*_2) = [12406_u16,56420_u16,30439_u16,50580_u16,32790_u16];
_6 = (-142445491151789285_i64) as f32;
(*_2) = [49918_u16,53985_u16,3449_u16,61230_u16,33250_u16];
_5 = [3188571869_u32,1366665845_u32,615530906_u32,1356634455_u32,2371813717_u32,4108644651_u32,2631905644_u32];
_6 = 50295_u16 as f32;
RET = 10940187398964596237_usize as u8;
_4.0 = (-98_i8) as i32;
RET = '\u{bc198}' as u8;
_8 = [(-135730368286035276865898753490659774470_i128),75220267440404125145267515748698843722_i128,56085564040968456267974525234356292014_i128,76995227483688338869233107904742719577_i128];
RET = !21_u8;
_4 = _7;
Call(_9 = fn16(_4.1, _8, _6, _2, (*_2), _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-33833855384473641977594802021188829632_i128) as u8;
_4.1 = ['\u{6ebea}','\u{62e72}'];
RET = 41_u8;
_6 = 4061739537_u32 as f32;
_9 = !11016603115893325947_u64;
(*_2) = [49904_u16,3183_u16,12761_u16,15446_u16,28870_u16];
_1 = !true;
Goto(bb2)
}
bb2 = {
_10 = _1 as isize;
_7.1 = ['\u{916d8}','\u{397df}'];
_10 = 56061_u16 as isize;
_7 = _4;
_5 = [192308021_u32,767947799_u32,2673965643_u32,2443879387_u32,3943653401_u32,1793957688_u32,687570975_u32];
_4.1 = _7.1;
_11 = _4.0 as u64;
match _3 {
0 => bb3,
1 => bb4,
108123247799676306660695193757465868197 => bb6,
_ => bb5
}
}
bb3 = {
RET = (-33833855384473641977594802021188829632_i128) as u8;
_4.1 = ['\u{6ebea}','\u{62e72}'];
RET = 41_u8;
_6 = 4061739537_u32 as f32;
_9 = !11016603115893325947_u64;
(*_2) = [49904_u16,3183_u16,12761_u16,15446_u16,28870_u16];
_1 = !true;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_7.1 = ['\u{68a8a}','\u{c0a86}'];
_1 = _11 >= _11;
_12 = (-47_i8);
_15.fld0.fld1.fld1 = -(-65871994191738581446673192693182283283_i128);
_15.fld0.fld4 = !6972_i16;
_15.fld1 = 7_usize << _11;
_15.fld5.fld1.fld0 = core::ptr::addr_of!(_15.fld2);
_5 = [2210492011_u32,161706146_u32,2579916406_u32,2100224362_u32,586671458_u32,72328904_u32,3902065969_u32];
match _12 {
0 => bb1,
1 => bb7,
2 => bb8,
340282366920938463463374607431768211409 => bb10,
_ => bb9
}
}
bb7 = {
_10 = _1 as isize;
_7.1 = ['\u{916d8}','\u{397df}'];
_10 = 56061_u16 as isize;
_7 = _4;
_5 = [192308021_u32,767947799_u32,2673965643_u32,2443879387_u32,3943653401_u32,1793957688_u32,687570975_u32];
_4.1 = _7.1;
_11 = _4.0 as u64;
match _3 {
0 => bb3,
1 => bb4,
108123247799676306660695193757465868197 => bb6,
_ => bb5
}
}
bb8 = {
RET = (-33833855384473641977594802021188829632_i128) as u8;
_4.1 = ['\u{6ebea}','\u{62e72}'];
RET = 41_u8;
_6 = 4061739537_u32 as f32;
_9 = !11016603115893325947_u64;
(*_2) = [49904_u16,3183_u16,12761_u16,15446_u16,28870_u16];
_1 = !true;
Goto(bb2)
}
bb9 = {
RET = (-33833855384473641977594802021188829632_i128) as u8;
_4.1 = ['\u{6ebea}','\u{62e72}'];
RET = 41_u8;
_6 = 4061739537_u32 as f32;
_9 = !11016603115893325947_u64;
(*_2) = [49904_u16,3183_u16,12761_u16,15446_u16,28870_u16];
_1 = !true;
Goto(bb2)
}
bb10 = {
_13.fld0 = core::ptr::addr_of!(_18.fld4.fld4.fld6.2);
_7.1 = _4.1;
_18.fld6.0 = [_1,_1];
_15.fld5.fld3.4 = '\u{5f016}';
_18.fld2.fld3 = 186_u8 ^ 79_u8;
_18.fld4.fld4.fld4.1 = 2855568249_u32 as i16;
_15.fld7 = (_15.fld0.fld4, _15.fld0.fld1.fld1);
match _12 {
0 => bb4,
1 => bb3,
340282366920938463463374607431768211409 => bb12,
_ => bb11
}
}
bb11 = {
RET = (-33833855384473641977594802021188829632_i128) as u8;
_4.1 = ['\u{6ebea}','\u{62e72}'];
RET = 41_u8;
_6 = 4061739537_u32 as f32;
_9 = !11016603115893325947_u64;
(*_2) = [49904_u16,3183_u16,12761_u16,15446_u16,28870_u16];
_1 = !true;
Goto(bb2)
}
bb12 = {
_18.fld4.fld4.fld6.0.1 = _15.fld0.fld1.fld1;
Goto(bb13)
}
bb13 = {
_15.fld5.fld3.1 = _15.fld1 * _15.fld1;
_15.fld5.fld2 = [_15.fld5.fld3.4,_15.fld5.fld3.4,_15.fld5.fld3.4,_15.fld5.fld3.4,_15.fld5.fld3.4];
_15.fld5.fld5 = _3 as i32;
_18.fld4.fld4.fld5 = (_15.fld5.fld5, _7.1);
_14 = _12 as f64;
match _12 {
340282366920938463463374607431768211409 => bb14,
_ => bb6
}
}
bb14 = {
_18.fld4.fld4.fld4 = (_1, _15.fld0.fld4);
_15.fld5.fld4.0 = _15.fld1 ^ _15.fld1;
_18.fld2.fld0.0 = [13216_u16,56373_u16,65439_u16,58747_u16,20057_u16];
_15.fld0.fld6 = _8;
_18.fld1 = [_10,_10,_10,_10,_10,_10,_10,_10];
RET = _15.fld7.0 as u8;
_15.fld7 = (_18.fld4.fld4.fld4.1, _15.fld0.fld1.fld1);
_18.fld2.fld0.1.1 = _4.0 as i16;
_18.fld4.fld4.fld6.4 = _15.fld5.fld3.4;
_7.0 = !_4.0;
_15.fld2 = _15.fld5.fld3.4 as u128;
RET = !_18.fld2.fld3;
_18.fld4.fld4.fld4.1 = _18.fld2.fld0.1.1 << _15.fld5.fld3.1;
_18.fld2.fld0.3 = [_18.fld4.fld4.fld6.0.1,_15.fld7.1,_15.fld0.fld1.fld1,_15.fld0.fld1.fld1,_15.fld0.fld1.fld1,_18.fld4.fld4.fld6.0.1,_15.fld0.fld1.fld1,_18.fld4.fld4.fld6.0.1];
_18.fld4.fld1.0 = _18.fld4.fld4.fld4.1 as usize;
_15.fld5.fld3.1 = _18.fld2.fld0.1.1 as usize;
_18.fld2.fld0.1.0 = !_18.fld4.fld4.fld4.0;
_17 = _15.fld2 as f32;
(*_2) = _18.fld2.fld0.0;
_13.fld1 = -_15.fld0.fld1.fld1;
_15.fld0.fld1.fld1 = _13.fld1;
_18.fld2.fld0.4 = [3592331896_u32,379953171_u32,2980428324_u32,1531481497_u32,3103591430_u32,201984791_u32,2617750398_u32];
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(15_usize, 12_usize, Move(_12), 10_usize, Move(_10), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(15_usize, 9_usize, Move(_9), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [char; 2],mut _2: [i128; 4],mut _3: f32,mut _4: *mut [u16; 5],mut _5: [u16; 5],mut _6: [i128; 4]) -> u64 {
mir! {
type RET = u64;
let _7: isize;
let _8: i128;
let _9: [u8; 8];
let _10: f32;
let _11: usize;
let _12: usize;
let _13: (bool, i128, *const i64, (usize,));
let _14: usize;
let _15: (bool, i128, *const i64, (usize,));
let _16: Adt62;
let _17: char;
let _18: Adt56;
let _19: (bool, i128, *const i64, (usize,));
let _20: char;
let _21: u64;
let _22: f32;
let _23: bool;
let _24: f32;
let _25: u64;
let _26: char;
let _27: Adt53;
let _28: [i16; 8];
let _29: Adt49;
let _30: Adt60;
let _31: isize;
let _32: f32;
let _33: u32;
let _34: [i128; 8];
let _35: [u16; 1];
let _36: [i128; 8];
let _37: (bool, i16);
let _38: i32;
let _39: *const [char; 5];
let _40: ();
let _41: ();
{
RET = 6049938016064399301_u64;
_5 = [19224_u16,1051_u16,3118_u16,24875_u16,9846_u16];
_6 = [158998110064655300877227958452818533927_i128,149941777418002723695964270406012947202_i128,133743612949635421066749599973527628980_i128,156508735565284749979945540942895620337_i128];
_6 = [97387490606583808872602219718487973381_i128,42343225681857892177668781625550513058_i128,(-152592575973405657206613062453133825079_i128),117829985973700401443446169000148524061_i128];
_8 = '\u{e88e5}' as i128;
RET = 11252323029904359354_u64;
_1 = ['\u{10a2a1}','\u{dcd33}'];
_1 = ['\u{72a14}','\u{5abeb}'];
RET = !16441717449261795499_u64;
_7 = 50_isize;
(*_4) = [54945_u16,11982_u16,16487_u16,5653_u16,5573_u16];
_1 = ['\u{d5077}','\u{f1192}'];
_10 = (-10648_i16) as f32;
_4 = core::ptr::addr_of_mut!((*_4));
_3 = _10;
_6 = [_8,_8,_8,_8];
_8 = 99852282715077666794956671786821628286_i128;
_10 = -_3;
Goto(bb1)
}
bb1 = {
RET = '\u{bdc93}' as u64;
_6 = _2;
_5 = (*_4);
_12 = 14356569702519462025_u64 as usize;
_13.3 = (_12,);
_13.0 = false;
_2 = [_8,_8,_8,_8];
_7 = 9223372036854775807_isize;
_13.1 = _8;
(*_4) = [14445_u16,47021_u16,42659_u16,55571_u16,33559_u16];
_15.0 = _13.0;
_13.3 = (_12,);
_15.3 = (_12,);
(*_4) = [38797_u16,23612_u16,45807_u16,48822_u16,34376_u16];
_6 = _2;
_12 = _15.3.0;
_15.0 = _8 > _13.1;
_16.fld0.fld0.fld3 = 173_u8 + 231_u8;
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_16.fld0.fld0.fld1 = _10 - _10;
_16.fld0.fld0.fld0.1 = (_15.0, (-17885_i16));
_17 = '\u{f5274}';
_16.fld0.fld0.fld0.3 = [_8,_13.1,_8,_13.1,_8,_8,_13.1,_8];
_16.fld2.fld1.fld1 = _13.1;
_14 = _15.3.0;
_16.fld0.fld4 = _6;
_16.fld0.fld4 = _6;
_15.1 = _8 - _8;
_3 = _16.fld0.fld0.fld1 - _16.fld0.fld0.fld1;
_16.fld0.fld4 = _2;
_16.fld0.fld0.fld2 = !3586269284_u32;
_18.fld7.fld2 = 8918144418217352445_i64 as u128;
_18.fld3.2 = !_18.fld7.fld2;
(*_4) = [56544_u16,15450_u16,51170_u16,19550_u16,61519_u16];
_18.fld5 = !_18.fld7.fld2;
_18.fld3.0.0 = _16.fld0.fld0.fld0.1.1 * _16.fld0.fld0.fld0.1.1;
_18.fld7.fld7.1 = _8;
_16.fld2.fld0.0 = _14 + _12;
_18.fld7.fld0.fld5 = [_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3];
_16.fld0.fld4 = _2;
_18.fld7.fld6 = [_7,_7,_7,_7,_7,_7,_7,_7];
_16.fld0.fld0.fld0.0 = [63348_u16,25389_u16,51313_u16,30642_u16,42489_u16];
Call(_7 = fn17(_16.fld0.fld0.fld0.1, _18.fld3.2, _13.0, _16.fld0.fld0.fld0.1.1, _16.fld0.fld0.fld0.3, (*_4)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_18.fld7.fld7.0 = _16.fld2.fld1.fld1 as i16;
_16.fld0.fld1 = [_17,_17];
_18.fld7.fld0.fld1.fld1 = -_15.1;
_18.fld7.fld5.fld5 = !369123416_i32;
_18.fld7.fld0.fld6 = _16.fld0.fld4;
_18.fld0 = _7 as u8;
_18.fld7.fld5.fld3 = (_18.fld7.fld7, _16.fld2.fld0.0, _18.fld7.fld2, _18.fld7.fld0.fld1.fld1, _17);
_18.fld7.fld0.fld2 = [_18.fld7.fld5.fld3.0.0,_18.fld3.0.0,_16.fld0.fld0.fld0.1.1,_18.fld3.0.0,_18.fld3.0.0,_16.fld0.fld0.fld0.1.1,_18.fld3.0.0,_18.fld3.0.0];
RET = 3408372752041765461_u64 & 6221312308642763472_u64;
_18.fld7.fld5.fld1.fld0 = core::ptr::addr_of!(_18.fld7.fld5.fld3.2);
_18.fld3.4 = _17;
_20 = _18.fld3.4;
_18.fld7.fld5.fld3.0.0 = _16.fld0.fld0.fld0.1.1;
_18.fld7.fld0.fld4 = 8826595837597952662_u64 as i16;
_15.1 = _18.fld7.fld7.1;
_17 = _18.fld3.4;
_18.fld4 = 18302419737318452489_u64 ^ 14531127670129585936_u64;
Goto(bb8)
}
bb8 = {
_16.fld0.fld2.fld1 = _13.1;
_16.fld2.fld0.0 = !_18.fld7.fld5.fld3.1;
_9 = [_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_18.fld0];
_15.3 = (_12,);
_18.fld7.fld3 = 111_i8 - (-123_i8);
_18.fld7.fld1 = _18.fld7.fld5.fld3.1;
_16.fld0.fld0.fld0.1.1 = !_18.fld3.0.0;
_18.fld7.fld0.fld2 = [_18.fld3.0.0,_18.fld3.0.0,_16.fld0.fld0.fld0.1.1,_16.fld0.fld0.fld0.1.1,_16.fld0.fld0.fld0.1.1,_18.fld3.0.0,_18.fld3.0.0,_18.fld7.fld0.fld4];
_19.3 = (_16.fld2.fld0.0,);
_18.fld6 = _16.fld0.fld0.fld2;
_12 = _18.fld7.fld1;
_16.fld2.fld1 = Adt52 { fld0: _18.fld7.fld5.fld1.fld0,fld1: _16.fld0.fld2.fld1 };
_21 = _18.fld7.fld5.fld5 as u64;
_18.fld7.fld5.fld3.2 = _18.fld3.2 * _18.fld7.fld2;
_18.fld4 = _21 & _21;
_16.fld0.fld0.fld0.0 = [21855_u16,784_u16,28571_u16,24133_u16,64558_u16];
_16.fld0.fld0.fld0.1.1 = -_18.fld3.0.0;
_18.fld7.fld5.fld3 = (_18.fld7.fld7, _12, _18.fld7.fld2, _15.1, _20);
_8 = -_18.fld7.fld0.fld1.fld1;
_1 = [_18.fld3.4,_17];
_18.fld7.fld5.fld1.fld1 = _15.0 as i128;
_11 = _16.fld2.fld0.0;
_18.fld7.fld5.fld0 = [_16.fld0.fld0.fld2,_18.fld6,_18.fld6,_18.fld6,_18.fld6,_16.fld0.fld0.fld2,_18.fld6];
_19.0 = _3 < _16.fld0.fld0.fld1;
match _18.fld7.fld5.fld3.0.1 {
99852282715077666794956671786821628286 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_24 = _16.fld0.fld0.fld1 / 1_f32;
_18.fld3.3 = _16.fld0.fld0.fld1 as i128;
_18.fld7.fld2 = _7 as u128;
_27.fld5 = [_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3];
_18.fld7.fld0.fld1.fld0 = core::ptr::addr_of!(_18.fld3.2);
_18.fld7.fld5.fld3.0.0 = _11 as i16;
_18.fld7.fld5.fld3.1 = _16.fld2.fld0.0;
_27.fld1.fld0 = core::ptr::addr_of!(_18.fld7.fld2);
_8 = _18.fld3.3;
_18.fld7.fld5.fld1.fld0 = core::ptr::addr_of!(_18.fld7.fld2);
_23 = _16.fld0.fld0.fld0.1.1 == _18.fld3.0.0;
_18.fld0 = _24 as u8;
_29.fld0.fld6 = (_18.fld7.fld5.fld3.0, _18.fld7.fld5.fld3.1, _18.fld7.fld5.fld3.2, _18.fld7.fld5.fld1.fld1, _17);
_6 = [_18.fld7.fld0.fld1.fld1,_8,_15.1,_18.fld7.fld7.1];
_11 = !_15.3.0;
_16.fld0.fld4 = [_8,_29.fld0.fld6.3,_8,_15.1];
_29.fld0.fld4.0 = _23;
_15.0 = !_29.fld0.fld4.0;
_29.fld0.fld4 = _16.fld0.fld0.fld0.1;
_15.0 = _29.fld0.fld4.0;
_16.fld0.fld0.fld0.1.0 = _23;
RET = !_18.fld4;
_16.fld0.fld0.fld0.4 = [_18.fld6,_18.fld6,_16.fld0.fld0.fld2,_18.fld6,_18.fld6,_16.fld0.fld0.fld2,_18.fld6];
_18.fld7.fld5.fld3.4 = _17;
_30.fld0.0 = _19.0;
match _29.fld0.fld6.0.1 {
0 => bb8,
1 => bb7,
2 => bb6,
3 => bb4,
99852282715077666794956671786821628286 => bb12,
_ => bb11
}
}
bb11 = {
_16.fld0.fld2.fld1 = _13.1;
_16.fld2.fld0.0 = !_18.fld7.fld5.fld3.1;
_9 = [_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_18.fld0];
_15.3 = (_12,);
_18.fld7.fld3 = 111_i8 - (-123_i8);
_18.fld7.fld1 = _18.fld7.fld5.fld3.1;
_16.fld0.fld0.fld0.1.1 = !_18.fld3.0.0;
_18.fld7.fld0.fld2 = [_18.fld3.0.0,_18.fld3.0.0,_16.fld0.fld0.fld0.1.1,_16.fld0.fld0.fld0.1.1,_16.fld0.fld0.fld0.1.1,_18.fld3.0.0,_18.fld3.0.0,_18.fld7.fld0.fld4];
_19.3 = (_16.fld2.fld0.0,);
_18.fld6 = _16.fld0.fld0.fld2;
_12 = _18.fld7.fld1;
_16.fld2.fld1 = Adt52 { fld0: _18.fld7.fld5.fld1.fld0,fld1: _16.fld0.fld2.fld1 };
_21 = _18.fld7.fld5.fld5 as u64;
_18.fld7.fld5.fld3.2 = _18.fld3.2 * _18.fld7.fld2;
_18.fld4 = _21 & _21;
_16.fld0.fld0.fld0.0 = [21855_u16,784_u16,28571_u16,24133_u16,64558_u16];
_16.fld0.fld0.fld0.1.1 = -_18.fld3.0.0;
_18.fld7.fld5.fld3 = (_18.fld7.fld7, _12, _18.fld7.fld2, _15.1, _20);
_8 = -_18.fld7.fld0.fld1.fld1;
_1 = [_18.fld3.4,_17];
_18.fld7.fld5.fld1.fld1 = _15.0 as i128;
_11 = _16.fld2.fld0.0;
_18.fld7.fld5.fld0 = [_16.fld0.fld0.fld2,_18.fld6,_18.fld6,_18.fld6,_18.fld6,_16.fld0.fld0.fld2,_18.fld6];
_19.0 = _3 < _16.fld0.fld0.fld1;
match _18.fld7.fld5.fld3.0.1 {
99852282715077666794956671786821628286 => bb10,
_ => bb9
}
}
bb12 = {
_18.fld7.fld7 = (_18.fld7.fld0.fld4, _8);
_30.fld4.fld4.0 = !_23;
_19.1 = _18.fld6 as i128;
_18.fld7.fld5.fld0 = [_18.fld6,_18.fld6,_18.fld6,_16.fld0.fld0.fld2,_18.fld6,_18.fld6,_16.fld0.fld0.fld2];
_18.fld7.fld0.fld0 = _16.fld0.fld0.fld0.1.0;
_18.fld3.0 = (_16.fld0.fld0.fld0.1.1, _8);
_16.fld2 = Adt61 { fld0: _15.3,fld1: Move(_18.fld7.fld5.fld1) };
_16.fld0.fld3 = [58061_u16];
_30.fld1.0 = _29.fld0.fld6.1;
Goto(bb13)
}
bb13 = {
_30.fld0.1 = _16.fld0.fld0.fld0.1.1;
_29.fld0.fld6.1 = _30.fld1.0;
_29.fld0.fld4.0 = _18.fld7.fld0.fld0;
_30.fld4.fld1 = core::ptr::addr_of_mut!(_16.fld0.fld0.fld0.2);
_18.fld3.0 = (_30.fld0.1, _15.1);
_29.fld0.fld5.0 = _29.fld0.fld6.4 as i32;
_20 = _17;
_29.fld0.fld2 = _17 as isize;
_27.fld1 = Adt52 { fld0: _18.fld7.fld0.fld1.fld0,fld1: _18.fld7.fld7.1 };
_8 = !_18.fld3.3;
_16.fld2.fld0 = (_30.fld1.0,);
_16.fld0.fld0.fld0.2 = [16684_u16,9437_u16,13654_u16,54513_u16,26022_u16];
_30.fld4.fld6 = _29.fld0.fld6;
_33 = !_16.fld0.fld0.fld2;
_13.1 = _30.fld4.fld6.3;
_18.fld7.fld0.fld1.fld0 = _27.fld1.fld0;
_30.fld4.fld6.0.0 = !_18.fld3.0.0;
_28 = _18.fld7.fld0.fld2;
_18.fld0 = _16.fld0.fld0.fld3 | _16.fld0.fld0.fld3;
_18.fld7.fld5.fld0 = [_16.fld0.fld0.fld2,_18.fld6,_33,_16.fld0.fld0.fld2,_33,_33,_33];
_30.fld4.fld1 = core::ptr::addr_of_mut!(_5);
_6 = [_30.fld4.fld6.0.1,_18.fld7.fld5.fld3.0.1,_18.fld3.3,_18.fld3.3];
_29.fld0.fld5 = (_18.fld7.fld5.fld5, _16.fld0.fld1);
match _16.fld0.fld2.fld1 {
0 => bb6,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
99852282715077666794956671786821628286 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
_16.fld0.fld2.fld1 = _13.1;
_16.fld2.fld0.0 = !_18.fld7.fld5.fld3.1;
_9 = [_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_18.fld0];
_15.3 = (_12,);
_18.fld7.fld3 = 111_i8 - (-123_i8);
_18.fld7.fld1 = _18.fld7.fld5.fld3.1;
_16.fld0.fld0.fld0.1.1 = !_18.fld3.0.0;
_18.fld7.fld0.fld2 = [_18.fld3.0.0,_18.fld3.0.0,_16.fld0.fld0.fld0.1.1,_16.fld0.fld0.fld0.1.1,_16.fld0.fld0.fld0.1.1,_18.fld3.0.0,_18.fld3.0.0,_18.fld7.fld0.fld4];
_19.3 = (_16.fld2.fld0.0,);
_18.fld6 = _16.fld0.fld0.fld2;
_12 = _18.fld7.fld1;
_16.fld2.fld1 = Adt52 { fld0: _18.fld7.fld5.fld1.fld0,fld1: _16.fld0.fld2.fld1 };
_21 = _18.fld7.fld5.fld5 as u64;
_18.fld7.fld5.fld3.2 = _18.fld3.2 * _18.fld7.fld2;
_18.fld4 = _21 & _21;
_16.fld0.fld0.fld0.0 = [21855_u16,784_u16,28571_u16,24133_u16,64558_u16];
_16.fld0.fld0.fld0.1.1 = -_18.fld3.0.0;
_18.fld7.fld5.fld3 = (_18.fld7.fld7, _12, _18.fld7.fld2, _15.1, _20);
_8 = -_18.fld7.fld0.fld1.fld1;
_1 = [_18.fld3.4,_17];
_18.fld7.fld5.fld1.fld1 = _15.0 as i128;
_11 = _16.fld2.fld0.0;
_18.fld7.fld5.fld0 = [_16.fld0.fld0.fld2,_18.fld6,_18.fld6,_18.fld6,_18.fld6,_16.fld0.fld0.fld2,_18.fld6];
_19.0 = _3 < _16.fld0.fld0.fld1;
match _18.fld7.fld5.fld3.0.1 {
99852282715077666794956671786821628286 => bb10,
_ => bb9
}
}
bb16 = {
_16.fld0.fld0.fld1 = _10 - _10;
_16.fld0.fld0.fld0.1 = (_15.0, (-17885_i16));
_17 = '\u{f5274}';
_16.fld0.fld0.fld0.3 = [_8,_13.1,_8,_13.1,_8,_8,_13.1,_8];
_16.fld2.fld1.fld1 = _13.1;
_14 = _15.3.0;
_16.fld0.fld4 = _6;
_16.fld0.fld4 = _6;
_15.1 = _8 - _8;
_3 = _16.fld0.fld0.fld1 - _16.fld0.fld0.fld1;
_16.fld0.fld4 = _2;
_16.fld0.fld0.fld2 = !3586269284_u32;
_18.fld7.fld2 = 8918144418217352445_i64 as u128;
_18.fld3.2 = !_18.fld7.fld2;
(*_4) = [56544_u16,15450_u16,51170_u16,19550_u16,61519_u16];
_18.fld5 = !_18.fld7.fld2;
_18.fld3.0.0 = _16.fld0.fld0.fld0.1.1 * _16.fld0.fld0.fld0.1.1;
_18.fld7.fld7.1 = _8;
_16.fld2.fld0.0 = _14 + _12;
_18.fld7.fld0.fld5 = [_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3];
_16.fld0.fld4 = _2;
_18.fld7.fld6 = [_7,_7,_7,_7,_7,_7,_7,_7];
_16.fld0.fld0.fld0.0 = [63348_u16,25389_u16,51313_u16,30642_u16,42489_u16];
Call(_7 = fn17(_16.fld0.fld0.fld0.1, _18.fld3.2, _13.0, _16.fld0.fld0.fld0.1.1, _16.fld0.fld0.fld0.3, (*_4)), ReturnTo(bb7), UnwindUnreachable())
}
bb17 = {
Return()
}
bb18 = {
_16.fld0.fld2.fld1 = _13.1;
_16.fld2.fld0.0 = !_18.fld7.fld5.fld3.1;
_9 = [_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_16.fld0.fld0.fld3,_18.fld0,_16.fld0.fld0.fld3,_18.fld0];
_15.3 = (_12,);
_18.fld7.fld3 = 111_i8 - (-123_i8);
_18.fld7.fld1 = _18.fld7.fld5.fld3.1;
_16.fld0.fld0.fld0.1.1 = !_18.fld3.0.0;
_18.fld7.fld0.fld2 = [_18.fld3.0.0,_18.fld3.0.0,_16.fld0.fld0.fld0.1.1,_16.fld0.fld0.fld0.1.1,_16.fld0.fld0.fld0.1.1,_18.fld3.0.0,_18.fld3.0.0,_18.fld7.fld0.fld4];
_19.3 = (_16.fld2.fld0.0,);
_18.fld6 = _16.fld0.fld0.fld2;
_12 = _18.fld7.fld1;
_16.fld2.fld1 = Adt52 { fld0: _18.fld7.fld5.fld1.fld0,fld1: _16.fld0.fld2.fld1 };
_21 = _18.fld7.fld5.fld5 as u64;
_18.fld7.fld5.fld3.2 = _18.fld3.2 * _18.fld7.fld2;
_18.fld4 = _21 & _21;
_16.fld0.fld0.fld0.0 = [21855_u16,784_u16,28571_u16,24133_u16,64558_u16];
_16.fld0.fld0.fld0.1.1 = -_18.fld3.0.0;
_18.fld7.fld5.fld3 = (_18.fld7.fld7, _12, _18.fld7.fld2, _15.1, _20);
_8 = -_18.fld7.fld0.fld1.fld1;
_1 = [_18.fld3.4,_17];
_18.fld7.fld5.fld1.fld1 = _15.0 as i128;
_11 = _16.fld2.fld0.0;
_18.fld7.fld5.fld0 = [_16.fld0.fld0.fld2,_18.fld6,_18.fld6,_18.fld6,_18.fld6,_16.fld0.fld0.fld2,_18.fld6];
_19.0 = _3 < _16.fld0.fld0.fld1;
match _18.fld7.fld5.fld3.0.1 {
99852282715077666794956671786821628286 => bb10,
_ => bb9
}
}
bb19 = {
_37.1 = -_30.fld4.fld6.0.0;
_27.fld5 = _18.fld7.fld0.fld5;
_18.fld7.fld5.fld4 = _30.fld1;
_30.fld4.fld6.4 = _18.fld7.fld5.fld3.4;
_16.fld0.fld3 = [52315_u16];
_30.fld4.fld4.0 = _23;
Goto(bb20)
}
bb20 = {
Call(_40 = dump_var(16_usize, 20_usize, Move(_20), 8_usize, Move(_8), 5_usize, Move(_5), 28_usize, Move(_28)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_40 = dump_var(16_usize, 6_usize, Move(_6), 2_usize, Move(_2), 33_usize, Move(_33), 21_usize, Move(_21)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: (bool, i16),mut _2: u128,mut _3: bool,mut _4: i16,mut _5: [i128; 8],mut _6: [u16; 5]) -> isize {
mir! {
type RET = isize;
let _7: *const u16;
let _8: (bool, i128, *const i64, (usize,));
let _9: Adt49;
let _10: [char; 5];
let _11: Adt64;
let _12: u16;
let _13: u16;
let _14: u8;
let _15: (bool, i16);
let _16: (i32, [char; 2]);
let _17: Adt53;
let _18: isize;
let _19: Adt53;
let _20: [u8; 8];
let _21: bool;
let _22: bool;
let _23: [char; 2];
let _24: [usize; 2];
let _25: [char; 5];
let _26: char;
let _27: [bool; 1];
let _28: isize;
let _29: [i128; 4];
let _30: ((i16, i128), usize, u128, i128, char);
let _31: f32;
let _32: [isize; 5];
let _33: i16;
let _34: *const f32;
let _35: (usize,);
let _36: bool;
let _37: *const [char; 5];
let _38: ();
let _39: ();
{
_8.3 = (7_usize,);
_8.1 = -40260612583595532424564785781354707561_i128;
_4 = !_1.1;
_2 = !147362620506307970327409190804245227641_u128;
_8.0 = _3;
_1 = (_3, _4);
RET = 9223372036854775807_isize;
_9.fld0.fld0 = [9223372036854775807_isize,(-14_isize),52_isize,9223372036854775807_isize,(-18_isize),9223372036854775807_isize,(-125_isize),9223372036854775807_isize];
_8.1 = !2234757582590218845652287475145887974_i128;
_9.fld0.fld6.1 = _8.3.0 * _8.3.0;
_1.1 = _4;
_8.3.0 = 581518894_i32 as usize;
_9.fld0.fld6.0 = (_1.1, _8.1);
_3 = !_8.0;
_9.fld0.fld6.1 = _8.3.0 | _8.3.0;
_11.fld1.fld3.0.1 = !_9.fld0.fld6.0.1;
_1.0 = _3 <= _8.0;
_11.fld1.fld3.1 = !_9.fld0.fld6.1;
_9.fld0.fld3 = [_3];
Goto(bb1)
}
bb1 = {
_9.fld0.fld6.0 = (_4, _11.fld1.fld3.0.1);
_11.fld1.fld3 = (_9.fld0.fld6.0, _8.3.0, _2, _9.fld0.fld6.0.1, '\u{85992}');
_11.fld0 = _9.fld0.fld6.0.1 + _11.fld1.fld3.0.1;
_9.fld0.fld6.4 = _11.fld1.fld3.4;
_9.fld0.fld6.4 = _11.fld1.fld3.4;
_9.fld0.fld6.3 = !_11.fld1.fld3.0.1;
RET = -(-96_isize);
_11.fld1.fld1.fld0 = core::ptr::addr_of!(_9.fld0.fld6.2);
_9.fld1 = [23_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_11.fld1.fld5 = (-219132645_i32) + 1487835148_i32;
_11.fld1.fld1.fld1 = (-61_i8) as i128;
_11.fld1.fld3.0 = (_4, _8.1);
_9.fld0.fld6.0 = (_11.fld1.fld3.0.0, _8.1);
_11.fld1.fld4.0 = 248_u8 as usize;
_11.fld1.fld0 = [2927768486_u32,1879296274_u32,4207958403_u32,3689125443_u32,2263736103_u32,2892629676_u32,3636625787_u32];
_9.fld0.fld5.1 = [_11.fld1.fld3.4,_11.fld1.fld3.4];
_12 = !49847_u16;
RET = !9223372036854775807_isize;
_9.fld0.fld5.0 = _11.fld1.fld5;
_7 = core::ptr::addr_of!(_13);
_9.fld0.fld4.1 = !_4;
_11.fld1.fld3.3 = _9.fld0.fld6.3;
Goto(bb2)
}
bb2 = {
_11.fld1.fld4.0 = _11.fld1.fld3.1 >> _11.fld1.fld3.0.0;
_13 = !_12;
_7 = core::ptr::addr_of!(_13);
_9.fld0.fld4.1 = _11.fld1.fld3.4 as i16;
_3 = !_8.0;
_11.fld1.fld2 = [_11.fld1.fld3.4,_9.fld0.fld6.4,_11.fld1.fld3.4,_11.fld1.fld3.4,_11.fld1.fld3.4];
_9.fld0.fld0 = [(-9223372036854775808_isize),41_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
Goto(bb3)
}
bb3 = {
_8.3 = (_11.fld1.fld4.0,);
_3 = !_8.0;
_11.fld1.fld3.0 = _9.fld0.fld6.0;
_11.fld1.fld3.0 = _9.fld0.fld6.0;
_3 = _1.0;
Goto(bb4)
}
bb4 = {
_11.fld1.fld3.0.1 = _11.fld0;
_11.fld1.fld3.0 = _9.fld0.fld6.0;
_14 = 136_u8;
Goto(bb5)
}
bb5 = {
_11.fld1.fld3.1 = !_8.3.0;
_11.fld1.fld5 = _9.fld0.fld5.0;
_11.fld2 = [_1.0,_3];
_9.fld1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-38_isize)];
Goto(bb6)
}
bb6 = {
_11.fld1.fld3.0 = (_9.fld0.fld4.1, _11.fld0);
_11.fld1.fld4 = (_9.fld0.fld6.1,);
_15 = (_1.0, _11.fld1.fld3.0.0);
_17.fld5 = [_14,_14,_14,_14,_14,_14,_14,_14];
_17.fld1 = Move(_11.fld1.fld1);
_8.1 = _11.fld1.fld3.0.1;
_18 = 9223372036854775807_isize;
RET = _18;
_4 = _11.fld1.fld3.0.0;
_9.fld0.fld6.2 = _12 as u128;
_9.fld0.fld6.1 = _8.3.0 ^ _11.fld1.fld3.1;
_15.1 = _4;
_11.fld1.fld4 = (_11.fld1.fld3.1,);
_18 = _9.fld0.fld6.2 as isize;
_11.fld2 = [_8.0,_1.0];
_19.fld0 = !_3;
_16.1 = [_9.fld0.fld6.4,_11.fld1.fld3.4];
_17.fld0 = _15.0;
RET = -_18;
Goto(bb7)
}
bb7 = {
_17.fld1.fld1 = _11.fld0 * _11.fld1.fld3.0.1;
_15 = (_3, _4);
_18 = 9223372036854775807_isize;
_8.0 = !_19.fld0;
_15.1 = _11.fld1.fld3.0.0;
_19.fld4 = _14 as i16;
_17.fld6 = [_17.fld1.fld1,_17.fld1.fld1,_11.fld1.fld3.3,_17.fld1.fld1];
match _14 {
0 => bb4,
136 => bb9,
_ => bb8
}
}
bb8 = {
_11.fld1.fld3.0 = (_9.fld0.fld4.1, _11.fld0);
_11.fld1.fld4 = (_9.fld0.fld6.1,);
_15 = (_1.0, _11.fld1.fld3.0.0);
_17.fld5 = [_14,_14,_14,_14,_14,_14,_14,_14];
_17.fld1 = Move(_11.fld1.fld1);
_8.1 = _11.fld1.fld3.0.1;
_18 = 9223372036854775807_isize;
RET = _18;
_4 = _11.fld1.fld3.0.0;
_9.fld0.fld6.2 = _12 as u128;
_9.fld0.fld6.1 = _8.3.0 ^ _11.fld1.fld3.1;
_15.1 = _4;
_11.fld1.fld4 = (_11.fld1.fld3.1,);
_18 = _9.fld0.fld6.2 as isize;
_11.fld2 = [_8.0,_1.0];
_19.fld0 = !_3;
_16.1 = [_9.fld0.fld6.4,_11.fld1.fld3.4];
_17.fld0 = _15.0;
RET = -_18;
Goto(bb7)
}
bb9 = {
_21 = !_1.0;
_22 = _21;
_19.fld0 = _8.0;
_17.fld4 = _1.1;
_19.fld1.fld1 = !_9.fld0.fld6.3;
_15 = (_21, _9.fld0.fld6.0.0);
_11.fld0 = (*_7) as i128;
_11.fld1.fld4.0 = _9.fld0.fld6.1;
_11.fld2 = [_15.0,_1.0];
Goto(bb10)
}
bb10 = {
_23 = _16.1;
_9.fld0.fld2 = -_18;
_16.1 = [_9.fld0.fld6.4,_11.fld1.fld3.4];
_11.fld1.fld3.3 = !_17.fld1.fld1;
_8.3.0 = !_11.fld1.fld4.0;
_8.3 = (_11.fld1.fld3.1,);
_9.fld0.fld6.0.1 = !_17.fld1.fld1;
_19.fld1 = Adt52 { fld0: _17.fld1.fld0,fld1: _9.fld0.fld6.0.1 };
_19.fld5 = _17.fld5;
_12 = (*_7);
_25 = _11.fld1.fld2;
_15 = _1;
_19.fld2 = [_4,_4,_11.fld1.fld3.0.0,_9.fld0.fld4.1,_4,_4,_9.fld0.fld4.1,_4];
_19.fld6 = [_9.fld0.fld6.0.1,_11.fld1.fld3.3,_11.fld1.fld3.3,_8.1];
_2 = _11.fld1.fld3.2;
RET = _11.fld1.fld3.2 as isize;
RET = _18;
_11.fld1.fld4 = (_9.fld0.fld6.1,);
_25 = [_9.fld0.fld6.4,_11.fld1.fld3.4,_11.fld1.fld3.4,_11.fld1.fld3.4,_11.fld1.fld3.4];
_16 = (_9.fld0.fld5.0, _9.fld0.fld5.1);
_11.fld1.fld0 = [3435242877_u32,3610113157_u32,291646687_u32,1637958665_u32,112112724_u32,1245382733_u32,3771834050_u32];
_6 = [(*_7),(*_7),_13,(*_7),_12];
_9.fld0.fld6.0 = (_11.fld1.fld3.0.0, _11.fld1.fld3.3);
_19.fld1.fld0 = core::ptr::addr_of!(_9.fld0.fld6.2);
Goto(bb11)
}
bb11 = {
_28 = _9.fld0.fld2 | _9.fld0.fld2;
_11.fld1.fld3.2 = 793032380_u32 as u128;
_19.fld5 = [_14,_14,_14,_14,_14,_14,_14,_14];
_17.fld2 = _19.fld2;
_17.fld2 = [_9.fld0.fld4.1,_9.fld0.fld6.0.0,_15.1,_9.fld0.fld4.1,_1.1,_4,_11.fld1.fld3.0.0,_4];
_30.0 = (_9.fld0.fld4.1, _9.fld0.fld6.0.1);
_19.fld5 = [_14,_14,_14,_14,_14,_14,_14,_14];
_17.fld1 = Move(_19.fld1);
_2 = !_11.fld1.fld3.2;
_19.fld4 = _9.fld0.fld4.1;
_19.fld5 = [_14,_14,_14,_14,_14,_14,_14,_14];
_9.fld0.fld0 = [_18,_18,_28,_9.fld0.fld2,_28,_28,_18,_28];
_17.fld1.fld0 = core::ptr::addr_of!(_11.fld1.fld3.2);
_19.fld6 = [_30.0.1,_11.fld1.fld3.3,_30.0.1,_17.fld1.fld1];
Goto(bb12)
}
bb12 = {
_9.fld0.fld6.4 = _11.fld1.fld3.4;
RET = _2 as isize;
_27 = [_8.0];
RET = 46_i8 as isize;
_30 = (_9.fld0.fld6.0, _9.fld0.fld6.1, _11.fld1.fld3.2, _11.fld1.fld3.0.1, _11.fld1.fld3.4);
_29 = [_11.fld1.fld3.3,_17.fld1.fld1,_17.fld1.fld1,_11.fld1.fld3.0.1];
_6 = [(*_7),_12,(*_7),(*_7),_12];
_28 = _18;
_13 = 17781921846329978468_u64 as u16;
Goto(bb13)
}
bb13 = {
_28 = _9.fld0.fld2 - _9.fld0.fld2;
_9.fld0.fld5.0 = _28 as i32;
_9.fld0.fld4 = (_19.fld0, _4);
_9.fld0.fld6.4 = _11.fld1.fld3.4;
_9.fld0.fld0 = [_28,_28,_28,_18,_9.fld0.fld2,_9.fld0.fld2,_28,_28];
_35 = (_30.1,);
_9.fld0.fld4.1 = _19.fld4 >> _12;
_15.1 = _1.1;
_9.fld0.fld6.0.0 = _11.fld1.fld3.0.0 & _1.1;
_9.fld0.fld5.1 = [_30.4,_9.fld0.fld6.4];
_9.fld0.fld1 = core::ptr::addr_of_mut!(_6);
_7 = core::ptr::addr_of!((*_7));
Goto(bb14)
}
bb14 = {
_24 = [_9.fld0.fld6.1,_35.0];
_8.1 = _16.0 as i128;
_11.fld0 = -_9.fld0.fld6.0.1;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(17_usize, 27_usize, Move(_27), 18_usize, Move(_18), 12_usize, Move(_12), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(17_usize, 6_usize, Move(_6), 30_usize, Move(_30), 2_usize, Move(_2), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(17_usize, 29_usize, Move(_29), 5_usize, Move(_5), 23_usize, Move(_23), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: *const f32,mut _2: [isize; 8],mut _3: f32,mut _4: (i16, i128),mut _5: char,mut _6: ([bool; 2],),mut _7: i64,mut _8: [bool; 2],mut _9: [isize; 8],mut _10: usize,mut _11: char,mut _12: isize) -> i128 {
mir! {
type RET = i128;
let _13: isize;
let _14: Adt56;
let _15: Adt49;
let _16: Adt48;
let _17: (usize,);
let _18: f32;
let _19: (bool, i128, *const i64, (usize,));
let _20: i64;
let _21: (bool, i16);
let _22: *const [char; 5];
let _23: u16;
let _24: char;
let _25: i128;
let _26: ([u16; 5], (bool, i16), [u16; 5], [i128; 8], [u32; 7]);
let _27: (i16, i128);
let _28: *const u128;
let _29: [i16; 8];
let _30: isize;
let _31: (usize,);
let _32: [u16; 5];
let _33: [char; 5];
let _34: (bool, i16);
let _35: [u16; 4];
let _36: [u8; 8];
let _37: isize;
let _38: [bool; 2];
let _39: f32;
let _40: [usize; 2];
let _41: Adt61;
let _42: u64;
let _43: ([u16; 5], (bool, i16), [u16; 5], [i128; 8], [u32; 7]);
let _44: f64;
let _45: i64;
let _46: Adt50;
let _47: Adt63;
let _48: ();
let _49: ();
{
_9 = [_12,_12,_12,_12,_12,_12,_12,_12];
_10 = 16973179330827608100_usize | 4_usize;
_9 = [_12,_12,_12,_12,_12,_12,_12,_12];
RET = 50523_u16 as i128;
_6.0 = [true,false];
_12 = _10 as isize;
_4.0 = 14442_i16 * 30833_i16;
_2 = [_12,_12,_12,_12,_12,_12,_12,_12];
(*_1) = -_3;
_7 = !6131891043107758182_i64;
_2 = [_12,_12,_12,_12,_12,_12,_12,_12];
_6.0 = [true,true];
_8 = [false,true];
_9 = _2;
_2 = [_12,_12,_12,_12,_12,_12,_12,_12];
_3 = (*_1);
_6.0 = _8;
RET = _4.1 << _4.0;
Goto(bb1)
}
bb1 = {
_14.fld7.fld2 = !15476721953360643372053795231667544167_u128;
_14.fld6 = (-78_i8) as u32;
_14.fld3.0.0 = 77_u8 as i16;
_14.fld7.fld5.fld3.4 = _5;
_14.fld7.fld5.fld3.0 = (_4.0, _4.1);
_9 = [_12,_12,_12,_12,_12,_12,_12,_12];
_14.fld7.fld5.fld3.2 = _14.fld7.fld2;
_14.fld7.fld7 = _14.fld7.fld5.fld3.0;
_16.fld0 = _2;
_16.fld2 = 11_u8 as isize;
_6.0 = [false,false];
_14.fld7.fld5.fld1.fld0 = core::ptr::addr_of!(_14.fld7.fld5.fld3.2);
Goto(bb2)
}
bb2 = {
_14.fld7.fld6 = [_12,_16.fld2,_12,_12,_12,_12,_12,_16.fld2];
_15.fld0.fld6.1 = !_10;
_14.fld3.0.0 = -_14.fld7.fld5.fld3.0.0;
_14.fld7.fld7 = (_4.0, _14.fld7.fld5.fld3.0.1);
_14.fld3.3 = _4.1 * _14.fld7.fld7.1;
_14.fld7.fld4 = [_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6];
(*_1) = _3;
_14.fld7.fld1 = _7 as usize;
_13 = _12;
_17 = (_10,);
_15.fld0.fld6.0 = (_14.fld7.fld7.0, _4.1);
_16.fld4 = (false, _14.fld3.0.0);
_15.fld0.fld3 = [_16.fld4.0];
_14.fld7.fld0.fld0 = !_16.fld4.0;
Call(_14.fld5 = fn19(_14.fld7.fld7.0, _1, _14.fld7.fld5.fld1.fld0, _1, _5, _4.0, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14.fld7.fld5.fld2 = [_14.fld7.fld5.fld3.4,_5,_14.fld7.fld5.fld3.4,_11,_5];
_21.0 = !_16.fld4.0;
_14.fld7.fld5.fld3.3 = _14.fld3.3;
_16.fld6.4 = _5;
_14.fld0 = !220_u8;
_15.fld0.fld3 = [_14.fld7.fld0.fld0];
_16.fld6.1 = _15.fld0.fld6.1 ^ _15.fld0.fld6.1;
_14.fld7.fld3 = (-123_i8) ^ (-68_i8);
_14.fld7.fld5.fld4 = (_14.fld7.fld1,);
Goto(bb4)
}
bb4 = {
_14.fld7.fld7.1 = -_4.1;
_14.fld3.1 = _14.fld7.fld1 / 7_usize;
_14.fld7.fld0.fld6 = [_4.1,_14.fld3.3,_14.fld7.fld7.1,_14.fld3.3];
_16.fld3 = _15.fld0.fld3;
_16.fld3 = [_14.fld7.fld0.fld0];
_16.fld5.0 = 2134079144_i32 >> _14.fld6;
_14.fld7.fld0.fld6 = [_15.fld0.fld6.0.1,_14.fld7.fld5.fld3.3,_14.fld7.fld5.fld3.0.1,_14.fld3.3];
_14.fld4 = 16217772385838761958_u64;
_14.fld7.fld0.fld4 = _16.fld4.0 as i16;
_9 = [_16.fld2,_12,_16.fld2,_12,_12,_13,_13,_12];
_21.1 = _14.fld7.fld3 as i16;
match _14.fld4 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
16217772385838761958 => bb10,
_ => bb9
}
}
bb5 = {
_14.fld7.fld5.fld2 = [_14.fld7.fld5.fld3.4,_5,_14.fld7.fld5.fld3.4,_11,_5];
_21.0 = !_16.fld4.0;
_14.fld7.fld5.fld3.3 = _14.fld3.3;
_16.fld6.4 = _5;
_14.fld0 = !220_u8;
_15.fld0.fld3 = [_14.fld7.fld0.fld0];
_16.fld6.1 = _15.fld0.fld6.1 ^ _15.fld0.fld6.1;
_14.fld7.fld3 = (-123_i8) ^ (-68_i8);
_14.fld7.fld5.fld4 = (_14.fld7.fld1,);
Goto(bb4)
}
bb6 = {
_14.fld7.fld6 = [_12,_16.fld2,_12,_12,_12,_12,_12,_16.fld2];
_15.fld0.fld6.1 = !_10;
_14.fld3.0.0 = -_14.fld7.fld5.fld3.0.0;
_14.fld7.fld7 = (_4.0, _14.fld7.fld5.fld3.0.1);
_14.fld3.3 = _4.1 * _14.fld7.fld7.1;
_14.fld7.fld4 = [_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6];
(*_1) = _3;
_14.fld7.fld1 = _7 as usize;
_13 = _12;
_17 = (_10,);
_15.fld0.fld6.0 = (_14.fld7.fld7.0, _4.1);
_16.fld4 = (false, _14.fld3.0.0);
_15.fld0.fld3 = [_16.fld4.0];
_14.fld7.fld0.fld0 = !_16.fld4.0;
Call(_14.fld5 = fn19(_14.fld7.fld7.0, _1, _14.fld7.fld5.fld1.fld0, _1, _5, _4.0, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_14.fld7.fld2 = !15476721953360643372053795231667544167_u128;
_14.fld6 = (-78_i8) as u32;
_14.fld3.0.0 = 77_u8 as i16;
_14.fld7.fld5.fld3.4 = _5;
_14.fld7.fld5.fld3.0 = (_4.0, _4.1);
_9 = [_12,_12,_12,_12,_12,_12,_12,_12];
_14.fld7.fld5.fld3.2 = _14.fld7.fld2;
_14.fld7.fld7 = _14.fld7.fld5.fld3.0;
_16.fld0 = _2;
_16.fld2 = 11_u8 as isize;
_6.0 = [false,false];
_14.fld7.fld5.fld1.fld0 = core::ptr::addr_of!(_14.fld7.fld5.fld3.2);
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_14.fld7.fld5.fld5 = -_16.fld5.0;
_19.1 = !_14.fld7.fld7.1;
_15.fld0.fld0 = _2;
_16.fld5.0 = -_14.fld7.fld5.fld5;
_14.fld7.fld7 = (_14.fld3.0.0, _14.fld7.fld5.fld3.0.1);
_14.fld7.fld0.fld4 = _14.fld7.fld0.fld0 as i16;
_16.fld6.3 = _14.fld7.fld5.fld3.0.1 ^ _14.fld7.fld5.fld3.3;
(*_1) = _3 + _3;
_14.fld0 = !110_u8;
_2 = _9;
_26.3 = [_14.fld7.fld7.1,_14.fld7.fld5.fld3.3,_14.fld3.3,_16.fld6.3,_14.fld7.fld5.fld3.3,_4.1,_4.1,_14.fld7.fld7.1];
_15.fld0.fld0 = _2;
_15.fld2 = _14.fld4;
_14.fld3.2 = !_14.fld5;
_15.fld0.fld4.1 = _10 as i16;
_15.fld1 = [_16.fld2,_12,_12,_13,_13];
_14.fld7.fld5.fld3.1 = _14.fld6 as usize;
_15.fld0.fld6.4 = _14.fld7.fld5.fld3.4;
RET = !_14.fld7.fld5.fld3.3;
_19.3.0 = _15.fld2 as usize;
_14.fld3.1 = !_19.3.0;
_14.fld4 = _15.fld2;
_26.2 = [27681_u16,16183_u16,59448_u16,35224_u16,46503_u16];
_14.fld7.fld5.fld3.2 = _10 as u128;
_14.fld7.fld6 = [_12,_12,_12,_12,_13,_16.fld2,_12,_12];
_14.fld0 = 171_u8 - 249_u8;
match _14.fld4 {
16217772385838761958 => bb11,
_ => bb2
}
}
bb11 = {
_15.fld0.fld4 = (_21.0, _21.1);
_17 = (_14.fld3.1,);
_14.fld7.fld5.fld3.0.0 = -_15.fld0.fld6.0.0;
_16.fld6.0.1 = _14.fld7.fld5.fld3.3 + _14.fld3.3;
_27.0 = _15.fld0.fld6.0.0 - _14.fld7.fld0.fld4;
_26.0 = _26.2;
_19.0 = _14.fld7.fld0.fld0;
_14.fld7.fld5.fld0 = _14.fld7.fld4;
_26.3 = [_4.1,_16.fld6.0.1,_14.fld3.3,_16.fld6.3,_16.fld6.0.1,_14.fld7.fld7.1,_14.fld7.fld7.1,_14.fld3.3];
_21.0 = !_19.0;
_34.1 = _12 as i16;
_14.fld7.fld4 = _14.fld7.fld5.fld0;
_34.1 = _27.0;
Goto(bb12)
}
bb12 = {
_14.fld7.fld5.fld5 = _7 as i32;
_14.fld7.fld5.fld0 = [_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6];
_14.fld7.fld5.fld5 = _14.fld7.fld5.fld3.0.1 as i32;
_15.fld0.fld6.2 = _14.fld3.2 / 309255400501239720708427732163181863913_u128;
_14.fld7.fld0.fld1.fld0 = core::ptr::addr_of!(_14.fld3.2);
_14.fld7.fld0.fld0 = !_21.0;
_31 = (_10,);
_21 = (_14.fld7.fld0.fld0, _15.fld0.fld6.0.0);
_14.fld3.3 = _19.0 as i128;
_19.2 = core::ptr::addr_of!(_20);
_41.fld1 = Adt52 { fld0: _14.fld7.fld0.fld1.fld0,fld1: _15.fld0.fld6.0.1 };
match _14.fld4 {
0 => bb6,
16217772385838761958 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_4.0 = _15.fld0.fld6.0.0;
_34 = (_19.0, _27.0);
_41.fld0.0 = _15.fld0.fld6.1;
_16.fld6.0.1 = _14.fld7.fld5.fld3.3;
_16.fld6.0.0 = _15.fld0.fld4.1 + _16.fld4.1;
_15.fld0.fld6 = (_16.fld6.0, _10, _14.fld7.fld5.fld3.2, _14.fld3.3, _11);
_14.fld7.fld0.fld5 = [_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0];
_14.fld7.fld5.fld0 = [_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6,_14.fld6];
_27 = _14.fld7.fld5.fld3.0;
_15.fld0.fld6 = _14.fld7.fld5.fld3;
_41.fld1 = Adt52 { fld0: _14.fld7.fld0.fld1.fld0,fld1: _14.fld7.fld5.fld3.0.1 };
_18 = (*_1);
_34.1 = !_14.fld7.fld0.fld4;
_43.1.0 = _14.fld7.fld5.fld3.3 < _14.fld7.fld5.fld3.0.1;
_6.0 = [_14.fld7.fld0.fld0,_14.fld7.fld0.fld0];
_15.fld0.fld6.1 = _16.fld6.1 * _31.0;
_15.fld2 = _14.fld4 ^ _14.fld4;
_44 = _27.0 as f64;
_47.fld3 = _15.fld2;
_47.fld2.fld0.2 = [61573_u16,54365_u16,513_u16,57254_u16,39236_u16];
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(18_usize, 8_usize, Move(_8), 9_usize, Move(_9), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(18_usize, 11_usize, Move(_11), 7_usize, Move(_7), 21_usize, Move(_21), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i16,mut _2: *const f32,mut _3: *const u128,mut _4: *const f32,mut _5: char,mut _6: i16,mut _7: char) -> u128 {
mir! {
type RET = u128;
let _8: u8;
let _9: Adt58;
let _10: usize;
let _11: u32;
let _12: u128;
let _13: Adt52;
let _14: isize;
let _15: Adt56;
let _16: u8;
let _17: char;
let _18: char;
let _19: *const i64;
let _20: [bool; 2];
let _21: bool;
let _22: f64;
let _23: *const f32;
let _24: bool;
let _25: Adt48;
let _26: f64;
let _27: f64;
let _28: i128;
let _29: f32;
let _30: (i32, [char; 2]);
let _31: bool;
let _32: i64;
let _33: [char; 2];
let _34: isize;
let _35: u128;
let _36: char;
let _37: ();
let _38: ();
{
_5 = _7;
(*_3) = !329184013780327854077170182580463048151_u128;
(*_3) = !180279127463311988564541117294025955024_u128;
(*_2) = 57_i8 as f32;
RET = (*_3) >> (*_3);
_2 = _4;
_6 = _1 - _1;
RET = (*_3);
(*_4) = 109_isize as f32;
(*_3) = 204992051609845549427904069747607512028_u128 >> _6;
_3 = core::ptr::addr_of!((*_3));
_9.fld4 = [152211574576329213610520472670202880888_i128,129665899168855329975293239442569278443_i128,44541252341753054903584165475531238905_i128,(-157948298719609087310342850844327048565_i128)];
(*_2) = 14337670965265154541_u64 as f32;
_9.fld0.fld3 = !236_u8;
_9.fld0.fld0.4 = [879252650_u32,3354763200_u32,885526364_u32,287661416_u32,468972057_u32,158020102_u32,4063022998_u32];
_6 = -_1;
(*_3) = 161184528909579404848527286530567579172_u128;
_8 = _9.fld0.fld3 << _6;
_11 = !2301264954_u32;
_9.fld0.fld1 = 0_usize as f32;
(*_3) = 4717337753452709079_usize as u128;
_9.fld0.fld0.1.1 = _1;
RET = !(*_3);
_2 = core::ptr::addr_of!((*_2));
_9.fld0.fld1 = (*_2) / 0.0000000000000000000000000000000000000014925356060385416_f32;
_7 = _5;
_4 = core::ptr::addr_of!((*_4));
Goto(bb1)
}
bb1 = {
_9.fld0.fld0.0 = [19231_u16,61588_u16,51819_u16,51263_u16,6331_u16];
_12 = (*_3);
_9.fld0.fld0.1.0 = !true;
(*_2) = -_9.fld0.fld1;
_2 = _4;
_13.fld1 = !90654897404843864181379913212803502837_i128;
_7 = _5;
_9.fld2 = Adt52 { fld0: _3,fld1: _13.fld1 };
_9.fld2.fld1 = 616074320831024944_i64 as i128;
Goto(bb2)
}
bb2 = {
_15.fld3.1 = 25634_u16 as usize;
RET = !(*_3);
_15.fld7.fld5.fld5 = (-1599632920_i32);
_15.fld7.fld5.fld5 = (-353689222_i32) << _9.fld2.fld1;
_15.fld7.fld1 = _15.fld3.1 | _15.fld3.1;
_15.fld7.fld5.fld4.0 = _15.fld7.fld5.fld5 as usize;
_15.fld3.1 = !_15.fld7.fld5.fld4.0;
_15.fld7.fld5.fld3.0.1 = _13.fld1;
_15.fld7.fld5.fld0 = _9.fld0.fld0.4;
_15.fld7.fld0.fld0 = _9.fld0.fld0.1.0;
_15.fld7.fld7 = (_6, _9.fld2.fld1);
_12 = (*_3) % 312004935259290682495502327144265937938_u128;
_15.fld4 = 12644826199377090776_u64 - 16986782527465035089_u64;
_12 = (*_3);
_15.fld7.fld5.fld3.3 = -_9.fld2.fld1;
_13.fld1 = _15.fld7.fld5.fld5 as i128;
_15.fld7.fld5.fld3 = (_15.fld7.fld7, _15.fld7.fld1, _12, _9.fld2.fld1, _7);
_10 = _15.fld7.fld1 | _15.fld7.fld1;
_15.fld3.0.0 = _9.fld0.fld0.1.1;
_15.fld6 = !_11;
_15.fld7.fld2 = (*_3);
Goto(bb3)
}
bb3 = {
_9.fld0.fld0.3 = [_15.fld7.fld5.fld3.3,_13.fld1,_15.fld7.fld7.1,_9.fld2.fld1,_15.fld7.fld5.fld3.0.1,_13.fld1,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3];
(*_2) = _15.fld7.fld5.fld5 as f32;
_15.fld7.fld5.fld2 = [_7,_5,_7,_5,_7];
_9.fld4 = [_13.fld1,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3,_13.fld1];
_15.fld3.3 = _15.fld7.fld0.fld0 as i128;
_15.fld7.fld0.fld5 = [_8,_9.fld0.fld3,_8,_8,_8,_8,_8,_9.fld0.fld3];
_15.fld7.fld0.fld1 = Move(_9.fld2);
_15.fld3.0.0 = !_15.fld7.fld7.0;
(*_3) = _15.fld7.fld5.fld3.2 | _12;
_11 = !_15.fld6;
_15.fld7.fld5.fld1.fld0 = _15.fld7.fld0.fld1.fld0;
_6 = -_9.fld0.fld0.1.1;
_15.fld7.fld7.1 = -_13.fld1;
_15.fld0 = _8 / 81_u8;
_15.fld3.0.1 = _15.fld6 as i128;
_9.fld3 = [6452_u16];
_9.fld0.fld0.2 = [62390_u16,44391_u16,56609_u16,52004_u16,39350_u16];
(*_2) = _9.fld0.fld1 * _9.fld0.fld1;
_15.fld7.fld3 = !111_i8;
_16 = _15.fld0;
Goto(bb4)
}
bb4 = {
RET = (*_3) - _15.fld7.fld2;
_15.fld7.fld1 = _15.fld7.fld5.fld3.1 | _15.fld7.fld5.fld3.1;
_14 = 23_isize;
_18 = _15.fld7.fld5.fld3.4;
_9.fld0.fld0.3 = [_15.fld3.3,_15.fld7.fld0.fld1.fld1,_13.fld1,_15.fld7.fld7.1,_15.fld7.fld7.1,_15.fld7.fld7.1,_15.fld7.fld7.1,_13.fld1];
_15.fld5 = (*_3) * _15.fld7.fld5.fld3.2;
_15.fld7.fld0.fld2 = [_1,_1,_9.fld0.fld0.1.1,_9.fld0.fld0.1.1,_1,_9.fld0.fld0.1.1,_6,_15.fld7.fld5.fld3.0.0];
_6 = _9.fld0.fld0.1.1;
_8 = _16 | _9.fld0.fld3;
_15.fld3.0.0 = -_15.fld7.fld7.0;
_15.fld7.fld5.fld0 = [_15.fld6,_11,_15.fld6,_15.fld6,_11,_15.fld6,_15.fld6];
_9.fld4 = [_15.fld7.fld0.fld1.fld1,_15.fld7.fld7.1,_13.fld1,_15.fld7.fld5.fld3.3];
_15.fld7.fld0.fld1 = Adt52 { fld0: _3,fld1: _13.fld1 };
_15.fld3.2 = _15.fld0 as u128;
_9.fld0.fld0.3 = [_13.fld1,_15.fld7.fld5.fld3.0.1,_15.fld7.fld7.1,_15.fld7.fld0.fld1.fld1,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3,_13.fld1,_15.fld3.3];
_9.fld0.fld0.2 = [65010_u16,11232_u16,7754_u16,43786_u16,53161_u16];
_15.fld7.fld5.fld1.fld0 = core::ptr::addr_of!((*_3));
_15.fld7.fld6 = [_14,_14,_14,_14,_14,_14,_14,_14];
_15.fld3.0 = _15.fld7.fld5.fld3.0;
_15.fld7.fld3 = (-53_i8) + 57_i8;
match _14 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
23 => bb10,
_ => bb9
}
}
bb5 = {
_9.fld0.fld0.3 = [_15.fld7.fld5.fld3.3,_13.fld1,_15.fld7.fld7.1,_9.fld2.fld1,_15.fld7.fld5.fld3.0.1,_13.fld1,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3];
(*_2) = _15.fld7.fld5.fld5 as f32;
_15.fld7.fld5.fld2 = [_7,_5,_7,_5,_7];
_9.fld4 = [_13.fld1,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3,_13.fld1];
_15.fld3.3 = _15.fld7.fld0.fld0 as i128;
_15.fld7.fld0.fld5 = [_8,_9.fld0.fld3,_8,_8,_8,_8,_8,_9.fld0.fld3];
_15.fld7.fld0.fld1 = Move(_9.fld2);
_15.fld3.0.0 = !_15.fld7.fld7.0;
(*_3) = _15.fld7.fld5.fld3.2 | _12;
_11 = !_15.fld6;
_15.fld7.fld5.fld1.fld0 = _15.fld7.fld0.fld1.fld0;
_6 = -_9.fld0.fld0.1.1;
_15.fld7.fld7.1 = -_13.fld1;
_15.fld0 = _8 / 81_u8;
_15.fld3.0.1 = _15.fld6 as i128;
_9.fld3 = [6452_u16];
_9.fld0.fld0.2 = [62390_u16,44391_u16,56609_u16,52004_u16,39350_u16];
(*_2) = _9.fld0.fld1 * _9.fld0.fld1;
_15.fld7.fld3 = !111_i8;
_16 = _15.fld0;
Goto(bb4)
}
bb6 = {
_15.fld3.1 = 25634_u16 as usize;
RET = !(*_3);
_15.fld7.fld5.fld5 = (-1599632920_i32);
_15.fld7.fld5.fld5 = (-353689222_i32) << _9.fld2.fld1;
_15.fld7.fld1 = _15.fld3.1 | _15.fld3.1;
_15.fld7.fld5.fld4.0 = _15.fld7.fld5.fld5 as usize;
_15.fld3.1 = !_15.fld7.fld5.fld4.0;
_15.fld7.fld5.fld3.0.1 = _13.fld1;
_15.fld7.fld5.fld0 = _9.fld0.fld0.4;
_15.fld7.fld0.fld0 = _9.fld0.fld0.1.0;
_15.fld7.fld7 = (_6, _9.fld2.fld1);
_12 = (*_3) % 312004935259290682495502327144265937938_u128;
_15.fld4 = 12644826199377090776_u64 - 16986782527465035089_u64;
_12 = (*_3);
_15.fld7.fld5.fld3.3 = -_9.fld2.fld1;
_13.fld1 = _15.fld7.fld5.fld5 as i128;
_15.fld7.fld5.fld3 = (_15.fld7.fld7, _15.fld7.fld1, _12, _9.fld2.fld1, _7);
_10 = _15.fld7.fld1 | _15.fld7.fld1;
_15.fld3.0.0 = _9.fld0.fld0.1.1;
_15.fld6 = !_11;
_15.fld7.fld2 = (*_3);
Goto(bb3)
}
bb7 = {
_9.fld0.fld0.0 = [19231_u16,61588_u16,51819_u16,51263_u16,6331_u16];
_12 = (*_3);
_9.fld0.fld0.1.0 = !true;
(*_2) = -_9.fld0.fld1;
_2 = _4;
_13.fld1 = !90654897404843864181379913212803502837_i128;
_7 = _5;
_9.fld2 = Adt52 { fld0: _3,fld1: _13.fld1 };
_9.fld2.fld1 = 616074320831024944_i64 as i128;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
(*_3) = _15.fld5 >> _15.fld3.2;
_15.fld7.fld5.fld3.0.0 = _6 << _10;
_15.fld3.0.1 = _15.fld7.fld0.fld1.fld1 - _15.fld7.fld0.fld1.fld1;
_15.fld7.fld0.fld6 = _9.fld4;
_25.fld4.1 = _15.fld7.fld5.fld3.4 as i16;
_15.fld7.fld0.fld0 = !_9.fld0.fld0.1.0;
_15.fld7.fld5.fld3.3 = _15.fld7.fld0.fld1.fld1;
_15.fld7.fld5.fld4 = (_10,);
_25.fld3 = [_9.fld0.fld0.1.0];
_2 = core::ptr::addr_of!((*_4));
_9.fld0.fld0.2 = [2955_u16,54331_u16,24487_u16,18054_u16,21335_u16];
_25.fld4.0 = !_15.fld7.fld0.fld0;
_15.fld1 = core::ptr::addr_of_mut!(_9.fld0.fld0.0);
_15.fld7.fld5.fld3.0.1 = (*_3) as i128;
Goto(bb11)
}
bb11 = {
_7 = _5;
_25.fld6.3 = _15.fld3.0.1;
_25.fld4 = _9.fld0.fld0.1;
_9.fld0.fld1 = -(*_4);
_13 = Adt52 { fld0: _3,fld1: _15.fld7.fld0.fld1.fld1 };
_25.fld6.4 = _7;
_9.fld3 = [54324_u16];
Goto(bb12)
}
bb12 = {
_15.fld7.fld0.fld4 = _15.fld7.fld5.fld3.0.0 >> (*_3);
_15.fld7.fld0.fld6 = [_15.fld3.0.1,_15.fld7.fld5.fld3.0.1,_15.fld7.fld0.fld1.fld1,_15.fld7.fld5.fld3.0.1];
_15.fld3.4 = _15.fld7.fld5.fld3.4;
_25.fld3 = [_25.fld4.0];
_15.fld7.fld4 = [_15.fld6,_15.fld6,_15.fld6,_15.fld6,_15.fld6,_15.fld6,_15.fld6];
_20 = [_9.fld0.fld0.1.0,_15.fld7.fld0.fld0];
_25.fld5.1 = [_5,_25.fld6.4];
_15.fld7.fld2 = (*_3) >> _15.fld7.fld5.fld3.3;
_15.fld3.1 = _15.fld3.4 as usize;
_9.fld0.fld0.2 = _9.fld0.fld0.0;
match _14 {
0 => bb7,
1 => bb2,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
23 => bb19,
_ => bb18
}
}
bb13 = {
_9.fld0.fld0.0 = [19231_u16,61588_u16,51819_u16,51263_u16,6331_u16];
_12 = (*_3);
_9.fld0.fld0.1.0 = !true;
(*_2) = -_9.fld0.fld1;
_2 = _4;
_13.fld1 = !90654897404843864181379913212803502837_i128;
_7 = _5;
_9.fld2 = Adt52 { fld0: _3,fld1: _13.fld1 };
_9.fld2.fld1 = 616074320831024944_i64 as i128;
Goto(bb2)
}
bb14 = {
_15.fld3.1 = 25634_u16 as usize;
RET = !(*_3);
_15.fld7.fld5.fld5 = (-1599632920_i32);
_15.fld7.fld5.fld5 = (-353689222_i32) << _9.fld2.fld1;
_15.fld7.fld1 = _15.fld3.1 | _15.fld3.1;
_15.fld7.fld5.fld4.0 = _15.fld7.fld5.fld5 as usize;
_15.fld3.1 = !_15.fld7.fld5.fld4.0;
_15.fld7.fld5.fld3.0.1 = _13.fld1;
_15.fld7.fld5.fld0 = _9.fld0.fld0.4;
_15.fld7.fld0.fld0 = _9.fld0.fld0.1.0;
_15.fld7.fld7 = (_6, _9.fld2.fld1);
_12 = (*_3) % 312004935259290682495502327144265937938_u128;
_15.fld4 = 12644826199377090776_u64 - 16986782527465035089_u64;
_12 = (*_3);
_15.fld7.fld5.fld3.3 = -_9.fld2.fld1;
_13.fld1 = _15.fld7.fld5.fld5 as i128;
_15.fld7.fld5.fld3 = (_15.fld7.fld7, _15.fld7.fld1, _12, _9.fld2.fld1, _7);
_10 = _15.fld7.fld1 | _15.fld7.fld1;
_15.fld3.0.0 = _9.fld0.fld0.1.1;
_15.fld6 = !_11;
_15.fld7.fld2 = (*_3);
Goto(bb3)
}
bb15 = {
_9.fld0.fld0.3 = [_15.fld7.fld5.fld3.3,_13.fld1,_15.fld7.fld7.1,_9.fld2.fld1,_15.fld7.fld5.fld3.0.1,_13.fld1,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3];
(*_2) = _15.fld7.fld5.fld5 as f32;
_15.fld7.fld5.fld2 = [_7,_5,_7,_5,_7];
_9.fld4 = [_13.fld1,_15.fld7.fld5.fld3.3,_15.fld7.fld5.fld3.3,_13.fld1];
_15.fld3.3 = _15.fld7.fld0.fld0 as i128;
_15.fld7.fld0.fld5 = [_8,_9.fld0.fld3,_8,_8,_8,_8,_8,_9.fld0.fld3];
_15.fld7.fld0.fld1 = Move(_9.fld2);
_15.fld3.0.0 = !_15.fld7.fld7.0;
(*_3) = _15.fld7.fld5.fld3.2 | _12;
_11 = !_15.fld6;
_15.fld7.fld5.fld1.fld0 = _15.fld7.fld0.fld1.fld0;
_6 = -_9.fld0.fld0.1.1;
_15.fld7.fld7.1 = -_13.fld1;
_15.fld0 = _8 / 81_u8;
_15.fld3.0.1 = _15.fld6 as i128;
_9.fld3 = [6452_u16];
_9.fld0.fld0.2 = [62390_u16,44391_u16,56609_u16,52004_u16,39350_u16];
(*_2) = _9.fld0.fld1 * _9.fld0.fld1;
_15.fld7.fld3 = !111_i8;
_16 = _15.fld0;
Goto(bb4)
}
bb16 = {
Return()
}
bb17 = {
_9.fld0.fld0.0 = [19231_u16,61588_u16,51819_u16,51263_u16,6331_u16];
_12 = (*_3);
_9.fld0.fld0.1.0 = !true;
(*_2) = -_9.fld0.fld1;
_2 = _4;
_13.fld1 = !90654897404843864181379913212803502837_i128;
_7 = _5;
_9.fld2 = Adt52 { fld0: _3,fld1: _13.fld1 };
_9.fld2.fld1 = 616074320831024944_i64 as i128;
Goto(bb2)
}
bb18 = {
_15.fld3.1 = 25634_u16 as usize;
RET = !(*_3);
_15.fld7.fld5.fld5 = (-1599632920_i32);
_15.fld7.fld5.fld5 = (-353689222_i32) << _9.fld2.fld1;
_15.fld7.fld1 = _15.fld3.1 | _15.fld3.1;
_15.fld7.fld5.fld4.0 = _15.fld7.fld5.fld5 as usize;
_15.fld3.1 = !_15.fld7.fld5.fld4.0;
_15.fld7.fld5.fld3.0.1 = _13.fld1;
_15.fld7.fld5.fld0 = _9.fld0.fld0.4;
_15.fld7.fld0.fld0 = _9.fld0.fld0.1.0;
_15.fld7.fld7 = (_6, _9.fld2.fld1);
_12 = (*_3) % 312004935259290682495502327144265937938_u128;
_15.fld4 = 12644826199377090776_u64 - 16986782527465035089_u64;
_12 = (*_3);
_15.fld7.fld5.fld3.3 = -_9.fld2.fld1;
_13.fld1 = _15.fld7.fld5.fld5 as i128;
_15.fld7.fld5.fld3 = (_15.fld7.fld7, _15.fld7.fld1, _12, _9.fld2.fld1, _7);
_10 = _15.fld7.fld1 | _15.fld7.fld1;
_15.fld3.0.0 = _9.fld0.fld0.1.1;
_15.fld6 = !_11;
_15.fld7.fld2 = (*_3);
Goto(bb3)
}
bb19 = {
_15.fld7.fld2 = !_15.fld3.2;
RET = (*_3);
_25.fld6.0.0 = -_25.fld4.1;
(*_3) = !_15.fld3.2;
_8 = !_16;
_19 = core::ptr::addr_of!(_32);
(*_4) = _9.fld0.fld1;
_15.fld5 = _15.fld7.fld0.fld0 as u128;
(*_19) = (-1764892488681390397_i64);
_25.fld0 = [_14,_14,_14,_14,_14,_14,_14,_14];
_15.fld4 = _15.fld7.fld5.fld3.1 as u64;
_20 = [_25.fld4.0,_25.fld4.0];
_15.fld7.fld5.fld3 = _15.fld3;
_25.fld4.1 = _15.fld7.fld0.fld4;
_25.fld6.3 = _15.fld7.fld7.1 << _25.fld4.1;
_1 = _9.fld0.fld1 as i16;
_15.fld7.fld6 = [_14,_14,_14,_14,_14,_14,_14,_14];
_26 = _32 as f64;
_25.fld6.4 = _18;
_13.fld0 = _3;
_25.fld2 = -_14;
_15.fld7.fld5.fld0 = [_15.fld6,_15.fld6,_11,_11,_15.fld6,_15.fld6,_11];
Goto(bb20)
}
bb20 = {
Call(_37 = dump_var(19_usize, 10_usize, Move(_10), 11_usize, Move(_11), 6_usize, Move(_6), 16_usize, Move(_16)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_37 = dump_var(19_usize, 12_usize, Move(_12), 32_usize, Move(_32), 38_usize, _38, 38_usize, _38), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
pub fn main() {
                println!("{:?}", fn0(std::hint::black_box(false), std::hint::black_box('\u{3812d}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-102_i8)), std::hint::black_box((-31358_i16)), std::hint::black_box((-1353162248_i32)), std::hint::black_box(3799484084286041889_i64), std::hint::black_box(249326406110888006345585577920301769278_u128), std::hint::black_box(10737401243154007957_usize), std::hint::black_box(156_u8), std::hint::black_box(51258_u16), std::hint::black_box(2870174851_u32), std::hint::black_box(11274070694556472934_u64)));
                
            }
#[derive(Debug)]
pub struct Adt48 {
fld0: [isize; 8],
fld1: *mut [u16; 5],
fld2: isize,
fld3: [bool; 1],
fld4: (bool, i16),
fld5: (i32, [char; 2]),
fld6: ((i16, i128), usize, u128, i128, char),
}
#[derive(Debug)]
pub struct Adt49 {
fld0: Adt48,
fld1: [isize; 5],
fld2: u64,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt50 {
fld0: [u16; 5],
fld1: u8,
fld2: [isize; 5],
fld3: ([bool; 2],),
fld4: [char; 5],
fld5: f64,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt51 {
fld0: ([bool; 2],),
fld1: (bool, i128, *const i64, (usize,)),
fld2: *const [char; 5],
fld3: [bool; 1],
fld4: *mut [u8; 8],
fld5: u8,
fld6: i64,
}
#[derive(Debug)]
pub struct Adt52 {
fld0: *const u128,
fld1: i128,
}
#[derive(Debug)]
pub struct Adt53 {
fld0: bool,
fld1: Adt52,
fld2: [i16; 8],
fld3: *mut f64,
fld4: i16,
fld5: [u8; 8],
fld6: [i128; 4],
}
#[derive(Debug)]
pub struct Adt54 {
fld0: [u32; 7],
fld1: Adt52,
fld2: [char; 5],
fld3: ((i16, i128), usize, u128, i128, char),
fld4: (usize,),
fld5: i32,
}
#[derive(Debug)]
pub struct Adt55 {
fld0: Adt53,
fld1: usize,
fld2: u128,
fld3: i8,
fld4: [u32; 7],
fld5: Adt54,
fld6: [isize; 8],
fld7: (i16, i128),
}
#[derive(Debug)]
pub struct Adt56 {
fld0: u8,
fld1: *mut [u16; 5],
fld2: *mut f64,
fld3: ((i16, i128), usize, u128, i128, char),
fld4: u64,
fld5: u128,
fld6: u32,
fld7: Adt55,
}
#[derive(Debug)]
pub struct Adt57 {
fld0: ([u16; 5], (bool, i16), [u16; 5], [i128; 8], [u32; 7]),
fld1: f32,
fld2: u32,
fld3: u8,
}
#[derive(Debug)]
pub struct Adt58 {
fld0: Adt57,
fld1: [char; 2],
fld2: Adt52,
fld3: [u16; 1],
fld4: [i128; 4],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt59 {
fld0: u128,
}
#[derive(Debug)]
pub struct Adt60 {
fld0: (bool, i16),
fld1: (usize,),
fld2: *mut [u16; 5],
fld3: f32,
fld4: Adt48,
}
#[derive(Debug)]
pub struct Adt61 {
fld0: (usize,),
fld1: Adt52,
}
#[derive(Debug)]
pub struct Adt62 {
fld0: Adt58,
fld1: i32,
fld2: Adt61,
}
#[derive(Debug)]
pub struct Adt63 {
fld0: bool,
fld1: [isize; 8],
fld2: Adt57,
fld3: u64,
fld4: Adt60,
fld5: i32,
fld6: ([bool; 2],),
}
#[derive(Debug)]
pub struct Adt64 {
fld0: i128,
fld1: Adt54,
fld2: [bool; 2],
}

