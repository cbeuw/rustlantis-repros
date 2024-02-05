#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics, const_hash)]
    #![allow(unused_parens, unused_assignments, overflowing_literals)]
    extern crate core;
    use core::intrinsics::mir::*;

    use std::fmt::Debug;

    #[inline(never)]
    pub fn dump_var<T: Debug, U: Debug, V: Debug, W: Debug>(f: usize,
        var0: usize, val0: T,
        var1: usize, val1: U,
        var2: usize, val2: V,
        var3: usize, val3: W,
    ) {
        println!("fn{f}:_{var0} = {val0:?}\n_{var1} = {val1:?}\n_{var2} = {val2:?}\n_{var3} = {val3:?}");
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: bool,mut _2: u32,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16) -> [u8; 1] {
mir! {
type RET = [u8; 1];
let _12: f64;
let _13: bool;
let _14: u32;
let _15: u64;
let _16: *const u128;
let _17: *const *const usize;
let _18: Adt55;
let _19: [i32; 5];
let _20: bool;
let _21: Adt47;
let _22: isize;
let _23: i64;
let _24: (u128,);
let _25: (usize,);
let _26: (i64, u32);
let _27: bool;
let _28: u8;
let _29: (u128,);
let _30: [isize; 5];
let _31: Adt48;
let _32: isize;
let _33: i128;
let _34: i128;
let _35: (i32, *const [u8; 1]);
let _36: [i8; 4];
let _37: ();
let _38: ();
{
_11 = 31179_u16;
_2 = 3462981822_u32;
_7 = 1944189951000817456_i64 | (-7956315623155778498_i64);
RET = [89_u8];
_8 = (-19118_i16) as i128;
_10 = !94_u8;
_1 = !true;
_3 = (-9223372036854775808_isize) << _7;
_5 = (-4323_i16);
_4 = 104_i8;
_11 = !42721_u16;
_4 = (-114_i8) * (-8_i8);
_10 = 72_u8 / 204_u8;
_4 = -(-41_i8);
_2 = _3 as u32;
_10 = 149_u8;
_2 = 1086322019_u32;
_12 = _4 as f64;
_12 = _5 as f64;
_6 = 988167106_i32;
RET = [_10];
_3 = 23_isize;
match _10 {
0 => bb1,
149 => bb3,
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
_1 = !false;
_2 = 2029441424_u32;
_13 = !_1;
_8 = _5 as i128;
_11 = _13 as u16;
_3 = 37_isize & (-108_isize);
_4 = _10 as i8;
_12 = _11 as f64;
_8 = (-60718315368066716828284178750588868897_i128) * 136881241746909649171326715922541626458_i128;
_13 = !_1;
_12 = _4 as f64;
_9 = 11729073620023706774_usize;
_4 = -(-18_i8);
_13 = !_1;
_11 = _5 as u16;
_13 = _1;
_8 = 34010793856911330369750289765765762620_i128;
_1 = _13;
_13 = _1;
_9 = 1_usize;
_15 = 10128560916999392973_u64 << _8;
_14 = _2 + _2;
_6 = (-1283150635_i32) >> _3;
_4 = !13_i8;
_4 = _5 as i8;
_1 = !_13;
Goto(bb4)
}
bb4 = {
RET = [_10];
_1 = !_13;
_18.fld0.0.1 = (_9,);
_18.fld0.1 = core::ptr::addr_of_mut!(_12);
_18.fld3.3 = core::ptr::addr_of!(_9);
_7 = !6476967726780500781_i64;
_18.fld5 = 220630792861135108503728504855708302925_u128;
_18.fld3.0.0 = !_9;
_18.fld0.0.1 = (_18.fld3.0.0,);
_13 = _1 ^ _1;
_18.fld0.2 = [_3,_3,_3,_3,_3,_3];
_7 = _12 as i64;
_21.fld0[_9] = -_4;
_18.fld3.3 = core::ptr::addr_of!(_9);
_5 = _6 as i16;
_18.fld3.1.0 = _15;
Goto(bb5)
}
bb5 = {
_18.fld7.1[_9] = _6;
_18.fld7.1[_9] = -_6;
_20 = _9 != _18.fld0.0.1.0;
_18.fld3.0 = (_9,);
_7 = _18.fld3.0.0 as i64;
_18.fld3.1 = (_15,);
_7 = -4149520041408350389_i64;
_11 = _3 as u16;
_18.fld7.0 = _18.fld0.2[_9];
_18.fld3.1.0 = _15 >> _10;
RET = [_10];
_18.fld0.1 = core::ptr::addr_of_mut!(_12);
Call(_18.fld3.2 = fn1(_12, _3, _18.fld0.2, _6, _18.fld3.1, _18.fld0.0.1.0, _18.fld3.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1 = !_20;
_18.fld6.0 = (_15,);
_4 = 88_i8;
_16 = core::ptr::addr_of!(_24.0);
_1 = !_20;
_22 = _18.fld5 as isize;
_18.fld7.1 = [_6,_6,_6,_6,_6];
_22 = _18.fld3.2 as isize;
_19 = _18.fld7.1;
_21.fld0 = [_4,_4,_4,_4];
_1 = !_18.fld3.2;
_15 = !_18.fld3.1.0;
_7 = 8602062252548809136_i64 << _2;
_18.fld5 = 290570187033953975960448278093656680127_u128;
_18.fld6.0 = _18.fld3.1;
_18.fld1 = (_15,);
_18.fld7 = (_3, _19);
_18.fld7.0 = !_3;
_18.fld3.1 = (_15,);
(*_16) = '\u{c6a81}' as u128;
_11 = !64183_u16;
_18.fld7.1 = [_6,_6,_6,_6,_6];
_1 = _24.0 < _18.fld5;
_18.fld1.0 = _8 as u64;
_18.fld0.2 = [_3,_3,_22,_3,_3,_3];
_18.fld3.3 = core::ptr::addr_of!(_18.fld3.0.0);
_18.fld7.1 = _19;
_18.fld3.3 = core::ptr::addr_of!(_25.0);
Goto(bb7)
}
bb7 = {
_26.1 = !_14;
_18.fld0.2 = [_18.fld7.0,_3,_18.fld7.0,_3,_22,_22];
_28 = _10 & _10;
_25 = _18.fld3.0;
_10 = !_28;
_24 = (_18.fld5,);
_26 = (_7, _2);
_18.fld1.0 = _15 << _26.0;
_18.fld5 = _14 as u128;
_2 = _12 as u32;
_24.0 = _28 as u128;
_26.1 = _14 / 4143882723_u32;
_4 = _18.fld3.2 as i8;
_25.0 = _3 as usize;
_18.fld3.1 = (_18.fld1.0,);
_19 = [_6,_6,_6,_6,_6];
_8 = !106267489007123802458561679137886817374_i128;
_29.0 = !(*_16);
_9 = _25.0;
_25.0 = _18.fld3.0.0;
(*_16) = !_18.fld5;
_31.fld6.3 = core::ptr::addr_of_mut!(_2);
_31.fld3 = (_25, _18.fld6.0, _1, _18.fld3.3);
_32 = -_3;
match _25.0 {
0 => bb8,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
1 => bb16,
_ => bb15
}
}
bb8 = {
_1 = !_20;
_18.fld6.0 = (_15,);
_4 = 88_i8;
_16 = core::ptr::addr_of!(_24.0);
_1 = !_20;
_22 = _18.fld5 as isize;
_18.fld7.1 = [_6,_6,_6,_6,_6];
_22 = _18.fld3.2 as isize;
_19 = _18.fld7.1;
_21.fld0 = [_4,_4,_4,_4];
_1 = !_18.fld3.2;
_15 = !_18.fld3.1.0;
_7 = 8602062252548809136_i64 << _2;
_18.fld5 = 290570187033953975960448278093656680127_u128;
_18.fld6.0 = _18.fld3.1;
_18.fld1 = (_15,);
_18.fld7 = (_3, _19);
_18.fld7.0 = !_3;
_18.fld3.1 = (_15,);
(*_16) = '\u{c6a81}' as u128;
_11 = !64183_u16;
_18.fld7.1 = [_6,_6,_6,_6,_6];
_1 = _24.0 < _18.fld5;
_18.fld1.0 = _8 as u64;
_18.fld0.2 = [_3,_3,_22,_3,_3,_3];
_18.fld3.3 = core::ptr::addr_of!(_18.fld3.0.0);
_18.fld7.1 = _19;
_18.fld3.3 = core::ptr::addr_of!(_25.0);
Goto(bb7)
}
bb9 = {
_18.fld7.1[_9] = _6;
_18.fld7.1[_9] = -_6;
_20 = _9 != _18.fld0.0.1.0;
_18.fld3.0 = (_9,);
_7 = _18.fld3.0.0 as i64;
_18.fld3.1 = (_15,);
_7 = -4149520041408350389_i64;
_11 = _3 as u16;
_18.fld7.0 = _18.fld0.2[_9];
_18.fld3.1.0 = _15 >> _10;
RET = [_10];
_18.fld0.1 = core::ptr::addr_of_mut!(_12);
Call(_18.fld3.2 = fn1(_12, _3, _18.fld0.2, _6, _18.fld3.1, _18.fld0.0.1.0, _18.fld3.1), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
RET = [_10];
_1 = !_13;
_18.fld0.0.1 = (_9,);
_18.fld0.1 = core::ptr::addr_of_mut!(_12);
_18.fld3.3 = core::ptr::addr_of!(_9);
_7 = !6476967726780500781_i64;
_18.fld5 = 220630792861135108503728504855708302925_u128;
_18.fld3.0.0 = !_9;
_18.fld0.0.1 = (_18.fld3.0.0,);
_13 = _1 ^ _1;
_18.fld0.2 = [_3,_3,_3,_3,_3,_3];
_7 = _12 as i64;
_21.fld0[_9] = -_4;
_18.fld3.3 = core::ptr::addr_of!(_9);
_5 = _6 as i16;
_18.fld3.1.0 = _15;
Goto(bb5)
}
bb11 = {
_1 = !false;
_2 = 2029441424_u32;
_13 = !_1;
_8 = _5 as i128;
_11 = _13 as u16;
_3 = 37_isize & (-108_isize);
_4 = _10 as i8;
_12 = _11 as f64;
_8 = (-60718315368066716828284178750588868897_i128) * 136881241746909649171326715922541626458_i128;
_13 = !_1;
_12 = _4 as f64;
_9 = 11729073620023706774_usize;
_4 = -(-18_i8);
_13 = !_1;
_11 = _5 as u16;
_13 = _1;
_8 = 34010793856911330369750289765765762620_i128;
_1 = _13;
_13 = _1;
_9 = 1_usize;
_15 = 10128560916999392973_u64 << _8;
_14 = _2 + _2;
_6 = (-1283150635_i32) >> _3;
_4 = !13_i8;
_4 = _5 as i8;
_1 = !_13;
Goto(bb4)
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
_18.fld3.1.0 = !_15;
_31.fld3 = (_18.fld0.0.1, _18.fld3.1, _1, _18.fld3.3);
_18.fld0.0.0 = _4 | _4;
_18.fld3.3 = _31.fld3.3;
_18.fld0.0.0 = _8 as i8;
_31.fld3 = _18.fld3;
_31.fld6.2.0 = _18.fld0.0.1.0 & _9;
_31.fld5.1 = [_6,_6,_6,_6,_6];
_16 = core::ptr::addr_of!(_24.0);
_31.fld5 = _18.fld7;
_18.fld6.0.0 = !_31.fld3.1.0;
_31.fld3.3 = _18.fld3.3;
_31.fld6.3 = core::ptr::addr_of_mut!(_26.1);
_17 = core::ptr::addr_of!(_31.fld3.3);
_36 = [_18.fld0.0.0,_18.fld0.0.0,_18.fld0.0.0,_4];
_18.fld3.0 = _31.fld3.0;
_18.fld1.0 = _11 as u64;
_31.fld5 = (_3, _18.fld7.1);
_18.fld0.1 = core::ptr::addr_of_mut!(_12);
(*_16) = _18.fld5;
_31.fld1 = (_18.fld3.1.0,);
_31.fld2 = [_31.fld3.1.0,_31.fld1.0,_15,_18.fld1.0];
_6 = (-20070837_i32) | (-1579736483_i32);
_24 = (_29.0,);
_18.fld3.2 = _31.fld1.0 == _18.fld3.1.0;
_34 = _8 << _24.0;
_11 = !21480_u16;
_22 = _18.fld6.0.0 as isize;
_24 = _29;
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(0_usize, 11_usize, Move(_11), 26_usize, Move(_26), 19_usize, Move(_19), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(0_usize, 7_usize, Move(_7), 5_usize, Move(_5), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_37 = dump_var(0_usize, 20_usize, Move(_20), 3_usize, Move(_3), 8_usize, Move(_8), 22_usize, Move(_22)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: f64,mut _2: isize,mut _3: [isize; 6],mut _4: i32,mut _5: (u64,),mut _6: usize,mut _7: (u64,)) -> bool {
mir! {
type RET = bool;
let _8: *mut f32;
let _9: [isize; 6];
let _10: [i32; 5];
let _11: (i64, u32);
let _12: (usize,);
let _13: [u64; 4];
let _14: i16;
let _15: (u64,);
let _16: i32;
let _17: ((u64,), f64);
let _18: i128;
let _19: (i32, *const [u8; 1]);
let _20: u64;
let _21: *const usize;
let _22: isize;
let _23: i32;
let _24: bool;
let _25: i128;
let _26: f64;
let _27: isize;
let _28: *mut f32;
let _29: bool;
let _30: Adt55;
let _31: ();
let _32: ();
{
RET = true;
_1 = 21470_i16 as f64;
_5.0 = _7.0 ^ _7.0;
RET = true & false;
_7.0 = !_5.0;
_7.0 = _5.0;
_9 = [_2,_2,_2,_2,_2,_2];
RET = !false;
Call(_8 = fn2(_2, _9, _7, _5.0, _3, _2, _5.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _5;
RET = false ^ false;
RET = _2 < _2;
_5 = (_7.0,);
RET = false;
RET = !false;
_11 = ((-3272195627843303644_i64), 1041118418_u32);
_1 = _6 as f64;
_9 = [_2,_2,_2,_2,_2,_2];
_6 = 7_usize;
_9 = [_2,_2,_2,_2,_2,_2];
_6 = 4821854267893311103_usize / 1_usize;
_12 = (_6,);
_12 = (_6,);
_3 = [_2,_2,_2,_2,_2,_2];
_10 = [_4,_4,_4,_4,_4];
_12 = (_6,);
_3 = [_2,_2,_2,_2,_2,_2];
_12 = (_6,);
Goto(bb2)
}
bb2 = {
_13 = [_5.0,_7.0,_7.0,_7.0];
RET = _5.0 >= _5.0;
_3 = _9;
RET = !false;
_7.0 = !_5.0;
_7.0 = '\u{f768c}' as u64;
_12.0 = _6 << _11.1;
_7 = _5;
_15.0 = _12.0 as u64;
RET = true;
RET = !false;
_10 = [_4,_4,_4,_4,_4];
_14 = (-61_i8) as i16;
_6 = _1 as usize;
_11 = (4497466534028578133_i64, 3292483075_u32);
_2 = '\u{96e9d}' as isize;
_5.0 = !_15.0;
Goto(bb3)
}
bb3 = {
_3 = [_2,_2,_2,_2,_2,_2];
_15.0 = _2 as u64;
_7 = (_5.0,);
_17.0.0 = _1 as u64;
RET = true & false;
_20 = '\u{3e2d2}' as u64;
RET = true;
_20 = _7.0 >> _11.1;
_18 = 82530674331359139647014072604019928807_i128 * 64674858476086881502210609213917499966_i128;
_11.0 = -2716272217474893516_i64;
_7.0 = false as u64;
_12 = (_6,);
_17 = (_5, _1);
_17.0 = (_7.0,);
_21 = core::ptr::addr_of!(_12.0);
RET = _15.0 >= _5.0;
_5 = (_7.0,);
_17 = (_5, _1);
_19.0 = '\u{f47f3}' as i32;
_21 = core::ptr::addr_of!((*_21));
_5.0 = _20 & _20;
_13 = [_20,_5.0,_5.0,_5.0];
_14 = 20148_i16;
_17.1 = _1;
_7.0 = _5.0;
match _11.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3292483075 => bb6,
_ => bb5
}
}
bb4 = {
_13 = [_5.0,_7.0,_7.0,_7.0];
RET = _5.0 >= _5.0;
_3 = _9;
RET = !false;
_7.0 = !_5.0;
_7.0 = '\u{f768c}' as u64;
_12.0 = _6 << _11.1;
_7 = _5;
_15.0 = _12.0 as u64;
RET = true;
RET = !false;
_10 = [_4,_4,_4,_4,_4];
_14 = (-61_i8) as i16;
_6 = _1 as usize;
_11 = (4497466534028578133_i64, 3292483075_u32);
_2 = '\u{96e9d}' as isize;
_5.0 = !_15.0;
Goto(bb3)
}
bb5 = {
_7 = _5;
RET = false ^ false;
RET = _2 < _2;
_5 = (_7.0,);
RET = false;
RET = !false;
_11 = ((-3272195627843303644_i64), 1041118418_u32);
_1 = _6 as f64;
_9 = [_2,_2,_2,_2,_2,_2];
_6 = 7_usize;
_9 = [_2,_2,_2,_2,_2,_2];
_6 = 4821854267893311103_usize / 1_usize;
_12 = (_6,);
_12 = (_6,);
_3 = [_2,_2,_2,_2,_2,_2];
_10 = [_4,_4,_4,_4,_4];
_12 = (_6,);
_3 = [_2,_2,_2,_2,_2,_2];
_12 = (_6,);
Goto(bb2)
}
bb6 = {
_12 = (_6,);
RET = false;
_15 = (_5.0,);
_15 = (_7.0,);
_17.0.0 = _15.0;
Goto(bb7)
}
bb7 = {
_9 = _3;
_20 = _5.0 + _7.0;
_23 = _4 >> _17.0.0;
RET = false;
_4 = _23 - _23;
RET = true | true;
_15.0 = _23 as u64;
_21 = core::ptr::addr_of!(_12.0);
RET = !false;
_3 = [_2,_2,_2,_2,_2,_2];
_5.0 = _15.0;
_5 = (_20,);
_18 = !(-112381684171384310321618583608373385984_i128);
_21 = core::ptr::addr_of!((*_21));
RET = !true;
_17 = (_15, _1);
_25 = !_18;
_15.0 = !_5.0;
_2 = _12.0 as isize;
_15 = _7;
_25 = _18;
match _11.1 {
0 => bb3,
1 => bb8,
2 => bb9,
3 => bb10,
3292483075 => bb12,
_ => bb11
}
}
bb8 = {
_12 = (_6,);
RET = false;
_15 = (_5.0,);
_15 = (_7.0,);
_17.0.0 = _15.0;
Goto(bb7)
}
bb9 = {
_7 = _5;
RET = false ^ false;
RET = _2 < _2;
_5 = (_7.0,);
RET = false;
RET = !false;
_11 = ((-3272195627843303644_i64), 1041118418_u32);
_1 = _6 as f64;
_9 = [_2,_2,_2,_2,_2,_2];
_6 = 7_usize;
_9 = [_2,_2,_2,_2,_2,_2];
_6 = 4821854267893311103_usize / 1_usize;
_12 = (_6,);
_12 = (_6,);
_3 = [_2,_2,_2,_2,_2,_2];
_10 = [_4,_4,_4,_4,_4];
_12 = (_6,);
_3 = [_2,_2,_2,_2,_2,_2];
_12 = (_6,);
Goto(bb2)
}
bb10 = {
_7 = _5;
RET = false ^ false;
RET = _2 < _2;
_5 = (_7.0,);
RET = false;
RET = !false;
_11 = ((-3272195627843303644_i64), 1041118418_u32);
_1 = _6 as f64;
_9 = [_2,_2,_2,_2,_2,_2];
_6 = 7_usize;
_9 = [_2,_2,_2,_2,_2,_2];
_6 = 4821854267893311103_usize / 1_usize;
_12 = (_6,);
_12 = (_6,);
_3 = [_2,_2,_2,_2,_2,_2];
_10 = [_4,_4,_4,_4,_4];
_12 = (_6,);
_3 = [_2,_2,_2,_2,_2,_2];
_12 = (_6,);
Goto(bb2)
}
bb11 = {
_3 = [_2,_2,_2,_2,_2,_2];
_15.0 = _2 as u64;
_7 = (_5.0,);
_17.0.0 = _1 as u64;
RET = true & false;
_20 = '\u{3e2d2}' as u64;
RET = true;
_20 = _7.0 >> _11.1;
_18 = 82530674331359139647014072604019928807_i128 * 64674858476086881502210609213917499966_i128;
_11.0 = -2716272217474893516_i64;
_7.0 = false as u64;
_12 = (_6,);
_17 = (_5, _1);
_17.0 = (_7.0,);
_21 = core::ptr::addr_of!(_12.0);
RET = _15.0 >= _5.0;
_5 = (_7.0,);
_17 = (_5, _1);
_19.0 = '\u{f47f3}' as i32;
_21 = core::ptr::addr_of!((*_21));
_5.0 = _20 & _20;
_13 = [_20,_5.0,_5.0,_5.0];
_14 = 20148_i16;
_17.1 = _1;
_7.0 = _5.0;
match _11.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3292483075 => bb6,
_ => bb5
}
}
bb12 = {
_7.0 = _17.0.0;
_26 = _1;
_12 = (_6,);
_14 = 18376_i16;
_24 = false ^ true;
_28 = _8;
_17.1 = 194_u8 as f64;
_17.0 = _15;
_24 = _5.0 < _5.0;
_12.0 = _6;
_5.0 = _17.0.0 + _7.0;
_24 = !true;
_15 = (_20,);
_10 = [_4,_4,_23,_4,_4];
_17.1 = _26 + _26;
_3 = [_2,_2,_2,_2,_2,_2];
_18 = '\u{63cee}' as i128;
(*_21) = 24_u8 as usize;
_7 = (_20,);
_11.1 = 7322797_u32;
_11.1 = !2552842334_u32;
Goto(bb13)
}
bb13 = {
_6 = '\u{a8b15}' as usize;
_7.0 = _20;
_21 = core::ptr::addr_of!(_12.0);
_18 = -_25;
_17 = (_5, _1);
_3 = [_2,_2,_2,_2,_2,_2];
_21 = core::ptr::addr_of!((*_21));
_17.0.0 = !_5.0;
_17.0 = _15;
_23 = _11.1 as i32;
_2 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
Goto(bb14)
}
bb14 = {
_11.0 = '\u{9c05f}' as i64;
_14 = 49260_u16 as i16;
_29 = _24;
_17 = (_15, _26);
_17.0 = (_7.0,);
_10 = [_19.0,_4,_4,_4,_4];
_17.0.0 = _7.0 % 4191533965067388490_u64;
_18 = _25;
_1 = _26 + _17.1;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(1_usize, 5_usize, Move(_5), 23_usize, Move(_23), 29_usize, Move(_29), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(1_usize, 13_usize, Move(_13), 14_usize, Move(_14), 24_usize, Move(_24), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(1_usize, 15_usize, Move(_15), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: [isize; 6],mut _3: (u64,),mut _4: u64,mut _5: [isize; 6],mut _6: isize,mut _7: u64) -> *mut f32 {
mir! {
type RET = *mut f32;
let _8: Adt59;
let _9: (isize, [i32; 5]);
let _10: Adt63;
let _11: Adt62;
let _12: char;
let _13: (i64, u32);
let _14: ((usize,), (u64,), bool, *const usize);
let _15: [isize; 6];
let _16: Adt61;
let _17: [isize; 6];
let _18: isize;
let _19: *const f32;
let _20: Adt55;
let _21: [i8; 4];
let _22: *const usize;
let _23: (isize, [i32; 5]);
let _24: isize;
let _25: f64;
let _26: f64;
let _27: isize;
let _28: Adt56;
let _29: f32;
let _30: *const *const usize;
let _31: (isize, [i32; 5]);
let _32: [u8; 1];
let _33: i128;
let _34: char;
let _35: *mut u32;
let _36: *mut u32;
let _37: [u64; 4];
let _38: Adt51;
let _39: [i8; 4];
let _40: f64;
let _41: char;
let _42: Adt49;
let _43: isize;
let _44: Adt51;
let _45: ();
let _46: ();
{
_5 = [_1,_6,_1,_1,_1,_1];
Goto(bb1)
}
bb1 = {
_3.0 = !_4;
_6 = 2_usize as isize;
_3.0 = _4;
_6 = _1 + _1;
_2 = _5;
_3.0 = _4 ^ _4;
_3.0 = !_4;
_3.0 = _4;
_5 = [_1,_6,_1,_6,_6,_1];
_5 = [_1,_6,_6,_6,_6,_6];
_3.0 = !_7;
Call(_8.fld7.fld0.2.0 = fn3(_6, _2, _5, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = core::ptr::addr_of_mut!(_8.fld7.fld0.1);
_8.fld7.fld1.0 = !21345936185796803580105699168169188908_u128;
_3 = (_4,);
_8.fld7.fld1.0 = 114571525181520128155614629110581298544_u128;
_3 = (_4,);
_8.fld5 = (-598246154_i32);
(*RET) = 1027725522_u32 as f32;
_8.fld7.fld1 = (52826574218964742260643502436141712959_u128,);
_4 = _3.0;
_9.1 = [_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5];
_5 = [_6,_6,_6,_6,_6,_6];
(*RET) = _8.fld7.fld0.2.0 as f32;
_8.fld2 = (-65821570296233567963405842021635292977_i128) as isize;
_6 = _1 - _1;
_7 = _3.0 % 13695673636962002675_u64;
(*RET) = (-2885619810686897761_i64) as f32;
_8.fld7.fld0.0 = 117_u8 / 254_u8;
_9.0 = _8.fld7.fld0.0 as isize;
_10.fld0.fld0.0 = _8.fld7.fld0.0;
_10.fld0.fld0.1 = _8.fld7.fld0.1 + _8.fld7.fld0.1;
_3.0 = _4;
_10.fld0.fld0.2 = (_8.fld7.fld0.2.0,);
_3.0 = !_4;
_11.fld2.fld4.fld3.0.0 = _8.fld7.fld0.2.0;
_11.fld2.fld1.fld2 = 128143010029005549359585079511790051149_i128;
Goto(bb3)
}
bb3 = {
_11.fld1.fld0 = true;
_11.fld2.fld4.fld1.0 = !_3.0;
_11.fld1.fld5 = _5;
_11.fld2.fld3.fld2 = _5;
_11.fld2.fld3.fld1.2.0 = 896269554144599118_i64 as usize;
_8.fld7.fld0.2.0 = !_10.fld0.fld0.2.0;
Call(_11.fld2.fld4.fld0.0.0 = core::intrinsics::bswap((-43_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11.fld2.fld4.fld7 = _9;
_11.fld4 = core::ptr::addr_of_mut!(_11.fld1.fld4);
_14.1 = (_7,);
_14.3 = core::ptr::addr_of!(_14.0.0);
_11.fld1.fld2 = _1;
_11.fld1.fld0 = !true;
_10.fld0.fld0.3 = core::ptr::addr_of_mut!(_13.1);
_8.fld2 = !_9.0;
_3.0 = _14.1.0;
_16.fld2.fld1.fld3.3 = core::ptr::addr_of!(_8.fld7.fld0.2.0);
(*RET) = -_10.fld0.fld0.1;
(*RET) = _10.fld0.fld0.1 + _10.fld0.fld0.1;
_11.fld3 = core::ptr::addr_of_mut!(_8.fld7.fld0.1);
_11.fld2.fld2 = [_11.fld2.fld1.fld2];
_11.fld1.fld3 = core::ptr::addr_of_mut!(_11.fld1.fld1);
Goto(bb5)
}
bb5 = {
_11.fld2.fld4.fld3.1 = (_4,);
_11.fld2.fld1 = Adt49 { fld0: _5,fld1: _11.fld2.fld2,fld2: (-80515138753263401639160313716966477551_i128) };
_16.fld2.fld0 = core::ptr::addr_of!(_16.fld2.fld1.fld3.3);
_14.1.0 = _7;
_16.fld2.fld3 = core::ptr::addr_of!(_11.fld1.fld1.1.0);
_20.fld3.3 = core::ptr::addr_of!(_11.fld2.fld4.fld3.0.0);
_20.fld7.0 = '\u{3900d}' as isize;
_20.fld7 = (_1, _9.1);
_16.fld2.fld1.fld6.0 = !_8.fld7.fld0.0;
_10.fld1 = [_6,_11.fld2.fld4.fld7.0,_11.fld1.fld2,_8.fld2,_20.fld7.0,_6];
_20.fld3.0.0 = _11.fld2.fld1.fld2 as usize;
_10.fld0.fld1.0 = !_8.fld7.fld1.0;
_17 = [_11.fld1.fld2,_1,_20.fld7.0,_8.fld2,_6,_20.fld7.0];
_16.fld2.fld1.fld3.1.0 = (-42_i8) as u64;
_20.fld0.2 = [_8.fld2,_1,_8.fld2,_20.fld7.0,_1,_11.fld1.fld2];
_11.fld1.fld6 = Adt49 { fld0: _17,fld1: _11.fld2.fld2,fld2: _11.fld2.fld1.fld2 };
Goto(bb6)
}
bb6 = {
_16.fld2.fld1.fld6 = _10.fld0.fld0;
_11.fld2.fld4.fld6.1 = _11.fld3;
_20.fld6.1 = core::ptr::addr_of_mut!(_8.fld7.fld0.1);
_8.fld7 = Adt52 { fld0: _16.fld2.fld1.fld6,fld1: _10.fld0.fld1 };
_8.fld1 = '\u{e6a25}';
_11.fld2.fld3.fld1.3 = core::ptr::addr_of_mut!(_13.1);
_8.fld7.fld0.3 = core::ptr::addr_of_mut!(_16.fld2.fld2.1);
_11.fld1.fld2 = (-117_i8) as isize;
_13.0 = (-4135487101286671016_i64) << _16.fld2.fld1.fld6.0;
_28.fld4.fld0.2 = [_20.fld7.0,_6,_20.fld7.0,_6,_11.fld1.fld2,_1];
_16.fld2.fld1.fld6.2 = (_8.fld7.fld0.2.0,);
_20.fld3.2 = _11.fld1.fld0 ^ _11.fld1.fld0;
_28.fld3.fld1.3 = core::ptr::addr_of_mut!(_16.fld2.fld2.1);
_28.fld4.fld2 = [_11.fld2.fld1.fld2];
_8.fld2 = !_9.0;
_16.fld2.fld1.fld5 = (_8.fld2, _20.fld7.1);
_15 = _10.fld1;
_28.fld3.fld1.2 = (_20.fld3.0.0,);
_16.fld2.fld1.fld0 = !_20.fld3.2;
_8.fld6 = (_10.fld0.fld1.0,);
_14.0 = _20.fld3.0;
_20.fld0.0.0 = 41_i8 >> _16.fld2.fld1.fld5.0;
_28.fld4.fld0.2 = _5;
match _11.fld1.fld6.fld2 {
0 => bb1,
1 => bb7,
2 => bb8,
259767228167675061824214293714801733905 => bb10,
_ => bb9
}
}
bb7 = {
_11.fld2.fld4.fld3.1 = (_4,);
_11.fld2.fld1 = Adt49 { fld0: _5,fld1: _11.fld2.fld2,fld2: (-80515138753263401639160313716966477551_i128) };
_16.fld2.fld0 = core::ptr::addr_of!(_16.fld2.fld1.fld3.3);
_14.1.0 = _7;
_16.fld2.fld3 = core::ptr::addr_of!(_11.fld1.fld1.1.0);
_20.fld3.3 = core::ptr::addr_of!(_11.fld2.fld4.fld3.0.0);
_20.fld7.0 = '\u{3900d}' as isize;
_20.fld7 = (_1, _9.1);
_16.fld2.fld1.fld6.0 = !_8.fld7.fld0.0;
_10.fld1 = [_6,_11.fld2.fld4.fld7.0,_11.fld1.fld2,_8.fld2,_20.fld7.0,_6];
_20.fld3.0.0 = _11.fld2.fld1.fld2 as usize;
_10.fld0.fld1.0 = !_8.fld7.fld1.0;
_17 = [_11.fld1.fld2,_1,_20.fld7.0,_8.fld2,_6,_20.fld7.0];
_16.fld2.fld1.fld3.1.0 = (-42_i8) as u64;
_20.fld0.2 = [_8.fld2,_1,_8.fld2,_20.fld7.0,_1,_11.fld1.fld2];
_11.fld1.fld6 = Adt49 { fld0: _17,fld1: _11.fld2.fld2,fld2: _11.fld2.fld1.fld2 };
Goto(bb6)
}
bb8 = {
_11.fld2.fld4.fld7 = _9;
_11.fld4 = core::ptr::addr_of_mut!(_11.fld1.fld4);
_14.1 = (_7,);
_14.3 = core::ptr::addr_of!(_14.0.0);
_11.fld1.fld2 = _1;
_11.fld1.fld0 = !true;
_10.fld0.fld0.3 = core::ptr::addr_of_mut!(_13.1);
_8.fld2 = !_9.0;
_3.0 = _14.1.0;
_16.fld2.fld1.fld3.3 = core::ptr::addr_of!(_8.fld7.fld0.2.0);
(*RET) = -_10.fld0.fld0.1;
(*RET) = _10.fld0.fld0.1 + _10.fld0.fld0.1;
_11.fld3 = core::ptr::addr_of_mut!(_8.fld7.fld0.1);
_11.fld2.fld2 = [_11.fld2.fld1.fld2];
_11.fld1.fld3 = core::ptr::addr_of_mut!(_11.fld1.fld1);
Goto(bb5)
}
bb9 = {
_3.0 = !_4;
_6 = 2_usize as isize;
_3.0 = _4;
_6 = _1 + _1;
_2 = _5;
_3.0 = _4 ^ _4;
_3.0 = !_4;
_3.0 = _4;
_5 = [_1,_6,_1,_6,_6,_1];
_5 = [_1,_6,_6,_6,_6,_6];
_3.0 = !_7;
Call(_8.fld7.fld0.2.0 = fn3(_6, _2, _5, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_11.fld2.fld4.fld6 = (_14.1, _11.fld3);
_16.fld2.fld2 = (_13.0, 3300985123_u32);
_16.fld2.fld7 = (_16.fld2.fld1.fld6.0, (*RET), _28.fld3.fld1.2, _8.fld7.fld0.3);
_28.fld4.fld3.0.0 = _28.fld3.fld1.2.0 >> _11.fld2.fld3.fld1.2.0;
_28.fld4.fld6 = (_11.fld2.fld4.fld1, _11.fld3);
_19 = core::ptr::addr_of!(_10.fld0.fld0.1);
_10.fld0.fld1.0 = _8.fld7.fld0.0 as u128;
_11.fld1.fld1.1.0 = _16.fld2.fld1.fld6.2.0;
_28.fld4.fld7.1 = _16.fld2.fld1.fld5.1;
_11.fld2.fld2 = _28.fld4.fld2;
_16.fld2.fld1.fld0 = !_11.fld1.fld0;
_11.fld1.fld7 = core::ptr::addr_of_mut!(_26);
Call(_28.fld4 = fn5(_8.fld7.fld1, _11.fld1.fld0, _11.fld2.fld4.fld6, _2, _16.fld2.fld1.fld6.3, _16.fld2.fld1.fld3.3, _11.fld2.fld4.fld3.1.0, (*_19), _17, _11.fld1.fld5, _8.fld7.fld0, _11.fld2.fld4.fld6.1, _20.fld7.1, _16.fld2.fld7.0, _10.fld0.fld0.2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11.fld2.fld4.fld3.2 = _28.fld4.fld3.2 & _28.fld4.fld3.2;
_16.fld2.fld7.3 = _10.fld0.fld0.3;
_26 = (-24294_i16) as f64;
_16.fld2.fld4.fld1.2.0 = _8.fld7.fld0.2.0 / 1_usize;
_20.fld0.0 = _28.fld4.fld0.0;
_10.fld0.fld1.0 = !_8.fld7.fld1.0;
_16.fld2.fld7 = _8.fld7.fld0;
_8.fld2 = _6;
(*_19) = _16.fld2.fld1.fld6.1 * _8.fld7.fld0.1;
_28.fld4.fld1.0 = _11.fld2.fld4.fld3.1.0;
_11.fld2.fld4.fld0.0 = (_20.fld0.0.0, _16.fld2.fld4.fld1.2, _28.fld4.fld6.1);
_11.fld2.fld4.fld6 = (_11.fld2.fld4.fld3.1, _20.fld0.0.2);
match _11.fld1.fld6.fld2 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
259767228167675061824214293714801733905 => bb17,
_ => bb16
}
}
bb12 = {
_11.fld2.fld4.fld6 = (_14.1, _11.fld3);
_16.fld2.fld2 = (_13.0, 3300985123_u32);
_16.fld2.fld7 = (_16.fld2.fld1.fld6.0, (*RET), _28.fld3.fld1.2, _8.fld7.fld0.3);
_28.fld4.fld3.0.0 = _28.fld3.fld1.2.0 >> _11.fld2.fld3.fld1.2.0;
_28.fld4.fld6 = (_11.fld2.fld4.fld1, _11.fld3);
_19 = core::ptr::addr_of!(_10.fld0.fld0.1);
_10.fld0.fld1.0 = _8.fld7.fld0.0 as u128;
_11.fld1.fld1.1.0 = _16.fld2.fld1.fld6.2.0;
_28.fld4.fld7.1 = _16.fld2.fld1.fld5.1;
_11.fld2.fld2 = _28.fld4.fld2;
_16.fld2.fld1.fld0 = !_11.fld1.fld0;
_11.fld1.fld7 = core::ptr::addr_of_mut!(_26);
Call(_28.fld4 = fn5(_8.fld7.fld1, _11.fld1.fld0, _11.fld2.fld4.fld6, _2, _16.fld2.fld1.fld6.3, _16.fld2.fld1.fld3.3, _11.fld2.fld4.fld3.1.0, (*_19), _17, _11.fld1.fld5, _8.fld7.fld0, _11.fld2.fld4.fld6.1, _20.fld7.1, _16.fld2.fld7.0, _10.fld0.fld0.2), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_11.fld2.fld4.fld7 = _9;
_11.fld4 = core::ptr::addr_of_mut!(_11.fld1.fld4);
_14.1 = (_7,);
_14.3 = core::ptr::addr_of!(_14.0.0);
_11.fld1.fld2 = _1;
_11.fld1.fld0 = !true;
_10.fld0.fld0.3 = core::ptr::addr_of_mut!(_13.1);
_8.fld2 = !_9.0;
_3.0 = _14.1.0;
_16.fld2.fld1.fld3.3 = core::ptr::addr_of!(_8.fld7.fld0.2.0);
(*RET) = -_10.fld0.fld0.1;
(*RET) = _10.fld0.fld0.1 + _10.fld0.fld0.1;
_11.fld3 = core::ptr::addr_of_mut!(_8.fld7.fld0.1);
_11.fld2.fld2 = [_11.fld2.fld1.fld2];
_11.fld1.fld3 = core::ptr::addr_of_mut!(_11.fld1.fld1);
Goto(bb5)
}
bb14 = {
_11.fld2.fld4.fld7 = _9;
_11.fld4 = core::ptr::addr_of_mut!(_11.fld1.fld4);
_14.1 = (_7,);
_14.3 = core::ptr::addr_of!(_14.0.0);
_11.fld1.fld2 = _1;
_11.fld1.fld0 = !true;
_10.fld0.fld0.3 = core::ptr::addr_of_mut!(_13.1);
_8.fld2 = !_9.0;
_3.0 = _14.1.0;
_16.fld2.fld1.fld3.3 = core::ptr::addr_of!(_8.fld7.fld0.2.0);
(*RET) = -_10.fld0.fld0.1;
(*RET) = _10.fld0.fld0.1 + _10.fld0.fld0.1;
_11.fld3 = core::ptr::addr_of_mut!(_8.fld7.fld0.1);
_11.fld2.fld2 = [_11.fld2.fld1.fld2];
_11.fld1.fld3 = core::ptr::addr_of_mut!(_11.fld1.fld1);
Goto(bb5)
}
bb15 = {
_11.fld2.fld4.fld3.1 = (_4,);
_11.fld2.fld1 = Adt49 { fld0: _5,fld1: _11.fld2.fld2,fld2: (-80515138753263401639160313716966477551_i128) };
_16.fld2.fld0 = core::ptr::addr_of!(_16.fld2.fld1.fld3.3);
_14.1.0 = _7;
_16.fld2.fld3 = core::ptr::addr_of!(_11.fld1.fld1.1.0);
_20.fld3.3 = core::ptr::addr_of!(_11.fld2.fld4.fld3.0.0);
_20.fld7.0 = '\u{3900d}' as isize;
_20.fld7 = (_1, _9.1);
_16.fld2.fld1.fld6.0 = !_8.fld7.fld0.0;
_10.fld1 = [_6,_11.fld2.fld4.fld7.0,_11.fld1.fld2,_8.fld2,_20.fld7.0,_6];
_20.fld3.0.0 = _11.fld2.fld1.fld2 as usize;
_10.fld0.fld1.0 = !_8.fld7.fld1.0;
_17 = [_11.fld1.fld2,_1,_20.fld7.0,_8.fld2,_6,_20.fld7.0];
_16.fld2.fld1.fld3.1.0 = (-42_i8) as u64;
_20.fld0.2 = [_8.fld2,_1,_8.fld2,_20.fld7.0,_1,_11.fld1.fld2];
_11.fld1.fld6 = Adt49 { fld0: _17,fld1: _11.fld2.fld2,fld2: _11.fld2.fld1.fld2 };
Goto(bb6)
}
bb16 = {
_11.fld2.fld4.fld3.1 = (_4,);
_11.fld2.fld1 = Adt49 { fld0: _5,fld1: _11.fld2.fld2,fld2: (-80515138753263401639160313716966477551_i128) };
_16.fld2.fld0 = core::ptr::addr_of!(_16.fld2.fld1.fld3.3);
_14.1.0 = _7;
_16.fld2.fld3 = core::ptr::addr_of!(_11.fld1.fld1.1.0);
_20.fld3.3 = core::ptr::addr_of!(_11.fld2.fld4.fld3.0.0);
_20.fld7.0 = '\u{3900d}' as isize;
_20.fld7 = (_1, _9.1);
_16.fld2.fld1.fld6.0 = !_8.fld7.fld0.0;
_10.fld1 = [_6,_11.fld2.fld4.fld7.0,_11.fld1.fld2,_8.fld2,_20.fld7.0,_6];
_20.fld3.0.0 = _11.fld2.fld1.fld2 as usize;
_10.fld0.fld1.0 = !_8.fld7.fld1.0;
_17 = [_11.fld1.fld2,_1,_20.fld7.0,_8.fld2,_6,_20.fld7.0];
_16.fld2.fld1.fld3.1.0 = (-42_i8) as u64;
_20.fld0.2 = [_8.fld2,_1,_8.fld2,_20.fld7.0,_1,_11.fld1.fld2];
_11.fld1.fld6 = Adt49 { fld0: _17,fld1: _11.fld2.fld2,fld2: _11.fld2.fld1.fld2 };
Goto(bb6)
}
bb17 = {
_16.fld2.fld1.fld2 = [_28.fld4.fld3.1.0,_11.fld2.fld4.fld3.1.0,_3.0,_14.1.0];
_28.fld3.fld1.0 = _28.fld4.fld5 as u8;
_38.fld0 = (-5947_i16) as i32;
_32 = [_8.fld7.fld0.0];
_19 = core::ptr::addr_of!(_16.fld2.fld7.1);
_20.fld3.1 = (_11.fld2.fld4.fld3.1.0,);
_28.fld4.fld5 = _16.fld2.fld4.fld1.2.0 as u128;
_16.fld2.fld1.fld3.1.0 = _28.fld4.fld6.0.0;
_8.fld7.fld0.2 = (_16.fld2.fld7.2.0,);
_10.fld0.fld0.2.0 = _11.fld2.fld4.fld0.0.1.0;
Goto(bb18)
}
bb18 = {
Call(_45 = dump_var(2_usize, 4_usize, Move(_4), 32_usize, Move(_32), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(2_usize, 6_usize, Move(_6), 13_usize, Move(_13), 46_usize, _46, 46_usize, _46), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: [isize; 6],mut _3: [isize; 6],mut _4: isize) -> usize {
mir! {
type RET = usize;
let _5: isize;
let _6: f64;
let _7: (i64, u32);
let _8: char;
let _9: usize;
let _10: (isize, [i32; 5]);
let _11: u16;
let _12: Adt49;
let _13: [u8; 1];
let _14: isize;
let _15: (isize, [i32; 5]);
let _16: (usize,);
let _17: bool;
let _18: char;
let _19: *const u128;
let _20: bool;
let _21: (isize, [i32; 5]);
let _22: (i32, *const [u8; 1]);
let _23: ((usize,), (u64,), bool, *const usize);
let _24: ((u64,), f64);
let _25: (u64,);
let _26: i32;
let _27: u16;
let _28: f64;
let _29: Adt56;
let _30: u128;
let _31: char;
let _32: Adt51;
let _33: ();
let _34: ();
{
_4 = _1;
RET = 13681555272917371769_usize * 7_usize;
_1 = _4;
RET = 6_usize + 3870974187365640695_usize;
_2 = [_1,_1,_1,_4,_4,_4];
_5 = !_4;
_2 = [_5,_4,_1,_5,_1,_1];
_5 = _4 >> _4;
RET = 11541580203375794944_usize;
RET = 16709751129095005863_usize;
_5 = !_1;
_2 = _3;
_1 = (-60685080079483499950026117168041367719_i128) as isize;
_1 = _4 ^ _5;
_1 = (-43_i8) as isize;
_1 = _5;
_3 = [_1,_1,_1,_4,_1,_5];
Call(_7.0 = fn4(_4, _1, _5, _3, _3, _3, _2, _2, _1, _2, _1, _5, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = ((-2300117029762046503_i64), 3342383694_u32);
_7 = (997015395515224231_i64, 1707353335_u32);
_4 = -_1;
RET = !9740527685598944854_usize;
_7.0 = 2175401436165755836_i64 & (-2141272657123809630_i64);
_1 = _4;
_3 = _2;
_2 = [_1,_4,_4,_4,_4,_4];
_5 = 14945_i16 as isize;
_4 = -_1;
_8 = '\u{66785}';
_9 = 12787532866227597913_usize;
_11 = !55800_u16;
_3 = [_1,_5,_4,_4,_1,_1];
_10.0 = _7.1 as isize;
Goto(bb2)
}
bb2 = {
_12.fld1 = [(-8868254585139716115997475640382879222_i128)];
_15.0 = !_4;
_16 = (_9,);
_15.0 = _1;
_7.0 = 5125328401185425060_i64;
_5 = _4;
RET = _11 as usize;
_3 = _2;
_1 = _10.0 ^ _5;
_3 = [_10.0,_1,_10.0,_4,_5,_4];
_16.0 = !_9;
_17 = true;
_7.1 = !4046544086_u32;
RET = !_16.0;
_14 = -_4;
_12.fld1 = [(-139874826019034541952401279190737215556_i128)];
_10.1 = [1727665453_i32,(-529195078_i32),2093260204_i32,1966122312_i32,622287898_i32];
_15.0 = _1;
_15 = (_1, _10.1);
_18 = _8;
match _9 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
12787532866227597913 => bb8,
_ => bb7
}
}
bb3 = {
_7 = ((-2300117029762046503_i64), 3342383694_u32);
_7 = (997015395515224231_i64, 1707353335_u32);
_4 = -_1;
RET = !9740527685598944854_usize;
_7.0 = 2175401436165755836_i64 & (-2141272657123809630_i64);
_1 = _4;
_3 = _2;
_2 = [_1,_4,_4,_4,_4,_4];
_5 = 14945_i16 as isize;
_4 = -_1;
_8 = '\u{66785}';
_9 = 12787532866227597913_usize;
_11 = !55800_u16;
_3 = [_1,_5,_4,_4,_1,_1];
_10.0 = _7.1 as isize;
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
_7.0 = !(-3947040143994911515_i64);
_7.1 = 3737075104_u32 - 1889905336_u32;
_22.1 = core::ptr::addr_of!(_13);
_10.0 = !_15.0;
_23.3 = core::ptr::addr_of!(_9);
_10.0 = _14 & _1;
_21 = (_15.0, _15.1);
_12.fld2 = -(-46265386952228345473288323492006125754_i128);
_24.1 = _7.1 as f64;
_13 = [209_u8];
_16 = (_9,);
_7 = ((-5089494297593820658_i64), 481891217_u32);
_7 = ((-1698481407062323983_i64), 2316780755_u32);
_23.1 = (16049806392457697061_u64,);
_12.fld1 = [_12.fld2];
_23.0 = (_16.0,);
_15.1 = [(-873931201_i32),1097289096_i32,(-1434615755_i32),69854748_i32,2116356442_i32];
_7 = ((-4139200205918626683_i64), 3173633919_u32);
_22.0 = _7.0 as i32;
_23.2 = _10.0 >= _10.0;
_18 = _8;
_6 = 112_i8 as f64;
match _16.0 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
12787532866227597913 => bb15,
_ => bb14
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_7 = ((-2300117029762046503_i64), 3342383694_u32);
_7 = (997015395515224231_i64, 1707353335_u32);
_4 = -_1;
RET = !9740527685598944854_usize;
_7.0 = 2175401436165755836_i64 & (-2141272657123809630_i64);
_1 = _4;
_3 = _2;
_2 = [_1,_4,_4,_4,_4,_4];
_5 = 14945_i16 as isize;
_4 = -_1;
_8 = '\u{66785}';
_9 = 12787532866227597913_usize;
_11 = !55800_u16;
_3 = [_1,_5,_4,_4,_1,_1];
_10.0 = _7.1 as isize;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_7 = ((-2300117029762046503_i64), 3342383694_u32);
_7 = (997015395515224231_i64, 1707353335_u32);
_4 = -_1;
RET = !9740527685598944854_usize;
_7.0 = 2175401436165755836_i64 & (-2141272657123809630_i64);
_1 = _4;
_3 = _2;
_2 = [_1,_4,_4,_4,_4,_4];
_5 = 14945_i16 as isize;
_4 = -_1;
_8 = '\u{66785}';
_9 = 12787532866227597913_usize;
_11 = !55800_u16;
_3 = [_1,_5,_4,_4,_1,_1];
_10.0 = _7.1 as isize;
Goto(bb2)
}
bb14 = {
_12.fld1 = [(-8868254585139716115997475640382879222_i128)];
_15.0 = !_4;
_16 = (_9,);
_15.0 = _1;
_7.0 = 5125328401185425060_i64;
_5 = _4;
RET = _11 as usize;
_3 = _2;
_1 = _10.0 ^ _5;
_3 = [_10.0,_1,_10.0,_4,_5,_4];
_16.0 = !_9;
_17 = true;
_7.1 = !4046544086_u32;
RET = !_16.0;
_14 = -_4;
_12.fld1 = [(-139874826019034541952401279190737215556_i128)];
_10.1 = [1727665453_i32,(-529195078_i32),2093260204_i32,1966122312_i32,622287898_i32];
_15.0 = _1;
_15 = (_1, _10.1);
_18 = _8;
match _9 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
12787532866227597913 => bb8,
_ => bb7
}
}
bb15 = {
_21.0 = _8 as isize;
_7 = (3052866209362245308_i64, 2639726921_u32);
_12.fld2 = (-89400687511724145050114918906459045131_i128);
_23.1 = (2326172768243804995_u64,);
_10.1 = [_22.0,_22.0,_22.0,_22.0,_22.0];
_21 = _10;
_29.fld4.fld6.0 = (_23.1.0,);
_29.fld4.fld7.1 = _10.1;
_29.fld3.fld1.1 = _24.1 as f32;
_22.1 = core::ptr::addr_of!(_13);
_23.1.0 = !_29.fld4.fld6.0.0;
_15 = (_10.0, _21.1);
_25.0 = _29.fld4.fld6.0.0;
_29.fld4.fld7 = _15;
RET = _7.0 as usize;
_7.1 = 2815289175_u32 % 2277233713_u32;
_7.1 = _11 as u32;
_29.fld3.fld1.2.0 = _16.0;
_29.fld4.fld6.0 = (_25.0,);
_26 = _22.0;
_7.0 = -7854834154445356300_i64;
_29.fld4.fld3.3 = core::ptr::addr_of!(_29.fld3.fld1.2.0);
_30 = 286786357632061482690267184885067363192_u128;
_29.fld4.fld0.0.2 = core::ptr::addr_of_mut!(_29.fld3.fld1.1);
_29.fld3.fld1.3 = core::ptr::addr_of_mut!(_7.1);
_29.fld4.fld6 = (_23.1, _29.fld4.fld0.0.2);
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(3_usize, 18_usize, Move(_18), 5_usize, Move(_5), 30_usize, Move(_30), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(3_usize, 1_usize, Move(_1), 14_usize, Move(_14), 8_usize, Move(_8), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(3_usize, 3_usize, Move(_3), 21_usize, Move(_21), 34_usize, _34, 34_usize, _34), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [isize; 6],mut _5: [isize; 6],mut _6: [isize; 6],mut _7: [isize; 6],mut _8: [isize; 6],mut _9: isize,mut _10: [isize; 6],mut _11: isize,mut _12: isize,mut _13: [isize; 6],mut _14: [isize; 6]) -> i64 {
mir! {
type RET = i64;
let _15: [isize; 5];
let _16: (usize,);
let _17: isize;
let _18: (usize,);
let _19: [isize; 6];
let _20: *mut (i8, (usize,), *mut f32);
let _21: bool;
let _22: usize;
let _23: (u8, f32, (usize,), *mut u32);
let _24: i64;
let _25: [i32; 5];
let _26: [i8; 4];
let _27: (i64, u32);
let _28: (u8, f32, (usize,), *mut u32);
let _29: [i32; 5];
let _30: [i8; 4];
let _31: ((u64,), *mut f32);
let _32: i128;
let _33: [u8; 1];
let _34: ();
let _35: ();
{
_14 = _7;
_2 = 35_i8 as isize;
_13 = [_1,_3,_12,_11,_9,_12];
_8 = _5;
_13 = [_11,_12,_12,_11,_12,_2];
RET = 95016944906310117984988753413997966811_i128 as i64;
_7 = [_3,_3,_11,_1,_3,_11];
RET = -(-948777782703996533_i64);
_13 = [_11,_1,_2,_12,_1,_3];
_4 = [_1,_12,_1,_9,_3,_3];
_16.0 = 11796294458870982598_usize;
_11 = _9 & _1;
_16 = (16846470893293621643_usize,);
RET = 6_u8 as i64;
RET = (-7665310795296234955_i64) >> _11;
_17 = 72256266260200937738059210833769560740_i128 as isize;
_14 = _5;
_12 = _11;
match _16.0 {
0 => bb1,
16846470893293621643 => bb3,
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
_2 = -_12;
_17 = _12;
_7 = _5;
_18.0 = !_16.0;
_5 = _8;
_19 = [_1,_17,_11,_9,_17,_3];
RET = (-1893103172360606791_i64) - (-103935950583466297_i64);
_7 = [_1,_17,_17,_11,_2,_2];
_23.0 = 232_u8;
_23.1 = 36070_u16 as f32;
match _16.0 {
0 => bb4,
1 => bb5,
16846470893293621643 => bb7,
_ => bb6
}
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
_24 = _23.0 as i64;
_14 = [_2,_11,_2,_2,_2,_12];
_19 = [_12,_11,_2,_17,_11,_3];
_21 = true;
_23.0 = !134_u8;
_16 = (_18.0,);
_16.0 = _11 as usize;
_16.0 = _18.0;
_15 = [_2,_12,_9,_11,_2];
_4 = [_12,_17,_2,_2,_11,_12];
_28.3 = core::ptr::addr_of_mut!(_27.1);
_13 = [_2,_2,_2,_12,_17,_11];
Call(_22 = core::intrinsics::transmute(_2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_18 = _16;
_3 = '\u{376c4}' as isize;
_17 = _1;
_5 = [_2,_1,_11,_17,_12,_9];
_23.1 = 334659319982303742933041006428075201410_u128 as f32;
_23.3 = core::ptr::addr_of_mut!(_27.1);
_21 = false ^ true;
_27 = (_24, 1101583848_u32);
_22 = 23439_u16 as usize;
_23.2.0 = _22;
_6 = [_12,_11,_1,_11,_11,_17];
_14 = [_1,_1,_12,_2,_9,_2];
_4 = [_2,_1,_12,_17,_2,_9];
_7 = _13;
_10 = [_12,_9,_12,_12,_12,_12];
_26 = [79_i8,(-101_i8),73_i8,67_i8];
_25 = [891697749_i32,(-1473158532_i32),(-734137141_i32),1673622136_i32,93735584_i32];
_28.2.0 = _16.0 / 1_usize;
_15 = [_1,_2,_12,_12,_2];
match _27.1 {
0 => bb7,
1 => bb2,
2 => bb6,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
1101583848 => bb14,
_ => bb13
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
Return()
}
bb13 = {
Return()
}
bb14 = {
_29 = [(-823938709_i32),(-132237707_i32),705700776_i32,1735712801_i32,(-41928805_i32)];
_4 = _8;
_15 = [_2,_2,_2,_9,_9];
_18 = (_22,);
_28.3 = core::ptr::addr_of_mut!(_27.1);
_16.0 = (-665_i16) as usize;
_18 = (_16.0,);
_31.1 = core::ptr::addr_of_mut!(_23.1);
_23.1 = _27.1 as f32;
_19 = _13;
_3 = 60914377449482017515258408953376586621_i128 as isize;
_28 = _23;
_15 = [_11,_11,_12,_11,_12];
_22 = _23.2.0 - _28.2.0;
_23.3 = _28.3;
_14 = [_11,_2,_11,_2,_9,_9];
_17 = !_1;
_23.2.0 = _16.0 - _16.0;
_12 = _16.0 as isize;
_23.0 = (-145626907913053881539048084202446383777_i128) as u8;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(4_usize, 25_usize, Move(_25), 27_usize, Move(_27), 26_usize, Move(_26), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(4_usize, 5_usize, Move(_5), 29_usize, Move(_29), 22_usize, Move(_22), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(4_usize, 9_usize, Move(_9), 17_usize, Move(_17), 15_usize, Move(_15), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(4_usize, 7_usize, Move(_7), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: (u128,),mut _2: bool,mut _3: ((u64,), *mut f32),mut _4: [isize; 6],mut _5: *mut u32,mut _6: *const usize,mut _7: u64,mut _8: f32,mut _9: [isize; 6],mut _10: [isize; 6],mut _11: (u8, f32, (usize,), *mut u32),mut _12: *mut f32,mut _13: [i32; 5],mut _14: u8,mut _15: (usize,)) -> Adt55 {
mir! {
type RET = Adt55;
let _16: i64;
let _17: isize;
let _18: isize;
let _19: [u8; 1];
let _20: Adt49;
let _21: bool;
let _22: Adt55;
let _23: f64;
let _24: (isize, [i32; 5]);
let _25: [i32; 5];
let _26: i8;
let _27: Adt51;
let _28: Adt52;
let _29: Adt49;
let _30: Adt49;
let _31: [i128; 1];
let _32: [i8; 4];
let _33: u16;
let _34: [isize; 5];
let _35: char;
let _36: bool;
let _37: (isize, [i32; 5]);
let _38: char;
let _39: ();
let _40: ();
{
_9 = _10;
RET.fld0.0.1.0 = !_15.0;
RET.fld0.2 = _10;
RET.fld1.0 = !_3.0.0;
RET.fld0.0.2 = _12;
RET.fld3 = (_15, _3.0, _2, _6);
RET.fld2 = [105332354024472742543289069383916027367_i128];
_15.0 = (*_6) % 1_usize;
(*_12) = 45312900401704448420937712256786314529_i128 as f32;
(*_6) = !_11.2.0;
RET.fld0.0.1 = (_15.0,);
RET.fld6.0 = (_7,);
RET.fld3.0.0 = !_15.0;
_17 = -102_isize;
_11.2.0 = _17 as usize;
_20.fld2 = !(-59237371712991323951315740319741521928_i128);
Call(RET.fld0.0 = fn6(_3.1, _9, _11, _10, _3, _11.1, (*_6), _3, _5, _11, _11.1, _3, _4, _3.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.2.0 = _15.0 - _15.0;
_17 = _20.fld2 as isize;
_22.fld5 = !_1.0;
RET.fld4 = core::ptr::addr_of!(_19);
_1 = (_22.fld5,);
RET.fld0.0.2 = _12;
RET.fld1 = (_7,);
_11.0 = _14;
RET.fld6.1 = core::ptr::addr_of_mut!((*_12));
_22.fld0.0.1 = (_15.0,);
_24 = (_17, _13);
_19 = [_14];
(*_5) = _2 as u32;
Call(RET.fld6 = fn16(_9, (*_12), _24.1, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_22.fld0.0.1 = (_11.2.0,);
_22.fld3.0.0 = _11.2.0 * _11.2.0;
_22.fld3 = (_11.2, _3.0, _2, _6);
RET.fld7 = (_24.0, _24.1);
RET.fld4 = core::ptr::addr_of!(_19);
_20.fld1 = [_20.fld2];
Goto(bb3)
}
bb3 = {
_1.0 = !_22.fld5;
RET.fld0.0.2 = _12;
RET.fld2 = [_20.fld2];
RET.fld0.0.0 = (-63_i8) >> _22.fld3.0.0;
RET.fld1.0 = (*_5) as u64;
RET.fld6.1 = core::ptr::addr_of_mut!((*_12));
_22.fld3.0.0 = 7_i8 as usize;
RET.fld5 = (-24394_i16) as u128;
_28.fld0.2.0 = _11.2.0 & _22.fld0.0.1.0;
Goto(bb4)
}
bb4 = {
_16 = -(-844895720085380101_i64);
_4 = _10;
RET.fld0.0.1 = ((*_6),);
_16 = (-2619189923408348740_i64) | (-2391245818735268832_i64);
_22.fld6.0.0 = _22.fld3.2 as u64;
_22.fld0.0.1 = (_28.fld0.2.0,);
_29.fld2 = _20.fld2 - _20.fld2;
RET.fld0.0.2 = core::ptr::addr_of_mut!((*_12));
RET.fld0.2 = _4;
Goto(bb5)
}
bb5 = {
_27.fld0 = (-308308194_i32);
(*_6) = _28.fld0.2.0 | _28.fld0.2.0;
_30.fld1 = [_29.fld2];
(*_12) = -_11.1;
_18 = _22.fld5 as isize;
RET.fld3.1 = _3.0;
RET.fld3.0.0 = _11.2.0 >> (*_6);
_22.fld0.0 = ((-60_i8), _15, _12);
RET.fld1.0 = !_7;
_32 = [_22.fld0.0.0,_22.fld0.0.0,_22.fld0.0.0,_22.fld0.0.0];
_22.fld0.0.1 = _15;
_28.fld0.2 = _22.fld0.0.1;
_28 = Adt52 { fld0: _11,fld1: _1 };
_27 = Adt51 { fld0: (-930783884_i32) };
_22.fld2 = _30.fld1;
RET.fld7.1 = [_27.fld0,_27.fld0,_27.fld0,_27.fld0,_27.fld0];
_22.fld3.1.0 = _7 & _7;
_22.fld3.3 = _6;
_21 = (*_6) == _28.fld0.2.0;
_25 = _24.1;
(*_12) = _28.fld0.1;
_15 = ((*_6),);
RET.fld3 = (_15, _22.fld3.1, _21, _6);
Goto(bb6)
}
bb6 = {
RET.fld0.1 = core::ptr::addr_of_mut!(_23);
RET.fld7.1 = [_27.fld0,_27.fld0,_27.fld0,_27.fld0,_27.fld0];
_30.fld2 = (-2429_i16) as i128;
RET.fld0.0.1.0 = !(*_6);
_15.0 = _28.fld0.2.0 % 7_usize;
_22.fld0.2 = [_18,_18,_24.0,_18,_17,_18];
_18 = _22.fld0.0.0 as isize;
_20 = Adt49 { fld0: _9,fld1: _22.fld2,fld2: _30.fld2 };
RET.fld6.1 = core::ptr::addr_of_mut!((*_12));
_28.fld0.3 = core::ptr::addr_of_mut!((*_5));
RET.fld0.0.0 = (*_12) as i8;
_22.fld6.1 = core::ptr::addr_of_mut!(_28.fld0.1);
_36 = (*_6) == (*_6);
_20.fld2 = _29.fld2;
_24.1 = [_27.fld0,_27.fld0,_27.fld0,_27.fld0,_27.fld0];
_30 = Move(_20);
_6 = core::ptr::addr_of!(_11.2.0);
_5 = core::ptr::addr_of_mut!((*_5));
Goto(bb7)
}
bb7 = {
Call(_39 = dump_var(5_usize, 25_usize, Move(_25), 19_usize, Move(_19), 24_usize, Move(_24), 9_usize, Move(_9)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_39 = dump_var(5_usize, 15_usize, Move(_15), 16_usize, Move(_16), 32_usize, Move(_32), 2_usize, Move(_2)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_39 = dump_var(5_usize, 17_usize, Move(_17), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: *mut f32,mut _2: [isize; 6],mut _3: (u8, f32, (usize,), *mut u32),mut _4: [isize; 6],mut _5: ((u64,), *mut f32),mut _6: f32,mut _7: usize,mut _8: ((u64,), *mut f32),mut _9: *mut u32,mut _10: (u8, f32, (usize,), *mut u32),mut _11: f32,mut _12: ((u64,), *mut f32),mut _13: [isize; 6],mut _14: (u64,)) -> (i8, (usize,), *mut f32) {
mir! {
type RET = (i8, (usize,), *mut f32);
let _15: (usize,);
let _16: ((i8, (usize,), *mut f32), *mut f64, [isize; 6]);
let _17: i8;
let _18: f64;
let _19: char;
let _20: ((u64,), *mut f32);
let _21: (u128,);
let _22: i16;
let _23: bool;
let _24: (u128,);
let _25: i64;
let _26: isize;
let _27: (isize, [i32; 5]);
let _28: [u64; 4];
let _29: char;
let _30: bool;
let _31: [i8; 4];
let _32: char;
let _33: ();
let _34: ();
{
RET.2 = _8.1;
_4 = _2;
RET.0 = -(-97_i8);
RET = (96_i8, _3.2, _1);
_3.3 = _10.3;
_10.2.0 = !_3.2.0;
_7 = _10.2.0 + _3.2.0;
_2 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_10.3 = core::ptr::addr_of_mut!((*_9));
_3 = _10;
(*_1) = -_6;
_11 = _10.1;
_11 = 7847_u16 as f32;
_10.1 = _3.1;
_9 = core::ptr::addr_of_mut!((*_9));
_2 = _4;
_5 = (_8.0, _8.1);
_14 = (_8.0.0,);
_15.0 = _7;
Call(_8.0 = fn7(_3, _10.3, _12.1, _11, _12.0.0, _11, _3.2, _9, _3.2.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = -(*_1);
_4 = [(-9223372036854775808_isize),125_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-95_isize),9223372036854775807_isize];
_5.1 = _8.1;
_12.0.0 = 9223372036854775807_isize as u64;
RET.2 = core::ptr::addr_of_mut!(_11);
_9 = _10.3;
_15.0 = !_7;
_10.0 = !_3.0;
_3.3 = _10.3;
RET.0 = 9_i8 ^ 78_i8;
RET = ((-89_i8), _10.2, _5.1);
_10.2 = (_3.2.0,);
_15 = (_10.2.0,);
_10.2.0 = (-9223372036854775808_isize) as usize;
Goto(bb2)
}
bb2 = {
(*_9) = !4202884060_u32;
_16.0 = (48_i8, _15, _1);
_3.3 = _10.3;
_20.0.0 = _5.0.0 + _5.0.0;
_15 = _10.2;
RET = (_16.0.0, _16.0.1, _5.1);
_1 = _5.1;
_18 = 9223372036854775807_isize as f64;
_8.1 = core::ptr::addr_of_mut!((*_1));
_16.0.1 = (_7,);
_23 = true;
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-1_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_15.0 = !_10.2.0;
_22 = 877142519_i32 as i16;
_3.2.0 = _16.0.1.0 >> _20.0.0;
_8.1 = _1;
RET.1.0 = (*_1) as usize;
_12.0.0 = _20.0.0;
_25 = (-2002024009032702668_i64);
_16.2 = [(-93_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-40_isize)];
match _25 {
0 => bb3,
1 => bb4,
340282366920938463461372583422735508788 => bb6,
_ => bb5
}
}
bb3 = {
_11 = -(*_1);
_4 = [(-9223372036854775808_isize),125_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-95_isize),9223372036854775807_isize];
_5.1 = _8.1;
_12.0.0 = 9223372036854775807_isize as u64;
RET.2 = core::ptr::addr_of_mut!(_11);
_9 = _10.3;
_15.0 = !_7;
_10.0 = !_3.0;
_3.3 = _10.3;
RET.0 = 9_i8 ^ 78_i8;
RET = ((-89_i8), _10.2, _5.1);
_10.2 = (_3.2.0,);
_15 = (_10.2.0,);
_10.2.0 = (-9223372036854775808_isize) as usize;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_10.2 = (_7,);
_16.0.2 = core::ptr::addr_of_mut!(_3.1);
_8.0.0 = _12.0.0;
_20.0.0 = _12.0.0 << _5.0.0;
_3.1 = 1041204592_i32 as f32;
_8 = (_14, _12.1);
_15 = (_3.2.0,);
_20 = _12;
match _16.0.0 {
0 => bb7,
1 => bb8,
48 => bb10,
_ => bb9
}
}
bb7 = {
(*_9) = !4202884060_u32;
_16.0 = (48_i8, _15, _1);
_3.3 = _10.3;
_20.0.0 = _5.0.0 + _5.0.0;
_15 = _10.2;
RET = (_16.0.0, _16.0.1, _5.1);
_1 = _5.1;
_18 = 9223372036854775807_isize as f64;
_8.1 = core::ptr::addr_of_mut!((*_1));
_16.0.1 = (_7,);
_23 = true;
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-1_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_15.0 = !_10.2.0;
_22 = 877142519_i32 as i16;
_3.2.0 = _16.0.1.0 >> _20.0.0;
_8.1 = _1;
RET.1.0 = (*_1) as usize;
_12.0.0 = _20.0.0;
_25 = (-2002024009032702668_i64);
_16.2 = [(-93_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-40_isize)];
match _25 {
0 => bb3,
1 => bb4,
340282366920938463461372583422735508788 => bb6,
_ => bb5
}
}
bb8 = {
Return()
}
bb9 = {
_11 = -(*_1);
_4 = [(-9223372036854775808_isize),125_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-95_isize),9223372036854775807_isize];
_5.1 = _8.1;
_12.0.0 = 9223372036854775807_isize as u64;
RET.2 = core::ptr::addr_of_mut!(_11);
_9 = _10.3;
_15.0 = !_7;
_10.0 = !_3.0;
_3.3 = _10.3;
RET.0 = 9_i8 ^ 78_i8;
RET = ((-89_i8), _10.2, _5.1);
_10.2 = (_3.2.0,);
_15 = (_10.2.0,);
_10.2.0 = (-9223372036854775808_isize) as usize;
Goto(bb2)
}
bb10 = {
RET.2 = core::ptr::addr_of_mut!(_3.1);
_27.0 = (-9223372036854775808_isize) << (*_9);
_7 = _15.0 ^ _15.0;
_23 = _15.0 > _7;
(*_1) = -_11;
_21.0 = 73658094086223788986461348217595187031_u128 / 97864739048307229934332101116060978781_u128;
_3 = (_10.0, _10.1, _15, _9);
_16.0.1 = (_7,);
_9 = core::ptr::addr_of_mut!((*_9));
_19 = '\u{5d817}';
RET.1 = (_7,);
RET.0 = _21.0 as i8;
_26 = _27.0;
_7 = _3.2.0;
_10 = _3;
_29 = _19;
RET.0 = 93918418412543094393005665913463451155_i128 as i8;
_24.0 = _21.0;
Goto(bb11)
}
bb11 = {
Call(_33 = dump_var(6_usize, 22_usize, Move(_22), 29_usize, Move(_29), 23_usize, Move(_23), 2_usize, Move(_2)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_33 = dump_var(6_usize, 14_usize, Move(_14), 26_usize, Move(_26), 24_usize, Move(_24), 34_usize, _34), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: (u8, f32, (usize,), *mut u32),mut _2: *mut u32,mut _3: *mut f32,mut _4: f32,mut _5: u64,mut _6: f32,mut _7: (usize,),mut _8: *mut u32,mut _9: usize) -> (u64,) {
mir! {
type RET = (u64,);
let _10: [i128; 1];
let _11: f32;
let _12: bool;
let _13: i64;
let _14: (u8, f32, (usize,), *mut u32);
let _15: u32;
let _16: u128;
let _17: [i8; 4];
let _18: ((i8, (usize,), *mut f32), *mut f64, [isize; 6]);
let _19: bool;
let _20: [u8; 1];
let _21: (u64,);
let _22: ((u64,), f64);
let _23: [i128; 1];
let _24: *mut f32;
let _25: bool;
let _26: [u64; 4];
let _27: [u8; 1];
let _28: isize;
let _29: *const *const usize;
let _30: *const [u8; 1];
let _31: *const f32;
let _32: [i128; 1];
let _33: *const [u8; 1];
let _34: [i128; 1];
let _35: Adt48;
let _36: isize;
let _37: Adt47;
let _38: [isize; 6];
let _39: isize;
let _40: isize;
let _41: (isize, [i32; 5]);
let _42: [i32; 5];
let _43: [i128; 1];
let _44: f32;
let _45: *mut f32;
let _46: i8;
let _47: char;
let _48: bool;
let _49: ((i8, (usize,), *mut f32), *mut f64, [isize; 6]);
let _50: ();
let _51: ();
{
RET.0 = !_5;
(*_2) = _5 as u32;
_1.0 = 224_u8 ^ 196_u8;
_8 = core::ptr::addr_of_mut!((*_8));
RET.0 = _1.0 as u64;
RET = (_5,);
_1.1 = _4 * _6;
(*_8) = 2891854147_u32 - 3939476201_u32;
RET = (_5,);
(*_3) = _6;
(*_3) = _1.2.0 as f32;
_8 = core::ptr::addr_of_mut!((*_2));
_1.3 = core::ptr::addr_of_mut!((*_2));
RET = (_5,);
_1.1 = (*_3) * _4;
(*_3) = _1.1 * _4;
_4 = _1.0 as f32;
Call((*_2) = fn8(_1, _6, (*_3), _2, _7.0, _7.0, _8, _1, _5, _8, _1.2.0, _2, (*_3), _1.3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _8;
RET.0 = _5 & _5;
_1.2 = (_9,);
(*_8) = 2718445043_u32 >> _5;
_10 = [130481729974494511928919291456965341122_i128];
_5 = !13464624036993856995_u64;
_1.2.0 = (*_3) as usize;
_7.0 = _1.2.0 ^ _1.2.0;
_12 = true;
_4 = -_6;
RET = (_5,);
_12 = _1.1 != (*_3);
RET = (_5,);
_11 = 1754931866_i32 as f32;
_2 = _8;
Goto(bb2)
}
bb2 = {
_1.2 = _7;
(*_8) = 2741873288_u32 | 1052131121_u32;
_2 = core::ptr::addr_of_mut!((*_8));
_7 = (_9,);
RET.0 = !_5;
(*_3) = _11 - _6;
_5 = 17048301215128955379_u64 * 9553227611048704345_u64;
(*_3) = _1.1 + _6;
_1.3 = _8;
_2 = core::ptr::addr_of_mut!((*_8));
RET.0 = _5;
_12 = false;
(*_2) = !4127576351_u32;
_1.3 = core::ptr::addr_of_mut!((*_8));
_13 = -2926796476716046383_i64;
Call(_4 = core::intrinsics::transmute((*_8)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_2) = 2749157697_u32;
_4 = -(*_3);
_5 = 18086277708178885776_u64;
(*_2) = 1582348140_u32 & 1251014263_u32;
_9 = !_1.2.0;
_14.2.0 = _9;
_1.2 = (_14.2.0,);
(*_3) = (-267654836_i32) as f32;
_17 = [(-79_i8),(-108_i8),(-60_i8),120_i8];
_14.0 = !_1.0;
_7.0 = 14139_u16 as usize;
_13 = 7223645000065912431_i64 << (*_2);
_6 = (*_3);
RET = (_5,);
_2 = _1.3;
RET.0 = _5;
_18.2 = [(-9223372036854775808_isize),36_isize,9223372036854775807_isize,(-9223372036854775808_isize),113_isize,(-4_isize)];
_14.1 = (-9223372036854775808_isize) as f32;
_7.0 = 122837256379962046643046689199536737655_i128 as usize;
_18.0 = ((-54_i8), _14.2, _3);
_1.2.0 = _18.0.1.0 ^ _18.0.1.0;
_14 = (_1.0, _11, _18.0.1, _1.3);
_14 = _1;
_19 = _12;
(*_3) = _1.0 as f32;
match _18.0.0 {
340282366920938463463374607431768211402 => bb5,
_ => bb4
}
}
bb4 = {
_1.2 = _7;
(*_8) = 2741873288_u32 | 1052131121_u32;
_2 = core::ptr::addr_of_mut!((*_8));
_7 = (_9,);
RET.0 = !_5;
(*_3) = _11 - _6;
_5 = 17048301215128955379_u64 * 9553227611048704345_u64;
(*_3) = _1.1 + _6;
_1.3 = _8;
_2 = core::ptr::addr_of_mut!((*_8));
RET.0 = _5;
_12 = false;
(*_2) = !4127576351_u32;
_1.3 = core::ptr::addr_of_mut!((*_8));
_13 = -2926796476716046383_i64;
Call(_4 = core::intrinsics::transmute((*_8)), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_7.0 = (-9223372036854775808_isize) as usize;
(*_8) = 841434782_u32 ^ 1248598420_u32;
_18.0.1.0 = !_1.2.0;
_2 = _8;
RET = (_5,);
_9 = _19 as usize;
_14.1 = _1.1 / 0.000000000000000000000000000000000000000640672256590842_f32;
_12 = _19;
(*_3) = _4;
_7.0 = _18.0.1.0;
_5 = 3099796562566054288_u64;
_5 = !576358676666590214_u64;
_10 = [91646324332673180725698222926063666276_i128];
_8 = _2;
_14 = (_1.0, _11, _18.0.1, _2);
_19 = _12;
_18.0.2 = core::ptr::addr_of_mut!(_1.1);
_6 = _13 as f32;
_1.3 = core::ptr::addr_of_mut!(_15);
_18.0.1 = (_1.2.0,);
_18.2 = [9223372036854775807_isize,9223372036854775807_isize,(-114_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_15 = (*_2);
_1.0 = !_14.0;
(*_2) = !_15;
Goto(bb6)
}
bb6 = {
_9 = _1.2.0;
_21.0 = _5 + _5;
_7 = (_1.2.0,);
Goto(bb7)
}
bb7 = {
_18.1 = core::ptr::addr_of_mut!(_22.1);
_21 = (_5,);
_7 = (_18.0.1.0,);
_1 = (_14.0, _11, _14.2, _2);
_8 = core::ptr::addr_of_mut!((*_8));
_22.1 = _14.0 as f64;
Call(RET = fn12(_5, _1.2, _17, _14, _18.0.0, _12, _14.2.0, (*_3), (*_8), _7.0, _1, _14, _14.3, (*_3), (*_8), _12), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1.2.0 = _12 as usize;
(*_2) = !_15;
_3 = _18.0.2;
_11 = _4;
_26 = [_21.0,_5,_21.0,_5];
(*_3) = _14.2.0 as f32;
_23 = [97886191727687579727101406749925369466_i128];
_20 = [_1.0];
_1.1 = -_6;
_18.0.0 = (*_2) as i8;
_4 = _1.1;
_26 = [_5,_5,_21.0,_5];
_17 = [_18.0.0,_18.0.0,_18.0.0,_18.0.0];
(*_2) = !_15;
_1.2 = (_7.0,);
_12 = !_19;
_16 = 122434376379958039267161789973076293270_u128 + 184270808652046458001603627251085673921_u128;
_7.0 = _21.0 as usize;
_23 = [(-12983016265741366860737240141073069002_i128)];
_20 = [_14.0];
_30 = core::ptr::addr_of!(_27);
_25 = !_12;
_27 = [_14.0];
Goto(bb9)
}
bb9 = {
_18.0.2 = core::ptr::addr_of_mut!((*_3));
_30 = core::ptr::addr_of!(_27);
_14.2.0 = _9;
_2 = core::ptr::addr_of_mut!((*_8));
(*_8) = _15 ^ _15;
_20 = [_1.0];
_22.1 = _18.0.0 as f64;
_26 = [_21.0,_5,_5,_5];
_1.0 = _14.0 % 216_u8;
_14.1 = -(*_3);
_18.2 = [(-9223372036854775808_isize),9223372036854775807_isize,30_isize,(-9223372036854775808_isize),108_isize,(-9223372036854775808_isize)];
_15 = (*_8) << _9;
(*_30) = _20;
_22.0.0 = _21.0;
(*_8) = _15;
_24 = core::ptr::addr_of_mut!(_14.1);
_32 = [(-57931053097609324035163731294429424935_i128)];
(*_30) = _20;
(*_2) = !_15;
_1.2 = (_9,);
RET.0 = !_22.0.0;
_14.2.0 = !_1.2.0;
_1.2 = (_18.0.1.0,);
_17 = [_18.0.0,_18.0.0,_18.0.0,_18.0.0];
Goto(bb10)
}
bb10 = {
_11 = _1.1 * (*_24);
_29 = core::ptr::addr_of!(_35.fld3.3);
_1.1 = -_14.1;
_4 = -(*_24);
_18.0.1 = (_1.2.0,);
_31 = core::ptr::addr_of!(_1.1);
_35.fld3.0.0 = !_1.2.0;
_22.1 = (*_31) as f64;
_31 = core::ptr::addr_of!((*_31));
_14.0 = !_1.0;
(*_3) = 624249009_i32 as f32;
_37.fld0 = _17;
_35.fld3.0.0 = !_14.2.0;
(*_3) = _11 / (-0.000000000000000000000000000000000000007167221255482142_f32);
_9 = _1.2.0;
_22.1 = 21808_u16 as f64;
_23 = _10;
_31 = core::ptr::addr_of!(_6);
_39 = 13704_i16 as isize;
_32 = [(-47948463301329434427884737143352533440_i128)];
_23 = [(-64499926729359049710482038447211030466_i128)];
_1.0 = _14.0 << _1.2.0;
_35.fld5.0 = _22.1 as isize;
_14.3 = core::ptr::addr_of_mut!((*_8));
Goto(bb11)
}
bb11 = {
(*_31) = 1069084302_i32 as f32;
_1.3 = core::ptr::addr_of_mut!((*_8));
_43 = [7755166940269241152953568730400002022_i128];
Goto(bb12)
}
bb12 = {
(*_29) = core::ptr::addr_of!(_7.0);
Goto(bb13)
}
bb13 = {
_21.0 = _22.0.0 % 10080375824879936066_u64;
_46 = !_18.0.0;
_11 = (*_8) as f32;
_16 = !51370931953909115442923287745016438249_u128;
(*_24) = _4;
_35.fld3.2 = !_19;
_5 = '\u{cbc7}' as u64;
_37.fld1 = core::ptr::addr_of!(_27);
_5 = _21.0;
_29 = core::ptr::addr_of!((*_29));
_35.fld0 = _19 & _19;
(*_29) = core::ptr::addr_of!(_35.fld6.2.0);
_11 = _1.1 - (*_3);
_29 = core::ptr::addr_of!(_35.fld3.3);
_22.0 = _21;
_20 = [_1.0];
_47 = '\u{be0c9}';
Goto(bb14)
}
bb14 = {
_18.1 = core::ptr::addr_of_mut!(_22.1);
_34 = _23;
_46 = _1.2.0 as i8;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(7_usize, 32_usize, Move(_32), 10_usize, Move(_10), 16_usize, Move(_16), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(7_usize, 43_usize, Move(_43), 12_usize, Move(_12), 15_usize, Move(_15), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(7_usize, 5_usize, Move(_5), 47_usize, Move(_47), 7_usize, Move(_7), 51_usize, _51), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: (u8, f32, (usize,), *mut u32),mut _2: f32,mut _3: f32,mut _4: *mut u32,mut _5: usize,mut _6: usize,mut _7: *mut u32,mut _8: (u8, f32, (usize,), *mut u32),mut _9: u64,mut _10: *mut u32,mut _11: usize,mut _12: *mut u32,mut _13: f32,mut _14: *mut u32) -> u32 {
mir! {
type RET = u32;
let _15: f32;
let _16: char;
let _17: f64;
let _18: isize;
let _19: Adt49;
let _20: i32;
let _21: *mut f64;
let _22: usize;
let _23: *const [u8; 1];
let _24: [i32; 5];
let _25: [i128; 1];
let _26: u64;
let _27: isize;
let _28: u16;
let _29: *const usize;
let _30: [isize; 5];
let _31: u64;
let _32: (usize,);
let _33: bool;
let _34: (i64, u32);
let _35: [i32; 5];
let _36: isize;
let _37: [u8; 1];
let _38: [i128; 1];
let _39: (u128,);
let _40: i16;
let _41: Adt48;
let _42: (u8, f32, (usize,), *mut u32);
let _43: (u128,);
let _44: f32;
let _45: char;
let _46: isize;
let _47: [i32; 5];
let _48: i128;
let _49: i128;
let _50: f32;
let _51: (u64,);
let _52: u32;
let _53: i16;
let _54: ();
let _55: ();
{
_9 = !7771740236223150718_u64;
_8 = _1;
_8 = (_1.0, _3, _1.2, _1.3);
_1 = _8;
_1.2 = (_5,);
_11 = _1.2.0 << _5;
_6 = _11;
_8.2 = (_1.2.0,);
_11 = !_6;
_1.3 = _7;
Call(RET = fn9(_14, _7, _7, _13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = _1.3;
_15 = _3;
_1.2.0 = _11 - _11;
_1 = _8;
_9 = !10747985071651117260_u64;
_14 = _4;
_3 = (-1921703935893642663_i64) as f32;
_1.2.0 = 1655923059_u32 as usize;
_6 = !_5;
RET = 1875844813_u32;
_8.3 = _4;
_16 = '\u{5f7ae}';
_19.fld2 = 4101552426222699456287491438850135568_i128 | 154888799854405505207213706516895388463_i128;
Goto(bb2)
}
bb2 = {
Goto(bb3)
}
bb3 = {
_18 = -125_isize;
_4 = _10;
_2 = _1.1 + _13;
RET = _19.fld2 as u32;
_2 = -_8.1;
_1.0 = _8.0;
_1.2.0 = _6 - _11;
_19.fld2 = !51633614877717126949865089648952345897_i128;
RET = _1.0 as u32;
RET = !1097347510_u32;
_8.0 = _1.0 / 198_u8;
Goto(bb4)
}
bb4 = {
_17 = 251159508254528065706572124312252087045_u128 as f64;
_6 = _8.2.0;
_1.1 = _2;
_17 = 254862817980349185101082263658063756301_u128 as f64;
_24 = [(-188609239_i32),457803452_i32,77241569_i32,(-10463733_i32),(-43714264_i32)];
_25 = [_19.fld2];
_5 = _1.2.0 >> _11;
_21 = core::ptr::addr_of_mut!(_17);
_5 = !_8.2.0;
_8.3 = _1.3;
_6 = !_1.2.0;
Goto(bb5)
}
bb5 = {
_5 = !_11;
_8.2.0 = _1.2.0;
_19.fld1 = [_19.fld2];
_4 = _8.3;
_1.2 = _8.2;
_8.2 = _1.2;
_6 = !_8.2.0;
Goto(bb6)
}
bb6 = {
_26 = _9 + _9;
_26 = (-30601_i16) as u64;
_8.1 = _19.fld2 as f32;
_6 = _5;
_22 = _11 | _8.2.0;
RET = !910853132_u32;
_20 = 806882316_i32;
_9 = _26;
_1.0 = _8.0;
_12 = _7;
_2 = -_15;
_12 = _4;
_6 = _8.2.0 % 7_usize;
_12 = _4;
_1.2.0 = _26 as usize;
_20 = 2110975673_i32 ^ 2068576346_i32;
_19.fld1 = [_19.fld2];
_8.2.0 = _6 - _6;
_27 = _18 & _18;
_3 = _15;
_1 = (_8.0, _3, _8.2, _14);
_11 = _6;
_14 = _8.3;
_26 = _9 - _9;
Goto(bb7)
}
bb7 = {
_8.2 = _1.2;
_27 = 1756633206680654373_i64 as isize;
_16 = '\u{77419}';
_24 = [_20,_20,_20,_20,_20];
_14 = _8.3;
_24 = [_20,_20,_20,_20,_20];
_8.2 = (_1.2.0,);
_1.2.0 = _11 + _8.2.0;
_22 = !_6;
_6 = _1.2.0;
_1.1 = _2 / 0.0000000000000000000000000000000000000012220948115131022_f32;
_8.1 = _3 / f32::NAN;
_8.3 = _10;
_29 = core::ptr::addr_of!(_1.2.0);
_8.3 = core::ptr::addr_of_mut!(_34.1);
_17 = (*_29) as f64;
(*_21) = 7383891465311115650_i64 as f64;
_35 = _24;
_19.fld0 = [_18,_18,_27,_27,_27,_18];
_25 = [_19.fld2];
_1.1 = (-3632370423501479058_i64) as f32;
Call(_8.0 = core::intrinsics::transmute(_1.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20 = 268303966_i32 + 1749182308_i32;
_8 = _1;
_36 = 34_i8 as isize;
_4 = _1.3;
_19.fld2 = false as i128;
_8 = (_1.0, _13, _1.2, _7);
RET = !3072461694_u32;
_33 = !false;
_1.2.0 = 150613901_u32 as usize;
_20 = _16 as i32;
_6 = _8.2.0;
_8.1 = _2 * _15;
(*_29) = _5 | _6;
_14 = core::ptr::addr_of_mut!(_34.1);
_12 = core::ptr::addr_of_mut!(_34.1);
_3 = _8.1 / (-0.000000000000000000000000000000000000008627434511142567_f32);
_8.2.0 = 83052957692152472947059873101348255645_u128 as usize;
(*_12) = _26 as u32;
_33 = true;
_9 = _16 as u64;
_37 = [_8.0];
_19.fld1 = [_19.fld2];
Goto(bb9)
}
bb9 = {
_1.3 = _8.3;
_1.3 = core::ptr::addr_of_mut!((*_12));
_34 = (4008144222210445726_i64, 1562170574_u32);
_8 = (_1.0, _2, _1.2, _4);
(*_12) = 4108049875_u32;
_18 = _36;
_1.1 = _8.1 / (-0.000000000000000000000000000000000000006908775575811323_f32);
_15 = _8.1 / 0.000000000000000000000000000000000000009577834366004659_f32;
_41.fld3.3 = core::ptr::addr_of!(_8.2.0);
(*_14) = 18186_u16 as u32;
_4 = _8.3;
_41.fld6.2 = (_5,);
(*_29) = _8.2.0;
_41.fld6.1 = _34.1 as f32;
_42.2.0 = (*_29) ^ _8.2.0;
_17 = _3 as f64;
_8.1 = _2 + _3;
_6 = !(*_29);
Call(_42.2 = fn11(_8, _41.fld6.2, (*_29), Move(_19), _10, _27, (*_29), _8.2, _41.fld3.3, _15, _1.2.0, _36, _13, _1.1, _41.fld3.3, _41.fld6.2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Goto(bb11)
}
bb11 = {
(*_14) = 3422330040_u32;
_39.0 = 83055790106537257119578670373264362657_u128 * 25883479347854635872461295175694405246_u128;
_39 = (112634480783162274304011199252585951094_u128,);
_21 = core::ptr::addr_of_mut!(_17);
_33 = true;
_44 = _8.1 - _1.1;
_41.fld6 = (_8.0, _8.1, _42.2, _4);
_36 = _18 * _18;
_42.2 = _8.2;
_41.fld1.0 = _26;
_27 = _18;
_41.fld6.3 = core::ptr::addr_of_mut!((*_14));
_41.fld4 = 35522_u16 ^ 33930_u16;
_6 = (*_29) >> (*_29);
_16 = '\u{dc288}';
_41.fld5.0 = _36 + _36;
_41.fld5.1 = _35;
Goto(bb12)
}
bb12 = {
_41.fld3.1.0 = _41.fld1.0 - _41.fld1.0;
_14 = core::ptr::addr_of_mut!((*_12));
_20 = (-14642274_i32);
_8.0 = !_41.fld6.0;
_41.fld2 = [_41.fld1.0,_26,_41.fld1.0,_41.fld3.1.0];
_11 = _22;
_30 = [_18,_27,_41.fld5.0,_27,_27];
_11 = !_8.2.0;
Goto(bb13)
}
bb13 = {
_9 = _41.fld1.0 / 3792745802406485098_u64;
_41.fld4 = _16 as u16;
_35 = [_20,_20,_20,_20,_20];
_41.fld3.3 = core::ptr::addr_of!(_5);
_41.fld5.0 = _18;
_41.fld6 = _8;
_41.fld6 = _8;
_46 = (-114436672634981772291937720281268442199_i128) as isize;
_8 = (_1.0, _2, _41.fld6.2, _12);
match (*_14) {
3422330040 => bb15,
_ => bb14
}
}
bb14 = {
Goto(bb3)
}
bb15 = {
_39.0 = 201034972220729251975355782034460586532_u128 | 286425518730402491577075900370904845450_u128;
_15 = _3;
_42 = (_41.fld6.0, _44, _1.2, _14);
_14 = core::ptr::addr_of_mut!(_34.1);
_41.fld6.3 = core::ptr::addr_of_mut!(_52);
_9 = _26 - _41.fld1.0;
_1.2 = _8.2;
_4 = core::ptr::addr_of_mut!(_52);
_10 = core::ptr::addr_of_mut!((*_12));
_4 = core::ptr::addr_of_mut!((*_4));
_41.fld2 = [_26,_9,_41.fld3.1.0,_41.fld3.1.0];
_35 = [_20,_20,_20,_20,_20];
_42.3 = core::ptr::addr_of_mut!((*_14));
_41.fld3.1.0 = _9 | _9;
(*_12) = _8.1 as u32;
_41.fld3 = (_1.2, _41.fld1, _33, _29);
_34 = ((-2846925824452030472_i64), 1233946328_u32);
_50 = -_1.1;
_34.1 = 993000380_u32;
_53 = !(-25341_i16);
_27 = !_46;
_42 = (_1.0, _8.1, _41.fld3.0, _12);
Goto(bb16)
}
bb16 = {
Call(_54 = dump_var(8_usize, 5_usize, Move(_5), 20_usize, Move(_20), 16_usize, Move(_16), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_54 = dump_var(8_usize, 26_usize, Move(_26), 36_usize, Move(_36), 35_usize, Move(_35), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_54 = dump_var(8_usize, 22_usize, Move(_22), 34_usize, Move(_34), 55_usize, _55, 55_usize, _55), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *mut u32,mut _2: *mut u32,mut _3: *mut u32,mut _4: f32) -> u32 {
mir! {
type RET = u32;
let _5: Adt51;
let _6: *mut f64;
let _7: isize;
let _8: isize;
let _9: [i32; 5];
let _10: [isize; 6];
let _11: u16;
let _12: [isize; 5];
let _13: isize;
let _14: f32;
let _15: char;
let _16: u16;
let _17: i32;
let _18: u32;
let _19: f32;
let _20: Adt49;
let _21: f32;
let _22: u64;
let _23: [i32; 5];
let _24: [u64; 4];
let _25: Adt51;
let _26: isize;
let _27: Adt50;
let _28: (u128,);
let _29: ();
let _30: ();
{
RET = !4083904107_u32;
_1 = _2;
_2 = _1;
_3 = _2;
Goto(bb1)
}
bb1 = {
_2 = _1;
_3 = _1;
_1 = _2;
RET = true as u32;
_3 = _1;
RET = 1118491370_u32;
RET = 114039434179273698257470820630027607769_u128 as u32;
_3 = _1;
_2 = _1;
_4 = 5704679648402616950483915263896858902_u128 as f32;
RET = 1426948984_u32 % 2170089113_u32;
RET = 1994179120_u32;
_3 = _1;
RET = !3124936095_u32;
Call(_7 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = 9259407666107602861_usize as f32;
_3 = _1;
_2 = _3;
_5 = Adt51 { fld0: (-168143125_i32) };
_4 = 2161109428_u32 as f32;
_5.fld0 = (-1129580630_i32) - (-1608267574_i32);
_3 = _1;
RET = 471146482_u32;
Goto(bb3)
}
bb3 = {
_2 = _1;
_3 = _1;
Call(RET = fn10(_1, _7, _1, _1, _2, _5.fld0, _1, _5.fld0, _2, _3, _4, _3, _5, _5, _5.fld0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = 13_isize as f32;
_3 = _1;
_1 = _2;
_8 = -(-9223372036854775808_isize);
_2 = _1;
_5 = Adt51 { fld0: (-1966553188_i32) };
_12 = [_8,_8,_8,_8,_8];
_9 = [_5.fld0,_5.fld0,_5.fld0,_5.fld0,_5.fld0];
_12 = [_8,_8,_8,_8,_8];
_5 = Adt51 { fld0: 1613904648_i32 };
_10 = [_8,_8,_8,_8,_8,_8];
Goto(bb5)
}
bb5 = {
_13 = _8 ^ _8;
_5 = Adt51 { fld0: (-2059912972_i32) };
_9 = [_5.fld0,_5.fld0,_5.fld0,_5.fld0,_5.fld0];
_9 = [_5.fld0,_5.fld0,_5.fld0,_5.fld0,_5.fld0];
_14 = -_4;
_16 = !40405_u16;
_4 = _14;
_15 = '\u{79db9}';
_7 = _8 ^ _13;
_2 = _3;
_12 = [_8,_7,_7,_13,_7];
_13 = _7;
_11 = _16 >> _8;
_8 = _13 << _13;
_8 = -_13;
_12 = [_13,_7,_8,_8,_7];
_2 = _1;
_3 = _1;
_7 = _5.fld0 as isize;
_3 = _2;
match _5.fld0 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463463374607429708298484 => bb13,
_ => bb12
}
}
bb6 = {
_4 = 13_isize as f32;
_3 = _1;
_1 = _2;
_8 = -(-9223372036854775808_isize);
_2 = _1;
_5 = Adt51 { fld0: (-1966553188_i32) };
_12 = [_8,_8,_8,_8,_8];
_9 = [_5.fld0,_5.fld0,_5.fld0,_5.fld0,_5.fld0];
_12 = [_8,_8,_8,_8,_8];
_5 = Adt51 { fld0: 1613904648_i32 };
_10 = [_8,_8,_8,_8,_8,_8];
Goto(bb5)
}
bb7 = {
_2 = _1;
_3 = _1;
Call(RET = fn10(_1, _7, _1, _1, _2, _5.fld0, _1, _5.fld0, _2, _3, _4, _3, _5, _5, _5.fld0), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_4 = 9259407666107602861_usize as f32;
_3 = _1;
_2 = _3;
_5 = Adt51 { fld0: (-168143125_i32) };
_4 = 2161109428_u32 as f32;
_5.fld0 = (-1129580630_i32) - (-1608267574_i32);
_3 = _1;
RET = 471146482_u32;
Goto(bb3)
}
bb9 = {
_2 = _1;
_3 = _1;
_1 = _2;
RET = true as u32;
_3 = _1;
RET = 1118491370_u32;
RET = 114039434179273698257470820630027607769_u128 as u32;
_3 = _1;
_2 = _1;
_4 = 5704679648402616950483915263896858902_u128 as f32;
RET = 1426948984_u32 % 2170089113_u32;
RET = 1994179120_u32;
_3 = _1;
RET = !3124936095_u32;
Call(_7 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb2), UnwindUnreachable())
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
_7 = _13;
_5.fld0 = 219_u8 as i32;
_5 = Adt51 { fld0: (-1040752554_i32) };
_17 = _5.fld0 - _5.fld0;
_1 = core::ptr::addr_of_mut!(_18);
(*_1) = !1194102507_u32;
_2 = _1;
_14 = 0_usize as f32;
(*_2) = 633597621_u32;
_12 = [_7,_8,_7,_8,_7];
_15 = '\u{23d26}';
_3 = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of_mut!((*_3));
_7 = _15 as isize;
_20.fld0 = _10;
_2 = _3;
_5.fld0 = -_17;
(*_3) = !286726128_u32;
_20.fld1 = [(-46538178794440803632271330415334634058_i128)];
_20.fld0 = [_7,_8,_8,_13,_13,_13];
RET = !(*_1);
_17 = !_5.fld0;
_10 = _20.fld0;
(*_3) = 3052349166_u32 << _11;
Goto(bb14)
}
bb14 = {
(*_3) = 2355252536_u32;
(*_1) = !982858259_u32;
_10 = _20.fld0;
(*_3) = 5658744402769741343_i64 as u32;
(*_2) = !864687740_u32;
_20.fld2 = !(-12484515803222286091936403556885729045_i128);
_12 = [_13,_13,_13,_8,_8];
_23 = [_5.fld0,_5.fld0,_5.fld0,_17,_17];
_17 = _5.fld0;
_12 = [_8,_8,_13,_7,_13];
_11 = _16;
_22 = !15215106102785679224_u64;
RET = !(*_1);
_20.fld1 = [_20.fld2];
_22 = !526295761179267775_u64;
_7 = -_13;
_5.fld0 = _17 - _17;
_12 = [_8,_7,_8,_8,_13];
_26 = !_7;
_5 = Adt51 { fld0: _17 };
_20.fld0 = [_13,_8,_13,_7,_13,_8];
_27.fld1.3 = core::ptr::addr_of_mut!((*_1));
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(9_usize, 10_usize, Move(_10), 17_usize, Move(_17), 9_usize, Move(_9), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(9_usize, 23_usize, Move(_23), 12_usize, Move(_12), 11_usize, Move(_11), 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: *mut u32,mut _2: isize,mut _3: *mut u32,mut _4: *mut u32,mut _5: *mut u32,mut _6: i32,mut _7: *mut u32,mut _8: i32,mut _9: *mut u32,mut _10: *mut u32,mut _11: f32,mut _12: *mut u32,mut _13: Adt51,mut _14: Adt51,mut _15: i32) -> u32 {
mir! {
type RET = u32;
let _16: ((u64,), f64);
let _17: isize;
let _18: (isize, [i32; 5]);
let _19: *const f32;
let _20: u8;
let _21: char;
let _22: i8;
let _23: isize;
let _24: Adt59;
let _25: isize;
let _26: i32;
let _27: ();
let _28: ();
{
_13 = Adt51 { fld0: _6 };
_14.fld0 = !_13.fld0;
_6 = _13.fld0;
RET = !1618114209_u32;
_7 = _10;
_13.fld0 = _6;
_14 = _13;
_15 = '\u{b47c2}' as i32;
_5 = _1;
_13.fld0 = _14.fld0;
_1 = _10;
_8 = !_13.fld0;
_16.1 = 3891672037179491234_i64 as f64;
_1 = _4;
_2 = 103_isize;
Goto(bb1)
}
bb1 = {
_13.fld0 = _14.fld0;
_9 = _5;
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
103 => bb8,
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
_5 = _3;
_11 = 269169080780858684084089741357600767276_u128 as f32;
_5 = _3;
_13 = Adt51 { fld0: _15 };
_5 = _12;
_17 = !_2;
_8 = _6;
_5 = _7;
_19 = core::ptr::addr_of!(_11);
_19 = core::ptr::addr_of!((*_19));
(*_19) = 50419_u16 as f32;
_18.1 = [_8,_8,_14.fld0,_8,_6];
_16.0 = (2696814338315433511_u64,);
_13 = Adt51 { fld0: _15 };
_19 = core::ptr::addr_of!((*_19));
_4 = _7;
_16.0.0 = !13865150143131699183_u64;
_20 = 186_u8;
_20 = 273599166603821909947788584439566388385_u128 as u8;
_13 = _14;
match _2 {
0 => bb9,
1 => bb10,
103 => bb12,
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
_18.1 = [_14.fld0,_15,_8,_13.fld0,_13.fld0];
_3 = _7;
_17 = _11 as isize;
_13 = Adt51 { fld0: _14.fld0 };
_18.0 = (-23_i8) as isize;
_21 = '\u{d27c6}';
_13 = Adt51 { fld0: _6 };
_24.fld7.fld1 = (164832571520999080074037295069797422700_u128,);
_24.fld6 = (_24.fld7.fld1.0,);
_5 = _12;
_5 = _4;
match _24.fld7.fld1.0 {
0 => bb7,
1 => bb6,
2 => bb9,
3 => bb4,
4 => bb13,
5 => bb14,
6 => bb15,
164832571520999080074037295069797422700 => bb17,
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
Return()
}
bb17 = {
RET = 3987460795_u32 | 3496899783_u32;
_4 = _10;
_24.fld7.fld0.2 = (0_usize,);
_4 = _12;
_24.fld3 = 30_i8 >> _24.fld7.fld1.0;
_22 = _17 as i8;
_24.fld7.fld0.3 = _1;
_24.fld7.fld0.2 = (7442949537295304149_usize,);
_25 = !_17;
_20 = 149_u8 - 172_u8;
_18.1 = [_14.fld0,_8,_6,_15,_6];
_11 = 4109916050_u32 as f32;
_18.1 = [_15,_8,_13.fld0,_6,_15];
_24.fld5 = _6;
_8 = _14.fld0;
_25 = _18.0;
_18.0 = _24.fld7.fld0.2.0 as isize;
_16.0.0 = 16243694706795860218_u64 % 8707983649657485295_u64;
(*_19) = 57834_u16 as f32;
_5 = _3;
_18.1 = [_24.fld5,_6,_24.fld5,_14.fld0,_14.fld0];
Goto(bb18)
}
bb18 = {
Call(_27 = dump_var(10_usize, 8_usize, Move(_8), 17_usize, Move(_17), 15_usize, Move(_15), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(10_usize, 20_usize, Move(_20), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: (u8, f32, (usize,), *mut u32),mut _2: (usize,),mut _3: usize,mut _4: Adt49,mut _5: *mut u32,mut _6: isize,mut _7: usize,mut _8: (usize,),mut _9: *const usize,mut _10: f32,mut _11: usize,mut _12: isize,mut _13: f32,mut _14: f32,mut _15: *const usize,mut _16: (usize,)) -> (usize,) {
mir! {
type RET = (usize,);
let _17: [isize; 6];
let _18: Adt48;
let _19: Adt48;
let _20: f32;
let _21: u16;
let _22: (u64,);
let _23: Adt50;
let _24: ();
let _25: ();
{
_1.1 = -_13;
_1.3 = _5;
(*_9) = 15713_u16 as usize;
_9 = core::ptr::addr_of!((*_15));
Call(_2.0 = core::intrinsics::transmute(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_18.fld3.0 = (_1.2.0,);
_2 = (_7,);
_18.fld6 = (_1.0, _1.1, _8, _1.3);
_18.fld6.0 = _1.0 - _1.0;
_7 = !_2.0;
_14 = _13;
(*_9) = !_8.0;
_19.fld5.1 = [1065099694_i32,(-63283513_i32),677187137_i32,454818237_i32,2057994471_i32];
_18.fld6.2 = _18.fld3.0;
_19.fld5.0 = -_6;
_7 = _2.0;
_19.fld6.2 = ((*_15),);
Goto(bb2)
}
bb2 = {
_19.fld0 = !false;
_19.fld3.3 = core::ptr::addr_of!(_19.fld3.0.0);
_18.fld2 = [17079623481387890096_u64,6830280438970893402_u64,14240998575913038687_u64,7884311105806668577_u64];
RET = (_3,);
_19.fld6.0 = !_18.fld6.0;
_18.fld4 = !471_u16;
_9 = core::ptr::addr_of!(_11);
_18.fld3.0 = _8;
_21 = _4.fld2 as u16;
_1.2.0 = (*_9);
_1 = _18.fld6;
_20 = _10 + _18.fld6.1;
_19.fld0 = !true;
_19.fld4 = !_18.fld4;
_10 = _1.1 * _13;
_21 = !_19.fld4;
Goto(bb3)
}
bb3 = {
Call(_24 = dump_var(11_usize, 2_usize, Move(_2), 16_usize, Move(_16), 8_usize, Move(_8), 11_usize, Move(_11)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u64,mut _2: (usize,),mut _3: [i8; 4],mut _4: (u8, f32, (usize,), *mut u32),mut _5: i8,mut _6: bool,mut _7: usize,mut _8: f32,mut _9: u32,mut _10: usize,mut _11: (u8, f32, (usize,), *mut u32),mut _12: (u8, f32, (usize,), *mut u32),mut _13: *mut u32,mut _14: f32,mut _15: u32,mut _16: bool) -> (u64,) {
mir! {
type RET = (u64,);
let _17: (isize, [i32; 5]);
let _18: f32;
let _19: Adt50;
let _20: *mut (i8, (usize,), *mut f32);
let _21: (u64,);
let _22: isize;
let _23: Adt51;
let _24: isize;
let _25: u128;
let _26: isize;
let _27: Adt51;
let _28: char;
let _29: i64;
let _30: [u8; 1];
let _31: i16;
let _32: (u128,);
let _33: isize;
let _34: Adt47;
let _35: [u64; 4];
let _36: (i32, *const [u8; 1]);
let _37: bool;
let _38: Adt54;
let _39: char;
let _40: *mut char;
let _41: u16;
let _42: char;
let _43: isize;
let _44: f32;
let _45: [isize; 5];
let _46: [isize; 6];
let _47: u64;
let _48: isize;
let _49: [u64; 4];
let _50: (i64, u32);
let _51: (u128,);
let _52: [i32; 5];
let _53: (isize, [i32; 5]);
let _54: Adt57;
let _55: char;
let _56: Adt60;
let _57: f64;
let _58: bool;
let _59: Adt51;
let _60: ();
let _61: ();
{
_11.1 = _14;
_5 = 9184552269028127286_i64 as i8;
_4 = (_12.0, _11.1, _12.2, _12.3);
_4.2.0 = _8 as usize;
RET.0 = _1;
_4.2 = (_10,);
_11.1 = _4.1;
_11 = _4;
_12.1 = _4.1 - _14;
_11.0 = _12.0 - _12.0;
_4.2.0 = _10;
(*_13) = _9 / 2291939459_u32;
_9 = (*_13) / 1108221586_u32;
_11.2 = _12.2;
(*_13) = _9 - _9;
_4.2 = _2;
_15 = '\u{ab4}' as u32;
_19.fld1.2 = _4.2;
_4.2 = (_7,);
_19.fld1.1 = _11.1 * _8;
_4.1 = _14;
_19.fld0 = [78765223940447219955149935329098980604_i128];
_11.2 = (_10,);
_19.fld1 = (_4.0, _4.1, _12.2, _11.3);
_12 = (_4.0, _4.1, _4.2, _19.fld1.3);
(*_13) = 115_isize as u32;
_12 = (_4.0, _8, _2, _13);
Goto(bb1)
}
bb1 = {
_12.0 = !_11.0;
_4.2.0 = _2.0 * _10;
_12.3 = _13;
_18 = 3316_i16 as f32;
_7 = _10 & _10;
_2.0 = !_11.2.0;
_21.0 = 28218_i16 as u64;
_4.0 = _15 as u8;
_4.2.0 = _7;
_14 = -_11.1;
_7 = _4.2.0 / 12065720125660610496_usize;
_12.2.0 = !_2.0;
_19.fld1.3 = core::ptr::addr_of_mut!(_15);
_24 = (-2_isize) + (-9223372036854775808_isize);
_13 = core::ptr::addr_of_mut!(_9);
_12.1 = _11.1 / f32::NAN;
_19.fld2 = [_24,_24,_24,_24,_24,_24];
_26 = _24;
Call(_11 = fn13(_26, _12.3, _4, _12, _6, _19, _10, _19.fld2, _19.fld1, _4.2, _12, _19.fld1.2, _19.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _16;
Call(_4.0 = core::intrinsics::bswap(_11.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11.1 = _12.1;
_11.2 = (_12.2.0,);
_1 = _21.0 | _21.0;
(*_13) = _15 << _4.2.0;
RET = (_21.0,);
_1 = (-1984690092793915448_i64) as u64;
_11.1 = 296433921921896905285038868183750568360_u128 as f32;
_10 = !_11.2.0;
_19.fld1.2 = (_11.2.0,);
_21 = (_1,);
(*_13) = !_15;
_4.2 = (_19.fld1.2.0,);
_13 = _4.3;
RET.0 = 17590_i16 as u64;
Goto(bb4)
}
bb4 = {
_19.fld0 = [14149326014959543241811916169681675921_i128];
_19.fld1.2.0 = !_7;
_4.0 = _11.0 / 74_u8;
_11.2.0 = _5 as usize;
_4.2.0 = _19.fld1.2.0 | _7;
RET.0 = !_21.0;
_23.fld0 = -1626885508_i32;
_13 = core::ptr::addr_of_mut!(_15);
_31 = (-18521_i16) ^ 2270_i16;
_12.2.0 = !_4.2.0;
_19.fld1 = _12;
_24 = _21.0 as isize;
_16 = !_6;
_4.2 = _2;
Goto(bb5)
}
bb5 = {
_32 = (149178628278234256874993275594030527300_u128,);
_19.fld1.2 = (_12.2.0,);
_19.fld1.3 = _12.3;
_21.0 = _1;
Call(_10 = core::intrinsics::transmute(_21.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_27.fld0 = (-8208279446936803173_i64) as i32;
_11.0 = _16 as u8;
_14 = _31 as f32;
_10 = _19.fld1.2.0 + _19.fld1.2.0;
_31 = (-21285_i16);
_6 = !_16;
_4.3 = core::ptr::addr_of_mut!(_15);
_24 = _26;
_17.1 = [_23.fld0,_23.fld0,_23.fld0,_27.fld0,_27.fld0];
_12.2 = (_10,);
RET.0 = _1;
_4.2 = (_19.fld1.2.0,);
_35 = [_1,_21.0,_21.0,_21.0];
_18 = -_11.1;
_27.fld0 = -_23.fld0;
_17.0 = _24;
_21.0 = !_1;
Goto(bb7)
}
bb7 = {
_11.3 = _13;
_19.fld1.1 = _8;
(*_13) = 27129_u16 as u32;
_4.1 = _8;
RET.0 = _21.0;
_34.fld0 = [_5,_5,_5,_5];
_32 = (188064393563920708979273853091498892662_u128,);
_11 = _4;
_30 = [_4.0];
_16 = _12.2.0 != _4.2.0;
_34.fld1 = core::ptr::addr_of!(_30);
Call(_5 = core::intrinsics::bswap((-13_i8)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12.1 = _11.1 * _8;
_4.3 = _12.3;
_33 = _8 as isize;
_11.2 = (_19.fld1.2.0,);
_39 = '\u{a582}';
_19.fld1.1 = -_8;
_1 = _5 as u64;
_23 = Adt51 { fld0: _27.fld0 };
_4.2.0 = _19.fld1.2.0 ^ _19.fld1.2.0;
_15 = _9;
_4 = _12;
_12.0 = _4.0;
_26 = _33;
_12 = _4;
_4.2.0 = _32.0 as usize;
_25 = _32.0;
_10 = _12.2.0 / 15336710489848558702_usize;
Call(_4.1 = fn14(_13, _32, _7, _19.fld1.0, _19, _11.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_32.0 = _25;
_15 = _32.0 as u32;
_12.2.0 = _23.fld0 as usize;
_12.0 = _11.0 / 200_u8;
_28 = _39;
_38.fld0.fld1 = core::ptr::addr_of!(_30);
_36 = (_23.fld0, _38.fld0.fld1);
_4 = _11;
_32 = (_25,);
_30 = [_11.0];
_42 = _39;
_19.fld1.3 = core::ptr::addr_of_mut!(_15);
_19.fld1.1 = _12.1 - _12.1;
_19.fld1 = (_11.0, _14, _4.2, _11.3);
_32.0 = _12.0 as u128;
_38.fld0.fld0 = [_5,_5,_5,_5];
_21.0 = !_1;
_10 = _2.0;
_34 = Adt47 { fld0: _3,fld1: _38.fld0.fld1 };
_15 = _9;
_4.0 = !_19.fld1.0;
match _25 {
0 => bb4,
1 => bb3,
188064393563920708979273853091498892662 => bb11,
_ => bb10
}
}
bb10 = {
_11.1 = _12.1;
_11.2 = (_12.2.0,);
_1 = _21.0 | _21.0;
(*_13) = _15 << _4.2.0;
RET = (_21.0,);
_1 = (-1984690092793915448_i64) as u64;
_11.1 = 296433921921896905285038868183750568360_u128 as f32;
_10 = !_11.2.0;
_19.fld1.2 = (_11.2.0,);
_21 = (_1,);
(*_13) = !_15;
_4.2 = (_19.fld1.2.0,);
_13 = _4.3;
RET.0 = 17590_i16 as u64;
Goto(bb4)
}
bb11 = {
_5 = (-12_i8);
_7 = _27.fld0 as usize;
_44 = _14 - _4.1;
_4.2 = (_11.2.0,);
_2.0 = _19.fld1.2.0;
_3 = _34.fld0;
_27.fld0 = _36.0;
_12.3 = _11.3;
_37 = !_16;
_37 = _16;
_33 = _26;
_17.0 = _19.fld1.2.0 as isize;
_24 = _8 as isize;
_9 = _15;
_37 = _16;
_19.fld0 = [75455105036371961951846082630988326490_i128];
_48 = !_17.0;
_12 = (_11.0, _44, _19.fld1.2, _13);
_4.1 = _12.1 / 0.000000000000000000000000000000000000007889971767023881_f32;
_50.1 = _15 / 2745987440_u32;
_48 = _17.0 & _17.0;
Goto(bb12)
}
bb12 = {
_19.fld2 = [_48,_48,_48,_17.0,_17.0,_48];
_2.0 = _21.0 as usize;
_33 = (-3864313857617157660_i64) as isize;
_12 = _11;
_12.1 = -_4.1;
_25 = _32.0 - _32.0;
_50 = (5697635931809175075_i64, _15);
_33 = _48;
_35 = [_1,_21.0,_21.0,_1];
_12 = (_19.fld1.0, _18, _19.fld1.2, _4.3);
_38.fld1 = _1 as u32;
Goto(bb13)
}
bb13 = {
_2.0 = _11.2.0;
_43 = _48;
_11.3 = core::ptr::addr_of_mut!(_50.1);
_51.0 = _25 | _25;
_53 = (_33, _17.1);
_19.fld1 = (_4.0, _4.1, _11.2, _12.3);
_51.0 = _43 as u128;
_19.fld0 = [126583018722193442127765770020609182811_i128];
_32.0 = !_51.0;
(*_13) = _32.0 as u32;
_22 = _53.0 & _53.0;
match _31 {
0 => bb9,
1 => bb11,
2 => bb14,
3 => bb15,
340282366920938463463374607431768190171 => bb17,
_ => bb16
}
}
bb14 = {
_12.0 = !_11.0;
_4.2.0 = _2.0 * _10;
_12.3 = _13;
_18 = 3316_i16 as f32;
_7 = _10 & _10;
_2.0 = !_11.2.0;
_21.0 = 28218_i16 as u64;
_4.0 = _15 as u8;
_4.2.0 = _7;
_14 = -_11.1;
_7 = _4.2.0 / 12065720125660610496_usize;
_12.2.0 = !_2.0;
_19.fld1.3 = core::ptr::addr_of_mut!(_15);
_24 = (-2_isize) + (-9223372036854775808_isize);
_13 = core::ptr::addr_of_mut!(_9);
_12.1 = _11.1 / f32::NAN;
_19.fld2 = [_24,_24,_24,_24,_24,_24];
_26 = _24;
Call(_11 = fn13(_26, _12.3, _4, _12, _6, _19, _10, _19.fld2, _19.fld1, _4.2, _12, _19.fld1.2, _19.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_5 = (-12_i8);
_7 = _27.fld0 as usize;
_44 = _14 - _4.1;
_4.2 = (_11.2.0,);
_2.0 = _19.fld1.2.0;
_3 = _34.fld0;
_27.fld0 = _36.0;
_12.3 = _11.3;
_37 = !_16;
_37 = _16;
_33 = _26;
_17.0 = _19.fld1.2.0 as isize;
_24 = _8 as isize;
_9 = _15;
_37 = _16;
_19.fld0 = [75455105036371961951846082630988326490_i128];
_48 = !_17.0;
_12 = (_11.0, _44, _19.fld1.2, _13);
_4.1 = _12.1 / 0.000000000000000000000000000000000000007889971767023881_f32;
_50.1 = _15 / 2745987440_u32;
_48 = _17.0 & _17.0;
Goto(bb12)
}
bb16 = {
_32 = (149178628278234256874993275594030527300_u128,);
_19.fld1.2 = (_12.2.0,);
_19.fld1.3 = _12.3;
_21.0 = _1;
Call(_10 = core::intrinsics::transmute(_21.0), ReturnTo(bb6), UnwindUnreachable())
}
bb17 = {
_19.fld1.2.0 = !_11.2.0;
_49 = [_21.0,_1,_21.0,_21.0];
_38.fld2 = core::ptr::addr_of_mut!(_12.1);
_33 = -_22;
_31 = -(-12255_i16);
_19.fld2 = [_33,_43,_53.0,_48,_22,_53.0];
_54.fld1.0 = !_7;
_55 = _42;
RET = _21;
_19.fld1.2 = (_4.2.0,);
_56.fld0.fld3.0.0 = _11.2.0;
_32.0 = _25 - _25;
_56.fld0.fld5.1 = [_27.fld0,_23.fld0,_36.0,_36.0,_23.fld0];
Goto(bb18)
}
bb18 = {
Call(_60 = dump_var(12_usize, 9_usize, Move(_9), 42_usize, Move(_42), 24_usize, Move(_24), 16_usize, Move(_16)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_60 = dump_var(12_usize, 6_usize, Move(_6), 53_usize, Move(_53), 35_usize, Move(_35), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_60 = dump_var(12_usize, 43_usize, Move(_43), 37_usize, Move(_37), 31_usize, Move(_31), 28_usize, Move(_28)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_60 = dump_var(12_usize, 1_usize, Move(_1), 26_usize, Move(_26), 10_usize, Move(_10), 22_usize, Move(_22)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: *mut u32,mut _3: (u8, f32, (usize,), *mut u32),mut _4: (u8, f32, (usize,), *mut u32),mut _5: bool,mut _6: Adt50,mut _7: usize,mut _8: [isize; 6],mut _9: (u8, f32, (usize,), *mut u32),mut _10: (usize,),mut _11: (u8, f32, (usize,), *mut u32),mut _12: (usize,),mut _13: (u8, f32, (usize,), *mut u32)) -> (u8, f32, (usize,), *mut u32) {
mir! {
type RET = (u8, f32, (usize,), *mut u32);
let _14: f64;
let _15: (u64,);
let _16: Adt49;
let _17: bool;
let _18: ();
let _19: ();
{
_3.3 = core::ptr::addr_of_mut!((*_2));
RET.2 = (_6.fld1.2.0,);
_7 = _12.0;
_13.2 = (_4.2.0,);
(*_2) = !505307030_u32;
_6.fld1.2 = _3.2;
_16 = Adt49 { fld0: _8,fld1: _6.fld0,fld2: (-42646746090891853963057953723833466697_i128) };
_13.1 = _9.1;
RET.2 = _10;
RET.1 = _3.1;
_15.0 = 9519358145759529759_u64 - 12442159420154391506_u64;
_13.1 = _4.1;
_16.fld1 = [_16.fld2];
_11.3 = core::ptr::addr_of_mut!((*_2));
_17 = _9.2.0 >= _3.2.0;
_6 = Adt50 { fld0: _16.fld1,fld1: _4,fld2: _8 };
_14 = _16.fld2 as f64;
RET.1 = _9.1;
RET = _11;
_6.fld1.2 = _13.2;
RET = (_11.0, _6.fld1.1, _10, _3.3);
_4.2 = _12;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(13_usize, 17_usize, Move(_17), 10_usize, Move(_10), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *mut u32,mut _2: (u128,),mut _3: usize,mut _4: u8,mut _5: Adt50,mut _6: f32) -> f32 {
mir! {
type RET = f32;
let _7: isize;
let _8: u16;
let _9: [u64; 4];
let _10: (u8, f32, (usize,), *mut u32);
let _11: u16;
let _12: [isize; 6];
let _13: [u64; 4];
let _14: i32;
let _15: *mut (i8, (usize,), *mut f32);
let _16: isize;
let _17: isize;
let _18: f64;
let _19: isize;
let _20: Adt63;
let _21: [u8; 1];
let _22: f32;
let _23: f64;
let _24: Adt52;
let _25: bool;
let _26: bool;
let _27: [u64; 4];
let _28: ();
let _29: ();
{
(*_1) = 30100_u16 as u32;
_5.fld2 = [(-9223372036854775808_isize),9223372036854775807_isize,93_isize,(-9223372036854775808_isize),79_isize,9223372036854775807_isize];
_5.fld0 = [(-157485767146049704366607243323376025913_i128)];
_3 = 29013_i16 as usize;
_5.fld1.2.0 = _3;
RET = -_5.fld1.1;
_5.fld1.1 = -_6;
_4 = _5.fld1.0 >> (*_1);
RET = -_5.fld1.1;
_6 = _5.fld1.1 * _5.fld1.1;
_5.fld1.2 = (_3,);
_5.fld0 = [81797427823532083614097828497717321238_i128];
_5.fld1.1 = 31552_i16 as f32;
_2 = (140292487033561792009938685823564060821_u128,);
_4 = _5.fld1.0;
RET = -_6;
_4 = 9223372036854775807_isize as u8;
_6 = _5.fld1.1;
_5.fld0 = [57327981319857270356891943709687040305_i128];
Call(_4 = core::intrinsics::bswap(_5.fld1.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _5.fld1.0 / 63_u8;
_5.fld0 = [(-25469770473919852343772455499549184789_i128)];
_2.0 = !329696033986481346538071365755632064311_u128;
RET = _5.fld1.1 * _6;
_2 = (10486555319157425766284618445988241512_u128,);
_6 = _5.fld1.1;
(*_1) = 1418568127_u32;
_5.fld1.3 = core::ptr::addr_of_mut!((*_1));
_5.fld1.0 = !_4;
_5.fld2 = [43_isize,9223372036854775807_isize,(-119_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = _5.fld1.1 - _6;
_5.fld2 = [9223372036854775807_isize,(-9223372036854775808_isize),(-37_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_6 = _5.fld1.1 / (-0.000000000000000000000000000000000000010312092340535434_f32);
_5.fld1.2.0 = _3 * _3;
(*_1) = 910745929_u32 % 2930501933_u32;
_5.fld1.0 = !_4;
(*_1) = !3714371335_u32;
_5.fld0 = [47715920933895884980026429454151960951_i128];
_2.0 = 70047582747031890838424545473314231935_u128;
_5.fld1.0 = _4 - _4;
_3 = _5.fld1.2.0 | _5.fld1.2.0;
_5.fld1.3 = _1;
_4 = _5.fld1.0 >> _5.fld1.0;
_3 = _5.fld1.2.0 / 5_usize;
_5.fld0 = [163259453450482146824335812507452065538_i128];
_5.fld1.1 = -_6;
_5.fld1.2.0 = _3;
_5.fld1.1 = _6 + _6;
RET = 35548_u16 as f32;
_5.fld1.3 = core::ptr::addr_of_mut!((*_1));
_7 = (-9223372036854775808_isize) & (-30_isize);
match _2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
70047582747031890838424545473314231935 => bb8,
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
_7 = (-9223372036854775808_isize) << _4;
_3 = 37764_u16 as usize;
_5.fld1.1 = 2579392798148363988_i64 as f32;
_8 = !39072_u16;
_5.fld1.3 = core::ptr::addr_of_mut!((*_1));
_7 = _8 as isize;
_1 = _5.fld1.3;
(*_1) = 2052603271_u32 << _4;
_7 = !59_isize;
_5.fld1.3 = core::ptr::addr_of_mut!((*_1));
_5.fld1.0 = _4 ^ _4;
_5.fld2 = [_7,_7,_7,_7,_7,_7];
_5.fld2 = [_7,_7,_7,_7,_7,_7];
_5.fld1.0 = _4 + _4;
_5.fld1.1 = _6 / f32::NEG_INFINITY;
_5.fld1.1 = _6 - _6;
_2.0 = 20071513083395606622561442912806731691_u128 | 33187745831649796430855746636570511134_u128;
_3 = !_5.fld1.2.0;
Goto(bb9)
}
bb9 = {
_5.fld1.3 = _1;
_9 = [4104300614660340091_u64,8064828255775620245_u64,14862528991553657506_u64,6224860410680743760_u64];
RET = _7 as f32;
_5.fld1.0 = !_4;
(*_1) = _5.fld1.1 as u32;
RET = _6 + _5.fld1.1;
(*_1) = 152420941_i32 as u32;
_5.fld1.0 = _4;
_5.fld1.2.0 = !_3;
_5.fld1.2 = (_3,);
_10.3 = core::ptr::addr_of_mut!((*_1));
_5.fld1.2.0 = (-1466726634_i32) as usize;
_5.fld1.2.0 = (-125_i8) as usize;
RET = -_5.fld1.1;
Call((*_1) = core::intrinsics::bswap(2254590627_u32), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4 = _5.fld1.0 & _5.fld1.0;
_4 = _5.fld1.0 << _5.fld1.0;
_10 = (_4, _5.fld1.1, _5.fld1.2, _5.fld1.3);
_10.2 = _5.fld1.2;
_5.fld1.2 = (_3,);
RET = _10.1 - _6;
_2 = (307947717226769484333496612126923122418_u128,);
_5.fld1 = _10;
_10.2.0 = !_3;
(*_1) = !2839275518_u32;
_8 = (-1733738036_i32) as u16;
(*_1) = !3096783053_u32;
_5.fld1.1 = _10.1 + _10.1;
RET = _5.fld1.1;
_11 = _8 - _8;
_10 = (_4, _5.fld1.1, _5.fld1.2, _1);
_10.3 = core::ptr::addr_of_mut!((*_1));
_10.3 = core::ptr::addr_of_mut!((*_1));
_3 = _5.fld1.0 as usize;
_10.1 = _6;
_5.fld1.2 = (_3,);
_5.fld1.0 = 15028_i16 as u8;
_5.fld1.1 = (-3819_i16) as f32;
_10.0 = _4;
Goto(bb11)
}
bb11 = {
_5.fld1.1 = _3 as f32;
_8 = (-28_i8) as u16;
_9 = [5281787553658028365_u64,7230049691648986722_u64,15805405753980264908_u64,12258271670839835863_u64];
_11 = _8;
_5.fld1.1 = 2169865904303274114_i64 as f32;
_5.fld1.1 = _2.0 as f32;
_4 = _10.0;
RET = _6 / f32::INFINITY;
_10.2 = (_5.fld1.2.0,);
_4 = _7 as u8;
_10.3 = _1;
_4 = _7 as u8;
_5.fld2 = [_7,_7,_7,_7,_7,_7];
_11 = _8 / 21837_u16;
_5.fld1.3 = _1;
RET = 149233420908642996042508013121756951723_i128 as f32;
RET = _5.fld1.1 - _10.1;
Goto(bb12)
}
bb12 = {
_4 = _10.0;
_17 = _7;
_2 = (118691805481492464334498835639225958719_u128,);
_14 = (-44_i8) as i32;
_9 = [15553342055441227929_u64,1669581255045902675_u64,1251067383341085177_u64,2068914219700376208_u64];
_10.0 = _5.fld1.0 | _5.fld1.0;
_6 = _10.1;
_10.0 = !_4;
(*_1) = !1178453247_u32;
_2.0 = 115871656431493275246597892452767885224_u128;
Call(_1 = fn15(_8, (*_1), _5, _10.2.0, _10, _5.fld0, _2.0, _5, _10, _10.1, _10.3, _2.0, _11, _5.fld1.0, _10.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_19 = _7 | _17;
_16 = _7 >> _4;
RET = -_10.1;
_21 = [_4];
_13 = [13642405516162691885_u64,12743424449707554437_u64,3568890181058887352_u64,2624227279227396653_u64];
RET = _10.1 - _6;
_10 = (_4, _6, _5.fld1.2, _1);
_20.fld0.fld0.0 = _10.0;
_3 = _10.2.0 | _5.fld1.2.0;
_5.fld1 = (_10.0, _6, _10.2, _10.3);
_12 = _5.fld2;
_4 = _5.fld1.0;
_7 = _16;
_19 = _7 & _16;
_5.fld1.0 = _10.0;
RET = (-113420176425611575613648072967812607082_i128) as f32;
_3 = !_5.fld1.2.0;
_20.fld1 = _12;
Goto(bb14)
}
bb14 = {
_23 = (*_1) as f64;
_11 = _8;
RET = _6 - _6;
_20.fld0.fld0.0 = 7971316549310080582_i64 as u8;
_9 = [1908131597966340468_u64,8519831400604899812_u64,12791223901536084566_u64,654339907797003820_u64];
_7 = !_19;
(*_1) = 8839083353597654031_i64 as u32;
_5.fld1.2 = (_10.2.0,);
_20.fld0.fld0.2 = (_5.fld1.2.0,);
_17 = -_7;
(*_1) = !2629678547_u32;
_20.fld0.fld0 = _5.fld1;
_20.fld0.fld0.0 = (-25_i8) as u8;
RET = _16 as f32;
_24.fld0.2.0 = !_5.fld1.2.0;
_25 = true;
_5.fld1 = (_10.0, _6, _20.fld0.fld0.2, _20.fld0.fld0.3);
_20.fld1 = _12;
_5.fld1 = (_4, _20.fld0.fld0.1, _10.2, _1);
_24.fld0 = (_10.0, _5.fld1.1, _5.fld1.2, _1);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(14_usize, 16_usize, Move(_16), 17_usize, Move(_17), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(14_usize, 11_usize, Move(_11), 14_usize, Move(_14), 3_usize, Move(_3), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: u16,mut _2: u32,mut _3: Adt50,mut _4: usize,mut _5: (u8, f32, (usize,), *mut u32),mut _6: [i128; 1],mut _7: u128,mut _8: Adt50,mut _9: (u8, f32, (usize,), *mut u32),mut _10: f32,mut _11: *mut u32,mut _12: u128,mut _13: u16,mut _14: u8,mut _15: f32) -> *mut u32 {
mir! {
type RET = *mut u32;
let _16: char;
let _17: Adt58;
let _18: u128;
let _19: char;
let _20: isize;
let _21: f64;
let _22: *mut char;
let _23: isize;
let _24: (isize, [i32; 5]);
let _25: isize;
let _26: ((u64,), f64);
let _27: char;
let _28: u128;
let _29: Adt57;
let _30: isize;
let _31: [isize; 6];
let _32: *mut f64;
let _33: Adt62;
let _34: ();
let _35: ();
{
_7 = _12;
_7 = !_12;
_14 = !_5.0;
_8.fld1.2 = _9.2;
_1 = _13 * _13;
_13 = _1;
_9.1 = -_5.1;
_8.fld1.1 = _10 / (-0.000000000000000000000000000000000000007263610971649189_f32);
_5 = (_9.0, _8.fld1.1, _3.fld1.2, _11);
Goto(bb1)
}
bb1 = {
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
115871656431493275246597892452767885224 => bb9,
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
_3.fld1.1 = _10 - _15;
_1 = 747142595_i32 as u16;
RET = core::ptr::addr_of_mut!((*_11));
_18 = _12 / 302958251081574454600254694288044958671_u128;
_8.fld1.0 = _9.0;
_17.fld3 = core::ptr::addr_of_mut!(_17.fld1);
_17.fld6.fld0 = _8.fld2;
_9.3 = _11;
_3.fld1.1 = -_15;
_17.fld1.2 = core::ptr::addr_of_mut!(_3.fld1.1);
Goto(bb10)
}
bb10 = {
_9.2.0 = !_8.fld1.2.0;
_8.fld0 = _3.fld0;
_5.1 = _4 as f32;
_5.0 = !_14;
_6 = [124929251154387085450317820906601696173_i128];
_8.fld1.2.0 = _3.fld1.2.0;
_8.fld1.1 = _15 - _5.1;
_17.fld1.2 = core::ptr::addr_of_mut!(_10);
match _12 {
115871656431493275246597892452767885224 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_10 = _8.fld1.1 / f32::NAN;
_14 = _8.fld1.0 * _5.0;
_5.1 = -_8.fld1.1;
_17.fld2 = _7 as isize;
_5.1 = _8.fld1.1 + _8.fld1.1;
_17.fld6 = Adt49 { fld0: _8.fld2,fld1: _3.fld0,fld2: (-137291544068183806086623931359061066779_i128) };
_17.fld4 = core::ptr::addr_of!(_15);
_5.3 = core::ptr::addr_of_mut!(_2);
(*_11) = true as u32;
_20 = _17.fld2 ^ _17.fld2;
_17.fld1.0 = !(-28_i8);
RET = _9.3;
_3.fld1.0 = _14 & _14;
_17.fld6.fld2 = 108667550151581375917238272924851316478_i128 + (-91318992971966270758565055093701635371_i128);
_10 = _5.1 / 0.000000000000000000000000000000000000007804121216607022_f32;
_15 = _8.fld1.1 * _5.1;
(*_11) = _2;
(*RET) = !_2;
_17.fld6 = Adt49 { fld0: _8.fld2,fld1: _6,fld2: 154361112228782523459475173073274272636_i128 };
_19 = '\u{be5b0}';
_3.fld0 = _8.fld0;
_1 = !_13;
_3.fld1.2.0 = !_4;
Goto(bb13)
}
bb13 = {
_21 = _8.fld1.2.0 as f64;
_21 = _7 as f64;
_9 = (_3.fld1.0, _15, _5.2, _11);
_17.fld1.0 = _20 as i8;
_9.2 = (_3.fld1.2.0,);
_9.1 = _20 as f32;
_19 = '\u{3494a}';
_9.2 = (_4,);
_9.0 = _3.fld1.0;
_9.2 = (_4,);
_1 = _13;
_3.fld2 = [_20,_20,_17.fld2,_20,_20,_20];
_9.3 = core::ptr::addr_of_mut!((*RET));
_3 = Adt50 { fld0: _6,fld1: _5,fld2: _8.fld2 };
_9.1 = _10 / (-0.0000000000000000000000000000000000000004347262238859523_f32);
_17.fld7 = core::ptr::addr_of_mut!(_21);
_7 = _18 - _18;
_17.fld1.1.0 = _8.fld1.2.0;
_17.fld1.1 = _3.fld1.2;
(*_11) = _2;
_3 = Adt50 { fld0: _17.fld6.fld1,fld1: _9,fld2: _8.fld2 };
_24.1 = [(-1209382520_i32),2028854815_i32,(-1669777494_i32),185754535_i32,1417154955_i32];
_17.fld1.1 = _8.fld1.2;
_9.2 = (_5.2.0,);
Goto(bb14)
}
bb14 = {
_16 = _19;
(*RET) = _2 / 2857322495_u32;
_17.fld0 = false;
_3.fld1.3 = core::ptr::addr_of_mut!((*RET));
_25 = _20 ^ _17.fld2;
_9.2 = (_5.2.0,);
_26.0 = (322945957214208775_u64,);
_29.fld0 = [_17.fld1.0,_17.fld1.0,_17.fld1.0,_17.fld1.0];
_9.2.0 = _16 as usize;
(*RET) = !_2;
_3.fld1.2.0 = _4 >> _8.fld1.0;
_5.2 = (_3.fld1.2.0,);
_32 = core::ptr::addr_of_mut!(_26.1);
_24.0 = _20;
_27 = _16;
_8.fld0 = [_17.fld6.fld2];
_29.fld3 = !925962632714210814_i64;
_28 = _7;
_33.fld2.fld4.fld0.0.2 = core::ptr::addr_of_mut!(_5.1);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(15_usize, 19_usize, Move(_19), 27_usize, Move(_27), 12_usize, Move(_12), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(15_usize, 7_usize, Move(_7), 25_usize, Move(_25), 16_usize, Move(_16), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [isize; 6],mut _2: f32,mut _3: [i32; 5],mut _4: [i32; 5]) -> ((u64,), *mut f32) {
mir! {
type RET = ((u64,), *mut f32);
let _5: *mut f32;
let _6: char;
let _7: i64;
let _8: *mut f64;
let _9: [i32; 5];
let _10: i128;
let _11: isize;
let _12: i32;
let _13: [i32; 5];
let _14: u8;
let _15: [i32; 5];
let _16: Adt47;
let _17: i128;
let _18: [isize; 5];
let _19: bool;
let _20: Adt55;
let _21: ();
let _22: ();
{
RET.0.0 = 1213848638418427203_u64;
RET.0.0 = 8329969462724094890_u64;
RET.1 = core::ptr::addr_of_mut!(_2);
_1 = [9223372036854775807_isize,(-24_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET.0.0 = !8450014430410647047_u64;
RET.0.0 = 6679280157363418177_i64 as u64;
RET.1 = core::ptr::addr_of_mut!(_2);
_3 = _4;
RET.0.0 = !5013337581433983936_u64;
_2 = 107_u8 as f32;
_4 = [(-1187809565_i32),(-394007867_i32),105401044_i32,(-728812324_i32),1904038179_i32];
_5 = core::ptr::addr_of_mut!(_2);
_7 = 6493789706105329819_usize as i64;
_5 = core::ptr::addr_of_mut!(_2);
RET.1 = core::ptr::addr_of_mut!((*_5));
_7 = 6190497065701846029766324285747146911_i128 as i64;
RET.0 = (699038773839040609_u64,);
RET.0.0 = 12056408726793202599_u64 << _7;
_3 = [304901505_i32,(-1317817959_i32),(-357007441_i32),169866252_i32,124312618_i32];
RET.1 = _5;
(*_5) = 8850_u16 as f32;
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_10 = 117431058963090231398507118847902433719_i128;
_7 = !(-3624321257354552670_i64);
_5 = core::ptr::addr_of_mut!((*_5));
_4 = _3;
RET.0.0 = !4584976611982211796_u64;
_2 = (-112_i8) as f32;
Goto(bb3)
}
bb3 = {
(*_5) = (-47_isize) as f32;
(*_5) = 55_i8 as f32;
_3 = [154348271_i32,(-460801152_i32),(-1718683849_i32),(-681043332_i32),(-735216313_i32)];
_9 = [1565376758_i32,248921110_i32,(-429518646_i32),(-2120672889_i32),(-107218170_i32)];
(*_5) = _7 as f32;
RET.0 = (1755240073190514380_u64,);
_6 = '\u{7f8da}';
RET.1 = _5;
_6 = '\u{7f5f5}';
match _10 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
117431058963090231398507118847902433719 => bb9,
_ => bb8
}
}
bb4 = {
_10 = 117431058963090231398507118847902433719_i128;
_7 = !(-3624321257354552670_i64);
_5 = core::ptr::addr_of_mut!((*_5));
_4 = _3;
RET.0.0 = !4584976611982211796_u64;
_2 = (-112_i8) as f32;
Goto(bb3)
}
bb5 = {
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
_3 = _4;
_11 = 9223372036854775807_isize + (-9223372036854775808_isize);
_3 = [(-1210171163_i32),852699485_i32,714811013_i32,382277034_i32,(-822459186_i32)];
_1 = [_11,_11,_11,_11,_11,_11];
_9 = [(-1093973792_i32),912289260_i32,1245393485_i32,(-716518154_i32),(-1082123798_i32)];
_13 = [327475205_i32,897909948_i32,572949678_i32,603397466_i32,1691989940_i32];
_13 = [1733963635_i32,2113029885_i32,(-1626486253_i32),1335608071_i32,(-1349243696_i32)];
_1 = [_11,_11,_11,_11,_11,_11];
RET.0.0 = !3387342528079729488_u64;
_1 = [_11,_11,_11,_11,_11,_11];
_7 = (-4976042205196190853_i64);
RET.0 = (10780615520350631146_u64,);
_13 = _9;
_2 = 14135243946710896986_u64 as f32;
RET.1 = _5;
RET.0 = (4309669487789255988_u64,);
_4 = _13;
RET.1 = core::ptr::addr_of_mut!((*_5));
Goto(bb10)
}
bb10 = {
RET.0.0 = 12231127646038238710_u64;
_9 = [144528251_i32,2095556432_i32,1979623262_i32,254001764_i32,674176943_i32];
Goto(bb11)
}
bb11 = {
(*_5) = 39_u8 as f32;
_2 = _10 as f32;
_14 = 121_u8;
RET.1 = core::ptr::addr_of_mut!((*_5));
_7 = 5579949111142419077_i64 & 8470977927315190672_i64;
_12 = 1411037958_u32 as i32;
RET.1 = core::ptr::addr_of_mut!(_2);
(*_5) = 420_u16 as f32;
_7 = -4235452793965154709_i64;
_3 = [_12,_12,_12,_12,_12];
match _10 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
117431058963090231398507118847902433719 => bb17,
_ => bb16
}
}
bb12 = {
RET.0.0 = 12231127646038238710_u64;
_9 = [144528251_i32,2095556432_i32,1979623262_i32,254001764_i32,674176943_i32];
Goto(bb11)
}
bb13 = {
_3 = _4;
_11 = 9223372036854775807_isize + (-9223372036854775808_isize);
_3 = [(-1210171163_i32),852699485_i32,714811013_i32,382277034_i32,(-822459186_i32)];
_1 = [_11,_11,_11,_11,_11,_11];
_9 = [(-1093973792_i32),912289260_i32,1245393485_i32,(-716518154_i32),(-1082123798_i32)];
_13 = [327475205_i32,897909948_i32,572949678_i32,603397466_i32,1691989940_i32];
_13 = [1733963635_i32,2113029885_i32,(-1626486253_i32),1335608071_i32,(-1349243696_i32)];
_1 = [_11,_11,_11,_11,_11,_11];
RET.0.0 = !3387342528079729488_u64;
_1 = [_11,_11,_11,_11,_11,_11];
_7 = (-4976042205196190853_i64);
RET.0 = (10780615520350631146_u64,);
_13 = _9;
_2 = 14135243946710896986_u64 as f32;
RET.1 = _5;
RET.0 = (4309669487789255988_u64,);
_4 = _13;
RET.1 = core::ptr::addr_of_mut!((*_5));
Goto(bb10)
}
bb14 = {
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_10 = 117431058963090231398507118847902433719_i128;
_7 = !(-3624321257354552670_i64);
_5 = core::ptr::addr_of_mut!((*_5));
_4 = _3;
RET.0.0 = !4584976611982211796_u64;
_2 = (-112_i8) as f32;
Goto(bb3)
}
bb17 = {
_7 = 7555988772385994046_i64;
_10 = !(-2535286692025119324784664758157214915_i128);
_2 = _10 as f32;
_12 = -(-1378485489_i32);
_12 = (-1942341410_i32);
RET.0 = (8542649710325620570_u64,);
_11 = !9223372036854775807_isize;
_3 = [_12,_12,_12,_12,_12];
_15 = _9;
_16.fld0 = [(-113_i8),34_i8,1_i8,(-25_i8)];
_1 = [_11,_11,_11,_11,_11,_11];
_2 = _11 as f32;
RET.0.0 = true as u64;
(*_5) = 15473081165505500701_usize as f32;
RET.0.0 = 4975391955024947580_u64 >> _10;
_16.fld0 = [73_i8,16_i8,(-122_i8),(-10_i8)];
_11 = 4118265132229598717_usize as isize;
_10 = (-5300253099636198922877805198995132427_i128);
_20.fld0.0.1.0 = !8819837616103990436_usize;
_20.fld0.2 = [_11,_11,_11,_11,_11,_11];
_20.fld0.0.2 = core::ptr::addr_of_mut!(_2);
_20.fld2 = [_10];
_17 = _12 as i128;
_20.fld3.0 = (_20.fld0.0.1.0,);
_19 = !true;
Goto(bb18)
}
bb18 = {
Call(_21 = dump_var(16_usize, 15_usize, Move(_15), 17_usize, Move(_17), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_21 = dump_var(16_usize, 6_usize, Move(_6), 12_usize, Move(_12), 19_usize, Move(_19), 22_usize, _22), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                println!("{:?}", fn0(std::hint::black_box(true), std::hint::black_box(2908171531_u32), std::hint::black_box((-43_isize)), std::hint::black_box(91_i8), std::hint::black_box((-28624_i16)), std::hint::black_box(1070431833_i32), std::hint::black_box((-2101831518285826115_i64)), std::hint::black_box((-78395991777493099074173862746876413833_i128)), std::hint::black_box(14634500524755381828_usize), std::hint::black_box(106_u8), std::hint::black_box(61041_u16)));
                
            }
#[derive(Debug,Copy,Clone)]
pub struct Adt47 {
fld0: [i8; 4],
fld1: *const [u8; 1],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt48 {
fld0: bool,
fld1: (u64,),
fld2: [u64; 4],
fld3: ((usize,), (u64,), bool, *const usize),
fld4: u16,
fld5: (isize, [i32; 5]),
fld6: (u8, f32, (usize,), *mut u32),
}
#[derive(Debug)]
pub struct Adt49 {
fld0: [isize; 6],
fld1: [i128; 1],
fld2: i128,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt50 {
fld0: [i128; 1],
fld1: (u8, f32, (usize,), *mut u32),
fld2: [isize; 6],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt51 {
fld0: i32,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt52 {
fld0: (u8, f32, (usize,), *mut u32),
fld1: (u128,),
}
#[derive(Debug)]
pub struct Adt53 {
fld0: *const *const usize,
fld1: Adt48,
fld2: (i64, u32),
fld3: *const usize,
fld4: Adt50,
fld5: (i32, *const [u8; 1]),
fld6: Adt51,
fld7: (u8, f32, (usize,), *mut u32),
}
#[derive(Debug)]
pub struct Adt54 {
fld0: Adt47,
fld1: u32,
fld2: *mut f32,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt55 {
fld0: ((i8, (usize,), *mut f32), *mut f64, [isize; 6]),
fld1: (u64,),
fld2: [i128; 1],
fld3: ((usize,), (u64,), bool, *const usize),
fld4: *const [u8; 1],
fld5: u128,
fld6: ((u64,), *mut f32),
fld7: (isize, [i32; 5]),
}
#[derive(Debug)]
pub struct Adt56 {
fld0: u8,
fld1: Adt49,
fld2: [i128; 1],
fld3: Adt50,
fld4: Adt55,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt57 {
fld0: [i8; 4],
fld1: (usize,),
fld2: *mut *const f32,
fld3: i64,
fld4: *mut f32,
fld5: u16,
}
#[derive(Debug)]
pub struct Adt58 {
fld0: bool,
fld1: (i8, (usize,), *mut f32),
fld2: isize,
fld3: *mut (i8, (usize,), *mut f32),
fld4: *const f32,
fld5: [isize; 6],
fld6: Adt49,
fld7: *mut f64,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt59 {
fld0: [u8; 1],
fld1: char,
fld2: isize,
fld3: i8,
fld4: *const usize,
fld5: i32,
fld6: (u128,),
fld7: Adt52,
}
#[derive(Debug)]
pub struct Adt60 {
fld0: Adt48,
}
#[derive(Debug)]
pub struct Adt61 {
fld0: *mut *const f32,
fld1: (u64,),
fld2: Adt53,
fld3: [u64; 4],
fld4: [isize; 6],
}
#[derive(Debug)]
pub struct Adt62 {
fld0: [u64; 4],
fld1: Adt58,
fld2: Adt56,
fld3: *mut f32,
fld4: *mut *const f32,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt63 {
fld0: Adt52,
fld1: [isize; 6],
fld2: *mut char,
}

