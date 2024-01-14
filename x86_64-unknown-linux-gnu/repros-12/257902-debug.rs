#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics, const_hash)]
    #![allow(unused_parens, unused_assignments, overflowing_literals)]
    extern crate core;
    use core::intrinsics::mir::*;

    use std::fmt::Debug;

    #[inline(never)]
    fn dump_var(
        f: usize,
        var0: usize, val0: impl Debug,
        var1: usize, val1: impl Debug,
        var2: usize, val2: impl Debug,
        var3: usize, val3: impl Debug,
    ) {
        println!("fn{f}:_{var0} = {val0:?}\n_{var1} = {val1:?}\n_{var2} = {val2:?}\n_{var3} = {val3:?}");
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: bool,mut _2: u64,mut _3: isize,mut _4: i8,mut _5: usize,mut _6: i64) -> f64 {
mir! {
type RET = f64;
let _7: bool;
let _8: Adt51;
let _9: [usize; 2];
let _10: *const [bool; 8];
let _11: u64;
let _12: Adt50;
let _13: char;
let _14: f64;
let _15: *const [bool; 8];
let _16: ([u64; 2], i16, char);
let _17: [u16; 8];
let _18: i8;
let _19: ((i64, usize, (i16, [isize; 7])),);
let _20: isize;
let _21: Adt50;
let _22: Adt56;
let _23: ();
let _24: ();
{
_2 = 47750_u16 as u64;
_4 = (-110_i8);
_5 = 3_usize;
RET = 1643967111270444034_i64 as f64;
_6 = _5 as i64;
_6 = 456960212858004750_i64 ^ (-4807583837419892540_i64);
RET = 137351620325430104462317864244211207539_i128 as f64;
_2 = !16253712426233789242_u64;
RET = _6 as f64;
_7 = false;
_4 = (-26_i8);
_2 = 58_u8 as u64;
_1 = _7;
_6 = '\u{5a72f}' as i64;
_6 = (-7334527822610292590_i64);
_7 = _1 ^ _1;
RET = 181_u8 as f64;
_7 = _1;
Goto(bb1)
}
bb1 = {
RET = _4 as f64;
_9 = [_5,_5];
_2 = _6 as u64;
_6 = !2251232831716316227_i64;
_3 = (-33_isize);
_2 = !2163296581412810844_u64;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _7;
_11 = 38_u8 as u64;
_4 = '\u{39aaf}' as i8;
_4 = (-111107366_i32) as i8;
_9 = [_5,_5];
_4 = _2 as i8;
_2 = _7 as u64;
RET = _5 as f64;
_13 = '\u{6ed63}';
_13 = '\u{5ce89}';
_4 = RET as i8;
_17 = [28493_u16,49257_u16,42298_u16,22066_u16,26786_u16,22541_u16,31322_u16,40299_u16];
_17 = [42952_u16,18511_u16,56146_u16,52416_u16,52664_u16,6196_u16,54605_u16,26209_u16];
_16.1 = 14866_u16 as i16;
_17 = [105_u16,64071_u16,60501_u16,34267_u16,23721_u16,63818_u16,58954_u16,5736_u16];
RET = _2 as f64;
_17 = [49728_u16,54021_u16,1618_u16,17986_u16,50792_u16,19962_u16,65433_u16,18927_u16];
Call(_12 = fn1(_11, _1, _3, _7, _13, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = -(-9223372036854775808_isize);
_18 = _4;
_16.1 = !Field::<(f64, (i16, [isize; 7]))>(Variant(_12, 1), 4).1.0;
_19.0.2.0 = Field::<(i16, [isize; 7])>(Variant(_12, 1), 1).0;
RET = Field::<(f64, (i16, [isize; 7]))>(Variant(_12, 1), 4).0 + Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_12, 1), 6).1.0;
_16.2 = _13;
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_12, 1), 6)).1.1 = Field::<(f64, (i16, [isize; 7]))>(Variant(_12, 1), 4).1;
_7 = !Field::<bool>(Variant(_12, 1), 0);
place!(Field::<(f64, (i16, [isize; 7]))>(Variant(_12, 1), 4)) = (Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_12, 1), 6).1.0, Field::<(i16, [isize; 7])>(Variant(_12, 1), 1));
place!(Field::<(f64, (i16, [isize; 7]))>(Variant(_12, 1), 4)).1 = (_16.1, Field::<(i16, [isize; 7])>(Variant(_12, 1), 1).1);
_19.0.2.0 = !_16.1;
_22.fld1 = (Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_12, 1), 6).0, Field::<(i16, [isize; 7])>(Variant(_12, 1), 1).0, _16.2);
RET = -Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_12, 1), 6).1.0;
_1 = Field::<bool>(Variant(_12, 1), 0);
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(0_usize, 7_usize, Move(_7), 6_usize, Move(_6), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(0_usize, 1_usize, Move(_1), 18_usize, Move(_18), 24_usize, _24, 24_usize, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u64,mut _2: bool,mut _3: isize,mut _4: bool,mut _5: char,mut _6: u64) -> Adt50 {
mir! {
type RET = Adt50;
let _7: (char, i8);
let _8: i32;
let _9: char;
let _10: (i16, [isize; 7]);
let _11: [u64; 3];
let _12: i64;
let _13: f32;
let _14: f64;
let _15: char;
let _16: bool;
let _17: (u8,);
let _18: (char, i8);
let _19: u128;
let _20: [u16; 8];
let _21: f64;
let _22: i16;
let _23: (i64, usize, (i16, [isize; 7]));
let _24: Adt65;
let _25: isize;
let _26: isize;
let _27: bool;
let _28: usize;
let _29: bool;
let _30: i32;
let _31: [u64; 2];
let _32: char;
let _33: Adt52;
let _34: ([u64; 2], u16, [char; 8], i128);
let _35: Adt63;
let _36: ([usize; 2], u16, [usize; 2]);
let _37: f32;
let _38: u32;
let _39: isize;
let _40: (i16, [isize; 7]);
let _41: isize;
let _42: i8;
let _43: u16;
let _44: [char; 8];
let _45: f64;
let _46: *const i32;
let _47: i16;
let _48: u8;
let _49: char;
let _50: (bool,);
let _51: ((i64, usize, (i16, [isize; 7])),);
let _52: (i128, [bool; 8], [i32; 1], *const [bool; 8]);
let _53: Adt64;
let _54: Adt59;
let _55: (u8,);
let _56: ([usize; 2], u16, [usize; 2]);
let _57: usize;
let _58: u16;
let _59: f64;
let _60: [usize; 2];
let _61: [u64; 3];
let _62: (bool,);
let _63: f32;
let _64: isize;
let _65: Adt54;
let _66: [isize; 7];
let _67: (i16, [isize; 7]);
let _68: isize;
let _69: f32;
let _70: char;
let _71: (i16, [isize; 7]);
let _72: [usize; 2];
let _73: (char, i8);
let _74: f64;
let _75: Adt65;
let _76: f64;
let _77: (f64, (i16, [isize; 7]));
let _78: [i8; 8];
let _79: (bool,);
let _80: f32;
let _81: bool;
let _82: *const [bool; 8];
let _83: [i32; 1];
let _84: [u32; 4];
let _85: Adt58;
let _86: ((i64, usize, (i16, [isize; 7])),);
let _87: ((i64, usize, (i16, [isize; 7])),);
let _88: isize;
let _89: isize;
let _90: [u16; 8];
let _91: isize;
let _92: [i32; 1];
let _93: isize;
let _94: f32;
let _95: f32;
let _96: u64;
let _97: (f64, (i16, [isize; 7]));
let _98: char;
let _99: u8;
let _100: i128;
let _101: u32;
let _102: char;
let _103: *const i32;
let _104: (i16, [isize; 7]);
let _105: Adt60;
let _106: i16;
let _107: [u64; 3];
let _108: ([u64; 3],);
let _109: isize;
let _110: bool;
let _111: (u8,);
let _112: [u16; 8];
let _113: *const i64;
let _114: [u64; 3];
let _115: ([u64; 2], i16, char);
let _116: u64;
let _117: Adt50;
let _118: char;
let _119: usize;
let _120: Adt61;
let _121: ([u64; 3],);
let _122: f64;
let _123: f32;
let _124: (*const [bool; 8], [char; 8]);
let _125: isize;
let _126: isize;
let _127: f32;
let _128: *mut char;
let _129: isize;
let _130: ([u64; 2], u16, [char; 8], i128);
let _131: i64;
let _132: *mut char;
let _133: ();
let _134: ();
{
_6 = !_1;
_5 = '\u{1098d8}';
_1 = 96049290603479461203941895034069554677_u128 as u64;
_8 = !1167799957_i32;
_2 = _4;
_7 = (_5, 9_i8);
_3 = 115_isize;
_7.0 = _5;
_9 = _5;
_10.0 = (-16652_i16);
Goto(bb1)
}
bb1 = {
_2 = _7.1 >= _7.1;
_5 = _7.0;
_1 = !_6;
_7 = (_9, 40_i8);
_10.1 = [_3,_3,_3,_3,_3,_3,_3];
_2 = !_4;
_11 = [_6,_1,_1];
_3 = 10301284296758731599_usize as isize;
_5 = _9;
_4 = !_2;
_4 = _2;
_5 = _9;
_9 = _7.0;
Goto(bb2)
}
bb2 = {
_2 = _4;
_10.1 = [_3,_3,_3,_3,_3,_3,_3];
_12 = 4363368500529272579_i64;
_7.1 = (-34_i8) + 7_i8;
_10.0 = _8 as i16;
_1 = _6 << _7.1;
_12 = _3 as i64;
_6 = _1;
_4 = _2 & _2;
_13 = 139_u8 as f32;
_15 = _9;
_10.0 = (-1214_i16) * 10874_i16;
_3 = _2 as isize;
_6 = !_1;
_10.0 = !(-20647_i16);
_17 = (245_u8,);
_13 = _8 as f32;
_7.0 = _9;
_13 = 332885157978525951746313485556415002917_u128 as f32;
_11 = [_6,_1,_1];
_7 = (_5, (-64_i8));
Call(_14 = core::intrinsics::transmute(_6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = (_15, (-47_i8));
_18.1 = _13 as i8;
Goto(bb4)
}
bb4 = {
_19 = 191181363219063941731932234206451957606_u128 + 325668241786312714374092262243976488207_u128;
_16 = _5 == _15;
_17.0 = 128_u8;
_6 = _1 * _1;
_21 = _14 - _14;
_1 = !_6;
_12 = 25763283175786516747460911372228156205_i128 as i64;
_18.0 = _5;
_7.0 = _5;
_7 = (_9, _18.1);
_8 = 1915177124_i32 << _12;
_14 = _21;
_7.0 = _5;
_20 = [3564_u16,32260_u16,56950_u16,18590_u16,61365_u16,4012_u16,6588_u16,14194_u16];
_15 = _5;
_5 = _15;
match _17.0 {
0 => bb5,
128 => bb7,
_ => bb6
}
}
bb5 = {
_7 = (_15, (-47_i8));
_18.1 = _13 as i8;
Goto(bb4)
}
bb6 = {
_2 = _4;
_10.1 = [_3,_3,_3,_3,_3,_3,_3];
_12 = 4363368500529272579_i64;
_7.1 = (-34_i8) + 7_i8;
_10.0 = _8 as i16;
_1 = _6 << _7.1;
_12 = _3 as i64;
_6 = _1;
_4 = _2 & _2;
_13 = 139_u8 as f32;
_15 = _9;
_10.0 = (-1214_i16) * 10874_i16;
_3 = _2 as isize;
_6 = !_1;
_10.0 = !(-20647_i16);
_17 = (245_u8,);
_13 = _8 as f32;
_7.0 = _9;
_13 = 332885157978525951746313485556415002917_u128 as f32;
_11 = [_6,_1,_1];
_7 = (_5, (-64_i8));
Call(_14 = core::intrinsics::transmute(_6), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_24.fld1.2 = core::ptr::addr_of_mut!(_19);
_13 = _18.1 as f32;
_5 = _15;
_23.2.0 = _10.0 & _10.0;
_23.0 = _12;
_22 = 12775697777581088155_usize as i16;
_24.fld1.1.0 = _21 * _14;
_22 = _23.2.0 | _23.2.0;
_7.1 = -_18.1;
_23.0 = _12;
_24.fld0 = [2_usize,7_usize];
_24.fld1.1 = (_14, _10);
_29 = _2;
_24.fld1.1.0 = _14;
_24.fld1.1.1.1 = [_3,_3,_3,_3,_3,_3,_3];
_2 = _4;
_9 = _7.0;
_20 = [39848_u16,50805_u16,14470_u16,8795_u16,23867_u16,28939_u16,46850_u16,38460_u16];
_31 = [_6,_6];
Goto(bb8)
}
bb8 = {
_23.0 = -_12;
_10.1 = [_3,_3,_3,_3,_3,_3,_3];
_18.1 = _7.1;
_24.fld1.0 = [_1,_1];
_27 = _2 | _2;
_18.1 = !_7.1;
_18.1 = _7.1;
_34.3 = !(-46104918150559940222625871883308464479_i128);
_31 = [_6,_1];
_7.0 = _9;
_1 = _6;
_35.fld1.fld4 = core::ptr::addr_of!(_23.0);
_36.1 = 55399_u16;
_28 = !5_usize;
_26 = -_3;
_30 = _8 << _22;
_10 = _24.fld1.1.1;
_35.fld1.fld4 = core::ptr::addr_of!(_23.0);
_19 = 310719310299611976292345140250753470147_u128;
Call(_35.fld1.fld0 = fn2(_21, _24.fld1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7 = (_9, _18.1);
SetDiscriminant(_35.fld1.fld0, 1);
_33 = Adt52::Variant3 { fld0: _10 };
_35.fld1.fld0 = Adt49::Variant3 { fld0: _17.0,fld1: _28,fld2: _34.3,fld3: _11 };
_4 = !_29;
_22 = !_24.fld1.1.1.0;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)) = (_22, _10.1);
_23.2.1 = [_3,_3,_3,_3,_26,_3,_3];
_25 = !_26;
_37 = _13;
_28 = _36.1 as usize;
_5 = _18.0;
_35.fld1.fld1.0 = [_1,_6];
_23 = (_12, _28, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0));
Goto(bb10)
}
bb10 = {
_40 = (_22, _10.1);
_17 = (Field::<u8>(Variant(_35.fld1.fld0, 3), 0),);
_35.fld1.fld2 = [_25,_3,_3,_3,_3,_3,_26];
_5 = _18.0;
_17.0 = 1929244631_u32 as u8;
_27 = !_2;
_17.0 = Field::<u8>(Variant(_35.fld1.fld0, 3), 0);
_31 = [_6,_6];
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)).0 = _24.fld1.1.1.0 * _24.fld1.1.1.0;
_41 = _26;
place!(Field::<i128>(Variant(_35.fld1.fld0, 3), 2)) = -_34.3;
_35.fld1.fld0 = Adt49::Variant1 { fld0: _35.fld1.fld1.0,fld1: _40.1 };
_6 = _17.0 as u64;
_18 = (_5, _7.1);
_34.3 = !(-104490881872037275146126895230399884144_i128);
_17.0 = !200_u8;
_24.fld1.1.1 = (_10.0, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1);
_24.fld1.1.1 = (_22, _40.1);
_35.fld1.fld3 = _23.1;
_18.0 = _5;
_11 = [_1,_1,_6];
_23 = (_12, _28, _40);
_34.3 = -(-144775445705049143468361640792097533104_i128);
_44 = [_15,_5,_5,_15,_9,_7.0,_5,_18.0];
_35.fld0 = Adt54::Variant1 { fld0: _16 };
_40 = (_22, _23.2.1);
Call(_7.0 = fn19(Move(_35.fld1.fld0), _23, _35.fld1.fld2, _31, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1, _10.1, _24.fld1, Move(_24), Field::<bool>(Variant(_35.fld0, 1), 0)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_24.fld1.1.1.0 = Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).0 + _40.0;
_29 = Field::<bool>(Variant(_35.fld0, 1), 0);
_25 = _34.3 as isize;
_26 = _29 as isize;
_6 = !_1;
_34 = (_35.fld1.fld1.0, _36.1, _44, 104776495488487235854777864477490833346_i128);
_23.2.0 = Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).0;
_34.0 = _35.fld1.fld1.0;
SetDiscriminant(_35.fld0, 0);
_46 = core::ptr::addr_of!(_8);
_36.2 = [_23.1,_28];
_35.fld1.fld1 = (_34.0, _24.fld1.1.1.0, _7.0);
_35.fld1.fld3 = !_23.1;
_23.2.0 = -Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).0;
_38 = 3410942591_u32;
_7.0 = _9;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)).1 = [_26,_3,_26,_3,_26,_26,_3];
_18.0 = _5;
_35.fld1.fld5 = (*_46) | (*_46);
_24.fld1.2 = core::ptr::addr_of_mut!(_19);
_44 = [_18.0,_15,_7.0,_35.fld1.fld1.2,_9,_35.fld1.fld1.2,_18.0,_5];
_24.fld1.1 = (_14, _40);
_35.fld1.fld1.1 = _22;
_35.fld1.fld1.1 = _23.2.0 & _40.0;
match _34.3 {
0 => bb7,
1 => bb12,
2 => bb13,
104776495488487235854777864477490833346 => bb15,
_ => bb14
}
}
bb12 = {
_40 = (_22, _10.1);
_17 = (Field::<u8>(Variant(_35.fld1.fld0, 3), 0),);
_35.fld1.fld2 = [_25,_3,_3,_3,_3,_3,_26];
_5 = _18.0;
_17.0 = 1929244631_u32 as u8;
_27 = !_2;
_17.0 = Field::<u8>(Variant(_35.fld1.fld0, 3), 0);
_31 = [_6,_6];
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)).0 = _24.fld1.1.1.0 * _24.fld1.1.1.0;
_41 = _26;
place!(Field::<i128>(Variant(_35.fld1.fld0, 3), 2)) = -_34.3;
_35.fld1.fld0 = Adt49::Variant1 { fld0: _35.fld1.fld1.0,fld1: _40.1 };
_6 = _17.0 as u64;
_18 = (_5, _7.1);
_34.3 = !(-104490881872037275146126895230399884144_i128);
_17.0 = !200_u8;
_24.fld1.1.1 = (_10.0, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1);
_24.fld1.1.1 = (_22, _40.1);
_35.fld1.fld3 = _23.1;
_18.0 = _5;
_11 = [_1,_1,_6];
_23 = (_12, _28, _40);
_34.3 = -(-144775445705049143468361640792097533104_i128);
_44 = [_15,_5,_5,_15,_9,_7.0,_5,_18.0];
_35.fld0 = Adt54::Variant1 { fld0: _16 };
_40 = (_22, _23.2.1);
Call(_7.0 = fn19(Move(_35.fld1.fld0), _23, _35.fld1.fld2, _31, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1, _10.1, _24.fld1, Move(_24), Field::<bool>(Variant(_35.fld0, 1), 0)), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_19 = 191181363219063941731932234206451957606_u128 + 325668241786312714374092262243976488207_u128;
_16 = _5 == _15;
_17.0 = 128_u8;
_6 = _1 * _1;
_21 = _14 - _14;
_1 = !_6;
_12 = 25763283175786516747460911372228156205_i128 as i64;
_18.0 = _5;
_7.0 = _5;
_7 = (_9, _18.1);
_8 = 1915177124_i32 << _12;
_14 = _21;
_7.0 = _5;
_20 = [3564_u16,32260_u16,56950_u16,18590_u16,61365_u16,4012_u16,6588_u16,14194_u16];
_15 = _5;
_5 = _15;
match _17.0 {
0 => bb5,
128 => bb7,
_ => bb6
}
}
bb14 = {
_2 = _4;
_10.1 = [_3,_3,_3,_3,_3,_3,_3];
_12 = 4363368500529272579_i64;
_7.1 = (-34_i8) + 7_i8;
_10.0 = _8 as i16;
_1 = _6 << _7.1;
_12 = _3 as i64;
_6 = _1;
_4 = _2 & _2;
_13 = 139_u8 as f32;
_15 = _9;
_10.0 = (-1214_i16) * 10874_i16;
_3 = _2 as isize;
_6 = !_1;
_10.0 = !(-20647_i16);
_17 = (245_u8,);
_13 = _8 as f32;
_7.0 = _9;
_13 = 332885157978525951746313485556415002917_u128 as f32;
_11 = [_6,_1,_1];
_7 = (_5, (-64_i8));
Call(_14 = core::intrinsics::transmute(_6), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
place!(Field::<isize>(Variant(_35.fld0, 0), 2)) = _26;
place!(Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0)).2 = [_28,_23.1];
place!(Field::<Adt49>(Variant(_35.fld0, 0), 1)) = Adt49::Variant3 { fld0: _17.0,fld1: _35.fld1.fld3,fld2: _34.3,fld3: _11 };
_8 = _35.fld1.fld5;
_24.fld1.1.1 = _10;
_25 = Field::<isize>(Variant(_35.fld0, 0), 2) & _41;
_23.1 = _35.fld1.fld3 | Field::<usize>(Variant(Field::<Adt49>(Variant(_35.fld0, 0), 1), 3), 1);
_3 = _25;
_38 = 3611146421_u32 & 1567354937_u32;
_24.fld1.1 = (_14, _40);
place!(Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0)).1 = (*_46) as u16;
SetDiscriminant(Field::<Adt49>(Variant(_35.fld0, 0), 1), 1);
_36.1 = _34.1 | _34.1;
place!(Field::<[isize; 7]>(Variant(place!(Field::<Adt49>(Variant(_35.fld0, 0), 1)), 1), 1)) = Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1;
_22 = _24.fld1.1.1.0 * _24.fld1.1.1.0;
_27 = _4;
_2 = _16;
place!(Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0)).0 = [_23.1,_35.fld1.fld3];
place!(Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0)).1 = _14 as u16;
match _34.3 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb4,
4 => bb13,
5 => bb12,
6 => bb7,
104776495488487235854777864477490833346 => bb17,
_ => bb16
}
}
bb16 = {
_19 = 191181363219063941731932234206451957606_u128 + 325668241786312714374092262243976488207_u128;
_16 = _5 == _15;
_17.0 = 128_u8;
_6 = _1 * _1;
_21 = _14 - _14;
_1 = !_6;
_12 = 25763283175786516747460911372228156205_i128 as i64;
_18.0 = _5;
_7.0 = _5;
_7 = (_9, _18.1);
_8 = 1915177124_i32 << _12;
_14 = _21;
_7.0 = _5;
_20 = [3564_u16,32260_u16,56950_u16,18590_u16,61365_u16,4012_u16,6588_u16,14194_u16];
_15 = _5;
_5 = _15;
match _17.0 {
0 => bb5,
128 => bb7,
_ => bb6
}
}
bb17 = {
_34.1 = !_36.1;
place!(Field::<[u64; 2]>(Variant(place!(Field::<Adt49>(Variant(_35.fld0, 0), 1)), 1), 0)) = _34.0;
_31 = [_6,_1];
_23.0 = _29 as i64;
_31 = [_6,_1];
match _34.3 {
0 => bb8,
1 => bb16,
2 => bb3,
3 => bb6,
104776495488487235854777864477490833346 => bb18,
_ => bb5
}
}
bb18 = {
SetDiscriminant(Field::<Adt49>(Variant(_35.fld0, 0), 1), 1);
_51 = (_23,);
SetDiscriminant(_33, 0);
place!(Field::<(i16, [isize; 7])>(Variant(_33, 0), 1)).0 = _18.1 as i16;
place!(Field::<(bool,)>(Variant(_33, 0), 3)).0 = !_2;
_31 = [_1,_6];
_51.0.1 = _28 * _23.1;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 0), 1)).1 = [_25,_3,_25,_3,Field::<isize>(Variant(_35.fld0, 0), 2),_3,_3];
_42 = -_7.1;
_11 = [_1,_6,_6];
place!(Field::<Adt49>(Variant(_35.fld0, 0), 1)) = Adt49::Variant1 { fld0: _34.0,fld1: _24.fld1.1.1.1 };
place!(Field::<u128>(Variant(_33, 0), 2)) = _14 as u128;
_53.fld3 = !_18.1;
_34.2 = [_18.0,_7.0,_35.fld1.fld1.2,_7.0,_35.fld1.fld1.2,_7.0,_18.0,_5];
_23.2.1 = [_26,_26,Field::<isize>(Variant(_35.fld0, 0), 2),_3,Field::<isize>(Variant(_35.fld0, 0), 2),Field::<isize>(Variant(_35.fld0, 0), 2),_26];
_48 = _34.3 as u8;
_52.0 = _38 as i128;
Goto(bb19)
}
bb19 = {
_45 = _21;
_11 = [_1,_6,_6];
_2 = !Field::<(bool,)>(Variant(_33, 0), 3).0;
_32 = _35.fld1.fld1.2;
_17.0 = _48 * _48;
_23 = (_12, _51.0.1, Field::<(i16, [isize; 7])>(Variant(_33, 0), 1));
_54.fld1.1 = [_29,_16,_2,_2,_27,_27,_27,Field::<(bool,)>(Variant(_33, 0), 3).0];
_53.fld0 = !_2;
_15 = _32;
_51.0.2.0 = _10.0;
_29 = _16;
match _34.3 {
0 => bb1,
1 => bb4,
2 => bb3,
104776495488487235854777864477490833346 => bb21,
_ => bb20
}
}
bb20 = {
_40 = (_22, _10.1);
_17 = (Field::<u8>(Variant(_35.fld1.fld0, 3), 0),);
_35.fld1.fld2 = [_25,_3,_3,_3,_3,_3,_26];
_5 = _18.0;
_17.0 = 1929244631_u32 as u8;
_27 = !_2;
_17.0 = Field::<u8>(Variant(_35.fld1.fld0, 3), 0);
_31 = [_6,_6];
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)).0 = _24.fld1.1.1.0 * _24.fld1.1.1.0;
_41 = _26;
place!(Field::<i128>(Variant(_35.fld1.fld0, 3), 2)) = -_34.3;
_35.fld1.fld0 = Adt49::Variant1 { fld0: _35.fld1.fld1.0,fld1: _40.1 };
_6 = _17.0 as u64;
_18 = (_5, _7.1);
_34.3 = !(-104490881872037275146126895230399884144_i128);
_17.0 = !200_u8;
_24.fld1.1.1 = (_10.0, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1);
_24.fld1.1.1 = (_22, _40.1);
_35.fld1.fld3 = _23.1;
_18.0 = _5;
_11 = [_1,_1,_6];
_23 = (_12, _28, _40);
_34.3 = -(-144775445705049143468361640792097533104_i128);
_44 = [_15,_5,_5,_15,_9,_7.0,_5,_18.0];
_35.fld0 = Adt54::Variant1 { fld0: _16 };
_40 = (_22, _23.2.1);
Call(_7.0 = fn19(Move(_35.fld1.fld0), _23, _35.fld1.fld2, _31, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1, _10.1, _24.fld1, Move(_24), Field::<bool>(Variant(_35.fld0, 1), 0)), ReturnTo(bb11), UnwindUnreachable())
}
bb21 = {
_34.1 = _36.1 * Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0).1;
_35.fld1.fld0 = Adt49::Variant1 { fld0: _35.fld1.fld1.0,fld1: _23.2.1 };
match _34.3 {
0 => bb18,
1 => bb20,
2 => bb16,
3 => bb22,
104776495488487235854777864477490833346 => bb24,
_ => bb23
}
}
bb22 = {
_2 = _7.1 >= _7.1;
_5 = _7.0;
_1 = !_6;
_7 = (_9, 40_i8);
_10.1 = [_3,_3,_3,_3,_3,_3,_3];
_2 = !_4;
_11 = [_6,_1,_1];
_3 = 10301284296758731599_usize as isize;
_5 = _9;
_4 = !_2;
_4 = _2;
_5 = _9;
_9 = _7.0;
Goto(bb2)
}
bb23 = {
_7 = (_9, _18.1);
SetDiscriminant(_35.fld1.fld0, 1);
_33 = Adt52::Variant3 { fld0: _10 };
_35.fld1.fld0 = Adt49::Variant3 { fld0: _17.0,fld1: _28,fld2: _34.3,fld3: _11 };
_4 = !_29;
_22 = !_24.fld1.1.1.0;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)) = (_22, _10.1);
_23.2.1 = [_3,_3,_3,_3,_26,_3,_3];
_25 = !_26;
_37 = _13;
_28 = _36.1 as usize;
_5 = _18.0;
_35.fld1.fld1.0 = [_1,_6];
_23 = (_12, _28, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0));
Goto(bb10)
}
bb24 = {
place!(Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4)).1.2.1 = Field::<[isize; 7]>(Variant(_35.fld1.fld0, 1), 1);
_47 = -_10.0;
_23.2.0 = _22 & _22;
_12 = _51.0.0;
_17.0 = _37 as u8;
_52.1 = [Field::<(bool,)>(Variant(_33, 0), 3).0,_29,_53.fld0,_4,_29,_2,_16,_2];
place!(Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4)).1.1 = !_23.1;
_35.fld1.fld5 = !_8;
place!(Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4)).1 = _23;
place!(Field::<Adt49>(Variant(_35.fld0, 0), 1)) = Move(_35.fld1.fld0);
SetDiscriminant(_35.fld0, 1);
place!(Field::<bool>(Variant(_35.fld0, 1), 0)) = _6 >= _1;
SetDiscriminant(_35.fld0, 1);
_47 = !Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4).1.2.0;
Goto(bb25)
}
bb25 = {
_54.fld1.0 = _52.0;
_23.1 = _48 as usize;
_18.1 = -_53.fld3;
_52.3 = core::ptr::addr_of!(_54.fld1.1);
_7.0 = _5;
_24.fld1.2 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_33, 0), 2)));
_53.fld1 = Adt52::Variant1 { fld0: _17,fld1: _37,fld2: _34.3,fld3: _21 };
_11 = [_6,_1,_6];
_51.0.1 = _23.1;
place!(Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4)).2 = _6 ^ _6;
_56 = (_36.2, _36.1, _36.2);
SetDiscriminant(_53.fld1, 3);
place!(Field::<[u16; 8]>(Variant(_33, 0), 5)) = [_34.1,_34.1,_34.1,_34.1,_56.1,_34.1,_34.1,_34.1];
_10 = Field::<(i16, [isize; 7])>(Variant(_33, 0), 1);
_39 = _35.fld1.fld1.1 as isize;
place!(Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4)).3 = -_34.3;
_35.fld1.fld1.0 = [_1,_1];
_1 = _6 ^ _6;
_60 = [_51.0.1,_51.0.1];
match _34.3 {
0 => bb12,
1 => bb22,
2 => bb9,
3 => bb14,
4 => bb5,
5 => bb6,
104776495488487235854777864477490833346 => bb27,
_ => bb26
}
}
bb26 = {
place!(Field::<isize>(Variant(_35.fld0, 0), 2)) = _26;
place!(Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0)).2 = [_28,_23.1];
place!(Field::<Adt49>(Variant(_35.fld0, 0), 1)) = Adt49::Variant3 { fld0: _17.0,fld1: _35.fld1.fld3,fld2: _34.3,fld3: _11 };
_8 = _35.fld1.fld5;
_24.fld1.1.1 = _10;
_25 = Field::<isize>(Variant(_35.fld0, 0), 2) & _41;
_23.1 = _35.fld1.fld3 | Field::<usize>(Variant(Field::<Adt49>(Variant(_35.fld0, 0), 1), 3), 1);
_3 = _25;
_38 = 3611146421_u32 & 1567354937_u32;
_24.fld1.1 = (_14, _40);
place!(Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0)).1 = (*_46) as u16;
SetDiscriminant(Field::<Adt49>(Variant(_35.fld0, 0), 1), 1);
_36.1 = _34.1 | _34.1;
place!(Field::<[isize; 7]>(Variant(place!(Field::<Adt49>(Variant(_35.fld0, 0), 1)), 1), 1)) = Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1;
_22 = _24.fld1.1.1.0 * _24.fld1.1.1.0;
_27 = _4;
_2 = _16;
place!(Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0)).0 = [_23.1,_35.fld1.fld3];
place!(Field::<([usize; 2], u16, [usize; 2])>(Variant(_35.fld0, 0), 0)).1 = _14 as u16;
match _34.3 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb4,
4 => bb13,
5 => bb12,
6 => bb7,
104776495488487235854777864477490833346 => bb17,
_ => bb16
}
}
bb27 = {
_7.0 = _5;
_47 = -Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4).1.2.0;
place!(Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4)).1.1 = !_28;
_53.fld3 = _18.1 >> _34.1;
Goto(bb28)
}
bb28 = {
_18.1 = _1 as i8;
_35.fld1.fld3 = _23.1;
_19 = Field::<u128>(Variant(_33, 0), 2);
_36 = _56;
_22 = -Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4).1.2.0;
_11 = [_1,_1,_1];
_51.0.0 = _12 ^ _12;
place!(Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4)).1.1 = _35.fld1.fld3;
place!(Field::<(bool,)>(Variant(_33, 0), 3)).0 = _16 | _53.fld0;
(*_46) = -_30;
_55.0 = _48 | _17.0;
_54.fld1.3 = core::ptr::addr_of!(_52.1);
_34.2 = _44;
_62.0 = !_16;
_25 = _39;
_19 = Field::<u128>(Variant(_33, 0), 2);
_42 = _30 as i8;
_63 = _37 - _37;
_52.0 = _34.3 >> _48;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 0), 1)) = (Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4).1.2.0, _10.1);
_48 = !_55.0;
_9 = _18.0;
_12 = _48 as i64;
Goto(bb29)
}
bb29 = {
Goto(bb30)
}
bb30 = {
_55.0 = (*_46) as u8;
_10.0 = _40.0;
(*_46) = -_35.fld1.fld5;
_63 = _54.fld1.0 as f32;
_27 = _35.fld1.fld5 >= _30;
match _34.3 {
0 => bb17,
1 => bb29,
2 => bb25,
3 => bb4,
4 => bb5,
104776495488487235854777864477490833346 => bb32,
_ => bb31
}
}
bb31 = {
_23.0 = -_12;
_10.1 = [_3,_3,_3,_3,_3,_3,_3];
_18.1 = _7.1;
_24.fld1.0 = [_1,_1];
_27 = _2 | _2;
_18.1 = !_7.1;
_18.1 = _7.1;
_34.3 = !(-46104918150559940222625871883308464479_i128);
_31 = [_6,_1];
_7.0 = _9;
_1 = _6;
_35.fld1.fld4 = core::ptr::addr_of!(_23.0);
_36.1 = 55399_u16;
_28 = !5_usize;
_26 = -_3;
_30 = _8 << _22;
_10 = _24.fld1.1.1;
_35.fld1.fld4 = core::ptr::addr_of!(_23.0);
_19 = 310719310299611976292345140250753470147_u128;
Call(_35.fld1.fld0 = fn2(_21, _24.fld1), ReturnTo(bb9), UnwindUnreachable())
}
bb32 = {
_27 = !_29;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 0), 1)) = (_22, _23.2.1);
place!(Field::<(i16, [isize; 7])>(Variant(_33, 0), 1)).1 = [_3,_26,_3,_25,_39,_25,_3];
_33 = Adt52::Variant1 { fld0: _55,fld1: _13,fld2: _34.3,fld3: _45 };
_3 = _19 as isize;
place!(Field::<(i16, [isize; 7])>(Variant(_53.fld1, 3), 0)).0 = _10.0 & _35.fld1.fld1.1;
_35.fld1.fld5 = _30;
_69 = -_63;
_5 = _32;
_54.fld1.2 = [_35.fld1.fld5];
_5 = _15;
_36.0 = _60;
_53.fld3 = _6 as i8;
place!(Field::<(i16, [isize; 7])>(Variant(_53.fld1, 3), 0)).1 = _40.1;
SetDiscriminant(_53.fld1, 2);
_61 = [_6,_1,_1];
_52.2 = [_35.fld1.fld5];
match _34.3 {
0 => bb25,
104776495488487235854777864477490833346 => bb33,
_ => bb13
}
}
bb33 = {
_54 = Adt59 { fld0: _20,fld1: _52 };
place!(Field::<f64>(Variant(_33, 1), 3)) = _24.fld1.1.0;
_75.fld1.1 = _24.fld1.1;
place!(Field::<*const [u64; 3]>(Variant(_53.fld1, 2), 0)) = core::ptr::addr_of!(_11);
_44 = [_32,_15,_5,_35.fld1.fld1.2,_5,_32,_35.fld1.fld1.2,_32];
_75.fld1.1.1.0 = !_35.fld1.fld1.1;
_75.fld1.0 = _34.0;
_17.0 = _19 as u8;
place!(Field::<f32>(Variant(_33, 1), 1)) = -_37;
_56.0 = _60;
_18 = _7;
_71 = (_35.fld1.fld1.1, _23.2.1);
_73 = (_35.fld1.fld1.2, _53.fld3);
SetDiscriminant(_33, 2);
_43 = !_34.1;
_71.1 = _23.2.1;
_75.fld1.1.1.1 = [_3,_41,_39,_3,_25,_25,_3];
_54 = Adt59 { fld0: _20,fld1: _52 };
_62 = (_53.fld0,);
_21 = -_24.fld1.1.0;
_18 = (_5, _73.1);
place!(Field::<i64>(Variant(_33, 2), 3)) = _12 >> _23.1;
_61 = [_6,_6,_6];
_56.2 = [_51.0.1,_23.1];
_23.2.1 = [_3,_26,_3,_25,_39,_3,_3];
Goto(bb34)
}
bb34 = {
_67.0 = _40.0;
_34 = (_75.fld1.0, _43, _44, _54.fld1.0);
place!(Field::<u32>(Variant(_33, 2), 2)) = _38;
_27 = _16;
_23.2 = (_71.0, _10.1);
_41 = _6 as isize;
_59 = _30 as f64;
_40 = _10;
_18.0 = _15;
_25 = _39 * _39;
_3 = _41 & _39;
_54.fld1.0 = _34.3 + _52.0;
_79.0 = !_16;
_40.1 = [_3,_3,_41,_3,_3,_26,_3];
_45 = -_21;
_67.1 = [_25,_3,_3,_41,_26,_25,_26];
_53.fld1 = Adt52::Variant1 { fld0: _55,fld1: _69,fld2: _54.fld1.0,fld3: _75.fld1.1.0 };
_35.fld1.fld2 = [_41,_41,_3,_41,_25,_41,_25];
_77.1.0 = _8 as i16;
_55.0 = _32 as u8;
Goto(bb35)
}
bb35 = {
_35.fld1.fld3 = _51.0.1;
_24.fld1.0 = _75.fld1.0;
_24.fld1.1.1 = (_47, _35.fld1.fld2);
_75.fld1.1 = _24.fld1.1;
_51.0.0 = _12 + Field::<i64>(Variant(_33, 2), 3);
_67 = (_47, _75.fld1.1.1.1);
_11 = _61;
_24.fld1.1.1 = (_23.2.0, _40.1);
_57 = _51.0.1 >> _54.fld1.0;
_77.1 = _23.2;
_58 = _34.1;
_24.fld1.1 = _75.fld1.1;
SetDiscriminant(_53.fld1, 2);
(*_46) = _35.fld1.fld5 << _47;
_24.fld1.1.1.1 = [_3,_3,_25,_3,_41,_41,_25];
_34 = (_24.fld1.0, _43, _44, _54.fld1.0);
_35.fld1.fld3 = _57;
_67.0 = _24.fld1.1.1.0 & _47;
_72 = [_51.0.1,_57];
_33 = Adt52::Variant1 { fld0: _17,fld1: _69,fld2: _52.0,fld3: _75.fld1.1.0 };
_24.fld1.1.1.0 = _67.0;
Goto(bb36)
}
bb36 = {
_72 = _60;
_23.2 = (_67.0, _75.fld1.1.1.1);
_82 = core::ptr::addr_of!(_52.1);
_51 = (_23,);
_56 = (_36.0, _43, _72);
_51.0.2.1 = _40.1;
_51.0.0 = _12;
_87 = (_51.0,);
_18.1 = Field::<i128>(Variant(_33, 1), 2) as i8;
_81 = (*_46) > (*_46);
_29 = _52.0 > _52.0;
_35.fld1.fld1.2 = _15;
_47 = (*_46) as i16;
_53.fld3 = _57 as i8;
_49 = _32;
_70 = _35.fld1.fld1.2;
_75.fld1.1.1 = _10;
_69 = _38 as f32;
_65 = Adt54::Variant1 { fld0: _29 };
_75.fld1.2 = core::ptr::addr_of_mut!(_19);
Call(_48 = core::intrinsics::bswap(_17.0), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
place!(Field::<i16>(Variant(_53.fld1, 2), 4)) = _67.0 << _35.fld1.fld3;
_36.0 = [_57,_35.fld1.fld3];
place!(Field::<(u8,)>(Variant(_33, 1), 0)) = (_48,);
place!(Field::<i64>(Variant(_53.fld1, 2), 3)) = _51.0.0 << _51.0.1;
place!(Field::<i128>(Variant(_33, 1), 2)) = !_54.fld1.0;
_34.3 = !Field::<i128>(Variant(_33, 1), 2);
_53.fld2 = Adt55::Variant0 { fld0: _56.2,fld1: _1,fld2: _24.fld1.2,fld3: _35.fld1.fld4,fld4: _87.0.0,fld5: _54.fld1.0 };
Goto(bb38)
}
bb38 = {
_62.0 = Field::<bool>(Variant(_65, 1), 0) ^ Field::<bool>(Variant(_65, 1), 0);
_42 = _53.fld3 * _53.fld3;
_59 = Field::<i16>(Variant(_53.fld1, 2), 4) as f64;
_84 = [_38,_38,_38,_38];
_40.0 = _69 as i16;
_43 = _56.1 * _34.1;
Goto(bb39)
}
bb39 = {
_42 = _56.1 as i8;
_74 = (*_46) as f64;
_34.2 = [_49,_18.0,_32,_35.fld1.fld1.2,_9,_73.0,_32,_73.0];
_87.0.0 = _51.0.0;
_48 = _59 as u8;
_10.1 = [_3,_39,_3,_3,_3,_41,_3];
_40.0 = _24.fld1.1.1.0;
_53.fld1 = Adt52::Variant1 { fld0: Field::<(u8,)>(Variant(_33, 1), 0),fld1: _69,fld2: Field::<i128>(Variant(_53.fld2, 0), 5),fld3: _45 };
_14 = -_21;
_24.fld0 = [_57,_35.fld1.fld3];
_86.0 = (_23.0, _57, _51.0.2);
_24.fld1.2 = core::ptr::addr_of_mut!(_19);
_86.0 = _23;
_58 = _56.1;
_35.fld0 = Move(_65);
_60 = [_35.fld1.fld3,_35.fld1.fld3];
_87.0.2 = _86.0.2;
Goto(bb40)
}
bb40 = {
_76 = _59 * _59;
_34 = (_35.fld1.fld1.0, _43, _44, _54.fld1.0);
_18.0 = _70;
_35.fld1.fld1 = (_24.fld1.0, _47, _49);
_85 = Adt58::Variant1 { fld0: _24.fld1 };
_54.fld1 = (Field::<i128>(Variant(_53.fld1, 1), 2), (*_82), _52.2, _82);
_24.fld1.0 = [_6,_1];
_20 = _54.fld0;
_77 = _24.fld1.1;
_21 = -_77.0;
_23.1 = !_86.0.1;
_24.fld1.1.1.0 = -_67.0;
_44 = _34.2;
_68 = _3;
_92 = _54.fld1.2;
Goto(bb41)
}
bb41 = {
_56.0 = [_57,_57];
_86.0 = _87.0;
_53.fld3 = _18.1;
_81 = _29;
SetDiscriminant(_33, 3);
Goto(bb42)
}
bb42 = {
_67.1 = _87.0.2.1;
_84 = [_38,_38,_38,_38];
_52.0 = _48 as i128;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)).1 = _86.0.2.1;
_61 = [Field::<u64>(Variant(_53.fld2, 0), 1),Field::<u64>(Variant(_53.fld2, 0), 1),_1];
_24.fld1.1.1 = (_86.0.2.0, _86.0.2.1);
_77.1 = (_71.0, _10.1);
_40 = (_77.1.0, _10.1);
_52.2 = [_8];
_79.0 = !_62.0;
_81 = _62.0;
_77.1.1 = [_41,_3,_25,_68,_68,_25,_26];
_99 = _48 + _48;
_17.0 = _99 & _99;
SetDiscriminant(_35.fld0, 2);
_54.fld1.1 = [_79.0,_81,_62.0,_81,_62.0,_29,_29,_29];
_97 = (_77.0, _51.0.2);
_5 = _35.fld1.fld1.2;
_76 = _59 + Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_85, 1), 0).1.0;
_7.1 = -_53.fld3;
Goto(bb43)
}
bb43 = {
_54.fld1.1 = [_79.0,_81,_79.0,_62.0,_29,_81,_62.0,_29];
_54.fld1.1 = [_81,_81,_79.0,_79.0,_81,_62.0,_29,_62.0];
_52 = (Field::<i128>(Variant(_53.fld1, 1), 2), _54.fld1.1, _92, _54.fld1.3);
_67.1 = _24.fld1.1.1.1;
_96 = !Field::<u64>(Variant(_53.fld2, 0), 1);
place!(Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1)).1 = _57 << _54.fld1.0;
_21 = -_76;
Goto(bb44)
}
bb44 = {
_23.2.1 = [_3,_68,_41,_41,_39,_26,_68];
SetDiscriminant(_53.fld2, 0);
SetDiscriminant(_53.fld1, 2);
_36.0 = [_35.fld1.fld3,_35.fld1.fld3];
_68 = _25 ^ _25;
_11 = _61;
_55 = (_99,);
_54.fld1.2 = _52.2;
_10.0 = _76 as i16;
_54.fld1.3 = core::ptr::addr_of!((*_82));
_24.fld1.0 = [_1,_96];
_20 = [_58,_43,_58,_34.1,_36.1,_34.1,_34.1,_43];
_75.fld1.1.0 = _21 * _24.fld1.1.0;
_37 = _41 as f32;
_24.fld0 = [_23.1,_57];
_86.0.0 = -_87.0.0;
SetDiscriminant(_85, 1);
Goto(bb45)
}
bb45 = {
place!(Field::<*mut char>(Variant(_35.fld0, 2), 0)) = core::ptr::addr_of_mut!(_98);
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_85, 1), 0)).1.1.1 = [_25,_25,_68,_41,_41,_25,_26];
_89 = !_3;
place!(Field::<*mut u128>(Variant(_53.fld2, 0), 2)) = _75.fld1.2;
_95 = -_63;
_23.0 = _12 - _87.0.0;
_94 = _13 + _95;
place!(Field::<i64>(Variant(_53.fld2, 0), 4)) = _8 as i64;
_87.0.2.1 = [_39,_68,_68,_3,_3,_3,_3];
_75.fld1.1.1.0 = -_10.0;
_103 = core::ptr::addr_of!((*_46));
_95 = _97.1.0 as f32;
_51.0.2 = (_23.2.0, Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_85, 1), 0).1.1.1);
Goto(bb46)
}
bb46 = {
place!(Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1)).2 = (_75.fld1.1.1.0, _97.1.1);
_10.0 = Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).2.0;
_50.0 = _29 ^ _81;
_92 = _52.2;
(*_82) = [_62.0,_29,_62.0,_81,_62.0,_50.0,_62.0,_29];
_98 = _73.0;
_34.3 = _41 as i128;
_14 = _96 as f64;
_51.0.1 = (*_103) as usize;
place!(Field::<*const i64>(Variant(_53.fld2, 0), 3)) = _35.fld1.fld4;
_85 = Adt58::Variant3 { fld0: (*_46),fld1: _38 };
_75.fld0 = [Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).1,_87.0.1];
_10 = Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).2;
place!(Field::<i64>(Variant(_53.fld1, 2), 3)) = Field::<i64>(Variant(_53.fld2, 0), 4) - _23.0;
_86.0.1 = _23.1;
_35.fld1.fld3 = Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).1 & _57;
_67 = (_75.fld1.1.1.0, Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).2.1);
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)).0 = _37 as i16;
Goto(bb47)
}
bb47 = {
_7 = (_35.fld1.fld1.2, _42);
place!(Field::<u32>(Variant(_53.fld1, 2), 2)) = _38;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)).1 = [_3,_25,_3,_41,_68,_89,_89];
_10.1 = _24.fld1.1.1.1;
_90 = [_34.1,_58,_34.1,_34.1,_43,_56.1,_58,_34.1];
_34 = (_24.fld1.0, _43, _44, _52.0);
_110 = _81 & _50.0;
_93 = !_39;
_10 = (_75.fld1.1.1.0, Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).2.1);
_82 = _54.fld1.3;
place!(Field::<i64>(Variant(_53.fld1, 2), 3)) = _87.0.0;
_104.1 = _87.0.2.1;
_36 = _56;
place!(Field::<*const [u64; 3]>(Variant(_53.fld1, 2), 0)) = core::ptr::addr_of!(_108.0);
_86.0.2.1 = _71.1;
_77.1 = (_47, Field::<(i16, [isize; 7])>(Variant(_33, 3), 0).1);
_48 = _55.0;
_35.fld1.fld3 = Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).1 << _55.0;
_28 = _86.0.1 | _87.0.1;
place!(Field::<i64>(Variant(_53.fld1, 2), 3)) = _51.0.0 >> _1;
_97.1 = _10;
_91 = _68 << _55.0;
_75.fld1.1 = (_59, _67);
_81 = _50.0;
_75.fld1 = (_31, _97, _24.fld1.2);
_101 = _7.1 as u32;
_38 = _17.0 as u32;
_109 = _91;
Goto(bb48)
}
bb48 = {
_52.1 = [_110,_62.0,_29,_81,_81,_50.0,_50.0,_62.0];
place!(Field::<i128>(Variant(_53.fld2, 0), 5)) = _52.0 & _54.fld1.0;
_102 = _49;
_106 = Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).2.0;
place!(Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1)).2.1 = _97.1.1;
place!(Field::<*const i64>(Variant(_53.fld2, 0), 3)) = core::ptr::addr_of!(place!(Field::<i64>(Variant(_53.fld2, 0), 4)));
place!(Field::<[usize; 2]>(Variant(_53.fld2, 0), 0)) = [_86.0.1,_86.0.1];
Goto(bb49)
}
bb49 = {
_70 = _49;
place!(Field::<u32>(Variant(_85, 3), 1)) = !_38;
_75.fld1.1 = _97;
SetDiscriminant(_85, 3);
_44 = [_73.0,_35.fld1.fld1.2,_98,_18.0,_102,_49,_70,_102];
SetDiscriminant(_33, 3);
_111 = (_55.0,);
_99 = _48 - _111.0;
_27 = !_79.0;
_97.0 = _59 + _21;
_2 = _81 >= _81;
_75.fld1.1.1.1 = [_91,_109,_3,_91,_39,_91,_109];
_54.fld1.0 = Field::<i128>(Variant(_53.fld2, 0), 5);
_23.2.1 = [_91,_91,_109,_41,_25,_68,_3];
_46 = _103;
_44 = [_7.0,_70,_98,_9,_49,_15,_7.0,_5];
_8 = _30 & _35.fld1.fld5;
_87 = (_23,);
_54.fld1 = (_34.3, _52.1, _52.2, _82);
_75.fld1.1.1.1 = _87.0.2.1;
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)) = (Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).2.0, _87.0.2.1);
Goto(bb50)
}
bb50 = {
_56.2 = [_51.0.1,_35.fld1.fld3];
SetDiscriminant(_33, 3);
place!(Field::<(i16, [isize; 7])>(Variant(_33, 3), 0)) = (_75.fld1.1.1.0, _35.fld1.fld2);
_8 = -_30;
_81 = !_50.0;
_98 = _102;
place!(Field::<i32>(Variant(_85, 3), 0)) = -(*_103);
_113 = core::ptr::addr_of!(_23.0);
_7 = (_5, _18.1);
_104 = (_10.0, _87.0.2.1);
_69 = (*_113) as f32;
_35.fld1.fld1.1 = -_104.0;
_75.fld1.1.0 = _21 - _45;
_35.fld1.fld3 = _87.0.1;
_70 = _49;
_111 = _55;
_61 = [_6,_1,_1];
_12 = !(*_113);
_13 = _69;
_115 = (_24.fld1.0, _67.0, _70);
_7 = _18;
_54.fld1.3 = _52.3;
SetDiscriminant(_33, 0);
place!(Field::<(bool,)>(Variant(_33, 0), 3)).0 = _50.0;
Call((*_113) = core::intrinsics::transmute(Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).1), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
_103 = _46;
_51.0.1 = !Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).1;
_103 = core::ptr::addr_of!(_30);
_110 = _81;
_91 = _109 | _109;
_23.1 = !Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1).1;
_21 = _75.fld1.1.0 - _24.fld1.1.0;
Call(_18.1 = core::intrinsics::bswap(_73.1), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
place!(Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4)).1.1 = _28;
place!(Field::<(u8,)>(Variant(_53.fld1, 2), 1)) = (_111.0,);
_75.fld1.0 = [_6,_96];
_115.2 = _32;
place!(Field::<(i64, usize, (i16, [isize; 7]))>(Variant(_35.fld0, 2), 1)).1 = Field::<([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128)>(Variant(_33, 0), 4).1.1;
_11 = _61;
_79 = (Field::<(bool,)>(Variant(_33, 0), 3).0,);
_24 = Adt65 { fld0: _75.fld0,fld1: _75.fld1 };
place!(Field::<i32>(Variant(_85, 3), 0)) = _30 * (*_46);
_117 = Adt50::Variant1 { fld0: _79.0,fld1: _86.0.2,fld2: _35.fld1.fld4,fld3: _11,fld4: _97,fld5: (*_46),fld6: _75.fld1 };
_16 = _50.0 | Field::<(bool,)>(Variant(_33, 0), 3).0;
_74 = Field::<(f64, (i16, [isize; 7]))>(Variant(_117, 1), 4).0 * _97.0;
(*_82) = _54.fld1.1;
_49 = _98;
place!(Field::<*mut u128>(Variant(_53.fld2, 0), 2)) = core::ptr::addr_of_mut!(_19);
_85 = Adt58::Variant1 { fld0: _24.fld1 };
_78 = [_53.fld3,_42,_7.1,_42,_7.1,_42,_18.1,_18.1];
Goto(bb53)
}
bb53 = {
_52 = (Field::<i128>(Variant(_53.fld2, 0), 5), _54.fld1.1, _54.fld1.2, _54.fld1.3);
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_85, 1), 0)).1.1 = _23.2;
_2 = Field::<bool>(Variant(_117, 1), 0);
Goto(bb54)
}
bb54 = {
_119 = _35.fld1.fld3 ^ _51.0.1;
_66 = [_91,_109,_109,_91,_91,_109,_109];
RET = Move(_117);
_54.fld1 = _52;
_23.2.1 = [_109,_91,_91,_109,_109,_91,_91];
_131 = _23.0;
SetDiscriminant(_85, 0);
_112 = _54.fld0;
_54.fld1.2 = [(*_46)];
place!(Field::<(i16, [isize; 7])>(Variant(RET, 1), 1)) = (_97.1.0, _24.fld1.1.1.1);
Goto(bb55)
}
bb55 = {
Call(_133 = dump_var(1_usize, 86_usize, Move(_86), 48_usize, Move(_48), 44_usize, Move(_44), 40_usize, Move(_40)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_133 = dump_var(1_usize, 111_usize, Move(_111), 12_usize, Move(_12), 4_usize, Move(_4), 55_usize, Move(_55)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_133 = dump_var(1_usize, 104_usize, Move(_104), 9_usize, Move(_9), 39_usize, Move(_39), 99_usize, Move(_99)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_133 = dump_var(1_usize, 81_usize, Move(_81), 78_usize, Move(_78), 115_usize, Move(_115), 66_usize, Move(_66)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_133 = dump_var(1_usize, 20_usize, Move(_20), 131_usize, Move(_131), 73_usize, Move(_73), 18_usize, Move(_18)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_133 = dump_var(1_usize, 93_usize, Move(_93), 92_usize, Move(_92), 43_usize, Move(_43), 106_usize, Move(_106)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Call(_133 = dump_var(1_usize, 72_usize, Move(_72), 3_usize, Move(_3), 15_usize, Move(_15), 56_usize, Move(_56)), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Call(_133 = dump_var(1_usize, 5_usize, Move(_5), 89_usize, Move(_89), 8_usize, Move(_8), 29_usize, Move(_29)), ReturnTo(bb63), UnwindUnreachable())
}
bb63 = {
Call(_133 = dump_var(1_usize, 32_usize, Move(_32), 102_usize, Move(_102), 84_usize, Move(_84), 6_usize, Move(_6)), ReturnTo(bb64), UnwindUnreachable())
}
bb64 = {
Call(_133 = dump_var(1_usize, 91_usize, Move(_91), 7_usize, Move(_7), 60_usize, Move(_60), 36_usize, Move(_36)), ReturnTo(bb65), UnwindUnreachable())
}
bb65 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: f64,mut _2: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128)) -> Adt49 {
mir! {
type RET = Adt49;
let _3: (bool,);
let _4: u64;
let _5: (*const [bool; 8], [char; 8]);
let _6: *mut u128;
let _7: usize;
let _8: ([u64; 3],);
let _9: bool;
let _10: bool;
let _11: [bool; 8];
let _12: [i8; 8];
let _13: bool;
let _14: u128;
let _15: [usize; 2];
let _16: isize;
let _17: u16;
let _18: *const [bool; 8];
let _19: Adt50;
let _20: (i64, usize, (i16, [isize; 7]));
let _21: i16;
let _22: Adt51;
let _23: *const i32;
let _24: ([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128);
let _25: f64;
let _26: u16;
let _27: isize;
let _28: Adt55;
let _29: usize;
let _30: i8;
let _31: Adt50;
let _32: u32;
let _33: [u16; 8];
let _34: usize;
let _35: [u32; 4];
let _36: u8;
let _37: *const [u64; 3];
let _38: ([u64; 3],);
let _39: f64;
let _40: ();
let _41: ();
{
_2.1.1.0 = false as i16;
_1 = _2.1.0;
_2.0 = [4785117548366264555_u64,13473958674815145861_u64];
RET = Adt49::Variant1 { fld0: _2.0,fld1: _2.1.1.1 };
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = [9223372036854775807_isize,(-122_isize),(-92_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_2.1.0 = 9223372036854775807_isize as f64;
_2.1.0 = _1;
_2.1.1 = ((-32239_i16), Field::<[isize; 7]>(Variant(RET, 1), 1));
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = _2.1.1.1;
_2.1.1.0 = (-21161_i16) | 27775_i16;
place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = _2.0;
_3 = (false,);
_2.1.1.0 = 7401_u16 as i16;
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = [77_isize,48_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),52_isize,9223372036854775807_isize,(-101_isize)];
_2.1.1.0 = -(-9852_i16);
_4 = !2185999928257557065_u64;
_1 = _2.1.0 - _2.1.0;
_4 = !67451971709622564_u64;
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = _2.1.1.1;
_2.1.0 = -_1;
Call(place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = fn3(_2.1, _2, _2, _2, _2, Field::<[isize; 7]>(Variant(RET, 1), 1), Field::<[isize; 7]>(Variant(RET, 1), 1), _1, _2.1.1.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.1.1.1 = [28_isize,(-9223372036854775808_isize),3_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-71_isize)];
place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = [_4,_4];
_2.1.1.1 = Field::<[isize; 7]>(Variant(RET, 1), 1);
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = [(-104_isize),59_isize,9223372036854775807_isize,9223372036854775807_isize,61_isize,(-9223372036854775808_isize),(-94_isize)];
place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = [_4,_4];
_3 = (true,);
_3 = (false,);
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = [(-9223372036854775808_isize),66_isize,73_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_2.1.1 = ((-12370_i16), Field::<[isize; 7]>(Variant(RET, 1), 1));
_6 = _2.2;
_2.2 = _6;
place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = [_4,_4];
_1 = -_2.1.0;
_2.0 = [_4,_4];
RET = Adt49::Variant1 { fld0: _2.0,fld1: _2.1.1.1 };
_2.0 = [_4,_4];
_2.0 = [_4,_4];
_3.0 = !true;
SetDiscriminant(RET, 3);
_1 = _2.1.0;
place!(Field::<[u64; 3]>(Variant(RET, 3), 3)) = [_4,_4,_4];
_7 = 1_usize * 2_usize;
_8 = (Field::<[u64; 3]>(Variant(RET, 3), 3),);
_7 = !1_usize;
match _2.1.1.0 {
0 => bb2,
340282366920938463463374607431768199086 => bb4,
_ => bb3
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
_8.0 = [_4,_4,_4];
place!(Field::<usize>(Variant(RET, 3), 1)) = !_7;
_7 = Field::<usize>(Variant(RET, 3), 1) - Field::<usize>(Variant(RET, 3), 1);
_3 = (true,);
_2.2 = _6;
(*_6) = 49_isize as u128;
_2.1.1.0 = 28695_i16 << (*_6);
place!(Field::<i128>(Variant(RET, 3), 2)) = (-152005413763628705486826542550177461469_i128);
_6 = core::ptr::addr_of_mut!((*_6));
_3 = (true,);
_10 = _3.0;
place!(Field::<i128>(Variant(RET, 3), 2)) = (-34209195697954002401500595501195764886_i128);
_20.0 = !4178972454579531002_i64;
_20.2 = (_2.1.1.0, _2.1.1.1);
_20.1 = _7 & _7;
_10 = _3.0 | _3.0;
(*_6) = _20.2.0 as u128;
Goto(bb5)
}
bb5 = {
place!(Field::<u8>(Variant(RET, 3), 0)) = 38_u8;
place!(Field::<usize>(Variant(RET, 3), 1)) = 3796204706_u32 as usize;
_7 = _20.1;
_2.1.0 = _1;
_12 = [52_i8,(-84_i8),38_i8,(-108_i8),(-64_i8),(-53_i8),0_i8,(-50_i8)];
_2.1.0 = -_1;
_20.0 = !(-6260738116805780156_i64);
place!(Field::<i128>(Variant(RET, 3), 2)) = _4 as i128;
_11 = [_3.0,_10,_10,_10,_3.0,_10,_10,_10];
SetDiscriminant(RET, 3);
place!(Field::<usize>(Variant(RET, 3), 1)) = _20.1;
_18 = core::ptr::addr_of!(_11);
_24.1.0 = -_20.0;
_2.0 = [_4,_4];
(*_6) = 210980174437492963721824913964573651458_u128;
_24.1 = (_20.0, _7, _2.1.1);
_3.0 = _24.1.1 == Field::<usize>(Variant(RET, 3), 1);
_20.2 = (_24.1.2.0, _2.1.1.1);
_15 = [_24.1.1,Field::<usize>(Variant(RET, 3), 1)];
(*_18) = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_3.0,_10];
_24.2 = '\u{e9fd9}' as u64;
_24 = (_12, _20, _4, (-114043201419465600302082035277170616888_i128));
_11 = [_3.0,_3.0,_3.0,_10,_3.0,_3.0,_3.0,_3.0];
_21 = -_2.1.1.0;
match (*_6) {
0 => bb2,
1 => bb6,
2 => bb7,
3 => bb8,
210980174437492963721824913964573651458 => bb10,
_ => bb9
}
}
bb6 = {
_8.0 = [_4,_4,_4];
place!(Field::<usize>(Variant(RET, 3), 1)) = !_7;
_7 = Field::<usize>(Variant(RET, 3), 1) - Field::<usize>(Variant(RET, 3), 1);
_3 = (true,);
_2.2 = _6;
(*_6) = 49_isize as u128;
_2.1.1.0 = 28695_i16 << (*_6);
place!(Field::<i128>(Variant(RET, 3), 2)) = (-152005413763628705486826542550177461469_i128);
_6 = core::ptr::addr_of_mut!((*_6));
_3 = (true,);
_10 = _3.0;
place!(Field::<i128>(Variant(RET, 3), 2)) = (-34209195697954002401500595501195764886_i128);
_20.0 = !4178972454579531002_i64;
_20.2 = (_2.1.1.0, _2.1.1.1);
_20.1 = _7 & _7;
_10 = _3.0 | _3.0;
(*_6) = _20.2.0 as u128;
Goto(bb5)
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_2.1.1.1 = [28_isize,(-9223372036854775808_isize),3_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-71_isize)];
place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = [_4,_4];
_2.1.1.1 = Field::<[isize; 7]>(Variant(RET, 1), 1);
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = [(-104_isize),59_isize,9223372036854775807_isize,9223372036854775807_isize,61_isize,(-9223372036854775808_isize),(-94_isize)];
place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = [_4,_4];
_3 = (true,);
_3 = (false,);
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = [(-9223372036854775808_isize),66_isize,73_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_2.1.1 = ((-12370_i16), Field::<[isize; 7]>(Variant(RET, 1), 1));
_6 = _2.2;
_2.2 = _6;
place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = [_4,_4];
_1 = -_2.1.0;
_2.0 = [_4,_4];
RET = Adt49::Variant1 { fld0: _2.0,fld1: _2.1.1.1 };
_2.0 = [_4,_4];
_2.0 = [_4,_4];
_3.0 = !true;
SetDiscriminant(RET, 3);
_1 = _2.1.0;
place!(Field::<[u64; 3]>(Variant(RET, 3), 3)) = [_4,_4,_4];
_7 = 1_usize * 2_usize;
_8 = (Field::<[u64; 3]>(Variant(RET, 3), 3),);
_7 = !1_usize;
match _2.1.1.0 {
0 => bb2,
340282366920938463463374607431768199086 => bb4,
_ => bb3
}
}
bb10 = {
_24.1 = _20;
place!(Field::<u8>(Variant(RET, 3), 0)) = _10 as u8;
_20.0 = '\u{df9ac}' as i64;
_2.1.0 = _1 - _1;
_20 = _24.1;
_20.2.1 = [(-74_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
place!(Field::<usize>(Variant(RET, 3), 1)) = !_7;
_21 = !_2.1.1.0;
_20.0 = _24.1.0 ^ _24.1.0;
place!(Field::<i128>(Variant(RET, 3), 2)) = _24.3 >> _4;
match (*_6) {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb7,
210980174437492963721824913964573651458 => bb11,
_ => bb6
}
}
bb11 = {
_3.0 = _10;
_20 = _24.1;
_24.1.2.0 = _2.1.1.0 + _21;
_14 = (*_6);
_24.2 = _4;
_24.0 = [(-4_i8),(-41_i8),53_i8,(-61_i8),(-112_i8),14_i8,7_i8,83_i8];
place!(Field::<u8>(Variant(RET, 3), 0)) = 49855_u16 as u8;
_11 = [_3.0,_10,_3.0,_3.0,_10,_10,_10,_10];
_24 = (_12, _20, _4, Field::<i128>(Variant(RET, 3), 2));
_2.1.1.0 = _24.1.2.0 + _20.2.0;
_5.0 = core::ptr::addr_of!(_11);
_27 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_27 = (-9223372036854775808_isize) + (-17_isize);
_5.1 = ['\u{ba572}','\u{7da0e}','\u{56df7}','\u{3b01e}','\u{10ba77}','\u{5849d}','\u{8397b}','\u{db689}'];
_16 = _27;
_2.1.0 = _1;
_27 = _16;
_17 = _2.1.1.0 as u16;
_26 = !_17;
_20.2.0 = 73_i8 as i16;
(*_18) = [_3.0,_10,_10,_10,_10,_10,_10,_3.0];
_5.1 = ['\u{1fc4d}','\u{9d8d4}','\u{6acf2}','\u{fadc3}','\u{93b89}','\u{a99a6}','\u{24449}','\u{c2fc6}'];
place!(Field::<i128>(Variant(RET, 3), 2)) = _24.1.0 as i128;
_24.1.1 = !Field::<usize>(Variant(RET, 3), 1);
place!(Field::<[u64; 3]>(Variant(RET, 3), 3)) = [_4,_24.2,_24.2];
_30 = 1766084089_i32 as i8;
_2.1.1.1 = _24.1.2.1;
Goto(bb12)
}
bb12 = {
(*_18) = [_10,_10,_10,_10,_10,_10,_3.0,_3.0];
place!(Field::<[u64; 3]>(Variant(RET, 3), 3)) = [_4,_4,_24.2];
_17 = _4 as u16;
_32 = Field::<u8>(Variant(RET, 3), 0) as u32;
_2.1.1.1 = [_16,_27,_27,_16,_16,_16,_27];
_20.2.1 = [_16,_16,_27,_27,_27,_27,_27];
place!(Field::<i128>(Variant(RET, 3), 2)) = _24.3 & _24.3;
_34 = _20.1;
_24.1 = _20;
SetDiscriminant(RET, 1);
_13 = _24.2 < _4;
_11 = [_10,_3.0,_3.0,_13,_10,_10,_3.0,_10];
place!(Field::<[isize; 7]>(Variant(RET, 1), 1)) = [_16,_16,_27,_16,_27,_16,_16];
_2.1.1.0 = _20.2.0 * _21;
_30 = 120_i8 ^ 107_i8;
_2.0 = [_24.2,_4];
place!(Field::<[u64; 2]>(Variant(RET, 1), 0)) = _2.0;
_20.1 = _24.1.1 * _7;
(*_6) = !_14;
_29 = _26 as usize;
_2.1.1.1 = [_27,_27,_16,_27,_27,_27,_27];
(*_6) = 230_u8 as u128;
_25 = -_1;
_20 = _24.1;
Goto(bb13)
}
bb13 = {
Call(_40 = dump_var(2_usize, 8_usize, Move(_8), 24_usize, Move(_24), 11_usize, Move(_11), 34_usize, Move(_34)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_40 = dump_var(2_usize, 27_usize, Move(_27), 14_usize, Move(_14), 32_usize, Move(_32), 29_usize, Move(_29)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_40 = dump_var(2_usize, 30_usize, Move(_30), 15_usize, Move(_15), 41_usize, _41, 41_usize, _41), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: (f64, (i16, [isize; 7])),mut _2: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _3: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _4: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _5: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _6: [isize; 7],mut _7: [isize; 7],mut _8: f64,mut _9: [isize; 7]) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _10: [char; 8];
let _11: i8;
let _12: Adt49;
let _13: f64;
let _14: Adt51;
let _15: (u8,);
let _16: u8;
let _17: isize;
let _18: [isize; 7];
let _19: f64;
let _20: [i8; 8];
let _21: (f64, (i16, [isize; 7]));
let _22: u64;
let _23: u32;
let _24: isize;
let _25: i128;
let _26: [isize; 7];
let _27: [u64; 3];
let _28: u32;
let _29: ([u64; 3],);
let _30: ((i64, usize, (i16, [isize; 7])),);
let _31: (char, i8);
let _32: Adt55;
let _33: ();
let _34: ();
{
_3.1.0 = _4.1.0;
_5.1.1 = _2.1.1;
_1.1.0 = -_2.1.1.0;
_2.1.1 = _3.1.1;
_4.1.0 = -_5.1.0;
RET = _3.0;
_1.1 = _3.1.1;
_4.2 = _3.2;
_5.2 = _2.2;
_2.1 = (_8, _4.1.1);
_3.1.1.1 = [9223372036854775807_isize,(-65_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = (_4.0, _2.1, _4.2);
_2.2 = _3.2;
_2.1.1.1 = _9;
_5.1.1.1 = _6;
_10 = ['\u{6bddc}','\u{108af2}','\u{b5269}','\u{11aa1}','\u{a3102}','\u{3a238}','\u{104139}','\u{3bdd0}'];
_8 = _3.1.0 * _3.1.0;
_5.2 = _4.2;
_5.1 = (_8, _2.1.1);
_3.1.1.0 = _5.1.1.0;
Call(_5.2 = fn4(_5.1, _2.1.0, _3, _2, _5.1.0, _2.1.0, _3.1.1, _1.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.1.1.0 = -_2.1.1.0;
_2.1 = _3.1;
_3.1.1.1 = _9;
_3.1 = _5.1;
_4 = (_3.0, _1, _2.2);
_5.1.1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,108_isize,9223372036854775807_isize,(-35_isize),(-63_isize)];
_2 = _5;
_5 = (_4.0, _3.1, _2.2);
_8 = 743390415684585731_usize as f64;
_4.0 = [11286022013333860668_u64,12145923705881440526_u64];
_5.1.1.0 = _1.1.0 - _3.1.1.0;
_2 = (_4.0, _5.1, _3.2);
_5.1.1.0 = _2.1.1.0;
_13 = _3.1.0 + _5.1.0;
_1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-19_isize),9223372036854775807_isize,(-62_isize),(-42_isize)];
_3.1.1 = (_2.1.1.0, _2.1.1.1);
_3.1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-104_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),41_isize,(-9223372036854775808_isize)];
_3.0 = [711835678316339018_u64,12780743314501483248_u64];
_4.1.1.1 = _9;
_1 = (_13, _3.1.1);
_13 = 45_isize as f64;
_2.1.0 = _13 - _1.0;
_4.1 = (_1.0, _1.1);
_3.2 = _4.2;
_15 = (33_u8,);
Goto(bb2)
}
bb2 = {
_2.1 = _1;
_5.1 = (_4.1.0, _1.1);
_1.0 = _5.1.0;
_18 = _6;
_5.1.1.0 = _2.1.0 as i16;
_2.1.0 = -_1.0;
_5 = (RET, _4.1, _4.2);
_5.1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5.1.1 = _2.1.1;
_19 = -_5.1.0;
RET = [11986906619422290101_u64,5039913960520121876_u64];
_1.1.0 = '\u{5fd8}' as i16;
_5 = (RET, _4.1, _2.2);
_4.1.1 = (_2.1.1.0, _9);
_16 = _15.0 / _15.0;
_1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),103_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
match _15.0 {
33 => bb3,
_ => bb1
}
}
bb3 = {
_5.1.0 = _4.1.0;
_3.1.1.1 = [(-9223372036854775808_isize),110_isize,(-74_isize),99_isize,(-68_isize),9223372036854775807_isize,32_isize];
_5 = (_3.0, _4.1, _4.2);
_9 = [(-7_isize),(-85_isize),(-119_isize),9223372036854775807_isize,9223372036854775807_isize,86_isize,(-12_isize)];
_5.2 = _3.2;
_1 = _4.1;
Call(_2.0 = fn5(_5.1.0, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_18 = [(-9223372036854775808_isize),46_isize,(-9223372036854775808_isize),(-68_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5 = _2;
match _15.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
33 => bb8,
_ => bb7
}
}
bb5 = {
_5.1.0 = _4.1.0;
_3.1.1.1 = [(-9223372036854775808_isize),110_isize,(-74_isize),99_isize,(-68_isize),9223372036854775807_isize,32_isize];
_5 = (_3.0, _4.1, _4.2);
_9 = [(-7_isize),(-85_isize),(-119_isize),9223372036854775807_isize,9223372036854775807_isize,86_isize,(-12_isize)];
_5.2 = _3.2;
_1 = _4.1;
Call(_2.0 = fn5(_5.1.0, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_2.1 = _1;
_5.1 = (_4.1.0, _1.1);
_1.0 = _5.1.0;
_18 = _6;
_5.1.1.0 = _2.1.0 as i16;
_2.1.0 = -_1.0;
_5 = (RET, _4.1, _4.2);
_5.1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5.1.1 = _2.1.1;
_19 = -_5.1.0;
RET = [11986906619422290101_u64,5039913960520121876_u64];
_1.1.0 = '\u{5fd8}' as i16;
_5 = (RET, _4.1, _2.2);
_4.1.1 = (_2.1.1.0, _9);
_16 = _15.0 / _15.0;
_1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),103_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
match _15.0 {
33 => bb3,
_ => bb1
}
}
bb7 = {
_4.1.1.0 = -_2.1.1.0;
_2.1 = _3.1;
_3.1.1.1 = _9;
_3.1 = _5.1;
_4 = (_3.0, _1, _2.2);
_5.1.1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,108_isize,9223372036854775807_isize,(-35_isize),(-63_isize)];
_2 = _5;
_5 = (_4.0, _3.1, _2.2);
_8 = 743390415684585731_usize as f64;
_4.0 = [11286022013333860668_u64,12145923705881440526_u64];
_5.1.1.0 = _1.1.0 - _3.1.1.0;
_2 = (_4.0, _5.1, _3.2);
_5.1.1.0 = _2.1.1.0;
_13 = _3.1.0 + _5.1.0;
_1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-19_isize),9223372036854775807_isize,(-62_isize),(-42_isize)];
_3.1.1 = (_2.1.1.0, _2.1.1.1);
_3.1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-104_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),41_isize,(-9223372036854775808_isize)];
_3.0 = [711835678316339018_u64,12780743314501483248_u64];
_4.1.1.1 = _9;
_1 = (_13, _3.1.1);
_13 = 45_isize as f64;
_2.1.0 = _13 - _1.0;
_4.1 = (_1.0, _1.1);
_3.2 = _4.2;
_15 = (33_u8,);
Goto(bb2)
}
bb8 = {
_5.1.1.0 = _2.1.1.0 - _1.1.0;
RET = [13872038017460131348_u64,2233976069953011237_u64];
_19 = 5142434616783987118_usize as f64;
_5.2 = _3.2;
_8 = _4.1.0;
_15.0 = !_16;
_3.1.1.0 = -_5.1.1.0;
_3.1.1 = (_5.1.1.0, _4.1.1.1);
_5.1 = (_8, _1.1);
_5.1.1.1 = [116_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-88_isize),61_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4.1 = (_8, _1.1);
_21.1.1 = [22_isize,(-9223372036854775808_isize),9223372036854775807_isize,0_isize,32_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.1.1.1 = _2.1.1.1;
_5 = (_2.0, _4.1, _3.2);
_3.0 = [15631545228616455983_u64,14919355862026633658_u64];
_16 = !_15.0;
_4.0 = [7319533868347705127_u64,476164590246733074_u64];
_4.1 = (_5.1.0, _1.1);
_3 = (RET, _2.1, _5.2);
_21 = _3.1;
_21.1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-86_isize),26_isize,9223372036854775807_isize];
_2.1 = _5.1;
_24 = 9223372036854775807_isize | 9223372036854775807_isize;
_5.2 = _4.2;
_3.2 = _2.2;
_21.1 = (_1.1.0, _6);
Goto(bb9)
}
bb9 = {
_5 = (RET, _21, _2.2);
_7 = _18;
_23 = '\u{101981}' as u32;
_10 = ['\u{7b298}','\u{58495}','\u{b008b}','\u{a1f38}','\u{754fa}','\u{884ab}','\u{89f1a}','\u{dab1c}'];
_3 = (_2.0, _2.1, _2.2);
_1.0 = _3.1.0;
_11 = -(-30_i8);
_21.1.1 = [_24,_24,_24,_24,_24,_24,_24];
_3.0 = _4.0;
_18 = _3.1.1.1;
_5.2 = _4.2;
_1 = _5.1;
Goto(bb10)
}
bb10 = {
_2.2 = _4.2;
_1.1.1 = [_24,_24,_24,_24,_24,_24,_24];
_16 = !_15.0;
_2.1.1.1 = _9;
_10 = ['\u{cccce}','\u{fa946}','\u{eae6f}','\u{1046f}','\u{5a8cb}','\u{de58b}','\u{1c203}','\u{92229}'];
_5.1.1.0 = _11 as i16;
_22 = 4886962649677232827_u64;
_5.1.1.1 = [_24,_24,_24,_24,_24,_24,_24];
_2.1.1.0 = -_1.1.0;
_2 = (_5.0, _5.1, _5.2);
_1.1 = _2.1.1;
_11 = !(-4_i8);
_3.2 = _4.2;
match _22 {
4886962649677232827 => bb12,
_ => bb11
}
}
bb11 = {
_2.1 = _1;
_5.1 = (_4.1.0, _1.1);
_1.0 = _5.1.0;
_18 = _6;
_5.1.1.0 = _2.1.0 as i16;
_2.1.0 = -_1.0;
_5 = (RET, _4.1, _4.2);
_5.1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5.1.1 = _2.1.1;
_19 = -_5.1.0;
RET = [11986906619422290101_u64,5039913960520121876_u64];
_1.1.0 = '\u{5fd8}' as i16;
_5 = (RET, _4.1, _2.2);
_4.1.1 = (_2.1.1.0, _9);
_16 = _15.0 / _15.0;
_1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),103_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
match _15.0 {
33 => bb3,
_ => bb1
}
}
bb12 = {
_4.0 = [_22,_22];
_22 = !13251449125787597231_u64;
_4.1.1 = (_3.1.1.0, _7);
_3.1.1.0 = _5.1.1.0 | _2.1.1.0;
_26 = [_24,_24,_24,_24,_24,_24,_24];
_18 = _4.1.1.1;
_5.1.1.1 = _18;
_2.2 = _5.2;
_5.2 = _3.2;
_4.1.1.0 = !_2.1.1.0;
_2.1.0 = -_21.0;
_18 = _2.1.1.1;
_13 = 1861372130_i32 as f64;
_2.2 = _4.2;
_3.1.0 = _1.0 + _5.1.0;
_5.1.1.0 = _24 as i16;
_3.0 = _2.0;
Goto(bb13)
}
bb13 = {
_23 = 1893641889_u32;
_5.1.1.1 = [_24,_24,_24,_24,_24,_24,_24];
_3.1 = _1;
_26 = _6;
_3.2 = _4.2;
_21.1.1 = _2.1.1.1;
_5.1.1.0 = !_3.1.1.0;
_1.1 = (_21.1.0, _7);
_26 = _4.1.1.1;
_1.1.0 = _3.1.1.0;
_12 = Adt49::Variant1 { fld0: _5.0,fld1: _26 };
_1.1.1 = _5.1.1.1;
_25 = 126009270097257792724561184493280959749_i128 & (-41810971467741980856128920146546361357_i128);
_21.0 = -_5.1.0;
_4.1.1.0 = 6020454478599629042_i64 as i16;
_1.1.1 = [_24,_24,_24,_24,_24,_24,_24];
Goto(bb14)
}
bb14 = {
SetDiscriminant(_12, 2);
_2.1.1.0 = _21.1.0 + _1.1.0;
_2 = _5;
_5.2 = _4.2;
_30.0.0 = (-1325005479815314136_i64);
place!(Field::<*const i64>(Variant(_12, 2), 4)) = core::ptr::addr_of!(_30.0.0);
_21.0 = _1.0;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(3_usize, 24_usize, Move(_24), 11_usize, Move(_11), 25_usize, Move(_25), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(3_usize, 16_usize, Move(_16), 9_usize, Move(_9), 34_usize, _34, 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (f64, (i16, [isize; 7])),mut _2: f64,mut _3: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _4: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _5: f64,mut _6: f64,mut _7: (i16, [isize; 7]),mut _8: (i16, [isize; 7])) -> *mut u128 {
mir! {
type RET = *mut u128;
let _9: [char; 8];
let _10: ((i64, usize, (i16, [isize; 7])),);
let _11: ([u64; 3],);
let _12: [bool; 8];
let _13: f32;
let _14: u64;
let _15: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128);
let _16: isize;
let _17: [isize; 7];
let _18: Adt49;
let _19: char;
let _20: Adt62;
let _21: ([u64; 2], i16, char);
let _22: *const [bool; 8];
let _23: [u32; 4];
let _24: isize;
let _25: u128;
let _26: Adt61;
let _27: Adt65;
let _28: [i32; 1];
let _29: [char; 8];
let _30: isize;
let _31: ();
let _32: ();
{
_5 = _1.0;
_3.2 = _4.2;
_1.1 = (_7.0, _3.1.1.1);
_3.1 = _1;
_3.0 = [5813645351661203988_u64,4217267941447039288_u64];
_2 = -_1.0;
_10.0.2.1 = [(-2_isize),117_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_3.2 = _4.2;
_1.0 = 316208113086416662758693825803354534731_u128 as f64;
Goto(bb1)
}
bb1 = {
_1.1 = (_4.1.1.0, _4.1.1.1);
_7 = (_1.1.0, _8.1);
_10.0.2 = _4.1.1;
_2 = 1468793545192557461934327989833196720_i128 as f64;
_9 = ['\u{10d48b}','\u{95326}','\u{be500}','\u{bee29}','\u{3b710}','\u{b9a29}','\u{c83e3}','\u{8ecf6}'];
_8.1 = [9223372036854775807_isize,9223372036854775807_isize,(-53_isize),(-9223372036854775808_isize),87_isize,9223372036854775807_isize,(-96_isize)];
_3.0 = _4.0;
RET = _3.2;
_4.1.1.0 = 62_u8 as i16;
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = !267108281055675084877508331042107774996_u128;
_3.1.0 = (*RET) as f64;
RET = _3.2;
_7 = (_3.1.1.0, _4.1.1.1);
_14 = (-9223372036854775808_isize) as u64;
_15 = _4;
_7.1 = _15.1.1.1;
RET = core::ptr::addr_of_mut!((*RET));
_15.1.0 = _5;
_3 = (_15.0, _15.1, _4.2);
Goto(bb2)
}
bb2 = {
_12 = [true,false,true,false,true,true,true,true];
_1.1.1 = [9223372036854775807_isize,78_isize,(-9223372036854775808_isize),(-41_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
(*RET) = 272418378478872730768341165225113525575_u128 >> _4.1.1.0;
_9 = ['\u{ba47b}','\u{1f917}','\u{87bd2}','\u{4e570}','\u{85aed}','\u{a5d5b}','\u{10f948}','\u{1041b6}'];
RET = _3.2;
_1 = (_15.1.0, _10.0.2);
(*RET) = 183567321695427291830013203840473779546_u128 >> _1.1.0;
_8 = (_10.0.2.0, _1.1.1);
_15.1.1.1 = _7.1;
_10.0.0 = 8289090789916350893_i64;
_1.1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,30_isize];
_4.0 = _3.0;
_7 = _15.1.1;
_15.1.0 = 0_usize as f64;
_10.0.2.0 = _8.0;
_10.0 = ((-5390761302132946775_i64), 7_usize, _7);
_15.2 = core::ptr::addr_of_mut!((*RET));
_9 = ['\u{5e956}','\u{395a2}','\u{10a109}','\u{210ba}','\u{93bc9}','\u{42881}','\u{522e0}','\u{8882a}'];
_1.1.1 = _8.1;
_3.1.1.0 = 554099220_u32 as i16;
_10.0.2.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-27_isize),(-9223372036854775808_isize),(-30_isize)];
_3.2 = core::ptr::addr_of_mut!((*RET));
(*RET) = 45473244945941143570543597322865873185_u128 - 151162237332482421353915377822860986614_u128;
_3.1.0 = _5 + _5;
_4.1.0 = (-96685695582881680128330188391088801737_i128) as f64;
_7.1 = _4.1.1.1;
_3.0 = [_14,_14];
_3.1.1.0 = _1.1.0 + _8.0;
_4.1.1.0 = _1.1.0;
_2 = -_6;
Goto(bb3)
}
bb3 = {
_3.2 = core::ptr::addr_of_mut!((*RET));
_3.1 = _1;
_3.0 = _4.0;
RET = _3.2;
_3.1.1.0 = -_10.0.2.0;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = !84702347600149653022780140661343645951_u128;
_8.1 = [(-9223372036854775808_isize),(-34_isize),9223372036854775807_isize,(-9223372036854775808_isize),64_isize,(-29_isize),7_isize];
_10.0 = (4588926044619958236_i64, 14599258865793335010_usize, _4.1.1);
_4.1.1 = _15.1.1;
_4.1.0 = -_2;
_10.0.2 = _7;
_10.0.1 = 2974531092_u32 as usize;
_15 = _3;
_15.2 = core::ptr::addr_of_mut!((*RET));
_1.0 = _3.1.0 * _5;
_13 = (-93487767091102780056238867894087178186_i128) as f32;
_3.1.1 = (_4.1.1.0, _1.1.1);
_3.1.1.1 = [(-9223372036854775808_isize),9223372036854775807_isize,7_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
match _10.0.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4588926044619958236 => bb7,
_ => bb6
}
}
bb4 = {
_12 = [true,false,true,false,true,true,true,true];
_1.1.1 = [9223372036854775807_isize,78_isize,(-9223372036854775808_isize),(-41_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
(*RET) = 272418378478872730768341165225113525575_u128 >> _4.1.1.0;
_9 = ['\u{ba47b}','\u{1f917}','\u{87bd2}','\u{4e570}','\u{85aed}','\u{a5d5b}','\u{10f948}','\u{1041b6}'];
RET = _3.2;
_1 = (_15.1.0, _10.0.2);
(*RET) = 183567321695427291830013203840473779546_u128 >> _1.1.0;
_8 = (_10.0.2.0, _1.1.1);
_15.1.1.1 = _7.1;
_10.0.0 = 8289090789916350893_i64;
_1.1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,30_isize];
_4.0 = _3.0;
_7 = _15.1.1;
_15.1.0 = 0_usize as f64;
_10.0.2.0 = _8.0;
_10.0 = ((-5390761302132946775_i64), 7_usize, _7);
_15.2 = core::ptr::addr_of_mut!((*RET));
_9 = ['\u{5e956}','\u{395a2}','\u{10a109}','\u{210ba}','\u{93bc9}','\u{42881}','\u{522e0}','\u{8882a}'];
_1.1.1 = _8.1;
_3.1.1.0 = 554099220_u32 as i16;
_10.0.2.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-27_isize),(-9223372036854775808_isize),(-30_isize)];
_3.2 = core::ptr::addr_of_mut!((*RET));
(*RET) = 45473244945941143570543597322865873185_u128 - 151162237332482421353915377822860986614_u128;
_3.1.0 = _5 + _5;
_4.1.0 = (-96685695582881680128330188391088801737_i128) as f64;
_7.1 = _4.1.1.1;
_3.0 = [_14,_14];
_3.1.1.0 = _1.1.0 + _8.0;
_4.1.1.0 = _1.1.0;
_2 = -_6;
Goto(bb3)
}
bb5 = {
_1.1 = (_4.1.1.0, _4.1.1.1);
_7 = (_1.1.0, _8.1);
_10.0.2 = _4.1.1;
_2 = 1468793545192557461934327989833196720_i128 as f64;
_9 = ['\u{10d48b}','\u{95326}','\u{be500}','\u{bee29}','\u{3b710}','\u{b9a29}','\u{c83e3}','\u{8ecf6}'];
_8.1 = [9223372036854775807_isize,9223372036854775807_isize,(-53_isize),(-9223372036854775808_isize),87_isize,9223372036854775807_isize,(-96_isize)];
_3.0 = _4.0;
RET = _3.2;
_4.1.1.0 = 62_u8 as i16;
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = !267108281055675084877508331042107774996_u128;
_3.1.0 = (*RET) as f64;
RET = _3.2;
_7 = (_3.1.1.0, _4.1.1.1);
_14 = (-9223372036854775808_isize) as u64;
_15 = _4;
_7.1 = _15.1.1.1;
RET = core::ptr::addr_of_mut!((*RET));
_15.1.0 = _5;
_3 = (_15.0, _15.1, _4.2);
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
(*RET) = !12818670957584060735216594559429889815_u128;
_15 = (_3.0, _1, _3.2);
_1.0 = _2;
_3.1.1 = _7;
_1 = (_5, _10.0.2);
_4 = _3;
_20.fld3.1.2 = _8;
_19 = '\u{a6660}';
_20.fld3.3 = 47672044628010342302450355756206264022_i128;
_3 = (_4.0, _15.1, RET);
_10.0 = ((-4714219498304424768_i64), 1_usize, _8);
_20.fld1 = Adt53::Variant1 { fld0: _12 };
_20.fld3.1.2 = (_10.0.2.0, _4.1.1.1);
_16 = (-1217714978_i32) as isize;
_20.fld3.1.1 = !_10.0.1;
_10.0.1 = _20.fld3.3 as usize;
_3.1.0 = _10.0.0 as f64;
_20.fld3.1.0 = true as i64;
_20.fld2 = [_20.fld3.1.1,_20.fld3.1.1];
_10.0.2.0 = _1.1.0;
_16 = (-112_isize) << _10.0.0;
_15.1.1.1 = [_16,_16,_16,_16,_16,_16,_16];
_4.0 = [_14,_14];
_10.0.0 = 205_u8 as i64;
(*RET) = 326726160430645783270123966989916693270_u128;
match (*RET) {
0 => bb3,
1 => bb2,
326726160430645783270123966989916693270 => bb9,
_ => bb8
}
}
bb8 = {
_3.2 = core::ptr::addr_of_mut!((*RET));
_3.1 = _1;
_3.0 = _4.0;
RET = _3.2;
_3.1.1.0 = -_10.0.2.0;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = !84702347600149653022780140661343645951_u128;
_8.1 = [(-9223372036854775808_isize),(-34_isize),9223372036854775807_isize,(-9223372036854775808_isize),64_isize,(-29_isize),7_isize];
_10.0 = (4588926044619958236_i64, 14599258865793335010_usize, _4.1.1);
_4.1.1 = _15.1.1;
_4.1.0 = -_2;
_10.0.2 = _7;
_10.0.1 = 2974531092_u32 as usize;
_15 = _3;
_15.2 = core::ptr::addr_of_mut!((*RET));
_1.0 = _3.1.0 * _5;
_13 = (-93487767091102780056238867894087178186_i128) as f32;
_3.1.1 = (_4.1.1.0, _1.1.1);
_3.1.1.1 = [(-9223372036854775808_isize),9223372036854775807_isize,7_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
match _10.0.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4588926044619958236 => bb7,
_ => bb6
}
}
bb9 = {
_4.2 = RET;
_1.0 = _20.fld3.1.0 as f64;
_20.fld3.1 = (_10.0.0, _10.0.1, _7);
_15.1.1 = (_1.1.0, _1.1.1);
_4.1.1.0 = _1.1.0;
match (*RET) {
0 => bb1,
1 => bb2,
2 => bb10,
326726160430645783270123966989916693270 => bb12,
_ => bb11
}
}
bb10 = {
_1.1 = (_4.1.1.0, _4.1.1.1);
_7 = (_1.1.0, _8.1);
_10.0.2 = _4.1.1;
_2 = 1468793545192557461934327989833196720_i128 as f64;
_9 = ['\u{10d48b}','\u{95326}','\u{be500}','\u{bee29}','\u{3b710}','\u{b9a29}','\u{c83e3}','\u{8ecf6}'];
_8.1 = [9223372036854775807_isize,9223372036854775807_isize,(-53_isize),(-9223372036854775808_isize),87_isize,9223372036854775807_isize,(-96_isize)];
_3.0 = _4.0;
RET = _3.2;
_4.1.1.0 = 62_u8 as i16;
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = !267108281055675084877508331042107774996_u128;
_3.1.0 = (*RET) as f64;
RET = _3.2;
_7 = (_3.1.1.0, _4.1.1.1);
_14 = (-9223372036854775808_isize) as u64;
_15 = _4;
_7.1 = _15.1.1.1;
RET = core::ptr::addr_of_mut!((*RET));
_15.1.0 = _5;
_3 = (_15.0, _15.1, _4.2);
Goto(bb2)
}
bb11 = {
Return()
}
bb12 = {
_3.1.0 = _3.1.1.0 as f64;
_1 = _15.1;
_9 = [_19,_19,_19,_19,_19,_19,_19,_19];
_3 = (_15.0, _15.1, RET);
_19 = '\u{fb99}';
_14 = _1.0 as u64;
_1.1.0 = _8.0 - _3.1.1.0;
_12 = [true,true,false,true,true,true,true,false];
_20.fld3.2 = _20.fld3.1.1 as u64;
_10.0.2 = (_1.1.0, _4.1.1.1);
_14 = !_20.fld3.2;
_24 = _16;
_21.2 = _19;
_6 = _5;
_15.2 = _3.2;
_7.0 = (*RET) as i16;
Goto(bb13)
}
bb13 = {
_10 = (_20.fld3.1,);
_27.fld1.0 = _3.0;
_27.fld1.1 = (_2, _4.1.1);
_1.1 = _3.1.1;
_3.1.1.0 = -_15.1.1.0;
_13 = 113_u8 as f32;
SetDiscriminant(_20.fld1, 1);
_15.1.0 = _2;
_20.fld3.2 = !_14;
Goto(bb14)
}
bb14 = {
_3.1.0 = 1737038316_i32 as f64;
_28 = [901583046_i32];
_15.2 = core::ptr::addr_of_mut!((*RET));
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(4_usize, 19_usize, Move(_19), 9_usize, Move(_9), 10_usize, Move(_10), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(4_usize, 14_usize, Move(_14), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: f64,mut _2: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128)) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _3: u64;
let _4: (*const [bool; 8], [char; 8]);
let _5: *const [bool; 8];
let _6: Adt65;
let _7: isize;
let _8: [isize; 7];
let _9: ([u64; 3],);
let _10: (i16, [isize; 7]);
let _11: char;
let _12: (i128, [bool; 8], [i32; 1], *const [bool; 8]);
let _13: u8;
let _14: ([u64; 2], i16, char);
let _15: Adt58;
let _16: u64;
let _17: *const [u64; 3];
let _18: ([usize; 2], u16, [usize; 2]);
let _19: ([u64; 2], i16, char);
let _20: (char, i8);
let _21: f32;
let _22: [i32; 1];
let _23: Adt64;
let _24: f64;
let _25: isize;
let _26: u64;
let _27: f32;
let _28: (i16, [isize; 7]);
let _29: i32;
let _30: i16;
let _31: Adt62;
let _32: Adt58;
let _33: i16;
let _34: ();
let _35: ();
{
_1 = -_2.1.0;
_2.0 = [6486425618219629578_u64,5789072079987602334_u64];
_2.1.1.1 = [(-9223372036854775808_isize),(-92_isize),(-9223372036854775808_isize),(-120_isize),9223372036854775807_isize,9223372036854775807_isize,(-61_isize)];
_2.1.0 = _1;
RET = [14745978299192773113_u64,14803853939571786517_u64];
_2.0 = [4499647010105333388_u64,15337958784766215446_u64];
RET = [346355908341250491_u64,6903527084795732615_u64];
_2.1.1.1 = [9223372036854775807_isize,(-44_isize),(-9223372036854775808_isize),(-88_isize),(-111_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2.1.0 = _1 + _1;
RET = [12973354695926048996_u64,1255284145717900509_u64];
_2.1.0 = _1;
_2.1.1.1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-76_isize),9223372036854775807_isize];
_1 = _2.1.0;
_2.1.1.1 = [(-9223372036854775808_isize),(-31_isize),(-70_isize),(-103_isize),9223372036854775807_isize,9223372036854775807_isize,108_isize];
_3 = 10371022630521333360_u64;
RET = [_3,_3];
_3 = !16072116421837503641_u64;
_4.1 = ['\u{cd3a9}','\u{50b25}','\u{5e3e0}','\u{10f0c6}','\u{ae04d}','\u{677ec}','\u{cb0f0}','\u{ede5a}'];
_1 = _2.1.0;
RET = _2.0;
_2.0 = [_3,_3];
_3 = 12135724729877397350_u64;
_4.1 = ['\u{110d1}','\u{10feef}','\u{2f933}','\u{1a49}','\u{42715}','\u{3801d}','\u{c5e4e}','\u{d174b}'];
RET = [_3,_3];
_6.fld1.1.1.0 = _2.1.1.0 ^ _2.1.1.0;
_4.1 = ['\u{9d224}','\u{1c522}','\u{d8855}','\u{32d94}','\u{8b52c}','\u{d3bfc}','\u{72121}','\u{e9891}'];
_6.fld1.1.1 = (_2.1.1.0, _2.1.1.1);
Goto(bb1)
}
bb1 = {
_6.fld0 = [4_usize,3449291027131948838_usize];
_2.1.1.1 = _6.fld1.1.1.1;
_6.fld1.1.0 = _2.1.0 * _2.1.0;
_2.1.1.0 = -_6.fld1.1.1.0;
Goto(bb2)
}
bb2 = {
_2.1.1 = (_6.fld1.1.1.0, _6.fld1.1.1.1);
_6.fld1.1.1.0 = _2.1.1.0;
_6.fld1.0 = [_3,_3];
_2.1 = (_6.fld1.1.0, _6.fld1.1.1);
Goto(bb3)
}
bb3 = {
RET = [_3,_3];
_2.1.1.0 = '\u{2747d}' as i16;
RET = _2.0;
_3 = 9414132258014862805_u64;
Call(_9.0 = fn6(_2, _6.fld1.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6.fld1.2 = _2.2;
_7 = 253_u8 as isize;
_2 = (_6.fld1.0, _6.fld1.1, _6.fld1.2);
_6.fld1.1.1 = (_2.1.1.0, _2.1.1.1);
RET = _2.0;
_2.2 = _6.fld1.2;
_6.fld1.1.1.0 = _2.1.1.0 >> _2.1.1.0;
_8 = [_7,_7,_7,_7,_7,_7,_7];
_2.1 = _6.fld1.1;
Goto(bb5)
}
bb5 = {
_11 = '\u{60b60}';
_2.1 = (_1, _6.fld1.1.1);
_6.fld1.1.1.1 = [_7,_7,_7,_7,_7,_7,_7];
_10 = (_6.fld1.1.1.0, _2.1.1.1);
_2.1.1.0 = _6.fld1.1.1.0 + _10.0;
RET = _6.fld1.0;
_11 = '\u{7547b}';
_2.1.1.0 = 6361764504523851732_i64 as i16;
_10 = _2.1.1;
_9.0 = [_3,_3,_3];
_6.fld1.1.1.0 = 319672089767434453511389598096741673315_u128 as i16;
_12.1 = [true,true,false,true,false,false,false,false];
match _3 {
0 => bb1,
9414132258014862805 => bb6,
_ => bb3
}
}
bb6 = {
_9.0 = [_3,_3,_3];
RET = _6.fld1.0;
_13 = 119_u8;
_12.1 = [false,false,false,true,true,true,false,false];
_12.3 = core::ptr::addr_of!(_12.1);
_2.1.0 = _6.fld1.1.0 * _6.fld1.1.0;
_9.0 = [_3,_3,_3];
_10.1 = [_7,_7,_7,_7,_7,_7,_7];
_6.fld1.1.1.0 = _10.0;
_14.1 = !_6.fld1.1.1.0;
_7 = (-9223372036854775808_isize) << _2.1.1.0;
_2.1.1.0 = -_6.fld1.1.1.0;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
9414132258014862805 => bb11,
_ => bb10
}
}
bb7 = {
_11 = '\u{60b60}';
_2.1 = (_1, _6.fld1.1.1);
_6.fld1.1.1.1 = [_7,_7,_7,_7,_7,_7,_7];
_10 = (_6.fld1.1.1.0, _2.1.1.1);
_2.1.1.0 = _6.fld1.1.1.0 + _10.0;
RET = _6.fld1.0;
_11 = '\u{7547b}';
_2.1.1.0 = 6361764504523851732_i64 as i16;
_10 = _2.1.1;
_9.0 = [_3,_3,_3];
_6.fld1.1.1.0 = 319672089767434453511389598096741673315_u128 as i16;
_12.1 = [true,true,false,true,false,false,false,false];
match _3 {
0 => bb1,
9414132258014862805 => bb6,
_ => bb3
}
}
bb8 = {
_6.fld1.2 = _2.2;
_7 = 253_u8 as isize;
_2 = (_6.fld1.0, _6.fld1.1, _6.fld1.2);
_6.fld1.1.1 = (_2.1.1.0, _2.1.1.1);
RET = _2.0;
_2.2 = _6.fld1.2;
_6.fld1.1.1.0 = _2.1.1.0 >> _2.1.1.0;
_8 = [_7,_7,_7,_7,_7,_7,_7];
_2.1 = _6.fld1.1;
Goto(bb5)
}
bb9 = {
RET = [_3,_3];
_2.1.1.0 = '\u{2747d}' as i16;
RET = _2.0;
_3 = 9414132258014862805_u64;
Call(_9.0 = fn6(_2, _6.fld1.1), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_2.1.1 = (_6.fld1.1.1.0, _6.fld1.1.1.1);
_6.fld1.1.1.0 = _2.1.1.0;
_6.fld1.0 = [_3,_3];
_2.1 = (_6.fld1.1.0, _6.fld1.1.1);
Goto(bb3)
}
bb11 = {
_2.1.0 = -_1;
_4.0 = _12.3;
_10.1 = _2.1.1.1;
_14.2 = _11;
_12.2 = [(-982180131_i32)];
_6.fld1.1 = _2.1;
_6.fld1.1.1 = _2.1.1;
_2.0 = [_3,_3];
RET = [_3,_3];
_11 = _14.2;
_14.0 = [_3,_3];
_6.fld0 = [13351704595930930674_usize,12281846715664416878_usize];
_14.0 = _2.0;
_1 = _6.fld1.1.0;
_15 = Adt58::Variant1 { fld0: _6.fld1 };
_2.1.1.0 = -_10.0;
_14.0 = [_3,_3];
_6.fld1.0 = [_3,_3];
_12.2 = [(-55152834_i32)];
_12.1 = [false,true,false,false,true,true,true,false];
_17 = core::ptr::addr_of!(_9.0);
_20 = (_11, 50_i8);
_19.1 = _14.1;
_19.2 = _11;
_16 = !_3;
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_15, 1), 0)).1.1.0 = _19.1 & _10.0;
match _20.1 {
0 => bb8,
1 => bb5,
2 => bb6,
50 => bb13,
_ => bb12
}
}
bb12 = {
_6.fld1.2 = _2.2;
_7 = 253_u8 as isize;
_2 = (_6.fld1.0, _6.fld1.1, _6.fld1.2);
_6.fld1.1.1 = (_2.1.1.0, _2.1.1.1);
RET = _2.0;
_2.2 = _6.fld1.2;
_6.fld1.1.1.0 = _2.1.1.0 >> _2.1.1.0;
_8 = [_7,_7,_7,_7,_7,_7,_7];
_2.1 = _6.fld1.1;
Goto(bb5)
}
bb13 = {
_1 = Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_15, 1), 0).1.0;
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_15, 1), 0)).1.0 = -_2.1.0;
_6.fld0 = [1_usize,0_usize];
_23.fld0 = true;
_19.1 = -_14.1;
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_15, 1), 0)) = (RET, _6.fld1.1, _6.fld1.2);
SetDiscriminant(_15, 2);
place!(Field::<[u64; 2]>(Variant(_15, 2), 1)) = [_3,_3];
_18.0 = _6.fld0;
_18.2 = [8039188402387894427_usize,9191120570031654928_usize];
_5 = core::ptr::addr_of!(_12.1);
_6.fld1.0 = Field::<[u64; 2]>(Variant(_15, 2), 1);
_14.0 = [_16,_3];
_14.2 = _11;
_2.1.1 = (_10.0, _10.1);
_25 = _16 as isize;
_2.1.1.1 = [_25,_7,_7,_7,_25,_7,_7];
_22 = _12.2;
_21 = _2.1.0 as f32;
_7 = _25;
_10 = (_6.fld1.1.1.0, _6.fld1.1.1.1);
Goto(bb14)
}
bb14 = {
place!(Field::<i16>(Variant(_15, 2), 4)) = -_10.0;
_29 = _7 as i32;
_28.1 = _2.1.1.1;
_31.fld3.1.1 = !9990873102001959311_usize;
_4.0 = _5;
_27 = _21;
_28 = (_2.1.1.0, _10.1);
_18.2 = [_31.fld3.1.1,_31.fld3.1.1];
_31.fld3.1.2.0 = _14.1 << _10.0;
_28.0 = !Field::<i16>(Variant(_15, 2), 4);
_19.0 = [_3,_16];
_31.fld3.1.2 = (Field::<i16>(Variant(_15, 2), 4), _28.1);
_6.fld1.1.1.1 = [_7,_25,_7,_25,_25,_25,_7];
_7 = _25;
_14.0 = [_3,_16];
_16 = _3 | _3;
_30 = Field::<i16>(Variant(_15, 2), 4);
_19 = (Field::<[u64; 2]>(Variant(_15, 2), 1), _14.1, _11);
_10.0 = -_2.1.1.0;
_31.fld3.1 = (4983341832274498190_i64, 3_usize, _28);
_2.2 = _6.fld1.2;
_31.fld3.2 = !_16;
_6.fld1.0 = [_3,_16];
_31.fld3.1 = ((-2537639097478408926_i64), 18238965520993069675_usize, _28);
_6.fld1.1 = (_2.1.0, _10);
_15 = Adt58::Variant3 { fld0: _29,fld1: 165128178_u32 };
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(5_usize, 16_usize, Move(_16), 30_usize, Move(_30), 19_usize, Move(_19), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(5_usize, 14_usize, Move(_14), 25_usize, Move(_25), 10_usize, Move(_10), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _2: (f64, (i16, [isize; 7]))) -> [u64; 3] {
mir! {
type RET = [u64; 3];
let _3: Adt65;
let _4: Adt63;
let _5: char;
let _6: Adt63;
let _7: [i32; 1];
let _8: i8;
let _9: Adt54;
let _10: u64;
let _11: [i8; 8];
let _12: isize;
let _13: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128);
let _14: isize;
let _15: ([u64; 2], u16, [char; 8], i128);
let _16: i32;
let _17: u16;
let _18: f64;
let _19: i64;
let _20: u64;
let _21: (f64, (i16, [isize; 7]));
let _22: *mut char;
let _23: char;
let _24: [isize; 7];
let _25: Adt51;
let _26: Adt62;
let _27: i16;
let _28: bool;
let _29: bool;
let _30: usize;
let _31: u32;
let _32: ([usize; 2], u16, [usize; 2]);
let _33: (char, i8);
let _34: isize;
let _35: ([usize; 2], u16, [usize; 2]);
let _36: (i128, [bool; 8], [i32; 1], *const [bool; 8]);
let _37: *mut char;
let _38: ();
let _39: ();
{
_1.1.1 = (_2.1.0, _2.1.1);
Goto(bb1)
}
bb1 = {
RET = [11130539700002971290_u64,7601018311436077242_u64,4662124226323273036_u64];
RET = [5973373407108143077_u64,606741629038802240_u64,12247636915896419618_u64];
_4.fld1.fld1.2 = '\u{2ea61}';
_3.fld0 = [1_usize,3_usize];
_3.fld1.1.1 = (_1.1.1.0, _2.1.1);
RET = [16110080466878213435_u64,11032159630120566277_u64,4508575045295633533_u64];
_4.fld1.fld1.1 = 1487171687_i32 as i16;
_3.fld0 = [5_usize,14465095848388067572_usize];
_1.1.1 = _3.fld1.1.1;
_4.fld1.fld3 = !74066070103762421_usize;
_2.1 = (_1.1.1.0, _1.1.1.1);
_6.fld1.fld0 = Adt49::Variant3 { fld0: 166_u8,fld1: _4.fld1.fld3,fld2: (-14107274578906381363232301092073067114_i128),fld3: RET };
_3.fld1 = _1;
Call(RET = fn7(_1.1.0, _3.fld1.1, _1, _3.fld1, _1, _3.fld1, Move(_3), _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6.fld1.fld2 = [21_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-47_isize),9223372036854775807_isize,9223372036854775807_isize];
_3.fld1.1.1 = (_2.1.0, _2.1.1);
_6.fld1.fld1.0 = [15250232975202034800_u64,1253349162155065348_u64];
_5 = _4.fld1.fld1.2;
_3.fld1 = (_1.0, _1.1, _1.2);
_4.fld1.fld1 = (_6.fld1.fld1.0, _3.fld1.1.1.0, _5);
place!(Field::<i128>(Variant(_6.fld1.fld0, 3), 2)) = 9508120190071427233613451672558027937_i128;
_6.fld1.fld1.1 = -_2.1.0;
_6.fld1.fld5 = 1429776588_i32 >> _4.fld1.fld3;
_3.fld1 = (_4.fld1.fld1.0, _2, _1.2);
place!(Field::<usize>(Variant(_6.fld1.fld0, 3), 1)) = _4.fld1.fld3 ^ _4.fld1.fld3;
_2.0 = _1.1.0;
_3.fld1 = (_4.fld1.fld1.0, _1.1, _1.2);
_6.fld1.fld0 = Adt49::Variant3 { fld0: 121_u8,fld1: _4.fld1.fld3,fld2: 127011581784451365766244262327871898271_i128,fld3: RET };
_2.1.0 = _3.fld1.1.1.0;
_5 = _4.fld1.fld1.2;
_6.fld1.fld1.0 = [12572533819660026088_u64,464929621875456985_u64];
_4.fld1.fld5 = false as i32;
_6.fld1.fld3 = !_4.fld1.fld3;
_6.fld1.fld1.0 = [12364521424458565463_u64,17871966656797019930_u64];
_6.fld1.fld1 = (_1.0, _2.1.0, _5);
_3.fld0 = [_6.fld1.fld3,_6.fld1.fld3];
_1.0 = [5353331351077909541_u64,5978661935234455479_u64];
_4.fld1.fld0 = Adt49::Variant3 { fld0: 120_u8,fld1: Field::<usize>(Variant(_6.fld1.fld0, 3), 1),fld2: (-1720670845239502691783233990818922875_i128),fld3: RET };
place!(Field::<u8>(Variant(_6.fld1.fld0, 3), 0)) = 13_u8;
_3.fld1.0 = _1.0;
match Field::<u8>(Variant(_6.fld1.fld0, 3), 0) {
13 => bb3,
_ => bb1
}
}
bb3 = {
place!(Field::<u8>(Variant(_4.fld1.fld0, 3), 0)) = 9223372036854775807_isize as u8;
_3.fld1.1 = (_1.1.0, _2.1);
_2.1.1 = _1.1.1.1;
place!(Field::<i128>(Variant(_4.fld1.fld0, 3), 2)) = 82588446501782826140703905134242640993_i128;
place!(Field::<i128>(Variant(_4.fld1.fld0, 3), 2)) = (-23_i8) as i128;
SetDiscriminant(_4.fld1.fld0, 2);
place!(Field::<i128>(Variant(_6.fld1.fld0, 3), 2)) = -153784514709535872918697743821087205254_i128;
Goto(bb4)
}
bb4 = {
place!(Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5)).0 = [10936138262619193331_u64,1325386246511658877_u64,6740701369608073806_u64];
place!(Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1)) = (_4.fld1.fld1.2, 95_i8);
place!(Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1)) = (_4.fld1.fld1.2, (-95_i8));
_6.fld1.fld1 = _4.fld1.fld1;
_1.1.1 = _3.fld1.1.1;
Goto(bb5)
}
bb5 = {
_3.fld1.1.1 = (_4.fld1.fld1.1, _2.1.1);
_4.fld1.fld1.1 = !_2.1.0;
_7 = [_4.fld1.fld5];
_6.fld1.fld1.1 = Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).1 as i16;
_15.0 = _6.fld1.fld1.0;
_1.1.1.0 = _3.fld1.1.1.0;
_13.0 = [8840160904274493648_u64,2313890084883712677_u64];
SetDiscriminant(_6.fld1.fld0, 1);
_3.fld1.0 = _15.0;
_13.1.1 = (_2.1.0, _3.fld1.1.1.1);
Call(_8 = fn13(_2, _13.1.1, _2.0, _3.fld1.1.0, _3.fld1, Move(_3), _1, _2.1, _1.1.0, _13.1.1.1, _2.0, Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5), _1, _1.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13.0 = [12520762036376452771_u64,13335790476194191035_u64];
place!(Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5)) = (RET,);
place!(Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1)).1 = (-10660352782346886783436999949053151091_i128) as i8;
_15.3 = (-77744841629869370606794852061266985552_i128) * (-79587708662523566439214164646712942381_i128);
Goto(bb7)
}
bb7 = {
_1.1.1.0 = _4.fld1.fld1.1 >> _8;
_2.1.0 = _1.1.1.0;
_15.2 = [_4.fld1.fld1.2,_6.fld1.fld1.2,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).0,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).0,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).0,_5,_4.fld1.fld1.2,_5];
_1.1.1 = _2.1;
_15.1 = 8661318584444249748_u64 as u16;
_2.1 = _1.1.1;
_18 = _8 as f64;
_1.1.1 = (_2.1.0, _6.fld1.fld2);
_3.fld1.0 = [11141166172963428601_u64,15619391594841819252_u64];
_14 = 183257622571256270352973610587920236562_u128 as isize;
place!(Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1)).0 = _6.fld1.fld1.2;
place!(Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1)) = (_5, _8);
_3.fld1.1.1.0 = 210336757314132924190280020952365018451_u128 as i16;
_1.1.1.0 = _2.1.0 | _2.1.0;
Goto(bb8)
}
bb8 = {
_13.1.1.1 = _6.fld1.fld2;
_13.1.1.1 = _1.1.1.1;
_21.1 = _2.1;
_4.fld1.fld4 = core::ptr::addr_of!(_19);
RET = Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5).0;
_16 = false as i32;
_21 = (_18, _2.1);
_13.1 = (_21.0, _21.1);
_1.1.1 = (_2.1.0, _13.1.1.1);
_13.2 = _1.2;
place!(Field::<*const i64>(Variant(_4.fld1.fld0, 2), 4)) = core::ptr::addr_of!(_19);
_4.fld1.fld1 = (_3.fld1.0, _13.1.1.0, _6.fld1.fld1.2);
_17 = _15.1;
_2.1 = (_13.1.1.0, _21.1.1);
_3.fld1.1.0 = -_21.0;
_3.fld1 = _13;
_6.fld1.fld1.0 = [15759980484920234609_u64,15996719857819952712_u64];
_10 = 15678335422378257130_u64 << _13.1.1.0;
RET = Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5).0;
_11 = [_8,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).1,_8,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).1,_8,_8,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).1,_8];
Call(_4.fld1.fld3 = fn16(_13.1.1, _13.1, _21.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3.fld0 = [_4.fld1.fld3,_4.fld1.fld3];
_6.fld1.fld2 = [_14,_14,_14,_14,_14,_14,_14];
_13 = (_1.0, _3.fld1.1, _1.2);
_19 = (-4208146318027455910_i64) - (-4300849868772889411_i64);
_6.fld1.fld2 = [_14,_14,_14,_14,_14,_14,_14];
_22 = core::ptr::addr_of_mut!(_4.fld1.fld1.2);
place!(Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5)).0 = [_10,_10,_10];
_3.fld1.0 = [_10,_10];
_13.1.1.1 = [_14,_14,_14,_14,_14,_14,_14];
_6.fld1.fld4 = core::ptr::addr_of!(_26.fld3.1.0);
Goto(bb10)
}
bb10 = {
place!(Field::<[u32; 4]>(Variant(_4.fld1.fld0, 2), 3)) = [170067603_u32,828208101_u32,1828221358_u32,1370332452_u32];
_4.fld1.fld1.1 = _2.1.0 | _21.1.0;
_2 = _21;
_3.fld1.1 = (_18, _13.1.1);
place!(Field::<[u64; 2]>(Variant(_6.fld1.fld0, 1), 0)) = [_10,_10];
_3.fld1.2 = _1.2;
_26.fld3.1.2.0 = _4.fld1.fld3 as i16;
_15.1 = _17 >> _2.1.0;
_26.fld3.1.0 = !_19;
place!(Field::<f32>(Variant(_4.fld1.fld0, 2), 0)) = _15.3 as f32;
_24 = [_14,_14,_14,_14,_14,_14,_14];
_26.fld0 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_4.fld1.fld2 = [_14,_14,_14,_14,_14,_14,_14];
place!(Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5)) = (RET,);
Call(_21.0 = core::intrinsics::transmute(_10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_26.fld3.1.0 = !_19;
_1.1.1.1 = [_14,_14,_14,_14,_14,_14,_14];
_2.1 = (_4.fld1.fld1.1, _1.1.1.1);
_6.fld1.fld1.2 = Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).0;
_21 = (_13.1.0, _2.1);
_6.fld0 = Adt54::Variant1 { fld0: true };
_20 = !_10;
_18 = _1.1.0 - _2.0;
_13.2 = _1.2;
_15.1 = !_17;
_2.1 = (_4.fld1.fld1.1, _6.fld1.fld2);
place!(Field::<[u64; 2]>(Variant(_6.fld1.fld0, 1), 0)) = [_10,_20];
_2 = (_13.1.0, _3.fld1.1.1);
(*_22) = Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).0;
(*_22) = _6.fld1.fld1.2;
_13.1.1.1 = [_14,_14,_14,_14,_14,_14,_14];
_29 = false;
Goto(bb12)
}
bb12 = {
_13 = _3.fld1;
_4.fld1.fld1.0 = _13.0;
_4.fld1.fld3 = _6.fld1.fld3;
_3.fld1.2 = _1.2;
_32.1 = !_15.1;
_26.fld3.1.2.0 = _1.1.1.0 | _3.fld1.1.1.0;
_21.0 = -_13.1.0;
_26.fld3.1.1 = !_4.fld1.fld3;
_3.fld1.1 = (_18, _2.1);
_2.1 = _3.fld1.1.1;
_6.fld1.fld5 = _4.fld1.fld5 << _20;
_26.fld3.1.2 = _3.fld1.1.1;
place!(Field::<[u32; 4]>(Variant(_4.fld1.fld0, 2), 3)) = [1257983948_u32,2746597937_u32,2537954167_u32,354313134_u32];
_3.fld1.1.1 = (_21.1.0, _21.1.1);
_11 = [Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).1,_8,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).1,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).1,_8,_8,Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).1,_8];
_23 = (*_22);
Goto(bb13)
}
bb13 = {
_4.fld1.fld0 = Adt49::Variant3 { fld0: 248_u8,fld1: _6.fld1.fld3,fld2: _15.3,fld3: RET };
place!(Field::<[u64; 2]>(Variant(_6.fld1.fld0, 1), 0)) = _3.fld1.0;
_4.fld1.fld2 = [_14,_14,_14,_14,_14,_14,_14];
_26.fld3.1.1 = _6.fld1.fld3;
_21.1.1 = [_14,_14,_14,_14,_14,_14,_14];
place!(Field::<u8>(Variant(_4.fld1.fld0, 3), 0)) = 159_u8;
_3.fld1.1.1.1 = [_14,_14,_14,_14,_14,_14,_14];
_26.fld2 = _3.fld0;
_2.1.0 = -_21.1.0;
_13.1.1.1 = [_14,_14,_14,_14,_14,_14,_14];
(*_22) = _23;
_32.1 = !_17;
_1.1 = (_2.0, _2.1);
_34 = _14;
_13.1.1.0 = !_2.1.0;
_25 = Adt51::Variant2 { fld0: _21.1,fld1: Field::<[u64; 3]>(Variant(_4.fld1.fld0, 3), 3),fld2: _11 };
_21.1.0 = _3.fld1.1.1.0 >> _26.fld3.1.2.0;
_14 = !_34;
_26.fld3.1.1 = 3836865872_u32 as usize;
match Field::<u8>(Variant(_4.fld1.fld0, 3), 0) {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb14,
4 => bb15,
159 => bb17,
_ => bb16
}
}
bb14 = {
_3.fld0 = [_4.fld1.fld3,_4.fld1.fld3];
_6.fld1.fld2 = [_14,_14,_14,_14,_14,_14,_14];
_13 = (_1.0, _3.fld1.1, _1.2);
_19 = (-4208146318027455910_i64) - (-4300849868772889411_i64);
_6.fld1.fld2 = [_14,_14,_14,_14,_14,_14,_14];
_22 = core::ptr::addr_of_mut!(_4.fld1.fld1.2);
place!(Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5)).0 = [_10,_10,_10];
_3.fld1.0 = [_10,_10];
_13.1.1.1 = [_14,_14,_14,_14,_14,_14,_14];
_6.fld1.fld4 = core::ptr::addr_of!(_26.fld3.1.0);
Goto(bb10)
}
bb15 = {
_26.fld3.1.0 = !_19;
_1.1.1.1 = [_14,_14,_14,_14,_14,_14,_14];
_2.1 = (_4.fld1.fld1.1, _1.1.1.1);
_6.fld1.fld1.2 = Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).0;
_21 = (_13.1.0, _2.1);
_6.fld0 = Adt54::Variant1 { fld0: true };
_20 = !_10;
_18 = _1.1.0 - _2.0;
_13.2 = _1.2;
_15.1 = !_17;
_2.1 = (_4.fld1.fld1.1, _6.fld1.fld2);
place!(Field::<[u64; 2]>(Variant(_6.fld1.fld0, 1), 0)) = [_10,_20];
_2 = (_13.1.0, _3.fld1.1.1);
(*_22) = Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1).0;
(*_22) = _6.fld1.fld1.2;
_13.1.1.1 = [_14,_14,_14,_14,_14,_14,_14];
_29 = false;
Goto(bb12)
}
bb16 = {
_13.0 = [12520762036376452771_u64,13335790476194191035_u64];
place!(Field::<([u64; 3],)>(Variant(_4.fld1.fld0, 2), 5)) = (RET,);
place!(Field::<(char, i8)>(Variant(_4.fld1.fld0, 2), 1)).1 = (-10660352782346886783436999949053151091_i128) as i8;
_15.3 = (-77744841629869370606794852061266985552_i128) * (-79587708662523566439214164646712942381_i128);
Goto(bb7)
}
bb17 = {
_35.2 = [_26.fld3.1.1,Field::<usize>(Variant(_4.fld1.fld0, 3), 1)];
_2 = _21;
_32.2 = _3.fld0;
_13.0 = _3.fld1.0;
_35.1 = _15.1;
_13 = (_4.fld1.fld1.0, _2, _1.2);
_3.fld1.1.0 = _18 * _13.1.0;
_31 = 3551889930_u32;
_30 = _6.fld1.fld3 & _6.fld1.fld3;
_3.fld1.0 = _4.fld1.fld1.0;
_33.1 = _8 + _8;
_36.1 = [_29,_29,_29,_29,_29,_29,_29,_29];
_2 = _21;
RET = [_20,_20,_20];
_26.fld2 = [Field::<usize>(Variant(_4.fld1.fld0, 3), 1),_6.fld1.fld3];
_33.0 = _6.fld1.fld1.2;
_2.1 = (_21.1.0, Field::<(i16, [isize; 7])>(Variant(_25, 2), 0).1);
_2.0 = _26.fld3.1.2.0 as f64;
SetDiscriminant(_25, 0);
Goto(bb18)
}
bb18 = {
Call(_38 = dump_var(6_usize, 5_usize, Move(_5), 23_usize, Move(_23), 8_usize, Move(_8), 34_usize, Move(_34)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(6_usize, 16_usize, Move(_16), 24_usize, Move(_24), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(6_usize, 30_usize, Move(_30), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: f64,mut _2: (f64, (i16, [isize; 7])),mut _3: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _4: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _5: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _6: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _7: Adt65,mut _8: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _9: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128)) -> [u64; 3] {
mir! {
type RET = [u64; 3];
let _10: ([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128);
let _11: *mut u128;
let _12: isize;
let _13: (f64, (i16, [isize; 7]));
let _14: Adt61;
let _15: Adt55;
let _16: f32;
let _17: *const [u64; 3];
let _18: char;
let _19: [i8; 8];
let _20: (f64, (i16, [isize; 7]));
let _21: u16;
let _22: i8;
let _23: char;
let _24: isize;
let _25: isize;
let _26: ((i64, usize, (i16, [isize; 7])),);
let _27: i64;
let _28: f64;
let _29: char;
let _30: ([u64; 2], i16, char);
let _31: char;
let _32: ();
let _33: ();
{
RET = [8182481733608500373_u64,14390475691860801503_u64,15294734331469186373_u64];
_2 = _4.1;
_4.1.1 = (_7.fld1.1.1.0, _6.1.1.1);
_4.2 = _3.2;
_6.1.1 = (_4.1.1.0, _2.1.1);
_2.1 = (_7.fld1.1.1.0, _5.1.1.1);
_6.1.1 = (_7.fld1.1.1.0, _4.1.1.1);
_10.1.0 = !(-4055245143875528719_i64);
_7.fld1.0 = [16241951546282982942_u64,2575047823131918973_u64];
_8.0 = [16783045477431853997_u64,16084065096863935729_u64];
_9.0 = [911960027278038579_u64,613542259312996296_u64];
_8.1.1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8.1.1.1 = [65_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-32_isize)];
_10.3 = (-77715728556245885813992772089037287371_i128) + (-132355676003808587838488224563088257063_i128);
Goto(bb1)
}
bb1 = {
_6 = (_8.0, _4.1, _4.2);
_4.1 = _5.1;
_12 = (-9223372036854775808_isize);
_3.1.1.1 = [_12,_12,_12,_12,_12,_12,_12];
_7.fld1.1.1.1 = [_12,_12,_12,_12,_12,_12,_12];
_10.1.0 = (-7943812413939374909_i64) ^ (-1475380828651520487_i64);
_10.1 = ((-2270180439161798743_i64), 1_usize, _4.1.1);
_2.0 = 2760195036_u32 as f64;
_8.1.1.0 = _4.1.1.0;
_13.1.1 = _4.1.1.1;
_2 = (_5.1.0, _4.1.1);
_11 = _4.2;
_3.0 = [12492983559243763360_u64,12541442757404065004_u64];
_2 = _8.1;
_1 = _10.3 as f64;
_8.1.1.1 = [_12,_12,_12,_12,_12,_12,_12];
_7.fld1 = (_6.0, _6.1, _3.2);
_10.2 = 17325839_i32 as u64;
match _10.1.1 {
0 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1 => bb9,
_ => bb8
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
Return()
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_7.fld1 = (_6.0, _2, _5.2);
_8.1 = (_5.1.0, _7.fld1.1.1);
_8.1.1.0 = _3.1.1.0;
_9.1.1 = _2.1;
_20.0 = -_8.1.0;
_22 = _10.1.0 as i8;
_16 = _10.3 as f32;
_7.fld1.1 = (_3.1.0, _6.1.1);
_10.0 = [_22,_22,_22,_22,_22,_22,_22,_22];
_25 = _12;
_8 = (_3.0, _3.1, _9.2);
_5.1.1 = (_6.1.1.0, _9.1.1.1);
Call(_3.1.1 = fn8(_9, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_26 = (_10.1,);
_5.0 = [_10.2,_10.2];
match _10.1.1 {
0 => bb11,
2 => bb13,
1 => bb15,
_ => bb14
}
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
Return()
}
bb15 = {
_13.0 = _8.1.0 - _8.1.0;
_8 = (_6.0, _9.1, _4.2);
_2.1 = (_6.1.1.0, _7.fld1.1.1.1);
_13.1.0 = !_2.1.0;
_20 = (_13.0, _13.1);
_2.1 = _10.1.2;
_2.1.0 = (-613768361_i32) as i16;
_2 = _9.1;
_2.1.1 = [_12,_25,_25,_12,_25,_25,_25];
_8 = (_7.fld1.0, _5.1, _6.2);
_6.1.1 = (_4.1.1.0, _26.0.2.1);
_10.1.0 = _26.0.0;
_26.0.0 = _10.1.0;
_22 = 72_u8 as i8;
_19 = [_22,_22,_22,_22,_22,_22,_22,_22];
_7.fld1.1.0 = _4.1.0 - _4.1.0;
_13.1.0 = _9.1.1.0;
_5.1.1 = (_7.fld1.1.1.0, _10.1.2.1);
_2.1.0 = _9.1.1.0 ^ _7.fld1.1.1.0;
_31 = '\u{fd1f1}';
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(7_usize, 31_usize, Move(_31), 22_usize, Move(_22), 25_usize, Move(_25), 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _2: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128)) -> (i16, [isize; 7]) {
mir! {
type RET = (i16, [isize; 7]);
let _3: ([i32; 1], *mut char, (i64, usize, (i16, [isize; 7])), i8, f32);
let _4: (bool,);
let _5: ([u64; 2], i16, char);
let _6: (i64, usize, (i16, [isize; 7]));
let _7: [u64; 3];
let _8: bool;
let _9: ([usize; 2], u16, [usize; 2]);
let _10: f64;
let _11: isize;
let _12: char;
let _13: ([u64; 3],);
let _14: f32;
let _15: ();
let _16: ();
{
RET.0 = (-117094260693158220623857411308596499798_i128) as i16;
_3.2.2.0 = !_1.1.1.0;
RET.1 = [9223372036854775807_isize,(-108_isize),53_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_2.1 = (_1.1.0, RET);
_1.2 = _2.2;
_1 = _2;
_4 = (true,);
_1.1 = (_2.1.0, RET);
_3.2.2 = _2.1.1;
_4.0 = !true;
_1.1 = (_2.1.0, _3.2.2);
_3.2.0 = (-8357427998929152053_i64) + (-3514025298693492010_i64);
_3.1 = core::ptr::addr_of_mut!(_5.2);
_5.0 = [7965031758354083364_u64,10520482497761718291_u64];
RET = (_3.2.2.0, _2.1.1.1);
_3.0 = [(-68751770_i32)];
_1.0 = [4561779683227085778_u64,15220329416488328085_u64];
_3.2 = ((-8856663170879087269_i64), 3_usize, RET);
Call(_3.2.2.1 = fn9(_2, _2.1.0, _1, _1.1, _2.1, _1, _2.1, _1, _2.1.0, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.2.0 = _3.2.2.0;
RET = (_1.1.1.0, _1.1.1.1);
RET.1 = [(-9223372036854775808_isize),(-59_isize),(-44_isize),(-40_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2.1.1.0 = _6.2.0 - RET.0;
_5 = (_1.0, RET.0, '\u{e0492}');
_2.0 = _1.0;
_6.2 = (_2.1.1.0, _3.2.2.1);
RET.1 = [9223372036854775807_isize,109_isize,6_isize,9223372036854775807_isize,91_isize,(-119_isize),9223372036854775807_isize];
_4.0 = !false;
_6.2 = _2.1.1;
_4.0 = true;
match _3.2.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb7,
_ => bb6
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
Return()
}
bb7 = {
_1.1.1.0 = _6.2.0;
_6 = _3.2;
_3.0 = [2028233413_i32];
_2.1.0 = (-1447508301_i32) as f64;
_9.1 = !18636_u16;
_6.1 = !_3.2.1;
_6.2.1 = [(-85_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,8_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_2.1.1.0 = _6.2.0;
_1.2 = _2.2;
RET.0 = _3.2.2.0 << _6.0;
_2.2 = _1.2;
_8 = _1.1.0 >= _1.1.0;
RET.0 = -_6.2.0;
_3.2.2.1 = RET.1;
_5.2 = '\u{ffcb5}';
_10 = _6.1 as f64;
_2 = (_1.0, _1.1, _1.2);
_4.0 = !_8;
_7 = [9354649832633898055_u64,2861094357425222832_u64,8887029631338613484_u64];
_5.0 = [16750760798658755855_u64,8266315601916826721_u64];
_4 = (_8,);
_10 = -_1.1.0;
RET.0 = _5.1;
_3.3 = -(-103_i8);
_6.2 = (_1.1.1.0, _2.1.1.1);
_3.2.2.1 = [9223372036854775807_isize,9223372036854775807_isize,77_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-68_isize)];
match _3.2.0 {
0 => bb6,
340282366920938463454517944260889124187 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_9.1 = _8 as u16;
_3.3 = (-97_i8) * 123_i8;
_5.2 = '\u{5727e}';
RET.0 = _1.1.1.0;
_6 = (_3.2.0, _3.2.1, _2.1.1);
_5.1 = RET.0;
_6.1 = !_3.2.1;
_11 = 150107235202665890890944953325280598665_u128 as isize;
_3.3 = _5.2 as i8;
_3.2.2.0 = _6.2.0 << _9.1;
_9.0 = [_6.1,_3.2.1];
match _6.0 {
0 => bb6,
1 => bb5,
2 => bb10,
3 => bb11,
4 => bb12,
340282366920938463454517944260889124187 => bb14,
_ => bb13
}
}
bb10 = {
_6.2.0 = _3.2.2.0;
RET = (_1.1.1.0, _1.1.1.1);
RET.1 = [(-9223372036854775808_isize),(-59_isize),(-44_isize),(-40_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2.1.1.0 = _6.2.0 - RET.0;
_5 = (_1.0, RET.0, '\u{e0492}');
_2.0 = _1.0;
_6.2 = (_2.1.1.0, _3.2.2.1);
RET.1 = [9223372036854775807_isize,109_isize,6_isize,9223372036854775807_isize,91_isize,(-119_isize),9223372036854775807_isize];
_4.0 = !false;
_6.2 = _2.1.1;
_4.0 = true;
match _3.2.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb7,
_ => bb6
}
}
bb11 = {
_1.1.1.0 = _6.2.0;
_6 = _3.2;
_3.0 = [2028233413_i32];
_2.1.0 = (-1447508301_i32) as f64;
_9.1 = !18636_u16;
_6.1 = !_3.2.1;
_6.2.1 = [(-85_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,8_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_2.1.1.0 = _6.2.0;
_1.2 = _2.2;
RET.0 = _3.2.2.0 << _6.0;
_2.2 = _1.2;
_8 = _1.1.0 >= _1.1.0;
RET.0 = -_6.2.0;
_3.2.2.1 = RET.1;
_5.2 = '\u{ffcb5}';
_10 = _6.1 as f64;
_2 = (_1.0, _1.1, _1.2);
_4.0 = !_8;
_7 = [9354649832633898055_u64,2861094357425222832_u64,8887029631338613484_u64];
_5.0 = [16750760798658755855_u64,8266315601916826721_u64];
_4 = (_8,);
_10 = -_1.1.0;
RET.0 = _5.1;
_3.3 = -(-103_i8);
_6.2 = (_1.1.1.0, _2.1.1.1);
_3.2.2.1 = [9223372036854775807_isize,9223372036854775807_isize,77_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-68_isize)];
match _3.2.0 {
0 => bb6,
340282366920938463454517944260889124187 => bb9,
_ => bb8
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_1.1.1.0 = _3.2.2.0 & _3.2.2.0;
_3.2.0 = 2556892394_u32 as i64;
_3.2.2.1 = RET.1;
_1 = _2;
_2.1.1.0 = _9.1 as i16;
_3.2.2 = _2.1.1;
Goto(bb15)
}
bb15 = {
Call(_15 = dump_var(8_usize, 4_usize, Move(_4), 8_usize, Move(_8), 7_usize, Move(_7), 16_usize, _16), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _2: f64,mut _3: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _4: (f64, (i16, [isize; 7])),mut _5: (f64, (i16, [isize; 7])),mut _6: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _7: (f64, (i16, [isize; 7])),mut _8: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _9: f64,mut _10: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _11: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128)) -> [isize; 7] {
mir! {
type RET = [isize; 7];
let _12: isize;
let _13: [u32; 4];
let _14: [u16; 8];
let _15: isize;
let _16: (u8,);
let _17: Adt58;
let _18: ([u64; 2], u16, [char; 8], i128);
let _19: u128;
let _20: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128);
let _21: isize;
let _22: [u64; 3];
let _23: isize;
let _24: (i64, usize, (i16, [isize; 7]));
let _25: [i32; 1];
let _26: (i16, [isize; 7]);
let _27: isize;
let _28: i64;
let _29: (bool,);
let _30: (i16, [isize; 7]);
let _31: [u32; 4];
let _32: i128;
let _33: u128;
let _34: u32;
let _35: (f64, (i16, [isize; 7]));
let _36: u16;
let _37: i64;
let _38: (i128, [bool; 8], [i32; 1], *const [bool; 8]);
let _39: bool;
let _40: ();
let _41: ();
{
_7.1.0 = (-28_i8) as i16;
_11.1.1.0 = !_6.1.1.0;
_8.1 = (_7.0, _1.1.1);
Goto(bb1)
}
bb1 = {
_13 = [265127855_u32,1814022229_u32,4238413721_u32,2079015509_u32];
_4.0 = _3.1.0;
_1.1 = (_5.0, _4.1);
_6.1.0 = _5.0;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,51_isize,(-9223372036854775808_isize),45_isize,(-76_isize)];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),4_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,45_isize];
_7.1.1 = [(-45_isize),(-9223372036854775808_isize),37_isize,84_isize,9223372036854775807_isize,(-9223372036854775808_isize),39_isize];
_8.1.0 = (-1254756206_i32) as f64;
_9 = 178_u8 as f64;
_6.0 = _8.0;
_10.1.1.0 = _7.1.0;
_5.1.0 = _11.1.1.0 << _7.1.0;
_18.1 = 44871_u16;
_4.1.1 = _11.1.1.1;
_6.1 = _11.1;
_18.2 = ['\u{869e6}','\u{f4263}','\u{e0877}','\u{2991c}','\u{f1281}','\u{1ad40}','\u{6b534}','\u{103c04}'];
_8.2 = core::ptr::addr_of_mut!(_19);
_5.1.1 = [48_isize,97_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-35_isize)];
_11.1.0 = -_6.1.0;
_7.0 = _4.0;
_11.1.1.1 = [(-49_isize),(-55_isize),(-9223372036854775808_isize),(-3_isize),(-23_isize),98_isize,9223372036854775807_isize];
_1.1.1.0 = _5.1.0 - _7.1.0;
match _18.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
44871 => bb8,
_ => bb7
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
Return()
}
bb7 = {
Return()
}
bb8 = {
RET = [(-9223372036854775808_isize),9223372036854775807_isize,26_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_17 = Adt58::Variant3 { fld0: (-224473905_i32),fld1: 1221091638_u32 };
_8.1.1.1 = _4.1.1;
_4.1.1 = _1.1.1.1;
_8.2 = core::ptr::addr_of_mut!(_19);
_18.1 = !62985_u16;
_3.1.1.0 = 8510212917149559285_usize as i16;
_11.2 = core::ptr::addr_of_mut!(_19);
_10.0 = [802461024315253265_u64,1995652606891374695_u64];
_4.1 = _8.1.1;
_12 = 4733752089379609371_u64 as isize;
_15 = -_12;
_20.2 = core::ptr::addr_of_mut!(_19);
_20.1.1.1 = [_12,_15,_12,_12,_12,_15,_12];
_1.1.1.1 = [_12,_15,_12,_12,_15,_12,_12];
_10.1.1 = _8.1.1;
_3.1.0 = 13551100141906815827395157655953630983_u128 as f64;
_11.1.1 = _4.1;
_3.1.1.1 = _7.1.1;
_8.1.1 = _7.1;
_6.1.1 = (_1.1.1.0, _3.1.1.1);
_7.1.1 = _6.1.1.1;
_4 = (_6.1.0, _11.1.1);
_19 = 189979880311388707706828049486526821723_u128;
Goto(bb9)
}
bb9 = {
_4.0 = 704900697_i32 as f64;
_11.2 = _10.2;
_8.2 = core::ptr::addr_of_mut!(_19);
place!(Field::<i32>(Variant(_17, 3), 0)) = (-453272735_i32);
_15 = _12;
_11.2 = core::ptr::addr_of_mut!(_19);
_3 = _1;
_20.0 = [16752611053177013797_u64,2827843808376708596_u64];
_7.1.0 = _5.0 as i16;
_24.2 = _7.1;
_8.1.1 = _11.1.1;
_17 = Adt58::Variant3 { fld0: (-49372413_i32),fld1: 969071570_u32 };
_19 = 294651103769094588613826397908818602423_u128;
_6.2 = _3.2;
_8.1.1.1 = [_12,_12,_12,_15,_12,_12,_12];
_18.2 = ['\u{4e6be}','\u{3c777}','\u{3c0fb}','\u{102476}','\u{100758}','\u{53489}','\u{f1d0d}','\u{97bdb}'];
_11.1.1 = _24.2;
_10 = _3;
_20.1 = (_2, _7.1);
_8.1 = (_2, _5.1);
_8.0 = [18072073668508283288_u64,2369876065024210793_u64];
_24 = (9132457032296095981_i64, 15675242045306367685_usize, _20.1.1);
_15 = -_12;
_16.0 = 109_u8 + 106_u8;
Call(_4.1.0 = fn10(_11.1.0, _8.1, _20.1.1.0, _20.1, _3.1, _8, _3.1, _8.1, _6, _24.2, _24, _10.1.0, _20.1.1.0, _24.0, _8, _8.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_6.1.0 = -_20.1.0;
_15 = _12;
_17 = Adt58::Variant3 { fld0: (-522639472_i32),fld1: 2466326745_u32 };
_3.1.1.0 = (-15_i8) as i16;
_11.1.1.1 = [_12,_12,_12,_15,_15,_15,_12];
_13 = [2092240803_u32,1173102241_u32,4212702825_u32,754431582_u32];
_8.1.1.0 = _7.1.0;
_11 = (_20.0, _4, _8.2);
_7 = (_1.1.0, _11.1.1);
_11.1.0 = _1.1.0 * _8.1.0;
_20.0 = [7375989342524511289_u64,17546033174856757297_u64];
_3.1.1 = (_24.2.0, _8.1.1.1);
_20.1 = (_11.1.0, _11.1.1);
_6 = (_8.0, _8.1, _20.2);
_29.0 = !false;
_6.0 = _3.0;
place!(Field::<u32>(Variant(_17, 3), 1)) = !3567150086_u32;
Call(_20.2 = fn11(_24, _5, _24, _11.1.1.1, _11.1.1, _8, _24.0, _3.1.1, _2, _8, _3.1.1.0, _8, _6.1, _11.1.0, _24.0, _6.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_10.2 = core::ptr::addr_of_mut!(_19);
_24.2.0 = _16.0 as i16;
_8.1 = _4;
_8.2 = _3.2;
_10.1.1.1 = _6.1.1.1;
_3.1.1.1 = [_15,_12,_12,_15,_12,_12,_12];
_11.1.1 = (_8.1.1.0, _6.1.1.1);
_28 = _24.0;
_18.0 = _6.0;
_14 = [_18.1,_18.1,_18.1,_18.1,_18.1,_18.1,_18.1,_18.1];
_24.2.1 = [_15,_15,_12,_15,_15,_15,_12];
_11.1.1.0 = _4.1.0 + _7.1.0;
Goto(bb12)
}
bb12 = {
_29 = (false,);
_4.1.0 = _6.1.1.0 * _11.1.1.0;
Goto(bb13)
}
bb13 = {
_3.1.1.0 = -_4.1.0;
_11.0 = [12804803989450558925_u64,6591504768388327801_u64];
_18.1 = 45300_u16 << _7.1.0;
_18.0 = [5603720988593788875_u64,3975001422000334748_u64];
_25 = [(-433500564_i32)];
_20.1.1.1 = [_15,_12,_15,_12,_12,_12,_15];
_1.2 = core::ptr::addr_of_mut!(_33);
_35.1.1 = [_12,_12,_12,_15,_12,_12,_12];
_8.2 = _6.2;
_30 = (_6.1.1.0, _10.1.1.1);
_33 = !_19;
_7.1.0 = -_30.0;
_20.1.1 = _6.1.1;
_7.1.1 = _11.1.1.1;
_36 = _18.1;
_5 = (_10.1.0, _30);
_24.2.0 = !_6.1.1.0;
Goto(bb14)
}
bb14 = {
_6.2 = _11.2;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(9_usize, 29_usize, Move(_29), 30_usize, Move(_30), 33_usize, Move(_33), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(9_usize, 28_usize, Move(_28), 36_usize, Move(_36), 41_usize, _41, 41_usize, _41), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: f64,mut _2: (f64, (i16, [isize; 7])),mut _3: i16,mut _4: (f64, (i16, [isize; 7])),mut _5: (f64, (i16, [isize; 7])),mut _6: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _7: (f64, (i16, [isize; 7])),mut _8: (f64, (i16, [isize; 7])),mut _9: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _10: (i16, [isize; 7]),mut _11: (i64, usize, (i16, [isize; 7])),mut _12: f64,mut _13: i16,mut _14: i64,mut _15: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _16: (f64, (i16, [isize; 7]))) -> i16 {
mir! {
type RET = i16;
let _17: Adt50;
let _18: ();
let _19: ();
{
_15.1.1.1 = _16.1.1;
_11.0 = _14;
RET = !_3;
_15.1.0 = _12 + _6.1.0;
_16.1.1 = _10.1;
_15.1.1 = _11.2;
_4 = _16;
_6.1.0 = -_15.1.0;
_9.1.0 = -_8.0;
_15.1 = (_6.1.0, _10);
RET = _10.0;
_2.1.0 = (-1580010764_i32) as i16;
RET = !_3;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(10_usize, 14_usize, Move(_14), 11_usize, Move(_11), 19_usize, _19, 19_usize, _19), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: (i64, usize, (i16, [isize; 7])),mut _2: (f64, (i16, [isize; 7])),mut _3: (i64, usize, (i16, [isize; 7])),mut _4: [isize; 7],mut _5: (i16, [isize; 7]),mut _6: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _7: i64,mut _8: (i16, [isize; 7]),mut _9: f64,mut _10: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _11: i16,mut _12: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _13: (f64, (i16, [isize; 7])),mut _14: f64,mut _15: i64,mut _16: (f64, (i16, [isize; 7]))) -> *mut u128 {
mir! {
type RET = *mut u128;
let _17: (f64, (i16, [isize; 7]));
let _18: char;
let _19: Adt61;
let _20: [usize; 2];
let _21: f64;
let _22: (*const [bool; 8], [char; 8]);
let _23: f64;
let _24: Adt64;
let _25: isize;
let _26: [u64; 3];
let _27: f64;
let _28: [isize; 7];
let _29: f32;
let _30: i16;
let _31: ();
let _32: ();
{
_5.1 = _12.1.1.1;
_13.0 = _6.1.0 + _14;
_1.2 = _10.1.1;
_6.0 = [10324554504645012081_u64,1990461292280277567_u64];
_6.2 = _10.2;
_6.2 = _12.2;
_13.1.0 = _12.1.1.0;
_10.1.0 = _1.0 as f64;
_1 = _3;
_3.0 = _15 >> _16.1.0;
_12 = (_6.0, _13, _6.2);
_10 = (_12.0, _12.1, _6.2);
_5.0 = 17677_u16 as i16;
_3.1 = _1.1 - _1.1;
_17.1 = _8;
_2.1.0 = _16.1.0 >> _1.2.0;
_7 = 9845_u16 as i64;
_11 = _3.2.0 - _13.1.0;
RET = _10.2;
_5.1 = [(-115_isize),9223372036854775807_isize,35_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_16 = (_14, _10.1.1);
_1.0 = _3.0 ^ _15;
(*RET) = 202_u8 as u128;
_17.0 = 1611583825_u32 as f64;
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
9132457032296095981 => bb6,
_ => bb5
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
_7 = _1.0 * _15;
_3.1 = _1.1 + _1.1;
_17.0 = _13.0;
_12.1.1.0 = _13.1.0;
_1.0 = _16.1.0 as i64;
_8 = (_11, _16.1.1);
_10.0 = [198619046957937515_u64,2377600621768969666_u64];
_1.2.0 = _16.1.0;
_17.1.1 = [(-21_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),28_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = (_6.1.1.0, _17.1.1);
_10.0 = _12.0;
_7 = _3.0;
_8.0 = -_13.1.0;
(*RET) = 77342911362933830550161674518297220369_u128;
_10.1 = (_13.0, _13.1);
_10.0 = [11609365326556515054_u64,17363118182355228782_u64];
_15 = (*RET) as i64;
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_16.0 = -_17.0;
_17.1 = (_1.2.0, _2.1.1);
_10.2 = core::ptr::addr_of_mut!((*RET));
_5.0 = _8.0;
_10.1.1 = (_16.1.0, _16.1.1);
_18 = '\u{174f5}';
_13 = (_16.0, _1.2);
_3.1 = _1.1;
Call(_3 = fn12(_10.1.1, _16, _12.1.1, _1, _5, _12, _6, _10, _13.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_25 = 9223372036854775807_isize;
match _1.1 {
15675242045306367685 => bb8,
_ => bb3
}
}
bb8 = {
_12.1 = (_17.0, _10.1.1);
_12.2 = core::ptr::addr_of_mut!((*RET));
_13.1.0 = _5.0;
_3.0 = -_7;
_3.2.0 = _1.1 as i16;
_12.1.1.0 = _17.1.0;
_1 = (_7, _3.1, _2.1);
_1.1 = true as usize;
Goto(bb9)
}
bb9 = {
_1.2 = (_3.2.0, _17.1.1);
_10.1.1.1 = _5.1;
Goto(bb10)
}
bb10 = {
_1.1 = !_3.1;
_2.1.1 = _17.1.1;
_13.1 = _17.1;
_1.0 = _3.0 | _3.0;
_10.1.1.0 = false as i16;
match _25 {
0 => bb9,
1 => bb4,
2 => bb3,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
9223372036854775807 => bb16,
_ => bb15
}
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
Return()
}
bb15 = {
Return()
}
bb16 = {
_6.1.1 = (_13.1.0, _12.1.1.1);
(*RET) = !143605270228162554552078020852930298012_u128;
_12.1.0 = -_13.0;
_10.1.1.1 = [_25,_25,_25,_25,_25,_25,_25];
_24.fld0 = _13.1.0 < _2.1.0;
_3.2.1 = _12.1.1.1;
_1.2.1 = _2.1.1;
_28 = [_25,_25,_25,_25,_25,_25,_25];
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(11_usize, 1_usize, Move(_1), 11_usize, Move(_11), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(11_usize, 8_usize, Move(_8), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (i16, [isize; 7]),mut _2: (f64, (i16, [isize; 7])),mut _3: (i16, [isize; 7]),mut _4: (i64, usize, (i16, [isize; 7])),mut _5: (i16, [isize; 7]),mut _6: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _7: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _8: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _9: (i16, [isize; 7])) -> (i64, usize, (i16, [isize; 7])) {
mir! {
type RET = (i64, usize, (i16, [isize; 7]));
let _10: [u64; 3];
let _11: *const i32;
let _12: u8;
let _13: bool;
let _14: ();
let _15: ();
{
_6.1.1 = (_2.1.0, _8.1.1.1);
_5.0 = 12012122363691543899_u64 as i16;
RET.2.0 = _3.0 ^ _2.1.0;
_2.1 = (_4.2.0, _7.1.1.1);
RET.2.1 = [(-9223372036854775808_isize),114_isize,(-9223372036854775808_isize),60_isize,(-67_isize),(-9223372036854775808_isize),67_isize];
_9 = RET.2;
RET.1 = (-9223372036854775808_isize) as usize;
_7.1.1.1 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-56_isize)];
_5.1 = _2.1.1;
RET.2 = (_6.1.1.0, _7.1.1.1);
_3.0 = (-18817164032767102325291213951533495703_i128) as i16;
_4 = ((-2027033265120809526_i64), RET.1, _9);
match _4.0 {
340282366920938463461347574166647401930 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET.2.0 = _7.1.1.0;
_8.0 = [3219061486986551043_u64,13638917072491626948_u64];
_9 = (_1.0, _3.1);
_12 = 6935404110120290248_u64 as u8;
_7.0 = [1056038293081361083_u64,3252599198097209359_u64];
RET = (_4.0, _4.1, _7.1.1);
_6.0 = [5111567867535333410_u64,3610269240999895976_u64];
RET.1 = (-38_i8) as usize;
_13 = !true;
_8.1.1.0 = _7.1.1.0;
_6.1.1 = (_4.2.0, _4.2.1);
_1 = (_7.1.1.0, _2.1.1);
_13 = !false;
_7.1.1.1 = [20_isize,9223372036854775807_isize,(-43_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,118_isize];
RET.2.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-96_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.1 = _2.1.1;
Goto(bb3)
}
bb3 = {
Call(_14 = dump_var(12_usize, 12_usize, Move(_12), 13_usize, Move(_13), 9_usize, Move(_9), 15_usize, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: (f64, (i16, [isize; 7])),mut _2: (i16, [isize; 7]),mut _3: f64,mut _4: f64,mut _5: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _6: Adt65,mut _7: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _8: (i16, [isize; 7]),mut _9: f64,mut _10: [isize; 7],mut _11: f64,mut _12: ([u64; 3],),mut _13: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _14: (f64, (i16, [isize; 7]))) -> i8 {
mir! {
type RET = i8;
let _15: [i8; 8];
let _16: *const i64;
let _17: char;
let _18: Adt49;
let _19: isize;
let _20: char;
let _21: Adt56;
let _22: isize;
let _23: f64;
let _24: (u8,);
let _25: char;
let _26: u64;
let _27: Adt58;
let _28: i16;
let _29: isize;
let _30: ((i64, usize, (i16, [isize; 7])),);
let _31: ([i32; 1], *mut char, (i64, usize, (i16, [isize; 7])), i8, f32);
let _32: Adt54;
let _33: u8;
let _34: u32;
let _35: u32;
let _36: ((i64, usize, (i16, [isize; 7])),);
let _37: isize;
let _38: isize;
let _39: Adt61;
let _40: *const i64;
let _41: ();
let _42: ();
{
_6.fld0 = [3485299110261936027_usize,812904013422233955_usize];
RET = 5275127988543426690_u64 as i8;
_1.1.0 = _7.1.1.0;
_13.1.1 = _2;
_13 = (_7.0, _1, _5.2);
_6.fld1.1 = (_11, _2);
_7.1.1.1 = [9223372036854775807_isize,109_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _7.1.1;
_13.2 = _5.2;
_6.fld1.2 = _13.2;
_6.fld0 = [3_usize,15264678711717745714_usize];
_6.fld1.1.0 = 34804_u16 as f64;
_13.0 = _6.fld1.0;
_6.fld0 = [5_usize,10241288906583125641_usize];
_3 = -_9;
_13.1.1.1 = _1.1.1;
_13.1 = _1;
_8.1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),79_isize,(-75_isize),(-9223372036854775808_isize)];
_7.1.1.0 = !_8.0;
_1.1.0 = !_6.fld1.1.1.0;
_6.fld1.1.1 = _8;
_7.1 = _13.1;
_1.1.0 = _6.fld1.1.1.0 + _8.0;
_7.1.1.1 = [105_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-128_isize),(-9223372036854775808_isize),54_isize,(-9223372036854775808_isize)];
Goto(bb1)
}
bb1 = {
_1.1 = (_8.0, _6.fld1.1.1.1);
_4 = -_9;
_14.1 = _1.1;
_7.2 = _13.2;
_1.1.0 = _7.1.1.0 >> _14.1.0;
_6.fld1.1.1.0 = !_13.1.1.0;
_13.1.1.0 = 153313984017975764647113654822327822942_i128 as i16;
_5.1.0 = _13.1.0;
_7.1 = _1;
_14 = _13.1;
_9 = -_5.1.0;
_1.1.1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4 = _7.1.0;
Goto(bb2)
}
bb2 = {
_19 = _5.1.0 as isize;
_20 = '\u{671c9}';
_12.0 = [17475765376213690012_u64,9501689294653743274_u64,12146258180322341250_u64];
_6.fld1.1.1.0 = !_2.0;
_5.0 = [13105397591003336891_u64,4208170201823311751_u64];
_14.0 = _5.1.0;
_1.1.1 = _13.1.1.1;
_6.fld1.1.1.1 = _2.1;
_2.1 = [_19,_19,_19,_19,_19,_19,_19];
_13.1.1.0 = _1.1.0;
_6.fld1.2 = _5.2;
_15 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = _5.1.0 - _9;
_1 = _5.1;
_12.0 = [12385985554971481359_u64,17386359678931854003_u64,12379005583627927989_u64];
_21.fld1 = (_13.0, _7.1.1.0, _20);
_9 = _4;
_11 = RET as f64;
_7.1.1 = _2;
_1.1 = (_13.1.1.0, _2.1);
_13 = (_7.0, _14, _5.2);
Goto(bb3)
}
bb3 = {
_13 = _7;
_13.2 = _7.2;
_14 = _1;
_21.fld3 = !6_usize;
_1 = (_14.0, _8);
_18 = Adt49::Variant3 { fld0: 25_u8,fld1: _21.fld3,fld2: (-56966557952552499372787068244186349841_i128),fld3: _12.0 };
_22 = _21.fld1.1 as isize;
_1.1.1 = [_19,_19,_19,_19,_19,_19,_19];
_5.1.0 = _3 - _14.0;
_14.1.0 = _1.1.0 >> _19;
Goto(bb4)
}
bb4 = {
_2.0 = _19 as i16;
_2 = (_14.1.0, _1.1.1);
_1.1 = (_2.0, _6.fld1.1.1.1);
_21.fld3 = false as usize;
_13.1.1.0 = -_2.0;
_17 = _21.fld1.2;
_25 = _17;
_14.1 = _2;
_13.1.1 = _1.1;
_6.fld1.1.1.1 = [_19,_19,_19,_19,_19,_19,_19];
Call(_21.fld3 = fn14(_13, _7.1.1, _13.1.1, _5, _7.1, _7.1.1, _13.1.0, _6.fld1.1, _3, Move(_6)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_1 = (_3, _7.1.1);
_5.1.1.1 = [_19,_19,_19,_19,_19,_19,_22];
_7.1.1 = (_14.1.0, _1.1.1);
_10 = _2.1;
place!(Field::<u8>(Variant(_18, 3), 0)) = !112_u8;
_5.1.1.1 = _7.1.1.1;
_6.fld1.0 = _7.0;
_6.fld1.1.1.1 = _2.1;
_26 = 133186486839158175_u64;
_5.0 = [_26,_26];
_12 = (Field::<[u64; 3]>(Variant(_18, 3), 3),);
_13 = (_6.fld1.0, _14, _7.2);
_21.fld1.1 = _14.1.0;
_14 = _1;
_31.2.1 = !Field::<usize>(Variant(_18, 3), 1);
_6.fld1.1.1.0 = (-4504032418097608562_i64) as i16;
_7.1.1 = _6.fld1.1.1;
_13.1 = (_9, _5.1.1);
_7.2 = _13.2;
place!(Field::<usize>(Variant(_18, 3), 1)) = _21.fld3;
_5.1.1.1 = [_19,_19,_19,_19,_19,_19,_19];
_30.0.2 = (_2.0, _5.1.1.1);
_7.1.0 = 1760213685_i32 as f64;
_31.2 = ((-3658826227734509319_i64), Field::<usize>(Variant(_18, 3), 1), _7.1.1);
Goto(bb6)
}
bb6 = {
_24 = (Field::<u8>(Variant(_18, 3), 0),);
_28 = Field::<usize>(Variant(_18, 3), 1) as i16;
_30.0.1 = RET as usize;
_13.1.1.1 = [_19,_19,_19,_19,_19,_22,_19];
_31.2.0 = (-5003190205440500304_i64);
_1.1.1 = [_19,_19,_19,_19,_19,_19,_19];
_8 = (_21.fld1.1, _6.fld1.1.1.1);
_36.0 = _31.2;
_6.fld1 = _7;
_30.0.2 = (_21.fld1.1, _1.1.1);
_6.fld1.1.1 = (_2.0, _31.2.2.1);
_5.1.1.0 = !_6.fld1.1.1.0;
_15 = [RET,RET,RET,RET,RET,RET,RET,RET];
_36 = (_31.2,);
_31.1 = core::ptr::addr_of_mut!(_20);
_5.1.1.0 = _6.fld1.1.1.0 * _2.0;
_34 = 524127500_u32 >> _6.fld1.1.1.0;
_24.0 = Field::<u8>(Variant(_18, 3), 0) | Field::<u8>(Variant(_18, 3), 0);
_13.1.0 = -_4;
_1.1.0 = _30.0.2.0 << _30.0.2.0;
RET = (-65_i8) >> _2.0;
_30.0.2.1 = [_19,_19,_19,_19,_19,_19,_19];
_12 = (Field::<[u64; 3]>(Variant(_18, 3), 3),);
_12.0 = Field::<[u64; 3]>(Variant(_18, 3), 3);
Goto(bb7)
}
bb7 = {
Call(_41 = dump_var(13_usize, 2_usize, Move(_2), 22_usize, Move(_22), 12_usize, Move(_12), 8_usize, Move(_8)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_41 = dump_var(13_usize, 15_usize, Move(_15), 28_usize, Move(_28), 20_usize, Move(_20), 42_usize, _42), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _2: (i16, [isize; 7]),mut _3: (i16, [isize; 7]),mut _4: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _5: (f64, (i16, [isize; 7])),mut _6: (i16, [isize; 7]),mut _7: f64,mut _8: (f64, (i16, [isize; 7])),mut _9: f64,mut _10: Adt65) -> usize {
mir! {
type RET = usize;
let _11: i8;
let _12: f32;
let _13: [char; 8];
let _14: ([u64; 3],);
let _15: bool;
let _16: Adt59;
let _17: (bool,);
let _18: [u16; 8];
let _19: i64;
let _20: isize;
let _21: Adt50;
let _22: i16;
let _23: Adt49;
let _24: usize;
let _25: ([u64; 3],);
let _26: ([u64; 3],);
let _27: u128;
let _28: *const [bool; 8];
let _29: ();
let _30: ();
{
_2.0 = -_1.1.1.0;
_5.1 = (_1.1.1.0, _8.1.1);
Goto(bb1)
}
bb1 = {
_5.1 = (_3.0, _10.fld1.1.1.1);
_10.fld0 = [7_usize,13246166657293316714_usize];
_10.fld1.1.1 = _6;
_4.1.1.0 = !_1.1.1.0;
_5.1 = (_1.1.1.0, _8.1.1);
_10.fld1.1.0 = -_7;
_10.fld0 = [7867832226593649452_usize,4_usize];
_10.fld1 = _1;
_1.1.1.1 = [(-54_isize),(-75_isize),48_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_9 = 9946922668294319524_u64 as f64;
_4.1.0 = 141654311402999204496563213431634670869_i128 as f64;
_5.0 = 11_u8 as f64;
_11 = 379921979439003509_u64 as i8;
_6.0 = 239_u8 as i16;
_13 = ['\u{2d167}','\u{c0c1f}','\u{8262a}','\u{d60dd}','\u{6db53}','\u{86c1b}','\u{c02bc}','\u{365e4}'];
_1.1.1.0 = _3.0;
_4.2 = _1.2;
RET = 7_usize;
_6.1 = _5.1.1;
_8.0 = -_1.1.0;
_10.fld1 = _1;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
7 => bb7,
_ => bb6
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
Return()
}
bb7 = {
_6.0 = _10.fld1.1.1.0 >> _1.1.1.0;
_10.fld1.0 = _1.0;
_12 = 4764_u16 as f32;
Call(_11 = core::intrinsics::bswap((-100_i8)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1.1.1 = (_5.1.0, _6.1);
_10.fld1.2 = _4.2;
_5.1 = _8.1;
_5.0 = _10.fld1.1.0 * _8.0;
_4.0 = _10.fld1.0;
_1.1 = (_8.0, _4.1.1);
_10.fld0 = [RET,RET];
_8 = _10.fld1.1;
_9 = _8.0 - _1.1.0;
_12 = _11 as f32;
_10.fld1.1.1.1 = _5.1.1;
_16.fld1.1 = [true,false,false,true,false,true,false,true];
_4.0 = [827339907619700493_u64,11583037193518331786_u64];
_4.1.1 = _10.fld1.1.1;
_10.fld1.2 = _4.2;
_16.fld0[RET] = _11 as u16;
_4 = _10.fld1;
_2.0 = _11 as i16;
_10.fld1.1 = _5;
_6.1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-87_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_10.fld1.1.1.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_16.fld1.1[RET] = !false;
Call(_16.fld1.0 = fn15(_1.1, _1, _6.0, _10.fld1.1.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18 = [8134_u16,25481_u16,41328_u16,22125_u16,64686_u16,17318_u16,39712_u16,27563_u16];
_10.fld1.1.1.0 = _4.1.1.0;
_1.1.1.0 = (-519467438_i32) as i16;
_16.fld0 = [9111_u16,25468_u16,51104_u16,31034_u16,42242_u16,63055_u16,5574_u16,3388_u16];
_2.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-72_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_6 = _8.1;
_17 = (true,);
_1.0 = _10.fld1.0;
_13 = ['\u{d0329}','\u{2ffec}','\u{107b12}','\u{a466d}','\u{e3600}','\u{be7e}','\u{c7352}','\u{f50a1}'];
Goto(bb10)
}
bb10 = {
_10.fld1.2 = _1.2;
_3.0 = _10.fld1.1.1.0 | _8.1.0;
_1.1.0 = _5.0 * _8.0;
_9 = _8.0;
_2.0 = _8.1.0;
_10.fld1.1.0 = _4.1.0;
_6.1 = [(-125_isize),(-118_isize),63_isize,2_isize,42_isize,(-18_isize),100_isize];
_19 = '\u{a619c}' as i64;
_4.1.1.1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-36_isize),9223372036854775807_isize];
_10.fld1 = (_1.0, _1.1, _4.2);
_3.0 = _2.0 + _4.1.1.0;
_4.1 = _8;
_10.fld1.0 = [10428554539624826299_u64,17998878970722891707_u64];
_5.1.1 = [(-9223372036854775808_isize),4_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),34_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4 = _1;
match RET {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb8,
7 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_16.fld1.3 = core::ptr::addr_of!(_16.fld1.1);
_4.2 = _1.2;
_17 = (true,);
_16.fld0 = _18;
_14.0 = [12476730020898483706_u64,5484601653774323915_u64,387306741065516289_u64];
_9 = _19 as f64;
_16.fld0 = _18;
_12 = 12674064867531219940_u64 as f32;
_15 = !_17.0;
_5.0 = -_1.1.0;
_12 = 13461_u16 as f32;
_10.fld1.1.1 = (_8.1.0, _3.1);
_18 = [8829_u16,18106_u16,37383_u16,1693_u16,10488_u16,54883_u16,56376_u16,15940_u16];
_3 = (_8.1.0, _10.fld1.1.1.1);
_10.fld1.1.0 = _8.0;
_8.1 = (_10.fld1.1.1.0, _1.1.1.1);
_10.fld1.2 = _1.2;
match RET {
0 => bb13,
7 => bb15,
_ => bb14
}
}
bb13 = {
_1.1.1 = (_5.1.0, _6.1);
_10.fld1.2 = _4.2;
_5.1 = _8.1;
_5.0 = _10.fld1.1.0 * _8.0;
_4.0 = _10.fld1.0;
_1.1 = (_8.0, _4.1.1);
_10.fld0 = [RET,RET];
_8 = _10.fld1.1;
_9 = _8.0 - _1.1.0;
_12 = _11 as f32;
_10.fld1.1.1.1 = _5.1.1;
_16.fld1.1 = [true,false,false,true,false,true,false,true];
_4.0 = [827339907619700493_u64,11583037193518331786_u64];
_4.1.1 = _10.fld1.1.1;
_10.fld1.2 = _4.2;
_16.fld0[RET] = _11 as u16;
_4 = _10.fld1;
_2.0 = _11 as i16;
_10.fld1.1 = _5;
_6.1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-87_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_10.fld1.1.1.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_16.fld1.1[RET] = !false;
Call(_16.fld1.0 = fn15(_1.1, _1, _6.0, _10.fld1.1.0), ReturnTo(bb9), UnwindUnreachable())
}
bb14 = {
_6.0 = _10.fld1.1.1.0 >> _1.1.1.0;
_10.fld1.0 = _1.0;
_12 = 4764_u16 as f32;
Call(_11 = core::intrinsics::bswap((-100_i8)), ReturnTo(bb8), UnwindUnreachable())
}
bb15 = {
_4.1.1 = (_3.0, _10.fld1.1.1.1);
_24 = !RET;
_1.1.1.1 = [(-9223372036854775808_isize),1_isize,(-9223372036854775808_isize),(-70_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),93_isize];
_10.fld0 = [_24,_24];
_22 = _4.1.1.0 << _2.0;
_10.fld1.0 = [14955102645642142183_u64,13470643906617188053_u64];
_5.1.1 = [90_isize,9223372036854775807_isize,56_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-14_isize),9223372036854775807_isize];
_4 = _1;
_1.2 = _4.2;
_10.fld0 = [RET,_24];
_6.1 = [(-9223372036854775808_isize),(-57_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_8.1.0 = -_22;
_7 = -_5.0;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(14_usize, 2_usize, Move(_2), 18_usize, Move(_18), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(14_usize, 17_usize, Move(_17), 3_usize, Move(_3), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: (f64, (i16, [isize; 7])),mut _2: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _3: i16,mut _4: f64) -> i128 {
mir! {
type RET = i128;
let _5: (f64, (i16, [isize; 7]));
let _6: [u64; 2];
let _7: [i32; 1];
let _8: [bool; 8];
let _9: u128;
let _10: i128;
let _11: f64;
let _12: [usize; 2];
let _13: ([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128);
let _14: ((i64, usize, (i16, [isize; 7])),);
let _15: f64;
let _16: (i64, usize, (i16, [isize; 7]));
let _17: i128;
let _18: usize;
let _19: (f64, (i16, [isize; 7]));
let _20: i128;
let _21: ();
let _22: ();
{
_1.1.0 = !_3;
_4 = _2.1.0 + _2.1.0;
_1.1.0 = 5820259658644895227_u64 as i16;
_1.1.0 = _3 ^ _3;
RET = 45396394265022782035119333742706522049_i128;
_1 = (_2.1.0, _2.1.1);
_1.1.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-30_isize),9223372036854775807_isize];
_1 = (_2.1.0, _2.1.1);
_2.1.1.0 = !_3;
_5.1.0 = _1.1.0 + _2.1.1.0;
RET = -(-118857250932699261009020445910432713973_i128);
_5.1 = _1.1;
_1 = (_2.1.0, _2.1.1);
_1.0 = -_4;
_5 = _1;
RET = (-84999522218523922528889151363299334875_i128) + 51620484756980308768181950408870501177_i128;
_6 = [2832644705947948785_u64,5614965197660321459_u64];
RET = !114490268053711487261807655300294726355_i128;
_2.1.1.0 = _1.1.0;
RET = 4_usize as i128;
_3 = 219734312911530974391189157837228732942_u128 as i16;
RET = (-83_i8) as i128;
Goto(bb1)
}
bb1 = {
_5.1 = _2.1.1;
_2.0 = [12976245674580868187_u64,7253158741993404430_u64];
Call(_5.1.0 = core::intrinsics::transmute(_1.1.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.0 = _2.1.0 * _4;
_2.1.1 = (_5.1.0, _5.1.1);
_7 = [1161380878_i32];
RET = 6146815822747480708_i64 as i128;
_1.1.0 = _5.1.0 << _5.1.0;
_2.1.0 = -_1.0;
_5.1.1 = [105_isize,9223372036854775807_isize,9223372036854775807_isize,18_isize,(-86_isize),93_isize,9223372036854775807_isize];
_5.1.1 = _2.1.1.1;
_1 = (_2.1.0, _2.1.1);
_5.0 = (-6_i8) as f64;
_3 = !_1.1.0;
_2.1.1.0 = !_3;
_2.1.1.0 = _5.1.0;
_2.2 = core::ptr::addr_of_mut!(_9);
_8 = [false,true,true,true,true,true,false,false];
_9 = !28569297885765303209622167423587257969_u128;
_1.1.0 = 90_u8 as i16;
_5.0 = _2.1.0 + _2.1.0;
_5.1.1 = _2.1.1.1;
RET = 33768455737286725210091118872001791779_i128 - 34294154620954682230847455878824647907_i128;
_6 = _2.0;
Goto(bb3)
}
bb3 = {
_9 = _2.1.1.0 as u128;
_2.1 = (_4, _5.1);
Goto(bb4)
}
bb4 = {
_12 = [11838871086036284475_usize,5707324239694076398_usize];
_9 = 9223372036854775807_isize as u128;
_12 = [5_usize,2_usize];
_5.1.1 = [9223372036854775807_isize,9223372036854775807_isize,121_isize,(-5_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_2.2 = core::ptr::addr_of_mut!(_9);
_11 = _2.1.0;
_2.1.1 = _1.1;
_13.0 = [(-13_i8),67_i8,(-100_i8),(-127_i8),77_i8,112_i8,(-105_i8),(-15_i8)];
_13.1 = ((-6187650382552143725_i64), 4371068311453147281_usize, _5.1);
_5.0 = _9 as f64;
RET = (-81792846416556713806466833058748494304_i128);
_1.1.1 = [94_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_13.1.2 = _5.1;
_12 = [_13.1.1,_13.1.1];
_10 = RET;
_5.1.0 = _13.1.2.0;
RET = '\u{32db9}' as i128;
_13.3 = 59234_u16 as i128;
match _13.1.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463457186957049216067731 => bb10,
_ => bb9
}
}
bb5 = {
_9 = _2.1.1.0 as u128;
_2.1 = (_4, _5.1);
Goto(bb4)
}
bb6 = {
_1.0 = _2.1.0 * _4;
_2.1.1 = (_5.1.0, _5.1.1);
_7 = [1161380878_i32];
RET = 6146815822747480708_i64 as i128;
_1.1.0 = _5.1.0 << _5.1.0;
_2.1.0 = -_1.0;
_5.1.1 = [105_isize,9223372036854775807_isize,9223372036854775807_isize,18_isize,(-86_isize),93_isize,9223372036854775807_isize];
_5.1.1 = _2.1.1.1;
_1 = (_2.1.0, _2.1.1);
_5.0 = (-6_i8) as f64;
_3 = !_1.1.0;
_2.1.1.0 = !_3;
_2.1.1.0 = _5.1.0;
_2.2 = core::ptr::addr_of_mut!(_9);
_8 = [false,true,true,true,true,true,false,false];
_9 = !28569297885765303209622167423587257969_u128;
_1.1.0 = 90_u8 as i16;
_5.0 = _2.1.0 + _2.1.0;
_5.1.1 = _2.1.1.1;
RET = 33768455737286725210091118872001791779_i128 - 34294154620954682230847455878824647907_i128;
_6 = _2.0;
Goto(bb3)
}
bb7 = {
_5.1 = _2.1.1;
_2.0 = [12976245674580868187_u64,7253158741993404430_u64];
Call(_5.1.0 = core::intrinsics::transmute(_1.1.0), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_13.1.2.1 = _1.1.1;
_13.1.2.1 = [9223372036854775807_isize,108_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _13.3 & _10;
_13.1 = (3353136479707605539_i64, 6_usize, _5.1);
_14.0.1 = (-24_i8) as usize;
_2.1 = _5;
RET = _13.1.1 as i128;
_14.0.2.1 = _1.1.1;
_14.0 = (_13.1.0, _13.1.1, _13.1.2);
_2.1.1.1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),31_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_13.1.2.0 = RET as i16;
_14.0.2.1 = _1.1.1;
_9 = 75_i8 as u128;
RET = _13.3;
_4 = _11 + _11;
_1.1.0 = _14.0.2.0 * _5.1.0;
_14.0.2.1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),99_isize];
_11 = -_4;
_1.0 = -_4;
_13.1.2 = (_2.1.1.0, _5.1.1);
_2.2 = core::ptr::addr_of_mut!(_9);
RET = !_13.3;
_2.1.1 = _1.1;
_16.1 = _14.0.1;
_14 = (_13.1,);
_16 = (_13.1.0, _14.0.1, _14.0.2);
match _16.1 {
0 => bb4,
1 => bb2,
2 => bb11,
3 => bb12,
4 => bb13,
6 => bb15,
_ => bb14
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_12 = [11838871086036284475_usize,5707324239694076398_usize];
_9 = 9223372036854775807_isize as u128;
_12 = [5_usize,2_usize];
_5.1.1 = [9223372036854775807_isize,9223372036854775807_isize,121_isize,(-5_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_2.2 = core::ptr::addr_of_mut!(_9);
_11 = _2.1.0;
_2.1.1 = _1.1;
_13.0 = [(-13_i8),67_i8,(-100_i8),(-127_i8),77_i8,112_i8,(-105_i8),(-15_i8)];
_13.1 = ((-6187650382552143725_i64), 4371068311453147281_usize, _5.1);
_5.0 = _9 as f64;
RET = (-81792846416556713806466833058748494304_i128);
_1.1.1 = [94_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_13.1.2 = _5.1;
_12 = [_13.1.1,_13.1.1];
_10 = RET;
_5.1.0 = _13.1.2.0;
RET = '\u{32db9}' as i128;
_13.3 = 59234_u16 as i128;
match _13.1.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463457186957049216067731 => bb10,
_ => bb9
}
}
bb14 = {
_1.0 = _2.1.0 * _4;
_2.1.1 = (_5.1.0, _5.1.1);
_7 = [1161380878_i32];
RET = 6146815822747480708_i64 as i128;
_1.1.0 = _5.1.0 << _5.1.0;
_2.1.0 = -_1.0;
_5.1.1 = [105_isize,9223372036854775807_isize,9223372036854775807_isize,18_isize,(-86_isize),93_isize,9223372036854775807_isize];
_5.1.1 = _2.1.1.1;
_1 = (_2.1.0, _2.1.1);
_5.0 = (-6_i8) as f64;
_3 = !_1.1.0;
_2.1.1.0 = !_3;
_2.1.1.0 = _5.1.0;
_2.2 = core::ptr::addr_of_mut!(_9);
_8 = [false,true,true,true,true,true,false,false];
_9 = !28569297885765303209622167423587257969_u128;
_1.1.0 = 90_u8 as i16;
_5.0 = _2.1.0 + _2.1.0;
_5.1.1 = _2.1.1.1;
RET = 33768455737286725210091118872001791779_i128 - 34294154620954682230847455878824647907_i128;
_6 = _2.0;
Goto(bb3)
}
bb15 = {
_14.0.2 = (_13.1.2.0, _1.1.1);
_14.0.2.1 = [93_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_14.0.2 = _5.1;
_11 = _1.0 - _4;
_7 = [(-2059991712_i32)];
_1.1.1 = [9223372036854775807_isize,(-84_isize),(-9223372036854775808_isize),(-35_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_14.0.0 = _13.3 as i64;
_15 = -_11;
_19 = (_1.0, _1.1);
_13.3 = _16.1 as i128;
_14 = (_16,);
_19.0 = 1887700366_i32 as f64;
_2.1.1 = _5.1;
_12 = [_14.0.1,_16.1];
RET = _13.3 ^ _13.3;
_13.2 = 72_u8 as u64;
Goto(bb16)
}
bb16 = {
Call(_21 = dump_var(15_usize, 9_usize, Move(_9), 16_usize, Move(_16), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_21 = dump_var(15_usize, 3_usize, Move(_3), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: (i16, [isize; 7]),mut _2: (f64, (i16, [isize; 7])),mut _3: (i16, [isize; 7])) -> usize {
mir! {
type RET = usize;
let _4: [i8; 8];
let _5: ([u64; 2], u16, [char; 8], i128);
let _6: f32;
let _7: [isize; 7];
let _8: (f64, (i16, [isize; 7]));
let _9: usize;
let _10: f64;
let _11: ([u64; 2], u16, [char; 8], i128);
let _12: (i64, usize, (i16, [isize; 7]));
let _13: [u16; 8];
let _14: Adt58;
let _15: i128;
let _16: isize;
let _17: *const i32;
let _18: *const i32;
let _19: isize;
let _20: isize;
let _21: (f64, (i16, [isize; 7]));
let _22: u128;
let _23: ();
let _24: ();
{
RET = 37471_u16 as usize;
_1 = _2.1;
_2.1 = _1;
_2.0 = 47_i8 as f64;
_2.0 = 1950410000_i32 as f64;
_5.2 = ['\u{7f56d}','\u{eb957}','\u{a3e3d}','\u{4fe04}','\u{1cf22}','\u{6d04b}','\u{6b928}','\u{5068b}'];
_1.1 = [49_isize,69_isize,9223372036854775807_isize,(-90_isize),55_isize,(-9223372036854775808_isize),85_isize];
_4 = [(-54_i8),34_i8,38_i8,108_i8,(-19_i8),(-35_i8),(-38_i8),(-92_i8)];
Call(RET = fn17(_3, _2, _2, _1, _2.1.0, _3, _2.1, _2, _2.1, _1, _1, _1, _1, _3.0, _1.0, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5.3 = 1785873460499556128321812910713132471_i128 - (-16421775444504459740988482767516619360_i128);
_5.0 = [11704317891739053957_u64,7947022112980576319_u64];
_5.3 = -(-88485215120801955609862881829677798229_i128);
_2.0 = _5.3 as f64;
_5.2 = ['\u{c70bc}','\u{2e36e}','\u{43d05}','\u{56279}','\u{77f10}','\u{9b6eb}','\u{505dc}','\u{e6190}'];
RET = _2.0 as usize;
_1.1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,124_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_5.3 = -(-1012163949177573197150760956610945953_i128);
_2.0 = _1.0 as f64;
RET = !6_usize;
_2.1 = (_1.0, _3.1);
_5.0 = [11678531034284821442_u64,3006832942254844013_u64];
_8.1.1 = [(-21_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-15_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2.1.0 = _3.0 >> _1.0;
_4 = [(-96_i8),127_i8,113_i8,124_i8,61_i8,79_i8,(-52_i8),(-20_i8)];
_3 = (_1.0, _8.1.1);
Goto(bb2)
}
bb2 = {
_7 = [110_isize,9223372036854775807_isize,109_isize,53_isize,(-124_isize),(-14_isize),(-86_isize)];
_5.3 = 246055839429512660283150189769838025850_u128 as i128;
_8.1.0 = _1.0;
_1 = (_3.0, _3.1);
_5.1 = 53902_u16;
RET = 2_usize << _2.1.0;
_2.0 = 1355829991_u32 as f64;
_5.0 = [8260031029995129699_u64,12268380971790910637_u64];
RET = 6367455840130206617_usize;
_5.3 = (-84329494529918886721114268933092067221_i128) + 155166034118867689139158743529855126795_i128;
RET = 12631161195086593298_usize & 4_usize;
_12 = (8036156390736311137_i64, RET, _3);
_8 = _2;
_1 = (_12.2.0, _7);
Goto(bb3)
}
bb3 = {
_12.2.0 = _8.1.0;
_2 = (_8.0, _1);
_12 = ((-3178686319134577680_i64), RET, _1);
_3 = (_12.2.0, _8.1.1);
_2.1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_6 = 540410090_i32 as f32;
_2.1.1 = [9223372036854775807_isize,127_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,23_isize];
_4 = [(-117_i8),(-125_i8),(-108_i8),(-61_i8),(-35_i8),81_i8,73_i8,(-5_i8)];
_10 = -_8.0;
RET = _2.0 as usize;
_12.0 = -3315925206117220614_i64;
_10 = _2.0 + _2.0;
_2.1 = (_3.0, _12.2.1);
_8.1.1 = [(-41_isize),4_isize,65_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),28_isize];
_12.1 = RET - RET;
_11 = (_5.0, _5.1, _5.2, _5.3);
_6 = 5311838965746251602_u64 as f32;
Call(_11.1 = core::intrinsics::bswap(_5.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = (_5.0, _5.1, _5.2, _5.3);
_1.0 = 364575323_i32 as i16;
_8.1.0 = true as i16;
Goto(bb5)
}
bb5 = {
_12.2.0 = !_2.1.0;
_9 = _12.1;
_2.1.1 = [9223372036854775807_isize,9223372036854775807_isize,81_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_12.1 = !_9;
_8 = (_2.0, _3);
_8 = (_2.0, _12.2);
_16 = 9223372036854775807_isize;
_11 = (_5.0, _5.1, _5.2, _5.3);
_5.3 = _16 as i128;
_3.1 = _2.1.1;
_2.1.0 = !_8.1.0;
_12.2.0 = _6 as i16;
RET = _12.1;
_2.1 = _8.1;
_8.1.1 = [_16,_16,_16,_16,_16,_16,_16];
_12.0 = 9109997378872440241_i64 | (-192489826193117366_i64);
_11.0 = [9138932179347395232_u64,14293419003342321968_u64];
_8.0 = _2.0 + _2.0;
_8.1 = (_3.0, _1.1);
match _11.1 {
0 => bb2,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
53902 => bb13,
_ => bb12
}
}
bb6 = {
_11 = (_5.0, _5.1, _5.2, _5.3);
_1.0 = 364575323_i32 as i16;
_8.1.0 = true as i16;
Goto(bb5)
}
bb7 = {
_12.2.0 = _8.1.0;
_2 = (_8.0, _1);
_12 = ((-3178686319134577680_i64), RET, _1);
_3 = (_12.2.0, _8.1.1);
_2.1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_6 = 540410090_i32 as f32;
_2.1.1 = [9223372036854775807_isize,127_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,23_isize];
_4 = [(-117_i8),(-125_i8),(-108_i8),(-61_i8),(-35_i8),81_i8,73_i8,(-5_i8)];
_10 = -_8.0;
RET = _2.0 as usize;
_12.0 = -3315925206117220614_i64;
_10 = _2.0 + _2.0;
_2.1 = (_3.0, _12.2.1);
_8.1.1 = [(-41_isize),4_isize,65_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),28_isize];
_12.1 = RET - RET;
_11 = (_5.0, _5.1, _5.2, _5.3);
_6 = 5311838965746251602_u64 as f32;
Call(_11.1 = core::intrinsics::bswap(_5.1), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_7 = [110_isize,9223372036854775807_isize,109_isize,53_isize,(-124_isize),(-14_isize),(-86_isize)];
_5.3 = 246055839429512660283150189769838025850_u128 as i128;
_8.1.0 = _1.0;
_1 = (_3.0, _3.1);
_5.1 = 53902_u16;
RET = 2_usize << _2.1.0;
_2.0 = 1355829991_u32 as f64;
_5.0 = [8260031029995129699_u64,12268380971790910637_u64];
RET = 6367455840130206617_usize;
_5.3 = (-84329494529918886721114268933092067221_i128) + 155166034118867689139158743529855126795_i128;
RET = 12631161195086593298_usize & 4_usize;
_12 = (8036156390736311137_i64, RET, _3);
_8 = _2;
_1 = (_12.2.0, _7);
Goto(bb3)
}
bb9 = {
_5.3 = 1785873460499556128321812910713132471_i128 - (-16421775444504459740988482767516619360_i128);
_5.0 = [11704317891739053957_u64,7947022112980576319_u64];
_5.3 = -(-88485215120801955609862881829677798229_i128);
_2.0 = _5.3 as f64;
_5.2 = ['\u{c70bc}','\u{2e36e}','\u{43d05}','\u{56279}','\u{77f10}','\u{9b6eb}','\u{505dc}','\u{e6190}'];
RET = _2.0 as usize;
_1.1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,124_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_5.3 = -(-1012163949177573197150760956610945953_i128);
_2.0 = _1.0 as f64;
RET = !6_usize;
_2.1 = (_1.0, _3.1);
_5.0 = [11678531034284821442_u64,3006832942254844013_u64];
_8.1.1 = [(-21_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-15_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2.1.0 = _3.0 >> _1.0;
_4 = [(-96_i8),127_i8,113_i8,124_i8,61_i8,79_i8,(-52_i8),(-20_i8)];
_3 = (_1.0, _8.1.1);
Goto(bb2)
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
_12.0 = _5.3 as i64;
_8.1.1 = [_16,_16,_16,_16,_16,_16,_16];
_13 = [_5.1,_11.1,_11.1,_5.1,_5.1,_11.1,_5.1,_11.1];
_4 = [(-113_i8),(-57_i8),20_i8,47_i8,(-10_i8),(-34_i8),4_i8,89_i8];
_12.2.1 = _1.1;
_6 = RET as f32;
_14 = Adt58::Variant2 { fld0: _13,fld1: _11.0,fld2: _4,fld3: _6,fld4: _3.0 };
_8.1.1 = [_16,_16,_16,_16,_16,_16,_16];
_13 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1,_11.1,_5.1];
_12 = (6120989972597865725_i64, _9, _8.1);
_12.2.1 = [_16,_16,_16,_16,_16,_16,_16];
_5.1 = _11.1;
_7 = [_16,_16,_16,_16,_16,_16,_16];
_5 = (Field::<[u64; 2]>(Variant(_14, 2), 1), _11.1, _11.2, _11.3);
_8.1.1 = _2.1.1;
_2.1 = _12.2;
_7 = _1.1;
place!(Field::<f32>(Variant(_14, 2), 3)) = _6;
_12.0 = (-5756672650636747572_i64) * 248336656746850601_i64;
_1 = (_2.1.0, _7);
RET = !_9;
_8 = (_10, _3);
_3.1 = _1.1;
_13 = Field::<[u16; 8]>(Variant(_14, 2), 0);
_13 = [_11.1,_11.1,_11.1,_5.1,_11.1,_11.1,_11.1,_11.1];
Call(_2 = fn18(_12.2, _14, _12.2, _1, _3, _1, _8, _12.2, _1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_11.3 = _5.3;
_3 = (Field::<i16>(Variant(_14, 2), 4), _2.1.1);
_12.2.1 = [_16,_16,_16,_16,_16,_16,_16];
_5.1 = _11.1 % _11.1;
_5.0 = [12259446553013383452_u64,14944056183969600742_u64];
_1.0 = _12.2.0;
_19 = _16 - _16;
_11.1 = _5.1 + _5.1;
_4 = Field::<[i8; 8]>(Variant(_14, 2), 2);
_19 = _16;
_12.2.1 = [_19,_19,_19,_19,_16,_16,_16];
_5 = _11;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(16_usize, 13_usize, Move(_13), 19_usize, Move(_19), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(16_usize, 9_usize, Move(_9), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: (i16, [isize; 7]),mut _2: (f64, (i16, [isize; 7])),mut _3: (f64, (i16, [isize; 7])),mut _4: (i16, [isize; 7]),mut _5: i16,mut _6: (i16, [isize; 7]),mut _7: (i16, [isize; 7]),mut _8: (f64, (i16, [isize; 7])),mut _9: (i16, [isize; 7]),mut _10: (i16, [isize; 7]),mut _11: (i16, [isize; 7]),mut _12: (i16, [isize; 7]),mut _13: (i16, [isize; 7]),mut _14: i16,mut _15: i16,mut _16: (i16, [isize; 7])) -> usize {
mir! {
type RET = usize;
let _17: isize;
let _18: Adt57;
let _19: bool;
let _20: ();
let _21: ();
{
_3.0 = _2.0;
_16.0 = _3.1.0 | _10.0;
RET = 7_usize << _12.0;
_10 = (_9.0, _2.1.1);
_3 = (_8.0, _1);
_1 = _16;
_15 = RET as i16;
_15 = _8.0 as i16;
_3 = (_8.0, _16);
_3.1 = _9;
_12.0 = 5261889764427782963_u64 as i16;
_17 = 21_isize + (-9223372036854775808_isize);
_17 = -(-9223372036854775808_isize);
_3.1.0 = _1.0;
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(17_usize, 10_usize, Move(_10), 9_usize, Move(_9), 12_usize, Move(_12), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(17_usize, 13_usize, Move(_13), 7_usize, Move(_7), 17_usize, Move(_17), 21_usize, _21), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: (i16, [isize; 7]),mut _2: Adt58,mut _3: (i16, [isize; 7]),mut _4: (i16, [isize; 7]),mut _5: (i16, [isize; 7]),mut _6: (i16, [isize; 7]),mut _7: (f64, (i16, [isize; 7])),mut _8: (i16, [isize; 7]),mut _9: (i16, [isize; 7])) -> (f64, (i16, [isize; 7])) {
mir! {
type RET = (f64, (i16, [isize; 7]));
let _10: f32;
let _11: ((i64, usize, (i16, [isize; 7])),);
let _12: (i16, [isize; 7]);
let _13: ();
let _14: ();
{
SetDiscriminant(_2, 1);
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_2, 1), 0)).1.1.1 = _6.1;
RET.1.0 = -_7.1.0;
RET.0 = _7.0;
_5.0 = _7.1.0;
RET.1 = _5;
_5.1 = _6.1;
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_2, 1), 0)).1.1 = (_6.0, RET.1.1);
place!(Field::<([u64; 2], (f64, (i16, [isize; 7])), *mut u128)>(Variant(_2, 1), 0)).1.0 = 50_isize as f64;
RET.1 = (_1.0, _6.1);
_11.0.2.0 = !RET.1.0;
_8.1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,1_isize,9223372036854775807_isize];
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(18_usize, 5_usize, Move(_5), 6_usize, Move(_6), 9_usize, Move(_9), 14_usize, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: Adt49,mut _2: (i64, usize, (i16, [isize; 7])),mut _3: [isize; 7],mut _4: [u64; 2],mut _5: [isize; 7],mut _6: [isize; 7],mut _7: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),mut _8: Adt65,mut _9: bool) -> char {
mir! {
type RET = char;
let _10: [u64; 3];
let _11: ([u64; 3],);
let _12: bool;
let _13: usize;
let _14: (u8,);
let _15: f64;
let _16: i64;
let _17: ([usize; 2], u16, [usize; 2]);
let _18: isize;
let _19: (bool,);
let _20: char;
let _21: ((i64, usize, (i16, [isize; 7])),);
let _22: ([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128);
let _23: bool;
let _24: f32;
let _25: (u8,);
let _26: ();
let _27: ();
{
_7.1 = _8.fld1.1;
place!(Field::<[u64; 2]>(Variant(_1, 1), 0)) = _7.0;
_2.2.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
RET = '\u{d6a2a}';
_8.fld1.0 = [15178641812731633329_u64,13162201070135165685_u64];
_7.1.0 = _8.fld1.1.0 - _8.fld1.1.0;
_8.fld1.1.0 = -_7.1.0;
_8.fld1.0 = [2361443819273778292_u64,4841835042689585299_u64];
_8.fld1.2 = _7.2;
_2.0 = 4978671795002039134_i64 - 8199540261511321902_i64;
_2.2.1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-72_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8.fld1.1.1 = _2.2;
_7.0 = Field::<[u64; 2]>(Variant(_1, 1), 0);
_7.0 = [13860492563552762653_u64,11885421607199992730_u64];
place!(Field::<[isize; 7]>(Variant(_1, 1), 1)) = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-101_isize)];
_7.0 = [8075872287784017109_u64,10822882798403112839_u64];
_8.fld1.2 = _7.2;
_7.1.1.0 = 43_isize as i16;
_6 = Field::<[isize; 7]>(Variant(_1, 1), 1);
_8.fld1.2 = _7.2;
_4 = Field::<[u64; 2]>(Variant(_1, 1), 0);
_7 = _8.fld1;
place!(Field::<[u64; 2]>(Variant(_1, 1), 0)) = [13068459574158694102_u64,10628078905683463320_u64];
_8.fld1.0 = [9410852756890017322_u64,7995767845181069844_u64];
_7.1.0 = -_8.fld1.1.0;
_13 = RET as usize;
_5 = [9223372036854775807_isize,(-9223372036854775808_isize),(-38_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),35_isize];
_12 = _9;
Goto(bb1)
}
bb1 = {
_7.1.1.1 = _3;
_2.2 = _8.fld1.1.1;
SetDiscriminant(_1, 3);
_7.1.0 = -_8.fld1.1.0;
_7.1 = (_8.fld1.1.0, _8.fld1.1.1);
_2 = (5108667439403182386_i64, _13, _7.1.1);
_8.fld1.1 = (_7.1.0, _2.2);
Goto(bb2)
}
bb2 = {
_8.fld1.2 = _7.2;
_7.1 = _8.fld1.1;
_2 = ((-1494586120746424611_i64), _13, _7.1.1);
_15 = _8.fld1.1.0;
_8.fld1.1.0 = _7.1.0 - _7.1.0;
place!(Field::<usize>(Variant(_1, 3), 1)) = _2.1 * _2.1;
_8.fld1.0 = [2779634228983677274_u64,4734542385619957567_u64];
_14 = (67_u8,);
_17.1 = 45818_u16;
_12 = _7.1.0 < _7.1.0;
place!(Field::<i128>(Variant(_1, 3), 2)) = (-126303809736831213603845357232631822904_i128) >> _17.1;
match _2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463461880021311021786845 => bb7,
_ => bb6
}
}
bb3 = {
_7.1.1.1 = _3;
_2.2 = _8.fld1.1.1;
SetDiscriminant(_1, 3);
_7.1.0 = -_8.fld1.1.0;
_7.1 = (_8.fld1.1.0, _8.fld1.1.1);
_2 = (5108667439403182386_i64, _13, _7.1.1);
_8.fld1.1 = (_7.1.0, _2.2);
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
_18 = 101_isize | (-9223372036854775808_isize);
_17.0 = [_2.1,_2.1];
_8.fld1.0 = [6872929394146661311_u64,12884183501874446045_u64];
_17.2 = _8.fld0;
_2.0 = 1385342061533536621_i64 ^ (-4327715315924678229_i64);
place!(Field::<[u64; 3]>(Variant(_1, 3), 3)) = [7262074157545301394_u64,14020189462339347964_u64,14357184560448895539_u64];
_9 = _12;
_7.1 = _8.fld1.1;
Goto(bb8)
}
bb8 = {
place!(Field::<[u64; 3]>(Variant(_1, 3), 3)) = [9833607734771682762_u64,10188097894317129846_u64,3167260258614336676_u64];
place!(Field::<u8>(Variant(_1, 3), 0)) = _14.0;
_11 = (Field::<[u64; 3]>(Variant(_1, 3), 3),);
_2.0 = !1598864646881034825_i64;
SetDiscriminant(_1, 1);
_8.fld1.1.1.1 = _3;
_8.fld1.1.1 = (_2.2.0, _6);
_7.1.1.0 = _8.fld1.1.1.0;
_10 = [8509777715216491126_u64,12673519555551000559_u64,7755712163415801818_u64];
place!(Field::<[u64; 2]>(Variant(_1, 1), 0)) = _4;
_22.1.2 = (_7.1.1.0, _5);
_8.fld1.0 = [11036678413574024631_u64,2067600375672564412_u64];
_9 = !_12;
_21.0.0 = (-1158850744_i32) as i64;
_13 = _2.1;
_7 = (_8.fld1.0, _8.fld1.1, _8.fld1.2);
_8.fld1 = _7;
_21.0.1 = _13 | _2.1;
match _14.0 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
67 => bb16,
_ => bb15
}
}
bb9 = {
_18 = 101_isize | (-9223372036854775808_isize);
_17.0 = [_2.1,_2.1];
_8.fld1.0 = [6872929394146661311_u64,12884183501874446045_u64];
_17.2 = _8.fld0;
_2.0 = 1385342061533536621_i64 ^ (-4327715315924678229_i64);
place!(Field::<[u64; 3]>(Variant(_1, 3), 3)) = [7262074157545301394_u64,14020189462339347964_u64,14357184560448895539_u64];
_9 = _12;
_7.1 = _8.fld1.1;
Goto(bb8)
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
_7.1.1.1 = _3;
_2.2 = _8.fld1.1.1;
SetDiscriminant(_1, 3);
_7.1.0 = -_8.fld1.1.0;
_7.1 = (_8.fld1.1.0, _8.fld1.1.1);
_2 = (5108667439403182386_i64, _13, _7.1.1);
_8.fld1.1 = (_7.1.0, _2.2);
Goto(bb2)
}
bb14 = {
_8.fld1.2 = _7.2;
_7.1 = _8.fld1.1;
_2 = ((-1494586120746424611_i64), _13, _7.1.1);
_15 = _8.fld1.1.0;
_8.fld1.1.0 = _7.1.0 - _7.1.0;
place!(Field::<usize>(Variant(_1, 3), 1)) = _2.1 * _2.1;
_8.fld1.0 = [2779634228983677274_u64,4734542385619957567_u64];
_14 = (67_u8,);
_17.1 = 45818_u16;
_12 = _7.1.0 < _7.1.0;
place!(Field::<i128>(Variant(_1, 3), 2)) = (-126303809736831213603845357232631822904_i128) >> _17.1;
match _2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463461880021311021786845 => bb7,
_ => bb6
}
}
bb15 = {
_7.1.1.1 = _3;
_2.2 = _8.fld1.1.1;
SetDiscriminant(_1, 3);
_7.1.0 = -_8.fld1.1.0;
_7.1 = (_8.fld1.1.0, _8.fld1.1.1);
_2 = (5108667439403182386_i64, _13, _7.1.1);
_8.fld1.1 = (_7.1.0, _2.2);
Goto(bb2)
}
bb16 = {
_22.3 = (-23164515890084264328366533499453186671_i128) | 149641422237056629673073890611047980118_i128;
_21.0.2 = _8.fld1.1.1;
_21.0.2 = (_8.fld1.1.1.0, _2.2.1);
_9 = !_12;
_7.0 = _4;
_4 = [7925883265233386746_u64,1981704536262686752_u64];
_14.0 = _17.1 as u8;
_22.1.1 = !_21.0.1;
_16 = 6012424525476942259384930924818760710_u128 as i64;
_10 = [4566157702517557833_u64,4838460534470487581_u64,4676566144922767522_u64];
_22.1.2.1 = [_18,_18,_18,_18,_18,_18,_18];
_2 = (_16, _21.0.1, _8.fld1.1.1);
_7.1.1.1 = [_18,_18,_18,_18,_18,_18,_18];
_7.1.1.1 = [_18,_18,_18,_18,_18,_18,_18];
_21.0.2.1 = _8.fld1.1.1.1;
_8.fld1.2 = _7.2;
_25 = _14;
_7.1.1.0 = -_21.0.2.0;
_10 = [9665542624290620304_u64,64587131567269174_u64,10321464296965393878_u64];
_24 = 33928532151570154214601261116872935617_u128 as f32;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(19_usize, 13_usize, Move(_13), 14_usize, Move(_14), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(19_usize, 17_usize, Move(_17), 16_usize, Move(_16), 10_usize, Move(_10), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(7350972106412128638_u64), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-1_i8)), std::hint::black_box(2_usize), std::hint::black_box((-6174043606242121839_i64)));
                
            }
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: u8,
fld1: (i64, usize, (i16, [isize; 7])),
fld2: ([u64; 3],),
fld3: i8,
fld4: *const [bool; 8],
fld5: [usize; 2],
fld6: [i8; 8],
fld7: i128,

},
Variant1{
fld0: [u64; 2],
fld1: [isize; 7],

},
Variant2{
fld0: f32,
fld1: (char, i8),
fld2: [char; 8],
fld3: [u32; 4],
fld4: *const i64,
fld5: ([u64; 3],),

},
Variant3{
fld0: u8,
fld1: usize,
fld2: i128,
fld3: [u64; 3],

}}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: [isize; 7],
fld1: f64,
fld2: *const i64,
fld3: (bool,),
fld4: *const [u64; 3],
fld5: u128,

},
Variant1{
fld0: bool,
fld1: (i16, [isize; 7]),
fld2: *const i64,
fld3: [u64; 3],
fld4: (f64, (i16, [isize; 7])),
fld5: i32,
fld6: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),

},
Variant2{
fld0: (f64, (i16, [isize; 7])),
fld1: u32,
fld2: *const i64,
fld3: *const [u64; 3],
fld4: f32,
fld5: i32,
fld6: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),
fld7: u128,

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: (u8,),
fld1: i128,
fld2: [u32; 4],
fld3: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),
fld4: *mut char,
fld5: ([u64; 2], u16, [char; 8], i128),

},
Variant1{
fld0: u16,
fld1: char,
fld2: [u32; 4],
fld3: ([u64; 2], i16, char),
fld4: [u64; 3],
fld5: [bool; 8],
fld6: f64,
fld7: i128,

},
Variant2{
fld0: (i16, [isize; 7]),
fld1: [u64; 3],
fld2: [i8; 8],

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: bool,
fld1: (i16, [isize; 7]),
fld2: u128,
fld3: (bool,),
fld4: ([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128),
fld5: [u16; 8],

},
Variant1{
fld0: (u8,),
fld1: f32,
fld2: i128,
fld3: f64,

},
Variant2{
fld0: *const [u64; 3],
fld1: (u8,),
fld2: u32,
fld3: i64,
fld4: i16,

},
Variant3{
fld0: (i16, [isize; 7]),

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt53 {
Variant0{
fld0: ([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128),
fld1: *mut char,

},
Variant1{
fld0: [bool; 8],

}}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: ([usize; 2], u16, [usize; 2]),
fld1: Adt49,
fld2: isize,

},
Variant1{
fld0: bool,

},
Variant2{
fld0: *mut char,
fld1: (i64, usize, (i16, [isize; 7])),
fld2: u16,
fld3: [isize; 7],

},
Variant3{
fld0: ([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128),
fld1: *const i32,
fld2: [u64; 3],
fld3: i64,
fld4: (char, i8),
fld5: ([i32; 1], *mut char, (i64, usize, (i16, [isize; 7])), i8, f32),

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: [usize; 2],
fld1: u64,
fld2: *mut u128,
fld3: *const i64,
fld4: i64,
fld5: i128,

},
Variant1{
fld0: Adt51,
fld1: u64,

},
Variant2{
fld0: [bool; 8],
fld1: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),
fld2: [u64; 3],
fld3: *const i32,
fld4: [i8; 8],
fld5: ([u64; 3],),
fld6: (*const [bool; 8], [char; 8]),

},
Variant3{
fld0: ([u64; 3],),
fld1: char,
fld2: [u32; 4],
fld3: Adt49,
fld4: Adt50,

}}
#[derive(Debug)]
pub struct Adt56 {
fld0: Adt49,
fld1: ([u64; 2], i16, char),
fld2: [isize; 7],
fld3: usize,
fld4: *const i64,
fld5: i32,
fld6: Adt50,
}
#[derive(Debug)]
pub enum Adt57 {
Variant0{
fld0: [i8; 8],
fld1: (i64, usize, (i16, [isize; 7])),
fld2: [isize; 7],
fld3: ([u64; 2], u16, [char; 8], i128),
fld4: i16,
fld5: (i128, [bool; 8], [i32; 1], *const [bool; 8]),

},
Variant1{
fld0: i128,

},
Variant2{
fld0: Adt56,
fld1: i32,
fld2: i128,
fld3: f32,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt58 {
Variant0{
fld0: [u64; 3],
fld1: [i8; 8],

},
Variant1{
fld0: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),

},
Variant2{
fld0: [u16; 8],
fld1: [u64; 2],
fld2: [i8; 8],
fld3: f32,
fld4: i16,

},
Variant3{
fld0: i32,
fld1: u32,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt59 {
fld0: [u16; 8],
fld1: (i128, [bool; 8], [i32; 1], *const [bool; 8]),
}
#[derive(Debug)]
pub enum Adt60 {
Variant0{
fld0: Adt55,
fld1: u128,
fld2: *mut u128,

},
Variant1{
fld0: Adt49,
fld1: Adt54,
fld2: u32,
fld3: Adt56,
fld4: [bool; 8],
fld5: [u16; 8],
fld6: (i16, [isize; 7]),

},
Variant2{
fld0: Adt52,
fld1: [i8; 8],

}}
#[derive(Debug)]
pub enum Adt61 {
Variant0{
fld0: [usize; 2],
fld1: *const [u64; 3],
fld2: Adt50,
fld3: i8,
fld4: u128,
fld5: f32,
fld6: i64,

},
Variant1{
fld0: [char; 8],
fld1: *const [bool; 8],
fld2: i128,
fld3: ([i32; 1], *mut char, (i64, usize, (i16, [isize; 7])), i8, f32),
fld4: Adt59,
fld5: (i64, usize, (i16, [isize; 7])),

},
Variant2{
fld0: [usize; 2],
fld1: Adt55,

},
Variant3{
fld0: u8,
fld1: Adt51,
fld2: (bool,),
fld3: *const [u64; 3],
fld4: Adt53,
fld5: (*const [bool; 8], [char; 8]),
fld6: Adt52,

}}
#[derive(Debug)]
pub struct Adt62 {
fld0: [u16; 8],
fld1: Adt53,
fld2: [usize; 2],
fld3: ([i8; 8], (i64, usize, (i16, [isize; 7])), u64, i128),
}
#[derive(Debug)]
pub struct Adt63 {
fld0: Adt54,
fld1: Adt56,
}
#[derive(Debug)]
pub struct Adt64 {
fld0: bool,
fld1: Adt52,
fld2: Adt55,
fld3: i8,
fld4: Adt61,
}
#[derive(Debug)]
pub struct Adt65 {
fld0: [usize; 2],
fld1: ([u64; 2], (f64, (i16, [isize; 7])), *mut u128),
}

