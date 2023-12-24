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
pub fn fn0(mut _1: u16,mut _2: char,mut _3: u8,mut _4: u64,mut _5: i16,mut _6: usize,mut _7: i64,mut _8: i128) -> f32 {
mir! {
type RET = f32;
let _9: [char; 6];
let _10: i8;
let _11: u8;
let _12: Adt50;
let _13: isize;
let _14: bool;
let _15: (isize, f64, u16);
let _16: char;
let _17: i128;
let _18: i8;
let _19: [char; 1];
let _20: (f32, [u16; 6], i64);
let _21: [isize; 3];
let _22: [char; 5];
let _23: f64;
let _24: Adt51;
let _25: f32;
let _26: *const i128;
let _27: [u32; 5];
let _28: i32;
let _29: i128;
let _30: i128;
let _31: ();
let _32: ();
{
_5 = (-5107_i16) & (-15944_i16);
_4 = 4953058258738910549_usize as u64;
_5 = 32743_u16 as i16;
_7 = (-697797473369653197_i64) * 2144684050210855990_i64;
_8 = (-11955640961993861272031781141053765097_i128);
RET = _8 as f32;
_1 = 52642_u16;
RET = 148_u8 as f32;
RET = _5 as f32;
_10 = 53_i8;
_6 = 13339674738222155489_usize;
_7 = (-2159037849445359163_i64);
_5 = (-17912_i16);
_4 = 1497059935013698805_u64 & 5379688658133040437_u64;
_6 = RET as usize;
RET = 1587956243_i32 as f32;
_10 = !35_i8;
_7 = (-9064328786169660525_i64);
_6 = _7 as usize;
_2 = '\u{18069}';
Call(_12.fld1 = fn1(_7, _2, _2, _4, _7, _4), bb1, UnwindUnreachable())
}
bb1 = {
_6 = 3371397367780341625_usize;
_3 = 42_u8 ^ 49_u8;
_9 = [_2,_2,_2,_2,_2,_2];
_8 = _4 as i128;
_11 = _3 >> _8;
_5 = 25153_i16;
_9 = [_2,_2,_2,_2,_2,_2];
_12.fld0.2 = !_7;
_12.fld0.2 = -_7;
_3 = _8 as u8;
_3 = _11;
_12.fld0.2 = false as i64;
_12.fld1 = [_2,_2,_2,_2,_2,_2];
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
25153 => bb9,
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
_2 = '\u{e6b33}';
_3 = !_11;
_13 = 9223372036854775807_isize & 9223372036854775807_isize;
_12.fld0.0 = -RET;
_8 = 28421598118075201582118035268826910821_i128;
_2 = '\u{faf7c}';
_2 = '\u{cd371}';
RET = _1 as f32;
_12.fld0.2 = _6 as i64;
match _5 {
25153 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_7 = _12.fld0.2 << _11;
_12.fld0.2 = -_7;
_13 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_8 = (-53282105491915924337238380660706609895_i128);
_15.2 = _1 | _1;
_2 = '\u{4083a}';
RET = _8 as f32;
_2 = '\u{f6418}';
_6 = 18091352645213221969_usize - 12193574875085873873_usize;
_2 = '\u{10683d}';
RET = _12.fld0.0 + _12.fld0.0;
_18 = _8 as i8;
_15.2 = 1760234179_u32 as u16;
_18 = _10;
_13 = 9223372036854775807_isize + (-9223372036854775808_isize);
_12.fld0.0 = RET * RET;
_18 = _10;
_14 = !true;
_17 = _12.fld0.0 as i128;
_19 = [_2];
_15.2 = _1 | _1;
_14 = !false;
RET = _4 as f32;
_20.2 = _8 as i64;
Goto(bb12)
}
bb12 = {
_10 = _18;
_8 = _17 & _17;
_16 = _2;
_21 = [_13,_13,_13];
_10 = _13 as i8;
RET = _4 as f32;
_21 = [_13,_13,_13];
_7 = _20.2 - _12.fld0.2;
RET = _12.fld0.0;
_24.fld5 = _8;
_9 = [_16,_16,_2,_16,_2,_16];
RET = -_12.fld0.0;
_20.2 = -_12.fld0.2;
_1 = _15.2;
_24.fld2.1 = _13 as f64;
RET = _12.fld0.0 * _12.fld0.0;
_3 = _11;
_23 = -_24.fld2.1;
Goto(bb13)
}
bb13 = {
_24.fld3 = (_16,);
match _5 {
0 => bb7,
1 => bb2,
2 => bb14,
25153 => bb16,
_ => bb15
}
}
bb14 = {
_10 = _18;
_8 = _17 & _17;
_16 = _2;
_21 = [_13,_13,_13];
_10 = _13 as i8;
RET = _4 as f32;
_21 = [_13,_13,_13];
_7 = _20.2 - _12.fld0.2;
RET = _12.fld0.0;
_24.fld5 = _8;
_9 = [_16,_16,_2,_16,_2,_16];
RET = -_12.fld0.0;
_20.2 = -_12.fld0.2;
_1 = _15.2;
_24.fld2.1 = _13 as f64;
RET = _12.fld0.0 * _12.fld0.0;
_3 = _11;
_23 = -_24.fld2.1;
Goto(bb13)
}
bb15 = {
_7 = _12.fld0.2 << _11;
_12.fld0.2 = -_7;
_13 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_8 = (-53282105491915924337238380660706609895_i128);
_15.2 = _1 | _1;
_2 = '\u{4083a}';
RET = _8 as f32;
_2 = '\u{f6418}';
_6 = 18091352645213221969_usize - 12193574875085873873_usize;
_2 = '\u{10683d}';
RET = _12.fld0.0 + _12.fld0.0;
_18 = _8 as i8;
_15.2 = 1760234179_u32 as u16;
_18 = _10;
_13 = 9223372036854775807_isize + (-9223372036854775808_isize);
_12.fld0.0 = RET * RET;
_18 = _10;
_14 = !true;
_17 = _12.fld0.0 as i128;
_19 = [_2];
_15.2 = _1 | _1;
_14 = !false;
RET = _4 as f32;
_20.2 = _8 as i64;
Goto(bb12)
}
bb16 = {
_24.fld2.0 = _6 as isize;
_24.fld0.2 = [_4,_4];
_12.fld1 = _9;
_11 = !_3;
_4 = 14150805423489067837_u64;
_15.1 = -_24.fld2.1;
_24.fld6 = _6;
_12.fld0.1 = [_1,_1,_15.2,_15.2,_15.2,_1];
_24.fld2.2 = _5 as u16;
_2 = _16;
_29 = _8 - _8;
_9 = _12.fld1;
_15 = (_13, _24.fld2.1, _24.fld2.2);
_24.fld2.1 = _23;
_24.fld0.0 = core::ptr::addr_of!(_5);
_30 = _29;
_23 = (-684924632_i32) as f64;
_20.1 = _12.fld0.1;
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(0_usize, 2_usize, Move(_2), 29_usize, Move(_29), 11_usize, Move(_11), 4_usize, Move(_4)), bb18, UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(0_usize, 30_usize, Move(_30), 18_usize, Move(_18), 13_usize, Move(_13), 14_usize, Move(_14)), bb19, UnwindUnreachable())
}
bb19 = {
Call(_31 = dump_var(0_usize, 19_usize, Move(_19), 17_usize, Move(_17), 32_usize, _32, 32_usize, _32), bb20, UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i64,mut _2: char,mut _3: char,mut _4: u64,mut _5: i64,mut _6: u64) -> [char; 6] {
mir! {
type RET = [char; 6];
let _7: [u64; 2];
let _8: Adt57;
let _9: [u32; 5];
let _10: *const i16;
let _11: bool;
let _12: char;
let _13: f32;
let _14: (f32, [u16; 6], i64);
let _15: (f64, [char; 6], u64, bool, f64);
let _16: usize;
let _17: f32;
let _18: isize;
let _19: isize;
let _20: isize;
let _21: [i32; 1];
let _22: [i16; 6];
let _23: ();
let _24: ();
{
_4 = _6;
_5 = -_1;
_6 = 31032411896228715652482278114457916068_i128 as u64;
_2 = _3;
RET = [_3,_3,_3,_2,_2,_3];
_1 = _5 ^ _5;
_7 = [_4,_4];
RET = [_3,_3,_2,_3,_2,_2];
_4 = !_6;
_2 = _3;
_1 = !_5;
_6 = 13362117826397592035_usize as u64;
_2 = _3;
_1 = _5;
_4 = 12530936485462162644_usize as u64;
_1 = _5;
_6 = !_4;
Goto(bb1)
}
bb1 = {
_6 = (-51_i8) as u64;
_6 = _4 | _4;
_9 = [1684882411_u32,295200845_u32,3825625696_u32,1252120886_u32,2122157283_u32];
_2 = _3;
_4 = _6 * _6;
_9 = [2699623700_u32,3426861779_u32,591722040_u32,3764041584_u32,1279993402_u32];
_3 = _2;
Call(_8 = fn2(_7, _3, _6, _6, _9, _3, _1, _1, _6), bb2, UnwindUnreachable())
}
bb2 = {
SetDiscriminant(_8, 0);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = !538649078_u32;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld4 = core::ptr::addr_of!(_4);
place!(Field::<i64>(Variant(_8, 0), 3)) = -_5;
_9 = [Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4];
_6 = _4 >> _4;
RET = [_3,_2,_2,_3,_3,_2];
place!(Field::<u64>(Variant(_8, 0), 4)) = _6 * _4;
_1 = _5 << _4;
_2 = _3;
place!(Field::<u32>(Variant(_8, 0), 1)) = !Field::<Adt51>(Variant(_8, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = !Field::<u32>(Variant(_8, 0), 1);
Goto(bb3)
}
bb3 = {
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.2 = 48611_u16;
_6 = Field::<u64>(Variant(_8, 0), 4) - Field::<u64>(Variant(_8, 0), 4);
_4 = Field::<u64>(Variant(_8, 0), 4);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.2 = (-460194630_i32) as u16;
_11 = !true;
place!(Field::<u64>(Variant(_8, 0), 4)) = _6;
place!(Field::<i64>(Variant(_8, 0), 3)) = _5;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld3.0 = _2;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.1 = 250393614_i32 as f64;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.2 = _7;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.3 = core::ptr::addr_of!(_5);
_9 = [Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<u32>(Variant(_8, 0), 1),Field::<u32>(Variant(_8, 0), 1),Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4];
_9 = [Field::<u32>(Variant(_8, 0), 1),Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<u32>(Variant(_8, 0), 1),Field::<u32>(Variant(_8, 0), 1)];
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld4 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_8, 0), 4)));
Goto(bb4)
}
bb4 = {
_14.2 = _1 - _5;
_2 = _3;
_5 = !_1;
_15.0 = Field::<Adt51>(Variant(_8, 0), 2).fld2.1 * Field::<Adt51>(Variant(_8, 0), 2).fld2.1;
_2 = _3;
RET = [_2,_2,_2,_3,_2,_2];
Goto(bb5)
}
bb5 = {
_14.1 = [Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2];
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld5 = 105559209818060008828798091721125426814_i128 << _6;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld6 = 4_usize | 1_usize;
_15 = (Field::<Adt51>(Variant(_8, 0), 2).fld2.1, RET, _4, _11, Field::<Adt51>(Variant(_8, 0), 2).fld2.1);
_15.0 = _15.4 - _15.4;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.1 = !_15.3;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.1 = _15.3;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = Field::<u32>(Variant(_8, 0), 1);
_7 = [Field::<u64>(Variant(_8, 0), 4),_6];
_18 = (-58_isize);
_12 = _3;
_4 = !Field::<u64>(Variant(_8, 0), 4);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.1 = -_15.0;
match _18 {
0 => bb6,
1 => bb7,
340282366920938463463374607431768211398 => bb9,
_ => bb8
}
}
bb6 = {
_6 = (-51_i8) as u64;
_6 = _4 | _4;
_9 = [1684882411_u32,295200845_u32,3825625696_u32,1252120886_u32,2122157283_u32];
_2 = _3;
_4 = _6 * _6;
_9 = [2699623700_u32,3426861779_u32,591722040_u32,3764041584_u32,1279993402_u32];
_3 = _2;
Call(_8 = fn2(_7, _3, _6, _6, _9, _3, _1, _1, _6), bb2, UnwindUnreachable())
}
bb7 = {
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.2 = 48611_u16;
_6 = Field::<u64>(Variant(_8, 0), 4) - Field::<u64>(Variant(_8, 0), 4);
_4 = Field::<u64>(Variant(_8, 0), 4);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.2 = (-460194630_i32) as u16;
_11 = !true;
place!(Field::<u64>(Variant(_8, 0), 4)) = _6;
place!(Field::<i64>(Variant(_8, 0), 3)) = _5;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld3.0 = _2;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.1 = 250393614_i32 as f64;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.2 = _7;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.3 = core::ptr::addr_of!(_5);
_9 = [Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<u32>(Variant(_8, 0), 1),Field::<u32>(Variant(_8, 0), 1),Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4];
_9 = [Field::<u32>(Variant(_8, 0), 1),Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<u32>(Variant(_8, 0), 1),Field::<u32>(Variant(_8, 0), 1)];
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld4 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_8, 0), 4)));
Goto(bb4)
}
bb8 = {
SetDiscriminant(_8, 0);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = !538649078_u32;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld4 = core::ptr::addr_of!(_4);
place!(Field::<i64>(Variant(_8, 0), 3)) = -_5;
_9 = [Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4];
_6 = _4 >> _4;
RET = [_3,_2,_2,_3,_3,_2];
place!(Field::<u64>(Variant(_8, 0), 4)) = _6 * _4;
_1 = _5 << _4;
_2 = _3;
place!(Field::<u32>(Variant(_8, 0), 1)) = !Field::<Adt51>(Variant(_8, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = !Field::<u32>(Variant(_8, 0), 1);
Goto(bb3)
}
bb9 = {
_16 = 20_i8 as usize;
_11 = _15.3;
_3 = _2;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.0 = _18;
_12 = _2;
_15.3 = _11;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.2 = [_4,Field::<u64>(Variant(_8, 0), 4)];
_2 = Field::<Adt51>(Variant(_8, 0), 2).fld3.0;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = !Field::<u32>(Variant(_8, 0), 1);
_12 = Field::<Adt51>(Variant(_8, 0), 2).fld3.0;
_13 = 25652_i16 as f32;
_16 = Field::<Adt51>(Variant(_8, 0), 2).fld6;
_14.0 = -_13;
RET = [_2,_2,_2,_2,_2,Field::<Adt51>(Variant(_8, 0), 2).fld3.0];
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld5 = 48991253251560854611584427447205809561_i128 ^ (-117091910627124638443342524642584260981_i128);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.1 = Field::<Adt51>(Variant(_8, 0), 2).fld0.4 as f64;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld4 = core::ptr::addr_of!(_4);
_5 = Field::<Adt51>(Variant(_8, 0), 2).fld5 as i64;
_14.2 = Field::<Adt51>(Variant(_8, 0), 2).fld3.0 as i64;
_17 = -_13;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.2 = 12155_u16;
place!(Field::<u64>(Variant(_8, 0), 4)) = _4;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2 = (_18, _15.0, 56320_u16);
_12 = Field::<Adt51>(Variant(_8, 0), 2).fld3.0;
_3 = Field::<Adt51>(Variant(_8, 0), 2).fld3.0;
_1 = !_14.2;
_19 = Field::<Adt51>(Variant(_8, 0), 2).fld2.0 & Field::<Adt51>(Variant(_8, 0), 2).fld2.0;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.1 = _15.4 + _15.4;
match _18 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463463374607431768211398 => bb15,
_ => bb14
}
}
bb10 = {
_6 = (-51_i8) as u64;
_6 = _4 | _4;
_9 = [1684882411_u32,295200845_u32,3825625696_u32,1252120886_u32,2122157283_u32];
_2 = _3;
_4 = _6 * _6;
_9 = [2699623700_u32,3426861779_u32,591722040_u32,3764041584_u32,1279993402_u32];
_3 = _2;
Call(_8 = fn2(_7, _3, _6, _6, _9, _3, _1, _1, _6), bb2, UnwindUnreachable())
}
bb11 = {
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.2 = 48611_u16;
_6 = Field::<u64>(Variant(_8, 0), 4) - Field::<u64>(Variant(_8, 0), 4);
_4 = Field::<u64>(Variant(_8, 0), 4);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.2 = (-460194630_i32) as u16;
_11 = !true;
place!(Field::<u64>(Variant(_8, 0), 4)) = _6;
place!(Field::<i64>(Variant(_8, 0), 3)) = _5;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld3.0 = _2;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.1 = 250393614_i32 as f64;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.2 = _7;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.3 = core::ptr::addr_of!(_5);
_9 = [Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<u32>(Variant(_8, 0), 1),Field::<u32>(Variant(_8, 0), 1),Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4];
_9 = [Field::<u32>(Variant(_8, 0), 1),Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<u32>(Variant(_8, 0), 1),Field::<u32>(Variant(_8, 0), 1)];
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld4 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_8, 0), 4)));
Goto(bb4)
}
bb12 = {
SetDiscriminant(_8, 0);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = !538649078_u32;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld4 = core::ptr::addr_of!(_4);
place!(Field::<i64>(Variant(_8, 0), 3)) = -_5;
_9 = [Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4,Field::<Adt51>(Variant(_8, 0), 2).fld0.4];
_6 = _4 >> _4;
RET = [_3,_2,_2,_3,_3,_2];
place!(Field::<u64>(Variant(_8, 0), 4)) = _6 * _4;
_1 = _5 << _4;
_2 = _3;
place!(Field::<u32>(Variant(_8, 0), 1)) = !Field::<Adt51>(Variant(_8, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = !Field::<u32>(Variant(_8, 0), 1);
Goto(bb3)
}
bb13 = {
_14.1 = [Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2,Field::<Adt51>(Variant(_8, 0), 2).fld2.2];
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld5 = 105559209818060008828798091721125426814_i128 << _6;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld6 = 4_usize | 1_usize;
_15 = (Field::<Adt51>(Variant(_8, 0), 2).fld2.1, RET, _4, _11, Field::<Adt51>(Variant(_8, 0), 2).fld2.1);
_15.0 = _15.4 - _15.4;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.1 = !_15.3;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.1 = _15.3;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.4 = Field::<u32>(Variant(_8, 0), 1);
_7 = [Field::<u64>(Variant(_8, 0), 4),_6];
_18 = (-58_isize);
_12 = _3;
_4 = !Field::<u64>(Variant(_8, 0), 4);
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2.1 = -_15.0;
match _18 {
0 => bb6,
1 => bb7,
340282366920938463463374607431768211398 => bb9,
_ => bb8
}
}
bb14 = {
_14.2 = _1 - _5;
_2 = _3;
_5 = !_1;
_15.0 = Field::<Adt51>(Variant(_8, 0), 2).fld2.1 * Field::<Adt51>(Variant(_8, 0), 2).fld2.1;
_2 = _3;
RET = [_2,_2,_2,_3,_2,_2];
Goto(bb5)
}
bb15 = {
_4 = (-848281087_i32) as u64;
RET = _15.1;
_20 = !_19;
_14.0 = _5 as f32;
_2 = _3;
RET = _15.1;
_15.2 = 67_i8 as u64;
_16 = Field::<Adt51>(Variant(_8, 0), 2).fld6 * Field::<Adt51>(Variant(_8, 0), 2).fld6;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2 = (_20, _15.0, 9884_u16);
_6 = _13 as u64;
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld0.3 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_8, 0), 3)));
place!(Field::<Adt51>(Variant(_8, 0), 2)).fld2 = (_19, _15.0, 22293_u16);
_15.3 = !_11;
_1 = _11 as i64;
_14.2 = _17 as i64;
_12 = _3;
RET = _15.1;
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(1_usize, 18_usize, Move(_18), 11_usize, Move(_11), 5_usize, Move(_5), 3_usize, Move(_3)), bb17, UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(1_usize, 1_usize, Move(_1), 4_usize, Move(_4), 7_usize, Move(_7), 24_usize, _24), bb18, UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: [u64; 2],mut _2: char,mut _3: u64,mut _4: u64,mut _5: [u32; 5],mut _6: char,mut _7: i64,mut _8: i64,mut _9: u64) -> Adt57 {
mir! {
type RET = Adt57;
let _10: i8;
let _11: char;
let _12: i16;
let _13: (f32, [u16; 6], i64);
let _14: [isize; 3];
let _15: [u32; 5];
let _16: char;
let _17: char;
let _18: [u64; 2];
let _19: f32;
let _20: *const *mut f64;
let _21: *mut u32;
let _22: u16;
let _23: i8;
let _24: Adt63;
let _25: isize;
let _26: [char; 6];
let _27: f32;
let _28: isize;
let _29: i8;
let _30: bool;
let _31: u128;
let _32: Adt62;
let _33: i8;
let _34: (isize, f64, u16);
let _35: u32;
let _36: f64;
let _37: Adt63;
let _38: Adt55;
let _39: f64;
let _40: *const (i128, (i128, i128, i8));
let _41: Adt63;
let _42: *const u64;
let _43: [i32; 1];
let _44: f64;
let _45: i16;
let _46: char;
let _47: (i8, isize, u32, f64);
let _48: Adt50;
let _49: *const i128;
let _50: char;
let _51: bool;
let _52: (i16,);
let _53: Adt61;
let _54: ([u64; 2], u128);
let _55: Adt53;
let _56: i16;
let _57: [isize; 8];
let _58: (*const i16, bool, [u64; 2], *const i64, u32);
let _59: [u8; 8];
let _60: [char; 5];
let _61: Adt62;
let _62: bool;
let _63: [char; 1];
let _64: [char; 6];
let _65: (i16,);
let _66: f64;
let _67: i128;
let _68: Adt55;
let _69: isize;
let _70: i16;
let _71: Adt61;
let _72: isize;
let _73: u64;
let _74: [i32; 8];
let _75: usize;
let _76: *mut u32;
let _77: (f64, [char; 6], u64, bool, f64);
let _78: i32;
let _79: *mut f64;
let _80: f64;
let _81: i64;
let _82: [isize; 8];
let _83: [i32; 1];
let _84: Adt61;
let _85: Adt55;
let _86: f64;
let _87: *const (i128, (i128, i128, i8));
let _88: char;
let _89: (i16,);
let _90: [u8; 8];
let _91: bool;
let _92: (char,);
let _93: [u64; 2];
let _94: bool;
let _95: (i16,);
let _96: (i128, (i128, i128, i8));
let _97: *const i16;
let _98: i128;
let _99: Adt54;
let _100: ();
let _101: ();
{
_1 = [_4,_9];
_1 = [_9,_9];
_4 = _9;
_1 = [_3,_3];
_6 = _2;
Goto(bb1)
}
bb1 = {
_7 = _8 ^ _8;
_4 = _9;
_5 = [716661919_u32,2329716271_u32,535706919_u32,4135258131_u32,3823579322_u32];
_2 = _6;
_2 = _6;
_3 = _4;
_1 = [_4,_3];
_11 = _6;
_13.2 = _7 + _7;
_13.1 = [29857_u16,32947_u16,27505_u16,62091_u16,63357_u16,35009_u16];
_4 = !_3;
_10 = (-10907_i16) as i8;
_13.0 = _4 as f32;
_11 = _6;
_12 = 60055_u16 as i16;
Call(RET = fn3(_9, _6, _9, _7), bb2, UnwindUnreachable())
}
bb2 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -93_isize;
_6 = _11;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = (-14_isize);
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
SetDiscriminant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !9017_u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.4, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<u64>(Variant(RET, 0), 4)) = !_3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 >> Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, 89669140486428790458689673514113377275_u128);
_10 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<u32>(Variant(RET, 0), 1);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, 6833_u16);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_3 = Field::<u64>(Variant(RET, 0), 4) << Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_3 = !_4;
Goto(bb3)
}
bb3 = {
_10 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
_18 = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).0;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = -Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
_11 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [_3,_9];
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [Field::<Adt51>(Variant(RET, 0), 2).fld2.0,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0 as u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
place!(Field::<i64>(Variant(RET, 0), 3)) = Field::<Adt51>(Variant(RET, 0), 2).fld2.2 as i64;
match Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
89669140486428790458689673514113377275 => bb8,
_ => bb7
}
}
bb4 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -93_isize;
_6 = _11;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = (-14_isize);
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
SetDiscriminant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !9017_u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.4, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<u64>(Variant(RET, 0), 4)) = !_3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 >> Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, 89669140486428790458689673514113377275_u128);
_10 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<u32>(Variant(RET, 0), 1);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, 6833_u16);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_3 = Field::<u64>(Variant(RET, 0), 4) << Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_3 = !_4;
Goto(bb3)
}
bb5 = {
_7 = _8 ^ _8;
_4 = _9;
_5 = [716661919_u32,2329716271_u32,535706919_u32,4135258131_u32,3823579322_u32];
_2 = _6;
_2 = _6;
_3 = _4;
_1 = [_4,_3];
_11 = _6;
_13.2 = _7 + _7;
_13.1 = [29857_u16,32947_u16,27505_u16,62091_u16,63357_u16,35009_u16];
_4 = !_3;
_10 = (-10907_i16) as i8;
_13.0 = _4 as f32;
_11 = _6;
_12 = 60055_u16 as i16;
Call(RET = fn3(_9, _6, _9, _7), bb2, UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [Field::<Adt51>(Variant(RET, 0), 2).fld2.0,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1];
_5 = [Field::<u32>(Variant(RET, 0), 1),Field::<u32>(Variant(RET, 0), 1),Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2,Field::<u32>(Variant(RET, 0), 1),Field::<Adt51>(Variant(RET, 0), 2).fld0.4];
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = _10;
_9 = Field::<u64>(Variant(RET, 0), 4) | Field::<u64>(Variant(RET, 0), 4);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = (-163614323203591562809767374524115994882_i128) | 113710118492020007913036743203258842593_i128;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld4 = core::ptr::addr_of!(_4);
_6 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld4 = core::ptr::addr_of!(_9);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = (-128609817144308926001301050027188598976_i128);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = 20413_u16 - 4298_u16;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = 11564_u16;
_13.2 = _2 as i64;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = Field::<Adt51>(Variant(RET, 0), 2).fld6 < Field::<Adt51>(Variant(RET, 0), 2).fld6;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
_22 = Field::<Adt51>(Variant(RET, 0), 2).fld2.2 | Field::<Adt51>(Variant(RET, 0), 2).fld2.2;
_15 = _5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld4 = core::ptr::addr_of!(_4);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = -(-145383071707834972864106371428507661367_i128);
Goto(bb9)
}
bb9 = {
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).0 = [_9,_4];
_12 = !25261_i16;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = false | false;
_22 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 as u16;
_21 = core::ptr::addr_of_mut!(place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = Field::<Adt51>(Variant(RET, 0), 2).fld2.1 * Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = !_10;
_23 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<u64>(Variant(RET, 0), 4)) = Field::<Adt51>(Variant(RET, 0), 2).fld0.1 as u64;
place!(Field::<u32>(Variant(RET, 0), 1)) = (*_21) * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = Field::<u32>(Variant(RET, 0), 1);
_26 = [_11,_11,Field::<Adt51>(Variant(RET, 0), 2).fld3.0,_6,_6,_2];
_11 = _2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = !Field::<Adt51>(Variant(RET, 0), 2).fld2.0;
place!(Field::<i64>(Variant(RET, 0), 3)) = _8 * _7;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = _6 as u32;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld6 = 14076189283540606592_usize & 1_usize;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = -Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
place!(Field::<u64>(Variant(RET, 0), 4)) = _12 as u64;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_18, 141568460909247596176065304809179881255_u128);
_10 = _23 >> Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
match Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 {
0 => bb5,
1 => bb4,
2 => bb3,
141568460909247596176065304809179881255 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_25 = (-626279333_i32) as isize;
_21 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(RET, 0), 1)));
(*_21) = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2 - Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_7);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
_26 = [Field::<Adt51>(Variant(RET, 0), 2).fld3.0,_6,_2,Field::<Adt51>(Variant(RET, 0), 2).fld3.0,_11,Field::<Adt51>(Variant(RET, 0), 2).fld3.0];
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).0 = [_4,_9];
_5 = _15;
_8 = Field::<Adt51>(Variant(RET, 0), 2).fld0.1 as i64;
_13.1 = [_22,_22,_22,_22,_22,_22];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = _18;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !147348173895839417276314872422936154233_u128;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).0 = [_3,_4];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld4 = core::ptr::addr_of!(_9);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = 107271830027005582402522639708715707730_u128;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = _10 >> Field::<Adt51>(Variant(RET, 0), 2).fld6;
_13.0 = Field::<u32>(Variant(RET, 0), 1) as f32;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = -Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
Goto(bb12)
}
bb12 = {
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_18, 183656092933466471813719541822393047990_u128);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = !(*_21);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
_17 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<u64>(Variant(RET, 0), 4)) = _8 as u64;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = _12 as u32;
_13.1 = [_22,_22,_22,_22,_22,_22];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_14 = [Field::<Adt51>(Variant(RET, 0), 2).fld2.0,Field::<Adt51>(Variant(RET, 0), 2).fld2.0,Field::<Adt51>(Variant(RET, 0), 2).fld2.0];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3 + Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld4 = core::ptr::addr_of!(place!(Field::<u64>(Variant(RET, 0), 4)));
_6 = _17;
Goto(bb13)
}
bb13 = {
_7 = Field::<Adt51>(Variant(RET, 0), 2).fld5 as i64;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = Field::<Adt51>(Variant(RET, 0), 2).fld2.0 + Field::<Adt51>(Variant(RET, 0), 2).fld2.0;
place!(Field::<i64>(Variant(RET, 0), 3)) = _8 >> Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
_11 = _2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0 as isize;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
match Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 {
0 => bb14,
183656092933466471813719541822393047990 => bb16,
_ => bb15
}
}
bb14 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -93_isize;
_6 = _11;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = (-14_isize);
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
SetDiscriminant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !9017_u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.4, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<u64>(Variant(RET, 0), 4)) = !_3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 >> Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, 89669140486428790458689673514113377275_u128);
_10 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<u32>(Variant(RET, 0), 1);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, 6833_u16);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_3 = Field::<u64>(Variant(RET, 0), 4) << Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_3 = !_4;
Goto(bb3)
}
bb15 = {
_7 = _8 ^ _8;
_4 = _9;
_5 = [716661919_u32,2329716271_u32,535706919_u32,4135258131_u32,3823579322_u32];
_2 = _6;
_2 = _6;
_3 = _4;
_1 = [_4,_3];
_11 = _6;
_13.2 = _7 + _7;
_13.1 = [29857_u16,32947_u16,27505_u16,62091_u16,63357_u16,35009_u16];
_4 = !_3;
_10 = (-10907_i16) as i8;
_13.0 = _4 as f32;
_11 = _6;
_12 = 60055_u16 as i16;
Call(RET = fn3(_9, _6, _9, _7), bb2, UnwindUnreachable())
}
bb16 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !_22;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld6 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0 as usize;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !130119862228447442354390200994651167096_u128;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !272669223657773107747665943118490246160_u128;
_27 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3 as f32;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [Field::<u64>(Variant(RET, 0), 4),_3];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, _22);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = _10;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_7);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = !_25;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [_4,_9];
_19 = -_13.0;
_34 = Field::<Adt51>(Variant(RET, 0), 2).fld2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = (*_21);
_31 = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).0 = _18;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = _34.1;
place!(Field::<u64>(Variant(RET, 0), 4)) = _9 << Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
_23 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0 * _10;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = _34;
_17 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
_12 = (-30372_i16) >> Field::<u32>(Variant(RET, 0), 1);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !_31;
_29 = !_10;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_17,);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_18, _31);
Goto(bb17)
}
bb17 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = (-12698513357788962144378058227272591026_i128) * (-150962384915797045644330797744424307919_i128);
_30 = !Field::<Adt51>(Variant(RET, 0), 2).fld0.1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [Field::<u64>(Variant(RET, 0), 4),Field::<u64>(Variant(RET, 0), 4)];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_2,);
_16 = _6;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_18, _31);
place!(Field::<u32>(Variant(RET, 0), 1)) = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = (*_21);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [Field::<u64>(Variant(RET, 0), 4),Field::<u64>(Variant(RET, 0), 4)];
_28 = !Field::<Adt51>(Variant(RET, 0), 2).fld2.0;
_18 = [Field::<u64>(Variant(RET, 0), 4),Field::<u64>(Variant(RET, 0), 4)];
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [_28,_25,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_8);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_16,);
place!(Field::<u64>(Variant(RET, 0), 4)) = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 as u64;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = _18;
place!(Field::<i64>(Variant(RET, 0), 3)) = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 as i64;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = _19 as u32;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = _30;
_33 = !_23;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = _30;
Goto(bb18)
}
bb18 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld6 = 5217830579173884258_usize;
_35 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<i64>(Variant(RET, 0), 3)) = _13.2 | _7;
_10 = _27 as i8;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = _12 as f64;
_9 = _4 | _4;
_23 = _33 | _33;
_1 = [_9,_9];
_29 = -Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_18, _31);
_34.1 = Field::<Adt51>(Variant(RET, 0), 2).fld2.1 + Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
_7 = _8;
_25 = _13.0 as isize;
_31 = _25 as u128;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).0 = [_9,_4];
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = _29 ^ _10;
_22 = _29 as u16;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !_34.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = _25;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = _23 << Field::<Adt51>(Variant(RET, 0), 2).fld2.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
_5 = [_35,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2,Field::<Adt51>(Variant(RET, 0), 2).fld0.4,Field::<u32>(Variant(RET, 0), 1)];
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [_25,Field::<Adt51>(Variant(RET, 0), 2).fld2.0,_25];
Goto(bb19)
}
bb19 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_7);
_19 = _13.0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = (*_21);
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb20,
1 => bb21,
2 => bb22,
5217830579173884258 => bb24,
_ => bb23
}
}
bb20 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld6 = 5217830579173884258_usize;
_35 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<i64>(Variant(RET, 0), 3)) = _13.2 | _7;
_10 = _27 as i8;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = _12 as f64;
_9 = _4 | _4;
_23 = _33 | _33;
_1 = [_9,_9];
_29 = -Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_18, _31);
_34.1 = Field::<Adt51>(Variant(RET, 0), 2).fld2.1 + Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
_7 = _8;
_25 = _13.0 as isize;
_31 = _25 as u128;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).0 = [_9,_4];
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = _29 ^ _10;
_22 = _29 as u16;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !_34.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = _25;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = _23 << Field::<Adt51>(Variant(RET, 0), 2).fld2.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
_5 = [_35,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2,Field::<Adt51>(Variant(RET, 0), 2).fld0.4,Field::<u32>(Variant(RET, 0), 1)];
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [_25,Field::<Adt51>(Variant(RET, 0), 2).fld2.0,_25];
Goto(bb19)
}
bb21 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -93_isize;
_6 = _11;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = (-14_isize);
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
SetDiscriminant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !9017_u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.4, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<u64>(Variant(RET, 0), 4)) = !_3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 >> Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, 89669140486428790458689673514113377275_u128);
_10 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<u32>(Variant(RET, 0), 1);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, 6833_u16);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_3 = Field::<u64>(Variant(RET, 0), 4) << Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_3 = !_4;
Goto(bb3)
}
bb22 = {
Return()
}
bb23 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -93_isize;
_6 = _11;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = (-14_isize);
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
SetDiscriminant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !9017_u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.4, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<u64>(Variant(RET, 0), 4)) = !_3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 >> Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, 89669140486428790458689673514113377275_u128);
_10 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<u32>(Variant(RET, 0), 1);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, 6833_u16);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_3 = Field::<u64>(Variant(RET, 0), 4) << Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_3 = !_4;
Goto(bb3)
}
bb24 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = _33 as i128;
_1 = Field::<Adt51>(Variant(RET, 0), 2).fld0.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (_25, _34.1, _22);
_38 = Adt55::Variant3 { fld0: Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3 };
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = (-1272220921_i32) as u32;
_22 = Field::<Adt51>(Variant(RET, 0), 2).fld2.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, _35, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
SetDiscriminant(_38, 0);
Goto(bb25)
}
bb25 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = -70237082612331421432185723922820956757_i128;
Goto(bb26)
}
bb26 = {
_13.2 = -_7;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
_29 = _33;
_28 = Field::<Adt51>(Variant(RET, 0), 2).fld2.0 >> _31;
_13.0 = _19 * _27;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.3 = Field::<Adt51>(Variant(RET, 0), 2).fld0.3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = _4 as isize;
_34.0 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld3.0 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
_8 = _16 as i64;
_25 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
_39 = Field::<Adt51>(Variant(RET, 0), 2).fld2.1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3;
_11 = _17;
_11 = _16;
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [_34.0,_34.0,_34.0];
_10 = _29;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(place!(Field::<(i16,)>(Variant(_38, 0), 4)).0);
place!(Field::<[u16; 6]>(Variant(_38, 0), 2)) = [Field::<Adt51>(Variant(RET, 0), 2).fld2.2,Field::<Adt51>(Variant(RET, 0), 2).fld2.2,Field::<Adt51>(Variant(RET, 0), 2).fld2.2,_22,Field::<Adt51>(Variant(RET, 0), 2).fld2.2,_22];
_43 = [(-2005895756_i32)];
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb7,
1 => bb23,
2 => bb3,
5217830579173884258 => bb28,
_ => bb27
}
}
bb27 = {
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).0 = [_9,_4];
_12 = !25261_i16;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = false | false;
_22 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 as u16;
_21 = core::ptr::addr_of_mut!(place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = Field::<Adt51>(Variant(RET, 0), 2).fld2.1 * Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = !_10;
_23 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<u64>(Variant(RET, 0), 4)) = Field::<Adt51>(Variant(RET, 0), 2).fld0.1 as u64;
place!(Field::<u32>(Variant(RET, 0), 1)) = (*_21) * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = Field::<u32>(Variant(RET, 0), 1);
_26 = [_11,_11,Field::<Adt51>(Variant(RET, 0), 2).fld3.0,_6,_6,_2];
_11 = _2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = !Field::<Adt51>(Variant(RET, 0), 2).fld2.0;
place!(Field::<i64>(Variant(RET, 0), 3)) = _8 * _7;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = _6 as u32;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld6 = 14076189283540606592_usize & 1_usize;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = -Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
place!(Field::<u64>(Variant(RET, 0), 4)) = _12 as u64;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_18, 141568460909247596176065304809179881255_u128);
_10 = _23 >> Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
match Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 {
0 => bb5,
1 => bb4,
2 => bb3,
141568460909247596176065304809179881255 => bb11,
_ => bb10
}
}
bb28 = {
_45 = _12 & _12;
_23 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2 as i8;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb1,
1 => bb12,
2 => bb21,
5217830579173884258 => bb29,
_ => bb16
}
}
bb29 = {
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld4 = core::ptr::addr_of!(_9);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
Goto(bb30)
}
bb30 = {
_35 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
_46 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.3 = core::ptr::addr_of!(_7);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = _39 as u32;
(*_21) = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2 ^ _35;
_34.0 = -Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.0 = core::ptr::addr_of!(_12);
_30 = _35 <= (*_21);
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1,_28];
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_13 = (_27, Field::<[u16; 6]>(Variant(_38, 0), 2), _8);
_42 = core::ptr::addr_of!(place!(Field::<u64>(Variant(RET, 0), 4)));
_23 = _29;
place!(Field::<u64>(Variant(RET, 0), 4)) = _3;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb25,
1 => bb20,
2 => bb29,
3 => bb17,
4 => bb31,
5 => bb32,
5217830579173884258 => bb34,
_ => bb33
}
}
bb31 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = -70237082612331421432185723922820956757_i128;
Goto(bb26)
}
bb32 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -93_isize;
_6 = _11;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = (-14_isize);
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
SetDiscriminant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !9017_u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.4, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<u64>(Variant(RET, 0), 4)) = !_3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 >> Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, 89669140486428790458689673514113377275_u128);
_10 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<u32>(Variant(RET, 0), 1);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, 6833_u16);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_3 = Field::<u64>(Variant(RET, 0), 4) << Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_3 = !_4;
Goto(bb3)
}
bb33 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_7);
_19 = _13.0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = (*_21);
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb20,
1 => bb21,
2 => bb22,
5217830579173884258 => bb24,
_ => bb23
}
}
bb34 = {
_47.3 = -Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3;
_21 = core::ptr::addr_of_mut!(place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2);
Goto(bb35)
}
bb35 = {
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.1 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3;
_21 = core::ptr::addr_of_mut!(place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2);
_48.fld0 = (_27, Field::<[u16; 6]>(Variant(_38, 0), 2), _7);
_47.1 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * _28;
_47.1 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.0 = core::ptr::addr_of!(_45);
_25 = _33 as isize;
_18 = Field::<Adt51>(Variant(RET, 0), 2).fld0.2;
_19 = 1021169345_i32 as f32;
_39 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3 + Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [_34.0,_28,_34.0];
place!(Field::<[u16; 6]>(Variant(_38, 0), 2)) = [_34.2,_22,Field::<Adt51>(Variant(RET, 0), 2).fld2.2,_34.2,Field::<Adt51>(Variant(RET, 0), 2).fld2.2,_22];
_31 = !Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
_47 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3);
_30 = Field::<Adt51>(Variant(RET, 0), 2).fld0.1;
Goto(bb36)
}
bb36 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3.0 = _16;
_33 = _10;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = _31;
_12 = -_45;
_17 = _46;
_47.3 = Field::<Adt51>(Variant(RET, 0), 2).fld6 as f64;
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [Field::<Adt51>(Variant(_38, 0), 1).fld2.0,_34.0,_34.0];
place!(Field::<u64>(Variant(RET, 0), 4)) = _3 ^ _9;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_23, _34.0, _47.2, Field::<Adt51>(Variant(_38, 0), 1).fld2.1);
_46 = _6;
_56 = _12 ^ _45;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _34.1 * _34.1;
_6 = _16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = (*_21) as isize;
place!(Field::<u32>(Variant(RET, 0), 1)) = _35 - _47.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = !_35;
_47.1 = !Field::<Adt51>(Variant(_38, 0), 1).fld2.0;
_47.1 = _34.0 - _28;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb30,
1 => bb21,
2 => bb22,
5217830579173884258 => bb37,
_ => bb4
}
}
bb37 = {
_48 = Adt50 { fld0: _13,fld1: _26 };
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !_31;
_4 = !(*_42);
_46 = _16;
_22 = _34.2 >> _35;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0 = (Field::<Adt51>(Variant(RET, 0), 2).fld0.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.1, Field::<Adt51>(Variant(RET, 0), 2).fld0.2, Field::<Adt51>(Variant(RET, 0), 2).fld0.3, (*_21));
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.0 = Field::<Adt51>(Variant(RET, 0), 2).fld0.0;
_6 = _46;
_54 = (_18, Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1);
_54.1 = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 | _31;
place!(Field::<*mut *const i128>(Variant(RET, 0), 0)) = core::ptr::addr_of_mut!(_49);
place!(Field::<[u16; 6]>(Variant(_38, 0), 2)) = [Field::<Adt51>(Variant(RET, 0), 2).fld2.2,_34.2,_34.2,_22,_22,_22];
_18 = [(*_42),Field::<u64>(Variant(RET, 0), 4)];
_58.1 = !_30;
(*_42) = !_9;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld3 = (_17,);
_5 = [Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2,(*_21),(*_21),_47.2,_47.2];
_6 = _17;
_8 = !_7;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb14,
1 => bb36,
2 => bb35,
3 => bb8,
4 => bb17,
5 => bb12,
5217830579173884258 => bb39,
_ => bb38
}
}
bb38 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !_22;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld6 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0 as usize;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !130119862228447442354390200994651167096_u128;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !272669223657773107747665943118490246160_u128;
_27 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3 as f32;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [Field::<u64>(Variant(RET, 0), 4),_3];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, _22);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).0 = _10;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_7);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = !_25;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [_4,_9];
_19 = -_13.0;
_34 = Field::<Adt51>(Variant(RET, 0), 2).fld2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = (*_21);
_31 = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).0 = _18;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = _34.1;
place!(Field::<u64>(Variant(RET, 0), 4)) = _9 << Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
_23 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0 * _10;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = _34;
_17 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
_12 = (-30372_i16) >> Field::<u32>(Variant(RET, 0), 1);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !_31;
_29 = !_10;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_17,);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_18, _31);
Goto(bb17)
}
bb39 = {
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_33, _34.0, _47.2, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i16,)>(Variant(_38, 0), 4)) = (_45,);
_51 = _58.1;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb33,
1 => bb24,
2 => bb8,
3 => bb28,
4 => bb40,
5 => bb41,
6 => bb42,
5217830579173884258 => bb44,
_ => bb43
}
}
bb40 = {
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld4 = core::ptr::addr_of!(_9);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
Goto(bb30)
}
bb41 = {
_48 = Adt50 { fld0: _13,fld1: _26 };
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = !_31;
_4 = !(*_42);
_46 = _16;
_22 = _34.2 >> _35;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0 = (Field::<Adt51>(Variant(RET, 0), 2).fld0.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.1, Field::<Adt51>(Variant(RET, 0), 2).fld0.2, Field::<Adt51>(Variant(RET, 0), 2).fld0.3, (*_21));
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.0 = Field::<Adt51>(Variant(RET, 0), 2).fld0.0;
_6 = _46;
_54 = (_18, Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1);
_54.1 = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 | _31;
place!(Field::<*mut *const i128>(Variant(RET, 0), 0)) = core::ptr::addr_of_mut!(_49);
place!(Field::<[u16; 6]>(Variant(_38, 0), 2)) = [Field::<Adt51>(Variant(RET, 0), 2).fld2.2,_34.2,_34.2,_22,_22,_22];
_18 = [(*_42),Field::<u64>(Variant(RET, 0), 4)];
_58.1 = !_30;
(*_42) = !_9;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld3 = (_17,);
_5 = [Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2,(*_21),(*_21),_47.2,_47.2];
_6 = _17;
_8 = !_7;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb14,
1 => bb36,
2 => bb35,
3 => bb8,
4 => bb17,
5 => bb12,
5217830579173884258 => bb39,
_ => bb38
}
}
bb42 = {
Return()
}
bb43 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = -70237082612331421432185723922820956757_i128;
Goto(bb26)
}
bb44 = {
place!(Field::<*mut *const i128>(Variant(RET, 0), 0)) = core::ptr::addr_of_mut!(_49);
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.4 = !_47.2;
_17 = Field::<Adt51>(Variant(_38, 0), 1).fld3.0;
place!(Field::<*mut *const i128>(Variant(RET, 0), 0)) = core::ptr::addr_of_mut!(_49);
_58.2 = [(*_42),Field::<u64>(Variant(RET, 0), 4)];
_48.fld0.1 = _13.1;
_28 = -_34.0;
_19 = _48.fld0.0 * _48.fld0.0;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = Field::<Adt51>(Variant(_38, 0), 1).fld0.0;
_58.3 = core::ptr::addr_of!(_7);
_54 = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, _31);
_58 = (Field::<Adt51>(Variant(_38, 0), 1).fld0.0, Field::<Adt51>(Variant(_38, 0), 1).fld0.1, _54.0, Field::<Adt51>(Variant(_38, 0), 1).fld0.3, Field::<Adt51>(Variant(_38, 0), 1).fld0.4);
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld5 = Field::<Adt51>(Variant(RET, 0), 2).fld5 & Field::<Adt51>(Variant(RET, 0), 2).fld5;
Goto(bb45)
}
bb45 = {
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.2 = _22;
_48.fld0.2 = _23 as i64;
place!(Field::<[u16; 6]>(Variant(_38, 0), 2)) = _48.fld0.1;
_47.0 = !_10;
_36 = Field::<Adt51>(Variant(_38, 0), 1).fld5 as f64;
_35 = Field::<Adt51>(Variant(_38, 0), 1).fld0.4 + _47.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_45);
(*_42) = _3 >> Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
_29 = _13.0 as i8;
place!(Field::<i64>(Variant(_38, 0), 0)) = 2095844698_i32 as i64;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<Adt51>(Variant(_38, 0), 1).fld2.0 * Field::<Adt51>(Variant(_38, 0), 1).fld2.0;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = _33 as u128;
_52.0 = _12 & _56;
(*_42) = _9;
Call(_58.0 = core::intrinsics::arith_offset(Field::<Adt51>(Variant(_38, 0), 1).fld0.0, (-9223372036854775808_isize)), bb46, UnwindUnreachable())
}
bb46 = {
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld5 = Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.2 = [_9,(*_42)];
_16 = _46;
_42 = core::ptr::addr_of!(_4);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = _51 & _58.1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
_10 = _23 >> _45;
_15 = [_58.4,(*_21),_35,Field::<Adt51>(Variant(_38, 0), 1).fld0.4,Field::<u32>(Variant(RET, 0), 1)];
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
_6 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<i64>(Variant(_38, 0), 0)) = !_48.fld0.2;
_12 = Field::<Adt51>(Variant(_38, 0), 1).fld2.2 as i16;
_66 = _39;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<Adt51>(Variant(_38, 0), 1).fld2.0, _34.1, Field::<Adt51>(Variant(_38, 0), 1).fld2.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = -_36;
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld6 = Field::<Adt51>(Variant(RET, 0), 2).fld2.2 as usize;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb42,
5217830579173884258 => bb48,
_ => bb47
}
}
bb47 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -93_isize;
_6 = _11;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = (-14_isize);
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
SetDiscriminant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !9017_u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.4, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<u64>(Variant(RET, 0), 4)) = !_3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 >> Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, 89669140486428790458689673514113377275_u128);
_10 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<u32>(Variant(RET, 0), 1);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, 6833_u16);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_3 = Field::<u64>(Variant(RET, 0), 4) << Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_3 = !_4;
Goto(bb3)
}
bb48 = {
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_58.2, _31);
_32 = Adt62::Variant2 { fld0: Field::<(i16,)>(Variant(_38, 0), 4) };
_60 = [_16,_16,_2,_46,Field::<Adt51>(Variant(RET, 0), 2).fld3.0];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = _47.1 >> _56;
SetDiscriminant(_32, 0);
_56 = _34.1 as i16;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_1, _31);
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = [_34.0,_34.0,_28];
(*_21) = !_58.4;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld5 = _39 as i128;
_69 = _66 as isize;
_33 = -_47.0;
_5 = [Field::<Adt51>(Variant(RET, 0), 2).fld0.4,(*_21),_58.4,_58.4,_47.2];
_62 = _34.2 != Field::<Adt51>(Variant(_38, 0), 1).fld2.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_45);
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.1 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3 * _39;
_70 = !_56;
_52 = (_12,);
_57 = [_69,Field::<Adt51>(Variant(RET, 0), 2).fld2.0,_25,Field::<Adt51>(Variant(RET, 0), 2).fld2.0,Field::<Adt51>(Variant(_38, 0), 1).fld2.0,_25,_28,_69];
Goto(bb49)
}
bb49 = {
_61 = Adt62::Variant2 { fld0: _52 };
_32 = Move(_61);
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.2 = [_4,Field::<u64>(Variant(RET, 0), 4)];
_21 = core::ptr::addr_of_mut!(_35);
_1 = [(*_42),(*_42)];
_71.fld0 = 161_u8 as u16;
_44 = _39;
_54.1 = _28 as u128;
_64 = _26;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_17,);
place!(Field::<u32>(Variant(RET, 0), 1)) = 1591722435_i32 as u32;
_1 = _54.0;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld5 = Field::<Adt51>(Variant(RET, 0), 2).fld5 << _58.4;
_35 = Field::<Adt51>(Variant(_38, 0), 1).fld0.4;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld4 = core::ptr::addr_of!(_73);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -_47.1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = _34.0 < _47.1;
_77.3 = !Field::<Adt51>(Variant(RET, 0), 2).fld0.1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld4 = _42;
_58.2 = _54.0;
place!(Field::<u32>(Variant(RET, 0), 1)) = Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb20,
1 => bb40,
5217830579173884258 => bb50,
_ => bb8
}
}
bb50 = {
SetDiscriminant(_32, 0);
_65 = _52;
Call(_72 = core::intrinsics::transmute(Field::<Adt51>(Variant(_38, 0), 1).fld2.0), bb51, UnwindUnreachable())
}
bb51 = {
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
5217830579173884258 => bb52,
_ => bb18
}
}
bb52 = {
place!(Field::<(i16,)>(Variant(_32, 0), 1)).0 = _52.0 << _25;
_31 = !_54.1;
_52 = (Field::<(i16,)>(Variant(_32, 0), 1).0,);
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb10,
1 => bb53,
2 => bb54,
5217830579173884258 => bb56,
_ => bb55
}
}
bb53 = {
_45 = _12 & _12;
_23 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2 as i8;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb1,
1 => bb12,
2 => bb21,
5217830579173884258 => bb29,
_ => bb16
}
}
bb54 = {
_7 = _8 ^ _8;
_4 = _9;
_5 = [716661919_u32,2329716271_u32,535706919_u32,4135258131_u32,3823579322_u32];
_2 = _6;
_2 = _6;
_3 = _4;
_1 = [_4,_3];
_11 = _6;
_13.2 = _7 + _7;
_13.1 = [29857_u16,32947_u16,27505_u16,62091_u16,63357_u16,35009_u16];
_4 = !_3;
_10 = (-10907_i16) as i8;
_13.0 = _4 as f32;
_11 = _6;
_12 = 60055_u16 as i16;
Call(RET = fn3(_9, _6, _9, _7), bb2, UnwindUnreachable())
}
bb55 = {
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld5 = Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.2 = [_9,(*_42)];
_16 = _46;
_42 = core::ptr::addr_of!(_4);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = _51 & _58.1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
_10 = _23 >> _45;
_15 = [_58.4,(*_21),_35,Field::<Adt51>(Variant(_38, 0), 1).fld0.4,Field::<u32>(Variant(RET, 0), 1)];
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
_6 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<i64>(Variant(_38, 0), 0)) = !_48.fld0.2;
_12 = Field::<Adt51>(Variant(_38, 0), 1).fld2.2 as i16;
_66 = _39;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<Adt51>(Variant(_38, 0), 1).fld2.0, _34.1, Field::<Adt51>(Variant(_38, 0), 1).fld2.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = -_36;
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld6 = Field::<Adt51>(Variant(RET, 0), 2).fld2.2 as usize;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb42,
5217830579173884258 => bb48,
_ => bb47
}
}
bb56 = {
_70 = _52.0 << Field::<Adt51>(Variant(RET, 0), 2).fld2.0;
place!(Field::<Adt55>(Variant(_32, 0), 2)) = Adt55::Variant3 { fld0: Field::<Adt51>(Variant(_38, 0), 1).fld2.1 };
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3.0 = _16;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = _48.fld0.0 as f64;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb31,
1 => bb57,
2 => bb58,
3 => bb59,
4 => bb60,
5 => bb61,
6 => bb62,
5217830579173884258 => bb64,
_ => bb63
}
}
bb57 = {
_7 = _8 ^ _8;
_4 = _9;
_5 = [716661919_u32,2329716271_u32,535706919_u32,4135258131_u32,3823579322_u32];
_2 = _6;
_2 = _6;
_3 = _4;
_1 = [_4,_3];
_11 = _6;
_13.2 = _7 + _7;
_13.1 = [29857_u16,32947_u16,27505_u16,62091_u16,63357_u16,35009_u16];
_4 = !_3;
_10 = (-10907_i16) as i8;
_13.0 = _4 as f32;
_11 = _6;
_12 = 60055_u16 as i16;
Call(RET = fn3(_9, _6, _9, _7), bb2, UnwindUnreachable())
}
bb58 = {
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.2 = _22;
_48.fld0.2 = _23 as i64;
place!(Field::<[u16; 6]>(Variant(_38, 0), 2)) = _48.fld0.1;
_47.0 = !_10;
_36 = Field::<Adt51>(Variant(_38, 0), 1).fld5 as f64;
_35 = Field::<Adt51>(Variant(_38, 0), 1).fld0.4 + _47.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_45);
(*_42) = _3 >> Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
_29 = _13.0 as i8;
place!(Field::<i64>(Variant(_38, 0), 0)) = 2095844698_i32 as i64;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<Adt51>(Variant(_38, 0), 1).fld2.0 * Field::<Adt51>(Variant(_38, 0), 1).fld2.0;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)).1 = _33 as u128;
_52.0 = _12 & _56;
(*_42) = _9;
Call(_58.0 = core::intrinsics::arith_offset(Field::<Adt51>(Variant(_38, 0), 1).fld0.0, (-9223372036854775808_isize)), bb46, UnwindUnreachable())
}
bb59 = {
_45 = _12 & _12;
_23 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2 as i8;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb1,
1 => bb12,
2 => bb21,
5217830579173884258 => bb29,
_ => bb16
}
}
bb60 = {
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld5 = Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.2 = [_9,(*_42)];
_16 = _46;
_42 = core::ptr::addr_of!(_4);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.1 = _51 & _58.1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
_10 = _23 >> _45;
_15 = [_58.4,(*_21),_35,Field::<Adt51>(Variant(_38, 0), 1).fld0.4,Field::<u32>(Variant(RET, 0), 1)];
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
_6 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<i64>(Variant(_38, 0), 0)) = !_48.fld0.2;
_12 = Field::<Adt51>(Variant(_38, 0), 1).fld2.2 as i16;
_66 = _39;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<Adt51>(Variant(_38, 0), 1).fld2.0, _34.1, Field::<Adt51>(Variant(_38, 0), 1).fld2.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = -_36;
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).2;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld6 = Field::<Adt51>(Variant(RET, 0), 2).fld2.2 as usize;
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb42,
5217830579173884258 => bb48,
_ => bb47
}
}
bb61 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = -93_isize;
_6 = _11;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = (-14_isize);
place!(Field::<u32>(Variant(RET, 0), 1)) = !Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
SetDiscriminant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.2 = !9017_u16;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<Adt51>(Variant(RET, 0), 2).fld0.4, Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<u64>(Variant(RET, 0), 4)) = !_3;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 >> Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = core::ptr::addr_of!(_12);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (Field::<Adt51>(Variant(RET, 0), 2).fld0.2, 89669140486428790458689673514113377275_u128);
_10 = !Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = (_10, Field::<Adt51>(Variant(RET, 0), 2).fld2.0, Field::<u32>(Variant(RET, 0), 1), Field::<Adt51>(Variant(RET, 0), 2).fld2.1);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<u32>(Variant(RET, 0), 1);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1, Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).3, 6833_u16);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<i64>(Variant(RET, 0), 3)) = !_13.2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = _13.0 as f64;
_3 = Field::<u64>(Variant(RET, 0), 4) << Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1 * Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_13.2);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_11,);
_3 = !_4;
Goto(bb3)
}
bb62 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.3 = core::ptr::addr_of!(_7);
_19 = _13.0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = (*_21);
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
0 => bb20,
1 => bb21,
2 => bb22,
5217830579173884258 => bb24,
_ => bb23
}
}
bb63 = {
_7 = Field::<Adt51>(Variant(RET, 0), 2).fld5 as i64;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = Field::<Adt51>(Variant(RET, 0), 2).fld2.0 + Field::<Adt51>(Variant(RET, 0), 2).fld2.0;
place!(Field::<i64>(Variant(RET, 0), 3)) = _8 >> Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0;
_11 = _2;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).1 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0 as isize;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld3 = (_6,);
match Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1 {
0 => bb14,
183656092933466471813719541822393047990 => bb16,
_ => bb15
}
}
bb64 = {
place!(Field::<i64>(Variant(_38, 0), 0)) = !_48.fld0.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [Field::<u64>(Variant(RET, 0), 4),Field::<u64>(Variant(RET, 0), 4)];
(*_21) = _48.fld0.0 as u32;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld0.2 = [_4,_4];
_60 = [Field::<Adt51>(Variant(_38, 0), 1).fld3.0,_11,_16,_11,_11];
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 1)) = (_54.0, _31);
(*_42) = Field::<u64>(Variant(RET, 0), 4) * _9;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).3 = -Field::<Adt51>(Variant(RET, 0), 2).fld2.1;
_56 = _72 as i16;
_28 = -Field::<Adt51>(Variant(RET, 0), 2).fld2.0;
_77.1 = [_16,_16,Field::<Adt51>(Variant(_38, 0), 1).fld3.0,_11,_16,Field::<Adt51>(Variant(RET, 0), 2).fld3.0];
place!(Field::<*const *mut f64>(Variant(_32, 0), 0)) = core::ptr::addr_of!(_79);
Goto(bb65)
}
bb65 = {
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (_25, _66, _22);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.2 = [(*_42),_4];
place!(Field::<i64>(Variant(RET, 0), 3)) = _72 as i64;
_6 = Field::<Adt51>(Variant(RET, 0), 2).fld3.0;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2 = (_69, _44, Field::<Adt51>(Variant(_38, 0), 1).fld2.2);
match Field::<Adt51>(Variant(RET, 0), 2).fld6 {
5217830579173884258 => bb66,
_ => bb52
}
}
bb66 = {
place!(Field::<Adt55>(Variant(_32, 0), 2)) = Adt55::Variant1 { fld0: Field::<Adt51>(Variant(_38, 0), 1).fld6,fld1: Field::<Adt51>(Variant(_38, 0), 1).fld0,fld2: _25,fld3: Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0,fld4: _54.1 };
SetDiscriminant(Field::<Adt55>(Variant(_32, 0), 2), 3);
Goto(bb67)
}
bb67 = {
_34.0 = -_69;
place!(Field::<*const *mut f64>(Variant(_32, 0), 0)) = core::ptr::addr_of!(_79);
Goto(bb68)
}
bb68 = {
_76 = _21;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.0 = _25 - _47.1;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld5 = Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<(i16,)>(Variant(_38, 0), 4)) = Field::<(i16,)>(Variant(_32, 0), 1);
_34.2 = Field::<Adt51>(Variant(_38, 0), 1).fld2.2;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld6 = Field::<Adt51>(Variant(_38, 0), 1).fld6 >> _47.1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.0 = _58.0;
_8 = _48.fld0.2;
place!(Field::<u64>(Variant(_38, 0), 3)) = Field::<u64>(Variant(RET, 0), 4) ^ (*_42);
_77 = (_34.1, _48.fld1, Field::<u64>(Variant(_38, 0), 3), _62, Field::<Adt51>(Variant(_38, 0), 1).fld2.1);
_33 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0 | _29;
place!(Field::<Adt51>(Variant(_38, 0), 1)).fld2.0 = _25;
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 0)) = _14;
Goto(bb69)
}
bb69 = {
place!(Field::<u16>(Variant(_32, 0), 3)) = Field::<Adt51>(Variant(_38, 0), 1).fld2.2;
_58.1 = _77.3;
place!(Field::<(i16,)>(Variant(_38, 0), 4)).0 = _54.1 as i16;
(*_76) = !_47.2;
_79 = core::ptr::addr_of_mut!(_34.1);
_34.2 = _22;
_82 = [Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).1,_28,_34.0,Field::<Adt51>(Variant(RET, 0), 2).fld2.0,Field::<Adt51>(Variant(_38, 0), 1).fld2.0,Field::<Adt51>(Variant(RET, 0), 2).fld2.0,_25,_69];
_74 = [(-1544045843_i32),1836885321_i32,1305448012_i32,(-1128958178_i32),245594212_i32,(-1641099068_i32),1200895595_i32,(-178018574_i32)];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld4 = core::ptr::addr_of!((*_42));
_27 = -_19;
_43 = [85904736_i32];
_58.4 = Field::<Adt51>(Variant(_38, 0), 1).fld2.0 as u32;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = !Field::<Adt51>(Variant(_38, 0), 1).fld5;
_22 = !Field::<Adt51>(Variant(_38, 0), 1).fld2.2;
(*_21) = !Field::<Adt51>(Variant(_38, 0), 1).fld0.4;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 3).0 as i128;
_48.fld0.0 = -_19;
_92.0 = _46;
Goto(bb70)
}
bb70 = {
_77.1 = _64;
_54.1 = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
Call(_57 = core::intrinsics::transmute(_82), bb71, UnwindUnreachable())
}
bb71 = {
_83 = [(-351107089_i32)];
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.0 = _34.0;
_74 = [(-1507672906_i32),2013310362_i32,269868118_i32,547901150_i32,87838309_i32,(-605429874_i32),1031092028_i32,(-1902117078_i32)];
_84.fld0 = _71.fld0 - Field::<u16>(Variant(_32, 0), 3);
_28 = _66 as isize;
_31 = Field::<([u64; 2], u128)>(Variant(Field::<Adt51>(Variant(RET, 0), 2).fld1, 0), 1).1;
_58.1 = _62;
_89.0 = (*_42) as i16;
_75 = Field::<Adt51>(Variant(RET, 0), 2).fld6 >> _58.4;
_81 = -Field::<i64>(Variant(_38, 0), 0);
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld0.4 = _35;
_87 = core::ptr::addr_of!(_96);
(*_79) = -_66;
_60 = [_16,_17,_92.0,_46,_2];
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)) = _47;
Goto(bb72)
}
bb72 = {
_65 = _52;
_13.1 = [Field::<u16>(Variant(_32, 0), 3),Field::<Adt51>(Variant(RET, 0), 2).fld2.2,_34.2,_84.fld0,_22,_84.fld0];
_26 = _48.fld1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld5 = Field::<Adt51>(Variant(_38, 0), 1).fld5 - Field::<Adt51>(Variant(_38, 0), 1).fld5;
_70 = Field::<(i16,)>(Variant(_38, 0), 4).0 ^ _52.0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 3)).2 = Field::<Adt51>(Variant(RET, 0), 2).fld0.4;
(*_87).0 = 53_u8 as i128;
_30 = _58.1;
place!(Field::<Adt51>(Variant(RET, 0), 2)).fld2.1 = -_77.4;
place!(Field::<(i16,)>(Variant(_38, 0), 4)) = (_12,);
_67 = Field::<Adt51>(Variant(RET, 0), 2).fld5 * Field::<Adt51>(Variant(RET, 0), 2).fld5;
place!(Field::<(i16,)>(Variant(_32, 0), 1)) = (_12,);
_48.fld0 = _13;
_64 = [_92.0,Field::<Adt51>(Variant(_38, 0), 1).fld3.0,_2,_11,_16,Field::<Adt51>(Variant(RET, 0), 2).fld3.0];
_89 = (_52.0,);
_97 = core::ptr::addr_of!(_52.0);
place!(Field::<*const (i128, (i128, i128, i8))>(Variant(place!(Field::<Adt51>(Variant(RET, 0), 2)).fld1, 0), 2)) = core::ptr::addr_of!((*_87));
_98 = -Field::<Adt51>(Variant(RET, 0), 2).fld5;
Goto(bb73)
}
bb73 = {
Call(_100 = dump_var(2_usize, 29_usize, Move(_29), 26_usize, Move(_26), 72_usize, Move(_72), 15_usize, Move(_15)), bb74, UnwindUnreachable())
}
bb74 = {
Call(_100 = dump_var(2_usize, 25_usize, Move(_25), 62_usize, Move(_62), 81_usize, Move(_81), 33_usize, Move(_33)), bb75, UnwindUnreachable())
}
bb75 = {
Call(_100 = dump_var(2_usize, 57_usize, Move(_57), 28_usize, Move(_28), 10_usize, Move(_10), 67_usize, Move(_67)), bb76, UnwindUnreachable())
}
bb76 = {
Call(_100 = dump_var(2_usize, 3_usize, Move(_3), 92_usize, Move(_92), 6_usize, Move(_6), 64_usize, Move(_64)), bb77, UnwindUnreachable())
}
bb77 = {
Call(_100 = dump_var(2_usize, 35_usize, Move(_35), 98_usize, Move(_98), 2_usize, Move(_2), 9_usize, Move(_9)), bb78, UnwindUnreachable())
}
bb78 = {
Call(_100 = dump_var(2_usize, 69_usize, Move(_69), 89_usize, Move(_89), 22_usize, Move(_22), 83_usize, Move(_83)), bb79, UnwindUnreachable())
}
bb79 = {
Call(_100 = dump_var(2_usize, 51_usize, Move(_51), 101_usize, _101, 101_usize, _101, 101_usize, _101), bb80, UnwindUnreachable())
}
bb80 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u64,mut _2: char,mut _3: u64,mut _4: i64) -> Adt57 {
mir! {
type RET = Adt57;
let _5: *const *mut f64;
let _6: bool;
let _7: [char; 5];
let _8: ([u64; 2], u128);
let _9: f64;
let _10: f32;
let _11: isize;
let _12: u32;
let _13: (i8, isize, u32, f64);
let _14: u8;
let _15: Adt49;
let _16: (isize, f64, u16);
let _17: i8;
let _18: [char; 1];
let _19: u32;
let _20: u128;
let _21: [i32; 8];
let _22: bool;
let _23: u16;
let _24: Adt58;
let _25: *mut u32;
let _26: *mut u32;
let _27: [char; 1];
let _28: Adt55;
let _29: Adt55;
let _30: [i32; 8];
let _31: *mut u32;
let _32: [i32; 1];
let _33: *const *mut f64;
let _34: char;
let _35: [u32; 5];
let _36: u16;
let _37: *const i16;
let _38: (f32, [u16; 6], i64);
let _39: [char; 1];
let _40: *const i128;
let _41: u32;
let _42: (f32, [u16; 6], i64);
let _43: isize;
let _44: [i32; 1];
let _45: Adt61;
let _46: (i16,);
let _47: Adt60;
let _48: (*const i16, bool, [u64; 2], *const i64, u32);
let _49: Adt58;
let _50: Adt62;
let _51: [i32; 1];
let _52: f32;
let _53: u16;
let _54: u8;
let _55: Adt54;
let _56: ([u64; 2], u128);
let _57: Adt54;
let _58: [isize; 3];
let _59: [isize; 3];
let _60: f32;
let _61: Adt59;
let _62: (char,);
let _63: f32;
let _64: char;
let _65: [u16; 6];
let _66: f32;
let _67: Adt62;
let _68: isize;
let _69: char;
let _70: (i8, isize, u32, f64);
let _71: (isize, f64, u16);
let _72: i16;
let _73: u32;
let _74: char;
let _75: [u16; 6];
let _76: *const *mut f64;
let _77: (f64, [char; 6], u64, bool, f64);
let _78: i64;
let _79: i8;
let _80: [isize; 3];
let _81: char;
let _82: i32;
let _83: char;
let _84: Adt58;
let _85: isize;
let _86: Adt56;
let _87: [i32; 8];
let _88: u64;
let _89: (i8, isize, u32, f64);
let _90: [isize; 3];
let _91: (i8, isize, u32, f64);
let _92: [u64; 2];
let _93: u32;
let _94: *mut f64;
let _95: *mut f64;
let _96: [u32; 5];
let _97: (i8, isize, u32, f64);
let _98: Adt60;
let _99: *mut f64;
let _100: bool;
let _101: i8;
let _102: (i128, (i128, i128, i8));
let _103: u32;
let _104: Adt48;
let _105: usize;
let _106: (i16,);
let _107: Adt64;
let _108: char;
let _109: (isize, f64, u16);
let _110: [u64; 2];
let _111: char;
let _112: *mut *const i128;
let _113: f32;
let _114: [char; 5];
let _115: (i8, isize, u32, f64);
let _116: [isize; 8];
let _117: [i16; 6];
let _118: *mut f64;
let _119: Adt50;
let _120: u64;
let _121: Adt57;
let _122: (i128, (i128, i128, i8));
let _123: (i16,);
let _124: isize;
let _125: [char; 5];
let _126: bool;
let _127: i32;
let _128: Adt64;
let _129: *const i16;
let _130: (char,);
let _131: u64;
let _132: *mut *const i128;
let _133: char;
let _134: Adt48;
let _135: char;
let _136: f64;
let _137: f64;
let _138: f64;
let _139: i128;
let _140: *mut u32;
let _141: Adt54;
let _142: char;
let _143: Adt55;
let _144: i128;
let _145: u128;
let _146: (i16,);
let _147: u32;
let _148: [char; 5];
let _149: Adt54;
let _150: Adt52;
let _151: Adt64;
let _152: i8;
let _153: [u32; 5];
let _154: Adt56;
let _155: u128;
let _156: f32;
let _157: i32;
let _158: isize;
let _159: [char; 1];
let _160: isize;
let _161: Adt51;
let _162: [u16; 6];
let _163: Adt58;
let _164: Adt61;
let _165: i32;
let _166: (char,);
let _167: (i16,);
let _168: *mut *const i128;
let _169: f64;
let _170: u8;
let _171: *const i128;
let _172: Adt50;
let _173: [i32; 8];
let _174: bool;
let _175: u32;
let _176: f32;
let _177: Adt53;
let _178: [u32; 5];
let _179: (*const i16, bool, [u64; 2], *const i64, u32);
let _180: Adt49;
let _181: f32;
let _182: f64;
let _183: (i16,);
let _184: Adt53;
let _185: (f64, [char; 6], u64, bool, f64);
let _186: i64;
let _187: [i32; 1];
let _188: Adt62;
let _189: (i128, i128, i8);
let _190: u128;
let _191: [char; 1];
let _192: f64;
let _193: u64;
let _194: isize;
let _195: u8;
let _196: isize;
let _197: i128;
let _198: ([u64; 2], u128);
let _199: Adt60;
let _200: (*const i16, bool, [u64; 2], *const i64, u32);
let _201: isize;
let _202: f64;
let _203: [i16; 6];
let _204: u32;
let _205: (i128, (i128, i128, i8));
let _206: Adt50;
let _207: *const i128;
let _208: isize;
let _209: [isize; 3];
let _210: (f64, [char; 6], u64, bool, f64);
let _211: isize;
let _212: f32;
let _213: bool;
let _214: f32;
let _215: *const (i128, (i128, i128, i8));
let _216: Adt60;
let _217: [char; 5];
let _218: u32;
let _219: (f64, [char; 6], u64, bool, f64);
let _220: [isize; 8];
let _221: Adt50;
let _222: [char; 6];
let _223: [char; 1];
let _224: (char,);
let _225: i16;
let _226: [u32; 5];
let _227: isize;
let _228: Adt64;
let _229: isize;
let _230: isize;
let _231: (f64, [char; 6], u64, bool, f64);
let _232: char;
let _233: *const i128;
let _234: [i16; 6];
let _235: isize;
let _236: (i8, isize, u32, f64);
let _237: (char,);
let _238: u64;
let _239: bool;
let _240: [u64; 2];
let _241: [i32; 1];
let _242: isize;
let _243: (i16,);
let _244: char;
let _245: bool;
let _246: *const i128;
let _247: f32;
let _248: f32;
let _249: isize;
let _250: (char,);
let _251: [i16; 6];
let _252: *mut u32;
let _253: f32;
let _254: isize;
let _255: [isize; 8];
let _256: bool;
let _257: Adt60;
let _258: Adt55;
let _259: Adt52;
let _260: isize;
let _261: *const i16;
let _262: f32;
let _263: Adt55;
let _264: bool;
let _265: f64;
let _266: i32;
let _267: *const i16;
let _268: [u32; 5];
let _269: [char; 6];
let _270: [u32; 5];
let _271: Adt54;
let _272: (i128, (i128, i128, i8));
let _273: u8;
let _274: u16;
let _275: bool;
let _276: (isize, f64, u16);
let _277: [char; 5];
let _278: bool;
let _279: [u8; 8];
let _280: [isize; 3];
let _281: bool;
let _282: [u64; 2];
let _283: *const i16;
let _284: (f64, [char; 6], u64, bool, f64);
let _285: [i32; 1];
let _286: [i16; 6];
let _287: u32;
let _288: bool;
let _289: [u32; 5];
let _290: char;
let _291: isize;
let _292: (i128, (i128, i128, i8));
let _293: ([u64; 2], u128);
let _294: u16;
let _295: [i32; 1];
let _296: [char; 1];
let _297: (i16,);
let _298: bool;
let _299: Adt63;
let _300: [i16; 6];
let _301: isize;
let _302: isize;
let _303: *const i128;
let _304: f32;
let _305: isize;
let _306: bool;
let _307: [u64; 2];
let _308: bool;
let _309: isize;
let _310: [i16; 6];
let _311: [i32; 8];
let _312: Adt57;
let _313: bool;
let _314: [char; 5];
let _315: (i128, (i128, i128, i8));
let _316: i32;
let _317: isize;
let _318: *const i16;
let _319: isize;
let _320: u32;
let _321: bool;
let _322: isize;
let _323: *mut *const i128;
let _324: Adt58;
let _325: Adt60;
let _326: *const *mut f64;
let _327: Adt59;
let _328: u128;
let _329: isize;
let _330: [char; 6];
let _331: (i8, isize, u32, f64);
let _332: u32;
let _333: i32;
let _334: f64;
let _335: ([u64; 2], u128);
let _336: *const i128;
let _337: u128;
let _338: u8;
let _339: Adt52;
let _340: f32;
let _341: (i8, isize, u32, f64);
let _342: u16;
let _343: (i128, (i128, i128, i8));
let _344: Adt64;
let _345: Adt54;
let _346: [u16; 6];
let _347: (isize, f64, u16);
let _348: f32;
let _349: char;
let _350: *const (i128, (i128, i128, i8));
let _351: bool;
let _352: [i16; 6];
let _353: u8;
let _354: [char; 1];
let _355: f64;
let _356: char;
let _357: f64;
let _358: (i128, i128, i8);
let _359: (i128, i128, i8);
let _360: i128;
let _361: [u8; 8];
let _362: char;
let _363: char;
let _364: [u32; 5];
let _365: f64;
let _366: f64;
let _367: (char,);
let _368: *const u64;
let _369: f32;
let _370: (char,);
let _371: bool;
let _372: f32;
let _373: Adt63;
let _374: Adt51;
let _375: *const (i128, (i128, i128, i8));
let _376: (f32, [u16; 6], i64);
let _377: (f32, [u16; 6], i64);
let _378: char;
let _379: usize;
let _380: Adt63;
let _381: (i16,);
let _382: *const u64;
let _383: char;
let _384: u32;
let _385: [u8; 8];
let _386: f32;
let _387: bool;
let _388: isize;
let _389: isize;
let _390: *const i16;
let _391: f32;
let _392: usize;
let _393: u64;
let _394: (i128, (i128, i128, i8));
let _395: u8;
let _396: isize;
let _397: (f64, [char; 6], u64, bool, f64);
let _398: u32;
let _399: usize;
let _400: *mut f64;
let _401: (i128, i128, i8);
let _402: (f32, [u16; 6], i64);
let _403: char;
let _404: (*const i16, bool, [u64; 2], *const i64, u32);
let _405: (i128, i128, i8);
let _406: (*const i16, bool, [u64; 2], *const i64, u32);
let _407: (i16,);
let _408: (isize, f64, u16);
let _409: ([u64; 2], u128);
let _410: (i128, i128, i8);
let _411: *const i16;
let _412: ([u64; 2], u128);
let _413: (*const i16, bool, [u64; 2], *const i64, u32);
let _414: [u16; 6];
let _415: isize;
let _416: [isize; 3];
let _417: Adt50;
let _418: char;
let _419: [i32; 1];
let _420: u8;
let _421: f64;
let _422: f32;
let _423: [isize; 8];
let _424: (i8, isize, u32, f64);
let _425: Adt55;
let _426: ();
let _427: ();
{
_3 = 107_isize as u64;
_4 = -(-3267256975262236251_i64);
_2 = '\u{b3a90}';
_3 = _2 as u64;
_2 = '\u{26d1}';
_2 = '\u{f83d9}';
_1 = _3;
_1 = _3;
_6 = true;
_6 = _1 < _1;
_2 = '\u{90e50}';
_8.1 = !124661216166713763623105975737169793493_u128;
_1 = _3 * _3;
_4 = (-4831080190839808823_i64);
_6 = !true;
_8.1 = !38196587075449186213674053699224817191_u128;
_8.1 = !223976037777144286769384051756585848323_u128;
_2 = '\u{813d9}';
_7 = [_2,_2,_2,_2,_2];
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463458543527240928402633 => bb6,
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
_8.0 = [_1,_3];
_3 = !_1;
_7 = [_2,_2,_2,_2,_2];
_3 = _1 ^ _1;
_6 = !false;
_7 = [_2,_2,_2,_2,_2];
_9 = 25_u8 as f64;
_10 = (-9223372036854775808_isize) as f32;
_1 = 10874744360208718038_usize as u64;
_3 = _1 | _1;
_3 = (-51_isize) as u64;
_12 = 13_i8 as u32;
Goto(bb7)
}
bb7 = {
_3 = !_1;
_4 = -9090666585329364385_i64;
_10 = (-17902_i16) as f32;
_1 = !_3;
_4 = (-7598130229829855051_i64) + 3334452980523051957_i64;
_12 = 509903627_u32 ^ 2187363010_u32;
_8.1 = 188744444220393779528356791527339203973_u128 & 110302095921986413826214468588179530122_u128;
_6 = _2 > _2;
_11 = (-40_isize) & 17_isize;
_13 = (68_i8, _11, _12, _9);
_4 = -6184436884952208360_i64;
Goto(bb8)
}
bb8 = {
_1 = _3;
_13.0 = 54_i8 + 114_i8;
_3 = !_1;
_12 = _13.3 as u32;
_11 = _13.1 - _13.1;
_13.1 = 30_u8 as isize;
_13.3 = -_9;
_6 = false;
_12 = _11 as u32;
_2 = '\u{d00c}';
_1 = !_3;
_11 = !_13.1;
_7 = [_2,_2,_2,_2,_2];
_13 = ((-71_i8), _11, _12, _9);
_8.0 = [_3,_3];
_11 = _13.1 << _1;
_8.0 = [_3,_3];
_13.3 = -_9;
Call(_10 = fn4(_13.0, _1, _13, _13.0, _13.0), bb9, UnwindUnreachable())
}
bb9 = {
_13.0 = 120_i8;
_14 = 159_u8;
_2 = '\u{ada71}';
_7 = [_2,_2,_2,_2,_2];
_10 = 5_usize as f32;
_6 = !true;
_1 = _8.1 as u64;
_13.2 = _12 | _12;
_13.2 = _12;
_13.0 = (-158162110141614854351871548856830290461_i128) as i8;
_1 = _3 - _3;
_16 = (_13.1, _13.3, 7265_u16);
_16 = (_11, _13.3, 38169_u16);
_16.0 = _11 & _13.1;
_16 = (_11, _9, 8849_u16);
_12 = !_13.2;
_7 = [_2,_2,_2,_2,_2];
_8.1 = 74193432930989793646309532060057913349_u128 & 278007604218750838367299406576067771857_u128;
_3 = _1;
_13 = (95_i8, _11, _12, _16.1);
_14 = 145_u8 << _16.2;
_4 = !(-1464957664605104163_i64);
_13.2 = !_12;
Call(_13.3 = fn9(_13.0, _7, _11, _7, _13.0), bb10, UnwindUnreachable())
}
bb10 = {
_8.1 = 51140791828656654071765554938944768454_u128;
_4 = !4800063455210199834_i64;
_16 = (_13.1, _13.3, 11860_u16);
_16 = (_13.1, _13.3, 20301_u16);
_16.1 = -_13.3;
_21 = [(-1321322609_i32),(-51615678_i32),(-1140722024_i32),1468137865_i32,554021606_i32,176613358_i32,1956596032_i32,108388148_i32];
_12 = _13.2 & _13.2;
_18 = [_2];
_16.1 = _13.3 + _13.3;
_13.0 = _16.1 as i8;
_7 = [_2,_2,_2,_2,_2];
_16.2 = 16873_u16;
_12 = _16.0 as u32;
_9 = _4 as f64;
_19 = !_13.2;
_20 = !_8.1;
_20 = _6 as u128;
_21 = [1684381928_i32,(-36207625_i32),1370526671_i32,(-903837733_i32),1097096883_i32,1473716572_i32,(-1348085056_i32),(-1352862003_i32)];
_21 = [101356621_i32,985021183_i32,1291869952_i32,912349415_i32,(-1321975743_i32),(-1215861914_i32),(-1308721571_i32),(-759033814_i32)];
_21 = [222181840_i32,689186776_i32,1554140022_i32,(-202646674_i32),(-302136676_i32),553728735_i32,(-176323168_i32),(-1558313304_i32)];
_20 = !_8.1;
Goto(bb11)
}
bb11 = {
_25 = core::ptr::addr_of_mut!(_19);
_2 = '\u{3587c}';
_26 = core::ptr::addr_of_mut!(_19);
(*_26) = _13.2;
_13.0 = _16.2 as i8;
_6 = !false;
_13.0 = 25_i8 * (-92_i8);
_14 = 9_u8;
_20 = _8.1 << (*_25);
_22 = _6 & _6;
_7 = [_2,_2,_2,_2,_2];
_2 = '\u{3043f}';
_2 = '\u{abe40}';
_14 = _22 as u8;
_14 = 192_u8;
_23 = 1885312917_i32 as u16;
_13.1 = (-1475692077_i32) as isize;
(*_26) = _13.2;
_13.2 = (*_25);
_17 = _13.0 >> _13.2;
_8.1 = _20;
_18 = [_2];
Call(_10 = fn11(_25, _16.1, _13, _16, _13.3, _16, _16), bb12, UnwindUnreachable())
}
bb12 = {
_8.0 = [_1,_1];
_4 = !(-2190387432024114632_i64);
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = _13.2 << _14;
_23 = _16.2;
_17 = _13.0;
(*_25) = _12 ^ _13.2;
_13.2 = 24252_i16 as u32;
_23 = _16.2;
_9 = _16.1;
_8.0 = [_1,_3];
_13 = (_17, _11, (*_25), _16.1);
_17 = _13.0;
_20 = _8.1 + _8.1;
_30 = [181864928_i32,520152660_i32,242294800_i32,827454233_i32,755086660_i32,(-757562413_i32),1062542704_i32,941045019_i32];
_9 = _16.1 - _16.1;
_26 = core::ptr::addr_of_mut!(_19);
_13.3 = _16.2 as f64;
_2 = '\u{ea62b}';
_13.3 = _16.1 * _9;
_30 = [1663167037_i32,193661406_i32,858725521_i32,(-1774873767_i32),1318859460_i32,1054568243_i32,871689246_i32,257317675_i32];
_20 = _8.1 ^ _8.1;
_7 = [_2,_2,_2,_2,_2];
_13 = (_17, _16.0, (*_26), _9);
_31 = core::ptr::addr_of_mut!((*_25));
(*_25) = !_13.2;
match _23 {
0 => bb11,
1 => bb2,
2 => bb7,
16873 => bb14,
_ => bb13
}
}
bb13 = {
_25 = core::ptr::addr_of_mut!(_19);
_2 = '\u{3587c}';
_26 = core::ptr::addr_of_mut!(_19);
(*_26) = _13.2;
_13.0 = _16.2 as i8;
_6 = !false;
_13.0 = 25_i8 * (-92_i8);
_14 = 9_u8;
_20 = _8.1 << (*_25);
_22 = _6 & _6;
_7 = [_2,_2,_2,_2,_2];
_2 = '\u{3043f}';
_2 = '\u{abe40}';
_14 = _22 as u8;
_14 = 192_u8;
_23 = 1885312917_i32 as u16;
_13.1 = (-1475692077_i32) as isize;
(*_26) = _13.2;
_13.2 = (*_25);
_17 = _13.0 >> _13.2;
_8.1 = _20;
_18 = [_2];
Call(_10 = fn11(_25, _16.1, _13, _16, _13.3, _16, _16), bb12, UnwindUnreachable())
}
bb14 = {
_13.1 = _10 as isize;
_12 = (*_26);
_4 = (-5702781883970881890_i64);
_16 = (_13.1, _9, _23);
(*_25) = _14 as u32;
_8.1 = !_20;
_13.2 = (*_25);
_28 = Adt55::Variant3 { fld0: _13.3 };
_16.2 = _23 | _23;
_13 = (_17, _16.0, (*_25), _9);
_7 = [_2,_2,_2,_2,_2];
(*_31) = _12;
_31 = core::ptr::addr_of_mut!(_12);
_1 = _3 >> _13.1;
_13.3 = 798195387_i32 as f64;
(*_31) = (*_25);
_23 = _16.2 - _16.2;
_13 = (_17, _16.0, (*_31), Field::<f64>(Variant(_28, 3), 0));
match _14 {
0 => bb15,
1 => bb16,
192 => bb18,
_ => bb17
}
}
bb15 = {
Return()
}
bb16 = {
_3 = !_1;
_4 = -9090666585329364385_i64;
_10 = (-17902_i16) as f32;
_1 = !_3;
_4 = (-7598130229829855051_i64) + 3334452980523051957_i64;
_12 = 509903627_u32 ^ 2187363010_u32;
_8.1 = 188744444220393779528356791527339203973_u128 & 110302095921986413826214468588179530122_u128;
_6 = _2 > _2;
_11 = (-40_isize) & 17_isize;
_13 = (68_i8, _11, _12, _9);
_4 = -6184436884952208360_i64;
Goto(bb8)
}
bb17 = {
Return()
}
bb18 = {
_8.1 = _20;
_29 = Move(_28);
_30 = [126082057_i32,(-504640165_i32),1694032903_i32,(-1914353090_i32),(-1602620178_i32),386611712_i32,(-1926126165_i32),(-385351424_i32)];
_31 = _26;
_36 = _2 as u16;
_39 = [_2];
place!(Field::<f64>(Variant(_29, 3), 0)) = -_16.1;
(*_25) = _4 as u32;
_34 = _2;
_4 = (-8368292663405945841_i64);
_2 = _34;
_38.1 = [_36,_23,_23,_23,_23,_23];
_13.1 = _16.0 << _8.1;
_16.2 = !_23;
_42.2 = _8.1 as i64;
_9 = 8528696503175167757_usize as f64;
_41 = !_19;
_43 = _22 as isize;
_42 = (_10, _38.1, _4);
_44 = [1484222084_i32];
_16.0 = !_13.1;
Goto(bb19)
}
bb19 = {
_42.1 = [_16.2,_23,_16.2,_16.2,_23,_16.2];
(*_31) = _42.2 as u32;
_14 = 117_u8 + 29_u8;
_38.0 = _10 * _42.0;
_36 = _2 as u16;
_38.0 = -_10;
_46 = (6023_i16,);
_41 = _4 as u32;
_13.1 = _16.0;
_38.1 = _42.1;
_31 = _25;
_16.2 = _36 << _13.1;
_2 = _34;
_38.2 = _42.2 & _42.2;
_45.fld0 = _16.2 | _16.2;
_48.0 = core::ptr::addr_of!(_46.0);
_44 = [617245479_i32];
_32 = [1600290840_i32];
_43 = _22 as isize;
match _4 {
0 => bb20,
1 => bb21,
2 => bb22,
340282366920938463455006314768362265615 => bb24,
_ => bb23
}
}
bb20 = {
Return()
}
bb21 = {
_13.0 = 120_i8;
_14 = 159_u8;
_2 = '\u{ada71}';
_7 = [_2,_2,_2,_2,_2];
_10 = 5_usize as f32;
_6 = !true;
_1 = _8.1 as u64;
_13.2 = _12 | _12;
_13.2 = _12;
_13.0 = (-158162110141614854351871548856830290461_i128) as i8;
_1 = _3 - _3;
_16 = (_13.1, _13.3, 7265_u16);
_16 = (_11, _13.3, 38169_u16);
_16.0 = _11 & _13.1;
_16 = (_11, _9, 8849_u16);
_12 = !_13.2;
_7 = [_2,_2,_2,_2,_2];
_8.1 = 74193432930989793646309532060057913349_u128 & 278007604218750838367299406576067771857_u128;
_3 = _1;
_13 = (95_i8, _11, _12, _16.1);
_14 = 145_u8 << _16.2;
_4 = !(-1464957664605104163_i64);
_13.2 = !_12;
Call(_13.3 = fn9(_13.0, _7, _11, _7, _13.0), bb10, UnwindUnreachable())
}
bb22 = {
_1 = _3;
_13.0 = 54_i8 + 114_i8;
_3 = !_1;
_12 = _13.3 as u32;
_11 = _13.1 - _13.1;
_13.1 = 30_u8 as isize;
_13.3 = -_9;
_6 = false;
_12 = _11 as u32;
_2 = '\u{d00c}';
_1 = !_3;
_11 = !_13.1;
_7 = [_2,_2,_2,_2,_2];
_13 = ((-71_i8), _11, _12, _9);
_8.0 = [_3,_3];
_11 = _13.1 << _1;
_8.0 = [_3,_3];
_13.3 = -_9;
Call(_10 = fn4(_13.0, _1, _13, _13.0, _13.0), bb9, UnwindUnreachable())
}
bb23 = {
Return()
}
bb24 = {
_51 = [1916050570_i32];
_8.1 = !_20;
_13.2 = (*_31);
_37 = core::ptr::addr_of!(_46.0);
(*_31) = _12 - _12;
Call(_19 = core::intrinsics::transmute(_34), bb25, UnwindUnreachable())
}
bb25 = {
_48.3 = core::ptr::addr_of!(_38.2);
(*_26) = !_12;
_54 = _12 as u8;
SetDiscriminant(_29, 2);
_14 = _43 as u8;
_34 = _2;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld1 = [_34,_34,_2,_2,_2,_34];
_27 = [_2];
_48.0 = _37;
_16.2 = 1372543162_i32 as u16;
_7 = [_2,_34,_34,_2,_34];
place!(Field::<[i16; 6]>(Variant(_29, 2), 2)) = [_46.0,_46.0,_46.0,(*_37),_46.0,_46.0];
_53 = !_23;
_17 = _13.0;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0 = (_10, _38.1, _42.2);
_7 = [_34,_2,_34,_2,_2];
_13.1 = -_16.0;
_7 = [_2,_34,_34,_2,_2];
_13.2 = (*_25);
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0 = (_42.0, _42.1, _4);
_17 = _13.0 >> _8.1;
_42.1 = Field::<Adt50>(Variant(_29, 2), 0).fld0.1;
Call(_13.2 = fn13(_45.fld0, _38.0, _16.0, Field::<Adt50>(Variant(_29, 2), 0).fld0.0, _16.1, _13.1, _13.3, _16.0, Field::<Adt50>(Variant(_29, 2), 0).fld0, _38.1, Field::<Adt50>(Variant(_29, 2), 0).fld0.0, _13.1, _42.0, Field::<Adt50>(Variant(_29, 2), 0).fld0.0, _16), bb26, UnwindUnreachable())
}
bb26 = {
_35 = [(*_26),(*_25),_19,(*_26),_19];
_2 = _34;
(*_37) = _13.1 as i16;
_3 = !_1;
_21 = [(-1634728754_i32),(-1643363918_i32),1583172544_i32,(-561303879_i32),1756385197_i32,1566611304_i32,1480437606_i32,485755229_i32];
(*_25) = 301860589_i32 as u32;
_45.fld0 = _36 << _13.1;
_11 = 65769864974959250833081813579850657343_i128 as isize;
_25 = core::ptr::addr_of_mut!(_13.2);
Call(_38 = fn14(_23, _16, _13.3, _8), bb27, UnwindUnreachable())
}
bb27 = {
_6 = _22;
_16 = (_13.1, _13.3, _45.fld0);
_8.0 = [_3,_3];
_9 = _13.3;
_16.1 = _9 * _9;
_48.4 = _20 as u32;
_48.1 = _16.0 != _16.0;
_13.0 = !_17;
_13.1 = _16.0 ^ _16.0;
_51 = _44;
_48.4 = !_12;
(*_26) = _48.4;
_51 = [(-112607104_i32)];
_10 = _3 as f32;
_37 = _48.0;
_20 = _8.1;
_48.2 = _8.0;
Goto(bb28)
}
bb28 = {
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0.2 = -_4;
_13.0 = _17;
_16.0 = -_13.1;
_54 = _45.fld0 as u8;
_16 = (_13.1, _9, _45.fld0);
_45.fld0 = _16.2;
_56.0 = [_1,_1];
_60 = _38.2 as f32;
_9 = -_16.1;
_59 = [_16.0,_16.0,_13.1];
_16 = (_13.1, _13.3, _45.fld0);
_56.0 = [_3,_3];
match _42.2 {
340282366920938463455006314768362265615 => bb29,
_ => bb15
}
}
bb29 = {
_3 = _1 - _1;
_18 = [_2];
_42.1 = [_45.fld0,_16.2,_53,_16.2,_45.fld0,_45.fld0];
_48.3 = core::ptr::addr_of!(_38.2);
_42 = (_60, Field::<Adt50>(Variant(_29, 2), 0).fld0.1, _38.2);
_16 = (_13.1, _13.3, _45.fld0);
_2 = _34;
_56.1 = _17 as u128;
(*_25) = (*_26) >> _3;
_60 = 34152752655027303586724660557429562509_i128 as f32;
_54 = !_14;
_22 = !_48.1;
(*_31) = !(*_25);
(*_37) = (-65592876210869858393691467019214734744_i128) as i16;
_16.1 = _13.3;
_18 = [_34];
_14 = _54 ^ _54;
_54 = !_14;
_42.1 = [_36,_45.fld0,_45.fld0,_45.fld0,_53,_53];
_16.2 = !_53;
_16.2 = (-1466608884_i32) as u16;
_48.2 = [_3,_3];
_44 = [253398603_i32];
_28 = Adt55::Variant3 { fld0: _13.3 };
match _4 {
0 => bb30,
340282366920938463455006314768362265615 => bb32,
_ => bb31
}
}
bb30 = {
_8.1 = 51140791828656654071765554938944768454_u128;
_4 = !4800063455210199834_i64;
_16 = (_13.1, _13.3, 11860_u16);
_16 = (_13.1, _13.3, 20301_u16);
_16.1 = -_13.3;
_21 = [(-1321322609_i32),(-51615678_i32),(-1140722024_i32),1468137865_i32,554021606_i32,176613358_i32,1956596032_i32,108388148_i32];
_12 = _13.2 & _13.2;
_18 = [_2];
_16.1 = _13.3 + _13.3;
_13.0 = _16.1 as i8;
_7 = [_2,_2,_2,_2,_2];
_16.2 = 16873_u16;
_12 = _16.0 as u32;
_9 = _4 as f64;
_19 = !_13.2;
_20 = !_8.1;
_20 = _6 as u128;
_21 = [1684381928_i32,(-36207625_i32),1370526671_i32,(-903837733_i32),1097096883_i32,1473716572_i32,(-1348085056_i32),(-1352862003_i32)];
_21 = [101356621_i32,985021183_i32,1291869952_i32,912349415_i32,(-1321975743_i32),(-1215861914_i32),(-1308721571_i32),(-759033814_i32)];
_21 = [222181840_i32,689186776_i32,1554140022_i32,(-202646674_i32),(-302136676_i32),553728735_i32,(-176323168_i32),(-1558313304_i32)];
_20 = !_8.1;
Goto(bb11)
}
bb31 = {
_3 = !_1;
_4 = -9090666585329364385_i64;
_10 = (-17902_i16) as f32;
_1 = !_3;
_4 = (-7598130229829855051_i64) + 3334452980523051957_i64;
_12 = 509903627_u32 ^ 2187363010_u32;
_8.1 = 188744444220393779528356791527339203973_u128 & 110302095921986413826214468588179530122_u128;
_6 = _2 > _2;
_11 = (-40_isize) & 17_isize;
_13 = (68_i8, _11, _12, _9);
_4 = -6184436884952208360_i64;
Goto(bb8)
}
bb32 = {
_52 = -Field::<Adt50>(Variant(_29, 2), 0).fld0.0;
_23 = _45.fld0 * _45.fld0;
_12 = (*_26) | (*_31);
match _4 {
0 => bb10,
1 => bb21,
2 => bb29,
3 => bb33,
4 => bb34,
340282366920938463455006314768362265615 => bb36,
_ => bb35
}
}
bb33 = {
Return()
}
bb34 = {
_1 = _3;
_13.0 = 54_i8 + 114_i8;
_3 = !_1;
_12 = _13.3 as u32;
_11 = _13.1 - _13.1;
_13.1 = 30_u8 as isize;
_13.3 = -_9;
_6 = false;
_12 = _11 as u32;
_2 = '\u{d00c}';
_1 = !_3;
_11 = !_13.1;
_7 = [_2,_2,_2,_2,_2];
_13 = ((-71_i8), _11, _12, _9);
_8.0 = [_3,_3];
_11 = _13.1 << _1;
_8.0 = [_3,_3];
_13.3 = -_9;
Call(_10 = fn4(_13.0, _1, _13, _13.0, _13.0), bb9, UnwindUnreachable())
}
bb35 = {
_48.3 = core::ptr::addr_of!(_38.2);
(*_26) = !_12;
_54 = _12 as u8;
SetDiscriminant(_29, 2);
_14 = _43 as u8;
_34 = _2;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld1 = [_34,_34,_2,_2,_2,_34];
_27 = [_2];
_48.0 = _37;
_16.2 = 1372543162_i32 as u16;
_7 = [_2,_34,_34,_2,_34];
place!(Field::<[i16; 6]>(Variant(_29, 2), 2)) = [_46.0,_46.0,_46.0,(*_37),_46.0,_46.0];
_53 = !_23;
_17 = _13.0;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0 = (_10, _38.1, _42.2);
_7 = [_34,_2,_34,_2,_2];
_13.1 = -_16.0;
_7 = [_2,_34,_34,_2,_2];
_13.2 = (*_25);
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0 = (_42.0, _42.1, _4);
_17 = _13.0 >> _8.1;
_42.1 = Field::<Adt50>(Variant(_29, 2), 0).fld0.1;
Call(_13.2 = fn13(_45.fld0, _38.0, _16.0, Field::<Adt50>(Variant(_29, 2), 0).fld0.0, _16.1, _13.1, _13.3, _16.0, Field::<Adt50>(Variant(_29, 2), 0).fld0, _38.1, Field::<Adt50>(Variant(_29, 2), 0).fld0.0, _13.1, _42.0, Field::<Adt50>(Variant(_29, 2), 0).fld0.0, _16), bb26, UnwindUnreachable())
}
bb36 = {
_62.0 = _34;
_58 = [_16.0,_16.0,_16.0];
_67 = Adt62::Variant2 { fld0: _46 };
_1 = _3;
_63 = _10;
(*_26) = !(*_25);
_39 = _18;
SetDiscriminant(_28, 3);
_30 = [1283111693_i32,1230403071_i32,(-746655050_i32),261060838_i32,(-1646367669_i32),954306739_i32,901927683_i32,(-694610008_i32)];
_65 = [_23,_45.fld0,_23,_23,_23,_45.fld0];
_38.1 = [_45.fld0,_23,_23,_23,_45.fld0,_23];
_36 = _23;
_14 = _54 >> _13.1;
_48.3 = core::ptr::addr_of!(_4);
_9 = -_13.3;
_42.1 = _65;
_7 = [_34,_62.0,_2,_34,_34];
_71.0 = _53 as isize;
_64 = _2;
(*_26) = !(*_25);
_58 = _59;
place!(Field::<f64>(Variant(_28, 3), 0)) = _16.1 * _13.3;
_52 = _14 as f32;
_25 = core::ptr::addr_of_mut!(_70.2);
_30 = [(-1257109035_i32),(-1925521307_i32),1588052322_i32,1771257451_i32,1655349903_i32,(-401108203_i32),1609594973_i32,195406142_i32];
_70 = (_13.0, _13.1, (*_26), _9);
(*_31) = _13.2;
_6 = !_22;
Goto(bb37)
}
bb37 = {
_38.0 = 5_usize as f32;
_46 = Field::<(i16,)>(Variant(_67, 2), 0);
_58 = [_11,_70.1,_70.1];
_51 = _32;
(*_37) = Field::<(i16,)>(Variant(_67, 2), 0).0;
_45.fld0 = _23 >> (*_25);
(*_37) = Field::<(i16,)>(Variant(_67, 2), 0).0;
_71.2 = !_23;
place!(Field::<Adt49>(Variant(_29, 2), 1)) = Adt49::Variant0 { fld0: 5_usize,fld1: _26 };
_16.2 = _45.fld0 + _45.fld0;
_42 = (_10, _65, _38.2);
_69 = _34;
_13.2 = !(*_25);
_38.1 = _42.1;
_39 = _27;
_70.0 = !_13.0;
_29 = Move(_28);
Call(_42.2 = fn15(_13.1, _65), bb38, UnwindUnreachable())
}
bb38 = {
_13.3 = _19 as f64;
_31 = _25;
_39 = [_69];
_34 = _2;
_13.2 = (*_26);
_48.3 = core::ptr::addr_of!(_38.2);
match _4 {
0 => bb15,
1 => bb18,
2 => bb12,
3 => bb36,
4 => bb8,
5 => bb6,
340282366920938463455006314768362265615 => bb40,
_ => bb39
}
}
bb39 = {
_8.1 = 51140791828656654071765554938944768454_u128;
_4 = !4800063455210199834_i64;
_16 = (_13.1, _13.3, 11860_u16);
_16 = (_13.1, _13.3, 20301_u16);
_16.1 = -_13.3;
_21 = [(-1321322609_i32),(-51615678_i32),(-1140722024_i32),1468137865_i32,554021606_i32,176613358_i32,1956596032_i32,108388148_i32];
_12 = _13.2 & _13.2;
_18 = [_2];
_16.1 = _13.3 + _13.3;
_13.0 = _16.1 as i8;
_7 = [_2,_2,_2,_2,_2];
_16.2 = 16873_u16;
_12 = _16.0 as u32;
_9 = _4 as f64;
_19 = !_13.2;
_20 = !_8.1;
_20 = _6 as u128;
_21 = [1684381928_i32,(-36207625_i32),1370526671_i32,(-903837733_i32),1097096883_i32,1473716572_i32,(-1348085056_i32),(-1352862003_i32)];
_21 = [101356621_i32,985021183_i32,1291869952_i32,912349415_i32,(-1321975743_i32),(-1215861914_i32),(-1308721571_i32),(-759033814_i32)];
_21 = [222181840_i32,689186776_i32,1554140022_i32,(-202646674_i32),(-302136676_i32),553728735_i32,(-176323168_i32),(-1558313304_i32)];
_20 = !_8.1;
Goto(bb11)
}
bb40 = {
_6 = !_22;
_27 = _18;
_35 = [(*_26),(*_31),(*_31),_13.2,_19];
_71.0 = (*_37) as isize;
_56.1 = _46.0 as u128;
_74 = _2;
place!(Field::<(i16,)>(Variant(_67, 2), 0)) = ((*_37),);
_23 = _20 as u16;
_13.2 = (*_31);
_4 = !_42.2;
_12 = !_48.4;
(*_31) = (*_26) + _13.2;
SetDiscriminant(_29, 2);
_18 = [_2];
_46 = Field::<(i16,)>(Variant(_67, 2), 0);
_39 = [_34];
_38.1 = _42.1;
_48.4 = (*_25) - _13.2;
Call(_16.0 = fn17(_38.2, _71.2, _70, _31, _70, _48, _25, _13, _48), bb41, UnwindUnreachable())
}
bb41 = {
_42.0 = -_52;
_50 = Move(_67);
_62 = (_2,);
_11 = _70.1 << (*_25);
_78 = _42.2 & _4;
SetDiscriminant(_50, 0);
_28 = Adt55::Variant1 { fld0: 7171600019312768498_usize,fld1: _48,fld2: _11,fld3: _70.0,fld4: _20 };
_16.2 = _45.fld0;
_75 = [_16.2,_16.2,_36,_36,_45.fld0,_71.2];
_28 = Adt55::Variant1 { fld0: 0_usize,fld1: _48,fld2: _13.1,fld3: _17,fld4: _8.1 };
Goto(bb42)
}
bb42 = {
_68 = _70.1 - _16.0;
_60 = -_42.0;
_8.0 = [_3,_3];
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0.0 = _14 as f32;
_77.0 = -_70.3;
_9 = -_16.1;
_28 = Adt55::Variant3 { fld0: _70.3 };
_71.1 = -_16.1;
_18 = _39;
_77.0 = _71.1 * _9;
_48.4 = (*_25) << (*_25);
(*_31) = _74 as u32;
place!(Field::<(i16,)>(Variant(_50, 0), 1)).0 = !(*_37);
_14 = _54 & _54;
_2 = _62.0;
_59 = [_68,_11,_68];
_23 = _16.2;
_12 = _48.4;
_46.0 = _74 as i16;
(*_25) = _48.4;
_32 = [1281397824_i32];
(*_37) = !Field::<(i16,)>(Variant(_50, 0), 1).0;
_50 = Adt62::Variant2 { fld0: _46 };
_75 = [_45.fld0,_16.2,_36,_45.fld0,_71.2,_16.2];
_9 = _77.0;
Call(place!(Field::<[i16; 6]>(Variant(_29, 2), 2)) = core::intrinsics::transmute(_38.1), bb43, UnwindUnreachable())
}
bb43 = {
_65 = _75;
_77.2 = !_1;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0.1 = [_23,_16.2,_36,_16.2,_23,_53];
_21 = _30;
_71.1 = _77.0;
(*_31) = 1438468950_i32 as u32;
_46 = (Field::<(i16,)>(Variant(_50, 2), 0).0,);
_71.2 = _70.0 as u16;
_43 = _13.1 - _68;
_16.0 = _43;
_13.0 = -_70.0;
_62.0 = _2;
_8 = (_56.0, _20);
_77.1 = [_62.0,_62.0,_69,_69,_2,_69];
SetDiscriminant(_28, 2);
_62 = (_34,);
_79 = _14 as i8;
place!(Field::<Adt50>(Variant(_28, 2), 0)).fld1 = [_64,_2,_34,_64,_74,_62.0];
_26 = _31;
_70.0 = _17;
_82 = _20 as i32;
Goto(bb44)
}
bb44 = {
_13.3 = _70.3 - _71.1;
_78 = _42.2 ^ _42.2;
_38 = _42;
_10 = _60 * _52;
_13 = _70;
_72 = (*_37);
_44 = _32;
_13.1 = -_11;
_77.3 = !_6;
_38.1 = _75;
_68 = -_43;
_77 = (_9, Field::<Adt50>(Variant(_28, 2), 0).fld1, _1, _6, _71.1);
_77.2 = _13.1 as u64;
_86 = Adt56 { fld0: (*_37) };
_78 = !_4;
place!(Field::<Adt50>(Variant(_28, 2), 0)).fld0.2 = _77.2 as i64;
_39 = [_69];
_43 = -_11;
place!(Field::<[i16; 6]>(Variant(_28, 2), 2)) = Field::<[i16; 6]>(Variant(_29, 2), 2);
_28 = Adt55::Variant1 { fld0: 12968475590688229179_usize,fld1: _48,fld2: _16.0,fld3: _79,fld4: _20 };
Goto(bb45)
}
bb45 = {
_89.2 = !Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).4;
Call(_88 = core::intrinsics::transmute(Field::<isize>(Variant(_28, 1), 2)), bb46, UnwindUnreachable())
}
bb46 = {
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1)).1 = !_48.1;
_91.2 = !_89.2;
_87 = [_82,_82,_82,_82,_82,_82,_82,_82];
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0.1 = [_16.2,_53,_36,_23,_16.2,_16.2];
_35 = [Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).4,_12,_19,_19,_48.4];
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1)).2 = _48.2;
place!(Field::<Adt50>(Variant(_29, 2), 0)) = Adt50 { fld0: _38,fld1: _77.1 };
Call(_87 = fn18(Field::<isize>(Variant(_28, 1), 2), _35, Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).4), bb47, UnwindUnreachable())
}
bb47 = {
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1)).1 = _77.2 != _88;
_44 = [_82];
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1)).1 = _77.3;
(*_37) = _86.fld0;
place!(Field::<(i16,)>(Variant(_50, 2), 0)) = ((*_37),);
_2 = _64;
SetDiscriminant(_50, 1);
_4 = _78 << _19;
_70.3 = (*_37) as f64;
_42.1 = [_71.2,_23,_45.fld0,_36,_16.2,_36];
_71 = _16;
_1 = _8.1 as u64;
_38.1 = Field::<Adt50>(Variant(_29, 2), 0).fld0.1;
_91.3 = _16.1 - _9;
_78 = _4;
_71.2 = !_45.fld0;
_93 = _77.2 as u32;
_86 = Adt56 { fld0: (*_37) };
_46 = (_86.fld0,);
_34 = _62.0;
_31 = _25;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0.0 = -_63;
(*_31) = _19;
_97.0 = _70.0 ^ _79;
Goto(bb48)
}
bb48 = {
place!(Field::<isize>(Variant(_28, 1), 2)) = (*_26) as isize;
_19 = _78 as u32;
_26 = _25;
_8.1 = _3 as u128;
place!(Field::<[u8; 8]>(Variant(_50, 1), 1)) = [_54,_14,_14,_14,_54,_14,_54,_14];
_91.2 = (*_31);
(*_31) = _82 as u32;
_23 = _82 as u16;
_66 = -_52;
_97.1 = Field::<isize>(Variant(_28, 1), 2);
_48 = (_37, _77.3, _8.0, Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).3, _91.2);
_48 = Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1);
_18 = [_64];
_16.2 = _36;
_74 = _62.0;
_39 = [_62.0];
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0.1 = [_45.fld0,_45.fld0,_36,_45.fld0,_36,_36];
_89.3 = -_16.1;
_91.0 = _4 as i8;
_102.1.1 = _77.2 as i128;
_1 = !_88;
place!(Field::<[u8; 8]>(Variant(_50, 1), 1)) = [_14,_54,_54,_14,_14,_14,_14,_14];
_59 = [_43,_16.0,_71.0];
_13.0 = _91.0 << _12;
_92 = [_88,_1];
_60 = -_42.0;
Goto(bb49)
}
bb49 = {
_83 = _69;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0.2 = Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).1 as i64;
_31 = _26;
_48 = (Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).0, _6, _92, Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).3, _89.2);
_17 = _13.0 << Field::<Adt50>(Variant(_29, 2), 0).fld0.2;
_41 = (*_31) << _70.1;
_94 = core::ptr::addr_of_mut!(_13.3);
_101 = _17;
place!(Field::<(i128, i128, i8)>(Variant(_50, 1), 0)) = (_102.1.1, _102.1.1, _91.0);
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld1 = [_34,_62.0,_64,_69,_34,_62.0];
_73 = _93;
Call(_3 = core::intrinsics::bswap(_88), bb50, UnwindUnreachable())
}
bb50 = {
_48.3 = core::ptr::addr_of!(_4);
_2 = _64;
_96 = _35;
_78 = _82 as i64;
_112 = core::ptr::addr_of_mut!(_40);
place!(Field::<Adt50>(Variant(_29, 2), 0)) = Adt50 { fld0: _42,fld1: _77.1 };
_15 = Adt49::Variant0 { fld0: 4_usize,fld1: _26 };
_105 = !2864114820585760879_usize;
_56.1 = Field::<u128>(Variant(_28, 1), 4);
_13 = (_91.0, _16.0, (*_31), _71.1);
_99 = core::ptr::addr_of_mut!(_16.1);
(*_94) = _54 as f64;
_20 = _8.1;
_89.1 = !_71.0;
_13.0 = _8.1 as i8;
_88 = _1 | _77.2;
place!(Field::<usize>(Variant(_28, 1), 0)) = _105 - _105;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld1 = [_69,_34,_69,_34,_64,_34];
_12 = !_93;
_13 = (_91.0, _71.0, (*_25), _16.1);
Goto(bb51)
}
bb51 = {
(*_112) = core::ptr::addr_of!(place!(Field::<(i128, i128, i8)>(Variant(_50, 1), 0)).1);
(*_94) = _71.0 as f64;
_53 = !_45.fld0;
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1)) = (_48.0, _22, _56.0, _48.3, _48.4);
_48.1 = Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).1 | _6;
_13.2 = _48.4 >> _13.0;
_9 = _102.1.1 as f64;
Goto(bb52)
}
bb52 = {
SetDiscriminant(_50, 2);
_81 = _83;
_66 = _63 * _42.0;
_60 = _66;
_115.0 = _13.0 | _17;
_116 = [_70.1,_89.1,_16.0,_43,_13.1,Field::<isize>(Variant(_28, 1), 2),_70.1,_68];
_75 = [_53,_71.2,_36,_16.2,_71.2,_45.fld0];
SetDiscriminant(_28, 3);
_42.0 = Field::<Adt50>(Variant(_29, 2), 0).fld0.0 - _10;
_91.2 = _78 as u32;
_51 = [_82];
_8 = _56;
_48.3 = core::ptr::addr_of!(_38.2);
_109.1 = _77.0;
(*_94) = _42.0 as f64;
_19 = _41 + _73;
_89.2 = _12 >> _4;
place!(Field::<Adt50>(Variant(_29, 2), 0)).fld0.2 = _4;
_102.1.0 = _102.1.1 - _102.1.1;
_97.2 = _42.0 as u32;
_44 = _51;
_71.0 = _77.2 as isize;
_91.0 = -_101;
_115.2 = !_93;
Goto(bb53)
}
bb53 = {
_48.3 = core::ptr::addr_of!(_4);
_64 = _34;
_80 = [_70.1,_70.1,_68];
_110 = [_77.2,_88];
_28 = Adt55::Variant3 { fld0: _9 };
_102.1.1 = !_102.1.0;
_102.1.2 = _101 & _115.0;
_86.fld0 = (*_37);
_71 = _16;
_7 = [_83,_2,_81,_62.0,_81];
_92 = [_1,_1];
_73 = (*_26);
_115 = (_17, _13.1, _97.2, _13.3);
_86 = Adt56 { fld0: (*_37) };
_6 = _48.1;
_124 = _11 - _43;
_95 = core::ptr::addr_of_mut!((*_99));
_89.2 = _19;
_26 = _31;
(*_112) = core::ptr::addr_of!(_102.0);
SetDiscriminant(_28, 2);
_118 = core::ptr::addr_of_mut!(_91.3);
_3 = _88;
_10 = -_60;
_38.2 = Field::<Adt50>(Variant(_29, 2), 0).fld0.2 - _4;
Goto(bb54)
}
bb54 = {
_111 = _81;
_91.1 = _43;
_122.1.0 = _82 as i128;
(*_99) = -(*_118);
place!(Field::<[i16; 6]>(Variant(_28, 2), 2)) = [_86.fld0,_72,_72,(*_37),(*_37),(*_37)];
place!(Field::<Adt49>(Variant(_29, 2), 1)) = Adt49::Variant0 { fld0: _105,fld1: _25 };
(*_40) = _102.1.1;
_35 = [_48.4,_93,_93,_13.2,_19];
_117 = Field::<[i16; 6]>(Variant(_29, 2), 2);
SetDiscriminant(_29, 3);
_119.fld1 = _77.1;
_58 = [_124,_97.1,_70.1];
_97.2 = !_115.2;
_109.2 = _71.2 | _71.2;
_74 = _34;
_111 = _34;
_5 = core::ptr::addr_of!(_99);
_77.4 = _38.2 as f64;
_118 = core::ptr::addr_of_mut!(_115.3);
_123 = (_46.0,);
_93 = !_13.2;
_113 = _88 as f32;
_128 = Adt64::Variant1 { fld0: _14,fld1: _64,fld2: _102.0 };
Goto(bb55)
}
bb55 = {
_77.1 = _119.fld1;
_116 = [_68,_91.1,_115.1,_70.1,_124,_89.1,_16.0,_97.1];
_72 = !(*_37);
_65 = [_109.2,_53,_45.fld0,_45.fld0,_36,_16.2];
_122.1.0 = _102.1.0;
_103 = !_19;
_88 = _77.2 & _3;
Goto(bb56)
}
bb56 = {
_97.3 = (*_99) * _89.3;
_118 = _95;
place!(Field::<f64>(Variant(_29, 3), 0)) = _91.0 as f64;
_115.3 = (*_95);
_128 = Adt64::Variant1 { fld0: _14,fld1: _69,fld2: _102.1.0 };
(*_95) = -(*_94);
_111 = _81;
_89.0 = _62.0 as i8;
_42 = (_113, _65, _4);
_100 = _48.1;
(*_40) = -Field::<i128>(Variant(_128, 1), 2);
_63 = -_113;
_102.0 = _122.1.0 + _102.1.1;
_126 = !_22;
Call(_127 = core::intrinsics::transmute((*_25)), bb57, UnwindUnreachable())
}
bb57 = {
_91.3 = (*_94) + _9;
SetDiscriminant(_128, 1);
_109 = _16;
_102.1.2 = _48.4 as i8;
SetDiscriminant(_29, 3);
(*_26) = _48.4;
_52 = _12 as f32;
_20 = _8.1;
_69 = _111;
_92 = _110;
_94 = core::ptr::addr_of_mut!((*_94));
(*_94) = (*_118) * _91.3;
_102.0 = !_102.1.1;
Goto(bb58)
}
bb58 = {
_120 = _88 + _77.2;
_38.2 = _4;
_114 = [_69,_83,_69,_81,_81];
_125 = [_83,_81,_74,_81,_2];
_122.1.2 = _13.3 as i8;
_12 = _97.2 | _70.2;
_132 = _112;
_22 = _77.3;
_73 = _70.2 ^ (*_25);
_70.2 = !_89.2;
_122.1 = _102.1;
place!(Field::<f64>(Variant(_29, 3), 0)) = (*_94);
SetDiscriminant(_29, 1);
_89.2 = _66 as u32;
place!(Field::<usize>(Variant(_15, 0), 0)) = !_105;
_34 = _111;
_97 = (_102.1.2, _16.0, (*_31), _91.3);
(*_5) = _95;
_91.0 = -_97.0;
Goto(bb59)
}
bb59 = {
place!(Field::<Adt50>(Variant(_28, 2), 0)).fld0 = (_42.0, _75, _78);
_74 = _2;
place!(Field::<(i16,)>(Variant(_50, 2), 0)).0 = _83 as i16;
_13 = (_102.1.2, _43, (*_25), (*_95));
SetDiscriminant(_50, 0);
SetDiscriminant(_15, 0);
place!(Field::<[i16; 6]>(Variant(_28, 2), 2)) = _117;
_91.3 = _97.3 * _89.3;
_35 = [_73,_73,(*_31),_73,_13.2];
_37 = _48.0;
_75 = [_109.2,_53,_45.fld0,_71.2,_45.fld0,_36];
place!(Field::<*mut u32>(Variant(_15, 0), 1)) = _31;
_11 = _20 as isize;
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_29, 1), 1)).1 = _48.1 | _48.1;
Goto(bb60)
}
bb60 = {
_42 = (_52, _38.1, _4);
_20 = !_56.1;
_85 = _89.1;
_76 = _5;
_122.0 = (*_95) as i128;
place!(Field::<i8>(Variant(_29, 1), 3)) = !_13.0;
_113 = _66 + Field::<Adt50>(Variant(_28, 2), 0).fld0.0;
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_29, 1), 1)).4 = _12 * _41;
_45.fld0 = !_109.2;
_71.1 = _77.0 + _77.4;
place!(Field::<(i16,)>(Variant(_50, 0), 1)).0 = _72 * _86.fld0;
_26 = core::ptr::addr_of_mut!(_19);
Goto(bb61)
}
bb61 = {
_109.0 = _105 as isize;
_62 = (_2,);
(*_40) = _38.2 as i128;
_106.0 = _70.1 as i16;
place!(Field::<u128>(Variant(_29, 1), 4)) = _111 as u128;
place!(Field::<Adt50>(Variant(_28, 2), 0)).fld1 = _77.1;
_115.0 = _43 as i8;
_97.1 = _43;
_120 = _88;
_4 = -_78;
Goto(bb62)
}
bb62 = {
_109.0 = -_85;
(*_95) = _71.1;
(*_118) = -(*_94);
place!(Field::<u8>(Variant(_128, 1), 0)) = _54 + _14;
_138 = _97.3 - _77.4;
(*_112) = core::ptr::addr_of!(_102.1.1);
_32 = _51;
_21 = [_127,_82,_82,_82,_82,_127,_82,_82];
_75 = _65;
_109 = (_13.1, _77.0, _53);
_122.1.0 = _77.0 as i128;
_32 = _51;
_8 = (_110, _20);
_119.fld0 = _38;
(*_132) = core::ptr::addr_of!(_102.1.0);
(*_5) = _118;
_77.3 = _6 ^ _6;
Goto(bb63)
}
bb63 = {
_90 = [_89.1,_91.1,_68];
place!(Field::<u16>(Variant(_50, 0), 3)) = _53 >> _122.1.1;
_36 = _48.1 as u16;
Goto(bb64)
}
bb64 = {
_125 = _7;
_137 = -(*_94);
_131 = !_3;
(*_112) = core::ptr::addr_of!((*_40));
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_29, 1), 1)).4 = _105 as u32;
_154 = Adt56 { fld0: _106.0 };
place!(Field::<usize>(Variant(_29, 1), 0)) = _105;
_75 = _65;
_22 = !Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_29, 1), 1).1;
_110 = [_120,_1];
Goto(bb65)
}
bb65 = {
_152 = -_115.0;
_126 = !Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_29, 1), 1).1;
_68 = _11 & _13.1;
_6 = !_77.3;
_13 = (_91.0, _71.0, (*_31), _89.3);
place!(Field::<Adt55>(Variant(_50, 0), 2)) = Adt55::Variant1 { fld0: Field::<usize>(Variant(_29, 1), 0),fld1: _48,fld2: _71.0,fld3: _101,fld4: _20 };
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 1), 1)).1 = _48.1;
(*_99) = -_97.3;
_42 = Field::<Adt50>(Variant(_28, 2), 0).fld0;
_120 = _1;
_89.3 = (*_99);
_120 = !_1;
_51 = [_127];
_119.fld0.2 = Field::<usize>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 1), 0) as i64;
Goto(bb66)
}
bb66 = {
_135 = _64;
_16.1 = _154.fld0 as f64;
_20 = Field::<u128>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 1), 4) << _109.2;
_38.0 = -_60;
(*_31) = _73 - _12;
_160 = _89.1;
_39 = [_111];
_136 = _97.0 as f64;
_16.0 = -_68;
_68 = Field::<isize>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 1), 2) << _16.2;
SetDiscriminant(Field::<Adt55>(Variant(_50, 0), 2), 0);
_16 = (_71.0, _97.3, _36);
place!(Field::<Adt59>(Variant(_50, 0), 4)) = Adt59::Variant0 { fld0: _77.2,fld1: _132 };
place!(Field::<u16>(Variant(_50, 0), 3)) = !_109.2;
_8 = (_110, _20);
_122.0 = -_122.1.0;
_89.2 = _115.2;
_142 = _81;
Goto(bb67)
}
bb67 = {
_144 = (*_40);
_29 = Adt55::Variant1 { fld0: _105,fld1: _48,fld2: _85,fld3: _13.0,fld4: _8.1 };
SetDiscriminant(_29, 3);
_92 = _56.0;
_99 = core::ptr::addr_of_mut!(_115.3);
_138 = _14 as f64;
place!(Field::<Adt50>(Variant(_28, 2), 0)).fld1 = [_62.0,_69,_81,_2,_69,_81];
_11 = _16.0 ^ _85;
_60 = _10;
_146 = _106;
_87 = _30;
place!(Field::<i128>(Variant(_128, 1), 2)) = !_102.1.0;
place!(Field::<Adt50>(Variant(_28, 2), 0)).fld0.0 = _82 as f32;
Goto(bb68)
}
bb68 = {
_109.2 = !_45.fld0;
_155 = _20 >> _89.2;
place!(Field::<(i16,)>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 4)) = _146;
place!(Field::<u64>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 3)) = _105 as u64;
(*_25) = _74 as u32;
_138 = _16.1 - _13.3;
_154 = Adt56 { fld0: _146.0 };
_13 = (_115.0, _91.1, (*_26), (*_99));
_130.0 = _62.0;
_102.1.2 = -_13.0;
_97.1 = !_71.0;
_122.0 = -_122.1.1;
_32 = [_82];
SetDiscriminant(Field::<Adt59>(Variant(_50, 0), 4), 0);
_112 = _132;
place!(Field::<(i16,)>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 4)) = (_146.0,);
_71 = (_89.1, _138, _53);
_42.2 = !_4;
_115.0 = _3 as i8;
_161.fld2.2 = !_53;
place!(Field::<u8>(Variant(_128, 1), 0)) = _54;
_115.1 = _20 as isize;
Call(_86.fld0 = core::intrinsics::transmute(_16.2), bb69, UnwindUnreachable())
}
bb69 = {
_96 = _35;
_168 = core::ptr::addr_of_mut!((*_112));
_117 = [_86.fld0,_106.0,_86.fld0,_154.fld0,_106.0,_86.fld0];
place!(Field::<Adt59>(Variant(_50, 0), 4)) = Adt59::Variant1 { fld0: (*_5),fld1: _115.0 };
_48.3 = core::ptr::addr_of!(_42.2);
_44 = _51;
Goto(bb70)
}
bb70 = {
_161.fld0.2 = _8.0;
_169 = Field::<i8>(Variant(Field::<Adt59>(Variant(_50, 0), 4), 1), 1) as f64;
_161.fld0 = (_37, _22, _110, _48.3, _13.2);
_147 = _97.2;
_118 = core::ptr::addr_of_mut!((*_95));
_42.0 = _119.fld0.0 + _52;
_19 = _115.2 * _89.2;
_42 = _38;
_150.fld0 = core::ptr::addr_of_mut!(_70.2);
_161.fld2.0 = _43;
place!(Field::<*const *mut f64>(Variant(_50, 0), 0)) = core::ptr::addr_of!((*_5));
_115 = (_91.0, _11, _161.fld0.4, _77.0);
_91 = (_152, _97.1, _12, _97.3);
_131 = !_3;
_160 = _109.0 & _115.1;
(*_99) = (*_118);
_115.3 = -(*_94);
_97.1 = _91.1;
_161.fld5 = !_122.0;
_106.0 = _154.fld0 * _154.fld0;
_154.fld0 = _86.fld0 >> (*_40);
_56.1 = !_8.1;
SetDiscriminant(Field::<Adt59>(Variant(_50, 0), 4), 0);
Goto(bb71)
}
bb71 = {
(*_5) = core::ptr::addr_of_mut!(_71.1);
_161.fld3 = (_142,);
place!(Field::<*mut *const i128>(Variant(place!(Field::<Adt59>(Variant(_50, 0), 4)), 0), 1)) = core::ptr::addr_of_mut!(_171);
Goto(bb72)
}
bb72 = {
place!(Field::<Adt59>(Variant(_50, 0), 4)) = Adt59::Variant0 { fld0: _1,fld1: _112 };
_40 = core::ptr::addr_of!(_122.1.1);
_47 = Adt60::Variant2 { fld0: _161.fld0.3,fld1: _142,fld2: _30,fld3: _161.fld3,fld4: _5,fld5: _122,fld6: _90,fld7: _77 };
_91.2 = !_115.2;
_162 = _42.1;
SetDiscriminant(_47, 1);
place!(Field::<*mut f64>(Variant(_47, 1), 1)) = (*_5);
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld4 = core::ptr::addr_of!(_88);
_35 = _96;
_97 = _91;
_60 = _52 + _38.0;
(*_94) = (*_95);
_130 = (_83,);
_21 = _87;
_35 = [_103,_115.2,_48.4,_41,_115.2];
_137 = -_71.1;
_128 = Adt64::Variant1 { fld0: _14,fld1: _69,fld2: _161.fld5 };
_6 = _22;
_125 = [_34,_135,_2,_64,_62.0];
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld2.2 = !_161.fld2.2;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld0.1 = _77.3 <= _161.fld0.1;
SetDiscriminant(_128, 0);
_130 = (_64,);
_119.fld0.0 = _60 + _60;
SetDiscriminant(Field::<Adt59>(Variant(_50, 0), 4), 0);
_167 = (_154.fld0,);
_109.2 = !Field::<u16>(Variant(_50, 0), 3);
Goto(bb73)
}
bb73 = {
_91.3 = (*_95) * _137;
place!(Field::<*mut *const i128>(Variant(place!(Field::<Adt59>(Variant(_50, 0), 4)), 0), 1)) = _168;
_172.fld1 = [_81,_64,_81,_2,_34,_111];
_5 = core::ptr::addr_of!(_99);
place!(Field::<f32>(Variant(_47, 1), 6)) = _38.0 - _60;
_33 = core::ptr::addr_of!(_99);
_28 = Adt55::Variant3 { fld0: _115.3 };
_115.0 = _152;
_127 = _82;
_105 = 3_usize << _161.fld5;
_172.fld0.2 = -_42.2;
_89.1 = (*_40) as isize;
_122 = (_102.1.1, _102.1);
_173 = _87;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld2.2 = _109.2;
_175 = _13.2 >> _1;
_23 = Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2;
_119.fld0.1 = [_71.2,Field::<u16>(Variant(_50, 0), 3),_71.2,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2,_36,_161.fld2.2];
_63 = -_10;
_112 = core::ptr::addr_of_mut!((*_168));
Goto(bb74)
}
bb74 = {
_60 = _42.0 * _52;
_14 = _167.0 as u8;
_154.fld0 = !Field::<(i16,)>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 4).0;
Goto(bb75)
}
bb75 = {
_172 = Adt50 { fld0: _119.fld0,fld1: _119.fld1 };
place!(Field::<i128>(Variant(_128, 0), 0)) = _102.0;
_12 = _13.1 as u32;
place!(Field::<[u16; 6]>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 2)) = [_45.fld0,_16.2,_16.2,_16.2,_71.2,_109.2];
_42.1 = [_45.fld0,_36,_109.2,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2,_36,_53];
(*_112) = core::ptr::addr_of!(place!(Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3)).0);
place!(Field::<u64>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 3)) = _88 - _1;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld0 = _48;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld6 = _105 * _105;
(*_94) = _138;
_156 = -Field::<f32>(Variant(_47, 1), 6);
_69 = _81;
_161.fld2 = _109;
_130 = (_69,);
(*_37) = _127 as i16;
_179.4 = _12 + _41;
_161.fld3.0 = _135;
Goto(bb76)
}
bb76 = {
_164.fld0 = _122.1.1 as u16;
_38.1 = [Field::<u16>(Variant(_50, 0), 3),Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2,_53,_36,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2,_45.fld0];
SetDiscriminant(_28, 0);
_118 = core::ptr::addr_of_mut!(_182);
place!(Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3)).1 = _122.1;
_48.0 = core::ptr::addr_of!(place!(Field::<Adt56>(Variant(_47, 1), 7)).fld0);
_141 = Adt54::Variant0 { fld0: (*_112),fld1: Move(_150),fld2: _91.1,fld3: _172,fld4: _56.1,fld5: _147 };
place!(Field::<Adt51>(Variant(_28, 0), 1)).fld0.3 = core::ptr::addr_of!(_42.2);
(*_132) = Field::<*const i128>(Variant(_141, 0), 0);
_70.1 = _89.1 + _161.fld2.0;
_70.2 = !_115.2;
place!(Field::<u64>(Variant(place!(Field::<Adt59>(Variant(_50, 0), 4)), 0), 0)) = _86.fld0 as u64;
place!(Field::<*mut u32>(Variant(_15, 0), 1)) = core::ptr::addr_of_mut!((*_31));
place!(Field::<Adt51>(Variant(_28, 0), 1)).fld3.0 = _161.fld3.0;
_39 = [_130.0];
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld2.2 = _53;
place!(Field::<Adt56>(Variant(_47, 1), 7)) = Adt56 { fld0: _86.fld0 };
_100 = !_48.1;
_35 = [_103,_103,_93,_13.2,(*_26)];
_95 = core::ptr::addr_of_mut!((*_94));
_146.0 = Field::<(i16,)>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 4).0 << _86.fld0;
_31 = core::ptr::addr_of_mut!(_179.4);
place!(Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3)).1.0 = Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.1;
_55 = Move(_141);
Goto(bb77)
}
bb77 = {
_189.1 = _122.0 - _122.0;
(*_95) = _8.1 as f64;
Goto(bb78)
}
bb78 = {
_185.0 = _120 as f64;
SetDiscriminant(Field::<Adt59>(Variant(_50, 0), 4), 0);
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld0.0 = core::ptr::addr_of!(place!(Field::<Adt56>(Variant(_47, 1), 7)).fld0);
_42.1 = [_164.fld0,_53,_36,_23,Field::<u16>(Variant(_50, 0), 3),_23];
_169 = _71.1;
_130 = _161.fld3;
_39 = [_142];
_127 = _102.1.1 as i32;
_198.0 = _110;
place!(Field::<(i16,)>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 4)).0 = Field::<u128>(Variant(_55, 0), 4) as i16;
_138 = _70.0 as f64;
place!(Field::<u64>(Variant(_28, 0), 3)) = _3;
_150 = Adt52 { fld0: _25 };
_57 = Move(_55);
_119 = _172;
_95 = _99;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld0.2 = [_1,_3];
place!(Field::<Adt59>(Variant(_50, 0), 4)) = Adt59::Variant1 { fld0: (*_5),fld1: _102.1.2 };
place!(Field::<Adt50>(Variant(_57, 0), 3)).fld0.2 = _38.2;
Goto(bb79)
}
bb79 = {
_46.0 = _161.fld0.1 as i16;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3)).1.1 = Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.0;
place!(Field::<[u8; 8]>(Variant(_128, 0), 3)) = [_14,_14,_14,_14,_14,_14,_14,_14];
_168 = core::ptr::addr_of_mut!(_171);
_89.0 = _101;
_197 = !_102.0;
_178 = [_48.4,(*_26),_41,(*_31),_12];
_6 = _71.2 > Field::<u16>(Variant(_50, 0), 3);
place!(Field::<Adt51>(Variant(_28, 0), 1)).fld0 = _48;
_187 = [_82];
place!(Field::<f64>(Variant(_29, 3), 0)) = Field::<Adt50>(Variant(_57, 0), 3).fld0.2 as f64;
place!(Field::<*const *mut f64>(Variant(_50, 0), 0)) = _33;
_160 = !_68;
SetDiscriminant(_57, 1);
_42.0 = _156 - _63;
_131 = !Field::<u64>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 3);
place!(Field::<*mut *const i128>(Variant(_128, 0), 4)) = core::ptr::addr_of_mut!(_171);
_105 = Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld6 << _103;
_126 = !_48.1;
(*_94) = -_161.fld2.1;
_18 = [_64];
_91.2 = !_13.2;
_62 = (_111,);
_189.0 = -_122.0;
Goto(bb80)
}
bb80 = {
_4 = _127 as i64;
SetDiscriminant(_29, 0);
_161.fld0.4 = _41 ^ (*_31);
_161.fld4 = Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld4;
place!(Field::<Adt51>(Variant(_28, 0), 1)).fld6 = Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld6;
_146 = (_167.0,);
_65 = [_23,_53,_161.fld2.2,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2,_36];
_66 = _46.0 as f32;
Goto(bb81)
}
bb81 = {
_180 = Adt49::Variant1 { fld0: _6,fld1: _102.1,fld2: _117,fld3: _155,fld4: _48.3,fld5: _119.fld0.0,fld6: Field::<*mut f64>(Variant(Field::<Adt59>(Variant(_50, 0), 4), 1), 0),fld7: _132 };
place!(Field::<i64>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 0)) = _20 as i64;
_54 = _14;
_102.1.2 = _1 as i8;
_16.1 = -_9;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld2 = (_71.0, _91.3, _53);
SetDiscriminant(Field::<Adt59>(Variant(_50, 0), 4), 1);
_172.fld0.1 = [_16.2,_53,_164.fld0,_45.fld0,_164.fld0,_36];
place!(Field::<Adt51>(Variant(_28, 0), 1)).fld2.0 = _85;
place!(Field::<Adt51>(Variant(_28, 0), 1)).fld0.0 = core::ptr::addr_of!(_72);
_183 = (_106.0,);
_163 = Adt58::Variant1 { fld0: _119,fld1: _14,fld2: _161.fld4,fld3: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2 };
Goto(bb82)
}
bb82 = {
_170 = _14;
_44 = _51;
_161.fld1 = Adt48::Variant3 { fld0: (*_112),fld1: _127,fld2: _187,fld3: _117 };
Goto(bb83)
}
bb83 = {
_81 = _111;
_192 = _136 * (*_94);
_96 = [(*_25),(*_26),(*_25),_115.2,_103];
_161.fld1 = Adt48::Variant2 { fld0: Field::<Adt50>(Variant(_163, 1), 0).fld1,fld1: _161.fld0.2,fld2: Field::<i64>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 0),fld3: _40,fld4: Field::<[u8; 8]>(Variant(_128, 0), 3),fld5: _127 };
_198 = (_48.2, _56.1);
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld5 = _102.1.1;
place!(Field::<([u64; 2], u128)>(Variant(_57, 1), 2)).1 = !_8.1;
_89.1 = Field::<Adt51>(Variant(_28, 0), 1).fld2.0;
_97.0 = _89.0;
place!(Field::<i64>(Variant(_28, 0), 0)) = _4;
_85 = _109.0;
_205.1 = Field::<(i128, i128, i8)>(Variant(_180, 1), 1);
_38 = (_172.fld0.0, _119.fld0.1, Field::<i64>(Variant(_28, 0), 0));
(*_95) = -_16.1;
Call(place!(Field::<(i16,)>(Variant(_28, 0), 4)).0 = core::intrinsics::transmute(Field::<(isize, f64, u16)>(Variant(_163, 1), 3).2), bb84, UnwindUnreachable())
}
bb84 = {
_16.1 = _77.4;
_119.fld0.1 = [_71.2,_164.fld0,_16.2,_71.2,Field::<(isize, f64, u16)>(Variant(_163, 1), 3).2,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2];
_149 = Adt54::Variant1 { fld0: (*_76),fld1: _3,fld2: _8,fld3: _161.fld1 };
place!(Field::<Adt51>(Variant(_29, 0), 1)) = Adt51 { fld0: _161.fld0,fld1: Field::<Adt48>(Variant(_149, 1), 3),fld2: _109,fld3: _62,fld4: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld4,fld5: _161.fld5,fld6: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld6 };
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld2.0 = !_68;
_164.fld0 = !_16.2;
_205.0 = -Field::<(i128, i128, i8)>(Variant(_180, 1), 1).0;
SetDiscriminant(_180, 1);
(*_118) = Field::<Adt51>(Variant(_28, 0), 1).fld6 as f64;
_59 = [_11,_109.0,_115.1];
Goto(bb85)
}
bb85 = {
_186 = !_42.2;
place!(Field::<u64>(Variant(_57, 1), 1)) = !_1;
place!(Field::<Adt54>(Variant(_47, 1), 0)) = Adt54::Variant0 { fld0: Field::<*const i128>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 3),fld1: Move(_150),fld2: _70.1,fld3: Field::<Adt50>(Variant(_163, 1), 0),fld4: _8.1,fld5: (*_31) };
place!(Field::<Adt50>(Variant(place!(Field::<Adt54>(Variant(_47, 1), 0)), 0), 3)).fld0.2 = _205.1.2 as i64;
(*_5) = core::ptr::addr_of_mut!(_136);
_72 = _16.0 as i16;
place!(Field::<Adt50>(Variant(place!(Field::<Adt54>(Variant(_47, 1), 0)), 0), 3)) = _119;
Goto(bb86)
}
bb86 = {
_132 = _112;
_165 = _127;
_150.fld0 = Field::<*mut u32>(Variant(_15, 0), 1);
_17 = Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.2;
_121 = Adt57::Variant0 { fld0: _132,fld1: (*_25),fld2: Field::<Adt51>(Variant(_29, 0), 1),fld3: Field::<i64>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 0),fld4: _120 };
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld1 = Adt48::Variant2 { fld0: Field::<[char; 6]>(Variant(Field::<Adt48>(Variant(_149, 1), 3), 2), 0),fld1: Field::<[u64; 2]>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 1),fld2: _4,fld3: Field::<*const i128>(Variant(Field::<Adt54>(Variant(_47, 1), 0), 0), 0),fld4: Field::<[u8; 8]>(Variant(_161.fld1, 2), 4),fld5: Field::<i32>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 5) };
_200.4 = !Field::<Adt51>(Variant(_121, 0), 2).fld0.4;
place!(Field::<Adt51>(Variant(_28, 0), 1)).fld0.2 = [_88,_131];
_27 = _18;
_145 = Field::<([u64; 2], u128)>(Variant(_149, 1), 2).1 | _155;
SetDiscriminant(Field::<Adt51>(Variant(_121, 0), 2).fld1, 2);
(*_94) = _71.1;
_179.0 = core::ptr::addr_of!(place!(Field::<Adt56>(Variant(_47, 1), 7)).fld0);
_38 = (_66, _65, Field::<i64>(Variant(_28, 0), 0));
_172.fld0.2 = _124 as i64;
place!(Field::<[u8; 8]>(Variant(_128, 0), 3)) = Field::<[u8; 8]>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 4);
_125 = [_64,_34,_69,Field::<Adt51>(Variant(_29, 0), 1).fld3.0,_34];
Goto(bb87)
}
bb87 = {
_122.1 = (_122.0, _205.1.0, _91.0);
_161.fld0 = (Field::<Adt51>(Variant(_28, 0), 1).fld0.0, _100, Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld0.2, _48.3, (*_26));
Goto(bb88)
}
bb88 = {
_222 = [_2,_83,_81,_111,_62.0,Field::<Adt51>(Variant(_29, 0), 1).fld3.0];
place!(Field::<i64>(Variant(place!(Field::<Adt48>(Variant(_149, 1), 3)), 2), 2)) = _38.2 - Field::<i64>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 2);
_70.0 = Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.2 * _13.0;
_172.fld0.0 = _165 as f32;
_122.1.1 = Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.0 * Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.1;
_130.0 = _2;
_127 = Field::<i32>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 5) * Field::<i32>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 5);
(*_25) = (*_26);
_83 = _2;
place!(Field::<*mut f64>(Variant(place!(Field::<Adt59>(Variant(_50, 0), 4)), 1), 0)) = core::ptr::addr_of_mut!((*_118));
_123 = Field::<(i16,)>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 4);
_203 = _117;
place!(Field::<Adt59>(Variant(_50, 0), 4)) = Adt59::Variant1 { fld0: _95,fld1: _102.1.2 };
_156 = _119.fld0.0;
Goto(bb89)
}
bb89 = {
SetDiscriminant(_161.fld1, 1);
place!(Field::<Adt51>(Variant(_121, 0), 2)).fld0.2 = Field::<Adt51>(Variant(_29, 0), 1).fld0.2;
_210.4 = (*_95);
Goto(bb90)
}
bb90 = {
_214 = -_66;
_8.1 = !_198.1;
place!(Field::<Adt57>(Variant(_128, 0), 2)) = Adt57::Variant1 { fld0: _96,fld1: _116,fld2: Field::<[u8; 8]>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 4),fld3: _89.0,fld4: Field::<Adt48>(Variant(_149, 1), 3),fld5: _179.0 };
_189.1 = _161.fld5 >> _13.1;
place!(Field::<f32>(Variant(_180, 1), 5)) = _66 * Field::<f32>(Variant(_47, 1), 6);
place!(Field::<[char; 6]>(Variant(place!(Field::<Adt51>(Variant(_121, 0), 2)).fld1, 2), 0)) = [_111,_2,_62.0,_135,_64,_34];
place!(Field::<(isize, f64, u16)>(Variant(_163, 1), 3)) = Field::<Adt51>(Variant(_29, 0), 1).fld2;
_189.1 = Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.0 - Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld5;
Goto(bb91)
}
bb91 = {
_181 = _210.4 as f32;
_188 = Adt62::Variant1 { fld0: Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1,fld1: Field::<[u8; 8]>(Variant(Field::<Adt48>(Variant(Field::<Adt57>(Variant(_128, 0), 2), 1), 4), 2), 4) };
(*_26) = !_70.2;
_179 = (_48.0, _48.1, Field::<Adt51>(Variant(_28, 0), 1).fld0.2, Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld0.3, _93);
place!(Field::<(i128, i128, i8)>(Variant(_180, 1), 1)).0 = _205.0;
_219.3 = _179.1;
_15 = Adt49::Variant2 { fld0: _56.1,fld1: _83,fld2: _38,fld3: _48.0,fld4: Field::<Adt51>(Variant(_29, 0), 1).fld2.2,fld5: Field::<u64>(Variant(_121, 0), 4),fld6: Field::<Adt51>(Variant(_29, 0), 1).fld1,fld7: _48 };
_221.fld0 = (_52, Field::<Adt50>(Variant(Field::<Adt54>(Variant(_47, 1), 0), 0), 3).fld0.1, _186);
_49 = Adt58::Variant0 { fld0: _155,fld1: Field::<Adt51>(Variant(_29, 0), 1),fld2: Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_15, 2), 7),fld3: Move(_150),fld4: _90,fld5: Move(_15) };
_77.2 = _131 - Field::<u64>(Variant(_121, 0), 4);
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_49, 0), 2)).1 = Field::<Adt51>(Variant(_28, 0), 1).fld0.1;
_179.4 = !_91.2;
place!(Field::<Adt51>(Variant(_49, 0), 1)).fld1 = Adt48::Variant1 { fld0: Field::<Adt51>(Variant(_29, 0), 1).fld0.1,fld1: _87,fld2: _38.0,fld3: Field::<[char; 6]>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 0),fld4: _5 };
Goto(bb92)
}
bb92 = {
place!(Field::<f32>(Variant(_161.fld1, 1), 2)) = Field::<f32>(Variant(_180, 1), 5);
_45 = Adt61 { fld0: Field::<(isize, f64, u16)>(Variant(_163, 1), 3).2,fld1: Move(Field::<Adt57>(Variant(_128, 0), 2)) };
_77.4 = Field::<i32>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 5) as f64;
_225 = _205.0 as i16;
_87 = _21;
_71.1 = -_89.3;
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld1, 2), 4)) = [_170,Field::<u8>(Variant(_163, 1), 1),Field::<u8>(Variant(_163, 1), 1),Field::<u8>(Variant(_163, 1), 1),_54,_170,_14,_14];
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld4 = core::ptr::addr_of!(_1);
_139 = _144;
_140 = core::ptr::addr_of_mut!(_73);
Goto(bb93)
}
bb93 = {
RET = Move(_45.fld1);
_161.fld0.0 = core::ptr::addr_of!(place!(Field::<(i16,)>(Variant(_29, 0), 4)).0);
place!(Field::<bool>(Variant(_180, 1), 0)) = (*_31) > _12;
_76 = Field::<*const *mut f64>(Variant(_50, 0), 0);
place!(Field::<[u16; 6]>(Variant(_28, 0), 2)) = [Field::<u16>(Variant(Field::<Adt49>(Variant(_49, 0), 5), 2), 4),_53,_16.2,_164.fld0,Field::<Adt51>(Variant(_29, 0), 1).fld2.2,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2];
_165 = -Field::<i32>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 5);
_164 = Adt61 { fld0: Field::<u16>(Variant(Field::<Adt49>(Variant(_49, 0), 5), 2), 4),fld1: Move(RET) };
_185.4 = -(*_99);
Goto(bb94)
}
bb94 = {
place!(Field::<Adt50>(Variant(_163, 1), 0)).fld0.0 = _172.fld0.0;
_205.1 = (_144, Field::<Adt51>(Variant(_29, 0), 1).fld5, _91.0);
place!(Field::<Adt56>(Variant(_47, 1), 7)).fld0 = _86.fld0 ^ (*_37);
place!(Field::<Adt51>(Variant(_121, 0), 2)).fld4 = core::ptr::addr_of!(_210.2);
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_49, 0), 2)).1 = _100 & Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt49>(Variant(_49, 0), 5), 2), 7).1;
SetDiscriminant(_188, 1);
Goto(bb95)
}
bb95 = {
_128 = Adt64::Variant1 { fld0: _14,fld1: Field::<char>(Variant(Field::<Adt49>(Variant(_49, 0), 5), 2), 1),fld2: Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.1 };
place!(Field::<*mut f64>(Variant(_180, 1), 6)) = Field::<*mut f64>(Variant(Field::<Adt59>(Variant(_50, 0), 4), 1), 0);
place!(Field::<u64>(Variant(_57, 1), 1)) = !_3;
_42.2 = Field::<i64>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 2);
place!(Field::<u64>(Variant(_29, 0), 3)) = _1 * Field::<u64>(Variant(_57, 1), 1);
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld0 = (_161.fld0.0, _22, _198.0, Field::<Adt51>(Variant(_28, 0), 1).fld0.3, _89.2);
_13.0 = Field::<i8>(Variant(Field::<Adt59>(Variant(_50, 0), 4), 1), 1) & _17;
place!(Field::<[isize; 8]>(Variant(_164.fld1, 1), 1)) = [_68,Field::<Adt51>(Variant(_29, 0), 1).fld2.0,_161.fld2.0,Field::<Adt51>(Variant(_49, 0), 1).fld2.0,_43,_160,Field::<Adt51>(Variant(_29, 0), 1).fld2.0,Field::<Adt51>(Variant(_121, 0), 2).fld2.0];
_25 = core::ptr::addr_of_mut!(_200.4);
_121 = Adt57::Variant0 { fld0: _112,fld1: Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt49>(Variant(_49, 0), 5), 2), 7).4,fld2: Field::<Adt51>(Variant(_29, 0), 1),fld3: _4,fld4: _120 };
Goto(bb96)
}
bb96 = {
_34 = Field::<Adt51>(Variant(_28, 0), 1).fld3.0;
_190 = Field::<u128>(Variant(_49, 0), 0) >> _88;
_179.4 = _91.2 + _41;
place!(Field::<Adt50>(Variant(place!(Field::<Adt54>(Variant(_47, 1), 0)), 0), 3)).fld0.1 = [_36,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld2.2,_45.fld0,_23,_109.2,Field::<Adt51>(Variant(_49, 0), 1).fld2.2];
_175 = Field::<Adt51>(Variant(_49, 0), 1).fld5 as u32;
_220 = _116;
_188 = Adt62::Variant1 { fld0: _102.1,fld1: Field::<[u8; 8]>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 4) };
Goto(bb97)
}
bb97 = {
_91 = (_205.1.2, Field::<Adt51>(Variant(_28, 0), 1).fld2.0, Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld0.4, Field::<Adt51>(Variant(_121, 0), 2).fld2.1);
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_49, 0), 2)).2 = _179.2;
place!(Field::<f32>(Variant(_47, 1), 6)) = Field::<Adt50>(Variant(_163, 1), 0).fld0.0;
_143 = Adt55::Variant2 { fld0: Field::<Adt50>(Variant(_163, 1), 0),fld1: Move(Field::<Adt49>(Variant(_49, 0), 5)),fld2: _117 };
_232 = _111;
place!(Field::<Adt50>(Variant(_163, 1), 0)) = _119;
place!(Field::<Adt50>(Variant(_143, 2), 0)).fld1 = Field::<[char; 6]>(Variant(Field::<Adt48>(Variant(_164.fld1, 1), 4), 2), 0);
_219 = (Field::<Adt51>(Variant(_49, 0), 1).fld2.1, _222, Field::<u64>(Variant(_29, 0), 3), Field::<Adt51>(Variant(_49, 0), 1).fld0.1, _185.0);
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld6 = Field::<Adt51>(Variant(_28, 0), 1).fld6 - Field::<Adt51>(Variant(_28, 0), 1).fld6;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld0 = (Field::<*const i16>(Variant(_164.fld1, 1), 5), Field::<bool>(Variant(_180, 1), 0), _161.fld0.2, Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 7).3, Field::<u32>(Variant(_121, 0), 1));
_200.3 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_121, 0), 3)));
_185.1 = [_34,Field::<Adt51>(Variant(_28, 0), 1).fld3.0,_130.0,_130.0,Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1),_81];
place!(Field::<[char; 6]>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld1, 2), 0)) = Field::<[char; 6]>(Variant(Field::<Adt51>(Variant(_49, 0), 1).fld1, 1), 3);
place!(Field::<[u64; 2]>(Variant(place!(Field::<Adt51>(Variant(_121, 0), 2)).fld1, 2), 1)) = [Field::<u64>(Variant(_149, 1), 1),Field::<u64>(Variant(_121, 0), 4)];
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld4 = core::ptr::addr_of!(_185.2);
place!(Field::<Adt51>(Variant(_49, 0), 1)).fld1 = Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1;
_70.1 = Field::<Adt51>(Variant(_121, 0), 2).fld2.0;
place!(Field::<i64>(Variant(_28, 0), 0)) = Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.2 as i64;
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld0.2 = Field::<[u64; 2]>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 1);
Goto(bb98)
}
bb98 = {
_167 = _146;
_200.3 = core::ptr::addr_of!(_221.fld0.2);
Goto(bb99)
}
bb99 = {
place!(Field::<[u8; 8]>(Variant(_164.fld1, 1), 2)) = Field::<[u8; 8]>(Variant(Field::<Adt51>(Variant(_49, 0), 1).fld1, 2), 4);
_127 = Field::<i32>(Variant(Field::<Adt48>(Variant(_164.fld1, 1), 4), 2), 5);
_19 = Field::<u8>(Variant(_163, 1), 1) as u32;
place!(Field::<u64>(Variant(_149, 1), 1)) = Field::<i32>(Variant(Field::<Adt48>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 6), 2), 5) as u64;
place!(Field::<u8>(Variant(_128, 1), 0)) = !_170;
_35 = [Field::<Adt51>(Variant(_28, 0), 1).fld0.4,_19,_103,_19,Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 7).4];
place!(Field::<([u64; 2], u128)>(Variant(_57, 1), 2)) = (Field::<Adt51>(Variant(_49, 0), 1).fld0.2, Field::<u128>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 0));
(*_94) = -(*_95);
_148 = _114;
_102.1.0 = -Field::<(i128, i128, i8)>(Variant(_180, 1), 1).0;
place!(Field::<Adt51>(Variant(_49, 0), 1)).fld2.0 = -Field::<Adt51>(Variant(_121, 0), 2).fld2.0;
_48.1 = _77.3;
place!(Field::<u64>(Variant(_57, 1), 1)) = !Field::<u64>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 3);
_230 = Field::<Adt51>(Variant(_28, 0), 1).fld2.0;
place!(Field::<u128>(Variant(_180, 1), 3)) = _198.1 >> _102.1.0;
_49 = Adt58::Variant1 { fld0: _172,fld1: _54,fld2: Field::<*const u64>(Variant(_163, 1), 2),fld3: _109 };
_28 = Adt55::Variant0 { fld0: Field::<i64>(Variant(Field::<Adt48>(Variant(_164.fld1, 1), 4), 2), 2),fld1: Field::<Adt51>(Variant(_29, 0), 1),fld2: _65,fld3: _120,fld4: _123 };
_161.fld4 = core::ptr::addr_of!(place!(Field::<u64>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 3)));
_206.fld0 = (Field::<f32>(Variant(_47, 1), 6), Field::<[u16; 6]>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 2), Field::<i64>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 2));
_206.fld1 = [_130.0,_232,_64,_64,_232,_135];
place!(Field::<Adt51>(Variant(_121, 0), 2)).fld0.4 = _12 * _147;
_119.fld0.1 = _172.fld0.1;
SetDiscriminant(Field::<Adt48>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 6), 2);
_136 = -_169;
place!(Field::<[char; 5]>(Variant(_47, 1), 4)) = [_161.fld3.0,Field::<char>(Variant(_128, 1), 1),_83,_111,Field::<char>(Variant(_128, 1), 1)];
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld2 = (Field::<Adt51>(Variant(_121, 0), 2).fld2.0, (*_118), Field::<Adt51>(Variant(_121, 0), 2).fld2.2);
place!(Field::<Adt51>(Variant(_121, 0), 2)).fld5 = Field::<(i128, i128, i8)>(Variant(_188, 1), 0).1;
place!(Field::<[u64; 2]>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld1, 2), 1)) = [_131,Field::<u64>(Variant(_121, 0), 4)];
_161.fld2.0 = Field::<(isize, f64, u16)>(Variant(_49, 1), 3).0 * Field::<Adt51>(Variant(_121, 0), 2).fld2.0;
_210.0 = _210.4 - Field::<Adt51>(Variant(_29, 0), 1).fld2.1;
Goto(bb100)
}
bb100 = {
place!(Field::<Adt50>(Variant(_143, 2), 0)).fld0 = (_42.0, _38.1, Field::<i64>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 2));
_36 = _71.2 >> Field::<i64>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 2);
_189.1 = _102.1.0;
place!(Field::<*const i128>(Variant(place!(Field::<Adt51>(Variant(_28, 0), 1)).fld1, 2), 3)) = Field::<*const i128>(Variant(Field::<Adt54>(Variant(_47, 1), 0), 0), 0);
_95 = core::ptr::addr_of_mut!(place!(Field::<(isize, f64, u16)>(Variant(_163, 1), 3)).1);
_23 = _100 as u16;
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt51>(Variant(_121, 0), 2)).fld1, 2), 4)) = [Field::<u8>(Variant(_49, 1), 1),Field::<u8>(Variant(_49, 1), 1),_170,Field::<u8>(Variant(_49, 1), 1),_54,Field::<u8>(Variant(_49, 1), 1),_14,_54];
place!(Field::<*const i128>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 6)), 2), 3)) = Field::<*const i128>(Variant(Field::<Adt48>(Variant(_164.fld1, 1), 4), 2), 3);
Call(_156 = core::intrinsics::transmute(Field::<i32>(Variant(Field::<Adt51>(Variant(_121, 0), 2).fld1, 2), 5)), bb101, UnwindUnreachable())
}
bb101 = {
place!(Field::<*mut f64>(Variant(_47, 1), 1)) = core::ptr::addr_of_mut!(_136);
_98 = Adt60::Variant1 { fld0: Move(Field::<Adt54>(Variant(_47, 1), 0)),fld1: Field::<*mut f64>(Variant(_180, 1), 6),fld2: Move(_28),fld3: _122,fld4: _7,fld5: Field::<i32>(Variant(Field::<Adt48>(Variant(_149, 1), 3), 2), 5),fld6: _38.0,fld7: Move(Field::<Adt56>(Variant(_47, 1), 7)) };
Goto(bb102)
}
bb102 = {
place!(Field::<*const i64>(Variant(_180, 1), 4)) = core::ptr::addr_of!(_119.fld0.2);
place!(Field::<i64>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 2), 2)) = _102.1.1 as i64;
_184 = Adt53::Variant1 { fld0: Field::<[isize; 8]>(Variant(_164.fld1, 1), 1),fld1: _142,fld2: Move(Field::<Adt52>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 0), 1)),fld3: _146,fld4: Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3),fld5: Field::<i32>(Variant(Field::<Adt48>(Variant(_149, 1), 3), 2), 5),fld6: Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3).0 };
_212 = -_156;
place!(Field::<Adt51>(Variant(_121, 0), 2)).fld6 = !Field::<Adt51>(Variant(_29, 0), 1).fld6;
place!(Field::<Adt56>(Variant(_98, 1), 7)).fld0 = _46.0 ^ _46.0;
_236.3 = -_210.0;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3)).0 = _161.fld5;
_195 = !Field::<u8>(Variant(_49, 1), 1);
place!(Field::<*const i128>(Variant(place!(Field::<Adt48>(Variant(_164.fld1, 1), 4)), 2), 3)) = core::ptr::addr_of!(place!(Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4)).0);
_238 = Field::<u64>(Variant(_29, 0), 3) ^ _1;
SetDiscriminant(Field::<Adt59>(Variant(_50, 0), 4), 0);
_102.1.0 = Field::<i128>(Variant(_184, 1), 6);
_206.fld0.0 = Field::<u8>(Variant(_163, 1), 1) as f32;
_121 = Adt57::Variant1 { fld0: _178,fld1: _220,fld2: Field::<[u8; 8]>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 4),fld3: _97.0,fld4: Field::<Adt48>(Variant(_149, 1), 3),fld5: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld0.0 };
_128 = Adt64::Variant2 { fld0: _187,fld1: _161.fld4,fld2: Move(_188),fld3: Field::<[u32; 5]>(Variant(_164.fld1, 1), 0),fld4: Field::<Adt51>(Variant(_29, 0), 1).fld1,fld5: _33,fld6: _42.2 };
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld2.1 = _71.1;
Goto(bb103)
}
bb103 = {
_200.0 = core::ptr::addr_of!(place!(Field::<Adt56>(Variant(_98, 1), 7)).fld0);
_161 = Adt51 { fld0: _48,fld1: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld1,fld2: _16,fld3: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld3,fld4: Field::<*const u64>(Variant(_49, 1), 2),fld5: Field::<(i128, i128, i8)>(Variant(Field::<Adt62>(Variant(_128, 2), 2), 1), 0).1,fld6: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld6 };
_223 = [_161.fld3.0];
place!(Field::<[i32; 1]>(Variant(_128, 2), 0)) = [Field::<i32>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_50, 0), 2), 0), 1).fld1, 2), 5)];
_54 = _122.1.2 as u8;
_209 = [_97.1,_68,_11];
place!(Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4)).1 = _102.1;
_236.2 = _73;
_106 = (Field::<(i16,)>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 4).0,);
_77.2 = Field::<u64>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 3) & _1;
_185.2 = !_3;
(*_40) = -Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4).1.0;
place!(Field::<i64>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 6)), 2), 2)) = Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld2.0 as i64;
_166 = (Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1),);
_92 = _110;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_50, 0), 2)), 0), 1)).fld3 = (_34,);
_59 = _58;
place!(Field::<(i16,)>(Variant(_29, 0), 4)).0 = -_146.0;
(*_112) = core::ptr::addr_of!(_144);
Goto(bb104)
}
bb104 = {
_88 = Field::<u64>(Variant(_29, 0), 3);
_185.3 = _126 & Field::<bool>(Variant(_180, 1), 0);
place!(Field::<Adt56>(Variant(_47, 1), 7)) = Move(_86);
SetDiscriminant(Field::<Adt55>(Variant(_50, 0), 2), 2);
place!(Field::<(i128, i128, i8)>(Variant(_180, 1), 1)) = (Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.0, Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4).0, _97.0);
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld6 = Field::<u64>(Variant(_57, 1), 1) as usize;
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 0), 1)).fld2.1 = _185.0;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4)) = (_205.1.1, _205.1);
_109.0 = -_97.1;
place!(Field::<Adt50>(Variant(_143, 2), 0)).fld0 = Field::<(f32, [u16; 6], i64)>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 2);
_16.1 = _13.3 * _91.3;
(*_112) = Field::<*const i128>(Variant(Field::<Adt48>(Variant(_128, 2), 4), 2), 3);
_221.fld0.0 = -Field::<f32>(Variant(_98, 1), 6);
SetDiscriminant(_149, 0);
_81 = _62.0;
_127 = Field::<i32>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 5);
SetDiscriminant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 3);
place!(Field::<Adt50>(Variant(_149, 0), 3)).fld0 = (_221.fld0.0, _42.1, _4);
_63 = Field::<(f32, [u16; 6], i64)>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 2).0;
Goto(bb105)
}
bb105 = {
_27 = [Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1)];
_206.fld0.0 = _10;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3)).1 = _122.1;
place!(Field::<bool>(Variant(_180, 1), 0)) = Field::<Adt51>(Variant(_29, 0), 1).fld6 >= Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld6;
place!(Field::<i64>(Variant(place!(Field::<Adt48>(Variant(_164.fld1, 1), 4)), 2), 2)) = Field::<Adt51>(Variant(_29, 0), 1).fld6 as i64;
place!(Field::<(isize, f64, u16)>(Variant(_49, 1), 3)).0 = _71.0 - _230;
place!(Field::<*const i128>(Variant(place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 0), 1)).fld1, 2), 3)) = core::ptr::addr_of!(place!(Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3)).0);
place!(Field::<u32>(Variant(_149, 0), 5)) = _179.4 + (*_31);
place!(Field::<i32>(Variant(_47, 1), 5)) = !Field::<i32>(Variant(Field::<Adt48>(Variant(_128, 2), 4), 2), 5);
Goto(bb106)
}
bb106 = {
_50 = Move(Field::<Adt62>(Variant(_128, 2), 2));
place!(Field::<i32>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 3), 1)) = Field::<i32>(Variant(_98, 1), 5);
_161.fld0.4 = _91.2 >> _152;
place!(Field::<([u64; 2], u128)>(Variant(_57, 1), 2)).0 = [_88,_238];
_62 = _130;
_128 = Adt64::Variant2 { fld0: _187,fld1: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld4,fld2: Move(_50),fld3: _178,fld4: _161.fld1,fld5: _76,fld6: Field::<i64>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 0) };
_206 = Field::<Adt50>(Variant(_143, 2), 0);
place!(Field::<Adt50>(Variant(_143, 2), 0)).fld0.2 = !Field::<i64>(Variant(Field::<Adt48>(Variant(_128, 2), 4), 2), 2);
_53 = _161.fld2.2;
_64 = Field::<Adt51>(Variant(_29, 0), 1).fld3.0;
place!(Field::<([u64; 2], u128)>(Variant(_57, 1), 2)).1 = _82 as u128;
_150 = Move(Field::<Adt52>(Variant(_184, 1), 2));
_118 = core::ptr::addr_of_mut!(_185.4);
place!(Field::<u64>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 0), 3)) = !_131;
place!(Field::<Adt54>(Variant(_98, 1), 0)) = Adt54::Variant1 { fld0: (*_76),fld1: _1,fld2: _8,fld3: Field::<Adt48>(Variant(_164.fld1, 1), 4) };
_200.1 = !_219.3;
Goto(bb107)
}
bb107 = {
_158 = _16.0 & Field::<(isize, f64, u16)>(Variant(_49, 1), 3).0;
place!(Field::<isize>(Variant(_149, 0), 2)) = _230 - _160;
place!(Field::<Adt51>(Variant(_29, 0), 1)) = Adt51 { fld0: Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 7),fld1: Field::<Adt48>(Variant(_164.fld1, 1), 4),fld2: Field::<(isize, f64, u16)>(Variant(_49, 1), 3),fld3: _166,fld4: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld4,fld5: Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).0,fld6: _161.fld6 };
place!(Field::<[char; 6]>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 3)), 2), 0)) = [Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1),Field::<Adt51>(Variant(_29, 0), 1).fld3.0,Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld3.0,_142,_130.0,Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1)];
_240 = _8.0;
_226 = [Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld0.4,_89.2,_91.2,_97.2,_12];
Goto(bb108)
}
bb108 = {
place!(Field::<Adt56>(Variant(_47, 1), 7)) = Adt56 { fld0: _183.0 };
_77.2 = _238 << _161.fld2.2;
_200 = (Field::<Adt51>(Variant(_29, 0), 1).fld0.0, _100, Field::<[u64; 2]>(Variant(Field::<Adt48>(Variant(_164.fld1, 1), 4), 2), 1), _48.3, (*_140));
place!(Field::<[char; 6]>(Variant(_161.fld1, 2), 0)) = [_161.fld3.0,Field::<char>(Variant(_184, 1), 1),_232,Field::<Adt51>(Variant(_29, 0), 1).fld3.0,_232,_111];
_91.2 = Field::<u32>(Variant(_149, 0), 5);
_156 = _63;
_231.3 = _161.fld0.1;
place!(Field::<isize>(Variant(_149, 0), 2)) = _71.0;
place!(Field::<(isize, f64, u16)>(Variant(_49, 1), 3)).1 = Field::<Adt51>(Variant(_29, 0), 1).fld6 as f64;
_189.0 = _161.fld5 << _73;
place!(Field::<Adt56>(Variant(_47, 1), 7)) = Adt56 { fld0: _72 };
_41 = !Field::<u32>(Variant(_149, 0), 5);
_163 = Adt58::Variant1 { fld0: _119,fld1: _195,fld2: _161.fld4,fld3: Field::<Adt51>(Variant(_29, 0), 1).fld2 };
_221.fld0.0 = -_214;
_239 = _22 & _161.fld0.1;
place!(Field::<*mut f64>(Variant(_180, 1), 6)) = _118;
_89.0 = _115.0;
_227 = !Field::<isize>(Variant(_149, 0), 2);
place!(Field::<(i128, i128, i8)>(Variant(place!(Field::<Adt62>(Variant(_128, 2), 2)), 1), 0)).1 = _205.1.0 & Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4).0;
SetDiscriminant(_164.fld1, 2);
_170 = _77.3 as u8;
_38 = Field::<Adt50>(Variant(_163, 1), 0).fld0;
place!(Field::<Adt50>(Variant(_143, 2), 0)).fld0.2 = Field::<Adt50>(Variant(_149, 0), 3).fld0.2;
Goto(bb109)
}
bb109 = {
place!(Field::<[isize; 8]>(Variant(_184, 1), 0)) = Field::<[isize; 8]>(Variant(_121, 1), 1);
place!(Field::<u8>(Variant(_49, 1), 1)) = _54 | _170;
_149 = Adt54::Variant1 { fld0: (*_33),fld1: _131,fld2: Field::<([u64; 2], u128)>(Variant(_57, 1), 2),fld3: Field::<Adt48>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 3) };
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld0 = (Field::<*const i16>(Variant(_121, 1), 5), _77.3, _198.0, Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 7).3, _236.2);
_236.0 = _115.0;
_103 = _88 as u32;
place!(Field::<(i16,)>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 0), 4)) = (Field::<Adt56>(Variant(_47, 1), 7).fld0,);
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 7)).2 = Field::<([u64; 2], u128)>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 2).0;
_185 = (_89.3, Field::<[char; 6]>(Variant(Field::<Adt48>(Variant(_128, 2), 4), 2), 0), _1, Field::<Adt51>(Variant(_29, 0), 1).fld0.1, _71.1);
_29 = Adt55::Variant0 { fld0: Field::<i64>(Variant(Field::<Adt48>(Variant(_149, 1), 3), 2), 2),fld1: Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1),fld2: Field::<Adt50>(Variant(_49, 1), 0).fld0.1,fld3: _1,fld4: _146 };
_161.fld2.1 = Field::<Adt51>(Variant(_29, 0), 1).fld2.1 - _13.3;
_164.fld1 = Move(_121);
place!(Field::<[u32; 5]>(Variant(_128, 2), 3)) = _35;
place!(Field::<*mut f64>(Variant(_57, 1), 0)) = core::ptr::addr_of_mut!(_192);
place!(Field::<Adt51>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 0), 1)).fld5 = -Field::<(i128, i128, i8)>(Variant(Field::<Adt62>(Variant(_128, 2), 2), 1), 0).1;
_108 = _135;
place!(Field::<(i128, i128, i8)>(Variant(place!(Field::<Adt62>(Variant(_128, 2), 2)), 1), 0)) = (Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4).0, Field::<(i128, i128, i8)>(Variant(_180, 1), 1).0, Field::<(i128, i128, i8)>(Variant(_180, 1), 1).2);
_207 = core::ptr::addr_of!(_205.1.1);
_210 = ((*_118), Field::<Adt50>(Variant(_143, 2), 0).fld1, _131, _161.fld0.1, _192);
_179.2 = [_185.2,Field::<u64>(Variant(_29, 0), 3)];
place!(Field::<[u64; 2]>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 6)), 2), 1)) = _198.0;
_216 = Adt60::Variant2 { fld0: _161.fld0.3,fld1: _74,fld2: _21,fld3: _166,fld4: _33,fld5: _102,fld6: _209,fld7: _219 };
_41 = _93 + _115.2;
_119.fld0.0 = _66;
_150 = Adt52 { fld0: _31 };
Goto(bb110)
}
bb110 = {
place!(Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3)).1.2 = _101;
_167.0 = Field::<(i16,)>(Variant(_29, 0), 4).0 & Field::<(i16,)>(Variant(_184, 1), 3).0;
_161.fld3 = _62;
_19 = _161.fld0.4 & _200.4;
_155 = !_20;
_263 = Adt55::Variant0 { fld0: _4,fld1: _161,fld2: _65,fld3: _88,fld4: Field::<(i16,)>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 4) };
place!(Field::<Adt52>(Variant(_184, 1), 2)).fld0 = core::ptr::addr_of_mut!((*_26));
place!(Field::<Adt50>(Variant(_49, 1), 0)).fld0.2 = Field::<i64>(Variant(Field::<Adt51>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 0), 1).fld1, 2), 2) & _42.2;
place!(Field::<Adt55>(Variant(_98, 1), 2)) = Adt55::Variant1 { fld0: _105,fld1: Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 7),fld2: _71.0,fld3: _101,fld4: _20 };
place!(Field::<Adt50>(Variant(_143, 2), 0)) = Adt50 { fld0: _38,fld1: Field::<[char; 6]>(Variant(Field::<Adt48>(Variant(_164.fld1, 1), 4), 2), 0) };
_133 = _64;
_44 = _51;
_38.1 = _42.1;
_157 = Field::<i32>(Variant(_47, 1), 5);
place!(Field::<[u64; 2]>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 2), 1)) = [_3,Field::<u64>(Variant(_57, 1), 1)];
_84 = Move(_163);
_268 = [Field::<Adt51>(Variant(_29, 0), 1).fld0.4,_48.4,_161.fld0.4,_73,_12];
place!(Field::<(i128, (i128, i128, i8))>(Variant(_216, 2), 5)) = (_205.1.0, Field::<(i128, i128, i8)>(Variant(Field::<Adt62>(Variant(_128, 2), 2), 1), 0));
_42.1 = _65;
_45.fld0 = _16.2 >> _101;
_261 = _179.0;
_236.2 = _152 as u32;
_149 = Adt54::Variant0 { fld0: _207,fld1: Move(Field::<Adt52>(Variant(_184, 1), 2)),fld2: _109.0,fld3: _206,fld4: _145,fld5: _41 };
_250 = (_108,);
_231 = (_185.0, _77.1, Field::<u64>(Variant(_29, 0), 3), _6, _136);
_114 = [_108,_133,Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1),_142,_83];
Goto(bb111)
}
bb111 = {
place!(Field::<i8>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 1), 3)) = !_152;
_38 = _42;
_152 = Field::<(i128, i128, i8)>(Variant(_180, 1), 1).2;
(*_168) = (*_112);
place!(Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3)).1.1 = _14 as i128;
_110 = _200.2;
SetDiscriminant(Field::<Adt54>(Variant(_98, 1), 0), 1);
(*_31) = _48.4 * Field::<Adt51>(Variant(_29, 0), 1).fld0.4;
(*_26) = _250.0 as u32;
(*_112) = core::ptr::addr_of!(_122.0);
place!(Field::<*mut f64>(Variant(_98, 1), 1)) = Field::<*mut f64>(Variant(_57, 1), 0);
_203 = [_146.0,_46.0,_146.0,_167.0,(*_261),_72];
Goto(bb112)
}
bb112 = {
_77.0 = _236.0 as f64;
place!(Field::<Adt50>(Variant(_143, 2), 0)) = Field::<Adt50>(Variant(_84, 1), 0);
_161.fld3.0 = Field::<char>(Variant(_184, 1), 1);
_48.1 = _219.3;
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld2.0 = _68;
_152 = _23 as i8;
place!(Field::<*const u64>(Variant(_128, 2), 1)) = core::ptr::addr_of!(place!(Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_216, 2), 7)).2);
_179.0 = Field::<*const i16>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 3);
_234 = [Field::<(i16,)>(Variant(_29, 0), 4).0,(*_261),Field::<(i16,)>(Variant(_184, 1), 3).0,_183.0,(*_37),_123.0];
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld4 = core::ptr::addr_of!(_88);
_259.fld0 = core::ptr::addr_of_mut!(_175);
place!(Field::<i8>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 1), 3)) = !_205.1.2;
_255 = Field::<[isize; 8]>(Variant(_164.fld1, 1), 1);
place!(Field::<(i128, i128, i8)>(Variant(place!(Field::<Adt62>(Variant(_128, 2), 2)), 1), 0)).0 = -Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.0;
place!(Field::<*mut f64>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 0)) = core::ptr::addr_of_mut!(_97.3);
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld4 = core::ptr::addr_of!(_131);
_17 = !_70.0;
place!(Field::<(isize, f64, u16)>(Variant(_84, 1), 3)) = (_68, (*_99), _36);
place!(Field::<Adt56>(Variant(_98, 1), 7)).fld0 = Field::<(i16,)>(Variant(_263, 0), 4).0 | _167.0;
_150 = Adt52 { fld0: _259.fld0 };
place!(Field::<Adt50>(Variant(_143, 2), 0)) = Adt50 { fld0: _221.fld0,fld1: _210.1 };
_243 = (_146.0,);
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld0.3 = _179.3;
place!(Field::<[i32; 8]>(Variant(_216, 2), 2)) = [_157,Field::<i32>(Variant(_98, 1), 5),Field::<i32>(Variant(_98, 1), 5),Field::<i32>(Variant(_184, 1), 5),Field::<i32>(Variant(Field::<Adt48>(Variant(_128, 2), 4), 2), 5),Field::<i32>(Variant(_184, 1), 5),Field::<i32>(Variant(_98, 1), 5),Field::<i32>(Variant(_98, 1), 5)];
Goto(bb113)
}
bb113 = {
place!(Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_216, 2), 7)).1 = [Field::<char>(Variant(_216, 2), 1),_108,_64,_135,Field::<Adt51>(Variant(_263, 0), 1).fld3.0,_135];
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld2.1 = _71.1;
_246 = (*_168);
_272.1 = ((*_246), (*_246), Field::<i8>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 1), 3));
place!(Field::<i32>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 2), 5)) = Field::<i32>(Variant(_98, 1), 5) * _157;
_206.fld0 = _172.fld0;
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld0.3 = _48.3;
_102.1.1 = (*_25) as i128;
_118 = core::ptr::addr_of_mut!(_91.3);
place!(Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_216, 2), 7)).1 = [_232,_142,_81,Field::<char>(Variant(_184, 1), 1),_69,_111];
place!(Field::<Adt50>(Variant(_49, 1), 0)).fld0.2 = Field::<i64>(Variant(_29, 0), 0);
Call(place!(Field::<(i128, i128, i8)>(Variant(_180, 1), 1)).0 = core::intrinsics::transmute(Field::<(i128, i128, i8)>(Variant(Field::<Adt62>(Variant(_128, 2), 2), 1), 0).0), bb114, UnwindUnreachable())
}
bb114 = {
place!(Field::<[u64; 2]>(Variant(_161.fld1, 2), 1)) = Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 1), 1).2;
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 7)).3 = _161.fld0.3;
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 2), 4)) = Field::<[u8; 8]>(Variant(Field::<Adt48>(Variant(_164.fld1, 1), 4), 2), 4);
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 7)).3 = core::ptr::addr_of!(place!(Field::<i64>(Variant(place!(Field::<Adt48>(Variant(_128, 2), 4)), 2), 2)));
place!(Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3)).0 = Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4).1.0 * _139;
place!(Field::<i64>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 2), 2)) = Field::<i64>(Variant(Field::<Adt48>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 6), 2), 2) << Field::<(i16,)>(Variant(_29, 0), 4).0;
place!(Field::<i32>(Variant(_161.fld1, 2), 5)) = Field::<i32>(Variant(_184, 1), 5);
_82 = Field::<u8>(Variant(_84, 1), 1) as i32;
_250 = (_64,);
_42 = Field::<(f32, [u16; 6], i64)>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 2);
_272 = (Field::<(i128, (i128, i128, i8))>(Variant(_216, 2), 5).1.1, Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1);
place!(Field::<[u64; 2]>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 2), 1)) = _56.0;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3)).1.0 = Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4).0;
_153 = _226;
_54 = _14 * _170;
_224.0 = _135;
place!(Field::<u128>(Variant(_149, 0), 4)) = _145 >> Field::<i32>(Variant(Field::<Adt48>(Variant(_128, 2), 4), 2), 5);
_86 = Move(Field::<Adt56>(Variant(_47, 1), 7));
_61 = Adt59::Variant1 { fld0: Field::<*mut f64>(Variant(_57, 1), 0),fld1: Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.2 };
_13.3 = -_231.0;
_66 = _46.0 as f32;
_230 = _16.0;
Goto(bb115)
}
bb115 = {
(*_76) = core::ptr::addr_of_mut!(_231.4);
(*_140) = !(*_31);
_248 = Field::<Adt50>(Variant(_49, 1), 0).fld0.2 as f32;
_250.0 = Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1);
_193 = _248 as u64;
_45 = Adt61 { fld0: _16.2,fld1: Move(_164.fld1) };
place!(Field::<*mut f64>(Variant(_61, 1), 0)) = core::ptr::addr_of_mut!(_219.4);
_92 = Field::<([u64; 2], u128)>(Variant(_57, 1), 2).0;
_177 = Adt53::Variant1 { fld0: _255,fld1: _111,fld2: Move(_259),fld3: _46,fld4: _272,fld5: _127,fld6: Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4).1.0 };
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 2)) = (_110, Field::<([u64; 2], u128)>(Variant(_57, 1), 2).1);
_58 = [_70.1,Field::<(isize, f64, u16)>(Variant(_84, 1), 3).0,_230];
Goto(bb116)
}
bb116 = {
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 2), 4)) = [Field::<u8>(Variant(_49, 1), 1),Field::<u8>(Variant(_49, 1), 1),_170,_170,_54,_170,_14,_54];
place!(Field::<*const i16>(Variant(_45.fld1, 1), 5)) = _37;
_57 = Adt54::Variant1 { fld0: _95,fld1: _238,fld2: _198,fld3: Field::<Adt48>(Variant(_45.fld1, 1), 4) };
_235 = !_230;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 2)).1 = Field::<i32>(Variant(_47, 1), 5) as u128;
_122.0 = (*_246);
place!(Field::<[i16; 6]>(Variant(_180, 1), 2)) = _234;
_23 = _16.2 ^ _164.fld0;
_179.4 = _115.2;
place!(Field::<i64>(Variant(place!(Field::<Adt48>(Variant(_45.fld1, 1), 4)), 2), 2)) = Field::<u64>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 5) as i64;
_96 = [_70.2,(*_31),_12,_161.fld0.4,(*_140)];
_196 = _71.0 * _70.1;
(*_25) = !_175;
_230 = Field::<Adt51>(Variant(_263, 0), 1).fld2.0 & _71.0;
_270 = _96;
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld2.0 = Field::<(isize, f64, u16)>(Variant(_84, 1), 3).0;
_70.0 = Field::<(i128, (i128, i128, i8))>(Variant(_47, 1), 3).1.2;
place!(Field::<(i128, i128, i8)>(Variant(place!(Field::<Adt62>(Variant(_128, 2), 2)), 1), 0)).1 = _190 as i128;
place!(Field::<(i16,)>(Variant(_177, 1), 3)) = (_225,);
Call(_66 = core::intrinsics::transmute(_82), bb117, UnwindUnreachable())
}
bb117 = {
_179 = (_37, _161.fld0.1, Field::<[u64; 2]>(Variant(Field::<Adt51>(Variant(_263, 0), 1).fld1, 2), 1), Field::<*const i64>(Variant(_216, 2), 0), _175);
_161.fld6 = _175 as usize;
_16 = Field::<(isize, f64, u16)>(Variant(_84, 1), 3);
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 6)), 2), 4)) = Field::<[u8; 8]>(Variant(Field::<Adt51>(Variant(_263, 0), 1).fld1, 2), 4);
_102.1.1 = Field::<f32>(Variant(_98, 1), 6) as i128;
_16.2 = _161.fld2.2 | _109.2;
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt48>(Variant(_128, 2), 4)), 2), 4)) = [_14,_54,_14,_170,_54,_195,Field::<u8>(Variant(_49, 1), 1),Field::<u8>(Variant(_84, 1), 1)];
_231.1 = [Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1),Field::<char>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 1),_74,_69,_69,_135];
_212 = _113 + _38.0;
_11 = Field::<isize>(Variant(Field::<Adt55>(Variant(_98, 1), 2), 1), 2);
_76 = _33;
place!(Field::<*const *mut f64>(Variant(_128, 2), 5)) = core::ptr::addr_of!(_118);
_238 = Field::<u64>(Variant(_29, 0), 3) >> _16.0;
place!(Field::<*const i128>(Variant(place!(Field::<Adt48>(Variant(_57, 1), 3)), 2), 3)) = core::ptr::addr_of!((*_246));
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt48>(Variant(_57, 1), 3)), 2), 4)) = Field::<[u8; 8]>(Variant(_45.fld1, 1), 2);
place!(Field::<u128>(Variant(_180, 1), 3)) = _190 * Field::<([u64; 2], u128)>(Variant(_57, 1), 2).1;
SetDiscriminant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 0);
place!(Field::<(i16,)>(Variant(_177, 1), 3)) = (_243.0,);
_231.4 = _115.3 * Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_216, 2), 7).0;
Goto(bb118)
}
bb118 = {
_82 = Field::<i32>(Variant(_47, 1), 5) & Field::<i32>(Variant(_98, 1), 5);
_174 = !_219.3;
_231.3 = _179.1 | _48.1;
place!(Field::<char>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 1)) = _74;
place!(Field::<Adt48>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 3)) = _161.fld1;
_18 = [_250.0];
_161.fld2 = (_115.1, Field::<Adt51>(Variant(_263, 0), 1).fld2.1, Field::<u16>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 4));
_25 = core::ptr::addr_of_mut!(_97.2);
_224.0 = _142;
_234 = _117;
place!(Field::<Adt50>(Variant(_49, 1), 0)).fld0.0 = _91.2 as f32;
Goto(bb119)
}
bb119 = {
place!(Field::<(f32, [u16; 6], i64)>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 2)).1 = [Field::<Adt51>(Variant(_263, 0), 1).fld2.2,Field::<(isize, f64, u16)>(Variant(_84, 1), 3).2,_161.fld2.2,_164.fld0,Field::<Adt51>(Variant(_29, 0), 1).fld2.2,Field::<u16>(Variant(Field::<Adt49>(Variant(_143, 2), 1), 2), 4)];
_146 = _243;
Goto(bb120)
}
bb120 = {
(*_261) = !_123.0;
place!(Field::<Adt50>(Variant(_149, 0), 3)).fld0 = (_66, _119.fld0.1, Field::<i64>(Variant(Field::<Adt48>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 3), 2), 2));
_91.2 = _236.2;
SetDiscriminant(Field::<Adt51>(Variant(_263, 0), 1).fld1, 3);
SetDiscriminant(_49, 0);
_292.1.0 = !(*_171);
_278 = Field::<Adt51>(Variant(_263, 0), 1).fld0.1 ^ Field::<Adt51>(Variant(_29, 0), 1).fld0.1;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_177, 1), 4)).1.0 = Field::<(i128, i128, i8)>(Variant(_180, 1), 1).1;
SetDiscriminant(Field::<Adt48>(Variant(_57, 1), 3), 1);
place!(Field::<[char; 6]>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 6)), 2), 0)) = _172.fld1;
place!(Field::<*const i64>(Variant(_180, 1), 4)) = core::ptr::addr_of!(_206.fld0.2);
_259 = Adt52 { fld0: _140 };
Goto(bb121)
}
bb121 = {
_292.1.2 = _205.1.2;
place!(Field::<bool>(Variant(place!(Field::<Adt48>(Variant(_57, 1), 3)), 1), 0)) = _170 >= _195;
place!(Field::<Adt48>(Variant(place!(Field::<Adt49>(Variant(_143, 2), 1)), 2), 6)) = Adt48::Variant3 { fld0: (*_112),fld1: Field::<i32>(Variant(_161.fld1, 2), 5),fld2: Field::<[i32; 1]>(Variant(_128, 2), 0),fld3: Field::<[i16; 6]>(Variant(_180, 1), 2) };
place!(Field::<Adt48>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 3)) = Adt48::Variant2 { fld0: Field::<Adt50>(Variant(_143, 2), 0).fld1,fld1: Field::<([u64; 2], u128)>(Variant(_57, 1), 2).0,fld2: _186,fld3: (*_112),fld4: Field::<[u8; 8]>(Variant(_45.fld1, 1), 2),fld5: Field::<i32>(Variant(_177, 1), 5) };
place!(Field::<Adt50>(Variant(_143, 2), 0)) = Adt50 { fld0: _206.fld0,fld1: Field::<[char; 6]>(Variant(Field::<Adt48>(Variant(_45.fld1, 1), 4), 2), 0) };
Goto(bb122)
}
bb122 = {
(*_140) = (*_25);
SetDiscriminant(Field::<Adt49>(Variant(_143, 2), 1), 2);
_161.fld3 = Field::<Adt51>(Variant(_263, 0), 1).fld3;
_172.fld0.2 = Field::<i64>(Variant(Field::<Adt48>(Variant(_45.fld1, 1), 4), 2), 2);
place!(Field::<Adt54>(Variant(_47, 1), 0)) = Move(_149);
_179.3 = core::ptr::addr_of!(_42.2);
_109.1 = _185.4 * _219.4;
place!(Field::<[isize; 3]>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 0), 0)) = [_124,_161.fld2.0,Field::<(isize, f64, u16)>(Variant(_84, 1), 3).0];
_115.2 = (*_140);
place!(Field::<(i128, (i128, i128, i8))>(Variant(_177, 1), 4)).1 = Field::<(i128, i128, i8)>(Variant(Field::<Adt62>(Variant(_128, 2), 2), 1), 0);
_16.2 = !Field::<(isize, f64, u16)>(Variant(_84, 1), 3).2;
_284.4 = _236.3 - (*_99);
_143 = Move(Field::<Adt55>(Variant(_98, 1), 2));
_185.4 = -Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_216, 2), 7).0;
_198.0 = _48.2;
Goto(bb123)
}
bb123 = {
place!(Field::<f32>(Variant(_47, 1), 6)) = _195 as f32;
_34 = _81;
_231.2 = _88;
place!(Field::<Adt52>(Variant(_49, 0), 3)) = Move(_259);
SetDiscriminant(_161.fld1, 0);
_115.0 = -_70.0;
_47 = Move(_216);
_97.3 = _9 - _185.0;
(*_140) = (*_25) ^ (*_25);
_218 = _210.0 as u32;
(*_37) = _91.2 as i16;
_46.0 = Field::<(i16,)>(Variant(_177, 1), 3).0 - _154.fld0;
_275 = _6;
_237 = _224;
SetDiscriminant(_47, 2);
Goto(bb124)
}
bb124 = {
_210.3 = !Field::<bool>(Variant(Field::<Adt48>(Variant(_57, 1), 3), 1), 0);
place!(Field::<Adt52>(Variant(_184, 1), 2)) = Move(_150);
_258 = Adt55::Variant1 { fld0: Field::<usize>(Variant(_143, 1), 0),fld1: Field::<Adt51>(Variant(_29, 0), 1).fld0,fld2: _109.0,fld3: Field::<(i128, i128, i8)>(Variant(Field::<Adt62>(Variant(_128, 2), 2), 1), 0).2,fld4: _56.1 };
_221.fld0.0 = _137 as f32;
place!(Field::<u128>(Variant(_49, 0), 0)) = _52 as u128;
place!(Field::<(i16,)>(Variant(_184, 1), 3)).0 = Field::<Adt56>(Variant(_98, 1), 7).fld0;
place!(Field::<Adt51>(Variant(_49, 0), 1)).fld3 = _166;
place!(Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7)).1 = _222;
_121 = Move(_45.fld1);
_182 = -Field::<Adt51>(Variant(_263, 0), 1).fld2.1;
_67 = Adt62::Variant2 { fld0: Field::<(i16,)>(Variant(_29, 0), 4) };
_158 = Field::<Adt51>(Variant(_263, 0), 1).fld3.0 as isize;
place!(Field::<[u8; 8]>(Variant(_121, 1), 2)) = [Field::<u8>(Variant(_84, 1), 1),_195,_14,_170,_170,Field::<u8>(Variant(_84, 1), 1),_54,_54];
place!(Field::<[char; 6]>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 3)), 2), 0)) = Field::<[char; 6]>(Variant(Field::<Adt48>(Variant(_121, 1), 4), 2), 0);
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld2.0 = _196 | _97.1;
Goto(bb125)
}
bb125 = {
place!(Field::<*const u64>(Variant(_84, 1), 2)) = core::ptr::addr_of!(_210.2);
place!(Field::<Adt51>(Variant(_49, 0), 1)).fld4 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_29, 0), 3)));
Call(place!(Field::<Adt51>(Variant(_49, 0), 1)).fld2.1 = core::intrinsics::fmaf64(_169, _185.4, _136), bb126, UnwindUnreachable())
}
bb126 = {
_244 = Field::<Adt51>(Variant(_29, 0), 1).fld3.0;
place!(Field::<[u16; 6]>(Variant(_29, 0), 2)) = [_164.fld0,_109.2,_71.2,_109.2,_36,_45.fld0];
_278 = _219.3;
_221 = Adt50 { fld0: _206.fld0,fld1: _210.1 };
place!(Field::<i8>(Variant(_121, 1), 3)) = _13.0;
place!(Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7)).0 = _210.4 - _161.fld2.1;
_99 = core::ptr::addr_of_mut!(_9);
place!(Field::<[isize; 8]>(Variant(_121, 1), 1)) = _116;
_164 = Adt61 { fld0: _161.fld2.2,fld1: Move(_121) };
(*_207) = _102.1.0;
(*_207) = Field::<Adt51>(Variant(_263, 0), 1).fld2.0 as i128;
place!(Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7)) = (Field::<Adt51>(Variant(_49, 0), 1).fld2.1, _172.fld1, _231.2, _100, _185.4);
place!(Field::<(i128, i128, i8)>(Variant(place!(Field::<Adt62>(Variant(_128, 2), 2)), 1), 0)).2 = Field::<i32>(Variant(Field::<Adt48>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 3), 2), 5) as i8;
(*_33) = core::ptr::addr_of_mut!(place!(Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7)).0);
_121 = Move(_164.fld1);
_281 = _219.3 & _22;
_194 = _16.0;
_124 = Field::<Adt56>(Variant(_98, 1), 7).fld0 as isize;
place!(Field::<i128>(Variant(_177, 1), 6)) = -(*_207);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 2)) = (_240, _198.1);
Goto(bb127)
}
bb127 = {
_256 = _89.3 < _210.0;
place!(Field::<char>(Variant(_177, 1), 1)) = _64;
_80 = _59;
_315.1.1 = !Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3).1.0;
_130 = (_69,);
_164.fld0 = _71.2 + _23;
_213 = _174;
_78 = _131 as i64;
_17 = !Field::<(i128, i128, i8)>(Variant(Field::<Adt62>(Variant(_128, 2), 2), 1), 0).2;
place!(Field::<(i8, isize, u32, f64)>(Variant(_161.fld1, 0), 3)).1 = _71.0;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 0), 3)).1 = _105 as isize;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_47, 2), 5)).1.0 = _88 as i128;
_48.4 = _91.3 as u32;
_277 = [_62.0,_83,_161.fld3.0,_142,_133];
_241 = [Field::<i32>(Variant(_184, 1), 5)];
place!(Field::<[u64; 2]>(Variant(place!(Field::<Adt48>(Variant(_121, 1), 4)), 2), 1)) = [Field::<u64>(Variant(_29, 0), 3),_131];
_85 = _109.0;
_134 = Adt48::Variant2 { fld0: _206.fld1,fld1: Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_143, 1), 1).2,fld2: _78,fld3: Field::<*const i128>(Variant(Field::<Adt48>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 3), 2), 3),fld4: Field::<[u8; 8]>(Variant(_121, 1), 2),fld5: Field::<i32>(Variant(Field::<Adt48>(Variant(_121, 1), 4), 2), 5) };
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld2.1 = _231.4 - _71.1;
place!(Field::<Adt55>(Variant(_98, 1), 2)) = Adt55::Variant3 { fld0: _77.4 };
_13.2 = _232 as u32;
_17 = _231.2 as i8;
_294 = _23;
Goto(bb128)
}
bb128 = {
place!(Field::<i32>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 3)), 2), 5)) = Field::<i32>(Variant(_177, 1), 5);
place!(Field::<(i128, (i128, i128, i8))>(Variant(_177, 1), 4)).1 = (_189.1, _139, _102.1.2);
_115.3 = _115.1 as f64;
(*_99) = _16.1 * _219.4;
_301 = _16.0;
_49 = Move(_84);
place!(Field::<Adt52>(Variant(_177, 1), 2)).fld0 = core::ptr::addr_of_mut!((*_26));
_293.0 = [_238,_77.2];
_118 = core::ptr::addr_of_mut!(_136);
_138 = _71.1 * _210.4;
Goto(bb129)
}
bb129 = {
place!(Field::<Adt56>(Variant(_98, 1), 7)) = Adt56 { fld0: _167.0 };
_311 = _21;
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_258, 1), 1)) = _48;
Goto(bb130)
}
bb130 = {
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 0), 1)).0 = [_219.2,_238];
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1 = Field::<Adt48>(Variant(_121, 1), 4);
_221.fld1 = [_83,_133,_224.0,_111,_237.0,_74];
_284 = (_231.0, _77.1, _210.2, Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7).3, (*_99));
_97 = (_101, Field::<(i8, isize, u32, f64)>(Variant(_161.fld1, 0), 3).1, (*_140), _210.0);
_317 = _230 ^ _43;
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld3 = _130;
place!(Field::<Adt51>(Variant(_29, 0), 1)).fld5 = Field::<(i128, i128, i8)>(Variant(_180, 1), 1).0;
place!(Field::<Adt51>(Variant(_263, 0), 1)) = Field::<Adt51>(Variant(_29, 0), 1);
_161.fld2.1 = -(*_94);
_47 = Adt60::Variant0 { fld0: Field::<Adt48>(Variant(_128, 2), 4),fld1: _261 };
place!(Field::<*const i64>(Variant(_180, 1), 4)) = core::ptr::addr_of!(place!(Field::<i64>(Variant(_128, 2), 6)));
place!(Field::<bool>(Variant(_180, 1), 0)) = _77.3;
SetDiscriminant(Field::<Adt48>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 3), 2);
_276 = (_70.1, _97.3, _109.2);
place!(Field::<Adt50>(Variant(_49, 1), 0)).fld0.2 = _221.fld0.2;
SetDiscriminant(_184, 0);
_304 = _119.fld0.0;
_206.fld0.2 = Field::<i64>(Variant(Field::<Adt48>(Variant(_128, 2), 4), 2), 2) & Field::<i64>(Variant(_128, 2), 6);
_315.1.2 = _115.0 | _236.0;
_167.0 = Field::<(i16,)>(Variant(_67, 2), 0).0 | (*_37);
_326 = core::ptr::addr_of!(_118);
place!(Field::<i64>(Variant(_29, 0), 0)) = Field::<i64>(Variant(Field::<Adt48>(Variant(_128, 2), 4), 2), 2);
Goto(bb131)
}
bb131 = {
SetDiscriminant(Field::<Adt55>(Variant(_98, 1), 2), 3);
SetDiscriminant(_121, 2);
place!(Field::<*mut f64>(Variant(_180, 1), 6)) = core::ptr::addr_of_mut!(_137);
place!(Field::<u64>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 1)) = Field::<u64>(Variant(_29, 0), 3);
_161.fld0.1 = Field::<(i16,)>(Variant(_67, 2), 0).0 > (*_37);
_286 = [(*_37),_167.0,_146.0,(*_37),_72,Field::<(i16,)>(Variant(_263, 0), 4).0];
place!(Field::<Adt48>(Variant(_184, 0), 6)) = Field::<Adt51>(Variant(_263, 0), 1).fld1;
_27 = [_74];
_155 = !_20;
_89.1 = -Field::<(i8, isize, u32, f64)>(Variant(_161.fld1, 0), 3).1;
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld0.0 = core::ptr::addr_of!(_183.0);
_97.2 = _71.0 as u32;
_206.fld0 = _42;
_138 = -_219.4;
_264 = Field::<bool>(Variant(Field::<Adt48>(Variant(_57, 1), 3), 1), 0) ^ _200.1;
place!(Field::<i32>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 2), 5)) = _206.fld0.2 as i32;
_283 = _37;
place!(Field::<(i128, i128, i8)>(Variant(_180, 1), 1)).2 = !_70.0;
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_258, 1), 1)).0 = core::ptr::addr_of!(_297.0);
_300 = [_46.0,_183.0,Field::<Adt56>(Variant(_98, 1), 7).fld0,_86.fld0,Field::<(i16,)>(Variant(_177, 1), 3).0,_243.0];
(*_33) = core::ptr::addr_of_mut!(_169);
SetDiscriminant(Field::<Adt51>(Variant(_263, 0), 1).fld1, 0);
_230 = _317;
Goto(bb132)
}
bb132 = {
SetDiscriminant(Field::<Adt48>(Variant(_128, 2), 4), 1);
_42.2 = _93 as i64;
_233 = Field::<*const i128>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 3);
_328 = Field::<([u64; 2], u128)>(Variant(_57, 1), 2).1;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 2)).1 = _45.fld0 as u128;
_76 = core::ptr::addr_of!(_99);
SetDiscriminant(_177, 0);
_247 = Field::<(i128, i128, i8)>(Variant(Field::<Adt62>(Variant(_128, 2), 2), 1), 0).0 as f32;
place!(Field::<*const i128>(Variant(place!(Field::<Adt48>(Variant(_184, 0), 6)), 2), 3)) = _233;
Call(_210.0 = core::intrinsics::transmute(Field::<usize>(Variant(_143, 1), 0)), bb133, UnwindUnreachable())
}
bb133 = {
_167.0 = _86.fld0 << _1;
_276 = Field::<Adt51>(Variant(_263, 0), 1).fld2;
place!(Field::<*const u64>(Variant(_49, 1), 2)) = core::ptr::addr_of!(_1);
Goto(bb134)
}
bb134 = {
_38.1 = [_36,_161.fld2.2,_164.fld0,_276.2,_164.fld0,_276.2];
_280 = [_68,_85,_109.0];
place!(Field::<*mut f64>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 0)) = (*_76);
place!(Field::<[u8; 8]>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 2), 4)) = [_195,_170,Field::<u8>(Variant(_49, 1), 1),_54,_195,_54,_14,_170];
place!(Field::<u128>(Variant(_258, 1), 4)) = _179.4 as u128;
_66 = _97.1 as f32;
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_121, 2), 3)).1 = !_179.1;
Call(_20 = core::intrinsics::transmute(Field::<u128>(Variant(_258, 1), 4)), bb135, UnwindUnreachable())
}
bb135 = {
place!(Field::<*mut *const i128>(Variant(_121, 2), 1)) = core::ptr::addr_of_mut!(_40);
_302 = Field::<(i8, isize, u32, f64)>(Variant(_161.fld1, 0), 3).1 >> _1;
_172 = Adt50 { fld0: Field::<Adt50>(Variant(_49, 1), 0).fld0,fld1: _119.fld1 };
_130 = (_64,);
place!(Field::<(i8, isize, u32, f64)>(Variant(_161.fld1, 0), 3)).2 = Field::<Adt51>(Variant(_263, 0), 1).fld2.1 as u32;
_190 = Field::<u128>(Variant(_258, 1), 4);
place!(Field::<f32>(Variant(place!(Field::<Adt48>(Variant(_128, 2), 4)), 1), 2)) = -_304;
_336 = core::ptr::addr_of!(_315.0);
Goto(bb136)
}
bb136 = {
SetDiscriminant(Field::<Adt48>(Variant(_47, 0), 0), 3);
_109.1 = -_276.1;
place!(Field::<bool>(Variant(place!(Field::<Adt48>(Variant(_128, 2), 4)), 1), 0)) = !_213;
_183.0 = (*_283);
place!(Field::<(i128, i128, i8)>(Variant(_180, 1), 1)).0 = _272.0;
_115.1 = _235 - _235;
place!(Field::<[char; 6]>(Variant(place!(Field::<Adt51>(Variant(_29, 0), 1)).fld1, 2), 0)) = [_83,_2,_224.0,Field::<Adt51>(Variant(_263, 0), 1).fld3.0,_135,_133];
place!(Field::<(i128, i128, i8)>(Variant(_177, 0), 1)) = (Field::<(i128, i128, i8)>(Variant(_180, 1), 1).0, _189.0, Field::<i8>(Variant(_258, 1), 3));
place!(Field::<(i128, i128, i8)>(Variant(_184, 0), 1)).0 = _102.1.1;
Goto(bb137)
}
bb137 = {
place!(Field::<*const u64>(Variant(_128, 2), 1)) = core::ptr::addr_of!(_185.2);
_221.fld0 = (_181, _172.fld0.1, Field::<i64>(Variant(_263, 0), 0));
_314 = _125;
SetDiscriminant(_143, 2);
_206.fld0.2 = _221.fld0.2;
_332 = _93;
(*_132) = core::ptr::addr_of!((*_336));
place!(Field::<[i16; 6]>(Variant(_143, 2), 2)) = [_46.0,(*_37),_46.0,(*_283),(*_37),Field::<(i16,)>(Variant(_263, 0), 4).0];
_161.fld0.4 = (*_31);
place!(Field::<(i128, i128, i8)>(Variant(_184, 0), 1)).1 = -_272.1.1;
_331 = (_152, _11, _147, (*_94));
_122 = (_205.1.1, Field::<(i128, i128, i8)>(Variant(_180, 1), 1));
_219.4 = (*_99) + _109.1;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 0), 3)).0 = _17;
_337 = _145;
place!(Field::<[i32; 8]>(Variant(place!(Field::<Adt48>(Variant(_128, 2), 4)), 1), 1)) = [_157,_165,Field::<i32>(Variant(_98, 1), 5),Field::<i32>(Variant(_134, 2), 5),Field::<i32>(Variant(Field::<Adt51>(Variant(_29, 0), 1).fld1, 2), 5),_165,_157,_82];
_77.1 = [_166.0,_250.0,_62.0,_74,_81,_237.0];
_13.2 = _82 as u32;
_315.1 = Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3).1;
_135 = _111;
_89 = _13;
place!(Field::<*const *mut f64>(Variant(place!(Field::<Adt48>(Variant(_128, 2), 4)), 1), 4)) = _5;
_129 = _200.0;
Goto(bb138)
}
bb138 = {
_123 = _146;
SetDiscriminant(_134, 1);
_281 = _179.1;
_210.3 = _48.1;
place!(Field::<Adt50>(Variant(_143, 2), 0)).fld0.0 = _91.3 as f32;
_318 = core::ptr::addr_of!(_86.fld0);
_205.1.2 = _89.0;
SetDiscriminant(_29, 1);
_118 = core::ptr::addr_of_mut!((*_94));
_308 = Field::<(isize, f64, u16)>(Variant(_49, 1), 3).2 != _71.2;
_270 = [Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_258, 1), 1).4,_41,(*_31),_97.2,(*_25)];
_292.1 = (_144, _189.1, _236.0);
_198.0 = _240;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3)).1.2 = !Field::<i8>(Variant(_61, 1), 1);
_292 = ((*_207), _315.1);
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld0.2 = [Field::<u64>(Variant(_57, 1), 1),_284.2];
place!(Field::<*mut f64>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 0)) = core::ptr::addr_of_mut!(_136);
_282 = [_231.2,Field::<u64>(Variant(_263, 0), 3)];
_59 = _280;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 0), 1)).1 = !_56.1;
_14 = _16.0 as u8;
_179.4 = !_332;
place!(Field::<[u16; 6]>(Variant(_263, 0), 2)) = [_36,_276.2,Field::<Adt51>(Variant(_263, 0), 1).fld2.2,_109.2,_276.2,_109.2];
Goto(bb139)
}
bb139 = {
_172.fld0.0 = _113 - Field::<f32>(Variant(_98, 1), 6);
(*_33) = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 3), 0)));
_231.0 = _331.3;
place!(Field::<Adt50>(Variant(_184, 0), 3)).fld1 = [_161.fld3.0,_34,_111,_250.0,_135,_34];
place!(Field::<Adt50>(Variant(_177, 0), 3)).fld0.1 = _65;
_348 = _52;
_4 = _236.0 as i64;
SetDiscriminant(Field::<Adt48>(Variant(_184, 0), 6), 3);
_161.fld3.0 = _62.0;
place!(Field::<i32>(Variant(_177, 0), 5)) = Field::<Adt51>(Variant(_263, 0), 1).fld0.4 as i32;
place!(Field::<*const u64>(Variant(_128, 2), 1)) = core::ptr::addr_of!(place!(Field::<u64>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 1)));
_38.2 = Field::<Adt50>(Variant(_49, 1), 0).fld0.2 >> _101;
_79 = Field::<(i128, i128, i8)>(Variant(_177, 0), 1).0 as i8;
place!(Field::<i64>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 3)), 2), 2)) = Field::<i64>(Variant(_128, 2), 6);
_327 = Move(_61);
place!(Field::<i32>(Variant(_177, 0), 5)) = _281 as i32;
_159 = [_130.0];
place!(Field::<bool>(Variant(_184, 0), 0)) = _239;
Goto(bb140)
}
bb140 = {
place!(Field::<Adt48>(Variant(_128, 2), 4)) = Adt48::Variant3 { fld0: _171,fld1: _165,fld2: _241,fld3: _300 };
_119.fld1 = [_62.0,_108,_130.0,_83,_232,_166.0];
_142 = _232;
_99 = core::ptr::addr_of_mut!((*_118));
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1 = Adt48::Variant3 { fld0: _171,fld1: _127,fld2: _32,fld3: Field::<[i16; 6]>(Variant(_180, 1), 2) };
_180 = Adt49::Variant1 { fld0: _284.3,fld1: _122.1,fld2: Field::<[i16; 6]>(Variant(_143, 2), 2),fld3: _328,fld4: _48.3,fld5: _42.0,fld6: (*_5),fld7: _168 };
_202 = _182 * (*_94);
place!(Field::<[i32; 1]>(Variant(place!(Field::<Adt48>(Variant(_184, 0), 6)), 3), 2)) = [_165];
Call(_169 = core::intrinsics::transmute(Field::<(isize, f64, u16)>(Variant(_49, 1), 3).0), bb141, UnwindUnreachable())
}
bb141 = {
_341.2 = _13.2 | _236.2;
_344 = Adt64::Variant1 { fld0: _195,fld1: _237.0,fld2: Field::<(i128, i128, i8)>(Variant(_177, 0), 1).0 };
_278 = Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_121, 2), 3).1 & _48.1;
_257 = Adt60::Variant2 { fld0: _179.3,fld1: _161.fld3.0,fld2: _21,fld3: _130,fld4: _5,fld5: _102,fld6: _280,fld7: _219 };
place!(Field::<[u64; 2]>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 3)), 2), 1)) = [Field::<u64>(Variant(_57, 1), 1),Field::<u64>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 1)];
(*_76) = core::ptr::addr_of_mut!(_71.1);
place!(Field::<Adt50>(Variant(_184, 0), 3)).fld0.2 = !_38.2;
_343.1.1 = Field::<Adt51>(Variant(_263, 0), 1).fld0.1 as i128;
SetDiscriminant(_128, 0);
_323 = core::ptr::addr_of_mut!((*_168));
place!(Field::<Adt50>(Variant(_143, 2), 0)).fld0.1 = [Field::<Adt51>(Variant(_263, 0), 1).fld2.2,_45.fld0,_45.fld0,Field::<Adt51>(Variant(_263, 0), 1).fld2.2,_53,_109.2];
Goto(bb142)
}
bb142 = {
_228 = Move(_344);
_135 = Field::<Adt51>(Variant(_263, 0), 1).fld3.0;
(*_26) = _126 as u32;
_288 = _284.3;
_122.1.2 = Field::<u128>(Variant(_180, 1), 3) as i8;
SetDiscriminant(_67, 2);
_156 = _214;
place!(Field::<Adt50>(Variant(_184, 0), 3)) = Adt50 { fld0: _119.fld0,fld1: _284.1 };
_197 = _123.0 as i128;
_90 = [_91.1,_43,Field::<(i8, isize, u32, f64)>(Variant(_161.fld1, 0), 3).1];
_93 = _195 as u32;
_138 = -_276.1;
_321 = Field::<u8>(Variant(_228, 1), 0) >= _54;
SetDiscriminant(_257, 0);
_310 = [_146.0,(*_318),(*_283),(*_37),_167.0,_46.0];
place!(Field::<[i16; 6]>(Variant(place!(Field::<Adt48>(Variant(_184, 0), 6)), 3), 3)) = [Field::<(i16,)>(Variant(_263, 0), 4).0,_106.0,_123.0,(*_318),_154.fld0,_146.0];
_319 = _13.1;
_99 = core::ptr::addr_of_mut!(_192);
_335.0 = [_88,_238];
_104 = Field::<Adt51>(Variant(_263, 0), 1).fld1;
(*_5) = core::ptr::addr_of_mut!(_276.1);
_45.fld0 = Field::<Adt51>(Variant(_263, 0), 1).fld2.1 as u16;
_107 = Move(_228);
Goto(bb143)
}
bb143 = {
_110 = _8.0;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3)).1 = (_122.0, _139, _331.0);
place!(Field::<Adt50>(Variant(_177, 0), 3)).fld1 = [_111,_142,_62.0,Field::<char>(Variant(_107, 1), 1),_64,_224.0];
_162 = _42.1;
_141 = Adt54::Variant1 { fld0: Field::<*mut f64>(Variant(_327, 1), 0),fld1: _219.2,fld2: _8,fld3: _104 };
_329 = _198.1 as isize;
_172.fld0 = (Field::<Adt50>(Variant(_143, 2), 0).fld0.0, Field::<[u16; 6]>(Variant(_263, 0), 2), _4);
_95 = _118;
_157 = _82;
_284.1 = _206.fld1;
_121 = Adt57::Variant0 { fld0: _168,fld1: (*_25),fld2: Field::<Adt51>(Variant(_263, 0), 1),fld3: _38.2,fld4: _1 };
place!(Field::<i32>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 3), 1)) = _161.fld2.2 as i32;
_57 = Adt54::Variant1 { fld0: (*_326),fld1: _219.2,fld2: _198,fld3: _104 };
_315 = _272;
_28 = Adt55::Variant1 { fld0: _161.fld6,fld1: Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_258, 1), 1),fld2: Field::<Adt51>(Variant(_263, 0), 1).fld2.0,fld3: _292.1.2,fld4: _155 };
place!(Field::<[i32; 1]>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 3), 2)) = [_127];
place!(Field::<isize>(Variant(_177, 0), 2)) = _284.3 as isize;
_13.0 = !_292.1.2;
_266 = Field::<i32>(Variant(_104, 3), 1) | _157;
Goto(bb144)
}
bb144 = {
place!(Field::<f64>(Variant(place!(Field::<Adt55>(Variant(_98, 1), 2)), 3), 0)) = _185.0;
_285 = Field::<[i32; 1]>(Variant(_104, 3), 2);
place!(Field::<(isize, f64, u16)>(Variant(_49, 1), 3)).2 = _45.fld0;
_70.2 = !Field::<Adt51>(Variant(_121, 0), 2).fld0.4;
_252 = _26;
place!(Field::<Adt51>(Variant(_121, 0), 2)).fld2 = (_16.0, (*_95), _36);
SetDiscriminant(Field::<Adt55>(Variant(_98, 1), 2), 3);
(*_207) = _292.1.0;
(*_168) = core::ptr::addr_of!(_292.1.1);
place!(Field::<([u64; 2], u128)>(Variant(_161.fld1, 0), 1)).1 = _145;
_349 = _111;
_347.0 = _72 as isize;
_284.2 = !_210.2;
_200.2 = _92;
_213 = !_219.3;
_41 = _166.0 as u32;
place!(Field::<*const i16>(Variant(_257, 0), 1)) = Field::<*const i16>(Variant(_47, 0), 1);
_102.0 = _122.0;
place!(Field::<*const u64>(Variant(_49, 1), 2)) = _161.fld4;
place!(Field::<*mut f64>(Variant(_98, 1), 1)) = (*_33);
place!(Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3)).0 = _100 as i128;
Goto(bb145)
}
bb145 = {
_360 = _34 as i128;
SetDiscriminant(Field::<Adt51>(Variant(_121, 0), 2).fld1, 3);
_210.4 = _13.3 + Field::<Adt51>(Variant(_121, 0), 2).fld2.1;
Goto(bb146)
}
bb146 = {
_225 = -(*_37);
_189.0 = !_272.1.0;
(*_132) = Field::<*const i128>(Variant(_104, 3), 0);
_76 = core::ptr::addr_of!((*_326));
place!(Field::<[i32; 1]>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 3), 2)) = Field::<[i32; 1]>(Variant(Field::<Adt48>(Variant(_184, 0), 6), 3), 2);
_309 = -_347.0;
place!(Field::<Adt50>(Variant(_177, 0), 3)) = Adt50 { fld0: _206.fld0,fld1: _119.fld1 };
_71.1 = _276.1;
place!(Field::<(isize, f64, u16)>(Variant(_49, 1), 3)).2 = _109.2 | _36;
_310 = [(*_318),(*_37),Field::<Adt56>(Variant(_98, 1), 7).fld0,_225,Field::<Adt56>(Variant(_98, 1), 7).fld0,_123.0];
_75 = [_16.2,_164.fld0,_45.fld0,_276.2,_23,_71.2];
Goto(bb147)
}
bb147 = {
place!(Field::<isize>(Variant(_29, 1), 2)) = _97.1;
_102 = (Field::<(i128, i128, i8)>(Variant(_177, 0), 1).0, Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3).1);
_236.2 = (*_26);
place!(Field::<i32>(Variant(_104, 3), 1)) = Field::<i32>(Variant(_177, 0), 5);
_308 = !Field::<Adt51>(Variant(_121, 0), 2).fld0.1;
_206.fld0.2 = -Field::<Adt50>(Variant(_49, 1), 0).fld0.2;
_97 = (Field::<(i128, i128, i8)>(Variant(_180, 1), 1).2, _71.0, _175, Field::<Adt51>(Variant(_263, 0), 1).fld2.1);
_89.1 = !Field::<isize>(Variant(_177, 0), 2);
(*_99) = _185.4;
_47 = Adt60::Variant0 { fld0: _104,fld1: _179.0 };
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_258, 1), 1)).3 = Field::<Adt51>(Variant(_121, 0), 2).fld0.3;
_94 = core::ptr::addr_of_mut!((*_95));
_315 = (_102.1.0, _272.1);
_221.fld0 = (Field::<f32>(Variant(_98, 1), 6), Field::<Adt50>(Variant(_177, 0), 3).fld0.1, _172.fld0.2);
Goto(bb148)
}
bb148 = {
_89.1 = _302;
_191 = [_81];
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld2.0 = Field::<Adt51>(Variant(_121, 0), 2).fld2.0 | _109.0;
_367.0 = _74;
place!(Field::<(i16,)>(Variant(_67, 2), 0)).0 = _205.1.0 as i16;
(*_37) = Field::<(i16,)>(Variant(_67, 2), 0).0;
_316 = _157;
_48 = (_179.0, _100, _240, Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).3, _12);
place!(Field::<i32>(Variant(_184, 0), 5)) = Field::<i32>(Variant(Field::<Adt48>(Variant(_47, 0), 0), 3), 1) - Field::<i32>(Variant(Field::<Adt48>(Variant(_57, 1), 3), 3), 1);
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt54>(Variant(_98, 1), 0)), 1), 2)).0 = [Field::<u64>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 1),_284.2];
_374.fld2.0 = _42.2 as isize;
place!(Field::<u64>(Variant(_141, 1), 1)) = _77.2;
Call(place!(Field::<Adt50>(Variant(_184, 0), 3)).fld0.1 = fn19(Field::<Adt51>(Variant(_121, 0), 2).fld5, (*_171), Field::<u128>(Variant(_180, 1), 3), Field::<Adt51>(Variant(_121, 0), 2).fld0.1, _146, _243.0, _82, Field::<Adt51>(Variant(_263, 0), 1).fld0), bb149, UnwindUnreachable())
}
bb149 = {
_341.0 = _202 as i8;
place!(Field::<*const i128>(Variant(place!(Field::<Adt51>(Variant(_121, 0), 2)).fld1, 3), 0)) = core::ptr::addr_of!((*_207));
_60 = _71.2 as f32;
_259 = Adt52 { fld0: _26 };
SetDiscriminant(_107, 1);
_189.0 = _86.fld0 as i128;
_374.fld2.1 = _185.0;
_29 = Adt55::Variant1 { fld0: _161.fld6,fld1: _200,fld2: _115.1,fld3: _122.1.2,fld4: Field::<u128>(Variant(_28, 1), 4) };
(*_132) = core::ptr::addr_of!(_272.1.0);
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld2.1 = (*_118);
place!(Field::<Adt50>(Variant(_143, 2), 0)).fld0.2 = _2 as i64;
_184 = Adt53::Variant1 { fld0: _220,fld1: _74,fld2: Move(_259),fld3: _183,fld4: Field::<(i128, (i128, i128, i8))>(Variant(_98, 1), 3),fld5: Field::<i32>(Variant(_104, 3), 1),fld6: (*_336) };
Goto(bb150)
}
bb150 = {
_350 = core::ptr::addr_of!(_102);
SetDiscriminant(_47, 0);
_306 = !_275;
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld6 = Field::<Adt51>(Variant(_121, 0), 2).fld0.4 as usize;
_357 = _284.4;
_360 = !_122.1.1;
_185.4 = _210.4 + Field::<Adt51>(Variant(_121, 0), 2).fld2.1;
(*_323) = core::ptr::addr_of!((*_207));
_221.fld0.1 = [_23,_294,Field::<(isize, f64, u16)>(Variant(_49, 1), 3).2,_161.fld2.2,_23,_53];
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld2 = (Field::<isize>(Variant(_258, 1), 2), _185.4, _36);
_70.3 = _123.0 as f64;
_362 = _349;
place!(Field::<([u64; 2], u128)>(Variant(_57, 1), 2)).0 = [Field::<u64>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 1),Field::<u64>(Variant(_263, 0), 3)];
_230 = !_91.1;
_367 = _166;
_370.0 = _244;
(*_336) = -_343.1.1;
place!(Field::<Adt51>(Variant(_121, 0), 2)).fld1 = Adt48::Variant0 { fld0: _59,fld1: _56,fld2: _350,fld3: _331 };
Goto(bb151)
}
bb151 = {
_73 = Field::<Adt51>(Variant(_263, 0), 1).fld5 as u32;
_374.fld0.4 = _183.0 as u32;
place!(Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_29, 1), 1)).3 = core::ptr::addr_of!(place!(Field::<Adt50>(Variant(_177, 0), 3)).fld0.2);
place!(Field::<*const i16>(Variant(_257, 0), 1)) = core::ptr::addr_of!((*_318));
Goto(bb152)
}
bb152 = {
(*_37) = -Field::<(i16,)>(Variant(_263, 0), 4).0;
place!(Field::<i8>(Variant(_28, 1), 3)) = Field::<Adt50>(Variant(_49, 1), 0).fld0.2 as i8;
_200.1 = Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).1;
_63 = -Field::<Adt50>(Variant(_49, 1), 0).fld0.0;
_136 = _212 as f64;
Goto(bb153)
}
bb153 = {
_98 = Adt60::Variant1 { fld0: Move(_141),fld1: (*_5),fld2: Move(_29),fld3: _205,fld4: _148,fld5: _165,fld6: _119.fld0.0,fld7: Move(_154) };
place!(Field::<[isize; 8]>(Variant(_128, 0), 1)) = [Field::<(isize, f64, u16)>(Variant(_49, 1), 3).0,_91.1,_91.1,_194,_13.1,_329,_70.1,Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(_121, 0), 2).fld1, 0), 3).1];
_91.3 = _231.4 * _210.4;
_76 = _326;
_250.0 = _69;
_189.0 = Field::<(i128, i128, i8)>(Variant(_180, 1), 1).0;
_37 = _318;
place!(Field::<i128>(Variant(_107, 1), 2)) = _122.1.0;
_210.4 = -_13.3;
_61 = Adt59::Variant1 { fld0: (*_5),fld1: Field::<i8>(Variant(_28, 1), 3) };
(*_350).0 = _64 as i128;
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(_121, 0), 2)).fld1, 0), 3)) = (Field::<i8>(Variant(_327, 1), 1), _317, _147, _182);
(*_76) = core::ptr::addr_of_mut!((*_94));
(*_326) = Field::<*mut f64>(Variant(Field::<Adt54>(Variant(_98, 1), 0), 1), 0);
SetDiscriminant(_98, 0);
_277 = [_142,_161.fld3.0,_2,_166.0,_135];
_42.0 = _10 - Field::<Adt50>(Variant(_143, 2), 0).fld0.0;
_164 = Adt61 { fld0: _53,fld1: Move(_121) };
_297 = _167;
_254 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(_164.fld1, 0), 2).fld1, 0), 3).1 - _16.0;
place!(Field::<[i32; 1]>(Variant(_104, 3), 2)) = [_157];
place!(Field::<[i32; 1]>(Variant(place!(Field::<Adt48>(Variant(_57, 1), 3)), 3), 2)) = [_82];
_272.1.1 = Field::<(i128, i128, i8)>(Variant(_180, 1), 1).1;
_185 = (_138, Field::<Adt50>(Variant(_49, 1), 0).fld1, _131, _306, _219.4);
_321 = !_179.1;
SetDiscriminant(_67, 1);
_394.1.0 = _189.0 | (*_336);
Goto(bb154)
}
bb154 = {
_386 = -Field::<Adt50>(Variant(_49, 1), 0).fld0.0;
_341.1 = _254;
_394 = (_343.1.1, Field::<(i128, i128, i8)>(Variant(_177, 0), 1));
_262 = -_247;
_211 = _331.1 ^ _89.1;
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld2 = (_89.1, (*_118), _164.fld0);
_287 = Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt51>(Variant(_164.fld1, 0), 2).fld1, 0), 3).2;
(*_33) = core::ptr::addr_of_mut!(_265);
_161 = Adt51 { fld0: _200,fld1: Field::<Adt51>(Variant(_164.fld1, 0), 2).fld1,fld2: Field::<(isize, f64, u16)>(Variant(_49, 1), 3),fld3: _237,fld4: Field::<*const u64>(Variant(_49, 1), 2),fld5: _394.0,fld6: Field::<Adt51>(Variant(_164.fld1, 0), 2).fld6 };
_380 = Adt63::Variant0 { fld0: _220,fld1: Move(_49),fld2: _161.fld0.3 };
_354 = [_111];
_374.fld3.0 = _142;
_377.2 = _185.2 as i64;
_374.fld0.4 = !_13.2;
_297.0 = -(*_283);
_161.fld0.4 = _89.0 as u32;
SetDiscriminant(_380, 0);
_127 = _157 - Field::<i32>(Variant(Field::<Adt51>(Variant(_263, 0), 1).fld1, 3), 1);
_374.fld4 = Field::<Adt51>(Variant(_164.fld1, 0), 2).fld4;
_271 = Adt54::Variant1 { fld0: (*_76),fld1: Field::<u64>(Variant(_263, 0), 3),fld2: _56,fld3: Field::<Adt51>(Variant(_164.fld1, 0), 2).fld1 };
place!(Field::<*const i16>(Variant(_98, 0), 1)) = Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_28, 1), 1).0;
_115.0 = !_13.0;
_318 = core::ptr::addr_of!(_183.0);
_345 = Adt54::Variant0 { fld0: _336,fld1: Move(Field::<Adt52>(Variant(_184, 1), 2)),fld2: Field::<(i8, isize, u32, f64)>(Variant(Field::<Adt48>(Variant(_271, 1), 3), 0), 3).1,fld3: Field::<Adt50>(Variant(_177, 0), 3),fld4: _56.1,fld5: _175 };
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld0.1 = !_100;
_402.1 = [_276.2,_161.fld2.2,_53,_294,Field::<Adt51>(Variant(_263, 0), 1).fld2.2,_23];
Goto(bb155)
}
bb155 = {
place!(Field::<i128>(Variant(_128, 0), 0)) = _272.0 * (*_350).1.1;
_27 = _18;
_401.0 = (*_207);
place!(Field::<*mut f64>(Variant(_180, 1), 6)) = core::ptr::addr_of_mut!(_192);
SetDiscriminant(_28, 2);
_347.0 = !_374.fld2.0;
_99 = core::ptr::addr_of_mut!(_355);
Goto(bb156)
}
bb156 = {
_16.1 = _97.3;
place!(Field::<([u64; 2], u128)>(Variant(place!(Field::<Adt48>(Variant(_271, 1), 3)), 0), 1)).1 = !_8.1;
(*_33) = core::ptr::addr_of_mut!(_276.1);
SetDiscriminant(_271, 1);
_219 = ((*_99), _185.1, _1, _6, Field::<Adt51>(Variant(_164.fld1, 0), 2).fld2.1);
_293.0 = [Field::<u64>(Variant(_263, 0), 3),_238];
_146.0 = _106.0 & (*_283);
_382 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_57, 1), 1)));
place!(Field::<*const i64>(Variant(_380, 0), 2)) = core::ptr::addr_of!(_186);
_232 = _161.fld3.0;
place!(Field::<[i32; 1]>(Variant(place!(Field::<Adt48>(Variant(_57, 1), 3)), 3), 2)) = Field::<[i32; 1]>(Variant(Field::<Adt51>(Variant(_263, 0), 1).fld1, 3), 2);
SetDiscriminant(_345, 1);
_361 = [_195,_54,_195,_14,_14,_170,_195,_54];
_219 = _284;
_271 = Move(_57);
_47 = Adt60::Variant2 { fld0: _179.3,fld1: _237.0,fld2: _87,fld3: Field::<Adt51>(Variant(_164.fld1, 0), 2).fld3,fld4: _76,fld5: _102,fld6: _280,fld7: _77 };
Goto(bb157)
}
bb157 = {
_13.0 = Field::<usize>(Variant(_258, 1), 0) as i8;
place!(Field::<([u64; 2], u128)>(Variant(_345, 1), 2)).1 = Field::<([u64; 2], u128)>(Variant(_161.fld1, 0), 1).1 << _11;
_385 = [_170,_54,_54,_195,_54,_170,_195,_14];
_229 = !_194;
_397.2 = _213 as u64;
_206.fld0 = (_248, _65, _186);
_350 = core::ptr::addr_of!(_102);
_228 = Adt64::Variant1 { fld0: _170,fld1: _224.0,fld2: _315.0 };
_279 = _385;
_161.fld1 = Adt48::Variant2 { fld0: _119.fld1,fld1: Field::<([u64; 2], u128)>(Variant(_271, 1), 2).0,fld2: _206.fld0.2,fld3: _246,fld4: _361,fld5: _316 };
_291 = Field::<([u64; 2], u128)>(Variant(_271, 1), 2).1 as isize;
_304 = Field::<([u64; 2], u128)>(Variant(_271, 1), 2).1 as f32;
_203 = _234;
place!(Field::<u64>(Variant(_345, 1), 1)) = !_219.2;
_33 = _326;
Goto(bb158)
}
bb158 = {
_199 = Adt60::Variant0 { fld0: Field::<Adt51>(Variant(_263, 0), 1).fld1,fld1: _318 };
_161.fld0.1 = !_321;
_265 = _236.3 * _115.3;
place!(Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4)).0 = _54 as i128;
(*_33) = _99;
_290 = _133;
_185.0 = _9 + _161.fld2.1;
_143 = Adt55::Variant0 { fld0: _172.fld0.2,fld1: Field::<Adt51>(Variant(_263, 0), 1),fld2: Field::<[u16; 6]>(Variant(_263, 0), 2),fld3: _131,fld4: Field::<(i16,)>(Variant(_263, 0), 4) };
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld0.4 = (*_283) as u32;
_194 = _317 - _276.0;
_115 = (_152, _211, _93, Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7).0);
_339.fld0 = core::ptr::addr_of_mut!((*_26));
_340 = _181 * _212;
_232 = _133;
_404.2 = [_1,_284.2];
_276 = (_317, (*_94), _71.2);
_73 = !(*_252);
_399 = !Field::<usize>(Variant(_258, 1), 0);
(*_283) = _16.0 as i16;
place!(Field::<(i128, i128, i8)>(Variant(_177, 0), 1)).1 = Field::<Adt51>(Variant(_143, 0), 1).fld5;
place!(Field::<[i32; 8]>(Variant(_47, 2), 2)) = [_165,Field::<i32>(Variant(Field::<Adt48>(Variant(_199, 0), 0), 3), 1),_157,_127,Field::<i32>(Variant(Field::<Adt48>(Variant(_199, 0), 0), 3), 1),_127,_157,Field::<i32>(Variant(Field::<Adt48>(Variant(_271, 1), 3), 3), 1)];
_150 = Adt52 { fld0: _31 };
_212 = _340;
_374.fld0 = (_283, _126, Field::<[u64; 2]>(Variant(_161.fld1, 2), 1), _200.3, _91.2);
place!(Field::<Adt49>(Variant(_177, 0), 4)) = Adt49::Variant2 { fld0: _190,fld1: Field::<Adt51>(Variant(_263, 0), 1).fld3.0,fld2: _206.fld0,fld3: _179.0,fld4: Field::<Adt51>(Variant(_164.fld1, 0), 2).fld2.2,fld5: Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7).2,fld6: _161.fld1,fld7: Field::<Adt51>(Variant(_263, 0), 1).fld0 };
_276.2 = (*_252) as u16;
place!(Field::<*const i128>(Variant(place!(Field::<Adt48>(Variant(_271, 1), 3)), 3), 0)) = core::ptr::addr_of!(place!(Field::<(i128, i128, i8)>(Variant(_180, 1), 1)).1);
Goto(bb159)
}
bb159 = {
place!(Field::<Adt50>(Variant(_28, 2), 0)).fld0.2 = Field::<i64>(Variant(_143, 0), 0);
place!(Field::<*const i128>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt49>(Variant(_177, 0), 4)), 2), 6)), 2), 3)) = core::ptr::addr_of!(place!(Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4)).1.1);
_86.fld0 = !_297.0;
_405.2 = _97.0 - _70.0;
_146.0 = _237.0 as i16;
_100 = !_213;
_267 = _261;
_39 = [_83];
_16 = Field::<Adt51>(Variant(_164.fld1, 0), 2).fld2;
_189.1 = !Field::<(i128, (i128, i128, i8))>(Variant(_47, 2), 5).1.1;
_374 = Adt51 { fld0: _179,fld1: Field::<Adt51>(Variant(_164.fld1, 0), 2).fld1,fld2: _16,fld3: _250,fld4: Field::<Adt51>(Variant(_263, 0), 1).fld4,fld5: _315.1.0,fld6: Field::<usize>(Variant(_258, 1), 0) };
_272.0 = _315.1.0 - Field::<i128>(Variant(_128, 0), 0);
(*_5) = core::ptr::addr_of_mut!(_9);
_404.0 = Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(_258, 1), 1).0;
Goto(bb160)
}
bb160 = {
_79 = _236.0 * _70.0;
_397.4 = _172.fld0.0 as f64;
_394.1.0 = (*_171);
_390 = core::ptr::addr_of!(_183.0);
_237 = (_224.0,);
_71.0 = -_115.1;
_341.0 = _133 as i8;
_165 = Field::<i32>(Variant(_177, 0), 5) * _316;
Goto(bb161)
}
bb161 = {
place!(Field::<Adt48>(Variant(_257, 0), 0)) = Field::<Adt51>(Variant(_143, 0), 1).fld1;
_4 = Field::<Adt50>(Variant(_177, 0), 3).fld0.2 ^ Field::<(f32, [u16; 6], i64)>(Variant(Field::<Adt49>(Variant(_177, 0), 4), 2), 2).2;
SetDiscriminant(Field::<Adt51>(Variant(_143, 0), 1).fld1, 3);
SetDiscriminant(Field::<Adt48>(Variant(_271, 1), 3), 0);
_374.fld6 = Field::<Adt51>(Variant(_164.fld1, 0), 2).fld6;
_304 = _101 as f32;
_359.2 = -Field::<(i128, i128, i8)>(Variant(_180, 1), 1).2;
_292.0 = (*_336);
place!(Field::<(i8, isize, u32, f64)>(Variant(place!(Field::<Adt51>(Variant(_164.fld1, 0), 2)).fld1, 0), 3)) = (_102.1.2, Field::<isize>(Variant(_258, 1), 2), (*_31), (*_118));
_359.0 = _205.1.2 as i128;
_407.0 = (*_390) * Field::<(i16,)>(Variant(_263, 0), 4).0;
place!(Field::<*const i128>(Variant(place!(Field::<Adt48>(Variant(_257, 0), 0)), 3), 0)) = Field::<*const i128>(Variant(Field::<Adt48>(Variant(Field::<Adt49>(Variant(_177, 0), 4), 2), 6), 2), 3);
_315.1.2 = Field::<(i128, i128, i8)>(Variant(_180, 1), 1).2;
_172.fld0.1 = [_164.fld0,Field::<u16>(Variant(Field::<Adt49>(Variant(_177, 0), 4), 2), 4),Field::<Adt51>(Variant(_143, 0), 1).fld2.2,Field::<Adt51>(Variant(_263, 0), 1).fld2.2,Field::<Adt51>(Variant(_164.fld1, 0), 2).fld2.2,_45.fld0];
place!(Field::<i64>(Variant(_164.fld1, 0), 3)) = _231.4 as i64;
_326 = _33;
_221 = Adt50 { fld0: _119.fld0,fld1: Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7).1 };
place!(Field::<*mut *const i128>(Variant(_164.fld1, 0), 0)) = _168;
place!(Field::<u64>(Variant(_164.fld1, 0), 4)) = _219.2;
_374.fld0.0 = core::ptr::addr_of!((*_318));
_200.1 = !_306;
_219 = (_182, Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7).1, _77.2, _100, _192);
_185.1 = Field::<(f64, [char; 6], u64, bool, f64)>(Variant(_47, 2), 7).1;
_398 = (*_252) * _103;
place!(Field::<(i8, isize, u32, f64)>(Variant(_374.fld1, 0), 3)).0 = _152 >> Field::<u8>(Variant(_228, 1), 0);
Call(_266 = core::intrinsics::transmute(_374.fld0.4), bb162, UnwindUnreachable())
}
bb162 = {
SetDiscriminant(Field::<Adt48>(Variant(Field::<Adt49>(Variant(_177, 0), 4), 2), 6), 2);
_320 = !(*_252);
place!(Field::<*const i128>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 3), 0)) = (*_112);
place!(Field::<u64>(Variant(_164.fld1, 0), 4)) = !_1;
_341 = ((*_350).1.2, Field::<(i8, isize, u32, f64)>(Variant(_374.fld1, 0), 3).1, Field::<(*const i16, bool, [u64; 2], *const i64, u32)>(Variant(Field::<Adt49>(Variant(_177, 0), 4), 2), 7).4, _182);
_292.1.1 = -(*_207);
(*_37) = _374.fld6 as i16;
_185.1 = [Field::<char>(Variant(_184, 1), 1),_130.0,Field::<char>(Variant(_184, 1), 1),_370.0,Field::<Adt51>(Variant(_164.fld1, 0), 2).fld3.0,_83];
_404 = (_390, _306, _48.2, Field::<Adt51>(Variant(_164.fld1, 0), 2).fld0.3, _97.2);
_89.0 = _405.2 - _122.1.2;
_189.0 = Field::<(i128, (i128, i128, i8))>(Variant(_184, 1), 4).1.1;
_147 = !_91.2;
place!(Field::<[u8; 8]>(Variant(_161.fld1, 2), 4)) = [_195,_170,Field::<u8>(Variant(_228, 1), 0),Field::<u8>(Variant(_228, 1), 0),_195,_54,_195,_14];
_219.2 = _23 as u64;
_119.fld0.2 = Field::<Adt50>(Variant(_28, 2), 0).fld0.2 ^ Field::<i64>(Variant(_263, 0), 0);
place!(Field::<*const i128>(Variant(place!(Field::<Adt48>(Variant(_257, 0), 0)), 3), 0)) = core::ptr::addr_of!((*_336));
place!(Field::<[u16; 6]>(Variant(_143, 0), 2)) = [_45.fld0,_374.fld2.2,_16.2,_164.fld0,_109.2,_53];
(*_25) = _200.4 - (*_31);
_261 = _200.0;
_408 = Field::<Adt51>(Variant(_143, 0), 1).fld2;
_255 = [_317,_85,Field::<Adt51>(Variant(_263, 0), 1).fld2.0,_329,_115.1,_71.0,_302,Field::<isize>(Variant(_258, 1), 2)];
place!(Field::<i32>(Variant(place!(Field::<Adt51>(Variant(_263, 0), 1)).fld1, 3), 1)) = _266;
_213 = _174;
Goto(bb163)
}
bb163 = {
_183.0 = _72 | _297.0;
_411 = core::ptr::addr_of!(_407.0);
SetDiscriminant(_164.fld1, 2);
_401 = _122.1;
_131 = _77.2;
place!(Field::<f32>(Variant(_134, 1), 2)) = _262;
_274 = Field::<Adt51>(Variant(_143, 0), 1).fld2.2;
_82 = !Field::<i32>(Variant(Field::<Adt48>(Variant(_257, 0), 0), 3), 1);
SetDiscriminant(_228, 2);
_376.0 = Field::<Adt50>(Variant(_177, 0), 3).fld0.0;
_334 = _408.0 as f64;
_74 = _166.0;
_179.0 = Field::<Adt51>(Variant(_143, 0), 1).fld0.0;
_343.1 = (Field::<(i128, (i128, i128, i8))>(Variant(_47, 2), 5).1.1, Field::<i128>(Variant(_107, 1), 2), Field::<i8>(Variant(_61, 1), 1));
SetDiscriminant(_257, 2);
place!(Field::<i64>(Variant(place!(Field::<Adt48>(Variant(place!(Field::<Adt49>(Variant(_177, 0), 4)), 2), 6)), 2), 2)) = _161.fld6 as i64;
SetDiscriminant(_47, 0);
(*_118) = _284.0 * _71.1;
Goto(bb164)
}
bb164 = {
_355 = -_71.1;
_148 = _114;
_86.fld0 = !_183.0;
place!(Field::<(i128, i128, i8)>(Variant(_67, 1), 0)).0 = _343.1.1 >> _12;
_417.fld1 = [_232,_64,_81,_111,_166.0,Field::<char>(Variant(Field::<Adt49>(Variant(_177, 0), 4), 2), 1)];
_231.1 = [_237.0,_83,_362,_135,_244,_349];
_386 = _212 + _247;
RET = Adt57::Variant0 { fld0: Field::<*mut *const i128>(Variant(_180, 1), 7),fld1: _147,fld2: Field::<Adt51>(Variant(_263, 0), 1),fld3: _42.2,fld4: _120 };
place!(Field::<([u64; 2], u128)>(Variant(_345, 1), 2)).1 = !_190;
place!(Field::<Adt51>(Variant(_263, 0), 1)).fld0.2 = [_1,_193];
_377 = (Field::<f32>(Variant(_134, 1), 2), _221.fld0.1, _38.2);
place!(Field::<i128>(Variant(_128, 0), 0)) = Field::<Adt50>(Variant(_177, 0), 3).fld0.0 as i128;
_3 = !Field::<u64>(Variant(_263, 0), 3);
Goto(bb165)
}
bb165 = {
Call(_426 = dump_var(3_usize, 80_usize, Move(_80), 79_usize, Move(_79), 274_usize, Move(_274), 203_usize, Move(_203)), bb166, UnwindUnreachable())
}
bb166 = {
Call(_426 = dump_var(3_usize, 222_usize, Move(_222), 78_usize, Move(_78), 82_usize, Move(_82), 69_usize, Move(_69)), bb167, UnwindUnreachable())
}
bb167 = {
Call(_426 = dump_var(3_usize, 173_usize, Move(_173), 56_usize, Move(_56), 223_usize, Move(_223), 349_usize, Move(_349)), bb168, UnwindUnreachable())
}
bb168 = {
Call(_426 = dump_var(3_usize, 75_usize, Move(_75), 58_usize, Move(_58), 170_usize, Move(_170), 266_usize, Move(_266)), bb169, UnwindUnreachable())
}
bb169 = {
Call(_426 = dump_var(3_usize, 270_usize, Move(_270), 100_usize, Move(_100), 230_usize, Move(_230), 277_usize, Move(_277)), bb170, UnwindUnreachable())
}
bb170 = {
Call(_426 = dump_var(3_usize, 41_usize, Move(_41), 290_usize, Move(_290), 229_usize, Move(_229), 385_usize, Move(_385)), bb171, UnwindUnreachable())
}
bb171 = {
Call(_426 = dump_var(3_usize, 116_usize, Move(_116), 72_usize, Move(_72), 285_usize, Move(_285), 178_usize, Move(_178)), bb172, UnwindUnreachable())
}
bb172 = {
Call(_426 = dump_var(3_usize, 39_usize, Move(_39), 306_usize, Move(_306), 187_usize, Move(_187), 102_usize, Move(_102)), bb173, UnwindUnreachable())
}
bb173 = {
Call(_426 = dump_var(3_usize, 133_usize, Move(_133), 36_usize, Move(_36), 354_usize, Move(_354), 226_usize, Move(_226)), bb174, UnwindUnreachable())
}
bb174 = {
Call(_426 = dump_var(3_usize, 7_usize, Move(_7), 96_usize, Move(_96), 88_usize, Move(_88), 162_usize, Move(_162)), bb175, UnwindUnreachable())
}
bb175 = {
Call(_426 = dump_var(3_usize, 227_usize, Move(_227), 123_usize, Move(_123), 2_usize, Move(_2), 101_usize, Move(_101)), bb176, UnwindUnreachable())
}
bb176 = {
Call(_426 = dump_var(3_usize, 166_usize, Move(_166), 93_usize, Move(_93), 125_usize, Move(_125), 68_usize, Move(_68)), bb177, UnwindUnreachable())
}
bb177 = {
Call(_426 = dump_var(3_usize, 17_usize, Move(_17), 175_usize, Move(_175), 131_usize, Move(_131), 74_usize, Move(_74)), bb178, UnwindUnreachable())
}
bb178 = {
Call(_426 = dump_var(3_usize, 239_usize, Move(_239), 328_usize, Move(_328), 282_usize, Move(_282), 160_usize, Move(_160)), bb179, UnwindUnreachable())
}
bb179 = {
Call(_426 = dump_var(3_usize, 135_usize, Move(_135), 280_usize, Move(_280), 302_usize, Move(_302), 83_usize, Move(_83)), bb180, UnwindUnreachable())
}
bb180 = {
Call(_426 = dump_var(3_usize, 4_usize, Move(_4), 311_usize, Move(_311), 292_usize, Move(_292), 144_usize, Move(_144)), bb181, UnwindUnreachable())
}
bb181 = {
Call(_426 = dump_var(3_usize, 218_usize, Move(_218), 114_usize, Move(_114), 255_usize, Move(_255), 360_usize, Move(_360)), bb182, UnwindUnreachable())
}
bb182 = {
Call(_426 = dump_var(3_usize, 152_usize, Move(_152), 190_usize, Move(_190), 146_usize, Move(_146), 8_usize, Move(_8)), bb183, UnwindUnreachable())
}
bb183 = {
Call(_426 = dump_var(3_usize, 291_usize, Move(_291), 287_usize, Move(_287), 232_usize, Move(_232), 27_usize, Move(_27)), bb184, UnwindUnreachable())
}
bb184 = {
Call(_426 = dump_var(3_usize, 23_usize, Move(_23), 32_usize, Move(_32), 240_usize, Move(_240), 205_usize, Move(_205)), bb185, UnwindUnreachable())
}
bb185 = {
Call(_426 = dump_var(3_usize, 159_usize, Move(_159), 254_usize, Move(_254), 105_usize, Move(_105), 106_usize, Move(_106)), bb186, UnwindUnreachable())
}
bb186 = {
Call(_426 = dump_var(3_usize, 54_usize, Move(_54), 309_usize, Move(_309), 174_usize, Move(_174), 11_usize, Move(_11)), bb187, UnwindUnreachable())
}
bb187 = {
Call(_426 = dump_var(3_usize, 51_usize, Move(_51), 321_usize, Move(_321), 20_usize, Move(_20), 124_usize, Move(_124)), bb188, UnwindUnreachable())
}
bb188 = {
Call(_426 = dump_var(3_usize, 237_usize, Move(_237), 122_usize, Move(_122), 361_usize, Move(_361), 320_usize, Move(_320)), bb189, UnwindUnreachable())
}
bb189 = {
Call(_426 = dump_var(3_usize, 65_usize, Move(_65), 59_usize, Move(_59), 427_usize, _427, 427_usize, _427), bb190, UnwindUnreachable())
}
bb190 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i8,mut _2: u64,mut _3: (i8, isize, u32, f64),mut _4: i8,mut _5: i8) -> f32 {
mir! {
type RET = f32;
let _6: Adt58;
let _7: isize;
let _8: ([u64; 2], u128);
let _9: (i16,);
let _10: [isize; 8];
let _11: [isize; 8];
let _12: f32;
let _13: isize;
let _14: Adt56;
let _15: ();
let _16: ();
{
_5 = _3.0 + _3.0;
_7 = !_3.1;
RET = _3.1 as f32;
_8.0 = [_2,_2];
match _1 {
0 => bb1,
340282366920938463463374607431768211385 => bb3,
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
_1 = 21862_i16 as i8;
_8.0 = [_2,_2];
_9 = ((-27314_i16),);
_3.1 = -_7;
_4 = -_5;
_9.0 = !31225_i16;
_8.1 = 29017_u16 as u128;
_9 = (29117_i16,);
_5 = _3.0 - _4;
Call(RET = fn5(_3.0, _4, _5, _3, _4, _5, _3.0, _5, _3, _4, _3, _3.2), bb4, UnwindUnreachable())
}
bb4 = {
_2 = 18402311736222433615_u64;
_1 = -_3.0;
_5 = _4 * _3.0;
_4 = _5;
_3.2 = (-3722197098987426426_i64) as u32;
_1 = !_4;
_3.1 = !_7;
_3.3 = _7 as f64;
_11 = [_3.1,_7,_3.1,_7,_7,_7,_3.1,_3.1];
_3.3 = 52259326095565473775334972534913573853_i128 as f64;
_3.0 = 10850_u16 as i8;
_7 = !_3.1;
_1 = _4 << _7;
_9.0 = 150_u8 as i16;
_10 = _11;
RET = 20773743719837402201009355266246330381_i128 as f32;
_3.1 = -_7;
_3.0 = !_5;
_2 = 7191788006230349720_i64 as u64;
RET = (-64495519865092315409404280762333407229_i128) as f32;
_14 = Adt56 { fld0: _9.0 };
RET = _5 as f32;
_11 = [_7,_3.1,_7,_7,_7,_7,_3.1,_3.1];
_1 = -_3.0;
_3.3 = _9.0 as f64;
_5 = _3.0;
Goto(bb5)
}
bb5 = {
Call(_15 = dump_var(4_usize, 8_usize, Move(_8), 4_usize, Move(_4), 1_usize, Move(_1), 11_usize, Move(_11)), bb6, UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: (i8, isize, u32, f64),mut _5: i8,mut _6: i8,mut _7: i8,mut _8: i8,mut _9: (i8, isize, u32, f64),mut _10: i8,mut _11: (i8, isize, u32, f64),mut _12: u32) -> f32 {
mir! {
type RET = f32;
let _13: usize;
let _14: u128;
let _15: (f64, [char; 6], u64, bool, f64);
let _16: (i16,);
let _17: Adt62;
let _18: isize;
let _19: Adt56;
let _20: [i32; 8];
let _21: i8;
let _22: [char; 5];
let _23: bool;
let _24: i32;
let _25: i64;
let _26: usize;
let _27: u16;
let _28: u16;
let _29: ();
let _30: ();
{
RET = _4.1 as f32;
_10 = -_8;
_8 = _10 | _10;
_9.3 = _9.0 as f64;
_12 = 144_u8 as u32;
_10 = _4.0 << _8;
_9.3 = (-490743701_i32) as f64;
_9 = (_2, _4.1, _4.2, _4.3);
_11.0 = !_8;
_15.0 = -_4.3;
_15.3 = true;
_3 = _10 & _8;
_12 = _4.2;
_4.2 = _11.2 ^ _9.2;
_15.1 = ['\u{103a77}','\u{2ead7}','\u{c5e78}','\u{9fd02}','\u{5ceb7}','\u{333d8}'];
_9 = (_3, _11.1, _4.2, _15.0);
_7 = !_11.0;
_15.1 = ['\u{97aba}','\u{b267e}','\u{d3cdb}','\u{c4ff8}','\u{e8906}','\u{da028}'];
_15.2 = 4620713311606818320_u64;
Call(_2 = fn6(_1, _8, _10, _9, _15.1, _6, _9.0, _11.1, _6, _9.0, _4.0, _8, _15.2), bb1, UnwindUnreachable())
}
bb1 = {
_8 = _11.2 as i8;
_15.4 = _4.3;
_4 = (_3, _9.1, _11.2, _9.3);
_3 = _15.3 as i8;
_6 = _11.0 | _11.0;
_11.0 = -_10;
_6 = _4.0;
_4.0 = _15.0 as i8;
_9 = _11;
_11 = (_6, _9.1, _12, _9.3);
_11 = _9;
_15.2 = (-7033_i16) as u64;
_15.3 = !true;
Call(_9.3 = fn7(_6, _11.0, _11.0, _11.2, _6, _6, _9.0, _2), bb2, UnwindUnreachable())
}
bb2 = {
_6 = _4.0 * _2;
_8 = _5;
RET = 235835931857581978068064016333465102180_u128 as f32;
_9 = _11;
_10 = _11.0 + _6;
_15.1 = ['\u{4bc1f}','\u{71dd3}','\u{202c3}','\u{1795e}','\u{10602}','\u{7ab4c}'];
_13 = 4_usize & 6606364443879928118_usize;
_10 = _6;
_4.3 = _9.3 * _15.0;
_3 = 142984681294207148259916245235796198072_u128 as i8;
_8 = -_2;
match _1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211385 => bb7,
_ => bb6
}
}
bb3 = {
_8 = _11.2 as i8;
_15.4 = _4.3;
_4 = (_3, _9.1, _11.2, _9.3);
_3 = _15.3 as i8;
_6 = _11.0 | _11.0;
_11.0 = -_10;
_6 = _4.0;
_4.0 = _15.0 as i8;
_9 = _11;
_11 = (_6, _9.1, _12, _9.3);
_11 = _9;
_15.2 = (-7033_i16) as u64;
_15.3 = !true;
Call(_9.3 = fn7(_6, _11.0, _11.0, _11.2, _6, _6, _9.0, _2), bb2, UnwindUnreachable())
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
_3 = _10;
_6 = _10 - _2;
_15.3 = !false;
_8 = _3 >> _3;
_6 = '\u{a70fa}' as i8;
_16 = ((-9809_i16),);
_4.3 = _15.0 + _11.3;
_19.fld0 = _15.3 as i16;
_11.1 = -_4.1;
_9.0 = _3 * _8;
_21 = -_2;
_23 = !_15.3;
_22 = ['\u{76500}','\u{8882e}','\u{66c7a}','\u{109a2f}','\u{9813}'];
_4.2 = _10 as u32;
_20 = [720575245_i32,613946961_i32,2045634305_i32,(-356247573_i32),780292627_i32,1896391428_i32,1835991644_i32,(-394394799_i32)];
_11.0 = _7 ^ _8;
_2 = _8;
_4.1 = '\u{160ea}' as isize;
_10 = (-1414973614_i32) as i8;
_16 = (_19.fld0,);
_5 = _8;
_19 = Adt56 { fld0: _16.0 };
match _1 {
0 => bb1,
340282366920938463463374607431768211385 => bb8,
_ => bb4
}
}
bb8 = {
_23 = _2 < _11.0;
match _1 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607431768211385 => bb16,
_ => bb15
}
}
bb9 = {
_3 = _10;
_6 = _10 - _2;
_15.3 = !false;
_8 = _3 >> _3;
_6 = '\u{a70fa}' as i8;
_16 = ((-9809_i16),);
_4.3 = _15.0 + _11.3;
_19.fld0 = _15.3 as i16;
_11.1 = -_4.1;
_9.0 = _3 * _8;
_21 = -_2;
_23 = !_15.3;
_22 = ['\u{76500}','\u{8882e}','\u{66c7a}','\u{109a2f}','\u{9813}'];
_4.2 = _10 as u32;
_20 = [720575245_i32,613946961_i32,2045634305_i32,(-356247573_i32),780292627_i32,1896391428_i32,1835991644_i32,(-394394799_i32)];
_11.0 = _7 ^ _8;
_2 = _8;
_4.1 = '\u{160ea}' as isize;
_10 = (-1414973614_i32) as i8;
_16 = (_19.fld0,);
_5 = _8;
_19 = Adt56 { fld0: _16.0 };
match _1 {
0 => bb1,
340282366920938463463374607431768211385 => bb8,
_ => bb4
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
_8 = _11.2 as i8;
_15.4 = _4.3;
_4 = (_3, _9.1, _11.2, _9.3);
_3 = _15.3 as i8;
_6 = _11.0 | _11.0;
_11.0 = -_10;
_6 = _4.0;
_4.0 = _15.0 as i8;
_9 = _11;
_11 = (_6, _9.1, _12, _9.3);
_11 = _9;
_15.2 = (-7033_i16) as u64;
_15.3 = !true;
Call(_9.3 = fn7(_6, _11.0, _11.0, _11.2, _6, _6, _9.0, _2), bb2, UnwindUnreachable())
}
bb14 = {
_6 = _4.0 * _2;
_8 = _5;
RET = 235835931857581978068064016333465102180_u128 as f32;
_9 = _11;
_10 = _11.0 + _6;
_15.1 = ['\u{4bc1f}','\u{71dd3}','\u{202c3}','\u{1795e}','\u{10602}','\u{7ab4c}'];
_13 = 4_usize & 6606364443879928118_usize;
_10 = _6;
_4.3 = _9.3 * _15.0;
_3 = 142984681294207148259916245235796198072_u128 as i8;
_8 = -_2;
match _1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211385 => bb7,
_ => bb6
}
}
bb15 = {
_8 = _11.2 as i8;
_15.4 = _4.3;
_4 = (_3, _9.1, _11.2, _9.3);
_3 = _15.3 as i8;
_6 = _11.0 | _11.0;
_11.0 = -_10;
_6 = _4.0;
_4.0 = _15.0 as i8;
_9 = _11;
_11 = (_6, _9.1, _12, _9.3);
_11 = _9;
_15.2 = (-7033_i16) as u64;
_15.3 = !true;
Call(_9.3 = fn7(_6, _11.0, _11.0, _11.2, _6, _6, _9.0, _2), bb2, UnwindUnreachable())
}
bb16 = {
_9 = (_11.0, _4.1, _4.2, _15.0);
_18 = 6467582006452791097_i64 as isize;
_15.3 = !_23;
_4.2 = _9.2 * _9.2;
_24 = 152028690_i32;
_3 = _9.0;
_28 = 55863_u16;
_15.1 = ['\u{ac2b5}','\u{ed59a}','\u{a777a}','\u{b2795}','\u{75ede}','\u{78d8}'];
_15.1 = ['\u{4b32f}','\u{b4e56}','\u{ebc65}','\u{a6a5e}','\u{6c06b}','\u{44039}'];
_12 = !_9.2;
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(5_usize, 2_usize, Move(_2), 21_usize, Move(_21), 6_usize, Move(_6), 20_usize, Move(_20)), bb18, UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(5_usize, 8_usize, Move(_8), 10_usize, Move(_10), 24_usize, Move(_24), 3_usize, Move(_3)), bb19, UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(5_usize, 7_usize, Move(_7), 30_usize, _30, 30_usize, _30, 30_usize, _30), bb20, UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: (i8, isize, u32, f64),mut _5: [char; 6],mut _6: i8,mut _7: i8,mut _8: isize,mut _9: i8,mut _10: i8,mut _11: i8,mut _12: i8,mut _13: u64) -> i8 {
mir! {
type RET = i8;
let _14: (isize, f64, u16);
let _15: ();
let _16: ();
{
RET = _4.0;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(6_usize, 9_usize, Move(_9), 3_usize, Move(_3), 7_usize, Move(_7), 1_usize, Move(_1)), bb2, UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(6_usize, 13_usize, Move(_13), 11_usize, Move(_11), 16_usize, _16, 16_usize, _16), bb3, UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: u32,mut _5: i8,mut _6: i8,mut _7: i8,mut _8: i8) -> f64 {
mir! {
type RET = f64;
let _9: isize;
let _10: [isize; 3];
let _11: char;
let _12: f32;
let _13: isize;
let _14: u16;
let _15: Adt57;
let _16: isize;
let _17: char;
let _18: u8;
let _19: Adt53;
let _20: i8;
let _21: [char; 1];
let _22: u64;
let _23: *mut *const i128;
let _24: i32;
let _25: i16;
let _26: char;
let _27: i32;
let _28: (i128, (i128, i128, i8));
let _29: u64;
let _30: f32;
let _31: char;
let _32: Adt50;
let _33: u8;
let _34: isize;
let _35: u8;
let _36: i8;
let _37: isize;
let _38: Adt57;
let _39: isize;
let _40: i8;
let _41: ();
let _42: ();
{
_7 = 9178362141250181468_u64 as i8;
_5 = _3 << _1;
_6 = false as i8;
RET = 59215554795934704682377280374271952346_i128 as f64;
_3 = _8 * _5;
_5 = _3 * _2;
_2 = _3 + _5;
_8 = false as i8;
_4 = 2781981428_u32;
_1 = 1759830323_i32 as i8;
_7 = _2;
RET = 28128_u16 as f64;
_8 = !_2;
Call(_6 = fn8(_3, _5, _2, _2, _8), bb1, UnwindUnreachable())
}
bb1 = {
_1 = _5;
RET = 304049466589969543136265392107661329126_u128 as f64;
_5 = _7;
_6 = _7 ^ _1;
RET = (-8761122841155076005_i64) as f64;
_6 = _5 ^ _5;
_11 = '\u{dca26}';
_9 = RET as isize;
_10 = [_9,_9,_9];
_9 = (-1278078833_i32) as isize;
_10 = [_9,_9,_9];
_6 = 2809424545578221794_u64 as i8;
_10 = [_9,_9,_9];
_8 = -_2;
RET = 33193_u16 as f64;
_11 = '\u{2ed5e}';
_12 = RET as f32;
_12 = RET as f32;
_9 = !(-9223372036854775808_isize);
_1 = !_8;
_13 = 12_u8 as isize;
Call(_13 = core::intrinsics::transmute(_9), bb2, UnwindUnreachable())
}
bb2 = {
_6 = _5 & _7;
RET = _3 as f64;
_6 = _1 << _7;
_13 = _9;
_10 = [_13,_13,_13];
RET = 14098_u16 as f64;
_11 = '\u{a74e}';
_7 = 33005043790454160301825377612091504122_u128 as i8;
_10 = [_9,_13,_9];
_6 = !_1;
_11 = '\u{25886}';
_13 = _9;
match _4 {
0 => bb3,
1 => bb4,
2781981428 => bb6,
_ => bb5
}
}
bb3 = {
_1 = _5;
RET = 304049466589969543136265392107661329126_u128 as f64;
_5 = _7;
_6 = _7 ^ _1;
RET = (-8761122841155076005_i64) as f64;
_6 = _5 ^ _5;
_11 = '\u{dca26}';
_9 = RET as isize;
_10 = [_9,_9,_9];
_9 = (-1278078833_i32) as isize;
_10 = [_9,_9,_9];
_6 = 2809424545578221794_u64 as i8;
_10 = [_9,_9,_9];
_8 = -_2;
RET = 33193_u16 as f64;
_11 = '\u{2ed5e}';
_12 = RET as f32;
_12 = RET as f32;
_9 = !(-9223372036854775808_isize);
_1 = !_8;
_13 = 12_u8 as isize;
Call(_13 = core::intrinsics::transmute(_9), bb2, UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_14 = 31120_u16 * 19510_u16;
_11 = '\u{44948}';
_16 = _13 - _9;
_4 = 3510847800_u32;
_5 = _8;
_3 = !_2;
RET = (-4290_i16) as f64;
_17 = _11;
_2 = (-22048_i16) as i8;
_13 = !_9;
_18 = !169_u8;
_13 = 2740_i16 as isize;
_4 = 2697960526_u32 + 759682992_u32;
_7 = -_3;
_4 = 2798537068_u32;
_9 = _16 << _7;
_10 = [_9,_9,_9];
Goto(bb7)
}
bb7 = {
_8 = _6;
_7 = 1_usize as i8;
_7 = -_5;
_10 = [_9,_9,_9];
_2 = _8 & _5;
_11 = _17;
_9 = _16;
_12 = 1117052656977698748_i64 as f32;
_17 = _11;
_7 = _3;
_17 = _11;
_3 = _1;
_12 = _18 as f32;
_21 = [_11];
_8 = !_5;
_24 = _16 as i32;
_9 = !_16;
_13 = _16 >> _7;
_13 = _18 as isize;
_13 = 22618_i16 as isize;
_6 = _5 + _1;
_4 = !2971443641_u32;
_20 = _2 * _5;
_16 = _13 | _13;
_17 = _11;
_12 = _18 as f32;
Goto(bb8)
}
bb8 = {
_7 = _6 & _3;
_11 = _17;
_28.1.0 = (-136924672498614831504601647496940692374_i128) * 89689102620848842705376356137682821471_i128;
_5 = _20 | _3;
_26 = _11;
_7 = _1;
_4 = !778430195_u32;
_25 = 9605_i16;
RET = _18 as f64;
_10 = [_16,_16,_13];
_21 = [_17];
_28.1 = ((-127092285926527358657618893142188908339_i128), (-49040861621015798312958422422591402719_i128), _1);
_28.0 = _28.1.0 & _28.1.0;
_26 = _11;
_3 = -_20;
RET = 301481313937451257323710154399078921149_u128 as f64;
_28.1.2 = _6;
_9 = _16;
_6 = _20;
match _28.1.0 {
213190080994411104805755714289579303117 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_32.fld1 = [_26,_11,_26,_26,_17,_17];
_29 = !6861824398967655680_u64;
_28.1.1 = _28.0;
_20 = _8 * _3;
_31 = _11;
_11 = _26;
_28.1 = (_28.0, _28.0, _8);
_10 = [_9,_16,_16];
_27 = _24;
_6 = -_8;
_32.fld0.0 = _12 + _12;
RET = _4 as f64;
_18 = _27 as u8;
_1 = _16 as i8;
_21 = [_31];
_20 = !_6;
_22 = _29 - _29;
_28.1.1 = 276638523353321067541127922196089460410_u128 as i128;
_4 = !3524649974_u32;
_9 = !_16;
match _25 {
0 => bb2,
1 => bb11,
2 => bb12,
9605 => bb14,
_ => bb13
}
}
bb11 = {
_14 = 31120_u16 * 19510_u16;
_11 = '\u{44948}';
_16 = _13 - _9;
_4 = 3510847800_u32;
_5 = _8;
_3 = !_2;
RET = (-4290_i16) as f64;
_17 = _11;
_2 = (-22048_i16) as i8;
_13 = !_9;
_18 = !169_u8;
_13 = 2740_i16 as isize;
_4 = 2697960526_u32 + 759682992_u32;
_7 = -_3;
_4 = 2798537068_u32;
_9 = _16 << _7;
_10 = [_9,_9,_9];
Goto(bb7)
}
bb12 = {
_1 = _5;
RET = 304049466589969543136265392107661329126_u128 as f64;
_5 = _7;
_6 = _7 ^ _1;
RET = (-8761122841155076005_i64) as f64;
_6 = _5 ^ _5;
_11 = '\u{dca26}';
_9 = RET as isize;
_10 = [_9,_9,_9];
_9 = (-1278078833_i32) as isize;
_10 = [_9,_9,_9];
_6 = 2809424545578221794_u64 as i8;
_10 = [_9,_9,_9];
_8 = -_2;
RET = 33193_u16 as f64;
_11 = '\u{2ed5e}';
_12 = RET as f32;
_12 = RET as f32;
_9 = !(-9223372036854775808_isize);
_1 = !_8;
_13 = 12_u8 as isize;
Call(_13 = core::intrinsics::transmute(_9), bb2, UnwindUnreachable())
}
bb13 = {
_8 = _6;
_7 = 1_usize as i8;
_7 = -_5;
_10 = [_9,_9,_9];
_2 = _8 & _5;
_11 = _17;
_9 = _16;
_12 = 1117052656977698748_i64 as f32;
_17 = _11;
_7 = _3;
_17 = _11;
_3 = _1;
_12 = _18 as f32;
_21 = [_11];
_8 = !_5;
_24 = _16 as i32;
_9 = !_16;
_13 = _16 >> _7;
_13 = _18 as isize;
_13 = 22618_i16 as isize;
_6 = _5 + _1;
_4 = !2971443641_u32;
_20 = _2 * _5;
_16 = _13 | _13;
_17 = _11;
_12 = _18 as f32;
Goto(bb8)
}
bb14 = {
_26 = _31;
_7 = !_6;
_25 = (-3542_i16);
_26 = _11;
_28.1.0 = -_28.0;
_32.fld1 = [_17,_17,_11,_26,_17,_11];
_11 = _26;
_11 = _31;
_28.1 = (_28.0, _28.0, _6);
_32.fld0.1 = [_14,_14,_14,_14,_14,_14];
RET = 403528746921543641_i64 as f64;
_37 = -_16;
_35 = !_18;
_18 = _35 - _35;
_14 = 16506818094052709096615213961499682028_u128 as u16;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(7_usize, 24_usize, Move(_24), 2_usize, Move(_2), 9_usize, Move(_9), 22_usize, Move(_22)), bb16, UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(7_usize, 5_usize, Move(_5), 6_usize, Move(_6), 20_usize, Move(_20), 10_usize, Move(_10)), bb17, UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(7_usize, 14_usize, Move(_14), 11_usize, Move(_11), 31_usize, Move(_31), 18_usize, Move(_18)), bb18, UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(7_usize, 16_usize, Move(_16), 35_usize, Move(_35), 42_usize, _42, 42_usize, _42), bb19, UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: i8,mut _5: i8) -> i8 {
mir! {
type RET = i8;
let _6: [isize; 3];
let _7: isize;
let _8: ();
let _9: ();
{
_4 = _2;
RET = _1 * _4;
_5 = (-3731_i16) as i8;
_6 = [50_isize,9223372036854775807_isize,103_isize];
_6 = [9223372036854775807_isize,(-45_isize),88_isize];
_4 = (-9223372036854775808_isize) as i8;
RET = _1 - _3;
_7 = -(-122_isize);
_6 = [_7,_7,_7];
_7 = -9223372036854775807_isize;
_1 = 1120299184284556566_i64 as i8;
_2 = RET;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(8_usize, 5_usize, Move(_5), 6_usize, Move(_6), 1_usize, Move(_1), 9_usize, _9), bb2, UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i8,mut _2: [char; 5],mut _3: isize,mut _4: [char; 5],mut _5: i8) -> f64 {
mir! {
type RET = f64;
let _6: Adt56;
let _7: bool;
let _8: Adt57;
let _9: i16;
let _10: *const (i128, (i128, i128, i8));
let _11: Adt52;
let _12: (isize, f64, u16);
let _13: *mut *const i128;
let _14: (i128, (i128, i128, i8));
let _15: f32;
let _16: (f64, [char; 6], u64, bool, f64);
let _17: Adt50;
let _18: [char; 6];
let _19: *const i128;
let _20: f64;
let _21: ([u64; 2], u128);
let _22: char;
let _23: [char; 6];
let _24: f64;
let _25: char;
let _26: Adt52;
let _27: *const i16;
let _28: [u8; 8];
let _29: ();
let _30: ();
{
_2 = ['\u{57371}','\u{c4ddb}','\u{1797f}','\u{994e8}','\u{c61af}'];
_1 = -_5;
_1 = (-4718381268983582075_i64) as i8;
_5 = _1 & _1;
_5 = 5573774526376874729_i64 as i8;
_1 = _5;
_5 = (-5600_i16) as i8;
_3 = !(-9223372036854775808_isize);
_4 = _2;
RET = _3 as f64;
_2 = ['\u{e3e61}','\u{76168}','\u{d8b6e}','\u{a3b4}','\u{c5750}'];
_4 = ['\u{b72e8}','\u{1be0d}','\u{f9cd7}','\u{1e700}','\u{bda52}'];
RET = 12833864765615679213_u64 as f64;
_3 = 286429705616619490684833476099006398968_u128 as isize;
_4 = _2;
_3 = false as isize;
Call(_6 = fn10(_2, RET), bb1, UnwindUnreachable())
}
bb1 = {
_3 = 6284331328789782726_i64 as isize;
_1 = _5 * _5;
_6.fld0 = 30524_i16 & (-7944_i16);
_2 = ['\u{efae6}','\u{e304e}','\u{f4660}','\u{4ef53}','\u{5dad1}'];
_9 = _6.fld0 & _6.fld0;
_3 = (-9223372036854775808_isize);
_6 = Adt56 { fld0: _9 };
_3 = (-1217818343_i32) as isize;
_3 = 396517749_i32 as isize;
_2 = _4;
_3 = -9223372036854775807_isize;
_5 = -_1;
_12.0 = !_3;
_12.2 = !44238_u16;
_1 = !_5;
Goto(bb2)
}
bb2 = {
_6 = Adt56 { fld0: _9 };
_7 = _9 == _6.fld0;
_12.1 = 8634162300041822356_usize as f64;
_2 = ['\u{6bac8}','\u{81fb2}','\u{a83e}','\u{720f}','\u{d3901}'];
_4 = _2;
_4 = _2;
_5 = _1 - _1;
_6 = Adt56 { fld0: _9 };
_6.fld0 = _9;
RET = _12.1;
_3 = _12.0 * _12.0;
RET = _12.1;
_9 = _6.fld0;
_12.2 = 58647_u16 * 19397_u16;
_4 = ['\u{17e67}','\u{4993e}','\u{43d16}','\u{cca30}','\u{89ef9}'];
_1 = _5;
_14.1.1 = -(-71494945588798837087437258547317416252_i128);
_4 = _2;
_10 = core::ptr::addr_of!(_14);
(*_10).0 = _3 as i128;
_12.0 = _3 << _6.fld0;
_12 = (_3, RET, 58834_u16);
Goto(bb3)
}
bb3 = {
(*_10).1.1 = (*_10).0;
(*_10).1.0 = (*_10).0;
(*_10).1.0 = (*_10).0;
(*_10).1.1 = (*_10).0 >> _6.fld0;
_17.fld0.2 = !7102034160124063759_i64;
_16.2 = 11719856819245430893_u64;
(*_10).1.1 = (*_10).1.0;
_16.2 = 11001792065910382178_usize as u64;
_14.0 = _16.2 as i128;
(*_10).1 = (_14.0, (*_10).0, _5);
Goto(bb4)
}
bb4 = {
_4 = ['\u{fe068}','\u{be65a}','\u{d7f72}','\u{fc64d}','\u{f893b}'];
_16.3 = _7 & _7;
_18 = ['\u{9017b}','\u{c53b0}','\u{43e29}','\u{e98d8}','\u{30895}','\u{106b08}'];
(*_10).1.0 = _14.0;
_6 = Adt56 { fld0: _9 };
_19 = core::ptr::addr_of!(_14.0);
Goto(bb5)
}
bb5 = {
_16.4 = _12.1 + RET;
_13 = core::ptr::addr_of_mut!(_19);
_6.fld0 = _9 ^ _9;
match _12.2 {
0 => bb1,
1 => bb2,
2 => bb3,
58834 => bb7,
_ => bb6
}
}
bb6 = {
_4 = ['\u{fe068}','\u{be65a}','\u{d7f72}','\u{fc64d}','\u{f893b}'];
_16.3 = _7 & _7;
_18 = ['\u{9017b}','\u{c53b0}','\u{43e29}','\u{e98d8}','\u{30895}','\u{106b08}'];
(*_10).1.0 = _14.0;
_6 = Adt56 { fld0: _9 };
_19 = core::ptr::addr_of!(_14.0);
Goto(bb5)
}
bb7 = {
_17.fld0.1 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
(*_19) = -_14.1.0;
_14.1.2 = 2545327604_u32 as i8;
(*_10).1.0 = (*_10).1.1;
_14.1.1 = _3 as i128;
_17.fld0.1 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
(*_10).1 = ((*_10).0, (*_19), _5);
_21.1 = 173917615176418955203837946043952050864_u128 + 104342072022556182345701378128397986890_u128;
(*_10).0 = RET as i128;
_7 = _16.3 | _16.3;
_17.fld0.0 = _17.fld0.2 as f32;
Goto(bb8)
}
bb8 = {
_12.0 = _3 | _3;
_12.0 = !_3;
match _12.2 {
0 => bb1,
1 => bb4,
2 => bb9,
3 => bb10,
58834 => bb12,
_ => bb11
}
}
bb9 = {
_6 = Adt56 { fld0: _9 };
_7 = _9 == _6.fld0;
_12.1 = 8634162300041822356_usize as f64;
_2 = ['\u{6bac8}','\u{81fb2}','\u{a83e}','\u{720f}','\u{d3901}'];
_4 = _2;
_4 = _2;
_5 = _1 - _1;
_6 = Adt56 { fld0: _9 };
_6.fld0 = _9;
RET = _12.1;
_3 = _12.0 * _12.0;
RET = _12.1;
_9 = _6.fld0;
_12.2 = 58647_u16 * 19397_u16;
_4 = ['\u{17e67}','\u{4993e}','\u{43d16}','\u{cca30}','\u{89ef9}'];
_1 = _5;
_14.1.1 = -(-71494945588798837087437258547317416252_i128);
_4 = _2;
_10 = core::ptr::addr_of!(_14);
(*_10).0 = _3 as i128;
_12.0 = _3 << _6.fld0;
_12 = (_3, RET, 58834_u16);
Goto(bb3)
}
bb10 = {
_4 = ['\u{fe068}','\u{be65a}','\u{d7f72}','\u{fc64d}','\u{f893b}'];
_16.3 = _7 & _7;
_18 = ['\u{9017b}','\u{c53b0}','\u{43e29}','\u{e98d8}','\u{30895}','\u{106b08}'];
(*_10).1.0 = _14.0;
_6 = Adt56 { fld0: _9 };
_19 = core::ptr::addr_of!(_14.0);
Goto(bb5)
}
bb11 = {
_4 = ['\u{fe068}','\u{be65a}','\u{d7f72}','\u{fc64d}','\u{f893b}'];
_16.3 = _7 & _7;
_18 = ['\u{9017b}','\u{c53b0}','\u{43e29}','\u{e98d8}','\u{30895}','\u{106b08}'];
(*_10).1.0 = _14.0;
_6 = Adt56 { fld0: _9 };
_19 = core::ptr::addr_of!(_14.0);
Goto(bb5)
}
bb12 = {
RET = _21.1 as f64;
_21.1 = 239671421336087645171049646691812676703_u128 - 172894290632180063120706833950045327080_u128;
_16.1 = _18;
_9 = _16.2 as i16;
_17.fld0.0 = _16.4 as f32;
_16 = (RET, _18, 5941206814778368134_u64, _7, RET);
_21.0 = [_16.2,_16.2];
_3 = _12.0;
_21.1 = !80131963171631387989718352285223635461_u128;
(*_10).1.1 = !(*_10).0;
RET = _16.2 as f64;
(*_10).1.0 = (*_19);
_12.2 = 26979_u16 >> _12.0;
_6 = Adt56 { fld0: _9 };
_21.0 = [_16.2,_16.2];
_19 = core::ptr::addr_of!((*_10).1.1);
_17.fld0.0 = 1334312727_u32 as f32;
_21.1 = 282807333742191849842491084333167639284_u128;
_16 = (RET, _18, 3039460381859134948_u64, _7, RET);
_16 = (RET, _18, 6894816427648850752_u64, _7, RET);
_17.fld0.2 = 2722371016004748310_i64 + (-3023892266507377018_i64);
(*_10).1.1 = -_14.1.0;
_22 = '\u{94956}';
_17.fld1 = [_22,_22,_22,_22,_22,_22];
(*_10).1.0 = _17.fld0.2 as i128;
_1 = -(*_10).1.2;
Goto(bb13)
}
bb13 = {
Call(_29 = dump_var(9_usize, 9_usize, Move(_9), 22_usize, Move(_22), 4_usize, Move(_4), 14_usize, Move(_14)), bb14, UnwindUnreachable())
}
bb14 = {
Call(_29 = dump_var(9_usize, 2_usize, Move(_2), 30_usize, _30, 30_usize, _30, 30_usize, _30), bb15, UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [char; 5],mut _2: f64) -> Adt56 {
mir! {
type RET = Adt56;
let _3: [u64; 2];
let _4: Adt61;
let _5: Adt55;
let _6: [u32; 5];
let _7: u32;
let _8: (f64, [char; 6], u64, bool, f64);
let _9: u16;
let _10: isize;
let _11: [u32; 5];
let _12: isize;
let _13: *const (i128, (i128, i128, i8));
let _14: Adt53;
let _15: (f64, [char; 6], u64, bool, f64);
let _16: ();
let _17: ();
{
_1 = ['\u{1c4af}','\u{c1ea7}','\u{8cbdb}','\u{84600}','\u{175f5}'];
RET = Adt56 { fld0: 20370_i16 };
_4.fld0 = '\u{e4a3b}' as u16;
_4.fld0 = 8754_u16;
RET.fld0 = 2207892167435163964_u64 as i16;
_1 = ['\u{fc5b3}','\u{1f8ad}','\u{9d307}','\u{fa555}','\u{c2fc1}'];
RET = Adt56 { fld0: 16061_i16 };
_4.fld0 = 10321_u16;
RET.fld0 = 16377_i16 | (-9558_i16);
RET = Adt56 { fld0: (-32356_i16) };
_2 = (-116_i8) as f64;
RET = Adt56 { fld0: 16289_i16 };
_2 = (-340189599_i32) as f64;
_1 = ['\u{91e9b}','\u{a753c}','\u{da21c}','\u{1056bb}','\u{23071}'];
RET = Adt56 { fld0: (-17561_i16) };
match _4.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
10321 => bb8,
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
_2 = RET.fld0 as f64;
_3 = [12760369806689122157_u64,1276888608703932954_u64];
RET.fld0 = 5826_i16;
_5 = Adt55::Variant3 { fld0: _2 };
_6 = [63158812_u32,3737142936_u32,3036639094_u32,3616455161_u32,1848997626_u32];
RET = Adt56 { fld0: 22097_i16 };
_3 = [5186445246881519812_u64,14740744506796653659_u64];
place!(Field::<f64>(Variant(_5, 3), 0)) = _2 + _2;
_1 = ['\u{d3014}','\u{5840a}','\u{95f34}','\u{10f635}','\u{ed009}'];
RET.fld0 = (-1691351581_i32) as i16;
RET = Adt56 { fld0: 24266_i16 };
_3 = [1949824969628368760_u64,14655004762252363388_u64];
_1 = ['\u{40781}','\u{365bf}','\u{6038d}','\u{8a027}','\u{d8010}'];
_4.fld0 = _2 as u16;
_4.fld0 = 30752_u16 >> RET.fld0;
_7 = _4.fld0 as u32;
RET = Adt56 { fld0: 22356_i16 };
_7 = 670268729_i32 as u32;
_8.4 = Field::<f64>(Variant(_5, 3), 0) * Field::<f64>(Variant(_5, 3), 0);
RET.fld0 = (-25379_i16);
RET = Adt56 { fld0: (-4866_i16) };
_8.0 = _8.4 * _8.4;
RET = Adt56 { fld0: (-10876_i16) };
_8.2 = 1003879545445243015_u64;
match _8.2 {
0 => bb5,
1 => bb2,
1003879545445243015 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_8.1 = ['\u{e44ec}','\u{5ede5}','\u{3c3a1}','\u{c0e89}','\u{b0f1a}','\u{758c}'];
_8.0 = (-72_i8) as f64;
_4.fld0 = 64286_u16 | 16738_u16;
_8.2 = 7480023845642703222_u64 << _4.fld0;
SetDiscriminant(_5, 2);
place!(Field::<Adt50>(Variant(_5, 2), 0)).fld0.2 = -(-7635875062681654798_i64);
_10 = (-9223372036854775808_isize);
place!(Field::<Adt50>(Variant(_5, 2), 0)).fld0.1 = [_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0,_4.fld0];
_10 = (-9223372036854775808_isize) - 18_isize;
RET = Adt56 { fld0: 6908_i16 };
match RET.fld0 {
0 => bb7,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
6908 => bb16,
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
_9 = _4.fld0;
place!(Field::<Adt50>(Variant(_5, 2), 0)).fld0.0 = 3_u8 as f32;
place!(Field::<[i16; 6]>(Variant(_5, 2), 2)) = [RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0];
_8.0 = _10 as f64;
_8.3 = _2 > _8.4;
RET = Adt56 { fld0: 19539_i16 };
_8.1 = ['\u{10245e}','\u{8a5ef}','\u{f35bd}','\u{1015f0}','\u{8cbb2}','\u{70169}'];
place!(Field::<Adt50>(Variant(_5, 2), 0)).fld0.2 = 2288921077497103342_i64;
_4.fld0 = 155_u8 as u16;
_9 = _4.fld0 | _4.fld0;
RET = Adt56 { fld0: 6977_i16 };
place!(Field::<[i16; 6]>(Variant(_5, 2), 2)) = [RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0];
place!(Field::<Adt50>(Variant(_5, 2), 0)).fld1 = ['\u{c73ab}','\u{ad59}','\u{d083a}','\u{6ef03}','\u{ff73e}','\u{69851}'];
_15 = _8;
_4.fld0 = !_9;
place!(Field::<[i16; 6]>(Variant(_5, 2), 2)) = [RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0];
_12 = -_10;
_4.fld0 = Field::<Adt50>(Variant(_5, 2), 0).fld0.0 as u16;
_12 = _10 | _10;
_8.1 = ['\u{d836b}','\u{11a40}','\u{10eb96}','\u{5b221}','\u{d3d33}','\u{cf704}'];
_4.fld0 = 42290686480236184780332904724156895929_u128 as u16;
_8.1 = ['\u{63ff8}','\u{10ad34}','\u{342be}','\u{93ebe}','\u{1edf1}','\u{c7b9e}'];
_15.4 = -_15.0;
Goto(bb17)
}
bb17 = {
Call(_16 = dump_var(10_usize, 12_usize, Move(_12), 7_usize, Move(_7), 1_usize, Move(_1), 17_usize, _17), bb18, UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *mut u32,mut _2: f64,mut _3: (i8, isize, u32, f64),mut _4: (isize, f64, u16),mut _5: f64,mut _6: (isize, f64, u16),mut _7: (isize, f64, u16)) -> f32 {
mir! {
type RET = f32;
let _8: isize;
let _9: i16;
let _10: [i32; 1];
let _11: *const *mut f64;
let _12: isize;
let _13: [char; 5];
let _14: f32;
let _15: i16;
let _16: f64;
let _17: [isize; 8];
let _18: (i128, i128, i8);
let _19: Adt64;
let _20: f64;
let _21: isize;
let _22: i16;
let _23: isize;
let _24: isize;
let _25: bool;
let _26: (i16,);
let _27: [u16; 6];
let _28: isize;
let _29: char;
let _30: f64;
let _31: (i128, i128, i8);
let _32: Adt57;
let _33: i64;
let _34: [char; 1];
let _35: Adt61;
let _36: [i16; 6];
let _37: (char,);
let _38: i128;
let _39: (i8, isize, u32, f64);
let _40: (i128, (i128, i128, i8));
let _41: ();
let _42: ();
{
_4.2 = _6.2;
_4.0 = (*_1) as isize;
_3.2 = (*_1) | (*_1);
_7.2 = _6.2;
_7.1 = _6.1 + _6.1;
_4.2 = !_7.2;
_4.2 = _6.2;
_4.2 = !_6.2;
_4 = _7;
_8 = _6.2 as isize;
_3 = ((-33_i8), _6.0, (*_1), _7.1);
_6.1 = _4.1 + _4.1;
(*_1) = _3.2;
_4.1 = -_2;
_5 = _7.1 * _4.1;
_10 = [(-941956007_i32)];
_9 = (-3522_i16);
_4.1 = _7.1;
_8 = (*_1) as isize;
RET = 322019017239666087148408646912578661587_u128 as f32;
_3.1 = _4.0 - _7.0;
_4.0 = -_3.1;
_6.2 = _4.2;
_3.2 = !(*_1);
match _3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211423 => bb6,
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
(*_1) = _3.2;
_7.0 = _9 as isize;
Call(_3.2 = core::intrinsics::transmute((*_1)), bb7, UnwindUnreachable())
}
bb7 = {
_7.1 = _2 * _5;
_8 = _7.0 ^ _4.0;
_4.2 = _6.2;
_3 = (57_i8, _8, (*_1), _7.1);
_6.0 = _3.1 + _3.1;
_8 = 0_usize as isize;
_4 = _6;
_9 = !24741_i16;
_7.2 = _6.2 + _4.2;
_12 = '\u{90c8a}' as isize;
_13 = ['\u{50949}','\u{f031f}','\u{10761f}','\u{f10ec}','\u{b0d3a}'];
_4.2 = !_6.2;
_9 = false as i16;
_8 = _3.1 - _4.0;
_5 = _4.1 - _6.1;
_8 = _12;
_6.1 = _5;
_6.2 = _7.2;
_10 = [1484870113_i32];
_13 = ['\u{d2702}','\u{14f63}','\u{52d14}','\u{a68e}','\u{bc76c}'];
_4.0 = 281102043224474218818249098562456868302_u128 as isize;
_4.0 = _3.1;
_4.2 = !_7.2;
match _3.0 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
57 => bb10,
_ => bb9
}
}
bb8 = {
(*_1) = _3.2;
_7.0 = _9 as isize;
Call(_3.2 = core::intrinsics::transmute((*_1)), bb7, UnwindUnreachable())
}
bb9 = {
Return()
}
bb10 = {
_4 = (_6.0, _7.1, _6.2);
_7.0 = _3.0 as isize;
_3.2 = (*_1) & (*_1);
_15 = _7.1 as i16;
_7.0 = _4.0 | _4.0;
_14 = _4.0 as f32;
_2 = _6.2 as f64;
Goto(bb11)
}
bb11 = {
_17 = [_6.0,_7.0,_6.0,_3.1,_12,_6.0,_6.0,_7.0];
_3.3 = -_5;
_18.2 = _3.0;
(*_1) = _3.2;
_7 = (_6.0, _4.1, _4.2);
_18.2 = _3.0;
_3.1 = _4.0;
_16 = _4.1 - _6.1;
RET = _14;
_4.2 = _7.2 | _7.2;
_16 = _4.1;
_3.1 = _7.0;
_6.2 = 6586536311012821572_i64 as u16;
_3.2 = !(*_1);
_20 = _16 + _6.1;
_17 = [_7.0,_7.0,_6.0,_3.1,_7.0,_6.0,_6.0,_7.0];
_18 = (97796888562386643730754173193805053301_i128, (-113028675933672799414264440414584716670_i128), _3.0);
_3.2 = _18.2 as u32;
_17 = [_3.1,_3.1,_7.0,_4.0,_3.1,_3.1,_4.0,_3.1];
_6.2 = _4.2 ^ _7.2;
_4 = (_3.1, _7.1, _7.2);
_6.2 = _7.2 ^ _4.2;
_20 = 56_u8 as f64;
_3.2 = (*_1) + (*_1);
Call(_4 = fn12(_3.0, _15, _5, _3, _3, _18.2), bb12, UnwindUnreachable())
}
bb12 = {
_10 = [(-2056602165_i32)];
_23 = _6.0 | _3.1;
_3.2 = '\u{af8ac}' as u32;
_22 = 7281213964806535934_i64 as i16;
(*_1) = !_3.2;
_26.0 = (-931763929_i32) as i16;
_24 = !_4.0;
_7.0 = -_3.1;
_16 = _18.1 as f64;
_23 = -_4.0;
_18.0 = -_18.1;
_18 = (2363079846919248993707645893736240964_i128, (-2315854104240782724888689835553733555_i128), _3.0);
_3.1 = _23 >> _23;
_18.2 = !_3.0;
_7.2 = _4.2 * _4.2;
_28 = _6.0 * _3.1;
_15 = RET as i16;
match _3.0 {
0 => bb10,
57 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_31.1 = _18.0 | _18.0;
_20 = _4.2 as f64;
_4.0 = _28 ^ _28;
_23 = _4.0;
_26.0 = _15;
_31.1 = -_18.0;
_31.2 = _18.2 << _7.2;
_24 = 1989631779_i32 as isize;
_21 = !_28;
_27 = [_4.2,_4.2,_7.2,_7.2,_4.2,_4.2];
_2 = _7.1;
_17 = [_28,_28,_21,_23,_28,_21,_28,_21];
RET = _14 - _14;
_16 = -_5;
_26 = (_15,);
_27 = [_7.2,_4.2,_7.2,_4.2,_7.2,_7.2];
_4.2 = _7.2;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(11_usize, 23_usize, Move(_23), 28_usize, Move(_28), 10_usize, Move(_10), 9_usize, Move(_9)), bb16, UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(11_usize, 15_usize, Move(_15), 22_usize, Move(_22), 8_usize, Move(_8), 42_usize, _42), bb17, UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: i8,mut _2: i16,mut _3: f64,mut _4: (i8, isize, u32, f64),mut _5: (i8, isize, u32, f64),mut _6: i8) -> (isize, f64, u16) {
mir! {
type RET = (isize, f64, u16);
let _7: Adt57;
let _8: [i16; 6];
let _9: ();
let _10: ();
{
_4.2 = _5.2;
_4.0 = _5.0 + _5.0;
_5.2 = !_4.2;
_4 = (_5.0, _5.1, _5.2, _5.3);
RET.2 = 10725_u16 << _4.0;
RET.1 = _5.3 * _4.3;
RET.1 = 8267471900806435518_u64 as f64;
RET.0 = _5.1 >> _6;
RET = (_4.1, _4.3, 4682_u16);
_8 = [_2,_2,_2,_2,_2,_2];
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(12_usize, 1_usize, Move(_1), 2_usize, Move(_2), 10_usize, _10, 10_usize, _10), bb2, UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: u16,mut _2: f32,mut _3: isize,mut _4: f32,mut _5: f64,mut _6: isize,mut _7: f64,mut _8: isize,mut _9: (f32, [u16; 6], i64),mut _10: [u16; 6],mut _11: f32,mut _12: isize,mut _13: f32,mut _14: f32,mut _15: (isize, f64, u16)) -> u32 {
mir! {
type RET = u32;
let _16: (i128, i128, i8);
let _17: Adt49;
let _18: [u8; 8];
let _19: (i8, isize, u32, f64);
let _20: [u8; 8];
let _21: (i128, i128, i8);
let _22: Adt51;
let _23: u8;
let _24: u8;
let _25: ([u64; 2], u128);
let _26: bool;
let _27: Adt59;
let _28: u32;
let _29: Adt50;
let _30: (char,);
let _31: (char,);
let _32: Adt64;
let _33: *const i64;
let _34: ();
let _35: ();
{
_3 = _8 + _15.0;
_6 = -_8;
RET = 2365359316_u32;
_6 = _3 * _12;
_16.1 = (-150638451020314089658876344149370846383_i128) * 74973330562008722692737512819301541504_i128;
_16.2 = 8_i8 + (-114_i8);
_15 = (_6, _5, _1);
_15.1 = _5;
_16 = ((-6029478861152387597887929154549261274_i128), 138871941735642280525512753518578597490_i128, (-73_i8));
Goto(bb1)
}
bb1 = {
_10 = [_15.2,_1,_15.2,_15.2,_1,_1];
_16.1 = _16.0;
_7 = _15.1 * _15.1;
_3 = _9.2 as isize;
_19.0 = _5 as i8;
_9.2 = _19.0 as i64;
_9.1 = [_15.2,_15.2,_1,_1,_15.2,_15.2];
_7 = _5 - _5;
_15 = (_8, _7, _1);
_19 = (_16.2, _6, RET, _5);
_19.1 = _6;
_16.1 = _16.0 & _16.0;
_9 = (_13, _10, (-275443096160520160_i64));
_14 = _9.0;
_6 = _3;
_10 = _9.1;
_19 = (_16.2, _15.0, RET, _15.1);
_6 = _15.0 - _19.1;
_4 = _11 - _2;
_15.0 = -_19.1;
Goto(bb2)
}
bb2 = {
_18 = [195_u8,50_u8,52_u8,144_u8,166_u8,140_u8,150_u8,244_u8];
_12 = -_15.0;
_14 = _4 * _4;
_19.2 = !RET;
_15.1 = _19.3 - _7;
_15.0 = !_8;
_19.3 = _5;
_10 = _9.1;
_9.1 = [_1,_15.2,_1,_1,_1,_15.2];
_9.1 = [_1,_15.2,_1,_15.2,_1,_15.2];
_20 = [87_u8,222_u8,69_u8,15_u8,146_u8,37_u8,178_u8,4_u8];
_3 = 111484883441866985176369193332706876318_u128 as isize;
_9 = (_4, _10, (-5984398932196126373_i64));
match _9.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463457390208499572085083 => bb10,
_ => bb9
}
}
bb3 = {
_10 = [_15.2,_1,_15.2,_15.2,_1,_1];
_16.1 = _16.0;
_7 = _15.1 * _15.1;
_3 = _9.2 as isize;
_19.0 = _5 as i8;
_9.2 = _19.0 as i64;
_9.1 = [_15.2,_15.2,_1,_1,_15.2,_15.2];
_7 = _5 - _5;
_15 = (_8, _7, _1);
_19 = (_16.2, _6, RET, _5);
_19.1 = _6;
_16.1 = _16.0 & _16.0;
_9 = (_13, _10, (-275443096160520160_i64));
_14 = _9.0;
_6 = _3;
_10 = _9.1;
_19 = (_16.2, _15.0, RET, _15.1);
_6 = _15.0 - _19.1;
_4 = _11 - _2;
_15.0 = -_19.1;
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
_22.fld2.2 = _15.2;
_22.fld3.0 = '\u{a5131}';
_16 = (129811622393373372511736159163704487520_i128, (-85161097601851143598685661039212727173_i128), _19.0);
_21.2 = _16.2;
_16.2 = 5214916684827229557_u64 as i8;
_19.3 = _7 - _7;
_22.fld2 = (_19.1, _19.3, _15.2);
_22.fld6 = 2_usize - 14685093586418535176_usize;
_9 = (_13, _10, (-8470540274028309647_i64));
_15.0 = _6;
_9.2 = -(-67915491685564839_i64);
_22.fld0.2 = [3302943515114197732_u64,926661336425179080_u64];
_19.0 = -_21.2;
_22.fld3.0 = '\u{682b2}';
_22.fld0.4 = true as u32;
_21.0 = -_16.0;
_22.fld5 = _16.1;
_1 = true as u16;
_15.0 = (-1084544939_i32) as isize;
_22.fld3 = ('\u{6616d}',);
_20 = [239_u8,128_u8,109_u8,116_u8,233_u8,84_u8,67_u8,85_u8];
_9.0 = -_4;
_15.0 = -_6;
_22.fld0.1 = !false;
Goto(bb11)
}
bb11 = {
_19.2 = _22.fld0.4 - RET;
_16.2 = _19.0 + _21.2;
_2 = _4;
_19.0 = _15.1 as i8;
_22.fld0.3 = core::ptr::addr_of!(_9.2);
_8 = -_12;
_22.fld2 = (_6, _7, _15.2);
_22.fld5 = !_16.1;
match RET {
0 => bb6,
1 => bb9,
2365359316 => bb12,
_ => bb7
}
}
bb12 = {
RET = !_19.2;
_2 = _14;
RET = _22.fld0.4;
_22.fld2.1 = _15.1;
Call(_25.0 = core::intrinsics::transmute(_21.0), bb13, UnwindUnreachable())
}
bb13 = {
_12 = _15.0 >> _16.2;
_22.fld0.2 = _25.0;
_15.0 = _14 as isize;
_19.1 = _6 + _15.0;
_16.1 = _19.0 as i128;
_26 = _22.fld0.1;
_11 = _4;
_29.fld0.1 = [_15.2,_15.2,_22.fld2.2,_15.2,_15.2,_22.fld2.2];
_16.2 = _19.0;
_12 = 138_u8 as isize;
_28 = _9.2 as u32;
_18 = [226_u8,211_u8,169_u8,212_u8,195_u8,109_u8,144_u8,199_u8];
_22.fld2 = (_8, _15.1, _15.2);
_22.fld6 = 4_usize & 0_usize;
_21.0 = _16.1;
Goto(bb14)
}
bb14 = {
_29.fld0.0 = -_4;
_22.fld6 = !1369992089065653338_usize;
_24 = 8_u8;
_19.3 = -_15.1;
_19.3 = _22.fld6 as f64;
_32 = Adt64::Variant1 { fld0: _24,fld1: _22.fld3.0,fld2: _16.1 };
_8 = !_19.1;
_4 = 29858_i16 as f32;
_22.fld2.0 = _8;
_25 = (_22.fld0.2, 263944395879113476695233118822830205137_u128);
_16.0 = _16.1;
_16.2 = 2054911628374937169_u64 as i8;
SetDiscriminant(_32, 2);
_21.1 = _16.0;
_22.fld3 = ('\u{659ae}',);
_32 = Adt64::Variant1 { fld0: _24,fld1: _22.fld3.0,fld2: _21.1 };
_30 = _22.fld3;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(13_usize, 16_usize, Move(_16), 21_usize, Move(_21), 18_usize, Move(_18), 20_usize, Move(_20)), bb16, UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(13_usize, 12_usize, Move(_12), 3_usize, Move(_3), 28_usize, Move(_28), 35_usize, _35), bb17, UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: u16,mut _2: (isize, f64, u16),mut _3: f64,mut _4: ([u64; 2], u128)) -> (f32, [u16; 6], i64) {
mir! {
type RET = (f32, [u16; 6], i64);
let _5: bool;
let _6: [u32; 5];
let _7: (char,);
let _8: i8;
let _9: [u16; 6];
let _10: isize;
let _11: Adt54;
let _12: isize;
let _13: i32;
let _14: f32;
let _15: isize;
let _16: u32;
let _17: isize;
let _18: [i32; 8];
let _19: [isize; 8];
let _20: isize;
let _21: i128;
let _22: Adt50;
let _23: isize;
let _24: (*const i16, bool, [u64; 2], *const i64, u32);
let _25: isize;
let _26: *const *mut f64;
let _27: char;
let _28: [char; 6];
let _29: f64;
let _30: [char; 6];
let _31: ();
let _32: ();
{
RET.2 = -(-9015958998162295010_i64);
RET.2 = (-2676118423732227013_i64);
RET.0 = _4.1 as f32;
_5 = false;
RET.2 = _4.1 as i64;
RET.2 = !(-4637666441966309014_i64);
_3 = _2.1;
RET.1 = [_2.2,_1,_1,_1,_1,_1];
RET.1 = [_2.2,_1,_1,_2.2,_2.2,_1];
RET.0 = 14916177317669054154_u64 as f32;
_4.0 = [16938430390214795628_u64,2957336897063910519_u64];
_2.1 = _2.0 as f64;
_7.0 = '\u{f4d45}';
Goto(bb1)
}
bb1 = {
_3 = -_2.1;
_8 = (-115_i8) | 75_i8;
_4.0 = [7794340090202386411_u64,17880556551701424496_u64];
Goto(bb2)
}
bb2 = {
_5 = !false;
_9 = [_1,_1,_1,_1,_1,_1];
_4.0 = [12244417339182308264_u64,1454733020943583908_u64];
_4.0 = [8320159221138492219_u64,12259004510989557021_u64];
RET.0 = (-87047949_i32) as f32;
_7 = ('\u{3220}',);
_10 = _5 as isize;
_6 = [2934451690_u32,481159117_u32,40289864_u32,1446296459_u32,941707627_u32];
_8 = (-56_i8);
_7.0 = '\u{5e3c1}';
_9 = RET.1;
_8 = _7.0 as i8;
RET.0 = _8 as f32;
RET.0 = _8 as f32;
_9 = RET.1;
RET.1 = [_1,_1,_2.2,_1,_2.2,_1];
_5 = false ^ false;
_4.1 = 678597603_u32 as u128;
_1 = _2.2;
Goto(bb3)
}
bb3 = {
_2 = (_10, _3, _1);
_2.1 = RET.2 as f64;
RET.0 = RET.2 as f32;
_2.0 = 1147149065_u32 as isize;
_2.1 = _3;
_4.0 = [8398773565420462527_u64,11001879226055223188_u64];
_14 = 14269002939488187030_u64 as f32;
_16 = !3582302684_u32;
RET.0 = _14;
_16 = 200_u8 as u32;
_4.0 = [9307488866879058457_u64,9945764012750344740_u64];
_12 = _16 as isize;
RET.1 = [_2.2,_2.2,_1,_1,_1,_1];
_15 = (-30252_i16) as isize;
Goto(bb4)
}
bb4 = {
_2 = (_12, _3, _1);
RET.2 = (-5172852779501603841_i64) ^ 7605158416652862242_i64;
_5 = !false;
RET.1 = [_1,_1,_2.2,_2.2,_2.2,_2.2];
RET.0 = 1_usize as f32;
RET = (_14, _9, (-5206227067155911425_i64));
_17 = _1 as isize;
_8 = (-30252_i16) as i8;
_16 = 3785994457_u32;
_14 = -RET.0;
_12 = _10 >> _2.2;
_14 = -RET.0;
RET.0 = _14;
_7 = ('\u{80ce1}',);
_2 = (_12, _3, _1);
_16 = (-96214365775622914026980979512068553210_i128) as u32;
Goto(bb5)
}
bb5 = {
_13 = 215670531_i32;
RET.1 = _9;
RET = (_14, _9, 6844394548678818744_i64);
_17 = _10;
_7.0 = '\u{684b6}';
_4.1 = 45003192376703953616748545453668951434_u128 & 239646113105578051601696555406615155122_u128;
RET = (_14, _9, 3100979137858845514_i64);
RET.1 = [_2.2,_2.2,_1,_1,_1,_1];
_20 = _10;
_2.0 = _12 - _10;
_14 = -RET.0;
RET.2 = 2360819210543674250_i64 >> _15;
_2.2 = !_1;
_17 = _2.0;
_16 = 2603596356_u32;
_5 = !true;
Goto(bb6)
}
bb6 = {
_7 = ('\u{c9a6c}',);
RET = (_14, _9, (-1722357057869986673_i64));
match RET.2 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463461652250373898224783 => bb8,
_ => bb7
}
}
bb7 = {
_5 = !false;
_9 = [_1,_1,_1,_1,_1,_1];
_4.0 = [12244417339182308264_u64,1454733020943583908_u64];
_4.0 = [8320159221138492219_u64,12259004510989557021_u64];
RET.0 = (-87047949_i32) as f32;
_7 = ('\u{3220}',);
_10 = _5 as isize;
_6 = [2934451690_u32,481159117_u32,40289864_u32,1446296459_u32,941707627_u32];
_8 = (-56_i8);
_7.0 = '\u{5e3c1}';
_9 = RET.1;
_8 = _7.0 as i8;
RET.0 = _8 as f32;
RET.0 = _8 as f32;
_9 = RET.1;
RET.1 = [_1,_1,_2.2,_1,_2.2,_1];
_5 = false ^ false;
_4.1 = 678597603_u32 as u128;
_1 = _2.2;
Goto(bb3)
}
bb8 = {
_22.fld0.2 = RET.2 >> _15;
_12 = _17;
RET.1 = _9;
_10 = _12 ^ _17;
_10 = -_12;
_10 = _4.1 as isize;
_3 = _2.1;
_7 = ('\u{b0c7d}',);
_17 = _12;
_13 = _16 as i32;
_15 = -_17;
_24.1 = _5;
_9 = RET.1;
_4.1 = (-141521368741029762317288465976926926002_i128) as u128;
_18 = [_13,_13,_13,_13,_13,_13,_13,_13];
_1 = _8 as u16;
_27 = _7.0;
_14 = RET.0;
_22.fld0.1 = _9;
_7.0 = _27;
_23 = _17;
_2.2 = 6899197574070272067_u64 as u16;
_16 = 498269425_u32;
Goto(bb9)
}
bb9 = {
_8 = (-28_i8);
_24.4 = RET.0 as u32;
_22.fld0.0 = _14 - RET.0;
_24.3 = core::ptr::addr_of!(RET.2);
match _8 {
0 => bb5,
1 => bb6,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
340282366920938463463374607431768211428 => bb15,
_ => bb14
}
}
bb10 = {
_3 = -_2.1;
_8 = (-115_i8) | 75_i8;
_4.0 = [7794340090202386411_u64,17880556551701424496_u64];
Goto(bb2)
}
bb11 = {
_5 = !false;
_9 = [_1,_1,_1,_1,_1,_1];
_4.0 = [12244417339182308264_u64,1454733020943583908_u64];
_4.0 = [8320159221138492219_u64,12259004510989557021_u64];
RET.0 = (-87047949_i32) as f32;
_7 = ('\u{3220}',);
_10 = _5 as isize;
_6 = [2934451690_u32,481159117_u32,40289864_u32,1446296459_u32,941707627_u32];
_8 = (-56_i8);
_7.0 = '\u{5e3c1}';
_9 = RET.1;
_8 = _7.0 as i8;
RET.0 = _8 as f32;
RET.0 = _8 as f32;
_9 = RET.1;
RET.1 = [_1,_1,_2.2,_1,_2.2,_1];
_5 = false ^ false;
_4.1 = 678597603_u32 as u128;
_1 = _2.2;
Goto(bb3)
}
bb12 = {
_7 = ('\u{c9a6c}',);
RET = (_14, _9, (-1722357057869986673_i64));
match RET.2 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463461652250373898224783 => bb8,
_ => bb7
}
}
bb13 = {
_13 = 215670531_i32;
RET.1 = _9;
RET = (_14, _9, 6844394548678818744_i64);
_17 = _10;
_7.0 = '\u{684b6}';
_4.1 = 45003192376703953616748545453668951434_u128 & 239646113105578051601696555406615155122_u128;
RET = (_14, _9, 3100979137858845514_i64);
RET.1 = [_2.2,_2.2,_1,_1,_1,_1];
_20 = _10;
_2.0 = _12 - _10;
_14 = -RET.0;
RET.2 = 2360819210543674250_i64 >> _15;
_2.2 = !_1;
_17 = _2.0;
_16 = 2603596356_u32;
_5 = !true;
Goto(bb6)
}
bb14 = {
_2 = (_10, _3, _1);
_2.1 = RET.2 as f64;
RET.0 = RET.2 as f32;
_2.0 = 1147149065_u32 as isize;
_2.1 = _3;
_4.0 = [8398773565420462527_u64,11001879226055223188_u64];
_14 = 14269002939488187030_u64 as f32;
_16 = !3582302684_u32;
RET.0 = _14;
_16 = 200_u8 as u32;
_4.0 = [9307488866879058457_u64,9945764012750344740_u64];
_12 = _16 as isize;
RET.1 = [_2.2,_2.2,_1,_1,_1,_1];
_15 = (-30252_i16) as isize;
Goto(bb4)
}
bb15 = {
RET = (_22.fld0.0, _9, _22.fld0.2);
_7.0 = _27;
_15 = !_10;
RET = _22.fld0;
_7 = (_27,);
RET.2 = -_22.fld0.2;
_10 = 70_u8 as isize;
RET.1 = [_1,_1,_1,_1,_2.2,_2.2];
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(14_usize, 18_usize, Move(_18), 23_usize, Move(_23), 4_usize, Move(_4), 13_usize, Move(_13)), bb17, UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(14_usize, 1_usize, Move(_1), 12_usize, Move(_12), 7_usize, Move(_7), 9_usize, Move(_9)), bb18, UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: [u16; 6]) -> i64 {
mir! {
type RET = i64;
let _3: isize;
let _4: [i16; 6];
let _5: (f32, [u16; 6], i64);
let _6: u64;
let _7: isize;
let _8: (i128, i128, i8);
let _9: f32;
let _10: Adt53;
let _11: [char; 6];
let _12: Adt50;
let _13: f64;
let _14: f64;
let _15: (i16,);
let _16: f64;
let _17: (i128, i128, i8);
let _18: bool;
let _19: [u16; 6];
let _20: (*const i16, bool, [u64; 2], *const i64, u32);
let _21: isize;
let _22: Adt53;
let _23: ();
let _24: ();
{
RET = (-59_i8) as i64;
_2 = [41695_u16,42811_u16,25407_u16,65191_u16,15611_u16,52946_u16];
RET = 3331219922528182248_i64;
RET = -560872681655697885_i64;
RET = '\u{b1475}' as i64;
_1 = (-9223372036854775808_isize);
_2 = [60415_u16,17468_u16,59825_u16,19498_u16,19089_u16,2875_u16];
RET = !2623875815262056547_i64;
_3 = (-60720248242620005773157542977675425538_i128) as isize;
RET = !(-1527934764077902168_i64);
Call(RET = fn16(_1, _1, _2, _2, _3, _1, _1, _2, _1, _1, _2, _3, _2), bb1, UnwindUnreachable())
}
bb1 = {
RET = -(-2616085417577646825_i64);
_3 = (-57_i8) as isize;
_1 = 40770_u16 as isize;
RET = !4466817329619220025_i64;
_1 = _3;
_1 = !_3;
RET = 5735325428172359477_i64 * (-2315941175010402555_i64);
_3 = 1801224774_i32 as isize;
RET = (-6351120076234123270_i64);
_4 = [(-9535_i16),(-31558_i16),20976_i16,(-2212_i16),(-27112_i16),26351_i16];
_5.0 = 1717581308_u32 as f32;
_5.2 = -RET;
_4 = [(-30059_i16),14435_i16,(-7569_i16),27189_i16,(-24350_i16),1416_i16];
_5.2 = -RET;
_1 = _3;
_1 = _3;
RET = _5.2 << _3;
_5.2 = RET | RET;
RET = 15606_u16 as i64;
_3 = _1;
_2 = [58362_u16,52680_u16,48291_u16,13770_u16,34650_u16,14306_u16];
_5.0 = (-1068117381_i32) as f32;
_5.1 = [25191_u16,55158_u16,30718_u16,16397_u16,11005_u16,33777_u16];
_5.2 = 253258062251846170949643304511396992983_u128 as i64;
_1 = true as isize;
Goto(bb2)
}
bb2 = {
_5.1 = [37282_u16,20017_u16,29098_u16,53251_u16,47246_u16,61182_u16];
_5.1 = [15232_u16,7811_u16,5286_u16,62412_u16,10481_u16,32982_u16];
_1 = 1990335034_i32 as isize;
_5.0 = 298197193430722587837928809459170238260_u128 as f32;
_3 = _1 & _1;
_1 = 101_u8 as isize;
_5.2 = -RET;
_1 = _3;
_5.1 = [5639_u16,29236_u16,54676_u16,38864_u16,39821_u16,43908_u16];
_1 = !_3;
RET = (-24043_i16) as i64;
_4 = [24597_i16,(-23371_i16),6316_i16,24001_i16,(-4072_i16),18110_i16];
_5.2 = RET * RET;
Goto(bb3)
}
bb3 = {
_5.0 = 13779159693521962938_u64 as f32;
RET = _5.2;
_3 = _1;
_4 = [(-29358_i16),(-30387_i16),(-8017_i16),(-29446_i16),(-766_i16),(-19382_i16)];
_6 = 15490064392322426371_u64 + 17403334564686095259_u64;
_5.2 = RET | RET;
RET = _5.2 + _5.2;
_1 = !_3;
_6 = 5250547631667905870_u64 ^ 13298490143664210198_u64;
_6 = 43_i8 as u64;
RET = -_5.2;
_5.0 = 3099902434_u32 as f32;
RET = -_5.2;
_5.2 = !RET;
RET = _5.2 + _5.2;
RET = -_5.2;
_8.0 = '\u{f7cf6}' as i128;
_4 = [(-15625_i16),(-1013_i16),(-4591_i16),(-17398_i16),25381_i16,20394_i16];
_5.2 = _8.0 as i64;
_5.0 = 120_u8 as f32;
_7 = _3;
RET = _5.2 - _5.2;
Goto(bb4)
}
bb4 = {
_5.1 = [353_u16,54435_u16,46004_u16,25764_u16,34679_u16,45616_u16];
_8 = (39000444363408738148762511749401507683_i128, (-69114314944893356178857013914036176453_i128), 72_i8);
_4 = [(-30950_i16),(-32766_i16),8691_i16,(-12221_i16),934_i16,(-4155_i16)];
_5.1 = _2;
_8.0 = -_8.1;
_9 = -_5.0;
_5.0 = _9;
_5 = (_9, _2, RET);
_5.2 = RET;
_8.2 = 47_i8 | (-69_i8);
_5.0 = _9 * _9;
_6 = 18831650240784121631445394923644412740_u128 as u64;
_5.0 = _9 * _9;
Goto(bb5)
}
bb5 = {
_9 = _5.0 - _5.0;
_12.fld1 = ['\u{6eecd}','\u{942b2}','\u{cf7ab}','\u{7118b}','\u{f1ddc}','\u{bfe7c}'];
_12.fld0.2 = _5.2;
_12.fld0.1 = [17427_u16,14267_u16,43233_u16,55693_u16,1436_u16,45015_u16];
_5.0 = _9 + _9;
_3 = _6 as isize;
_11 = _12.fld1;
_7 = !_1;
_6 = !1569729986286349978_u64;
_8.2 = 124_i8 ^ 19_i8;
_11 = ['\u{1797}','\u{2a2c6}','\u{677cb}','\u{cb6ab}','\u{c9196}','\u{7de06}'];
_12.fld1 = ['\u{7d2ba}','\u{e33f7}','\u{a0bcb}','\u{74001}','\u{b6835}','\u{aa9c9}'];
_11 = _12.fld1;
_12 = Adt50 { fld0: _5,fld1: _11 };
_11 = _12.fld1;
_8.0 = _8.1 + _8.1;
_3 = 107022119232506226469632605705521555401_u128 as isize;
_4 = [27471_i16,(-1704_i16),(-3627_i16),(-4509_i16),27055_i16,3723_i16];
_8 = (123139126758737772490964685574302281986_i128, (-168626447782609845163214200230697353976_i128), 45_i8);
_5.0 = _8.2 as f32;
RET = _5.2 << _5.2;
_12.fld0.2 = !RET;
_12.fld0.2 = '\u{a068b}' as i64;
match _8.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
45 => bb10,
_ => bb9
}
}
bb6 = {
_5.1 = [353_u16,54435_u16,46004_u16,25764_u16,34679_u16,45616_u16];
_8 = (39000444363408738148762511749401507683_i128, (-69114314944893356178857013914036176453_i128), 72_i8);
_4 = [(-30950_i16),(-32766_i16),8691_i16,(-12221_i16),934_i16,(-4155_i16)];
_5.1 = _2;
_8.0 = -_8.1;
_9 = -_5.0;
_5.0 = _9;
_5 = (_9, _2, RET);
_5.2 = RET;
_8.2 = 47_i8 | (-69_i8);
_5.0 = _9 * _9;
_6 = 18831650240784121631445394923644412740_u128 as u64;
_5.0 = _9 * _9;
Goto(bb5)
}
bb7 = {
_5.0 = 13779159693521962938_u64 as f32;
RET = _5.2;
_3 = _1;
_4 = [(-29358_i16),(-30387_i16),(-8017_i16),(-29446_i16),(-766_i16),(-19382_i16)];
_6 = 15490064392322426371_u64 + 17403334564686095259_u64;
_5.2 = RET | RET;
RET = _5.2 + _5.2;
_1 = !_3;
_6 = 5250547631667905870_u64 ^ 13298490143664210198_u64;
_6 = 43_i8 as u64;
RET = -_5.2;
_5.0 = 3099902434_u32 as f32;
RET = -_5.2;
_5.2 = !RET;
RET = _5.2 + _5.2;
RET = -_5.2;
_8.0 = '\u{f7cf6}' as i128;
_4 = [(-15625_i16),(-1013_i16),(-4591_i16),(-17398_i16),25381_i16,20394_i16];
_5.2 = _8.0 as i64;
_5.0 = 120_u8 as f32;
_7 = _3;
RET = _5.2 - _5.2;
Goto(bb4)
}
bb8 = {
_5.1 = [37282_u16,20017_u16,29098_u16,53251_u16,47246_u16,61182_u16];
_5.1 = [15232_u16,7811_u16,5286_u16,62412_u16,10481_u16,32982_u16];
_1 = 1990335034_i32 as isize;
_5.0 = 298197193430722587837928809459170238260_u128 as f32;
_3 = _1 & _1;
_1 = 101_u8 as isize;
_5.2 = -RET;
_1 = _3;
_5.1 = [5639_u16,29236_u16,54676_u16,38864_u16,39821_u16,43908_u16];
_1 = !_3;
RET = (-24043_i16) as i64;
_4 = [24597_i16,(-23371_i16),6316_i16,24001_i16,(-4072_i16),18110_i16];
_5.2 = RET * RET;
Goto(bb3)
}
bb9 = {
RET = -(-2616085417577646825_i64);
_3 = (-57_i8) as isize;
_1 = 40770_u16 as isize;
RET = !4466817329619220025_i64;
_1 = _3;
_1 = !_3;
RET = 5735325428172359477_i64 * (-2315941175010402555_i64);
_3 = 1801224774_i32 as isize;
RET = (-6351120076234123270_i64);
_4 = [(-9535_i16),(-31558_i16),20976_i16,(-2212_i16),(-27112_i16),26351_i16];
_5.0 = 1717581308_u32 as f32;
_5.2 = -RET;
_4 = [(-30059_i16),14435_i16,(-7569_i16),27189_i16,(-24350_i16),1416_i16];
_5.2 = -RET;
_1 = _3;
_1 = _3;
RET = _5.2 << _3;
_5.2 = RET | RET;
RET = 15606_u16 as i64;
_3 = _1;
_2 = [58362_u16,52680_u16,48291_u16,13770_u16,34650_u16,14306_u16];
_5.0 = (-1068117381_i32) as f32;
_5.1 = [25191_u16,55158_u16,30718_u16,16397_u16,11005_u16,33777_u16];
_5.2 = 253258062251846170949643304511396992983_u128 as i64;
_1 = true as isize;
Goto(bb2)
}
bb10 = {
_4 = [(-20622_i16),21464_i16,(-10374_i16),(-6931_i16),(-26808_i16),(-2138_i16)];
_5.1 = [6860_u16,21812_u16,729_u16,52834_u16,11787_u16,28655_u16];
_8.1 = _5.2 as i128;
_2 = _5.1;
_8.0 = _8.1 >> _5.2;
_5 = (_12.fld0.0, _12.fld0.1, RET);
RET = _5.2;
_8.0 = _8.1 | _8.1;
_9 = _12.fld0.0;
_3 = _7;
RET = 14_u8 as i64;
_7 = _1;
_5.0 = _9 + _12.fld0.0;
_12.fld0.1 = [51787_u16,40421_u16,15530_u16,53799_u16,25445_u16,8709_u16];
_12.fld0.2 = RET;
_11 = ['\u{cfa5e}','\u{3f0cd}','\u{1b8b6}','\u{304a5}','\u{56f93}','\u{c6a38}'];
_8.1 = '\u{d4571}' as i128;
_11 = _12.fld1;
_8 = ((-161645237856220986918823659870342978393_i128), (-26877746333035062676221981276158565946_i128), 90_i8);
_9 = -_12.fld0.0;
RET = _5.2;
_12.fld1 = ['\u{5b3af}','\u{3e98}','\u{44696}','\u{102bd0}','\u{879f0}','\u{a63f8}'];
_1 = !_7;
_12.fld0.1 = [8618_u16,36563_u16,55341_u16,29338_u16,13912_u16,21342_u16];
_1 = _5.0 as isize;
_7 = -_1;
RET = _12.fld0.2 - _5.2;
_3 = _7;
_13 = _8.0 as f64;
Goto(bb11)
}
bb11 = {
_12.fld0.1 = _2;
_12.fld1 = ['\u{1cc42}','\u{d3ea1}','\u{10464b}','\u{851f9}','\u{6e35e}','\u{4e1e2}'];
RET = _5.2 * _5.2;
_8.1 = _8.0;
_12.fld0.0 = _13 as f32;
_5.2 = RET;
_12.fld0.1 = [4514_u16,59717_u16,38599_u16,54251_u16,1587_u16,26790_u16];
_1 = _5.2 as isize;
_12.fld0.0 = -_5.0;
_8.2 = 51943_u16 as i8;
_17 = (_8.1, _8.0, _8.2);
_12.fld0.0 = 66841334722409992858623994680478703374_u128 as f32;
_13 = 276834995153008324729991873035964715212_u128 as f64;
_16 = 191_u8 as f64;
_15 = ((-4635_i16),);
_4 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_14 = _16;
RET = _12.fld0.2;
_14 = _13 - _16;
Goto(bb12)
}
bb12 = {
_12.fld0.0 = _9;
_14 = (-1639739602_i32) as f64;
_18 = false;
_8.0 = _17.1 << _3;
_15 = (23080_i16,);
RET = !_5.2;
_5.2 = RET;
_17.0 = _6 as i128;
match _8.1 {
0 => bb5,
1 => bb2,
2 => bb13,
3 => bb14,
4 => bb15,
178637129064717476544550947561425233063 => bb17,
_ => bb16
}
}
bb13 = {
_12.fld0.1 = _2;
_12.fld1 = ['\u{1cc42}','\u{d3ea1}','\u{10464b}','\u{851f9}','\u{6e35e}','\u{4e1e2}'];
RET = _5.2 * _5.2;
_8.1 = _8.0;
_12.fld0.0 = _13 as f32;
_5.2 = RET;
_12.fld0.1 = [4514_u16,59717_u16,38599_u16,54251_u16,1587_u16,26790_u16];
_1 = _5.2 as isize;
_12.fld0.0 = -_5.0;
_8.2 = 51943_u16 as i8;
_17 = (_8.1, _8.0, _8.2);
_12.fld0.0 = 66841334722409992858623994680478703374_u128 as f32;
_13 = 276834995153008324729991873035964715212_u128 as f64;
_16 = 191_u8 as f64;
_15 = ((-4635_i16),);
_4 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_14 = _16;
RET = _12.fld0.2;
_14 = _13 - _16;
Goto(bb12)
}
bb14 = {
_4 = [(-20622_i16),21464_i16,(-10374_i16),(-6931_i16),(-26808_i16),(-2138_i16)];
_5.1 = [6860_u16,21812_u16,729_u16,52834_u16,11787_u16,28655_u16];
_8.1 = _5.2 as i128;
_2 = _5.1;
_8.0 = _8.1 >> _5.2;
_5 = (_12.fld0.0, _12.fld0.1, RET);
RET = _5.2;
_8.0 = _8.1 | _8.1;
_9 = _12.fld0.0;
_3 = _7;
RET = 14_u8 as i64;
_7 = _1;
_5.0 = _9 + _12.fld0.0;
_12.fld0.1 = [51787_u16,40421_u16,15530_u16,53799_u16,25445_u16,8709_u16];
_12.fld0.2 = RET;
_11 = ['\u{cfa5e}','\u{3f0cd}','\u{1b8b6}','\u{304a5}','\u{56f93}','\u{c6a38}'];
_8.1 = '\u{d4571}' as i128;
_11 = _12.fld1;
_8 = ((-161645237856220986918823659870342978393_i128), (-26877746333035062676221981276158565946_i128), 90_i8);
_9 = -_12.fld0.0;
RET = _5.2;
_12.fld1 = ['\u{5b3af}','\u{3e98}','\u{44696}','\u{102bd0}','\u{879f0}','\u{a63f8}'];
_1 = !_7;
_12.fld0.1 = [8618_u16,36563_u16,55341_u16,29338_u16,13912_u16,21342_u16];
_1 = _5.0 as isize;
_7 = -_1;
RET = _12.fld0.2 - _5.2;
_3 = _7;
_13 = _8.0 as f64;
Goto(bb11)
}
bb15 = {
_9 = _5.0 - _5.0;
_12.fld1 = ['\u{6eecd}','\u{942b2}','\u{cf7ab}','\u{7118b}','\u{f1ddc}','\u{bfe7c}'];
_12.fld0.2 = _5.2;
_12.fld0.1 = [17427_u16,14267_u16,43233_u16,55693_u16,1436_u16,45015_u16];
_5.0 = _9 + _9;
_3 = _6 as isize;
_11 = _12.fld1;
_7 = !_1;
_6 = !1569729986286349978_u64;
_8.2 = 124_i8 ^ 19_i8;
_11 = ['\u{1797}','\u{2a2c6}','\u{677cb}','\u{cb6ab}','\u{c9196}','\u{7de06}'];
_12.fld1 = ['\u{7d2ba}','\u{e33f7}','\u{a0bcb}','\u{74001}','\u{b6835}','\u{aa9c9}'];
_11 = _12.fld1;
_12 = Adt50 { fld0: _5,fld1: _11 };
_11 = _12.fld1;
_8.0 = _8.1 + _8.1;
_3 = 107022119232506226469632605705521555401_u128 as isize;
_4 = [27471_i16,(-1704_i16),(-3627_i16),(-4509_i16),27055_i16,3723_i16];
_8 = (123139126758737772490964685574302281986_i128, (-168626447782609845163214200230697353976_i128), 45_i8);
_5.0 = _8.2 as f32;
RET = _5.2 << _5.2;
_12.fld0.2 = !RET;
_12.fld0.2 = '\u{a068b}' as i64;
match _8.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
45 => bb10,
_ => bb9
}
}
bb16 = {
RET = -(-2616085417577646825_i64);
_3 = (-57_i8) as isize;
_1 = 40770_u16 as isize;
RET = !4466817329619220025_i64;
_1 = _3;
_1 = !_3;
RET = 5735325428172359477_i64 * (-2315941175010402555_i64);
_3 = 1801224774_i32 as isize;
RET = (-6351120076234123270_i64);
_4 = [(-9535_i16),(-31558_i16),20976_i16,(-2212_i16),(-27112_i16),26351_i16];
_5.0 = 1717581308_u32 as f32;
_5.2 = -RET;
_4 = [(-30059_i16),14435_i16,(-7569_i16),27189_i16,(-24350_i16),1416_i16];
_5.2 = -RET;
_1 = _3;
_1 = _3;
RET = _5.2 << _3;
_5.2 = RET | RET;
RET = 15606_u16 as i64;
_3 = _1;
_2 = [58362_u16,52680_u16,48291_u16,13770_u16,34650_u16,14306_u16];
_5.0 = (-1068117381_i32) as f32;
_5.1 = [25191_u16,55158_u16,30718_u16,16397_u16,11005_u16,33777_u16];
_5.2 = 253258062251846170949643304511396992983_u128 as i64;
_1 = true as isize;
Goto(bb2)
}
bb17 = {
_13 = _14;
_17.0 = !_8.0;
_8.1 = _17.1;
_12.fld0 = (_9, _2, RET);
_20.1 = _18 | _18;
_16 = -_13;
_15 = (29582_i16,);
_7 = _3 | _3;
_8 = (_17.0, _17.0, _17.2);
_12.fld1 = _11;
_20.0 = core::ptr::addr_of!(_15.0);
_11 = _12.fld1;
_21 = !_3;
_12.fld1 = ['\u{39bb7}','\u{d28c3}','\u{66e60}','\u{a9d1e}','\u{48b6f}','\u{52e40}'];
RET = _5.2;
_9 = 3860894869_u32 as f32;
_19 = [23354_u16,28181_u16,17099_u16,59618_u16,33690_u16,34258_u16];
Goto(bb18)
}
bb18 = {
Call(_23 = dump_var(15_usize, 3_usize, Move(_3), 17_usize, Move(_17), 2_usize, Move(_2), 6_usize, Move(_6)), bb19, UnwindUnreachable())
}
bb19 = {
Call(_23 = dump_var(15_usize, 11_usize, Move(_11), 19_usize, Move(_19), 24_usize, _24, 24_usize, _24), bb20, UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: [u16; 6],mut _4: [u16; 6],mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [u16; 6],mut _9: isize,mut _10: isize,mut _11: [u16; 6],mut _12: isize,mut _13: [u16; 6]) -> i64 {
mir! {
type RET = i64;
let _14: bool;
let _15: f32;
let _16: u16;
let _17: isize;
let _18: f32;
let _19: i16;
let _20: isize;
let _21: f32;
let _22: u64;
let _23: Adt57;
let _24: isize;
let _25: Adt52;
let _26: *mut u32;
let _27: *const i16;
let _28: Adt50;
let _29: bool;
let _30: [u8; 8];
let _31: isize;
let _32: char;
let _33: usize;
let _34: Adt51;
let _35: f32;
let _36: Adt63;
let _37: Adt60;
let _38: [u64; 2];
let _39: f32;
let _40: (char,);
let _41: char;
let _42: Adt61;
let _43: bool;
let _44: isize;
let _45: u128;
let _46: ();
let _47: ();
{
_6 = _9;
_6 = -_5;
_3 = [57171_u16,63606_u16,65121_u16,9345_u16,61478_u16,51647_u16];
_13 = [7564_u16,23947_u16,6120_u16,812_u16,24513_u16,9058_u16];
_11 = [56740_u16,23923_u16,22030_u16,28574_u16,33727_u16,19346_u16];
_12 = -_9;
_4 = _8;
_11 = [25590_u16,47666_u16,1795_u16,28145_u16,18378_u16,49265_u16];
_9 = 415602518_i32 as isize;
_4 = [1768_u16,4023_u16,49974_u16,14339_u16,42697_u16,50574_u16];
_5 = _2;
RET = _10 as i64;
_15 = 117339034_u32 as f32;
_8 = [32792_u16,104_u16,17632_u16,21006_u16,53886_u16,7475_u16];
_3 = [5926_u16,31739_u16,36900_u16,56686_u16,2882_u16,49278_u16];
_4 = [47895_u16,31702_u16,14200_u16,65023_u16,53663_u16,60372_u16];
_14 = !true;
_8 = _4;
_10 = !_5;
Goto(bb1)
}
bb1 = {
_3 = [5137_u16,47414_u16,33627_u16,39587_u16,28396_u16,56818_u16];
_14 = !true;
_14 = false;
_1 = _6 ^ _7;
Goto(bb2)
}
bb2 = {
_4 = [23167_u16,11701_u16,14202_u16,54584_u16,52904_u16,9516_u16];
_4 = [26136_u16,37977_u16,54039_u16,17431_u16,14048_u16,14973_u16];
_4 = [21562_u16,24739_u16,8736_u16,61774_u16,48407_u16,44815_u16];
_10 = _1 - _6;
_6 = _1;
_3 = [15703_u16,37546_u16,25329_u16,38302_u16,34019_u16,29672_u16];
_2 = !_10;
_8 = _3;
_6 = _9 | _10;
_17 = 5969417503657163262_u64 as isize;
_10 = _15 as isize;
RET = 79_u8 as i64;
_16 = 18598_i16 as u16;
_6 = _2 + _1;
_20 = 53_i8 as isize;
_16 = 36484_u16;
_18 = 13434938723988409139_u64 as f32;
_17 = 47_u8 as isize;
match _7 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb3 = {
_3 = [5137_u16,47414_u16,33627_u16,39587_u16,28396_u16,56818_u16];
_14 = !true;
_14 = false;
_1 = _6 ^ _7;
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
_12 = _2;
_18 = _15 - _15;
_15 = 301260167130932789912713088461609776117_u128 as f32;
RET = 160146821956167211290672154236283862744_i128 as i64;
_8 = [_16,_16,_16,_16,_16,_16];
_17 = _6 ^ _2;
_14 = !true;
_21 = _18;
_10 = _6 + _6;
_16 = 58194_u16;
_19 = RET as i16;
_9 = 15761754977066239593_usize as isize;
RET = 8153097201719216204_i64;
_9 = _10;
_13 = _3;
_20 = _18 as isize;
_3 = [_16,_16,_16,_16,_16,_16];
_9 = _17 & _6;
_10 = !_9;
_21 = _18 * _18;
_4 = [_16,_16,_16,_16,_16,_16];
Call(_10 = core::intrinsics::transmute(_20), bb9, UnwindUnreachable())
}
bb9 = {
RET = (-3915145424255624472_i64) ^ (-2106624237224025996_i64);
_8 = [_16,_16,_16,_16,_16,_16];
_10 = RET as isize;
_20 = _10;
RET = 565929259940673225_i64;
_28.fld0 = (_18, _13, RET);
_28.fld1 = ['\u{eba1f}','\u{81cc7}','\u{69f59}','\u{d5c53}','\u{e90f3}','\u{7a31a}'];
_28.fld0.2 = -RET;
Goto(bb10)
}
bb10 = {
_14 = !true;
RET = _28.fld0.2;
_17 = _6 - _9;
_2 = _9;
_30 = [39_u8,96_u8,153_u8,87_u8,251_u8,143_u8,201_u8,75_u8];
_10 = _17 * _9;
_28.fld1 = ['\u{65c01}','\u{844c7}','\u{6a6ff}','\u{af369}','\u{102579}','\u{4f135}'];
_31 = _9;
_22 = 8759931901169026009_u64 | 415451505584722946_u64;
_2 = !_17;
_3 = _28.fld0.1;
_31 = _14 as isize;
_27 = core::ptr::addr_of!(_19);
_14 = !false;
_19 = (-26764_i16);
_20 = !_2;
_28.fld0.0 = _16 as f32;
_22 = 17969090954866392209_u64 + 16286186459981423777_u64;
_18 = -_28.fld0.0;
_28.fld0.2 = _14 as i64;
Call(_22 = core::intrinsics::bswap(3625198782877347812_u64), bb11, UnwindUnreachable())
}
bb11 = {
_28.fld0.0 = 253_u8 as f32;
(*_27) = -13017_i16;
_10 = (-147514346158677345151473804875080741081_i128) as isize;
_28.fld0.0 = _22 as f32;
_28.fld0 = (_21, _3, RET);
_18 = _22 as f32;
_27 = core::ptr::addr_of!(_19);
_33 = 5_usize;
_20 = 96_i8 as isize;
_32 = _28.fld1[_33];
_22 = 7580044413241670582_u64 | 12848473590137867569_u64;
_28.fld0.2 = RET;
_14 = (*_27) != (*_27);
_10 = _2;
_7 = !_9;
_1 = _9 * _6;
_16 = _11[_33];
_24 = !_7;
_24 = !_17;
_28.fld0.0 = -_21;
_34.fld3 = (_32,);
match _11[_33] {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb7,
4 => bb5,
49265 => bb13,
_ => bb12
}
}
bb12 = {
_12 = _2;
_18 = _15 - _15;
_15 = 301260167130932789912713088461609776117_u128 as f32;
RET = 160146821956167211290672154236283862744_i128 as i64;
_8 = [_16,_16,_16,_16,_16,_16];
_17 = _6 ^ _2;
_14 = !true;
_21 = _18;
_10 = _6 + _6;
_16 = 58194_u16;
_19 = RET as i16;
_9 = 15761754977066239593_usize as isize;
RET = 8153097201719216204_i64;
_9 = _10;
_13 = _3;
_20 = _18 as isize;
_3 = [_16,_16,_16,_16,_16,_16];
_9 = _17 & _6;
_10 = !_9;
_21 = _18 * _18;
_4 = [_16,_16,_16,_16,_16,_16];
Call(_10 = core::intrinsics::transmute(_20), bb9, UnwindUnreachable())
}
bb13 = {
_21 = _28.fld0.0 + _28.fld0.0;
_14 = !false;
_32 = _34.fld3.0;
_32 = _34.fld3.0;
_34.fld3.0 = _32;
_34.fld0.1 = _14;
_28.fld1[_33] = _34.fld3.0;
_34.fld0.0 = core::ptr::addr_of!((*_27));
_18 = _21 - _21;
_26 = core::ptr::addr_of_mut!(_34.fld0.4);
_20 = !_10;
_18 = -_21;
_11 = [_8[_33],_16,_16,_13[_33],_13[_33],_28.fld0.1[_33]];
_28.fld0.1[_33] = _24 as u16;
_34.fld6 = _33 + _33;
_32 = _28.fld1[_33];
_8[_33] = _28.fld0.1[_33] ^ _13[_33];
_39 = -_21;
_28.fld0.1 = [_3[_33],_8[_33],_8[_33],_8[_33],_8[_33],_3[_33]];
RET = _28.fld0.2;
_14 = _34.fld3.0 < _28.fld1[_33];
RET = _28.fld0.2 - _28.fld0.2;
_34.fld0.0 = core::ptr::addr_of!(_19);
_29 = !_14;
_41 = _34.fld3.0;
_25.fld0 = core::ptr::addr_of_mut!((*_26));
match _30[_33] {
0 => bb14,
1 => bb15,
2 => bb16,
143 => bb18,
_ => bb17
}
}
bb14 = {
Return()
}
bb15 = {
_28.fld0.0 = 253_u8 as f32;
(*_27) = -13017_i16;
_10 = (-147514346158677345151473804875080741081_i128) as isize;
_28.fld0.0 = _22 as f32;
_28.fld0 = (_21, _3, RET);
_18 = _22 as f32;
_27 = core::ptr::addr_of!(_19);
_33 = 5_usize;
_20 = 96_i8 as isize;
_32 = _28.fld1[_33];
_22 = 7580044413241670582_u64 | 12848473590137867569_u64;
_28.fld0.2 = RET;
_14 = (*_27) != (*_27);
_10 = _2;
_7 = !_9;
_1 = _9 * _6;
_16 = _11[_33];
_24 = !_7;
_24 = !_17;
_28.fld0.0 = -_21;
_34.fld3 = (_32,);
match _11[_33] {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb7,
4 => bb5,
49265 => bb13,
_ => bb12
}
}
bb16 = {
_3 = [5137_u16,47414_u16,33627_u16,39587_u16,28396_u16,56818_u16];
_14 = !true;
_14 = false;
_1 = _6 ^ _7;
Goto(bb2)
}
bb17 = {
Return()
}
bb18 = {
_34.fld2.2 = !_11[_33];
_35 = -_28.fld0.0;
_34.fld0.0 = _27;
_34.fld2.2 = _3[_33];
_24 = _20;
_41 = _34.fld3.0;
_42.fld0 = _28.fld0.2 as u16;
_9 = _17;
_40 = (_32,);
_44 = !_1;
_30[_33] = _40.0 as u8;
_34.fld0.3 = core::ptr::addr_of!(_28.fld0.2);
_44 = _33 as isize;
_34.fld2.1 = (-1477344781_i32) as f64;
RET = !_28.fld0.2;
_6 = _1;
_14 = _29;
_42.fld0 = _3[_33] & _8[_33];
_38 = [_22,_22];
Goto(bb19)
}
bb19 = {
Call(_46 = dump_var(16_usize, 13_usize, Move(_13), 1_usize, Move(_1), 30_usize, Move(_30), 44_usize, Move(_44)), bb20, UnwindUnreachable())
}
bb20 = {
Call(_46 = dump_var(16_usize, 6_usize, Move(_6), 5_usize, Move(_5), 38_usize, Move(_38), 17_usize, Move(_17)), bb21, UnwindUnreachable())
}
bb21 = {
Call(_46 = dump_var(16_usize, 29_usize, Move(_29), 11_usize, Move(_11), 3_usize, Move(_3), 2_usize, Move(_2)), bb22, UnwindUnreachable())
}
bb22 = {
Call(_46 = dump_var(16_usize, 9_usize, Move(_9), 41_usize, Move(_41), 47_usize, _47, 47_usize, _47), bb23, UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i64,mut _2: u16,mut _3: (i8, isize, u32, f64),mut _4: *mut u32,mut _5: (i8, isize, u32, f64),mut _6: (*const i16, bool, [u64; 2], *const i64, u32),mut _7: *mut u32,mut _8: (i8, isize, u32, f64),mut _9: (*const i16, bool, [u64; 2], *const i64, u32)) -> isize {
mir! {
type RET = isize;
let _10: f64;
let _11: Adt49;
let _12: ();
let _13: ();
{
_7 = _4;
_9 = (_6.0, _6.1, _6.2, _6.3, _5.2);
_5.0 = -_8.0;
_3.3 = (-168854667126129808154989813588036761902_i128) as f64;
_5 = _3;
Goto(bb1)
}
bb1 = {
_5.1 = !_8.1;
(*_4) = _5.2;
_9 = (_6.0, _6.1, _6.2, _6.3, _3.2);
_9.4 = (-553112251_i32) as u32;
_5.1 = -_8.1;
_9.2 = _6.2;
_3 = (_5.0, _5.1, (*_7), _8.3);
_5.1 = _3.1 + _3.1;
RET = _8.1 * _3.1;
_5.1 = _3.3 as isize;
_6.3 = core::ptr::addr_of!(_1);
_6.2 = [1435960389786752673_u64,609038029753940013_u64];
_3.2 = !_6.4;
_3.1 = _8.1 << RET;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(17_usize, 1_usize, Move(_1), 13_usize, _13, 13_usize, _13, 13_usize, _13), bb3, UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: [u32; 5],mut _3: u32) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _4: u128;
let _5: [i16; 6];
let _6: [u32; 5];
let _7: (isize, f64, u16);
let _8: f32;
let _9: (f32, [u16; 6], i64);
let _10: [i32; 1];
let _11: u64;
let _12: [char; 5];
let _13: i16;
let _14: Adt48;
let _15: [i32; 1];
let _16: [i32; 8];
let _17: Adt60;
let _18: i32;
let _19: u16;
let _20: Adt53;
let _21: [u64; 2];
let _22: isize;
let _23: Adt50;
let _24: u32;
let _25: i8;
let _26: [i16; 6];
let _27: *const i64;
let _28: Adt62;
let _29: isize;
let _30: isize;
let _31: Adt55;
let _32: *mut f64;
let _33: (f32, [u16; 6], i64);
let _34: Adt53;
let _35: usize;
let _36: f32;
let _37: ();
let _38: ();
{
RET = [1761692175_i32,(-1315076709_i32),(-893708475_i32),1316702411_i32,1587830452_i32,2098133752_i32,(-1501676398_i32),(-1008053517_i32)];
_3 = false as u32;
Goto(bb1)
}
bb1 = {
_6 = [_3,_3,_3,_3,_3];
Goto(bb2)
}
bb2 = {
_3 = !3689213373_u32;
_7.2 = (-5842907916851162560_i64) as u16;
_7.2 = 28846_u16;
_7.0 = !_1;
RET = [597921873_i32,(-52852583_i32),(-1356033062_i32),475954861_i32,(-811198666_i32),(-724145830_i32),(-763972619_i32),273771591_i32];
_5 = [(-3840_i16),(-8494_i16),29247_i16,27423_i16,(-11501_i16),(-26788_i16)];
_2 = _6;
_9.0 = _1 as f32;
_8 = _9.0 * _9.0;
_3 = 84458507_u32 >> _7.0;
_9.1 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_7.2 = 52184_u16 >> _1;
_7.2 = !33270_u16;
_10 = [1563645919_i32];
_5 = [(-21420_i16),(-25713_i16),24955_i16,(-20719_i16),(-15635_i16),(-16659_i16)];
_7.2 = 22169_u16;
_4 = 173432739504056757912597880927337326845_u128 ^ 165879396036857997206396492276518839797_u128;
_6 = [_3,_3,_3,_3,_3];
_7.1 = (-78764830946810632945937059288023126750_i128) as f64;
RET = [1569238116_i32,(-1019812916_i32),268218264_i32,1835267353_i32,(-1574436450_i32),1551624961_i32,(-9572321_i32),1154340122_i32];
_1 = _7.0 >> _3;
_2 = [_3,_3,_3,_3,_3];
_11 = 1280160925010610553_u64;
_12 = ['\u{46d59}','\u{4a2b}','\u{4091a}','\u{b3406}','\u{b4b81}'];
_13 = -(-32355_i16);
_6 = [_3,_3,_3,_3,_3];
Call(_4 = core::intrinsics::bswap(338903862907969551225894741851077434358_u128), bb3, UnwindUnreachable())
}
bb3 = {
_1 = -_7.0;
RET = [(-503333698_i32),1012679150_i32,(-1455888605_i32),(-937508629_i32),512347224_i32,(-884376188_i32),(-150230172_i32),331712251_i32];
_9.0 = _11 as f32;
RET = [429082373_i32,1288536192_i32,972036920_i32,(-993177002_i32),13750038_i32,1161395767_i32,(-1917158730_i32),1187341329_i32];
_7.2 = 76248943763116207855808593904888097107_i128 as u16;
_10 = [(-336360450_i32)];
Goto(bb4)
}
bb4 = {
_9.0 = -_8;
_1 = _7.0 | _7.0;
_8 = 7_usize as f32;
_9.1 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_7.2 = false as u16;
_10 = [123575155_i32];
_10 = [1420576909_i32];
Goto(bb5)
}
bb5 = {
_7.1 = _3 as f64;
_4 = '\u{100578}' as u128;
match _11 {
0 => bb4,
1 => bb2,
1280160925010610553 => bb6,
_ => bb3
}
}
bb6 = {
_15 = [1461967265_i32];
_12 = ['\u{df696}','\u{b07af}','\u{d4074}','\u{71fb3}','\u{3bc9}'];
_5 = [_13,_13,_13,_13,_13,_13];
_7.1 = 8212622207011269782_usize as f64;
RET = [973434385_i32,(-393523575_i32),1227894647_i32,1946783268_i32,(-634611652_i32),(-1144841234_i32),1417741635_i32,879524175_i32];
_1 = _7.0;
_10 = [(-1829184156_i32)];
_16 = [1448184755_i32,(-1089754402_i32),(-71786270_i32),(-1686306882_i32),(-715672419_i32),(-88695322_i32),1346460669_i32,1456123260_i32];
_10 = [679695803_i32];
_3 = !3431611746_u32;
_7.2 = 471818836804247425_usize as u16;
_9.1 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_9.0 = _11 as f32;
_1 = -_7.0;
_1 = -_7.0;
_19 = _7.2 & _7.2;
RET = [1142513484_i32,1096297086_i32,(-1963123169_i32),(-1400238175_i32),(-480035803_i32),(-891538177_i32),447144678_i32,(-610329689_i32)];
_13 = (-20407_i16);
_18 = 2117525110_i32 + (-2021818810_i32);
_7.2 = (-131164911032419957592049531179400853238_i128) as u16;
_1 = _7.0 + _7.0;
_6 = [_3,_3,_3,_3,_3];
_9.2 = _11 as i64;
_16 = [_18,_18,_18,_18,_18,_18,_18,_18];
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768191049 => bb8,
_ => bb7
}
}
bb7 = {
_6 = [_3,_3,_3,_3,_3];
Goto(bb2)
}
bb8 = {
match _13 {
0 => bb9,
340282366920938463463374607431768191049 => bb11,
_ => bb10
}
}
bb9 = {
_7.1 = _3 as f64;
_4 = '\u{100578}' as u128;
match _11 {
0 => bb4,
1 => bb2,
1280160925010610553 => bb6,
_ => bb3
}
}
bb10 = {
_1 = -_7.0;
RET = [(-503333698_i32),1012679150_i32,(-1455888605_i32),(-937508629_i32),512347224_i32,(-884376188_i32),(-150230172_i32),331712251_i32];
_9.0 = _11 as f32;
RET = [429082373_i32,1288536192_i32,972036920_i32,(-993177002_i32),13750038_i32,1161395767_i32,(-1917158730_i32),1187341329_i32];
_7.2 = 76248943763116207855808593904888097107_i128 as u16;
_10 = [(-336360450_i32)];
Goto(bb4)
}
bb11 = {
_12 = ['\u{105ed9}','\u{a1ed2}','\u{3a99a}','\u{4c65}','\u{3430f}'];
_16 = [_18,_18,_18,_18,_18,_18,_18,_18];
_3 = 19392466689538721196103798714781943705_i128 as u32;
_18 = (-325889499_i32) >> _1;
_4 = false as u128;
_1 = !_7.0;
_11 = 4102795470955925230_u64;
_1 = _7.0;
_21 = [_11,_11];
_11 = !5273234847211511930_u64;
_23.fld0.1 = _9.1;
_24 = !_3;
_13 = -20953_i16;
_13 = !30541_i16;
_23.fld1 = ['\u{c33c3}','\u{8a93f}','\u{6796b}','\u{8ec08}','\u{101ce4}','\u{9228e}'];
_22 = true as isize;
_7.0 = _1;
_8 = -_9.0;
_25 = 68_u8 as i8;
_18 = (-1335715026_i32);
_23.fld0.2 = _4 as i64;
match _18 {
0 => bb5,
1 => bb2,
340282366920938463463374607430432496430 => bb12,
_ => bb8
}
}
bb12 = {
_7.1 = 11043246553914293493_usize as f64;
Goto(bb13)
}
bb13 = {
_27 = core::ptr::addr_of!(_9.2);
_12 = ['\u{405b4}','\u{9a132}','\u{cac89}','\u{57b54}','\u{50492}'];
_7.2 = _19;
_22 = _7.0 ^ _7.0;
_26 = [_13,_13,_13,_13,_13,_13];
_9.0 = -_8;
_7.0 = _1 - _1;
_7.2 = !_19;
_23.fld0.1 = [_7.2,_19,_19,_19,_19,_19];
_2 = [_24,_24,_3,_24,_3];
_23.fld0.2 = _7.1 as i64;
_23.fld1 = ['\u{fef20}','\u{6faa4}','\u{21927}','\u{5dfa2}','\u{48abc}','\u{71270}'];
_29 = _22;
_9.0 = -_8;
_23.fld1 = ['\u{c6c03}','\u{11599}','\u{34354}','\u{78600}','\u{17d8f}','\u{23b68}'];
_7.0 = _22;
_9.2 = -_23.fld0.2;
_29 = _22;
(*_27) = !_23.fld0.2;
match _18 {
0 => bb8,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb14,
340282366920938463463374607430432496430 => bb16,
_ => bb15
}
}
bb14 = {
_7.1 = _3 as f64;
_4 = '\u{100578}' as u128;
match _11 {
0 => bb4,
1 => bb2,
1280160925010610553 => bb6,
_ => bb3
}
}
bb15 = {
_7.1 = _3 as f64;
_4 = '\u{100578}' as u128;
match _11 {
0 => bb4,
1 => bb2,
1280160925010610553 => bb6,
_ => bb3
}
}
bb16 = {
_31 = Adt55::Variant3 { fld0: _7.1 };
_13 = !25922_i16;
_32 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_31, 3), 0)));
_7 = (_1, (*_32), _19);
RET = [_18,_18,_18,_18,_18,_18,_18,_18];
_23.fld0 = (_9.0, _9.1, (*_27));
_6 = [_3,_3,_3,_24,_3];
_23.fld0.1 = _9.1;
_9.1 = [_19,_19,_19,_7.2,_7.2,_19];
_27 = core::ptr::addr_of!(_23.fld0.2);
RET = [_18,_18,_18,_18,_18,_18,_18,_18];
_13 = _4 as i16;
(*_32) = -_7.1;
_19 = !_7.2;
_8 = _9.0 - _23.fld0.0;
_3 = 140_u8 as u32;
_26 = _5;
_25 = -(-35_i8);
_7 = (_1, (*_32), _19);
place!(Field::<f64>(Variant(_31, 3), 0)) = (*_27) as f64;
_30 = _1;
_3 = !_24;
SetDiscriminant(_31, 3);
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(18_usize, 18_usize, Move(_18), 6_usize, Move(_6), 15_usize, Move(_15), 21_usize, Move(_21)), bb18, UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(18_usize, 12_usize, Move(_12), 22_usize, Move(_22), 25_usize, Move(_25), 19_usize, Move(_19)), bb19, UnwindUnreachable())
}
bb19 = {
Call(_37 = dump_var(18_usize, 11_usize, Move(_11), 26_usize, Move(_26), 38_usize, _38, 38_usize, _38), bb20, UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i128,mut _2: i128,mut _3: u128,mut _4: bool,mut _5: (i16,),mut _6: i16,mut _7: i32,mut _8: (*const i16, bool, [u64; 2], *const i64, u32)) -> [u16; 6] {
mir! {
type RET = [u16; 6];
let _9: char;
let _10: Adt58;
let _11: *const u64;
let _12: *const i128;
let _13: isize;
let _14: f64;
let _15: (i8, isize, u32, f64);
let _16: i64;
let _17: f32;
let _18: Adt49;
let _19: Adt61;
let _20: [i32; 8];
let _21: isize;
let _22: isize;
let _23: isize;
let _24: [u64; 2];
let _25: char;
let _26: [i32; 8];
let _27: [char; 5];
let _28: u64;
let _29: Adt64;
let _30: isize;
let _31: [u64; 2];
let _32: (i128, i128, i8);
let _33: ();
let _34: ();
{
_1 = _2 << _6;
Goto(bb1)
}
bb1 = {
_8.0 = core::ptr::addr_of!(_6);
RET = [1029_u16,50689_u16,40953_u16,9500_u16,61115_u16,42261_u16];
_1 = _7 as i128;
_5.0 = 5745160580417177846_i64 as i16;
_8.2 = [7426479134504424070_u64,10396312929290575174_u64];
_8.0 = core::ptr::addr_of!(_5.0);
_4 = _1 != _1;
_8.1 = _7 <= _7;
_6 = -_5.0;
_1 = !_2;
RET = [37598_u16,17240_u16,5740_u16,5207_u16,39870_u16,23440_u16];
Goto(bb2)
}
bb2 = {
_8.0 = core::ptr::addr_of!(_6);
_1 = _2 + _2;
RET = [41404_u16,40494_u16,51913_u16,33220_u16,10571_u16,28795_u16];
_7 = 428814014_i32;
_6 = _5.0 | _5.0;
RET = [41030_u16,34098_u16,34505_u16,56111_u16,18901_u16,18141_u16];
_5 = (_6,);
_6 = _7 as i16;
_4 = !_8.1;
_5.0 = _1 as i16;
RET = [36021_u16,11038_u16,45421_u16,62273_u16,5372_u16,24158_u16];
_8.2 = [3942215007657289511_u64,16457819605453319933_u64];
RET = [50001_u16,31799_u16,47527_u16,55828_u16,64798_u16,17314_u16];
_2 = -_1;
match _7 {
428814014 => bb3,
_ => bb1
}
}
bb3 = {
_8.1 = _3 == _3;
RET = [9693_u16,22576_u16,7599_u16,28532_u16,27171_u16,27683_u16];
_12 = core::ptr::addr_of!(_2);
_8.0 = core::ptr::addr_of!(_6);
_8.4 = 8566473753822969322_usize as u32;
_2 = _1 - _1;
_9 = '\u{3f1bd}';
RET = [58959_u16,24329_u16,8821_u16,37844_u16,29787_u16,13328_u16];
RET = [59320_u16,36853_u16,63701_u16,26063_u16,61302_u16,34364_u16];
_5 = (_6,);
(*_12) = _7 as i128;
_14 = 164_u8 as f64;
_13 = (-9223372036854775808_isize);
_5.0 = _8.4 as i16;
_8.2 = [10250786262111370152_u64,16671944044905684650_u64];
_13 = -9223372036854775807_isize;
_12 = core::ptr::addr_of!((*_12));
_13 = (-105_isize) >> _3;
_12 = core::ptr::addr_of!(_1);
RET = [50185_u16,25248_u16,1397_u16,6334_u16,28254_u16,46660_u16];
_8.4 = 700569983_u32;
_15.2 = _9 as u32;
(*_12) = _2 >> _3;
match _8.4 {
0 => bb4,
1 => bb5,
2 => bb6,
700569983 => bb8,
_ => bb7
}
}
bb4 = {
_8.0 = core::ptr::addr_of!(_6);
_1 = _2 + _2;
RET = [41404_u16,40494_u16,51913_u16,33220_u16,10571_u16,28795_u16];
_7 = 428814014_i32;
_6 = _5.0 | _5.0;
RET = [41030_u16,34098_u16,34505_u16,56111_u16,18901_u16,18141_u16];
_5 = (_6,);
_6 = _7 as i16;
_4 = !_8.1;
_5.0 = _1 as i16;
RET = [36021_u16,11038_u16,45421_u16,62273_u16,5372_u16,24158_u16];
_8.2 = [3942215007657289511_u64,16457819605453319933_u64];
RET = [50001_u16,31799_u16,47527_u16,55828_u16,64798_u16,17314_u16];
_2 = -_1;
match _7 {
428814014 => bb3,
_ => bb1
}
}
bb5 = {
_8.0 = core::ptr::addr_of!(_6);
RET = [1029_u16,50689_u16,40953_u16,9500_u16,61115_u16,42261_u16];
_1 = _7 as i128;
_5.0 = 5745160580417177846_i64 as i16;
_8.2 = [7426479134504424070_u64,10396312929290575174_u64];
_8.0 = core::ptr::addr_of!(_5.0);
_4 = _1 != _1;
_8.1 = _7 <= _7;
_6 = -_5.0;
_1 = !_2;
RET = [37598_u16,17240_u16,5740_u16,5207_u16,39870_u16,23440_u16];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_15.0 = -(-91_i8);
_12 = core::ptr::addr_of!(_2);
(*_12) = _13 as i128;
_8.3 = core::ptr::addr_of!(_16);
Goto(bb9)
}
bb9 = {
_13 = (-9223372036854775808_isize);
(*_12) = !_1;
_8.4 = _1 as u32;
_8.2 = [14157445696805726895_u64,6951845291791572855_u64];
(*_12) = _8.4 as i128;
_8.2 = [16313029583303817956_u64,12999586203616866074_u64];
_16 = 2511448709717559802_i64;
_12 = core::ptr::addr_of!(_1);
_8.4 = !_15.2;
_15.0 = _13 as i8;
_15.2 = _8.4;
RET = [22901_u16,48868_u16,12268_u16,4790_u16,43073_u16,54538_u16];
_16 = !(-5510654245579652231_i64);
_15.1 = _8.1 as isize;
_15 = ((-28_i8), _13, _8.4, _14);
_12 = core::ptr::addr_of!(_1);
_8.1 = _1 <= _2;
_19.fld0 = 40178_u16 ^ 13555_u16;
_14 = _15.3 + _15.3;
_7 = (-1048823686_i32) ^ (-2050756675_i32);
_15.0 = _19.fld0 as i8;
Goto(bb10)
}
bb10 = {
_9 = '\u{2a386}';
_20 = [_7,_7,_7,_7,_7,_7,_7,_7];
RET = [_19.fld0,_19.fld0,_19.fld0,_19.fld0,_19.fld0,_19.fld0];
_17 = _14 as f32;
_14 = _15.3;
_15.1 = _13;
_15.2 = _15.0 as u32;
_15.3 = _14 - _14;
_15 = ((-52_i8), _13, _8.4, _14);
_12 = core::ptr::addr_of!(_1);
Call((*_12) = core::intrinsics::transmute(_2), bb11, UnwindUnreachable())
}
bb11 = {
_21 = -_13;
_15 = ((-5_i8), _13, _8.4, _14);
_13 = -_21;
_7 = !(-247537304_i32);
_15.1 = -_13;
_22 = _13 * _21;
_5.0 = _6 * _6;
_17 = _15.0 as f32;
Call(_16 = core::intrinsics::bswap((-4546989806276646308_i64)), bb12, UnwindUnreachable())
}
bb12 = {
_3 = !138292070263598749335909898555724182681_u128;
_21 = _17 as isize;
_15.2 = 12740526521731712841_usize as u32;
(*_12) = !_2;
_15.1 = _22;
_7 = 886917784_i32;
_3 = !12939946965250847739755813097364477891_u128;
_6 = _8.1 as i16;
_24 = _8.2;
_15 = ((-90_i8), _21, _8.4, _14);
_19.fld0 = 43300_u16 ^ 42777_u16;
_14 = _15.3 - _15.3;
_25 = _9;
_1 = _2 ^ _2;
_16 = (-2047454256773202109_i64) << _1;
_15 = ((-87_i8), _22, _8.4, _14);
_13 = -_21;
_15.3 = _14;
_5.0 = _6 * _6;
_14 = _15.3;
_16 = (-6018957527484657630_i64) ^ 8837775837573654913_i64;
_8.0 = core::ptr::addr_of!(_6);
_23 = _13 * _15.1;
RET = [_19.fld0,_19.fld0,_19.fld0,_19.fld0,_19.fld0,_19.fld0];
_25 = _9;
_16 = -(-3682993323400292564_i64);
Goto(bb13)
}
bb13 = {
_15.1 = -_13;
_15.0 = (-78_i8);
_19.fld0 = 49897_u16;
_19.fld0 = _25 as u16;
_19.fld0 = 13477_u16;
(*_12) = _2 >> _2;
_15.1 = _21;
match _19.fld0 {
13477 => bb14,
_ => bb3
}
}
bb14 = {
_12 = core::ptr::addr_of!(_1);
_8.2 = [12731021862115724933_u64,758200132939114102_u64];
_32.0 = _2;
RET = [_19.fld0,_19.fld0,_19.fld0,_19.fld0,_19.fld0,_19.fld0];
_27 = [_25,_25,_9,_25,_25];
_17 = _3 as f32;
_14 = _15.3 - _15.3;
_30 = _21 >> _2;
_4 = _8.1;
_27 = [_25,_25,_9,_25,_9];
_14 = _16 as f64;
_19.fld0 = _17 as u16;
_31 = [7784075721966732614_u64,9629818527423871266_u64];
_9 = _25;
_9 = _25;
_32 = (_2, _2, _15.0);
_9 = _25;
_26 = _20;
_1 = _25 as i128;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(19_usize, 6_usize, Move(_6), 7_usize, Move(_7), 20_usize, Move(_20), 2_usize, Move(_2)), bb16, UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(19_usize, 13_usize, Move(_13), 30_usize, Move(_30), 5_usize, Move(_5), 24_usize, Move(_24)), bb17, UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(19_usize, 3_usize, Move(_3), 21_usize, Move(_21), 34_usize, _34, 34_usize, _34), bb18, UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(36692_u16), std::hint::black_box('\u{39d1f}'), std::hint::black_box(210_u8), std::hint::black_box(3454542938270097226_u64), std::hint::black_box((-32369_i16)), std::hint::black_box(2_usize), std::hint::black_box((-1884477611839756334_i64)), std::hint::black_box((-844916304297514546605079973490207339_i128)));
                
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt48 {
Variant0{
fld0: [isize; 3],
fld1: ([u64; 2], u128),
fld2: *const (i128, (i128, i128, i8)),
fld3: (i8, isize, u32, f64),

},
Variant1{
fld0: bool,
fld1: [i32; 8],
fld2: f32,
fld3: [char; 6],
fld4: *const *mut f64,

},
Variant2{
fld0: [char; 6],
fld1: [u64; 2],
fld2: i64,
fld3: *const i128,
fld4: [u8; 8],
fld5: i32,

},
Variant3{
fld0: *const i128,
fld1: i32,
fld2: [i32; 1],
fld3: [i16; 6],

}}
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: usize,
fld1: *mut u32,

},
Variant1{
fld0: bool,
fld1: (i128, i128, i8),
fld2: [i16; 6],
fld3: u128,
fld4: *const i64,
fld5: f32,
fld6: *mut f64,
fld7: *mut *const i128,

},
Variant2{
fld0: u128,
fld1: char,
fld2: (f32, [u16; 6], i64),
fld3: *const i16,
fld4: u16,
fld5: u64,
fld6: Adt48,
fld7: (*const i16, bool, [u64; 2], *const i64, u32),

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt50 {
fld0: (f32, [u16; 6], i64),
fld1: [char; 6],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt51 {
fld0: (*const i16, bool, [u64; 2], *const i64, u32),
fld1: Adt48,
fld2: (isize, f64, u16),
fld3: (char,),
fld4: *const u64,
fld5: i128,
fld6: usize,
}
#[derive(Debug)]
pub struct Adt52 {
fld0: *mut u32,
}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: bool,
fld1: (i128, i128, i8),
fld2: isize,
fld3: Adt50,
fld4: Adt49,
fld5: i32,
fld6: Adt48,

},
Variant1{
fld0: [isize; 8],
fld1: char,
fld2: Adt52,
fld3: (i16,),
fld4: (i128, (i128, i128, i8)),
fld5: i32,
fld6: i128,

}}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: *const i128,
fld1: Adt52,
fld2: isize,
fld3: Adt50,
fld4: u128,
fld5: u32,

},
Variant1{
fld0: *mut f64,
fld1: u64,
fld2: ([u64; 2], u128),
fld3: Adt48,

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: i64,
fld1: Adt51,
fld2: [u16; 6],
fld3: u64,
fld4: (i16,),

},
Variant1{
fld0: usize,
fld1: (*const i16, bool, [u64; 2], *const i64, u32),
fld2: isize,
fld3: i8,
fld4: u128,

},
Variant2{
fld0: Adt50,
fld1: Adt49,
fld2: [i16; 6],

},
Variant3{
fld0: f64,

}}
#[derive(Debug)]
pub struct Adt56 {
fld0: i16,
}
#[derive(Debug)]
pub enum Adt57 {
Variant0{
fld0: *mut *const i128,
fld1: u32,
fld2: Adt51,
fld3: i64,
fld4: u64,

},
Variant1{
fld0: [u32; 5],
fld1: [isize; 8],
fld2: [u8; 8],
fld3: i8,
fld4: Adt48,
fld5: *const i16,

},
Variant2{
fld0: Adt52,
fld1: *mut *const i128,
fld2: *const *mut f64,
fld3: (*const i16, bool, [u64; 2], *const i64, u32),
fld4: Adt49,
fld5: [char; 6],
fld6: [u64; 2],

}}
#[derive(Debug)]
pub enum Adt58 {
Variant0{
fld0: u128,
fld1: Adt51,
fld2: (*const i16, bool, [u64; 2], *const i64, u32),
fld3: Adt52,
fld4: [isize; 3],
fld5: Adt49,

},
Variant1{
fld0: Adt50,
fld1: u8,
fld2: *const u64,
fld3: (isize, f64, u16),

}}
#[derive(Debug)]
pub enum Adt59 {
Variant0{
fld0: u64,
fld1: *mut *const i128,

},
Variant1{
fld0: *mut f64,
fld1: i8,

}}
#[derive(Debug)]
pub enum Adt60 {
Variant0{
fld0: Adt48,
fld1: *const i16,

},
Variant1{
fld0: Adt54,
fld1: *mut f64,
fld2: Adt55,
fld3: (i128, (i128, i128, i8)),
fld4: [char; 5],
fld5: i32,
fld6: f32,
fld7: Adt56,

},
Variant2{
fld0: *const i64,
fld1: char,
fld2: [i32; 8],
fld3: (char,),
fld4: *const *mut f64,
fld5: (i128, (i128, i128, i8)),
fld6: [isize; 3],
fld7: (f64, [char; 6], u64, bool, f64),

}}
#[derive(Debug)]
pub struct Adt61 {
fld0: u16,
fld1: Adt57,
}
#[derive(Debug)]
pub enum Adt62 {
Variant0{
fld0: *const *mut f64,
fld1: (i16,),
fld2: Adt55,
fld3: u16,
fld4: Adt59,

},
Variant1{
fld0: (i128, i128, i8),
fld1: [u8; 8],

},
Variant2{
fld0: (i16,),

}}
#[derive(Debug)]
pub enum Adt63 {
Variant0{
fld0: [isize; 8],
fld1: Adt58,
fld2: *const i64,

},
Variant1{
fld0: Adt61,
fld1: f32,
fld2: u8,
fld3: Adt51,
fld4: i16,
fld5: Adt52,

}}
#[derive(Debug)]
pub enum Adt64 {
Variant0{
fld0: i128,
fld1: [isize; 8],
fld2: Adt57,
fld3: [u8; 8],
fld4: *mut *const i128,
fld5: Adt61,

},
Variant1{
fld0: u8,
fld1: char,
fld2: i128,

},
Variant2{
fld0: [i32; 1],
fld1: *const u64,
fld2: Adt62,
fld3: [u32; 5],
fld4: Adt48,
fld5: *const *mut f64,
fld6: i64,

}}

