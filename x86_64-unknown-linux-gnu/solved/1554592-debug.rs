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
pub fn fn0(mut _1: usize,mut _2: char,mut _3: isize,mut _4: i16) -> [i128; 1] {
mir! {
type RET = [i128; 1];
let _5: (u64, i64);
let _6: Adt53;
let _7: [u8; 3];
let _8: f32;
let _9: i16;
let _10: [i16; 2];
let _11: Adt48;
let _12: i64;
let _13: u16;
let _14: u32;
let _15: [u8; 3];
let _16: Adt52;
let _17: Adt43;
let _18: char;
let _19: (u16, *mut *mut i64);
let _20: u64;
let _21: (&'static usize, [i128; 1]);
let _22: u32;
let _23: (u64, i64);
let _24: Adt53;
let _25: char;
let _26: i64;
let _27: f32;
let _28: (i32, [bool; 8], u8);
let _29: ();
let _30: ();
{
RET = [42042848311906486546152022498382258995_i128];
_3 = 9223372036854775807_isize | 9223372036854775807_isize;
_2 = '\u{a7bbf}';
_4 = 13508_i16 & (-30863_i16);
_2 = '\u{55cfe}';
_4 = 17250_i16;
_5 = (330599864900207487_u64, (-2723127909982382305_i64));
_5.0 = 540819490519178579_u64;
_5.1 = false as i64;
_5.0 = !1641356615449190275_u64;
RET = [849028218686433561923923224997164637_i128];
_1 = 13233702441666492238_usize;
Goto(bb1)
}
bb1 = {
RET = [(-12232857276659520985311951413238559801_i128)];
_5.1 = -4415821289556360811_i64;
_7 = [245_u8,41_u8,227_u8];
_2 = '\u{8c1ea}';
_7 = [155_u8,16_u8,58_u8];
_4 = !10285_i16;
Goto(bb2)
}
bb2 = {
_5.1 = !3942763133127472027_i64;
_3 = !(-9223372036854775808_isize);
_8 = 12_i8 as f32;
RET = [(-57914833230511724153910327246484336875_i128)];
_8 = (-10_i8) as f32;
RET = [(-2360702076178712207175371609485947167_i128)];
_5.1 = (-7744671734857342173_i64);
_5.0 = 5660195114310279252_u64 ^ 5918824753366137405_u64;
RET = [(-13669122697622737553575598387970016877_i128)];
_5 = (13111543209283747092_u64, (-2351648856663708839_i64));
_9 = -_4;
_5 = (3889760809796952489_u64, (-1544551829321887906_i64));
_5 = (16341428626769936039_u64, 2378837110027718260_i64);
_1 = 8887922990010326162_usize + 7_usize;
RET = [(-137013934698234232130449629634328699654_i128)];
_3 = 18_isize;
_2 = '\u{64cd7}';
_4 = _9 << _1;
_5 = (4112682867908257225_u64, (-4430787209403875693_i64));
Goto(bb3)
}
bb3 = {
_7 = [201_u8,88_u8,28_u8];
_8 = 1558219881_u32 as f32;
_7 = [251_u8,214_u8,42_u8];
RET = [(-139751491392414741739341795099209188842_i128)];
match _3 {
0 => bb4,
1 => bb5,
18 => bb7,
_ => bb6
}
}
bb4 = {
_5.1 = !3942763133127472027_i64;
_3 = !(-9223372036854775808_isize);
_8 = 12_i8 as f32;
RET = [(-57914833230511724153910327246484336875_i128)];
_8 = (-10_i8) as f32;
RET = [(-2360702076178712207175371609485947167_i128)];
_5.1 = (-7744671734857342173_i64);
_5.0 = 5660195114310279252_u64 ^ 5918824753366137405_u64;
RET = [(-13669122697622737553575598387970016877_i128)];
_5 = (13111543209283747092_u64, (-2351648856663708839_i64));
_9 = -_4;
_5 = (3889760809796952489_u64, (-1544551829321887906_i64));
_5 = (16341428626769936039_u64, 2378837110027718260_i64);
_1 = 8887922990010326162_usize + 7_usize;
RET = [(-137013934698234232130449629634328699654_i128)];
_3 = 18_isize;
_2 = '\u{64cd7}';
_4 = _9 << _1;
_5 = (4112682867908257225_u64, (-4430787209403875693_i64));
Goto(bb3)
}
bb5 = {
RET = [(-12232857276659520985311951413238559801_i128)];
_5.1 = -4415821289556360811_i64;
_7 = [245_u8,41_u8,227_u8];
_2 = '\u{8c1ea}';
_7 = [155_u8,16_u8,58_u8];
_4 = !10285_i16;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_9 = !_4;
_10 = [_4,_4];
_10 = [_9,_9];
_9 = -_4;
_10 = [_4,_9];
_4 = !_9;
_13 = 6052_u16 | 52943_u16;
_3 = !(-43_isize);
Goto(bb8)
}
bb8 = {
_4 = !_9;
_2 = '\u{19b83}';
_9 = -_4;
_7 = [77_u8,16_u8,140_u8];
_5.0 = !6334498772929276427_u64;
_8 = 172_u8 as f32;
_8 = 56987594_u32 as f32;
_9 = _4 - _4;
_3 = _1 as isize;
_5.0 = 15957306219529698131_u64 << _9;
Call(_17.fld0.fld2 = fn1(_4, _5.0, _9, _2, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_8 = _9 as f32;
_17.fld0.fld0 = [_4,_9,_9,_9,_4];
_10 = [_4,_9];
_1 = (-927632796_i32) as usize;
_1 = _17.fld0.fld2 * _17.fld0.fld2;
_17.fld0.fld1.0 = core::ptr::addr_of!(_18);
_17.fld4 = [_3,_3,_3];
_3 = _9 as isize;
_17.fld5 = false as i32;
_5.1 = 7223961907279552059_i64;
_5 = (15948353675592051351_u64, 2014032239036618038_i64);
RET = [73859023789338508349969312622500193753_i128];
_20 = _8 as u64;
_17.fld0.fld3 = 10_i8 - (-14_i8);
_21.1 = [120594584102595099092736911898508900525_i128];
_17.fld3 = [(-28262630433668375217048095762017065825_i128)];
_17.fld1 = _10;
_20 = _5.0 & _5.0;
_18 = _2;
_17.fld1 = [_9,_9];
_5.0 = _20;
_17.fld0.fld4.fld1 = core::ptr::addr_of!(_5.0);
Goto(bb10)
}
bb10 = {
_17.fld0.fld0 = [_4,_4,_9,_4,_9];
_22 = 3638371950_u32 & 1593022805_u32;
_21.0 = &_1;
_15 = _7;
_17.fld0.fld1.1 = [false,true,true,false,false,true,true,true];
_1 = _17.fld0.fld2;
_17.fld0.fld1.0 = core::ptr::addr_of!(_2);
_17.fld0.fld2 = 94326661124451374333027829879729444003_u128 as usize;
_15 = _7;
match _5.1 {
0 => bb11,
1 => bb12,
2 => bb13,
2014032239036618038 => bb15,
_ => bb14
}
}
bb11 = {
RET = [(-12232857276659520985311951413238559801_i128)];
_5.1 = -4415821289556360811_i64;
_7 = [245_u8,41_u8,227_u8];
_2 = '\u{8c1ea}';
_7 = [155_u8,16_u8,58_u8];
_4 = !10285_i16;
Goto(bb2)
}
bb12 = {
_4 = !_9;
_2 = '\u{19b83}';
_9 = -_4;
_7 = [77_u8,16_u8,140_u8];
_5.0 = !6334498772929276427_u64;
_8 = 172_u8 as f32;
_8 = 56987594_u32 as f32;
_9 = _4 - _4;
_3 = _1 as isize;
_5.0 = 15957306219529698131_u64 << _9;
Call(_17.fld0.fld2 = fn1(_4, _5.0, _9, _2, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
_7 = [201_u8,88_u8,28_u8];
_8 = 1558219881_u32 as f32;
_7 = [251_u8,214_u8,42_u8];
RET = [(-139751491392414741739341795099209188842_i128)];
match _3 {
0 => bb4,
1 => bb5,
18 => bb7,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
_14 = _22;
_18 = _2;
_18 = _2;
_5 = (_20, (-1133448397289058422_i64));
RET = _21.1;
_23.1 = _3 as i64;
_17.fld5 = (-1347046498_i32);
_8 = _23.1 as f32;
_8 = _3 as f32;
RET = _21.1;
_19.0 = !_13;
_17.fld3 = [(-64771755059641036317224398897551794450_i128)];
_17.fld0.fld0 = [_4,_9,_4,_9,_9];
_26 = _5.1 >> _20;
_17.fld0.fld1.1 = [true,false,true,true,true,false,true,true];
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(0_usize, 26_usize, Move(_26), 2_usize, Move(_2), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(0_usize, 22_usize, Move(_22), 20_usize, Move(_20), 13_usize, Move(_13), 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i16,mut _2: u64,mut _3: i16,mut _4: char,mut _5: [i16; 2]) -> usize {
mir! {
type RET = usize;
let _6: [i16; 2];
let _7: isize;
let _8: [u8; 3];
let _9: [i128; 1];
let _10: [bool; 8];
let _11: u32;
let _12: isize;
let _13: (&'static usize, [i128; 1]);
let _14: char;
let _15: *const u64;
let _16: (u64, i64);
let _17: (i32, [bool; 8], u8);
let _18: usize;
let _19: *const bool;
let _20: char;
let _21: isize;
let _22: ();
let _23: ();
{
RET = 6_usize + 6_usize;
_4 = '\u{e2956}';
Call(_5 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _1 | _1;
RET = 8198132714271547037_usize;
_2 = 10975261309609394893_u64;
RET = 16990503788242735216_usize - 7237022469169516015_usize;
RET = 6824116842998675553_usize;
_4 = '\u{b86a4}';
RET = 5_usize - 5_usize;
_4 = '\u{7c82d}';
_3 = 179_u8 as i16;
_3 = _1;
_4 = '\u{10960e}';
_1 = 1159867326_i32 as i16;
_2 = 16125114396347773227_u64 << _1;
_5 = [_3,_1];
RET = !0_usize;
RET = 267535998007011188830695412042439494907_u128 as usize;
RET = 10500_u16 as usize;
_3 = false as i16;
_2 = !15613573249287843199_u64;
RET = 8189748582692659267_usize & 644957415930572579_usize;
RET = 6_usize + 4_usize;
RET = !5_usize;
_3 = -_1;
_1 = _3;
_4 = '\u{d7bea}';
_1 = -_3;
Call(_1 = fn2(_4, _3, _5, _4, _4, _4, _5, _5, RET, _2, _5, _4, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = !_1;
RET = !902107124588066252_usize;
_7 = 9223372036854775807_isize ^ (-94_isize);
_2 = 12807186252367735299_u64 ^ 1684790228688306080_u64;
_2 = 4048252373571196201_u64;
_1 = 216_u8 as i16;
_8 = [180_u8,15_u8,195_u8];
_6 = [_3,_3];
RET = (-87_i8) as usize;
_3 = _1;
_6 = [_3,_1];
_4 = '\u{a6850}';
RET = 7_usize;
_9 = [(-18905900350701895333217995724274793602_i128)];
_5 = _6;
_3 = _1 >> _2;
_3 = _1;
_5 = [_3,_3];
_4 = '\u{99dcc}';
_8 = [204_u8,11_u8,92_u8];
_4 = '\u{adadb}';
Goto(bb3)
}
bb3 = {
_8 = [233_u8,90_u8,75_u8];
_4 = '\u{38a93}';
_5 = [_1,_3];
_1 = _3;
_1 = _3 - _3;
_3 = _1;
match _2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
4048252373571196201 => bb8,
_ => bb7
}
}
bb4 = {
_3 = !_1;
RET = !902107124588066252_usize;
_7 = 9223372036854775807_isize ^ (-94_isize);
_2 = 12807186252367735299_u64 ^ 1684790228688306080_u64;
_2 = 4048252373571196201_u64;
_1 = 216_u8 as i16;
_8 = [180_u8,15_u8,195_u8];
_6 = [_3,_3];
RET = (-87_i8) as usize;
_3 = _1;
_6 = [_3,_1];
_4 = '\u{a6850}';
RET = 7_usize;
_9 = [(-18905900350701895333217995724274793602_i128)];
_5 = _6;
_3 = _1 >> _2;
_3 = _1;
_5 = [_3,_3];
_4 = '\u{99dcc}';
_8 = [204_u8,11_u8,92_u8];
_4 = '\u{adadb}';
Goto(bb3)
}
bb5 = {
_3 = _1 | _1;
RET = 8198132714271547037_usize;
_2 = 10975261309609394893_u64;
RET = 16990503788242735216_usize - 7237022469169516015_usize;
RET = 6824116842998675553_usize;
_4 = '\u{b86a4}';
RET = 5_usize - 5_usize;
_4 = '\u{7c82d}';
_3 = 179_u8 as i16;
_3 = _1;
_4 = '\u{10960e}';
_1 = 1159867326_i32 as i16;
_2 = 16125114396347773227_u64 << _1;
_5 = [_3,_1];
RET = !0_usize;
RET = 267535998007011188830695412042439494907_u128 as usize;
RET = 10500_u16 as usize;
_3 = false as i16;
_2 = !15613573249287843199_u64;
RET = 8189748582692659267_usize & 644957415930572579_usize;
RET = 6_usize + 4_usize;
RET = !5_usize;
_3 = -_1;
_1 = _3;
_4 = '\u{d7bea}';
_1 = -_3;
Call(_1 = fn2(_4, _3, _5, _4, _4, _4, _5, _5, RET, _2, _5, _4, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_7 = (-9223372036854775808_isize) >> _1;
_1 = !_3;
_3 = -_1;
_7 = _4 as isize;
_11 = !2359159276_u32;
_10[RET] = _3 > _1;
_9 = [(-86686669572651102185535862172379840839_i128)];
_3 = 182556439443466471017901941272057520100_u128 as i16;
_10[RET] = _2 <= _2;
_6 = [_3,_1];
_8 = [229_u8,66_u8,142_u8];
_6 = _5;
_3 = _1 ^ _1;
_11 = !3042608403_u32;
_1 = _3 - _3;
_11 = !2264984333_u32;
Goto(bb9)
}
bb9 = {
_10 = [true,false,true,false,false,false,true,false];
_11 = RET as u32;
_8 = [232_u8,230_u8,21_u8];
_10 = [false,false,true,true,false,false,true,false];
_10[RET] = true;
_4 = '\u{d30a0}';
_10 = [false,false,true,true,true,false,false,true];
RET = 14364_u16 as usize;
_13.1 = [(-125428591161089261412307038114372781423_i128)];
_2 = !6854340319273864946_u64;
_11 = true as u32;
_13.0 = &RET;
_11 = !1768142413_u32;
Goto(bb10)
}
bb10 = {
_9 = _13.1;
_1 = _3;
_8 = [124_u8,158_u8,235_u8];
Goto(bb11)
}
bb11 = {
_1 = -_3;
_15 = core::ptr::addr_of!(_2);
_12 = !_7;
_7 = _12;
_6 = _5;
_16.1 = 1985322344550611232_i64;
_11 = 1495520249_u32 + 199533596_u32;
_2 = 14087388743793048292_u64;
_15 = core::ptr::addr_of!(_16.0);
_8 = [58_u8,143_u8,60_u8];
_16 = (_2, (-988660219978974721_i64));
_13.0 = &RET;
_6 = [_3,_3];
_10 = [false,false,true,true,true,true,false,false];
_16.1 = -852886677667265238_i64;
_10 = [false,false,false,false,true,true,true,true];
match (*_15) {
0 => bb12,
14087388743793048292 => bb14,
_ => bb13
}
}
bb12 = {
_9 = _13.1;
_1 = _3;
_8 = [124_u8,158_u8,235_u8];
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
_5 = [_3,_3];
_17.2 = 168_u8 << _11;
_17.0 = (-1029914910_i32) + 611144946_i32;
_16.0 = !_2;
_15 = core::ptr::addr_of!(_16.0);
_3 = _1;
_13.0 = &_18;
_17.1 = [true,false,true,false,false,false,true,false];
RET = !3_usize;
_18 = RET;
_17 = (785674613_i32, _10, 202_u8);
_11 = _4 as u32;
_11 = !149254000_u32;
_4 = '\u{96f4}';
_6 = [_3,_1];
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(1_usize, 1_usize, Move(_1), 8_usize, Move(_8), 6_usize, Move(_6), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(1_usize, 17_usize, Move(_17), 4_usize, Move(_4), 18_usize, Move(_18), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: char,mut _2: i16,mut _3: [i16; 2],mut _4: char,mut _5: char,mut _6: char,mut _7: [i16; 2],mut _8: [i16; 2],mut _9: usize,mut _10: u64,mut _11: [i16; 2],mut _12: char,mut _13: [i16; 2]) -> i16 {
mir! {
type RET = i16;
let _14: u8;
let _15: isize;
let _16: [isize; 3];
let _17: Adt43;
let _18: [i128; 1];
let _19: ([i16; 5],);
let _20: isize;
let _21: u16;
let _22: &'static usize;
let _23: Adt44;
let _24: i64;
let _25: u16;
let _26: ();
let _27: ();
{
_12 = _6;
_8 = [_2,_2];
_1 = _4;
Goto(bb1)
}
bb1 = {
RET = _2 - _2;
_4 = _12;
_9 = 4699454743270547077_usize + 6_usize;
RET = -_2;
RET = false as i16;
_6 = _5;
RET = !_2;
_7 = _11;
_4 = _5;
_3 = [_2,_2];
_6 = _4;
_1 = _6;
Call(_12 = fn3(_8, _8, _11, _4, _10, _1, _5, _11, _5, _13, _5, _5, _13, _7, _13, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = [RET,RET];
_6 = _1;
_10 = 6525418127759574821_u64;
RET = _2 >> _9;
_17.fld0.fld4.fld1 = core::ptr::addr_of!(_10);
_17.fld0.fld3 = (-106_i8) << _9;
_3 = _13;
_17.fld0.fld4.fld0 = _17.fld0.fld3 as f32;
_17.fld5 = (-273248242_i32);
_13 = _8;
_17.fld0.fld1.0 = core::ptr::addr_of!(_12);
_17.fld3 = [(-58076379744541062548229274884230406932_i128)];
_17.fld0.fld2 = _9 >> _17.fld5;
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
6525418127759574821 => bb10,
_ => bb9
}
}
bb3 = {
RET = _2 - _2;
_4 = _12;
_9 = 4699454743270547077_usize + 6_usize;
RET = -_2;
RET = false as i16;
_6 = _5;
RET = !_2;
_7 = _11;
_4 = _5;
_3 = [_2,_2];
_6 = _4;
_1 = _6;
Call(_12 = fn3(_8, _8, _11, _4, _10, _1, _5, _11, _5, _13, _5, _5, _13, _7, _13, _13), ReturnTo(bb2), UnwindUnreachable())
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
_17.fld4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = [RET,RET];
_17.fld0.fld2 = _9;
_17.fld1 = [RET,RET];
_11 = [_2,RET];
_20 = !(-9223372036854775808_isize);
_5 = _1;
_10 = !17805366182918863174_u64;
Goto(bb11)
}
bb11 = {
_5 = _6;
_4 = _12;
_12 = _6;
RET = _10 as i16;
_10 = 6521539191921480587_u64 & 17437140722457349791_u64;
_17.fld0.fld1.1 = [true,false,true,true,true,false,false,true];
_18 = [(-20942330933863146280980537648690611833_i128)];
_15 = _20 | _20;
Goto(bb12)
}
bb12 = {
_8 = [_2,RET];
_17.fld4 = [_20,_15,_15];
_6 = _5;
_17.fld5 = false as i32;
_5 = _6;
_8 = [_2,RET];
_7 = [_2,_2];
_14 = 30_u8;
_3 = [RET,_2];
_5 = _12;
match _14 {
0 => bb13,
1 => bb14,
30 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
RET = _2 - _2;
_4 = _12;
_9 = 4699454743270547077_usize + 6_usize;
RET = -_2;
RET = false as i16;
_6 = _5;
RET = !_2;
_7 = _11;
_4 = _5;
_3 = [_2,_2];
_6 = _4;
_1 = _6;
Call(_12 = fn3(_8, _8, _11, _4, _10, _1, _5, _11, _5, _13, _5, _5, _13, _7, _13, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_4 = _5;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(2_usize, 10_usize, Move(_10), 1_usize, Move(_1), 14_usize, Move(_14), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(2_usize, 15_usize, Move(_15), 18_usize, Move(_18), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i16; 2],mut _2: [i16; 2],mut _3: [i16; 2],mut _4: char,mut _5: u64,mut _6: char,mut _7: char,mut _8: [i16; 2],mut _9: char,mut _10: [i16; 2],mut _11: char,mut _12: char,mut _13: [i16; 2],mut _14: [i16; 2],mut _15: [i16; 2],mut _16: [i16; 2]) -> char {
mir! {
type RET = char;
let _17: bool;
let _18: bool;
let _19: bool;
let _20: Adt53;
let _21: Adt46;
let _22: f32;
let _23: u64;
let _24: [i128; 1];
let _25: Adt53;
let _26: (i32, [bool; 8], u8);
let _27: i16;
let _28: isize;
let _29: i16;
let _30: u8;
let _31: *mut *mut i64;
let _32: i64;
let _33: char;
let _34: [u8; 3];
let _35: i128;
let _36: Adt48;
let _37: isize;
let _38: bool;
let _39: [i16; 2];
let _40: *mut f64;
let _41: f64;
let _42: char;
let _43: ([i16; 5],);
let _44: [isize; 3];
let _45: usize;
let _46: [isize; 3];
let _47: Adt47;
let _48: [i16; 2];
let _49: f32;
let _50: ([i16; 5],);
let _51: isize;
let _52: i128;
let _53: i32;
let _54: *mut *mut i64;
let _55: u64;
let _56: i32;
let _57: usize;
let _58: *const char;
let _59: isize;
let _60: u16;
let _61: *const bool;
let _62: Adt44;
let _63: [i128; 1];
let _64: Adt53;
let _65: [u8; 3];
let _66: u64;
let _67: char;
let _68: [isize; 3];
let _69: [i128; 1];
let _70: char;
let _71: f32;
let _72: (i32, [bool; 8], u8);
let _73: f64;
let _74: [i128; 1];
let _75: f32;
let _76: (u16, *mut *mut i64);
let _77: bool;
let _78: *mut *mut i64;
let _79: [i16; 2];
let _80: ();
let _81: ();
{
_15 = [27043_i16,(-2721_i16)];
RET = _9;
_6 = _11;
_2 = _15;
_7 = _9;
_12 = _4;
RET = _11;
_3 = _8;
_16 = _8;
_10 = [(-24276_i16),(-987_i16)];
_8 = [23620_i16,13431_i16];
_12 = _6;
_12 = _6;
Goto(bb1)
}
bb1 = {
_10 = _1;
_16 = _1;
RET = _11;
_11 = _7;
_2 = _13;
_13 = [(-12345_i16),(-7140_i16)];
_2 = _16;
_3 = _10;
_4 = _12;
_17 = _5 < _5;
RET = _9;
_18 = _17;
_19 = _5 >= _5;
_2 = [3649_i16,18526_i16];
_22 = 2529604965_u32 as f32;
_8 = [3570_i16,23713_i16];
_23 = !_5;
_24 = [(-23299554145955317833667233063695686892_i128)];
Goto(bb2)
}
bb2 = {
_24 = [154961063507536170527584908657756795031_i128];
_26.2 = 19576_u16 as u8;
_1 = _13;
_26.1 = [_18,_19,_17,_18,_17,_18,_17,_17];
_4 = _9;
RET = _4;
RET = _6;
_6 = RET;
_2 = [(-1818_i16),(-13147_i16)];
_7 = _6;
_1 = [27217_i16,15027_i16];
_18 = !_19;
_13 = _2;
_23 = _5 >> _5;
Goto(bb3)
}
bb3 = {
_18 = _19;
_10 = _3;
_27 = _23 as i16;
_3 = [_27,_27];
_26.2 = !61_u8;
_10 = [_27,_27];
_22 = _27 as f32;
_26.0 = 30919205463652241835772171247108627877_i128 as i32;
_18 = _17;
_17 = _26.0 >= _26.0;
_10 = [_27,_27];
_28 = _22 as isize;
_11 = _12;
_22 = 3869075119_u32 as f32;
RET = _12;
_5 = _23;
_9 = _6;
Goto(bb4)
}
bb4 = {
_19 = _17;
_15 = [_27,_27];
_27 = (-2378_i16) - (-26536_i16);
_30 = _26.2;
_24 = [(-169991454527677514776862769367248511955_i128)];
_26.2 = _30 ^ _30;
_6 = RET;
_17 = _11 >= RET;
_33 = _9;
_29 = _27 & _27;
_32 = 1054339009761208220_i64;
_3 = [_29,_27];
_26.0 = _17 as i32;
_27 = _29;
_27 = _23 as i16;
_3 = [_29,_29];
_26.0 = 1301759905_i32;
_19 = _7 > _33;
RET = _11;
_4 = _12;
_8 = [_29,_27];
_30 = _26.2;
Call(_21 = fn4(_26.2, _28, _33, _28, _23, _33, RET), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
SetDiscriminant(Field::<Adt39>(Variant(_21, 2), 1), 0);
_4 = _6;
place!(Field::<u32>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 4)) = _23 as u32;
place!(Field::<i32>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 5)) = Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).2 as i32;
_34 = [_26.2,_30,Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).2];
place!(Field::<u8>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 3)) = 5174839275887867852_usize as u8;
_4 = _12;
_12 = _9;
Goto(bb6)
}
bb6 = {
_34 = [Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).2,Field::<u8>(Variant(Field::<Adt39>(Variant(_21, 2), 1), 0), 3),_30];
_4 = _11;
_8 = [_29,_29];
_19 = _18 & _17;
_2 = [_29,_29];
_24 = [(-14388796253407555775810333796973877809_i128)];
_26.0 = _22 as i32;
place!(Field::<i32>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 5)) = Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).0 & Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).0;
_28 = 9223372036854775807_isize | 9223372036854775807_isize;
place!(Field::<char>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 1)) = _33;
_39 = [_29,_27];
_7 = _33;
_26.2 = Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).2 + Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).2;
_24 = [(-133513022532538806040213766464877322692_i128)];
place!(Field::<u8>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 3)) = _4 as u8;
_11 = _33;
_1 = [_27,_29];
_37 = _28;
Goto(bb7)
}
bb7 = {
_37 = _28 - _28;
place!(Field::<u32>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 4)) = 1314453637_u32 * 358521895_u32;
_30 = _26.2 & _26.2;
_12 = _6;
_13 = [_29,_29];
_30 = _26.2 >> Field::<i32>(Variant(Field::<Adt39>(Variant(_21, 2), 1), 0), 5);
RET = _33;
place!(Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2)).1 = [_19,_18,_17,_18,_18,_19,_19,_19];
place!(Field::<isize>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 2)) = _28;
_26.1 = Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).1;
_22 = _23 as f32;
_13 = [_29,_29];
_39 = _2;
_43 = (Field::<([i16; 5],)>(Variant(_21, 2), 0).0,);
_16 = [_29,_29];
place!(Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2)).0 = _17 as i32;
_37 = -Field::<isize>(Variant(Field::<Adt39>(Variant(_21, 2), 1), 0), 2);
place!(Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2)).2 = _30;
place!(Field::<u32>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 4)) = !4117942567_u32;
_44 = [Field::<isize>(Variant(Field::<Adt39>(Variant(_21, 2), 1), 0), 2),Field::<isize>(Variant(Field::<Adt39>(Variant(_21, 2), 1), 0), 2),Field::<isize>(Variant(Field::<Adt39>(Variant(_21, 2), 1), 0), 2)];
match _32 {
0 => bb4,
1 => bb2,
1054339009761208220 => bb8,
_ => bb6
}
}
bb8 = {
_24 = [(-165142293794210092123088119433127417782_i128)];
_16 = _1;
_26.2 = Field::<i32>(Variant(Field::<Adt39>(Variant(_21, 2), 1), 0), 5) as u8;
_39 = _2;
_45 = 0_usize | 4882374389710781682_usize;
_4 = RET;
_42 = _33;
_43 = (Field::<([i16; 5],)>(Variant(_21, 2), 0).0,);
_32 = (-1955286791884731827_i64);
_35 = !(-58734347347185635116166221058949264040_i128);
_18 = _27 >= _29;
_28 = _23 as isize;
_49 = -_22;
_32 = 97003264078575209_i64 - 8020910456572949072_i64;
place!(Field::<([i16; 5],)>(Variant(_21, 2), 0)) = (_43.0,);
_40 = core::ptr::addr_of_mut!(_41);
_41 = Field::<i32>(Variant(Field::<Adt39>(Variant(_21, 2), 1), 0), 5) as f64;
place!(Field::<([i16; 5],)>(Variant(_21, 2), 0)).0 = _43.0;
_13 = _2;
_22 = _49;
_40 = core::ptr::addr_of_mut!((*_40));
place!(Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2)).2 = 13689_u16 as u8;
Goto(bb9)
}
bb9 = {
place!(Field::<i32>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 5)) = !Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).0;
_34 = [_30,_26.2,_30];
_26.1 = [_18,_19,_17,_18,_18,_18,_18,_17];
_41 = _26.2 as f64;
place!(Field::<i32>(Variant(place!(Field::<Adt39>(Variant(_21, 2), 1)), 0), 5)) = -Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2).0;
_34 = [_30,_26.2,_26.2];
_33 = _42;
_17 = _18;
_16 = [_29,_27];
_35 = (-31643599382852510271543400698421596685_i128);
_51 = -_37;
_50 = Field::<([i16; 5],)>(Variant(_21, 2), 0);
_38 = _29 != _29;
_16 = [_29,_29];
_19 = _18;
Goto(bb10)
}
bb10 = {
_52 = _6 as i128;
_17 = _18;
_48 = [_29,_27];
_65 = _34;
_66 = _32 as u64;
_50.0 = Field::<([i16; 5],)>(Variant(_21, 2), 0).0;
place!(Field::<([i16; 5],)>(Variant(_21, 2), 0)).0 = [_29,_29,_27,_29,_29];
_11 = _9;
_21 = Adt46::Variant1 { fld0: _52 };
_22 = 58173_u16 as f32;
_26.2 = !_30;
_29 = -_27;
_26.2 = _30;
_32 = (-8501009467611506146_i64) * 5571518897173894118_i64;
_23 = _5 - _5;
_63 = [Field::<i128>(Variant(_21, 1), 0)];
_35 = _52 << _23;
_39 = [_29,_29];
_58 = core::ptr::addr_of!(_67);
_56 = _28 as i32;
_55 = _66 & _5;
_56 = -_26.0;
_2 = [_27,_29];
_18 = _17 != _17;
RET = _42;
_37 = _56 as isize;
_57 = _45 ^ _45;
Goto(bb11)
}
bb11 = {
_12 = _42;
_8 = [_29,_27];
_53 = _56;
_43 = (_50.0,);
_41 = _56 as f64;
_66 = _23;
_44 = [_28,_28,_28];
_63 = [_35];
_40 = core::ptr::addr_of_mut!((*_40));
_10 = _1;
_71 = _49;
_19 = _18 & _17;
_72 = (_53, _26.1, _26.2);
_16 = _48;
_24 = [_35];
Goto(bb12)
}
bb12 = {
_38 = _18 & _19;
_68 = [_51,_51,_28];
_22 = _71;
_16 = [_29,_27];
Goto(bb13)
}
bb13 = {
_48 = [_29,_27];
_48 = [_27,_27];
_67 = _12;
_7 = _4;
_72.2 = _26.2 ^ _26.2;
_59 = _42 as isize;
_17 = _72.2 >= _72.2;
_26.1 = [_17,_17,_19,_38,_17,_38,_38,_17];
_22 = _49 - _49;
_32 = 246512976339783235761316137713783634265_u128 as i64;
_38 = _17;
_50.0 = [_27,_27,_29,_27,_29];
_39 = [_29,_29];
_11 = RET;
RET = _67;
_38 = _17;
_2 = [_29,_27];
_60 = !309_u16;
_53 = _26.0 >> _72.2;
_71 = -_49;
Goto(bb14)
}
bb14 = {
_59 = _37;
_69 = [_52];
_72.0 = !_53;
_45 = _57 + _57;
SetDiscriminant(_21, 2);
_34 = _65;
_33 = (*_58);
_52 = _35 & _35;
_61 = core::ptr::addr_of!(_18);
_4 = _11;
_66 = _23 + _23;
_42 = _67;
_58 = core::ptr::addr_of!(_42);
_55 = _66;
_74 = _63;
place!(Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2)).2 = _32 as u8;
_38 = !(*_61);
place!(Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2)).2 = !_72.2;
_77 = _72.0 != _72.0;
_53 = _72.0;
_72 = (_53, _26.1, _30);
_72 = _26;
_60 = !23914_u16;
_52 = _35 - _35;
_49 = _71;
place!(Field::<(i32, [bool; 8], u8)>(Variant(_21, 2), 2)).1 = [_18,_77,_77,_19,_77,_17,_77,_17];
_70 = _67;
Goto(bb15)
}
bb15 = {
Call(_80 = dump_var(3_usize, 4_usize, Move(_4), 19_usize, Move(_19), 53_usize, Move(_53), 38_usize, Move(_38)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_80 = dump_var(3_usize, 26_usize, Move(_26), 43_usize, Move(_43), 30_usize, Move(_30), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_80 = dump_var(3_usize, 32_usize, Move(_32), 37_usize, Move(_37), 7_usize, Move(_7), 50_usize, Move(_50)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_80 = dump_var(3_usize, 27_usize, Move(_27), 42_usize, Move(_42), 60_usize, Move(_60), 63_usize, Move(_63)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_80 = dump_var(3_usize, 70_usize, Move(_70), 12_usize, Move(_12), 59_usize, Move(_59), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_80 = dump_var(3_usize, 48_usize, Move(_48), 13_usize, Move(_13), 74_usize, Move(_74), 35_usize, Move(_35)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_80 = dump_var(3_usize, 34_usize, Move(_34), 17_usize, Move(_17), 72_usize, Move(_72), 52_usize, Move(_52)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u8,mut _2: isize,mut _3: char,mut _4: isize,mut _5: u64,mut _6: char,mut _7: char) -> Adt46 {
mir! {
type RET = Adt46;
let _8: Adt42;
let _9: Adt41;
let _10: ([i16; 5],);
let _11: Adt50;
let _12: u8;
let _13: isize;
let _14: [isize; 3];
let _15: char;
let _16: f32;
let _17: isize;
let _18: ([i16; 5],);
let _19: isize;
let _20: i16;
let _21: char;
let _22: (([i16; 5],), *mut *mut i64, [isize; 3], [isize; 3]);
let _23: usize;
let _24: isize;
let _25: bool;
let _26: Adt44;
let _27: [isize; 3];
let _28: f64;
let _29: [bool; 8];
let _30: [u8; 3];
let _31: bool;
let _32: Adt46;
let _33: i64;
let _34: *mut f64;
let _35: i32;
let _36: *mut f64;
let _37: Adt39;
let _38: [bool; 8];
let _39: bool;
let _40: [i128; 1];
let _41: f64;
let _42: [i16; 5];
let _43: Adt55;
let _44: bool;
let _45: i16;
let _46: isize;
let _47: isize;
let _48: ();
let _49: ();
{
_1 = 6_u8 >> _2;
_5 = 29133_u16 as u64;
_6 = _7;
_5 = !17741385550177542272_u64;
RET = Adt46::Variant1 { fld0: 164324445525576434483763215874166512905_i128 };
place!(Field::<i128>(Variant(RET, 1), 0)) = !4476757256568632524334464809529465913_i128;
_8.fld3 = !111_i8;
_7 = _3;
_8.fld4.fld0 = Field::<i128>(Variant(RET, 1), 0) as f32;
place!(Field::<i128>(Variant(RET, 1), 0)) = -158009366681574797586486357162036786293_i128;
_8.fld4.fld1 = core::ptr::addr_of!(_5);
_6 = _3;
_5 = !19908514085788195_u64;
_2 = !_4;
_7 = _3;
SetDiscriminant(RET, 1);
place!(Field::<i128>(Variant(RET, 1), 0)) = (-81228548514715654343572421375906656599_i128) + (-135748476996580537373217436961115656451_i128);
_8.fld4.fld1 = core::ptr::addr_of!(_5);
_1 = 47671_u16 as u8;
_5 = 32074335545689058_i64 as u64;
_8.fld1.1 = [false,true,true,true,false,false,false,true];
_8.fld1.0 = core::ptr::addr_of!(_7);
_6 = _7;
_8.fld3 = true as i8;
_4 = !_2;
_10.0 = [(-3531_i16),(-22841_i16),(-17306_i16),(-21541_i16),12667_i16];
Goto(bb1)
}
bb1 = {
_3 = _7;
Goto(bb2)
}
bb2 = {
_5 = 18276214408806935999_u64 << _2;
_8.fld2 = !13089223862015017412_usize;
_13 = (-1049346955_i32) as isize;
_1 = !103_u8;
_8.fld1.1 = [false,true,true,false,false,true,false,false];
_8.fld0 = [(-32344_i16),(-12255_i16),27393_i16,(-19571_i16),24387_i16];
_8.fld2 = 15428814118668423271_usize;
_8.fld0 = [32582_i16,8280_i16,21784_i16,20506_i16,(-9677_i16)];
_6 = _3;
_8.fld4.fld1 = core::ptr::addr_of!(_5);
_8.fld4.fld0 = _2 as f32;
_15 = _6;
_15 = _3;
_8.fld1.0 = core::ptr::addr_of!(_6);
_10.0 = _8.fld0;
SetDiscriminant(RET, 0);
_10.0 = _8.fld0;
place!(Field::<i64>(Variant(RET, 0), 3)) = 8217516702173216689_i64 >> _2;
place!(Field::<i16>(Variant(RET, 0), 4)) = true as i16;
place!(Field::<i16>(Variant(RET, 0), 4)) = (-453_i16);
_14 = [_2,_2,_2];
_13 = _2;
RET = Adt46::Variant1 { fld0: 55501526365842486712157249858806383493_i128 };
_15 = _3;
_12 = _1 & _1;
_8.fld0 = [10201_i16,27779_i16,(-28684_i16),(-19523_i16),11223_i16];
_8.fld4.fld0 = 5559362802282929578_i64 as f32;
place!(Field::<i128>(Variant(RET, 1), 0)) = _8.fld4.fld0 as i128;
Goto(bb3)
}
bb3 = {
_8.fld2 = 6730802251837203870_usize;
_6 = _7;
_8.fld4.fld1 = core::ptr::addr_of!(_5);
place!(Field::<i128>(Variant(RET, 1), 0)) = (-91229272057950994559216371115456751731_i128) | 144731863994662660773251810477276563627_i128;
_7 = _6;
_7 = _15;
_16 = _8.fld4.fld0 + _8.fld4.fld0;
_10.0 = [(-11728_i16),30629_i16,21777_i16,23856_i16,(-21082_i16)];
_17 = _2 * _2;
_15 = _7;
_6 = _15;
SetDiscriminant(RET, 3);
Goto(bb4)
}
bb4 = {
place!(Field::<char>(Variant(RET, 3), 1)) = _15;
place!(Field::<[u8; 3]>(Variant(RET, 3), 0)) = [_12,_12,_12];
_10.0 = [(-14267_i16),(-32135_i16),18675_i16,(-5045_i16),(-4041_i16)];
place!(Field::<i64>(Variant(RET, 3), 6)) = 7268629778344584325_i64;
place!(Field::<*mut u64>(Variant(RET, 3), 4)) = core::ptr::addr_of_mut!(_5);
_5 = 2526683161310642990_u64;
_1 = _8.fld2 as u8;
place!(Field::<*mut u64>(Variant(RET, 3), 4)) = core::ptr::addr_of_mut!(_5);
_8.fld4.fld1 = core::ptr::addr_of!(_5);
place!(Field::<[u8; 3]>(Variant(RET, 3), 0)) = [_12,_12,_12];
_3 = _15;
_5 = 17528693744617351232_u64;
_8.fld2 = !4603186212042104068_usize;
_5 = 17753846112063969055_u64;
place!(Field::<i64>(Variant(RET, 3), 6)) = -(-8851860060521763958_i64);
Goto(bb5)
}
bb5 = {
_14 = [_17,_13,_13];
place!(Field::<([i16; 5],)>(Variant(RET, 3), 5)) = (_8.fld0,);
place!(Field::<*const u64>(Variant(RET, 3), 2)) = core::ptr::addr_of!(_5);
place!(Field::<([i16; 5],)>(Variant(RET, 3), 5)) = (_10.0,);
place!(Field::<char>(Variant(RET, 3), 1)) = _3;
_12 = _1;
_2 = !_17;
_8.fld4.fld1 = core::ptr::addr_of!(_5);
place!(Field::<i8>(Variant(RET, 3), 3)) = true as i8;
_18.0 = Field::<([i16; 5],)>(Variant(RET, 3), 5).0;
_8.fld4.fld0 = _16;
place!(Field::<i64>(Variant(RET, 3), 6)) = _5 as i64;
_8.fld4.fld0 = _16;
_8.fld2 = 4_usize;
place!(Field::<[u8; 3]>(Variant(RET, 3), 0)) = [_12,_1,_1];
_8.fld2 = _1 as usize;
_21 = _6;
_8.fld4.fld1 = Field::<*const u64>(Variant(RET, 3), 2);
_13 = _2;
place!(Field::<*const u64>(Variant(RET, 3), 2)) = _8.fld4.fld1;
match _5 {
0 => bb3,
1 => bb2,
17753846112063969055 => bb7,
_ => bb6
}
}
bb6 = {
_5 = 18276214408806935999_u64 << _2;
_8.fld2 = !13089223862015017412_usize;
_13 = (-1049346955_i32) as isize;
_1 = !103_u8;
_8.fld1.1 = [false,true,true,false,false,true,false,false];
_8.fld0 = [(-32344_i16),(-12255_i16),27393_i16,(-19571_i16),24387_i16];
_8.fld2 = 15428814118668423271_usize;
_8.fld0 = [32582_i16,8280_i16,21784_i16,20506_i16,(-9677_i16)];
_6 = _3;
_8.fld4.fld1 = core::ptr::addr_of!(_5);
_8.fld4.fld0 = _2 as f32;
_15 = _6;
_15 = _3;
_8.fld1.0 = core::ptr::addr_of!(_6);
_10.0 = _8.fld0;
SetDiscriminant(RET, 0);
_10.0 = _8.fld0;
place!(Field::<i64>(Variant(RET, 0), 3)) = 8217516702173216689_i64 >> _2;
place!(Field::<i16>(Variant(RET, 0), 4)) = true as i16;
place!(Field::<i16>(Variant(RET, 0), 4)) = (-453_i16);
_14 = [_2,_2,_2];
_13 = _2;
RET = Adt46::Variant1 { fld0: 55501526365842486712157249858806383493_i128 };
_15 = _3;
_12 = _1 & _1;
_8.fld0 = [10201_i16,27779_i16,(-28684_i16),(-19523_i16),11223_i16];
_8.fld4.fld0 = 5559362802282929578_i64 as f32;
place!(Field::<i128>(Variant(RET, 1), 0)) = _8.fld4.fld0 as i128;
Goto(bb3)
}
bb7 = {
place!(Field::<([i16; 5],)>(Variant(RET, 3), 5)) = _18;
place!(Field::<*mut u64>(Variant(RET, 3), 4)) = core::ptr::addr_of_mut!(_5);
_8.fld3 = _1 as i8;
place!(Field::<i64>(Variant(RET, 3), 6)) = (-8478487109115632485_i64) & (-8740599233609397183_i64);
_7 = _21;
place!(Field::<char>(Variant(RET, 3), 1)) = _7;
place!(Field::<[u8; 3]>(Variant(RET, 3), 0)) = [_12,_12,_1];
Goto(bb8)
}
bb8 = {
_23 = _8.fld2 >> _17;
_22.0 = _18;
place!(Field::<*mut u64>(Variant(RET, 3), 4)) = core::ptr::addr_of_mut!(_5);
SetDiscriminant(RET, 0);
_19 = _4 << _13;
_8.fld1.0 = core::ptr::addr_of!(_15);
_18 = _22.0;
_20 = -31715_i16;
_6 = _7;
_17 = _19;
Goto(bb9)
}
bb9 = {
place!(Field::<f64>(Variant(RET, 0), 0)) = _19 as f64;
place!(Field::<i16>(Variant(RET, 0), 4)) = -_20;
_10.0 = [Field::<i16>(Variant(RET, 0), 4),_20,_20,_20,Field::<i16>(Variant(RET, 0), 4)];
_23 = !_8.fld2;
place!(Field::<[i16; 5]>(Variant(RET, 0), 1)) = [_20,_20,Field::<i16>(Variant(RET, 0), 4),_20,Field::<i16>(Variant(RET, 0), 4)];
_28 = 312264417097342609594990680733462873192_u128 as f64;
_8.fld4.fld0 = _16 + _16;
_26 = Adt44::Variant1 { fld0: _23 };
_7 = _15;
_22.2 = [_4,_17,_17];
_22.0.0 = _8.fld0;
_22.3 = [_13,_17,_17];
_24 = _17;
place!(Field::<f64>(Variant(RET, 0), 0)) = 53195_u16 as f64;
_8.fld4.fld1 = core::ptr::addr_of!(_5);
_22.3 = [_19,_2,_24];
place!(Field::<([i16; 5],)>(Variant(RET, 0), 2)).0 = [_20,_20,_20,_20,_20];
place!(Field::<[i16; 5]>(Variant(RET, 0), 1)) = [Field::<i16>(Variant(RET, 0), 4),_20,_20,_20,Field::<i16>(Variant(RET, 0), 4)];
place!(Field::<usize>(Variant(_26, 1), 0)) = _23;
_28 = Field::<f64>(Variant(RET, 0), 0);
_10.0 = _8.fld0;
RET = Adt46::Variant0 { fld0: _28,fld1: _10.0,fld2: _22.0,fld3: 5000165002642989844_i64,fld4: _20 };
match _5 {
17753846112063969055 => bb11,
_ => bb10
}
}
bb10 = {
_5 = 18276214408806935999_u64 << _2;
_8.fld2 = !13089223862015017412_usize;
_13 = (-1049346955_i32) as isize;
_1 = !103_u8;
_8.fld1.1 = [false,true,true,false,false,true,false,false];
_8.fld0 = [(-32344_i16),(-12255_i16),27393_i16,(-19571_i16),24387_i16];
_8.fld2 = 15428814118668423271_usize;
_8.fld0 = [32582_i16,8280_i16,21784_i16,20506_i16,(-9677_i16)];
_6 = _3;
_8.fld4.fld1 = core::ptr::addr_of!(_5);
_8.fld4.fld0 = _2 as f32;
_15 = _6;
_15 = _3;
_8.fld1.0 = core::ptr::addr_of!(_6);
_10.0 = _8.fld0;
SetDiscriminant(RET, 0);
_10.0 = _8.fld0;
place!(Field::<i64>(Variant(RET, 0), 3)) = 8217516702173216689_i64 >> _2;
place!(Field::<i16>(Variant(RET, 0), 4)) = true as i16;
place!(Field::<i16>(Variant(RET, 0), 4)) = (-453_i16);
_14 = [_2,_2,_2];
_13 = _2;
RET = Adt46::Variant1 { fld0: 55501526365842486712157249858806383493_i128 };
_15 = _3;
_12 = _1 & _1;
_8.fld0 = [10201_i16,27779_i16,(-28684_i16),(-19523_i16),11223_i16];
_8.fld4.fld0 = 5559362802282929578_i64 as f32;
place!(Field::<i128>(Variant(RET, 1), 0)) = _8.fld4.fld0 as i128;
Goto(bb3)
}
bb11 = {
_14 = [_2,_17,_4];
_4 = _19 + _17;
_30 = [_1,_12,_12];
Goto(bb12)
}
bb12 = {
SetDiscriminant(_26, 1);
place!(Field::<i16>(Variant(RET, 0), 4)) = Field::<f64>(Variant(RET, 0), 0) as i16;
_18.0 = [_20,_20,Field::<i16>(Variant(RET, 0), 4),Field::<i16>(Variant(RET, 0), 4),Field::<i16>(Variant(RET, 0), 4)];
place!(Field::<([i16; 5],)>(Variant(RET, 0), 2)) = _10;
_16 = (-615022099963666893792042005726548212_i128) as f32;
_8.fld4.fld0 = 1029156697_i32 as f32;
_24 = _19;
place!(Field::<[i16; 5]>(Variant(RET, 0), 1)) = [_20,_20,_20,Field::<i16>(Variant(RET, 0), 4),Field::<i16>(Variant(RET, 0), 4)];
_8.fld2 = 40990_u16 as usize;
_10.0 = _8.fld0;
_4 = _13;
_10 = (_22.0.0,);
_12 = _1 ^ _1;
_7 = _15;
_5 = !7559440464364700145_u64;
place!(Field::<f64>(Variant(RET, 0), 0)) = -_28;
_12 = _1 ^ _1;
_23 = _8.fld2 ^ _8.fld2;
_15 = _6;
_26 = Adt44::Variant1 { fld0: _23 };
_24 = _17 - _19;
place!(Field::<i16>(Variant(RET, 0), 4)) = _20 + _20;
_24 = _28 as isize;
_12 = Field::<i16>(Variant(RET, 0), 4) as u8;
_8.fld4.fld0 = -_16;
_22.0 = (Field::<([i16; 5],)>(Variant(RET, 0), 2).0,);
_24 = _19;
Call(place!(Field::<i64>(Variant(RET, 0), 3)) = fn5(_14, _17, _4, _19, _2, _7, _8.fld1.0, Field::<usize>(Variant(_26, 1), 0), _22.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<usize>(Variant(_26, 1), 0)) = _17 as usize;
_22.0.0 = _8.fld0;
_2 = _20 as isize;
_1 = _12;
place!(Field::<[i16; 5]>(Variant(RET, 0), 1)) = _22.0.0;
_27 = [_4,_19,_24];
place!(Field::<i64>(Variant(RET, 0), 3)) = (-5187592084193410693_i64);
_29 = [false,true,true,false,false,true,false,false];
SetDiscriminant(RET, 2);
_22.2 = [_19,_24,_24];
_18.0 = [_20,_20,_20,_20,_20];
_25 = _24 >= _19;
_13 = _17;
place!(Field::<(i32, [bool; 8], u8)>(Variant(RET, 2), 2)).1 = [_25,_25,_25,_25,_25,_25,_25,_25];
Call(_5 = core::intrinsics::bswap(17501192501929315658_u64), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<i8>(Variant(RET, 2), 3)) = _8.fld3;
_36 = core::ptr::addr_of_mut!(_28);
SetDiscriminant(_26, 3);
place!(Field::<([i16; 5],)>(Variant(RET, 2), 0)) = (_8.fld0,);
_7 = _21;
_21 = _7;
_35 = _1 as i32;
_4 = _17 ^ _19;
_37 = Adt39::Variant0 { fld0: _27,fld1: _7,fld2: _19,fld3: _1,fld4: 2059858062_u32,fld5: _35 };
_8.fld1.0 = core::ptr::addr_of!(_15);
_38 = [_25,_25,_25,_25,_25,_25,_25,_25];
_18 = Field::<([i16; 5],)>(Variant(RET, 2), 0);
_35 = _23 as i32;
_33 = (*_36) as i64;
place!(Field::<u8>(Variant(_37, 0), 3)) = _24 as u8;
Goto(bb15)
}
bb15 = {
_4 = !_13;
_22.0 = (_10.0,);
place!(Field::<[isize; 3]>(Variant(_37, 0), 0)) = [_4,_24,Field::<isize>(Variant(_37, 0), 2)];
_5 = !14136336045493382497_u64;
_5 = _25 as u64;
place!(Field::<char>(Variant(_26, 3), 1)) = _15;
_38 = Field::<(i32, [bool; 8], u8)>(Variant(RET, 2), 2).1;
place!(Field::<(i32, [bool; 8], u8)>(Variant(RET, 2), 2)).2 = _1 + Field::<u8>(Variant(_37, 0), 3);
place!(Field::<([i16; 5],)>(Variant(RET, 2), 0)).0 = _22.0.0;
place!(Field::<*mut u64>(Variant(_26, 3), 6)) = core::ptr::addr_of_mut!(_5);
_22.0 = _18;
place!(Field::<i16>(Variant(_26, 3), 4)) = _20;
place!(Field::<Adt40>(Variant(_26, 3), 2)).fld1 = _8.fld4.fld1;
place!(Field::<i16>(Variant(_26, 3), 4)) = _20;
_42 = _8.fld0;
Call(place!(Field::<(i32, [bool; 8], u8)>(Variant(RET, 2), 2)) = fn18(_13, Field::<([i16; 5],)>(Variant(RET, 2), 0).0, _8.fld1, _18.0, _23, _8.fld4.fld1, (*_36)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_8.fld4 = Adt40 { fld0: _16,fld1: Field::<Adt40>(Variant(_26, 3), 2).fld1 };
place!(Field::<Adt39>(Variant(RET, 2), 1)) = Adt39::Variant0 { fld0: _22.2,fld1: _6,fld2: _19,fld3: Field::<u8>(Variant(_37, 0), 3),fld4: 1798391659_u32,fld5: Field::<(i32, [bool; 8], u8)>(Variant(RET, 2), 2).0 };
place!(Field::<u8>(Variant(place!(Field::<Adt39>(Variant(RET, 2), 1)), 0), 3)) = _1;
_8.fld0 = [_20,_20,_20,Field::<i16>(Variant(_26, 3), 4),_20];
place!(Field::<[isize; 3]>(Variant(_37, 0), 0)) = _22.3;
Goto(bb17)
}
bb17 = {
_13 = _4;
place!(Field::<i16>(Variant(_26, 3), 4)) = !_20;
_33 = 63639_u16 as i64;
_28 = _8.fld3 as f64;
_40 = [(-107998211930966617257301405420976181515_i128)];
_3 = _7;
_8.fld1.0 = core::ptr::addr_of!(place!(Field::<char>(Variant(_37, 0), 1)));
place!(Field::<char>(Variant(_26, 3), 1)) = Field::<char>(Variant(_37, 0), 1);
_4 = _3 as isize;
place!(Field::<u8>(Variant(place!(Field::<Adt39>(Variant(RET, 2), 1)), 0), 3)) = Field::<char>(Variant(_26, 3), 1) as u8;
_8.fld2 = 71642395500136215534574312312824952_i128 as usize;
_18 = (_42,);
place!(Field::<i8>(Variant(_26, 3), 3)) = Field::<i8>(Variant(RET, 2), 3) - Field::<i8>(Variant(RET, 2), 3);
_22.3 = [_24,_2,_24];
place!(Field::<([i16; 5],)>(Variant(RET, 2), 0)).0 = _22.0.0;
_8.fld1.0 = core::ptr::addr_of!(_21);
_10 = (Field::<([i16; 5],)>(Variant(RET, 2), 0).0,);
place!(Field::<Adt40>(Variant(_26, 3), 2)) = _8.fld4;
_8.fld3 = 4294844292_u32 as i8;
place!(Field::<u8>(Variant(place!(Field::<Adt39>(Variant(RET, 2), 1)), 0), 3)) = (-168006724206197583427208816992205174050_i128) as u8;
place!(Field::<i8>(Variant(_26, 3), 3)) = _8.fld3;
place!(Field::<*mut u64>(Variant(_26, 3), 6)) = core::ptr::addr_of_mut!(_5);
place!(Field::<u32>(Variant(place!(Field::<Adt39>(Variant(RET, 2), 1)), 0), 4)) = 1555977253_u32 << Field::<isize>(Variant(_37, 0), 2);
place!(Field::<isize>(Variant(place!(Field::<Adt39>(Variant(RET, 2), 1)), 0), 2)) = _2 - Field::<isize>(Variant(_37, 0), 2);
Goto(bb18)
}
bb18 = {
Call(_48 = dump_var(4_usize, 33_usize, Move(_33), 19_usize, Move(_19), 21_usize, Move(_21), 15_usize, Move(_15)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(4_usize, 42_usize, Move(_42), 3_usize, Move(_3), 24_usize, Move(_24), 40_usize, Move(_40)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(4_usize, 20_usize, Move(_20), 35_usize, Move(_35), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_48 = dump_var(4_usize, 2_usize, Move(_2), 27_usize, Move(_27), 49_usize, _49, 49_usize, _49), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [isize; 3],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: char,mut _7: *const char,mut _8: usize,mut _9: [isize; 3]) -> i64 {
mir! {
type RET = i64;
let _10: i32;
let _11: isize;
let _12: f32;
let _13: [i16; 2];
let _14: ([i16; 5],);
let _15: bool;
let _16: Adt51;
let _17: Adt55;
let _18: [i128; 1];
let _19: [u8; 3];
let _20: f32;
let _21: Adt47;
let _22: isize;
let _23: [isize; 3];
let _24: f64;
let _25: char;
let _26: f32;
let _27: [i128; 1];
let _28: [i16; 2];
let _29: f64;
let _30: f32;
let _31: [bool; 8];
let _32: bool;
let _33: [i16; 5];
let _34: u32;
let _35: (u64, i64);
let _36: u128;
let _37: [bool; 8];
let _38: ();
let _39: ();
{
_10 = 1480269997_i32 & 318980783_i32;
Call(_5 = fn6(_1, _3, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 3925137504498358666_i64;
_10 = !(-1783056818_i32);
_8 = 0_usize & 7_usize;
_4 = _10 as isize;
RET = (-2143758952740669243_i64);
_11 = 11701309975362172556_u64 as isize;
_3 = _2 - _2;
_9 = _1;
_6 = '\u{b3165}';
_6 = '\u{2a8ff}';
_2 = _6 as isize;
_9 = _1;
_11 = (-73407368966176059561684052379923218204_i128) as isize;
_8 = !4_usize;
_8 = 4801751924137260434_usize;
Call(_10 = core::intrinsics::bswap((-1606673070_i32)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = (-6166392138595118908_i64);
_5 = _3 >> _3;
_11 = _5 & _5;
_5 = _11 - _3;
_9 = [_5,_5,_5];
_4 = (-19944319922614267630100209912847141666_i128) as isize;
_4 = _11;
_1 = [_5,_5,_5];
_7 = core::ptr::addr_of!(_6);
_5 = -_3;
_1 = [_4,_11,_11];
_7 = core::ptr::addr_of!((*_7));
RET = 25_i8 as i64;
_12 = _10 as f32;
_9 = [_3,_11,_4];
_10 = (-70819434_i32);
match _8 {
4801751924137260434 => bb4,
_ => bb3
}
}
bb3 = {
RET = 3925137504498358666_i64;
_10 = !(-1783056818_i32);
_8 = 0_usize & 7_usize;
_4 = _10 as isize;
RET = (-2143758952740669243_i64);
_11 = 11701309975362172556_u64 as isize;
_3 = _2 - _2;
_9 = _1;
_6 = '\u{b3165}';
_6 = '\u{2a8ff}';
_2 = _6 as isize;
_9 = _1;
_11 = (-73407368966176059561684052379923218204_i128) as isize;
_8 = !4_usize;
_8 = 4801751924137260434_usize;
Call(_10 = core::intrinsics::bswap((-1606673070_i32)), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_10 = RET as i32;
_14.0 = [(-22799_i16),5105_i16,(-10629_i16),(-16603_i16),(-16403_i16)];
_4 = _3;
_11 = _4;
_3 = _6 as isize;
_7 = core::ptr::addr_of!(_6);
_11 = _4 + _5;
_4 = _11;
_13 = [(-6822_i16),(-13770_i16)];
_1 = _9;
_10 = -1895755680_i32;
_3 = _5;
_10 = -1058082992_i32;
_2 = false as isize;
_7 = core::ptr::addr_of!((*_7));
_1 = _9;
_8 = !5_usize;
_13 = [(-24749_i16),(-15158_i16)];
RET = (-1448790960835814632_i64) << _11;
RET = !838937492511230923_i64;
_4 = 24504_i16 as isize;
Goto(bb5)
}
bb5 = {
_10 = (-34293355_i32) * 933566381_i32;
Goto(bb6)
}
bb6 = {
_8 = 7_usize;
RET = 3552947843584324485_i64 + (-1827692332128317385_i64);
_2 = -_5;
_15 = !false;
_11 = _3;
_14.0 = [17767_i16,(-27435_i16),(-20673_i16),(-17583_i16),(-5154_i16)];
_3 = _2 ^ _5;
_13 = [(-18340_i16),30698_i16];
_12 = 119186538048046895082536983393593048764_u128 as f32;
_5 = _3 ^ _3;
_3 = _5;
_9 = [_11,_3,_5];
RET = 13595843043474661003369704994772297816_i128 as i64;
_1 = [_3,_3,_3];
_14.0 = [11007_i16,32638_i16,(-21706_i16),(-3916_i16),(-29297_i16)];
Goto(bb7)
}
bb7 = {
_3 = _11 | _5;
_10 = _15 as i32;
_18 = [118338960895788285664834974551994248353_i128];
Call(_10 = fn7(_11, _3, _5, _3, _9), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12 = 14772319727204977062_u64 as f32;
_11 = _10 as isize;
_10 = (-159588607_i32) >> _3;
_19 = [42_u8,79_u8,151_u8];
_5 = _2 | _3;
_18 = [(-19741476565979708859506862537082271957_i128)];
_3 = _11;
_6 = '\u{f4b10}';
_18 = [22728620771823884877226968838829164991_i128];
_22 = 26685_i16 as isize;
Goto(bb9)
}
bb9 = {
_6 = '\u{2932b}';
_23 = [_5,_11,_5];
_10 = 177639864_i32;
_20 = (-98_i8) as f32;
_15 = _11 == _5;
_15 = !true;
_11 = -_5;
match _10 {
0 => bb1,
1 => bb8,
2 => bb6,
177639864 => bb10,
_ => bb5
}
}
bb10 = {
_22 = _11 >> _11;
_24 = 17979105308741508379_u64 as f64;
_11 = _24 as isize;
RET = -3257542837956966101_i64;
_13 = [30873_i16,(-10440_i16)];
_12 = -_20;
_3 = _24 as isize;
_9 = [_22,_22,_22];
_12 = _20 + _20;
_18 = [(-58633151207142669371176298955018149023_i128)];
_5 = _22 + _22;
_25 = (*_7);
_26 = _12 + _12;
_8 = 6_usize + 7_usize;
_14.0 = [(-26965_i16),(-3008_i16),(-992_i16),(-20309_i16),11290_i16];
_14.0 = [(-24207_i16),22605_i16,(-21485_i16),10497_i16,3372_i16];
_23 = [_5,_22,_22];
_8 = 5_usize - 4_usize;
_13 = [(-24877_i16),(-23015_i16)];
_15 = _3 <= _5;
_6 = _25;
_18 = [(-387678326702130918265272937233698410_i128)];
_5 = _22;
_20 = _12;
_29 = _24 + _24;
_20 = -_26;
Goto(bb11)
}
bb11 = {
_18 = [(-143157173811124241679640712883937749995_i128)];
_5 = _22;
_8 = 7871459053106038087_usize;
_28 = [(-11525_i16),(-32147_i16)];
_31 = [_15,_15,_15,_15,_15,_15,_15,_15];
_27 = [68790710749722978003724141106959242946_i128];
_18 = [(-48324468154380251524155734606317479672_i128)];
_6 = _25;
_14.0 = [(-32162_i16),(-31275_i16),(-29118_i16),(-5078_i16),26473_i16];
_15 = !true;
Call(_16 = fn9(_5, _9, _22, _22, _5, _9), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_11 = -_5;
place!(Field::<(i32, [bool; 8], u8)>(Variant(_16, 0), 1)).2 = 25_u8 & 219_u8;
_30 = -_12;
_32 = !_15;
_6 = _25;
_6 = _25;
place!(Field::<(i32, [bool; 8], u8)>(Variant(_16, 0), 1)) = (_10, _31, 181_u8);
place!(Field::<bool>(Variant(_16, 0), 0)) = Field::<(i32, [bool; 8], u8)>(Variant(_16, 0), 1).2 != Field::<(i32, [bool; 8], u8)>(Variant(_16, 0), 1).2;
_33 = [27400_i16,(-3216_i16),22122_i16,(-15146_i16),26206_i16];
place!(Field::<(i32, [bool; 8], u8)>(Variant(_16, 0), 1)).0 = Field::<(i32, [bool; 8], u8)>(Variant(_16, 0), 1).2 as i32;
_28 = [(-1293_i16),27012_i16];
_24 = 30487_i16 as f64;
place!(Field::<(i32, [bool; 8], u8)>(Variant(_16, 0), 1)).2 = !232_u8;
_7 = core::ptr::addr_of!((*_7));
_18 = _27;
_10 = Field::<(i32, [bool; 8], u8)>(Variant(_16, 0), 1).0;
SetDiscriminant(_16, 1);
_12 = -_30;
_2 = _11 | _5;
_31 = [_15,_32,_15,_32,_32,_32,_15,_32];
match _8 {
0 => bb11,
1 => bb3,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
7871459053106038087 => bb19,
_ => bb18
}
}
bb13 = {
_18 = [(-143157173811124241679640712883937749995_i128)];
_5 = _22;
_8 = 7871459053106038087_usize;
_28 = [(-11525_i16),(-32147_i16)];
_31 = [_15,_15,_15,_15,_15,_15,_15,_15];
_27 = [68790710749722978003724141106959242946_i128];
_18 = [(-48324468154380251524155734606317479672_i128)];
_6 = _25;
_14.0 = [(-32162_i16),(-31275_i16),(-29118_i16),(-5078_i16),26473_i16];
_15 = !true;
Call(_16 = fn9(_5, _9, _22, _22, _5, _9), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_10 = RET as i32;
_14.0 = [(-22799_i16),5105_i16,(-10629_i16),(-16603_i16),(-16403_i16)];
_4 = _3;
_11 = _4;
_3 = _6 as isize;
_7 = core::ptr::addr_of!(_6);
_11 = _4 + _5;
_4 = _11;
_13 = [(-6822_i16),(-13770_i16)];
_1 = _9;
_10 = -1895755680_i32;
_3 = _5;
_10 = -1058082992_i32;
_2 = false as isize;
_7 = core::ptr::addr_of!((*_7));
_1 = _9;
_8 = !5_usize;
_13 = [(-24749_i16),(-15158_i16)];
RET = (-1448790960835814632_i64) << _11;
RET = !838937492511230923_i64;
_4 = 24504_i16 as isize;
Goto(bb5)
}
bb15 = {
_10 = (-34293355_i32) * 933566381_i32;
Goto(bb6)
}
bb16 = {
_12 = 14772319727204977062_u64 as f32;
_11 = _10 as isize;
_10 = (-159588607_i32) >> _3;
_19 = [42_u8,79_u8,151_u8];
_5 = _2 | _3;
_18 = [(-19741476565979708859506862537082271957_i128)];
_3 = _11;
_6 = '\u{f4b10}';
_18 = [22728620771823884877226968838829164991_i128];
_22 = 26685_i16 as isize;
Goto(bb9)
}
bb17 = {
_3 = _11 | _5;
_10 = _15 as i32;
_18 = [118338960895788285664834974551994248353_i128];
Call(_10 = fn7(_11, _3, _5, _3, _9), ReturnTo(bb8), UnwindUnreachable())
}
bb18 = {
RET = (-6166392138595118908_i64);
_5 = _3 >> _3;
_11 = _5 & _5;
_5 = _11 - _3;
_9 = [_5,_5,_5];
_4 = (-19944319922614267630100209912847141666_i128) as isize;
_4 = _11;
_1 = [_5,_5,_5];
_7 = core::ptr::addr_of!(_6);
_5 = -_3;
_1 = [_4,_11,_11];
_7 = core::ptr::addr_of!((*_7));
RET = 25_i8 as i64;
_12 = _10 as f32;
_9 = [_3,_11,_4];
_10 = (-70819434_i32);
match _8 {
4801751924137260434 => bb4,
_ => bb3
}
}
bb19 = {
RET = _10 as i64;
_6 = _25;
_34 = 3963713199_u32 & 4254269113_u32;
_4 = _5 | _11;
Goto(bb20)
}
bb20 = {
Call(_38 = dump_var(5_usize, 11_usize, Move(_11), 22_usize, Move(_22), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_38 = dump_var(5_usize, 1_usize, Move(_1), 8_usize, Move(_8), 10_usize, Move(_10), 33_usize, Move(_33)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_38 = dump_var(5_usize, 2_usize, Move(_2), 4_usize, Move(_4), 19_usize, Move(_19), 5_usize, Move(_5)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [isize; 3],mut _2: isize,mut _3: isize) -> isize {
mir! {
type RET = isize;
let _4: char;
let _5: (i32, [bool; 8], u8);
let _6: [i16; 2];
let _7: bool;
let _8: i64;
let _9: f64;
let _10: isize;
let _11: [i16; 5];
let _12: [bool; 8];
let _13: u128;
let _14: [u8; 3];
let _15: u128;
let _16: Adt44;
let _17: *mut i64;
let _18: Adt46;
let _19: bool;
let _20: Adt47;
let _21: Adt39;
let _22: isize;
let _23: f64;
let _24: i32;
let _25: [bool; 8];
let _26: [i16; 5];
let _27: f64;
let _28: [u8; 3];
let _29: u128;
let _30: ();
let _31: ();
{
_3 = -_2;
RET = _3 ^ _3;
RET = 318656865_u32 as isize;
_1 = [RET,_3,_3];
_2 = _3 & RET;
_1 = [_2,_3,_3];
RET = _3 | _2;
_3 = !RET;
RET = _3;
RET = _2;
_4 = '\u{4921d}';
RET = _2;
RET = _3;
RET = !_3;
_1 = [_3,_3,RET];
_2 = 11497426666072969548_u64 as isize;
Goto(bb1)
}
bb1 = {
RET = _3 >> _3;
_1 = [RET,RET,RET];
_1 = [_2,RET,_3];
_5.2 = 30_u8 >> _3;
RET = (-20615_i16) as isize;
RET = 7727795573686640068_usize as isize;
_2 = _5.2 as isize;
_6 = [(-19038_i16),(-4693_i16)];
_6 = [24365_i16,23564_i16];
_4 = '\u{6befc}';
_3 = _2 * _2;
RET = _2;
_5.1 = [true,true,false,true,true,false,false,true];
RET = 2848869771_u32 as isize;
_7 = !false;
_6 = [(-25091_i16),(-20002_i16)];
_6 = [(-14300_i16),22091_i16];
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_10 = !_3;
_3 = _2;
_7 = _10 < _3;
Goto(bb2)
}
bb2 = {
_10 = !_2;
_5.2 = 217_u8;
_5 = (19011006_i32, _12, 201_u8);
_5 = (1077429435_i32, _12, 28_u8);
_7 = _3 == _10;
_13 = 28384_i16 as u128;
_8 = -7565862587143451008_i64;
RET = -_2;
_7 = true;
_5.2 = _4 as u8;
_11 = [(-20854_i16),(-2504_i16),348_i16,(-8156_i16),5498_i16];
_14 = [_5.2,_5.2,_5.2];
_13 = 34903196566844410004273802742695151178_u128;
_4 = '\u{c7660}';
_9 = 10303267551054809016_u64 as f64;
_5.0 = (-580929221_i32) ^ 81283577_i32;
_9 = 16747_i16 as f64;
_2 = (-11_i8) as isize;
_5.0 = (-519949383_i32);
match _5.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431248262073 => bb7,
_ => bb6
}
}
bb3 = {
RET = _3 >> _3;
_1 = [RET,RET,RET];
_1 = [_2,RET,_3];
_5.2 = 30_u8 >> _3;
RET = (-20615_i16) as isize;
RET = 7727795573686640068_usize as isize;
_2 = _5.2 as isize;
_6 = [(-19038_i16),(-4693_i16)];
_6 = [24365_i16,23564_i16];
_4 = '\u{6befc}';
_3 = _2 * _2;
RET = _2;
_5.1 = [true,true,false,true,true,false,false,true];
RET = 2848869771_u32 as isize;
_7 = !false;
_6 = [(-25091_i16),(-20002_i16)];
_6 = [(-14300_i16),22091_i16];
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_10 = !_3;
_3 = _2;
_7 = _10 < _3;
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
_3 = RET;
_10 = _9 as isize;
_2 = -_3;
_1 = [_2,_2,_2];
_11 = [2893_i16,(-4644_i16),(-2559_i16),(-31428_i16),11582_i16];
_1 = [_2,RET,_2];
_16 = Adt44::Variant1 { fld0: 1_usize };
_7 = false;
_17 = core::ptr::addr_of_mut!(_8);
RET = -_2;
_11 = [(-31870_i16),6751_i16,18641_i16,26733_i16,(-18386_i16)];
RET = _3;
_15 = _13 & _13;
_9 = (-110_i8) as f64;
_2 = _10;
Goto(bb8)
}
bb8 = {
place!(Field::<usize>(Variant(_16, 1), 0)) = 6468552930542451872_usize;
_5.2 = 178_u8;
_10 = -RET;
_15 = _13;
_18 = Adt46::Variant1 { fld0: 162474373487809880702941011750218491886_i128 };
place!(Field::<i128>(Variant(_18, 1), 0)) = (-160061246806560856036405455183698869854_i128) ^ (-135571988482631077443875435489657884952_i128);
_24 = _5.0 * _5.0;
_14 = [_5.2,_5.2,_5.2];
_21 = Adt39::Variant0 { fld0: _1,fld1: _4,fld2: RET,fld3: _5.2,fld4: 3482901809_u32,fld5: _24 };
_10 = -_3;
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_2 = _9 as isize;
_13 = !_15;
place!(Field::<i128>(Variant(_18, 1), 0)) = _8 as i128;
match _5.2 {
0 => bb6,
1 => bb2,
2 => bb4,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
178 => bb14,
_ => bb13
}
}
bb9 = {
_3 = RET;
_10 = _9 as isize;
_2 = -_3;
_1 = [_2,_2,_2];
_11 = [2893_i16,(-4644_i16),(-2559_i16),(-31428_i16),11582_i16];
_1 = [_2,RET,_2];
_16 = Adt44::Variant1 { fld0: 1_usize };
_7 = false;
_17 = core::ptr::addr_of_mut!(_8);
RET = -_2;
_11 = [(-31870_i16),6751_i16,18641_i16,26733_i16,(-18386_i16)];
RET = _3;
_15 = _13 & _13;
_9 = (-110_i8) as f64;
_2 = _10;
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
_10 = !_2;
_5.2 = 217_u8;
_5 = (19011006_i32, _12, 201_u8);
_5 = (1077429435_i32, _12, 28_u8);
_7 = _3 == _10;
_13 = 28384_i16 as u128;
_8 = -7565862587143451008_i64;
RET = -_2;
_7 = true;
_5.2 = _4 as u8;
_11 = [(-20854_i16),(-2504_i16),348_i16,(-8156_i16),5498_i16];
_14 = [_5.2,_5.2,_5.2];
_13 = 34903196566844410004273802742695151178_u128;
_4 = '\u{c7660}';
_9 = 10303267551054809016_u64 as f64;
_5.0 = (-580929221_i32) ^ 81283577_i32;
_9 = 16747_i16 as f64;
_2 = (-11_i8) as isize;
_5.0 = (-519949383_i32);
match _5.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431248262073 => bb7,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
RET = _3 >> _3;
_1 = [RET,RET,RET];
_1 = [_2,RET,_3];
_5.2 = 30_u8 >> _3;
RET = (-20615_i16) as isize;
RET = 7727795573686640068_usize as isize;
_2 = _5.2 as isize;
_6 = [(-19038_i16),(-4693_i16)];
_6 = [24365_i16,23564_i16];
_4 = '\u{6befc}';
_3 = _2 * _2;
RET = _2;
_5.1 = [true,true,false,true,true,false,false,true];
RET = 2848869771_u32 as isize;
_7 = !false;
_6 = [(-25091_i16),(-20002_i16)];
_6 = [(-14300_i16),22091_i16];
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_10 = !_3;
_3 = _2;
_7 = _10 < _3;
Goto(bb2)
}
bb14 = {
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
place!(Field::<i128>(Variant(_18, 1), 0)) = 53661862269651123005723598748417726790_i128;
_3 = RET;
_26 = _11;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(6_usize, 13_usize, Move(_13), 12_usize, Move(_12), 14_usize, Move(_14), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(6_usize, 26_usize, Move(_26), 4_usize, Move(_4), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [isize; 3]) -> i32 {
mir! {
type RET = i32;
let _6: bool;
let _7: [bool; 8];
let _8: ();
let _9: ();
{
RET = 64_u8 as i32;
_5 = [_2,_2,_2];
Call(_5 = fn8(_3, _2, _2, _2, _4, _3, _2, _2, _4, _4, _2, _4, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 313814575_i32;
RET = 1179221747_i32 * (-2003306129_i32);
_5 = [_4,_2,_3];
RET = 2084550995_u32 as i32;
_3 = _2 ^ _4;
_3 = _1;
_6 = false;
_2 = (-44479823703384652290626775095214418198_i128) as isize;
_5 = [_4,_3,_1];
_3 = _4;
_1 = (-1118329844108524498_i64) as isize;
_6 = _3 > _4;
_2 = _3;
_4 = _2;
_4 = _2 & _3;
_2 = _4 - _3;
_2 = -_4;
_2 = _3 & _4;
RET = -(-661681504_i32);
_3 = -_2;
RET = (-251958707_i32) >> _3;
Goto(bb2)
}
bb2 = {
Call(_8 = dump_var(7_usize, 1_usize, Move(_1), 3_usize, Move(_3), 6_usize, Move(_6), 9_usize, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize) -> [isize; 3] {
mir! {
type RET = [isize; 3];
let _14: i64;
let _15: char;
let _16: ();
let _17: ();
{
_12 = 6300418798614171981_u64 as isize;
RET = [_13,_5,_4];
_2 = _10;
_2 = _4;
_1 = 3272937139746944353_i64 as isize;
_8 = 62470_u16 as isize;
_9 = true as isize;
_3 = !_5;
_1 = _7;
_7 = _11 + _10;
_8 = !_2;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(8_usize, 10_usize, Move(_10), 13_usize, Move(_13), 2_usize, Move(_2), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(8_usize, 8_usize, Move(_8), 11_usize, Move(_11), 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: [isize; 3],mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [isize; 3]) -> Adt51 {
mir! {
type RET = Adt51;
let _7: ([i16; 5],);
let _8: isize;
let _9: isize;
let _10: ([i16; 5],);
let _11: [i16; 5];
let _12: [i16; 2];
let _13: isize;
let _14: [i16; 2];
let _15: ([i16; 5],);
let _16: ([i16; 5],);
let _17: i8;
let _18: char;
let _19: (i32, [bool; 8], u8);
let _20: u128;
let _21: [i16; 5];
let _22: f32;
let _23: i8;
let _24: f32;
let _25: ([i16; 5],);
let _26: (([i16; 5],), *mut *mut i64, [isize; 3], [isize; 3]);
let _27: char;
let _28: [isize; 3];
let _29: (i32, [bool; 8], u8);
let _30: isize;
let _31: [u8; 3];
let _32: [i16; 5];
let _33: bool;
let _34: u32;
let _35: [isize; 3];
let _36: u16;
let _37: u16;
let _38: isize;
let _39: i8;
let _40: [i16; 5];
let _41: ();
let _42: ();
{
_4 = (-51_i8) as isize;
_4 = !_5;
_2 = [_5,_4,_4];
_6 = [_5,_3,_1];
_3 = _1 * _5;
_3 = _5 * _1;
_1 = -_3;
_1 = _3;
_5 = -_3;
_4 = _1;
_5 = -_1;
_2 = [_4,_4,_1];
_2 = [_4,_4,_5];
_1 = _5 | _3;
_5 = _3 + _4;
_6 = [_5,_3,_1];
_3 = _4;
_5 = _4;
_1 = !_3;
_7.0 = [8423_i16,(-24_i16),(-6840_i16),(-18580_i16),32614_i16];
_3 = (-25_i8) as isize;
_5 = _1;
_5 = '\u{92943}' as isize;
_3 = _1 & _4;
Goto(bb1)
}
bb1 = {
_1 = _4;
_4 = _3 ^ _3;
_3 = _1;
_9 = 13589349512845475216_u64 as isize;
_1 = _4;
_2 = _6;
_4 = 55533_u16 as isize;
_4 = _1;
Goto(bb2)
}
bb2 = {
_7.0 = [24791_i16,7074_i16,29680_i16,8011_i16,22484_i16];
_10.0 = _7.0;
_10.0 = [(-5244_i16),(-10713_i16),(-29032_i16),26596_i16,15570_i16];
_6 = [_4,_4,_4];
Call(_1 = fn10(_4, _3, _6, _6, _3, _9, _3, _6, _3, _6, _6, _4, _3, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = 34_i8 as isize;
_4 = _3 ^ _1;
_6 = [_3,_3,_4];
_10 = (_7.0,);
_3 = 13282916170518461046_usize as isize;
_4 = (-73890961255499643499533697933187199173_i128) as isize;
_6 = [_1,_1,_1];
_11 = _7.0;
_4 = -_1;
_12 = [31301_i16,8259_i16];
_8 = _4 + _4;
_11 = [(-5223_i16),27182_i16,3511_i16,25133_i16,(-6498_i16)];
_7.0 = _11;
_5 = (-56678401506530850522267035729644859137_i128) as isize;
_2 = [_8,_4,_1];
_10 = _7;
_9 = -_1;
_9 = -_4;
_8 = -_9;
_12 = [(-26015_i16),4442_i16];
_1 = 39118_u16 as isize;
_10.0 = [7054_i16,19817_i16,(-31385_i16),29904_i16,(-4946_i16)];
_9 = !_4;
_1 = _4 & _8;
_10 = _7;
_7.0 = [14100_i16,16104_i16,21804_i16,(-5340_i16),24243_i16];
_13 = 13332_i16 as isize;
_4 = _9 << _13;
Call(_6 = fn12(_9, _9, _4, _9, _8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = [_8,_1,_8];
_14 = [(-6513_i16),14183_i16];
_8 = _9 - _4;
_7 = (_11,);
_10 = (_11,);
_13 = _1 + _1;
_2 = [_8,_4,_9];
_12 = [(-17928_i16),(-10063_i16)];
_1 = _4 >> _4;
_2 = [_9,_13,_4];
_10 = (_7.0,);
_5 = !_13;
_4 = -_1;
_7 = (_11,);
_8 = 40_i8 as isize;
_4 = _1 << _5;
_9 = !_5;
_7 = (_11,);
_8 = _1;
Call(_13 = fn13(_4, _8, _9, _1, _6, _9, _1, _9, _6, _4, _1, _5, _9, _4, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = _8;
_6 = [_9,_13,_9];
_3 = !_13;
_6 = [_9,_13,_5];
_14 = [29413_i16,(-27443_i16)];
_10 = (_11,);
_19.0 = 28891947709676181394055820639732939563_i128 as i32;
_10 = (_11,);
_15.0 = _11;
_19.0 = (-489554355_i32) & (-508574429_i32);
_17 = (-65_i8);
_16 = _7;
_4 = !_9;
_15.0 = _7.0;
_12 = [23858_i16,(-32375_i16)];
_18 = '\u{b099f}';
_17 = (-38_i8) << _1;
Goto(bb6)
}
bb6 = {
_23 = !_17;
_21 = _10.0;
_13 = _4;
_14 = [(-26208_i16),30618_i16];
_19.0 = 764311790_i32 & 1314505287_i32;
_20 = 57491640354399395725048654010032722096_u128 | 231364899459984856890078152679891065931_u128;
_19.0 = 253_u8 as i32;
_21 = [31345_i16,(-17849_i16),4037_i16,(-13402_i16),(-6589_i16)];
_22 = 2153671819743799355_usize as f32;
_24 = -_22;
_23 = _17;
_25.0 = _21;
_11 = [(-10633_i16),563_i16,(-1338_i16),28493_i16,11464_i16];
Goto(bb7)
}
bb7 = {
_19.0 = _20 as i32;
_4 = _3;
_26.0 = (_15.0,);
_19.2 = 54_u8 + 250_u8;
_12 = [21410_i16,(-13365_i16)];
_22 = _24;
_17 = _23 & _23;
_16.0 = [15070_i16,(-18776_i16),13666_i16,20159_i16,(-8195_i16)];
_7.0 = [(-12232_i16),8088_i16,30635_i16,26977_i16,(-22110_i16)];
_26.0.0 = [24216_i16,31459_i16,(-22289_i16),27107_i16,(-32026_i16)];
_27 = _18;
_15 = _7;
_15.0 = [(-26019_i16),1095_i16,(-1709_i16),(-4181_i16),25575_i16];
Goto(bb8)
}
bb8 = {
_6 = _2;
_28 = _6;
_8 = !_4;
_9 = _18 as isize;
_25 = (_26.0.0,);
_30 = _4 + _5;
_3 = !_8;
_29.0 = !_19.0;
_15.0 = _21;
_25 = _15;
_15 = (_26.0.0,);
_10.0 = [(-13853_i16),8000_i16,497_i16,10542_i16,(-20774_i16)];
Goto(bb9)
}
bb9 = {
_30 = _3;
_19.1 = [true,true,true,true,true,false,false,true];
_15.0 = [9131_i16,(-31028_i16),6373_i16,3335_i16,(-6191_i16)];
_13 = 83965811362393765369202813573077799591_i128 as isize;
_25.0 = _16.0;
_29 = (_19.0, _19.1, _19.2);
_28 = _6;
_3 = _17 as isize;
_3 = !_8;
RET = Adt51::Variant0 { fld0: false,fld1: _29,fld2: (-3988833414882582344_i64) };
_31 = [Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1).2,_19.2,_19.2];
place!(Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1)) = (_29.0, _29.1, _29.2);
_7 = _10;
_14 = _12;
_33 = !true;
_29.2 = Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1).2 << _1;
place!(Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1)).0 = 1480192187462737835_u64 as i32;
_25.0 = [(-21147_i16),13594_i16,12367_i16,(-6141_i16),26567_i16];
_17 = -_23;
Goto(bb10)
}
bb10 = {
_27 = _18;
Goto(bb11)
}
bb11 = {
_19.2 = _29.2;
_8 = _4 | _30;
_23 = _17;
_19.0 = _29.0 >> _19.2;
place!(Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1)).0 = 3549613751_u32 as i32;
_14 = [457_i16,6300_i16];
place!(Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1)).2 = 94846430074745441037494029339215318525_i128 as u8;
_29.1 = [_33,_33,_33,_33,_33,_33,_33,_33];
_36 = _3 as u16;
place!(Field::<bool>(Variant(RET, 0), 0)) = _19.0 == _19.0;
_23 = -_17;
_34 = !559988842_u32;
_26.0 = (_7.0,);
_19.1 = Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1).1;
Goto(bb12)
}
bb12 = {
_11 = [(-16380_i16),8122_i16,315_i16,(-4903_i16),12693_i16];
_12 = [(-16328_i16),(-24493_i16)];
_19.0 = _29.0 << _29.2;
_25 = _26.0;
_9 = !_4;
_13 = _24 as isize;
place!(Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1)).2 = _19.2 & _29.2;
_10 = _25;
_22 = -_24;
Call(_10 = fn14(_29.2, _5, _4, _26.0.0, _19.2, _3, _36), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_32 = [23086_i16,(-10920_i16),26600_i16,(-9016_i16),32742_i16];
place!(Field::<i64>(Variant(RET, 0), 2)) = 2032017416584230495_i64 * 5256698264999351835_i64;
_29.2 = Field::<(i32, [bool; 8], u8)>(Variant(RET, 0), 1).2;
place!(Field::<bool>(Variant(RET, 0), 0)) = _33;
Goto(bb14)
}
bb14 = {
Call(_41 = dump_var(9_usize, 23_usize, Move(_23), 27_usize, Move(_27), 31_usize, Move(_31), 5_usize, Move(_5)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_41 = dump_var(9_usize, 4_usize, Move(_4), 30_usize, Move(_30), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(9_usize, 21_usize, Move(_21), 29_usize, Move(_29), 36_usize, Move(_36), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(9_usize, 7_usize, Move(_7), 17_usize, Move(_17), 25_usize, Move(_25), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: [isize; 3],mut _4: [isize; 3],mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [isize; 3],mut _9: isize,mut _10: [isize; 3],mut _11: [isize; 3],mut _12: isize,mut _13: isize,mut _14: isize) -> isize {
mir! {
type RET = isize;
let _15: char;
let _16: [bool; 8];
let _17: char;
let _18: isize;
let _19: [u8; 3];
let _20: ();
let _21: ();
{
_6 = _12 ^ _2;
_13 = !_1;
_8 = _3;
_9 = !_6;
_12 = _13;
_2 = _14;
_15 = '\u{64b63}';
_11 = [_12,_5,_14];
Call(RET = fn11(_3, _14, _7, _2, _6, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_16 = [true,false,true,false,false,true,true,false];
_13 = true as isize;
_5 = true as isize;
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(10_usize, 10_usize, Move(_10), 13_usize, Move(_13), 14_usize, Move(_14), 11_usize, Move(_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(10_usize, 15_usize, Move(_15), 16_usize, Move(_16), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [isize; 3],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> isize {
mir! {
type RET = isize;
let _7: u16;
let _8: Adt45;
let _9: [i16; 2];
let _10: u128;
let _11: u128;
let _12: ((u16, *mut *mut i64), [bool; 8], bool, [i128; 1], ([i16; 5],));
let _13: isize;
let _14: [isize; 3];
let _15: char;
let _16: usize;
let _17: ();
let _18: ();
{
_2 = 389457718_u32 as isize;
_4 = _5 << _5;
_7 = !41284_u16;
_9 = [4952_i16,(-7115_i16)];
_8.fld0 = core::ptr::addr_of_mut!(_8.fld2);
Goto(bb1)
}
bb1 = {
_9 = [19718_i16,(-19545_i16)];
_2 = _5 * _6;
_10 = 206181588353374628097729185783061794339_u128 - 329206685317753455552624883672504762309_u128;
Goto(bb2)
}
bb2 = {
_8.fld1 = '\u{1078df}';
Goto(bb3)
}
bb3 = {
RET = -_3;
_12.2 = !true;
_14 = _1;
_3 = _4 + _4;
Goto(bb4)
}
bb4 = {
Call(_17 = dump_var(11_usize, 10_usize, Move(_10), 9_usize, Move(_9), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_17 = dump_var(11_usize, 2_usize, Move(_2), 18_usize, _18, 18_usize, _18, 18_usize, _18), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize) -> [isize; 3] {
mir! {
type RET = [isize; 3];
let _6: Adt44;
let _7: ();
let _8: ();
{
_2 = 3841203709_u32 as isize;
_4 = 7206312478183859828_usize as isize;
RET = [_1,_3,_1];
_3 = _1 >> _5;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(12_usize, 1_usize, Move(_1), 5_usize, Move(_5), 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [isize; 3],mut _6: isize,mut _7: isize,mut _8: isize,mut _9: [isize; 3],mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize) -> isize {
mir! {
type RET = isize;
let _16: f32;
let _17: ();
let _18: ();
{
_4 = -_12;
RET = _3;
_12 = '\u{8114f}' as isize;
_12 = !RET;
_2 = _4 ^ _6;
_12 = _11;
_1 = _6;
_15 = -_8;
_10 = -_2;
_5 = [_13,_6,_10];
_8 = -_11;
_9 = [_12,_3,_10];
_1 = !_4;
_11 = -_10;
_5 = [_13,_15,_11];
_9 = _5;
_3 = 8603052551096138055_i64 as isize;
RET = -_2;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(13_usize, 13_usize, Move(_13), 1_usize, Move(_1), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(13_usize, 4_usize, Move(_4), 5_usize, Move(_5), 10_usize, Move(_10), 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: u8,mut _2: isize,mut _3: isize,mut _4: [i16; 5],mut _5: u8,mut _6: isize,mut _7: u16) -> ([i16; 5],) {
mir! {
type RET = ([i16; 5],);
let _8: [u8; 3];
let _9: Adt45;
let _10: [isize; 3];
let _11: u16;
let _12: i128;
let _13: bool;
let _14: Adt45;
let _15: f64;
let _16: ([i16; 5],);
let _17: char;
let _18: u128;
let _19: i64;
let _20: Adt43;
let _21: f64;
let _22: isize;
let _23: i64;
let _24: Adt54;
let _25: Adt51;
let _26: u16;
let _27: Adt48;
let _28: f32;
let _29: isize;
let _30: *mut *mut i64;
let _31: isize;
let _32: ([i16; 5],);
let _33: ();
let _34: ();
{
_6 = -_3;
RET = (_4,);
RET = (_4,);
RET = (_4,);
Goto(bb1)
}
bb1 = {
_2 = _3 << _7;
_3 = _7 as isize;
_1 = _5 ^ _5;
RET.0 = [(-28373_i16),(-698_i16),(-17347_i16),(-12105_i16),(-27449_i16)];
_7 = 2532888212_u32 as u16;
_3 = _6;
_1 = _5;
RET.0 = [(-8440_i16),(-32351_i16),(-16404_i16),5999_i16,6684_i16];
_6 = !_2;
_7 = 4_usize as u16;
_2 = _3;
_7 = 57572_u16 - 44681_u16;
_5 = _1;
_6 = (-37232351324953028411101115856188145430_i128) as isize;
RET.0 = _4;
_7 = !33228_u16;
RET.0 = _4;
_3 = _2 << _1;
_2 = -_3;
_6 = _3;
_1 = _5;
RET.0 = [8246_i16,(-27407_i16),(-5749_i16),5287_i16,(-6441_i16)];
_2 = true as isize;
RET.0 = [8962_i16,(-19364_i16),26306_i16,(-1274_i16),17956_i16];
_8 = [_5,_5,_5];
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _5 + _5;
_4 = [4415_i16,25937_i16,(-9535_i16),15479_i16,(-12396_i16)];
_3 = !_6;
RET = (_4,);
_3 = !_6;
_1 = !_5;
_4 = RET.0;
_1 = _5 - _5;
_5 = !_1;
_3 = _6;
_9.fld0 = core::ptr::addr_of_mut!(_9.fld2);
_9.fld0 = core::ptr::addr_of_mut!(_9.fld2);
_10 = [_6,_3,_6];
_4 = [(-6632_i16),25224_i16,102_i16,32711_i16,(-29724_i16)];
RET = (_4,);
_3 = 5801530333280965749_u64 as isize;
_9.fld1 = '\u{7084f}';
_9.fld2 = 26995_i16 as f64;
Call(RET = fn15(_6, _10, _9.fld0, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = (-125_i8) as u16;
_4 = RET.0;
_1 = !_5;
RET.0 = [(-29398_i16),(-12971_i16),31913_i16,(-29152_i16),(-30347_i16)];
_3 = _6 & _6;
Goto(bb4)
}
bb4 = {
_12 = -44456722594167833998290615812905639949_i128;
_9.fld2 = _1 as f64;
_14.fld1 = _9.fld1;
RET.0 = _4;
_12 = _6 as i128;
_9.fld2 = (-1986055456_i32) as f64;
Call(_5 = core::intrinsics::bswap(_1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = (-1241596134323151089_i64) as isize;
_3 = 73839993195374497442058368285570784533_u128 as isize;
_12 = 21143084077602071740663996407789414701_i128 & 82023991125445611700921627098027043806_i128;
_14.fld2 = _9.fld2 - _9.fld2;
RET = (_4,);
_6 = !_3;
_14 = Adt45 { fld0: _9.fld0,fld1: _9.fld1,fld2: _9.fld2 };
Goto(bb6)
}
bb6 = {
_8 = [_1,_5,_1];
_11 = _14.fld1 as u16;
_9 = Move(_14);
RET = (_4,);
RET.0 = [(-12204_i16),(-19770_i16),6143_i16,17327_i16,(-23157_i16)];
_3 = _6;
_3 = _7 as isize;
_14.fld2 = _9.fld2;
_4 = [(-27036_i16),(-346_i16),(-1271_i16),(-4114_i16),(-28292_i16)];
_15 = -_14.fld2;
_14 = Adt45 { fld0: _9.fld0,fld1: _9.fld1,fld2: _9.fld2 };
_6 = !_2;
_13 = true | false;
_14.fld1 = _9.fld1;
_3 = 4_usize as isize;
Goto(bb7)
}
bb7 = {
RET = (_4,);
_14.fld1 = _9.fld1;
_13 = _1 <= _1;
_3 = _6;
_9.fld2 = -_15;
_9 = Adt45 { fld0: _14.fld0,fld1: _14.fld1,fld2: _14.fld2 };
_17 = _14.fld1;
_6 = _3;
_14.fld0 = core::ptr::addr_of_mut!(_9.fld2);
_9.fld0 = core::ptr::addr_of_mut!(_15);
_12 = 4666811176542739203_i64 as i128;
_14.fld1 = _9.fld1;
_12 = 48872963104672744753975334595879157862_i128 & (-19450900627775980113661228785242681420_i128);
_19 = _12 as i64;
_18 = 4292803549_u32 as u128;
_2 = !_6;
_3 = _6 + _6;
_16 = (_4,);
_17 = _9.fld1;
_16 = (RET.0,);
_14.fld1 = _17;
RET = _16;
_2 = 6_usize as isize;
_20.fld5 = _19 as i32;
Goto(bb8)
}
bb8 = {
_3 = _6;
_20.fld0.fld4.fld0 = 758353743_u32 as f32;
_20.fld0.fld2 = 9033750200087406001_usize;
_15 = -_9.fld2;
RET = (_16.0,);
_16 = RET;
_13 = !false;
_6 = !_2;
_20.fld3 = [_12];
_20.fld0.fld1.1 = [_13,_13,_13,_13,_13,_13,_13,_13];
_9.fld1 = _17;
_9.fld0 = _14.fld0;
_20.fld0.fld3 = 115_i8;
_7 = _11 >> _5;
RET = (_16.0,);
_14.fld0 = core::ptr::addr_of_mut!(_9.fld2);
_9.fld0 = core::ptr::addr_of_mut!(_9.fld2);
RET.0 = [(-19304_i16),17703_i16,(-16331_i16),(-5023_i16),13026_i16];
_20.fld0.fld1.0 = core::ptr::addr_of!(_17);
_20.fld1 = [1766_i16,6002_i16];
_8 = [_1,_5,_5];
_20.fld0.fld0 = _16.0;
match _20.fld0.fld2 {
0 => bb9,
1 => bb10,
9033750200087406001 => bb12,
_ => bb11
}
}
bb9 = {
RET = (_4,);
_14.fld1 = _9.fld1;
_13 = _1 <= _1;
_3 = _6;
_9.fld2 = -_15;
_9 = Adt45 { fld0: _14.fld0,fld1: _14.fld1,fld2: _14.fld2 };
_17 = _14.fld1;
_6 = _3;
_14.fld0 = core::ptr::addr_of_mut!(_9.fld2);
_9.fld0 = core::ptr::addr_of_mut!(_15);
_12 = 4666811176542739203_i64 as i128;
_14.fld1 = _9.fld1;
_12 = 48872963104672744753975334595879157862_i128 & (-19450900627775980113661228785242681420_i128);
_19 = _12 as i64;
_18 = 4292803549_u32 as u128;
_2 = !_6;
_3 = _6 + _6;
_16 = (_4,);
_17 = _9.fld1;
_16 = (RET.0,);
_14.fld1 = _17;
RET = _16;
_2 = 6_usize as isize;
_20.fld5 = _19 as i32;
Goto(bb8)
}
bb10 = {
_2 = _3 << _7;
_3 = _7 as isize;
_1 = _5 ^ _5;
RET.0 = [(-28373_i16),(-698_i16),(-17347_i16),(-12105_i16),(-27449_i16)];
_7 = 2532888212_u32 as u16;
_3 = _6;
_1 = _5;
RET.0 = [(-8440_i16),(-32351_i16),(-16404_i16),5999_i16,6684_i16];
_6 = !_2;
_7 = 4_usize as u16;
_2 = _3;
_7 = 57572_u16 - 44681_u16;
_5 = _1;
_6 = (-37232351324953028411101115856188145430_i128) as isize;
RET.0 = _4;
_7 = !33228_u16;
RET.0 = _4;
_3 = _2 << _1;
_2 = -_3;
_6 = _3;
_1 = _5;
RET.0 = [8246_i16,(-27407_i16),(-5749_i16),5287_i16,(-6441_i16)];
_2 = true as isize;
RET.0 = [8962_i16,(-19364_i16),26306_i16,(-1274_i16),17956_i16];
_8 = [_5,_5,_5];
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_3 = (-1241596134323151089_i64) as isize;
_3 = 73839993195374497442058368285570784533_u128 as isize;
_12 = 21143084077602071740663996407789414701_i128 & 82023991125445611700921627098027043806_i128;
_14.fld2 = _9.fld2 - _9.fld2;
RET = (_4,);
_6 = !_3;
_14 = Adt45 { fld0: _9.fld0,fld1: _9.fld1,fld2: _9.fld2 };
Goto(bb6)
}
bb12 = {
_20.fld4 = [_6,_6,_2];
_15 = _20.fld5 as f64;
_20.fld0.fld1.0 = core::ptr::addr_of!(_14.fld1);
_13 = true;
_7 = _3 as u16;
_20.fld4 = [_3,_2,_2];
_16 = (_20.fld0.fld0,);
_14.fld0 = core::ptr::addr_of_mut!(_21);
_13 = _5 != _1;
_17 = _14.fld1;
_20.fld1 = [20270_i16,(-20321_i16)];
_2 = _6 >> _1;
_21 = _12 as f64;
_24 = Adt54::Variant0 { fld0: _7,fld1: _20.fld0.fld2,fld2: _20.fld0.fld1.0,fld3: _12 };
_20.fld0.fld3 = (-48_i8) | (-115_i8);
_19 = _20.fld0.fld3 as i64;
_8 = [_5,_1,_1];
_23 = _15 as i64;
_20.fld0.fld0 = [(-9929_i16),11473_i16,(-28007_i16),21556_i16,(-29981_i16)];
_14.fld2 = _12 as f64;
_13 = false;
_14.fld2 = -_21;
_14.fld0 = _9.fld0;
_14.fld2 = _9.fld2 * _21;
Goto(bb13)
}
bb13 = {
_9.fld2 = _2 as f64;
_19 = -_23;
_14.fld1 = _17;
_14 = Move(_9);
_1 = _5;
_20.fld5 = Field::<u16>(Variant(_24, 0), 0) as i32;
_20.fld0.fld1.0 = Field::<*const char>(Variant(_24, 0), 2);
_9.fld0 = _14.fld0;
_4 = _16.0;
_28 = _20.fld0.fld4.fld0;
RET = (_16.0,);
_18 = _7 as u128;
place!(Field::<*const char>(Variant(_24, 0), 2)) = core::ptr::addr_of!(_14.fld1);
_20.fld5 = 7393_i16 as i32;
_16.0 = RET.0;
_12 = Field::<i128>(Variant(_24, 0), 3) >> _1;
_14.fld0 = _9.fld0;
_20.fld0.fld2 = 2603586548259400966_u64 as usize;
_8 = [_1,_5,_5];
_26 = _18 as u16;
_19 = _23 + _23;
SetDiscriminant(_24, 0);
_10 = _20.fld4;
_19 = _20.fld0.fld3 as i64;
_19 = _23 - _23;
_15 = -_14.fld2;
_22 = _2;
Goto(bb14)
}
bb14 = {
_1 = !_5;
_20.fld0.fld3 = 13_i8;
_26 = _11;
_29 = _2;
_28 = _20.fld0.fld4.fld0 * _20.fld0.fld4.fld0;
_22 = _2 ^ _2;
_20.fld0.fld3 = 4218570875_u32 as i8;
_23 = _12 as i64;
_26 = _11 - _11;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(14_usize, 16_usize, Move(_16), 22_usize, Move(_22), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(14_usize, 13_usize, Move(_13), 4_usize, Move(_4), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(14_usize, 18_usize, Move(_18), 19_usize, Move(_19), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: [isize; 3],mut _3: *mut f64,mut _4: [isize; 3]) -> ([i16; 5],) {
mir! {
type RET = ([i16; 5],);
let _5: [u8; 3];
let _6: i128;
let _7: (i32, [bool; 8], u8);
let _8: [i16; 5];
let _9: Adt46;
let _10: isize;
let _11: Adt41;
let _12: f32;
let _13: (u64, i64);
let _14: [i16; 2];
let _15: [i16; 5];
let _16: [i16; 5];
let _17: Adt53;
let _18: isize;
let _19: isize;
let _20: Adt43;
let _21: bool;
let _22: Adt51;
let _23: i32;
let _24: char;
let _25: f32;
let _26: *mut f64;
let _27: [isize; 3];
let _28: char;
let _29: [bool; 8];
let _30: Adt49;
let _31: bool;
let _32: isize;
let _33: Adt49;
let _34: Adt54;
let _35: [bool; 8];
let _36: i8;
let _37: u32;
let _38: isize;
let _39: (i32, [bool; 8], u8);
let _40: char;
let _41: (i32, [bool; 8], u8);
let _42: Adt46;
let _43: f32;
let _44: f64;
let _45: bool;
let _46: isize;
let _47: isize;
let _48: (i32, [bool; 8], u8);
let _49: ();
let _50: ();
{
_4 = [_1,_1,_1];
RET.0 = [5481_i16,(-4550_i16),(-6488_i16),10811_i16,(-11372_i16)];
Call(RET = fn16(_4, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = [(-14815_i16),3562_i16,25319_i16,(-16190_i16),32461_i16];
_5 = [54_u8,132_u8,195_u8];
_2 = [_1,_1,_1];
_7.2 = 240_u8;
_7.0 = (-410521865_i32);
_2 = [_1,_1,_1];
_7.0 = 66124504_i32 & (-1920378910_i32);
_6 = 43609384285991871529330056727397488107_i128;
RET.0 = [(-10044_i16),(-13997_i16),(-16358_i16),(-18417_i16),(-30934_i16)];
_1 = 9223372036854775807_isize;
_4 = [_1,_1,_1];
_7.2 = !85_u8;
_7.2 = 57_u8 & 29_u8;
_7.1 = [false,true,false,true,false,true,false,false];
_6 = -17812720817383018246114379267756277307_i128;
_1 = 582_i16 as isize;
Call(_1 = core::intrinsics::transmute(_7.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.0 = [12858_i16,18787_i16,1771_i16,(-23110_i16),13793_i16];
_13.0 = 8630164488361208473_u64;
_4 = [_1,_1,_1];
_13.0 = 6205525755114302240_u64 + 12096890327097559436_u64;
RET.0 = [(-16944_i16),(-15925_i16),(-15686_i16),23992_i16,1685_i16];
_13 = (12624496712417744262_u64, (-8238599444812022118_i64));
_10 = _1 >> _13.1;
_6 = !55860747350801687097108331273249494796_i128;
_7.0 = -1313942721_i32;
_10 = -_1;
_4 = _2;
RET.0 = [29449_i16,(-14279_i16),(-18464_i16),(-14071_i16),27184_i16];
_6 = _13.0 as i128;
_12 = _10 as f32;
_6 = -15404610420845151121658826659615174125_i128;
_2 = [_1,_10,_1];
_9 = Adt46::Variant1 { fld0: _6 };
_7.0 = !1756960026_i32;
_15 = [10165_i16,(-11482_i16),17079_i16,(-27625_i16),24566_i16];
_6 = false as i128;
_14 = [(-3582_i16),(-9110_i16)];
_8 = [(-16524_i16),(-26410_i16),(-30156_i16),(-1517_i16),(-11724_i16)];
_1 = true as isize;
_13.1 = 61418_u16 as i64;
match _13.0 {
12624496712417744262 => bb3,
_ => bb1
}
}
bb3 = {
_20.fld1 = [4272_i16,30419_i16];
_8 = [9178_i16,(-23916_i16),(-12061_i16),6917_i16,(-7277_i16)];
_20.fld0.fld4.fld1 = core::ptr::addr_of!(_13.0);
RET.0 = [14147_i16,(-31018_i16),(-30209_i16),(-31604_i16),(-15856_i16)];
_18 = !_10;
_21 = !true;
_20.fld0.fld1.1 = _7.1;
_20.fld0.fld4.fld0 = -_12;
_20.fld5 = _7.0;
_15 = [(-15160_i16),(-9029_i16),(-976_i16),23991_i16,(-11005_i16)];
_13.0 = 7373878523987685623_u64;
_20.fld0.fld2 = !8567573092739753141_usize;
_16 = _8;
RET.0 = [17671_i16,6222_i16,(-11945_i16),7370_i16,25683_i16];
_1 = _10;
_26 = _3;
_23 = _7.0;
_13.1 = (-199342333052975352_i64) & 1039948812556116823_i64;
_20.fld4 = [_1,_18,_1];
_4 = [_18,_18,_1];
_8 = RET.0;
_10 = !_18;
_12 = -_20.fld0.fld4.fld0;
_10 = _1 & _1;
Goto(bb4)
}
bb4 = {
_7.0 = !_20.fld5;
_26 = _3;
RET.0 = [2498_i16,22983_i16,(-30099_i16),(-32480_i16),2362_i16];
_22 = Adt51::Variant0 { fld0: _21,fld1: _7,fld2: _13.1 };
RET.0 = _8;
_3 = _26;
_13.1 = Field::<i64>(Variant(_22, 0), 2);
_20.fld4 = _2;
_7.1 = [Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0),_21,_21,Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0),_21,Field::<bool>(Variant(_22, 0), 0)];
_13 = (14907788376506151100_u64, Field::<i64>(Variant(_22, 0), 2));
_20.fld0.fld1.0 = core::ptr::addr_of!(_28);
_28 = '\u{b6dcc}';
_1 = Field::<bool>(Variant(_22, 0), 0) as isize;
_15 = [(-23976_i16),(-28646_i16),(-7224_i16),(-12586_i16),29911_i16];
_20.fld0.fld0 = [(-7216_i16),(-30707_i16),10428_i16,(-6547_i16),(-15170_i16)];
_20.fld0.fld1.0 = core::ptr::addr_of!(_28);
_20.fld0.fld4.fld1 = core::ptr::addr_of!(_13.0);
Goto(bb5)
}
bb5 = {
place!(Field::<i64>(Variant(_22, 0), 2)) = (-837_i16) as i64;
_25 = _12 * _12;
_28 = '\u{b405a}';
place!(Field::<(i32, [bool; 8], u8)>(Variant(_22, 0), 1)) = (_23, _20.fld0.fld1.1, _7.2);
_16 = [4881_i16,18029_i16,27028_i16,(-25308_i16),12612_i16];
_27 = _20.fld4;
_24 = _28;
RET.0 = [(-27501_i16),(-8857_i16),(-28688_i16),(-24437_i16),14549_i16];
_13 = (17563169433795989239_u64, Field::<i64>(Variant(_22, 0), 2));
place!(Field::<bool>(Variant(_22, 0), 0)) = _25 < _20.fld0.fld4.fld0;
_20.fld3 = [_6];
SetDiscriminant(_9, 1);
_27 = [_10,_10,_1];
_2 = [_10,_18,_1];
_4 = [_10,_10,_10];
_24 = _28;
_29 = [Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0),_21,Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0)];
_5 = [Field::<(i32, [bool; 8], u8)>(Variant(_22, 0), 1).2,Field::<(i32, [bool; 8], u8)>(Variant(_22, 0), 1).2,_7.2];
_25 = 56260_u16 as f32;
_2 = [_10,_10,_10];
_27 = [_10,_10,_10];
_7.0 = -Field::<(i32, [bool; 8], u8)>(Variant(_22, 0), 1).0;
_15 = _8;
_27 = _2;
match _13.0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
17563169433795989239 => bb11,
_ => bb10
}
}
bb6 = {
_7.0 = !_20.fld5;
_26 = _3;
RET.0 = [2498_i16,22983_i16,(-30099_i16),(-32480_i16),2362_i16];
_22 = Adt51::Variant0 { fld0: _21,fld1: _7,fld2: _13.1 };
RET.0 = _8;
_3 = _26;
_13.1 = Field::<i64>(Variant(_22, 0), 2);
_20.fld4 = _2;
_7.1 = [Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0),_21,_21,Field::<bool>(Variant(_22, 0), 0),Field::<bool>(Variant(_22, 0), 0),_21,Field::<bool>(Variant(_22, 0), 0)];
_13 = (14907788376506151100_u64, Field::<i64>(Variant(_22, 0), 2));
_20.fld0.fld1.0 = core::ptr::addr_of!(_28);
_28 = '\u{b6dcc}';
_1 = Field::<bool>(Variant(_22, 0), 0) as isize;
_15 = [(-23976_i16),(-28646_i16),(-7224_i16),(-12586_i16),29911_i16];
_20.fld0.fld0 = [(-7216_i16),(-30707_i16),10428_i16,(-6547_i16),(-15170_i16)];
_20.fld0.fld1.0 = core::ptr::addr_of!(_28);
_20.fld0.fld4.fld1 = core::ptr::addr_of!(_13.0);
Goto(bb5)
}
bb7 = {
_20.fld1 = [4272_i16,30419_i16];
_8 = [9178_i16,(-23916_i16),(-12061_i16),6917_i16,(-7277_i16)];
_20.fld0.fld4.fld1 = core::ptr::addr_of!(_13.0);
RET.0 = [14147_i16,(-31018_i16),(-30209_i16),(-31604_i16),(-15856_i16)];
_18 = !_10;
_21 = !true;
_20.fld0.fld1.1 = _7.1;
_20.fld0.fld4.fld0 = -_12;
_20.fld5 = _7.0;
_15 = [(-15160_i16),(-9029_i16),(-976_i16),23991_i16,(-11005_i16)];
_13.0 = 7373878523987685623_u64;
_20.fld0.fld2 = !8567573092739753141_usize;
_16 = _8;
RET.0 = [17671_i16,6222_i16,(-11945_i16),7370_i16,25683_i16];
_1 = _10;
_26 = _3;
_23 = _7.0;
_13.1 = (-199342333052975352_i64) & 1039948812556116823_i64;
_20.fld4 = [_1,_18,_1];
_4 = [_18,_18,_1];
_8 = RET.0;
_10 = !_18;
_12 = -_20.fld0.fld4.fld0;
_10 = _1 & _1;
Goto(bb4)
}
bb8 = {
RET.0 = [12858_i16,18787_i16,1771_i16,(-23110_i16),13793_i16];
_13.0 = 8630164488361208473_u64;
_4 = [_1,_1,_1];
_13.0 = 6205525755114302240_u64 + 12096890327097559436_u64;
RET.0 = [(-16944_i16),(-15925_i16),(-15686_i16),23992_i16,1685_i16];
_13 = (12624496712417744262_u64, (-8238599444812022118_i64));
_10 = _1 >> _13.1;
_6 = !55860747350801687097108331273249494796_i128;
_7.0 = -1313942721_i32;
_10 = -_1;
_4 = _2;
RET.0 = [29449_i16,(-14279_i16),(-18464_i16),(-14071_i16),27184_i16];
_6 = _13.0 as i128;
_12 = _10 as f32;
_6 = -15404610420845151121658826659615174125_i128;
_2 = [_1,_10,_1];
_9 = Adt46::Variant1 { fld0: _6 };
_7.0 = !1756960026_i32;
_15 = [10165_i16,(-11482_i16),17079_i16,(-27625_i16),24566_i16];
_6 = false as i128;
_14 = [(-3582_i16),(-9110_i16)];
_8 = [(-16524_i16),(-26410_i16),(-30156_i16),(-1517_i16),(-11724_i16)];
_1 = true as isize;
_13.1 = 61418_u16 as i64;
match _13.0 {
12624496712417744262 => bb3,
_ => bb1
}
}
bb9 = {
RET.0 = [(-14815_i16),3562_i16,25319_i16,(-16190_i16),32461_i16];
_5 = [54_u8,132_u8,195_u8];
_2 = [_1,_1,_1];
_7.2 = 240_u8;
_7.0 = (-410521865_i32);
_2 = [_1,_1,_1];
_7.0 = 66124504_i32 & (-1920378910_i32);
_6 = 43609384285991871529330056727397488107_i128;
RET.0 = [(-10044_i16),(-13997_i16),(-16358_i16),(-18417_i16),(-30934_i16)];
_1 = 9223372036854775807_isize;
_4 = [_1,_1,_1];
_7.2 = !85_u8;
_7.2 = 57_u8 & 29_u8;
_7.1 = [false,true,false,true,false,true,false,false];
_6 = -17812720817383018246114379267756277307_i128;
_1 = 582_i16 as isize;
Call(_1 = core::intrinsics::transmute(_7.1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
place!(Field::<(i32, [bool; 8], u8)>(Variant(_22, 0), 1)) = (_23, _29, _7.2);
RET.0 = _8;
_20.fld0.fld1.0 = core::ptr::addr_of!(_28);
_24 = _28;
match _13.0 {
0 => bb6,
1 => bb8,
17563169433795989239 => bb12,
_ => bb5
}
}
bb12 = {
_20.fld1 = [(-18770_i16),32713_i16];
_12 = _20.fld0.fld4.fld0 - _20.fld0.fld4.fld0;
_3 = _26;
SetDiscriminant(_22, 1);
_20.fld0.fld4.fld1 = core::ptr::addr_of!(_13.0);
_20.fld0.fld4.fld0 = _12 + _12;
_5 = [_7.2,_7.2,_7.2];
_14 = _20.fld1;
_3 = _26;
Call(_13 = fn17(_12, _2, _20.fld3, _27, _20.fld4, _20.fld0.fld0, _3, _20.fld0.fld4, _18, _20.fld0.fld1, _20.fld4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = (_16,);
_20.fld4 = _27;
_10 = _18;
place!(Field::<[i16; 2]>(Variant(_22, 1), 7)) = [16504_i16,3803_i16];
_39.2 = _7.2 - _7.2;
_35 = _29;
Goto(bb14)
}
bb14 = {
_39.2 = !_7.2;
_41.2 = 288728902126531603687737460268840485896_u128 as u8;
_40 = _24;
_41.1 = _29;
_40 = _24;
_39 = _7;
_41 = (_39.0, _29, _7.2);
RET.0 = [(-28996_i16),(-6341_i16),8088_i16,(-29104_i16),(-28360_i16)];
_15 = _20.fld0.fld0;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(15_usize, 4_usize, Move(_4), 18_usize, Move(_18), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(15_usize, 14_usize, Move(_14), 8_usize, Move(_8), 40_usize, Move(_40), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(15_usize, 5_usize, Move(_5), 7_usize, Move(_7), 21_usize, Move(_21), 50_usize, _50), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [isize; 3],mut _2: isize) -> ([i16; 5],) {
mir! {
type RET = ([i16; 5],);
let _3: f32;
let _4: [i16; 5];
let _5: u8;
let _6: f64;
let _7: ([i16; 5],);
let _8: [i16; 2];
let _9: *mut u64;
let _10: (i32, [bool; 8], u8);
let _11: [isize; 3];
let _12: f32;
let _13: (u64, i64);
let _14: u16;
let _15: isize;
let _16: u32;
let _17: ((u16, *mut *mut i64), [bool; 8], bool, [i128; 1], ([i16; 5],));
let _18: [u8; 3];
let _19: ([i16; 5],);
let _20: char;
let _21: [u8; 3];
let _22: Adt55;
let _23: isize;
let _24: [i128; 1];
let _25: [i128; 1];
let _26: [i16; 5];
let _27: f32;
let _28: Adt48;
let _29: ();
let _30: ();
{
_2 = (-9223372036854775808_isize);
RET.0 = [30252_i16,18206_i16,11426_i16,19807_i16,(-29933_i16)];
RET.0 = [1771_i16,(-10919_i16),(-32207_i16),27600_i16,19516_i16];
_2 = (-9223372036854775808_isize);
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463454151235394913435648 => bb6,
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
_2 = '\u{56cb9}' as isize;
_2 = !38_isize;
_3 = (-26245_i16) as f32;
_4 = RET.0;
_5 = 225398459_u32 as u8;
_4 = RET.0;
_1 = [_2,_2,_2];
RET = (_4,);
_4 = RET.0;
RET.0 = [(-30157_i16),26266_i16,4507_i16,32000_i16,10642_i16];
RET = (_4,);
_2 = (-128424185_i32) as isize;
_6 = (-4106067607639074414_i64) as f64;
RET = (_4,);
_8 = [(-8734_i16),(-4132_i16)];
_6 = _3 as f64;
RET.0 = [3583_i16,5909_i16,(-15299_i16),(-9575_i16),11517_i16];
_7.0 = [20840_i16,(-30459_i16),(-7802_i16),22165_i16,17484_i16];
RET = (_4,);
Goto(bb7)
}
bb7 = {
RET = (_4,);
_7 = (_4,);
_8 = [16435_i16,18749_i16];
_3 = 4286069587384203941_usize as f32;
Goto(bb8)
}
bb8 = {
_3 = 15030_u16 as f32;
_2 = -9223372036854775807_isize;
RET.0 = [(-7863_i16),26842_i16,11235_i16,6226_i16,18876_i16];
_7 = (_4,);
_6 = 3347630606_u32 as f64;
_7.0 = [28195_i16,25620_i16,(-2728_i16),12707_i16,(-3254_i16)];
_6 = 3307665498_u32 as f64;
_5 = !165_u8;
RET.0 = [20616_i16,31417_i16,524_i16,21518_i16,(-7618_i16)];
_2 = 9223372036854775807_isize + (-9223372036854775808_isize);
_8 = [(-31416_i16),(-26353_i16)];
_6 = 106568465794120237347567221528410947813_u128 as f64;
_6 = (-7374780548166945571_i64) as f64;
_6 = 300259651065951911505408781597536147132_u128 as f64;
_5 = !243_u8;
_2 = 76_isize;
_5 = 135_u8;
_7 = (_4,);
RET = (_7.0,);
_5 = 262115878266855484271018423807995358779_u128 as u8;
_10.0 = (-4467701_i32) + (-1803488728_i32);
_7 = RET;
_2 = 65_isize;
_10.1 = [true,false,true,true,true,false,true,true];
_11 = [_2,_2,_2];
_10.2 = _6 as u8;
Goto(bb9)
}
bb9 = {
RET = (_4,);
_10.2 = !_5;
_10.1 = [false,false,true,false,false,false,false,true];
_1 = [_2,_2,_2];
RET = (_4,);
_12 = -_3;
RET.0 = [17713_i16,(-13761_i16),31513_i16,22373_i16,29227_i16];
_2 = 103_isize;
_11 = [_2,_2,_2];
_7 = RET;
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb6,
103 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
RET.0 = [9624_i16,32168_i16,18111_i16,1666_i16,3171_i16];
RET.0 = [14461_i16,(-10198_i16),13005_i16,(-31431_i16),(-23833_i16)];
_13.1 = !(-5195689051595045183_i64);
RET = _7;
_13.1 = (-6427215095034610006_i64);
RET.0 = [7370_i16,29907_i16,1588_i16,28887_i16,(-8576_i16)];
RET = _7;
_7 = (_4,);
_13.1 = 15092_i16 as i64;
_13 = (15998331604790871810_u64, 6879043557366313841_i64);
_9 = core::ptr::addr_of_mut!(_13.0);
_5 = _10.2 + _10.2;
_14 = 29853_u16;
_13 = (2711388209900015903_u64, (-1880660222057294526_i64));
_7.0 = [(-14312_i16),7345_i16,(-16048_i16),(-11883_i16),2821_i16];
_8 = [6048_i16,(-6111_i16)];
_10.0 = false as i32;
_14 = 25241_u16;
RET.0 = _4;
RET.0 = [6001_i16,(-25120_i16),(-9386_i16),(-24058_i16),14014_i16];
RET.0 = [801_i16,31790_i16,5691_i16,(-24749_i16),31450_i16];
_13 = (8297577294710410685_u64, (-9109967320742798023_i64));
_5 = _10.2 << _10.2;
_13 = (8418546402304079644_u64, (-2458779621447601226_i64));
_8 = [(-9052_i16),(-18614_i16)];
_2 = !(-9223372036854775808_isize);
_12 = _3 - _3;
_11 = [_2,_2,_2];
_5 = _10.2;
RET.0 = [22040_i16,(-26156_i16),21882_i16,(-4665_i16),15573_i16];
Call(_2 = core::intrinsics::bswap(38_isize), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_3 = _12 - _12;
RET = (_7.0,);
_14 = 37005_u16;
_18 = [_5,_5,_10.2];
_19.0 = RET.0;
_17.2 = false;
_16 = !3682696401_u32;
_18 = [_10.2,_10.2,_10.2];
_12 = _3;
_17.2 = false;
RET = _7;
RET = (_4,);
RET = (_7.0,);
RET.0 = [(-13309_i16),10386_i16,(-2662_i16),(-12633_i16),(-20311_i16)];
_1 = [_2,_2,_2];
RET = (_4,);
_19 = _7;
_17.0.0 = _14;
RET.0 = [(-19810_i16),(-8912_i16),9442_i16,(-5308_i16),(-16902_i16)];
_17.4 = (RET.0,);
_8 = [13015_i16,831_i16];
_17.2 = _3 < _12;
_1 = [_2,_2,_2];
Goto(bb13)
}
bb13 = {
_4 = [3967_i16,(-32008_i16),(-23781_i16),27487_i16,13587_i16];
_5 = !_10.2;
RET.0 = _17.4.0;
RET = (_7.0,);
_11 = [_2,_2,_2];
_21 = [_10.2,_10.2,_10.2];
_23 = _2 ^ _2;
_24 = [107097428743914028975115049869866338143_i128];
Goto(bb14)
}
bb14 = {
_25 = [(-143348883838099683897315097874585074430_i128)];
_10.1 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_27 = _12;
_25 = _24;
_10.2 = _5;
RET = (_17.4.0,);
RET.0 = [9042_i16,6655_i16,(-4455_i16),6900_i16,(-28336_i16)];
_10.1 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_10.2 = _6 as u8;
_3 = 113648656114608015701740169763807387715_i128 as f32;
RET.0 = [15588_i16,19740_i16,(-3257_i16),(-19352_i16),3180_i16];
_19 = _7;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(16_usize, 11_usize, Move(_11), 16_usize, Move(_16), 19_usize, Move(_19), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(16_usize, 8_usize, Move(_8), 13_usize, Move(_13), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: f32,mut _2: [isize; 3],mut _3: [i128; 1],mut _4: [isize; 3],mut _5: [isize; 3],mut _6: [i16; 5],mut _7: *mut f64,mut _8: Adt40,mut _9: isize,mut _10: (*const char, [bool; 8]),mut _11: [isize; 3]) -> (u64, i64) {
mir! {
type RET = (u64, i64);
let _12: u128;
let _13: *mut f64;
let _14: char;
let _15: [i16; 2];
let _16: Adt55;
let _17: [i16; 5];
let _18: isize;
let _19: [i16; 2];
let _20: i128;
let _21: bool;
let _22: u64;
let _23: (i32, [bool; 8], u8);
let _24: [i16; 2];
let _25: (i32, [bool; 8], u8);
let _26: *mut f64;
let _27: f32;
let _28: ();
let _29: ();
{
RET = (16300827657758598095_u64, 2669398445232343593_i64);
_3 = [52479062311246881163352681777344911546_i128];
_11 = [_9,_9,_9];
RET.1 = RET.0 as i64;
_13 = _7;
_9 = (-9223372036854775808_isize);
_11 = _4;
RET.1 = 884851879448210532_i64 ^ (-3923426178812079281_i64);
RET = (13073115201386084650_u64, 4473145123532917437_i64);
_3 = [108061948223342899588798843222201346946_i128];
_1 = _8.fld0 - _8.fld0;
_11 = [_9,_9,_9];
RET.0 = !7316039393459164437_u64;
_15 = [23562_i16,(-21981_i16)];
Goto(bb1)
}
bb1 = {
RET.0 = 10289809313404496799_u64;
_1 = _8.fld0;
RET.0 = 13157379970682003107_u64 | 213005182284848465_u64;
_2 = [_9,_9,_9];
RET.1 = -4186872028958533270_i64;
_14 = '\u{4808d}';
_5 = _4;
_3 = [(-116159835946469189451329099769662668868_i128)];
_11 = [_9,_9,_9];
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463454151235394913435648 => bb9,
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
_9 = (-16165_i16) as isize;
RET.1 = 3391040023522100671_i64 >> RET.0;
RET.0 = 18151766424490521345_u64;
Goto(bb10)
}
bb10 = {
_11 = [_9,_9,_9];
RET.1 = (-7451269171509600955_i64) + 4917238191414509933_i64;
_9 = 9223372036854775807_isize;
_12 = 4254144681439392986_usize as u128;
_2 = [_9,_9,_9];
_14 = '\u{3b46f}';
_18 = _9;
RET.0 = _18 as u64;
RET.1 = 4830394876792661862_i64 << _12;
_8.fld1 = core::ptr::addr_of!(RET.0);
_12 = !340193399987678744046031305098238953351_u128;
_19 = [26599_i16,(-16709_i16)];
_18 = _9;
_4 = [_9,_9,_9];
RET.0 = 32858_u16 as u64;
_5 = _2;
_13 = _7;
RET = (14893912454095984904_u64, (-4999521632236230354_i64));
_20 = 141042145068001996779046957839944089387_i128 | 148122899545821764289921918143175480900_i128;
_23.2 = !173_u8;
_21 = _8.fld0 > _8.fld0;
match RET.0 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
14893912454095984904 => bb18,
_ => bb17
}
}
bb11 = {
_9 = (-16165_i16) as isize;
RET.1 = 3391040023522100671_i64 >> RET.0;
RET.0 = 18151766424490521345_u64;
Goto(bb10)
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
Return()
}
bb17 = {
Return()
}
bb18 = {
_15 = [24364_i16,(-22069_i16)];
_22 = RET.1 as u64;
_23.0 = -(-678725513_i32);
_3 = [_20];
RET.1 = !5182696531822014641_i64;
_9 = _18 >> RET.1;
_11 = [_18,_9,_18];
RET.0 = _22;
_6 = [(-23790_i16),25215_i16,(-20064_i16),(-11635_i16),31412_i16];
_2 = [_18,_18,_9];
_10.0 = core::ptr::addr_of!(_14);
_9 = _18;
_23.1 = [_21,_21,_21,_21,_21,_21,_21,_21];
_24 = _15;
_3 = [_20];
_25.0 = _23.0;
_26 = _7;
_8.fld1 = core::ptr::addr_of!(RET.0);
_18 = _9 - _9;
RET.0 = !_22;
_12 = 35569_u16 as u128;
_19 = [9810_i16,(-25467_i16)];
_21 = true & false;
_25.1 = [_21,_21,_21,_21,_21,_21,_21,_21];
Goto(bb19)
}
bb19 = {
Call(_28 = dump_var(17_usize, 15_usize, Move(_15), 23_usize, Move(_23), 22_usize, Move(_22), 5_usize, Move(_5)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_28 = dump_var(17_usize, 19_usize, Move(_19), 2_usize, Move(_2), 6_usize, Move(_6), 12_usize, Move(_12)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: [i16; 5],mut _3: (*const char, [bool; 8]),mut _4: [i16; 5],mut _5: usize,mut _6: *const u64,mut _7: f64) -> (i32, [bool; 8], u8) {
mir! {
type RET = (i32, [bool; 8], u8);
let _8: u8;
let _9: Adt52;
let _10: *const u64;
let _11: Adt48;
let _12: [i16; 5];
let _13: f32;
let _14: f32;
let _15: isize;
let _16: bool;
let _17: i64;
let _18: char;
let _19: isize;
let _20: bool;
let _21: char;
let _22: [i16; 2];
let _23: i128;
let _24: usize;
let _25: isize;
let _26: u8;
let _27: (&'static usize, [i128; 1]);
let _28: ();
let _29: ();
{
RET.2 = !22_u8;
RET = ((-933120213_i32), _3.1, 119_u8);
_3.1 = [true,false,false,false,true,false,true,false];
RET = ((-450706874_i32), _3.1, 34_u8);
RET = ((-164617997_i32), _3.1, 50_u8);
_2 = [277_i16,31927_i16,31852_i16,(-1572_i16),31539_i16];
_2 = [(-15244_i16),31694_i16,25043_i16,(-31053_i16),(-13713_i16)];
RET.1 = [false,true,false,false,true,false,false,false];
_3.1 = [false,false,true,false,false,false,false,true];
_4 = _2;
RET.2 = 143_u8 << RET.0;
RET.1 = [false,false,false,true,true,false,true,false];
RET.2 = 101_u8;
RET = ((-595410256_i32), _3.1, 166_u8);
Goto(bb1)
}
bb1 = {
RET.0 = -(-1541738544_i32);
_4 = [31126_i16,20304_i16,(-1849_i16),(-13749_i16),(-5615_i16)];
RET.1 = [true,true,false,false,false,false,true,true];
RET = (208265024_i32, _3.1, 156_u8);
Goto(bb2)
}
bb2 = {
_5 = !0_usize;
RET.0 = 1923273393_i32 | 130044527_i32;
_5 = 16864295624110758994_usize;
RET = ((-136143476_i32), _3.1, 189_u8);
RET.1 = _3.1;
RET = (1794655601_i32, _3.1, 221_u8);
RET.1 = [true,false,true,true,true,false,true,false];
RET.0 = (-468725501_i32);
_1 = RET.2 as isize;
RET = ((-1469363497_i32), _3.1, 185_u8);
_8 = true as u8;
_2 = [14226_i16,14226_i16,25818_i16,(-5762_i16),10039_i16];
RET.0 = _7 as i32;
_7 = 335763958483542640091813662307280481497_u128 as f64;
RET.0 = 346735763_i32;
_7 = 101117380158071232025869793694311223881_i128 as f64;
_1 = _7 as isize;
_8 = RET.2;
RET.1 = _3.1;
_5 = 9273044709276042380_usize + 7870696478422800123_usize;
RET.0 = 403497839_i32;
RET.1 = [false,true,true,true,true,true,false,false];
_10 = _6;
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
185 => bb10,
_ => bb9
}
}
bb3 = {
RET.0 = -(-1541738544_i32);
_4 = [31126_i16,20304_i16,(-1849_i16),(-13749_i16),(-5615_i16)];
RET.1 = [true,true,false,false,false,false,true,true];
RET = (208265024_i32, _3.1, 156_u8);
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
_8 = RET.2;
RET = (1137246929_i32, _3.1, _8);
_1 = 9223372036854775807_isize | 76_isize;
RET.1 = [true,false,false,false,false,true,true,false];
_8 = !RET.2;
RET.0 = (-136403843_i32);
_7 = 13433898692361144842_u64 as f64;
_8 = RET.0 as u8;
RET.1 = _3.1;
_7 = 216296794369347546187506529329913332040_u128 as f64;
_3.1 = [true,false,false,true,false,true,true,false];
_8 = (-16922_i16) as u8;
_8 = _5 as u8;
_1 = 9223372036854775807_isize;
_5 = RET.0 as usize;
_3.1 = [true,false,true,true,false,true,true,false];
_2 = [(-5830_i16),(-18871_i16),30883_i16,(-10774_i16),(-25007_i16)];
RET = (592755946_i32, _3.1, _8);
_4 = [25943_i16,2726_i16,21996_i16,29743_i16,12684_i16];
_2 = _4;
_6 = _10;
_5 = !13754442180061663633_usize;
RET.0 = 818127827_i32;
_6 = _10;
_12 = _4;
match RET.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
818127827 => bb11,
_ => bb7
}
}
bb11 = {
RET.0 = (-1606535850_i32);
_13 = _5 as f32;
RET.2 = !_8;
_15 = _1;
RET.2 = _1 as u8;
Goto(bb12)
}
bb12 = {
_16 = false ^ true;
_5 = 1407177632471835078_usize;
_10 = _6;
RET.2 = _8 & _8;
_17 = (-101_i8) as i64;
RET = ((-288487735_i32), _3.1, _8);
RET.0 = _13 as i32;
RET = (1688555567_i32, _3.1, _8);
RET = (1474581454_i32, _3.1, _8);
RET.2 = _15 as u8;
_5 = 9057219826311042654_usize;
_18 = '\u{26646}';
_1 = _15;
RET.2 = _8;
_4 = _2;
_1 = _15 + _15;
RET.2 = !_8;
RET.0 = !(-1753005205_i32);
_20 = _16 & _16;
_18 = '\u{c71dd}';
_21 = _18;
RET.2 = _8 ^ _8;
_10 = _6;
_14 = _13;
_1 = _15 ^ _15;
_13 = _14 + _14;
_13 = _14;
Goto(bb13)
}
bb13 = {
RET.1 = [_20,_16,_20,_20,_16,_16,_16,_16];
_16 = _20;
_15 = _1 & _1;
_8 = !RET.2;
RET.1 = _3.1;
RET.2 = !_8;
_21 = _18;
_8 = !RET.2;
_21 = _18;
_10 = _6;
match _5 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
9057219826311042654 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_8 = RET.2;
RET = (1137246929_i32, _3.1, _8);
_1 = 9223372036854775807_isize | 76_isize;
RET.1 = [true,false,false,false,false,true,true,false];
_8 = !RET.2;
RET.0 = (-136403843_i32);
_7 = 13433898692361144842_u64 as f64;
_8 = RET.0 as u8;
RET.1 = _3.1;
_7 = 216296794369347546187506529329913332040_u128 as f64;
_3.1 = [true,false,false,true,false,true,true,false];
_8 = (-16922_i16) as u8;
_8 = _5 as u8;
_1 = 9223372036854775807_isize;
_5 = RET.0 as usize;
_3.1 = [true,false,true,true,false,true,true,false];
_2 = [(-5830_i16),(-18871_i16),30883_i16,(-10774_i16),(-25007_i16)];
RET = (592755946_i32, _3.1, _8);
_4 = [25943_i16,2726_i16,21996_i16,29743_i16,12684_i16];
_2 = _4;
_6 = _10;
_5 = !13754442180061663633_usize;
RET.0 = 818127827_i32;
_6 = _10;
_12 = _4;
match RET.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
818127827 => bb11,
_ => bb7
}
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
RET.2 = _8;
_3.1 = RET.1;
_6 = _10;
_22 = [27770_i16,16698_i16];
_24 = !_5;
_22 = [(-27167_i16),(-13509_i16)];
_24 = _5;
_26 = !_8;
RET.0 = (-1510345634_i32) - 1798758298_i32;
RET = ((-73970318_i32), _3.1, _8);
_13 = -_14;
RET = ((-133152240_i32), _3.1, _26);
_15 = _1;
_8 = !_26;
_23 = 160717774460627806218948685882655369921_i128;
_2 = [(-8908_i16),(-21653_i16),1829_i16,8623_i16,(-29882_i16)];
_19 = -_1;
Goto(bb20)
}
bb20 = {
Call(_28 = dump_var(18_usize, 16_usize, Move(_16), 26_usize, Move(_26), 20_usize, Move(_20), 19_usize, Move(_19)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_28 = dump_var(18_usize, 2_usize, Move(_2), 12_usize, Move(_12), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(1447133589810134153_usize), std::hint::black_box('\u{1fdaa}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-27608_i16)));
                
            }
#[derive(Debug)]
pub enum Adt39 {
Variant0{
fld0: [isize; 3],
fld1: char,
fld2: isize,
fld3: u8,
fld4: u32,
fld5: i32,

},
Variant1{
fld0: (u64, i64),
fld1: (u16, *mut *mut i64),
fld2: [isize; 3],
fld3: *mut *mut i64,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt40 {
fld0: f32,
fld1: *const u64,
}
#[derive(Debug)]
pub enum Adt41 {
Variant0{
fld0: bool,
fld1: *mut i64,
fld2: u128,

},
Variant1{
fld0: bool,
fld1: Adt40,
fld2: [bool; 8],
fld3: i8,
fld4: (*const char, [bool; 8]),
fld5: i32,
fld6: *const bool,
fld7: u8,

},
Variant2{
fld0: *const u64,
fld1: char,
fld2: ((u16, *mut *mut i64), [bool; 8], bool, [i128; 1], ([i16; 5],)),
fld3: *mut f64,
fld4: (*const char, [bool; 8]),
fld5: [i16; 5],
fld6: usize,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt42 {
fld0: [i16; 5],
fld1: (*const char, [bool; 8]),
fld2: usize,
fld3: i8,
fld4: Adt40,
}
#[derive(Debug)]
pub struct Adt43 {
fld0: Adt42,
fld1: [i16; 2],
fld2: Adt41,
fld3: [i128; 1],
fld4: [isize; 3],
fld5: i32,
}
#[derive(Debug,Copy,Clone)]
pub enum Adt44 {
Variant0{
fld0: (*const char, [bool; 8]),
fld1: i64,
fld2: *mut *mut i64,
fld3: (i32, [bool; 8], u8),
fld4: ((u16, *mut *mut i64), [bool; 8], bool, [i128; 1], ([i16; 5],)),
fld5: (u16, *mut *mut i64),

},
Variant1{
fld0: usize,

},
Variant2{
fld0: usize,
fld1: [i16; 2],
fld2: [u8; 3],

},
Variant3{
fld0: *mut f64,
fld1: char,
fld2: Adt40,
fld3: i8,
fld4: i16,
fld5: usize,
fld6: *mut u64,

}}
#[derive(Debug)]
pub struct Adt45 {
fld0: *mut f64,
fld1: char,
fld2: f64,
}
#[derive(Debug)]
pub enum Adt46 {
Variant0{
fld0: f64,
fld1: [i16; 5],
fld2: ([i16; 5],),
fld3: i64,
fld4: i16,

},
Variant1{
fld0: i128,

},
Variant2{
fld0: ([i16; 5],),
fld1: Adt39,
fld2: (i32, [bool; 8], u8),
fld3: i8,

},
Variant3{
fld0: [u8; 3],
fld1: char,
fld2: *const u64,
fld3: i8,
fld4: *mut u64,
fld5: ([i16; 5],),
fld6: i64,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt47 {
Variant0{
fld0: [u8; 3],
fld1: [isize; 3],
fld2: u32,
fld3: u128,
fld4: *mut u64,
fld5: Adt42,
fld6: f64,
fld7: *const char,

},
Variant1{
fld0: bool,
fld1: ([i16; 5],),
fld2: *mut i64,

}}
#[derive(Debug)]
pub enum Adt48 {
Variant0{
fld0: Adt39,
fld1: u16,
fld2: [u8; 3],
fld3: (*const char, [bool; 8]),
fld4: Adt46,

},
Variant1{
fld0: *mut i64,
fld1: *mut f64,
fld2: Adt44,
fld3: ([i16; 5],),
fld4: [bool; 8],
fld5: i32,

}}
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: (i32, [bool; 8], u8),
fld1: [i16; 5],
fld2: i8,

},
Variant1{
fld0: Adt46,
fld1: Adt47,
fld2: [i16; 5],
fld3: Adt45,
fld4: u8,
fld5: (([i16; 5],), *mut *mut i64, [isize; 3], [isize; 3]),

},
Variant2{
fld0: u8,
fld1: f64,
fld2: isize,
fld3: [bool; 8],
fld4: [isize; 3],
fld5: [i16; 5],
fld6: Adt39,
fld7: *const u64,

},
Variant3{
fld0: *mut *mut i64,
fld1: u128,
fld2: isize,
fld3: [isize; 3],
fld4: i16,
fld5: i32,
fld6: i64,
fld7: [bool; 8],

}}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: (([i16; 5],), *mut *mut i64, [isize; 3], [isize; 3]),
fld1: char,
fld2: *mut i64,

},
Variant1{
fld0: [isize; 3],
fld1: [i128; 1],
fld2: *mut f64,
fld3: [bool; 8],
fld4: *const bool,
fld5: i32,
fld6: Adt44,

},
Variant2{
fld0: (i32, [bool; 8], u8),
fld1: (*const char, [bool; 8]),
fld2: u16,

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: bool,
fld1: (i32, [bool; 8], u8),
fld2: i64,

},
Variant1{
fld0: bool,
fld1: char,
fld2: usize,
fld3: *const bool,
fld4: Adt47,
fld5: Adt48,
fld6: i64,
fld7: [i16; 2],

},
Variant2{
fld0: Adt44,

},
Variant3{
fld0: (u16, *mut *mut i64),
fld1: Adt43,
fld2: [i128; 1],

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: *mut *mut i64,
fld1: [i16; 5],

},
Variant1{
fld0: *mut *mut i64,
fld1: [i16; 5],
fld2: *const bool,
fld3: Adt43,

}}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: Adt46,
fld1: char,
fld2: Adt45,
fld3: Adt43,
fld4: *const u64,
fld5: i32,
fld6: u32,

},
Variant1{
fld0: Adt49,
fld1: *const char,
fld2: Adt40,

},
Variant2{
fld0: [isize; 3],
fld1: Adt39,
fld2: Adt49,

},
Variant3{
fld0: Adt39,
fld1: [i16; 5],
fld2: Adt47,
fld3: *mut f64,
fld4: i16,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt54 {
Variant0{
fld0: u16,
fld1: usize,
fld2: *const char,
fld3: i128,

},
Variant1{
fld0: *mut i64,
fld1: (u16, *mut *mut i64),

},
Variant2{
fld0: bool,
fld1: i16,

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: *const bool,
fld1: Adt50,
fld2: Adt53,

},
Variant1{
fld0: Adt49,
fld1: u128,
fld2: f32,
fld3: usize,

}}

