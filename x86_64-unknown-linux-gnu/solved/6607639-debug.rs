#![recursion_limit = "256"]
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
pub fn fn0(mut _1: u64,mut _2: char,mut _3: u16,mut _4: i8,mut _5: i16,mut _6: u128,mut _7: i64,mut _8: u32,mut _9: usize,mut _10: u8) -> (f32, char, u8, f64, f32) {
mir! {
type RET = (f32, char, u8, f64, f32);
let _11: [usize; 5];
let _12: (u32, [usize; 5], u32, i32);
let _13: [isize; 4];
let _14: Adt59;
let _15: usize;
let _16: [i64; 5];
let _17: (*mut i32,);
let _18: char;
let _19: Adt52;
let _20: isize;
let _21: [i64; 5];
let _22: (u32, [usize; 5], u32, i32);
let _23: [i64; 5];
let _24: (f32, char, u8, f64, f32);
let _25: ([isize; 8], usize, u32, [isize; 8]);
let _26: isize;
let _27: f32;
let _28: i64;
let _29: u128;
let _30: i32;
let _31: [char; 4];
let _32: ();
let _33: ();
{
RET.2 = 40_u8 + 160_u8;
RET.0 = 29_u8 as f32;
_10 = 0_u8 | 121_u8;
_4 = 5543545648280829912_u64 as i8;
RET.1 = '\u{c732d}';
RET.4 = 3803494185_u32 as f32;
RET.4 = _10 as f32;
RET.4 = (-72345404897439574100435053827890758332_i128) as f32;
_8 = !4096243551_u32;
RET.3 = 17850112719461210217_u64 as f64;
_7 = (-121_isize) as i64;
_14.fld0 = -9223372036854775807_isize;
_3 = false as u16;
_3 = !22254_u16;
_12.3 = '\u{e230f}' as i32;
RET.4 = _3 as f32;
_12.2 = _8 + _8;
RET.4 = 9256142331314070397_u64 as f32;
_15 = 3_usize * 7_usize;
RET.3 = 134418178472357315921994591567176589738_i128 as f64;
RET.4 = _10 as f32;
_12.2 = _8 & _8;
_7 = (-649975328654989473_i64) << _3;
_6 = _7 as u128;
_14 = Adt59 { fld0: (-9223372036854775808_isize) };
Goto(bb1)
}
bb1 = {
RET.1 = '\u{3bba1}';
RET.4 = _12.2 as f32;
RET.2 = 8223645157418172953_u64 as u8;
_12.0 = (-83158339801576811069457379018524745094_i128) as u32;
_8 = _12.2;
_4 = !(-26_i8);
_5 = 12306_i16;
_12.1 = [_15,_15,_15,_15,_15];
_14.fld0 = _12.2 as isize;
_1 = !12401297902648805763_u64;
_12.3 = -1943860485_i32;
_14 = Adt59 { fld0: (-9223372036854775808_isize) };
RET.0 = 21650662582484707515975290407608497979_i128 as f32;
_3 = _1 as u16;
Goto(bb2)
}
bb2 = {
_12.1 = [_15,_15,_15,_15,_15];
_1 = 16016244315729405490_u64 * 10834365039098693304_u64;
_13 = [_14.fld0,_14.fld0,_14.fld0,_14.fld0];
RET.4 = _7 as f32;
match _14.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb3 = {
RET.1 = '\u{3bba1}';
RET.4 = _12.2 as f32;
RET.2 = 8223645157418172953_u64 as u8;
_12.0 = (-83158339801576811069457379018524745094_i128) as u32;
_8 = _12.2;
_4 = !(-26_i8);
_5 = 12306_i16;
_12.1 = [_15,_15,_15,_15,_15];
_14.fld0 = _12.2 as isize;
_1 = !12401297902648805763_u64;
_12.3 = -1943860485_i32;
_14 = Adt59 { fld0: (-9223372036854775808_isize) };
RET.0 = 21650662582484707515975290407608497979_i128 as f32;
_3 = _1 as u16;
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
_6 = !89865665534119905690383627002091435954_u128;
RET.1 = '\u{4031d}';
_14.fld0 = !9223372036854775807_isize;
_8 = _12.2;
_11 = [_15,_15,_15,_15,_15];
_14 = Adt59 { fld0: 9223372036854775807_isize };
_19.fld0.0 = _12.3 as f32;
RET.3 = _12.3 as f64;
_19.fld4.2 = _3 as u32;
_18 = '\u{a7df9}';
Goto(bb8)
}
bb8 = {
RET.1 = _18;
RET.1 = _18;
_19.fld0.4 = _19.fld0.0;
_16 = [_7,_7,_7,_7,_7];
_19.fld0.1 = _18;
_19.fld1 = _1 as f64;
RET = (_19.fld0.4, _18, _10, _19.fld1, _19.fld0.4);
_19.fld3.0 = core::ptr::addr_of_mut!(_19.fld4.3);
_16 = [_7,_7,_7,_7,_7];
_10 = 199_u8;
_19.fld4.3 = _14.fld0 as i32;
_3 = 13704_u16;
_19.fld0.0 = _19.fld0.4;
_19.fld6 = core::ptr::addr_of_mut!(_4);
_12.3 = _19.fld4.3 - _19.fld4.3;
_19.fld4 = (_8, _12.1, _12.2, _12.3);
_4 = !123_i8;
_6 = 257578009838375707939505976680519707291_u128 | 322988677597189811704048720994399414389_u128;
_2 = _18;
_9 = _15;
_19.fld7 = !_15;
_17 = (_19.fld3.0,);
_21 = [_7,_7,_7,_7,_7];
_12.3 = _19.fld4.3 << _7;
_15 = _10 as usize;
_11 = [_15,_19.fld7,_19.fld7,_9,_19.fld7];
RET.2 = _10 ^ _10;
_19.fld0.2 = _10 % 244_u8;
Call(RET = fn1(_18, _12.2, _19.fld0.4, _19.fld4.1, _13, _19.fld3.0, _10, _19.fld0.1, _8, _19.fld6, Move(_14), _19.fld3, _12.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12.3 = _19.fld4.3;
_19.fld3.0 = core::ptr::addr_of_mut!(_19.fld4.3);
_9 = _19.fld7 - _15;
_13 = [(-110_isize),124_isize,92_isize,9223372036854775807_isize];
_19.fld7 = _15;
_22.3 = (-78757806866323536920369347411935909575_i128) as i32;
_19.fld4.1 = _12.1;
_3 = _19.fld4.3 as u16;
_19.fld4.3 = !_12.3;
_22.2 = _8 * _19.fld4.0;
_19.fld4 = (_22.2, _11, _12.2, _22.3);
_22 = (_19.fld4.0, _12.1, _19.fld4.2, _19.fld4.3);
RET = (_19.fld0.0, _18, _19.fld0.2, _19.fld1, _19.fld0.0);
_10 = _19.fld0.2 / 59_u8;
_22.1 = [_15,_19.fld7,_15,_9,_9];
RET.2 = !_19.fld0.2;
_19.fld0.3 = -_19.fld1;
_19.fld0.3 = -_19.fld1;
_12 = (_19.fld4.0, _11, _22.0, _19.fld4.3);
RET.4 = -_19.fld0.0;
_19.fld0.2 = _1 as u8;
Goto(bb10)
}
bb10 = {
RET.1 = _2;
_13 = [38_isize,(-9223372036854775808_isize),75_isize,72_isize];
_24.0 = _19.fld0.0;
_26 = 9223372036854775807_isize;
_21 = _16;
_24.3 = -_19.fld1;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
12306 => bb11,
_ => bb9
}
}
bb11 = {
_19.fld3.0 = core::ptr::addr_of_mut!(_12.3);
_25.1 = _9 ^ _19.fld7;
_19.fld4.3 = _26 as i32;
_24.2 = _1 as u8;
_12.3 = -_19.fld4.3;
Call(_22.3 = fn10(_19.fld4.1, _10, _6, _3, _22.1, _6, _18, _19.fld4.1, _3, _19.fld0.2, _26), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_1 = _19.fld4.3 as u64;
RET.1 = _18;
_19.fld0.0 = -_24.0;
_10 = !_24.2;
_12.3 = _19.fld4.3;
_24.4 = _10 as f32;
_19.fld7 = _26 as usize;
_19.fld5 = core::ptr::addr_of_mut!(_25.3);
_25.2 = _12.2;
_17 = (_19.fld3.0,);
RET = (_24.4, _19.fld0.1, _19.fld0.2, _19.fld1, _24.4);
_19.fld0.3 = -_19.fld1;
_24.1 = _19.fld0.1;
_15 = _25.1 | _9;
_19.fld0.3 = _19.fld1 / (-0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000015724725511276265_f64);
_22.3 = !_19.fld4.3;
_9 = !_15;
RET = (_24.4, _2, _24.2, _24.3, _19.fld0.4);
RET.0 = _24.4;
_5 = !(-25836_i16);
_27 = _24.4;
_24 = _19.fld0;
_25.0 = [_26,_26,_26,_26,_26,_26,_26,_26];
_6 = 89098119768421740599855922555388671908_u128;
_19.fld4.1 = [_9,_15,_15,_15,_15];
RET.0 = _4 as f32;
RET.0 = -_27;
_23 = [_7,_7,_7,_7,_7];
_19.fld4.3 = _19.fld0.4 as i32;
match _6 {
0 => bb1,
1 => bb2,
2 => bb11,
89098119768421740599855922555388671908 => bb13,
_ => bb4
}
}
bb13 = {
_4 = (-94_i8);
_25.1 = !_15;
_3 = 45568_u16;
RET = (_19.fld0.0, _24.1, _19.fld0.2, _19.fld1, _27);
_22.1 = [_9,_25.1,_9,_15,_9];
_4 = (-80_i8);
RET.2 = _10;
_28 = !_7;
_19.fld3.0 = core::ptr::addr_of_mut!(_30);
_22.0 = _19.fld4.2 | _19.fld4.2;
_19.fld0.2 = _24.2 % 103_u8;
RET.0 = 24841607258129932867382952304747040055_i128 as f32;
_29 = !_6;
_19.fld3 = _17;
_24.0 = -_19.fld0.4;
_4 = 86_i8;
_19.fld4 = _12;
Call(_24.3 = core::intrinsics::fmaf64(_19.fld0.3, _19.fld0.3, _19.fld1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17.0 = _19.fld3.0;
_19.fld4 = (_12.0, _22.1, _8, _22.3);
_3 = 48066_u16 >> _15;
_24.2 = !_19.fld0.2;
_7 = !_28;
_20 = _26 - _26;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(0_usize, 9_usize, Move(_9), 23_usize, Move(_23), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(0_usize, 18_usize, Move(_18), 5_usize, Move(_5), 2_usize, Move(_2), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(0_usize, 1_usize, Move(_1), 11_usize, Move(_11), 26_usize, Move(_26), 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: char,mut _2: u32,mut _3: f32,mut _4: [usize; 5],mut _5: [isize; 4],mut _6: *mut i32,mut _7: u8,mut _8: char,mut _9: u32,mut _10: *mut i8,mut _11: Adt59,mut _12: (*mut i32,),mut _13: u32) -> (f32, char, u8, f64, f32) {
mir! {
type RET = (f32, char, u8, f64, f32);
let _14: Adt55;
let _15: Adt55;
let _16: char;
let _17: usize;
let _18: [isize; 8];
let _19: isize;
let _20: f32;
let _21: (f32, *mut i32, u128, (char, i16, i16));
let _22: usize;
let _23: [isize; 8];
let _24: i64;
let _25: usize;
let _26: Adt64;
let _27: i64;
let _28: u32;
let _29: Adt53;
let _30: [usize; 5];
let _31: Adt51;
let _32: ((*const *const u64, isize, *const [u32; 2], isize, bool), *const *const [u32; 2]);
let _33: bool;
let _34: (u32, [usize; 5], u32, i32);
let _35: Adt49;
let _36: ();
let _37: ();
{
RET.2 = !_7;
RET.4 = -_3;
RET.3 = _9 as f64;
(*_6) = 818965920_i32 + (-1511290673_i32);
_8 = _1;
(*_6) = 1772823613_i32 >> _13;
RET.1 = _8;
(*_10) = _11.fld0 as i8;
_14.fld1 = [_7,_7];
RET.0 = _3;
(*_10) = -14_i8;
_15 = Adt55 { fld0: 64596_u16,fld1: _14.fld1 };
(*_6) = -(-1209057939_i32);
_5 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0];
(*_6) = -1554863755_i32;
Call(_4 = fn2(_12.0, _11.fld0, _11.fld0, _15.fld0, _10, (*_6), _5, _11.fld0, _15, _8, _7, _5, Move(_11), _12, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.3 = (-72_isize) as f64;
_8 = _1;
_16 = _1;
RET.3 = (-18788_i16) as f64;
_7 = 68_u8;
_5 = [(-82_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _13;
RET.1 = _8;
Goto(bb2)
}
bb2 = {
RET.4 = -_3;
_4 = [16859611218379854123_usize,0_usize,4864662869015434259_usize,363418896196881565_usize,4_usize];
_9 = _13 + _2;
RET.2 = _7;
_17 = !243289881449215745_usize;
_3 = _2 as f32;
RET.0 = _3 * _3;
_14.fld0 = !_15.fld0;
_12 = (_6,);
RET.1 = _16;
RET.4 = -_3;
_18 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_18 = [106_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-42_isize),(-9223372036854775808_isize),52_isize,(-9223372036854775808_isize)];
_16 = _8;
RET.0 = (-5620463116431538467_i64) as f32;
_15.fld0 = _14.fld0 + _14.fld0;
_7 = _16 as u8;
_15.fld0 = _14.fld0;
_16 = _1;
RET.2 = _3 as u8;
_17 = 3_usize;
_19 = _5[_17];
_21.1 = core::ptr::addr_of_mut!((*_6));
_21.3.1 = -(-21669_i16);
_12 = (_21.1,);
_21.2 = !247765149669469694754985601203611619684_u128;
_15 = Adt55 { fld0: _14.fld0,fld1: _14.fld1 };
match _5[_17] {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb3 = {
RET.3 = (-72_isize) as f64;
_8 = _1;
_16 = _1;
RET.3 = (-18788_i16) as f64;
_7 = 68_u8;
_5 = [(-82_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _13;
RET.1 = _8;
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
_21.3.0 = _16;
RET.3 = _21.2 as f64;
_15.fld1 = _14.fld1;
RET.0 = _15.fld0 as f32;
_17 = _8 as usize;
_21.3.0 = _8;
RET.0 = _3;
_15 = Adt55 { fld0: _14.fld0,fld1: _14.fld1 };
_23 = _18;
RET.3 = _17 as f64;
(*_10) = _21.3.1 as i8;
match _19 {
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_5 = [_19,_19,_19,_19];
_15.fld1 = [_7,_7];
_18 = _23;
RET.2 = _7 << _21.3.1;
_10 = core::ptr::addr_of_mut!((*_10));
_16 = _21.3.0;
_12 = (_6,);
(*_6) = _21.3.1 as i32;
Goto(bb10)
}
bb10 = {
RET.1 = _8;
(*_10) = 5_i8 ^ (-78_i8);
_5 = [_19,_19,_19,_19];
_14 = Adt55 { fld0: _15.fld0,fld1: _15.fld1 };
_21.1 = _12.0;
_21.3.1 = (-5828_i16);
_24 = _21.3.0 as i64;
(*_10) = 36_i8 ^ 38_i8;
_7 = 255_u8;
_8 = _16;
_22 = (-17375364607544232711350599212073462192_i128) as usize;
_15.fld0 = _14.fld0;
RET.1 = _16;
RET.3 = (*_6) as f64;
Call(_26.fld0.fld1.0 = fn8(_10, _9, _14.fld0, _14, _19, _3, _17, _23, _19, _9), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_26.fld0.fld4.fld3.1 = !_17;
_26.fld0.fld4.fld3.0 = [_19,_19,_19,_19,_19,_19,_19,_19];
_26.fld0.fld0 = [_2,_9];
_26.fld0.fld1.2.2 = (_6,);
(*_6) = (-94932941_i32);
_26.fld0.fld1.2.0 = (_16, _21.3.1, _21.3.1);
_29.fld3.fld1 = (*_10) - (*_10);
_12.0 = _6;
_7 = !2_u8;
_29.fld4.fld3 = (_23, _22, _2, _23);
_24 = _3 as i64;
_27 = -_24;
_26.fld2.2 = -_26.fld0.fld1.2.0.2;
_29.fld4.fld2 = _24;
_30 = [_17,_22,_29.fld4.fld3.1,_29.fld4.fld3.1,_22];
_29.fld3.fld1 = _26.fld2.2 as i8;
RET.1 = _21.3.0;
_8 = _26.fld0.fld1.2.0.0;
_29.fld2.0 = !_29.fld3.fld1;
_26.fld2 = (_8, _21.3.1, _26.fld0.fld1.2.0.1);
_19 = 9223372036854775807_isize;
_14.fld0 = _15.fld0 << (*_10);
_26.fld0.fld2 = (_29.fld3.fld1,);
_29.fld1.3 = [_27,_27,_24,_24,_24];
_29.fld1.1 = !_29.fld4.fld3.1;
_26.fld1.2.0 = core::ptr::addr_of_mut!((*_6));
match (*_6) {
340282366920938463463374607431673278515 => bb13,
_ => bb12
}
}
bb12 = {
RET.1 = _8;
(*_10) = 5_i8 ^ (-78_i8);
_5 = [_19,_19,_19,_19];
_14 = Adt55 { fld0: _15.fld0,fld1: _15.fld1 };
_21.1 = _12.0;
_21.3.1 = (-5828_i16);
_24 = _21.3.0 as i64;
(*_10) = 36_i8 ^ 38_i8;
_7 = 255_u8;
_8 = _16;
_22 = (-17375364607544232711350599212073462192_i128) as usize;
_15.fld0 = _14.fld0;
RET.1 = _16;
RET.3 = (*_6) as f64;
Call(_26.fld0.fld1.0 = fn8(_10, _9, _14.fld0, _14, _19, _3, _17, _23, _19, _9), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_26.fld0.fld4.fld1.4 = false | true;
_26.fld0.fld1.3 = _29.fld1.3;
_26.fld0.fld5 = !_29.fld4.fld3.2;
_29.fld4.fld3.0 = [_19,_19,_19,_19,_19,_19,_19,_19];
_29.fld1.2.0.2 = _26.fld2.1;
_26.fld0.fld1.2.2.0 = _21.1;
_26.fld1.0.2 = _14.fld0 as i16;
_29.fld4.fld1.1 = !_19;
_26.fld1.1 = [_29.fld4.fld3.1,_17,_26.fld0.fld4.fld3.1,_29.fld1.1,_26.fld0.fld4.fld3.1];
_29.fld1.2.0 = (_1, _26.fld1.0.2, _21.3.1);
_29.fld1.2 = (_26.fld0.fld1.2.0, _4, _26.fld0.fld1.2.2);
_26.fld0.fld1.4 = _14.fld0 + _14.fld0;
_26.fld1.0.2 = -_26.fld2.1;
_26.fld0.fld4.fld3.2 = _13;
_29.fld1.6 = !_26.fld0.fld1.4;
_21 = (_3, _6, 36233635722902618629849328272235510687_u128, _26.fld2);
_14.fld0 = _29.fld1.6;
_29.fld4.fld1.3 = _29.fld1.1 as isize;
RET.3 = _29.fld2.0 as f64;
_29.fld4.fld2 = _24 * _24;
match _26.fld0.fld1.2.0.2 {
340282366920938463463374607431768205628 => bb14,
_ => bb2
}
}
bb14 = {
_26.fld0.fld1.5 = _26.fld0.fld1.0;
_26.fld0.fld4.fld2 = !_29.fld4.fld2;
_28 = _26.fld0.fld4.fld3.2;
_33 = _26.fld0.fld4.fld1.4 ^ _26.fld0.fld4.fld1.4;
_29.fld4.fld3.2 = _9;
_35.fld0 = _21.0 + _21.0;
_29.fld1.0 = core::ptr::addr_of!(_26.fld0.fld1.6);
_34.1 = [_26.fld0.fld4.fld3.1,_22,_17,_29.fld1.1,_17];
_26.fld0.fld1.1 = !_29.fld1.1;
_29.fld0 = _26.fld0.fld0;
_26.fld1 = _29.fld1.2;
_29.fld1.2 = (_26.fld1.0, _30, _26.fld0.fld1.2.2);
_34.3 = (*_6);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(1_usize, 18_usize, Move(_18), 27_usize, Move(_27), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(1_usize, 1_usize, Move(_1), 5_usize, Move(_5), 7_usize, Move(_7), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(1_usize, 22_usize, Move(_22), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: *mut i32,mut _2: isize,mut _3: isize,mut _4: u16,mut _5: *mut i8,mut _6: i32,mut _7: [isize; 4],mut _8: isize,mut _9: Adt55,mut _10: char,mut _11: u8,mut _12: [isize; 4],mut _13: Adt59,mut _14: (*mut i32,),mut _15: u8) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _16: u16;
let _17: u128;
let _18: u64;
let _19: (u32, [usize; 5], u32, i32);
let _20: [u32; 5];
let _21: (i8,);
let _22: Adt60;
let _23: [usize; 5];
let _24: ([isize; 8], usize, u32, [isize; 8]);
let _25: Adt52;
let _26: isize;
let _27: (*const *const u64, isize, *const [u32; 2], isize, bool);
let _28: u64;
let _29: (i8,);
let _30: [char; 4];
let _31: char;
let _32: (f32, char, u8, f64, f32);
let _33: isize;
let _34: usize;
let _35: Adt53;
let _36: [u8; 2];
let _37: Adt55;
let _38: *const *const u64;
let _39: Adt55;
let _40: bool;
let _41: (*const u16, usize, ((char, i16, i16), [usize; 5], (*mut i32,)), [i64; 5], u16, *const u16, u16);
let _42: Adt59;
let _43: *mut [i64; 5];
let _44: [char; 4];
let _45: Adt58;
let _46: ();
let _47: ();
{
RET = [2_usize,6235386431869299409_usize,1_usize,13945340151275033866_usize,7_usize];
_6 = (*_1) * (*_1);
(*_1) = _6;
_7 = [_13.fld0,_8,_3,_2];
(*_1) = _6;
_15 = _11 | _11;
_6 = _15 as i32;
_11 = !_15;
_7 = [_13.fld0,_3,_13.fld0,_13.fld0];
_8 = !_3;
(*_1) = 12316_i16 as i32;
(*_1) = 2_usize as i32;
match _9.fld0 {
0 => bb1,
64596 => bb3,
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
RET = [7_usize,9933556706057444576_usize,6_usize,15312378365323555789_usize,10527628606017680009_usize];
RET = [7_usize,17242210005464975698_usize,1_usize,36590996444773490_usize,5_usize];
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 293652968330913028876224666168452454425_u128 as i32;
_17 = _8 as u128;
Goto(bb4)
}
bb4 = {
(*_1) = !_6;
_6 = (*_1) & (*_1);
(*_1) = -_6;
_19.0 = _10 as u32;
_19.2 = !_19.0;
_11 = _15;
Goto(bb5)
}
bb5 = {
_17 = (-3785999734069549174_i64) as u128;
_9.fld1 = [_15,_11];
_9.fld0 = 110364468312672373191671401694896329909_i128 as u16;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = _6;
_19.1 = [12142509952516356541_usize,5_usize,5_usize,11693624612931408332_usize,3030817964342956750_usize];
_18 = !5704090727023172309_u64;
Goto(bb6)
}
bb6 = {
RET = [0_usize,5_usize,3761193002857353272_usize,2_usize,8878568230542102982_usize];
_22.fld1.3 = (_10, (-674_i16), 28752_i16);
_22.fld0 = core::ptr::addr_of!(_18);
_24.1 = 6582814202936316775_usize & 2_usize;
_24.3 = [_2,_13.fld0,_8,_3,_2,_13.fld0,_3,_3];
_19.3 = _6;
_24.3 = [_13.fld0,_2,_13.fld0,_3,_3,_3,_13.fld0,_8];
_22.fld1.3.2 = -_22.fld1.3.1;
_15 = !_11;
_2 = _19.2 as isize;
_4 = _9.fld0 * _9.fld0;
_24.2 = _19.2;
Goto(bb7)
}
bb7 = {
_2 = -_13.fld0;
_25.fld6 = core::ptr::addr_of_mut!((*_5));
_20 = [_19.0,_19.0,_19.0,_19.2,_19.0];
_22.fld2 = core::ptr::addr_of!(_9.fld0);
(*_1) = _6;
_22.fld1.3 = (_10, 197_i16, (-32603_i16));
_13 = Adt59 { fld0: _3 };
_22.fld1.3.0 = _10;
_25.fld7 = (-7795754111122650178_i64) as usize;
_11 = _18 as u8;
_25.fld4 = (_19.2, _19.1, _19.2, _19.3);
_25.fld4 = (_19.0, _19.1, _19.2, (*_1));
Call(_1 = fn3(_25.fld7, _3, _5, _22.fld0, _14, _22.fld2, _22.fld1.3, _7, _3, _22.fld2, _20, _4, _2, _19.2, _13.fld0, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_16 = !_9.fld0;
_22.fld1.2 = _17 % 198526292840765065644448693580176544296_u128;
_25.fld0.1 = _10;
_2 = _8;
_19.1 = [_25.fld7,_25.fld7,_25.fld7,_24.1,_24.1];
_25.fld0.0 = _8 as f32;
_22.fld1.3.1 = _22.fld1.3.2;
_22.fld1.3.2 = -_22.fld1.3.1;
_27.0 = core::ptr::addr_of!(_22.fld0);
_28 = _18 / 11335183979177601363_u64;
_24.0 = [_8,_3,_13.fld0,_2,_2,_3,_13.fld0,_8];
_25.fld4 = (_19.0, _19.1, _24.2, _6);
_25.fld2 = core::ptr::addr_of!(_22.fld0);
_29.0 = (*_5);
_9.fld0 = _4;
_30 = [_22.fld1.3.0,_10,_22.fld1.3.0,_25.fld0.1];
_29.0 = (*_5);
_25.fld1 = _29.0 as f64;
_25.fld0.3 = -_25.fld1;
_26 = !_3;
match _22.fld1.3.1 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb10,
340282366920938463463374607431768178853 => bb12,
_ => bb11
}
}
bb9 = {
_2 = -_13.fld0;
_25.fld6 = core::ptr::addr_of_mut!((*_5));
_20 = [_19.0,_19.0,_19.0,_19.2,_19.0];
_22.fld2 = core::ptr::addr_of!(_9.fld0);
(*_1) = _6;
_22.fld1.3 = (_10, 197_i16, (-32603_i16));
_13 = Adt59 { fld0: _3 };
_22.fld1.3.0 = _10;
_25.fld7 = (-7795754111122650178_i64) as usize;
_11 = _18 as u8;
_25.fld4 = (_19.2, _19.1, _19.2, _19.3);
_25.fld4 = (_19.0, _19.1, _19.2, (*_1));
Call(_1 = fn3(_25.fld7, _3, _5, _22.fld0, _14, _22.fld2, _22.fld1.3, _7, _3, _22.fld2, _20, _4, _2, _19.2, _13.fld0, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
RET = [0_usize,5_usize,3761193002857353272_usize,2_usize,8878568230542102982_usize];
_22.fld1.3 = (_10, (-674_i16), 28752_i16);
_22.fld0 = core::ptr::addr_of!(_18);
_24.1 = 6582814202936316775_usize & 2_usize;
_24.3 = [_2,_13.fld0,_8,_3,_2,_13.fld0,_3,_3];
_19.3 = _6;
_24.3 = [_13.fld0,_2,_13.fld0,_3,_3,_3,_13.fld0,_8];
_22.fld1.3.2 = -_22.fld1.3.1;
_15 = !_11;
_2 = _19.2 as isize;
_4 = _9.fld0 * _9.fld0;
_24.2 = _19.2;
Goto(bb7)
}
bb11 = {
Return()
}
bb12 = {
_14.0 = core::ptr::addr_of_mut!(_19.3);
_25.fld3.0 = core::ptr::addr_of_mut!(_25.fld4.3);
_27.0 = _25.fld2;
_25.fld0.0 = _25.fld4.3 as f32;
_25.fld4 = _19;
_21 = ((*_5),);
_31 = _22.fld1.3.0;
_22.fld1.0 = _25.fld0.0;
_8 = _3;
_32 = (_25.fld0.0, _31, _15, _25.fld1, _25.fld0.0);
_25.fld0 = (_22.fld1.0, _31, _32.2, _32.3, _32.4);
_21.0 = _32.2 as i8;
_32.4 = -_32.0;
_21.0 = _29.0 * _29.0;
_27.3 = false as isize;
_27.0 = core::ptr::addr_of!(_22.fld0);
_25.fld4.2 = !_24.2;
_16 = 6524242997175341296_i64 as u16;
_32.0 = -_32.4;
RET = _19.1;
Call(_9 = fn5(_31, _25.fld4.2, _25.fld2, _8, _21.0, _32.1, _14, _22.fld1.3.2, _8, _25.fld2, _20, _24.1, _25.fld4.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_22.fld1.3 = (_32.1, 19705_i16, 6994_i16);
_35.fld4.fld3.2 = _25.fld4.2;
_35.fld4.fld1.4 = true;
_35.fld1.4 = _6 as u16;
_35.fld4.fld1.2 = core::ptr::addr_of!(_35.fld0);
_35.fld4.fld3.3 = _24.3;
_25.fld4.1 = _19.1;
_25.fld3.0 = core::ptr::addr_of_mut!(_6);
_35.fld0 = [_25.fld4.2,_25.fld4.0];
_35.fld4.fld1.3 = _32.0 as isize;
match _22.fld1.3.1 {
0 => bb8,
1 => bb10,
2 => bb3,
3 => bb4,
4 => bb5,
19705 => bb14,
_ => bb9
}
}
bb14 = {
_35.fld1.2.2.0 = core::ptr::addr_of_mut!(_25.fld4.3);
_37.fld0 = _4 % 21266_u16;
_39.fld1 = [_15,_15];
_37 = Adt55 { fld0: _4,fld1: _39.fld1 };
_45.fld5.fld1.4 = !_35.fld4.fld1.4;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(2_usize, 21_usize, Move(_21), 24_usize, Move(_24), 18_usize, Move(_18), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(2_usize, 30_usize, Move(_30), 8_usize, Move(_8), 31_usize, Move(_31), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(2_usize, 3_usize, Move(_3), 16_usize, Move(_16), 4_usize, Move(_4), 47_usize, _47), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: usize,mut _2: isize,mut _3: *mut i8,mut _4: *const u64,mut _5: (*mut i32,),mut _6: *const u16,mut _7: (char, i16, i16),mut _8: [isize; 4],mut _9: isize,mut _10: *const u16,mut _11: [u32; 5],mut _12: u16,mut _13: isize,mut _14: u32,mut _15: isize,mut _16: char) -> *mut i32 {
mir! {
type RET = *mut i32;
let _17: isize;
let _18: [char; 4];
let _19: f32;
let _20: ([u32; 5],);
let _21: (u32, [usize; 5], u32, i32);
let _22: [usize; 5];
let _23: [isize; 8];
let _24: [u8; 2];
let _25: *mut i8;
let _26: (char, i16, i16);
let _27: [u32; 2];
let _28: bool;
let _29: (char, i16, i16);
let _30: (f32, char, u8, f64, f32);
let _31: (char, i16, i16);
let _32: [u8; 2];
let _33: ();
let _34: ();
{
_14 = 848065935_u32;
_15 = _2;
RET = _5.0;
(*_4) = 125432373100311338633570318922732160344_u128 as u64;
Call(_3 = fn4(_15, _2, (*_4), _5.0, (*_6), _7.2, _8, (*_10), _9, _15, _15, _9, _15, (*_10), (*RET)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_6) = _12 - _12;
_13 = _9;
(*_4) = 12248030091278817601_u64 / 571887495811222660_u64;
_18 = [_16,_16,_16,_16];
(*_6) = _12 >> _7.1;
RET = core::ptr::addr_of_mut!((*RET));
_9 = !_15;
_6 = core::ptr::addr_of!((*_6));
_12 = !(*_10);
(*_4) = !5524268342784249341_u64;
_7.2 = _7.1;
_1 = !11008041583547146303_usize;
_7 = (_16, (-16609_i16), 16221_i16);
_9 = (-47_i8) as isize;
_5.0 = core::ptr::addr_of_mut!((*RET));
_5.0 = core::ptr::addr_of_mut!((*RET));
(*_4) = _7.2 as u64;
(*_10) = _14 as u16;
(*RET) = -(-178428011_i32);
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
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
_7 = (_16, (-25837_i16), (-18918_i16));
_13 = !_2;
_8 = [_2,_9,_9,_13];
(*_4) = 17040589485342533286_u64;
(*RET) = (-1821941129_i32) << _12;
_16 = _7.0;
(*_10) = _12;
_7 = (_16, (-21552_i16), 11905_i16);
_19 = _7.1 as f32;
_4 = core::ptr::addr_of!((*_4));
_14 = _19 as u32;
_1 = !7_usize;
_9 = 134282326232653055928976417773147410989_i128 as isize;
_21.2 = _14;
_7 = (_16, (-10096_i16), (-28116_i16));
(*_10) = _12;
_21.0 = _14 & _14;
_15 = 63_u8 as isize;
_13 = _2 | _2;
match _7.1 {
0 => bb9,
1 => bb10,
340282366920938463463374607431768201360 => bb12,
_ => bb11
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
_25 = _3;
_17 = _13 - _15;
(*_10) = _7.1 as u16;
_20 = (_11,);
(*_4) = 3991575225664231440_u64 / 7155416790847559302_u64;
_10 = core::ptr::addr_of!(_12);
(*RET) = (-2095885952_i32);
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 179878063_i32;
_21.1 = [_1,_1,_1,_1,_1];
_9 = _13 & _17;
RET = _5.0;
RET = _5.0;
_26.1 = -_7.2;
_7 = (_16, _26.1, _26.1);
RET = core::ptr::addr_of_mut!((*RET));
_22 = [_1,_1,_1,_1,_1];
_13 = 26_i8 as isize;
_14 = !_21.0;
_26.0 = _7.0;
(*RET) = -348189464_i32;
match _2 {
9223372036854775807 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_19 = _1 as f32;
RET = core::ptr::addr_of_mut!(_21.3);
_29.1 = (-34010014607822672377498246602728077402_i128) as i16;
_26 = (_16, _7.2, _7.2);
(*_4) = _21.0 as u64;
_30.0 = _19;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(3_usize, 20_usize, Move(_20), 12_usize, Move(_12), 15_usize, Move(_15), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(3_usize, 9_usize, Move(_9), 7_usize, Move(_7), 1_usize, Move(_1), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: isize,mut _3: u64,mut _4: *mut i32,mut _5: u16,mut _6: i16,mut _7: [isize; 4],mut _8: u16,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: u16,mut _15: i32) -> *mut i8 {
mir! {
type RET = *mut i8;
let _16: [char; 4];
let _17: u16;
let _18: Adt53;
let _19: u8;
let _20: f32;
let _21: f64;
let _22: f64;
let _23: Adt54;
let _24: *mut [usize; 5];
let _25: u128;
let _26: f64;
let _27: char;
let _28: *const u64;
let _29: (f32, char, u8, f64, f32);
let _30: [i64; 5];
let _31: char;
let _32: i32;
let _33: ();
let _34: ();
{
_12 = _9 + _11;
_7 = [_12,_2,_10,_9];
_15 = (*_4) + (*_4);
_9 = _13;
_18.fld1.5 = core::ptr::addr_of!(_5);
_18.fld4.fld1.4 = false;
_18.fld4.fld2 = '\u{5abb2}' as i64;
_8 = _14 & _5;
_18.fld1.6 = _8;
_18.fld1.2.0.2 = _6 * _6;
_18.fld0 = [4155888637_u32,2608320691_u32];
_18.fld6 = [_2,_2,_12,_9,_12,_2,_10,_9];
_7 = [_13,_10,_2,_13];
_18.fld1.2.2 = (_4,);
_7 = [_11,_13,_10,_9];
_18.fld4.fld1.2 = core::ptr::addr_of!(_18.fld0);
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
9223372036854775807 => bb5,
_ => bb4
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
_18.fld1.0 = core::ptr::addr_of!(_8);
_18.fld3.fld1 = !50_i8;
_19 = 164_u8 * 48_u8;
_16 = ['\u{d55ae}','\u{9d543}','\u{7b048}','\u{e3b37}'];
_18.fld3.fld1 = _3 as i8;
RET = core::ptr::addr_of_mut!(_18.fld2.0);
_23.fld4.fld4.fld3.1 = 0_usize >> (*_4);
_20 = _23.fld4.fld4.fld3.1 as f32;
_23.fld7 = core::ptr::addr_of_mut!(_18.fld6);
_23.fld4.fld6 = [_13,_12,_10,_1,_2,_9,_11,_12];
_18.fld4.fld1.2 = core::ptr::addr_of!(_23.fld5);
_23.fld1.fld1.4 = _15 == (*_4);
_18.fld3.fld2 = core::ptr::addr_of!(_3);
_23.fld1.fld3.2 = !979878464_u32;
Goto(bb6)
}
bb6 = {
(*RET) = _18.fld3.fld1 - _18.fld3.fld1;
_18.fld4.fld1.1 = -_2;
_24 = core::ptr::addr_of_mut!(_18.fld1.2.1);
_23.fld4.fld4.fld1.2 = core::ptr::addr_of!(_18.fld0);
_23.fld1.fld1.1 = _20 as isize;
_17 = _14 + _8;
(*_24) = [_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1];
_23.fld1.fld1.4 = _18.fld4.fld1.4;
_18.fld3.fld2 = core::ptr::addr_of!(_3);
_23.fld1.fld1.0 = core::ptr::addr_of!(_18.fld3.fld2);
_23.fld1.fld1.2 = _23.fld4.fld4.fld1.2;
_23.fld1.fld3.2 = 279910964_u32;
_23.fld5 = _18.fld0;
_19 = 209_u8;
_23.fld4.fld1.2.0 = ('\u{a83ff}', _6, _18.fld1.2.0.2);
_23.fld1.fld3.2 = 367629762_u32;
_23.fld4.fld4.fld1.0 = _23.fld1.fld1.0;
_18.fld1.2.0.2 = _17 as i16;
_23.fld4.fld1.0 = core::ptr::addr_of!(_18.fld1.4);
_23.fld4.fld1.3 = [_18.fld4.fld2,_18.fld4.fld2,_18.fld4.fld2,_18.fld4.fld2,_18.fld4.fld2];
_18.fld1.2.0.0 = _23.fld4.fld1.2.0.0;
_23.fld4.fld4.fld3 = (_18.fld6, 0_usize, _23.fld1.fld3.2, _23.fld4.fld6);
_18.fld1.3 = [_18.fld4.fld2,_18.fld4.fld2,_18.fld4.fld2,_18.fld4.fld2,_18.fld4.fld2];
RET = core::ptr::addr_of_mut!((*RET));
_23.fld1.fld3.3 = [_9,_13,_9,_9,_2,_12,_18.fld4.fld1.1,_1];
_18.fld1.2.0.0 = _23.fld4.fld1.2.0.0;
_23.fld4.fld4.fld1.1 = _23.fld1.fld1.1;
_18.fld1.2.0.2 = -_23.fld4.fld1.2.0.2;
_23.fld4.fld1.2.2.0 = core::ptr::addr_of_mut!((*_4));
match _23.fld1.fld3.2 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
367629762 => bb14,
_ => bb13
}
}
bb7 = {
_18.fld1.0 = core::ptr::addr_of!(_8);
_18.fld3.fld1 = !50_i8;
_19 = 164_u8 * 48_u8;
_16 = ['\u{d55ae}','\u{9d543}','\u{7b048}','\u{e3b37}'];
_18.fld3.fld1 = _3 as i8;
RET = core::ptr::addr_of_mut!(_18.fld2.0);
_23.fld4.fld4.fld3.1 = 0_usize >> (*_4);
_20 = _23.fld4.fld4.fld3.1 as f32;
_23.fld7 = core::ptr::addr_of_mut!(_18.fld6);
_23.fld4.fld6 = [_13,_12,_10,_1,_2,_9,_11,_12];
_18.fld4.fld1.2 = core::ptr::addr_of!(_23.fld5);
_23.fld1.fld1.4 = _15 == (*_4);
_18.fld3.fld2 = core::ptr::addr_of!(_3);
_23.fld1.fld3.2 = !979878464_u32;
Goto(bb6)
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
Return()
}
bb14 = {
_18.fld1.2.1 = [_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1];
_18.fld4.fld1.0 = core::ptr::addr_of!(_18.fld3.fld2);
_18.fld1.2.2 = _23.fld4.fld1.2.2;
_18.fld1.2.0 = _23.fld4.fld1.2.0;
_29.2 = _19;
_16 = [_23.fld4.fld1.2.0.0,_23.fld4.fld1.2.0.0,_18.fld1.2.0.0,_18.fld1.2.0.0];
_23.fld7 = core::ptr::addr_of_mut!(_23.fld4.fld6);
_23.fld4.fld4.fld1.4 = _23.fld1.fld1.4;
_23.fld4.fld2.0 = _18.fld3.fld1;
_18.fld1.2.0.2 = -_23.fld4.fld1.2.0.1;
(*_24) = [_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1,_23.fld4.fld4.fld3.1];
_23.fld1.fld1.0 = core::ptr::addr_of!(_23.fld4.fld3.fld2);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(4_usize, 10_usize, Move(_10), 19_usize, Move(_19), 13_usize, Move(_13), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(4_usize, 1_usize, Move(_1), 5_usize, Move(_5), 17_usize, Move(_17), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: char,mut _2: u32,mut _3: *const *const u64,mut _4: isize,mut _5: i8,mut _6: char,mut _7: (*mut i32,),mut _8: i16,mut _9: isize,mut _10: *const *const u64,mut _11: [u32; 5],mut _12: usize,mut _13: u32) -> Adt55 {
mir! {
type RET = Adt55;
let _14: isize;
let _15: [isize; 8];
let _16: bool;
let _17: char;
let _18: (char, i16, i16);
let _19: isize;
let _20: isize;
let _21: (i8,);
let _22: bool;
let _23: u32;
let _24: Adt55;
let _25: [char; 4];
let _26: usize;
let _27: u8;
let _28: f32;
let _29: Adt55;
let _30: ([isize; 8], usize, u32, [isize; 8]);
let _31: f32;
let _32: [u8; 2];
let _33: isize;
let _34: i16;
let _35: *const *mut i32;
let _36: [u32; 5];
let _37: f32;
let _38: (char, i16, i16);
let _39: [u32; 2];
let _40: [u32; 5];
let _41: bool;
let _42: (i8,);
let _43: [isize; 4];
let _44: u64;
let _45: [u32; 5];
let _46: [char; 4];
let _47: ([isize; 8], usize, u32, [isize; 8]);
let _48: [usize; 5];
let _49: isize;
let _50: Adt63;
let _51: i128;
let _52: u32;
let _53: ();
let _54: ();
{
_12 = !0_usize;
_10 = _3;
_5 = _1 as i8;
_6 = _1;
_3 = core::ptr::addr_of!((*_10));
_12 = 54128_u16 as usize;
_6 = _1;
_1 = _6;
RET.fld0 = !65492_u16;
Goto(bb1)
}
bb1 = {
RET.fld0 = 14121_u16 & 12739_u16;
_8 = true as i16;
RET.fld0 = !28452_u16;
_2 = _13;
RET.fld1 = [69_u8,193_u8];
RET.fld0 = 43195_u16;
_9 = _4 ^ _4;
RET.fld1 = [246_u8,168_u8];
RET.fld0 = 24741_u16;
_17 = _1;
RET.fld0 = 29182_u16 & 12808_u16;
_18.1 = _8;
_9 = -_4;
RET.fld1 = [236_u8,65_u8];
_14 = _4 | _9;
RET.fld0 = 48486_u16 * 25343_u16;
_10 = _3;
_19 = !_4;
_14 = _9;
_8 = 21623_u16 as i16;
Goto(bb2)
}
bb2 = {
_12 = 6337414972916207665_usize;
_4 = -_9;
_1 = _6;
RET.fld0 = 15277_u16 + 48289_u16;
_15 = [_14,_4,_9,_19,_14,_4,_14,_19];
_12 = 4_usize;
_18.0 = _17;
RET.fld0 = 234_u8 as u16;
_18.2 = _8 - _18.1;
_11[_12] = _2;
_12 = 3053021325739132736_usize % 5436633897920295391_usize;
RET.fld0 = !17429_u16;
_23 = true as u32;
_12 = 2_usize ^ 5350893686879058103_usize;
_22 = _4 >= _19;
_20 = (-11980209140574686149036423875418324233_i128) as isize;
_12 = 2_usize ^ 16575925123789617568_usize;
_14 = _20 ^ _9;
_16 = !_22;
_3 = _10;
_11 = [_23,_13,_13,_2,_23];
_4 = 35834_u16 as isize;
_19 = _9;
_21 = (_5,);
_18.1 = -_18.2;
Goto(bb3)
}
bb3 = {
_24.fld0 = 20540_u16 << _14;
_10 = core::ptr::addr_of!((*_10));
_23 = _13;
_18 = (_17, _8, _8);
_6 = _1;
_8 = 1644888757960579553_i64 as i16;
_23 = _2;
_9 = 1164421557_i32 as isize;
RET.fld1 = [80_u8,242_u8];
_12 = 13528721187231026909_usize;
_25 = [_6,_6,_18.0,_6];
_18.0 = _1;
_4 = _19;
_23 = _2;
_6 = _17;
_19 = _9 * _20;
_19 = _14;
_27 = 186_u8 << _2;
_18.0 = _1;
_2 = _23 - _13;
_3 = _10;
_24.fld1 = [_27,_27];
_18.1 = !_8;
_18.1 = 49270562433955331593461836541858573362_i128 as i16;
Call(_23 = fn6((*_3), (*_10), _24.fld1, _7.0, _8, (*_3), (*_3), _24.fld1, (*_10), _18, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_24.fld1 = [_27,_27];
Goto(bb5)
}
bb5 = {
_16 = _27 != _27;
_21.0 = _5 | _5;
_9 = !_14;
_18.2 = -_8;
_19 = _14;
_18 = (_17, _8, _8);
_26 = !_12;
match _12 {
0 => bb2,
1 => bb6,
13528721187231026909 => bb8,
_ => bb7
}
}
bb6 = {
_24.fld1 = [_27,_27];
Goto(bb5)
}
bb7 = {
_12 = 6337414972916207665_usize;
_4 = -_9;
_1 = _6;
RET.fld0 = 15277_u16 + 48289_u16;
_15 = [_14,_4,_9,_19,_14,_4,_14,_19];
_12 = 4_usize;
_18.0 = _17;
RET.fld0 = 234_u8 as u16;
_18.2 = _8 - _18.1;
_11[_12] = _2;
_12 = 3053021325739132736_usize % 5436633897920295391_usize;
RET.fld0 = !17429_u16;
_23 = true as u32;
_12 = 2_usize ^ 5350893686879058103_usize;
_22 = _4 >= _19;
_20 = (-11980209140574686149036423875418324233_i128) as isize;
_12 = 2_usize ^ 16575925123789617568_usize;
_14 = _20 ^ _9;
_16 = !_22;
_3 = _10;
_11 = [_23,_13,_13,_2,_23];
_4 = 35834_u16 as isize;
_19 = _9;
_21 = (_5,);
_18.1 = -_18.2;
Goto(bb3)
}
bb8 = {
_30.2 = _24.fld0 as u32;
_25 = [_1,_1,_17,_1];
Goto(bb9)
}
bb9 = {
RET.fld0 = _24.fld0;
_25 = [_1,_1,_17,_6];
_29.fld0 = _24.fld0 ^ _24.fld0;
_12 = _26;
_30.3 = [_14,_9,_19,_9,_14,_20,_19,_14];
_30.2 = _23 & _13;
RET = Adt55 { fld0: _24.fld0,fld1: _24.fld1 };
_30.0 = [_20,_14,_19,_14,_14,_9,_9,_4];
_2 = _29.fld0 as u32;
_26 = _16 as usize;
_21.0 = _12 as i8;
_23 = _2;
_24.fld0 = _29.fld0;
RET.fld0 = _24.fld0;
_4 = _27 as isize;
RET.fld0 = !_24.fld0;
_1 = _18.0;
RET = _24;
_6 = _17;
_29 = Adt55 { fld0: _24.fld0,fld1: _24.fld1 };
_9 = _14 << _13;
_30.3 = _30.0;
RET = Adt55 { fld0: _29.fld0,fld1: _29.fld1 };
_26 = _12;
RET.fld0 = _24.fld0;
_11 = [_30.2,_23,_30.2,_30.2,_13];
_24.fld0 = !_29.fld0;
Goto(bb10)
}
bb10 = {
_18.0 = _17;
RET.fld1 = [_27,_27];
_21 = (_5,);
_11 = [_23,_23,_23,_23,_23];
_30 = (_15, _26, _23, _15);
_29.fld0 = _24.fld0;
_21 = (_5,);
_24.fld0 = _29.fld0;
_8 = _5 as i16;
_30.1 = !_26;
_21.0 = 1767921779_i32 as i8;
_18.1 = _18.2;
RET.fld1 = [_27,_27];
_18.2 = !_8;
Goto(bb11)
}
bb11 = {
_10 = core::ptr::addr_of!((*_3));
_3 = core::ptr::addr_of!((*_3));
RET.fld1 = [_27,_27];
_35 = core::ptr::addr_of!(_7.0);
_29.fld0 = 1122552127634984762_u64 as u16;
_18.2 = !_8;
RET = _24;
_8 = 10301334227158664976_u64 as i16;
_18.0 = _17;
_25 = [_1,_6,_6,_6];
_16 = _2 == _2;
_19 = _9;
_33 = !_9;
_13 = _2 - _30.2;
_37 = _5 as f32;
_38.1 = _33 as i16;
_30 = (_15, _12, _23, _15);
_14 = -_20;
_30.2 = 9531280310165981453_u64 as u32;
_15 = [_4,_33,_19,_9,_20,_19,_4,_33];
_24 = Adt55 { fld0: _29.fld0,fld1: _29.fld1 };
_33 = _9;
_21.0 = _5 << _9;
Goto(bb12)
}
bb12 = {
_19 = _12 as isize;
_38.0 = _6;
_35 = core::ptr::addr_of!((*_35));
_20 = _9 & _9;
_9 = _20;
_21.0 = !_5;
_38 = _18;
_41 = _16;
_30.1 = _12;
_27 = 215_u8 & 76_u8;
_28 = 337739487459778033040774261063017930747_u128 as f32;
_14 = _20 - _9;
_38.0 = _6;
_38.2 = _9 as i16;
_41 = _16;
_18.2 = _8 - _38.2;
Goto(bb13)
}
bb13 = {
_11 = [_13,_23,_23,_13,_2];
_18 = _38;
_28 = -_37;
_12 = 276322229748853861856557318909994382581_u128 as usize;
_32 = [_27,_27];
_35 = core::ptr::addr_of!((*_35));
_42.0 = 12927277677787631883_u64 as i8;
_24.fld0 = _29.fld0 + _29.fld0;
_5 = _42.0;
_18.1 = _18.2 - _18.2;
_41 = !_16;
RET.fld0 = _5 as u16;
_10 = core::ptr::addr_of!((*_10));
_34 = _38.2 + _38.2;
_2 = _13;
_32 = [_27,_27];
_8 = _16 as i16;
_10 = core::ptr::addr_of!((*_10));
_36 = [_2,_13,_2,_2,_2];
_44 = _37 as u64;
_17 = _1;
_14 = -_9;
_35 = core::ptr::addr_of!(_7.0);
Goto(bb14)
}
bb14 = {
RET.fld1 = [_27,_27];
_38 = (_1, _8, _34);
_50.fld0.fld0.fld1.2 = core::ptr::addr_of!(_39);
_47 = _30;
_29 = Adt55 { fld0: _24.fld0,fld1: _32 };
_25 = [_6,_17,_17,_38.0];
_47.1 = _2 as usize;
_50.fld0.fld0.fld1.0 = core::ptr::addr_of!((*_3));
_45 = [_2,_2,_23,_13,_13];
_25 = [_1,_6,_18.0,_6];
_43 = [_19,_20,_20,_9];
_50.fld1 = _44 * _44;
_46 = _25;
_10 = _50.fld0.fld0.fld1.0;
RET.fld1 = [_27,_27];
_47.1 = !_26;
_50.fld0.fld3 = _21.0;
(*_3) = core::ptr::addr_of!(_44);
_16 = _41;
_50.fld0.fld0.fld3.1 = _12 << _13;
_22 = !_16;
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(5_usize, 23_usize, Move(_23), 34_usize, Move(_34), 21_usize, Move(_21), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(5_usize, 13_usize, Move(_13), 38_usize, Move(_38), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(5_usize, 27_usize, Move(_27), 6_usize, Move(_6), 15_usize, Move(_15), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(5_usize, 46_usize, Move(_46), 8_usize, Move(_8), 36_usize, Move(_36), 43_usize, Move(_43)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(5_usize, 30_usize, Move(_30), 47_usize, Move(_47), 54_usize, _54, 54_usize, _54), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *const u64,mut _2: *const u64,mut _3: [u8; 2],mut _4: *mut i32,mut _5: i16,mut _6: *const u64,mut _7: *const u64,mut _8: [u8; 2],mut _9: *const u64,mut _10: (char, i16, i16),mut _11: *const *const u64) -> u32 {
mir! {
type RET = u32;
let _12: (*mut i32,);
let _13: [usize; 5];
let _14: [usize; 5];
let _15: isize;
let _16: isize;
let _17: Adt55;
let _18: bool;
let _19: i128;
let _20: (char, i16, i16);
let _21: *mut [isize; 8];
let _22: i128;
let _23: Adt59;
let _24: Adt65;
let _25: Adt52;
let _26: (u32, [usize; 5], u32, i32);
let _27: *mut [usize; 5];
let _28: ([isize; 8], usize, u32, [isize; 8]);
let _29: [char; 4];
let _30: *mut [usize; 5];
let _31: u128;
let _32: i16;
let _33: *const *const [u32; 2];
let _34: ();
let _35: ();
{
RET = 442058700_u32;
(*_1) = 9223372036854775807_isize as u64;
(*_2) = 148_u8 as u64;
Call((*_11) = fn7((*_7), _9, (*_2), (*_7), (*_1), (*_4), _4, _10.0, _10.0, _2, (*_2), _6, (*_9)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10.0 = '\u{7eab9}';
_12 = (_4,);
_8 = [194_u8,187_u8];
RET = 146_u8 as u32;
(*_9) = _5 as u64;
_1 = core::ptr::addr_of!((*_7));
_3 = _8;
(*_6) = !10565073155339258474_u64;
_15 = (-24_isize) + (-9223372036854775808_isize);
(*_6) = 8963363911794712007_u64;
(*_6) = !17538635794272772498_u64;
Call((*_6) = core::intrinsics::bswap(8342963735419645119_u64), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = core::ptr::addr_of!((*_2));
_1 = core::ptr::addr_of!((*_6));
_17.fld1 = [81_u8,195_u8];
(*_6) = 29239_u16 as u64;
(*_9) = 18190218073593177221_u64;
_8 = [73_u8,104_u8];
_14 = [16467960487433992210_usize,2_usize,3373829585898222497_usize,7_usize,4261181776048479645_usize];
match (*_9) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
18190218073593177221 => bb10,
_ => bb9
}
}
bb3 = {
_10.0 = '\u{7eab9}';
_12 = (_4,);
_8 = [194_u8,187_u8];
RET = 146_u8 as u32;
(*_9) = _5 as u64;
_1 = core::ptr::addr_of!((*_7));
_3 = _8;
(*_6) = !10565073155339258474_u64;
_15 = (-24_isize) + (-9223372036854775808_isize);
(*_6) = 8963363911794712007_u64;
(*_6) = !17538635794272772498_u64;
Call((*_6) = core::intrinsics::bswap(8342963735419645119_u64), ReturnTo(bb2), UnwindUnreachable())
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
_5 = _10.1;
_13 = [1853076131721441220_usize,15862261259636247692_usize,11633776553551754314_usize,9640636602163041747_usize,16971407691006132946_usize];
_17.fld0 = !40260_u16;
_6 = core::ptr::addr_of!((*_1));
_16 = _15;
_20.1 = 11002527204170736867366573699319692381_u128 as i16;
_20.1 = !_5;
_22 = (-135495747673414924536929653387068854931_i128) - 39018101709904533942800593952675901106_i128;
_17.fld0 = 30039_u16 - 11865_u16;
_18 = !true;
(*_6) = _15 as u64;
Goto(bb11)
}
bb11 = {
(*_2) = _17.fld0 as u64;
_17 = Adt55 { fld0: 39876_u16,fld1: _8 };
(*_4) = 158390484_i32;
_21 = core::ptr::addr_of_mut!(_24.fld4.fld4.fld3.3);
_24.fld4.fld0 = [1043085747_u32,1862119759_u32];
_24.fld4.fld1.2.0.0 = _10.0;
(*_1) = _20.1 as u64;
_24.fld3.fld0 = 2216216527_u32 as isize;
_24.fld2.fld0.fld3 = (-14_i8);
_24.fld2.fld0.fld0.fld1.4 = !_18;
_24.fld2.fld0.fld0.fld1.3 = _24.fld3.fld0;
_24.fld4.fld0 = [1610472589_u32,1548145955_u32];
_24.fld1.fld0 = core::ptr::addr_of_mut!(_24.fld4.fld3.fld1);
(*_1) = 13314220888231607606_u64 >> _16;
_24.fld1.fld0 = core::ptr::addr_of_mut!(_24.fld2.fld0.fld3);
_3 = [89_u8,212_u8];
(*_21) = [_24.fld3.fld0,_24.fld2.fld0.fld0.fld1.3,_15,_16,_15,_15,_24.fld2.fld0.fld0.fld1.3,_16];
_24.fld4.fld4.fld3.1 = 15523045955365808819_usize;
(*_9) = 7428453643496036546_u64;
_24.fld4.fld4.fld2 = 8277544123800462199_i64;
_24.fld4.fld1.2.2.0 = core::ptr::addr_of_mut!((*_4));
_25.fld5 = _21;
_20 = (_24.fld4.fld1.2.0.0, _10.2, _5);
_20.0 = _10.0;
Goto(bb12)
}
bb12 = {
_25.fld1 = (*_1) as f64;
_24.fld2.fld0.fld0.fld3.0 = [_24.fld3.fld0,_16,_16,_24.fld2.fld0.fld0.fld1.3,_24.fld2.fld0.fld0.fld1.3,_16,_16,_16];
_28.1 = !_24.fld4.fld4.fld3.1;
_17.fld1 = _3;
_25.fld7 = _24.fld4.fld4.fld3.1;
(*_6) = !11678184481468434390_u64;
_20.2 = _10.1;
_24.fld4.fld4.fld3.0 = [_15,_15,_15,_16,_24.fld2.fld0.fld0.fld1.3,_16,_24.fld2.fld0.fld0.fld1.3,_24.fld2.fld0.fld0.fld1.3];
_21 = core::ptr::addr_of_mut!(_24.fld4.fld6);
_24.fld4.fld4.fld3.0 = [_16,_15,_16,_15,_16,_15,_24.fld3.fld0,_24.fld2.fld0.fld0.fld1.3];
_25.fld4.2 = 3405834232_u32 / 1886896483_u32;
_24.fld2.fld0.fld0.fld1.4 = _18;
_24.fld4.fld1.2.0.2 = !_20.1;
RET = _25.fld4.2;
(*_21) = _24.fld2.fld0.fld0.fld3.0;
(*_21) = [_16,_15,_15,_15,_15,_15,_24.fld3.fld0,_24.fld3.fld0];
_24.fld2.fld0.fld3 = 0_i8 ^ 109_i8;
match _24.fld4.fld4.fld2 {
0 => bb7,
1 => bb2,
2 => bb9,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
8277544123800462199 => bb18,
_ => bb17
}
}
bb13 = {
(*_2) = _17.fld0 as u64;
_17 = Adt55 { fld0: 39876_u16,fld1: _8 };
(*_4) = 158390484_i32;
_21 = core::ptr::addr_of_mut!(_24.fld4.fld4.fld3.3);
_24.fld4.fld0 = [1043085747_u32,1862119759_u32];
_24.fld4.fld1.2.0.0 = _10.0;
(*_1) = _20.1 as u64;
_24.fld3.fld0 = 2216216527_u32 as isize;
_24.fld2.fld0.fld3 = (-14_i8);
_24.fld2.fld0.fld0.fld1.4 = !_18;
_24.fld2.fld0.fld0.fld1.3 = _24.fld3.fld0;
_24.fld4.fld0 = [1610472589_u32,1548145955_u32];
_24.fld1.fld0 = core::ptr::addr_of_mut!(_24.fld4.fld3.fld1);
(*_1) = 13314220888231607606_u64 >> _16;
_24.fld1.fld0 = core::ptr::addr_of_mut!(_24.fld2.fld0.fld3);
_3 = [89_u8,212_u8];
(*_21) = [_24.fld3.fld0,_24.fld2.fld0.fld0.fld1.3,_15,_16,_15,_15,_24.fld2.fld0.fld0.fld1.3,_16];
_24.fld4.fld4.fld3.1 = 15523045955365808819_usize;
(*_9) = 7428453643496036546_u64;
_24.fld4.fld4.fld2 = 8277544123800462199_i64;
_24.fld4.fld1.2.2.0 = core::ptr::addr_of_mut!((*_4));
_25.fld5 = _21;
_20 = (_24.fld4.fld1.2.0.0, _10.2, _5);
_20.0 = _10.0;
Goto(bb12)
}
bb14 = {
_10.0 = '\u{7eab9}';
_12 = (_4,);
_8 = [194_u8,187_u8];
RET = 146_u8 as u32;
(*_9) = _5 as u64;
_1 = core::ptr::addr_of!((*_7));
_3 = _8;
(*_6) = !10565073155339258474_u64;
_15 = (-24_isize) + (-9223372036854775808_isize);
(*_6) = 8963363911794712007_u64;
(*_6) = !17538635794272772498_u64;
Call((*_6) = core::intrinsics::bswap(8342963735419645119_u64), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_6 = core::ptr::addr_of!((*_2));
_1 = core::ptr::addr_of!((*_6));
_17.fld1 = [81_u8,195_u8];
(*_6) = 29239_u16 as u64;
(*_9) = 18190218073593177221_u64;
_8 = [73_u8,104_u8];
_14 = [16467960487433992210_usize,2_usize,3373829585898222497_usize,7_usize,4261181776048479645_usize];
match (*_9) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
18190218073593177221 => bb10,
_ => bb9
}
}
bb18 = {
_26.2 = _25.fld4.2;
_6 = (*_11);
_24.fld4.fld1.1 = _25.fld7 / 1_usize;
_24.fld4.fld4.fld1.3 = _16;
_24.fld4.fld4.fld3.2 = 280014461119242132810896711232457475733_u128 as u32;
_25.fld0.4 = 177674015942204684663580886844766028551_u128 as f32;
_24.fld4.fld4.fld1.4 = _18;
_29 = [_20.0,_10.0,_10.0,_24.fld4.fld1.2.0.0];
_10 = (_20.0, _5, _20.2);
_17.fld0 = 23001_u16 / 28108_u16;
_24.fld4.fld3.fld2 = core::ptr::addr_of!((*_9));
(*_7) = _25.fld4.2 as u64;
_24.fld2.fld0.fld2.4 = _25.fld0.4;
_20.0 = _10.0;
_24.fld4.fld1.6 = _24.fld2.fld0.fld3 as u16;
_25.fld0 = (_24.fld2.fld0.fld2.4, _20.0, 34_u8, _25.fld1, _24.fld2.fld0.fld2.4);
_24.fld4.fld4.fld1.1 = _15 * _24.fld2.fld0.fld0.fld1.3;
_25.fld4.3 = (*_4) | (*_4);
_24.fld4.fld3.fld1 = !_24.fld2.fld0.fld3;
_24.fld2.fld0.fld2.3 = -_25.fld0.3;
Goto(bb19)
}
bb19 = {
Call(_34 = dump_var(6_usize, 20_usize, Move(_20), 8_usize, Move(_8), 3_usize, Move(_3), 18_usize, Move(_18)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(6_usize, 29_usize, Move(_29), 15_usize, Move(_15), 35_usize, _35, 35_usize, _35), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u64,mut _2: *const u64,mut _3: u64,mut _4: u64,mut _5: u64,mut _6: i32,mut _7: *mut i32,mut _8: char,mut _9: char,mut _10: *const u64,mut _11: u64,mut _12: *const u64,mut _13: u64) -> *const u64 {
mir! {
type RET = *const u64;
let _14: u32;
let _15: isize;
let _16: f32;
let _17: i16;
let _18: ([isize; 8], usize, u32, [isize; 8]);
let _19: [u32; 5];
let _20: f32;
let _21: f32;
let _22: [u32; 5];
let _23: i64;
let _24: (u32, [usize; 5], u32, i32);
let _25: (char, i16, i16);
let _26: char;
let _27: Adt49;
let _28: Adt62;
let _29: isize;
let _30: isize;
let _31: Adt61;
let _32: [i64; 5];
let _33: u64;
let _34: (char, i16, i16);
let _35: f64;
let _36: [usize; 5];
let _37: ([isize; 8], usize, u32, [isize; 8]);
let _38: f64;
let _39: isize;
let _40: isize;
let _41: u128;
let _42: [isize; 8];
let _43: ([isize; 8], usize, u32, [isize; 8]);
let _44: ();
let _45: ();
{
(*_2) = _3;
(*_10) = _11 << _11;
_9 = _8;
_7 = core::ptr::addr_of_mut!((*_7));
(*_12) = 54_u8 as u64;
_13 = (*_2);
_8 = _9;
RET = core::ptr::addr_of!(_1);
Goto(bb1)
}
bb1 = {
RET = _10;
_13 = (-9223372036854775808_isize) as u64;
(*_10) = _5;
(*_7) = (-2668932903870470397_i64) as i32;
Goto(bb2)
}
bb2 = {
_12 = _10;
(*RET) = _11 / 231202737693434082_u64;
_5 = _1 * (*_2);
(*RET) = (-9223372036854775808_isize) as u64;
(*RET) = !_13;
(*RET) = _1;
(*RET) = !_1;
_6 = (-3697180194486750791871442957474349064_i128) as i32;
(*_7) = !_6;
(*_7) = 39_i8 as i32;
_9 = _8;
_2 = _10;
(*_2) = _13;
_7 = core::ptr::addr_of_mut!((*_7));
(*RET) = _5 * _3;
Goto(bb3)
}
bb3 = {
_4 = 9196483946786193511_i64 as u64;
_4 = !(*_12);
_5 = !(*_12);
_16 = 6422676895448894474_i64 as f32;
(*RET) = _5;
_6 = -(*_7);
_12 = core::ptr::addr_of!(_3);
_18.3 = [(-94_isize),9223372036854775807_isize,(-37_isize),(-9223372036854775808_isize),29_isize,9223372036854775807_isize,(-109_isize),9223372036854775807_isize];
_12 = core::ptr::addr_of!((*_2));
_18.1 = _16 as usize;
_4 = 1072849777_u32 as u64;
(*_2) = _11 * _5;
_16 = _5 as f32;
_19 = [220802487_u32,152392458_u32,1317778953_u32,3631670006_u32,3564105801_u32];
_20 = _16;
RET = core::ptr::addr_of!(_1);
(*_7) = 18391_i16 as i32;
_2 = _12;
Goto(bb4)
}
bb4 = {
(*_10) = (*RET);
_12 = _2;
_6 = (*_7);
_12 = _10;
_22 = _19;
(*RET) = 55289_u16 as u64;
_21 = _20;
_11 = !(*RET);
_4 = !_5;
_14 = 3392152385_u32 & 1391176679_u32;
_15 = !(-62_isize);
_23 = -(-6471294206819509492_i64);
_5 = _23 as u64;
_21 = (*_7) as f32;
_2 = _12;
_24.3 = (*_7);
_24.3 = (*_7);
(*RET) = _4;
_5 = (*RET);
_1 = _9 as u64;
_18.2 = _14;
_14 = _18.2 << _5;
_13 = _11 * _3;
_17 = 15142_i16 << _13;
RET = core::ptr::addr_of!(_5);
_4 = !_5;
Goto(bb5)
}
bb5 = {
(*_2) = !_4;
_19 = _22;
_27.fld6 = core::ptr::addr_of!((*_10));
_24.3 = 63149_u16 as i32;
_27.fld1 = 153_u8 as f64;
_27.fld5 = (*_7) & _24.3;
_24.0 = _27.fld1 as u32;
_25.0 = _9;
(*_7) = _6;
(*_2) = !_4;
_28.fld4.0.0 = _8;
_28.fld5.fld2 = (_20, _25.0, 151_u8, _27.fld1, _20);
Goto(bb6)
}
bb6 = {
RET = _2;
_24.0 = _14;
_28.fld5.fld2 = (_20, _8, 91_u8, _27.fld1, _21);
(*RET) = _24.0 as u64;
_28.fld5.fld0.fld1.1 = _21 as isize;
_27.fld2 = (_19,);
_28.fld3 = core::ptr::addr_of!(_28.fld4.2.0);
_10 = core::ptr::addr_of!(_3);
_28.fld5.fld2.3 = _27.fld1;
_28.fld2 = _27.fld1 as isize;
_21 = _20;
_27.fld2.0 = [_18.2,_24.0,_18.2,_14,_14];
_24.1 = [_18.1,_18.1,_18.1,_18.1,_18.1];
_28.fld5.fld0.fld3.1 = _18.1 % 1_usize;
_28.fld5.fld0.fld2 = _23;
Call(_1 = core::intrinsics::transmute((*_12)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_2) = !_1;
_24.2 = _28.fld2 as u32;
_28.fld5.fld5 = !_24.0;
_25 = (_28.fld4.0.0, _17, _17);
_1 = (*_12) << _28.fld5.fld5;
_31.fld2.0.0 = core::ptr::addr_of!(_10);
_15 = _28.fld5.fld0.fld1.1;
_28.fld5.fld0.fld3 = (_18.3, _18.1, _24.0, _18.3);
_31.fld0 = _24.0;
_28.fld4.0.2 = (*_7) as i16;
_18.1 = _28.fld5.fld0.fld3.1;
(*_12) = _28.fld5.fld2.3 as u64;
_27.fld2.0 = [_31.fld0,_28.fld5.fld0.fld3.2,_28.fld5.fld0.fld3.2,_28.fld5.fld5,_28.fld5.fld5];
_14 = !_31.fld0;
_28.fld5.fld0.fld3.3 = [_15,_15,_28.fld5.fld0.fld1.1,_15,_28.fld2,_15,_15,_28.fld5.fld0.fld1.1];
_28.fld5.fld0.fld0 = [_28.fld5.fld2.2,_28.fld5.fld2.2];
_20 = _27.fld5 as f32;
_28.fld5.fld1 = core::ptr::addr_of!(_12);
_27.fld3 = _20 as i8;
(*RET) = _1;
Goto(bb8)
}
bb8 = {
_14 = _31.fld0 >> (*RET);
_28.fld5.fld0.fld1.4 = _31.fld0 < _24.0;
_28.fld5.fld2 = (_16, _25.0, 213_u8, _27.fld1, _16);
_20 = _16 * _16;
(*_12) = _1;
_28.fld5.fld0.fld1.0 = core::ptr::addr_of!(_2);
_28.fld5.fld5 = _24.0;
_34 = (_25.0, _17, _25.1);
_33 = (*_2) - (*_2);
_28.fld4.0 = (_34.0, _25.1, _34.1);
_25.0 = _8;
(*_10) = (*_12);
_18.0 = [_28.fld5.fld0.fld1.1,_15,_28.fld5.fld0.fld1.1,_28.fld2,_28.fld5.fld0.fld1.1,_28.fld2,_28.fld5.fld0.fld1.1,_28.fld5.fld0.fld1.1];
_28.fld5.fld0.fld1.4 = true;
_28.fld5.fld0.fld1.4 = !false;
_28.fld5.fld0.fld3.3 = [_28.fld5.fld0.fld1.1,_15,_28.fld5.fld0.fld1.1,_28.fld2,_28.fld5.fld0.fld1.1,_28.fld5.fld0.fld1.1,_15,_15];
_31.fld2.1 = core::ptr::addr_of!(_28.fld5.fld0.fld1.2);
_18.3 = [_15,_28.fld5.fld0.fld1.1,_28.fld5.fld0.fld1.1,_28.fld2,_28.fld5.fld0.fld1.1,_15,_28.fld5.fld0.fld1.1,_28.fld5.fld0.fld1.1];
_27.fld6 = core::ptr::addr_of!(_4);
Goto(bb9)
}
bb9 = {
_32 = [_28.fld5.fld0.fld2,_28.fld5.fld0.fld2,_23,_23,_23];
_2 = core::ptr::addr_of!(_1);
_31.fld3 = core::ptr::addr_of!(_31.fld2.0.2);
_6 = (*_7) >> _1;
_35 = -_28.fld5.fld2.3;
_34 = (_25.0, _28.fld4.0.2, _28.fld4.0.2);
_27.fld0 = _28.fld5.fld0.fld1.1 as f32;
_22 = _19;
_24.1 = [_28.fld5.fld0.fld3.1,_18.1,_28.fld5.fld0.fld3.1,_28.fld5.fld0.fld3.1,_18.1];
(*_2) = _28.fld5.fld0.fld1.4 as u64;
_28.fld5.fld0.fld1.3 = _28.fld2 << (*_12);
_34 = _25;
(*_7) = !_6;
(*_10) = _33;
(*_2) = (*_12);
_18.1 = _28.fld5.fld0.fld3.1;
(*RET) = 246578280854579685851108594072950949540_u128 as u64;
_15 = (*_10) as isize;
(*RET) = _33 / 16536349692935316775_u64;
_31.fld2.0.0 = core::ptr::addr_of!(_12);
_29 = _6 as isize;
_3 = _33;
_29 = !_15;
_28.fld5.fld2 = (_27.fld0, _25.0, 128_u8, _27.fld1, _16);
_18.0 = [_15,_29,_15,_29,_15,_29,_29,_15];
_28.fld5.fld0.fld2 = _23;
Goto(bb10)
}
bb10 = {
(*_12) = _3 - (*_2);
_21 = _3 as f32;
_27.fld2.0 = [_14,_28.fld5.fld0.fld3.2,_14,_31.fld0,_14];
_8 = _28.fld5.fld2.1;
_28.fld1 = core::ptr::addr_of_mut!(_18.0);
_28.fld5.fld1 = core::ptr::addr_of!(_12);
_33 = 44748_u16 as u64;
_31.fld4 = core::ptr::addr_of_mut!(_36);
_28.fld5.fld0.fld2 = _29 as i64;
_28.fld0 = [_28.fld5.fld2.1,_25.0,_9,_34.0];
_28.fld5.fld0.fld3.2 = _28.fld5.fld0.fld3.1 as u32;
_27.fld3 = _28.fld5.fld2.2 as i8;
_31.fld2.0.3 = _15;
_14 = !_24.0;
Goto(bb11)
}
bb11 = {
_40 = !_31.fld2.0.3;
_17 = _18.1 as i16;
(*_12) = !_3;
_24.3 = _28.fld5.fld2.1 as i32;
_17 = -_34.2;
Goto(bb12)
}
bb12 = {
_9 = _25.0;
_25.1 = _34.2;
_43.1 = _27.fld3 as usize;
match _28.fld5.fld2.2 {
0 => bb5,
1 => bb13,
2 => bb14,
3 => bb15,
128 => bb17,
_ => bb16
}
}
bb13 = {
(*_2) = !_4;
_19 = _22;
_27.fld6 = core::ptr::addr_of!((*_10));
_24.3 = 63149_u16 as i32;
_27.fld1 = 153_u8 as f64;
_27.fld5 = (*_7) & _24.3;
_24.0 = _27.fld1 as u32;
_25.0 = _9;
(*_7) = _6;
(*_2) = !_4;
_28.fld4.0.0 = _8;
_28.fld5.fld2 = (_20, _25.0, 151_u8, _27.fld1, _20);
Goto(bb6)
}
bb14 = {
_4 = 9196483946786193511_i64 as u64;
_4 = !(*_12);
_5 = !(*_12);
_16 = 6422676895448894474_i64 as f32;
(*RET) = _5;
_6 = -(*_7);
_12 = core::ptr::addr_of!(_3);
_18.3 = [(-94_isize),9223372036854775807_isize,(-37_isize),(-9223372036854775808_isize),29_isize,9223372036854775807_isize,(-109_isize),9223372036854775807_isize];
_12 = core::ptr::addr_of!((*_2));
_18.1 = _16 as usize;
_4 = 1072849777_u32 as u64;
(*_2) = _11 * _5;
_16 = _5 as f32;
_19 = [220802487_u32,152392458_u32,1317778953_u32,3631670006_u32,3564105801_u32];
_20 = _16;
RET = core::ptr::addr_of!(_1);
(*_7) = 18391_i16 as i32;
_2 = _12;
Goto(bb4)
}
bb15 = {
(*_2) = !_1;
_24.2 = _28.fld2 as u32;
_28.fld5.fld5 = !_24.0;
_25 = (_28.fld4.0.0, _17, _17);
_1 = (*_12) << _28.fld5.fld5;
_31.fld2.0.0 = core::ptr::addr_of!(_10);
_15 = _28.fld5.fld0.fld1.1;
_28.fld5.fld0.fld3 = (_18.3, _18.1, _24.0, _18.3);
_31.fld0 = _24.0;
_28.fld4.0.2 = (*_7) as i16;
_18.1 = _28.fld5.fld0.fld3.1;
(*_12) = _28.fld5.fld2.3 as u64;
_27.fld2.0 = [_31.fld0,_28.fld5.fld0.fld3.2,_28.fld5.fld0.fld3.2,_28.fld5.fld5,_28.fld5.fld5];
_14 = !_31.fld0;
_28.fld5.fld0.fld3.3 = [_15,_15,_28.fld5.fld0.fld1.1,_15,_28.fld2,_15,_15,_28.fld5.fld0.fld1.1];
_28.fld5.fld0.fld0 = [_28.fld5.fld2.2,_28.fld5.fld2.2];
_20 = _27.fld5 as f32;
_28.fld5.fld1 = core::ptr::addr_of!(_12);
_27.fld3 = _20 as i8;
(*RET) = _1;
Goto(bb8)
}
bb16 = {
(*_10) = (*RET);
_12 = _2;
_6 = (*_7);
_12 = _10;
_22 = _19;
(*RET) = 55289_u16 as u64;
_21 = _20;
_11 = !(*RET);
_4 = !_5;
_14 = 3392152385_u32 & 1391176679_u32;
_15 = !(-62_isize);
_23 = -(-6471294206819509492_i64);
_5 = _23 as u64;
_21 = (*_7) as f32;
_2 = _12;
_24.3 = (*_7);
_24.3 = (*_7);
(*RET) = _4;
_5 = (*RET);
_1 = _9 as u64;
_18.2 = _14;
_14 = _18.2 << _5;
_13 = _11 * _3;
_17 = 15142_i16 << _13;
RET = core::ptr::addr_of!(_5);
_4 = !_5;
Goto(bb5)
}
bb17 = {
(*_12) = _33 + (*_10);
_27.fld6 = _10;
_37.0 = _18.0;
_8 = _28.fld5.fld2.1;
Goto(bb18)
}
bb18 = {
Call(_44 = dump_var(7_usize, 23_usize, Move(_23), 17_usize, Move(_17), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(7_usize, 33_usize, Move(_33), 22_usize, Move(_22), 32_usize, Move(_32), 40_usize, Move(_40)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(7_usize, 3_usize, Move(_3), 19_usize, Move(_19), 25_usize, Move(_25), 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: *mut i8,mut _2: u32,mut _3: u16,mut _4: Adt55,mut _5: isize,mut _6: f32,mut _7: usize,mut _8: [isize; 8],mut _9: isize,mut _10: u32) -> *const u16 {
mir! {
type RET = *const u16;
let _11: char;
let _12: Adt63;
let _13: u16;
let _14: [isize; 4];
let _15: bool;
let _16: isize;
let _17: (f32, *mut i32, u128, (char, i16, i16));
let _18: char;
let _19: [u8; 2];
let _20: [isize; 8];
let _21: (*mut i32,);
let _22: (u32, [usize; 5], u32, i32);
let _23: ([isize; 8], usize, u32, [isize; 8]);
let _24: ((char, i16, i16), [usize; 5], (*mut i32,));
let _25: i64;
let _26: *mut i32;
let _27: ();
let _28: ();
{
_7 = !1_usize;
RET = core::ptr::addr_of!(_4.fld0);
_5 = 9915359221669768327_u64 as isize;
(*RET) = true as u16;
_10 = (-7445990754152275142_i64) as u32;
_7 = 1_usize;
(*_1) = -119_i8;
_4.fld1 = [88_u8,214_u8];
_12.fld0.fld0.fld3.3 = _8;
_12.fld0.fld2.2 = _4.fld1[_7] | _4.fld1[_7];
_12.fld0.fld0.fld3.3[_7] = -_8[_7];
_12.fld0.fld2.1 = '\u{1056df}';
_8[_7] = _12.fld0.fld0.fld3.3[_7] & _9;
_12.fld0.fld0.fld2 = -(-7447847913992776993_i64);
_12.fld0.fld0.fld0[_7] = !_4.fld1[_7];
_12.fld0.fld0.fld1.3 = _12.fld0.fld0.fld3.3[_7] | _8[_7];
(*RET) = !_3;
_5 = _8[_7];
_12.fld0.fld2.4 = -_6;
(*RET) = _3;
_12.fld0.fld0.fld1.4 = _5 < _5;
_5 = _7 as isize;
_12.fld0.fld2.0 = _12.fld0.fld2.4;
_11 = _12.fld0.fld2.1;
_13 = _4.fld0 - _3;
_12.fld0.fld0.fld3.0 = _8;
_12.fld0.fld2.2 = _4.fld1[_7] % 1_u8;
_12.fld0.fld2.2 = (-50167237486945937343244031515904556118_i128) as u8;
Goto(bb1)
}
bb1 = {
_14[_7] = -_12.fld0.fld0.fld1.3;
(*RET) = !_13;
_12.fld0.fld2.0 = _12.fld0.fld2.4 * _12.fld0.fld2.4;
_12.fld0.fld0.fld3.1 = _7;
_12.fld0.fld0.fld3.3[_7] = (*RET) as isize;
_12.fld0.fld0.fld1.3 = _12.fld0.fld0.fld3.0[_7];
match _9 {
0 => bb2,
1 => bb3,
340282366920938463454151235394913435648 => bb5,
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
_12.fld0.fld0.fld3.0 = [_12.fld0.fld0.fld1.3,_12.fld0.fld0.fld1.3,_14[_7],_12.fld0.fld0.fld1.3,_14[_7],_12.fld0.fld0.fld1.3,_14[_7],_14[_7]];
_11 = _12.fld0.fld2.1;
_14 = [_5,_8[_7],_8[_7],_9];
_9 = _8[_7] >> _14[_7];
_12.fld0.fld0.fld0 = _4.fld1;
RET = core::ptr::addr_of!(_13);
match _12.fld0.fld0.fld0[_7] {
0 => bb1,
1 => bb2,
2 => bb3,
214 => bb6,
_ => bb4
}
}
bb6 = {
_9 = -_12.fld0.fld0.fld1.3;
_4.fld0 = (*RET) & _13;
match _12.fld0.fld0.fld0[_7] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
214 => bb7,
_ => bb5
}
}
bb7 = {
(*RET) = _4.fld0;
(*_1) = -103_i8;
_12.fld0.fld2.3 = (*_1) as f64;
_12.fld0.fld0.fld3 = (_8, _7, _2, _8);
_17.3.0 = _11;
_17.0 = _12.fld0.fld2.0 / 1_f32;
_12.fld0.fld0.fld3.0 = [_8[_7],_9,_14[_7],_12.fld0.fld0.fld1.3,_14[_7],_14[_7],_14[_7],_9];
_12.fld0.fld0.fld1.1 = _12.fld0.fld0.fld3.3[_7] << _4.fld0;
_5 = _12.fld0.fld0.fld3.0[_7] ^ _12.fld0.fld0.fld3.0[_7];
_16 = _12.fld0.fld0.fld1.1;
RET = core::ptr::addr_of!(_4.fld0);
_17.2 = 257336656381303986738400218874336742223_u128;
_3 = 10792354322966364839_u64 as u16;
_12.fld0.fld0.fld3 = (_8, _7, _10, _8);
_13 = _12.fld0.fld0.fld1.1 as u16;
Call(_17.3.2 = fn9(_13, _9, _12.fld0.fld0.fld3.3[_7], _4.fld1[_7], _12.fld0.fld2, _7, _8[_7], _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_1) = _12.fld0.fld0.fld2 as i8;
_12.fld0.fld5 = _12.fld0.fld0.fld2 as u32;
_12.fld0.fld2.2 = !226_u8;
_6 = _12.fld0.fld2.0;
_12.fld0.fld0.fld0 = [_12.fld0.fld2.2,_12.fld0.fld2.2];
_17.3.1 = _17.3.2;
_15 = !_12.fld0.fld0.fld1.4;
_12.fld0.fld3 = (*_1) & (*_1);
_17.3.0 = _11;
_10 = _2;
_12.fld0.fld2.4 = -_17.0;
_17.3.1 = _17.3.2;
_22.1 = [_12.fld0.fld0.fld3.1,_12.fld0.fld0.fld3.1,_12.fld0.fld0.fld3.1,_7,_12.fld0.fld0.fld3.1];
Call(_7 = core::intrinsics::transmute(_12.fld0.fld0.fld2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14 = [_12.fld0.fld0.fld1.1,_16,_12.fld0.fld0.fld1.1,_16];
(*_1) = -_12.fld0.fld3;
_9 = 11846041883764741069_u64 as isize;
_12.fld0.fld0.fld3 = (_8, _7, _2, _8);
_1 = core::ptr::addr_of_mut!(_12.fld0.fld3);
_4.fld0 = _13;
_8 = _12.fld0.fld0.fld3.3;
(*_1) = 2_i8 | (-4_i8);
_22.2 = _10 ^ _10;
_22.0 = !_22.2;
_23 = _12.fld0.fld0.fld3;
match _17.2 {
0 => bb7,
1 => bb6,
2 => bb8,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
257336656381303986738400218874336742223 => bb15,
_ => bb14
}
}
bb10 = {
_14[_7] = -_12.fld0.fld0.fld1.3;
(*RET) = !_13;
_12.fld0.fld2.0 = _12.fld0.fld2.4 * _12.fld0.fld2.4;
_12.fld0.fld0.fld3.1 = _7;
_12.fld0.fld0.fld3.3[_7] = (*RET) as isize;
_12.fld0.fld0.fld1.3 = _12.fld0.fld0.fld3.0[_7];
match _9 {
0 => bb2,
1 => bb3,
340282366920938463454151235394913435648 => bb5,
_ => bb4
}
}
bb11 = {
(*RET) = _4.fld0;
(*_1) = -103_i8;
_12.fld0.fld2.3 = (*_1) as f64;
_12.fld0.fld0.fld3 = (_8, _7, _2, _8);
_17.3.0 = _11;
_17.0 = _12.fld0.fld2.0 / 1_f32;
_12.fld0.fld0.fld3.0 = [_8[_7],_9,_14[_7],_12.fld0.fld0.fld1.3,_14[_7],_14[_7],_14[_7],_9];
_12.fld0.fld0.fld1.1 = _12.fld0.fld0.fld3.3[_7] << _4.fld0;
_5 = _12.fld0.fld0.fld3.0[_7] ^ _12.fld0.fld0.fld3.0[_7];
_16 = _12.fld0.fld0.fld1.1;
RET = core::ptr::addr_of!(_4.fld0);
_17.2 = 257336656381303986738400218874336742223_u128;
_3 = 10792354322966364839_u64 as u16;
_12.fld0.fld0.fld3 = (_8, _7, _10, _8);
_13 = _12.fld0.fld0.fld1.1 as u16;
Call(_17.3.2 = fn9(_13, _9, _12.fld0.fld0.fld3.3[_7], _4.fld1[_7], _12.fld0.fld2, _7, _8[_7], _1), ReturnTo(bb8), UnwindUnreachable())
}
bb12 = {
_9 = -_12.fld0.fld0.fld1.3;
_4.fld0 = (*RET) & _13;
match _12.fld0.fld0.fld0[_7] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
214 => bb7,
_ => bb5
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_23.0 = _23.3;
_22.3 = 1785492156_i32;
_21.0 = core::ptr::addr_of_mut!(_22.3);
_14 = [_16,_16,_12.fld0.fld0.fld1.3,_12.fld0.fld0.fld1.1];
_12.fld0.fld0.fld0 = _4.fld1;
_12.fld0.fld0.fld1.3 = _5 + _12.fld0.fld0.fld1.1;
_12.fld0.fld0.fld1.4 = !_15;
_19 = [_12.fld0.fld2.2,_12.fld0.fld2.2];
_12.fld0.fld0.fld0 = [_12.fld0.fld2.2,_12.fld0.fld2.2];
_11 = _12.fld0.fld2.1;
_14 = [_5,_16,_16,_12.fld0.fld0.fld1.1];
_12.fld1 = 4605360930380237310_u64 * 11732332896819012003_u64;
_12.fld0.fld0.fld1.3 = !_5;
_12.fld0.fld2.3 = _22.3 as f64;
(*RET) = _13 + _13;
_17.3.1 = !_17.3.2;
_24.0.1 = _17.3.2;
(*_1) = 22934215912223056127685681818531713679_i128 as i8;
_12.fld0.fld2.3 = _17.2 as f64;
_26 = core::ptr::addr_of_mut!(_22.3);
(*_26) = 1608175097_i32 * 629042066_i32;
_25 = (*_26) as i64;
_5 = _16 | _9;
_19 = [_12.fld0.fld2.2,_12.fld0.fld2.2];
_10 = (-169551426611105943865175432730072179731_i128) as u32;
_24.2 = (_21.0,);
_24.0 = _17.3;
_12.fld0.fld0.fld1.4 = _15 ^ _15;
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(8_usize, 22_usize, Move(_22), 13_usize, Move(_13), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(8_usize, 10_usize, Move(_10), 3_usize, Move(_3), 23_usize, Move(_23), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u16,mut _2: isize,mut _3: isize,mut _4: u8,mut _5: (f32, char, u8, f64, f32),mut _6: usize,mut _7: isize,mut _8: *mut i8) -> i16 {
mir! {
type RET = i16;
let _9: char;
let _10: char;
let _11: u32;
let _12: ();
let _13: ();
{
(*_8) = -(-88_i8);
_1 = 12020_u16;
_4 = _5.2 / 105_u8;
_8 = core::ptr::addr_of_mut!((*_8));
Goto(bb1)
}
bb1 = {
(*_8) = _6 as i8;
(*_8) = 21_i8 >> _7;
_2 = _3 >> (*_8);
RET = -16655_i16;
_7 = _2 + _2;
_6 = !3061130207686473400_usize;
RET = 18043_i16;
_7 = _2 ^ _3;
RET = _2 as i16;
_1 = !33550_u16;
_2 = !_3;
_9 = _5.1;
_3 = _7;
_10 = _5.1;
_9 = _5.1;
_5.4 = _5.0;
_10 = _9;
_1 = !16149_u16;
(*_8) = 905108164_i32 as i8;
_5.0 = (-15039_i16) as f32;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(9_usize, 4_usize, Move(_4), 6_usize, Move(_6), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [usize; 5],mut _2: u8,mut _3: u128,mut _4: u16,mut _5: [usize; 5],mut _6: u128,mut _7: char,mut _8: [usize; 5],mut _9: u16,mut _10: u8,mut _11: isize) -> i32 {
mir! {
type RET = i32;
let _12: usize;
let _13: i64;
let _14: isize;
let _15: char;
let _16: isize;
let _17: *mut i32;
let _18: f64;
let _19: Adt53;
let _20: [i64; 5];
let _21: ([u32; 5],);
let _22: [u8; 2];
let _23: i16;
let _24: ([isize; 8], usize, u32, [isize; 8]);
let _25: ();
let _26: ();
{
_9 = _7 as u16;
_2 = _10;
_3 = _6;
RET = 1180956245_i32 >> _3;
RET = 1302812445_i32;
_13 = 8384546526171475284_i64;
_3 = _6 >> _10;
_5 = [10171211726130055586_usize,3_usize,9170232429969688601_usize,11199028609215329720_usize,18410279315006836198_usize];
_10 = _2 - _2;
_10 = (-1453962650_i32) as u8;
_2 = _10 ^ _10;
_14 = -_11;
_9 = _4;
_8 = _5;
Call(_2 = fn11(_13, _8, _13, _1, _11, _8, _4, _7, _8, _6, _14, _8, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = _7;
_13 = (-646704160237922530_i64);
_9 = _4 << _6;
_14 = (-13577_i16) as isize;
_7 = _15;
_3 = _6 % 174045852130961309233580342609497471838_u128;
_16 = _14 + _11;
_5 = [4_usize,6_usize,15567615961268079745_usize,6_usize,16392244206305774149_usize];
_6 = _2 as u128;
_9 = !_4;
_6 = (-27192_i16) as u128;
_18 = _13 as f64;
RET = (-27876_i16) as i32;
_18 = _3 as f64;
_18 = (-93_i8) as f64;
_5 = [276432569227973242_usize,0_usize,7_usize,15957507447694864913_usize,1_usize];
_2 = !_10;
_6 = _3 / 127615660140420942872849263358880513318_u128;
_10 = _2 | _2;
_13 = (-7130470405573962206_i64);
_6 = _3;
_7 = _15;
_3 = _4 as u128;
RET = 826003573_i32 << _10;
_1 = [8721063455186636364_usize,5_usize,0_usize,13641183312568331251_usize,7994373504316847005_usize];
_14 = 13356326050925757353_usize as isize;
match _13 {
0 => bb2,
1 => bb3,
340282366920938463456244137026194249250 => bb5,
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
_12 = 7_usize;
_5 = [_12,_12,_12,_12,_12];
_11 = _16;
_19.fld5 = !749136161_u32;
_19.fld4.fld1.0 = core::ptr::addr_of!(_19.fld3.fld2);
_19.fld6[_12] = _11 << _10;
_19.fld1.3 = [_13,_13,_13,_13,_13];
_10 = !_2;
_14 = _16;
match _13 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463456244137026194249250 => bb10,
_ => bb9
}
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
_15 = _7;
_13 = (-646704160237922530_i64);
_9 = _4 << _6;
_14 = (-13577_i16) as isize;
_7 = _15;
_3 = _6 % 174045852130961309233580342609497471838_u128;
_16 = _14 + _11;
_5 = [4_usize,6_usize,15567615961268079745_usize,6_usize,16392244206305774149_usize];
_6 = _2 as u128;
_9 = !_4;
_6 = (-27192_i16) as u128;
_18 = _13 as f64;
RET = (-27876_i16) as i32;
_18 = _3 as f64;
_18 = (-93_i8) as f64;
_5 = [276432569227973242_usize,0_usize,7_usize,15957507447694864913_usize,1_usize];
_2 = !_10;
_6 = _3 / 127615660140420942872849263358880513318_u128;
_10 = _2 | _2;
_13 = (-7130470405573962206_i64);
_6 = _3;
_7 = _15;
_3 = _4 as u128;
RET = 826003573_i32 << _10;
_1 = [8721063455186636364_usize,5_usize,0_usize,13641183312568331251_usize,7994373504316847005_usize];
_14 = 13356326050925757353_usize as isize;
match _13 {
0 => bb2,
1 => bb3,
340282366920938463456244137026194249250 => bb5,
_ => bb4
}
}
bb10 = {
_15 = _7;
_19.fld2 = (6_i8,);
_19.fld4.fld3.3[_12] = _19.fld6[_12];
_19.fld4.fld1.3 = _19.fld4.fld3.3[_12];
_11 = _6 as isize;
_19.fld1.2.0.1 = _19.fld5 as i16;
_19.fld4.fld2 = _13;
_19.fld4.fld2 = -_13;
_19.fld1.5 = core::ptr::addr_of!(_9);
_19.fld4.fld3.1 = (-5298525527707983966238098354684157316_i128) as usize;
_15 = _7;
_19.fld1.2.1 = [_12,_12,_12,_12,_12];
_19.fld3.fld1 = _19.fld2.0;
_8 = [_12,_12,_12,_12,_19.fld4.fld3.1];
_21.0 = [_19.fld5,_19.fld5,_19.fld5,_19.fld5,_19.fld5];
_19.fld1.2.0.1 = !3446_i16;
_7 = _15;
_19.fld4.fld0 = [_2,_2];
_19.fld1.0 = _19.fld1.5;
_19.fld4.fld1.1 = !_19.fld4.fld3.3[_12];
_19.fld1.1 = _12 | _12;
_19.fld3.fld1 = _19.fld2.0;
match _12 {
0 => bb7,
1 => bb2,
2 => bb11,
3 => bb12,
4 => bb13,
7 => bb15,
_ => bb14
}
}
bb11 = {
_15 = _7;
_13 = (-646704160237922530_i64);
_9 = _4 << _6;
_14 = (-13577_i16) as isize;
_7 = _15;
_3 = _6 % 174045852130961309233580342609497471838_u128;
_16 = _14 + _11;
_5 = [4_usize,6_usize,15567615961268079745_usize,6_usize,16392244206305774149_usize];
_6 = _2 as u128;
_9 = !_4;
_6 = (-27192_i16) as u128;
_18 = _13 as f64;
RET = (-27876_i16) as i32;
_18 = _3 as f64;
_18 = (-93_i8) as f64;
_5 = [276432569227973242_usize,0_usize,7_usize,15957507447694864913_usize,1_usize];
_2 = !_10;
_6 = _3 / 127615660140420942872849263358880513318_u128;
_10 = _2 | _2;
_13 = (-7130470405573962206_i64);
_6 = _3;
_7 = _15;
_3 = _4 as u128;
RET = 826003573_i32 << _10;
_1 = [8721063455186636364_usize,5_usize,0_usize,13641183312568331251_usize,7994373504316847005_usize];
_14 = 13356326050925757353_usize as isize;
match _13 {
0 => bb2,
1 => bb3,
340282366920938463456244137026194249250 => bb5,
_ => bb4
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_15 = _7;
_13 = (-646704160237922530_i64);
_9 = _4 << _6;
_14 = (-13577_i16) as isize;
_7 = _15;
_3 = _6 % 174045852130961309233580342609497471838_u128;
_16 = _14 + _11;
_5 = [4_usize,6_usize,15567615961268079745_usize,6_usize,16392244206305774149_usize];
_6 = _2 as u128;
_9 = !_4;
_6 = (-27192_i16) as u128;
_18 = _13 as f64;
RET = (-27876_i16) as i32;
_18 = _3 as f64;
_18 = (-93_i8) as f64;
_5 = [276432569227973242_usize,0_usize,7_usize,15957507447694864913_usize,1_usize];
_2 = !_10;
_6 = _3 / 127615660140420942872849263358880513318_u128;
_10 = _2 | _2;
_13 = (-7130470405573962206_i64);
_6 = _3;
_7 = _15;
_3 = _4 as u128;
RET = 826003573_i32 << _10;
_1 = [8721063455186636364_usize,5_usize,0_usize,13641183312568331251_usize,7994373504316847005_usize];
_14 = 13356326050925757353_usize as isize;
match _13 {
0 => bb2,
1 => bb3,
340282366920938463456244137026194249250 => bb5,
_ => bb4
}
}
bb15 = {
_19.fld1.2.0.2 = _19.fld1.2.0.1 - _19.fld1.2.0.1;
RET = 9686381338339157090_u64 as i32;
_19.fld1.2.0.1 = _4 as i16;
_19.fld6 = [_16,_19.fld4.fld1.1,_19.fld4.fld1.1,_19.fld4.fld1.3,_16,_19.fld4.fld1.3,_19.fld4.fld1.3,_11];
_19.fld1.2.0 = (_7, (-16240_i16), 18_i16);
Goto(bb16)
}
bb16 = {
Call(_25 = dump_var(10_usize, 7_usize, Move(_7), 10_usize, Move(_10), 15_usize, Move(_15), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_25 = dump_var(10_usize, 14_usize, Move(_14), 13_usize, Move(_13), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: i64,mut _2: [usize; 5],mut _3: i64,mut _4: [usize; 5],mut _5: isize,mut _6: [usize; 5],mut _7: u16,mut _8: char,mut _9: [usize; 5],mut _10: u128,mut _11: isize,mut _12: [usize; 5],mut _13: [usize; 5]) -> u8 {
mir! {
type RET = u8;
let _14: [char; 4];
let _15: (u32, [usize; 5], u32, i32);
let _16: f32;
let _17: (i8,);
let _18: [isize; 4];
let _19: Adt62;
let _20: f32;
let _21: f64;
let _22: i128;
let _23: f64;
let _24: bool;
let _25: [u8; 2];
let _26: Adt57;
let _27: ([isize; 8], usize, u32, [isize; 8]);
let _28: char;
let _29: bool;
let _30: usize;
let _31: [usize; 5];
let _32: [u32; 5];
let _33: [isize; 4];
let _34: u16;
let _35: isize;
let _36: ((*const *const u64, isize, *const [u32; 2], isize, bool), *const *const [u32; 2]);
let _37: isize;
let _38: Adt56;
let _39: usize;
let _40: u32;
let _41: ();
let _42: ();
{
_9 = _12;
_2 = _6;
_14 = [_8,_8,_8,_8];
_2 = [5_usize,2_usize,9210060914528031063_usize,2973734744795820006_usize,2_usize];
RET = 231_u8 + 247_u8;
_13 = [16284974417207062154_usize,14801332422878189638_usize,2_usize,8943140224496597656_usize,2_usize];
Goto(bb1)
}
bb1 = {
_7 = 32279_u16 << _3;
_1 = _3;
_6 = _13;
_13 = [5846149013306366291_usize,438605541659994147_usize,3_usize,9614120859800110942_usize,11814588053311521660_usize];
_15.1 = [16283205905059005811_usize,3_usize,7_usize,11020727227516333565_usize,11328354423794145446_usize];
_15.3 = 116_i8 as i32;
_5 = _8 as isize;
_15.0 = 217_u8 as u32;
match _3 {
8384546526171475284 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_11 = _5;
_17 = (59_i8,);
_15.2 = _15.0 / 2257993355_u32;
_2 = [7_usize,6_usize,3653001037280704295_usize,7_usize,6_usize];
_17 = (124_i8,);
_18 = [_11,_11,_5,_5];
_7 = !337_u16;
_10 = 335030582708024800528164315661059896951_u128 >> _5;
_15 = (2717772048_u32, _6, 810920296_u32, (-957050747_i32));
_16 = _17.0 as f32;
match _15.2 {
0 => bb1,
810920296 => bb4,
_ => bb2
}
}
bb4 = {
_15.3 = (-1367818831_i32) ^ 713374530_i32;
_17 = ((-44_i8),);
_15.2 = _10 as u32;
_13 = [17857129335371190682_usize,4720046555152169157_usize,6344101510626472162_usize,14949375823647836349_usize,6_usize];
RET = 113_u8 | 159_u8;
_17.0 = 7450328296900147866_usize as i8;
RET = _8 as u8;
_10 = !25572215243675547412680471556420183793_u128;
RET = 128_u8 ^ 197_u8;
_16 = _15.3 as f32;
_17.0 = (-25406_i16) as i8;
_15.1 = _9;
RET = 134_u8;
_19.fld5.fld5 = !_15.0;
_19.fld4.1 = [12024316738353525584_usize,3711184714762805062_usize,2185140452149422663_usize,7_usize,1_usize];
_19.fld5.fld0.fld3.2 = !_19.fld5.fld5;
_3 = !_1;
_15.2 = _16 as u32;
_19.fld5.fld0.fld3.3 = [_5,_11,_5,_11,_5,_5,_5,_11];
_19.fld4.1 = _2;
_19.fld4.0.1 = _10 as i16;
Goto(bb5)
}
bb5 = {
_19.fld5.fld0.fld1.1 = _5;
_19.fld5.fld0.fld0 = [254_u8,37_u8];
_15.2 = _19.fld5.fld0.fld3.2 * _19.fld5.fld5;
_19.fld3 = core::ptr::addr_of!(_19.fld4.2.0);
_19.fld5.fld3 = _17.0 | _17.0;
_19.fld5.fld2.3 = _17.0 as f64;
_19.fld4.2.0 = core::ptr::addr_of_mut!(_15.3);
_19.fld5.fld0.fld1.4 = !true;
_2 = [7_usize,14901731341398288611_usize,5_usize,8171812788554494388_usize,14011764088020676524_usize];
_2 = [7_usize,4_usize,5_usize,12900355171429155396_usize,15576300964158839488_usize];
_15.2 = !_15.0;
_4 = [13948803808405324633_usize,11996573012128488905_usize,2_usize,4292036221023059188_usize,16079365370718674192_usize];
_19.fld0 = [_8,_8,_8,_8];
_15.0 = !_19.fld5.fld5;
_20 = -_16;
_19.fld5.fld2.4 = _20;
_15.1 = [3_usize,5_usize,15987277669034481623_usize,883386841242915300_usize,4_usize];
_6 = [3532452015921021191_usize,4_usize,3_usize,2720225210368433369_usize,5821833555365585717_usize];
_19.fld5.fld2.4 = _20 + _20;
_19.fld5.fld0.fld1.3 = !_5;
Call(_1 = fn12(_12, _19.fld5.fld0.fld1.1, _6, _19.fld5.fld0.fld0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19.fld5.fld0.fld3.0 = [_5,_19.fld5.fld0.fld1.1,_11,_11,_5,_11,_11,_11];
_19.fld4.0 = (_8, 27089_i16, (-32480_i16));
_18 = [_19.fld5.fld0.fld1.1,_19.fld5.fld0.fld1.3,_11,_5];
_19.fld4.1 = _4;
_19.fld4.0.0 = _8;
_22 = (-143179573039152762599219814910939915475_i128) << _19.fld5.fld5;
_19.fld4.1 = _9;
_12 = _4;
_19.fld1 = core::ptr::addr_of_mut!(_19.fld5.fld0.fld3.3);
_15.0 = _15.2 / 3415600731_u32;
_19.fld5.fld0.fld1.1 = -_19.fld5.fld0.fld1.3;
_15.1 = [5_usize,12194657734269806718_usize,15258667147922511178_usize,7_usize,5_usize];
_23 = -_19.fld5.fld2.3;
_8 = _19.fld4.0.0;
RET = 231_u8 - 170_u8;
_19.fld5.fld2.3 = _23;
_5 = _16 as isize;
_19.fld4.1 = _9;
_26.fld1 = !4_usize;
_19.fld5.fld3 = !_17.0;
_26.fld0 = core::ptr::addr_of_mut!(_17.0);
_26.fld1 = 3414143360291008168_usize + 8883539307549758996_usize;
_19.fld5.fld0.fld2 = !_3;
_19.fld5.fld0.fld1.1 = _26.fld1 as isize;
Goto(bb7)
}
bb7 = {
_5 = _19.fld5.fld0.fld1.3 & _19.fld5.fld0.fld1.1;
_16 = _19.fld5.fld2.4 + _20;
_22 = _23 as i128;
_19.fld5.fld0.fld1.3 = _11 + _19.fld5.fld0.fld1.1;
_15.0 = _15.2;
match _19.fld4.0.1 {
0 => bb3,
1 => bb8,
27089 => bb10,
_ => bb9
}
}
bb8 = {
_15.3 = (-1367818831_i32) ^ 713374530_i32;
_17 = ((-44_i8),);
_15.2 = _10 as u32;
_13 = [17857129335371190682_usize,4720046555152169157_usize,6344101510626472162_usize,14949375823647836349_usize,6_usize];
RET = 113_u8 | 159_u8;
_17.0 = 7450328296900147866_usize as i8;
RET = _8 as u8;
_10 = !25572215243675547412680471556420183793_u128;
RET = 128_u8 ^ 197_u8;
_16 = _15.3 as f32;
_17.0 = (-25406_i16) as i8;
_15.1 = _9;
RET = 134_u8;
_19.fld5.fld5 = !_15.0;
_19.fld4.1 = [12024316738353525584_usize,3711184714762805062_usize,2185140452149422663_usize,7_usize,1_usize];
_19.fld5.fld0.fld3.2 = !_19.fld5.fld5;
_3 = !_1;
_15.2 = _16 as u32;
_19.fld5.fld0.fld3.3 = [_5,_11,_5,_11,_5,_5,_5,_11];
_19.fld4.1 = _2;
_19.fld4.0.1 = _10 as i16;
Goto(bb5)
}
bb9 = {
_11 = _5;
_17 = (59_i8,);
_15.2 = _15.0 / 2257993355_u32;
_2 = [7_usize,6_usize,3653001037280704295_usize,7_usize,6_usize];
_17 = (124_i8,);
_18 = [_11,_11,_5,_5];
_7 = !337_u16;
_10 = 335030582708024800528164315661059896951_u128 >> _5;
_15 = (2717772048_u32, _6, 810920296_u32, (-957050747_i32));
_16 = _17.0 as f32;
match _15.2 {
0 => bb1,
810920296 => bb4,
_ => bb2
}
}
bb10 = {
_19.fld5.fld0.fld1.4 = _15.3 <= _15.3;
_19.fld5.fld0.fld3.2 = !_15.2;
_4 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1];
_19.fld5.fld2 = (_20, _19.fld4.0.0, 228_u8, _23, _20);
_1 = _16 as i64;
_27.2 = _16 as u32;
_24 = _20 > _20;
_19.fld4.0.1 = -_19.fld4.0.2;
_19.fld5.fld0.fld2 = _1 & _1;
_21 = _23 - _23;
_27.0 = [_19.fld5.fld0.fld1.3,_5,_5,_5,_5,_19.fld5.fld0.fld1.3,_5,_19.fld5.fld0.fld1.1];
_15.3 = -1195587855_i32;
_1 = !_19.fld5.fld0.fld2;
_19.fld4.0.2 = _19.fld4.0.1 ^ _19.fld4.0.1;
_28 = _19.fld5.fld2.1;
_25 = [_19.fld5.fld2.2,_19.fld5.fld2.2];
_30 = _26.fld1 << _15.0;
_31 = [_30,_30,_30,_30,_30];
_19.fld5.fld5 = _15.2;
Call(_9 = fn16(_21, _25, _18, _19.fld4.0, _19.fld4.1, _14, _21, _19.fld4.0.0, _8, _8, _19.fld5.fld2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_29 = !_24;
_27.3 = _19.fld5.fld0.fld3.0;
_19.fld4.0.2 = _19.fld4.0.1;
_29 = !_19.fld5.fld0.fld1.4;
_19.fld5.fld0.fld1.3 = !_5;
_27.0 = [_5,_5,_5,_5,_5,_19.fld5.fld0.fld1.3,_19.fld5.fld0.fld1.3,_19.fld5.fld0.fld1.3];
_19.fld5.fld0.fld1.4 = !_29;
_19.fld5.fld2.2 = 184_u8 | 241_u8;
_33 = _18;
_34 = _7;
Goto(bb12)
}
bb12 = {
_19.fld4.0.0 = _8;
_25 = [_19.fld5.fld2.2,_19.fld5.fld2.2];
_19.fld0 = [_8,_19.fld4.0.0,_8,_28];
_22 = _19.fld5.fld2.2 as i128;
Call(_27 = fn17(_19.fld5.fld0.fld1.4, _21, _2, _19.fld5.fld0.fld1.3, _16, _16, _19.fld5.fld2.3, _15, _20, _19.fld4.2, _19.fld5.fld2.4, _26.fld1, _31), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_19.fld5.fld0.fld3.2 = _15.2 * _27.2;
_19.fld4.0.2 = _19.fld5.fld0.fld3.2 as i16;
_19.fld5.fld0.fld3.3 = _27.3;
_31 = _12;
_12 = [_30,_30,_30,_30,_30];
_8 = _28;
_6 = [_30,_30,_30,_30,_30];
_19.fld5.fld0.fld3.1 = _19.fld5.fld2.2 as usize;
_17 = (_19.fld5.fld3,);
_13 = _6;
_19.fld5.fld0.fld1.1 = _11;
_25 = _19.fld5.fld0.fld0;
_11 = _5 >> _19.fld4.0.2;
_38.fld0.fld2 = -_1;
Goto(bb14)
}
bb14 = {
_19.fld5.fld0.fld3.1 = !_30;
_38.fld0.fld1.3 = -_11;
_23 = -_21;
_6 = [_19.fld5.fld0.fld3.1,_30,_30,_19.fld5.fld0.fld3.1,_30];
_26.fld0 = core::ptr::addr_of_mut!(_17.0);
_20 = -_19.fld5.fld2.0;
_15.3 = 1704683678_i32;
_36.0.4 = !_19.fld5.fld0.fld1.4;
_19.fld5.fld0.fld1.4 = !_29;
_19.fld5.fld2 = (_16, _28, 205_u8, _21, _16);
_38.fld2.0 = 9146064067514095339_u64 as f32;
_15.1 = [_30,_26.fld1,_30,_19.fld5.fld0.fld3.1,_26.fld1];
_19.fld5.fld0.fld3.3 = [_11,_38.fld0.fld1.3,_38.fld0.fld1.3,_38.fld0.fld1.3,_11,_38.fld0.fld1.3,_38.fld0.fld1.3,_19.fld5.fld0.fld1.1];
_18 = _33;
_32 = [_15.0,_19.fld5.fld5,_15.0,_19.fld5.fld0.fld3.2,_15.0];
_27.2 = _38.fld0.fld2 as u32;
_38.fld0.fld1.1 = !_38.fld0.fld1.3;
_38.fld2 = _19.fld5.fld2;
_19.fld2 = !_38.fld0.fld1.3;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(11_usize, 14_usize, Move(_14), 1_usize, Move(_1), 2_usize, Move(_2), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(11_usize, 22_usize, Move(_22), 8_usize, Move(_8), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(11_usize, 12_usize, Move(_12), 25_usize, Move(_25), 27_usize, Move(_27), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(11_usize, 6_usize, Move(_6), 5_usize, Move(_5), 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [usize; 5],mut _2: isize,mut _3: [usize; 5],mut _4: [u8; 2]) -> i64 {
mir! {
type RET = i64;
let _5: f32;
let _6: char;
let _7: [isize; 8];
let _8: *mut [usize; 5];
let _9: isize;
let _10: i64;
let _11: Adt59;
let _12: [isize; 8];
let _13: *const *const [u32; 2];
let _14: u128;
let _15: (u32, [usize; 5], u32, i32);
let _16: *mut [isize; 8];
let _17: i128;
let _18: Adt50;
let _19: Adt56;
let _20: [isize; 4];
let _21: f32;
let _22: isize;
let _23: [u32; 2];
let _24: Adt57;
let _25: Adt65;
let _26: Adt59;
let _27: Adt52;
let _28: char;
let _29: isize;
let _30: isize;
let _31: isize;
let _32: ();
let _33: ();
{
_3 = [5_usize,7100041425828980116_usize,6_usize,5_usize,3_usize];
RET = 219841756516154599_i64;
Goto(bb1)
}
bb1 = {
RET = !2548383816547118370_i64;
RET = 2153134754_u32 as i64;
_4 = [57_u8,43_u8];
_2 = 40879_u16 as isize;
RET = (-7141170650579841490_i64);
RET = (-9174969356177347289_i64) + 6931258380005373460_i64;
Goto(bb2)
}
bb2 = {
_3 = [4_usize,4_usize,9799058648942416501_usize,7_usize,3_usize];
RET = (-8471842764002105304_i64) << _2;
_5 = 2913863934917060448_i64 as f32;
_5 = 1_usize as f32;
_2 = (-9223372036854775808_isize);
_2 = -69_isize;
_6 = '\u{e94e2}';
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
_4 = [187_u8,156_u8];
RET = (-7250642217501529874_i64) | 4676240253266948413_i64;
RET = 1668495982189688132_i64;
_3 = [6_usize,6_usize,5_usize,5562738264402813009_usize,0_usize];
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
_6 = '\u{a6c5}';
_5 = (-29094_i16) as f32;
_8 = core::ptr::addr_of_mut!(_1);
_6 = '\u{659de}';
_3 = [4512669008365627826_usize,2_usize,12603280892544746628_usize,1_usize,7779088940983010523_usize];
_9 = _2;
_2 = _9;
_4 = [168_u8,1_u8];
RET = 6552_i16 as i64;
_9 = _2;
_9 = _2 * _2;
Call((*_8) = fn13(_8, _8, _2, _6, _7, _9, _6, _8, _6, _5, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _5 as i64;
Goto(bb4)
}
bb4 = {
_5 = 7_usize as f32;
_6 = '\u{7dad4}';
_1 = _3;
(*_8) = [6619902534610663600_usize,10104040418338857878_usize,7_usize,18206858118218079876_usize,2_usize];
_7 = [_2,_9,_9,_9,_9,_9,_9,_2];
_3 = [704648412957926560_usize,3_usize,14122405237104801867_usize,13545461880692169941_usize,10850092505576985473_usize];
_8 = core::ptr::addr_of_mut!((*_8));
_3 = [2282713102281760332_usize,2_usize,4547760875084434417_usize,7080133857412534601_usize,7_usize];
_6 = '\u{5eb7}';
RET = 1017693373974788292_i64 | 2517910975747200956_i64;
_6 = '\u{415c3}';
_6 = '\u{f63c9}';
_8 = core::ptr::addr_of_mut!(_1);
_8 = core::ptr::addr_of_mut!((*_8));
_5 = 55141639188048439809119004339057256059_i128 as f32;
RET = (-5089277592666933850_i64);
_8 = core::ptr::addr_of_mut!((*_8));
_3 = [15829813315711747457_usize,0_usize,13699463160218361145_usize,2_usize,2_usize];
_2 = _5 as isize;
_9 = 56_i8 as isize;
RET = 1303159360258147653_i64 | 3136689346748432265_i64;
Goto(bb5)
}
bb5 = {
RET = 132_u8 as i64;
_7 = [_9,_2,_9,_9,_2,_2,_2,_2];
_5 = _9 as f32;
(*_8) = [7_usize,2_usize,9564344184105616031_usize,12196362576760806052_usize,3_usize];
Call(_4 = fn14(_5, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = 4595416988771661252_i64;
Call(_2 = core::intrinsics::transmute(_9), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11 = Adt59 { fld0: _2 };
(*_8) = _3;
_3 = (*_8);
_7 = [_11.fld0,_2,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_9];
_4 = [102_u8,246_u8];
_12 = _7;
RET = 3599097513215546036_i64;
_14 = 95350822806446812969870442709857768195_i128 as u128;
_5 = (-741031135_i32) as f32;
_7 = _12;
_2 = (-1227184847_i32) as isize;
_3 = [6_usize,16024634552776670097_usize,6_usize,3_usize,7_usize];
_6 = '\u{6d9a8}';
_11.fld0 = -_2;
_3 = [14713839806897716059_usize,3259075693906934635_usize,16543298624664068660_usize,7_usize,7_usize];
_4 = [192_u8,22_u8];
_9 = !_2;
_11.fld0 = _9 * _2;
_15.1 = [17784451119699409819_usize,15821899893944688620_usize,12613100749741764948_usize,2337382069591584993_usize,12912175850135740543_usize];
Goto(bb8)
}
bb8 = {
_10 = !4872274135322071924_i64;
_7 = _12;
_15.3 = _5 as i32;
RET = _10 & _10;
_10 = 1179429009200100063_i64 * (-2890718611868587569_i64);
_3 = [4179524677379474220_usize,1110679564451289247_usize,5_usize,2_usize,7908041473083064890_usize];
_15.3 = (-1433037427_i32);
_11.fld0 = _9 - _9;
_10 = (-5551942696243816792_i64) & 7887730311128652328_i64;
_9 = _11.fld0 - _11.fld0;
(*_8) = _15.1;
RET = _10 + _10;
_3 = [7657383708712261953_usize,2_usize,13503962603052615302_usize,16306807119985063449_usize,5_usize];
_2 = !_11.fld0;
_15.2 = 3854983128_u32 + 824692030_u32;
_9 = !_11.fld0;
Goto(bb9)
}
bb9 = {
_18.fld3.3 = [_9,_2,_2,_9,_2,_11.fld0,_2,_9];
_10 = (-995258731867039445_i64);
_17 = 31796052763654804646721328669146875571_i128 >> _14;
RET = _5 as i64;
_18.fld1.1 = _2 << _11.fld0;
_6 = '\u{643fa}';
_15.0 = _15.2 - _15.2;
_19.fld0.fld3 = (_18.fld3.3, 7_usize, _15.0, _18.fld3.3);
_19.fld3 = 66_i8 >> _19.fld0.fld3.1;
_19.fld0.fld1.4 = true ^ false;
_18.fld2 = _14 as i64;
_18.fld1.3 = _18.fld1.1;
_17 = 5140908154585512407615549390995337491_i128;
_19.fld2.4 = -_5;
Goto(bb10)
}
bb10 = {
_19.fld5 = _19.fld0.fld3.2;
_18.fld3.1 = (-2721_i16) as usize;
_9 = _11.fld0;
_18.fld3.0 = [_18.fld1.3,_18.fld1.3,_2,_2,_2,_9,_9,_18.fld1.1];
(*_8) = [_18.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1];
_19.fld2.3 = 15432_i16 as f64;
_15.1 = [_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1];
_14 = 209392388989675801760875841265473320254_u128 % 141053081299786230671193227724537929643_u128;
Goto(bb11)
}
bb11 = {
_6 = '\u{46264}';
_18.fld0 = _4;
_19.fld0.fld1.3 = _11.fld0;
_19.fld0.fld3.2 = _15.0;
RET = _18.fld2;
_10 = _19.fld2.3 as i64;
_13 = core::ptr::addr_of!(_19.fld0.fld1.2);
_18.fld2 = _10;
RET = -_18.fld2;
_18.fld3 = _19.fld0.fld3;
_19.fld0.fld1.1 = _11.fld0;
_15 = (_18.fld3.2, _1, _19.fld0.fld3.2, (-787994690_i32));
_19.fld2.1 = _6;
_18.fld1.1 = _19.fld0.fld1.3;
_18.fld3.2 = _15.2;
_11.fld0 = _2;
_18.fld1.4 = !_19.fld0.fld1.4;
_19.fld0.fld0 = [227_u8,45_u8];
_6 = _19.fld2.1;
_18.fld0 = [211_u8,87_u8];
_19.fld0.fld1.2 = core::ptr::addr_of!(_23);
_16 = core::ptr::addr_of_mut!(_7);
_19.fld0.fld1.1 = _9 + _18.fld1.3;
_17 = -156534530233230806308701635950063573517_i128;
(*_8) = [_19.fld0.fld3.1,_19.fld0.fld3.1,_18.fld3.1,_19.fld0.fld3.1,_18.fld3.1];
_25.fld4.fld1.5 = core::ptr::addr_of!(_25.fld4.fld1.4);
match _19.fld0.fld3.1 {
7 => bb13,
_ => bb12
}
}
bb12 = {
_10 = !4872274135322071924_i64;
_7 = _12;
_15.3 = _5 as i32;
RET = _10 & _10;
_10 = 1179429009200100063_i64 * (-2890718611868587569_i64);
_3 = [4179524677379474220_usize,1110679564451289247_usize,5_usize,2_usize,7908041473083064890_usize];
_15.3 = (-1433037427_i32);
_11.fld0 = _9 - _9;
_10 = (-5551942696243816792_i64) & 7887730311128652328_i64;
_9 = _11.fld0 - _11.fld0;
(*_8) = _15.1;
RET = _10 + _10;
_3 = [7657383708712261953_usize,2_usize,13503962603052615302_usize,16306807119985063449_usize,5_usize];
_2 = !_11.fld0;
_15.2 = 3854983128_u32 + 824692030_u32;
_9 = !_11.fld0;
Goto(bb9)
}
bb13 = {
_25.fld4.fld4.fld1.4 = !_18.fld1.4;
_25.fld4.fld4.fld1.4 = !_18.fld1.4;
_25.fld4.fld1.2.0.2 = !(-19810_i16);
_18.fld2 = 9181546289991095360_u64 as i64;
_25.fld4.fld1.2.0.0 = _6;
_25.fld4.fld4.fld3 = (_19.fld0.fld3.0, _19.fld0.fld3.1, _19.fld0.fld3.2, _18.fld3.0);
_11.fld0 = _15.3 as isize;
_25.fld2.fld0.fld0.fld3.1 = 32048_u16 as usize;
_24.fld1 = _18.fld3.1;
_25.fld4.fld1.3 = [_18.fld2,_18.fld2,_10,_18.fld2,_18.fld2];
_25.fld2.fld0.fld2.1 = _19.fld2.1;
_25.fld2.fld0.fld5 = _25.fld4.fld4.fld3.2 ^ _18.fld3.2;
_25.fld4.fld2 = (_19.fld3,);
_18.fld3.2 = _19.fld2.3 as u32;
_25.fld4.fld1.2.0.0 = _19.fld2.1;
match _15.3 {
0 => bb14,
1 => bb15,
2 => bb16,
340282366920938463463374607430980216766 => bb18,
_ => bb17
}
}
bb14 = {
RET = 132_u8 as i64;
_7 = [_9,_2,_9,_9,_2,_2,_2,_2];
_5 = _9 as f32;
(*_8) = [7_usize,2_usize,9564344184105616031_usize,12196362576760806052_usize,3_usize];
Call(_4 = fn14(_5, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_3 = [4_usize,4_usize,9799058648942416501_usize,7_usize,3_usize];
RET = (-8471842764002105304_i64) << _2;
_5 = 2913863934917060448_i64 as f32;
_5 = 1_usize as f32;
_2 = (-9223372036854775808_isize);
_2 = -69_isize;
_6 = '\u{e94e2}';
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
_4 = [187_u8,156_u8];
RET = (-7250642217501529874_i64) | 4676240253266948413_i64;
RET = 1668495982189688132_i64;
_3 = [6_usize,6_usize,5_usize,5562738264402813009_usize,0_usize];
_7 = [_2,_2,_2,_2,_2,_2,_2,_2];
_6 = '\u{a6c5}';
_5 = (-29094_i16) as f32;
_8 = core::ptr::addr_of_mut!(_1);
_6 = '\u{659de}';
_3 = [4512669008365627826_usize,2_usize,12603280892544746628_usize,1_usize,7779088940983010523_usize];
_9 = _2;
_2 = _9;
_4 = [168_u8,1_u8];
RET = 6552_i16 as i64;
_9 = _2;
_9 = _2 * _2;
Call((*_8) = fn13(_8, _8, _2, _6, _7, _9, _6, _8, _6, _5, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_19.fld5 = _19.fld0.fld3.2;
_18.fld3.1 = (-2721_i16) as usize;
_9 = _11.fld0;
_18.fld3.0 = [_18.fld1.3,_18.fld1.3,_2,_2,_2,_9,_9,_18.fld1.1];
(*_8) = [_18.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1];
_19.fld2.3 = 15432_i16 as f64;
_15.1 = [_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1,_19.fld0.fld3.1];
_14 = 209392388989675801760875841265473320254_u128 % 141053081299786230671193227724537929643_u128;
Goto(bb11)
}
bb17 = {
_18.fld3.3 = [_9,_2,_2,_9,_2,_11.fld0,_2,_9];
_10 = (-995258731867039445_i64);
_17 = 31796052763654804646721328669146875571_i128 >> _14;
RET = _5 as i64;
_18.fld1.1 = _2 << _11.fld0;
_6 = '\u{643fa}';
_15.0 = _15.2 - _15.2;
_19.fld0.fld3 = (_18.fld3.3, 7_usize, _15.0, _18.fld3.3);
_19.fld3 = 66_i8 >> _19.fld0.fld3.1;
_19.fld0.fld1.4 = true ^ false;
_18.fld2 = _14 as i64;
_18.fld1.3 = _18.fld1.1;
_17 = 5140908154585512407615549390995337491_i128;
_19.fld2.4 = -_5;
Goto(bb10)
}
bb18 = {
_25.fld4.fld1.2.0.0 = _19.fld2.1;
_25.fld2.fld0.fld1 = core::ptr::addr_of!(_25.fld4.fld3.fld2);
_25.fld3 = Adt59 { fld0: _9 };
_25.fld4.fld1.2.0.1 = _25.fld4.fld1.2.0.2 * _25.fld4.fld1.2.0.2;
_25.fld4.fld1.3 = [_18.fld2,_10,_18.fld2,_18.fld2,_18.fld2];
_19.fld4 = core::ptr::addr_of_mut!(_25.fld4.fld1.3);
_19.fld1 = core::ptr::addr_of!(_25.fld4.fld3.fld2);
_19.fld0.fld1.0 = core::ptr::addr_of!(_25.fld4.fld3.fld2);
_25.fld4.fld1.4 = 9054461957153412893_u64 as u16;
_25.fld4.fld1.5 = core::ptr::addr_of!(_25.fld4.fld1.4);
_25.fld2.fld0.fld0.fld1.1 = _25.fld4.fld1.2.0.1 as isize;
_25.fld4.fld1.2.0 = (_6, 13653_i16, (-3663_i16));
_25.fld2.fld0.fld0.fld3.3 = [_11.fld0,_11.fld0,_19.fld0.fld1.3,_11.fld0,_18.fld1.1,_19.fld0.fld1.1,_11.fld0,_9];
(*_16) = _25.fld2.fld0.fld0.fld3.3;
_25.fld4.fld4 = Adt50 { fld0: _4,fld1: _19.fld0.fld1,fld2: _18.fld2,fld3: _19.fld0.fld3 };
_6 = _25.fld4.fld1.2.0.0;
_23 = [_19.fld0.fld3.2,_25.fld2.fld0.fld5];
_19.fld0.fld3.3 = [_19.fld0.fld1.1,_25.fld4.fld4.fld1.1,_11.fld0,_25.fld4.fld4.fld1.1,_25.fld3.fld0,_9,_25.fld4.fld4.fld1.1,_11.fld0];
_25.fld2.fld0.fld0.fld3 = ((*_16), _18.fld3.1, _25.fld2.fld0.fld5, _7);
_27.fld4.1 = [_19.fld0.fld3.1,_19.fld0.fld3.1,_25.fld2.fld0.fld0.fld3.1,_24.fld1,_19.fld0.fld3.1];
_29 = _9;
_25.fld2.fld1 = !9204867977630426613_u64;
_25.fld2.fld0.fld0.fld1.2 = (*_13);
_19.fld0.fld3.2 = !_15.2;
_25.fld4.fld1.5 = core::ptr::addr_of!(_25.fld4.fld1.4);
_25.fld2.fld0.fld2 = (_19.fld2.4, _19.fld2.1, 137_u8, _19.fld2.3, _19.fld2.4);
Goto(bb19)
}
bb19 = {
Call(_32 = dump_var(12_usize, 14_usize, Move(_14), 12_usize, Move(_12), 29_usize, Move(_29), 4_usize, Move(_4)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_32 = dump_var(12_usize, 3_usize, Move(_3), 10_usize, Move(_10), 1_usize, Move(_1), 33_usize, _33), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: *mut [usize; 5],mut _2: *mut [usize; 5],mut _3: isize,mut _4: char,mut _5: [isize; 8],mut _6: isize,mut _7: char,mut _8: *mut [usize; 5],mut _9: char,mut _10: f32,mut _11: isize) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _12: *const u64;
let _13: f64;
let _14: Adt58;
let _15: ([isize; 8], usize, u32, [isize; 8]);
let _16: i64;
let _17: (u32, [usize; 5], u32, i32);
let _18: Adt64;
let _19: [isize; 4];
let _20: Adt64;
let _21: Adt49;
let _22: f32;
let _23: u32;
let _24: u8;
let _25: [i64; 5];
let _26: [isize; 4];
let _27: u8;
let _28: [isize; 4];
let _29: *const u64;
let _30: u128;
let _31: [char; 4];
let _32: (*mut i32,);
let _33: (i8,);
let _34: ((char, i16, i16), [usize; 5], (*mut i32,));
let _35: [isize; 8];
let _36: *const *const u64;
let _37: (u32, [usize; 5], u32, i32);
let _38: [isize; 8];
let _39: f32;
let _40: ();
let _41: ();
{
_9 = _4;
_5 = [_6,_6,_11,_3,_11,_11,_6,_11];
_3 = _11;
_1 = _8;
_9 = _4;
Goto(bb1)
}
bb1 = {
_3 = !_6;
_5 = [_3,_6,_6,_6,_3,_11,_11,_11];
RET = [2_usize,5380608810619089525_usize,5_usize,6_usize,9302161010611752322_usize];
_9 = _7;
_2 = _1;
_8 = _1;
_2 = _1;
RET = [5_usize,0_usize,2481553703117468427_usize,4384489531022748083_usize,5491812511044196818_usize];
_13 = 4_usize as f64;
Goto(bb2)
}
bb2 = {
_8 = _2;
_14.fld0.fld4.fld1.1 = 2087543232285262759_usize;
_14.fld0.fld4.fld4.fld0 = [120_u8,61_u8];
_14.fld0.fld4.fld4.fld1.0 = core::ptr::addr_of!(_12);
_14.fld2 = 3405273037316447084_u64 as isize;
_14.fld0.fld4.fld4.fld3.0 = _5;
_14.fld6 = core::ptr::addr_of_mut!(_14.fld0.fld0);
_14.fld1 = (_5, _14.fld0.fld4.fld1.1, 2640553806_u32, _5);
_14.fld0.fld1.fld3 = (_14.fld1.0, _14.fld1.1, _14.fld1.2, _14.fld1.3);
_14.fld0.fld4.fld4.fld2 = 1140623390946753988_i64;
_14.fld0.fld1.fld1.1 = _6 | _3;
_14.fld5.fld3.3 = _14.fld1.0;
_14.fld5.fld3.0 = [_11,_14.fld0.fld1.fld1.1,_11,_14.fld2,_14.fld2,_11,_14.fld0.fld1.fld1.1,_14.fld0.fld1.fld1.1];
_14.fld0.fld4.fld1.2.0 = (_7, (-19489_i16), 9886_i16);
_14.fld0.fld1.fld1.3 = -_11;
_14.fld0.fld4.fld4.fld3 = _14.fld1;
_14.fld0.fld3 = -(-66_i8);
Call(_14.fld0.fld1.fld1.0 = core::intrinsics::arith_offset(_14.fld0.fld4.fld4.fld1.0, 9223372036854775807_isize), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14.fld3.3 = (_4, _14.fld0.fld4.fld1.2.0.2, _14.fld0.fld4.fld1.2.0.2);
_14.fld0.fld0 = [_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2];
_5 = [_6,_11,_6,_14.fld0.fld1.fld1.1,_14.fld0.fld1.fld1.3,_11,_14.fld0.fld1.fld1.3,_14.fld0.fld1.fld1.3];
_14.fld0.fld4.fld2 = (_14.fld0.fld3,);
_14.fld5.fld1.1 = _14.fld0.fld4.fld1.2.0.1 as isize;
_14.fld0.fld4.fld1.3 = [_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2];
_14.fld3.3.1 = _14.fld3.3.2;
_14.fld3.1 = core::ptr::addr_of_mut!(_17.3);
_14.fld5.fld1.1 = _13 as isize;
_14.fld5.fld0 = _14.fld0.fld4.fld4.fld0;
_14.fld5.fld3 = _14.fld0.fld4.fld4.fld3;
_9 = _14.fld0.fld4.fld1.2.0.0;
match _14.fld0.fld1.fld3.1 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
2087543232285262759 => bb9,
_ => bb8
}
}
bb4 = {
_8 = _2;
_14.fld0.fld4.fld1.1 = 2087543232285262759_usize;
_14.fld0.fld4.fld4.fld0 = [120_u8,61_u8];
_14.fld0.fld4.fld4.fld1.0 = core::ptr::addr_of!(_12);
_14.fld2 = 3405273037316447084_u64 as isize;
_14.fld0.fld4.fld4.fld3.0 = _5;
_14.fld6 = core::ptr::addr_of_mut!(_14.fld0.fld0);
_14.fld1 = (_5, _14.fld0.fld4.fld1.1, 2640553806_u32, _5);
_14.fld0.fld1.fld3 = (_14.fld1.0, _14.fld1.1, _14.fld1.2, _14.fld1.3);
_14.fld0.fld4.fld4.fld2 = 1140623390946753988_i64;
_14.fld0.fld1.fld1.1 = _6 | _3;
_14.fld5.fld3.3 = _14.fld1.0;
_14.fld5.fld3.0 = [_11,_14.fld0.fld1.fld1.1,_11,_14.fld2,_14.fld2,_11,_14.fld0.fld1.fld1.1,_14.fld0.fld1.fld1.1];
_14.fld0.fld4.fld1.2.0 = (_7, (-19489_i16), 9886_i16);
_14.fld0.fld1.fld1.3 = -_11;
_14.fld0.fld4.fld4.fld3 = _14.fld1;
_14.fld0.fld3 = -(-66_i8);
Call(_14.fld0.fld1.fld1.0 = core::intrinsics::arith_offset(_14.fld0.fld4.fld4.fld1.0, 9223372036854775807_isize), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_3 = !_6;
_5 = [_3,_6,_6,_6,_3,_11,_11,_11];
RET = [2_usize,5380608810619089525_usize,5_usize,6_usize,9302161010611752322_usize];
_9 = _7;
_2 = _1;
_8 = _1;
_2 = _1;
RET = [5_usize,0_usize,2481553703117468427_usize,4384489531022748083_usize,5491812511044196818_usize];
_13 = 4_usize as f64;
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
_14.fld0.fld1.fld3.0 = _14.fld0.fld4.fld4.fld3.0;
_10 = _14.fld0.fld1.fld1.3 as f32;
_14.fld5.fld1.0 = core::ptr::addr_of!(_12);
_14.fld0.fld4.fld1.6 = 4024_u16 ^ 29471_u16;
_14.fld0.fld4.fld2.0 = !_14.fld0.fld3;
_17.1 = [_14.fld0.fld1.fld3.1,_14.fld0.fld1.fld3.1,_14.fld0.fld1.fld3.1,_14.fld0.fld1.fld3.1,_14.fld0.fld4.fld4.fld3.1];
_15.0 = _14.fld1.3;
_14.fld1.2 = !_14.fld5.fld3.2;
_18.fld2.1 = _14.fld0.fld4.fld1.2.0.2;
_15.2 = _14.fld1.2 + _14.fld1.2;
_18.fld1.0.0 = _14.fld3.3.0;
_14.fld1 = (_14.fld5.fld3.0, _14.fld0.fld1.fld3.1, _15.2, _14.fld0.fld1.fld3.3);
_19 = [_14.fld0.fld1.fld1.1,_11,_3,_3];
_18.fld0.fld1.0 = core::ptr::addr_of!(_14.fld0.fld4.fld1.4);
_14.fld0.fld3 = _14.fld0.fld4.fld1.6 as i8;
_5 = [_6,_6,_14.fld2,_14.fld0.fld1.fld1.1,_14.fld0.fld1.fld1.3,_6,_6,_14.fld0.fld1.fld1.1];
_18.fld0.fld1.3 = [_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2];
_20.fld0.fld1.5 = core::ptr::addr_of!(_14.fld0.fld4.fld1.4);
_20.fld1.0 = (_14.fld0.fld4.fld1.2.0.0, _14.fld3.3.2, _14.fld3.3.2);
_14.fld0.fld4.fld1.2.1 = [_14.fld5.fld3.1,_14.fld1.1,_14.fld0.fld4.fld1.1,_14.fld0.fld4.fld1.1,_14.fld5.fld3.1];
_13 = 15883361471399894904_u64 as f64;
_15.1 = _14.fld1.1 + _14.fld1.1;
_14.fld0.fld1.fld3.0 = [_14.fld0.fld1.fld1.1,_14.fld0.fld1.fld1.1,_14.fld5.fld1.1,_6,_14.fld0.fld1.fld1.3,_14.fld0.fld1.fld1.1,_6,_11];
_20.fld2 = _20.fld1.0;
_20.fld2 = (_20.fld1.0.0, _18.fld2.1, _20.fld1.0.1);
_8 = core::ptr::addr_of_mut!(_14.fld0.fld4.fld1.2.1);
Goto(bb10)
}
bb10 = {
_14.fld0.fld4.fld1.3 = [_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2,_14.fld0.fld4.fld4.fld2];
_18.fld0.fld1.1 = !_14.fld5.fld3.1;
_18.fld0.fld4.fld0 = _14.fld5.fld0;
_20.fld1.2.0 = core::ptr::addr_of_mut!(_21.fld5);
_20.fld0.fld1.6 = (-77263931069074958666618534904492032274_i128) as u16;
_14.fld0.fld4.fld4.fld1.3 = _14.fld0.fld1.fld1.1;
_17 = (_14.fld0.fld4.fld4.fld3.2, _14.fld0.fld4.fld1.2.1, _15.2, (-1248202986_i32));
_18.fld1.1 = [_14.fld0.fld1.fld3.1,_15.1,_15.1,_14.fld0.fld1.fld3.1,_14.fld1.1];
_14.fld0.fld4.fld2 = (_14.fld0.fld3,);
_14.fld0.fld4.fld4.fld1.2 = core::ptr::addr_of!(_14.fld0.fld5);
_21.fld3 = _14.fld0.fld4.fld2.0;
_20.fld0.fld4.fld3.2 = _17.2 / 958250056_u32;
_14.fld0.fld4.fld1.2.2 = (_14.fld3.1,);
_20.fld1.1 = (*_8);
_14.fld5.fld1.4 = !false;
_18.fld0.fld1.2.2 = _14.fld0.fld4.fld1.2.2;
Goto(bb11)
}
bb11 = {
_14.fld0.fld4.fld6 = [_14.fld0.fld1.fld1.1,_11,_14.fld0.fld4.fld4.fld1.3,_14.fld0.fld1.fld1.1,_6,_14.fld0.fld4.fld4.fld1.3,_3,_14.fld0.fld4.fld4.fld1.3];
_15.0 = [_6,_14.fld0.fld1.fld1.1,_14.fld5.fld1.1,_14.fld0.fld1.fld1.3,_14.fld0.fld1.fld1.1,_6,_14.fld0.fld4.fld4.fld1.3,_14.fld0.fld1.fld1.1];
_20.fld0.fld1.2.2.0 = core::ptr::addr_of_mut!(_21.fld5);
_18.fld1 = (_14.fld0.fld4.fld1.2.0, (*_8), _18.fld0.fld1.2.2);
_14.fld5.fld3.3 = [_14.fld0.fld4.fld4.fld1.3,_14.fld0.fld1.fld1.1,_14.fld0.fld4.fld4.fld1.3,_14.fld0.fld1.fld1.3,_14.fld0.fld1.fld1.1,_6,_14.fld0.fld1.fld1.1,_14.fld0.fld4.fld4.fld1.3];
_23 = _17.2;
_17.0 = !_14.fld0.fld4.fld4.fld3.2;
_18.fld0.fld4.fld3.0 = _15.0;
_20.fld0.fld1.2.2.0 = core::ptr::addr_of_mut!(_21.fld5);
_20.fld0.fld4.fld1.4 = !_14.fld5.fld1.4;
_18.fld0.fld1.2 = _14.fld0.fld4.fld1.2;
_24 = 17_u8 & 159_u8;
_20.fld1.2 = (_18.fld0.fld1.2.2.0,);
_5 = [_14.fld0.fld4.fld4.fld1.3,_14.fld0.fld1.fld1.1,_14.fld0.fld1.fld1.1,_11,_14.fld0.fld1.fld1.3,_11,_6,_14.fld0.fld4.fld4.fld1.3];
(*_8) = [_18.fld0.fld1.1,_15.1,_15.1,_15.1,_18.fld0.fld1.1];
_18.fld0.fld4.fld1.0 = core::ptr::addr_of!(_21.fld6);
_15.0 = _14.fld0.fld4.fld4.fld3.3;
_18.fld1 = (_14.fld3.3, (*_8), _14.fld0.fld4.fld1.2.2);
_10 = _14.fld0.fld1.fld1.1 as f32;
_15.3 = [_14.fld0.fld4.fld4.fld1.3,_14.fld0.fld1.fld1.1,_14.fld0.fld1.fld1.3,_14.fld5.fld1.1,_6,_14.fld0.fld4.fld4.fld1.3,_6,_14.fld0.fld1.fld1.1];
_14.fld0.fld5 = [_20.fld0.fld4.fld3.2,_14.fld1.2];
_21.fld1 = _13 / f64::NAN;
_20.fld1.0 = (_18.fld0.fld1.2.0.0, _14.fld0.fld4.fld1.2.0.2, _18.fld2.1);
Call(_20.fld0.fld1.2.0.1 = core::intrinsics::bswap(_14.fld3.3.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14.fld0.fld4.fld4.fld1.4 = _14.fld5.fld1.4;
_14.fld5.fld2 = _14.fld0.fld4.fld4.fld2;
_20.fld0.fld3.fld1 = !_14.fld0.fld3;
_14.fld0.fld1.fld1.1 = _14.fld0.fld4.fld4.fld1.3;
_14.fld0.fld7 = core::ptr::addr_of_mut!(_20.fld0.fld4.fld3.0);
_14.fld0.fld4.fld1.4 = _17.3 as u16;
_14.fld5.fld3 = _14.fld1;
_20.fld0.fld4.fld3.3 = [_6,_14.fld0.fld4.fld4.fld1.3,_14.fld0.fld4.fld4.fld1.3,_14.fld2,_14.fld0.fld1.fld1.1,_11,_14.fld5.fld1.1,_14.fld0.fld4.fld4.fld1.3];
_20.fld0.fld1.1 = _14.fld0.fld4.fld2.0 as usize;
_14.fld3.3.2 = !_18.fld2.1;
_18.fld0.fld1.2.0.1 = _20.fld1.0.1 - _20.fld2.1;
_20.fld0.fld2.0 = -_14.fld0.fld4.fld2.0;
_14.fld3.3.2 = _18.fld0.fld1.2.0.1 + _14.fld3.3.1;
_20.fld0.fld1.6 = !_14.fld0.fld4.fld1.4;
_14.fld0.fld4.fld4.fld3 = (_14.fld0.fld1.fld3.0, _14.fld1.1, _23, _14.fld1.3);
Goto(bb13)
}
bb13 = {
_14.fld0.fld1.fld1.1 = -_14.fld0.fld4.fld4.fld1.3;
_14.fld0.fld4.fld4.fld1.2 = core::ptr::addr_of!(_20.fld0.fld0);
_18.fld0.fld3.fld1 = 8493310103291213325_u64 as i8;
_20.fld0.fld1.2.2.0 = _14.fld3.1;
_14.fld5.fld1.3 = _4 as isize;
_18.fld0.fld4.fld1.3 = _11 << _18.fld2.1;
_14.fld0.fld4.fld1.2 = _18.fld1;
_14.fld5.fld1.3 = _18.fld0.fld4.fld1.3;
RET = _14.fld0.fld4.fld1.2.1;
_18.fld0.fld6 = [_3,_18.fld0.fld4.fld1.3,_3,_3,_18.fld0.fld4.fld1.3,_14.fld0.fld1.fld1.1,_11,_11];
_27 = !_24;
_14.fld4 = _14.fld0.fld1.fld3.1;
_21.fld2.0 = [_20.fld0.fld4.fld3.2,_14.fld1.2,_14.fld1.2,_17.0,_14.fld0.fld4.fld4.fld3.2];
_18.fld2 = (_14.fld0.fld4.fld1.2.0.0, _14.fld3.3.2, _20.fld2.1);
_16 = _14.fld0.fld4.fld4.fld2 - _14.fld5.fld2;
_18.fld1.0.2 = _20.fld2.1;
_14.fld4 = _27 as usize;
_34.0 = (_14.fld3.3.0, _18.fld2.2, _20.fld1.0.2);
_18.fld0.fld1.0 = core::ptr::addr_of!(_14.fld0.fld4.fld1.4);
Goto(bb14)
}
bb14 = {
_18.fld1 = (_14.fld0.fld4.fld1.2.0, _14.fld0.fld4.fld1.2.1, _14.fld0.fld4.fld1.2.2);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(13_usize, 4_usize, Move(_4), 16_usize, Move(_16), 9_usize, Move(_9), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(13_usize, 6_usize, Move(_6), 15_usize, Move(_15), 3_usize, Move(_3), 41_usize, _41), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: f32,mut _2: *mut [usize; 5]) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _3: ((*const *const u64, isize, *const [u32; 2], isize, bool), *const *const [u32; 2]);
let _4: i64;
let _5: bool;
let _6: [usize; 5];
let _7: u128;
let _8: usize;
let _9: Adt52;
let _10: f64;
let _11: Adt59;
let _12: (u32, [usize; 5], u32, i32);
let _13: ((*const *const u64, isize, *const [u32; 2], isize, bool), *const *const [u32; 2]);
let _14: i32;
let _15: Adt62;
let _16: char;
let _17: char;
let _18: (*const *const u64, isize, *const [u32; 2], isize, bool);
let _19: isize;
let _20: i8;
let _21: u128;
let _22: i128;
let _23: bool;
let _24: Adt58;
let _25: [char; 4];
let _26: char;
let _27: Adt59;
let _28: (char, i16, i16);
let _29: (char, i16, i16);
let _30: i8;
let _31: u8;
let _32: i64;
let _33: char;
let _34: char;
let _35: char;
let _36: bool;
let _37: f32;
let _38: ();
let _39: ();
{
(*_2) = [3_usize,2639569759453305147_usize,5_usize,5194905699248469309_usize,6_usize];
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = [6_usize,17270533798144864910_usize,5_usize,3932287262829014302_usize,15459614575602312252_usize];
RET = [237_u8,35_u8];
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = [7_usize,14631720732631860149_usize,5_usize,1145287457940809632_usize,1599921028173769204_usize];
RET = [47_u8,228_u8];
RET = [36_u8,58_u8];
_3.1 = core::ptr::addr_of!(_3.0.2);
_3.0.1 = (-1401593390_i32) as isize;
_3.0.4 = false;
_1 = 220_u8 as f32;
_3.1 = core::ptr::addr_of!(_3.0.2);
_3.0.4 = !true;
(*_2) = [3_usize,1786846724795363201_usize,3_usize,9507067813141553078_usize,6_usize];
(*_2) = [4_usize,4670972219021826628_usize,16237576429304851760_usize,10724436726354494402_usize,11288012364123078829_usize];
RET = [219_u8,208_u8];
_2 = core::ptr::addr_of_mut!((*_2));
_4 = 5701207516283006434_i64;
(*_2) = [0_usize,7_usize,6_usize,6_usize,0_usize];
RET = [9_u8,245_u8];
_3.0.4 = _3.0.1 <= _3.0.1;
RET = [44_u8,223_u8];
_1 = 35466404934673159928426636986816884872_i128 as f32;
match _4 {
0 => bb1,
5701207516283006434 => bb3,
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
_4 = 2494059765391572324_i64 & 7337882551037643429_i64;
_5 = _3.0.4;
_3.0.4 = _5 | _5;
_6 = [386636236333697671_usize,4_usize,2_usize,10318086425953241271_usize,11735292591712158823_usize];
_4 = !(-5986487979020653687_i64);
_3.0.3 = _3.0.1;
_1 = 17413031028374361028_usize as f32;
RET = [223_u8,34_u8];
Goto(bb4)
}
bb4 = {
_6 = (*_2);
_3.1 = core::ptr::addr_of!(_3.0.2);
_2 = core::ptr::addr_of_mut!((*_2));
_4 = 3971589595012508151_usize as i64;
_3.0.1 = 47025_u16 as isize;
_5 = !_3.0.4;
_8 = 10401515310021166335_usize;
RET = [118_u8,162_u8];
_9.fld7 = _8 * _8;
_9.fld4.0 = 237_u8 as u32;
(*_2) = [_9.fld7,_9.fld7,_9.fld7,_8,_8];
_9.fld4.1 = _6;
_9.fld0.4 = _9.fld4.0 as f32;
_9.fld4.2 = 178_u8 as u32;
_3.1 = core::ptr::addr_of!(_13.0.2);
_10 = (-1514_i16) as f64;
_9.fld1 = -_10;
_15.fld2 = _3.0.1 * _3.0.3;
_9.fld1 = (-31812_i16) as f64;
RET = [33_u8,117_u8];
_9.fld0.3 = (-1989988974_i32) as f64;
Goto(bb5)
}
bb5 = {
_15.fld5.fld2.0 = 7516_i16 as f32;
_15.fld4.0.2 = -(-22881_i16);
_15.fld5.fld2.3 = _9.fld1 / f64::NAN;
_7 = _10 as u128;
_11 = Adt59 { fld0: _15.fld2 };
_9.fld0.3 = _9.fld4.2 as f64;
_9.fld6 = core::ptr::addr_of_mut!(_15.fld5.fld3);
_15.fld5.fld0.fld3.1 = _9.fld0.3 as usize;
Goto(bb6)
}
bb6 = {
_15.fld5.fld2.3 = -_9.fld1;
_12.3 = (-663068523_i32) * (-2010095399_i32);
_15.fld4.0 = ('\u{bd35f}', (-16640_i16), 2850_i16);
_10 = _9.fld1;
_9.fld4.1 = [_15.fld5.fld0.fld3.1,_8,_8,_9.fld7,_9.fld7];
_12.2 = !_9.fld4.2;
_13.0.3 = !_3.0.3;
_15.fld5.fld2.1 = _15.fld4.0.0;
Goto(bb7)
}
bb7 = {
_15.fld5.fld2 = (_1, _15.fld4.0.0, 130_u8, _10, _9.fld0.4);
_15.fld5.fld0.fld3.2 = _15.fld5.fld2.2 as u32;
_15.fld4.1 = (*_2);
_15.fld3 = core::ptr::addr_of!(_9.fld3.0);
_19 = _15.fld2 - _15.fld2;
_22 = -(-489667826810160496413088964852700171_i128);
_15.fld5.fld3 = !(-9_i8);
_14 = _12.3;
_15.fld4.1 = _6;
_15.fld5.fld2.4 = _1;
_24.fld0.fld4.fld4.fld3.1 = _4 as usize;
_15.fld4.0.2 = -_15.fld4.0.1;
match _8 {
0 => bb2,
1 => bb8,
2 => bb9,
3 => bb10,
10401515310021166335 => bb12,
_ => bb11
}
}
bb8 = {
_15.fld5.fld2.3 = -_9.fld1;
_12.3 = (-663068523_i32) * (-2010095399_i32);
_15.fld4.0 = ('\u{bd35f}', (-16640_i16), 2850_i16);
_10 = _9.fld1;
_9.fld4.1 = [_15.fld5.fld0.fld3.1,_8,_8,_9.fld7,_9.fld7];
_12.2 = !_9.fld4.2;
_13.0.3 = !_3.0.3;
_15.fld5.fld2.1 = _15.fld4.0.0;
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_4 = 2494059765391572324_i64 & 7337882551037643429_i64;
_5 = _3.0.4;
_3.0.4 = _5 | _5;
_6 = [386636236333697671_usize,4_usize,2_usize,10318086425953241271_usize,11735292591712158823_usize];
_4 = !(-5986487979020653687_i64);
_3.0.3 = _3.0.1;
_1 = 17413031028374361028_usize as f32;
RET = [223_u8,34_u8];
Goto(bb4)
}
bb12 = {
_9.fld4 = (_12.2, (*_2), _15.fld5.fld0.fld3.2, _12.3);
_12 = _9.fld4;
_9.fld4.3 = _12.3 + _12.3;
_24.fld0.fld4.fld4.fld1.0 = core::ptr::addr_of!(_24.fld0.fld4.fld3.fld2);
(*_2) = _6;
_5 = !_3.0.4;
_24.fld0.fld4.fld4.fld3.3 = [_11.fld0,_13.0.3,_13.0.3,_19,_3.0.3,_11.fld0,_11.fld0,_11.fld0];
_13.0.2 = core::ptr::addr_of!(_24.fld0.fld4.fld0);
_24.fld0.fld1.fld1.1 = _3.0.1 + _19;
_24.fld1 = (_24.fld0.fld4.fld4.fld3.3, _24.fld0.fld4.fld4.fld3.1, _12.2, _24.fld0.fld4.fld4.fld3.3);
_18.1 = _3.0.3 * _24.fld0.fld1.fld1.1;
_9.fld0 = (_15.fld5.fld2.0, _15.fld5.fld2.1, _15.fld5.fld2.2, _10, _15.fld5.fld2.4);
_15.fld5.fld0.fld3.1 = !_9.fld7;
_10 = _15.fld5.fld2.3;
_15.fld4.0 = (_15.fld5.fld2.1, (-26019_i16), 13578_i16);
(*_2) = _15.fld4.1;
_9.fld4.0 = _24.fld1.2 ^ _24.fld1.2;
_9.fld6 = core::ptr::addr_of_mut!(_24.fld0.fld4.fld3.fld1);
_3.0.2 = core::ptr::addr_of!(_24.fld0.fld5);
_18.2 = core::ptr::addr_of!(_24.fld0.fld4.fld0);
Call(_24.fld5.fld1.2 = fn15(_4, _24.fld1.3, _10, Move(_11)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_24.fld0.fld1.fld0 = [_9.fld0.2,_15.fld5.fld2.2];
_29.1 = -_15.fld4.0.2;
_24.fld0.fld4.fld1.2.0.1 = _15.fld5.fld3 as i16;
_24.fld5.fld0 = [_9.fld0.2,_9.fld0.2];
_33 = _15.fld4.0.0;
_24.fld0.fld4.fld1.2.0.0 = _9.fld0.1;
match _9.fld0.2 {
0 => bb14,
1 => bb15,
2 => bb16,
130 => bb18,
_ => bb17
}
}
bb14 = {
_4 = 2494059765391572324_i64 & 7337882551037643429_i64;
_5 = _3.0.4;
_3.0.4 = _5 | _5;
_6 = [386636236333697671_usize,4_usize,2_usize,10318086425953241271_usize,11735292591712158823_usize];
_4 = !(-5986487979020653687_i64);
_3.0.3 = _3.0.1;
_1 = 17413031028374361028_usize as f32;
RET = [223_u8,34_u8];
Goto(bb4)
}
bb15 = {
_4 = 2494059765391572324_i64 & 7337882551037643429_i64;
_5 = _3.0.4;
_3.0.4 = _5 | _5;
_6 = [386636236333697671_usize,4_usize,2_usize,10318086425953241271_usize,11735292591712158823_usize];
_4 = !(-5986487979020653687_i64);
_3.0.3 = _3.0.1;
_1 = 17413031028374361028_usize as f32;
RET = [223_u8,34_u8];
Goto(bb4)
}
bb16 = {
Return()
}
bb17 = {
_15.fld5.fld2.3 = -_9.fld1;
_12.3 = (-663068523_i32) * (-2010095399_i32);
_15.fld4.0 = ('\u{bd35f}', (-16640_i16), 2850_i16);
_10 = _9.fld1;
_9.fld4.1 = [_15.fld5.fld0.fld3.1,_8,_8,_9.fld7,_9.fld7];
_12.2 = !_9.fld4.2;
_13.0.3 = !_3.0.3;
_15.fld5.fld2.1 = _15.fld4.0.0;
Goto(bb7)
}
bb18 = {
_24.fld0.fld4.fld4.fld1.1 = !_24.fld0.fld1.fld1.1;
_29.0 = _15.fld4.0.0;
_12.3 = -_9.fld4.3;
_24.fld0.fld1.fld1.4 = _5;
_24.fld0.fld1.fld1.2 = core::ptr::addr_of!(_24.fld0.fld4.fld0);
_24.fld0.fld1.fld3 = (_24.fld1.0, _8, _24.fld1.2, _24.fld1.0);
_24.fld3.3 = _15.fld4.0;
_12.2 = !_24.fld1.2;
_31 = !_15.fld5.fld2.2;
_24.fld6 = core::ptr::addr_of_mut!(_24.fld0.fld4.fld1.3);
_24.fld0.fld1.fld1.0 = core::ptr::addr_of!(_24.fld0.fld4.fld3.fld2);
_15.fld5.fld0.fld3 = (_24.fld0.fld4.fld4.fld3.3, _24.fld0.fld4.fld4.fld3.1, _9.fld4.0, _24.fld1.0);
_24.fld0.fld4.fld1.1 = _9.fld4.0 as usize;
_24.fld0.fld4.fld2 = (_15.fld5.fld3,);
_24.fld3.3.0 = _24.fld0.fld4.fld1.2.0.0;
_24.fld0.fld1.fld3 = (_24.fld0.fld4.fld4.fld3.3, _24.fld0.fld4.fld1.1, _9.fld4.0, _24.fld1.3);
_15.fld4.1 = [_24.fld0.fld4.fld1.1,_24.fld0.fld4.fld1.1,_24.fld0.fld4.fld1.1,_24.fld0.fld1.fld3.1,_24.fld0.fld1.fld3.1];
_24.fld5.fld1.0 = core::ptr::addr_of!(_24.fld0.fld4.fld3.fld2);
_24.fld0.fld3 = -_24.fld0.fld4.fld2.0;
_24.fld0.fld1.fld3.3 = _24.fld0.fld4.fld4.fld3.3;
_24.fld1 = (_24.fld0.fld4.fld4.fld3.3, _24.fld0.fld1.fld3.1, _9.fld4.0, _15.fld5.fld0.fld3.3);
Goto(bb19)
}
bb19 = {
Call(_38 = dump_var(14_usize, 14_usize, Move(_14), 5_usize, Move(_5), 22_usize, Move(_22), 33_usize, Move(_33)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(14_usize, 4_usize, Move(_4), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i64,mut _2: [isize; 8],mut _3: f64,mut _4: Adt59) -> *const [u32; 2] {
mir! {
type RET = *const [u32; 2];
let _5: f32;
let _6: (f32, char, u8, f64, f32);
let _7: *const *const u64;
let _8: Adt56;
let _9: char;
let _10: [u8; 2];
let _11: *mut i32;
let _12: u8;
let _13: Adt65;
let _14: isize;
let _15: [isize; 8];
let _16: [i64; 5];
let _17: u16;
let _18: isize;
let _19: ((char, i16, i16), [usize; 5], (*mut i32,));
let _20: [isize; 4];
let _21: [u32; 2];
let _22: isize;
let _23: isize;
let _24: ();
let _25: ();
{
_2 = [_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0];
_3 = 58876_u16 as f64;
_6.1 = '\u{eeedf}';
_2 = [_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0];
_6.1 = '\u{1642f}';
_6.2 = 67_u8;
_6.0 = (-8382_i16) as f32;
_5 = (-17994_i16) as f32;
match _6.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
67 => bb8,
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
_6.4 = 4034_u16 as f32;
_2 = [_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0];
_6.1 = '\u{3a98f}';
_6.2 = !18_u8;
_4.fld0 = 1496678911_u32 as isize;
_6.1 = '\u{7ebd2}';
_3 = _6.2 as f64;
_2 = [_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0];
_6.0 = _6.4;
_8.fld0.fld3.0 = [_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0];
_8.fld0.fld3 = (_2, 3_usize, 2050763708_u32, _2);
_8.fld2.3 = _3 / f64::NAN;
_8.fld2.1 = _6.1;
_6 = (_5, _8.fld2.1, 73_u8, _8.fld2.3, _5);
_8.fld0.fld1.4 = false | true;
_8.fld2.4 = 62924929724857679888811947630791572047_i128 as f32;
_2 = _8.fld0.fld3.0;
_6.2 = 964594376_i32 as u8;
_6.1 = _8.fld2.1;
_8.fld2.1 = _6.1;
_8.fld2.0 = _6.4 - _8.fld2.4;
_8.fld2 = (_5, _6.1, _6.2, _3, _5);
_8.fld2.3 = -_6.3;
_8.fld3 = 92_i8;
Goto(bb9)
}
bb9 = {
_4.fld0 = 104600856242009576252323501091855083247_u128 as isize;
_8.fld3 = 62_i8 >> _1;
_10 = [_6.2,_8.fld2.2];
_1 = _8.fld0.fld3.2 as i64;
_8.fld0.fld1.1 = _4.fld0;
_8.fld0.fld3 = (_2, 0_usize, 2387221550_u32, _2);
_6.1 = _8.fld2.1;
_8.fld0.fld1.3 = _8.fld2.3 as isize;
_8.fld0.fld3.0 = [_8.fld0.fld1.3,_8.fld0.fld1.3,_8.fld0.fld1.3,_8.fld0.fld1.3,_4.fld0,_4.fld0,_4.fld0,_4.fld0];
_8.fld0.fld0 = [_6.2,_8.fld2.2];
_9 = _8.fld2.1;
match _8.fld0.fld3.2 {
0 => bb6,
1 => bb2,
2 => bb10,
2387221550 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_8.fld0.fld3.0 = _8.fld0.fld3.3;
_13.fld4.fld4.fld1.1 = -_4.fld0;
_13.fld2.fld1 = !10967523789404096447_u64;
_13.fld2.fld0.fld2.3 = 25544_u16 as f64;
_13.fld4.fld4.fld0 = [_6.2,_8.fld2.2];
_13.fld4.fld6 = [_13.fld4.fld4.fld1.1,_8.fld0.fld1.3,_13.fld4.fld4.fld1.1,_8.fld0.fld1.1,_8.fld0.fld1.3,_8.fld0.fld1.3,_8.fld0.fld1.3,_8.fld0.fld1.3];
_12 = _6.2;
_13.fld2.fld0.fld3 = _8.fld0.fld3.2 as i8;
_13.fld2.fld0.fld5 = _13.fld2.fld0.fld3 as u32;
_8.fld2.0 = _8.fld0.fld3.2 as f32;
_13.fld4.fld4.fld3 = (_13.fld4.fld6, _8.fld0.fld3.1, _8.fld0.fld3.2, _8.fld0.fld3.0);
_13.fld2.fld0.fld0.fld1.3 = -_13.fld4.fld4.fld1.1;
_13.fld2.fld0.fld0.fld1.4 = _1 == _1;
_13.fld2.fld0.fld0.fld1.4 = _8.fld0.fld1.4;
_13.fld2.fld0.fld0.fld3 = _8.fld0.fld3;
_13.fld4.fld6 = [_8.fld0.fld1.1,_8.fld0.fld1.3,_8.fld0.fld1.1,_13.fld4.fld4.fld1.1,_13.fld4.fld4.fld1.1,_8.fld0.fld1.1,_4.fld0,_4.fld0];
_13.fld2.fld0.fld0.fld1.0 = core::ptr::addr_of!(_13.fld4.fld3.fld2);
_8.fld3 = _13.fld2.fld0.fld3;
_13.fld2.fld0.fld0.fld1.0 = core::ptr::addr_of!(_13.fld4.fld3.fld2);
_13.fld4.fld1.5 = core::ptr::addr_of!(_13.fld4.fld1.6);
match _8.fld0.fld3.1 {
1 => bb2,
2 => bb4,
3 => bb13,
0 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_10 = [_8.fld2.2,_12];
_13.fld2.fld0.fld0.fld2 = _1 << _13.fld2.fld0.fld0.fld3.2;
_13.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_13.fld4.fld1.3);
_13.fld3 = Adt59 { fld0: _8.fld0.fld1.1 };
_13.fld2.fld0.fld2.0 = _8.fld2.0 + _5;
_13.fld2.fld0.fld0.fld3.0 = _13.fld2.fld0.fld0.fld3.3;
_13.fld4.fld4.fld3.1 = !_8.fld0.fld3.1;
_15 = _13.fld4.fld4.fld3.0;
_9 = _8.fld2.1;
_8.fld1 = core::ptr::addr_of!(_13.fld4.fld3.fld2);
_8.fld2.4 = _6.2 as f32;
_13.fld4.fld4.fld0 = _10;
_13.fld4.fld4.fld3.3 = _15;
_13.fld4.fld2.0 = !_13.fld2.fld0.fld3;
_13.fld4.fld1.2.0.0 = _6.1;
_13.fld4.fld1.5 = core::ptr::addr_of!(_13.fld4.fld1.6);
_13.fld2.fld0.fld0.fld1.0 = core::ptr::addr_of!(_13.fld4.fld3.fld2);
_13.fld2.fld0.fld5 = !_8.fld0.fld3.2;
_13.fld4.fld3.fld1 = _13.fld2.fld0.fld3 & _8.fld3;
RET = core::ptr::addr_of!(_13.fld4.fld0);
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(15_usize, 10_usize, Move(_10), 9_usize, Move(_9), 15_usize, Move(_15), 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: f64,mut _2: [u8; 2],mut _3: [isize; 4],mut _4: (char, i16, i16),mut _5: [usize; 5],mut _6: [char; 4],mut _7: f64,mut _8: char,mut _9: char,mut _10: char,mut _11: (f32, char, u8, f64, f32)) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _12: u128;
let _13: (char, i16, i16);
let _14: bool;
let _15: Adt51;
let _16: ([isize; 8], usize, u32, [isize; 8]);
let _17: [i64; 5];
let _18: i16;
let _19: ([u32; 5],);
let _20: ([isize; 8], usize, u32, [isize; 8]);
let _21: f64;
let _22: Adt52;
let _23: Adt50;
let _24: i8;
let _25: [u32; 5];
let _26: u16;
let _27: (f32, char, u8, f64, f32);
let _28: (f32, char, u8, f64, f32);
let _29: Adt61;
let _30: i32;
let _31: [isize; 4];
let _32: bool;
let _33: Adt54;
let _34: *const u16;
let _35: ();
let _36: ();
{
_9 = _8;
RET = [1_usize,6_usize,7_usize,3_usize,6_usize];
_11.1 = _8;
RET = _5;
_11.4 = -_11.0;
_6 = [_4.0,_4.0,_9,_8];
_12 = 312808238888508685972867553702311576208_u128 << _11.2;
_11.4 = _11.2 as f32;
_11.4 = _11.0;
_4.1 = _11.2 as i16;
_4 = (_9, 4919_i16, 15141_i16);
_4.2 = _7 as i16;
_4.2 = _4.1 << _12;
_6 = [_10,_8,_9,_10];
_11.4 = -_11.0;
_4.0 = _10;
_2 = [_11.2,_11.2];
Call(_11.0 = core::intrinsics::transmute(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.0 = _9;
Goto(bb2)
}
bb2 = {
_3 = [(-9223372036854775808_isize),(-42_isize),9223372036854775807_isize,(-69_isize)];
_13.2 = _4.2;
_13.1 = _4.2;
_4.1 = _13.1;
Goto(bb3)
}
bb3 = {
_3 = [22_isize,9223372036854775807_isize,(-117_isize),56_isize];
_13 = (_11.1, _4.1, _4.2);
_2 = [_11.2,_11.2];
_4.2 = !_4.1;
_6 = [_10,_8,_10,_13.0];
_11.2 = 203_u8 & 181_u8;
_11.0 = (-87_i8) as f32;
_15.fld0 = core::ptr::addr_of_mut!(_11);
_8 = _13.0;
_13 = (_4.0, _4.1, _4.2);
_11.1 = _9;
_4.2 = _13.2 ^ _13.1;
RET = [6_usize,7_usize,3_usize,6097096301596775180_usize,6_usize];
_1 = (-104_isize) as f64;
_9 = _8;
_16.2 = 1738861087_u32 / 1766711547_u32;
_16.1 = !5_usize;
Goto(bb4)
}
bb4 = {
_3 = [57_isize,9223372036854775807_isize,28_isize,(-9223372036854775808_isize)];
_11.1 = _8;
RET = [_16.1,_16.1,_16.1,_16.1,_16.1];
_13.0 = _8;
_8 = _4.0;
_1 = -_7;
_16.0 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-101_isize),45_isize];
_4.0 = _8;
_12 = _10 as u128;
_15.fld0 = core::ptr::addr_of_mut!(_11);
_3 = [13_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_11.3 = -_1;
_12 = 2686825571788637363_u64 as u128;
_8 = _9;
Call(_11.3 = core::intrinsics::transmute(_16.1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14 = !false;
_4.0 = _9;
_4.1 = 33428878_i32 as i16;
_11.3 = (-9223372036854775808_isize) as f64;
_6 = [_10,_4.0,_13.0,_10];
_8 = _4.0;
_19.0 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_11.1 = _4.0;
_16.1 = 7_usize - 6_usize;
_1 = _11.3 * _11.3;
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_13.2 = _4.2 & _4.2;
_16.3 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-123_isize),6_isize,9223372036854775807_isize,(-107_isize),9223372036854775807_isize];
_15.fld0 = core::ptr::addr_of_mut!(_11);
_13.1 = !_13.2;
Goto(bb6)
}
bb6 = {
_9 = _11.1;
_17 = [6635980768466487080_i64,(-7091044957102529655_i64),602354499093120333_i64,4261931682286983872_i64,5803479418167178127_i64];
_15.fld1 = !(-38_i8);
RET = [_16.1,_16.1,_16.1,_16.1,_16.1];
_13.2 = -_13.1;
_11.0 = 10480_u16 as f32;
_20.0 = _16.3;
_17 = [(-3592906905769302240_i64),6675657596930003391_i64,(-4872229052930102032_i64),2391760865914309197_i64,(-8151347727109406649_i64)];
_8 = _11.1;
_13.1 = _13.2;
_12 = 4050381503905140909_i64 as u128;
_4.0 = _10;
_22.fld4.1 = _5;
_22.fld3.0 = core::ptr::addr_of_mut!(_22.fld4.3);
_11.3 = -_1;
Goto(bb7)
}
bb7 = {
_13 = _4;
_20.3 = _20.0;
_23.fld3.3 = [9223372036854775807_isize,9223372036854775807_isize,114_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_19.0 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_4.0 = _11.1;
_22.fld0 = _11;
_11.1 = _8;
_22.fld0.3 = _7;
_22.fld4.2 = (-77439546276361842311313932740870344786_i128) as u32;
_16.3 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-3_isize),(-9223372036854775808_isize),(-112_isize),(-9223372036854775808_isize)];
_22.fld5 = core::ptr::addr_of_mut!(_23.fld3.3);
_13.0 = _4.0;
_23.fld1.1 = _14 as isize;
_4.2 = _13.2 >> _16.2;
_23.fld3.1 = 13907413836426764748_u64 as usize;
_20.3 = _20.0;
_19.0 = [_16.2,_16.2,_16.2,_16.2,_22.fld4.2];
_22.fld4.2 = !_16.2;
_27.0 = _22.fld0.4 - _11.4;
_28.1 = _4.0;
Call(_16.3 = core::intrinsics::transmute(_20.3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_25 = [_22.fld4.2,_22.fld4.2,_22.fld4.2,_16.2,_16.2];
_27.0 = _4.2 as f32;
_22.fld4.3 = (-665243475_i32);
_22.fld2 = core::ptr::addr_of!(_15.fld2);
match _22.fld4.3 {
0 => bb5,
1 => bb6,
2 => bb9,
3 => bb10,
340282366920938463463374607431102967981 => bb12,
_ => bb11
}
}
bb9 = {
_4.0 = _9;
Goto(bb2)
}
bb10 = {
_9 = _11.1;
_17 = [6635980768466487080_i64,(-7091044957102529655_i64),602354499093120333_i64,4261931682286983872_i64,5803479418167178127_i64];
_15.fld1 = !(-38_i8);
RET = [_16.1,_16.1,_16.1,_16.1,_16.1];
_13.2 = -_13.1;
_11.0 = 10480_u16 as f32;
_20.0 = _16.3;
_17 = [(-3592906905769302240_i64),6675657596930003391_i64,(-4872229052930102032_i64),2391760865914309197_i64,(-8151347727109406649_i64)];
_8 = _11.1;
_13.1 = _13.2;
_12 = 4050381503905140909_i64 as u128;
_4.0 = _10;
_22.fld4.1 = _5;
_22.fld3.0 = core::ptr::addr_of_mut!(_22.fld4.3);
_11.3 = -_1;
Goto(bb7)
}
bb11 = {
_14 = !false;
_4.0 = _9;
_4.1 = 33428878_i32 as i16;
_11.3 = (-9223372036854775808_isize) as f64;
_6 = [_10,_4.0,_13.0,_10];
_8 = _4.0;
_19.0 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_11.1 = _4.0;
_16.1 = 7_usize - 6_usize;
_1 = _11.3 * _11.3;
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_13.2 = _4.2 & _4.2;
_16.3 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-123_isize),6_isize,9223372036854775807_isize,(-107_isize),9223372036854775807_isize];
_15.fld0 = core::ptr::addr_of_mut!(_11);
_13.1 = !_13.2;
Goto(bb6)
}
bb12 = {
_22.fld2 = core::ptr::addr_of!(_15.fld2);
_23.fld3.3 = _20.0;
_29.fld0 = _22.fld4.2;
_13.0 = _9;
match _22.fld4.3 {
0 => bb1,
1 => bb10,
340282366920938463463374607431102967981 => bb14,
_ => bb13
}
}
bb13 = {
_4.0 = _9;
Goto(bb2)
}
bb14 = {
_15.fld1 = 51_i8 ^ 124_i8;
_29.fld4 = core::ptr::addr_of_mut!(_5);
_30 = _22.fld4.3;
_21 = _1;
_23.fld3.0 = [_23.fld1.1,_23.fld1.1,_23.fld1.1,_23.fld1.1,_23.fld1.1,_23.fld1.1,_23.fld1.1,_23.fld1.1];
_23.fld3 = _16;
_22.fld4 = (_29.fld0, _5, _23.fld3.2, _30);
_23.fld1.3 = _23.fld1.1 - _23.fld1.1;
_22.fld4.1 = [_23.fld3.1,_23.fld3.1,_23.fld3.1,_16.1,_16.1];
_22.fld7 = _16.1;
_27.4 = _27.0 - _27.0;
_1 = -_21;
_27.3 = -_7;
_23.fld3.1 = (-32595058005447978404770246811606809688_i128) as usize;
_13.0 = _22.fld0.1;
_33.fld4.fld1.6 = 49824_u16 - 25523_u16;
_32 = !_14;
_29.fld2.0.1 = -_23.fld1.1;
_33.fld4.fld4.fld3.0 = _23.fld3.0;
_29.fld3 = core::ptr::addr_of!(_33.fld1.fld1.2);
_33.fld4.fld3.fld0 = core::ptr::addr_of_mut!(_28);
_31 = [_23.fld1.1,_23.fld1.1,_29.fld2.0.1,_23.fld1.3];
_33.fld4.fld1.4 = _22.fld0.1 as u16;
_26 = _22.fld4.0 as u16;
_29.fld2.0.0 = core::ptr::addr_of!(_33.fld4.fld3.fld2);
_33.fld4.fld1.1 = _22.fld7;
_33.fld4.fld4.fld3 = (_20.3, _33.fld4.fld1.1, _29.fld0, _23.fld3.3);
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(16_usize, 32_usize, Move(_32), 3_usize, Move(_3), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(16_usize, 26_usize, Move(_26), 31_usize, Move(_31), 16_usize, Move(_16), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(16_usize, 10_usize, Move(_10), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: bool,mut _2: f64,mut _3: [usize; 5],mut _4: isize,mut _5: f32,mut _6: f32,mut _7: f64,mut _8: (u32, [usize; 5], u32, i32),mut _9: f32,mut _10: (*mut i32,),mut _11: f32,mut _12: usize,mut _13: [usize; 5]) -> ([isize; 8], usize, u32, [isize; 8]) {
mir! {
type RET = ([isize; 8], usize, u32, [isize; 8]);
let _14: [usize; 5];
let _15: [isize; 4];
let _16: bool;
let _17: Adt60;
let _18: Adt59;
let _19: u32;
let _20: [u32; 2];
let _21: *mut i8;
let _22: f32;
let _23: ();
let _24: ();
{
RET.1 = _8.0 as usize;
_7 = -_2;
RET.0 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET.2 = _8.0 << _8.2;
_5 = _6;
_8.3 = _12 as i32;
_13 = [_12,_12,_12,_12,_12];
_8.3 = -(-397975461_i32);
_14 = _8.1;
_3 = [_12,_12,_12,_12,_12];
_3 = _8.1;
RET.2 = 118354588713138133335502533240172081557_u128 as u32;
RET.0 = [_4,_4,_4,_4,_4,_4,_4,_4];
_8.2 = 17591_u16 as u32;
RET.2 = _8.0;
_13 = [_12,_12,_12,_12,_12];
_15 = [_4,_4,_4,_4];
RET.3 = [_4,_4,_4,_4,_4,_4,_4,_4];
_1 = !false;
_8.1 = _13;
_1 = false;
RET.2 = 159_u8 as u32;
_4 = 9223372036854775807_isize << _8.0;
_3 = [_12,_12,_12,_12,_12];
_8 = (2422071003_u32, _14, 524136842_u32, (-728230537_i32));
RET.1 = _8.3 as usize;
_4 = !96_isize;
_7 = 236515656480350856287282507020470219215_u128 as f64;
_2 = _7;
match _8.3 {
340282366920938463463374607431039980919 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_8.0 = _8.2;
_10.0 = core::ptr::addr_of_mut!(_8.3);
_13 = _14;
RET.2 = _8.2 + _8.0;
_8.3 = _8.2 as i32;
RET.2 = !_8.0;
_9 = _11;
_11 = -_6;
_13 = _14;
_16 = _8.2 >= _8.0;
_7 = 11680866378387584219_u64 as f64;
RET.1 = !_12;
_2 = -_7;
_9 = _5;
_8.1 = [_12,_12,_12,_12,_12];
_8.2 = _8.0;
RET.1 = _12;
_1 = _16;
RET.1 = '\u{f42b}' as usize;
RET.0 = [_4,_4,_4,_4,_4,_4,_4,_4];
_8.2 = _8.0 / 3030219184_u32;
_8.3 = 1818552288_i32 * (-1792406085_i32);
_14 = _3;
_11 = _2 as f32;
_7 = _2 + _2;
RET.0 = [_4,_4,_4,_4,_4,_4,_4,_4];
_8.0 = _8.2;
Goto(bb3)
}
bb3 = {
Goto(bb4)
}
bb4 = {
_17.fld1.1 = _10.0;
Goto(bb5)
}
bb5 = {
_5 = _9;
_17.fld1.3.1 = 21913_i16;
_8.0 = !_8.2;
_17.fld1.3.2 = _17.fld1.3.1;
_11 = _6 * _5;
_4 = -(-9223372036854775808_isize);
Goto(bb6)
}
bb6 = {
_18.fld0 = _4 >> _8.0;
_4 = _18.fld0;
Call(_11 = core::intrinsics::transmute(_8.2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET.3 = [_18.fld0,_4,_4,_4,_18.fld0,_4,_18.fld0,_18.fld0];
_17.fld1.3.0 = '\u{c5e66}';
RET.2 = !_8.2;
_8.0 = _8.2 + _8.2;
Goto(bb8)
}
bb8 = {
RET.2 = _8.0 % 3781078436_u32;
_17.fld1.3.1 = _17.fld1.3.0 as i16;
_10.0 = _17.fld1.1;
_13 = [_12,_12,_12,_12,_12];
_18.fld0 = -_4;
RET.0 = [_18.fld0,_4,_18.fld0,_4,_18.fld0,_4,_4,_18.fld0];
_17.fld1.0 = _5;
_10.0 = core::ptr::addr_of_mut!(_8.3);
_2 = _7 / 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011126031124950123_f64;
RET.0 = [_18.fld0,_18.fld0,_18.fld0,_4,_4,_18.fld0,_4,_18.fld0];
RET.2 = _8.0;
_10.0 = _17.fld1.1;
_17.fld1.3.2 = _17.fld1.3.1;
_11 = 20033_u16 as f32;
_17.fld1.3 = ('\u{720a1}', 22230_i16, (-8447_i16));
Goto(bb9)
}
bb9 = {
Call(_23 = dump_var(17_usize, 16_usize, Move(_16), 4_usize, Move(_4), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(5755449137493060688_u64), std::hint::black_box('\u{cffa3}'), std::hint::black_box(13602_u16), std::hint::black_box((-35_i8)), std::hint::black_box((-19306_i16)), std::hint::black_box(28816769734200188768417994421904716766_u128), std::hint::black_box(5511815558880870173_i64), std::hint::black_box(3554124855_u32), std::hint::black_box(1_usize), std::hint::black_box(155_u8));
                
            }
#[derive(Debug,Copy,Clone)]
pub struct Adt49 {
fld0: f32,
fld1: f64,
fld2: ([u32; 5],),
fld3: i8,
fld4: *mut [usize; 5],
fld5: i32,
fld6: *const u64,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt50 {
fld0: [u8; 2],
fld1: (*const *const u64, isize, *const [u32; 2], isize, bool),
fld2: i64,
fld3: ([isize; 8], usize, u32, [isize; 8]),
}
#[derive(Debug)]
pub struct Adt51 {
fld0: *mut (f32, char, u8, f64, f32),
fld1: i8,
fld2: *const u64,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt52 {
fld0: (f32, char, u8, f64, f32),
fld1: f64,
fld2: *const *const u64,
fld3: (*mut i32,),
fld4: (u32, [usize; 5], u32, i32),
fld5: *mut [isize; 8],
fld6: *mut i8,
fld7: usize,
}
#[derive(Debug)]
pub struct Adt53 {
fld0: [u32; 2],
fld1: (*const u16, usize, ((char, i16, i16), [usize; 5], (*mut i32,)), [i64; 5], u16, *const u16, u16),
fld2: (i8,),
fld3: Adt51,
fld4: Adt50,
fld5: u32,
fld6: [isize; 8],
}
#[derive(Debug)]
pub struct Adt54 {
fld0: [i64; 5],
fld1: Adt50,
fld2: *const *const [u32; 2],
fld3: i8,
fld4: Adt53,
fld5: [u32; 2],
fld6: *mut i8,
fld7: *mut [isize; 8],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt55 {
fld0: u16,
fld1: [u8; 2],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt56 {
fld0: Adt50,
fld1: *const *const u64,
fld2: (f32, char, u8, f64, f32),
fld3: i8,
fld4: *mut [i64; 5],
fld5: u32,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt57 {
fld0: *mut i8,
fld1: usize,
}
#[derive(Debug)]
pub struct Adt58 {
fld0: Adt54,
fld1: ([isize; 8], usize, u32, [isize; 8]),
fld2: isize,
fld3: (f32, *mut i32, u128, (char, i16, i16)),
fld4: usize,
fld5: Adt50,
fld6: *mut [i64; 5],
}
#[derive(Debug)]
pub struct Adt59 {
fld0: isize,
}
#[derive(Debug)]
pub struct Adt60 {
fld0: *const u64,
fld1: (f32, *mut i32, u128, (char, i16, i16)),
fld2: *const u16,
}
#[derive(Debug)]
pub struct Adt61 {
fld0: u32,
fld1: u8,
fld2: ((*const *const u64, isize, *const [u32; 2], isize, bool), *const *const [u32; 2]),
fld3: *const *const [u32; 2],
fld4: *mut [usize; 5],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt62 {
fld0: [char; 4],
fld1: *mut [isize; 8],
fld2: isize,
fld3: *const *mut i32,
fld4: ((char, i16, i16), [usize; 5], (*mut i32,)),
fld5: Adt56,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt63 {
fld0: Adt56,
fld1: u64,
}
#[derive(Debug)]
pub struct Adt64 {
fld0: Adt53,
fld1: ((char, i16, i16), [usize; 5], (*mut i32,)),
fld2: (char, i16, i16),
}
#[derive(Debug)]
pub struct Adt65 {
fld0: [isize; 8],
fld1: Adt57,
fld2: Adt63,
fld3: Adt59,
fld4: Adt53,
}

