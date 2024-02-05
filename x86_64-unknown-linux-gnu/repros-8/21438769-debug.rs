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
pub fn fn0(mut _1: usize,mut _2: char,mut _3: i128,mut _4: u32,mut _5: i16,mut _6: u64,mut _7: i64) -> Adt62 {
mir! {
type RET = Adt62;
let _8: (i8,);
let _9: ((bool, i32, bool), (char, i32), f32, u32, isize);
let _10: u128;
let _11: (char, i32);
let _12: *const u32;
let _13: [i8; 3];
let _14: u32;
let _15: Adt65;
let _16: u16;
let _17: (char, i32);
let _18: [i8; 3];
let _19: [i16; 8];
let _20: Adt58;
let _21: ();
let _22: ();
{
RET.fld1.fld1.1 = '\u{596f5}';
RET.fld1.fld1.2 = [(-25_i8),(-90_i8),(-85_i8)];
_4 = 3284986527_u32;
RET.fld1.fld0 = !15295_u16;
RET.fld1.fld0 = 63409_u16 / 59780_u16;
Goto(bb1)
}
bb1 = {
RET.fld1.fld1.1 = '\u{5cf09}';
_3 = 5979232292985118370_i64 as i128;
_8.0 = (-4231444734014745870_i64) as i8;
_6 = '\u{1d254}' as u64;
RET.fld1.fld2 = [9223372036854775807_isize,(-95_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-120_isize)];
_10 = (-9846_i16) as u128;
_8 = ((-11_i8),);
_5 = -28948_i16;
_1 = 14918927011752168193_usize / 13484182802782120190_usize;
_9.1.1 = (-943995421_i32) * 1360737230_i32;
_2 = '\u{bb93d}';
_9.0.1 = !_9.1.1;
match _8.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768211445 => bb8,
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
RET.fld1.fld0 = 45578_u16 + 22080_u16;
RET.fld1.fld2 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,58_isize,(-114_isize)];
_9.0.1 = _9.1.1;
RET.fld1.fld1.1 = _2;
_9.0.0 = false;
_9.1 = (_2, _9.0.1);
_9.1.0 = _2;
Call(_9 = fn1(_1, _2, _8.0, _8.0, _2, _3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_9.4 = _9.2 as isize;
RET.fld1.fld2 = [_9.4,_9.4,_9.4,_9.4,_9.4];
RET.fld1.fld0 = 59382_u16 | 20043_u16;
_9.0.0 = !_9.0.2;
_8 = ((-98_i8),);
_11.0 = _2;
_7 = !(-4112556193393371934_i64);
_7 = (-8218951747073238030_i64);
_9.0 = (false, _9.1.1, false);
_7 = _10 as i64;
_6 = _3 as u64;
_11.0 = _9.1.0;
_6 = _9.2 as u64;
_7 = 4119294736086206348_i64 ^ 4583367996886297784_i64;
_11.1 = -_9.1.1;
RET.fld1.fld2 = [_9.4,_9.4,_9.4,_9.4,_9.4];
_12 = core::ptr::addr_of!(_9.3);
_9.0 = (false, _9.1.1, true);
_9.0.0 = _9.0.2;
Call(RET.fld1 = fn2(_4, _4, (*_12), (*_12), (*_12), _8.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_11.1 = _10 as i32;
_3 = _9.0.2 as i128;
_11.0 = _2;
_12 = core::ptr::addr_of!(_4);
_9.1.0 = _2;
_8 = (112_i8,);
_9.1 = (_2, _11.1);
_8 = (122_i8,);
_7 = (-7859942652187584716_i64);
_8.0 = 76_i8;
(*_12) = _9.3 / 77618503_u32;
(*_12) = _9.3 % 3117627889_u32;
_9.0.2 = _9.0.0;
_9.4 = (-9223372036854775808_isize) | 8_isize;
_9.1 = (_11.0, _9.0.1);
_5 = _6 as i16;
_6 = 10872368490897701628_u64;
_14 = !(*_12);
_4 = !_9.3;
_11 = (_2, _9.0.1);
_13 = [_8.0,_8.0,_8.0];
_4 = _14;
RET.fld1.fld2 = [_9.4,_9.4,_9.4,_9.4,_9.4];
RET.fld1.fld0 = !19358_u16;
match _8.0 {
0 => bb8,
1 => bb2,
2 => bb6,
3 => bb9,
4 => bb5,
5 => bb11,
76 => bb13,
_ => bb12
}
}
bb11 = {
_9.4 = _9.2 as isize;
RET.fld1.fld2 = [_9.4,_9.4,_9.4,_9.4,_9.4];
RET.fld1.fld0 = 59382_u16 | 20043_u16;
_9.0.0 = !_9.0.2;
_8 = ((-98_i8),);
_11.0 = _2;
_7 = !(-4112556193393371934_i64);
_7 = (-8218951747073238030_i64);
_9.0 = (false, _9.1.1, false);
_7 = _10 as i64;
_6 = _3 as u64;
_11.0 = _9.1.0;
_6 = _9.2 as u64;
_7 = 4119294736086206348_i64 ^ 4583367996886297784_i64;
_11.1 = -_9.1.1;
RET.fld1.fld2 = [_9.4,_9.4,_9.4,_9.4,_9.4];
_12 = core::ptr::addr_of!(_9.3);
_9.0 = (false, _9.1.1, true);
_9.0.0 = _9.0.2;
Call(RET.fld1 = fn2(_4, _4, (*_12), (*_12), (*_12), _8.0), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
RET.fld1.fld0 = 45578_u16 + 22080_u16;
RET.fld1.fld2 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,58_isize,(-114_isize)];
_9.0.1 = _9.1.1;
RET.fld1.fld1.1 = _2;
_9.0.0 = false;
_9.1 = (_2, _9.0.1);
_9.1.0 = _2;
Call(_9 = fn1(_1, _2, _8.0, _8.0, _2, _3), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
RET.fld1.fld0 = _6 as u16;
_2 = _9.1.0;
_14 = !_9.3;
_14 = _5 as u32;
RET.fld1.fld1.1 = _2;
_9.1 = (_2, _9.0.1);
_9.1.0 = _2;
(*_12) = !_9.3;
RET.fld1.fld1.2 = [_8.0,_8.0,_8.0];
_13 = [_8.0,_8.0,_8.0];
_9.0 = (true, _11.1, false);
_9.4 = _3 as isize;
_3 = 128543250870198486844588754376046711492_i128 & 15615890423465170767837792771381218127_i128;
_11 = _9.1;
_15.fld3 = _8.0 * _8.0;
RET.fld1.fld0 = _2 as u16;
_7 = !(-517565821535432060_i64);
_16 = !46234_u16;
_9.0.2 = _9.0.0 & _9.0.0;
_15.fld2 = _8.0 as isize;
_5 = _7 as i16;
_9.4 = _9.0.2 as isize;
RET.fld1.fld1.2 = _13;
_17.0 = _11.0;
_8 = (_15.fld3,);
match _6 {
0 => bb8,
10872368490897701628 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
RET.fld1.fld0 = _3 as u16;
_2 = _17.0;
RET.fld1.fld2 = [_9.4,_9.4,_15.fld2,_9.4,_9.4];
_9.1.1 = _11.1;
_7 = -(-7913875557977320004_i64);
_11.0 = _2;
RET.fld1.fld1.1 = _11.0;
_6 = 263745629978423094_u64;
_15.fld0 = core::ptr::addr_of_mut!(_9.2);
_18 = _13;
_15.fld4 = [_4];
RET.fld0 = core::ptr::addr_of_mut!(_15.fld1);
_16 = 16436_u16 - 17402_u16;
_14 = 111_u8 as u32;
_17 = (_2, _11.1);
_6 = _5 as u64;
RET.fld1.fld2 = [_9.4,_9.4,_9.4,_9.4,_9.4];
_9.4 = _15.fld2 + _15.fld2;
_17.0 = _9.1.0;
RET.fld1.fld1.1 = _17.0;
RET.fld1.fld2 = [_9.4,_15.fld2,_9.4,_15.fld2,_9.4];
RET.fld1.fld0 = _16 & _16;
RET.fld0 = core::ptr::addr_of_mut!(_15.fld1);
RET.fld1.fld1.1 = _17.0;
Goto(bb16)
}
bb16 = {
Call(_21 = dump_var(0_usize, 14_usize, Move(_14), 18_usize, Move(_18), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_21 = dump_var(0_usize, 2_usize, Move(_2), 16_usize, Move(_16), 3_usize, Move(_3), 22_usize, _22), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: usize,mut _2: char,mut _3: i8,mut _4: i8,mut _5: char,mut _6: i128) -> ((bool, i32, bool), (char, i32), f32, u32, isize) {
mir! {
type RET = ((bool, i32, bool), (char, i32), f32, u32, isize);
let _7: f64;
let _8: Adt58;
let _9: ();
let _10: ();
{
RET.0.1 = 1727104797_i32 | 861658210_i32;
_4 = _3;
RET.0 = (true, 952759711_i32, false);
RET.1 = (_5, (-489231602_i32));
RET.2 = (-489576707_i32) as f32;
RET.2 = 26665_i16 as f32;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211445 => bb9,
_ => bb8
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
Return()
}
bb9 = {
_5 = _2;
RET.0 = (false, (-1556482006_i32), true);
_4 = -_3;
RET.1.0 = _5;
_6 = (-114724395522083610035536145420366461911_i128);
RET.4 = 9223372036854775807_isize - (-9223372036854775808_isize);
_3 = -_4;
RET.1 = (_5, 134226840_i32);
RET.1.1 = 56476_u16 as i32;
_3 = _4;
_8 = Adt58 { fld0: 16453307636569406624_u64 };
match _8.fld0 {
0 => bb5,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb11,
6 => bb12,
16453307636569406624 => bb14,
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
RET.3 = 1504388951_u32;
RET.1.0 = _5;
RET.0 = (true, 1366268814_i32, false);
RET.0.0 = !false;
_8.fld0 = 2300638439779043855_u64 % 13075521555788689660_u64;
RET.0.0 = !false;
Goto(bb15)
}
bb15 = {
Call(_9 = dump_var(1_usize, 3_usize, Move(_3), 4_usize, Move(_4), 1_usize, Move(_1), 10_usize, _10), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u32,mut _2: u32,mut _3: u32,mut _4: u32,mut _5: u32,mut _6: i8) -> Adt51 {
mir! {
type RET = Adt51;
let _7: isize;
let _8: Adt52;
let _9: f64;
let _10: [u32; 1];
let _11: Adt52;
let _12: f64;
let _13: Adt62;
let _14: (u8, char, [char; 2], (i64, u32, i128, i16), [char; 2]);
let _15: u8;
let _16: bool;
let _17: Adt60;
let _18: [u32; 6];
let _19: u16;
let _20: i32;
let _21: Adt52;
let _22: u8;
let _23: ();
let _24: ();
{
_8.fld0.1.1 = -716397704_i32;
RET.fld0 = 8126_u16 % 5650_u16;
_8.fld0.0.0 = _4 >= _3;
_9 = (-7764022820013033993_i64) as f64;
_1 = _2 ^ _2;
_1 = _5;
_8.fld0.3 = !_2;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
1504388951 => bb8,
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
_8.fld0.2 = 80404603305646995662737954468805923342_i128 as f32;
_8.fld0.1 = ('\u{ed21}', (-85835809_i32));
_8.fld0.1 = ('\u{64be2}', 339876485_i32);
_2 = _5;
_7 = -(-58_isize);
RET.fld1.1 = _8.fld0.1.0;
RET.fld2 = [_7,_7,_7,_7,_7];
RET.fld1.2 = [_6,_6,_6];
RET.fld2 = [_7,_7,_7,_7,_7];
_5 = 38000321604799955748324372603751122209_u128 as u32;
_8.fld1 = [_8.fld0.1.0,_8.fld0.1.0];
_8.fld1 = [_8.fld0.1.0,_8.fld0.1.0];
RET.fld0 = !55481_u16;
RET.fld0 = 22344_u16 % 32828_u16;
_8.fld0.1 = ('\u{7be2b}', 252139614_i32);
RET.fld0 = 57938_u16 << _8.fld0.3;
_5 = !_2;
Call(_7 = fn3(_8.fld0.1.0, _8.fld0.1.1, _5, _8.fld0.1.1, _4, _6, _8.fld0.2, _1, _1, _3, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET.fld1.1 = _8.fld0.1.0;
_8.fld1 = [_8.fld0.1.0,_8.fld0.1.0];
_11.fld0.0.0 = _8.fld0.0.0;
RET.fld0 = 19448_u16;
RET.fld2 = [_7,_7,_7,_7,_7];
_13.fld1.fld0 = 1952_u16 << _5;
_11.fld0.0.0 = _6 < _6;
_11.fld0.1.1 = !_8.fld0.1.1;
_11.fld0.0.2 = !_8.fld0.0.0;
_8.fld0.0.1 = _11.fld0.1.1 + _11.fld0.1.1;
_11.fld0.3 = _1 % 2464718789_u32;
_13.fld1.fld1.2 = [_6,_6,_6];
_8.fld0.3 = _11.fld0.3 ^ _4;
_11.fld0.1 = (_8.fld0.1.0, _8.fld0.1.1);
_4 = 92403610164553808305944174091559002029_u128 as u32;
RET.fld2 = [_7,_7,_7,_7,_7];
Call(_8.fld0.0.2 = fn4(_13.fld1.fld1.2, _7, _8.fld0.0.0, _7, _8.fld0.0.0, _8.fld0.1.0, _8.fld0.1.1, _8.fld0.1.0, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_11.fld0.4 = -_7;
_8.fld0.4 = !_7;
_6 = 42_i8;
_8.fld0.1 = _11.fld0.1;
_8.fld0.2 = (-2033872277548141427_i64) as f32;
_5 = _8.fld0.3 | _2;
_14.3.1 = _1;
_10 = [_1];
_2 = 28328937214281908235733920823635389531_i128 as u32;
_2 = !_4;
_13.fld1.fld2 = [_11.fld0.4,_7,_7,_7,_11.fld0.4];
_17.fld2.fld4.0.0 = _8.fld0.0.2;
_17.fld2.fld2 = 15175970539335976175_u64;
Goto(bb11)
}
bb11 = {
_8.fld0.1 = _11.fld0.1;
_17.fld2.fld4.2 = _8.fld0.2 * _8.fld0.2;
_17.fld6.0.0 = _8.fld0.1.0;
_17.fld1.1 = _8.fld0.1.0;
_14.3.1 = _3 | _5;
_11.fld1 = _8.fld1;
_11.fld0.0 = _8.fld0.0;
_14.3.2 = !36879000224527993742350379052957391401_i128;
_15 = 86_u8 * 168_u8;
_17.fld2.fld4.3 = !_14.3.1;
_14.0 = _15;
_17.fld6 = (_8.fld0.1, _8.fld0.4, _17.fld2.fld2, _17.fld2.fld4.0.0);
RET.fld1.2 = _13.fld1.fld1.2;
_14.2 = [_11.fld0.1.0,_8.fld0.1.0];
RET.fld1.1 = _8.fld0.1.0;
_13.fld1.fld0 = 51896_u16 / 3441_u16;
_14.0 = _15;
match _8.fld0.1.1 {
0 => bb1,
1 => bb12,
2 => bb13,
3 => bb14,
252139614 => bb16,
_ => bb15
}
}
bb12 = {
_11.fld0.4 = -_7;
_8.fld0.4 = !_7;
_6 = 42_i8;
_8.fld0.1 = _11.fld0.1;
_8.fld0.2 = (-2033872277548141427_i64) as f32;
_5 = _8.fld0.3 | _2;
_14.3.1 = _1;
_10 = [_1];
_2 = 28328937214281908235733920823635389531_i128 as u32;
_2 = !_4;
_13.fld1.fld2 = [_11.fld0.4,_7,_7,_7,_11.fld0.4];
_17.fld2.fld4.0.0 = _8.fld0.0.2;
_17.fld2.fld2 = 15175970539335976175_u64;
Goto(bb11)
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
_14.3 = (7250592510016627883_i64, _3, (-71750219381784638168396173027298215905_i128), (-13642_i16));
Goto(bb17)
}
bb17 = {
_17.fld3.fld0 = _8.fld0;
_17.fld2.fld4.0.2 = _17.fld2.fld4.0.0;
_17.fld5.2 = !_11.fld0.0.1;
_11.fld0.0.1 = _9 as i32;
_14.3 = ((-8359519033820970061_i64), _17.fld2.fld4.3, (-39965296737722470697684762193606692901_i128), (-29377_i16));
_11.fld0.3 = _17.fld3.fld0.3;
_17.fld6.3 = !_11.fld0.0.2;
_19 = _13.fld1.fld0 >> _11.fld0.3;
_19 = _13.fld1.fld0;
_13.fld1.fld2 = [_17.fld3.fld0.4,_7,_17.fld6.1,_7,_17.fld3.fld0.4];
_8.fld0.0 = (_17.fld2.fld4.0.2, _11.fld0.0.1, _11.fld0.0.2);
_17.fld3 = Adt52 { fld0: _8.fld0,fld1: _11.fld1 };
match _14.3.3 {
0 => bb2,
1 => bb18,
2 => bb19,
340282366920938463463374607431768182079 => bb21,
_ => bb20
}
}
bb18 = {
RET.fld1.1 = _8.fld0.1.0;
_8.fld1 = [_8.fld0.1.0,_8.fld0.1.0];
_11.fld0.0.0 = _8.fld0.0.0;
RET.fld0 = 19448_u16;
RET.fld2 = [_7,_7,_7,_7,_7];
_13.fld1.fld0 = 1952_u16 << _5;
_11.fld0.0.0 = _6 < _6;
_11.fld0.1.1 = !_8.fld0.1.1;
_11.fld0.0.2 = !_8.fld0.0.0;
_8.fld0.0.1 = _11.fld0.1.1 + _11.fld0.1.1;
_11.fld0.3 = _1 % 2464718789_u32;
_13.fld1.fld1.2 = [_6,_6,_6];
_8.fld0.3 = _11.fld0.3 ^ _4;
_11.fld0.1 = (_8.fld0.1.0, _8.fld0.1.1);
_4 = 92403610164553808305944174091559002029_u128 as u32;
RET.fld2 = [_7,_7,_7,_7,_7];
Call(_8.fld0.0.2 = fn4(_13.fld1.fld1.2, _7, _8.fld0.0.0, _7, _8.fld0.0.0, _8.fld0.1.0, _8.fld0.1.1, _8.fld0.1.0, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
RET.fld1.2 = [_6,_6,_6];
_17.fld2.fld4.2 = _5 as f32;
_11.fld0.0 = (_17.fld2.fld4.0.2, _11.fld0.1.1, _8.fld0.0.2);
_17.fld6.0 = (_8.fld0.1.0, _11.fld0.0.1);
_21.fld0.0.0 = _11.fld0.0.2 & _17.fld3.fld0.0.0;
_14.1 = _11.fld0.1.0;
_8.fld0.0.1 = !_8.fld0.1.1;
_17.fld2.fld3 = core::ptr::addr_of!(_8.fld0.3);
RET.fld1.0 = core::ptr::addr_of_mut!(_15);
_17.fld5.1 = !_13.fld1.fld0;
_17.fld4.fld1.1 = _11.fld0.1.0;
_14.0 = _15 ^ _15;
_12 = _9;
_8.fld0.0.0 = !_11.fld0.0.2;
_21.fld0.3 = !_5;
_16 = _17.fld2.fld4.0.0 & _11.fld0.0.0;
RET.fld1.0 = core::ptr::addr_of_mut!(_15);
_8.fld0.4 = _7;
_17.fld5.0 = _8.fld0.3 << _17.fld2.fld2;
Goto(bb22)
}
bb22 = {
Call(_23 = dump_var(2_usize, 4_usize, Move(_4), 15_usize, Move(_15), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_23 = dump_var(2_usize, 2_usize, Move(_2), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: char,mut _2: i32,mut _3: u32,mut _4: i32,mut _5: u32,mut _6: i8,mut _7: f32,mut _8: u32,mut _9: u32,mut _10: u32,mut _11: u32) -> isize {
mir! {
type RET = isize;
let _12: isize;
let _13: isize;
let _14: Adt54;
let _15: Adt53;
let _16: Adt60;
let _17: isize;
let _18: Adt57;
let _19: Adt58;
let _20: isize;
let _21: f32;
let _22: (char, i32);
let _23: u16;
let _24: [u32; 6];
let _25: Adt63;
let _26: u64;
let _27: bool;
let _28: char;
let _29: char;
let _30: i32;
let _31: f64;
let _32: f32;
let _33: isize;
let _34: [u16; 8];
let _35: [isize; 5];
let _36: Adt54;
let _37: [i8; 3];
let _38: ();
let _39: ();
{
_5 = !_3;
_3 = _8;
_7 = (-31390_i16) as f32;
_7 = (-125888426906741897864486561283580513096_i128) as f32;
Call(_2 = core::intrinsics::transmute(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _6 as isize;
_4 = _2 >> _11;
_5 = _10 >> _6;
_12 = (-9223372036854775808_isize) << _8;
_1 = '\u{5c9ac}';
_14.fld0 = _12 as f64;
_9 = _7 as u32;
_6 = 110_i8;
_16.fld2.fld0 = (_6,);
_1 = '\u{6773c}';
_16.fld2.fld4.0.1 = _4;
_16.fld2.fld3 = core::ptr::addr_of!(_16.fld3.fld0.3);
RET = !_12;
_16.fld4.fld0 = 49751_u16;
_16.fld6.2 = !2924047248727217408_u64;
Goto(bb2)
}
bb2 = {
_16.fld4.fld1.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_16.fld3.fld0.1.0 = _1;
_16.fld2.fld0 = (_6,);
_16.fld3.fld0.0 = (true, _4, true);
_15.fld1 = _14.fld0 + _14.fld0;
_16.fld1.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_15.fld0.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_16.fld1.1 = _16.fld3.fld0.1.0;
_18.fld0.2 = !_16.fld3.fld0.0.0;
_16.fld1.2 = [_6,_16.fld2.fld0.0,_16.fld2.fld0.0];
_16.fld6.0.1 = _16.fld2.fld4.0.1 * _16.fld3.fld0.0.1;
_16.fld4.fld1 = _16.fld1;
_16.fld6.0.0 = _1;
_15.fld0.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_16.fld2.fld4.1 = (_16.fld1.1, _4);
_15.fld2 = (_10, _16.fld4.fld0, _16.fld2.fld4.0.1);
_16.fld4.fld2 = [_12,_12,_12,_12,_12];
_16.fld3.fld0.1.0 = _16.fld2.fld4.1.0;
_16.fld3.fld1 = [_16.fld3.fld0.1.0,_16.fld2.fld4.1.0];
_2 = _16.fld6.0.1;
_15.fld2 = (_3, _16.fld4.fld0, _16.fld2.fld4.1.1);
_16.fld2.fld4.0.2 = _18.fld0.2;
_16.fld2.fld4.0.0 = _16.fld3.fld0.0.0 < _16.fld3.fld0.0.2;
_16.fld2.fld4.2 = 162_u8 as f32;
Goto(bb3)
}
bb3 = {
_16.fld4.fld0 = !_15.fld2.1;
_4 = _2 ^ _15.fld2.2;
RET = _12 ^ _12;
_16.fld3.fld0 = (_16.fld2.fld4.0, _16.fld6.0, _16.fld2.fld4.2, _9, _12);
_16.fld2.fld4.0.1 = _16.fld6.0.1;
_18 = Adt57 { fld0: _16.fld2.fld4.0 };
_17 = _16.fld3.fld0.4;
_1 = _16.fld1.1;
_6 = (-10382306324089927898613717704894105326_i128) as i8;
_16.fld3.fld0.4 = !_17;
_16.fld1.1 = _16.fld6.0.0;
_16.fld6 = (_16.fld2.fld4.1, _12, 3138825610771417696_u64, _16.fld2.fld4.0.0);
_18.fld0.1 = !_4;
_22.1 = _16.fld6.0.1 * _4;
_16.fld3.fld0.2 = _16.fld2.fld0.0 as f32;
_16.fld2.fld4.3 = _9;
_19.fld0 = 17966_i16 as u64;
_16.fld2.fld0.0 = _6;
match _16.fld6.2 {
0 => bb4,
1 => bb5,
3138825610771417696 => bb7,
_ => bb6
}
}
bb4 = {
_16.fld4.fld1.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_16.fld3.fld0.1.0 = _1;
_16.fld2.fld0 = (_6,);
_16.fld3.fld0.0 = (true, _4, true);
_15.fld1 = _14.fld0 + _14.fld0;
_16.fld1.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_15.fld0.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_16.fld1.1 = _16.fld3.fld0.1.0;
_18.fld0.2 = !_16.fld3.fld0.0.0;
_16.fld1.2 = [_6,_16.fld2.fld0.0,_16.fld2.fld0.0];
_16.fld6.0.1 = _16.fld2.fld4.0.1 * _16.fld3.fld0.0.1;
_16.fld4.fld1 = _16.fld1;
_16.fld6.0.0 = _1;
_15.fld0.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_16.fld2.fld4.1 = (_16.fld1.1, _4);
_15.fld2 = (_10, _16.fld4.fld0, _16.fld2.fld4.0.1);
_16.fld4.fld2 = [_12,_12,_12,_12,_12];
_16.fld3.fld0.1.0 = _16.fld2.fld4.1.0;
_16.fld3.fld1 = [_16.fld3.fld0.1.0,_16.fld2.fld4.1.0];
_2 = _16.fld6.0.1;
_15.fld2 = (_3, _16.fld4.fld0, _16.fld2.fld4.1.1);
_16.fld2.fld4.0.2 = _18.fld0.2;
_16.fld2.fld4.0.0 = _16.fld3.fld0.0.0 < _16.fld3.fld0.0.2;
_16.fld2.fld4.2 = 162_u8 as f32;
Goto(bb3)
}
bb5 = {
RET = _6 as isize;
_4 = _2 >> _11;
_5 = _10 >> _6;
_12 = (-9223372036854775808_isize) << _8;
_1 = '\u{5c9ac}';
_14.fld0 = _12 as f64;
_9 = _7 as u32;
_6 = 110_i8;
_16.fld2.fld0 = (_6,);
_1 = '\u{6773c}';
_16.fld2.fld4.0.1 = _4;
_16.fld2.fld3 = core::ptr::addr_of!(_16.fld3.fld0.3);
RET = !_12;
_16.fld4.fld0 = 49751_u16;
_16.fld6.2 = !2924047248727217408_u64;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_3 = _11;
_22.0 = _16.fld4.fld1.1;
_9 = !_15.fld2.0;
_18.fld0 = _16.fld2.fld4.0;
_15.fld1 = -_14.fld0;
_16.fld2.fld0 = (_6,);
_16.fld5.0 = _8;
_16.fld5.1 = _15.fld2.1;
_14 = Adt54 { fld0: _15.fld1 };
_16.fld4.fld1.2 = [_16.fld2.fld0.0,_16.fld2.fld0.0,_16.fld2.fld0.0];
_16.fld2.fld2 = _16.fld6.2;
_15.fld5 = [_16.fld4.fld0,_16.fld4.fld0,_16.fld4.fld0,_15.fld2.1,_16.fld4.fld0,_15.fld2.1,_16.fld5.1,_16.fld5.1];
_17 = -_16.fld6.1;
_9 = !_16.fld5.0;
_16.fld2.fld4.1.0 = _22.0;
match _16.fld6.2 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb8,
4 => bb9,
5 => bb10,
3138825610771417696 => bb12,
_ => bb11
}
}
bb8 = {
Return()
}
bb9 = {
RET = _6 as isize;
_4 = _2 >> _11;
_5 = _10 >> _6;
_12 = (-9223372036854775808_isize) << _8;
_1 = '\u{5c9ac}';
_14.fld0 = _12 as f64;
_9 = _7 as u32;
_6 = 110_i8;
_16.fld2.fld0 = (_6,);
_1 = '\u{6773c}';
_16.fld2.fld4.0.1 = _4;
_16.fld2.fld3 = core::ptr::addr_of!(_16.fld3.fld0.3);
RET = !_12;
_16.fld4.fld0 = 49751_u16;
_16.fld6.2 = !2924047248727217408_u64;
Goto(bb2)
}
bb10 = {
RET = _6 as isize;
_4 = _2 >> _11;
_5 = _10 >> _6;
_12 = (-9223372036854775808_isize) << _8;
_1 = '\u{5c9ac}';
_14.fld0 = _12 as f64;
_9 = _7 as u32;
_6 = 110_i8;
_16.fld2.fld0 = (_6,);
_1 = '\u{6773c}';
_16.fld2.fld4.0.1 = _4;
_16.fld2.fld3 = core::ptr::addr_of!(_16.fld3.fld0.3);
RET = !_12;
_16.fld4.fld0 = 49751_u16;
_16.fld6.2 = !2924047248727217408_u64;
Goto(bb2)
}
bb11 = {
_16.fld4.fld0 = !_15.fld2.1;
_4 = _2 ^ _15.fld2.2;
RET = _12 ^ _12;
_16.fld3.fld0 = (_16.fld2.fld4.0, _16.fld6.0, _16.fld2.fld4.2, _9, _12);
_16.fld2.fld4.0.1 = _16.fld6.0.1;
_18 = Adt57 { fld0: _16.fld2.fld4.0 };
_17 = _16.fld3.fld0.4;
_1 = _16.fld1.1;
_6 = (-10382306324089927898613717704894105326_i128) as i8;
_16.fld3.fld0.4 = !_17;
_16.fld1.1 = _16.fld6.0.0;
_16.fld6 = (_16.fld2.fld4.1, _12, 3138825610771417696_u64, _16.fld2.fld4.0.0);
_18.fld0.1 = !_4;
_22.1 = _16.fld6.0.1 * _4;
_16.fld3.fld0.2 = _16.fld2.fld0.0 as f32;
_16.fld2.fld4.3 = _9;
_19.fld0 = 17966_i16 as u64;
_16.fld2.fld0.0 = _6;
match _16.fld6.2 {
0 => bb4,
1 => bb5,
3138825610771417696 => bb7,
_ => bb6
}
}
bb12 = {
_25 = Adt63 { fld0: _16.fld1.2 };
_16.fld3.fld0.4 = _12;
_16.fld2.fld1 = _16.fld2.fld4.1.0;
_15.fld0.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_11 = _8;
_15.fld1 = (-106978908548516134456250067882466831966_i128) as f64;
_28 = _16.fld2.fld1;
_14 = Adt54 { fld0: _15.fld1 };
_16.fld4.fld2 = [_16.fld6.1,_16.fld6.1,_17,_12,_16.fld3.fld0.4];
_23 = _15.fld2.1;
_16.fld2.fld4.0 = _18.fld0;
_15.fld3 = _16.fld2.fld2;
_15.fld2.1 = 6197956034173773595_i64 as u16;
_16.fld4.fld1.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_27 = _16.fld6.2 < _15.fld3;
_15.fld2 = (_5, _23, _16.fld3.fld0.1.1);
_16.fld2.fld4 = (_18.fld0, _16.fld3.fld0.1, _7, _15.fld2.0, _16.fld6.1);
_16.fld3.fld0.0.1 = 243_u8 as i32;
_27 = !_18.fld0.0;
_27 = !_18.fld0.2;
_16.fld3.fld0 = (_16.fld2.fld4.0, _22, _7, _5, _12);
_16.fld2.fld4.2 = _7;
_29 = _16.fld4.fld1.1;
_16.fld2.fld4.0.1 = !_16.fld3.fld0.0.1;
_15.fld0.1 = _16.fld6.0.0;
Goto(bb13)
}
bb13 = {
_13 = -_17;
_18.fld0.0 = _16.fld3.fld0.4 == _16.fld2.fld4.4;
_16.fld2.fld4.4 = !_13;
_16.fld5.0 = 174_u8 as u32;
_16.fld2.fld2 = _15.fld3 % 16966507095716944613_u64;
_18.fld0 = (_16.fld2.fld4.0.0, _15.fld2.2, _16.fld2.fld4.0.0);
_5 = !_9;
_16.fld3.fld0.1.0 = _16.fld6.0.0;
_16.fld4.fld2 = [_16.fld3.fld0.4,_12,_12,_12,_16.fld2.fld4.4];
_11 = !_9;
_16.fld4.fld1 = _16.fld1;
_16.fld2.fld1 = _28;
_35 = [_17,_16.fld2.fld4.4,_12,_16.fld2.fld4.4,_16.fld2.fld4.4];
_23 = _16.fld4.fld0 / 2745_u16;
Goto(bb14)
}
bb14 = {
_16.fld2.fld4.2 = -_16.fld3.fld0.2;
_34 = [_23,_23,_23,_15.fld2.1,_23,_15.fld2.1,_23,_16.fld5.1];
_16.fld1.0 = core::ptr::addr_of_mut!(_16.fld2.fld5);
_15.fld0.1 = _29;
_35 = _16.fld4.fld2;
_9 = _16.fld3.fld0.3;
_23 = !_16.fld4.fld0;
_30 = _4 | _2;
_15.fld0.2 = _16.fld4.fld1.2;
_18.fld0.2 = _18.fld0.0;
_23 = !_15.fld2.1;
_16.fld5.2 = 92701167375670801358216417182053263021_u128 as i32;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(3_usize, 29_usize, Move(_29), 27_usize, Move(_27), 12_usize, Move(_12), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(3_usize, 23_usize, Move(_23), 8_usize, Move(_8), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(3_usize, 22_usize, Move(_22), 6_usize, Move(_6), 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [i8; 3],mut _2: isize,mut _3: bool,mut _4: isize,mut _5: bool,mut _6: char,mut _7: i32,mut _8: char,mut _9: u32) -> bool {
mir! {
type RET = bool;
let _10: f64;
let _11: ((char, i32), isize, u64, bool);
let _12: [i16; 8];
let _13: (bool, i32, bool);
let _14: [isize; 5];
let _15: [u32; 1];
let _16: isize;
let _17: Adt66;
let _18: (u32, u16, i32);
let _19: Adt56;
let _20: *mut u16;
let _21: ((bool, i32, bool), (char, i32), f32, u32, isize);
let _22: Adt63;
let _23: f32;
let _24: *const u32;
let _25: [char; 2];
let _26: i64;
let _27: *const *mut u32;
let _28: ((char, i32), *mut char, *mut u8, [char; 2], (char, i32));
let _29: (char, i32);
let _30: Adt50;
let _31: f64;
let _32: char;
let _33: f32;
let _34: isize;
let _35: i128;
let _36: isize;
let _37: char;
let _38: u32;
let _39: Adt51;
let _40: [isize; 5];
let _41: Adt59;
let _42: *mut u8;
let _43: f32;
let _44: ();
let _45: ();
{
_8 = _6;
RET = _4 > _4;
_1 = [(-35_i8),(-121_i8),(-13_i8)];
_8 = _6;
_2 = _9 as isize;
_5 = _3;
_8 = _6;
_8 = _6;
_4 = _2;
_1 = [104_i8,(-101_i8),(-77_i8)];
Call(_6 = fn5(_4, _4, _9, _1, _9, _4, _4, _4, _4, _9, _1, _8, _2, _7, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = [(-97_i8),(-104_i8),85_i8];
_1 = [80_i8,56_i8,(-93_i8)];
_7 = (-6_i8) as i32;
_8 = _6;
_1 = [(-125_i8),82_i8,(-90_i8)];
_10 = (-23677_i16) as f64;
_5 = _6 != _6;
_4 = _2;
_11.0.1 = -_7;
_1 = [37_i8,125_i8,(-1_i8)];
Goto(bb2)
}
bb2 = {
_3 = _5;
_11.0.0 = _8;
Call(_11.0.1 = core::intrinsics::bswap(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11.2 = _3 as u64;
_12 = [4413_i16,(-28814_i16),(-30428_i16),(-10848_i16),2594_i16,19591_i16,(-31872_i16),(-27234_i16)];
_11.1 = _2;
RET = _3;
RET = !_5;
_11.2 = !13449646708326961735_u64;
_6 = _8;
_10 = _2 as f64;
_11.3 = !_3;
_6 = _8;
Goto(bb4)
}
bb4 = {
_2 = _4 << _11.2;
_11.0 = (_6, _7);
_11.0 = (_8, _7);
_11.0.1 = _7;
_5 = _2 < _2;
RET = !_11.3;
_6 = _8;
_10 = _4 as f64;
_14 = [_2,_11.1,_4,_2,_11.1];
_17.fld5.0.0 = !_5;
_17.fld5.2 = _10 as f32;
_17.fld5.3 = _9 ^ _9;
_17.fld3 = (_11.0.0, _7);
_17.fld7.3 = [_17.fld3.0,_11.0.0];
_17.fld7.4 = (_11.0.0, _11.0.1);
_17.fld5.0.2 = _17.fld5.0.0;
_15 = [_17.fld5.3];
_17.fld7.1 = core::ptr::addr_of_mut!(_11.0.0);
_17.fld7.0 = (_8, _17.fld7.4.1);
_17.fld4 = core::ptr::addr_of_mut!(_17.fld5.0.0);
_17.fld4 = core::ptr::addr_of_mut!(_19.fld2.2);
_17.fld7.1 = core::ptr::addr_of_mut!(_17.fld7.0.0);
match _9 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
1504388951 => bb9,
_ => bb8
}
}
bb5 = {
_11.2 = _3 as u64;
_12 = [4413_i16,(-28814_i16),(-30428_i16),(-10848_i16),2594_i16,19591_i16,(-31872_i16),(-27234_i16)];
_11.1 = _2;
RET = _3;
RET = !_5;
_11.2 = !13449646708326961735_u64;
_6 = _8;
_10 = _2 as f64;
_11.3 = !_3;
_6 = _8;
Goto(bb4)
}
bb6 = {
_3 = _5;
_11.0.0 = _8;
Call(_11.0.1 = core::intrinsics::bswap(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_1 = [(-97_i8),(-104_i8),85_i8];
_1 = [80_i8,56_i8,(-93_i8)];
_7 = (-6_i8) as i32;
_8 = _6;
_1 = [(-125_i8),82_i8,(-90_i8)];
_10 = (-23677_i16) as f64;
_5 = _6 != _6;
_4 = _2;
_11.0.1 = -_7;
_1 = [37_i8,125_i8,(-1_i8)];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_19.fld2.1 = _17.fld7.4.1 ^ _17.fld7.0.1;
_19.fld0 = core::ptr::addr_of_mut!(_17.fld7.1);
_18.2 = -_17.fld7.0.1;
_17.fld5.4 = 35_i8 as isize;
_17.fld5.4 = _11.1 * _2;
_18 = (_17.fld5.3, 20608_u16, _19.fld2.1);
_19.fld2.0 = _15;
_17.fld7.0.1 = _7 ^ _7;
_17.fld5.1 = (_8, _11.0.1);
_17.fld3.0 = _8;
_19.fld1 = [(-26099_i16),1139_i16,(-18172_i16),(-10689_i16),(-19465_i16),(-7915_i16),(-24471_i16),(-21106_i16)];
_18.1 = 25489_u16 + 28105_u16;
_17.fld7.0.0 = _17.fld7.4.0;
_17.fld5.0.0 = !_17.fld5.0.2;
_4 = _2 + _2;
_21.0 = (_17.fld5.0.0, _17.fld7.0.1, _17.fld5.0.2);
_17.fld3.0 = _17.fld7.0.0;
_21.2 = -_17.fld5.2;
_13.2 = _21.0.2 & _11.3;
_19.fld0 = core::ptr::addr_of_mut!(_17.fld7.1);
_17.fld0 = Adt54 { fld0: _10 };
_21.4 = !_4;
_17.fld5.0.1 = -_18.2;
_19.fld2.1 = 135772044137814847250041239762266463724_u128 as i32;
_17.fld5.0.2 = !_5;
_21.0.0 = _21.0.2 & _21.0.2;
Call(_13.1 = core::intrinsics::transmute(_17.fld5.1.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17.fld5.4 = !_4;
_17.fld5.4 = -_2;
_19.fld2.2 = _17.fld5.0.0 > _21.0.0;
_17.fld7.1 = core::ptr::addr_of_mut!(_8);
_19.fld2 = (_15, _21.0.1, _17.fld5.0.2);
_19.fld2.0 = [_18.0];
_13.0 = !_21.0.0;
_11.3 = !_17.fld5.0.2;
_16 = _4;
_17.fld7.3 = [_6,_11.0.0];
_17.fld7.0.1 = _17.fld5.0.1 & _18.2;
_12 = [(-8067_i16),(-15621_i16),26989_i16,17780_i16,(-13637_i16),3583_i16,(-2725_i16),28158_i16];
_17.fld5.1.0 = _17.fld7.4.0;
_17.fld5.3 = _9 * _18.0;
_17.fld7.4.0 = _11.0.0;
_21.0.2 = !_13.0;
_17.fld5.3 = _18.0 ^ _18.0;
_20 = core::ptr::addr_of_mut!(_18.1);
_17.fld7.1 = core::ptr::addr_of_mut!(_17.fld7.0.0);
_17.fld5 = (_21.0, _11.0, _21.2, _18.0, _4);
RET = !_17.fld5.0.0;
_17.fld5.1.0 = _17.fld7.0.0;
_24 = core::ptr::addr_of!(_21.3);
_17.fld4 = core::ptr::addr_of_mut!(_5);
_23 = 137_u8 as f32;
_21 = _17.fld5;
_14 = [_2,_2,_21.4,_16,_16];
Goto(bb11)
}
bb11 = {
_17.fld0 = Adt54 { fld0: _10 };
_8 = _17.fld7.4.0;
_28.0.0 = _17.fld7.4.0;
_17.fld5.1.0 = _21.1.0;
_21.0.2 = _17.fld5.0.0;
_13 = (_21.0.0, _17.fld5.0.1, _21.0.2);
RET = _17.fld5.4 <= _16;
_30.fld4.1.1 = -_13.1;
_17.fld5.1.0 = _21.1.0;
_17.fld6 = core::ptr::addr_of_mut!(_28.2);
_30.fld4.0.2 = !_21.0.2;
_11.1 = _4;
_28.0 = (_6, _30.fld4.1.1);
_32 = _8;
_28.0.1 = _17.fld5.0.1 - _13.1;
_21.4 = !_17.fld5.4;
_11.1 = _4 + _21.4;
_2 = 19165_i16 as isize;
_17.fld5.0.0 = !_30.fld4.0.2;
_30.fld2 = _11.2;
_15 = [_9];
_17.fld6 = core::ptr::addr_of_mut!(_17.fld7.2);
Goto(bb12)
}
bb12 = {
_31 = _17.fld0.fld0;
(*_24) = 26_i8 as u32;
_17.fld5.0.1 = _28.0.1 << (*_20);
_14 = [_11.1,_11.1,_4,_4,_4];
_17.fld7.2 = core::ptr::addr_of_mut!(_30.fld5);
_35 = _30.fld4.0.2 as i128;
_21.0 = (_17.fld5.0.2, _17.fld3.1, _11.3);
(*_20) = 64876_u16 - 23911_u16;
_14 = [_16,_4,_11.1,_4,_16];
_17.fld5.0.1 = _21.0.1;
_33 = _17.fld5.2;
_21.0 = (_19.fld2.2, _17.fld3.1, _17.fld5.0.2);
_30.fld4.1.0 = _21.1.0;
_19.fld2.0 = _15;
_11.1 = !_21.4;
_35 = (-158027055911556401510905544379897110680_i128);
_24 = core::ptr::addr_of!(_38);
_30.fld3 = core::ptr::addr_of!(_30.fld4.3);
_2 = (-8967860919846692179_i64) as isize;
_30.fld4 = (_21.0, _17.fld7.0, _23, _18.0, _21.4);
_11.2 = _30.fld2 & _30.fld2;
_29.0 = _17.fld7.4.0;
_13.1 = _21.3 as i32;
match _9 {
0 => bb9,
1 => bb2,
2 => bb13,
3 => bb14,
4 => bb15,
1504388951 => bb17,
_ => bb16
}
}
bb13 = {
_1 = [(-97_i8),(-104_i8),85_i8];
_1 = [80_i8,56_i8,(-93_i8)];
_7 = (-6_i8) as i32;
_8 = _6;
_1 = [(-125_i8),82_i8,(-90_i8)];
_10 = (-23677_i16) as f64;
_5 = _6 != _6;
_4 = _2;
_11.0.1 = -_7;
_1 = [37_i8,125_i8,(-1_i8)];
Goto(bb2)
}
bb14 = {
_2 = _4 << _11.2;
_11.0 = (_6, _7);
_11.0 = (_8, _7);
_11.0.1 = _7;
_5 = _2 < _2;
RET = !_11.3;
_6 = _8;
_10 = _4 as f64;
_14 = [_2,_11.1,_4,_2,_11.1];
_17.fld5.0.0 = !_5;
_17.fld5.2 = _10 as f32;
_17.fld5.3 = _9 ^ _9;
_17.fld3 = (_11.0.0, _7);
_17.fld7.3 = [_17.fld3.0,_11.0.0];
_17.fld7.4 = (_11.0.0, _11.0.1);
_17.fld5.0.2 = _17.fld5.0.0;
_15 = [_17.fld5.3];
_17.fld7.1 = core::ptr::addr_of_mut!(_11.0.0);
_17.fld7.0 = (_8, _17.fld7.4.1);
_17.fld4 = core::ptr::addr_of_mut!(_17.fld5.0.0);
_17.fld4 = core::ptr::addr_of_mut!(_19.fld2.2);
_17.fld7.1 = core::ptr::addr_of_mut!(_17.fld7.0.0);
match _9 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
1504388951 => bb9,
_ => bb8
}
}
bb15 = {
_19.fld2.1 = _17.fld7.4.1 ^ _17.fld7.0.1;
_19.fld0 = core::ptr::addr_of_mut!(_17.fld7.1);
_18.2 = -_17.fld7.0.1;
_17.fld5.4 = 35_i8 as isize;
_17.fld5.4 = _11.1 * _2;
_18 = (_17.fld5.3, 20608_u16, _19.fld2.1);
_19.fld2.0 = _15;
_17.fld7.0.1 = _7 ^ _7;
_17.fld5.1 = (_8, _11.0.1);
_17.fld3.0 = _8;
_19.fld1 = [(-26099_i16),1139_i16,(-18172_i16),(-10689_i16),(-19465_i16),(-7915_i16),(-24471_i16),(-21106_i16)];
_18.1 = 25489_u16 + 28105_u16;
_17.fld7.0.0 = _17.fld7.4.0;
_17.fld5.0.0 = !_17.fld5.0.2;
_4 = _2 + _2;
_21.0 = (_17.fld5.0.0, _17.fld7.0.1, _17.fld5.0.2);
_17.fld3.0 = _17.fld7.0.0;
_21.2 = -_17.fld5.2;
_13.2 = _21.0.2 & _11.3;
_19.fld0 = core::ptr::addr_of_mut!(_17.fld7.1);
_17.fld0 = Adt54 { fld0: _10 };
_21.4 = !_4;
_17.fld5.0.1 = -_18.2;
_19.fld2.1 = 135772044137814847250041239762266463724_u128 as i32;
_17.fld5.0.2 = !_5;
_21.0.0 = _21.0.2 & _21.0.2;
Call(_13.1 = core::intrinsics::transmute(_17.fld5.1.1), ReturnTo(bb10), UnwindUnreachable())
}
bb16 = {
_3 = _5;
_11.0.0 = _8;
Call(_11.0.1 = core::intrinsics::bswap(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_10 = _17.fld0.fld0 - _17.fld0.fld0;
_35 = (-142683019625346261819127365251567213920_i128) & 82049223403705331186787047221092876147_i128;
_34 = -_30.fld4.4;
_41.fld1.0 = (_30.fld4.1.0, _19.fld2.1);
_39.fld2 = [_34,_11.1,_34,_16,_34];
_13 = (_5, _28.0.1, _30.fld4.0.0);
(*_24) = !_17.fld5.3;
_5 = _30.fld4.2 == _21.2;
_30.fld1 = _32;
_17.fld3 = (_30.fld1, _28.0.1);
_11.3 = !_30.fld4.0.2;
_26 = -8407629534511662976_i64;
_41.fld1.0.1 = _11.0.1;
_19.fld0 = core::ptr::addr_of_mut!(_17.fld7.1);
_37 = _17.fld5.1.0;
_41.fld2 = core::ptr::addr_of_mut!(_39.fld1.0);
_21.3 = !_30.fld4.3;
_42 = core::ptr::addr_of_mut!(_30.fld5);
_30.fld5 = !25_u8;
_28.1 = _17.fld7.1;
_13.2 = _30.fld4.0.2 | _3;
_41.fld1 = (_21.1, _28.1, _17.fld7.2, _17.fld7.3, _17.fld3);
_30.fld0.0 = 69_i8 >> _41.fld1.4.1;
_21.2 = -_17.fld5.2;
_41.fld1.4 = (_11.0.0, _13.1);
_25 = [_17.fld3.0,_17.fld5.1.0];
Goto(bb18)
}
bb18 = {
Call(_44 = dump_var(4_usize, 37_usize, Move(_37), 8_usize, Move(_8), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(4_usize, 32_usize, Move(_32), 35_usize, Move(_35), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(4_usize, 5_usize, Move(_5), 25_usize, Move(_25), 16_usize, Move(_16), 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: u32,mut _4: [i8; 3],mut _5: u32,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: u32,mut _11: [i8; 3],mut _12: char,mut _13: isize,mut _14: i32,mut _15: u32) -> char {
mir! {
type RET = char;
let _16: *const u32;
let _17: f64;
let _18: isize;
let _19: char;
let _20: (char, i32);
let _21: u32;
let _22: *mut ((char, i32), isize, u64, bool);
let _23: Adt62;
let _24: [char; 2];
let _25: i16;
let _26: isize;
let _27: isize;
let _28: Adt63;
let _29: char;
let _30: char;
let _31: [i16; 5];
let _32: (i64, u32, i128, i16);
let _33: bool;
let _34: *mut *mut u8;
let _35: (u8, char, [char; 2], (i64, u32, i128, i16), [char; 2]);
let _36: Adt55;
let _37: (i64, u32, i128, i16);
let _38: u8;
let _39: i128;
let _40: usize;
let _41: char;
let _42: i32;
let _43: f64;
let _44: bool;
let _45: ();
let _46: ();
{
_6 = _9 << _5;
_6 = _7 - _8;
RET = _12;
_8 = _7;
_8 = _9 << _6;
_10 = _3;
_13 = _7 >> _6;
_2 = _6;
_9 = -_8;
_3 = _10 % 1089518085_u32;
_9 = _2 + _2;
_4 = [(-100_i8),(-113_i8),(-127_i8)];
Call(_15 = core::intrinsics::bswap(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _13 + _13;
RET = _12;
_5 = _10 * _10;
_2 = _6;
_11 = [(-109_i8),87_i8,77_i8];
_7 = _13;
_15 = _5;
_10 = _3;
_11 = [90_i8,(-6_i8),98_i8];
_3 = _15 & _10;
_3 = !_5;
_12 = '\u{1dd53}';
_15 = !_5;
_16 = core::ptr::addr_of!(_10);
_3 = false as u32;
_15 = (*_16) & (*_16);
_18 = !_7;
_11 = [57_i8,122_i8,101_i8];
_10 = _3 - _15;
_16 = core::ptr::addr_of!(_5);
_7 = 6939630002874909435_u64 as isize;
_5 = !_10;
_5 = !_10;
_19 = _12;
_4 = [80_i8,10_i8,(-70_i8)];
_20 = (_12, _14);
Goto(bb2)
}
bb2 = {
_20 = (_12, _14);
_9 = _8 + _8;
_4 = [(-102_i8),(-92_i8),115_i8];
RET = _19;
Goto(bb3)
}
bb3 = {
_8 = !_13;
_2 = _9;
_5 = true as u32;
_17 = 75105940266766905190808859226283143157_u128 as f64;
_21 = _10 - _10;
_8 = _9 + _6;
_5 = _21 + _21;
_9 = 8183104256009348770_usize as isize;
_15 = (*_16);
_10 = 8_i8 as u32;
_1 = _8;
_10 = (*_16) ^ (*_16);
_14 = !_20.1;
_20 = (_19, _14);
_3 = (*_16);
_21 = _5 ^ _10;
_23.fld1.fld2 = [_1,_18,_8,_8,_13];
_10 = !(*_16);
_4 = _11;
Call(RET = fn6(_12, _16, _18, _19, _1, _19, _8, _7, _9, _20.1, _10, _6, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_23.fld1.fld1.2 = [101_i8,(-6_i8),(-124_i8)];
_14 = _20.1;
_16 = core::ptr::addr_of!(_10);
_25 = (-953_i16) << (*_16);
_23.fld0 = core::ptr::addr_of_mut!(_23.fld1.fld1.0);
_24 = [_19,_19];
_12 = _19;
_23.fld1.fld0 = !65273_u16;
RET = _19;
_20 = (_12, _14);
(*_16) = _3;
_8 = !_1;
_3 = !_21;
Goto(bb5)
}
bb5 = {
_28 = Adt63 { fld0: _11 };
_17 = 1_usize as f64;
Goto(bb6)
}
bb6 = {
_1 = _25 as isize;
_16 = core::ptr::addr_of!(_3);
_7 = !_1;
RET = _12;
_24 = [_12,_20.0];
_26 = _7;
(*_16) = (-122_i8) as u32;
(*_16) = _21;
_27 = 5469709887472339824_i64 as isize;
RET = _20.0;
_25 = !10160_i16;
RET = _12;
_28 = Adt63 { fld0: _4 };
_28 = Adt63 { fld0: _4 };
_26 = -_7;
_28.fld0 = _23.fld1.fld1.2;
_28 = Adt63 { fld0: _23.fld1.fld1.2 };
_26 = 1805837310423815563_u64 as isize;
Goto(bb7)
}
bb7 = {
_21 = !_10;
_20 = (_19, _14);
_23.fld1.fld0 = 5902_u16 | 41528_u16;
_28.fld0 = [(-69_i8),(-127_i8),114_i8];
_20 = (_19, _14);
_15 = !_3;
Call(_8 = core::intrinsics::bswap(_18), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17 = (*_16) as f64;
_28.fld0 = _11;
RET = _20.0;
_14 = _20.1;
_14 = -_20.1;
_19 = _12;
_20 = (_19, _14);
_7 = _1 << _3;
_4 = [33_i8,(-97_i8),(-17_i8)];
_8 = _7;
Goto(bb9)
}
bb9 = {
_5 = _21;
_20 = (_19, _14);
_13 = !_7;
_28.fld0 = _11;
_6 = !_7;
_17 = 151512958084433512374084201594353337798_i128 as f64;
RET = _12;
_32.3 = _25 + _25;
_9 = _7 ^ _6;
_32.0 = !(-5763740360646186477_i64);
_30 = _19;
RET = _20.0;
_32.2 = !137254864180318687000261583501540566890_i128;
_5 = _21 | _15;
_32.0 = 65_u8 as i64;
_33 = true;
Call(_27 = core::intrinsics::bswap(_13), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_26 = _7;
_35.3.1 = _3;
Goto(bb11)
}
bb11 = {
_36.fld1.fld0.0.1 = _20.1;
_36.fld3 = core::ptr::addr_of_mut!(_12);
_36.fld1.fld0.0 = (_33, _14, _33);
_36.fld1.fld0.0.2 = _36.fld1.fld0.0.0 ^ _36.fld1.fld0.0.0;
_36.fld0 = _15 == _21;
_36.fld1.fld0.4 = !_9;
_10 = !(*_16);
_35.2 = [_30,_19];
_23.fld1.fld1.1 = _12;
_37.3 = -_32.3;
_37.1 = (*_16) % 2415463878_u32;
_37 = (_32.0, _3, _32.2, _32.3);
_40 = 10561441622282399711_usize;
Goto(bb12)
}
bb12 = {
_36.fld1.fld0.1.0 = _20.0;
_35.2 = _24;
_35 = (59_u8, _23.fld1.fld1.1, _24, _37, _24);
_37 = (_32.0, _3, _32.2, _35.3.3);
_27 = _13;
_32 = (_35.3.0, _10, _35.3.2, _35.3.3);
_29 = _30;
_36.fld3 = core::ptr::addr_of_mut!(_12);
_3 = _32.2 as u32;
_32.2 = _37.2;
_35.3.2 = _37.2 >> _32.1;
_13 = _15 as isize;
_36.fld2 = 231246592499873110758512411186632022548_u128;
_34 = core::ptr::addr_of_mut!(_23.fld1.fld1.0);
Goto(bb13)
}
bb13 = {
_23.fld1.fld1.1 = _12;
_20 = (_29, _36.fld1.fld0.0.1);
_24 = _35.2;
_32.1 = _37.1 | _10;
_44 = _33;
_36.fld1.fld0.2 = _36.fld2 as f32;
_40 = 3_usize & 10204127297796618224_usize;
_36.fld1.fld1 = [_12,_35.1];
_31 = [_35.3.3,_35.3.3,_32.3,_25,_25];
_36.fld1.fld0.3 = !_15;
_42 = -_14;
_37.1 = _17 as u32;
_37.0 = _32.0 * _35.3.0;
_21 = _35.3.0 as u32;
_35.3.2 = _35.1 as i128;
_35.3 = (_32.0, _36.fld1.fld0.3, _37.2, _32.3);
_18 = _36.fld2 as isize;
_38 = _23.fld1.fld0 as u8;
_16 = core::ptr::addr_of!(_5);
_37 = (_35.3.0, _32.1, _32.2, _25);
(*_34) = core::ptr::addr_of_mut!(_35.0);
_42 = _36.fld1.fld0.0.1 * _36.fld1.fld0.0.1;
_37.1 = _35.3.1 / 519023867_u32;
_36.fld1.fld0.0.1 = !_42;
_20.0 = _35.1;
_36.fld1.fld0.0.0 = _32.1 < _32.1;
_24 = _36.fld1.fld1;
_35.3.3 = _32.3 + _37.3;
match _36.fld2 {
231246592499873110758512411186632022548 => bb15,
_ => bb14
}
}
bb14 = {
_17 = (*_16) as f64;
_28.fld0 = _11;
RET = _20.0;
_14 = _20.1;
_14 = -_20.1;
_19 = _12;
_20 = (_19, _14);
_7 = _1 << _3;
_4 = [33_i8,(-97_i8),(-17_i8)];
_8 = _7;
Goto(bb9)
}
bb15 = {
_36.fld0 = !_36.fld1.fld0.0.0;
_14 = !_36.fld1.fld0.0.1;
_5 = !_32.1;
_23.fld1.fld1.2 = [55_i8,(-126_i8),32_i8];
_26 = -_36.fld1.fld0.4;
_39 = _37.2;
_36.fld1.fld0.1 = (_19, _20.1);
(*_34) = core::ptr::addr_of_mut!(_35.0);
_35.3.2 = -_39;
_36.fld1.fld0.1 = (_19, _36.fld1.fld0.0.1);
_2 = _9 & _7;
Goto(bb16)
}
bb16 = {
Call(_45 = dump_var(5_usize, 11_usize, Move(_11), 14_usize, Move(_14), 13_usize, Move(_13), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(5_usize, 9_usize, Move(_9), 33_usize, Move(_33), 37_usize, Move(_37), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(5_usize, 42_usize, Move(_42), 24_usize, Move(_24), 32_usize, Move(_32), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(5_usize, 5_usize, Move(_5), 39_usize, Move(_39), 35_usize, Move(_35), 27_usize, Move(_27)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(5_usize, 19_usize, Move(_19), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: char,mut _2: *const u32,mut _3: isize,mut _4: char,mut _5: isize,mut _6: char,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: i32,mut _11: u32,mut _12: isize,mut _13: i32) -> char {
mir! {
type RET = char;
let _14: *mut *mut char;
let _15: u128;
let _16: Adt61;
let _17: f32;
let _18: Adt54;
let _19: [isize; 5];
let _20: Adt54;
let _21: *mut u8;
let _22: bool;
let _23: ((char, i32), isize, u64, bool);
let _24: (bool, i32, bool);
let _25: (char, i32);
let _26: isize;
let _27: [i16; 8];
let _28: bool;
let _29: isize;
let _30: Adt63;
let _31: i64;
let _32: u16;
let _33: i32;
let _34: (bool, i32, bool);
let _35: [u16; 8];
let _36: u8;
let _37: f64;
let _38: Adt59;
let _39: Adt59;
let _40: f64;
let _41: [char; 2];
let _42: f64;
let _43: ();
let _44: ();
{
_9 = (*_2) as isize;
RET = _1;
RET = _4;
Goto(bb1)
}
bb1 = {
_16.fld1.1 = 141_u8 as u32;
_16.fld4.fld2 = [_7,_9,_7,_3,_9];
_8 = _7;
_16.fld6.1 = _4;
_16.fld1.2 = (-85984417651982688847950437758230054395_i128) & (-66865913731129633351393468315420544003_i128);
_16.fld6.3.2 = -_16.fld1.2;
_16.fld1 = (4174544764958759188_i64, (*_2), _16.fld6.3.2, (-5512_i16));
(*_2) = !_11;
_16.fld1 = (8481388956692511642_i64, (*_2), _16.fld6.3.2, 23293_i16);
_16.fld1.3 = 9372_i16 - (-7055_i16);
(*_2) = 16128655952930512557_usize as u32;
_1 = _16.fld6.1;
RET = _4;
_17 = 75_i8 as f32;
_7 = _8;
_16.fld0 = !false;
_16.fld2 = core::ptr::addr_of_mut!(_16.fld4.fld1.0);
_16.fld3 = [_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3];
_16.fld6.3.0 = _16.fld1.0;
_16.fld4.fld1.1 = _6;
_3 = _8 >> _11;
_16.fld6.0 = 248_u8;
_16.fld5 = 6940_u16 as usize;
_8 = _9 & _9;
_3 = _5;
_16.fld3 = [_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3];
match _16.fld6.3.0 {
0 => bb2,
8481388956692511642 => bb4,
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
_16.fld1.2 = !_16.fld6.3.2;
_16.fld1 = (_16.fld6.3.0, _11, _16.fld6.3.2, (-23851_i16));
_7 = _5 | _12;
_20.fld0 = _16.fld1.3 as f64;
Call(_18.fld0 = core::intrinsics::fmaf64(_20.fld0, _20.fld0, _20.fld0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16.fld6.3 = (_16.fld1.0, _11, _16.fld1.2, _16.fld1.3);
_16.fld0 = _20.fld0 != _20.fld0;
_20.fld0 = (-3_i8) as f64;
_11 = _16.fld6.3.1;
_5 = _9 ^ _3;
_16.fld4.fld1.1 = _6;
_22 = _16.fld0;
_16.fld2 = core::ptr::addr_of_mut!(_16.fld4.fld1.0);
_16.fld1.1 = !_16.fld6.3.1;
_8 = _9 + _9;
(*_2) = !_11;
_16.fld2 = core::ptr::addr_of_mut!(_21);
_16.fld1.1 = _11;
_16.fld6.3.1 = _16.fld1.1 << _16.fld1.0;
_16.fld1 = (_16.fld6.3.0, (*_2), _16.fld6.3.2, _16.fld6.3.3);
_12 = -_5;
_23.1 = _3 - _9;
_25 = (_4, _10);
_13 = _25.1 & _10;
_7 = _16.fld6.3.2 as isize;
_1 = _6;
_16.fld4.fld1.1 = _16.fld6.1;
_24.2 = _16.fld0 & _22;
Goto(bb6)
}
bb6 = {
_5 = !_23.1;
_10 = -_25.1;
(*_2) = !_16.fld6.3.1;
_16.fld1.0 = _16.fld6.3.0;
_1 = _4;
_16.fld6.2 = [_6,_6];
_18.fld0 = _20.fld0 - _20.fld0;
RET = _6;
_21 = core::ptr::addr_of_mut!(_16.fld6.0);
_5 = !_23.1;
_19 = [_5,_5,_3,_9,_23.1];
_16.fld4.fld1.0 = core::ptr::addr_of_mut!((*_21));
match _16.fld6.3.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
340282366920938463463374607431768187605 => bb9,
_ => bb8
}
}
bb7 = {
_16.fld1.1 = 141_u8 as u32;
_16.fld4.fld2 = [_7,_9,_7,_3,_9];
_8 = _7;
_16.fld6.1 = _4;
_16.fld1.2 = (-85984417651982688847950437758230054395_i128) & (-66865913731129633351393468315420544003_i128);
_16.fld6.3.2 = -_16.fld1.2;
_16.fld1 = (4174544764958759188_i64, (*_2), _16.fld6.3.2, (-5512_i16));
(*_2) = !_11;
_16.fld1 = (8481388956692511642_i64, (*_2), _16.fld6.3.2, 23293_i16);
_16.fld1.3 = 9372_i16 - (-7055_i16);
(*_2) = 16128655952930512557_usize as u32;
_1 = _16.fld6.1;
RET = _4;
_17 = 75_i8 as f32;
_7 = _8;
_16.fld0 = !false;
_16.fld2 = core::ptr::addr_of_mut!(_16.fld4.fld1.0);
_16.fld3 = [_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3];
_16.fld6.3.0 = _16.fld1.0;
_16.fld4.fld1.1 = _6;
_3 = _8 >> _11;
_16.fld6.0 = 248_u8;
_16.fld5 = 6940_u16 as usize;
_8 = _9 & _9;
_3 = _5;
_16.fld3 = [_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3];
match _16.fld6.3.0 {
0 => bb2,
8481388956692511642 => bb4,
_ => bb3
}
}
bb8 = {
Return()
}
bb9 = {
_21 = core::ptr::addr_of_mut!((*_21));
_23.2 = 13509649657448427074_u64;
_16.fld6.3.3 = _16.fld5 as i16;
_2 = core::ptr::addr_of!((*_2));
_20.fld0 = _18.fld0;
Call(_16.fld4.fld0 = core::intrinsics::transmute(_16.fld1.3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_18 = Adt54 { fld0: _20.fld0 };
match _16.fld1.3 {
0 => bb8,
1 => bb5,
340282366920938463463374607431768187605 => bb12,
_ => bb11
}
}
bb11 = {
_21 = core::ptr::addr_of_mut!((*_21));
_23.2 = 13509649657448427074_u64;
_16.fld6.3.3 = _16.fld5 as i16;
_2 = core::ptr::addr_of!((*_2));
_20.fld0 = _18.fld0;
Call(_16.fld4.fld0 = core::intrinsics::transmute(_16.fld1.3), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_18.fld0 = -_20.fld0;
_25 = (_6, _10);
_24 = (_16.fld0, _13, _16.fld0);
_16.fld6.2 = [_25.0,_16.fld6.1];
_16.fld4.fld2 = [_8,_5,_23.1,_9,_8];
_18.fld0 = _23.2 as f64;
_25.0 = _6;
_8 = !_9;
_29 = _3;
_23.0.0 = _16.fld4.fld1.1;
_24.1 = _10 * _13;
_16.fld5 = 4644580714659068050_usize;
_27 = [_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3,_16.fld1.3];
_10 = _24.1;
_15 = !117173990010636163478043513451752759493_u128;
_16.fld0 = !_22;
_23.0.0 = _6;
_23.0.1 = _13;
_15 = 97585783616667027958650872306400134136_u128 % 147034086616911018892039194967000879220_u128;
_4 = _6;
_4 = _23.0.0;
_32 = _16.fld4.fld0;
_4 = _23.0.0;
_26 = _17 as isize;
Goto(bb13)
}
bb13 = {
_16.fld1 = (_16.fld6.3.0, (*_2), _16.fld6.3.2, _16.fld6.3.3);
_23.0.1 = -_10;
_15 = 170301004148331233946342736436387008676_u128 & 37264407207084009536100265449577554973_u128;
_31 = -_16.fld1.0;
(*_21) = 44_u8 | 194_u8;
_16.fld4.fld1.0 = core::ptr::addr_of_mut!(_16.fld6.0);
RET = _4;
_16.fld2 = core::ptr::addr_of_mut!(_21);
_16.fld1 = (_31, _16.fld6.3.1, _16.fld6.3.2, _16.fld6.3.3);
_30.fld0 = [58_i8,(-65_i8),(-20_i8)];
_1 = _16.fld6.1;
_13 = _10 >> _29;
_1 = _23.0.0;
_16.fld6.3.2 = (*_2) as i128;
_29 = _8 - _12;
_23.0.1 = -_25.1;
_25 = (_16.fld6.1, _24.1);
Call(_30.fld0 = fn7(_2, _18.fld0, _2, _23.0.0, _18.fld0, _24.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_34.0 = _24.0;
_7 = _16.fld4.fld0 as isize;
_34.1 = -_10;
_23 = (_25, _12, 810700609463951194_u64, _24.2);
_25 = (_4, _13);
_16.fld2 = core::ptr::addr_of_mut!(_16.fld4.fld1.0);
_34.2 = _24.2;
_38.fld1.0.0 = _6;
_16.fld6.3.3 = _16.fld0 as i16;
_16.fld1.1 = _16.fld6.0 as u32;
_38.fld0 = [_11,_16.fld6.3.1,_11,_11,_11,_16.fld6.3.1];
_38.fld1.1 = core::ptr::addr_of_mut!(_4);
_16.fld4.fld0 = _32;
_38.fld2 = core::ptr::addr_of_mut!(_16.fld4.fld1.0);
_39.fld1.3 = [_25.0,_16.fld4.fld1.1];
_18.fld0 = _20.fld0 - _20.fld0;
_16.fld6 = (50_u8, _1, _39.fld1.3, _16.fld1, _39.fld1.3);
_20 = Adt54 { fld0: _18.fld0 };
_39.fld1.2 = _16.fld4.fld1.0;
_38.fld1.4.0 = _16.fld4.fld1.1;
_39.fld1.0 = (_4, _25.1);
_38.fld1.1 = core::ptr::addr_of_mut!(_38.fld1.4.0);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(6_usize, 32_usize, Move(_32), 10_usize, Move(_10), 34_usize, Move(_34), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(6_usize, 25_usize, Move(_25), 22_usize, Move(_22), 5_usize, Move(_5), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(6_usize, 11_usize, Move(_11), 12_usize, Move(_12), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *const u32,mut _2: f64,mut _3: *const u32,mut _4: char,mut _5: f64,mut _6: i32) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _7: u8;
let _8: Adt66;
let _9: [i8; 3];
let _10: (([u32; 1], i32, bool),);
let _11: i16;
let _12: (bool, i32, bool);
let _13: [u8; 3];
let _14: ([u32; 1], i32, bool);
let _15: f64;
let _16: u64;
let _17: *mut char;
let _18: Adt58;
let _19: Adt50;
let _20: Adt58;
let _21: isize;
let _22: ();
let _23: ();
{
RET = [48_i8,(-26_i8),(-27_i8)];
_5 = _2;
(*_1) = 153548137572129944224107262300760382267_i128 as u32;
_6 = (-955655406_i32);
(*_3) = !2215765588_u32;
_2 = _5 - _5;
(*_1) = !1459096042_u32;
RET = [(-21_i8),(-72_i8),(-1_i8)];
_7 = 135_u8 / 136_u8;
(*_3) = !1748138784_u32;
(*_3) = !967328066_u32;
(*_1) = 64469_u16 as u32;
(*_1) = 3883012_u32 / 628238290_u32;
_2 = _5 + _5;
_8.fld0 = Adt54 { fld0: _5 };
_8.fld7.4.1 = _8.fld0.fld0 as i32;
(*_3) = !2176916872_u32;
_8.fld7.4.0 = _4;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607430812556050 => bb5,
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
_8.fld0.fld0 = -_2;
_8.fld7.0.1 = -_6;
_11 = _4 as i16;
_2 = 54000_u16 as f64;
_8.fld5.0.0 = false;
(*_3) = !3087893911_u32;
_8.fld5.1 = _8.fld7.4;
(*_3) = 9223372036854775807_isize as u32;
_11 = 16042476078326994159_u64 as i16;
_11 = 81_isize as i16;
_8.fld7.1 = core::ptr::addr_of_mut!(_8.fld3.0);
(*_3) = 3551405971_u32;
_9 = [(-90_i8),(-20_i8),(-91_i8)];
Goto(bb6)
}
bb6 = {
_8.fld2 = !10833270801368731710_u64;
_8.fld5.0.2 = _8.fld5.0.0;
_6 = _8.fld7.0.1 + _8.fld7.4.1;
_8.fld4 = core::ptr::addr_of_mut!(_12.2);
_8.fld0.fld0 = _6 as f64;
_7 = 59_u8 - 232_u8;
match (*_1) {
0 => bb1,
1 => bb7,
2 => bb8,
3 => bb9,
3551405971 => bb11,
_ => bb10
}
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
Return()
}
bb11 = {
_8.fld5.0 = (true, _6, false);
_10.0.2 = _8.fld5.0.0;
_8.fld3 = (_4, _6);
_7 = 98_u8;
_8.fld3 = (_8.fld7.4.0, _6);
(*_1) = !1458050191_u32;
_8.fld5.1 = (_4, _8.fld7.0.1);
RET = [70_i8,(-38_i8),(-79_i8)];
_7 = 174_u8;
Call(_14.2 = fn8(_8.fld4, _8.fld5.1.1, _10.0.2, _8.fld5.0, _10.0.2, _8.fld7.0.1, _8.fld7.1, (*_1), _4, _7, _1, _5, _5, _8.fld3, _8.fld3.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6 = _8.fld7.0.1;
_8.fld7.2 = core::ptr::addr_of_mut!(_7);
_8.fld0.fld0 = 2_usize as f64;
_8.fld1 = _9;
_10.0.1 = _8.fld5.0.1;
_8.fld7.0.0 = _4;
_6 = -_8.fld3.1;
_4 = _8.fld7.0.0;
_2 = _8.fld2 as f64;
_12 = (_8.fld5.0.2, _6, _8.fld5.0.0);
RET = [43_i8,(-35_i8),(-101_i8)];
_8.fld5.1.0 = _8.fld7.0.0;
_12.1 = -_6;
_8.fld3.1 = !_12.1;
_8.fld0.fld0 = -_5;
_8.fld5.1.1 = _6 << (*_3);
_8.fld7.2 = core::ptr::addr_of_mut!(_7);
_8.fld7.4.0 = _8.fld3.0;
(*_1) = !76363675_u32;
_8.fld7.2 = core::ptr::addr_of_mut!(_7);
_8.fld5.0 = _12;
_14.0 = [(*_1)];
_8.fld7.3 = [_8.fld7.0.0,_8.fld3.0];
_13 = [_7,_7,_7];
_18 = Adt58 { fld0: _8.fld2 };
_8.fld5.0.2 = _12.0;
_8.fld6 = core::ptr::addr_of_mut!(_8.fld7.2);
match _7 {
0 => bb6,
1 => bb10,
174 => bb13,
_ => bb3
}
}
bb13 = {
_8.fld5.1 = (_8.fld3.0, _6);
_8.fld7.0 = (_8.fld3.0, _8.fld5.0.1);
Call(_8.fld7.4.1 = fn11(_8.fld7.0, _8.fld5.0.1, _13), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_1) = 240836429128119047858771047707348182461_u128 as u32;
_8.fld0.fld0 = _11 as f64;
_8.fld5.4 = 9223372036854775807_isize;
_10.0.0 = _14.0;
_14.0 = _10.0.0;
_8.fld7.0 = (_8.fld5.1.0, _8.fld5.1.1);
_8.fld0 = Adt54 { fld0: _5 };
_12 = _8.fld5.0;
_1 = core::ptr::addr_of!(_8.fld5.3);
_8.fld3.0 = _8.fld7.4.0;
_8.fld1 = _9;
_14 = (_10.0.0, _8.fld5.1.1, _8.fld5.0.2);
_11 = 60_i8 as i16;
_19.fld4.1.1 = !_6;
_8.fld2 = !_18.fld0;
_8.fld7.0.0 = _8.fld5.1.0;
_20 = Adt58 { fld0: _8.fld2 };
(*_1) = !(*_3);
_19.fld0.0 = (-59_i8);
_10.0.2 = _8.fld5.0.0;
_16 = _8.fld2;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(7_usize, 9_usize, Move(_9), 4_usize, Move(_4), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(7_usize, 12_usize, Move(_12), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: *mut bool,mut _2: i32,mut _3: bool,mut _4: (bool, i32, bool),mut _5: bool,mut _6: i32,mut _7: *mut char,mut _8: u32,mut _9: char,mut _10: u8,mut _11: *const u32,mut _12: f64,mut _13: f64,mut _14: (char, i32),mut _15: char) -> bool {
mir! {
type RET = bool;
let _16: (u32, u16, i32);
let _17: isize;
let _18: f64;
let _19: *const [u32; 6];
let _20: u8;
let _21: *const [u32; 6];
let _22: bool;
let _23: isize;
let _24: i128;
let _25: [i16; 8];
let _26: i16;
let _27: Adt58;
let _28: Adt52;
let _29: ();
let _30: ();
{
RET = !_3;
(*_11) = 5704456777892595564_i64 as u32;
_16.1 = 52677_u16;
_16 = ((*_11), 48695_u16, _14.1);
_16.0 = (*_11) & (*_11);
(*_11) = _8;
_3 = _4.0;
(*_7) = _15;
_9 = (*_7);
(*_11) = !_16.0;
Goto(bb1)
}
bb1 = {
_3 = !_4.2;
(*_1) = (*_11) < (*_11);
_4 = (_3, _6, _3);
_16.0 = !(*_11);
_1 = core::ptr::addr_of_mut!(_4.2);
_18 = _12 / 0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001166030902107142_f64;
(*_11) = _16.0;
_4.2 = _16.1 < _16.1;
Goto(bb2)
}
bb2 = {
_17 = 4_usize as isize;
_14 = ((*_7), _6);
_16 = ((*_11), 35174_u16, _6);
(*_1) = !_5;
_7 = core::ptr::addr_of_mut!(_14.0);
_20 = _10 | _10;
_15 = _14.0;
(*_7) = _9;
match _16.1 {
0 => bb3,
1 => bb4,
2 => bb5,
35174 => bb7,
_ => bb6
}
}
bb3 = {
_3 = !_4.2;
(*_1) = (*_11) < (*_11);
_4 = (_3, _6, _3);
_16.0 = !(*_11);
_1 = core::ptr::addr_of_mut!(_4.2);
_18 = _12 / 0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001166030902107142_f64;
(*_11) = _16.0;
_4.2 = _16.1 < _16.1;
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
_4.0 = (*_1);
(*_11) = !_8;
_5 = _3 | _4.2;
_14.0 = _9;
_10 = _20 | _20;
Goto(bb8)
}
bb8 = {
_13 = 45_i8 as f64;
_6 = _16.0 as i32;
_14 = (_9, _2);
_16 = ((*_11), 56800_u16, _2);
_10 = !_20;
_8 = (*_11);
_13 = _12 / f64::NAN;
Call(_23 = fn9(_13, _10, _11, _10, _17, (*_7), _17, (*_1), _17, (*_7), _17, _11, _14), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_4.0 = (*_11) <= _16.0;
RET = (*_11) > (*_11);
_4.1 = -_16.2;
_15 = (*_7);
_25 = [(-17046_i16),4450_i16,26138_i16,(-1709_i16),22062_i16,(-30281_i16),4099_i16,15984_i16];
_18 = -_13;
_12 = _18 - _18;
_16 = ((*_11), 39468_u16, _6);
_5 = !_4.0;
_16.2 = _6 * _6;
_13 = _18 * _12;
_3 = !_4.0;
_28.fld0.0 = (_5, _16.2, _4.2);
_14 = (_15, _16.2);
_7 = core::ptr::addr_of_mut!(_15);
_27 = Adt58 { fld0: 3258175745549225811_u64 };
_28.fld0.1.0 = _14.0;
_28.fld0.2 = 17715197746169683446_usize as f32;
Goto(bb10)
}
bb10 = {
Call(_29 = dump_var(8_usize, 3_usize, Move(_3), 10_usize, Move(_10), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_29 = dump_var(8_usize, 15_usize, Move(_15), 8_usize, Move(_8), 16_usize, Move(_16), 30_usize, _30), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: f64,mut _2: u8,mut _3: *const u32,mut _4: u8,mut _5: isize,mut _6: char,mut _7: isize,mut _8: bool,mut _9: isize,mut _10: char,mut _11: isize,mut _12: *const u32,mut _13: (char, i32)) -> isize {
mir! {
type RET = isize;
let _14: Adt63;
let _15: isize;
let _16: *mut f32;
let _17: char;
let _18: isize;
let _19: [char; 2];
let _20: [isize; 5];
let _21: Adt59;
let _22: Adt61;
let _23: *mut bool;
let _24: [u32; 1];
let _25: isize;
let _26: isize;
let _27: *mut *mut char;
let _28: Adt66;
let _29: isize;
let _30: bool;
let _31: f32;
let _32: ((char, i32), *mut char, *mut u8, [char; 2], (char, i32));
let _33: i8;
let _34: (bool, i32, bool);
let _35: bool;
let _36: i128;
let _37: (char, i32);
let _38: [u32; 1];
let _39: i64;
let _40: Adt50;
let _41: ();
let _42: ();
{
_7 = _11;
_9 = 82611102715663745887080804492477210906_u128 as isize;
Goto(bb1)
}
bb1 = {
_3 = core::ptr::addr_of!((*_3));
RET = _13.0 as isize;
_11 = _7 * _9;
(*_12) = 2187735321848989637_i64 as u32;
_2 = _4 << _4;
_11 = -_9;
_13.1 = (-860702674_i32) ^ (-85506500_i32);
_5 = 6807524505908395414_u64 as isize;
_13 = (_6, (-1406960046_i32));
_14.fld0 = [(-42_i8),(-10_i8),(-95_i8)];
Goto(bb2)
}
bb2 = {
_3 = core::ptr::addr_of!((*_3));
(*_12) = !589468518_u32;
(*_3) = (-130635426072444564151409160352681293547_i128) as u32;
_9 = !_5;
_13.0 = _6;
Goto(bb3)
}
bb3 = {
(*_12) = 1509362781_u32 - 3403864460_u32;
_11 = _9 | _7;
_3 = core::ptr::addr_of!((*_3));
_1 = _2 as f64;
_13 = (_10, 1180454722_i32);
_1 = 23598_i16 as f64;
(*_3) = 1575301978_u32 % 55416245_u32;
_7 = _11;
_14.fld0 = [120_i8,(-47_i8),(-52_i8)];
_5 = 29_i8 as isize;
_13.0 = _6;
(*_3) = !43602994_u32;
_7 = _1 as isize;
_9 = _5;
_18 = (-3266_i16) as isize;
(*_3) = _1 as u32;
_17 = _6;
_17 = _13.0;
_20 = [_11,_9,_9,_11,_11];
_14.fld0 = [(-39_i8),(-94_i8),115_i8];
Goto(bb4)
}
bb4 = {
_22.fld6.1 = _13.0;
_1 = 61648967325824385846903966088381844812_i128 as f64;
_21.fld1.0.1 = 1_usize as i32;
(*_3) = 2216650271_u32;
_21.fld0 = [(*_3),(*_3),(*_3),(*_3),(*_3),(*_3)];
_20 = [_5,_11,_11,_5,_11];
_21.fld1.2 = core::ptr::addr_of_mut!(_2);
_22.fld4.fld1.1 = _10;
_7 = -_18;
_22.fld6.3.3 = 53767389154650768703551612615222789117_u128 as i16;
_22.fld6.3.1 = (*_12) * (*_3);
_17 = _22.fld6.1;
_22.fld2 = core::ptr::addr_of_mut!(_22.fld4.fld1.0);
_22.fld6.3.0 = (-171340129090092625_i64);
_23 = core::ptr::addr_of_mut!(_8);
_13 = (_17, _21.fld1.0.1);
_22.fld4.fld2 = [_18,_9,_11,_9,_18];
_21.fld0 = [(*_3),(*_3),_22.fld6.3.1,(*_3),(*_12),(*_3)];
_5 = _7 >> _9;
_9 = !_7;
_14.fld0 = [21_i8,(-36_i8),(-101_i8)];
_22.fld1.2 = -(-9277629012802584798123673274859731311_i128);
_21.fld1.0.1 = _13.1;
Call(_22.fld6.3.2 = fn10((*_23), _22.fld4.fld1.1, (*_3), _18, _10, _10, _5, _2, (*_3), _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_21.fld3 = _2;
_22.fld1.3 = -_22.fld6.3.3;
_10 = _6;
match (*_3) {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
4 => bb7,
2216650271 => bb9,
_ => bb8
}
}
bb6 = {
_22.fld6.1 = _13.0;
_1 = 61648967325824385846903966088381844812_i128 as f64;
_21.fld1.0.1 = 1_usize as i32;
(*_3) = 2216650271_u32;
_21.fld0 = [(*_3),(*_3),(*_3),(*_3),(*_3),(*_3)];
_20 = [_5,_11,_11,_5,_11];
_21.fld1.2 = core::ptr::addr_of_mut!(_2);
_22.fld4.fld1.1 = _10;
_7 = -_18;
_22.fld6.3.3 = 53767389154650768703551612615222789117_u128 as i16;
_22.fld6.3.1 = (*_12) * (*_3);
_17 = _22.fld6.1;
_22.fld2 = core::ptr::addr_of_mut!(_22.fld4.fld1.0);
_22.fld6.3.0 = (-171340129090092625_i64);
_23 = core::ptr::addr_of_mut!(_8);
_13 = (_17, _21.fld1.0.1);
_22.fld4.fld2 = [_18,_9,_11,_9,_18];
_21.fld0 = [(*_3),(*_3),_22.fld6.3.1,(*_3),(*_12),(*_3)];
_5 = _7 >> _9;
_9 = !_7;
_14.fld0 = [21_i8,(-36_i8),(-101_i8)];
_22.fld1.2 = -(-9277629012802584798123673274859731311_i128);
_21.fld1.0.1 = _13.1;
Call(_22.fld6.3.2 = fn10((*_23), _22.fld4.fld1.1, (*_3), _18, _10, _10, _5, _2, (*_3), _11), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
(*_12) = 1509362781_u32 - 3403864460_u32;
_11 = _9 | _7;
_3 = core::ptr::addr_of!((*_3));
_1 = _2 as f64;
_13 = (_10, 1180454722_i32);
_1 = 23598_i16 as f64;
(*_3) = 1575301978_u32 % 55416245_u32;
_7 = _11;
_14.fld0 = [120_i8,(-47_i8),(-52_i8)];
_5 = 29_i8 as isize;
_13.0 = _6;
(*_3) = !43602994_u32;
_7 = _1 as isize;
_9 = _5;
_18 = (-3266_i16) as isize;
(*_3) = _1 as u32;
_17 = _6;
_17 = _13.0;
_20 = [_11,_9,_9,_11,_11];
_14.fld0 = [(-39_i8),(-94_i8),115_i8];
Goto(bb4)
}
bb8 = {
_3 = core::ptr::addr_of!((*_3));
(*_12) = !589468518_u32;
(*_3) = (-130635426072444564151409160352681293547_i128) as u32;
_9 = !_5;
_13.0 = _6;
Goto(bb3)
}
bb9 = {
_22.fld5 = !2_usize;
_22.fld1.3 = !_22.fld6.3.3;
_22.fld6.2 = [_22.fld4.fld1.1,_6];
(*_23) = true;
_21.fld1.3 = _22.fld6.2;
(*_3) = _22.fld6.3.1 / 3268250907_u32;
_10 = _17;
_21.fld1.4.1 = !_13.1;
_22.fld4.fld1.2 = [(-53_i8),(-32_i8),(-112_i8)];
_24 = [(*_3)];
_22.fld1.2 = -_22.fld6.3.2;
(*_23) = !false;
_22.fld4.fld0 = 32332_u16 ^ 47385_u16;
_6 = _17;
_22.fld1 = (_22.fld6.3.0, (*_3), _22.fld6.3.2, _22.fld6.3.3);
_22.fld1 = (_22.fld6.3.0, (*_12), _22.fld6.3.2, _22.fld6.3.3);
_21.fld1.0.0 = _22.fld6.1;
_22.fld6 = (_2, _6, _21.fld1.3, _22.fld1, _21.fld1.3);
(*_12) = !_22.fld1.1;
_21.fld3 = !_4;
_22.fld6 = (_4, _13.0, _21.fld1.3, _22.fld1, _21.fld1.3);
_22.fld4.fld1.2 = _14.fld0;
match _22.fld6.3.0 {
0 => bb2,
340282366920938463463203267302678118831 => bb11,
_ => bb10
}
}
bb10 = {
_3 = core::ptr::addr_of!((*_3));
(*_12) = !589468518_u32;
(*_3) = (-130635426072444564151409160352681293547_i128) as u32;
_9 = !_5;
_13.0 = _6;
Goto(bb3)
}
bb11 = {
_19 = _21.fld1.3;
_26 = _5;
match _22.fld1.0 {
0 => bb10,
1 => bb8,
2 => bb6,
340282366920938463463203267302678118831 => bb12,
_ => bb4
}
}
bb12 = {
_22.fld3 = [_22.fld6.3.3,_22.fld6.3.3,_22.fld1.3,_22.fld1.3,_22.fld6.3.3,_22.fld6.3.3,_22.fld6.3.3,_22.fld1.3];
RET = !_7;
(*_12) = !_22.fld6.3.1;
_28.fld5.0.0 = _22.fld6.3.1 <= (*_12);
_28.fld0 = Adt54 { fld0: _1 };
_10 = _22.fld4.fld1.1;
_28.fld7.4.1 = _13.1;
_28.fld4 = core::ptr::addr_of_mut!((*_23));
_28.fld5.1 = _13;
_28.fld1 = [(-91_i8),(-17_i8),(-34_i8)];
_6 = _28.fld5.1.0;
match _22.fld6.3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb9,
340282366920938463463203267302678118831 => bb13,
_ => bb10
}
}
bb13 = {
_26 = _7;
_28.fld5.0.1 = !_13.1;
_28.fld2 = _21.fld1.4.1 as u64;
_32.0 = (_22.fld4.fld1.1, _21.fld1.0.1);
_28.fld7.0 = (_6, _21.fld1.0.1);
_21.fld1.1 = core::ptr::addr_of_mut!(_6);
_32 = (_28.fld5.1, _21.fld1.1, _21.fld1.2, _19, _13);
_31 = _22.fld4.fld0 as f32;
_17 = _32.0.0;
RET = -_9;
_18 = _11;
_22.fld6 = (_2, _10, _19, _22.fld1, _32.3);
_31 = _22.fld6.3.0 as f32;
_10 = _22.fld6.1;
_4 = _22.fld6.0 % 234_u8;
_12 = _3;
match _22.fld1.0 {
0 => bb9,
1 => bb12,
2 => bb5,
340282366920938463463203267302678118831 => bb14,
_ => bb4
}
}
bb14 = {
_28.fld7.1 = core::ptr::addr_of_mut!(_28.fld7.4.0);
_21.fld2 = core::ptr::addr_of_mut!(_32.2);
_32.1 = core::ptr::addr_of_mut!(_28.fld3.0);
_22.fld0 = !_28.fld5.0.0;
(*_3) = _22.fld6.3.1 | _22.fld1.1;
_32.2 = core::ptr::addr_of_mut!(_22.fld6.0);
_37.1 = _28.fld5.0.1 - _21.fld1.0.1;
_29 = _26;
_28.fld7.1 = core::ptr::addr_of_mut!(_13.0);
_28.fld2 = !4850908471870532322_u64;
_32.0.0 = _17;
_28.fld3 = _32.0;
_22.fld6.2 = [_10,_17];
_2 = _28.fld2 as u8;
_20 = _22.fld4.fld2;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(9_usize, 4_usize, Move(_4), 17_usize, Move(_17), 29_usize, Move(_29), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(9_usize, 5_usize, Move(_5), 9_usize, Move(_9), 10_usize, Move(_10), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: bool,mut _2: char,mut _3: u32,mut _4: isize,mut _5: char,mut _6: char,mut _7: isize,mut _8: u8,mut _9: u32,mut _10: isize) -> i128 {
mir! {
type RET = i128;
let _11: [i16; 5];
let _12: *mut u32;
let _13: [i16; 5];
let _14: f32;
let _15: ([u32; 1], i32, bool);
let _16: i32;
let _17: *mut f32;
let _18: Adt54;
let _19: isize;
let _20: ([u32; 1], i32, bool);
let _21: (bool, i32, bool);
let _22: *mut *mut u8;
let _23: isize;
let _24: ((char, i32), *mut char, *mut u8, [char; 2], (char, i32));
let _25: i16;
let _26: *mut u32;
let _27: [isize; 5];
let _28: *mut *mut u8;
let _29: bool;
let _30: f64;
let _31: isize;
let _32: isize;
let _33: f64;
let _34: f64;
let _35: ();
let _36: ();
{
_1 = !false;
_10 = _4;
RET = 19190531327372311800919017804881990277_i128;
_2 = _5;
_5 = _2;
RET = 157214408932402420095511755311888984500_i128 * 144082826926397027516993585195775716144_i128;
_11 = [(-5354_i16),21441_i16,(-23327_i16),(-19663_i16),32010_i16];
_6 = _5;
_1 = !true;
_9 = _3;
_4 = 56_i8 as isize;
_10 = 38_i8 as isize;
_8 = 7_usize as u8;
_2 = _5;
_8 = 210_u8;
_12 = core::ptr::addr_of_mut!(_3);
_12 = core::ptr::addr_of_mut!(_3);
_2 = _6;
_8 = 149_u8 / 240_u8;
_11 = [(-18912_i16),22171_i16,(-32567_i16),(-5767_i16),(-9242_i16)];
_13 = _11;
_4 = _10;
_9 = _4 as u32;
_13 = [31763_i16,(-10722_i16),(-24409_i16),21178_i16,26638_i16];
_4 = -_10;
_14 = (-17204_i16) as f32;
Goto(bb1)
}
bb1 = {
_15.2 = !_1;
_15.0 = [_3];
_13 = [(-11385_i16),27739_i16,(-6777_i16),31767_i16,(-26544_i16)];
_10 = _4;
_15.2 = !_1;
_10 = _7;
_15.1 = (-274985187_i32);
_2 = _5;
match (*_12) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2216650271 => bb9,
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
_1 = _15.2 == _15.2;
_2 = _6;
_14 = 61180_u16 as f32;
_18.fld0 = 9937_i16 as f64;
RET = _14 as i128;
_6 = _5;
_17 = core::ptr::addr_of_mut!(_14);
(*_17) = 4291752438905216644_usize as f32;
_8 = 143124006614077145007814610342457319966_u128 as u8;
_2 = _6;
_16 = _15.1;
(*_12) = _2 as u32;
_1 = _15.2;
_9 = !_3;
_20.2 = !_1;
RET = (-122475607835889999176946525651245841576_i128);
_2 = _5;
_7 = _4;
_20 = (_15.0, _15.1, _15.2);
(*_17) = 3_usize as f32;
_6 = _5;
match _16 {
0 => bb6,
1 => bb8,
2 => bb5,
3 => bb4,
4 => bb10,
340282366920938463463374607431493226269 => bb12,
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
_22 = core::ptr::addr_of_mut!(_24.2);
_15.0 = [(*_12)];
_20 = (_15.0, _15.1, _1);
_16 = 258868511754902916996617946704319598921_u128 as i32;
RET = 68943067229867119134744977912649052412_i128;
_15.0 = [(*_12)];
_12 = core::ptr::addr_of_mut!((*_12));
_6 = _2;
_6 = _5;
_18.fld0 = _3 as f64;
Goto(bb13)
}
bb13 = {
_24.2 = core::ptr::addr_of_mut!(_8);
_24.4.1 = !_15.1;
(*_17) = 36417713880977705773238766791526477876_i128 as f32;
(*_22) = core::ptr::addr_of_mut!(_8);
_24.1 = core::ptr::addr_of_mut!(_24.4.0);
_24.0 = (_2, _15.1);
_26 = core::ptr::addr_of_mut!((*_12));
_22 = core::ptr::addr_of_mut!(_24.2);
(*_12) = !_9;
_21.2 = !_20.2;
_4 = _10;
_25 = !(-30600_i16);
_18.fld0 = 182515476590452657490926790963888189625_u128 as f64;
_6 = _24.0.0;
_21 = (_15.2, _24.0.1, _1);
_23 = _7;
_8 = _25 as u8;
_21.1 = _20.1 << _24.0.1;
_22 = core::ptr::addr_of_mut!(_24.2);
_1 = _14 == (*_17);
RET = (-121213616467901075581378612725277664694_i128) ^ (-116735976067483484525856843293523904860_i128);
Goto(bb14)
}
bb14 = {
_1 = !_15.2;
(*_22) = core::ptr::addr_of_mut!(_8);
_24.0.1 = !_21.1;
(*_22) = core::ptr::addr_of_mut!(_8);
_15.0 = _20.0;
_26 = core::ptr::addr_of_mut!((*_26));
_24.4 = _24.0;
_12 = core::ptr::addr_of_mut!(_9);
_21 = (_1, _16, _1);
_5 = _6;
_14 = _10 as f32;
_27 = [_4,_4,_4,_10,_4];
_18.fld0 = _24.4.1 as f64;
_9 = !(*_26);
_4 = -_7;
_15.2 = _21.0;
_24.4 = _24.0;
_27 = [_7,_4,_10,_10,_7];
_15.1 = !_16;
_31 = _23 << (*_12);
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(10_usize, 4_usize, Move(_4), 21_usize, Move(_21), 20_usize, Move(_20), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(10_usize, 2_usize, Move(_2), 9_usize, Move(_9), 23_usize, Move(_23), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(10_usize, 25_usize, Move(_25), 3_usize, Move(_3), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: (char, i32),mut _2: i32,mut _3: [u8; 3]) -> i32 {
mir! {
type RET = i32;
let _4: u32;
let _5: *mut *mut char;
let _6: usize;
let _7: char;
let _8: [i16; 8];
let _9: isize;
let _10: f64;
let _11: char;
let _12: f32;
let _13: Adt58;
let _14: isize;
let _15: (i64, u32, i128, i16);
let _16: Adt59;
let _17: *mut u16;
let _18: isize;
let _19: Adt58;
let _20: u32;
let _21: [u32; 6];
let _22: (u8, char, [char; 2], (i64, u32, i128, i16), [char; 2]);
let _23: u8;
let _24: (u32, u16, i32);
let _25: u64;
let _26: f64;
let _27: *mut ((char, i32), isize, u64, bool);
let _28: f32;
let _29: (char, i32);
let _30: f64;
let _31: (bool, i32, bool);
let _32: f64;
let _33: isize;
let _34: Adt66;
let _35: Adt58;
let _36: *const *mut u32;
let _37: usize;
let _38: Adt50;
let _39: char;
let _40: bool;
let _41: Adt60;
let _42: u64;
let _43: char;
let _44: Adt57;
let _45: ();
let _46: ();
{
RET = 9223372036854775807_isize as i32;
_1 = ('\u{44323}', _2);
RET = _1.1;
_3 = [62_u8,114_u8,238_u8];
_1 = ('\u{d9c8b}', _2);
_1.0 = '\u{db9e4}';
_2 = _1.1 ^ _1.1;
Call(_2 = core::intrinsics::transmute(_1.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = ('\u{a4e07}', _2);
RET = !_2;
_1.1 = _2;
Call(_1 = fn12(_2, _2, _3, _2, _3, _2, _2, _3, _2, _2, _3, _2, _3, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.1 = _2;
RET = -_2;
RET = !_2;
_1 = ('\u{10aefa}', _2);
_1.0 = '\u{94d5a}';
Goto(bb3)
}
bb3 = {
_3 = [105_u8,25_u8,114_u8];
RET = _2 >> _1.1;
_3 = [159_u8,66_u8,119_u8];
_1.0 = '\u{d8ef2}';
RET = 3730413783_u32 as i32;
_2 = _1.1;
RET = 11674789196575130129356814040477179587_i128 as i32;
_1.0 = '\u{d6b6f}';
Goto(bb4)
}
bb4 = {
_3 = [113_u8,226_u8,239_u8];
RET = _2;
_7 = _1.0;
_1.1 = (-12006_i16) as i32;
_1 = (_7, _2);
Goto(bb5)
}
bb5 = {
_1.1 = !_2;
_8 = [(-8003_i16),(-25820_i16),23066_i16,25638_i16,16726_i16,16219_i16,32608_i16,3254_i16];
_1.0 = _7;
_3 = [216_u8,217_u8,139_u8];
_1.1 = (-26059_i16) as i32;
_1 = (_7, _2);
_2 = _1.1 - _1.1;
_6 = 3_usize + 4740716573782772693_usize;
_4 = 36_i8 as u32;
_3 = [159_u8,29_u8,4_u8];
_9 = 9223372036854775807_isize;
_9 = 9223372036854775807_isize << _6;
Goto(bb6)
}
bb6 = {
_6 = 32285_u16 as usize;
RET = -_2;
_8 = [(-6863_i16),(-17098_i16),22010_i16,(-20729_i16),26197_i16,6029_i16,(-30516_i16),11871_i16];
RET = _1.1;
_10 = _9 as f64;
Goto(bb7)
}
bb7 = {
_2 = _1.1 + _1.1;
_7 = _1.0;
_10 = 44925_u16 as f64;
_8 = [26618_i16,(-14559_i16),7588_i16,(-15041_i16),(-31807_i16),13941_i16,(-23792_i16),(-1701_i16)];
_13.fld0 = 1159809172878033699_u64 * 18018140300442173165_u64;
_13 = Adt58 { fld0: 2923277192721349507_u64 };
_6 = 0_usize;
_7 = _1.0;
_16.fld1.0.1 = -_2;
Goto(bb8)
}
bb8 = {
_1 = (_7, _2);
_8 = [(-17914_i16),25184_i16,14686_i16,(-17978_i16),(-32388_i16),(-28995_i16),12096_i16,26660_i16];
_15 = (2557084004671838838_i64, _4, (-108287237118027646857582482547136179926_i128), _8[_6]);
_16.fld2 = core::ptr::addr_of_mut!(_16.fld1.2);
RET = _2 | _2;
_15.1 = _4;
_16.fld3 = _3[_6];
_7 = _1.0;
_16.fld3 = _3[_6] * _3[_6];
_10 = _6 as f64;
_12 = _3[_6] as f32;
_16.fld1.2 = core::ptr::addr_of_mut!(_3[_6]);
_8 = [_15.3,_15.3,_15.3,_15.3,_15.3,_15.3,_15.3,_15.3];
_5 = core::ptr::addr_of_mut!(_16.fld1.1);
_12 = 55317_u16 as f32;
_16.fld3 = !_3[_6];
_16.fld1.0 = (_1.0, _1.1);
_15.0 = (-4805305783197019290_i64);
_15.0 = (-448569058523908176_i64);
_12 = _16.fld3 as f32;
_16.fld1.3[_6] = _7;
_13.fld0 = 10687387295097390714_u64;
(*_5) = core::ptr::addr_of_mut!(_16.fld1.3[_6]);
_12 = _16.fld1.0.1 as f32;
_16.fld1.4 = (_16.fld1.3[_6], _16.fld1.0.1);
(*_5) = core::ptr::addr_of_mut!(_7);
match _8[_6] {
340282366920938463463374607431768193542 => bb10,
_ => bb9
}
}
bb9 = {
_1.1 = !_2;
_8 = [(-8003_i16),(-25820_i16),23066_i16,25638_i16,16726_i16,16219_i16,32608_i16,3254_i16];
_1.0 = _7;
_3 = [216_u8,217_u8,139_u8];
_1.1 = (-26059_i16) as i32;
_1 = (_7, _2);
_2 = _1.1 - _1.1;
_6 = 3_usize + 4740716573782772693_usize;
_4 = 36_i8 as u32;
_3 = [159_u8,29_u8,4_u8];
_9 = 9223372036854775807_isize;
_9 = 9223372036854775807_isize << _6;
Goto(bb6)
}
bb10 = {
_16.fld0[_6] = _15.1 % 1193561629_u32;
_7 = _16.fld1.0.0;
_19.fld0 = _15.3 as u64;
_10 = _15.3 as f64;
_16.fld1.0.0 = _1.0;
_15.3 = _8[_6];
_22.4 = [_7,_16.fld1.4.0];
_23 = 96_i8 as u8;
_24.2 = _8[_6] as i32;
_16.fld0[_6] = _19.fld0 as u32;
_17 = core::ptr::addr_of_mut!(_24.1);
_15.0 = _12 as i64;
_3[_6] = _23 & _23;
_16.fld1.0.1 = _24.2;
_22.0 = !_3[_6];
_19 = Adt58 { fld0: _13.fld0 };
_22.3 = _15;
_16.fld1.4 = (_16.fld1.3[_6], _24.2);
_3[_6] = _16.fld3;
_24 = (_4, 47920_u16, _2);
_11 = _16.fld1.0.0;
_7 = _16.fld1.4.0;
_19.fld0 = _16.fld0[_6] as u64;
Goto(bb11)
}
bb11 = {
_2 = _22.0 as i32;
_16.fld1.3 = [_7,_16.fld1.4.0];
_15.1 = _16.fld0[_6] * _16.fld0[_6];
_16.fld1.0 = _1;
_15 = (_22.3.0, _16.fld0[_6], _22.3.2, _8[_6]);
_11 = _16.fld1.3[_6];
_22 = (_23, _16.fld1.0.0, _16.fld1.3, _15, _16.fld1.3);
(*_5) = core::ptr::addr_of_mut!(_11);
_16.fld1.4.0 = _22.1;
_22 = (_16.fld3, _16.fld1.3[_6], _16.fld1.3, _15, _16.fld1.3);
_8[_6] = _22.3.3 & _15.3;
_16.fld1.4 = (_1.0, _2);
_29 = (_1.0, _2);
RET = !_2;
_22.1 = _29.0;
_24.0 = _22.3.1 % 989157707_u32;
_18 = _9;
_9 = _18;
_22.2[_6] = _16.fld1.3[_6];
_26 = _6 as f64;
_28 = _12;
_14 = _18;
_31 = (false, _1.1, true);
_12 = _28 - _28;
Goto(bb12)
}
bb12 = {
_22.3 = _15;
_30 = _22.3.3 as f64;
_2 = _13.fld0 as i32;
_24.2 = _11 as i32;
_16.fld1.4.1 = 35_i8 as i32;
_34.fld5.1 = _16.fld1.0;
Goto(bb13)
}
bb13 = {
_22.2 = _22.4;
_34.fld7 = (_1, _16.fld1.1, _16.fld1.2, _16.fld1.3, _29);
_32 = _10;
_3 = [_16.fld3,_16.fld3,_22.0];
_34.fld1[_6] = !(-70_i8);
_29.1 = !_34.fld5.1.1;
_34.fld5 = (_31, _29, _28, _15.1, _9);
_34.fld5.3 = _15.1 + _15.1;
_34.fld1[_6] = -(-8_i8);
_34.fld3.0 = _29.0;
_38.fld4.1.0 = _16.fld1.3[_6];
_38.fld4.2 = -_12;
_34.fld7.0.0 = _16.fld1.0.0;
_34.fld1[_6] = !(-76_i8);
_41.fld3.fld0.1 = (_16.fld1.4.0, _16.fld1.0.1);
_16.fld3 = _16.fld1.4.1 as u8;
_34.fld7.0 = (_16.fld1.0.0, _34.fld5.0.1);
_34.fld6 = core::ptr::addr_of_mut!(_41.fld4.fld1.0);
_41.fld3.fld0.4 = _9 & _9;
Call(_3 = fn17(_34.fld6, _16.fld1.0.0, _31, _2, _34.fld5.2, _26), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_38.fld0.0 = 32_i8;
_16.fld1.0.1 = _41.fld3.fld0.1.1 | _29.1;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(11_usize, 6_usize, Move(_6), 29_usize, Move(_29), 7_usize, Move(_7), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(11_usize, 15_usize, Move(_15), 4_usize, Move(_4), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i32,mut _2: i32,mut _3: [u8; 3],mut _4: i32,mut _5: [u8; 3],mut _6: i32,mut _7: i32,mut _8: [u8; 3],mut _9: i32,mut _10: i32,mut _11: [u8; 3],mut _12: i32,mut _13: [u8; 3],mut _14: i32) -> (char, i32) {
mir! {
type RET = (char, i32);
let _15: Adt58;
let _16: [u16; 8];
let _17: *mut ((char, i32), isize, u64, bool);
let _18: (i64, u32, i128, i16);
let _19: u16;
let _20: char;
let _21: f32;
let _22: *mut bool;
let _23: (bool, i32, bool);
let _24: *mut char;
let _25: i128;
let _26: char;
let _27: i128;
let _28: *const *mut u32;
let _29: u8;
let _30: (*mut u8, char, [i8; 3]);
let _31: Adt61;
let _32: char;
let _33: bool;
let _34: isize;
let _35: Adt50;
let _36: i32;
let _37: usize;
let _38: *mut bool;
let _39: i16;
let _40: Adt51;
let _41: isize;
let _42: isize;
let _43: u8;
let _44: isize;
let _45: f64;
let _46: isize;
let _47: *mut ((char, i32), isize, u64, bool);
let _48: Adt61;
let _49: isize;
let _50: ();
let _51: ();
{
RET = ('\u{36630}', _1);
_13 = _8;
RET = ('\u{27abd}', _7);
_3 = _5;
RET.0 = '\u{a2529}';
RET.1 = _1 << _6;
_11 = [226_u8,60_u8,88_u8];
_4 = 189250459367137804725102721383192564879_u128 as i32;
RET.1 = _12 ^ _4;
_7 = _6 << _1;
Call(_2 = fn13(_14, _14, _6, _8, _3, _14, _14, _8, _8, _11, _1, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = _10;
_1 = _10;
_7 = 61_u8 as i32;
_7 = -_10;
_2 = _14 + _6;
_14 = _1;
RET.1 = !_6;
RET.0 = '\u{72d16}';
_16 = [12962_u16,5659_u16,25404_u16,31549_u16,51140_u16,2377_u16,16739_u16,38447_u16];
_15 = Adt58 { fld0: 7883457361965674962_u64 };
_12 = _7 >> _1;
_12 = 14120204616360128284_usize as i32;
RET.0 = '\u{be7ba}';
RET = ('\u{8e783}', _2);
_15 = Adt58 { fld0: 17602335131611495542_u64 };
_3 = _13;
RET.0 = '\u{561ca}';
_1 = _4;
_15.fld0 = false as u64;
_11 = [63_u8,35_u8,141_u8];
_5 = _8;
_2 = _6;
_7 = 20_u8 as i32;
RET.0 = '\u{29ca5}';
RET.1 = -_6;
_9 = _1;
RET = ('\u{8f215}', _6);
_12 = _6 - _4;
_14 = _2;
Goto(bb2)
}
bb2 = {
RET.1 = 84_isize as i32;
_15.fld0 = 32300_u16 as u64;
_15 = Adt58 { fld0: 11905395134165801211_u64 };
_18.0 = (-2279364467730443741_i64) * 6940451881716518053_i64;
_11 = [187_u8,201_u8,107_u8];
_18.2 = !145079359657107314077914151489554200891_i128;
RET.1 = 309252340300313360420122960584060743680_u128 as i32;
RET = ('\u{b2a2e}', _6);
_18.1 = true as u32;
_18.3 = 29538_i16;
_19 = !31383_u16;
_18.3 = 111_i8 as i16;
_12 = 115319859022245835713683762626930976682_u128 as i32;
_18 = ((-7450090452643433602_i64), 3677658713_u32, (-64803094410099427601044346627230911184_i128), 7056_i16);
_18.1 = !4151996183_u32;
_11 = [236_u8,237_u8,195_u8];
_15.fld0 = 11610268245373000254_u64 - 10035793843392097638_u64;
_5 = _11;
_3 = [82_u8,131_u8,104_u8];
_5 = _8;
_18.3 = (-18114_i16);
_18.0 = !7534503745874720608_i64;
RET.1 = _6;
_12 = _9;
_11 = [218_u8,217_u8,104_u8];
Goto(bb3)
}
bb3 = {
_12 = 4_usize as i32;
_1 = _15.fld0 as i32;
_3 = [160_u8,128_u8,201_u8];
_1 = _18.1 as i32;
_21 = _18.3 as f32;
_10 = -_2;
_8 = [211_u8,79_u8,79_u8];
RET.0 = '\u{c4c56}';
_20 = '\u{b3fe8}';
_1 = _20 as i32;
_20 = '\u{855da}';
_8 = [182_u8,117_u8,193_u8];
_6 = _14 * _1;
_4 = _18.1 as i32;
_23.0 = true ^ true;
_12 = _6 >> _18.0;
_9 = 11174862146783574427_usize as i32;
_18.2 = _15.fld0 as i128;
_26 = _20;
_26 = _20;
RET = (_26, _10);
_18 = (5800410028731709108_i64, 1695663897_u32, 143901989687671430551811602713005328588_i128, (-16164_i16));
Goto(bb4)
}
bb4 = {
_5 = _3;
_26 = _20;
_18 = ((-3899931687954912877_i64), 2445637527_u32, 168819268994903482143716487949819070117_i128, 27515_i16);
_20 = _26;
_21 = _18.0 as f32;
RET.1 = _21 as i32;
_6 = _18.1 as i32;
_30.0 = core::ptr::addr_of_mut!(_29);
_23 = (true, _2, true);
_30.1 = _20;
_31.fld6.3.0 = _18.3 as i64;
_1 = _6 | _14;
_25 = _18.2;
_31.fld6.3.1 = _12 as u32;
_15 = Adt58 { fld0: 14075662412419638313_u64 };
_24 = core::ptr::addr_of_mut!(_31.fld6.1);
_31.fld6.3.3 = _18.3 + _18.3;
_31.fld6.0 = _18.3 as u8;
_31.fld1.1 = _21 as u32;
_8 = [_31.fld6.0,_31.fld6.0,_31.fld6.0];
_31.fld4.fld0 = 9223372036854775807_isize as u16;
match _25 {
0 => bb5,
1 => bb6,
2 => bb7,
168819268994903482143716487949819070117 => bb9,
_ => bb8
}
}
bb5 = {
_12 = 4_usize as i32;
_1 = _15.fld0 as i32;
_3 = [160_u8,128_u8,201_u8];
_1 = _18.1 as i32;
_21 = _18.3 as f32;
_10 = -_2;
_8 = [211_u8,79_u8,79_u8];
RET.0 = '\u{c4c56}';
_20 = '\u{b3fe8}';
_1 = _20 as i32;
_20 = '\u{855da}';
_8 = [182_u8,117_u8,193_u8];
_6 = _14 * _1;
_4 = _18.1 as i32;
_23.0 = true ^ true;
_12 = _6 >> _18.0;
_9 = 11174862146783574427_usize as i32;
_18.2 = _15.fld0 as i128;
_26 = _20;
_26 = _20;
RET = (_26, _10);
_18 = (5800410028731709108_i64, 1695663897_u32, 143901989687671430551811602713005328588_i128, (-16164_i16));
Goto(bb4)
}
bb6 = {
RET.1 = 84_isize as i32;
_15.fld0 = 32300_u16 as u64;
_15 = Adt58 { fld0: 11905395134165801211_u64 };
_18.0 = (-2279364467730443741_i64) * 6940451881716518053_i64;
_11 = [187_u8,201_u8,107_u8];
_18.2 = !145079359657107314077914151489554200891_i128;
RET.1 = 309252340300313360420122960584060743680_u128 as i32;
RET = ('\u{b2a2e}', _6);
_18.1 = true as u32;
_18.3 = 29538_i16;
_19 = !31383_u16;
_18.3 = 111_i8 as i16;
_12 = 115319859022245835713683762626930976682_u128 as i32;
_18 = ((-7450090452643433602_i64), 3677658713_u32, (-64803094410099427601044346627230911184_i128), 7056_i16);
_18.1 = !4151996183_u32;
_11 = [236_u8,237_u8,195_u8];
_15.fld0 = 11610268245373000254_u64 - 10035793843392097638_u64;
_5 = _11;
_3 = [82_u8,131_u8,104_u8];
_5 = _8;
_18.3 = (-18114_i16);
_18.0 = !7534503745874720608_i64;
RET.1 = _6;
_12 = _9;
_11 = [218_u8,217_u8,104_u8];
Goto(bb3)
}
bb7 = {
_12 = _10;
_1 = _10;
_7 = 61_u8 as i32;
_7 = -_10;
_2 = _14 + _6;
_14 = _1;
RET.1 = !_6;
RET.0 = '\u{72d16}';
_16 = [12962_u16,5659_u16,25404_u16,31549_u16,51140_u16,2377_u16,16739_u16,38447_u16];
_15 = Adt58 { fld0: 7883457361965674962_u64 };
_12 = _7 >> _1;
_12 = 14120204616360128284_usize as i32;
RET.0 = '\u{be7ba}';
RET = ('\u{8e783}', _2);
_15 = Adt58 { fld0: 17602335131611495542_u64 };
_3 = _13;
RET.0 = '\u{561ca}';
_1 = _4;
_15.fld0 = false as u64;
_11 = [63_u8,35_u8,141_u8];
_5 = _8;
_2 = _6;
_7 = 20_u8 as i32;
RET.0 = '\u{29ca5}';
RET.1 = -_6;
_9 = _1;
RET = ('\u{8f215}', _6);
_12 = _6 - _4;
_14 = _2;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
(*_24) = _20;
_31.fld1 = (_18.0, _31.fld6.3.1, _25, _18.3);
_19 = _31.fld4.fld0;
_31.fld4.fld1.2 = [(-61_i8),(-52_i8),125_i8];
match _18.3 {
0 => bb4,
1 => bb10,
2 => bb11,
27515 => bb13,
_ => bb12
}
}
bb10 = {
RET.1 = 84_isize as i32;
_15.fld0 = 32300_u16 as u64;
_15 = Adt58 { fld0: 11905395134165801211_u64 };
_18.0 = (-2279364467730443741_i64) * 6940451881716518053_i64;
_11 = [187_u8,201_u8,107_u8];
_18.2 = !145079359657107314077914151489554200891_i128;
RET.1 = 309252340300313360420122960584060743680_u128 as i32;
RET = ('\u{b2a2e}', _6);
_18.1 = true as u32;
_18.3 = 29538_i16;
_19 = !31383_u16;
_18.3 = 111_i8 as i16;
_12 = 115319859022245835713683762626930976682_u128 as i32;
_18 = ((-7450090452643433602_i64), 3677658713_u32, (-64803094410099427601044346627230911184_i128), 7056_i16);
_18.1 = !4151996183_u32;
_11 = [236_u8,237_u8,195_u8];
_15.fld0 = 11610268245373000254_u64 - 10035793843392097638_u64;
_5 = _11;
_3 = [82_u8,131_u8,104_u8];
_5 = _8;
_18.3 = (-18114_i16);
_18.0 = !7534503745874720608_i64;
RET.1 = _6;
_12 = _9;
_11 = [218_u8,217_u8,104_u8];
Goto(bb3)
}
bb11 = {
_5 = _3;
_26 = _20;
_18 = ((-3899931687954912877_i64), 2445637527_u32, 168819268994903482143716487949819070117_i128, 27515_i16);
_20 = _26;
_21 = _18.0 as f32;
RET.1 = _21 as i32;
_6 = _18.1 as i32;
_30.0 = core::ptr::addr_of_mut!(_29);
_23 = (true, _2, true);
_30.1 = _20;
_31.fld6.3.0 = _18.3 as i64;
_1 = _6 | _14;
_25 = _18.2;
_31.fld6.3.1 = _12 as u32;
_15 = Adt58 { fld0: 14075662412419638313_u64 };
_24 = core::ptr::addr_of_mut!(_31.fld6.1);
_31.fld6.3.3 = _18.3 + _18.3;
_31.fld6.0 = _18.3 as u8;
_31.fld1.1 = _21 as u32;
_8 = [_31.fld6.0,_31.fld6.0,_31.fld6.0];
_31.fld4.fld0 = 9223372036854775807_isize as u16;
match _25 {
0 => bb5,
1 => bb6,
2 => bb7,
168819268994903482143716487949819070117 => bb9,
_ => bb8
}
}
bb12 = {
RET.1 = 84_isize as i32;
_15.fld0 = 32300_u16 as u64;
_15 = Adt58 { fld0: 11905395134165801211_u64 };
_18.0 = (-2279364467730443741_i64) * 6940451881716518053_i64;
_11 = [187_u8,201_u8,107_u8];
_18.2 = !145079359657107314077914151489554200891_i128;
RET.1 = 309252340300313360420122960584060743680_u128 as i32;
RET = ('\u{b2a2e}', _6);
_18.1 = true as u32;
_18.3 = 29538_i16;
_19 = !31383_u16;
_18.3 = 111_i8 as i16;
_12 = 115319859022245835713683762626930976682_u128 as i32;
_18 = ((-7450090452643433602_i64), 3677658713_u32, (-64803094410099427601044346627230911184_i128), 7056_i16);
_18.1 = !4151996183_u32;
_11 = [236_u8,237_u8,195_u8];
_15.fld0 = 11610268245373000254_u64 - 10035793843392097638_u64;
_5 = _11;
_3 = [82_u8,131_u8,104_u8];
_5 = _8;
_18.3 = (-18114_i16);
_18.0 = !7534503745874720608_i64;
RET.1 = _6;
_12 = _9;
_11 = [218_u8,217_u8,104_u8];
Goto(bb3)
}
bb13 = {
_18.3 = _31.fld6.3.3 >> _31.fld6.3.3;
_31.fld5 = 3_usize & 7779024388809986266_usize;
_36 = _23.1;
_35.fld0 = ((-111_i8),);
_12 = _35.fld0.0 as i32;
_31.fld1.2 = _15.fld0 as i128;
_31.fld3 = [_18.3,_18.3,_31.fld1.3,_18.3,_18.3,_31.fld6.3.3,_18.3,_18.3];
_35.fld0.0 = 86_i8;
_30.2 = _31.fld4.fld1.2;
_32 = (*_24);
_23 = (false, _12, true);
_40.fld1.0 = core::ptr::addr_of_mut!(_31.fld6.0);
_8 = _5;
_35.fld5 = !_31.fld6.0;
_40.fld1 = _30;
_26 = _31.fld6.1;
Call(_35.fld4.3 = core::intrinsics::transmute((*_24)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_44 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_35.fld4.1.1 = _36;
_31.fld6.3 = (_31.fld1.0, _31.fld1.1, _31.fld1.2, _18.3);
_31.fld2 = core::ptr::addr_of_mut!(_40.fld1.0);
_35.fld4.4 = _44 << _15.fld0;
_39 = -_18.3;
_35.fld4.0.2 = _23.0;
_35.fld4.2 = _21;
_35.fld4.0 = (_23.0, _6, _23.2);
_48.fld4.fld1 = _40.fld1;
_27 = _15.fld0 as i128;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(12_usize, 25_usize, Move(_25), 5_usize, Move(_5), 19_usize, Move(_19), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(12_usize, 6_usize, Move(_6), 4_usize, Move(_4), 18_usize, Move(_18), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(12_usize, 36_usize, Move(_36), 2_usize, Move(_2), 13_usize, Move(_13), 39_usize, Move(_39)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(12_usize, 16_usize, Move(_16), 51_usize, _51, 51_usize, _51, 51_usize, _51), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: i32,mut _2: i32,mut _3: i32,mut _4: [u8; 3],mut _5: [u8; 3],mut _6: i32,mut _7: i32,mut _8: [u8; 3],mut _9: [u8; 3],mut _10: [u8; 3],mut _11: i32,mut _12: [u8; 3]) -> i32 {
mir! {
type RET = i32;
let _13: usize;
let _14: ((bool, i32, bool), (char, i32), f32, u32, isize);
let _15: u32;
let _16: Adt56;
let _17: u16;
let _18: f64;
let _19: *mut u8;
let _20: Adt58;
let _21: isize;
let _22: i128;
let _23: Adt64;
let _24: i8;
let _25: ([u32; 1], i32, bool);
let _26: u128;
let _27: isize;
let _28: isize;
let _29: f64;
let _30: f64;
let _31: *const u32;
let _32: [u8; 3];
let _33: u128;
let _34: Adt57;
let _35: Adt64;
let _36: u32;
let _37: Adt59;
let _38: *mut ((char, i32), isize, u64, bool);
let _39: *const [u32; 6];
let _40: isize;
let _41: f32;
let _42: char;
let _43: *const u32;
let _44: char;
let _45: Adt55;
let _46: [char; 2];
let _47: i16;
let _48: (i64, u32, i128, i16);
let _49: ();
let _50: ();
{
_13 = 13019885426667528111_usize / 8262171830614705968_usize;
RET = !_1;
_9 = _5;
_3 = _2;
_4 = [18_u8,82_u8,210_u8];
_8 = [120_u8,28_u8,240_u8];
_14.4 = 9223372036854775807_isize;
RET = 5121366907214692617_i64 as i32;
_2 = 21040_u16 as i32;
_4 = _9;
_14.0.2 = false ^ true;
_14.1 = ('\u{e724c}', _1);
_12 = [156_u8,137_u8,93_u8];
_10 = [242_u8,236_u8,137_u8];
_14.0.0 = !_14.0.2;
_16.fld1 = [14117_i16,3000_i16,(-17155_i16),(-32326_i16),281_i16,15923_i16,9463_i16,5538_i16];
_14.3 = 2770450347_u32;
_3 = _14.0.2 as i32;
_14.0.1 = _14.1.1;
_14.0.1 = !_1;
_14.3 = !3250189718_u32;
_6 = _14.0.1 ^ _2;
_10 = [253_u8,116_u8,77_u8];
Goto(bb1)
}
bb1 = {
_13 = !12971692130071303887_usize;
_9 = [197_u8,193_u8,20_u8];
_17 = 103_i8 as u16;
match _14.4 {
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
_3 = -_7;
_23.fld3.fld4.0.2 = _14.0.2;
_22 = 222_u8 as i128;
_15 = 23739_i16 as u32;
_23.fld3.fld4.2 = (-23182_i16) as f32;
_23.fld2.3 = [_14.1.0,_14.1.0];
_14.0.1 = _14.3 as i32;
_23.fld3.fld4.4 = _14.4 >> _3;
_23.fld4.1 = _14.1.0;
_12 = _10;
_23.fld2.4 = (_14.1.0, _3);
_14.1.1 = -_3;
_23.fld2.3 = [_14.1.0,_23.fld2.4.0];
_23.fld3.fld4.1 = _14.1;
_23.fld1.fld1.fld1 = [_23.fld2.4.0,_14.1.0];
_23.fld1.fld1.fld0 = (_14.0, _23.fld2.4, _23.fld3.fld4.2, _14.3, _14.4);
_15 = _14.3 ^ _14.3;
_23.fld3.fld4.0.0 = _14.0.0;
_13 = !1773516797406222163_usize;
_23.fld1.fld1.fld0.2 = -_23.fld3.fld4.2;
_25.1 = _6;
_14.1.1 = _2 & _6;
_23.fld4.2 = [(-99_i8),(-97_i8),(-35_i8)];
Call(_23.fld1.fld1.fld0.4 = core::intrinsics::bswap(_14.4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_23.fld2.4.1 = _23.fld1.fld1.fld0.0.1 & _14.1.1;
_23.fld3.fld4.3 = 106534061525046844574439572082911932152_u128 as u32;
match _14.4 {
9223372036854775807 => bb8,
_ => bb3
}
}
bb8 = {
_23.fld1.fld2 = 14442472377272680302_u64 as u128;
_3 = 123_u8 as i32;
_23.fld4.1 = _23.fld3.fld4.1.0;
_23.fld0 = _22 as f64;
_23.fld3.fld4.0 = (_23.fld1.fld1.fld0.0.2, _11, _23.fld1.fld1.fld0.0.2);
_23.fld2.4 = (_23.fld3.fld4.1.0, _2);
_29 = _23.fld0;
_23.fld2.3 = [_23.fld3.fld4.1.0,_23.fld1.fld1.fld0.1.0];
_23.fld3.fld4.0.2 = !_14.0.2;
_12 = [95_u8,116_u8,140_u8];
_25.0 = [_14.3];
_14.2 = _23.fld3.fld4.2;
RET = -_1;
_3 = !_25.1;
_14.3 = 1309123978073134408_u64 as u32;
_23.fld3.fld4.1 = _14.1;
_23.fld6 = (-4057217206172816355_i64) & 5188476915539355029_i64;
_23.fld3.fld4 = (_23.fld1.fld1.fld0.0, _14.1, _23.fld1.fld1.fld0.2, _15, _14.4);
_14.0.1 = _23.fld2.4.1;
_23.fld3.fld0 = (94_i8,);
_23.fld2.4 = _14.1;
_23.fld2.1 = core::ptr::addr_of_mut!(_23.fld2.4.0);
Call(_25.2 = fn14(_23.fld1.fld1.fld0.3, _23.fld3.fld4.2, _23.fld1.fld1.fld0.2, _14.0.2, _23.fld1.fld1.fld0.1.0, _23.fld2.4, _15, _23.fld1.fld1.fld0.3, _23.fld4.2, _23.fld1.fld1.fld0.1, _23.fld4.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_23.fld1.fld1.fld0.1.0 = _23.fld2.4.0;
_23.fld1.fld1.fld0.3 = !_15;
_35.fld1.fld1.fld0.1.1 = _6;
_23.fld1.fld1.fld0.0.1 = _23.fld3.fld4.0.0 as i32;
_23.fld1.fld1.fld0.1.0 = _23.fld3.fld4.1.0;
_30 = _29;
_16.fld2.2 = _23.fld1.fld1.fld0.0.0 < _23.fld1.fld1.fld0.0.0;
_23.fld6 = 1859261658706075405_i64 * (-3214979787196556210_i64);
_35.fld3.fld1 = _23.fld1.fld1.fld0.1.0;
_35.fld4.1 = _23.fld1.fld1.fld0.1.0;
_35.fld1.fld1.fld0.2 = _23.fld1.fld1.fld0.2;
_16.fld2.1 = -_25.1;
_20.fld0 = 3280154504497758805_u64;
_14.1 = _23.fld1.fld1.fld0.1;
Goto(bb10)
}
bb10 = {
_35.fld1.fld0 = _23.fld3.fld4.0.0;
_23.fld1.fld1.fld0.1 = (_23.fld3.fld4.1.0, _16.fld2.1);
_14.4 = _13 as isize;
_23.fld1.fld1.fld0 = _14;
_23.fld4.2 = [_23.fld3.fld0.0,_23.fld3.fld0.0,_23.fld3.fld0.0];
_35.fld3.fld4.4 = _23.fld1.fld1.fld0.4 + _14.4;
_21 = _17 as isize;
_23.fld7 = [180_u8,97_u8,33_u8];
_35.fld6 = !_23.fld6;
_35.fld3.fld4 = (_23.fld1.fld1.fld0.0, _14.1, _23.fld1.fld1.fld0.2, _14.3, _14.4);
_14.3 = _30 as u32;
_35.fld1 = Adt55 { fld0: _16.fld2.2,fld1: _23.fld1.fld1,fld2: _23.fld1.fld2,fld3: _23.fld2.1 };
Goto(bb11)
}
bb11 = {
_23.fld3.fld1 = _23.fld4.1;
_23.fld4.0 = core::ptr::addr_of_mut!(_37.fld3);
_35.fld1.fld1 = Adt52 { fld0: _35.fld3.fld4,fld1: _23.fld2.3 };
_22 = _23.fld4.1 as i128;
_23.fld3.fld4 = (_35.fld3.fld4.0, _23.fld2.4, _35.fld3.fld4.2, _35.fld1.fld1.fld0.3, _35.fld1.fld1.fld0.4);
_14.2 = _23.fld3.fld4.2 - _23.fld3.fld4.2;
_31 = core::ptr::addr_of!(_15);
_35.fld3.fld2 = !_20.fld0;
_23.fld1.fld0 = _23.fld3.fld4.0.0 != _35.fld1.fld1.fld0.0.0;
_23.fld3.fld0.0 = 7_i8;
_14 = (_35.fld3.fld4.0, _23.fld1.fld1.fld0.1, _23.fld3.fld4.2, _15, _21);
_14.1.1 = !_23.fld1.fld1.fld0.1.1;
_35.fld1.fld2 = _35.fld6 as u128;
_14.1.0 = _35.fld1.fld1.fld0.1.0;
_35.fld3.fld4.4 = _23.fld3.fld4.4;
_41 = -_23.fld1.fld1.fld0.2;
_23.fld2.0 = _14.1;
_45.fld1.fld0.0.1 = !_23.fld3.fld4.1.1;
_23.fld3.fld0 = (90_i8,);
match _23.fld3.fld0.0 {
0 => bb12,
1 => bb13,
90 => bb15,
_ => bb14
}
}
bb12 = {
_35.fld1.fld0 = _23.fld3.fld4.0.0;
_23.fld1.fld1.fld0.1 = (_23.fld3.fld4.1.0, _16.fld2.1);
_14.4 = _13 as isize;
_23.fld1.fld1.fld0 = _14;
_23.fld4.2 = [_23.fld3.fld0.0,_23.fld3.fld0.0,_23.fld3.fld0.0];
_35.fld3.fld4.4 = _23.fld1.fld1.fld0.4 + _14.4;
_21 = _17 as isize;
_23.fld7 = [180_u8,97_u8,33_u8];
_35.fld6 = !_23.fld6;
_35.fld3.fld4 = (_23.fld1.fld1.fld0.0, _14.1, _23.fld1.fld1.fld0.2, _14.3, _14.4);
_14.3 = _30 as u32;
_35.fld1 = Adt55 { fld0: _16.fld2.2,fld1: _23.fld1.fld1,fld2: _23.fld1.fld2,fld3: _23.fld2.1 };
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
_23.fld2.4.1 = _23.fld1.fld1.fld0.0.1 & _14.1.1;
_23.fld3.fld4.3 = 106534061525046844574439572082911932152_u128 as u32;
match _14.4 {
9223372036854775807 => bb8,
_ => bb3
}
}
bb15 = {
_37.fld1.0 = _23.fld2.0;
_45.fld1.fld1 = _23.fld2.3;
_35.fld0 = _23.fld1.fld1.fld0.2 as f64;
_45.fld0 = !_14.0.2;
_35.fld2.0 = _35.fld3.fld4.1;
_23.fld1.fld1.fld1 = [_35.fld3.fld4.1.0,_35.fld4.1];
_35.fld3.fld4.1 = (_37.fld1.0.0, _11);
_23.fld1.fld3 = core::ptr::addr_of_mut!(_35.fld4.1);
_35.fld3.fld4.4 = _35.fld1.fld2 as isize;
_48.0 = (-16733_i16) as i64;
_45.fld1.fld0.4 = _21;
_19 = core::ptr::addr_of_mut!(_37.fld3);
_23.fld1.fld1.fld0.3 = !_14.3;
_37.fld1.0 = (_23.fld3.fld4.1.0, _25.1);
_35.fld1.fld1.fld0.0.2 = !_14.0.0;
Goto(bb16)
}
bb16 = {
Call(_49 = dump_var(13_usize, 17_usize, Move(_17), 15_usize, Move(_15), 12_usize, Move(_12), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(13_usize, 13_usize, Move(_13), 3_usize, Move(_3), 21_usize, Move(_21), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(13_usize, 6_usize, Move(_6), 50_usize, _50, 50_usize, _50, 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: u32,mut _2: f32,mut _3: f32,mut _4: bool,mut _5: char,mut _6: (char, i32),mut _7: u32,mut _8: u32,mut _9: [i8; 3],mut _10: (char, i32),mut _11: char) -> bool {
mir! {
type RET = bool;
let _12: isize;
let _13: char;
let _14: f64;
let _15: ((char, i32), isize, u64, bool);
let _16: (u8, char, [char; 2], (i64, u32, i128, i16), [char; 2]);
let _17: ([u32; 1], i32, bool);
let _18: *const *mut u32;
let _19: Adt56;
let _20: [i8; 3];
let _21: f64;
let _22: u32;
let _23: u32;
let _24: bool;
let _25: ();
let _26: ();
{
RET = _4;
_11 = _10.0;
_2 = _3 * _3;
_10.0 = _5;
_7 = _4 as u32;
_2 = _3;
_12 = 139_u8 as isize;
_11 = _6.0;
_4 = _11 >= _6.0;
RET = !_4;
_1 = !_8;
RET = _4;
_13 = _5;
_12 = (-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
_6 = (_11, _10.1);
_11 = _6.0;
RET = _4;
_2 = _3;
_2 = _3 * _3;
match _12 {
340282366920938463454151235394913435648 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_2 = _3;
_3 = -_2;
_6.1 = _10.1;
_10 = (_6.0, _6.1);
_7 = (-65853110706996595457290465877425363346_i128) as u32;
_12 = !(-51_isize);
_14 = 165_u8 as f64;
_15.1 = _12;
_15 = (_10, _12, 11878292821330154589_u64, _4);
_11 = _10.0;
_2 = _3;
_10.1 = _15.0.1;
_3 = _2 / f32::NEG_INFINITY;
_16.3.3 = 22368_i16;
_15 = (_6, _12, 2285105451899873658_u64, _4);
_16.2 = [_10.0,_5];
_4 = !_15.3;
_10.1 = _15.1 as i32;
_5 = _10.0;
_15.1 = _12 | _12;
_17.1 = _15.0.1;
match _15.2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
2285105451899873658 => bb10,
_ => bb9
}
}
bb4 = {
Return()
}
bb5 = {
_6 = (_11, _10.1);
_11 = _6.0;
RET = _4;
_2 = _3;
_2 = _3 * _3;
match _12 {
340282366920938463454151235394913435648 => bb3,
_ => bb2
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
Return()
}
bb10 = {
_15.3 = _10.0 == _15.0.0;
_16.0 = _14 as u8;
_17.0 = [_7];
_16.3 = (2515425085973615902_i64, _7, (-109308381554593988451278305372208032060_i128), (-24779_i16));
_15.2 = 245856607558712275332843620322589516018_u128 as u64;
RET = _16.3.3 > _16.3.3;
_11 = _15.0.0;
_19.fld1 = [_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3];
_15.3 = _11 != _11;
_6 = (_15.0.0, _10.1);
_17.1 = -_10.1;
_19.fld2 = (_17.0, _10.1, _4);
RET = _16.3.0 <= _16.3.0;
_17 = (_19.fld2.0, _19.fld2.1, _4);
_19.fld2.0 = [_1];
_7 = _8 << _10.1;
_2 = _3;
_15.3 = !_19.fld2.2;
_17.0 = _19.fld2.0;
_16.1 = _15.0.0;
Call(_13 = fn15(_14, _15.2, _16.3.1, _10, _6.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9 = [112_i8,(-36_i8),19_i8];
_4 = _15.3;
_16.3.3 = (-25184_i16) * 7781_i16;
_17 = (_19.fld2.0, _6.1, _4);
_10.0 = _15.0.0;
_16.3.2 = !145146518646172661984053453547942940838_i128;
_10.0 = _6.0;
RET = _12 <= _12;
match _16.3.0 {
0 => bb7,
1 => bb8,
2 => bb6,
3 => bb4,
2515425085973615902 => bb12,
_ => bb10
}
}
bb12 = {
_19.fld1 = [_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3];
_15.3 = _4;
_19.fld2.1 = _6.1;
_8 = !_1;
_16.3.0 = (-1047323250799224583_i64) & (-8321536054789953294_i64);
_12 = _15.1;
_19.fld1 = [_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3];
_17.2 = _19.fld2.2 | _19.fld2.2;
_16.3.2 = (-55252774605819435929815650159417128875_i128);
_16.1 = _6.0;
_16.4 = [_10.0,_15.0.0];
_21 = _15.1 as f64;
_9 = [(-35_i8),59_i8,36_i8];
_15.3 = _4;
_10.1 = _19.fld2.1 ^ _6.1;
match _16.3.2 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
285029592315119027533558957272351082581 => bb19,
_ => bb18
}
}
bb13 = {
_9 = [112_i8,(-36_i8),19_i8];
_4 = _15.3;
_16.3.3 = (-25184_i16) * 7781_i16;
_17 = (_19.fld2.0, _6.1, _4);
_10.0 = _15.0.0;
_16.3.2 = !145146518646172661984053453547942940838_i128;
_10.0 = _6.0;
RET = _12 <= _12;
match _16.3.0 {
0 => bb7,
1 => bb8,
2 => bb6,
3 => bb4,
2515425085973615902 => bb12,
_ => bb10
}
}
bb14 = {
_15.3 = _10.0 == _15.0.0;
_16.0 = _14 as u8;
_17.0 = [_7];
_16.3 = (2515425085973615902_i64, _7, (-109308381554593988451278305372208032060_i128), (-24779_i16));
_15.2 = 245856607558712275332843620322589516018_u128 as u64;
RET = _16.3.3 > _16.3.3;
_11 = _15.0.0;
_19.fld1 = [_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3,_16.3.3];
_15.3 = _11 != _11;
_6 = (_15.0.0, _10.1);
_17.1 = -_10.1;
_19.fld2 = (_17.0, _10.1, _4);
RET = _16.3.0 <= _16.3.0;
_17 = (_19.fld2.0, _19.fld2.1, _4);
_19.fld2.0 = [_1];
_7 = _8 << _10.1;
_2 = _3;
_15.3 = !_19.fld2.2;
_17.0 = _19.fld2.0;
_16.1 = _15.0.0;
Call(_13 = fn15(_14, _15.2, _16.3.1, _10, _6.1), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_14 = _21 + _21;
_16.4 = [_13,_5];
_23 = _17.2 as u32;
_19.fld2.1 = _10.1 | _6.1;
_20 = [(-115_i8),(-123_i8),42_i8];
_15.0.1 = _10.1;
_7 = !_23;
_16.1 = _13;
_8 = _7 % 1717884997_u32;
_16.3.1 = _16.3.3 as u32;
_7 = _10.1 as u32;
_6 = (_13, _10.1);
_6.0 = _15.0.0;
_19.fld2.2 = !_17.2;
_6.0 = _11;
_5 = _16.1;
_4 = _19.fld2.2 ^ _15.3;
Goto(bb20)
}
bb20 = {
Call(_25 = dump_var(14_usize, 10_usize, Move(_10), 4_usize, Move(_4), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_25 = dump_var(14_usize, 20_usize, Move(_20), 16_usize, Move(_16), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: f64,mut _2: u64,mut _3: u32,mut _4: (char, i32),mut _5: i32) -> char {
mir! {
type RET = char;
let _6: ([u32; 1], i32, bool);
let _7: (([u32; 1], i32, bool),);
let _8: *mut u32;
let _9: Adt63;
let _10: usize;
let _11: i128;
let _12: Adt60;
let _13: *mut *mut u8;
let _14: (char, i32);
let _15: [char; 2];
let _16: u8;
let _17: char;
let _18: Adt60;
let _19: *mut *mut u8;
let _20: u32;
let _21: isize;
let _22: bool;
let _23: char;
let _24: Adt59;
let _25: i8;
let _26: Adt57;
let _27: Adt63;
let _28: i16;
let _29: Adt51;
let _30: ();
let _31: ();
{
RET = _4.0;
_2 = _4.1 as u64;
_4.1 = _5 ^ _5;
_5 = _4.1 | _4.1;
_8 = core::ptr::addr_of_mut!(_3);
_6.0 = [(*_8)];
_4.1 = _5;
_7.0.0 = [(*_8)];
RET = _4.0;
_6.1 = _4.1 ^ _4.1;
_6.2 = false;
_7 = (_6,);
_6.1 = -_5;
_2 = 5129083906685815722_u64;
_4.1 = -_7.0.1;
_4.0 = '\u{805dc}';
_4.0 = '\u{6a8c2}';
_7.0.2 = _4.0 <= _4.0;
Call(_10 = fn16(_7.0, _7.0.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.0.0 = _6.0;
_1 = (-5943951298658228108_i64) as f64;
_7.0.0 = [(*_8)];
_12.fld6.0.0 = _4.0;
Goto(bb2)
}
bb2 = {
_12.fld3.fld0.4 = 30793_i16 as isize;
_1 = 65354_u16 as f64;
_12.fld5.2 = 63_u8 as i32;
_12.fld4.fld1.2 = [(-29_i8),0_i8,111_i8];
_12.fld1.1 = _12.fld6.0.0;
_8 = core::ptr::addr_of_mut!((*_8));
_9 = Adt63 { fld0: _12.fld4.fld1.2 };
_12.fld5.0 = (*_8);
_12.fld2.fld0.0 = 8_i8 | 107_i8;
_12.fld2.fld4.0 = (_7.0.2, _6.1, _7.0.2);
_12.fld6.2 = 81_u8 as u64;
_12.fld6.0.0 = _4.0;
match _2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
5129083906685815722 => bb9,
_ => bb8
}
}
bb3 = {
_7.0.0 = _6.0;
_1 = (-5943951298658228108_i64) as f64;
_7.0.0 = [(*_8)];
_12.fld6.0.0 = _4.0;
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
_12.fld3.fld0.0 = (_7.0.2, _6.1, _12.fld2.fld4.0.0);
_6.1 = !_4.1;
_18.fld2.fld2 = 52882911616360947965971380617760894009_i128 as u64;
_12.fld2.fld4.2 = _3 as f32;
_12.fld6.0.1 = _6.1 & _6.1;
_15 = [_12.fld1.1,_4.0];
_12.fld6 = (_4, _12.fld3.fld0.4, _18.fld2.fld2, _12.fld2.fld4.0.2);
_18.fld3.fld0.4 = _12.fld3.fld0.4 >> _4.1;
_19 = core::ptr::addr_of_mut!(_18.fld4.fld1.0);
_12.fld4.fld0 = 61_u8 as u16;
_2 = _12.fld6.2 & _12.fld6.2;
(*_19) = core::ptr::addr_of_mut!(_12.fld2.fld5);
_12.fld2.fld4.1 = _4;
_18.fld6.0.0 = _4.0;
_18.fld2.fld0 = (_12.fld2.fld0.0,);
_18.fld3.fld0.0 = (_12.fld3.fld0.0.0, _12.fld6.0.1, _12.fld3.fld0.0.2);
_18.fld6.2 = !_12.fld6.2;
_18.fld1.1 = _12.fld1.1;
(*_19) = core::ptr::addr_of_mut!(_12.fld2.fld5);
_18.fld6.2 = !_2;
_18.fld5 = ((*_8), _12.fld4.fld0, _6.1);
_18.fld1.1 = _12.fld2.fld4.1.0;
_7.0 = _6;
Goto(bb10)
}
bb10 = {
_18.fld3.fld0.1.0 = _18.fld6.0.0;
_12.fld3.fld0.2 = _1 as f32;
_18.fld3.fld0 = (_12.fld3.fld0.0, _12.fld2.fld4.1, _12.fld3.fld0.2, _12.fld5.0, _12.fld3.fld0.4);
_12.fld2.fld3 = core::ptr::addr_of!(_18.fld2.fld4.3);
_18.fld2.fld2 = _2;
_10 = 7751772080788652439_usize >> _12.fld2.fld4.0.1;
(*_8) = !_18.fld3.fld0.3;
_7.0.0 = [(*_8)];
(*_19) = core::ptr::addr_of_mut!(_24.fld3);
_18.fld2.fld4.1 = _12.fld6.0;
Goto(bb11)
}
bb11 = {
_24.fld1.1 = core::ptr::addr_of_mut!(_12.fld3.fld0.1.0);
_18.fld1.2 = [_12.fld2.fld0.0,_18.fld2.fld0.0,_18.fld2.fld0.0];
_12.fld6.0 = _18.fld2.fld4.1;
_12.fld3.fld0 = (_18.fld3.fld0.0, _4, _12.fld2.fld4.2, (*_8), _12.fld6.1);
_24.fld1.2 = core::ptr::addr_of_mut!(_16);
_23 = _12.fld1.1;
_18.fld1.2 = [_12.fld2.fld0.0,_18.fld2.fld0.0,_12.fld2.fld0.0];
_24.fld1.1 = core::ptr::addr_of_mut!(_24.fld1.4.0);
_18.fld3 = Adt52 { fld0: _12.fld3.fld0,fld1: _15 };
_22 = !_12.fld3.fld0.0.2;
_18.fld5 = ((*_8), _12.fld4.fld0, _18.fld3.fld0.1.1);
_18.fld1.0 = core::ptr::addr_of_mut!(_18.fld2.fld5);
Goto(bb12)
}
bb12 = {
_18.fld3.fld0.2 = _12.fld3.fld0.2 * _12.fld3.fld0.2;
_18.fld6.3 = _22;
_18.fld3.fld0 = (_12.fld3.fld0.0, _12.fld6.0, _12.fld2.fld4.2, _18.fld5.0, _12.fld3.fld0.4);
_18.fld2.fld4.0.1 = _5;
_24.fld1.4 = (_12.fld2.fld4.1.0, _6.1);
_24.fld1.0 = _18.fld3.fld0.1;
_18.fld6 = (_12.fld2.fld4.1, _12.fld6.1, _12.fld6.2, _18.fld3.fld0.0.0);
_18.fld4.fld1.2 = [_18.fld2.fld0.0,_12.fld2.fld0.0,_12.fld2.fld0.0];
_18.fld4.fld1.0 = core::ptr::addr_of_mut!(_24.fld3);
_18.fld4.fld1.0 = core::ptr::addr_of_mut!(_12.fld2.fld5);
_12.fld3.fld0.1.1 = (-143441229883904385382260932562744388028_i128) as i32;
_18.fld2.fld0 = (_12.fld2.fld0.0,);
_18.fld2.fld3 = core::ptr::addr_of!(_12.fld2.fld4.3);
_21 = _18.fld6.1;
_6.1 = _12.fld3.fld0.0.2 as i32;
_12.fld4.fld1 = _18.fld1;
_7.0.1 = -_12.fld3.fld0.0.1;
_25 = -_18.fld2.fld0.0;
_12.fld2.fld4.1.1 = _18.fld3.fld0.2 as i32;
_18.fld2.fld5 = 40_u8 / 218_u8;
_12.fld6.2 = _2 - _18.fld2.fld2;
_10 = !13160484710721844912_usize;
_6.2 = _12.fld3.fld0.0.2;
_6.1 = _12.fld4.fld0 as i32;
_12.fld2.fld5 = _18.fld2.fld5 / 74_u8;
Goto(bb13)
}
bb13 = {
(*_19) = core::ptr::addr_of_mut!(_24.fld3);
_12.fld5.1 = (-3553_i16) as u16;
_12.fld2.fld2 = _18.fld2.fld2 & _18.fld6.2;
_7.0 = (_6.0, _24.fld1.0.1, _12.fld3.fld0.0.0);
_18.fld2.fld0.0 = !_12.fld2.fld0.0;
Goto(bb14)
}
bb14 = {
_12.fld3.fld0.1 = (_18.fld2.fld4.1.0, _24.fld1.4.1);
_12.fld6.2 = !_12.fld2.fld2;
_18.fld5 = (_12.fld5.0, _12.fld4.fld0, _4.1);
_3 = _18.fld5.0;
_6 = (_7.0.0, _18.fld3.fld0.1.1, _22);
_24.fld0 = [_12.fld5.0,_12.fld5.0,(*_8),(*_8),_12.fld5.0,_18.fld3.fld0.3];
_12.fld6.0 = (_18.fld2.fld4.1.0, _18.fld3.fld0.0.1);
_18.fld2.fld4.1.0 = _12.fld4.fld1.1;
_18.fld2.fld4.2 = _12.fld2.fld4.2;
_18.fld2.fld4.1.0 = _24.fld1.4.0;
_14.0 = _12.fld1.1;
_18.fld3.fld0.1 = (_23, _4.1);
_27 = Adt63 { fld0: _12.fld4.fld1.2 };
_12.fld3.fld0.1.0 = _4.0;
_12.fld4.fld1.1 = _12.fld1.1;
_24.fld1.0 = (_23, _18.fld2.fld4.0.1);
_18.fld5.0 = _12.fld3.fld0.3;
_6.2 = !_12.fld3.fld0.0.2;
_14.1 = _4.1 ^ _18.fld3.fld0.0.1;
_18.fld3.fld0.0.2 = !_18.fld3.fld0.0.0;
_9.fld0 = [_25,_25,_12.fld2.fld0.0];
_18.fld3.fld0.2 = _12.fld3.fld0.2;
_18.fld2.fld3 = core::ptr::addr_of!(_12.fld3.fld0.3);
_18.fld2.fld4.2 = _1 as f32;
_18.fld2.fld4.0 = (_12.fld2.fld4.0.2, _12.fld3.fld0.0.1, _12.fld2.fld4.0.2);
(*_19) = core::ptr::addr_of_mut!(_24.fld3);
_14 = (_12.fld1.1, _24.fld1.4.1);
_24.fld1.2 = _12.fld4.fld1.0;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(15_usize, 22_usize, Move(_22), 21_usize, Move(_21), 15_usize, Move(_15), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(15_usize, 4_usize, Move(_4), 2_usize, Move(_2), 31_usize, _31, 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: ([u32; 1], i32, bool),mut _2: bool) -> usize {
mir! {
type RET = usize;
let _3: (u8, char, [char; 2], (i64, u32, i128, i16), [char; 2]);
let _4: i16;
let _5: [u8; 3];
let _6: i128;
let _7: ((bool, i32, bool), (char, i32), f32, u32, isize);
let _8: (u8, char, [char; 2], (i64, u32, i128, i16), [char; 2]);
let _9: ();
let _10: ();
{
_3.3.3 = -7560_i16;
_3.3 = ((-6537368332539329201_i64), 3613044477_u32, 133770190622639342486856226286300542423_i128, 14335_i16);
_1.0 = [_3.3.1];
_3.2 = ['\u{a6beb}','\u{56ea7}'];
_3.0 = 77_u8 % 213_u8;
_2 = _3.3.2 <= _3.3.2;
_4 = -_3.3.3;
_3.3.1 = 81_i8 as u32;
RET = !4476105341911420030_usize;
_7.0 = (_2, _1.1, _2);
_3.2 = ['\u{ec041}','\u{fdb0c}'];
_7.0.0 = !_2;
_7.1.1 = _7.0.1;
Goto(bb1)
}
bb1 = {
_2 = _7.0.2 ^ _7.0.0;
_8.3.0 = -_3.3.0;
_7.3 = _3.3.0 as u32;
_8.4 = ['\u{45967}','\u{eb83e}'];
_7.0.2 = _2 != _7.0.0;
_2 = _1.2 ^ _7.0.2;
_3.3 = (_8.3.0, _7.3, 958141873455512671520564286780817749_i128, _4);
_3.3.3 = _4 ^ _4;
_7.1.1 = _7.0.1 - _7.0.1;
_7.0.2 = !_2;
_3.2 = ['\u{c54ec}','\u{84c2f}'];
RET = _7.1.1 as usize;
_3.2 = ['\u{dc607}','\u{3cf73}'];
_7.1.0 = '\u{96a89}';
_8.3.2 = _3.3.2;
_3.3 = (_8.3.0, _7.3, _8.3.2, _4);
_8.1 = _7.1.0;
_2 = !_7.0.2;
Goto(bb2)
}
bb2 = {
Call(_9 = dump_var(16_usize, 4_usize, Move(_4), 10_usize, _10, 10_usize, _10, 10_usize, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *mut *mut u8,mut _2: char,mut _3: (bool, i32, bool),mut _4: i32,mut _5: f32,mut _6: f64) -> [u8; 3] {
mir! {
type RET = [u8; 3];
let _7: Adt63;
let _8: u16;
let _9: u128;
let _10: [char; 2];
let _11: isize;
let _12: *mut u16;
let _13: Adt50;
let _14: u64;
let _15: ((char, i32), isize, u64, bool);
let _16: bool;
let _17: u64;
let _18: isize;
let _19: [isize; 5];
let _20: f64;
let _21: Adt50;
let _22: isize;
let _23: [u8; 3];
let _24: Adt50;
let _25: isize;
let _26: char;
let _27: Adt63;
let _28: isize;
let _29: isize;
let _30: [u32; 6];
let _31: i32;
let _32: ();
let _33: ();
{
_5 = 215_u8 as f32;
_6 = 2182448272_u32 as f64;
RET = [155_u8,5_u8,176_u8];
_4 = _3.1;
_4 = 28590_u16 as i32;
Goto(bb1)
}
bb1 = {
_1 = core::ptr::addr_of_mut!((*_1));
_4 = _2 as i32;
_3 = (true, _4, false);
RET = [170_u8,106_u8,148_u8];
_2 = '\u{93520}';
_4 = !_3.1;
_1 = core::ptr::addr_of_mut!((*_1));
_1 = core::ptr::addr_of_mut!((*_1));
_3.0 = !_3.2;
_3.2 = !_3.0;
RET = [112_u8,178_u8,63_u8];
_3 = (false, _4, true);
_3.0 = _3.2;
_4 = !_3.1;
_3.2 = !_3.0;
_3 = (false, _4, true);
_4 = -_3.1;
_4 = _3.1 ^ _3.1;
_5 = _4 as f32;
_4 = -_3.1;
_3 = (true, _4, true);
_5 = 10465308622870124047_u64 as f32;
_6 = (-126_i8) as f64;
_1 = core::ptr::addr_of_mut!((*_1));
_5 = 120_u8 as f32;
Goto(bb2)
}
bb2 = {
_1 = core::ptr::addr_of_mut!((*_1));
RET = [248_u8,252_u8,49_u8];
_3 = (true, _4, true);
_6 = 3426419560_u32 as f64;
_1 = core::ptr::addr_of_mut!((*_1));
_5 = (-19_i8) as f32;
_3.2 = _3.0 > _3.0;
_3.0 = !_3.2;
_6 = (-103_i8) as f64;
_6 = 4991172327734541500_u64 as f64;
_3.2 = !_3.0;
_3.2 = _5 == _5;
_3.0 = _3.2 ^ _3.2;
_4 = (-9223372036854775808_isize) as i32;
_5 = 9223372036854775807_isize as f32;
RET = [3_u8,142_u8,228_u8];
_7.fld0 = [65_i8,38_i8,63_i8];
_1 = core::ptr::addr_of_mut!((*_1));
_3.2 = _3.0;
_3.2 = _3.0;
_2 = '\u{846d7}';
_4 = _3.1;
_8 = 7741375364718494169_u64 as u16;
RET = [121_u8,45_u8,59_u8];
RET = [234_u8,73_u8,0_u8];
_9 = 810764938559758843783876672761735567_u128;
_6 = 2239191248154332119_i64 as f64;
Goto(bb3)
}
bb3 = {
_6 = 93_u8 as f64;
_8 = 16386_u16 ^ 1990_u16;
_9 = 255247456692466046707601807648067294623_u128 + 55729885759702807172600102786047410839_u128;
_9 = 169_u8 as u128;
_3.1 = _4;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = _9 as i32;
Goto(bb4)
}
bb4 = {
_9 = 169642714990842698333861287198409913333_u128 | 302723026600831893620032767303239783814_u128;
_10 = [_2,_2];
Goto(bb5)
}
bb5 = {
_1 = core::ptr::addr_of_mut!((*_1));
_3 = (true, _4, true);
_7.fld0 = [(-55_i8),(-87_i8),(-113_i8)];
Goto(bb6)
}
bb6 = {
_9 = 327257939342601561429732742566456363804_u128 % 53150029322595358426232668807370050842_u128;
RET = [155_u8,126_u8,138_u8];
_11 = !(-9223372036854775808_isize);
RET = [252_u8,255_u8,219_u8];
_3.0 = !_3.2;
_12 = core::ptr::addr_of_mut!(_8);
RET = [218_u8,164_u8,150_u8];
_11 = _6 as isize;
_11 = !(-21_isize);
_7.fld0 = [37_i8,7_i8,(-61_i8)];
_13.fld1 = _2;
_11 = !9223372036854775807_isize;
(*_12) = 42657_u16 - 42349_u16;
_11 = 7159676156494001403_u64 as isize;
_13.fld4.1.0 = _13.fld1;
_11 = -(-9223372036854775808_isize);
_13.fld3 = core::ptr::addr_of!(_13.fld4.3);
(*_1) = core::ptr::addr_of_mut!(_13.fld5);
_15.0.0 = _13.fld4.1.0;
_4 = _9 as i32;
_3 = (true, _4, false);
_13.fld4.4 = (-2235209138105570873_i64) as isize;
Goto(bb7)
}
bb7 = {
_13.fld5 = 45_u8 << (*_12);
_11 = -_13.fld4.4;
_10 = [_13.fld1,_13.fld1];
_13.fld0.0 = (-77_i8) - (-94_i8);
_10 = [_15.0.0,_15.0.0];
_13.fld4.0.0 = _3.2 ^ _3.2;
_13.fld4.3 = _13.fld0.0 as u32;
_13.fld4.0 = (_3.2, _3.1, _3.0);
_2 = _13.fld1;
_13.fld2 = !7010203988767349869_u64;
Call(_13.fld5 = core::intrinsics::bswap(81_u8), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_6 = _13.fld4.3 as f64;
_3.1 = _4 >> _9;
_13.fld1 = _13.fld4.1.0;
_15.2 = _13.fld2 << _13.fld4.4;
(*_1) = core::ptr::addr_of_mut!(_13.fld5);
(*_1) = core::ptr::addr_of_mut!(_13.fld5);
Goto(bb9)
}
bb9 = {
_3.2 = _13.fld4.0.2 | _3.0;
Goto(bb10)
}
bb10 = {
_3 = _13.fld4.0;
Goto(bb11)
}
bb11 = {
_19 = [_13.fld4.4,_11,_13.fld4.4,_13.fld4.4,_11];
_15.3 = _13.fld4.3 != _13.fld4.3;
_13.fld4.0.1 = _4;
_18 = _13.fld4.4 + _11;
_16 = _15.3;
Goto(bb12)
}
bb12 = {
_3.2 = _16;
_3.0 = _13.fld2 != _13.fld2;
_13.fld1 = _15.0.0;
_13.fld4.0.2 = !_3.0;
_18 = !_13.fld4.4;
_21.fld0 = _13.fld0;
_21.fld4.4 = _13.fld4.4 & _18;
_7.fld0 = [_13.fld0.0,_13.fld0.0,_21.fld0.0];
_23 = [_13.fld5,_13.fld5,_13.fld5];
_21.fld3 = _13.fld3;
_7.fld0 = [_21.fld0.0,_13.fld0.0,_13.fld0.0];
_21.fld4.1.1 = _4;
_21.fld4.1 = (_13.fld1, _13.fld4.0.1);
_17 = !_15.2;
_15.2 = _13.fld2;
_15.0 = _21.fld4.1;
_21.fld4 = (_3, _15.0, _5, _13.fld4.3, _13.fld4.4);
_24 = Adt50 { fld0: _21.fld0,fld1: _2,fld2: _17,fld3: _13.fld3,fld4: _21.fld4,fld5: _13.fld5 };
_15.1 = -_13.fld4.4;
_16 = _13.fld4.0.2 | _21.fld4.0.2;
(*_12) = 7900_u16 % 49341_u16;
_21.fld1 = _24.fld4.1.0;
Call(_24.fld4.0.1 = core::intrinsics::transmute(_21.fld4.0.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_21 = Adt50 { fld0: _24.fld0,fld1: _2,fld2: _13.fld2,fld3: _24.fld3,fld4: _24.fld4,fld5: _13.fld5 };
_3.1 = !_13.fld4.0.1;
_16 = !_21.fld4.0.0;
_21.fld4.3 = _24.fld4.3 + _24.fld4.3;
(*_1) = core::ptr::addr_of_mut!(_21.fld5);
_24.fld5 = _13.fld4.4 as u8;
_21.fld5 = _24.fld5 ^ _13.fld5;
_7.fld0 = [_21.fld0.0,_21.fld0.0,_24.fld0.0];
_24.fld4 = (_21.fld4.0, _21.fld4.1, _21.fld4.2, _21.fld4.3, _18);
_27.fld0 = _7.fld0;
_21.fld4.0.0 = _16;
_15.0 = (_21.fld1, _21.fld4.0.1);
_11 = !_18;
Goto(bb14)
}
bb14 = {
_13.fld4.1 = (_2, _24.fld4.1.1);
_14 = _24.fld2;
_13.fld4.0.1 = 29429_i16 as i32;
_21.fld0.0 = _24.fld0.0;
_15.0.0 = _24.fld1;
_24.fld1 = _2;
(*_12) = _24.fld0.0 as u16;
_7 = _27;
_24.fld4.1 = _15.0;
_21.fld4.0.0 = _24.fld4.0.2;
_13.fld4.0 = (_21.fld4.0.0, _24.fld4.1.1, _21.fld4.0.2);
_15.3 = !_13.fld4.0.2;
_21.fld4.0 = (_24.fld4.0.2, _24.fld4.0.1, _3.2);
_21 = Adt50 { fld0: _13.fld0,fld1: _13.fld4.1.0,fld2: _17,fld3: _13.fld3,fld4: _24.fld4,fld5: _13.fld5 };
_13.fld1 = _15.0.0;
(*_1) = core::ptr::addr_of_mut!(_24.fld5);
_13.fld4 = (_3, _24.fld4.1, _24.fld4.2, _24.fld4.3, _15.1);
_21.fld4.1 = (_24.fld1, _3.1);
_4 = !_24.fld4.0.1;
_13.fld4.2 = -_21.fld4.2;
_13.fld4.3 = !_21.fld4.3;
_24.fld4.4 = _9 as isize;
_15.0.0 = _24.fld1;
_28 = _15.1 - _11;
_27.fld0 = _7.fld0;
_7 = Adt63 { fld0: _27.fld0 };
_21.fld0.0 = _24.fld0.0;
_13.fld4.0.1 = _24.fld4.0.1 * _21.fld4.1.1;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(17_usize, 18_usize, Move(_18), 15_usize, Move(_15), 23_usize, Move(_23), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(17_usize, 2_usize, Move(_2), 11_usize, Move(_11), 9_usize, Move(_9), 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(17227793378774564005_usize), std::hint::black_box('\u{3ba21}'), std::hint::black_box(7361659258191466005640002080115946669_i128), std::hint::black_box(1093817233_u32), std::hint::black_box((-23211_i16)), std::hint::black_box(18251496892304154514_u64), std::hint::black_box((-2022559959663462981_i64)));
                
            }
#[derive(Debug)]
pub struct Adt50 {
fld0: (i8,),
fld1: char,
fld2: u64,
fld3: *const u32,
fld4: ((bool, i32, bool), (char, i32), f32, u32, isize),
fld5: u8,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt51 {
fld0: u16,
fld1: (*mut u8, char, [i8; 3]),
fld2: [isize; 5],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt52 {
fld0: ((bool, i32, bool), (char, i32), f32, u32, isize),
fld1: [char; 2],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt53 {
fld0: (*mut u8, char, [i8; 3]),
fld1: f64,
fld2: (u32, u16, i32),
fld3: u64,
fld4: [i8; 3],
fld5: [u16; 8],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt54 {
fld0: f64,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt55 {
fld0: bool,
fld1: Adt52,
fld2: u128,
fld3: *mut char,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt56 {
fld0: *mut *mut char,
fld1: [i16; 8],
fld2: ([u32; 1], i32, bool),
}
#[derive(Debug)]
pub struct Adt57 {
fld0: (bool, i32, bool),
}
#[derive(Debug)]
pub struct Adt58 {
fld0: u64,
}
#[derive(Debug)]
pub struct Adt59 {
fld0: [u32; 6],
fld1: ((char, i32), *mut char, *mut u8, [char; 2], (char, i32)),
fld2: *mut *mut u8,
fld3: u8,
}
#[derive(Debug)]
pub struct Adt60 {
fld0: *const [u32; 6],
fld1: (*mut u8, char, [i8; 3]),
fld2: Adt50,
fld3: Adt52,
fld4: Adt51,
fld5: (u32, u16, i32),
fld6: ((char, i32), isize, u64, bool),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt61 {
fld0: bool,
fld1: (i64, u32, i128, i16),
fld2: *mut *mut u8,
fld3: [i16; 8],
fld4: Adt51,
fld5: usize,
fld6: (u8, char, [char; 2], (i64, u32, i128, i16), [char; 2]),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt62 {
fld0: *mut *mut u8,
fld1: Adt51,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt63 {
fld0: [i8; 3],
}
#[derive(Debug)]
pub struct Adt64 {
fld0: f64,
fld1: Adt55,
fld2: ((char, i32), *mut char, *mut u8, [char; 2], (char, i32)),
fld3: Adt50,
fld4: (*mut u8, char, [i8; 3]),
fld5: *const u32,
fld6: i64,
fld7: [u8; 3],
}
#[derive(Debug)]
pub struct Adt65 {
fld0: *mut f32,
fld1: *mut u8,
fld2: isize,
fld3: i8,
fld4: [u32; 1],
fld5: *const u32,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt66 {
fld0: Adt54,
fld1: [i8; 3],
fld2: u64,
fld3: (char, i32),
fld4: *mut bool,
fld5: ((bool, i32, bool), (char, i32), f32, u32, isize),
fld6: *mut *mut u8,
fld7: ((char, i32), *mut char, *mut u8, [char; 2], (char, i32)),
}

