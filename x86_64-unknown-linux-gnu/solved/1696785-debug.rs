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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: u64,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32) -> [u64; 6] {
mir! {
type RET = [u64; 6];
let _13: i32;
let _14: [u16; 8];
let _15: u32;
let _16: Adt53;
let _17: ([u16; 8],);
let _18: Adt51;
let _19: u8;
let _20: isize;
let _21: bool;
let _22: ([u16; 8],);
let _23: bool;
let _24: [i8; 6];
let _25: [i8; 6];
let _26: (isize, (i8,), u64, f32, i32, [u64; 6]);
let _27: f64;
let _28: i16;
let _29: *mut i16;
let _30: Adt61;
let _31: Adt62;
let _32: isize;
let _33: *const *const u64;
let _34: isize;
let _35: (*const u64,);
let _36: Adt48;
let _37: u64;
let _38: [u64; 6];
let _39: ();
let _40: ();
{
_7 = -(-4642722828524980390_i64);
_12 = !145514707_u32;
_6 = 17607900921540241811_u64 - 7189351054804364158_u64;
_2 = '\u{10a870}';
RET = [_6,_6,_6,_6,_6,_6];
_8 = (-140710386792014326685282931718958881134_i128);
_1 = _7 != _7;
_12 = 2625415694_u32 * 424231747_u32;
RET = [_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6];
_5 = 8353_i16;
_12 = 1295197090_u32;
_9 = _7 as usize;
_11 = 233_u8 as u16;
_9 = !2_usize;
_3 = !9223372036854775807_isize;
_11 = !23740_u16;
_2 = '\u{ebb10}';
Goto(bb1)
}
bb1 = {
_9 = 11566293100955349104_usize;
_1 = false & false;
_6 = 15444613513995710899_u64 & 15771806970713303747_u64;
_7 = 6012356095027261125_i64;
_3 = -30_isize;
_13 = !798070490_i32;
Call(_3 = fn1(_7, _2, _11, _8, _6, _11, _12, _5, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = (-8625998993465596028_i64) >> _5;
Goto(bb3)
}
bb3 = {
_2 = '\u{82dfd}';
_9 = _11 as usize;
_2 = '\u{107196}';
_14 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = (-93_i8) as u8;
_2 = '\u{65fd7}';
_13 = (-1024354236_i32) & (-623110773_i32);
_9 = _3 as usize;
_7 = -(-578715176030899529_i64);
_12 = 1694199902_u32 | 2207396207_u32;
_4 = 0_i8;
_6 = 4966248867129083675_u64;
_7 = 8434066517909176671_i64 & (-7138890158318733822_i64);
_15 = _12;
_3 = _10 as isize;
_15 = _12;
Goto(bb4)
}
bb4 = {
_1 = false;
_17 = (_14,);
_5 = 3877_i16 - 31129_i16;
_17.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_17.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_14 = _17.0;
_11 = 44222_u16;
_11 = 53614_u16;
_10 = !192_u8;
_19 = !_10;
_17 = (_14,);
_20 = _3;
_11 = !63784_u16;
_14 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = [_6,_6,_6,_6,_6,_6];
Call(_6 = core::intrinsics::transmute(_7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12 = !_15;
_6 = 4244270646356282925_u64 >> _13;
_8 = (-70838116270890227303213938050416674183_i128);
_4 = -(-70_i8);
_2 = '\u{a5f3b}';
_6 = 13174357804876385940_u64 ^ 14835126109438571046_u64;
_3 = _15 as isize;
RET = [_6,_6,_6,_6,_6,_6];
_14 = [_11,_11,_11,_11,_11,_11,_11,_11];
_19 = _10;
_17 = (_14,);
_20 = _3;
_8 = _13 as i128;
_5 = 9464_i16;
_14 = [_11,_11,_11,_11,_11,_11,_11,_11];
_3 = _20;
RET = [_6,_6,_6,_6,_6,_6];
Goto(bb6)
}
bb6 = {
_20 = _3 | _3;
match _5 {
0 => bb5,
9464 => bb7,
_ => bb3
}
}
bb7 = {
_13 = (-1962879159_i32);
_20 = !_3;
_7 = 266654950778801630500222967686476273151_u128 as i64;
_17 = (_14,);
_17.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = 113853576995542017962059113802724614058_i128 - 134195312727246827383944745942791099145_i128;
RET = [_6,_6,_6,_6,_6,_6];
_20 = _3;
_3 = -_20;
_22 = _17;
RET = [_6,_6,_6,_6,_6,_6];
_12 = _15 - _15;
_23 = _1;
_6 = 14281894541464523326_u64 ^ 11713267458100519581_u64;
_19 = _10 ^ _10;
_2 = '\u{9715a}';
_8 = (-137476121867570854273760928696741051290_i128) - 84869102459562392971920547900523922149_i128;
_9 = _19 as usize;
_24 = [_4,_4,_4,_4,_4,_4];
_5 = 8029_i16 * (-26232_i16);
RET = [_6,_6,_6,_6,_6,_6];
_17.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_17.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_17.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
Goto(bb8)
}
bb8 = {
_12 = !_15;
_15 = _12 * _12;
_22 = _17;
RET = [_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6];
_22.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_14 = [_11,_11,_11,_11,_11,_11,_11,_11];
Goto(bb9)
}
bb9 = {
_10 = !_19;
_21 = _1;
_2 = '\u{56fcd}';
_4 = _2 as i8;
_19 = !_10;
Goto(bb10)
}
bb10 = {
_26.5 = [_6,_6,_6,_6,_6,_6];
_5 = (-21862_i16) & 27921_i16;
_26.2 = _6;
Goto(bb11)
}
bb11 = {
RET = _26.5;
_25 = [_4,_4,_4,_4,_4,_4];
_26.1.0 = _5 as i8;
_26.5 = [_6,_6,_6,_6,_26.2,_6];
_17 = _22;
_22 = (_14,);
_3 = _13 as isize;
_23 = _2 >= _2;
_26.3 = _5 as f32;
_23 = _10 <= _10;
_23 = !_21;
_24 = [_26.1.0,_26.1.0,_26.1.0,_26.1.0,_26.1.0,_4];
_17 = _22;
_23 = !_1;
_11 = 31675_u16 ^ 36103_u16;
_1 = _9 > _9;
_1 = !_21;
Call(_27 = fn4(_2, _22.0, _26.5, _26.5, _8, RET, _26.2, _24, _19, _9), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = [_26.2,_26.2,_6,_6,_26.2,_6];
_13 = _2 as i32;
_26.5 = [_26.2,_26.2,_6,_6,_26.2,_26.2];
_8 = !162548578826010480228457071313528168235_i128;
_20 = _27 as isize;
_19 = !_10;
_27 = _10 as f64;
_15 = _12;
_13 = (-663309814_i32) + 2016301078_i32;
_4 = _26.1.0 - _26.1.0;
_26.2 = _13 as u64;
_26.5 = [_6,_26.2,_6,_26.2,_6,_6];
_12 = _11 as u32;
Goto(bb13)
}
bb13 = {
_2 = '\u{606e4}';
_3 = _20;
_9 = !1_usize;
_30.fld3 = !_26.2;
_3 = _20;
_22.0 = _14;
_18 = Adt51::Variant3 { fld0: _26.1,fld1: _8 };
_21 = !_1;
_20 = _3 >> _15;
_26.4 = _13;
Call(_26 = fn7(_10, _20, _30.fld3, _15, _20, _11, _18, _20, _21), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_14 = _17.0;
_30.fld0.1 = _21 as i32;
_27 = _30.fld0.1 as f64;
_25 = _24;
_26.1 = Field::<(i8,)>(Variant(_18, 3), 0);
_7 = _15 as i64;
_34 = _26.0;
_26.0 = 23920276069190146425713282380710652140_u128 as isize;
_30.fld3 = _34 as u64;
_30.fld0.2 = _21 as i32;
_30.fld3 = _6;
_33 = core::ptr::addr_of!(_35.0);
RET = [_30.fld3,_6,_30.fld3,_30.fld3,_26.2,_30.fld3];
RET = [_30.fld3,_26.2,_6,_30.fld3,_6,_6];
_23 = _1;
SetDiscriminant(_18, 2);
_7 = -(-6553247409366354262_i64);
_30.fld0.0 = core::ptr::addr_of_mut!(_27);
_25 = [_26.1.0,_26.1.0,_26.1.0,_4,_4,_4];
_3 = -_34;
RET = [_30.fld3,_30.fld3,_6,_6,_30.fld3,_30.fld3];
_34 = _3;
_30.fld0.3 = core::ptr::addr_of_mut!(_28);
_29 = core::ptr::addr_of_mut!(_28);
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(0_usize, 17_usize, Move(_17), 15_usize, Move(_15), 20_usize, Move(_20), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(0_usize, 19_usize, Move(_19), 25_usize, Move(_25), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(0_usize, 21_usize, Move(_21), 8_usize, Move(_8), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i64,mut _2: char,mut _3: u16,mut _4: i128,mut _5: u64,mut _6: u16,mut _7: u32,mut _8: i16,mut _9: i128) -> isize {
mir! {
type RET = isize;
let _10: [i128; 6];
let _11: bool;
let _12: [i128; 6];
let _13: [u16; 8];
let _14: char;
let _15: u16;
let _16: bool;
let _17: isize;
let _18: f32;
let _19: (i64,);
let _20: u32;
let _21: *mut i64;
let _22: (i8,);
let _23: [u16; 4];
let _24: f32;
let _25: i32;
let _26: Adt49;
let _27: (isize, (i8,), u64, f32, i32, [u64; 6]);
let _28: ();
let _29: ();
{
_6 = (-85_i8) as u16;
_9 = 9223372036854775807_isize as i128;
_7 = _5 as u32;
_6 = _3;
RET = 72_isize ^ 9223372036854775807_isize;
_3 = !_6;
_9 = _4;
_5 = 6764645986260168200_u64;
RET = -9223372036854775807_isize;
RET = (-89_isize) & (-9223372036854775808_isize);
RET = 9223372036854775807_isize;
_13 = [_3,_3,_3,_6,_6,_3,_6,_6];
_10 = [_4,_4,_4,_4,_9,_4];
_8 = !(-5470_i16);
RET = 115_isize - (-9223372036854775808_isize);
_8 = (-21587_i16) - 11157_i16;
_15 = !_3;
_4 = _9 << _9;
_14 = _2;
RET = -9223372036854775807_isize;
_15 = !_3;
RET = !9223372036854775807_isize;
_4 = !_9;
_2 = _14;
_4 = _9;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
6764645986260168200 => bb6,
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
_1 = (-8872179663533220958_i64) - 4622827644825075767_i64;
_3 = _6;
_1 = 3326132433450924721_i64 & (-2274862873086740780_i64);
_11 = _2 <= _14;
_1 = (-8600379691696853287_i64);
_8 = _7 as i16;
_17 = RET;
_11 = true;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463454774227740071358169 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_6 = 71_i8 as u16;
Call(_10 = fn2(_5, _2, _7, _8, _5, _6, _17, _11, _1, _9, _13), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12 = [_9,_9,_4,_4,_4,_4];
_14 = _2;
_16 = !_11;
_14 = _2;
_1 = 4274411183617917028_i64 & 1534841798710430151_i64;
RET = _17 << _3;
Goto(bb10)
}
bb10 = {
_4 = _9;
_10 = [_4,_4,_9,_9,_4,_4];
_3 = _6;
_1 = -(-4900002686177192615_i64);
_21 = core::ptr::addr_of_mut!(_19.0);
_18 = 216428048304125598672945844903703304561_u128 as f32;
_19 = (_1,);
_19 = (_1,);
RET = -_17;
_4 = -_9;
_16 = !_11;
_22.0 = _16 as i8;
_18 = 13664767384664400774_usize as f32;
_11 = _16;
_2 = _14;
_13 = [_3,_15,_3,_6,_15,_15,_3,_3];
_7 = 1342591018_u32 - 2781141956_u32;
RET = _17;
_19 = (_1,);
_15 = _3 * _6;
_7 = 1773109418_u32 * 3154918744_u32;
match _9 {
0 => bb9,
1 => bb8,
2 => bb6,
3 => bb7,
4 => bb5,
5 => bb11,
6 => bb12,
199571980128924136778091675712809330322 => bb14,
_ => bb13
}
}
bb11 = {
_12 = [_9,_9,_4,_4,_4,_4];
_14 = _2;
_16 = !_11;
_14 = _2;
_1 = 4274411183617917028_i64 & 1534841798710430151_i64;
RET = _17 << _3;
Goto(bb10)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_6 = _18 as u16;
_8 = _22.0 as i16;
_24 = _8 as f32;
_15 = _3;
_23 = [_15,_6,_6,_15];
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(1_usize, 12_usize, Move(_12), 13_usize, Move(_13), 22_usize, Move(_22), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(1_usize, 10_usize, Move(_10), 11_usize, Move(_11), 2_usize, Move(_2), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(1_usize, 7_usize, Move(_7), 5_usize, Move(_5), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u64,mut _2: char,mut _3: u32,mut _4: i16,mut _5: u64,mut _6: u16,mut _7: isize,mut _8: bool,mut _9: i64,mut _10: i128,mut _11: [u16; 8]) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _12: [usize; 8];
let _13: u16;
let _14: (u16,);
let _15: u8;
let _16: isize;
let _17: u32;
let _18: isize;
let _19: [usize; 8];
let _20: i8;
let _21: (u16,);
let _22: [u16; 4];
let _23: char;
let _24: Adt54;
let _25: [u16; 4];
let _26: *mut i16;
let _27: ([u16; 8],);
let _28: (*const i16, (u8, usize, *mut u128, i16, i64));
let _29: bool;
let _30: [usize; 8];
let _31: u32;
let _32: f32;
let _33: (i8,);
let _34: ();
let _35: ();
{
RET = [_10,_10,_10,_10,_10,_10];
_3 = 1969905881_u32 >> _1;
RET = [_10,_10,_10,_10,_10,_10];
_4 = (-31424_i16);
_1 = _5 + _5;
RET = [_10,_10,_10,_10,_10,_10];
_9 = !(-7892296960107947465_i64);
_9 = 5863467342044069454_i64 ^ 5267588972453221695_i64;
_9 = (-8784529163729724224_i64) | 867865096515874214_i64;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = (-118571580583078521603347009543839527210_i128);
RET = [_10,_10,_10,_10,_10,_10];
_6 = 4835_u16 | 30882_u16;
_2 = '\u{5be42}';
_12 = [11534202413171242174_usize,3504250428822374788_usize,5_usize,6_usize,7_usize,5_usize,10735737034503272911_usize,6_usize];
_9 = 5696844711852859380_i64;
_2 = '\u{32732}';
_1 = !_5;
RET = [_10,_10,_10,_10,_10,_10];
_5 = _4 as u64;
_12 = [1438338142024564703_usize,325485161813552092_usize,16144348639285109074_usize,4_usize,0_usize,9594294650162056002_usize,1617633149991139512_usize,15180107733771078152_usize];
_12 = [6_usize,5255487561020888840_usize,12443982561137910537_usize,17413507860123900575_usize,3_usize,9088694787576634488_usize,5_usize,6_usize];
_7 = (-9223372036854775808_isize);
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463454151235394913435648 => bb7,
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
_5 = _1;
_1 = _5;
_12 = [14918277419434093099_usize,12111846462847889623_usize,3_usize,6_usize,3_usize,13142840974769954490_usize,7730516918080912579_usize,75194278605591810_usize];
RET = [_10,_10,_10,_10,_10,_10];
_5 = _1;
RET = [_10,_10,_10,_10,_10,_10];
_9 = 2476904388470656471_i64;
_13 = _5 as u16;
_5 = _1;
_3 = !866996589_u32;
_3 = 9_u8 as u32;
_10 = !85077640719411379241778533574207012972_i128;
_4 = 9291_i16;
_1 = _5;
_8 = !true;
_14.0 = !_6;
_10 = 198876658761102475216611132666119367797_u128 as i128;
_6 = _2 as u16;
_8 = !true;
_13 = !_14.0;
_10 = -89924903191749480557004971531189696442_i128;
_13 = _14.0 & _6;
_3 = _8 as u32;
_7 = _8 as isize;
RET = [_10,_10,_10,_10,_10,_10];
_10 = -(-60712058534903895505354048549237286084_i128);
_16 = _3 as isize;
Goto(bb8)
}
bb8 = {
_13 = _14.0 + _14.0;
_10 = _3 as i128;
RET = [_10,_10,_10,_10,_10,_10];
_11 = [_14.0,_13,_13,_13,_13,_13,_14.0,_6];
Call(_1 = core::intrinsics::bswap(_5), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12 = [7_usize,17575510735875672140_usize,7_usize,5222746184850304271_usize,3417355702695228367_usize,7_usize,16257198147451554510_usize,1_usize];
_5 = !_1;
_9 = _7 as i64;
RET = [_10,_10,_10,_10,_10,_10];
RET = [_10,_10,_10,_10,_10,_10];
_6 = _13 ^ _13;
_13 = !_6;
_3 = !1277986126_u32;
_5 = !_1;
_7 = _13 as isize;
_12 = [5_usize,0_usize,6_usize,3_usize,2948753820463287030_usize,13986756718553102054_usize,5_usize,5_usize];
_13 = _14.0;
_10 = 124129447395567127308078484825192484929_i128 * 18309250654270848288084380661392426958_i128;
_6 = !_14.0;
Goto(bb10)
}
bb10 = {
_14.0 = _6;
_17 = _3 * _3;
_2 = '\u{e5642}';
RET = [_10,_10,_10,_10,_10,_10];
_4 = 17887_i16;
_9 = !(-4408596074665433810_i64);
_9 = -230241996096944376_i64;
_17 = _3;
_13 = _4 as u16;
_14 = (_6,);
RET = [_10,_10,_10,_10,_10,_10];
_18 = _7;
_20 = -116_i8;
_3 = _4 as u32;
_2 = '\u{b215c}';
_20 = 127_i8;
Call(_1 = core::intrinsics::bswap(_5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15 = _3 as u8;
_10 = 128753939791055764793852902946162756575_i128;
_6 = _20 as u16;
_2 = '\u{69698}';
_12 = [16408779952258618720_usize,5691397024650005607_usize,2_usize,1_usize,4_usize,2841936503204313666_usize,11654922967520939138_usize,7_usize];
_2 = '\u{12635}';
_22 = [_14.0,_6,_14.0,_6];
_4 = (-1397327086_i32) as i16;
_4 = !12781_i16;
_16 = _18 >> _18;
_2 = '\u{871d9}';
_6 = !_14.0;
_9 = 14489688674732484439_usize as i64;
_23 = _2;
_14 = (_6,);
_21 = (_13,);
_12 = [4_usize,17241413997353975813_usize,7_usize,7_usize,5005704815330455367_usize,10071275563181193832_usize,6062310668219727817_usize,5670896006460954547_usize];
_19 = [3_usize,17395720637156709798_usize,4_usize,3207991658147222066_usize,8855676327540856330_usize,12191059695077686237_usize,1_usize,4_usize];
_8 = false | false;
_22 = [_6,_14.0,_21.0,_6];
_4 = _20 as i16;
RET = [_10,_10,_10,_10,_10,_10];
_3 = _17 | _17;
_19 = _12;
_17 = _3;
_18 = _16 * _7;
_3 = !_17;
Call(_24 = fn3(_10, _2, _18, _11), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_25 = [_13,_6,_21.0,_13];
_23 = _2;
_9 = -(-516290866100571824_i64);
_19 = [7_usize,112238410888421555_usize,6826136047246719349_usize,7_usize,1_usize,1_usize,6_usize,17917448364414262099_usize];
_21 = _14;
Call(_15 = core::intrinsics::transmute(_8), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_21 = _14;
_8 = _23 <= _2;
_7 = _18 + _16;
_27 = (_11,);
_3 = !_17;
match _20 {
0 => bb5,
1 => bb6,
2 => bb8,
3 => bb11,
4 => bb14,
5 => bb15,
127 => bb17,
_ => bb16
}
}
bb14 = {
_13 = _14.0 + _14.0;
_10 = _3 as i128;
RET = [_10,_10,_10,_10,_10,_10];
_11 = [_14.0,_13,_13,_13,_13,_13,_14.0,_6];
Call(_1 = core::intrinsics::bswap(_5), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
_15 = _3 as u8;
_10 = 128753939791055764793852902946162756575_i128;
_6 = _20 as u16;
_2 = '\u{69698}';
_12 = [16408779952258618720_usize,5691397024650005607_usize,2_usize,1_usize,4_usize,2841936503204313666_usize,11654922967520939138_usize,7_usize];
_2 = '\u{12635}';
_22 = [_14.0,_6,_14.0,_6];
_4 = (-1397327086_i32) as i16;
_4 = !12781_i16;
_16 = _18 >> _18;
_2 = '\u{871d9}';
_6 = !_14.0;
_9 = 14489688674732484439_usize as i64;
_23 = _2;
_14 = (_6,);
_21 = (_13,);
_12 = [4_usize,17241413997353975813_usize,7_usize,7_usize,5005704815330455367_usize,10071275563181193832_usize,6062310668219727817_usize,5670896006460954547_usize];
_19 = [3_usize,17395720637156709798_usize,4_usize,3207991658147222066_usize,8855676327540856330_usize,12191059695077686237_usize,1_usize,4_usize];
_8 = false | false;
_22 = [_6,_14.0,_21.0,_6];
_4 = _20 as i16;
RET = [_10,_10,_10,_10,_10,_10];
_3 = _17 | _17;
_19 = _12;
_17 = _3;
_18 = _16 * _7;
_3 = !_17;
Call(_24 = fn3(_10, _2, _18, _11), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
_13 = _21.0 + _6;
_1 = _13 as u64;
_14 = _21;
_8 = _18 > _16;
_1 = 12420652710867314892_usize as u64;
SetDiscriminant(_24, 2);
_14.0 = _6;
_1 = _15 as u64;
_26 = core::ptr::addr_of_mut!(_4);
place!(Field::<u8>(Variant(_24, 2), 0)) = _15;
_12 = [3_usize,6446867285875618880_usize,10583361801740833090_usize,0_usize,7_usize,3_usize,3_usize,13822874509267300838_usize];
_15 = Field::<u8>(Variant(_24, 2), 0);
_28.1.0 = Field::<u8>(Variant(_24, 2), 0);
_17 = _3;
_20 = (-21_i8);
_28.1.1 = _3 as usize;
_33 = (_20,);
_33.0 = _20;
_28.1.3 = -_4;
_30 = [_28.1.1,_28.1.1,_28.1.1,_28.1.1,_28.1.1,_28.1.1,_28.1.1,_28.1.1];
Goto(bb18)
}
bb18 = {
Call(_34 = dump_var(2_usize, 13_usize, Move(_13), 22_usize, Move(_22), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(2_usize, 15_usize, Move(_15), 3_usize, Move(_3), 17_usize, Move(_17), 33_usize, Move(_33)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(2_usize, 4_usize, Move(_4), 27_usize, Move(_27), 25_usize, Move(_25), 23_usize, Move(_23)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_34 = dump_var(2_usize, 18_usize, Move(_18), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i128,mut _2: char,mut _3: isize,mut _4: [u16; 8]) -> Adt54 {
mir! {
type RET = Adt54;
let _5: [i128; 6];
let _6: f32;
let _7: i32;
let _8: Adt52;
let _9: Adt61;
let _10: ();
let _11: ();
{
RET = Adt54::Variant2 { fld0: 44_u8,fld1: _3 };
place!(Field::<u8>(Variant(RET, 2), 0)) = 28883_u16 as u8;
place!(Field::<isize>(Variant(RET, 2), 1)) = -_3;
_2 = '\u{77eb6}';
_1 = (-120450641364691533019400904570757515585_i128) ^ (-32559990546953280893631738187534216792_i128);
place!(Field::<isize>(Variant(RET, 2), 1)) = 4105266593311618810_u64 as isize;
SetDiscriminant(RET, 2);
place!(Field::<u8>(Variant(RET, 2), 0)) = 125_u8 ^ 141_u8;
_1 = -(-45576701987851165135879351111229181090_i128);
_3 = !9223372036854775807_isize;
place!(Field::<u8>(Variant(RET, 2), 0)) = !62_u8;
place!(Field::<u8>(Variant(RET, 2), 0)) = !184_u8;
place!(Field::<isize>(Variant(RET, 2), 1)) = !_3;
_6 = 4194282208_u32 as f32;
_9.fld3 = 12_i8 as u64;
_2 = '\u{35d50}';
place!(Field::<isize>(Variant(RET, 2), 1)) = _3 & _3;
place!(Field::<isize>(Variant(RET, 2), 1)) = 4611837218960350873_i64 as isize;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(3_usize, 2_usize, Move(_2), 4_usize, Move(_4), 11_usize, _11, 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: char,mut _2: [u16; 8],mut _3: [u64; 6],mut _4: [u64; 6],mut _5: i128,mut _6: [u64; 6],mut _7: u64,mut _8: [i8; 6],mut _9: u8,mut _10: usize) -> f64 {
mir! {
type RET = f64;
let _11: [usize; 8];
let _12: (i8,);
let _13: f64;
let _14: (isize, (i8,), u64, f32, i32, [u64; 6]);
let _15: u128;
let _16: isize;
let _17: u8;
let _18: ();
let _19: ();
{
_2 = [15475_u16,769_u16,46990_u16,10568_u16,39982_u16,51039_u16,63277_u16,20537_u16];
_4 = [_7,_7,_7,_7,_7,_7];
_2 = [13308_u16,17708_u16,30019_u16,42836_u16,63612_u16,4417_u16,28917_u16,25956_u16];
_6 = [_7,_7,_7,_7,_7,_7];
_5 = (-46292300902972265344406502896365223140_i128);
_7 = 13633014582050817748_u64 << _9;
_4 = [_7,_7,_7,_7,_7,_7];
_2 = [49726_u16,46597_u16,25736_u16,29807_u16,50612_u16,1471_u16,4505_u16,9116_u16];
_7 = !9166700298926188905_u64;
RET = _7 as f64;
_8 = [5_i8,22_i8,5_i8,(-21_i8),101_i8,37_i8];
RET = (-9223372036854775808_isize) as f64;
_6 = _3;
_1 = '\u{f51d}';
_4 = _3;
RET = _7 as f64;
_4 = _3;
_10 = 1006447823376791609_usize;
_7 = !4685207227685221890_u64;
_11 = [_10,_10,_10,_10,_10,_10,_10,_10];
_8 = [99_i8,(-100_i8),87_i8,7_i8,(-122_i8),(-42_i8)];
_14.1.0 = _1 as i8;
_14.0 = 268150087598545771440993183710076135996_u128 as isize;
_14.4 = !223466698_i32;
_12.0 = _14.1.0 >> _7;
_5 = (-89596565007097975147164391624690355182_i128);
RET = _10 as f64;
Call(_12 = fn5(_3, _7, _14.1, _4, _6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_12.0 = _14.1.0 ^ _14.1.0;
_14.0 = (-9223372036854775808_isize) >> _14.4;
_14.5 = [_7,_7,_7,_7,_7,_7];
_1 = '\u{26aff}';
_15 = 254839202006416788967539871631138267054_u128 + 79029169334931599776089469595920996232_u128;
_13 = _7 as f64;
_12.0 = _14.1.0;
_6 = [_7,_7,_7,_7,_7,_7];
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1006447823376791609 => bb9,
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
_12 = (_14.1.0,);
_14.1 = (_12.0,);
_14.0 = (-4_isize) >> _12.0;
_8 = [_14.1.0,_14.1.0,_12.0,_14.1.0,_12.0,_12.0];
_14.3 = 950660613_u32 as f32;
_16 = _1 as isize;
_14.1.0 = _12.0 - _12.0;
_16 = _14.0;
_14.1 = (_12.0,);
_14.1 = (_12.0,);
_9 = (-26046_i16) as u8;
_14.0 = _16 ^ _16;
_13 = RET;
_14.4 = -535974185_i32;
_14.2 = false as u64;
_12.0 = -_14.1.0;
_4 = [_7,_14.2,_7,_14.2,_14.2,_7];
_14.2 = _7;
_12 = (_14.1.0,);
_16 = !_14.0;
_5 = (-64420417799822540877427064258397668644_i128);
_17 = _9;
_1 = '\u{1bedb}';
_14.1.0 = 2651360479444968910_i64 as i8;
_14.2 = !_7;
_12.0 = !_14.1.0;
_4 = [_7,_7,_7,_7,_14.2,_14.2];
_1 = '\u{6509a}';
_10 = 8013696033849655528_usize;
_16 = _14.4 as isize;
_14.1.0 = (-3810197552098543393_i64) as i8;
Goto(bb10)
}
bb10 = {
_8 = [_14.1.0,_14.1.0,_14.1.0,_14.1.0,_14.1.0,_14.1.0];
_11 = [_10,_10,_10,_10,_10,_10,_10,_10];
_14.2 = _7 - _7;
_9 = _17 + _17;
_5 = _12.0 as i128;
match _10 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
8013696033849655528 => bb16,
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
_13 = -RET;
_13 = RET - RET;
_10 = 4_usize;
_14.3 = _9 as f32;
_11[_10] = 1517428952012194193_i64 as usize;
_7 = _3[_10];
_14.2 = !_3[_10];
_11[_10] = _10;
_14.0 = _5 as isize;
_9 = _17 + _17;
_4[_10] = _14.2;
_14.4 = _5 as i32;
_9 = _17 ^ _17;
_14.0 = _13 as isize;
_14.5[_10] = _4[_10];
RET = _13 - _13;
Goto(bb17)
}
bb17 = {
Call(_18 = dump_var(4_usize, 8_usize, Move(_8), 17_usize, Move(_17), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_18 = dump_var(4_usize, 10_usize, Move(_10), 9_usize, Move(_9), 4_usize, Move(_4), 19_usize, _19), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [u64; 6],mut _2: u64,mut _3: (i8,),mut _4: [u64; 6],mut _5: [u64; 6],mut _6: [u64; 6]) -> (i8,) {
mir! {
type RET = (i8,);
let _7: *mut f64;
let _8: Adt63;
let _9: char;
let _10: Adt63;
let _11: f64;
let _12: u32;
let _13: ();
let _14: ();
{
_3.0 = 9223372036854775807_isize as i8;
_4 = [_2,_2,_2,_2,_2,_2];
RET = (_3.0,);
RET.0 = _3.0 & _3.0;
_3.0 = !RET.0;
_3.0 = RET.0 - RET.0;
_4 = [_2,_2,_2,_2,_2,_2];
_4 = [_2,_2,_2,_2,_2,_2];
RET = (_3.0,);
_6 = [_2,_2,_2,_2,_2,_2];
Call(RET = fn6(_3, _3, _5, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = [_2,_2,_2,_2,_2,_2];
_9 = '\u{102737}';
_5 = [_2,_2,_2,_2,_2,_2];
_4 = [_2,_2,_2,_2,_2,_2];
RET = (_3.0,);
_5 = _1;
_5 = [_2,_2,_2,_2,_2,_2];
_6 = _5;
_9 = '\u{914c2}';
RET.0 = _3.0 | _3.0;
_4 = _1;
RET.0 = _3.0 >> _2;
_3 = RET;
_9 = '\u{16e35}';
_3.0 = 3061764621_u32 as i8;
_3.0 = RET.0 & RET.0;
_6 = [_2,_2,_2,_2,_2,_2];
RET.0 = -_3.0;
Goto(bb2)
}
bb2 = {
_7 = core::ptr::addr_of_mut!(_11);
RET = _3;
_4 = [_2,_2,_2,_2,_2,_2];
_9 = '\u{cd5e7}';
RET.0 = _3.0 - _3.0;
_9 = '\u{95655}';
_7 = core::ptr::addr_of_mut!((*_7));
_11 = (-9223372036854775808_isize) as f64;
_6 = [_2,_2,_2,_2,_2,_2];
_6 = [_2,_2,_2,_2,_2,_2];
Goto(bb3)
}
bb3 = {
Call(_13 = dump_var(5_usize, 2_usize, Move(_2), 6_usize, Move(_6), 3_usize, Move(_3), 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: (i8,),mut _2: (i8,),mut _3: [u64; 6],mut _4: u64) -> (i8,) {
mir! {
type RET = (i8,);
let _5: i16;
let _6: bool;
let _7: isize;
let _8: [u64; 6];
let _9: [u64; 6];
let _10: (*mut f64, char, bool);
let _11: [u16; 4];
let _12: (i64,);
let _13: u8;
let _14: ();
let _15: ();
{
_1.0 = _2.0;
_2 = (_1.0,);
RET = _2;
_2 = (RET.0,);
_3 = [_4,_4,_4,_4,_4,_4];
_6 = false;
_2.0 = _1.0;
_7 = 9223372036854775807_isize + (-9223372036854775808_isize);
_1 = (RET.0,);
_5 = 29587_i16;
_8 = [_4,_4,_4,_4,_4,_4];
RET.0 = !_1.0;
_1.0 = _2.0 + RET.0;
_6 = true;
_5 = _6 as i16;
Goto(bb1)
}
bb1 = {
_2 = (_1.0,);
RET.0 = _2.0;
_4 = !16975498490731963434_u64;
_1 = (_2.0,);
_6 = _2.0 == _1.0;
_6 = true ^ false;
_1 = (_2.0,);
RET.0 = _2.0 | _2.0;
_6 = false;
_7 = 9223372036854775807_isize;
_10.2 = !_6;
_3 = [_4,_4,_4,_4,_4,_4];
_12.0 = -6756149532186848284_i64;
_12 = (7609748170390966202_i64,);
_10.1 = '\u{53fb4}';
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(6_usize, 6_usize, Move(_6), 5_usize, Move(_5), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u8,mut _2: isize,mut _3: u64,mut _4: u32,mut _5: isize,mut _6: u16,mut _7: Adt51,mut _8: isize,mut _9: bool) -> (isize, (i8,), u64, f32, i32, [u64; 6]) {
mir! {
type RET = (isize, (i8,), u64, f32, i32, [u64; 6]);
let _10: (isize, (i8,), u64, f32, i32, [u64; 6]);
let _11: [i8; 6];
let _12: ();
let _13: ();
{
RET.2 = !_3;
_4 = !609618179_u32;
RET.5 = [_3,RET.2,RET.2,_3,_3,RET.2];
RET.2 = 2057557349_i32 as u64;
RET.4 = 1224154928_i32 ^ (-1148026986_i32);
RET.1.0 = Field::<(i8,)>(Variant(_7, 3), 0).0;
place!(Field::<i128>(Variant(_7, 3), 1)) = 21942705853441668509355245222678532196_i128 ^ 9186708752094465019294474040993546530_i128;
Goto(bb1)
}
bb1 = {
RET.4 = _6 as i32;
_4 = RET.1.0 as u32;
RET.0 = -_2;
_5 = !RET.0;
RET.1 = Field::<(i8,)>(Variant(_7, 3), 0);
_4 = 4242755547_u32 >> _5;
_1 = !139_u8;
RET.3 = 4456_i16 as f32;
RET.0 = _2 & _2;
place!(Field::<i128>(Variant(_7, 3), 1)) = -(-37316143239834795992840576632868105022_i128);
RET.5 = [RET.2,RET.2,RET.2,_3,_3,_3];
_10.5 = [RET.2,RET.2,_3,_3,RET.2,RET.2];
RET.5 = [_3,_3,RET.2,_3,RET.2,_3];
SetDiscriminant(_7, 1);
place!(Field::<(u8, usize, *mut u128, i16, i64)>(Variant(_7, 1), 1)).0 = !_1;
RET.1 = (97_i8,);
place!(Field::<(u8, usize, *mut u128, i16, i64)>(Variant(_7, 1), 1)).3 = !4781_i16;
_10.2 = _6 as u64;
_11 = [RET.1.0,RET.1.0,RET.1.0,RET.1.0,RET.1.0,RET.1.0];
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(7_usize, 8_usize, Move(_8), 4_usize, Move(_4), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{b7c39}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(11_i8), std::hint::black_box((-1726_i16)), std::hint::black_box(10635641196069147597_u64), std::hint::black_box(1513586077081851726_i64), std::hint::black_box((-142545671423916493611539479900150261140_i128)), std::hint::black_box(2_usize), std::hint::black_box(154_u8), std::hint::black_box(55948_u16), std::hint::black_box(2783792247_u32));
                
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt47 {
Variant0{
fld0: (isize, (i8,), u64, f32, i32, [u64; 6]),
fld1: *const *mut isize,
fld2: *const i16,

},
Variant1{
fld0: [i8; 6],
fld1: u64,
fld2: (usize, *mut u128),
fld3: (*mut f64, i32, i32, *mut i16),
fld4: *const *mut isize,
fld5: [i64; 8],

},
Variant2{
fld0: [i128; 6],
fld1: [usize; 8],
fld2: [u16; 4],
fld3: u128,
fld4: (*mut f64, char, bool),
fld5: u64,

},
Variant3{
fld0: *const u64,
fld1: u64,
fld2: (usize, *mut u128),
fld3: (*const u64,),
fld4: *mut i64,
fld5: u32,
fld6: u16,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt48 {
Variant0{
fld0: *mut f64,
fld1: u128,
fld2: f32,
fld3: i128,
fld4: *mut i16,

},
Variant1{
fld0: (*mut f64, i32, i32, *mut i16),
fld1: (*const i16, (u8, usize, *mut u128, i16, i64)),
fld2: usize,
fld3: u64,
fld4: u32,
fld5: i32,
fld6: u8,

}}
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: [u16; 8],
fld1: (i8,),
fld2: [i8; 6],
fld3: f64,
fld4: *const i16,
fld5: Adt47,
fld6: (*mut f64, char, bool),

},
Variant1{
fld0: (*mut f64, char, bool),
fld1: (*const i16, (u8, usize, *mut u128, i16, i64)),
fld2: (u8, usize, *mut u128, i16, i64),
fld3: [u16; 8],

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt50 {
Variant0{
fld0: *const u64,
fld1: [i128; 6],

},
Variant1{
fld0: (*mut f64, char, bool),
fld1: [u16; 4],

},
Variant2{
fld0: *const i16,
fld1: *const *mut isize,
fld2: u16,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt51 {
Variant0{
fld0: (i64,),
fld1: u32,
fld2: (*mut f64, i32, i32, *mut i16),
fld3: [u16; 4],
fld4: (u8, usize, *mut u128, i16, i64),
fld5: [u64; 6],

},
Variant1{
fld0: bool,
fld1: (u8, usize, *mut u128, i16, i64),

},
Variant2{
fld0: *const i16,
fld1: *mut f64,

},
Variant3{
fld0: (i8,),
fld1: i128,

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: (u16,),

},
Variant1{
fld0: *const *const u64,
fld1: char,
fld2: [u64; 6],
fld3: *const u64,
fld4: i128,
fld5: f32,

},
Variant2{
fld0: *const *const u64,

},
Variant3{
fld0: i16,
fld1: char,

}}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: bool,
fld1: u128,
fld2: [i64; 8],
fld3: Adt49,
fld4: usize,
fld5: *mut isize,
fld6: (*mut f64, char, bool),

},
Variant1{
fld0: *const i16,
fld1: Adt51,
fld2: (i64,),
fld3: (u16,),
fld4: [usize; 8],

}}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: (*const u64,),
fld1: u128,
fld2: i64,
fld3: i16,

},
Variant1{
fld0: Adt52,
fld1: u64,
fld2: [u64; 6],
fld3: *mut f64,
fld4: i16,
fld5: *mut isize,
fld6: Adt51,

},
Variant2{
fld0: u8,
fld1: isize,

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: (*const i16, (u8, usize, *mut u128, i16, i64)),
fld1: [u16; 4],
fld2: (i64,),
fld3: [usize; 8],
fld4: (isize, (i8,), u64, f32, i32, [u64; 6]),
fld5: [i8; 6],
fld6: *const i16,
fld7: (*mut f64, char, bool),

},
Variant1{
fld0: (usize, *mut u128),
fld1: i32,
fld2: u8,
fld3: *mut i64,
fld4: *const *const u64,

},
Variant2{
fld0: [usize; 8],
fld1: [u16; 4],
fld2: Adt47,

},
Variant3{
fld0: i64,
fld1: i128,
fld2: Adt49,
fld3: i8,
fld4: (*mut f64, i32, i32, *mut i16),
fld5: i32,

}}
#[derive(Debug)]
pub enum Adt56 {
Variant0{
fld0: bool,
fld1: u32,
fld2: usize,
fld3: *mut f64,

},
Variant1{
fld0: *mut u128,
fld1: (*const i16, (u8, usize, *mut u128, i16, i64)),
fld2: *const i16,
fld3: u16,
fld4: u32,
fld5: u8,

},
Variant2{
fld0: *mut u128,
fld1: *const i16,
fld2: i128,
fld3: i32,

},
Variant3{
fld0: Adt55,
fld1: Adt54,
fld2: *mut u128,
fld3: (*const u64,),
fld4: i16,
fld5: *mut i64,
fld6: (i64,),

}}
#[derive(Debug)]
pub enum Adt57 {
Variant0{
fld0: u64,
fld1: Adt51,
fld2: [usize; 8],
fld3: (*mut f64, i32, i32, *mut i16),
fld4: i16,
fld5: i32,

},
Variant1{
fld0: (i64,),
fld1: (isize, (i8,), u64, f32, i32, [u64; 6]),
fld2: Adt47,
fld3: *mut i16,
fld4: i16,
fld5: *const u64,

},
Variant2{
fld0: [i64; 8],
fld1: i32,
fld2: *mut i16,
fld3: (u8, usize, *mut u128, i16, i64),
fld4: *mut f64,

},
Variant3{
fld0: *mut i64,
fld1: (*const u64,),
fld2: Adt55,
fld3: i8,
fld4: i32,

}}
#[derive(Debug)]
pub enum Adt58 {
Variant0{
fld0: Adt51,
fld1: *mut i64,
fld2: (i8,),
fld3: Adt48,
fld4: *const *const u64,

},
Variant1{
fld0: (i8,),
fld1: *mut i16,
fld2: Adt52,

},
Variant2{
fld0: i32,
fld1: Adt51,

}}
#[derive(Debug)]
pub enum Adt59 {
Variant0{
fld0: bool,
fld1: (i8,),
fld2: *const i16,
fld3: i8,
fld4: i16,
fld5: (u16,),
fld6: u32,
fld7: *const *const u64,

},
Variant1{
fld0: Adt47,
fld1: (i8,),
fld2: *mut i64,
fld3: u32,

},
Variant2{
fld0: f32,
fld1: char,
fld2: isize,
fld3: (i64,),
fld4: i16,
fld5: u64,
fld6: [u16; 8],
fld7: u32,

}}
#[derive(Debug)]
pub enum Adt60 {
Variant0{
fld0: *mut i16,
fld1: [u16; 8],
fld2: *const u64,
fld3: [i64; 8],
fld4: Adt47,
fld5: *mut i64,

},
Variant1{
fld0: bool,
fld1: char,
fld2: [i64; 8],
fld3: Adt47,
fld4: usize,
fld5: (*const i16, (u8, usize, *mut u128, i16, i64)),
fld6: (isize, (i8,), u64, f32, i32, [u64; 6]),

},
Variant2{
fld0: (i8,),
fld1: char,
fld2: (*const i16, (u8, usize, *mut u128, i16, i64)),
fld3: Adt54,
fld4: u16,

}}
#[derive(Debug)]
pub struct Adt61 {
fld0: (*mut f64, i32, i32, *mut i16),
fld1: Adt47,
fld2: Adt52,
fld3: u64,
fld4: *const *mut isize,
}
#[derive(Debug)]
pub enum Adt62 {
Variant0{
fld0: Adt57,
fld1: [u64; 6],

},
Variant1{
fld0: ([u16; 8],),

},
Variant2{
fld0: Adt55,
fld1: (usize, *mut u128),
fld2: i32,
fld3: (*const u64,),

},
Variant3{
fld0: u32,
fld1: (*const u64,),
fld2: u64,
fld3: Adt58,
fld4: (i64,),
fld5: Adt50,
fld6: *mut isize,

}}
#[derive(Debug)]
pub enum Adt63 {
Variant0{
fld0: bool,
fld1: char,
fld2: [i64; 8],
fld3: (*const u64,),
fld4: i16,

},
Variant1{
fld0: Adt57,
fld1: Adt56,
fld2: Adt54,
fld3: i8,

},
Variant2{
fld0: Adt49,
fld1: Adt62,
fld2: Adt48,
fld3: f32,
fld4: (u8, usize, *mut u128, i16, i64),
fld5: Adt58,

}}

