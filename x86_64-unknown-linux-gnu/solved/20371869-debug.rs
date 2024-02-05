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
pub fn fn0(mut _1: usize,mut _2: u16,mut _3: i64,mut _4: i8,mut _5: u32,mut _6: u64) -> (u64, bool) {
mir! {
type RET = (u64, bool);
let _7: i16;
let _8: u32;
let _9: char;
let _10: [i16; 7];
let _11: [i8; 2];
let _12: ();
let _13: ();
{
RET.1 = (-10996_i16) >= 26722_i16;
_2 = 14638902089967228395_usize as u16;
_6 = 2914152325686884576_u64 >> _2;
_2 = 38715_u16;
RET.1 = !false;
_6 = 3684756539372313146_u64 << _2;
RET.0 = !_6;
RET.1 = _6 != _6;
_4 = -(-1_i8);
_7 = '\u{39d1a}' as i16;
_1 = 6_usize;
_1 = !1_usize;
Goto(bb1)
}
bb1 = {
_5 = 1590794955_u32 >> _6;
RET.1 = !true;
_3 = (-388891398035839772_i64) ^ 4008785855945068915_i64;
_1 = 2_usize ^ 4_usize;
_2 = !12124_u16;
RET.0 = !_6;
_4 = !95_i8;
RET = Checked(_6 * _6);
RET.1 = false;
Call(RET = fn1(_7, _6, _2, _2, _3, _5, _7, _2, _5, _1, _7, _3, _7, _7, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Goto(bb3)
}
bb3 = {
Call(_12 = dump_var(0_usize, 4_usize, Move(_4), 1_usize, Move(_1), 6_usize, Move(_6), 13_usize, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i16,mut _2: u64,mut _3: u16,mut _4: u16,mut _5: i64,mut _6: u32,mut _7: i16,mut _8: u16,mut _9: u32,mut _10: usize,mut _11: i16,mut _12: i64,mut _13: i16,mut _14: i16,mut _15: usize) -> (u64, bool) {
mir! {
type RET = (u64, bool);
let _16: bool;
let _17: (i16,);
let _18: isize;
let _19: i8;
let _20: i16;
let _21: char;
let _22: isize;
let _23: ();
let _24: ();
{
_4 = _11 as u16;
RET = Checked(_2 * _2);
_2 = true as u64;
_2 = _12 as u64;
_5 = _12;
_6 = _9 * _9;
_15 = _14 as usize;
_15 = !_10;
RET = (_2, true);
RET = (_2, false);
_8 = _9 as u16;
Goto(bb1)
}
bb1 = {
_6 = _9 >> _10;
RET = (_2, true);
_9 = _6;
_12 = -_5;
_9 = !_6;
_3 = '\u{7e822}' as u16;
_4 = _3 & _3;
_3 = _8;
_17.0 = (-69675843_i32) as i16;
RET.1 = !true;
Call(_11 = fn2(_17, _13, _10, _12, _14, _1, _17.0, _8, _4, _9, _17, _9, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15 = _10 % 8102435840851546745_usize;
_13 = _11 ^ _11;
_6 = _9;
_1 = _5 as i16;
_9 = !_6;
Goto(bb3)
}
bb3 = {
RET = Checked(_2 - _2);
_5 = _12;
RET.1 = true;
_4 = _3;
RET.0 = (-116503057_i32) as u64;
RET.1 = !false;
_16 = _13 == _11;
_1 = !_11;
_14 = !_13;
RET = (_2, _16);
_4 = _8 / 59331_u16;
_14 = !_11;
_16 = _11 != _14;
RET.0 = 127755475245300198081460737298055474192_u128 as u64;
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(1_usize, 12_usize, Move(_12), 14_usize, Move(_14), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(1_usize, 5_usize, Move(_5), 15_usize, Move(_15), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: (i16,),mut _2: i16,mut _3: usize,mut _4: i64,mut _5: i16,mut _6: i16,mut _7: i16,mut _8: u16,mut _9: u16,mut _10: u32,mut _11: (i16,),mut _12: u32,mut _13: i64) -> i16 {
mir! {
type RET = i16;
let _14: bool;
let _15: (f32, f64, *const f64, f64);
let _16: f32;
let _17: f64;
let _18: isize;
let _19: Adt57;
let _20: ();
let _21: ();
{
_1.0 = -_5;
RET = _2 | _1.0;
_1 = _11;
_10 = !_12;
_2 = 221949077430266125951021121524655322656_u128 as i16;
_1.0 = _2 >> _12;
_5 = _1.0 >> _10;
_11 = _1;
_14 = !true;
_1.0 = _2 & _5;
Call(_15 = fn3(_1.0, _12, _1.0, _7, _8, _7, _9, _3, _14), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = !_10;
_14 = !true;
_2 = _5 ^ _5;
_6 = _1.0;
_15.0 = _3 as f32;
_1.0 = _2;
_14 = !false;
_15.3 = _10 as f64;
_11.0 = 138640189481919386724450393711795727478_u128 as i16;
Goto(bb2)
}
bb2 = {
_15.3 = _15.1;
_9 = !_8;
_15.0 = (-9223372036854775808_isize) as f32;
_2 = _5;
_1 = (_5,);
_10 = _12;
_13 = _4;
_1 = (_11.0,);
_1.0 = _6 | _2;
_15.1 = _15.3 / (-0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002107556197424766_f64);
_10 = !_12;
_6 = -_1.0;
Goto(bb3)
}
bb3 = {
_2 = _5;
_15.3 = _15.1;
_5 = _6;
_14 = _2 < _6;
_11.0 = _1.0 >> _5;
_16 = (-32243399115439995663736675957569560956_i128) as f32;
_1.0 = _6 ^ _11.0;
_5 = _2 - _1.0;
RET = _5;
_18 = -(-9223372036854775808_isize);
_10 = _12;
_10 = _14 as u32;
_1.0 = _5 + _5;
_11 = (_5,);
_19.fld5.fld3 = _3;
_18 = -(-9223372036854775808_isize);
_19.fld5.fld0 = core::ptr::addr_of!(_15.3);
_18 = 9223372036854775807_isize;
_19.fld0 = _15;
_15.2 = _19.fld5.fld0;
_19.fld2.0 = _19.fld5.fld0;
_19.fld0.2 = core::ptr::addr_of!(_19.fld0.3);
Goto(bb4)
}
bb4 = {
Call(_20 = dump_var(2_usize, 18_usize, Move(_18), 11_usize, Move(_11), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_20 = dump_var(2_usize, 10_usize, Move(_10), 12_usize, Move(_12), 5_usize, Move(_5), 21_usize, _21), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: i16,mut _2: u32,mut _3: i16,mut _4: i16,mut _5: u16,mut _6: i16,mut _7: u16,mut _8: usize,mut _9: bool) -> (f32, f64, *const f64, f64) {
mir! {
type RET = (f32, f64, *const f64, f64);
let _10: [i16; 7];
let _11: [u8; 3];
let _12: isize;
let _13: (u64, bool);
let _14: Adt52;
let _15: [i16; 7];
let _16: ([i32; 6],);
let _17: (*const f64,);
let _18: i64;
let _19: (i16,);
let _20: [u8; 2];
let _21: *const u64;
let _22: *mut ((u64, bool),);
let _23: *const f64;
let _24: usize;
let _25: bool;
let _26: [i8; 2];
let _27: (*const f64,);
let _28: ((u64, bool),);
let _29: i8;
let _30: [i32; 6];
let _31: [char; 3];
let _32: isize;
let _33: [char; 3];
let _34: [i32; 6];
let _35: isize;
let _36: u32;
let _37: [i8; 2];
let _38: f64;
let _39: [usize; 3];
let _40: [u8; 2];
let _41: char;
let _42: Adt52;
let _43: ();
let _44: ();
{
_5 = _2 as u16;
_9 = !true;
_4 = _3 - _1;
_10 = [_4,_4,_4,_6,_4,_1,_1];
RET.0 = _8 as f32;
_5 = '\u{b8742}' as u16;
RET.1 = _8 as f64;
_14.fld2 = _5;
_11 = [231_u8,198_u8,139_u8];
_13.1 = !_9;
_12 = _8 as isize;
_11 = [82_u8,185_u8,66_u8];
_14.fld4 = core::ptr::addr_of!(_14.fld2);
_14.fld2 = '\u{80695}' as u16;
_15 = [_3,_4,_1,_3,_4,_3,_4];
Call(_14.fld2 = fn4(_15, _15, _2, _1, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13.1 = !_9;
RET.3 = _8 as f64;
RET.1 = (-76_i8) as f64;
_13 = (8440928286326866904_u64, _9);
match _13.0 {
8440928286326866904 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_14.fld2 = _13.1 as u16;
_15 = [_4,_3,_4,_1,_1,_4,_3];
RET.1 = 11_i8 as f64;
_19.0 = _4;
_6 = _4 << _3;
_18 = _13.0 as i64;
RET.1 = 230194330844842309266593544123411208774_u128 as f64;
_9 = _13.1;
_3 = '\u{f2678}' as i16;
_13.1 = !_9;
_3 = _1;
RET.1 = (-8456242788831236126831019676571768947_i128) as f64;
_14.fld3 = _13.0;
RET.3 = 118_u8 as f64;
_4 = _3 >> _6;
_6 = _5 as i16;
_14.fld2 = !_5;
_14.fld4 = core::ptr::addr_of!(_5);
Call(_5 = core::intrinsics::transmute(_7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = !_19.0;
_19 = (_4,);
_7 = _5 & _5;
_13 = (_14.fld3, _9);
_14.fld3 = 139_u8 as u64;
_7 = _5;
_14.fld3 = _13.0;
_9 = !_13.1;
_14.fld1 = -_18;
_16.0 = [(-1583702168_i32),(-1373469457_i32),(-1820675892_i32),(-1435729779_i32),(-1964358540_i32),425905527_i32];
_19 = (_4,);
_7 = _5 % 58257_u16;
_11 = [224_u8,126_u8,114_u8];
_14.fld4 = core::ptr::addr_of!(_7);
RET.3 = 160613491794921026525447677148486403445_u128 as f64;
_14.fld2 = !_7;
_10 = [_4,_1,_4,_4,_3,_19.0,_4];
Goto(bb5)
}
bb5 = {
_13.0 = !_14.fld3;
_11 = [213_u8,17_u8,24_u8];
_14.fld1 = -_18;
_3 = (-169654185005759481316572969966907334651_i128) as i16;
RET.1 = _13.0 as f64;
RET.1 = _13.0 as f64;
RET.1 = _12 as f64;
_19 = (_4,);
_18 = 1020698156_i32 as i64;
_15 = [_1,_4,_19.0,_1,_19.0,_4,_1];
RET.3 = _2 as f64;
_14.fld2 = _7;
_13 = (_14.fld3, _9);
match _14.fld3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
8440928286326866904 => bb10,
_ => bb9
}
}
bb6 = {
_1 = !_19.0;
_19 = (_4,);
_7 = _5 & _5;
_13 = (_14.fld3, _9);
_14.fld3 = 139_u8 as u64;
_7 = _5;
_14.fld3 = _13.0;
_9 = !_13.1;
_14.fld1 = -_18;
_16.0 = [(-1583702168_i32),(-1373469457_i32),(-1820675892_i32),(-1435729779_i32),(-1964358540_i32),425905527_i32];
_19 = (_4,);
_7 = _5 % 58257_u16;
_11 = [224_u8,126_u8,114_u8];
_14.fld4 = core::ptr::addr_of!(_7);
RET.3 = 160613491794921026525447677148486403445_u128 as f64;
_14.fld2 = !_7;
_10 = [_4,_1,_4,_4,_3,_19.0,_4];
Goto(bb5)
}
bb7 = {
_14.fld2 = _13.1 as u16;
_15 = [_4,_3,_4,_1,_1,_4,_3];
RET.1 = 11_i8 as f64;
_19.0 = _4;
_6 = _4 << _3;
_18 = _13.0 as i64;
RET.1 = 230194330844842309266593544123411208774_u128 as f64;
_9 = _13.1;
_3 = '\u{f2678}' as i16;
_13.1 = !_9;
_3 = _1;
RET.1 = (-8456242788831236126831019676571768947_i128) as f64;
_14.fld3 = _13.0;
RET.3 = 118_u8 as f64;
_4 = _3 >> _6;
_6 = _5 as i16;
_14.fld2 = !_5;
_14.fld4 = core::ptr::addr_of!(_5);
Call(_5 = core::intrinsics::transmute(_7), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_13.1 = !_9;
RET.3 = _8 as f64;
RET.1 = (-76_i8) as f64;
_13 = (8440928286326866904_u64, _9);
match _13.0 {
8440928286326866904 => bb3,
_ => bb2
}
}
bb10 = {
RET.0 = (-101_i8) as f32;
_12 = 88_isize;
_25 = !_9;
_16.0 = [479010630_i32,1798401066_i32,2271736_i32,(-1856735264_i32),(-627112546_i32),887006077_i32];
_14.fld3 = _18 as u64;
_9 = !_25;
_13.0 = _14.fld3;
_13 = (_14.fld3, _25);
_25 = _13.1 | _13.1;
_16.0 = [964715105_i32,319131281_i32,1509302205_i32,(-132298152_i32),(-229020256_i32),1175147742_i32];
RET.3 = _4 as f64;
RET.3 = _2 as f64;
_8 = !6_usize;
_10 = [_1,_19.0,_1,_1,_19.0,_19.0,_4];
_14.fld1 = !_18;
_28.0.0 = !_14.fld3;
_9 = _2 > _2;
RET.3 = (-17_i8) as f64;
_28 = (_13,);
_28.0 = (_14.fld3, _9);
RET.0 = _12 as f32;
_14.fld4 = core::ptr::addr_of!(_14.fld2);
match _12 {
0 => bb9,
1 => bb6,
2 => bb7,
3 => bb5,
4 => bb11,
88 => bb13,
_ => bb12
}
}
bb11 = {
_1 = !_19.0;
_19 = (_4,);
_7 = _5 & _5;
_13 = (_14.fld3, _9);
_14.fld3 = 139_u8 as u64;
_7 = _5;
_14.fld3 = _13.0;
_9 = !_13.1;
_14.fld1 = -_18;
_16.0 = [(-1583702168_i32),(-1373469457_i32),(-1820675892_i32),(-1435729779_i32),(-1964358540_i32),425905527_i32];
_19 = (_4,);
_7 = _5 % 58257_u16;
_11 = [224_u8,126_u8,114_u8];
_14.fld4 = core::ptr::addr_of!(_7);
RET.3 = 160613491794921026525447677148486403445_u128 as f64;
_14.fld2 = !_7;
_10 = [_4,_1,_4,_4,_3,_19.0,_4];
Goto(bb5)
}
bb12 = {
_1 = !_19.0;
_19 = (_4,);
_7 = _5 & _5;
_13 = (_14.fld3, _9);
_14.fld3 = 139_u8 as u64;
_7 = _5;
_14.fld3 = _13.0;
_9 = !_13.1;
_14.fld1 = -_18;
_16.0 = [(-1583702168_i32),(-1373469457_i32),(-1820675892_i32),(-1435729779_i32),(-1964358540_i32),425905527_i32];
_19 = (_4,);
_7 = _5 % 58257_u16;
_11 = [224_u8,126_u8,114_u8];
_14.fld4 = core::ptr::addr_of!(_7);
RET.3 = 160613491794921026525447677148486403445_u128 as f64;
_14.fld2 = !_7;
_10 = [_4,_1,_4,_4,_3,_19.0,_4];
Goto(bb5)
}
bb13 = {
_21 = core::ptr::addr_of!(_14.fld3);
_8 = 241_u8 as usize;
_20 = [0_u8,165_u8];
_21 = core::ptr::addr_of!((*_21));
_14.fld1 = (-110_i8) as i64;
RET.0 = _13.0 as f32;
_14.fld2 = _5;
_10 = [_4,_4,_4,_4,_1,_1,_1];
RET.1 = 100_u8 as f64;
_13.0 = _1 as u64;
_19.0 = !_4;
_9 = _25;
_14.fld1 = _19.0 as i64;
_19 = (_4,);
_30 = [(-438731317_i32),(-804830478_i32),702582802_i32,(-617961095_i32),(-2092435042_i32),(-2083688076_i32)];
_29 = 36_i8 + (-29_i8);
match _12 {
88 => bb14,
_ => bb7
}
}
bb14 = {
_8 = !7561234362892620777_usize;
_15 = [_4,_4,_1,_4,_19.0,_19.0,_1];
_28.0.0 = _13.0;
RET.0 = 177_u8 as f32;
_26 = [_29,_29];
Goto(bb15)
}
bb15 = {
_28.0.1 = !_9;
_14.fld3 = !_13.0;
(*_21) = _28.0.0;
_14.fld3 = _7 as u64;
_16.0 = [1361597046_i32,1648752598_i32,6275071_i32,(-436506819_i32),(-1701193152_i32),(-2049873956_i32)];
_14.fld0 = core::ptr::addr_of_mut!(_28);
_25 = _28.0.0 < _28.0.0;
_19 = (_4,);
_14.fld4 = core::ptr::addr_of!(_7);
_18 = _14.fld1 >> _19.0;
_30 = [(-852128223_i32),(-414681769_i32),409151602_i32,912565677_i32,(-924362382_i32),(-1904181204_i32)];
_1 = _4 - _4;
_14.fld1 = _18;
_34 = _16.0;
_22 = _14.fld0;
_15 = _10;
_20 = [28_u8,81_u8];
_14.fld1 = _18;
_31 = ['\u{115f0}','\u{c98d1}','\u{8340c}'];
_16.0 = _34;
_10 = _15;
Goto(bb16)
}
bb16 = {
RET.3 = _28.0.0 as f64;
_6 = -_19.0;
RET.1 = 47_u8 as f64;
_18 = _14.fld1;
_13.1 = _25;
_29 = -(-93_i8);
_16 = (_34,);
(*_22).0 = ((*_21), _13.1);
(*_22).0.0 = _8 as u64;
_31 = ['\u{b09c9}','\u{3011c}','\u{10fb5d}'];
_9 = _1 <= _1;
_5 = (-34579329713785829763882848382463933237_i128) as u16;
match _12 {
0 => bb11,
1 => bb10,
2 => bb3,
3 => bb9,
4 => bb12,
5 => bb17,
6 => bb18,
88 => bb20,
_ => bb19
}
}
bb17 = {
Return()
}
bb18 = {
_8 = !7561234362892620777_usize;
_15 = [_4,_4,_1,_4,_19.0,_19.0,_1];
_28.0.0 = _13.0;
RET.0 = 177_u8 as f32;
_26 = [_29,_29];
Goto(bb15)
}
bb19 = {
_1 = !_19.0;
_19 = (_4,);
_7 = _5 & _5;
_13 = (_14.fld3, _9);
_14.fld3 = 139_u8 as u64;
_7 = _5;
_14.fld3 = _13.0;
_9 = !_13.1;
_14.fld1 = -_18;
_16.0 = [(-1583702168_i32),(-1373469457_i32),(-1820675892_i32),(-1435729779_i32),(-1964358540_i32),425905527_i32];
_19 = (_4,);
_7 = _5 % 58257_u16;
_11 = [224_u8,126_u8,114_u8];
_14.fld4 = core::ptr::addr_of!(_7);
RET.3 = 160613491794921026525447677148486403445_u128 as f64;
_14.fld2 = !_7;
_10 = [_4,_1,_4,_4,_3,_19.0,_4];
Goto(bb5)
}
bb20 = {
_33 = ['\u{341b7}','\u{95905}','\u{b7216}'];
RET.3 = _5 as f64;
_1 = _6 - _19.0;
(*_22).0 = _13;
_19 = (_1,);
Goto(bb21)
}
bb21 = {
RET.2 = core::ptr::addr_of!(_38);
_14.fld4 = core::ptr::addr_of!(_7);
_14.fld2 = (*_22).0.0 as u16;
(*_22) = (_13,);
_6 = !_1;
_24 = _8 + _8;
RET.3 = _14.fld1 as f64;
_5 = _14.fld2 ^ _14.fld2;
_27.0 = core::ptr::addr_of!(_38);
_17.0 = core::ptr::addr_of!(_38);
_14.fld1 = _18;
(*_21) = (*_22).0.0 ^ (*_22).0.0;
_32 = 247_u8 as isize;
Goto(bb22)
}
bb22 = {
Call(_43 = dump_var(3_usize, 3_usize, Move(_3), 6_usize, Move(_6), 26_usize, Move(_26), 1_usize, Move(_1)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_43 = dump_var(3_usize, 2_usize, Move(_2), 15_usize, Move(_15), 24_usize, Move(_24), 20_usize, Move(_20)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_43 = dump_var(3_usize, 25_usize, Move(_25), 5_usize, Move(_5), 28_usize, Move(_28), 12_usize, Move(_12)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_43 = dump_var(3_usize, 29_usize, Move(_29), 7_usize, Move(_7), 44_usize, _44, 44_usize, _44), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [i16; 7],mut _2: [i16; 7],mut _3: u32,mut _4: i16,mut _5: usize) -> u16 {
mir! {
type RET = u16;
let _6: i16;
let _7: Adt53;
let _8: Adt56;
let _9: [i16; 7];
let _10: char;
let _11: bool;
let _12: i64;
let _13: ([i32; 6],);
let _14: (u64, bool);
let _15: *mut i128;
let _16: u8;
let _17: [u8; 3];
let _18: char;
let _19: Adt62;
let _20: char;
let _21: [i32; 6];
let _22: Adt49;
let _23: char;
let _24: Adt55;
let _25: u32;
let _26: f32;
let _27: Adt52;
let _28: ([i32; 6],);
let _29: ([i32; 6],);
let _30: Adt50;
let _31: bool;
let _32: i8;
let _33: f32;
let _34: [char; 3];
let _35: Adt52;
let _36: (*const u128, (f32, i32, [char; 3]), usize, f64);
let _37: [u8; 3];
let _38: u8;
let _39: *const f64;
let _40: [i32; 6];
let _41: ();
let _42: ();
{
RET = 17262_u16;
RET = 5523_u16 ^ 18445_u16;
_1 = [_4,_4,_4,_4,_4,_4,_4];
RET = 16261_u16;
_3 = 1569665191_u32;
_6 = _5 as i16;
Call(_7.fld1.0 = fn5(_5, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.fld0 = 126_u8 - 33_u8;
_3 = !1355400529_u32;
_7.fld1.0 = _4 >> _6;
_7.fld4 = !_7.fld1.0;
_6 = (-68456599014106557143546399342757959529_i128) as i16;
_7.fld3.1 = !(-601461101_i32);
_7.fld2 = _3;
_6 = _7.fld1.0;
_7.fld3.0 = 15942634913638615744_u64 as f32;
_8.fld2.fld0 = !_7.fld0;
_7.fld4 = 89_i8 as i16;
_11 = false;
_3 = _7.fld2 / 3582765561_u32;
_8.fld2.fld1 = (_6,);
_7.fld3.0 = 20264_u16 as f32;
_8.fld1.2 = [_5,_5,_5];
_8.fld2.fld2 = _3;
_8.fld1.0 = _3;
_8.fld2.fld3.2 = ['\u{cee9a}','\u{72243}','\u{2abec}'];
_8.fld2.fld3.1 = _7.fld3.1 + _7.fld3.1;
_5 = 6319469596027113805_usize ^ 1_usize;
_8.fld2.fld1 = (_6,);
_8.fld2.fld4 = -_7.fld1.0;
_8.fld2.fld3.2 = ['\u{a296a}','\u{ecc6a}','\u{8ce0c}'];
_8.fld2.fld4 = 1669075708860257127_i64 as i16;
Goto(bb2)
}
bb2 = {
_7.fld4 = _4 >> _8.fld2.fld1.0;
_7.fld3.0 = _5 as f32;
_9 = [_4,_7.fld1.0,_6,_7.fld4,_4,_7.fld1.0,_8.fld2.fld1.0];
RET = 199892847414941459238328932077879030861_u128 as u16;
_11 = true;
_10 = '\u{3e380}';
Call(_10 = fn19(_7.fld4, _8.fld2.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8.fld2.fld3.0 = _7.fld3.0 - _7.fld3.0;
_8.fld2.fld0 = _7.fld0 - _7.fld0;
_7.fld2 = _7.fld4 as u32;
_7.fld1.0 = _7.fld4;
RET = 25412_u16;
_7.fld3 = (_8.fld2.fld3.0, _8.fld2.fld3.1, _8.fld2.fld3.2);
_1 = _9;
_8.fld2.fld3.2 = [_10,_10,_10];
_8.fld1.2 = [_5,_5,_5];
Goto(bb4)
}
bb4 = {
_8.fld2 = Adt53 { fld0: _7.fld0,fld1: _7.fld1,fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4 };
_8.fld1.0 = _8.fld2.fld2 % 1253590777_u32;
_5 = 7_usize & 1_usize;
_5 = 3_usize;
_9[_5] = (-90_i8) as i16;
_7.fld0 = !_8.fld2.fld0;
_7.fld4 = _1[_5];
_8.fld1.4 = core::ptr::addr_of_mut!(_13.0);
_14.0 = 5447957803300738361_u64 / 4158885755548552831_u64;
_9 = [_8.fld2.fld4,_7.fld1.0,_7.fld1.0,_7.fld1.0,_2[_5],_7.fld1.0,_1[_5]];
_8.fld1.1 = core::ptr::addr_of_mut!(_8.fld2.fld3.2);
_3 = (-3165905593202640468_i64) as u32;
_12 = !(-5524915544380768222_i64);
_5 = _12 as usize;
RET = (-108_i8) as u16;
_7.fld3 = (_8.fld2.fld3.0, _8.fld2.fld3.1, _8.fld2.fld3.2);
_8.fld1.1 = core::ptr::addr_of_mut!(_7.fld3.2);
_8.fld0 = _12 as u128;
_14.0 = 4875398437152259045_u64;
_7.fld3.0 = _7.fld2 as f32;
_8.fld1.4 = core::ptr::addr_of_mut!(_13.0);
_14.0 = _10 as u64;
Goto(bb5)
}
bb5 = {
_8.fld1.4 = core::ptr::addr_of_mut!(_13.0);
_8.fld2.fld1.0 = 36325_u16 as i16;
_18 = _10;
_8.fld2.fld3.2 = [_10,_18,_10];
_19 = Adt62 { fld0: (-76_i8) };
_7.fld4 = _14.0 as i16;
_8.fld2.fld2 = _8.fld2.fld0 as u32;
_4 = !_6;
_12 = -(-824806863702662438_i64);
_3 = _8.fld1.0 & _8.fld1.0;
_14 = Checked(12422551141002816633_u64 * 1793655648835870520_u64);
_13.0 = [_7.fld3.1,_7.fld3.1,_7.fld3.1,_8.fld2.fld3.1,_7.fld3.1,_7.fld3.1];
_7.fld3.0 = _8.fld2.fld3.0 * _8.fld2.fld3.0;
_8.fld2.fld3.1 = _7.fld3.1 >> _3;
_8.fld1.0 = _3 - _3;
_13.0 = [_8.fld2.fld3.1,_8.fld2.fld3.1,_8.fld2.fld3.1,_8.fld2.fld3.1,_8.fld2.fld3.1,_8.fld2.fld3.1];
_8.fld1.4 = core::ptr::addr_of_mut!(_13.0);
_7.fld3 = (_8.fld2.fld3.0, _8.fld2.fld3.1, _8.fld2.fld3.2);
_22.fld2.1.3 = !_8.fld0;
_22.fld6 = [_8.fld2.fld0,_8.fld2.fld0];
_16 = _8.fld2.fld3.0 as u8;
_22.fld2.0 = core::ptr::addr_of_mut!(_13.0);
_19.fld0 = (-106_i8) + (-121_i8);
_17 = [_8.fld2.fld0,_8.fld2.fld0,_16];
Call(_6 = core::intrinsics::bswap(_7.fld1.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_18 = _10;
_3 = _7.fld3.0 as u32;
_13.0 = [_7.fld3.1,_7.fld3.1,_8.fld2.fld3.1,_8.fld2.fld3.1,_7.fld3.1,_7.fld3.1];
_22.fld2.1 = (_7.fld3.1, _8.fld2.fld3.1, _14.0, _8.fld0);
_24.fld4.0 = _8.fld1.0 as f32;
_7.fld1 = _8.fld2.fld1;
_25 = !_8.fld1.0;
_8.fld1.1 = core::ptr::addr_of_mut!(_7.fld3.2);
_22.fld2.0 = core::ptr::addr_of_mut!(_21);
_22.fld7.0 = core::ptr::addr_of!(_24.fld4.3);
_7 = Adt53 { fld0: _16,fld1: _8.fld2.fld1,fld2: _25,fld3: _8.fld2.fld3,fld4: _8.fld2.fld1.0 };
_7 = Adt53 { fld0: _8.fld2.fld0,fld1: _8.fld2.fld1,fld2: _25,fld3: _8.fld2.fld3,fld4: _8.fld2.fld4 };
_7.fld2 = _22.fld2.1.1 as u32;
Goto(bb7)
}
bb7 = {
_18 = _10;
_11 = _8.fld2.fld3.1 <= _7.fld3.1;
_8.fld2.fld4 = -_7.fld4;
_22.fld6 = [_8.fld2.fld0,_7.fld0];
_24.fld5.0 = _24.fld4.0;
_7.fld2 = _8.fld1.0 | _25;
_22.fld1 = _18;
_8.fld2.fld2 = _25;
Goto(bb8)
}
bb8 = {
_24.fld3 = _5;
_12 = 971559289624339696_i64;
_26 = _24.fld4.0 / f32::NAN;
_8.fld1.1 = core::ptr::addr_of_mut!(_7.fld3.2);
_7 = Move(_8.fld2);
_22.fld6 = [_7.fld0,_7.fld0];
_7.fld3.0 = _24.fld5.0 / 1_f32;
_24.fld5 = _7.fld3;
_14 = Checked(_22.fld2.1.2 + _22.fld2.1.2);
_27.fld3 = _14.0 | _22.fld2.1.2;
_22.fld4 = core::ptr::addr_of_mut!(_22.fld2);
_7.fld1.0 = _7.fld4 & _7.fld4;
_14.1 = !_11;
_23 = _22.fld1;
_24.fld4.2 = core::ptr::addr_of!(_24.fld4.1);
_7.fld3.2 = _24.fld5.2;
_4 = _7.fld1.0;
_22.fld0 = (_7.fld4,);
_7 = Adt53 { fld0: _16,fld1: _22.fld0,fld2: _25,fld3: _24.fld5,fld4: _22.fld0.0 };
Goto(bb9)
}
bb9 = {
_24.fld4.1 = _24.fld5.0 as f64;
_27.fld1 = !_12;
_24.fld2.fld0 = core::ptr::addr_of_mut!(_18);
_7.fld3.2 = [_10,_23,_10];
_22.fld1 = _10;
_5 = !_24.fld3;
_24.fld1 = [_24.fld3,_24.fld3,_5];
_22.fld2.2 = [_7.fld0,_16];
_22.fld2.2 = _22.fld6;
_19.fld0 = _6 as i8;
_8.fld1.3 = (_24.fld4.2,);
_27.fld2 = _7.fld2 as u16;
_24.fld4.3 = _24.fld4.1;
_22.fld7 = (_24.fld4.2,);
_24.fld5.1 = !_7.fld3.1;
_24.fld2.fld0 = core::ptr::addr_of_mut!(_10);
_28 = (_13.0,);
_24.fld5 = (_7.fld3.0, _22.fld2.1.1, _7.fld3.2);
_7.fld1 = (_4,);
_13.0 = _28.0;
_16 = !_7.fld0;
_22.fld2.3 = !_11;
_24.fld5.1 = _26 as i32;
_30.fld2 = _8.fld1.1;
_13 = _28;
_7.fld2 = !_25;
Goto(bb10)
}
bb10 = {
_27.fld4 = core::ptr::addr_of!(_27.fld2);
Goto(bb11)
}
bb11 = {
_31 = !_11;
_24.fld0 = _8.fld1.1;
_13 = _28;
_8.fld1.2 = _24.fld1;
_26 = -_24.fld5.0;
_17 = [_7.fld0,_7.fld0,_7.fld0];
_8.fld0 = !_22.fld2.1.3;
_29.0 = [_24.fld5.1,_22.fld2.1.1,_22.fld2.1.0,_24.fld5.1,_24.fld5.1,_7.fld3.1];
_30.fld3 = (_26, _24.fld4.1, _22.fld7.0, _24.fld4.3);
_28.0 = [_22.fld2.1.1,_24.fld5.1,_24.fld5.1,_24.fld5.1,_24.fld5.1,_22.fld2.1.1];
_22.fld5 = core::ptr::addr_of!(_27.fld2);
_22.fld2.1.2 = _27.fld3;
_30.fld3.3 = _30.fld3.1;
_22.fld2.1.3 = _19.fld0 as u128;
_35.fld3 = _7.fld1.0 as u64;
_34 = [_22.fld1,_10,_22.fld1];
_21 = _28.0;
_30.fld1 = _22.fld2.2;
_24.fld0 = core::ptr::addr_of_mut!(_24.fld5.2);
_23 = _18;
Call(_35.fld4 = core::intrinsics::arith_offset(_27.fld4, (-53_isize)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_24.fld5.0 = -_7.fld3.0;
_33 = _26;
_35.fld2 = !_27.fld2;
_19 = Adt62 { fld0: 97_i8 };
_24.fld5.2 = [_18,_10,_22.fld1];
_27.fld2 = !_35.fld2;
_24.fld5.1 = _22.fld2.1.1 >> _7.fld2;
_29 = (_28.0,);
_35.fld1 = _27.fld1 << _35.fld2;
_3 = _7.fld2;
_30.fld2 = core::ptr::addr_of_mut!(_36.1.2);
_36.2 = _5 - _5;
_17 = [_16,_16,_7.fld0];
_7.fld3.1 = -_22.fld2.1.1;
_22.fld2.1.0 = _19.fld0 as i32;
_7.fld1.0 = -_7.fld4;
_22.fld2.1.2 = _35.fld3 + _35.fld3;
Goto(bb13)
}
bb13 = {
_4 = -_7.fld4;
_26 = _30.fld3.0;
_24.fld1 = _8.fld1.2;
_35.fld1 = _12;
_22.fld2.2 = [_7.fld0,_7.fld0];
_17 = [_16,_7.fld0,_16];
_1 = [_4,_22.fld0.0,_6,_22.fld0.0,_7.fld1.0,_4,_22.fld0.0];
_30.fld3.1 = _24.fld4.1;
_37 = [_16,_16,_16];
_19 = Adt62 { fld0: 20_i8 };
_22.fld2.3 = _31;
_20 = _10;
_22.fld3 = !_24.fld3;
_38 = !_16;
_19.fld0 = (-61_i8);
_7.fld2 = !_25;
_19.fld0 = 60072933798234650691860636642261423162_i128 as i8;
_24.fld4.0 = -_26;
_35.fld5 = core::ptr::addr_of_mut!(_22.fld2);
_14 = (_27.fld3, _11);
_33 = -_26;
_36.1.1 = _27.fld2 as i32;
_27.fld4 = _35.fld4;
_35.fld2 = _27.fld2;
_22.fld7.0 = core::ptr::addr_of!(_30.fld3.1);
_11 = _31;
_30.fld3.2 = core::ptr::addr_of!(_24.fld4.3);
_22.fld2.0 = _8.fld1.4;
match _12 {
971559289624339696 => bb14,
_ => bb11
}
}
bb14 = {
_27.fld5 = core::ptr::addr_of_mut!(_22.fld2);
_38 = _35.fld2 as u8;
_24.fld5.2 = _34;
_8.fld1.3.0 = core::ptr::addr_of!(_30.fld3.3);
_22.fld2.2 = [_38,_38];
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(4_usize, 34_usize, Move(_34), 38_usize, Move(_38), 23_usize, Move(_23), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(4_usize, 9_usize, Move(_9), 10_usize, Move(_10), 2_usize, Move(_2), 37_usize, Move(_37)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(4_usize, 20_usize, Move(_20), 6_usize, Move(_6), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: usize,mut _2: i16) -> i16 {
mir! {
type RET = i16;
let _3: Adt62;
let _4: char;
let _5: isize;
let _6: char;
let _7: (i16,);
let _8: Adt59;
let _9: u32;
let _10: [u8; 2];
let _11: [char; 3];
let _12: u64;
let _13: [u8; 2];
let _14: [usize; 3];
let _15: f64;
let _16: f64;
let _17: f64;
let _18: *const u64;
let _19: isize;
let _20: [char; 3];
let _21: Adt61;
let _22: *mut [char; 3];
let _23: u32;
let _24: f64;
let _25: usize;
let _26: (*const u128, (f32, i32, [char; 3]), usize, f64);
let _27: [u8; 2];
let _28: ();
let _29: ();
{
_1 = 5_usize + 6_usize;
_2 = (-7116_i16);
RET = _2 | _2;
RET = _2 * _2;
_2 = (-11777_i16) >> _1;
_2 = 28234_i16;
_3.fld0 = 106_i8 - 27_i8;
_2 = 26937_i16;
_3 = Adt62 { fld0: (-80_i8) };
RET = 3210219047_u32 as i16;
RET = _2 - _2;
_2 = -11329_i16;
Call(_2 = core::intrinsics::bswap(20304_i16), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 1160266902824238680_usize & 17889812905321586435_usize;
_3.fld0 = !(-18_i8);
_4 = '\u{2a0aa}';
_4 = '\u{1336b}';
_4 = '\u{e03df}';
_3.fld0 = 102_u8 as i8;
_3 = Adt62 { fld0: (-57_i8) };
_3 = Adt62 { fld0: 75_i8 };
RET = !_2;
_7.0 = 3001818798612898471552733564088909548_i128 as i16;
_6 = _4;
_8.fld3 = 250983299698451301176031323027547199364_u128;
_7 = (_2,);
_7 = (_2,);
_2 = _7.0 & _7.0;
_8.fld2 = (-9223372036854775808_isize) + 73_isize;
RET = !_7.0;
_8.fld6 = _8.fld2 as u16;
_3 = Adt62 { fld0: (-4_i8) };
_8.fld2 = 9223372036854775807_isize;
_3.fld0 = (-122_i8);
_7 = (_2,);
_2 = _8.fld3 as i16;
_3.fld0 = 100_i8 + (-30_i8);
_9 = (-5005975537286497856_i64) as u32;
match _8.fld2 {
0 => bb2,
9223372036854775807 => bb4,
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
_5 = _8.fld2;
_2 = _8.fld6 as i16;
_9 = 1346319791_u32 - 3583033881_u32;
_8.fld6 = 8236_u16;
_1 = 12782943552374593362_usize;
_8.fld2 = _5;
_10 = [82_u8,156_u8];
_3.fld0 = _2 as i8;
_4 = _6;
RET = _7.0 | _2;
_12 = 14618864755910329661_u64 ^ 15889991442449372245_u64;
_8.fld2 = _5 - _5;
_11 = [_6,_4,_4];
RET = _7.0 | _2;
_3 = Adt62 { fld0: 121_i8 };
_14 = [_1,_1,_1];
_8.fld3 = !285915798807383618576239896028792397844_u128;
Call(_8.fld4 = fn6(_5, _3.fld0, _12, _8.fld6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8.fld4 = core::ptr::addr_of!(_12);
RET = _7.0;
RET = _7.0;
_1 = 11299397764166432084_usize / 2_usize;
_5 = _12 as isize;
_13 = _10;
Goto(bb6)
}
bb6 = {
_13 = _10;
_8.fld2 = _5;
_10 = [153_u8,169_u8];
_8.fld1.0 = core::ptr::addr_of!(_15);
_8.fld3 = !86767857391169874290523233349898574352_u128;
_5 = -_8.fld2;
_8.fld0 = core::ptr::addr_of!(_15);
_15 = _1 as f64;
_8.fld1 = (_8.fld0,);
_16 = _7.0 as f64;
RET = _7.0 ^ _2;
_14 = [_1,_1,_1];
_15 = _16;
_17 = -_15;
_8.fld3 = _4 as u128;
match _8.fld6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
8236 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_10 = [155_u8,5_u8];
_16 = _15 * _17;
RET = 1630873444_i32 as i16;
_3 = Adt62 { fld0: 43_i8 };
_15 = 5099278406028997347_i64 as f64;
_3.fld0 = 1849466028_i32 as i8;
Goto(bb9)
}
bb9 = {
_21.fld1.fld5.fld3 = !_1;
_21.fld1.fld0.3 = _8.fld6 as f64;
_10 = [110_u8,36_u8];
_21.fld1.fld3 = [_1,_21.fld1.fld5.fld3,_1];
_21.fld1.fld0.0 = _15 as f32;
_8.fld6 = 26248_u16 * 52511_u16;
_16 = _21.fld1.fld0.3 + _17;
_8.fld3 = 333408846597757586416608156949626997881_u128;
_18 = core::ptr::addr_of!(_12);
_3.fld0 = 110_i8;
_21.fld1.fld5.fld3 = !_1;
_7.0 = _2 | _2;
(*_18) = 12914364733642024354_u64 ^ 9438069017367969985_u64;
_24 = _16 * _16;
_21.fld1.fld1.0 = [1716891380_i32,1750628725_i32,1065749293_i32,1609927264_i32,400855861_i32,(-1365550918_i32)];
_2 = _9 as i16;
_8.fld2 = _5 - _5;
_8.fld6 = 1506942995_i32 as u16;
_21.fld1.fld0.1 = -_16;
_8.fld4 = core::ptr::addr_of!((*_18));
_14 = [_21.fld1.fld5.fld3,_21.fld1.fld5.fld3,_1];
_2 = _7.0;
match _3.fld0 {
0 => bb6,
1 => bb8,
2 => bb4,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
110 => bb15,
_ => bb14
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_1 = 1160266902824238680_usize & 17889812905321586435_usize;
_3.fld0 = !(-18_i8);
_4 = '\u{2a0aa}';
_4 = '\u{1336b}';
_4 = '\u{e03df}';
_3.fld0 = 102_u8 as i8;
_3 = Adt62 { fld0: (-57_i8) };
_3 = Adt62 { fld0: 75_i8 };
RET = !_2;
_7.0 = 3001818798612898471552733564088909548_i128 as i16;
_6 = _4;
_8.fld3 = 250983299698451301176031323027547199364_u128;
_7 = (_2,);
_7 = (_2,);
_2 = _7.0 & _7.0;
_8.fld2 = (-9223372036854775808_isize) + 73_isize;
RET = !_7.0;
_8.fld6 = _8.fld2 as u16;
_3 = Adt62 { fld0: (-4_i8) };
_8.fld2 = 9223372036854775807_isize;
_3.fld0 = (-122_i8);
_7 = (_2,);
_2 = _8.fld3 as i16;
_3.fld0 = 100_i8 + (-30_i8);
_9 = (-5005975537286497856_i64) as u32;
match _8.fld2 {
0 => bb2,
9223372036854775807 => bb4,
_ => bb3
}
}
bb13 = {
_8.fld4 = core::ptr::addr_of!(_12);
RET = _7.0;
RET = _7.0;
_1 = 11299397764166432084_usize / 2_usize;
_5 = _12 as isize;
_13 = _10;
Goto(bb6)
}
bb14 = {
_5 = _8.fld2;
_2 = _8.fld6 as i16;
_9 = 1346319791_u32 - 3583033881_u32;
_8.fld6 = 8236_u16;
_1 = 12782943552374593362_usize;
_8.fld2 = _5;
_10 = [82_u8,156_u8];
_3.fld0 = _2 as i8;
_4 = _6;
RET = _7.0 | _2;
_12 = 14618864755910329661_u64 ^ 15889991442449372245_u64;
_8.fld2 = _5 - _5;
_11 = [_6,_4,_4];
RET = _7.0 | _2;
_3 = Adt62 { fld0: 121_i8 };
_14 = [_1,_1,_1];
_8.fld3 = !285915798807383618576239896028792397844_u128;
Call(_8.fld4 = fn6(_5, _3.fld0, _12, _8.fld6), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_26.0 = core::ptr::addr_of!(_8.fld3);
_20 = [_6,_6,_6];
_13 = _10;
_21.fld1.fld2.0 = _8.fld0;
_9 = 1683719191_u32;
_26.2 = !_21.fld1.fld5.fld3;
_24 = 92301533537171402330559132209619232594_i128 as f64;
_26.1.1 = -1359161349_i32;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(5_usize, 9_usize, Move(_9), 14_usize, Move(_14), 13_usize, Move(_13), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(5_usize, 2_usize, Move(_2), 11_usize, Move(_11), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: i8,mut _3: u64,mut _4: u16) -> *const u64 {
mir! {
type RET = *const u64;
let _5: [u8; 2];
let _6: f32;
let _7: isize;
let _8: isize;
let _9: Adt61;
let _10: (i32, i32, u64, u128);
let _11: char;
let _12: u128;
let _13: i32;
let _14: Adt49;
let _15: *const u128;
let _16: ((u64, bool),);
let _17: bool;
let _18: Adt57;
let _19: [u8; 2];
let _20: Adt54;
let _21: u16;
let _22: u32;
let _23: [i32; 6];
let _24: usize;
let _25: f32;
let _26: *mut (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool);
let _27: (i8,);
let _28: f64;
let _29: bool;
let _30: isize;
let _31: (i32, i32, u64, u128);
let _32: (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool);
let _33: *const u8;
let _34: u64;
let _35: char;
let _36: i128;
let _37: u16;
let _38: Adt62;
let _39: f32;
let _40: isize;
let _41: char;
let _42: isize;
let _43: Adt54;
let _44: char;
let _45: Adt47;
let _46: (i8,);
let _47: Adt49;
let _48: (u64, bool);
let _49: (i8,);
let _50: [i32; 6];
let _51: ();
let _52: ();
{
_2 = (-59_i8) >> _1;
RET = core::ptr::addr_of!(_3);
RET = core::ptr::addr_of!((*RET));
_2 = (-86_i8) >> (*RET);
(*RET) = 47_u8 as u64;
RET = core::ptr::addr_of!(_3);
(*RET) = 1317635410_u32 as u64;
_4 = 149614378663431215648815917061397017906_i128 as u16;
_5 = [115_u8,155_u8];
(*RET) = 16114818302538350116_u64;
_6 = 1592568460_i32 as f32;
(*RET) = 14628868359416600588_u64 * 6013112750675654273_u64;
match _1 {
0 => bb1,
9223372036854775807 => bb3,
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
_1 = (-9223372036854775808_isize);
_4 = 2493_u16;
_3 = 5466677628914090746_u64 << _2;
_3 = (-14928862460015958761768839541602691074_i128) as u64;
_3 = !14579493696308316622_u64;
RET = core::ptr::addr_of!(_3);
_4 = !52756_u16;
Goto(bb4)
}
bb4 = {
_2 = (-67_i8);
_7 = !_1;
RET = core::ptr::addr_of!((*RET));
_9.fld1.fld5.fld3 = 3_usize * 7_usize;
_10.3 = 4981699648862395862_i64 as u128;
_10.2 = !_3;
_10.0 = (-395947142_i32);
_10.0 = 2079248679_i32;
_10.3 = !122133927065739754304220237096540827255_u128;
_13 = -_10.0;
_9.fld1.fld2.0 = core::ptr::addr_of!(_9.fld1.fld0.1);
_12 = (-24990126277732773599324470496030698975_i128) as u128;
_9.fld1.fld1.0 = [_10.0,_13,_13,_13,_13,_13];
_13 = _10.0;
match _10.0 {
2079248679 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
_9.fld1.fld1.0 = [_10.0,_10.0,_10.0,_10.0,_13,_13];
_2 = _9.fld1.fld5.fld3 as i8;
Goto(bb7)
}
bb7 = {
_4 = 52240_u16 % 49568_u16;
_9.fld1.fld0.2 = core::ptr::addr_of!(_9.fld1.fld0.3);
_9.fld1.fld0.1 = (*RET) as f64;
_9.fld1.fld5.fld0 = _9.fld1.fld2.0;
_9.fld1.fld5.fld1 = '\u{811f}';
_14.fld2.0 = core::ptr::addr_of_mut!(_9.fld1.fld1.0);
_14.fld7 = (_9.fld1.fld2.0,);
_14.fld7 = (_9.fld1.fld2.0,);
_14.fld7 = (_9.fld1.fld5.fld0,);
_9.fld1.fld5.fld1 = '\u{fb546}';
_9.fld1.fld0.2 = _14.fld7.0;
_14.fld1 = _9.fld1.fld5.fld1;
_9.fld1.fld3 = [_9.fld1.fld5.fld3,_9.fld1.fld5.fld3,_9.fld1.fld5.fld3];
_10.0 = (-160846506416128927556314117860390760246_i128) as i32;
_14.fld2.1.3 = _10.3 | _10.3;
_9.fld1.fld0.0 = _6;
_14.fld3 = _9.fld1.fld0.1 as usize;
_16.0 = Checked(_3 * _3);
_9.fld1.fld0.2 = core::ptr::addr_of!(_9.fld1.fld0.1);
_18.fld0.2 = core::ptr::addr_of!(_9.fld1.fld0.3);
_10 = (_13, _13, (*RET), _14.fld2.1.3);
_18.fld0.1 = _9.fld1.fld0.1 + _9.fld1.fld0.1;
_11 = _14.fld1;
_10.0 = !_10.1;
Call(_9.fld1.fld4 = fn7(_9.fld1.fld3, _13, _10, _14.fld2.0, _16.0.1, _6, _9.fld1.fld2.0, _9.fld1.fld5.fld3, _9.fld1.fld5.fld0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13 = _6 as i32;
_18.fld1.0 = [_13,_10.0,_10.1,_10.1,_10.0,_13];
_19 = _5;
_20.fld2.fld2 = _4;
_14.fld2.3 = !_16.0.1;
(*RET) = _16.0.0 % 873418365143156676_u64;
_9.fld1.fld5.fld0 = _9.fld1.fld0.2;
_20.fld1 = core::ptr::addr_of_mut!(_14.fld2);
_18.fld2.0 = core::ptr::addr_of!(_9.fld1.fld0.3);
_20.fld2.fld4 = core::ptr::addr_of!(_21);
_20.fld2.fld0 = core::ptr::addr_of_mut!(_16);
_17 = _14.fld2.3;
_14.fld2.1.0 = _13 + _10.0;
_14.fld2.1.2 = (*RET) ^ (*RET);
_18.fld5.fld1 = _9.fld1.fld5.fld1;
_20.fld3.3 = !_14.fld2.1.3;
_14.fld2.1 = (_10.0, _13, (*RET), _10.3);
Call((*RET) = core::intrinsics::bswap(_10.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18.fld1.0 = _9.fld1.fld1.0;
(*RET) = !_10.2;
_26 = core::ptr::addr_of_mut!(_14.fld2);
_14.fld7 = (_9.fld1.fld5.fld0,);
_18.fld5.fld3 = _14.fld3 << _13;
_14.fld2.1.1 = (*RET) as i32;
_18.fld0.2 = _14.fld7.0;
_9.fld0 = core::ptr::addr_of!(_3);
_14.fld2.1 = _10;
Goto(bb10)
}
bb10 = {
_31 = ((*_26).1.1, (*_26).1.0, _14.fld2.1.2, _10.3);
(*_26).1.0 = (*_26).1.1;
_22 = !1927619659_u32;
_9.fld1.fld1 = (_18.fld1.0,);
(*_26).3 = (*RET) < (*RET);
match _31.0 {
0 => bb7,
2079248679 => bb11,
_ => bb2
}
}
bb11 = {
_39 = _3 as f32;
Call((*RET) = core::intrinsics::bswap(_16.0.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_24 = !_9.fld1.fld5.fld3;
_20.fld1 = core::ptr::addr_of_mut!((*_26));
_9.fld1.fld0.3 = -_9.fld1.fld0.1;
(*_26).1.3 = _31.3 ^ _10.3;
_20.fld3.1 = (*_26).3 as i32;
_28 = -_9.fld1.fld0.3;
_14.fld3 = !_24;
(*_26).1.2 = (*RET) << _14.fld3;
_14.fld2.1.1 = -_13;
_37 = _20.fld2.fld2 + _4;
_14.fld0.0 = _7 as i16;
_27 = (_2,);
_46 = (_27.0,);
_43.fld2.fld4 = core::ptr::addr_of!(_43.fld2.fld2);
_13 = _20.fld2.fld2 as i32;
_40 = -_1;
_8 = _7 + _7;
_18.fld3 = [_24,_9.fld1.fld5.fld3,_24];
_31 = ((*_26).1.1, (*_26).1.1, _14.fld2.1.2, (*_26).1.3);
_47.fld0 = (_14.fld0.0,);
_20.fld3.2 = _46.0 as u64;
_20.fld3.1 = !(*_26).1.0;
match _1 {
0 => bb11,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
340282366920938463454151235394913435648 => bb20,
_ => bb19
}
}
bb13 = {
_39 = _3 as f32;
Call((*RET) = core::intrinsics::bswap(_16.0.0), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_31 = ((*_26).1.1, (*_26).1.0, _14.fld2.1.2, _10.3);
(*_26).1.0 = (*_26).1.1;
_22 = !1927619659_u32;
_9.fld1.fld1 = (_18.fld1.0,);
(*_26).3 = (*RET) < (*RET);
match _31.0 {
0 => bb7,
2079248679 => bb11,
_ => bb2
}
}
bb15 = {
Return()
}
bb16 = {
_13 = _6 as i32;
_18.fld1.0 = [_13,_10.0,_10.1,_10.1,_10.0,_13];
_19 = _5;
_20.fld2.fld2 = _4;
_14.fld2.3 = !_16.0.1;
(*RET) = _16.0.0 % 873418365143156676_u64;
_9.fld1.fld5.fld0 = _9.fld1.fld0.2;
_20.fld1 = core::ptr::addr_of_mut!(_14.fld2);
_18.fld2.0 = core::ptr::addr_of!(_9.fld1.fld0.3);
_20.fld2.fld4 = core::ptr::addr_of!(_21);
_20.fld2.fld0 = core::ptr::addr_of_mut!(_16);
_17 = _14.fld2.3;
_14.fld2.1.0 = _13 + _10.0;
_14.fld2.1.2 = (*RET) ^ (*RET);
_18.fld5.fld1 = _9.fld1.fld5.fld1;
_20.fld3.3 = !_14.fld2.1.3;
_14.fld2.1 = (_10.0, _13, (*RET), _10.3);
Call((*RET) = core::intrinsics::bswap(_10.2), ReturnTo(bb9), UnwindUnreachable())
}
bb17 = {
Return()
}
bb18 = {
_9.fld1.fld1.0 = [_10.0,_10.0,_10.0,_10.0,_13,_13];
_2 = _9.fld1.fld5.fld3 as i8;
Goto(bb7)
}
bb19 = {
Return()
}
bb20 = {
_47.fld0.0 = _14.fld0.0;
_36 = (-125454405244319975468314576678558201589_i128);
_43.fld2.fld2 = _22 as u16;
Goto(bb21)
}
bb21 = {
Call(_51 = dump_var(6_usize, 46_usize, Move(_46), 1_usize, Move(_1), 8_usize, Move(_8), 17_usize, Move(_17)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_51 = dump_var(6_usize, 19_usize, Move(_19), 2_usize, Move(_2), 36_usize, Move(_36), 5_usize, Move(_5)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_51 = dump_var(6_usize, 7_usize, Move(_7), 27_usize, Move(_27), 37_usize, Move(_37), 52_usize, _52), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [usize; 3],mut _2: i32,mut _3: (i32, i32, u64, u128),mut _4: *mut [i32; 6],mut _5: bool,mut _6: f32,mut _7: *const f64,mut _8: usize,mut _9: *const f64) -> [i8; 2] {
mir! {
type RET = [i8; 2];
let _10: (u32, *mut [char; 3], [usize; 3], (*const f64,), *mut [i32; 6]);
let _11: char;
let _12: Adt53;
let _13: [i32; 6];
let _14: (i8,);
let _15: ([i32; 6],);
let _16: f32;
let _17: bool;
let _18: Adt56;
let _19: Adt53;
let _20: *mut ((u64, bool),);
let _21: char;
let _22: (i8,);
let _23: ((u64, bool),);
let _24: [i32; 6];
let _25: Adt62;
let _26: char;
let _27: [i32; 6];
let _28: f64;
let _29: (f32, f64, *const f64, f64);
let _30: [u8; 3];
let _31: Adt62;
let _32: (u64, bool);
let _33: f32;
let _34: (i16,);
let _35: [u8; 3];
let _36: u64;
let _37: (i8,);
let _38: i8;
let _39: Adt47;
let _40: (u64, bool);
let _41: bool;
let _42: ();
let _43: ();
{
(*_9) = _8 as f64;
(*_4) = [_2,_2,_3.1,_2,_2,_3.0];
_3 = (_2, _2, 6957323113112270305_u64, 207790951595328928938242984575331469415_u128);
_3.0 = (-8373186554661183937_i64) as i32;
_5 = !false;
_10.4 = core::ptr::addr_of_mut!((*_4));
_10.3.0 = _7;
_3.2 = 10951_i16 as u64;
_3.0 = -_3.1;
_10.2 = [_8,_8,_8];
(*_7) = _3.3 as f64;
Call(_7 = fn8(_1, _10.3, _3.0, _10.4, (*_4), (*_9), (*_4), _3.2, _10.3, _3.3, _10.4, _3.0, _4, _10.4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.0 = _3.1 - _2;
_9 = core::ptr::addr_of!((*_9));
_12.fld0 = 109_u8 & 24_u8;
_12.fld2 = 673075920_u32 ^ 2471837985_u32;
_10.3 = (_9,);
_12.fld3.1 = _3.1 | _3.1;
_12.fld3.1 = '\u{fb19d}' as i32;
_10.3.0 = core::ptr::addr_of!((*_9));
_10.1 = core::ptr::addr_of_mut!(_12.fld3.2);
_5 = false;
_10.3 = (_9,);
_8 = 4_usize;
_10.0 = _12.fld2;
_6 = (-2330658957812608929_i64) as f32;
_8 = (-103_isize) as usize;
_12.fld1 = (23978_i16,);
_13 = [_3.0,_12.fld3.1,_3.0,_3.1,_3.0,_3.1];
_12.fld3.1 = !_3.1;
_3.1 = _3.0;
_13 = (*_4);
_10.4 = core::ptr::addr_of_mut!((*_4));
_10.0 = 117470906137249476630903141596641580069_i128 as u32;
_14 = (36_i8,);
RET = [_14.0,_14.0];
Call(_10.3 = fn18(_10.1, _2, _14, _3.2, _8, _12.fld1.0, _5, (*_4), _2, _12.fld2, (*_9), _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15.0 = [_3.0,_3.0,_3.0,_12.fld3.1,_12.fld3.1,_3.0];
_5 = true;
(*_4) = _15.0;
_5 = !false;
_15 = ((*_4),);
_10.3 = (_7,);
_11 = '\u{10b6eb}';
_16 = _8 as f32;
_12.fld1.0 = -4451_i16;
_12.fld3.1 = _3.0;
_3.2 = 9282336957584973197_u64 - 4894078971707029486_u64;
_12.fld4 = _8 as i16;
_10.0 = _12.fld2;
RET = [_14.0,_14.0];
_5 = _8 != _8;
_12.fld2 = _10.0 << _12.fld0;
_7 = core::ptr::addr_of!((*_9));
_10.0 = _12.fld2;
_10.2 = _1;
match _3.3 {
0 => bb1,
1 => bb3,
2 => bb4,
207790951595328928938242984575331469415 => bb6,
_ => bb5
}
}
bb3 = {
_3.0 = _3.1 - _2;
_9 = core::ptr::addr_of!((*_9));
_12.fld0 = 109_u8 & 24_u8;
_12.fld2 = 673075920_u32 ^ 2471837985_u32;
_10.3 = (_9,);
_12.fld3.1 = _3.1 | _3.1;
_12.fld3.1 = '\u{fb19d}' as i32;
_10.3.0 = core::ptr::addr_of!((*_9));
_10.1 = core::ptr::addr_of_mut!(_12.fld3.2);
_5 = false;
_10.3 = (_9,);
_8 = 4_usize;
_10.0 = _12.fld2;
_6 = (-2330658957812608929_i64) as f32;
_8 = (-103_isize) as usize;
_12.fld1 = (23978_i16,);
_13 = [_3.0,_12.fld3.1,_3.0,_3.1,_3.0,_3.1];
_12.fld3.1 = !_3.1;
_3.1 = _3.0;
_13 = (*_4);
_10.4 = core::ptr::addr_of_mut!((*_4));
_10.0 = 117470906137249476630903141596641580069_i128 as u32;
_14 = (36_i8,);
RET = [_14.0,_14.0];
Call(_10.3 = fn18(_10.1, _2, _14, _3.2, _8, _12.fld1.0, _5, (*_4), _2, _12.fld2, (*_9), _5), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
(*_9) = _3.0 as f64;
_12.fld1 = (_12.fld4,);
_13 = [_3.0,_2,_2,_12.fld3.1,_3.0,_12.fld3.1];
RET = [_14.0,_14.0];
_10.3 = (_9,);
_11 = '\u{a2244}';
_5 = !false;
_16 = _6;
_3 = (_12.fld3.1, _12.fld3.1, 6771303520122201299_u64, 319729659592664684468269144113178040048_u128);
(*_9) = _12.fld1.0 as f64;
_18.fld0 = _3.3;
_12.fld4 = _14.0 as i16;
_18.fld1.4 = _4;
_17 = _5;
_11 = '\u{2e616}';
(*_7) = 4113_u16 as f64;
_19.fld3.1 = _12.fld3.1 << _3.0;
RET = [_14.0,_14.0];
_18.fld2.fld4 = _12.fld4 ^ _12.fld4;
_18.fld2.fld3.1 = _19.fld3.1;
match _3.2 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb4,
6771303520122201299 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_18.fld2.fld3.2 = [_11,_11,_11];
_12.fld3 = (_16, _3.1, _18.fld2.fld3.2);
_10.3 = (_9,);
_18.fld1.2 = _10.2;
_18.fld1.3 = (_7,);
_19.fld1.0 = -_18.fld2.fld4;
_3 = (_12.fld3.1, _18.fld2.fld3.1, 6137283433189961463_u64, _18.fld0);
_18.fld2.fld2 = !_10.0;
_19.fld3.2 = [_11,_11,_11];
_3.2 = 13611477596551018242_u64;
_18.fld1.0 = _18.fld2.fld2 - _18.fld2.fld2;
_18.fld3 = core::ptr::addr_of_mut!(_23);
(*_7) = 5431812803427101864_i64 as f64;
_23.0.1 = _18.fld1.0 == _18.fld1.0;
_12.fld3 = (_6, _19.fld3.1, _18.fld2.fld3.2);
_18.fld2.fld3 = _12.fld3;
(*_4) = [_12.fld3.1,_19.fld3.1,_19.fld3.1,_18.fld2.fld3.1,_18.fld2.fld3.1,_18.fld2.fld3.1];
_19.fld1 = _12.fld1;
match _3.3 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
319729659592664684468269144113178040048 => bb9,
_ => bb5
}
}
bb9 = {
_1 = [_8,_8,_8];
_29.2 = _7;
_18.fld1.0 = _18.fld2.fld2 / 1492504976_u32;
_22 = (_14.0,);
_18.fld1.1 = _10.1;
_6 = _12.fld3.0;
_10 = (_18.fld1.0, _18.fld1.1, _18.fld1.2, _18.fld1.3, _4);
RET = [_14.0,_22.0];
_10.3 = (_18.fld1.3.0,);
_18.fld3 = core::ptr::addr_of_mut!(_23);
_10 = (_12.fld2, _18.fld1.1, _18.fld1.2, _18.fld1.3, _4);
_29.1 = (*_7) * (*_7);
_25 = Adt62 { fld0: _14.0 };
_24 = [_3.1,_3.1,_3.0,_18.fld2.fld3.1,_3.0,_19.fld3.1];
_3.3 = !_18.fld0;
_22.0 = _3.3 as i8;
_19.fld3 = (_16, _18.fld2.fld3.1, _12.fld3.2);
_19.fld3.2 = [_11,_11,_11];
(*_4) = [_12.fld3.1,_2,_19.fld3.1,_19.fld3.1,_19.fld3.1,_18.fld2.fld3.1];
match _18.fld0 {
0 => bb6,
1 => bb2,
2 => bb7,
3 => bb4,
319729659592664684468269144113178040048 => bb10,
_ => bb5
}
}
bb10 = {
_19.fld3.1 = _11 as i32;
match _25.fld0 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
36 => bb18,
_ => bb17
}
}
bb11 = {
_1 = [_8,_8,_8];
_29.2 = _7;
_18.fld1.0 = _18.fld2.fld2 / 1492504976_u32;
_22 = (_14.0,);
_18.fld1.1 = _10.1;
_6 = _12.fld3.0;
_10 = (_18.fld1.0, _18.fld1.1, _18.fld1.2, _18.fld1.3, _4);
RET = [_14.0,_22.0];
_10.3 = (_18.fld1.3.0,);
_18.fld3 = core::ptr::addr_of_mut!(_23);
_10 = (_12.fld2, _18.fld1.1, _18.fld1.2, _18.fld1.3, _4);
_29.1 = (*_7) * (*_7);
_25 = Adt62 { fld0: _14.0 };
_24 = [_3.1,_3.1,_3.0,_18.fld2.fld3.1,_3.0,_19.fld3.1];
_3.3 = !_18.fld0;
_22.0 = _3.3 as i8;
_19.fld3 = (_16, _18.fld2.fld3.1, _12.fld3.2);
_19.fld3.2 = [_11,_11,_11];
(*_4) = [_12.fld3.1,_2,_19.fld3.1,_19.fld3.1,_19.fld3.1,_18.fld2.fld3.1];
match _18.fld0 {
0 => bb6,
1 => bb2,
2 => bb7,
3 => bb4,
319729659592664684468269144113178040048 => bb10,
_ => bb5
}
}
bb12 = {
_15.0 = [_3.0,_3.0,_3.0,_12.fld3.1,_12.fld3.1,_3.0];
_5 = true;
(*_4) = _15.0;
_5 = !false;
_15 = ((*_4),);
_10.3 = (_7,);
_11 = '\u{10b6eb}';
_16 = _8 as f32;
_12.fld1.0 = -4451_i16;
_12.fld3.1 = _3.0;
_3.2 = 9282336957584973197_u64 - 4894078971707029486_u64;
_12.fld4 = _8 as i16;
_10.0 = _12.fld2;
RET = [_14.0,_14.0];
_5 = _8 != _8;
_12.fld2 = _10.0 << _12.fld0;
_7 = core::ptr::addr_of!((*_9));
_10.0 = _12.fld2;
_10.2 = _1;
match _3.3 {
0 => bb1,
1 => bb3,
2 => bb4,
207790951595328928938242984575331469415 => bb6,
_ => bb5
}
}
bb13 = {
Return()
}
bb14 = {
_3.0 = _3.1 - _2;
_9 = core::ptr::addr_of!((*_9));
_12.fld0 = 109_u8 & 24_u8;
_12.fld2 = 673075920_u32 ^ 2471837985_u32;
_10.3 = (_9,);
_12.fld3.1 = _3.1 | _3.1;
_12.fld3.1 = '\u{fb19d}' as i32;
_10.3.0 = core::ptr::addr_of!((*_9));
_10.1 = core::ptr::addr_of_mut!(_12.fld3.2);
_5 = false;
_10.3 = (_9,);
_8 = 4_usize;
_10.0 = _12.fld2;
_6 = (-2330658957812608929_i64) as f32;
_8 = (-103_isize) as usize;
_12.fld1 = (23978_i16,);
_13 = [_3.0,_12.fld3.1,_3.0,_3.1,_3.0,_3.1];
_12.fld3.1 = !_3.1;
_3.1 = _3.0;
_13 = (*_4);
_10.4 = core::ptr::addr_of_mut!((*_4));
_10.0 = 117470906137249476630903141596641580069_i128 as u32;
_14 = (36_i8,);
RET = [_14.0,_14.0];
Call(_10.3 = fn18(_10.1, _2, _14, _3.2, _8, _12.fld1.0, _5, (*_4), _2, _12.fld2, (*_9), _5), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_3.0 = _3.1 - _2;
_9 = core::ptr::addr_of!((*_9));
_12.fld0 = 109_u8 & 24_u8;
_12.fld2 = 673075920_u32 ^ 2471837985_u32;
_10.3 = (_9,);
_12.fld3.1 = _3.1 | _3.1;
_12.fld3.1 = '\u{fb19d}' as i32;
_10.3.0 = core::ptr::addr_of!((*_9));
_10.1 = core::ptr::addr_of_mut!(_12.fld3.2);
_5 = false;
_10.3 = (_9,);
_8 = 4_usize;
_10.0 = _12.fld2;
_6 = (-2330658957812608929_i64) as f32;
_8 = (-103_isize) as usize;
_12.fld1 = (23978_i16,);
_13 = [_3.0,_12.fld3.1,_3.0,_3.1,_3.0,_3.1];
_12.fld3.1 = !_3.1;
_3.1 = _3.0;
_13 = (*_4);
_10.4 = core::ptr::addr_of_mut!((*_4));
_10.0 = 117470906137249476630903141596641580069_i128 as u32;
_14 = (36_i8,);
RET = [_14.0,_14.0];
Call(_10.3 = fn18(_10.1, _2, _14, _3.2, _8, _12.fld1.0, _5, (*_4), _2, _12.fld2, (*_9), _5), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_19 = Adt53 { fld0: _12.fld0,fld1: _12.fld1,fld2: _10.0,fld3: _18.fld2.fld3,fld4: _18.fld2.fld4 };
_32.0 = _18.fld1.0 as u64;
_29.1 = (*_9) / 1_f64;
_19.fld0 = _12.fld0 & _12.fld0;
_26 = _11;
_29 = (_6, (*_9), _7, (*_7));
_12.fld3 = _19.fld3;
_15.0 = [_3.1,_19.fld3.1,_3.1,_3.0,_18.fld2.fld3.1,_19.fld3.1];
_3 = (_2, _12.fld3.1, _32.0, _18.fld0);
_3.2 = !_32.0;
_24 = (*_4);
_18.fld1.2 = [_8,_8,_8];
_32.1 = _23.0.1 < _23.0.1;
_12.fld3.2 = _18.fld2.fld3.2;
_31.fld0 = -_22.0;
_18.fld1.3 = _10.3;
_12.fld3.0 = _8 as f32;
_34 = (_19.fld1.0,);
_19.fld4 = _3.2 as i16;
_18.fld1 = (_18.fld2.fld2, _10.1, _10.2, _10.3, _10.4);
_30 = [_19.fld0,_19.fld0,_19.fld0];
_12.fld0 = _19.fld0;
_37.0 = _22.0 & _31.fld0;
_34.0 = _18.fld2.fld4;
_18.fld2.fld3.1 = _16 as i32;
_16 = -_6;
Goto(bb19)
}
bb19 = {
Call(_42 = dump_var(7_usize, 26_usize, Move(_26), 30_usize, Move(_30), 17_usize, Move(_17), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_42 = dump_var(7_usize, 1_usize, Move(_1), 24_usize, Move(_24), 3_usize, Move(_3), 37_usize, Move(_37)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [usize; 3],mut _2: (*const f64,),mut _3: i32,mut _4: *mut [i32; 6],mut _5: [i32; 6],mut _6: f64,mut _7: [i32; 6],mut _8: u64,mut _9: (*const f64,),mut _10: u128,mut _11: *mut [i32; 6],mut _12: i32,mut _13: *mut [i32; 6],mut _14: *mut [i32; 6],mut _15: *mut [i32; 6]) -> *const f64 {
mir! {
type RET = *const f64;
let _16: [usize; 3];
let _17: (i32, i32, u64, u128);
let _18: (i8,);
let _19: i8;
let _20: char;
let _21: [usize; 3];
let _22: ((u64, bool),);
let _23: u32;
let _24: Adt62;
let _25: Adt54;
let _26: (i8,);
let _27: [u8; 3];
let _28: *const u16;
let _29: isize;
let _30: ();
let _31: ();
{
_9 = (_2.0,);
_3 = _12;
_13 = _14;
_5 = [_3,_12,_12,_12,_3,_12];
_7 = [_12,_12,_3,_12,_3,_12];
_8 = 1109153514710784421_u64;
_6 = (-3024445472686463677_i64) as f64;
(*_15) = _7;
_16 = [5_usize,18102847281710374608_usize,7_usize];
_7 = [_3,_3,_12,_12,_12,_12];
(*_15) = _7;
_12 = !_3;
(*_13) = _7;
_17.3 = _10 - _10;
_16 = _1;
(*_14) = [_12,_3,_12,_12,_3,_12];
_4 = core::ptr::addr_of_mut!((*_11));
_18 = ((-62_i8),);
Goto(bb1)
}
bb1 = {
RET = core::ptr::addr_of!(_6);
(*_15) = [_12,_3,_3,_3,_12,_3];
_4 = core::ptr::addr_of_mut!((*_4));
_16 = [7_usize,7_usize,1810732196568989468_usize];
(*RET) = 11683084511383617590_usize as f64;
_15 = core::ptr::addr_of_mut!((*_15));
_17 = (_12, _12, _8, _10);
_3 = _17.1;
(*_11) = _5;
RET = core::ptr::addr_of!((*RET));
(*RET) = (-9223372036854775808_isize) as f64;
Call(_18 = fn9(_16, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_22.0.1 = false;
_21 = [6363266324166464963_usize,6_usize,6479592931658055102_usize];
Call(_12 = core::intrinsics::transmute(_17.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17.0 = _10 as i32;
(*_13) = [_17.0,_17.0,_17.0,_12,_17.0,_3];
_19 = -_18.0;
_17.2 = _8;
_17.3 = _10 + _10;
_13 = core::ptr::addr_of_mut!(_7);
_1 = _16;
(*RET) = (-9223372036854775808_isize) as f64;
(*_14) = (*_13);
_13 = _4;
_10 = _17.3 | _17.3;
(*_11) = [_17.0,_3,_17.0,_17.0,_17.0,_3];
_22.0 = Checked(_17.2 * _17.2);
_13 = core::ptr::addr_of_mut!((*_4));
_18 = (_19,);
_8 = (-7713_i16) as u64;
_13 = _15;
(*RET) = 224_u8 as f64;
_2.0 = core::ptr::addr_of!((*RET));
_18.0 = _17.2 as i8;
_22.0.0 = 79912026018910638436941950785273096400_i128 as u64;
_25.fld0 = [7923687355610947836_usize,12181401388299841220_usize,2_usize];
_18.0 = _19 << _10;
match _17.2 {
0 => bb2,
1 => bb4,
2 => bb5,
1109153514710784421 => bb7,
_ => bb6
}
}
bb4 = {
_22.0.1 = false;
_21 = [6363266324166464963_usize,6_usize,6479592931658055102_usize];
Call(_12 = core::intrinsics::transmute(_17.1), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = core::ptr::addr_of!(_6);
(*_15) = [_12,_3,_3,_3,_12,_3];
_4 = core::ptr::addr_of_mut!((*_4));
_16 = [7_usize,7_usize,1810732196568989468_usize];
(*RET) = 11683084511383617590_usize as f64;
_15 = core::ptr::addr_of_mut!((*_15));
_17 = (_12, _12, _8, _10);
_3 = _17.1;
(*_11) = _5;
RET = core::ptr::addr_of!((*RET));
(*RET) = (-9223372036854775808_isize) as f64;
Call(_18 = fn9(_16, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_12 = 9223372036854775807_isize as i32;
match _17.2 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
1109153514710784421 => bb14,
_ => bb13
}
}
bb8 = {
Return()
}
bb9 = {
RET = core::ptr::addr_of!(_6);
(*_15) = [_12,_3,_3,_3,_12,_3];
_4 = core::ptr::addr_of_mut!((*_4));
_16 = [7_usize,7_usize,1810732196568989468_usize];
(*RET) = 11683084511383617590_usize as f64;
_15 = core::ptr::addr_of_mut!((*_15));
_17 = (_12, _12, _8, _10);
_3 = _17.1;
(*_11) = _5;
RET = core::ptr::addr_of!((*RET));
(*RET) = (-9223372036854775808_isize) as f64;
Call(_18 = fn9(_16, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_22.0.1 = false;
_21 = [6363266324166464963_usize,6_usize,6479592931658055102_usize];
Call(_12 = core::intrinsics::transmute(_17.1), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_17.0 = _10 as i32;
(*_13) = [_17.0,_17.0,_17.0,_12,_17.0,_3];
_19 = -_18.0;
_17.2 = _8;
_17.3 = _10 + _10;
_13 = core::ptr::addr_of_mut!(_7);
_1 = _16;
(*RET) = (-9223372036854775808_isize) as f64;
(*_14) = (*_13);
_13 = _4;
_10 = _17.3 | _17.3;
(*_11) = [_17.0,_3,_17.0,_17.0,_17.0,_3];
_22.0 = Checked(_17.2 * _17.2);
_13 = core::ptr::addr_of_mut!((*_4));
_18 = (_19,);
_8 = (-7713_i16) as u64;
_13 = _15;
(*RET) = 224_u8 as f64;
_2.0 = core::ptr::addr_of!((*RET));
_18.0 = _17.2 as i8;
_22.0.0 = 79912026018910638436941950785273096400_i128 as u64;
_25.fld0 = [7923687355610947836_usize,12181401388299841220_usize,2_usize];
_18.0 = _19 << _10;
match _17.2 {
0 => bb2,
1 => bb4,
2 => bb5,
1109153514710784421 => bb7,
_ => bb6
}
}
bb12 = {
_22.0.1 = false;
_21 = [6363266324166464963_usize,6_usize,6479592931658055102_usize];
Call(_12 = core::intrinsics::transmute(_17.1), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
RET = core::ptr::addr_of!(_6);
(*_15) = [_12,_3,_3,_3,_12,_3];
_4 = core::ptr::addr_of_mut!((*_4));
_16 = [7_usize,7_usize,1810732196568989468_usize];
(*RET) = 11683084511383617590_usize as f64;
_15 = core::ptr::addr_of_mut!((*_15));
_17 = (_12, _12, _8, _10);
_3 = _17.1;
(*_11) = _5;
RET = core::ptr::addr_of!((*RET));
(*RET) = (-9223372036854775808_isize) as f64;
Call(_18 = fn9(_16, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_24 = Adt62 { fld0: _18.0 };
_25.fld3.3 = (-13932450517370248539352615980251024456_i128) as u128;
(*_4) = [_17.0,_17.0,_17.0,_3,_3,_17.1];
_17.3 = _10;
_20 = '\u{e4fc7}';
_25.fld2.fld4 = core::ptr::addr_of!(_25.fld2.fld2);
(*_14) = _5;
_25.fld0 = [13672514942526484312_usize,12727488495899473838_usize,5_usize];
_25.fld3.2 = _22.0.0;
_25.fld3 = (_17.0, _17.0, _17.2, _10);
_24.fld0 = !_18.0;
_19 = _24.fld0;
_26.0 = _19;
_25.fld2.fld2 = !51989_u16;
_25.fld2.fld3 = _22.0.0;
(*_13) = [_17.0,_25.fld3.1,_25.fld3.1,_25.fld3.0,_25.fld3.0,_17.0];
_13 = core::ptr::addr_of_mut!((*_15));
_26 = _18;
_2 = (_9.0,);
_17.1 = _25.fld3.0 * _25.fld3.1;
_25.fld0 = _21;
RET = core::ptr::addr_of!(_6);
_17.1 = _25.fld3.1;
(*_13) = _7;
(*_11) = [_25.fld3.0,_17.1,_17.1,_25.fld3.0,_25.fld3.0,_25.fld3.0];
_16 = _25.fld0;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(8_usize, 8_usize, Move(_8), 16_usize, Move(_16), 10_usize, Move(_10), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(8_usize, 17_usize, Move(_17), 26_usize, Move(_26), 5_usize, Move(_5), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [usize; 3],mut _2: (*const f64,)) -> (i8,) {
mir! {
type RET = (i8,);
let _3: Adt52;
let _4: [u8; 3];
let _5: f64;
let _6: [usize; 3];
let _7: [i16; 7];
let _8: (i16,);
let _9: (u64, bool);
let _10: (u64, bool);
let _11: char;
let _12: ([i32; 6],);
let _13: ((u64, bool),);
let _14: isize;
let _15: [i16; 7];
let _16: char;
let _17: [i8; 2];
let _18: bool;
let _19: f32;
let _20: u8;
let _21: f64;
let _22: (*const u128, (f32, i32, [char; 3]), usize, f64);
let _23: char;
let _24: Adt52;
let _25: [u8; 2];
let _26: (*const f64,);
let _27: Adt56;
let _28: char;
let _29: Adt53;
let _30: u128;
let _31: [usize; 3];
let _32: Adt59;
let _33: (u32, *mut [char; 3], [usize; 3], (*const f64,), *mut [i32; 6]);
let _34: isize;
let _35: [u8; 2];
let _36: Adt56;
let _37: [u8; 2];
let _38: bool;
let _39: [usize; 3];
let _40: i32;
let _41: i16;
let _42: u16;
let _43: isize;
let _44: [u8; 3];
let _45: Adt60;
let _46: ();
let _47: ();
{
RET = ((-72_i8),);
Goto(bb1)
}
bb1 = {
_1 = [1_usize,4_usize,2_usize];
RET.0 = 3123495140_u32 as i8;
RET.0 = 34_i8;
RET.0 = (-495143070_i32) as i8;
RET.0 = -(-125_i8);
RET.0 = 65_i8 | (-13_i8);
_3.fld3 = !9759283480935535641_u64;
RET.0 = (-7_i8) * (-77_i8);
_3.fld3 = 386299954_u32 as u64;
RET.0 = !23_i8;
_1 = [15996538391270212161_usize,10774946625716912069_usize,5_usize];
_3.fld2 = (-19_isize) as u16;
_3.fld1 = (-7952430022343096308_i64) ^ 5241551121294484755_i64;
_3.fld2 = !45046_u16;
RET = ((-3_i8),);
RET = ((-72_i8),);
RET = (15_i8,);
_5 = 1317258710_i32 as f64;
_3.fld1 = !7153588935659224371_i64;
_2.0 = core::ptr::addr_of!(_5);
_1 = [14851147443404713593_usize,18110623346711565294_usize,2_usize];
_3.fld4 = core::ptr::addr_of!(_3.fld2);
_3.fld1 = -(-5431454258068119790_i64);
_2.0 = core::ptr::addr_of!(_5);
Goto(bb2)
}
bb2 = {
RET.0 = 221_u8 as i8;
_6 = _1;
Call(RET = fn10(_3.fld4, _5, _3.fld1, _2, _2.0, _3.fld4, _3.fld2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = [2_u8,39_u8,144_u8];
_3.fld3 = 9225423636763716892_u64;
_3.fld4 = core::ptr::addr_of!(_3.fld2);
RET = (81_i8,);
_3.fld2 = 31907_u16 % 25612_u16;
_1 = [17158950683492063058_usize,4383538021405332593_usize,9189601126513298492_usize];
RET = (90_i8,);
RET = ((-79_i8),);
_6 = _1;
_5 = 1005415060_u32 as f64;
Goto(bb4)
}
bb4 = {
_4 = [239_u8,173_u8,10_u8];
_9 = (_3.fld3, true);
_4 = [173_u8,192_u8,186_u8];
_8.0 = 26357_i16 - (-20747_i16);
_3.fld4 = core::ptr::addr_of!(_3.fld2);
RET.0 = 77676965_i32 as i8;
RET.0 = !121_i8;
_9.1 = true;
_10 = _9;
Call(_8 = fn11(_9.1, _10, _9.0, _3.fld4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3.fld4 = core::ptr::addr_of!(_3.fld2);
_2.0 = core::ptr::addr_of!(_5);
_3.fld3 = !_10.0;
_12.0 = [(-582298016_i32),(-54005190_i32),(-1400294454_i32),(-1977979041_i32),(-367586676_i32),1842717571_i32];
_6 = _1;
_9 = (_10.0, _10.1);
RET.0 = 79_i8;
_8.0 = 9847_i16;
_10 = (_9.0, _9.1);
_7 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
RET = (103_i8,);
_3.fld1 = !350809001202179006_i64;
_3.fld0 = core::ptr::addr_of_mut!(_13);
_12.0 = [(-1974674490_i32),(-2133851649_i32),1455268501_i32,(-1474017666_i32),(-470474222_i32),1178407793_i32];
_13.0.0 = !_10.0;
_3.fld4 = core::ptr::addr_of!(_3.fld2);
_3.fld0 = core::ptr::addr_of_mut!(_13);
Goto(bb6)
}
bb6 = {
_9.0 = !_3.fld3;
_10.1 = _9.1 <= _9.1;
_3.fld2 = 98_i8 as u16;
_5 = 170813848540421719719286036025590899_i128 as f64;
_13 = (_9,);
_6 = _1;
_11 = '\u{dd826}';
RET = (15_i8,);
_11 = '\u{10bc0}';
_7 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_11 = '\u{bc152}';
_3.fld3 = !_10.0;
_9.1 = _10.1;
_4 = [56_u8,26_u8,79_u8];
_10.1 = _13.0.1 ^ _9.1;
_13.0.0 = (-134410078150611286175559651018094851547_i128) as u64;
_8 = (5271_i16,);
_3.fld0 = core::ptr::addr_of_mut!(_13);
RET = ((-48_i8),);
_13.0 = (_9.0, _9.1);
_3.fld0 = core::ptr::addr_of_mut!(_13);
Goto(bb7)
}
bb7 = {
_5 = 91414034457450364724047860090657471725_u128 as f64;
_15 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_2.0 = core::ptr::addr_of!(_5);
Goto(bb8)
}
bb8 = {
_3.fld1 = 1504690390_i32 as i64;
_13 = (_9,);
_9.1 = !_10.1;
_4 = [28_u8,207_u8,35_u8];
RET.0 = (-21_i8) & (-71_i8);
_3.fld4 = core::ptr::addr_of!(_3.fld2);
_12.0 = [(-1823688345_i32),(-1439609767_i32),1108815466_i32,107767770_i32,(-810610878_i32),(-670123342_i32)];
_13.0.1 = !_9.1;
_9 = Checked(_13.0.0 * _3.fld3);
_4 = [32_u8,57_u8,238_u8];
_10.1 = !_13.0.1;
_19 = 0_usize as f32;
_16 = _11;
_22.3 = _5 / f64::NAN;
_7 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_18 = _10.1;
_22.1.0 = -_19;
_20 = !183_u8;
RET.0 = 68_i8 << _20;
Goto(bb9)
}
bb9 = {
_3.fld4 = core::ptr::addr_of!(_24.fld2);
_3.fld3 = _9.0;
_21 = 145459410440533845755460713660837557716_i128 as f64;
RET = ((-81_i8),);
_22.2 = 11675536004298579120_usize;
_24.fld4 = core::ptr::addr_of!(_3.fld2);
_14 = 54_isize;
RET = ((-45_i8),);
_22.1.1 = (-1807160276_i32);
_22.1.0 = _20 as f32;
_13.0.1 = _8.0 < _8.0;
_24.fld2 = 128781371654728086407344445761735323484_i128 as u16;
_9.1 = _10.1 | _10.1;
_10.0 = (-137355569128839745339272484129866664551_i128) as u64;
_14 = -9223372036854775807_isize;
_16 = _11;
_13 = (_10,);
_3.fld0 = core::ptr::addr_of_mut!(_13);
_27.fld2.fld1 = (_8.0,);
_20 = 179_u8 | 18_u8;
_27.fld1.0 = _22.2 as u32;
_10.1 = _9.1;
Goto(bb10)
}
bb10 = {
_24.fld3 = !_9.0;
_29.fld2 = _24.fld2 as u32;
RET = ((-121_i8),);
_1 = [_22.2,_22.2,_22.2];
_24.fld1 = !_3.fld1;
_27.fld2.fld3.0 = _22.1.0 + _22.1.0;
_1 = [_22.2,_22.2,_22.2];
_14 = !34_isize;
_9.0 = 85139143188517018215192902148287004092_u128 as u64;
_20 = !198_u8;
_3.fld3 = _24.fld3 >> _24.fld2;
_24.fld3 = _22.2 as u64;
_15 = [_27.fld2.fld1.0,_27.fld2.fld1.0,_27.fld2.fld1.0,_27.fld2.fld1.0,_27.fld2.fld1.0,_27.fld2.fld1.0,_27.fld2.fld1.0];
Call(_27.fld2.fld2 = core::intrinsics::transmute(_16), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11 = _16;
_29.fld1 = (_27.fld2.fld1.0,);
_13 = (_9,);
_21 = _22.3 - _22.3;
_12.0 = [_22.1.1,_22.1.1,_22.1.1,_22.1.1,_22.1.1,_22.1.1];
_27.fld0 = _22.2 as u128;
_22.0 = core::ptr::addr_of!(_27.fld0);
_26 = (_2.0,);
RET.0 = (-64_i8);
_16 = _11;
_27.fld2.fld3.1 = _22.1.1 - _22.1.1;
_13 = (_9,);
_27.fld2.fld3.2 = [_11,_16,_11];
_14 = _27.fld2.fld3.0 as isize;
match _29.fld1.0 {
5271 => bb12,
_ => bb8
}
}
bb12 = {
_29.fld3.0 = _3.fld1 as f32;
_9 = _13.0;
_27.fld1.4 = core::ptr::addr_of_mut!(_12.0);
RET = (92_i8,);
_28 = _16;
_13.0 = (_3.fld3, _18);
_26 = (_2.0,);
_32.fld1.0 = core::ptr::addr_of!(_22.3);
_26 = (_32.fld1.0,);
_30 = _27.fld0;
_31 = _1;
_13 = (_10,);
_9.0 = _3.fld3;
_36.fld2.fld3.1 = _24.fld1 as i32;
_27.fld2.fld4 = _10.0 as i16;
_32.fld6 = !_24.fld2;
_36.fld2.fld0 = !_20;
_29 = Adt53 { fld0: _36.fld2.fld0,fld1: _27.fld2.fld1,fld2: _27.fld1.0,fld3: _27.fld2.fld3,fld4: _27.fld2.fld4 };
_36.fld2.fld4 = _29.fld1.0 << _27.fld0;
_13.0 = Checked(_3.fld3 * _3.fld3);
_27.fld1.1 = core::ptr::addr_of_mut!(_27.fld2.fld3.2);
_27.fld2.fld1.0 = _13.0.0 as i16;
_33.3.0 = core::ptr::addr_of!(_5);
_13 = (_10,);
_36.fld1 = (_27.fld1.0, _27.fld1.1, _6, _26, _27.fld1.4);
_20 = _3.fld1 as u8;
Call(_32.fld2 = core::intrinsics::bswap(_14), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_36.fld2.fld4 = _27.fld0 as i16;
_9 = _13.0;
_17 = [53_i8,(-90_i8)];
_36.fld1.2 = [_22.2,_22.2,_22.2];
_7 = _15;
_27.fld2.fld3.2 = [_28,_16,_11];
_13.0.1 = !_18;
_33.4 = core::ptr::addr_of_mut!(_12.0);
_36.fld2.fld1 = _27.fld2.fld1;
_10 = (_3.fld3, _18);
_36.fld2.fld1.0 = -_8.0;
_32.fld1.0 = core::ptr::addr_of!(_21);
_36.fld2.fld3.0 = -_27.fld2.fld3.0;
Goto(bb14)
}
bb14 = {
_27.fld2.fld4 = !_27.fld2.fld1.0;
_13.0.0 = _27.fld2.fld3.1 as u64;
_27.fld1.3 = _2;
_27.fld2.fld3.0 = _29.fld3.0 - _19;
_30 = !_27.fld0;
RET = ((-17_i8),);
_27.fld2.fld1 = (_27.fld2.fld4,);
_27.fld2.fld2 = _29.fld2 ^ _27.fld1.0;
_33.2 = _6;
_45 = Adt60 { fld0: _3.fld0,fld1: 115242319555362382720795222638109969837_i128 };
_27.fld1.0 = !_27.fld2.fld2;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(9_usize, 18_usize, Move(_18), 9_usize, Move(_9), 7_usize, Move(_7), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(9_usize, 20_usize, Move(_20), 8_usize, Move(_8), 30_usize, Move(_30), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(9_usize, 31_usize, Move(_31), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: *const u16,mut _2: f64,mut _3: i64,mut _4: (*const f64,),mut _5: *const f64,mut _6: *const u16,mut _7: u16) -> (i8,) {
mir! {
type RET = (i8,);
let _8: ((u64, bool),);
let _9: [u8; 2];
let _10: ((u64, bool),);
let _11: isize;
let _12: u16;
let _13: i16;
let _14: [char; 3];
let _15: f64;
let _16: [i16; 7];
let _17: (f32, i32, [char; 3]);
let _18: Adt53;
let _19: usize;
let _20: (i16,);
let _21: bool;
let _22: Adt63;
let _23: (i8,);
let _24: isize;
let _25: Adt53;
let _26: usize;
let _27: f64;
let _28: *mut i128;
let _29: Adt54;
let _30: char;
let _31: f32;
let _32: ();
let _33: ();
{
(*_1) = _7;
(*_5) = _2;
_5 = core::ptr::addr_of!((*_5));
RET = (113_i8,);
(*_1) = _7;
(*_1) = true as u16;
RET = ((-31_i8),);
(*_5) = _7 as f64;
_8.0.1 = !true;
_7 = !(*_1);
_8.0.0 = !12754419016135654106_u64;
RET = ((-124_i8),);
Goto(bb1)
}
bb1 = {
_6 = core::ptr::addr_of!(_7);
RET.0 = -(-107_i8);
(*_5) = -_2;
_8.0.0 = !10381240694064153851_u64;
(*_5) = -_2;
RET = ((-120_i8),);
(*_1) = 3003667534750391497_usize as u16;
_5 = core::ptr::addr_of!((*_5));
(*_1) = (*_6);
RET = ((-95_i8),);
_4.0 = core::ptr::addr_of!((*_5));
(*_1) = (*_6);
_1 = _6;
(*_5) = -_2;
RET = (30_i8,);
RET.0 = (*_1) as i8;
(*_6) = 45931_u16;
RET.0 = 101_i8 ^ (-124_i8);
_10.0 = _8.0;
(*_1) = 11916_u16;
_5 = _4.0;
_5 = core::ptr::addr_of!(_2);
(*_6) = 10798_u16;
(*_1) = !14514_u16;
(*_5) = _8.0.0 as f64;
Goto(bb2)
}
bb2 = {
_8.0.1 = (*_1) <= (*_6);
RET.0 = 29_i8;
_8.0.0 = 1_usize as u64;
_8 = (_10.0,);
_2 = 12454_i16 as f64;
_12 = !_7;
_9 = [188_u8,58_u8];
(*_5) = _8.0.0 as f64;
RET = (38_i8,);
_10.0 = _8.0;
_6 = core::ptr::addr_of!((*_1));
_4.0 = core::ptr::addr_of!((*_5));
(*_6) = !_12;
_3 = !(-4337541307671053401_i64);
_14 = ['\u{7338b}','\u{10483b}','\u{fe951}'];
_10 = (_8.0,);
_10 = (_8.0,);
(*_6) = (*_5) as u16;
_5 = core::ptr::addr_of!((*_5));
_1 = _6;
RET.0 = !25_i8;
RET = (21_i8,);
RET.0 = !(-121_i8);
_1 = core::ptr::addr_of!((*_1));
_8 = _10;
(*_1) = _12;
_13 = 19437_i16;
_15 = -_2;
match _13 {
0 => bb1,
1 => bb3,
2 => bb4,
19437 => bb6,
_ => bb5
}
}
bb3 = {
_6 = core::ptr::addr_of!(_7);
RET.0 = -(-107_i8);
(*_5) = -_2;
_8.0.0 = !10381240694064153851_u64;
(*_5) = -_2;
RET = ((-120_i8),);
(*_1) = 3003667534750391497_usize as u16;
_5 = core::ptr::addr_of!((*_5));
(*_1) = (*_6);
RET = ((-95_i8),);
_4.0 = core::ptr::addr_of!((*_5));
(*_1) = (*_6);
_1 = _6;
(*_5) = -_2;
RET = (30_i8,);
RET.0 = (*_1) as i8;
(*_6) = 45931_u16;
RET.0 = 101_i8 ^ (-124_i8);
_10.0 = _8.0;
(*_1) = 11916_u16;
_5 = _4.0;
_5 = core::ptr::addr_of!(_2);
(*_6) = 10798_u16;
(*_1) = !14514_u16;
(*_5) = _8.0.0 as f64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_11 = (*_1) as isize;
_8.0 = _10.0;
RET.0 = !(-79_i8);
(*_5) = _11 as f64;
_9 = [242_u8,230_u8];
_10.0 = Checked(_8.0.0 + _8.0.0);
(*_5) = _15;
(*_1) = _12;
_15 = -(*_5);
_10.0.1 = !_8.0.1;
RET.0 = !29_i8;
_13 = !(-15597_i16);
_5 = core::ptr::addr_of!(_2);
_2 = _15;
_11 = '\u{e8526}' as isize;
RET.0 = _2 as i8;
_4 = (_5,);
_8.0.0 = _10.0.0;
_10.0.0 = (*_6) as u64;
_2 = _15;
(*_1) = !_12;
_17.0 = _8.0.0 as f32;
_9 = [63_u8,137_u8];
_8 = _10;
_7 = _12 + _12;
_10.0.0 = !_8.0.0;
_17.0 = 37_u8 as f32;
Goto(bb7)
}
bb7 = {
_18.fld4 = -_13;
_16 = [_18.fld4,_13,_13,_13,_13,_13,_18.fld4];
_10 = (_8.0,);
_18.fld4 = -_13;
RET = ((-33_i8),);
_1 = _6;
(*_6) = _12 >> _3;
(*_5) = _15;
_4 = (_5,);
RET.0 = (-108_i8) + 112_i8;
_19 = !9772073931119773180_usize;
Goto(bb8)
}
bb8 = {
_13 = _18.fld4 * _18.fld4;
(*_5) = _15;
RET.0 = 60_i8;
_11 = -57_isize;
_17.1 = 518074472_i32 ^ 694520769_i32;
_2 = _15 + _15;
(*_1) = _12;
_22.fld1 = [_17.1,_17.1,_17.1,_17.1,_17.1,_17.1];
_10.0.1 = _8.0.1;
(*_1) = _18.fld4 as u16;
Goto(bb9)
}
bb9 = {
_14 = ['\u{bfbdc}','\u{2733c}','\u{d5a7d}'];
_18.fld2 = 231955049993038849757013842966293325280_u128 as u32;
_10.0.1 = _8.0.1 ^ _8.0.1;
_25.fld1 = (_18.fld4,);
_22.fld4.fld2 = (_4.0,);
_22.fld3 = [_19,_19,_19];
(*_6) = !_12;
_14 = ['\u{e17f4}','\u{a2d78}','\u{f96dd}'];
_22.fld4.fld1 = (_22.fld1,);
_29.fld0 = [_19,_19,_19];
_29.fld2.fld3 = _10.0.0 - _8.0.0;
_25.fld1.0 = _13 + _13;
_25.fld2 = _18.fld2 + _18.fld2;
_22.fld4.fld0.3 = (*_6) as f64;
_25.fld4 = _13;
_24 = -_11;
Goto(bb10)
}
bb10 = {
_22.fld4.fld7 = core::ptr::addr_of!(_25.fld0);
RET.0 = (-84_i8);
_7 = !_12;
_29.fld3.3 = 267828449589869597182521185167168118934_u128;
_22.fld4.fld5.fld3 = _11 as usize;
_12 = _29.fld3.3 as u16;
_29.fld2.fld3 = !_8.0.0;
_18.fld1.0 = _18.fld4 | _25.fld1.0;
_25.fld1.0 = !_13;
_25.fld3.0 = -_17.0;
_14 = ['\u{fbf5d}','\u{ea191}','\u{3a187}'];
_29.fld2.fld0 = core::ptr::addr_of_mut!(_10);
_29.fld2.fld3 = !_10.0.0;
_20 = (_25.fld4,);
_22.fld4.fld1.0 = _22.fld1;
_22.fld3 = _29.fld0;
_22.fld4.fld6 = _3;
_8.0.1 = !_10.0.1;
_18.fld3.1 = _29.fld3.3 as i32;
_25.fld3.2 = _14;
match _29.fld3.3 {
0 => bb7,
1 => bb2,
2 => bb8,
3 => bb5,
4 => bb11,
5 => bb12,
267828449589869597182521185167168118934 => bb14,
_ => bb13
}
}
bb11 = {
_6 = core::ptr::addr_of!(_7);
RET.0 = -(-107_i8);
(*_5) = -_2;
_8.0.0 = !10381240694064153851_u64;
(*_5) = -_2;
RET = ((-120_i8),);
(*_1) = 3003667534750391497_usize as u16;
_5 = core::ptr::addr_of!((*_5));
(*_1) = (*_6);
RET = ((-95_i8),);
_4.0 = core::ptr::addr_of!((*_5));
(*_1) = (*_6);
_1 = _6;
(*_5) = -_2;
RET = (30_i8,);
RET.0 = (*_1) as i8;
(*_6) = 45931_u16;
RET.0 = 101_i8 ^ (-124_i8);
_10.0 = _8.0;
(*_1) = 11916_u16;
_5 = _4.0;
_5 = core::ptr::addr_of!(_2);
(*_6) = 10798_u16;
(*_1) = !14514_u16;
(*_5) = _8.0.0 as f64;
Goto(bb2)
}
bb12 = {
_13 = _18.fld4 * _18.fld4;
(*_5) = _15;
RET.0 = 60_i8;
_11 = -57_isize;
_17.1 = 518074472_i32 ^ 694520769_i32;
_2 = _15 + _15;
(*_1) = _12;
_22.fld1 = [_17.1,_17.1,_17.1,_17.1,_17.1,_17.1];
_10.0.1 = _8.0.1;
(*_1) = _18.fld4 as u16;
Goto(bb9)
}
bb13 = {
_8.0.1 = (*_1) <= (*_6);
RET.0 = 29_i8;
_8.0.0 = 1_usize as u64;
_8 = (_10.0,);
_2 = 12454_i16 as f64;
_12 = !_7;
_9 = [188_u8,58_u8];
(*_5) = _8.0.0 as f64;
RET = (38_i8,);
_10.0 = _8.0;
_6 = core::ptr::addr_of!((*_1));
_4.0 = core::ptr::addr_of!((*_5));
(*_6) = !_12;
_3 = !(-4337541307671053401_i64);
_14 = ['\u{7338b}','\u{10483b}','\u{fe951}'];
_10 = (_8.0,);
_10 = (_8.0,);
(*_6) = (*_5) as u16;
_5 = core::ptr::addr_of!((*_5));
_1 = _6;
RET.0 = !25_i8;
RET = (21_i8,);
RET.0 = !(-121_i8);
_1 = core::ptr::addr_of!((*_1));
_8 = _10;
(*_1) = _12;
_13 = 19437_i16;
_15 = -_2;
match _13 {
0 => bb1,
1 => bb3,
2 => bb4,
19437 => bb6,
_ => bb5
}
}
bb14 = {
_22.fld4.fld7 = core::ptr::addr_of!(_25.fld0);
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(10_usize, 13_usize, Move(_13), 11_usize, Move(_11), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(10_usize, 7_usize, Move(_7), 10_usize, Move(_10), 33_usize, _33, 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: bool,mut _2: (u64, bool),mut _3: u64,mut _4: *const u16) -> (i16,) {
mir! {
type RET = (i16,);
let _5: (i32, i32, u64, u128);
let _6: u8;
let _7: *const u128;
let _8: *const u8;
let _9: *mut [char; 3];
let _10: [char; 3];
let _11: u8;
let _12: Adt58;
let _13: Adt57;
let _14: i128;
let _15: [i8; 2];
let _16: Adt62;
let _17: ();
let _18: ();
{
RET.0 = (-45_i8) as i16;
(*_4) = !63588_u16;
_5 = (1715890899_i32, 1722918191_i32, _3, 77542774596777873435526653272902953166_u128);
(*_4) = (-25_i8) as u16;
_6 = !217_u8;
RET.0 = 836_i16 + (-29285_i16);
_5.0 = _5.1 | _5.1;
_4 = core::ptr::addr_of!((*_4));
_3 = _5.2 >> _6;
(*_4) = !46343_u16;
match _5.3 {
77542774596777873435526653272902953166 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_5.1 = !_5.0;
_5.1 = !_5.0;
RET = ((-27078_i16),);
_6 = !202_u8;
_2.0 = _3 ^ _3;
_5.1 = _5.0 - _5.0;
match _5.3 {
0 => bb3,
1 => bb4,
2 => bb5,
77542774596777873435526653272902953166 => bb7,
_ => bb6
}
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
(*_4) = 5841_u16 + 3371_u16;
RET.0 = (-25426_i16) + (-17680_i16);
_6 = 88_u8;
_7 = core::ptr::addr_of!(_5.3);
_3 = _2.0 & _2.0;
(*_7) = 40338128119367488208528333677838175094_u128;
RET = (22839_i16,);
(*_4) = 35252_u16;
_2.1 = !_1;
_5.3 = (-9223372036854775808_isize) as u128;
(*_4) = 15358_u16 % 29647_u16;
_8 = core::ptr::addr_of!(_6);
_4 = core::ptr::addr_of!((*_4));
_5.2 = _2.0;
RET.0 = _5.1 as i16;
(*_4) = !27099_u16;
_2 = Checked(_5.2 - _3);
RET.0 = (-7805_i16) & (-19680_i16);
(*_7) = 31020955647586804534029406062495918492_u128;
_2.0 = _3 & _3;
RET.0 = 21444_i16;
_5.3 = 276622291536752806335319243607326583590_u128 >> _5.0;
_2.1 = _1;
_5 = ((-460979269_i32), 970009372_i32, _3, 226014754353584588757918670818174579071_u128);
(*_4) = 17068_u16 << _5.0;
RET = ((-17639_i16),);
Goto(bb8)
}
bb8 = {
_9 = core::ptr::addr_of_mut!(_10);
_12.fld1.2 = [(*_8),(*_8)];
(*_7) = 210132520042467162962958153761870277608_u128;
(*_4) = !25346_u16;
Call(_12.fld1.3 = fn12(_9, (*_8), _7, _3, _1, _5.3, _7, _7, _5.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET.0 = !22536_i16;
_12.fld1.1.2 = _5.2 % 16530469001971884426_u64;
_12.fld1.1.3 = !(*_7);
_9 = core::ptr::addr_of_mut!(_12.fld2.2);
_12.fld3.fld1 = '\u{28628}';
_13.fld5.fld0 = core::ptr::addr_of!(_13.fld0.1);
_12.fld5.fld4 = 1866002628_u32 | 957913000_u32;
_12.fld1.1.3 = !(*_7);
_12.fld1.1.0 = _5.0 >> _12.fld1.1.2;
match _5.1 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb6,
970009372 => bb10,
_ => bb5
}
}
bb10 = {
_2 = (_5.2, _1);
_12.fld1.1 = _5;
(*_4) = 28352_u16 / 33737_u16;
_1 = _2.1;
_3 = _2.0;
(*_7) = _12.fld1.1.3;
_2 = (_5.2, _1);
(*_9) = [_12.fld3.fld1,_12.fld3.fld1,_12.fld3.fld1];
_12.fld1.1.3 = 2_usize as u128;
_3 = _12.fld1.1.2;
_12.fld5.fld4 = 10584_i16 as u32;
_13.fld2.0 = core::ptr::addr_of!(_13.fld0.3);
_12.fld1.1.3 = (*_7);
_12.fld3.fld1 = '\u{3e533}';
_13.fld0.1 = 93_i8 as f64;
_5.0 = _12.fld1.1.0;
_12.fld3.fld1 = '\u{ced3}';
_12.fld1.3 = (*_4) > (*_4);
_13.fld3 = [3_usize,9624183573280444335_usize,8385074860621156261_usize];
_12.fld2.1 = -_12.fld1.1.1;
(*_4) = 15129_u16 + 34754_u16;
_13.fld0.3 = _13.fld0.1 - _13.fld0.1;
match _5.1 {
970009372 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_12.fld3.fld2 = core::ptr::addr_of_mut!(_14);
_13.fld5.fld1 = _12.fld3.fld1;
_12.fld5.fld1.0 = core::ptr::addr_of_mut!(_13.fld1.0);
_12.fld1.1.1 = _2.0 as i32;
(*_8) = 214_u8 | 130_u8;
_12.fld3.fld2 = core::ptr::addr_of_mut!(_14);
_13.fld5.fld2 = core::ptr::addr_of_mut!(_14);
_12.fld5.fld3 = [28364_i16,(-16821_i16),(-3706_i16),(-17304_i16),(-30006_i16),31744_i16,(-25823_i16)];
Call(_13.fld6 = fn13(_13.fld2.0, _12.fld1.1.1, _2.1, _13.fld2.0, _13.fld5.fld2, _13.fld5.fld1, _12.fld5.fld1.0, _5.3, _13.fld5.fld1, _12.fld1.1.0, _12.fld3.fld2, _4, _5, (*_7)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_14 = _13.fld0.1 as i128;
_12.fld5.fld4 = 3_usize as u32;
_11 = _13.fld5.fld1 as u8;
_12.fld1.0 = _12.fld5.fld1.0;
_2.1 = _12.fld1.3;
_12.fld5.fld1.1.3 = _5.3;
_12.fld3 = Adt51 { fld0: _13.fld2.0,fld1: _13.fld5.fld1,fld2: _13.fld5.fld2,fld3: 4065823768164048972_usize };
_9 = core::ptr::addr_of_mut!(_12.fld2.2);
_13.fld0.2 = core::ptr::addr_of!(_13.fld0.3);
_12.fld2.1 = _12.fld1.1.1;
_15 = [(-65_i8),67_i8];
_12.fld5.fld1 = (_12.fld1.0, _12.fld1.1, _12.fld1.2, _2.1);
_12.fld1.1.3 = (*_7);
_13.fld4 = [(-79_i8),103_i8];
_2.0 = _12.fld5.fld1.1.2;
match _12.fld3.fld3 {
0 => bb1,
4065823768164048972 => bb14,
_ => bb6
}
}
bb14 = {
_13.fld5.fld2 = core::ptr::addr_of_mut!(_14);
_12.fld3.fld1 = _13.fld5.fld1;
_13.fld4 = [(-59_i8),(-80_i8)];
RET = ((-18248_i16),);
_12.fld5.fld4 = 1968640251_u32 >> _5.0;
_10 = [_12.fld3.fld1,_13.fld5.fld1,_13.fld5.fld1];
RET.0 = _12.fld3.fld3 as i16;
_12.fld1.1.1 = _12.fld1.1.0;
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(11_usize, 3_usize, Move(_3), 2_usize, Move(_2), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: *mut [char; 3],mut _2: u8,mut _3: *const u128,mut _4: u64,mut _5: bool,mut _6: u128,mut _7: *const u128,mut _8: *const u128,mut _9: i32) -> bool {
mir! {
type RET = bool;
let _10: *mut (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool);
let _11: u8;
let _12: *mut i128;
let _13: f64;
let _14: Adt52;
let _15: isize;
let _16: Adt62;
let _17: Adt48;
let _18: u8;
let _19: f32;
let _20: isize;
let _21: usize;
let _22: Adt53;
let _23: u64;
let _24: f32;
let _25: u16;
let _26: ((u64, bool),);
let _27: (*const f64,);
let _28: i64;
let _29: *mut i128;
let _30: i8;
let _31: Adt55;
let _32: usize;
let _33: (u32, *mut [char; 3], [usize; 3], (*const f64,), *mut [i32; 6]);
let _34: ();
let _35: ();
{
(*_7) = _2 as u128;
(*_8) = _6;
_4 = 11624351219414849693_u64;
(*_7) = _6 ^ _6;
RET = _5 | _5;
(*_3) = !_6;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = ['\u{6f8fa}','\u{1bea3}','\u{e7f12}'];
_6 = (*_8) / 318984741345616488429240625017947183120_u128;
(*_8) = !_6;
_11 = _2;
_3 = core::ptr::addr_of!((*_7));
Goto(bb1)
}
bb1 = {
_1 = core::ptr::addr_of_mut!((*_1));
RET = !_5;
_9 = (-80633130847143190139124108789882513817_i128) as i32;
_3 = core::ptr::addr_of!((*_8));
RET = (*_3) != _6;
RET = _5;
(*_1) = ['\u{19ea}','\u{e7e6b}','\u{4921b}'];
_11 = _2;
(*_1) = ['\u{5fa93}','\u{a1b5a}','\u{568f9}'];
(*_1) = ['\u{fbd69}','\u{27991}','\u{3260}'];
_1 = core::ptr::addr_of_mut!((*_1));
RET = _5;
_13 = (-9223372036854775808_isize) as f64;
(*_1) = ['\u{7b865}','\u{ddf9b}','\u{d9fd0}'];
_11 = _2;
_11 = _2 - _2;
(*_7) = !_6;
Call(_6 = core::intrinsics::transmute((*_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14.fld1 = -(-6308937215935704980_i64);
_3 = core::ptr::addr_of!((*_3));
(*_1) = ['\u{cbecc}','\u{6f7e1}','\u{4b1d2}'];
(*_3) = (-62_i8) as u128;
_3 = _7;
_5 = true;
match _2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
88 => bb10,
_ => bb9
}
}
bb3 = {
_1 = core::ptr::addr_of_mut!((*_1));
RET = !_5;
_9 = (-80633130847143190139124108789882513817_i128) as i32;
_3 = core::ptr::addr_of!((*_8));
RET = (*_3) != _6;
RET = _5;
(*_1) = ['\u{19ea}','\u{e7e6b}','\u{4921b}'];
_11 = _2;
(*_1) = ['\u{5fa93}','\u{a1b5a}','\u{568f9}'];
(*_1) = ['\u{fbd69}','\u{27991}','\u{3260}'];
_1 = core::ptr::addr_of_mut!((*_1));
RET = _5;
_13 = (-9223372036854775808_isize) as f64;
(*_1) = ['\u{7b865}','\u{ddf9b}','\u{d9fd0}'];
_11 = _2;
_11 = _2 - _2;
(*_7) = !_6;
Call(_6 = core::intrinsics::transmute((*_7)), ReturnTo(bb2), UnwindUnreachable())
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
_14.fld2 = !45988_u16;
_16.fld0 = (-96_i8);
_14.fld3 = _4;
_14.fld3 = 369324841_u32 as u64;
(*_8) = _6 % 116999646903781666457875190516960729464_u128;
_5 = !false;
_17.fld1.3 = _5;
_17.fld1.3 = !_5;
_14.fld3 = _4 | _4;
_1 = core::ptr::addr_of_mut!((*_1));
match _2 {
0 => bb3,
88 => bb11,
_ => bb8
}
}
bb11 = {
_2 = _13 as u8;
_15 = 9223372036854775807_isize * (-9223372036854775808_isize);
_17.fld1.1.0 = _9 + _9;
_11 = !_2;
_18 = '\u{165ad}' as u8;
_17.fld1.1.1 = _17.fld1.1.0;
_9 = _17.fld1.1.1;
_17.fld1.1.1 = _17.fld1.1.0;
_14.fld5 = core::ptr::addr_of_mut!(_17.fld1);
RET = _17.fld1.3;
_4 = _14.fld3;
_20 = _15 ^ _15;
_17.fld1.1.2 = !_14.fld3;
_11 = _17.fld1.1.1 as u8;
_22.fld2 = 3336502576_u32 & 566193241_u32;
Call(_14.fld3 = core::intrinsics::transmute(_15), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_1 = core::ptr::addr_of_mut!((*_1));
_8 = _7;
_21 = 5_usize - 9508881308461712112_usize;
_14.fld2 = 33563_u16;
_17.fld4 = 162397055782256629975822687489634684342_i128 as u32;
_20 = _15 ^ _15;
_24 = (-15968_i16) as f32;
_14.fld4 = core::ptr::addr_of!(_25);
_22.fld0 = !_11;
_22.fld1 = (31103_i16,);
(*_7) = !_6;
_14.fld0 = core::ptr::addr_of_mut!(_26);
_21 = _14.fld1 as usize;
_26.0 = Checked(_17.fld1.1.2 * _4);
_21 = 2_usize;
(*_8) = _6;
(*_1) = ['\u{788f9}','\u{db4ee}','\u{d8e82}'];
_22.fld3.2 = (*_1);
_17.fld2 = _14.fld0;
_22.fld2 = _17.fld4 ^ _17.fld4;
_6 = (*_7);
_11 = !_18;
_17.fld1.1.3 = (*_7) * (*_3);
Goto(bb13)
}
bb13 = {
_22.fld3 = (_24, _17.fld1.1.0, (*_1));
_22.fld3.0 = _24;
_20 = (*_1)[_21] as isize;
(*_8) = !_6;
_14.fld3 = _4 | _26.0.0;
_27.0 = core::ptr::addr_of!(_13);
_22.fld1 = (31417_i16,);
_17.fld3 = [_22.fld1.0,_22.fld1.0,_22.fld1.0,_22.fld1.0,_22.fld1.0,_22.fld1.0,_22.fld1.0];
_3 = _8;
_22.fld0 = _18 >> _14.fld3;
match _22.fld1.0 {
0 => bb4,
31417 => bb15,
_ => bb14
}
}
bb14 = {
_2 = _13 as u8;
_15 = 9223372036854775807_isize * (-9223372036854775808_isize);
_17.fld1.1.0 = _9 + _9;
_11 = !_2;
_18 = '\u{165ad}' as u8;
_17.fld1.1.1 = _17.fld1.1.0;
_9 = _17.fld1.1.1;
_17.fld1.1.1 = _17.fld1.1.0;
_14.fld5 = core::ptr::addr_of_mut!(_17.fld1);
RET = _17.fld1.3;
_4 = _14.fld3;
_20 = _15 ^ _15;
_17.fld1.1.2 = !_14.fld3;
_11 = _17.fld1.1.1 as u8;
_22.fld2 = 3336502576_u32 & 566193241_u32;
Call(_14.fld3 = core::intrinsics::transmute(_15), ReturnTo(bb12), UnwindUnreachable())
}
bb15 = {
_21 = 9105543541062871319_usize % 13255124815827251846_usize;
(*_7) = _16.fld0 as u128;
(*_7) = !_6;
_13 = _14.fld2 as f64;
_27.0 = core::ptr::addr_of!(_13);
_17.fld2 = core::ptr::addr_of_mut!(_26);
_22.fld1 = ((-19454_i16),);
RET = _17.fld1.3;
_22.fld1.0 = !13481_i16;
_30 = _14.fld2 as i8;
_14.fld2 = _21 as u16;
(*_1) = ['\u{35612}','\u{f7d90}','\u{a6393}'];
_32 = !_21;
_26.0.1 = _17.fld1.3 & _5;
_6 = _13 as u128;
_31.fld4.2 = core::ptr::addr_of!(_31.fld4.3);
_13 = _4 as f64;
(*_3) = _17.fld1.1.3 & _17.fld1.1.3;
_31.fld5 = (_24, _22.fld3.1, _22.fld3.2);
_17.fld1.1.2 = _14.fld3;
_22.fld4 = _22.fld1.0;
_22.fld3.2 = (*_1);
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(12_usize, 2_usize, Move(_2), 15_usize, Move(_15), 6_usize, Move(_6), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(12_usize, 18_usize, Move(_18), 21_usize, Move(_21), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *const f64,mut _2: i32,mut _3: bool,mut _4: *const f64,mut _5: *mut i128,mut _6: char,mut _7: *mut [i32; 6],mut _8: u128,mut _9: char,mut _10: i32,mut _11: *mut i128,mut _12: *const u16,mut _13: (i32, i32, u64, u128),mut _14: u128) -> i64 {
mir! {
type RET = i64;
let _15: i128;
let _16: Adt47;
let _17: i16;
let _18: [i32; 6];
let _19: (i16,);
let _20: [u8; 2];
let _21: char;
let _22: Adt52;
let _23: [usize; 3];
let _24: [i8; 2];
let _25: *const u8;
let _26: Adt53;
let _27: [u8; 2];
let _28: u16;
let _29: [i8; 2];
let _30: ();
let _31: ();
{
_3 = _8 == _14;
(*_7) = [_10,_13.1,_2,_13.0,_10,_2];
_2 = _13.0 | _13.1;
(*_1) = (-74814229616440883534048178731503409537_i128) as f64;
_15 = !111630012036355298423592751871889796651_i128;
RET = 3758428020760329028_i64;
_13.1 = !_2;
(*_12) = _3 as u16;
_17 = 52_i8 as i16;
_1 = _4;
(*_5) = (*_1) as i128;
(*_5) = _15;
(*_12) = 8433_u16 >> _13.1;
(*_11) = _15 - _15;
_13.0 = -_2;
_6 = _9;
Goto(bb1)
}
bb1 = {
_16.fld0 = core::ptr::addr_of_mut!(_6);
(*_7) = [_2,_13.1,_13.1,_13.0,_2,_10];
_19 = (_17,);
Call(_13.3 = core::intrinsics::transmute((*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = core::ptr::addr_of!((*_1));
_19.0 = _17;
_20 = [149_u8,157_u8];
_20 = [28_u8,53_u8];
_8 = 13_i8 as u128;
_12 = core::ptr::addr_of!((*_12));
_14 = (*_1) as u128;
(*_4) = 13_u8 as f64;
_13.1 = !_13.0;
(*_1) = 13410079461284725789_usize as f64;
_3 = !true;
_9 = _6;
(*_4) = _13.1 as f64;
_10 = _13.0;
RET = !(-2357709898534307896_i64);
_18 = [_13.1,_13.0,_13.1,_2,_13.1,_2];
(*_4) = 9223372036854775807_isize as f64;
_14 = _13.3;
_19 = (_17,);
(*_5) = _3 as i128;
Goto(bb3)
}
bb3 = {
(*_7) = [_13.0,_13.1,_13.1,_2,_10,_13.1];
_4 = core::ptr::addr_of!((*_1));
_15 = _3 as i128;
_21 = _6;
_9 = _6;
RET = (-8635711606420942907_i64) | (-3774243740545518490_i64);
_4 = core::ptr::addr_of!((*_1));
(*_12) = 35790_u16;
_3 = !false;
_13.2 = !8180674033549668578_u64;
Call(_4 = fn14(_16.fld0, (*_5), (*_1), _13, _16.fld0, _17, _18, _6, _1, (*_12), _5, _21, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13.1 = !_13.0;
_22.fld4 = core::ptr::addr_of!((*_12));
_22.fld3 = _3 as u64;
(*_1) = 11_i8 as f64;
_13 = (_2, _2, _22.fld3, _14);
_22.fld4 = core::ptr::addr_of!(_22.fld2);
(*_5) = -_15;
_21 = _6;
Call(_18 = core::intrinsics::transmute((*_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_22.fld4 = core::ptr::addr_of!(_22.fld2);
_6 = _9;
_13.2 = _22.fld3;
RET = !1438371059020763439_i64;
_11 = _5;
(*_1) = 9223372036854775807_isize as f64;
_17 = 9223372036854775807_isize as i16;
(*_7) = [_10,_13.0,_2,_13.1,_13.1,_13.0];
_12 = core::ptr::addr_of!((*_12));
_9 = _6;
(*_12) = 47665_u16;
_2 = _13.0;
_22.fld3 = _13.2 / 3360797972999868711_u64;
_24 = [33_i8,(-121_i8)];
_13.3 = 21_u8 as u128;
(*_7) = [_10,_2,_10,_10,_10,_13.0];
RET = (-2808234379713653555_i64);
_3 = false & false;
(*_5) = -_15;
_2 = _13.0;
RET = (-61093752642718202_i64);
_13.2 = !_22.fld3;
(*_11) = _15 * _15;
_22.fld4 = _12;
(*_1) = 2434763512_u32 as f64;
_26.fld3.1 = _10;
_22.fld1 = -(-2347360430701690038_i64);
_26.fld2 = !1037366411_u32;
_26.fld4 = _17 - _17;
match (*_12) {
0 => bb1,
1 => bb3,
2 => bb6,
3 => bb7,
4 => bb8,
47665 => bb10,
_ => bb9
}
}
bb6 = {
_13.1 = !_13.0;
_22.fld4 = core::ptr::addr_of!((*_12));
_22.fld3 = _3 as u64;
(*_1) = 11_i8 as f64;
_13 = (_2, _2, _22.fld3, _14);
_22.fld4 = core::ptr::addr_of!(_22.fld2);
(*_5) = -_15;
_21 = _6;
Call(_18 = core::intrinsics::transmute((*_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
(*_7) = [_13.0,_13.1,_13.1,_2,_10,_13.1];
_4 = core::ptr::addr_of!((*_1));
_15 = _3 as i128;
_21 = _6;
_9 = _6;
RET = (-8635711606420942907_i64) | (-3774243740545518490_i64);
_4 = core::ptr::addr_of!((*_1));
(*_12) = 35790_u16;
_3 = !false;
_13.2 = !8180674033549668578_u64;
Call(_4 = fn14(_16.fld0, (*_5), (*_1), _13, _16.fld0, _17, _18, _6, _1, (*_12), _5, _21, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_4 = core::ptr::addr_of!((*_1));
_19.0 = _17;
_20 = [149_u8,157_u8];
_20 = [28_u8,53_u8];
_8 = 13_i8 as u128;
_12 = core::ptr::addr_of!((*_12));
_14 = (*_1) as u128;
(*_4) = 13_u8 as f64;
_13.1 = !_13.0;
(*_1) = 13410079461284725789_usize as f64;
_3 = !true;
_9 = _6;
(*_4) = _13.1 as f64;
_10 = _13.0;
RET = !(-2357709898534307896_i64);
_18 = [_13.1,_13.0,_13.1,_2,_13.1,_2];
(*_4) = 9223372036854775807_isize as f64;
_14 = _13.3;
_19 = (_17,);
(*_5) = _3 as i128;
Goto(bb3)
}
bb9 = {
_16.fld0 = core::ptr::addr_of_mut!(_6);
(*_7) = [_2,_13.1,_13.1,_13.0,_2,_10];
_19 = (_17,);
Call(_13.3 = core::intrinsics::transmute((*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_9 = _21;
_19.0 = !_26.fld4;
_26.fld3.1 = _13.1 * _13.1;
_3 = _2 != _26.fld3.1;
_19.0 = -_26.fld4;
_19.0 = _26.fld4;
_22.fld3 = !_13.2;
_12 = core::ptr::addr_of!(_22.fld2);
_27 = [3_u8,231_u8];
RET = _22.fld1 * _22.fld1;
_12 = _22.fld4;
_3 = true & true;
_15 = 47_u8 as i128;
_1 = _4;
_23 = [4_usize,4_usize,11133803453303256992_usize];
_16.fld0 = core::ptr::addr_of_mut!(_6);
_13 = (_26.fld3.1, _10, _22.fld3, _8);
_21 = _6;
_27 = [210_u8,13_u8];
_19 = (_26.fld4,);
_13.1 = _26.fld3.1 >> _13.0;
_22.fld2 = !(*_12);
_16.fld0 = core::ptr::addr_of_mut!(_9);
(*_11) = (*_12) as i128;
match (*_12) {
0 => bb9,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
47665 => bb17,
_ => bb16
}
}
bb11 = {
_16.fld0 = core::ptr::addr_of_mut!(_6);
(*_7) = [_2,_13.1,_13.1,_13.0,_2,_10];
_19 = (_17,);
Call(_13.3 = core::intrinsics::transmute((*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_4 = core::ptr::addr_of!((*_1));
_19.0 = _17;
_20 = [149_u8,157_u8];
_20 = [28_u8,53_u8];
_8 = 13_i8 as u128;
_12 = core::ptr::addr_of!((*_12));
_14 = (*_1) as u128;
(*_4) = 13_u8 as f64;
_13.1 = !_13.0;
(*_1) = 13410079461284725789_usize as f64;
_3 = !true;
_9 = _6;
(*_4) = _13.1 as f64;
_10 = _13.0;
RET = !(-2357709898534307896_i64);
_18 = [_13.1,_13.0,_13.1,_2,_13.1,_2];
(*_4) = 9223372036854775807_isize as f64;
_14 = _13.3;
_19 = (_17,);
(*_5) = _3 as i128;
Goto(bb3)
}
bb13 = {
(*_7) = [_13.0,_13.1,_13.1,_2,_10,_13.1];
_4 = core::ptr::addr_of!((*_1));
_15 = _3 as i128;
_21 = _6;
_9 = _6;
RET = (-8635711606420942907_i64) | (-3774243740545518490_i64);
_4 = core::ptr::addr_of!((*_1));
(*_12) = 35790_u16;
_3 = !false;
_13.2 = !8180674033549668578_u64;
Call(_4 = fn14(_16.fld0, (*_5), (*_1), _13, _16.fld0, _17, _18, _6, _1, (*_12), _5, _21, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_4 = core::ptr::addr_of!((*_1));
_19.0 = _17;
_20 = [149_u8,157_u8];
_20 = [28_u8,53_u8];
_8 = 13_i8 as u128;
_12 = core::ptr::addr_of!((*_12));
_14 = (*_1) as u128;
(*_4) = 13_u8 as f64;
_13.1 = !_13.0;
(*_1) = 13410079461284725789_usize as f64;
_3 = !true;
_9 = _6;
(*_4) = _13.1 as f64;
_10 = _13.0;
RET = !(-2357709898534307896_i64);
_18 = [_13.1,_13.0,_13.1,_2,_13.1,_2];
(*_4) = 9223372036854775807_isize as f64;
_14 = _13.3;
_19 = (_17,);
(*_5) = _3 as i128;
Goto(bb3)
}
bb15 = {
_16.fld0 = core::ptr::addr_of_mut!(_6);
(*_7) = [_2,_13.1,_13.1,_13.0,_2,_10];
_19 = (_17,);
Call(_13.3 = core::intrinsics::transmute((*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_13.1 = !_13.0;
_22.fld4 = core::ptr::addr_of!((*_12));
_22.fld3 = _3 as u64;
(*_1) = 11_i8 as f64;
_13 = (_2, _2, _22.fld3, _14);
_22.fld4 = core::ptr::addr_of!(_22.fld2);
(*_5) = -_15;
_21 = _6;
Call(_18 = core::intrinsics::transmute((*_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb17 = {
_29 = [(-121_i8),(-61_i8)];
_26.fld3.0 = (*_11) as f32;
_24 = [(-3_i8),(-8_i8)];
(*_12) = _22.fld2;
Goto(bb18)
}
bb18 = {
Call(_30 = dump_var(13_usize, 20_usize, Move(_20), 2_usize, Move(_2), 19_usize, Move(_19), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(13_usize, 24_usize, Move(_24), 21_usize, Move(_21), 14_usize, Move(_14), 27_usize, Move(_27)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_30 = dump_var(13_usize, 15_usize, Move(_15), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: *mut char,mut _2: i128,mut _3: f64,mut _4: (i32, i32, u64, u128),mut _5: *mut char,mut _6: i16,mut _7: [i32; 6],mut _8: char,mut _9: *const f64,mut _10: u16,mut _11: *mut i128,mut _12: char,mut _13: *mut i128) -> *const f64 {
mir! {
type RET = *const f64;
let _14: Adt61;
let _15: u8;
let _16: usize;
let _17: char;
let _18: Adt52;
let _19: f32;
let _20: Adt53;
let _21: *mut [i32; 6];
let _22: char;
let _23: [i32; 6];
let _24: usize;
let _25: [i32; 6];
let _26: (f32, i32, [char; 3]);
let _27: *const u8;
let _28: isize;
let _29: [i16; 7];
let _30: u16;
let _31: ([i32; 6],);
let _32: isize;
let _33: u64;
let _34: ([i32; 6],);
let _35: isize;
let _36: [i8; 2];
let _37: ((u64, bool),);
let _38: [u8; 2];
let _39: i16;
let _40: [u8; 2];
let _41: (i8,);
let _42: ();
let _43: ();
{
RET = core::ptr::addr_of!((*_9));
_4.3 = _10 as u128;
RET = _9;
(*RET) = _4.0 as f64;
(*_9) = _3 * _3;
_14.fld1.fld1 = (_7,);
_14.fld1.fld0.1 = (*_9);
_14.fld1.fld5.fld3 = 1_usize >> _4.0;
(*_9) = -_14.fld1.fld0.1;
_14.fld1.fld5 = Adt51 { fld0: _9,fld1: (*_5),fld2: _11,fld3: 13916502753665839145_usize };
_14.fld1.fld2 = (_14.fld1.fld5.fld0,);
_1 = core::ptr::addr_of_mut!((*_1));
_14.fld2 = core::ptr::addr_of!(_10);
_14.fld1.fld5.fld1 = (*_1);
(*_11) = _2;
_14.fld1.fld1 = (_7,);
_15 = true as u8;
_14.fld1.fld5 = Adt51 { fld0: _9,fld1: (*_1),fld2: _11,fld3: 0_usize };
_10 = _14.fld1.fld5.fld1 as u16;
Call(_11 = fn15(_3, _14.fld1.fld0.1, _1, _5, (*_11), _12, _8, _13, (*_5), (*_5), _14.fld1.fld2.0, (*_5)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = (*_1);
_14.fld1.fld5.fld1 = (*_5);
_17 = _8;
_7 = _14.fld1.fld1.0;
(*_9) = _14.fld1.fld0.1;
(*RET) = _14.fld1.fld0.1 / (-0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001467798494102466_f64);
_17 = (*_1);
_20.fld1 = (_6,);
_1 = core::ptr::addr_of_mut!((*_5));
_20.fld1 = (_6,);
_5 = _1;
_14.fld1.fld5.fld3 = _4.1 as usize;
_20.fld1.0 = -_6;
_19 = (-9223372036854775808_isize) as f32;
_14.fld0 = core::ptr::addr_of!(_18.fld3);
_14.fld1.fld3 = [_14.fld1.fld5.fld3,_14.fld1.fld5.fld3,_14.fld1.fld5.fld3];
_20.fld2 = 1619295584_u32 & 2751722503_u32;
_14.fld1.fld4 = [119_i8,125_i8];
(*_5) = _17;
_14.fld1.fld7 = core::ptr::addr_of!(_20.fld0);
_14.fld1.fld5.fld2 = core::ptr::addr_of_mut!((*_13));
_18.fld2 = _10 / 63121_u16;
_14.fld1.fld4 = [52_i8,70_i8];
_22 = _12;
_14.fld1.fld1.0 = _7;
_14.fld1.fld0.0 = _19;
(*_5) = _14.fld1.fld5.fld1;
Goto(bb2)
}
bb2 = {
_5 = core::ptr::addr_of_mut!((*_1));
_14.fld1.fld6 = (-4700701045148487689_i64);
_14.fld1.fld7 = core::ptr::addr_of!(_20.fld0);
_14.fld1.fld0.3 = _3;
_20.fld3.0 = _19;
_24 = !_14.fld1.fld5.fld3;
_20.fld0 = _15;
_20.fld3.2 = [(*_5),_12,(*_5)];
_3 = -(*RET);
_4.3 = 183874584826535605589608556608167430978_u128 ^ 319599352238162798034289975425838687837_u128;
_2 = (*_13) | (*_13);
(*_5) = _12;
_26 = (_20.fld3.0, _4.1, _20.fld3.2);
_26.0 = _14.fld1.fld0.0;
_18.fld3 = !_4.2;
match _14.fld1.fld6 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463458673906386619723767 => bb7,
_ => bb6
}
}
bb3 = {
_8 = (*_1);
_14.fld1.fld5.fld1 = (*_5);
_17 = _8;
_7 = _14.fld1.fld1.0;
(*_9) = _14.fld1.fld0.1;
(*RET) = _14.fld1.fld0.1 / (-0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001467798494102466_f64);
_17 = (*_1);
_20.fld1 = (_6,);
_1 = core::ptr::addr_of_mut!((*_5));
_20.fld1 = (_6,);
_5 = _1;
_14.fld1.fld5.fld3 = _4.1 as usize;
_20.fld1.0 = -_6;
_19 = (-9223372036854775808_isize) as f32;
_14.fld0 = core::ptr::addr_of!(_18.fld3);
_14.fld1.fld3 = [_14.fld1.fld5.fld3,_14.fld1.fld5.fld3,_14.fld1.fld5.fld3];
_20.fld2 = 1619295584_u32 & 2751722503_u32;
_14.fld1.fld4 = [119_i8,125_i8];
(*_5) = _17;
_14.fld1.fld7 = core::ptr::addr_of!(_20.fld0);
_14.fld1.fld5.fld2 = core::ptr::addr_of_mut!((*_13));
_18.fld2 = _10 / 63121_u16;
_14.fld1.fld4 = [52_i8,70_i8];
_22 = _12;
_14.fld1.fld1.0 = _7;
_14.fld1.fld0.0 = _19;
(*_5) = _14.fld1.fld5.fld1;
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
(*_1) = _8;
(*RET) = _14.fld1.fld0.1;
_9 = core::ptr::addr_of!((*_9));
_14.fld1.fld0 = (_19, _3, _14.fld1.fld2.0, (*_9));
_14.fld1.fld5.fld0 = core::ptr::addr_of!((*_9));
_9 = _14.fld1.fld0.2;
_4.1 = -_26.1;
_14.fld1.fld5.fld2 = core::ptr::addr_of_mut!((*_13));
_26.0 = _6 as f32;
_26.1 = _4.0;
_14.fld1.fld5 = Adt51 { fld0: _14.fld1.fld0.2,fld1: _17,fld2: _11,fld3: _24 };
_12 = _22;
(*RET) = _3;
_3 = (*RET) / (-0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002103285213400102_f64);
_4.3 = 314268053210914424371992749452546783367_u128 / 206815758026587825448802434617816181240_u128;
_5 = core::ptr::addr_of_mut!(_12);
_20.fld3 = _26;
_20.fld2 = 3241067536_u32 * 3875535902_u32;
_18.fld1 = _14.fld1.fld6;
_14.fld1.fld1.0 = [_26.1,_4.1,_4.0,_20.fld3.1,_26.1,_4.0];
_12 = (*_1);
Goto(bb8)
}
bb8 = {
(*_1) = _14.fld1.fld5.fld1;
_18.fld4 = core::ptr::addr_of!(_18.fld2);
_14.fld1.fld6 = _18.fld1;
(*RET) = -_3;
_12 = (*_1);
(*_5) = _17;
(*_1) = _14.fld1.fld5.fld1;
_23 = [_4.1,_4.1,_4.0,_26.1,_4.0,_4.0];
_13 = core::ptr::addr_of_mut!(_2);
_14.fld1.fld1.0 = [_4.1,_4.1,_4.1,_4.1,_20.fld3.1,_4.1];
_24 = _14.fld1.fld5.fld3;
_16 = (*_13) as usize;
_14.fld1.fld2 = (_14.fld1.fld5.fld0,);
_14.fld1.fld0.2 = _14.fld1.fld2.0;
(*_13) = !75711090389988174807676402069401038039_i128;
_20.fld3.0 = -_14.fld1.fld0.0;
_14.fld1.fld7 = core::ptr::addr_of!(_15);
_14.fld1.fld0 = (_26.0, (*RET), _14.fld1.fld2.0, (*_9));
_14.fld1.fld5 = Adt51 { fld0: _14.fld1.fld0.2,fld1: (*_1),fld2: _11,fld3: _24 };
_20.fld3 = (_26.0, _26.1, _26.2);
_20.fld0 = _15 / 56_u8;
(*_1) = _14.fld1.fld5.fld1;
RET = _9;
_27 = core::ptr::addr_of!(_20.fld0);
_25 = _7;
_14.fld1.fld0.3 = _14.fld1.fld5.fld3 as f64;
Call((*_5) = fn16((*RET), (*RET), _14.fld1.fld5.fld0, _26.0, _14.fld2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14.fld0 = core::ptr::addr_of!(_4.2);
_14.fld1.fld5.fld2 = core::ptr::addr_of_mut!(_2);
_26.2 = [_22,(*_5),_17];
(*RET) = (-56_i8) as f64;
_14.fld1.fld0.0 = _26.0;
_26.0 = -_19;
_32 = 9223372036854775807_isize;
(*_5) = _8;
_5 = core::ptr::addr_of_mut!((*_1));
_30 = _19 as u16;
_14.fld1.fld7 = _27;
_14.fld1.fld0.3 = (*_9);
match _14.fld1.fld6 {
0 => bb5,
1 => bb6,
2 => bb10,
3 => bb11,
340282366920938463458673906386619723767 => bb13,
_ => bb12
}
}
bb10 = {
_8 = (*_1);
_14.fld1.fld5.fld1 = (*_5);
_17 = _8;
_7 = _14.fld1.fld1.0;
(*_9) = _14.fld1.fld0.1;
(*RET) = _14.fld1.fld0.1 / (-0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001467798494102466_f64);
_17 = (*_1);
_20.fld1 = (_6,);
_1 = core::ptr::addr_of_mut!((*_5));
_20.fld1 = (_6,);
_5 = _1;
_14.fld1.fld5.fld3 = _4.1 as usize;
_20.fld1.0 = -_6;
_19 = (-9223372036854775808_isize) as f32;
_14.fld0 = core::ptr::addr_of!(_18.fld3);
_14.fld1.fld3 = [_14.fld1.fld5.fld3,_14.fld1.fld5.fld3,_14.fld1.fld5.fld3];
_20.fld2 = 1619295584_u32 & 2751722503_u32;
_14.fld1.fld4 = [119_i8,125_i8];
(*_5) = _17;
_14.fld1.fld7 = core::ptr::addr_of!(_20.fld0);
_14.fld1.fld5.fld2 = core::ptr::addr_of_mut!((*_13));
_18.fld2 = _10 / 63121_u16;
_14.fld1.fld4 = [52_i8,70_i8];
_22 = _12;
_14.fld1.fld1.0 = _7;
_14.fld1.fld0.0 = _19;
(*_5) = _14.fld1.fld5.fld1;
Goto(bb2)
}
bb11 = {
_5 = core::ptr::addr_of_mut!((*_1));
_14.fld1.fld6 = (-4700701045148487689_i64);
_14.fld1.fld7 = core::ptr::addr_of!(_20.fld0);
_14.fld1.fld0.3 = _3;
_20.fld3.0 = _19;
_24 = !_14.fld1.fld5.fld3;
_20.fld0 = _15;
_20.fld3.2 = [(*_5),_12,(*_5)];
_3 = -(*RET);
_4.3 = 183874584826535605589608556608167430978_u128 ^ 319599352238162798034289975425838687837_u128;
_2 = (*_13) | (*_13);
(*_5) = _12;
_26 = (_20.fld3.0, _4.1, _20.fld3.2);
_26.0 = _14.fld1.fld0.0;
_18.fld3 = !_4.2;
match _14.fld1.fld6 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463458673906386619723767 => bb7,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
_14.fld1.fld6 = !_18.fld1;
_34 = (_25,);
_20.fld1.0 = _6;
_13 = core::ptr::addr_of_mut!((*_13));
_14.fld1.fld5 = Adt51 { fld0: _14.fld1.fld2.0,fld1: (*_1),fld2: _13,fld3: _24 };
_18.fld2 = _14.fld1.fld6 as u16;
(*RET) = _14.fld1.fld0.3 / f64::NEG_INFINITY;
_11 = core::ptr::addr_of_mut!((*_13));
Goto(bb14)
}
bb14 = {
_21 = core::ptr::addr_of_mut!(_25);
_14.fld1.fld1 = _34;
RET = core::ptr::addr_of!(_3);
_18.fld1 = _14.fld1.fld6 >> _24;
(*RET) = -(*_9);
_37.0.0 = (-40_i8) as u64;
_18.fld4 = core::ptr::addr_of!(_10);
_33 = _37.0.0;
_15 = _20.fld0;
_36 = [(-111_i8),(-5_i8)];
_38 = [_15,_20.fld0];
_1 = core::ptr::addr_of_mut!(_8);
_14.fld1.fld0.0 = _20.fld3.1 as f32;
(*_11) = !(-66598698664999306228374570142652252860_i128);
_34 = ((*_21),);
_14.fld1.fld0.1 = (*RET);
(*RET) = _24 as f64;
(*_11) = 47639297408296623256913846630023792022_i128 | (-71179705847227750611690292550852267069_i128);
_18.fld1 = _14.fld1.fld6;
_14.fld1.fld1.0 = _25;
_6 = !_20.fld1.0;
_10 = !_18.fld2;
_8 = _14.fld1.fld5.fld1;
_3 = _37.0.0 as f64;
_20.fld0 = _15 * _15;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(14_usize, 33_usize, Move(_33), 2_usize, Move(_2), 8_usize, Move(_8), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(14_usize, 6_usize, Move(_6), 23_usize, Move(_23), 34_usize, Move(_34), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(14_usize, 12_usize, Move(_12), 15_usize, Move(_15), 43_usize, _43, 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: f64,mut _2: f64,mut _3: *mut char,mut _4: *mut char,mut _5: i128,mut _6: char,mut _7: char,mut _8: *mut i128,mut _9: char,mut _10: char,mut _11: *const f64,mut _12: char) -> *mut i128 {
mir! {
type RET = *mut i128;
let _13: bool;
let _14: (*const u128, (f32, i32, [char; 3]), usize, f64);
let _15: f64;
let _16: isize;
let _17: Adt53;
let _18: (u64, bool);
let _19: f32;
let _20: f64;
let _21: [char; 3];
let _22: isize;
let _23: char;
let _24: i16;
let _25: (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool);
let _26: *const u8;
let _27: u32;
let _28: bool;
let _29: Adt47;
let _30: f32;
let _31: Adt63;
let _32: i8;
let _33: u16;
let _34: i8;
let _35: ();
let _36: ();
{
_8 = core::ptr::addr_of_mut!(_5);
_8 = core::ptr::addr_of_mut!(_5);
(*_11) = 3284208336_u32 as f64;
(*_3) = _10;
(*_8) = (-24368494655056651978154820733515958927_i128);
RET = core::ptr::addr_of_mut!((*_8));
(*RET) = !(-129685798644231139993010762110065809996_i128);
(*_8) = 51595025217408571185201365156094527660_i128;
(*_11) = -_2;
_13 = (*_4) >= (*_4);
(*_11) = _2 / 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001077286572519454_f64;
_13 = (*_3) == (*_4);
(*_4) = _9;
(*_4) = _12;
(*_3) = _12;
_2 = (*RET) as f64;
_9 = _10;
Goto(bb1)
}
bb1 = {
_14.1.2 = [_7,_10,_12];
(*_11) = -_1;
(*_4) = _6;
_14.1.0 = (-258_i16) as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 22873524885997277252463806566329163458_i128;
_8 = core::ptr::addr_of_mut!((*_8));
_14.2 = !0_usize;
RET = core::ptr::addr_of_mut!((*_8));
(*RET) = 90126234945989279673949307977640121279_i128;
_8 = core::ptr::addr_of_mut!((*RET));
_9 = (*_4);
_14.1.0 = 9223372036854775807_isize as f32;
match (*_8) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
90126234945989279673949307977640121279 => bb8,
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
(*RET) = 128805657967645565075164611707529569856_i128;
(*RET) = 19126362978490071548542671683530458026_i128 >> _14.2;
_14.2 = 4_usize;
(*_3) = _12;
(*_8) = (-109035325596523115040886418369194728748_i128);
(*RET) = 77494473510860269_u64 as i128;
_17.fld1.0 = (-31447_i16);
_17.fld3.0 = _14.1.0;
_14.1.1 = 1678935932_i32 & (-119844481_i32);
_16 = _17.fld1.0 as isize;
match _17.fld1.0 {
0 => bb1,
1 => bb6,
2 => bb4,
3 => bb9,
340282366920938463463374607431768180009 => bb11,
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
_11 = core::ptr::addr_of!(_2);
_22 = -_16;
_19 = _17.fld3.0 / f32::NAN;
_17.fld4 = -_17.fld1.0;
_17.fld1 = (_17.fld4,);
(*RET) = (-86607186197925023523832800205562367213_i128);
_17.fld3.0 = -_14.1.0;
_7 = (*_4);
(*_4) = _9;
_21 = _14.1.2;
_17.fld3 = (_19, _14.1.1, _14.1.2);
_5 = 86598405637264755110615797663232967843_i128;
_18.1 = _13;
_17.fld1.0 = _17.fld4 ^ _17.fld4;
_21 = [_10,_12,(*_4)];
_19 = 4187153868_u32 as f32;
(*_11) = _1;
(*RET) = -(-400466349394235550225994472112499057_i128);
_16 = -_22;
_25.3 = _17.fld1.0 == _17.fld1.0;
_15 = (*_11) * _1;
_12 = (*_3);
match _14.2 {
0 => bb2,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb16,
_ => bb15
}
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
_25.1.3 = !84997330322003528165802949405903063841_u128;
_13 = _25.3;
_20 = 5911908709991228063_u64 as f64;
_24 = !_17.fld1.0;
_9 = _6;
_10 = _6;
_17.fld1.0 = _17.fld3.0 as i16;
_18.0 = 1360006537456379092_u64 % 17125473777504616123_u64;
_25.3 = _13;
_17.fld2 = _16 as u32;
(*_11) = _15;
_1 = (*RET) as f64;
_26 = core::ptr::addr_of!(_17.fld0);
(*_4) = _7;
_11 = core::ptr::addr_of!(_15);
RET = _8;
(*_26) = !135_u8;
_23 = _9;
_6 = _7;
_17.fld4 = _24 | _17.fld1.0;
_31.fld4.fld1.0 = [_14.1.1,_14.1.1,_17.fld3.1,_17.fld3.1,_14.1.1,_17.fld3.1];
_20 = _19 as f64;
_27 = _17.fld2;
_25.0 = core::ptr::addr_of_mut!(_31.fld1);
_4 = core::ptr::addr_of_mut!(_9);
_31.fld3 = [_14.2,_14.2,_14.2];
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(15_usize, 22_usize, Move(_22), 12_usize, Move(_12), 9_usize, Move(_9), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(15_usize, 21_usize, Move(_21), 10_usize, Move(_10), 7_usize, Move(_7), 36_usize, _36), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: f64,mut _2: f64,mut _3: *const f64,mut _4: f32,mut _5: *const u16) -> char {
mir! {
type RET = char;
let _6: u64;
let _7: Adt53;
let _8: ([i32; 6],);
let _9: (i8,);
let _10: u32;
let _11: *const u16;
let _12: [usize; 3];
let _13: (i32, i32, u64, u128);
let _14: Adt55;
let _15: [u8; 3];
let _16: u8;
let _17: Adt62;
let _18: isize;
let _19: f32;
let _20: isize;
let _21: *const u64;
let _22: Adt53;
let _23: *mut char;
let _24: [i16; 7];
let _25: (*const f64,);
let _26: (i8,);
let _27: i16;
let _28: f64;
let _29: [char; 3];
let _30: bool;
let _31: f64;
let _32: isize;
let _33: (*const f64,);
let _34: f64;
let _35: ();
let _36: ();
{
(*_3) = -_2;
(*_5) = 40316_u16;
RET = '\u{1aab0}';
(*_3) = -_1;
RET = '\u{565fe}';
(*_3) = -_1;
Call((*_3) = fn17((*_5), _5, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 119_u8 as f32;
RET = '\u{d0d60}';
_3 = core::ptr::addr_of!((*_3));
_2 = (-28904_i16) as f64;
(*_3) = -_1;
(*_5) = !51042_u16;
_1 = (*_3) * (*_3);
_6 = 15198391597680537160_u64;
_5 = core::ptr::addr_of!((*_5));
(*_3) = _1 / 1_f64;
RET = '\u{e8fba}';
RET = '\u{f5b26}';
(*_5) = 9223372036854775807_isize as u16;
(*_3) = _1;
_1 = (*_3) / f64::NAN;
(*_3) = _2;
_1 = -(*_3);
_2 = 302301359_u32 as f64;
_1 = 4_usize as f64;
_2 = _6 as f64;
_6 = (-608876655_i32) as u64;
_1 = (*_3) / f64::NEG_INFINITY;
_7.fld1 = ((-24300_i16),);
RET = '\u{7099d}';
_7.fld3.1 = -212975059_i32;
_4 = 100269390790329819616498052756657726176_u128 as f32;
(*_5) = !23417_u16;
RET = '\u{30364}';
Goto(bb2)
}
bb2 = {
_9.0 = -118_i8;
RET = '\u{8c47c}';
_7.fld3.0 = -_4;
(*_3) = -_1;
_5 = core::ptr::addr_of!((*_5));
_1 = (*_3) + (*_3);
_7.fld3.0 = _4;
RET = '\u{c45c4}';
_6 = 12079261582289886918_u64;
_7.fld0 = 204_u8 ^ 185_u8;
_5 = core::ptr::addr_of!((*_5));
RET = '\u{784ce}';
RET = '\u{5a28b}';
_7.fld3.2 = ['\u{104e6c}','\u{c5d74}','\u{17d0b}'];
_7.fld2 = _1 as u32;
_1 = (*_3) * _2;
_5 = core::ptr::addr_of!((*_5));
Goto(bb3)
}
bb3 = {
_7.fld2 = _7.fld0 as u32;
_7.fld3.1 = 590900491_i32;
_9.0 = 127_i8;
_7.fld3.0 = _4;
_1 = (*_5) as f64;
_4 = _7.fld3.0 * _7.fld3.0;
(*_3) = -_1;
_7.fld2 = 3883092491_u32;
_2 = _4 as f64;
_9.0 = 103_i8;
_6 = 10368594735342844310_u64;
RET = '\u{e606b}';
_2 = -_1;
(*_3) = 859835633954216067_i64 as f64;
_14.fld4.2 = core::ptr::addr_of!((*_3));
_9.0 = !(-2_i8);
_13 = (_7.fld3.1, _7.fld3.1, _6, 12509909960018081487890590905803600814_u128);
(*_5) = !5566_u16;
match _13.0 {
0 => bb1,
590900491 => bb4,
_ => bb2
}
}
bb4 = {
_12 = [3410675673742832460_usize,17003729854539745510_usize,10081099331630027813_usize];
_14.fld5.0 = _4;
_7.fld1 = (17695_i16,);
match _13.3 {
0 => bb5,
1 => bb6,
2 => bb7,
12509909960018081487890590905803600814 => bb9,
_ => bb8
}
}
bb5 = {
_7.fld2 = _7.fld0 as u32;
_7.fld3.1 = 590900491_i32;
_9.0 = 127_i8;
_7.fld3.0 = _4;
_1 = (*_5) as f64;
_4 = _7.fld3.0 * _7.fld3.0;
(*_3) = -_1;
_7.fld2 = 3883092491_u32;
_2 = _4 as f64;
_9.0 = 103_i8;
_6 = 10368594735342844310_u64;
RET = '\u{e606b}';
_2 = -_1;
(*_3) = 859835633954216067_i64 as f64;
_14.fld4.2 = core::ptr::addr_of!((*_3));
_9.0 = !(-2_i8);
_13 = (_7.fld3.1, _7.fld3.1, _6, 12509909960018081487890590905803600814_u128);
(*_5) = !5566_u16;
match _13.0 {
0 => bb1,
590900491 => bb4,
_ => bb2
}
}
bb6 = {
_9.0 = -118_i8;
RET = '\u{8c47c}';
_7.fld3.0 = -_4;
(*_3) = -_1;
_5 = core::ptr::addr_of!((*_5));
_1 = (*_3) + (*_3);
_7.fld3.0 = _4;
RET = '\u{c45c4}';
_6 = 12079261582289886918_u64;
_7.fld0 = 204_u8 ^ 185_u8;
_5 = core::ptr::addr_of!((*_5));
RET = '\u{784ce}';
RET = '\u{5a28b}';
_7.fld3.2 = ['\u{104e6c}','\u{c5d74}','\u{17d0b}'];
_7.fld2 = _1 as u32;
_1 = (*_3) * _2;
_5 = core::ptr::addr_of!((*_5));
Goto(bb3)
}
bb7 = {
_4 = 119_u8 as f32;
RET = '\u{d0d60}';
_3 = core::ptr::addr_of!((*_3));
_2 = (-28904_i16) as f64;
(*_3) = -_1;
(*_5) = !51042_u16;
_1 = (*_3) * (*_3);
_6 = 15198391597680537160_u64;
_5 = core::ptr::addr_of!((*_5));
(*_3) = _1 / 1_f64;
RET = '\u{e8fba}';
RET = '\u{f5b26}';
(*_5) = 9223372036854775807_isize as u16;
(*_3) = _1;
_1 = (*_3) / f64::NAN;
(*_3) = _2;
_1 = -(*_3);
_2 = 302301359_u32 as f64;
_1 = 4_usize as f64;
_2 = _6 as f64;
_6 = (-608876655_i32) as u64;
_1 = (*_3) / f64::NEG_INFINITY;
_7.fld1 = ((-24300_i16),);
RET = '\u{7099d}';
_7.fld3.1 = -212975059_i32;
_4 = 100269390790329819616498052756657726176_u128 as f32;
(*_5) = !23417_u16;
RET = '\u{30364}';
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_3 = core::ptr::addr_of!((*_3));
_13.2 = _6;
(*_5) = _13.3 as u16;
_13.2 = _7.fld0 as u64;
match _7.fld2 {
3883092491 => bb10,
_ => bb3
}
}
bb10 = {
_7.fld3.2 = ['\u{549bf}','\u{808ba}','\u{1052ff}'];
_7.fld4 = !_7.fld1.0;
_14.fld5.2 = ['\u{109ac1}','\u{2f893}','\u{591f7}'];
_14.fld4.1 = (*_3);
_11 = _5;
_13.1 = !_13.0;
_15 = [_7.fld0,_7.fld0,_7.fld0];
_14.fld0 = core::ptr::addr_of_mut!(_7.fld3.2);
Goto(bb11)
}
bb11 = {
_18 = (*_5) as isize;
_7.fld4 = _7.fld1.0;
RET = '\u{374b5}';
_14.fld5.1 = (*_3) as i32;
_14.fld4.2 = core::ptr::addr_of!(_2);
_7.fld3 = _14.fld5;
_8.0 = [_13.0,_13.0,_14.fld5.1,_7.fld3.1,_13.0,_7.fld3.1];
_14.fld4.0 = _14.fld5.0 + _4;
_20 = !_18;
_15 = [_7.fld0,_7.fld0,_7.fld0];
_22.fld4 = 713701906758349938_i64 as i16;
_22.fld3 = (_4, _7.fld3.1, _7.fld3.2);
_17 = Adt62 { fld0: _9.0 };
Goto(bb12)
}
bb12 = {
(*_11) = 35291_u16 | 191_u16;
_22.fld3.2 = ['\u{f17c3}','\u{2af62}','\u{4865}'];
(*_5) = 2790_u16 - 34788_u16;
_7.fld2 = _13.3 as u32;
_14.fld5.0 = _7.fld0 as f32;
_14.fld4.1 = -(*_3);
_15 = [_7.fld0,_7.fld0,_7.fld0];
_22.fld1 = (_7.fld4,);
_7.fld3 = (_14.fld5.0, _14.fld5.1, _14.fld5.2);
_22.fld3 = _7.fld3;
_17 = Adt62 { fld0: _9.0 };
_7.fld1 = (_22.fld4,);
_7.fld2 = 4006401449_u32 - 2266395693_u32;
_7 = Adt53 { fld0: 79_u8,fld1: _22.fld1,fld2: 1826250019_u32,fld3: _22.fld3,fld4: _22.fld1.0 };
_14.fld3 = 12816755348655521069_usize & 4_usize;
_14.fld4.0 = _4;
_25 = (_14.fld4.2,);
(*_3) = _14.fld3 as f64;
_7.fld3 = (_14.fld4.0, _22.fld3.1, _22.fld3.2);
Goto(bb13)
}
bb13 = {
_25 = (_3,);
_21 = core::ptr::addr_of!(_13.2);
_22 = Adt53 { fld0: _7.fld0,fld1: _7.fld1,fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld1.0 };
_7.fld2 = !_22.fld2;
(*_3) = _7.fld2 as f64;
_19 = (*_11) as f32;
_6 = (*_21);
(*_3) = -_2;
_22.fld1.0 = _7.fld4 >> (*_5);
_14.fld4 = (_22.fld3.0, _1, _3, _2);
_9.0 = _17.fld0 >> _7.fld3.1;
_10 = _22.fld2;
Goto(bb14)
}
bb14 = {
_7.fld3.2 = _14.fld5.2;
_32 = _18 << _14.fld5.1;
_11 = core::ptr::addr_of!((*_11));
_2 = _14.fld3 as f64;
(*_11) = 46874301826748361534196591090975567966_i128 as u16;
_14.fld5.2 = ['\u{65e97}','\u{f4c3a}','\u{fbf28}'];
_7.fld3.1 = _14.fld5.1;
_13.3 = 158733472444283774300294478307857035772_u128;
_22.fld1.0 = _22.fld0 as i16;
_13.3 = 258544871869770949579923464940894091453_u128 / 200006863022137431514601068127489130648_u128;
_14.fld1 = _12;
_11 = core::ptr::addr_of!((*_5));
_2 = _14.fld4.3;
_25.0 = core::ptr::addr_of!(_28);
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(16_usize, 32_usize, Move(_32), 20_usize, Move(_20), 18_usize, Move(_18), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(16_usize, 12_usize, Move(_12), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: u16,mut _2: *const u16,mut _3: *const u16) -> f64 {
mir! {
type RET = f64;
let _4: bool;
let _5: ([i32; 6],);
let _6: *mut [char; 3];
let _7: isize;
let _8: i128;
let _9: bool;
let _10: Adt60;
let _11: i32;
let _12: f32;
let _13: Adt50;
let _14: [u8; 2];
let _15: isize;
let _16: isize;
let _17: u128;
let _18: Adt62;
let _19: (i16,);
let _20: Adt59;
let _21: Adt58;
let _22: [usize; 3];
let _23: bool;
let _24: usize;
let _25: (i32, i32, u64, u128);
let _26: [usize; 3];
let _27: u8;
let _28: [usize; 3];
let _29: isize;
let _30: bool;
let _31: *mut ((u64, bool),);
let _32: u32;
let _33: *const u8;
let _34: (u64, bool);
let _35: ();
let _36: ();
{
_1 = (*_3) | (*_3);
RET = _1 as f64;
_1 = 7_usize as u16;
(*_2) = _1;
_5.0 = [1918870626_i32,(-1720016562_i32),(-1627506356_i32),862661140_i32,241118949_i32,(-1571382327_i32)];
_4 = !true;
_3 = _2;
(*_3) = !_1;
RET = 13843902423844375250_usize as f64;
_4 = (*_3) > (*_3);
_3 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_3));
_4 = !false;
_9 = _4 != _4;
_3 = core::ptr::addr_of!((*_2));
(*_2) = _9 as u16;
_1 = (*_3) / 17364_u16;
_8 = 129653926968229269256780339055117216586_i128;
_2 = core::ptr::addr_of!((*_2));
RET = 2111279694_u32 as f64;
_9 = (*_3) <= _1;
Goto(bb1)
}
bb1 = {
(*_3) = !_1;
_3 = core::ptr::addr_of!((*_2));
_11 = (-540171690_i32);
(*_3) = !_1;
_7 = !(-9223372036854775808_isize);
_4 = !_9;
_10.fld1 = -_8;
_12 = _10.fld1 as f32;
_4 = _9 | _9;
RET = _7 as f64;
_13.fld3.3 = 15531732350045697992_u64 as f64;
_10.fld1 = _8;
_13.fld3.1 = _13.fld3.3 - _13.fld3.3;
_1 = (*_3);
Goto(bb2)
}
bb2 = {
_15 = 6663869498481166807_i64 as isize;
_13.fld3.0 = 60_u8 as f32;
_13.fld1 = [246_u8,18_u8];
_19.0 = 22801_i16;
_20.fld0 = core::ptr::addr_of!(_13.fld3.3);
_21.fld5.fld1.2 = _13.fld1;
_20.fld1.0 = _20.fld0;
_21.fld1.1.3 = 18860773355433321436087772885915268229_u128 / 234191104293282466936183287577282024212_u128;
_21.fld5.fld4 = !3369737302_u32;
_21.fld5.fld1.2 = _13.fld1;
_21.fld1.3 = _4;
_17 = _21.fld1.1.3 / 331405956937902903177807752769316575292_u128;
_21.fld5.fld1.3 = _21.fld1.3;
RET = -_13.fld3.1;
_1 = (-77_i8) as u16;
_21.fld5.fld5 = _11 ^ _11;
_21.fld5.fld1.1.1 = !_21.fld5.fld5;
_13.fld3.2 = _20.fld0;
_21.fld1.2 = [219_u8,91_u8];
_21.fld5.fld1.1.1 = _21.fld5.fld5;
_20.fld6 = (*_3) & (*_3);
_6 = core::ptr::addr_of_mut!(_21.fld2.2);
_21.fld3.fld0 = core::ptr::addr_of!(_13.fld3.3);
match _10.fld1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
129653926968229269256780339055117216586 => bb8,
_ => bb7
}
}
bb3 = {
(*_3) = !_1;
_3 = core::ptr::addr_of!((*_2));
_11 = (-540171690_i32);
(*_3) = !_1;
_7 = !(-9223372036854775808_isize);
_4 = !_9;
_10.fld1 = -_8;
_12 = _10.fld1 as f32;
_4 = _9 | _9;
RET = _7 as f64;
_13.fld3.3 = 15531732350045697992_u64 as f64;
_10.fld1 = _8;
_13.fld3.1 = _13.fld3.3 - _13.fld3.3;
_1 = (*_3);
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
_21.fld5.fld1.1.0 = -_21.fld5.fld5;
_21.fld1.0 = core::ptr::addr_of_mut!(_5.0);
_13.fld3.2 = _21.fld3.fld0;
_21.fld5.fld4 = 1180613878_u32;
_13.fld2 = core::ptr::addr_of_mut!(_21.fld2.2);
_23 = _21.fld1.3 == _9;
_21.fld1.1.0 = _21.fld5.fld1.1.0;
_20.fld5 = core::ptr::addr_of_mut!(_21.fld5.fld1);
_18 = Adt62 { fld0: 55_i8 };
_21.fld5.fld1.1.1 = _13.fld3.0 as i32;
_21.fld4 = _19.0;
_21.fld0 = [1684792126584772956_usize,13590356185130637834_usize,14303778047465125441_usize];
_21.fld2.1 = _21.fld1.1.0;
_22 = [3870817319741355996_usize,7_usize,4546234096689094794_usize];
_8 = -_10.fld1;
match _21.fld4 {
0 => bb3,
22801 => bb9,
_ => bb4
}
}
bb9 = {
_20.fld2 = _15 >> _21.fld5.fld1.1.0;
_21.fld1.1.2 = 15780567765762731213_u64;
_21.fld3.fld3 = 7_usize;
_16 = -_20.fld2;
_21.fld1.1 = (_21.fld2.1, _21.fld5.fld1.1.1, 10500233527643127542_u64, _17);
_21.fld2.2 = ['\u{3a483}','\u{40f6b}','\u{aab43}'];
(*_3) = _20.fld6;
_20.fld6 = (*_3) | (*_3);
match _21.fld1.1.2 {
0 => bb1,
1 => bb7,
2 => bb4,
3 => bb10,
4 => bb11,
10500233527643127542 => bb13,
_ => bb12
}
}
bb10 = {
_21.fld5.fld1.1.0 = -_21.fld5.fld5;
_21.fld1.0 = core::ptr::addr_of_mut!(_5.0);
_13.fld3.2 = _21.fld3.fld0;
_21.fld5.fld4 = 1180613878_u32;
_13.fld2 = core::ptr::addr_of_mut!(_21.fld2.2);
_23 = _21.fld1.3 == _9;
_21.fld1.1.0 = _21.fld5.fld1.1.0;
_20.fld5 = core::ptr::addr_of_mut!(_21.fld5.fld1);
_18 = Adt62 { fld0: 55_i8 };
_21.fld5.fld1.1.1 = _13.fld3.0 as i32;
_21.fld4 = _19.0;
_21.fld0 = [1684792126584772956_usize,13590356185130637834_usize,14303778047465125441_usize];
_21.fld2.1 = _21.fld1.1.0;
_22 = [3870817319741355996_usize,7_usize,4546234096689094794_usize];
_8 = -_10.fld1;
match _21.fld4 {
0 => bb3,
22801 => bb9,
_ => bb4
}
}
bb11 = {
(*_3) = !_1;
_3 = core::ptr::addr_of!((*_2));
_11 = (-540171690_i32);
(*_3) = !_1;
_7 = !(-9223372036854775808_isize);
_4 = !_9;
_10.fld1 = -_8;
_12 = _10.fld1 as f32;
_4 = _9 | _9;
RET = _7 as f64;
_13.fld3.3 = 15531732350045697992_u64 as f64;
_10.fld1 = _8;
_13.fld3.1 = _13.fld3.3 - _13.fld3.3;
_1 = (*_3);
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_20.fld4 = core::ptr::addr_of!(_21.fld5.fld1.1.2);
(*_2) = '\u{feb4c}' as u16;
_21.fld2.0 = _18.fld0 as f32;
_16 = -_7;
_13.fld3.3 = (-6833323271850191974_i64) as f64;
_19.0 = _21.fld4 * _21.fld4;
_19 = (_21.fld4,);
_20.fld2 = _10.fld1 as isize;
_21.fld5.fld1.1 = _21.fld1.1;
_8 = _21.fld5.fld1.1.2 as i128;
_25.0 = -_21.fld5.fld5;
_27 = 228_u8;
_25.3 = _19.0 as u128;
_2 = _3;
(*_3) = _21.fld1.3 as u16;
Goto(bb14)
}
bb14 = {
_24 = !_21.fld3.fld3;
_1 = _21.fld1.1.2 as u16;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(17_usize, 8_usize, Move(_8), 9_usize, Move(_9), 23_usize, Move(_23), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(17_usize, 24_usize, Move(_24), 22_usize, Move(_22), 16_usize, Move(_16), 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: *mut [char; 3],mut _2: i32,mut _3: (i8,),mut _4: u64,mut _5: usize,mut _6: i16,mut _7: bool,mut _8: [i32; 6],mut _9: i32,mut _10: u32,mut _11: f64,mut _12: bool) -> (*const f64,) {
mir! {
type RET = (*const f64,);
let _13: i16;
let _14: i64;
let _15: f64;
let _16: i32;
let _17: ((u64, bool),);
let _18: isize;
let _19: (f32, i32, [char; 3]);
let _20: bool;
let _21: Adt58;
let _22: f64;
let _23: char;
let _24: f64;
let _25: (f32, i32, [char; 3]);
let _26: char;
let _27: ();
let _28: ();
{
_11 = 110_isize as f64;
_12 = _7;
(*_1) = ['\u{262c5}','\u{c0d56}','\u{ae069}'];
_12 = !_7;
_7 = !_12;
_13 = 41_u8 as i16;
RET.0 = core::ptr::addr_of!(_11);
RET.0 = core::ptr::addr_of!(_15);
_14 = 4125768120935919708_i64 * 7267792041268735544_i64;
_13 = -_6;
_3 = (1_i8,);
(*_1) = ['\u{58d59}','\u{cf8f2}','\u{41ecb}'];
(*_1) = ['\u{393f1}','\u{e6b21}','\u{8c32a}'];
_10 = _6 as u32;
_2 = _9;
_17.0 = (_4, _7);
RET.0 = core::ptr::addr_of!(_11);
_17.0 = (_4, _7);
_8 = [_2,_2,_2,_2,_2,_2];
Goto(bb1)
}
bb1 = {
_15 = _11;
_10 = '\u{103f59}' as u32;
_12 = !_7;
_13 = '\u{103fc5}' as i16;
RET.0 = core::ptr::addr_of!(_15);
_12 = _7;
_10 = 2388548850_u32 << _13;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = _17.0.0 / 6339125043035863208_u64;
_9 = _2 >> _5;
RET.0 = core::ptr::addr_of!(_11);
_3 = (13_i8,);
_14 = (-9223372036854775808_isize) as i64;
_16 = _9;
Goto(bb2)
}
bb2 = {
_11 = _15 + _15;
_17.0.0 = _4 + _4;
_5 = 10217519749754827681_usize - 0_usize;
_3.0 = 127_i8 << _9;
RET.0 = core::ptr::addr_of!(_15);
_15 = -_11;
_17.0.0 = _4 << _10;
_4 = _14 as u64;
_15 = 302367633160407650057285488503017113406_u128 as f64;
(*_1) = ['\u{e51e3}','\u{9ee0f}','\u{62bcc}'];
_15 = -_11;
match _6 {
0 => bb3,
1 => bb4,
2 => bb5,
23978 => bb7,
_ => bb6
}
}
bb3 = {
_15 = _11;
_10 = '\u{103f59}' as u32;
_12 = !_7;
_13 = '\u{103fc5}' as i16;
RET.0 = core::ptr::addr_of!(_15);
_12 = _7;
_10 = 2388548850_u32 << _13;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = _17.0.0 / 6339125043035863208_u64;
_9 = _2 >> _5;
RET.0 = core::ptr::addr_of!(_11);
_3 = (13_i8,);
_14 = (-9223372036854775808_isize) as i64;
_16 = _9;
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
_3 = (31_i8,);
_7 = _17.0.0 != _17.0.0;
_3 = ((-58_i8),);
_11 = _15;
_8 = [_16,_16,_16,_9,_9,_16];
_14 = !(-4377636510851779404_i64);
_12 = _7 == _17.0.1;
RET.0 = core::ptr::addr_of!(_15);
_15 = _11 - _11;
RET.0 = core::ptr::addr_of!(_11);
(*_1) = ['\u{31fe6}','\u{3165}','\u{29c8b}'];
(*_1) = ['\u{43615}','\u{ee1f4}','\u{77d00}'];
_4 = _17.0.0 | _17.0.0;
_17.0.0 = !_4;
_3 = (95_i8,);
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb8)
}
bb8 = {
_10 = !3837703022_u32;
_5 = !9720809153368097326_usize;
_18 = 9223372036854775807_isize;
_5 = 10609794293739121236_usize % 6_usize;
_7 = _12;
RET.0 = core::ptr::addr_of!(_11);
_14 = (-4467993306564802879_i64);
match _18 {
0 => bb4,
1 => bb6,
9223372036854775807 => bb10,
_ => bb9
}
}
bb9 = {
_15 = _11;
_10 = '\u{103f59}' as u32;
_12 = !_7;
_13 = '\u{103fc5}' as i16;
RET.0 = core::ptr::addr_of!(_15);
_12 = _7;
_10 = 2388548850_u32 << _13;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = _17.0.0 / 6339125043035863208_u64;
_9 = _2 >> _5;
RET.0 = core::ptr::addr_of!(_11);
_3 = (13_i8,);
_14 = (-9223372036854775808_isize) as i64;
_16 = _9;
Goto(bb2)
}
bb10 = {
_6 = _13;
RET.0 = core::ptr::addr_of!(_11);
_16 = _9 ^ _9;
_19.0 = 8623078484623729456391538366548478728_i128 as f32;
_10 = !867689680_u32;
_5 = 6_usize >> _6;
_3 = ((-22_i8),);
_16 = -_9;
_1 = core::ptr::addr_of_mut!((*_1));
_7 = _12;
_20 = !_12;
_21.fld5.fld1.0 = core::ptr::addr_of_mut!(_8);
_21.fld2.0 = _19.0 / 0.0000000000000000000000000000000000000025286556905603113_f32;
_13 = _6 << _16;
_7 = _12;
(*_1) = ['\u{f721b}','\u{fd2a7}','\u{108d5f}'];
_21.fld5.fld1.1.1 = -_9;
_19.2 = (*_1);
_21.fld5.fld1.1.0 = !_16;
_19.1 = _16 >> _13;
Goto(bb11)
}
bb11 = {
_21.fld5.fld4 = _10 ^ _10;
_21.fld5.fld1.1 = (_2, _19.1, _4, 232524983653029924334025746306086593496_u128);
_11 = _18 as f64;
_21.fld1.0 = _21.fld5.fld1.0;
_17.0.0 = _4 % 1376695660575069061_u64;
_21.fld1.3 = !_12;
_6 = _18 as i16;
_7 = !_20;
_21.fld5.fld1.1.1 = !_9;
_6 = _5 as i16;
(*_1) = ['\u{dd640}','\u{41c16}','\u{af36e}'];
_18 = 108_isize;
_16 = _21.fld5.fld1.1.1 + _19.1;
_3 = ((-66_i8),);
_21.fld4 = _13;
_7 = _12 | _12;
_21.fld5.fld1.2 = [147_u8,126_u8];
_21.fld5.fld3 = [_21.fld4,_13,_13,_6,_21.fld4,_13,_21.fld4];
_21.fld5.fld1.0 = _21.fld1.0;
_16 = _2;
_21.fld5.fld1.1.1 = _18 as i32;
_13 = _6;
_21.fld1.1.0 = -_9;
Goto(bb12)
}
bb12 = {
_21.fld2.1 = _19.1 + _19.1;
_21.fld5.fld5 = -_21.fld2.1;
_21.fld2 = (_19.0, _21.fld5.fld5, _19.2);
_9 = _21.fld5.fld5 >> _5;
_16 = _21.fld2.1 ^ _21.fld1.1.0;
match _3.0 {
0 => bb6,
1 => bb9,
2 => bb13,
3 => bb14,
4 => bb15,
340282366920938463463374607431768211390 => bb17,
_ => bb16
}
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
_10 = !3837703022_u32;
_5 = !9720809153368097326_usize;
_18 = 9223372036854775807_isize;
_5 = 10609794293739121236_usize % 6_usize;
_7 = _12;
RET.0 = core::ptr::addr_of!(_11);
_14 = (-4467993306564802879_i64);
match _18 {
0 => bb4,
1 => bb6,
9223372036854775807 => bb10,
_ => bb9
}
}
bb17 = {
_21.fld1 = (_21.fld5.fld1.0, _21.fld5.fld1.1, _21.fld5.fld1.2, _12);
_19.0 = -_21.fld2.0;
_3.0 = -20_i8;
_21.fld5.fld5 = !_16;
_21.fld5.fld1.1 = (_16, _21.fld2.1, _21.fld1.1.2, _21.fld1.1.3);
_21.fld1.1.2 = !_17.0.0;
_21.fld3.fld1 = '\u{f466e}';
_25.1 = _16;
_26 = _21.fld3.fld1;
Goto(bb18)
}
bb18 = {
Call(_27 = dump_var(18_usize, 12_usize, Move(_12), 20_usize, Move(_20), 16_usize, Move(_16), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(18_usize, 18_usize, Move(_18), 2_usize, Move(_2), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i16,mut _2: i16) -> char {
mir! {
type RET = char;
let _3: i32;
let _4: (i32, i32, u64, u128);
let _5: u32;
let _6: i16;
let _7: usize;
let _8: i16;
let _9: isize;
let _10: Adt61;
let _11: [i16; 7];
let _12: isize;
let _13: [u8; 2];
let _14: *mut ((u64, bool),);
let _15: ();
let _16: ();
{
RET = '\u{6b4cb}';
RET = '\u{10aa94}';
RET = '\u{a644a}';
_1 = _2 & _2;
_1 = 209069408612074985811624531440683853440_u128 as i16;
_2 = _1;
_1 = _2;
RET = '\u{9ff8b}';
_1 = _2 & _2;
_1 = true as i16;
_2 = _1;
_1 = _2 ^ _2;
_1 = _2 << _2;
Goto(bb1)
}
bb1 = {
RET = '\u{66b4c}';
RET = '\u{dbf1a}';
_1 = _2 << _2;
_2 = _1 >> _1;
_3 = (-1015448798_i32) + (-615008554_i32);
_1 = _2 & _2;
RET = '\u{29ede}';
RET = '\u{8665c}';
_1 = _2 & _2;
RET = '\u{2b3ce}';
_4.2 = 8514444414031086549_u64 & 6981779585002319867_u64;
_4.3 = 157159708051544379702719753271849519420_u128 & 113966150851764236391675336566380548506_u128;
_4.1 = _3 ^ _3;
Goto(bb2)
}
bb2 = {
RET = '\u{bc335}';
_4 = (_3, _3, 15924370847331087567_u64, 10800355196937113727422823943645700474_u128);
_4.1 = !_4.0;
_2 = 14775_u16 as i16;
_4.1 = _3;
_4 = (_3, _3, 12483342865216332689_u64, 78128350396787688487407071839393896694_u128);
_5 = 57597_u16 as u32;
_4.3 = 304323703575615492226889717357580452855_u128;
_1 = !_2;
RET = '\u{adbe1}';
_2 = _1 - _1;
_4.2 = !13725183456677564329_u64;
Goto(bb3)
}
bb3 = {
_7 = '\u{720bf}' as usize;
_7 = 9433181822293584875_usize;
_6 = _2;
_8 = _1;
_4.3 = !292975683806043913925566353050988938093_u128;
_4 = (_3, _3, 9828082242683865608_u64, 107336836784320597068126982273265755419_u128);
match _4.2 {
0 => bb4,
1 => bb5,
9828082242683865608 => bb7,
_ => bb6
}
}
bb4 = {
RET = '\u{bc335}';
_4 = (_3, _3, 15924370847331087567_u64, 10800355196937113727422823943645700474_u128);
_4.1 = !_4.0;
_2 = 14775_u16 as i16;
_4.1 = _3;
_4 = (_3, _3, 12483342865216332689_u64, 78128350396787688487407071839393896694_u128);
_5 = 57597_u16 as u32;
_4.3 = 304323703575615492226889717357580452855_u128;
_1 = !_2;
RET = '\u{adbe1}';
_2 = _1 - _1;
_4.2 = !13725183456677564329_u64;
Goto(bb3)
}
bb5 = {
RET = '\u{66b4c}';
RET = '\u{dbf1a}';
_1 = _2 << _2;
_2 = _1 >> _1;
_3 = (-1015448798_i32) + (-615008554_i32);
_1 = _2 & _2;
RET = '\u{29ede}';
RET = '\u{8665c}';
_1 = _2 & _2;
RET = '\u{2b3ce}';
_4.2 = 8514444414031086549_u64 & 6981779585002319867_u64;
_4.3 = 157159708051544379702719753271849519420_u128 & 113966150851764236391675336566380548506_u128;
_4.1 = _3 ^ _3;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
RET = '\u{c2aa5}';
_4.3 = !7195984630047951637932781351035892122_u128;
_10.fld1.fld5.fld1 = '\u{d9bab}';
Goto(bb8)
}
bb8 = {
_10.fld1.fld5.fld0 = core::ptr::addr_of!(_10.fld1.fld0.3);
_10.fld1.fld0.0 = (-126361294108855620039555232093403381687_i128) as f32;
_10.fld1.fld5.fld1 = '\u{226cc}';
_10.fld0 = core::ptr::addr_of!(_4.2);
_4 = (_3, _3, 2787675398090254821_u64, 194748098134283459304163960673437650710_u128);
_10.fld1.fld2.0 = core::ptr::addr_of!(_10.fld1.fld0.1);
_10.fld1.fld1.0 = [_3,_4.1,_4.1,_3,_4.0,_4.1];
_10.fld1.fld5.fld0 = core::ptr::addr_of!(_10.fld1.fld0.3);
match _4.2 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
2787675398090254821 => bb17,
_ => bb16
}
}
bb9 = {
RET = '\u{c2aa5}';
_4.3 = !7195984630047951637932781351035892122_u128;
_10.fld1.fld5.fld1 = '\u{d9bab}';
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
RET = '\u{66b4c}';
RET = '\u{dbf1a}';
_1 = _2 << _2;
_2 = _1 >> _1;
_3 = (-1015448798_i32) + (-615008554_i32);
_1 = _2 & _2;
RET = '\u{29ede}';
RET = '\u{8665c}';
_1 = _2 & _2;
RET = '\u{2b3ce}';
_4.2 = 8514444414031086549_u64 & 6981779585002319867_u64;
_4.3 = 157159708051544379702719753271849519420_u128 & 113966150851764236391675336566380548506_u128;
_4.1 = _3 ^ _3;
Goto(bb2)
}
bb12 = {
RET = '\u{bc335}';
_4 = (_3, _3, 15924370847331087567_u64, 10800355196937113727422823943645700474_u128);
_4.1 = !_4.0;
_2 = 14775_u16 as i16;
_4.1 = _3;
_4 = (_3, _3, 12483342865216332689_u64, 78128350396787688487407071839393896694_u128);
_5 = 57597_u16 as u32;
_4.3 = 304323703575615492226889717357580452855_u128;
_1 = !_2;
RET = '\u{adbe1}';
_2 = _1 - _1;
_4.2 = !13725183456677564329_u64;
Goto(bb3)
}
bb13 = {
_7 = '\u{720bf}' as usize;
_7 = 9433181822293584875_usize;
_6 = _2;
_8 = _1;
_4.3 = !292975683806043913925566353050988938093_u128;
_4 = (_3, _3, 9828082242683865608_u64, 107336836784320597068126982273265755419_u128);
match _4.2 {
0 => bb4,
1 => bb5,
9828082242683865608 => bb7,
_ => bb6
}
}
bb14 = {
RET = '\u{bc335}';
_4 = (_3, _3, 15924370847331087567_u64, 10800355196937113727422823943645700474_u128);
_4.1 = !_4.0;
_2 = 14775_u16 as i16;
_4.1 = _3;
_4 = (_3, _3, 12483342865216332689_u64, 78128350396787688487407071839393896694_u128);
_5 = 57597_u16 as u32;
_4.3 = 304323703575615492226889717357580452855_u128;
_1 = !_2;
RET = '\u{adbe1}';
_2 = _1 - _1;
_4.2 = !13725183456677564329_u64;
Goto(bb3)
}
bb15 = {
RET = '\u{66b4c}';
RET = '\u{dbf1a}';
_1 = _2 << _2;
_2 = _1 >> _1;
_3 = (-1015448798_i32) + (-615008554_i32);
_1 = _2 & _2;
RET = '\u{29ede}';
RET = '\u{8665c}';
_1 = _2 & _2;
RET = '\u{2b3ce}';
_4.2 = 8514444414031086549_u64 & 6981779585002319867_u64;
_4.3 = 157159708051544379702719753271849519420_u128 & 113966150851764236391675336566380548506_u128;
_4.1 = _3 ^ _3;
Goto(bb2)
}
bb16 = {
Return()
}
bb17 = {
_10.fld1.fld3 = [_7,_7,_7];
RET = _10.fld1.fld5.fld1;
_10.fld1.fld0.2 = core::ptr::addr_of!(_10.fld1.fld0.3);
_10.fld1.fld0.1 = _1 as f64;
_10.fld1.fld4 = [(-121_i8),38_i8];
_10.fld1.fld3 = [_7,_7,_7];
_10.fld1.fld3 = [_7,_7,_7];
RET = _10.fld1.fld5.fld1;
_9 = 9223372036854775807_isize + (-9223372036854775808_isize);
_10.fld1.fld3 = [_7,_7,_7];
_10.fld1.fld5.fld1 = '\u{8eba1}';
_4.1 = _4.0;
_5 = !2670586150_u32;
_11 = [_2,_6,_1,_1,_2,_6,_2];
_1 = _6 ^ _2;
_3 = _4.0 - _4.0;
_10.fld1.fld0.0 = _4.2 as f32;
_13 = [143_u8,206_u8];
_7 = !5_usize;
_13 = [223_u8,236_u8];
Goto(bb18)
}
bb18 = {
Call(_15 = dump_var(19_usize, 8_usize, Move(_8), 13_usize, Move(_13), 5_usize, Move(_5), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_15 = dump_var(19_usize, 6_usize, Move(_6), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(6326899740409810229_usize), std::hint::black_box(577_u16), std::hint::black_box(2085248741565477244_i64), std::hint::black_box((-78_i8)), std::hint::black_box(2564242604_u32), std::hint::black_box(3090164762414774140_u64));
                
            }
#[derive(Debug)]
pub struct Adt47 {
fld0: *mut char,
}
#[derive(Debug)]
pub struct Adt48 {
fld0: *mut char,
fld1: (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool),
fld2: *mut ((u64, bool),),
fld3: [i16; 7],
fld4: u32,
fld5: i32,
}
#[derive(Debug)]
pub struct Adt49 {
fld0: (i16,),
fld1: char,
fld2: (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool),
fld3: usize,
fld4: *mut (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool),
fld5: *const u16,
fld6: [u8; 2],
fld7: (*const f64,),
}
#[derive(Debug)]
pub struct Adt50 {
fld0: *const u8,
fld1: [u8; 2],
fld2: *mut [char; 3],
fld3: (f32, f64, *const f64, f64),
}
#[derive(Debug)]
pub struct Adt51 {
fld0: *const f64,
fld1: char,
fld2: *mut i128,
fld3: usize,
}
#[derive(Debug)]
pub struct Adt52 {
fld0: *mut ((u64, bool),),
fld1: i64,
fld2: u16,
fld3: u64,
fld4: *const u16,
fld5: *mut (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool),
}
#[derive(Debug)]
pub struct Adt53 {
fld0: u8,
fld1: (i16,),
fld2: u32,
fld3: (f32, i32, [char; 3]),
fld4: i16,
}
#[derive(Debug)]
pub struct Adt54 {
fld0: [usize; 3],
fld1: *mut (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool),
fld2: Adt52,
fld3: (i32, i32, u64, u128),
}
#[derive(Debug)]
pub struct Adt55 {
fld0: *mut [char; 3],
fld1: [usize; 3],
fld2: Adt47,
fld3: usize,
fld4: (f32, f64, *const f64, f64),
fld5: (f32, i32, [char; 3]),
}
#[derive(Debug)]
pub struct Adt56 {
fld0: u128,
fld1: (u32, *mut [char; 3], [usize; 3], (*const f64,), *mut [i32; 6]),
fld2: Adt53,
fld3: *mut ((u64, bool),),
}
#[derive(Debug)]
pub struct Adt57 {
fld0: (f32, f64, *const f64, f64),
fld1: ([i32; 6],),
fld2: (*const f64,),
fld3: [usize; 3],
fld4: [i8; 2],
fld5: Adt51,
fld6: i64,
fld7: *const u8,
}
#[derive(Debug)]
pub struct Adt58 {
fld0: [usize; 3],
fld1: (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool),
fld2: (f32, i32, [char; 3]),
fld3: Adt51,
fld4: i16,
fld5: Adt48,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt59 {
fld0: *const f64,
fld1: (*const f64,),
fld2: isize,
fld3: u128,
fld4: *const u64,
fld5: *mut (*mut [i32; 6], (i32, i32, u64, u128), [u8; 2], bool),
fld6: u16,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt60 {
fld0: *mut ((u64, bool),),
fld1: i128,
}
#[derive(Debug)]
pub struct Adt61 {
fld0: *const u64,
fld1: Adt57,
fld2: *const u16,
}
#[derive(Debug)]
pub struct Adt62 {
fld0: i8,
}
#[derive(Debug)]
pub struct Adt63 {
fld0: *mut ((u64, bool),),
fld1: [i32; 6],
fld2: u128,
fld3: [usize; 3],
fld4: Adt57,
fld5: (i16,),
fld6: *mut i128,
}

