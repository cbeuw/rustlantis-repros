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
pub fn fn0(mut _1: u8,mut _2: char,mut _3: isize) -> *mut char {
mir! {
type RET = *mut char;
let _4: [u16; 6];
let _5: [i8; 3];
let _6: u32;
let _7: bool;
let _8: f64;
let _9: isize;
let _10: isize;
let _11: [i32; 7];
let _12: usize;
let _13: i32;
let _14: [u128; 6];
let _15: (u8, [i64; 8]);
let _16: *const i128;
let _17: (char,);
let _18: i8;
let _19: [i64; 1];
let _20: i64;
let _21: (char,);
let _22: (i64, i8);
let _23: f64;
let _24: *const *const f32;
let _25: *const f32;
let _26: ([u128; 6],);
let _27: f32;
let _28: i16;
let _29: ([u128; 6],);
let _30: ([i64; 1], (u8, usize, f64, i16, u16, [i64; 1]), usize);
let _31: [i32; 7];
let _32: (i64, i8);
let _33: f32;
let _34: isize;
let _35: (u8, [i64; 8]);
let _36: ();
let _37: ();
{
_1 = 239_u8 * 186_u8;
RET = core::ptr::addr_of_mut!(_2);
RET = core::ptr::addr_of_mut!((*RET));
_4 = [18917_u16,28052_u16,57402_u16,34115_u16,46698_u16,61483_u16];
_4 = [4319_u16,2049_u16,25157_u16,25444_u16,44915_u16,9486_u16];
_2 = '\u{6aaab}';
_3 = 74_isize;
_3 = 9223372036854775807_isize ^ 9223372036854775807_isize;
RET = core::ptr::addr_of_mut!((*RET));
_5 = [56_i8,43_i8,81_i8];
RET = core::ptr::addr_of_mut!((*RET));
_2 = '\u{7fcf5}';
_2 = '\u{b0c55}';
RET = core::ptr::addr_of_mut!((*RET));
Goto(bb1)
}
bb1 = {
_6 = 3495993219_u32 << _1;
RET = core::ptr::addr_of_mut!((*RET));
_4 = [11501_u16,61881_u16,49792_u16,32840_u16,27038_u16,53351_u16];
_4 = [9136_u16,10535_u16,45578_u16,25940_u16,8415_u16,30237_u16];
_5 = [(-83_i8),(-44_i8),102_i8];
_5 = [(-68_i8),(-113_i8),43_i8];
_6 = 11214942700898881266_u64 as u32;
RET = core::ptr::addr_of_mut!((*RET));
_4 = [4605_u16,57096_u16,12006_u16,54624_u16,40683_u16,42523_u16];
_5 = [18_i8,(-117_i8),26_i8];
_6 = 1762858817_u32;
_3 = -9223372036854775807_isize;
_3 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_8 = 2849296389219218634_i64 as f64;
Goto(bb2)
}
bb2 = {
_7 = !false;
_7 = false;
_4 = [21329_u16,48382_u16,59008_u16,26104_u16,53688_u16,10003_u16];
_2 = '\u{30310}';
_9 = _6 as isize;
_5 = [(-36_i8),(-33_i8),53_i8];
_9 = _1 as isize;
_8 = 120_i8 as f64;
_4 = [2954_u16,63628_u16,64119_u16,12447_u16,56631_u16,32637_u16];
_4 = [32615_u16,19341_u16,11034_u16,17373_u16,18315_u16,28625_u16];
_6 = !2614985734_u32;
_8 = 69_i8 as f64;
_10 = _3 | _3;
_5 = [(-1_i8),106_i8,(-18_i8)];
_5 = [50_i8,(-29_i8),13_i8];
_12 = 1_usize;
match _4[_12] {
0 => bb3,
1 => bb4,
2 => bb5,
19341 => bb7,
_ => bb6
}
}
bb3 = {
_6 = 3495993219_u32 << _1;
RET = core::ptr::addr_of_mut!((*RET));
_4 = [11501_u16,61881_u16,49792_u16,32840_u16,27038_u16,53351_u16];
_4 = [9136_u16,10535_u16,45578_u16,25940_u16,8415_u16,30237_u16];
_5 = [(-83_i8),(-44_i8),102_i8];
_5 = [(-68_i8),(-113_i8),43_i8];
_6 = 11214942700898881266_u64 as u32;
RET = core::ptr::addr_of_mut!((*RET));
_4 = [4605_u16,57096_u16,12006_u16,54624_u16,40683_u16,42523_u16];
_5 = [18_i8,(-117_i8),26_i8];
_6 = 1762858817_u32;
_3 = -9223372036854775807_isize;
_3 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_8 = 2849296389219218634_i64 as f64;
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
_9 = _3 << _12;
_11 = [(-1969145517_i32),800322142_i32,647475284_i32,990098478_i32,1383304543_i32,(-1969959580_i32),1526270029_i32];
_6 = _7 as u32;
_11 = [1512175464_i32,(-940519730_i32),(-2083414623_i32),342971696_i32,(-740106356_i32),1184055020_i32,1212805955_i32];
_15.1 = [(-224275272429106803_i64),(-7335121170762207636_i64),7590981404792130224_i64,7849228124300573972_i64,6821536147662592666_i64,(-6061973060462662629_i64),6698006449676196887_i64,(-3834771306919546885_i64)];
_2 = '\u{209cc}';
_14 = [250909010085205341255861215839439886617_u128,56022109957365266034293978577779815130_u128,227941343022159105745127427951636492439_u128,54194849054241025749011785922901576499_u128,329316396621293313515047665676002715241_u128,273726499711809141518324874367373019951_u128];
RET = core::ptr::addr_of_mut!(_2);
_6 = !3327214872_u32;
_6 = 1006717293_u32;
_15.0 = _1 * _1;
Goto(bb8)
}
bb8 = {
_3 = _10;
_17 = ((*RET),);
_8 = _11[_12] as f64;
_13 = _11[_12] | _11[_12];
_7 = _10 <= _9;
RET = core::ptr::addr_of_mut!((*RET));
_7 = false;
_14[_12] = 67434737148098522349673612232590757489_u128 ^ 161268658462863437494730375634621101386_u128;
_5[_12] = _6 as i8;
_14[_12] = 28818072509106065030946190971890305378_u128 & 320578493561792959621502091327911765096_u128;
_18 = _4[_12] as i8;
_19 = [_15.1[_12]];
match _11[_12] {
0 => bb4,
1 => bb9,
340282366920938463463374607430827691726 => bb11,
_ => bb10
}
}
bb9 = {
_6 = 3495993219_u32 << _1;
RET = core::ptr::addr_of_mut!((*RET));
_4 = [11501_u16,61881_u16,49792_u16,32840_u16,27038_u16,53351_u16];
_4 = [9136_u16,10535_u16,45578_u16,25940_u16,8415_u16,30237_u16];
_5 = [(-83_i8),(-44_i8),102_i8];
_5 = [(-68_i8),(-113_i8),43_i8];
_6 = 11214942700898881266_u64 as u32;
RET = core::ptr::addr_of_mut!((*RET));
_4 = [4605_u16,57096_u16,12006_u16,54624_u16,40683_u16,42523_u16];
_5 = [18_i8,(-117_i8),26_i8];
_6 = 1762858817_u32;
_3 = -9223372036854775807_isize;
_3 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_8 = 2849296389219218634_i64 as f64;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_11 = [_13,_13,_13,_13,_13,_13,_13];
_3 = !_10;
_18 = !_5[_12];
_17 = (_2,);
_11[_12] = _13 - _13;
_3 = _8 as isize;
_17 = ((*RET),);
_21.0 = _17.0;
_21 = ((*RET),);
_18 = !_5[_12];
Goto(bb12)
}
bb12 = {
_20 = !_15.1[_12];
RET = core::ptr::addr_of_mut!(_2);
_7 = !true;
_9 = _3 + _3;
_14[_12] = 129662969338507979089687296181119514724_u128 >> _9;
_22.0 = _15.1[_12];
_13 = _4[_12] as i32;
_3 = _9;
_7 = (*RET) < (*RET);
match _4[_12] {
0 => bb1,
19341 => bb13,
_ => bb4
}
}
bb13 = {
_21.0 = _17.0;
_3 = _9;
_15.1[_12] = _20;
_11[_12] = _14[_12] as i32;
_13 = _6 as i32;
_2 = _17.0;
_18 = _8 as i8;
_29 = (_14,);
_26 = _29;
_25 = core::ptr::addr_of!(_27);
_24 = core::ptr::addr_of!(_25);
Call(_15.1[_12] = fn1(_19, _14[_12], _26, _26.0, _9, _9, _7, _11[_12], _8, _20, _26.0[_12], _19, _29.0, _26), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_30.1.5 = [_20];
_21 = _17;
_30.1.3 = (-2519_i16) * 12910_i16;
_4 = [26928_u16,44312_u16,19479_u16,7035_u16,18544_u16,62524_u16];
_23 = _8 * _8;
_4 = [37254_u16,46435_u16,38351_u16,62699_u16,63885_u16,40026_u16];
_32.0 = _20;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(0_usize, 20_usize, Move(_20), 9_usize, Move(_9), 19_usize, Move(_19), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(0_usize, 10_usize, Move(_10), 1_usize, Move(_1), 17_usize, Move(_17), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(0_usize, 4_usize, Move(_4), 18_usize, Move(_18), 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [i64; 1],mut _2: u128,mut _3: ([u128; 6],),mut _4: [u128; 6],mut _5: isize,mut _6: isize,mut _7: bool,mut _8: i32,mut _9: f64,mut _10: i64,mut _11: u128,mut _12: [i64; 1],mut _13: [u128; 6],mut _14: ([u128; 6],)) -> i64 {
mir! {
type RET = i64;
let _15: isize;
let _16: ([u128; 6],);
let _17: i64;
let _18: bool;
let _19: usize;
let _20: (u8, usize, f64, i16, u16, [i64; 1]);
let _21: [i64; 1];
let _22: f64;
let _23: i8;
let _24: [i128; 3];
let _25: Adt64;
let _26: usize;
let _27: ();
let _28: ();
{
_9 = _8 as f64;
_15 = _5 ^ _6;
_3 = _14;
Call(_3 = fn2(_5, _14, _10, _1, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = !1264423119_i32;
Call(_8 = fn3(_4, _11, _6, _14.0, _9, _15, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = _2 & _2;
RET = -_10;
_8 = -1620685677_i32;
_8 = (-285837530_i32);
Goto(bb3)
}
bb3 = {
_13 = _4;
_13 = _4;
_6 = '\u{105b8b}' as isize;
_3 = _14;
_14.0 = [_11,_11,_11,_2,_2,_2];
_11 = _2 & _2;
_3.0 = _14.0;
_2 = _11 << RET;
_15 = _6;
_9 = _2 as f64;
_18 = _7;
_12 = _1;
_14 = (_3.0,);
_16 = (_3.0,);
RET = !_10;
_19 = 47_u8 as usize;
_6 = _5 ^ _5;
_16.0 = _13;
_13 = [_11,_11,_11,_11,_2,_11];
_3.0 = _4;
_20.5 = [RET];
_1 = [RET];
_17 = RET - RET;
_17 = -RET;
_6 = !_5;
Goto(bb4)
}
bb4 = {
_21 = [_17];
_19 = 0_usize | 2_usize;
_16 = _14;
_19 = 8494665755192206375_usize;
_8 = 15328816068507024183876068174975016779_i128 as i32;
_7 = _18;
_3.0 = _16.0;
_20.0 = 176_u8 << _11;
_20.0 = 36_u8 - 109_u8;
_7 = _18;
_20.4 = 22660_u16 + 25457_u16;
_16 = (_13,);
_3 = (_14.0,);
_19 = _6 as usize;
_15 = _5;
_20 = (74_u8, _19, _9, 20292_i16, 22885_u16, _1);
_16.0 = [_2,_2,_2,_11,_11,_2];
_14 = (_3.0,);
_1 = [_10];
RET = _10 & _17;
Goto(bb5)
}
bb5 = {
Call(_27 = dump_var(1_usize, 14_usize, Move(_14), 19_usize, Move(_19), 18_usize, Move(_18), 17_usize, Move(_17)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_27 = dump_var(1_usize, 6_usize, Move(_6), 8_usize, Move(_8), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_27 = dump_var(1_usize, 11_usize, Move(_11), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: ([u128; 6],),mut _3: i64,mut _4: [i64; 1],mut _5: isize) -> ([u128; 6],) {
mir! {
type RET = ([u128; 6],);
let _6: [u16; 6];
let _7: ();
let _8: ();
{
RET = _2;
RET = _2;
_2 = (RET.0,);
_3 = true as i64;
RET.0 = [219439371177234021017342564146751994895_u128,224751638897257311893775659142705065525_u128,292270518936044118906791085410693377133_u128,73599417618345661365517571596567919975_u128,230962505253328819554178233387271986216_u128,133713717825356836525926014395920842603_u128];
RET.0 = [136336587085202409243401852413195758040_u128,313920143411213520744169450059855629993_u128,296877888804546062590883392234172610415_u128,263311828164073055681188290949188250347_u128,219704976948296514047250862433775546879_u128,319991889624879093883758400920581025167_u128];
RET = (_2.0,);
_4 = [_3];
_3 = 4930890000153961941_i64;
_3 = 148440502034785747_i64 + (-6221333072113000643_i64);
RET.0 = [155599218661820763357398972169513201733_u128,124671197089961355576808617315974129750_u128,137553678789484713197122947747798051188_u128,84176346039983523158374631956629561560_u128,76926660490389225388417448613940251602_u128,316814450562296334274292944598924549577_u128];
RET.0 = [212687825605446871065677867106492188222_u128,176026646771923367701078821202153362923_u128,151029164364012893641499045837672051470_u128,262942724813882444624073344071777384534_u128,263940346921017927373905222415836212644_u128,203004322265462181590244823797609320127_u128];
_3 = 36_i8 as i64;
_3 = !(-3098867604258217146_i64);
_3 = (-3649456849019792252_i64);
RET = (_2.0,);
_4 = [_3];
_1 = _5;
_6 = [33811_u16,29025_u16,41955_u16,15360_u16,48908_u16,34592_u16];
_2 = RET;
_1 = 79_u8 as isize;
_4 = [_3];
_4 = [_3];
RET = (_2.0,);
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(2_usize, 3_usize, Move(_3), 6_usize, Move(_6), 2_usize, Move(_2), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [u128; 6],mut _2: u128,mut _3: isize,mut _4: [u128; 6],mut _5: f64,mut _6: isize,mut _7: [u128; 6]) -> i32 {
mir! {
type RET = i32;
let _8: Adt49;
let _9: i8;
let _10: Adt51;
let _11: *const f32;
let _12: [i128; 3];
let _13: Adt61;
let _14: (([u128; 6],), *const f32, ([u128; 6],), u64);
let _15: u8;
let _16: u128;
let _17: ([i64; 1], (u8, usize, f64, i16, u16, [i64; 1]), usize);
let _18: Adt61;
let _19: (i16, [i64; 7], *mut i8);
let _20: Adt48;
let _21: u16;
let _22: char;
let _23: ((u8, usize, f64, i16, u16, [i64; 1]), [i64; 7], isize);
let _24: Adt58;
let _25: char;
let _26: *const i64;
let _27: Adt48;
let _28: f64;
let _29: ();
let _30: ();
{
RET = -2008397301_i32;
_5 = 77_i8 as f64;
_2 = !34729077222257830161076983360274386914_u128;
_2 = false as u128;
_6 = 36967_u16 as isize;
_5 = 100_i8 as f64;
_2 = 269357149638522844863436550214390684356_u128 * 156338181576882269177557541004577822073_u128;
RET = (-1763832470_i32) << _6;
_5 = 124161142498353536_usize as f64;
_1 = [_2,_2,_2,_2,_2,_2];
_4 = [_2,_2,_2,_2,_2,_2];
_3 = _6 ^ _6;
_4 = [_2,_2,_2,_2,_2,_2];
RET = (-833034314_i32) >> _2;
Goto(bb1)
}
bb1 = {
_3 = -_6;
RET = 1971566913_i32;
_6 = !_3;
RET = !(-1956769534_i32);
_9 = 70_i8;
_7 = _4;
_9 = 16_i8;
_6 = -_3;
RET = (-1314812512_i32);
_6 = 34521_u16 as isize;
_5 = _2 as f64;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607430453398944 => bb9,
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
_12 = [63448450757940576864521863478874856986_i128,(-70161346540428034245428985357793087294_i128),54697247378518475523585297136025163510_i128];
_9 = (-40_i8);
_12 = [164822047713431849352270372427941382842_i128,1930006901827273502685882913490962472_i128,129914275726927791856990138416301047269_i128];
_4 = [_2,_2,_2,_2,_2,_2];
RET = (-1842367222_i32);
_14.0 = (_1,);
_14.3 = 8657118882213258204_u64;
_14.3 = 5036436495738185074_u64 - 1816064765547962087_u64;
_6 = _3 | _3;
_2 = !24089280758362459263095670571565640375_u128;
_14.2.0 = _7;
RET = 109_u8 as i32;
_5 = _9 as f64;
_17.1.3 = (-13346_i16);
_17.2 = RET as usize;
RET = _14.3 as i32;
_1 = [_2,_2,_2,_2,_2,_2];
_17.1.3 = 10415_i16 | (-16360_i16);
_12 = [150729076963932272955226557152714898418_i128,63437308254977521817235150052051779541_i128,(-86901908558697403231470382896919018144_i128)];
_17.2 = 3_usize & 7_usize;
RET = (-1735096619_i32);
Goto(bb10)
}
bb10 = {
_19.2 = core::ptr::addr_of_mut!(_9);
_17.1.1 = _17.2;
_15 = 77_u8;
_17.1.0 = _2 as u8;
_7 = _14.2.0;
_20.fld1.1 = _9 | _9;
Goto(bb11)
}
bb11 = {
_17.1.4 = 50045_u16 ^ 36690_u16;
_17.0 = [8631743787996327075_i64];
_20.fld1.0 = 1263315914475270889_i64;
_16 = _20.fld1.1 as u128;
_20.fld0.0 = [_2,_16,_16,_2,_16,_16];
_20.fld3 = _20.fld1.1;
_20.fld1.0 = _20.fld3 as i64;
_12 = [150617371134906307966429617071255372644_i128,(-36477803405641764015884379408630867623_i128),(-156212181012531130895666637960058906842_i128)];
_17.1.5 = [_20.fld1.0];
_20.fld2.0 = _17.1.3;
Goto(bb12)
}
bb12 = {
_27.fld1.0 = 3712428931_u32 as i64;
match RET {
0 => bb13,
1 => bb14,
340282366920938463463374607430033114837 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_19.2 = core::ptr::addr_of_mut!(_9);
_17.1.1 = _17.2;
_15 = 77_u8;
_17.1.0 = _2 as u8;
_7 = _14.2.0;
_20.fld1.1 = _9 | _9;
Goto(bb11)
}
bb15 = {
Return()
}
bb16 = {
_27.fld0.0 = [_16,_16,_16,_2,_16,_16];
_4 = [_16,_16,_16,_16,_16,_16];
_17.1 = (_15, _17.2, _5, _20.fld2.0, 336_u16, _17.0);
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(3_usize, 16_usize, Move(_16), 1_usize, Move(_1), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(3_usize, 9_usize, Move(_9), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(73_u8), std::hint::black_box('\u{df35b}'), std::hint::black_box((-86_isize)));
                
            }
#[derive(Debug)]
pub struct Adt48 {
fld0: ([u128; 6],),
fld1: (i64, i8),
fld2: (i16,),
fld3: i8,
fld4: *const f32,
}
#[derive(Debug,Copy,Clone)]
pub enum Adt49 {
Variant0{
fld0: [u128; 6],
fld1: u16,
fld2: usize,
fld3: u32,
fld4: i16,
fld5: u128,
fld6: *const f32,

},
Variant1{
fld0: f64,
fld1: char,
fld2: *const *const f32,

},
Variant2{
fld0: bool,
fld1: [i8; 3],
fld2: isize,
fld3: [i64; 7],
fld4: (u8, [i64; 8]),
fld5: u128,

},
Variant3{
fld0: *const *const f32,
fld1: [i64; 1],
fld2: *const f32,
fld3: [i64; 8],
fld4: i16,
fld5: u64,
fld6: (i16,),

}}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: [i32; 7],
fld1: u32,
fld2: *mut (([u128; 6],), *const f32, ([u128; 6],), u64),
fld3: [u64; 4],

},
Variant1{
fld0: [i64; 7],
fld1: ((u8, usize, f64, i16, u16, [i64; 1]), [i64; 7], isize),
fld2: [u128; 6],
fld3: [i32; 7],
fld4: *const i128,
fld5: [i128; 3],
fld6: i64,

},
Variant2{
fld0: usize,
fld1: char,
fld2: (i16,),
fld3: i8,
fld4: *mut char,

},
Variant3{
fld0: [u128; 6],
fld1: f32,
fld2: *mut i8,
fld3: *mut u8,
fld4: u8,
fld5: ([i64; 1], (u8, usize, f64, i16, u16, [i64; 1]), usize),

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: (i16, [i64; 7], *mut i8),
fld1: [i64; 1],
fld2: *const i128,

},
Variant1{
fld0: (i128, [u32; 2]),
fld1: (i16,),

},
Variant2{
fld0: *mut (([u128; 6],), *const f32, ([u128; 6],), u64),
fld1: f64,

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: *const f32,

},
Variant1{
fld0: ([u128; 6],),
fld1: i128,
fld2: (u128, *const *const f32, (i16,)),
fld3: [i32; 7],
fld4: *mut char,

},
Variant2{
fld0: u8,
fld1: char,
fld2: u16,
fld3: *const i128,
fld4: (u8, usize, f64, i16, u16, [i64; 1]),
fld5: (([u128; 6],), *const f32, ([u128; 6],), u64),

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt53 {
fld0: usize,
fld1: i128,
fld2: (i64, i8),
fld3: [i128; 3],
}
#[derive(Debug,Copy,Clone)]
pub enum Adt54 {
Variant0{
fld0: i32,

},
Variant1{
fld0: *const *const f32,
fld1: (u8, usize, f64, i16, u16, [i64; 1]),
fld2: i16,
fld3: i8,

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: (u8, usize, f64, i16, u16, [i64; 1]),
fld1: Adt53,

},
Variant1{
fld0: (([u128; 6],), *const f32, ([u128; 6],), u64),
fld1: ([u128; 6],),
fld2: [i32; 7],
fld3: Adt51,
fld4: *const f32,
fld5: *const *const f32,

},
Variant2{
fld0: bool,
fld1: ([i64; 1], (u8, usize, f64, i16, u16, [i64; 1]), usize),
fld2: Adt54,
fld3: *mut i8,
fld4: [u128; 6],
fld5: *mut char,
fld6: (i128, [u32; 2]),

},
Variant3{
fld0: *const *const f32,
fld1: *const i128,
fld2: u16,
fld3: (i64, i8),
fld4: *const i64,
fld5: i64,

}}
#[derive(Debug)]
pub enum Adt56 {
Variant0{
fld0: u8,
fld1: usize,
fld2: *mut i8,
fld3: [i32; 7],
fld4: u64,
fld5: *const i64,
fld6: *const f32,
fld7: *mut (([u128; 6],), *const f32, ([u128; 6],), u64),

},
Variant1{
fld0: i64,

}}
#[derive(Debug)]
pub enum Adt57 {
Variant0{
fld0: bool,

},
Variant1{
fld0: Adt50,
fld1: char,
fld2: i128,
fld3: Adt54,
fld4: i16,
fld5: [i64; 7],

}}
#[derive(Debug)]
pub enum Adt58 {
Variant0{
fld0: [i128; 3],
fld1: ([u128; 6],),
fld2: i128,

},
Variant1{
fld0: (([u128; 6],), *const f32, ([u128; 6],), u64),
fld1: [u32; 2],
fld2: [i8; 3],
fld3: *mut (([u128; 6],), *const f32, ([u128; 6],), u64),
fld4: u64,
fld5: i32,
fld6: *mut char,

},
Variant2{
fld0: Adt55,
fld1: *const i128,
fld2: (i128, [u32; 2]),
fld3: [i128; 3],
fld4: *const f32,
fld5: *const *const f32,

}}
#[derive(Debug)]
pub enum Adt59 {
Variant0{
fld0: [i64; 1],
fld1: (char,),
fld2: i128,
fld3: i32,
fld4: i16,

},
Variant1{
fld0: i128,
fld1: Adt57,

}}
#[derive(Debug)]
pub enum Adt60 {
Variant0{
fld0: u64,
fld1: (i64, i8),
fld2: ((u8, usize, f64, i16, u16, [i64; 1]), [i64; 7], isize),
fld3: [i8; 3],
fld4: Adt48,
fld5: Adt58,
fld6: Adt49,

},
Variant1{
fld0: (i16, [i64; 7], *mut i8),
fld1: Adt48,
fld2: isize,
fld3: usize,
fld4: u32,
fld5: ((u8, usize, f64, i16, u16, [i64; 1]), [i64; 7], isize),

}}
#[derive(Debug)]
pub enum Adt61 {
Variant0{
fld0: Adt48,
fld1: u16,
fld2: *const i64,
fld3: Adt55,

},
Variant1{
fld0: *const i128,
fld1: Adt53,
fld2: [i64; 7],
fld3: (i128, [u32; 2]),
fld4: Adt50,
fld5: Adt60,
fld6: u32,
fld7: f64,

},
Variant2{
fld0: Adt54,
fld1: [u128; 6],
fld2: *const i128,
fld3: [u16; 6],
fld4: Adt51,

}}
#[derive(Debug)]
pub enum Adt62 {
Variant0{
fld0: Adt58,
fld1: [u32; 2],

},
Variant1{
fld0: [i32; 7],
fld1: (char,),
fld2: [u32; 2],
fld3: u64,
fld4: (i64, i8),
fld5: Adt54,
fld6: i64,
fld7: i128,

}}
#[derive(Debug)]
pub struct Adt63 {
fld0: usize,
fld1: Adt54,
fld2: isize,
fld3: Adt49,
fld4: [u32; 2],
fld5: Adt61,
fld6: Adt55,
}
#[derive(Debug)]
pub enum Adt64 {
Variant0{
fld0: Adt61,
fld1: [i32; 7],

},
Variant1{
fld0: u8,
fld1: (i128, [u32; 2]),
fld2: *mut (([u128; 6],), *const f32, ([u128; 6],), u64),
fld3: (([u128; 6],), *const f32, ([u128; 6],), u64),
fld4: *const i128,

},
Variant2{
fld0: Adt59,
fld1: u16,
fld2: *mut i8,

}}

