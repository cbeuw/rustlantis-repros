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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u128,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: u16,mut _9: usize,mut _10: u64) -> u128 {
mir! {
type RET = u128;
let _11: Adt66;
let _12: (*mut bool,);
let _13: i64;
let _14: *const *const u32;
let _15: [u128; 1];
let _16: [u32; 6];
let _17: (bool, f64, u32);
let _18: i64;
let _19: i16;
let _20: Adt57;
let _21: f64;
let _22: [i32; 8];
let _23: i16;
let _24: f32;
let _25: ([i16; 3], f64);
let _26: isize;
let _27: [i16; 3];
let _28: [i32; 8];
let _29: bool;
let _30: u128;
let _31: i8;
let _32: [u128; 1];
let _33: f64;
let _34: char;
let _35: f32;
let _36: (i32, f64);
let _37: u32;
let _38: usize;
let _39: ();
let _40: ();
{
_10 = 12791159573456222153_u64;
_10 = !3591378855507996001_u64;
_9 = !1_usize;
_3 = !264122547199756803136412437213024033883_u128;
_6 = (-2040204110_i32);
_5 = 9223372036854775807_isize as i16;
_2 = '\u{e8797}';
RET = _3;
_8 = 113_u8 as u16;
_2 = '\u{45b6e}';
_7 = 5649155534987911727_i64 << RET;
RET = _3;
RET = !_3;
_1 = !false;
_2 = '\u{f3e8}';
_2 = '\u{6f100}';
RET = _3 & _3;
_12.0 = core::ptr::addr_of_mut!(_1);
_15 = [RET];
_13 = _7 ^ _7;
_12.0 = core::ptr::addr_of_mut!(_1);
_15 = [RET];
_7 = _9 as i64;
_13 = _7;
Call(_11 = fn1(_8, _6, _3, _15, _3, _7, _13, _12.0, _6, _8, _2, _12, _2, RET, _12.0, _6), bb1)
}
bb1 = {
_12.0 = core::ptr::addr_of_mut!(_1);
_4 = -27_i8;
_6 = (-1683471301_i32) & (-1749036567_i32);
_3 = RET;
_8 = _9 as u16;
_4 = -(-23_i8);
_9 = 4_usize;
_16 = [2760483211_u32,4000276093_u32,3463086933_u32,2732388406_u32,2356611129_u32,2535044278_u32];
_10 = !2922561206773235704_u64;
_15 = [_3];
_6 = (-878767129_i32);
_15 = [_3];
_13 = _7;
_3 = _1 as u128;
_1 = !true;
_15 = [_3];
_16[_9] = !1435863977_u32;
RET = _2 as u128;
_3 = RET + RET;
_2 = '\u{5a92e}';
_12.0 = core::ptr::addr_of_mut!(_1);
_10 = !6730064101881708509_u64;
_18 = -_7;
_16 = [2255113851_u32,2816697575_u32,3118417188_u32,3367934825_u32,2310877464_u32,3729423208_u32];
_20.fld2.0 = core::ptr::addr_of_mut!(_1);
Goto(bb2)
}
bb2 = {
_20.fld3 = [145120165537797686454002016787700824841_i128,(-151797406086350685219883387258663747497_i128)];
_20.fld3 = [38732124915084809311797164725132251600_i128,(-44541985278854781251453305399453228846_i128)];
_19 = !_5;
_20.fld3 = [(-166745058253793359314428815554587849046_i128),124459867218700997065583633455717149903_i128];
_10 = !10828436640552623825_u64;
_20.fld2.0 = _12.0;
_20.fld0 = _1 ^ _1;
_10 = !11846390257227184899_u64;
_20.fld1 = core::ptr::addr_of_mut!(RET);
_20.fld2 = (_12.0,);
_13 = _18 + _18;
_20.fld2.0 = core::ptr::addr_of_mut!(_1);
_17.0 = !_20.fld0;
_17.1 = _8 as f64;
_20.fld4.0 = !_17.0;
_20.fld4.1 = _16[_9] as f64;
_1 = _20.fld4.0;
_17.2 = _16[_9] & _16[_9];
Goto(bb3)
}
bb3 = {
_17.2 = _10 as u32;
_17.0 = _1;
match _16[_9] {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2310877464 => bb9,
_ => bb8
}
}
bb4 = {
_20.fld3 = [145120165537797686454002016787700824841_i128,(-151797406086350685219883387258663747497_i128)];
_20.fld3 = [38732124915084809311797164725132251600_i128,(-44541985278854781251453305399453228846_i128)];
_19 = !_5;
_20.fld3 = [(-166745058253793359314428815554587849046_i128),124459867218700997065583633455717149903_i128];
_10 = !10828436640552623825_u64;
_20.fld2.0 = _12.0;
_20.fld0 = _1 ^ _1;
_10 = !11846390257227184899_u64;
_20.fld1 = core::ptr::addr_of_mut!(RET);
_20.fld2 = (_12.0,);
_13 = _18 + _18;
_20.fld2.0 = core::ptr::addr_of_mut!(_1);
_17.0 = !_20.fld0;
_17.1 = _8 as f64;
_20.fld4.0 = !_17.0;
_20.fld4.1 = _16[_9] as f64;
_1 = _20.fld4.0;
_17.2 = _16[_9] & _16[_9];
Goto(bb3)
}
bb5 = {
_12.0 = core::ptr::addr_of_mut!(_1);
_4 = -27_i8;
_6 = (-1683471301_i32) & (-1749036567_i32);
_3 = RET;
_8 = _9 as u16;
_4 = -(-23_i8);
_9 = 4_usize;
_16 = [2760483211_u32,4000276093_u32,3463086933_u32,2732388406_u32,2356611129_u32,2535044278_u32];
_10 = !2922561206773235704_u64;
_15 = [_3];
_6 = (-878767129_i32);
_15 = [_3];
_13 = _7;
_3 = _1 as u128;
_1 = !true;
_15 = [_3];
_16[_9] = !1435863977_u32;
RET = _2 as u128;
_3 = RET + RET;
_2 = '\u{5a92e}';
_12.0 = core::ptr::addr_of_mut!(_1);
_10 = !6730064101881708509_u64;
_18 = -_7;
_16 = [2255113851_u32,2816697575_u32,3118417188_u32,3367934825_u32,2310877464_u32,3729423208_u32];
_20.fld2.0 = core::ptr::addr_of_mut!(_1);
Goto(bb2)
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
_17.1 = -_20.fld4.1;
_2 = '\u{ed4c5}';
_20.fld4 = _17;
match _16[_9] {
0 => bb8,
1 => bb7,
2 => bb10,
3 => bb11,
2310877464 => bb13,
_ => bb12
}
}
bb10 = {
_12.0 = core::ptr::addr_of_mut!(_1);
_4 = -27_i8;
_6 = (-1683471301_i32) & (-1749036567_i32);
_3 = RET;
_8 = _9 as u16;
_4 = -(-23_i8);
_9 = 4_usize;
_16 = [2760483211_u32,4000276093_u32,3463086933_u32,2732388406_u32,2356611129_u32,2535044278_u32];
_10 = !2922561206773235704_u64;
_15 = [_3];
_6 = (-878767129_i32);
_15 = [_3];
_13 = _7;
_3 = _1 as u128;
_1 = !true;
_15 = [_3];
_16[_9] = !1435863977_u32;
RET = _2 as u128;
_3 = RET + RET;
_2 = '\u{5a92e}';
_12.0 = core::ptr::addr_of_mut!(_1);
_10 = !6730064101881708509_u64;
_18 = -_7;
_16 = [2255113851_u32,2816697575_u32,3118417188_u32,3367934825_u32,2310877464_u32,3729423208_u32];
_20.fld2.0 = core::ptr::addr_of_mut!(_1);
Goto(bb2)
}
bb11 = {
_17.2 = _10 as u32;
_17.0 = _1;
match _16[_9] {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2310877464 => bb9,
_ => bb8
}
}
bb12 = {
Return()
}
bb13 = {
_16 = [_20.fld4.2,_20.fld4.2,_20.fld4.2,_20.fld4.2,_20.fld4.2,_17.2];
_27 = [_5,_19,_5];
_17.2 = _20.fld4.2;
_3 = !RET;
_25.1 = _4 as f64;
_5 = !_19;
_20.fld4.0 = _20.fld0 ^ _1;
_20.fld1 = core::ptr::addr_of_mut!(_30);
_27 = [_5,_19,_5];
_8 = 45699_u16;
_8 = _17.0 as u16;
_28 = [_6,_6,_6,_6,_6,_6,_6,_6];
Call(_4 = core::intrinsics::transmute(_17.0), bb14)
}
bb14 = {
RET = _13 as u128;
_17.1 = -_20.fld4.1;
_6 = _9 as i32;
_20.fld4.1 = _3 as f64;
_1 = _20.fld4.0 & _20.fld4.0;
_32 = [_3];
_24 = _8 as f32;
_16 = [_20.fld4.2,_17.2,_20.fld4.2,_20.fld4.2,_17.2,_20.fld4.2];
_33 = _17.1;
_25.0 = [_19,_5,_5];
_18 = _7;
_34 = _2;
_20.fld4 = (_1, _17.1, _16[_9]);
_27 = [_19,_5,_19];
_28 = [_6,_6,_6,_6,_6,_6,_6,_6];
_15 = [_3];
_13 = _7 - _18;
_23 = _19 & _19;
_26 = _6 as isize;
_12 = (_20.fld2.0,);
_4 = !30_i8;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(0_usize, 5_usize, Move(_5), 28_usize, Move(_28), 6_usize, Move(_6), 23_usize, Move(_23)), bb16)
}
bb16 = {
Call(_39 = dump_var(0_usize, 1_usize, Move(_1), 10_usize, Move(_10), 26_usize, Move(_26), 13_usize, Move(_13)), bb17)
}
bb17 = {
Call(_39 = dump_var(0_usize, 9_usize, Move(_9), 19_usize, Move(_19), 40_usize, _40, 40_usize, _40), bb18)
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u16,mut _2: i32,mut _3: u128,mut _4: [u128; 1],mut _5: u128,mut _6: i64,mut _7: i64,mut _8: *mut bool,mut _9: i32,mut _10: u16,mut _11: char,mut _12: (*mut bool,),mut _13: char,mut _14: u128,mut _15: *mut bool,mut _16: i32) -> Adt66 {
mir! {
type RET = Adt66;
let _17: isize;
let _18: Adt52;
let _19: *const ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64));
let _20: [u128; 1];
let _21: i128;
let _22: Adt64;
let _23: bool;
let _24: f64;
let _25: char;
let _26: char;
let _27: Adt60;
let _28: isize;
let _29: i128;
let _30: i32;
let _31: bool;
let _32: f64;
let _33: (f64,);
let _34: u128;
let _35: isize;
let _36: Adt50;
let _37: [i16; 3];
let _38: (i32, f64);
let _39: f64;
let _40: ([i16; 3], f64);
let _41: char;
let _42: (i32, f64);
let _43: [u32; 6];
let _44: ([i16; 3], f64);
let _45: i128;
let _46: f32;
let _47: usize;
let _48: (i32, f64);
let _49: f32;
let _50: Adt59;
let _51: (u16,);
let _52: (u32, *mut (u16,), f64);
let _53: ();
let _54: ();
{
_12.0 = core::ptr::addr_of_mut!((*_8));
_9 = _2;
(*_8) = true;
_17 = 1296488067751286385_u64 as isize;
_11 = _13;
_5 = _3;
_10 = _1 << _3;
_18.fld3 = -8_i8;
_18.fld2 = _5 as isize;
(*_8) = _14 >= _14;
Goto(bb1)
}
bb1 = {
_4 = [_14];
(*_8) = true;
_18.fld1 = core::ptr::addr_of_mut!(_5);
_15 = core::ptr::addr_of_mut!((*_8));
(*_15) = true & false;
_18.fld2 = _17;
_18.fld1 = core::ptr::addr_of_mut!(_3);
_18.fld2 = 3029275213_u32 as isize;
_15 = core::ptr::addr_of_mut!((*_8));
_2 = 13040_i16 as i32;
(*_8) = false;
(*_8) = !false;
_6 = _7 << _17;
_3 = !_14;
_10 = 31537_i16 as u16;
_20 = [_14];
(*_15) = true;
_18.fld0 = !_7;
_18.fld0 = _3 as i64;
_3 = !_5;
_12.0 = core::ptr::addr_of_mut!((*_15));
_7 = _18.fld0;
_2 = !_16;
Call((*_8) = fn2(_6, _4, _11, _10, _2, _16, _12.0, _16, _1, _18.fld2, _18.fld0, _18.fld1, _14, _15), bb2)
}
bb2 = {
_6 = !_18.fld0;
_1 = _10 >> _6;
_12 = (_15,);
_5 = _14;
_12 = (_8,);
(*_15) = !false;
_12.0 = core::ptr::addr_of_mut!((*_15));
_6 = _7;
(*_8) = false;
_7 = _18.fld0;
_17 = _18.fld2 >> _3;
_1 = _10;
_9 = _16 * _16;
_20 = [_5];
(*_15) = true;
_13 = _11;
_11 = _13;
_11 = _13;
_10 = _1 ^ _1;
Call(_7 = core::intrinsics::bswap(_6), bb3)
}
bb3 = {
(*_8) = !true;
_18.fld0 = _6;
_18.fld0 = _7 ^ _7;
_12.0 = core::ptr::addr_of_mut!(_23);
_8 = _15;
_1 = !_10;
_5 = _14;
(*_8) = !false;
_4 = [_5];
_4 = [_14];
Goto(bb4)
}
bb4 = {
(*_15) = !true;
_20 = [_3];
_13 = _11;
_14 = _3 * _3;
_11 = _13;
_18.fld1 = core::ptr::addr_of_mut!(_14);
_23 = !(*_15);
(*_8) = !_23;
_2 = _14 as i32;
_8 = _12.0;
_1 = !_10;
_9 = _2 | _2;
_18.fld0 = -_6;
_18.fld2 = _3 as isize;
_17 = _18.fld2;
_22 = Adt64::Variant0 { fld0: _8 };
_13 = _11;
(*_8) = (*_15);
_10 = _1;
_18.fld5 = core::ptr::addr_of!(_24);
_2 = _9;
(*_8) = _2 <= _16;
_14 = !_3;
Goto(bb5)
}
bb5 = {
_18.fld2 = _17 >> _9;
_3 = _14 - _14;
_25 = _13;
_25 = _11;
_18.fld5 = core::ptr::addr_of!(_24);
(*_8) = (*_15) & (*_15);
_30 = -_2;
_23 = (*_15);
_16 = !_9;
_18.fld2 = _17 - _17;
(*_8) = _17 != _18.fld2;
place!(Field::<*mut bool>(Variant(_22, 0), 0)) = core::ptr::addr_of_mut!(_31);
_7 = _6 * _18.fld0;
(*_8) = !(*_15);
_7 = !_6;
_21 = (-9870422565950287654787020518441912742_i128);
place!(Field::<*mut bool>(Variant(_22, 0), 0)) = core::ptr::addr_of_mut!((*_8));
_26 = _13;
_7 = !_18.fld0;
_24 = _18.fld3 as f64;
_4 = [_5];
_31 = _2 >= _9;
match _21 {
330411944354988175808587586913326298714 => bb7,
_ => bb6
}
}
bb6 = {
(*_15) = !true;
_20 = [_3];
_13 = _11;
_14 = _3 * _3;
_11 = _13;
_18.fld1 = core::ptr::addr_of_mut!(_14);
_23 = !(*_15);
(*_8) = !_23;
_2 = _14 as i32;
_8 = _12.0;
_1 = !_10;
_9 = _2 | _2;
_18.fld0 = -_6;
_18.fld2 = _3 as isize;
_17 = _18.fld2;
_22 = Adt64::Variant0 { fld0: _8 };
_13 = _11;
(*_8) = (*_15);
_10 = _1;
_18.fld5 = core::ptr::addr_of!(_24);
_2 = _9;
(*_8) = _2 <= _16;
_14 = !_3;
Goto(bb5)
}
bb7 = {
_23 = _31;
_18.fld3 = (-5_i8);
_28 = _1 as isize;
_26 = _25;
_33.0 = -_24;
_11 = _26;
_4 = _20;
_29 = 10803064222794148560_u64 as i128;
place!(Field::<*mut bool>(Variant(_22, 0), 0)) = core::ptr::addr_of_mut!(_23);
_18.fld0 = !_7;
(*_8) = _9 < _2;
Goto(bb8)
}
bb8 = {
_35 = -_18.fld2;
_23 = _30 < _30;
(*_8) = _31;
_34 = _3 & _14;
_12 = (Field::<*mut bool>(Variant(_22, 0), 0),);
_2 = !_30;
_18.fld2 = -_35;
_9 = _30;
_12 = (Field::<*mut bool>(Variant(_22, 0), 0),);
SetDiscriminant(_22, 0);
_32 = _33.0 + _33.0;
_31 = (*_8) > (*_8);
_8 = core::ptr::addr_of_mut!(_31);
_37 = [(-16457_i16),(-18495_i16),23738_i16];
_36.fld2 = core::ptr::addr_of!(_36.fld3);
_38 = (_16, _32);
_18.fld2 = _28 - _35;
_20 = [_34];
_18.fld2 = _17;
match _21 {
0 => bb3,
330411944354988175808587586913326298714 => bb9,
_ => bb6
}
}
bb9 = {
_36.fld5 = 3436365400912109621_usize;
_21 = !_29;
_10 = _1;
_4 = _20;
_11 = _13;
(*_15) = _31 | _23;
_38 = (_16, _32);
_36.fld1 = !45_u8;
match _36.fld5 {
0 => bb7,
3436365400912109621 => bb10,
_ => bb2
}
}
bb10 = {
(*_15) = _31;
_2 = _18.fld3 as i32;
_18.fld1 = core::ptr::addr_of_mut!(_3);
_20 = [_34];
_18.fld2 = -_35;
_36.fld5 = 5_usize ^ 2_usize;
_36.fld5 = 1262031354987533974_usize * 4571896488423946949_usize;
_41 = _13;
match _18.fld3 {
0 => bb7,
340282366920938463463374607431768211451 => bb11,
_ => bb5
}
}
bb11 = {
_29 = _21 - _21;
_18.fld2 = _35;
_7 = -_18.fld0;
_33 = (_38.1,);
_39 = _38.1 * _32;
_6 = _39 as i64;
_13 = _26;
_1 = !_10;
_36.fld4 = 1260_i16 | 16887_i16;
_11 = _41;
_36.fld2 = core::ptr::addr_of!(_36.fld3);
_33.0 = _6 as f64;
_21 = -_29;
_33.0 = _39 - _24;
_4 = _20;
_36.fld0 = (_37, _24);
_42 = _38;
_26 = _41;
_36.fld6 = _36.fld1 as f32;
_40.1 = _39;
_38 = _42;
Goto(bb12)
}
bb12 = {
_44.0 = _36.fld0.0;
_38 = _42;
_48.0 = !_30;
_14 = _3;
_20 = _4;
_42.0 = _9 - _30;
_12 = (_8,);
_36.fld5 = 14931460093702163016_usize * 4_usize;
SetDiscriminant(_22, 1);
place!(Field::<Adt58>(Variant(_22, 1), 3)).fld0 = _18.fld0 - _7;
_40 = (_36.fld0.0, _33.0);
_7 = -_6;
place!(Field::<Adt58>(Variant(_22, 1), 3)).fld3.6 = core::ptr::addr_of_mut!(place!(Field::<(u16,)>(Variant(_22, 1), 5)));
_17 = _14 as isize;
_25 = _26;
(*_15) = (*_8);
_34 = !_14;
_38.0 = 11310313916269504922_u64 as i32;
_20 = [_14];
_45 = _21;
place!(Field::<Adt58>(Variant(_22, 1), 3)).fld3.5.2 = _33.0 as i128;
_46 = _36.fld6;
_36.fld3 = _36.fld5 as u32;
_2 = _30;
place!(Field::<Adt58>(Variant(_22, 1), 3)).fld3.3 = !_36.fld1;
place!(Field::<Adt58>(Variant(_22, 1), 3)).fld3.5.0 = _33.0 * _39;
_17 = _28;
SetDiscriminant(Field::<Adt56>(Variant(_22, 1), 1), 3);
place!(Field::<bool>(Variant(_22, 1), 0)) = (*_8);
match _18.fld3 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb13,
5 => bb14,
340282366920938463463374607431768211451 => bb16,
_ => bb15
}
}
bb13 = {
_35 = -_18.fld2;
_23 = _30 < _30;
(*_8) = _31;
_34 = _3 & _14;
_12 = (Field::<*mut bool>(Variant(_22, 0), 0),);
_2 = !_30;
_18.fld2 = -_35;
_9 = _30;
_12 = (Field::<*mut bool>(Variant(_22, 0), 0),);
SetDiscriminant(_22, 0);
_32 = _33.0 + _33.0;
_31 = (*_8) > (*_8);
_8 = core::ptr::addr_of_mut!(_31);
_37 = [(-16457_i16),(-18495_i16),23738_i16];
_36.fld2 = core::ptr::addr_of!(_36.fld3);
_38 = (_16, _32);
_18.fld2 = _28 - _35;
_20 = [_34];
_18.fld2 = _17;
match _21 {
0 => bb3,
330411944354988175808587586913326298714 => bb9,
_ => bb6
}
}
bb14 = {
_23 = _31;
_18.fld3 = (-5_i8);
_28 = _1 as isize;
_26 = _25;
_33.0 = -_24;
_11 = _26;
_4 = _20;
_29 = 10803064222794148560_u64 as i128;
place!(Field::<*mut bool>(Variant(_22, 0), 0)) = core::ptr::addr_of_mut!(_23);
_18.fld0 = !_7;
(*_8) = _9 < _2;
Goto(bb8)
}
bb15 = {
(*_15) = !true;
_20 = [_3];
_13 = _11;
_14 = _3 * _3;
_11 = _13;
_18.fld1 = core::ptr::addr_of_mut!(_14);
_23 = !(*_15);
(*_8) = !_23;
_2 = _14 as i32;
_8 = _12.0;
_1 = !_10;
_9 = _2 | _2;
_18.fld0 = -_6;
_18.fld2 = _3 as isize;
_17 = _18.fld2;
_22 = Adt64::Variant0 { fld0: _8 };
_13 = _11;
(*_8) = (*_15);
_10 = _1;
_18.fld5 = core::ptr::addr_of!(_24);
_2 = _9;
(*_8) = _2 <= _16;
_14 = !_3;
Goto(bb5)
}
bb16 = {
place!(Field::<Adt58>(Variant(_22, 1), 3)).fld2 = !_14;
SetDiscriminant(Field::<Adt60>(Variant(_22, 1), 2), 1);
_35 = _18.fld2 | _28;
_48.1 = _40.1;
_35 = _1 as isize;
_29 = _45 & Field::<Adt58>(Variant(_22, 1), 3).fld3.5.2;
place!(Field::<Adt58>(Variant(_22, 1), 3)).fld3.1.2 = _10 as i128;
_44.0 = [_36.fld4,_36.fld4,_36.fld4];
place!(Field::<Adt58>(Variant(place!(Field::<Adt60>(Variant(_22, 1), 2)), 1), 0)).fld3.5 = (Field::<Adt58>(Variant(_22, 1), 3).fld3.5.0, _18.fld2, _21);
place!(Field::<f64>(Variant(place!(Field::<Adt60>(Variant(_22, 1), 2)), 1), 2)) = -Field::<Adt58>(Variant(_22, 1), 3).fld3.5.0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt56>(Variant(_22, 1), 1)), 3), 4)).5.1 = !_18.fld2;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt56>(Variant(_22, 1), 1)), 3), 4)).5 = (Field::<f64>(Variant(Field::<Adt60>(Variant(_22, 1), 2), 1), 2), _35, _29);
_19 = core::ptr::addr_of!(_50.fld3.1);
place!(Field::<(i128, (*mut (u16,), u128), (*mut (u16,), u128))>(Variant(place!(Field::<Adt56>(Variant(_22, 1), 1)), 3), 3)).2 = (Field::<Adt58>(Variant(_22, 1), 3).fld3.6, _5);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt56>(Variant(_22, 1), 1)), 3), 4)).1.2 = _29 >> Field::<(i128, (*mut (u16,), u128), (*mut (u16,), u128))>(Variant(Field::<Adt56>(Variant(_22, 1), 1), 3), 3).2.1;
_50.fld3.1.1.0 = _16 & _16;
(*_19).1.1 = Field::<f64>(Variant(Field::<Adt60>(Variant(_22, 1), 2), 1), 2) - Field::<f64>(Variant(Field::<Adt60>(Variant(_22, 1), 2), 1), 2);
(*_19).5.1 = (*_19).1.1;
place!(Field::<Adt58>(Variant(place!(Field::<Adt60>(Variant(_22, 1), 2)), 1), 0)).fld3.7 = [17082270765907279633_u64,7771289283964754350_u64,13027775520883949091_u64];
_31 = !(*_15);
place!(Field::<(i32, f64)>(Variant(place!(Field::<Adt60>(Variant(_22, 1), 2)), 1), 4)).1 = _36.fld1 as f64;
Goto(bb17)
}
bb17 = {
Call(_53 = dump_var(1_usize, 11_usize, Move(_11), 21_usize, Move(_21), 34_usize, Move(_34), 5_usize, Move(_5)), bb18)
}
bb18 = {
Call(_53 = dump_var(1_usize, 29_usize, Move(_29), 6_usize, Move(_6), 26_usize, Move(_26), 25_usize, Move(_25)), bb19)
}
bb19 = {
Call(_53 = dump_var(1_usize, 31_usize, Move(_31), 14_usize, Move(_14), 7_usize, Move(_7), 37_usize, Move(_37)), bb20)
}
bb20 = {
Call(_53 = dump_var(1_usize, 17_usize, Move(_17), 20_usize, Move(_20), 54_usize, _54, 54_usize, _54), bb21)
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i64,mut _2: [u128; 1],mut _3: char,mut _4: u16,mut _5: i32,mut _6: i32,mut _7: *mut bool,mut _8: i32,mut _9: u16,mut _10: isize,mut _11: i64,mut _12: *mut u128,mut _13: u128,mut _14: *mut bool) -> bool {
mir! {
type RET = bool;
let _15: (f64, isize, i128);
let _16: f32;
let _17: [u32; 6];
let _18: [i16; 4];
let _19: (f64,);
let _20: [i16; 4];
let _21: [u32; 6];
let _22: i16;
let _23: Adt64;
let _24: *const ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64));
let _25: bool;
let _26: f64;
let _27: Adt65;
let _28: u32;
let _29: (*mut (u16,),);
let _30: bool;
let _31: char;
let _32: (*mut bool,);
let _33: u32;
let _34: ([i16; 3], f64);
let _35: bool;
let _36: Adt62;
let _37: i8;
let _38: isize;
let _39: (*mut (u16,), u128);
let _40: (u16,);
let _41: ();
let _42: ();
{
_10 = 23_isize;
_1 = -_11;
_11 = _1;
_1 = _11;
_15.0 = 243_u8 as f64;
_15.2 = _9 as i128;
_3 = '\u{2e76d}';
_15.0 = 67_i8 as f64;
_11 = _1;
_8 = _5 & _5;
_8 = -_6;
_15.2 = (-72094062357331833867480401640179590202_i128);
_1 = !_11;
RET = !false;
_16 = 16456771019410932539_usize as f32;
_1 = _13 as i64;
_15.2 = 43251728279309818698814593659652401882_i128;
_1 = _11 * _11;
_18 = [(-1719_i16),13770_i16,15433_i16,(-6789_i16)];
_12 = core::ptr::addr_of_mut!((*_12));
_4 = (-37_i8) as u16;
_18 = [14005_i16,(-5060_i16),10725_i16,5994_i16];
_19 = (_15.0,);
Call(_16 = core::intrinsics::transmute(_6), bb1)
}
bb1 = {
_20 = [(-20548_i16),(-9406_i16),18909_i16,25807_i16];
(*_12) = _13 | _13;
_19 = (_15.0,);
_4 = _9;
_15.0 = _19.0;
_17 = [203067045_u32,676770393_u32,3505164229_u32,1051079265_u32,990448029_u32,4076369811_u32];
_2 = [(*_12)];
match _15.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
43251728279309818698814593659652401882 => bb7,
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
_14 = core::ptr::addr_of_mut!(RET);
_17 = [2396357799_u32,2372629750_u32,2435620970_u32,3786101781_u32,1338740527_u32,694110689_u32];
_14 = _7;
_9 = _4;
_23 = Adt64::Variant0 { fld0: _14 };
_1 = !_11;
place!(Field::<*mut bool>(Variant(_23, 0), 0)) = core::ptr::addr_of_mut!(RET);
Goto(bb8)
}
bb8 = {
_21 = [258295209_u32,1921658887_u32,979867615_u32,1366256598_u32,781946488_u32,3403817644_u32];
_20 = [(-15048_i16),4174_i16,(-29052_i16),19993_i16];
_15.1 = _10 + _10;
_8 = _5;
_11 = _5 as i64;
_15.2 = (-85883308214941185349325440706291372498_i128);
_15.0 = -_19.0;
_15.1 = _10 ^ _10;
_4 = _15.2 as u16;
(*_12) = !_13;
_23 = Adt64::Variant0 { fld0: _14 };
_22 = !29136_i16;
_5 = -_8;
_15 = (_19.0, _10, 92499102559017228600514636644275774636_i128);
_12 = core::ptr::addr_of_mut!((*_12));
_13 = 237_u8 as u128;
_11 = _22 as i64;
_9 = _4;
_25 = _8 > _6;
_16 = (-94_i8) as f32;
_22 = 1249774650_u32 as i16;
_19.0 = _15.2 as f64;
_19.0 = _15.0;
SetDiscriminant(_23, 0);
SetDiscriminant(_23, 1);
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.4 = [_22,_22,_22];
_9 = _4 | _4;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.6 = core::ptr::addr_of_mut!(place!(Field::<(u16,)>(Variant(_23, 1), 5)));
Call(place!(Field::<Adt56>(Variant(_23, 1), 1)) = fn3(_15.2, _15.2, _17, _3, _9, _15, _15.1, _12, _15.1, _3, _3, _21), bb9)
}
bb9 = {
SetDiscriminant(Field::<Adt60>(Variant(_23, 1), 2), 2);
place!(Field::<(f64, isize, i128)>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 5)).0 = -_15.0;
place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld2.1 = _13;
_22 = (-10068_i16) * 731_i16;
place!(Field::<usize>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 0)) = 9307263934363343720_usize >> _5;
place!(Field::<(f64, isize, i128)>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 5)).1 = _15.1;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.3 = 237_u8 << _9;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.1.2 = _16 as i128;
_27.fld2.fld3.1.4 = core::ptr::addr_of_mut!(place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld2);
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.7 = [1923501026024772861_u64,14150941555535214155_u64,3452503678552204416_u64];
place!(Field::<*const *const u32>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 3)) = core::ptr::addr_of!(_27.fld1.fld2);
_27.fld2.fld3.1.5.0 = -_5;
place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld1 = (*_12);
_19.0 = _15.0;
_26 = _19.0;
_27.fld1.fld3 = !1636161392_u32;
_27.fld2.fld3.1.1.1 = Field::<(f64, isize, i128)>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 5).0 * _26;
Call(_17 = fn8(_15.1, _15, _27.fld1.fld3, _15, _15.1), bb10)
}
bb10 = {
place!(Field::<i64>(Variant(_23, 1), 4)) = _22 as i64;
_27.fld1.fld0 = (Field::<Adt58>(Variant(_23, 1), 3).fld3.4, _27.fld2.fld3.1.1.1);
place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld0 = Field::<i64>(Variant(_23, 1), 4) << _9;
_27.fld2.fld1.1 = _27.fld1.fld0.1 + _27.fld1.fld0.1;
_25 = !RET;
Goto(bb11)
}
bb11 = {
_27.fld1.fld1 = Field::<Adt58>(Variant(_23, 1), 3).fld3.3 + Field::<Adt58>(Variant(_23, 1), 3).fld3.3;
_29.0 = Field::<Adt58>(Variant(_23, 1), 3).fld3.6;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.5 = _15;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.1.0 = -_15.0;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld0 = Field::<Adt55>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 2).fld0 | Field::<Adt55>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 2).fld0;
_27.fld3 = [_27.fld1.fld3,_27.fld1.fld3,_27.fld1.fld3,_27.fld1.fld3,_27.fld1.fld3,_27.fld1.fld3];
place!(Field::<(f64, isize, i128)>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 5)).0 = -_27.fld2.fld1.1;
place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld2 = (Field::<Adt58>(Variant(_23, 1), 3).fld3.6, _13);
_27.fld1.fld4 = _22 - _22;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.5.2 = Field::<usize>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 0) as i128;
Call(place!(Field::<Adt58>(Variant(_23, 1), 3)).fld4 = fn15(_2, Field::<Adt55>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 2).fld0, Move(Field::<Adt51>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 6)), Field::<Adt58>(Variant(_23, 1), 3).fld3.3, Field::<Adt58>(Variant(_23, 1), 3).fld3.6, _27.fld2.fld3.1.1.1, _15.2, _15.2), bb12)
}
bb12 = {
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.1.1 = -_10;
_20 = [_27.fld1.fld4,_22,_27.fld1.fld4,_27.fld1.fld4];
place!(Field::<(f64, isize, i128)>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 5)) = Field::<Adt58>(Variant(_23, 1), 3).fld3.1;
SetDiscriminant(Field::<Adt51>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 6), 1);
place!(Field::<(u16,)>(Variant(_23, 1), 5)) = (_4,);
_26 = _27.fld2.fld1.1 + _27.fld2.fld1.1;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld4 = [_15.2,_15.2];
_27.fld2.fld3.1.0 = _27.fld1.fld0.0;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.0 = !_27.fld1.fld1;
_27.fld2.fld3.5.0 = _27.fld2.fld3.1.0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).1 = Field::<Adt58>(Variant(_23, 1), 3).fld3.5;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld1 = !_15.2;
(*_12) = _13 + Field::<Adt55>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 2).fld1;
_27.fld3 = _21;
_27.fld2.fld3.5.0 = _27.fld1.fld0.0;
place!(Field::<(i32, f64)>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 7)) = (_8, _27.fld2.fld1.1);
_10 = !Field::<(f64, isize, i128)>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 5).1;
_27.fld2.fld3.2 = _3;
_33 = _27.fld1.fld3;
_27.fld2.fld3.1.1.0 = _8;
place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld3 = 118_i8 >> Field::<usize>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 0);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).7 = Field::<Adt58>(Variant(_23, 1), 3).fld3.7;
_18 = _20;
Goto(bb13)
}
bb13 = {
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).6 = Field::<Adt55>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 2).fld2.0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).5.2 = Field::<Adt58>(Variant(_23, 1), 3).fld3.5.2 * Field::<Adt58>(Variant(_23, 1), 3).fld3.5.2;
_27.fld1.fld0.1 = _26;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).2 = _4;
_30 = !RET;
_31 = _27.fld2.fld3.2;
_27.fld2.fld1.0 = _27.fld2.fld3.1.1.0 * _6;
_27.fld2.fld1 = (Field::<(i32, f64)>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 7).0, _27.fld2.fld3.1.1.1);
place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld1 = _13 & (*_12);
place!(Field::<Adt56>(Variant(_23, 1), 1)) = Move(Field::<Adt56>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 1));
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).7 = [2306224029359304859_u64,1998489432267204180_u64,4925717425747706823_u64];
_27.fld2.fld3.1.4 = core::ptr::addr_of_mut!(place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld2);
_27.fld1.fld6 = Field::<Adt55>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 2).fld3 as f32;
_27.fld2.fld3.1.0 = [_22,_27.fld1.fld4,_22];
place!(Field::<bool>(Variant(_23, 1), 0)) = !_30;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).3 = !_27.fld1.fld1;
_27.fld1.fld0 = (_27.fld2.fld3.1.0, Field::<(i32, f64)>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 7).1);
RET = !_30;
match _15.2 {
0 => bb7,
1 => bb11,
2 => bb10,
3 => bb14,
4 => bb15,
5 => bb16,
92499102559017228600514636644275774636 => bb18,
_ => bb17
}
}
bb14 = {
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.1.1 = -_10;
_20 = [_27.fld1.fld4,_22,_27.fld1.fld4,_27.fld1.fld4];
place!(Field::<(f64, isize, i128)>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 5)) = Field::<Adt58>(Variant(_23, 1), 3).fld3.1;
SetDiscriminant(Field::<Adt51>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 6), 1);
place!(Field::<(u16,)>(Variant(_23, 1), 5)) = (_4,);
_26 = _27.fld2.fld1.1 + _27.fld2.fld1.1;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld4 = [_15.2,_15.2];
_27.fld2.fld3.1.0 = _27.fld1.fld0.0;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld3.0 = !_27.fld1.fld1;
_27.fld2.fld3.5.0 = _27.fld2.fld3.1.0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).1 = Field::<Adt58>(Variant(_23, 1), 3).fld3.5;
place!(Field::<Adt58>(Variant(_23, 1), 3)).fld1 = !_15.2;
(*_12) = _13 + Field::<Adt55>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 2).fld1;
_27.fld3 = _21;
_27.fld2.fld3.5.0 = _27.fld1.fld0.0;
place!(Field::<(i32, f64)>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 7)) = (_8, _27.fld2.fld1.1);
_10 = !Field::<(f64, isize, i128)>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 5).1;
_27.fld2.fld3.2 = _3;
_33 = _27.fld1.fld3;
_27.fld2.fld3.1.1.0 = _8;
place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld3 = 118_i8 >> Field::<usize>(Variant(Field::<Adt60>(Variant(_23, 1), 2), 2), 0);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).7 = Field::<Adt58>(Variant(_23, 1), 3).fld3.7;
_18 = _20;
Goto(bb13)
}
bb15 = {
Return()
}
bb16 = {
place!(Field::<i64>(Variant(_23, 1), 4)) = _22 as i64;
_27.fld1.fld0 = (Field::<Adt58>(Variant(_23, 1), 3).fld3.4, _27.fld2.fld3.1.1.1);
place!(Field::<Adt55>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 2)).fld0 = Field::<i64>(Variant(_23, 1), 4) << _9;
_27.fld2.fld1.1 = _27.fld1.fld0.1 + _27.fld1.fld0.1;
_25 = !RET;
Goto(bb11)
}
bb17 = {
Return()
}
bb18 = {
place!(Field::<(f64, isize, i128)>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 5)).1 = !_15.1;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 1)).5.0 = _26;
place!(Field::<(f64,)>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt60>(Variant(_23, 1), 2)), 2), 6)), 1), 2)) = _19;
_15 = (_27.fld2.fld3.1.1.1, _10, Field::<Adt58>(Variant(_23, 1), 3).fld3.5.2);
Goto(bb19)
}
bb19 = {
Call(_41 = dump_var(2_usize, 10_usize, Move(_10), 31_usize, Move(_31), 8_usize, Move(_8), 11_usize, Move(_11)), bb20)
}
bb20 = {
Call(_41 = dump_var(2_usize, 4_usize, Move(_4), 5_usize, Move(_5), 22_usize, Move(_22), 33_usize, Move(_33)), bb21)
}
bb21 = {
Call(_41 = dump_var(2_usize, 9_usize, Move(_9), 13_usize, Move(_13), 42_usize, _42, 42_usize, _42), bb22)
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i128,mut _2: i128,mut _3: [u32; 6],mut _4: char,mut _5: u16,mut _6: (f64, isize, i128),mut _7: isize,mut _8: *mut u128,mut _9: isize,mut _10: char,mut _11: char,mut _12: [u32; 6]) -> Adt56 {
mir! {
type RET = Adt56;
let _13: (bool, f64, u32);
let _14: [i16; 3];
let _15: char;
let _16: f32;
let _17: isize;
let _18: [u64; 3];
let _19: u64;
let _20: [i16; 4];
let _21: u64;
let _22: (bool, f64, u32);
let _23: *mut (u16,);
let _24: isize;
let _25: ();
let _26: ();
{
_8 = core::ptr::addr_of_mut!((*_8));
_8 = core::ptr::addr_of_mut!((*_8));
_7 = _9 + _9;
_3 = [2429545937_u32,1479379410_u32,1449071516_u32,1699303513_u32,3631757323_u32,18225674_u32];
(*_8) = 114031769385223547258847258888363397988_u128 ^ 281055934101735823222806563837512267092_u128;
_13.2 = !202392675_u32;
_4 = _11;
_13.0 = false ^ true;
_13.0 = _11 != _4;
_2 = _6.2 + _1;
_10 = _11;
_15 = _10;
match _1 {
0 => bb1,
92499102559017228600514636644275774636 => bb3,
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
_12 = [_13.2,_13.2,_13.2,_13.2,_13.2,_13.2];
_3 = [_13.2,_13.2,_13.2,_13.2,_13.2,_13.2];
_17 = !_6.1;
_13.0 = !true;
_7 = _9;
_14 = [(-9802_i16),24999_i16,31307_i16];
_8 = core::ptr::addr_of_mut!((*_8));
_10 = _15;
match _6.2 {
0 => bb1,
1 => bb2,
2 => bb4,
92499102559017228600514636644275774636 => bb6,
_ => bb5
}
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
(*_8) = 77137981299288141918287628933186091751_u128;
_3 = _12;
_5 = 43895_u16;
_8 = core::ptr::addr_of_mut!((*_8));
_13 = (true, _6.0, 527001292_u32);
_18 = [16835658569112584351_u64,10254759884429956351_u64,12672935037652921058_u64];
_6 = (_13.1, _9, _1);
_9 = !_17;
_14 = [6865_i16,(-30632_i16),9046_i16];
Goto(bb7)
}
bb7 = {
_13.1 = _6.0;
_6 = (_13.1, _17, _1);
_11 = _4;
(*_8) = 244230784820887820742415223229830202279_u128;
_16 = _5 as f32;
_6 = (_13.1, _7, _2);
_14 = [21297_i16,(-15864_i16),(-25001_i16)];
_19 = 10358128803485785786_u64;
_13.2 = !2684844529_u32;
_4 = _10;
_10 = _4;
_9 = !_17;
_13.0 = !false;
_4 = _11;
_6 = (_13.1, _9, _2);
_4 = _15;
_21 = (-21_i8) as u64;
Call(_3 = fn4(_6, _6, _13.0, _4, (*_8), _18, _16, (*_8), _6.2, _6.2), bb8)
}
bb8 = {
_13 = (false, _6.0, 1622656109_u32);
_1 = -_6.2;
_3 = _12;
_22.0 = _13.0 & _13.0;
_22.2 = _13.2;
_13.0 = !_22.0;
match (*_8) {
0 => bb6,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
244230784820887820742415223229830202279 => bb16,
_ => bb15
}
}
bb9 = {
_13.1 = _6.0;
_6 = (_13.1, _17, _1);
_11 = _4;
(*_8) = 244230784820887820742415223229830202279_u128;
_16 = _5 as f32;
_6 = (_13.1, _7, _2);
_14 = [21297_i16,(-15864_i16),(-25001_i16)];
_19 = 10358128803485785786_u64;
_13.2 = !2684844529_u32;
_4 = _10;
_10 = _4;
_9 = !_17;
_13.0 = !false;
_4 = _11;
_6 = (_13.1, _9, _2);
_4 = _15;
_21 = (-21_i8) as u64;
Call(_3 = fn4(_6, _6, _13.0, _4, (*_8), _18, _16, (*_8), _6.2, _6.2), bb8)
}
bb10 = {
(*_8) = 77137981299288141918287628933186091751_u128;
_3 = _12;
_5 = 43895_u16;
_8 = core::ptr::addr_of_mut!((*_8));
_13 = (true, _6.0, 527001292_u32);
_18 = [16835658569112584351_u64,10254759884429956351_u64,12672935037652921058_u64];
_6 = (_13.1, _9, _1);
_9 = !_17;
_14 = [6865_i16,(-30632_i16),9046_i16];
Goto(bb7)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_12 = [_13.2,_13.2,_13.2,_13.2,_13.2,_13.2];
_3 = [_13.2,_13.2,_13.2,_13.2,_13.2,_13.2];
_17 = !_6.1;
_13.0 = !true;
_7 = _9;
_14 = [(-9802_i16),24999_i16,31307_i16];
_8 = core::ptr::addr_of_mut!((*_8));
_10 = _15;
match _6.2 {
0 => bb1,
1 => bb2,
2 => bb4,
92499102559017228600514636644275774636 => bb6,
_ => bb5
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_13 = (_22.0, _6.0, _22.2);
_3 = [_22.2,_13.2,_22.2,_22.2,_22.2,_22.2];
_15 = _4;
(*_8) = _16 as u128;
_5 = 27562_u16 - 2675_u16;
_6.0 = _13.1;
_16 = 1749186692_i32 as f32;
_22 = (_13.0, _13.1, _13.2);
_19 = !_21;
Goto(bb17)
}
bb17 = {
Call(_25 = dump_var(3_usize, 1_usize, Move(_1), 11_usize, Move(_11), 9_usize, Move(_9), 21_usize, Move(_21)), bb18)
}
bb18 = {
Call(_25 = dump_var(3_usize, 3_usize, Move(_3), 2_usize, Move(_2), 5_usize, Move(_5), 7_usize, Move(_7)), bb19)
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (f64, isize, i128),mut _2: (f64, isize, i128),mut _3: bool,mut _4: char,mut _5: u128,mut _6: [u64; 3],mut _7: f32,mut _8: u128,mut _9: i128,mut _10: i128) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _11: f32;
let _12: ([i16; 3], f64);
let _13: isize;
let _14: [i16; 3];
let _15: Adt56;
let _16: [u64; 1];
let _17: i8;
let _18: isize;
let _19: [u32; 6];
let _20: *mut (*mut (u16,), u128);
let _21: usize;
let _22: [u64; 3];
let _23: usize;
let _24: [char; 8];
let _25: i64;
let _26: [i32; 8];
let _27: [u128; 1];
let _28: i16;
let _29: [u128; 1];
let _30: isize;
let _31: [u32; 6];
let _32: f32;
let _33: i16;
let _34: isize;
let _35: (u16,);
let _36: Adt53;
let _37: ();
let _38: ();
{
_6 = [6060880489205218547_u64,10322174732451367097_u64,578202871741612610_u64];
Goto(bb1)
}
bb1 = {
_9 = !_1.2;
_5 = !_8;
_2.2 = _10;
match _8 {
244230784820887820742415223229830202279 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_1 = (_2.0, _2.1, _10);
_3 = _9 <= _2.2;
_4 = '\u{107eae}';
RET = [2317771732_u32,186778928_u32,2882264933_u32,2582151815_u32,171613184_u32,2959874117_u32];
RET = [3676726176_u32,1699968603_u32,3611891470_u32,2944896182_u32,630213382_u32,3411706694_u32];
RET = [2917073043_u32,304180892_u32,12603911_u32,924672902_u32,227466564_u32,1539479540_u32];
_5 = 74_i8 as u128;
_7 = 4018_u16 as f32;
RET = [2725802714_u32,1533992357_u32,1617412673_u32,522044550_u32,3431753049_u32,3637361696_u32];
_5 = 3095679980_u32 as u128;
_9 = (-991428699_i32) as i128;
_2.0 = _1.0;
_12.1 = _2.0;
_12.0 = [730_i16,20248_i16,24297_i16];
_4 = '\u{3c7b9}';
_3 = _1.2 == _2.2;
_8 = _5;
_1 = (_2.0, _2.1, _2.2);
RET = [551474430_u32,1287007138_u32,307199911_u32,1739740718_u32,899153070_u32,72516605_u32];
_2.1 = (-1283013878_i32) as isize;
Goto(bb4)
}
bb4 = {
_4 = '\u{4b2b1}';
_13 = _2.1 + _1.1;
_1.1 = _13;
_12.0 = [(-10588_i16),28214_i16,29116_i16];
_9 = -_1.2;
_1.2 = _3 as i128;
_3 = !false;
_8 = _5;
_14 = _12.0;
_1.1 = _2.1 * _13;
_5 = !_8;
_12.1 = -_2.0;
_13 = _2.1 + _1.1;
Goto(bb5)
}
bb5 = {
_9 = _10 & _1.2;
_2.0 = -_1.0;
_17 = !(-4_i8);
_1.0 = -_12.1;
_6 = [8697606087355583968_u64,685309973782386102_u64,1789337403691068707_u64];
_8 = !_5;
_2.0 = _8 as f64;
_12.1 = _17 as f64;
_2.2 = _9 | _1.2;
_11 = _1.1 as f32;
_14 = [3618_i16,(-14488_i16),21794_i16];
_18 = -_13;
_12.0 = [(-27943_i16),(-31233_i16),(-8367_i16)];
_1.1 = -_13;
_12.1 = -_1.0;
_14 = [26250_i16,(-22154_i16),7472_i16];
_1.1 = _18;
_1.1 = _13;
_3 = false | false;
_22 = [14304314164653024379_u64,11812361655170016350_u64,9156095757408604127_u64];
Call(_21 = fn5(_2, _13, _2, _2.2, _2, _1, _14, _10, _18, _1, _2, _6, RET, _2), bb6)
}
bb6 = {
_16 = [3477473962014739169_u64];
_13 = !_18;
_2.0 = _12.1;
_5 = _8 * _8;
RET = [2321790644_u32,981648107_u32,1594840893_u32,2658392979_u32,4144581268_u32,2041654701_u32];
_12.1 = _1.0;
_13 = _18;
_22 = [861086036284460923_u64,6498750004509512801_u64,638150443087333448_u64];
Goto(bb7)
}
bb7 = {
_1.1 = _18;
_14 = _12.0;
_14 = _12.0;
_16 = [10527846937921182369_u64];
_1.1 = _13 ^ _13;
_17 = 39_i8;
_1 = _2;
_16 = [3802436563887481257_u64];
_10 = -_1.2;
_19 = [428876806_u32,2187818696_u32,1399312926_u32,3301862664_u32,1872343962_u32,961893010_u32];
Call(_2.1 = fn7(_21, _10, _10, _18, _1, _1, _1, _1.1), bb8)
}
bb8 = {
_1.2 = !_9;
_21 = !2_usize;
_12 = (_14, _2.0);
_12 = (_14, _2.0);
_22 = [14202300353856908553_u64,8784942585905754682_u64,8902653138372734990_u64];
RET = [1403939506_u32,936505197_u32,2616623772_u32,1274704456_u32,1288554313_u32,3081986663_u32];
_19 = RET;
_12.0 = [3319_i16,651_i16,32128_i16];
_14 = [(-21217_i16),9222_i16,(-15401_i16)];
_11 = _7 + _7;
_25 = (-5769790426889578548_i64) >> _2.1;
_1.2 = _10 + _9;
_9 = _1.2 + _1.2;
_28 = 31805_u16 as i16;
match _17 {
0 => bb4,
1 => bb9,
2 => bb10,
39 => bb12,
_ => bb11
}
}
bb9 = {
_1.1 = _18;
_14 = _12.0;
_14 = _12.0;
_16 = [10527846937921182369_u64];
_1.1 = _13 ^ _13;
_17 = 39_i8;
_1 = _2;
_16 = [3802436563887481257_u64];
_10 = -_1.2;
_19 = [428876806_u32,2187818696_u32,1399312926_u32,3301862664_u32,1872343962_u32,961893010_u32];
Call(_2.1 = fn7(_21, _10, _10, _18, _1, _1, _1, _1.1), bb8)
}
bb10 = {
_1 = (_2.0, _2.1, _10);
_3 = _9 <= _2.2;
_4 = '\u{107eae}';
RET = [2317771732_u32,186778928_u32,2882264933_u32,2582151815_u32,171613184_u32,2959874117_u32];
RET = [3676726176_u32,1699968603_u32,3611891470_u32,2944896182_u32,630213382_u32,3411706694_u32];
RET = [2917073043_u32,304180892_u32,12603911_u32,924672902_u32,227466564_u32,1539479540_u32];
_5 = 74_i8 as u128;
_7 = 4018_u16 as f32;
RET = [2725802714_u32,1533992357_u32,1617412673_u32,522044550_u32,3431753049_u32,3637361696_u32];
_5 = 3095679980_u32 as u128;
_9 = (-991428699_i32) as i128;
_2.0 = _1.0;
_12.1 = _2.0;
_12.0 = [730_i16,20248_i16,24297_i16];
_4 = '\u{3c7b9}';
_3 = _1.2 == _2.2;
_8 = _5;
_1 = (_2.0, _2.1, _2.2);
RET = [551474430_u32,1287007138_u32,307199911_u32,1739740718_u32,899153070_u32,72516605_u32];
_2.1 = (-1283013878_i32) as isize;
Goto(bb4)
}
bb11 = {
_9 = _10 & _1.2;
_2.0 = -_1.0;
_17 = !(-4_i8);
_1.0 = -_12.1;
_6 = [8697606087355583968_u64,685309973782386102_u64,1789337403691068707_u64];
_8 = !_5;
_2.0 = _8 as f64;
_12.1 = _17 as f64;
_2.2 = _9 | _1.2;
_11 = _1.1 as f32;
_14 = [3618_i16,(-14488_i16),21794_i16];
_18 = -_13;
_12.0 = [(-27943_i16),(-31233_i16),(-8367_i16)];
_1.1 = -_13;
_12.1 = -_1.0;
_14 = [26250_i16,(-22154_i16),7472_i16];
_1.1 = _18;
_1.1 = _13;
_3 = false | false;
_22 = [14304314164653024379_u64,11812361655170016350_u64,9156095757408604127_u64];
Call(_21 = fn5(_2, _13, _2, _2.2, _2, _1, _14, _10, _18, _1, _2, _6, RET, _2), bb6)
}
bb12 = {
_12 = (_14, _1.0);
_12 = (_14, _2.0);
_21 = _3 as usize;
_12 = (_14, _2.0);
_2.0 = _1.0 * _12.1;
_22 = [8321323828666534755_u64,17130468701540836520_u64,12768119909105405011_u64];
_16 = [1037003699770073219_u64];
_26 = [1206574169_i32,(-1969333921_i32),926155028_i32,(-1627557461_i32),1619796561_i32,360023177_i32,(-1137477378_i32),1734046996_i32];
_33 = -_28;
_2.1 = -_13;
_30 = !_13;
_29 = [_5];
_4 = '\u{1bc38}';
RET = _19;
_3 = !false;
_12.0 = [_28,_33,_33];
_29 = [_5];
_5 = _11 as u128;
_1.2 = _9;
_34 = _18 << _9;
_23 = _21 - _21;
_24 = [_4,_4,_4,_4,_4,_4,_4,_4];
_12.0 = [_28,_33,_33];
_3 = !true;
_33 = _28 << _34;
_33 = !_28;
_31 = [2476022375_u32,2190873610_u32,1565143845_u32,3029667421_u32,3227222674_u32,1147389982_u32];
Call(_2.0 = core::intrinsics::transmute(_34), bb13)
}
bb13 = {
_1.2 = _2.0 as i128;
_1.2 = -_9;
_1.0 = -_2.0;
_7 = _28 as f32;
_22 = _6;
_27 = [_8];
_10 = _1.2 * _9;
match _17 {
0 => bb14,
39 => bb16,
_ => bb15
}
}
bb14 = {
_12 = (_14, _1.0);
_12 = (_14, _2.0);
_21 = _3 as usize;
_12 = (_14, _2.0);
_2.0 = _1.0 * _12.1;
_22 = [8321323828666534755_u64,17130468701540836520_u64,12768119909105405011_u64];
_16 = [1037003699770073219_u64];
_26 = [1206574169_i32,(-1969333921_i32),926155028_i32,(-1627557461_i32),1619796561_i32,360023177_i32,(-1137477378_i32),1734046996_i32];
_33 = -_28;
_2.1 = -_13;
_30 = !_13;
_29 = [_5];
_4 = '\u{1bc38}';
RET = _19;
_3 = !false;
_12.0 = [_28,_33,_33];
_29 = [_5];
_5 = _11 as u128;
_1.2 = _9;
_34 = _18 << _9;
_23 = _21 - _21;
_24 = [_4,_4,_4,_4,_4,_4,_4,_4];
_12.0 = [_28,_33,_33];
_3 = !true;
_33 = _28 << _34;
_33 = !_28;
_31 = [2476022375_u32,2190873610_u32,1565143845_u32,3029667421_u32,3227222674_u32,1147389982_u32];
Call(_2.0 = core::intrinsics::transmute(_34), bb13)
}
bb15 = {
_1 = (_2.0, _2.1, _10);
_3 = _9 <= _2.2;
_4 = '\u{107eae}';
RET = [2317771732_u32,186778928_u32,2882264933_u32,2582151815_u32,171613184_u32,2959874117_u32];
RET = [3676726176_u32,1699968603_u32,3611891470_u32,2944896182_u32,630213382_u32,3411706694_u32];
RET = [2917073043_u32,304180892_u32,12603911_u32,924672902_u32,227466564_u32,1539479540_u32];
_5 = 74_i8 as u128;
_7 = 4018_u16 as f32;
RET = [2725802714_u32,1533992357_u32,1617412673_u32,522044550_u32,3431753049_u32,3637361696_u32];
_5 = 3095679980_u32 as u128;
_9 = (-991428699_i32) as i128;
_2.0 = _1.0;
_12.1 = _2.0;
_12.0 = [730_i16,20248_i16,24297_i16];
_4 = '\u{3c7b9}';
_3 = _1.2 == _2.2;
_8 = _5;
_1 = (_2.0, _2.1, _2.2);
RET = [551474430_u32,1287007138_u32,307199911_u32,1739740718_u32,899153070_u32,72516605_u32];
_2.1 = (-1283013878_i32) as isize;
Goto(bb4)
}
bb16 = {
_25 = (-7712910331729606429_i64) >> _10;
_29 = [_5];
_25 = 5020620511126146659_i64 | (-801887073975240298_i64);
_29 = _27;
_19 = [4105873933_u32,1587988674_u32,4142873392_u32,3143934820_u32,1710659601_u32,1360309420_u32];
_8 = _5;
_2 = (_1.0, _34, _10);
_28 = _33 ^ _33;
_30 = !_34;
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(4_usize, 34_usize, Move(_34), 4_usize, Move(_4), 31_usize, Move(_31), 13_usize, Move(_13)), bb18)
}
bb18 = {
Call(_37 = dump_var(4_usize, 6_usize, Move(_6), 26_usize, Move(_26), 8_usize, Move(_8), 25_usize, Move(_25)), bb19)
}
bb19 = {
Call(_37 = dump_var(4_usize, 16_usize, Move(_16), 24_usize, Move(_24), 30_usize, Move(_30), 28_usize, Move(_28)), bb20)
}
bb20 = {
Call(_37 = dump_var(4_usize, 9_usize, Move(_9), 38_usize, _38, 38_usize, _38, 38_usize, _38), bb21)
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: (f64, isize, i128),mut _2: isize,mut _3: (f64, isize, i128),mut _4: i128,mut _5: (f64, isize, i128),mut _6: (f64, isize, i128),mut _7: [i16; 3],mut _8: i128,mut _9: isize,mut _10: (f64, isize, i128),mut _11: (f64, isize, i128),mut _12: [u64; 3],mut _13: [u32; 6],mut _14: (f64, isize, i128)) -> usize {
mir! {
type RET = usize;
let _15: Adt51;
let _16: u128;
let _17: f64;
let _18: i8;
let _19: [i128; 2];
let _20: ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64));
let _21: (i32, f64);
let _22: (i32, f64);
let _23: (bool, f64, u32);
let _24: isize;
let _25: u32;
let _26: f64;
let _27: ();
let _28: ();
{
_1.0 = -_3.0;
_2 = _14.2 as isize;
_14.1 = _2;
_11.1 = !_14.1;
_6.0 = _10.0;
_10 = (_5.0, _14.1, _4);
_3.1 = _11.1;
_14.1 = 882254136_u32 as isize;
RET = 1404315322_i32 as usize;
_10.2 = -_4;
_13 = [1719616871_u32,3000821291_u32,2605179656_u32,3739234472_u32,483385192_u32,3978306179_u32];
Goto(bb1)
}
bb1 = {
_5.1 = _2;
_9 = 1277910245_i32 as isize;
_3.1 = !_11.1;
_1.0 = (-4_i8) as f64;
_11.1 = _10.2 as isize;
_3.1 = (-325970497_i32) as isize;
Goto(bb2)
}
bb2 = {
_5.0 = _2 as f64;
_5.1 = !_11.1;
_11.0 = _5.0;
_11.2 = _1.2 * _4;
_8 = _3.2;
_9 = _5.1;
_3.1 = _11.1 << _1.2;
_9 = _6.1 ^ _3.1;
_6.2 = _11.2;
_14 = _5;
_10.1 = _6.2 as isize;
RET = 0_usize;
_12[RET] = 8575470408887270078_u64;
_3.1 = _9;
_6.0 = _5.0 + _5.0;
_1.2 = !_6.2;
Call(_3.1 = core::intrinsics::transmute(_2), bb3)
}
bb3 = {
_1.1 = _14.1 * _14.1;
_14.2 = _8 << _11.1;
_5.2 = _8;
_4 = _11.2;
_10.0 = _5.0 - _14.0;
_11.1 = 73_u8 as isize;
_13 = [3085224184_u32,1356883537_u32,558390088_u32,3501578931_u32,1355341954_u32,3054703803_u32];
_14 = (_10.0, _3.1, _11.2);
_7 = [(-23820_i16),24860_i16,1253_i16];
_6 = (_14.0, _9, _10.2);
Goto(bb4)
}
bb4 = {
_5.1 = -_9;
_3 = _14;
_1.2 = 41428_u16 as i128;
_13 = [3871899780_u32,2346616647_u32,4243105873_u32,1351852470_u32,3332322070_u32,4117500646_u32];
_1.1 = _5.1 | _10.1;
RET = (-538775908_i32) as usize;
_11 = (_3.0, _2, _3.2);
_17 = 61447_u16 as f64;
RET = 24153_u16 as usize;
Goto(bb5)
}
bb5 = {
_5.1 = _3.1 * _1.1;
_10.0 = _3.0 + _14.0;
_6 = (_3.0, _2, _3.2);
_17 = -_3.0;
_10.1 = _14.1;
_11.1 = !_9;
_1 = _3;
_12 = [6792393106685527530_u64,16349436610310201179_u64,10865537887348650088_u64];
RET = 16765563335485263343_usize & 0_usize;
_10.0 = _11.0;
_5 = (_10.0, _9, _4);
_3.0 = _1.1 as f64;
_10.2 = -_14.2;
_5.0 = _3.0;
_5.2 = _4 >> _10.2;
_13 = [1863254123_u32,3587726618_u32,3115875993_u32,547204747_u32,423082927_u32,651195649_u32];
_9 = _1.1;
_11 = (_3.0, _9, _10.2);
_4 = _6.2 ^ _14.2;
_10.0 = -_5.0;
_5.0 = _6.0 + _6.0;
_11 = (_14.0, _2, _10.2);
_3 = _1;
_5.0 = (-3608051027836806612_i64) as f64;
_11 = (_6.0, _9, _4);
_8 = (-4352395000396424849_i64) as i128;
Call(_18 = core::intrinsics::bswap((-85_i8)), bb6)
}
bb6 = {
_2 = 14066116398592101133_u64 as isize;
_3.2 = _4;
_1.0 = -_10.0;
_16 = !248602337144450035428939192904510754642_u128;
_1.2 = _11.2;
_14.0 = -_10.0;
_4 = _14.2 >> _11.2;
_6 = _10;
_10 = (_3.0, _14.1, _14.2);
_9 = _14.1;
_10.1 = (-1037600564_i32) as isize;
_10.0 = -_11.0;
_10.2 = !_3.2;
_19 = [_14.2,_14.2];
_20.5.0 = 16801388671312445472_u64 as i32;
Call(_6 = fn6(_1.2, _5, _10, _10, _5, _19), bb7)
}
bb7 = {
_12 = [12981191070763976196_u64,12211391556072544252_u64,9192706234392873087_u64];
_1 = (_14.0, _3.1, _5.2);
_11.1 = _5.1;
_10.2 = -_4;
_3.2 = (-5801_i16) as i128;
_20.0 = [20468_i16,29605_i16,(-16939_i16)];
_20.5 = (1126540209_i32, _10.0);
Goto(bb8)
}
bb8 = {
_22.0 = true as i32;
_11 = _14;
_22.0 = false as i32;
_8 = !_5.2;
_14.1 = -_11.1;
_11.2 = _1.2;
_5.1 = _20.5.0 as isize;
_21 = (_20.5.0, _10.0);
_1.2 = !_5.2;
_11.2 = _14.2;
_23.1 = -_20.5.1;
_16 = 19608310260201395616683259859745998073_u128 - 340033368102581713553694245547880994846_u128;
_6.1 = _1.1;
_6 = (_14.0, _5.1, _8);
_23.1 = 132_u8 as f64;
_22.1 = _6.0 * _10.0;
_20.5.0 = _21.0 << _9;
_20.5.1 = 3942920044_u32 as f64;
Goto(bb9)
}
bb9 = {
_5.1 = _6.1 - _6.1;
_5.1 = _3.1 | _6.1;
_20.3 = _9;
_24 = -_5.1;
_12 = [6476929141906459588_u64,1130541662613540701_u64,12762379646946385065_u64];
RET = _24 as usize;
_20.5 = (_21.0, _21.1);
_25 = !108472922_u32;
_11 = _6;
_20.5 = (_21.0, _17);
_20.1 = (_20.5.0, _22.1);
_17 = _3.0;
_10.2 = _11.2 >> _20.1.0;
_20.5.0 = -_21.0;
_5 = (_22.1, _20.3, _6.2);
_6.2 = _10.2 + _5.2;
Goto(bb10)
}
bb10 = {
Call(_27 = dump_var(5_usize, 25_usize, Move(_25), 4_usize, Move(_4), 2_usize, Move(_2), 16_usize, Move(_16)), bb11)
}
bb11 = {
Call(_27 = dump_var(5_usize, 9_usize, Move(_9), 12_usize, Move(_12), 28_usize, _28, 28_usize, _28), bb12)
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: i128,mut _2: (f64, isize, i128),mut _3: (f64, isize, i128),mut _4: (f64, isize, i128),mut _5: (f64, isize, i128),mut _6: [i128; 2]) -> (f64, isize, i128) {
mir! {
type RET = (f64, isize, i128);
let _7: f32;
let _8: f64;
let _9: usize;
let _10: (f64, isize, i128);
let _11: char;
let _12: ();
let _13: ();
{
RET.0 = _4.0;
Call(RET.1 = core::intrinsics::bswap(_2.1), bb1)
}
bb1 = {
_6 = [_4.2,_5.2];
_4.2 = _1 | _2.2;
RET.2 = _4.2;
_4.2 = (-40_i8) as i128;
RET = _3;
RET = _3;
RET.0 = -_3.0;
_1 = RET.2;
_3.1 = _2.1 - _2.1;
_4.0 = (-1285514339_i32) as f64;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(6_usize, 6_usize, Move(_6), 13_usize, _13, 13_usize, _13, 13_usize, _13), bb3)
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: usize,mut _2: i128,mut _3: i128,mut _4: isize,mut _5: (f64, isize, i128),mut _6: (f64, isize, i128),mut _7: (f64, isize, i128),mut _8: isize) -> isize {
mir! {
type RET = isize;
let _9: (f64,);
let _10: f32;
let _11: u16;
let _12: [i16; 4];
let _13: char;
let _14: ([i16; 3], f64);
let _15: ([i16; 3], f64);
let _16: ();
let _17: ();
{
_8 = _5.1 - _4;
RET = !_8;
_6.1 = _4 << _1;
_6.2 = (-30969_i16) as i128;
_3 = '\u{e7597}' as i128;
_7.1 = -_8;
_5.1 = _6.1;
_6.0 = 19021_u16 as f64;
RET = _5.1;
_4 = _6.1 | _6.1;
_6 = _5;
_6.2 = -_5.2;
_3 = -_6.2;
_7 = (_5.0, _4, _6.2);
_9.0 = -_6.0;
_1 = 318874522_i32 as usize;
_7 = (_6.0, RET, _3);
_6.0 = _9.0 + _5.0;
_10 = _1 as f32;
_6.0 = _5.0 * _9.0;
RET = _6.1;
_5 = (_9.0, RET, _7.2);
_6.1 = _5.1 - _7.1;
_5.0 = _5.1 as f64;
_5.1 = !_4;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(7_usize, 2_usize, Move(_2), 8_usize, Move(_8), 17_usize, _17, 17_usize, _17), bb2)
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: (f64, isize, i128),mut _3: u32,mut _4: (f64, isize, i128),mut _5: isize) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _6: bool;
let _7: i32;
let _8: u128;
let _9: (f64,);
let _10: isize;
let _11: f64;
let _12: bool;
let _13: (f64,);
let _14: isize;
let _15: bool;
let _16: isize;
let _17: [char; 8];
let _18: f64;
let _19: [u128; 1];
let _20: u16;
let _21: isize;
let _22: ([i16; 3], ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)), char, bool, i16, ([i16; 3], f64));
let _23: f32;
let _24: (u16,);
let _25: isize;
let _26: Adt60;
let _27: (i32, f64);
let _28: i128;
let _29: ();
let _30: ();
{
_4.2 = _2.2;
RET = [_3,_3,_3,_3,_3,_3];
match _4.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
92499102559017228600514636644275774636 => bb6,
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
_5 = -_1;
_6 = true;
_2.0 = -_4.0;
_6 = !true;
_2.2 = 0_usize as i128;
_7 = (-1265635946_i32);
_7 = 1447640156_i32 >> _2.1;
_6 = true;
RET = [_3,_3,_3,_3,_3,_3];
RET = [_3,_3,_3,_3,_3,_3];
RET = [_3,_3,_3,_3,_3,_3];
_1 = -_2.1;
Goto(bb7)
}
bb7 = {
_4.0 = (-56_i8) as f64;
_7 = (-2132449787_i32);
_3 = 5_i8 as u32;
_4.0 = -_2.0;
_11 = -_4.0;
RET = [_3,_3,_3,_3,_3,_3];
_2.2 = !_4.2;
_4.0 = _11 - _2.0;
_9 = (_4.0,);
_11 = _9.0;
_6 = !false;
_2.0 = _11;
RET = [_3,_3,_3,_3,_3,_3];
_4.2 = 53353602934954241915698423664491799919_u128 as i128;
_4 = (_9.0, _1, _2.2);
_6 = !false;
_14 = _5;
_10 = _5 << _3;
_14 = 44_i8 as isize;
RET = [_3,_3,_3,_3,_3,_3];
_13 = (_4.0,);
_3 = 2723070101_u32 >> _4.2;
match _2.1 {
0 => bb4,
1 => bb2,
2 => bb5,
3 => bb8,
4 => bb9,
23 => bb11,
_ => bb10
}
}
bb8 = {
_5 = -_1;
_6 = true;
_2.0 = -_4.0;
_6 = !true;
_2.2 = 0_usize as i128;
_7 = (-1265635946_i32);
_7 = 1447640156_i32 >> _2.1;
_6 = true;
RET = [_3,_3,_3,_3,_3,_3];
RET = [_3,_3,_3,_3,_3,_3];
RET = [_3,_3,_3,_3,_3,_3];
_1 = -_2.1;
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_2.1 = !_10;
_12 = _6;
_8 = !322463366257767840870146434757846932533_u128;
_15 = _12;
_7 = _8 as i32;
_18 = -_13.0;
_1 = !_4.1;
_4.2 = _2.2 | _2.2;
_4.0 = -_9.0;
_15 = !_12;
Call(_1 = core::intrinsics::transmute(_10), bb12)
}
bb12 = {
_17 = ['\u{3013}','\u{d2a1e}','\u{f84a9}','\u{7ed4}','\u{2efa1}','\u{29237}','\u{f8798}','\u{8816c}'];
_6 = _13.0 <= _18;
_4 = (_2.0, _1, _2.2);
_5 = _1 - _2.1;
_1 = _10 - _14;
_2 = (_13.0, _1, _4.2);
_20 = 2721_u16;
_5 = -_2.1;
_17 = ['\u{100284}','\u{5435a}','\u{e9c62}','\u{78ccb}','\u{df9c6}','\u{c299a}','\u{cabb8}','\u{50419}'];
RET = [_3,_3,_3,_3,_3,_3];
_2.2 = _4.2;
Call(_2 = fn9(_7, _10, _9), bb13)
}
bb13 = {
_9.0 = _2.0 - _13.0;
_10 = _2.1;
_12 = _5 != _4.1;
_2 = _4;
RET = [_3,_3,_3,_3,_3,_3];
_22.1.5.0 = _7;
_22.5.1 = _3 as f64;
_3 = 1263910387_u32;
_22.2 = '\u{b3b4c}';
_5 = _10;
_22.1.5.0 = _7;
_22.1.1.0 = 202_u8 as i32;
_6 = _12 & _12;
_22.1.5.0 = _7;
_22.0 = [22741_i16,(-4759_i16),(-27072_i16)];
_18 = _11 * _4.0;
Goto(bb14)
}
bb14 = {
_22.1.3 = _5;
_11 = _18 + _18;
_22.1.5.0 = _7 >> _5;
_2.1 = _10;
_19 = [_8];
_22.1.1.0 = _7;
_4.1 = _10 * _5;
_24 = (_20,);
_22.1.2 = _22.2;
_22.4 = !(-27675_i16);
_20 = _22.1.5.0 as u16;
_5 = _4.1 >> _2.1;
_22.1.1.1 = _9.0;
_21 = !_2.1;
_2 = (_22.1.1.1, _22.1.3, _4.2);
_15 = _12;
_5 = _21 << _22.1.5.0;
_7 = _22.1.5.0;
_23 = _2.2 as f32;
_2.1 = !_5;
_22.5.0 = _22.0;
_22.3 = _6;
_22.1.2 = _22.2;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(8_usize, 8_usize, Move(_8), 10_usize, Move(_10), 14_usize, Move(_14), 24_usize, Move(_24)), bb16)
}
bb16 = {
Call(_29 = dump_var(8_usize, 3_usize, Move(_3), 6_usize, Move(_6), 21_usize, Move(_21), 30_usize, _30), bb17)
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i32,mut _2: isize,mut _3: (f64,)) -> (f64, isize, i128) {
mir! {
type RET = (f64, isize, i128);
let _4: [u32; 6];
let _5: i32;
let _6: isize;
let _7: [u128; 1];
let _8: f64;
let _9: i64;
let _10: ();
let _11: ();
{
RET = (_3.0, _2, (-93818303052674485102315959092211922767_i128));
_4 = [3241741662_u32,2222827372_u32,4203422496_u32,367394458_u32,1960875648_u32,3787555698_u32];
_3 = (RET.0,);
_3 = (RET.0,);
_2 = RET.1;
_3.0 = RET.0;
_3.0 = RET.0 + RET.0;
RET = (_3.0, _2, (-73145522276846292902709986521179375675_i128));
Call(_2 = fn10(_3.0, _3, _3.0, RET.2, _3.0, _3.0, RET), bb1)
}
bb1 = {
RET.1 = _2;
_5 = _1 + _1;
RET = (_3.0, _2, (-136755971111544788610481009395708155325_i128));
RET.2 = !109764826180157841887121916443289634528_i128;
RET.1 = !_2;
RET.1 = _3.0 as isize;
_1 = _5 & _5;
_4 = [2635517633_u32,1836236574_u32,3188480975_u32,2173247246_u32,1306781989_u32,380130875_u32];
_5 = -_1;
RET.2 = true as i128;
RET.2 = 6102915063892333506_i64 as i128;
_3 = (RET.0,);
RET = (_3.0, _2, (-169977068037432999492881509281779805019_i128));
_4 = [2452282136_u32,3151899927_u32,501597740_u32,3444451464_u32,4050510412_u32,842672213_u32];
RET = (_3.0, _2, (-99071015109696377535852041951519290644_i128));
_5 = RET.2 as i32;
_6 = -_2;
Goto(bb2)
}
bb2 = {
Call(_10 = dump_var(9_usize, 4_usize, Move(_4), 5_usize, Move(_5), 11_usize, _11, 11_usize, _11), bb3)
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: f64,mut _2: (f64,),mut _3: f64,mut _4: i128,mut _5: f64,mut _6: f64,mut _7: (f64, isize, i128)) -> isize {
mir! {
type RET = isize;
let _8: isize;
let _9: [i16; 4];
let _10: [u32; 6];
let _11: [u64; 3];
let _12: *mut u128;
let _13: u8;
let _14: Adt57;
let _15: char;
let _16: *const u32;
let _17: Adt57;
let _18: i32;
let _19: (i32, f64);
let _20: char;
let _21: u64;
let _22: bool;
let _23: isize;
let _24: *const *const u32;
let _25: isize;
let _26: i8;
let _27: [i32; 8];
let _28: ();
let _29: ();
{
RET = _7.1;
_1 = -_6;
_2 = (_7.0,);
_6 = _7.0 - _3;
_4 = _7.2;
_1 = _2.0 - _7.0;
RET = _7.1 & _7.1;
_7.2 = !_4;
RET = _7.1 - _7.1;
_10 = [2649244252_u32,2818855951_u32,1481361530_u32,3679841270_u32,2797589034_u32,240150966_u32];
_2 = (_5,);
_2.0 = -_3;
_7 = (_1, RET, _4);
RET = 7058040447764758274_u64 as isize;
_7.2 = 174_u8 as i128;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
267136844644092170560664620910588835781 => bb8,
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
_8 = _7.1 | _7.1;
_15 = '\u{108954}';
_14.fld3 = [_4,_4];
_14.fld4.2 = !2560033023_u32;
_9 = [(-6392_i16),(-25403_i16),2705_i16,28289_i16];
_14.fld2.0 = core::ptr::addr_of_mut!(_14.fld0);
_4 = _7.2 | _7.2;
_7 = (_5, _8, _4);
_14.fld4 = (true, _1, 1699566973_u32);
_17.fld4.2 = _14.fld4.2 ^ _14.fld4.2;
_2 = (_1,);
_14.fld0 = _7.1 < _7.1;
_17.fld0 = _14.fld4.0;
_11 = [394347793369514321_u64,17757632774257452855_u64,9678796362342159213_u64];
_14.fld4 = (_17.fld0, _2.0, _17.fld4.2);
_17.fld3 = [_4,_7.2];
_17.fld2.0 = core::ptr::addr_of_mut!(_17.fld0);
_2.0 = 41_u8 as f64;
Call(_17.fld4.0 = fn11(_17.fld4.2, _14.fld4.2, _17.fld2.0, _3, _2, _7.1, _17.fld4.2), bb9)
}
bb9 = {
_17.fld4 = _14.fld4;
_13 = 213_u8;
_20 = _15;
_2 = (_5,);
_7.0 = _1;
_2 = (_7.0,);
_17.fld3 = _14.fld3;
Goto(bb10)
}
bb10 = {
_14.fld2.0 = core::ptr::addr_of_mut!(_17.fld4.0);
_15 = _20;
_14.fld4.2 = 58820511253732348203821343237518389569_u128 as u32;
_9 = [18747_i16,389_i16,12205_i16,26819_i16];
_6 = _17.fld4.1 - _17.fld4.1;
_19 = ((-1776954568_i32), _17.fld4.1);
_1 = -_6;
_7.2 = _4;
Goto(bb11)
}
bb11 = {
_6 = _3;
_4 = -_7.2;
Goto(bb12)
}
bb12 = {
_1 = 7037877315860439154_i64 as f64;
_19.0 = 477663898_i32 ^ (-775897080_i32);
_17.fld0 = _17.fld4.0;
_21 = 1402609648920617190_u64;
_22 = _17.fld4.2 == _17.fld4.2;
_16 = core::ptr::addr_of!(_17.fld4.2);
RET = !_8;
_7.0 = _13 as f64;
_24 = core::ptr::addr_of!(_16);
_17.fld4 = _14.fld4;
RET = _7.1;
_14.fld0 = _22 > _22;
_14.fld4.1 = 9034_i16 as f64;
_17.fld2.0 = core::ptr::addr_of_mut!(_17.fld4.0);
_23 = 22818_u16 as isize;
_7 = (_6, _8, _4);
_17.fld4 = (_14.fld0, _2.0, _14.fld4.2);
Call(_20 = fn12(_17.fld4, _17.fld4.0, _17.fld2.0, _14.fld0, _14.fld2.0, _17.fld2.0, _17.fld2.0, _17.fld2.0, _14.fld2.0, _8, _17.fld4, _22, _17.fld2.0, _14.fld2.0, _14.fld2, _14.fld4.0), bb13)
}
bb13 = {
_18 = _4 as i32;
_7.0 = _13 as f64;
Goto(bb14)
}
bb14 = {
_1 = _17.fld4.1;
_25 = RET;
_14.fld4.0 = _17.fld4.0;
_17.fld4.1 = _21 as f64;
_17.fld2 = (_14.fld2.0,);
_8 = 111804736891023059258866173343530462453_u128 as isize;
_14.fld4.2 = (*_16);
_7.0 = 110_i8 as f64;
_6 = _19.1;
_17.fld2 = _14.fld2;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(10_usize, 11_usize, Move(_11), 18_usize, Move(_18), 23_usize, Move(_23), 25_usize, Move(_25)), bb16)
}
bb16 = {
Call(_28 = dump_var(10_usize, 10_usize, Move(_10), 13_usize, Move(_13), 29_usize, _29, 29_usize, _29), bb17)
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: u32,mut _2: u32,mut _3: *mut bool,mut _4: f64,mut _5: (f64,),mut _6: isize,mut _7: u32) -> bool {
mir! {
type RET = bool;
let _8: *const *const u32;
let _9: (bool, f64, u32);
let _10: (f64, isize, i128);
let _11: u64;
let _12: (u32, *mut (u16,), f64);
let _13: Adt58;
let _14: i64;
let _15: [i16; 4];
let _16: [i128; 2];
let _17: [i128; 2];
let _18: u32;
let _19: char;
let _20: ();
let _21: ();
{
_7 = !_2;
(*_3) = false;
_5.0 = -_4;
RET = (*_3);
Goto(bb1)
}
bb1 = {
(*_3) = !RET;
_9 = ((*_3), _5.0, _2);
_2 = _7;
RET = !(*_3);
(*_3) = _9.0 ^ _9.0;
_9.0 = !(*_3);
_5.0 = _9.1 + _9.1;
_10.2 = !92959808906178495387724251913720298368_i128;
_10 = (_5.0, _6, (-58818271434346132687427460121260114679_i128));
_1 = 52_i8 as u32;
_5.0 = _10.0;
RET = !(*_3);
RET = (*_3) ^ _9.0;
RET = (*_3);
_1 = 2107657110814015960_i64 as u32;
_10.1 = -_6;
_9.1 = 9839570203782407779_u64 as f64;
_5.0 = 108_u8 as f64;
Call(_10.0 = core::intrinsics::fmaf64(_4, _4, _4), bb2)
}
bb2 = {
_11 = 14897148592167212926_u64 * 12110502002543756739_u64;
_10.1 = _6;
_10.1 = _6 & _6;
_7 = 9721_u16 as u32;
_9.2 = _2;
_4 = -_10.0;
(*_3) = _2 != _9.2;
_10 = (_9.1, _6, (-98761663771537996009622086373072111768_i128));
_13.fld3.5 = _10;
_7 = !_9.2;
_13.fld4 = [_13.fld3.5.2,_10.2];
_13.fld3.1.2 = _13.fld3.5.2 << _9.2;
Goto(bb3)
}
bb3 = {
_9 = ((*_3), _4, _2);
_13.fld3.1.0 = 149_u8 as f64;
_5 = (_9.1,);
_13.fld1 = _10.2;
_9.0 = !(*_3);
_5 = (_9.1,);
_13.fld3.3 = (-699230778_i32) as u8;
Goto(bb4)
}
bb4 = {
_13.fld3.0 = 66_i8 as u8;
_15 = [5173_i16,26036_i16,(-11015_i16),1133_i16];
_13.fld4 = [_10.2,_13.fld3.1.2];
_1 = _13.fld3.0 as u32;
RET = _9.0;
_14 = _13.fld3.3 as i64;
_9.2 = 7_usize as u32;
_16 = [_13.fld3.1.2,_13.fld3.1.2];
RET = !(*_3);
_13.fld3.0 = _13.fld3.3 << _6;
_13.fld3.1.0 = _5.0;
_10.2 = !_13.fld1;
_9.1 = _4 - _4;
_13.fld1 = 1338984597_i32 as i128;
_13.fld3.4 = [2119_i16,19914_i16,(-25480_i16)];
_3 = core::ptr::addr_of_mut!((*_3));
_2 = !_7;
_2 = !_7;
_13.fld1 = -_13.fld3.5.2;
_13.fld4 = [_13.fld3.1.2,_13.fld3.1.2];
_13.fld3.5.1 = _6 << _2;
(*_3) = _9.0;
_10.2 = -_13.fld1;
RET = (*_3) ^ _9.0;
_13.fld3.7 = [_11,_11,_11];
Goto(bb5)
}
bb5 = {
Call(_20 = dump_var(11_usize, 7_usize, Move(_7), 16_usize, Move(_16), 6_usize, Move(_6), 11_usize, Move(_11)), bb6)
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (bool, f64, u32),mut _2: bool,mut _3: *mut bool,mut _4: bool,mut _5: *mut bool,mut _6: *mut bool,mut _7: *mut bool,mut _8: *mut bool,mut _9: *mut bool,mut _10: isize,mut _11: (bool, f64, u32),mut _12: bool,mut _13: *mut bool,mut _14: *mut bool,mut _15: (*mut bool,),mut _16: bool) -> char {
mir! {
type RET = char;
let _17: f64;
let _18: [i128; 2];
let _19: i8;
let _20: *const u32;
let _21: char;
let _22: f64;
let _23: [i32; 8];
let _24: [i128; 2];
let _25: [i128; 2];
let _26: u32;
let _27: f64;
let _28: i32;
let _29: f32;
let _30: [i128; 2];
let _31: Adt57;
let _32: [u32; 6];
let _33: bool;
let _34: i128;
let _35: Adt66;
let _36: ();
let _37: ();
{
RET = '\u{d9af3}';
(*_13) = _11.0;
_9 = core::ptr::addr_of_mut!(_12);
(*_7) = _2;
_14 = core::ptr::addr_of_mut!(_4);
_11.2 = _1.2 << _1.2;
_15 = (_9,);
(*_3) = !_11.0;
_9 = core::ptr::addr_of_mut!((*_6));
(*_3) = _11.2 <= _1.2;
_1.0 = !_4;
(*_5) = _12 <= _11.0;
_4 = (*_13) & (*_9);
_13 = _8;
Goto(bb1)
}
bb1 = {
_11.2 = _1.2 | _1.2;
_11.2 = !_1.2;
(*_9) = !_12;
_15 = (_5,);
_16 = !(*_3);
_5 = _13;
_3 = _15.0;
(*_14) = _16 <= (*_13);
(*_7) = _4;
_1.0 = !(*_5);
_13 = core::ptr::addr_of_mut!((*_3));
_1.1 = -_11.1;
(*_13) = (*_14) | _2;
_1 = _11;
Call((*_9) = fn13(_6, _6, _13, _11), bb2)
}
bb2 = {
(*_3) = (*_14);
(*_14) = (*_8);
(*_14) = (*_3) == _11.0;
_13 = _15.0;
_1.2 = (*_3) as u32;
_19 = 115_i8;
(*_14) = !(*_5);
_1.2 = _11.2;
(*_13) = !_16;
(*_7) = _11.0 > _4;
_2 = _4 >= (*_13);
_12 = (*_9) & (*_5);
_20 = core::ptr::addr_of!(_1.2);
match _19 {
0 => bb1,
1 => bb3,
2 => bb4,
115 => bb6,
_ => bb5
}
}
bb3 = {
_11.2 = _1.2 | _1.2;
_11.2 = !_1.2;
(*_9) = !_12;
_15 = (_5,);
_16 = !(*_3);
_5 = _13;
_3 = _15.0;
(*_14) = _16 <= (*_13);
(*_7) = _4;
_1.0 = !(*_5);
_13 = core::ptr::addr_of_mut!((*_3));
_1.1 = -_11.1;
(*_13) = (*_14) | _2;
_1 = _11;
Call((*_9) = fn13(_6, _6, _13, _11), bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
(*_3) = _16 & (*_14);
_1 = ((*_8), _11.1, _11.2);
(*_8) = _11.0;
_19 = -64_i8;
(*_6) = _12 | _2;
RET = '\u{68f69}';
Call((*_3) = fn14(_4, _2, _5, _3, _8, _2, _2), bb7)
}
bb7 = {
_25 = [84511507577515858164827471663571117660_i128,(-28566375317957088203765318156958171747_i128)];
_8 = core::ptr::addr_of_mut!((*_7));
(*_14) = _11.0 | _2;
_5 = core::ptr::addr_of_mut!((*_7));
_1.2 = _11.2;
_7 = core::ptr::addr_of_mut!((*_6));
RET = '\u{1a1df}';
_4 = (*_13) > (*_8);
(*_8) = _1.0 | _4;
_24 = [(-41012941201787293881696159966796989751_i128),(-66049277125696539856533467324038484295_i128)];
_7 = _8;
RET = '\u{9d3b2}';
_27 = _1.1 - _1.1;
_23 = [736771614_i32,499352845_i32,236665212_i32,170170482_i32,940106093_i32,190965562_i32,(-1863676526_i32),(-486220318_i32)];
_8 = _5;
(*_8) = _11.0 < (*_14);
_27 = -_1.1;
(*_7) = !_1.0;
(*_6) = _4 >= (*_14);
(*_8) = _11.0 <= _4;
_25 = [147786390741975469187387795079253961833_i128,(-154114043465866650049568823071947549210_i128)];
Goto(bb8)
}
bb8 = {
_15.0 = core::ptr::addr_of_mut!((*_14));
_10 = (-9223372036854775808_isize);
_8 = core::ptr::addr_of_mut!((*_6));
_19 = !66_i8;
Goto(bb9)
}
bb9 = {
_1.1 = -_27;
_28 = 1181660694_i32 ^ 674967269_i32;
_15.0 = core::ptr::addr_of_mut!((*_5));
_31.fld4 = ((*_6), _1.1, (*_20));
(*_13) = _16;
(*_13) = _31.fld4.0;
_7 = core::ptr::addr_of_mut!((*_9));
match _10 {
0 => bb1,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb10 = {
_25 = [84511507577515858164827471663571117660_i128,(-28566375317957088203765318156958171747_i128)];
_8 = core::ptr::addr_of_mut!((*_7));
(*_14) = _11.0 | _2;
_5 = core::ptr::addr_of_mut!((*_7));
_1.2 = _11.2;
_7 = core::ptr::addr_of_mut!((*_6));
RET = '\u{1a1df}';
_4 = (*_13) > (*_8);
(*_8) = _1.0 | _4;
_24 = [(-41012941201787293881696159966796989751_i128),(-66049277125696539856533467324038484295_i128)];
_7 = _8;
RET = '\u{9d3b2}';
_27 = _1.1 - _1.1;
_23 = [736771614_i32,499352845_i32,236665212_i32,170170482_i32,940106093_i32,190965562_i32,(-1863676526_i32),(-486220318_i32)];
_8 = _5;
(*_8) = _11.0 < (*_14);
_27 = -_1.1;
(*_7) = !_1.0;
(*_6) = _4 >= (*_14);
(*_8) = _11.0 <= _4;
_25 = [147786390741975469187387795079253961833_i128,(-154114043465866650049568823071947549210_i128)];
Goto(bb8)
}
bb11 = {
(*_7) = !_31.fld4.0;
(*_13) = !_31.fld4.0;
_27 = 0_usize as f64;
_3 = core::ptr::addr_of_mut!((*_9));
_2 = !_11.0;
_31.fld4.2 = _1.2 & _11.2;
(*_3) = (*_14);
_15.0 = _6;
_26 = !_11.2;
_11 = ((*_6), _31.fld4.1, _31.fld4.2);
_31.fld0 = !(*_13);
_13 = core::ptr::addr_of_mut!((*_8));
_7 = _9;
Goto(bb12)
}
bb12 = {
_16 = _31.fld4.0 ^ (*_5);
_29 = 139_u8 as f32;
_16 = (*_6) != (*_13);
_11.1 = _1.1 - _27;
_9 = _7;
_14 = core::ptr::addr_of_mut!(_12);
_31.fld0 = (*_5);
_31.fld4 = (_4, _11.1, (*_20));
(*_6) = _11.0;
(*_8) = _1.0 & _12;
_1.2 = _11.2 + _31.fld4.2;
(*_14) = !(*_3);
(*_9) = (*_14) & _31.fld4.0;
_17 = _11.1;
_31.fld4.2 = (*_20);
_18 = [75087630945215925929008045404161377064_i128,(-25119971450115529100131060935230629882_i128)];
_8 = core::ptr::addr_of_mut!(_1.0);
_28 = 1879375070_i32;
_31.fld3 = [140629286794576404760037696471751024398_i128,(-2753882586589855762582797157781233896_i128)];
_30 = [61066852406673465697443965251807244286_i128,(-108890866028428069250593214074552267936_i128)];
Goto(bb13)
}
bb13 = {
_29 = 74_u8 as f32;
_26 = _1.2;
_31.fld0 = !_31.fld4.0;
_31.fld2.0 = _6;
(*_8) = !(*_5);
_12 = (*_9);
_9 = core::ptr::addr_of_mut!((*_9));
_31.fld2.0 = core::ptr::addr_of_mut!((*_7));
_23 = [_28,_28,_28,_28,_28,_28,_28,_28];
(*_6) = !_16;
(*_5) = !(*_14);
(*_6) = _1.0;
_21 = RET;
_16 = !(*_13);
(*_6) = !(*_14);
_1.1 = _17;
_31.fld3 = [166566320001200929230527124547390006523_i128,139120768602134558568451698732495926558_i128];
_31.fld4.2 = !(*_20);
(*_14) = !(*_13);
(*_5) = !_4;
Goto(bb14)
}
bb14 = {
_19 = 43_i8 + 120_i8;
_31.fld4.1 = -_1.1;
(*_9) = _4 > (*_8);
_19 = 69_i8 >> _31.fld4.2;
_16 = (*_5) | (*_13);
(*_8) = !_12;
_31.fld4.1 = _28 as f64;
_31.fld4 = ((*_6), _17, _1.2);
_15.0 = core::ptr::addr_of_mut!((*_9));
(*_6) = !_1.0;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(12_usize, 10_usize, Move(_10), 26_usize, Move(_26), 12_usize, Move(_12), 23_usize, Move(_23)), bb16)
}
bb16 = {
Call(_36 = dump_var(12_usize, 21_usize, Move(_21), 28_usize, Move(_28), 2_usize, Move(_2), 37_usize, _37), bb17)
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *mut bool,mut _2: *mut bool,mut _3: *mut bool,mut _4: (bool, f64, u32)) -> bool {
mir! {
type RET = bool;
let _5: [u64; 1];
let _6: ();
let _7: ();
{
RET = !_4.0;
RET = !_4.0;
RET = _4.0 != _4.0;
RET = _4.0 ^ _4.0;
_3 = core::ptr::addr_of_mut!(_4.0);
(*_3) = RET;
_4.0 = RET;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: bool,mut _2: bool,mut _3: *mut bool,mut _4: *mut bool,mut _5: *mut bool,mut _6: bool,mut _7: bool) -> bool {
mir! {
type RET = bool;
let _8: i16;
let _9: [u64; 1];
let _10: i128;
let _11: f32;
let _12: (u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3]);
let _13: ();
let _14: ();
{
_1 = _6;
RET = _6;
RET = !_6;
RET = _7 != _7;
_7 = _1 <= RET;
_3 = _4;
_6 = _7 == RET;
_3 = core::ptr::addr_of_mut!(_1);
_5 = _4;
_9 = [16398226636343293845_u64];
RET = (*_3) ^ _7;
_8 = '\u{7fd3b}' as i16;
_8 = 27863_i16;
_5 = core::ptr::addr_of_mut!((*_3));
_12.7 = [9963047947347303525_u64,15204657564040814078_u64,159661207775783784_u64];
RET = !(*_3);
(*_5) = _6 ^ RET;
_12.5.1 = 2962566975_u32 as isize;
_12.4 = [_8,_8,_8];
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(14_usize, 2_usize, Move(_2), 1_usize, Move(_1), 8_usize, Move(_8), 14_usize, _14), bb2)
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [u128; 1],mut _2: i64,mut _3: Adt51,mut _4: u8,mut _5: *mut (u16,),mut _6: f64,mut _7: i128,mut _8: i128) -> [i128; 2] {
mir! {
type RET = [i128; 2];
let _9: ([i16; 3], ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)), char, bool, i16, ([i16; 3], f64));
let _10: [i128; 2];
let _11: bool;
let _12: u64;
let _13: u64;
let _14: [u64; 3];
let _15: Adt51;
let _16: Adt54;
let _17: char;
let _18: u128;
let _19: isize;
let _20: isize;
let _21: ();
let _22: ();
{
(*_5) = (19606_u16,);
RET = [_8,_8];
(*_5) = (43624_u16,);
_9.1.1.1 = -_6;
_9.1.1.1 = _6;
(*_5).0 = !60779_u16;
SetDiscriminant(_3, 1);
Goto(bb1)
}
bb1 = {
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.2 = false as i128;
place!(Field::<(f64,)>(Variant(_3, 1), 2)) = (_6,);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.0 = Field::<(f64,)>(Variant(_3, 1), 2).0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).0 = 1901390206_u32 as u8;
_9.1.5 = ((-1216297043_i32), _9.1.1.1);
_5 = core::ptr::addr_of_mut!((*_5));
_9.1.5 = ((-1159071677_i32), Field::<(f64,)>(Variant(_3, 1), 2).0);
_9.5.1 = (-15757_i16) as f64;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.0 = _6;
_9.1.1.0 = _9.1.5.0 - _9.1.5.0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5 = (_6, (-9223372036854775808_isize), _8);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).3 = !_4;
_9.1.3 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.2 as isize;
(*_5).0 = !44858_u16;
place!(Field::<*mut (u16,)>(Variant(_3, 1), 3)) = _5;
_9.1.1 = (_9.1.5.0, Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.0);
RET = [_7,_7];
_9.1.5.1 = _6;
place!(Field::<(f64,)>(Variant(_3, 1), 2)).0 = (*_5).0 as f64;
Goto(bb2)
}
bb2 = {
_9.1.0 = [(-1625_i16),28888_i16,25039_i16];
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).4 = _9.1.0;
_9.1.5 = (_9.1.1.0, Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.0);
_9.1.3 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1 * Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1;
place!(Field::<[u64; 1]>(Variant(_3, 1), 0)) = [2938569921820649038_u64];
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5 = (_9.1.1.1, _9.1.3, _7);
_9.2 = '\u{7e9e0}';
_9.5 = (_9.1.0, Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.0);
_9.2 = '\u{ed985}';
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).6 = _5;
_9.2 = '\u{89311}';
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).2 = !(*_5).0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5.0 = _9.1.5.1 - _9.5.1;
(*_5) = (Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).2,);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).4 = [31244_i16,(-26305_i16),(-318_i16)];
_9.1.3 = -Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1;
_9.1.5 = (_9.1.1.0, _6);
_10 = [Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.2,_8];
Goto(bb3)
}
bb3 = {
_9.1.1.0 = _9.1.5.0 ^ _9.1.5.0;
_11 = false;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.1 = -Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1;
(*_5) = (Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).2,);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.2 = !_8;
Goto(bb4)
}
bb4 = {
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5 = (_6, Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.1, Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.2);
place!(Field::<*mut (u16,)>(Variant(_3, 1), 3)) = core::ptr::addr_of_mut!((*_5));
_9.1.3 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1 - Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.1;
match _7 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
92499102559017228600514636644275774636 => bb13,
_ => bb12
}
}
bb5 = {
_9.1.1.0 = _9.1.5.0 ^ _9.1.5.0;
_11 = false;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.1 = -Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1;
(*_5) = (Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).2,);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.2 = !_8;
Goto(bb4)
}
bb6 = {
_9.1.0 = [(-1625_i16),28888_i16,25039_i16];
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).4 = _9.1.0;
_9.1.5 = (_9.1.1.0, Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.0);
_9.1.3 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1 * Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1;
place!(Field::<[u64; 1]>(Variant(_3, 1), 0)) = [2938569921820649038_u64];
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5 = (_9.1.1.1, _9.1.3, _7);
_9.2 = '\u{7e9e0}';
_9.5 = (_9.1.0, Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.0);
_9.2 = '\u{ed985}';
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).6 = _5;
_9.2 = '\u{89311}';
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).2 = !(*_5).0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5.0 = _9.1.5.1 - _9.5.1;
(*_5) = (Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).2,);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).4 = [31244_i16,(-26305_i16),(-318_i16)];
_9.1.3 = -Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.1;
_9.1.5 = (_9.1.1.0, _6);
_10 = [Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.2,_8];
Goto(bb3)
}
bb7 = {
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.2 = false as i128;
place!(Field::<(f64,)>(Variant(_3, 1), 2)) = (_6,);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.0 = Field::<(f64,)>(Variant(_3, 1), 2).0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).0 = 1901390206_u32 as u8;
_9.1.5 = ((-1216297043_i32), _9.1.1.1);
_5 = core::ptr::addr_of_mut!((*_5));
_9.1.5 = ((-1159071677_i32), Field::<(f64,)>(Variant(_3, 1), 2).0);
_9.5.1 = (-15757_i16) as f64;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.0 = _6;
_9.1.1.0 = _9.1.5.0 - _9.1.5.0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5 = (_6, (-9223372036854775808_isize), _8);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).3 = !_4;
_9.1.3 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.2 as isize;
(*_5).0 = !44858_u16;
place!(Field::<*mut (u16,)>(Variant(_3, 1), 3)) = _5;
_9.1.1 = (_9.1.5.0, Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).1.0);
RET = [_7,_7];
_9.1.5.1 = _6;
place!(Field::<(f64,)>(Variant(_3, 1), 2)).0 = (*_5).0 as f64;
Goto(bb2)
}
bb8 = {
Return()
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
_9.2 = '\u{58dd4}';
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).0 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).3 + Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).3;
_9.1.5 = (_9.1.1.0, Field::<(f64,)>(Variant(_3, 1), 2).0);
_9.1.0 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).4;
_9.1.1.0 = _9.1.5.0 | _9.1.5.0;
_9.1.2 = _9.2;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).7 = [41184877001408509_u64,1133247709541926384_u64,17977061001870160616_u64];
SetDiscriminant(_3, 1);
_12 = _8 as u64;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).7 = [_12,_12,_12];
(*_5).0 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).2 | Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).2;
_14 = [_12,_12,_12];
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.0 = 985156470_u32 as f64;
Goto(bb14)
}
bb14 = {
place!(Field::<[u64; 1]>(Variant(_3, 1), 0)) = [_12];
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).5.2 = !_7;
_9.5.0 = [22211_i16,(-22458_i16),(-32272_i16)];
_9.3 = _11;
(*_5) = (Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).2,);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.0 = -_6;
_9.5.1 = -Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.0;
_9.4 = 31256_i16 & 22414_i16;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).7 = [_12,_12,_12];
_9.0 = [_9.4,_9.4,_9.4];
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5;
_13 = Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).3 as u64;
_9.5.1 = -Field::<(f64,)>(Variant(_3, 1), 2).0;
_9.1.0 = [_9.4,_9.4,_9.4];
_9.1.1.0 = _9.1.5.0;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).3 = _9.1.2 as u8;
_7 = !Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1).5.2;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1 = (_6, _9.1.3, _8);
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).1.0 = _9.5.1;
place!(Field::<(u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3])>(Variant(_3, 1), 1)).7 = [_12,_13,_13];
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(15_usize, 10_usize, Move(_10), 11_usize, Move(_11), 14_usize, Move(_14), 13_usize, Move(_13)), bb16)
}
bb16 = {
Call(_21 = dump_var(15_usize, 4_usize, Move(_4), 22_usize, _22, 22_usize, _22, 22_usize, _22), bb17)
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{38cb6}'), std::hint::black_box(95153445675135354569956667182333594883_u128), std::hint::black_box(88_i8), std::hint::black_box(9614_i16), std::hint::black_box(1029183711_i32), std::hint::black_box((-8279786551043878916_i64)), std::hint::black_box(6093_u16), std::hint::black_box(3_usize), std::hint::black_box(254419225439742125_u64));
                
            }
#[derive(Debug,Copy,Clone)]
pub struct Adt50 {
fld0: ([i16; 3], f64),
fld1: u8,
fld2: *const u32,
fld3: u32,
fld4: i16,
fld5: usize,
fld6: f32,
}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: bool,
fld1: *const ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)),
fld2: [u128; 1],
fld3: f32,
fld4: (i128, (*mut (u16,), u128), (*mut (u16,), u128)),
fld5: [u32; 6],

},
Variant1{
fld0: [u64; 1],
fld1: (u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3]),
fld2: (f64,),
fld3: *mut (u16,),

},
Variant2{
fld0: ([i16; 3], f64),
fld1: *mut u128,
fld2: f32,
fld3: *mut bool,
fld4: (*mut (u16,), u128),
fld5: i32,
fld6: *const *const (f64, isize, i128),
fld7: (i32, f64),

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt52 {
fld0: i64,
fld1: *mut u128,
fld2: isize,
fld3: i8,
fld4: *mut (*mut (u16,), u128),
fld5: *const f64,
}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: (*mut (u16,), u128),
fld1: [i16; 4],
fld2: *const *const (f64, isize, i128),
fld3: u128,
fld4: *const *const u32,
fld5: ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)),
fld6: [u128; 1],

},
Variant1{
fld0: u128,
fld1: [char; 8],
fld2: (u32, *mut (u16,), f64),

},
Variant2{
fld0: Adt51,
fld1: [i32; 8],
fld2: u16,
fld3: (f64, isize, i128),
fld4: ([i16; 3], f64),
fld5: f32,

},
Variant3{
fld0: (*mut bool,),
fld1: (i128, (*mut (u16,), u128), (*mut (u16,), u128)),
fld2: [u64; 1],
fld3: *const *const (f64, isize, i128),

}}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: *const f64,
fld1: u16,
fld2: Adt51,
fld3: (f64, isize, i128),
fld4: (*mut (u16,), u128),

},
Variant1{
fld0: ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)),
fld1: *mut u128,
fld2: [u64; 3],
fld3: (u16,),
fld4: *mut (u16,),

},
Variant2{
fld0: *const ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)),
fld1: (*mut (u16,),),
fld2: isize,
fld3: (i32, f64),
fld4: usize,
fld5: *mut bool,

},
Variant3{
fld0: (*mut bool,),
fld1: [u64; 1],
fld2: ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)),
fld3: ([i16; 3], f64),
fld4: *const u32,
fld5: f32,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt55 {
fld0: i64,
fld1: u128,
fld2: (*mut (u16,), u128),
fld3: i8,
fld4: i16,
}
#[derive(Debug)]
pub enum Adt56 {
Variant0{
fld0: (u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3]),
fld1: u16,
fld2: [u64; 1],
fld3: i8,
fld4: Adt51,
fld5: [u32; 6],

},
Variant1{
fld0: [i16; 4],
fld1: Adt54,
fld2: [u64; 3],
fld3: u64,
fld4: u16,
fld5: u8,
fld6: i64,

},
Variant2{
fld0: *const (f64, isize, i128),
fld1: char,
fld2: (bool, f64, u32),
fld3: i8,
fld4: u16,

},
Variant3{
fld0: bool,
fld1: [i16; 3],
fld2: [i32; 8],
fld3: (i128, (*mut (u16,), u128), (*mut (u16,), u128)),
fld4: (u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3]),
fld5: (bool, f64, u32),
fld6: *const *const u32,

}}
#[derive(Debug)]
pub struct Adt57 {
fld0: bool,
fld1: *mut u128,
fld2: (*mut bool,),
fld3: [i128; 2],
fld4: (bool, f64, u32),
}
#[derive(Debug)]
pub struct Adt58 {
fld0: i64,
fld1: i128,
fld2: u128,
fld3: (u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3]),
fld4: [i128; 2],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt59 {
fld0: *const *const (f64, isize, i128),
fld1: (i32, f64),
fld2: usize,
fld3: ([i16; 3], ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)), char, bool, i16, ([i16; 3], f64)),
}
#[derive(Debug)]
pub enum Adt60 {
Variant0{
fld0: (i32, f64),
fld1: (u16,),
fld2: (u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3]),
fld3: Adt53,
fld4: i16,
fld5: Adt54,
fld6: [char; 8],
fld7: i128,

},
Variant1{
fld0: Adt58,
fld1: (u32, *mut (u16,), f64),
fld2: f64,
fld3: u64,
fld4: (i32, f64),

},
Variant2{
fld0: usize,
fld1: Adt56,
fld2: Adt55,
fld3: *const *const u32,
fld4: f32,
fld5: (f64, isize, i128),
fld6: Adt51,
fld7: (i32, f64),

}}
#[derive(Debug)]
pub enum Adt61 {
Variant0{
fld0: ([i16; 3], f64),
fld1: usize,
fld2: *mut u128,
fld3: i16,

},
Variant1{
fld0: Adt50,
fld1: Adt51,
fld2: *mut u128,
fld3: [i128; 2],
fld4: u16,
fld5: [u128; 1],

},
Variant2{
fld0: isize,

},
Variant3{
fld0: u32,
fld1: *mut (u16,),
fld2: isize,
fld3: Adt55,
fld4: i16,

}}
#[derive(Debug)]
pub enum Adt62 {
Variant0{
fld0: u8,
fld1: [i32; 8],
fld2: (*mut (u16,), u128),
fld3: *mut bool,
fld4: *const (f64, isize, i128),
fld5: f32,

},
Variant1{
fld0: (f64,),
fld1: ([i16; 3], f64),

},
Variant2{
fld0: (*mut (u16,),),
fld1: *const *const u32,
fld2: (u8, (f64, isize, i128), u16, u8, [i16; 3], (f64, isize, i128), *mut (u16,), [u64; 3]),
fld3: *const *const (f64, isize, i128),
fld4: [u64; 1],
fld5: [i16; 4],

},
Variant3{
fld0: *mut (u16,),
fld1: i16,
fld2: [char; 8],

}}
#[derive(Debug)]
pub enum Adt63 {
Variant0{
fld0: *const *const (f64, isize, i128),
fld1: *const (f64, isize, i128),
fld2: ([i16; 3], (i32, f64), char, isize, *mut (*mut (u16,), u128), (i32, f64)),
fld3: (bool, f64, u32),
fld4: Adt50,

},
Variant1{
fld0: *mut (u16,),
fld1: ([i16; 3], f64),
fld2: u32,
fld3: u16,
fld4: Adt55,

}}
#[derive(Debug)]
pub enum Adt64 {
Variant0{
fld0: *mut bool,

},
Variant1{
fld0: bool,
fld1: Adt56,
fld2: Adt60,
fld3: Adt58,
fld4: i64,
fld5: (u16,),

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt65 {
fld0: (*mut (u16,), u128),
fld1: Adt50,
fld2: Adt59,
fld3: [u32; 6],
}
#[derive(Debug)]
pub enum Adt66 {
Variant0{
fld0: (i128, (*mut (u16,), u128), (*mut (u16,), u128)),
fld1: Adt55,
fld2: *const u32,
fld3: i8,
fld4: i16,
fld5: i32,
fld6: i64,
fld7: Adt50,

},
Variant1{
fld0: (*mut (u16,), u128),
fld1: char,
fld2: (*mut (u16,),),
fld3: Adt56,
fld4: u64,
fld5: *mut u128,
fld6: *const *const u32,
fld7: [u32; 6],

},
Variant2{
fld0: ([i16; 3], f64),
fld1: u8,
fld2: *const *const u32,
fld3: *const f64,
fld4: (f64,),
fld5: u128,
fld6: i64,
fld7: i128,

},
Variant3{
fld0: u64,
fld1: *mut bool,
fld2: isize,
fld3: Adt52,
fld4: [u64; 1],
fld5: *const *const (f64, isize, i128),

}}

