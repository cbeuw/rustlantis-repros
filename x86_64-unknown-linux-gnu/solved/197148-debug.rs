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
pub fn fn0(mut _1: bool,mut _2: u128,mut _3: i32,mut _4: i8,mut _5: u32) -> u8 {
mir! {
type RET = u8;
let _6: u16;
let _7: *const *const *mut u16;
let _8: i8;
let _9: char;
let _10: isize;
let _11: [i16; 3];
let _12: f32;
let _13: (*const i16, i32, *const char);
let _14: isize;
let _15: *const *const *mut u16;
let _16: isize;
let _17: isize;
let _18: isize;
let _19: u64;
let _20: (bool, i8, [i32; 5]);
let _21: Adt62;
let _22: Adt54;
let _23: (usize, u8);
let _24: *const i16;
let _25: isize;
let _26: [i32; 1];
let _27: [i16; 2];
let _28: (usize, u8);
let _29: [i32; 5];
let _30: isize;
let _31: (usize, u8);
let _32: Adt48;
let _33: i64;
let _34: ();
let _35: ();
{
_4 = -10_i8;
_1 = false;
_1 = true;
_3 = (-74421160_i32);
_5 = 3831304897_u32;
RET = (-9223372036854775808_isize) as u8;
RET = 130_u8 | 126_u8;
_5 = 1717241920_u32;
_2 = !163256209042320788828647813474578471588_u128;
_2 = 158890616698523873356769477005832540422_u128 ^ 206789255545445320014730649267901532688_u128;
_2 = 36523432142181487897856825044353072368_u128;
RET = 161_u8;
_3 = (-37597886871001714312313343116927929527_i128) as i32;
_6 = 25620_u16 ^ 32706_u16;
_2 = 46884922140721393901254818045342236180_u128;
RET = !165_u8;
_2 = 262309274766464231523551821634753990236_u128 | 147369966280341857832069320107117756717_u128;
_6 = 55118_u16;
_3 = _6 as i32;
_2 = 29207975780031171893800457048268052017_u128;
RET = 175_u8 << _4;
RET = !99_u8;
_1 = RET <= RET;
Call(_7 = fn1(_2, _1, _3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = 3844_u16 + 1814_u16;
RET = 27_u8 & 37_u8;
_1 = true;
RET = 153_u8;
_6 = !30919_u16;
_9 = '\u{dc9b3}';
_8 = _4 * _4;
RET = _6 as u8;
_8 = -_4;
RET = 4753860831330775778_i64 as u8;
_9 = '\u{a6765}';
RET = 43_u8;
_11 = [(-32294_i16),18935_i16,(-3853_i16)];
_1 = _2 == _2;
_11 = [(-31906_i16),17419_i16,(-28458_i16)];
RET = _5 as u8;
_9 = '\u{8b96d}';
_10 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_13.2 = core::ptr::addr_of!(_9);
_6 = !33518_u16;
_5 = 1235_i16 as u32;
_13.1 = _2 as i32;
RET = 18_u8;
_10 = 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
_5 = RET as u32;
_9 = '\u{d3326}';
_10 = (-9223372036854775808_isize) | (-25_isize);
_15 = _7;
_1 = _5 == _5;
_4 = !_8;
_8 = -_4;
_12 = (-146636763126714230702337148341537095076_i128) as f32;
RET = 8928318052448396388_u64 as u8;
_14 = 5387648511165270478_u64 as isize;
_13.1 = _6 as i32;
RET = _8 as u8;
_3 = _13.1 * _13.1;
_8 = _14 as i8;
_16 = _2 as isize;
RET = 88_u8;
RET = !203_u8;
RET = !226_u8;
_9 = '\u{fa008}';
Goto(bb3)
}
bb3 = {
_14 = -_16;
_12 = 92705807244540406246311324741087443044_i128 as f32;
_6 = 13739_u16 & 23918_u16;
_7 = _15;
_15 = _7;
_6 = 12681_u16 | 30224_u16;
_9 = '\u{f2c16}';
_17 = _6 as isize;
_21.fld2 = (1_usize, RET);
_12 = _4 as f32;
_20.2 = [_3,_3,_3,_13.1,_3];
RET = _21.fld2.1;
_13.2 = core::ptr::addr_of!(_9);
_20.2 = [_13.1,_3,_13.1,_3,_3];
match _21.fld2.0 {
1 => bb4,
_ => bb2
}
}
bb4 = {
_13.1 = _9 as i32;
_15 = _7;
_12 = _17 as f32;
_21.fld3.2 = core::ptr::addr_of_mut!(_6);
_13.2 = core::ptr::addr_of!(_21.fld1);
_18 = 6307825616990790911450472394851799877_i128 as isize;
_21.fld3.1 = 20352_i16 as f32;
_4 = _2 as i8;
_14 = _17 + _10;
_21.fld0 = !_1;
_7 = _15;
_5 = 3087016222_u32 * 2610068758_u32;
_23.1 = _9 as u8;
_21.fld2.0 = 7_usize;
_18 = _17 * _14;
Goto(bb5)
}
bb5 = {
_1 = !_21.fld0;
_20.0 = _21.fld0 & _21.fld0;
_21.fld3.0 = _4 << _6;
_8 = _21.fld3.0;
_18 = _17;
_11 = [(-7950_i16),(-23564_i16),5965_i16];
_5 = _21.fld3.1 as u32;
_1 = _21.fld0;
_2 = 111311969190190699489272520726621987542_u128 ^ 134382409804741053354042910198850677617_u128;
_19 = 2717517490218286973_u64;
_17 = _1 as isize;
RET = _23.1 | _21.fld2.1;
_23.0 = _14 as usize;
_23.0 = _21.fld2.0 - _21.fld2.0;
_5 = 1712905470_u32;
_21.fld1 = _9;
_23 = _21.fld2;
_18 = !_14;
_18 = _17;
_20.2 = [_3,_3,_13.1,_13.1,_3];
match _23.0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
7 => bb11,
_ => bb10
}
}
bb6 = {
_13.1 = _9 as i32;
_15 = _7;
_12 = _17 as f32;
_21.fld3.2 = core::ptr::addr_of_mut!(_6);
_13.2 = core::ptr::addr_of!(_21.fld1);
_18 = 6307825616990790911450472394851799877_i128 as isize;
_21.fld3.1 = 20352_i16 as f32;
_4 = _2 as i8;
_14 = _17 + _10;
_21.fld0 = !_1;
_7 = _15;
_5 = 3087016222_u32 * 2610068758_u32;
_23.1 = _9 as u8;
_21.fld2.0 = 7_usize;
_18 = _17 * _14;
Goto(bb5)
}
bb7 = {
_14 = -_16;
_12 = 92705807244540406246311324741087443044_i128 as f32;
_6 = 13739_u16 & 23918_u16;
_7 = _15;
_15 = _7;
_6 = 12681_u16 | 30224_u16;
_9 = '\u{f2c16}';
_17 = _6 as isize;
_21.fld2 = (1_usize, RET);
_12 = _4 as f32;
_20.2 = [_3,_3,_3,_13.1,_3];
RET = _21.fld2.1;
_13.2 = core::ptr::addr_of!(_9);
_20.2 = [_13.1,_3,_13.1,_3,_3];
match _21.fld2.0 {
1 => bb4,
_ => bb2
}
}
bb8 = {
_5 = RET as u32;
_9 = '\u{d3326}';
_10 = (-9223372036854775808_isize) | (-25_isize);
_15 = _7;
_1 = _5 == _5;
_4 = !_8;
_8 = -_4;
_12 = (-146636763126714230702337148341537095076_i128) as f32;
RET = 8928318052448396388_u64 as u8;
_14 = 5387648511165270478_u64 as isize;
_13.1 = _6 as i32;
RET = _8 as u8;
_3 = _13.1 * _13.1;
_8 = _14 as i8;
_16 = _2 as isize;
RET = 88_u8;
RET = !203_u8;
RET = !226_u8;
_9 = '\u{fa008}';
Goto(bb3)
}
bb9 = {
_6 = 3844_u16 + 1814_u16;
RET = 27_u8 & 37_u8;
_1 = true;
RET = 153_u8;
_6 = !30919_u16;
_9 = '\u{dc9b3}';
_8 = _4 * _4;
RET = _6 as u8;
_8 = -_4;
RET = 4753860831330775778_i64 as u8;
_9 = '\u{a6765}';
RET = 43_u8;
_11 = [(-32294_i16),18935_i16,(-3853_i16)];
_1 = _2 == _2;
_11 = [(-31906_i16),17419_i16,(-28458_i16)];
RET = _5 as u8;
_9 = '\u{8b96d}';
_10 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_13.2 = core::ptr::addr_of!(_9);
_6 = !33518_u16;
_5 = 1235_i16 as u32;
_13.1 = _2 as i32;
RET = 18_u8;
_10 = 9223372036854775807_isize;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_28.1 = RET;
_20.2 = [_3,_3,_13.1,_3,_13.1];
_16 = !_14;
_27 = [(-29104_i16),25014_i16];
_13.1 = _3;
_7 = _15;
_28 = (_23.0, RET);
_21.fld0 = _28.1 <= _21.fld2.1;
_28.0 = _23.0 + _21.fld2.0;
_23.1 = _6 as u8;
_27 = [(-18630_i16),14389_i16];
_17 = 119686611840358042198088094707693926867_i128 as isize;
_2 = !38795004501580430702404587874877782816_u128;
_20.0 = !_1;
_13.2 = core::ptr::addr_of!(_9);
_21.fld1 = _9;
_20.2 = [_3,_13.1,_13.1,_3,_13.1];
match _21.fld2.0 {
0 => bb8,
1 => bb12,
7 => bb14,
_ => bb13
}
}
bb12 = {
_5 = RET as u32;
_9 = '\u{d3326}';
_10 = (-9223372036854775808_isize) | (-25_isize);
_15 = _7;
_1 = _5 == _5;
_4 = !_8;
_8 = -_4;
_12 = (-146636763126714230702337148341537095076_i128) as f32;
RET = 8928318052448396388_u64 as u8;
_14 = 5387648511165270478_u64 as isize;
_13.1 = _6 as i32;
RET = _8 as u8;
_3 = _13.1 * _13.1;
_8 = _14 as i8;
_16 = _2 as isize;
RET = 88_u8;
RET = !203_u8;
RET = !226_u8;
_9 = '\u{fa008}';
Goto(bb3)
}
bb13 = {
_13.1 = _9 as i32;
_15 = _7;
_12 = _17 as f32;
_21.fld3.2 = core::ptr::addr_of_mut!(_6);
_13.2 = core::ptr::addr_of!(_21.fld1);
_18 = 6307825616990790911450472394851799877_i128 as isize;
_21.fld3.1 = 20352_i16 as f32;
_4 = _2 as i8;
_14 = _17 + _10;
_21.fld0 = !_1;
_7 = _15;
_5 = 3087016222_u32 * 2610068758_u32;
_23.1 = _9 as u8;
_21.fld2.0 = 7_usize;
_18 = _17 * _14;
Goto(bb5)
}
bb14 = {
RET = _2 as u8;
_8 = _21.fld3.0;
_8 = 31070_i16 as i8;
_2 = !124268358273477215555474312292894934_u128;
_3 = -_13.1;
_26 = [_13.1];
RET = !_28.1;
_27 = [1960_i16,22589_i16];
_13.2 = core::ptr::addr_of!(_21.fld1);
_23.1 = _1 as u8;
_9 = _21.fld1;
_14 = -_16;
_14 = _6 as isize;
_30 = _16;
_31 = (_21.fld2.0, _28.1);
_18 = _9 as isize;
_12 = _5 as f32;
RET = 51560385233632438176104584165957930128_i128 as u8;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(0_usize, 18_usize, Move(_18), 6_usize, Move(_6), 28_usize, Move(_28), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(0_usize, 4_usize, Move(_4), 10_usize, Move(_10), 11_usize, Move(_11), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(0_usize, 5_usize, Move(_5), 23_usize, Move(_23), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u128,mut _2: bool,mut _3: i32,mut _4: bool) -> *const *const *mut u16 {
mir! {
type RET = *const *const *mut u16;
let _5: bool;
let _6: *const i16;
let _7: Adt61;
let _8: [i32; 1];
let _9: bool;
let _10: [i32; 1];
let _11: ([i32; 1], (u8,));
let _12: [bool; 7];
let _13: [u64; 2];
let _14: Adt61;
let _15: Adt54;
let _16: isize;
let _17: ([i32; 1], (u8,));
let _18: [i32; 5];
let _19: bool;
let _20: ((*mut u16, u32, char), *const char, *const char);
let _21: char;
let _22: u16;
let _23: Adt56;
let _24: i16;
let _25: (usize, u8);
let _26: bool;
let _27: isize;
let _28: [i64; 4];
let _29: [i32; 1];
let _30: i64;
let _31: u8;
let _32: *mut u16;
let _33: (bool, i8, [i32; 5]);
let _34: f64;
let _35: i128;
let _36: i32;
let _37: [i64; 4];
let _38: (*const i16, i32, *const char);
let _39: isize;
let _40: Adt56;
let _41: f64;
let _42: isize;
let _43: *mut (*mut u16, u32, char);
let _44: u16;
let _45: char;
let _46: [i32; 1];
let _47: f32;
let _48: ([i32; 1], (u8,));
let _49: *const *const *mut u16;
let _50: [bool; 7];
let _51: usize;
let _52: i64;
let _53: *mut *mut u16;
let _54: Adt55;
let _55: isize;
let _56: u32;
let _57: (bool, i8, [i32; 5]);
let _58: (usize, u8);
let _59: f64;
let _60: i16;
let _61: f32;
let _62: [u64; 2];
let _63: Adt55;
let _64: Adt50;
let _65: i32;
let _66: (u8,);
let _67: f64;
let _68: u32;
let _69: bool;
let _70: i64;
let _71: *const i16;
let _72: *mut (i8, f32, *mut u16);
let _73: ([i32; 1], (u8,));
let _74: [u64; 2];
let _75: Adt55;
let _76: f32;
let _77: i64;
let _78: bool;
let _79: *const i16;
let _80: Adt55;
let _81: [i32; 1];
let _82: i128;
let _83: (usize, u8);
let _84: f32;
let _85: f32;
let _86: i32;
let _87: Adt60;
let _88: u8;
let _89: ();
let _90: ();
{
_4 = _2 | _2;
_3 = (-1981516841_i32) - 276495925_i32;
_5 = _4;
_5 = _4 >= _4;
_3 = 5343_i16 as i32;
_4 = _5 == _5;
_2 = !_4;
_4 = _5;
Call(_3 = fn2(_1, _2, _5, _4, _4, _5, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 604646520_i32 << _1;
_5 = _2;
_2 = _5;
_1 = 87115784655464122157960458798123680230_u128 | 190942851609882953334175992722175929136_u128;
_4 = _5 != _5;
_4 = _5;
_3 = '\u{e1a74}' as i32;
_4 = _2 | _2;
_4 = _1 > _1;
_1 = _5 as u128;
_3 = (-1586655455_i32) >> _1;
Goto(bb2)
}
bb2 = {
_2 = !_5;
_5 = _3 != _3;
_4 = !_5;
_2 = _5 >= _4;
_5 = _4;
_4 = _5 >= _2;
_1 = 36_isize as u128;
_2 = !_4;
_5 = !_4;
_4 = _5;
_5 = !_4;
Goto(bb3)
}
bb3 = {
_3 = !1288808170_i32;
_3 = (-10915_i16) as i32;
_1 = 114939945086498336529905510911803887264_u128;
_2 = !_4;
_3 = !798169780_i32;
_5 = _2;
_4 = _2;
_2 = _5;
match _1 {
0 => bb1,
114939945086498336529905510911803887264 => bb4,
_ => bb2
}
}
bb4 = {
_5 = _4 == _2;
_2 = _5 <= _5;
_2 = _4 < _5;
_3 = (-831600739_i32);
_2 = _4 < _5;
_2 = _5 >= _4;
_3 = !(-1539691664_i32);
_3 = 43598_u16 as i32;
_1 = 300775332954312854757017358922393289371_u128 | 149807933514341258777198815671459733723_u128;
_3 = (-65075430_i32) ^ 1031997563_i32;
_8 = [_3];
_4 = _5;
_4 = _2;
_2 = !_4;
_11.0 = [_3];
_11.0 = [_3];
_2 = !_4;
_13 = [5345812446427386867_u64,4963979866890805833_u64];
_4 = _5 >= _2;
_9 = !_2;
_4 = !_9;
_5 = _2 > _9;
Call(_9 = fn3(_5, _5, _2, _4, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5 = _2;
_12 = [_5,_9,_4,_2,_5,_9,_2];
_11.1.0 = 23_u8;
_16 = 54_isize & (-112_isize);
_11.0 = [_3];
_1 = 250026487124657030484553651598141698327_u128;
_12 = [_2,_9,_4,_9,_2,_9,_9];
_17.1 = (_11.1.0,);
_15 = Adt54::Variant1 { fld0: _11,fld1: 36900_u16 };
place!(Field::<u16>(Variant(_15, 1), 1)) = 30140_u16;
_1 = 104304767048077307924003256850739323007_u128 << Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0;
Goto(bb6)
}
bb6 = {
_10 = [_3];
_10 = _8;
_17.1.0 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0 & Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0;
place!(Field::<u16>(Variant(_15, 1), 1)) = !51493_u16;
_9 = _2;
_17.1.0 = _2 as u8;
_18 = [_3,_3,_3,_3,_3];
_16 = -24_isize;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_10, _17.1);
_15 = Adt54::Variant1 { fld0: _11,fld1: 5148_u16 };
_9 = !_2;
_20.0.2 = '\u{75952}';
_20.1 = core::ptr::addr_of!(_21);
_20.2 = core::ptr::addr_of!(_20.0.2);
_12 = [_2,_9,_4,_4,_4,_5,_5];
Goto(bb7)
}
bb7 = {
_9 = _2 == _5;
_21 = _20.0.2;
_10 = [_3];
_18 = [_3,_3,_3,_3,_3];
_19 = !_9;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = (_17.1.0,);
_20.0.1 = !1346708061_u32;
_22 = 13702_u16 * 52519_u16;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = _17.1;
_13 = [5423841934028015328_u64,3758226589480530426_u64];
_17.0 = [_3];
_17.1.0 = _5 as u8;
_20.2 = _20.1;
_16 = !(-9223372036854775808_isize);
_10 = [_3];
place!(Field::<u16>(Variant(_15, 1), 1)) = !_22;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = (_17.1.0,);
Call(_20.0.2 = fn4(_5, Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1, Move(_15), _2, _9, _5, _19, _5, _2, _2, _5, _17.1.0, _17.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17.1.0 = !_11.1.0;
_20.0.0 = core::ptr::addr_of_mut!(_22);
_20.2 = _20.1;
_2 = _9 ^ _9;
_20.0.0 = core::ptr::addr_of_mut!(_22);
_18 = [_3,_3,_3,_3,_3];
_6 = core::ptr::addr_of!(_24);
_11.1.0 = _22 as u8;
_10 = [_3];
_20.2 = core::ptr::addr_of!(_21);
_26 = _5;
_20.0.0 = core::ptr::addr_of_mut!(_22);
_15 = Adt54::Variant1 { fld0: _17,fld1: _22 };
_11.1 = (_17.1.0,);
_16 = 103_isize;
_28 = [3450221660525988740_i64,8203693335533469906_i64,(-7135638343224772499_i64),(-2819483327901208605_i64)];
_20.0.1 = 1541588361_u32;
_25.1 = 3185078921049925393_i64 as u8;
_4 = !_19;
_27 = _16;
_30 = -2768032571867704360_i64;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1.0 = !_11.1.0;
_6 = core::ptr::addr_of!((*_6));
_17 = (Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).0, Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1);
Goto(bb9)
}
bb9 = {
_31 = _25.1;
SetDiscriminant(_15, 1);
Goto(bb10)
}
bb10 = {
place!(Field::<u16>(Variant(_15, 1), 1)) = _22 & _22;
_4 = _26;
_30 = -(-6705220539573975889_i64);
_25.0 = _1 as usize;
_33 = (_19, 4_i8, _18);
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).0 = _8;
_24 = !303_i16;
_31 = _17.1.0 - _25.1;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_17.0, _17.1);
match _33.1 {
4 => bb12,
_ => bb11
}
}
bb11 = {
_10 = [_3];
_10 = _8;
_17.1.0 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0 & Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0;
place!(Field::<u16>(Variant(_15, 1), 1)) = !51493_u16;
_9 = _2;
_17.1.0 = _2 as u8;
_18 = [_3,_3,_3,_3,_3];
_16 = -24_isize;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_10, _17.1);
_15 = Adt54::Variant1 { fld0: _11,fld1: 5148_u16 };
_9 = !_2;
_20.0.2 = '\u{75952}';
_20.1 = core::ptr::addr_of!(_21);
_20.2 = core::ptr::addr_of!(_20.0.2);
_12 = [_2,_9,_4,_4,_4,_5,_5];
Goto(bb7)
}
bb12 = {
_20.1 = _20.2;
Goto(bb13)
}
bb13 = {
_7 = Adt61::Variant1 { fld0: _6 };
_14 = Move(_7);
_17.1.0 = _31 << _33.1;
_25.0 = 3_usize;
_9 = _33.1 < _33.1;
Call(_20.2 = fn19(_17.1.0, _33.1, _9, _33.1, _9, _26, _33.1, _17.1, _17.1.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_5 = !_26;
_11 = (_17.0, _17.1);
_17.1 = _11.1;
_4 = _33.0;
_7 = Adt61::Variant1 { fld0: Field::<*const i16>(Variant(_14, 1), 0) };
_17.0 = _11.0;
_5 = !_4;
_32 = core::ptr::addr_of_mut!(_22);
_27 = _16 - _16;
_28 = [_30,_30,_30,_30];
_25 = (0_usize, _11.1.0);
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_8, _11.1);
_27 = _17.1.0 as isize;
place!(Field::<*const i16>(Variant(_14, 1), 0)) = _6;
_2 = _19;
_20.2 = _20.1;
_1 = !201838547403881980144944698876485189500_u128;
_11.1.0 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0 * _17.1.0;
Goto(bb15)
}
bb15 = {
Goto(bb16)
}
bb16 = {
_2 = !_4;
_20.0.1 = 3162206307_u32 << _25.1;
_15 = Adt54::Variant1 { fld0: _17,fld1: _22 };
_13 = [14907049803533126755_u64,8856348605277104375_u64];
place!(Field::<*const i16>(Variant(_14, 1), 0)) = _6;
_33.1 = 84_i8;
SetDiscriminant(_15, 1);
_12 = [_19,_9,_26,_26,_19,_33.0,_9];
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).0 = _8;
_38.0 = core::ptr::addr_of!(_24);
_17.1.0 = _1 as u8;
_20.0 = (_32, 520468358_u32, _21);
match _25.0 {
0 => bb19,
_ => bb18
}
}
bb17 = {
_10 = [_3];
_10 = _8;
_17.1.0 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0 & Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0;
place!(Field::<u16>(Variant(_15, 1), 1)) = !51493_u16;
_9 = _2;
_17.1.0 = _2 as u8;
_18 = [_3,_3,_3,_3,_3];
_16 = -24_isize;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_10, _17.1);
_15 = Adt54::Variant1 { fld0: _11,fld1: 5148_u16 };
_9 = !_2;
_20.0.2 = '\u{75952}';
_20.1 = core::ptr::addr_of!(_21);
_20.2 = core::ptr::addr_of!(_20.0.2);
_12 = [_2,_9,_4,_4,_4,_5,_5];
Goto(bb7)
}
bb18 = {
_31 = _25.1;
SetDiscriminant(_15, 1);
Goto(bb10)
}
bb19 = {
_25 = (11056456277193718618_usize, _11.1.0);
_30 = (-3127812933451755537_i64);
_20.0.2 = _21;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = _11.1;
_3 = 261229736_i32 | (-1138393257_i32);
_39 = _27 * _27;
_38.2 = _20.1;
_20.0 = (_32, 3886264773_u32, _21);
_16 = _27 + _27;
_33.1 = (-104_i8) << _27;
_4 = !_26;
match _25.0 {
0 => bb7,
11056456277193718618 => bb21,
_ => bb20
}
}
bb20 = {
_17.1.0 = !_11.1.0;
_20.0.0 = core::ptr::addr_of_mut!(_22);
_20.2 = _20.1;
_2 = _9 ^ _9;
_20.0.0 = core::ptr::addr_of_mut!(_22);
_18 = [_3,_3,_3,_3,_3];
_6 = core::ptr::addr_of!(_24);
_11.1.0 = _22 as u8;
_10 = [_3];
_20.2 = core::ptr::addr_of!(_21);
_26 = _5;
_20.0.0 = core::ptr::addr_of_mut!(_22);
_15 = Adt54::Variant1 { fld0: _17,fld1: _22 };
_11.1 = (_17.1.0,);
_16 = 103_isize;
_28 = [3450221660525988740_i64,8203693335533469906_i64,(-7135638343224772499_i64),(-2819483327901208605_i64)];
_20.0.1 = 1541588361_u32;
_25.1 = 3185078921049925393_i64 as u8;
_4 = !_19;
_27 = _16;
_30 = -2768032571867704360_i64;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1.0 = !_11.1.0;
_6 = core::ptr::addr_of!((*_6));
_17 = (Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).0, Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1);
Goto(bb9)
}
bb21 = {
_36 = _3;
_17.1.0 = _33.1 as u8;
Call(place!(Field::<u16>(Variant(_15, 1), 1)) = core::intrinsics::transmute((*_6)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_36 = _3 & _3;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_8, _11.1);
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1.0 = !_17.1.0;
_41 = _22 as f64;
_17.1.0 = _11.1.0;
_20.0 = (_32, 1612159965_u32, _21);
SetDiscriminant(_14, 0);
SetDiscriminant(_15, 1);
_25.0 = !4_usize;
_27 = _39;
_16 = _4 as isize;
_32 = _20.0.0;
place!(Field::<Adt60>(Variant(_14, 0), 0)) = Adt60::Variant3 { fld0: _28,fld1: _21,fld2: _25.0 };
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = (_25.1,);
_33.2 = [_3,_36,_36,_36,_36];
_34 = _41 * _41;
match _20.0.1 {
1612159965 => bb24,
_ => bb23
}
}
bb23 = {
_3 = !1288808170_i32;
_3 = (-10915_i16) as i32;
_1 = 114939945086498336529905510911803887264_u128;
_2 = !_4;
_3 = !798169780_i32;
_5 = _2;
_4 = _2;
_2 = _5;
match _1 {
0 => bb1,
114939945086498336529905510911803887264 => bb4,
_ => bb2
}
}
bb24 = {
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = _17;
_9 = !_4;
_14 = Move(_7);
_12 = [_2,_9,_4,_19,_33.0,_4,_26];
_44 = _22 & _22;
_30 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0 as i64;
_39 = -_16;
_17.0 = [_3];
_34 = _41;
_20.0 = (_32, 1568288974_u32, _21);
_14 = Adt61::Variant1 { fld0: _38.0 };
_17.1 = (_11.1.0,);
_11 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0);
match _20.0.1 {
0 => bb6,
1 => bb8,
2 => bb10,
3 => bb23,
4 => bb25,
1568288974 => bb27,
_ => bb26
}
}
bb25 = {
_31 = _25.1;
SetDiscriminant(_15, 1);
Goto(bb10)
}
bb26 = {
_36 = _3 & _3;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_8, _11.1);
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1.0 = !_17.1.0;
_41 = _22 as f64;
_17.1.0 = _11.1.0;
_20.0 = (_32, 1612159965_u32, _21);
SetDiscriminant(_14, 0);
SetDiscriminant(_15, 1);
_25.0 = !4_usize;
_27 = _39;
_16 = _4 as isize;
_32 = _20.0.0;
place!(Field::<Adt60>(Variant(_14, 0), 0)) = Adt60::Variant3 { fld0: _28,fld1: _21,fld2: _25.0 };
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = (_25.1,);
_33.2 = [_3,_36,_36,_36,_36];
_34 = _41 * _41;
match _20.0.1 {
1612159965 => bb24,
_ => bb23
}
}
bb27 = {
_20.1 = core::ptr::addr_of!(_21);
_17.1.0 = !_11.1.0;
SetDiscriminant(_14, 0);
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).1 = (_11.1.0,);
_17 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0);
_41 = _34;
_43 = core::ptr::addr_of_mut!(_20.0);
_38 = (_6, _36, _20.1);
_29 = [_36];
_4 = _19 & _33.0;
_3 = -_36;
_16 = !_39;
_25 = (4_usize, Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0);
_45 = (*_43).2;
_36 = _3;
match _25.0 {
0 => bb22,
1 => bb14,
2 => bb3,
3 => bb4,
5 => bb6,
6 => bb21,
4 => bb28,
_ => bb15
}
}
bb28 = {
_12 = [_26,_19,_26,_9,_4,_26,_9];
_25 = (5410263972238109871_usize, Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2).1.0);
_17 = (_8, _11.1);
_30 = 8715310034362633276_i64 ^ (-4516174476367612405_i64);
_2 = _26 | _5;
_42 = _39;
_38 = (_6, _3, _20.2);
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)) = _11;
place!(Field::<[bool; 7]>(Variant(_14, 0), 1)) = [_4,_26,_33.0,_2,_5,_9,_19];
_10 = [_36];
_46 = [_36];
_29 = [_38.1];
_11.0 = [_38.1];
_36 = (-13979859212568537575077848805138416626_i128) as i32;
_18 = _33.2;
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).0 = [_38.1];
_11.1.0 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0;
_37 = [_30,_30,_30,_30];
_20.2 = _38.2;
_17.0 = [_38.1];
place!(Field::<[bool; 7]>(Variant(_14, 0), 1)) = [_19,_4,_5,_33.0,_4,_2,_5];
_33 = (_2, 91_i8, _18);
_26 = _9;
_20.0.0 = core::ptr::addr_of_mut!(_44);
_41 = -_34;
_46 = [_38.1];
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).0 = _8;
match _33.1 {
0 => bb15,
1 => bb2,
2 => bb22,
3 => bb19,
4 => bb9,
5 => bb6,
6 => bb29,
91 => bb31,
_ => bb30
}
}
bb29 = {
_3 = !1288808170_i32;
_3 = (-10915_i16) as i32;
_1 = 114939945086498336529905510911803887264_u128;
_2 = !_4;
_3 = !798169780_i32;
_5 = _2;
_4 = _2;
_2 = _5;
match _1 {
0 => bb1,
114939945086498336529905510911803887264 => bb4,
_ => bb2
}
}
bb30 = {
_36 = _3 & _3;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_8, _11.1);
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1.0 = !_17.1.0;
_41 = _22 as f64;
_17.1.0 = _11.1.0;
_20.0 = (_32, 1612159965_u32, _21);
SetDiscriminant(_14, 0);
SetDiscriminant(_15, 1);
_25.0 = !4_usize;
_27 = _39;
_16 = _4 as isize;
_32 = _20.0.0;
place!(Field::<Adt60>(Variant(_14, 0), 0)) = Adt60::Variant3 { fld0: _28,fld1: _21,fld2: _25.0 };
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = (_25.1,);
_33.2 = [_3,_36,_36,_36,_36];
_34 = _41 * _41;
match _20.0.1 {
1612159965 => bb24,
_ => bb23
}
}
bb31 = {
_21 = (*_43).2;
_10 = [_38.1];
Call(_13 = core::intrinsics::transmute(_1), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_38.2 = _20.2;
_21 = (*_43).2;
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).1.0 = _25.1;
_30 = (-6448118955572779175_i64);
_16 = _27 - _42;
_52 = _30;
_20.2 = core::ptr::addr_of!(_20.0.2);
_8 = _46;
_51 = !_25.0;
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).1.0 = _25.1;
_9 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0 <= Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2).1.0;
_28 = [_52,_52,_30,_30];
match _25.0 {
5410263972238109871 => bb33,
_ => bb19
}
}
bb33 = {
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).1 = (_25.1,);
_11.1.0 = _44 as u8;
_20.1 = core::ptr::addr_of!((*_43).2);
_5 = _9 >= _19;
_47 = _38.1 as f32;
_35 = 65787795570666768900719716454935141536_i128 << _51;
_20.1 = core::ptr::addr_of!(_21);
_48.0 = [_3];
_17.1 = (_25.1,);
_54.fld1.fld2 = (_25.1,);
_32 = _20.0.0;
_14 = Adt61::Variant1 { fld0: _38.0 };
SetDiscriminant(_14, 0);
_24 = 27213_i16 ^ (-27801_i16);
_31 = _54.fld1.fld2.0;
_39 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1.0 as isize;
_31 = !_25.1;
_48 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0);
_54.fld3 = _31 + _17.1.0;
_29 = [_38.1];
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).0 = [_38.1];
_38.2 = core::ptr::addr_of!((*_43).2);
_36 = _38.1 - _3;
_33 = (_5, 53_i8, _18);
Call(_22 = core::intrinsics::transmute(_44), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_25 = (_51, _54.fld3);
_15 = Adt54::Variant1 { fld0: _48,fld1: _22 };
place!(Field::<Adt60>(Variant(_14, 0), 0)) = Adt60::Variant1 { fld0: _28,fld1: Move(_15),fld2: _42 };
_16 = _35 as isize;
Goto(bb35)
}
bb35 = {
_11.1.0 = !_17.1.0;
_54.fld2 = _38;
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).0 = [_54.fld2.1];
_50 = [_19,_5,_26,_2,_4,_19,_5];
_54.fld1.fld4 = _11.0;
_33 = (_26, (-124_i8), _18);
match _33.1 {
0 => bb7,
1 => bb2,
2 => bb31,
3 => bb4,
4 => bb6,
5 => bb36,
340282366920938463463374607431768211332 => bb38,
_ => bb37
}
}
bb36 = {
_3 = !1288808170_i32;
_3 = (-10915_i16) as i32;
_1 = 114939945086498336529905510911803887264_u128;
_2 = !_4;
_3 = !798169780_i32;
_5 = _2;
_4 = _2;
_2 = _5;
match _1 {
0 => bb1,
114939945086498336529905510911803887264 => bb4,
_ => bb2
}
}
bb37 = {
_36 = _3 & _3;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = (_8, _11.1);
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1.0 = !_17.1.0;
_41 = _22 as f64;
_17.1.0 = _11.1.0;
_20.0 = (_32, 1612159965_u32, _21);
SetDiscriminant(_14, 0);
SetDiscriminant(_15, 1);
_25.0 = !4_usize;
_27 = _39;
_16 = _4 as isize;
_32 = _20.0.0;
place!(Field::<Adt60>(Variant(_14, 0), 0)) = Adt60::Variant3 { fld0: _28,fld1: _21,fld2: _25.0 };
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = (_25.1,);
_33.2 = [_3,_36,_36,_36,_36];
_34 = _41 * _41;
match _20.0.1 {
1612159965 => bb24,
_ => bb23
}
}
bb38 = {
_54.fld0 = Adt51::Variant0 { fld0: _20 };
_42 = _39;
place!(Field::<([i32; 1], (u8,))>(Variant(place!(Field::<Adt54>(Variant(place!(Field::<Adt60>(Variant(_14, 0), 0)), 1), 1)), 1), 0)).1 = (_25.1,);
_38.1 = _51 as i32;
_13 = [12252953525952748455_u64,13446155185480755535_u64];
_54.fld2.1 = _22 as i32;
_3 = _38.1;
_31 = !_54.fld1.fld2.0;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)).2 = _38.2;
Goto(bb39)
}
bb39 = {
_58 = _25;
_54.fld3 = _11.1.0;
Goto(bb40)
}
bb40 = {
place!(Field::<([i32; 1], (u8,))>(Variant(place!(Field::<Adt54>(Variant(place!(Field::<Adt60>(Variant(_14, 0), 0)), 1), 1)), 1), 0)) = (_10, _17.1);
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).1 = _17.1;
_8 = [_3];
_11 = Field::<([i32; 1], (u8,))>(Variant(Field::<Adt54>(Variant(Field::<Adt60>(Variant(_14, 0), 0), 1), 1), 1), 0);
place!(Field::<([i32; 1], (u8,))>(Variant(place!(Field::<Adt54>(Variant(place!(Field::<Adt60>(Variant(_14, 0), 0)), 1), 1)), 1), 0)).1.0 = _11.1.0;
_30 = _52 >> _33.1;
place!(Field::<([i32; 1], (u8,))>(Variant(place!(Field::<Adt54>(Variant(place!(Field::<Adt60>(Variant(_14, 0), 0)), 1), 1)), 1), 0)).1 = (Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2).1.0,);
_54.fld4 = !9945510615508635605_u64;
_17 = Field::<([i32; 1], (u8,))>(Variant(Field::<Adt54>(Variant(Field::<Adt60>(Variant(_14, 0), 0), 1), 1), 1), 0);
_57.1 = _33.1 | _33.1;
_42 = _16 >> _11.1.0;
_15 = Move(Field::<Adt54>(Variant(Field::<Adt60>(Variant(_14, 0), 0), 1), 1));
_12 = [_4,_26,_26,_5,_9,_19,_5];
place!(Field::<u16>(Variant(_15, 1), 1)) = _22;
_57 = (_9, _33.1, _18);
_48.1 = _11.1;
Call(_51 = core::intrinsics::bswap(_58.0), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).1 = (_17.1.0,);
_17 = (_48.0, Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2).1);
SetDiscriminant(_15, 1);
_34 = _41 + _41;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)).1 = core::ptr::addr_of!((*_43).2);
_20.0 = (_32, Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0).0.1, _21);
_14 = Adt61::Variant1 { fld0: _6 };
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1 = (_31,);
_52 = !_30;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)).1 = _20.1;
_2 = _3 >= _38.1;
SetDiscriminant(_14, 0);
_20.2 = _54.fld2.2;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)) = _20;
_20.0 = Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0).0;
_13 = [_54.fld4,_54.fld4];
_10 = [_3];
_48.0 = [_38.1];
_17.1 = (_58.1,);
_58.0 = !_51;
place!(Field::<u16>(Variant(_15, 1), 1)) = _45 as u16;
_57.0 = !_9;
_47 = _22 as f32;
_15 = Adt54::Variant1 { fld0: _17,fld1: _44 };
SetDiscriminant(_54.fld0, 0);
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).0 = [_38.1];
_58 = (_25.0, _54.fld3);
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)).0 = (*_43);
_42 = !_27;
match _57.1 {
0 => bb1,
1 => bb40,
2 => bb14,
3 => bb20,
4 => bb18,
340282366920938463463374607431768211332 => bb43,
_ => bb42
}
}
bb42 = {
_2 = !_4;
_20.0.1 = 3162206307_u32 << _25.1;
_15 = Adt54::Variant1 { fld0: _17,fld1: _22 };
_13 = [14907049803533126755_u64,8856348605277104375_u64];
place!(Field::<*const i16>(Variant(_14, 1), 0)) = _6;
_33.1 = 84_i8;
SetDiscriminant(_15, 1);
_12 = [_19,_9,_26,_26,_19,_33.0,_9];
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).0 = _8;
_38.0 = core::ptr::addr_of!(_24);
_17.1.0 = _1 as u8;
_20.0 = (_32, 520468358_u32, _21);
match _25.0 {
0 => bb19,
_ => bb18
}
}
bb43 = {
_54.fld1.fld1 = Adt47::Variant3 { fld0: _42,fld1: _13 };
_56 = _42 as u32;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)).2 = _20.2;
_33.2 = [_38.1,_38.1,_38.1,_38.1,_38.1];
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)) = ((*_43), _54.fld2.2, _38.2);
place!(Field::<[u64; 2]>(Variant(_54.fld1.fld1, 3), 1)) = [_54.fld4,_54.fld4];
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)).0.0 = core::ptr::addr_of_mut!(_22);
_33.2 = [_3,_3,_38.1,_38.1,_3];
_48.0 = [_38.1];
_58.0 = _33.1 as usize;
_45 = (*_43).2;
Goto(bb44)
}
bb44 = {
_2 = _19;
_63.fld1.fld1 = Adt47::Variant1 { fld0: _20 };
_38 = (_6, _3, _54.fld2.2);
_10 = [_3];
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld1.fld1, 1), 0)) = (Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0).0, _20.2, Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0).2);
_63.fld0 = Move(_54.fld0);
_34 = -_41;
_3 = _38.1;
_38.1 = -_54.fld2.1;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld0, 0), 0)).1 = core::ptr::addr_of!(_45);
_54.fld1.fld4 = [_3];
_20.1 = Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld1.fld1, 1), 0).2;
_46 = _54.fld1.fld4;
_11.0 = [_3];
Goto(bb45)
}
bb45 = {
_33.1 = (*_43).2 as i8;
_17.0 = [_3];
_54.fld1.fld2.0 = _1 as u8;
_34 = _41 * _41;
_63.fld2.1 = _3;
_63.fld1.fld1 = _54.fld1.fld1;
_5 = _33.0 | _57.0;
_48.1.0 = _54.fld4 as u8;
_61 = _47 + _47;
_38.1 = _3 - _3;
SetDiscriminant(_54.fld1.fld1, 2);
place!(Field::<u16>(Variant(_15, 1), 1)) = _22;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld0, 0), 0)) = _20;
_63.fld1.fld4 = [_38.1];
_20.0.1 = !_56;
_64.fld4 = [_38.1];
Goto(bb46)
}
bb46 = {
_12 = _50;
_65 = _3;
_56 = !(*_43).1;
_52 = !_30;
place!(Field::<(bool, i8, [i32; 5])>(Variant(_54.fld1.fld1, 2), 0)).0 = _5;
_64.fld4 = [_65];
_48.0 = _10;
place!(Field::<*mut *mut u16>(Variant(_54.fld1.fld1, 2), 2)) = core::ptr::addr_of_mut!((*_43).0);
_30 = _42 as i64;
_63.fld2.0 = _6;
_21 = _20.0.2;
_71 = _54.fld2.0;
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)).1.0 = !_31;
place!(Field::<i64>(Variant(_54.fld1.fld1, 2), 1)) = (*_6) as i64;
_66 = (_54.fld3,);
_8 = [_3];
Call(_68 = core::intrinsics::transmute(_54.fld1.fld4), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
_54.fld0 = Move(_63.fld0);
_64.fld2 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1;
_63.fld4 = _54.fld4;
_68 = !(*_43).1;
_58 = (_25.0, _11.1.0);
_42 = _21 as isize;
_26 = _2;
_63.fld1.fld3 = _22;
_71 = _6;
_33 = _57;
_54.fld2.2 = core::ptr::addr_of!((*_43).2);
_53 = core::ptr::addr_of_mut!(place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0)).0.0);
_20.0.0 = (*_53);
Goto(bb48)
}
bb48 = {
_68 = _33.1 as u32;
_20.0.1 = _56 / Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_54.fld0, 0), 0).0.1;
_29 = [_65];
_54.fld3 = _25.0 as u8;
_54.fld2.0 = _38.0;
_29 = [_54.fld2.1];
_65 = !_63.fld2.1;
_75.fld2 = (_71, _65, _20.1);
place!(Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2)).1.0 = _52 as u8;
_57.1 = _44 as i8;
_75.fld1.fld1 = Adt47::Variant1 { fld0: _20 };
match _33.1 {
340282366920938463463374607431768211332 => bb50,
_ => bb49
}
}
bb49 = {
_33.1 = (*_43).2 as i8;
_17.0 = [_3];
_54.fld1.fld2.0 = _1 as u8;
_34 = _41 * _41;
_63.fld2.1 = _3;
_63.fld1.fld1 = _54.fld1.fld1;
_5 = _33.0 | _57.0;
_48.1.0 = _54.fld4 as u8;
_61 = _47 + _47;
_38.1 = _3 - _3;
SetDiscriminant(_54.fld1.fld1, 2);
place!(Field::<u16>(Variant(_15, 1), 1)) = _22;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld0, 0), 0)) = _20;
_63.fld1.fld4 = [_38.1];
_20.0.1 = !_56;
_64.fld4 = [_38.1];
Goto(bb46)
}
bb50 = {
place!(Field::<[bool; 7]>(Variant(_14, 0), 1)) = [_26,_2,_33.0,_9,Field::<(bool, i8, [i32; 5])>(Variant(_54.fld1.fld1, 2), 0).0,_19,_26];
_75.fld5 = Adt52::Variant1 { fld0: _75.fld1.fld1 };
SetDiscriminant(_75.fld1.fld1, 0);
SetDiscriminant(Field::<Adt47>(Variant(_75.fld5, 1), 0), 1);
place!(Field::<(*const i16, i32, *const char)>(Variant(_75.fld1.fld1, 0), 2)).0 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_54.fld1.fld1, 2), 4)));
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(place!(Field::<Adt47>(Variant(_75.fld5, 1), 0)), 1), 0)).0.1 = !_68;
place!(Field::<(*const i16, i32, *const char)>(Variant(_75.fld1.fld1, 0), 2)) = (_54.fld2.0, _3, _38.2);
_54.fld2 = _38;
_57.2 = [_3,_38.1,_3,_36,Field::<(*const i16, i32, *const char)>(Variant(_75.fld1.fld1, 0), 2).1];
_63.fld2.2 = _38.2;
_64.fld2.0 = _66.0;
_37 = [_30,_30,_30,_52];
place!(Field::<i64>(Variant(_54.fld1.fld1, 2), 1)) = _30 ^ _30;
_33.0 = _4 > _9;
_75.fld1.fld3 = Field::<u16>(Variant(_15, 1), 1);
_63.fld0 = Move(_54.fld0);
_4 = !_33.0;
_20.1 = core::ptr::addr_of!(place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(place!(Field::<Adt47>(Variant(_75.fld5, 1), 0)), 1), 0)).0.2);
_54.fld3 = _11.1.0 << _39;
_54.fld1.fld4 = [_65];
_73.0 = [_54.fld2.1];
_71 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_54.fld1.fld1, 2), 4)));
_43 = core::ptr::addr_of_mut!(place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(place!(Field::<Adt47>(Variant(_75.fld5, 1), 0)), 1), 0)).0);
Goto(bb51)
}
bb51 = {
_11 = (_10, _66);
match _33.1 {
0 => bb41,
1 => bb49,
340282366920938463463374607431768211332 => bb53,
_ => bb52
}
}
bb52 = {
_3 = 604646520_i32 << _1;
_5 = _2;
_2 = _5;
_1 = 87115784655464122157960458798123680230_u128 | 190942851609882953334175992722175929136_u128;
_4 = _5 != _5;
_4 = _5;
_3 = '\u{e1a74}' as i32;
_4 = _2 | _2;
_4 = _1 > _1;
_1 = _5 as u128;
_3 = (-1586655455_i32) >> _1;
Goto(bb2)
}
bb53 = {
place!(Field::<Adt47>(Variant(_75.fld5, 1), 0)) = _63.fld1.fld1;
_28 = _37;
_80.fld1.fld3 = _22 * _75.fld1.fld3;
RET = core::ptr::addr_of!(place!(Field::<*const *mut u16>(Variant(_75.fld1.fld1, 0), 0)));
_28 = [_30,_30,_52,_30];
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld0, 0), 0)) = (_20.0, _20.1, Field::<(*const i16, i32, *const char)>(Variant(_75.fld1.fld1, 0), 2).2);
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld0, 0), 0)).0.2 = _21;
place!(Field::<*const *mut u16>(Variant(_75.fld1.fld1, 0), 0)) = core::ptr::addr_of!(place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld0, 0), 0)).0.0);
_13 = Field::<[u64; 2]>(Variant(_63.fld1.fld1, 3), 1);
place!(Field::<(bool, i8, [i32; 5])>(Variant(_54.fld1.fld1, 2), 0)).2 = _57.2;
_77 = !Field::<i64>(Variant(_54.fld1.fld1, 2), 1);
place!(Field::<(*const i16, i32, *const char)>(Variant(_75.fld1.fld1, 0), 2)) = (_71, _54.fld2.1, _63.fld2.2);
_83 = _58;
_80.fld5 = Move(_75.fld5);
_63.fld2.0 = core::ptr::addr_of!(_24);
_31 = _2 as u8;
_54.fld1.fld4 = [Field::<(*const i16, i32, *const char)>(Variant(_75.fld1.fld1, 0), 2).1];
place!(Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0)) = Field::<([i32; 1], (u8,))>(Variant(_14, 0), 2);
_73.1 = Field::<([i32; 1], (u8,))>(Variant(_15, 1), 0).1;
_75.fld1.fld2.0 = !_17.1.0;
_18 = [_63.fld2.1,Field::<(*const i16, i32, *const char)>(Variant(_75.fld1.fld1, 0), 2).1,_63.fld2.1,_3,_54.fld2.1];
_75.fld1.fld3 = Field::<u16>(Variant(_15, 1), 1);
_80.fld1.fld2 = (_25.1,);
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_63.fld0, 0), 0)) = (_20.0, _20.1, _38.2);
Goto(bb54)
}
bb54 = {
Call(_89 = dump_var(1_usize, 4_usize, Move(_4), 24_usize, Move(_24), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_89 = dump_var(1_usize, 44_usize, Move(_44), 45_usize, Move(_45), 27_usize, Move(_27), 17_usize, Move(_17)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_89 = dump_var(1_usize, 13_usize, Move(_13), 26_usize, Move(_26), 77_usize, Move(_77), 48_usize, Move(_48)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_89 = dump_var(1_usize, 58_usize, Move(_58), 22_usize, Move(_22), 42_usize, Move(_42), 51_usize, Move(_51)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_89 = dump_var(1_usize, 83_usize, Move(_83), 28_usize, Move(_28), 33_usize, Move(_33), 36_usize, Move(_36)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_89 = dump_var(1_usize, 19_usize, Move(_19), 73_usize, Move(_73), 35_usize, Move(_35), 90_usize, _90), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u128,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool) -> i32 {
mir! {
type RET = i32;
let _8: Adt48;
let _9: &'static isize;
let _10: u16;
let _11: isize;
let _12: f32;
let _13: isize;
let _14: bool;
let _15: isize;
let _16: isize;
let _17: isize;
let _18: ();
let _19: ();
{
_6 = _1 > _1;
_3 = _5;
_2 = !_4;
_7 = !_6;
RET = !95242986_i32;
_4 = _3 != _3;
_7 = !_3;
_6 = !_7;
_6 = _4 > _5;
RET = (-554659564_i32);
_10 = !6259_u16;
_5 = _2 == _4;
_6 = _2 ^ _7;
RET = (-9223372036854775808_isize) as i32;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
29207975780031171893800457048268052017 => bb9,
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
_4 = _6;
_9 = &_11;
_10 = 32993_u16 ^ 41872_u16;
_9 = &_11;
_9 = &(*_9);
_3 = _6 & _6;
RET = (-76909278388707239409765129430009687765_i128) as i32;
_4 = _2;
RET = '\u{f2912}' as i32;
_5 = _6;
_11 = (-6211753810826295312503334321084199854_i128) as isize;
_11 = (-9223372036854775808_isize);
_6 = _3;
_4 = !_6;
_9 = &_11;
_3 = _2;
_13 = 112_i8 as isize;
_13 = (*_9) & (*_9);
_12 = 85_i8 as f32;
_6 = !_4;
_15 = (*_9);
_14 = _6 <= _7;
_3 = !_6;
_3 = !_4;
_15 = _13;
_10 = !48552_u16;
match (*_9) {
0 => bb1,
1 => bb7,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
340282366920938463454151235394913435648 => bb16,
_ => bb15
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
Return()
}
bb15 = {
Return()
}
bb16 = {
_1 = !10702461314696608767784084912433472982_u128;
_11 = _13;
_4 = !_14;
_3 = _11 != _13;
_6 = _4;
RET = (-1207158759_i32);
_7 = _6 != _4;
_5 = !_7;
Goto(bb17)
}
bb17 = {
Call(_18 = dump_var(2_usize, 1_usize, Move(_1), 2_usize, Move(_2), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_18 = dump_var(2_usize, 6_usize, Move(_6), 5_usize, Move(_5), 19_usize, _19, 19_usize, _19), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool) -> bool {
mir! {
type RET = bool;
let _6: Adt54;
let _7: f64;
let _8: bool;
let _9: ();
let _10: ();
{
_4 = _3 == _1;
RET = _3;
_2 = !_1;
_8 = _2;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(3_usize, 4_usize, Move(_4), 5_usize, Move(_5), 2_usize, Move(_2), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: (u8,),mut _3: Adt54,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: u8,mut _13: (u8,)) -> char {
mir! {
type RET = char;
let _14: u64;
let _15: *mut (i8, f32, *mut u16);
let _16: [i16; 2];
let _17: *const *mut u16;
let _18: [bool; 7];
let _19: [bool; 7];
let _20: Adt53;
let _21: [u64; 2];
let _22: i64;
let _23: [i64; 4];
let _24: i16;
let _25: (*const i16,);
let _26: Adt54;
let _27: i8;
let _28: (usize, u8);
let _29: char;
let _30: u16;
let _31: i32;
let _32: (*const i16,);
let _33: [i16; 2];
let _34: f64;
let _35: Adt47;
let _36: i128;
let _37: ([i32; 1], (u8,));
let _38: [i16; 2];
let _39: f32;
let _40: [bool; 7];
let _41: bool;
let _42: u8;
let _43: [i32; 1];
let _44: i16;
let _45: isize;
let _46: (bool, i8, [i32; 5]);
let _47: isize;
let _48: f32;
let _49: *mut (i8, f32, *mut u16);
let _50: i64;
let _51: [u64; 2];
let _52: f64;
let _53: [i16; 2];
let _54: *mut (*mut u16, u32, char);
let _55: ();
let _56: ();
{
_1 = _5 < _9;
_9 = !_7;
RET = '\u{ddf37}';
Call(_8 = fn5(_2, _4, _13.0, _5, Field::<([i32; 1], (u8,))>(Variant(_3, 1), 0).1, _6, Move(_3), _2, _11, _13, _10, _9, _2.0, _10, _11, _2.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = (_13.0,);
_8 = !_1;
_12 = (-99_i8) as u8;
_5 = !_9;
_16 = [(-28123_i16),(-29173_i16)];
_13.0 = !_2.0;
_9 = _6 <= _4;
_4 = _11 ^ _11;
_6 = _8 > _4;
_11 = _6 & _8;
Goto(bb2)
}
bb2 = {
_13 = (_2.0,);
_2 = _13;
_19 = [_9,_1,_11,_5,_1,_5,_11];
_7 = _10;
_18 = [_10,_11,_7,_1,_4,_11,_11];
_22 = (-9223372036854775808_isize) as i64;
_8 = !_6;
_1 = _2.0 != _2.0;
_2 = (_13.0,);
Call(_21 = fn6(_8, _13.0, _2.0, _13.0, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_22 = 7959336937947085588_i64;
_5 = !_10;
_23 = [_22,_22,_22,_22];
_5 = _8;
_4 = !_6;
_18 = [_1,_1,_6,_4,_7,_11,_8];
_23 = [_22,_22,_22,_22];
_14 = _13.0 as u64;
_1 = _8;
_13.0 = _2.0;
_5 = _4 > _4;
_24 = 11400_i16;
_24 = (-18928_i16);
_19 = [_4,_9,_6,_11,_9,_1,_7];
_5 = !_7;
_16 = [_24,_24];
_25.0 = core::ptr::addr_of!(_24);
_24 = (-1675333568_i32) as i16;
_13 = _2;
_19 = [_8,_6,_8,_5,_10,_5,_5];
_21 = [_14,_14];
_16 = [_24,_24];
_23 = [_22,_22,_22,_22];
_13 = (_2.0,);
_2.0 = !_13.0;
_24 = _13.0 as i16;
_13.0 = _2.0 ^ _2.0;
Goto(bb4)
}
bb4 = {
_2.0 = _13.0 * _13.0;
RET = '\u{2d9ae}';
_28 = (16480403373519226705_usize, _13.0);
_27 = (-59_i8);
_2.0 = _13.0 & _28.1;
_12 = _2.0 & _28.1;
_30 = _6 as u16;
_13.0 = !_28.1;
_23 = [_22,_22,_22,_22];
_8 = !_11;
_28.1 = 2556500037_u32 as u8;
_32.0 = _25.0;
_31 = _24 as i32;
RET = '\u{8ddd7}';
_2 = (_12,);
_23 = [_22,_22,_22,_22];
RET = '\u{879b0}';
_9 = _28.0 > _28.0;
RET = '\u{5747d}';
_19 = [_11,_1,_9,_6,_8,_1,_6];
_31 = (-1196601556_i32) + 801763614_i32;
Goto(bb5)
}
bb5 = {
_34 = _22 as f64;
_2.0 = !_13.0;
_16 = [_24,_24];
_25.0 = _32.0;
_29 = RET;
_7 = _6;
_32 = (_25.0,);
_23 = [_22,_22,_22,_22];
_14 = !2225196446695865688_u64;
RET = _29;
_30 = 10624_u16;
_34 = 289766408022859839980513811211475310760_u128 as f64;
_19 = _18;
_8 = _4 <= _6;
Goto(bb6)
}
bb6 = {
_6 = !_7;
_9 = !_6;
_33 = [_24,_24];
_16 = [_24,_24];
_25.0 = core::ptr::addr_of!(_24);
_30 = 34616_u16;
_32.0 = core::ptr::addr_of!(_24);
_2.0 = !_12;
_28 = (6774073277337810472_usize, _13.0);
_29 = RET;
_12 = _28.1;
_32 = (_25.0,);
Goto(bb7)
}
bb7 = {
_13 = (_12,);
_25 = (_32.0,);
_37.0 = [_31];
_11 = !_6;
match _28.0 {
0 => bb5,
1 => bb8,
2 => bb9,
6774073277337810472 => bb11,
_ => bb10
}
}
bb8 = {
_2 = (_13.0,);
_8 = !_1;
_12 = (-99_i8) as u8;
_5 = !_9;
_16 = [(-28123_i16),(-29173_i16)];
_13.0 = !_2.0;
_9 = _6 <= _4;
_4 = _11 ^ _11;
_6 = _8 > _4;
_11 = _6 & _8;
Goto(bb2)
}
bb9 = {
_34 = _22 as f64;
_2.0 = !_13.0;
_16 = [_24,_24];
_25.0 = _32.0;
_29 = RET;
_7 = _6;
_32 = (_25.0,);
_23 = [_22,_22,_22,_22];
_14 = !2225196446695865688_u64;
RET = _29;
_30 = 10624_u16;
_34 = 289766408022859839980513811211475310760_u128 as f64;
_19 = _18;
_8 = _4 <= _6;
Goto(bb6)
}
bb10 = {
_2.0 = _13.0 * _13.0;
RET = '\u{2d9ae}';
_28 = (16480403373519226705_usize, _13.0);
_27 = (-59_i8);
_2.0 = _13.0 & _28.1;
_12 = _2.0 & _28.1;
_30 = _6 as u16;
_13.0 = !_28.1;
_23 = [_22,_22,_22,_22];
_8 = !_11;
_28.1 = 2556500037_u32 as u8;
_32.0 = _25.0;
_31 = _24 as i32;
RET = '\u{8ddd7}';
_2 = (_12,);
_23 = [_22,_22,_22,_22];
RET = '\u{879b0}';
_9 = _28.0 > _28.0;
RET = '\u{5747d}';
_19 = [_11,_1,_9,_6,_8,_1,_6];
_31 = (-1196601556_i32) + 801763614_i32;
Goto(bb5)
}
bb11 = {
_16 = [_24,_24];
_36 = _14 as i128;
_7 = _11 | _8;
_11 = _1 ^ _1;
_13 = _2;
_18 = _19;
_37.1.0 = _22 as u8;
_43 = _37.0;
RET = _29;
_39 = 1126917834_u32 as f32;
_4 = !_8;
_4 = _8 & _6;
_2 = (_28.1,);
_38 = [_24,_24];
_10 = _6 & _9;
_44 = _24 * _24;
_38 = [_44,_24];
_5 = _2.0 >= _2.0;
_23 = [_22,_22,_22,_22];
_28.1 = _13.0 + _2.0;
_43 = [_31];
_7 = _9 | _11;
_42 = _10 as u8;
Goto(bb12)
}
bb12 = {
_6 = _9 ^ _11;
RET = _29;
match _28.0 {
0 => bb1,
6774073277337810472 => bb13,
_ => bb5
}
}
bb13 = {
_46.2 = [_31,_31,_31,_31,_31];
_25 = _32;
_47 = 272761368645202415660250739284772234135_u128 as isize;
_3 = Adt54::Variant1 { fld0: _37,fld1: _30 };
_30 = 3755317269_u32 as u16;
_6 = !_10;
_47 = _2.0 as isize;
_32.0 = core::ptr::addr_of!(_24);
match _28.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6774073277337810472 => bb21,
_ => bb20
}
}
bb14 = {
_6 = _9 ^ _11;
RET = _29;
match _28.0 {
0 => bb1,
6774073277337810472 => bb13,
_ => bb5
}
}
bb15 = {
_2.0 = _13.0 * _13.0;
RET = '\u{2d9ae}';
_28 = (16480403373519226705_usize, _13.0);
_27 = (-59_i8);
_2.0 = _13.0 & _28.1;
_12 = _2.0 & _28.1;
_30 = _6 as u16;
_13.0 = !_28.1;
_23 = [_22,_22,_22,_22];
_8 = !_11;
_28.1 = 2556500037_u32 as u8;
_32.0 = _25.0;
_31 = _24 as i32;
RET = '\u{8ddd7}';
_2 = (_12,);
_23 = [_22,_22,_22,_22];
RET = '\u{879b0}';
_9 = _28.0 > _28.0;
RET = '\u{5747d}';
_19 = [_11,_1,_9,_6,_8,_1,_6];
_31 = (-1196601556_i32) + 801763614_i32;
Goto(bb5)
}
bb16 = {
_2.0 = _13.0 * _13.0;
RET = '\u{2d9ae}';
_28 = (16480403373519226705_usize, _13.0);
_27 = (-59_i8);
_2.0 = _13.0 & _28.1;
_12 = _2.0 & _28.1;
_30 = _6 as u16;
_13.0 = !_28.1;
_23 = [_22,_22,_22,_22];
_8 = !_11;
_28.1 = 2556500037_u32 as u8;
_32.0 = _25.0;
_31 = _24 as i32;
RET = '\u{8ddd7}';
_2 = (_12,);
_23 = [_22,_22,_22,_22];
RET = '\u{879b0}';
_9 = _28.0 > _28.0;
RET = '\u{5747d}';
_19 = [_11,_1,_9,_6,_8,_1,_6];
_31 = (-1196601556_i32) + 801763614_i32;
Goto(bb5)
}
bb17 = {
_2 = (_13.0,);
_8 = !_1;
_12 = (-99_i8) as u8;
_5 = !_9;
_16 = [(-28123_i16),(-29173_i16)];
_13.0 = !_2.0;
_9 = _6 <= _4;
_4 = _11 ^ _11;
_6 = _8 > _4;
_11 = _6 & _8;
Goto(bb2)
}
bb18 = {
_2 = (_13.0,);
_8 = !_1;
_12 = (-99_i8) as u8;
_5 = !_9;
_16 = [(-28123_i16),(-29173_i16)];
_13.0 = !_2.0;
_9 = _6 <= _4;
_4 = _11 ^ _11;
_6 = _8 > _4;
_11 = _6 & _8;
Goto(bb2)
}
bb19 = {
_13 = (_12,);
_25 = (_32.0,);
_37.0 = [_31];
_11 = !_6;
match _28.0 {
0 => bb5,
1 => bb8,
2 => bb9,
6774073277337810472 => bb11,
_ => bb10
}
}
bb20 = {
_6 = !_7;
_9 = !_6;
_33 = [_24,_24];
_16 = [_24,_24];
_25.0 = core::ptr::addr_of!(_24);
_30 = 34616_u16;
_32.0 = core::ptr::addr_of!(_24);
_2.0 = !_12;
_28 = (6774073277337810472_usize, _13.0);
_29 = RET;
_12 = _28.1;
_32 = (_25.0,);
Goto(bb7)
}
bb21 = {
_46.1 = _27;
_6 = !_1;
_26 = Move(_3);
_3 = Move(_26);
_13.0 = _42;
_35 = Adt47::Variant3 { fld0: _47,fld1: _21 };
_50 = _22 - _22;
_51 = [_14,_14];
_47 = -Field::<isize>(Variant(_35, 3), 0);
_25 = (_32.0,);
_52 = _34;
_37.1 = _13;
place!(Field::<([i32; 1], (u8,))>(Variant(_3, 1), 0)).1 = _13;
_43 = [_31];
_28.0 = Field::<u16>(Variant(_3, 1), 1) as usize;
RET = _29;
_12 = !Field::<([i32; 1], (u8,))>(Variant(_3, 1), 0).1.0;
Goto(bb22)
}
bb22 = {
Call(_55 = dump_var(4_usize, 14_usize, Move(_14), 5_usize, Move(_5), 16_usize, Move(_16), 2_usize, Move(_2)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_55 = dump_var(4_usize, 29_usize, Move(_29), 12_usize, Move(_12), 4_usize, Move(_4), 10_usize, Move(_10)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_55 = dump_var(4_usize, 43_usize, Move(_43), 24_usize, Move(_24), 27_usize, Move(_27), 1_usize, Move(_1)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_55 = dump_var(4_usize, 38_usize, Move(_38), 9_usize, Move(_9), 13_usize, Move(_13), 30_usize, Move(_30)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_55 = dump_var(4_usize, 36_usize, Move(_36), 56_usize, _56, 56_usize, _56, 56_usize, _56), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: (u8,),mut _2: bool,mut _3: u8,mut _4: bool,mut _5: (u8,),mut _6: bool,mut _7: Adt54,mut _8: (u8,),mut _9: bool,mut _10: (u8,),mut _11: bool,mut _12: bool,mut _13: u8,mut _14: bool,mut _15: bool,mut _16: u8) -> bool {
mir! {
type RET = bool;
let _17: u64;
let _18: [i32; 1];
let _19: u8;
let _20: [i64; 4];
let _21: ();
let _22: ();
{
place!(Field::<([i32; 1], (u8,))>(Variant(_7, 1), 0)).1.0 = _13 + _16;
place!(Field::<u16>(Variant(_7, 1), 1)) = 63690_u16 >> _16;
_4 = !_12;
Goto(bb1)
}
bb1 = {
_1.0 = !Field::<([i32; 1], (u8,))>(Variant(_7, 1), 0).1.0;
_12 = _14;
RET = _13 >= Field::<([i32; 1], (u8,))>(Variant(_7, 1), 0).1.0;
_5.0 = 2427635871_u32 as u8;
_10.0 = _15 as u8;
_12 = Field::<([i32; 1], (u8,))>(Variant(_7, 1), 0).1.0 < _8.0;
_15 = !_4;
place!(Field::<([i32; 1], (u8,))>(Variant(_7, 1), 0)).1.0 = _1.0 - _10.0;
RET = _6;
_1.0 = !_10.0;
_17 = 9464734100888210307_u64;
_11 = !_12;
_6 = RET;
_19 = _1.0;
_17 = 3960652279656800497_u64;
Goto(bb2)
}
bb2 = {
Call(_21 = dump_var(5_usize, 11_usize, Move(_11), 2_usize, Move(_2), 12_usize, Move(_12), 16_usize, Move(_16)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_21 = dump_var(5_usize, 1_usize, Move(_1), 3_usize, Move(_3), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: u8,mut _3: u8,mut _4: u8,mut _5: (u8,)) -> [u64; 2] {
mir! {
type RET = [u64; 2];
let _6: i128;
let _7: i128;
let _8: u64;
let _9: f32;
let _10: isize;
let _11: (*mut u16, u32, char);
let _12: isize;
let _13: ([u64; 2], *const [i32; 1]);
let _14: isize;
let _15: i16;
let _16: [i32; 5];
let _17: f64;
let _18: char;
let _19: u16;
let _20: i128;
let _21: usize;
let _22: *mut (i8, f32, *mut u16);
let _23: i16;
let _24: *const [i32; 1];
let _25: [i64; 4];
let _26: [i32; 5];
let _27: bool;
let _28: Adt63;
let _29: [i16; 2];
let _30: u64;
let _31: *mut (i8, f32, *mut u16);
let _32: [i16; 3];
let _33: f32;
let _34: (bool, i8, [i32; 5]);
let _35: [i16; 3];
let _36: Adt54;
let _37: isize;
let _38: (bool, i8, [i32; 5]);
let _39: i128;
let _40: *const char;
let _41: [i32; 5];
let _42: isize;
let _43: [i16; 3];
let _44: ([i32; 1], (u8,));
let _45: ();
let _46: ();
{
_5.0 = !_2;
_5 = (_4,);
_5 = (_4,);
_5.0 = _3;
RET = [433837010492862971_u64,26477540253328241_u64];
_7 = 6028003744941658030_i64 as i128;
_2 = _3 * _5.0;
_1 = !true;
_4 = _3;
_9 = 92_i8 as f32;
_5 = (_2,);
_8 = 17976367540442602952_u64 + 16170207192714615336_u64;
_10 = 9223372036854775807_isize;
_5.0 = _3 << _4;
RET = [_8,_8];
match _10 {
9223372036854775807 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_4 = !_5.0;
_2 = !_4;
_5 = (_3,);
RET = [_8,_8];
_11.1 = !811376295_u32;
RET = [_8,_8];
_9 = _8 as f32;
_4 = _5.0 & _5.0;
RET = [_8,_8];
_8 = 2016094614684768744_u64 + 12037311614804545507_u64;
_3 = '\u{f5a0}' as u8;
_4 = _5.0;
_13.0 = [_8,_8];
_10 = -9223372036854775807_isize;
RET = _13.0;
_11.2 = '\u{d717f}';
_14 = !_10;
Goto(bb3)
}
bb3 = {
_5 = (_4,);
_16 = [(-1109104450_i32),1421954543_i32,863802531_i32,1262329290_i32,(-360874690_i32)];
_15 = (-26982_i16) & (-11671_i16);
_14 = _10 << _2;
_7 = (-17016983943084034441264332132473612822_i128);
_2 = _4 ^ _5.0;
_18 = _11.2;
RET = [_8,_8];
_3 = _5.0;
_17 = _9 as f64;
Goto(bb4)
}
bb4 = {
_6 = _7 >> _5.0;
RET = _13.0;
_10 = -_14;
_15 = -29737_i16;
_9 = 99047958681718953233651162777894997809_u128 as f32;
_11.0 = core::ptr::addr_of_mut!(_19);
RET = [_8,_8];
_11.2 = _18;
_19 = !10156_u16;
_7 = _6 << _10;
_1 = !false;
RET = _13.0;
_20 = _7 * _7;
RET = [_8,_8];
_15 = !2127_i16;
_2 = _4 ^ _4;
_23 = _15;
_13.0 = RET;
_6 = _20 * _20;
_23 = _10 as i16;
_15 = _8 as i16;
_11.2 = _18;
Call(_16 = fn7(_10, _10, _6, _2, _4, _10, _6, _6, _23, _5.0, _23, _5.0, _20, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = [_8,_8];
_26 = [1801172056_i32,1689367176_i32,(-2094481805_i32),1280447413_i32,2007448450_i32];
_28.fld5.1.0 = !_2;
RET = _13.0;
_4 = !_3;
_28.fld3 = core::ptr::addr_of!(_11.2);
_28.fld5.1 = (_5.0,);
_12 = _14 + _14;
_8 = 6642764420023177070_u64;
_14 = _6 as isize;
match _8 {
6642764420023177070 => bb6,
_ => bb2
}
}
bb6 = {
_29 = [_23,_23];
_27 = _1;
_11.1 = 1537231447_u32;
_2 = _5.0 & _3;
_16 = [(-232018391_i32),(-1616456960_i32),456339660_i32,(-2084115965_i32),(-596292044_i32)];
_4 = _28.fld5.1.0 * _2;
match _8 {
0 => bb4,
1 => bb7,
6642764420023177070 => bb9,
_ => bb8
}
}
bb7 = {
RET = [_8,_8];
_26 = [1801172056_i32,1689367176_i32,(-2094481805_i32),1280447413_i32,2007448450_i32];
_28.fld5.1.0 = !_2;
RET = _13.0;
_4 = !_3;
_28.fld3 = core::ptr::addr_of!(_11.2);
_28.fld5.1 = (_5.0,);
_12 = _14 + _14;
_8 = 6642764420023177070_u64;
_14 = _6 as isize;
match _8 {
6642764420023177070 => bb6,
_ => bb2
}
}
bb8 = {
_4 = !_5.0;
_2 = !_4;
_5 = (_3,);
RET = [_8,_8];
_11.1 = !811376295_u32;
RET = [_8,_8];
_9 = _8 as f32;
_4 = _5.0 & _5.0;
RET = [_8,_8];
_8 = 2016094614684768744_u64 + 12037311614804545507_u64;
_3 = '\u{f5a0}' as u8;
_4 = _5.0;
_13.0 = [_8,_8];
_10 = -9223372036854775807_isize;
RET = _13.0;
_11.2 = '\u{d717f}';
_14 = !_10;
Goto(bb3)
}
bb9 = {
_21 = 7_usize;
Goto(bb10)
}
bb10 = {
_9 = _21 as f32;
_14 = -_10;
_30 = _8;
_13.1 = core::ptr::addr_of!(_28.fld5.0);
_9 = _21 as f32;
_28.fld5.1 = (_4,);
_8 = _30 | _30;
_2 = _3 << _3;
_28.fld0.2 = _11.2;
_28.fld0 = (_11.0, _11.1, _18);
_11.2 = _28.fld0.2;
_28.fld0 = _11;
_23 = !_15;
_34 = (_27, 31_i8, _26);
_28.fld5.1 = (_5.0,);
_29 = [_15,_23];
_33 = -_9;
_11.0 = core::ptr::addr_of_mut!(_19);
_28.fld5.1.0 = !_3;
_34 = (_1, 93_i8, _26);
_13.0 = [_8,_30];
_34.2 = [1406956067_i32,1254088535_i32,1508335638_i32,1114876186_i32,1106881401_i32];
_35 = [_15,_23,_15];
Call(_28.fld1 = fn16(_7, _12, _10, _7, _10, _10, _14, _10, _28.fld5.1.0, _12, _12, _11), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_38 = (_34.0, _34.1, _26);
_28.fld0.1 = _11.1;
Goto(bb12)
}
bb12 = {
_23 = _15;
_28.fld0.1 = _11.1 ^ _11.1;
_28.fld3 = core::ptr::addr_of!(_18);
_21 = 4873124178030929722_usize - 11651162432883411897_usize;
_28.fld0 = (_11.0, _11.1, _28.fld1);
_28.fld0.2 = _18;
_37 = _6 as isize;
_28.fld2 = [_30,_8];
_25 = [3336176920345614597_i64,783790542510213172_i64,8415372128286137322_i64,2019308334117206178_i64];
_28.fld0.2 = _18;
RET = [_30,_8];
_8 = _23 as u64;
_2 = _5.0;
_11.2 = _28.fld0.2;
_28.fld0.0 = core::ptr::addr_of_mut!(_19);
_34.0 = _37 < _12;
_38 = (_34.0, _34.1, _26);
_15 = _23;
Goto(bb13)
}
bb13 = {
_19 = !199_u16;
_7 = !_6;
_5.0 = !_3;
_11.0 = core::ptr::addr_of_mut!(_19);
_11 = (_28.fld0.0, _28.fld0.1, _18);
_11.2 = _18;
_24 = core::ptr::addr_of!(_28.fld5.0);
_3 = _19 as u8;
_37 = -_10;
_38.2 = _34.2;
_14 = _34.0 as isize;
_23 = _15 + _15;
_23 = _4 as i16;
_11.0 = _28.fld0.0;
_23 = _34.1 as i16;
_28.fld5.0 = [1415371859_i32];
match _38.1 {
0 => bb1,
93 => bb14,
_ => bb3
}
}
bb14 = {
RET = _28.fld2;
_20 = -_7;
_30 = _8;
_16 = [(-486773819_i32),(-1672036756_i32),(-623658179_i32),(-861703564_i32),(-2025373936_i32)];
_34 = _38;
_3 = _28.fld5.1.0 & _4;
_38 = (_34.0, _34.1, _26);
_28.fld5.1 = (_4,);
_20 = _6 + _7;
_2 = !_5.0;
_14 = -_12;
_26 = [356089066_i32,2088522490_i32,(-1325823760_i32),913456952_i32,(-900254174_i32)];
_27 = _38.0;
_5.0 = !_28.fld5.1.0;
_28.fld5.0 = [95773133_i32];
_24 = core::ptr::addr_of!(_28.fld5.0);
_13 = (RET, _24);
_30 = _20 as u64;
_28.fld0.2 = _18;
_28.fld0.0 = core::ptr::addr_of_mut!(_19);
_32 = [_23,_15,_23];
_16 = [2003489831_i32,(-2083188302_i32),(-1091700474_i32),(-1547951569_i32),(-2105710981_i32)];
_17 = _9 as f64;
_33 = _9 - _9;
_34.2 = [2110121387_i32,576143457_i32,(-1157496564_i32),1738011361_i32,(-2108144257_i32)];
_44.1.0 = !_4;
_28.fld0.0 = core::ptr::addr_of_mut!(_19);
_28.fld5.1.0 = _2;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(6_usize, 8_usize, Move(_8), 5_usize, Move(_5), 16_usize, Move(_16), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(6_usize, 7_usize, Move(_7), 6_usize, Move(_6), 27_usize, Move(_27), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(6_usize, 1_usize, Move(_1), 12_usize, Move(_12), 37_usize, Move(_37), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(6_usize, 2_usize, Move(_2), 30_usize, Move(_30), 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: isize,mut _3: i128,mut _4: u8,mut _5: u8,mut _6: isize,mut _7: i128,mut _8: i128,mut _9: i16,mut _10: u8,mut _11: i16,mut _12: u8,mut _13: i128,mut _14: (u8,)) -> [i32; 5] {
mir! {
type RET = [i32; 5];
let _15: isize;
let _16: char;
let _17: usize;
let _18: u16;
let _19: i128;
let _20: isize;
let _21: *const *mut u16;
let _22: (((*mut u16, u32, char), *const char, *const char),);
let _23: f32;
let _24: u32;
let _25: f32;
let _26: u32;
let _27: Adt60;
let _28: [u64; 2];
let _29: f32;
let _30: Adt47;
let _31: [bool; 7];
let _32: i8;
let _33: [i16; 3];
let _34: char;
let _35: (u8,);
let _36: i32;
let _37: ();
let _38: ();
{
_8 = _3 << _3;
_2 = _1 * _1;
_14 = (_5,);
_14.0 = _12 >> _12;
RET = [1042171613_i32,139816037_i32,1122384233_i32,1971583125_i32,1648961400_i32];
_3 = !_7;
_1 = _2;
_8 = 3781207928_u32 as i128;
_13 = 0_usize as i128;
_15 = _1;
Goto(bb1)
}
bb1 = {
_10 = _14.0 >> _6;
_13 = -_3;
_16 = '\u{101227}';
_7 = -_13;
_15 = _11 as isize;
_14 = (_4,);
_10 = !_5;
_8 = !_13;
_9 = 50339_u16 as i16;
_10 = (-310594218_i32) as u8;
_3 = _7;
_4 = _12 - _5;
_17 = (-2210165759985797542_i64) as usize;
_6 = _2;
_5 = _4 << _7;
_9 = -_11;
RET = [(-434234505_i32),(-667043754_i32),(-2051604527_i32),(-771576357_i32),(-999283851_i32)];
_9 = -_11;
_12 = !_14.0;
_11 = -_9;
_19 = _8;
_14 = (_12,);
_14.0 = _16 as u8;
_2 = -_1;
_17 = 23_i8 as usize;
_9 = !_11;
_20 = -_6;
Call(_2 = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [(-530709761_i32),607559077_i32,(-1635240342_i32),(-1658028364_i32),1663159879_i32];
_11 = (-199602450_i32) as i16;
_14 = (_5,);
_6 = _20;
_3 = _13;
_5 = _4 << _4;
Goto(bb3)
}
bb3 = {
_20 = _2 * _15;
_13 = !_3;
_9 = _11 | _11;
_11 = _9 & _9;
_15 = _2 | _6;
_17 = !6_usize;
_5 = !_4;
_1 = _6 ^ _15;
_6 = -_2;
_22.0.1 = core::ptr::addr_of!(_22.0.0.2);
_3 = _13 - _19;
_22.0.0.2 = _16;
_14 = (_12,);
_22.0.0.0 = core::ptr::addr_of_mut!(_18);
_3 = 7828968991361266269_u64 as i128;
_15 = 1908034099_u32 as isize;
_13 = true as i128;
_22.0.2 = _22.0.1;
_22.0.0.2 = _16;
Call(RET = fn8(_5, _14, _14, _7, _20, _14, _2, _2, _1, _2, _20, _7, _20, _1, _6, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_22.0.0.0 = core::ptr::addr_of_mut!(_18);
_14 = (_12,);
_19 = -_7;
_7 = _8;
_2 = _1;
_19 = _8;
_19 = _9 as i128;
_17 = 10130789593186399099_usize * 12094366297855123162_usize;
_21 = core::ptr::addr_of!(_22.0.0.0);
Goto(bb5)
}
bb5 = {
_2 = _6;
_21 = core::ptr::addr_of!(_22.0.0.0);
_23 = (-3107902914523872598_i64) as f32;
_1 = _20 >> _14.0;
_21 = core::ptr::addr_of!((*_21));
_25 = _9 as f32;
_3 = _25 as i128;
_26 = 2573021430_u32 | 1466048730_u32;
_8 = !_7;
_14.0 = _12 * _12;
_22.0.0.0 = core::ptr::addr_of_mut!(_18);
_1 = -_6;
_18 = 7824_u16;
_12 = _4 << _2;
_20 = _6;
_22.0.2 = _22.0.1;
_29 = _25 - _25;
_22.0.0.1 = 247942394950223166319739411607803326229_u128 as u32;
_20 = (-703636885_i32) as isize;
_12 = _14.0;
RET = [(-2007686608_i32),(-372468711_i32),(-527674542_i32),706346671_i32,1001096381_i32];
_21 = core::ptr::addr_of!((*_21));
match _18 {
0 => bb2,
7824 => bb7,
_ => bb6
}
}
bb6 = {
_10 = _14.0 >> _6;
_13 = -_3;
_16 = '\u{101227}';
_7 = -_13;
_15 = _11 as isize;
_14 = (_4,);
_10 = !_5;
_8 = !_13;
_9 = 50339_u16 as i16;
_10 = (-310594218_i32) as u8;
_3 = _7;
_4 = _12 - _5;
_17 = (-2210165759985797542_i64) as usize;
_6 = _2;
_5 = _4 << _7;
_9 = -_11;
RET = [(-434234505_i32),(-667043754_i32),(-2051604527_i32),(-771576357_i32),(-999283851_i32)];
_9 = -_11;
_12 = !_14.0;
_11 = -_9;
_19 = _8;
_14 = (_12,);
_14.0 = _16 as u8;
_2 = -_1;
_17 = 23_i8 as usize;
_9 = !_11;
_20 = -_6;
Call(_2 = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_8 = _22.0.0.1 as i128;
_22.0.1 = _22.0.2;
_12 = _16 as u8;
_16 = _22.0.0.2;
_3 = _7;
_15 = _6 - _2;
RET = [1355952443_i32,1216945197_i32,(-1260775935_i32),(-328675894_i32),(-977138860_i32)];
_25 = _18 as f32;
RET = [(-605234528_i32),(-751450607_i32),(-2093965899_i32),(-351289089_i32),1617173948_i32];
_28 = [17260712750231763597_u64,492935202976928739_u64];
_26 = _22.0.0.1;
_15 = 605026174_i32 as isize;
_22.0.1 = core::ptr::addr_of!(_22.0.0.2);
_7 = _3 | _3;
_19 = _7;
_16 = _22.0.0.2;
_22.0.2 = _22.0.1;
_14.0 = _26 as u8;
_31 = [false,false,false,false,true,false,false];
_17 = !8011581457051009442_usize;
_9 = 20826935782032758233054011055433087683_u128 as i16;
_11 = !_9;
_30 = Adt47::Variant1 { fld0: _22.0 };
_1 = _6 ^ _2;
Goto(bb8)
}
bb8 = {
_24 = Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0).0.1 ^ _22.0.0.1;
_31 = [false,true,true,false,true,false,false];
match _18 {
0 => bb1,
1 => bb9,
2 => bb10,
7824 => bb12,
_ => bb11
}
}
bb9 = {
_8 = _22.0.0.1 as i128;
_22.0.1 = _22.0.2;
_12 = _16 as u8;
_16 = _22.0.0.2;
_3 = _7;
_15 = _6 - _2;
RET = [1355952443_i32,1216945197_i32,(-1260775935_i32),(-328675894_i32),(-977138860_i32)];
_25 = _18 as f32;
RET = [(-605234528_i32),(-751450607_i32),(-2093965899_i32),(-351289089_i32),1617173948_i32];
_28 = [17260712750231763597_u64,492935202976928739_u64];
_26 = _22.0.0.1;
_15 = 605026174_i32 as isize;
_22.0.1 = core::ptr::addr_of!(_22.0.0.2);
_7 = _3 | _3;
_19 = _7;
_16 = _22.0.0.2;
_22.0.2 = _22.0.1;
_14.0 = _26 as u8;
_31 = [false,false,false,false,true,false,false];
_17 = !8011581457051009442_usize;
_9 = 20826935782032758233054011055433087683_u128 as i16;
_11 = !_9;
_30 = Adt47::Variant1 { fld0: _22.0 };
_1 = _6 ^ _2;
Goto(bb8)
}
bb10 = {
RET = [(-530709761_i32),607559077_i32,(-1635240342_i32),(-1658028364_i32),1663159879_i32];
_11 = (-199602450_i32) as i16;
_14 = (_5,);
_6 = _20;
_3 = _13;
_5 = _4 << _4;
Goto(bb3)
}
bb11 = {
_2 = _6;
_21 = core::ptr::addr_of!(_22.0.0.0);
_23 = (-3107902914523872598_i64) as f32;
_1 = _20 >> _14.0;
_21 = core::ptr::addr_of!((*_21));
_25 = _9 as f32;
_3 = _25 as i128;
_26 = 2573021430_u32 | 1466048730_u32;
_8 = !_7;
_14.0 = _12 * _12;
_22.0.0.0 = core::ptr::addr_of_mut!(_18);
_1 = -_6;
_18 = 7824_u16;
_12 = _4 << _2;
_20 = _6;
_22.0.2 = _22.0.1;
_29 = _25 - _25;
_22.0.0.1 = 247942394950223166319739411607803326229_u128 as u32;
_20 = (-703636885_i32) as isize;
_12 = _14.0;
RET = [(-2007686608_i32),(-372468711_i32),(-527674542_i32),706346671_i32,1001096381_i32];
_21 = core::ptr::addr_of!((*_21));
match _18 {
0 => bb2,
7824 => bb7,
_ => bb6
}
}
bb12 = {
_7 = _3 | _19;
Call(_24 = core::intrinsics::transmute(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0).0.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_32 = !112_i8;
RET = [1087997159_i32,(-1327706010_i32),2110679528_i32,43188334_i32,(-848539961_i32)];
_12 = _26 as u8;
_22.0.0.0 = core::ptr::addr_of_mut!(_18);
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0)).0 = _22.0.0;
_14 = (_5,);
_19 = _7;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0)).2 = core::ptr::addr_of!(_22.0.0.2);
_33 = [_9,_11,_9];
_31 = [false,true,true,false,false,true,false];
_8 = (-8650599103972427751_i64) as i128;
_15 = -_2;
_19 = _25 as i128;
_35.0 = _14.0;
_3 = _32 as i128;
_22.0.0.1 = !_24;
Goto(bb14)
}
bb14 = {
_17 = 2_usize & 7873622276454050689_usize;
_8 = _7;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0)).1 = Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0).2;
_22.0.2 = core::ptr::addr_of!(place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0)).0.2);
SetDiscriminant(_30, 1);
_12 = _5 - _5;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0)).0 = ((*_21), _26, _22.0.0.2);
_22.0.1 = core::ptr::addr_of!(_34);
_21 = core::ptr::addr_of!((*_21));
_4 = _12;
place!(Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0)).0.2 = _22.0.0.2;
_23 = -_29;
_29 = _23 + _23;
_22.0.0 = (Field::<((*mut u16, u32, char), *const char, *const char)>(Variant(_30, 1), 0).0.0, _26, _16);
_36 = !(-936287048_i32);
_5 = _4 ^ _35.0;
_33 = [_11,_9,_11];
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(7_usize, 2_usize, Move(_2), 19_usize, Move(_19), 17_usize, Move(_17), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(7_usize, 1_usize, Move(_1), 9_usize, Move(_9), 10_usize, Move(_10), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(7_usize, 8_usize, Move(_8), 35_usize, Move(_35), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(7_usize, 12_usize, Move(_12), 32_usize, Move(_32), 38_usize, _38, 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: u8,mut _2: (u8,),mut _3: (u8,),mut _4: i128,mut _5: isize,mut _6: (u8,),mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: i128,mut _13: isize,mut _14: isize,mut _15: isize,mut _16: isize) -> [i32; 5] {
mir! {
type RET = [i32; 5];
let _17: u8;
let _18: ([u64; 2], *const [i32; 1]);
let _19: [i32; 1];
let _20: i32;
let _21: isize;
let _22: (*const i16,);
let _23: char;
let _24: Adt61;
let _25: Adt59;
let _26: i8;
let _27: [i128; 8];
let _28: *const *mut u16;
let _29: *const *const *mut u16;
let _30: [i64; 4];
let _31: [i16; 3];
let _32: char;
let _33: *mut *mut u16;
let _34: isize;
let _35: usize;
let _36: [bool; 7];
let _37: isize;
let _38: i128;
let _39: isize;
let _40: isize;
let _41: Adt48;
let _42: [i16; 3];
let _43: Adt57;
let _44: Adt60;
let _45: isize;
let _46: isize;
let _47: usize;
let _48: [i16; 2];
let _49: bool;
let _50: [i16; 2];
let _51: u32;
let _52: isize;
let _53: (bool, i8, [i32; 5]);
let _54: Adt60;
let _55: f32;
let _56: Adt59;
let _57: usize;
let _58: (u8,);
let _59: ();
let _60: ();
{
_11 = -_9;
_16 = 14503529956722715234_u64 as isize;
_6.0 = _3.0;
_3.0 = !_2.0;
_1 = _6.0 | _6.0;
RET = [(-1303619996_i32),(-110756334_i32),2013749164_i32,645733664_i32,(-1156914425_i32)];
_10 = _5 | _13;
_16 = _14;
_5 = _9 - _8;
_17 = _3.0;
_15 = _5 | _7;
RET = [(-563710377_i32),1300625867_i32,(-442340325_i32),86533741_i32,1019712886_i32];
_15 = _14;
_13 = _16 ^ _11;
_18.1 = core::ptr::addr_of!(_19);
_2 = (_1,);
_3 = (_17,);
_12 = _4;
_14 = _15 >> _16;
_19 = [(-54612560_i32)];
_12 = _4;
_1 = 12544_i16 as u8;
RET = [505920822_i32,(-1902316476_i32),2045878860_i32,1600788199_i32,(-396660108_i32)];
_1 = _3.0;
_14 = !_16;
_10 = _11;
Goto(bb1)
}
bb1 = {
_17 = _3.0;
_14 = 42826_u16 as isize;
_8 = _5 >> _4;
_2 = (_6.0,);
_1 = _17 >> _6.0;
_3 = (_1,);
RET = [(-1345730058_i32),208429976_i32,794862655_i32,(-770265198_i32),(-1690350777_i32)];
Call(_7 = core::intrinsics::transmute(_15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = _8;
_5 = (-2139460191_i32) as isize;
_4 = _12 >> _9;
_5 = !_8;
Call(_18.1 = fn9(_16, _9, _2.0, _9, _1, _16, _1, _13, _5, _3, _9, _10, _13, _2, _3.0, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _12;
_17 = !_3.0;
_11 = _8 << _13;
Goto(bb4)
}
bb4 = {
_9 = _11 - _7;
_15 = 46897_u16 as isize;
_5 = _10;
_21 = -_5;
_13 = _21 << _16;
RET = [(-415842298_i32),(-854915451_i32),513594645_i32,(-107561545_i32),(-1309086626_i32)];
_8 = -_7;
RET = [(-164832311_i32),(-1493778587_i32),(-634220663_i32),(-52291978_i32),1331206196_i32];
_7 = !_16;
RET = [1762774185_i32,(-1857806694_i32),1106600016_i32,719677859_i32,110863330_i32];
_3 = (_6.0,);
_5 = _11;
_19 = [640743868_i32];
_3.0 = !_6.0;
RET = [20360400_i32,1817857720_i32,1382988525_i32,340145714_i32,(-1949717583_i32)];
_8 = _21 ^ _13;
_8 = !_7;
RET = [(-117873259_i32),(-1674082756_i32),(-841824723_i32),1210120101_i32,359791984_i32];
_13 = _9 ^ _7;
Call(_22.0 = fn13(_17, _10, _5, _5, _7, _8, _10, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20 = false as i32;
_11 = -_9;
_20 = -1249370178_i32;
RET = [_20,_20,_20,_20,_20];
_17 = _3.0 & _6.0;
_23 = '\u{56fc6}';
Goto(bb6)
}
bb6 = {
_23 = '\u{3db12}';
_10 = _8 | _13;
_10 = _21 | _7;
_15 = -_10;
_17 = 6242_u16 as u8;
_23 = '\u{319d8}';
_17 = _12 as u8;
_6 = _3;
_10 = _15;
_13 = -_15;
_7 = _13 + _15;
RET = [_20,_20,_20,_20,_20];
_18.1 = core::ptr::addr_of!(_19);
_3 = _2;
_8 = false as isize;
_15 = 119_i8 as isize;
_2.0 = !_1;
Goto(bb7)
}
bb7 = {
_7 = _21 | _16;
_11 = !_21;
_14 = 13753755717046770500_u64 as isize;
_24 = Adt61::Variant1 { fld0: _22.0 };
_9 = _10;
SetDiscriminant(_24, 0);
_6 = _2;
_2.0 = 2356397793309952432_usize as u8;
place!(Field::<[bool; 7]>(Variant(_24, 0), 1)) = [true,false,true,false,false,false,false];
place!(Field::<[bool; 7]>(Variant(_24, 0), 1)) = [true,true,true,true,true,true,false];
_13 = 1870878232_u32 as isize;
_18.0 = [13657245143715904447_u64,2836566703258772785_u64];
RET = [_20,_20,_20,_20,_20];
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).1 = _6;
_11 = _7 ^ _9;
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)) = (_19, _6);
_8 = true as isize;
_3.0 = _1;
_14 = _12 as isize;
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)) = (_19, _6);
_17 = !_6.0;
_26 = (-2396263196727270643_i64) as i8;
_10 = 12383038013993003071_usize as isize;
_21 = -_11;
_9 = _7 ^ _5;
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).1 = _3;
_21 = -_9;
_27 = [_12,_4,_12,_4,_12,_4,_12,_12];
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).1.0 = !_6.0;
Goto(bb8)
}
bb8 = {
place!(Field::<[bool; 7]>(Variant(_24, 0), 1)) = [true,false,false,true,true,false,false];
RET = [_20,_20,_20,_20,_20];
_1 = !_3.0;
_27 = [_12,_4,_12,_4,_4,_12,_4,_12];
_3 = (_6.0,);
_10 = _5 ^ _21;
_19 = [_20];
_19 = [_20];
_19 = [_20];
_3.0 = _4 as u8;
_9 = 16634_u16 as isize;
_2.0 = false as u8;
place!(Field::<[bool; 7]>(Variant(_24, 0), 1)) = [true,false,true,true,true,true,true];
_25 = Adt59::Variant0 { fld0: 1141041222166875837_usize,fld1: _20 };
RET = [_20,_20,_20,_20,Field::<i32>(Variant(_25, 0), 1)];
_11 = _20 as isize;
place!(Field::<usize>(Variant(_25, 0), 0)) = !6_usize;
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).0 = _19;
SetDiscriminant(_25, 0);
Goto(bb9)
}
bb9 = {
_30 = [(-636050508835582029_i64),4044914910178594405_i64,6248321148706049797_i64,2208241873016942238_i64];
_11 = _5 ^ _16;
RET = [_20,_20,_20,_20,_20];
_7 = _11;
_17 = _1 | Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2).1.0;
_30 = [6130327338433096457_i64,3224910884907878787_i64,(-279384070124775398_i64),3333115752928051198_i64];
_12 = 422349508_u32 as i128;
_14 = 26816_u16 as isize;
_4 = -_12;
_14 = _5;
_25 = Adt59::Variant0 { fld0: 7_usize,fld1: _20 };
Goto(bb10)
}
bb10 = {
_35 = !3273318423301911653_usize;
_6 = (_1,);
_7 = -_14;
_39 = true as isize;
_37 = true as isize;
_21 = _10 & _11;
_3.0 = !Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2).1.0;
_18.1 = core::ptr::addr_of!(place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).0);
_29 = core::ptr::addr_of!(_28);
_34 = !_21;
_23 = '\u{c3265}';
_6.0 = _1;
_30 = [6618549034513338006_i64,(-234539026046101781_i64),6035534332682959513_i64,(-8096898841260824223_i64)];
_31 = [(-12217_i16),(-7482_i16),(-16404_i16)];
_2 = Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2).1;
_15 = _11 * _5;
_11 = _5;
_6.0 = !_17;
place!(Field::<usize>(Variant(_25, 0), 0)) = !_35;
_40 = true as isize;
Goto(bb11)
}
bb11 = {
_6 = Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2).1;
_8 = -_7;
_32 = _23;
_37 = _11;
_13 = _12 as isize;
_31 = [(-26837_i16),503_i16,22039_i16];
RET = [Field::<i32>(Variant(_25, 0), 1),_20,Field::<i32>(Variant(_25, 0), 1),Field::<i32>(Variant(_25, 0), 1),_20];
_29 = core::ptr::addr_of!((*_29));
_43 = Adt57::Variant0 { fld0: true,fld1: _6.0,fld2: _35,fld3: _19,fld4: _31,fld5: Field::<i32>(Variant(_25, 0), 1),fld6: _30 };
_8 = _21 >> _6.0;
place!(Field::<[i64; 4]>(Variant(_43, 0), 6)) = [(-2759225017448500679_i64),(-4131141147784096515_i64),5154445643717860330_i64,(-5813623332100117631_i64)];
_6.0 = _2.0 >> _21;
_38 = _12 | _4;
_29 = core::ptr::addr_of!(_28);
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)) = (_19, _6);
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).1 = (_17,);
_15 = _34;
_6 = (_2.0,);
_46 = _8;
Goto(bb12)
}
bb12 = {
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)) = (_19, _6);
_48 = [(-21954_i16),(-3416_i16)];
_3 = Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2).1;
_12 = _4;
_20 = Field::<i32>(Variant(_43, 0), 5);
_32 = _23;
place!(Field::<i32>(Variant(_25, 0), 1)) = Field::<i32>(Variant(_43, 0), 5) & Field::<i32>(Variant(_43, 0), 5);
place!(Field::<bool>(Variant(_43, 0), 0)) = !true;
_30 = [(-9143234563933034391_i64),1037725849873899521_i64,(-6816971971677240068_i64),3960736968863621118_i64];
_19 = Field::<[i32; 1]>(Variant(_43, 0), 3);
_18.0 = [14955981830691992852_u64,12329794241575737254_u64];
_31 = [25449_i16,(-24657_i16),11573_i16];
_19 = [Field::<i32>(Variant(_43, 0), 5)];
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).1 = (_2.0,);
_44 = Adt60::Variant3 { fld0: _30,fld1: _23,fld2: Field::<usize>(Variant(_43, 0), 2) };
_23 = Field::<char>(Variant(_44, 3), 1);
_7 = _10 >> _34;
_26 = Field::<usize>(Variant(_43, 0), 2) as i8;
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)) = (Field::<[i32; 1]>(Variant(_43, 0), 3), _2);
SetDiscriminant(_25, 2);
place!(Field::<usize>(Variant(_44, 3), 2)) = !Field::<usize>(Variant(_43, 0), 2);
RET = [_20,_20,Field::<i32>(Variant(_43, 0), 5),Field::<i32>(Variant(_43, 0), 5),_20];
place!(Field::<[i64; 4]>(Variant(_43, 0), 6)) = _30;
place!(Field::<u8>(Variant(_25, 2), 0)) = !_1;
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).1 = (_2.0,);
Call(_12 = core::intrinsics::bswap(_38), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_23 = _32;
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)).0 = [Field::<i32>(Variant(_43, 0), 5)];
_49 = Field::<bool>(Variant(_43, 0), 0);
place!(Field::<[bool; 7]>(Variant(_24, 0), 1)) = [Field::<bool>(Variant(_43, 0), 0),_49,Field::<bool>(Variant(_43, 0), 0),Field::<bool>(Variant(_43, 0), 0),_49,Field::<bool>(Variant(_43, 0), 0),Field::<bool>(Variant(_43, 0), 0)];
_50 = [32232_i16,15291_i16];
place!(Field::<([i32; 1], (u8,))>(Variant(_24, 0), 2)) = (_19, _2);
_53.0 = !Field::<bool>(Variant(_43, 0), 0);
_9 = -_16;
RET = [Field::<i32>(Variant(_43, 0), 5),_20,_20,_20,Field::<i32>(Variant(_43, 0), 5)];
_16 = _34;
_26 = !83_i8;
SetDiscriminant(_43, 0);
_50 = [25681_i16,(-26519_i16)];
Call(_2.0 = core::intrinsics::bswap(_1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_52 = _1 as isize;
_52 = _21 & _16;
_40 = 6278929222347777115_i64 as isize;
place!(Field::<bool>(Variant(_43, 0), 0)) = _11 >= _7;
_25 = Adt59::Variant0 { fld0: Field::<usize>(Variant(_44, 3), 2),fld1: _20 };
_42 = _31;
_31 = [(-2661_i16),6276_i16,(-23569_i16)];
place!(Field::<u8>(Variant(_43, 0), 1)) = !_6.0;
_9 = 7622_i16 as isize;
_7 = -_14;
_23 = _32;
place!(Field::<i32>(Variant(_43, 0), 5)) = Field::<bool>(Variant(_43, 0), 0) as i32;
place!(Field::<usize>(Variant(_25, 0), 0)) = Field::<char>(Variant(_44, 3), 1) as usize;
_45 = _16;
_6 = (Field::<u8>(Variant(_43, 0), 1),);
_53.1 = _26;
SetDiscriminant(_44, 1);
_31 = [(-721_i16),6775_i16,8252_i16];
place!(Field::<[i16; 3]>(Variant(_43, 0), 4)) = [13402_i16,14312_i16,(-7867_i16)];
_3.0 = _35 as u8;
_51 = 3877617867_u32 ^ 479572902_u32;
_2 = (Field::<u8>(Variant(_43, 0), 1),);
_27 = [_4,_4,_38,_12,_12,_4,_4,_38];
place!(Field::<u8>(Variant(_43, 0), 1)) = _35 as u8;
_20 = -Field::<i32>(Variant(_43, 0), 5);
Goto(bb15)
}
bb15 = {
Call(_59 = dump_var(8_usize, 42_usize, Move(_42), 9_usize, Move(_9), 8_usize, Move(_8), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_59 = dump_var(8_usize, 51_usize, Move(_51), 50_usize, Move(_50), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_59 = dump_var(8_usize, 46_usize, Move(_46), 35_usize, Move(_35), 11_usize, Move(_11), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_59 = dump_var(8_usize, 27_usize, Move(_27), 39_usize, Move(_39), 23_usize, Move(_23), 20_usize, Move(_20)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_59 = dump_var(8_usize, 31_usize, Move(_31), 26_usize, Move(_26), 4_usize, Move(_4), 19_usize, Move(_19)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: isize,mut _2: isize,mut _3: u8,mut _4: isize,mut _5: u8,mut _6: isize,mut _7: u8,mut _8: isize,mut _9: isize,mut _10: (u8,),mut _11: isize,mut _12: isize,mut _13: isize,mut _14: (u8,),mut _15: u8,mut _16: isize) -> *const [i32; 1] {
mir! {
type RET = *const [i32; 1];
let _17: ([i32; 1], (u8,));
let _18: Adt51;
let _19: Adt58;
let _20: f64;
let _21: char;
let _22: i128;
let _23: u8;
let _24: bool;
let _25: [bool; 7];
let _26: Adt60;
let _27: [i16; 3];
let _28: (bool, i8, [i32; 5]);
let _29: *const [i32; 1];
let _30: *mut *mut u16;
let _31: (usize, u8);
let _32: [i64; 4];
let _33: usize;
let _34: [i32; 5];
let _35: [i128; 8];
let _36: &'static isize;
let _37: Adt50;
let _38: (u8,);
let _39: isize;
let _40: *mut ((*mut u16, u32, char), *const char, *const char);
let _41: (bool, i8, [i32; 5]);
let _42: [i16; 2];
let _43: ([i32; 1], (u8,));
let _44: (*mut u16, u32, char);
let _45: i128;
let _46: ([i32; 1], (u8,));
let _47: [bool; 7];
let _48: isize;
let _49: f32;
let _50: char;
let _51: [u64; 2];
let _52: ([i32; 1], (u8,));
let _53: ();
let _54: ();
{
_1 = -_4;
_17.0 = [(-929813035_i32)];
_16 = _1;
_10.0 = _3 ^ _7;
_3 = 5005009392113485105_u64 as u8;
_14.0 = _5 << _16;
_10 = (_7,);
_17.1.0 = _7;
_9 = _1;
_2 = -_8;
Goto(bb1)
}
bb1 = {
_2 = false as isize;
_15 = 300890306888786496405551872638910284284_u128 as u8;
RET = core::ptr::addr_of!(_17.0);
_2 = _17.1.0 as isize;
Call(_9 = fn10(_17.1, _17.1.0, _8, _2, _10, _1, _11, _2, _4, _11, _1, _12, _16, _17.1.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17.1 = _14;
Goto(bb3)
}
bb3 = {
_14 = (_17.1.0,);
_12 = _1 - _16;
_15 = !_10.0;
_15 = !_5;
_3 = _10.0;
_14.0 = !_15;
_12 = -_13;
_14.0 = !_10.0;
Goto(bb4)
}
bb4 = {
_20 = (-1450348736256155807_i64) as f64;
RET = core::ptr::addr_of!((*RET));
_8 = _13 & _4;
_5 = 214820303815160435483839404120913578437_u128 as u8;
_7 = _14.0;
_9 = 11373_i16 as isize;
_20 = 198326979392312833016452335169292392611_u128 as f64;
_21 = '\u{d50ef}';
_12 = _2;
_11 = -_1;
_22 = 5509031514554719716_u64 as i128;
_14.0 = !_3;
_17.1.0 = _7;
_14.0 = true as u8;
_17.1.0 = _15;
_17.0 = [441927284_i32];
_9 = (-22_i8) as isize;
_10 = (_15,);
_10 = (_17.1.0,);
_24 = false;
_7 = _3 | _15;
_2 = _11 * _11;
_21 = '\u{8dd9}';
_23 = !_3;
_17.0 = [(-552139571_i32)];
Goto(bb5)
}
bb5 = {
_1 = _16 ^ _13;
_3 = _17.1.0 & _17.1.0;
_17.1 = _10;
_27 = [28798_i16,667_i16,(-14230_i16)];
_6 = _2 ^ _1;
_11 = _24 as isize;
_9 = 3737725895_u32 as isize;
_24 = false;
_17.1.0 = _22 as u8;
_23 = _7 + _7;
_13 = _1;
Call(_10 = fn11(_2, _13, _8, _12, _12, _4, _6, _12, _3, _15, _4, _16, _4, _6, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_31.0 = 5_usize & 5486462992851999183_usize;
_29 = core::ptr::addr_of!((*RET));
_17.0 = [1025794857_i32];
_25 = [_24,_24,_24,_24,_24,_24,_24];
_33 = _31.0;
_13 = _16;
_10 = (_15,);
Call(_14.0 = core::intrinsics::bswap(_3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Goto(bb8)
}
bb8 = {
_32 = [5006109870666144482_i64,6798450610594415387_i64,3261811954872596236_i64,773102811630041229_i64];
_17.1.0 = _7;
_28.0 = _24 | _24;
_31.1 = _8 as u8;
Goto(bb9)
}
bb9 = {
_25 = [_28.0,_28.0,_28.0,_24,_24,_28.0,_28.0];
_7 = !_31.1;
_26 = Adt60::Variant3 { fld0: _32,fld1: _21,fld2: _33 };
_28.0 = _24;
_16 = _6;
_33 = _31.0;
Goto(bb10)
}
bb10 = {
_28.1 = _20 as i8;
_23 = _28.1 as u8;
_5 = (-1193299778_i32) as u8;
SetDiscriminant(_26, 1);
_9 = 208970071989125849071609096196136994388_u128 as isize;
place!(Field::<[i64; 4]>(Variant(_26, 1), 0)) = _32;
_17.1.0 = !_15;
_28.0 = _24 | _24;
_27 = [8708_i16,(-27082_i16),(-27299_i16)];
_33 = _2 as usize;
_28.2 = [336133179_i32,440606433_i32,1150701700_i32,(-2033120778_i32),1897619209_i32];
_37.fld2 = _17.1;
_8 = _28.1 as isize;
_17.1 = _10;
_28.1 = 46_i8;
_25 = [_28.0,_28.0,_28.0,_28.0,_28.0,_24,_24];
_35 = [_22,_22,_22,_22,_22,_22,_22,_22];
_29 = RET;
_37.fld2.0 = !_31.1;
_10 = (_17.1.0,);
_33 = !_31.0;
place!(Field::<isize>(Variant(_26, 1), 2)) = _1;
_17.1.0 = _7;
_11 = _12;
Call(_31.1 = core::intrinsics::bswap(_10.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_35 = [_22,_22,_22,_22,_22,_22,_22,_22];
Goto(bb12)
}
bb12 = {
_20 = _28.1 as f64;
_14.0 = _5;
_14.0 = _17.1.0 << _2;
_37.fld4 = (*RET);
_32 = [3059007813401275042_i64,(-659389974479653830_i64),1046305901240498345_i64,(-8078654117459435280_i64)];
_4 = !_13;
place!(Field::<[i64; 4]>(Variant(_26, 1), 0)) = [5239529487212269524_i64,5633889274656004245_i64,(-4487989922678682026_i64),(-1760223849312641916_i64)];
_34 = [(-1008637152_i32),(-297082101_i32),830386552_i32,(-777591838_i32),(-2106158453_i32)];
_39 = _4 >> _14.0;
_11 = _16;
_31.0 = _24 as usize;
_31.0 = _33 * _33;
_36 = &_16;
_28.2 = [(-84530481_i32),1622919040_i32,1463372430_i32,773676515_i32,1002291541_i32];
_16 = 14080986808442270861_u64 as isize;
_37.fld3 = 3339_u16;
_44.2 = _21;
Goto(bb13)
}
bb13 = {
_25 = [_28.0,_24,_24,_28.0,_28.0,_24,_28.0];
Call(_29 = fn12(_14.0, _15), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_43 = _17;
_31 = (_33, _15);
_28.0 = !_24;
_31.1 = _44.2 as u8;
_42 = [(-13371_i16),27017_i16];
_10.0 = _3;
_41.2 = [430736494_i32,(-1726437849_i32),(-1843516195_i32),868475524_i32,534342887_i32];
_29 = RET;
_43.1.0 = _14.0;
_43.1.0 = _14.0;
_46.1.0 = _37.fld2.0;
RET = _29;
_41.2 = [(-1974712660_i32),(-903776528_i32),2040426550_i32,1549265468_i32,1307031274_i32];
_17 = (_43.0, _37.fld2);
_49 = 941761367_u32 as f32;
_10 = (_17.1.0,);
_29 = RET;
_28.2 = [964135317_i32,1245727936_i32,(-646989974_i32),1091570542_i32,1326531145_i32];
_32 = Field::<[i64; 4]>(Variant(_26, 1), 0);
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(9_usize, 42_usize, Move(_42), 4_usize, Move(_4), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(9_usize, 15_usize, Move(_15), 22_usize, Move(_22), 28_usize, Move(_28), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(9_usize, 10_usize, Move(_10), 24_usize, Move(_24), 11_usize, Move(_11), 33_usize, Move(_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(9_usize, 31_usize, Move(_31), 14_usize, Move(_14), 32_usize, Move(_32), 27_usize, Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: (u8,),mut _2: u8,mut _3: isize,mut _4: isize,mut _5: (u8,),mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: u8) -> isize {
mir! {
type RET = isize;
let _15: bool;
let _16: (i8, f32, *mut u16);
let _17: f64;
let _18: Adt55;
let _19: ([i32; 1], (u8,));
let _20: ();
let _21: ();
{
RET = -_7;
_6 = !_7;
_8 = -_7;
_9 = _11 << _5.0;
RET = -_7;
_6 = -RET;
_14 = '\u{ae10}' as u8;
_16.0 = -(-114_i8);
_5 = (_1.0,);
_10 = -_11;
_4 = _9;
_13 = _10;
_18.fld1.fld2 = (_1.0,);
_18.fld3 = !_5.0;
RET = _13 >> _11;
_18.fld4 = 1513588188817742319_u64;
_8 = _12;
RET = _3 * _7;
_15 = false;
_5.0 = _1.0 * _18.fld1.fld2.0;
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(10_usize, 8_usize, Move(_8), 13_usize, Move(_13), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(10_usize, 9_usize, Move(_9), 1_usize, Move(_1), 6_usize, Move(_6), 21_usize, _21), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: u8,mut _10: u8,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize) -> (u8,) {
mir! {
type RET = (u8,);
let _16: u32;
let _17: bool;
let _18: i32;
let _19: u16;
let _20: i32;
let _21: bool;
let _22: f64;
let _23: f32;
let _24: u16;
let _25: [u64; 2];
let _26: u8;
let _27: *const char;
let _28: ();
let _29: ();
{
_12 = _14 * _14;
_9 = (-51_i8) as u8;
_15 = _13 | _8;
_16 = 339730033_u32;
_12 = (-702025145_i32) as isize;
_9 = (-109191307440126551441990238356283154531_i128) as u8;
RET = (_10,);
_1 = -_8;
_10 = 2181_i16 as u8;
_12 = -_11;
_5 = '\u{59c12}' as isize;
_7 = _13;
_4 = _11 - _11;
_12 = _14;
RET = (_10,);
_16 = 1938659503_u32;
_17 = false;
_15 = !_14;
_7 = _3;
_5 = _4 | _6;
Goto(bb1)
}
bb1 = {
_19 = !62632_u16;
_9 = RET.0;
_20 = (-316477217_i32) | 1285812221_i32;
RET.0 = _9 + _9;
_1 = 51_i8 as isize;
RET = (_9,);
_11 = _17 as isize;
_8 = _13;
_5 = _17 as isize;
match _16 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
1938659503 => bb8,
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
_9 = !_10;
_2 = _16 as isize;
match _16 {
0 => bb3,
1938659503 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
RET.0 = _10;
RET = (_9,);
_19 = 60489_u16 << _13;
_13 = _14 | _15;
_5 = !_14;
_15 = -_4;
Goto(bb11)
}
bb11 = {
_12 = -_7;
_6 = 183912714292909339587157971764513897854_u128 as isize;
_6 = _7 * _15;
_3 = -_13;
_10 = _9;
_14 = 15304147399163936948_usize as isize;
Goto(bb12)
}
bb12 = {
_17 = true;
_1 = 3592462959541377731_u64 as isize;
RET = (_10,);
_14 = -_12;
_4 = _8;
_12 = -_4;
RET.0 = '\u{75a2c}' as u8;
_11 = _13 >> _6;
_10 = '\u{fb896}' as u8;
match _16 {
0 => bb1,
1 => bb8,
2 => bb11,
3 => bb4,
4 => bb10,
5 => bb6,
6 => bb13,
1938659503 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
RET.0 = _10;
RET = (_9,);
_19 = 60489_u16 << _13;
_13 = _14 | _15;
_5 = !_14;
_15 = -_4;
Goto(bb11)
}
bb15 = {
_18 = _20;
_4 = _12;
_23 = 6679306893608023177_i64 as f32;
_2 = _15 | _4;
RET = (_9,);
_22 = 24952_i16 as f64;
RET.0 = _10 & _10;
_8 = (-47_i8) as isize;
_5 = _7 | _6;
_12 = -_3;
_22 = _13 as f64;
_9 = _10 - _10;
_9 = RET.0 >> _14;
_18 = _20 & _20;
_23 = 213470667220214044725529699450975293373_u128 as f32;
_3 = !_12;
_13 = -_14;
_16 = 2028797212_u32 - 801427874_u32;
_12 = _11;
RET = (_9,);
RET.0 = '\u{d3cc4}' as u8;
_1 = _11 + _2;
_13 = -_5;
_16 = !2131415407_u32;
_7 = -_5;
_21 = !_17;
RET.0 = !_9;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(11_usize, 10_usize, Move(_10), 1_usize, Move(_1), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(11_usize, 12_usize, Move(_12), 16_usize, Move(_16), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(11_usize, 5_usize, Move(_5), 19_usize, Move(_19), 29_usize, _29, 29_usize, _29), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u8,mut _2: u8) -> *const [i32; 1] {
mir! {
type RET = *const [i32; 1];
let _3: Adt57;
let _4: i32;
let _5: [i16; 2];
let _6: u32;
let _7: [i32; 5];
let _8: f64;
let _9: f32;
let _10: (u8,);
let _11: bool;
let _12: Adt48;
let _13: u16;
let _14: (*const i16,);
let _15: [i16; 2];
let _16: [i16; 3];
let _17: char;
let _18: i32;
let _19: ([u64; 2], *const [i32; 1]);
let _20: usize;
let _21: i128;
let _22: f64;
let _23: u32;
let _24: i32;
let _25: i8;
let _26: [i32; 5];
let _27: [i32; 1];
let _28: u16;
let _29: f64;
let _30: bool;
let _31: Adt57;
let _32: [i32; 5];
let _33: ();
let _34: ();
{
_2 = 3_i8 as u8;
_1 = '\u{a9a21}' as u8;
_1 = true as u8;
_2 = 25720_u16 as u8;
_1 = _2;
_1 = _2 >> _2;
_4 = 2889392892271200778_u64 as i32;
_1 = _2 + _2;
_4 = 542855568_i32 << _1;
_1 = !_2;
_4 = false as i32;
_2 = 26540_u16 as u8;
_1 = _2;
_4 = 2085703742_i32 - (-128208497_i32);
_2 = !_1;
Goto(bb1)
}
bb1 = {
_4 = 805247385_i32;
_1 = _2;
_5 = [30208_i16,(-3197_i16)];
_5 = [(-30623_i16),(-8048_i16)];
_1 = _2 * _2;
_6 = 3378782386_u32;
_5 = [(-4854_i16),18043_i16];
_5 = [11814_i16,7752_i16];
_1 = 45_i8 as u8;
_5 = [(-4155_i16),3158_i16];
_2 = _1;
_10 = (_1,);
_1 = !_2;
_11 = true ^ false;
_1 = !_2;
_2 = !_1;
_2 = !_1;
_2 = !_10.0;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
805247385 => bb8,
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
_10.0 = _2 | _2;
_6 = 948316573_u32;
Goto(bb9)
}
bb9 = {
_9 = _10.0 as f32;
_7 = [_4,_4,_4,_4,_4];
_8 = 9585526767840091249_u64 as f64;
_6 = !1696640485_u32;
_10 = (_1,);
_1 = _10.0 ^ _2;
_4 = -1240582421_i32;
_5 = [(-8496_i16),(-23855_i16)];
_5 = [(-29568_i16),5753_i16];
_16 = [17608_i16,20047_i16,(-15641_i16)];
Call(_5 = core::intrinsics::transmute(_6), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10.0 = _9 as u8;
_11 = true;
_17 = '\u{58c6b}';
_10 = (_1,);
_4 = !(-1512309613_i32);
_10.0 = !_2;
_11 = true;
_7 = [_4,_4,_4,_4,_4];
_1 = !_2;
_9 = 17003332603721624440_usize as f32;
_7 = [_4,_4,_4,_4,_4];
_10 = (_2,);
_16 = [30032_i16,8154_i16,2826_i16];
_15 = _5;
_4 = 1540859500_i32;
_6 = 2523463132_u32;
_5 = _15;
_8 = (-85704641424744691554029530936808613849_i128) as f64;
_16 = [5703_i16,(-14680_i16),9500_i16];
_10 = (_1,);
_4 = 15814743665621888811_u64 as i32;
_11 = !true;
_18 = !_4;
_15 = _5;
_17 = '\u{8770}';
_4 = _18;
_4 = -_18;
Goto(bb11)
}
bb11 = {
_19.0 = [12702821408814997358_u64,16556168251844284830_u64];
_17 = '\u{107d5e}';
_19.0 = [1985933965091985632_u64,12574428237794066823_u64];
_16 = [(-16468_i16),27111_i16,4315_i16];
_13 = !46738_u16;
_20 = _2 as usize;
_5 = _15;
Goto(bb12)
}
bb12 = {
_16 = [19937_i16,32680_i16,23946_i16];
_2 = !_1;
_6 = 1996227215_u32 ^ 53088832_u32;
_10.0 = _2 << _20;
_18 = -_4;
_6 = !1361502379_u32;
Goto(bb13)
}
bb13 = {
_4 = _18;
_6 = !2854895194_u32;
_2 = _6 as u8;
_22 = _8;
_10 = (_2,);
_23 = 9441533810258151573_u64 as u32;
_24 = _4;
_1 = (-120_i8) as u8;
RET = core::ptr::addr_of!(_27);
_11 = true;
_17 = '\u{31aa}';
Goto(bb14)
}
bb14 = {
_23 = !_6;
_15 = [(-4272_i16),(-23707_i16)];
_25 = -(-41_i8);
_9 = _20 as f32;
_27 = [_24];
_1 = _10.0 - _10.0;
_19.0 = [1372555827584314847_u64,978735994804753706_u64];
_29 = _25 as f64;
_1 = _2 + _10.0;
_19.1 = core::ptr::addr_of!((*RET));
_23 = _6 - _6;
_7 = [_4,_24,_4,_18,_24];
_16 = [(-9432_i16),(-31789_i16),(-31752_i16)];
RET = core::ptr::addr_of!((*RET));
_26 = [_18,_4,_24,_4,_4];
_13 = 103856635285634003_u64 as u16;
_9 = 9896927505099895374_u64 as f32;
_2 = !_1;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(12_usize, 20_usize, Move(_20), 27_usize, Move(_27), 16_usize, Move(_16), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(12_usize, 15_usize, Move(_15), 1_usize, Move(_1), 18_usize, Move(_18), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(12_usize, 2_usize, Move(_2), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: u8,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize) -> *const i16 {
mir! {
type RET = *const i16;
let _9: bool;
let _10: bool;
let _11: [i32; 5];
let _12: [u64; 2];
let _13: f32;
let _14: [i64; 4];
let _15: *mut (*mut u16, u32, char);
let _16: [i128; 8];
let _17: (bool, i8, [i32; 5]);
let _18: isize;
let _19: usize;
let _20: f32;
let _21: u8;
let _22: char;
let _23: u8;
let _24: (usize, u8);
let _25: Adt47;
let _26: char;
let _27: i8;
let _28: isize;
let _29: u16;
let _30: [i32; 1];
let _31: [bool; 7];
let _32: ((*mut u16, u32, char), *const char, *const char);
let _33: [u64; 2];
let _34: (*mut u16, u32, char);
let _35: isize;
let _36: ([u64; 2], *const [i32; 1]);
let _37: (u8,);
let _38: ([i32; 1], (u8,));
let _39: [i64; 4];
let _40: bool;
let _41: Adt51;
let _42: i16;
let _43: char;
let _44: isize;
let _45: (usize, u8);
let _46: Adt51;
let _47: [i64; 4];
let _48: u8;
let _49: (u8,);
let _50: Adt53;
let _51: Adt50;
let _52: ();
let _53: ();
{
_7 = _6;
_5 = (-8688668096154171629_i64) as isize;
_3 = _2;
_8 = !_6;
_2 = _6 << _7;
_6 = _4;
_7 = _8 - _8;
_6 = 26052_i16 as isize;
_4 = _2 * _3;
_3 = !_8;
_9 = true ^ false;
_4 = _3 + _8;
_9 = !false;
_5 = _3 << _2;
_4 = _3 ^ _5;
_1 = 203_u8;
_4 = (-9193844594358170430_i64) as isize;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
203 => bb7,
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
_5 = _8 - _8;
_3 = !_8;
_6 = _3;
_1 = 140_u8;
_5 = -_6;
_7 = _8 ^ _6;
match _1 {
0 => bb2,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
140 => bb14,
_ => bb13
}
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
_7 = _2 >> _5;
_7 = _9 as isize;
_12 = [606217975666647315_u64,18213505812855560391_u64];
_7 = _2 - _2;
_13 = 6082814960237856425_usize as f32;
_17.1 = _9 as i8;
_5 = !_2;
_3 = !_5;
_1 = 220_u8;
_17.2 = [(-440199684_i32),563130868_i32,2018359225_i32,(-328464588_i32),(-2019074904_i32)];
_1 = 819832831_i32 as u8;
_9 = !false;
_12 = [15516457121980621409_u64,6095330847232097246_u64];
_16 = [165524070025810992608027508550353939220_i128,(-161491303606933248872756822194931460756_i128),(-87277082877447110901326503694556889502_i128),140629291263322820072648911727471761369_i128,85627008898607776129506373734378950188_i128,(-118563764841045845477234470726132440386_i128),95436804424932675302455745323917753000_i128,(-147741644590454390143144397594320776527_i128)];
Goto(bb15)
}
bb15 = {
_12 = [4327931795272634022_u64,9630725188898516482_u64];
_4 = _13 as isize;
_5 = _6 - _2;
_5 = _6;
_16 = [38319790004437009047206213279282375734_i128,89655301171029353642771309658370318320_i128,(-99351238994445223703451110530483339125_i128),2242122775556365408974206149250103482_i128,62046311593885193956555002034796869733_i128,(-70574257986079224400620355901443495172_i128),(-133050608614455767874780252651292902755_i128),(-114342244368592789185390007024706716337_i128)];
_18 = !_5;
_12 = [17963872368165178733_u64,10181002556523716216_u64];
_17.0 = _9;
_14 = [3484573775909080124_i64,(-2442272361932833263_i64),(-5713526115120476372_i64),(-640090152977821226_i64)];
_13 = 744_i16 as f32;
_17.1 = 13277_i16 as i8;
_6 = _7 & _7;
_4 = _9 as isize;
_5 = _7 << _7;
_17.2 = [(-1462121110_i32),(-1609447805_i32),1407018467_i32,535145857_i32,285035485_i32];
_20 = _13 + _13;
_6 = !_8;
_12 = [13010714785352769046_u64,2273477680829921753_u64];
_14 = [7272173725723170350_i64,3503096088183807364_i64,(-3540709738544831030_i64),2221877557374406277_i64];
_16 = [159901559101978568009408692517877321692_i128,(-127079023912467497330177762105598422744_i128),34896757515927545445897322160480621644_i128,155960737776398270724177193653590638064_i128,(-113162958213431579380304198990582191948_i128),5391078925510610286940007570881781087_i128,(-129968423007076581174480020067319779105_i128),84251299314119439141611735132274386074_i128];
Call(_20 = fn14(_2, _18, _6, _18, _18, _3, _18, _2, _5, _5, _6), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_24.1 = (-913976627870644908_i64) as u8;
_18 = !_6;
_14 = [(-3146311760768475546_i64),(-5812941693899729272_i64),6583329494450226531_i64,(-7541310697031855716_i64)];
_22 = '\u{10d299}';
_10 = _17.0;
_17.2 = [538471715_i32,(-993860460_i32),(-1620851120_i32),829487805_i32,438962389_i32];
_3 = _6;
_1 = _18 as u8;
_24.1 = _1 << _18;
_24 = (5_usize, _1);
_4 = (-729667418_i32) as isize;
_19 = _22 as usize;
_1 = _24.1;
_24.1 = _1 >> _5;
_23 = !_24.1;
_17.2 = [991956985_i32,(-399626387_i32),(-989931187_i32),(-2021731965_i32),1387765029_i32];
_19 = _24.0 & _24.0;
_19 = _24.0 & _24.0;
_11 = _17.2;
_26 = _22;
_28 = 1085558522_u32 as isize;
_3 = _10 as isize;
_17.1 = 84_i8 ^ 58_i8;
_14 = [429867440374707695_i64,(-2609789105157756621_i64),(-1965681427763737102_i64),(-7485745897862977459_i64)];
_17.2 = [2103347165_i32,1648884974_i32,(-1663038740_i32),1283472076_i32,(-1299546878_i32)];
_23 = _1 << _1;
_8 = 3820710261_u32 as isize;
match _24.0 {
0 => bb9,
5 => bb17,
_ => bb6
}
}
bb17 = {
_16 = [(-111540837956527797752151337150236854224_i128),(-138729419913833833659346853880466068803_i128),(-94429925575774581839566427221598637820_i128),101023518933095218736959304398204739844_i128,98459253907525637848342196297356154463_i128,116939115584919725055173633312713609909_i128,(-163929230997274681196129087232878429635_i128),(-70593509327282724850437950748086058421_i128)];
_28 = _2;
Goto(bb18)
}
bb18 = {
_18 = _6 | _28;
_24 = (_19, _1);
_27 = _17.1 << _28;
_18 = -_5;
_16 = [23907052324586228495487028897036923004_i128,125032261208273003342636757994470748785_i128,(-82852461617228398144250714360157184395_i128),(-127256426344789096386919374728265939657_i128),16967771652094123107746234734502111149_i128,149855195212181288735150620752268964803_i128,28269148247235413668491351293257811735_i128,7574913635442134236364505938135839326_i128];
_21 = _6 as u8;
_1 = 14373554387959481042_u64 as u8;
_17.1 = _27;
_17 = (_9, _27, _11);
_8 = -_2;
_14 = [1971037906435666560_i64,7660759022350723357_i64,(-7668214334119386870_i64),(-5483274770379824555_i64)];
_29 = !50238_u16;
_2 = _28 << _24.1;
_27 = _24.0 as i8;
_10 = _5 >= _8;
_30 = [986318202_i32];
_16 = [(-83955406606060533771870328200188840767_i128),67374529871841478237877457547246055924_i128,82103056717867985916519341247767070964_i128,(-74414351852392181259356267442107739636_i128),14935798015362946548738834400896647784_i128,(-100943242100918251390758036393031170491_i128),49447274617605725980498863332742547476_i128,74684130963765330623042294293398025595_i128];
_9 = _10;
_16 = [51812476056204030600672709084627842165_i128,(-4866450271238297490265249583121654350_i128),(-120740976164185310664791310901274531913_i128),(-168119650276518534381766095810758347725_i128),143116071826218773963767778875810581257_i128,(-165318705536630636619363151954689615625_i128),(-65296343961599207393376618022775380666_i128),(-153513265798634526956955232414594764156_i128)];
_8 = _7 >> _23;
Goto(bb19)
}
bb19 = {
_20 = _13;
Goto(bb20)
}
bb20 = {
_31 = [_10,_9,_10,_9,_10,_9,_9];
_10 = !_9;
_16 = [21270718484975728998391402895148249362_i128,95236220318105147700128682635014562098_i128,(-104245060661967119533032914105533342901_i128),8720690145202495922697204726948380139_i128,16251287789070747160255483998531153507_i128,(-47762618645685729857616766709468494211_i128),(-56274252081764705101965239577032884237_i128),(-79389743862189712242876902943551000496_i128)];
_32.0.2 = _22;
_32.2 = core::ptr::addr_of!(_32.0.2);
_32.0.1 = !1696990171_u32;
_17.2 = [(-281041670_i32),1405642813_i32,(-1601645063_i32),(-1312472697_i32),1029306742_i32];
_24.0 = !_19;
_12 = [1014536958662316489_u64,12073551747660915544_u64];
_29 = 27979_u16 - 40639_u16;
_1 = _24.1;
_10 = !_9;
_32.0.0 = core::ptr::addr_of_mut!(_29);
_34.2 = _22;
_14 = [(-9018784304796380945_i64),(-7247420638003951701_i64),6449026813730995982_i64,287602325625631115_i64];
_29 = _27 as u16;
_8 = _18;
_15 = core::ptr::addr_of_mut!(_32.0);
_32.1 = _32.2;
_34.2 = _32.0.2;
Call(_19 = fn15(_32.0.0, _7, _23, _27, _23, _17, _8, _21, _21, _23, _31, _27, _29), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_10 = _9;
_33 = [12373956389196272403_u64,10532154352187068804_u64];
_17 = (_9, _27, _11);
_35 = _5 ^ _8;
_12 = [160288861894683105_u64,14138432919768509164_u64];
_16 = [109391488589652377356726821187217890005_i128,(-35781694341035922357119571725802562688_i128),63566673944864537994514921074263320209_i128,149047553521244093642848700691045730466_i128,75769517025023228002919874806306439534_i128,(-97280066212682132964501859869336558047_i128),142664822359873020563431388330015516661_i128,(-37477939941770484407115570077705330518_i128)];
_17 = (_9, _27, _11);
_32.0.0 = core::ptr::addr_of_mut!(_29);
_21 = _1 - _1;
_8 = _28 * _35;
_2 = _35 << _5;
_20 = _13;
_17 = (_9, _27, _11);
_28 = 13448261916772637329_u64 as isize;
_3 = _8 & _18;
_37.0 = _1;
_37.0 = !_21;
_5 = 8056532721522409351_u64 as isize;
_28 = -_35;
_38.1.0 = (-10821_i16) as u8;
_3 = _8 ^ _2;
_10 = !_9;
_34.2 = (*_15).2;
_40 = _17.0 | _10;
_32.0.2 = _22;
match _16[_19] {
0 => bb4,
302804426979167979056259037354062880938 => bb23,
_ => bb22
}
}
bb22 = {
Return()
}
bb23 = {
_36.1 = core::ptr::addr_of!(_30);
_38.1.0 = !_37.0;
_29 = _1 as u16;
_10 = _9;
_32.1 = core::ptr::addr_of!(_22);
_20 = _13 * _13;
_43 = _32.0.2;
_17.2 = [597356116_i32,1296432584_i32,92336176_i32,1621116834_i32,921644592_i32];
_32.0.2 = _26;
_42 = -32140_i16;
_29 = !33126_u16;
_36.0 = [13969299436282952002_u64,12538151900178022837_u64];
_29 = 38712_u16;
_34.0 = core::ptr::addr_of_mut!(_29);
_32.1 = _32.2;
Goto(bb24)
}
bb24 = {
_38.0 = [752689598_i32];
_14 = [(-4477842231399277240_i64),(-715770230461381547_i64),(-3309761615569217555_i64),(-2203109430629160183_i64)];
_6 = _20 as isize;
_33 = [10448792809791841192_u64,13838818289761988368_u64];
_6 = _7 ^ _8;
_19 = _24.0 >> _17.1;
_39 = _14;
_8 = _7;
_40 = _17.1 == _27;
_37 = (_21,);
_31 = [_17.0,_10,_40,_40,_17.0,_10,_17.0];
_24.1 = !_21;
_33 = [2316119814170894922_u64,966746984407313847_u64];
_9 = !_17.0;
_48 = 115999795958295623713448852294881525921_u128 as u8;
_24.1 = _37.0;
_31 = [_9,_10,_9,_9,_17.0,_9,_40];
_3 = _18 >> _1;
_38.1.0 = !_37.0;
_17 = (_40, _27, _11);
_45.1 = _37.0 & _21;
_17.0 = !_9;
_36.1 = core::ptr::addr_of!(_38.0);
match _29 {
0 => bb25,
1 => bb26,
2 => bb27,
3 => bb28,
4 => bb29,
5 => bb30,
38712 => bb32,
_ => bb31
}
}
bb25 = {
Return()
}
bb26 = {
Return()
}
bb27 = {
Return()
}
bb28 = {
_31 = [_10,_9,_10,_9,_10,_9,_9];
_10 = !_9;
_16 = [21270718484975728998391402895148249362_i128,95236220318105147700128682635014562098_i128,(-104245060661967119533032914105533342901_i128),8720690145202495922697204726948380139_i128,16251287789070747160255483998531153507_i128,(-47762618645685729857616766709468494211_i128),(-56274252081764705101965239577032884237_i128),(-79389743862189712242876902943551000496_i128)];
_32.0.2 = _22;
_32.2 = core::ptr::addr_of!(_32.0.2);
_32.0.1 = !1696990171_u32;
_17.2 = [(-281041670_i32),1405642813_i32,(-1601645063_i32),(-1312472697_i32),1029306742_i32];
_24.0 = !_19;
_12 = [1014536958662316489_u64,12073551747660915544_u64];
_29 = 27979_u16 - 40639_u16;
_1 = _24.1;
_10 = !_9;
_32.0.0 = core::ptr::addr_of_mut!(_29);
_34.2 = _22;
_14 = [(-9018784304796380945_i64),(-7247420638003951701_i64),6449026813730995982_i64,287602325625631115_i64];
_29 = _27 as u16;
_8 = _18;
_15 = core::ptr::addr_of_mut!(_32.0);
_32.1 = _32.2;
_34.2 = _32.0.2;
Call(_19 = fn15(_32.0.0, _7, _23, _27, _23, _17, _8, _21, _21, _23, _31, _27, _29), ReturnTo(bb21), UnwindUnreachable())
}
bb29 = {
Return()
}
bb30 = {
_12 = [4327931795272634022_u64,9630725188898516482_u64];
_4 = _13 as isize;
_5 = _6 - _2;
_5 = _6;
_16 = [38319790004437009047206213279282375734_i128,89655301171029353642771309658370318320_i128,(-99351238994445223703451110530483339125_i128),2242122775556365408974206149250103482_i128,62046311593885193956555002034796869733_i128,(-70574257986079224400620355901443495172_i128),(-133050608614455767874780252651292902755_i128),(-114342244368592789185390007024706716337_i128)];
_18 = !_5;
_12 = [17963872368165178733_u64,10181002556523716216_u64];
_17.0 = _9;
_14 = [3484573775909080124_i64,(-2442272361932833263_i64),(-5713526115120476372_i64),(-640090152977821226_i64)];
_13 = 744_i16 as f32;
_17.1 = 13277_i16 as i8;
_6 = _7 & _7;
_4 = _9 as isize;
_5 = _7 << _7;
_17.2 = [(-1462121110_i32),(-1609447805_i32),1407018467_i32,535145857_i32,285035485_i32];
_20 = _13 + _13;
_6 = !_8;
_12 = [13010714785352769046_u64,2273477680829921753_u64];
_14 = [7272173725723170350_i64,3503096088183807364_i64,(-3540709738544831030_i64),2221877557374406277_i64];
_16 = [159901559101978568009408692517877321692_i128,(-127079023912467497330177762105598422744_i128),34896757515927545445897322160480621644_i128,155960737776398270724177193653590638064_i128,(-113162958213431579380304198990582191948_i128),5391078925510610286940007570881781087_i128,(-129968423007076581174480020067319779105_i128),84251299314119439141611735132274386074_i128];
Call(_20 = fn14(_2, _18, _6, _18, _18, _3, _18, _2, _5, _5, _6), ReturnTo(bb16), UnwindUnreachable())
}
bb31 = {
_16 = [(-111540837956527797752151337150236854224_i128),(-138729419913833833659346853880466068803_i128),(-94429925575774581839566427221598637820_i128),101023518933095218736959304398204739844_i128,98459253907525637848342196297356154463_i128,116939115584919725055173633312713609909_i128,(-163929230997274681196129087232878429635_i128),(-70593509327282724850437950748086058421_i128)];
_28 = _2;
Goto(bb18)
}
bb32 = {
_49 = (_38.1.0,);
_47 = [8822956586100025946_i64,(-1357101848254703569_i64),(-1944066548219432642_i64),839132522874538188_i64];
RET = core::ptr::addr_of!(_42);
_48 = _23 | _45.1;
_34.1 = 967673593_i32 as u32;
_24.1 = !_21;
_18 = _2;
_8 = _9 as isize;
_39 = _14;
_48 = !_24.1;
_40 = !_9;
_26 = _22;
Goto(bb33)
}
bb33 = {
Call(_52 = dump_var(13_usize, 47_usize, Move(_47), 1_usize, Move(_1), 21_usize, Move(_21), 42_usize, Move(_42)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_52 = dump_var(13_usize, 11_usize, Move(_11), 29_usize, Move(_29), 27_usize, Move(_27), 40_usize, Move(_40)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_52 = dump_var(13_usize, 26_usize, Move(_26), 22_usize, Move(_22), 24_usize, Move(_24), 5_usize, Move(_5)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_52 = dump_var(13_usize, 30_usize, Move(_30), 35_usize, Move(_35), 19_usize, Move(_19), 8_usize, Move(_8)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_52 = dump_var(13_usize, 39_usize, Move(_39), 2_usize, Move(_2), 14_usize, Move(_14), 53_usize, _53), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize) -> f32 {
mir! {
type RET = f32;
let _12: Adt60;
let _13: Adt57;
let _14: [i64; 4];
let _15: [i16; 3];
let _16: ();
let _17: ();
{
_8 = _11 + _9;
_5 = _10;
_10 = -_3;
_6 = _5 + _5;
_2 = _8 << _4;
_9 = _6 & _1;
_3 = 6260379042932066410_i64 as isize;
_14 = [(-716885613479087999_i64),8703162449742054544_i64,4894191393384045178_i64,(-8926183198913160361_i64)];
RET = _11 as f32;
_1 = (-19331_i16) as isize;
_12 = Adt60::Variant3 { fld0: _14,fld1: '\u{b6df}',fld2: 150373767897479279_usize };
_4 = 48_u8 as isize;
place!(Field::<[i64; 4]>(Variant(_12, 3), 0)) = _14;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(14_usize, 11_usize, Move(_11), 4_usize, Move(_4), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(14_usize, 7_usize, Move(_7), 8_usize, Move(_8), 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: *mut u16,mut _2: isize,mut _3: u8,mut _4: i8,mut _5: u8,mut _6: (bool, i8, [i32; 5]),mut _7: isize,mut _8: u8,mut _9: u8,mut _10: u8,mut _11: [bool; 7],mut _12: i8,mut _13: u16) -> usize {
mir! {
type RET = usize;
let _14: [i16; 2];
let _15: (u8,);
let _16: (u8,);
let _17: u128;
let _18: (i8, f32, *mut u16);
let _19: i16;
let _20: Adt50;
let _21: isize;
let _22: Adt61;
let _23: i128;
let _24: [i128; 8];
let _25: [u64; 2];
let _26: i32;
let _27: [i16; 2];
let _28: isize;
let _29: (usize, u8);
let _30: ();
let _31: ();
{
RET = !12359441056258966470_usize;
_13 = 63977_u16 - 43989_u16;
_4 = !_6.1;
_2 = _7 >> _10;
Call(_10 = core::intrinsics::transmute(_12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = [15925_i16,24851_i16];
_3 = 1347001701_i32 as u8;
_9 = _10;
_5 = _8 + _8;
_3 = _9;
_8 = !_9;
_6.0 = false;
_5 = !_9;
_15.0 = _9 * _8;
_6.0 = !true;
Goto(bb2)
}
bb2 = {
_15.0 = _6.0 as u8;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_15.0 = _9;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_16 = (_9,);
_16.0 = RET as u8;
_5 = _9;
_8 = _5 & _10;
_18.0 = _4;
_17 = 14644880541910185209_u64 as u128;
_2 = _7 >> _7;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
Goto(bb3)
}
bb3 = {
_8 = _17 as u8;
_18.1 = (-27848_i16) as f32;
_19 = -(-18228_i16);
_18.2 = _1;
_16.0 = _10 - _5;
_18.2 = _1;
_3 = '\u{7bc46}' as u8;
_16 = (_9,);
_13 = 10142_u16;
_6.0 = !false;
_4 = !_18.0;
_6.0 = true;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
RET = !3730237836640892049_usize;
_6.1 = _6.0 as i8;
_20.fld3 = !_13;
_5 = !_10;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_20.fld4 = [1317410502_i32];
_20.fld2 = _15;
_13 = !_20.fld3;
_9 = _10;
RET = (-1131375907_i32) as usize;
_21 = _2 ^ _2;
Goto(bb4)
}
bb4 = {
_7 = _16.0 as isize;
Goto(bb5)
}
bb5 = {
_6.0 = _5 <= _10;
_16 = _15;
_14 = [_19,_19];
_6.2 = [(-488088395_i32),(-616365765_i32),616953488_i32,(-405750272_i32),(-1942504087_i32)];
_19 = !(-11961_i16);
_16.0 = !_9;
RET = 7_usize;
_18.1 = 1669241005_u32 as f32;
_14 = [_19,_19];
_18.2 = core::ptr::addr_of_mut!(_13);
_8 = _20.fld2.0 - _5;
_6.2 = [(-547566259_i32),(-1870978762_i32),1841770894_i32,1486122490_i32,317548933_i32];
_18.1 = 124179688_i32 as f32;
_16.0 = !_5;
_8 = _10;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_4 = _18.0;
_14 = [_19,_19];
_9 = !_5;
_3 = !_8;
_18.1 = _17 as f32;
_14 = [_19,_19];
_14 = [_19,_19];
_18.1 = 7615388023683100405_u64 as f32;
_6.0 = true & true;
_8 = _10;
_15.0 = !_3;
_3 = 6449332865528174415_i64 as u8;
Goto(bb6)
}
bb6 = {
_1 = _18.2;
_18.2 = core::ptr::addr_of_mut!(_20.fld3);
_24[RET] = (-68008595289872447903928477987146895255_i128);
_26 = (-1786251765_i32);
_12 = _18.0 ^ _4;
_6.1 = _18.0 ^ _12;
_18.1 = _19 as f32;
_8 = !_5;
_8 = !_5;
_18.1 = _20.fld2.0 as f32;
_13 = _20.fld3 << _5;
_10 = !_8;
_4 = !_12;
_25 = [3253003167262116043_u64,16206840439586738581_u64];
_18.2 = core::ptr::addr_of_mut!(_13);
match _26 {
0 => bb2,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463463374607429981959691 => bb14,
_ => bb13
}
}
bb7 = {
_6.0 = _5 <= _10;
_16 = _15;
_14 = [_19,_19];
_6.2 = [(-488088395_i32),(-616365765_i32),616953488_i32,(-405750272_i32),(-1942504087_i32)];
_19 = !(-11961_i16);
_16.0 = !_9;
RET = 7_usize;
_18.1 = 1669241005_u32 as f32;
_14 = [_19,_19];
_18.2 = core::ptr::addr_of_mut!(_13);
_8 = _20.fld2.0 - _5;
_6.2 = [(-547566259_i32),(-1870978762_i32),1841770894_i32,1486122490_i32,317548933_i32];
_18.1 = 124179688_i32 as f32;
_16.0 = !_5;
_8 = _10;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_4 = _18.0;
_14 = [_19,_19];
_9 = !_5;
_3 = !_8;
_18.1 = _17 as f32;
_14 = [_19,_19];
_14 = [_19,_19];
_18.1 = 7615388023683100405_u64 as f32;
_6.0 = true & true;
_8 = _10;
_15.0 = !_3;
_3 = 6449332865528174415_i64 as u8;
Goto(bb6)
}
bb8 = {
_7 = _16.0 as isize;
Goto(bb5)
}
bb9 = {
_8 = _17 as u8;
_18.1 = (-27848_i16) as f32;
_19 = -(-18228_i16);
_18.2 = _1;
_16.0 = _10 - _5;
_18.2 = _1;
_3 = '\u{7bc46}' as u8;
_16 = (_9,);
_13 = 10142_u16;
_6.0 = !false;
_4 = !_18.0;
_6.0 = true;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
RET = !3730237836640892049_usize;
_6.1 = _6.0 as i8;
_20.fld3 = !_13;
_5 = !_10;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_20.fld4 = [1317410502_i32];
_20.fld2 = _15;
_13 = !_20.fld3;
_9 = _10;
RET = (-1131375907_i32) as usize;
_21 = _2 ^ _2;
Goto(bb4)
}
bb10 = {
_15.0 = _6.0 as u8;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_15.0 = _9;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_16 = (_9,);
_16.0 = RET as u8;
_5 = _9;
_8 = _5 & _10;
_18.0 = _4;
_17 = 14644880541910185209_u64 as u128;
_2 = _7 >> _7;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
Goto(bb3)
}
bb11 = {
_14 = [15925_i16,24851_i16];
_3 = 1347001701_i32 as u8;
_9 = _10;
_5 = _8 + _8;
_3 = _9;
_8 = !_9;
_6.0 = false;
_5 = !_9;
_15.0 = _9 * _8;
_6.0 = !true;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_20.fld3 = _13 ^ _13;
_7 = _6.0 as isize;
_20.fld3 = _18.1 as u16;
_4 = _18.0;
_18.0 = _26 as i8;
_6.0 = true;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(15_usize, 2_usize, Move(_2), 25_usize, Move(_25), 13_usize, Move(_13), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(15_usize, 14_usize, Move(_14), 19_usize, Move(_19), 15_usize, Move(_15), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(15_usize, 17_usize, Move(_17), 5_usize, Move(_5), 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i128,mut _2: isize,mut _3: isize,mut _4: i128,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: u8,mut _10: isize,mut _11: isize,mut _12: (*mut u16, u32, char)) -> char {
mir! {
type RET = char;
let _13: [i128; 8];
let _14: char;
let _15: f64;
let _16: bool;
let _17: u128;
let _18: f64;
let _19: i32;
let _20: (bool, i8, [i32; 5]);
let _21: u16;
let _22: (*const i16, i32, *const char);
let _23: [i16; 3];
let _24: [bool; 7];
let _25: (*const i16,);
let _26: f32;
let _27: [u64; 2];
let _28: [bool; 7];
let _29: Adt47;
let _30: (*const i16, i32, *const char);
let _31: (usize, u8);
let _32: (usize, u8);
let _33: (i8, f32, *mut u16);
let _34: ();
let _35: ();
{
_9 = !70_u8;
_1 = !_4;
_9 = (-33_i8) as u8;
RET = _12.2;
_9 = 14106954689376745966_u64 as u8;
_1 = !_4;
_10 = (-7027_i16) as isize;
_12.2 = RET;
_2 = _8 * _11;
_12.2 = RET;
_13 = [_4,_4,_1,_4,_1,_4,_1,_4];
_2 = _12.2 as isize;
_2 = _7 + _8;
_14 = _12.2;
_8 = _3;
_10 = 636981017885470567_u64 as isize;
_16 = !false;
_13 = [_1,_1,_4,_4,_1,_1,_4,_4];
_5 = _7 * _2;
_12.1 = 83_i8 as u32;
_20.2 = [2121034725_i32,840114173_i32,(-833653273_i32),74147841_i32,863130589_i32];
_2 = -_7;
Call(_17 = fn17(_7, _3, _2, _13, _7, _8, _11, _11, _2, _13, _2, _7, _11, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = RET as isize;
_9 = !203_u8;
_12.1 = 1760037277_u32 - 163378854_u32;
_5 = _2;
_13 = [_1,_4,_4,_4,_1,_1,_4,_4];
_6 = !_11;
_7 = _11 >> _6;
_20.1 = 103_i8;
_13 = [_4,_4,_4,_1,_1,_4,_4,_1];
_10 = _6;
_7 = _3;
RET = _14;
_7 = -_11;
_12.1 = 11367_i16 as u32;
_5 = _2 & _2;
Goto(bb2)
}
bb2 = {
_15 = _9 as f64;
_9 = !104_u8;
match _20.1 {
0 => bb1,
1 => bb3,
103 => bb5,
_ => bb4
}
}
bb3 = {
_5 = RET as isize;
_9 = !203_u8;
_12.1 = 1760037277_u32 - 163378854_u32;
_5 = _2;
_13 = [_1,_4,_4,_4,_1,_1,_4,_4];
_6 = !_11;
_7 = _11 >> _6;
_20.1 = 103_i8;
_13 = [_4,_4,_4,_1,_1,_4,_4,_1];
_10 = _6;
_7 = _3;
RET = _14;
_7 = -_11;
_12.1 = 11367_i16 as u32;
_5 = _2 & _2;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_6 = _7 >> _2;
_4 = _1;
RET = _12.2;
_18 = _15;
_21 = !35056_u16;
_22.2 = core::ptr::addr_of!(_14);
_7 = _2;
Call(_2 = core::intrinsics::bswap(_10), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = !_1;
_3 = _7 ^ _2;
_24 = [_16,_16,_16,_16,_16,_16,_16];
_2 = -_3;
_16 = false;
_4 = _1 ^ _1;
_2 = _9 as isize;
_4 = _21 as i128;
_23 = [15345_i16,32430_i16,12206_i16];
_21 = 7773_u16 * 52183_u16;
_1 = -_4;
_11 = -_10;
_19 = (-1315303956_i32) * 2055712750_i32;
_20.0 = _16;
_8 = _3;
match _20.1 {
0 => bb5,
1 => bb2,
103 => bb7,
_ => bb3
}
}
bb7 = {
_26 = (-11027_i16) as f32;
_26 = _12.1 as f32;
_3 = -_8;
_20.1 = 44_i8 | (-15_i8);
_16 = _11 == _3;
_24 = [_16,_16,_16,_16,_16,_16,_16];
_10 = _11 + _7;
_15 = (-1640872234738216984_i64) as f64;
_27 = [7077377659158312807_u64,14538526282613516976_u64];
_12.1 = 242929420_u32 << _8;
_20.0 = !_16;
_8 = _7 | _7;
_22.1 = _19 & _19;
_12.0 = core::ptr::addr_of_mut!(_21);
_6 = _5;
_18 = _9 as f64;
_17 = 294295950532316179839993725840394012332_u128;
_26 = (-5980197691361337480_i64) as f32;
_13 = [_4,_4,_1,_1,_4,_4,_4,_4];
_5 = _9 as isize;
_20.0 = _11 != _3;
RET = _14;
_7 = _10 - _6;
_22.2 = core::ptr::addr_of!(_14);
_14 = RET;
_19 = _9 as i32;
_24 = [_20.0,_16,_20.0,_20.0,_20.0,_16,_20.0];
match _17 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
294295950532316179839993725840394012332 => bb16,
_ => bb15
}
}
bb8 = {
_4 = !_1;
_3 = _7 ^ _2;
_24 = [_16,_16,_16,_16,_16,_16,_16];
_2 = -_3;
_16 = false;
_4 = _1 ^ _1;
_2 = _9 as isize;
_4 = _21 as i128;
_23 = [15345_i16,32430_i16,12206_i16];
_21 = 7773_u16 * 52183_u16;
_1 = -_4;
_11 = -_10;
_19 = (-1315303956_i32) * 2055712750_i32;
_20.0 = _16;
_8 = _3;
match _20.1 {
0 => bb5,
1 => bb2,
103 => bb7,
_ => bb3
}
}
bb9 = {
_6 = _7 >> _2;
_4 = _1;
RET = _12.2;
_18 = _15;
_21 = !35056_u16;
_22.2 = core::ptr::addr_of!(_14);
_7 = _2;
Call(_2 = core::intrinsics::bswap(_10), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_5 = RET as isize;
_9 = !203_u8;
_12.1 = 1760037277_u32 - 163378854_u32;
_5 = _2;
_13 = [_1,_4,_4,_4,_1,_1,_4,_4];
_6 = !_11;
_7 = _11 >> _6;
_20.1 = 103_i8;
_13 = [_4,_4,_4,_1,_1,_4,_4,_1];
_10 = _6;
_7 = _3;
RET = _14;
_7 = -_11;
_12.1 = 11367_i16 as u32;
_5 = _2 & _2;
Goto(bb2)
}
bb12 = {
_15 = _9 as f64;
_9 = !104_u8;
match _20.1 {
0 => bb1,
1 => bb3,
103 => bb5,
_ => bb4
}
}
bb13 = {
_5 = RET as isize;
_9 = !203_u8;
_12.1 = 1760037277_u32 - 163378854_u32;
_5 = _2;
_13 = [_1,_4,_4,_4,_1,_1,_4,_4];
_6 = !_11;
_7 = _11 >> _6;
_20.1 = 103_i8;
_13 = [_4,_4,_4,_1,_1,_4,_4,_1];
_10 = _6;
_7 = _3;
RET = _14;
_7 = -_11;
_12.1 = 11367_i16 as u32;
_5 = _2 & _2;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_1 = _4 ^ _4;
_30.1 = -_22.1;
_22.1 = _30.1 << _12.1;
_9 = _17 as u8;
_30.2 = _22.2;
_23 = [(-24145_i16),2117_i16,15689_i16];
_33.1 = _20.1 as f32;
_16 = _20.0 ^ _20.0;
_26 = _33.1 * _33.1;
_24 = [_20.0,_20.0,_16,_20.0,_16,_20.0,_20.0];
RET = _14;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(16_usize, 2_usize, Move(_2), 14_usize, Move(_14), 3_usize, Move(_3), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(16_usize, 1_usize, Move(_1), 5_usize, Move(_5), 21_usize, Move(_21), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(16_usize, 24_usize, Move(_24), 8_usize, Move(_8), 35_usize, _35, 35_usize, _35), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [i128; 8],mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: [i128; 8],mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> u128 {
mir! {
type RET = u128;
let _15: [bool; 7];
let _16: Adt56;
let _17: i32;
let _18: Adt58;
let _19: f32;
let _20: (*const i16,);
let _21: [i16; 2];
let _22: char;
let _23: Adt55;
let _24: *const char;
let _25: [u64; 2];
let _26: i8;
let _27: i32;
let _28: isize;
let _29: char;
let _30: (*const i16,);
let _31: char;
let _32: (i8, f32, *mut u16);
let _33: [i16; 2];
let _34: *mut ((*mut u16, u32, char), *const char, *const char);
let _35: ();
let _36: ();
{
RET = 226080777083699075054462793862507310963_u128 - 275358861257454857586246156152111569822_u128;
_15 = [false,false,false,true,true,false,true];
_2 = _13 + _12;
_3 = _6 << _12;
_10 = [168902980559238583956918568064039555740_i128,(-122756523061031120695585815643663639818_i128),(-131509627512429715827661901400748889961_i128),(-140854034600991845670777839125448723462_i128),(-165946432995643783587993852400624865834_i128),83361677624091073846790090799515465064_i128,(-122319374290305098567034543497180056534_i128),13429302526727600522577634694442462084_i128];
_8 = _5;
_12 = !_5;
RET = 16822665808427296097_usize as u128;
_5 = 28721_u16 as isize;
_15 = [false,false,true,true,false,false,true];
_9 = RET as isize;
_12 = _1 - _7;
_4 = _10;
Goto(bb1)
}
bb1 = {
_11 = _2;
RET = 5_usize as u128;
_3 = -_12;
Goto(bb2)
}
bb2 = {
_10 = [84728097087484032083017610313803169820_i128,160583211348059202253888443476659935691_i128,47555489076449845391318735263379208136_i128,139999024471913982506675548843774354672_i128,49417739846872863216980208078537271435_i128,(-153969784054007304029177680292174446256_i128),129379742474287262801384696310369728459_i128,19309760245288063152950346082476384080_i128];
_5 = -_1;
_13 = _14;
_3 = _12;
_14 = 3640443350_u32 as isize;
_3 = _8 << _7;
_17 = (-721547134_i32) - 868731363_i32;
_10 = _4;
_7 = _3;
_6 = _11 & _1;
_8 = !_2;
_23.fld1.fld3 = 11724_u16;
_8 = _7;
_23.fld1.fld3 = 80_u8 as u16;
_23.fld1.fld2 = (116_u8,);
_19 = 5_usize as f32;
_14 = _6 + _5;
_13 = !_7;
_23.fld2.1 = !_17;
_8 = _2;
_23.fld1.fld3 = 43135_u16 ^ 60245_u16;
_21 = [(-8485_i16),(-13694_i16)];
Goto(bb3)
}
bb3 = {
_24 = core::ptr::addr_of!(_22);
_11 = _19 as isize;
match _23.fld1.fld2.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
116 => bb9,
_ => bb8
}
}
bb4 = {
_10 = [84728097087484032083017610313803169820_i128,160583211348059202253888443476659935691_i128,47555489076449845391318735263379208136_i128,139999024471913982506675548843774354672_i128,49417739846872863216980208078537271435_i128,(-153969784054007304029177680292174446256_i128),129379742474287262801384696310369728459_i128,19309760245288063152950346082476384080_i128];
_5 = -_1;
_13 = _14;
_3 = _12;
_14 = 3640443350_u32 as isize;
_3 = _8 << _7;
_17 = (-721547134_i32) - 868731363_i32;
_10 = _4;
_7 = _3;
_6 = _11 & _1;
_8 = !_2;
_23.fld1.fld3 = 11724_u16;
_8 = _7;
_23.fld1.fld3 = 80_u8 as u16;
_23.fld1.fld2 = (116_u8,);
_19 = 5_usize as f32;
_14 = _6 + _5;
_13 = !_7;
_23.fld2.1 = !_17;
_8 = _2;
_23.fld1.fld3 = 43135_u16 ^ 60245_u16;
_21 = [(-8485_i16),(-13694_i16)];
Goto(bb3)
}
bb5 = {
_11 = _2;
RET = 5_usize as u128;
_3 = -_12;
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
_23.fld1.fld4 = [_17];
_23.fld2.1 = _17 * _17;
_23.fld4 = (-1720022473956713142_i64) as u64;
_27 = _17;
match _23.fld1.fld2.0 {
0 => bb4,
1 => bb2,
116 => bb10,
_ => bb3
}
}
bb10 = {
_23.fld1.fld4 = [_23.fld2.1];
_23.fld1.fld3 = 7074_u16;
_24 = core::ptr::addr_of!(_22);
_2 = !_1;
_1 = (-983999693735822771_i64) as isize;
RET = _23.fld1.fld3 as u128;
_11 = -_14;
_9 = '\u{c50a8}' as isize;
_25 = [_23.fld4,_23.fld4];
_10 = [(-115622382217404064425814104692161117251_i128),67286477871149094036411013750889552058_i128,(-82137999074068961307515777408889916107_i128),(-4629627539362060008639882994155653505_i128),(-73367987237206239746945144193456399869_i128),(-73176208128863401610734560420718437105_i128),59005148601984739498370254163912647339_i128,93236713593678829687160713338608278526_i128];
_23.fld2.2 = core::ptr::addr_of!((*_24));
_23.fld2.2 = core::ptr::addr_of!((*_24));
_23.fld1.fld3 = !29737_u16;
_22 = '\u{377f9}';
_32.2 = core::ptr::addr_of_mut!(_23.fld1.fld3);
_23.fld1.fld3 = 4052_u16;
_23.fld1.fld4 = [_17];
Call(_23.fld1.fld2 = fn18(_13, _3, _2, _5, _7, _12, _14, _10, _2, _8, _14, _14, _8, _12, _2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_23.fld1.fld1 = Adt47::Variant3 { fld0: _11,fld1: _25 };
_6 = _14;
SetDiscriminant(_23.fld1.fld1, 3);
_11 = 8447_i16 as isize;
_31 = (*_24);
_17 = _23.fld2.1 ^ _23.fld2.1;
_28 = _8;
_12 = -_6;
_23.fld1.fld4 = [_17];
_23.fld1.fld3 = !15399_u16;
_23.fld1.fld3 = 54513_u16;
_23.fld1.fld4 = [_23.fld2.1];
_14 = _17 as isize;
match _23.fld1.fld3 {
0 => bb1,
1 => bb9,
2 => bb12,
3 => bb13,
54513 => bb15,
_ => bb14
}
}
bb12 = {
_10 = [84728097087484032083017610313803169820_i128,160583211348059202253888443476659935691_i128,47555489076449845391318735263379208136_i128,139999024471913982506675548843774354672_i128,49417739846872863216980208078537271435_i128,(-153969784054007304029177680292174446256_i128),129379742474287262801384696310369728459_i128,19309760245288063152950346082476384080_i128];
_5 = -_1;
_13 = _14;
_3 = _12;
_14 = 3640443350_u32 as isize;
_3 = _8 << _7;
_17 = (-721547134_i32) - 868731363_i32;
_10 = _4;
_7 = _3;
_6 = _11 & _1;
_8 = !_2;
_23.fld1.fld3 = 11724_u16;
_8 = _7;
_23.fld1.fld3 = 80_u8 as u16;
_23.fld1.fld2 = (116_u8,);
_19 = 5_usize as f32;
_14 = _6 + _5;
_13 = !_7;
_23.fld2.1 = !_17;
_8 = _2;
_23.fld1.fld3 = 43135_u16 ^ 60245_u16;
_21 = [(-8485_i16),(-13694_i16)];
Goto(bb3)
}
bb13 = {
_11 = _2;
RET = 5_usize as u128;
_3 = -_12;
Goto(bb2)
}
bb14 = {
_11 = _2;
RET = 5_usize as u128;
_3 = -_12;
Goto(bb2)
}
bb15 = {
_32.2 = core::ptr::addr_of_mut!(_23.fld1.fld3);
_23.fld1.fld4 = [_23.fld2.1];
_29 = _22;
_5 = _12;
_17 = !_27;
place!(Field::<isize>(Variant(_23.fld1.fld1, 3), 0)) = 7_usize as isize;
_31 = (*_24);
_22 = _29;
_10 = _4;
_3 = (-48_i8) as isize;
RET = 18585220120168078927562908236308536029_u128 * 73019721679186831025840226776410619154_u128;
Goto(bb16)
}
bb16 = {
Call(_35 = dump_var(17_usize, 21_usize, Move(_21), 28_usize, Move(_28), 15_usize, Move(_15), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(17_usize, 14_usize, Move(_14), 31_usize, Move(_31), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(17_usize, 9_usize, Move(_9), 4_usize, Move(_4), 25_usize, Move(_25), 36_usize, _36), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [i128; 8],mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize) -> (u8,) {
mir! {
type RET = (u8,);
let _16: Adt50;
let _17: isize;
let _18: i128;
let _19: f32;
let _20: ();
let _21: ();
{
_7 = -_12;
_2 = !_5;
RET.0 = (-10504_i16) as u8;
_14 = false as isize;
_13 = '\u{10a72d}' as isize;
_9 = 1938800868_i32 as isize;
RET.0 = 128_u8 - 21_u8;
_12 = _10 | _1;
_12 = -_10;
_11 = _15 ^ _5;
RET = (69_u8,);
_5 = _10;
_4 = (-46_i8) as isize;
RET.0 = 16621981790250913900_u64 as u8;
_8 = [(-31588808168377130257046413310389912679_i128),(-136644237569145367849986744313675780250_i128),(-125434271874683942742177325582347089664_i128),(-45464875894944084352662293240824834532_i128),(-101361840125153141307779740614992253531_i128),(-66533931678969639017203359200749061678_i128),118940374878951808733604483588792759848_i128,(-142137106872033964750376778252807358985_i128)];
RET.0 = !24_u8;
_2 = _12 & _7;
RET.0 = 52252259673546117730350759417597295488_i128 as u8;
RET = (149_u8,);
_6 = 4291969241_u32 as isize;
_4 = !_3;
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
149 => bb9,
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
_8 = [115528407568796107551265281481144746972_i128,83372404996079225319037492879300603228_i128,17380905965913222192341863400476682932_i128,90037280768449617806228422533258053571_i128,(-120372714261120974039308919863703992419_i128),57297912590882681603942864510960972053_i128,161177401388148049827073739999167581685_i128,7218647947728200265677332169199260531_i128];
RET.0 = !199_u8;
RET = (167_u8,);
_16.fld3 = !49786_u16;
_16.fld2 = (RET.0,);
_12 = _7 << _1;
_13 = _5;
_16.fld2 = RET;
_6 = _4;
RET.0 = _16.fld2.0 >> _2;
RET = (_16.fld2.0,);
_18 = 37489327194401869091710359040198713147_i128 >> _3;
_15 = _5;
_16.fld2.0 = !RET.0;
match RET.0 {
0 => bb5,
167 => bb10,
_ => bb2
}
}
bb10 = {
_19 = _18 as f32;
_4 = -_5;
_16.fld4 = [141578314_i32];
_13 = _1;
_16.fld4 = [(-499375519_i32)];
_12 = _6 - _10;
_7 = -_4;
_10 = _12 - _12;
_2 = '\u{5c9fa}' as isize;
_17 = _5;
_16.fld4 = [1218755889_i32];
_18 = -64341656770674508738404882971483679672_i128;
_17 = !_15;
_2 = -_3;
match RET.0 {
0 => bb6,
1 => bb7,
2 => bb5,
3 => bb11,
4 => bb12,
5 => bb13,
167 => bb15,
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
_16.fld2 = (RET.0,);
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(18_usize, 6_usize, Move(_6), 13_usize, Move(_13), 12_usize, Move(_12), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(18_usize, 3_usize, Move(_3), 10_usize, Move(_10), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: u8,mut _2: i8,mut _3: bool,mut _4: i8,mut _5: bool,mut _6: bool,mut _7: i8,mut _8: (u8,),mut _9: u8) -> *const char {
mir! {
type RET = *const char;
let _10: (usize, u8);
let _11: [i16; 3];
let _12: i128;
let _13: usize;
let _14: Adt63;
let _15: u128;
let _16: i32;
let _17: isize;
let _18: Adt52;
let _19: *const [i32; 1];
let _20: isize;
let _21: [i64; 4];
let _22: f64;
let _23: [i32; 5];
let _24: isize;
let _25: (u8,);
let _26: char;
let _27: ([i32; 1], (u8,));
let _28: ();
let _29: ();
{
_6 = !_3;
_8 = (_1,);
_1 = !_9;
_4 = _2;
_3 = !_6;
_8.0 = _9;
_4 = _1 as i8;
_6 = !_3;
_10 = (1425293982716868568_usize, _1);
Goto(bb1)
}
bb1 = {
_8 = (_1,);
_9 = _10.0 as u8;
_5 = _7 < _2;
_8 = (_9,);
_12 = (-162813257068857414437766688888203519694_i128);
_10.0 = 1320013544527688553_usize;
_5 = _4 == _2;
_5 = !_6;
_7 = _2 - _2;
_12 = !101432928950102937616522176239153947197_i128;
_14.fld5.0 = [1336648976_i32];
_10.1 = _9 * _1;
Goto(bb2)
}
bb2 = {
_8.0 = _9;
_14.fld3 = core::ptr::addr_of!(_14.fld0.2);
_14.fld5.1.0 = _7 as u8;
_6 = _3;
_17 = (-11_isize) ^ 9223372036854775807_isize;
RET = core::ptr::addr_of!(_14.fld0.2);
_14.fld5.0 = [26033773_i32];
_6 = _5;
RET = core::ptr::addr_of!((*RET));
_3 = _5;
_14.fld2 = [3610569148701568273_u64,11422259999024269391_u64];
_16 = (-1031779749_i32);
_14.fld1 = '\u{d609c}';
match _2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb8,
_ => bb7
}
}
bb3 = {
_8 = (_1,);
_9 = _10.0 as u8;
_5 = _7 < _2;
_8 = (_9,);
_12 = (-162813257068857414437766688888203519694_i128);
_10.0 = 1320013544527688553_usize;
_5 = _4 == _2;
_5 = !_6;
_7 = _2 - _2;
_12 = !101432928950102937616522176239153947197_i128;
_14.fld5.0 = [1336648976_i32];
_10.1 = _9 * _1;
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
_6 = _5;
_10.1 = !_9;
_14.fld3 = core::ptr::addr_of!(_14.fld1);
_14.fld2 = [7033270606579055776_u64,16524450801047912635_u64];
_13 = !_10.0;
_12 = -(-111264401111483127108030186869467357981_i128);
_15 = 193586979776900300813571534819465511514_u128;
_14.fld0.1 = 3645547441_u32;
_19 = core::ptr::addr_of!(_14.fld5.0);
_6 = !_5;
_5 = _8.0 < _14.fld5.1.0;
_14.fld5.1.0 = _10.1;
_9 = _14.fld5.1.0;
_11 = [(-31349_i16),4389_i16,14140_i16];
_10 = (_13, _8.0);
_1 = _16 as u8;
_4 = 17714_i16 as i8;
_8.0 = (-31595_i16) as u8;
_14.fld3 = core::ptr::addr_of!(_14.fld0.2);
_14.fld0.2 = _14.fld1;
Goto(bb9)
}
bb9 = {
match _2 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb10,
5 => bb12,
4 => bb14,
_ => bb13
}
}
bb10 = {
_6 = _5;
_10.1 = !_9;
_14.fld3 = core::ptr::addr_of!(_14.fld1);
_14.fld2 = [7033270606579055776_u64,16524450801047912635_u64];
_13 = !_10.0;
_12 = -(-111264401111483127108030186869467357981_i128);
_15 = 193586979776900300813571534819465511514_u128;
_14.fld0.1 = 3645547441_u32;
_19 = core::ptr::addr_of!(_14.fld5.0);
_6 = !_5;
_5 = _8.0 < _14.fld5.1.0;
_14.fld5.1.0 = _10.1;
_9 = _14.fld5.1.0;
_11 = [(-31349_i16),4389_i16,14140_i16];
_10 = (_13, _8.0);
_1 = _16 as u8;
_4 = 17714_i16 as i8;
_8.0 = (-31595_i16) as u8;
_14.fld3 = core::ptr::addr_of!(_14.fld0.2);
_14.fld0.2 = _14.fld1;
Goto(bb9)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_8.0 = _9;
_14.fld3 = core::ptr::addr_of!(_14.fld0.2);
_14.fld5.1.0 = _7 as u8;
_6 = _3;
_17 = (-11_isize) ^ 9223372036854775807_isize;
RET = core::ptr::addr_of!(_14.fld0.2);
_14.fld5.0 = [26033773_i32];
_6 = _5;
RET = core::ptr::addr_of!((*RET));
_3 = _5;
_14.fld2 = [3610569148701568273_u64,11422259999024269391_u64];
_16 = (-1031779749_i32);
_14.fld1 = '\u{d609c}';
match _2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb8,
_ => bb7
}
}
bb14 = {
_27 = _14.fld5;
_14.fld4 = Adt49::Variant3 { fld0: _2 };
_16 = 64243_u16 as i32;
_7 = _2;
_19 = core::ptr::addr_of!((*_19));
SetDiscriminant(_14.fld4, 2);
_14.fld1 = (*RET);
_14.fld0.2 = _14.fld1;
_16 = (-1379329980_i32);
place!(Field::<[i32; 1]>(Variant(_14.fld4, 2), 0)) = [_16];
_10.1 = _27.1.0 >> _2;
_23 = [_16,_16,_16,_16,_16];
_10 = (_13, _14.fld5.1.0);
_23 = [_16,_16,_16,_16,_16];
_20 = -_17;
_14.fld5 = (Field::<[i32; 1]>(Variant(_14.fld4, 2), 0), _27.1);
_14.fld5.1.0 = !_9;
_14.fld1 = _14.fld0.2;
_27.1.0 = _12 as u8;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(19_usize, 1_usize, Move(_1), 16_usize, Move(_16), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(19_usize, 13_usize, Move(_13), 23_usize, Move(_23), 9_usize, Move(_9), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(19_usize, 27_usize, Move(_27), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(10997826534507526947264543473983282173_u128), std::hint::black_box(508974756_i32), std::hint::black_box((-6_i8)), std::hint::black_box(1249486174_u32));
                
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt47 {
Variant0{
fld0: *const *mut u16,
fld1: [i16; 3],
fld2: (*const i16, i32, *const char),
fld3: i8,
fld4: u8,
fld5: u16,
fld6: i64,

},
Variant1{
fld0: ((*mut u16, u32, char), *const char, *const char),

},
Variant2{
fld0: (bool, i8, [i32; 5]),
fld1: i64,
fld2: *mut *mut u16,
fld3: i8,
fld4: i16,
fld5: i32,

},
Variant3{
fld0: isize,
fld1: [u64; 2],

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt48 {
Variant0{
fld0: ((*mut u16, u32, char), *const char, *const char),
fld1: [i128; 8],
fld2: usize,
fld3: u32,
fld4: *const *mut u16,
fld5: i32,
fld6: (*const i16,),

},
Variant1{
fld0: u8,
fld1: char,
fld2: ([u64; 2], *const [i32; 1]),
fld3: *mut ((*mut u16, u32, char), *const char, *const char),
fld4: (*const i16,),
fld5: *mut (i8, f32, *mut u16),

},
Variant2{
fld0: Adt47,
fld1: (usize, u8),
fld2: f64,
fld3: [i32; 5],
fld4: (((*mut u16, u32, char), *const char, *const char),),

}}
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: [u64; 2],
fld1: i16,
fld2: *mut (i8, f32, *mut u16),
fld3: u64,

},
Variant1{
fld0: [i64; 4],
fld1: Adt48,

},
Variant2{
fld0: [i32; 1],
fld1: u32,
fld2: [i16; 3],
fld3: i8,
fld4: *mut u16,

},
Variant3{
fld0: i8,

}}
#[derive(Debug)]
pub struct Adt50 {
fld0: Adt48,
fld1: Adt47,
fld2: (u8,),
fld3: u16,
fld4: [i32; 1],
}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: ((*mut u16, u32, char), *const char, *const char),

},
Variant1{
fld0: *const char,
fld1: u128,
fld2: [u64; 2],
fld3: ((*mut u16, u32, char), *const char, *const char),

},
Variant2{
fld0: *const *const *mut u16,
fld1: Adt50,
fld2: i128,
fld3: i8,
fld4: [i128; 8],
fld5: i32,

},
Variant3{
fld0: [i128; 8],
fld1: *const i16,
fld2: isize,
fld3: i8,
fld4: (bool, i8, [i32; 5]),
fld5: Adt48,
fld6: *const [i32; 1],

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: usize,
fld1: i64,
fld2: *mut (i8, f32, *mut u16),

},
Variant1{
fld0: Adt47,

},
Variant2{
fld0: f32,
fld1: *const i16,
fld2: Adt47,
fld3: Adt51,

},
Variant3{
fld0: u64,
fld1: (*mut u16, u32, char),
fld2: *const char,
fld3: [i128; 8],
fld4: ([u64; 2], *const [i32; 1]),

}}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: *const [i32; 1],

},
Variant1{
fld0: [i32; 5],
fld1: *mut *mut u16,
fld2: Adt49,
fld3: *mut (i8, f32, *mut u16),

}}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: (u8,),
fld1: f32,
fld2: isize,
fld3: usize,
fld4: ([u64; 2], *const [i32; 1]),
fld5: i32,
fld6: *const char,
fld7: *const *mut u16,

},
Variant1{
fld0: ([i32; 1], (u8,)),
fld1: u16,

}}
#[derive(Debug)]
pub struct Adt55 {
fld0: Adt51,
fld1: Adt50,
fld2: (*const i16, i32, *const char),
fld3: u8,
fld4: u64,
fld5: Adt52,
}
#[derive(Debug,Copy,Clone)]
pub enum Adt56 {
Variant0{
fld0: ([i32; 1], (u8,)),
fld1: [i128; 8],
fld2: ([u64; 2], *const [i32; 1]),
fld3: [i16; 3],
fld4: (((*mut u16, u32, char), *const char, *const char),),
fld5: *const *const *mut u16,
fld6: u128,
fld7: [i32; 5],

},
Variant1{
fld0: [i32; 5],
fld1: *const *const *mut u16,
fld2: (u8,),
fld3: i16,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt57 {
Variant0{
fld0: bool,
fld1: u8,
fld2: usize,
fld3: [i32; 1],
fld4: [i16; 3],
fld5: i32,
fld6: [i64; 4],

},
Variant1{
fld0: Adt48,
fld1: (*const i16,),
fld2: *mut ((*mut u16, u32, char), *const char, *const char),

}}
#[derive(Debug)]
pub enum Adt58 {
Variant0{
fld0: *const char,
fld1: (((*mut u16, u32, char), *const char, *const char),),

},
Variant1{
fld0: (u8,),
fld1: [i128; 8],
fld2: ((*mut u16, u32, char), *const char, *const char),

},
Variant2{
fld0: f32,
fld1: [i32; 5],
fld2: *mut ((*mut u16, u32, char), *const char, *const char),
fld3: Adt47,
fld4: [i128; 8],
fld5: [i16; 3],
fld6: u32,

},
Variant3{
fld0: Adt55,
fld1: u16,

}}
#[derive(Debug)]
pub enum Adt59 {
Variant0{
fld0: usize,
fld1: i32,

},
Variant1{
fld0: [i128; 8],
fld1: i128,
fld2: Adt52,

},
Variant2{
fld0: u8,
fld1: Adt58,
fld2: Adt50,

}}
#[derive(Debug)]
pub enum Adt60 {
Variant0{
fld0: Adt52,
fld1: [i16; 2],

},
Variant1{
fld0: [i64; 4],
fld1: Adt54,
fld2: isize,

},
Variant2{
fld0: (*const i16,),
fld1: (u8,),
fld2: Adt54,
fld3: Adt48,
fld4: f64,
fld5: [i16; 2],
fld6: ([u64; 2], *const [i32; 1]),
fld7: Adt58,

},
Variant3{
fld0: [i64; 4],
fld1: char,
fld2: usize,

}}
#[derive(Debug)]
pub enum Adt61 {
Variant0{
fld0: Adt60,
fld1: [bool; 7],
fld2: ([i32; 1], (u8,)),

},
Variant1{
fld0: *const i16,

}}
#[derive(Debug)]
pub struct Adt62 {
fld0: bool,
fld1: char,
fld2: (usize, u8),
fld3: (i8, f32, *mut u16),
}
#[derive(Debug)]
pub struct Adt63 {
fld0: (*mut u16, u32, char),
fld1: char,
fld2: [u64; 2],
fld3: *const char,
fld4: Adt49,
fld5: ([i32; 1], (u8,)),
}

