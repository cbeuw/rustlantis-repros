#![recursion_limit = "256"]
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
pub fn fn0(mut _1: u128,mut _2: u64,mut _3: isize,mut _4: i8,mut _5: i16) -> isize {
mir! {
type RET = isize;
let _6: isize;
let _7: isize;
let _8: Adt61;
let _9: Adt60;
let _10: [i32; 7];
let _11: *const f32;
let _12: Adt55;
let _13: isize;
let _14: [i8; 3];
let _15: isize;
let _16: isize;
let _17: u32;
let _18: isize;
let _19: isize;
let _20: [u32; 1];
let _21: i8;
let _22: Adt51;
let _23: (i32,);
let _24: (*mut (i16, *mut i128, [i8; 3]), i8, i64, usize);
let _25: (char,);
let _26: (*mut (i16, *mut i128, [i8; 3]), i8, i64, usize);
let _27: isize;
let _28: ();
let _29: ();
{
_6 = !9223372036854775807_isize;
RET = _6 ^ _6;
_1 = 322154412346885722749806764074711436955_u128 % 201816645216594552293722023013461025147_u128;
_8.fld4.fld1.2 = [39_i8,(-81_i8),116_i8];
_8.fld0 = (_8.fld4.fld1.2, 68009148_i32, (-1321333560141157984034315431549211057_i128));
_9.fld3.fld0.1 = core::ptr::addr_of_mut!(_8.fld0.2);
_5 = 23127_i16;
_9.fld1.fld0.0 = !_6;
_12.fld0.fld6.fld1.3 = core::ptr::addr_of!(_9.fld3.fld0.1);
_8.fld0 = (_8.fld4.fld1.2, 1780840500_i32, 79016378375026185238791835265206827157_i128);
_12.fld0.fld6.fld1.0 = 14723878741467795902_u64 + 190141771495025101_u64;
_8.fld2.fld0 = (-823397298666204712_i64);
_12.fld0.fld3.1 = [_8.fld0.1,_8.fld0.1,_8.fld0.1,_8.fld0.1,_8.fld0.1,_8.fld0.1,_8.fld0.1];
_5 = !15719_i16;
_14 = [115_i8,50_i8,21_i8];
_9.fld1.fld1.fld1 = _8.fld0;
_8.fld4.fld1.0 = _12.fld0.fld6.fld1.0;
match _8.fld0.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
79016378375026185238791835265206827157 => bb8,
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
_14 = [(-10_i8),(-14_i8),(-44_i8)];
_9.fld3.fld0.2 = [29_i8,(-70_i8),(-50_i8)];
_9.fld1.fld1.fld0 = _8.fld0.1 as f32;
_12.fld0.fld6.fld1.2 = [(-64_i8),50_i8,(-16_i8)];
_12.fld0.fld2.fld1 = '\u{1db6e}';
_8.fld4.fld1 = (_12.fld0.fld6.fld1.0, _12.fld0.fld6.fld1.3, _9.fld3.fld0.2, _12.fld0.fld6.fld1.3);
_9.fld1.fld0.6 = _5;
Call(_9.fld1.fld0.4 = fn1(_12.fld0.fld6.fld1.0, _8.fld0.2, _12.fld0.fld2.fld1, _8.fld0.1, _8.fld0.2, _12.fld0.fld6.fld1.3, _12.fld0.fld6.fld1.3, _8.fld4.fld1.1, _12.fld0.fld6.fld1.3, _8.fld0.1, _14, _9.fld3.fld0.1, _12.fld0.fld6.fld1.3, _9.fld1.fld1.fld1.2, _9.fld1.fld1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_9.fld1.fld0.2 = core::ptr::addr_of_mut!(_9.fld1.fld1.fld1.2);
_12.fld0.fld2.fld2 = core::ptr::addr_of!(_9.fld1.fld0.2);
_8.fld4.fld1.3 = core::ptr::addr_of!(_9.fld3.fld0.1);
_12.fld1.fld1.0 = _9.fld1.fld1.fld1.1;
_3 = !_6;
_9.fld3.fld0 = (_9.fld1.fld0.6, _9.fld1.fld0.2, _8.fld4.fld1.2);
_12.fld0.fld2.fld2 = core::ptr::addr_of!(_9.fld1.fld0.2);
_2 = _8.fld4.fld1.0;
_9.fld1.fld0.6 = _8.fld2.fld0 as i16;
RET = -_6;
_12.fld0.fld0 = core::ptr::addr_of_mut!(_9.fld3.fld0);
_4 = !_9.fld1.fld0.4;
_9.fld1.fld0.1.0 = !_12.fld1.fld1.0;
_8.fld0.2 = !_9.fld1.fld1.fld1.2;
_12.fld0.fld6.fld1 = _8.fld4.fld1;
Call(_8.fld1 = core::intrinsics::transmute(_12.fld1.fld1.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_8.fld2 = Adt50 { fld0: 3635318605957366245_i64,fld1: _12.fld0.fld2.fld1,fld2: _12.fld0.fld2.fld2 };
_11 = core::ptr::addr_of!(_9.fld1.fld1.fld0);
_16 = _3;
_9.fld3.fld1 = [_9.fld1.fld0.4,_4,_9.fld1.fld0.4];
_9.fld2 = [_9.fld1.fld0.1.0,_12.fld1.fld1.0,_12.fld1.fld1.0,_8.fld0.1,_8.fld0.1,_8.fld0.1,_9.fld1.fld1.fld1.1];
_7 = -_9.fld1.fld0.0;
_8.fld0.1 = _9.fld1.fld0.1.0 + _12.fld1.fld1.0;
_12.fld1.fld1 = (_9.fld1.fld1.fld1.1,);
_8.fld4.fld2 = core::ptr::addr_of_mut!(_9.fld1.fld0);
_12.fld0.fld7 = _12.fld0.fld3.1;
_12.fld0.fld1 = [237_u8,68_u8,192_u8,50_u8];
_8.fld2.fld0 = 50_u8 as i64;
_12.fld0.fld4 = _9.fld3.fld0.0 & _9.fld3.fld0.0;
_17 = !1942977566_u32;
_9.fld3.fld0.2 = _12.fld0.fld6.fld1.2;
_9.fld2 = [_9.fld1.fld0.1.0,_9.fld1.fld0.1.0,_9.fld1.fld1.fld1.1,_9.fld1.fld0.1.0,_8.fld0.1,_8.fld0.1,_8.fld0.1];
_12.fld1.fld0 = (_8.fld4.fld2, _12.fld0.fld7);
_12.fld0.fld2.fld0 = !_8.fld2.fld0;
_9.fld3.fld1 = _14;
_9.fld1.fld0.6 = -_9.fld3.fld0.0;
_8.fld4.fld1.0 = !_2;
_8.fld0.1 = _12.fld1.fld1.0;
_12.fld0.fld6 = Adt51 { fld0: _8.fld1,fld1: _8.fld4.fld1,fld2: _8.fld4.fld2 };
_7 = _12.fld1.fld1.0 as isize;
_9.fld1.fld0 = (_16, _12.fld1.fld1, _9.fld3.fld0.1, _4, _4, _9.fld3.fld0.1, _9.fld3.fld0.0);
_9.fld1.fld0 = (_7, _12.fld1.fld1, _9.fld3.fld0.1, _4, _4, _9.fld3.fld0.1, _12.fld0.fld4);
_12.fld0.fld1 = [75_u8,236_u8,190_u8,166_u8];
_12.fld1.fld0 = (_12.fld0.fld6.fld2, _9.fld2);
Goto(bb11)
}
bb11 = {
RET = _16 ^ _9.fld1.fld0.0;
_18 = _7 ^ _16;
_9.fld1.fld3 = _8.fld2.fld0;
_12.fld1.fld2 = _12.fld1.fld1.0 as u32;
_9.fld1.fld0.1.0 = !_9.fld1.fld1.fld1.1;
_9.fld1.fld1.fld1.2 = _8.fld4.fld1.0 as i128;
_12.fld0.fld0 = core::ptr::addr_of_mut!(_9.fld3.fld0);
_9.fld1.fld1.fld0 = 55002_u16 as f32;
_12.fld0.fld6.fld1 = _8.fld4.fld1;
_7 = -_9.fld1.fld0.0;
match _9.fld1.fld1.fld1.1 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb10,
1780840500 => bb12,
_ => bb7
}
}
bb12 = {
_8.fld4 = _12.fld0.fld6;
_20 = [_12.fld1.fld2];
_12.fld0.fld3.1 = [_9.fld1.fld1.fld1.1,_9.fld1.fld1.fld1.1,_8.fld0.1,_8.fld0.1,_8.fld0.1,_9.fld1.fld1.fld1.1,_9.fld1.fld0.1.0];
_8.fld2 = Move(_12.fld0.fld2);
_24.0 = core::ptr::addr_of_mut!(_9.fld3.fld0);
_22.fld1.1 = core::ptr::addr_of!(_9.fld3.fld0.1);
_26.3 = 1_usize;
_22 = Adt51 { fld0: _8.fld1,fld1: _8.fld4.fld1,fld2: _12.fld0.fld6.fld2 };
_8.fld1 = [_17];
_1 = 64065960025314705230514127896903989151_u128 * 37356760315266640322495972764915947639_u128;
_8.fld3.0 = core::ptr::addr_of_mut!(_26.2);
_24.3 = _26.3 % 6751404419384804612_usize;
_21 = _8.fld2.fld1 as i8;
_12.fld0.fld7 = [_9.fld1.fld0.1.0,_8.fld0.1,_9.fld1.fld1.fld1.1,_9.fld1.fld0.1.0,_12.fld1.fld1.0,_12.fld1.fld1.0,_9.fld1.fld0.1.0];
_25 = (_8.fld2.fld1,);
_12.fld1.fld0.1 = [_8.fld0.1,_8.fld0.1,_8.fld0.1,_12.fld1.fld1.0,_9.fld1.fld1.fld1.1,_9.fld1.fld0.1.0,_12.fld1.fld1.0];
_24 = (_12.fld0.fld0, _9.fld1.fld0.3, _9.fld1.fld3, _26.3);
_8.fld4.fld1.3 = _8.fld2.fld2;
_9.fld1.fld3 = _8.fld0.2 as i64;
_12.fld0.fld6.fld1.1 = core::ptr::addr_of!(_9.fld1.fld0.5);
_12.fld0.fld5 = !_1;
_3 = !_18;
Goto(bb13)
}
bb13 = {
_9.fld1.fld3 = _8.fld2.fld0;
_23.0 = -_9.fld1.fld0.1.0;
_8.fld0.2 = _9.fld1.fld1.fld1.2 << _12.fld1.fld1.0;
_10 = _9.fld2;
(*_11) = _12.fld1.fld2 as f32;
_22.fld0 = [_12.fld1.fld2];
_8.fld2.fld2 = core::ptr::addr_of!(_9.fld1.fld0.2);
_27 = -_3;
_15 = !_18;
_9.fld1.fld0.5 = _9.fld3.fld0.1;
_10 = _9.fld2;
_25 = (_8.fld2.fld1,);
_9.fld3.fld1 = [_9.fld1.fld0.4,_4,_24.1];
Goto(bb14)
}
bb14 = {
_25.0 = _8.fld2.fld1;
_8.fld1 = _22.fld0;
_12.fld0.fld5 = !_1;
_10 = _9.fld2;
_23 = (_9.fld1.fld0.1.0,);
_24.2 = !_8.fld2.fld0;
_25 = (_8.fld2.fld1,);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(0_usize, 4_usize, Move(_4), 20_usize, Move(_20), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(0_usize, 14_usize, Move(_14), 27_usize, Move(_27), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(0_usize, 2_usize, Move(_2), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u64,mut _2: i128,mut _3: char,mut _4: i32,mut _5: i128,mut _6: *const *mut i128,mut _7: *const *mut i128,mut _8: *const *mut i128,mut _9: *const *mut i128,mut _10: i32,mut _11: [i8; 3],mut _12: *mut i128,mut _13: *const *mut i128,mut _14: i128,mut _15: Adt49) -> i8 {
mir! {
type RET = i8;
let _16: isize;
let _17: ([i8; 3], i32, i128);
let _18: [i128; 3];
let _19: i128;
let _20: isize;
let _21: bool;
let _22: Adt65;
let _23: ([i8; 3], i32, i128);
let _24: i128;
let _25: Adt58;
let _26: (i32,);
let _27: [i8; 3];
let _28: usize;
let _29: i16;
let _30: ();
let _31: ();
{
RET = (-9223372036854775808_isize) as i8;
_1 = 18066002191548804837_u64;
(*_12) = -_14;
_7 = core::ptr::addr_of!((*_6));
_15.fld1.0 = [(-121_i8),17_i8,60_i8];
_4 = _15.fld1.1;
_15.fld1.1 = _10;
(*_9) = core::ptr::addr_of_mut!(_2);
_8 = _13;
_7 = _6;
_13 = core::ptr::addr_of!((*_9));
Call(_15.fld1 = fn2(_1, _1, (*_7)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_13) = core::ptr::addr_of_mut!(_14);
_11 = [(-77_i8),39_i8,56_i8];
_15.fld1 = (_11, _10, (*_12));
(*_13) = core::ptr::addr_of_mut!((*_12));
_5 = 1853_i16 as i128;
(*_12) = !_15.fld1.2;
(*_12) = !_15.fld1.2;
_10 = _3 as i32;
_10 = -_4;
_16 = _15.fld0 as isize;
_4 = 1_usize as i32;
_12 = (*_6);
(*_6) = core::ptr::addr_of_mut!((*_12));
(*_7) = _12;
_9 = _13;
_2 = _14 - (*_12);
_8 = core::ptr::addr_of!((*_7));
_15.fld1.0 = _11;
(*_9) = core::ptr::addr_of_mut!(_15.fld1.2);
_8 = core::ptr::addr_of!((*_6));
_15.fld1.2 = _2;
RET = 51_i8 >> _14;
_7 = core::ptr::addr_of!((*_13));
match _1 {
18066002191548804837 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
(*_8) = core::ptr::addr_of_mut!(_15.fld1.2);
_17.2 = !_2;
(*_8) = core::ptr::addr_of_mut!(_14);
(*_9) = core::ptr::addr_of_mut!((*_12));
_15.fld1.2 = _17.2 | _17.2;
_7 = core::ptr::addr_of!((*_7));
RET = 2588428178_u32 as i8;
_8 = core::ptr::addr_of!((*_6));
(*_13) = core::ptr::addr_of_mut!(_14);
match _14 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
79016378375026185238791835265206827157 => bb11,
_ => bb10
}
}
bb4 = {
Return()
}
bb5 = {
(*_13) = core::ptr::addr_of_mut!(_14);
_11 = [(-77_i8),39_i8,56_i8];
_15.fld1 = (_11, _10, (*_12));
(*_13) = core::ptr::addr_of_mut!((*_12));
_5 = 1853_i16 as i128;
(*_12) = !_15.fld1.2;
(*_12) = !_15.fld1.2;
_10 = _3 as i32;
_10 = -_4;
_16 = _15.fld0 as isize;
_4 = 1_usize as i32;
_12 = (*_6);
(*_6) = core::ptr::addr_of_mut!((*_12));
(*_7) = _12;
_9 = _13;
_2 = _14 - (*_12);
_8 = core::ptr::addr_of!((*_7));
_15.fld1.0 = _11;
(*_9) = core::ptr::addr_of_mut!(_15.fld1.2);
_8 = core::ptr::addr_of!((*_6));
_15.fld1.2 = _2;
RET = 51_i8 >> _14;
_7 = core::ptr::addr_of!((*_13));
match _1 {
18066002191548804837 => bb3,
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
Return()
}
bb11 = {
(*_9) = core::ptr::addr_of_mut!((*_12));
_4 = _15.fld1.1;
_12 = core::ptr::addr_of_mut!(_17.2);
_12 = (*_8);
_1 = !13130452959446878228_u64;
_17.0 = [(-85_i8),(-63_i8),(-75_i8)];
_14 = _15.fld1.2;
_16 = !9223372036854775807_isize;
(*_7) = core::ptr::addr_of_mut!((*_12));
_20 = -_16;
_8 = core::ptr::addr_of!((*_9));
Goto(bb12)
}
bb12 = {
_2 = !_14;
_10 = -_15.fld1.1;
_7 = core::ptr::addr_of!((*_9));
(*_7) = _12;
RET = (-56_i8);
_5 = -(*_12);
_6 = _8;
_21 = false & false;
_4 = _10;
_14 = -_2;
(*_13) = core::ptr::addr_of_mut!(_15.fld1.2);
_19 = _17.2 >> _17.2;
(*_9) = core::ptr::addr_of_mut!((*_12));
(*_6) = core::ptr::addr_of_mut!(_5);
_12 = (*_9);
_17.2 = 5_usize as i128;
_17 = (_15.fld1.0, _15.fld1.1, (*_12));
_18 = [_15.fld1.2,_19,_2];
_15.fld1.0 = [110_i8,63_i8,(-42_i8)];
(*_7) = core::ptr::addr_of_mut!(_14);
_17 = _15.fld1;
(*_12) = _17.2 << _19;
_17.1 = _4 + _10;
_22.fld5.fld1.fld1.fld1.0 = _15.fld1.0;
Call(_11 = fn3(_22.fld5.fld1.fld1.fld1.0, _17, _3, _17.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_22.fld5.fld1.fld1.fld1.2 = _16 as i128;
_6 = core::ptr::addr_of!((*_7));
_24 = !(*_12);
_22.fld5.fld1.fld1.fld0 = 66_i8 as f32;
(*_9) = core::ptr::addr_of_mut!(_23.2);
_13 = core::ptr::addr_of!((*_8));
_24 = _1 as i128;
_25.fld2.1 = [_15.fld1.1,_4,_4,_4,_10,_17.1,_10];
Goto(bb14)
}
bb14 = {
_22.fld3 = [_3,_3];
_22.fld5.fld2 = [_10,_17.1,_4,_10,_17.1,_10,_4];
_22.fld5.fld1.fld0.6 = (-16523_i16) - (-9102_i16);
_22.fld5.fld3.fld1 = [124_i8,86_i8,57_i8];
_22.fld4.1 = [_1,_1,_1,_1,_1,_1,_1,_1];
_22.fld5.fld1.fld0.1 = (_17.1,);
_22.fld5.fld3.fld0.2 = [8_i8,(-44_i8),(-93_i8)];
_25.fld0 = (_3,);
_22.fld0 = _15.fld0 / f32::NEG_INFINITY;
_22.fld0 = 0_usize as f32;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(1_usize, 2_usize, Move(_2), 17_usize, Move(_17), 18_usize, Move(_18), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(1_usize, 24_usize, Move(_24), 10_usize, Move(_10), 14_usize, Move(_14), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u64,mut _2: u64,mut _3: *mut i128) -> ([i8; 3], i32, i128) {
mir! {
type RET = ([i8; 3], i32, i128);
let _4: f32;
let _5: Adt57;
let _6: Adt50;
let _7: bool;
let _8: usize;
let _9: i64;
let _10: ([i8; 3], i32, i128);
let _11: bool;
let _12: Adt57;
let _13: (i32,);
let _14: ([i8; 3], i32, i128);
let _15: u32;
let _16: [i32; 7];
let _17: isize;
let _18: ([i8; 3], i32, i128);
let _19: f64;
let _20: [i32; 7];
let _21: [i8; 3];
let _22: usize;
let _23: i128;
let _24: ([i8; 3], i32, i128);
let _25: [i32; 7];
let _26: f64;
let _27: (char,);
let _28: f64;
let _29: ([char; 2], [u64; 8], bool);
let _30: isize;
let _31: ();
let _32: ();
{
(*_3) = -64315441241562395610392394803837274732_i128;
RET.0 = [(-83_i8),76_i8,5_i8];
_4 = (*_3) as f32;
RET.1 = _4 as i32;
RET.1 = 1451367413_i32;
RET.1 = !2115971819_i32;
RET.2 = (-24827_i16) as i128;
RET.1 = (-95_i8) as i32;
RET.2 = (*_3);
_2 = (-15_i8) as u64;
RET.1 = 62_u8 as i32;
_6.fld0 = 6373681270130988698_i64 * 4739622393158943755_i64;
RET.2 = (*_3) >> _1;
_6.fld0 = 4429393933745166512_i64 | (-399573390378236908_i64);
_5.fld5 = core::ptr::addr_of_mut!(_2);
RET.0 = [105_i8,99_i8,96_i8];
_5.fld7 = -(*_3);
_6.fld2 = core::ptr::addr_of!(_3);
Goto(bb1)
}
bb1 = {
_8 = 3_usize;
_6.fld1 = '\u{421c0}';
_7 = true;
_5.fld2 = (-9223372036854775808_isize);
_4 = 1261076892_u32 as f32;
_6.fld0 = -3515240819843928180_i64;
_8 = !1_usize;
_6.fld1 = '\u{d7b69}';
_11 = !_7;
RET.1 = 258948742_i32;
_5.fld4.0 = [(-86_i8),125_i8,56_i8];
_10.0 = _5.fld4.0;
_5.fld7 = 167_u8 as i128;
_7 = _11;
RET.0 = [(-19_i8),118_i8,(-83_i8)];
_4 = 1713137284_u32 as f32;
RET.0 = [(-108_i8),(-5_i8),30_i8];
_5.fld7 = (-437847649_i32) as i128;
_12.fld4.2 = -(*_3);
_15 = 245836337_u32 + 2112679302_u32;
_5.fld4.1 = (-198479120_i32) ^ 210210410_i32;
_16 = [_5.fld4.1,_5.fld4.1,_5.fld4.1,_5.fld4.1,_5.fld4.1,_5.fld4.1,_5.fld4.1];
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
18066002191548804837 => bb7,
_ => bb6
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
_9 = _15 as i64;
RET = (_5.fld4.0, _5.fld4.1, (*_3));
_12.fld4.2 = -(*_3);
_10.2 = _5.fld7 ^ _12.fld4.2;
RET.0 = _10.0;
RET.1 = !_5.fld4.1;
_10 = (_5.fld4.0, _5.fld4.1, (*_3));
_5.fld0 = _5.fld4.0;
_14.1 = _10.1;
_5.fld5 = core::ptr::addr_of_mut!(_2);
_10.2 = -_5.fld7;
_12.fld6 = 27556456827958311196360013279196458746_u128 as f32;
_5.fld0 = [(-3_i8),16_i8,(-18_i8)];
_12.fld4.0 = _5.fld4.0;
_5.fld4.2 = _5.fld7;
RET.2 = (-22537_i16) as i128;
_13 = (_10.1,);
_7 = !_11;
match _5.fld2 {
0 => bb6,
1 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_9 = _6.fld0 - _6.fld0;
_21 = [78_i8,(-77_i8),(-74_i8)];
_18.1 = _13.0;
_14 = (_21, _13.0, _5.fld7);
_21 = _14.0;
RET.2 = !_5.fld4.2;
Goto(bb11)
}
bb11 = {
RET.1 = !_5.fld4.1;
_5.fld0 = [(-8_i8),(-75_i8),(-58_i8)];
_2 = _1;
_6.fld1 = '\u{dd49e}';
_5.fld4.2 = !_14.2;
_12.fld0 = _12.fld4.0;
Call(_13.0 = core::intrinsics::transmute(_14.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_5.fld4.2 = (*_3) ^ (*_3);
_5.fld6 = _12.fld6;
_7 = _11;
_20 = [_13.0,_14.1,_5.fld4.1,_14.1,_10.1,_5.fld4.1,_14.1];
_5.fld4 = _10;
_10 = (_12.fld0, _18.1, (*_3));
_23 = _12.fld6 as i128;
_12.fld7 = _5.fld7 & _14.2;
_5.fld0 = [32_i8,(-8_i8),44_i8];
_12.fld7 = _5.fld2 as i128;
RET.1 = _10.1;
(*_3) = 122623432802452890113574632526119573886_u128 as i128;
RET.1 = _5.fld4.1;
_12.fld2 = _5.fld2;
_12.fld4.2 = (*_3);
_17 = !_12.fld2;
Goto(bb13)
}
bb13 = {
_7 = !_11;
RET.1 = _18.1 | _5.fld4.1;
_12.fld4 = _5.fld4;
_16 = [_12.fld4.1,_12.fld4.1,_10.1,_14.1,_13.0,_13.0,_12.fld4.1];
_5.fld4.2 = _2 as i128;
_5.fld4.2 = _23 >> _10.1;
_5.fld4.2 = _5.fld6 as i128;
_15 = 3859791251_u32 + 2569361061_u32;
_24.1 = -_10.1;
_24 = (_12.fld0, _12.fld4.1, _14.2);
_24.0 = [58_i8,125_i8,(-96_i8)];
_22 = _8 * _8;
Goto(bb14)
}
bb14 = {
_11 = _7;
_12.fld0 = _21;
_13.0 = !_5.fld4.1;
_5.fld5 = core::ptr::addr_of_mut!(_1);
_8 = _22 | _22;
_10.2 = !_12.fld4.2;
_12.fld5 = core::ptr::addr_of_mut!(_2);
RET.0 = [35_i8,(-21_i8),(-26_i8)];
_26 = (-79_i8) as f64;
_14.2 = _12.fld6 as i128;
(*_3) = 123879562881546412217185816745186335521_u128 as i128;
_6.fld0 = _9;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(2_usize, 2_usize, Move(_2), 22_usize, Move(_22), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(2_usize, 20_usize, Move(_20), 23_usize, Move(_23), 13_usize, Move(_13), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i8; 3],mut _2: ([i8; 3], i32, i128),mut _3: char,mut _4: i32) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _5: i64;
let _6: u8;
let _7: [u8; 4];
let _8: u32;
let _9: (isize, (i32,), *mut i128, i8, i8, *mut i128, i16);
let _10: [u32; 1];
let _11: isize;
let _12: i64;
let _13: char;
let _14: Adt62;
let _15: [u32; 1];
let _16: i128;
let _17: isize;
let _18: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]);
let _19: [i128; 3];
let _20: (isize, (i32,), *mut i128, i8, i8, *mut i128, i16);
let _21: isize;
let _22: (isize, (i32,), *mut i128, i8, i8, *mut i128, i16);
let _23: bool;
let _24: *const i32;
let _25: ();
let _26: ();
{
_3 = '\u{5ce76}';
_2.1 = -_4;
RET = [(-9_i8),(-110_i8),(-24_i8)];
Call(_5 = fn4(_2.2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _2.0;
RET = [(-33_i8),83_i8,51_i8];
_2 = (_1, _4, (-151708513751221961138926065682951846134_i128));
_6 = 245_u8;
_2 = (_1, _4, (-26794467717394048190988603945570541985_i128));
_3 = '\u{106c7e}';
_4 = -_2.1;
_9.0 = _2.2 as isize;
_9.4 = 11521192052971601982_usize as i8;
_9.1.0 = _4 | _4;
_9.5 = core::ptr::addr_of_mut!(_2.2);
_9.5 = core::ptr::addr_of_mut!(_2.2);
_9.1.0 = -_2.1;
_6 = 226_u8;
_3 = '\u{7f0d2}';
_9.6 = 21840_i16 + (-3819_i16);
_2.0 = [_9.4,_9.4,_9.4];
_5 = 313374396947089556_i64;
_9.6 = (-22861_i16) + (-30607_i16);
RET = [_9.4,_9.4,_9.4];
_2.0 = [_9.4,_9.4,_9.4];
_2 = (_1, _9.1.0, (-129307136714643020198277735230206116563_i128));
match _2.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
210975230206295443265096872201562094893 => bb9,
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
_2.0 = [_9.4,_9.4,_9.4];
RET = _1;
RET = [_9.4,_9.4,_9.4];
_9.2 = core::ptr::addr_of_mut!(_2.2);
_8 = 2255791685_u32 ^ 894108237_u32;
_2.0 = _1;
_12 = _5 + _5;
_9.3 = -_9.4;
_2.1 = _2.2 as i32;
_10 = [_8];
_10 = [_8];
_1 = [_9.3,_9.3,_9.3];
RET = _2.0;
_9.0 = (-105_isize) & 78_isize;
_2.2 = 62172030089577995511025301774524198188_i128;
_9.0 = _4 as isize;
RET = [_9.3,_9.3,_9.3];
_9.5 = core::ptr::addr_of_mut!(_2.2);
_13 = _3;
Goto(bb10)
}
bb10 = {
_7 = [_6,_6,_6,_6];
_4 = 98160279013009600326481474621459719725_u128 as i32;
_2.0 = [_9.3,_9.4,_9.4];
_15 = [_8];
_14.fld2.fld1 = _9.1;
_14.fld1 = !277527771816847319090293283179985499336_u128;
_13 = _3;
_14.fld2.fld2 = _8 | _8;
Goto(bb11)
}
bb11 = {
_8 = !_14.fld2.fld2;
_9.1 = _14.fld2.fld1;
_14.fld2.fld2 = !_8;
_2.1 = _9.1.0 ^ _14.fld2.fld1.0;
_5 = -_12;
_1 = _2.0;
_14.fld0 = [_3,_13];
match _2.2 {
0 => bb12,
62172030089577995511025301774524198188 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_9.1.0 = _2.1 ^ _2.1;
_5 = -_12;
_9.6 = -(-7865_i16);
_9.0 = 3_usize as isize;
_6 = _5 as u8;
_14.fld2.fld2 = _8;
_2.2 = (-3742817570024575043995850991106899353_i128) | (-95599217372372858818648595540721231660_i128);
_4 = _14.fld2.fld2 as i32;
_11 = _9.0;
_21 = _11;
_19 = [_2.2,_2.2,_2.2];
_2.0 = [_9.3,_9.3,_9.3];
_20.4 = !_9.4;
RET = [_9.4,_20.4,_9.4];
_18.0 = core::ptr::addr_of_mut!(_20);
_3 = _13;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(3_usize, 3_usize, Move(_3), 5_usize, Move(_5), 21_usize, Move(_21), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(3_usize, 10_usize, Move(_10), 12_usize, Move(_12), 6_usize, Move(_6), 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i128,mut _2: ([i8; 3], i32, i128)) -> i64 {
mir! {
type RET = i64;
let _3: (*mut (i16, *mut i128, [i8; 3]), i8, i64, usize);
let _4: [i32; 7];
let _5: bool;
let _6: Adt64;
let _7: f32;
let _8: Adt51;
let _9: (isize, (i32,), *mut i128, i8, i8, *mut i128, i16);
let _10: bool;
let _11: *mut ([i8; 3], i32, i128);
let _12: ([i8; 3], i32, i128);
let _13: ([i8; 3], i32, i128);
let _14: [i32; 7];
let _15: i8;
let _16: [i16; 6];
let _17: [i32; 7];
let _18: u64;
let _19: [i8; 3];
let _20: f64;
let _21: ([i8; 3], i32, i128);
let _22: i16;
let _23: bool;
let _24: u128;
let _25: i128;
let _26: Adt56;
let _27: [i16; 6];
let _28: (i32,);
let _29: usize;
let _30: u8;
let _31: isize;
let _32: f32;
let _33: [char; 2];
let _34: ();
let _35: ();
{
RET = false as i64;
_2.2 = -_1;
_2.2 = 53_u8 as i128;
_2.0 = [(-119_i8),105_i8,(-26_i8)];
_2.2 = 277059975167077735735324493390768744281_u128 as i128;
_3.1 = false as i8;
_1 = _2.2 & _2.2;
_3.3 = 5_usize - 4_usize;
_3.2 = 107566129529336810907390989882019068913_u128 as i64;
_3.1 = (-67_i8);
_2.1 = 28057_u16 as i32;
_3.1 = 12437666218818032884171214378642091111_u128 as i8;
_4 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
RET = _3.2 << _3.1;
_5 = !false;
_6.fld2.fld1.1 = core::ptr::addr_of!(_6.fld3.fld0.1);
_3.1 = 86_i8;
_6.fld3.fld6 = 65254268591745063844052625373412824747_u128;
_6.fld2.fld2 = core::ptr::addr_of_mut!(_6.fld3.fld7);
_2.1 = (-901588109_i32);
_3.0 = core::ptr::addr_of_mut!(_6.fld3.fld0);
_6.fld3.fld7.5 = core::ptr::addr_of_mut!(_2.2);
_6.fld3.fld7.0 = 7563570205231045432_u64 as isize;
_6.fld2.fld1.3 = core::ptr::addr_of!(_6.fld3.fld7.2);
_6.fld3.fld7.4 = _3.1 & _3.1;
_6.fld2.fld0 = [4065501131_u32];
Goto(bb1)
}
bb1 = {
_6.fld3.fld4.fld3 = core::ptr::addr_of!(_6.fld3.fld0.1);
_6.fld3.fld2 = !_6.fld3.fld7.0;
_6.fld3.fld1 = core::ptr::addr_of_mut!(_6.fld1.2);
_8.fld1.0 = 7705692578309963465_u64 >> _2.2;
_3.0 = core::ptr::addr_of_mut!(_6.fld3.fld0);
_6.fld0 = _6.fld3.fld2 as f32;
_6.fld3.fld4.fld0 = ('\u{a4fa}',);
_6.fld3.fld4.fld2 = (_6.fld2.fld2, _4);
_2.2 = _1 | _1;
_6.fld2.fld1.2 = _2.0;
_6.fld3.fld4.fld1 = -(-32762_i16);
_6.fld3.fld4.fld0 = ('\u{25f02}',);
_2.1 = (-689402852_i32) - (-705085770_i32);
Goto(bb2)
}
bb2 = {
_6.fld2.fld1.0 = !_8.fld1.0;
_6.fld3.fld0.2 = [_6.fld3.fld7.4,_6.fld3.fld7.4,_6.fld3.fld7.4];
_12 = _2;
_12.2 = -_2.2;
Goto(bb3)
}
bb3 = {
_6.fld3.fld7.1.0 = _6.fld3.fld4.fld0.0 as i32;
_2.0 = [_6.fld3.fld7.4,_3.1,_6.fld3.fld7.4];
_6.fld3.fld0.0 = 6007_u16 as i16;
_12.2 = _2.2;
_3.1 = _12.2 as i8;
_2.0 = _6.fld3.fld0.2;
_6.fld0 = _3.3 as f32;
Call(_6.fld3.fld4.fld3 = fn5(_6.fld3.fld4.fld2, _6.fld2.fld2, _6.fld3.fld0.2, _3.0, _3, _6.fld2, _6.fld3.fld4.fld2, _6.fld3.fld7.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6.fld3.fld7.2 = core::ptr::addr_of_mut!(_2.2);
RET = -_3.2;
_6.fld3.fld7.6 = _6.fld3.fld4.fld1 | _6.fld3.fld4.fld1;
_6.fld3.fld0 = (_6.fld3.fld7.6, _6.fld3.fld7.2, _6.fld2.fld1.2);
_11 = core::ptr::addr_of_mut!(_13);
_15 = _6.fld3.fld7.4;
_13.0 = _2.0;
_12 = ((*_11).0, _2.1, _2.2);
_6.fld3.fld0.2 = [_6.fld3.fld7.3,_6.fld3.fld7.3,_6.fld3.fld7.3];
(*_11).1 = _6.fld3.fld7.1.0 | _2.1;
_5 = false | false;
_6.fld3.fld7.5 = core::ptr::addr_of_mut!(_12.2);
_6.fld3.fld4.fld2.0 = _6.fld2.fld2;
_6.fld1.1 = _6.fld3.fld7.4;
_6.fld0 = 179_u8 as f32;
_7 = -_6.fld0;
_12.0 = _2.0;
Goto(bb5)
}
bb5 = {
_9.6 = -_6.fld3.fld7.6;
_6.fld1 = (_3.0, _6.fld3.fld7.4, _3.2, _3.3);
_8.fld0 = [1190437666_u32];
_9.1 = ((*_11).1,);
_19 = [_6.fld3.fld7.3,_6.fld3.fld7.3,_6.fld3.fld7.4];
Goto(bb6)
}
bb6 = {
_21.2 = _2.1 as i128;
_8 = Adt51 { fld0: _6.fld2.fld0,fld1: _6.fld2.fld1,fld2: _6.fld2.fld2 };
_9.2 = _6.fld3.fld7.5;
_6.fld2.fld1.1 = core::ptr::addr_of!(_9.2);
_6.fld3.fld7 = (_6.fld3.fld2, _9.1, _6.fld3.fld0.1, _15, _15, _9.2, _6.fld3.fld0.0);
_9.6 = !_6.fld3.fld7.6;
match _6.fld3.fld6 {
0 => bb7,
65254268591745063844052625373412824747 => bb9,
_ => bb8
}
}
bb7 = {
_6.fld3.fld7.1.0 = _6.fld3.fld4.fld0.0 as i32;
_2.0 = [_6.fld3.fld7.4,_3.1,_6.fld3.fld7.4];
_6.fld3.fld0.0 = 6007_u16 as i16;
_12.2 = _2.2;
_3.1 = _12.2 as i8;
_2.0 = _6.fld3.fld0.2;
_6.fld0 = _3.3 as f32;
Call(_6.fld3.fld4.fld3 = fn5(_6.fld3.fld4.fld2, _6.fld2.fld2, _6.fld3.fld0.2, _3.0, _3, _6.fld2, _6.fld3.fld4.fld2, _6.fld3.fld7.0), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_6.fld3.fld7.2 = core::ptr::addr_of_mut!(_2.2);
RET = -_3.2;
_6.fld3.fld7.6 = _6.fld3.fld4.fld1 | _6.fld3.fld4.fld1;
_6.fld3.fld0 = (_6.fld3.fld7.6, _6.fld3.fld7.2, _6.fld2.fld1.2);
_11 = core::ptr::addr_of_mut!(_13);
_15 = _6.fld3.fld7.4;
_13.0 = _2.0;
_12 = ((*_11).0, _2.1, _2.2);
_6.fld3.fld0.2 = [_6.fld3.fld7.3,_6.fld3.fld7.3,_6.fld3.fld7.3];
(*_11).1 = _6.fld3.fld7.1.0 | _2.1;
_5 = false | false;
_6.fld3.fld7.5 = core::ptr::addr_of_mut!(_12.2);
_6.fld3.fld4.fld2.0 = _6.fld2.fld2;
_6.fld1.1 = _6.fld3.fld7.4;
_6.fld0 = 179_u8 as f32;
_7 = -_6.fld0;
_12.0 = _2.0;
Goto(bb5)
}
bb9 = {
_6.fld3.fld4.fld0 = ('\u{9eeb7}',);
_6.fld3.fld4.fld2 = (_8.fld2, _4);
_8.fld2 = core::ptr::addr_of_mut!(_9);
_6.fld2.fld1.2 = [_15,_6.fld3.fld7.4,_15];
_6.fld3.fld7.6 = -_9.6;
_6.fld3.fld4.fld1 = !_9.6;
_18 = _6.fld2.fld1.0;
_19 = [_15,_6.fld3.fld7.3,_6.fld3.fld7.4];
(*_11) = (_12.0, _12.1, _12.2);
_6.fld3.fld4.fld2.0 = core::ptr::addr_of_mut!(_9);
_3.2 = -_6.fld1.2;
Call(_21.0 = core::intrinsics::transmute(_2.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3.2 = _13.1 as i64;
match _6.fld3.fld6 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb6,
4 => bb5,
5 => bb11,
6 => bb12,
65254268591745063844052625373412824747 => bb14,
_ => bb13
}
}
bb11 = {
_6.fld3.fld4.fld0 = ('\u{9eeb7}',);
_6.fld3.fld4.fld2 = (_8.fld2, _4);
_8.fld2 = core::ptr::addr_of_mut!(_9);
_6.fld2.fld1.2 = [_15,_6.fld3.fld7.4,_15];
_6.fld3.fld7.6 = -_9.6;
_6.fld3.fld4.fld1 = !_9.6;
_18 = _6.fld2.fld1.0;
_19 = [_15,_6.fld3.fld7.3,_6.fld3.fld7.4];
(*_11) = (_12.0, _12.1, _12.2);
_6.fld3.fld4.fld2.0 = core::ptr::addr_of_mut!(_9);
_3.2 = -_6.fld1.2;
Call(_21.0 = core::intrinsics::transmute(_2.0), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_6.fld3.fld7.2 = core::ptr::addr_of_mut!(_2.2);
RET = -_3.2;
_6.fld3.fld7.6 = _6.fld3.fld4.fld1 | _6.fld3.fld4.fld1;
_6.fld3.fld0 = (_6.fld3.fld7.6, _6.fld3.fld7.2, _6.fld2.fld1.2);
_11 = core::ptr::addr_of_mut!(_13);
_15 = _6.fld3.fld7.4;
_13.0 = _2.0;
_12 = ((*_11).0, _2.1, _2.2);
_6.fld3.fld0.2 = [_6.fld3.fld7.3,_6.fld3.fld7.3,_6.fld3.fld7.3];
(*_11).1 = _6.fld3.fld7.1.0 | _2.1;
_5 = false | false;
_6.fld3.fld7.5 = core::ptr::addr_of_mut!(_12.2);
_6.fld3.fld4.fld2.0 = _6.fld2.fld2;
_6.fld1.1 = _6.fld3.fld7.4;
_6.fld0 = 179_u8 as f32;
_7 = -_6.fld0;
_12.0 = _2.0;
Goto(bb5)
}
bb13 = {
_6.fld3.fld7.1.0 = _6.fld3.fld4.fld0.0 as i32;
_2.0 = [_6.fld3.fld7.4,_3.1,_6.fld3.fld7.4];
_6.fld3.fld0.0 = 6007_u16 as i16;
_12.2 = _2.2;
_3.1 = _12.2 as i8;
_2.0 = _6.fld3.fld0.2;
_6.fld0 = _3.3 as f32;
Call(_6.fld3.fld4.fld3 = fn5(_6.fld3.fld4.fld2, _6.fld2.fld2, _6.fld3.fld0.2, _3.0, _3, _6.fld2, _6.fld3.fld4.fld2, _6.fld3.fld7.0), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_9.0 = (*_11).2 as isize;
_16 = [_6.fld3.fld7.6,_6.fld3.fld0.0,_9.6,_9.6,_6.fld3.fld7.6,_6.fld3.fld4.fld1];
_26.fld0.5 = core::ptr::addr_of_mut!(_2.2);
_6.fld3.fld7.0 = _1 as isize;
_3 = (_6.fld1.0, _6.fld1.1, _6.fld1.2, _6.fld1.3);
_26.fld1.fld1.2 = (*_11).2;
_26.fld1.fld1 = ((*_11).0, _9.1.0, _13.2);
_25 = 1100662931_u32 as i128;
_13.0 = [_6.fld3.fld7.3,_3.1,_6.fld3.fld7.3];
(*_11) = (_6.fld3.fld0.2, _6.fld3.fld7.1.0, _12.2);
_26.fld0.1 = (_26.fld1.fld1.1,);
_30 = !4_u8;
_6.fld3.fld4.fld2.1 = [(*_11).1,(*_11).1,_9.1.0,_9.1.0,_26.fld1.fld1.1,_6.fld3.fld7.1.0,_26.fld1.fld1.1];
_9.3 = -_6.fld3.fld7.4;
_1 = (*_11).2;
_6.fld1.0 = core::ptr::addr_of_mut!(_6.fld3.fld0);
_6.fld2.fld1.2 = [_6.fld3.fld7.3,_15,_15];
_6.fld3.fld4.fld2 = (_6.fld2.fld2, _4);
_17 = [(*_11).1,_26.fld0.1.0,_13.1,_6.fld3.fld7.1.0,_6.fld3.fld7.1.0,_2.1,_26.fld1.fld1.1];
_8.fld1.1 = core::ptr::addr_of!(_6.fld3.fld7.2);
_13.1 = _6.fld3.fld7.1.0 << _9.3;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(4_usize, 1_usize, Move(_1), 30_usize, Move(_30), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(4_usize, 16_usize, Move(_16), 5_usize, Move(_5), 35_usize, _35, 35_usize, _35), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]),mut _2: *mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16),mut _3: [i8; 3],mut _4: *mut (i16, *mut i128, [i8; 3]),mut _5: (*mut (i16, *mut i128, [i8; 3]), i8, i64, usize),mut _6: Adt51,mut _7: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]),mut _8: isize) -> *const *mut i128 {
mir! {
type RET = *const *mut i128;
let _9: *mut bool;
let _10: (char,);
let _11: *const *mut i128;
let _12: ([i8; 3], i32, i128);
let _13: *const *mut ([i8; 3], i32, i128);
let _14: isize;
let _15: isize;
let _16: [i8; 3];
let _17: f32;
let _18: [i16; 6];
let _19: char;
let _20: [i128; 3];
let _21: *mut bool;
let _22: i16;
let _23: f64;
let _24: isize;
let _25: char;
let _26: usize;
let _27: bool;
let _28: usize;
let _29: f64;
let _30: [char; 2];
let _31: isize;
let _32: u32;
let _33: Adt64;
let _34: f64;
let _35: i32;
let _36: ([char; 2], [u64; 8], bool);
let _37: usize;
let _38: f32;
let _39: (i32,);
let _40: (i16, *mut i128, [i8; 3]);
let _41: ([char; 2], [u64; 8], bool);
let _42: isize;
let _43: Adt65;
let _44: bool;
let _45: bool;
let _46: f32;
let _47: (*mut (i16, *mut i128, [i8; 3]), i8, i64, usize);
let _48: f32;
let _49: (*mut (i16, *mut i128, [i8; 3]), i8, i64, usize);
let _50: Adt63;
let _51: f64;
let _52: i16;
let _53: Adt60;
let _54: f64;
let _55: Adt65;
let _56: u32;
let _57: i128;
let _58: isize;
let _59: u64;
let _60: [u8; 4];
let _61: (char,);
let _62: isize;
let _63: u16;
let _64: isize;
let _65: [u32; 1];
let _66: (u64, *const *mut i128, [i8; 3], *const *mut i128);
let _67: [i128; 3];
let _68: ();
let _69: ();
{
_6.fld1.1 = core::ptr::addr_of!((*_4).1);
_6.fld1.0 = _5.2 as u64;
(*_4) = (29156_i16, (*_2).5, _6.fld1.2);
(*_4).0 = _5.2 as i16;
_11 = core::ptr::addr_of!((*_2).2);
_5 = (_4, (*_2).4, 5223925901662828223_i64, 4_usize);
(*_11) = core::ptr::addr_of_mut!(_12.2);
(*_2).3 = false as i8;
(*_2).4 = !_5.1;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2).3 = _5.1 - (*_2).4;
_1.0 = core::ptr::addr_of_mut!((*_2));
_6.fld0 = [2780255459_u32];
(*_2).1 = (1458575968_i32,);
_12.1 = (*_2).1.0;
(*_2).2 = (*_2).5;
RET = _6.fld1.1;
(*_2).1 = (_12.1,);
(*_4).0 = 21494_i16;
Call(RET = fn6(_1.0, _6.fld1.1, _1, (*RET), _6.fld1.3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_2).0 = _8 >> (*_2).4;
_6.fld1.1 = _11;
(*_2).2 = core::ptr::addr_of_mut!(_12.2);
(*_2).4 = false as i8;
_5.0 = core::ptr::addr_of_mut!((*_4));
_3 = (*_4).2;
(*_4).2 = _3;
Goto(bb2)
}
bb2 = {
_17 = _5.3 as f32;
_5.1 = (*_2).4 << _5.3;
(*_4).0 = (*_2).6;
(*_2).4 = _5.1;
_19 = '\u{eb08f}';
_3 = (*_4).2;
(*_2).1.0 = _12.1 - _12.1;
_3 = [(*_2).4,_5.1,(*_2).4];
_6.fld1.1 = core::ptr::addr_of!((*_2).2);
(*_2).0 = _17 as isize;
(*_4).1 = core::ptr::addr_of_mut!(_12.2);
_3 = [_5.1,_5.1,(*_2).3];
_6.fld1.3 = core::ptr::addr_of!((*_2).2);
_1 = (_6.fld2, _7.1);
_16 = [(*_2).4,_5.1,(*_2).4];
_5 = (_4, (*_2).4, 7082506583100042803_i64, 17485828184139352175_usize);
(*_2).1 = (_12.1,);
(*_2).1 = (_12.1,);
Goto(bb3)
}
bb3 = {
(*_4) = ((*_2).6, (*_2).5, _16);
_14 = !(*_2).0;
_10.0 = _19;
_12.2 = !148296500675042473555314865494290057624_i128;
(*_2).3 = -(*_2).4;
_18 = [(*_2).6,(*_2).6,(*_2).6,(*_2).6,(*_2).6,(*_2).6];
_7.0 = _6.fld2;
_7.0 = core::ptr::addr_of_mut!((*_2));
(*_2).4 = 88_u8 as i8;
_11 = _6.fld1.3;
(*_2).0 = _8;
(*_2).2 = core::ptr::addr_of_mut!(_12.2);
_20 = [_12.2,_12.2,_12.2];
_11 = core::ptr::addr_of!((*_2).5);
match _5.2 {
0 => bb1,
1 => bb4,
7082506583100042803 => bb6,
_ => bb5
}
}
bb4 = {
_17 = _5.3 as f32;
_5.1 = (*_2).4 << _5.3;
(*_4).0 = (*_2).6;
(*_2).4 = _5.1;
_19 = '\u{eb08f}';
_3 = (*_4).2;
(*_2).1.0 = _12.1 - _12.1;
_3 = [(*_2).4,_5.1,(*_2).4];
_6.fld1.1 = core::ptr::addr_of!((*_2).2);
(*_2).0 = _17 as isize;
(*_4).1 = core::ptr::addr_of_mut!(_12.2);
_3 = [_5.1,_5.1,(*_2).3];
_6.fld1.3 = core::ptr::addr_of!((*_2).2);
_1 = (_6.fld2, _7.1);
_16 = [(*_2).4,_5.1,(*_2).4];
_5 = (_4, (*_2).4, 7082506583100042803_i64, 17485828184139352175_usize);
(*_2).1 = (_12.1,);
(*_2).1 = (_12.1,);
Goto(bb3)
}
bb5 = {
(*_2).0 = _8 >> (*_2).4;
_6.fld1.1 = _11;
(*_2).2 = core::ptr::addr_of_mut!(_12.2);
(*_2).4 = false as i8;
_5.0 = core::ptr::addr_of_mut!((*_4));
_3 = (*_4).2;
(*_4).2 = _3;
Goto(bb2)
}
bb6 = {
_27 = !false;
_6.fld2 = core::ptr::addr_of_mut!((*_2));
_5.1 = (*_2).3;
_12.1 = (*_2).1.0 >> _12.2;
(*_4) = ((*_2).6, (*_2).2, _3);
_24 = _14;
(*_2).3 = _5.1;
_26 = (*_2).3 as usize;
_6.fld2 = core::ptr::addr_of_mut!((*_2));
(*_2).5 = core::ptr::addr_of_mut!(_12.2);
(*_2).0 = -_14;
_8 = (*_2).0 >> _5.3;
(*_2).3 = _5.1;
_29 = _6.fld1.0 as f64;
(*_2).1.0 = (*_4).0 as i32;
_5.0 = core::ptr::addr_of_mut!((*_4));
_15 = (*_2).0;
_9 = core::ptr::addr_of_mut!(_27);
(*_2).2 = core::ptr::addr_of_mut!(_12.2);
_6.fld1.2 = [(*_2).4,(*_2).3,(*_2).3];
(*_2).1.0 = _12.1;
Goto(bb7)
}
bb7 = {
_23 = _29 + _29;
_1.0 = core::ptr::addr_of_mut!((*_2));
_3 = [(*_2).3,(*_2).3,(*_2).3];
_1.0 = core::ptr::addr_of_mut!((*_2));
_33.fld2 = _6;
(*_2).4 = (*_2).3 | (*_2).3;
(*_4) = ((*_2).6, (*_2).5, _3);
_33.fld3.fld4.fld0 = (_10.0,);
_33.fld3.fld4.fld2 = (_6.fld2, _7.1);
_5.2 = 6220752273040302399_i64 << _8;
_33.fld3.fld7.2 = core::ptr::addr_of_mut!(_12.2);
_7 = _33.fld3.fld4.fld2;
_33.fld3.fld4.fld3 = _33.fld2.fld1.1;
match _5.3 {
17485828184139352175 => bb8,
_ => bb1
}
}
bb8 = {
(*_4).1 = (*_2).5;
_37 = _26;
_33.fld3.fld7.6 = (*_4).0;
_6 = Adt51 { fld0: _33.fld2.fld0,fld1: _33.fld2.fld1,fld2: _33.fld2.fld2 };
_8 = _14 * (*_2).0;
_33.fld3.fld4.fld3 = core::ptr::addr_of!((*_11));
_33.fld3.fld4.fld1 = (*_2).1.0 as i16;
_28 = _5.3 >> _12.1;
(*_4).1 = core::ptr::addr_of_mut!(_12.2);
_30 = [_10.0,_33.fld3.fld4.fld0.0];
_1.0 = core::ptr::addr_of_mut!((*_2));
_17 = _5.3 as f32;
(*_9) = true;
_33.fld2.fld1.2 = [(*_2).4,(*_2).3,(*_2).4];
_3 = [(*_2).4,(*_2).4,(*_2).3];
_9 = core::ptr::addr_of_mut!((*_9));
_33.fld0 = _17 - _17;
_32 = (*_2).1.0 as u32;
_33.fld3.fld0.2 = [(*_2).4,(*_2).4,(*_2).3];
_39.0 = 64350_u16 as i32;
_6.fld1.1 = core::ptr::addr_of!((*_2).2);
(*_2).2 = core::ptr::addr_of_mut!(_12.2);
_12.0 = [(*_2).3,_5.1,(*_2).4];
_31 = _15 + _15;
(*_2).2 = _33.fld3.fld7.2;
Goto(bb9)
}
bb9 = {
_18 = [(*_2).6,_33.fld3.fld4.fld1,(*_4).0,_33.fld3.fld7.6,_33.fld3.fld7.6,_33.fld3.fld7.6];
_40.2 = [(*_2).4,(*_2).4,_5.1];
(*_2).5 = (*_2).2;
_22 = (*_2).6 * (*_4).0;
_30 = [_33.fld3.fld4.fld0.0,_33.fld3.fld4.fld0.0];
_33.fld3.fld6 = !138836978699227052951919103148113337685_u128;
_6.fld2 = core::ptr::addr_of_mut!((*_2));
_33.fld3.fld7.0 = !_8;
_33.fld3.fld2 = !_33.fld3.fld7.0;
(*_2).2 = (*_4).1;
_12.2 = (-92315467676348665107731066343523161727_i128) >> _15;
RET = core::ptr::addr_of!(_40.1);
_33.fld3.fld3 = _18;
_12.2 = !(-98409135549523346458162480899871530804_i128);
Goto(bb10)
}
bb10 = {
(*_2) = (_31, _39, _33.fld3.fld7.2, _5.1, _5.1, _33.fld3.fld7.2, _22);
_33.fld2.fld0 = _6.fld0;
_33.fld3.fld3 = [_22,_33.fld3.fld4.fld1,_33.fld3.fld7.6,(*_2).6,(*_4).0,_33.fld3.fld4.fld1];
_43.fld5.fld3.fld2 = core::ptr::addr_of_mut!(_43.fld5.fld1.fld1.fld1);
_40.0 = _33.fld3.fld4.fld1 + (*_2).6;
_24 = _33.fld2.fld1.0 as isize;
_10 = _33.fld3.fld4.fld0;
(*_2).3 = -(*_2).4;
_43.fld4.0 = _30;
_36.1 = [_33.fld2.fld1.0,_6.fld1.0,_6.fld1.0,_33.fld2.fld1.0,_6.fld1.0,_6.fld1.0,_6.fld1.0,_33.fld2.fld1.0];
(*_2).6 = _33.fld3.fld4.fld1;
_36.0 = [_10.0,_10.0];
(*RET) = (*_4).1;
_33.fld3.fld7.2 = (*_2).2;
_5.2 = (-5560191039504257488_i64);
_6.fld1.3 = core::ptr::addr_of!((*_4).1);
Goto(bb11)
}
bb11 = {
_33.fld3.fld6 = 132234552396436780462050990132858290149_u128;
_49.1 = -(*_2).3;
_33.fld3.fld0.0 = _40.0;
_43.fld5.fld1.fld0.1.0 = _10.0 as i32;
_43.fld5.fld1.fld1.fld1 = _12;
_33.fld3.fld7.4 = -_49.1;
_43.fld5.fld1.fld0.4 = _5.1 | _5.1;
_50.fld7.fld1 = _33.fld2.fld0;
_50.fld7.fld0 = (_3, _39.0, _12.2);
_43.fld5.fld3.fld0.1 = core::ptr::addr_of_mut!(_43.fld5.fld1.fld1.fld1.2);
_50.fld7.fld4.fld0 = _6.fld0;
_37 = _27 as usize;
_33.fld3.fld7.3 = _33.fld3.fld7.4;
_50.fld6 = !_32;
(*RET) = (*_2).5;
_15 = _49.1 as isize;
RET = core::ptr::addr_of!((*_4).1);
_54 = _29;
_21 = core::ptr::addr_of_mut!((*_9));
_50.fld7.fld4 = _6;
_5.1 = (*_2).4;
_11 = core::ptr::addr_of!((*_11));
_50.fld7.fld2 = Adt50 { fld0: _5.2,fld1: _10.0,fld2: _33.fld2.fld1.3 };
_6.fld2 = _7.0;
_53.fld3.fld2 = _43.fld5.fld3.fld2;
Goto(bb12)
}
bb12 = {
_53.fld1.fld0.3 = (*_2).3;
_3 = _50.fld7.fld0.0;
_52 = _22;
_43.fld5.fld1.fld0.3 = _33.fld3.fld7.4;
_55.fld4.0 = [_33.fld3.fld4.fld0.0,_19];
_53.fld1.fld1.fld1.2 = 20350_u16 as i128;
_55.fld5.fld3.fld0.0 = _33.fld3.fld0.0;
_50.fld5 = 50_u8 as i32;
_33.fld3.fld0.1 = core::ptr::addr_of_mut!(_55.fld2);
_6.fld1.0 = _43.fld5.fld1.fld1.fld1.2 as u64;
_16 = [_43.fld5.fld1.fld0.4,(*_2).4,_49.1];
_12.0 = [_43.fld5.fld1.fld0.4,_43.fld5.fld1.fld0.4,(*_2).4];
_43.fld5.fld1.fld2 = core::ptr::addr_of!(_17);
(*RET) = (*_2).2;
_12.2 = _50.fld7.fld0.2 * _43.fld5.fld1.fld1.fld1.2;
_54 = _17 as f64;
_43.fld2 = -_50.fld7.fld0.2;
_53.fld1.fld0.1 = _43.fld5.fld1.fld0.1;
_43.fld3 = [_50.fld7.fld2.fld1,_19];
_50.fld4 = [_32];
Goto(bb13)
}
bb13 = {
(*_2).1.0 = _50.fld7.fld2.fld1 as i32;
_43.fld5.fld1.fld0.6 = 22_u8 as i16;
(*_9) = !false;
_4 = core::ptr::addr_of_mut!(_40);
_55.fld5.fld1.fld1.fld0 = -_33.fld0;
_55.fld5.fld1.fld1.fld1.2 = _12.2 << (*_2).3;
_43.fld4.1 = [_33.fld2.fld1.0,_6.fld1.0,_33.fld2.fld1.0,_6.fld1.0,_50.fld7.fld4.fld1.0,_33.fld2.fld1.0,_33.fld2.fld1.0,_33.fld2.fld1.0];
_33.fld3.fld4.fld0.0 = _50.fld7.fld2.fld1;
_36.2 = (*_9);
_33.fld2 = Adt51 { fld0: _50.fld7.fld1,fld1: _50.fld7.fld4.fld1,fld2: _6.fld2 };
_61 = (_19,);
_47.2 = -_5.2;
_56 = _32 << _40.0;
_63 = _50.fld7.fld4.fld1.0 as u16;
_55.fld5.fld1.fld0.3 = (*_2).3;
_1.1 = _33.fld3.fld4.fld2.1;
_10 = (_50.fld7.fld2.fld1,);
_43.fld4.2 = (*_9);
_50.fld6 = (*_2).1.0 as u32;
Goto(bb14)
}
bb14 = {
_1.0 = core::ptr::addr_of_mut!((*_2));
_33.fld3.fld7.6 = (*_4).0 | _52;
_30 = [_61.0,_19];
Goto(bb15)
}
bb15 = {
Call(_68 = dump_var(5_usize, 16_usize, Move(_16), 19_usize, Move(_19), 63_usize, Move(_63), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_68 = dump_var(5_usize, 12_usize, Move(_12), 18_usize, Move(_18), 14_usize, Move(_14), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_68 = dump_var(5_usize, 56_usize, Move(_56), 26_usize, Move(_26), 20_usize, Move(_20), 36_usize, Move(_36)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: *mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16),mut _2: *const *mut i128,mut _3: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]),mut _4: *mut i128,mut _5: *const *mut i128) -> *const *mut i128 {
mir! {
type RET = *const *mut i128;
let _6: [u64; 8];
let _7: Adt52;
let _8: [u64; 8];
let _9: (char,);
let _10: Adt51;
let _11: char;
let _12: [u8; 4];
let _13: i64;
let _14: [i32; 7];
let _15: (i32,);
let _16: [char; 2];
let _17: char;
let _18: f64;
let _19: *const u8;
let _20: Adt60;
let _21: *const f32;
let _22: [i8; 3];
let _23: [char; 2];
let _24: isize;
let _25: [u32; 1];
let _26: f32;
let _27: [i32; 7];
let _28: f64;
let _29: [char; 2];
let _30: f32;
let _31: Adt65;
let _32: bool;
let _33: i128;
let _34: ();
let _35: ();
{
(*_2) = _4;
RET = _2;
(*_1).4 = (*_1).3;
(*_1).3 = 111586785855668957101787924315501761301_u128 as i8;
(*_5) = core::ptr::addr_of_mut!((*_4));
(*_1).6 = !(-4456_i16);
(*_1).0 = -27_isize;
(*_1).0 = -9223372036854775807_isize;
(*RET) = (*_1).5;
(*_1).1.0 = 1018962200_i32;
(*_1).2 = core::ptr::addr_of_mut!((*_4));
(*RET) = (*_1).5;
(*_1).1 = (195493973_i32,);
(*_1).2 = core::ptr::addr_of_mut!((*_4));
Goto(bb1)
}
bb1 = {
_2 = core::ptr::addr_of!((*_1).5);
(*_5) = core::ptr::addr_of_mut!((*_4));
(*RET) = core::ptr::addr_of_mut!((*_4));
_2 = core::ptr::addr_of!((*_1).2);
_7.fld0.0 = (*_1).6 - (*_1).6;
(*_1).2 = (*_1).5;
(*_1).3 = 11551317293752068412_usize as i8;
(*_5) = core::ptr::addr_of_mut!((*_4));
_5 = _2;
_4 = core::ptr::addr_of_mut!((*_4));
_6 = [9309183382526114957_u64,5465137683553951755_u64,3679895493286879139_u64,12967595959098938275_u64,13544241564745191488_u64,14895774653802892917_u64,2423037080121601277_u64,623641385423686453_u64];
(*RET) = core::ptr::addr_of_mut!((*_4));
_7.fld0.1 = core::ptr::addr_of_mut!((*_4));
Goto(bb2)
}
bb2 = {
_3.0 = core::ptr::addr_of_mut!((*_1));
_8 = [14230122808896323699_u64,2417601256177061851_u64,8797274948100716841_u64,16529282373295972519_u64,9886210993144365683_u64,6770587712490633500_u64,7476966144672913961_u64,6061714131904697445_u64];
_7.fld0.2 = [(*_1).4,(*_1).4,(*_1).4];
_10.fld1 = (2421582661091846974_u64, _2, _7.fld0.2, _5);
_7.fld0.0 = (*_1).6;
_7.fld0.1 = (*_1).2;
_14 = _3.1;
(*_4) = !(-32737685039057337448630602070564288745_i128);
_1 = core::ptr::addr_of_mut!((*_1));
_7.fld0 = ((*_1).6, (*_1).5, _10.fld1.2);
Goto(bb3)
}
bb3 = {
_7.fld1 = [(*_1).4,(*_1).4,(*_1).4];
_10.fld1.1 = _5;
(*_1).4 = (*_1).3 ^ (*_1).3;
(*_1).6 = !_7.fld0.0;
(*RET) = core::ptr::addr_of_mut!((*_4));
(*_1).4 = _10.fld1.0 as i8;
_10.fld0 = [3109994495_u32];
(*_1).1.0 = 128_u8 as i32;
_10.fld1.1 = _5;
_9.0 = '\u{90827}';
_9.0 = '\u{be14}';
(*_1).1.0 = 2125077791_i32 >> (*_1).3;
_14 = [(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0];
(*_1).0 = (-9223372036854775808_isize);
_10.fld1 = (8230582241505578896_u64, _2, _7.fld1, _5);
_12 = [196_u8,156_u8,86_u8,170_u8];
Call(_7.fld2 = fn7(_5, _7.fld0.2, (*_5), _2, (*_1).0, _2, _3.0, (*_1).3, (*_5), (*_2), (*_4), (*RET), _3, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14 = [(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0];
_8 = _6;
(*_4) = (*_1).0 as i128;
(*_1).2 = _4;
(*_1).2 = (*_1).5;
_15 = ((*_1).1.0,);
_16 = [_9.0,_9.0];
(*_4) = (-92995910613060481820686934837923273830_i128) | 156168033036488128967689648888007080588_i128;
_10.fld1.0 = (*_1).0 as u64;
(*RET) = core::ptr::addr_of_mut!((*_4));
(*_5) = core::ptr::addr_of_mut!((*_4));
_14 = _3.1;
(*_1).6 = -_7.fld0.0;
_20.fld1.fld2 = core::ptr::addr_of!(_20.fld1.fld1.fld0);
(*_1).6 = _7.fld0.0;
(*_1).2 = (*RET);
(*_5) = core::ptr::addr_of_mut!(_20.fld1.fld1.fld1.2);
(*_5) = core::ptr::addr_of_mut!((*_4));
_3.1 = [(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,_15.0];
_20.fld1.fld0.1 = (*_1).1;
_14 = [(*_1).1.0,(*_1).1.0,_20.fld1.fld0.1.0,(*_1).1.0,(*_1).1.0,_15.0,_20.fld1.fld0.1.0];
(*_1).4 = !(*_1).3;
_3.1 = [(*_1).1.0,_20.fld1.fld0.1.0,_15.0,_15.0,_15.0,(*_1).1.0,_15.0];
_20.fld1.fld1.fld1.2 = (*_4) ^ (*_4);
(*_1) = (9223372036854775807_isize, _20.fld1.fld0.1, _7.fld0.1, 119_i8, 110_i8, _4, _7.fld0.0);
(*_1) = (9223372036854775807_isize, _15, _7.fld0.1, (-42_i8), (-84_i8), (*RET), _7.fld0.0);
Goto(bb5)
}
bb5 = {
_13 = -3189536416927450453_i64;
(*_2) = core::ptr::addr_of_mut!(_20.fld1.fld1.fld1.2);
(*_2) = _4;
_18 = _20.fld1.fld1.fld1.2 as f64;
_20.fld1.fld1.fld1.1 = _18 as i32;
_20.fld1.fld0.3 = 136_u8 as i8;
Goto(bb6)
}
bb6 = {
_9 = ('\u{9daed}',);
_17 = _9.0;
_17 = _9.0;
(*_1).2 = core::ptr::addr_of_mut!((*_4));
(*_1).4 = _13 as i8;
(*_1).3 = (*_1).4;
_10.fld1.2 = [(*_1).3,(*_1).4,(*_1).4];
_20.fld3.fld0 = (_7.fld0.0, (*_5), _7.fld0.2);
_25 = [1305438079_u32];
(*_1).2 = core::ptr::addr_of_mut!((*_4));
Goto(bb7)
}
bb7 = {
_20.fld1.fld1.fld1 = (_20.fld3.fld0.2, _15.0, (*_4));
_12 = [168_u8,69_u8,175_u8,61_u8];
_20.fld1.fld0.0 = (*_1).0;
_10.fld2 = core::ptr::addr_of_mut!((*_1));
_20.fld1.fld1.fld0 = 35854_u16 as f32;
(*_4) = _20.fld1.fld1.fld1.2;
_3 = (_1, _14);
_7.fld1 = [(*_1).4,(*_1).3,_20.fld1.fld0.3];
(*_1).0 = 130406341861581673602750348614130995511_u128 as isize;
_18 = _10.fld1.0 as f64;
_20.fld1.fld1.fld1 = (_7.fld1, _20.fld1.fld0.1.0, (*_4));
_22 = [(*_1).4,(*_1).3,(*_1).4];
(*_1).5 = core::ptr::addr_of_mut!((*_4));
match _20.fld1.fld0.0 {
9223372036854775807 => bb9,
_ => bb8
}
}
bb8 = {
_13 = -3189536416927450453_i64;
(*_2) = core::ptr::addr_of_mut!(_20.fld1.fld1.fld1.2);
(*_2) = _4;
_18 = _20.fld1.fld1.fld1.2 as f64;
_20.fld1.fld1.fld1.1 = _18 as i32;
_20.fld1.fld0.3 = 136_u8 as i8;
Goto(bb6)
}
bb9 = {
_21 = _20.fld1.fld2;
_20.fld1.fld1.fld1.2 = (*_4);
_20.fld1.fld2 = _21;
_29 = [_9.0,_17];
RET = core::ptr::addr_of!((*_5));
_29 = _16;
_29 = _16;
(*_1).6 = _20.fld3.fld0.0 >> (*_1).4;
(*_21) = 49_u8 as f32;
Goto(bb10)
}
bb10 = {
(*_1) = (_20.fld1.fld0.0, _15, _7.fld0.1, _20.fld1.fld0.3, _20.fld1.fld0.3, _7.fld0.1, _20.fld3.fld0.0);
_26 = (*_21) * (*_21);
_6 = [_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0];
_23 = _16;
_20.fld3.fld0.2 = _7.fld0.2;
_7.fld0 = _20.fld3.fld0;
_7.fld0.0 = -(*_1).6;
_20.fld3 = Adt52 { fld0: _7.fld0,fld1: _7.fld0.2,fld2: _7.fld2 };
_1 = core::ptr::addr_of_mut!((*_1));
_20.fld1.fld0.0 = (*_1).0 >> _20.fld1.fld1.fld1.1;
_22 = [(*_1).3,(*_1).4,(*_1).3];
_31.fld5.fld1.fld1.fld1 = _20.fld1.fld1.fld1;
_20.fld1.fld3 = (*_1).0 as i64;
(*_1).1 = (_20.fld1.fld0.1.0,);
_7.fld0.1 = core::ptr::addr_of_mut!(_31.fld5.fld1.fld1.fld1.2);
(*_1).2 = (*_1).5;
_6 = [_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0,_10.fld1.0];
_10.fld1.1 = core::ptr::addr_of!((*RET));
_7.fld1 = _7.fld0.2;
_10.fld2 = _1;
RET = core::ptr::addr_of!(_31.fld5.fld1.fld0.2);
(*_21) = -_26;
_31.fld5.fld1.fld1.fld1.1 = 174_u8 as i32;
match (*_1).0 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
9223372036854775807 => bb19,
_ => bb18
}
}
bb11 = {
_21 = _20.fld1.fld2;
_20.fld1.fld1.fld1.2 = (*_4);
_20.fld1.fld2 = _21;
_29 = [_9.0,_17];
RET = core::ptr::addr_of!((*_5));
_29 = _16;
_29 = _16;
(*_1).6 = _20.fld3.fld0.0 >> (*_1).4;
(*_21) = 49_u8 as f32;
Goto(bb10)
}
bb12 = {
_2 = core::ptr::addr_of!((*_1).5);
(*_5) = core::ptr::addr_of_mut!((*_4));
(*RET) = core::ptr::addr_of_mut!((*_4));
_2 = core::ptr::addr_of!((*_1).2);
_7.fld0.0 = (*_1).6 - (*_1).6;
(*_1).2 = (*_1).5;
(*_1).3 = 11551317293752068412_usize as i8;
(*_5) = core::ptr::addr_of_mut!((*_4));
_5 = _2;
_4 = core::ptr::addr_of_mut!((*_4));
_6 = [9309183382526114957_u64,5465137683553951755_u64,3679895493286879139_u64,12967595959098938275_u64,13544241564745191488_u64,14895774653802892917_u64,2423037080121601277_u64,623641385423686453_u64];
(*RET) = core::ptr::addr_of_mut!((*_4));
_7.fld0.1 = core::ptr::addr_of_mut!((*_4));
Goto(bb2)
}
bb13 = {
_20.fld1.fld1.fld1 = (_20.fld3.fld0.2, _15.0, (*_4));
_12 = [168_u8,69_u8,175_u8,61_u8];
_20.fld1.fld0.0 = (*_1).0;
_10.fld2 = core::ptr::addr_of_mut!((*_1));
_20.fld1.fld1.fld0 = 35854_u16 as f32;
(*_4) = _20.fld1.fld1.fld1.2;
_3 = (_1, _14);
_7.fld1 = [(*_1).4,(*_1).3,_20.fld1.fld0.3];
(*_1).0 = 130406341861581673602750348614130995511_u128 as isize;
_18 = _10.fld1.0 as f64;
_20.fld1.fld1.fld1 = (_7.fld1, _20.fld1.fld0.1.0, (*_4));
_22 = [(*_1).4,(*_1).3,(*_1).4];
(*_1).5 = core::ptr::addr_of_mut!((*_4));
match _20.fld1.fld0.0 {
9223372036854775807 => bb9,
_ => bb8
}
}
bb14 = {
_9 = ('\u{9daed}',);
_17 = _9.0;
_17 = _9.0;
(*_1).2 = core::ptr::addr_of_mut!((*_4));
(*_1).4 = _13 as i8;
(*_1).3 = (*_1).4;
_10.fld1.2 = [(*_1).3,(*_1).4,(*_1).4];
_20.fld3.fld0 = (_7.fld0.0, (*_5), _7.fld0.2);
_25 = [1305438079_u32];
(*_1).2 = core::ptr::addr_of_mut!((*_4));
Goto(bb7)
}
bb15 = {
_13 = -3189536416927450453_i64;
(*_2) = core::ptr::addr_of_mut!(_20.fld1.fld1.fld1.2);
(*_2) = _4;
_18 = _20.fld1.fld1.fld1.2 as f64;
_20.fld1.fld1.fld1.1 = _18 as i32;
_20.fld1.fld0.3 = 136_u8 as i8;
Goto(bb6)
}
bb16 = {
_14 = [(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0];
_8 = _6;
(*_4) = (*_1).0 as i128;
(*_1).2 = _4;
(*_1).2 = (*_1).5;
_15 = ((*_1).1.0,);
_16 = [_9.0,_9.0];
(*_4) = (-92995910613060481820686934837923273830_i128) | 156168033036488128967689648888007080588_i128;
_10.fld1.0 = (*_1).0 as u64;
(*RET) = core::ptr::addr_of_mut!((*_4));
(*_5) = core::ptr::addr_of_mut!((*_4));
_14 = _3.1;
(*_1).6 = -_7.fld0.0;
_20.fld1.fld2 = core::ptr::addr_of!(_20.fld1.fld1.fld0);
(*_1).6 = _7.fld0.0;
(*_1).2 = (*RET);
(*_5) = core::ptr::addr_of_mut!(_20.fld1.fld1.fld1.2);
(*_5) = core::ptr::addr_of_mut!((*_4));
_3.1 = [(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,_15.0];
_20.fld1.fld0.1 = (*_1).1;
_14 = [(*_1).1.0,(*_1).1.0,_20.fld1.fld0.1.0,(*_1).1.0,(*_1).1.0,_15.0,_20.fld1.fld0.1.0];
(*_1).4 = !(*_1).3;
_3.1 = [(*_1).1.0,_20.fld1.fld0.1.0,_15.0,_15.0,_15.0,(*_1).1.0,_15.0];
_20.fld1.fld1.fld1.2 = (*_4) ^ (*_4);
(*_1) = (9223372036854775807_isize, _20.fld1.fld0.1, _7.fld0.1, 119_i8, 110_i8, _4, _7.fld0.0);
(*_1) = (9223372036854775807_isize, _15, _7.fld0.1, (-42_i8), (-84_i8), (*RET), _7.fld0.0);
Goto(bb5)
}
bb17 = {
_7.fld1 = [(*_1).4,(*_1).4,(*_1).4];
_10.fld1.1 = _5;
(*_1).4 = (*_1).3 ^ (*_1).3;
(*_1).6 = !_7.fld0.0;
(*RET) = core::ptr::addr_of_mut!((*_4));
(*_1).4 = _10.fld1.0 as i8;
_10.fld0 = [3109994495_u32];
(*_1).1.0 = 128_u8 as i32;
_10.fld1.1 = _5;
_9.0 = '\u{90827}';
_9.0 = '\u{be14}';
(*_1).1.0 = 2125077791_i32 >> (*_1).3;
_14 = [(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0,(*_1).1.0];
(*_1).0 = (-9223372036854775808_isize);
_10.fld1 = (8230582241505578896_u64, _2, _7.fld1, _5);
_12 = [196_u8,156_u8,86_u8,170_u8];
Call(_7.fld2 = fn7(_5, _7.fld0.2, (*_5), _2, (*_1).0, _2, _3.0, (*_1).3, (*_5), (*_2), (*_4), (*RET), _3, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb18 = {
_3.0 = core::ptr::addr_of_mut!((*_1));
_8 = [14230122808896323699_u64,2417601256177061851_u64,8797274948100716841_u64,16529282373295972519_u64,9886210993144365683_u64,6770587712490633500_u64,7476966144672913961_u64,6061714131904697445_u64];
_7.fld0.2 = [(*_1).4,(*_1).4,(*_1).4];
_10.fld1 = (2421582661091846974_u64, _2, _7.fld0.2, _5);
_7.fld0.0 = (*_1).6;
_7.fld0.1 = (*_1).2;
_14 = _3.1;
(*_4) = !(-32737685039057337448630602070564288745_i128);
_1 = core::ptr::addr_of_mut!((*_1));
_7.fld0 = ((*_1).6, (*_1).5, _10.fld1.2);
Goto(bb3)
}
bb19 = {
_31.fld5.fld3 = _7;
_7.fld1 = _7.fld0.2;
Goto(bb20)
}
bb20 = {
Call(_34 = dump_var(6_usize, 15_usize, Move(_15), 9_usize, Move(_9), 12_usize, Move(_12), 16_usize, Move(_16)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_34 = dump_var(6_usize, 22_usize, Move(_22), 13_usize, Move(_13), 35_usize, _35, 35_usize, _35), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *const *mut i128,mut _2: [i8; 3],mut _3: *mut i128,mut _4: *const *mut i128,mut _5: isize,mut _6: *const *mut i128,mut _7: *mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16),mut _8: i8,mut _9: *mut i128,mut _10: *mut i128,mut _11: i128,mut _12: *mut i128,mut _13: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]),mut _14: [u64; 8]) -> *mut ([i8; 3], i32, i128) {
mir! {
type RET = *mut ([i8; 3], i32, i128);
let _15: f64;
let _16: [i32; 7];
let _17: *mut u64;
let _18: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]);
let _19: *const *mut i128;
let _20: i128;
let _21: isize;
let _22: [i32; 7];
let _23: Adt61;
let _24: [u32; 1];
let _25: f32;
let _26: u64;
let _27: ();
let _28: ();
{
(*_7).1.0 = 4642919714763146222_i64 as i32;
(*_7).2 = core::ptr::addr_of_mut!((*_12));
(*_7).0 = _5;
(*_7).6 = '\u{10296c}' as i16;
_3 = core::ptr::addr_of_mut!((*_12));
(*_12) = -_11;
_15 = 34022_u16 as f64;
(*_7).0 = 259158131833056777484373971119528507796_u128 as isize;
(*_7).5 = core::ptr::addr_of_mut!((*_9));
_8 = (*_7).0 as i8;
_12 = core::ptr::addr_of_mut!((*_9));
(*_7).5 = core::ptr::addr_of_mut!((*_10));
(*_7).3 = !(*_7).4;
Call((*_7).3 = fn8((*_4), (*_7).2, (*_7).6, _7, (*_7).0, _1, (*_7).5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_7).4 = (*_7).0 as i8;
(*_7).0 = _5 + _5;
(*_7).4 = (*_7).3 - (*_7).3;
(*_3) = _15 as i128;
_3 = core::ptr::addr_of_mut!((*_3));
(*_7).1.0 = 1605062742_i32;
_10 = _9;
(*_4) = core::ptr::addr_of_mut!((*_9));
_2 = [(*_7).3,(*_7).3,(*_7).4];
_16 = _13.1;
_14 = [10634831303994506926_u64,8262098135499279469_u64,5991549048456348820_u64,6687719503495040508_u64,3278855946795304098_u64,10700654515226075155_u64,15401490279563883428_u64,6215616901097036786_u64];
(*_3) = _11;
(*_7).4 = !(*_7).3;
_1 = core::ptr::addr_of!((*_7).5);
(*_1) = core::ptr::addr_of_mut!((*_3));
_1 = core::ptr::addr_of!((*_7).5);
(*_7).0 = _5;
(*_7).4 = (*_7).3 & (*_7).3;
_5 = -(*_7).0;
match (*_7).1.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1605062742 => bb7,
_ => bb6
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
(*_7).3 = (*_7).6 as i8;
(*_9) = _11;
_18.1 = [(*_7).1.0,(*_7).1.0,(*_7).1.0,(*_7).1.0,(*_7).1.0,(*_7).1.0,(*_7).1.0];
(*_7).4 = -(*_7).3;
_14 = [14955531921828147122_u64,2166331980722882702_u64,3618612675562008095_u64,17240019650640386236_u64,10110095208709428699_u64,8508447544362161107_u64,4882019681361641185_u64,6192255004819861205_u64];
(*_7).2 = _10;
_18.0 = core::ptr::addr_of_mut!((*_7));
_13 = (_7, _16);
(*_7).3 = _8 >> (*_10);
Call((*_7).2 = fn9(_4, _7, _6, _4, (*_7).1, (*_9), (*_1), (*_9), (*_7).0, (*_9), (*_1), (*_7).1.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_12) = _11 + _11;
(*_9) = _11 ^ _11;
_4 = core::ptr::addr_of!((*_4));
(*_7).5 = core::ptr::addr_of_mut!((*_3));
_4 = core::ptr::addr_of!((*_1));
(*_7).0 = -_5;
_14 = [5426468822395656414_u64,17307640705988696868_u64,4263861702254801764_u64,2661569298744588338_u64,13299412938070284822_u64,15275543809845541618_u64,15058820141701279374_u64,17231751483771830420_u64];
_22 = _13.1;
(*_7).4 = (*_7).3;
(*_7).5 = core::ptr::addr_of_mut!((*_9));
_19 = core::ptr::addr_of!((*_1));
_23.fld0.2 = 7_usize as i128;
_23.fld3.0 = core::ptr::addr_of_mut!(_23.fld2.fld0);
_23.fld4.fld1 = (9520679269460563507_u64, _19, _2, _19);
_23.fld0 = (_2, (*_7).1.0, (*_10));
_23.fld1 = [2967016806_u32];
(*_9) = -_11;
(*_7).5 = core::ptr::addr_of_mut!((*_9));
(*_10) = _15 as i128;
_23.fld2 = Adt50 { fld0: (-6084044787135759956_i64),fld1: '\u{b936}',fld2: _6 };
(*_7).5 = core::ptr::addr_of_mut!((*_10));
_18.0 = _13.0;
RET = core::ptr::addr_of_mut!(_23.fld0);
_24 = _23.fld1;
Goto(bb9)
}
bb9 = {
Call(_27 = dump_var(7_usize, 2_usize, Move(_2), 16_usize, Move(_16), 5_usize, Move(_5), 14_usize, Move(_14)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: *mut i128,mut _2: *mut i128,mut _3: i16,mut _4: *mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16),mut _5: isize,mut _6: *const *mut i128,mut _7: *mut i128) -> i8 {
mir! {
type RET = i8;
let _8: (i32,);
let _9: f32;
let _10: isize;
let _11: bool;
let _12: ();
let _13: ();
{
(*_6) = (*_4).5;
Goto(bb1)
}
bb1 = {
(*_4).6 = _3;
RET = (*_4).4;
(*_4).5 = core::ptr::addr_of_mut!((*_2));
(*_4).6 = 56562387123388213169734767456736418561_u128 as i16;
_8 = ((*_4).1.0,);
(*_4).4 = (-126_i8);
(*_4).2 = core::ptr::addr_of_mut!((*_7));
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(8_usize, 8_usize, Move(_8), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: *const *mut i128,mut _2: *mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16),mut _3: *const *mut i128,mut _4: *const *mut i128,mut _5: (i32,),mut _6: i128,mut _7: *mut i128,mut _8: i128,mut _9: isize,mut _10: i128,mut _11: *mut i128,mut _12: i32) -> *mut i128 {
mir! {
type RET = *mut i128;
let _13: isize;
let _14: [i8; 3];
let _15: usize;
let _16: char;
let _17: [i32; 7];
let _18: u8;
let _19: isize;
let _20: (i32,);
let _21: [u64; 8];
let _22: [u32; 1];
let _23: u32;
let _24: char;
let _25: [u64; 8];
let _26: ([i8; 3], i32, i128);
let _27: [u64; 8];
let _28: [i8; 3];
let _29: ([i8; 3], i32, i128);
let _30: ();
let _31: ();
{
(*_2).6 = 29234_i16 ^ 10741_i16;
(*_11) = (*_2).1.0 as i128;
Goto(bb1)
}
bb1 = {
(*_2).5 = _11;
_6 = _8;
(*_7) = _10 + _8;
_6 = 5722917544125868624_u64 as i128;
(*_2).4 = !(*_2).3;
_14 = [(*_2).3,(*_2).4,(*_2).4];
(*_2).6 = 12712_i16 >> _5.0;
_8 = 215550622537787360771942136355897342308_u128 as i128;
(*_7) = (*_2).3 as i128;
(*_2).4 = -(*_2).3;
match (*_2).0 {
0 => bb2,
340282366920938463454151235394913435648 => bb4,
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
(*_7) = (*_2).1.0 as i128;
(*_11) = _6;
RET = core::ptr::addr_of_mut!((*_11));
_13 = _9 + (*_2).0;
_8 = (*RET);
(*_2).3 = (*_2).4 >> (*_7);
_12 = (*_2).1.0;
Call(_17 = fn10((*_2).0, (*_2).1.0, (*_2).0, _13, _7, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = (*_11) as isize;
(*_2).0 = _9 + _13;
_15 = !6826179500202816757_usize;
_5 = ((*_2).1.0,);
(*_7) = _6 << (*_2).4;
_3 = core::ptr::addr_of!(_11);
(*_3) = core::ptr::addr_of_mut!((*_7));
(*_2).1.0 = '\u{7f9d9}' as i32;
(*_11) = -_6;
Goto(bb6)
}
bb6 = {
(*_7) = _10 << (*_2).0;
(*_2).3 = (*_2).4 | (*_2).4;
_15 = 147_u8 as usize;
_11 = core::ptr::addr_of_mut!((*_7));
_9 = -_13;
_16 = '\u{6fe03}';
(*_2).0 = !_13;
_4 = core::ptr::addr_of!((*_3));
(*_3) = core::ptr::addr_of_mut!((*_7));
_21 = [4043502775085634454_u64,11136337981012627876_u64,15989614032192350083_u64,856662933997943685_u64,13253311688470892742_u64,9288274632810079665_u64,16400740142880038434_u64,8042181655325189734_u64];
(*_7) = _6;
_5.0 = _12;
_20 = ((*_2).1.0,);
_17 = [(*_2).1.0,_5.0,_20.0,_20.0,(*_2).1.0,_5.0,_5.0];
(*_7) = _10 >> (*_2).6;
(*_2).4 = (*_2).3;
(*_2).0 = !_9;
_4 = core::ptr::addr_of!((*_3));
_4 = core::ptr::addr_of!((*_3));
(*_2).1 = (_20.0,);
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
1605062742 => bb9,
_ => bb8
}
}
bb7 = {
(*_2).5 = _11;
_6 = _8;
(*_7) = _10 + _8;
_6 = 5722917544125868624_u64 as i128;
(*_2).4 = !(*_2).3;
_14 = [(*_2).3,(*_2).4,(*_2).4];
(*_2).6 = 12712_i16 >> _5.0;
_8 = 215550622537787360771942136355897342308_u128 as i128;
(*_7) = (*_2).3 as i128;
(*_2).4 = -(*_2).3;
match (*_2).0 {
0 => bb2,
340282366920938463454151235394913435648 => bb4,
_ => bb3
}
}
bb8 = {
(*_7) = (*_2).1.0 as i128;
(*_11) = _6;
RET = core::ptr::addr_of_mut!((*_11));
_13 = _9 + (*_2).0;
_8 = (*RET);
(*_2).3 = (*_2).4 >> (*_7);
_12 = (*_2).1.0;
Call(_17 = fn10((*_2).0, (*_2).1.0, (*_2).0, _13, _7, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_19 = (*_2).0;
(*_3) = core::ptr::addr_of_mut!((*_11));
_12 = _5.0 ^ (*_2).1.0;
match _5.0 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb6,
5 => bb10,
6 => bb11,
1605062742 => bb13,
_ => bb12
}
}
bb10 = {
(*_7) = (*_2).1.0 as i128;
(*_11) = _6;
RET = core::ptr::addr_of_mut!((*_11));
_13 = _9 + (*_2).0;
_8 = (*RET);
(*_2).3 = (*_2).4 >> (*_7);
_12 = (*_2).1.0;
Call(_17 = fn10((*_2).0, (*_2).1.0, (*_2).0, _13, _7, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
(*_7) = _10 << (*_2).0;
(*_2).3 = (*_2).4 | (*_2).4;
_15 = 147_u8 as usize;
_11 = core::ptr::addr_of_mut!((*_7));
_9 = -_13;
_16 = '\u{6fe03}';
(*_2).0 = !_13;
_4 = core::ptr::addr_of!((*_3));
(*_3) = core::ptr::addr_of_mut!((*_7));
_21 = [4043502775085634454_u64,11136337981012627876_u64,15989614032192350083_u64,856662933997943685_u64,13253311688470892742_u64,9288274632810079665_u64,16400740142880038434_u64,8042181655325189734_u64];
(*_7) = _6;
_5.0 = _12;
_20 = ((*_2).1.0,);
_17 = [(*_2).1.0,_5.0,_20.0,_20.0,(*_2).1.0,_5.0,_5.0];
(*_7) = _10 >> (*_2).6;
(*_2).4 = (*_2).3;
(*_2).0 = !_9;
_4 = core::ptr::addr_of!((*_3));
_4 = core::ptr::addr_of!((*_3));
(*_2).1 = (_20.0,);
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
1605062742 => bb9,
_ => bb8
}
}
bb13 = {
_9 = _13 >> (*_11);
(*_2).1 = _5;
_22 = [3838445854_u32];
(*_2).1 = _5;
_23 = 196528898_u32 * 293280042_u32;
_26.0 = [(*_2).3,(*_2).3,(*_2).3];
(*_2).6 = 26906_i16;
(*RET) = _10 + _10;
_24 = _16;
(*RET) = _6;
(*_2).4 = (*_2).3;
RET = (*_2).5;
_4 = _1;
RET = (*_3);
_27 = _21;
(*_2).1 = (_5.0,);
(*_2).1 = _5;
(*_2).6 = _15 as i16;
Goto(bb14)
}
bb14 = {
_27 = _21;
_26.2 = -_6;
_26 = (_14, (*_2).1.0, _6);
(*_2).0 = _9;
(*_2).6 = 25053_i16;
_22 = [_23];
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(9_usize, 6_usize, Move(_6), 8_usize, Move(_8), 27_usize, Move(_27), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(9_usize, 21_usize, Move(_21), 10_usize, Move(_10), 17_usize, Move(_17), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(9_usize, 19_usize, Move(_19), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: i32,mut _3: isize,mut _4: isize,mut _5: *mut i128,mut _6: *mut i128) -> [i32; 7] {
mir! {
type RET = [i32; 7];
let _7: [char; 2];
let _8: (*mut (i16, *mut i128, [i8; 3]), i32);
let _9: (i16, *mut i128, [i8; 3]);
let _10: *mut ([i8; 3], i32, i128);
let _11: (char,);
let _12: [u32; 1];
let _13: i64;
let _14: i64;
let _15: i64;
let _16: f64;
let _17: *mut u64;
let _18: f64;
let _19: *mut bool;
let _20: ([i8; 3], i32, i128);
let _21: Adt55;
let _22: ([i8; 3], i32, i128);
let _23: *const f32;
let _24: Adt50;
let _25: ();
let _26: ();
{
RET = [_2,_2,_2,_2,_2,_2,_2];
(*_6) = !83623348941089130059149268287713390550_i128;
_5 = core::ptr::addr_of_mut!((*_5));
match _1 {
0 => bb1,
340282366920938463454151235394913435648 => bb3,
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
_3 = 3651614718303674958_u64 as isize;
_1 = _4 + _4;
Goto(bb4)
}
bb4 = {
_3 = _4 & _4;
_8.1 = _2 << (*_5);
_1 = _3;
_9.0 = (-18690_i16);
_9.1 = core::ptr::addr_of_mut!((*_5));
_8.1 = _2;
_9.0 = (-37_i8) as i16;
_8.0 = core::ptr::addr_of_mut!(_9);
_9.2 = [30_i8,57_i8,35_i8];
_11 = ('\u{6e985}',);
(*_5) = (-86472645971599804742564035038523446775_i128);
_5 = core::ptr::addr_of_mut!((*_6));
_5 = core::ptr::addr_of_mut!((*_6));
_5 = core::ptr::addr_of_mut!((*_6));
_6 = core::ptr::addr_of_mut!((*_6));
_12 = [257047108_u32];
_13 = (-701527579054719990_i64) << (*_6);
(*_5) = (-32715703994558446547461664023288232822_i128);
Goto(bb5)
}
bb5 = {
_13 = 1414142417457438873_i64 << _3;
_11 = ('\u{91c85}',);
(*_5) = (-81_i8) as i128;
_7 = [_11.0,_11.0];
RET = [_8.1,_2,_2,_8.1,_2,_8.1,_8.1];
_9.0 = !(-6237_i16);
_4 = !_3;
RET = [_8.1,_8.1,_8.1,_2,_8.1,_8.1,_8.1];
_8.1 = !_2;
_4 = _1 * _3;
_11.0 = '\u{8c2}';
_2 = _13 as i32;
_11 = ('\u{100514}',);
_11.0 = '\u{afc5f}';
_4 = !_1;
_9.2 = [56_i8,51_i8,54_i8];
_9.0 = (-23672_i16) & (-13721_i16);
_9.0 = 285270624899248307735225877446748312448_u128 as i16;
_9.0 = _1 as i16;
_9.1 = core::ptr::addr_of_mut!((*_6));
Goto(bb6)
}
bb6 = {
_9.1 = _6;
(*_6) = (-5710329092802600273253767941379300025_i128) - (-54339502419977101686704505780891456089_i128);
Call(_9.2 = fn11((*_5), _6, _4, _4, (*_6), _11, _3, _8, _8, _9.1, _9.1, _1, _13, _1, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_13 = -(-2755284854532798116_i64);
_5 = core::ptr::addr_of_mut!((*_6));
_13 = !(-7074834966066253679_i64);
_5 = core::ptr::addr_of_mut!((*_6));
(*_6) = 83601248194478464382940759209375190177_i128 | (-98991513028445039214850287384911583718_i128);
_7 = [_11.0,_11.0];
_11 = ('\u{7b32a}',);
Goto(bb8)
}
bb8 = {
RET = [_2,_8.1,_8.1,_2,_2,_2,_2];
_5 = core::ptr::addr_of_mut!((*_5));
_14 = !_13;
_12 = [1695415389_u32];
(*_6) = 40_i8 as i128;
Goto(bb9)
}
bb9 = {
_1 = 208_u8 as isize;
_8.1 = _11.0 as i32;
_7 = [_11.0,_11.0];
(*_5) = 85432611935700677488495595865730367004_i128 << _3;
_9.2 = [(-50_i8),33_i8,(-24_i8)];
_9.1 = core::ptr::addr_of_mut!((*_6));
_9.2 = [(-20_i8),16_i8,(-112_i8)];
_14 = 48965_u16 as i64;
_11.0 = '\u{a4e76}';
_8.0 = core::ptr::addr_of_mut!(_9);
_7 = [_11.0,_11.0];
_9.0 = (-3739_i16);
_9.1 = core::ptr::addr_of_mut!((*_5));
_7 = [_11.0,_11.0];
Goto(bb10)
}
bb10 = {
_9.0 = !(-9836_i16);
_1 = _3;
RET = [_2,_2,_2,_2,_8.1,_8.1,_2];
_9.1 = core::ptr::addr_of_mut!((*_6));
_9.2 = [120_i8,17_i8,91_i8];
RET = [_2,_2,_2,_2,_8.1,_8.1,_2];
(*_6) = (-121772401669886989104442864087524371118_i128);
_14 = 198_u8 as i64;
_9.0 = _11.0 as i16;
(*_6) = -(-104045428371047457107191043228411386491_i128);
(*_6) = 67540878214998288381012228140232379608_u128 as i128;
_9.2 = [34_i8,(-61_i8),(-92_i8)];
(*_5) = 7866107678520444959_usize as i128;
_2 = -_8.1;
_9.1 = _5;
_9.1 = core::ptr::addr_of_mut!((*_6));
RET = [_2,_2,_2,_2,_2,_2,_8.1];
_11 = ('\u{106147}',);
_9.2 = [8_i8,(-33_i8),(-11_i8)];
_12 = [3437568815_u32];
(*_5) = (-65603403428515710944667190295109134717_i128);
_4 = _3 & _3;
_16 = _13 as f64;
RET = [_2,_8.1,_2,_2,_8.1,_2,_8.1];
_6 = core::ptr::addr_of_mut!((*_6));
_15 = _11.0 as i64;
Call(_16 = fn19((*_6), _12, (*_5), _9.0, _2, _12, _9.1, _9.1, _11.0, _13), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = _9.1;
_13 = (*_6) as i64;
(*_5) = 25433943593071295473510020159089074342_i128 - (-19251603092277946687838950705582458423_i128);
_1 = _4 | _4;
_6 = core::ptr::addr_of_mut!((*_5));
_3 = -_1;
_9.1 = core::ptr::addr_of_mut!((*_6));
_11 = ('\u{49188}',);
_18 = 2_u8 as f64;
RET = [_2,_2,_2,_8.1,_2,_8.1,_2];
_18 = -_16;
_9.0 = (-16784_i16);
_6 = core::ptr::addr_of_mut!((*_5));
_10 = core::ptr::addr_of_mut!(_20);
(*_10).1 = !_2;
(*_10).2 = (*_5);
_8.1 = (*_10).1 | (*_10).1;
(*_10).1 = 0_usize as i32;
_21.fld0.fld6.fld1.1 = core::ptr::addr_of!(_6);
_21.fld1.fld1 = (_8.1,);
_21.fld0.fld1 = [148_u8,243_u8,41_u8,120_u8];
_21.fld0.fld2.fld0 = (*_10).2 as i64;
_20.0 = [62_i8,(-110_i8),14_i8];
_21.fld0.fld6.fld1.2 = (*_10).0;
_20 = (_9.2, _8.1, (*_5));
_10 = core::ptr::addr_of_mut!((*_10));
_21.fld0.fld3.1 = [(*_10).1,_8.1,(*_10).1,_20.1,(*_10).1,_21.fld1.fld1.0,_2];
match _9.0 {
340282366920938463463374607431768194672 => bb12,
_ => bb9
}
}
bb12 = {
RET = _21.fld0.fld3.1;
_15 = _9.0 as i64;
_21.fld0.fld6.fld1.0 = !18207480270778999984_u64;
_9.2 = [(-66_i8),51_i8,(-30_i8)];
_16 = _18 + _18;
_16 = _18 - _18;
_21.fld0.fld6.fld0 = [3605480405_u32];
_20.0 = _9.2;
_20.1 = _21.fld0.fld6.fld1.0 as i32;
_5 = _9.1;
_21.fld1.fld0.1 = [(*_10).1,_20.1,_8.1,_20.1,_21.fld1.fld1.0,_8.1,_21.fld1.fld1.0];
RET = [_21.fld1.fld1.0,(*_10).1,_2,(*_10).1,_20.1,_20.1,_8.1];
match _9.0 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
340282366920938463463374607431768194672 => bb20,
_ => bb19
}
}
bb13 = {
_6 = _9.1;
_13 = (*_6) as i64;
(*_5) = 25433943593071295473510020159089074342_i128 - (-19251603092277946687838950705582458423_i128);
_1 = _4 | _4;
_6 = core::ptr::addr_of_mut!((*_5));
_3 = -_1;
_9.1 = core::ptr::addr_of_mut!((*_6));
_11 = ('\u{49188}',);
_18 = 2_u8 as f64;
RET = [_2,_2,_2,_8.1,_2,_8.1,_2];
_18 = -_16;
_9.0 = (-16784_i16);
_6 = core::ptr::addr_of_mut!((*_5));
_10 = core::ptr::addr_of_mut!(_20);
(*_10).1 = !_2;
(*_10).2 = (*_5);
_8.1 = (*_10).1 | (*_10).1;
(*_10).1 = 0_usize as i32;
_21.fld0.fld6.fld1.1 = core::ptr::addr_of!(_6);
_21.fld1.fld1 = (_8.1,);
_21.fld0.fld1 = [148_u8,243_u8,41_u8,120_u8];
_21.fld0.fld2.fld0 = (*_10).2 as i64;
_20.0 = [62_i8,(-110_i8),14_i8];
_21.fld0.fld6.fld1.2 = (*_10).0;
_20 = (_9.2, _8.1, (*_5));
_10 = core::ptr::addr_of_mut!((*_10));
_21.fld0.fld3.1 = [(*_10).1,_8.1,(*_10).1,_20.1,(*_10).1,_21.fld1.fld1.0,_2];
match _9.0 {
340282366920938463463374607431768194672 => bb12,
_ => bb9
}
}
bb14 = {
_3 = _4 & _4;
_8.1 = _2 << (*_5);
_1 = _3;
_9.0 = (-18690_i16);
_9.1 = core::ptr::addr_of_mut!((*_5));
_8.1 = _2;
_9.0 = (-37_i8) as i16;
_8.0 = core::ptr::addr_of_mut!(_9);
_9.2 = [30_i8,57_i8,35_i8];
_11 = ('\u{6e985}',);
(*_5) = (-86472645971599804742564035038523446775_i128);
_5 = core::ptr::addr_of_mut!((*_6));
_5 = core::ptr::addr_of_mut!((*_6));
_5 = core::ptr::addr_of_mut!((*_6));
_6 = core::ptr::addr_of_mut!((*_6));
_12 = [257047108_u32];
_13 = (-701527579054719990_i64) << (*_6);
(*_5) = (-32715703994558446547461664023288232822_i128);
Goto(bb5)
}
bb15 = {
_1 = 208_u8 as isize;
_8.1 = _11.0 as i32;
_7 = [_11.0,_11.0];
(*_5) = 85432611935700677488495595865730367004_i128 << _3;
_9.2 = [(-50_i8),33_i8,(-24_i8)];
_9.1 = core::ptr::addr_of_mut!((*_6));
_9.2 = [(-20_i8),16_i8,(-112_i8)];
_14 = 48965_u16 as i64;
_11.0 = '\u{a4e76}';
_8.0 = core::ptr::addr_of_mut!(_9);
_7 = [_11.0,_11.0];
_9.0 = (-3739_i16);
_9.1 = core::ptr::addr_of_mut!((*_5));
_7 = [_11.0,_11.0];
Goto(bb10)
}
bb16 = {
_3 = 3651614718303674958_u64 as isize;
_1 = _4 + _4;
Goto(bb4)
}
bb17 = {
_13 = -(-2755284854532798116_i64);
_5 = core::ptr::addr_of_mut!((*_6));
_13 = !(-7074834966066253679_i64);
_5 = core::ptr::addr_of_mut!((*_6));
(*_6) = 83601248194478464382940759209375190177_i128 | (-98991513028445039214850287384911583718_i128);
_7 = [_11.0,_11.0];
_11 = ('\u{7b32a}',);
Goto(bb8)
}
bb18 = {
_9.1 = _6;
(*_6) = (-5710329092802600273253767941379300025_i128) - (-54339502419977101686704505780891456089_i128);
Call(_9.2 = fn11((*_5), _6, _4, _4, (*_6), _11, _3, _8, _8, _9.1, _9.1, _1, _13, _1, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb19 = {
Return()
}
bb20 = {
_21.fld0.fld7 = [_20.1,_21.fld1.fld1.0,_8.1,_21.fld1.fld1.0,_21.fld1.fld1.0,_21.fld1.fld1.0,_8.1];
(*_5) = (*_10).2;
_21.fld0.fld2.fld2 = core::ptr::addr_of!(_5);
(*_10) = (_21.fld0.fld6.fld1.2, _21.fld1.fld1.0, (*_6));
_12 = _21.fld0.fld6.fld0;
_21.fld0.fld6.fld1.3 = core::ptr::addr_of!(_5);
_17 = core::ptr::addr_of_mut!(_21.fld0.fld6.fld1.0);
_18 = _16;
_21.fld0.fld2.fld2 = core::ptr::addr_of!(_5);
_21.fld0.fld2.fld2 = core::ptr::addr_of!(_6);
_6 = core::ptr::addr_of_mut!((*_6));
_11.0 = '\u{754d5}';
(*_17) = 16485051190656502497_u64;
_3 = !_1;
_21.fld0.fld4 = _9.0;
_3 = _1 ^ _4;
_7 = [_11.0,_11.0];
_21.fld0.fld6.fld1.2 = [69_i8,(-126_i8),40_i8];
_22 = (*_10);
_21.fld0.fld6.fld1.0 = !18260584664471552185_u64;
Goto(bb21)
}
bb21 = {
Call(_25 = dump_var(10_usize, 3_usize, Move(_3), 11_usize, Move(_11), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_25 = dump_var(10_usize, 12_usize, Move(_12), 2_usize, Move(_2), 26_usize, _26, 26_usize, _26), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: i128,mut _2: *mut i128,mut _3: isize,mut _4: isize,mut _5: i128,mut _6: (char,),mut _7: isize,mut _8: (*mut (i16, *mut i128, [i8; 3]), i32),mut _9: (*mut (i16, *mut i128, [i8; 3]), i32),mut _10: *mut i128,mut _11: *mut i128,mut _12: isize,mut _13: i64,mut _14: isize,mut _15: *mut i128) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _16: *const f32;
let _17: u32;
let _18: [char; 2];
let _19: isize;
let _20: [u64; 8];
let _21: (i16, *mut i128, [i8; 3]);
let _22: isize;
let _23: isize;
let _24: isize;
let _25: i64;
let _26: isize;
let _27: [u32; 1];
let _28: i16;
let _29: i128;
let _30: [i32; 7];
let _31: ([i8; 3], i32, i128);
let _32: [u32; 1];
let _33: Adt56;
let _34: (*mut (i16, *mut i128, [i8; 3]), i32);
let _35: ([char; 2], [u64; 8], bool);
let _36: [i16; 6];
let _37: i128;
let _38: isize;
let _39: [i32; 7];
let _40: i64;
let _41: [i32; 7];
let _42: Adt58;
let _43: ();
let _44: ();
{
_4 = _7;
(*_11) = 149_u8 as i128;
_15 = core::ptr::addr_of_mut!((*_11));
RET = [(-118_i8),79_i8,(-25_i8)];
(*_11) = _5 - _1;
_8.1 = 6878923776925976040_usize as i32;
RET = [(-81_i8),122_i8,70_i8];
_6 = ('\u{b632c}',);
(*_10) = 1_usize as i128;
_8.1 = -_9.1;
_6.0 = '\u{c75d4}';
_9 = (_8.0, _8.1);
_8 = (_9.0, _9.1);
_15 = core::ptr::addr_of_mut!((*_15));
_12 = _7;
_9.1 = -_8.1;
(*_2) = _1 << _8.1;
RET = [(-77_i8),(-22_i8),30_i8];
(*_2) = _1;
Goto(bb1)
}
bb1 = {
_3 = _13 as isize;
_5 = (*_15);
_8.1 = !_9.1;
_8 = (_9.0, _9.1);
(*_2) = false as i128;
_12 = !_3;
_8.0 = _9.0;
_18 = [_6.0,_6.0];
_10 = _2;
_9 = (_8.0, _8.1);
_10 = core::ptr::addr_of_mut!((*_2));
_9.0 = _8.0;
_15 = core::ptr::addr_of_mut!((*_2));
_10 = core::ptr::addr_of_mut!((*_11));
(*_11) = 3981550881_u32 as i128;
_8.0 = _9.0;
_6.0 = '\u{87f01}';
_6.0 = '\u{480cb}';
_19 = 86_i8 as isize;
Call((*_10) = fn12(_14, _4, _6.0, _8.0, _7, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_2) = _5;
_6 = ('\u{ee3fc}',);
(*_2) = _1;
Goto(bb3)
}
bb3 = {
_14 = _4 ^ _12;
Goto(bb4)
}
bb4 = {
_15 = _2;
_6 = ('\u{f8ac7}',);
_24 = 1454929614_u32 as isize;
_9.1 = _8.1;
_9.1 = -_8.1;
_26 = 13436066419569156635_usize as isize;
(*_10) = _1;
Goto(bb5)
}
bb5 = {
(*_2) = -_1;
RET = [115_i8,(-61_i8),(-89_i8)];
_1 = !(*_2);
_22 = _14 >> (*_11);
_22 = (*_15) as isize;
_24 = !_26;
(*_11) = -_1;
_11 = _2;
_23 = !_14;
_5 = true as i128;
_9.0 = core::ptr::addr_of_mut!(_21);
_9.1 = _8.1 << _26;
_22 = _14;
_19 = !_22;
_17 = _9.1 as u32;
_1 = !(*_2);
_8.0 = core::ptr::addr_of_mut!(_21);
_25 = _13 + _13;
(*_15) = !_1;
_29 = (*_15);
RET = [74_i8,69_i8,(-114_i8)];
_19 = !_23;
Goto(bb6)
}
bb6 = {
_1 = _23 as i128;
Goto(bb7)
}
bb7 = {
_20 = [8986676084880786725_u64,15324856507087135984_u64,5286421808831126484_u64,6619862363869449087_u64,13345382677573931565_u64,3311152831031947120_u64,9033287433589403535_u64,4401860507516614995_u64];
_21.2 = [109_i8,(-17_i8),7_i8];
_25 = -_13;
_28 = 18674_u16 as i16;
_11 = core::ptr::addr_of_mut!(_1);
_2 = core::ptr::addr_of_mut!(_1);
_28 = (-80_i8) as i16;
(*_11) = 0_u8 as i128;
_6 = ('\u{4dbf6}',);
_33.fld3 = _8.1 as i64;
_23 = !_22;
_2 = core::ptr::addr_of_mut!((*_2));
_26 = 262969221002796617_u64 as isize;
Goto(bb8)
}
bb8 = {
_33.fld0.4 = 38_i8;
_2 = _15;
_21.2 = [_33.fld0.4,_33.fld0.4,_33.fld0.4];
_31.1 = _9.1 * _9.1;
_33.fld0.6 = -_28;
_16 = core::ptr::addr_of!(_33.fld1.fld0);
_8.0 = core::ptr::addr_of_mut!(_21);
_35.2 = true;
_9 = (_8.0, _31.1);
_14 = _19 & _3;
_33.fld0.1 = (_31.1,);
_33.fld2 = core::ptr::addr_of!((*_16));
match _33.fld0.4 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb5,
5 => bb9,
38 => bb11,
_ => bb10
}
}
bb9 = {
_14 = _4 ^ _12;
Goto(bb4)
}
bb10 = {
_1 = _23 as i128;
Goto(bb7)
}
bb11 = {
_33.fld0.0 = !_14;
_9.1 = _6.0 as i32;
_32 = [_17];
_8.0 = core::ptr::addr_of_mut!(_21);
Goto(bb12)
}
bb12 = {
_19 = !_3;
_20 = [12392824838528370730_u64,7996441976684565561_u64,14120224759291552689_u64,8292302580986626695_u64,12599454758290188084_u64,11818242643252828694_u64,3394621984294390958_u64,11763860023370322552_u64];
_8 = _9;
RET = [_33.fld0.4,_33.fld0.4,_33.fld0.4];
Goto(bb13)
}
bb13 = {
_22 = -_33.fld0.0;
(*_11) = (*_15) >> _22;
_32 = [_17];
Goto(bb14)
}
bb14 = {
_33.fld0.0 = _14 * _3;
_15 = _11;
_38 = _33.fld0.6 as isize;
_31.2 = _1 ^ (*_2);
_33.fld0.1.0 = _9.1;
_6 = ('\u{1fb88}',);
_1 = -_31.2;
(*_16) = _33.fld0.4 as f32;
_33.fld0.3 = -_33.fld0.4;
_25 = _13 - _33.fld3;
_39 = [_9.1,_31.1,_31.1,_33.fld0.1.0,_8.1,_31.1,_31.1];
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(11_usize, 19_usize, Move(_19), 6_usize, Move(_6), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(11_usize, 32_usize, Move(_32), 3_usize, Move(_3), 17_usize, Move(_17), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(11_usize, 20_usize, Move(_20), 12_usize, Move(_12), 25_usize, Move(_25), 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: isize,mut _3: char,mut _4: *mut (i16, *mut i128, [i8; 3]),mut _5: isize,mut _6: *mut i128) -> i128 {
mir! {
type RET = i128;
let _7: [char; 2];
let _8: isize;
let _9: Adt65;
let _10: f64;
let _11: [i8; 3];
let _12: Adt58;
let _13: [u8; 4];
let _14: f32;
let _15: *mut (i16, *mut i128, [i8; 3]);
let _16: Adt57;
let _17: char;
let _18: ([char; 2], [u64; 8], bool);
let _19: Adt49;
let _20: f32;
let _21: char;
let _22: *const f32;
let _23: bool;
let _24: ([char; 2], [u64; 8], bool);
let _25: Adt58;
let _26: f64;
let _27: isize;
let _28: [u8; 4];
let _29: bool;
let _30: *const *mut ([i8; 3], i32, i128);
let _31: [i128; 3];
let _32: isize;
let _33: f32;
let _34: ();
let _35: ();
{
(*_4).0 = 292112274070553751233948541635499052641_u128 as i16;
_6 = (*_4).1;
RET = !(-76240300949711846517249972251217833133_i128);
_5 = _1 - _1;
RET = (-16337649630817265364614891839721174994_i128) - 169104825383839879011234621533770513788_i128;
RET = 101484367813507836754337958150501243752_i128 - 60719010918018622929404362754220697579_i128;
Call(_4 = fn13((*_4).0, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _5 >> _1;
RET = -(-10252661452849025942797106992220166582_i128);
RET = 1650949136_i32 as i128;
RET = (-160000052217878060893296664670871318535_i128) * 116996742288217765386605842314645965631_i128;
_9.fld5.fld1.fld3 = 7657990327400485484_i64;
_9.fld5.fld1.fld0.5 = core::ptr::addr_of_mut!(_9.fld2);
_9.fld5.fld3.fld2 = core::ptr::addr_of_mut!(_9.fld5.fld1.fld1.fld1);
_9.fld0 = 7_usize as f32;
_9.fld5.fld1.fld1.fld1.0 = [48_i8,(-65_i8),(-24_i8)];
_9.fld5.fld2 = [859764885_i32,521263339_i32,(-1646870988_i32),(-1972399724_i32),1400464018_i32,(-1369145555_i32),820491660_i32];
RET = 111102083433996616544881603435922397762_i128 | (-112797438962215552417868165798496733740_i128);
_9.fld2 = (-3245268091642809315831427261904549753_i128) - (-27448910592464887145978745968850505932_i128);
_9.fld5.fld1.fld0.4 = 33_i8;
_9.fld5.fld1.fld0.1.0 = (-864064030_i32);
_5 = _2;
match _9.fld5.fld1.fld3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
7657990327400485484 => bb8,
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
_7 = [_3,_3];
_9.fld1 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_9.fld3 = [_3,_3];
_9.fld4.1 = [2829290564173941330_u64,1536696232465112162_u64,8856640166408276483_u64,12002468529283571654_u64,15923845122890429002_u64,5107638187918455426_u64,2343705961028255524_u64,7068360946249467540_u64];
_9.fld4.1 = [16652007451182093449_u64,3743581243175818100_u64,6444675238928415953_u64,7863825869899926887_u64,4042202538442551449_u64,7224440306674932427_u64,3213788409700955874_u64,15596650367157169603_u64];
_9.fld5.fld3.fld0.2 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_9.fld5.fld1.fld0.5 = core::ptr::addr_of_mut!(_9.fld2);
_9.fld5.fld1.fld1.fld1.0 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_12.fld1 = (-5444_i16) - 16945_i16;
_9.fld5.fld1.fld2 = core::ptr::addr_of!(_9.fld5.fld1.fld1.fld0);
_12.fld1 = !8808_i16;
_9.fld5.fld1.fld1.fld0 = _9.fld0;
_12.fld2.1 = [_9.fld5.fld1.fld0.1.0,_9.fld5.fld1.fld0.1.0,_9.fld5.fld1.fld0.1.0,_9.fld5.fld1.fld0.1.0,_9.fld5.fld1.fld0.1.0,_9.fld5.fld1.fld0.1.0,_9.fld5.fld1.fld0.1.0];
_12.fld0.0 = _3;
_1 = _2;
_1 = _8;
_7 = [_3,_12.fld0.0];
_9.fld5.fld1.fld1.fld1.2 = -_9.fld2;
_9.fld5.fld1.fld1.fld1.2 = _9.fld2 * _9.fld2;
_10 = _9.fld0 as f64;
_11 = _9.fld5.fld1.fld1.fld1.0;
_13 = [217_u8,52_u8,92_u8,238_u8];
_9.fld5.fld1.fld1.fld0 = _9.fld0 - _9.fld0;
_9.fld5.fld1.fld0.2 = _9.fld5.fld1.fld0.5;
_9.fld5.fld1.fld1.fld1.2 = _9.fld2 ^ _9.fld2;
Goto(bb9)
}
bb9 = {
_9.fld5.fld1.fld1.fld1 = (_9.fld5.fld3.fld0.2, _9.fld5.fld1.fld0.1.0, _9.fld2);
_6 = core::ptr::addr_of_mut!(_16.fld7);
_17 = _3;
_9.fld5.fld3.fld0 = (_12.fld1, _9.fld5.fld1.fld0.5, _9.fld5.fld1.fld1.fld1.0);
(*_6) = _9.fld2;
RET = -_9.fld2;
_2 = _8 ^ _8;
_17 = _12.fld0.0;
_16.fld4.1 = 2206218559_u32 as i32;
_12.fld3 = core::ptr::addr_of!(_6);
_2 = _1;
_12.fld2.0 = core::ptr::addr_of_mut!(_9.fld5.fld1.fld0);
_6 = core::ptr::addr_of_mut!(_9.fld5.fld1.fld1.fld1.2);
_16.fld4.0 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_16.fld4.0 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_18.1 = [3017920448337471400_u64,12591891458801630049_u64,15873669731823479605_u64,1872742404891564498_u64,17123945797149249415_u64,13518442794848939470_u64,2461405720429095809_u64,4523545701993679600_u64];
Goto(bb10)
}
bb10 = {
_18 = (_9.fld3, _9.fld4.1, false);
_9.fld5.fld1.fld1.fld1.1 = _9.fld5.fld1.fld1.fld0 as i32;
_12.fld1 = _9.fld5.fld3.fld0.0;
_16.fld4.2 = (*_6);
_9.fld2 = !_16.fld4.2;
_9.fld5.fld1.fld0.5 = core::ptr::addr_of_mut!(_16.fld7);
_9.fld5.fld3.fld1 = _16.fld4.0;
_9.fld5.fld1.fld1.fld0 = _9.fld0;
_9.fld5.fld1.fld0.6 = _9.fld5.fld3.fld0.0;
_9.fld5.fld1.fld0.3 = !_9.fld5.fld1.fld0.4;
_15 = core::ptr::addr_of_mut!(_9.fld5.fld3.fld0);
_5 = _1;
_9.fld5.fld3.fld0.0 = _12.fld1;
_19.fld1.1 = _9.fld5.fld1.fld1.fld1.1 & _16.fld4.1;
_9.fld5.fld1.fld0.4 = _9.fld5.fld1.fld0.3 & _9.fld5.fld1.fld0.3;
_9.fld5.fld1.fld0.1.0 = _16.fld4.1 & _19.fld1.1;
(*_15).0 = !_9.fld5.fld1.fld0.6;
_19.fld1 = (_9.fld5.fld3.fld1, _16.fld4.1, (*_6));
_21 = _17;
_16.fld0 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_9.fld5.fld1.fld0.0 = _5 | _1;
match _9.fld5.fld1.fld3 {
0 => bb1,
1 => bb5,
2 => bb3,
7657990327400485484 => bb12,
_ => bb11
}
}
bb11 = {
_9.fld5.fld1.fld1.fld1 = (_9.fld5.fld3.fld0.2, _9.fld5.fld1.fld0.1.0, _9.fld2);
_6 = core::ptr::addr_of_mut!(_16.fld7);
_17 = _3;
_9.fld5.fld3.fld0 = (_12.fld1, _9.fld5.fld1.fld0.5, _9.fld5.fld1.fld1.fld1.0);
(*_6) = _9.fld2;
RET = -_9.fld2;
_2 = _8 ^ _8;
_17 = _12.fld0.0;
_16.fld4.1 = 2206218559_u32 as i32;
_12.fld3 = core::ptr::addr_of!(_6);
_2 = _1;
_12.fld2.0 = core::ptr::addr_of_mut!(_9.fld5.fld1.fld0);
_6 = core::ptr::addr_of_mut!(_9.fld5.fld1.fld1.fld1.2);
_16.fld4.0 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_16.fld4.0 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_18.1 = [3017920448337471400_u64,12591891458801630049_u64,15873669731823479605_u64,1872742404891564498_u64,17123945797149249415_u64,13518442794848939470_u64,2461405720429095809_u64,4523545701993679600_u64];
Goto(bb10)
}
bb12 = {
_17 = _12.fld0.0;
_9.fld4.0 = _7;
(*_15).2 = [_9.fld5.fld1.fld0.3,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_19.fld1.2 = _9.fld5.fld1.fld1.fld1.2;
RET = -(*_6);
_9.fld5.fld1.fld0.5 = core::ptr::addr_of_mut!((*_6));
_18.2 = true;
_17 = _21;
_13 = [179_u8,217_u8,195_u8,125_u8];
_9.fld5.fld3.fld0.2 = [_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.3,_9.fld5.fld1.fld0.3];
_23 = _5 <= _8;
_12.fld1 = _9.fld0 as i16;
_19.fld0 = _9.fld5.fld1.fld1.fld0 - _9.fld5.fld1.fld1.fld0;
_3 = _21;
_9.fld1 = [_9.fld5.fld1.fld0.3,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.3];
(*_6) = !_16.fld4.2;
_16.fld6 = _19.fld0 - _19.fld0;
_7 = [_3,_17];
_9.fld4.2 = _1 >= _8;
Goto(bb13)
}
bb13 = {
_1 = _8;
_15 = _4;
_6 = _9.fld5.fld3.fld0.1;
_9.fld5.fld1.fld1.fld1.2 = _16.fld4.2;
_12.fld0.0 = _3;
_18 = _9.fld4;
_9.fld2 = _18.2 as i128;
_9.fld5.fld3.fld0.0 = !_12.fld1;
_24 = (_9.fld4.0, _18.1, _9.fld4.2);
_23 = _18.2;
_12.fld3 = core::ptr::addr_of!(_9.fld5.fld1.fld0.2);
_9.fld5.fld1.fld0.6 = _12.fld1 | _9.fld5.fld3.fld0.0;
_9.fld4.1 = [9858673879672924914_u64,11021675373988060936_u64,16596512400475541873_u64,11531918330640307360_u64,14651188240536506926_u64,1849618231114217435_u64,13741951552697351954_u64,10477031394840977502_u64];
Goto(bb14)
}
bb14 = {
_25.fld2 = (_12.fld2.0, _9.fld5.fld2);
_10 = _9.fld5.fld1.fld3 as f64;
_25.fld2 = (_12.fld2.0, _9.fld5.fld2);
_9.fld5.fld1.fld1.fld1.1 = _19.fld1.1;
_9.fld5.fld3.fld0.2 = [_9.fld5.fld1.fld0.3,_9.fld5.fld1.fld0.4,_9.fld5.fld1.fld0.4];
_17 = _3;
_25.fld3 = _12.fld3;
RET = 4795461966410427700_u64 as i128;
_12.fld2 = (_25.fld2.0, _9.fld5.fld2);
_9.fld5.fld1.fld0.1.0 = _16.fld4.1 >> _8;
_8 = !_5;
_15 = _4;
_14 = _5 as f32;
_2 = -_1;
_9.fld5.fld1.fld3 = -7528487820918958182_i64;
_16.fld3 = core::ptr::addr_of_mut!(_9.fld5.fld3.fld0);
_5 = _9.fld5.fld1.fld0.0 << _9.fld2;
_9.fld5.fld1.fld3 = _9.fld5.fld1.fld0.6 as i64;
_25 = Adt58 { fld0: _12.fld0,fld1: _9.fld5.fld3.fld0.0,fld2: _12.fld2,fld3: _12.fld3 };
_22 = core::ptr::addr_of!(_14);
_31 = [(*_6),_9.fld2,(*_6)];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(12_usize, 24_usize, Move(_24), 11_usize, Move(_11), 13_usize, Move(_13), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(12_usize, 17_usize, Move(_17), 7_usize, Move(_7), 1_usize, Move(_1), 35_usize, _35), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: i16,mut _2: isize) -> *mut (i16, *mut i128, [i8; 3]) {
mir! {
type RET = *mut (i16, *mut i128, [i8; 3]);
let _3: ([i8; 3], i32, i128);
let _4: usize;
let _5: i8;
let _6: [i8; 3];
let _7: isize;
let _8: [u8; 4];
let _9: [i8; 3];
let _10: char;
let _11: (isize, (i32,), *mut i128, i8, i8, *mut i128, i16);
let _12: char;
let _13: u8;
let _14: bool;
let _15: Adt60;
let _16: *mut u64;
let _17: char;
let _18: Adt65;
let _19: i8;
let _20: [i8; 3];
let _21: isize;
let _22: (i16, *mut i128, [i8; 3]);
let _23: usize;
let _24: ([i8; 3], i32, i128);
let _25: (char,);
let _26: Adt52;
let _27: [u32; 1];
let _28: Adt65;
let _29: u8;
let _30: [char; 2];
let _31: *mut u64;
let _32: isize;
let _33: ();
let _34: ();
{
_1 = (-2265_i16);
_1 = (-24921_i16) | 13247_i16;
_1 = 5_usize as i16;
_3.0 = [(-68_i8),118_i8,35_i8];
_3.2 = 68404626033488810036067244989310477657_u128 as i128;
_3.1 = 261965683_i32 << _1;
_3.1 = (-1969611520_i32);
_1 = (-21062_i16) | (-6521_i16);
_1 = (-23428_i16) - (-27013_i16);
_3.2 = (-167060723607443553370593452807499073462_i128);
_3.2 = 8720800880792563264_i64 as i128;
_3.0 = [(-89_i8),(-116_i8),119_i8];
_3.0 = [(-118_i8),(-115_i8),3_i8];
_3.2 = -67945135891626685779435975614871503331_i128;
_4 = 39555_u16 as usize;
_3.2 = -(-101094497926214162929693783520229905260_i128);
_5 = (-77_i8) - 14_i8;
_2 = (-12_isize) >> _1;
_6 = [_5,_5,_5];
_3.0 = _6;
match _3.1 {
340282366920938463463374607429798599936 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_5 = (-110_i8);
_5 = 21_i8;
_1 = !(-27730_i16);
_1 = 16232_i16;
_7 = _2;
_1 = -(-3099_i16);
_6 = [_5,_5,_5];
_3.0 = _6;
_7 = _4 as isize;
_8 = [190_u8,237_u8,132_u8,94_u8];
_6 = [_5,_5,_5];
_3.0 = [_5,_5,_5];
_2 = false as isize;
_1 = (-6218_i16) + 14314_i16;
_10 = '\u{c2345}';
_3.2 = 49288798930697364049468864998235544256_i128 | 72217058390751741122386769857406690765_i128;
_6 = [_5,_5,_5];
_7 = -_2;
_3 = (_6, 1416521821_i32, 169923768685922007244630466251614866508_i128);
_8 = [144_u8,199_u8,128_u8,112_u8];
_6 = [_5,_5,_5];
Goto(bb3)
}
bb3 = {
_11.1 = (_3.1,);
_9 = [_5,_5,_5];
_11.1 = (_3.1,);
_10 = '\u{78c93}';
_8 = [232_u8,177_u8,104_u8,179_u8];
_11.1.0 = _3.1;
_11.5 = core::ptr::addr_of_mut!(_3.2);
_5 = (-86_i8) + (-97_i8);
_9 = [_5,_5,_5];
_11.3 = !_5;
_11.0 = true as isize;
_4 = 13722368239690346868_usize ^ 12545687768437294537_usize;
_11.6 = _1;
_12 = _10;
_11.0 = !_2;
_11.0 = _2 & _2;
RET = core::ptr::addr_of_mut!(_15.fld3.fld0);
_13 = 124_u8;
_4 = false as usize;
_15.fld2 = [_3.1,_3.1,_3.1,_3.1,_11.1.0,_3.1,_3.1];
_15.fld1.fld1.fld1.0 = _9;
Call(_15.fld1.fld0.0 = core::intrinsics::transmute(_7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15.fld1.fld1.fld1 = _3;
_15.fld1.fld3 = 2883595548654253704_i64;
_15.fld3.fld0.1 = core::ptr::addr_of_mut!(_3.2);
_5 = _11.3;
_15.fld1.fld0.2 = (*RET).1;
_6 = [_11.3,_5,_5];
(*RET) = (_11.6, _11.5, _6);
_15.fld1.fld2 = core::ptr::addr_of!(_15.fld1.fld1.fld0);
_15.fld1.fld0.1.0 = -_15.fld1.fld1.fld1.1;
_8 = [_13,_13,_13,_13];
_15.fld1.fld2 = core::ptr::addr_of!(_15.fld1.fld1.fld0);
_15.fld1.fld0.5 = core::ptr::addr_of_mut!(_15.fld1.fld1.fld1.2);
(*RET).0 = _3.1 as i16;
Goto(bb5)
}
bb5 = {
_15.fld1.fld3 = -3062898820266796861_i64;
_11.1.0 = _3.1 + _3.1;
_2 = _15.fld1.fld3 as isize;
_15.fld1.fld0.2 = core::ptr::addr_of_mut!(_3.2);
(*RET) = (_1, _11.5, _6);
_18.fld5.fld1.fld1.fld0 = 22361_u16 as f32;
_18.fld5.fld1.fld0.1.0 = _11.1.0 ^ _11.1.0;
_15.fld3.fld0.1 = core::ptr::addr_of_mut!(_15.fld1.fld1.fld1.2);
_15.fld3.fld0.2 = [_11.3,_5,_5];
_21 = _11.0;
_15.fld1.fld0.0 = !_11.0;
(*RET).1 = core::ptr::addr_of_mut!(_3.2);
_18.fld4.0 = [_10,_12];
_18.fld4.0 = [_12,_12];
_18.fld5.fld1.fld3 = !_15.fld1.fld3;
_6 = [_5,_5,_5];
_3.2 = 7781749848778443958_u64 as i128;
_11.1.0 = _18.fld5.fld1.fld0.1.0;
_5 = _11.6 as i8;
Call(_18.fld5.fld1.fld0.4 = fn14(_11.6, _15.fld1.fld3, (*RET).1, _4, _11.3, _11.5, _15.fld1.fld0.0, _15.fld1.fld0.5, (*RET).2, (*RET).1, _15.fld1.fld0.2, _15.fld1.fld2, (*RET).2, _11.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_18.fld4.1 = [15340005019202661856_u64,10813058956692296199_u64,14339658582282043829_u64,3255391730658818577_u64,2954126380402127161_u64,5386180803294112120_u64,6931800695222310941_u64,11387598693842148635_u64];
_18.fld1 = _6;
_21 = _18.fld5.fld1.fld0.4 as isize;
_24 = _3;
_3 = _24;
_6 = [_5,_5,_5];
_22.2 = [_18.fld5.fld1.fld0.4,_18.fld5.fld1.fld0.4,_11.3];
_18.fld5.fld1.fld0.3 = _5;
_11 = (_21, _18.fld5.fld1.fld0.1, (*RET).1, _18.fld5.fld1.fld0.3, _5, (*RET).1, _15.fld3.fld0.0);
_18.fld4.0 = [_12,_12];
_18.fld4.1 = [11426615593175553521_u64,2214239799574342404_u64,12123218135269731505_u64,3129636559737170802_u64,17253652459565167913_u64,11713964399448782914_u64,8825836114462226688_u64,4796262932221191670_u64];
_15.fld1.fld1.fld1.2 = _24.2;
_15.fld1.fld0.4 = !_18.fld5.fld1.fld0.4;
_15.fld3.fld0 = (_11.6, _15.fld1.fld0.5, _3.0);
_18.fld5.fld1.fld0.4 = _5;
_18.fld5.fld1.fld1.fld1.2 = _15.fld1.fld1.fld1.2 - _3.2;
_18.fld5.fld1.fld1.fld1.0 = [_18.fld5.fld1.fld0.4,_18.fld5.fld1.fld0.3,_11.4];
_15.fld0 = core::ptr::addr_of!(_13);
_1 = _18.fld5.fld1.fld1.fld1.2 as i16;
_15.fld3.fld1 = [_11.4,_11.4,_15.fld1.fld0.4];
_26.fld0.0 = -_1;
(*RET).0 = _11.6;
_28.fld5.fld3.fld0.1 = core::ptr::addr_of_mut!(_18.fld5.fld1.fld1.fld1.2);
_28.fld5.fld1.fld1.fld0 = _15.fld1.fld1.fld0;
_11.4 = -_15.fld1.fld0.4;
_18.fld5.fld3.fld0.0 = !_11.6;
match _3.1 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb8,
1416521821 => bb10,
_ => bb9
}
}
bb7 = {
_15.fld1.fld3 = -3062898820266796861_i64;
_11.1.0 = _3.1 + _3.1;
_2 = _15.fld1.fld3 as isize;
_15.fld1.fld0.2 = core::ptr::addr_of_mut!(_3.2);
(*RET) = (_1, _11.5, _6);
_18.fld5.fld1.fld1.fld0 = 22361_u16 as f32;
_18.fld5.fld1.fld0.1.0 = _11.1.0 ^ _11.1.0;
_15.fld3.fld0.1 = core::ptr::addr_of_mut!(_15.fld1.fld1.fld1.2);
_15.fld3.fld0.2 = [_11.3,_5,_5];
_21 = _11.0;
_15.fld1.fld0.0 = !_11.0;
(*RET).1 = core::ptr::addr_of_mut!(_3.2);
_18.fld4.0 = [_10,_12];
_18.fld4.0 = [_12,_12];
_18.fld5.fld1.fld3 = !_15.fld1.fld3;
_6 = [_5,_5,_5];
_3.2 = 7781749848778443958_u64 as i128;
_11.1.0 = _18.fld5.fld1.fld0.1.0;
_5 = _11.6 as i8;
Call(_18.fld5.fld1.fld0.4 = fn14(_11.6, _15.fld1.fld3, (*RET).1, _4, _11.3, _11.5, _15.fld1.fld0.0, _15.fld1.fld0.5, (*RET).2, (*RET).1, _15.fld1.fld0.2, _15.fld1.fld2, (*RET).2, _11.1), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_15.fld1.fld1.fld1 = _3;
_15.fld1.fld3 = 2883595548654253704_i64;
_15.fld3.fld0.1 = core::ptr::addr_of_mut!(_3.2);
_5 = _11.3;
_15.fld1.fld0.2 = (*RET).1;
_6 = [_11.3,_5,_5];
(*RET) = (_11.6, _11.5, _6);
_15.fld1.fld2 = core::ptr::addr_of!(_15.fld1.fld1.fld0);
_15.fld1.fld0.1.0 = -_15.fld1.fld1.fld1.1;
_8 = [_13,_13,_13,_13];
_15.fld1.fld2 = core::ptr::addr_of!(_15.fld1.fld1.fld0);
_15.fld1.fld0.5 = core::ptr::addr_of_mut!(_15.fld1.fld1.fld1.2);
(*RET).0 = _3.1 as i16;
Goto(bb5)
}
bb9 = {
_5 = (-110_i8);
_5 = 21_i8;
_1 = !(-27730_i16);
_1 = 16232_i16;
_7 = _2;
_1 = -(-3099_i16);
_6 = [_5,_5,_5];
_3.0 = _6;
_7 = _4 as isize;
_8 = [190_u8,237_u8,132_u8,94_u8];
_6 = [_5,_5,_5];
_3.0 = [_5,_5,_5];
_2 = false as isize;
_1 = (-6218_i16) + 14314_i16;
_10 = '\u{c2345}';
_3.2 = 49288798930697364049468864998235544256_i128 | 72217058390751741122386769857406690765_i128;
_6 = [_5,_5,_5];
_7 = -_2;
_3 = (_6, 1416521821_i32, 169923768685922007244630466251614866508_i128);
_8 = [144_u8,199_u8,128_u8,112_u8];
_6 = [_5,_5,_5];
Goto(bb3)
}
bb10 = {
_15.fld3.fld0.2 = _22.2;
_15.fld3.fld0.0 = _1 - _1;
_18.fld4.2 = !false;
_28.fld5.fld1.fld1.fld1 = _24;
match _15.fld1.fld1.fld1.1 {
0 => bb1,
1 => bb7,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
1416521821 => bb16,
_ => bb15
}
}
bb11 = {
_5 = (-110_i8);
_5 = 21_i8;
_1 = !(-27730_i16);
_1 = 16232_i16;
_7 = _2;
_1 = -(-3099_i16);
_6 = [_5,_5,_5];
_3.0 = _6;
_7 = _4 as isize;
_8 = [190_u8,237_u8,132_u8,94_u8];
_6 = [_5,_5,_5];
_3.0 = [_5,_5,_5];
_2 = false as isize;
_1 = (-6218_i16) + 14314_i16;
_10 = '\u{c2345}';
_3.2 = 49288798930697364049468864998235544256_i128 | 72217058390751741122386769857406690765_i128;
_6 = [_5,_5,_5];
_7 = -_2;
_3 = (_6, 1416521821_i32, 169923768685922007244630466251614866508_i128);
_8 = [144_u8,199_u8,128_u8,112_u8];
_6 = [_5,_5,_5];
Goto(bb3)
}
bb12 = {
_15.fld1.fld1.fld1 = _3;
_15.fld1.fld3 = 2883595548654253704_i64;
_15.fld3.fld0.1 = core::ptr::addr_of_mut!(_3.2);
_5 = _11.3;
_15.fld1.fld0.2 = (*RET).1;
_6 = [_11.3,_5,_5];
(*RET) = (_11.6, _11.5, _6);
_15.fld1.fld2 = core::ptr::addr_of!(_15.fld1.fld1.fld0);
_15.fld1.fld0.1.0 = -_15.fld1.fld1.fld1.1;
_8 = [_13,_13,_13,_13];
_15.fld1.fld2 = core::ptr::addr_of!(_15.fld1.fld1.fld0);
_15.fld1.fld0.5 = core::ptr::addr_of_mut!(_15.fld1.fld1.fld1.2);
(*RET).0 = _3.1 as i16;
Goto(bb5)
}
bb13 = {
Return()
}
bb14 = {
_5 = (-110_i8);
_5 = 21_i8;
_1 = !(-27730_i16);
_1 = 16232_i16;
_7 = _2;
_1 = -(-3099_i16);
_6 = [_5,_5,_5];
_3.0 = _6;
_7 = _4 as isize;
_8 = [190_u8,237_u8,132_u8,94_u8];
_6 = [_5,_5,_5];
_3.0 = [_5,_5,_5];
_2 = false as isize;
_1 = (-6218_i16) + 14314_i16;
_10 = '\u{c2345}';
_3.2 = 49288798930697364049468864998235544256_i128 | 72217058390751741122386769857406690765_i128;
_6 = [_5,_5,_5];
_7 = -_2;
_3 = (_6, 1416521821_i32, 169923768685922007244630466251614866508_i128);
_8 = [144_u8,199_u8,128_u8,112_u8];
_6 = [_5,_5,_5];
Goto(bb3)
}
bb15 = {
_15.fld1.fld3 = -3062898820266796861_i64;
_11.1.0 = _3.1 + _3.1;
_2 = _15.fld1.fld3 as isize;
_15.fld1.fld0.2 = core::ptr::addr_of_mut!(_3.2);
(*RET) = (_1, _11.5, _6);
_18.fld5.fld1.fld1.fld0 = 22361_u16 as f32;
_18.fld5.fld1.fld0.1.0 = _11.1.0 ^ _11.1.0;
_15.fld3.fld0.1 = core::ptr::addr_of_mut!(_15.fld1.fld1.fld1.2);
_15.fld3.fld0.2 = [_11.3,_5,_5];
_21 = _11.0;
_15.fld1.fld0.0 = !_11.0;
(*RET).1 = core::ptr::addr_of_mut!(_3.2);
_18.fld4.0 = [_10,_12];
_18.fld4.0 = [_12,_12];
_18.fld5.fld1.fld3 = !_15.fld1.fld3;
_6 = [_5,_5,_5];
_3.2 = 7781749848778443958_u64 as i128;
_11.1.0 = _18.fld5.fld1.fld0.1.0;
_5 = _11.6 as i8;
Call(_18.fld5.fld1.fld0.4 = fn14(_11.6, _15.fld1.fld3, (*RET).1, _4, _11.3, _11.5, _15.fld1.fld0.0, _15.fld1.fld0.5, (*RET).2, (*RET).1, _15.fld1.fld0.2, _15.fld1.fld2, (*RET).2, _11.1), ReturnTo(bb6), UnwindUnreachable())
}
bb16 = {
_28.fld4.1 = [1917169438261499781_u64,5983413409474996880_u64,7514730138071706545_u64,3601210842243657358_u64,7972853183979451149_u64,4227677404936671514_u64,5340218045049047087_u64,6733305862741843982_u64];
_28.fld2 = -_28.fld5.fld1.fld1.fld1.2;
(*RET).2 = [_5,_15.fld1.fld0.4,_5];
(*RET) = (_26.fld0.0, _15.fld1.fld0.2, _18.fld5.fld1.fld1.fld1.0);
_18.fld5.fld1.fld0.1.0 = _11.1.0;
_18.fld5.fld3.fld0.1 = _15.fld1.fld0.5;
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(13_usize, 10_usize, Move(_10), 3_usize, Move(_3), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(13_usize, 5_usize, Move(_5), 21_usize, Move(_21), 6_usize, Move(_6), 34_usize, _34), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i16,mut _2: i64,mut _3: *mut i128,mut _4: usize,mut _5: i8,mut _6: *mut i128,mut _7: isize,mut _8: *mut i128,mut _9: [i8; 3],mut _10: *mut i128,mut _11: *mut i128,mut _12: *const f32,mut _13: [i8; 3],mut _14: (i32,)) -> i8 {
mir! {
type RET = i8;
let _15: f32;
let _16: [i32; 7];
let _17: *const *mut i128;
let _18: u16;
let _19: isize;
let _20: Adt55;
let _21: isize;
let _22: f32;
let _23: (u64, *const *mut i128, [i8; 3], *const *mut i128);
let _24: Adt49;
let _25: bool;
let _26: f32;
let _27: isize;
let _28: i32;
let _29: bool;
let _30: ([i8; 3], i32, i128);
let _31: ([i8; 3], i32, i128);
let _32: i128;
let _33: (i16, *mut i128, [i8; 3]);
let _34: f32;
let _35: ();
let _36: ();
{
(*_3) = !(*_8);
_2 = (-4946127656914839198_i64) | (-1561089775388347559_i64);
_13 = _9;
_5 = 34_i8 >> (*_11);
_4 = 9885267355484429072_usize;
(*_10) = false as i128;
_15 = 262801978551794685789613725264338504021_u128 as f32;
(*_11) = !(*_8);
RET = _5 | _5;
_1 = -(-12264_i16);
_2 = 2964556901885713086_i64;
Goto(bb1)
}
bb1 = {
_3 = core::ptr::addr_of_mut!((*_10));
_1 = 22927_i16;
(*_10) = (*_8) | (*_8);
(*_6) = (*_8) - (*_8);
(*_6) = 2005522627_u32 as i128;
(*_8) = -(*_3);
_2 = (-3409376074272727724_i64);
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463459965231357495483732 => bb9,
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
(*_6) = (*_8) * (*_8);
(*_12) = _15 - _15;
(*_10) = (*_8) >> _1;
(*_10) = _4 as i128;
_12 = core::ptr::addr_of!(_15);
RET = -_5;
_7 = !(-9223372036854775808_isize);
(*_8) = (*_10);
_10 = core::ptr::addr_of_mut!((*_8));
(*_10) = _7 as i128;
_15 = _2 as f32;
_7 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_12 = core::ptr::addr_of!((*_12));
_16 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_15 = 7667135302599321146_u64 as f32;
RET = false as i8;
_18 = _2 as u16;
(*_3) = (*_10);
Goto(bb10)
}
bb10 = {
_18 = 31502_u16;
_17 = core::ptr::addr_of!(_3);
_15 = 1828372665295922350_u64 as f32;
RET = !_5;
(*_12) = 2949587325793020526_u64 as f32;
_3 = core::ptr::addr_of_mut!((*_6));
(*_12) = _1 as f32;
_6 = core::ptr::addr_of_mut!((*_3));
_11 = core::ptr::addr_of_mut!((*_3));
(*_3) = (*_10);
_5 = -74_i8;
(*_17) = core::ptr::addr_of_mut!((*_8));
_16 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_11 = core::ptr::addr_of_mut!((*_8));
_20.fld0.fld6.fld0 = [2338365473_u32];
_20.fld0.fld6.fld1 = (4794432975037775447_u64, _17, _13, _17);
_8 = _3;
_7 = 9223372036854775807_isize;
(*_3) = _5 as i128;
_9 = _20.fld0.fld6.fld1.2;
_19 = !_7;
_7 = _19 - _19;
Call(_20.fld0.fld6 = fn15((*_17), _17, (*_6), _11, _14.0, _8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_21 = _7;
_1 = -4655_i16;
_11 = _6;
_5 = (-57_i8);
_20.fld1.fld0.0 = _20.fld0.fld6.fld2;
_20.fld0.fld6.fld1.3 = core::ptr::addr_of!((*_17));
_20.fld0.fld7 = _16;
_23.2 = [_5,_5,_5];
_24.fld1.1 = 248_u8 as i32;
_20.fld1.fld2 = 1080596818_u32;
Goto(bb12)
}
bb12 = {
_26 = -(*_12);
(*_10) = (*_6);
_20.fld1.fld0.1 = [_24.fld1.1,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
Goto(bb13)
}
bb13 = {
_6 = core::ptr::addr_of_mut!((*_10));
_20.fld1.fld1 = _14;
(*_17) = core::ptr::addr_of_mut!((*_8));
_20.fld0.fld3.0 = _20.fld0.fld6.fld2;
_15 = _7 as f32;
(*_6) = _1 as i128;
_22 = (*_12);
_16 = _20.fld1.fld0.1;
_20.fld0.fld6.fld0 = [_20.fld1.fld2];
_26 = _15;
_20.fld0.fld6.fld1.1 = _17;
_24.fld1.0 = _9;
_1 = !(-31135_i16);
_12 = core::ptr::addr_of!((*_12));
_25 = true | true;
_14 = (_20.fld1.fld1.0,);
_23.2 = [_5,_5,_5];
_1 = (-17233_i16) ^ (-10464_i16);
(*_11) = (*_6) * (*_10);
RET = _5 + _5;
_20.fld0.fld2.fld2 = core::ptr::addr_of!((*_17));
(*_12) = _22 - _22;
Goto(bb14)
}
bb14 = {
(*_12) = -_22;
_20.fld0.fld2.fld0 = _20.fld1.fld1.0 as i64;
_20.fld0.fld6.fld0 = [_20.fld1.fld2];
_23 = (_20.fld0.fld6.fld1.0, _20.fld0.fld6.fld1.1, _9, _17);
_29 = !_25;
_20.fld0.fld6.fld1.2 = [_5,_5,_5];
_20.fld0.fld6.fld1.1 = _17;
_20.fld0.fld2.fld0 = -_2;
_20.fld0.fld3.1 = _20.fld1.fld0.1;
_24.fld1.2 = (*_6);
_20.fld1.fld2 = 897549744_u32;
_13 = [_5,_5,_5];
_20.fld1.fld0 = (_20.fld0.fld6.fld2, _20.fld0.fld3.1);
_20.fld1.fld0.1 = _20.fld0.fld3.1;
(*_10) = (*_11);
_31 = _24.fld1;
_6 = core::ptr::addr_of_mut!((*_6));
_21 = !_7;
(*_8) = !_31.2;
_20.fld0.fld4 = _1;
_2 = _4 as i64;
_20.fld1.fld1.0 = -_14.0;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(14_usize, 4_usize, Move(_4), 31_usize, Move(_31), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(14_usize, 9_usize, Move(_9), 25_usize, Move(_25), 13_usize, Move(_13), 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: *mut i128,mut _2: *const *mut i128,mut _3: i128,mut _4: *mut i128,mut _5: i32,mut _6: *mut i128) -> Adt51 {
mir! {
type RET = Adt51;
let _7: *mut ([i8; 3], i32, i128);
let _8: f64;
let _9: f32;
let _10: [i32; 7];
let _11: f64;
let _12: Adt63;
let _13: isize;
let _14: isize;
let _15: (i16, *mut i128, [i8; 3]);
let _16: Adt59;
let _17: bool;
let _18: Adt55;
let _19: char;
let _20: [i128; 3];
let _21: i32;
let _22: [u64; 8];
let _23: [i8; 3];
let _24: isize;
let _25: u64;
let _26: f64;
let _27: isize;
let _28: [u8; 4];
let _29: u32;
let _30: ();
let _31: ();
{
RET.fld1.2 = [(-47_i8),58_i8,(-30_i8)];
_5 = (-1577364888_i32);
(*_2) = core::ptr::addr_of_mut!(_3);
RET.fld1.0 = 15816622306042040889_u64 | 10560436952636583909_u64;
RET.fld1.1 = _2;
(*_1) = _3 + _3;
_2 = core::ptr::addr_of!((*_2));
RET.fld1.3 = core::ptr::addr_of!((*_2));
_1 = core::ptr::addr_of_mut!((*_1));
RET.fld0 = [1704768438_u32];
_1 = core::ptr::addr_of_mut!((*_4));
RET.fld1.0 = 4844007164018531776_u64 * 9297043965036371538_u64;
_5 = 1466232152_i32 & (-90023972_i32);
RET.fld1.3 = core::ptr::addr_of!(_1);
(*_4) = 1785038044_u32 as i128;
_2 = core::ptr::addr_of!(_4);
RET.fld1.2 = [(-45_i8),(-60_i8),12_i8];
_5 = (*_1) as i32;
(*_1) = _3;
(*_6) = -_3;
Goto(bb1)
}
bb1 = {
RET.fld0 = [990444324_u32];
RET.fld1.0 = 6328968700521917954_u64 & 8182978937225470777_u64;
_2 = core::ptr::addr_of!((*_2));
(*_4) = 56439_u16 as i128;
RET.fld1.3 = core::ptr::addr_of!(_1);
RET.fld1.0 = 9032110923658762451_usize as u64;
_10 = [_5,_5,_5,_5,_5,_5,_5];
_3 = (*_4) >> (*_1);
(*_4) = _3 ^ _3;
_6 = (*_2);
(*_2) = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of!((*_2));
RET.fld1.1 = core::ptr::addr_of!(_4);
_8 = 793925844_u32 as f64;
_12.fld7.fld4.fld1.3 = core::ptr::addr_of!(_1);
_2 = core::ptr::addr_of!(_1);
_12.fld3 = -(-119_i8);
(*_6) = -_3;
_12.fld7.fld1 = [1295602903_u32];
_12.fld0 = 286817684965704265264832333163690436405_u128;
Goto(bb2)
}
bb2 = {
_12.fld7.fld4.fld0 = _12.fld7.fld1;
_12.fld7.fld2 = Adt50 { fld0: 7203935262774351176_i64,fld1: '\u{91c24}',fld2: _2 };
RET.fld1.1 = core::ptr::addr_of!(_1);
_4 = core::ptr::addr_of_mut!((*_1));
_12.fld7.fld0.1 = !_5;
_12.fld7.fld0.0 = [_12.fld3,_12.fld3,_12.fld3];
RET.fld1.3 = core::ptr::addr_of!((*_2));
_12.fld7.fld0.2 = (*_1);
_13 = -(-5_isize);
_1 = _4;
_15.2 = _12.fld7.fld0.0;
(*_6) = _12.fld7.fld0.2;
_7 = core::ptr::addr_of_mut!(_12.fld7.fld0);
_15.2 = [_12.fld3,_12.fld3,_12.fld3];
_16.fld4.fld2.1 = [(*_7).1,(*_7).1,(*_7).1,_5,_5,(*_7).1,(*_7).1];
RET.fld1.1 = _2;
_12.fld2 = _12.fld0 as isize;
_16.fld7.4 = _12.fld3 << _12.fld7.fld2.fld0;
_12.fld0 = !187314166499874631665679994669654866581_u128;
_18.fld1.fld1 = (_12.fld7.fld0.1,);
_18.fld0.fld3.1 = [(*_7).1,(*_7).1,(*_7).1,(*_7).1,_12.fld7.fld0.1,_5,_12.fld7.fld0.1];
_12.fld7.fld4.fld1.3 = _12.fld7.fld2.fld2;
match _12.fld7.fld2.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
7203935262774351176 => bb9,
_ => bb8
}
}
bb3 = {
RET.fld0 = [990444324_u32];
RET.fld1.0 = 6328968700521917954_u64 & 8182978937225470777_u64;
_2 = core::ptr::addr_of!((*_2));
(*_4) = 56439_u16 as i128;
RET.fld1.3 = core::ptr::addr_of!(_1);
RET.fld1.0 = 9032110923658762451_usize as u64;
_10 = [_5,_5,_5,_5,_5,_5,_5];
_3 = (*_4) >> (*_1);
(*_4) = _3 ^ _3;
_6 = (*_2);
(*_2) = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of!((*_2));
RET.fld1.1 = core::ptr::addr_of!(_4);
_8 = 793925844_u32 as f64;
_12.fld7.fld4.fld1.3 = core::ptr::addr_of!(_1);
_2 = core::ptr::addr_of!(_1);
_12.fld3 = -(-119_i8);
(*_6) = -_3;
_12.fld7.fld1 = [1295602903_u32];
_12.fld0 = 286817684965704265264832333163690436405_u128;
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
_15.2 = [_16.fld7.4,_12.fld3,_16.fld7.4];
_6 = (*_2);
_18.fld0.fld6.fld2 = core::ptr::addr_of_mut!(_16.fld7);
_16.fld7.3 = _12.fld7.fld2.fld0 as i8;
(*_7).2 = (*_1);
Goto(bb10)
}
bb10 = {
_11 = _8 / f64::NAN;
_15.1 = core::ptr::addr_of_mut!((*_4));
_16.fld7 = (_12.fld2, _18.fld1.fld1, _4, _12.fld3, _12.fld3, (*_2), (-5648_i16));
_12.fld7.fld4.fld1 = (7094810704584147024_u64, _2, _15.2, _12.fld7.fld2.fld2);
_16.fld7.0 = _12.fld2 - _13;
_16.fld4.fld0.0 = _12.fld7.fld2.fld1;
RET.fld1 = (_12.fld7.fld4.fld1.0, _12.fld7.fld2.fld2, _15.2, _12.fld7.fld4.fld1.1);
_18.fld0.fld3.1 = _10;
_6 = core::ptr::addr_of_mut!((*_4));
_12.fld2 = !_16.fld7.0;
_16.fld1 = core::ptr::addr_of_mut!(_12.fld7.fld2.fld0);
_20 = [_12.fld7.fld0.2,(*_6),(*_7).2];
match _12.fld7.fld4.fld1.0 {
0 => bb4,
1 => bb6,
2 => bb11,
7094810704584147024 => bb13,
_ => bb12
}
}
bb11 = {
_15.2 = [_16.fld7.4,_12.fld3,_16.fld7.4];
_6 = (*_2);
_18.fld0.fld6.fld2 = core::ptr::addr_of_mut!(_16.fld7);
_16.fld7.3 = _12.fld7.fld2.fld0 as i8;
(*_7).2 = (*_1);
Goto(bb10)
}
bb12 = {
RET.fld0 = [990444324_u32];
RET.fld1.0 = 6328968700521917954_u64 & 8182978937225470777_u64;
_2 = core::ptr::addr_of!((*_2));
(*_4) = 56439_u16 as i128;
RET.fld1.3 = core::ptr::addr_of!(_1);
RET.fld1.0 = 9032110923658762451_usize as u64;
_10 = [_5,_5,_5,_5,_5,_5,_5];
_3 = (*_4) >> (*_1);
(*_4) = _3 ^ _3;
_6 = (*_2);
(*_2) = core::ptr::addr_of_mut!((*_1));
_2 = core::ptr::addr_of!((*_2));
RET.fld1.1 = core::ptr::addr_of!(_4);
_8 = 793925844_u32 as f64;
_12.fld7.fld4.fld1.3 = core::ptr::addr_of!(_1);
_2 = core::ptr::addr_of!(_1);
_12.fld3 = -(-119_i8);
(*_6) = -_3;
_12.fld7.fld1 = [1295602903_u32];
_12.fld0 = 286817684965704265264832333163690436405_u128;
Goto(bb2)
}
bb13 = {
_12.fld1 = core::ptr::addr_of_mut!(_11);
_18.fld0.fld6.fld1.0 = !_12.fld7.fld4.fld1.0;
_16.fld4.fld3 = _12.fld7.fld4.fld1.3;
RET = Adt51 { fld0: _12.fld7.fld4.fld0,fld1: _12.fld7.fld4.fld1,fld2: _18.fld0.fld6.fld2 };
_16.fld4.fld2 = (_18.fld0.fld6.fld2, _18.fld0.fld3.1);
_16.fld0.0 = !_16.fld7.6;
_16.fld7 = (_12.fld2, _18.fld1.fld1, _1, _12.fld3, _12.fld3, (*_2), _16.fld0.0);
_16.fld4.fld2.0 = core::ptr::addr_of_mut!(_16.fld7);
_26 = _8;
(*_4) = !(*_7).2;
Call(_18.fld1.fld0 = fn16((*_4), _20, _12.fld2, _12.fld7.fld0.2, _12.fld7.fld4.fld1.0, _12.fld7.fld0.0, _12.fld7.fld4.fld1.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_16.fld4.fld2 = (_18.fld0.fld6.fld2, _18.fld1.fld0.1);
_11 = _26 / f64::NEG_INFINITY;
_12.fld7.fld3 = (_16.fld1,);
(*_7).0 = _15.2;
RET.fld1.3 = core::ptr::addr_of!(_16.fld7.2);
_18.fld0.fld2.fld1 = _12.fld7.fld2.fld1;
_18.fld0.fld3.0 = _18.fld1.fld0.0;
(*_7).2 = 2761229239_u32 as i128;
_18.fld0.fld4 = _16.fld7.6;
_15 = (_16.fld0.0, _6, _12.fld7.fld0.0);
_18.fld0.fld6.fld1 = _12.fld7.fld4.fld1;
_12.fld7.fld2.fld1 = _18.fld0.fld2.fld1;
RET.fld1.2 = _12.fld7.fld0.0;
_16.fld7.6 = -_16.fld0.0;
(*_2) = core::ptr::addr_of_mut!((*_6));
_16.fld0.1 = _6;
RET = Adt51 { fld0: _12.fld7.fld1,fld1: _18.fld0.fld6.fld1,fld2: _18.fld0.fld3.0 };
_18.fld0.fld1 = [169_u8,42_u8,26_u8,207_u8];
(*_6) = 45_u8 as i128;
(*_7).2 = !(*_6);
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(15_usize, 13_usize, Move(_13), 20_usize, Move(_20), 31_usize, _31, 31_usize, _31), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i128,mut _2: [i128; 3],mut _3: isize,mut _4: i128,mut _5: u64,mut _6: [i8; 3],mut _7: [i8; 3]) -> (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]) {
mir! {
type RET = (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]);
let _8: *mut ([i8; 3], i32, i128);
let _9: u16;
let _10: *mut i128;
let _11: isize;
let _12: i16;
let _13: *mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16);
let _14: [u8; 4];
let _15: i32;
let _16: isize;
let _17: (i16, *mut i128, [i8; 3]);
let _18: isize;
let _19: f32;
let _20: u8;
let _21: [i32; 7];
let _22: f32;
let _23: ([i8; 3], i32, i128);
let _24: [u32; 1];
let _25: Adt52;
let _26: char;
let _27: isize;
let _28: [u64; 8];
let _29: Adt53;
let _30: [i128; 3];
let _31: f32;
let _32: *mut f64;
let _33: [i128; 3];
let _34: isize;
let _35: *const u8;
let _36: [i128; 3];
let _37: char;
let _38: Adt49;
let _39: ([i8; 3], i32, i128);
let _40: [u32; 1];
let _41: [i32; 7];
let _42: ([i8; 3], i32, i128);
let _43: isize;
let _44: u16;
let _45: ([char; 2], [u64; 8], bool);
let _46: usize;
let _47: isize;
let _48: i128;
let _49: f32;
let _50: [i16; 6];
let _51: isize;
let _52: [i8; 3];
let _53: isize;
let _54: u64;
let _55: f64;
let _56: ([i8; 3], i32, i128);
let _57: [char; 2];
let _58: [i16; 6];
let _59: f32;
let _60: *const *mut ([i8; 3], i32, i128);
let _61: i16;
let _62: ([i8; 3], i32, i128);
let _63: f64;
let _64: [i8; 3];
let _65: isize;
let _66: usize;
let _67: f64;
let _68: [i32; 7];
let _69: i8;
let _70: Adt63;
let _71: (char,);
let _72: Adt52;
let _73: *mut u64;
let _74: isize;
let _75: i32;
let _76: f64;
let _77: [u32; 1];
let _78: [u32; 1];
let _79: bool;
let _80: isize;
let _81: f64;
let _82: *const u8;
let _83: isize;
let _84: i32;
let _85: isize;
let _86: ([char; 2], [u64; 8], bool);
let _87: [i128; 3];
let _88: [u32; 1];
let _89: Adt58;
let _90: (char,);
let _91: ([char; 2], [u64; 8], bool);
let _92: i16;
let _93: u64;
let _94: bool;
let _95: (*mut (i16, *mut i128, [i8; 3]), i32);
let _96: Adt62;
let _97: (u64, *const *mut i128, [i8; 3], *const *mut i128);
let _98: [i8; 3];
let _99: i8;
let _100: isize;
let _101: Adt55;
let _102: char;
let _103: f64;
let _104: (i32,);
let _105: bool;
let _106: *mut u64;
let _107: *mut ([i8; 3], i32, i128);
let _108: Adt56;
let _109: f64;
let _110: [u32; 1];
let _111: bool;
let _112: f32;
let _113: [u32; 1];
let _114: isize;
let _115: i32;
let _116: ([i8; 3], i32, i128);
let _117: isize;
let _118: f32;
let _119: usize;
let _120: *mut (i16, *mut i128, [i8; 3]);
let _121: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]);
let _122: ();
let _123: ();
{
_5 = 10816258553944514311_u64;
RET.1 = [(-801889140_i32),1637693818_i32,(-1272249037_i32),1818384215_i32,2118616178_i32,2110103054_i32,2096677122_i32];
_9 = 33000_u16;
RET.1 = [(-1806764541_i32),1127841245_i32,492090153_i32,1231010805_i32,591598825_i32,(-2062795127_i32),(-672347443_i32)];
_4 = _1 | _1;
RET.1 = [1560218489_i32,249927190_i32,319782028_i32,(-1226215077_i32),1422585380_i32,597543208_i32,648616247_i32];
_5 = 13997037276101105664_u64 << _1;
_1 = !_4;
_6 = [(-41_i8),(-22_i8),(-124_i8)];
_1 = _4;
_7 = [87_i8,(-119_i8),87_i8];
_6 = [10_i8,11_i8,93_i8];
_11 = _3;
_3 = _11 + _11;
_2 = [_1,_1,_4];
_2 = [_1,_1,_4];
Goto(bb1)
}
bb1 = {
_12 = true as i16;
_5 = 4765638692413553996_u64 & 1213009901229995988_u64;
_12 = 29990_i16 | 28683_i16;
_11 = -_3;
RET.1 = [(-719995624_i32),1429164869_i32,1720649551_i32,934435937_i32,1355773602_i32,1452207270_i32,(-1969001214_i32)];
_6 = [108_i8,(-21_i8),(-32_i8)];
_16 = -_11;
_1 = _4;
RET.1 = [(-114250839_i32),(-488080773_i32),(-307723641_i32),214435605_i32,477244128_i32,(-1797786666_i32),903831843_i32];
_12 = 27182_i16 + 27332_i16;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
33000 => bb6,
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
Goto(bb7)
}
bb7 = {
_15 = _1 as i32;
_14 = [136_u8,104_u8,247_u8,70_u8];
_17.1 = core::ptr::addr_of_mut!(_4);
_12 = (-19427_i16) >> _11;
_19 = 90_i8 as f32;
_10 = _17.1;
_9 = 18049_u16;
_15 = 1224108525_i32;
_16 = _5 as isize;
_7 = [11_i8,34_i8,(-62_i8)];
_7 = [33_i8,52_i8,(-3_i8)];
_17.2 = [(-58_i8),(-82_i8),125_i8];
_18 = (-6334966820975977199_i64) as isize;
_17.0 = _12 & _12;
_17.2 = [(-18_i8),21_i8,119_i8];
_2 = [(*_10),_4,_4];
_5 = 15519300162906536683_u64 / 6149136572445444618_u64;
_17.0 = _11 as i16;
_21 = [_15,_15,_15,_15,_15,_15,_15];
_5 = (*_10) as u64;
_10 = core::ptr::addr_of_mut!(_4);
_18 = _11 | _11;
_7 = _6;
_17.1 = core::ptr::addr_of_mut!((*_10));
_9 = !5600_u16;
_17.2 = [(-3_i8),(-39_i8),(-76_i8)];
Goto(bb8)
}
bb8 = {
_2 = [(*_10),_4,(*_10)];
_10 = core::ptr::addr_of_mut!((*_10));
_20 = 64_u8;
_24 = [1331925533_u32];
(*_10) = _1 ^ _1;
_14 = [_20,_20,_20,_20];
_3 = !_18;
_8 = core::ptr::addr_of_mut!(_23);
_21 = [_15,_15,_15,_15,_15,_15,_15];
_25.fld0 = (_12, _17.1, _7);
(*_8) = (_17.2, _15, (*_10));
_12 = _25.fld0.0;
_9 = !4209_u16;
_25.fld1 = [(-42_i8),109_i8,85_i8];
_5 = _20 as u64;
_26 = '\u{10339e}';
_11 = _18 + _3;
_11 = _26 as isize;
(*_8).2 = !_4;
_18 = _11;
RET.1 = [(*_8).1,(*_8).1,_23.1,_15,_15,_23.1,(*_8).1];
_14 = [_20,_20,_20,_20];
_11 = _16;
(*_8) = (_25.fld1, _15, (*_10));
_17.2 = [(-69_i8),(-107_i8),111_i8];
_17.1 = core::ptr::addr_of_mut!((*_10));
match _23.1 {
0 => bb7,
1224108525 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_25.fld0 = (_12, _17.1, (*_8).0);
_25.fld0 = _17;
_6 = [(-72_i8),(-19_i8),(-15_i8)];
(*_10) = -_23.2;
_12 = _25.fld0.0;
_9 = 42918_u16 & 5867_u16;
(*_8).1 = -_15;
Goto(bb11)
}
bb11 = {
_11 = _3 | _3;
_27 = _11 << _4;
_29.fld2 = 3576193982_u32 & 1681503630_u32;
Goto(bb12)
}
bb12 = {
_25 = Adt52 { fld0: _17,fld1: _7,fld2: _8 };
_19 = (*_8).1 as f32;
_30 = [(*_10),(*_8).2,(*_10)];
match _20 {
0 => bb13,
64 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_12 = true as i16;
_5 = 4765638692413553996_u64 & 1213009901229995988_u64;
_12 = 29990_i16 | 28683_i16;
_11 = -_3;
RET.1 = [(-719995624_i32),1429164869_i32,1720649551_i32,934435937_i32,1355773602_i32,1452207270_i32,(-1969001214_i32)];
_6 = [108_i8,(-21_i8),(-32_i8)];
_16 = -_11;
_1 = _4;
RET.1 = [(-114250839_i32),(-488080773_i32),(-307723641_i32),214435605_i32,477244128_i32,(-1797786666_i32),903831843_i32];
_12 = 27182_i16 + 27332_i16;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
33000 => bb6,
_ => bb5
}
}
bb15 = {
_9 = 32588_u16 % 14752_u16;
RET.1 = [(*_8).1,_23.1,_23.1,_23.1,_23.1,_15,(*_8).1];
_9 = 24195_u16 & 61826_u16;
(*_10) = !(*_8).2;
_6 = _25.fld0.2;
_23.2 = 3320938143767141002_usize as i128;
_29.fld0.1 = [(*_8).1,(*_8).1,(*_8).1,_15,(*_8).1,_15,(*_8).1];
_22 = _19 + _19;
_2 = [(*_10),_4,(*_10)];
Goto(bb16)
}
bb16 = {
_31 = -_22;
(*_8).1 = !_15;
_30 = [_4,(*_8).2,(*_10)];
_26 = '\u{da1e9}';
_24 = [_29.fld2];
_25.fld1 = [110_i8,(-69_i8),(-10_i8)];
_31 = -_19;
_22 = _19 * _19;
_25.fld0.1 = _10;
_26 = '\u{a67}';
_29.fld1.0 = _31 as i32;
(*_8).0 = [57_i8,(-57_i8),96_i8];
Goto(bb17)
}
bb17 = {
match _15 {
0 => bb18,
1224108525 => bb20,
_ => bb19
}
}
bb18 = {
_15 = _1 as i32;
_14 = [136_u8,104_u8,247_u8,70_u8];
_17.1 = core::ptr::addr_of_mut!(_4);
_12 = (-19427_i16) >> _11;
_19 = 90_i8 as f32;
_10 = _17.1;
_9 = 18049_u16;
_15 = 1224108525_i32;
_16 = _5 as isize;
_7 = [11_i8,34_i8,(-62_i8)];
_7 = [33_i8,52_i8,(-3_i8)];
_17.2 = [(-58_i8),(-82_i8),125_i8];
_18 = (-6334966820975977199_i64) as isize;
_17.0 = _12 & _12;
_17.2 = [(-18_i8),21_i8,119_i8];
_2 = [(*_10),_4,_4];
_5 = 15519300162906536683_u64 / 6149136572445444618_u64;
_17.0 = _11 as i16;
_21 = [_15,_15,_15,_15,_15,_15,_15];
_5 = (*_10) as u64;
_10 = core::ptr::addr_of_mut!(_4);
_18 = _11 | _11;
_7 = _6;
_17.1 = core::ptr::addr_of_mut!((*_10));
_9 = !5600_u16;
_17.2 = [(-3_i8),(-39_i8),(-76_i8)];
Goto(bb8)
}
bb19 = {
Return()
}
bb20 = {
(*_8).0 = [(-92_i8),(-101_i8),(-108_i8)];
_33 = _2;
_23.2 = !(*_10);
_4 = (-102_i8) as i128;
(*_8).0 = [59_i8,(-32_i8),(-113_i8)];
_12 = _25.fld0.0 | _17.0;
_25.fld0 = _17;
_25.fld0.0 = 23_i8 as i16;
_19 = _22 - _22;
_19 = 8_i8 as f32;
_36 = _30;
_5 = 3736970247066426610_u64;
_25.fld0.0 = -_12;
_26 = '\u{100e13}';
_3 = _11;
_5 = 16325764999758875041_u64 << _23.2;
_5 = !11669649637301121140_u64;
_23.2 = _29.fld1.0 as i128;
_16 = _11;
_38 = Adt49 { fld0: _22,fld1: (*_8) };
(*_8).2 = _15 as i128;
_39.2 = _38.fld1.2 ^ _1;
(*_8).1 = _38.fld1.1 - _15;
_33 = _36;
match _15 {
0 => bb19,
1 => bb12,
1224108525 => bb21,
_ => bb8
}
}
bb21 = {
(*_8).1 = !_29.fld1.0;
_33 = _2;
_23.2 = -(*_10);
(*_8).1 = true as i32;
_17.0 = _5 as i16;
_10 = core::ptr::addr_of_mut!((*_10));
_22 = _39.2 as f32;
_19 = _22 * _22;
_23 = (_38.fld1.0, _38.fld1.1, _1);
Goto(bb22)
}
bb22 = {
_39.0 = [56_i8,(-16_i8),(-77_i8)];
(*_8) = (_39.0, _15, _4);
(*_8).0 = _25.fld0.2;
_20 = 54_u8;
_38.fld1.0 = [(-66_i8),7_i8,(-22_i8)];
match _15 {
0 => bb23,
1 => bb24,
1224108525 => bb26,
_ => bb25
}
}
bb23 = {
_25 = Adt52 { fld0: _17,fld1: _7,fld2: _8 };
_19 = (*_8).1 as f32;
_30 = [(*_10),(*_8).2,(*_10)];
match _20 {
0 => bb13,
64 => bb15,
_ => bb14
}
}
bb24 = {
_12 = true as i16;
_5 = 4765638692413553996_u64 & 1213009901229995988_u64;
_12 = 29990_i16 | 28683_i16;
_11 = -_3;
RET.1 = [(-719995624_i32),1429164869_i32,1720649551_i32,934435937_i32,1355773602_i32,1452207270_i32,(-1969001214_i32)];
_6 = [108_i8,(-21_i8),(-32_i8)];
_16 = -_11;
_1 = _4;
RET.1 = [(-114250839_i32),(-488080773_i32),(-307723641_i32),214435605_i32,477244128_i32,(-1797786666_i32),903831843_i32];
_12 = 27182_i16 + 27332_i16;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
33000 => bb6,
_ => bb5
}
}
bb25 = {
_12 = true as i16;
_5 = 4765638692413553996_u64 & 1213009901229995988_u64;
_12 = 29990_i16 | 28683_i16;
_11 = -_3;
RET.1 = [(-719995624_i32),1429164869_i32,1720649551_i32,934435937_i32,1355773602_i32,1452207270_i32,(-1969001214_i32)];
_6 = [108_i8,(-21_i8),(-32_i8)];
_16 = -_11;
_1 = _4;
RET.1 = [(-114250839_i32),(-488080773_i32),(-307723641_i32),214435605_i32,477244128_i32,(-1797786666_i32),903831843_i32];
_12 = 27182_i16 + 27332_i16;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
33000 => bb6,
_ => bb5
}
}
bb26 = {
(*_8).2 = _11 as i128;
_39.1 = !(*_8).1;
_25.fld0.0 = -_12;
_7 = [(-48_i8),84_i8,(-46_i8)];
(*_8) = (_25.fld1, _29.fld1.0, _1);
_25.fld0.0 = _12;
_39.2 = _38.fld1.2;
_39.1 = 237453160801222172552894492708932543547_u128 as i32;
(*_10) = _25.fld0.0 as i128;
_38.fld1.2 = _23.2;
_41 = [_23.1,(*_8).1,_23.1,(*_8).1,_23.1,_38.fld1.1,(*_8).1];
_42.0 = [51_i8,(-98_i8),(-95_i8)];
_31 = _20 as f32;
_22 = _11 as f32;
_26 = '\u{68ceb}';
_42.2 = -_4;
_2 = [(*_10),_39.2,(*_10)];
_7 = [35_i8,18_i8,(-7_i8)];
_23.2 = _26 as i128;
(*_8).0 = [(-9_i8),110_i8,(-117_i8)];
_15 = (*_8).1;
_29.fld2 = 459429835_u32 + 2936667730_u32;
_38.fld0 = _22 / f32::NAN;
_28 = [_5,_5,_5,_5,_5,_5,_5,_5];
_38.fld1.2 = (*_10);
_35 = core::ptr::addr_of!(_20);
Goto(bb27)
}
bb27 = {
_34 = _3;
RET.1 = [_15,(*_8).1,_39.1,_23.1,_23.1,_23.1,_15];
_39.2 = _9 as i128;
_25 = Adt52 { fld0: _17,fld1: _23.0,fld2: _8 };
_40 = [_29.fld2];
_25.fld2 = core::ptr::addr_of_mut!(_38.fld1);
_2 = [(*_10),(*_10),_42.2];
RET.1 = _41;
_17.2 = _38.fld1.0;
_43 = -_16;
_37 = _26;
_29.fld0.1 = [_38.fld1.1,(*_8).1,_29.fld1.0,_23.1,(*_8).1,_38.fld1.1,(*_8).1];
_33 = [(*_10),_1,_42.2];
_40 = _24;
_39 = (_17.2, _38.fld1.1, (*_10));
(*_8) = (_42.0, _29.fld1.0, _42.2);
_23 = _38.fld1;
(*_10) = (*_8).2 | _39.2;
_45.0 = [_37,_37];
match (*_35) {
0 => bb23,
54 => bb28,
_ => bb19
}
}
bb28 = {
_12 = -_17.0;
_38 = Adt49 { fld0: _22,fld1: _39 };
_7 = [(-80_i8),(-73_i8),(-40_i8)];
_15 = _20 as i32;
_42 = _38.fld1;
_11 = _29.fld2 as isize;
(*_10) = !_23.2;
_10 = core::ptr::addr_of_mut!(_39.2);
_29.fld2 = (*_35) as u32;
_49 = _29.fld2 as f32;
_23.2 = _39.2;
(*_10) = !_4;
_38 = Adt49 { fld0: _49,fld1: (*_8) };
_21 = [_42.1,(*_8).1,(*_8).1,_39.1,_42.1,(*_8).1,(*_8).1];
_21 = _41;
_29.fld1.0 = _29.fld2 as i32;
_49 = _4 as f32;
(*_10) = _42.2 ^ _4;
match (*_35) {
0 => bb29,
1 => bb30,
2 => bb31,
54 => bb33,
_ => bb32
}
}
bb29 = {
Return()
}
bb30 = {
match _15 {
0 => bb18,
1224108525 => bb20,
_ => bb19
}
}
bb31 = {
_12 = true as i16;
_5 = 4765638692413553996_u64 & 1213009901229995988_u64;
_12 = 29990_i16 | 28683_i16;
_11 = -_3;
RET.1 = [(-719995624_i32),1429164869_i32,1720649551_i32,934435937_i32,1355773602_i32,1452207270_i32,(-1969001214_i32)];
_6 = [108_i8,(-21_i8),(-32_i8)];
_16 = -_11;
_1 = _4;
RET.1 = [(-114250839_i32),(-488080773_i32),(-307723641_i32),214435605_i32,477244128_i32,(-1797786666_i32),903831843_i32];
_12 = 27182_i16 + 27332_i16;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
33000 => bb6,
_ => bb5
}
}
bb32 = {
_25 = Adt52 { fld0: _17,fld1: _7,fld2: _8 };
_19 = (*_8).1 as f32;
_30 = [(*_10),(*_8).2,(*_10)];
match _20 {
0 => bb13,
64 => bb15,
_ => bb14
}
}
bb33 = {
_25.fld1 = [(-20_i8),9_i8,(-13_i8)];
(*_8) = (_7, _15, _1);
(*_8).2 = (*_35) as i128;
_38.fld1 = (*_8);
_25 = Adt52 { fld0: _17,fld1: _17.2,fld2: _8 };
_42.2 = _23.2 | (*_10);
_21 = [(*_8).1,_29.fld1.0,(*_8).1,_39.1,(*_8).1,_15,_38.fld1.1];
_10 = _25.fld0.1;
_24 = [_29.fld2];
_25.fld1 = _42.0;
_36 = _33;
_34 = _27 + _27;
_51 = _3;
_10 = core::ptr::addr_of_mut!(_42.2);
_45.0 = [_37,_37];
_44 = !_9;
_23.0 = [(-75_i8),104_i8,15_i8];
match (*_35) {
0 => bb34,
1 => bb35,
2 => bb36,
3 => bb37,
54 => bb39,
_ => bb38
}
}
bb34 = {
Return()
}
bb35 = {
_25.fld0 = (_12, _17.1, (*_8).0);
_25.fld0 = _17;
_6 = [(-72_i8),(-19_i8),(-15_i8)];
(*_10) = -_23.2;
_12 = _25.fld0.0;
_9 = 42918_u16 & 5867_u16;
(*_8).1 = -_15;
Goto(bb11)
}
bb36 = {
match _15 {
0 => bb18,
1224108525 => bb20,
_ => bb19
}
}
bb37 = {
(*_8).2 = _11 as i128;
_39.1 = !(*_8).1;
_25.fld0.0 = -_12;
_7 = [(-48_i8),84_i8,(-46_i8)];
(*_8) = (_25.fld1, _29.fld1.0, _1);
_25.fld0.0 = _12;
_39.2 = _38.fld1.2;
_39.1 = 237453160801222172552894492708932543547_u128 as i32;
(*_10) = _25.fld0.0 as i128;
_38.fld1.2 = _23.2;
_41 = [_23.1,(*_8).1,_23.1,(*_8).1,_23.1,_38.fld1.1,(*_8).1];
_42.0 = [51_i8,(-98_i8),(-95_i8)];
_31 = _20 as f32;
_22 = _11 as f32;
_26 = '\u{68ceb}';
_42.2 = -_4;
_2 = [(*_10),_39.2,(*_10)];
_7 = [35_i8,18_i8,(-7_i8)];
_23.2 = _26 as i128;
(*_8).0 = [(-9_i8),110_i8,(-117_i8)];
_15 = (*_8).1;
_29.fld2 = 459429835_u32 + 2936667730_u32;
_38.fld0 = _22 / f32::NAN;
_28 = [_5,_5,_5,_5,_5,_5,_5,_5];
_38.fld1.2 = (*_10);
_35 = core::ptr::addr_of!(_20);
Goto(bb27)
}
bb38 = {
Return()
}
bb39 = {
(*_8).2 = !_4;
_47 = _34;
_9 = _44 ^ _44;
_48 = _23.2 | (*_8).2;
Goto(bb40)
}
bb40 = {
_28 = [_5,_5,_5,_5,_5,_5,_5,_5];
_17.0 = _25.fld0.0 & _12;
_29.fld1 = (_42.1,);
_41 = _29.fld0.1;
_50 = [_12,_25.fld0.0,_17.0,_12,_17.0,_25.fld0.0];
_36 = _33;
(*_8).1 = (*_8).2 as i32;
_53 = _3 * _16;
_2 = [_23.2,_1,_42.2];
_38 = Adt49 { fld0: _49,fld1: _42 };
_59 = _22;
_6 = _7;
match (*_35) {
0 => bb31,
1 => bb5,
2 => bb25,
3 => bb4,
4 => bb41,
5 => bb42,
6 => bb43,
54 => bb45,
_ => bb44
}
}
bb41 = {
Return()
}
bb42 = {
match _15 {
0 => bb18,
1224108525 => bb20,
_ => bb19
}
}
bb43 = {
_25.fld0 = (_12, _17.1, (*_8).0);
_25.fld0 = _17;
_6 = [(-72_i8),(-19_i8),(-15_i8)];
(*_10) = -_23.2;
_12 = _25.fld0.0;
_9 = 42918_u16 & 5867_u16;
(*_8).1 = -_15;
Goto(bb11)
}
bb44 = {
_12 = true as i16;
_5 = 4765638692413553996_u64 & 1213009901229995988_u64;
_12 = 29990_i16 | 28683_i16;
_11 = -_3;
RET.1 = [(-719995624_i32),1429164869_i32,1720649551_i32,934435937_i32,1355773602_i32,1452207270_i32,(-1969001214_i32)];
_6 = [108_i8,(-21_i8),(-32_i8)];
_16 = -_11;
_1 = _4;
RET.1 = [(-114250839_i32),(-488080773_i32),(-307723641_i32),214435605_i32,477244128_i32,(-1797786666_i32),903831843_i32];
_12 = 27182_i16 + 27332_i16;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
33000 => bb6,
_ => bb5
}
}
bb45 = {
_56.1 = _39.2 as i32;
(*_8) = _42;
_25.fld0 = (_12, _17.1, _42.0);
_39.2 = 10064758901750311780_usize as i128;
_45.1 = [_5,_5,_5,_5,_5,_5,_5,_5];
_23.2 = !(*_10);
_62.2 = _23.2;
_64 = _7;
_17.0 = -_12;
_34 = 2_usize as isize;
_25.fld0.0 = -_17.0;
_60 = core::ptr::addr_of!(_8);
_16 = !_43;
(*_8).0 = _6;
_52 = [(-17_i8),119_i8,32_i8];
_30 = [_23.2,(*_10),(*_8).2];
_55 = _12 as f64;
_56 = (_25.fld0.2, (*_8).1, _38.fld1.2);
_24 = [_29.fld2];
Call(_37 = fn17((*_8).1, _16, _39.1, _9, _26, _25, _5, _38.fld0, _2, _52), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
_18 = _47 | _3;
_25.fld0.2 = _38.fld1.0;
_59 = _22 - _22;
_61 = !_25.fld0.0;
_62.0 = _39.0;
_17.2 = [0_i8,68_i8,(-35_i8)];
(*_10) = false as i128;
_14 = [(*_35),(*_35),_20,(*_35)];
_40 = [_29.fld2];
_40 = [_29.fld2];
_45.2 = _53 > _27;
match (*_35) {
0 => bb30,
1 => bb17,
2 => bb16,
3 => bb47,
4 => bb48,
54 => bb50,
_ => bb49
}
}
bb47 = {
_12 = -_17.0;
_38 = Adt49 { fld0: _22,fld1: _39 };
_7 = [(-80_i8),(-73_i8),(-40_i8)];
_15 = _20 as i32;
_42 = _38.fld1;
_11 = _29.fld2 as isize;
(*_10) = !_23.2;
_10 = core::ptr::addr_of_mut!(_39.2);
_29.fld2 = (*_35) as u32;
_49 = _29.fld2 as f32;
_23.2 = _39.2;
(*_10) = !_4;
_38 = Adt49 { fld0: _49,fld1: (*_8) };
_21 = [_42.1,(*_8).1,(*_8).1,_39.1,_42.1,(*_8).1,(*_8).1];
_21 = _41;
_29.fld1.0 = _29.fld2 as i32;
_49 = _4 as f32;
(*_10) = _42.2 ^ _4;
match (*_35) {
0 => bb29,
1 => bb30,
2 => bb31,
54 => bb33,
_ => bb32
}
}
bb48 = {
_12 = true as i16;
_5 = 4765638692413553996_u64 & 1213009901229995988_u64;
_12 = 29990_i16 | 28683_i16;
_11 = -_3;
RET.1 = [(-719995624_i32),1429164869_i32,1720649551_i32,934435937_i32,1355773602_i32,1452207270_i32,(-1969001214_i32)];
_6 = [108_i8,(-21_i8),(-32_i8)];
_16 = -_11;
_1 = _4;
RET.1 = [(-114250839_i32),(-488080773_i32),(-307723641_i32),214435605_i32,477244128_i32,(-1797786666_i32),903831843_i32];
_12 = 27182_i16 + 27332_i16;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
33000 => bb6,
_ => bb5
}
}
bb49 = {
Return()
}
bb50 = {
(*_8) = (_64, _38.fld1.1, _4);
_70.fld0 = !272470290017470672142432878146545233115_u128;
_22 = _38.fld0 - _38.fld0;
_48 = _4;
_23.0 = _42.0;
_45.1 = [_5,_5,_5,_5,_5,_5,_5,_5];
_17.1 = core::ptr::addr_of_mut!(_38.fld1.2);
(*_8) = (_42.0, _15, _4);
_70.fld7.fld2.fld0 = (-4230652594569107923_i64) & 336696005709620993_i64;
_70.fld7.fld1 = [_29.fld2];
_56.0 = [(-31_i8),(-128_i8),(-95_i8)];
_17.0 = _61;
_70.fld7.fld0.0 = [(-48_i8),21_i8,65_i8];
_31 = 2136935338748779223_usize as f32;
_35 = core::ptr::addr_of!((*_35));
_70.fld1 = core::ptr::addr_of_mut!(_55);
_64 = [123_i8,(-76_i8),97_i8];
_19 = _38.fld0 - _49;
_30 = [_38.fld1.2,_62.2,_56.2];
(*_8).0 = [126_i8,17_i8,36_i8];
(*_60) = core::ptr::addr_of_mut!(_39);
_71 = (_37,);
Goto(bb51)
}
bb51 = {
_44 = _9 | _9;
(*_8).1 = _42.1;
_26 = _37;
_70.fld7.fld2.fld0 = -(-5198288212779686053_i64);
_42.2 = _56.2 >> _47;
_70.fld7.fld2.fld1 = _71.0;
_72 = Adt52 { fld0: _17,fld1: _42.0,fld2: (*_60) };
_39.1 = _29.fld1.0 + _56.1;
_70.fld7.fld1 = [_29.fld2];
_10 = core::ptr::addr_of_mut!(_70.fld7.fld0.2);
(*_8).0 = _62.0;
_65 = _51;
_9 = (*_35) as u16;
_28 = _45.1;
_70.fld7.fld4.fld1.1 = core::ptr::addr_of!(_10);
_72.fld0.0 = _61;
_28 = [_5,_5,_5,_5,_5,_5,_5,_5];
_72.fld2 = (*_60);
_45.0 = [_37,_70.fld7.fld2.fld1];
_20 = 196_u8 - 87_u8;
_10 = _25.fld0.1;
_24 = [_29.fld2];
_70.fld7.fld2.fld0 = !121283729338092724_i64;
_70.fld7.fld4.fld1.3 = core::ptr::addr_of!(_25.fld0.1);
Goto(bb52)
}
bb52 = {
_2 = _30;
_62 = _42;
_29.fld1 = ((*_8).1,);
_7 = [15_i8,(-118_i8),(-10_i8)];
_67 = -_55;
_70.fld7.fld2.fld2 = core::ptr::addr_of!(_17.1);
_70.fld7.fld0.0 = [24_i8,(-28_i8),93_i8];
(*_8) = (_7, _29.fld1.0, _42.2);
_12 = _17.0 * _25.fld0.0;
_25.fld0.2 = [28_i8,(-59_i8),88_i8];
_70.fld7.fld2.fld1 = _26;
_63 = _67 / (-0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000021140965672285613_f64);
_17.2 = [(-63_i8),77_i8,80_i8];
_17.0 = _25.fld0.0 << _70.fld0;
_17.1 = core::ptr::addr_of_mut!(_42.2);
(*_60) = core::ptr::addr_of_mut!(_23);
_39.1 = _29.fld1.0 << (*_8).2;
_68 = _21;
(*_8) = _39;
_70.fld6 = _29.fld2 / 1902489089_u32;
_70.fld7.fld2.fld0 = _29.fld2 as i64;
_38.fld1.0 = [(-69_i8),123_i8,34_i8];
_70.fld7.fld0 = _23;
(*_8).0 = _70.fld7.fld0.0;
_47 = -_18;
_39.0 = [(-29_i8),80_i8,3_i8];
_70.fld7.fld3.0 = core::ptr::addr_of_mut!(_70.fld7.fld2.fld0);
Goto(bb53)
}
bb53 = {
_70.fld2 = -_27;
(*_8).0 = [(-29_i8),31_i8,73_i8];
_39.2 = !(*_8).2;
_64 = [(-123_i8),(-82_i8),(-128_i8)];
_38 = Adt49 { fld0: _59,fld1: (*_8) };
(*_60) = _25.fld2;
_79 = !_45.2;
(*_60) = _72.fld2;
_70.fld5 = _23.1 * _38.fld1.1;
_5 = _38.fld1.2 as u64;
_41 = _68;
_38.fld1 = (_23.0, (*_8).1, _39.2);
_15 = (*_8).1;
_62.0 = _56.0;
_8 = core::ptr::addr_of_mut!(_39);
_35 = core::ptr::addr_of!((*_35));
_78 = [_29.fld2];
_77 = [_29.fld2];
_31 = _22 - _59;
_7 = _64;
_53 = _38.fld1.2 as isize;
_70.fld5 = (*_8).1;
(*_8).0 = [(-122_i8),66_i8,(-64_i8)];
_25.fld0 = (_17.0, _72.fld0.1, _38.fld1.0);
_70.fld6 = !_29.fld2;
_71.0 = _37;
_38.fld1.2 = _23.1 as i128;
Goto(bb54)
}
bb54 = {
_31 = -_38.fld0;
_79 = _45.2 | _45.2;
_72.fld1 = [96_i8,85_i8,93_i8];
_70.fld2 = _70.fld7.fld2.fld0 as isize;
_59 = _61 as f32;
_70.fld0 = !14290612769996929360241692176409150832_u128;
_70.fld1 = core::ptr::addr_of_mut!(_76);
_8 = _72.fld2;
_70.fld6 = _29.fld2;
_42 = ((*_8).0, (*_8).1, (*_8).2);
Goto(bb55)
}
bb55 = {
_67 = -_63;
_70.fld7.fld0.1 = -_23.1;
_29.fld1.0 = _70.fld7.fld0.1;
Goto(bb56)
}
bb56 = {
_56.0 = _72.fld1;
_70.fld1 = core::ptr::addr_of_mut!(_63);
_23.1 = _39.1 >> _47;
_66 = _70.fld7.fld2.fld0 as usize;
_70.fld6 = _29.fld2 ^ _29.fld2;
_61 = _72.fld0.0;
Goto(bb57)
}
bb57 = {
_51 = _18;
_52 = [(-97_i8),88_i8,(-81_i8)];
_84 = _20 as i32;
(*_10) = _5 as i128;
_76 = -_63;
_60 = core::ptr::addr_of!(_8);
_77 = [_29.fld2];
Goto(bb58)
}
bb58 = {
(*_35) = 174_u8 << _42.2;
_47 = _53 * _27;
_16 = _51 - _53;
_52 = _72.fld0.2;
_32 = _70.fld1;
_70.fld4 = [_29.fld2];
_10 = core::ptr::addr_of_mut!((*_8).2);
_39 = (_56.0, _42.1, _4);
_77 = [_70.fld6];
_1 = _20 as i128;
_70.fld2 = _16;
_75 = (*_8).1;
_81 = _5 as f64;
_70.fld5 = (*_8).1;
RET.1 = _21;
_38.fld1.1 = (*_8).1;
_90.0 = _26;
_70.fld7.fld2.fld1 = _90.0;
_82 = _35;
_86.2 = _29.fld1.0 >= _23.1;
_42.2 = -(*_8).2;
_58 = [_17.0,_25.fld0.0,_17.0,_12,_17.0,_61];
Goto(bb59)
}
bb59 = {
_91.1 = _28;
_70.fld6 = _29.fld2 / 293772678_u32;
Goto(bb60)
}
bb60 = {
_58 = [_17.0,_72.fld0.0,_12,_17.0,_72.fld0.0,_72.fld0.0];
_96.fld0 = [_26,_90.0];
_56.1 = _47 as i32;
_23.2 = _39.2 & _4;
(*_60) = core::ptr::addr_of_mut!(_39);
(*_60) = core::ptr::addr_of_mut!(_23);
_96.fld0 = _45.0;
(*_10) = -_62.2;
_96.fld2.fld1 = (_15,);
Call(_38.fld1.2 = core::intrinsics::bswap((*_8).2), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
_4 = _1 + (*_8).2;
_44 = _9 ^ _9;
_70.fld7.fld2 = Adt50 { fld0: (-3665357094449505727_i64),fld1: _71.0,fld2: _70.fld7.fld4.fld1.3 };
_72.fld0.1 = _10;
_61 = _72.fld0.0;
_72.fld0.0 = _17.0 << (*_35);
_72.fld1 = [117_i8,105_i8,(-111_i8)];
_97.0 = _5;
_89.fld3 = core::ptr::addr_of!(_72.fld0.1);
(*_8).0 = _25.fld0.2;
_70.fld7.fld3.0 = core::ptr::addr_of_mut!(_101.fld0.fld2.fld0);
_93 = !_97.0;
_25 = Adt52 { fld0: _72.fld0,fld1: _38.fld1.0,fld2: _8 };
(*_82) = 128_u8;
_101.fld1.fld1 = _29.fld1;
_2 = [(*_10),(*_8).2,(*_8).2];
_96.fld2.fld2 = _70.fld6;
_101.fld0.fld7 = _41;
_73 = core::ptr::addr_of_mut!(_54);
_70.fld7.fld4.fld1.1 = core::ptr::addr_of!(_72.fld0.1);
_23 = _56;
match _70.fld7.fld2.fld0 {
0 => bb51,
340282366920938463459709250337318705729 => bb62,
_ => bb39
}
}
bb62 = {
_91 = _45;
_39 = (_42.0, _75, _23.2);
_26 = _70.fld7.fld2.fld1;
_70.fld7.fld4.fld1.0 = _97.0 / 11465985004096603996_u64;
(*_8).1 = _38.fld0 as i32;
_34 = _70.fld7.fld4.fld1.0 as isize;
_101.fld0.fld2.fld2 = core::ptr::addr_of!(_17.1);
Goto(bb63)
}
bb63 = {
_10 = _17.1;
_83 = -_53;
_5 = _81 as u64;
_96.fld2.fld1.0 = (*_8).1 << _18;
_25 = _72;
_70.fld7.fld2.fld0 = 3465514032671885469_i64;
_20 = !26_u8;
match _70.fld7.fld2.fld0 {
0 => bb64,
1 => bb65,
2 => bb66,
3 => bb67,
4 => bb68,
5 => bb69,
6 => bb70,
3465514032671885469 => bb72,
_ => bb71
}
}
bb64 = {
(*_8).2 = _11 as i128;
_39.1 = !(*_8).1;
_25.fld0.0 = -_12;
_7 = [(-48_i8),84_i8,(-46_i8)];
(*_8) = (_25.fld1, _29.fld1.0, _1);
_25.fld0.0 = _12;
_39.2 = _38.fld1.2;
_39.1 = 237453160801222172552894492708932543547_u128 as i32;
(*_10) = _25.fld0.0 as i128;
_38.fld1.2 = _23.2;
_41 = [_23.1,(*_8).1,_23.1,(*_8).1,_23.1,_38.fld1.1,(*_8).1];
_42.0 = [51_i8,(-98_i8),(-95_i8)];
_31 = _20 as f32;
_22 = _11 as f32;
_26 = '\u{68ceb}';
_42.2 = -_4;
_2 = [(*_10),_39.2,(*_10)];
_7 = [35_i8,18_i8,(-7_i8)];
_23.2 = _26 as i128;
(*_8).0 = [(-9_i8),110_i8,(-117_i8)];
_15 = (*_8).1;
_29.fld2 = 459429835_u32 + 2936667730_u32;
_38.fld0 = _22 / f32::NAN;
_28 = [_5,_5,_5,_5,_5,_5,_5,_5];
_38.fld1.2 = (*_10);
_35 = core::ptr::addr_of!(_20);
Goto(bb27)
}
bb65 = {
_25.fld0 = (_12, _17.1, (*_8).0);
_25.fld0 = _17;
_6 = [(-72_i8),(-19_i8),(-15_i8)];
(*_10) = -_23.2;
_12 = _25.fld0.0;
_9 = 42918_u16 & 5867_u16;
(*_8).1 = -_15;
Goto(bb11)
}
bb66 = {
Return()
}
bb67 = {
_25.fld1 = [(-20_i8),9_i8,(-13_i8)];
(*_8) = (_7, _15, _1);
(*_8).2 = (*_35) as i128;
_38.fld1 = (*_8);
_25 = Adt52 { fld0: _17,fld1: _17.2,fld2: _8 };
_42.2 = _23.2 | (*_10);
_21 = [(*_8).1,_29.fld1.0,(*_8).1,_39.1,(*_8).1,_15,_38.fld1.1];
_10 = _25.fld0.1;
_24 = [_29.fld2];
_25.fld1 = _42.0;
_36 = _33;
_34 = _27 + _27;
_51 = _3;
_10 = core::ptr::addr_of_mut!(_42.2);
_45.0 = [_37,_37];
_44 = !_9;
_23.0 = [(-75_i8),104_i8,15_i8];
match (*_35) {
0 => bb34,
1 => bb35,
2 => bb36,
3 => bb37,
54 => bb39,
_ => bb38
}
}
bb68 = {
(*_35) = 174_u8 << _42.2;
_47 = _53 * _27;
_16 = _51 - _53;
_52 = _72.fld0.2;
_32 = _70.fld1;
_70.fld4 = [_29.fld2];
_10 = core::ptr::addr_of_mut!((*_8).2);
_39 = (_56.0, _42.1, _4);
_77 = [_70.fld6];
_1 = _20 as i128;
_70.fld2 = _16;
_75 = (*_8).1;
_81 = _5 as f64;
_70.fld5 = (*_8).1;
RET.1 = _21;
_38.fld1.1 = (*_8).1;
_90.0 = _26;
_70.fld7.fld2.fld1 = _90.0;
_82 = _35;
_86.2 = _29.fld1.0 >= _23.1;
_42.2 = -(*_8).2;
_58 = [_17.0,_25.fld0.0,_17.0,_12,_17.0,_61];
Goto(bb59)
}
bb69 = {
(*_8).0 = [(-92_i8),(-101_i8),(-108_i8)];
_33 = _2;
_23.2 = !(*_10);
_4 = (-102_i8) as i128;
(*_8).0 = [59_i8,(-32_i8),(-113_i8)];
_12 = _25.fld0.0 | _17.0;
_25.fld0 = _17;
_25.fld0.0 = 23_i8 as i16;
_19 = _22 - _22;
_19 = 8_i8 as f32;
_36 = _30;
_5 = 3736970247066426610_u64;
_25.fld0.0 = -_12;
_26 = '\u{100e13}';
_3 = _11;
_5 = 16325764999758875041_u64 << _23.2;
_5 = !11669649637301121140_u64;
_23.2 = _29.fld1.0 as i128;
_16 = _11;
_38 = Adt49 { fld0: _22,fld1: (*_8) };
(*_8).2 = _15 as i128;
_39.2 = _38.fld1.2 ^ _1;
(*_8).1 = _38.fld1.1 - _15;
_33 = _36;
match _15 {
0 => bb19,
1 => bb12,
1224108525 => bb21,
_ => bb8
}
}
bb70 = {
_11 = _3 | _3;
_27 = _11 << _4;
_29.fld2 = 3576193982_u32 & 1681503630_u32;
Goto(bb12)
}
bb71 = {
_12 = true as i16;
_5 = 4765638692413553996_u64 & 1213009901229995988_u64;
_12 = 29990_i16 | 28683_i16;
_11 = -_3;
RET.1 = [(-719995624_i32),1429164869_i32,1720649551_i32,934435937_i32,1355773602_i32,1452207270_i32,(-1969001214_i32)];
_6 = [108_i8,(-21_i8),(-32_i8)];
_16 = -_11;
_1 = _4;
RET.1 = [(-114250839_i32),(-488080773_i32),(-307723641_i32),214435605_i32,477244128_i32,(-1797786666_i32),903831843_i32];
_12 = 27182_i16 + 27332_i16;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
33000 => bb6,
_ => bb5
}
}
bb72 = {
(*_73) = (-40_i8) as u64;
_96.fld2.fld1.0 = (*_8).1;
_45.1 = _28;
_38 = Adt49 { fld0: _22,fld1: (*_8) };
_33 = _2;
_97.3 = core::ptr::addr_of!(_10);
_101.fld1.fld2 = _96.fld2.fld2;
_50 = [_72.fld0.0,_72.fld0.0,_25.fld0.0,_25.fld0.0,_72.fld0.0,_25.fld0.0];
_17.1 = _10;
_70.fld7.fld2.fld2 = core::ptr::addr_of!(_72.fld0.1);
_39.1 = !_29.fld1.0;
_39.2 = (*_8).2 << _25.fld0.0;
_70.fld4 = [_70.fld6];
_101.fld0.fld5 = _38.fld0 as u128;
_81 = _9 as f64;
_86.0 = _96.fld0;
_80 = _67 as isize;
_72.fld0 = (_25.fld0.0, _25.fld0.1, _25.fld0.2);
_56.1 = _42.1;
_104.0 = _38.fld1.1 ^ _38.fld1.1;
_29.fld2 = _101.fld1.fld2 * _96.fld2.fld2;
_70.fld7.fld4.fld0 = _78;
_45.0 = [_70.fld7.fld2.fld1,_37];
_101.fld0.fld0 = core::ptr::addr_of_mut!(_17);
_67 = -_76;
match _70.fld7.fld2.fld0 {
3465514032671885469 => bb73,
_ => bb26
}
}
bb73 = {
_95.1 = _22 as i32;
_70.fld0 = _70.fld7.fld2.fld0 as u128;
_17 = _72.fld0;
_45.2 = _86.2;
(*_60) = core::ptr::addr_of_mut!(_70.fld7.fld0);
_38 = Adt49 { fld0: _19,fld1: _42 };
_108.fld1.fld1.2 = !_1;
_16 = -_70.fld2;
_70.fld7.fld0.2 = _66 as i128;
_66 = 3560253084242463608_usize;
_50 = [_17.0,_25.fld0.0,_72.fld0.0,_17.0,_25.fld0.0,_25.fld0.0];
_70.fld7.fld4.fld1.0 = _93;
_42.1 = _66 as i32;
match _70.fld7.fld2.fld0 {
3465514032671885469 => bb75,
_ => bb74
}
}
bb74 = {
(*_8).0 = [(-92_i8),(-101_i8),(-108_i8)];
_33 = _2;
_23.2 = !(*_10);
_4 = (-102_i8) as i128;
(*_8).0 = [59_i8,(-32_i8),(-113_i8)];
_12 = _25.fld0.0 | _17.0;
_25.fld0 = _17;
_25.fld0.0 = 23_i8 as i16;
_19 = _22 - _22;
_19 = 8_i8 as f32;
_36 = _30;
_5 = 3736970247066426610_u64;
_25.fld0.0 = -_12;
_26 = '\u{100e13}';
_3 = _11;
_5 = 16325764999758875041_u64 << _23.2;
_5 = !11669649637301121140_u64;
_23.2 = _29.fld1.0 as i128;
_16 = _11;
_38 = Adt49 { fld0: _22,fld1: (*_8) };
(*_8).2 = _15 as i128;
_39.2 = _38.fld1.2 ^ _1;
(*_8).1 = _38.fld1.1 - _15;
_33 = _36;
match _15 {
0 => bb19,
1 => bb12,
1224108525 => bb21,
_ => bb8
}
}
bb75 = {
_16 = !_65;
_97.0 = _5;
(*_32) = _76 + _76;
_6 = _39.0;
_101.fld1.fld0.0 = core::ptr::addr_of_mut!(_108.fld0);
_35 = core::ptr::addr_of!((*_35));
_101.fld1.fld2 = _70.fld6 - _29.fld2;
_93 = _5 + _70.fld7.fld4.fld1.0;
_70.fld1 = core::ptr::addr_of_mut!(_81);
_13 = core::ptr::addr_of_mut!(_108.fld0);
_101.fld0.fld0 = core::ptr::addr_of_mut!(_72.fld0);
(*_82) = 69_u8 & 73_u8;
_89.fld2.0 = core::ptr::addr_of_mut!((*_13));
(*_13).0 = !_43;
_91 = (_86.0, _28, _86.2);
(*_13).3 = !(-96_i8);
_70.fld7.fld0.1 = !_101.fld1.fld1.0;
_101.fld1.fld0 = (_89.fld2.0, _101.fld0.fld7);
_65 = _16;
_101.fld0.fld6.fld1.1 = _70.fld7.fld4.fld1.3;
match _70.fld7.fld2.fld0 {
0 => bb49,
1 => bb27,
2 => bb74,
3 => bb71,
4 => bb11,
5 => bb48,
6 => bb76,
3465514032671885469 => bb78,
_ => bb77
}
}
bb76 = {
match _15 {
0 => bb18,
1224108525 => bb20,
_ => bb19
}
}
bb77 = {
_4 = _1 + (*_8).2;
_44 = _9 ^ _9;
_70.fld7.fld2 = Adt50 { fld0: (-3665357094449505727_i64),fld1: _71.0,fld2: _70.fld7.fld4.fld1.3 };
_72.fld0.1 = _10;
_61 = _72.fld0.0;
_72.fld0.0 = _17.0 << (*_35);
_72.fld1 = [117_i8,105_i8,(-111_i8)];
_97.0 = _5;
_89.fld3 = core::ptr::addr_of!(_72.fld0.1);
(*_8).0 = _25.fld0.2;
_70.fld7.fld3.0 = core::ptr::addr_of_mut!(_101.fld0.fld2.fld0);
_93 = !_97.0;
_25 = Adt52 { fld0: _72.fld0,fld1: _38.fld1.0,fld2: _8 };
(*_82) = 128_u8;
_101.fld1.fld1 = _29.fld1;
_2 = [(*_10),(*_8).2,(*_8).2];
_96.fld2.fld2 = _70.fld6;
_101.fld0.fld7 = _41;
_73 = core::ptr::addr_of_mut!(_54);
_70.fld7.fld4.fld1.1 = core::ptr::addr_of!(_72.fld0.1);
_23 = _56;
match _70.fld7.fld2.fld0 {
0 => bb51,
340282366920938463459709250337318705729 => bb62,
_ => bb39
}
}
bb78 = {
_95.1 = _34 as i32;
_96 = Adt62 { fld0: _45.0,fld1: _70.fld0,fld2: _101.fld1 };
_108.fld1.fld1.2 = _42.2 << _4;
_87 = [_1,_39.2,_39.2];
_96.fld1 = _101.fld0.fld5 << _17.0;
_35 = core::ptr::addr_of!((*_82));
_108.fld0 = (_47, _104, _10, 23_i8, (-36_i8), _10, _25.fld0.0);
_51 = -(*_13).0;
_45.1 = [_5,_93,_5,_5,_5,_97.0,_5,_93];
_62.2 = _93 as i128;
_21 = _101.fld0.fld7;
(*_10) = _22 as i128;
_108.fld1 = Adt49 { fld0: _31,fld1: _39 };
_97 = (_70.fld7.fld4.fld1.0, _70.fld7.fld2.fld2, _25.fld1, _101.fld0.fld6.fld1.1);
_57 = [_37,_71.0];
_70.fld5 = -(*_13).1.0;
_105 = _86.2;
_102 = _26;
Goto(bb79)
}
bb79 = {
_25.fld0.0 = (*_13).6;
_108.fld0.2 = _17.1;
RET.0 = _89.fld2.0;
_95.1 = _108.fld0.1.0 & _70.fld5;
_5 = _70.fld7.fld4.fld1.0 ^ _97.0;
_70.fld3 = _108.fld0.4 | (*_13).4;
_96.fld2.fld0.1 = [_95.1,(*_13).1.0,_95.1,_29.fld1.0,_70.fld5,(*_13).1.0,_70.fld5];
_101.fld1.fld0.0 = core::ptr::addr_of_mut!((*_13));
_114 = _53 + (*_13).0;
_72.fld0.0 = _70.fld2 as i16;
_70.fld7.fld4 = Adt51 { fld0: _70.fld7.fld1,fld1: _97,fld2: _96.fld2.fld0.0 };
_70.fld7.fld4 = Adt51 { fld0: _78,fld1: _97,fld2: _89.fld2.0 };
_96.fld2.fld0 = (_89.fld2.0, _21);
_101.fld0.fld2.fld0 = -_70.fld7.fld2.fld0;
_63 = _5 as f64;
_97.1 = core::ptr::addr_of!(_17.1);
Goto(bb80)
}
bb80 = {
Call(_122 = dump_var(16_usize, 12_usize, Move(_12), 61_usize, Move(_61), 52_usize, Move(_52), 7_usize, Move(_7)), ReturnTo(bb81), UnwindUnreachable())
}
bb81 = {
Call(_122 = dump_var(16_usize, 104_usize, Move(_104), 102_usize, Move(_102), 93_usize, Move(_93), 78_usize, Move(_78)), ReturnTo(bb82), UnwindUnreachable())
}
bb82 = {
Call(_122 = dump_var(16_usize, 1_usize, Move(_1), 33_usize, Move(_33), 41_usize, Move(_41), 62_usize, Move(_62)), ReturnTo(bb83), UnwindUnreachable())
}
bb83 = {
Call(_122 = dump_var(16_usize, 3_usize, Move(_3), 47_usize, Move(_47), 105_usize, Move(_105), 40_usize, Move(_40)), ReturnTo(bb84), UnwindUnreachable())
}
bb84 = {
Call(_122 = dump_var(16_usize, 20_usize, Move(_20), 11_usize, Move(_11), 91_usize, Move(_91), 36_usize, Move(_36)), ReturnTo(bb85), UnwindUnreachable())
}
bb85 = {
Call(_122 = dump_var(16_usize, 90_usize, Move(_90), 44_usize, Move(_44), 27_usize, Move(_27), 39_usize, Move(_39)), ReturnTo(bb86), UnwindUnreachable())
}
bb86 = {
Call(_122 = dump_var(16_usize, 48_usize, Move(_48), 65_usize, Move(_65), 77_usize, Move(_77), 114_usize, Move(_114)), ReturnTo(bb87), UnwindUnreachable())
}
bb87 = {
Call(_122 = dump_var(16_usize, 79_usize, Move(_79), 2_usize, Move(_2), 51_usize, Move(_51), 68_usize, Move(_68)), ReturnTo(bb88), UnwindUnreachable())
}
bb88 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i32,mut _2: isize,mut _3: i32,mut _4: u16,mut _5: char,mut _6: Adt52,mut _7: u64,mut _8: f32,mut _9: [i128; 3],mut _10: [i8; 3]) -> char {
mir! {
type RET = char;
let _11: f64;
let _12: Adt51;
let _13: [i8; 3];
let _14: [i16; 6];
let _15: u128;
let _16: [char; 2];
let _17: f32;
let _18: i64;
let _19: f64;
let _20: (i32,);
let _21: Adt62;
let _22: [u64; 8];
let _23: isize;
let _24: isize;
let _25: *const *mut ([i8; 3], i32, i128);
let _26: f64;
let _27: Adt50;
let _28: bool;
let _29: f32;
let _30: [i16; 6];
let _31: char;
let _32: isize;
let _33: [u64; 8];
let _34: char;
let _35: [u8; 4];
let _36: bool;
let _37: Adt54;
let _38: (*mut (i16, *mut i128, [i8; 3]), i32);
let _39: i128;
let _40: [i8; 3];
let _41: isize;
let _42: ([char; 2], [u64; 8], bool);
let _43: i64;
let _44: isize;
let _45: ();
let _46: ();
{
_5 = '\u{ca272}';
RET = _5;
RET = _5;
_6.fld1 = [42_i8,(-43_i8),(-107_i8)];
_4 = 16399_u16;
_7 = !1975243491669253285_u64;
_11 = 15702622359951649181906145922881810198_u128 as f64;
_12.fld1.3 = core::ptr::addr_of!(_6.fld0.1);
_12.fld1.2 = [120_i8,(-100_i8),87_i8];
_12.fld1.1 = _12.fld1.3;
Goto(bb1)
}
bb1 = {
_11 = _1 as f64;
RET = _5;
_6.fld0.0 = -70_i16;
_6.fld0.2 = [(-23_i8),12_i8,35_i8];
_12.fld0 = [3795764795_u32];
RET = _5;
_12.fld1.3 = core::ptr::addr_of!(_6.fld0.1);
_12.fld1.3 = _12.fld1.1;
_11 = _4 as f64;
_6.fld0.2 = [51_i8,10_i8,(-20_i8)];
_6.fld0.2 = [(-21_i8),97_i8,108_i8];
_11 = 110_i8 as f64;
_12.fld1.3 = _12.fld1.1;
RET = _5;
Goto(bb2)
}
bb2 = {
_6.fld1 = _6.fld0.2;
_12.fld1.2 = _10;
_1 = _3;
_12.fld1.2 = [53_i8,89_i8,40_i8];
_15 = 226692578048300135469901146153860444613_u128;
Goto(bb3)
}
bb3 = {
_13 = [(-15_i8),(-13_i8),(-17_i8)];
_12.fld1.0 = _7 ^ _7;
_12.fld1.0 = _7;
_11 = 2133634100901139180_i64 as f64;
_11 = _4 as f64;
_11 = 1408555893_u32 as f64;
_10 = [60_i8,94_i8,11_i8];
_16 = [_5,_5];
_17 = _8;
_16 = [_5,_5];
_15 = 93188828516709941592639903425478985868_u128 / 191920308144350191510455156803713106687_u128;
_18 = (-8595654105726688891_i64) - 1484114420397337682_i64;
_6.fld0.0 = (-15880_i16);
RET = _5;
_12.fld1.0 = !_7;
Goto(bb4)
}
bb4 = {
_12.fld1.0 = _7;
_12.fld1.3 = _12.fld1.1;
_14 = [_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0];
_8 = -_17;
_10 = [46_i8,(-95_i8),(-46_i8)];
RET = _5;
_14 = [_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0];
Goto(bb5)
}
bb5 = {
_12.fld1.2 = _10;
_2 = 9223372036854775807_isize;
_20 = (_1,);
RET = _5;
_19 = _6.fld0.0 as f64;
Call(_11 = fn18(_7, _6.fld2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_21.fld1 = _15 - _15;
_6.fld0.2 = _10;
_21.fld2.fld1.0 = -_20.0;
Goto(bb7)
}
bb7 = {
_13 = _12.fld1.2;
_19 = _11 / 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005952069084332525_f64;
_12.fld1.2 = _6.fld1;
_17 = 4015242713_u32 as f32;
_21.fld2.fld2 = !4275553838_u32;
_20.0 = _1;
_10 = [(-81_i8),(-59_i8),(-33_i8)];
match _2 {
0 => bb2,
1 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb8 = {
_12.fld1.0 = _7;
_12.fld1.3 = _12.fld1.1;
_14 = [_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0];
_8 = -_17;
_10 = [46_i8,(-95_i8),(-46_i8)];
RET = _5;
_14 = [_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0];
Goto(bb5)
}
bb9 = {
_11 = _1 as f64;
RET = _5;
_6.fld0.0 = -70_i16;
_6.fld0.2 = [(-23_i8),12_i8,35_i8];
_12.fld0 = [3795764795_u32];
RET = _5;
_12.fld1.3 = core::ptr::addr_of!(_6.fld0.1);
_12.fld1.3 = _12.fld1.1;
_11 = _4 as f64;
_6.fld0.2 = [51_i8,10_i8,(-20_i8)];
_6.fld0.2 = [(-21_i8),97_i8,108_i8];
_11 = 110_i8 as f64;
_12.fld1.3 = _12.fld1.1;
RET = _5;
Goto(bb2)
}
bb10 = {
_21.fld1 = _15 / 120499627901364879365472157106331037135_u128;
_21.fld2.fld2 = 3471625662_u32;
_21.fld2.fld0.1 = [_3,_3,_3,_1,_20.0,_1,_1];
_6.fld1 = _10;
_21.fld0 = [_5,_5];
match _21.fld2.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3471625662 => bb11,
_ => bb9
}
}
bb11 = {
_17 = _8 * _8;
_12.fld1.0 = _21.fld2.fld2 as u64;
_27 = Adt50 { fld0: _18,fld1: _5,fld2: _12.fld1.3 };
_12.fld0 = [_21.fld2.fld2];
_24 = !_2;
_7 = _12.fld1.0;
_6.fld0.0 = -6523_i16;
_27 = Adt50 { fld0: _18,fld1: _5,fld2: _12.fld1.3 };
_9 = [(-73306646662408182741854711150814560368_i128),(-107045850073951440571142277065184563602_i128),(-35653404613397697230839884387108352925_i128)];
_28 = !true;
_12.fld1.1 = core::ptr::addr_of!(_6.fld0.1);
_22 = [_12.fld1.0,_7,_12.fld1.0,_12.fld1.0,_12.fld1.0,_7,_12.fld1.0,_7];
_17 = _8;
_6.fld0.0 = (-24289_i16) - 22614_i16;
_9 = [100160817794746808805185870735192174745_i128,5619254347250923941695569303506487780_i128,(-125582800056619466279773864421417886190_i128)];
_6.fld0.0 = -24660_i16;
_2 = _6.fld0.0 as isize;
_24 = _2;
_22 = [_7,_7,_7,_7,_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0];
_6.fld0.2 = _13;
_22 = [_7,_12.fld1.0,_7,_12.fld1.0,_7,_7,_12.fld1.0,_12.fld1.0];
_1 = _20.0;
Goto(bb12)
}
bb12 = {
_18 = _27.fld0 * _27.fld0;
_25 = core::ptr::addr_of!(_6.fld2);
_27 = Adt50 { fld0: _18,fld1: _5,fld2: _12.fld1.3 };
_21.fld1 = _15 / 292741464035601894550052965886441569539_u128;
_29 = _8;
_14 = [_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0];
_8 = _15 as f32;
_34 = _5;
RET = _27.fld1;
_6.fld1 = [(-10_i8),95_i8,(-99_i8)];
_21.fld2.fld0.1 = [_1,_20.0,_1,_1,_20.0,_21.fld2.fld1.0,_21.fld2.fld1.0];
_12.fld1.0 = _7 % 4145916576719449828_u64;
_37.fld1 = [87_u8,100_u8,98_u8,120_u8];
_37.fld4 = -_6.fld0.0;
_10 = _13;
_37.fld6.fld1.1 = core::ptr::addr_of!(_6.fld0.1);
Goto(bb13)
}
bb13 = {
_39 = 109046928840880895526574540618217246420_i128 | 125483834857256239478360457839347047733_i128;
_40 = [(-107_i8),126_i8,(-61_i8)];
_15 = _21.fld1 - _21.fld1;
_14 = [_37.fld4,_6.fld0.0,_37.fld4,_37.fld4,_37.fld4,_37.fld4];
_6.fld0.0 = _37.fld4;
_27.fld1 = _5;
_32 = _2;
_27.fld2 = core::ptr::addr_of!(_6.fld0.1);
_22 = [_12.fld1.0,_7,_12.fld1.0,_12.fld1.0,_7,_12.fld1.0,_7,_12.fld1.0];
_38.0 = core::ptr::addr_of_mut!(_6.fld0);
_27.fld0 = -_18;
match _21.fld2.fld2 {
0 => bb4,
1 => bb14,
2 => bb15,
3471625662 => bb17,
_ => bb16
}
}
bb14 = {
_6.fld1 = _6.fld0.2;
_12.fld1.2 = _10;
_1 = _3;
_12.fld1.2 = [53_i8,89_i8,40_i8];
_15 = 226692578048300135469901146153860444613_u128;
Goto(bb3)
}
bb15 = {
_11 = _1 as f64;
RET = _5;
_6.fld0.0 = -70_i16;
_6.fld0.2 = [(-23_i8),12_i8,35_i8];
_12.fld0 = [3795764795_u32];
RET = _5;
_12.fld1.3 = core::ptr::addr_of!(_6.fld0.1);
_12.fld1.3 = _12.fld1.1;
_11 = _4 as f64;
_6.fld0.2 = [51_i8,10_i8,(-20_i8)];
_6.fld0.2 = [(-21_i8),97_i8,108_i8];
_11 = 110_i8 as f64;
_12.fld1.3 = _12.fld1.1;
RET = _5;
Goto(bb2)
}
bb16 = {
_12.fld1.0 = _7;
_12.fld1.3 = _12.fld1.1;
_14 = [_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0];
_8 = -_17;
_10 = [46_i8,(-95_i8),(-46_i8)];
RET = _5;
_14 = [_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0,_6.fld0.0];
Goto(bb5)
}
bb17 = {
_37.fld1 = [190_u8,206_u8,165_u8,132_u8];
_36 = !_28;
_15 = _21.fld1 & _21.fld1;
_43 = -_18;
_39 = !(-152566939528153237006180067689843114025_i128);
_33 = _22;
_37.fld2.fld0 = _18 >> _21.fld2.fld2;
_36 = _37.fld2.fld0 <= _18;
_41 = _12.fld1.0 as isize;
_37.fld1 = [226_u8,51_u8,125_u8,124_u8];
_43 = -_27.fld0;
Goto(bb18)
}
bb18 = {
Call(_45 = dump_var(17_usize, 7_usize, Move(_7), 3_usize, Move(_3), 13_usize, Move(_13), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(17_usize, 9_usize, Move(_9), 14_usize, Move(_14), 43_usize, Move(_43), 22_usize, Move(_22)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(17_usize, 24_usize, Move(_24), 20_usize, Move(_20), 40_usize, Move(_40), 41_usize, Move(_41)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: u64,mut _2: *mut ([i8; 3], i32, i128)) -> f64 {
mir! {
type RET = f64;
let _3: isize;
let _4: u32;
let _5: i64;
let _6: Adt62;
let _7: char;
let _8: (u64, *const *mut i128, [i8; 3], *const *mut i128);
let _9: (*mut (i16, *mut i128, [i8; 3]), i8, i64, usize);
let _10: (i16, *mut i128, [i8; 3]);
let _11: *mut bool;
let _12: [i16; 6];
let _13: f32;
let _14: Adt62;
let _15: bool;
let _16: isize;
let _17: isize;
let _18: u8;
let _19: (*mut (i16, *mut i128, [i8; 3]), i32);
let _20: ([char; 2], [u64; 8], bool);
let _21: Adt56;
let _22: isize;
let _23: isize;
let _24: ();
let _25: ();
{
(*_2).1 = !(-1998272162_i32);
RET = (*_2).1 as f64;
(*_2).0 = [(-81_i8),11_i8,72_i8];
_1 = '\u{b218}' as u64;
(*_2).2 = (-9223372036854775808_isize) as i128;
(*_2).0 = [121_i8,(-26_i8),(-33_i8)];
RET = 196_u8 as f64;
(*_2).1 = (-9223372036854775808_isize) as i32;
_3 = -(-9223372036854775808_isize);
(*_2).0 = [8_i8,37_i8,(-26_i8)];
(*_2).1 = -986293239_i32;
_4 = !2719569762_u32;
(*_2).0 = [(-17_i8),122_i8,24_i8];
Goto(bb1)
}
bb1 = {
(*_2).0 = [90_i8,102_i8,87_i8];
(*_2).2 = (-30531800584930413637199245924509140044_i128) << _1;
(*_2).0 = [(-101_i8),24_i8,88_i8];
_1 = 3997579728380994375_u64;
_1 = 6862548452759719212_u64 * 10190193157670413552_u64;
RET = (-111_i8) as f64;
(*_2).2 = 40350103937491929978890508407111082758_i128 ^ (-138210605768943047511923437432245055429_i128);
RET = _3 as f64;
(*_2).0 = [(-20_i8),69_i8,(-33_i8)];
_4 = 3075498866_u32 / 2674127108_u32;
(*_2).2 = _1 as i128;
_4 = 3731271790_u32 % 210766820_u32;
_6.fld2.fld1 = ((*_2).1,);
_6.fld1 = 308022051950887755735454497144654962483_u128;
(*_2).0 = [90_i8,65_i8,(-16_i8)];
_6.fld0 = ['\u{4834d}','\u{1071ab}'];
_4 = !3927308592_u32;
(*_2).1 = _6.fld2.fld1.0;
(*_2).0 = [115_i8,6_i8,(-60_i8)];
_6.fld2.fld0.1 = [(*_2).1,(*_2).1,_6.fld2.fld1.0,(*_2).1,(*_2).1,_6.fld2.fld1.0,_6.fld2.fld1.0];
_2 = core::ptr::addr_of_mut!((*_2));
_1 = 13932690222669023403_u64;
Goto(bb2)
}
bb2 = {
RET = (-5058598189149413979_i64) as f64;
_6.fld0 = ['\u{d59a9}','\u{118be}'];
(*_2).2 = 17454860032269084614_usize as i128;
_7 = '\u{f12be}';
(*_2).2 = !(-101270724494977499185191815684060337672_i128);
_5 = (-2267963133435515525_i64) & 1193713508140685211_i64;
_8.0 = _1;
RET = (*_2).2 as f64;
_6.fld2.fld2 = !_4;
_9.1 = _7 as i8;
_5 = 5817400637475062151_i64 + (-4732585646890907839_i64);
_6.fld2.fld1.0 = !(*_2).1;
RET = _3 as f64;
(*_2).2 = _4 as i128;
RET = _3 as f64;
(*_2).1 = _6.fld2.fld1.0 & _6.fld2.fld1.0;
_6.fld2.fld0.1 = [(*_2).1,(*_2).1,(*_2).1,(*_2).1,(*_2).1,(*_2).1,(*_2).1];
_8.1 = core::ptr::addr_of!(_10.1);
Goto(bb3)
}
bb3 = {
_6.fld1 = 104709893651168146848219996180144547180_u128 & 100239210499971460552004688193029465139_u128;
Call(_8.3 = core::intrinsics::arith_offset(_8.1, 9223372036854775807_isize), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10.0 = (-22190_i16) >> _6.fld1;
_8.2 = [_9.1,_9.1,_9.1];
_10.1 = core::ptr::addr_of_mut!((*_2).2);
(*_2) = (_8.2, _6.fld2.fld1.0, (-20511173915320375735990826265742902798_i128));
_6.fld0 = [_7,_7];
(*_2).0 = _8.2;
(*_2) = (_8.2, _6.fld2.fld1.0, (-169865779132061570365423491766328613648_i128));
_10.2 = [_9.1,_9.1,_9.1];
(*_2).2 = 84760018175012622698775227924756146652_i128;
_14.fld2.fld1 = (_6.fld2.fld1.0,);
(*_2).0 = _8.2;
_13 = (*_2).2 as f32;
_14.fld2.fld0.1 = [_6.fld2.fld1.0,_14.fld2.fld1.0,_14.fld2.fld1.0,_6.fld2.fld1.0,(*_2).1,_14.fld2.fld1.0,(*_2).1];
_14.fld0 = [_7,_7];
Call(_8.2 = core::intrinsics::transmute((*_2).0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14.fld2.fld1 = ((*_2).1,);
(*_2).2 = !(-94488599458785047464479737562768539528_i128);
_1 = !_8.0;
_12 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
(*_2).0 = _8.2;
_9.2 = _5 - _5;
_13 = _6.fld1 as f32;
_12 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
_9.3 = !8424920089804191062_usize;
_8.1 = core::ptr::addr_of!(_10.1);
(*_2).0 = [_9.1,_9.1,_9.1];
(*_2).2 = _9.3 as i128;
_10.1 = core::ptr::addr_of_mut!((*_2).2);
_8.0 = _1;
_16 = !_3;
_16 = !_3;
(*_2) = (_8.2, _14.fld2.fld1.0, 12413815406206076659835050973811365630_i128);
_9.0 = core::ptr::addr_of_mut!(_10);
_15 = _14.fld2.fld1.0 != (*_2).1;
match (*_2).2 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
12413815406206076659835050973811365630 => bb14,
_ => bb13
}
}
bb6 = {
_10.0 = (-22190_i16) >> _6.fld1;
_8.2 = [_9.1,_9.1,_9.1];
_10.1 = core::ptr::addr_of_mut!((*_2).2);
(*_2) = (_8.2, _6.fld2.fld1.0, (-20511173915320375735990826265742902798_i128));
_6.fld0 = [_7,_7];
(*_2).0 = _8.2;
(*_2) = (_8.2, _6.fld2.fld1.0, (-169865779132061570365423491766328613648_i128));
_10.2 = [_9.1,_9.1,_9.1];
(*_2).2 = 84760018175012622698775227924756146652_i128;
_14.fld2.fld1 = (_6.fld2.fld1.0,);
(*_2).0 = _8.2;
_13 = (*_2).2 as f32;
_14.fld2.fld0.1 = [_6.fld2.fld1.0,_14.fld2.fld1.0,_14.fld2.fld1.0,_6.fld2.fld1.0,(*_2).1,_14.fld2.fld1.0,(*_2).1];
_14.fld0 = [_7,_7];
Call(_8.2 = core::intrinsics::transmute((*_2).0), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_6.fld1 = 104709893651168146848219996180144547180_u128 & 100239210499971460552004688193029465139_u128;
Call(_8.3 = core::intrinsics::arith_offset(_8.1, 9223372036854775807_isize), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
RET = (-5058598189149413979_i64) as f64;
_6.fld0 = ['\u{d59a9}','\u{118be}'];
(*_2).2 = 17454860032269084614_usize as i128;
_7 = '\u{f12be}';
(*_2).2 = !(-101270724494977499185191815684060337672_i128);
_5 = (-2267963133435515525_i64) & 1193713508140685211_i64;
_8.0 = _1;
RET = (*_2).2 as f64;
_6.fld2.fld2 = !_4;
_9.1 = _7 as i8;
_5 = 5817400637475062151_i64 + (-4732585646890907839_i64);
_6.fld2.fld1.0 = !(*_2).1;
RET = _3 as f64;
(*_2).2 = _4 as i128;
RET = _3 as f64;
(*_2).1 = _6.fld2.fld1.0 & _6.fld2.fld1.0;
_6.fld2.fld0.1 = [(*_2).1,(*_2).1,(*_2).1,(*_2).1,(*_2).1,(*_2).1,(*_2).1];
_8.1 = core::ptr::addr_of!(_10.1);
Goto(bb3)
}
bb9 = {
(*_2).0 = [90_i8,102_i8,87_i8];
(*_2).2 = (-30531800584930413637199245924509140044_i128) << _1;
(*_2).0 = [(-101_i8),24_i8,88_i8];
_1 = 3997579728380994375_u64;
_1 = 6862548452759719212_u64 * 10190193157670413552_u64;
RET = (-111_i8) as f64;
(*_2).2 = 40350103937491929978890508407111082758_i128 ^ (-138210605768943047511923437432245055429_i128);
RET = _3 as f64;
(*_2).0 = [(-20_i8),69_i8,(-33_i8)];
_4 = 3075498866_u32 / 2674127108_u32;
(*_2).2 = _1 as i128;
_4 = 3731271790_u32 % 210766820_u32;
_6.fld2.fld1 = ((*_2).1,);
_6.fld1 = 308022051950887755735454497144654962483_u128;
(*_2).0 = [90_i8,65_i8,(-16_i8)];
_6.fld0 = ['\u{4834d}','\u{1071ab}'];
_4 = !3927308592_u32;
(*_2).1 = _6.fld2.fld1.0;
(*_2).0 = [115_i8,6_i8,(-60_i8)];
_6.fld2.fld0.1 = [(*_2).1,(*_2).1,_6.fld2.fld1.0,(*_2).1,(*_2).1,_6.fld2.fld1.0,_6.fld2.fld1.0];
_2 = core::ptr::addr_of_mut!((*_2));
_1 = 13932690222669023403_u64;
Goto(bb2)
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
_19 = (_9.0, _14.fld2.fld1.0);
_6.fld2.fld2 = _4;
_21.fld0 = (_3, _6.fld2.fld1, _10.1, _9.1, _9.1, _10.1, _10.0);
_9.1 = _13 as i8;
_21.fld1.fld1.2 = -(*_2).2;
_21.fld0.3 = _9.3 as i8;
_6.fld2.fld0.0 = core::ptr::addr_of_mut!(_21.fld0);
_14.fld2.fld0.0 = _6.fld2.fld0.0;
(*_2) = (_8.2, _19.1, _21.fld1.fld1.2);
_4 = _6.fld2.fld2 | _6.fld2.fld2;
_14 = Adt62 { fld0: _6.fld0,fld1: _6.fld1,fld2: _6.fld2 };
_10 = (_21.fld0.6, _21.fld0.2, (*_2).0);
_21.fld3 = !_9.2;
_21.fld0.1.0 = _16 as i32;
_5 = (*_2).2 as i64;
_18 = 154_u8 * 181_u8;
(*_2).1 = _14.fld2.fld1.0;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(18_usize, 15_usize, Move(_15), 16_usize, Move(_16), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i128,mut _2: [u32; 1],mut _3: i128,mut _4: i16,mut _5: i32,mut _6: [u32; 1],mut _7: *mut i128,mut _8: *mut i128,mut _9: char,mut _10: i64) -> f64 {
mir! {
type RET = f64;
let _11: [i128; 3];
let _12: char;
let _13: f64;
let _14: usize;
let _15: f64;
let _16: i16;
let _17: Adt63;
let _18: f64;
let _19: isize;
let _20: f32;
let _21: f32;
let _22: isize;
let _23: f32;
let _24: *mut f64;
let _25: char;
let _26: (isize, (i32,), *mut i128, i8, i8, *mut i128, i16);
let _27: *const *mut i128;
let _28: ([char; 2], [u64; 8], bool);
let _29: isize;
let _30: [u8; 4];
let _31: u16;
let _32: isize;
let _33: ([i8; 3], i32, i128);
let _34: isize;
let _35: Adt63;
let _36: char;
let _37: *mut bool;
let _38: char;
let _39: ();
let _40: ();
{
_5 = 82_u8 as i32;
_5 = 99252108348923090430928711392023280683_u128 as i32;
_11 = [(*_7),_3,_3];
_3 = !(*_7);
_1 = _3;
(*_8) = 16161_u16 as i128;
_1 = (*_8) << (*_7);
_4 = 79_u8 as i16;
RET = 328900963229671944597097623655278187542_u128 as f64;
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = false as i128;
_11 = [_3,(*_7),(*_8)];
_12 = _9;
_13 = _1 as f64;
(*_8) = -_3;
_6 = _2;
_11 = [(*_7),(*_8),(*_8)];
RET = -_13;
(*_8) = _3;
Call((*_8) = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = (-5085729941206807674_i64);
_10 = -8279055042718493090_i64;
_4 = 14052_i16 | (-2684_i16);
_5 = _4 as i32;
RET = 9223372036854775807_isize as f64;
_8 = core::ptr::addr_of_mut!((*_8));
Call((*_7) = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = (*_8) as f64;
(*_7) = _3 - _3;
_8 = core::ptr::addr_of_mut!(_3);
_2 = [841288866_u32];
RET = _13;
_13 = 240239178823589308864101357037715090423_u128 as f64;
RET = _13;
_17.fld7.fld2.fld0 = _10 * _10;
_14 = !16566170353228843615_usize;
_13 = 628080399013777251_u64 as f64;
_17.fld7.fld0.0 = [(-51_i8),77_i8,85_i8];
RET = 30962_u16 as f64;
RET = _13;
_10 = !_17.fld7.fld2.fld0;
_17.fld7.fld1 = [3265041431_u32];
Goto(bb3)
}
bb3 = {
_17.fld7.fld0.0 = [(-112_i8),24_i8,(-91_i8)];
_19 = 9223372036854775807_isize * (-9223372036854775808_isize);
_16 = _4 + _4;
_11 = [(*_7),(*_7),_1];
_8 = _7;
_5 = 676110733_i32 & 254791668_i32;
_17.fld3 = (*_8) as i8;
_11 = [(*_7),_3,_3];
RET = _13;
_17.fld7.fld4.fld1.2 = [_17.fld3,_17.fld3,_17.fld3];
_20 = _13 as f32;
_17.fld4 = [3007279123_u32];
_17.fld7.fld2.fld2 = core::ptr::addr_of!(_8);
_10 = !_17.fld7.fld2.fld0;
_17.fld7.fld4.fld0 = [2122390479_u32];
_17.fld2 = _12 as isize;
_17.fld7.fld0.1 = _5 ^ _5;
_17.fld7.fld3.0 = core::ptr::addr_of_mut!(_17.fld7.fld2.fld0);
_5 = _14 as i32;
_22 = !_19;
_17.fld1 = core::ptr::addr_of_mut!(_18);
_23 = -_20;
_18 = _13 / f64::NEG_INFINITY;
Call(_17.fld3 = core::intrinsics::bswap((-54_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = !_16;
_17.fld7.fld4.fld1.2 = [_17.fld3,_17.fld3,_17.fld3];
_21 = _20 - _20;
_19 = 1266862411234558648_u64 as isize;
_8 = _7;
_23 = _20 / (-0.000000000000000000000000000000000000001321676685581881_f32);
_6 = [3246137301_u32];
_17.fld7.fld0.1 = 8372826076039759573_u64 as i32;
_17.fld7.fld3.0 = core::ptr::addr_of_mut!(_17.fld7.fld2.fld0);
_17.fld7.fld2.fld1 = _12;
_14 = _10 as usize;
_17.fld7.fld4.fld1 = (6783185217887332973_u64, _17.fld7.fld2.fld2, _17.fld7.fld0.0, _17.fld7.fld2.fld2);
_17.fld7.fld4.fld0 = [2620328945_u32];
_17.fld7.fld4.fld2 = core::ptr::addr_of_mut!(_26);
_17.fld7.fld4.fld2 = core::ptr::addr_of_mut!(_26);
_28.0 = [_12,_9];
_2 = [2579377416_u32];
_28.1 = [_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0];
_27 = core::ptr::addr_of!(_26.5);
_25 = _12;
_9 = _25;
_26.5 = core::ptr::addr_of_mut!(_17.fld7.fld0.2);
_26.3 = _17.fld3;
_8 = core::ptr::addr_of_mut!((*_8));
_21 = _23 * _23;
match _17.fld7.fld4.fld1.0 {
0 => bb1,
1 => bb2,
6783185217887332973 => bb5,
_ => bb3
}
}
bb5 = {
_26.5 = core::ptr::addr_of_mut!(_3);
_18 = 231861151376864003092977350226876875908_u128 as f64;
match _17.fld7.fld4.fld1.0 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
6783185217887332973 => bb10,
_ => bb9
}
}
bb6 = {
_4 = !_16;
_17.fld7.fld4.fld1.2 = [_17.fld3,_17.fld3,_17.fld3];
_21 = _20 - _20;
_19 = 1266862411234558648_u64 as isize;
_8 = _7;
_23 = _20 / (-0.000000000000000000000000000000000000001321676685581881_f32);
_6 = [3246137301_u32];
_17.fld7.fld0.1 = 8372826076039759573_u64 as i32;
_17.fld7.fld3.0 = core::ptr::addr_of_mut!(_17.fld7.fld2.fld0);
_17.fld7.fld2.fld1 = _12;
_14 = _10 as usize;
_17.fld7.fld4.fld1 = (6783185217887332973_u64, _17.fld7.fld2.fld2, _17.fld7.fld0.0, _17.fld7.fld2.fld2);
_17.fld7.fld4.fld0 = [2620328945_u32];
_17.fld7.fld4.fld2 = core::ptr::addr_of_mut!(_26);
_17.fld7.fld4.fld2 = core::ptr::addr_of_mut!(_26);
_28.0 = [_12,_9];
_2 = [2579377416_u32];
_28.1 = [_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0,_17.fld7.fld4.fld1.0];
_27 = core::ptr::addr_of!(_26.5);
_25 = _12;
_9 = _25;
_26.5 = core::ptr::addr_of_mut!(_17.fld7.fld0.2);
_26.3 = _17.fld3;
_8 = core::ptr::addr_of_mut!((*_8));
_21 = _23 * _23;
match _17.fld7.fld4.fld1.0 {
0 => bb1,
1 => bb2,
6783185217887332973 => bb5,
_ => bb3
}
}
bb7 = {
_17.fld7.fld0.0 = [(-112_i8),24_i8,(-91_i8)];
_19 = 9223372036854775807_isize * (-9223372036854775808_isize);
_16 = _4 + _4;
_11 = [(*_7),(*_7),_1];
_8 = _7;
_5 = 676110733_i32 & 254791668_i32;
_17.fld3 = (*_8) as i8;
_11 = [(*_7),_3,_3];
RET = _13;
_17.fld7.fld4.fld1.2 = [_17.fld3,_17.fld3,_17.fld3];
_20 = _13 as f32;
_17.fld4 = [3007279123_u32];
_17.fld7.fld2.fld2 = core::ptr::addr_of!(_8);
_10 = !_17.fld7.fld2.fld0;
_17.fld7.fld4.fld0 = [2122390479_u32];
_17.fld2 = _12 as isize;
_17.fld7.fld0.1 = _5 ^ _5;
_17.fld7.fld3.0 = core::ptr::addr_of_mut!(_17.fld7.fld2.fld0);
_5 = _14 as i32;
_22 = !_19;
_17.fld1 = core::ptr::addr_of_mut!(_18);
_23 = -_20;
_18 = _13 / f64::NEG_INFINITY;
Call(_17.fld3 = core::intrinsics::bswap((-54_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_13 = (*_8) as f64;
(*_7) = _3 - _3;
_8 = core::ptr::addr_of_mut!(_3);
_2 = [841288866_u32];
RET = _13;
_13 = 240239178823589308864101357037715090423_u128 as f64;
RET = _13;
_17.fld7.fld2.fld0 = _10 * _10;
_14 = !16566170353228843615_usize;
_13 = 628080399013777251_u64 as f64;
_17.fld7.fld0.0 = [(-51_i8),77_i8,85_i8];
RET = 30962_u16 as f64;
RET = _13;
_10 = !_17.fld7.fld2.fld0;
_17.fld7.fld1 = [3265041431_u32];
Goto(bb3)
}
bb9 = {
_10 = (-5085729941206807674_i64);
_10 = -8279055042718493090_i64;
_4 = 14052_i16 | (-2684_i16);
_5 = _4 as i32;
RET = 9223372036854775807_isize as f64;
_8 = core::ptr::addr_of_mut!((*_8));
Call((*_7) = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_26.5 = _8;
RET = -_18;
_10 = 7_u8 as i64;
(*_8) = !_1;
(*_8) = _3 + _3;
_23 = _21 - _21;
_26.1 = (_17.fld7.fld0.1,);
_13 = _18 - _18;
_26.5 = _8;
_17.fld7.fld0.2 = _14 as i128;
_17.fld7.fld2.fld0 = !_10;
_17.fld5 = 4107940340_u32 as i32;
_3 = (*_7);
_24 = core::ptr::addr_of_mut!(_15);
_26.4 = _26.3 & _26.3;
_31 = 18569_u16;
_3 = 122_u8 as i128;
_13 = _31 as f64;
_32 = !_22;
_28.2 = !false;
_4 = _16 * _16;
match _17.fld7.fld4.fld1.0 {
6783185217887332973 => bb12,
_ => bb11
}
}
bb11 = {
_17.fld7.fld0.0 = [(-112_i8),24_i8,(-91_i8)];
_19 = 9223372036854775807_isize * (-9223372036854775808_isize);
_16 = _4 + _4;
_11 = [(*_7),(*_7),_1];
_8 = _7;
_5 = 676110733_i32 & 254791668_i32;
_17.fld3 = (*_8) as i8;
_11 = [(*_7),_3,_3];
RET = _13;
_17.fld7.fld4.fld1.2 = [_17.fld3,_17.fld3,_17.fld3];
_20 = _13 as f32;
_17.fld4 = [3007279123_u32];
_17.fld7.fld2.fld2 = core::ptr::addr_of!(_8);
_10 = !_17.fld7.fld2.fld0;
_17.fld7.fld4.fld0 = [2122390479_u32];
_17.fld2 = _12 as isize;
_17.fld7.fld0.1 = _5 ^ _5;
_17.fld7.fld3.0 = core::ptr::addr_of_mut!(_17.fld7.fld2.fld0);
_5 = _14 as i32;
_22 = !_19;
_17.fld1 = core::ptr::addr_of_mut!(_18);
_23 = -_20;
_18 = _13 / f64::NEG_INFINITY;
Call(_17.fld3 = core::intrinsics::bswap((-54_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_17.fld0 = _32 as u128;
(*_27) = _7;
_13 = -_18;
_15 = _13;
_2 = [816391985_u32];
_12 = _9;
_26.1.0 = _17.fld0 as i32;
_17.fld7.fld4.fld1 = (10437920974817771947_u64, _17.fld7.fld2.fld2, _17.fld7.fld0.0, _27);
_21 = _17.fld7.fld2.fld0 as f32;
_17.fld7.fld4.fld1 = (3654245249411538918_u64, _17.fld7.fld2.fld2, _17.fld7.fld0.0, _27);
_17.fld7.fld4.fld2 = core::ptr::addr_of_mut!(_26);
Goto(bb13)
}
bb13 = {
_17.fld5 = _26.4 as i32;
_35.fld7.fld4.fld1 = (_17.fld7.fld4.fld1.0, _17.fld7.fld4.fld1.3, _17.fld7.fld4.fld1.2, _17.fld7.fld4.fld1.3);
_17.fld7.fld0 = (_17.fld7.fld4.fld1.2, _5, _1);
_35.fld7 = Adt61 { fld0: _17.fld7.fld0,fld1: _17.fld7.fld1,fld2: Move(_17.fld7.fld2),fld3: _17.fld7.fld3,fld4: _17.fld7.fld4 };
(*_8) = -_1;
_17.fld7.fld4.fld2 = _35.fld7.fld4.fld2;
_17.fld7.fld0.2 = _35.fld7.fld0.2;
(*_24) = _18;
_15 = _13 * _18;
_17.fld7.fld4.fld1.1 = _35.fld7.fld4.fld1.3;
_35.fld0 = _17.fld0 % 1417633923902438058526896062823090903_u128;
_17.fld7.fld0.1 = -_5;
(*_7) = -_3;
_35.fld7.fld2.fld1 = _9;
_29 = _32 >> _17.fld5;
_26.1 = (_17.fld5,);
match _17.fld7.fld4.fld1.0 {
0 => bb6,
1 => bb2,
2 => bb14,
3 => bb15,
3654245249411538918 => bb17,
_ => bb16
}
}
bb14 = {
_10 = (-5085729941206807674_i64);
_10 = -8279055042718493090_i64;
_4 = 14052_i16 | (-2684_i16);
_5 = _4 as i32;
RET = 9223372036854775807_isize as f64;
_8 = core::ptr::addr_of_mut!((*_8));
Call((*_7) = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_13 = (*_8) as f64;
(*_7) = _3 - _3;
_8 = core::ptr::addr_of_mut!(_3);
_2 = [841288866_u32];
RET = _13;
_13 = 240239178823589308864101357037715090423_u128 as f64;
RET = _13;
_17.fld7.fld2.fld0 = _10 * _10;
_14 = !16566170353228843615_usize;
_13 = 628080399013777251_u64 as f64;
_17.fld7.fld0.0 = [(-51_i8),77_i8,85_i8];
RET = 30962_u16 as f64;
RET = _13;
_10 = !_17.fld7.fld2.fld0;
_17.fld7.fld1 = [3265041431_u32];
Goto(bb3)
}
bb16 = {
_17.fld7.fld0.0 = [(-112_i8),24_i8,(-91_i8)];
_19 = 9223372036854775807_isize * (-9223372036854775808_isize);
_16 = _4 + _4;
_11 = [(*_7),(*_7),_1];
_8 = _7;
_5 = 676110733_i32 & 254791668_i32;
_17.fld3 = (*_8) as i8;
_11 = [(*_7),_3,_3];
RET = _13;
_17.fld7.fld4.fld1.2 = [_17.fld3,_17.fld3,_17.fld3];
_20 = _13 as f32;
_17.fld4 = [3007279123_u32];
_17.fld7.fld2.fld2 = core::ptr::addr_of!(_8);
_10 = !_17.fld7.fld2.fld0;
_17.fld7.fld4.fld0 = [2122390479_u32];
_17.fld2 = _12 as isize;
_17.fld7.fld0.1 = _5 ^ _5;
_17.fld7.fld3.0 = core::ptr::addr_of_mut!(_17.fld7.fld2.fld0);
_5 = _14 as i32;
_22 = !_19;
_17.fld1 = core::ptr::addr_of_mut!(_18);
_23 = -_20;
_18 = _13 / f64::NEG_INFINITY;
Call(_17.fld3 = core::intrinsics::bswap((-54_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb17 = {
_26.0 = _29 >> _26.1.0;
_7 = core::ptr::addr_of_mut!((*_7));
_35.fld4 = [1954994984_u32];
_38 = _35.fld7.fld2.fld1;
_17.fld7 = Adt61 { fld0: _35.fld7.fld0,fld1: _2,fld2: Move(_35.fld7.fld2),fld3: _35.fld7.fld3,fld4: _35.fld7.fld4 };
_35.fld2 = _4 as isize;
_35.fld7.fld0 = (_17.fld7.fld0.0, _26.1.0, (*_8));
_17.fld2 = -_22;
(*_27) = _8;
_35.fld6 = !3717203579_u32;
_3 = (*_8);
(*_24) = _13 + _13;
_28.0 = [_9,_38];
Goto(bb18)
}
bb18 = {
Call(_39 = dump_var(19_usize, 6_usize, Move(_6), 29_usize, Move(_29), 12_usize, Move(_12), 32_usize, Move(_32)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_39 = dump_var(19_usize, 10_usize, Move(_10), 31_usize, Move(_31), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_39 = dump_var(19_usize, 28_usize, Move(_28), 25_usize, Move(_25), 40_usize, _40, 40_usize, _40), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
pub fn main() {
                println!("{:?}", fn0(std::hint::black_box(57511688100190139123849779091259821589_u128), std::hint::black_box(5354262818648133520_u64), std::hint::black_box(30_isize), std::hint::black_box(19_i8), std::hint::black_box((-32734_i16))));
                
            }
#[derive(Debug,Copy,Clone)]
pub struct Adt49 {
fld0: f32,
fld1: ([i8; 3], i32, i128),
}
#[derive(Debug)]
pub struct Adt50 {
fld0: i64,
fld1: char,
fld2: *const *mut i128,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt51 {
fld0: [u32; 1],
fld1: (u64, *const *mut i128, [i8; 3], *const *mut i128),
fld2: *mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt52 {
fld0: (i16, *mut i128, [i8; 3]),
fld1: [i8; 3],
fld2: *mut ([i8; 3], i32, i128),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt53 {
fld0: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]),
fld1: (i32,),
fld2: u32,
}
#[derive(Debug)]
pub struct Adt54 {
fld0: *mut (i16, *mut i128, [i8; 3]),
fld1: [u8; 4],
fld2: Adt50,
fld3: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]),
fld4: i16,
fld5: u128,
fld6: Adt51,
fld7: [i32; 7],
}
#[derive(Debug)]
pub struct Adt55 {
fld0: Adt54,
fld1: Adt53,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt56 {
fld0: (isize, (i32,), *mut i128, i8, i8, *mut i128, i16),
fld1: Adt49,
fld2: *const f32,
fld3: i64,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt57 {
fld0: [i8; 3],
fld1: *const u8,
fld2: isize,
fld3: *mut (i16, *mut i128, [i8; 3]),
fld4: ([i8; 3], i32, i128),
fld5: *mut u64,
fld6: f32,
fld7: i128,
}
#[derive(Debug)]
pub struct Adt58 {
fld0: (char,),
fld1: i16,
fld2: (*mut (isize, (i32,), *mut i128, i8, i8, *mut i128, i16), [i32; 7]),
fld3: *const *mut i128,
}
#[derive(Debug)]
pub struct Adt59 {
fld0: (i16, *mut i128, [i8; 3]),
fld1: *mut i64,
fld2: isize,
fld3: [i16; 6],
fld4: Adt58,
fld5: *const u8,
fld6: u128,
fld7: (isize, (i32,), *mut i128, i8, i8, *mut i128, i16),
}
#[derive(Debug)]
pub struct Adt60 {
fld0: *const u8,
fld1: Adt56,
fld2: [i32; 7],
fld3: Adt52,
}
#[derive(Debug)]
pub struct Adt61 {
fld0: ([i8; 3], i32, i128),
fld1: [u32; 1],
fld2: Adt50,
fld3: (*mut i64,),
fld4: Adt51,
}
#[derive(Debug)]
pub struct Adt62 {
fld0: [char; 2],
fld1: u128,
fld2: Adt53,
}
#[derive(Debug)]
pub struct Adt63 {
fld0: u128,
fld1: *mut f64,
fld2: isize,
fld3: i8,
fld4: [u32; 1],
fld5: i32,
fld6: u32,
fld7: Adt61,
}
#[derive(Debug)]
pub struct Adt64 {
fld0: f32,
fld1: (*mut (i16, *mut i128, [i8; 3]), i8, i64, usize),
fld2: Adt51,
fld3: Adt59,
}
#[derive(Debug)]
pub struct Adt65 {
fld0: f32,
fld1: [i8; 3],
fld2: i128,
fld3: [char; 2],
fld4: ([char; 2], [u64; 8], bool),
fld5: Adt60,
}

