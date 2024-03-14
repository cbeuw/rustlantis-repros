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
pub fn fn0(mut _1: i64,mut _2: char,mut _3: isize) -> f64 {
mir! {
type RET = f64;
let _4: f64;
let _5: bool;
let _6: [u32; 4];
let _7: isize;
let _8: *mut u32;
let _9: isize;
let _10: char;
let _11: bool;
let _12: i16;
let _13: u64;
let _14: char;
let _15: i64;
let _16: usize;
let _17: Adt47;
let _18: Adt53;
let _19: [u32; 4];
let _20: usize;
let _21: i32;
let _22: [u16; 1];
let _23: (*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize);
let _24: (i64, u128);
let _25: [u16; 1];
let _26: [u32; 4];
let _27: Adt45;
let _28: (u16, u16, i16);
let _29: f32;
let _30: f32;
let _31: Adt43;
let _32: u64;
let _33: [u32; 4];
let _34: [u16; 6];
let _35: Adt51;
let _36: (u16, f64, i8, i64, u16);
let _37: [usize; 6];
let _38: Adt46;
let _39: isize;
let _40: Adt47;
let _41: [isize; 7];
let _42: *const i16;
let _43: Adt46;
let _44: usize;
let _45: ();
let _46: ();
{
Goto(bb1)
}
bb1 = {
RET = 117955485384462647164402217046824310283_i128 as f64;
_5 = true;
_3 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_2 = '\u{1c666}';
_6 = [3598081713_u32,3402406570_u32,800087933_u32,4202667827_u32];
_7 = -_3;
_2 = '\u{64b7a}';
_4 = -RET;
_5 = _7 < _7;
RET = -_4;
Call(_1 = fn1(_2, RET, _3, RET, RET, _2, _7, _6, _5, _6, _6, _7, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = 34504_u16 as isize;
_1 = 59099023_i32 as i64;
_3 = _2 as isize;
Goto(bb3)
}
bb3 = {
RET = 0_usize as f64;
_6 = [664081192_u32,2784941260_u32,636075584_u32,1276790550_u32];
_2 = '\u{d2355}';
_10 = _2;
_4 = -RET;
_7 = _9 >> _3;
_11 = _7 >= _7;
Call(_1 = core::intrinsics::bswap((-4777558938993073006_i64)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = RET;
_1 = 4162431377287116203_i64;
_15 = 323065772869528273009737420269524553236_u128 as i64;
_6 = [2470893309_u32,2780963538_u32,473055253_u32,4034088005_u32];
_15 = _1;
_4 = -RET;
_13 = 1893600585382656345_u64 & 8605069470526181820_u64;
_5 = _11 ^ _11;
_14 = _2;
_11 = !_5;
_13 = 104075756341650751_u64 << _1;
_12 = 9802_i16 >> _3;
_3 = _9;
_7 = _15 as isize;
Goto(bb5)
}
bb5 = {
_2 = _14;
_2 = _14;
_3 = _9;
_18.fld5 = 13_u8;
_13 = 3707961509830521640_u64;
_10 = _2;
_18.fld6.fld5 = [24247_u16,21637_u16,29435_u16,28095_u16,63115_u16,54224_u16];
_18.fld6.fld2 = 806_u16 * 17633_u16;
_16 = !5_usize;
_14 = _10;
_11 = !_5;
_18.fld6.fld1 = _13 | _13;
_18.fld6.fld0.1 = _18.fld6.fld2 != _18.fld6.fld2;
_18.fld6.fld0.2 = 325311483447742200366354229236734560350_u128 >> _18.fld6.fld1;
_3 = _7 ^ _7;
_18.fld3.1 = core::ptr::addr_of!(_24);
_14 = _10;
_19 = _6;
_18.fld6.fld7 = [_10];
Call(_18.fld3.3 = fn3(_7, _7, _5, _11, _18.fld6.fld2, RET, _18.fld6.fld5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_22 = [_18.fld6.fld2];
Goto(bb7)
}
bb7 = {
_18.fld4 = _12 - _12;
_20 = _16 | _16;
_18.fld5 = 0_u8 & 92_u8;
_18.fld6.fld5 = [_18.fld6.fld2,_18.fld6.fld2,_18.fld6.fld2,_18.fld6.fld2,_18.fld6.fld2,_18.fld6.fld2];
_24.0 = _1;
Call(_10 = fn16(_5, _13, _18.fld5, _1, _18.fld6.fld5, _18.fld6.fld0.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_26 = _19;
_22 = [_18.fld6.fld2];
_4 = RET * RET;
_28.2 = -_18.fld4;
_24.1 = !_18.fld6.fld0.2;
_10 = _14;
_28.1 = _18.fld6.fld2 >> _24.1;
_18.fld3.0 = _2;
_23.1.0.3 = _3 & _3;
_23.1.0.1 = _4 as f32;
_18.fld6.fld0.0 = _28.2 as usize;
_21 = !1946573986_i32;
_18.fld1 = Adt50::Variant3 { fld0: _22 };
_15 = -_1;
_15 = !_1;
_18.fld6.fld3 = _24.1 as i8;
_9 = _23.1.0.3 >> _23.1.0.3;
_28.2 = _12;
_4 = RET * RET;
_14 = _10;
_23.1.0.2 = RET * RET;
_15 = _9 as i64;
_28 = (_18.fld6.fld2, _18.fld6.fld2, _18.fld4);
_18.fld3.2 = _10;
_28 = (_18.fld6.fld2, _18.fld6.fld2, _18.fld4);
_18.fld6.fld4 = _18.fld4 ^ _18.fld4;
_6 = _26;
Goto(bb9)
}
bb9 = {
_25 = [_18.fld6.fld2];
Goto(bb10)
}
bb10 = {
_16 = _9 as usize;
_9 = -_23.1.0.3;
_29 = _23.1.0.1 * _23.1.0.1;
_28.1 = _18.fld6.fld2;
_18.fld3.0 = _10;
_32 = _18.fld6.fld0.1 as u64;
_5 = _11;
_23.1.0.0 = !_5;
_14 = _2;
_23.2 = _16 | _18.fld6.fld0.0;
_6 = _26;
RET = -_23.1.0.2;
_12 = -_18.fld6.fld4;
Goto(bb11)
}
bb11 = {
_23.0 = core::ptr::addr_of!(_12);
_18.fld3.3 = core::ptr::addr_of_mut!(_29);
_23.1.0.0 = !_5;
_20 = _18.fld6.fld3 as usize;
_18.fld3.0 = _2;
_23.1.2 = !_28.1;
_17 = Adt47::Variant0 { fld0: _9,fld1: _18.fld6.fld7 };
_18.fld6.fld3 = 102_i8;
_28 = (_18.fld6.fld2, _18.fld6.fld2, _18.fld4);
Call(_14 = fn17(_18.fld6.fld0, _16, _23.2, _16, _18.fld6.fld0.1, _6, _19, _18.fld6.fld2, _12, _23.2, _15, _23.2, _18.fld3.3, _18.fld6.fld5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_33 = [2066933895_u32,4263173661_u32,3951030042_u32,3541991643_u32];
_4 = RET;
_39 = _18.fld5 as isize;
_6 = _26;
_34 = [_18.fld6.fld2,_28.0,_28.0,_28.0,_28.1,_23.1.2];
_23.2 = _18.fld6.fld0.0;
_42 = core::ptr::addr_of!(_18.fld4);
_18.fld6.fld0.1 = _23.1.0.0 | _23.1.0.0;
_6 = [964117751_u32,391124140_u32,1374878804_u32,1466567573_u32];
_18.fld2 = Field::<isize>(Variant(_17, 0), 0);
_34 = [_28.0,_28.1,_23.1.2,_28.1,_18.fld6.fld2,_28.1];
_23.0 = _42;
_15 = _24.0 | _24.0;
_18.fld6.fld0.1 = _5 & _5;
_3 = _39 - _18.fld2;
_42 = core::ptr::addr_of!(_18.fld6.fld4);
_23.1.1 = _18.fld3.3;
_23.1.0.1 = _29 - _29;
_18.fld3.1 = core::ptr::addr_of!(_24);
_15 = -_24.0;
_34 = [_23.1.2,_23.1.2,_28.0,_23.1.2,_28.0,_18.fld6.fld2];
match _18.fld6.fld3 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb5,
5 => bb9,
6 => bb7,
102 => bb14,
_ => bb13
}
}
bb13 = {
_26 = _19;
_22 = [_18.fld6.fld2];
_4 = RET * RET;
_28.2 = -_18.fld4;
_24.1 = !_18.fld6.fld0.2;
_10 = _14;
_28.1 = _18.fld6.fld2 >> _24.1;
_18.fld3.0 = _2;
_23.1.0.3 = _3 & _3;
_23.1.0.1 = _4 as f32;
_18.fld6.fld0.0 = _28.2 as usize;
_21 = !1946573986_i32;
_18.fld1 = Adt50::Variant3 { fld0: _22 };
_15 = -_1;
_15 = !_1;
_18.fld6.fld3 = _24.1 as i8;
_9 = _23.1.0.3 >> _23.1.0.3;
_28.2 = _12;
_4 = RET * RET;
_14 = _10;
_23.1.0.2 = RET * RET;
_15 = _9 as i64;
_28 = (_18.fld6.fld2, _18.fld6.fld2, _18.fld4);
_18.fld3.2 = _10;
_28 = (_18.fld6.fld2, _18.fld6.fld2, _18.fld4);
_18.fld6.fld4 = _18.fld4 ^ _18.fld4;
_6 = _26;
Goto(bb9)
}
bb14 = {
_42 = core::ptr::addr_of!(_28.2);
_9 = !_3;
_23.1.1 = core::ptr::addr_of_mut!(_30);
_4 = _18.fld6.fld3 as f64;
_40 = Adt47::Variant0 { fld0: Field::<isize>(Variant(_17, 0), 0),fld1: _18.fld6.fld7 };
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(0_usize, 25_usize, Move(_25), 7_usize, Move(_7), 9_usize, Move(_9), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(0_usize, 1_usize, Move(_1), 6_usize, Move(_6), 22_usize, Move(_22), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(0_usize, 32_usize, Move(_32), 26_usize, Move(_26), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(0_usize, 2_usize, Move(_2), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: char,mut _2: f64,mut _3: isize,mut _4: f64,mut _5: f64,mut _6: char,mut _7: isize,mut _8: [u32; 4],mut _9: bool,mut _10: [u32; 4],mut _11: [u32; 4],mut _12: isize,mut _13: bool) -> i64 {
mir! {
type RET = i64;
let _14: char;
let _15: ([isize; 7], *mut char, &'static i32);
let _16: [i64; 6];
let _17: i16;
let _18: f32;
let _19: f32;
let _20: [isize; 6];
let _21: (bool, f32, f64, isize);
let _22: usize;
let _23: (bool, f32, f64, isize);
let _24: Adt59;
let _25: [isize; 6];
let _26: Adt52;
let _27: [u16; 6];
let _28: i16;
let _29: char;
let _30: *const i32;
let _31: Adt53;
let _32: ();
let _33: ();
{
_6 = _1;
_1 = _6;
_11 = [1566839085_u32,2516122753_u32,1290180271_u32,712812206_u32];
RET = -3635665543187076812_i64;
_11 = _8;
_14 = _1;
_12 = _3 + _7;
_13 = _9;
_12 = (-493256630_i32) as isize;
_7 = _3;
_13 = _9;
_6 = _14;
_2 = -_5;
_3 = _12 + _7;
Goto(bb1)
}
bb1 = {
_15.0 = [_12,_3,_3,_7,_12,_7,_12];
_14 = _6;
_4 = _2 * _5;
_8 = [4213859518_u32,4096629253_u32,132653732_u32,249466443_u32];
_3 = -_7;
_7 = _9 as isize;
_8 = [3974271755_u32,1348951452_u32,3239826257_u32,2139543880_u32];
_15.1 = core::ptr::addr_of_mut!(_6);
_12 = _3 + _7;
_8 = _10;
_10 = _8;
RET = 4270918275995440384_i64;
_14 = _1;
_14 = _1;
_10 = [2572132232_u32,3199530888_u32,3695192032_u32,4289399551_u32];
_18 = (-16585_i16) as f32;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
4270918275995440384 => bb9,
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
_20 = [_12,_3,_12,_12,_7,_3];
_10 = [4215895810_u32,2469215313_u32,932098213_u32,270728769_u32];
_1 = _6;
_16 = [RET,RET,RET,RET,RET,RET];
_3 = -_12;
_20 = [_7,_3,_3,_3,_3,_3];
_10 = [1189846732_u32,2325799138_u32,1034396526_u32,2910031969_u32];
_1 = _6;
_2 = _5 + _5;
_3 = _2 as isize;
_18 = 131_u8 as f32;
_3 = _12;
_8 = [1732129107_u32,747913999_u32,2472223910_u32,411092074_u32];
Call(RET = fn2(_11, _20, _20, _15.0, _4, _11, _7, _3, _12, _7, _13, _20, _12, _7, _12), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10 = [2148934341_u32,644694881_u32,628451146_u32,21431804_u32];
_20 = [_7,_3,_3,_3,_3,_12];
_15.1 = core::ptr::addr_of_mut!(_6);
_8 = [2611111990_u32,1840038858_u32,2144206138_u32,123891112_u32];
_6 = _1;
_16 = [RET,RET,RET,RET,RET,RET];
_8 = [1951628590_u32,3083150775_u32,3970153344_u32,2596621704_u32];
_19 = _2 as f32;
_20 = [_7,_3,_7,_7,_7,_3];
_21.2 = _4;
RET = 54161_u16 as i64;
Goto(bb11)
}
bb11 = {
_21.0 = !_13;
_22 = !1_usize;
_3 = -_12;
_14 = _6;
_3 = 4709789_i32 as isize;
_16 = [RET,RET,RET,RET,RET,RET];
_21.2 = _4 + _2;
_23.0 = _21.0;
RET = 8974273176422292600_i64;
_23.2 = _21.2 * _21.2;
_21.0 = _13 & _23.0;
_21.1 = _19;
Goto(bb12)
}
bb12 = {
_22 = !3_usize;
_10 = _11;
_23.0 = !_21.0;
_8 = [1851375459_u32,3413132244_u32,392876239_u32,873611877_u32];
_5 = (-1708520351_i32) as f64;
_9 = !_21.0;
_22 = !6531330150141227461_usize;
_17 = 2622_i16 >> _7;
_5 = _4 + _23.2;
_5 = -_21.2;
_20 = [_12,_3,_12,_12,_7,_12];
_23 = (_13, _18, _5, _12);
_3 = _23.3;
_20 = [_23.3,_12,_3,_23.3,_3,_12];
_2 = -_5;
_21.0 = _23.3 != _12;
Call(_4 = core::intrinsics::fmaf64(_5, _21.2, _23.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_17 = (-1356_i16);
_15.0 = [_12,_12,_12,_23.3,_12,_23.3,_23.3];
_1 = _14;
_21.3 = _12 << _23.3;
_21.0 = _23.0 ^ _23.0;
_15.0 = [_23.3,_21.3,_12,_12,_7,_23.3,_12];
_15.0 = [_21.3,_12,_23.3,_21.3,_21.3,_23.3,_12];
_4 = 18225181040132176229_u64 as f64;
RET = 5355509448377858683_i64 >> _21.3;
_10 = _11;
_9 = _23.0;
_31.fld6.fld0.1 = _12 == _12;
_31.fld3.3 = core::ptr::addr_of_mut!(_18);
_29 = _6;
_31.fld0 = _31.fld3.3;
_31.fld6.fld0 = (_22, _21.0, 280911614767869500643061297138500043125_u128);
_31.fld6.fld2 = 65014_u16;
_23.3 = _7;
_25 = [_21.3,_21.3,_12,_3,_7,_21.3];
_28 = !_17;
_31.fld6.fld3 = -104_i8;
Goto(bb14)
}
bb14 = {
Call(_32 = dump_var(1_usize, 7_usize, Move(_7), 9_usize, Move(_9), 22_usize, Move(_22), 16_usize, Move(_16)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_32 = dump_var(1_usize, 17_usize, Move(_17), 29_usize, Move(_29), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(1_usize, 14_usize, Move(_14), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [u32; 4],mut _2: [isize; 6],mut _3: [isize; 6],mut _4: [isize; 7],mut _5: f64,mut _6: [u32; 4],mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: bool,mut _12: [isize; 6],mut _13: isize,mut _14: isize,mut _15: isize) -> i64 {
mir! {
type RET = i64;
let _16: *const usize;
let _17: f32;
let _18: f32;
let _19: Adt48;
let _20: char;
let _21: [isize; 6];
let _22: [u16; 1];
let _23: u32;
let _24: [u16; 6];
let _25: char;
let _26: usize;
let _27: usize;
let _28: ([isize; 7], *mut char, &'static i32);
let _29: (u16, u16, i16);
let _30: ([isize; 7], *mut char, &'static i32);
let _31: (usize, bool, u128);
let _32: (i64, u128);
let _33: [u32; 4];
let _34: [bool; 7];
let _35: u32;
let _36: Adt56;
let _37: i32;
let _38: &'static i32;
let _39: (bool, f32, f64, isize);
let _40: [u32; 4];
let _41: i8;
let _42: [isize; 7];
let _43: isize;
let _44: (u16, u16, i16);
let _45: ();
let _46: ();
{
_7 = _14;
RET = 159916108694138582_i64 << _10;
_13 = -_9;
RET = !1611051212137668541_i64;
_5 = 3698048990_u32 as f64;
_9 = 3847377455_u32 as isize;
_5 = 1747749658_u32 as f64;
_13 = !_14;
_13 = -_7;
_9 = !_14;
_1 = [783493058_u32,2037332051_u32,4294855033_u32,1540835706_u32];
RET = 1741595882_i32 as i64;
_10 = -_14;
_11 = !true;
_11 = !false;
_7 = _5 as isize;
RET = (-69814083225660181910148646572996013511_i128) as i64;
_18 = 8000968023864261866_usize as f32;
RET = !925252825915405220_i64;
_8 = !_15;
_17 = _18;
Goto(bb1)
}
bb1 = {
_5 = 172683538054285115619760294392450533341_u128 as f64;
RET = 17838587945524916970_u64 as i64;
_14 = !_8;
_9 = RET as isize;
_23 = !2817731768_u32;
_21 = [_15,_15,_7,_15,_14,_7];
_22 = [45952_u16];
_8 = -_10;
_8 = _13;
_23 = 729820596_u32 << RET;
_18 = _17;
_1 = _6;
_13 = _9;
_15 = _8 & _8;
_2 = [_14,_15,_15,_15,_14,_9];
_13 = 14548_u16 as isize;
_5 = 96_u8 as f64;
_16 = core::ptr::addr_of!(_26);
RET = (-6708310055671551253_i64);
_24 = [1590_u16,56436_u16,26725_u16,48322_u16,60418_u16,36581_u16];
_4 = [_10,_14,_14,_15,_10,_14,_14];
_27 = 2287534752020707516_usize + 6_usize;
_20 = '\u{b7153}';
_3 = [_15,_15,_15,_8,_7,_15];
Goto(bb2)
}
bb2 = {
_8 = _15 + _15;
RET = _27 as i64;
_24 = [62844_u16,61361_u16,56901_u16,50371_u16,6438_u16,16915_u16];
_10 = _15;
_11 = !false;
_25 = _20;
_5 = 16511233287815033205_u64 as f64;
_3 = _2;
_9 = 17787340175578860173_u64 as isize;
_25 = _20;
_7 = _5 as isize;
_6 = _1;
_23 = 603001949_u32;
_28.0 = [_15,_15,_10,_15,_10,_10,_15];
_16 = core::ptr::addr_of!((*_16));
_2 = [_8,_8,_9,_8,_14,_8];
_7 = _8;
RET = !2200383798883543128_i64;
_16 = core::ptr::addr_of!((*_16));
RET = !2577430908878759813_i64;
_30.1 = core::ptr::addr_of_mut!(_25);
_28.1 = _30.1;
match _23 {
0 => bb1,
603001949 => bb4,
_ => bb3
}
}
bb3 = {
_5 = 172683538054285115619760294392450533341_u128 as f64;
RET = 17838587945524916970_u64 as i64;
_14 = !_8;
_9 = RET as isize;
_23 = !2817731768_u32;
_21 = [_15,_15,_7,_15,_14,_7];
_22 = [45952_u16];
_8 = -_10;
_8 = _13;
_23 = 729820596_u32 << RET;
_18 = _17;
_1 = _6;
_13 = _9;
_15 = _8 & _8;
_2 = [_14,_15,_15,_15,_14,_9];
_13 = 14548_u16 as isize;
_5 = 96_u8 as f64;
_16 = core::ptr::addr_of!(_26);
RET = (-6708310055671551253_i64);
_24 = [1590_u16,56436_u16,26725_u16,48322_u16,60418_u16,36581_u16];
_4 = [_10,_14,_14,_15,_10,_14,_14];
_27 = 2287534752020707516_usize + 6_usize;
_20 = '\u{b7153}';
_3 = [_15,_15,_15,_8,_7,_15];
Goto(bb2)
}
bb4 = {
_8 = _14 - _14;
_27 = !15681913775384809389_usize;
_28.1 = core::ptr::addr_of_mut!(_25);
_21 = _3;
_30.0 = _28.0;
_22 = [36852_u16];
_32 = (RET, 338641613088115816908262167052347979676_u128);
_29.2 = 17099_i16;
_12 = _2;
_32.1 = 272095823245680909958906185136122827864_u128 | 327344891254333800037659065505932095078_u128;
_9 = _18 as isize;
_29.0 = _20 as u16;
_5 = 8281991557099753772_u64 as f64;
_31.2 = !_32.1;
_32.0 = RET;
_36.fld1 = [_8,_7,_14,_14,_14,_15,_10];
_31.1 = !_11;
_39.1 = _32.0 as f32;
_20 = _25;
Goto(bb5)
}
bb5 = {
_36.fld0 = [_31.1,_31.1,_31.1,_11,_11,_31.1,_11];
_14 = _10 * _8;
_12 = [_10,_10,_14,_14,_8,_8];
_31.2 = !_32.1;
_31.0 = _27 | _27;
_28.2 = &_37;
match _29.2 {
0 => bb6,
1 => bb7,
2 => bb8,
17099 => bb10,
_ => bb9
}
}
bb6 = {
_8 = _14 - _14;
_27 = !15681913775384809389_usize;
_28.1 = core::ptr::addr_of_mut!(_25);
_21 = _3;
_30.0 = _28.0;
_22 = [36852_u16];
_32 = (RET, 338641613088115816908262167052347979676_u128);
_29.2 = 17099_i16;
_12 = _2;
_32.1 = 272095823245680909958906185136122827864_u128 | 327344891254333800037659065505932095078_u128;
_9 = _18 as isize;
_29.0 = _20 as u16;
_5 = 8281991557099753772_u64 as f64;
_31.2 = !_32.1;
_32.0 = RET;
_36.fld1 = [_8,_7,_14,_14,_14,_15,_10];
_31.1 = !_11;
_39.1 = _32.0 as f32;
_20 = _25;
Goto(bb5)
}
bb7 = {
_5 = 172683538054285115619760294392450533341_u128 as f64;
RET = 17838587945524916970_u64 as i64;
_14 = !_8;
_9 = RET as isize;
_23 = !2817731768_u32;
_21 = [_15,_15,_7,_15,_14,_7];
_22 = [45952_u16];
_8 = -_10;
_8 = _13;
_23 = 729820596_u32 << RET;
_18 = _17;
_1 = _6;
_13 = _9;
_15 = _8 & _8;
_2 = [_14,_15,_15,_15,_14,_9];
_13 = 14548_u16 as isize;
_5 = 96_u8 as f64;
_16 = core::ptr::addr_of!(_26);
RET = (-6708310055671551253_i64);
_24 = [1590_u16,56436_u16,26725_u16,48322_u16,60418_u16,36581_u16];
_4 = [_10,_14,_14,_15,_10,_14,_14];
_27 = 2287534752020707516_usize + 6_usize;
_20 = '\u{b7153}';
_3 = [_15,_15,_15,_8,_7,_15];
Goto(bb2)
}
bb8 = {
_8 = _15 + _15;
RET = _27 as i64;
_24 = [62844_u16,61361_u16,56901_u16,50371_u16,6438_u16,16915_u16];
_10 = _15;
_11 = !false;
_25 = _20;
_5 = 16511233287815033205_u64 as f64;
_3 = _2;
_9 = 17787340175578860173_u64 as isize;
_25 = _20;
_7 = _5 as isize;
_6 = _1;
_23 = 603001949_u32;
_28.0 = [_15,_15,_10,_15,_10,_10,_15];
_16 = core::ptr::addr_of!((*_16));
_2 = [_8,_8,_9,_8,_14,_8];
_7 = _8;
RET = !2200383798883543128_i64;
_16 = core::ptr::addr_of!((*_16));
RET = !2577430908878759813_i64;
_30.1 = core::ptr::addr_of_mut!(_25);
_28.1 = _30.1;
match _23 {
0 => bb1,
603001949 => bb4,
_ => bb3
}
}
bb9 = {
_5 = 172683538054285115619760294392450533341_u128 as f64;
RET = 17838587945524916970_u64 as i64;
_14 = !_8;
_9 = RET as isize;
_23 = !2817731768_u32;
_21 = [_15,_15,_7,_15,_14,_7];
_22 = [45952_u16];
_8 = -_10;
_8 = _13;
_23 = 729820596_u32 << RET;
_18 = _17;
_1 = _6;
_13 = _9;
_15 = _8 & _8;
_2 = [_14,_15,_15,_15,_14,_9];
_13 = 14548_u16 as isize;
_5 = 96_u8 as f64;
_16 = core::ptr::addr_of!(_26);
RET = (-6708310055671551253_i64);
_24 = [1590_u16,56436_u16,26725_u16,48322_u16,60418_u16,36581_u16];
_4 = [_10,_14,_14,_15,_10,_14,_14];
_27 = 2287534752020707516_usize + 6_usize;
_20 = '\u{b7153}';
_3 = [_15,_15,_15,_8,_7,_15];
Goto(bb2)
}
bb10 = {
_26 = _27;
_13 = _14;
_38 = &_37;
_32.0 = RET | RET;
_39.3 = _7;
_1 = [_23,_23,_23,_23];
_17 = -_18;
_6 = _1;
_44.1 = _29.0 * _29.0;
_29 = (_44.1, _44.1, (-12349_i16));
_35 = _23;
_15 = -_13;
_44 = (_29.0, _29.1, _29.2);
_10 = _14;
_15 = _7;
_30.1 = _28.1;
_17 = _18 + _18;
_30.2 = &(*_38);
match _44.2 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
340282366920938463463374607431768199107 => bb16,
_ => bb15
}
}
bb11 = {
_5 = 172683538054285115619760294392450533341_u128 as f64;
RET = 17838587945524916970_u64 as i64;
_14 = !_8;
_9 = RET as isize;
_23 = !2817731768_u32;
_21 = [_15,_15,_7,_15,_14,_7];
_22 = [45952_u16];
_8 = -_10;
_8 = _13;
_23 = 729820596_u32 << RET;
_18 = _17;
_1 = _6;
_13 = _9;
_15 = _8 & _8;
_2 = [_14,_15,_15,_15,_14,_9];
_13 = 14548_u16 as isize;
_5 = 96_u8 as f64;
_16 = core::ptr::addr_of!(_26);
RET = (-6708310055671551253_i64);
_24 = [1590_u16,56436_u16,26725_u16,48322_u16,60418_u16,36581_u16];
_4 = [_10,_14,_14,_15,_10,_14,_14];
_27 = 2287534752020707516_usize + 6_usize;
_20 = '\u{b7153}';
_3 = [_15,_15,_15,_8,_7,_15];
Goto(bb2)
}
bb12 = {
_8 = _15 + _15;
RET = _27 as i64;
_24 = [62844_u16,61361_u16,56901_u16,50371_u16,6438_u16,16915_u16];
_10 = _15;
_11 = !false;
_25 = _20;
_5 = 16511233287815033205_u64 as f64;
_3 = _2;
_9 = 17787340175578860173_u64 as isize;
_25 = _20;
_7 = _5 as isize;
_6 = _1;
_23 = 603001949_u32;
_28.0 = [_15,_15,_10,_15,_10,_10,_15];
_16 = core::ptr::addr_of!((*_16));
_2 = [_8,_8,_9,_8,_14,_8];
_7 = _8;
RET = !2200383798883543128_i64;
_16 = core::ptr::addr_of!((*_16));
RET = !2577430908878759813_i64;
_30.1 = core::ptr::addr_of_mut!(_25);
_28.1 = _30.1;
match _23 {
0 => bb1,
603001949 => bb4,
_ => bb3
}
}
bb13 = {
_8 = _14 - _14;
_27 = !15681913775384809389_usize;
_28.1 = core::ptr::addr_of_mut!(_25);
_21 = _3;
_30.0 = _28.0;
_22 = [36852_u16];
_32 = (RET, 338641613088115816908262167052347979676_u128);
_29.2 = 17099_i16;
_12 = _2;
_32.1 = 272095823245680909958906185136122827864_u128 | 327344891254333800037659065505932095078_u128;
_9 = _18 as isize;
_29.0 = _20 as u16;
_5 = 8281991557099753772_u64 as f64;
_31.2 = !_32.1;
_32.0 = RET;
_36.fld1 = [_8,_7,_14,_14,_14,_15,_10];
_31.1 = !_11;
_39.1 = _32.0 as f32;
_20 = _25;
Goto(bb5)
}
bb14 = {
_8 = _14 - _14;
_27 = !15681913775384809389_usize;
_28.1 = core::ptr::addr_of_mut!(_25);
_21 = _3;
_30.0 = _28.0;
_22 = [36852_u16];
_32 = (RET, 338641613088115816908262167052347979676_u128);
_29.2 = 17099_i16;
_12 = _2;
_32.1 = 272095823245680909958906185136122827864_u128 | 327344891254333800037659065505932095078_u128;
_9 = _18 as isize;
_29.0 = _20 as u16;
_5 = 8281991557099753772_u64 as f64;
_31.2 = !_32.1;
_32.0 = RET;
_36.fld1 = [_8,_7,_14,_14,_14,_15,_10];
_31.1 = !_11;
_39.1 = _32.0 as f32;
_20 = _25;
Goto(bb5)
}
bb15 = {
_36.fld0 = [_31.1,_31.1,_31.1,_11,_11,_31.1,_11];
_14 = _10 * _8;
_12 = [_10,_10,_14,_14,_8,_8];
_31.2 = !_32.1;
_31.0 = _27 | _27;
_28.2 = &_37;
match _29.2 {
0 => bb6,
1 => bb7,
2 => bb8,
17099 => bb10,
_ => bb9
}
}
bb16 = {
_12 = [_7,_10,_13,_14,_15,_15];
_26 = _31.1 as usize;
_29 = _44;
_3 = [_13,_10,_7,_7,_15,_13];
_4 = [_14,_14,_13,_13,_7,_7,_13];
_28.0 = _4;
_8 = _17 as isize;
_32 = (RET, _31.2);
_36.fld1 = _4;
_38 = &_37;
_21 = _12;
Goto(bb17)
}
bb17 = {
Call(_45 = dump_var(2_usize, 8_usize, Move(_8), 13_usize, Move(_13), 32_usize, Move(_32), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(2_usize, 27_usize, Move(_27), 12_usize, Move(_12), 44_usize, Move(_44), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(2_usize, 9_usize, Move(_9), 26_usize, Move(_26), 21_usize, Move(_21), 20_usize, Move(_20)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(2_usize, 7_usize, Move(_7), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: u16,mut _6: f64,mut _7: [u16; 6]) -> *mut f32 {
mir! {
type RET = *mut f32;
let _8: Adt53;
let _9: isize;
let _10: isize;
let _11: [u16; 6];
let _12: *const i16;
let _13: u128;
let _14: i128;
let _15: i32;
let _16: Adt44;
let _17: Adt55;
let _18: [char; 1];
let _19: f64;
let _20: i32;
let _21: f32;
let _22: Adt54;
let _23: f64;
let _24: char;
let _25: (*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize);
let _26: i8;
let _27: ([isize; 7], *mut char, &'static i32);
let _28: (bool, f32, f64, isize);
let _29: u128;
let _30: u32;
let _31: [bool; 7];
let _32: [i64; 6];
let _33: f32;
let _34: isize;
let _35: bool;
let _36: char;
let _37: [i64; 6];
let _38: isize;
let _39: ();
let _40: ();
{
_8.fld4 = 1553597149_i32 as i16;
_6 = _2 as f64;
_8.fld6.fld0.0 = 3_usize;
_8.fld6.fld7 = ['\u{65248}'];
_8.fld2 = _1 & _2;
_8.fld6.fld0.1 = _3 <= _4;
_8.fld6.fld7 = ['\u{3254d}'];
_8.fld5 = _8.fld6.fld0.0 as u8;
_2 = _8.fld2 & _1;
_6 = 4865465342654918805_i64 as f64;
_8.fld6.fld3 = -99_i8;
_8.fld6.fld0 = (8952945324014029158_usize, _3, 287481012237541024171648009913253153220_u128);
_8.fld6.fld0 = (7859913262032603488_usize, _4, 54570664769204728073252005940495383349_u128);
_1 = _5 as isize;
_8.fld3.0 = '\u{703b9}';
_8.fld6.fld4 = -_8.fld4;
_2 = 387925627_i32 as isize;
_2 = 2901624584_u32 as isize;
_3 = !_8.fld6.fld0.1;
_4 = !_3;
match _8.fld6.fld0.2 {
54570664769204728073252005940495383349 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_8.fld2 = 168127300_i32 as isize;
_8.fld5 = !187_u8;
_8.fld3.2 = _8.fld3.0;
_8.fld6.fld0.2 = 282301410066325054853005375165721356537_u128 - 30928016890574678088201960227363285216_u128;
_2 = _8.fld2;
_12 = core::ptr::addr_of!(_8.fld4);
_11 = [_5,_5,_5,_5,_5,_5];
_6 = _1 as f64;
_6 = 2680641177098456468_u64 as f64;
_8.fld6.fld1 = _8.fld5 as u64;
_8.fld6.fld3 = (-56_i8) | 113_i8;
_8.fld3.2 = _8.fld3.0;
_8.fld6.fld2 = _3 as u16;
_8.fld6.fld0.2 = !59928152392478452487176617315533102016_u128;
_8.fld6.fld3 = !(-79_i8);
_8.fld6.fld1 = 11849759175143492892_u64 & 7078181998407690185_u64;
_8.fld6.fld0.0 = 5_usize * 4_usize;
_8.fld6.fld0 = (2_usize, _4, 52061012391238668951160966678787637270_u128);
_11 = [_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_5];
Goto(bb3)
}
bb3 = {
_8.fld6.fld2 = _5 ^ _5;
_8.fld6.fld1 = !12184657088220405379_u64;
_9 = _2 >> _8.fld6.fld0.0;
_8.fld6.fld0.1 = !_4;
_2 = _1;
_8.fld6.fld5 = [_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_5,_5,_5];
_8.fld4 = -_8.fld6.fld4;
_8.fld2 = _5 as isize;
_4 = _3;
_8.fld6.fld0.0 = _8.fld6.fld1 as usize;
_8.fld6.fld3 = !15_i8;
_8.fld6.fld0 = (6_usize, _4, 7083475450524407572102734168011225654_u128);
_8.fld6.fld3 = (-96_i8);
_10 = _8.fld6.fld2 as isize;
_8.fld3.2 = _8.fld3.0;
_8.fld6.fld0 = (2_usize, _3, 292908597001534285299189459419764708271_u128);
_11 = [_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2];
_8.fld6.fld1 = 1441326922554537786_u64 - 14536716520193403697_u64;
_8.fld3.0 = _8.fld3.2;
_8.fld6.fld5 = _7;
_8.fld6.fld0.2 = _8.fld3.0 as u128;
_9 = 139575449388831601677395556530069182244_i128 as isize;
_8.fld6.fld2 = _5;
Call(_8.fld6 = fn4(_3, _9, _4, _4, _11, _11, _1, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_8.fld4 = !_8.fld6.fld4;
_4 = !_3;
_1 = _8.fld2 | _9;
_15 = 3127467072_u32 as i32;
_13 = _8.fld6.fld0.2;
_10 = _1;
_6 = 573613783_u32 as f64;
_8.fld5 = 244_u8;
_1 = _6 as isize;
_19 = -_6;
_1 = _8.fld5 as isize;
_8.fld6.fld5 = [_5,_8.fld6.fld2,_5,_5,_8.fld6.fld2,_8.fld6.fld2];
_8.fld3.3 = core::ptr::addr_of_mut!(_21);
_8.fld2 = _8.fld6.fld0.0 as isize;
match _8.fld6.fld0.0 {
5557644557378790946 => bb5,
_ => bb3
}
}
bb5 = {
_15 = (-1994139748_i32) << _8.fld6.fld2;
_8.fld6.fld5 = [_8.fld6.fld2,_8.fld6.fld2,_5,_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2];
_8.fld4 = -_8.fld6.fld4;
RET = core::ptr::addr_of_mut!(_21);
SetDiscriminant(_8.fld6.fld6, 2);
_8.fld6.fld0.2 = (-1345266578039591245_i64) as u128;
_14 = (-103157336452245951356177256244725688065_i128);
place!(Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2)) = (_8.fld6.fld2, _8.fld6.fld2, _8.fld4);
_6 = -_19;
_1 = -_2;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).3 = 461150909153786886_i64 + (-8427424718828327661_i64);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).4 = _8.fld3.2 as u16;
RET = core::ptr::addr_of_mut!((*RET));
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).0 = Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0 << Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).1;
_6 = _19;
_8.fld6.fld7 = [_8.fld3.2];
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).2 = _8.fld6.fld3 - _8.fld6.fld3;
_8.fld6.fld0 = (6_usize, _3, _13);
_8.fld6.fld3 = Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).2 << _8.fld2;
_21 = _8.fld5 as f32;
_12 = core::ptr::addr_of!((*_12));
_8.fld0 = RET;
_10 = Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).3 as isize;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)) = (Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).1, _19, _8.fld6.fld3, 1291596494693392984_i64, Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).1);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).1 = -_6;
Call(_8.fld3 = fn12(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).3, Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0), Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).3 = 6892546452074200377_i64 >> _8.fld6.fld0.0;
_8.fld6.fld7 = [_8.fld3.2];
_5 = _8.fld5 as u16;
_8.fld6.fld0.1 = _3;
_10 = !_2;
_17 = Adt55::Variant3 { fld0: Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).1,fld1: Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2),fld2: (*RET) };
_18 = _8.fld6.fld7;
_8.fld4 = -_8.fld6.fld4;
place!(Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2)) = Field::<(u16, u16, i16)>(Variant(_17, 3), 1);
_20 = _8.fld3.0 as i32;
_8.fld6.fld7 = [_8.fld3.0];
_8.fld3.2 = _8.fld3.0;
Goto(bb7)
}
bb7 = {
_8.fld6.fld2 = _8.fld6.fld1 as u16;
_25.1.0.2 = _6 - _6;
_25.1.0.3 = (*_12) as isize;
_27.0 = [_1,_25.1.0.3,_1,_2,_10,_1,_8.fld2];
_4 = _8.fld6.fld0.1;
_25.0 = _12;
Goto(bb8)
}
bb8 = {
_23 = _19;
place!(Field::<(u16, u16, i16)>(Variant(_17, 3), 1)) = (Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0, Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0, (*_12));
_4 = _3;
_27.2 = &_20;
_28 = (_4, (*RET), _6, _2);
place!(Field::<(u16, u16, i16)>(Variant(_17, 3), 1)).0 = !Field::<u16>(Variant(_17, 3), 0);
_4 = Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).1 > Field::<u16>(Variant(_17, 3), 0);
place!(Field::<(u16, u16, i16)>(Variant(_17, 3), 1)).1 = _14 as u16;
place!(Field::<(u16, u16, i16)>(Variant(_17, 3), 1)).2 = _28.3 as i16;
_8.fld6.fld0 = (1885620842997742996_usize, _4, _13);
_29 = !_8.fld6.fld0.2;
_25.1.2 = _14 as u16;
_6 = -_25.1.0.2;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).0 = _4 as u16;
_7 = _8.fld6.fld5;
_25.1.0.1 = Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).3 as f32;
_27.1 = core::ptr::addr_of_mut!(_24);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).1 = _23;
_8.fld6.fld1 = _15 as u64;
SetDiscriminant(_17, 2);
match _8.fld6.fld0.0 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb4,
1885620842997742996 => bb10,
_ => bb9
}
}
bb9 = {
_8.fld6.fld2 = _8.fld6.fld1 as u16;
_25.1.0.2 = _6 - _6;
_25.1.0.3 = (*_12) as isize;
_27.0 = [_1,_25.1.0.3,_1,_2,_10,_1,_8.fld2];
_4 = _8.fld6.fld0.1;
_25.0 = _12;
Goto(bb8)
}
bb10 = {
_26 = Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).2;
_30 = 2722839818_u32;
place!(Field::<*const (i64, u128)>(Variant(_17, 2), 4)) = _8.fld3.1;
_8.fld6.fld0 = (1439164333846788273_usize, _4, _13);
_24 = _8.fld3.0;
_7 = [Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).0,Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0];
_8.fld2 = _28.3 & _1;
_25.1.0.0 = _3;
_31 = [_8.fld6.fld0.1,_4,_28.0,_8.fld6.fld0.1,_3,_4,_8.fld6.fld0.1];
_8.fld6.fld2 = Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0 | Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0;
_9 = _25.1.0.3;
Goto(bb11)
}
bb11 = {
_25.2 = _30 as usize;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1)).4 = _8.fld6.fld1 as u16;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).1 = _8.fld6.fld0.0 as f64;
_8.fld0 = _8.fld3.3;
_25.1.0.1 = _21 + (*RET);
_8.fld6.fld3 = -_26;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1)).3 = !Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).3;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).2 = -_8.fld6.fld3;
_25.1.2 = (*_12) as u16;
place!(Field::<u64>(Variant(_17, 2), 2)) = _8.fld6.fld1;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1)).3 = -Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).3;
_24 = _8.fld3.2;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0)).0 = _5;
match _8.fld6.fld0.0 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
1439164333846788273 => bb13,
_ => bb12
}
}
bb12 = {
_8.fld2 = 168127300_i32 as isize;
_8.fld5 = !187_u8;
_8.fld3.2 = _8.fld3.0;
_8.fld6.fld0.2 = 282301410066325054853005375165721356537_u128 - 30928016890574678088201960227363285216_u128;
_2 = _8.fld2;
_12 = core::ptr::addr_of!(_8.fld4);
_11 = [_5,_5,_5,_5,_5,_5];
_6 = _1 as f64;
_6 = 2680641177098456468_u64 as f64;
_8.fld6.fld1 = _8.fld5 as u64;
_8.fld6.fld3 = (-56_i8) | 113_i8;
_8.fld3.2 = _8.fld3.0;
_8.fld6.fld2 = _3 as u16;
_8.fld6.fld0.2 = !59928152392478452487176617315533102016_u128;
_8.fld6.fld3 = !(-79_i8);
_8.fld6.fld1 = 11849759175143492892_u64 & 7078181998407690185_u64;
_8.fld6.fld0.0 = 5_usize * 4_usize;
_8.fld6.fld0 = (2_usize, _4, 52061012391238668951160966678787637270_u128);
_11 = [_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_8.fld6.fld2,_5];
Goto(bb3)
}
bb13 = {
_16 = Adt44::Variant0 { fld0: _8.fld6.fld5,fld1: (*RET) };
_12 = core::ptr::addr_of!((*_12));
_8.fld3.3 = RET;
Goto(bb14)
}
bb14 = {
_30 = _8.fld6.fld1 as u32;
_20 = _15 ^ _15;
_25.1 = (_28, _8.fld0, Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0);
_32 = [Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).3,Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1).3,Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).3,Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1).3,Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1).3,Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).3];
place!(Field::<[u16; 6]>(Variant(_16, 0), 0)) = [_8.fld6.fld2,_25.1.2,_8.fld6.fld2,_25.1.2,_8.fld6.fld2,Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1).4];
_8.fld2 = _8.fld6.fld2 as isize;
_8.fld5 = !65_u8;
_7 = [Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).4,Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0,_8.fld6.fld2,_8.fld6.fld2,Field::<(u16, u16, i16)>(Variant(_8.fld6.fld6, 2), 2).0];
RET = core::ptr::addr_of_mut!(_21);
_8.fld6.fld0.1 = _28.0;
_15 = -_20;
place!(Field::<*const (i64, u128)>(Variant(_17, 2), 4)) = _8.fld3.1;
_25.1.2 = !_8.fld6.fld2;
_8.fld6.fld0.0 = _28.0 as usize;
_8.fld6.fld0.0 = _30 as usize;
_15 = _13 as i32;
_8.fld5 = (*_12) as u8;
RET = core::ptr::addr_of_mut!((*RET));
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1)).4 = Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).4;
_8.fld4 = _8.fld6.fld4 ^ _8.fld6.fld4;
_26 = !_8.fld6.fld3;
_25.1 = (_28, _8.fld0, Field::<(u16, f64, i8, i64, u16)>(Variant(_17, 2), 1).4);
place!(Field::<f64>(Variant(_16, 2), 5)) = -Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).1;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_16, 2), 3)).2 = !_25.1.2;
place!(Field::<f64>(Variant(_8.fld6.fld6, 2), 1)) = Field::<(u16, f64, i8, i64, u16)>(Variant(_8.fld6.fld6, 2), 0).1;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(3_usize, 15_usize, Move(_15), 1_usize, Move(_1), 26_usize, Move(_26), 30_usize, Move(_30)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(3_usize, 31_usize, Move(_31), 4_usize, Move(_4), 11_usize, Move(_11), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(3_usize, 32_usize, Move(_32), 18_usize, Move(_18), 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: bool,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: [u16; 6],mut _6: [u16; 6],mut _7: isize,mut _8: [u16; 6]) -> Adt49 {
mir! {
type RET = Adt49;
let _9: u16;
let _10: bool;
let _11: u128;
let _12: i8;
let _13: i128;
let _14: Adt44;
let _15: (u16, f64, i8, i64, u16);
let _16: (bool, f32, f64, isize);
let _17: isize;
let _18: i8;
let _19: bool;
let _20: [u16; 1];
let _21: i16;
let _22: usize;
let _23: char;
let _24: [char; 1];
let _25: Adt44;
let _26: u64;
let _27: u64;
let _28: isize;
let _29: (*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize);
let _30: [u32; 4];
let _31: bool;
let _32: ();
let _33: ();
{
RET.fld0.2 = 165418734774595642588716253414275836691_u128 << _2;
RET.fld3 = 7_i8 << RET.fld0.2;
RET.fld5 = _8;
_3 = _1;
RET.fld0 = (3_usize, _3, 11568613524620227509328069541024392636_u128);
_3 = !_4;
RET.fld0.0 = !3_usize;
RET.fld4 = 8646_i16;
RET.fld2 = !18879_u16;
RET.fld5 = _5;
RET.fld0.2 = _7 as u128;
_9 = RET.fld2 | RET.fld2;
_3 = _1;
_6 = _8;
_5 = [_9,_9,RET.fld2,_9,_9,_9];
RET.fld1 = 10621886766720532690_u64;
_9 = RET.fld2 - RET.fld2;
match RET.fld1 {
0 => bb1,
1 => bb2,
2 => bb3,
10621886766720532690 => bb5,
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
RET.fld7 = ['\u{8f761}'];
_7 = _2 | _2;
RET.fld0 = (4_usize, _3, 246001821242426113226079207809461838250_u128);
RET.fld4 = 3433167021_u32 as i16;
match RET.fld0.0 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb10,
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
Return()
}
bb10 = {
RET.fld0.2 = _4 as u128;
_6 = [RET.fld2,_9,RET.fld2,_9,_9,_9];
_5 = [_9,RET.fld2,_9,RET.fld2,_9,_9];
_8 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,_9,RET.fld2];
RET.fld5 = [_9,_9,RET.fld2,RET.fld2,_9,_9];
RET.fld5 = [_9,_9,_9,_9,RET.fld2,RET.fld2];
_3 = !RET.fld0.1;
RET.fld4 = -635_i16;
RET.fld3 = _7 as i8;
RET.fld2 = '\u{9f474}' as u16;
_10 = _1;
_12 = RET.fld3;
RET.fld4 = _10 as i16;
_7 = _2;
_11 = RET.fld0.2 << RET.fld0.2;
match RET.fld0.0 {
0 => bb7,
1 => bb5,
2 => bb3,
3 => bb9,
4 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_10 = _3;
RET.fld0.0 = 1154502992_u32 as usize;
RET.fld0.2 = _11;
_9 = 44_u8 as u16;
RET.fld4 = _11 as i16;
RET.fld0 = (5_usize, _1, _11);
_3 = _10;
RET.fld0.2 = (-1670461893_i32) as u128;
RET.fld1 = 215_u8 as u64;
RET.fld2 = _9;
_15.4 = !RET.fld2;
_16.3 = _2 & _7;
_8 = [_15.4,RET.fld2,_9,RET.fld2,RET.fld2,RET.fld2];
_8 = RET.fld5;
RET.fld5 = [_15.4,RET.fld2,RET.fld2,_9,RET.fld2,RET.fld2];
RET.fld3 = 4137761187_u32 as i8;
_9 = (-823820409_i32) as u16;
_15.3 = (-5577558549010425691_i64);
RET.fld4 = _1 as i16;
RET.fld0 = (5367464517492038520_usize, _3, _11);
RET.fld3 = _12;
_4 = !_3;
_17 = RET.fld1 as isize;
RET.fld0.0 = 9523109512938677605_usize | 4_usize;
_16.2 = RET.fld1 as f64;
match _15.3 {
0 => bb6,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
340282366920938463457797048882757785765 => bb20,
_ => bb19
}
}
bb14 = {
Return()
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
Return()
}
bb20 = {
_4 = RET.fld0.1;
RET.fld1 = (-1642799227_i32) as u64;
RET.fld0.1 = _3 & _1;
_15 = (_9, _16.2, _12, 2755724044757874006_i64, _9);
_19 = _1 | RET.fld0.1;
RET.fld7 = ['\u{743e8}'];
_15.3 = _16.3 as i64;
_18 = -_15.2;
_2 = 85552126_u32 as isize;
_20 = [_15.4];
RET.fld4 = (-20382_i16);
_21 = RET.fld4;
_20 = [_9];
_16.1 = 69455011_u32 as f32;
RET.fld0.0 = 7_usize + 0_usize;
_6 = [_9,RET.fld2,_15.0,_9,_15.0,_15.0];
_16.0 = !RET.fld0.1;
_10 = _15.3 >= _15.3;
_17 = _2;
Goto(bb21)
}
bb21 = {
_16.1 = RET.fld4 as f32;
_11 = !RET.fld0.2;
RET.fld3 = _18 + _15.2;
_12 = RET.fld3 - RET.fld3;
_13 = (-168932321276281444989323610825690498972_i128);
RET.fld4 = _15.3 as i16;
_15.0 = _15.4 + RET.fld2;
_15.4 = _16.3 as u16;
RET.fld2 = _19 as u16;
_15.2 = !RET.fld3;
RET.fld4 = _21;
RET.fld3 = 19_u8 as i8;
_15.3 = !(-6799593724060579053_i64);
_11 = !RET.fld0.2;
match RET.fld4 {
340282366920938463463374607431768191074 => bb23,
_ => bb22
}
}
bb22 = {
Return()
}
bb23 = {
_2 = !_16.3;
_7 = !_16.3;
RET.fld0 = (4_usize, _1, _11);
_24 = RET.fld7;
_16.3 = -_2;
_15 = (RET.fld2, _16.2, _18, (-7664981126087759155_i64), RET.fld2);
RET.fld7 = ['\u{4a0ec}'];
RET.fld0.2 = 2650203346_u32 as u128;
_13 = '\u{82b08}' as i128;
_12 = 1220003618_u32 as i8;
RET.fld0.0 = _16.2 as usize;
RET.fld0.2 = !_11;
_22 = RET.fld0.0;
_15.2 = 190_u8 as i8;
RET.fld0.1 = _16.0;
_7 = (-1229351561_i32) as isize;
_6 = [RET.fld2,_15.0,RET.fld2,RET.fld2,_15.4,_15.0];
_16.0 = !_4;
RET.fld3 = -_18;
RET.fld0.0 = !_22;
_15 = (RET.fld2, _16.2, _18, 9045581393322030595_i64, RET.fld2);
_14 = Adt44::Variant0 { fld0: _6,fld1: _16.1 };
_25 = _14;
_1 = RET.fld2 != RET.fld2;
RET.fld7 = ['\u{5c34f}'];
Call(RET = fn5(_16.0, _15.4, _25, _15.3, Field::<[u16; 6]>(Variant(_25, 0), 0), Field::<[u16; 6]>(Variant(_14, 0), 0), _25), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
RET.fld0.2 = _11;
_20 = [_15.4];
RET.fld5 = [_15.0,_15.4,_15.4,_15.4,RET.fld2,_15.4];
_1 = !_19;
SetDiscriminant(_25, 1);
RET.fld7 = ['\u{105317}'];
RET.fld2 = _15.0;
_16.0 = !_10;
place!(Field::<(bool, f32, f64, isize)>(Variant(_25, 1), 4)) = (_1, Field::<f32>(Variant(_14, 0), 1), _16.2, _7);
_15 = (RET.fld2, _16.2, _12, (-3356454821217263201_i64), RET.fld2);
_9 = _15.4 >> RET.fld0.2;
_25 = _14;
_16.3 = !_17;
_4 = _1;
RET.fld4 = _21 & _21;
_29.1.0.0 = !_19;
_10 = _19 & _4;
_5 = [RET.fld2,_15.4,_9,_9,_15.4,_9];
RET.fld3 = '\u{b0fb7}' as i8;
RET.fld7 = ['\u{1ba1e}'];
_16 = (_4, Field::<f32>(Variant(_14, 0), 1), _15.1, _2);
_16.3 = _2;
_4 = !_10;
_7 = _11 as isize;
RET.fld7 = ['\u{6a80}'];
Goto(bb25)
}
bb25 = {
Call(_32 = dump_var(4_usize, 18_usize, Move(_18), 10_usize, Move(_10), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_32 = dump_var(4_usize, 24_usize, Move(_24), 12_usize, Move(_12), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_32 = dump_var(4_usize, 19_usize, Move(_19), 17_usize, Move(_17), 33_usize, _33, 33_usize, _33), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: bool,mut _2: u16,mut _3: Adt44,mut _4: i64,mut _5: [u16; 6],mut _6: [u16; 6],mut _7: Adt44) -> Adt49 {
mir! {
type RET = Adt49;
let _8: bool;
let _9: Adt56;
let _10: usize;
let _11: f64;
let _12: ([isize; 7], *mut char, &'static i32);
let _13: u16;
let _14: ();
let _15: ();
{
_2 = 5469_u16;
RET.fld5 = _6;
place!(Field::<f32>(Variant(_3, 0), 1)) = 3126029199_u32 as f32;
RET.fld0.0 = 5_usize;
RET.fld0.1 = !_1;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
9045581393322030595 => bb5,
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
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = [_2,_2,_2,_2,_2,_2];
RET.fld1 = 14467472117827133048_u64;
match _4 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
9045581393322030595 => bb8,
_ => bb7
}
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
RET.fld0.1 = _4 <= _4;
RET.fld0 = (2_usize, _1, 277546803743903245383962501932969248482_u128);
match _4 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb9,
9045581393322030595 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
RET.fld2 = _2 << _4;
RET.fld1 = 9561505568147726766_u64;
_8 = RET.fld0.1;
_9.fld0 = [_8,_8,RET.fld0.1,RET.fld0.1,_8,_1,_1];
RET.fld0.1 = _8;
RET.fld2 = _2 & _2;
RET.fld3 = -72_i8;
_4 = 6162072368045176783_i64 >> RET.fld0.0;
_3 = _7;
RET.fld5 = [RET.fld2,RET.fld2,_2,RET.fld2,RET.fld2,RET.fld2];
_3 = _7;
place!(Field::<[u16; 6]>(Variant(_7, 0), 0)) = [RET.fld2,_2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_1 = RET.fld0.0 < RET.fld0.0;
RET.fld0.0 = 44_u8 as usize;
Call(place!(Field::<[u16; 6]>(Variant(_7, 0), 0)) = fn6(_3, _3, _3, _9.fld0, _6, _3, Field::<[u16; 6]>(Variant(_3, 0), 0), _3, RET.fld0.2, _3, _1, RET.fld0.2, _5, _2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET.fld0.2 = 3437570508_u32 as u128;
SetDiscriminant(_3, 0);
_9.fld1 = [(-9223372036854775808_isize),28_isize,9223372036854775807_isize,87_isize,(-9223372036854775808_isize),9223372036854775807_isize,12_isize];
RET.fld7 = ['\u{f4f9c}'];
_3 = _7;
_5 = _6;
Call(RET = fn11(_7, _6, _7, _3, _9, _7, _3, _7, _9.fld1, _7, _5, _3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
match RET.fld0.0 {
0 => bb8,
1 => bb14,
2 => bb15,
5557644557378790946 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_4 = -(-3545248100081877628_i64);
_7 = Adt44::Variant0 { fld0: _6,fld1: Field::<f32>(Variant(_3, 0), 1) };
Goto(bb18)
}
bb18 = {
Call(_14 = dump_var(5_usize, 4_usize, Move(_4), 5_usize, Move(_5), 6_usize, Move(_6), 15_usize, _15), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: Adt44,mut _2: Adt44,mut _3: Adt44,mut _4: [bool; 7],mut _5: [u16; 6],mut _6: Adt44,mut _7: [u16; 6],mut _8: Adt44,mut _9: u128,mut _10: Adt44,mut _11: bool,mut _12: u128,mut _13: [u16; 6],mut _14: u16) -> [u16; 6] {
mir! {
type RET = [u16; 6];
let _15: f32;
let _16: u128;
let _17: [i64; 6];
let _18: Adt59;
let _19: Adt56;
let _20: char;
let _21: Adt49;
let _22: u32;
let _23: isize;
let _24: (usize, bool, u128);
let _25: (u16, f64, i8, i64, u16);
let _26: [i64; 6];
let _27: i128;
let _28: [u16; 6];
let _29: ();
let _30: ();
{
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = [_14,_14,_14,_14,_14,_14];
_6 = _3;
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
277546803743903245383962501932969248482 => bb7,
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
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = Field::<[u16; 6]>(Variant(_1, 0), 0);
SetDiscriminant(_1, 1);
RET = [_14,_14,_14,_14,_14,_14];
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [_14,_14,_14,_14,_14,_14];
place!(Field::<(usize, bool, u128)>(Variant(_1, 1), 7)) = (7_usize, _11, _9);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).4 = _14;
place!(Field::<[u16; 6]>(Variant(_1, 1), 5)) = [Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,_14,_14,_14];
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).3 = Field::<(usize, bool, u128)>(Variant(_1, 1), 7).0 as isize;
place!(Field::<f32>(Variant(_10, 0), 1)) = _12 as f32;
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).2 = (-867060207657150509_i64) as f64;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).0 = _14 % Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4;
place!(Field::<(usize, bool, u128)>(Variant(_1, 1), 7)).2 = !_12;
place!(Field::<[u16; 6]>(Variant(_10, 0), 0)) = [Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4];
_4 = [Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,_11,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1];
place!(Field::<f32>(Variant(_2, 0), 1)) = Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4).3 as f32;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).2 = (-103_i8) * 26_i8;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)) = (_14, Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4).2, 73_i8, 8366836504781561222_i64, _14);
place!(Field::<*mut char>(Variant(_1, 1), 2)) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_1, 1), 1)));
place!(Field::<[u16; 6]>(Variant(_1, 1), 5)) = [_14,_14,_14,_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0];
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).3 = (-9223372036854775808_isize) | 9223372036854775807_isize;
place!(Field::<(usize, bool, u128)>(Variant(_1, 1), 7)).1 = _11 ^ _11;
_3 = _6;
_4 = [Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,_11,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,_11];
Call(_2 = fn7(_10, _6, Field::<(usize, bool, u128)>(Variant(_1, 1), 7), Field::<(usize, bool, u128)>(Variant(_1, 1), 7).0, _12, Field::<*mut char>(Variant(_1, 1), 2), _3, _11, _10, Field::<(usize, bool, u128)>(Variant(_1, 1), 7), Field::<f32>(Variant(_10, 0), 1), _10, Field::<(usize, bool, u128)>(Variant(_1, 1), 7)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).1 = Field::<f32>(Variant(_2, 0), 1) + Field::<f32>(Variant(_6, 0), 1);
place!(Field::<[u16; 6]>(Variant(_2, 0), 0)) = Field::<[u16; 6]>(Variant(_8, 0), 0);
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = [_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0];
place!(Field::<(usize, bool, u128)>(Variant(_1, 1), 7)).0 = !1184014037825073815_usize;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).0 = _14;
_5 = _7;
place!(Field::<[u16; 6]>(Variant(_10, 0), 0)) = [_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0,_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4];
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).1 = -Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4).2;
place!(Field::<f32>(Variant(_6, 0), 1)) = -Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4).1;
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = [_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0];
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).1 = -Field::<f32>(Variant(_2, 0), 1);
_1 = _10;
_17 = [(-6141655627056187394_i64),4565742279080678079_i64,2527946791794447011_i64,(-7905228207046847708_i64),6057187951769174700_i64,(-8269054279004716800_i64)];
_11 = true;
place!(Field::<f32>(Variant(_3, 0), 1)) = -Field::<f32>(Variant(_2, 0), 1);
Goto(bb9)
}
bb9 = {
_4 = [_11,_11,_11,_11,_11,_11,_11];
_15 = Field::<f32>(Variant(_2, 0), 1);
place!(Field::<f32>(Variant(_3, 0), 1)) = Field::<f32>(Variant(_6, 0), 1);
match _12 {
0 => bb1,
1 => bb8,
2 => bb6,
3 => bb10,
4 => bb11,
277546803743903245383962501932969248482 => bb13,
_ => bb12
}
}
bb10 = {
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).1 = Field::<f32>(Variant(_2, 0), 1) + Field::<f32>(Variant(_6, 0), 1);
place!(Field::<[u16; 6]>(Variant(_2, 0), 0)) = Field::<[u16; 6]>(Variant(_8, 0), 0);
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = [_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0];
place!(Field::<(usize, bool, u128)>(Variant(_1, 1), 7)).0 = !1184014037825073815_usize;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).0 = _14;
_5 = _7;
place!(Field::<[u16; 6]>(Variant(_10, 0), 0)) = [_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0,_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4];
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).1 = -Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4).2;
place!(Field::<f32>(Variant(_6, 0), 1)) = -Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4).1;
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = [_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0];
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).1 = -Field::<f32>(Variant(_2, 0), 1);
_1 = _10;
_17 = [(-6141655627056187394_i64),4565742279080678079_i64,2527946791794447011_i64,(-7905228207046847708_i64),6057187951769174700_i64,(-8269054279004716800_i64)];
_11 = true;
place!(Field::<f32>(Variant(_3, 0), 1)) = -Field::<f32>(Variant(_2, 0), 1);
Goto(bb9)
}
bb11 = {
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = Field::<[u16; 6]>(Variant(_1, 0), 0);
SetDiscriminant(_1, 1);
RET = [_14,_14,_14,_14,_14,_14];
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [_14,_14,_14,_14,_14,_14];
place!(Field::<(usize, bool, u128)>(Variant(_1, 1), 7)) = (7_usize, _11, _9);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).4 = _14;
place!(Field::<[u16; 6]>(Variant(_1, 1), 5)) = [Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,_14,_14,_14];
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).3 = Field::<(usize, bool, u128)>(Variant(_1, 1), 7).0 as isize;
place!(Field::<f32>(Variant(_10, 0), 1)) = _12 as f32;
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).2 = (-867060207657150509_i64) as f64;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).0 = _14 % Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4;
place!(Field::<(usize, bool, u128)>(Variant(_1, 1), 7)).2 = !_12;
place!(Field::<[u16; 6]>(Variant(_10, 0), 0)) = [Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4,_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).4];
_4 = [Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,_11,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1];
place!(Field::<f32>(Variant(_2, 0), 1)) = Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4).3 as f32;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)).2 = (-103_i8) * 26_i8;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6)) = (_14, Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4).2, 73_i8, 8366836504781561222_i64, _14);
place!(Field::<*mut char>(Variant(_1, 1), 2)) = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_1, 1), 1)));
place!(Field::<[u16; 6]>(Variant(_1, 1), 5)) = [_14,_14,_14,_14,_14,Field::<(u16, f64, i8, i64, u16)>(Variant(_1, 1), 6).0];
place!(Field::<(bool, f32, f64, isize)>(Variant(_1, 1), 4)).3 = (-9223372036854775808_isize) | 9223372036854775807_isize;
place!(Field::<(usize, bool, u128)>(Variant(_1, 1), 7)).1 = _11 ^ _11;
_3 = _6;
_4 = [Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,_11,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,Field::<(usize, bool, u128)>(Variant(_1, 1), 7).1,_11];
Call(_2 = fn7(_10, _6, Field::<(usize, bool, u128)>(Variant(_1, 1), 7), Field::<(usize, bool, u128)>(Variant(_1, 1), 7).0, _12, Field::<*mut char>(Variant(_1, 1), 2), _3, _11, _10, Field::<(usize, bool, u128)>(Variant(_1, 1), 7), Field::<f32>(Variant(_10, 0), 1), _10, Field::<(usize, bool, u128)>(Variant(_1, 1), 7)), ReturnTo(bb8), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_13 = [_14,_14,_14,_14,_14,_14];
_21.fld0.2 = _12 % _9;
_6 = _3;
_21.fld1 = !12914804443580872524_u64;
place!(Field::<f32>(Variant(_3, 0), 1)) = -Field::<f32>(Variant(_6, 0), 1);
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = [_14,_14,_14,_14,_14,_14];
SetDiscriminant(_8, 0);
_19.fld0 = [_11,_11,_11,_11,_11,_11,_11];
place!(Field::<[u16; 6]>(Variant(_1, 0), 0)) = _7;
_21.fld3 = 101_i8;
_6 = _2;
_21.fld0 = (6_usize, _11, _12);
_19.fld0 = _4;
place!(Field::<[u16; 6]>(Variant(_2, 0), 0)) = [_14,_14,_14,_14,_14,_14];
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [_14,_14,_14,_14,_14,_14];
place!(Field::<f32>(Variant(_2, 0), 1)) = -Field::<f32>(Variant(_6, 0), 1);
_13 = [_14,_14,_14,_14,_14,_14];
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)).2 = !_14;
SetDiscriminant(_6, 0);
_15 = -Field::<f32>(Variant(_1, 0), 1);
_21.fld1 = _21.fld0.2 as u64;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)).0.1 = Field::<f32>(Variant(_3, 0), 1) * Field::<f32>(Variant(_1, 0), 1);
_24 = (_21.fld0.0, _21.fld0.1, _21.fld0.2);
Call(_21.fld3 = fn8(Field::<[u16; 6]>(Variant(_1, 0), 0), Field::<f32>(Variant(_2, 0), 1), _5, _2, _3, _2, _2, _1, _21.fld1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_15 = Field::<f32>(Variant(_3, 0), 1) * Field::<f32>(Variant(_2, 0), 1);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_10, 2), 4)).4 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3).2;
_10 = Adt44::Variant0 { fld0: _7,fld1: Field::<f32>(Variant(_1, 0), 1) };
_22 = 2213120577_u32;
_7 = [_14,_14,_14,_14,_14,_14];
_8 = _2;
_20 = '\u{666d3}';
_5 = [_14,_14,_14,_14,_14,_14];
_21.fld0.2 = _24.2 << _24.2;
_21.fld5 = [_14,_14,_14,_14,_14,_14];
_16 = !_21.fld0.2;
_4 = _19.fld0;
_23 = (-97422283945570461077322145163605802823_i128) as isize;
_26 = [(-4949112924656026798_i64),8246259520064737255_i64,(-7194797130289135317_i64),1088051064952951892_i64,9086923176926097982_i64,7657432747675722255_i64];
_25.1 = _21.fld1 as f64;
SetDiscriminant(_1, 0);
_9 = _16;
_25.3 = -(-3645089784694379945_i64);
_16 = !_21.fld0.2;
_20 = '\u{70802}';
_21.fld2 = _14 / _14;
_27 = 42180742766829962914291721517479502359_i128;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(6_usize, 17_usize, Move(_17), 7_usize, Move(_7), 4_usize, Move(_4), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(6_usize, 13_usize, Move(_13), 24_usize, Move(_24), 12_usize, Move(_12), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: Adt44,mut _2: Adt44,mut _3: (usize, bool, u128),mut _4: usize,mut _5: u128,mut _6: *mut char,mut _7: Adt44,mut _8: bool,mut _9: Adt44,mut _10: (usize, bool, u128),mut _11: f32,mut _12: Adt44,mut _13: (usize, bool, u128)) -> Adt44 {
mir! {
type RET = Adt44;
let _14: Adt55;
let _15: ();
let _16: ();
{
_7 = Adt44::Variant0 { fld0: Field::<[u16; 6]>(Variant(_2, 0), 0),fld1: Field::<f32>(Variant(_1, 0), 1) };
_3 = _10;
place!(Field::<f32>(Variant(_12, 0), 1)) = Field::<f32>(Variant(_2, 0), 1) * Field::<f32>(Variant(_9, 0), 1);
_8 = !_13.1;
place!(Field::<f32>(Variant(_1, 0), 1)) = -Field::<f32>(Variant(_9, 0), 1);
_10.0 = _4 & _3.0;
RET = _7;
_10 = _13;
place!(Field::<f32>(Variant(RET, 0), 1)) = Field::<f32>(Variant(_7, 0), 1);
_13.2 = _10.2;
place!(Field::<f32>(Variant(_9, 0), 1)) = Field::<f32>(Variant(RET, 0), 1);
place!(Field::<bool>(Variant(_12, 2), 0)) = _10.1;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_12, 2), 7)).1.0.2 = _13.0 as f64;
_1 = _7;
place!(Field::<[u16; 6]>(Variant(RET, 0), 0)) = Field::<[u16; 6]>(Variant(_7, 0), 0);
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_12, 2), 7)).1.1 = core::ptr::addr_of_mut!(place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_12, 2), 7)).1.0.1);
RET = _1;
place!(Field::<(i64, u128)>(Variant(_12, 2), 2)) = ((-6749407619856726016_i64), _13.2);
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(7_usize, 13_usize, Move(_13), 10_usize, Move(_10), 3_usize, Move(_3), 16_usize, _16), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [u16; 6],mut _2: f32,mut _3: [u16; 6],mut _4: Adt44,mut _5: Adt44,mut _6: Adt44,mut _7: Adt44,mut _8: Adt44,mut _9: u64) -> i8 {
mir! {
type RET = i8;
let _10: u128;
let _11: char;
let _12: u8;
let _13: bool;
let _14: u128;
let _15: *mut char;
let _16: [u16; 6];
let _17: isize;
let _18: (u16, u16, i16);
let _19: i128;
let _20: *mut u32;
let _21: Adt43;
let _22: [isize; 7];
let _23: Adt48;
let _24: i8;
let _25: [isize; 7];
let _26: Adt57;
let _27: i8;
let _28: i16;
let _29: Adt47;
let _30: *const (i64, u128);
let _31: [u16; 1];
let _32: char;
let _33: i8;
let _34: [char; 1];
let _35: ();
let _36: ();
{
RET = !63_i8;
_5 = _4;
place!(Field::<f32>(Variant(_6, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) - Field::<f32>(Variant(_4, 0), 1);
Goto(bb1)
}
bb1 = {
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = _1;
place!(Field::<[u16; 6]>(Variant(_5, 0), 0)) = [50820_u16,30627_u16,18615_u16,57053_u16,46503_u16,53746_u16];
_9 = false as u64;
_9 = 16570737046854606611_u64;
_4 = _5;
place!(Field::<f32>(Variant(_4, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) + _2;
place!(Field::<f32>(Variant(_7, 0), 1)) = Field::<f32>(Variant(_6, 0), 1);
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).1.0.1 = Field::<f32>(Variant(_4, 0), 1);
place!(Field::<f32>(Variant(_4, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) + Field::<f32>(Variant(_5, 0), 1);
_9 = !8928785338958003629_u64;
place!(Field::<(i64, u128)>(Variant(_8, 2), 2)).1 = 103885117261907760861213072300637206463_u128 - 45325307771263404108415891600317028488_u128;
place!(Field::<(i64, u128)>(Variant(_8, 2), 2)).0 = 9032105186178727314_i64 & 1003820610332514618_i64;
_1 = Field::<[u16; 6]>(Variant(_6, 0), 0);
_11 = '\u{48c01}';
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).0.2 = RET as f64;
place!(Field::<f64>(Variant(_8, 2), 5)) = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3).0.2 * Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3).0.2;
Call(place!(Field::<f32>(Variant(_7, 0), 1)) = fn9(_5, _3, _2, _5, _5, _4, _6, _4, _1, Field::<[u16; 6]>(Variant(_5, 0), 0), _3, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = [52825_u16,51267_u16,19220_u16,44459_u16,19375_u16,24028_u16];
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).1 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_7, 0), 1)));
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).2 = !5_usize;
_8 = _5;
_9 = !3513222464404336937_u64;
_11 = '\u{b05e4}';
_11 = '\u{bc555}';
RET = 93_i8 + 113_i8;
_2 = Field::<f32>(Variant(_7, 0), 1) + Field::<f32>(Variant(_4, 0), 1);
_6 = _4;
place!(Field::<f32>(Variant(_6, 0), 1)) = Field::<f32>(Variant(_4, 0), 1);
_10 = false as u128;
place!(Field::<f32>(Variant(_6, 0), 1)) = RET as f32;
_15 = core::ptr::addr_of_mut!(_11);
_14 = _10 & _10;
_15 = core::ptr::addr_of_mut!((*_15));
place!(Field::<[u16; 6]>(Variant(_5, 0), 0)) = Field::<[u16; 6]>(Variant(_7, 0), 0);
_7 = Adt44::Variant0 { fld0: _3,fld1: _2 };
_12 = (*_15) as u8;
_3 = [57160_u16,30815_u16,19598_u16,39208_u16,22948_u16,62217_u16];
Call(place!(Field::<f32>(Variant(_4, 0), 1)) = fn10(_7, _6, _8, Field::<f32>(Variant(_7, 0), 1), _7, _7, _7, _8, _3, _7, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
place!(Field::<f32>(Variant(_6, 0), 1)) = -Field::<f32>(Variant(_4, 0), 1);
_22 = [87_isize,(-42_isize),(-87_isize),9223372036854775807_isize,83_isize,(-39_isize),108_isize];
place!(Field::<f32>(Variant(_4, 0), 1)) = -_2;
place!(Field::<[u16; 6]>(Variant(_4, 0), 0)) = [61762_u16,26295_u16,25286_u16,31152_u16,18737_u16,56368_u16];
place!(Field::<f32>(Variant(_6, 0), 1)) = Field::<f32>(Variant(_7, 0), 1);
_18 = (23996_u16, 43348_u16, 30277_i16);
_10 = _14 | _14;
_18 = (49415_u16, 60486_u16, 28103_i16);
_3 = [_18.0,_18.0,_18.1,_18.0,_18.1,_18.1];
_14 = _18.1 as u128;
_6 = Adt44::Variant0 { fld0: Field::<[u16; 6]>(Variant(_7, 0), 0),fld1: Field::<f32>(Variant(_7, 0), 1) };
_6 = _4;
_4 = _8;
_9 = (-41469297844686526845865100192925130066_i128) as u64;
_17 = 9223372036854775807_isize - 9223372036854775807_isize;
_9 = _2 as u64;
_7 = Adt44::Variant0 { fld0: Field::<[u16; 6]>(Variant(_4, 0), 0),fld1: _2 };
place!(Field::<f32>(Variant(_5, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) + Field::<f32>(Variant(_7, 0), 1);
_13 = !true;
_13 = true;
match _18.1 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
60486 => bb8,
_ => bb7
}
}
bb4 = {
_1 = [52825_u16,51267_u16,19220_u16,44459_u16,19375_u16,24028_u16];
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).1 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_7, 0), 1)));
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).2 = !5_usize;
_8 = _5;
_9 = !3513222464404336937_u64;
_11 = '\u{b05e4}';
_11 = '\u{bc555}';
RET = 93_i8 + 113_i8;
_2 = Field::<f32>(Variant(_7, 0), 1) + Field::<f32>(Variant(_4, 0), 1);
_6 = _4;
place!(Field::<f32>(Variant(_6, 0), 1)) = Field::<f32>(Variant(_4, 0), 1);
_10 = false as u128;
place!(Field::<f32>(Variant(_6, 0), 1)) = RET as f32;
_15 = core::ptr::addr_of_mut!(_11);
_14 = _10 & _10;
_15 = core::ptr::addr_of_mut!((*_15));
place!(Field::<[u16; 6]>(Variant(_5, 0), 0)) = Field::<[u16; 6]>(Variant(_7, 0), 0);
_7 = Adt44::Variant0 { fld0: _3,fld1: _2 };
_12 = (*_15) as u8;
_3 = [57160_u16,30815_u16,19598_u16,39208_u16,22948_u16,62217_u16];
Call(place!(Field::<f32>(Variant(_4, 0), 1)) = fn10(_7, _6, _8, Field::<f32>(Variant(_7, 0), 1), _7, _7, _7, _8, _3, _7, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = _1;
place!(Field::<[u16; 6]>(Variant(_5, 0), 0)) = [50820_u16,30627_u16,18615_u16,57053_u16,46503_u16,53746_u16];
_9 = false as u64;
_9 = 16570737046854606611_u64;
_4 = _5;
SetDiscriminant(_8, 2);
place!(Field::<f32>(Variant(_4, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) + _2;
place!(Field::<f32>(Variant(_7, 0), 1)) = Field::<f32>(Variant(_6, 0), 1);
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).1.0.1 = Field::<f32>(Variant(_4, 0), 1);
place!(Field::<f32>(Variant(_4, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) + Field::<f32>(Variant(_5, 0), 1);
_9 = !8928785338958003629_u64;
place!(Field::<(i64, u128)>(Variant(_8, 2), 2)).1 = 103885117261907760861213072300637206463_u128 - 45325307771263404108415891600317028488_u128;
place!(Field::<(i64, u128)>(Variant(_8, 2), 2)).0 = 9032105186178727314_i64 & 1003820610332514618_i64;
_1 = Field::<[u16; 6]>(Variant(_6, 0), 0);
_11 = '\u{48c01}';
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).0.2 = RET as f64;
place!(Field::<f64>(Variant(_8, 2), 5)) = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3).0.2 * Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3).0.2;
Call(place!(Field::<f32>(Variant(_7, 0), 1)) = fn9(_5, _3, _2, _5, _5, _4, _6, _4, _1, Field::<[u16; 6]>(Variant(_5, 0), 0), _3, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_8 = _5;
_18 = (52130_u16, 24142_u16, (-25080_i16));
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3)).0.2 = 467460554_i32 as f64;
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = Field::<[u16; 6]>(Variant(_7, 0), 0);
_22 = [_17,_17,_17,_17,_17,_17,_17];
_10 = !_14;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3)).0.3 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3).0.2 as isize;
Goto(bb9)
}
bb9 = {
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_5, 2), 4)).1 = -Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3).0.2;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_5, 2), 4)).1 = -Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3).0.2;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_5, 2), 7)).2 = _9 as usize;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_5, 2), 4)).2 = -RET;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_5, 2), 7)).1.0.2 = Field::<(u16, f64, i8, i64, u16)>(Variant(_5, 2), 4).1 + Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3).0.2;
Goto(bb10)
}
bb10 = {
_5 = _7;
_28 = _18.2 + _18.2;
_14 = _10 | _10;
_16 = [_18.0,_18.0,_18.1,_18.1,_18.0,_18.1];
_27 = RET & RET;
place!(Field::<f32>(Variant(_6, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) + Field::<f32>(Variant(_8, 0), 1);
_4 = _8;
_28 = _18.2 - _18.2;
place!(Field::<f32>(Variant(_4, 0), 1)) = Field::<f32>(Variant(_8, 0), 1) - Field::<f32>(Variant(_6, 0), 1);
_18.1 = _18.0;
place!(Field::<[u16; 6]>(Variant(_5, 0), 0)) = Field::<[u16; 6]>(Variant(_7, 0), 0);
_14 = _10 & _10;
_2 = -Field::<f32>(Variant(_8, 0), 1);
SetDiscriminant(_6, 1);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).1 = _27 as f64;
_10 = !_14;
_22 = [_17,_17,_17,_17,_17,_17,_17];
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)).2 = _27 as u128;
_6 = _7;
_17 = (-387983983_i32) as isize;
RET = _27 - _27;
_31 = [_18.0];
_10 = _12 as u128;
_1 = Field::<[u16; 6]>(Variant(_6, 0), 0);
RET = _27 - _27;
Goto(bb11)
}
bb11 = {
_1 = [_18.1,_18.1,_18.0,_18.0,_18.1,_18.1];
_14 = _10;
_21 = Adt43::Variant1 { fld0: _18.2 };
place!(Field::<f32>(Variant(_8, 0), 1)) = -Field::<f32>(Variant(_4, 0), 1);
_32 = (*_15);
_31 = [_18.1];
_27 = _9 as i8;
_12 = _28 as u8;
_18 = (34206_u16, 62560_u16, _28);
_25 = [_17,_17,_17,_17,_17,_17,_17];
_1 = Field::<[u16; 6]>(Variant(_6, 0), 0);
_5 = _4;
_19 = (-75774464293967906673689078957427615823_i128);
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [_18.0,_18.0,_18.0,_18.0,_18.1,_18.1];
_7 = _4;
SetDiscriminant(_5, 1);
_27 = !RET;
Goto(bb12)
}
bb12 = {
_30 = core::ptr::addr_of!(place!(Field::<(i64, u128)>(Variant(_6, 2), 2)));
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_6, 2), 7)).1.1 = core::ptr::addr_of_mut!(place!(Field::<(bool, f32, f64, isize)>(Variant(_5, 1), 4)).1);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 2), 4)).2 = RET;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_5, 1), 6)).1 = _12 as f64;
_33 = RET ^ Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 2), 4).2;
_17 = 9223372036854775807_isize ^ (-128_isize);
place!(Field::<[u16; 6]>(Variant(_5, 1), 5)) = [_18.0,_18.0,_18.1,_18.0,_18.0,_18.1];
place!(Field::<(bool, f32, f64, isize)>(Variant(_5, 1), 4)) = (_13, _2, Field::<(u16, f64, i8, i64, u16)>(Variant(_5, 1), 6).1, _17);
_9 = 10498142074384817602_u64 >> _17;
_7 = _4;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_6, 2), 3)).0.2 = Field::<(bool, f32, f64, isize)>(Variant(_5, 1), 4).2 + Field::<(bool, f32, f64, isize)>(Variant(_5, 1), 4).2;
place!(Field::<*mut char>(Variant(_5, 1), 2)) = _15;
_18.1 = _18.0 ^ _18.0;
place!(Field::<(usize, bool, u128)>(Variant(_5, 1), 7)).0 = 3820711700997668736_usize;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 2), 4)).0 = !_18.0;
_10 = !_14;
_33 = Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 2), 4).2;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_6, 2), 3)).0 = (_13, Field::<f32>(Variant(_7, 0), 1), Field::<(bool, f32, f64, isize)>(Variant(_5, 1), 4).2, _17);
place!(Field::<(usize, bool, u128)>(Variant(_5, 1), 7)).2 = !_14;
match Field::<i16>(Variant(_21, 1), 0) {
0 => bb6,
1 => bb2,
2 => bb7,
3 => bb8,
4 => bb11,
5 => bb13,
6 => bb14,
340282366920938463463374607431768186376 => bb16,
_ => bb15
}
}
bb13 = {
_1 = [_18.1,_18.1,_18.0,_18.0,_18.1,_18.1];
_14 = _10;
_21 = Adt43::Variant1 { fld0: _18.2 };
place!(Field::<f32>(Variant(_8, 0), 1)) = -Field::<f32>(Variant(_4, 0), 1);
_32 = (*_15);
_31 = [_18.1];
_27 = _9 as i8;
_12 = _28 as u8;
_18 = (34206_u16, 62560_u16, _28);
_25 = [_17,_17,_17,_17,_17,_17,_17];
_1 = Field::<[u16; 6]>(Variant(_6, 0), 0);
_5 = _4;
_19 = (-75774464293967906673689078957427615823_i128);
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [_18.0,_18.0,_18.0,_18.0,_18.1,_18.1];
_7 = _4;
SetDiscriminant(_5, 1);
_27 = !RET;
SetDiscriminant(_6, 2);
Goto(bb12)
}
bb14 = {
_5 = _7;
_28 = _18.2 + _18.2;
_14 = _10 | _10;
_16 = [_18.0,_18.0,_18.1,_18.1,_18.0,_18.1];
_27 = RET & RET;
place!(Field::<f32>(Variant(_6, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) + Field::<f32>(Variant(_8, 0), 1);
_4 = _8;
_28 = _18.2 - _18.2;
place!(Field::<f32>(Variant(_4, 0), 1)) = Field::<f32>(Variant(_8, 0), 1) - Field::<f32>(Variant(_6, 0), 1);
_18.1 = _18.0;
place!(Field::<[u16; 6]>(Variant(_5, 0), 0)) = Field::<[u16; 6]>(Variant(_7, 0), 0);
_14 = _10 & _10;
_2 = -Field::<f32>(Variant(_8, 0), 1);
SetDiscriminant(_6, 1);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).1 = _27 as f64;
_10 = !_14;
_22 = [_17,_17,_17,_17,_17,_17,_17];
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)).2 = _27 as u128;
_6 = _7;
_17 = (-387983983_i32) as isize;
RET = _27 - _27;
_31 = [_18.0];
_10 = _12 as u128;
_1 = Field::<[u16; 6]>(Variant(_6, 0), 0);
RET = _27 - _27;
Goto(bb11)
}
bb15 = {
_8 = _5;
_18 = (52130_u16, 24142_u16, (-25080_i16));
SetDiscriminant(_5, 2);
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3)).0.2 = 467460554_i32 as f64;
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = Field::<[u16; 6]>(Variant(_7, 0), 0);
_22 = [_17,_17,_17,_17,_17,_17,_17];
_10 = !_14;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3)).0.3 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_5, 2), 3).0.2 as isize;
Goto(bb9)
}
bb16 = {
_23 = Adt48::Variant1 { fld0: Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 2), 4).0,fld1: Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_6, 2), 7).1.1,fld2: _17,fld3: _18,fld4: _9,fld5: Field::<(bool, f32, f64, isize)>(Variant(_5, 1), 4).2 };
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_6, 2), 7)).1.0.3 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_6, 2), 3).0.3 << _18.0;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_6, 2), 7)).1 = (Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_6, 2), 3).0, Field::<*mut f32>(Variant(_23, 1), 1), Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 2), 4).0);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 2), 4)).4 = Field::<(u16, u16, i16)>(Variant(_23, 1), 3).1 >> Field::<u16>(Variant(_23, 1), 0);
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_6, 2), 3)).2 = Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 2), 4).0;
place!(Field::<(bool, f32, f64, isize)>(Variant(_5, 1), 4)).3 = _17 ^ _17;
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(8_usize, 32_usize, Move(_32), 28_usize, Move(_28), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(8_usize, 33_usize, Move(_33), 13_usize, Move(_13), 19_usize, Move(_19), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(8_usize, 12_usize, Move(_12), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: Adt44,mut _2: [u16; 6],mut _3: f32,mut _4: Adt44,mut _5: Adt44,mut _6: Adt44,mut _7: Adt44,mut _8: Adt44,mut _9: [u16; 6],mut _10: [u16; 6],mut _11: [u16; 6],mut _12: f32) -> f32 {
mir! {
type RET = f32;
let _13: [i64; 6];
let _14: i64;
let _15: bool;
let _16: u64;
let _17: char;
let _18: ();
let _19: ();
{
place!(Field::<[u16; 6]>(Variant(_4, 0), 0)) = [44174_u16,27757_u16,42998_u16,30516_u16,33731_u16,749_u16];
place!(Field::<f32>(Variant(_5, 0), 1)) = Field::<f32>(Variant(_7, 0), 1) * Field::<f32>(Variant(_1, 0), 1);
place!(Field::<f32>(Variant(_5, 0), 1)) = -Field::<f32>(Variant(_1, 0), 1);
place!(Field::<[u16; 6]>(Variant(_4, 0), 0)) = [39741_u16,24226_u16,9546_u16,31222_u16,8565_u16,25689_u16];
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = [64290_u16,15541_u16,39162_u16,51281_u16,59143_u16,21909_u16];
place!(Field::<[u16; 6]>(Variant(_1, 0), 0)) = [33960_u16,40334_u16,17776_u16,16069_u16,54532_u16,51156_u16];
_11 = Field::<[u16; 6]>(Variant(_5, 0), 0);
_2 = [12411_u16,25752_u16,6864_u16,21618_u16,38353_u16,58401_u16];
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = Field::<[u16; 6]>(Variant(_7, 0), 0);
place!(Field::<[u16; 6]>(Variant(_7, 0), 0)) = [31741_u16,46885_u16,55742_u16,22950_u16,18366_u16,18915_u16];
SetDiscriminant(_6, 1);
RET = 5640304812102226390_i64 as f32;
_12 = (-166133136778060645186098845039966960878_i128) as f32;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).0 = 32256_u16;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).4 = true as u16;
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)).2 = !323300012047135177623938124103634208938_u128;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).1 = 1343827258_i32 as f64;
place!(Field::<[u16; 6]>(Variant(_5, 0), 0)) = [Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0];
place!(Field::<f32>(Variant(_4, 0), 1)) = Field::<f32>(Variant(_8, 0), 1);
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)).2 = false as u128;
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).2 = -Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).1;
_13 = [5240193146858727951_i64,(-5517545428134541232_i64),1850162065837218027_i64,(-1269808965733820844_i64),7455255942026595923_i64,6558884964528845061_i64];
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).0 = Field::<f32>(Variant(_4, 0), 1) != Field::<f32>(Variant(_4, 0), 1);
Goto(bb1)
}
bb1 = {
_1 = _7;
place!(Field::<[u16; 6]>(Variant(_6, 1), 5)) = [Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4];
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).3 = 9223372036854775807_isize >> Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0;
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4];
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)).1 = Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).0 < Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).0;
_5 = _7;
place!(Field::<f32>(Variant(_4, 0), 1)) = _3;
match Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
32256 => bb10,
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
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)).0 = 17891018552027295628_usize;
place!(Field::<[u16; 6]>(Variant(_1, 0), 0)) = [Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4];
place!(Field::<[u32; 4]>(Variant(_6, 1), 0)) = [1626049981_u32,163631048_u32,559839287_u32,2973160184_u32];
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).3 = -(-7698942705044031384_i64);
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).1 = Field::<f32>(Variant(_8, 0), 1);
_12 = Field::<f32>(Variant(_4, 0), 1) + Field::<f32>(Variant(_8, 0), 1);
RET = -Field::<f32>(Variant(_5, 0), 1);
place!(Field::<f32>(Variant(_7, 0), 1)) = Field::<f32>(Variant(_8, 0), 1);
_8 = _5;
_1 = Adt44::Variant0 { fld0: _11,fld1: _3 };
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).2 = -37_i8;
_5 = _4;
_8 = Adt44::Variant0 { fld0: Field::<[u16; 6]>(Variant(_7, 0), 0),fld1: Field::<f32>(Variant(_4, 0), 1) };
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)) = (12044_u16, Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).2, 80_i8, (-985018431890681246_i64), 51224_u16);
Goto(bb11)
}
bb11 = {
Call(_18 = dump_var(9_usize, 13_usize, Move(_13), 11_usize, Move(_11), 19_usize, _19, 19_usize, _19), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: Adt44,mut _2: Adt44,mut _3: Adt44,mut _4: f32,mut _5: Adt44,mut _6: Adt44,mut _7: Adt44,mut _8: Adt44,mut _9: [u16; 6],mut _10: Adt44,mut _11: Adt44) -> f32 {
mir! {
type RET = f32;
let _12: *mut f32;
let _13: (i64, u128);
let _14: ();
let _15: ();
{
_6 = _7;
place!(Field::<[u16; 6]>(Variant(_2, 0), 0)) = Field::<[u16; 6]>(Variant(_6, 0), 0);
place!(Field::<[u16; 6]>(Variant(_1, 0), 0)) = [43948_u16,7999_u16,32527_u16,42359_u16,40040_u16,35164_u16];
RET = Field::<f32>(Variant(_10, 0), 1) - Field::<f32>(Variant(_8, 0), 1);
place!(Field::<[u16; 6]>(Variant(_10, 0), 0)) = [6414_u16,57658_u16,63355_u16,55833_u16,49869_u16,7043_u16];
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [22391_u16,65021_u16,57133_u16,32138_u16,17005_u16,59886_u16];
RET = Field::<f32>(Variant(_10, 0), 1) * Field::<f32>(Variant(_7, 0), 1);
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: Adt44,mut _2: [u16; 6],mut _3: Adt44,mut _4: Adt44,mut _5: Adt56,mut _6: Adt44,mut _7: Adt44,mut _8: Adt44,mut _9: [isize; 7],mut _10: Adt44,mut _11: [u16; 6],mut _12: Adt44) -> Adt49 {
mir! {
type RET = Adt49;
let _13: &'static i32;
let _14: i8;
let _15: Adt55;
let _16: [char; 1];
let _17: Adt48;
let _18: isize;
let _19: (u16, f64, i8, i64, u16);
let _20: f32;
let _21: i32;
let _22: Adt56;
let _23: i128;
let _24: [char; 1];
let _25: *mut f32;
let _26: *mut f32;
let _27: i128;
let _28: (bool, f32, f64, isize);
let _29: [usize; 6];
let _30: (u16, f64, i8, i64, u16);
let _31: char;
let _32: u32;
let _33: i8;
let _34: (i64, u128);
let _35: [u16; 6];
let _36: f32;
let _37: *mut char;
let _38: [i64; 6];
let _39: *mut char;
let _40: [u32; 4];
let _41: [u16; 6];
let _42: i64;
let _43: u32;
let _44: Adt45;
let _45: isize;
let _46: Adt56;
let _47: Adt46;
let _48: [u16; 1];
let _49: [bool; 7];
let _50: [u16; 6];
let _51: Adt52;
let _52: Adt55;
let _53: (u16, f64, i8, i64, u16);
let _54: ();
let _55: ();
{
RET.fld2 = !51444_u16;
place!(Field::<f32>(Variant(_10, 0), 1)) = -Field::<f32>(Variant(_7, 0), 1);
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_6 = _8;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7)).1.0.3 = (-86_isize) ^ (-98_isize);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4)).3 = (-514520148128723474_i64) & (-9001319752063729615_i64);
place!(Field::<[u16; 6]>(Variant(_1, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
place!(Field::<char>(Variant(_7, 2), 1)) = '\u{7a431}';
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4)).0 = (-16174_i16) as u16;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7)).1.2 = Field::<f32>(Variant(_8, 0), 1) as u16;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7)).1.1 = core::ptr::addr_of_mut!(place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7)).1.0.1);
Goto(bb1)
}
bb1 = {
_5.fld1 = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3];
RET.fld7 = [Field::<char>(Variant(_7, 2), 1)];
RET.fld0.2 = 299794580507440839726701466421443764145_u128 | 55069094456809954351511660978121708421_u128;
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,RET.fld2,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4).0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2];
RET.fld0.1 = true & true;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7)).1.0.1 = Field::<f32>(Variant(_6, 0), 1);
_11 = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4).0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,RET.fld2,RET.fld2];
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4)).0 = Field::<char>(Variant(_7, 2), 1) as u16;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_7, 2), 3)).0.3 = !Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3;
RET.fld1 = 15739015168088229915_u64 << Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3;
_14 = (-6456_i16) as i8;
place!(Field::<char>(Variant(_7, 2), 1)) = '\u{321b8}';
RET.fld0.1 = RET.fld1 > RET.fld1;
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = Field::<[u16; 6]>(Variant(_4, 0), 0);
place!(Field::<f32>(Variant(_10, 0), 1)) = Field::<f32>(Variant(_8, 0), 1);
place!(Field::<f32>(Variant(_1, 0), 1)) = Field::<f32>(Variant(_4, 0), 1);
_7 = _1;
RET.fld0 = (6034555346069582922_usize, true, 309593435543241312747534328829881251614_u128);
match RET.fld0.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
309593435543241312747534328829881251614 => bb9,
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
place!(Field::<[u16; 6]>(Variant(_10, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
place!(Field::<f32>(Variant(_6, 0), 1)) = Field::<f32>(Variant(_1, 0), 1) + Field::<f32>(Variant(_7, 0), 1);
_5.fld0 = [RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1];
RET.fld0.0 = !1192598586962899918_usize;
place!(Field::<f32>(Variant(_10, 0), 1)) = Field::<f32>(Variant(_12, 0), 1);
RET.fld3 = _14;
_10 = _3;
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
place!(Field::<[u16; 6]>(Variant(_7, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
place!(Field::<[u16; 6]>(Variant(_8, 0), 0)) = _2;
_5.fld0 = [RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1];
_9 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),30_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
Goto(bb10)
}
bb10 = {
_13 = &_21;
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_2 = Field::<[u16; 6]>(Variant(_8, 0), 0);
_18 = RET.fld0.1 as isize;
_16 = RET.fld7;
place!(Field::<f32>(Variant(_4, 0), 1)) = RET.fld0.0 as f32;
_19.3 = (-1594845424635816818_i64);
RET.fld4 = 4558_i16 << _19.3;
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
place!(Field::<f32>(Variant(_4, 0), 1)) = Field::<f32>(Variant(_1, 0), 1) * Field::<f32>(Variant(_8, 0), 1);
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = _2;
RET.fld5 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_19.1 = _14 as f64;
RET.fld7 = _16;
place!(Field::<f32>(Variant(_6, 0), 1)) = Field::<f32>(Variant(_12, 0), 1);
RET.fld2 = Field::<f32>(Variant(_12, 0), 1) as u16;
_10 = _8;
RET.fld4 = 30678_i16 & (-17118_i16);
match RET.fld0.2 {
0 => bb7,
1 => bb2,
2 => bb3,
309593435543241312747534328829881251614 => bb11,
_ => bb4
}
}
bb11 = {
_19.2 = _14 * RET.fld3;
_22 = Adt56 { fld0: _5.fld0,fld1: _9 };
_20 = -Field::<f32>(Variant(_12, 0), 1);
RET.fld3 = _19.2 & _14;
_23 = (-129350015992264455057364400382708482884_i128) * 137145015401740785590634701317261183382_i128;
_5 = _22;
_5.fld0 = [RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1];
RET.fld1 = RET.fld0.2 as u64;
place!(Field::<[u16; 6]>(Variant(_1, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_23 = (-86850473271225076665872747688422422281_i128);
place!(Field::<[u16; 6]>(Variant(_6, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
RET.fld1 = _23 as u64;
_2 = Field::<[u16; 6]>(Variant(_8, 0), 0);
match RET.fld0.2 {
0 => bb9,
309593435543241312747534328829881251614 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_1 = _8;
place!(Field::<f32>(Variant(_7, 0), 1)) = _19.2 as f32;
SetDiscriminant(_8, 1);
Goto(bb14)
}
bb14 = {
_28.1 = _20 - Field::<f32>(Variant(_3, 0), 1);
_29 = [RET.fld0.0,RET.fld0.0,RET.fld0.0,RET.fld0.0,RET.fld0.0,RET.fld0.0];
place!(Field::<(bool, f32, f64, isize)>(Variant(_8, 1), 4)).2 = _19.1 - _19.1;
RET.fld7 = _16;
RET.fld0.0 = !6973579020827327543_usize;
_28.1 = Field::<f32>(Variant(_10, 0), 1);
_16 = ['\u{8b350}'];
_16 = RET.fld7;
_21 = 1979821423_i32;
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
place!(Field::<f32>(Variant(_3, 0), 1)) = RET.fld3 as f32;
Goto(bb15)
}
bb15 = {
SetDiscriminant(_6, 1);
_8 = Adt44::Variant0 { fld0: _2,fld1: Field::<f32>(Variant(_3, 0), 1) };
place!(Field::<[u16; 6]>(Variant(_7, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_30.3 = _19.3 * _19.3;
RET.fld0.1 = !false;
_3 = _1;
_26 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_1, 0), 1)));
_8 = _10;
_28.3 = '\u{fab3a}' as isize;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).4 = !RET.fld2;
_28.2 = -_19.1;
_22 = _5;
_32 = 1532739679_u32;
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).0 = _14 == _19.2;
place!(Field::<f32>(Variant(_4, 0), 1)) = _18 as f32;
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)) = (RET.fld0.1, _28.1, _28.2, _18);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).0 = Field::<f32>(Variant(_4, 0), 1) as u16;
_20 = Field::<f32>(Variant(_12, 0), 1) - Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).1;
place!(Field::<f32>(Variant(_7, 0), 1)) = -Field::<f32>(Variant(_10, 0), 1);
match RET.fld0.2 {
0 => bb16,
309593435543241312747534328829881251614 => bb18,
_ => bb17
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
RET.fld3 = _19.2;
RET.fld0.0 = 4_usize ^ 5_usize;
_30.3 = !_19.3;
place!(Field::<[u32; 4]>(Variant(_6, 1), 0)) = [_32,_32,_32,_32];
RET.fld0 = (5557644557378790946_usize, Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).0, 114185242540723492969686718242071097625_u128);
_11 = Field::<[u16; 6]>(Variant(_3, 0), 0);
place!(Field::<f32>(Variant(_1, 0), 1)) = -Field::<f32>(Variant(_4, 0), 1);
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)).1 = Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).0 | RET.fld0.1;
_30.2 = _19.2;
_22 = Adt56 { fld0: _5.fld0,fld1: _5.fld1 };
RET.fld0.1 = Field::<(usize, bool, u128)>(Variant(_6, 1), 7).1;
match RET.fld0.2 {
0 => bb1,
1 => bb14,
2 => bb12,
114185242540723492969686718242071097625 => bb20,
_ => bb19
}
}
bb19 = {
SetDiscriminant(_6, 1);
_8 = Adt44::Variant0 { fld0: _2,fld1: Field::<f32>(Variant(_3, 0), 1) };
place!(Field::<[u16; 6]>(Variant(_7, 0), 0)) = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_30.3 = _19.3 * _19.3;
RET.fld0.1 = !false;
_3 = _1;
_26 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_1, 0), 1)));
_8 = _10;
_28.3 = '\u{fab3a}' as isize;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).4 = !RET.fld2;
_28.2 = -_19.1;
_22 = _5;
_32 = 1532739679_u32;
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).0 = _14 == _19.2;
place!(Field::<f32>(Variant(_4, 0), 1)) = _18 as f32;
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)) = (RET.fld0.1, _28.1, _28.2, _18);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).0 = Field::<f32>(Variant(_4, 0), 1) as u16;
_20 = Field::<f32>(Variant(_12, 0), 1) - Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).1;
place!(Field::<f32>(Variant(_7, 0), 1)) = -Field::<f32>(Variant(_10, 0), 1);
match RET.fld0.2 {
0 => bb16,
309593435543241312747534328829881251614 => bb18,
_ => bb17
}
}
bb20 = {
_7 = _10;
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).2 = RET.fld1 as f64;
Goto(bb21)
}
bb21 = {
_30 = (Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0, _19.1, RET.fld3, _19.3, Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4);
_24 = ['\u{2ccb}'];
_31 = '\u{c8bdd}';
_3 = _10;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)) = (_30.0, _19.1, _14, _30.3, RET.fld2);
match RET.fld0.2 {
0 => bb9,
1 => bb17,
2 => bb19,
3 => bb4,
114185242540723492969686718242071097625 => bb22,
_ => bb16
}
}
bb22 = {
_38 = [_19.3,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).3,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).3,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).3,_19.3,_30.3];
_41 = [RET.fld2,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,_30.0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,_30.0];
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8, 2), 4)) = (Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4, _19.1, _19.2, Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).3, RET.fld2);
_19.3 = !_30.3;
_28.0 = Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).0;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8, 2), 4)) = (Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4, _28.2, RET.fld3, Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).3, _30.4);
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7)).1 = (Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4), _26, Field::<(u16, f64, i8, i64, u16)>(Variant(_8, 2), 4).0);
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).1.2 = _30.4;
place!(Field::<(i64, u128)>(Variant(_10, 2), 2)).1 = !RET.fld0.2;
_40 = Field::<[u32; 4]>(Variant(_6, 1), 0);
_13 = &_21;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).1 = (Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4), _26, _30.0);
Goto(bb23)
}
bb23 = {
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_10, 2), 4)).1 = Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).3 as f64;
match RET.fld0.2 {
0 => bb18,
1 => bb2,
2 => bb3,
3 => bb11,
114185242540723492969686718242071097625 => bb24,
_ => bb19
}
}
bb24 = {
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).0.2 = Field::<(u16, f64, i8, i64, u16)>(Variant(_8, 2), 4).1;
_5.fld1 = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.3,_18,Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).3];
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).0.0 = Field::<(usize, bool, u128)>(Variant(_6, 1), 7).1 & Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.0;
_40 = [_32,_32,_32,_32];
_42 = _30.3;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).2 = !RET.fld0.0;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)).0.3 = -Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).3;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)).1 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.1;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).0.0 = Field::<(usize, bool, u128)>(Variant(_6, 1), 7).1 | _28.0;
_45 = -Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.3;
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).2 = _28.2 * Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.2;
_9 = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.3,Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3).0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.3,Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3).0.3,Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3).0.3];
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).1.0.0 = _30.0 >= _30.0;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_10, 2), 4)).0 = _21 as u16;
place!(Field::<(i64, u128)>(Variant(_4, 2), 2)).1 = !Field::<(i64, u128)>(Variant(_10, 2), 2).1;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_8, 2), 4)) = (RET.fld2, _19.1, RET.fld3, _30.3, _30.0);
place!(Field::<i8>(Variant(_6, 1), 3)) = RET.fld0.0 as i8;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3)) = (Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0, Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.1, Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.2);
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7)).1.1 = core::ptr::addr_of_mut!(place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7)).1.0.1);
Goto(bb25)
}
bb25 = {
place!(Field::<char>(Variant(_8, 2), 1)) = _31;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)).0.1 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).2 as f32;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7)).1.0 = _28;
_37 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_4, 2), 1)));
place!(Field::<i8>(Variant(_6, 1), 3)) = _19.2;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7)).1.0 = (Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.0, (*_26), Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).2, _45);
place!(Field::<char>(Variant(_4, 2), 1)) = _31;
_20 = RET.fld0.0 as f32;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7)).1.0.1 = -(*_26);
RET.fld0.2 = Field::<(i64, u128)>(Variant(_4, 2), 2).1 + Field::<(i64, u128)>(Variant(_10, 2), 2).1;
_11 = [RET.fld2,RET.fld2,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).4,Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3).2];
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).3 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.3 * Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.3;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7)).0 = core::ptr::addr_of!(RET.fld4);
place!(Field::<*mut u32>(Variant(_10, 2), 6)) = core::ptr::addr_of_mut!(_32);
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7)).1.0 = Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4);
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3)).0.1 = Field::<f32>(Variant(_3, 0), 1);
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_4, 2), 4)).2 = !Field::<i8>(Variant(_6, 1), 3);
_34.1 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.1 as u128;
_46.fld0 = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.0,Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3).0.0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.0];
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).1 = _26;
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)) = (Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).2, Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7).1.0.0, RET.fld0.2);
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3)).0.1 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_8, 2), 7).1.0.1 - Field::<f32>(Variant(_1, 0), 1);
RET.fld0.1 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3).0.0;
_19.1 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7).1.0.2;
_14 = _19.2 ^ _30.2;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7)).1 = (Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7).1.0, Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3).1, Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0);
_34.0 = _42 + Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).3;
Goto(bb26)
}
bb26 = {
place!(Field::<char>(Variant(_10, 2), 1)) = (*_37);
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7)).0 = core::ptr::addr_of!(RET.fld4);
RET.fld2 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3).0.1 as u16;
place!(Field::<f64>(Variant(_7, 2), 5)) = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3).0.2 - Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_8, 2), 3).0.2;
place!(Field::<(i64, u128)>(Variant(_7, 2), 2)).0 = _30.3;
_8 = Adt44::Variant1 { fld0: Field::<[u32; 4]>(Variant(_6, 1), 0),fld1: (*_37),fld2: _37,fld3: Field::<(u16, f64, i8, i64, u16)>(Variant(_4, 2), 4).2,fld4: Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0,fld5: Field::<[u16; 6]>(Variant(_1, 0), 0),fld6: Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6),fld7: RET.fld0 };
_33 = (*_26) as i8;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7)).1.2 = 182_u8 as u16;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7)).1.0 = (Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.0, (*_26), Field::<f64>(Variant(_7, 2), 5), Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3).0.3);
RET.fld5 = [RET.fld2,Field::<(u16, f64, i8, i64, u16)>(Variant(_8, 1), 6).0,RET.fld2,Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.2,Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3).2];
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7)).2 = 242_u8 as usize;
_7 = Adt44::Variant1 { fld0: _40,fld1: (*_37),fld2: _37,fld3: Field::<(u16, f64, i8, i64, u16)>(Variant(_4, 2), 4).2,fld4: Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0,fld5: _2,fld6: Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6),fld7: RET.fld0 };
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6)).0 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.2;
match RET.fld0.0 {
5557644557378790946 => bb28,
_ => bb27
}
}
bb27 = {
_5.fld1 = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3];
RET.fld7 = [Field::<char>(Variant(_7, 2), 1)];
RET.fld0.2 = 299794580507440839726701466421443764145_u128 | 55069094456809954351511660978121708421_u128;
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,RET.fld2,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4).0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2];
RET.fld0.1 = true & true;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7)).1.0.1 = Field::<f32>(Variant(_6, 0), 1);
_11 = [Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4).0,Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4).0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.2,RET.fld2,RET.fld2];
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_7, 2), 4)).0 = Field::<char>(Variant(_7, 2), 1) as u16;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_7, 2), 3)).0.3 = !Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3;
RET.fld1 = 15739015168088229915_u64 << Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_7, 2), 7).1.0.3;
_14 = (-6456_i16) as i8;
place!(Field::<char>(Variant(_7, 2), 1)) = '\u{321b8}';
RET.fld0.1 = RET.fld1 > RET.fld1;
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = Field::<[u16; 6]>(Variant(_4, 0), 0);
place!(Field::<f32>(Variant(_10, 0), 1)) = Field::<f32>(Variant(_8, 0), 1);
place!(Field::<f32>(Variant(_1, 0), 1)) = Field::<f32>(Variant(_4, 0), 1);
_7 = _1;
RET.fld0 = (6034555346069582922_usize, true, 309593435543241312747534328829881251614_u128);
match RET.fld0.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
309593435543241312747534328829881251614 => bb9,
_ => bb8
}
}
bb28 = {
place!(Field::<(usize, bool, u128)>(Variant(_6, 1), 7)).1 = _28.0;
_19.0 = Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).3 as u16;
Goto(bb29)
}
bb29 = {
place!(Field::<*mut char>(Variant(_8, 1), 2)) = _37;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)).1 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3).1;
_34 = (_42, Field::<(usize, bool, u128)>(Variant(_8, 1), 7).2);
_49 = _22.fld0;
_46.fld0 = [Field::<(bool, f32, f64, isize)>(Variant(_7, 1), 4).0,Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7).1.0.0,RET.fld0.1,Field::<(usize, bool, u128)>(Variant(_7, 1), 7).1,Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).0,Field::<(usize, bool, u128)>(Variant(_7, 1), 7).1,RET.fld0.1];
_46.fld1 = [Field::<(bool, f32, f64, isize)>(Variant(_8, 1), 4).3,Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).3,Field::<(bool, f32, f64, isize)>(Variant(_8, 1), 4).3,Field::<(bool, f32, f64, isize)>(Variant(_7, 1), 4).3,_18,Field::<(bool, f32, f64, isize)>(Variant(_7, 1), 4).3,Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).3];
place!(Field::<(bool, f32, f64, isize)>(Variant(_8, 1), 4)).2 = Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4).2 * Field::<(bool, f32, f64, isize)>(Variant(_7, 1), 4).2;
Goto(bb30)
}
bb30 = {
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7)).1 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3);
_19.1 = -Field::<(u16, f64, i8, i64, u16)>(Variant(_8, 1), 6).1;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_4, 2), 4)) = (Field::<(u16, f64, i8, i64, u16)>(Variant(_10, 2), 4).0, Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_4, 2), 7).1.0.2, Field::<(u16, f64, i8, i64, u16)>(Variant(_6, 1), 6).2, _19.3, Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.2);
RET.fld6 = Adt45::Variant0 { fld0: Field::<*mut u32>(Variant(_10, 2), 6) };
RET.fld0.2 = !Field::<(usize, bool, u128)>(Variant(_8, 1), 7).2;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)).0 = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3).0;
place!(Field::<f32>(Variant(_1, 0), 1)) = Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_4, 2), 3).0.2 as f32;
place!(Field::<(bool, f32, f64, isize)>(Variant(_6, 1), 4)).1 = -Field::<f32>(Variant(_1, 0), 1);
RET.fld0 = (Field::<(usize, bool, u128)>(Variant(_8, 1), 7).0, Field::<(usize, bool, u128)>(Variant(_6, 1), 7).1, Field::<(usize, bool, u128)>(Variant(_7, 1), 7).2);
_6 = Adt44::Variant0 { fld0: Field::<[u16; 6]>(Variant(_3, 0), 0),fld1: _28.1 };
RET.fld3 = _19.2;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_10, 2), 4)).3 = Field::<(u16, f64, i8, i64, u16)>(Variant(_4, 2), 4).3;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7)).2 = Field::<(usize, bool, u128)>(Variant(_7, 1), 7).0;
_44 = RET.fld6;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)).0.2 = -Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.0.2;
place!(Field::<f32>(Variant(_6, 0), 1)) = RET.fld1 as f32;
_53 = _30;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_4, 2), 4)).4 = (*_37) as u16;
place!(Field::<(u16, f64, i8, i64, u16)>(Variant(_4, 2), 4)).2 = Field::<i8>(Variant(_8, 1), 3) & _30.2;
place!(Field::<bool>(Variant(_10, 2), 0)) = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).2 > RET.fld0.0;
place!(Field::<[u16; 6]>(Variant(_3, 0), 0)) = Field::<[u16; 6]>(Variant(_8, 1), 5);
place!(Field::<(i64, u128)>(Variant(_4, 2), 2)).0 = Field::<(u16, f64, i8, i64, u16)>(Variant(_10, 2), 4).3;
place!(Field::<((bool, f32, f64, isize), *mut f32, u16)>(Variant(_10, 2), 3)) = (Field::<(bool, f32, f64, isize)>(Variant(_7, 1), 4), Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.1, Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_10, 2), 7).1.2);
_35 = Field::<[u16; 6]>(Variant(_3, 0), 0);
place!(Field::<char>(Variant(_10, 2), 1)) = Field::<char>(Variant(_7, 1), 1);
Goto(bb31)
}
bb31 = {
Call(_54 = dump_var(11_usize, 33_usize, Move(_33), 38_usize, Move(_38), 23_usize, Move(_23), 45_usize, Move(_45)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_54 = dump_var(11_usize, 34_usize, Move(_34), 24_usize, Move(_24), 21_usize, Move(_21), 32_usize, Move(_32)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_54 = dump_var(11_usize, 29_usize, Move(_29), 9_usize, Move(_9), 55_usize, _55, 55_usize, _55), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i64,mut _2: (u16, f64, i8, i64, u16),mut _3: (u16, f64, i8, i64, u16)) -> (char, *const (i64, u128), char, *mut f32) {
mir! {
type RET = (char, *const (i64, u128), char, *mut f32);
let _4: u64;
let _5: (bool, f32, f64, isize);
let _6: Adt54;
let _7: isize;
let _8: isize;
let _9: Adt47;
let _10: ();
let _11: ();
{
_2.3 = 13409054286848490062188163446216176788_u128 as i64;
RET.2 = '\u{e13a8}';
_3.1 = _2.1;
_2.2 = _3.2 | _3.2;
_1 = _3.3;
RET.3 = core::ptr::addr_of_mut!(_5.1);
RET.3 = core::ptr::addr_of_mut!(_5.1);
RET.0 = RET.2;
RET.0 = RET.2;
_5.1 = _3.1 as f32;
_3.0 = _2.0 * _2.4;
_3.1 = -_2.1;
_3.4 = _2.4 >> _3.3;
RET.2 = RET.0;
Call(RET.1 = fn13(_3.3, _1, _3, _3.4, _3, _3.3, _3.3, _3.3, _3.4, _1, _3.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.1 = _2.1;
RET.0 = RET.2;
RET.3 = core::ptr::addr_of_mut!(_5.1);
_7 = (-79_isize);
_2 = (_3.4, _3.1, _3.2, _3.3, _3.4);
_5.0 = true;
_5.1 = _3.0 as f32;
_4 = 11245199209679383922_u64 << _2.0;
_5.3 = _7;
_2.1 = _3.1 + _3.1;
_2.2 = -_3.2;
_3.1 = _2.1;
_5.3 = -_7;
_2 = _3;
_2.4 = (-126548290461162978429857757735206875221_i128) as u16;
_1 = !_2.3;
RET.0 = RET.2;
_7 = _5.3;
_3.1 = -_2.1;
Goto(bb2)
}
bb2 = {
Call(_10 = dump_var(12_usize, 1_usize, Move(_1), 11_usize, _11, 11_usize, _11, 11_usize, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i64,mut _2: i64,mut _3: (u16, f64, i8, i64, u16),mut _4: u16,mut _5: (u16, f64, i8, i64, u16),mut _6: i64,mut _7: i64,mut _8: i64,mut _9: u16,mut _10: i64,mut _11: u16) -> *const (i64, u128) {
mir! {
type RET = *const (i64, u128);
let _12: f32;
let _13: i16;
let _14: [usize; 6];
let _15: isize;
let _16: [i64; 6];
let _17: f64;
let _18: i64;
let _19: i128;
let _20: *const (i64, u128);
let _21: [u32; 4];
let _22: Adt50;
let _23: Adt56;
let _24: bool;
let _25: f32;
let _26: [bool; 7];
let _27: bool;
let _28: Adt47;
let _29: bool;
let _30: f32;
let _31: (usize, bool, u128);
let _32: f32;
let _33: u128;
let _34: Adt54;
let _35: f32;
let _36: [u16; 1];
let _37: f32;
let _38: char;
let _39: (u16, f64, i8, i64, u16);
let _40: [bool; 7];
let _41: Adt43;
let _42: i8;
let _43: i64;
let _44: *const (i64, u128);
let _45: Adt47;
let _46: i8;
let _47: (u16, f64, i8, i64, u16);
let _48: f64;
let _49: Adt54;
let _50: Adt44;
let _51: &'static i32;
let _52: bool;
let _53: ();
let _54: ();
{
_3.2 = _5.2;
_5.0 = 2602972333_u32 as u16;
_3.2 = _5.2;
_3.0 = _11;
_8 = _9 as i64;
_4 = _3.0;
_6 = (-9223372036854775808_isize) as i64;
_5.2 = _3.2;
_4 = !_3.0;
_3.2 = _5.2 & _5.2;
_12 = (-6599_i16) as f32;
_9 = _3.4;
_12 = 66891204202555688065562775470690595048_u128 as f32;
match _5.3 {
1291596494693392984 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_13 = (-5764_i16);
_3 = (_9, _5.1, _5.2, _10, _5.4);
_12 = _5.2 as f32;
_10 = _1 << _7;
_5.4 = _3.0;
_5.2 = _3.2;
_2 = _3.2 as i64;
_6 = _3.1 as i64;
_5.1 = (-29_isize) as f64;
_5.3 = _3.0 as i64;
_5.4 = (-45224147391199000221665941129010509141_i128) as u16;
_3.2 = _5.2 >> _3.4;
_5.1 = _3.1;
_12 = 71062219829186526803545359451002885846_i128 as f32;
_5.2 = -_3.2;
_6 = 3111021183_u32 as i64;
Goto(bb3)
}
bb3 = {
_14 = [6_usize,4118364802410531418_usize,939174792480788040_usize,3_usize,0_usize,4033472283925076104_usize];
_3.3 = -_1;
_3.3 = 3041937691_u32 as i64;
_3.1 = _5.1;
_10 = 3345034820611746825_u64 as i64;
_5.0 = !_11;
_18 = '\u{be716}' as i64;
_3.4 = _4;
_3.0 = !_11;
_2 = 2906559861_u32 as i64;
_5.1 = -_3.1;
_3 = _5;
_17 = _3.1;
_10 = (-48835545434278955462647029410790016171_i128) as i64;
_17 = 930827420_i32 as f64;
_5.2 = _3.2 * _3.2;
_5.4 = (-9223372036854775808_isize) as u16;
_3 = (_5.0, _5.1, _5.2, _5.3, _9);
_5.1 = _17 + _3.1;
_5.3 = _1 - _1;
_9 = _4 << _1;
_4 = '\u{45c2a}' as u16;
_15 = -9223372036854775807_isize;
_4 = !_3.0;
_16 = [_5.3,_7,_5.3,_3.3,_5.3,_5.3];
_12 = 638919362_u32 as f32;
match _7 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
1291596494693392984 => bb10,
_ => bb9
}
}
bb4 = {
_13 = (-5764_i16);
_3 = (_9, _5.1, _5.2, _10, _5.4);
_12 = _5.2 as f32;
_10 = _1 << _7;
_5.4 = _3.0;
_5.2 = _3.2;
_2 = _3.2 as i64;
_6 = _3.1 as i64;
_5.1 = (-29_isize) as f64;
_5.3 = _3.0 as i64;
_5.4 = (-45224147391199000221665941129010509141_i128) as u16;
_3.2 = _5.2 >> _3.4;
_5.1 = _3.1;
_12 = 71062219829186526803545359451002885846_i128 as f32;
_5.2 = -_3.2;
_6 = 3111021183_u32 as i64;
Goto(bb3)
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
_15 = (-52_isize) & (-10_isize);
_5.1 = _17;
_19 = -(-112766162389053448228814295843288603542_i128);
_15 = 9223372036854775807_isize << _5.0;
_3.2 = _5.2 - _5.2;
_12 = _19 as f32;
_17 = 145_u8 as f64;
_12 = 321484517220619102747747233537273420103_u128 as f32;
_11 = _3.4 ^ _9;
_3.4 = _11 * _11;
_5.0 = !_3.4;
_6 = !_5.3;
_21 = [3133978090_u32,3887405675_u32,838929678_u32,736008995_u32];
_12 = _5.3 as f32;
_5.0 = _11 + _11;
match _7 {
0 => bb8,
1 => bb11,
2 => bb12,
1291596494693392984 => bb14,
_ => bb13
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
_11 = _3.0;
_23.fld1 = [_15,_15,_15,_15,_15,_15,_15];
_12 = 15577842544802900332_usize as f32;
_25 = -_12;
_6 = _5.3 * _1;
_6 = '\u{73f59}' as i64;
_15 = false as isize;
_3 = _5;
_5 = (_9, _3.1, _3.2, _7, _3.0);
_23.fld1 = [_15,_15,_15,_15,_15,_15,_15];
_24 = _8 >= _1;
_8 = _1;
_6 = -_3.3;
_23.fld0 = [_24,_24,_24,_24,_24,_24,_24];
match _1 {
0 => bb5,
1 => bb8,
2 => bb6,
3 => bb4,
4 => bb15,
5 => bb16,
6 => bb17,
1291596494693392984 => bb19,
_ => bb18
}
}
bb15 = {
Return()
}
bb16 = {
_13 = (-5764_i16);
_3 = (_9, _5.1, _5.2, _10, _5.4);
_12 = _5.2 as f32;
_10 = _1 << _7;
_5.4 = _3.0;
_5.2 = _3.2;
_2 = _3.2 as i64;
_6 = _3.1 as i64;
_5.1 = (-29_isize) as f64;
_5.3 = _3.0 as i64;
_5.4 = (-45224147391199000221665941129010509141_i128) as u16;
_3.2 = _5.2 >> _3.4;
_5.1 = _3.1;
_12 = 71062219829186526803545359451002885846_i128 as f32;
_5.2 = -_3.2;
_6 = 3111021183_u32 as i64;
Goto(bb3)
}
bb17 = {
Return()
}
bb18 = {
_15 = (-52_isize) & (-10_isize);
_5.1 = _17;
_19 = -(-112766162389053448228814295843288603542_i128);
_15 = 9223372036854775807_isize << _5.0;
_3.2 = _5.2 - _5.2;
_12 = _19 as f32;
_17 = 145_u8 as f64;
_12 = 321484517220619102747747233537273420103_u128 as f32;
_11 = _3.4 ^ _9;
_3.4 = _11 * _11;
_5.0 = !_3.4;
_6 = !_5.3;
_21 = [3133978090_u32,3887405675_u32,838929678_u32,736008995_u32];
_12 = _5.3 as f32;
_5.0 = _11 + _11;
match _7 {
0 => bb8,
1 => bb11,
2 => bb12,
1291596494693392984 => bb14,
_ => bb13
}
}
bb19 = {
_1 = _6 - _8;
_5.2 = _3.0 as i8;
_27 = !_24;
_5 = (_4, _17, _3.2, _3.3, _9);
_17 = _3.1;
_3.0 = !_4;
_3 = (_5.4, _5.1, _5.2, _1, _4);
_14 = [6797869209572865478_usize,13191995795682342833_usize,11301860730224492188_usize,1_usize,13952222412155024198_usize,411405651733067179_usize];
_26 = [_27,_27,_24,_27,_24,_24,_27];
_7 = -_8;
_19 = (-37871786163250450056562353559655465002_i128) >> _5.4;
_29 = _24 & _24;
Goto(bb20)
}
bb20 = {
_25 = -_12;
_23.fld1 = [_15,_15,_15,_15,_15,_15,_15];
_11 = !_9;
_31.0 = !0_usize;
_1 = _6 << _9;
_31 = (1_usize, _27, 281611799328461360783554815736159740200_u128);
_23.fld0 = [_27,_31.1,_24,_29,_24,_31.1,_29];
Goto(bb21)
}
bb21 = {
_1 = _13 as i64;
_9 = _31.2 as u16;
_31 = (517556480880731014_usize, _29, 295190305825581327760566861152064417552_u128);
_31.1 = _7 != _5.3;
_33 = !_31.2;
_5.1 = _25 as f64;
_26 = [_24,_24,_31.1,_29,_31.1,_29,_27];
_15 = -(-9223372036854775808_isize);
_24 = !_27;
_10 = !_5.3;
_13 = 25658_i16;
_3.3 = _25 as i64;
_5.3 = _7;
_26 = [_29,_27,_29,_27,_27,_24,_27];
_13 = _19 as i16;
_5.2 = _3.2;
_12 = -_25;
_26 = [_27,_31.1,_27,_27,_24,_27,_29];
Goto(bb22)
}
bb22 = {
_8 = _6 + _7;
_11 = _5.0 & _3.4;
_5.4 = !_11;
_15 = 9223372036854775807_isize;
_31.0 = 9878311841452687979_usize;
_5 = _3;
_1 = _8 - _8;
_14 = [_31.0,_31.0,_31.0,_31.0,_31.0,_31.0];
_5.4 = _3.0;
_19 = 51419323056289099633127886569166408881_i128 ^ 132970895758951375310256553321531763037_i128;
_5.3 = !_1;
_11 = _33 as u16;
_18 = _15 as i64;
_8 = _10;
_31 = (1_usize, _27, _33);
_19 = (-100197060112781901039155045225067423302_i128) - (-89074934858969307793899981778845275204_i128);
_11 = !_4;
_6 = _10;
Call(_8 = fn14(_3.2, _23.fld0), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_8 = _31.0 as i64;
_3.0 = _9 + _4;
_3.1 = 651860947_u32 as f64;
_26 = [_29,_29,_27,_24,_24,_27,_24];
_35 = _12 + _25;
_31.0 = 1775711159002230912_usize | 1_usize;
_30 = -_35;
_3.3 = _8 | _6;
_5.4 = !_5.0;
_31.1 = _24;
_5.2 = -_3.2;
_21 = [2403945367_u32,695472397_u32,1410146466_u32,3190400001_u32];
_29 = _3.2 >= _5.2;
_12 = _35 - _35;
_27 = _29 & _31.1;
_5 = (_3.0, _17, _3.2, _8, _9);
Call(_11 = fn15(_5.3, _31, _19, _26, _9, _31, _9), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_3 = (_5.0, _5.1, _5.2, _5.3, _5.4);
_14 = [_31.0,_31.0,_31.0,_31.0,_31.0,_31.0];
_5.2 = _3.2;
_3.3 = _5.3 ^ _8;
_11 = !_5.4;
_3 = _5;
_10 = -_1;
_5 = (_4, _17, _3.2, _1, _3.4);
_12 = _15 as f32;
_27 = _5.3 < _3.3;
_5.3 = _19 as i64;
_11 = _31.0 as u16;
_3.2 = _5.4 as i8;
_38 = '\u{53e0}';
_33 = _31.2;
_25 = _12;
_32 = _31.0 as f32;
_3.0 = _27 as u16;
_11 = !_5.4;
_5 = (_3.4, _17, _3.2, _1, _9);
_39.1 = _5.4 as f64;
_40 = [_29,_24,_24,_24,_31.1,_29,_29];
_39.0 = _5.4;
_4 = _3.0;
Goto(bb25)
}
bb25 = {
_39.3 = _5.3;
_36 = [_11];
_5.3 = 252_u8 as i64;
_6 = _8;
_5.1 = _19 as f64;
_21 = [1227468162_u32,1980179084_u32,1339485303_u32,1534665158_u32];
_5.4 = _3.4;
_30 = _32 + _32;
_25 = _30 + _32;
_3.1 = 2123320867_i32 as f64;
_1 = _8;
_16 = [_3.3,_3.3,_3.3,_3.3,_8,_1];
_1 = _8;
_12 = _25;
_5 = (_39.0, _39.1, _3.2, _8, _39.0);
_12 = -_35;
_5.2 = _3.2;
_41 = Adt43::Variant1 { fld0: _13 };
_5.0 = _38 as u16;
Goto(bb26)
}
bb26 = {
_47 = _5;
_5.1 = -_47.1;
_37 = -_35;
_39.4 = _5.3 as u16;
_16 = [_6,_8,_8,_3.3,_10,_8];
_14 = [_31.0,_31.0,_31.0,_31.0,_31.0,_31.0];
_16 = [_5.3,_3.3,_5.3,_8,_10,_6];
_42 = _5.2;
_21 = [904780200_u32,318437633_u32,3777858681_u32,537087589_u32];
_26 = [_24,_24,_24,_24,_29,_24,_24];
SetDiscriminant(_41, 2);
_39.4 = _4 + _47.4;
place!(Field::<*const i16>(Variant(_41, 2), 3)) = core::ptr::addr_of!(_13);
_1 = !_8;
_23.fld1 = [_15,_15,_15,_15,_15,_15,_15];
_39.2 = 3749894757756773819_u64 as i8;
Goto(bb27)
}
bb27 = {
place!(Field::<[u32; 4]>(Variant(_41, 2), 5)) = _21;
_39.4 = 29_u8 as u16;
_40 = [_29,_27,_29,_29,_29,_24,_27];
_47.4 = _3.4;
_12 = -_25;
place!(Field::<*const (i64, u128)>(Variant(_41, 2), 7)) = core::ptr::addr_of!(place!(Field::<(i64, u128)>(Variant(_41, 2), 2)));
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_41, 2), 1)).2 = _24 as usize;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_41, 2), 1)).1.2 = _47.4;
_27 = _31.1;
place!(Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_41, 2), 1)).1.0.0 = _24;
_47.2 = _3.2;
_8 = _3.3 + _3.3;
_4 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_41, 2), 1).1.2;
_6 = 136_u8 as i64;
_44 = core::ptr::addr_of!(place!(Field::<(i64, u128)>(Variant(_41, 2), 2)));
_43 = _3.3;
_25 = -_12;
_9 = Field::<(*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize)>(Variant(_41, 2), 1).1.2 << _3.3;
place!(Field::<f64>(Variant(_41, 2), 0)) = _35 as f64;
_24 = !_29;
_3.1 = -_39.1;
_13 = -(-12207_i16);
_21 = [611603372_u32,1394282357_u32,824491396_u32,1426647824_u32];
RET = _44;
_31.2 = _38 as u128;
place!(Field::<(i64, u128)>(Variant(_41, 2), 2)).1 = _33 - _31.2;
_9 = _5.4 << _39.3;
_31.1 = _43 > _1;
Goto(bb28)
}
bb28 = {
Call(_53 = dump_var(13_usize, 31_usize, Move(_31), 8_usize, Move(_8), 26_usize, Move(_26), 21_usize, Move(_21)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_53 = dump_var(13_usize, 19_usize, Move(_19), 36_usize, Move(_36), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_53 = dump_var(13_usize, 10_usize, Move(_10), 14_usize, Move(_14), 33_usize, Move(_33), 27_usize, Move(_27)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_53 = dump_var(13_usize, 43_usize, Move(_43), 54_usize, _54, 54_usize, _54, 54_usize, _54), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i8,mut _2: [bool; 7]) -> i64 {
mir! {
type RET = i64;
let _3: Adt47;
let _4: (i64, u128);
let _5: (bool, f32, f64, isize);
let _6: bool;
let _7: bool;
let _8: [u32; 4];
let _9: (usize, bool, u128);
let _10: (u16, u16, i16);
let _11: u8;
let _12: bool;
let _13: f64;
let _14: Adt48;
let _15: Adt59;
let _16: f64;
let _17: Adt44;
let _18: Adt44;
let _19: (bool, f32, f64, isize);
let _20: [char; 1];
let _21: u128;
let _22: Adt46;
let _23: u8;
let _24: [u16; 6];
let _25: i64;
let _26: u64;
let _27: ();
let _28: ();
{
RET = -4776542858458424312_i64;
_2 = [false,true,false,true,true,false,false];
_1 = -(-72_i8);
_2 = [false,false,false,false,true,true,false];
_2 = [false,true,true,false,false,true,true];
_1 = 1_i8 & 90_i8;
_1 = 4_i8 * 18_i8;
RET = (-5804475293548824075_i64);
_2 = [false,false,true,true,true,true,true];
_2 = [true,false,false,true,true,true,false];
RET = -(-1899637240290888473_i64);
RET = 281035365902436702141028367041925745784_u128 as i64;
RET = (-8748798594322766010_i64);
RET = (-6866147023548571641_i64);
_2 = [false,false,false,false,true,false,true];
RET = 3551220218_u32 as i64;
_4.1 = !283745899577099752102343379481176513867_u128;
_4.0 = -RET;
_2 = [true,true,true,false,true,true,false];
_4 = (RET, 293938589680150027063604916425279403283_u128);
_4.0 = -RET;
_1 = 104_i8;
_2 = [false,false,true,true,false,false,false];
_5.3 = 147178058391537573123238784229320181878_i128 as isize;
_5.2 = _5.3 as f64;
_5.1 = _4.1 as f32;
_4 = (RET, 136485585942964080392512992245389134057_u128);
match _4.1 {
0 => bb1,
1 => bb2,
136485585942964080392512992245389134057 => bb4,
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
_5.3 = (-9223372036854775808_isize) >> _4.0;
_5.0 = true & true;
_5.1 = 5409147178122181898_usize as f32;
_1 = _5.3 as i8;
RET = _5.0 as i64;
_1 = (-56_i8) - (-101_i8);
RET = 77_u8 as i64;
_5.0 = false;
_4 = (RET, 322340394052214536780637717623756240712_u128);
_5.1 = 17293_u16 as f32;
_4.0 = RET;
_4.0 = '\u{48122}' as i64;
_5.1 = 0_usize as f32;
_4 = (RET, 177512128372110333298128645121988960240_u128);
_4.1 = 201116893883526728401075755387419277702_u128 - 263945923944445425403746370109250023757_u128;
Goto(bb5)
}
bb5 = {
RET = -_4.0;
_7 = _5.0 & _5.0;
_6 = _1 == _1;
_5.2 = (-1944962701_i32) as f64;
_5.1 = _4.1 as f32;
_4 = (RET, 76087802958061766787657414043390588783_u128);
_9.0 = 0_usize;
_5.0 = _6 >= _7;
_5.2 = 186_u8 as f64;
_8 = [1762663224_u32,1901368129_u32,2373675398_u32,3081402381_u32];
RET = _4.0;
RET = -_4.0;
RET = 539890950_i32 as i64;
_9.1 = _5.1 <= _5.1;
_1 = 52_i8;
_4.1 = 80176683129718940623967879033306138163_u128;
_4 = (RET, 302873468569106963769242092871532911299_u128);
_7 = _5.0 & _9.1;
_9.1 = !_7;
_9 = (16171867987415091082_usize, _7, _4.1);
_9.2 = !_4.1;
_4 = (RET, _9.2);
_9 = (9852144834446726858_usize, _6, _4.1);
_5.3 = 41486_u16 as isize;
_1 = (-70871314398660024409857490375185002986_i128) as i8;
_5.2 = _5.3 as f64;
_1 = 71_i8;
Goto(bb6)
}
bb6 = {
_2 = [_7,_5.0,_6,_5.0,_7,_9.1,_7];
_7 = _5.0 == _9.1;
_5.3 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_5.3 = 9223372036854775807_isize;
_11 = !20_u8;
_9.1 = _7;
_10 = (4199_u16, 18326_u16, (-19670_i16));
_5.0 = _7;
RET = _9.0 as i64;
_9.0 = 4_usize;
_4.0 = _10.2 as i64;
_10.0 = _5.0 as u16;
_4.0 = RET + RET;
_10.0 = _10.1 % _10.1;
_9.1 = !_7;
_2 = [_9.1,_5.0,_5.0,_7,_7,_5.0,_6];
_5.1 = _4.0 as f32;
_4 = (RET, _9.2);
_9 = (7735207562155075477_usize, _5.0, _4.1);
RET = !_4.0;
_7 = _5.0;
_5.1 = 1300764098_i32 as f32;
_4.0 = !RET;
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb7,
5 => bb8,
7735207562155075477 => bb10,
_ => bb9
}
}
bb7 = {
Return()
}
bb8 = {
_5.3 = (-9223372036854775808_isize) >> _4.0;
_5.0 = true & true;
_5.1 = 5409147178122181898_usize as f32;
_1 = _5.3 as i8;
RET = _5.0 as i64;
_1 = (-56_i8) - (-101_i8);
RET = 77_u8 as i64;
_5.0 = false;
_4 = (RET, 322340394052214536780637717623756240712_u128);
_5.1 = 17293_u16 as f32;
_4.0 = RET;
_4.0 = '\u{48122}' as i64;
_5.1 = 0_usize as f32;
_4 = (RET, 177512128372110333298128645121988960240_u128);
_4.1 = 201116893883526728401075755387419277702_u128 - 263945923944445425403746370109250023757_u128;
Goto(bb5)
}
bb9 = {
Return()
}
bb10 = {
_10 = (32423_u16, 51647_u16, 26636_i16);
_9.1 = _7 & _7;
_12 = _9.1;
_6 = _9.1;
_12 = _5.0;
_13 = _5.2;
match _5.3 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb9,
5 => bb6,
6 => bb7,
9223372036854775807 => bb12,
_ => bb11
}
}
bb11 = {
_2 = [_7,_5.0,_6,_5.0,_7,_9.1,_7];
_7 = _5.0 == _9.1;
_5.3 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_5.3 = 9223372036854775807_isize;
_11 = !20_u8;
_9.1 = _7;
_10 = (4199_u16, 18326_u16, (-19670_i16));
_5.0 = _7;
RET = _9.0 as i64;
_9.0 = 4_usize;
_4.0 = _10.2 as i64;
_10.0 = _5.0 as u16;
_4.0 = RET + RET;
_10.0 = _10.1 % _10.1;
_9.1 = !_7;
_2 = [_9.1,_5.0,_5.0,_7,_7,_5.0,_6];
_5.1 = _4.0 as f32;
_4 = (RET, _9.2);
_9 = (7735207562155075477_usize, _5.0, _4.1);
RET = !_4.0;
_7 = _5.0;
_5.1 = 1300764098_i32 as f32;
_4.0 = !RET;
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb7,
5 => bb8,
7735207562155075477 => bb10,
_ => bb9
}
}
bb12 = {
_5.0 = _9.1 & _12;
_6 = !_12;
_10.0 = _10.1;
_5.2 = _11 as f64;
_19.1 = _5.1 - _5.1;
_19.3 = -_5.3;
_9.1 = RET >= _4.0;
_2 = [_5.0,_7,_5.0,_12,_12,_5.0,_5.0];
RET = (-111259364_i32) as i64;
_19.0 = _5.0;
_10 = (36532_u16, 33970_u16, 1289_i16);
_22 = Adt46::Variant0 { fld0: _19.1 };
_9.0 = 5_usize ^ 2197324318292347939_usize;
_5.2 = _13;
_9 = (6_usize, _6, _4.1);
RET = _4.0 >> _9.2;
RET = '\u{f1cd7}' as i64;
SetDiscriminant(_22, 1);
_23 = _11 - _11;
_22 = Adt46::Variant0 { fld0: _5.1 };
_24 = [_10.0,_10.0,_10.1,_10.1,_10.0,_10.0];
_6 = !_5.0;
_19.0 = _6 != _6;
_10.1 = !_10.0;
_7 = _19.0 == _19.0;
Call(_19.3 = core::intrinsics::bswap(_5.3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_16 = _13;
Goto(bb14)
}
bb14 = {
_19.3 = -_5.3;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(14_usize, 23_usize, Move(_23), 2_usize, Move(_2), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(14_usize, 7_usize, Move(_7), 10_usize, Move(_10), 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i64,mut _2: (usize, bool, u128),mut _3: i128,mut _4: [bool; 7],mut _5: u16,mut _6: (usize, bool, u128),mut _7: u16) -> u16 {
mir! {
type RET = u16;
let _8: bool;
let _9: bool;
let _10: i128;
let _11: Adt52;
let _12: Adt53;
let _13: ();
let _14: ();
{
_2.2 = _1 as u128;
_4 = [_2.1,_6.1,_6.1,_6.1,_2.1,_6.1,_2.1];
_6.2 = !_2.2;
_2.1 = _6.1 ^ _6.1;
_6 = _2;
_2.1 = _6.1;
_2.1 = !_6.1;
_6.2 = 4962952249533824346_u64 as u128;
_3 = !(-146791961888068827044736547786946253963_i128);
RET = _5 << _1;
_2.0 = 1805028186_i32 as usize;
_6.1 = !_2.1;
_2.0 = RET as usize;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(15_usize, 4_usize, Move(_4), 1_usize, Move(_1), 7_usize, Move(_7), 14_usize, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: u64,mut _3: u8,mut _4: i64,mut _5: [u16; 6],mut _6: bool) -> char {
mir! {
type RET = char;
let _7: bool;
let _8: [usize; 6];
let _9: f32;
let _10: [isize; 7];
let _11: f64;
let _12: isize;
let _13: ((bool, f32, f64, isize), *mut f32, u16);
let _14: f32;
let _15: Adt47;
let _16: (u16, f64, i8, i64, u16);
let _17: [char; 1];
let _18: ([isize; 7], *mut char, &'static i32);
let _19: [i64; 6];
let _20: Adt44;
let _21: Adt52;
let _22: (bool, f32, f64, isize);
let _23: [bool; 7];
let _24: isize;
let _25: [u32; 4];
let _26: Adt57;
let _27: i32;
let _28: Adt55;
let _29: [isize; 6];
let _30: [u16; 6];
let _31: i8;
let _32: bool;
let _33: [u16; 1];
let _34: (u16, f64, i8, i64, u16);
let _35: (bool, f32, f64, isize);
let _36: isize;
let _37: bool;
let _38: char;
let _39: i16;
let _40: ();
let _41: ();
{
_3 = 160533652150364738831452762301608799601_i128 as u8;
_2 = 2834075892610556149_u64 ^ 10798856128200983935_u64;
RET = '\u{b8c88}';
_2 = 2432443512788128922_u64 - 18351046484989032559_u64;
Goto(bb1)
}
bb1 = {
_1 = _6;
_1 = _6 != _6;
_2 = 845999801653148222_u64;
RET = '\u{e2ba1}';
RET = '\u{412dd}';
RET = '\u{d446b}';
_6 = _1 <= _1;
_1 = _6 > _6;
RET = '\u{a4b96}';
_1 = _6;
RET = '\u{cd1e8}';
_2 = !7339838909459650136_u64;
_6 = !_1;
Goto(bb2)
}
bb2 = {
_9 = 1848_i16 as f32;
_5 = [34649_u16,33724_u16,55184_u16,10485_u16,64069_u16,13684_u16];
_5 = [2674_u16,52616_u16,6879_u16,20080_u16,26720_u16,47938_u16];
_9 = _3 as f32;
RET = '\u{d2d95}';
_9 = (-91_i8) as f32;
_2 = 3155091709658571705_u64;
RET = '\u{a45ad}';
_2 = !9140324429667045966_u64;
_8 = [6_usize,2_usize,9764438334785546832_usize,4_usize,17105799290666764577_usize,5047649447544682692_usize];
_7 = _1;
_9 = (-1551835327_i32) as f32;
_3 = _1 as u8;
_3 = 102_u8;
_9 = 957070341_u32 as f32;
_10 = [9223372036854775807_isize,76_isize,45_isize,9223372036854775807_isize,38_isize,9223372036854775807_isize,9223372036854775807_isize];
_11 = 2_usize as f64;
_6 = _1 < _7;
_9 = (-74_i8) as f32;
_13.0.1 = _9 * _9;
_13.0.3 = _2 as isize;
_13.0.3 = 9223372036854775807_isize;
_13.1 = core::ptr::addr_of_mut!(_9);
_13.1 = core::ptr::addr_of_mut!(_9);
match _3 {
0 => bb1,
102 => bb4,
_ => bb3
}
}
bb3 = {
_1 = _6;
_1 = _6 != _6;
_2 = 845999801653148222_u64;
RET = '\u{e2ba1}';
RET = '\u{412dd}';
RET = '\u{d446b}';
_6 = _1 <= _1;
_1 = _6 > _6;
RET = '\u{a4b96}';
_1 = _6;
RET = '\u{cd1e8}';
_2 = !7339838909459650136_u64;
_6 = !_1;
Goto(bb2)
}
bb4 = {
_12 = !_13.0.3;
_14 = _13.0.1 + _13.0.1;
_13.0.1 = -_14;
_16.2 = (-63_i8) | 50_i8;
_13.0.0 = _7;
_7 = !_6;
RET = '\u{85018}';
_7 = _13.0.0;
RET = '\u{3a0a3}';
_4 = (-8311967817264218191_i64) - (-4167615641930786779_i64);
_13.0.0 = !_6;
_13.2 = 51247_u16 * 63877_u16;
_16 = (_13.2, _11, (-60_i8), _4, _13.2);
_14 = _13.0.1;
_14 = _13.0.1 - _13.0.1;
_18.0 = [_13.0.3,_12,_12,_12,_12,_12,_13.0.3];
_18.1 = core::ptr::addr_of_mut!(RET);
_3 = !68_u8;
_18.0 = [_12,_12,_13.0.3,_13.0.3,_13.0.3,_12,_13.0.3];
_13.1 = core::ptr::addr_of_mut!(_13.0.1);
_16.4 = !_16.0;
_19 = [_16.3,_16.3,_16.3,_16.3,_16.3,_4];
Goto(bb5)
}
bb5 = {
_3 = 202_u8;
_7 = _1 >= _1;
_18.0 = [_12,_12,_13.0.3,_12,_12,_12,_13.0.3];
_13.0.0 = _7 >= _1;
_18.1 = core::ptr::addr_of_mut!(RET);
_7 = _6;
_16.0 = 67365243588193206622195311396938245954_u128 as u16;
_16.2 = 98_i8 | 30_i8;
_13.0.2 = _11 * _16.1;
_16.1 = _13.0.2;
_18.1 = core::ptr::addr_of_mut!(RET);
_13.2 = _16.0;
_16 = (_13.2, _13.0.2, (-74_i8), _4, _13.2);
_22.1 = -_14;
_16.2 = (-17_i8) ^ (-17_i8);
_16.3 = _4;
_13.1 = core::ptr::addr_of_mut!(_9);
_22 = (_6, _14, _13.0.2, _12);
_16.0 = _13.2;
_7 = !_6;
RET = '\u{10991}';
_16.1 = (-24896_i16) as f64;
_17 = [RET];
_16.0 = _13.2 * _13.2;
_16.2 = 69_i8;
_16.0 = _16.4;
Goto(bb6)
}
bb6 = {
_13.0 = (_6, _22.1, _22.2, _12);
_16 = (_13.2, _22.2, (-8_i8), _4, _13.2);
_13.0.0 = !_7;
_13.1 = core::ptr::addr_of_mut!(_14);
_6 = _7;
_13.0 = (_7, _14, _11, _12);
_22.1 = _13.0.1 + _13.0.1;
_4 = _16.3 >> _16.2;
_22 = _13.0;
_13.0.3 = _12 >> _2;
_16.2 = 12598296500513608628_usize as i8;
_5 = [_16.4,_16.0,_13.2,_16.0,_16.0,_13.2];
_2 = !7370288830673650230_u64;
_10 = [_22.3,_12,_13.0.3,_12,_12,_13.0.3,_13.0.3];
_13.0.2 = _11 - _11;
_13.0.3 = -_22.3;
_8 = [4_usize,5868452008883505998_usize,3136449340185007581_usize,7829852897035662930_usize,17029073692319213887_usize,1_usize];
_1 = !_7;
_14 = _9 - _22.1;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
202 => bb7,
_ => bb5
}
}
bb7 = {
_15 = Adt47::Variant0 { fld0: _22.3,fld1: _17 };
_16.0 = _16.4 - _16.4;
_3 = 209_u8;
_23 = [_13.0.0,_7,_13.0.0,_7,_6,_22.0,_1];
_18.2 = &_27;
_13.2 = !_16.0;
_7 = _6 <= _6;
_16.0 = !_13.2;
_7 = _13.0.0 < _1;
_5 = [_13.2,_13.2,_16.4,_16.0,_13.2,_16.4];
place!(Field::<[char; 1]>(Variant(_15, 0), 1)) = [RET];
_24 = _22.3;
_22.3 = _13.0.3;
RET = '\u{4d53}';
_16.1 = _13.0.2 - _22.2;
_14 = _13.0.1 - _22.1;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
209 => bb9,
_ => bb8
}
}
bb8 = {
_12 = !_13.0.3;
_14 = _13.0.1 + _13.0.1;
_13.0.1 = -_14;
_16.2 = (-63_i8) | 50_i8;
_13.0.0 = _7;
_7 = !_6;
RET = '\u{85018}';
_7 = _13.0.0;
RET = '\u{3a0a3}';
_4 = (-8311967817264218191_i64) - (-4167615641930786779_i64);
_13.0.0 = !_6;
_13.2 = 51247_u16 * 63877_u16;
_16 = (_13.2, _11, (-60_i8), _4, _13.2);
_14 = _13.0.1;
_14 = _13.0.1 - _13.0.1;
_18.0 = [_13.0.3,_12,_12,_12,_12,_12,_13.0.3];
_18.1 = core::ptr::addr_of_mut!(RET);
_3 = !68_u8;
_18.0 = [_12,_12,_13.0.3,_13.0.3,_13.0.3,_12,_13.0.3];
_13.1 = core::ptr::addr_of_mut!(_13.0.1);
_16.4 = !_16.0;
_19 = [_16.3,_16.3,_16.3,_16.3,_16.3,_4];
Goto(bb5)
}
bb9 = {
_13.0 = (_22.0, _14, _11, _24);
RET = '\u{16c4b}';
_16 = (_13.2, _11, 82_i8, _4, _13.2);
_13.0.2 = -_22.2;
_17 = Field::<[char; 1]>(Variant(_15, 0), 1);
_15 = Adt47::Variant0 { fld0: _24,fld1: _17 };
_13.1 = core::ptr::addr_of_mut!(_13.0.1);
_14 = -_13.0.1;
_6 = _22.0;
_6 = _1;
_17 = [RET];
_17 = [RET];
_16.3 = _4 << _16.2;
_13.0 = (_6, _22.1, _16.1, _12);
_24 = _13.0.3;
_12 = !_22.3;
_9 = -_13.0.1;
_31 = -_16.2;
_1 = _7;
_13.1 = core::ptr::addr_of_mut!(_22.1);
_4 = _16.3;
match _16.2 {
0 => bb5,
1 => bb10,
2 => bb11,
82 => bb13,
_ => bb12
}
}
bb10 = {
_3 = 202_u8;
_7 = _1 >= _1;
_18.0 = [_12,_12,_13.0.3,_12,_12,_12,_13.0.3];
_13.0.0 = _7 >= _1;
_18.1 = core::ptr::addr_of_mut!(RET);
_7 = _6;
_16.0 = 67365243588193206622195311396938245954_u128 as u16;
_16.2 = 98_i8 | 30_i8;
_13.0.2 = _11 * _16.1;
_16.1 = _13.0.2;
_18.1 = core::ptr::addr_of_mut!(RET);
_13.2 = _16.0;
_16 = (_13.2, _13.0.2, (-74_i8), _4, _13.2);
_22.1 = -_14;
_16.2 = (-17_i8) ^ (-17_i8);
_16.3 = _4;
_13.1 = core::ptr::addr_of_mut!(_9);
_22 = (_6, _14, _13.0.2, _12);
_16.0 = _13.2;
_7 = !_6;
RET = '\u{10991}';
_16.1 = (-24896_i16) as f64;
_17 = [RET];
_16.0 = _13.2 * _13.2;
_16.2 = 69_i8;
_16.0 = _16.4;
Goto(bb6)
}
bb11 = {
_1 = _6;
_1 = _6 != _6;
_2 = 845999801653148222_u64;
RET = '\u{e2ba1}';
RET = '\u{412dd}';
RET = '\u{d446b}';
_6 = _1 <= _1;
_1 = _6 > _6;
RET = '\u{a4b96}';
_1 = _6;
RET = '\u{cd1e8}';
_2 = !7339838909459650136_u64;
_6 = !_1;
Goto(bb2)
}
bb12 = {
_13.0 = (_6, _22.1, _22.2, _12);
_16 = (_13.2, _22.2, (-8_i8), _4, _13.2);
_13.0.0 = !_7;
_13.1 = core::ptr::addr_of_mut!(_14);
_6 = _7;
_13.0 = (_7, _14, _11, _12);
_22.1 = _13.0.1 + _13.0.1;
_4 = _16.3 >> _16.2;
_22 = _13.0;
_13.0.3 = _12 >> _2;
_16.2 = 12598296500513608628_usize as i8;
_5 = [_16.4,_16.0,_13.2,_16.0,_16.0,_13.2];
_2 = !7370288830673650230_u64;
_10 = [_22.3,_12,_13.0.3,_12,_12,_13.0.3,_13.0.3];
_13.0.2 = _11 - _11;
_13.0.3 = -_22.3;
_8 = [4_usize,5868452008883505998_usize,3136449340185007581_usize,7829852897035662930_usize,17029073692319213887_usize,1_usize];
_1 = !_7;
_14 = _9 - _22.1;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
202 => bb7,
_ => bb5
}
}
bb13 = {
_16.0 = _1 as u16;
_16.2 = -_31;
_18.0 = _10;
_1 = _22.0;
_34 = (_13.2, _22.2, _16.2, _16.3, _16.0);
Goto(bb14)
}
bb14 = {
_22 = (_7, _9, _11, Field::<isize>(Variant(_15, 0), 0));
_16.2 = _34.2 ^ _31;
_13.1 = core::ptr::addr_of_mut!(_22.1);
_35 = _13.0;
_22.2 = _35.2 + _13.0.2;
_5 = [_16.0,_34.4,_34.4,_16.0,_16.0,_16.0];
_27 = (-941886093_i32) - (-1579805001_i32);
_16.2 = _34.2 ^ _34.2;
place!(Field::<[char; 1]>(Variant(_15, 0), 1)) = [RET];
_39 = 12143_i16 & 22691_i16;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(16_usize, 8_usize, Move(_8), 19_usize, Move(_19), 24_usize, Move(_24), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(16_usize, 23_usize, Move(_23), 31_usize, Move(_31), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: (usize, bool, u128),mut _2: usize,mut _3: usize,mut _4: usize,mut _5: bool,mut _6: [u32; 4],mut _7: [u32; 4],mut _8: u16,mut _9: i16,mut _10: usize,mut _11: i64,mut _12: usize,mut _13: *mut f32,mut _14: [u16; 6]) -> char {
mir! {
type RET = char;
let _15: i16;
let _16: char;
let _17: (u16, f64, i8, i64, u16);
let _18: Adt58;
let _19: [isize; 7];
let _20: char;
let _21: isize;
let _22: f32;
let _23: i16;
let _24: i128;
let _25: [char; 1];
let _26: Adt44;
let _27: bool;
let _28: u8;
let _29: isize;
let _30: isize;
let _31: [u16; 6];
let _32: i64;
let _33: [u16; 6];
let _34: (bool, f32, f64, isize);
let _35: isize;
let _36: ();
let _37: ();
{
_9 = 26968_i16;
_14 = [_8,_8,_8,_8,_8,_8];
Call(_8 = core::intrinsics::bswap(27975_u16), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _2 ^ _12;
_14 = [_8,_8,_8,_8,_8,_8];
_6 = [1656306008_u32,2073550250_u32,467740843_u32,2085549333_u32];
_1 = (_3, _5, 77929495482851753917749959052494486574_u128);
match _1.2 {
0 => bb2,
1 => bb3,
77929495482851753917749959052494486574 => bb5,
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
RET = '\u{afa78}';
_1.2 = !214148204880357921531631502559336501651_u128;
_5 = _1.1 & _1.1;
_1.0 = _2;
_6 = _7;
_15 = 1211782404_i32 as i16;
_1.0 = _10 >> _15;
_8 = 65438_u16;
_11 = 3156640960545299558_i64;
Goto(bb6)
}
bb6 = {
_9 = _15;
_1.0 = _3;
_12 = _3;
_16 = RET;
_7 = [3627380703_u32,966688593_u32,783932673_u32,600067668_u32];
_15 = _9 - _9;
_14 = [_8,_8,_8,_8,_8,_8];
_3 = !_2;
_12 = !_3;
_1.2 = 247934151957652810246773396612207231015_u128;
_17.2 = !123_i8;
_1.2 = !11344180342303846904658884721204883646_u128;
Goto(bb7)
}
bb7 = {
_14 = [_8,_8,_8,_8,_8,_8];
_7 = [1054252118_u32,855210633_u32,1086320621_u32,3154869806_u32];
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb8,
6 => bb9,
65438 => bb11,
_ => bb10
}
}
bb8 = {
Return()
}
bb9 = {
RET = '\u{afa78}';
_1.2 = !214148204880357921531631502559336501651_u128;
_5 = _1.1 & _1.1;
_1.0 = _2;
_6 = _7;
_15 = 1211782404_i32 as i16;
_1.0 = _10 >> _15;
_8 = 65438_u16;
_11 = 3156640960545299558_i64;
Goto(bb6)
}
bb10 = {
Return()
}
bb11 = {
_17.1 = 10119610787353788041488520427445862585_i128 as f64;
_3 = _4 & _12;
_17.4 = _8 >> _4;
_6 = [3127725425_u32,966898206_u32,401555093_u32,4096882774_u32];
_2 = 87_isize as usize;
_9 = 2861852703_u32 as i16;
RET = _16;
_1.1 = _17.2 != _17.2;
_1 = (_10, _5, 306765486428383039356214524565825644403_u128);
_8 = _4 as u16;
_15 = !_9;
_15 = 13759568845178812391_u64 as i16;
_21 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_3 = _4 ^ _10;
_23 = _21 as i16;
_5 = !_1.1;
_12 = !_3;
_7 = _6;
_15 = _1.2 as i16;
match _1.2 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
306765486428383039356214524565825644403 => bb12,
_ => bb7
}
}
bb12 = {
_21 = (-9223372036854775808_isize) >> _4;
_14 = [_17.4,_8,_17.4,_8,_8,_17.4];
_20 = _16;
_13 = core::ptr::addr_of_mut!(_22);
RET = _20;
_17.0 = _8;
_24 = _15 as i128;
_19 = [_21,_21,_21,_21,_21,_21,_21];
_20 = RET;
_9 = !_15;
_27 = !_1.1;
_17.0 = _8;
_8 = _17.0 ^ _17.4;
_14 = [_17.4,_17.0,_17.0,_8,_17.4,_8];
_25 = [_16];
_13 = core::ptr::addr_of_mut!((*_13));
RET = _20;
_6 = [1365966167_u32,4205717363_u32,2106856710_u32,2724499549_u32];
_22 = _15 as f32;
_3 = _4 >> _15;
_19 = [_21,_21,_21,_21,_21,_21,_21];
_31 = [_17.4,_17.0,_17.4,_17.4,_17.0,_17.0];
match _1.2 {
0 => bb4,
1 => bb2,
2 => bb13,
3 => bb14,
306765486428383039356214524565825644403 => bb16,
_ => bb15
}
}
bb13 = {
_9 = _15;
_1.0 = _3;
_12 = _3;
_16 = RET;
_7 = [3627380703_u32,966688593_u32,783932673_u32,600067668_u32];
_15 = _9 - _9;
_14 = [_8,_8,_8,_8,_8,_8];
_3 = !_2;
_12 = !_3;
_1.2 = 247934151957652810246773396612207231015_u128;
_17.2 = !123_i8;
_1.2 = !11344180342303846904658884721204883646_u128;
Goto(bb7)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_24 = (-10051662776702000511944697184334434872_i128);
_32 = _11 + _11;
_3 = !_12;
_30 = 696356780_i32 as isize;
Goto(bb17)
}
bb17 = {
Call(_36 = dump_var(17_usize, 14_usize, Move(_14), 5_usize, Move(_5), 2_usize, Move(_2), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(17_usize, 1_usize, Move(_1), 7_usize, Move(_7), 25_usize, Move(_25), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(17_usize, 21_usize, Move(_21), 6_usize, Move(_6), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(566943787196697617_i64), std::hint::black_box('\u{1b923}'), std::hint::black_box((-9223372036854775808_isize)));
                
            }
#[derive(Debug)]
pub enum Adt43 {
Variant0{
fld0: *const (i64, u128),
fld1: (usize, bool, u128),
fld2: ((bool, f32, f64, isize), *mut f32, u16),
fld3: *const usize,
fld4: *mut f32,
fld5: (i64, u128),

},
Variant1{
fld0: i16,

},
Variant2{
fld0: f64,
fld1: (*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize),
fld2: (i64, u128),
fld3: *const i16,
fld4: *mut char,
fld5: [u32; 4],
fld6: f32,
fld7: *const (i64, u128),

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt44 {
Variant0{
fld0: [u16; 6],
fld1: f32,

},
Variant1{
fld0: [u32; 4],
fld1: char,
fld2: *mut char,
fld3: i8,
fld4: (bool, f32, f64, isize),
fld5: [u16; 6],
fld6: (u16, f64, i8, i64, u16),
fld7: (usize, bool, u128),

},
Variant2{
fld0: bool,
fld1: char,
fld2: (i64, u128),
fld3: ((bool, f32, f64, isize), *mut f32, u16),
fld4: (u16, f64, i8, i64, u16),
fld5: f64,
fld6: *mut u32,
fld7: (*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize),

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt45 {
Variant0{
fld0: *mut u32,

},
Variant1{
fld0: *const usize,
fld1: ((bool, f32, f64, isize), *mut f32, u16),
fld2: (*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize),
fld3: usize,
fld4: f32,

},
Variant2{
fld0: (u16, f64, i8, i64, u16),
fld1: f64,
fld2: (u16, u16, i16),

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt46 {
Variant0{
fld0: f32,

},
Variant1{
fld0: [char; 1],

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt47 {
Variant0{
fld0: isize,
fld1: [char; 1],

},
Variant1{
fld0: [i64; 6],
fld1: (usize, bool, u128),
fld2: isize,
fld3: (i64, u128),
fld4: [char; 1],
fld5: [isize; 6],
fld6: i128,

}}
#[derive(Debug)]
pub enum Adt48 {
Variant0{
fld0: Adt46,
fld1: [usize; 6],

},
Variant1{
fld0: u16,
fld1: *mut f32,
fld2: isize,
fld3: (u16, u16, i16),
fld4: u64,
fld5: f64,

},
Variant2{
fld0: *const usize,
fld1: char,
fld2: *mut u32,
fld3: u64,
fld4: i16,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt49 {
fld0: (usize, bool, u128),
fld1: u64,
fld2: u16,
fld3: i8,
fld4: i16,
fld5: [u16; 6],
fld6: Adt45,
fld7: [char; 1],
}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: (u16, u16, i16),
fld1: char,
fld2: ((bool, f32, f64, isize), *mut f32, u16),
fld3: u64,

},
Variant1{
fld0: [u32; 4],
fld1: (*const i16, ((bool, f32, f64, isize), *mut f32, u16), usize),
fld2: u16,
fld3: Adt44,
fld4: [u16; 6],

},
Variant2{
fld0: i16,
fld1: u8,
fld2: Adt44,
fld3: [bool; 7],

},
Variant3{
fld0: [u16; 1],

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: bool,
fld1: char,
fld2: [i64; 6],
fld3: Adt48,
fld4: i64,

},
Variant1{
fld0: *mut u32,
fld1: (u16, f64, i8, i64, u16),
fld2: [bool; 7],
fld3: *const (i64, u128),
fld4: i16,
fld5: Adt43,
fld6: usize,
fld7: *const i16,

},
Variant2{
fld0: (i64, u128),
fld1: [isize; 7],
fld2: Adt44,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt52 {
Variant0{
fld0: bool,
fld1: u128,
fld2: [u16; 6],
fld3: (i64, u128),
fld4: i128,
fld5: Adt47,

},
Variant1{
fld0: [u32; 4],
fld1: *const (i64, u128),
fld2: [isize; 7],
fld3: [i64; 6],

}}
#[derive(Debug)]
pub struct Adt53 {
fld0: *mut f32,
fld1: Adt50,
fld2: isize,
fld3: (char, *const (i64, u128), char, *mut f32),
fld4: i16,
fld5: u8,
fld6: Adt49,
}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: [char; 1],
fld1: char,
fld2: u8,
fld3: (u16, f64, i8, i64, u16),
fld4: Adt44,
fld5: *const usize,
fld6: [u16; 6],
fld7: [isize; 7],

},
Variant1{
fld0: [u16; 6],
fld1: ((bool, f32, f64, isize), *mut f32, u16),

},
Variant2{
fld0: Adt45,
fld1: usize,
fld2: Adt50,
fld3: u64,

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: (i64, u128),

},
Variant1{
fld0: Adt52,
fld1: *const (i64, u128),

},
Variant2{
fld0: *mut u32,
fld1: (u16, f64, i8, i64, u16),
fld2: u64,
fld3: Adt45,
fld4: *const (i64, u128),

},
Variant3{
fld0: u16,
fld1: (u16, u16, i16),
fld2: f32,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt56 {
fld0: [bool; 7],
fld1: [isize; 7],
}
#[derive(Debug)]
pub enum Adt57 {
Variant0{
fld0: *const (i64, u128),
fld1: Adt46,
fld2: Adt55,
fld3: Adt51,
fld4: Adt44,
fld5: *mut f32,
fld6: Adt45,
fld7: u64,

},
Variant1{
fld0: [u32; 4],
fld1: i64,
fld2: [i64; 6],
fld3: Adt46,
fld4: (bool, f32, f64, isize),
fld5: Adt44,

}}
#[derive(Debug)]
pub enum Adt58 {
Variant0{
fld0: Adt55,
fld1: i32,
fld2: Adt51,
fld3: Adt45,
fld4: ((bool, f32, f64, isize), *mut f32, u16),

},
Variant1{
fld0: [usize; 6],

},
Variant2{
fld0: Adt55,
fld1: ((bool, f32, f64, isize), *mut f32, u16),

},
Variant3{
fld0: f32,
fld1: Adt46,
fld2: Adt54,
fld3: Adt50,
fld4: u64,
fld5: [bool; 7],

}}
#[derive(Debug)]
pub enum Adt59 {
Variant0{
fld0: Adt48,
fld1: Adt46,

},
Variant1{
fld0: u128,
fld1: f64,
fld2: [usize; 6],
fld3: Adt48,

}}

