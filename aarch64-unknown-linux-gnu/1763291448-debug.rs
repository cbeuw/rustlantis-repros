#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics, lazy_get)]
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
pub fn fn0(mut _1: i64,mut _2: char,mut _3: u16,mut _4: u64) -> f32 {
mir! {
type RET = f32;
let _5: Adt31;
let _6: char;
let _7: char;
let _8: f64;
let _9: f32;
let _10: *const Adt18;
let _11: [char; 4];
let _12: bool;
let _13: isize;
let _14: i64;
let _15: *mut u128;
let _16: f64;
let _17: u32;
let _18: f64;
let _19: f64;
let _20: u64;
let _21: *const *const (i32,);
let _22: f64;
let _23: Adt38;
let _24: *mut f32;
let _25: char;
let _26: *const Adt18;
let _27: u128;
let _28: isize;
let _29: f32;
let _30: i8;
let _31: isize;
let _32: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _33: [i128; 3];
let _34: *mut bool;
let _35: ();
let _36: ();
{
_1 = (-4845008687440614377_i64) ^ (-8224985589383635576_i64);
_3 = 18216_u16 - 37126_u16;
RET = 984187816_i32 as f32;
_2 = '\u{87810}';
_4 = RET as u64;
RET = _1 as f32;
_6 = _2;
_3 = !19517_u16;
RET = _1 as f32;
_3 = 29663_u16;
_4 = 140517579_i32 as u64;
_4 = 14799950670703935327_u64 & 15104217224014283346_u64;
RET = 3796774988_u32 as f32;
_3 = !11588_u16;
Call(_7 = fn1(_1, _1, _6, _2, _4, _1, _2, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 4051227942335452216_u64 & 2816896615612372389_u64;
_2 = _6;
_1 = 13652574615966979576_usize as i64;
_4 = !4577502791915985872_u64;
_7 = _6;
_7 = _2;
_6 = _7;
_6 = _7;
_7 = _6;
_1 = 725440224335808737_i64 * 7760884049614858734_i64;
_6 = _2;
_4 = !6094272459067089507_u64;
_6 = _7;
_5 = Adt31::Variant1 { fld0: (-118_i8),fld1: 44107332847733878339026942651011967609_i128 };
_9 = -RET;
place!(Field::<i8>(Variant(_5, 1), 0)) = !83_i8;
_7 = _6;
Goto(bb2)
}
bb2 = {
_11 = [_6,_6,_7,_2];
_9 = RET + RET;
_4 = 8984311623061984716_u64 ^ 17061641661828935132_u64;
place!(Field::<i8>(Variant(_5, 1), 0)) = 2_usize as i8;
_8 = _3 as f64;
_3 = 7830_i16 as u16;
_6 = _7;
_9 = 9223372036854775807_isize as f32;
_12 = true & false;
place!(Field::<i128>(Variant(_5, 1), 1)) = (-114248169392445977799762752764368864223_i128);
_5 = Adt31::Variant1 { fld0: (-78_i8),fld1: (-60499361278704541249138290527774240855_i128) };
_2 = _6;
RET = -_9;
_9 = RET - RET;
_4 = 18050969138706806104_u64 >> _1;
place!(Field::<i8>(Variant(_5, 1), 0)) = 3771546322_u32 as i8;
_12 = !false;
place!(Field::<i128>(Variant(_5, 1), 1)) = 109515777071144716871506522786530983386_i128;
place!(Field::<i8>(Variant(_5, 1), 0)) = (-111_i8) >> _3;
_8 = _4 as f64;
_3 = 6_usize as u16;
_14 = _1;
place!(Field::<i8>(Variant(_5, 1), 0)) = 98_i8;
_12 = !true;
place!(Field::<i128>(Variant(_5, 1), 1)) = (-3020243020515651141606821205074678782_i128) ^ (-26746372778994296579585872196234633937_i128);
match Field::<i8>(Variant(_5, 1), 0) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
98 => bb10,
_ => bb9
}
}
bb3 = {
_4 = 4051227942335452216_u64 & 2816896615612372389_u64;
_2 = _6;
_1 = 13652574615966979576_usize as i64;
_4 = !4577502791915985872_u64;
_7 = _6;
_7 = _2;
_6 = _7;
_6 = _7;
_7 = _6;
_1 = 725440224335808737_i64 * 7760884049614858734_i64;
_6 = _2;
_4 = !6094272459067089507_u64;
_6 = _7;
_5 = Adt31::Variant1 { fld0: (-118_i8),fld1: 44107332847733878339026942651011967609_i128 };
_9 = -RET;
place!(Field::<i8>(Variant(_5, 1), 0)) = !83_i8;
_7 = _6;
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
Return()
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
RET = _9 + _9;
_13 = Field::<i8>(Variant(_5, 1), 0) as isize;
_13 = 217_u8 as isize;
RET = _4 as f32;
_12 = true | false;
_12 = _2 < _2;
place!(Field::<i8>(Variant(_5, 1), 0)) = _1 as i8;
_1 = 2113907520_i32 as i64;
_6 = _2;
place!(Field::<i8>(Variant(_5, 1), 0)) = (-36_i8) ^ 96_i8;
place!(Field::<i128>(Variant(_5, 1), 1)) = 99238936848693130417482581521676036273_i128 >> _14;
_9 = RET * RET;
_18 = _8 * _8;
_8 = _18 * _18;
_18 = _8 * _8;
_4 = 13016970449158394381_u64 + 12308572074726998958_u64;
_6 = _7;
_8 = _18;
_17 = 974117554_u32 * 3686898126_u32;
_19 = _18 + _18;
_19 = _18 - _18;
place!(Field::<i128>(Variant(_5, 1), 1)) = 169296504799862777236145344267886618128_i128;
Goto(bb11)
}
bb11 = {
_13 = 9223372036854775807_isize + 9223372036854775807_isize;
_16 = _19;
Goto(bb12)
}
bb12 = {
RET = _9;
_1 = _12 as i64;
_7 = _2;
_13 = -(-9223372036854775808_isize);
place!(Field::<i128>(Variant(_5, 1), 1)) = !(-139179764009239750425309828046052797793_i128);
place!(Field::<i128>(Variant(_5, 1), 1)) = (-152443524419084850742889943896847793287_i128) | 86174667032322769696926627768138974888_i128;
_20 = _4 & _4;
_3 = !4482_u16;
_22 = _18;
_18 = _16 + _19;
_22 = _18 + _18;
_11 = [_7,_7,_2,_7];
_4 = _20;
_9 = -RET;
_6 = _7;
_19 = _18 * _16;
_23.fld4 = !7_usize;
_17 = 104696090578231076674575924159538868097_u128 as u32;
_23.fld3 = core::ptr::addr_of_mut!(_3);
_23.fld2 = core::ptr::addr_of!(_12);
_23.fld0 = core::ptr::addr_of_mut!(_12);
_23.fld0 = core::ptr::addr_of_mut!(_12);
_23.fld0 = core::ptr::addr_of_mut!(_12);
_22 = (-1342565925_i32) as f64;
Goto(bb13)
}
bb13 = {
_1 = _14 + _14;
Goto(bb14)
}
bb14 = {
_23.fld4 = !5463281008958025945_usize;
_1 = _14 - _14;
_17 = !3173722377_u32;
_16 = _13 as f64;
_23.fld0 = core::ptr::addr_of_mut!(_12);
_15 = core::ptr::addr_of_mut!(_23.fld5);
_17 = !1010256092_u32;
_16 = -_18;
(*_15) = 116333795900555834558610629759913413595_u128;
_23.fld1 = _16 - _19;
_23.fld6 = _20 as i64;
_14 = _1 >> (*_15);
(*_15) = 316412465644552622385378370952392646090_u128;
_14 = (*_15) as i64;
place!(Field::<i128>(Variant(_5, 1), 1)) = (-112005734174788003569918005063606140330_i128) ^ (-86613601799762079847060264467285501874_i128);
_3 = 50784_u16 ^ 62632_u16;
(*_15) = _20 as u128;
_9 = RET;
_23.fld3 = core::ptr::addr_of_mut!(_3);
_16 = _19 + _19;
(*_15) = !57650920331322773852243966725564483456_u128;
_27 = !(*_15);
Goto(bb15)
}
bb15 = {
_1 = _23.fld6 ^ _14;
_16 = _19 * _18;
_7 = _2;
_19 = (-18365_i16) as f64;
_23.fld7 = [(-1829_i16),527_i16,(-19931_i16)];
(*_15) = !_27;
(*_15) = !_27;
Call((*_15) = core::intrinsics::transmute(_11), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_11 = [_2,_2,_6,_2];
_23.fld0 = core::ptr::addr_of_mut!(_12);
(*_15) = _27;
(*_15) = Field::<i8>(Variant(_5, 1), 0) as u128;
_23.fld4 = !3_usize;
_23.fld5 = _27 ^ _27;
(*_15) = _27;
Goto(bb17)
}
bb17 = {
(*_15) = 217_u8 as u128;
_20 = _4;
(*_15) = !_27;
place!(Field::<i8>(Variant(_5, 1), 0)) = 1302_i16 as i8;
_23.fld6 = _14 * _1;
_20 = _4 - _4;
_23.fld7 = [15661_i16,(-8766_i16),24133_i16];
_23.fld0 = core::ptr::addr_of_mut!(_12);
(*_15) = !_27;
_23.fld0 = core::ptr::addr_of_mut!(_12);
(*_15) = _27 * _27;
_29 = _23.fld4 as f32;
_31 = _13 & _13;
_17 = Field::<i128>(Variant(_5, 1), 1) as u32;
_17 = 116841973_u32;
_29 = RET - RET;
(*_15) = Field::<i128>(Variant(_5, 1), 1) as u128;
_23.fld1 = _16 + _18;
_25 = _7;
RET = _29 * _29;
_30 = Field::<i8>(Variant(_5, 1), 0) << (*_15);
Goto(bb18)
}
bb18 = {
Call(_35 = dump_var(0_usize, 31_usize, Move(_31), 2_usize, Move(_2), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(0_usize, 25_usize, Move(_25), 7_usize, Move(_7), 30_usize, Move(_30), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i64,mut _2: i64,mut _3: char,mut _4: char,mut _5: u64,mut _6: i64,mut _7: char,mut _8: u16,mut _9: u16) -> char {
mir! {
type RET = char;
let _10: bool;
let _11: *mut Adt31;
let _12: [char; 3];
let _13: i8;
let _14: f64;
let _15: [usize; 3];
let _16: f32;
let _17: char;
let _18: Adt38;
let _19: *mut f32;
let _20: f32;
let _21: [u16; 2];
let _22: &'static Adt38;
let _23: u8;
let _24: &'static mut Adt57;
let _25: ([char; 4], Adt57);
let _26: u128;
let _27: *mut f32;
let _28: *const bool;
let _29: (bool, u64, i8, u128);
let _30: isize;
let _31: [char; 4];
let _32: char;
let _33: &'static *const Adt18;
let _34: *const i8;
let _35: Adt27;
let _36: Adt27;
let _37: [usize; 3];
let _38: i8;
let _39: u32;
let _40: u64;
let _41: Adt23;
let _42: [char; 3];
let _43: char;
let _44: isize;
let _45: *const &'static mut i128;
let _46: char;
let _47: f32;
let _48: &'static mut i128;
let _49: isize;
let _50: f32;
let _51: i32;
let _52: i128;
let _53: (bool, u64, i8, u128);
let _54: &'static mut &'static usize;
let _55: isize;
let _56: (i16, *mut [usize; 3], u16);
let _57: u64;
let _58: [char; 3];
let _59: isize;
let _60: i16;
let _61: isize;
let _62: *mut bool;
let _63: Adt73;
let _64: *mut [i128; 4];
let _65: usize;
let _66: [i16; 3];
let _67: &'static mut isize;
let _68: u64;
let _69: *mut isize;
let _70: *const &'static mut i128;
let _71: &'static mut &'static usize;
let _72: (bool, u64, i8, u128);
let _73: bool;
let _74: i16;
let _75: (bool, u64, i8, u128);
let _76: &'static (bool, u64, i8, u128);
let _77: isize;
let _78: f64;
let _79: isize;
let _80: bool;
let _81: u128;
let _82: *mut *const bool;
let _83: ();
let _84: ();
{
RET = _4;
_1 = _2 + _2;
RET = _7;
_3 = RET;
_1 = _6 + _6;
_9 = _8;
_3 = _4;
_7 = _3;
_2 = _6 * _1;
_8 = 477241352_i32 as u16;
_3 = _7;
_8 = _9 ^ _9;
_6 = 2243816636_u32 as i64;
_4 = _7;
_10 = false;
_3 = _7;
_9 = !_8;
_12 = [_4,_4,_4];
_3 = _7;
_10 = _2 <= _1;
_3 = _7;
RET = _4;
_13 = -107_i8;
Call(_13 = fn2(_1, _3, _4, RET, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = [0_usize,1861192786039538340_usize,785786156574485049_usize];
_14 = (-69059442591136283555518628617093735302_i128) as f64;
_2 = _6;
_4 = _7;
_2 = (-1091506396_i32) as i64;
_17 = _7;
_18.fld5 = 283393512286914540940765485224269865862_u128;
_10 = true;
_18.fld4 = 4184662266_u32 as usize;
_18.fld2 = core::ptr::addr_of!(_10);
_1 = _6 << _13;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_13 = 81_i8 | (-42_i8);
_18.fld2 = core::ptr::addr_of!(_10);
_13 = 47_i8 - 40_i8;
_18.fld6 = _1 & _1;
_3 = _7;
_18.fld1 = _14 * _14;
_13 = 34_i8 ^ (-73_i8);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_15 = [_18.fld4,_18.fld4,_18.fld4];
_10 = _6 > _18.fld6;
_16 = 309385094_i32 as f32;
_4 = _17;
match _18.fld5 {
0 => bb2,
1 => bb3,
283393512286914540940765485224269865862 => bb5,
_ => bb4
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
_18.fld4 = 3_usize * 1_usize;
_18.fld2 = core::ptr::addr_of!(_10);
_12 = [_17,_4,_7];
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_18.fld2 = core::ptr::addr_of!(_10);
_7 = _3;
_7 = _4;
_6 = _1 * _18.fld6;
_13 = 59_i8 - 92_i8;
_18.fld4 = !5_usize;
_17 = _4;
_7 = _3;
_18.fld2 = core::ptr::addr_of!(_10);
_18.fld4 = 0_usize;
_7 = _4;
_5 = 10353592813953392329_u64 + 13064342791072572804_u64;
RET = _3;
_15 = [_18.fld4,_18.fld4,_18.fld4];
_7 = _3;
_13 = _18.fld5 as i8;
_15 = [_18.fld4,_18.fld4,_18.fld4];
_15 = [_18.fld4,_18.fld4,_18.fld4];
match _18.fld4 {
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb8,
0 => bb10,
_ => bb9
}
}
bb7 = {
_18.fld4 = 3_usize * 1_usize;
_18.fld2 = core::ptr::addr_of!(_10);
_12 = [_17,_4,_7];
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_15 = [0_usize,1861192786039538340_usize,785786156574485049_usize];
_14 = (-69059442591136283555518628617093735302_i128) as f64;
_2 = _6;
_4 = _7;
_2 = (-1091506396_i32) as i64;
_17 = _7;
_18.fld5 = 283393512286914540940765485224269865862_u128;
_10 = true;
_18.fld4 = 4184662266_u32 as usize;
_18.fld2 = core::ptr::addr_of!(_10);
_1 = _6 << _13;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_13 = 81_i8 | (-42_i8);
_18.fld2 = core::ptr::addr_of!(_10);
_13 = 47_i8 - 40_i8;
_18.fld6 = _1 & _1;
_3 = _7;
_18.fld1 = _14 * _14;
_13 = 34_i8 ^ (-73_i8);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_15 = [_18.fld4,_18.fld4,_18.fld4];
_10 = _6 > _18.fld6;
_16 = 309385094_i32 as f32;
_4 = _17;
match _18.fld5 {
0 => bb2,
1 => bb3,
283393512286914540940765485224269865862 => bb5,
_ => bb4
}
}
bb10 = {
_17 = _7;
_21 = [_8,_9];
_14 = -_18.fld1;
_8 = _9 + _9;
_17 = RET;
_6 = !_18.fld6;
_18.fld5 = 302764206047255712674979650961215356717_u128;
_23 = 232_u8 & 35_u8;
_18.fld6 = _1 * _6;
_18.fld3 = core::ptr::addr_of_mut!(_9);
_23 = 229_u8 - 49_u8;
Call(_20 = core::intrinsics::transmute(_21), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_18.fld7 = [(-30115_i16),21464_i16,25199_i16];
_6 = -_18.fld6;
_18.fld1 = _14;
_18.fld1 = _14 - _14;
_25.1.fld0.fld1 = _23;
_22 = &_18;
_26 = (*_22).fld5 & (*_22).fld5;
_25.0 = [_3,_7,RET,_7];
_13 = (-75_i8) ^ (-5_i8);
_18.fld4 = 8726126472177605596_usize - 1_usize;
_9 = _8 << (*_22).fld6;
_15 = [_18.fld4,_18.fld4,_18.fld4];
_15 = [_18.fld4,_18.fld4,_18.fld4];
_19 = core::ptr::addr_of_mut!(_20);
Goto(bb12)
}
bb12 = {
_25.1.fld1 = Move(_18.fld0);
_18.fld0 = Move(_25.1.fld1);
_26 = (*_22).fld5;
match (*_22).fld5 {
0 => bb6,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
302764206047255712674979650961215356717 => bb20,
_ => bb19
}
}
bb13 = {
_18.fld7 = [(-30115_i16),21464_i16,25199_i16];
_6 = -_18.fld6;
_18.fld1 = _14;
_18.fld1 = _14 - _14;
_25.1.fld0.fld1 = _23;
_22 = &_18;
_26 = (*_22).fld5 & (*_22).fld5;
_25.0 = [_3,_7,RET,_7];
_13 = (-75_i8) ^ (-5_i8);
_18.fld4 = 8726126472177605596_usize - 1_usize;
_9 = _8 << (*_22).fld6;
_15 = [_18.fld4,_18.fld4,_18.fld4];
_15 = [_18.fld4,_18.fld4,_18.fld4];
_19 = core::ptr::addr_of_mut!(_20);
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
_15 = [0_usize,1861192786039538340_usize,785786156574485049_usize];
_14 = (-69059442591136283555518628617093735302_i128) as f64;
_2 = _6;
_4 = _7;
_2 = (-1091506396_i32) as i64;
_17 = _7;
_18.fld5 = 283393512286914540940765485224269865862_u128;
_10 = true;
_18.fld4 = 4184662266_u32 as usize;
_18.fld2 = core::ptr::addr_of!(_10);
_1 = _6 << _13;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_13 = 81_i8 | (-42_i8);
_18.fld2 = core::ptr::addr_of!(_10);
_13 = 47_i8 - 40_i8;
_18.fld6 = _1 & _1;
_3 = _7;
_18.fld1 = _14 * _14;
_13 = 34_i8 ^ (-73_i8);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_15 = [_18.fld4,_18.fld4,_18.fld4];
_10 = _6 > _18.fld6;
_16 = 309385094_i32 as f32;
_4 = _17;
match _18.fld5 {
0 => bb2,
1 => bb3,
283393512286914540940765485224269865862 => bb5,
_ => bb4
}
}
bb16 = {
_15 = [0_usize,1861192786039538340_usize,785786156574485049_usize];
_14 = (-69059442591136283555518628617093735302_i128) as f64;
_2 = _6;
_4 = _7;
_2 = (-1091506396_i32) as i64;
_17 = _7;
_18.fld5 = 283393512286914540940765485224269865862_u128;
_10 = true;
_18.fld4 = 4184662266_u32 as usize;
_18.fld2 = core::ptr::addr_of!(_10);
_1 = _6 << _13;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_13 = 81_i8 | (-42_i8);
_18.fld2 = core::ptr::addr_of!(_10);
_13 = 47_i8 - 40_i8;
_18.fld6 = _1 & _1;
_3 = _7;
_18.fld1 = _14 * _14;
_13 = 34_i8 ^ (-73_i8);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_15 = [_18.fld4,_18.fld4,_18.fld4];
_10 = _6 > _18.fld6;
_16 = 309385094_i32 as f32;
_4 = _17;
match _18.fld5 {
0 => bb2,
1 => bb3,
283393512286914540940765485224269865862 => bb5,
_ => bb4
}
}
bb17 = {
_18.fld4 = 3_usize * 1_usize;
_18.fld2 = core::ptr::addr_of!(_10);
_12 = [_17,_4,_7];
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb18 = {
_18.fld2 = core::ptr::addr_of!(_10);
_7 = _3;
_7 = _4;
_6 = _1 * _18.fld6;
_13 = 59_i8 - 92_i8;
_18.fld4 = !5_usize;
_17 = _4;
_7 = _3;
_18.fld2 = core::ptr::addr_of!(_10);
_18.fld4 = 0_usize;
_7 = _4;
_5 = 10353592813953392329_u64 + 13064342791072572804_u64;
RET = _3;
_15 = [_18.fld4,_18.fld4,_18.fld4];
_7 = _3;
_13 = _18.fld5 as i8;
_15 = [_18.fld4,_18.fld4,_18.fld4];
_15 = [_18.fld4,_18.fld4,_18.fld4];
match _18.fld4 {
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb8,
0 => bb10,
_ => bb9
}
}
bb19 = {
_18.fld4 = 3_usize * 1_usize;
_18.fld2 = core::ptr::addr_of!(_10);
_12 = [_17,_4,_7];
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb20 = {
_25.1.fld2 = [(*_22).fld5,(*_22).fld5,(*_22).fld5,_18.fld5];
_26 = _18.fld4 as u128;
_25.1.fld0.fld1 = _23 << (*_22).fld6;
_4 = _17;
_3 = RET;
_18.fld1 = (*_22).fld6 as f64;
(*_19) = (-5655083492972967412066814754844154559_i128) as f32;
_25.1.fld0.fld1 = _23;
_18.fld2 = core::ptr::addr_of!(_10);
(*_19) = -_16;
(*_19) = _18.fld4 as f32;
(*_19) = -_16;
RET = _4;
(*_19) = -_16;
_25.1.fld1 = core::ptr::addr_of_mut!(_10);
_1 = _18.fld1 as i64;
_29.3 = (*_22).fld5 >> (*_22).fld6;
_17 = _7;
_17 = _7;
_31 = _25.0;
_25.0 = [_4,_4,RET,RET];
Goto(bb21)
}
bb21 = {
_34 = core::ptr::addr_of!(_13);
_14 = _18.fld1 - _18.fld1;
_32 = _3;
RET = _32;
_25.1.fld0.fld1 = _23;
_23 = !_25.1.fld0.fld1;
(*_19) = _16 * _16;
_3 = _32;
(*_19) = -_16;
(*_34) = (-24_i8);
_27 = core::ptr::addr_of_mut!((*_19));
(*_34) = !(-109_i8);
_21 = [_9,_9];
(*_19) = _16 - _16;
match (*_22).fld5 {
0 => bb22,
1 => bb23,
2 => bb24,
3 => bb25,
302764206047255712674979650961215356717 => bb27,
_ => bb26
}
}
bb22 = {
_17 = _7;
_21 = [_8,_9];
_14 = -_18.fld1;
_8 = _9 + _9;
_17 = RET;
_6 = !_18.fld6;
_18.fld5 = 302764206047255712674979650961215356717_u128;
_23 = 232_u8 & 35_u8;
_18.fld6 = _1 * _6;
_18.fld3 = core::ptr::addr_of_mut!(_9);
_23 = 229_u8 - 49_u8;
Call(_20 = core::intrinsics::transmute(_21), ReturnTo(bb11), UnwindUnreachable())
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
_15 = [0_usize,1861192786039538340_usize,785786156574485049_usize];
_14 = (-69059442591136283555518628617093735302_i128) as f64;
_2 = _6;
_4 = _7;
_2 = (-1091506396_i32) as i64;
_17 = _7;
_18.fld5 = 283393512286914540940765485224269865862_u128;
_10 = true;
_18.fld4 = 4184662266_u32 as usize;
_18.fld2 = core::ptr::addr_of!(_10);
_1 = _6 << _13;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_13 = 81_i8 | (-42_i8);
_18.fld2 = core::ptr::addr_of!(_10);
_13 = 47_i8 - 40_i8;
_18.fld6 = _1 & _1;
_3 = _7;
_18.fld1 = _14 * _14;
_13 = 34_i8 ^ (-73_i8);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_15 = [_18.fld4,_18.fld4,_18.fld4];
_10 = _6 > _18.fld6;
_16 = 309385094_i32 as f32;
_4 = _17;
match _18.fld5 {
0 => bb2,
1 => bb3,
283393512286914540940765485224269865862 => bb5,
_ => bb4
}
}
bb26 = {
_18.fld4 = 3_usize * 1_usize;
_18.fld2 = core::ptr::addr_of!(_10);
_12 = [_17,_4,_7];
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb27 = {
(*_19) = _16 - _16;
(*_19) = _16;
(*_34) = !105_i8;
_32 = _4;
_18.fld7 = [31250_i16,24552_i16,(-11251_i16)];
_10 = (*_22).fld6 >= (*_22).fld6;
_10 = true | true;
(*_34) = (-65_i8) - (-12_i8);
_29.3 = _26 % (*_22).fld5;
_40 = 11269_i16 as u64;
_16 = (*_19) + (*_19);
_37 = _15;
_28 = core::ptr::addr_of!(_29.0);
match (*_22).fld5 {
302764206047255712674979650961215356717 => bb28,
_ => bb5
}
}
bb28 = {
(*_28) = _10 ^ _10;
_4 = RET;
(*_28) = _10;
_14 = _18.fld1 + _18.fld1;
_18.fld2 = core::ptr::addr_of!((*_28));
_29.1 = !_5;
_1 = !(*_22).fld6;
_29.2 = _18.fld1 as i8;
_39 = 3794161120_u32;
_3 = RET;
_10 = !(*_28);
(*_34) = _29.2 >> (*_22).fld6;
_25.1.fld2 = [_29.3,_26,(*_22).fld5,_29.3];
_18.fld7 = [(-31921_i16),19946_i16,17588_i16];
_6 = -(*_22).fld6;
(*_34) = _14 as i8;
match (*_22).fld5 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb16,
4 => bb15,
5 => bb10,
302764206047255712674979650961215356717 => bb30,
_ => bb29
}
}
bb29 = {
Return()
}
bb30 = {
_14 = -_18.fld1;
_18.fld2 = core::ptr::addr_of!((*_28));
(*_28) = _10 | _10;
(*_28) = !_10;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_23 = _25.1.fld0.fld1 + _25.1.fld0.fld1;
_8 = _9;
_29 = (_10, _40, (*_34), (*_22).fld5);
(*_34) = _29.2;
Goto(bb31)
}
bb31 = {
(*_19) = _16;
_36 = Adt27::Variant0 { fld0: Move(_28),fld1: _9,fld2: (-9223372036854775808_isize),fld3: _13,fld4: (-25957_i16),fld5: _29,fld6: (*_22).fld6 };
Goto(bb32)
}
bb32 = {
(*_19) = _16;
_42 = _12;
_18.fld1 = _14;
_46 = _4;
(*_34) = _29.2 | Field::<i8>(Variant(_36, 0), 3);
place!(Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5)).0 = _29.0;
place!(Field::<i64>(Variant(_36, 0), 6)) = Field::<u16>(Variant(_36, 0), 1) as i64;
(*_34) = Field::<i8>(Variant(_36, 0), 3) >> Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
_17 = _46;
_44 = !9223372036854775807_isize;
_51 = 1498434802_i32;
_13 = Field::<i8>(Variant(_36, 0), 3) << (*_22).fld6;
(*_34) = Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2 * Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
_53 = (_29.0, _29.1, Field::<i8>(Variant(_36, 0), 3), (*_22).fld5);
_55 = _44 + _44;
Goto(bb33)
}
bb33 = {
_23 = !_25.1.fld0.fld1;
(*_34) = _29.2 & Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
_18.fld6 = -_6;
place!(Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5)).1 = _40 << _1;
(*_34) = Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2 * _29.2;
_47 = (-136630448727676111457592743157911255572_i128) as f32;
_56.2 = Field::<u16>(Variant(_36, 0), 1) >> (*_34);
_53 = (Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).0, Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).1, _29.2, (*_22).fld5);
_8 = !_56.2;
(*_19) = _47 - _47;
_31 = [_17,RET,_46,_4];
_52 = _18.fld6 as i128;
_29.2 = (*_34);
_18.fld6 = !_1;
_50 = (*_19);
Goto(bb34)
}
bb34 = {
(*_19) = _16;
_25.0 = [_46,_4,RET,RET];
_34 = core::ptr::addr_of!((*_34));
(*_34) = -Field::<i8>(Variant(_36, 0), 3);
place!(Field::<i64>(Variant(_36, 0), 6)) = _6 >> (*_34);
place!(Field::<i16>(Variant(_36, 0), 4)) = !16369_i16;
(*_34) = _39 as i8;
_57 = _53.1 >> _8;
_18.fld2 = core::ptr::addr_of!(place!(Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5)).0);
_18.fld7 = [Field::<i16>(Variant(_36, 0), 4),Field::<i16>(Variant(_36, 0), 4),Field::<i16>(Variant(_36, 0), 4)];
(*_19) = _47;
_58 = [_32,_32,_32];
_23 = _25.1.fld0.fld1 ^ _25.1.fld0.fld1;
_32 = _7;
place!(Field::<isize>(Variant(_36, 0), 2)) = -_44;
_50 = (*_19) * (*_19);
_42 = _58;
(*_34) = _29.2 & Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
place!(Field::<u16>(Variant(_36, 0), 1)) = _9 << (*_34);
_37 = [_18.fld4,_18.fld4,_18.fld4];
_58 = _42;
_43 = _32;
place!(Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5)).0 = _53.0;
(*_34) = Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
_44 = Field::<isize>(Variant(_36, 0), 2);
match (*_22).fld5 {
0 => bb19,
1 => bb25,
2 => bb3,
3 => bb15,
4 => bb28,
302764206047255712674979650961215356717 => bb35,
_ => bb6
}
}
bb35 = {
_25.1.fld0.fld1 = !_23;
_38 = _18.fld4 as i8;
Goto(bb36)
}
bb36 = {
_48 = &mut _52;
_34 = core::ptr::addr_of!((*_34));
(*_48) = (-76068824387652595497410050530643532384_i128) >> (*_34);
_17 = _32;
_66 = _18.fld7;
_14 = -_18.fld1;
_5 = _57 & _57;
_18.fld3 = core::ptr::addr_of_mut!(_9);
(*_19) = _16 + _50;
(*_34) = Field::<i8>(Variant(_36, 0), 3);
_18.fld3 = core::ptr::addr_of_mut!(_56.2);
_62 = core::ptr::addr_of_mut!(place!(Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5)).0);
(*_62) = !_53.0;
(*_48) = -49420982464740984694904193978245463963_i128;
(*_34) = -_53.2;
(*_62) = (*_34) <= (*_34);
(*_34) = Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2 & Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
_25.1.fld1 = core::ptr::addr_of_mut!((*_62));
(*_34) = _18.fld4 as i8;
(*_34) = _29.2 * _29.2;
_13 = _1 as i8;
_18.fld5 = _53.3 * Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).3;
place!(Field::<i16>(Variant(_36, 0), 4)) = (-18646_i16) ^ (-7505_i16);
match _29.3 {
0 => bb12,
1 => bb37,
2 => bb38,
3 => bb39,
4 => bb40,
5 => bb41,
302764206047255712674979650961215356717 => bb43,
_ => bb42
}
}
bb37 = {
_25.1.fld2 = [(*_22).fld5,(*_22).fld5,(*_22).fld5,_18.fld5];
_26 = _18.fld4 as u128;
_25.1.fld0.fld1 = _23 << (*_22).fld6;
_4 = _17;
_3 = RET;
_18.fld1 = (*_22).fld6 as f64;
(*_19) = (-5655083492972967412066814754844154559_i128) as f32;
_25.1.fld0.fld1 = _23;
_18.fld2 = core::ptr::addr_of!(_10);
(*_19) = -_16;
(*_19) = _18.fld4 as f32;
(*_19) = -_16;
RET = _4;
(*_19) = -_16;
_25.1.fld1 = core::ptr::addr_of_mut!(_10);
_1 = _18.fld1 as i64;
_29.3 = (*_22).fld5 >> (*_22).fld6;
_17 = _7;
_17 = _7;
_31 = _25.0;
_25.0 = [_4,_4,RET,RET];
Goto(bb21)
}
bb38 = {
_15 = [0_usize,1861192786039538340_usize,785786156574485049_usize];
_14 = (-69059442591136283555518628617093735302_i128) as f64;
_2 = _6;
_4 = _7;
_2 = (-1091506396_i32) as i64;
_17 = _7;
_18.fld5 = 283393512286914540940765485224269865862_u128;
_10 = true;
_18.fld4 = 4184662266_u32 as usize;
_18.fld2 = core::ptr::addr_of!(_10);
_1 = _6 << _13;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_13 = 81_i8 | (-42_i8);
_18.fld2 = core::ptr::addr_of!(_10);
_13 = 47_i8 - 40_i8;
_18.fld6 = _1 & _1;
_3 = _7;
_18.fld1 = _14 * _14;
_13 = 34_i8 ^ (-73_i8);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_15 = [_18.fld4,_18.fld4,_18.fld4];
_10 = _6 > _18.fld6;
_16 = 309385094_i32 as f32;
_4 = _17;
match _18.fld5 {
0 => bb2,
1 => bb3,
283393512286914540940765485224269865862 => bb5,
_ => bb4
}
}
bb39 = {
_15 = [0_usize,1861192786039538340_usize,785786156574485049_usize];
_14 = (-69059442591136283555518628617093735302_i128) as f64;
_2 = _6;
_4 = _7;
_2 = (-1091506396_i32) as i64;
_17 = _7;
_18.fld5 = 283393512286914540940765485224269865862_u128;
_10 = true;
_18.fld4 = 4184662266_u32 as usize;
_18.fld2 = core::ptr::addr_of!(_10);
_1 = _6 << _13;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_13 = 81_i8 | (-42_i8);
_18.fld2 = core::ptr::addr_of!(_10);
_13 = 47_i8 - 40_i8;
_18.fld6 = _1 & _1;
_3 = _7;
_18.fld1 = _14 * _14;
_13 = 34_i8 ^ (-73_i8);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_15 = [_18.fld4,_18.fld4,_18.fld4];
_10 = _6 > _18.fld6;
_16 = 309385094_i32 as f32;
_4 = _17;
match _18.fld5 {
0 => bb2,
1 => bb3,
283393512286914540940765485224269865862 => bb5,
_ => bb4
}
}
bb40 = {
_18.fld7 = [(-30115_i16),21464_i16,25199_i16];
_6 = -_18.fld6;
_18.fld1 = _14;
_18.fld1 = _14 - _14;
_25.1.fld0.fld1 = _23;
_22 = &_18;
_26 = (*_22).fld5 & (*_22).fld5;
_25.0 = [_3,_7,RET,_7];
_13 = (-75_i8) ^ (-5_i8);
_18.fld4 = 8726126472177605596_usize - 1_usize;
_9 = _8 << (*_22).fld6;
_15 = [_18.fld4,_18.fld4,_18.fld4];
_15 = [_18.fld4,_18.fld4,_18.fld4];
_19 = core::ptr::addr_of_mut!(_20);
Goto(bb12)
}
bb41 = {
(*_19) = _16;
_36 = Adt27::Variant0 { fld0: Move(_28),fld1: _9,fld2: (-9223372036854775808_isize),fld3: _13,fld4: (-25957_i16),fld5: _29,fld6: (*_22).fld6 };
Goto(bb32)
}
bb42 = {
_15 = [0_usize,1861192786039538340_usize,785786156574485049_usize];
_14 = (-69059442591136283555518628617093735302_i128) as f64;
_2 = _6;
_4 = _7;
_2 = (-1091506396_i32) as i64;
_17 = _7;
_18.fld5 = 283393512286914540940765485224269865862_u128;
_10 = true;
_18.fld4 = 4184662266_u32 as usize;
_18.fld2 = core::ptr::addr_of!(_10);
_1 = _6 << _13;
_18.fld3 = core::ptr::addr_of_mut!(_8);
_13 = 81_i8 | (-42_i8);
_18.fld2 = core::ptr::addr_of!(_10);
_13 = 47_i8 - 40_i8;
_18.fld6 = _1 & _1;
_3 = _7;
_18.fld1 = _14 * _14;
_13 = 34_i8 ^ (-73_i8);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_18.fld0 = core::ptr::addr_of_mut!(_10);
_15 = [_18.fld4,_18.fld4,_18.fld4];
_10 = _6 > _18.fld6;
_16 = 309385094_i32 as f32;
_4 = _17;
match _18.fld5 {
0 => bb2,
1 => bb3,
283393512286914540940765485224269865862 => bb5,
_ => bb4
}
}
bb43 = {
place!(Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5)).0 = _9 != _8;
(*_34) = !Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
(*_34) = _53.2;
_5 = _57 >> (*_34);
place!(Field::<u16>(Variant(_36, 0), 1)) = _56.2;
_18.fld3 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_36, 0), 1)));
_27 = core::ptr::addr_of_mut!((*_19));
(*_48) = (-86367117775006605182579579277575930377_i128) >> (*_34);
(*_19) = -_50;
_29.1 = Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).1 ^ _53.1;
(*_62) = _53.0 | _53.0;
_60 = Field::<i16>(Variant(_36, 0), 4) + Field::<i16>(Variant(_36, 0), 4);
_29 = ((*_62), _5, _13, _18.fld5);
(*_19) = _16 + _47;
(*_62) = !_10;
(*_62) = _29.0 | _53.0;
(*_62) = !_29.0;
place!(Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5)).3 = _18.fld5;
match _51 {
1498434802 => bb45,
_ => bb44
}
}
bb44 = {
Return()
}
bb45 = {
_38 = (*_34);
_67 = &mut _44;
_72.1 = !_29.1;
_70 = core::ptr::addr_of!(_48);
_53 = ((*_62), _5, (*_34), _26);
(*_34) = !_53.2;
place!(Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5)).1 = _72.1 >> (*_34);
(*_19) = _50 + _16;
match _39 {
3794161120 => bb47,
_ => bb46
}
}
bb46 = {
(*_28) = _10 ^ _10;
_4 = RET;
(*_28) = _10;
_14 = _18.fld1 + _18.fld1;
_18.fld2 = core::ptr::addr_of!((*_28));
_29.1 = !_5;
_1 = !(*_22).fld6;
_29.2 = _18.fld1 as i8;
_39 = 3794161120_u32;
_3 = RET;
_10 = !(*_28);
(*_34) = _29.2 >> (*_22).fld6;
_25.1.fld2 = [_29.3,_26,(*_22).fld5,_29.3];
_18.fld7 = [(-31921_i16),19946_i16,17588_i16];
_6 = -(*_22).fld6;
(*_34) = _14 as i8;
match (*_22).fld5 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb16,
4 => bb15,
5 => bb10,
302764206047255712674979650961215356717 => bb30,
_ => bb29
}
}
bb47 = {
(*_19) = -_50;
place!(Field::<isize>(Variant(_36, 0), 2)) = (*_67);
_20 = _18.fld4 as f32;
_70 = core::ptr::addr_of!((*_70));
(*_67) = _55;
_60 = Field::<i16>(Variant(_36, 0), 4) * Field::<i16>(Variant(_36, 0), 4);
_3 = _17;
_75.2 = _32 as i8;
(*_62) = (*_34) < (*_34);
_23 = _25.1.fld0.fld1;
_72.2 = (*_34) & (*_34);
_56.1 = core::ptr::addr_of_mut!(_37);
_29.3 = _18.fld5;
_78 = _14 * _18.fld1;
_65 = _18.fld4;
Goto(bb48)
}
bb48 = {
_18.fld5 = _26 | _29.3;
(*_34) = Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
(*_67) = Field::<isize>(Variant(_36, 0), 2) - _55;
_38 = (*_34) * (*_34);
_31 = [_7,_32,_17,_7];
_29 = ((*_62), _57, (*_34), _18.fld5);
_28 = core::ptr::addr_of!((*_62));
_68 = _72.1 + _72.1;
(*_19) = _50;
(*_67) = !Field::<isize>(Variant(_36, 0), 2);
_75.1 = _72.1 & _72.1;
(*_67) = _55 << (*_34);
Goto(bb49)
}
bb49 = {
_21 = [_56.2,_56.2];
(*_48) = -76053363187210023364251713665398597281_i128;
_10 = (*_19) >= (*_19);
(*_34) = Field::<i8>(Variant(_36, 0), 3);
(*_67) = !_55;
(*_34) = _53.1 as i8;
_75 = ((*_28), _5, (*_34), _29.3);
(*_28) = (*_34) == Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
_73 = (*_34) < _38;
_66 = [Field::<i16>(Variant(_36, 0), 4),Field::<i16>(Variant(_36, 0), 4),_60];
_26 = _53.3;
(*_19) = _16 + _50;
_40 = _5 + Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).1;
_18.fld0 = core::ptr::addr_of_mut!((*_28));
(*_28) = _73 & _75.0;
_75.2 = Field::<i8>(Variant(_36, 0), 3) << Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).2;
(*_62) = _75.0 | _73;
_29 = (Field::<(bool, u64, i8, u128)>(Variant(_36, 0), 5).0, _53.1, (*_34), _75.3);
_25.0 = [_3,_32,_32,_43];
_30 = (*_67) + (*_67);
_28 = core::ptr::addr_of!((*_62));
(*_19) = _50 * _50;
(*_67) = _30;
_80 = (*_28) <= (*_28);
(*_67) = _65 as isize;
_30 = (*_67) << _13;
_79 = _30 << Field::<i64>(Variant(_36, 0), 6);
_5 = _75.1 & _29.1;
(*_28) = !_75.0;
_25.1.fld1 = Move(_62);
Goto(bb50)
}
bb50 = {
Call(_83 = dump_var(1_usize, 1_usize, Move(_1), 2_usize, Move(_2), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_83 = dump_var(1_usize, 40_usize, Move(_40), 6_usize, Move(_6), 58_usize, Move(_58), 73_usize, Move(_73)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_83 = dump_var(1_usize, 46_usize, Move(_46), 53_usize, Move(_53), 12_usize, Move(_12), 43_usize, Move(_43)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_83 = dump_var(1_usize, 15_usize, Move(_15), 79_usize, Move(_79), 21_usize, Move(_21), 42_usize, Move(_42)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_83 = dump_var(1_usize, 65_usize, Move(_65), 29_usize, Move(_29), 51_usize, Move(_51), 75_usize, Move(_75)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_83 = dump_var(1_usize, 38_usize, Move(_38), 84_usize, _84, 84_usize, _84, 84_usize, _84), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i64,mut _2: char,mut _3: char,mut _4: char,mut _5: char) -> i8 {
mir! {
type RET = i8;
let _6: bool;
let _7: *mut Adt31;
let _8: Adt44;
let _9: u128;
let _10: u16;
let _11: *const *const (i32,);
let _12: &'static mut i128;
let _13: i128;
let _14: &'static mut *mut u16;
let _15: f32;
let _16: [usize; 3];
let _17: u8;
let _18: *mut isize;
let _19: &'static usize;
let _20: *mut &'static mut Adt27;
let _21: (i32,);
let _22: i64;
let _23: *const (i32,);
let _24: ();
let _25: ();
{
RET = (-24_i8);
RET = -49_i8;
RET = (-3_i8) >> _1;
_5 = _4;
_1 = (-7727076325687556801_i64) * (-3781824955560711169_i64);
_6 = false;
_6 = !false;
RET = !91_i8;
RET = 211_u8 as i8;
_5 = _2;
_5 = _3;
_3 = _5;
RET = 4_i8 - 56_i8;
_1 = 14_u8 as i64;
_4 = _5;
_8.fld1 = 130_u8 & 197_u8;
_4 = _3;
_1 = _6 as i64;
RET = 580134547_i32 as i8;
_3 = _4;
Call(_8 = fn3(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = Field::<bool>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 0) > Field::<bool>(Variant(_8.fld0, 0), 0);
_3 = _2;
place!(Field::<i128>(Variant(_8.fld0, 0), 1)) = (-95213436021906602922394920273965832053_i128);
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 3)) = _1 & _1;
place!(Field::<u64>(Variant(_8.fld0, 0), 3)) = Field::<u64>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 4) ^ Field::<u64>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 4);
_4 = _5;
Goto(bb2)
}
bb2 = {
_9 = !111367932666484728617545266542524291370_u128;
place!(Field::<f32>(Variant(_8.fld0, 0), 4)) = (-37_isize) as f32;
_4 = _2;
RET = !(-101_i8);
match Field::<i128>(Variant(_8.fld0, 0), 1) {
0 => bb1,
1 => bb3,
245068930899031860540979687157802379403 => bb5,
_ => bb4
}
}
bb3 = {
_6 = Field::<bool>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 0) > Field::<bool>(Variant(_8.fld0, 0), 0);
_3 = _2;
place!(Field::<i128>(Variant(_8.fld0, 0), 1)) = (-95213436021906602922394920273965832053_i128);
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 3)) = _1 & _1;
place!(Field::<u64>(Variant(_8.fld0, 0), 3)) = Field::<u64>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 4) ^ Field::<u64>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 4);
_4 = _5;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
place!(Field::<usize>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 1)) = 7_usize << _8.fld1;
RET = -92_i8;
_10 = 41056_u16 >> Field::<u64>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 4);
_10 = !61358_u16;
place!(Field::<i128>(Variant(_8.fld0, 0), 1)) = 120372670566689971285158919919857353921_i128 - 4714345160056980996238361267228499424_i128;
place!(Field::<u64>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 4)) = Field::<i64>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 3) as u64;
_1 = (-12454_i16) as i64;
_1 = !Field::<i64>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 3);
_2 = _3;
place!(Field::<f32>(Variant(_8.fld0, 0), 4)) = Field::<u64>(Variant(_8.fld0, 0), 3) as f32;
_3 = _5;
place!(Field::<u64>(Variant(_8.fld0, 0), 3)) = !Field::<u64>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 4);
_9 = Field::<f32>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 2) as u128;
RET = (-5_i8);
place!(Field::<bool>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 0)) = Field::<f32>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 2) <= Field::<f32>(Variant(Field::<Adt18>(Variant(_8.fld0, 0), 2), 0), 2);
place!(Field::<u64>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 4)) = _10 as u64;
place!(Field::<f32>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 2)) = -Field::<f32>(Variant(_8.fld0, 0), 4);
_2 = _4;
place!(Field::<f32>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 2)) = (-9223372036854775808_isize) as f32;
place!(Field::<usize>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 1)) = !2_usize;
place!(Field::<usize>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 1)) = 10971630268032990117_usize;
_5 = _3;
place!(Field::<f32>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 2)) = -Field::<f32>(Variant(_8.fld0, 0), 4);
place!(Field::<usize>(Variant(place!(Field::<Adt18>(Variant(_8.fld0, 0), 2)), 0), 1)) = 7_usize - 2896258801809063639_usize;
_12 = &mut place!(Field::<i128>(Variant(_8.fld0, 0), 1));
(*_12) = 139045452171690184884063325751569626937_i128 * 144853825913486309643974383288077464102_i128;
(*_12) = (-7571969470705593654277786969823470969_i128) << _9;
Goto(bb6)
}
bb6 = {
(*_12) = 88035100124272873737590707270812053430_i128;
(*_12) = 158354186334871565092218487831306368211_i128;
(*_12) = (-163638578691420262605002708724750925106_i128) << _9;
(*_12) = _6 as i128;
(*_12) = RET as i128;
Goto(bb7)
}
bb7 = {
(*_12) = 34471354864287170511072351412634103231_i128 - 90463762306976359685385586996363285301_i128;
_1 = 5628738599315730815_i64;
(*_12) = !3876784162611571221192263087942627248_i128;
_16 = [631366632408882460_usize,4870113769961343633_usize,6371350851142996563_usize];
RET = 23_i8;
_2 = _4;
(*_12) = -23507386293468892315812812444656613898_i128;
_3 = _4;
(*_12) = _10 as i128;
Goto(bb8)
}
bb8 = {
(*_12) = -70667752942669785862147827275284499060_i128;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
5628738599315730815 => bb10,
_ => bb9
}
}
bb9 = {
(*_12) = 34471354864287170511072351412634103231_i128 - 90463762306976359685385586996363285301_i128;
_1 = 5628738599315730815_i64;
(*_12) = !3876784162611571221192263087942627248_i128;
_16 = [631366632408882460_usize,4870113769961343633_usize,6371350851142996563_usize];
RET = 23_i8;
_2 = _4;
(*_12) = -23507386293468892315812812444656613898_i128;
_3 = _4;
(*_12) = _10 as i128;
Goto(bb8)
}
bb10 = {
(*_12) = -(-125578296157053240422929909414288581909_i128);
(*_12) = 29894971795816896903456454039302271524_i128;
Call((*_12) = core::intrinsics::bswap(137528214739288799878383545013579139424_i128), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_12) = 14946251552373809594869245967720365301_i128 | 39722215760447107945478660616959151909_i128;
_2 = _4;
(*_12) = (-38247179710412754196288073986658334272_i128);
Call(RET = core::intrinsics::transmute(_6), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_16 = [6_usize,16047824475453932255_usize,2_usize];
_4 = _3;
(*_12) = 29806_i16 as i128;
(*_12) = _9 as i128;
_16 = [4_usize,5700124990595997613_usize,0_usize];
_21 = (479524527_i32,);
_13 = (*_12) & (*_12);
(*_12) = -_13;
(*_12) = _13 >> RET;
_9 = 129748500526211705434057803558578757943_u128 & 217461548248463191885198932042843611656_u128;
_17 = !227_u8;
(*_12) = _13 * _13;
(*_12) = (-18742_i16) as i128;
Goto(bb13)
}
bb13 = {
Call(_24 = dump_var(2_usize, 21_usize, Move(_21), 2_usize, Move(_2), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_24 = dump_var(2_usize, 5_usize, Move(_5), 6_usize, Move(_6), 25_usize, _25, 25_usize, _25), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: char) -> Adt44 {
mir! {
type RET = Adt44;
let _2: [i128; 3];
let _3: &'static Adt57;
let _4: [u32; 6];
let _5: f32;
let _6: i8;
let _7: &'static mut isize;
let _8: &'static i32;
let _9: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _10: f32;
let _11: char;
let _12: &'static mut Adt57;
let _13: *const *const (i32,);
let _14: *const *const (i32,);
let _15: *mut &'static mut *mut u16;
let _16: f32;
let _17: f32;
let _18: (i32,);
let _19: bool;
let _20: isize;
let _21: &'static mut *mut u16;
let _22: Adt31;
let _23: [u128; 4];
let _24: u128;
let _25: (i16, *mut [usize; 3], u16);
let _26: (Adt23, u16, [i128; 4], &'static mut isize);
let _27: usize;
let _28: *const bool;
let _29: isize;
let _30: *mut u16;
let _31: &'static mut isize;
let _32: u128;
let _33: *mut i64;
let _34: u64;
let _35: *mut Adt31;
let _36: Adt23;
let _37: bool;
let _38: i64;
let _39: Adt44;
let _40: bool;
let _41: i128;
let _42: u128;
let _43: *mut &'static mut &'static usize;
let _44: *mut &'static mut Adt27;
let _45: f32;
let _46: (i32,);
let _47: bool;
let _48: (i16, *mut [usize; 3], u16);
let _49: u64;
let _50: f64;
let _51: char;
let _52: &'static mut i128;
let _53: *mut Adt31;
let _54: &'static *const Adt18;
let _55: *mut *mut [usize; 3];
let _56: [usize; 3];
let _57: u8;
let _58: f32;
let _59: f32;
let _60: isize;
let _61: [i8; 7];
let _62: i8;
let _63: bool;
let _64: *mut &'static mut *mut u16;
let _65: *mut Adt31;
let _66: *mut *mut [usize; 3];
let _67: *mut [usize; 3];
let _68: (i16, *mut [usize; 3], u16);
let _69: Adt73;
let _70: [i128; 3];
let _71: u64;
let _72: Adt18;
let _73: &'static usize;
let _74: i64;
let _75: *mut &'static mut *mut u16;
let _76: *mut &'static mut *mut u16;
let _77: char;
let _78: *mut u16;
let _79: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _80: bool;
let _81: usize;
let _82: bool;
let _83: &'static (bool, u64, i8, u128);
let _84: &'static *const Adt18;
let _85: isize;
let _86: &'static mut isize;
let _87: f32;
let _88: i128;
let _89: &'static *const Adt18;
let _90: Adt73;
let _91: *const *mut u16;
let _92: isize;
let _93: isize;
let _94: *mut *const bool;
let _95: u128;
let _96: isize;
let _97: Adt57;
let _98: *const Adt18;
let _99: char;
let _100: &'static mut i128;
let _101: isize;
let _102: &'static mut *mut u16;
let _103: Adt27;
let _104: [u128; 4];
let _105: *mut u16;
let _106: i16;
let _107: ([char; 4], Adt57);
let _108: u8;
let _109: f64;
let _110: [i16; 3];
let _111: bool;
let _112: [i32; 6];
let _113: *mut i64;
let _114: [u32; 6];
let _115: &'static mut Adt57;
let _116: isize;
let _117: *mut u16;
let _118: bool;
let _119: i8;
let _120: char;
let _121: usize;
let _122: *const *mut u16;
let _123: *mut *mut [usize; 3];
let _124: bool;
let _125: u32;
let _126: *mut [usize; 3];
let _127: f64;
let _128: u16;
let _129: *mut u16;
let _130: u32;
let _131: ();
let _132: ();
{
RET.fld1 = 65_u8 - 27_u8;
_1 = '\u{cdfe4}';
RET.fld1 = 76_u8;
RET.fld1 = _1 as u8;
_1 = '\u{59bfc}';
RET.fld1 = !140_u8;
_1 = '\u{5457}';
_1 = '\u{54bc1}';
RET.fld1 = !46_u8;
RET.fld1 = (-14550_i16) as u8;
_1 = '\u{992b0}';
_1 = '\u{672b0}';
RET.fld1 = 228_u8 >> 117_u8;
_1 = '\u{fe8ee}';
RET.fld1 = !221_u8;
RET.fld1 = 194_u8 * 70_u8;
_2 = [(-55982307178518619218639100798980223784_i128),(-1680433311456846349614372733836578663_i128),84610479921928132587792287395210548764_i128];
_2 = [(-96520735083004294224860906182347445052_i128),130542705608179641091590253383619243621_i128,(-164692631494502527095801406202234788834_i128)];
Goto(bb1)
}
bb1 = {
_1 = '\u{53da1}';
_2 = [28630466667240708864064223603617556719_i128,27106689419025912133505828533812762040_i128,(-46995881236269132447479323815774580110_i128)];
_1 = '\u{8f068}';
_1 = '\u{1d30}';
_1 = '\u{31a86}';
RET.fld1 = 180_u8 * 98_u8;
_4 = [1103410707_u32,3484423646_u32,2041754684_u32,3377810458_u32,850263334_u32,3445962694_u32];
_2 = [(-110086977713860668061279009021979462821_i128),55191348023081544867376818598343794433_i128,(-86344489119236703074392030685872959179_i128)];
_4 = [1114825465_u32,1499397140_u32,4076832632_u32,371620943_u32,3516162450_u32,1494612287_u32];
RET.fld1 = 123_u8 - 35_u8;
_1 = '\u{b7ddd}';
_5 = 16873_i16 as f32;
_1 = '\u{f75b6}';
_1 = '\u{4230e}';
RET.fld1 = 124_u8 - 29_u8;
RET.fld1 = !114_u8;
_1 = '\u{e34b8}';
_5 = 20943_u16 as f32;
_2 = [(-726975746872521154543416965255266596_i128),133788638423696482678063344625619990703_i128,(-142239776421841856162974971140854443198_i128)];
_2 = [(-96359043213529244647038589823622602095_i128),99256961868626277956538439150523053995_i128,(-154065860858967762789521479821155025242_i128)];
_6 = 60_i8;
_5 = (-12338_i16) as f32;
match _6 {
60 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_2 = [(-20777015297493681533908987839831622797_i128),(-158672695723480574854971691386605622027_i128),111318437810107773454349729786053241098_i128];
_4 = [1349695658_u32,555774666_u32,183466033_u32,1137483850_u32,1113849833_u32,789453906_u32];
_2 = [55309326640060312262840051986846310870_i128,100671010978427235923284208239977767717_i128,168800949703351717758887824257025204603_i128];
_1 = '\u{6b2f8}';
_6 = (-59_i8) - 122_i8;
_9.0 = core::ptr::addr_of!(_6);
_4 = [848952337_u32,3429292134_u32,1387415906_u32,2289750286_u32,1949351213_u32,1006459168_u32];
_11 = _1;
_4 = [2531741489_u32,3665067715_u32,449402790_u32,2631136250_u32,3442510484_u32,2852750696_u32];
_9.2 = (652683199_i32,);
_5 = _9.2.0 as f32;
_10 = _5 + _5;
_9.2.0 = (-279271740_i32) | 814336123_i32;
_1 = _11;
_4 = [87156040_u32,59019585_u32,3708558141_u32,2724693721_u32,4046996860_u32,2860082992_u32];
_11 = _1;
_1 = _11;
_1 = _11;
_2 = [126176086717322653734176335951139450034_i128,108847056159667279448826453705497901808_i128,75587527035038021002009790470055495843_i128];
_8 = &_9.2.0;
_4 = [360197844_u32,1989552582_u32,1839947657_u32,1309544135_u32,207766605_u32,2033672578_u32];
_2 = [106255824029278447595641503437642033244_i128,(-165949493394923581337643077759205229072_i128),(-97380948103643117401799187986745452128_i128)];
_9.0 = core::ptr::addr_of!(_6);
Goto(bb4)
}
bb4 = {
_10 = (-79_isize) as f32;
_1 = _11;
_4 = [1739365800_u32,2761653606_u32,2425783886_u32,4091069496_u32,108608458_u32,3063459883_u32];
_4 = [3200898137_u32,1298585255_u32,845665482_u32,4152799011_u32,4112753201_u32,2248246617_u32];
_6 = 8536437225256985414_u64 as i8;
_9.2.0 = true as i32;
_9.0 = core::ptr::addr_of!(_6);
_10 = _5;
_11 = _1;
_9.2 = (316451669_i32,);
_9.0 = core::ptr::addr_of!(_6);
_6 = 9223372036854775807_isize as i8;
_9.0 = core::ptr::addr_of!(_6);
_16 = _5;
_4 = [781984553_u32,3435475536_u32,2196143749_u32,3552331716_u32,3617137347_u32,550608449_u32];
RET.fld1 = 144_u8 - 126_u8;
_10 = -_16;
_11 = _1;
_4 = [2420346765_u32,879359807_u32,420044139_u32,3622052352_u32,2125996151_u32,3023880894_u32];
_4 = [469885305_u32,1788852850_u32,2553509897_u32,2580905363_u32,1933771664_u32,4258192609_u32];
_2 = [108751334972258821825109900990941382478_i128,(-3856280902367601202552317301393593643_i128),55140923505123054907121955295802567056_i128];
_17 = 9223372036854775807_isize as f32;
_1 = _11;
_1 = _11;
_17 = _5 + _16;
_10 = -_17;
Call(_17 = fn4(_9.2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_19 = true;
_5 = -_17;
RET.fld1 = !124_u8;
_6 = (-33_i8) - 55_i8;
_1 = _11;
RET.fld1 = _11 as u8;
_1 = _11;
_16 = _5;
_11 = _1;
_18.0 = _9.2.0 + _9.2.0;
_11 = _1;
_17 = (-9030383735673338436_i64) as f32;
_20 = (-69_isize) << _6;
Call(_10 = core::intrinsics::transmute(_11), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_9.2 = _18;
RET.fld1 = _19 as u8;
_17 = _16 + _5;
_20 = 72_isize ^ 9223372036854775807_isize;
_9.2 = _18;
_1 = _11;
_17 = (-67102423403462864108004881098986793717_i128) as f32;
_5 = _16;
_9.2 = (_18.0,);
_10 = (-137573177796909329283879747641513101273_i128) as f32;
_18.0 = _9.2.0;
_22 = Adt31::Variant1 { fld0: _6,fld1: 6144361597218019139956997171881515230_i128 };
_9.2 = (_18.0,);
_8 = &_18.0;
_18 = _9.2;
_10 = _16 * _16;
place!(Field::<i128>(Variant(_22, 1), 1)) = !(-44788605887484362287802209199229788816_i128);
_9.3 = &mut _20;
_9.2.0 = _1 as i32;
_10 = _5;
_1 = _11;
_1 = _11;
_11 = _1;
_19 = !true;
place!(Field::<i128>(Variant(_22, 1), 1)) = 33796943575159080484655355111141237534_i128 + (-8120373203531487053202179411453495009_i128);
_23 = [324490523639156045655241736476234775781_u128,133736590505713115245614250391508352529_u128,238588959863198685492507901544548145155_u128,3413738877416972190550232497297314901_u128];
_5 = -_10;
_6 = Field::<i8>(Variant(_22, 1), 0) << _18.0;
place!(Field::<i8>(Variant(_22, 1), 0)) = !_6;
_23 = [302706471303812847942989808878073792274_u128,177394319381288573389167203337522318314_u128,32405769187111315285568501243592538480_u128,44934086374305075603602070766929717023_u128];
Goto(bb7)
}
bb7 = {
_4 = [2995572274_u32,722562915_u32,3174989237_u32,629734903_u32,86469534_u32,181469838_u32];
_18.0 = 9842754533684040055_u64 as i32;
place!(Field::<i128>(Variant(_22, 1), 1)) = 48513052490380701621317529560601546108_i128 & (-113815144787646884875201032692970738555_i128);
_6 = Field::<i8>(Variant(_22, 1), 0) << Field::<i8>(Variant(_22, 1), 0);
_26.1 = !41965_u16;
_24 = 250341769542281441800788698821929627941_u128 * 308657790999568801703210597791159089737_u128;
_6 = _16 as i8;
_7 = Move(_9.3);
_18 = (_9.2.0,);
_18 = _9.2;
_26.1 = 20962_u16 << _6;
_5 = -_10;
_26.2 = [Field::<i128>(Variant(_22, 1), 1),Field::<i128>(Variant(_22, 1), 1),Field::<i128>(Variant(_22, 1), 1),Field::<i128>(Variant(_22, 1), 1)];
_27 = !2_usize;
_25.0 = 23450_i16 - 15742_i16;
_24 = 204449405145505254505555186582619502556_u128;
_9.0 = core::ptr::addr_of!(_6);
_18 = (_9.2.0,);
place!(Field::<i8>(Variant(_22, 1), 0)) = _6 - _6;
_27 = !1_usize;
_18 = (_9.2.0,);
_24 = 25096090406293211899749101503809717453_u128;
_29 = 9223372036854775807_isize >> Field::<i8>(Variant(_22, 1), 0);
Goto(bb8)
}
bb8 = {
_28 = core::ptr::addr_of!(_19);
_18.0 = _9.2.0 ^ _9.2.0;
(*_28) = !false;
match _24 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb9,
25096090406293211899749101503809717453 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
_9.2 = _18;
RET.fld1 = _19 as u8;
_17 = _16 + _5;
_20 = 72_isize ^ 9223372036854775807_isize;
_9.2 = _18;
_1 = _11;
_17 = (-67102423403462864108004881098986793717_i128) as f32;
_5 = _16;
_9.2 = (_18.0,);
_10 = (-137573177796909329283879747641513101273_i128) as f32;
_18.0 = _9.2.0;
_22 = Adt31::Variant1 { fld0: _6,fld1: 6144361597218019139956997171881515230_i128 };
_9.2 = (_18.0,);
_8 = &_18.0;
_18 = _9.2;
_10 = _16 * _16;
place!(Field::<i128>(Variant(_22, 1), 1)) = !(-44788605887484362287802209199229788816_i128);
_9.3 = &mut _20;
_9.2.0 = _1 as i32;
_10 = _5;
_1 = _11;
_1 = _11;
_11 = _1;
_19 = !true;
place!(Field::<i128>(Variant(_22, 1), 1)) = 33796943575159080484655355111141237534_i128 + (-8120373203531487053202179411453495009_i128);
_23 = [324490523639156045655241736476234775781_u128,133736590505713115245614250391508352529_u128,238588959863198685492507901544548145155_u128,3413738877416972190550232497297314901_u128];
_5 = -_10;
_6 = Field::<i8>(Variant(_22, 1), 0) << _18.0;
place!(Field::<i8>(Variant(_22, 1), 0)) = !_6;
_23 = [302706471303812847942989808878073792274_u128,177394319381288573389167203337522318314_u128,32405769187111315285568501243592538480_u128,44934086374305075603602070766929717023_u128];
Goto(bb7)
}
bb11 = {
(*_28) = false;
(*_28) = _16 >= _5;
(*_28) = !true;
_32 = _24;
Goto(bb12)
}
bb12 = {
_30 = core::ptr::addr_of_mut!(_26.1);
_30 = core::ptr::addr_of_mut!((*_30));
_4 = [3064277948_u32,4067002789_u32,2335035388_u32,728403300_u32,1631187622_u32,2011412734_u32];
(*_30) = 24739_u16 | 64904_u16;
(*_30) = 41934_u16 ^ 48120_u16;
_8 = &_9.2.0;
RET.fld1 = (*_28) as u8;
(*_30) = 63915_u16 >> _29;
(*_30) = !4352_u16;
_25.2 = (*_30) - (*_30);
_16 = _25.0 as f32;
_9.0 = core::ptr::addr_of!(_6);
_26.3 = &mut _29;
_6 = Field::<i8>(Variant(_22, 1), 0) | Field::<i8>(Variant(_22, 1), 0);
_30 = core::ptr::addr_of_mut!((*_30));
_9.2.0 = -_18.0;
(*_28) = false | false;
match _24 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb4,
25096090406293211899749101503809717453 => bb14,
_ => bb13
}
}
bb13 = {
_1 = '\u{53da1}';
_2 = [28630466667240708864064223603617556719_i128,27106689419025912133505828533812762040_i128,(-46995881236269132447479323815774580110_i128)];
_1 = '\u{8f068}';
_1 = '\u{1d30}';
_1 = '\u{31a86}';
RET.fld1 = 180_u8 * 98_u8;
_4 = [1103410707_u32,3484423646_u32,2041754684_u32,3377810458_u32,850263334_u32,3445962694_u32];
_2 = [(-110086977713860668061279009021979462821_i128),55191348023081544867376818598343794433_i128,(-86344489119236703074392030685872959179_i128)];
_4 = [1114825465_u32,1499397140_u32,4076832632_u32,371620943_u32,3516162450_u32,1494612287_u32];
RET.fld1 = 123_u8 - 35_u8;
_1 = '\u{b7ddd}';
_5 = 16873_i16 as f32;
_1 = '\u{f75b6}';
_1 = '\u{4230e}';
RET.fld1 = 124_u8 - 29_u8;
RET.fld1 = !114_u8;
_1 = '\u{e34b8}';
_5 = 20943_u16 as f32;
_2 = [(-726975746872521154543416965255266596_i128),133788638423696482678063344625619990703_i128,(-142239776421841856162974971140854443198_i128)];
_2 = [(-96359043213529244647038589823622602095_i128),99256961868626277956538439150523053995_i128,(-154065860858967762789521479821155025242_i128)];
_6 = 60_i8;
_5 = (-12338_i16) as f32;
match _6 {
60 => bb3,
_ => bb2
}
}
bb14 = {
_8 = &_18.0;
(*_30) = _25.2 | _25.2;
(*_30) = 7842127957122754554_u64 as u16;
_31 = Move(_26.3);
(*_30) = _25.2;
(*_28) = false;
place!(Field::<i8>(Variant(_22, 1), 0)) = _6 & _6;
(*_30) = !_25.2;
_32 = _24 % _24;
RET.fld1 = 190_u8 ^ 13_u8;
(*_30) = _25.2 << Field::<i8>(Variant(_22, 1), 0);
_6 = Field::<i8>(Variant(_22, 1), 0) | Field::<i8>(Variant(_22, 1), 0);
(*_30) = _25.2 * _25.2;
place!(Field::<i8>(Variant(_22, 1), 0)) = _6;
(*_28) = false & false;
Goto(bb15)
}
bb15 = {
_21 = &mut _30;
(*_28) = _6 != Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_27 = 5_usize << _6;
_42 = (*_28) as u128;
_23 = [_42,_42,_42,_42];
(*_28) = false & false;
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_25.2);
_16 = _10 * _10;
_6 = Field::<i8>(Variant(_22, 1), 0) * Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_42 = _32;
_42 = _25.0 as u128;
Call(_38 = core::intrinsics::bswap((-4937941133018140701_i64)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_45 = _5 * _5;
_34 = !14163302391832984850_u64;
_41 = Field::<i128>(Variant(_22, 1), 1) ^ Field::<i128>(Variant(_22, 1), 1);
_17 = _10 - _16;
(*_28) = Field::<i8>(Variant(_22, 1), 0) == Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_28) = false;
(*_28) = true;
_41 = Field::<i128>(Variant(_22, 1), 1) << _6;
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_48.2);
(*_28) = _5 >= _45;
_5 = _45 - _17;
_35 = core::ptr::addr_of_mut!(_22);
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 << Field::<i8>(Variant((*_35), 1), 0);
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41;
(*_28) = Field::<i128>(Variant((*_35), 1), 1) != Field::<i128>(Variant((*_35), 1), 1);
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 + _6;
match _24 {
0 => bb1,
1 => bb14,
2 => bb12,
3 => bb4,
4 => bb11,
5 => bb17,
25096090406293211899749101503809717453 => bb19,
_ => bb18
}
}
bb17 = {
_9.2 = _18;
RET.fld1 = _19 as u8;
_17 = _16 + _5;
_20 = 72_isize ^ 9223372036854775807_isize;
_9.2 = _18;
_1 = _11;
_17 = (-67102423403462864108004881098986793717_i128) as f32;
_5 = _16;
_9.2 = (_18.0,);
_10 = (-137573177796909329283879747641513101273_i128) as f32;
_18.0 = _9.2.0;
_22 = Adt31::Variant1 { fld0: _6,fld1: 6144361597218019139956997171881515230_i128 };
_9.2 = (_18.0,);
_8 = &_18.0;
_18 = _9.2;
_10 = _16 * _16;
place!(Field::<i128>(Variant(_22, 1), 1)) = !(-44788605887484362287802209199229788816_i128);
_9.3 = &mut _20;
_9.2.0 = _1 as i32;
_10 = _5;
_1 = _11;
_1 = _11;
_11 = _1;
_19 = !true;
place!(Field::<i128>(Variant(_22, 1), 1)) = 33796943575159080484655355111141237534_i128 + (-8120373203531487053202179411453495009_i128);
_23 = [324490523639156045655241736476234775781_u128,133736590505713115245614250391508352529_u128,238588959863198685492507901544548145155_u128,3413738877416972190550232497297314901_u128];
_5 = -_10;
_6 = Field::<i8>(Variant(_22, 1), 0) << _18.0;
place!(Field::<i8>(Variant(_22, 1), 0)) = !_6;
_23 = [302706471303812847942989808878073792274_u128,177394319381288573389167203337522318314_u128,32405769187111315285568501243592538480_u128,44934086374305075603602070766929717023_u128];
Goto(bb7)
}
bb18 = {
Return()
}
bb19 = {
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 ^ _6;
_50 = _26.1 as f64;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 & _41;
_9.0 = core::ptr::addr_of!(place!(Field::<i8>(Variant((*_35), 1), 0)));
_25.2 = _26.1;
place!(Field::<i8>(Variant((*_35), 1), 0)) = !_6;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 * _41;
(*_35) = Adt31::Variant1 { fld0: _6,fld1: _41 };
place!(Field::<i8>(Variant((*_35), 1), 0)) = !_6;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _11 as i128;
_28 = core::ptr::addr_of!((*_28));
match _24 {
0 => bb1,
1 => bb14,
2 => bb3,
3 => bb8,
4 => bb12,
5 => bb17,
25096090406293211899749101503809717453 => bb20,
_ => bb9
}
}
bb20 = {
(*_21) = core::ptr::addr_of_mut!(_25.2);
place!(Field::<i8>(Variant((*_35), 1), 0)) = -_6;
(*_21) = core::ptr::addr_of_mut!(_26.1);
match _24 {
0 => bb4,
1 => bb21,
25096090406293211899749101503809717453 => bb23,
_ => bb22
}
}
bb21 = {
Return()
}
bb22 = {
_21 = &mut _30;
(*_28) = _6 != Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_27 = 5_usize << _6;
_42 = (*_28) as u128;
_23 = [_42,_42,_42,_42];
(*_28) = false & false;
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_25.2);
_16 = _10 * _10;
_6 = Field::<i8>(Variant(_22, 1), 0) * Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_42 = _32;
_42 = _25.0 as u128;
Call(_38 = core::intrinsics::bswap((-4937941133018140701_i64)), ReturnTo(bb16), UnwindUnreachable())
}
bb23 = {
(*_35) = Adt31::Variant1 { fld0: _6,fld1: _41 };
(*_35) = Adt31::Variant1 { fld0: _6,fld1: _41 };
(*_28) = false;
_15 = core::ptr::addr_of_mut!(_21);
_37 = (*_28);
_46 = ((*_8),);
_19 = !_37;
_46.0 = _11 as i32;
_42 = Field::<i128>(Variant((*_35), 1), 1) as u128;
(*_21) = core::ptr::addr_of_mut!(_25.2);
RET.fld1 = 63_u8 << Field::<i128>(Variant((*_35), 1), 1);
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_28 = core::ptr::addr_of!((*_28));
_46.0 = (*_8);
_33 = core::ptr::addr_of_mut!(_38);
place!(Field::<i128>(Variant((*_35), 1), 1)) = (*_8) as i128;
_39.fld1 = RET.fld1;
_47 = !(*_28);
_48.0 = -_25.0;
Goto(bb24)
}
bb24 = {
(*_35) = Adt31::Variant1 { fld0: _6,fld1: _41 };
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6;
(*_21) = core::ptr::addr_of_mut!(_48.2);
(*_33) = 1710880439936062323_i64 ^ (-4321305471581488424_i64);
(*_35) = Adt31::Variant1 { fld0: _6,fld1: _41 };
(*_21) = core::ptr::addr_of_mut!(_48.2);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_34 = 6439504921043013121_u64 ^ 9048461692098245293_u64;
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 << Field::<i128>(Variant((*_35), 1), 1);
(*_33) = 3856128496496659574_i64 * (-3980437017191677930_i64);
_47 = Field::<i128>(Variant((*_35), 1), 1) >= Field::<i128>(Variant((*_35), 1), 1);
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 - _41;
_42 = _24 / _24;
_48.2 = _26.1 >> Field::<i128>(Variant((*_35), 1), 1);
(*_21) = core::ptr::addr_of_mut!(_25.2);
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 << Field::<i128>(Variant((*_35), 1), 1);
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 << Field::<i8>(Variant((*_35), 1), 0);
_48.1 = core::ptr::addr_of_mut!(_56);
place!(Field::<i128>(Variant((*_35), 1), 1)) = _11 as i128;
_52 = &mut place!(Field::<i128>(Variant((*_35), 1), 1));
_25.0 = _48.0;
(*_21) = core::ptr::addr_of_mut!(_26.1);
place!(Field::<i8>(Variant((*_35), 1), 0)) = -_6;
(*_28) = RET.fld1 >= RET.fld1;
(*_52) = _41 ^ _41;
(*_52) = _41 ^ _41;
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6;
match _24 {
0 => bb23,
1 => bb25,
25096090406293211899749101503809717453 => bb27,
_ => bb26
}
}
bb25 = {
_9.2 = _18;
RET.fld1 = _19 as u8;
_17 = _16 + _5;
_20 = 72_isize ^ 9223372036854775807_isize;
_9.2 = _18;
_1 = _11;
_17 = (-67102423403462864108004881098986793717_i128) as f32;
_5 = _16;
_9.2 = (_18.0,);
_10 = (-137573177796909329283879747641513101273_i128) as f32;
_18.0 = _9.2.0;
_22 = Adt31::Variant1 { fld0: _6,fld1: 6144361597218019139956997171881515230_i128 };
_9.2 = (_18.0,);
_8 = &_18.0;
_18 = _9.2;
_10 = _16 * _16;
place!(Field::<i128>(Variant(_22, 1), 1)) = !(-44788605887484362287802209199229788816_i128);
_9.3 = &mut _20;
_9.2.0 = _1 as i32;
_10 = _5;
_1 = _11;
_1 = _11;
_11 = _1;
_19 = !true;
place!(Field::<i128>(Variant(_22, 1), 1)) = 33796943575159080484655355111141237534_i128 + (-8120373203531487053202179411453495009_i128);
_23 = [324490523639156045655241736476234775781_u128,133736590505713115245614250391508352529_u128,238588959863198685492507901544548145155_u128,3413738877416972190550232497297314901_u128];
_5 = -_10;
_6 = Field::<i8>(Variant(_22, 1), 0) << _18.0;
place!(Field::<i8>(Variant(_22, 1), 0)) = !_6;
_23 = [302706471303812847942989808878073792274_u128,177394319381288573389167203337522318314_u128,32405769187111315285568501243592538480_u128,44934086374305075603602070766929717023_u128];
Goto(bb7)
}
bb26 = {
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 ^ _6;
_50 = _26.1 as f64;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 & _41;
_9.0 = core::ptr::addr_of!(place!(Field::<i8>(Variant((*_35), 1), 0)));
_25.2 = _26.1;
place!(Field::<i8>(Variant((*_35), 1), 0)) = !_6;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 * _41;
(*_35) = Adt31::Variant1 { fld0: _6,fld1: _41 };
place!(Field::<i8>(Variant((*_35), 1), 0)) = !_6;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _11 as i128;
_28 = core::ptr::addr_of!((*_28));
match _24 {
0 => bb1,
1 => bb14,
2 => bb3,
3 => bb8,
4 => bb12,
5 => bb17,
25096090406293211899749101503809717453 => bb20,
_ => bb9
}
}
bb27 = {
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 & _6;
(*_33) = !(-2523290524425832311_i64);
_61 = [Field::<i8>(Variant((*_35), 1), 0),Field::<i8>(Variant((*_35), 1), 0),Field::<i8>(Variant((*_35), 1), 0),Field::<i8>(Variant((*_35), 1), 0),Field::<i8>(Variant((*_35), 1), 0),Field::<i8>(Variant((*_35), 1), 0),Field::<i8>(Variant((*_35), 1), 0)];
(*_28) = _47;
_27 = 10247284689073300330_usize & 13503020817681657152_usize;
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 & _6;
_25.0 = _48.0 ^ _48.0;
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 - _6;
_39.fld1 = Field::<i8>(Variant((*_35), 1), 0) as u8;
_6 = 9223372036854775807_isize as i8;
_39.fld1 = RET.fld1;
_60 = 9223372036854775807_isize;
(*_33) = 5491029183702892031_i64;
(*_28) = !_47;
(*_21) = core::ptr::addr_of_mut!(_48.2);
(*_28) = _47;
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 + _6;
_2 = [(*_52),(*_52),_41];
_62 = -Field::<i8>(Variant((*_35), 1), 0);
(*_33) = -6959794214636150613_i64;
(*_33) = 2572377976003704573_i64;
_40 = !_19;
(*_33) = (-4404660781577121125_i64) << (*_52);
(*_52) = _41;
_26.2 = [(*_52),(*_52),(*_52),(*_52)];
place!(Field::<i8>(Variant((*_35), 1), 0)) = _62 * _6;
match _24 {
0 => bb25,
1 => bb23,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb21,
6 => bb9,
25096090406293211899749101503809717453 => bb29,
_ => bb28
}
}
bb28 = {
_28 = core::ptr::addr_of!(_19);
_18.0 = _9.2.0 ^ _9.2.0;
(*_28) = !false;
match _24 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb9,
25096090406293211899749101503809717453 => bb11,
_ => bb10
}
}
bb29 = {
(*_52) = _41 >> (*_33);
_57 = _25.0 as u8;
(*_33) = 4905069021134326621_i64;
_31 = &mut _60;
(*_33) = (-2184069058592418868_i64);
(*_31) = !(-9223372036854775808_isize);
_66 = core::ptr::addr_of_mut!(_48.1);
(*_33) = (-8153133837445694960_i64);
(*_31) = 118_isize - (-9223372036854775808_isize);
_7 = &mut (*_31);
_51 = _11;
_68.1 = core::ptr::addr_of_mut!(_56);
(*_7) = -9223372036854775807_isize;
match (*_33) {
0 => bb27,
1 => bb9,
2 => bb30,
3 => bb31,
340282366920938463455221473594322516496 => bb33,
_ => bb32
}
}
bb30 = {
_10 = (-79_isize) as f32;
_1 = _11;
_4 = [1739365800_u32,2761653606_u32,2425783886_u32,4091069496_u32,108608458_u32,3063459883_u32];
_4 = [3200898137_u32,1298585255_u32,845665482_u32,4152799011_u32,4112753201_u32,2248246617_u32];
_6 = 8536437225256985414_u64 as i8;
_9.2.0 = true as i32;
_9.0 = core::ptr::addr_of!(_6);
_10 = _5;
_11 = _1;
_9.2 = (316451669_i32,);
_9.0 = core::ptr::addr_of!(_6);
_6 = 9223372036854775807_isize as i8;
_9.0 = core::ptr::addr_of!(_6);
_16 = _5;
_4 = [781984553_u32,3435475536_u32,2196143749_u32,3552331716_u32,3617137347_u32,550608449_u32];
RET.fld1 = 144_u8 - 126_u8;
_10 = -_16;
_11 = _1;
_4 = [2420346765_u32,879359807_u32,420044139_u32,3622052352_u32,2125996151_u32,3023880894_u32];
_4 = [469885305_u32,1788852850_u32,2553509897_u32,2580905363_u32,1933771664_u32,4258192609_u32];
_2 = [108751334972258821825109900990941382478_i128,(-3856280902367601202552317301393593643_i128),55140923505123054907121955295802567056_i128];
_17 = 9223372036854775807_isize as f32;
_1 = _11;
_1 = _11;
_17 = _5 + _16;
_10 = -_17;
Call(_17 = fn4(_9.2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb31 = {
_28 = core::ptr::addr_of!(_19);
_18.0 = _9.2.0 ^ _9.2.0;
(*_28) = !false;
match _24 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb9,
25096090406293211899749101503809717453 => bb11,
_ => bb10
}
}
bb32 = {
_21 = &mut _30;
(*_28) = _6 != Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_27 = 5_usize << _6;
_42 = (*_28) as u128;
_23 = [_42,_42,_42,_42];
(*_28) = false & false;
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_25.2);
_16 = _10 * _10;
_6 = Field::<i8>(Variant(_22, 1), 0) * Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_42 = _32;
_42 = _25.0 as u128;
Call(_38 = core::intrinsics::bswap((-4937941133018140701_i64)), ReturnTo(bb16), UnwindUnreachable())
}
bb33 = {
(*_33) = (-6489422519724230131_i64) | 6143467123680035224_i64;
_16 = _17;
(*_7) = (-80_isize) + (-9223372036854775808_isize);
(*_52) = _11 as i128;
_31 = Move(_7);
_17 = -_10;
_48.2 = (*_8) as u16;
(*_66) = core::ptr::addr_of_mut!(_56);
_2 = [_41,_41,_41];
(*_28) = !_47;
_56 = [_27,_27,_27];
(*_21) = core::ptr::addr_of_mut!(_48.2);
_25 = (_48.0, Move((*_66)), _48.2);
(*_66) = core::ptr::addr_of_mut!(_56);
(*_33) = 935741782949912128_i64;
(*_52) = _41;
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 - _6;
_41 = (*_52) & (*_52);
_65 = Move(_35);
_46 = ((*_8),);
_10 = _34 as f32;
_68.2 = _26.1 + _25.2;
_4 = [4067621091_u32,320179121_u32,2780381476_u32,3561063816_u32,3850516036_u32,3263892329_u32];
_4 = [1330758610_u32,4088747187_u32,3468372281_u32,1174154044_u32,4119456197_u32,716960451_u32];
(*_66) = core::ptr::addr_of_mut!(_56);
Goto(bb34)
}
bb34 = {
_63 = (*_28) == (*_28);
_32 = _42 & _42;
(*_28) = (*_52) >= (*_52);
(*_21) = core::ptr::addr_of_mut!(_68.2);
Goto(bb35)
}
bb35 = {
_9.0 = core::ptr::addr_of!(_62);
_40 = (*_28);
(*_33) = _1 as i64;
_53 = Move(_65);
_57 = _9.2.0 as u8;
_59 = _17 * _17;
(*_33) = 8144186578364700181_i64 << (*_52);
(*_21) = core::ptr::addr_of_mut!(_68.2);
Call((*_33) = core::intrinsics::bswap(4387714159835821989_i64), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
_9.1 = core::ptr::addr_of!(_72);
_64 = Move(_15);
_1 = _11;
_73 = &_27;
(*_66) = core::ptr::addr_of_mut!(_56);
(*_33) = (-2653158034167151969_i64) ^ 6013685567155941773_i64;
_35 = Move(_53);
(*_21) = core::ptr::addr_of_mut!(_68.2);
(*_66) = Move(_68.1);
_15 = core::ptr::addr_of_mut!(_21);
(*_52) = _41 * _41;
_68.0 = _48.0 ^ _48.0;
_70 = _2;
(*_52) = _41 & _41;
_53 = Move(_35);
(*_21) = core::ptr::addr_of_mut!(_68.2);
(*_33) = 6542258838544182140_i64 * 4423784741034398751_i64;
_54 = &_9.1;
(*_28) = _47 == _47;
_64 = core::ptr::addr_of_mut!((*_15));
(*_21) = core::ptr::addr_of_mut!(_26.1);
match _24 {
0 => bb37,
25096090406293211899749101503809717453 => bb39,
_ => bb38
}
}
bb37 = {
_9.2 = _18;
RET.fld1 = _19 as u8;
_17 = _16 + _5;
_20 = 72_isize ^ 9223372036854775807_isize;
_9.2 = _18;
_1 = _11;
_17 = (-67102423403462864108004881098986793717_i128) as f32;
_5 = _16;
_9.2 = (_18.0,);
_10 = (-137573177796909329283879747641513101273_i128) as f32;
_18.0 = _9.2.0;
_22 = Adt31::Variant1 { fld0: _6,fld1: 6144361597218019139956997171881515230_i128 };
_9.2 = (_18.0,);
_8 = &_18.0;
_18 = _9.2;
_10 = _16 * _16;
place!(Field::<i128>(Variant(_22, 1), 1)) = !(-44788605887484362287802209199229788816_i128);
_9.3 = &mut _20;
_9.2.0 = _1 as i32;
_10 = _5;
_1 = _11;
_1 = _11;
_11 = _1;
_19 = !true;
place!(Field::<i128>(Variant(_22, 1), 1)) = 33796943575159080484655355111141237534_i128 + (-8120373203531487053202179411453495009_i128);
_23 = [324490523639156045655241736476234775781_u128,133736590505713115245614250391508352529_u128,238588959863198685492507901544548145155_u128,3413738877416972190550232497297314901_u128];
_5 = -_10;
_6 = Field::<i8>(Variant(_22, 1), 0) << _18.0;
place!(Field::<i8>(Variant(_22, 1), 0)) = !_6;
_23 = [302706471303812847942989808878073792274_u128,177394319381288573389167203337522318314_u128,32405769187111315285568501243592538480_u128,44934086374305075603602070766929717023_u128];
Goto(bb7)
}
bb38 = {
place!(Field::<i8>(Variant((*_35), 1), 0)) = _6 ^ _6;
_50 = _26.1 as f64;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 & _41;
_9.0 = core::ptr::addr_of!(place!(Field::<i8>(Variant((*_35), 1), 0)));
_25.2 = _26.1;
place!(Field::<i8>(Variant((*_35), 1), 0)) = !_6;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _41 * _41;
(*_35) = Adt31::Variant1 { fld0: _6,fld1: _41 };
place!(Field::<i8>(Variant((*_35), 1), 0)) = !_6;
place!(Field::<i128>(Variant((*_35), 1), 1)) = _11 as i128;
_28 = core::ptr::addr_of!((*_28));
match _24 {
0 => bb1,
1 => bb14,
2 => bb3,
3 => bb8,
4 => bb12,
5 => bb17,
25096090406293211899749101503809717453 => bb20,
_ => bb9
}
}
bb39 = {
_77 = _51;
_62 = 1333229060_u32 as i8;
(*_21) = core::ptr::addr_of_mut!(_26.1);
(*_28) = _5 > _59;
(*_52) = _41;
_80 = (*_28);
(*_66) = core::ptr::addr_of_mut!(_56);
_11 = _77;
(*_33) = (-6189398124440922195_i64) ^ (-3518615110860394581_i64);
_79.2.0 = !(*_8);
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_66) = core::ptr::addr_of_mut!(_56);
(*_52) = (*_33) as i128;
Goto(bb40)
}
bb40 = {
_46 = ((*_8),);
_65 = Move(_53);
_18 = (_46.0,);
(*_28) = !_40;
(*_28) = _40;
_48.1 = core::ptr::addr_of_mut!(_56);
Goto(bb41)
}
bb41 = {
_39.fld1 = RET.fld1 & RET.fld1;
(*_21) = core::ptr::addr_of_mut!(_48.2);
(*_28) = _45 != _5;
_39.fld1 = RET.fld1 << (*_73);
match _24 {
0 => bb9,
1 => bb34,
25096090406293211899749101503809717453 => bb42,
_ => bb30
}
}
bb42 = {
_68 = (_48.0, Move((*_66)), _26.1);
_82 = !(*_28);
(*_66) = core::ptr::addr_of_mut!(_56);
_40 = !(*_28);
_11 = _51;
(*_33) = 577693895471500203_i64 ^ 7464356328690360534_i64;
_9.0 = core::ptr::addr_of!(_62);
_81 = _48.0 as usize;
_41 = -(*_52);
_18 = (_46.0,);
_78 = core::ptr::addr_of_mut!(_26.1);
(*_78) = _32 as u16;
(*_66) = Move(_25.1);
(*_15) = &mut _78;
_66 = core::ptr::addr_of_mut!((*_66));
_81 = (*_52) as usize;
_25.0 = _48.0 >> _39.fld1;
(*_66) = core::ptr::addr_of_mut!(_56);
(*_28) = !_40;
_71 = _34 >> _25.0;
match _24 {
0 => bb43,
25096090406293211899749101503809717453 => bb45,
_ => bb44
}
}
bb43 = {
(*_21) = core::ptr::addr_of_mut!(_25.2);
place!(Field::<i8>(Variant((*_35), 1), 0)) = -_6;
(*_21) = core::ptr::addr_of_mut!(_26.1);
match _24 {
0 => bb4,
1 => bb21,
25096090406293211899749101503809717453 => bb23,
_ => bb22
}
}
bb44 = {
_30 = core::ptr::addr_of_mut!(_26.1);
_30 = core::ptr::addr_of_mut!((*_30));
_4 = [3064277948_u32,4067002789_u32,2335035388_u32,728403300_u32,1631187622_u32,2011412734_u32];
(*_30) = 24739_u16 | 64904_u16;
(*_30) = 41934_u16 ^ 48120_u16;
_8 = &_9.2.0;
RET.fld1 = (*_28) as u8;
(*_30) = 63915_u16 >> _29;
(*_30) = !4352_u16;
_25.2 = (*_30) - (*_30);
_16 = _25.0 as f32;
_9.0 = core::ptr::addr_of!(_6);
_26.3 = &mut _29;
_6 = Field::<i8>(Variant(_22, 1), 0) | Field::<i8>(Variant(_22, 1), 0);
_30 = core::ptr::addr_of_mut!((*_30));
_9.2.0 = -_18.0;
(*_28) = false | false;
match _24 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb4,
25096090406293211899749101503809717453 => bb14,
_ => bb13
}
}
bb45 = {
_55 = core::ptr::addr_of_mut!((*_66));
(*_66) = Move(_68.1);
_33 = core::ptr::addr_of_mut!((*_33));
match _24 {
0 => bb33,
1 => bb7,
2 => bb24,
3 => bb20,
25096090406293211899749101503809717453 => bb46,
_ => bb37
}
}
bb46 = {
_72 = Adt18::Variant0 { fld0: (*_28),fld1: (*_73),fld2: _59,fld3: (*_33),fld4: _71 };
(*_52) = _71 as i128;
_5 = _59 + _45;
(*_21) = core::ptr::addr_of_mut!(_26.1);
(*_33) = _25.0 as i64;
_49 = Field::<u64>(Variant(_72, 0), 4) * _71;
_91 = core::ptr::addr_of!((*_21));
(*_52) = _41 & _41;
_5 = -_16;
(*_21) = core::ptr::addr_of_mut!(_25.2);
_68 = (_25.0, Move((*_66)), _26.1);
_23 = [_42,_32,_32,_24];
(*_66) = core::ptr::addr_of_mut!(_56);
match _24 {
0 => bb30,
25096090406293211899749101503809717453 => bb47,
_ => bb17
}
}
bb47 = {
_36 = Adt23::Variant0 { fld0: _80,fld1: (*_52),fld2: _72,fld3: Field::<u64>(Variant(_72, 0), 4),fld4: _59 };
(*_21) = core::ptr::addr_of_mut!(_25.2);
_77 = _11;
(*_66) = core::ptr::addr_of_mut!(_56);
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_36, 0), 2)), 0), 3)) = (*_33);
_76 = core::ptr::addr_of_mut!((*_15));
_93 = (-114_isize);
(*_66) = core::ptr::addr_of_mut!(_56);
place!(Field::<u64>(Variant(_72, 0), 4)) = (*_28) as u64;
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_26.1);
(*_66) = core::ptr::addr_of_mut!(_56);
_95 = _32 / _24;
_68.1 = core::ptr::addr_of_mut!(_56);
_79.2.0 = _9.2.0 ^ _46.0;
place!(Field::<i128>(Variant(_36, 0), 1)) = -(*_52);
_79.0 = core::ptr::addr_of!(_6);
place!(Field::<i128>(Variant(_36, 0), 1)) = (*_52) >> (*_33);
(*_28) = Field::<i64>(Variant(Field::<Adt18>(Variant(_36, 0), 2), 0), 3) <= (*_33);
(*_52) = !Field::<i128>(Variant(_36, 0), 1);
_88 = (*_52) | (*_52);
_23 = [_95,_24,_95,_32];
_87 = _46.0 as f32;
(*_28) = _80 & _80;
_26.3 = &mut _93;
match _24 {
0 => bb45,
1 => bb4,
25096090406293211899749101503809717453 => bb49,
_ => bb48
}
}
bb48 = {
_9.2 = _18;
RET.fld1 = _19 as u8;
_17 = _16 + _5;
_20 = 72_isize ^ 9223372036854775807_isize;
_9.2 = _18;
_1 = _11;
_17 = (-67102423403462864108004881098986793717_i128) as f32;
_5 = _16;
_9.2 = (_18.0,);
_10 = (-137573177796909329283879747641513101273_i128) as f32;
_18.0 = _9.2.0;
_22 = Adt31::Variant1 { fld0: _6,fld1: 6144361597218019139956997171881515230_i128 };
_9.2 = (_18.0,);
_8 = &_18.0;
_18 = _9.2;
_10 = _16 * _16;
place!(Field::<i128>(Variant(_22, 1), 1)) = !(-44788605887484362287802209199229788816_i128);
_9.3 = &mut _20;
_9.2.0 = _1 as i32;
_10 = _5;
_1 = _11;
_1 = _11;
_11 = _1;
_19 = !true;
place!(Field::<i128>(Variant(_22, 1), 1)) = 33796943575159080484655355111141237534_i128 + (-8120373203531487053202179411453495009_i128);
_23 = [324490523639156045655241736476234775781_u128,133736590505713115245614250391508352529_u128,238588959863198685492507901544548145155_u128,3413738877416972190550232497297314901_u128];
_5 = -_10;
_6 = Field::<i8>(Variant(_22, 1), 0) << _18.0;
place!(Field::<i8>(Variant(_22, 1), 0)) = !_6;
_23 = [302706471303812847942989808878073792274_u128,177394319381288573389167203337522318314_u128,32405769187111315285568501243592538480_u128,44934086374305075603602070766929717023_u128];
Goto(bb7)
}
bb49 = {
_79 = (Move(_9.0), Move(_9.1), _18, Move(_26.3));
_25.2 = _48.2 << (*_33);
(*_52) = _88;
_63 = Field::<bool>(Variant(_72, 0), 0);
(*_33) = Field::<i64>(Variant(Field::<Adt18>(Variant(_36, 0), 2), 0), 3) << _71;
(*_21) = core::ptr::addr_of_mut!(_48.2);
(*_52) = _46.0 as i128;
_94 = core::ptr::addr_of_mut!(_28);
_81 = (*_73);
_39.fld0 = Move(_36);
(*_66) = core::ptr::addr_of_mut!(_56);
(*_33) = !Field::<i64>(Variant(Field::<Adt18>(Variant(_39.fld0, 0), 2), 0), 3);
_5 = _17 * Field::<f32>(Variant(_72, 0), 2);
(*_94) = core::ptr::addr_of!((*_28));
(*_94) = core::ptr::addr_of!((*_28));
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_39.fld0, 0), 2)), 0), 3)) = !(*_33);
(*_66) = core::ptr::addr_of_mut!(_56);
(*_52) = Field::<i128>(Variant(_39.fld0, 0), 1);
_8 = &_79.2.0;
(*_28) = !_47;
_34 = _49;
_9.0 = core::ptr::addr_of!(_6);
_97.fld0.fld1 = RET.fld1 ^ RET.fld1;
(*_21) = core::ptr::addr_of_mut!(_25.2);
match _24 {
0 => bb19,
1 => bb46,
2 => bb29,
3 => bb30,
4 => bb17,
5 => bb47,
25096090406293211899749101503809717453 => bb50,
_ => bb43
}
}
bb50 = {
_97.fld0.fld1 = _39.fld1 ^ _39.fld1;
(*_52) = _88;
(*_94) = core::ptr::addr_of!((*_28));
RET.fld0 = Move(_39.fld0);
_104 = _23;
(*_52) = _88 + _88;
_97.fld0 = Move(RET);
_80 = _25.0 <= _68.0;
_64 = core::ptr::addr_of_mut!((*_15));
_68.2 = _25.2 ^ _25.2;
RET.fld1 = _57;
place!(Field::<Adt18>(Variant(_97.fld0.fld0, 0), 2)) = _72;
_46.0 = -(*_8);
_26.2 = [(*_52),(*_52),(*_52),(*_52)];
(*_94) = core::ptr::addr_of!(_19);
_8 = &_46.0;
(*_94) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_97.fld0.fld0, 0), 0)));
_91 = core::ptr::addr_of!((*_21));
(*_66) = core::ptr::addr_of_mut!(_56);
_26.0 = Move(_97.fld0.fld0);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_102 = &mut (*_21);
(*_33) = (*_52) as i64;
_36 = Move(_26.0);
_6 = _62 << (*_52);
_39.fld0 = Move(_36);
Goto(bb51)
}
bb51 = {
(*_94) = core::ptr::addr_of!((*_28));
(*_76) = Move(_102);
_91 = core::ptr::addr_of!(_105);
(*_91) = core::ptr::addr_of_mut!(_26.1);
place!(Field::<usize>(Variant(_72, 0), 1)) = (*_33) as usize;
(*_15) = &mut (*_91);
_1 = _11;
_106 = _68.0 & _25.0;
RET.fld1 = _39.fld1 << Field::<u64>(Variant(_72, 0), 4);
(*_94) = core::ptr::addr_of!((*_28));
(*_28) = _47 ^ Field::<bool>(Variant(Field::<Adt18>(Variant(_39.fld0, 0), 2), 0), 0);
_97.fld0.fld1 = RET.fld1 & RET.fld1;
_68.0 = !_25.0;
_107.1.fld0 = Adt44 { fld0: Move(_39.fld0),fld1: _39.fld1 };
place!(Field::<i128>(Variant(_107.1.fld0.fld0, 0), 1)) = (*_52);
(*_52) = Field::<i128>(Variant(_107.1.fld0.fld0, 0), 1);
(*_28) = Field::<bool>(Variant(_72, 0), 0) ^ _82;
(*_52) = Field::<i128>(Variant(_107.1.fld0.fld0, 0), 1);
(*_94) = core::ptr::addr_of!(_111);
(*_33) = 9223372036854775807_isize as i64;
_70 = _2;
_97.fld0 = Adt44 { fld0: Move(_107.1.fld0.fld0),fld1: RET.fld1 };
(*_94) = core::ptr::addr_of!(_37);
(*_21) = core::ptr::addr_of_mut!(_26.1);
match _24 {
0 => bb23,
1 => bb31,
25096090406293211899749101503809717453 => bb53,
_ => bb52
}
}
bb52 = {
Return()
}
bb53 = {
_71 = Field::<u64>(Variant(Field::<Adt18>(Variant(_97.fld0.fld0, 0), 2), 0), 4) ^ Field::<u64>(Variant(_72, 0), 4);
_18.0 = (*_8) - (*_8);
_80 = Field::<bool>(Variant(_97.fld0.fld0, 0), 0) | Field::<bool>(Variant(Field::<Adt18>(Variant(_97.fld0.fld0, 0), 2), 0), 0);
_27 = Field::<usize>(Variant(_72, 0), 1);
(*_33) = (*_8) as i64;
_48.2 = _68.2 - _68.2;
(*_28) = _80 | _47;
_95 = _34 as u128;
_79.0 = core::ptr::addr_of!(_62);
(*_33) = -Field::<i64>(Variant(_72, 0), 3);
_98 = core::ptr::addr_of!(_72);
place!(Field::<f32>(Variant(_97.fld0.fld0, 0), 4)) = Field::<f32>(Variant((*_98), 0), 2) - Field::<f32>(Variant((*_98), 0), 2);
place!(Field::<f32>(Variant(_97.fld0.fld0, 0), 4)) = _87 - Field::<f32>(Variant((*_98), 0), 2);
place!(Field::<usize>(Variant((*_98), 0), 1)) = _27 + _27;
(*_33) = Field::<i64>(Variant((*_98), 0), 3) ^ Field::<i64>(Variant((*_98), 0), 3);
Goto(bb54)
}
bb54 = {
_58 = Field::<f32>(Variant((*_98), 0), 2) - Field::<f32>(Variant((*_98), 0), 2);
_107.0 = [_51,_1,_77,_11];
(*_94) = core::ptr::addr_of!((*_28));
_9.1 = core::ptr::addr_of!((*_98));
place!(Field::<f32>(Variant((*_98), 0), 2)) = _58 * _5;
_63 = (*_28) == Field::<bool>(Variant((*_98), 0), 0);
match _24 {
0 => bb12,
1 => bb28,
2 => bb46,
25096090406293211899749101503809717453 => bb56,
_ => bb55
}
}
bb55 = {
_21 = &mut _30;
(*_28) = _6 != Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_27 = 5_usize << _6;
_42 = (*_28) as u128;
_23 = [_42,_42,_42,_42];
(*_28) = false & false;
(*_21) = core::ptr::addr_of_mut!(_25.2);
(*_21) = core::ptr::addr_of_mut!(_25.2);
_16 = _10 * _10;
_6 = Field::<i8>(Variant(_22, 1), 0) * Field::<i8>(Variant(_22, 1), 0);
(*_21) = core::ptr::addr_of_mut!(_26.1);
_42 = _32;
_42 = _25.0 as u128;
Call(_38 = core::intrinsics::bswap((-4937941133018140701_i64)), ReturnTo(bb16), UnwindUnreachable())
}
bb56 = {
place!(Field::<i64>(Variant((*_98), 0), 3)) = (*_33);
place!(Field::<u64>(Variant((*_98), 0), 4)) = !_34;
_99 = _11;
place!(Field::<Adt18>(Variant(_97.fld0.fld0, 0), 2)) = Adt18::Variant0 { fld0: _47,fld1: Field::<usize>(Variant((*_98), 0), 1),fld2: _5,fld3: (*_33),fld4: Field::<u64>(Variant((*_98), 0), 4) };
place!(Field::<f32>(Variant((*_98), 0), 2)) = _95 as f32;
place!(Field::<usize>(Variant((*_98), 0), 1)) = Field::<usize>(Variant(Field::<Adt18>(Variant(_97.fld0.fld0, 0), 2), 0), 1);
(*_33) = Field::<u64>(Variant((*_98), 0), 4) as i64;
(*_94) = core::ptr::addr_of!((*_28));
(*_21) = core::ptr::addr_of_mut!(_26.1);
place!(Field::<u64>(Variant((*_98), 0), 4)) = !_34;
(*_66) = Move(_68.1);
(*_66) = core::ptr::addr_of_mut!(_56);
(*_28) = Field::<usize>(Variant((*_98), 0), 1) != Field::<usize>(Variant((*_98), 0), 1);
(*_66) = core::ptr::addr_of_mut!(_56);
place!(Field::<i64>(Variant(_72, 0), 3)) = (*_8) as i64;
place!(Field::<bool>(Variant((*_98), 0), 0)) = !Field::<bool>(Variant(_97.fld0.fld0, 0), 0);
place!(Field::<i64>(Variant((*_98), 0), 3)) = 96_isize as i64;
place!(Field::<bool>(Variant((*_98), 0), 0)) = !(*_28);
place!(Field::<usize>(Variant(_72, 0), 1)) = Field::<usize>(Variant(Field::<Adt18>(Variant(_97.fld0.fld0, 0), 2), 0), 1) + _27;
_9.1 = core::ptr::addr_of!((*_98));
Goto(bb57)
}
bb57 = {
_97.fld2 = [_95,_95,_42,_95];
(*_21) = core::ptr::addr_of_mut!(_48.2);
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_97.fld0.fld0, 0), 2)), 0), 3)) = _38 ^ (*_33);
_110 = [_68.0,_68.0,_106];
(*_66) = core::ptr::addr_of_mut!(_56);
_48.2 = !_68.2;
_35 = Move(_65);
place!(Field::<i64>(Variant((*_98), 0), 3)) = (*_33);
place!(Field::<f32>(Variant((*_98), 0), 2)) = Field::<usize>(Variant((*_98), 0), 1) as f32;
place!(Field::<f32>(Variant((*_98), 0), 2)) = _6 as f32;
Goto(bb58)
}
bb58 = {
(*_28) = !Field::<bool>(Variant((*_98), 0), 0);
_75 = core::ptr::addr_of_mut!((*_15));
(*_33) = -Field::<i64>(Variant((*_98), 0), 3);
(*_33) = Field::<i64>(Variant((*_98), 0), 3) ^ Field::<i64>(Variant((*_98), 0), 3);
(*_94) = core::ptr::addr_of!(place!(Field::<bool>(Variant((*_98), 0), 0)));
place!(Field::<f32>(Variant((*_98), 0), 2)) = _107.1.fld0.fld1 as f32;
_39 = Adt44 { fld0: Move(_97.fld0.fld0),fld1: _97.fld0.fld1 };
(*_28) = _40 & Field::<bool>(Variant(Field::<Adt18>(Variant(_39.fld0, 0), 2), 0), 0);
place!(Field::<bool>(Variant((*_98), 0), 0)) = (*_52) > (*_52);
_55 = core::ptr::addr_of_mut!((*_66));
place!(Field::<f32>(Variant((*_98), 0), 2)) = _68.2 as f32;
place!(Field::<u64>(Variant((*_98), 0), 4)) = _49 * Field::<u64>(Variant(_39.fld0, 0), 3);
place!(Field::<usize>(Variant((*_98), 0), 1)) = !Field::<usize>(Variant(Field::<Adt18>(Variant(_39.fld0, 0), 2), 0), 1);
_96 = 9223372036854775807_isize - 9223372036854775807_isize;
_107.1.fld0 = Move(_39);
(*_98) = Adt18::Variant1 { fld0: _96,fld1: _110 };
_39.fld1 = !_97.fld0.fld1;
_120 = _51;
_24 = _68.2 as u128;
_53 = Move(_35);
_92 = -Field::<isize>(Variant((*_98), 1), 0);
_45 = _106 as f32;
_97.fld2 = [_95,_24,_24,_24];
(*_66) = core::ptr::addr_of_mut!(_56);
(*_33) = Field::<i64>(Variant(Field::<Adt18>(Variant(_107.1.fld0.fld0, 0), 2), 0), 3) - Field::<i64>(Variant(Field::<Adt18>(Variant(_107.1.fld0.fld0, 0), 2), 0), 3);
place!(Field::<u64>(Variant(_107.1.fld0.fld0, 0), 3)) = _24 as u64;
Goto(bb59)
}
bb59 = {
(*_33) = Field::<i64>(Variant(Field::<Adt18>(Variant(_107.1.fld0.fld0, 0), 2), 0), 3);
place!(Field::<isize>(Variant((*_98), 1), 0)) = _25.2 as isize;
_25.0 = !_68.0;
place!(Field::<isize>(Variant((*_98), 1), 0)) = _6 as isize;
_99 = _11;
_17 = _16 * _58;
Goto(bb60)
}
bb60 = {
_82 = _63;
_65 = Move(_53);
(*_94) = core::ptr::addr_of!(_118);
_126 = core::ptr::addr_of_mut!(_56);
place!(Field::<[i16; 3]>(Variant((*_98), 1), 1)) = _110;
(*_33) = Field::<i64>(Variant(Field::<Adt18>(Variant(_107.1.fld0.fld0, 0), 2), 0), 3) - Field::<i64>(Variant(Field::<Adt18>(Variant(_107.1.fld0.fld0, 0), 2), 0), 3);
_18 = _46;
_107.1.fld1 = core::ptr::addr_of_mut!((*_28));
(*_94) = core::ptr::addr_of!(_118);
(*_126) = [Field::<usize>(Variant(Field::<Adt18>(Variant(_107.1.fld0.fld0, 0), 2), 0), 1),_27,Field::<usize>(Variant(Field::<Adt18>(Variant(_107.1.fld0.fld0, 0), 2), 0), 1)];
_26.0 = Adt23::Variant0 { fld0: _82,fld1: (*_52),fld2: (*_98),fld3: Field::<u64>(Variant(_107.1.fld0.fld0, 0), 3),fld4: _17 };
(*_66) = core::ptr::addr_of_mut!((*_126));
place!(Field::<isize>(Variant((*_98), 1), 0)) = Field::<isize>(Variant(Field::<Adt18>(Variant(_26.0, 0), 2), 1), 0);
place!(Field::<u64>(Variant(_26.0, 0), 3)) = _34 >> Field::<isize>(Variant((*_98), 1), 0);
_33 = core::ptr::addr_of_mut!((*_33));
(*_21) = core::ptr::addr_of_mut!(_128);
RET = Adt44 { fld0: Move(_107.1.fld0.fld0),fld1: _97.fld0.fld1 };
(*_28) = Field::<bool>(Variant(RET.fld0, 0), 0) ^ _63;
Goto(bb61)
}
bb61 = {
Call(_131 = dump_var(3_usize, 1_usize, Move(_1), 70_usize, Move(_70), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Call(_131 = dump_var(3_usize, 104_usize, Move(_104), 18_usize, Move(_18), 19_usize, Move(_19), 71_usize, Move(_71)), ReturnTo(bb63), UnwindUnreachable())
}
bb63 = {
Call(_131 = dump_var(3_usize, 23_usize, Move(_23), 56_usize, Move(_56), 63_usize, Move(_63), 29_usize, Move(_29)), ReturnTo(bb64), UnwindUnreachable())
}
bb64 = {
Call(_131 = dump_var(3_usize, 110_usize, Move(_110), 34_usize, Move(_34), 99_usize, Move(_99), 93_usize, Move(_93)), ReturnTo(bb65), UnwindUnreachable())
}
bb65 = {
Call(_131 = dump_var(3_usize, 96_usize, Move(_96), 106_usize, Move(_106), 42_usize, Move(_42), 62_usize, Move(_62)), ReturnTo(bb66), UnwindUnreachable())
}
bb66 = {
Call(_131 = dump_var(3_usize, 47_usize, Move(_47), 49_usize, Move(_49), 51_usize, Move(_51), 132_usize, _132), ReturnTo(bb67), UnwindUnreachable())
}
bb67 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i32) -> f32 {
mir! {
type RET = f32;
let _2: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _3: f64;
let _4: f64;
let _5: bool;
let _6: &'static usize;
let _7: f64;
let _8: u8;
let _9: isize;
let _10: *const *mut [usize; 3];
let _11: char;
let _12: &'static mut Adt27;
let _13: [u32; 6];
let _14: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _15: (bool, u64, i8, u128);
let _16: i32;
let _17: isize;
let _18: &'static usize;
let _19: bool;
let _20: (i16, *mut [usize; 3], u16);
let _21: Adt31;
let _22: [char; 4];
let _23: u8;
let _24: [i8; 7];
let _25: f32;
let _26: [usize; 3];
let _27: f32;
let _28: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _29: Adt57;
let _30: Adt31;
let _31: &'static &'static mut Adt57;
let _32: *mut i64;
let _33: [i16; 3];
let _34: *mut [usize; 3];
let _35: (Adt57, *const bool);
let _36: *const i8;
let _37: *const *const (i32,);
let _38: &'static mut i32;
let _39: u8;
let _40: (bool, u64, i8, u128);
let _41: f64;
let _42: usize;
let _43: [i128; 3];
let _44: *mut [usize; 3];
let _45: *const &'static mut i128;
let _46: isize;
let _47: isize;
let _48: *mut f32;
let _49: *mut *mut [usize; 3];
let _50: isize;
let _51: *const Adt18;
let _52: bool;
let _53: [u32; 6];
let _54: [char; 3];
let _55: [i32; 6];
let _56: &'static mut i128;
let _57: &'static (bool, u64, i8, u128);
let _58: usize;
let _59: (Adt23, u16, [i128; 4], &'static mut isize);
let _60: char;
let _61: u64;
let _62: f64;
let _63: [u32; 6];
let _64: (Adt57, *const bool);
let _65: &'static Adt57;
let _66: ();
let _67: ();
{
_2.2.fld1 = 96_u8 * 126_u8;
_2.1 = [71871205539979640354370178962682664131_i128,77020960699013080079924227944991239641_i128,(-166902138241344873447959918576353110912_i128),(-134277017214536581285437223387572049389_i128)];
_2.2.fld1 = !34_u8;
_1 = 384823302_i32 & (-1694426676_i32);
RET = 1144954225850183462_u64 as f32;
_1 = false as i32;
RET = _2.2.fld1 as f32;
RET = 14732124790666037162_u64 as f32;
_3 = 1675178457_u32 as f64;
RET = 126_i8 as f32;
_1 = 725834456_i32 + 60074866_i32;
_2.2.fld1 = 120_u8 >> _1;
_1 = 1958928355_i32;
_2.1 = [151818461535054546781332159379372502896_i128,147921965288571606237652302423012591618_i128,143750598987975361407476717956900973528_i128,(-118149075829582847078697773512209341487_i128)];
_2.1 = [(-144732271158792692838663557769117721611_i128),(-13613289756560830559726680524304508688_i128),(-2810117650994222401569903388524496505_i128),150950844419935730273922641374411310765_i128];
_1 = (-1171398749_i32) * (-1998206911_i32);
_4 = _3 * _3;
_1 = 478111922_i32;
_7 = 91_i8 as f64;
match _1 {
0 => bb1,
1 => bb2,
478111922 => bb4,
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
_4 = _3 * _7;
_3 = -_7;
_5 = true;
_5 = false & false;
RET = 27727856263665436984786524487187588854_u128 as f32;
_7 = _4 - _4;
_2.1 = [17410978404575509542720915830718184527_i128,129399167527971907410418880948665934483_i128,(-161329567716378037043277839991847827507_i128),63981069228692717274379256619453064306_i128];
_7 = 8203530519448672265_usize as f64;
_5 = true ^ false;
_3 = -_7;
RET = 71_i8 as f32;
Goto(bb5)
}
bb5 = {
_8 = _2.2.fld1;
_5 = false;
_2.1 = [(-90736467270851154887837025052335372342_i128),48038619549736161757169745732785319530_i128,37344427198482509074244056234536545530_i128,(-127069274736834481278775339912875156483_i128)];
_3 = (-90_i8) as f64;
_2.2.fld1 = _8 >> _1;
_2.2.fld1 = _1 as u8;
_8 = _2.2.fld1 >> _1;
_7 = _4 + _4;
_9 = !(-59_isize);
RET = _1 as f32;
_5 = _7 == _3;
_1 = -177672655_i32;
Goto(bb6)
}
bb6 = {
RET = _4 as f32;
Goto(bb7)
}
bb7 = {
_3 = _7 - _7;
_8 = 109_i8 as u8;
Goto(bb8)
}
bb8 = {
_8 = !_2.2.fld1;
_9 = 9223372036854775807_isize;
_8 = _2.2.fld1;
_11 = '\u{3998f}';
_3 = _7;
_7 = _3 - _3;
_8 = !_2.2.fld1;
_8 = _2.2.fld1;
_11 = '\u{95b21}';
RET = (-18609_i16) as f32;
_7 = _4;
_11 = '\u{8ce6}';
_4 = -_3;
_8 = (-6350_i16) as u8;
_8 = _2.2.fld1;
_4 = 313333895293285384228227108127383857408_u128 as f64;
_4 = (-31311_i16) as f64;
Goto(bb9)
}
bb9 = {
_3 = 108_i8 as f64;
_13 = [210213417_u32,936053502_u32,3782843071_u32,3201811379_u32,3873046165_u32,3343131090_u32];
_1 = _11 as i32;
_5 = !false;
_2.2.fld1 = RET as u8;
_1 = (-2499644651648189259_i64) as i32;
RET = _1 as f32;
_2.2.fld1 = _8 - _8;
_2.1 = [113868945291538486302855233405994969908_i128,(-106562018193108358510921500319670751320_i128),105913585228157887233830395021701877767_i128,(-91122480377497574638426793858074760678_i128)];
_4 = _3 - _7;
RET = 22864_i16 as f32;
_9 = 23354_u16 as isize;
_1 = (-937371844_i32);
RET = 28206_i16 as f32;
_1 = 1672373942_i32;
RET = 203889810482620776452203246578822986810_u128 as f32;
_7 = _4;
_8 = _2.2.fld1 ^ _2.2.fld1;
_3 = _4 * _4;
RET = 1180897589003023410_usize as f32;
_2.1 = [48204710813759997401358365808322289504_i128,58105473893570390295226302545476632377_i128,55087385119288642937580265042605268668_i128,109439906791277996416758390931826218090_i128];
_4 = -_3;
_11 = '\u{f7c45}';
_3 = _7;
_5 = true;
_7 = _4;
_8 = _2.2.fld1 * _2.2.fld1;
Call(RET = fn5(_1, _1, _2.1, _7, _1, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = _7 - _7;
_9 = (-40_isize) | (-9223372036854775808_isize);
_14.2 = (_1,);
_14.2.0 = _1;
_2.2.fld1 = _8 ^ _8;
_14.2 = (_1,);
_2.2.fld1 = _8 | _8;
_14.3 = &mut _9;
_2.1 = [(-109459947340191936497818273305382219240_i128),(-106514625555032235121374869815055150714_i128),18100478457659175641608689447048241100_i128,43164410371752266367602964358568559872_i128];
RET = 12259268148413443621_u64 as f32;
_7 = _3 + _4;
_4 = _7 * _3;
_2.1 = [132569216014052337904853785223006110733_i128,(-62558397204193121532066030535038358393_i128),(-116677904609710594184117894775752212037_i128),119162311952538641709978226725527848220_i128];
_8 = 8850143709413996805_u64 as u8;
RET = 3101566629_u32 as f32;
_2.2.fld1 = _8 ^ _8;
_7 = _4;
_3 = -_4;
_14.2.0 = _1 * _1;
_14.2 = (_1,);
_16 = _1 & _14.2.0;
_3 = -_4;
_2.1 = [(-135085219803937486083742497586046837016_i128),(-50778985173326902142365274554636853530_i128),(-44964360407044125726312471448595321415_i128),19440775436950746589783168938115973586_i128];
_8 = !_2.2.fld1;
_5 = _4 > _4;
Goto(bb11)
}
bb11 = {
_11 = '\u{49b04}';
_16 = _1 >> _8;
_8 = _2.2.fld1;
_15 = (_5, 8439523970408885133_u64, 84_i8, 70354855010565604370291010547845821490_u128);
_11 = '\u{40a9b}';
_4 = _7 - _7;
_19 = _5;
_17 = (-9223372036854775808_isize);
_10 = core::ptr::addr_of!(_20.1);
_15.3 = 11123125462665941789343411438443085445_u128 - 193506432371673178040307685168367625130_u128;
_5 = _15.1 < _15.1;
_23 = _8 | _8;
_14.0 = core::ptr::addr_of!(_15.2);
_14.2.0 = _16 + _16;
_15 = (_5, 1717996874685524162_u64, (-4_i8), 10598083667535676233368314372864452427_u128);
_15.0 = !_5;
_2.1 = [(-79694677681197211569013035340963736860_i128),85339218296060216183933318713046996672_i128,(-56336122241434502776526040666234708277_i128),10130753931399168868874725085335357494_i128];
_4 = -_7;
_15.0 = !_19;
_23 = _2.2.fld1 & _2.2.fld1;
_11 = '\u{e98f3}';
match _15.2 {
0 => bb8,
1 => bb7,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211452 => bb12,
_ => bb10
}
}
bb12 = {
_14.2.0 = -_16;
RET = 41968_u16 as f32;
_5 = _15.0;
_20.2 = 55560_u16 + 52398_u16;
_23 = _8 + _8;
_11 = '\u{b3819}';
_2.2.fld1 = !_23;
Goto(bb13)
}
bb13 = {
_5 = _15.0;
_13 = [1627267469_u32,1858857084_u32,2597552563_u32,2614613697_u32,2982406612_u32,3785709104_u32];
Goto(bb14)
}
bb14 = {
_15.3 = 221110830101262293702169455457670621729_u128;
_20.0 = (-16029_i16) ^ 8164_i16;
_2.1 = [118745354782016726281021128749624702297_i128,20794813209818158631433066785194804025_i128,25670807456368564801548045523290475980_i128,9108364207532348640839819258642507200_i128];
_19 = _5 | _5;
_22 = [_11,_11,_11,_11];
_14.0 = core::ptr::addr_of!(_15.2);
_15.0 = _5;
_15 = (_19, 7686564235695728037_u64, 113_i8, 122705304744601237339818553382210165133_u128);
_14.2 = (_16,);
_15 = (_19, 8676283686644644648_u64, 73_i8, 329386450365821267676375670280147232788_u128);
_15.1 = 3266459947713925584_u64 & 2664498155596879421_u64;
_14.0 = core::ptr::addr_of!(_15.2);
_24 = [_15.2,_15.2,_15.2,_15.2,_15.2,_15.2,_15.2];
_16 = -_14.2.0;
_15.2 = 86_i8 | (-17_i8);
_2.1 = [105298562643440209492580181161379693110_i128,(-153489867512832442139141674187128900933_i128),159656816546055527300437694392941588303_i128,164073564283043587824344257609811286204_i128];
match _15.3 {
0 => bb12,
1 => bb11,
2 => bb13,
3 => bb6,
4 => bb15,
5 => bb16,
6 => bb17,
329386450365821267676375670280147232788 => bb19,
_ => bb18
}
}
bb15 = {
RET = _4 as f32;
Goto(bb7)
}
bb16 = {
_14.2.0 = -_16;
RET = 41968_u16 as f32;
_5 = _15.0;
_20.2 = 55560_u16 + 52398_u16;
_23 = _8 + _8;
_11 = '\u{b3819}';
_2.2.fld1 = !_23;
Goto(bb13)
}
bb17 = {
_8 = !_2.2.fld1;
_9 = 9223372036854775807_isize;
_8 = _2.2.fld1;
_11 = '\u{3998f}';
_3 = _7;
_7 = _3 - _3;
_8 = !_2.2.fld1;
_8 = _2.2.fld1;
_11 = '\u{95b21}';
RET = (-18609_i16) as f32;
_7 = _4;
_11 = '\u{8ce6}';
_4 = -_3;
_8 = (-6350_i16) as u8;
_8 = _2.2.fld1;
_4 = 313333895293285384228227108127383857408_u128 as f64;
_4 = (-31311_i16) as f64;
Goto(bb9)
}
bb18 = {
_3 = 108_i8 as f64;
_13 = [210213417_u32,936053502_u32,3782843071_u32,3201811379_u32,3873046165_u32,3343131090_u32];
_1 = _11 as i32;
_5 = !false;
_2.2.fld1 = RET as u8;
_1 = (-2499644651648189259_i64) as i32;
RET = _1 as f32;
_2.2.fld1 = _8 - _8;
_2.1 = [113868945291538486302855233405994969908_i128,(-106562018193108358510921500319670751320_i128),105913585228157887233830395021701877767_i128,(-91122480377497574638426793858074760678_i128)];
_4 = _3 - _7;
RET = 22864_i16 as f32;
_9 = 23354_u16 as isize;
_1 = (-937371844_i32);
RET = 28206_i16 as f32;
_1 = 1672373942_i32;
RET = 203889810482620776452203246578822986810_u128 as f32;
_7 = _4;
_8 = _2.2.fld1 ^ _2.2.fld1;
_3 = _4 * _4;
RET = 1180897589003023410_usize as f32;
_2.1 = [48204710813759997401358365808322289504_i128,58105473893570390295226302545476632377_i128,55087385119288642937580265042605268668_i128,109439906791277996416758390931826218090_i128];
_4 = -_3;
_11 = '\u{f7c45}';
_3 = _7;
_5 = true;
_7 = _4;
_8 = _2.2.fld1 * _2.2.fld1;
Call(RET = fn5(_1, _1, _2.1, _7, _1, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb19 = {
_2.1 = [(-161516735612668144789338101911424610298_i128),158788694024371410731675181114269330534_i128,(-13874904332782789418562309711432368487_i128),34597779811026026257168752759866325321_i128];
_22 = [_11,_11,_11,_11];
_15 = (_19, 3827886444868535339_u64, (-116_i8), 285907296127963940767148125186416318282_u128);
_14.2.0 = _4 as i32;
_8 = _14.2.0 as u8;
match _15.3 {
0 => bb16,
1 => bb8,
285907296127963940767148125186416318282 => bb21,
_ => bb20
}
}
bb20 = {
_5 = _15.0;
_13 = [1627267469_u32,1858857084_u32,2597552563_u32,2614613697_u32,2982406612_u32,3785709104_u32];
Goto(bb14)
}
bb21 = {
_14.0 = core::ptr::addr_of!(_15.2);
_4 = _3 * _3;
_16 = !_14.2.0;
_20.0 = _7 as i16;
_26 = [888014266407099129_usize,3657379001253532498_usize,557698403090812580_usize];
_4 = _3;
(*_10) = core::ptr::addr_of_mut!(_26);
(*_10) = core::ptr::addr_of_mut!(_26);
_19 = _5 | _15.0;
_15.2 = (-50_i8) ^ 16_i8;
(*_10) = core::ptr::addr_of_mut!(_26);
(*_10) = core::ptr::addr_of_mut!(_26);
_5 = _19;
_2.2.fld1 = _8;
(*_10) = core::ptr::addr_of_mut!(_26);
(*_10) = core::ptr::addr_of_mut!(_26);
(*_10) = core::ptr::addr_of_mut!(_26);
_2.1 = [(-41157251655306032645177127945409506491_i128),77275661524448346562711220876335141891_i128,(-143714475478885404341996143494334815124_i128),(-37109390872036016632425432121705160754_i128)];
(*_10) = core::ptr::addr_of_mut!(_26);
match _15.1 {
0 => bb20,
1 => bb22,
2 => bb23,
3 => bb24,
3827886444868535339 => bb26,
_ => bb25
}
}
bb22 = {
Return()
}
bb23 = {
Return()
}
bb24 = {
_11 = '\u{49b04}';
_16 = _1 >> _8;
_8 = _2.2.fld1;
_15 = (_5, 8439523970408885133_u64, 84_i8, 70354855010565604370291010547845821490_u128);
_11 = '\u{40a9b}';
_4 = _7 - _7;
_19 = _5;
_17 = (-9223372036854775808_isize);
_10 = core::ptr::addr_of!(_20.1);
_15.3 = 11123125462665941789343411438443085445_u128 - 193506432371673178040307685168367625130_u128;
_5 = _15.1 < _15.1;
_23 = _8 | _8;
_14.0 = core::ptr::addr_of!(_15.2);
_14.2.0 = _16 + _16;
_15 = (_5, 1717996874685524162_u64, (-4_i8), 10598083667535676233368314372864452427_u128);
_15.0 = !_5;
_2.1 = [(-79694677681197211569013035340963736860_i128),85339218296060216183933318713046996672_i128,(-56336122241434502776526040666234708277_i128),10130753931399168868874725085335357494_i128];
_4 = -_7;
_15.0 = !_19;
_23 = _2.2.fld1 & _2.2.fld1;
_11 = '\u{e98f3}';
match _15.2 {
0 => bb8,
1 => bb7,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211452 => bb12,
_ => bb10
}
}
bb25 = {
_8 = !_2.2.fld1;
_9 = 9223372036854775807_isize;
_8 = _2.2.fld1;
_11 = '\u{3998f}';
_3 = _7;
_7 = _3 - _3;
_8 = !_2.2.fld1;
_8 = _2.2.fld1;
_11 = '\u{95b21}';
RET = (-18609_i16) as f32;
_7 = _4;
_11 = '\u{8ce6}';
_4 = -_3;
_8 = (-6350_i16) as u8;
_8 = _2.2.fld1;
_4 = 313333895293285384228227108127383857408_u128 as f64;
_4 = (-31311_i16) as f64;
Goto(bb9)
}
bb26 = {
(*_10) = core::ptr::addr_of_mut!(_26);
(*_10) = core::ptr::addr_of_mut!(_26);
(*_10) = core::ptr::addr_of_mut!(_26);
_14.2.0 = _16 * _16;
_13 = [224225648_u32,1731279896_u32,2008575216_u32,3269266108_u32,1651250467_u32,3808829097_u32];
match _15.3 {
285907296127963940767148125186416318282 => bb27,
_ => bb3
}
}
bb27 = {
_5 = _19;
(*_10) = core::ptr::addr_of_mut!(_26);
_5 = _4 < _4;
_17 = _15.0 as isize;
(*_10) = core::ptr::addr_of_mut!(_26);
_21 = Adt31::Variant0 { fld0: _15.3,fld1: _14.2,fld2: _15.1,fld3: _15.2,fld4: _20.0 };
match _15.3 {
285907296127963940767148125186416318282 => bb29,
_ => bb28
}
}
bb28 = {
_11 = '\u{49b04}';
_16 = _1 >> _8;
_8 = _2.2.fld1;
_15 = (_5, 8439523970408885133_u64, 84_i8, 70354855010565604370291010547845821490_u128);
_11 = '\u{40a9b}';
_4 = _7 - _7;
_19 = _5;
_17 = (-9223372036854775808_isize);
_10 = core::ptr::addr_of!(_20.1);
_15.3 = 11123125462665941789343411438443085445_u128 - 193506432371673178040307685168367625130_u128;
_5 = _15.1 < _15.1;
_23 = _8 | _8;
_14.0 = core::ptr::addr_of!(_15.2);
_14.2.0 = _16 + _16;
_15 = (_5, 1717996874685524162_u64, (-4_i8), 10598083667535676233368314372864452427_u128);
_15.0 = !_5;
_2.1 = [(-79694677681197211569013035340963736860_i128),85339218296060216183933318713046996672_i128,(-56336122241434502776526040666234708277_i128),10130753931399168868874725085335357494_i128];
_4 = -_7;
_15.0 = !_19;
_23 = _2.2.fld1 & _2.2.fld1;
_11 = '\u{e98f3}';
match _15.2 {
0 => bb8,
1 => bb7,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211452 => bb12,
_ => bb10
}
}
bb29 = {
_36 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_21, 0), 3)));
(*_10) = core::ptr::addr_of_mut!(_26);
(*_36) = _15.2 & _15.2;
_28.2.fld1 = _2.2.fld1;
_27 = RET + RET;
_5 = _15.0 ^ _15.0;
(*_10) = core::ptr::addr_of_mut!(_26);
(*_10) = core::ptr::addr_of_mut!(_26);
_35.0.fld2 = [Field::<u128>(Variant(_21, 0), 0),_15.3,Field::<u128>(Variant(_21, 0), 0),Field::<u128>(Variant(_21, 0), 0)];
_2.2.fld1 = _8;
_28.2.fld1 = !_2.2.fld1;
_29.fld2 = [Field::<u128>(Variant(_21, 0), 0),_15.3,Field::<u128>(Variant(_21, 0), 0),Field::<u128>(Variant(_21, 0), 0)];
_13 = [4210454722_u32,3317604023_u32,1876487075_u32,4119764958_u32,42164151_u32,3307266000_u32];
_38 = &mut _14.2.0;
place!(Field::<i16>(Variant(_21, 0), 4)) = _28.2.fld1 as i16;
_2.1 = [53613251588512705768149914164199958289_i128,(-146550946221772674230195144539345505044_i128),20549382170100325827396233057211442446_i128,80954874117607019594894833782390268193_i128];
place!(Field::<u64>(Variant(_21, 0), 2)) = _15.1 & _15.1;
(*_10) = core::ptr::addr_of_mut!(_26);
_35.0.fld1 = core::ptr::addr_of_mut!(_40.0);
_35.0.fld1 = core::ptr::addr_of_mut!(_19);
_39 = _27 as u8;
place!(Field::<u64>(Variant(_21, 0), 2)) = (-93061620160456140631367691425013316393_i128) as u64;
(*_38) = Field::<(i32,)>(Variant(_21, 0), 1).0;
(*_38) = Field::<(i32,)>(Variant(_21, 0), 1).0;
Goto(bb30)
}
bb30 = {
_21 = Adt31::Variant1 { fld0: _15.2,fld1: 54540699324320299779516182772205793941_i128 };
_15.3 = 219163404832369123607140372777036629132_u128 ^ 198110556676339368334578844974837790607_u128;
_10 = core::ptr::addr_of!((*_10));
(*_10) = core::ptr::addr_of_mut!(_26);
_44 = core::ptr::addr_of_mut!(_26);
_4 = _7 * _7;
_40 = (_5, _15.1, _15.2, _15.3);
(*_44) = [4_usize,6_usize,2_usize];
_36 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_21, 1), 0)));
(*_44) = [6421643872265178684_usize,5_usize,68426849144214427_usize];
_1 = (*_38);
_40.2 = -(*_36);
_35.0.fld0.fld1 = (*_36) as u8;
(*_44) = [0_usize,5_usize,13152635099625114093_usize];
_42 = 5778521224312402483_usize ^ 2_usize;
_40.0 = (*_38) < (*_38);
(*_36) = 1114067946_u32 as i8;
(*_38) = _16;
(*_38) = _16 | _1;
(*_44) = [_42,_42,_42];
Goto(bb31)
}
bb31 = {
(*_38) = _1;
_35.0.fld0.fld1 = _39;
(*_10) = core::ptr::addr_of_mut!((*_44));
_33 = [_20.0,_20.0,_20.0];
_43 = [(-150778324240886765145118191323309747917_i128),(-83430698444745316584146410370914617409_i128),120530876582587596223825057018848464824_i128];
(*_38) = (-6867260865848377931_i64) as i32;
_29.fld1 = core::ptr::addr_of_mut!(_15.0);
_20.0 = !3375_i16;
_35.1 = core::ptr::addr_of!(_19);
(*_36) = _15.2 & _15.2;
_28.1 = [(-85459877608675132532788157508715376133_i128),166022360817189555352805049307802353818_i128,75079825763113466341820850516577098081_i128,(-87467044309038695184150537264661543934_i128)];
_10 = core::ptr::addr_of!((*_10));
(*_44) = [_42,_42,_42];
(*_36) = _40.2 + _15.2;
(*_44) = [_42,_42,_42];
_8 = !_28.2.fld1;
(*_38) = _27 as i32;
_46 = _17 | _17;
_26 = [_42,_42,_42];
_41 = 2229762711_u32 as f64;
(*_36) = _15.1 as i8;
_19 = !_15.0;
_17 = _46 & _46;
_40.0 = _46 < _17;
Goto(bb32)
}
bb32 = {
_26 = [_42,_42,_42];
_27 = RET - RET;
(*_36) = _15.2 * _40.2;
(*_44) = [_42,_42,_42];
(*_10) = core::ptr::addr_of_mut!((*_44));
(*_38) = _1 + _16;
(*_38) = _20.2 as i32;
_34 = core::ptr::addr_of_mut!((*_44));
_24 = [(*_36),(*_36),(*_36),(*_36),(*_36),(*_36),(*_36)];
(*_36) = _40.2 << _46;
_24 = [(*_36),(*_36),(*_36),(*_36),(*_36),(*_36),(*_36)];
(*_44) = [_42,_42,_42];
(*_10) = core::ptr::addr_of_mut!((*_44));
(*_36) = -_40.2;
_49 = core::ptr::addr_of_mut!((*_10));
_30 = Adt31::Variant1 { fld0: (*_36),fld1: 59894068399366894652808420681855841952_i128 };
(*_49) = core::ptr::addr_of_mut!((*_44));
_33 = [_20.0,_20.0,_20.0];
_35.1 = core::ptr::addr_of!(_40.0);
_40 = _15;
(*_36) = (*_38) as i8;
Goto(bb33)
}
bb33 = {
_15.2 = _40.3 as i8;
(*_49) = core::ptr::addr_of_mut!((*_44));
(*_44) = [_42,_42,_42];
(*_36) = (-6051452390342912060_i64) as i8;
(*_36) = _20.0 as i8;
_10 = core::ptr::addr_of!((*_10));
(*_44) = [_42,_42,_42];
Goto(bb34)
}
bb34 = {
(*_36) = Field::<i8>(Variant(_30, 1), 0) & _40.2;
_52 = _40.0;
_4 = _3 * _3;
_46 = _27 as isize;
(*_36) = _15.2 | _15.2;
_25 = _27 * _27;
_22 = [_11,_11,_11,_11];
(*_10) = core::ptr::addr_of_mut!((*_44));
(*_10) = core::ptr::addr_of_mut!((*_44));
_34 = Move((*_10));
_29.fld1 = core::ptr::addr_of_mut!(_19);
(*_44) = [_42,_42,_42];
_15.3 = _40.3 | _40.3;
(*_49) = core::ptr::addr_of_mut!((*_44));
(*_36) = _5 as i8;
_59.1 = _20.2 * _20.2;
_55 = [_1,_1,_1,_16,_1,_1];
Goto(bb35)
}
bb35 = {
(*_44) = [_42,_42,_42];
_48 = core::ptr::addr_of_mut!(RET);
(*_48) = _27 * _25;
(*_49) = core::ptr::addr_of_mut!((*_44));
(*_48) = _27 * _25;
(*_48) = _25 + _25;
_63 = _13;
_6 = &_42;
_2.1 = [125431053461774006594989276787091716006_i128,(-95678783048344894097835836008513795850_i128),71778475655998957269268370756661265595_i128,76503901159506167163848981236422328162_i128];
_15 = (_19, _40.1, (*_36), _40.3);
(*_48) = (*_36) as f32;
(*_49) = core::ptr::addr_of_mut!((*_44));
(*_38) = !_1;
_5 = _15.0 | _19;
place!(Field::<i8>(Variant(_21, 1), 0)) = _15.2 - _15.2;
(*_38) = -_1;
Goto(bb36)
}
bb36 = {
Call(_66 = dump_var(4_usize, 23_usize, Move(_23), 52_usize, Move(_52), 8_usize, Move(_8), 24_usize, Move(_24)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_66 = dump_var(4_usize, 46_usize, Move(_46), 43_usize, Move(_43), 33_usize, Move(_33), 16_usize, Move(_16)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_66 = dump_var(4_usize, 17_usize, Move(_17), 19_usize, Move(_19), 26_usize, Move(_26), 67_usize, _67), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i32,mut _2: i32,mut _3: [i128; 4],mut _4: f64,mut _5: i32,mut _6: i32) -> f32 {
mir! {
type RET = f32;
let _7: bool;
let _8: &'static Adt57;
let _9: *mut &'static mut &'static usize;
let _10: Adt73;
let _11: [char; 4];
let _12: (i32,);
let _13: (Adt57, *const bool);
let _14: i8;
let _15: ([char; 4], Adt57);
let _16: [u16; 2];
let _17: [i128; 4];
let _18: Adt73;
let _19: (Adt57, *const bool);
let _20: ();
let _21: ();
{
_6 = -_5;
_5 = !_6;
RET = (-21705_i16) as f32;
_7 = true;
_1 = !_5;
_1 = _2 - _2;
_5 = _7 as i32;
_7 = _4 <= _4;
_4 = 1015507273193437533_u64 as f64;
RET = 172_u8 as f32;
_12.0 = _5;
_6 = !_12.0;
_3 = [(-134740516257510544543236283367142768374_i128),(-85076855318829270201027662019084484510_i128),522855457822427742826493932795787524_i128,118745702951834756316060063466178738101_i128];
_12.0 = RET as i32;
_13.1 = core::ptr::addr_of!(_7);
_2 = _12.0;
Goto(bb1)
}
bb1 = {
_3 = [83985341260051406921722767545645868522_i128,151005289193776156787660171535254429369_i128,(-112753627461622796041943457845762975643_i128),22402514333320647569629960712654390062_i128];
_13.0.fld2 = [35596609676203733615506886949450115032_u128,55149674268958646220403569632684697059_u128,237769306718345384810910131443088768439_u128,74357801516602969620325237295745101742_u128];
_13.0.fld1 = core::ptr::addr_of_mut!(_7);
_16 = [40892_u16,50474_u16];
Call(_13 = fn6(_7, _4, _5, _2, _1, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = Field::<f32>(Variant(_13.0.fld0.fld0, 0), 4) + Field::<f32>(Variant(Field::<Adt18>(Variant(_13.0.fld0.fld0, 0), 2), 0), 2);
place!(Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1)) = !159781251826509905089471773652793550475_i128;
_4 = Field::<usize>(Variant(Field::<Adt18>(Variant(_13.0.fld0.fld0, 0), 2), 0), 1) as f64;
place!(Field::<bool>(Variant(_13.0.fld0.fld0, 0), 0)) = _7;
_3 = [Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1)];
_13.1 = core::ptr::addr_of!(place!(Field::<bool>(Variant(place!(Field::<Adt18>(Variant(_13.0.fld0.fld0, 0), 2)), 0), 0)));
_4 = Field::<i64>(Variant(Field::<Adt18>(Variant(_13.0.fld0.fld0, 0), 2), 0), 3) as f64;
_17 = [Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_13.0.fld0.fld0, 0), 1)];
_15.1 = Adt57 { fld0: Move(_13.0.fld0),fld1: Move(_13.0.fld1),fld2: _13.0.fld2 };
place!(Field::<i128>(Variant(_15.1.fld0.fld0, 0), 1)) = (-81434724948884009230609411578122353893_i128) - (-52542418168277406051877831314187776197_i128);
_13.0 = Move(_15.1);
_11 = ['\u{f6b7e}','\u{be099}','\u{73ecb}','\u{a119c}'];
_15.1.fld1 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(place!(Field::<Adt18>(Variant(_13.0.fld0.fld0, 0), 2)), 0), 0)));
_16 = [45876_u16,3383_u16];
place!(Field::<f32>(Variant(_13.0.fld0.fld0, 0), 4)) = Field::<f32>(Variant(Field::<Adt18>(Variant(_13.0.fld0.fld0, 0), 2), 0), 2) + RET;
_15 = (_11, Move(_13.0));
place!(Field::<i128>(Variant(_15.1.fld0.fld0, 0), 1)) = 17736570695266197880601726538766216664_i128;
_14 = (-29_i8);
_19.0.fld0.fld1 = _15.1.fld0.fld1 >> Field::<u64>(Variant(_15.1.fld0.fld0, 0), 3);
_12 = (_1,);
_5 = _1;
place!(Field::<bool>(Variant(place!(Field::<Adt18>(Variant(_15.1.fld0.fld0, 0), 2)), 0), 0)) = Field::<u64>(Variant(_15.1.fld0.fld0, 0), 3) != Field::<u64>(Variant(_15.1.fld0.fld0, 0), 3);
_19 = (Move(_15.1), Move(_13.1));
Goto(bb3)
}
bb3 = {
Call(_20 = dump_var(5_usize, 17_usize, Move(_17), 14_usize, Move(_14), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_20 = dump_var(5_usize, 6_usize, Move(_6), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: f64,mut _3: i32,mut _4: i32,mut _5: i32,mut _6: i32) -> (Adt57, *const bool) {
mir! {
type RET = (Adt57, *const bool);
let _7: *const &'static mut i128;
let _8: *mut u16;
let _9: i64;
let _10: f64;
let _11: [u16; 2];
let _12: isize;
let _13: i128;
let _14: i64;
let _15: i16;
let _16: Adt31;
let _17: [char; 3];
let _18: [u128; 4];
let _19: u32;
let _20: Adt38;
let _21: *mut f32;
let _22: char;
let _23: isize;
let _24: *mut &'static mut Adt27;
let _25: [u32; 6];
let _26: *const *mut [usize; 3];
let _27: Adt57;
let _28: bool;
let _29: *mut [usize; 3];
let _30: f32;
let _31: i8;
let _32: &'static Adt38;
let _33: &'static *const Adt18;
let _34: *mut u16;
let _35: ([char; 4], Adt57);
let _36: [u32; 6];
let _37: [usize; 3];
let _38: *mut *const bool;
let _39: isize;
let _40: u32;
let _41: &'static mut Adt27;
let _42: *mut *const bool;
let _43: usize;
let _44: u32;
let _45: [u16; 2];
let _46: [i32; 6];
let _47: *mut u16;
let _48: &'static i32;
let _49: &'static Adt38;
let _50: &'static Adt38;
let _51: Adt18;
let _52: u128;
let _53: f32;
let _54: f32;
let _55: isize;
let _56: char;
let _57: bool;
let _58: Adt23;
let _59: i128;
let _60: (i32,);
let _61: *const i8;
let _62: i8;
let _63: [u128; 4];
let _64: Adt27;
let _65: bool;
let _66: [i128; 3];
let _67: [i128; 4];
let _68: char;
let _69: *mut *const bool;
let _70: [i128; 4];
let _71: &'static Adt57;
let _72: isize;
let _73: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _74: *const i8;
let _75: &'static [char; 3];
let _76: Adt57;
let _77: isize;
let _78: *const *const (i32,);
let _79: Adt31;
let _80: (i32,);
let _81: *const *mut u16;
let _82: bool;
let _83: (Adt57, *const bool);
let _84: u128;
let _85: f32;
let _86: *mut &'static mut *mut u16;
let _87: *const *mut u16;
let _88: char;
let _89: i8;
let _90: char;
let _91: char;
let _92: Adt38;
let _93: &'static mut i128;
let _94: &'static (bool, u64, i8, u128);
let _95: &'static [char; 3];
let _96: (bool, u64, i8, u128);
let _97: *const (i32,);
let _98: &'static mut *mut u16;
let _99: isize;
let _100: [i8; 7];
let _101: (i32,);
let _102: &'static &'static mut Adt57;
let _103: *mut u16;
let _104: i64;
let _105: i32;
let _106: isize;
let _107: ();
let _108: ();
{
RET.1 = core::ptr::addr_of!(_1);
_4 = _6;
RET.0.fld0.fld1 = !173_u8;
_4 = '\u{2f91e}' as i32;
RET.0.fld2 = [223183587313705988511919397833510480033_u128,172110649254795559209502980074220474899_u128,22995355608814806859288746667716500484_u128,222839921150362449569908757267818324565_u128];
_6 = _4;
_4 = _5;
Goto(bb1)
}
bb1 = {
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld2 = [5380121295141147408821747866539732195_u128,105836234971860091024031397910044019609_u128,99831748898757125319733256182459510218_u128,119003480158987237554988802693207787163_u128];
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_5 = _3 | _4;
_2 = 275987803851772655651479990506637292762_u128 as f64;
_5 = _4;
RET.1 = core::ptr::addr_of!(_1);
Goto(bb2)
}
bb2 = {
_2 = RET.0.fld0.fld1 as f64;
RET.1 = core::ptr::addr_of!(_1);
_4 = _5 & _5;
_2 = 96387842790399893864903294614286282414_i128 as f64;
_4 = _5 - _5;
_1 = !false;
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_4 = _3;
RET.0.fld2 = [120691601603433787233746164898516173001_u128,231081702542952952785124127125400605188_u128,235675061769396259133431340967234330838_u128,117493231051545630269723956697105066205_u128];
RET.0.fld0.fld1 = 136_u8 & 24_u8;
RET.0.fld2 = [96196131649315944263473346424967846477_u128,168300025300014244308014490809191962055_u128,255159226861401874121254968964177047374_u128,122717646642650408139491841932171282825_u128];
_2 = 16739262905647887431_u64 as f64;
RET.0.fld2 = [97111565094367871294445585883806684241_u128,72999413128252091132425820785255864927_u128,213288834360304346410327391889565960685_u128,328192383094148883540448773407937414141_u128];
RET.0.fld2 = [207961900654033344531463964474767095131_u128,226246583027907153616582107776092032684_u128,294498026790908586806182132338231029497_u128,26938840497057690797246370440674044843_u128];
RET.1 = core::ptr::addr_of!(_1);
_2 = (-85_isize) as f64;
Goto(bb3)
}
bb3 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb4 = {
_12 = 69_isize & 66_isize;
_14 = 95300757_u32 as i64;
_15 = 27464_i16 * (-4245_i16);
match _9 {
0 => bb1,
340282366920938463455239072544917800604 => bb6,
_ => bb5
}
}
bb5 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb6 = {
_9 = _14 << _12;
_11 = [64370_u16,46837_u16];
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_10 = 6_i8 as f64;
_20.fld5 = _15 as u128;
_19 = 577896809_u32;
_20.fld5 = _15 as u128;
_20.fld0 = core::ptr::addr_of_mut!(_1);
Goto(bb7)
}
bb7 = {
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_11 = [6809_u16,29357_u16];
_6 = _3 * _3;
_12 = (-90_isize) + 43_isize;
_2 = _10 * _10;
_17 = ['\u{12863}','\u{fcebd}','\u{c5d0b}'];
_20.fld7 = [_15,_15,_15];
_20.fld6 = _14 - _9;
_23 = !_12;
_20.fld4 = !1_usize;
_20.fld1 = _2 - _2;
RET.1 = core::ptr::addr_of!(_1);
_18 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_20.fld5 = !24048973597412634878327756685351936893_u128;
_20.fld1 = _2 - _10;
_6 = _3 ^ _3;
_10 = -_20.fld1;
_22 = '\u{6eb25}';
_20.fld2 = core::ptr::addr_of!(_1);
RET.1 = core::ptr::addr_of!(_1);
RET.1 = core::ptr::addr_of!(_1);
_14 = _20.fld6 - _20.fld6;
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_20.fld6 = _20.fld4 as i64;
_20.fld2 = core::ptr::addr_of!(_1);
Call(_9 = fn7(), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = _10 + _20.fld1;
_18 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_20.fld2 = core::ptr::addr_of!(_1);
_17 = [_22,_22,_22];
_13 = 20429950622369730960193201303654034123_i128 << _9;
_5 = _3;
_16 = Adt31::Variant1 { fld0: 43_i8,fld1: _13 };
_20.fld7 = [_15,_15,_15];
RET.0.fld2 = _18;
_25 = [_19,_19,_19,_19,_19,_19];
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_15 = 16160_i16 << _13;
match _19 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
577896809 => bb10,
_ => bb9
}
}
bb9 = {
_12 = 69_isize & 66_isize;
_14 = 95300757_u32 as i64;
_15 = 27464_i16 * (-4245_i16);
match _9 {
0 => bb1,
340282366920938463455239072544917800604 => bb6,
_ => bb5
}
}
bb10 = {
_17 = [_22,_22,_22];
_15 = 14571_i16 & 6456_i16;
RET.0.fld0.fld1 = !119_u8;
_16 = Adt31::Variant1 { fld0: (-116_i8),fld1: _13 };
_17 = [_22,_22,_22];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.1 = core::ptr::addr_of!(_1);
_1 = _5 == _6;
_25 = [_19,_19,_19,_19,_19,_19];
_27.fld0.fld1 = _12 as u8;
_5 = !_3;
place!(Field::<i128>(Variant(_16, 1), 1)) = _13;
_20.fld4 = 4573700492020095008_usize;
_20.fld0 = core::ptr::addr_of_mut!(_28);
_2 = _10;
_22 = '\u{981ff}';
_6 = !_4;
_16 = Adt31::Variant1 { fld0: (-75_i8),fld1: _13 };
_6 = _5 * _4;
_19 = !4271412820_u32;
_26 = core::ptr::addr_of!(_29);
match _20.fld4 {
0 => bb8,
1 => bb9,
2 => bb6,
3 => bb4,
4 => bb11,
4573700492020095008 => bb13,
_ => bb12
}
}
bb11 = {
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld2 = [5380121295141147408821747866539732195_u128,105836234971860091024031397910044019609_u128,99831748898757125319733256182459510218_u128,119003480158987237554988802693207787163_u128];
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_5 = _3 | _4;
_2 = 275987803851772655651479990506637292762_u128 as f64;
_5 = _4;
RET.1 = core::ptr::addr_of!(_1);
Goto(bb2)
}
bb12 = {
_2 = RET.0.fld0.fld1 as f64;
RET.1 = core::ptr::addr_of!(_1);
_4 = _5 & _5;
_2 = 96387842790399893864903294614286282414_i128 as f64;
_4 = _5 - _5;
_1 = !false;
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_4 = _3;
RET.0.fld2 = [120691601603433787233746164898516173001_u128,231081702542952952785124127125400605188_u128,235675061769396259133431340967234330838_u128,117493231051545630269723956697105066205_u128];
RET.0.fld0.fld1 = 136_u8 & 24_u8;
RET.0.fld2 = [96196131649315944263473346424967846477_u128,168300025300014244308014490809191962055_u128,255159226861401874121254968964177047374_u128,122717646642650408139491841932171282825_u128];
_2 = 16739262905647887431_u64 as f64;
RET.0.fld2 = [97111565094367871294445585883806684241_u128,72999413128252091132425820785255864927_u128,213288834360304346410327391889565960685_u128,328192383094148883540448773407937414141_u128];
RET.0.fld2 = [207961900654033344531463964474767095131_u128,226246583027907153616582107776092032684_u128,294498026790908586806182132338231029497_u128,26938840497057690797246370440674044843_u128];
RET.1 = core::ptr::addr_of!(_1);
_2 = (-85_isize) as f64;
Goto(bb3)
}
bb13 = {
_20.fld5 = 49472082335761326187723051600339280384_u128 - 233976410343442883579470867376840944776_u128;
_15 = (-23445_i16) >> _6;
place!(Field::<i128>(Variant(_16, 1), 1)) = _13 | _13;
place!(Field::<i8>(Variant(_16, 1), 0)) = _14 as i8;
_20.fld1 = _20.fld6 as f64;
_15 = -(-11352_i16);
_4 = _1 as i32;
_22 = '\u{6a334}';
_13 = Field::<i128>(Variant(_16, 1), 1) + Field::<i128>(Variant(_16, 1), 1);
match _20.fld4 {
0 => bb14,
1 => bb15,
4573700492020095008 => bb17,
_ => bb16
}
}
bb14 = {
_2 = RET.0.fld0.fld1 as f64;
RET.1 = core::ptr::addr_of!(_1);
_4 = _5 & _5;
_2 = 96387842790399893864903294614286282414_i128 as f64;
_4 = _5 - _5;
_1 = !false;
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_4 = _3;
RET.0.fld2 = [120691601603433787233746164898516173001_u128,231081702542952952785124127125400605188_u128,235675061769396259133431340967234330838_u128,117493231051545630269723956697105066205_u128];
RET.0.fld0.fld1 = 136_u8 & 24_u8;
RET.0.fld2 = [96196131649315944263473346424967846477_u128,168300025300014244308014490809191962055_u128,255159226861401874121254968964177047374_u128,122717646642650408139491841932171282825_u128];
_2 = 16739262905647887431_u64 as f64;
RET.0.fld2 = [97111565094367871294445585883806684241_u128,72999413128252091132425820785255864927_u128,213288834360304346410327391889565960685_u128,328192383094148883540448773407937414141_u128];
RET.0.fld2 = [207961900654033344531463964474767095131_u128,226246583027907153616582107776092032684_u128,294498026790908586806182132338231029497_u128,26938840497057690797246370440674044843_u128];
RET.1 = core::ptr::addr_of!(_1);
_2 = (-85_isize) as f64;
Goto(bb3)
}
bb15 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb16 = {
_17 = [_22,_22,_22];
_15 = 14571_i16 & 6456_i16;
RET.0.fld0.fld1 = !119_u8;
_16 = Adt31::Variant1 { fld0: (-116_i8),fld1: _13 };
_17 = [_22,_22,_22];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.1 = core::ptr::addr_of!(_1);
_1 = _5 == _6;
_25 = [_19,_19,_19,_19,_19,_19];
_27.fld0.fld1 = _12 as u8;
_5 = !_3;
place!(Field::<i128>(Variant(_16, 1), 1)) = _13;
_20.fld4 = 4573700492020095008_usize;
_20.fld0 = core::ptr::addr_of_mut!(_28);
_2 = _10;
_22 = '\u{981ff}';
_6 = !_4;
_16 = Adt31::Variant1 { fld0: (-75_i8),fld1: _13 };
_6 = _5 * _4;
_19 = !4271412820_u32;
_26 = core::ptr::addr_of!(_29);
match _20.fld4 {
0 => bb8,
1 => bb9,
2 => bb6,
3 => bb4,
4 => bb11,
4573700492020095008 => bb13,
_ => bb12
}
}
bb17 = {
_15 = (-14519_i16) | 5471_i16;
_12 = !_23;
_30 = _4 as f32;
_31 = !Field::<i8>(Variant(_16, 1), 0);
_28 = _27.fld0.fld1 <= _27.fld0.fld1;
_27.fld1 = core::ptr::addr_of_mut!(_1);
_20.fld6 = _2 as i64;
_19 = 1049601018_u32 * 2262466065_u32;
_20.fld0 = core::ptr::addr_of_mut!(_1);
_26 = core::ptr::addr_of!((*_26));
_22 = '\u{10477f}';
_35.1.fld1 = core::ptr::addr_of_mut!(_28);
_35.0 = [_22,_22,_22,_22];
_5 = _20.fld1 as i32;
_35.1.fld0.fld1 = _1 as u8;
_11 = [10759_u16,39509_u16];
_14 = _20.fld6 << _13;
_29 = core::ptr::addr_of_mut!(_37);
(*_26) = core::ptr::addr_of_mut!((*_29));
_21 = core::ptr::addr_of_mut!(_30);
(*_21) = _4 as f32;
(*_21) = _6 as f32;
(*_21) = 14068381066165699190_u64 as f32;
_14 = 549273938563338794_u64 as i64;
match _20.fld4 {
0 => bb18,
1 => bb19,
2 => bb20,
3 => bb21,
4573700492020095008 => bb23,
_ => bb22
}
}
bb18 = {
_2 = _10 + _20.fld1;
_18 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_20.fld2 = core::ptr::addr_of!(_1);
_17 = [_22,_22,_22];
_13 = 20429950622369730960193201303654034123_i128 << _9;
_5 = _3;
_16 = Adt31::Variant1 { fld0: 43_i8,fld1: _13 };
_20.fld7 = [_15,_15,_15];
RET.0.fld2 = _18;
_25 = [_19,_19,_19,_19,_19,_19];
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_15 = 16160_i16 << _13;
match _19 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
577896809 => bb10,
_ => bb9
}
}
bb19 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb20 = {
_2 = RET.0.fld0.fld1 as f64;
RET.1 = core::ptr::addr_of!(_1);
_4 = _5 & _5;
_2 = 96387842790399893864903294614286282414_i128 as f64;
_4 = _5 - _5;
_1 = !false;
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_4 = _3;
RET.0.fld2 = [120691601603433787233746164898516173001_u128,231081702542952952785124127125400605188_u128,235675061769396259133431340967234330838_u128,117493231051545630269723956697105066205_u128];
RET.0.fld0.fld1 = 136_u8 & 24_u8;
RET.0.fld2 = [96196131649315944263473346424967846477_u128,168300025300014244308014490809191962055_u128,255159226861401874121254968964177047374_u128,122717646642650408139491841932171282825_u128];
_2 = 16739262905647887431_u64 as f64;
RET.0.fld2 = [97111565094367871294445585883806684241_u128,72999413128252091132425820785255864927_u128,213288834360304346410327391889565960685_u128,328192383094148883540448773407937414141_u128];
RET.0.fld2 = [207961900654033344531463964474767095131_u128,226246583027907153616582107776092032684_u128,294498026790908586806182132338231029497_u128,26938840497057690797246370440674044843_u128];
RET.1 = core::ptr::addr_of!(_1);
_2 = (-85_isize) as f64;
Goto(bb3)
}
bb21 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb22 = {
_2 = RET.0.fld0.fld1 as f64;
RET.1 = core::ptr::addr_of!(_1);
_4 = _5 & _5;
_2 = 96387842790399893864903294614286282414_i128 as f64;
_4 = _5 - _5;
_1 = !false;
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_4 = _3;
RET.0.fld2 = [120691601603433787233746164898516173001_u128,231081702542952952785124127125400605188_u128,235675061769396259133431340967234330838_u128,117493231051545630269723956697105066205_u128];
RET.0.fld0.fld1 = 136_u8 & 24_u8;
RET.0.fld2 = [96196131649315944263473346424967846477_u128,168300025300014244308014490809191962055_u128,255159226861401874121254968964177047374_u128,122717646642650408139491841932171282825_u128];
_2 = 16739262905647887431_u64 as f64;
RET.0.fld2 = [97111565094367871294445585883806684241_u128,72999413128252091132425820785255864927_u128,213288834360304346410327391889565960685_u128,328192383094148883540448773407937414141_u128];
RET.0.fld2 = [207961900654033344531463964474767095131_u128,226246583027907153616582107776092032684_u128,294498026790908586806182132338231029497_u128,26938840497057690797246370440674044843_u128];
RET.1 = core::ptr::addr_of!(_1);
_2 = (-85_isize) as f64;
Goto(bb3)
}
bb23 = {
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_21) = _35.1.fld0.fld1 as f32;
(*_21) = _15 as f32;
Goto(bb24)
}
bb24 = {
(*_26) = core::ptr::addr_of_mut!((*_29));
_20.fld4 = 7532448650867641804_usize;
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
_25 = [_19,_19,_19,_19,_19,_19];
Goto(bb25)
}
bb25 = {
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_21) = _23 as f32;
(*_21) = _20.fld5 as f32;
_23 = -_12;
(*_21) = _13 as f32;
_38 = core::ptr::addr_of_mut!(_20.fld2);
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
(*_26) = core::ptr::addr_of_mut!((*_29));
_39 = _12 | _23;
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
match _20.fld4 {
0 => bb5,
1 => bb12,
2 => bb23,
3 => bb26,
4 => bb27,
7532448650867641804 => bb29,
_ => bb28
}
}
bb26 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb27 = {
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_21) = _35.1.fld0.fld1 as f32;
(*_21) = _15 as f32;
Goto(bb24)
}
bb28 = {
_17 = [_22,_22,_22];
_15 = 14571_i16 & 6456_i16;
RET.0.fld0.fld1 = !119_u8;
_16 = Adt31::Variant1 { fld0: (-116_i8),fld1: _13 };
_17 = [_22,_22,_22];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.1 = core::ptr::addr_of!(_1);
_1 = _5 == _6;
_25 = [_19,_19,_19,_19,_19,_19];
_27.fld0.fld1 = _12 as u8;
_5 = !_3;
place!(Field::<i128>(Variant(_16, 1), 1)) = _13;
_20.fld4 = 4573700492020095008_usize;
_20.fld0 = core::ptr::addr_of_mut!(_28);
_2 = _10;
_22 = '\u{981ff}';
_6 = !_4;
_16 = Adt31::Variant1 { fld0: (-75_i8),fld1: _13 };
_6 = _5 * _4;
_19 = !4271412820_u32;
_26 = core::ptr::addr_of!(_29);
match _20.fld4 {
0 => bb8,
1 => bb9,
2 => bb6,
3 => bb4,
4 => bb11,
4573700492020095008 => bb13,
_ => bb12
}
}
bb29 = {
(*_38) = Move(RET.1);
(*_38) = core::ptr::addr_of!(_1);
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
_31 = Field::<i8>(Variant(_16, 1), 0) << _9;
(*_38) = core::ptr::addr_of!(_1);
(*_21) = _20.fld5 as f32;
_4 = _20.fld5 as i32;
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_38) = core::ptr::addr_of!(_28);
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_21) = _15 as f32;
Goto(bb30)
}
bb30 = {
(*_26) = core::ptr::addr_of_mut!((*_29));
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
place!(Field::<i128>(Variant(_16, 1), 1)) = -_13;
(*_38) = core::ptr::addr_of!(_1);
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_26) = core::ptr::addr_of_mut!(_37);
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
Goto(bb31)
}
bb31 = {
_17 = [_22,_22,_22];
(*_26) = core::ptr::addr_of_mut!((*_29));
_6 = _20.fld4 as i32;
(*_26) = core::ptr::addr_of_mut!((*_29));
_17 = [_22,_22,_22];
(*_38) = core::ptr::addr_of!(_1);
(*_38) = core::ptr::addr_of!(_1);
(*_26) = core::ptr::addr_of_mut!((*_29));
_19 = 4003338097_u32;
(*_21) = _39 as f32;
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_21) = _20.fld4 as f32;
_21 = core::ptr::addr_of_mut!((*_21));
(*_38) = core::ptr::addr_of!(_1);
_20.fld0 = core::ptr::addr_of_mut!(_1);
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_38) = core::ptr::addr_of!(_28);
_11 = [30759_u16,11125_u16];
(*_21) = _20.fld1 as f32;
place!(Field::<i8>(Variant(_16, 1), 0)) = -_31;
_10 = _2 - _2;
(*_21) = _13 as f32;
(*_21) = Field::<i128>(Variant(_16, 1), 1) as f32;
_39 = !_12;
_6 = !_4;
_27.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_38) = core::ptr::addr_of!(_1);
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_38) = core::ptr::addr_of!(_28);
match _19 {
4003338097 => bb33,
_ => bb32
}
}
bb32 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb33 = {
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_21) = _14 as f32;
_22 = '\u{940db}';
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
_13 = Field::<i128>(Variant(_16, 1), 1);
_6 = _3 * _4;
(*_26) = core::ptr::addr_of_mut!(_37);
_43 = _20.fld4 << _19;
(*_38) = core::ptr::addr_of!(_1);
(*_21) = 44152_u16 as f32;
_44 = _19 & _19;
(*_38) = core::ptr::addr_of!(_1);
(*_38) = core::ptr::addr_of!(_28);
(*_38) = core::ptr::addr_of!(_1);
_40 = _44 % _19;
_20.fld5 = 283182350851842117982349622177963206285_u128 * 137123719369217650429060868158670179264_u128;
Goto(bb34)
}
bb34 = {
_20.fld4 = 3463_u16 as usize;
_48 = &_3;
(*_38) = core::ptr::addr_of!(_1);
(*_21) = _20.fld5 as f32;
(*_26) = core::ptr::addr_of_mut!((*_29));
_35.1.fld1 = Move(_27.fld1);
(*_38) = core::ptr::addr_of!(_1);
match _19 {
0 => bb17,
1 => bb30,
2 => bb35,
3 => bb36,
4003338097 => bb38,
_ => bb37
}
}
bb35 = {
_2 = _10 + _20.fld1;
_18 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_20.fld2 = core::ptr::addr_of!(_1);
_17 = [_22,_22,_22];
_13 = 20429950622369730960193201303654034123_i128 << _9;
_5 = _3;
_16 = Adt31::Variant1 { fld0: 43_i8,fld1: _13 };
_20.fld7 = [_15,_15,_15];
RET.0.fld2 = _18;
_25 = [_19,_19,_19,_19,_19,_19];
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_15 = 16160_i16 << _13;
match _19 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
577896809 => bb10,
_ => bb9
}
}
bb36 = {
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld2 = [5380121295141147408821747866539732195_u128,105836234971860091024031397910044019609_u128,99831748898757125319733256182459510218_u128,119003480158987237554988802693207787163_u128];
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.1 = core::ptr::addr_of!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_5 = _3 | _4;
_2 = 275987803851772655651479990506637292762_u128 as f64;
_5 = _4;
RET.1 = core::ptr::addr_of!(_1);
Goto(bb2)
}
bb37 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb38 = {
_20.fld1 = Field::<i8>(Variant(_16, 1), 0) as f64;
_35.1.fld2 = RET.0.fld2;
RET.1 = core::ptr::addr_of!(_1);
_12 = -_39;
(*_29) = [_43,_20.fld4,_43];
(*_21) = _40 as f32;
(*_26) = core::ptr::addr_of_mut!((*_29));
_35.1.fld2 = RET.0.fld2;
(*_38) = core::ptr::addr_of!(_28);
_1 = _28;
(*_38) = Move(RET.1);
(*_21) = _40 as f32;
(*_29) = [_43,_43,_43];
(*_38) = core::ptr::addr_of!(_28);
(*_21) = _14 as f32;
(*_38) = core::ptr::addr_of!(_28);
(*_21) = _39 as f32;
_39 = _12;
(*_26) = core::ptr::addr_of_mut!((*_29));
_22 = '\u{d35a3}';
_45 = [36711_u16,29414_u16];
_38 = core::ptr::addr_of_mut!((*_38));
_21 = core::ptr::addr_of_mut!((*_21));
_52 = _20.fld5;
(*_26) = core::ptr::addr_of_mut!((*_29));
Goto(bb39)
}
bb39 = {
_31 = Field::<i8>(Variant(_16, 1), 0);
(*_21) = _31 as f32;
(*_21) = Field::<i8>(Variant(_16, 1), 0) as f32;
_36 = [_40,_40,_44,_40,_44,_40];
_14 = _9 - _20.fld6;
(*_29) = [_43,_20.fld4,_43];
match _19 {
0 => bb5,
1 => bb25,
4003338097 => bb40,
_ => bb15
}
}
bb40 = {
_20.fld0 = core::ptr::addr_of_mut!(_28);
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_26) = core::ptr::addr_of_mut!((*_29));
_21 = core::ptr::addr_of_mut!((*_21));
(*_29) = [_43,_43,_43];
_54 = (*_21) + (*_21);
_20.fld5 = _20.fld1 as u128;
(*_29) = [_43,_20.fld4,_20.fld4];
(*_21) = _54;
(*_38) = core::ptr::addr_of!(_1);
(*_29) = [_20.fld4,_43,_20.fld4];
(*_26) = core::ptr::addr_of_mut!((*_29));
_35.0 = [_22,_22,_22,_22];
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_38) = core::ptr::addr_of!(_28);
(*_21) = _54 - _54;
_42 = core::ptr::addr_of_mut!((*_38));
_55 = _39 - _39;
_27.fld0.fld1 = _20.fld5 as u8;
Goto(bb41)
}
bb41 = {
(*_26) = core::ptr::addr_of_mut!((*_29));
_48 = &_4;
(*_21) = -_54;
(*_38) = core::ptr::addr_of!(_1);
_36 = _25;
(*_38) = core::ptr::addr_of!(_28);
(*_29) = [_43,_43,_43];
(*_29) = [_43,_43,_20.fld4];
_61 = core::ptr::addr_of!(place!(Field::<i8>(Variant(_16, 1), 0)));
(*_61) = _31 | _31;
(*_29) = [_43,_43,_43];
place!(Field::<i8>(Variant(_16, 1), 0)) = !_31;
_9 = _1 as i64;
_27.fld2 = _35.1.fld2;
(*_38) = core::ptr::addr_of!(_65);
(*_29) = [_20.fld4,_20.fld4,_43];
(*_21) = _20.fld1 as f32;
(*_21) = -_54;
Goto(bb42)
}
bb42 = {
(*_21) = _54 - _54;
_27.fld1 = core::ptr::addr_of_mut!(_65);
_26 = core::ptr::addr_of!((*_26));
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_61) = _31 * _31;
_39 = _23 << (*_61);
match _19 {
0 => bb6,
1 => bb43,
2 => bb44,
3 => bb45,
4 => bb46,
5 => bb47,
6 => bb48,
4003338097 => bb50,
_ => bb49
}
}
bb43 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb44 = {
_17 = [_22,_22,_22];
(*_26) = core::ptr::addr_of_mut!((*_29));
_6 = _20.fld4 as i32;
(*_26) = core::ptr::addr_of_mut!((*_29));
_17 = [_22,_22,_22];
(*_38) = core::ptr::addr_of!(_1);
(*_38) = core::ptr::addr_of!(_1);
(*_26) = core::ptr::addr_of_mut!((*_29));
_19 = 4003338097_u32;
(*_21) = _39 as f32;
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_21) = _20.fld4 as f32;
_21 = core::ptr::addr_of_mut!((*_21));
(*_38) = core::ptr::addr_of!(_1);
_20.fld0 = core::ptr::addr_of_mut!(_1);
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_38) = core::ptr::addr_of!(_28);
_11 = [30759_u16,11125_u16];
(*_21) = _20.fld1 as f32;
place!(Field::<i8>(Variant(_16, 1), 0)) = -_31;
_10 = _2 - _2;
(*_21) = _13 as f32;
(*_21) = Field::<i128>(Variant(_16, 1), 1) as f32;
_39 = !_12;
_6 = !_4;
_27.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
(*_38) = core::ptr::addr_of!(_1);
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_38) = core::ptr::addr_of!(_28);
match _19 {
4003338097 => bb33,
_ => bb32
}
}
bb45 = {
_2 = _10 + _20.fld1;
_18 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_20.fld2 = core::ptr::addr_of!(_1);
_17 = [_22,_22,_22];
_13 = 20429950622369730960193201303654034123_i128 << _9;
_5 = _3;
_16 = Adt31::Variant1 { fld0: 43_i8,fld1: _13 };
_20.fld7 = [_15,_15,_15];
RET.0.fld2 = _18;
_25 = [_19,_19,_19,_19,_19,_19];
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_15 = 16160_i16 << _13;
match _19 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
577896809 => bb10,
_ => bb9
}
}
bb46 = {
_17 = [_22,_22,_22];
_15 = 14571_i16 & 6456_i16;
RET.0.fld0.fld1 = !119_u8;
_16 = Adt31::Variant1 { fld0: (-116_i8),fld1: _13 };
_17 = [_22,_22,_22];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.1 = core::ptr::addr_of!(_1);
_1 = _5 == _6;
_25 = [_19,_19,_19,_19,_19,_19];
_27.fld0.fld1 = _12 as u8;
_5 = !_3;
place!(Field::<i128>(Variant(_16, 1), 1)) = _13;
_20.fld4 = 4573700492020095008_usize;
_20.fld0 = core::ptr::addr_of_mut!(_28);
_2 = _10;
_22 = '\u{981ff}';
_6 = !_4;
_16 = Adt31::Variant1 { fld0: (-75_i8),fld1: _13 };
_6 = _5 * _4;
_19 = !4271412820_u32;
_26 = core::ptr::addr_of!(_29);
match _20.fld4 {
0 => bb8,
1 => bb9,
2 => bb6,
3 => bb4,
4 => bb11,
4573700492020095008 => bb13,
_ => bb12
}
}
bb47 = {
_17 = [_22,_22,_22];
_15 = 14571_i16 & 6456_i16;
RET.0.fld0.fld1 = !119_u8;
_16 = Adt31::Variant1 { fld0: (-116_i8),fld1: _13 };
_17 = [_22,_22,_22];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.1 = core::ptr::addr_of!(_1);
_1 = _5 == _6;
_25 = [_19,_19,_19,_19,_19,_19];
_27.fld0.fld1 = _12 as u8;
_5 = !_3;
place!(Field::<i128>(Variant(_16, 1), 1)) = _13;
_20.fld4 = 4573700492020095008_usize;
_20.fld0 = core::ptr::addr_of_mut!(_28);
_2 = _10;
_22 = '\u{981ff}';
_6 = !_4;
_16 = Adt31::Variant1 { fld0: (-75_i8),fld1: _13 };
_6 = _5 * _4;
_19 = !4271412820_u32;
_26 = core::ptr::addr_of!(_29);
match _20.fld4 {
0 => bb8,
1 => bb9,
2 => bb6,
3 => bb4,
4 => bb11,
4573700492020095008 => bb13,
_ => bb12
}
}
bb48 = {
_2 = _10 + _20.fld1;
_18 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_20.fld2 = core::ptr::addr_of!(_1);
_17 = [_22,_22,_22];
_13 = 20429950622369730960193201303654034123_i128 << _9;
_5 = _3;
_16 = Adt31::Variant1 { fld0: 43_i8,fld1: _13 };
_20.fld7 = [_15,_15,_15];
RET.0.fld2 = _18;
_25 = [_19,_19,_19,_19,_19,_19];
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_15 = 16160_i16 << _13;
match _19 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
577896809 => bb10,
_ => bb9
}
}
bb49 = {
_2 = RET.0.fld0.fld1 as f64;
RET.1 = core::ptr::addr_of!(_1);
_4 = _5 & _5;
_2 = 96387842790399893864903294614286282414_i128 as f64;
_4 = _5 - _5;
_1 = !false;
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
RET.0.fld1 = core::ptr::addr_of_mut!(_1);
_4 = _3;
RET.0.fld2 = [120691601603433787233746164898516173001_u128,231081702542952952785124127125400605188_u128,235675061769396259133431340967234330838_u128,117493231051545630269723956697105066205_u128];
RET.0.fld0.fld1 = 136_u8 & 24_u8;
RET.0.fld2 = [96196131649315944263473346424967846477_u128,168300025300014244308014490809191962055_u128,255159226861401874121254968964177047374_u128,122717646642650408139491841932171282825_u128];
_2 = 16739262905647887431_u64 as f64;
RET.0.fld2 = [97111565094367871294445585883806684241_u128,72999413128252091132425820785255864927_u128,213288834360304346410327391889565960685_u128,328192383094148883540448773407937414141_u128];
RET.0.fld2 = [207961900654033344531463964474767095131_u128,226246583027907153616582107776092032684_u128,294498026790908586806182132338231029497_u128,26938840497057690797246370440674044843_u128];
RET.1 = core::ptr::addr_of!(_1);
_2 = (-85_isize) as f64;
Goto(bb3)
}
bb50 = {
(*_26) = core::ptr::addr_of_mut!((*_29));
_60.0 = (*_48);
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_21) = _54 - _54;
_65 = _1 | _1;
_63 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_59 = _9 as i128;
(*_26) = core::ptr::addr_of_mut!((*_29));
_31 = (*_61) + (*_61);
_29 = core::ptr::addr_of_mut!((*_29));
match _19 {
0 => bb1,
1 => bb38,
2 => bb12,
3 => bb39,
4 => bb23,
5 => bb18,
6 => bb51,
4003338097 => bb53,
_ => bb52
}
}
bb51 = {
_17 = [_22,_22,_22];
_15 = 14571_i16 & 6456_i16;
RET.0.fld0.fld1 = !119_u8;
_16 = Adt31::Variant1 { fld0: (-116_i8),fld1: _13 };
_17 = [_22,_22,_22];
RET.0.fld2 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
RET.1 = core::ptr::addr_of!(_1);
_1 = _5 == _6;
_25 = [_19,_19,_19,_19,_19,_19];
_27.fld0.fld1 = _12 as u8;
_5 = !_3;
place!(Field::<i128>(Variant(_16, 1), 1)) = _13;
_20.fld4 = 4573700492020095008_usize;
_20.fld0 = core::ptr::addr_of_mut!(_28);
_2 = _10;
_22 = '\u{981ff}';
_6 = !_4;
_16 = Adt31::Variant1 { fld0: (-75_i8),fld1: _13 };
_6 = _5 * _4;
_19 = !4271412820_u32;
_26 = core::ptr::addr_of!(_29);
match _20.fld4 {
0 => bb8,
1 => bb9,
2 => bb6,
3 => bb4,
4 => bb11,
4573700492020095008 => bb13,
_ => bb12
}
}
bb52 = {
RET.0.fld0.fld1 = 15081528076343772888_usize as u8;
_5 = _3 << RET.0.fld0.fld1;
_11 = [26098_u16,24682_u16];
_10 = _2 - _2;
_2 = 18306678810045024796_usize as f64;
_9 = (-8135534886850410852_i64);
_3 = 2579224155739839395_usize as i32;
RET.0.fld0.fld1 = 159_u8;
_3 = _4 - _5;
_6 = !_4;
_3 = _4 | _6;
RET.0.fld0.fld1 = 36_u8 & 182_u8;
_10 = (-11746_i16) as f64;
_12 = 9223372036854775807_isize << _6;
_6 = '\u{608ce}' as i32;
_13 = (-80623298171971433092028150157262577314_i128) << _3;
Goto(bb4)
}
bb53 = {
_65 = _1 ^ _28;
(*_61) = _31;
_17 = [_22,_22,_22];
(*_21) = _54 * _54;
_2 = Field::<i8>(Variant(_16, 1), 0) as f64;
_37 = [_43,_43,_43];
(*_26) = core::ptr::addr_of_mut!((*_29));
_20.fld2 = core::ptr::addr_of!(_1);
(*_21) = _54 - _54;
(*_61) = !_31;
_4 = _3 - _6;
_62 = !(*_61);
_27.fld2 = _63;
(*_61) = _28 as i8;
_35.0 = [_22,_22,_22,_22];
(*_26) = core::ptr::addr_of_mut!((*_29));
_56 = _22;
_27.fld1 = Move(_35.1.fld1);
(*_21) = -_54;
_19 = _27.fld0.fld1 as u32;
_51 = Adt18::Variant0 { fld0: _1,fld1: _43,fld2: (*_21),fld3: _9,fld4: 10264701183256219032_u64 };
_29 = core::ptr::addr_of_mut!((*_29));
(*_38) = core::ptr::addr_of!(_57);
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_21) = -Field::<f32>(Variant(_51, 0), 2);
_14 = Field::<i64>(Variant(_51, 0), 3);
(*_21) = Field::<f32>(Variant(_51, 0), 2);
_35.0 = [_22,_56,_22,_56];
(*_26) = core::ptr::addr_of_mut!((*_29));
Goto(bb54)
}
bb54 = {
_38 = core::ptr::addr_of_mut!((*_38));
_9 = Field::<i64>(Variant(_51, 0), 3) * _14;
(*_21) = Field::<f32>(Variant(_51, 0), 2) * _54;
_54 = -(*_21);
(*_26) = core::ptr::addr_of_mut!((*_29));
_43 = _20.fld4;
(*_21) = _54 * Field::<f32>(Variant(_51, 0), 2);
(*_61) = _62 | _62;
_69 = core::ptr::addr_of_mut!((*_38));
(*_29) = [Field::<usize>(Variant(_51, 0), 1),_20.fld4,Field::<usize>(Variant(_51, 0), 1)];
(*_26) = core::ptr::addr_of_mut!((*_29));
_38 = core::ptr::addr_of_mut!((*_69));
_35.1.fld0.fld1 = _2 as u8;
_44 = _19 - _19;
Call(_5 = core::intrinsics::bswap(_4), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
(*_61) = _62 | _31;
_26 = core::ptr::addr_of!((*_26));
(*_29) = [_43,Field::<usize>(Variant(_51, 0), 1),_43];
(*_21) = 11207431966540489733_u64 as f32;
(*_61) = _62 & _62;
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_21) = Field::<f32>(Variant(_51, 0), 2) * _54;
(*_38) = core::ptr::addr_of!(_57);
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_21) = _54 + Field::<f32>(Variant(_51, 0), 2);
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_38) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_51, 0), 0)));
_18 = [_20.fld5,_20.fld5,_20.fld5,_20.fld5];
_3 = _6 - _6;
place!(Field::<i128>(Variant(_16, 1), 1)) = _13 ^ _13;
(*_38) = core::ptr::addr_of!(_57);
(*_69) = core::ptr::addr_of!(_28);
Goto(bb56)
}
bb56 = {
_44 = !_19;
(*_26) = core::ptr::addr_of_mut!((*_29));
_15 = (-15949_i16) << (*_61);
(*_69) = core::ptr::addr_of!(_1);
(*_69) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_51, 0), 0)));
(*_21) = (*_61) as f32;
(*_69) = core::ptr::addr_of!(_28);
_9 = Field::<i64>(Variant(_51, 0), 3) << Field::<i8>(Variant(_16, 1), 0);
_74 = core::ptr::addr_of!((*_61));
(*_69) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_51, 0), 0)));
_73.1 = [_13,_59,Field::<i128>(Variant(_16, 1), 1),Field::<i128>(Variant(_16, 1), 1)];
(*_69) = core::ptr::addr_of!(_28);
_3 = _4 ^ _60.0;
(*_61) = _31 * _62;
_73.1 = [_13,Field::<i128>(Variant(_16, 1), 1),Field::<i128>(Variant(_16, 1), 1),_13];
(*_26) = core::ptr::addr_of_mut!((*_29));
Goto(bb57)
}
bb57 = {
_35.1.fld1 = core::ptr::addr_of_mut!(_28);
RET.1 = core::ptr::addr_of!(_57);
_79 = Adt31::Variant1 { fld0: (*_61),fld1: Field::<i128>(Variant(_16, 1), 1) };
(*_69) = core::ptr::addr_of!(_82);
place!(Field::<u64>(Variant(_51, 0), 4)) = 13956541632295291643_u64 | 2015345718253027281_u64;
_30 = _15 as f32;
_6 = _4;
(*_69) = core::ptr::addr_of!(_28);
(*_21) = Field::<f32>(Variant(_51, 0), 2) + _54;
_80 = (_6,);
_83.0.fld1 = Move(_27.fld1);
(*_61) = !_62;
(*_61) = _56 as i8;
_73.2.fld0 = Adt23::Variant0 { fld0: _1,fld1: Field::<i128>(Variant(_16, 1), 1),fld2: _51,fld3: Field::<u64>(Variant(_51, 0), 4),fld4: _30 };
_20.fld4 = Field::<usize>(Variant(_51, 0), 1) - Field::<usize>(Variant(Field::<Adt18>(Variant(_73.2.fld0, 0), 2), 0), 1);
(*_21) = _54 + Field::<f32>(Variant(_73.2.fld0, 0), 4);
_52 = Field::<i8>(Variant(_79, 1), 0) as u128;
(*_29) = [_20.fld4,_20.fld4,_20.fld4];
Goto(bb58)
}
bb58 = {
(*_29) = [Field::<usize>(Variant(_51, 0), 1),_20.fld4,Field::<usize>(Variant(_51, 0), 1)];
place!(Field::<u64>(Variant(_73.2.fld0, 0), 3)) = Field::<i8>(Variant(_79, 1), 0) as u64;
(*_69) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_51, 0), 0)));
_14 = _9 << _52;
_35.0 = [_22,_22,_22,_56];
(*_21) = Field::<f32>(Variant(_73.2.fld0, 0), 4) - Field::<f32>(Variant(Field::<Adt18>(Variant(_73.2.fld0, 0), 2), 0), 2);
_3 = _4 | _6;
_73.3 = core::ptr::addr_of!(_20.fld3);
(*_26) = core::ptr::addr_of_mut!((*_29));
_27.fld1 = Move(_35.1.fld1);
(*_61) = Field::<i8>(Variant(_79, 1), 0);
_20.fld7 = [_15,_15,_15];
Goto(bb59)
}
bb59 = {
_28 = !Field::<bool>(Variant(_73.2.fld0, 0), 0);
place!(Field::<i128>(Variant(_16, 1), 1)) = Field::<i128>(Variant(_79, 1), 1);
(*_26) = core::ptr::addr_of_mut!((*_29));
_79 = Move(_16);
(*_26) = core::ptr::addr_of_mut!((*_29));
_36 = [_44,_40,_44,_19,_40,_44];
_4 = _60.0 << _14;
_35.0 = [_56,_22,_56,_22];
(*_61) = Field::<i8>(Variant(_79, 1), 0);
_20.fld1 = _2;
(*_69) = core::ptr::addr_of!(_28);
place!(Field::<f32>(Variant(place!(Field::<Adt18>(Variant(_73.2.fld0, 0), 2)), 0), 2)) = _52 as f32;
Goto(bb60)
}
bb60 = {
_67 = [_59,Field::<i128>(Variant(_73.2.fld0, 0), 1),Field::<i128>(Variant(_79, 1), 1),_13];
_39 = _55;
_23 = _20.fld4 as isize;
(*_69) = core::ptr::addr_of!(_65);
_58 = Move(_73.2.fld0);
Goto(bb61)
}
bb61 = {
(*_61) = -Field::<i8>(Variant(_79, 1), 0);
(*_21) = _54 - Field::<f32>(Variant(_58, 0), 4);
_66 = [Field::<i128>(Variant(_79, 1), 1),_13,Field::<i128>(Variant(_79, 1), 1)];
place!(Field::<i128>(Variant(_58, 0), 1)) = _15 as i128;
RET.0.fld0.fld1 = _35.1.fld0.fld1 & _27.fld0.fld1;
(*_21) = -_54;
_96 = (Field::<bool>(Variant(_51, 0), 0), Field::<u64>(Variant(_58, 0), 3), (*_61), _20.fld5);
(*_69) = core::ptr::addr_of!(_57);
(*_26) = core::ptr::addr_of_mut!((*_29));
_46 = [_4,_4,_4,_4,_6,_4];
(*_26) = core::ptr::addr_of_mut!((*_29));
(*_61) = Field::<f32>(Variant(Field::<Adt18>(Variant(_58, 0), 2), 0), 2) as i8;
_12 = _39;
(*_61) = _96.2 >> _4;
(*_26) = core::ptr::addr_of_mut!((*_29));
_92.fld7 = [_15,_15,_15];
(*_69) = core::ptr::addr_of!(_57);
_39 = (*_61) as isize;
_72 = _39 << (*_61);
(*_61) = _96.3 as i8;
(*_61) = !_31;
_85 = -_54;
Goto(bb62)
}
bb62 = {
_17 = [_56,_22,_56];
_72 = _39 ^ _39;
_46 = [_4,_6,_4,_4,_3,_3];
_84 = !_96.3;
_82 = !_28;
(*_61) = Field::<usize>(Variant(_51, 0), 1) as i8;
_74 = core::ptr::addr_of!((*_61));
_92.fld6 = _9 - _9;
_73.1 = [Field::<i128>(Variant(_58, 0), 1),Field::<i128>(Variant(_58, 0), 1),Field::<i128>(Variant(_58, 0), 1),Field::<i128>(Variant(_58, 0), 1)];
Goto(bb63)
}
bb63 = {
_82 = _65;
_105 = Field::<usize>(Variant(_51, 0), 1) as i32;
_104 = _20.fld1 as i64;
_57 = !_65;
(*_74) = Field::<i8>(Variant(_79, 1), 0) << _15;
_96.3 = _20.fld5 & _52;
_27.fld0.fld0 = Move(_58);
RET = (Move(_27), Move((*_69)));
place!(Field::<i128>(Variant(_79, 1), 1)) = Field::<i128>(Variant(RET.0.fld0.fld0, 0), 1) << (*_74);
(*_61) = _96.2 >> Field::<i8>(Variant(_79, 1), 0);
Goto(bb64)
}
bb64 = {
Call(_107 = dump_var(6_usize, 67_usize, Move(_67), 60_usize, Move(_60), 4_usize, Move(_4), 56_usize, Move(_56)), ReturnTo(bb65), UnwindUnreachable())
}
bb65 = {
Call(_107 = dump_var(6_usize, 80_usize, Move(_80), 9_usize, Move(_9), 96_usize, Move(_96), 82_usize, Move(_82)), ReturnTo(bb66), UnwindUnreachable())
}
bb66 = {
Call(_107 = dump_var(6_usize, 62_usize, Move(_62), 14_usize, Move(_14), 72_usize, Move(_72), 17_usize, Move(_17)), ReturnTo(bb67), UnwindUnreachable())
}
bb67 = {
Call(_107 = dump_var(6_usize, 40_usize, Move(_40), 19_usize, Move(_19), 84_usize, Move(_84), 23_usize, Move(_23)), ReturnTo(bb68), UnwindUnreachable())
}
bb68 = {
Call(_107 = dump_var(6_usize, 55_usize, Move(_55), 105_usize, Move(_105), 65_usize, Move(_65), 46_usize, Move(_46)), ReturnTo(bb69), UnwindUnreachable())
}
bb69 = {
Call(_107 = dump_var(6_usize, 59_usize, Move(_59), 44_usize, Move(_44), 108_usize, _108, 108_usize, _108), ReturnTo(bb70), UnwindUnreachable())
}
bb70 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7() -> i64 {
mir! {
type RET = i64;
let _1: f64;
let _2: isize;
let _3: u64;
let _4: u128;
let _5: i8;
let _6: f64;
let _7: *mut bool;
let _8: u8;
let _9: isize;
let _10: &'static i32;
let _11: char;
let _12: *mut *mut [usize; 3];
let _13: f32;
let _14: i8;
let _15: *const i8;
let _16: *mut *mut [usize; 3];
let _17: *const bool;
let _18: u64;
let _19: ([char; 4], Adt57);
let _20: usize;
let _21: i32;
let _22: f32;
let _23: &'static mut *mut u16;
let _24: isize;
let _25: &'static Adt57;
let _26: Adt18;
let _27: u32;
let _28: f64;
let _29: *mut *const bool;
let _30: bool;
let _31: *mut i64;
let _32: ([char; 4], Adt57);
let _33: u64;
let _34: f64;
let _35: (Adt23, u16, [i128; 4], &'static mut isize);
let _36: (i32,);
let _37: i32;
let _38: &'static mut i32;
let _39: u64;
let _40: f32;
let _41: *const *mut u16;
let _42: [i128; 4];
let _43: &'static (bool, u64, i8, u128);
let _44: isize;
let _45: i32;
let _46: i64;
let _47: [u32; 6];
let _48: Adt18;
let _49: f64;
let _50: ([char; 4], Adt57);
let _51: (i32,);
let _52: *mut [usize; 3];
let _53: [char; 3];
let _54: &'static *const Adt18;
let _55: isize;
let _56: isize;
let _57: &'static [char; 3];
let _58: f32;
let _59: char;
let _60: f64;
let _61: bool;
let _62: isize;
let _63: u16;
let _64: isize;
let _65: f64;
let _66: &'static i32;
let _67: &'static mut isize;
let _68: bool;
let _69: f64;
let _70: (Adt23, u16, [i128; 4], &'static mut isize);
let _71: i8;
let _72: char;
let _73: *mut isize;
let _74: (i32,);
let _75: u16;
let _76: [char; 4];
let _77: &'static mut Adt57;
let _78: char;
let _79: *mut Adt31;
let _80: f64;
let _81: i64;
let _82: *mut [i128; 4];
let _83: *const Adt18;
let _84: u128;
let _85: *const *mut [usize; 3];
let _86: *const &'static mut i128;
let _87: Adt27;
let _88: f32;
let _89: isize;
let _90: *const *mut u16;
let _91: f64;
let _92: u16;
let _93: [char; 4];
let _94: *const i8;
let _95: bool;
let _96: bool;
let _97: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _98: &'static mut i128;
let _99: [u128; 4];
let _100: i16;
let _101: f64;
let _102: bool;
let _103: i128;
let _104: &'static Adt38;
let _105: [usize; 3];
let _106: isize;
let _107: [char; 4];
let _108: f32;
let _109: isize;
let _110: u16;
let _111: isize;
let _112: [u32; 6];
let _113: [char; 4];
let _114: bool;
let _115: usize;
let _116: *const *mut u16;
let _117: isize;
let _118: Adt23;
let _119: char;
let _120: i16;
let _121: i32;
let _122: &'static Adt57;
let _123: isize;
let _124: isize;
let _125: [u32; 6];
let _126: ();
let _127: ();
{
RET = (-7426059012427786970_i64);
RET = (-3605217056940534692_i64) ^ (-3808450596056261923_i64);
RET = 7658196840652880098_i64;
RET = 7880949572315898445_i64 ^ (-3834735751439025127_i64);
_1 = 1243216785396280979_usize as f64;
_1 = 32023_i16 as f64;
_1 = 3809277899_u32 as f64;
_2 = !9223372036854775807_isize;
_2 = 32156_u16 as isize;
RET = 111061822941645509240857588651532043905_u128 as i64;
RET = (-1651642982507049349_i64);
_4 = 301491566588042548490028309337799405664_u128;
_5 = 31_i8;
_1 = (-3095944089888845035238515372042041859_i128) as f64;
_5 = (-45_i8);
_6 = 8767474292985553198_u64 as f64;
_1 = 1935968087_i32 as f64;
Goto(bb1)
}
bb1 = {
_1 = _6;
_4 = !136045227287221785117560949015875882572_u128;
_5 = (-31_i8) >> _2;
_5 = 62635_u16 as i8;
_5 = (-109_i8);
_1 = _6 * _6;
_6 = _1 + _1;
_3 = 15609076982248200631_u64;
_6 = _1 * _1;
RET = (-3720959169750546096_i64) & (-5747951884597019731_i64);
_3 = _5 as u64;
RET = !(-5874742006805213230_i64);
RET = 7425789531259399405_i64 - (-5480959118204477478_i64);
_5 = (-108_i8) * 115_i8;
_3 = 1897038078818130665_u64;
_6 = -_1;
_6 = _5 as f64;
RET = 6499447205391328060_i64 >> _2;
_8 = 1_u8;
_4 = 293529937441870885481306549254734322280_u128 * 3273743530383228987180315560212517404_u128;
_2 = -9223372036854775807_isize;
RET = (-3449544336292494744_i64);
_5 = 67_i8 + (-48_i8);
_1 = _6;
Call(_4 = fn8(RET, _6, _2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _6;
_4 = 221171782703495244331349096445851629179_u128 | 34159885963472677989538762428461298089_u128;
_4 = !141287321770499982793215307222146339413_u128;
_11 = '\u{2a9f4}';
_3 = 8310598525659420259_u64;
RET = (-6385882503971064747_i64);
_6 = -_1;
_11 = '\u{6949a}';
_9 = (-20408_i16) as isize;
RET = !(-997364224391615997_i64);
_9 = _2 ^ _2;
_3 = _5 as u64;
_4 = 130477556347512434162643614426736578750_u128;
_6 = _1 * _1;
_1 = _6 - _6;
_13 = _8 as f32;
_6 = _1 * _1;
_3 = 1610180869124776900_u64 << _4;
_13 = _3 as f32;
RET = !4669057538577900686_i64;
_3 = !10813296836230094274_u64;
match _8 {
1 => bb4,
_ => bb3
}
}
bb3 = {
_1 = _6;
_4 = !136045227287221785117560949015875882572_u128;
_5 = (-31_i8) >> _2;
_5 = 62635_u16 as i8;
_5 = (-109_i8);
_1 = _6 * _6;
_6 = _1 + _1;
_3 = 15609076982248200631_u64;
_6 = _1 * _1;
RET = (-3720959169750546096_i64) & (-5747951884597019731_i64);
_3 = _5 as u64;
RET = !(-5874742006805213230_i64);
RET = 7425789531259399405_i64 - (-5480959118204477478_i64);
_5 = (-108_i8) * 115_i8;
_3 = 1897038078818130665_u64;
_6 = -_1;
_6 = _5 as f64;
RET = 6499447205391328060_i64 >> _2;
_8 = 1_u8;
_4 = 293529937441870885481306549254734322280_u128 * 3273743530383228987180315560212517404_u128;
_2 = -9223372036854775807_isize;
RET = (-3449544336292494744_i64);
_5 = 67_i8 + (-48_i8);
_1 = _6;
Call(_4 = fn8(RET, _6, _2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_4 = !203250431762347136580514527198346612702_u128;
_14 = 28251_i16 as i8;
_1 = _6 * _6;
_1 = _3 as f64;
_15 = core::ptr::addr_of!(_14);
(*_15) = -_5;
_15 = core::ptr::addr_of!((*_15));
(*_15) = -_5;
(*_15) = false as i8;
_2 = _9 ^ _9;
(*_15) = _3 as i8;
(*_15) = !_5;
(*_15) = _5;
(*_15) = _5 | _5;
(*_15) = 32001_i16 as i8;
_5 = -(*_15);
(*_15) = _5;
_4 = _2 as u128;
Goto(bb5)
}
bb5 = {
(*_15) = _5 & _5;
_6 = _1 - _1;
(*_15) = _5 << _4;
_4 = _1 as u128;
(*_15) = !_5;
(*_15) = _3 as i8;
_6 = 5401364345937766620_usize as f64;
(*_15) = !_5;
RET = (-2885207941624962229_i64) - (-8828009332440374305_i64);
(*_15) = _5 - _5;
Goto(bb6)
}
bb6 = {
_2 = -_9;
(*_15) = _5 >> _9;
(*_15) = (-7051594163346235689355909263822131991_i128) as i8;
(*_15) = _5 >> _9;
_2 = _9 ^ _9;
(*_15) = _5;
RET = (-2369623199636587843_i64) & (-8720629792187434874_i64);
_2 = !_9;
(*_15) = _5;
match _8 {
0 => bb2,
1 => bb8,
_ => bb7
}
}
bb7 = {
_1 = _6;
_4 = 221171782703495244331349096445851629179_u128 | 34159885963472677989538762428461298089_u128;
_4 = !141287321770499982793215307222146339413_u128;
_11 = '\u{2a9f4}';
_3 = 8310598525659420259_u64;
RET = (-6385882503971064747_i64);
_6 = -_1;
_11 = '\u{6949a}';
_9 = (-20408_i16) as isize;
RET = !(-997364224391615997_i64);
_9 = _2 ^ _2;
_3 = _5 as u64;
_4 = 130477556347512434162643614426736578750_u128;
_6 = _1 * _1;
_1 = _6 - _6;
_13 = _8 as f32;
_6 = _1 * _1;
_3 = 1610180869124776900_u64 << _4;
_13 = _3 as f32;
RET = !4669057538577900686_i64;
_3 = !10813296836230094274_u64;
match _8 {
1 => bb4,
_ => bb3
}
}
bb8 = {
_19.1.fld0.fld1 = _8 - _8;
(*_15) = RET as i8;
_3 = 3275858120780817271_u64 << (*_15);
_8 = _19.1.fld0.fld1 << (*_15);
_20 = 7_usize + 12576034105249381443_usize;
_5 = RET as i8;
Goto(bb9)
}
bb9 = {
(*_15) = _5 - _5;
(*_15) = _5 + _5;
_3 = 7298319542841274160_u64 << (*_15);
_19.0 = [_11,_11,_11,_11];
(*_15) = true as i8;
_8 = _19.1.fld0.fld1 + _19.1.fld0.fld1;
(*_15) = (-7912_i16) as i8;
(*_15) = _5;
(*_15) = !_5;
_1 = _6;
(*_15) = -_5;
_4 = _6 as u128;
(*_15) = _5 ^ _5;
_9 = _3 as isize;
_14 = (-58310934_i32) as i8;
_20 = !1_usize;
(*_15) = _5;
_18 = (-20906_i16) as u64;
RET = _3 as i64;
_15 = core::ptr::addr_of!((*_15));
_1 = _6;
_4 = _20 as u128;
(*_15) = _3 as i8;
Goto(bb10)
}
bb10 = {
_19.1.fld2 = [_4,_4,_4,_4];
_19.1.fld2 = [_4,_4,_4,_4];
(*_15) = _2 as i8;
_21 = 1748257336_i32 - (-1843586411_i32);
(*_15) = _5;
(*_15) = -_5;
(*_15) = _5 - _5;
(*_15) = !_5;
_18 = !_3;
_10 = &_21;
_1 = _13 as f64;
_14 = _5 & _5;
(*_15) = -_5;
_15 = core::ptr::addr_of!((*_15));
Goto(bb11)
}
bb11 = {
_21 = _2 as i32;
Goto(bb12)
}
bb12 = {
(*_15) = -_5;
(*_15) = _5;
_19.0 = [_11,_11,_11,_11];
(*_15) = _5 | _5;
_15 = core::ptr::addr_of!((*_15));
_9 = _8 as isize;
_9 = _20 as isize;
_11 = '\u{532b1}';
_15 = core::ptr::addr_of!((*_15));
(*_15) = _5 | _5;
_5 = _9 as i8;
(*_15) = -_5;
(*_15) = _5 << RET;
RET = -(-5069602917862796821_i64);
(*_15) = _13 as i8;
_5 = (*_15);
_24 = _2 * _9;
_5 = (*_15) | (*_15);
_19.1.fld0.fld1 = _8 - _8;
_22 = -_13;
(*_15) = _5 << _3;
(*_15) = _5;
RET = 16982_u16 as i64;
(*_15) = _5 * _5;
_26 = Adt18::Variant0 { fld0: true,fld1: _20,fld2: _13,fld3: RET,fld4: _3 };
_11 = '\u{a124}';
_15 = core::ptr::addr_of!((*_15));
Goto(bb13)
}
bb13 = {
_6 = -_1;
(*_15) = !_5;
(*_15) = _5 * _5;
_17 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_26, 0), 0)));
_14 = _22 as i8;
_18 = Field::<u64>(Variant(_26, 0), 4);
_5 = (*_15) | (*_15);
_27 = 178486212_u32 * 3634419357_u32;
(*_17) = (*_15) > (*_15);
(*_17) = true ^ false;
(*_15) = _5 - _5;
Call((*_15) = core::intrinsics::transmute(Field::<bool>(Variant(_26, 0), 0)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_17) = !false;
RET = !Field::<i64>(Variant(_26, 0), 3);
(*_17) = false & false;
_24 = _9 << (*_15);
(*_15) = !_5;
_24 = _9 * _9;
_18 = Field::<u64>(Variant(_26, 0), 4) + _3;
(*_15) = _5 + _5;
(*_17) = true;
(*_15) = -_5;
(*_15) = _21 as i8;
_22 = _13;
_32.1.fld2 = _19.1.fld2;
(*_17) = true | false;
_29 = core::ptr::addr_of_mut!(_17);
_32.1.fld0.fld1 = _19.1.fld0.fld1 + _19.1.fld0.fld1;
(*_17) = _14 <= (*_15);
(*_17) = _3 < _18;
(*_17) = false & true;
(*_15) = !_5;
Goto(bb15)
}
bb15 = {
_19.1.fld0.fld1 = !_32.1.fld0.fld1;
(*_17) = !true;
(*_29) = core::ptr::addr_of!((*_17));
_32.1.fld0.fld1 = _19.1.fld0.fld1 & _19.1.fld0.fld1;
(*_15) = _5;
_11 = '\u{32ccf}';
(*_17) = false ^ false;
place!(Field::<usize>(Variant(_26, 0), 1)) = !_20;
(*_29) = core::ptr::addr_of!(_30);
_7 = core::ptr::addr_of_mut!((*_17));
(*_7) = _14 > (*_15);
(*_7) = _18 < _18;
Goto(bb16)
}
bb16 = {
(*_15) = (-90661589731904729772589637354893021515_i128) as i8;
_19.0 = [_11,_11,_11,_11];
(*_7) = Field::<bool>(Variant(_26, 0), 0);
(*_7) = _32.1.fld0.fld1 != _32.1.fld0.fld1;
_8 = _32.1.fld0.fld1 + _19.1.fld0.fld1;
_32.1.fld1 = core::ptr::addr_of_mut!((*_7));
_24 = _9 + _9;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_26, 0), 0)));
(*_29) = core::ptr::addr_of!((*_17));
_28 = -_1;
_11 = '\u{47b8a}';
(*_29) = core::ptr::addr_of!((*_17));
(*_17) = !(*_7);
Goto(bb17)
}
bb17 = {
(*_15) = _5;
(*_29) = core::ptr::addr_of!((*_7));
(*_7) = !Field::<bool>(Variant(_26, 0), 0);
_32.1.fld1 = Move(_7);
_24 = _2;
_8 = _32.1.fld0.fld1;
RET = Field::<i64>(Variant(_26, 0), 3);
_19.1.fld2 = [_4,_4,_4,_4];
_32.1.fld0.fld0 = Adt23::Variant0 { fld0: Field::<bool>(Variant(_26, 0), 0),fld1: 11946636670072374725269157826870034048_i128,fld2: _26,fld3: Field::<u64>(Variant(_26, 0), 4),fld4: Field::<f32>(Variant(_26, 0), 2) };
_17 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_26, 0), 0)));
(*_29) = core::ptr::addr_of!((*_17));
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_26, 0), 0)));
_35.3 = &mut _2;
_4 = 91470370912206153186001269564174752179_u128;
(*_29) = core::ptr::addr_of!((*_17));
_14 = _5 + _5;
place!(Field::<usize>(Variant(place!(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2)), 0), 1)) = Field::<usize>(Variant(_26, 0), 1);
(*_15) = _5 + _5;
Goto(bb18)
}
bb18 = {
_14 = _5 - _5;
_30 = (*_17);
(*_17) = (*_15) < (*_15);
_32.1.fld0.fld1 = _8 >> _8;
_37 = (*_17) as i32;
place!(Field::<bool>(Variant(place!(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2)), 0), 0)) = !(*_17);
(*_29) = core::ptr::addr_of!((*_17));
Goto(bb19)
}
bb19 = {
_38 = &mut _21;
_36 = ((*_38),);
(*_29) = core::ptr::addr_of!((*_17));
(*_17) = _30 & _30;
_35.2 = [(-107540864614189921098112284457958662_i128),8678406183320261123754180701359481746_i128,62996970742355042555198095839656957587_i128,(-35642264094666181729722679848454087392_i128)];
place!(Field::<f32>(Variant(_32.1.fld0.fld0, 0), 4)) = Field::<usize>(Variant(_26, 0), 1) as f32;
(*_38) = _18 as i32;
(*_38) = -_37;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_32.1.fld0.fld0, 0), 0)));
(*_38) = _27 as i32;
place!(Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1)) = 88010182093246201301639561168516579151_i128;
_32.1.fld2 = [_4,_4,_4,_4];
_35.1 = 6245_u16;
(*_17) = (*_15) == (*_15);
_45 = Field::<usize>(Variant(_26, 0), 1) as i32;
(*_29) = core::ptr::addr_of!((*_17));
(*_38) = _37 * _37;
(*_17) = (*_15) == (*_15);
Goto(bb20)
}
bb20 = {
_26 = Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2);
place!(Field::<f32>(Variant(_26, 0), 2)) = Field::<f32>(Variant(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2), 0), 2) - _13;
(*_15) = _5 & _5;
(*_38) = -_37;
_36.0 = (*_38) - (*_38);
_50.1.fld2 = _19.1.fld2;
_47 = [_27,_27,_27,_27,_27,_27];
_51.0 = !(*_38);
_32.1.fld0.fld1 = _19.1.fld0.fld1 * _8;
place!(Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1)) = (-140674948937854698593640201582796787954_i128);
_50.1.fld1 = Move(_32.1.fld1);
Goto(bb21)
}
bb21 = {
(*_17) = (*_15) <= _14;
(*_29) = core::ptr::addr_of!((*_17));
place!(Field::<f32>(Variant(_26, 0), 2)) = _27 as f32;
(*_38) = -_51.0;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_32.1.fld0.fld0, 0), 0)));
_15 = core::ptr::addr_of!(_14);
match _35.1 {
0 => bb1,
1 => bb15,
2 => bb3,
3 => bb13,
6245 => bb22,
_ => bb18
}
}
bb22 = {
_19.1.fld1 = Move(_50.1.fld1);
place!(Field::<u64>(Variant(place!(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2)), 0), 4)) = Field::<u64>(Variant(_26, 0), 4);
_32.1.fld2 = _19.1.fld2;
_32.1.fld0.fld1 = !_8;
_50.0 = [_11,_11,_11,_11];
_31 = core::ptr::addr_of_mut!(place!(Field::<i64>(Variant(_26, 0), 3)));
(*_17) = !_30;
(*_15) = _5 * _5;
_30 = Field::<bool>(Variant(_32.1.fld0.fld0, 0), 0);
_55 = 24358_i16 as isize;
(*_29) = core::ptr::addr_of!((*_17));
(*_31) = -RET;
_35.2 = [Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1),Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1),Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1),Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1)];
_35.1 = 21277_u16 * 44147_u16;
_15 = core::ptr::addr_of!((*_15));
(*_15) = _5;
_12 = core::ptr::addr_of_mut!(_52);
(*_17) = _30;
_40 = _4 as f32;
place!(Field::<bool>(Variant(_32.1.fld0.fld0, 0), 0)) = _30;
Goto(bb23)
}
bb23 = {
(*_29) = core::ptr::addr_of!((*_17));
(*_17) = !_30;
(*_15) = _5 << _32.1.fld0.fld1;
(*_31) = Field::<i64>(Variant(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2), 0), 3) ^ RET;
(*_17) = Field::<bool>(Variant(_26, 0), 0) | _30;
(*_38) = _51.0 * _36.0;
_19.0 = [_11,_11,_11,_11];
_29 = core::ptr::addr_of_mut!((*_29));
(*_38) = -_36.0;
_56 = _19.1.fld0.fld1 as isize;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_26, 0), 0)));
(*_15) = _5;
(*_17) = Field::<bool>(Variant(_32.1.fld0.fld0, 0), 0) & Field::<bool>(Variant(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2), 0), 0);
_27 = 2650834649_u32;
(*_29) = core::ptr::addr_of!((*_17));
_4 = _27 as u128;
(*_38) = _37;
_36.0 = (*_38) | (*_38);
_62 = _56 ^ _9;
Goto(bb24)
}
bb24 = {
(*_29) = core::ptr::addr_of!((*_17));
_50.1.fld2 = [_4,_4,_4,_4];
_19.1.fld2 = [_4,_4,_4,_4];
_35.0 = Move(_32.1.fld0.fld0);
(*_17) = Field::<bool>(Variant(Field::<Adt18>(Variant(_35.0, 0), 2), 0), 0);
(*_38) = !_36.0;
(*_31) = -RET;
_50.1.fld0 = Adt44 { fld0: Move(_35.0),fld1: _19.1.fld0.fld1 };
_65 = _4 as f64;
(*_31) = RET * Field::<i64>(Variant(Field::<Adt18>(Variant(_50.1.fld0.fld0, 0), 2), 0), 3);
_48 = _26;
(*_38) = _36.0;
place!(Field::<u64>(Variant(place!(Field::<Adt18>(Variant(_50.1.fld0.fld0, 0), 2)), 0), 4)) = !_18;
(*_15) = _5 ^ _5;
(*_29) = core::ptr::addr_of!((*_17));
_39 = Field::<u64>(Variant(_48, 0), 4) + _18;
_7 = core::ptr::addr_of_mut!((*_17));
(*_7) = _50.1.fld0.fld1 > _32.1.fld0.fld1;
(*_7) = (*_38) < (*_38);
_32.1.fld0.fld0 = Adt23::Variant0 { fld0: (*_7),fld1: Field::<i128>(Variant(_50.1.fld0.fld0, 0), 1),fld2: _26,fld3: _3,fld4: _13 };
_44 = _24;
(*_17) = !_30;
(*_29) = core::ptr::addr_of!((*_17));
_53 = [_11,_11,_11];
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(place!(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2)), 0), 0)));
place!(Field::<bool>(Variant(_48, 0), 0)) = (*_17);
Call(_4 = core::intrinsics::bswap(69690979589310109719097178520841069885_u128), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_71 = (*_31) as i8;
(*_29) = core::ptr::addr_of!((*_17));
_19.1.fld0.fld0 = Move(_50.1.fld0.fld0);
_19.1.fld0 = Adt44 { fld0: Move(_32.1.fld0.fld0),fld1: _50.1.fld0.fld1 };
_68 = (*_15) < (*_15);
_35.3 = &mut _24;
_34 = _1;
_58 = Field::<f32>(Variant(_48, 0), 2) - Field::<f32>(Variant(_48, 0), 2);
(*_17) = Field::<u64>(Variant(_19.1.fld0.fld0, 0), 3) > Field::<u64>(Variant(_26, 0), 4);
_76 = [_11,_11,_11,_11];
(*_38) = _36.0;
_57 = &_53;
_15 = core::ptr::addr_of!((*_15));
_22 = (*_15) as f32;
_8 = _19.1.fld0.fld1 + _50.1.fld0.fld1;
_51 = (_36.0,);
_32 = Move(_19);
Goto(bb26)
}
bb26 = {
_42 = [Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1),Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1),Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1),Field::<i128>(Variant(_32.1.fld0.fld0, 0), 1)];
(*_38) = _51.0 << (*_31);
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_32.1.fld0.fld0, 0), 0)));
Goto(bb27)
}
bb27 = {
_76 = [_11,_11,_11,_11];
_40 = Field::<f32>(Variant(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2), 0), 2);
_38 = &mut _37;
_25 = &_32.1;
(*_31) = _34 as i64;
(*_38) = Field::<i128>(Variant((*_25).fld0.fld0, 0), 1) as i32;
(*_29) = core::ptr::addr_of!((*_17));
_64 = _27 as isize;
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2)), 0), 3)) = (*_31);
(*_15) = _5 | _71;
_7 = core::ptr::addr_of_mut!((*_17));
(*_31) = Field::<i64>(Variant(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2), 0), 3) - Field::<i64>(Variant(_48, 0), 3);
_78 = _11;
place!(Field::<u64>(Variant(place!(Field::<Adt18>(Variant(_32.1.fld0.fld0, 0), 2)), 0), 4)) = _35.1 as u64;
(*_31) = Field::<i64>(Variant(_48, 0), 3) & Field::<i64>(Variant(_48, 0), 3);
(*_29) = core::ptr::addr_of!(_68);
_85 = core::ptr::addr_of!((*_12));
_19 = Move(_32);
_75 = _35.1;
_25 = &_19.1;
_56 = _62 >> (*_25).fld0.fld1;
_34 = _1;
_63 = _75;
Goto(bb28)
}
bb28 = {
_46 = !(*_31);
_38 = &mut _36.0;
_35.0 = Move(_19.1.fld0.fld0);
(*_17) = Field::<bool>(Variant(Field::<Adt18>(Variant(_35.0, 0), 2), 0), 0) == Field::<bool>(Variant(_48, 0), 0);
_78 = _11;
_35.1 = _63 + _63;
Goto(bb29)
}
bb29 = {
(*_29) = core::ptr::addr_of!((*_17));
_76 = [_11,_11,_11,_78];
_50.1.fld1 = core::ptr::addr_of_mut!((*_17));
Goto(bb30)
}
bb30 = {
_74.0 = -(*_38);
_47 = [_27,_27,_27,_27,_27,_27];
(*_29) = core::ptr::addr_of!(_30);
(*_29) = core::ptr::addr_of!((*_17));
_49 = _28 - _34;
(*_15) = _5 * _5;
(*_31) = _75 as i64;
_66 = &(*_38);
_51 = (_74.0,);
_51 = ((*_38),);
(*_38) = _51.0;
place!(Field::<bool>(Variant(_48, 0), 0)) = (*_17);
_1 = _49 - _28;
(*_29) = core::ptr::addr_of!((*_17));
_13 = _58 * _40;
_32.1.fld2 = _19.1.fld2;
_53 = [_11,_78,_11];
(*_15) = _5 & _5;
_84 = _58 as u128;
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_35.0, 0), 2)), 0), 3)) = (*_31) * (*_31);
_32.1.fld1 = core::ptr::addr_of_mut!((*_17));
(*_29) = core::ptr::addr_of!((*_17));
(*_29) = core::ptr::addr_of!((*_17));
_32.1.fld2 = (*_25).fld2;
_9 = _62 * _62;
Goto(bb31)
}
bb31 = {
(*_29) = core::ptr::addr_of!((*_17));
_59 = _11;
_70.3 = &mut _9;
_47 = [_27,_27,_27,_27,_27,_27];
(*_15) = Field::<usize>(Variant(_48, 0), 1) as i8;
_96 = (*_17) == Field::<bool>(Variant(_35.0, 0), 0);
(*_15) = _5 | _5;
(*_38) = _74.0 * _51.0;
_51.0 = (*_38) ^ (*_38);
_47 = [_27,_27,_27,_27,_27,_27];
(*_15) = _5 << (*_38);
place!(Field::<Adt18>(Variant(_35.0, 0), 2)) = _48;
_20 = Field::<usize>(Variant(_48, 0), 1);
_32.1.fld0 = Adt44 { fld0: Move(_35.0),fld1: (*_25).fld0.fld1 };
_19.1 = Move(_32.1);
(*_29) = core::ptr::addr_of!(_61);
(*_17) = Field::<bool>(Variant(_19.1.fld0.fld0, 0), 0) != _68;
_7 = core::ptr::addr_of_mut!((*_17));
_50.0 = _19.0;
_76 = [_59,_59,_59,_78];
Call(_74.0 = core::intrinsics::transmute((*_38)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
(*_15) = !_5;
_48 = Adt18::Variant0 { fld0: (*_17),fld1: Field::<usize>(Variant(Field::<Adt18>(Variant(_19.1.fld0.fld0, 0), 2), 0), 1),fld2: _13,fld3: (*_31),fld4: _3 };
(*_7) = (*_38) > _51.0;
_96 = (*_7) == (*_17);
_97.2.fld0 = Move(_19.1.fld0.fld0);
(*_15) = _5 << (*_38);
(*_17) = _68 ^ _68;
_26 = Field::<Adt18>(Variant(_97.2.fld0, 0), 2);
_50.1.fld0.fld1 = _8 * _8;
_64 = _44 * _62;
_35 = (Move(_97.2.fld0), _63, _42, Move(_70.3));
_19.1.fld0.fld1 = !_8;
(*_29) = core::ptr::addr_of!((*_17));
_26 = _48;
_17 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_26, 0), 0)));
Goto(bb33)
}
bb33 = {
(*_38) = _51.0 - _74.0;
(*_15) = -_5;
match _27 {
0 => bb34,
2650834649 => bb36,
_ => bb35
}
}
bb34 = {
(*_15) = (-90661589731904729772589637354893021515_i128) as i8;
_19.0 = [_11,_11,_11,_11];
(*_7) = Field::<bool>(Variant(_26, 0), 0);
(*_7) = _32.1.fld0.fld1 != _32.1.fld0.fld1;
_8 = _32.1.fld0.fld1 + _19.1.fld0.fld1;
_32.1.fld1 = core::ptr::addr_of_mut!((*_7));
_24 = _9 + _9;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_26, 0), 0)));
(*_29) = core::ptr::addr_of!((*_17));
_28 = -_1;
_11 = '\u{47b8a}';
(*_29) = core::ptr::addr_of!((*_17));
(*_17) = !(*_7);
Goto(bb17)
}
bb35 = {
(*_15) = _5 & _5;
_6 = _1 - _1;
(*_15) = _5 << _4;
_4 = _1 as u128;
(*_15) = !_5;
(*_15) = _3 as i8;
_6 = 5401364345937766620_usize as f64;
(*_15) = !_5;
RET = (-2885207941624962229_i64) - (-8828009332440374305_i64);
(*_15) = _5 - _5;
Goto(bb6)
}
bb36 = {
(*_38) = _51.0 << _4;
_70.2 = [Field::<i128>(Variant(_35.0, 0), 1),Field::<i128>(Variant(_35.0, 0), 1),Field::<i128>(Variant(_35.0, 0), 1),Field::<i128>(Variant(_35.0, 0), 1)];
(*_38) = _51.0 - _51.0;
_39 = _18 << (*_38);
_20 = Field::<usize>(Variant(_26, 0), 1) & Field::<usize>(Variant(_48, 0), 1);
(*_17) = !Field::<bool>(Variant(_48, 0), 0);
_70.3 = &mut _44;
_63 = _35.1;
_70.0 = Adt23::Variant0 { fld0: (*_17),fld1: Field::<i128>(Variant(_35.0, 0), 1),fld2: _48,fld3: Field::<u64>(Variant(_48, 0), 4),fld4: _22 };
_94 = Move(_15);
(*_17) = !_96;
place!(Field::<f32>(Variant(place!(Field::<Adt18>(Variant(_70.0, 0), 2)), 0), 2)) = Field::<usize>(Variant(Field::<Adt18>(Variant(_70.0, 0), 2), 0), 1) as f32;
_83 = core::ptr::addr_of!(_26);
_8 = _19.1.fld0.fld1 * _50.1.fld0.fld1;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant((*_83), 0), 0)));
(*_83) = Field::<Adt18>(Variant(_35.0, 0), 2);
Goto(bb37)
}
bb37 = {
_19.1.fld2 = [_84,_84,_84,_84];
place!(Field::<i64>(Variant((*_83), 0), 3)) = _46 >> (*_38);
Goto(bb38)
}
bb38 = {
(*_83) = Adt18::Variant0 { fld0: _61,fld1: _20,fld2: Field::<f32>(Variant(_70.0, 0), 4),fld3: RET,fld4: _39 };
place!(Field::<u64>(Variant((*_83), 0), 4)) = _50.1.fld0.fld1 as u64;
_101 = _49 - _1;
_105 = [Field::<usize>(Variant((*_83), 0), 1),Field::<usize>(Variant(_26, 0), 1),Field::<usize>(Variant((*_83), 0), 1)];
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant((*_83), 0), 0)));
(*_83) = _48;
_108 = Field::<f32>(Variant((*_83), 0), 2) - Field::<f32>(Variant((*_83), 0), 2);
_35 = (Move(_70.0), _63, _70.2, Move(_70.3));
place!(Field::<u64>(Variant(_48, 0), 4)) = _39 - _39;
place!(Field::<u64>(Variant((*_83), 0), 4)) = _39 + Field::<u64>(Variant(_48, 0), 4);
(*_38) = _63 as i32;
_95 = !Field::<bool>(Variant((*_83), 0), 0);
_91 = _101 + _101;
_50.1.fld0.fld0 = Adt23::Variant1 { fld0: Move(_83),fld1: _11,fld2: _56,fld3: _5,fld4: _84,fld5: Field::<i128>(Variant(_35.0, 0), 1) };
(*_85) = core::ptr::addr_of_mut!(_105);
_102 = !Field::<bool>(Variant(_26, 0), 0);
(*_38) = _51.0 + _74.0;
(*_52) = [Field::<usize>(Variant(Field::<Adt18>(Variant(_35.0, 0), 2), 0), 1),Field::<usize>(Variant(Field::<Adt18>(Variant(_35.0, 0), 2), 0), 1),Field::<usize>(Variant(Field::<Adt18>(Variant(_35.0, 0), 2), 0), 1)];
(*_12) = core::ptr::addr_of_mut!((*_52));
_51 = ((*_38),);
(*_52) = [_20,Field::<usize>(Variant(_48, 0), 1),Field::<usize>(Variant(_26, 0), 1)];
_32.0 = [Field::<char>(Variant(_50.1.fld0.fld0, 1), 1),_78,_11,_11];
(*_38) = _74.0;
Call(_99 = core::intrinsics::transmute(_19.1.fld2), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
place!(Field::<char>(Variant(_50.1.fld0.fld0, 1), 1)) = _59;
(*_52) = [_20,Field::<usize>(Variant(_26, 0), 1),Field::<usize>(Variant(_48, 0), 1)];
Call(_103 = core::intrinsics::transmute(Field::<i128>(Variant(_35.0, 0), 1)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
(*_52) = [Field::<usize>(Variant(_48, 0), 1),_20,Field::<usize>(Variant(Field::<Adt18>(Variant(_35.0, 0), 2), 0), 1)];
(*_12) = core::ptr::addr_of_mut!((*_52));
_30 = _96 ^ Field::<bool>(Variant(_35.0, 0), 0);
_32.1 = Adt57 { fld0: Move(_50.1.fld0),fld1: Move(_7),fld2: _19.1.fld2 };
place!(Field::<i8>(Variant(_32.1.fld0.fld0, 1), 3)) = _14 << (*_38);
place!(Field::<u128>(Variant(_32.1.fld0.fld0, 1), 4)) = _84;
_70.3 = &mut _56;
_113 = [_78,Field::<char>(Variant(_32.1.fld0.fld0, 1), 1),_11,_78];
(*_12) = core::ptr::addr_of_mut!((*_52));
(*_52) = [Field::<usize>(Variant(Field::<Adt18>(Variant(_35.0, 0), 2), 0), 1),Field::<usize>(Variant(Field::<Adt18>(Variant(_35.0, 0), 2), 0), 1),Field::<usize>(Variant(_48, 0), 1)];
(*_12) = core::ptr::addr_of_mut!((*_52));
(*_29) = core::ptr::addr_of!(_61);
place!(Field::<u64>(Variant(_35.0, 0), 3)) = !_39;
_35 = (Move(_32.1.fld0.fld0), _63, _70.2, Move(_70.3));
_42 = [_103,_103,Field::<i128>(Variant(_35.0, 1), 5),_103];
_108 = _58;
(*_38) = Field::<i8>(Variant(_35.0, 1), 3) as i32;
_19.1.fld2 = [Field::<u128>(Variant(_35.0, 1), 4),_4,_84,_84];
(*_52) = [_20,_20,Field::<usize>(Variant(_26, 0), 1)];
_84 = Field::<u128>(Variant(_35.0, 1), 4) + Field::<u128>(Variant(_35.0, 1), 4);
(*_17) = _68;
_110 = _35.1;
(*_29) = core::ptr::addr_of!((*_17));
_81 = Field::<isize>(Variant(_35.0, 1), 2) as i64;
_19.1.fld0 = Adt44 { fld0: Move(_35.0),fld1: _8 };
Goto(bb41)
}
bb41 = {
(*_52) = [Field::<usize>(Variant(_26, 0), 1),_20,Field::<usize>(Variant(_26, 0), 1)];
_106 = Field::<isize>(Variant(_19.1.fld0.fld0, 1), 2);
_97.2 = Adt44 { fld0: Move(_19.1.fld0.fld0),fld1: _8 };
(*_38) = -_51.0;
place!(Field::<i128>(Variant(_97.2.fld0, 1), 5)) = _106 as i128;
_1 = _19.1.fld0.fld1 as f64;
_48 = Adt18::Variant0 { fld0: (*_17),fld1: _20,fld2: Field::<f32>(Variant(_26, 0), 2),fld3: Field::<i64>(Variant(_26, 0), 3),fld4: _39 };
_115 = Field::<usize>(Variant(_48, 0), 1) << _74.0;
_93 = [_59,_59,_11,Field::<char>(Variant(_97.2.fld0, 1), 1)];
_88 = _22 * _22;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_48, 0), 0)));
_19.1.fld0 = Adt44 { fld0: Move(_97.2.fld0),fld1: _97.2.fld1 };
_62 = -_64;
(*_29) = core::ptr::addr_of!((*_17));
_4 = _84 ^ Field::<u128>(Variant(_19.1.fld0.fld0, 1), 4);
Goto(bb42)
}
bb42 = {
_18 = _27 as u64;
(*_52) = [Field::<usize>(Variant(_48, 0), 1),_115,_115];
(*_52) = [_115,_115,_115];
_39 = Field::<u64>(Variant(_26, 0), 4) << (*_38);
(*_29) = core::ptr::addr_of!((*_17));
(*_38) = _51.0 * _51.0;
_103 = -Field::<i128>(Variant(_19.1.fld0.fld0, 1), 5);
(*_52) = [_115,_115,_115];
_50.1.fld1 = Move(_19.1.fld1);
_32.1.fld0 = Adt44 { fld0: Move(_19.1.fld0.fld0),fld1: _8 };
(*_17) = Field::<u64>(Variant(_48, 0), 4) < Field::<u64>(Variant(_26, 0), 4);
_50 = (_76, Move(_32.1));
(*_12) = core::ptr::addr_of_mut!((*_52));
(*_38) = _74.0 ^ _51.0;
_100 = (-23118_i16) & 11653_i16;
_42 = [Field::<i128>(Variant(_50.1.fld0.fld0, 1), 5),Field::<i128>(Variant(_50.1.fld0.fld0, 1), 5),_103,_103];
_68 = !(*_17);
(*_29) = core::ptr::addr_of!((*_17));
(*_29) = core::ptr::addr_of!(_102);
_19.1 = Adt57 { fld0: Move(_50.1.fld0),fld1: Move(_50.1.fld1),fld2: _99 };
match _27 {
0 => bb13,
1 => bb43,
2 => bb44,
3 => bb45,
4 => bb46,
5 => bb47,
2650834649 => bb49,
_ => bb48
}
}
bb43 = {
(*_38) = _51.0 - _74.0;
(*_15) = -_5;
match _27 {
0 => bb34,
2650834649 => bb36,
_ => bb35
}
}
bb44 = {
(*_17) = !false;
RET = !Field::<i64>(Variant(_26, 0), 3);
(*_17) = false & false;
_24 = _9 << (*_15);
(*_15) = !_5;
_24 = _9 * _9;
_18 = Field::<u64>(Variant(_26, 0), 4) + _3;
(*_15) = _5 + _5;
(*_17) = true;
(*_15) = -_5;
(*_15) = _21 as i8;
_22 = _13;
_32.1.fld2 = _19.1.fld2;
(*_17) = true | false;
_29 = core::ptr::addr_of_mut!(_17);
_32.1.fld0.fld1 = _19.1.fld0.fld1 + _19.1.fld0.fld1;
(*_17) = _14 <= (*_15);
(*_17) = _3 < _18;
(*_17) = false & true;
(*_15) = !_5;
Goto(bb15)
}
bb45 = {
(*_15) = -_5;
(*_15) = _5;
_19.0 = [_11,_11,_11,_11];
(*_15) = _5 | _5;
_15 = core::ptr::addr_of!((*_15));
_9 = _8 as isize;
_9 = _20 as isize;
_11 = '\u{532b1}';
_15 = core::ptr::addr_of!((*_15));
(*_15) = _5 | _5;
_5 = _9 as i8;
(*_15) = -_5;
(*_15) = _5 << RET;
RET = -(-5069602917862796821_i64);
(*_15) = _13 as i8;
_5 = (*_15);
_24 = _2 * _9;
_5 = (*_15) | (*_15);
_19.1.fld0.fld1 = _8 - _8;
_22 = -_13;
(*_15) = _5 << _3;
(*_15) = _5;
RET = 16982_u16 as i64;
(*_15) = _5 * _5;
_26 = Adt18::Variant0 { fld0: true,fld1: _20,fld2: _13,fld3: RET,fld4: _3 };
_11 = '\u{a124}';
_15 = core::ptr::addr_of!((*_15));
Goto(bb13)
}
bb46 = {
(*_17) = (*_15) <= _14;
(*_29) = core::ptr::addr_of!((*_17));
place!(Field::<f32>(Variant(_26, 0), 2)) = _27 as f32;
(*_38) = -_51.0;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_32.1.fld0.fld0, 0), 0)));
_15 = core::ptr::addr_of!(_14);
match _35.1 {
0 => bb1,
1 => bb15,
2 => bb3,
3 => bb13,
6245 => bb22,
_ => bb18
}
}
bb47 = {
(*_15) = _5 & _5;
_6 = _1 - _1;
(*_15) = _5 << _4;
_4 = _1 as u128;
(*_15) = !_5;
(*_15) = _3 as i8;
_6 = 5401364345937766620_usize as f64;
(*_15) = !_5;
RET = (-2885207941624962229_i64) - (-8828009332440374305_i64);
(*_15) = _5 - _5;
Goto(bb6)
}
bb48 = {
(*_38) = _51.0 << _4;
_70.2 = [Field::<i128>(Variant(_35.0, 0), 1),Field::<i128>(Variant(_35.0, 0), 1),Field::<i128>(Variant(_35.0, 0), 1),Field::<i128>(Variant(_35.0, 0), 1)];
(*_38) = _51.0 - _51.0;
_39 = _18 << (*_38);
_20 = Field::<usize>(Variant(_26, 0), 1) & Field::<usize>(Variant(_48, 0), 1);
(*_17) = !Field::<bool>(Variant(_48, 0), 0);
_70.3 = &mut _44;
_63 = _35.1;
_70.0 = Adt23::Variant0 { fld0: (*_17),fld1: Field::<i128>(Variant(_35.0, 0), 1),fld2: _48,fld3: Field::<u64>(Variant(_48, 0), 4),fld4: _22 };
_94 = Move(_15);
(*_17) = !_96;
place!(Field::<f32>(Variant(place!(Field::<Adt18>(Variant(_70.0, 0), 2)), 0), 2)) = Field::<usize>(Variant(Field::<Adt18>(Variant(_70.0, 0), 2), 0), 1) as f32;
_83 = core::ptr::addr_of!(_26);
_8 = _19.1.fld0.fld1 * _50.1.fld0.fld1;
(*_29) = core::ptr::addr_of!(place!(Field::<bool>(Variant((*_83), 0), 0)));
(*_83) = Field::<Adt18>(Variant(_35.0, 0), 2);
Goto(bb37)
}
bb49 = {
_97.2.fld0 = Move(_19.1.fld0.fld0);
(*_17) = _61;
_105 = [_115,_115,_115];
_18 = _106 as u64;
_19.1.fld1 = core::ptr::addr_of_mut!(_61);
_111 = (*_38) as isize;
(*_38) = -_51.0;
Goto(bb50)
}
bb50 = {
Call(_126 = dump_var(7_usize, 75_usize, Move(_75), 3_usize, Move(_3), 76_usize, Move(_76), 111_usize, Move(_111)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_126 = dump_var(7_usize, 81_usize, Move(_81), 106_usize, Move(_106), 11_usize, Move(_11), 115_usize, Move(_115)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_126 = dump_var(7_usize, 113_usize, Move(_113), 20_usize, Move(_20), 21_usize, Move(_21), 24_usize, Move(_24)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_126 = dump_var(7_usize, 62_usize, Move(_62), 84_usize, Move(_84), 93_usize, Move(_93), 71_usize, Move(_71)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_126 = dump_var(7_usize, 59_usize, Move(_59), 42_usize, Move(_42), 44_usize, Move(_44), 45_usize, Move(_45)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_126 = dump_var(7_usize, 46_usize, Move(_46), 95_usize, Move(_95), 105_usize, Move(_105), 53_usize, Move(_53)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_126 = dump_var(7_usize, 55_usize, Move(_55), 78_usize, Move(_78), 127_usize, _127, 127_usize, _127), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i64,mut _2: f64,mut _3: isize,mut _4: f64) -> u128 {
mir! {
type RET = u128;
let _5: &'static &'static mut Adt57;
let _6: i16;
let _7: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _8: &'static mut &'static usize;
let _9: *mut bool;
let _10: u8;
let _11: *mut &'static mut &'static usize;
let _12: i16;
let _13: u32;
let _14: [usize; 3];
let _15: f32;
let _16: *mut &'static mut Adt27;
let _17: f64;
let _18: isize;
let _19: &'static (bool, u64, i8, u128);
let _20: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _21: *mut isize;
let _22: i8;
let _23: f32;
let _24: f64;
let _25: &'static mut &'static usize;
let _26: Adt38;
let _27: bool;
let _28: isize;
let _29: char;
let _30: [i8; 7];
let _31: [u32; 6];
let _32: i16;
let _33: *mut Adt31;
let _34: *mut &'static mut &'static usize;
let _35: isize;
let _36: *mut isize;
let _37: *const Adt18;
let _38: i64;
let _39: [i128; 3];
let _40: *mut bool;
let _41: i32;
let _42: *mut [usize; 3];
let _43: &'static usize;
let _44: *const &'static mut i128;
let _45: [i128; 4];
let _46: bool;
let _47: char;
let _48: ([char; 4], Adt57);
let _49: Adt23;
let _50: *mut u128;
let _51: u32;
let _52: isize;
let _53: [u16; 2];
let _54: &'static mut i128;
let _55: *mut &'static mut &'static usize;
let _56: isize;
let _57: i64;
let _58: i16;
let _59: f64;
let _60: char;
let _61: bool;
let _62: *mut &'static mut &'static usize;
let _63: ();
let _64: ();
{
_4 = 1302_u16 as f64;
_1 = (-1005495844_i32) as i64;
_3 = 9223372036854775807_isize - (-85_isize);
_2 = -_4;
RET = _4 as u128;
_2 = -_4;
_4 = -_2;
_2 = 18813_i16 as f64;
_4 = 880860560_u32 as f64;
_2 = _4;
_2 = 7_i8 as f64;
_2 = 2237414308_u32 as f64;
RET = 325708002609353145491746342952124935260_u128;
RET = 1017866771574271756_u64 as u128;
_4 = _2;
RET = 191812466242346155255274230356241151613_u128;
_2 = _4 * _4;
_2 = -_4;
_2 = _4;
_4 = _2 + _2;
_3 = (-9223372036854775808_isize);
_2 = _4;
_4 = _2;
_4 = 17377_i16 as f64;
_3 = 9223372036854775807_isize & 9223372036854775807_isize;
_3 = !(-9223372036854775808_isize);
_1 = -3138344851729753552_i64;
RET = _3 as u128;
_1 = RET as i64;
Goto(bb1)
}
bb1 = {
RET = 161993670477126932688873024456414605418_u128;
_6 = !(-23299_i16);
RET = 244982858519484267865473029996167630635_u128 * 144506418001860282086719512113020551707_u128;
_2 = _4 - _4;
_4 = _2 * _2;
_3 = (-9223372036854775808_isize) << _6;
_1 = (-4096536162014306301_i64) << _6;
_1 = 981049797544911383_i64;
RET = !158350317076348825030385995997622221493_u128;
_3 = (-23_isize) * (-9223372036854775808_isize);
_6 = !17773_i16;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
981049797544911383 => bb10,
_ => bb9
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
Return()
}
bb10 = {
_1 = (-8271130348877855493_i64) << _3;
_3 = !(-9223372036854775808_isize);
_1 = (-1187074254866901489_i64) & (-1841812629785401901_i64);
_2 = _4 - _4;
_4 = -_2;
_2 = _4;
_2 = -_4;
RET = 89438331952097024137617857876812566_u128 << _3;
_2 = _6 as f64;
_7.1 = [99639476787386190954779050660174679429_i128,(-34124695232022698767657198528759417139_i128),100991143575906459059547611912011234107_i128,5650117232868568437609932920355297888_i128];
RET = 330684988627799981679053564293341683814_u128 & 305054488639254665248742380695570904861_u128;
Call(_1 = fn9(_4, _7.1, _3, _3, _4, RET, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_4 = -_2;
Goto(bb12)
}
bb12 = {
_7.2.fld1 = !73_u8;
RET = 748496985_i32 as u128;
_2 = _4 + _4;
_1 = -2799658686456117929_i64;
_6 = 17497_i16;
_7.2.fld1 = 189_u8 >> _3;
RET = '\u{3dfbb}' as u128;
_7.2.fld1 = 64_u8 & 45_u8;
_3 = !9223372036854775807_isize;
_1 = 2534613182157173615_i64 - (-4992694581334006456_i64);
_7.2.fld1 = 44_u8 ^ 40_u8;
_3 = 9223372036854775807_isize & 119_isize;
_7.2.fld1 = 225_u8;
_7.1 = [49454349201341442041128223229200578609_i128,(-136589574522480791439030793080108142833_i128),18762401568440334459970987618059597721_i128,(-120521522052061304235282559776115003031_i128)];
_7.2.fld1 = !15_u8;
_2 = _4 - _4;
_4 = _2 - _2;
_6 = 25133_i16;
_1 = (-5403683869519529590_i64) >> _6;
_7.1 = [77392679478513321920570679588496558929_i128,(-158215923362987805118304693133608649489_i128),147221417128212968587862428331549999968_i128,(-49515158821210625326146465254533984221_i128)];
_2 = -_4;
_7.1 = [(-110826499361877679724081380082904856082_i128),139567914530860887428582589570809832733_i128,98514206454804703166895910459739232761_i128,(-147716228496814525635106453038552545733_i128)];
_2 = _4;
Goto(bb13)
}
bb13 = {
_1 = 1686957654392080958_i64 + (-8940273271257353042_i64);
_1 = 33578708624623079033931289811231204815_i128 as i64;
RET = 29601478450235390314892581717086737305_u128;
_2 = _4 + _4;
Goto(bb14)
}
bb14 = {
_4 = _2 * _2;
_12 = _6 - _6;
_2 = _4 + _4;
_1 = '\u{a3498}' as i64;
_2 = _4 * _4;
_13 = (-30989194267810915837418921565577846435_i128) as u32;
_4 = _2 - _2;
_6 = _12;
_7.1 = [5225728862055294709933129802925471742_i128,(-84056946087694016609518616399388398639_i128),(-93037968979956146610771257035161450236_i128),136386604588991273563299373542569523226_i128];
_15 = _1 as f32;
_1 = (-2653662387398567294_i64) << _12;
match RET {
0 => bb1,
1 => bb7,
2 => bb4,
29601478450235390314892581717086737305 => bb16,
_ => bb15
}
}
bb15 = {
Return()
}
bb16 = {
_10 = 145731462803258953450386173138001765626_i128 as u8;
RET = 5_usize as u128;
_10 = _7.2.fld1 & _7.2.fld1;
_14 = [4587055001372101538_usize,2_usize,0_usize];
_7.1 = [153039644888904006972590502214543176141_i128,157348810181648486208290279554896896227_i128,59872084874498818810626741895539007515_i128,(-99119407812787710792323613760234756350_i128)];
_7.1 = [97164215311658405008191839750147788210_i128,(-63675869611678659610346562433755102996_i128),(-43697546538987253988979520531469487828_i128),(-144886410987411135525659357296158130925_i128)];
_13 = 202319214_i32 as u32;
_2 = _4 - _4;
_1 = 6501179700095706849_i64 + (-4159769983940673905_i64);
_17 = -_2;
_15 = (-2054811221_i32) as f32;
_1 = _15 as i64;
_7.2.fld1 = !_10;
_4 = _2 + _2;
_2 = 708514230877347813_usize as f64;
_7.1 = [(-163029027931863055150194445951562485804_i128),(-8145625997600045052141784966827733087_i128),(-38656793011024560852779232658119487463_i128),(-149912380123603862269702908419820113224_i128)];
_2 = _17 - _4;
_6 = _12;
_1 = true as i64;
Goto(bb17)
}
bb17 = {
_18 = _3 ^ _3;
_7.2.fld1 = _10 & _10;
_20.3 = &mut _18;
_2 = -_17;
RET = 318467407134291981406022170912804893860_u128 | 88188547380826235975495627823958385494_u128;
_6 = _12;
_20.2.0 = -(-267321875_i32);
_3 = _1 as isize;
_22 = !(-103_i8);
_3 = 12954677154550096554_u64 as isize;
Goto(bb18)
}
bb18 = {
_20.2 = (1349065057_i32,);
_22 = 55_i8;
_26.fld4 = 4_usize >> _10;
_20.2 = ((-1881637077_i32),);
_1 = (-4097654455984681561_i64);
_6 = '\u{ce8ad}' as i16;
_26.fld6 = _1;
_4 = _17 * _17;
Goto(bb19)
}
bb19 = {
_3 = !(-9223372036854775808_isize);
_20.2 = (1524966913_i32,);
_15 = _1 as f32;
_21 = core::ptr::addr_of_mut!(_3);
_7.3 = core::ptr::addr_of!(_26.fld3);
_7.1 = [(-32331032517807869419248594189985510212_i128),(-124815887346476659999630565786441618749_i128),(-129499495919566309296029622211588908977_i128),(-126187244367044931645682744048740340786_i128)];
_26.fld1 = _6 as f64;
_27 = !true;
_13 = !3196599342_u32;
_9 = core::ptr::addr_of_mut!(_27);
(*_9) = _2 > _2;
(*_21) = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_21) = _13 as isize;
_23 = _20.2.0 as f32;
(*_21) = 103_isize;
(*_9) = RET < RET;
(*_21) = (-9223372036854775808_isize) * (-106_isize);
_2 = _17;
(*_21) = (-9223372036854775808_isize) * (-9223372036854775808_isize);
(*_9) = !true;
(*_21) = (-9223372036854775808_isize) ^ 79_isize;
(*_9) = true;
_7.1 = [39336152993434145702886703403772324503_i128,(-21964677097139292128859066415797694594_i128),(-132334620873364274749018449725643497909_i128),64648154346315363005174089080948600269_i128];
Goto(bb20)
}
bb20 = {
(*_9) = true | false;
(*_9) = false;
(*_21) = (-9223372036854775808_isize);
_6 = _12 << (*_21);
(*_9) = _4 < _17;
_26.fld7 = [_6,_12,_12];
_27 = _2 == _17;
(*_21) = -(-9223372036854775808_isize);
_6 = _12 * _12;
(*_21) = 69_isize & 9223372036854775807_isize;
_6 = _12;
_26.fld7 = [_6,_6,_6];
_7.1 = [(-362074050050465061623741252480438583_i128),(-168728459874800268967616595014639557706_i128),18252265851650490190949668079552789065_i128,106412018336037054332963405269086004735_i128];
(*_9) = !false;
(*_9) = true;
(*_21) = 67_isize & 49_isize;
_26.fld4 = 14374548693612038226_usize;
(*_21) = -9223372036854775807_isize;
(*_9) = false;
_12 = _4 as i16;
(*_21) = (-9223372036854775808_isize) >> _12;
_26.fld5 = RET & RET;
(*_9) = true;
_4 = -_17;
_26.fld5 = '\u{46a2}' as u128;
match _22 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb4,
4 => bb18,
55 => bb21,
_ => bb14
}
}
bb21 = {
_20.3 = &mut (*_21);
RET = _26.fld5;
_26.fld7 = [_12,_12,_12];
(*_9) = false ^ true;
_26.fld4 = !13731365220067853532_usize;
_23 = _15;
(*_9) = !false;
_29 = '\u{65227}';
_7.1 = [(-131810670963925742614685798081874194257_i128),116940422079920242739435603325440549552_i128,29132867810435033638221007568826418577_i128,70135187581856134647381928354560129997_i128];
_24 = _17 - _4;
_26.fld1 = _2 - _17;
(*_9) = _12 == _12;
_14 = [_26.fld4,_26.fld4,_26.fld4];
(*_9) = _26.fld1 != _4;
match _26.fld6 {
0 => bb12,
1 => bb16,
2 => bb3,
3 => bb9,
4 => bb13,
5 => bb6,
6 => bb22,
340282366920938463459276952975783529895 => bb24,
_ => bb23
}
}
bb22 = {
Return()
}
bb23 = {
_1 = (-8271130348877855493_i64) << _3;
_3 = !(-9223372036854775808_isize);
_1 = (-1187074254866901489_i64) & (-1841812629785401901_i64);
_2 = _4 - _4;
_4 = -_2;
_2 = _4;
_2 = -_4;
RET = 89438331952097024137617857876812566_u128 << _3;
_2 = _6 as f64;
_7.1 = [99639476787386190954779050660174679429_i128,(-34124695232022698767657198528759417139_i128),100991143575906459059547611912011234107_i128,5650117232868568437609932920355297888_i128];
RET = 330684988627799981679053564293341683814_u128 & 305054488639254665248742380695570904861_u128;
Call(_1 = fn9(_4, _7.1, _3, _3, _4, RET, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb24 = {
(*_9) = _24 < _2;
(*_9) = true & true;
_26.fld2 = core::ptr::addr_of!((*_9));
(*_9) = true & false;
(*_9) = true;
_26.fld0 = Move(_9);
_2 = _26.fld1 + _26.fld1;
_23 = _15;
_26.fld0 = core::ptr::addr_of_mut!(_27);
_22 = (-71_i8) | 72_i8;
_28 = (-125_isize);
_13 = 1507657926_u32;
_30 = [_22,_22,_22,_22,_22,_22,_22];
_7.2.fld1 = _1 as u8;
_32 = _12 << _13;
_14 = [_26.fld4,_26.fld4,_26.fld4];
_17 = _26.fld1 - _26.fld1;
_24 = _26.fld1 + _26.fld1;
match _20.2.0 {
0 => bb1,
1 => bb18,
2 => bb17,
3 => bb4,
4 => bb12,
1524966913 => bb25,
_ => bb6
}
}
bb25 = {
_32 = !_12;
_28 = 9223372036854775807_isize;
_15 = _23 - _23;
_20.0 = core::ptr::addr_of!(_22);
_4 = _12 as f64;
_35 = _28 | _28;
_26.fld1 = _24;
_20.2 = (1808286930_i32,);
_22 = !(-106_i8);
_9 = core::ptr::addr_of_mut!(_27);
(*_9) = !true;
_20.3 = &mut _28;
_26.fld2 = core::ptr::addr_of!((*_9));
_13 = !458141026_u32;
(*_9) = _4 == _4;
_20.2 = ((-776220239_i32),);
_26.fld1 = -_2;
(*_9) = !true;
(*_9) = !false;
_32 = -_12;
Goto(bb26)
}
bb26 = {
(*_9) = _17 != _26.fld1;
(*_9) = true & false;
RET = _7.2.fld1 as u128;
(*_9) = _24 < _2;
(*_9) = false;
_17 = _24;
_36 = core::ptr::addr_of_mut!(_35);
_14 = [_26.fld4,_26.fld4,_26.fld4];
(*_36) = 110_isize << _32;
(*_9) = !false;
(*_36) = -9223372036854775807_isize;
_29 = '\u{c6aa2}';
(*_36) = 9223372036854775807_isize << _12;
_22 = 86_i8;
_41 = _20.2.0 << (*_36);
(*_36) = !(-10_isize);
_9 = core::ptr::addr_of_mut!((*_9));
Goto(bb27)
}
bb27 = {
_41 = _10 as i32;
_23 = _10 as f32;
(*_9) = true;
(*_36) = 9223372036854775807_isize;
(*_36) = (-9223372036854775808_isize);
(*_36) = 9223372036854775807_isize;
_20.0 = core::ptr::addr_of!(_22);
(*_9) = !true;
(*_36) = _26.fld5 as isize;
_1 = _29 as i64;
Goto(bb28)
}
bb28 = {
(*_9) = _26.fld1 != _4;
_26.fld1 = _24;
(*_9) = _12 != _32;
_13 = !2028858463_u32;
Goto(bb29)
}
bb29 = {
_45 = [(-130656768556309580600948498388980788354_i128),65444013773354707566066926134408629154_i128,158479769539684624513060399281212665300_i128,(-35334011594695329408953399300367707836_i128)];
(*_9) = !true;
_6 = -_12;
(*_36) = !2_isize;
(*_36) = (-29_isize);
Goto(bb30)
}
bb30 = {
_7.2.fld1 = _20.2.0 as u8;
_38 = _32 as i64;
_20.0 = core::ptr::addr_of!(_22);
(*_9) = _12 < _6;
_36 = core::ptr::addr_of_mut!((*_36));
Goto(bb31)
}
bb31 = {
_24 = -_2;
(*_36) = -(-9223372036854775808_isize);
_1 = _38 >> _12;
(*_9) = _17 < _2;
_17 = _26.fld1 * _2;
(*_36) = _17 as isize;
_4 = _2;
_48.1.fld1 = Move(_26.fld0);
_6 = _32;
(*_9) = !true;
_4 = _10 as f64;
_21 = core::ptr::addr_of_mut!((*_36));
_13 = 1393201865_u32;
(*_9) = (*_21) > (*_21);
_6 = _41 as i16;
(*_21) = -1_isize;
_39 = [(-92811657731823252568750584415055688536_i128),147607531610553240953676503175622128716_i128,(-150912897261282316443029264921346735132_i128)];
_47 = _29;
_15 = _23;
(*_9) = true ^ true;
_12 = !_32;
(*_21) = (-9223372036854775808_isize) << _32;
(*_9) = false | true;
(*_36) = -(-9223372036854775808_isize);
_43 = &_26.fld4;
_47 = _29;
_13 = 231760984_u32 | 3326888810_u32;
(*_36) = !1_isize;
(*_36) = _23 as isize;
Goto(bb32)
}
bb32 = {
RET = !_26.fld5;
(*_9) = !true;
_35 = (-9223372036854775808_isize) + 82_isize;
_48.1.fld2 = [RET,_26.fld5,RET,RET];
_17 = -_24;
_13 = _38 as u32;
(*_36) = 9223372036854775807_isize;
_8 = &mut _43;
(*_36) = 9223372036854775807_isize - 91_isize;
_48.0 = [_29,_29,_47,_47];
(*_36) = -9223372036854775807_isize;
(*_9) = true & false;
_51 = (*_9) as u32;
_4 = _26.fld1 * _2;
(*_36) = (-9223372036854775808_isize);
_11 = core::ptr::addr_of_mut!(_8);
_42 = core::ptr::addr_of_mut!(_14);
_34 = core::ptr::addr_of_mut!((*_11));
(*_42) = [_26.fld4,_26.fld4,_26.fld4];
_29 = _47;
Goto(bb33)
}
bb33 = {
(*_36) = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_25 = &mut (*_8);
(*_42) = [_26.fld4,_26.fld4,_26.fld4];
(*_42) = [_26.fld4,_26.fld4,_26.fld4];
_50 = core::ptr::addr_of_mut!(RET);
(*_50) = _26.fld5;
_24 = -_2;
(*_50) = !_26.fld5;
(*_36) = 45581_u16 as isize;
(*_11) = Move(_25);
_26.fld7 = [_12,_32,_32];
_6 = _29 as i16;
_50 = core::ptr::addr_of_mut!(RET);
(*_36) = (-65_isize) - 76_isize;
(*_50) = _26.fld5 * _26.fld5;
(*_9) = true;
_39 = [(-22190665439135973222019186183346713773_i128),(-85713601756738522753754210946604937182_i128),140183257796082515459643415295477010460_i128];
(*_42) = [_26.fld4,_26.fld4,_26.fld4];
Goto(bb34)
}
bb34 = {
_35 = !9223372036854775807_isize;
(*_50) = !_26.fld5;
(*_9) = false;
(*_36) = !(-9223372036854775808_isize);
(*_36) = (*_9) as isize;
_26.fld4 = 14287976955580427255_usize + 1_usize;
_26.fld4 = _12 as usize;
(*_9) = (*_50) >= (*_50);
_20.2 = (_41,);
_13 = (-79481358688158688042230783110301511575_i128) as u32;
(*_50) = _32 as u128;
(*_36) = 9223372036854775807_isize << (*_50);
(*_9) = !false;
(*_9) = (*_50) == (*_50);
(*_36) = 9223372036854775807_isize * (-9223372036854775808_isize);
_48.1.fld0.fld1 = !_10;
(*_42) = [_26.fld4,_26.fld4,_26.fld4];
Goto(bb35)
}
bb35 = {
Call(_63 = dump_var(8_usize, 45_usize, Move(_45), 3_usize, Move(_3), 30_usize, Move(_30), 10_usize, Move(_10)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_63 = dump_var(8_usize, 39_usize, Move(_39), 13_usize, Move(_13), 32_usize, Move(_32), 29_usize, Move(_29)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_63 = dump_var(8_usize, 22_usize, Move(_22), 27_usize, Move(_27), 64_usize, _64, 64_usize, _64), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: f64,mut _2: [i128; 4],mut _3: isize,mut _4: isize,mut _5: f64,mut _6: u128,mut _7: isize) -> i64 {
mir! {
type RET = i64;
let _8: f64;
let _9: &'static mut &'static usize;
let _10: u128;
let _11: [i128; 3];
let _12: char;
let _13: u128;
let _14: &'static mut Adt57;
let _15: [u16; 2];
let _16: bool;
let _17: (i16, *mut [usize; 3], u16);
let _18: i64;
let _19: u128;
let _20: i64;
let _21: *mut &'static mut &'static usize;
let _22: *const bool;
let _23: u64;
let _24: u128;
let _25: [i8; 7];
let _26: isize;
let _27: i8;
let _28: isize;
let _29: f64;
let _30: &'static Adt38;
let _31: &'static Adt57;
let _32: *const i8;
let _33: *mut &'static mut *mut u16;
let _34: isize;
let _35: &'static mut i32;
let _36: [i128; 3];
let _37: i128;
let _38: [u16; 2];
let _39: [i128; 3];
let _40: *mut u128;
let _41: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _42: &'static Adt38;
let _43: ();
let _44: ();
{
_5 = _1;
_8 = _5 + _5;
_2 = [(-30644356305388557718288729008060827218_i128),(-102978916628127783065381469444395515723_i128),(-78849662089695300256233411395458013371_i128),(-17591941900279404615151080506936243581_i128)];
_1 = -_8;
_4 = _7 | _3;
_3 = -_4;
_8 = _5 + _1;
_3 = !_4;
RET = (-8619834105230176986_i64);
_1 = _5;
_5 = _8 + _1;
_2 = [(-26313430437389507651859065229401719416_i128),(-254820525317023528747230185265321692_i128),128917995366105594961119105075127623192_i128,129764384456069060199061578106393787189_i128];
_1 = _8 * _8;
_4 = _3;
_3 = !_7;
_6 = 145204314433371594731754920783778672768_u128;
_3 = 1070889311_i32 as isize;
_7 = _3;
_6 = 239392652452387553114689603402464349392_u128 + 263257352431220015998738073110015784260_u128;
_2 = [(-3885259610640508972000186700938549153_i128),76748902195083109154973563424154293825_i128,(-80133981418251191377188630984907117272_i128),57747728326549157698273657475285793138_i128];
_1 = _5 - _8;
RET = 41137_u16 as i64;
_8 = _1;
_8 = _5 - _1;
_8 = _5;
_3 = _4 * _4;
Call(_6 = core::intrinsics::bswap(14183776560500938449660379468288459826_u128), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = 1052796190_u32 as f64;
Goto(bb2)
}
bb2 = {
_10 = !_6;
_4 = false as isize;
_10 = _6;
_3 = _4 ^ _7;
_4 = (-60719312023247031415161579435656688873_i128) as isize;
RET = 2386408622195681642_i64 - 177354797788266314_i64;
Goto(bb3)
}
bb3 = {
_5 = 26_u8 as f64;
Goto(bb4)
}
bb4 = {
_1 = -_8;
_3 = 57239_u16 as isize;
RET = !(-3065274343357276348_i64);
_4 = 17938782846379031334_usize as isize;
_5 = _8 * _8;
_5 = -_8;
_5 = -_8;
_4 = -_7;
_6 = 46_u8 as u128;
RET = 6578017519681275161_i64 + 3817612230603971799_i64;
_5 = _1 + _1;
_6 = _10;
_5 = _1 + _8;
_3 = false as isize;
_1 = _5;
_3 = '\u{45b38}' as isize;
_3 = _4;
RET = !(-1292574474306525618_i64);
RET = (-1870402828498958780_i64) ^ (-7872357870510569488_i64);
_6 = _10 >> _10;
_10 = _6;
_12 = '\u{4658a}';
RET = 4503042878534201529_i64 - 4565517416416081519_i64;
Call(_5 = fn10(_12, RET, _3, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_10 = _6 * _6;
_2 = [78855368554755971411438635312714607459_i128,(-42668500529532782032630325833500842639_i128),(-133726886500069106057302237465692094778_i128),82769952054530824339356146590143848731_i128];
_11 = [(-59872001395963289842565591543368968783_i128),(-53176193509755966079087676792205050092_i128),940664526142930377675908111364716940_i128];
_1 = -_5;
_1 = _5 * _5;
_11 = [142193343746828601778784906100260569374_i128,(-12231528308600576715052513775426794722_i128),62143201483074966766285464053603067705_i128];
_2 = [65665432431386401253511272384988318358_i128,123071842492571849344210005610799223872_i128,68656159140184260790442033943226114283_i128,(-95604228758374755592824793392389855877_i128)];
_11 = [115250099560507587718617301138401388342_i128,(-129300882300023747379644026451892374152_i128),(-69021639915354214310890819416639558376_i128)];
_12 = '\u{62dc}';
_11 = [(-80812564339999546563510244967040080409_i128),22115983745530931293670337840078442976_i128,49974355222887225398656834812918214336_i128];
Goto(bb6)
}
bb6 = {
_2 = [(-73280684864971881181929423326259980753_i128),55747590939018009707456648024633222572_i128,66759289380834827862403168891436139527_i128,77653377346621747442228831576132076097_i128];
_6 = _10 + _10;
RET = (-2016207881596714396_i64);
_6 = (-22_i8) as u128;
_10 = _6 - _6;
RET = _4 as i64;
_2 = [(-138006830220999467887027506274999308292_i128),(-21558867131670120693285057082275889533_i128),(-87261137057931548093743083170650364428_i128),134063191877879276501997027497831478731_i128];
_1 = _5;
_7 = _3 << _10;
RET = (-7177488317967268199_i64) << _3;
_6 = _10 >> _4;
_1 = _5 + _5;
_6 = !_10;
RET = (-6965461811587835514_i64) | 7404194422599481287_i64;
_5 = _1 * _1;
_10 = _6;
_13 = 54178_u16 as u128;
_3 = !_4;
_11 = [41672414476557791225675998626867216833_i128,72386415420217209248934590139310834992_i128,28172573013906778724395480080973919041_i128];
_11 = [(-134599132557068450354766367448480418239_i128),120489197833109824206702390167283734184_i128,24271125426660078844314272299545626606_i128];
RET = 5562799508292181253_i64;
_2 = [10892293330883064501029928264271583910_i128,9262856259943493145086689573477751526_i128,7748697300497575478327177688126749440_i128,57593430086592500831929836656722917171_i128];
_3 = RET as isize;
_10 = _6;
Call(_13 = fn12(RET, _2, _4, RET), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = 2510888696506252407_i64 >> _7;
_1 = 976200676_i32 as f64;
_4 = _7;
_4 = !_7;
_6 = _10 + _10;
_4 = 30863_u16 as isize;
_6 = _10;
_5 = 17659_u16 as f64;
_4 = _7;
RET = 8025217586875280654_i64 + 7136641329476159219_i64;
_13 = _10 >> _7;
_12 = '\u{102b59}';
RET = (-7778438148198565144_i64) << _10;
_5 = _8 + _8;
Goto(bb8)
}
bb8 = {
_4 = !_3;
_11 = [85947506011453504430634705998237186318_i128,(-115500709102006488414639371182711131289_i128),(-163461918910009966177773535172847002769_i128)];
RET = (-152232137645396335_i64) * 4880729523773043419_i64;
_7 = !_3;
_15 = [29348_u16,5782_u16];
RET = 8526836082115196643_i64;
RET = -1079080716095205723_i64;
_5 = -_1;
_11 = [82964336723142804759655185561835767022_i128,108870063654744616711476234593889935335_i128,151467707080041491714864455408967325290_i128];
_6 = _13 ^ _10;
_1 = (-55575033917761945302284361245211467322_i128) as f64;
_13 = !_6;
RET = 2908679152752232413_i64 * (-5833062290884151362_i64);
_10 = 244_u8 as u128;
_2 = [(-162731877205648253908713309732322709660_i128),(-89520518502798180367718973429153182997_i128),(-15207347626142963247527434611672794335_i128),(-39789622313966434908839195333132723127_i128)];
_3 = _7 << _13;
_18 = RET;
_4 = _3 ^ _3;
_5 = 2349971657628405000_u64 as f64;
_10 = !_13;
_18 = -RET;
_17.0 = 30264_i16 * 15205_i16;
_8 = _5 * _1;
_1 = _17.0 as f64;
_8 = _5;
_17.2 = 60234_u16 << _13;
_4 = _3 * _3;
_1 = _17.2 as f64;
Call(_17.2 = core::intrinsics::bswap(44722_u16), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_17.2 = 39567_u16 ^ 17799_u16;
_17.2 = 25769_u16;
_4 = 3272153412_u32 as isize;
_17.2 = 9743_u16;
_19 = _17.0 as u128;
_11 = [159212242847715643230173285527254368854_i128,(-24340931961393595697889743662724832664_i128),41995923727130421339707340750118143457_i128];
_7 = _3;
_15 = [_17.2,_17.2];
_10 = _13;
_16 = false;
_2 = [113152577864369926579237695466498200017_i128,(-86692392732345386368084737810081209879_i128),1868059264741662861499896604829272107_i128,(-141114539774088744227927932482715636423_i128)];
RET = _18 << _3;
_19 = (-847544756_i32) as u128;
_2 = [(-111635658721894075479960931614190520506_i128),(-44251131659986833079292919406583672676_i128),32124255998734384778126478039881740374_i128,(-21753451323619557363186457797986503073_i128)];
_3 = -_7;
Goto(bb10)
}
bb10 = {
_1 = (-68836652497893184333424408776884541188_i128) as f64;
_22 = core::ptr::addr_of!(_16);
(*_22) = RET > RET;
(*_22) = !false;
_16 = true;
(*_22) = true;
_15 = [_17.2,_17.2];
_5 = _1 - _8;
(*_22) = !true;
_12 = '\u{10032c}';
Goto(bb11)
}
bb11 = {
(*_22) = true ^ true;
(*_22) = _13 < _13;
RET = _18 | _18;
_1 = _5 + _8;
(*_22) = _8 <= _1;
(*_22) = !true;
(*_22) = false & true;
(*_22) = _13 < _10;
Goto(bb12)
}
bb12 = {
RET = _18 | _18;
_17.2 = 49506_u16 << _6;
(*_22) = true;
_4 = _7;
(*_22) = !false;
_17.0 = (-2583_i16);
_11 = [81499527786462749902719886546276258590_i128,49217610968222359620206147277210908138_i128,23698252176669825783433559669842261057_i128];
(*_22) = !true;
(*_22) = true | false;
(*_22) = true;
(*_22) = _7 >= _4;
RET = -_18;
_10 = 193_u8 as u128;
(*_22) = true | true;
_3 = _4 << _4;
_16 = false;
_23 = 5305655402990558844_u64;
_20 = RET - RET;
match _23 {
0 => bb9,
1 => bb8,
2 => bb5,
5305655402990558844 => bb14,
_ => bb13
}
}
bb13 = {
_10 = _6 * _6;
_2 = [78855368554755971411438635312714607459_i128,(-42668500529532782032630325833500842639_i128),(-133726886500069106057302237465692094778_i128),82769952054530824339356146590143848731_i128];
_11 = [(-59872001395963289842565591543368968783_i128),(-53176193509755966079087676792205050092_i128),940664526142930377675908111364716940_i128];
_1 = -_5;
_1 = _5 * _5;
_11 = [142193343746828601778784906100260569374_i128,(-12231528308600576715052513775426794722_i128),62143201483074966766285464053603067705_i128];
_2 = [65665432431386401253511272384988318358_i128,123071842492571849344210005610799223872_i128,68656159140184260790442033943226114283_i128,(-95604228758374755592824793392389855877_i128)];
_11 = [115250099560507587718617301138401388342_i128,(-129300882300023747379644026451892374152_i128),(-69021639915354214310890819416639558376_i128)];
_12 = '\u{62dc}';
_11 = [(-80812564339999546563510244967040080409_i128),22115983745530931293670337840078442976_i128,49974355222887225398656834812918214336_i128];
Goto(bb6)
}
bb14 = {
RET = -_20;
match _17.0 {
340282366920938463463374607431768208873 => bb15,
_ => bb1
}
}
bb15 = {
_26 = _7 >> _13;
(*_22) = true ^ true;
_28 = !_26;
_3 = _28;
_11 = [(-149784693293006321606339356537213958318_i128),(-101970744061512380581357381152715787345_i128),78810888433842642960150353631944809804_i128];
(*_22) = _28 > _28;
_23 = 9447925402239270763_u64 - 18422105911739623468_u64;
(*_22) = _1 <= _1;
(*_22) = _19 == _6;
_18 = RET & _20;
match _17.0 {
0 => bb16,
1 => bb17,
340282366920938463463374607431768208873 => bb19,
_ => bb18
}
}
bb16 = {
RET = -_20;
match _17.0 {
340282366920938463463374607431768208873 => bb15,
_ => bb1
}
}
bb17 = {
_4 = !_3;
_11 = [85947506011453504430634705998237186318_i128,(-115500709102006488414639371182711131289_i128),(-163461918910009966177773535172847002769_i128)];
RET = (-152232137645396335_i64) * 4880729523773043419_i64;
_7 = !_3;
_15 = [29348_u16,5782_u16];
RET = 8526836082115196643_i64;
RET = -1079080716095205723_i64;
_5 = -_1;
_11 = [82964336723142804759655185561835767022_i128,108870063654744616711476234593889935335_i128,151467707080041491714864455408967325290_i128];
_6 = _13 ^ _10;
_1 = (-55575033917761945302284361245211467322_i128) as f64;
_13 = !_6;
RET = 2908679152752232413_i64 * (-5833062290884151362_i64);
_10 = 244_u8 as u128;
_2 = [(-162731877205648253908713309732322709660_i128),(-89520518502798180367718973429153182997_i128),(-15207347626142963247527434611672794335_i128),(-39789622313966434908839195333132723127_i128)];
_3 = _7 << _13;
_18 = RET;
_4 = _3 ^ _3;
_5 = 2349971657628405000_u64 as f64;
_10 = !_13;
_18 = -RET;
_17.0 = 30264_i16 * 15205_i16;
_8 = _5 * _1;
_1 = _17.0 as f64;
_8 = _5;
_17.2 = 60234_u16 << _13;
_4 = _3 * _3;
_1 = _17.2 as f64;
Call(_17.2 = core::intrinsics::bswap(44722_u16), ReturnTo(bb9), UnwindUnreachable())
}
bb18 = {
RET = 2510888696506252407_i64 >> _7;
_1 = 976200676_i32 as f64;
_4 = _7;
_4 = !_7;
_6 = _10 + _10;
_4 = 30863_u16 as isize;
_6 = _10;
_5 = 17659_u16 as f64;
_4 = _7;
RET = 8025217586875280654_i64 + 7136641329476159219_i64;
_13 = _10 >> _7;
_12 = '\u{102b59}';
RET = (-7778438148198565144_i64) << _10;
_5 = _8 + _8;
Goto(bb8)
}
bb19 = {
_23 = !13345998519552713522_u64;
_17.2 = 40889_u16 - 59061_u16;
(*_22) = false ^ false;
match _17.0 {
0 => bb13,
1 => bb2,
2 => bb16,
3 => bb10,
4 => bb11,
340282366920938463463374607431768208873 => bb20,
_ => bb17
}
}
bb20 = {
(*_22) = true & false;
_26 = _13 as isize;
(*_22) = !false;
_4 = _7 << _26;
_10 = _6 - _13;
_12 = '\u{e4f3c}';
_38 = [_17.2,_17.2];
_4 = _3 | _28;
_3 = _4 + _7;
(*_22) = true;
(*_22) = RET <= _20;
match _17.0 {
0 => bb8,
1 => bb21,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
340282366920938463463374607431768208873 => bb27,
_ => bb26
}
}
bb21 = {
_1 = (-68836652497893184333424408776884541188_i128) as f64;
_22 = core::ptr::addr_of!(_16);
(*_22) = RET > RET;
(*_22) = !false;
_16 = true;
(*_22) = true;
_15 = [_17.2,_17.2];
_5 = _1 - _8;
(*_22) = !true;
_12 = '\u{10032c}';
Goto(bb11)
}
bb22 = {
RET = 2510888696506252407_i64 >> _7;
_1 = 976200676_i32 as f64;
_4 = _7;
_4 = !_7;
_6 = _10 + _10;
_4 = 30863_u16 as isize;
_6 = _10;
_5 = 17659_u16 as f64;
_4 = _7;
RET = 8025217586875280654_i64 + 7136641329476159219_i64;
_13 = _10 >> _7;
_12 = '\u{102b59}';
RET = (-7778438148198565144_i64) << _10;
_5 = _8 + _8;
Goto(bb8)
}
bb23 = {
_10 = _6 * _6;
_2 = [78855368554755971411438635312714607459_i128,(-42668500529532782032630325833500842639_i128),(-133726886500069106057302237465692094778_i128),82769952054530824339356146590143848731_i128];
_11 = [(-59872001395963289842565591543368968783_i128),(-53176193509755966079087676792205050092_i128),940664526142930377675908111364716940_i128];
_1 = -_5;
_1 = _5 * _5;
_11 = [142193343746828601778784906100260569374_i128,(-12231528308600576715052513775426794722_i128),62143201483074966766285464053603067705_i128];
_2 = [65665432431386401253511272384988318358_i128,123071842492571849344210005610799223872_i128,68656159140184260790442033943226114283_i128,(-95604228758374755592824793392389855877_i128)];
_11 = [115250099560507587718617301138401388342_i128,(-129300882300023747379644026451892374152_i128),(-69021639915354214310890819416639558376_i128)];
_12 = '\u{62dc}';
_11 = [(-80812564339999546563510244967040080409_i128),22115983745530931293670337840078442976_i128,49974355222887225398656834812918214336_i128];
Goto(bb6)
}
bb24 = {
_17.2 = 39567_u16 ^ 17799_u16;
_17.2 = 25769_u16;
_4 = 3272153412_u32 as isize;
_17.2 = 9743_u16;
_19 = _17.0 as u128;
_11 = [159212242847715643230173285527254368854_i128,(-24340931961393595697889743662724832664_i128),41995923727130421339707340750118143457_i128];
_7 = _3;
_15 = [_17.2,_17.2];
_10 = _13;
_16 = false;
_2 = [113152577864369926579237695466498200017_i128,(-86692392732345386368084737810081209879_i128),1868059264741662861499896604829272107_i128,(-141114539774088744227927932482715636423_i128)];
RET = _18 << _3;
_19 = (-847544756_i32) as u128;
_2 = [(-111635658721894075479960931614190520506_i128),(-44251131659986833079292919406583672676_i128),32124255998734384778126478039881740374_i128,(-21753451323619557363186457797986503073_i128)];
_3 = -_7;
Goto(bb10)
}
bb25 = {
_2 = [(-73280684864971881181929423326259980753_i128),55747590939018009707456648024633222572_i128,66759289380834827862403168891436139527_i128,77653377346621747442228831576132076097_i128];
_6 = _10 + _10;
RET = (-2016207881596714396_i64);
_6 = (-22_i8) as u128;
_10 = _6 - _6;
RET = _4 as i64;
_2 = [(-138006830220999467887027506274999308292_i128),(-21558867131670120693285057082275889533_i128),(-87261137057931548093743083170650364428_i128),134063191877879276501997027497831478731_i128];
_1 = _5;
_7 = _3 << _10;
RET = (-7177488317967268199_i64) << _3;
_6 = _10 >> _4;
_1 = _5 + _5;
_6 = !_10;
RET = (-6965461811587835514_i64) | 7404194422599481287_i64;
_5 = _1 * _1;
_10 = _6;
_13 = 54178_u16 as u128;
_3 = !_4;
_11 = [41672414476557791225675998626867216833_i128,72386415420217209248934590139310834992_i128,28172573013906778724395480080973919041_i128];
_11 = [(-134599132557068450354766367448480418239_i128),120489197833109824206702390167283734184_i128,24271125426660078844314272299545626606_i128];
RET = 5562799508292181253_i64;
_2 = [10892293330883064501029928264271583910_i128,9262856259943493145086689573477751526_i128,7748697300497575478327177688126749440_i128,57593430086592500831929836656722917171_i128];
_3 = RET as isize;
_10 = _6;
Call(_13 = fn12(RET, _2, _4, RET), ReturnTo(bb7), UnwindUnreachable())
}
bb26 = {
RET = -_20;
match _17.0 {
340282366920938463463374607431768208873 => bb15,
_ => bb1
}
}
bb27 = {
RET = _20 >> _4;
_17.0 = (-29166_i16) ^ 9856_i16;
_27 = 39_i8 << _3;
_8 = _1 - _1;
_37 = (-38645042931983628934399384676838321574_i128);
_10 = _13 << RET;
_25 = [_27,_27,_27,_27,_27,_27,_27];
_16 = !true;
Goto(bb28)
}
bb28 = {
Call(_43 = dump_var(9_usize, 26_usize, Move(_26), 3_usize, Move(_3), 18_usize, Move(_18), 6_usize, Move(_6)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_43 = dump_var(9_usize, 28_usize, Move(_28), 23_usize, Move(_23), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_43 = dump_var(9_usize, 13_usize, Move(_13), 15_usize, Move(_15), 44_usize, _44, 44_usize, _44), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: char,mut _2: i64,mut _3: isize,mut _4: char) -> f64 {
mir! {
type RET = f64;
let _5: f64;
let _6: isize;
let _7: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _8: &'static *const Adt18;
let _9: Adt31;
let _10: &'static &'static mut Adt57;
let _11: [i8; 7];
let _12: ();
let _13: ();
{
_3 = (-9223372036854775808_isize) >> _2;
_4 = _1;
_7.2.0 = 1119471755_i32;
_7.2 = ((-242883321_i32),);
_1 = _4;
_2 = 6332228645525573003_i64;
RET = 10379858254550581154_usize as f64;
_6 = _3;
_7.3 = &mut _3;
_7.2 = (837254580_i32,);
_5 = RET - RET;
_4 = _1;
_7.3 = &mut _6;
_7.2 = ((-342463841_i32),);
_2 = 620009923083013297_i64;
RET = _5;
_7.2 = (787718627_i32,);
_7.2.0 = (-2054020384_i32) & (-427832398_i32);
_1 = _4;
_4 = _1;
_2 = 7760895973662442584_i64 >> _7.2.0;
_1 = _4;
Call(RET = fn11(Move(_7.3), _7.2, _1, _2, _7.2, _7.2, _4, _2, _1, _1, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _1;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(10_usize, 1_usize, Move(_1), 6_usize, Move(_6), 13_usize, _13, 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: &'static mut isize,mut _2: (i32,),mut _3: char,mut _4: i64,mut _5: (i32,),mut _6: (i32,),mut _7: char,mut _8: i64,mut _9: char,mut _10: char,mut _11: char,mut _12: char) -> f64 {
mir! {
type RET = f64;
let _13: [char; 4];
let _14: i128;
let _15: *mut bool;
let _16: [i128; 4];
let _17: Adt23;
let _18: Adt44;
let _19: u128;
let _20: char;
let _21: isize;
let _22: *const i8;
let _23: &'static Adt57;
let _24: [i8; 7];
let _25: &'static i32;
let _26: (Adt57, *const bool);
let _27: isize;
let _28: (i32,);
let _29: &'static mut *mut u16;
let _30: i64;
let _31: isize;
let _32: [u32; 6];
let _33: f64;
let _34: ();
let _35: ();
{
_5 = (_2.0,);
_10 = _9;
_2 = _6;
_11 = _12;
_2.0 = _6.0 ^ _6.0;
RET = _2.0 as f64;
_10 = _7;
_4 = !_8;
_6 = (_2.0,);
_7 = _3;
_5.0 = -_6.0;
RET = (-60204319873164214603939389701975881468_i128) as f64;
_9 = _7;
_2 = _6;
_2.0 = _6.0 >> _6.0;
_6 = _2;
Goto(bb1)
}
bb1 = {
_8 = _4 | _4;
_2 = _6;
_8 = !_4;
_6 = (_5.0,);
_11 = _7;
_4 = 30_i8 as i64;
_6 = _2;
_5.0 = _6.0;
_5.0 = _6.0 ^ _6.0;
_14 = (-1121100063259938254224098497788985203_i128) << _6.0;
_9 = _10;
_8 = _4 ^ _4;
_2.0 = _6.0 * _5.0;
_2 = (_5.0,);
Goto(bb2)
}
bb2 = {
_18.fld1 = 227_u8;
_16 = [_14,_14,_14,_14];
_6 = _5;
_14 = !153270445174239586086185764084130007149_i128;
_3 = _12;
_5.0 = -_2.0;
_10 = _11;
_12 = _9;
_6 = (_5.0,);
_5.0 = _2.0;
_13 = [_12,_7,_9,_9];
_20 = _3;
_20 = _11;
_6.0 = _5.0 ^ _5.0;
_12 = _11;
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = _11;
_13 = [_3,_20,_11,_12];
RET = _18.fld1 as f64;
_7 = _12;
_6 = (_2.0,);
_25 = &_5.0;
_9 = _7;
_2 = _5;
_21 = 15689_u16 as isize;
_28 = ((*_25),);
_8 = _4 & _4;
_5 = _28;
_5 = _2;
_19 = 235129004006007027440997827593235241989_u128;
_19 = 150638112319262908006069657958564308253_u128 ^ 231048635303181374262778865395275742927_u128;
_14 = 68516967987454759508743735551903544414_i128 & 11691711948679165472085952456388377709_i128;
_10 = _9;
_5.0 = !_28.0;
_26.0.fld2 = [_19,_19,_19,_19];
_19 = 27865365761073798068026695503853339831_u128 >> _28.0;
_27 = _21 >> _6.0;
match _18.fld1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
227 => bb7,
_ => bb6
}
}
bb4 = {
_18.fld1 = 227_u8;
_16 = [_14,_14,_14,_14];
_6 = _5;
_14 = !153270445174239586086185764084130007149_i128;
_3 = _12;
_5.0 = -_2.0;
_10 = _11;
_12 = _9;
_6 = (_5.0,);
_5.0 = _2.0;
_13 = [_12,_7,_9,_9];
_20 = _3;
_20 = _11;
_6.0 = _5.0 ^ _5.0;
_12 = _11;
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_8 = _4 | _4;
_2 = _6;
_8 = !_4;
_6 = (_5.0,);
_11 = _7;
_4 = 30_i8 as i64;
_6 = _2;
_5.0 = _6.0;
_5.0 = _6.0 ^ _6.0;
_14 = (-1121100063259938254224098497788985203_i128) << _6.0;
_9 = _10;
_8 = _4 ^ _4;
_2.0 = _6.0 * _5.0;
_2 = (_5.0,);
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_5 = (_2.0,);
_6.0 = _5.0 | _28.0;
_3 = _10;
_2 = (_6.0,);
_26.0.fld2 = [_19,_19,_19,_19];
_26.0.fld0.fld1 = _18.fld1;
_5.0 = _28.0 & _28.0;
_26.0.fld0.fld1 = !_18.fld1;
_26.0.fld0.fld1 = _18.fld1 << _5.0;
_16 = [_14,_14,_14,_14];
_5.0 = _28.0 * _6.0;
_20 = _12;
_31 = _27 + _27;
_18.fld1 = _26.0.fld0.fld1 | _26.0.fld0.fld1;
_8 = _4;
_16 = [_14,_14,_14,_14];
_30 = _4 << _2.0;
RET = 4293301034_u32 as f64;
RET = _31 as f64;
_13 = [_10,_11,_9,_10];
_32 = [1823751570_u32,2365766419_u32,3113578832_u32,2507649277_u32,2715701810_u32,960145027_u32];
_25 = &_28.0;
Goto(bb8)
}
bb8 = {
Call(_34 = dump_var(11_usize, 2_usize, Move(_2), 32_usize, Move(_32), 16_usize, Move(_16), 5_usize, Move(_5)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_34 = dump_var(11_usize, 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8), 19_usize, Move(_19)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_34 = dump_var(11_usize, 31_usize, Move(_31), 11_usize, Move(_11), 12_usize, Move(_12), 35_usize, _35), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i64,mut _2: [i128; 4],mut _3: isize,mut _4: i64) -> u128 {
mir! {
type RET = u128;
let _5: i64;
let _6: f32;
let _7: i64;
let _8: f32;
let _9: isize;
let _10: f32;
let _11: u64;
let _12: bool;
let _13: isize;
let _14: char;
let _15: i64;
let _16: [char; 4];
let _17: isize;
let _18: u16;
let _19: bool;
let _20: [i16; 3];
let _21: *mut i64;
let _22: i8;
let _23: f64;
let _24: *const *const (i32,);
let _25: isize;
let _26: isize;
let _27: &'static Adt57;
let _28: *mut Adt31;
let _29: Adt18;
let _30: *const (i32,);
let _31: isize;
let _32: u32;
let _33: char;
let _34: char;
let _35: bool;
let _36: *const i8;
let _37: f32;
let _38: *mut &'static mut *mut u16;
let _39: &'static &'static mut Adt57;
let _40: *mut isize;
let _41: &'static Adt57;
let _42: Adt23;
let _43: (bool, u64, i8, u128);
let _44: bool;
let _45: isize;
let _46: *const *const (i32,);
let _47: isize;
let _48: isize;
let _49: *mut &'static mut Adt27;
let _50: i8;
let _51: bool;
let _52: isize;
let _53: char;
let _54: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _55: [char; 4];
let _56: i128;
let _57: &'static mut i32;
let _58: i128;
let _59: [char; 3];
let _60: &'static [char; 3];
let _61: bool;
let _62: bool;
let _63: isize;
let _64: u128;
let _65: f64;
let _66: u16;
let _67: bool;
let _68: *mut &'static mut Adt27;
let _69: ();
let _70: ();
{
RET = 144660516267397521214738394329631132435_u128 >> _4;
RET = 295692856318153429243362322653165109546_u128;
_5 = (-95_i8) as i64;
_4 = _3 as i64;
_2 = [29064629891034148641426607751839090613_i128,(-131853225654721480168519302371625838848_i128),26466173070658236830364825047947973397_i128,(-122152590790322566070912158898395697024_i128)];
_4 = _5 >> _1;
RET = 325486818715353649792851771670511735817_u128 + 254380039163827691879665709069659768849_u128;
_5 = _4;
_1 = _4 | _5;
_7 = _1 ^ _5;
_7 = (-18711_i16) as i64;
_4 = !_1;
_3 = -(-69_isize);
RET = 129096298311772108202462081867172843460_u128 - 259013817974177621139799526596318723402_u128;
_2 = [(-133250821833056648151547715709900196733_i128),98020247275745924035696349911605656051_i128,148239622082108378250287154513857366015_i128,(-21997841784493158848513256242200993128_i128)];
_6 = 152_u8 as f32;
_6 = 2241_i16 as f32;
_6 = 8497164620277762452_u64 as f32;
_1 = (-137287409552329718145820807075273297100_i128) as i64;
_2 = [35882004365651306054388936770765585448_i128,(-153644755554691442516136121906576244875_i128),164340859148033785221837054977896251000_i128,(-116644483395313792252263673396275544875_i128)];
_3 = 2_isize;
_4 = 47428827493896576551030402020347959890_i128 as i64;
_5 = _3 as i64;
_4 = _5 | _7;
Goto(bb1)
}
bb1 = {
RET = 286460678883891300768417394897633678751_u128 + 101378723023365093595139471714809725579_u128;
_6 = 3164010892_u32 as f32;
_6 = 48_i8 as f32;
_4 = (-1543253751_i32) as i64;
_4 = -_1;
_1 = _4;
_7 = -_5;
_7 = _5;
match _3 {
0 => bb2,
1 => bb3,
3 => bb5,
4 => bb6,
2 => bb8,
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
_4 = (-1941631909_i32) as i64;
_5 = _7 * _7;
_6 = 44829_u16 as f32;
_7 = _5 | _5;
_4 = _5;
_8 = 17126073393695220408_u64 as f32;
_7 = 1_usize as i64;
_6 = _3 as f32;
_3 = 2780_i16 as isize;
_3 = (-8_isize);
_3 = -(-9223372036854775808_isize);
_11 = !11816487138490185830_u64;
_10 = _6 + _8;
_9 = !_3;
_10 = 2974421407_u32 as f32;
_1 = _5;
_5 = _1 ^ _7;
Goto(bb9)
}
bb9 = {
_2 = [109183669219197696599848948220288212531_i128,57629519552763065011193182524883927980_i128,(-167917904932521103256689598504052868371_i128),(-65997366948889358883230644461899158240_i128)];
_14 = '\u{45e6d}';
_15 = -_4;
_1 = _5 & _5;
RET = _8 as u128;
_14 = '\u{9ff5}';
_2 = [(-16290002926774740865995603153868291925_i128),57083056784808127389336255734638992935_i128,1094015649970371688044078549802194061_i128,(-81920307586023698494288084318916555793_i128)];
_4 = _5 * _5;
_7 = _11 as i64;
_7 = _5 | _1;
Goto(bb10)
}
bb10 = {
_1 = -_4;
_7 = 3845947703_u32 as i64;
_9 = _3 * _3;
_1 = _5 & _5;
_9 = _3 & _3;
_6 = 159465658961512917010827971078489754337_i128 as f32;
_12 = !false;
_11 = !4935727480679078391_u64;
_15 = _8 as i64;
RET = 287924225975642981826535610177738924005_u128 + 3013899268822743341667047875476080516_u128;
_14 = '\u{d9b62}';
_13 = -_3;
_15 = 88_i8 as i64;
_1 = !_5;
_2 = [18404755750215605710687924118412427623_i128,97211663938873661996224536979346263074_i128,22757283482380231724180220252655297143_i128,129406522884517381938740470054605091680_i128];
_5 = !_1;
_8 = 64_u8 as f32;
_10 = _8;
_14 = '\u{3275a}';
_6 = _10;
_5 = _1 ^ _4;
_15 = _1 & _5;
_1 = !_4;
_7 = _4 * _4;
_15 = RET as i64;
_5 = _7 | _1;
_10 = _6 - _6;
Goto(bb11)
}
bb11 = {
_16 = [_14,_14,_14,_14];
_12 = true & false;
_4 = _5;
_15 = _4;
_3 = -_9;
_15 = RET as i64;
_10 = _8;
_10 = -_6;
RET = 270344220587126928187644814919462104169_u128 & 213760139285560487030465456947221695010_u128;
_14 = '\u{e22}';
_16 = [_14,_14,_14,_14];
_8 = _6 * _10;
Goto(bb12)
}
bb12 = {
_13 = -_9;
_17 = _9 * _9;
_13 = _9 - _17;
_9 = _13;
_12 = false;
_6 = _10;
_9 = _3 - _13;
_18 = 233_u8 as u16;
_15 = _1 ^ _7;
_9 = 42982012919273947787872599165586636891_i128 as isize;
Goto(bb13)
}
bb13 = {
_2 = [(-161254577130545955831503583218490658476_i128),(-20809047588656504609043954492480664233_i128),(-108832266284588070563293762196337765305_i128),147071062453792148561590439679927319092_i128];
_18 = 33151_u16 + 48414_u16;
_16 = [_14,_14,_14,_14];
_14 = '\u{ee0be}';
_3 = _9 * _17;
Goto(bb14)
}
bb14 = {
_6 = (-3017_i16) as f32;
_20 = [1875_i16,30883_i16,(-9783_i16)];
_15 = _5;
_13 = _3;
_5 = _15 + _4;
_19 = !_12;
_11 = 8578842649745459572_u64;
_22 = !106_i8;
_18 = 14794_u16;
_2 = [49434334404882424312253671963493025103_i128,77983881478265480133168313028575713292_i128,64876121543331957601542473742389427768_i128,166646064049947815949882268086388585242_i128];
_11 = !72298507404591308_u64;
_19 = !_12;
Goto(bb15)
}
bb15 = {
_19 = _12;
_20 = [17416_i16,(-29871_i16),(-5441_i16)];
_18 = 22911_u16 << _13;
_1 = !_15;
_14 = '\u{a4870}';
_17 = _3 | _3;
_13 = !_17;
_5 = -_7;
_21 = core::ptr::addr_of_mut!(_5);
_19 = _12;
_20 = [15291_i16,(-9516_i16),(-4390_i16)];
(*_21) = !_15;
_6 = _8;
_4 = RET as i64;
(*_21) = _15;
Goto(bb16)
}
bb16 = {
(*_21) = _15 + _7;
_15 = !(*_21);
(*_21) = !_15;
_13 = _17 - _17;
(*_21) = !_1;
(*_21) = _1 * _7;
Goto(bb17)
}
bb17 = {
_17 = !_9;
(*_21) = _7 & _15;
Goto(bb18)
}
bb18 = {
_3 = -_13;
_22 = _13 as i8;
(*_21) = _1;
_9 = _3;
RET = 329483572749532205001751856861137032995_u128 & 120133091476243248724367606372609462087_u128;
_25 = _3;
(*_21) = _15;
_14 = '\u{57dfd}';
_19 = _12;
_16 = [_14,_14,_14,_14];
(*_21) = _1 >> _1;
_20 = [(-11176_i16),(-21815_i16),(-4566_i16)];
_12 = !_19;
_7 = 4282988587_u32 as i64;
_2 = [(-142932984587302041543727266971983720677_i128),122917735957430118759713696112561032433_i128,112992752499227881739421935896586712185_i128,145761175219145174963179458185904361089_i128];
(*_21) = _15 + _15;
_11 = 3555487266948948081_u64 ^ 6852870770252842431_u64;
(*_21) = 143_u8 as i64;
_22 = -(-74_i8);
_25 = _13 ^ _9;
Goto(bb19)
}
bb19 = {
(*_21) = RET as i64;
(*_21) = _15 * _15;
(*_21) = _15 << _25;
(*_21) = _15;
(*_21) = _15;
_13 = (-4667_i16) as isize;
_16 = [_14,_14,_14,_14];
_20 = [26929_i16,17586_i16,(-18629_i16)];
(*_21) = (-348019571_i32) as i64;
(*_21) = -_15;
(*_21) = _15 | _15;
Goto(bb20)
}
bb20 = {
(*_21) = !_4;
(*_21) = _1 - _4;
Goto(bb21)
}
bb21 = {
_2 = [(-124982955022897591672714464884558632940_i128),(-90161811107250940002282372595685341975_i128),(-135554370394956968282950050885633141766_i128),72140563362514028703253765157434924503_i128];
(*_21) = _10 as i64;
_22 = 34_i8 ^ (-124_i8);
Goto(bb22)
}
bb22 = {
_14 = '\u{8b05c}';
_6 = (*_21) as f32;
(*_21) = _15;
(*_21) = _15 << _15;
(*_21) = _15 * _1;
_23 = _15 as f64;
_29 = Adt18::Variant1 { fld0: _3,fld1: _20 };
_17 = !_25;
(*_21) = -_1;
(*_21) = _15 ^ _15;
_1 = (*_21) & (*_21);
_31 = _3 | _9;
_17 = Field::<isize>(Variant(_29, 1), 0) & _9;
_4 = (*_21);
_7 = _1 << _31;
_21 = core::ptr::addr_of_mut!(_5);
Goto(bb23)
}
bb23 = {
_18 = 51465_u16;
(*_21) = _1;
_26 = Field::<isize>(Variant(_29, 1), 0);
(*_21) = -_4;
_22 = 52_i8 << (*_21);
_16 = [_14,_14,_14,_14];
_29 = Adt18::Variant1 { fld0: _17,fld1: _20 };
(*_21) = _7 + _1;
(*_21) = _1;
(*_21) = _19 as i64;
_6 = _10;
(*_21) = -_1;
_21 = core::ptr::addr_of_mut!((*_21));
_32 = !2250768923_u32;
_20 = Field::<[i16; 3]>(Variant(_29, 1), 1);
Goto(bb24)
}
bb24 = {
_7 = (*_21);
RET = 126855813955757009303908238541771716414_u128 + 222724303534274363267843516506877387343_u128;
_19 = (*_21) < _1;
Call((*_21) = fn13(_9, _10, _12, _9, Move(_21), _9, _9, _4, _25, _17), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_13 = _19 as isize;
_21 = core::ptr::addr_of_mut!(_1);
_11 = 17985908783367861583_u64 + 2201759184612572929_u64;
_25 = _9 >> _17;
_32 = !1359382410_u32;
_21 = core::ptr::addr_of_mut!(_7);
(*_21) = _4 ^ _15;
(*_21) = _5;
(*_21) = _5;
(*_21) = (-797508277_i32) as i64;
_19 = _12 | _12;
_12 = !_19;
_2 = [(-167166602533838414485239946938378850182_i128),96908105918387633209385527145582645048_i128,11204030439642078560270380908896015453_i128,146620270094946356663369051680159129565_i128];
_3 = -_31;
(*_21) = _22 as i64;
(*_21) = !_15;
(*_21) = _15;
place!(Field::<isize>(Variant(_29, 1), 0)) = (*_21) as isize;
Goto(bb26)
}
bb26 = {
(*_21) = _1;
(*_21) = _14 as i64;
Goto(bb27)
}
bb27 = {
_16 = [_14,_14,_14,_14];
_2 = [(-38911654732144330543238154513945395035_i128),4788993502202735969626897551412839692_i128,11402236050977601072943036419396707567_i128,154345417052698446385453764146161898224_i128];
(*_21) = 197_u8 as i64;
_2 = [111436435169316636440304854725048981340_i128,(-133508203170366019656415208626364340431_i128),(-23768993674269884974688144958413900167_i128),96394478908755411667652593441118211493_i128];
_18 = !30910_u16;
_22 = (-19_i8) ^ 51_i8;
_4 = _15 - _5;
_29 = Adt18::Variant0 { fld0: _19,fld1: 2_usize,fld2: _10,fld3: (*_21),fld4: _11 };
_37 = _8 + _6;
(*_21) = _4 >> _1;
(*_21) = -_4;
RET = 215472585834323370260319302916338226724_u128;
_25 = _3 - _3;
(*_21) = _1 * _15;
_16 = [_14,_14,_14,_14];
_32 = !2423945632_u32;
_1 = -(*_21);
place!(Field::<usize>(Variant(_29, 0), 1)) = 1_usize;
_32 = !2504455181_u32;
_34 = _14;
_7 = _1 >> _1;
_5 = (*_21);
_5 = (*_21);
match RET {
0 => bb9,
1 => bb26,
2 => bb22,
3 => bb28,
215472585834323370260319302916338226724 => bb30,
_ => bb29
}
}
bb28 = {
_19 = _12;
_20 = [17416_i16,(-29871_i16),(-5441_i16)];
_18 = 22911_u16 << _13;
_1 = !_15;
_14 = '\u{a4870}';
_17 = _3 | _3;
_13 = !_17;
_5 = -_7;
_21 = core::ptr::addr_of_mut!(_5);
_19 = _12;
_20 = [15291_i16,(-9516_i16),(-4390_i16)];
(*_21) = !_15;
_6 = _8;
_4 = RET as i64;
(*_21) = _15;
Goto(bb16)
}
bb29 = {
_13 = -_9;
_17 = _9 * _9;
_13 = _9 - _17;
_9 = _13;
_12 = false;
_6 = _10;
_9 = _3 - _13;
_18 = 233_u8 as u16;
_15 = _1 ^ _7;
_9 = 42982012919273947787872599165586636891_i128 as isize;
Goto(bb13)
}
bb30 = {
_25 = _13 * _9;
_19 = _12 ^ Field::<bool>(Variant(_29, 0), 0);
_6 = _8 * _8;
_29 = Adt18::Variant0 { fld0: _19,fld1: 14335278327580154004_usize,fld2: _37,fld3: (*_21),fld4: _11 };
(*_21) = (-160146684184722092321892307481453164874_i128) as i64;
_12 = Field::<bool>(Variant(_29, 0), 0) | Field::<bool>(Variant(_29, 0), 0);
_2 = [(-86266463038242477497211430351195527246_i128),33056355031864097542932011807449729385_i128,(-89831064153923777982175516996161287726_i128),37934702324360880797314393774121812819_i128];
(*_21) = _4 * _1;
_16 = [_34,_14,_14,_14];
_3 = _13 + _17;
_35 = (*_21) == (*_21);
_23 = (-7279_i16) as f64;
_1 = _4 ^ (*_21);
_35 = (*_21) >= (*_21);
_26 = -_17;
_10 = (*_21) as f32;
_25 = _11 as isize;
_18 = 1819_u16 & 36258_u16;
_15 = _32 as i64;
_8 = -_10;
_14 = _34;
_1 = (*_21) | (*_21);
(*_21) = Field::<i64>(Variant(_29, 0), 3) | Field::<i64>(Variant(_29, 0), 3);
place!(Field::<f32>(Variant(_29, 0), 2)) = _8;
_5 = -(*_21);
_26 = 24691935114898841119288800166501898947_i128 as isize;
Call(place!(Field::<f32>(Variant(_29, 0), 2)) = fn14(), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_33 = _34;
(*_21) = 1706902869_i32 as i64;
_26 = _17 >> Field::<i64>(Variant(_29, 0), 3);
match RET {
0 => bb12,
1 => bb17,
2 => bb4,
3 => bb32,
4 => bb33,
5 => bb34,
215472585834323370260319302916338226724 => bb36,
_ => bb35
}
}
bb32 = {
Return()
}
bb33 = {
_6 = (-3017_i16) as f32;
_20 = [1875_i16,30883_i16,(-9783_i16)];
_15 = _5;
_13 = _3;
_5 = _15 + _4;
_19 = !_12;
_11 = 8578842649745459572_u64;
_22 = !106_i8;
_18 = 14794_u16;
_2 = [49434334404882424312253671963493025103_i128,77983881478265480133168313028575713292_i128,64876121543331957601542473742389427768_i128,166646064049947815949882268086388585242_i128];
_11 = !72298507404591308_u64;
_19 = !_12;
Goto(bb15)
}
bb34 = {
_1 = -_4;
_7 = 3845947703_u32 as i64;
_9 = _3 * _3;
_1 = _5 & _5;
_9 = _3 & _3;
_6 = 159465658961512917010827971078489754337_i128 as f32;
_12 = !false;
_11 = !4935727480679078391_u64;
_15 = _8 as i64;
RET = 287924225975642981826535610177738924005_u128 + 3013899268822743341667047875476080516_u128;
_14 = '\u{d9b62}';
_13 = -_3;
_15 = 88_i8 as i64;
_1 = !_5;
_2 = [18404755750215605710687924118412427623_i128,97211663938873661996224536979346263074_i128,22757283482380231724180220252655297143_i128,129406522884517381938740470054605091680_i128];
_5 = !_1;
_8 = 64_u8 as f32;
_10 = _8;
_14 = '\u{3275a}';
_6 = _10;
_5 = _1 ^ _4;
_15 = _1 & _5;
_1 = !_4;
_7 = _4 * _4;
_15 = RET as i64;
_5 = _7 | _1;
_10 = _6 - _6;
Goto(bb11)
}
bb35 = {
_2 = [(-124982955022897591672714464884558632940_i128),(-90161811107250940002282372595685341975_i128),(-135554370394956968282950050885633141766_i128),72140563362514028703253765157434924503_i128];
(*_21) = _10 as i64;
_22 = 34_i8 ^ (-124_i8);
Goto(bb22)
}
bb36 = {
_32 = (-26476_i16) as u32;
_17 = _3 + _3;
_29 = Adt18::Variant0 { fld0: _35,fld1: 5_usize,fld2: _8,fld3: _4,fld4: _11 };
_21 = core::ptr::addr_of_mut!((*_21));
_7 = Field::<i64>(Variant(_29, 0), 3);
_34 = _14;
_24 = core::ptr::addr_of!(_30);
_32 = 3186663751_u32;
_21 = core::ptr::addr_of_mut!((*_21));
_43.3 = !RET;
_43.0 = _3 <= _3;
_29 = Adt18::Variant0 { fld0: _35,fld1: 17408370620730010007_usize,fld2: _8,fld3: (*_21),fld4: _11 };
(*_21) = _4 + _5;
_32 = 3815651812_u32 & 3325690474_u32;
place!(Field::<f32>(Variant(_29, 0), 2)) = _8;
_46 = core::ptr::addr_of!((*_24));
place!(Field::<f32>(Variant(_29, 0), 2)) = (*_21) as f32;
_43.0 = Field::<bool>(Variant(_29, 0), 0) ^ _35;
(*_21) = _1;
_43.0 = Field::<bool>(Variant(_29, 0), 0);
place!(Field::<u64>(Variant(_29, 0), 4)) = _11 | _11;
_3 = _13 + _31;
Goto(bb37)
}
bb37 = {
_40 = core::ptr::addr_of_mut!(_13);
_43.2 = _22 - _22;
_1 = (*_21);
(*_21) = -Field::<i64>(Variant(_29, 0), 3);
place!(Field::<i64>(Variant(_29, 0), 3)) = 159157977124201704495488789889240028543_i128 as i64;
(*_21) = _1 + _1;
_17 = (*_40);
place!(Field::<u64>(Variant(_29, 0), 4)) = (*_21) as u64;
_32 = 2618686229_u32 >> (*_21);
match RET {
0 => bb12,
1 => bb19,
215472585834323370260319302916338226724 => bb39,
_ => bb38
}
}
bb38 = {
_4 = (-1941631909_i32) as i64;
_5 = _7 * _7;
_6 = 44829_u16 as f32;
_7 = _5 | _5;
_4 = _5;
_8 = 17126073393695220408_u64 as f32;
_7 = 1_usize as i64;
_6 = _3 as f32;
_3 = 2780_i16 as isize;
_3 = (-8_isize);
_3 = -(-9223372036854775808_isize);
_11 = !11816487138490185830_u64;
_10 = _6 + _8;
_9 = !_3;
_10 = 2974421407_u32 as f32;
_1 = _5;
_5 = _1 ^ _7;
Goto(bb9)
}
bb39 = {
_48 = (*_40) | (*_40);
match RET {
0 => bb7,
215472585834323370260319302916338226724 => bb40,
_ => bb16
}
}
bb40 = {
_17 = (*_40) * (*_40);
(*_21) = _1;
(*_40) = _3;
(*_21) = -_5;
_46 = core::ptr::addr_of!((*_24));
_50 = _43.2 << _7;
_47 = (*_40) | (*_40);
place!(Field::<f32>(Variant(_29, 0), 2)) = 24886_i16 as f32;
_45 = (*_40);
(*_40) = _17;
_2 = [128883808502873860279747806698123301773_i128,105359231125048244099566854763699123260_i128,99937905954188698500097072269538184404_i128,(-14127464935534843952973740047279469797_i128)];
(*_40) = _31 | _31;
place!(Field::<i64>(Variant(_29, 0), 3)) = RET as i64;
_43.1 = Field::<u64>(Variant(_29, 0), 4) ^ Field::<u64>(Variant(_29, 0), 4);
_52 = _8 as isize;
_45 = (*_40);
_26 = _31 << (*_21);
(*_21) = _33 as i64;
(*_40) = _1 as isize;
(*_21) = !_4;
place!(Field::<i64>(Variant(_29, 0), 3)) = -(*_21);
_43.1 = _8 as u64;
_22 = _50 & _50;
_44 = Field::<bool>(Variant(_29, 0), 0) ^ Field::<bool>(Variant(_29, 0), 0);
(*_40) = -_26;
Goto(bb41)
}
bb41 = {
RET = 13448487455089059121_usize as u128;
(*_21) = _4 ^ Field::<i64>(Variant(_29, 0), 3);
_13 = (*_21) as isize;
Goto(bb42)
}
bb42 = {
(*_40) = !_47;
_32 = RET as u32;
(*_40) = _17 ^ _17;
(*_21) = _4 * _1;
Goto(bb43)
}
bb43 = {
(*_40) = _52 + _52;
(*_46) = core::ptr::addr_of!(_54.2);
_46 = core::ptr::addr_of!((*_46));
_54.1 = core::ptr::addr_of!(_29);
_9 = _33 as isize;
(*_30).0 = -357320591_i32;
_22 = -_50;
_54.0 = core::ptr::addr_of!(_50);
_9 = _52;
_17 = (*_40) & (*_40);
_5 = (*_21);
(*_21) = Field::<i64>(Variant(_29, 0), 3) & _1;
(*_46) = core::ptr::addr_of!((*_30));
(*_46) = core::ptr::addr_of!((*_30));
(*_24) = core::ptr::addr_of!((*_30));
(*_30).0 = 1747975916_i32;
_54.2 = (1365987557_i32,);
(*_40) = 12579_i16 as isize;
place!(Field::<u64>(Variant(_29, 0), 4)) = _43.1 << (*_21);
(*_21) = (-15859_i16) as i64;
(*_40) = _47 >> _47;
(*_40) = _48 * _17;
_10 = _8;
match (*_30).0 {
1365987557 => bb45,
_ => bb44
}
}
bb44 = {
RET = 13448487455089059121_usize as u128;
(*_21) = _4 ^ Field::<i64>(Variant(_29, 0), 3);
_13 = (*_21) as isize;
Goto(bb42)
}
bb45 = {
(*_21) = -_1;
(*_30).0 = (-1521674260_i32) + 391356279_i32;
(*_40) = Field::<u64>(Variant(_29, 0), 4) as isize;
_29 = Adt18::Variant1 { fld0: _52,fld1: _20 };
Goto(bb46)
}
bb46 = {
(*_21) = _4 ^ _1;
_33 = _34;
(*_30).0 = RET as i32;
(*_30).0 = -(-149266715_i32);
(*_21) = _1 | _5;
_52 = -(*_40);
_30 = core::ptr::addr_of!((*_30));
(*_30) = (1670971949_i32,);
_43.2 = (*_21) as i8;
(*_24) = core::ptr::addr_of!((*_30));
(*_24) = core::ptr::addr_of!((*_30));
(*_30) = ((-1227125956_i32),);
_40 = core::ptr::addr_of_mut!((*_40));
_11 = !_43.1;
_37 = 13531_i16 as f32;
(*_30).0 = 466671766_i32 ^ 391331863_i32;
Goto(bb47)
}
bb47 = {
(*_40) = _3 >> _47;
_54.2.0 = (*_40) as i32;
(*_30).0 = !(-1134823002_i32);
(*_24) = core::ptr::addr_of!((*_30));
(*_30).0 = (-1224660272_i32) - 289736875_i32;
(*_24) = core::ptr::addr_of!((*_30));
_29 = Adt18::Variant1 { fld0: (*_40),fld1: _20 };
_64 = _8 as u128;
_15 = _43.0 as i64;
_15 = (*_21) >> (*_21);
Goto(bb48)
}
bb48 = {
_33 = _14;
(*_30) = (521849543_i32,);
_10 = _8 * _8;
_6 = _10 * _10;
(*_40) = 1_usize as isize;
(*_30) = (599559931_i32,);
(*_30) = ((-716830512_i32),);
(*_30).0 = (-1541802781_i32) & (-1338219367_i32);
_58 = 63402960881715879889406782886110008953_i128 >> _11;
_53 = _33;
(*_21) = -_5;
_35 = _43.0 ^ _12;
_53 = _34;
(*_40) = _47 >> _50;
(*_24) = core::ptr::addr_of!((*_30));
Goto(bb49)
}
bb49 = {
(*_40) = _43.0 as isize;
(*_40) = !_17;
_61 = _12 & _35;
(*_30).0 = 1509687981_i32 << Field::<isize>(Variant(_29, 1), 0);
_56 = _58;
_5 = (*_21) ^ _1;
_43.2 = -_22;
_2 = [_58,_58,_56,_58];
(*_30) = (147440189_i32,);
_67 = _15 > (*_21);
Goto(bb50)
}
bb50 = {
Call(_69 = dump_var(12_usize, 1_usize, Move(_1), 31_usize, Move(_31), 26_usize, Move(_26), 4_usize, Move(_4)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_69 = dump_var(12_usize, 45_usize, Move(_45), 35_usize, Move(_35), 56_usize, Move(_56), 64_usize, Move(_64)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_69 = dump_var(12_usize, 12_usize, Move(_12), 32_usize, Move(_32), 34_usize, Move(_34), 15_usize, Move(_15)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_69 = dump_var(12_usize, 16_usize, Move(_16), 48_usize, Move(_48), 47_usize, Move(_47), 19_usize, Move(_19)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_69 = dump_var(12_usize, 20_usize, Move(_20), 22_usize, Move(_22), 25_usize, Move(_25), 70_usize, _70), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: f32,mut _3: bool,mut _4: isize,mut _5: *mut i64,mut _6: isize,mut _7: isize,mut _8: i64,mut _9: isize,mut _10: isize) -> i64 {
mir! {
type RET = i64;
let _11: [char; 3];
let _12: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _13: &'static mut *mut u16;
let _14: (Adt23, u16, [i128; 4], &'static mut isize);
let _15: ();
let _16: ();
{
_4 = _9;
_10 = _1 & _4;
RET = _8 * _8;
_8 = RET;
_6 = _7 * _10;
_10 = _6 | _6;
_10 = _7 - _1;
_12.3 = &mut _10;
_11 = ['\u{d02a9}','\u{b8ae0}','\u{f1b72}'];
_12.3 = &mut _1;
_14.1 = 39508_u16;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(13_usize, 9_usize, Move(_9), 11_usize, Move(_11), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14() -> f32 {
mir! {
type RET = f32;
let _1: f64;
let _2: i16;
let _3: u128;
let _4: i8;
let _5: u16;
let _6: usize;
let _7: *const (i32,);
let _8: *mut &'static mut Adt27;
let _9: isize;
let _10: [u32; 6];
let _11: i128;
let _12: &'static mut &'static usize;
let _13: bool;
let _14: f64;
let _15: &'static mut i128;
let _16: f64;
let _17: *mut [i128; 4];
let _18: Adt18;
let _19: isize;
let _20: *mut &'static mut &'static usize;
let _21: Adt18;
let _22: (bool, u64, i8, u128);
let _23: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _24: isize;
let _25: bool;
let _26: &'static &'static mut Adt57;
let _27: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _28: &'static (bool, u64, i8, u128);
let _29: u8;
let _30: *mut &'static mut &'static usize;
let _31: *mut i64;
let _32: [i128; 3];
let _33: char;
let _34: *const *mut u16;
let _35: u128;
let _36: u64;
let _37: (bool, u64, i8, u128);
let _38: Adt31;
let _39: *mut *mut [usize; 3];
let _40: *const *mut [usize; 3];
let _41: *mut *const bool;
let _42: *mut Adt31;
let _43: &'static mut &'static usize;
let _44: u8;
let _45: ([char; 4], Adt57);
let _46: f64;
let _47: &'static mut i128;
let _48: u32;
let _49: Adt44;
let _50: [char; 3];
let _51: isize;
let _52: *mut [usize; 3];
let _53: *mut &'static mut Adt27;
let _54: Adt44;
let _55: i8;
let _56: f32;
let _57: (Adt57, *const bool);
let _58: *mut isize;
let _59: u128;
let _60: Adt38;
let _61: (i32,);
let _62: *mut Adt31;
let _63: &'static mut Adt57;
let _64: Adt38;
let _65: [u32; 6];
let _66: isize;
let _67: *mut [usize; 3];
let _68: f64;
let _69: (Adt23, u16, [i128; 4], &'static mut isize);
let _70: (bool, u64, i8, u128);
let _71: isize;
let _72: [u16; 2];
let _73: u16;
let _74: *mut [usize; 3];
let _75: [i128; 4];
let _76: *const *mut u16;
let _77: [i128; 3];
let _78: &'static mut i128;
let _79: [u32; 6];
let _80: u16;
let _81: *mut &'static mut &'static usize;
let _82: char;
let _83: char;
let _84: char;
let _85: [u128; 4];
let _86: *const *mut u16;
let _87: &'static Adt57;
let _88: *const *mut u16;
let _89: &'static mut isize;
let _90: i128;
let _91: Adt57;
let _92: i16;
let _93: ();
let _94: ();
{
RET = 9223372036854775807_isize as f32;
_1 = 23738_u16 as f64;
_1 = 157_u8 as f64;
_1 = 9223372036854775807_isize as f64;
_1 = 140_u8 as f64;
_1 = 188_u8 as f64;
RET = 999912255525012516_u64 as f32;
RET = (-1524_i16) as f32;
_1 = RET as f64;
RET = (-1224456051854008600_i64) as f32;
RET = _1 as f32;
_1 = (-9223372036854775808_isize) as f64;
_2 = (-20375_i16);
_3 = 7465627834434192773_usize as u128;
_3 = !133673154018968088157287358081648512132_u128;
_2 = _1 as i16;
_1 = (-3271559688572838451_i64) as f64;
_1 = (-95_i8) as f64;
_3 = 288806250309600778276420496925129070000_u128 | 9590273522416731591101466350818516278_u128;
Call(RET = fn15(_3, _2, _1, _3, _1, _2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _1 as i8;
_2 = (-30369_i16) - (-8602_i16);
RET = 12865743887420358116_usize as f32;
_4 = 79_i8;
_3 = !201755675250628654238996545817975430160_u128;
RET = _4 as f32;
_4 = (-4_i8);
_5 = 9201_u16 + 18365_u16;
Goto(bb2)
}
bb2 = {
_5 = 47919_u16 + 55665_u16;
_5 = 11292_u16 + 64685_u16;
_3 = 64469766862739694581290531465793150361_u128;
RET = _4 as f32;
_4 = !(-121_i8);
_5 = 30916_u16;
_6 = 5_usize & 4_usize;
_3 = 82128695305514333538747109888270113650_u128 << _5;
_9 = RET as isize;
_6 = !7615946297194248516_usize;
_6 = 1586530853362402210_i64 as usize;
_6 = 7_usize + 7112650223068311284_usize;
_4 = (-76_i8) | 12_i8;
_1 = _4 as f64;
_1 = 931186831_u32 as f64;
_4 = _3 as i8;
_5 = !8099_u16;
_5 = 20661_u16 ^ 41799_u16;
_1 = 7077690318542931223_u64 as f64;
Goto(bb3)
}
bb3 = {
_3 = 54714893162870946359108458719919505438_u128 + 280387524294774607723833470750177279068_u128;
_3 = 143152097835212534716205718266683601466_u128;
_4 = (-101_i8) << _2;
_3 = 42055477115447914480035770463736302651_u128 * 164849574325220165108704052839273618213_u128;
_1 = 197_u8 as f64;
_10 = [2313300491_u32,3975299707_u32,1669221752_u32,2231114458_u32,1868097702_u32,3650035845_u32];
_2 = !(-32279_i16);
_9 = 14396747976305295854147961506839774718_i128 as isize;
_2 = (-23656_i16);
_3 = 64916685946502488940412461221276533191_u128 | 98254679175856252438426778747788271146_u128;
_10 = [1789989244_u32,3144166240_u32,923089800_u32,617268776_u32,2025313690_u32,392748321_u32];
_5 = 7035_u16 & 24409_u16;
_6 = 2593019312795319122_u64 as usize;
_5 = !15258_u16;
_10 = [3605069207_u32,4231625370_u32,3470917478_u32,2101896596_u32,3354383159_u32,1056233173_u32];
RET = _2 as f32;
Goto(bb4)
}
bb4 = {
_11 = 5201802596157532950943828718743549952_i128;
_10 = [1335878954_u32,3618151004_u32,2585481372_u32,881623481_u32,3331374572_u32,821635573_u32];
_9 = !(-9_isize);
_1 = _4 as f64;
_1 = _11 as f64;
_2 = 28232_i16;
_13 = _9 == _9;
_4 = 105_i8 | 119_i8;
_5 = !53819_u16;
_10 = [242841644_u32,2258142403_u32,4264066042_u32,425218420_u32,3774791231_u32,3577081306_u32];
_10 = [3606519184_u32,722708763_u32,2056794125_u32,3894379520_u32,2865745218_u32,4118501859_u32];
_2 = !(-26322_i16);
_3 = 33716572101534622467859131474914186909_u128 << _9;
_9 = 9223372036854775807_isize >> _4;
Goto(bb5)
}
bb5 = {
_6 = 2_usize;
_10[_6] = _5 as u32;
RET = 3526565092769710335_u64 as f32;
_6 = 5_usize * 5_usize;
_2 = _5 as i16;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
5201802596157532950943828718743549952 => bb9,
_ => bb8
}
}
bb6 = {
_11 = 5201802596157532950943828718743549952_i128;
_10 = [1335878954_u32,3618151004_u32,2585481372_u32,881623481_u32,3331374572_u32,821635573_u32];
_9 = !(-9_isize);
_1 = _4 as f64;
_1 = _11 as f64;
_2 = 28232_i16;
_13 = _9 == _9;
_4 = 105_i8 | 119_i8;
_5 = !53819_u16;
_10 = [242841644_u32,2258142403_u32,4264066042_u32,425218420_u32,3774791231_u32,3577081306_u32];
_10 = [3606519184_u32,722708763_u32,2056794125_u32,3894379520_u32,2865745218_u32,4118501859_u32];
_2 = !(-26322_i16);
_3 = 33716572101534622467859131474914186909_u128 << _9;
_9 = 9223372036854775807_isize >> _4;
Goto(bb5)
}
bb7 = {
_3 = 54714893162870946359108458719919505438_u128 + 280387524294774607723833470750177279068_u128;
_3 = 143152097835212534716205718266683601466_u128;
_4 = (-101_i8) << _2;
_3 = 42055477115447914480035770463736302651_u128 * 164849574325220165108704052839273618213_u128;
_1 = 197_u8 as f64;
_10 = [2313300491_u32,3975299707_u32,1669221752_u32,2231114458_u32,1868097702_u32,3650035845_u32];
_2 = !(-32279_i16);
_9 = 14396747976305295854147961506839774718_i128 as isize;
_2 = (-23656_i16);
_3 = 64916685946502488940412461221276533191_u128 | 98254679175856252438426778747788271146_u128;
_10 = [1789989244_u32,3144166240_u32,923089800_u32,617268776_u32,2025313690_u32,392748321_u32];
_5 = 7035_u16 & 24409_u16;
_6 = 2593019312795319122_u64 as usize;
_5 = !15258_u16;
_10 = [3605069207_u32,4231625370_u32,3470917478_u32,2101896596_u32,3354383159_u32,1056233173_u32];
RET = _2 as f32;
Goto(bb4)
}
bb8 = {
_4 = _1 as i8;
_2 = (-30369_i16) - (-8602_i16);
RET = 12865743887420358116_usize as f32;
_4 = 79_i8;
_3 = !201755675250628654238996545817975430160_u128;
RET = _4 as f32;
_4 = (-4_i8);
_5 = 9201_u16 + 18365_u16;
Goto(bb2)
}
bb9 = {
_9 = !9223372036854775807_isize;
_14 = -_1;
_15 = &mut _11;
_3 = 16386571271319038061_u64 as u128;
_10 = [3396713190_u32,3807331236_u32,1154261281_u32,3070200205_u32,3669132413_u32,2808353755_u32];
(*_15) = !158135790173113627863618570772923867766_i128;
_3 = _13 as u128;
Goto(bb10)
}
bb10 = {
(*_15) = (-6405041869195474095805203714877549137_i128) - (-32666888734781610430778351190834838094_i128);
(*_15) = '\u{d1990}' as i128;
(*_15) = -(-157825851138980940379098768300090170468_i128);
(*_15) = (-46631942702969584693077456238063384713_i128) << _6;
_10 = [2379293098_u32,2870598686_u32,693572980_u32,774471907_u32,1716180373_u32,3584852965_u32];
RET = _9 as f32;
_3 = 3705510318881437389470442249593646147_u128 * 98456572294291704265809244043698056174_u128;
Goto(bb11)
}
bb11 = {
(*_15) = (-108104274280874024100165890488906000821_i128) & (-11562210297454549469410238223642970372_i128);
(*_15) = !(-111659877860930106503150968124355128536_i128);
(*_15) = !157588920168467043244962235278585653720_i128;
(*_15) = (-158798231648857412744405727852839767631_i128);
(*_15) = '\u{fcfdf}' as i128;
_22 = (_13, 11458462210721369899_u64, _4, _3);
_22.3 = !_3;
(*_15) = (-112999265134838284282438431188931655144_i128) >> _6;
_7 = core::ptr::addr_of!(_23.2);
_7 = core::ptr::addr_of!((*_7));
(*_7).0 = 3185753298_u32 as i32;
(*_7).0 = 1322622130_i32;
Goto(bb12)
}
bb12 = {
(*_7).0 = !1014772069_i32;
(*_7).0 = 1884143241_i32 & (-288841526_i32);
(*_7) = ((-637819994_i32),);
(*_15) = 145402999952544159482302047011419038980_i128 + (-96870150668614387196387324656171242504_i128);
(*_7) = ((-782007798_i32),);
(*_7).0 = 216370512_i32;
(*_15) = (-119140587904168356421994299393942384675_i128) + 158876372195886783221201905242688396510_i128;
_13 = (*_15) != (*_15);
(*_7).0 = 2048593165_i32 * (-839152432_i32);
(*_15) = (-133315143226173289599447429262759532122_i128) + (-18378582504783619354798750663829466559_i128);
(*_15) = 4250469743498805869685500190155782534_i128 | 144760374085663651575818133645939925115_i128;
match _22.1 {
0 => bb1,
1 => bb2,
11458462210721369899 => bb13,
_ => bb8
}
}
bb13 = {
(*_7) = ((-958644277_i32),);
(*_7).0 = 710668862_i32 * 776495747_i32;
(*_7).0 = 236118917_i32 >> (*_15);
_23.3 = &mut _9;
(*_7) = ((-88081303_i32),);
(*_7).0 = 190_u8 as i32;
_23.2.0 = 679782195_i32 & (-399102021_i32);
(*_7) = ((-2043233175_i32),);
_4 = (*_15) as i8;
_22 = (_13, 11420110750370059121_u64, _4, _3);
(*_15) = 122067793365833428335837934902214266475_i128 & (-36495514036094284558045412765182609677_i128);
(*_7).0 = 1714398688_i32 - 1326387908_i32;
_23.0 = core::ptr::addr_of!(_22.2);
(*_7) = ((-699521527_i32),);
(*_15) = (-116230220094228570539151347660025972813_i128) & 117506216901783052251975095686310594205_i128;
_23.0 = core::ptr::addr_of!(_4);
(*_7) = (518988442_i32,);
(*_7) = (1211059277_i32,);
_14 = _1 + _1;
(*_7) = (294590923_i32,);
_23.0 = core::ptr::addr_of!(_4);
(*_7) = (21489401_i32,);
(*_7).0 = (-1270467743_i32) - (-1037848666_i32);
(*_15) = (-53819133779052315204499950298171604500_i128) | 81294157010929065249100228374580568685_i128;
_29 = !106_u8;
Goto(bb14)
}
bb14 = {
(*_15) = 106370714971747527729210050468842227094_i128 ^ 136510301500946506869113798592401473570_i128;
_23.0 = core::ptr::addr_of!(_22.2);
_2 = -17060_i16;
_22.0 = (*_15) <= (*_15);
(*_7) = ((-2137255507_i32),);
(*_15) = (-35924949055382147099687331526410407334_i128) << (*_7).0;
(*_7) = (2131699147_i32,);
_14 = _22.3 as f64;
(*_7).0 = (-423577000_i32) & 396013884_i32;
(*_7).0 = (-2113224066_i32) >> (*_15);
_16 = _1;
(*_15) = RET as i128;
RET = _22.1 as f32;
(*_15) = 3436301280_u32 as i128;
(*_7) = (701649532_i32,);
(*_7) = ((-1317669587_i32),);
_23.1 = core::ptr::addr_of!(_21);
Goto(bb15)
}
bb15 = {
(*_15) = 83156254715927027844347920624297434466_i128 - (-33802226506267464929020141672038378710_i128);
_37.0 = _22.0;
(*_15) = _29 as i128;
(*_7) = (578451042_i32,);
(*_7).0 = -186283510_i32;
(*_7) = ((-1030533589_i32),);
match (*_7).0 {
0 => bb8,
1 => bb11,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607430737677867 => bb16,
_ => bb9
}
}
bb16 = {
(*_15) = _5 as i128;
_22.3 = _13 as u128;
_37 = _22;
RET = (*_7).0 as f32;
_37.3 = _22.3 << (*_15);
_19 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = !(-23002_i16);
_32 = [(*_15),(*_15),(*_15)];
(*_7).0 = 1077297398_i32;
_25 = !_22.0;
(*_7).0 = (-2114303570_i32) << _37.2;
_38 = Adt31::Variant1 { fld0: _22.2,fld1: (*_15) };
_36 = _37.1 ^ _22.1;
_37.0 = (*_15) != Field::<i128>(Variant(_38, 1), 1);
_23.2.0 = 36755929_i32;
_21 = Adt18::Variant0 { fld0: _13,fld1: _6,fld2: RET,fld3: (-9019824292057649920_i64),fld4: _36 };
(*_7) = (997083029_i32,);
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1379212333_i32,);
_27.2.fld1 = !_29;
place!(Field::<bool>(Variant(_21, 0), 0)) = !_13;
(*_7).0 = 867569912_i32;
_27.1 = [(*_15),(*_15),(*_15),(*_15)];
(*_7) = ((-25974040_i32),);
match _22.1 {
0 => bb1,
11420110750370059121 => bb17,
_ => bb9
}
}
bb17 = {
(*_15) = Field::<i128>(Variant(_38, 1), 1);
_24 = _19 + _19;
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_15) = Field::<i128>(Variant(_38, 1), 1) - Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1395726440_i32,);
_17 = core::ptr::addr_of_mut!(_27.1);
Goto(bb18)
}
bb18 = {
(*_15) = Field::<i128>(Variant(_38, 1), 1);
_29 = !_27.2.fld1;
_21 = Adt18::Variant0 { fld0: _37.0,fld1: _6,fld2: RET,fld3: (-6909065149842076206_i64),fld4: _22.1 };
(*_15) = Field::<i128>(Variant(_38, 1), 1) & Field::<i128>(Variant(_38, 1), 1);
(*_7) = (412583731_i32,);
(*_7) = (1225543757_i32,);
(*_7).0 = !(-389256444_i32);
_31 = core::ptr::addr_of_mut!(place!(Field::<i64>(Variant(_21, 0), 3)));
_23.2.0 = (-549767673_i32);
(*_15) = Field::<i128>(Variant(_38, 1), 1) - Field::<i128>(Variant(_38, 1), 1);
(*_15) = Field::<i128>(Variant(_38, 1), 1) | Field::<i128>(Variant(_38, 1), 1);
place!(Field::<f32>(Variant(_21, 0), 2)) = RET + RET;
(*_15) = Field::<i128>(Variant(_38, 1), 1) - Field::<i128>(Variant(_38, 1), 1);
(*_31) = (-8029704722238015524_i64) ^ 4992709324622764737_i64;
(*_15) = !Field::<i128>(Variant(_38, 1), 1);
_42 = core::ptr::addr_of_mut!(_38);
(*_31) = 4107215452183805126_i64 << _36;
_36 = _37.1 | _37.1;
match (*_7).0 {
0 => bb6,
1 => bb16,
2 => bb7,
3 => bb19,
4 => bb20,
5 => bb21,
6 => bb22,
340282366920938463463374607431218443783 => bb24,
_ => bb23
}
}
bb19 = {
(*_15) = Field::<i128>(Variant(_38, 1), 1);
_24 = _19 + _19;
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_15) = Field::<i128>(Variant(_38, 1), 1) - Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1395726440_i32,);
_17 = core::ptr::addr_of_mut!(_27.1);
Goto(bb18)
}
bb20 = {
(*_15) = _5 as i128;
_22.3 = _13 as u128;
_37 = _22;
RET = (*_7).0 as f32;
_37.3 = _22.3 << (*_15);
_19 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = !(-23002_i16);
_32 = [(*_15),(*_15),(*_15)];
(*_7).0 = 1077297398_i32;
_25 = !_22.0;
(*_7).0 = (-2114303570_i32) << _37.2;
_38 = Adt31::Variant1 { fld0: _22.2,fld1: (*_15) };
_36 = _37.1 ^ _22.1;
_37.0 = (*_15) != Field::<i128>(Variant(_38, 1), 1);
_23.2.0 = 36755929_i32;
_21 = Adt18::Variant0 { fld0: _13,fld1: _6,fld2: RET,fld3: (-9019824292057649920_i64),fld4: _36 };
(*_7) = (997083029_i32,);
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1379212333_i32,);
_27.2.fld1 = !_29;
place!(Field::<bool>(Variant(_21, 0), 0)) = !_13;
(*_7).0 = 867569912_i32;
_27.1 = [(*_15),(*_15),(*_15),(*_15)];
(*_7) = ((-25974040_i32),);
match _22.1 {
0 => bb1,
11420110750370059121 => bb17,
_ => bb9
}
}
bb21 = {
(*_15) = 83156254715927027844347920624297434466_i128 - (-33802226506267464929020141672038378710_i128);
_37.0 = _22.0;
(*_15) = _29 as i128;
(*_7) = (578451042_i32,);
(*_7).0 = -186283510_i32;
(*_7) = ((-1030533589_i32),);
match (*_7).0 {
0 => bb8,
1 => bb11,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607430737677867 => bb16,
_ => bb9
}
}
bb22 = {
(*_15) = 106370714971747527729210050468842227094_i128 ^ 136510301500946506869113798592401473570_i128;
_23.0 = core::ptr::addr_of!(_22.2);
_2 = -17060_i16;
_22.0 = (*_15) <= (*_15);
(*_7) = ((-2137255507_i32),);
(*_15) = (-35924949055382147099687331526410407334_i128) << (*_7).0;
(*_7) = (2131699147_i32,);
_14 = _22.3 as f64;
(*_7).0 = (-423577000_i32) & 396013884_i32;
(*_7).0 = (-2113224066_i32) >> (*_15);
_16 = _1;
(*_15) = RET as i128;
RET = _22.1 as f32;
(*_15) = 3436301280_u32 as i128;
(*_7) = (701649532_i32,);
(*_7) = ((-1317669587_i32),);
_23.1 = core::ptr::addr_of!(_21);
Goto(bb15)
}
bb23 = {
_3 = 54714893162870946359108458719919505438_u128 + 280387524294774607723833470750177279068_u128;
_3 = 143152097835212534716205718266683601466_u128;
_4 = (-101_i8) << _2;
_3 = 42055477115447914480035770463736302651_u128 * 164849574325220165108704052839273618213_u128;
_1 = 197_u8 as f64;
_10 = [2313300491_u32,3975299707_u32,1669221752_u32,2231114458_u32,1868097702_u32,3650035845_u32];
_2 = !(-32279_i16);
_9 = 14396747976305295854147961506839774718_i128 as isize;
_2 = (-23656_i16);
_3 = 64916685946502488940412461221276533191_u128 | 98254679175856252438426778747788271146_u128;
_10 = [1789989244_u32,3144166240_u32,923089800_u32,617268776_u32,2025313690_u32,392748321_u32];
_5 = 7035_u16 & 24409_u16;
_6 = 2593019312795319122_u64 as usize;
_5 = !15258_u16;
_10 = [3605069207_u32,4231625370_u32,3470917478_u32,2101896596_u32,3354383159_u32,1056233173_u32];
RET = _2 as f32;
Goto(bb4)
}
bb24 = {
(*_15) = _24 as i128;
_27.2.fld0 = Adt23::Variant1 { fld0: Move(_23.1),fld1: '\u{26eab}',fld2: _19,fld3: Field::<i8>(Variant((*_42), 1), 0),fld4: _3,fld5: (*_15) };
place!(Field::<i8>(Variant((*_42), 1), 0)) = _4 + _4;
place!(Field::<i128>(Variant(_27.2.fld0, 1), 5)) = (*_15);
_6 = Field::<usize>(Variant(_21, 0), 1) | Field::<usize>(Variant(_21, 0), 1);
_45.1.fld0.fld1 = _27.2.fld1;
_33 = '\u{509eb}';
_50 = [_33,_33,_33];
(*_17) = [(*_15),(*_15),(*_15),(*_15)];
_23.2.0 = 349774787_i32 | (-1174275050_i32);
(*_17) = [Field::<i128>(Variant((*_42), 1), 1),(*_15),(*_15),(*_15)];
(*_31) = 7323339178124831137_i64 + (-7272051704313199591_i64);
_49.fld1 = _29;
_37.1 = _3 as u64;
_37.2 = _4 * Field::<i8>(Variant((*_42), 1), 0);
(*_31) = -1577184619551063477_i64;
(*_15) = -Field::<i128>(Variant(_27.2.fld0, 1), 5);
(*_15) = Field::<i128>(Variant((*_42), 1), 1);
_45.0 = [_33,_33,_33,_33];
(*_7).0 = -44722636_i32;
place!(Field::<i8>(Variant((*_42), 1), 0)) = (*_7).0 as i8;
place!(Field::<*const Adt18>(Variant(_27.2.fld0, 1), 0)) = core::ptr::addr_of!(_18);
(*_7).0 = !985998522_i32;
_45.1.fld1 = core::ptr::addr_of_mut!(_37.0);
place!(Field::<i128>(Variant((*_42), 1), 1)) = (*_15) - (*_15);
Call((*_31) = fn19(Move((*_42)), Move(_23.3), Move(_15), Move(Field::<*const Adt18>(Variant(_27.2.fld0, 1), 0))), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_24 = RET as isize;
(*_7).0 = (-1337492515_i32) & 1651586540_i32;
(*_31) = 3597150334006039692_i64 + (-8898565401487372231_i64);
_29 = _49.fld1 >> _6;
_18 = _21;
_44 = _29 + _29;
_23.2.0 = 660942149_i32 & (-1941400833_i32);
(*_7) = (1360668554_i32,);
_13 = (*_31) != (*_31);
(*_7) = ((-740932872_i32),);
(*_42) = Adt31::Variant0 { fld0: Field::<u128>(Variant(_27.2.fld0, 1), 4),fld1: (*_7),fld2: _36,fld3: _37.2,fld4: _2 };
(*_7).0 = _24 as i32;
place!(Field::<(i32,)>(Variant((*_42), 0), 1)) = (_23.2.0,);
_36 = !Field::<u64>(Variant((*_42), 0), 2);
place!(Field::<(i32,)>(Variant((*_42), 0), 1)).0 = (*_7).0 + (*_7).0;
_46 = _6 as f64;
_31 = core::ptr::addr_of_mut!((*_31));
_45.1.fld2 = [Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant(_38, 0), 0),Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant((*_42), 0), 0)];
_57.0.fld0.fld0 = Adt23::Variant0 { fld0: _22.0,fld1: Field::<i128>(Variant(_27.2.fld0, 1), 5),fld2: _18,fld3: Field::<u64>(Variant((*_42), 0), 2),fld4: Field::<f32>(Variant(_18, 0), 2) };
place!(Field::<(i32,)>(Variant((*_42), 0), 1)) = (*_7);
match Field::<u64>(Variant(_18, 0), 4) {
0 => bb26,
1 => bb27,
11420110750370059121 => bb29,
_ => bb28
}
}
bb26 = {
_4 = _1 as i8;
_2 = (-30369_i16) - (-8602_i16);
RET = 12865743887420358116_usize as f32;
_4 = 79_i8;
_3 = !201755675250628654238996545817975430160_u128;
RET = _4 as f32;
_4 = (-4_i8);
_5 = 9201_u16 + 18365_u16;
Goto(bb2)
}
bb27 = {
(*_15) = Field::<i128>(Variant(_38, 1), 1);
_24 = _19 + _19;
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_15) = Field::<i128>(Variant(_38, 1), 1) - Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1395726440_i32,);
_17 = core::ptr::addr_of_mut!(_27.1);
Goto(bb18)
}
bb28 = {
_4 = _1 as i8;
_2 = (-30369_i16) - (-8602_i16);
RET = 12865743887420358116_usize as f32;
_4 = 79_i8;
_3 = !201755675250628654238996545817975430160_u128;
RET = _4 as f32;
_4 = (-4_i8);
_5 = 9201_u16 + 18365_u16;
Goto(bb2)
}
bb29 = {
_41 = core::ptr::addr_of_mut!(_57.1);
(*_17) = [Field::<i128>(Variant(_27.2.fld0, 1), 5),Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_27.2.fld0, 1), 5)];
(*_17) = [Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_27.2.fld0, 1), 5),Field::<i128>(Variant(_27.2.fld0, 1), 5)];
place!(Field::<(i32,)>(Variant((*_42), 0), 1)) = ((*_7).0,);
_35 = Field::<u128>(Variant((*_42), 0), 0);
place!(Field::<i128>(Variant(_27.2.fld0, 1), 5)) = Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1) << Field::<i8>(Variant(_38, 0), 3);
(*_41) = core::ptr::addr_of!(_13);
place!(Field::<(i32,)>(Variant((*_42), 0), 1)) = ((*_7).0,);
place!(Field::<i16>(Variant((*_42), 0), 4)) = _2;
(*_42) = Adt31::Variant1 { fld0: Field::<i8>(Variant(_27.2.fld0, 1), 3),fld1: Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1) };
(*_31) = Field::<i64>(Variant(Field::<Adt18>(Variant(_57.0.fld0.fld0, 0), 2), 0), 3) & Field::<i64>(Variant(_18, 0), 3);
(*_41) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_57.0.fld0.fld0, 0), 0)));
(*_42) = Adt31::Variant0 { fld0: _22.3,fld1: (*_7),fld2: Field::<u64>(Variant(_21, 0), 4),fld3: _4,fld4: _2 };
place!(Field::<i8>(Variant((*_42), 0), 3)) = _4 + _4;
place!(Field::<(i32,)>(Variant((*_42), 0), 1)).0 = (*_7).0 >> Field::<u128>(Variant((*_42), 0), 0);
_41 = core::ptr::addr_of_mut!((*_41));
place!(Field::<(i32,)>(Variant((*_42), 0), 1)).0 = (*_7).0 & (*_7).0;
_40 = core::ptr::addr_of!(_52);
(*_41) = core::ptr::addr_of!(_25);
place!(Field::<i16>(Variant((*_42), 0), 4)) = _2;
_45.1.fld0.fld0 = Move(_57.0.fld0.fld0);
place!(Field::<u128>(Variant(_38, 0), 0)) = _35;
place!(Field::<u64>(Variant((*_42), 0), 2)) = Field::<u64>(Variant(_45.1.fld0.fld0, 0), 3);
_45.1.fld0.fld1 = _22.2 as u8;
(*_31) = Field::<i64>(Variant(_18, 0), 3);
(*_7) = (Field::<(i32,)>(Variant((*_42), 0), 1).0,);
Goto(bb30)
}
bb30 = {
_45.1.fld2 = [Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant((*_42), 0), 0)];
place!(Field::<isize>(Variant(_27.2.fld0, 1), 2)) = _24 << Field::<i8>(Variant((*_42), 0), 3);
place!(Field::<*const Adt18>(Variant(_27.2.fld0, 1), 0)) = core::ptr::addr_of!(place!(Field::<Adt18>(Variant(_45.1.fld0.fld0, 0), 2)));
(*_42) = Adt31::Variant1 { fld0: _37.2,fld1: Field::<i128>(Variant(_45.1.fld0.fld0, 0), 1) };
(*_7).0 = _33 as i32;
place!(Field::<i8>(Variant((*_42), 1), 0)) = _37.2 - Field::<i8>(Variant(_27.2.fld0, 1), 3);
Goto(bb31)
}
bb31 = {
_22.2 = Field::<i8>(Variant((*_42), 1), 0) ^ Field::<i8>(Variant((*_42), 1), 0);
_49.fld0 = Adt23::Variant1 { fld0: Move(Field::<*const Adt18>(Variant(_27.2.fld0, 1), 0)),fld1: _33,fld2: Field::<isize>(Variant(_27.2.fld0, 1), 2),fld3: Field::<i8>(Variant((*_42), 1), 0),fld4: _35,fld5: Field::<i128>(Variant((*_42), 1), 1) };
place!(Field::<i128>(Variant((*_42), 1), 1)) = Field::<i128>(Variant(_27.2.fld0, 1), 5) | Field::<i128>(Variant(_27.2.fld0, 1), 5);
_63 = &mut _45.1;
place!(Field::<i8>(Variant((*_42), 1), 0)) = -Field::<i8>(Variant(_27.2.fld0, 1), 3);
_34 = core::ptr::addr_of!(_60.fld3);
place!(Field::<Adt18>(Variant((*_63).fld0.fld0, 0), 2)) = Adt18::Variant0 { fld0: _13,fld1: _6,fld2: Field::<f32>(Variant((*_63).fld0.fld0, 0), 4),fld3: (*_31),fld4: Field::<u64>(Variant((*_63).fld0.fld0, 0), 3) };
(*_17) = [Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_63).fld0.fld0, 0), 1)];
(*_63).fld2 = [_22.3,Field::<u128>(Variant(_49.fld0, 1), 4),_37.3,_37.3];
place!(Field::<Adt18>(Variant((*_63).fld0.fld0, 0), 2)) = _21;
(*_63).fld0.fld1 = Field::<usize>(Variant(Field::<Adt18>(Variant((*_63).fld0.fld0, 0), 2), 0), 1) as u8;
(*_63).fld2 = [_37.3,_22.3,Field::<u128>(Variant(_27.2.fld0, 1), 4),_37.3];
(*_17) = [Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_63).fld0.fld0, 0), 1),Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant(_38, 1), 1)];
_3 = Field::<i128>(Variant((*_42), 1), 1) as u128;
(*_7).0 = (-583659047_i32);
place!(Field::<bool>(Variant(place!(Field::<Adt18>(Variant((*_63).fld0.fld0, 0), 2)), 0), 0)) = !Field::<bool>(Variant((*_63).fld0.fld0, 0), 0);
_69.3 = &mut place!(Field::<isize>(Variant(_27.2.fld0, 1), 2));
(*_7).0 = (-1431181181_i32) << Field::<i128>(Variant((*_42), 1), 1);
(*_17) = [Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1)];
place!(Field::<i8>(Variant((*_42), 1), 0)) = !Field::<i8>(Variant(_49.fld0, 1), 3);
_57.0.fld0 = Adt44 { fld0: Move((*_63).fld0.fld0),fld1: (*_63).fld0.fld1 };
_70.3 = _19 as u128;
(*_41) = core::ptr::addr_of!(_37.0);
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_57.0.fld0.fld0, 0), 2)), 0), 3)) = !(*_31);
(*_63).fld1 = core::ptr::addr_of_mut!(_37.0);
(*_31) = _37.3 as i64;
_69.0 = Adt23::Variant0 { fld0: Field::<bool>(Variant(_18, 0), 0),fld1: Field::<i128>(Variant((*_42), 1), 1),fld2: _21,fld3: _22.1,fld4: Field::<f32>(Variant(_21, 0), 2) };
(*_17) = [Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1)];
match Field::<u64>(Variant(_18, 0), 4) {
0 => bb32,
1 => bb33,
2 => bb34,
3 => bb35,
11420110750370059121 => bb37,
_ => bb36
}
}
bb32 = {
(*_15) = _5 as i128;
_22.3 = _13 as u128;
_37 = _22;
RET = (*_7).0 as f32;
_37.3 = _22.3 << (*_15);
_19 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = !(-23002_i16);
_32 = [(*_15),(*_15),(*_15)];
(*_7).0 = 1077297398_i32;
_25 = !_22.0;
(*_7).0 = (-2114303570_i32) << _37.2;
_38 = Adt31::Variant1 { fld0: _22.2,fld1: (*_15) };
_36 = _37.1 ^ _22.1;
_37.0 = (*_15) != Field::<i128>(Variant(_38, 1), 1);
_23.2.0 = 36755929_i32;
_21 = Adt18::Variant0 { fld0: _13,fld1: _6,fld2: RET,fld3: (-9019824292057649920_i64),fld4: _36 };
(*_7) = (997083029_i32,);
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1379212333_i32,);
_27.2.fld1 = !_29;
place!(Field::<bool>(Variant(_21, 0), 0)) = !_13;
(*_7).0 = 867569912_i32;
_27.1 = [(*_15),(*_15),(*_15),(*_15)];
(*_7) = ((-25974040_i32),);
match _22.1 {
0 => bb1,
11420110750370059121 => bb17,
_ => bb9
}
}
bb33 = {
_41 = core::ptr::addr_of_mut!(_57.1);
(*_17) = [Field::<i128>(Variant(_27.2.fld0, 1), 5),Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_27.2.fld0, 1), 5)];
(*_17) = [Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant(_27.2.fld0, 1), 5),Field::<i128>(Variant(_27.2.fld0, 1), 5)];
place!(Field::<(i32,)>(Variant((*_42), 0), 1)) = ((*_7).0,);
_35 = Field::<u128>(Variant((*_42), 0), 0);
place!(Field::<i128>(Variant(_27.2.fld0, 1), 5)) = Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1) << Field::<i8>(Variant(_38, 0), 3);
(*_41) = core::ptr::addr_of!(_13);
place!(Field::<(i32,)>(Variant((*_42), 0), 1)) = ((*_7).0,);
place!(Field::<i16>(Variant((*_42), 0), 4)) = _2;
(*_42) = Adt31::Variant1 { fld0: Field::<i8>(Variant(_27.2.fld0, 1), 3),fld1: Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1) };
(*_31) = Field::<i64>(Variant(Field::<Adt18>(Variant(_57.0.fld0.fld0, 0), 2), 0), 3) & Field::<i64>(Variant(_18, 0), 3);
(*_41) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_57.0.fld0.fld0, 0), 0)));
(*_42) = Adt31::Variant0 { fld0: _22.3,fld1: (*_7),fld2: Field::<u64>(Variant(_21, 0), 4),fld3: _4,fld4: _2 };
place!(Field::<i8>(Variant((*_42), 0), 3)) = _4 + _4;
place!(Field::<(i32,)>(Variant((*_42), 0), 1)).0 = (*_7).0 >> Field::<u128>(Variant((*_42), 0), 0);
_41 = core::ptr::addr_of_mut!((*_41));
place!(Field::<(i32,)>(Variant((*_42), 0), 1)).0 = (*_7).0 & (*_7).0;
_40 = core::ptr::addr_of!(_52);
(*_41) = core::ptr::addr_of!(_25);
place!(Field::<i16>(Variant((*_42), 0), 4)) = _2;
_45.1.fld0.fld0 = Move(_57.0.fld0.fld0);
place!(Field::<u128>(Variant(_38, 0), 0)) = _35;
place!(Field::<u64>(Variant((*_42), 0), 2)) = Field::<u64>(Variant(_45.1.fld0.fld0, 0), 3);
_45.1.fld0.fld1 = _22.2 as u8;
(*_31) = Field::<i64>(Variant(_18, 0), 3);
(*_7) = (Field::<(i32,)>(Variant((*_42), 0), 1).0,);
Goto(bb30)
}
bb34 = {
_24 = RET as isize;
(*_7).0 = (-1337492515_i32) & 1651586540_i32;
(*_31) = 3597150334006039692_i64 + (-8898565401487372231_i64);
_29 = _49.fld1 >> _6;
_18 = _21;
_44 = _29 + _29;
_23.2.0 = 660942149_i32 & (-1941400833_i32);
(*_7) = (1360668554_i32,);
_13 = (*_31) != (*_31);
(*_7) = ((-740932872_i32),);
(*_42) = Adt31::Variant0 { fld0: Field::<u128>(Variant(_27.2.fld0, 1), 4),fld1: (*_7),fld2: _36,fld3: _37.2,fld4: _2 };
(*_7).0 = _24 as i32;
place!(Field::<(i32,)>(Variant((*_42), 0), 1)) = (_23.2.0,);
_36 = !Field::<u64>(Variant((*_42), 0), 2);
place!(Field::<(i32,)>(Variant((*_42), 0), 1)).0 = (*_7).0 + (*_7).0;
_46 = _6 as f64;
_31 = core::ptr::addr_of_mut!((*_31));
_45.1.fld2 = [Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant(_38, 0), 0),Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant((*_42), 0), 0)];
_57.0.fld0.fld0 = Adt23::Variant0 { fld0: _22.0,fld1: Field::<i128>(Variant(_27.2.fld0, 1), 5),fld2: _18,fld3: Field::<u64>(Variant((*_42), 0), 2),fld4: Field::<f32>(Variant(_18, 0), 2) };
place!(Field::<(i32,)>(Variant((*_42), 0), 1)) = (*_7);
match Field::<u64>(Variant(_18, 0), 4) {
0 => bb26,
1 => bb27,
11420110750370059121 => bb29,
_ => bb28
}
}
bb35 = {
_3 = 54714893162870946359108458719919505438_u128 + 280387524294774607723833470750177279068_u128;
_3 = 143152097835212534716205718266683601466_u128;
_4 = (-101_i8) << _2;
_3 = 42055477115447914480035770463736302651_u128 * 164849574325220165108704052839273618213_u128;
_1 = 197_u8 as f64;
_10 = [2313300491_u32,3975299707_u32,1669221752_u32,2231114458_u32,1868097702_u32,3650035845_u32];
_2 = !(-32279_i16);
_9 = 14396747976305295854147961506839774718_i128 as isize;
_2 = (-23656_i16);
_3 = 64916685946502488940412461221276533191_u128 | 98254679175856252438426778747788271146_u128;
_10 = [1789989244_u32,3144166240_u32,923089800_u32,617268776_u32,2025313690_u32,392748321_u32];
_5 = 7035_u16 & 24409_u16;
_6 = 2593019312795319122_u64 as usize;
_5 = !15258_u16;
_10 = [3605069207_u32,4231625370_u32,3470917478_u32,2101896596_u32,3354383159_u32,1056233173_u32];
RET = _2 as f32;
Goto(bb4)
}
bb36 = {
(*_15) = Field::<i128>(Variant(_38, 1), 1);
_24 = _19 + _19;
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_15) = Field::<i128>(Variant(_38, 1), 1) - Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1395726440_i32,);
_17 = core::ptr::addr_of_mut!(_27.1);
Goto(bb18)
}
bb37 = {
_22.3 = 2898219796_u32 as u128;
_73 = Field::<f32>(Variant(Field::<Adt18>(Variant(_69.0, 0), 2), 0), 2) as u16;
_70.2 = Field::<i8>(Variant((*_42), 1), 0);
(*_63).fld2 = [Field::<u128>(Variant(_49.fld0, 1), 4),_3,_3,_3];
(*_34) = core::ptr::addr_of_mut!(_73);
_54 = Adt44 { fld0: Move(_69.0),fld1: _49.fld1 };
place!(Field::<u64>(Variant(_18, 0), 4)) = _46 as u64;
(*_63).fld0.fld1 = _13 as u8;
(*_34) = core::ptr::addr_of_mut!(_5);
_69.1 = _73 << (*_7).0;
_64.fld1 = -_1;
_57.0 = Adt57 { fld0: Move(_54),fld1: Move((*_63).fld1),fld2: (*_63).fld2 };
_69.0 = Adt23::Variant1 { fld0: Move(Field::<*const Adt18>(Variant(_49.fld0, 1), 0)),fld1: _33,fld2: Field::<isize>(Variant(_49.fld0, 1), 2),fld3: _22.2,fld4: _3,fld5: Field::<i128>(Variant((*_42), 1), 1) };
(*_41) = core::ptr::addr_of!(_22.0);
(*_42) = Adt31::Variant1 { fld0: Field::<i8>(Variant(_69.0, 1), 3),fld1: Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1) };
place!(Field::<i128>(Variant((*_42), 1), 1)) = _44 as i128;
(*_7).0 = (-1153348897_i32);
_40 = core::ptr::addr_of!((*_40));
(*_63).fld2 = _57.0.fld2;
_64.fld2 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_18, 0), 0)));
place!(Field::<i128>(Variant((*_42), 1), 1)) = _22.0 as i128;
(*_34) = core::ptr::addr_of_mut!(_69.1);
match (*_7).0 {
0 => bb35,
1 => bb33,
2 => bb29,
3 => bb9,
340282366920938463463374607430614862559 => bb38,
_ => bb28
}
}
bb38 = {
match _22.1 {
0 => bb25,
1 => bb3,
2 => bb39,
3 => bb40,
4 => bb41,
5 => bb42,
6 => bb43,
11420110750370059121 => bb45,
_ => bb44
}
}
bb39 = {
_45.1.fld2 = [Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant((*_42), 0), 0),Field::<u128>(Variant((*_42), 0), 0)];
place!(Field::<isize>(Variant(_27.2.fld0, 1), 2)) = _24 << Field::<i8>(Variant((*_42), 0), 3);
place!(Field::<*const Adt18>(Variant(_27.2.fld0, 1), 0)) = core::ptr::addr_of!(place!(Field::<Adt18>(Variant(_45.1.fld0.fld0, 0), 2)));
(*_42) = Adt31::Variant1 { fld0: _37.2,fld1: Field::<i128>(Variant(_45.1.fld0.fld0, 0), 1) };
(*_7).0 = _33 as i32;
place!(Field::<i8>(Variant((*_42), 1), 0)) = _37.2 - Field::<i8>(Variant(_27.2.fld0, 1), 3);
Goto(bb31)
}
bb40 = {
(*_15) = _5 as i128;
_22.3 = _13 as u128;
_37 = _22;
RET = (*_7).0 as f32;
_37.3 = _22.3 << (*_15);
_19 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = !(-23002_i16);
_32 = [(*_15),(*_15),(*_15)];
(*_7).0 = 1077297398_i32;
_25 = !_22.0;
(*_7).0 = (-2114303570_i32) << _37.2;
_38 = Adt31::Variant1 { fld0: _22.2,fld1: (*_15) };
_36 = _37.1 ^ _22.1;
_37.0 = (*_15) != Field::<i128>(Variant(_38, 1), 1);
_23.2.0 = 36755929_i32;
_21 = Adt18::Variant0 { fld0: _13,fld1: _6,fld2: RET,fld3: (-9019824292057649920_i64),fld4: _36 };
(*_7) = (997083029_i32,);
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1379212333_i32,);
_27.2.fld1 = !_29;
place!(Field::<bool>(Variant(_21, 0), 0)) = !_13;
(*_7).0 = 867569912_i32;
_27.1 = [(*_15),(*_15),(*_15),(*_15)];
(*_7) = ((-25974040_i32),);
match _22.1 {
0 => bb1,
11420110750370059121 => bb17,
_ => bb9
}
}
bb41 = {
(*_15) = _5 as i128;
_22.3 = _13 as u128;
_37 = _22;
RET = (*_7).0 as f32;
_37.3 = _22.3 << (*_15);
_19 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = !(-23002_i16);
_32 = [(*_15),(*_15),(*_15)];
(*_7).0 = 1077297398_i32;
_25 = !_22.0;
(*_7).0 = (-2114303570_i32) << _37.2;
_38 = Adt31::Variant1 { fld0: _22.2,fld1: (*_15) };
_36 = _37.1 ^ _22.1;
_37.0 = (*_15) != Field::<i128>(Variant(_38, 1), 1);
_23.2.0 = 36755929_i32;
_21 = Adt18::Variant0 { fld0: _13,fld1: _6,fld2: RET,fld3: (-9019824292057649920_i64),fld4: _36 };
(*_7) = (997083029_i32,);
(*_15) = Field::<i128>(Variant(_38, 1), 1) * Field::<i128>(Variant(_38, 1), 1);
(*_7) = (1379212333_i32,);
_27.2.fld1 = !_29;
place!(Field::<bool>(Variant(_21, 0), 0)) = !_13;
(*_7).0 = 867569912_i32;
_27.1 = [(*_15),(*_15),(*_15),(*_15)];
(*_7) = ((-25974040_i32),);
match _22.1 {
0 => bb1,
11420110750370059121 => bb17,
_ => bb9
}
}
bb42 = {
(*_15) = Field::<i128>(Variant(_38, 1), 1);
_29 = !_27.2.fld1;
_21 = Adt18::Variant0 { fld0: _37.0,fld1: _6,fld2: RET,fld3: (-6909065149842076206_i64),fld4: _22.1 };
(*_15) = Field::<i128>(Variant(_38, 1), 1) & Field::<i128>(Variant(_38, 1), 1);
(*_7) = (412583731_i32,);
(*_7) = (1225543757_i32,);
(*_7).0 = !(-389256444_i32);
_31 = core::ptr::addr_of_mut!(place!(Field::<i64>(Variant(_21, 0), 3)));
_23.2.0 = (-549767673_i32);
(*_15) = Field::<i128>(Variant(_38, 1), 1) - Field::<i128>(Variant(_38, 1), 1);
(*_15) = Field::<i128>(Variant(_38, 1), 1) | Field::<i128>(Variant(_38, 1), 1);
place!(Field::<f32>(Variant(_21, 0), 2)) = RET + RET;
(*_15) = Field::<i128>(Variant(_38, 1), 1) - Field::<i128>(Variant(_38, 1), 1);
(*_31) = (-8029704722238015524_i64) ^ 4992709324622764737_i64;
(*_15) = !Field::<i128>(Variant(_38, 1), 1);
_42 = core::ptr::addr_of_mut!(_38);
(*_31) = 4107215452183805126_i64 << _36;
_36 = _37.1 | _37.1;
match (*_7).0 {
0 => bb6,
1 => bb16,
2 => bb7,
3 => bb19,
4 => bb20,
5 => bb21,
6 => bb22,
340282366920938463463374607431218443783 => bb24,
_ => bb23
}
}
bb43 = {
(*_15) = 83156254715927027844347920624297434466_i128 - (-33802226506267464929020141672038378710_i128);
_37.0 = _22.0;
(*_15) = _29 as i128;
(*_7) = (578451042_i32,);
(*_7).0 = -186283510_i32;
(*_7) = ((-1030533589_i32),);
match (*_7).0 {
0 => bb8,
1 => bb11,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607430737677867 => bb16,
_ => bb9
}
}
bb44 = {
(*_15) = 106370714971747527729210050468842227094_i128 ^ 136510301500946506869113798592401473570_i128;
_23.0 = core::ptr::addr_of!(_22.2);
_2 = -17060_i16;
_22.0 = (*_15) <= (*_15);
(*_7) = ((-2137255507_i32),);
(*_15) = (-35924949055382147099687331526410407334_i128) << (*_7).0;
(*_7) = (2131699147_i32,);
_14 = _22.3 as f64;
(*_7).0 = (-423577000_i32) & 396013884_i32;
(*_7).0 = (-2113224066_i32) >> (*_15);
_16 = _1;
(*_15) = RET as i128;
RET = _22.1 as f32;
(*_15) = 3436301280_u32 as i128;
(*_7) = (701649532_i32,);
(*_7) = ((-1317669587_i32),);
_23.1 = core::ptr::addr_of!(_21);
Goto(bb15)
}
bb45 = {
(*_34) = core::ptr::addr_of_mut!(_80);
_60.fld0 = core::ptr::addr_of_mut!(_70.0);
_64.fld6 = (*_31);
_22.0 = Field::<bool>(Variant(_21, 0), 0);
place!(Field::<i128>(Variant(_69.0, 1), 5)) = -Field::<i128>(Variant((*_42), 1), 1);
(*_7) = (1959675343_i32,);
(*_41) = core::ptr::addr_of!(_70.0);
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_57.0.fld0.fld0, 0), 2)), 0), 3)) = -_64.fld6;
_70.3 = Field::<u128>(Variant(_69.0, 1), 4);
(*_7).0 = -532263330_i32;
(*_7).0 = (-310979451_i32);
_39 = core::ptr::addr_of_mut!((*_40));
place!(Field::<i8>(Variant((*_42), 1), 0)) = Field::<i8>(Variant(_49.fld0, 1), 3) >> _69.1;
_71 = (*_31) as isize;
_60.fld1 = (*_63).fld0.fld1 as f64;
_60.fld6 = _70.3 as i64;
(*_41) = core::ptr::addr_of!(_22.0);
place!(Field::<i64>(Variant(_21, 0), 3)) = _69.1 as i64;
_37.3 = _3 - Field::<u128>(Variant(_69.0, 1), 4);
(*_7) = (947686429_i32,);
match Field::<u64>(Variant(_57.0.fld0.fld0, 0), 3) {
11420110750370059121 => bb46,
_ => bb43
}
}
bb46 = {
(*_41) = Move(_64.fld2);
_79 = [3096167022_u32,2413050753_u32,2262133346_u32,1246229548_u32,2767037847_u32,714000369_u32];
(*_63).fld0.fld1 = _46 as u8;
Goto(bb47)
}
bb47 = {
_80 = _37.1 as u16;
(*_63).fld2 = _57.0.fld2;
(*_17) = [Field::<i128>(Variant(_49.fld0, 1), 5),Field::<i128>(Variant(_57.0.fld0.fld0, 0), 1),Field::<i128>(Variant((*_42), 1), 1),Field::<i128>(Variant((*_42), 1), 1)];
place!(Field::<u128>(Variant(_49.fld0, 1), 4)) = _37.3 | _3;
(*_7).0 = (-279565689_i32);
(*_34) = core::ptr::addr_of_mut!(_69.1);
_23.0 = core::ptr::addr_of!(place!(Field::<i8>(Variant((*_42), 1), 0)));
(*_63).fld2 = _57.0.fld2;
_75 = (*_17);
_60.fld4 = Field::<usize>(Variant(_21, 0), 1);
place!(Field::<u64>(Variant(_21, 0), 4)) = _22.1 | _36;
_54 = Adt44 { fld0: Move(_57.0.fld0.fld0),fld1: (*_63).fld0.fld1 };
_56 = RET + Field::<f32>(Variant(Field::<Adt18>(Variant(_54.fld0, 0), 2), 0), 2);
(*_31) = -Field::<i64>(Variant(_18, 0), 3);
_39 = core::ptr::addr_of_mut!((*_40));
_23.3 = &mut _19;
Goto(bb48)
}
bb48 = {
_78 = &mut place!(Field::<i128>(Variant((*_42), 1), 1));
(*_41) = core::ptr::addr_of!(place!(Field::<bool>(Variant(_18, 0), 0)));
_22.2 = Field::<char>(Variant(_49.fld0, 1), 1) as i8;
(*_63).fld2 = _57.0.fld2;
_3 = _70.3 & Field::<u128>(Variant(_49.fld0, 1), 4);
_64.fld6 = _60.fld6 + (*_31);
place!(Field::<bool>(Variant(_54.fld0, 0), 0)) = _22.0;
(*_31) = _60.fld6 & Field::<i64>(Variant(Field::<Adt18>(Variant(_54.fld0, 0), 2), 0), 3);
(*_7) = ((-1089415828_i32),);
(*_41) = core::ptr::addr_of!(_25);
(*_63).fld0.fld1 = _29;
(*_34) = core::ptr::addr_of_mut!(_69.1);
Goto(bb49)
}
bb49 = {
place!(Field::<isize>(Variant(_49.fld0, 1), 2)) = !Field::<isize>(Variant(_69.0, 1), 2);
(*_17) = _75;
(*_78) = Field::<u64>(Variant(_21, 0), 4) as i128;
_10 = [3213040965_u32,324569729_u32,4079022565_u32,2324325689_u32,4127082618_u32,3106334333_u32];
(*_7).0 = 1969115294_i32 & (-847478669_i32);
_58 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(_69.0, 1), 2)));
(*_17) = [Field::<i128>(Variant(_54.fld0, 0), 1),(*_78),(*_78),(*_78)];
_57.0.fld0.fld0 = Move(_69.0);
(*_7).0 = (-1514376228_i32);
place!(Field::<i8>(Variant(_57.0.fld0.fld0, 1), 3)) = Field::<i8>(Variant((*_42), 1), 0);
(*_17) = [(*_78),(*_78),(*_78),(*_78)];
(*_7) = ((-1784141552_i32),);
place!(Field::<*const Adt18>(Variant(_57.0.fld0.fld0, 1), 0)) = core::ptr::addr_of!(_21);
_17 = core::ptr::addr_of_mut!((*_17));
(*_17) = [(*_78),(*_78),(*_78),(*_78)];
_49.fld0 = Move(_57.0.fld0.fld0);
place!(Field::<i8>(Variant((*_42), 1), 0)) = Field::<i8>(Variant(_49.fld0, 1), 3) >> (*_78);
(*_7).0 = 338501929_i32;
_55 = Field::<i8>(Variant((*_42), 1), 0);
_28 = &_22;
(*_58) = !Field::<isize>(Variant(_49.fld0, 1), 2);
(*_58) = _24 - Field::<isize>(Variant(_49.fld0, 1), 2);
_49.fld0 = Adt23::Variant0 { fld0: (*_28).0,fld1: (*_78),fld2: Field::<Adt18>(Variant(_54.fld0, 0), 2),fld3: (*_28).1,fld4: _56 };
(*_17) = [(*_78),(*_78),(*_78),(*_78)];
Goto(bb50)
}
bb50 = {
Call(_93 = dump_var(14_usize, 2_usize, Move(_2), 37_usize, Move(_37), 32_usize, Move(_32), 33_usize, Move(_33)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_93 = dump_var(14_usize, 6_usize, Move(_6), 79_usize, Move(_79), 44_usize, Move(_44), 11_usize, Move(_11)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_93 = dump_var(14_usize, 50_usize, Move(_50), 35_usize, Move(_35), 80_usize, Move(_80), 24_usize, Move(_24)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_93 = dump_var(14_usize, 29_usize, Move(_29), 94_usize, _94, 94_usize, _94, 94_usize, _94), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: u128,mut _2: i16,mut _3: f64,mut _4: u128,mut _5: f64,mut _6: i16,mut _7: u128) -> f32 {
mir! {
type RET = f32;
let _8: isize;
let _9: [i128; 4];
let _10: isize;
let _11: *mut &'static mut &'static usize;
let _12: [u128; 4];
let _13: i128;
let _14: isize;
let _15: [char; 3];
let _16: f64;
let _17: i32;
let _18: [i16; 3];
let _19: *const *mut u16;
let _20: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _21: [usize; 3];
let _22: &'static mut Adt57;
let _23: isize;
let _24: &'static i32;
let _25: &'static [char; 3];
let _26: (i32,);
let _27: [i32; 6];
let _28: *mut &'static mut *mut u16;
let _29: &'static *const Adt18;
let _30: i32;
let _31: &'static i32;
let _32: u128;
let _33: u32;
let _34: f32;
let _35: i8;
let _36: [u32; 6];
let _37: i32;
let _38: *mut *const bool;
let _39: [i16; 3];
let _40: Adt23;
let _41: u8;
let _42: f32;
let _43: (i16, *mut [usize; 3], u16);
let _44: i128;
let _45: f32;
let _46: &'static Adt38;
let _47: ([char; 4], Adt57);
let _48: isize;
let _49: usize;
let _50: f64;
let _51: [u128; 4];
let _52: &'static Adt57;
let _53: &'static mut *mut u16;
let _54: bool;
let _55: isize;
let _56: isize;
let _57: bool;
let _58: char;
let _59: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _60: f32;
let _61: f64;
let _62: [usize; 3];
let _63: *mut f32;
let _64: f32;
let _65: f64;
let _66: bool;
let _67: i64;
let _68: *mut isize;
let _69: (Adt57, *const bool);
let _70: u32;
let _71: Adt31;
let _72: char;
let _73: isize;
let _74: &'static Adt57;
let _75: [u128; 4];
let _76: *mut u128;
let _77: &'static usize;
let _78: (Adt57, *const bool);
let _79: [i128; 4];
let _80: char;
let _81: isize;
let _82: bool;
let _83: i16;
let _84: [usize; 3];
let _85: ();
let _86: ();
{
_2 = !_6;
RET = 70_i8 as f32;
_4 = !_1;
_9 = [123013606942661997523568876402029079926_i128,85561148239422625444004673019574483873_i128,(-30928737015091392253514948247703386941_i128),71804885003201117158122133092731501828_i128];
_8 = -40_isize;
_8 = (-9223372036854775808_isize) >> _4;
_7 = !_1;
_8 = 9223372036854775807_isize;
RET = 138830319784318158036236158540631040314_i128 as f32;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
9223372036854775807 => bb8,
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
RET = _8 as f32;
_6 = _2 >> _1;
_10 = _8 ^ _8;
_10 = _8 | _8;
_4 = _7 << _1;
_4 = !_1;
RET = 2_usize as f32;
_3 = _5;
RET = 17842677583884656982_u64 as f32;
_10 = 23734_u16 as isize;
_3 = _5 * _5;
_9 = [(-13417046565081052217274709805414275425_i128),158984887617066885879945516762494178333_i128,52518081040726503864872520983075069561_i128,(-122378927271628839402591597660067721793_i128)];
_14 = !_8;
_2 = _7 as i16;
_9 = [(-72413516761937005494277787726862448791_i128),7725709006618466870855496001723597731_i128,140383114316295450169144215623155878087_i128,(-149583848947026478651027514551812860182_i128)];
_5 = _3 - _3;
_15 = ['\u{ae177}','\u{fde7c}','\u{1e313}'];
_4 = _3 as u128;
_9 = [(-8541386845955661665698813493119788325_i128),(-136202596922783256188378989345575190995_i128),(-98455709827672576906686766060792521135_i128),(-71113141403182230821538722404946579261_i128)];
_1 = 528626667_u32 as u128;
_12 = [_7,_7,_4,_7];
_1 = _4 << _10;
match _8 {
0 => bb1,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
9223372036854775807 => bb16,
_ => bb15
}
}
bb9 = {
Return()
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
Return()
}
bb15 = {
Return()
}
bb16 = {
_15 = ['\u{e7ec0}','\u{3bed5}','\u{ff9c}'];
_9 = [(-49450985719850509479602957194505636094_i128),133738361466871261332545516313438551757_i128,(-74319975748024389020152231124067509872_i128),137839194792128338486577800141514288272_i128];
_13 = 5090225318767304917_i64 as i128;
_18 = [_2,_6,_2];
_13 = 54151243153269469028525143603032425573_i128 & (-123888923844115901765603954021830236599_i128);
_17 = (-71_i8) as i32;
_2 = _17 as i16;
_20.2 = (_17,);
_5 = _3 - _3;
_3 = _5;
_17 = _20.2.0 + _20.2.0;
RET = 29_i8 as f32;
_16 = _3;
_13 = (-92431394133598384090047899679551132677_i128) >> _6;
RET = (-54_i8) as f32;
_14 = 14172336203039788461_usize as isize;
_5 = _3 + _16;
_10 = _8 | _14;
_18 = [_6,_2,_6];
_18 = [_6,_6,_6];
_15 = ['\u{dfa4c}','\u{101555}','\u{33c6e}'];
_3 = -_5;
_9 = [_13,_13,_13,_13];
Goto(bb17)
}
bb17 = {
_10 = _14;
_14 = _8 >> _7;
_3 = _16;
_23 = _14;
_15 = ['\u{aadaf}','\u{cb5ba}','\u{5233}'];
_25 = &_15;
_15 = ['\u{b2cae}','\u{3072}','\u{6547a}'];
_18 = [_6,_2,_6];
_18 = [_6,_2,_6];
_14 = (-60_i8) as isize;
_20.3 = &mut _10;
_24 = &_20.2.0;
_2 = _6 + _6;
_15 = ['\u{12b84}','\u{41edd}','\u{6ebb1}'];
_21 = [2_usize,4_usize,10600306578246952594_usize];
_2 = _6;
_17 = (*_24) & (*_24);
_26 = ((*_24),);
_27 = [(*_24),(*_24),(*_24),(*_24),(*_24),(*_24)];
RET = _13 as f32;
_9 = [_13,_13,_13,_13];
match _8 {
0 => bb9,
1 => bb13,
2 => bb3,
3 => bb11,
4 => bb5,
9223372036854775807 => bb19,
_ => bb18
}
}
bb18 = {
Return()
}
bb19 = {
_1 = !_4;
_1 = _4 << (*_24);
_17 = (*_24);
_23 = -_8;
_9 = [_13,_13,_13,_13];
_4 = _5 as u128;
_20.2.0 = 56644_u16 as i32;
_15 = ['\u{915c0}','\u{8209}','\u{e2edb}'];
_24 = &_17;
_8 = _6 as isize;
_17 = 77_i8 as i32;
_20.3 = &mut _23;
_4 = _7;
_31 = &_17;
_26.0 = _20.2.0 >> _13;
_8 = _14;
_9 = [_13,_13,_13,_13];
_4 = false as u128;
_5 = (-4548410076255157255_i64) as f64;
_16 = _3;
_24 = &_26.0;
_1 = _13 as u128;
_14 = 3759147018_u32 as isize;
_12 = [_1,_7,_7,_1];
_3 = _16 * _16;
_31 = &(*_24);
Call(_12 = fn16(Move(_25), _21, Move(_31), (*_24), Move(_20.3), Move(_24), _20.2.0, (*_24)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
RET = _13 as f32;
Goto(bb21)
}
bb21 = {
_32 = _4;
_7 = _1 | _1;
_15 = ['\u{2107d}','\u{25917}','\u{d1cb7}'];
Goto(bb22)
}
bb22 = {
_30 = _26.0 - _26.0;
_26 = (_30,);
_17 = !_30;
_16 = 3013682235457859353_i64 as f64;
_1 = _26.0 as u128;
Call(_34 = core::intrinsics::transmute(_17), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_20.2.0 = _26.0;
_20.3 = &mut _8;
Goto(bb24)
}
bb24 = {
_20.2 = (_17,);
_7 = _4;
_20.3 = &mut _14;
_21 = [13383239990805487510_usize,2_usize,6829436974468309037_usize];
_36 = [1666731079_u32,511220233_u32,1769071046_u32,828535916_u32,584967084_u32,1111547933_u32];
_20.0 = core::ptr::addr_of!(_35);
_30 = -_17;
Goto(bb25)
}
bb25 = {
_37 = !_20.2.0;
_9 = [_13,_13,_13,_13];
_31 = &_37;
_21 = [5_usize,4_usize,15435149190263220983_usize];
_20.0 = core::ptr::addr_of!(_35);
_16 = 45590_u16 as f64;
_25 = &_15;
_5 = 43_i8 as f64;
_21 = [11218328729136003827_usize,9819930915138310649_usize,1_usize];
_9 = [_13,_13,_13,_13];
_2 = _6 & _6;
_39 = [_2,_2,_2];
_43.2 = 15560_u16;
_35 = 98_i8 | (-109_i8);
_27 = [(*_31),_17,(*_31),(*_31),(*_31),(*_31)];
_37 = _26.0 >> _20.2.0;
_41 = _3 as u8;
Goto(bb26)
}
bb26 = {
_42 = _2 as f32;
_24 = &_20.2.0;
_41 = !149_u8;
_36 = [4082037839_u32,46864607_u32,1880180677_u32,767965699_u32,1161486398_u32,1836591860_u32];
Goto(bb27)
}
bb27 = {
_21 = [3519449889131824429_usize,11844555857282871743_usize,5_usize];
_20.2 = (_17,);
_20.2 = (_26.0,);
Goto(bb28)
}
bb28 = {
_21 = [2387092850303383016_usize,12747347046662927676_usize,27193923315733692_usize];
_33 = 1553476866_u32 | 4275309201_u32;
_30 = _1 as i32;
_7 = _1 & _1;
_42 = _17 as f32;
_18 = [_6,_2,_6];
_9 = [_13,_13,_13,_13];
_21 = [1508754270787181872_usize,7_usize,9479990477384413650_usize];
_32 = !_1;
_12 = [_1,_7,_4,_1];
_5 = _3 + _3;
_45 = RET * RET;
_18 = _39;
_43.0 = _2;
_44 = _13 & _13;
match _43.2 {
0 => bb1,
1 => bb26,
2 => bb6,
3 => bb20,
4 => bb29,
15560 => bb31,
_ => bb30
}
}
bb29 = {
Return()
}
bb30 = {
Return()
}
bb31 = {
_32 = _7 - _7;
_39 = _18;
_26 = (_37,);
_18 = [_6,_43.0,_2];
_35 = (-85_i8);
_3 = _5;
_20.0 = core::ptr::addr_of!(_35);
_5 = _3 - _3;
_42 = _45 * _34;
_7 = 1_usize as u128;
_49 = 6_usize >> _35;
_37 = _26.0;
match _35 {
0 => bb1,
1 => bb10,
2 => bb32,
340282366920938463463374607431768211371 => bb34,
_ => bb33
}
}
bb32 = {
Return()
}
bb33 = {
Return()
}
bb34 = {
_45 = -_42;
_2 = '\u{efae8}' as i16;
_49 = '\u{84406}' as usize;
_33 = 965676378_u32 >> _32;
_35 = _32 as i8;
_20.2 = _26;
_20.2.0 = _26.0 + _37;
_30 = _26.0 & _37;
_17 = _5 as i32;
_13 = !_44;
_37 = _20.2.0 ^ _17;
_32 = _1 << _26.0;
_34 = _42 - _45;
_4 = _32 & _32;
_1 = _32 | _32;
_20.0 = core::ptr::addr_of!(_35);
_26.0 = !_17;
_20.0 = core::ptr::addr_of!(_35);
_37 = !_20.2.0;
_47.1.fld0.fld1 = !_41;
_54 = !true;
_34 = _42 * _45;
_43.0 = _6;
Goto(bb35)
}
bb35 = {
_9 = [_13,_44,_13,_44];
Goto(bb36)
}
bb36 = {
_16 = _3 - _3;
_27 = [_30,_26.0,_17,_17,_37,_30];
_34 = RET - _42;
_1 = _4 ^ _4;
Goto(bb37)
}
bb37 = {
_5 = _33 as f64;
_49 = _41 as usize;
_1 = 9793305203576880747_u64 as u128;
_48 = (-5090739397533501308_i64) as isize;
_45 = -_42;
_6 = _35 as i16;
_24 = &_17;
_26 = ((*_24),);
_49 = 3_usize << (*_24);
_24 = &_37;
_51 = [_4,_4,_4,_32];
_20.2.0 = -(*_24);
_15 = ['\u{2c769}','\u{4664b}','\u{d9e5b}'];
_50 = _45 as f64;
_59.2.fld1 = _41 + _41;
_60 = _34;
_18 = _39;
match _43.2 {
0 => bb31,
15560 => bb39,
_ => bb38
}
}
bb38 = {
_30 = _26.0 - _26.0;
_26 = (_30,);
_17 = !_30;
_16 = 3013682235457859353_i64 as f64;
_1 = _26.0 as u128;
Call(_34 = core::intrinsics::transmute(_17), ReturnTo(bb23), UnwindUnreachable())
}
bb39 = {
_36 = [_33,_33,_33,_33,_33,_33];
_6 = _2;
match _43.2 {
0 => bb22,
1 => bb12,
2 => bb31,
3 => bb40,
4 => bb41,
15560 => bb43,
_ => bb42
}
}
bb40 = {
Return()
}
bb41 = {
Return()
}
bb42 = {
_20.2 = (_17,);
_7 = _4;
_20.3 = &mut _14;
_21 = [13383239990805487510_usize,2_usize,6829436974468309037_usize];
_36 = [1666731079_u32,511220233_u32,1769071046_u32,828535916_u32,584967084_u32,1111547933_u32];
_20.0 = core::ptr::addr_of!(_35);
_30 = -_17;
Goto(bb25)
}
bb43 = {
_47.1.fld1 = core::ptr::addr_of_mut!(_57);
_59.1 = [_44,_44,_13,_13];
_20.2 = (_17,);
_45 = _60 + _60;
_12 = [_4,_4,_4,_4];
_57 = _54;
_30 = _17 | (*_24);
_20.2.0 = (*_24) - _37;
_27 = [(*_24),(*_24),(*_24),(*_24),(*_24),(*_24)];
_2 = _6;
_58 = '\u{b71d1}';
_61 = _50 - _16;
RET = 8604586828954315393_u64 as f32;
_30 = !(*_24);
_2 = !_43.0;
_18 = _39;
_49 = 7_usize >> (*_24);
_16 = -_3;
_43.2 = 60536_u16 & 2190_u16;
_43.2 = _2 as u16;
_33 = _2 as u32;
_17 = _30 * (*_24);
_44 = _13;
Goto(bb44)
}
bb44 = {
_34 = -_45;
_26 = (_17,);
_66 = _57;
_54 = !_57;
_20.0 = core::ptr::addr_of!(_35);
_24 = &_26.0;
_47.1.fld2 = [_32,_4,_32,_32];
_43.1 = core::ptr::addr_of_mut!(_21);
Goto(bb45)
}
bb45 = {
_5 = _3 * _61;
_59.1 = [_13,_13,_44,_13];
_56 = !_48;
_39 = [_2,_43.0,_43.0];
_18 = _39;
_12 = [_4,_4,_4,_4];
_61 = -_5;
_16 = _43.0 as f64;
_65 = -_61;
_1 = !_4;
_30 = (*_24) << (*_24);
_36 = [_33,_33,_33,_33,_33,_33];
_42 = (-2944266291293709753_i64) as f32;
_20.2 = (_37,);
_47.0 = [_58,_58,_58,_58];
_47.1.fld1 = core::ptr::addr_of_mut!(_66);
_47.1.fld0.fld1 = _41 - _59.2.fld1;
_4 = !_7;
_69.0.fld1 = Move(_47.1.fld1);
_60 = _45 + _45;
_49 = 3_usize & 12236282470430687373_usize;
_55 = _56 | _48;
_72 = _58;
Call(_41 = core::intrinsics::bswap(_47.1.fld0.fld1), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
_47.1.fld0.fld1 = _35 as u8;
Goto(bb47)
}
bb47 = {
_71 = Adt31::Variant0 { fld0: _1,fld1: _26,fld2: 14088996290523021930_u64,fld3: _35,fld4: _43.0 };
place!(Field::<u64>(Variant(_71, 0), 2)) = 13302335916691462211_u64 << (*_24);
_47.0 = [_72,_58,_72,_58];
_43.0 = Field::<i16>(Variant(_71, 0), 4) << (*_24);
_56 = !_55;
_17 = -(*_24);
_15 = [_72,_72,_58];
_35 = Field::<u64>(Variant(_71, 0), 2) as i8;
_64 = _60;
_51 = [Field::<u128>(Variant(_71, 0), 0),_32,_1,_1];
_26 = _20.2;
_65 = _5 * _5;
_69.1 = core::ptr::addr_of!(_66);
_39 = _18;
_7 = Field::<u128>(Variant(_71, 0), 0);
place!(Field::<(i32,)>(Variant(_71, 0), 1)) = _26;
_38 = core::ptr::addr_of_mut!(_69.1);
_73 = _55 - _48;
_49 = 5_usize ^ 5_usize;
_1 = Field::<u128>(Variant(_71, 0), 0) + _32;
(*_38) = core::ptr::addr_of!(_57);
_50 = _33 as f64;
_65 = -_5;
_69.0.fld0.fld1 = _59.2.fld1 | _47.1.fld0.fld1;
_5 = Field::<i8>(Variant(_71, 0), 3) as f64;
_65 = -_61;
_20.2 = (_17,);
Goto(bb48)
}
bb48 = {
_59.1 = [_13,_13,_13,_13];
_39 = _18;
place!(Field::<(i32,)>(Variant(_71, 0), 1)).0 = -_37;
_20.2.0 = !_17;
_18 = [Field::<i16>(Variant(_71, 0), 4),_43.0,_43.0];
_20.2.0 = _30;
(*_38) = core::ptr::addr_of!(_66);
_26.0 = _17;
(*_38) = core::ptr::addr_of!(_54);
(*_38) = core::ptr::addr_of!(_57);
_36 = [_33,_33,_33,_33,_33,_33];
_57 = _54;
_42 = _34 + _60;
_55 = _73 & _48;
_81 = _73 << _69.0.fld0.fld1;
_43.2 = 21559_u16;
_41 = _69.0.fld0.fld1 | _47.1.fld0.fld1;
_65 = _3 * _5;
_79 = [_44,_44,_13,_44];
_5 = _61;
_79 = [_44,_44,_13,_44];
Goto(bb49)
}
bb49 = {
_50 = _5 - _65;
_41 = _59.2.fld1 * _47.1.fld0.fld1;
_66 = _57 & _54;
_68 = core::ptr::addr_of_mut!(_56);
(*_38) = core::ptr::addr_of!(_66);
_20.0 = core::ptr::addr_of!(_35);
_39 = _18;
_15 = [_58,_58,_72];
_31 = &_26.0;
(*_38) = core::ptr::addr_of!(_66);
_78.0.fld2 = [Field::<u128>(Variant(_71, 0), 0),_7,_7,_1];
_37 = (*_31);
(*_38) = core::ptr::addr_of!(_66);
_42 = _64 * _45;
_62 = [_49,_49,_49];
_69.1 = core::ptr::addr_of!(_54);
RET = _42 + _60;
Goto(bb50)
}
bb50 = {
Call(_85 = dump_var(15_usize, 72_usize, Move(_72), 2_usize, Move(_2), 35_usize, Move(_35), 39_usize, Move(_39)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_85 = dump_var(15_usize, 57_usize, Move(_57), 8_usize, Move(_8), 9_usize, Move(_9), 58_usize, Move(_58)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_85 = dump_var(15_usize, 12_usize, Move(_12), 13_usize, Move(_13), 14_usize, Move(_14), 15_usize, Move(_15)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_85 = dump_var(15_usize, 36_usize, Move(_36), 73_usize, Move(_73), 79_usize, Move(_79), 23_usize, Move(_23)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_85 = dump_var(15_usize, 26_usize, Move(_26), 27_usize, Move(_27), 48_usize, Move(_48), 66_usize, Move(_66)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: &'static [char; 3],mut _2: [usize; 3],mut _3: &'static i32,mut _4: i32,mut _5: &'static mut isize,mut _6: &'static i32,mut _7: i32,mut _8: i32) -> [u128; 4] {
mir! {
type RET = [u128; 4];
let _9: &'static usize;
let _10: isize;
let _11: u16;
let _12: Adt27;
let _13: i64;
let _14: u128;
let _15: isize;
let _16: isize;
let _17: *const *mut u16;
let _18: &'static mut Adt57;
let _19: *mut Adt31;
let _20: [u128; 4];
let _21: i8;
let _22: i128;
let _23: &'static usize;
let _24: *mut *mut [usize; 3];
let _25: Adt44;
let _26: f64;
let _27: *mut *mut [usize; 3];
let _28: *const bool;
let _29: [u32; 6];
let _30: &'static [char; 3];
let _31: &'static mut i128;
let _32: f64;
let _33: char;
let _34: &'static mut Adt27;
let _35: isize;
let _36: [char; 4];
let _37: (i32,);
let _38: u64;
let _39: usize;
let _40: Adt73;
let _41: [u16; 2];
let _42: i8;
let _43: [i128; 4];
let _44: Adt27;
let _45: u32;
let _46: [i32; 6];
let _47: (i16, *mut [usize; 3], u16);
let _48: i128;
let _49: isize;
let _50: i128;
let _51: char;
let _52: (bool, u64, i8, u128);
let _53: f32;
let _54: char;
let _55: *mut &'static mut *mut u16;
let _56: isize;
let _57: i8;
let _58: bool;
let _59: &'static i32;
let _60: u64;
let _61: f32;
let _62: isize;
let _63: u16;
let _64: &'static mut *mut u16;
let _65: &'static Adt38;
let _66: (i16, *mut [usize; 3], u16);
let _67: char;
let _68: (bool, u64, i8, u128);
let _69: &'static mut &'static usize;
let _70: (Adt57, *const bool);
let _71: *mut f32;
let _72: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _73: *const Adt18;
let _74: char;
let _75: u8;
let _76: &'static Adt57;
let _77: isize;
let _78: &'static mut isize;
let _79: f32;
let _80: char;
let _81: *mut u16;
let _82: Adt31;
let _83: f64;
let _84: Adt27;
let _85: *mut [i128; 4];
let _86: *mut Adt31;
let _87: *const *mut [usize; 3];
let _88: f32;
let _89: *mut [i128; 4];
let _90: bool;
let _91: isize;
let _92: isize;
let _93: u16;
let _94: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _95: &'static usize;
let _96: u8;
let _97: f32;
let _98: &'static mut isize;
let _99: [i128; 3];
let _100: i128;
let _101: char;
let _102: f32;
let _103: ();
let _104: ();
{
_3 = &_8;
_4 = -_8;
_7 = _4 << _8;
RET = [117328998276440212008708754484612771411_u128,155039702724672443870369431326093038263_u128,100245141370192806042614434839760615843_u128,204854579136278281280266559635627465066_u128];
_6 = &(*_3);
RET = [107070742044531381335498618836913543797_u128,129158659689684414490138145810898393001_u128,14691862638621103042337314514836396171_u128,136069395629803702350776165345276006268_u128];
_8 = _7 << _7;
_8 = _7 + _7;
_8 = !_4;
Goto(bb1)
}
bb1 = {
_7 = _4;
_4 = _8;
_3 = &_7;
_8 = (*_3) ^ (*_3);
RET = [53217126860942615039082763412990994674_u128,57830064024355327455939747777547314631_u128,319493165624579151775418601067207395796_u128,219338488255800199002410187549678474358_u128];
_8 = (*_3) | (*_3);
RET = [79928502838032728606336238007248885233_u128,276237769271384997620240938314227059494_u128,67706516044591605527345039314866845853_u128,294158137260116527849737215468280578053_u128];
_6 = Move(_3);
_6 = &_4;
_4 = _7 << _7;
_3 = Move(_6);
_4 = _7 - _8;
_6 = &_7;
_3 = &(*_6);
_3 = &_8;
RET = [218563104860416675848586924135999363613_u128,103752376761717832833640826303853728494_u128,56113740816152794418006805500453508221_u128,93855102575880421566715245179820216942_u128];
_7 = (*_3) & (*_3);
_7 = -(*_3);
_10 = (-28_isize) >> (*_3);
_10 = !(-9223372036854775808_isize);
_10 = 35_isize + 118_isize;
_6 = &(*_3);
_11 = !58309_u16;
_11 = 54611_u16 & 27268_u16;
Goto(bb2)
}
bb2 = {
_5 = &mut _10;
(*_5) = (-34_isize);
(*_5) = (-67_isize);
_8 = !_4;
_14 = !279105853661401548453821187621853258127_u128;
_2 = [7_usize,1_usize,11988944408901559793_usize];
RET = [_14,_14,_14,_14];
_15 = _14 as isize;
_6 = &_4;
(*_5) = _15 + _15;
(*_5) = !_15;
(*_5) = _15;
(*_5) = _15 + _15;
(*_5) = -_15;
RET = [_14,_14,_14,_14];
(*_5) = _15 | _15;
_15 = (*_5);
Goto(bb3)
}
bb3 = {
(*_5) = _15 - _15;
(*_5) = _15 ^ _15;
(*_5) = _15 - _15;
Call((*_5) = core::intrinsics::transmute(_15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = &mut _15;
_7 = (*_6) ^ (*_6);
(*_5) = (-62_isize);
(*_5) = 9223372036854775807_isize;
match (*_5) {
0 => bb1,
1 => bb2,
9223372036854775807 => bb5,
_ => bb3
}
}
bb5 = {
_14 = 212170010960895642436112818209088925620_u128;
(*_5) = 9223372036854775807_isize * (-9223372036854775808_isize);
(*_5) = 102_isize << (*_6);
(*_5) = '\u{40fbd}' as isize;
(*_5) = true as isize;
(*_5) = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
(*_5) = 49_isize;
(*_5) = 13_isize;
(*_5) = 5547017245218411587_usize as isize;
_14 = 182150138819511467387584929726260163048_u128 >> _4;
(*_5) = _11 as isize;
(*_5) = 9223372036854775807_isize << _8;
(*_5) = (-57_isize);
(*_5) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = &(*_6);
(*_5) = 9223372036854775807_isize;
(*_5) = 1634191991175662107_u64 as isize;
_7 = (*_6);
(*_5) = (-42_isize) * 9223372036854775807_isize;
_11 = 32016_u16 * 46047_u16;
_3 = &_7;
_14 = !201434588542903539514693243155353430945_u128;
(*_5) = 106_isize;
Goto(bb6)
}
bb6 = {
(*_5) = 13399821971725012195_u64 as isize;
(*_5) = (-30_isize) + (-9223372036854775808_isize);
Goto(bb7)
}
bb7 = {
_4 = (*_3);
_13 = !(-2944874379588284368_i64);
(*_5) = -(-9223372036854775808_isize);
_6 = &(*_3);
_3 = Move(_6);
(*_5) = 9223372036854775807_isize;
RET = [_14,_14,_14,_14];
(*_5) = _14 as isize;
(*_5) = (-52_isize);
_14 = 90130182039784985076288358731368564237_u128 - 82944277143497978119261107511913740609_u128;
(*_5) = _14 as isize;
Goto(bb8)
}
bb8 = {
_11 = !27609_u16;
(*_5) = '\u{6c4e1}' as isize;
_20 = [_14,_14,_14,_14];
(*_5) = !(-98_isize);
(*_5) = 9223372036854775807_isize;
(*_5) = 49_isize;
(*_5) = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*_5) = -(-9223372036854775808_isize);
_4 = !_7;
Goto(bb9)
}
bb9 = {
(*_5) = (-9223372036854775808_isize);
(*_5) = 21_isize & 9223372036854775807_isize;
_6 = &_8;
(*_5) = -9223372036854775807_isize;
(*_5) = 20_isize;
_25.fld1 = 215_u8 - 196_u8;
_22 = (*_6) as i128;
_3 = Move(_6);
_3 = &_4;
(*_5) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
(*_5) = !(-87_isize);
(*_5) = (-95_isize) - 9223372036854775807_isize;
_3 = &_7;
(*_5) = (-74_i8) as isize;
_6 = &(*_3);
(*_5) = (-48_isize);
_2 = [14355725896940684543_usize,7_usize,11397695403308819922_usize];
match (*_5) {
0 => bb3,
1 => bb10,
2 => bb11,
340282366920938463463374607431768211408 => bb13,
_ => bb12
}
}
bb10 = {
_5 = &mut _15;
_7 = (*_6) ^ (*_6);
(*_5) = (-62_isize);
(*_5) = 9223372036854775807_isize;
match (*_5) {
0 => bb1,
1 => bb2,
9223372036854775807 => bb5,
_ => bb3
}
}
bb11 = {
_4 = (*_3);
_13 = !(-2944874379588284368_i64);
(*_5) = -(-9223372036854775808_isize);
_6 = &(*_3);
_3 = Move(_6);
(*_5) = 9223372036854775807_isize;
RET = [_14,_14,_14,_14];
(*_5) = _14 as isize;
(*_5) = (-52_isize);
_14 = 90130182039784985076288358731368564237_u128 - 82944277143497978119261107511913740609_u128;
(*_5) = _14 as isize;
Goto(bb8)
}
bb12 = {
_14 = 212170010960895642436112818209088925620_u128;
(*_5) = 9223372036854775807_isize * (-9223372036854775808_isize);
(*_5) = 102_isize << (*_6);
(*_5) = '\u{40fbd}' as isize;
(*_5) = true as isize;
(*_5) = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
(*_5) = 49_isize;
(*_5) = 13_isize;
(*_5) = 5547017245218411587_usize as isize;
_14 = 182150138819511467387584929726260163048_u128 >> _4;
(*_5) = _11 as isize;
(*_5) = 9223372036854775807_isize << _8;
(*_5) = (-57_isize);
(*_5) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = &(*_6);
(*_5) = 9223372036854775807_isize;
(*_5) = 1634191991175662107_u64 as isize;
_7 = (*_6);
(*_5) = (-42_isize) * 9223372036854775807_isize;
_11 = 32016_u16 * 46047_u16;
_3 = &_7;
_14 = !201434588542903539514693243155353430945_u128;
(*_5) = 106_isize;
Goto(bb6)
}
bb13 = {
(*_5) = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_5) = 9223372036854775807_isize;
(*_5) = false as isize;
(*_5) = -9223372036854775807_isize;
_20 = [_14,_14,_14,_14];
(*_5) = 9223372036854775807_isize;
RET = _20;
(*_5) = (-9223372036854775808_isize) << (*_6);
(*_5) = !9223372036854775807_isize;
(*_5) = !(-9223372036854775808_isize);
_32 = 917794730_u32 as f64;
_7 = (*_5) as i32;
_13 = -(-5579861748756079251_i64);
_37 = (_8,);
(*_5) = (-46_isize) & 89_isize;
(*_5) = (-9223372036854775808_isize) & 123_isize;
_6 = &_4;
_25.fld1 = 115_u8;
_13 = (-1666577692478850074_i64);
match _25.fld1 {
0 => bb1,
1 => bb9,
115 => bb14,
_ => bb3
}
}
bb14 = {
(*_5) = !124_isize;
_21 = (-40_i8) >> (*_6);
(*_5) = 1_isize;
(*_5) = _8 as isize;
_29 = [3884858341_u32,38893352_u32,4052854838_u32,731638238_u32,632576564_u32,683714098_u32];
_35 = (*_5) >> (*_6);
(*_5) = _35 | _35;
_3 = Move(_6);
(*_5) = -_35;
(*_5) = !_35;
_39 = _21 as usize;
_23 = &_39;
(*_5) = 17196874710157515445_u64 as isize;
_9 = &(*_23);
_4 = _8 | _37.0;
(*_5) = _35 + _35;
(*_5) = _35 * _35;
_32 = 8782694270848949896_u64 as f64;
_29 = [2460573431_u32,964721323_u32,446961237_u32,4128384418_u32,1205493222_u32,438071513_u32];
match _13 {
0 => bb15,
340282366920938463461708029739289361382 => bb17,
_ => bb16
}
}
bb15 = {
(*_5) = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_5) = 9223372036854775807_isize;
(*_5) = false as isize;
(*_5) = -9223372036854775807_isize;
_20 = [_14,_14,_14,_14];
(*_5) = 9223372036854775807_isize;
RET = _20;
(*_5) = (-9223372036854775808_isize) << (*_6);
(*_5) = !9223372036854775807_isize;
(*_5) = !(-9223372036854775808_isize);
_32 = 917794730_u32 as f64;
_7 = (*_5) as i32;
_13 = -(-5579861748756079251_i64);
_37 = (_8,);
(*_5) = (-46_isize) & 89_isize;
(*_5) = (-9223372036854775808_isize) & 123_isize;
_6 = &_4;
_25.fld1 = 115_u8;
_13 = (-1666577692478850074_i64);
match _25.fld1 {
0 => bb1,
1 => bb9,
115 => bb14,
_ => bb3
}
}
bb16 = {
_11 = !27609_u16;
(*_5) = '\u{6c4e1}' as isize;
_20 = [_14,_14,_14,_14];
(*_5) = !(-98_isize);
(*_5) = 9223372036854775807_isize;
(*_5) = 49_isize;
(*_5) = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*_5) = -(-9223372036854775808_isize);
_4 = !_7;
Goto(bb9)
}
bb17 = {
_33 = '\u{2f6c2}';
(*_5) = _35;
(*_5) = 17672389748727049959_u64 as isize;
(*_5) = _14 as isize;
_43 = [_22,_22,_22,_22];
_48 = -_22;
(*_5) = _4 as isize;
_46 = [_8,_37.0,_4,_37.0,_4,_4];
_42 = _21 << (*_23);
(*_5) = _35 ^ _35;
_45 = 270718837_u32;
_52 = (false, 10186160937710746340_u64, _42, _14);
_52.2 = (*_5) as i8;
_46 = [_37.0,_8,_37.0,_4,_4,_4];
_37 = (_4,);
_20 = [_52.3,_52.3,_52.3,_52.3];
(*_5) = _35;
_48 = (-3420_i16) as i128;
(*_5) = !_35;
_51 = _33;
_42 = _52.2 & _52.2;
_32 = _25.fld1 as f64;
(*_5) = _52.0 as isize;
Call(_37.0 = fn17(Move(_9), _4, Move(_23), _52.3, (*_23), _43, _22, (*_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_25.fld1 = 10_u8 << (*_5);
(*_5) = _11 as isize;
_6 = &_7;
_38 = _52.1;
_50 = !_22;
_23 = &_39;
_36 = [_51,_33,_51,_33];
_28 = core::ptr::addr_of!(_52.0);
(*_5) = -_35;
_53 = _32 as f32;
_49 = _52.1 as isize;
_37 = (_4,);
(*_28) = true;
_35 = (*_5);
_54 = _51;
_14 = _52.3;
(*_28) = false;
(*_28) = !false;
_31 = &mut _48;
Goto(bb19)
}
bb19 = {
RET = [_14,_14,_14,_14];
(*_28) = (*_5) != (*_5);
match _38 {
0 => bb12,
1 => bb20,
2 => bb21,
10186160937710746340 => bb23,
_ => bb22
}
}
bb20 = {
_7 = _4;
_4 = _8;
_3 = &_7;
_8 = (*_3) ^ (*_3);
RET = [53217126860942615039082763412990994674_u128,57830064024355327455939747777547314631_u128,319493165624579151775418601067207395796_u128,219338488255800199002410187549678474358_u128];
_8 = (*_3) | (*_3);
RET = [79928502838032728606336238007248885233_u128,276237769271384997620240938314227059494_u128,67706516044591605527345039314866845853_u128,294158137260116527849737215468280578053_u128];
_6 = Move(_3);
_6 = &_4;
_4 = _7 << _7;
_3 = Move(_6);
_4 = _7 - _8;
_6 = &_7;
_3 = &(*_6);
_3 = &_8;
RET = [218563104860416675848586924135999363613_u128,103752376761717832833640826303853728494_u128,56113740816152794418006805500453508221_u128,93855102575880421566715245179820216942_u128];
_7 = (*_3) & (*_3);
_7 = -(*_3);
_10 = (-28_isize) >> (*_3);
_10 = !(-9223372036854775808_isize);
_10 = 35_isize + 118_isize;
_6 = &(*_3);
_11 = !58309_u16;
_11 = 54611_u16 & 27268_u16;
Goto(bb2)
}
bb21 = {
(*_5) = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_5) = 9223372036854775807_isize;
(*_5) = false as isize;
(*_5) = -9223372036854775807_isize;
_20 = [_14,_14,_14,_14];
(*_5) = 9223372036854775807_isize;
RET = _20;
(*_5) = (-9223372036854775808_isize) << (*_6);
(*_5) = !9223372036854775807_isize;
(*_5) = !(-9223372036854775808_isize);
_32 = 917794730_u32 as f64;
_7 = (*_5) as i32;
_13 = -(-5579861748756079251_i64);
_37 = (_8,);
(*_5) = (-46_isize) & 89_isize;
(*_5) = (-9223372036854775808_isize) & 123_isize;
_6 = &_4;
_25.fld1 = 115_u8;
_13 = (-1666577692478850074_i64);
match _25.fld1 {
0 => bb1,
1 => bb9,
115 => bb14,
_ => bb3
}
}
bb22 = {
_5 = &mut _10;
(*_5) = (-34_isize);
(*_5) = (-67_isize);
_8 = !_4;
_14 = !279105853661401548453821187621853258127_u128;
_2 = [7_usize,1_usize,11988944408901559793_usize];
RET = [_14,_14,_14,_14];
_15 = _14 as isize;
_6 = &_4;
(*_5) = _15 + _15;
(*_5) = !_15;
(*_5) = _15;
(*_5) = _15 + _15;
(*_5) = -_15;
RET = [_14,_14,_14,_14];
(*_5) = _15 | _15;
_15 = (*_5);
Goto(bb3)
}
bb23 = {
_56 = -(*_5);
(*_28) = !true;
(*_28) = (*_5) >= (*_5);
_27 = core::ptr::addr_of_mut!(_47.1);
_56 = (*_5) << _52.2;
(*_5) = _56 - _35;
_50 = _38 as i128;
_45 = 2461188987_u32;
_57 = _52.2;
_57 = _42 * _42;
_24 = core::ptr::addr_of_mut!((*_27));
(*_31) = _50 | _50;
(*_24) = core::ptr::addr_of_mut!(_2);
(*_28) = (*_5) != (*_5);
_53 = _13 as f32;
(*_5) = _56 ^ _56;
(*_31) = _22 ^ _50;
_33 = _54;
_41 = [_11,_11];
(*_28) = _42 >= _52.2;
(*_28) = true;
(*_31) = _50 & _22;
Goto(bb24)
}
bb24 = {
(*_5) = _56;
(*_24) = core::ptr::addr_of_mut!(_2);
(*_28) = !true;
(*_27) = core::ptr::addr_of_mut!(_2);
(*_27) = core::ptr::addr_of_mut!(_2);
_56 = !(*_5);
(*_31) = !_50;
_62 = (*_5) << _56;
(*_5) = _62;
(*_28) = _49 == (*_5);
(*_5) = _45 as isize;
(*_27) = core::ptr::addr_of_mut!(_2);
_49 = -_35;
(*_27) = core::ptr::addr_of_mut!(_2);
(*_31) = _52.3 as i128;
_52.2 = (*_23) as i8;
(*_31) = _8 as i128;
(*_27) = core::ptr::addr_of_mut!(_2);
_33 = _54;
(*_5) = _49;
_9 = Move(_23);
Call(_52 = fn18(Move(_47.1), (*_5), Move(_5), (*_5)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_51 = _33;
(*_27) = core::ptr::addr_of_mut!(_2);
(*_27) = core::ptr::addr_of_mut!(_2);
(*_27) = core::ptr::addr_of_mut!(_2);
_46 = [_37.0,_37.0,_4,_4,_8,_8];
(*_31) = !_50;
_50 = !(*_31);
_27 = core::ptr::addr_of_mut!((*_27));
(*_31) = _22;
(*_27) = core::ptr::addr_of_mut!(_2);
_24 = core::ptr::addr_of_mut!((*_27));
(*_24) = core::ptr::addr_of_mut!(_2);
_12 = Adt27::Variant0 { fld0: Move(_28),fld1: _11,fld2: _56,fld3: _57,fld4: 32235_i16,fld5: _52,fld6: _13 };
(*_27) = core::ptr::addr_of_mut!(_2);
_66.2 = _45 as u16;
_63 = _66.2 | _11;
Goto(bb26)
}
bb26 = {
_51 = _54;
_26 = _32 + _32;
(*_27) = core::ptr::addr_of_mut!(_2);
_51 = _54;
_70.0.fld2 = [Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5).3,_52.3,Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5).3,_14];
place!(Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5)).1 = _38 - _38;
(*_27) = core::ptr::addr_of_mut!(_2);
_33 = _51;
(*_27) = core::ptr::addr_of_mut!(_2);
_58 = Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5).0;
_47.0 = -(-1698_i16);
(*_27) = core::ptr::addr_of_mut!(_2);
place!(Field::<isize>(Variant(_12, 0), 2)) = _49 << _49;
_66.0 = _47.0 << Field::<isize>(Variant(_12, 0), 2);
_68.3 = _52.3;
_37.0 = _33 as i32;
_51 = _54;
match _38 {
0 => bb19,
1 => bb2,
2 => bb20,
3 => bb4,
4 => bb5,
5 => bb27,
10186160937710746340 => bb29,
_ => bb28
}
}
bb27 = {
(*_5) = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_5) = 9223372036854775807_isize;
(*_5) = false as isize;
(*_5) = -9223372036854775807_isize;
_20 = [_14,_14,_14,_14];
(*_5) = 9223372036854775807_isize;
RET = _20;
(*_5) = (-9223372036854775808_isize) << (*_6);
(*_5) = !9223372036854775807_isize;
(*_5) = !(-9223372036854775808_isize);
_32 = 917794730_u32 as f64;
_7 = (*_5) as i32;
_13 = -(-5579861748756079251_i64);
_37 = (_8,);
(*_5) = (-46_isize) & 89_isize;
(*_5) = (-9223372036854775808_isize) & 123_isize;
_6 = &_4;
_25.fld1 = 115_u8;
_13 = (-1666577692478850074_i64);
match _25.fld1 {
0 => bb1,
1 => bb9,
115 => bb14,
_ => bb3
}
}
bb28 = {
(*_5) = 13399821971725012195_u64 as isize;
(*_5) = (-30_isize) + (-9223372036854775808_isize);
Goto(bb7)
}
bb29 = {
_62 = Field::<i64>(Variant(_12, 0), 6) as isize;
_70.0.fld1 = core::ptr::addr_of_mut!(_58);
_52 = Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5);
(*_31) = _50 + _50;
_13 = Field::<i64>(Variant(_12, 0), 6) | Field::<i64>(Variant(_12, 0), 6);
_20 = _70.0.fld2;
(*_31) = _50 * _50;
(*_27) = core::ptr::addr_of_mut!(_2);
_46 = [_8,(*_6),_4,(*_6),_37.0,_4];
_13 = Field::<i64>(Variant(_12, 0), 6) - Field::<i64>(Variant(_12, 0), 6);
(*_31) = !_50;
_71 = core::ptr::addr_of_mut!(_61);
(*_71) = -_53;
_28 = core::ptr::addr_of!(_58);
match _52.3 {
0 => bb15,
154744667998533081924141067781196576658 => bb31,
_ => bb30
}
}
bb30 = {
_11 = !27609_u16;
(*_5) = '\u{6c4e1}' as isize;
_20 = [_14,_14,_14,_14];
(*_5) = !(-98_isize);
(*_5) = 9223372036854775807_isize;
(*_5) = 49_isize;
(*_5) = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*_5) = -(-9223372036854775808_isize);
_4 = !_7;
Goto(bb9)
}
bb31 = {
_21 = -_57;
_5 = &mut _35;
_52 = Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5);
_32 = _26;
_42 = _57 >> (*_5);
_74 = _51;
(*_31) = _22 | _22;
(*_5) = Field::<isize>(Variant(_12, 0), 2) << (*_31);
_56 = _57 as isize;
_3 = &(*_6);
match _38 {
0 => bb19,
1 => bb32,
10186160937710746340 => bb34,
_ => bb33
}
}
bb32 = {
_7 = _4;
_4 = _8;
_3 = &_7;
_8 = (*_3) ^ (*_3);
RET = [53217126860942615039082763412990994674_u128,57830064024355327455939747777547314631_u128,319493165624579151775418601067207395796_u128,219338488255800199002410187549678474358_u128];
_8 = (*_3) | (*_3);
RET = [79928502838032728606336238007248885233_u128,276237769271384997620240938314227059494_u128,67706516044591605527345039314866845853_u128,294158137260116527849737215468280578053_u128];
_6 = Move(_3);
_6 = &_4;
_4 = _7 << _7;
_3 = Move(_6);
_4 = _7 - _8;
_6 = &_7;
_3 = &(*_6);
_3 = &_8;
RET = [218563104860416675848586924135999363613_u128,103752376761717832833640826303853728494_u128,56113740816152794418006805500453508221_u128,93855102575880421566715245179820216942_u128];
_7 = (*_3) & (*_3);
_7 = -(*_3);
_10 = (-28_isize) >> (*_3);
_10 = !(-9223372036854775808_isize);
_10 = 35_isize + 118_isize;
_6 = &(*_3);
_11 = !58309_u16;
_11 = 54611_u16 & 27268_u16;
Goto(bb2)
}
bb33 = {
_51 = _33;
(*_27) = core::ptr::addr_of_mut!(_2);
(*_27) = core::ptr::addr_of_mut!(_2);
(*_27) = core::ptr::addr_of_mut!(_2);
_46 = [_37.0,_37.0,_4,_4,_8,_8];
(*_31) = !_50;
_50 = !(*_31);
_27 = core::ptr::addr_of_mut!((*_27));
(*_31) = _22;
(*_27) = core::ptr::addr_of_mut!(_2);
_24 = core::ptr::addr_of_mut!((*_27));
(*_24) = core::ptr::addr_of_mut!(_2);
_12 = Adt27::Variant0 { fld0: Move(_28),fld1: _11,fld2: _56,fld3: _57,fld4: 32235_i16,fld5: _52,fld6: _13 };
(*_27) = core::ptr::addr_of_mut!(_2);
_66.2 = _45 as u16;
_63 = _66.2 | _11;
Goto(bb26)
}
bb34 = {
_72.1 = [(*_31),(*_31),(*_31),(*_31)];
(*_71) = _45 as f32;
(*_31) = _25.fld1 as i128;
_16 = !_49;
(*_31) = -_50;
place!(Field::<i16>(Variant(_12, 0), 4)) = _66.0 & _66.0;
_20 = _70.0.fld2;
_68.0 = (*_5) < (*_5);
(*_27) = core::ptr::addr_of_mut!(_2);
(*_5) = Field::<isize>(Variant(_12, 0), 2);
(*_28) = _68.0 & _68.0;
(*_71) = _53 * _53;
_72.1 = [(*_31),(*_31),(*_31),(*_31)];
_23 = &_39;
(*_31) = _21 as i128;
_26 = _32 * _32;
match _38 {
10186160937710746340 => bb36,
_ => bb35
}
}
bb35 = {
(*_5) = _15 - _15;
(*_5) = _15 ^ _15;
(*_5) = _15 - _15;
Call((*_5) = core::intrinsics::transmute(_15), ReturnTo(bb4), UnwindUnreachable())
}
bb36 = {
(*_27) = core::ptr::addr_of_mut!(_2);
_70.0.fld2 = _20;
place!(Field::<i16>(Variant(_12, 0), 4)) = _63 as i16;
(*_71) = (*_23) as f32;
_66 = (_47.0, Move((*_27)), Field::<u16>(Variant(_12, 0), 1));
(*_28) = !_68.0;
match Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5).3 {
0 => bb1,
1 => bb35,
2 => bb32,
3 => bb7,
154744667998533081924141067781196576658 => bb37,
_ => bb9
}
}
bb37 = {
(*_71) = _53 * _53;
Goto(bb38)
}
bb38 = {
(*_5) = Field::<u16>(Variant(_12, 0), 1) as isize;
place!(Field::<i8>(Variant(_12, 0), 3)) = !_57;
(*_27) = core::ptr::addr_of_mut!(_2);
_17 = core::ptr::addr_of!(_81);
(*_71) = _63 as f32;
(*_31) = _50 | _50;
(*_17) = core::ptr::addr_of_mut!(_47.2);
(*_81) = Field::<u16>(Variant(_12, 0), 1);
match _38 {
0 => bb32,
1 => bb22,
2 => bb3,
3 => bb39,
4 => bb40,
10186160937710746340 => bb42,
_ => bb41
}
}
bb39 = {
_72.1 = [(*_31),(*_31),(*_31),(*_31)];
(*_71) = _45 as f32;
(*_31) = _25.fld1 as i128;
_16 = !_49;
(*_31) = -_50;
place!(Field::<i16>(Variant(_12, 0), 4)) = _66.0 & _66.0;
_20 = _70.0.fld2;
_68.0 = (*_5) < (*_5);
(*_27) = core::ptr::addr_of_mut!(_2);
(*_5) = Field::<isize>(Variant(_12, 0), 2);
(*_28) = _68.0 & _68.0;
(*_71) = _53 * _53;
_72.1 = [(*_31),(*_31),(*_31),(*_31)];
_23 = &_39;
(*_31) = _21 as i128;
_26 = _32 * _32;
match _38 {
10186160937710746340 => bb36,
_ => bb35
}
}
bb40 = {
_56 = -(*_5);
(*_28) = !true;
(*_28) = (*_5) >= (*_5);
_27 = core::ptr::addr_of_mut!(_47.1);
_56 = (*_5) << _52.2;
(*_5) = _56 - _35;
_50 = _38 as i128;
_45 = 2461188987_u32;
_57 = _52.2;
_57 = _42 * _42;
_24 = core::ptr::addr_of_mut!((*_27));
(*_31) = _50 | _50;
(*_24) = core::ptr::addr_of_mut!(_2);
(*_28) = (*_5) != (*_5);
_53 = _13 as f32;
(*_5) = _56 ^ _56;
(*_31) = _22 ^ _50;
_33 = _54;
_41 = [_11,_11];
(*_28) = _42 >= _52.2;
(*_28) = true;
(*_31) = _50 & _22;
Goto(bb24)
}
bb41 = {
(*_5) = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_5) = 9223372036854775807_isize;
(*_5) = false as isize;
(*_5) = -9223372036854775807_isize;
_20 = [_14,_14,_14,_14];
(*_5) = 9223372036854775807_isize;
RET = _20;
(*_5) = (-9223372036854775808_isize) << (*_6);
(*_5) = !9223372036854775807_isize;
(*_5) = !(-9223372036854775808_isize);
_32 = 917794730_u32 as f64;
_7 = (*_5) as i32;
_13 = -(-5579861748756079251_i64);
_37 = (_8,);
(*_5) = (-46_isize) & 89_isize;
(*_5) = (-9223372036854775808_isize) & 123_isize;
_6 = &_4;
_25.fld1 = 115_u8;
_13 = (-1666577692478850074_i64);
match _25.fld1 {
0 => bb1,
1 => bb9,
115 => bb14,
_ => bb3
}
}
bb42 = {
(*_81) = _11 - _63;
(*_5) = _56;
_72.1 = _43;
_68.2 = _52.3 as i8;
_81 = core::ptr::addr_of_mut!(_66.2);
(*_31) = _22 ^ _50;
_84 = Adt27::Variant0 { fld0: Move(_28),fld1: (*_81),fld2: (*_5),fld3: _42,fld4: Field::<i16>(Variant(_12, 0), 4),fld5: _52,fld6: _13 };
_66.0 = _47.0;
(*_27) = core::ptr::addr_of_mut!(_2);
(*_71) = _53 - _53;
(*_27) = core::ptr::addr_of_mut!(_2);
_54 = _33;
Goto(bb43)
}
bb43 = {
(*_31) = (*_5) as i128;
(*_27) = core::ptr::addr_of_mut!(_2);
(*_17) = core::ptr::addr_of_mut!((*_81));
_59 = &(*_6);
_88 = (*_71) * (*_71);
(*_5) = Field::<isize>(Variant(_84, 0), 2) >> (*_31);
_79 = -(*_71);
_37.0 = (*_59);
_58 = _68.0 & Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5).0;
_44 = Adt27::Variant1 { fld0: Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5),fld1: _51,fld2: (*_81) };
_68 = Field::<(bool, u64, i8, u128)>(Variant(_44, 1), 0);
_45 = (*_23) as u32;
_66.0 = -Field::<i16>(Variant(_12, 0), 4);
(*_5) = !_56;
_29 = [_45,_45,_45,_45,_45,_45];
_29 = [_45,_45,_45,_45,_45,_45];
(*_5) = _57 as isize;
_83 = _45 as f64;
(*_27) = core::ptr::addr_of_mut!(_2);
match _38 {
10186160937710746340 => bb44,
_ => bb33
}
}
bb44 = {
_86 = core::ptr::addr_of_mut!(_82);
(*_17) = core::ptr::addr_of_mut!((*_81));
_42 = _57 - _21;
(*_31) = _50;
(*_31) = _50 * _50;
(*_81) = !_47.2;
(*_17) = core::ptr::addr_of_mut!((*_81));
_44 = Move(_84);
(*_71) = -_88;
place!(Field::<isize>(Variant(_44, 0), 2)) = (*_5) * (*_5);
_74 = _33;
_24 = core::ptr::addr_of_mut!((*_27));
(*_17) = core::ptr::addr_of_mut!(_63);
_66 = (_47.0, Move((*_27)), (*_81));
(*_27) = core::ptr::addr_of_mut!(_2);
(*_27) = Move(_66.1);
_44 = Adt27::Variant1 { fld0: _52,fld1: _51,fld2: (*_81) };
match _38 {
0 => bb35,
1 => bb7,
2 => bb15,
10186160937710746340 => bb46,
_ => bb45
}
}
bb45 = {
_25.fld1 = 10_u8 << (*_5);
(*_5) = _11 as isize;
_6 = &_7;
_38 = _52.1;
_50 = !_22;
_23 = &_39;
_36 = [_51,_33,_51,_33];
_28 = core::ptr::addr_of!(_52.0);
(*_5) = -_35;
_53 = _32 as f32;
_49 = _52.1 as isize;
_37 = (_4,);
(*_28) = true;
_35 = (*_5);
_54 = _51;
_14 = _52.3;
(*_28) = false;
(*_28) = !false;
_31 = &mut _48;
Goto(bb19)
}
bb46 = {
_66.0 = _47.0 ^ _47.0;
place!(Field::<(bool, u64, i8, u128)>(Variant(_44, 1), 0)).3 = Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5).3;
(*_17) = core::ptr::addr_of_mut!(_63);
_61 = _79;
(*_27) = core::ptr::addr_of_mut!(_2);
(*_71) = _53;
_70.0.fld1 = core::ptr::addr_of_mut!(place!(Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5)).0);
(*_5) = _56 | Field::<isize>(Variant(_12, 0), 2);
(*_31) = _22 ^ _22;
_52.0 = _58;
(*_27) = core::ptr::addr_of_mut!(_2);
_66.2 = (*_81);
(*_71) = _83 as f32;
_85 = core::ptr::addr_of_mut!(_43);
place!(Field::<char>(Variant(_44, 1), 1)) = _33;
(*_27) = core::ptr::addr_of_mut!(_2);
(*_85) = [(*_31),(*_31),(*_31),(*_31)];
(*_31) = -_50;
(*_31) = _83 as i128;
Goto(bb47)
}
bb47 = {
_64 = &mut (*_17);
_71 = core::ptr::addr_of_mut!((*_71));
_47.2 = Field::<u16>(Variant(_44, 1), 2) & Field::<u16>(Variant(_12, 0), 1);
_52.1 = Field::<(bool, u64, i8, u128)>(Variant(_44, 1), 0).1 | Field::<(bool, u64, i8, u128)>(Variant(_12, 0), 5).1;
_90 = Field::<(bool, u64, i8, u128)>(Variant(_44, 1), 0).0 ^ Field::<(bool, u64, i8, u128)>(Variant(_44, 1), 0).0;
_20 = [_68.3,Field::<(bool, u64, i8, u128)>(Variant(_44, 1), 0).3,Field::<(bool, u64, i8, u128)>(Variant(_44, 1), 0).3,_52.3];
place!(Field::<u16>(Variant(_44, 1), 2)) = _47.2 << (*_5);
_50 = (*_31);
(*_31) = _22;
(*_64) = core::ptr::addr_of_mut!(_11);
_34 = &mut _44;
place!(Field::<(bool, u64, i8, u128)>(Variant((*_34), 1), 0)).2 = Field::<i8>(Variant(_12, 0), 3) ^ Field::<i8>(Variant(_12, 0), 3);
place!(Field::<(bool, u64, i8, u128)>(Variant((*_34), 1), 0)).2 = Field::<i64>(Variant(_12, 0), 6) as i8;
match _38 {
10186160937710746340 => bb49,
_ => bb48
}
}
bb48 = {
RET = [_14,_14,_14,_14];
(*_28) = (*_5) != (*_5);
match _38 {
0 => bb12,
1 => bb20,
2 => bb21,
10186160937710746340 => bb23,
_ => bb22
}
}
bb49 = {
(*_71) = -_79;
place!(Field::<(bool, u64, i8, u128)>(Variant((*_34), 1), 0)).2 = -_42;
place!(Field::<char>(Variant((*_34), 1), 1)) = _74;
_49 = (*_5) - (*_5);
(*_86) = Adt31::Variant1 { fld0: Field::<i8>(Variant(_12, 0), 3),fld1: (*_31) };
_74 = Field::<char>(Variant((*_34), 1), 1);
(*_31) = Field::<i128>(Variant((*_86), 1), 1);
place!(Field::<i128>(Variant((*_86), 1), 1)) = !(*_31);
place!(Field::<(bool, u64, i8, u128)>(Variant((*_34), 1), 0)).1 = _68.1;
place!(Field::<u16>(Variant((*_34), 1), 2)) = _66.2;
(*_31) = Field::<i128>(Variant((*_86), 1), 1) * Field::<i128>(Variant((*_86), 1), 1);
Goto(bb50)
}
bb50 = {
Call(_103 = dump_var(16_usize, 2_usize, Move(_2), 4_usize, Move(_4), 90_usize, Move(_90), 8_usize, Move(_8)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_103 = dump_var(16_usize, 10_usize, Move(_10), 11_usize, Move(_11), 13_usize, Move(_13), 39_usize, Move(_39)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_103 = dump_var(16_usize, 15_usize, Move(_15), 49_usize, Move(_49), 51_usize, Move(_51), 68_usize, Move(_68)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_103 = dump_var(16_usize, 46_usize, Move(_46), 29_usize, Move(_29), 52_usize, Move(_52), 41_usize, Move(_41)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_103 = dump_var(16_usize, 54_usize, Move(_54), 62_usize, Move(_62), 38_usize, Move(_38), 104_usize, _104), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: &'static usize,mut _2: i32,mut _3: &'static usize,mut _4: u128,mut _5: usize,mut _6: [i128; 4],mut _7: i128,mut _8: usize) -> i32 {
mir! {
type RET = i32;
let _9: *const *mut u16;
let _10: ();
let _11: ();
{
_7 = false as i128;
_8 = _5 ^ _5;
_1 = &_8;
_5 = (*_1) + (*_1);
_1 = &_5;
_1 = &_8;
RET = !_2;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(17_usize, 6_usize, Move(_6), 4_usize, Move(_4), 8_usize, Move(_8), 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: *mut [usize; 3],mut _2: isize,mut _3: &'static mut isize,mut _4: isize) -> (bool, u64, i8, u128) {
mir! {
type RET = (bool, u64, i8, u128);
let _5: [u128; 4];
let _6: bool;
let _7: Adt18;
let _8: isize;
let _9: Adt44;
let _10: (i16, *mut [usize; 3], u16);
let _11: *const *mut u16;
let _12: [u128; 4];
let _13: i64;
let _14: [char; 4];
let _15: bool;
let _16: u8;
let _17: &'static mut &'static usize;
let _18: i32;
let _19: bool;
let _20: &'static mut Adt57;
let _21: isize;
let _22: ();
let _23: ();
{
RET.1 = !3922422422258023813_u64;
RET.0 = false;
RET.0 = false;
_5 = [300273356187955978817046197946611941987_u128,219472492981365852371684002368248086701_u128,140590193106513626236651036651008317020_u128,259878434526898087208683272218697297980_u128];
_4 = _2;
_4 = -_2;
_2 = (-4946679315418212139_i64) as isize;
_9.fld1 = !187_u8;
_8 = _4;
RET.2 = (-61_i8) * (-96_i8);
RET.3 = 238088510503119865661432634177455469183_u128 | 87587409281640089505668986633359479863_u128;
RET = (false, 13801680845970327867_u64, 77_i8, 98011987503440859386197960208548475917_u128);
_5 = [RET.3,RET.3,RET.3,RET.3];
RET.0 = false;
_4 = _8 | _8;
_8 = _4 ^ _4;
_10.2 = (-21981_i16) as u16;
_8 = _4;
match RET.2 {
0 => bb1,
77 => bb3,
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
_10.0 = (-25483_i16);
match RET.1 {
0 => bb2,
1 => bb4,
2 => bb5,
13801680845970327867 => bb7,
_ => bb6
}
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
_6 = RET.0 ^ RET.0;
_10.1 = Move(_1);
Goto(bb8)
}
bb8 = {
_3 = &mut _8;
(*_3) = _4 >> RET.3;
(*_3) = 773323538_i32 as isize;
(*_3) = _4 ^ _4;
(*_3) = 44429938277427660040198062619311141074_i128 as isize;
_13 = !4338323683095418468_i64;
(*_3) = !_4;
(*_3) = _4 | _4;
_4 = _6 as isize;
(*_3) = _4 + _2;
_14 = ['\u{b0f37}','\u{e65c}','\u{d3646}','\u{ad2f6}'];
(*_3) = _6 as isize;
(*_3) = RET.0 as isize;
(*_3) = RET.1 as isize;
Goto(bb9)
}
bb9 = {
_12 = [RET.3,RET.3,RET.3,RET.3];
_1 = Move(_10.1);
match RET.1 {
0 => bb6,
1 => bb8,
13801680845970327867 => bb10,
_ => bb3
}
}
bb10 = {
_6 = RET.0 | RET.0;
(*_3) = RET.2 as isize;
_12 = _5;
_14 = ['\u{ca24f}','\u{5648f}','\u{3b954}','\u{41e8e}'];
_5 = _12;
_10.1 = Move(_1);
(*_3) = RET.3 as isize;
RET.0 = _6;
(*_3) = _4 + _4;
_15 = !RET.0;
(*_3) = RET.2 as isize;
_2 = -(*_3);
RET.3 = 154311394667269210552574970424746841816_u128 & 225401277872589645839899081287650420164_u128;
(*_3) = _2;
_3 = &mut _4;
_15 = _6 | RET.0;
_16 = !_9.fld1;
Goto(bb11)
}
bb11 = {
_5 = _12;
(*_3) = !_2;
_1 = Move(_10.1);
(*_3) = _2;
(*_3) = _6 as isize;
(*_3) = _2 << RET.1;
(*_3) = _2;
(*_3) = RET.3 as isize;
(*_3) = _2 + _2;
_13 = 1444351206005097672_i64 << (*_3);
_6 = RET.0;
_10.1 = Move(_1);
(*_3) = _2 << RET.1;
(*_3) = _2;
RET = (_6, 12688499974703005753_u64, (-21_i8), 154744667998533081924141067781196576658_u128);
(*_3) = 3118053693_u32 as isize;
_2 = !(*_3);
(*_3) = -_2;
(*_3) = _2 | _2;
_1 = Move(_10.1);
(*_3) = _10.2 as isize;
_5 = _12;
match RET.2 {
0 => bb4,
340282366920938463463374607431768211435 => bb12,
_ => bb9
}
}
bb12 = {
_14 = ['\u{b513e}','\u{65906}','\u{525e2}','\u{a0904}'];
_19 = _13 <= _13;
_12 = _5;
_10.1 = Move(_1);
_1 = Move(_10.1);
(*_3) = _2;
RET.0 = _19 == _19;
(*_3) = _2;
(*_3) = _2;
_9.fld1 = _16 * _16;
(*_3) = !_2;
(*_3) = !_2;
(*_3) = _2;
RET.0 = _19 & _19;
_10 = ((-13075_i16), Move(_1), 28127_u16);
(*_3) = _2 << _13;
RET.2 = (-41_i8) | 27_i8;
_6 = (*_3) != (*_3);
_12 = [RET.3,RET.3,RET.3,RET.3];
_10.0 = (-10744_i16);
(*_3) = _9.fld1 as isize;
Goto(bb13)
}
bb13 = {
Call(_22 = dump_var(18_usize, 15_usize, Move(_15), 12_usize, Move(_12), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_22 = dump_var(18_usize, 19_usize, Move(_19), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: Adt31,mut _2: &'static mut isize,mut _3: &'static mut i128,mut _4: *const Adt18) -> i64 {
mir! {
type RET = i64;
let _5: *const (i32,);
let _6: char;
let _7: u16;
let _8: &'static mut Adt57;
let _9: u64;
let _10: &'static mut Adt57;
let _11: *mut isize;
let _12: *const Adt18;
let _13: bool;
let _14: bool;
let _15: *mut &'static mut *mut u16;
let _16: *const &'static mut i128;
let _17: f64;
let _18: [u32; 6];
let _19: i64;
let _20: i8;
let _21: &'static &'static mut Adt57;
let _22: *mut isize;
let _23: (Adt23, u16, [i128; 4], &'static mut isize);
let _24: isize;
let _25: i128;
let _26: bool;
let _27: f64;
let _28: (*mut &'static mut Adt27, [i128; 4], Adt44, *const *mut u16);
let _29: bool;
let _30: (i32,);
let _31: i16;
let _32: bool;
let _33: [u16; 2];
let _34: &'static mut &'static usize;
let _35: &'static mut Adt57;
let _36: (i16, *mut [usize; 3], u16);
let _37: (i32,);
let _38: isize;
let _39: *mut &'static mut &'static usize;
let _40: *mut [i128; 4];
let _41: f32;
let _42: f64;
let _43: isize;
let _44: *mut &'static mut Adt27;
let _45: u128;
let _46: char;
let _47: Adt18;
let _48: u16;
let _49: &'static &'static mut Adt57;
let _50: f64;
let _51: *mut &'static mut Adt27;
let _52: u64;
let _53: bool;
let _54: (Adt23, u16, [i128; 4], &'static mut isize);
let _55: &'static &'static mut Adt57;
let _56: Adt57;
let _57: Adt38;
let _58: i128;
let _59: *const bool;
let _60: Adt27;
let _61: &'static &'static mut Adt57;
let _62: isize;
let _63: (Adt23, u16, [i128; 4], &'static mut isize);
let _64: *const *mut [usize; 3];
let _65: &'static &'static mut Adt57;
let _66: (bool, u64, i8, u128);
let _67: bool;
let _68: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _69: i16;
let _70: *const *mut [usize; 3];
let _71: isize;
let _72: bool;
let _73: isize;
let _74: [i128; 4];
let _75: bool;
let _76: bool;
let _77: &'static mut i32;
let _78: [i128; 4];
let _79: f32;
let _80: Adt23;
let _81: isize;
let _82: f32;
let _83: i128;
let _84: i16;
let _85: *const bool;
let _86: *mut &'static mut *mut u16;
let _87: [i128; 3];
let _88: u64;
let _89: &'static &'static mut Adt57;
let _90: &'static *const Adt18;
let _91: (bool, u64, i8, u128);
let _92: bool;
let _93: [usize; 3];
let _94: char;
let _95: Adt27;
let _96: &'static mut Adt57;
let _97: i64;
let _98: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _99: Adt27;
let _100: *mut [i128; 4];
let _101: [i32; 6];
let _102: bool;
let _103: Adt57;
let _104: (*const i8, *const Adt18, (i32,), &'static mut isize);
let _105: *mut u128;
let _106: *const *mut [usize; 3];
let _107: i8;
let _108: *mut u16;
let _109: *const *mut [usize; 3];
let _110: ();
let _111: ();
{
place!(Field::<i128>(Variant(_1, 1), 1)) = 125204130024705059482685994931655111981_i128;
place!(Field::<i8>(Variant(_1, 1), 0)) = 54_i8 | 48_i8;
RET = 6677361814336955575_i64 >> Field::<i128>(Variant(_1, 1), 1);
place!(Field::<i128>(Variant(_1, 1), 1)) = (-83327604225919589890779735531296943541_i128);
_6 = '\u{ff2d2}';
place!(Field::<i8>(Variant(_1, 1), 0)) = 100_i8;
place!(Field::<i8>(Variant(_1, 1), 0)) = 39_i8 | (-80_i8);
RET = 4255592594684981748_i64 + 6869504075710900752_i64;
_1 = Adt31::Variant1 { fld0: 12_i8,fld1: 75543357427538359404183264814157258918_i128 };
_1 = Adt31::Variant1 { fld0: (-39_i8),fld1: 168869874220210570473586952369065252062_i128 };
RET = (-3344647214728179757_i64) >> (-329216583_i32);
_7 = 26354_u16 >> RET;
_7 = 3639531440_u32 as u16;
_6 = '\u{c20c2}';
_7 = !32465_u16;
_9 = !9740730179486100264_u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = !(-120_i8);
_9 = 1576711798920110462_u64 ^ 14155589565312416168_u64;
_6 = '\u{a707e}';
place!(Field::<i8>(Variant(_1, 1), 0)) = (-100_i8) - 76_i8;
_1 = Adt31::Variant1 { fld0: 87_i8,fld1: 39478632455067634122162907344602020814_i128 };
_6 = '\u{87b62}';
_9 = 28324_i16 as u64;
Goto(bb1)
}
bb1 = {
_9 = 121_i8 as u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = 112_i8 - (-71_i8);
place!(Field::<i128>(Variant(_1, 1), 1)) = _7 as i128;
place!(Field::<i128>(Variant(_1, 1), 1)) = 78036039447883364838579692944716270019_i128;
Goto(bb2)
}
bb2 = {
_7 = _6 as u16;
Goto(bb3)
}
bb3 = {
_9 = false as u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = (-105_i8) | (-37_i8);
place!(Field::<i128>(Variant(_1, 1), 1)) = (-45162466231438430256955231975927242439_i128) ^ (-48330280649541752831753297128275553965_i128);
_6 = '\u{98091}';
_9 = 6313616767175105344_u64 + 11815452080944138364_u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = (-58_i8) >> Field::<i128>(Variant(_1, 1), 1);
_3 = &mut place!(Field::<i128>(Variant(_1, 1), 1));
RET = (-329899178420537587_i64) + 1677242281768192026_i64;
(*_3) = !(-11792144153619027307732197708043544232_i128);
(*_3) = 145407059603400539820774994965741822112_i128 * (-144004588343363192397335124126221360192_i128);
(*_3) = -145958633480464507833807343261063281346_i128;
(*_3) = -(-164052093230125060051983694917531019070_i128);
(*_3) = (-97409118250684362047168453608085080136_i128) & (-114494643358854690862363194255565290102_i128);
(*_3) = !16256306285451019523002549470330801078_i128;
(*_3) = 233_u8 as i128;
_13 = false & false;
Goto(bb4)
}
bb4 = {
(*_3) = (-74000203811975076221691853520682870361_i128) + 98322703681866955492039580748212536124_i128;
(*_3) = -97742790455571536607217313465341100437_i128;
_14 = (*_3) >= (*_3);
(*_3) = (-55971448052824043242705383107285074973_i128);
match (*_3) {
0 => bb1,
1 => bb2,
2 => bb3,
284310918868114420220669224324483136483 => bb6,
_ => bb5
}
}
bb5 = {
_7 = _6 as u16;
Goto(bb3)
}
bb6 = {
(*_3) = (-88224027765950720080888434233075696756_i128) - 60333383260385784871303724337055325513_i128;
(*_3) = 86390028458821142616653162204841001193_i128 - 101336886251140502935994126882770634863_i128;
(*_3) = 99699922467691892741452900352022205220_i128;
_16 = core::ptr::addr_of!(_3);
_19 = RET >> (*_3);
(*_3) = -65155356708234571947700548658942128381_i128;
(*_3) = -(-59291647186509357447578227597480538585_i128);
(*_3) = 57_u8 as i128;
(*_3) = 18931978493650458180026547822576457279_i128;
(*_3) = 154055876148467189887870415132380605442_i128 >> _7;
(*_3) = 165315606391156087963997707754510194236_i128;
match (*_3) {
0 => bb4,
165315606391156087963997707754510194236 => bb8,
_ => bb7
}
}
bb7 = {
_9 = 121_i8 as u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = 112_i8 - (-71_i8);
place!(Field::<i128>(Variant(_1, 1), 1)) = _7 as i128;
place!(Field::<i128>(Variant(_1, 1), 1)) = 78036039447883364838579692944716270019_i128;
Goto(bb2)
}
bb8 = {
_6 = '\u{dfb2f}';
(*_3) = RET as i128;
(*_3) = (-2720944_i32) as i128;
(*_3) = !(-19012507244785729331771153584636120740_i128);
_20 = 167130429_u32 as i8;
(*_3) = 121698237309904578803881831361887099063_i128 >> _7;
(*_3) = _9 as i128;
(*_3) = (-149587927330067329980008886906552418808_i128) << _7;
_13 = !_14;
(*_3) = 168613670071658939821965480931473845209_i128 ^ 25974879654339314448350020477965968653_i128;
_6 = '\u{6d98}';
(*_3) = 91874641121101903022299402955745598701_i128;
_17 = 5_usize as f64;
(*_3) = -(-143586215056968508300611183971021127180_i128);
Goto(bb9)
}
bb9 = {
_22 = core::ptr::addr_of_mut!(_24);
(*_22) = (-9223372036854775808_isize);
_23.1 = _7 | _7;
_20 = -50_i8;
_23.1 = !_7;
(*_22) = !(-9223372036854775808_isize);
(*_3) = 13087831714620277829105486213477720493_i128 ^ (-85418967941808326933563236756994916023_i128);
_16 = core::ptr::addr_of!((*_16));
(*_22) = 59_isize;
(*_22) = RET as isize;
(*_22) = (-9223372036854775808_isize) & (-9223372036854775808_isize);
(*_3) = (-32727188386939067209516035727451187653_i128) - 95897005107038681373407767957042043038_i128;
(*_3) = -(-43049997858316229326460307824245235386_i128);
(*_22) = 5_isize & (-9223372036854775808_isize);
(*_22) = !(-9223372036854775808_isize);
(*_22) = 9223372036854775807_isize;
(*_3) = -17295555708615673339691468593323615278_i128;
_2 = &mut (*_22);
_26 = _13 ^ _14;
(*_3) = (-45486484656194753899557092318594887846_i128);
(*_2) = -(-120_isize);
(*_2) = 51_isize ^ (-9223372036854775808_isize);
(*_3) = 112472038070536053160045674043765795291_i128 & 152091654430095554521037233064541961484_i128;
(*_3) = (-158389657747682300705797075262635672540_i128) ^ 149085271120104729623671123754837472947_i128;
_20 = (-39_i8) | (-75_i8);
(*_2) = 9223372036854775807_isize;
(*_2) = _13 as isize;
Goto(bb10)
}
bb10 = {
_25 = -(*_3);
(*_2) = (-9223372036854775808_isize);
_3 = &mut _25;
(*_2) = 154157669937284823402860442670379447795_u128 as isize;
(*_2) = !(-50_isize);
_30 = (1041373118_i32,);
_27 = _17 + _17;
(*_2) = 144_u8 as isize;
(*_3) = (-100305458324308141010618558726145959222_i128) | (-13733745250682791215432367839256526521_i128);
_27 = _17 * _17;
(*_3) = 105199889619188017289464029449093402571_i128;
(*_3) = -115477939247022261321047923284171821761_i128;
_16 = core::ptr::addr_of!((*_16));
(*_3) = 128739689318548330840400995808976163706_i128;
(*_2) = !9223372036854775807_isize;
(*_2) = !33_isize;
_11 = Move(_22);
Goto(bb11)
}
bb11 = {
(*_2) = _30.0 as isize;
(*_3) = 5414152195315717432543303094392792800_i128 ^ 15580272057214877560382304431920437896_i128;
(*_2) = (-9223372036854775808_isize);
_32 = _17 != _27;
(*_3) = 29636437923861204740599715654522593241_i128 - 159631222800102206131372688322511077960_i128;
_28.2.fld1 = 190_u8 + 133_u8;
_23.2 = [(*_3),(*_3),(*_3),(*_3)];
(*_2) = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_23.1 = _7 - _7;
_31 = 6277_i16 << _19;
_7 = _23.1;
(*_2) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_30 = (1065291561_i32,);
(*_2) = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_19 = RET - RET;
(*_2) = (-9223372036854775808_isize) << (*_3);
_20 = 20_i8;
(*_2) = !9223372036854775807_isize;
(*_3) = 143038650155301755519559763603293361397_i128 * 28752648076200940568937278629456773753_i128;
_23.3 = Move(_2);
Goto(bb12)
}
bb12 = {
_11 = core::ptr::addr_of_mut!(_38);
_23.2 = [(*_3),(*_3),(*_3),(*_3)];
(*_11) = _30.0 as isize;
(*_3) = (-76242989922133150718049990201685047912_i128);
_5 = core::ptr::addr_of!(_37);
_26 = !_14;
(*_5).0 = _30.0 >> (*_3);
(*_5).0 = _30.0 | _30.0;
(*_5) = _30;
_5 = core::ptr::addr_of!((*_5));
_9 = 17209188816450823483_u64 & 8284342484297714702_u64;
RET = _19 * _19;
(*_3) = -161622462565029975812544427964539382049_i128;
(*_11) = -9223372036854775807_isize;
(*_5).0 = RET as i32;
(*_5) = _30;
(*_5).0 = _30.0 - _30.0;
(*_5).0 = -_30.0;
(*_11) = 117_isize;
RET = _19 ^ _19;
(*_11) = -(-107_isize);
(*_3) = 77563681510044119018393566754848130043_i128;
(*_11) = 312192512425740543305792589067099603796_u128 as isize;
(*_5).0 = _28.2.fld1 as i32;
(*_11) = 9223372036854775807_isize | 73_isize;
Call((*_11) = core::intrinsics::transmute(_19), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_3) = !159576735812437195769556205961487008028_i128;
(*_5) = (_30.0,);
(*_5) = (_30.0,);
(*_11) = !(-59_isize);
(*_11) = (-13_isize);
(*_11) = 2_usize as isize;
(*_3) = (-104110772928581740866788919226897644901_i128) | 20239675774660468448662002431021176815_i128;
(*_5).0 = _30.0;
(*_3) = (-88594729421396141769512579811537495366_i128);
Call(_30.0 = core::intrinsics::bswap(_37.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_28.1 = _23.2;
(*_11) = (-61_isize);
_30 = ((*_5).0,);
_33 = [_23.1,_7];
(*_5) = _30;
(*_5).0 = !_30.0;
(*_5) = (_30.0,);
_26 = _32 ^ _14;
_42 = _17 - _27;
(*_5) = _30;
_42 = -_27;
(*_5).0 = _30.0;
(*_3) = 87787553167818623209793028479617295052_i128;
(*_11) = (-9223372036854775808_isize) | (-9223372036854775808_isize);
(*_3) = (-148147149908834528686872433261747968147_i128);
_17 = _27 + _27;
Goto(bb15)
}
bb15 = {
_43 = (*_11);
(*_3) = -(-94373074280251350235045443056196371278_i128);
(*_3) = -(-69384863951408687726474933741874792263_i128);
_41 = _27 as f32;
_40 = core::ptr::addr_of_mut!(_28.1);
(*_3) = 160367902588295039978958980810333233157_i128 << (*_11);
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
(*_5).0 = _30.0;
(*_11) = -_43;
match (*_5).0 {
0 => bb11,
1 => bb10,
2 => bb3,
3 => bb16,
4 => bb17,
1065291561 => bb19,
_ => bb18
}
}
bb16 = {
_9 = 121_i8 as u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = 112_i8 - (-71_i8);
place!(Field::<i128>(Variant(_1, 1), 1)) = _7 as i128;
place!(Field::<i128>(Variant(_1, 1), 1)) = 78036039447883364838579692944716270019_i128;
Goto(bb2)
}
bb17 = {
(*_3) = !159576735812437195769556205961487008028_i128;
(*_5) = (_30.0,);
(*_5) = (_30.0,);
(*_11) = !(-59_isize);
(*_11) = (-13_isize);
(*_11) = 2_usize as isize;
(*_3) = (-104110772928581740866788919226897644901_i128) | 20239675774660468448662002431021176815_i128;
(*_5).0 = _30.0;
(*_3) = (-88594729421396141769512579811537495366_i128);
Call(_30.0 = core::intrinsics::bswap(_37.0), ReturnTo(bb14), UnwindUnreachable())
}
bb18 = {
_7 = _6 as u16;
Goto(bb3)
}
bb19 = {
_29 = (*_3) < (*_3);
_36.0 = -_31;
_9 = !8222325180092523071_u64;
(*_40) = _23.2;
_12 = core::ptr::addr_of!(_47);
match (*_5).0 {
0 => bb11,
1 => bb13,
2 => bb18,
3 => bb16,
4 => bb5,
1065291561 => bb21,
_ => bb20
}
}
bb20 = {
_9 = 121_i8 as u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = 112_i8 - (-71_i8);
place!(Field::<i128>(Variant(_1, 1), 1)) = _7 as i128;
place!(Field::<i128>(Variant(_1, 1), 1)) = 78036039447883364838579692944716270019_i128;
Goto(bb2)
}
bb21 = {
(*_11) = -_43;
(*_5).0 = _30.0;
(*_5).0 = _30.0 | _30.0;
(*_40) = _23.2;
(*_12) = Adt18::Variant0 { fld0: _26,fld1: 4_usize,fld2: _41,fld3: _19,fld4: _9 };
(*_12) = Adt18::Variant0 { fld0: _14,fld1: 9303613686128793442_usize,fld2: _41,fld3: RET,fld4: _9 };
_43 = (*_11);
_46 = _6;
place!(Field::<i64>(Variant((*_12), 0), 3)) = RET & _19;
place!(Field::<usize>(Variant((*_12), 0), 1)) = !7725376902233062429_usize;
place!(Field::<usize>(Variant((*_12), 0), 1)) = !6888352369028877306_usize;
(*_11) = _43 * _43;
place!(Field::<bool>(Variant((*_12), 0), 0)) = Field::<i64>(Variant((*_12), 0), 3) == Field::<i64>(Variant((*_12), 0), 3);
place!(Field::<i64>(Variant((*_12), 0), 3)) = RET | _19;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
place!(Field::<usize>(Variant((*_12), 0), 1)) = 1999888788097124572_usize >> Field::<i64>(Variant(_47, 0), 3);
(*_5).0 = (*_3) as i32;
_41 = Field::<f32>(Variant((*_12), 0), 2) + Field::<f32>(Variant((*_12), 0), 2);
place!(Field::<bool>(Variant(_47, 0), 0)) = _13 < _13;
(*_3) = -86962751419348178956155411990720920778_i128;
(*_12) = Adt18::Variant0 { fld0: _26,fld1: 0_usize,fld2: _41,fld3: RET,fld4: _9 };
Goto(bb22)
}
bb22 = {
(*_5).0 = _30.0;
Goto(bb23)
}
bb23 = {
(*_11) = _43 & _43;
(*_11) = _43 ^ _43;
place!(Field::<u64>(Variant((*_12), 0), 4)) = _23.1 as u64;
place!(Field::<bool>(Variant((*_12), 0), 0)) = !_14;
(*_11) = _7 as isize;
place!(Field::<usize>(Variant((*_12), 0), 1)) = 1_usize >> Field::<i64>(Variant((*_12), 0), 3);
place!(Field::<bool>(Variant((*_12), 0), 0)) = _29;
(*_11) = _28.2.fld1 as isize;
(*_3) = 166480943874887495027765835917672248114_i128;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
(*_5) = _30;
place!(Field::<bool>(Variant((*_12), 0), 0)) = _13;
_5 = core::ptr::addr_of!((*_5));
(*_11) = _43 * _43;
(*_3) = -(-22308179461679021581679334017380881407_i128);
place!(Field::<u64>(Variant((*_12), 0), 4)) = _9 * _9;
(*_5) = (_30.0,);
(*_11) = !_43;
place!(Field::<i64>(Variant((*_12), 0), 3)) = RET << Field::<usize>(Variant((*_12), 0), 1);
place!(Field::<bool>(Variant((*_12), 0), 0)) = !_29;
match (*_5).0 {
1065291561 => bb25,
_ => bb24
}
}
bb24 = {
_29 = (*_3) < (*_3);
_36.0 = -_31;
_9 = !8222325180092523071_u64;
(*_40) = _23.2;
_12 = core::ptr::addr_of!(_47);
match (*_5).0 {
0 => bb11,
1 => bb13,
2 => bb18,
3 => bb16,
4 => bb5,
1065291561 => bb21,
_ => bb20
}
}
bb25 = {
(*_11) = -_43;
place!(Field::<i64>(Variant((*_12), 0), 3)) = _19 * RET;
place!(Field::<f32>(Variant((*_12), 0), 2)) = _41;
_4 = core::ptr::addr_of!((*_12));
place!(Field::<f32>(Variant((*_4), 0), 2)) = _41 * _41;
place!(Field::<usize>(Variant((*_12), 0), 1)) = 4480519750740307673_usize;
_14 = Field::<bool>(Variant((*_12), 0), 0) | Field::<bool>(Variant((*_12), 0), 0);
place!(Field::<u64>(Variant((*_4), 0), 4)) = _9 >> Field::<i64>(Variant((*_12), 0), 3);
(*_3) = 111226553678606355209359872471975915092_i128;
_13 = Field::<bool>(Variant(_47, 0), 0);
_57.fld1 = -_17;
(*_12) = Adt18::Variant0 { fld0: _29,fld1: 3_usize,fld2: _41,fld3: RET,fld4: _9 };
place!(Field::<u64>(Variant((*_12), 0), 4)) = _46 as u64;
(*_12) = Adt18::Variant0 { fld0: _13,fld1: 0_usize,fld2: _41,fld3: RET,fld4: _9 };
place!(Field::<usize>(Variant(_47, 0), 1)) = 1_usize ^ 7_usize;
_23.0 = Adt23::Variant1 { fld0: Move(_12),fld1: _6,fld2: (*_11),fld3: _20,fld4: 151451356514273102028853401888680323692_u128,fld5: (*_3) };
(*_3) = !Field::<i128>(Variant(_23.0, 1), 5);
_54.1 = _7 + _7;
_56.fld2 = [104744261913703513910679278188534766319_u128,182553883943361808561077510143887146587_u128,317016346417833161972217432312569961437_u128,197753338456973733352934866967595880286_u128];
Call((*_3) = core::intrinsics::transmute(Field::<i128>(Variant(_23.0, 1), 5)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
(*_3) = Field::<i128>(Variant(_23.0, 1), 5) >> Field::<i8>(Variant(_23.0, 1), 3);
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
(*_40) = _23.2;
_20 = Field::<i8>(Variant(_23.0, 1), 3) >> (*_3);
_63.3 = &mut (*_11);
match (*_5).0 {
0 => bb16,
1065291561 => bb28,
_ => bb27
}
}
bb27 = {
_28.1 = _23.2;
(*_11) = (-61_isize);
_30 = ((*_5).0,);
_33 = [_23.1,_7];
(*_5) = _30;
(*_5).0 = !_30.0;
(*_5) = (_30.0,);
_26 = _32 ^ _14;
_42 = _17 - _27;
(*_5) = _30;
_42 = -_27;
(*_5).0 = _30.0;
(*_3) = 87787553167818623209793028479617295052_i128;
(*_11) = (-9223372036854775808_isize) | (-9223372036854775808_isize);
(*_3) = (-148147149908834528686872433261747968147_i128);
_17 = _27 + _27;
Goto(bb15)
}
bb28 = {
(*_40) = _23.2;
_57.fld5 = 224730352835024132990669424557298645566_u128 & 116396825455953033837331162248586552445_u128;
(*_16) = &mut place!(Field::<i128>(Variant(_23.0, 1), 5));
_12 = core::ptr::addr_of!(_47);
place!(Field::<f32>(Variant((*_12), 0), 2)) = _41;
_22 = core::ptr::addr_of_mut!(_43);
(*_3) = !(-27812316106795822460469520732233304943_i128);
(*_22) = 9223372036854775807_isize ^ (-97_isize);
(*_12) = Adt18::Variant0 { fld0: _29,fld1: 2703101046286816391_usize,fld2: _41,fld3: RET,fld4: _9 };
Goto(bb29)
}
bb29 = {
(*_22) = (-9223372036854775808_isize) >> Field::<i64>(Variant((*_12), 0), 3);
place!(Field::<u64>(Variant((*_12), 0), 4)) = _9 - _9;
_37.0 = !_30.0;
place!(Field::<bool>(Variant((*_12), 0), 0)) = Field::<f32>(Variant((*_12), 0), 2) >= Field::<f32>(Variant((*_12), 0), 2);
_56.fld1 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant((*_12), 0), 0)));
(*_5).0 = _30.0 + _30.0;
_68.0 = core::ptr::addr_of!(_66.2);
place!(Field::<usize>(Variant(_47, 0), 1)) = 3_usize & 4_usize;
_20 = !(-62_i8);
place!(Field::<i64>(Variant((*_12), 0), 3)) = -_19;
_63.0 = Adt23::Variant0 { fld0: Field::<bool>(Variant((*_12), 0), 0),fld1: (*_3),fld2: (*_12),fld3: Field::<u64>(Variant((*_12), 0), 4),fld4: Field::<f32>(Variant((*_12), 0), 2) };
(*_12) = Field::<Adt18>(Variant(_63.0, 0), 2);
_56.fld0.fld0 = Adt23::Variant0 { fld0: Field::<bool>(Variant((*_12), 0), 0),fld1: (*_3),fld2: (*_12),fld3: Field::<u64>(Variant((*_12), 0), 4),fld4: Field::<f32>(Variant((*_12), 0), 2) };
place!(Field::<bool>(Variant(_56.fld0.fld0, 0), 0)) = Field::<f32>(Variant((*_12), 0), 2) == Field::<f32>(Variant((*_12), 0), 2);
place!(Field::<f32>(Variant((*_12), 0), 2)) = _41 - Field::<f32>(Variant(Field::<Adt18>(Variant(_56.fld0.fld0, 0), 2), 0), 2);
(*_5).0 = _30.0;
(*_22) = (-9_isize) >> (*_3);
place!(Field::<bool>(Variant(_63.0, 0), 0)) = Field::<bool>(Variant((*_12), 0), 0) | Field::<bool>(Variant((*_12), 0), 0);
(*_16) = &mut place!(Field::<i128>(Variant(_63.0, 0), 1));
(*_3) = _46 as i128;
place!(Field::<f32>(Variant((*_12), 0), 2)) = Field::<f32>(Variant(_56.fld0.fld0, 0), 4);
place!(Field::<f32>(Variant(place!(Field::<Adt18>(Variant(_56.fld0.fld0, 0), 2)), 0), 2)) = Field::<f32>(Variant((*_12), 0), 2) + Field::<f32>(Variant((*_12), 0), 2);
place!(Field::<u64>(Variant((*_12), 0), 4)) = Field::<u64>(Variant(Field::<Adt18>(Variant(_56.fld0.fld0, 0), 2), 0), 4) * Field::<u64>(Variant(_56.fld0.fld0, 0), 3);
match (*_5).0 {
0 => bb11,
1 => bb15,
2 => bb10,
3 => bb21,
4 => bb19,
5 => bb24,
6 => bb30,
1065291561 => bb32,
_ => bb31
}
}
bb30 = {
_9 = false as u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = (-105_i8) | (-37_i8);
place!(Field::<i128>(Variant(_1, 1), 1)) = (-45162466231438430256955231975927242439_i128) ^ (-48330280649541752831753297128275553965_i128);
_6 = '\u{98091}';
_9 = 6313616767175105344_u64 + 11815452080944138364_u64;
place!(Field::<i8>(Variant(_1, 1), 0)) = (-58_i8) >> Field::<i128>(Variant(_1, 1), 1);
_3 = &mut place!(Field::<i128>(Variant(_1, 1), 1));
RET = (-329899178420537587_i64) + 1677242281768192026_i64;
(*_3) = !(-11792144153619027307732197708043544232_i128);
(*_3) = 145407059603400539820774994965741822112_i128 * (-144004588343363192397335124126221360192_i128);
(*_3) = -145958633480464507833807343261063281346_i128;
(*_3) = -(-164052093230125060051983694917531019070_i128);
(*_3) = (-97409118250684362047168453608085080136_i128) & (-114494643358854690862363194255565290102_i128);
(*_3) = !16256306285451019523002549470330801078_i128;
(*_3) = 233_u8 as i128;
_13 = false & false;
Goto(bb4)
}
bb31 = {
(*_3) = (-88224027765950720080888434233075696756_i128) - 60333383260385784871303724337055325513_i128;
(*_3) = 86390028458821142616653162204841001193_i128 - 101336886251140502935994126882770634863_i128;
(*_3) = 99699922467691892741452900352022205220_i128;
_16 = core::ptr::addr_of!(_3);
_19 = RET >> (*_3);
(*_3) = -65155356708234571947700548658942128381_i128;
(*_3) = -(-59291647186509357447578227597480538585_i128);
(*_3) = 57_u8 as i128;
(*_3) = 18931978493650458180026547822576457279_i128;
(*_3) = 154055876148467189887870415132380605442_i128 >> _7;
(*_3) = 165315606391156087963997707754510194236_i128;
match (*_3) {
0 => bb4,
165315606391156087963997707754510194236 => bb8,
_ => bb7
}
}
bb32 = {
(*_16) = &mut place!(Field::<i128>(Variant(_56.fld0.fld0, 0), 1));
(*_5).0 = _30.0 + _30.0;
_29 = Field::<f32>(Variant((*_12), 0), 2) >= Field::<f32>(Variant((*_12), 0), 2);
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
_41 = -Field::<f32>(Variant((*_12), 0), 2);
place!(Field::<bool>(Variant((*_12), 0), 0)) = Field::<f32>(Variant((*_12), 0), 2) != Field::<f32>(Variant((*_12), 0), 2);
_46 = _6;
(*_22) = 95_isize ^ (-38_isize);
place!(Field::<usize>(Variant((*_12), 0), 1)) = 11669144023627498206_usize ^ 2_usize;
place!(Field::<f32>(Variant((*_12), 0), 2)) = -_41;
(*_3) = !46332823840250701732438606558647125446_i128;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
(*_5) = (_30.0,);
_66.2 = _20;
place!(Field::<bool>(Variant((*_12), 0), 0)) = _29;
(*_5) = _30;
_53 = Field::<bool>(Variant((*_12), 0), 0) == _29;
Goto(bb33)
}
bb33 = {
_54.3 = &mut (*_22);
(*_12) = Adt18::Variant0 { fld0: _32,fld1: 1_usize,fld2: _41,fld3: RET,fld4: _9 };
_62 = (-84_isize) & (-9223372036854775808_isize);
_57.fld3 = core::ptr::addr_of_mut!(_36.2);
_57.fld2 = core::ptr::addr_of!(place!(Field::<bool>(Variant((*_12), 0), 0)));
(*_5) = (_30.0,);
(*_3) = (-18780575148056742198122301206639803817_i128);
_30.0 = -(*_5).0;
(*_5).0 = _30.0;
_50 = _57.fld1 * _57.fld1;
place!(Field::<i64>(Variant((*_12), 0), 3)) = RET << (*_5).0;
place!(Field::<usize>(Variant((*_12), 0), 1)) = !2618161606545408708_usize;
(*_5) = (_30.0,);
(*_3) = _62 as i128;
place!(Field::<u64>(Variant((*_12), 0), 4)) = _9 & _9;
_52 = !Field::<u64>(Variant((*_12), 0), 4);
place!(Field::<i64>(Variant((*_12), 0), 3)) = -_19;
_68.2 = ((*_5).0,);
place!(Field::<f32>(Variant((*_12), 0), 2)) = _41 - _41;
_13 = Field::<f32>(Variant((*_12), 0), 2) != Field::<f32>(Variant(_47, 0), 2);
_57.fld7 = [_36.0,_31,_31];
_70 = core::ptr::addr_of!(_36.1);
(*_12) = Adt18::Variant1 { fld0: _62,fld1: _57.fld7 };
(*_5).0 = _68.2.0 & _68.2.0;
_54.1 = _7 | _7;
(*_5) = (_68.2.0,);
place!(Field::<isize>(Variant((*_12), 1), 0)) = !_62;
Goto(bb34)
}
bb34 = {
_17 = _41 as f64;
(*_3) = (-162936021413132295293547104255679317512_i128) ^ 132643684716168883534227014380388129796_i128;
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = _57.fld7;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
_45 = _57.fld5 | _57.fld5;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
place!(Field::<isize>(Variant((*_12), 1), 0)) = !_62;
_16 = core::ptr::addr_of!((*_16));
Goto(bb35)
}
bb35 = {
_48 = _57.fld1 as u16;
(*_5).0 = _68.2.0;
_9 = _52;
(*_12) = Adt18::Variant1 { fld0: _62,fld1: _57.fld7 };
place!(Field::<isize>(Variant((*_12), 1), 0)) = _62 << (*_5).0;
_72 = _29;
(*_12) = Adt18::Variant0 { fld0: _72,fld1: 3_usize,fld2: _41,fld3: RET,fld4: _52 };
(*_5) = (_68.2.0,);
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
place!(Field::<f32>(Variant((*_12), 0), 2)) = -_41;
place!(Field::<usize>(Variant((*_12), 0), 1)) = !0_usize;
(*_3) = 2945364648003728060263932312672476391_i128;
(*_5).0 = !_68.2.0;
_48 = _54.1 - _54.1;
place!(Field::<u64>(Variant((*_12), 0), 4)) = _52 * _52;
_76 = _13;
(*_3) = 96177467638083268146651618455202603355_i128 >> Field::<i64>(Variant((*_12), 0), 3);
Goto(bb36)
}
bb36 = {
_77 = &mut (*_5).0;
_53 = !Field::<bool>(Variant((*_12), 0), 0);
place!(Field::<usize>(Variant((*_12), 0), 1)) = 6_usize;
_57.fld3 = core::ptr::addr_of_mut!(_54.1);
_30.0 = (*_77) | (*_77);
(*_77) = _68.2.0 >> _54.1;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
place!(Field::<u64>(Variant((*_12), 0), 4)) = _52 << (*_3);
place!(Field::<u64>(Variant((*_12), 0), 4)) = _52 - _9;
(*_12) = Adt18::Variant0 { fld0: _76,fld1: 5540021536140992979_usize,fld2: _41,fld3: _19,fld4: _9 };
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
place!(Field::<f32>(Variant((*_12), 0), 2)) = -_41;
place!(Field::<f32>(Variant((*_12), 0), 2)) = _41 + _41;
_62 = 6_usize as isize;
(*_12) = Adt18::Variant0 { fld0: _29,fld1: 1041720403804614181_usize,fld2: _41,fld3: RET,fld4: _52 };
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
place!(Field::<u64>(Variant((*_12), 0), 4)) = _9 ^ _52;
(*_3) = (-3611582436781075871672333705101339659_i128) | 168022359215888766016240322797210052331_i128;
(*_12) = Adt18::Variant1 { fld0: _62,fld1: _57.fld7 };
(*_3) = 168938349283938176232043853336937720231_i128;
(*_3) = !52638302287904013686871133651036675314_i128;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
place!(Field::<isize>(Variant((*_12), 1), 0)) = _62 ^ _62;
_2 = Move(_54.3);
Goto(bb37)
}
bb37 = {
_54.2 = (*_40);
_54.1 = _48 | _48;
_74 = [(*_3),(*_3),(*_3),(*_3)];
_83 = (*_3) ^ (*_3);
(*_16) = &mut _83;
(*_77) = 3_usize as i32;
(*_12) = Adt18::Variant1 { fld0: _62,fld1: _57.fld7 };
_64 = core::ptr::addr_of!((*_70));
(*_3) = 116823216383360289974394761963310139506_i128 + (-169662544004023429400577954122856420448_i128);
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = _57.fld7;
_26 = _13;
_66.1 = _9 * _52;
_82 = _41;
(*_12) = Adt18::Variant0 { fld0: _53,fld1: 18230411663124169327_usize,fld2: _41,fld3: _19,fld4: _9 };
_82 = Field::<f32>(Variant((*_12), 0), 2) + Field::<f32>(Variant((*_12), 0), 2);
_78 = (*_40);
(*_12) = Adt18::Variant0 { fld0: _29,fld1: 7_usize,fld2: _82,fld3: _19,fld4: _52 };
_40 = core::ptr::addr_of_mut!((*_40));
_52 = Field::<u64>(Variant((*_12), 0), 4) | Field::<u64>(Variant((*_12), 0), 4);
_57.fld1 = _17 - _17;
_28.3 = core::ptr::addr_of!(_57.fld3);
(*_12) = Adt18::Variant1 { fld0: _62,fld1: _57.fld7 };
place!(Field::<isize>(Variant((*_12), 1), 0)) = _62 >> (*_3);
_36.0 = RET as i16;
Goto(bb38)
}
bb38 = {
(*_3) = !(-133498178651508970287875971143611048641_i128);
_46 = _6;
_68.3 = &mut place!(Field::<isize>(Variant((*_12), 1), 0));
_57.fld6 = RET;
_78 = [(*_3),(*_3),(*_3),(*_3)];
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
_58 = !(*_3);
(*_3) = _54.1 as i128;
Goto(bb39)
}
bb39 = {
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = [_36.0,_31,_31];
_84 = !_36.0;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
(*_16) = &mut _58;
Goto(bb40)
}
bb40 = {
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = [_31,_84,_36.0];
(*_77) = _30.0 >> _19;
(*_77) = !_30.0;
(*_40) = _54.2;
_66 = (_26, _52, _20, _45);
_57.fld3 = core::ptr::addr_of_mut!(_36.2);
(*_77) = -_68.2.0;
(*_3) = 28630857898463013228133777540593530004_i128 * (-52767000106065884753231620292851111587_i128);
(*_70) = core::ptr::addr_of_mut!(_93);
_91.2 = -_66.2;
_19 = -RET;
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = [_84,_84,_84];
_81 = _62 << (*_3);
(*_3) = 158529249806699110780805436701345669792_i128 ^ 36928974004521218878623252721403481047_i128;
_46 = _6;
(*_70) = core::ptr::addr_of_mut!(_93);
_93 = [2_usize,9688952843936094942_usize,1_usize];
(*_70) = core::ptr::addr_of_mut!(_93);
_19 = RET;
RET = -_57.fld6;
Call(_30.0 = core::intrinsics::bswap((*_77)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
_50 = RET as f64;
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = [_31,_84,_31];
(*_77) = _68.2.0 - _30.0;
_57.fld5 = _45;
_40 = core::ptr::addr_of_mut!((*_40));
(*_70) = core::ptr::addr_of_mut!(_93);
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = [_84,_31,_84];
_48 = !_54.1;
(*_40) = _54.2;
(*_70) = core::ptr::addr_of_mut!(_93);
(*_77) = -_30.0;
_59 = core::ptr::addr_of!(_91.0);
_91.0 = !_76;
_80 = Adt23::Variant1 { fld0: Move(_4),fld1: _46,fld2: _62,fld3: _20,fld4: _57.fld5,fld5: (*_3) };
(*_77) = _30.0 * _68.2.0;
_52 = _66.1;
Goto(bb42)
}
bb42 = {
_28.2.fld0 = Move(_80);
(*_3) = !Field::<i128>(Variant(_28.2.fld0, 1), 5);
(*_3) = -Field::<i128>(Variant(_28.2.fld0, 1), 5);
(*_70) = core::ptr::addr_of_mut!(_93);
(*_59) = !_32;
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = [_84,_84,_84];
_88 = _66.1 * _66.1;
_98.0 = core::ptr::addr_of!(_20);
_100 = core::ptr::addr_of_mut!((*_40));
(*_100) = [(*_3),(*_3),(*_3),(*_3)];
(*_77) = _31 as i32;
_4 = Move(Field::<*const Adt18>(Variant(_28.2.fld0, 1), 0));
_98.2 = ((*_77),);
_6 = _46;
_50 = _17 - _57.fld1;
_2 = &mut place!(Field::<isize>(Variant(_28.2.fld0, 1), 2));
(*_77) = _42 as i32;
Goto(bb43)
}
bb43 = {
_66.2 = !_20;
_2 = Move(_68.3);
_68.2 = ((*_77),);
(*_3) = 13524363714417757254_usize as i128;
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = [_36.0,_84,_36.0];
(*_100) = _54.2;
Goto(bb44)
}
bb44 = {
(*_77) = _68.2.0 - _98.2.0;
_19 = _57.fld6;
(*_70) = core::ptr::addr_of_mut!(_93);
_68.3 = &mut _81;
_67 = (*_59) > _32;
(*_77) = _98.2.0 * _30.0;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
_5 = core::ptr::addr_of!(_104.2);
(*_77) = _91.2 as i32;
(*_5).0 = (*_77);
place!(Field::<[i16; 3]>(Variant((*_12), 1), 1)) = [_31,_84,_36.0];
_19 = _6 as i64;
(*_40) = [(*_3),(*_3),(*_3),(*_3)];
_104 = (Move(_98.0), Move(_12), _68.2, Move(_68.3));
(*_59) = _26 == _26;
(*_5) = _98.2;
Goto(bb45)
}
bb45 = {
_57.fld3 = core::ptr::addr_of_mut!(_54.1);
_104.1 = Move(_4);
_104.3 = &mut _62;
(*_59) = _13 > _26;
_105 = core::ptr::addr_of_mut!(_57.fld5);
Goto(bb46)
}
bb46 = {
(*_5) = (_98.2.0,);
_66.3 = (*_105);
_93 = [12076683581547902584_usize,7_usize,7733397961475141098_usize];
(*_40) = _78;
(*_105) = !_66.3;
(*_40) = _78;
_14 = _76;
_91.0 = _53 & _13;
_103.fld2 = [(*_105),(*_105),(*_105),(*_105)];
(*_105) = _66.1 as u128;
Goto(bb47)
}
bb47 = {
_103.fld0.fld0 = Adt23::Variant1 { fld0: Move(_104.1),fld1: _46,fld2: 9223372036854775807_isize,fld3: _20,fld4: (*_105),fld5: (*_3) };
(*_105) = Field::<u128>(Variant(_103.fld0.fld0, 1), 4) & _66.3;
Call(_91.3 = core::intrinsics::transmute(_57.fld5), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
(*_5).0 = _30.0 - (*_77);
_98.3 = Move(_104.3);
Goto(bb49)
}
bb49 = {
_91.1 = _66.1;
_100 = core::ptr::addr_of_mut!((*_40));
(*_77) = (*_5).0 - (*_5).0;
_41 = _82 - _82;
_104.2 = _68.2;
_98.1 = Move(Field::<*const Adt18>(Variant(_103.fld0.fld0, 1), 0));
(*_40) = _54.2;
_50 = _57.fld1 + _42;
Goto(bb50)
}
bb50 = {
Call(_110 = dump_var(19_usize, 67_usize, Move(_67), 7_usize, Move(_7), 46_usize, Move(_46), 74_usize, Move(_74)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_110 = dump_var(19_usize, 14_usize, Move(_14), 19_usize, Move(_19), 84_usize, Move(_84), 48_usize, Move(_48)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_110 = dump_var(19_usize, 25_usize, Move(_25), 78_usize, Move(_78), 72_usize, Move(_72), 30_usize, Move(_30)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_110 = dump_var(19_usize, 31_usize, Move(_31), 32_usize, Move(_32), 33_usize, Move(_33), 52_usize, Move(_52)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_110 = dump_var(19_usize, 38_usize, Move(_38), 43_usize, Move(_43), 111_usize, _111, 111_usize, _111), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-260991112716735370_i64)), std::hint::black_box('\u{100fed}'), std::hint::black_box(60687_u16), std::hint::black_box(8158561192604709113_u64));
                
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt18 {
Variant0{
fld0: bool,
fld1: usize,
fld2: f32,
fld3: i64,
fld4: u64,

},
Variant1{
fld0: isize,
fld1: [i16; 3],

}}
#[derive(Debug)]
pub enum Adt23 {
Variant0{
fld0: bool,
fld1: i128,
fld2: Adt18,
fld3: u64,
fld4: f32,

},
Variant1{
fld0: *const Adt18,
fld1: char,
fld2: isize,
fld3: i8,
fld4: u128,
fld5: i128,

},
Variant2{
fld0: [i16; 3],
fld1: u64,
fld2: Adt18,
fld3: *const Adt18,

}}
#[derive(Debug)]
pub enum Adt27 {
Variant0{
fld0: *const bool,
fld1: u16,
fld2: isize,
fld3: i8,
fld4: i16,
fld5: (bool, u64, i8, u128),
fld6: i64,

},
Variant1{
fld0: (bool, u64, i8, u128),
fld1: char,
fld2: u16,

}}
#[derive(Debug)]
pub enum Adt31 {
Variant0{
fld0: u128,
fld1: (i32,),
fld2: u64,
fld3: i8,
fld4: i16,

},
Variant1{
fld0: i8,
fld1: i128,

},
Variant2{
fld0: bool,
fld1: *const bool,
fld2: [usize; 3],
fld3: (bool, u64, i8, u128),
fld4: (i32,),
fld5: i32,
fld6: u128,
fld7: Adt23,

}}
#[derive(Debug)]
pub struct Adt38 {
fld0: *mut bool,
fld1: f64,
fld2: *const bool,
fld3: *mut u16,
fld4: usize,
fld5: u128,
fld6: i64,
fld7: [i16; 3],
}
#[derive(Debug)]
pub struct Adt44 {
fld0: Adt23,
fld1: u8,
}
#[derive(Debug)]
pub struct Adt57 {
fld0: Adt44,
fld1: *mut bool,
fld2: [u128; 4],
}
#[derive(Debug)]
pub enum Adt73 {
Variant0{
fld0: *mut u16,
fld1: u128,
fld2: *const *const (i32,),
fld3: i8,
fld4: [char; 4],
fld5: i32,
fld6: Adt44,

},
Variant1{
fld0: (i32,),
fld1: Adt57,
fld2: *const *mut [usize; 3],
fld3: *mut isize,
fld4: *const bool,
fld5: (i16, *mut [usize; 3], u16),

},
Variant2{
fld0: bool,
fld1: (Adt57, *const bool),
fld2: *mut f32,
fld3: i8,
fld4: [i128; 3],
fld5: *mut [usize; 3],
fld6: usize,
fld7: Adt38,

}}

