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
pub fn fn0(mut _1: u16,mut _2: char,mut _3: i16,mut _4: u8) -> i16 {
mir! {
type RET = i16;
let _5: Adt55;
let _6: isize;
let _7: i8;
let _8: Adt63;
let _9: *mut u128;
let _10: isize;
let _11: [bool; 5];
let _12: Adt57;
let _13: Adt52;
let _14: Adt58;
let _15: [bool; 5];
let _16: f32;
let _17: Adt48;
let _18: bool;
let _19: [u32; 6];
let _20: [i16; 6];
let _21: Adt63;
let _22: ();
let _23: ();
{
_2 = '\u{b1001}';
_3 = -31709_i16;
Goto(bb1)
}
bb1 = {
RET = _3;
_1 = 47059_u16 >> _3;
_3 = (-30467_i16);
_4 = (-85121462925609247596417480693389981709_i128) as u8;
_5.fld1 = core::ptr::addr_of_mut!(_5.fld3);
_5.fld7.fld1 = (11104699463038947251_u64,);
_5.fld5.5 = core::ptr::addr_of_mut!(_5.fld7.fld1.0);
_5.fld6 = [false,true,false,false,false];
_5.fld4 = [2892283944_u32,1951872294_u32,3350451019_u32,3602887543_u32,1480602933_u32,2452447873_u32];
_5.fld3.3 = 212279121201063293229895842618246808396_u128 as u32;
_5.fld4 = [_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3];
_5.fld7.fld0 = core::ptr::addr_of_mut!(_5.fld7.fld1.0);
_5.fld5.3.0 = 9223372036854775807_isize;
_1 = 53993_u16;
_5.fld5.4 = _5.fld5.3.0 ^ _5.fld5.3.0;
_5.fld0 = _4;
_5.fld7.fld1 = (14952704496513913212_u64,);
Goto(bb2)
}
bb2 = {
_8.fld2.fld4 = (_5.fld7.fld1.0,);
_8.fld1.0 = (94_i8,);
_3 = _2 as i16;
_5.fld1 = core::ptr::addr_of_mut!(_5.fld3);
_8.fld2.fld1 = (_8.fld1.0.0,);
_8.fld1.0 = (_8.fld2.fld1.0,);
_5.fld3.0 = core::ptr::addr_of!(_8.fld2.fld4.0);
_3 = -(-20651_i16);
_5.fld5.4 = -_5.fld5.3.0;
RET = _3;
_8.fld1.0 = _8.fld2.fld1;
_6 = -_5.fld5.3.0;
Goto(bb3)
}
bb3 = {
_5.fld4 = [_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3];
_12.fld2.0 = [0_usize];
RET = _3 >> _8.fld2.fld4.0;
_7 = _8.fld1.0.0;
_12.fld2.2 = 4486711916312790014_i64 >> _8.fld1.0.0;
_5.fld3.1 = [false,false];
_8.fld2.fld2 = [false,false,true,true,false];
_5.fld3.0 = core::ptr::addr_of!(_12.fld1);
_5.fld5.0 = [false,true];
_8.fld1.0.0 = _7;
_5.fld2 = [_12.fld2.2,_12.fld2.2,_12.fld2.2,_12.fld2.2];
_8.fld1 = (_8.fld2.fld1,);
_6 = _5.fld5.3.0;
Call(_12.fld1 = fn1(_8.fld1.0, _8.fld2.fld1.0, _5.fld7.fld0, _2, _5.fld3.1, _1, _5.fld5.3.0, _5.fld7.fld1.0, _5.fld1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5.fld5.2 = _4 as i128;
_8.fld2 = Adt53 { fld0: _1,fld1: _8.fld1.0,fld2: _5.fld6,fld3: _5.fld3.2,fld4: _5.fld7.fld1 };
_12.fld2.1 = _5.fld1;
_8.fld2.fld0 = _5.fld7.fld1.0 as u16;
_12.fld0 = _5.fld3.3 as f32;
_8.fld2.fld2 = [true,false,true,true,false];
_11 = [true,false,false,false,false];
_8.fld2.fld3 = core::ptr::addr_of!(_12.fld2.2);
_8.fld2.fld1.0 = _8.fld1.0.0;
_12.fld1 = _5.fld7.fld1.0 << _8.fld1.0.0;
_5.fld5.4 = _6;
_10 = _5.fld0 as isize;
_5.fld0 = !_4;
_8.fld2.fld0 = _12.fld1 as u16;
RET = _8.fld2.fld1.0 as i16;
RET = -_3;
_5.fld7.fld0 = core::ptr::addr_of_mut!(_8.fld2.fld4.0);
_5.fld5.3.2 = _12.fld0;
_5.fld2 = [_12.fld2.2,_12.fld2.2,_12.fld2.2,_12.fld2.2];
Goto(bb5)
}
bb5 = {
_5.fld4 = [_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3];
_8.fld2.fld0 = _1;
_4 = _5.fld3.3 as u8;
_5.fld7.fld1 = (_12.fld1,);
_4 = _5.fld0 + _5.fld0;
_8.fld1.0.0 = _8.fld2.fld1.0;
_8.fld2.fld1.0 = _8.fld1.0.0;
_8.fld1.0 = _8.fld2.fld1;
_8.fld2.fld4 = (_12.fld1,);
_5.fld3.0 = core::ptr::addr_of!(_5.fld7.fld1.0);
_5.fld5.5 = core::ptr::addr_of_mut!(_12.fld1);
_5.fld3.3 = !2153071575_u32;
_8.fld2.fld4 = (_12.fld1,);
Call(_5.fld5.3.2 = fn2(_8.fld2.fld0, _12.fld1, _8.fld2, _5.fld5.2, _8.fld2.fld1, _5.fld5.3.0, _5.fld3.0, _5.fld5.5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_5.fld3.1 = _5.fld5.0;
_5.fld5.5 = core::ptr::addr_of_mut!(_12.fld1);
_5.fld3.3 = _8.fld2.fld4.0 as u32;
_6 = _2 as isize;
_3 = (-21999_i16);
_4 = _5.fld0 - _5.fld0;
match _8.fld2.fld0 {
53993 => bb8,
_ => bb7
}
}
bb7 = {
_8.fld2.fld4 = (_5.fld7.fld1.0,);
_8.fld1.0 = (94_i8,);
_3 = _2 as i16;
_5.fld1 = core::ptr::addr_of_mut!(_5.fld3);
_8.fld2.fld1 = (_8.fld1.0.0,);
_8.fld1.0 = (_8.fld2.fld1.0,);
_5.fld3.0 = core::ptr::addr_of!(_8.fld2.fld4.0);
_3 = -(-20651_i16);
_5.fld5.4 = -_5.fld5.3.0;
RET = _3;
_8.fld1.0 = _8.fld2.fld1;
_6 = -_5.fld5.3.0;
Goto(bb3)
}
bb8 = {
_5.fld3.1 = _5.fld5.0;
_5.fld5.2 = (-31741941520350334671495032584150849468_i128);
_12.fld1 = _5.fld7.fld1.0 | _8.fld2.fld4.0;
_12.fld2.0 = [6_usize];
_5.fld5.0 = [false,false];
_5.fld7.fld1 = (_12.fld1,);
_6 = !_10;
_5.fld5.4 = _5.fld5.3.0 - _5.fld5.3.0;
_14.fld2.0 = _1 as i8;
_8.fld2.fld4.0 = !_5.fld7.fld1.0;
_8.fld2.fld1 = (_14.fld2.0,);
_14.fld0 = core::ptr::addr_of!(_5.fld3.0);
_4 = _5.fld0 % 104_u8;
RET = -_3;
RET = _3 >> _4;
RET = -_3;
_6 = 4_usize as isize;
_5.fld7.fld0 = core::ptr::addr_of_mut!(_5.fld7.fld1.0);
_14.fld1 = !_12.fld2.2;
_8.fld1.0 = _8.fld2.fld1;
_6 = !_5.fld5.4;
_5.fld5.0 = [true,true];
RET = _5.fld5.2 as i16;
_5.fld4 = [_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3];
_17.fld0 = _8.fld2.fld1;
match _7 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
94 => bb15,
_ => bb14
}
}
bb9 = {
_8.fld2.fld4 = (_5.fld7.fld1.0,);
_8.fld1.0 = (94_i8,);
_3 = _2 as i16;
_5.fld1 = core::ptr::addr_of_mut!(_5.fld3);
_8.fld2.fld1 = (_8.fld1.0.0,);
_8.fld1.0 = (_8.fld2.fld1.0,);
_5.fld3.0 = core::ptr::addr_of!(_8.fld2.fld4.0);
_3 = -(-20651_i16);
_5.fld5.4 = -_5.fld5.3.0;
RET = _3;
_8.fld1.0 = _8.fld2.fld1;
_6 = -_5.fld5.3.0;
Goto(bb3)
}
bb10 = {
_5.fld3.1 = _5.fld5.0;
_5.fld5.5 = core::ptr::addr_of_mut!(_12.fld1);
_5.fld3.3 = _8.fld2.fld4.0 as u32;
_6 = _2 as isize;
_3 = (-21999_i16);
_4 = _5.fld0 - _5.fld0;
match _8.fld2.fld0 {
53993 => bb8,
_ => bb7
}
}
bb11 = {
_5.fld4 = [_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3];
_8.fld2.fld0 = _1;
_4 = _5.fld3.3 as u8;
_5.fld7.fld1 = (_12.fld1,);
_4 = _5.fld0 + _5.fld0;
_8.fld1.0.0 = _8.fld2.fld1.0;
_8.fld2.fld1.0 = _8.fld1.0.0;
_8.fld1.0 = _8.fld2.fld1;
_8.fld2.fld4 = (_12.fld1,);
_5.fld3.0 = core::ptr::addr_of!(_5.fld7.fld1.0);
_5.fld5.5 = core::ptr::addr_of_mut!(_12.fld1);
_5.fld3.3 = !2153071575_u32;
_8.fld2.fld4 = (_12.fld1,);
Call(_5.fld5.3.2 = fn2(_8.fld2.fld0, _12.fld1, _8.fld2, _5.fld5.2, _8.fld2.fld1, _5.fld5.3.0, _5.fld3.0, _5.fld5.5), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_5.fld5.2 = _4 as i128;
_8.fld2 = Adt53 { fld0: _1,fld1: _8.fld1.0,fld2: _5.fld6,fld3: _5.fld3.2,fld4: _5.fld7.fld1 };
_12.fld2.1 = _5.fld1;
_8.fld2.fld0 = _5.fld7.fld1.0 as u16;
_12.fld0 = _5.fld3.3 as f32;
_8.fld2.fld2 = [true,false,true,true,false];
_11 = [true,false,false,false,false];
_8.fld2.fld3 = core::ptr::addr_of!(_12.fld2.2);
_8.fld2.fld1.0 = _8.fld1.0.0;
_12.fld1 = _5.fld7.fld1.0 << _8.fld1.0.0;
_5.fld5.4 = _6;
_10 = _5.fld0 as isize;
_5.fld0 = !_4;
_8.fld2.fld0 = _12.fld1 as u16;
RET = _8.fld2.fld1.0 as i16;
RET = -_3;
_5.fld7.fld0 = core::ptr::addr_of_mut!(_8.fld2.fld4.0);
_5.fld5.3.2 = _12.fld0;
_5.fld2 = [_12.fld2.2,_12.fld2.2,_12.fld2.2,_12.fld2.2];
Goto(bb5)
}
bb13 = {
_5.fld4 = [_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3];
_12.fld2.0 = [0_usize];
RET = _3 >> _8.fld2.fld4.0;
_7 = _8.fld1.0.0;
_12.fld2.2 = 4486711916312790014_i64 >> _8.fld1.0.0;
_5.fld3.1 = [false,false];
_8.fld2.fld2 = [false,false,true,true,false];
_5.fld3.0 = core::ptr::addr_of!(_12.fld1);
_5.fld5.0 = [false,true];
_8.fld1.0.0 = _7;
_5.fld2 = [_12.fld2.2,_12.fld2.2,_12.fld2.2,_12.fld2.2];
_8.fld1 = (_8.fld2.fld1,);
_6 = _5.fld5.3.0;
Call(_12.fld1 = fn1(_8.fld1.0, _8.fld2.fld1.0, _5.fld7.fld0, _2, _5.fld3.1, _1, _5.fld5.3.0, _5.fld7.fld1.0, _5.fld1), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
RET = _3;
_1 = 47059_u16 >> _3;
_3 = (-30467_i16);
_4 = (-85121462925609247596417480693389981709_i128) as u8;
_5.fld1 = core::ptr::addr_of_mut!(_5.fld3);
_5.fld7.fld1 = (11104699463038947251_u64,);
_5.fld5.5 = core::ptr::addr_of_mut!(_5.fld7.fld1.0);
_5.fld6 = [false,true,false,false,false];
_5.fld4 = [2892283944_u32,1951872294_u32,3350451019_u32,3602887543_u32,1480602933_u32,2452447873_u32];
_5.fld3.3 = 212279121201063293229895842618246808396_u128 as u32;
_5.fld4 = [_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3,_5.fld3.3];
_5.fld7.fld0 = core::ptr::addr_of_mut!(_5.fld7.fld1.0);
_5.fld5.3.0 = 9223372036854775807_isize;
_1 = 53993_u16;
_5.fld5.4 = _5.fld5.3.0 ^ _5.fld5.3.0;
_5.fld0 = _4;
_5.fld7.fld1 = (14952704496513913212_u64,);
Goto(bb2)
}
bb15 = {
_5.fld7.fld1 = (_8.fld2.fld4.0,);
_5.fld6 = [true,false,false,true,false];
_8.fld2.fld1.0 = _17.fld0.0 ^ _7;
_5.fld3.0 = core::ptr::addr_of!(_5.fld7.fld1.0);
_14.fld0 = core::ptr::addr_of!(_17.fld1.0);
_5.fld1 = core::ptr::addr_of_mut!(_5.fld3);
_4 = _5.fld5.2 as u8;
_18 = _12.fld1 <= _5.fld7.fld1.0;
_17.fld2 = core::ptr::addr_of!(_8.fld2.fld4.0);
_8.fld1.0 = (_14.fld2.0,);
_17.fld3 = [_18,_18,_18,_18,_18];
_5.fld3.0 = core::ptr::addr_of!(_5.fld7.fld1.0);
_4 = _5.fld0;
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(0_usize, 1_usize, Move(_1), 10_usize, Move(_10), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: (i8,),mut _2: i8,mut _3: *mut u64,mut _4: char,mut _5: [bool; 2],mut _6: u16,mut _7: isize,mut _8: u64,mut _9: *mut (*const u64, [bool; 2], *const i64, u32)) -> u64 {
mir! {
type RET = u64;
let _10: [bool; 5];
let _11: char;
let _12: Adt59;
let _13: isize;
let _14: u64;
let _15: ((i8,),);
let _16: f64;
let _17: u128;
let _18: u128;
let _19: f64;
let _20: *mut u64;
let _21: i128;
let _22: bool;
let _23: u8;
let _24: bool;
let _25: [i64; 4];
let _26: [char; 1];
let _27: (i8,);
let _28: isize;
let _29: isize;
let _30: [u32; 6];
let _31: isize;
let _32: ();
let _33: ();
{
_7 = _1.0 as isize;
(*_9).3 = 2908196298_u32;
_12.fld3.fld0.0 = _2 << _8;
RET = 60_u8 as u64;
_12.fld0.2 = 107479335808551105817276931091173691309_i128 as f32;
_12.fld3.fld1.0 = core::ptr::addr_of!(_12.fld4.1.0);
_3 = core::ptr::addr_of_mut!(_8);
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
94 => bb5,
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
RET = (-1381809459_i32) as u64;
(*_3) = 5287734455884371293_u64 / 5104841427491691535_u64;
_12.fld2.2 = (-24287090839690905256786638736823794327_i128) - (-81594899052396572350499908610437500515_i128);
_4 = '\u{107805}';
_12.fld2.4 = _7;
_12.fld2.5 = _3;
(*_9).2 = core::ptr::addr_of!(_12.fld1);
_12.fld0.0 = !_12.fld2.4;
(*_9).1 = _5;
(*_9).1 = [false,false];
_12.fld3.fld4 = _9;
_2 = !_1.0;
_8 = _12.fld0.2 as u64;
Goto(bb6)
}
bb6 = {
_12.fld3.fld4 = core::ptr::addr_of_mut!((*_9));
_10 = [false,true,false,true,false];
_1.0 = _12.fld3.fld0.0 & _12.fld3.fld0.0;
_12.fld4.0 = [_12.fld0.0,_7,_7,_7,_12.fld0.0,_12.fld2.4,_7,_12.fld0.0];
_13 = !_7;
_6 = 13054_u16;
(*_3) = _12.fld2.2 as u64;
(*_9).1 = [true,false];
_12.fld2.2 = (-7928999454509972773062608355197068732_i128);
_2 = _1.0;
_12.fld4.0 = [_12.fld0.0,_7,_7,_12.fld0.0,_12.fld2.4,_12.fld2.4,_12.fld0.0,_12.fld0.0];
_12.fld2.5 = _3;
_12.fld4.1 = (_8,);
_12.fld3.fld4 = core::ptr::addr_of_mut!((*_9));
_12.fld4.0 = [_13,_12.fld2.4,_12.fld0.0,_12.fld0.0,_12.fld2.4,_12.fld2.4,_12.fld2.4,_12.fld2.4];
_12.fld3.fld0 = (_1.0,);
RET = !_8;
_12.fld1 = _4 as i64;
(*_9).1 = _5;
_12.fld2.3.0 = (*_3) as isize;
_14 = _12.fld4.1.0 << _8;
_12.fld3.fld3 = _10;
_15.0.0 = _2;
(*_9).2 = core::ptr::addr_of!(_12.fld1);
_7 = _12.fld2.3.0 | _12.fld2.3.0;
match _12.fld2.2 {
332353367466428490690311999076571142724 => bb8,
_ => bb7
}
}
bb7 = {
RET = (-1381809459_i32) as u64;
(*_3) = 5287734455884371293_u64 / 5104841427491691535_u64;
_12.fld2.2 = (-24287090839690905256786638736823794327_i128) - (-81594899052396572350499908610437500515_i128);
_4 = '\u{107805}';
_12.fld2.4 = _7;
_12.fld2.5 = _3;
(*_9).2 = core::ptr::addr_of!(_12.fld1);
_12.fld0.0 = !_12.fld2.4;
(*_9).1 = _5;
(*_9).1 = [false,false];
_12.fld3.fld4 = _9;
_2 = !_1.0;
_8 = _12.fld0.2 as u64;
Goto(bb6)
}
bb8 = {
_15.0 = _12.fld3.fld0;
_12.fld4.0 = [_12.fld2.3.0,_7,_12.fld0.0,_12.fld0.0,_7,_13,_7,_7];
_1 = (_2,);
(*_9).2 = core::ptr::addr_of!(_12.fld1);
_12.fld3.fld4 = core::ptr::addr_of_mut!((*_9));
_12.fld0.0 = _7;
RET = !(*_3);
_10 = [false,false,false,false,true];
_12.fld0.2 = _7 as f32;
_15.0.0 = _12.fld3.fld0.0 ^ _12.fld3.fld0.0;
_12.fld2.2 = 7829917584822238703125616433162461896_i128 * 13431619149546123773278779359430857235_i128;
_12.fld4.0 = [_7,_7,_12.fld2.3.0,_7,_12.fld0.0,_12.fld2.3.0,_12.fld0.0,_7];
_4 = '\u{22d0b}';
_12.fld2.0 = [false,false];
_12.fld4.1.0 = 15032078050819721020_usize as u64;
(*_9).0 = _12.fld3.fld1.0;
_12.fld3.fld2 = (*_9).0;
match _6 {
0 => bb9,
1 => bb10,
13054 => bb12,
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
RET = (-1381809459_i32) as u64;
(*_3) = 5287734455884371293_u64 / 5104841427491691535_u64;
_12.fld2.2 = (-24287090839690905256786638736823794327_i128) - (-81594899052396572350499908610437500515_i128);
_4 = '\u{107805}';
_12.fld2.4 = _7;
_12.fld2.5 = _3;
(*_9).2 = core::ptr::addr_of!(_12.fld1);
_12.fld0.0 = !_12.fld2.4;
(*_9).1 = _5;
(*_9).1 = [false,false];
_12.fld3.fld4 = _9;
_2 = !_1.0;
_8 = _12.fld0.2 as u64;
Goto(bb6)
}
bb12 = {
_13 = _12.fld0.0 - _12.fld0.0;
_12.fld2.4 = 18001866811447463655_usize as isize;
Goto(bb13)
}
bb13 = {
_2 = _15.0.0 ^ _1.0;
_14 = _8 ^ _12.fld4.1.0;
_11 = _4;
_4 = _11;
_3 = core::ptr::addr_of_mut!(_14);
_22 = false;
_27.0 = _15.0.0 * _2;
(*_9).3 = 2143998371_u32;
_7 = 1323972662_i32 as isize;
(*_9).1 = [_22,_22];
_1 = (_27.0,);
_12.fld3.fld0 = (_2,);
_12.fld2.2 = 78110345545174780826893654777285444218_i128 + (-44795573680010659215676859246867675113_i128);
_23 = 168_u8;
_12.fld3.fld3 = [_22,_22,_22,_22,_22];
RET = 49110423435420164333667662601598633781_u128 as u64;
_24 = (*_3) <= _14;
(*_9).2 = core::ptr::addr_of!(_12.fld1);
_26 = [_11];
_2 = -_12.fld3.fld0.0;
_8 = (*_3) >> _27.0;
_20 = core::ptr::addr_of_mut!(_8);
match _23 {
0 => bb1,
1 => bb10,
2 => bb9,
3 => bb4,
4 => bb8,
5 => bb14,
6 => bb15,
168 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
RET = (-1381809459_i32) as u64;
(*_3) = 5287734455884371293_u64 / 5104841427491691535_u64;
_12.fld2.2 = (-24287090839690905256786638736823794327_i128) - (-81594899052396572350499908610437500515_i128);
_4 = '\u{107805}';
_12.fld2.4 = _7;
_12.fld2.5 = _3;
(*_9).2 = core::ptr::addr_of!(_12.fld1);
_12.fld0.0 = !_12.fld2.4;
(*_9).1 = _5;
(*_9).1 = [false,false];
_12.fld3.fld4 = _9;
_2 = !_1.0;
_8 = _12.fld0.2 as u64;
Goto(bb6)
}
bb16 = {
Return()
}
bb17 = {
_20 = _12.fld2.5;
Goto(bb18)
}
bb18 = {
Call(_32 = dump_var(1_usize, 7_usize, Move(_7), 6_usize, Move(_6), 8_usize, Move(_8), 27_usize, Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_32 = dump_var(1_usize, 4_usize, Move(_4), 22_usize, Move(_22), 26_usize, Move(_26), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u16,mut _2: u64,mut _3: Adt53,mut _4: i128,mut _5: (i8,),mut _6: isize,mut _7: *const u64,mut _8: *mut u64) -> f32 {
mir! {
type RET = f32;
let _9: Adt50;
let _10: char;
let _11: [i64; 1];
let _12: [char; 1];
let _13: ((i8,),);
let _14: isize;
let _15: (*const u64, *mut u128, [bool; 2]);
let _16: i32;
let _17: char;
let _18: i32;
let _19: usize;
let _20: (*const u64, *mut u128, [bool; 2]);
let _21: u128;
let _22: i8;
let _23: char;
let _24: [u32; 6];
let _25: bool;
let _26: [bool; 5];
let _27: f32;
let _28: char;
let _29: ([usize; 1], *mut (*const u64, [bool; 2], *const i64, u32), i64);
let _30: Adt52;
let _31: [i64; 4];
let _32: isize;
let _33: isize;
let _34: (u64,);
let _35: f64;
let _36: Adt63;
let _37: usize;
let _38: *mut u128;
let _39: ([bool; 2],);
let _40: char;
let _41: ();
let _42: ();
{
_5.0 = 3_usize as i8;
_4 = (-125407911351021625441257880544185066555_i128) ^ 169948048685401071959151508896370052356_i128;
_9.fld1.0 = (-21450_i16) as i8;
(*_7) = !_3.fld4.0;
RET = (-28610_i16) as f32;
_9.fld0 = ('\u{c3173}',);
(*_8) = _3.fld0 as u64;
_3.fld2 = [true,false,false,true,true];
_9.fld0.0 = '\u{5d4b1}';
_9.fld0 = ('\u{68fd}',);
_8 = core::ptr::addr_of_mut!((*_7));
RET = 3360839009102127278_i64 as f32;
_5 = (_9.fld1.0,);
_6 = (-101_isize) - (-9223372036854775808_isize);
RET = _6 as f32;
_9.fld1.0 = -_3.fld1.0;
_3.fld1.0 = _9.fld1.0;
_3.fld1 = _9.fld1;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
53993 => bb8,
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
_9.fld0 = ('\u{100977}',);
_5 = _9.fld1;
_4 = 143871795494507187238936788939780613718_i128 << _9.fld1.0;
_2 = 16799374618266243705998849435138133786_u128 as u64;
_3.fld4.0 = !(*_7);
_3.fld1 = (_5.0,);
_2 = (*_7);
_9.fld0.0 = '\u{c13d2}';
_5.0 = 3518828515280188153_i64 as i8;
_10 = _9.fld0.0;
_10 = _9.fld0.0;
_12 = [_10];
_3.fld1.0 = _5.0 >> _9.fld1.0;
_3.fld1 = _9.fld1;
(*_8) = !_2;
_8 = core::ptr::addr_of_mut!((*_8));
_6 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
RET = 1990207451_i32 as f32;
_5.0 = _3.fld1.0;
_3.fld4.0 = !(*_8);
_9.fld1.0 = 35364990809548056339830376138805784911_u128 as i8;
_1 = _3.fld0 ^ _3.fld0;
_9.fld0.0 = _10;
RET = 1265516207_i32 as f32;
_9.fld1.0 = _5.0;
_3.fld0 = _1;
Call(_10 = fn3(_3.fld1, _3.fld1.0, _5.0, (*_8), _9.fld1.0, _6, _9, _9.fld0.0, _9.fld0.0, _4, _6, _2, _7, _3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3.fld1.0 = !_9.fld1.0;
(*_7) = _2 * _3.fld4.0;
_3.fld2 = [false,false,true,true,true];
_5 = (_9.fld1.0,);
_17 = _10;
_13.0.0 = _5.0;
_3.fld2 = [true,false,false,true,false];
_3.fld0 = !_1;
_2 = _3.fld0 as u64;
_17 = _10;
_15.1 = core::ptr::addr_of_mut!(_21);
_3.fld0 = !_1;
_12 = [_10];
_7 = core::ptr::addr_of!(_3.fld4.0);
_15.2 = [true,true];
_12 = [_9.fld0.0];
_3.fld4 = (_2,);
_3.fld0 = !_1;
Goto(bb10)
}
bb10 = {
_19 = !6_usize;
_8 = core::ptr::addr_of_mut!((*_8));
(*_8) = _4 as u64;
_15.0 = _7;
(*_7) = _10 as u64;
_20.1 = core::ptr::addr_of_mut!(_21);
_13 = (_3.fld1,);
_5.0 = _9.fld1.0;
_23 = _10;
_19 = 4_usize + 0_usize;
_10 = _23;
_17 = _10;
_20 = _15;
_13.0.0 = _3.fld0 as i8;
_10 = _9.fld0.0;
_20.0 = core::ptr::addr_of!((*_7));
_14 = _6;
Call(_22 = core::intrinsics::bswap(_13.0.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11 = [8473217136923512462_i64];
_3.fld1 = (_5.0,);
_9.fld1.0 = _2 as i8;
_2 = (-959941994_i32) as u64;
_4 = 4281127877_u32 as i128;
_9.fld1 = (_5.0,);
_9.fld1 = _3.fld1;
_19 = _1 as usize;
_26 = [false,false,false,false,false];
_13 = (_9.fld1,);
_18 = 21838155_i32;
_5 = (_13.0.0,);
_15.1 = core::ptr::addr_of_mut!(_21);
_8 = core::ptr::addr_of_mut!((*_7));
_5 = (_13.0.0,);
_8 = core::ptr::addr_of_mut!(_2);
_14 = _6 << _3.fld4.0;
_25 = _9.fld1.0 <= _3.fld1.0;
_17 = _23;
_3.fld0 = _1;
_21 = 327894129425114599780683523648983587959_u128 % 164241073819190758587384581386733483447_u128;
_20.2 = [_25,_25];
_4 = -(-112825905669901371083585550454198501928_i128);
_9.fld1 = _5;
_19 = !239591737608211150_usize;
match _18 {
0 => bb1,
1 => bb8,
2 => bb10,
3 => bb4,
21838155 => bb12,
_ => bb6
}
}
bb12 = {
_15.0 = core::ptr::addr_of!((*_7));
_3.fld3 = core::ptr::addr_of!(_29.2);
_24 = [2393214267_u32,3768008242_u32,3087472260_u32,3358973435_u32,2250452066_u32,3269897854_u32];
(*_8) = 119_u8 as u64;
_25 = _19 <= _19;
_26 = _3.fld2;
(*_7) = !(*_8);
_30.fld0 = [_9.fld0.0];
_20 = (_7, _15.1, _15.2);
(*_8) = _1 as u64;
_5.0 = _3.fld1.0;
_13.0.0 = _5.0;
_31 = [8831777831834320981_i64,(-4354087498939339483_i64),5890915158491091441_i64,(-4981315719694589279_i64)];
_16 = _18;
_20 = (_7, _15.1, _15.2);
_30 = Adt52 { fld0: _12 };
_20.1 = core::ptr::addr_of_mut!(_21);
_5 = (_13.0.0,);
_20.2 = [_25,_25];
_27 = _5.0 as f32;
Call(_9.fld1.0 = core::intrinsics::transmute(_5.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_22 = -_3.fld1.0;
Call(_16 = fn18(_8, _17, _30, _5.0, (*_7), _17, _4, _9.fld0.0, _9.fld0.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_9.fld1 = (_13.0.0,);
_13 = (_3.fld1,);
_16 = _18;
(*_8) = !_3.fld4.0;
_11 = [(-5153915715296594957_i64)];
_2 = !(*_7);
_11 = [7389255154117548324_i64];
(*_7) = (*_8);
RET = 98_u8 as f32;
_36.fld0 = _20.1;
_36.fld2.fld4.0 = _3.fld4.0;
_21 = !250303751268932425936709130433059794000_u128;
_25 = _19 == _19;
(*_8) = !(*_7);
_15 = (_7, _36.fld0, _20.2);
_38 = core::ptr::addr_of_mut!(_21);
_36.fld1 = (_13.0,);
_3.fld0 = _18 as u16;
_21 = _4 as u128;
RET = _18 as f32;
_39 = (_15.2,);
_3.fld1.0 = !_13.0.0;
_9.fld0.0 = _23;
_36.fld2.fld0 = !_3.fld0;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(2_usize, 10_usize, Move(_10), 26_usize, Move(_26), 4_usize, Move(_4), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(2_usize, 25_usize, Move(_25), 12_usize, Move(_12), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(2_usize, 1_usize, Move(_1), 2_usize, Move(_2), 17_usize, Move(_17), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: (i8,),mut _2: i8,mut _3: i8,mut _4: u64,mut _5: i8,mut _6: isize,mut _7: Adt50,mut _8: char,mut _9: char,mut _10: i128,mut _11: isize,mut _12: u64,mut _13: *const u64,mut _14: Adt53) -> char {
mir! {
type RET = char;
let _15: (i8,);
let _16: Adt63;
let _17: [bool; 2];
let _18: isize;
let _19: i64;
let _20: [isize; 8];
let _21: [i64; 1];
let _22: f32;
let _23: ((*mut u64, usize, char), [char; 1], (i8,), [char; 1]);
let _24: usize;
let _25: isize;
let _26: f32;
let _27: (i8,);
let _28: *const *const u64;
let _29: isize;
let _30: u8;
let _31: *mut u128;
let _32: i32;
let _33: bool;
let _34: char;
let _35: i16;
let _36: (*const u64, *mut u128, [bool; 2]);
let _37: f32;
let _38: (char,);
let _39: Adt59;
let _40: [i64; 4];
let _41: f32;
let _42: ();
let _43: ();
{
_1.0 = _12 as i8;
_4 = !(*_13);
_12 = _4 >> _11;
RET = _9;
_14.fld1.0 = _1.0;
_8 = _7.fld0.0;
_7.fld1.0 = _14.fld1.0;
_11 = _6;
_15 = _7.fld1;
_1.0 = _3 - _5;
RET = _9;
RET = _8;
_4 = _12;
_10 = (-63133252800712329938849403318987819614_i128) + 109488454036993265033639985590032454992_i128;
_8 = _9;
_16.fld2.fld0 = _14.fld0;
_7.fld0.0 = _8;
Call(_14.fld3 = fn4(_7.fld1.0, _15, _14.fld1, _6, _15, _11, _14.fld1, _16.fld2.fld0, _7.fld0, _7, _6, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = !_5;
_18 = _6;
_1 = (_7.fld1.0,);
_16.fld1.0.0 = _1.0 * _2;
_23.2.0 = _15.0 - _7.fld1.0;
_16.fld2.fld2 = _14.fld2;
_7.fld1 = (_14.fld1.0,);
_14.fld4 = ((*_13),);
_23.0.2 = _7.fld0.0;
_1 = (_7.fld1.0,);
_23.0.2 = _8;
RET = _7.fld0.0;
_15.0 = _10 as i8;
(*_13) = !_14.fld4.0;
_23.0.2 = _8;
_24 = !7_usize;
_25 = 65755861_u32 as isize;
_27 = (_2,);
_21 = [(-7794428440800466445_i64)];
_6 = true as isize;
_23.0.1 = _24;
_22 = _12 as f32;
_4 = (*_13) ^ (*_13);
Goto(bb2)
}
bb2 = {
_24 = 269310694030776139749513207524241376648_u128 as usize;
(*_13) = !_4;
_16.fld2.fld4 = ((*_13),);
_20 = [_6,_11,_25,_11,_18,_6,_11,_11];
_26 = _22;
_23.0.1 = _24 - _24;
_10 = _7.fld0.0 as i128;
_28 = core::ptr::addr_of!(_13);
_16.fld2.fld3 = _14.fld3;
_23.2 = (_16.fld1.0.0,);
_10 = (-51485850282866208937702697757679777888_i128);
Call(_17 = fn5(_23.2.0, _11, (*_28), _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = _14.fld1;
_14 = Adt53 { fld0: _16.fld2.fld0,fld1: _16.fld1.0,fld2: _16.fld2.fld2,fld3: _16.fld2.fld3,fld4: _16.fld2.fld4 };
_14.fld4 = ((*_13),);
_16.fld2.fld1 = (_15.0,);
_3 = _22 as i8;
_12 = 235_u8 as u64;
_16.fld2.fld3 = _14.fld3;
_6 = _16.fld2.fld1.0 as isize;
_19 = (-3763287981442559966_i64);
_23.0.0 = core::ptr::addr_of_mut!(_14.fld4.0);
_23.2 = (_1.0,);
_24 = _23.0.1 % 11403684008611993595_usize;
_1.0 = _19 as i8;
Goto(bb4)
}
bb4 = {
_1 = (_3,);
Goto(bb5)
}
bb5 = {
_11 = _18;
_19 = _18 as i64;
_23.2 = _7.fld1;
_14.fld4.0 = (*_13);
_30 = _9 as u8;
RET = _8;
_5 = _1.0 << _18;
_20 = [_11,_18,_6,_6,_25,_18,_25,_6];
_23.1 = [_7.fld0.0];
_14.fld3 = _16.fld2.fld3;
_30 = !86_u8;
_24 = _23.0.1 >> (*_13);
RET = _9;
_7.fld0 = (_9,);
_14 = Adt53 { fld0: _16.fld2.fld0,fld1: _1,fld2: _16.fld2.fld2,fld3: _16.fld2.fld3,fld4: _16.fld2.fld4 };
_7.fld1 = (_1.0,);
_29 = 137399832355684530620216555377053786178_u128 as isize;
Goto(bb6)
}
bb6 = {
_8 = _9;
_33 = _6 == _18;
_27 = (_5,);
(*_13) = _22 as u64;
_20 = [_11,_6,_18,_6,_6,_6,_29,_6];
_27 = _23.2;
_1 = _7.fld1;
RET = _9;
_16.fld2 = Adt53 { fld0: _14.fld0,fld1: _1,fld2: _14.fld2,fld3: _14.fld3,fld4: _14.fld4 };
_34 = _8;
(*_13) = _16.fld2.fld4.0 & _4;
_16.fld1.0.0 = _7.fld1.0 | _7.fld1.0;
RET = _8;
RET = _9;
_1.0 = _15.0;
_34 = _23.0.2;
Call(_35 = core::intrinsics::transmute(_17), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = _7.fld0.0;
Goto(bb8)
}
bb8 = {
_27 = (_5,);
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
288796516638072254525671909674088433568 => bb10,
_ => bb9
}
}
bb9 = {
_24 = 269310694030776139749513207524241376648_u128 as usize;
(*_13) = !_4;
_16.fld2.fld4 = ((*_13),);
_20 = [_6,_11,_25,_11,_18,_6,_11,_11];
_26 = _22;
_23.0.1 = _24 - _24;
_10 = _7.fld0.0 as i128;
_28 = core::ptr::addr_of!(_13);
_16.fld2.fld3 = _14.fld3;
_23.2 = (_16.fld1.0.0,);
_10 = (-51485850282866208937702697757679777888_i128);
Call(_17 = fn5(_23.2.0, _11, (*_28), _10), ReturnTo(bb3), UnwindUnreachable())
}
bb10 = {
_23.0.1 = !_24;
_16.fld2.fld3 = core::ptr::addr_of!(_19);
_14.fld1.0 = _1.0 >> _18;
_14.fld1 = _1;
RET = _9;
_26 = _18 as f32;
Goto(bb11)
}
bb11 = {
_27.0 = _3;
_23.2 = _16.fld1.0;
_39.fld2.3.0 = -_18;
_7.fld0 = (_9,);
_39.fld2.5 = core::ptr::addr_of_mut!(_39.fld4.1.0);
_25 = _10 as isize;
_39.fld2.3.1 = core::ptr::addr_of!(_32);
_39.fld2.0 = [_33,_33];
_11 = _6;
match _10 {
0 => bb9,
1 => bb12,
2 => bb13,
3 => bb14,
288796516638072254525671909674088433568 => bb16,
_ => bb15
}
}
bb12 = {
_23.0.1 = !_24;
_16.fld2.fld3 = core::ptr::addr_of!(_19);
_14.fld1.0 = _1.0 >> _18;
_14.fld1 = _1;
RET = _9;
_26 = _18 as f32;
Goto(bb11)
}
bb13 = {
_11 = _18;
_19 = _18 as i64;
_23.2 = _7.fld1;
_14.fld4.0 = (*_13);
_30 = _9 as u8;
RET = _8;
_5 = _1.0 << _18;
_20 = [_11,_18,_6,_6,_25,_18,_25,_6];
_23.1 = [_7.fld0.0];
_14.fld3 = _16.fld2.fld3;
_30 = !86_u8;
_24 = _23.0.1 >> (*_13);
RET = _9;
_7.fld0 = (_9,);
_14 = Adt53 { fld0: _16.fld2.fld0,fld1: _1,fld2: _16.fld2.fld2,fld3: _16.fld2.fld3,fld4: _16.fld2.fld4 };
_7.fld1 = (_1.0,);
_29 = 137399832355684530620216555377053786178_u128 as isize;
Goto(bb6)
}
bb14 = {
_8 = _9;
_33 = _6 == _18;
_27 = (_5,);
(*_13) = _22 as u64;
_20 = [_11,_6,_18,_6,_6,_6,_29,_6];
_27 = _23.2;
_1 = _7.fld1;
RET = _9;
_16.fld2 = Adt53 { fld0: _14.fld0,fld1: _1,fld2: _14.fld2,fld3: _14.fld3,fld4: _14.fld4 };
_34 = _8;
(*_13) = _16.fld2.fld4.0 & _4;
_16.fld1.0.0 = _7.fld1.0 | _7.fld1.0;
RET = _8;
RET = _9;
_1.0 = _15.0;
_34 = _23.0.2;
Call(_35 = core::intrinsics::transmute(_17), ReturnTo(bb7), UnwindUnreachable())
}
bb15 = {
_9 = _7.fld0.0;
Goto(bb8)
}
bb16 = {
_16.fld2.fld2 = _14.fld2;
_16.fld2.fld1 = _16.fld1.0;
_28 = core::ptr::addr_of!(_13);
_28 = core::ptr::addr_of!(_39.fld3.fld2);
_39.fld2.3.2 = _22;
_16.fld2.fld4.0 = (*_13);
_27.0 = _15.0;
_39.fld2.3.3 = core::ptr::addr_of!(_32);
RET = _8;
_36.2 = [_33,_33];
_19 = 532532435361415673_i64 & 2584395338570211533_i64;
_39.fld4.0 = [_18,_18,_18,_11,_6,_6,_11,_6];
_27 = (_3,);
_18 = 31081306389973491136017872170492139510_u128 as isize;
_39.fld1 = _19;
_39.fld0.2 = _22;
_5 = _7.fld1.0;
_39.fld0 = _39.fld2.3;
_8 = _9;
_16.fld2 = Adt53 { fld0: _14.fld0,fld1: _27,fld2: _14.fld2,fld3: _14.fld3,fld4: _14.fld4 };
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(3_usize, 8_usize, Move(_8), 9_usize, Move(_9), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(3_usize, 11_usize, Move(_11), 6_usize, Move(_6), 33_usize, Move(_33), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(3_usize, 30_usize, Move(_30), 17_usize, Move(_17), 3_usize, Move(_3), 27_usize, Move(_27)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i8,mut _2: (i8,),mut _3: (i8,),mut _4: isize,mut _5: (i8,),mut _6: isize,mut _7: (i8,),mut _8: u16,mut _9: (char,),mut _10: Adt50,mut _11: isize,mut _12: (i8,)) -> *const i64 {
mir! {
type RET = *const i64;
let _13: Adt47;
let _14: isize;
let _15: ((i8,),);
let _16: u8;
let _17: isize;
let _18: (char,);
let _19: [isize; 8];
let _20: ([bool; 2],);
let _21: i64;
let _22: i32;
let _23: [i64; 4];
let _24: [usize; 1];
let _25: bool;
let _26: [isize; 8];
let _27: isize;
let _28: ();
let _29: ();
{
_3 = _5;
_13.fld0 = 244799775619742007541811014197890341655_u128 % 158359777387126317487806090728061788010_u128;
_13.fld2.1 = (11123440088346663101_u64,);
_5.0 = _2.0 << _11;
_13.fld0 = 312458889678488579299134674596792322908_u128 / 165169030326486085677681957268495751889_u128;
_3.0 = !_10.fld1.0;
_11 = _6 ^ _4;
_11 = _6 + _6;
_14 = !_6;
_13.fld2.0 = [_11,_6,_11,_14,_6,_11,_11,_14];
_7.0 = _12.0 >> _2.0;
_5.0 = _1 - _7.0;
_15 = (_7,);
_2.0 = _10.fld0.0 as i8;
_5 = (_7.0,);
_13.fld1 = (-101429864702289522511755975523635349468_i128) ^ (-130153171376956592583493573765956096521_i128);
_5.0 = _7.0;
_5 = (_3.0,);
_15.0 = (_5.0,);
_14 = 18695_i16 as isize;
_13.fld2.1 = (11897106478839519280_u64,);
_1 = _3.0;
_3.0 = _7.0;
_15.0.0 = -_2.0;
_2.0 = _10.fld1.0 ^ _7.0;
Call(_10.fld1.0 = core::intrinsics::transmute(_12.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13.fld1 = 31367005564670174391438150038535318040_i128;
_13.fld0 = 219966300778371143640429048549336477125_u128 | 105377796465046622234977428246932795484_u128;
_2.0 = -_7.0;
_13.fld0 = (-7427221601561005286_i64) as u128;
_10.fld0.0 = _9.0;
_6 = _9.0 as isize;
_10.fld1.0 = _12.0;
_10.fld0.0 = _9.0;
_13.fld2.0 = [_4,_11,_4,_6,_14,_11,_11,_11];
_9 = (_10.fld0.0,);
_3 = (_7.0,);
_18 = (_10.fld0.0,);
_19 = _13.fld2.0;
_2.0 = 227_u8 as i8;
_2.0 = _15.0.0;
_10.fld0.0 = _9.0;
_14 = _11;
_12 = (_10.fld1.0,);
_8 = _10.fld1.0 as u16;
_15.0.0 = _13.fld1 as i8;
_16 = 186_u8 - 215_u8;
_17 = _13.fld1 as isize;
_14 = _17;
_2.0 = true as i8;
_14 = _11 - _4;
_9 = (_18.0,);
_13.fld0 = !157847922438282618600365529027330003777_u128;
Goto(bb2)
}
bb2 = {
_3 = (_10.fld1.0,);
_16 = 96_u8 + 239_u8;
_3 = (_12.0,);
_20.0 = [false,false];
_2.0 = _10.fld1.0 & _5.0;
_5 = (_2.0,);
_13.fld2.1 = (13701353998954241689_u64,);
match _13.fld2.1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
13701353998954241689 => bb8,
_ => bb7
}
}
bb3 = {
_13.fld1 = 31367005564670174391438150038535318040_i128;
_13.fld0 = 219966300778371143640429048549336477125_u128 | 105377796465046622234977428246932795484_u128;
_2.0 = -_7.0;
_13.fld0 = (-7427221601561005286_i64) as u128;
_10.fld0.0 = _9.0;
_6 = _9.0 as isize;
_10.fld1.0 = _12.0;
_10.fld0.0 = _9.0;
_13.fld2.0 = [_4,_11,_4,_6,_14,_11,_11,_11];
_9 = (_10.fld0.0,);
_3 = (_7.0,);
_18 = (_10.fld0.0,);
_19 = _13.fld2.0;
_2.0 = 227_u8 as i8;
_2.0 = _15.0.0;
_10.fld0.0 = _9.0;
_14 = _11;
_12 = (_10.fld1.0,);
_8 = _10.fld1.0 as u16;
_15.0.0 = _13.fld1 as i8;
_16 = 186_u8 - 215_u8;
_17 = _13.fld1 as isize;
_14 = _17;
_2.0 = true as i8;
_14 = _11 - _4;
_9 = (_18.0,);
_13.fld0 = !157847922438282618600365529027330003777_u128;
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
_5 = (_3.0,);
_12 = (_5.0,);
_12.0 = _18.0 as i8;
RET = core::ptr::addr_of!(_21);
_2 = (_10.fld1.0,);
_18.0 = _10.fld0.0;
_13.fld2.0 = _19;
_1 = !_5.0;
(*RET) = 8527157542412895004_i64;
_19 = [_14,_11,_6,_4,_4,_6,_11,_11];
_13.fld2.1.0 = !14746434530896326337_u64;
_7.0 = _13.fld0 as i8;
_2 = (_5.0,);
_13.fld2.0 = [_4,_17,_4,_14,_14,_14,_11,_14];
_12 = (_2.0,);
_5 = _7;
match _21 {
0 => bb5,
1 => bb9,
2 => bb10,
3 => bb11,
8527157542412895004 => bb13,
_ => bb12
}
}
bb9 = {
_3 = (_10.fld1.0,);
_16 = 96_u8 + 239_u8;
_3 = (_12.0,);
_20.0 = [false,false];
_2.0 = _10.fld1.0 & _5.0;
_5 = (_2.0,);
_13.fld2.1 = (13701353998954241689_u64,);
match _13.fld2.1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
13701353998954241689 => bb8,
_ => bb7
}
}
bb10 = {
_13.fld1 = 31367005564670174391438150038535318040_i128;
_13.fld0 = 219966300778371143640429048549336477125_u128 | 105377796465046622234977428246932795484_u128;
_2.0 = -_7.0;
_13.fld0 = (-7427221601561005286_i64) as u128;
_10.fld0.0 = _9.0;
_6 = _9.0 as isize;
_10.fld1.0 = _12.0;
_10.fld0.0 = _9.0;
_13.fld2.0 = [_4,_11,_4,_6,_14,_11,_11,_11];
_9 = (_10.fld0.0,);
_3 = (_7.0,);
_18 = (_10.fld0.0,);
_19 = _13.fld2.0;
_2.0 = 227_u8 as i8;
_2.0 = _15.0.0;
_10.fld0.0 = _9.0;
_14 = _11;
_12 = (_10.fld1.0,);
_8 = _10.fld1.0 as u16;
_15.0.0 = _13.fld1 as i8;
_16 = 186_u8 - 215_u8;
_17 = _13.fld1 as isize;
_14 = _17;
_2.0 = true as i8;
_14 = _11 - _4;
_9 = (_18.0,);
_13.fld0 = !157847922438282618600365529027330003777_u128;
Goto(bb2)
}
bb11 = {
_13.fld1 = 31367005564670174391438150038535318040_i128;
_13.fld0 = 219966300778371143640429048549336477125_u128 | 105377796465046622234977428246932795484_u128;
_2.0 = -_7.0;
_13.fld0 = (-7427221601561005286_i64) as u128;
_10.fld0.0 = _9.0;
_6 = _9.0 as isize;
_10.fld1.0 = _12.0;
_10.fld0.0 = _9.0;
_13.fld2.0 = [_4,_11,_4,_6,_14,_11,_11,_11];
_9 = (_10.fld0.0,);
_3 = (_7.0,);
_18 = (_10.fld0.0,);
_19 = _13.fld2.0;
_2.0 = 227_u8 as i8;
_2.0 = _15.0.0;
_10.fld0.0 = _9.0;
_14 = _11;
_12 = (_10.fld1.0,);
_8 = _10.fld1.0 as u16;
_15.0.0 = _13.fld1 as i8;
_16 = 186_u8 - 215_u8;
_17 = _13.fld1 as isize;
_14 = _17;
_2.0 = true as i8;
_14 = _11 - _4;
_9 = (_18.0,);
_13.fld0 = !157847922438282618600365529027330003777_u128;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_12.0 = _10.fld1.0 & _2.0;
_5 = (_2.0,);
_22 = (-1556629895_i32) >> _14;
_3 = (_2.0,);
_18 = (_9.0,);
_9 = _18;
_22 = 38951102_i32 << _1;
_10.fld1.0 = _12.0;
_8 = _13.fld0 as u16;
_10.fld0.0 = _18.0;
(*RET) = 4693751044846535855_i64 >> _17;
RET = core::ptr::addr_of!((*RET));
_11 = _6;
_18.0 = _10.fld0.0;
RET = core::ptr::addr_of!((*RET));
_12.0 = _2.0;
_12 = _3;
_6 = _17;
_13.fld2.1.0 = 11820294269334729815_u64 >> _22;
_27 = _14;
_13.fld1 = 62797674559453625448521292233944653182_i128 | (-96799260892658542391080543383038304755_i128);
_3 = (_2.0,);
Goto(bb14)
}
bb14 = {
_13.fld1 = 151226433313386714224978736716275363788_i128;
_5 = _10.fld1;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(4_usize, 6_usize, Move(_6), 1_usize, Move(_1), 20_usize, Move(_20), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(4_usize, 5_usize, Move(_5), 17_usize, Move(_17), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(4_usize, 15_usize, Move(_15), 14_usize, Move(_14), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i8,mut _2: isize,mut _3: *const u64,mut _4: i128) -> [bool; 2] {
mir! {
type RET = [bool; 2];
let _5: [usize; 1];
let _6: Adt52;
let _7: [u32; 6];
let _8: ([bool; 2], *const i32, i128, (isize, *const i32, f32, *const i32), isize, *mut u64);
let _9: char;
let _10: u16;
let _11: Adt58;
let _12: Adt55;
let _13: ([usize; 1], *mut (*const u64, [bool; 2], *const i64, u32), i64);
let _14: [bool; 5];
let _15: ([bool; 2], *const i32, i128, (isize, *const i32, f32, *const i32), isize, *mut u64);
let _16: bool;
let _17: [char; 1];
let _18: [i64; 1];
let _19: Adt55;
let _20: *const *const u64;
let _21: isize;
let _22: f64;
let _23: f32;
let _24: [char; 1];
let _25: Adt62;
let _26: f32;
let _27: u128;
let _28: [i16; 6];
let _29: Adt56;
let _30: ();
let _31: ();
{
RET = [true,false];
_2 = '\u{4d407}' as isize;
(*_3) = 669164224796049703_u64 * 4898040978306972204_u64;
(*_3) = 2_usize as u64;
RET = [true,true];
_5 = [3_usize];
_2 = 80_isize << (*_3);
_5 = [0_usize];
_4 = 49_u8 as i128;
(*_3) = !15180634599842972456_u64;
RET = [true,true];
_1 = (-77_i8) - (-55_i8);
(*_3) = _1 as u64;
(*_3) = (-1275046422_i32) as u64;
_8.3.2 = 7_usize as f32;
_8.5 = core::ptr::addr_of_mut!((*_3));
_8.3.2 = _1 as f32;
_7 = [4258163896_u32,4247120074_u32,1408740291_u32,1969462921_u32,2885855690_u32,3267085333_u32];
(*_3) = 3162021338_u32 as u64;
_8.2 = -_4;
_9 = '\u{6a471}';
_8.4 = _2 | _2;
Goto(bb1)
}
bb1 = {
_6.fld0 = [_9];
_12.fld3.3 = 2827115218_u32;
_12.fld7.fld0 = core::ptr::addr_of_mut!((*_3));
_4 = _8.2;
_12.fld7.fld1 = ((*_3),);
(*_3) = !_12.fld7.fld1.0;
_12.fld5.5 = _8.5;
_12.fld4 = [_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3];
_12.fld5.2 = !_4;
_12.fld1 = core::ptr::addr_of_mut!(_12.fld3);
_1 = 42455_u16 as i8;
_12.fld2 = [(-1222698976928720639_i64),(-3055752544520810627_i64),5195533335045645086_i64,(-4868993583154560754_i64)];
_12.fld2 = [(-9173381209785478236_i64),(-6331865782698242314_i64),5338843880039520088_i64,(-3261762649262797535_i64)];
_15.0 = [false,false];
Goto(bb2)
}
bb2 = {
_15.3.0 = -_8.4;
_12.fld3.3 = !1460921697_u32;
_15.3.0 = (-811493651_i32) as isize;
_17 = [_9];
_15.4 = _8.4 + _2;
_12.fld5.3.0 = _15.3.0;
_19.fld2 = [7506627280724291812_i64,7220907717487466832_i64,912495798080103624_i64,5589249438406096907_i64];
_12.fld5.2 = _8.2;
RET = _15.0;
_11.fld1 = (-6920460917948770325_i64);
_12.fld6 = [true,true,false,false,true];
_19.fld3.3 = _1 as u32;
_19.fld5.3.0 = _15.4;
_19.fld5.4 = 295857923784207722797470517063757379211_u128 as isize;
_19.fld3.3 = !_12.fld3.3;
_12.fld4 = [_12.fld3.3,_19.fld3.3,_12.fld3.3,_19.fld3.3,_19.fld3.3,_19.fld3.3];
_19.fld7.fld1 = ((*_3),);
_12.fld3.2 = core::ptr::addr_of!(_11.fld1);
_15.5 = core::ptr::addr_of_mut!(_19.fld7.fld1.0);
_10 = 21580_u16;
_19.fld5.3.2 = -_8.3.2;
Goto(bb3)
}
bb3 = {
_12.fld3.3 = _19.fld3.3;
_15.2 = _8.2 * _8.2;
_8.0 = [false,false];
_16 = false;
_12.fld5.0 = _8.0;
_19.fld7.fld1.0 = (*_3) * (*_3);
_19.fld6 = [_16,_16,_16,_16,_16];
_12.fld7.fld1 = _19.fld7.fld1;
_12.fld5.2 = _4;
_19.fld4 = [_19.fld3.3,_12.fld3.3,_19.fld3.3,_19.fld3.3,_12.fld3.3,_12.fld3.3];
_15.4 = !_8.4;
_8.3.0 = _19.fld5.3.0;
_12.fld3.3 = !_19.fld3.3;
_9 = '\u{23e2a}';
_15.2 = _12.fld5.2 | _8.2;
Goto(bb4)
}
bb4 = {
_8.0 = _15.0;
_14 = _12.fld6;
_19.fld3 = (_3, _12.fld5.0, _12.fld3.2, _12.fld3.3);
_19.fld2 = [_11.fld1,_11.fld1,_11.fld1,_11.fld1];
_12.fld0 = _11.fld1 as u8;
Call(_19.fld4 = fn6(_9, _8.4, _12.fld1, _12.fld6, _19.fld3, _12.fld1, _6, _12.fld5.3.0, _12.fld0, _19.fld3.2, _8.3.2, _4, _7, _8.5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_19.fld2 = [_11.fld1,_11.fld1,_11.fld1,_11.fld1];
_4 = _12.fld5.2 * _15.2;
(*_3) = !_19.fld7.fld1.0;
_11.fld2.0 = _12.fld3.3 as i8;
_12.fld2 = _19.fld2;
_19.fld0 = (*_3) as u8;
_13.1 = core::ptr::addr_of_mut!(_12.fld3);
_13.2 = _11.fld1 & _11.fld1;
_8.3.2 = _19.fld5.3.2;
_19.fld5.2 = _4 ^ _15.2;
_13 = (_5, _12.fld1, _11.fld1);
_15.3.0 = _15.4 | _19.fld5.3.0;
_4 = _12.fld5.2;
_13.1 = core::ptr::addr_of_mut!(_12.fld3);
_5 = _13.0;
_15.3.2 = _15.2 as f32;
match _10 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
21580 => bb10,
_ => bb9
}
}
bb6 = {
_8.0 = _15.0;
_14 = _12.fld6;
_19.fld3 = (_3, _12.fld5.0, _12.fld3.2, _12.fld3.3);
_19.fld2 = [_11.fld1,_11.fld1,_11.fld1,_11.fld1];
_12.fld0 = _11.fld1 as u8;
Call(_19.fld4 = fn6(_9, _8.4, _12.fld1, _12.fld6, _19.fld3, _12.fld1, _6, _12.fld5.3.0, _12.fld0, _19.fld3.2, _8.3.2, _4, _7, _8.5), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_12.fld3.3 = _19.fld3.3;
_15.2 = _8.2 * _8.2;
_8.0 = [false,false];
_16 = false;
_12.fld5.0 = _8.0;
_19.fld7.fld1.0 = (*_3) * (*_3);
_19.fld6 = [_16,_16,_16,_16,_16];
_12.fld7.fld1 = _19.fld7.fld1;
_12.fld5.2 = _4;
_19.fld4 = [_19.fld3.3,_12.fld3.3,_19.fld3.3,_19.fld3.3,_12.fld3.3,_12.fld3.3];
_15.4 = !_8.4;
_8.3.0 = _19.fld5.3.0;
_12.fld3.3 = !_19.fld3.3;
_9 = '\u{23e2a}';
_15.2 = _12.fld5.2 | _8.2;
Goto(bb4)
}
bb8 = {
_15.3.0 = -_8.4;
_12.fld3.3 = !1460921697_u32;
_15.3.0 = (-811493651_i32) as isize;
_17 = [_9];
_15.4 = _8.4 + _2;
_12.fld5.3.0 = _15.3.0;
_19.fld2 = [7506627280724291812_i64,7220907717487466832_i64,912495798080103624_i64,5589249438406096907_i64];
_12.fld5.2 = _8.2;
RET = _15.0;
_11.fld1 = (-6920460917948770325_i64);
_12.fld6 = [true,true,false,false,true];
_19.fld3.3 = _1 as u32;
_19.fld5.3.0 = _15.4;
_19.fld5.4 = 295857923784207722797470517063757379211_u128 as isize;
_19.fld3.3 = !_12.fld3.3;
_12.fld4 = [_12.fld3.3,_19.fld3.3,_12.fld3.3,_19.fld3.3,_19.fld3.3,_19.fld3.3];
_19.fld7.fld1 = ((*_3),);
_12.fld3.2 = core::ptr::addr_of!(_11.fld1);
_15.5 = core::ptr::addr_of_mut!(_19.fld7.fld1.0);
_10 = 21580_u16;
_19.fld5.3.2 = -_8.3.2;
Goto(bb3)
}
bb9 = {
_6.fld0 = [_9];
_12.fld3.3 = 2827115218_u32;
_12.fld7.fld0 = core::ptr::addr_of_mut!((*_3));
_4 = _8.2;
_12.fld7.fld1 = ((*_3),);
(*_3) = !_12.fld7.fld1.0;
_12.fld5.5 = _8.5;
_12.fld4 = [_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3];
_12.fld5.2 = !_4;
_12.fld1 = core::ptr::addr_of_mut!(_12.fld3);
_1 = 42455_u16 as i8;
_12.fld2 = [(-1222698976928720639_i64),(-3055752544520810627_i64),5195533335045645086_i64,(-4868993583154560754_i64)];
_12.fld2 = [(-9173381209785478236_i64),(-6331865782698242314_i64),5338843880039520088_i64,(-3261762649262797535_i64)];
_15.0 = [false,false];
Goto(bb2)
}
bb10 = {
_8.2 = _19.fld5.3.2 as i128;
_17 = [_9];
_7 = [_19.fld3.3,_19.fld3.3,_12.fld3.3,_19.fld3.3,_19.fld3.3,_19.fld3.3];
_19.fld5.0 = [_16,_16];
_13 = (_5, _12.fld1, _11.fld1);
_11.fld2.0 = _1 << _19.fld0;
_20 = core::ptr::addr_of!(_3);
_19.fld7.fld1.0 = _9 as u64;
(*_3) = _19.fld7.fld1.0 % 18079354228911010641_u64;
_12.fld1 = core::ptr::addr_of_mut!(_19.fld3);
_10 = 29007_u16 ^ 31415_u16;
_11.fld0 = _20;
_18 = [_11.fld1];
Call(_19.fld7.fld1 = fn7(_11.fld0, _11.fld2, _15.3.0, _8.4, _9, (*_20), (*_20), _19.fld5.3.0, (*_20), _15.0, _2, _13, _13.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_20) = core::ptr::addr_of!((*_3));
_10 = 40939_u16;
_19.fld3.0 = _3;
_12.fld3.0 = core::ptr::addr_of!((*_3));
_19.fld3.0 = core::ptr::addr_of!((*_3));
_19.fld1 = core::ptr::addr_of_mut!(_12.fld3);
_24 = [_9];
_8.4 = _15.3.0;
_25.fld6.fld1.1.0 = !(*_3);
_15.2 = -_8.2;
_19.fld5.2 = _4;
_19.fld0 = !_12.fld0;
Call(_25.fld0 = core::intrinsics::transmute(_19.fld5.3.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_12.fld5.3.2 = -_19.fld5.3.2;
match _13.2 {
0 => bb10,
1 => bb2,
2 => bb8,
3 => bb11,
4 => bb5,
5 => bb13,
6 => bb14,
340282366920938463456454146513819441131 => bb16,
_ => bb15
}
}
bb13 = {
_8.0 = _15.0;
_14 = _12.fld6;
_19.fld3 = (_3, _12.fld5.0, _12.fld3.2, _12.fld3.3);
_19.fld2 = [_11.fld1,_11.fld1,_11.fld1,_11.fld1];
_12.fld0 = _11.fld1 as u8;
Call(_19.fld4 = fn6(_9, _8.4, _12.fld1, _12.fld6, _19.fld3, _12.fld1, _6, _12.fld5.3.0, _12.fld0, _19.fld3.2, _8.3.2, _4, _7, _8.5), ReturnTo(bb5), UnwindUnreachable())
}
bb14 = {
_8.2 = _19.fld5.3.2 as i128;
_17 = [_9];
_7 = [_19.fld3.3,_19.fld3.3,_12.fld3.3,_19.fld3.3,_19.fld3.3,_19.fld3.3];
_19.fld5.0 = [_16,_16];
_13 = (_5, _12.fld1, _11.fld1);
_11.fld2.0 = _1 << _19.fld0;
_20 = core::ptr::addr_of!(_3);
_19.fld7.fld1.0 = _9 as u64;
(*_3) = _19.fld7.fld1.0 % 18079354228911010641_u64;
_12.fld1 = core::ptr::addr_of_mut!(_19.fld3);
_10 = 29007_u16 ^ 31415_u16;
_11.fld0 = _20;
_18 = [_11.fld1];
Call(_19.fld7.fld1 = fn7(_11.fld0, _11.fld2, _15.3.0, _8.4, _9, (*_20), (*_20), _19.fld5.3.0, (*_20), _15.0, _2, _13, _13.0), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_6.fld0 = [_9];
_12.fld3.3 = 2827115218_u32;
_12.fld7.fld0 = core::ptr::addr_of_mut!((*_3));
_4 = _8.2;
_12.fld7.fld1 = ((*_3),);
(*_3) = !_12.fld7.fld1.0;
_12.fld5.5 = _8.5;
_12.fld4 = [_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3,_12.fld3.3];
_12.fld5.2 = !_4;
_12.fld1 = core::ptr::addr_of_mut!(_12.fld3);
_1 = 42455_u16 as i8;
_12.fld2 = [(-1222698976928720639_i64),(-3055752544520810627_i64),5195533335045645086_i64,(-4868993583154560754_i64)];
_12.fld2 = [(-9173381209785478236_i64),(-6331865782698242314_i64),5338843880039520088_i64,(-3261762649262797535_i64)];
_15.0 = [false,false];
Goto(bb2)
}
bb16 = {
_25.fld1 = _9;
_3 = core::ptr::addr_of!((*_3));
(*_20) = core::ptr::addr_of!(_25.fld6.fld1.1.0);
_19.fld7.fld2 = core::ptr::addr_of_mut!(_27);
_11.fld2 = (_1,);
_6 = Adt52 { fld0: _17 };
_12.fld5.3.0 = _19.fld5.3.0 ^ _19.fld5.3.0;
_19.fld3.3 = _12.fld3.3;
_8.5 = core::ptr::addr_of_mut!((*_3));
_12.fld4 = _19.fld4;
_12.fld7.fld2 = core::ptr::addr_of_mut!(_29.fld5.fld0);
_8.3.0 = _2;
_29.fld5.fld0 = 184612345737988659723059865883580808906_u128 % 294391928170996051777290685783124041414_u128;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(5_usize, 5_usize, Move(_5), 1_usize, Move(_1), 24_usize, Move(_24), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(5_usize, 18_usize, Move(_18), 17_usize, Move(_17), 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: char,mut _2: isize,mut _3: *mut (*const u64, [bool; 2], *const i64, u32),mut _4: [bool; 5],mut _5: (*const u64, [bool; 2], *const i64, u32),mut _6: *mut (*const u64, [bool; 2], *const i64, u32),mut _7: Adt52,mut _8: isize,mut _9: u8,mut _10: *const i64,mut _11: f32,mut _12: i128,mut _13: [u32; 6],mut _14: *mut u64) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _15: [u32; 6];
let _16: *const u64;
let _17: [bool; 5];
let _18: i64;
let _19: char;
let _20: i16;
let _21: ();
let _22: ();
{
(*_3).0 = core::ptr::addr_of!((*_14));
_3 = core::ptr::addr_of_mut!((*_6));
(*_3).1 = [false,false];
_10 = core::ptr::addr_of!((*_10));
(*_6) = (_5.0, _5.1, _10, _5.3);
_6 = _3;
(*_3).2 = core::ptr::addr_of!((*_10));
_15 = _13;
_1 = '\u{4deb6}';
(*_3).2 = core::ptr::addr_of!((*_10));
(*_3).0 = core::ptr::addr_of!((*_14));
(*_6).1 = [true,false];
_8 = _2;
RET = _13;
_5.2 = core::ptr::addr_of!((*_10));
(*_3).0 = core::ptr::addr_of!((*_14));
_3 = core::ptr::addr_of_mut!((*_6));
_11 = (*_6).3 as f32;
_2 = -_8;
_14 = core::ptr::addr_of_mut!((*_14));
RET = [(*_3).3,(*_6).3,(*_3).3,_5.3,(*_3).3,(*_6).3];
_10 = core::ptr::addr_of!((*_10));
(*_3).2 = core::ptr::addr_of!((*_10));
_2 = -_8;
_9 = (-45_i8) as u8;
_17 = [false,true,false,true,true];
_9 = 71_u8;
match (*_10) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463456454146513819441131 => bb9,
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
(*_6).0 = core::ptr::addr_of!((*_14));
(*_6) = _5;
_6 = core::ptr::addr_of_mut!((*_3));
_8 = _2 << (*_6).3;
Goto(bb10)
}
bb10 = {
RET = _15;
match (*_10) {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
340282366920938463456454146513819441131 => bb18,
_ => bb17
}
}
bb11 = {
(*_6).0 = core::ptr::addr_of!((*_14));
(*_6) = _5;
_6 = core::ptr::addr_of_mut!((*_3));
_8 = _2 << (*_6).3;
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
_20 = (-1089631930_i32) as i16;
Goto(bb19)
}
bb19 = {
Call(_21 = dump_var(6_usize, 4_usize, Move(_4), 15_usize, Move(_15), 12_usize, Move(_12), 9_usize, Move(_9)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_21 = dump_var(6_usize, 1_usize, Move(_1), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *const *const u64,mut _2: (i8,),mut _3: isize,mut _4: isize,mut _5: char,mut _6: *const u64,mut _7: *const u64,mut _8: isize,mut _9: *const u64,mut _10: [bool; 2],mut _11: isize,mut _12: ([usize; 1], *mut (*const u64, [bool; 2], *const i64, u32), i64),mut _13: [usize; 1]) -> (u64,) {
mir! {
type RET = (u64,);
let _14: ((i8,),);
let _15: isize;
let _16: (*const u64,);
let _17: *mut u128;
let _18: u8;
let _19: f32;
let _20: *mut u128;
let _21: f64;
let _22: isize;
let _23: isize;
let _24: isize;
let _25: f64;
let _26: i32;
let _27: f32;
let _28: u128;
let _29: ([usize; 1], *mut (*const u64, [bool; 2], *const i64, u32), i64);
let _30: ([isize; 8], (u64,));
let _31: isize;
let _32: [bool; 5];
let _33: bool;
let _34: Adt57;
let _35: f64;
let _36: (*mut u64, usize, char);
let _37: i64;
let _38: (u64,);
let _39: char;
let _40: [i64; 1];
let _41: bool;
let _42: Adt57;
let _43: bool;
let _44: usize;
let _45: Adt57;
let _46: ();
let _47: ();
{
_13 = [1_usize];
(*_6) = !10624665428746987522_u64;
_9 = _6;
RET.0 = (*_6);
_13 = [0_usize];
_10 = [true,false];
_10 = [false,false];
_4 = _3;
RET.0 = (*_9);
_19 = 72609548808373656779373607623405157917_i128 as f32;
_10 = [false,false];
(*_1) = core::ptr::addr_of!((*_6));
_5 = '\u{3ce78}';
RET.0 = !(*_9);
_5 = '\u{fecdd}';
Goto(bb1)
}
bb1 = {
_18 = !54_u8;
RET.0 = (*_7);
Call(_3 = core::intrinsics::bswap(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.0 = !(*_6);
_22 = _18 as isize;
_16.0 = core::ptr::addr_of!((*_6));
(*_7) = 1834476174828364586_u64 - 17817078092587731772_u64;
_14.0 = (_2.0,);
_1 = core::ptr::addr_of!(_6);
(*_7) = 13643223676095281850_u64 % 4523029897200152849_u64;
_22 = _4 >> _3;
_22 = _4 << _18;
_23 = _4;
_2 = (_14.0.0,);
(*_9) = !800184108833065926_u64;
Call(_17 = fn8(_12.2, _10, _22, _9, _16, _13, (*_6), _11, _12.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = '\u{1091d6}';
(*_1) = core::ptr::addr_of!((*_9));
_21 = (-595957552_i32) as f64;
_15 = _22 - _23;
_8 = !_15;
(*_9) = 233283133491305483171323594558163342686_u128 as u64;
_16 = ((*_1),);
_22 = _8;
_20 = _17;
_3 = _22;
_25 = _21 + _21;
_3 = _8;
_29.0 = [0_usize];
_29.2 = -_12.2;
_4 = (-43453305_i32) as isize;
_24 = false as isize;
_3 = _8;
_31 = !_3;
match _12.2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463456454146513819441131 => bb8,
_ => bb7
}
}
bb4 = {
RET.0 = !(*_6);
_22 = _18 as isize;
_16.0 = core::ptr::addr_of!((*_6));
(*_7) = 1834476174828364586_u64 - 17817078092587731772_u64;
_14.0 = (_2.0,);
_1 = core::ptr::addr_of!(_6);
(*_7) = 13643223676095281850_u64 % 4523029897200152849_u64;
_22 = _4 >> _3;
_22 = _4 << _18;
_23 = _4;
_2 = (_14.0.0,);
(*_9) = !800184108833065926_u64;
Call(_17 = fn8(_12.2, _10, _22, _9, _16, _13, (*_6), _11, _12.1), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_18 = !54_u8;
RET.0 = (*_7);
Call(_3 = core::intrinsics::bswap(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_34.fld0 = _19;
(*_7) = _25 as u64;
_23 = _31 ^ _22;
_33 = _3 > _3;
_4 = _23;
_6 = core::ptr::addr_of!((*_7));
_29.0 = [2_usize];
_34.fld0 = 332767921916136209325196402221462707263_u128 as f32;
_4 = !_15;
_5 = '\u{1a2e6}';
_36.0 = core::ptr::addr_of_mut!((*_7));
_27 = _19 + _34.fld0;
_1 = core::ptr::addr_of!(_7);
_30.1 = ((*_7),);
_34.fld1 = (*_7);
_34.fld2.0 = [14293240716714419763_usize];
_16 = (_6,);
(*_9) = _4 as u64;
_18 = _33 as u8;
_28 = _12.2 as u128;
Call(_38.0 = core::intrinsics::bswap((*_7)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_39 = _5;
_29.0 = _13;
_34.fld2.1 = _12.1;
_30.1 = ((*_9),);
_29.0 = _34.fld2.0;
_15 = _28 as isize;
_42.fld2.1 = _34.fld2.1;
_42 = Adt57 { fld0: _34.fld0,fld1: (*_9),fld2: _12 };
_25 = -_21;
match _42.fld2.2 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb7,
340282366920938463456454146513819441131 => bb10,
_ => bb5
}
}
bb10 = {
_35 = _27 as f64;
match _12.2 {
0 => bb5,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463456454146513819441131 => bb15,
_ => bb14
}
}
bb11 = {
_39 = _5;
_29.0 = _13;
_34.fld2.1 = _12.1;
_30.1 = ((*_9),);
_29.0 = _34.fld2.0;
_15 = _28 as isize;
_42.fld2.1 = _34.fld2.1;
_42 = Adt57 { fld0: _34.fld0,fld1: (*_9),fld2: _12 };
_25 = -_21;
match _42.fld2.2 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb7,
340282366920938463456454146513819441131 => bb10,
_ => bb5
}
}
bb12 = {
_34.fld0 = _19;
(*_7) = _25 as u64;
_23 = _31 ^ _22;
_33 = _3 > _3;
_4 = _23;
_6 = core::ptr::addr_of!((*_7));
_29.0 = [2_usize];
_34.fld0 = 332767921916136209325196402221462707263_u128 as f32;
_4 = !_15;
_5 = '\u{1a2e6}';
_36.0 = core::ptr::addr_of_mut!((*_7));
_27 = _19 + _34.fld0;
_1 = core::ptr::addr_of!(_7);
_30.1 = ((*_7),);
_34.fld1 = (*_7);
_34.fld2.0 = [14293240716714419763_usize];
_16 = (_6,);
(*_9) = _4 as u64;
_18 = _33 as u8;
_28 = _12.2 as u128;
Call(_38.0 = core::intrinsics::bswap((*_7)), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
RET.0 = !(*_6);
_22 = _18 as isize;
_16.0 = core::ptr::addr_of!((*_6));
(*_7) = 1834476174828364586_u64 - 17817078092587731772_u64;
_14.0 = (_2.0,);
_1 = core::ptr::addr_of!(_6);
(*_7) = 13643223676095281850_u64 % 4523029897200152849_u64;
_22 = _4 >> _3;
_22 = _4 << _18;
_23 = _4;
_2 = (_14.0.0,);
(*_9) = !800184108833065926_u64;
Call(_17 = fn8(_12.2, _10, _22, _9, _16, _13, (*_6), _11, _12.1), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_42.fld2.1 = _12.1;
_22 = _21 as isize;
_36.1 = 2_usize * 4429516796919347387_usize;
_25 = _35 - _21;
_41 = _33;
_36.2 = _5;
RET.0 = !_42.fld1;
_11 = _19 as isize;
_7 = _6;
_36.2 = _5;
_45.fld2.1 = _34.fld2.1;
_10 = [_33,_41];
_10 = [_33,_33];
_38 = ((*_6),);
_42.fld2.2 = _27 as i64;
_45.fld0 = _27;
_16 = (_9,);
_4 = _23 & _23;
_43 = _33;
Goto(bb16)
}
bb16 = {
Call(_46 = dump_var(7_usize, 43_usize, Move(_43), 2_usize, Move(_2), 15_usize, Move(_15), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(7_usize, 13_usize, Move(_13), 41_usize, Move(_41), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(7_usize, 4_usize, Move(_4), 18_usize, Move(_18), 47_usize, _47, 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i64,mut _2: [bool; 2],mut _3: isize,mut _4: *const u64,mut _5: (*const u64,),mut _6: [usize; 1],mut _7: u64,mut _8: isize,mut _9: *mut (*const u64, [bool; 2], *const i64, u32)) -> *mut u128 {
mir! {
type RET = *mut u128;
let _10: (*const u64, *mut u128, [bool; 2]);
let _11: i64;
let _12: f64;
let _13: isize;
let _14: Adt60;
let _15: Adt62;
let _16: (i8,);
let _17: i64;
let _18: ([bool; 2],);
let _19: f32;
let _20: ((i8,),);
let _21: u128;
let _22: (char,);
let _23: [u32; 6];
let _24: u128;
let _25: bool;
let _26: isize;
let _27: i8;
let _28: ([bool; 2], *const i32, i128, (isize, *const i32, f32, *const i32), isize, *mut u64);
let _29: usize;
let _30: isize;
let _31: [char; 1];
let _32: (char,);
let _33: ((i8,),);
let _34: ([bool; 2],);
let _35: char;
let _36: [i16; 6];
let _37: isize;
let _38: i8;
let _39: Adt62;
let _40: i8;
let _41: (u64,);
let _42: f64;
let _43: Adt55;
let _44: char;
let _45: f64;
let _46: char;
let _47: i32;
let _48: isize;
let _49: *mut u128;
let _50: isize;
let _51: isize;
let _52: ([usize; 1], *mut (*const u64, [bool; 2], *const i64, u32), i64);
let _53: ((i8,),);
let _54: ();
let _55: ();
{
_4 = (*_9).0;
(*_9).3 = 3156136523_u32;
(*_9).2 = core::ptr::addr_of!(_1);
(*_9).1 = [false,true];
_4 = (*_9).0;
_5 = ((*_9).0,);
_1 = (-4839374243213431712_i64);
_5.0 = core::ptr::addr_of!(_7);
_13 = -_3;
_12 = 9234_i16 as f64;
_13 = -_3;
(*_9).3 = 21657_i16 as u32;
_15.fld6.fld1.1 = ((*_4),);
(*_9).3 = 2511382515_u32;
_15.fld1 = '\u{ad110}';
_14.fld3.0.0 = !101_i8;
_9 = core::ptr::addr_of_mut!((*_9));
(*_9).3 = 1884480421_u32 % 949742777_u32;
_14.fld1 = [(-4237_i16),(-3077_i16),(-16442_i16),10332_i16,2952_i16,(-13760_i16)];
_15.fld1 = '\u{a794f}';
Goto(bb1)
}
bb1 = {
_15.fld0 = (-64922158440840216921796614102142688589_i128) as u64;
_14.fld3.0.0 = (*_9).3 as i8;
_14.fld7.0 = core::ptr::addr_of!(_15.fld6.fld1.1.0);
_10.1 = core::ptr::addr_of_mut!(_15.fld5);
_15.fld6.fld3.2 = 46475_u16 as i128;
(*_9).1 = [true,false];
_15.fld6.fld3.4 = _8;
_7 = (*_4) * _15.fld6.fld1.1.0;
_1 = (*_4) as i64;
_14.fld0 = 4562096952606568128_usize as u64;
(*_9).0 = core::ptr::addr_of!((*_4));
_4 = core::ptr::addr_of!(_15.fld0);
_15.fld6.fld1.1 = ((*_4),);
_6 = [7819544747691682066_usize];
_15.fld2 = _15.fld6.fld3.2;
_15.fld6.fld3.0 = (*_9).1;
Goto(bb2)
}
bb2 = {
_10.1 = core::ptr::addr_of_mut!(_15.fld5);
_15.fld6.fld3.3.0 = _13 + _13;
_15.fld6.fld3.3.2 = 5481472827223282720_usize as f32;
_15.fld1 = '\u{9389c}';
RET = core::ptr::addr_of_mut!(_15.fld5);
_10.2 = (*_9).1;
_10.0 = (*_9).0;
_11 = _1;
_15.fld6.fld1.0 = [_15.fld6.fld3.3.0,_15.fld6.fld3.4,_15.fld6.fld3.3.0,_15.fld6.fld3.3.0,_13,_15.fld6.fld3.3.0,_8,_3];
_15.fld6.fld3.4 = _15.fld6.fld3.3.0;
_14.fld4 = [4704587334512740059_usize];
Goto(bb3)
}
bb3 = {
(*RET) = 253695794328928105239778179804133331963_u128;
_15.fld6.fld2 = 3923_i16 + (-14941_i16);
_16 = (_14.fld3.0.0,);
_15.fld2 = _12 as i128;
_15.fld6.fld3.5 = core::ptr::addr_of_mut!((*_4));
_10.0 = core::ptr::addr_of!((*_4));
_4 = _5.0;
(*_9).0 = _5.0;
_15.fld6.fld3.3.0 = _15.fld6.fld3.4 - _15.fld6.fld3.4;
Goto(bb4)
}
bb4 = {
_15.fld6.fld3.4 = _3;
_14.fld3.0.0 = _12 as i8;
_4 = core::ptr::addr_of!(_14.fld0);
_15.fld0 = _14.fld0 << (*_9).3;
_15.fld6.fld0 = _14.fld7;
(*_9).3 = 3473328733_u32 * 1770716289_u32;
(*RET) = 291134464800746536440639043266069984404_u128 << _14.fld3.0.0;
_18 = (_2,);
_14.fld7 = (_4,);
_9 = core::ptr::addr_of_mut!((*_9));
_6 = [2461804944175886280_usize];
(*_9).1 = _10.2;
_15.fld4 = 10475_u16 as i16;
(*_9).0 = core::ptr::addr_of!(_14.fld0);
_17 = _12 as i64;
_14.fld6 = (*RET) as i64;
_20.0.0 = !_16.0;
_3 = !_8;
_3 = _15.fld6.fld3.3.0 & _15.fld6.fld3.3.0;
(*_9).0 = core::ptr::addr_of!((*_4));
_3 = -_15.fld6.fld3.3.0;
_16.0 = -_20.0.0;
(*RET) = !196909765396367078044426574819909248210_u128;
_14.fld2 = _3;
_15.fld6.fld2 = _15.fld4;
_15.fld6.fld3.2 = !_15.fld2;
Call(_7 = fn9(_6, (*_9).0, _5.0, _14.fld3.0, (*_9).2, _14.fld4, _15.fld1, _14.fld0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_22.0 = _15.fld1;
_14.fld0 = !_15.fld0;
_14.fld2 = 42006_u16 as isize;
_6 = _14.fld4;
_13 = _15.fld6.fld3.4;
_21 = (*RET);
_22 = (_15.fld1,);
_14.fld2 = -_15.fld6.fld3.4;
_10.1 = core::ptr::addr_of_mut!((*RET));
_14.fld2 = _15.fld6.fld3.3.0;
_15.fld6.fld3.3.2 = _13 as f32;
_15.fld3 = _11 as i8;
Call(_14.fld2 = core::intrinsics::transmute(_8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_15.fld6.fld3.3.2 = _3 as f32;
_19 = _15.fld6.fld3.3.2;
_9 = core::ptr::addr_of_mut!((*_9));
_15.fld6.fld1.1 = ((*_4),);
Goto(bb7)
}
bb7 = {
_2 = [false,false];
_2 = [true,false];
_4 = core::ptr::addr_of!((*_4));
(*_9).0 = core::ptr::addr_of!(_7);
_10.2 = _18.0;
_14.fld3 = (_20.0,);
_23 = [(*_9).3,(*_9).3,(*_9).3,(*_9).3,(*_9).3,(*_9).3];
_5.0 = core::ptr::addr_of!(_14.fld0);
(*_4) = _7;
_11 = _1 >> _3;
_15.fld0 = !_14.fld0;
Goto(bb8)
}
bb8 = {
_15.fld0 = (*_4) * _14.fld0;
_14.fld2 = _3;
_26 = _3;
(*_9).1 = _2;
_6 = _14.fld4;
(*RET) = _21;
_14.fld3.0.0 = !_16.0;
_20.0 = (_14.fld3.0.0,);
_14.fld7 = ((*_9).0,);
_11 = _14.fld6 ^ _14.fld6;
_15.fld6.fld3.2 = _15.fld2;
_10.2 = [true,true];
(*RET) = !_21;
_20 = _14.fld3;
(*_4) = _15.fld0;
_15.fld2 = _12 as i128;
_15.fld6.fld3.3.0 = -_14.fld2;
_15.fld6.fld2 = -_15.fld4;
_14.fld7.0 = core::ptr::addr_of!(_15.fld0);
(*_9).1 = [false,true];
Goto(bb9)
}
bb9 = {
_15.fld5 = _21 / 56912899244832243038769604376854851579_u128;
_14.fld3 = (_20.0,);
_12 = 2009563900_i32 as f64;
(*_9).3 = !2132013080_u32;
_17 = _14.fld6;
_10.1 = core::ptr::addr_of_mut!(_24);
_15.fld6.fld2 = _13 as i16;
(*_4) = !_15.fld6.fld1.1.0;
_1 = _12 as i64;
(*RET) = !_21;
_15.fld0 = _15.fld6.fld2 as u64;
_14.fld6 = _11 - _17;
_20.0.0 = _14.fld3.0.0 ^ _16.0;
_13 = _14.fld2 | _26;
_22.0 = _15.fld1;
_15.fld6.fld3.3.0 = _20.0.0 as isize;
_28.3.0 = !_26;
RET = core::ptr::addr_of_mut!(_24);
_20.0 = (_14.fld3.0.0,);
RET = core::ptr::addr_of_mut!((*RET));
_25 = _13 < _13;
_14.fld0 = _15.fld6.fld1.1.0 ^ _15.fld0;
_14.fld5 = [_22.0];
_15.fld6.fld1.1.0 = (*_4);
_3 = _13 & _26;
_28.3.0 = !_13;
_30 = !_3;
Call(_13 = fn17(_7, (*_9), _15.fld2, _15.fld5, Move(_14), _19, (*_9).0, _15.fld3, _15.fld6.fld3.0, _10.1, _15.fld0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_18 = ((*_9).1,);
_27 = _15.fld3;
_5.0 = core::ptr::addr_of!(_7);
_15.fld6.fld3.5 = core::ptr::addr_of_mut!(_15.fld0);
_1 = _11;
Goto(bb11)
}
bb11 = {
_28.3.2 = _19;
_29 = !18088805944622827001_usize;
_28.2 = _15.fld6.fld3.2;
_12 = 161_u8 as f64;
_8 = _30 | _28.3.0;
_11 = _15.fld6.fld1.1.0 as i64;
(*_9).3 = _11 as u32;
_28.4 = _13 >> _28.3.0;
_31 = [_22.0];
_22 = (_15.fld1,);
(*_9).1 = [_25,_25];
_15.fld6.fld3.0 = [_25,_25];
_28.5 = _15.fld6.fld3.5;
_5.0 = core::ptr::addr_of!(_15.fld0);
_23 = [(*_9).3,(*_9).3,(*_9).3,(*_9).3,(*_9).3,(*_9).3];
_33 = _20;
_15.fld1 = _22.0;
_16 = _33.0;
_22 = (_15.fld1,);
(*_9).3 = !2938609526_u32;
_6 = [_29];
_29 = 31031_u16 as usize;
_7 = _15.fld1 as u64;
_34.0 = [_25,_25];
(*RET) = _21 / 208922258253140075209958342930037914844_u128;
Goto(bb12)
}
bb12 = {
_27 = _16.0 * _16.0;
_15.fld6.fld3.2 = _28.2 & _15.fld2;
_15.fld5 = _15.fld6.fld3.3.2 as u128;
_39.fld1 = _15.fld1;
_39.fld6.fld3.0 = [_25,_25];
_39.fld6.fld0 = _5;
_38 = _16.0;
_29 = 2_usize;
_15.fld4 = _25 as i16;
Goto(bb13)
}
bb13 = {
_39.fld6.fld1.0[_29] = _8 | _28.3.0;
_39.fld6.fld0 = (_10.0,);
_41 = _15.fld6.fld1.1;
_3 = _15.fld6.fld3.3.0;
_9 = core::ptr::addr_of_mut!((*_9));
_39.fld6.fld3.1 = core::ptr::addr_of!(_47);
_18.0 = [_25,_25];
_39.fld6.fld3.5 = core::ptr::addr_of_mut!(_15.fld6.fld1.1.0);
_34.0 = [_25,_25];
_2 = [_25,_25];
_39.fld3 = _27;
match _29 {
0 => bb1,
1 => bb11,
3 => bb9,
2 => bb14,
_ => bb5
}
}
bb14 = {
_39.fld6.fld1.0 = [_30,_28.3.0,_28.3.0,_13,_26,_28.3.0,_28.3.0,_15.fld6.fld1.0[_29]];
_12 = _39.fld6.fld1.0[_29] as f64;
_36 = [_15.fld4,_15.fld4,_15.fld4,_15.fld4,_15.fld4,_15.fld4];
_43.fld3.3 = !_23[_29];
_39.fld6.fld3.3.0 = !_8;
_28.1 = core::ptr::addr_of!(_47);
(*_9).2 = core::ptr::addr_of!(_1);
_43.fld1 = core::ptr::addr_of_mut!((*_9));
_43.fld5.2 = _29 as i128;
_39.fld4 = _15.fld4;
_34 = _18;
_28.3.1 = core::ptr::addr_of!(_47);
_43.fld7.fld1 = (_41.0,);
(*_9).0 = core::ptr::addr_of!(_15.fld0);
_30 = _29 as isize;
_23 = [_43.fld3.3,_43.fld3.3,_43.fld3.3,_43.fld3.3,_43.fld3.3,_43.fld3.3];
_38 = _39.fld3;
_15.fld1 = _39.fld1;
_52.1 = core::ptr::addr_of_mut!(_43.fld3);
_35 = _39.fld1;
_53 = _33;
_43.fld3.0 = _4;
_47 = (-1477037488_i32) + (-1870393353_i32);
_10.1 = core::ptr::addr_of_mut!(_15.fld5);
_23[_29] = _15.fld6.fld3.3.2 as u32;
_39.fld6.fld1.0[_29] = _28.3.0;
(*_9).1 = [_25,_25];
Goto(bb15)
}
bb15 = {
Call(_54 = dump_var(8_usize, 24_usize, Move(_24), 25_usize, Move(_25), 21_usize, Move(_21), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_54 = dump_var(8_usize, 29_usize, Move(_29), 35_usize, Move(_35), 16_usize, Move(_16), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_54 = dump_var(8_usize, 20_usize, Move(_20), 1_usize, Move(_1), 2_usize, Move(_2), 38_usize, Move(_38)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_54 = dump_var(8_usize, 13_usize, Move(_13), 34_usize, Move(_34), 18_usize, Move(_18), 55_usize, _55), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [usize; 1],mut _2: *const u64,mut _3: *const u64,mut _4: (i8,),mut _5: *const i64,mut _6: [usize; 1],mut _7: char,mut _8: u64) -> u64 {
mir! {
type RET = u64;
let _9: Adt59;
let _10: u32;
let _11: [isize; 8];
let _12: ((i8,),);
let _13: char;
let _14: f32;
let _15: isize;
let _16: char;
let _17: i32;
let _18: i64;
let _19: *const u64;
let _20: u128;
let _21: (char,);
let _22: Adt52;
let _23: Adt50;
let _24: [isize; 8];
let _25: isize;
let _26: ([usize; 1], *mut (*const u64, [bool; 2], *const i64, u32), i64);
let _27: i32;
let _28: char;
let _29: i8;
let _30: ();
let _31: ();
{
_6 = [1_usize];
_3 = core::ptr::addr_of!(_8);
_3 = _2;
(*_2) = _8 * _8;
_5 = core::ptr::addr_of!((*_5));
(*_3) = _8;
_7 = '\u{1d65a}';
RET = (*_3);
(*_3) = _8 / 10916201923500217303_u64;
Call(_4.0 = fn10(_1, (*_5), _1, (*_5), (*_3), (*_3), (*_5), (*_3), (*_3), _5, (*_3), _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = [17726610080000365604_usize];
_4 = (118_i8,);
_8 = !(*_3);
_2 = _3;
_8 = (*_2);
_9.fld2.2 = 157379129381690303622867371603166642238_i128 - 21840925454042084810264067779485882442_i128;
_9.fld2.4 = (-72_isize);
_9.fld2.5 = core::ptr::addr_of_mut!((*_3));
_9.fld2.0 = [true,false];
_9.fld4.0 = [_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4];
(*_5) = true as i64;
_5 = core::ptr::addr_of!(_9.fld1);
(*_3) = _8 - _8;
_1 = [2_usize];
_9.fld3.fld1 = (_3,);
_9.fld2.3.2 = 50842_u16 as f32;
_9.fld4.1 = ((*_2),);
(*_5) = 6774113878839906435_i64 & 3968259282800351737_i64;
_9.fld3.fld2 = core::ptr::addr_of!((*_3));
_5 = core::ptr::addr_of!((*_5));
match _4.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
118 => bb7,
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
_9.fld3.fld3 = [true,false,true,false,false];
_11 = _9.fld4.0;
(*_3) = !_8;
_9.fld2.3.0 = _4.0 as isize;
_9.fld4.0 = [_9.fld2.4,_9.fld2.3.0,_9.fld2.3.0,_9.fld2.3.0,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4];
_9.fld0.2 = _9.fld2.3.2;
_12 = (_4,);
_9.fld3.fld0 = (_12.0.0,);
_12.0 = _4;
_10 = !3520113258_u32;
_10 = 1379105196_u32 & 161701682_u32;
_9.fld4.1.0 = (*_3);
_9.fld4.1.0 = (*_2);
(*_3) = _8 / 5233453692521070621_u64;
_9.fld3.fld3 = [true,true,true,false,true];
_11 = [_9.fld2.3.0,_9.fld2.4,_9.fld2.3.0,_9.fld2.3.0,_9.fld2.3.0,_9.fld2.3.0,_9.fld2.4,_9.fld2.3.0];
_9.fld0.0 = _10 as isize;
Call(_9.fld3.fld0.0 = fn16(_7, _7, _4.0, _9.fld4.1, _9.fld2.4, _9.fld4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13 = _7;
_9.fld0.2 = -_9.fld2.3.2;
_9.fld0.2 = -_9.fld2.3.2;
_9.fld2.4 = _9.fld0.0 | _9.fld2.3.0;
_9.fld2.3.2 = _9.fld0.2;
_9.fld3.fld3 = [false,false,true,false,false];
match _12.0.0 {
0 => bb2,
118 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_15 = _9.fld2.4;
_9.fld4.1 = ((*_3),);
_12 = (_9.fld3.fld0,);
_9.fld3.fld3 = [true,true,false,false,false];
Goto(bb11)
}
bb11 = {
_9.fld3.fld3 = [false,false,false,false,false];
_16 = _7;
_9.fld0.1 = core::ptr::addr_of!(_17);
_15 = false as isize;
_4.0 = _9.fld3.fld0.0;
_9.fld2.3.0 = 2_usize as isize;
(*_2) = _9.fld4.1.0;
_10 = 1614396564_u32 % 967216437_u32;
_9.fld3.fld1.0 = core::ptr::addr_of!((*_3));
_9.fld2.3.0 = !_9.fld2.4;
_17 = (-1209544882_i32);
_9.fld0.3 = _9.fld0.1;
(*_2) = 86_i16 as u64;
_20 = 290894319804056751004273898167816347769_u128 % 72176879394269918900122942019867856576_u128;
(*_5) = -(-1498560127264835775_i64);
_14 = _9.fld0.2;
_22.fld0 = [_16];
_13 = _16;
_23.fld1 = (_9.fld3.fld0.0,);
_9.fld2.3.2 = _8 as f32;
_9.fld2.3.3 = core::ptr::addr_of!(_17);
_10 = !1727276593_u32;
_9.fld2.5 = core::ptr::addr_of_mut!(_8);
match _17 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb12,
4 => bb13,
340282366920938463463374607430558666574 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
_1 = [17726610080000365604_usize];
_4 = (118_i8,);
_8 = !(*_3);
_2 = _3;
_8 = (*_2);
_9.fld2.2 = 157379129381690303622867371603166642238_i128 - 21840925454042084810264067779485882442_i128;
_9.fld2.4 = (-72_isize);
_9.fld2.5 = core::ptr::addr_of_mut!((*_3));
_9.fld2.0 = [true,false];
_9.fld4.0 = [_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4,_9.fld2.4];
(*_5) = true as i64;
_5 = core::ptr::addr_of!(_9.fld1);
(*_3) = _8 - _8;
_1 = [2_usize];
_9.fld3.fld1 = (_3,);
_9.fld2.3.2 = 50842_u16 as f32;
_9.fld4.1 = ((*_2),);
(*_5) = 6774113878839906435_i64 & 3968259282800351737_i64;
_9.fld3.fld2 = core::ptr::addr_of!((*_3));
_5 = core::ptr::addr_of!((*_5));
match _4.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
118 => bb7,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
_9.fld2.3.0 = _14 as isize;
(*_2) = _20 as u64;
RET = _8;
(*_3) = !_8;
_4.0 = 145_u8 as i8;
_9.fld2.3.1 = _9.fld0.1;
_9.fld1 = !5218021611918948608_i64;
_9.fld1 = (-4860985726558958849_i64) ^ (-5471911716357395869_i64);
_18 = 21_u8 as i64;
_28 = _7;
RET = _20 as u64;
_23.fld0 = (_7,);
_4.0 = !_23.fld1.0;
_22.fld0 = [_28];
_9.fld4.1.0 = (*_2) >> (*_3);
_9.fld0.2 = _9.fld2.3.2 + _9.fld2.3.2;
Goto(bb16)
}
bb16 = {
Call(_30 = dump_var(9_usize, 15_usize, Move(_15), 7_usize, Move(_7), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(9_usize, 16_usize, Move(_16), 10_usize, Move(_10), 20_usize, Move(_20), 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [usize; 1],mut _2: i64,mut _3: [usize; 1],mut _4: i64,mut _5: u64,mut _6: u64,mut _7: i64,mut _8: u64,mut _9: u64,mut _10: *const i64,mut _11: u64,mut _12: *const u64) -> i8 {
mir! {
type RET = i8;
let _13: ([bool; 2],);
let _14: i64;
let _15: i128;
let _16: i64;
let _17: [usize; 1];
let _18: *const i64;
let _19: u8;
let _20: usize;
let _21: char;
let _22: [i16; 6];
let _23: bool;
let _24: isize;
let _25: [bool; 5];
let _26: Adt55;
let _27: f32;
let _28: (char,);
let _29: isize;
let _30: Adt57;
let _31: isize;
let _32: (u64,);
let _33: [bool; 2];
let _34: [u32; 6];
let _35: [i16; 6];
let _36: char;
let _37: *mut u128;
let _38: isize;
let _39: ([bool; 2],);
let _40: ();
let _41: ();
{
_4 = (-26409_i16) as i64;
_4 = !_2;
_6 = (*_10) as u64;
_7 = (*_10);
_2 = -(*_10);
_12 = core::ptr::addr_of!((*_12));
Goto(bb1)
}
bb1 = {
(*_10) = _7;
_8 = true as u64;
(*_10) = _8 as i64;
_7 = _4 - (*_10);
_10 = core::ptr::addr_of!(_2);
RET = (-97_i8);
_13.0 = [false,true];
_14 = !_2;
_6 = !_11;
Call((*_10) = fn11((*_12), _3, _5, _11, _12, _4, _3, _6, _10, _11, _10, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = !_11;
_15 = 109794487823342782429807185818000915425_i128;
RET = (-77_i8) >> _14;
_5 = _6 - _9;
(*_12) = _5 - _9;
(*_10) = true as i64;
RET = !78_i8;
_16 = _7;
_2 = _16;
(*_10) = _16;
_2 = _16;
_11 = _4 as u64;
_5 = _6 - _11;
_12 = core::ptr::addr_of!(_6);
Call(_3 = fn12((*_12), _16, _13, _6, _8, _9, _2, (*_10), _13, _12, (*_12), _2, _7, _15, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17 = _3;
(*_10) = -_14;
Goto(bb4)
}
bb4 = {
_6 = _5;
_19 = 251_u8;
_15 = (-31356917967628790772298416172916999949_i128) & (-144047640485613207821733607901752520274_i128);
_6 = _15 as u64;
_18 = core::ptr::addr_of!(_16);
_3 = _17;
(*_12) = _11;
_13.0 = [true,true];
_15 = 31356026212851089874964968755093847230_i128;
Goto(bb5)
}
bb5 = {
_22 = [(-15790_i16),(-22214_i16),4928_i16,(-25180_i16),(-7091_i16),12194_i16];
_22 = [(-12932_i16),(-17819_i16),(-2451_i16),(-1096_i16),6660_i16,(-14243_i16)];
_14 = _7 + (*_18);
_15 = 63496483437343606890248383712406187473_i128;
_7 = -_16;
_9 = _5;
_20 = _19 as usize;
_16 = _14 + _2;
_10 = core::ptr::addr_of!((*_10));
_20 = 53282_u16 as usize;
_16 = true as i64;
_15 = _20 as i128;
_18 = core::ptr::addr_of!(_4);
RET = 306095066769996422174029208353496260769_u128 as i8;
(*_12) = _5 - _8;
_11 = !(*_12);
_2 = (-81_isize) as i64;
match _19 {
0 => bb1,
1 => bb3,
251 => bb7,
_ => bb6
}
}
bb6 = {
_17 = _3;
(*_10) = -_14;
Goto(bb4)
}
bb7 = {
(*_18) = -(*_10);
_13.0 = [false,false];
RET = 96_i8 | 110_i8;
_1 = [_20];
_4 = !_7;
_14 = 84_i8 as i64;
RET = 35_i8;
_25 = [true,false,true,true,true];
_10 = _18;
_9 = !_6;
RET = _20 as i8;
_26.fld5.3.0 = -50_isize;
match _19 {
0 => bb8,
251 => bb10,
_ => bb9
}
}
bb8 = {
_17 = _3;
(*_10) = -_14;
Goto(bb4)
}
bb9 = {
_17 = _3;
(*_10) = -_14;
Goto(bb4)
}
bb10 = {
_23 = !true;
_26.fld5.0 = _13.0;
_21 = '\u{d695c}';
_1 = _3;
Goto(bb11)
}
bb11 = {
_28.0 = _21;
_26.fld5.4 = _26.fld5.3.0 + _26.fld5.3.0;
_26.fld7.fld1.0 = (*_12);
_22 = [26228_i16,11030_i16,10596_i16,17440_i16,(-18920_i16),14710_i16];
_2 = !(*_18);
_26.fld3.0 = _12;
_2 = _7 >> (*_10);
_16 = 12196_i16 as i64;
_26.fld1 = core::ptr::addr_of_mut!(_26.fld3);
RET = _20 as i8;
_13 = (_26.fld5.0,);
_30.fld2 = (_3, _26.fld1, (*_10));
_26.fld3 = (_12, _26.fld5.0, _10, 2157049273_u32);
match _26.fld3.3 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
2157049273 => bb18,
_ => bb17
}
}
bb12 = {
_23 = !true;
_26.fld5.0 = _13.0;
_21 = '\u{d695c}';
_1 = _3;
Goto(bb11)
}
bb13 = {
_17 = _3;
(*_10) = -_14;
Goto(bb4)
}
bb14 = {
_17 = _3;
(*_10) = -_14;
Goto(bb4)
}
bb15 = {
_17 = _3;
(*_10) = -_14;
Goto(bb4)
}
bb16 = {
_6 = _5;
_19 = 251_u8;
_15 = (-31356917967628790772298416172916999949_i128) & (-144047640485613207821733607901752520274_i128);
_6 = _15 as u64;
_18 = core::ptr::addr_of!(_16);
_3 = _17;
(*_12) = _11;
_13.0 = [true,true];
_15 = 31356026212851089874964968755093847230_i128;
Goto(bb5)
}
bb17 = {
(*_10) = _7;
_8 = true as u64;
(*_10) = _8 as i64;
_7 = _4 - (*_10);
_10 = core::ptr::addr_of!(_2);
RET = (-97_i8);
_13.0 = [false,true];
_14 = !_2;
_6 = !_11;
Call((*_10) = fn11((*_12), _3, _5, _11, _12, _4, _3, _6, _10, _11, _10, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_26.fld5.3.2 = 94581337814527873513052988139855352472_u128 as f32;
RET = (-18_i8) * (-115_i8);
_26.fld3.1 = [_23,_23];
(*_18) = _16;
_28 = (_21,);
_6 = _28.0 as u64;
_26.fld5.5 = core::ptr::addr_of_mut!((*_12));
_2 = -_7;
_13 = (_26.fld5.0,);
_4 = _14 ^ _7;
_20 = 10037656649609079485_usize;
_30.fld2 = (_1, _26.fld1, (*_10));
_9 = _11;
_14 = -(*_18);
_26.fld3 = (_12, _13.0, _18, 3329102000_u32);
_26.fld4 = [_26.fld3.3,_26.fld3.3,_26.fld3.3,_26.fld3.3,_26.fld3.3,_26.fld3.3];
_1 = _30.fld2.0;
_26.fld2 = [_14,(*_18),(*_10),_30.fld2.2];
Goto(bb19)
}
bb19 = {
Call(_40 = dump_var(10_usize, 17_usize, Move(_17), 16_usize, Move(_16), 23_usize, Move(_23), 3_usize, Move(_3)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_40 = dump_var(10_usize, 9_usize, Move(_9), 19_usize, Move(_19), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_40 = dump_var(10_usize, 8_usize, Move(_8), 1_usize, Move(_1), 11_usize, Move(_11), 41_usize, _41), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: u64,mut _2: [usize; 1],mut _3: u64,mut _4: u64,mut _5: *const u64,mut _6: i64,mut _7: [usize; 1],mut _8: u64,mut _9: *const i64,mut _10: u64,mut _11: *const i64,mut _12: u64) -> i64 {
mir! {
type RET = i64;
let _13: [char; 1];
let _14: [i64; 1];
let _15: f32;
let _16: u32;
let _17: (i8,);
let _18: char;
let _19: [usize; 1];
let _20: char;
let _21: *const i64;
let _22: i16;
let _23: i8;
let _24: *const i64;
let _25: bool;
let _26: *const i32;
let _27: ((i8,),);
let _28: Adt57;
let _29: (u64,);
let _30: u128;
let _31: ([isize; 8], (u64,));
let _32: Adt54;
let _33: bool;
let _34: f64;
let _35: [bool; 5];
let _36: f64;
let _37: u16;
let _38: ();
let _39: ();
{
_10 = (*_5) & _4;
_9 = core::ptr::addr_of!(_6);
(*_5) = !_8;
_12 = !_3;
Goto(bb1)
}
bb1 = {
_8 = 68219160256174848280154958539200727523_u128 as u64;
_3 = 9223372036854775807_isize as u64;
RET = (*_9);
_12 = !(*_5);
(*_9) = !(-5598645321401333769_i64);
_10 = !(*_5);
_11 = _9;
(*_11) = (-1149417679386509014_i64) | (-4703211862512061069_i64);
(*_5) = !_3;
_10 = (-681043463_i32) as u64;
(*_5) = !_4;
_16 = !2316702803_u32;
(*_9) = -(-5518704052745442984_i64);
_14 = [(*_11)];
_5 = core::ptr::addr_of!((*_5));
_13 = ['\u{d2d2}'];
_9 = _11;
_16 = 88_i8 as u32;
_7 = _2;
_17 = ((-19_i8),);
match _17.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211437 => bb7,
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
_1 = !(*_5);
_19 = _2;
match _17.0 {
0 => bb1,
1 => bb2,
2 => bb5,
340282366920938463463374607431768211437 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
(*_5) = _8;
_22 = 43938_u16 as i16;
_21 = core::ptr::addr_of!((*_9));
_24 = core::ptr::addr_of!((*_9));
_4 = _8;
_12 = (*_5);
_2 = [639046160152644275_usize];
(*_11) = 162444270599760022481509557718874480045_u128 as i64;
RET = (*_21) * (*_24);
_18 = '\u{eba83}';
_20 = _18;
_5 = core::ptr::addr_of!(_12);
_13 = [_20];
(*_9) = 3723431027869364094_i64 << _1;
_3 = (*_5);
(*_5) = !_8;
_1 = _3 + _4;
_17 = ((-102_i8),);
_16 = 3410302082_u32 - 1572379077_u32;
(*_9) = (-5127941426106308618_i64) << _8;
_16 = (-846387552_i32) as u32;
(*_11) = !7387134251691977970_i64;
_18 = _20;
_8 = _1;
_15 = _16 as f32;
_16 = 2338353979_u32 | 2986635478_u32;
match _17.0 {
340282366920938463463374607431768211354 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_4 = _1;
_17.0 = (-10_i8);
_18 = _20;
_13 = [_20];
_20 = _18;
_19 = _2;
_28.fld2.2 = -(*_11);
_25 = (*_11) < (*_21);
_10 = !(*_5);
_29 = (_4,);
(*_5) = (*_9) as u64;
_27.0 = (_17.0,);
_6 = _28.fld2.2;
_28.fld2.0 = [7_usize];
_31.1.0 = _1;
_32.fld1.0 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_32.fld3.2 = -153871169655467688373320302190240664181_i128;
_32.fld3.5 = core::ptr::addr_of_mut!((*_5));
_9 = _24;
_31.1 = _29;
_25 = true;
_32.fld3.3.0 = 9223372036854775807_isize & (-9223372036854775808_isize);
match _27.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb9,
340282366920938463463374607431768211446 => bb12,
_ => bb10
}
}
bb12 = {
_14 = [(*_9)];
_3 = _29.0;
_1 = !_8;
_31.0 = [_32.fld3.3.0,_32.fld3.3.0,_32.fld3.3.0,_32.fld3.3.0,_32.fld3.3.0,_32.fld3.3.0,_32.fld3.3.0,_32.fld3.3.0];
_2 = [17918435925318055116_usize];
_29.0 = _3 + _8;
_32.fld0.0 = _5;
match _17.0 {
0 => bb9,
1 => bb6,
2 => bb4,
3 => bb13,
340282366920938463463374607431768211446 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_22 = (-4824_i16);
_3 = !(*_5);
_32.fld3.2 = 98990222268668452313970307249879668403_i128 * (-31585219157072323653379990966465952312_i128);
(*_24) = _28.fld2.2 | _28.fld2.2;
_23 = _27.0.0;
_33 = _25;
_32.fld1.0 = _31.0;
_32.fld1.0 = _31.0;
_18 = _20;
_32.fld3.2 = 94679541767337853430112980734274697285_i128;
_8 = _32.fld3.3.0 as u64;
_32.fld1.1 = (_8,);
_9 = _11;
(*_11) = 25276_u16 as i64;
_32.fld0.0 = core::ptr::addr_of!(_12);
_29 = _31.1;
_17.0 = (-1548142925_i32) as i8;
_4 = (*_11) as u64;
_35 = [_33,_25,_33,_33,_33];
RET = (*_11) - (*_9);
_32.fld1 = (_31.0, _29);
_37 = 0_usize as u16;
(*_9) = _28.fld2.2 << _32.fld3.3.0;
(*_24) = _28.fld2.2 * _28.fld2.2;
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(11_usize, 2_usize, Move(_2), 19_usize, Move(_19), 27_usize, Move(_27), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(11_usize, 17_usize, Move(_17), 3_usize, Move(_3), 31_usize, Move(_31), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(11_usize, 23_usize, Move(_23), 33_usize, Move(_33), 8_usize, Move(_8), 37_usize, Move(_37)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u64,mut _2: i64,mut _3: ([bool; 2],),mut _4: u64,mut _5: u64,mut _6: u64,mut _7: i64,mut _8: i64,mut _9: ([bool; 2],),mut _10: *const u64,mut _11: u64,mut _12: i64,mut _13: i64,mut _14: i128,mut _15: u64) -> [usize; 1] {
mir! {
type RET = [usize; 1];
let _16: f64;
let _17: char;
let _18: (char,);
let _19: u32;
let _20: i16;
let _21: isize;
let _22: i16;
let _23: [i64; 1];
let _24: isize;
let _25: Adt50;
let _26: isize;
let _27: *mut u128;
let _28: ();
let _29: ();
{
_8 = _7;
_14 = 58139906486070904123634809073507315703_i128;
_3 = (_9.0,);
_10 = core::ptr::addr_of!(_5);
_5 = _14 as u64;
_19 = (-9223372036854775808_isize) as u32;
_18 = ('\u{3ba95}',);
_15 = _11;
_11 = _1;
_13 = _7;
_13 = _1 as i64;
match _14 {
0 => bb1,
1 => bb2,
58139906486070904123634809073507315703 => bb4,
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
_3.0 = _9.0;
_18 = ('\u{57cdb}',);
_6 = _14 as u64;
_2 = 1556582600_i32 as i64;
_3 = (_9.0,);
_5 = _18.0 as u64;
_12 = -_8;
_10 = core::ptr::addr_of!((*_10));
_11 = !(*_10);
_4 = (*_10);
_19 = 2713869250_u32 + 2416152869_u32;
_11 = _14 as u64;
_10 = core::ptr::addr_of!(_1);
_14 = 34388625497690691454545793028792313440_u128 as i128;
Call(_17 = fn13(_18.0, _9, _15, (*_10), _3, _15, _18.0, _5, (*_10), (*_10), _9, _1, _3.0, _18.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = (_3.0,);
_6 = _4 << _7;
RET = [2_usize];
_5 = _6 / 5847000411139516080_u64;
_9.0 = [false,true];
_10 = core::ptr::addr_of!(_5);
RET = [3649860195974669534_usize];
_5 = _6;
_12 = _7 & _13;
_23 = [_7];
_10 = core::ptr::addr_of!(_15);
_2 = -_12;
_3 = _9;
_24 = 9223372036854775807_isize;
_16 = 17_u8 as f64;
_25.fld1 = ((-3_i8),);
_9.0 = [false,false];
_25.fld1.0 = -(-94_i8);
Call(_3.0 = fn14(_5, _10, _1, _10, _9, _9.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3.0 = [true,false];
_1 = _5 - _6;
_2 = _12 * _8;
_25.fld0 = (_18.0,);
(*_10) = !_1;
Goto(bb7)
}
bb7 = {
_21 = _24;
_18 = _25.fld0;
(*_10) = !_6;
_20 = (-13090_i16) ^ 11108_i16;
_16 = (-1458699335_i32) as f64;
_3 = (_9.0,);
_1 = !_15;
_25.fld1 = ((-101_i8),);
_24 = _20 as isize;
match _25.fld1.0 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607431768211355 => bb12,
_ => bb11
}
}
bb8 = {
_3.0 = [true,false];
_1 = _5 - _6;
_2 = _12 * _8;
_25.fld0 = (_18.0,);
(*_10) = !_1;
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
_3.0 = _9.0;
_18 = ('\u{57cdb}',);
_6 = _14 as u64;
_2 = 1556582600_i32 as i64;
_3 = (_9.0,);
_5 = _18.0 as u64;
_12 = -_8;
_10 = core::ptr::addr_of!((*_10));
_11 = !(*_10);
_4 = (*_10);
_19 = 2713869250_u32 + 2416152869_u32;
_11 = _14 as u64;
_10 = core::ptr::addr_of!(_1);
_14 = 34388625497690691454545793028792313440_u128 as i128;
Call(_17 = fn13(_18.0, _9, _15, (*_10), _3, _15, _18.0, _5, (*_10), (*_10), _9, _1, _3.0, _18.0), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
RET = [13685920916347925076_usize];
_24 = _21;
match _24 {
0 => bb11,
1 => bb13,
2 => bb14,
9223372036854775807 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_9 = (_3.0,);
_6 = _4 << _7;
RET = [2_usize];
_5 = _6 / 5847000411139516080_u64;
_9.0 = [false,true];
_10 = core::ptr::addr_of!(_5);
RET = [3649860195974669534_usize];
_5 = _6;
_12 = _7 & _13;
_23 = [_7];
_10 = core::ptr::addr_of!(_15);
_2 = -_12;
_3 = _9;
_24 = 9223372036854775807_isize;
_16 = 17_u8 as f64;
_25.fld1 = ((-3_i8),);
_9.0 = [false,false];
_25.fld1.0 = -(-94_i8);
Call(_3.0 = fn14(_5, _10, _1, _10, _9, _9.0), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_21 = _24;
_18 = _25.fld0;
(*_10) = !_6;
_20 = (-13090_i16) ^ 11108_i16;
_16 = (-1458699335_i32) as f64;
_3 = (_9.0,);
_1 = !_15;
_25.fld1 = ((-101_i8),);
_24 = _20 as isize;
match _25.fld1.0 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607431768211355 => bb12,
_ => bb11
}
}
bb16 = {
_10 = core::ptr::addr_of!(_11);
_20 = 17049_i16 & 6489_i16;
_25.fld0 = _18;
_2 = _14 as i64;
_14 = _25.fld1.0 as i128;
_23 = [_13];
_25.fld1 = ((-76_i8),);
_1 = (*_10) ^ _15;
_8 = -_7;
_15 = _11;
_17 = _18.0;
RET = [6063696985597262343_usize];
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(12_usize, 19_usize, Move(_19), 23_usize, Move(_23), 21_usize, Move(_21), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(12_usize, 2_usize, Move(_2), 20_usize, Move(_20), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(12_usize, 18_usize, Move(_18), 17_usize, Move(_17), 29_usize, _29, 29_usize, _29), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: char,mut _2: ([bool; 2],),mut _3: u64,mut _4: u64,mut _5: ([bool; 2],),mut _6: u64,mut _7: char,mut _8: u64,mut _9: u64,mut _10: u64,mut _11: ([bool; 2],),mut _12: u64,mut _13: [bool; 2],mut _14: char) -> char {
mir! {
type RET = char;
let _15: [i64; 1];
let _16: Adt47;
let _17: u64;
let _18: u32;
let _19: Adt47;
let _20: [i64; 4];
let _21: f64;
let _22: i16;
let _23: u16;
let _24: char;
let _25: (*mut u64, usize, char);
let _26: [i64; 1];
let _27: bool;
let _28: char;
let _29: f64;
let _30: isize;
let _31: f32;
let _32: u16;
let _33: i64;
let _34: [bool; 2];
let _35: (*const u64, [bool; 2], *const i64, u32);
let _36: bool;
let _37: (u64,);
let _38: [usize; 1];
let _39: char;
let _40: u32;
let _41: ();
let _42: ();
{
_11 = _2;
_14 = _7;
_11 = (_5.0,);
_11 = (_13,);
Goto(bb1)
}
bb1 = {
_5.0 = _11.0;
_10 = (-34_i8) as u64;
_16.fld2.0 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-21_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_11 = (_5.0,);
RET = _7;
_6 = _9;
_16.fld2.0 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-122_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5.0 = [false,true];
_16.fld1 = !(-16765185294729291513518290761476246991_i128);
_9 = !_3;
_16.fld2.0 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-65_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-30_isize)];
_10 = _12;
_6 = _4 * _3;
_16.fld0 = _16.fld1 as u128;
_13 = [true,true];
_16.fld1 = !(-124502748183109036782285964375260289081_i128);
_16.fld2.1.0 = 5599_i16 as u64;
Goto(bb2)
}
bb2 = {
_16.fld0 = 55_isize as u128;
_19.fld2.1.0 = _6;
Goto(bb3)
}
bb3 = {
_19.fld2 = (_16.fld2.0, _16.fld2.1);
_14 = _7;
_19.fld0 = 114_i8 as u128;
_19.fld2.1.0 = !_6;
_19 = Move(_16);
_10 = _9 / 15167282196806208836_u64;
_10 = _19.fld2.1.0 | _6;
Call(_18 = core::intrinsics::transmute(_14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_8 = !_10;
_10 = !_8;
_21 = (-1788111418_i32) as f64;
_15 = [7762777701735943020_i64];
Goto(bb5)
}
bb5 = {
RET = _7;
_2 = _5;
_3 = !_9;
_11.0 = [true,false];
_10 = _9 | _9;
_19.fld2.1 = (_3,);
_12 = (-9223372036854775808_isize) as u64;
RET = _1;
_17 = _10;
_19.fld1 = 40126298006180990500447666211339657643_i128;
_23 = _10 as u16;
_2 = _5;
_11.0 = [true,false];
_19.fld2.1.0 = _10;
_25.0 = core::ptr::addr_of_mut!(_6);
_3 = !_12;
_10 = _17;
_11.0 = _13;
_19.fld1 = -(-31905651436482982092087784844714238325_i128);
_17 = _10 ^ _8;
_5.0 = [true,false];
Goto(bb6)
}
bb6 = {
_25.2 = _1;
_5.0 = _13;
_1 = _25.2;
_19.fld0 = 256342125482491342656453797193050566678_u128 / 326351272208973101018780685761865898518_u128;
_23 = 64027_u16 / 29499_u16;
_20 = [741792413343662469_i64,3309940571898262531_i64,8732006553142133603_i64,3515091080420614922_i64];
_28 = _25.2;
_25.1 = !1401641976612982625_usize;
_8 = _12;
_22 = (-2002221316_i32) as i16;
_19.fld0 = 291034532078225618425699174326158564784_u128;
match _19.fld0 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb7,
4 => bb8,
291034532078225618425699174326158564784 => bb10,
_ => bb9
}
}
bb7 = {
RET = _7;
_2 = _5;
_3 = !_9;
_11.0 = [true,false];
_10 = _9 | _9;
_19.fld2.1 = (_3,);
_12 = (-9223372036854775808_isize) as u64;
RET = _1;
_17 = _10;
_19.fld1 = 40126298006180990500447666211339657643_i128;
_23 = _10 as u16;
_2 = _5;
_11.0 = [true,false];
_19.fld2.1.0 = _10;
_25.0 = core::ptr::addr_of_mut!(_6);
_3 = !_12;
_10 = _17;
_11.0 = _13;
_19.fld1 = -(-31905651436482982092087784844714238325_i128);
_17 = _10 ^ _8;
_5.0 = [true,false];
Goto(bb6)
}
bb8 = {
_16.fld0 = 55_isize as u128;
_19.fld2.1.0 = _6;
Goto(bb3)
}
bb9 = {
_19.fld2 = (_16.fld2.0, _16.fld2.1);
_14 = _7;
_19.fld0 = 114_i8 as u128;
_19.fld2.1.0 = !_6;
_19 = Move(_16);
_10 = _9 / 15167282196806208836_u64;
_10 = _19.fld2.1.0 | _6;
Call(_18 = core::intrinsics::transmute(_14), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
RET = _7;
_7 = _14;
_19.fld2.0 = [9223372036854775807_isize,(-47_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_21 = 12_i8 as f64;
_14 = _7;
_27 = true;
_19.fld2.1 = (_17,);
_3 = _17;
_1 = _25.2;
_29 = _21 - _21;
_10 = _19.fld2.1.0;
_11.0 = [_27,_27];
_25.2 = _7;
_10 = !_3;
_27 = _6 > _10;
_19.fld0 = (-82_i8) as u128;
Goto(bb11)
}
bb11 = {
_10 = _17 & _19.fld2.1.0;
_4 = _19.fld2.1.0 + _17;
RET = _7;
_8 = !_4;
_32 = _23 >> _8;
_21 = _29;
_32 = _23 & _23;
_19.fld2.1.0 = _8;
_14 = _25.2;
_30 = _27 as isize;
_19.fld0 = _25.2 as u128;
_5.0 = _11.0;
_19.fld0 = _27 as u128;
_24 = _7;
_25.1 = 211_u8 as usize;
_19.fld1 = 42790149315028085578426260570904890302_i128;
_9 = !_4;
_2 = (_11.0,);
_19.fld2.0 = [_30,_30,_30,_30,_30,_30,_30,_30];
_33 = 4251151783567137582_i64;
Goto(bb12)
}
bb12 = {
_19.fld2.0 = [_30,_30,_30,_30,_30,_30,_30,_30];
_11.0 = [_27,_27];
_35.2 = core::ptr::addr_of!(_33);
match _19.fld1 {
0 => bb10,
42790149315028085578426260570904890302 => bb14,
_ => bb13
}
}
bb13 = {
RET = _7;
_2 = _5;
_3 = !_9;
_11.0 = [true,false];
_10 = _9 | _9;
_19.fld2.1 = (_3,);
_12 = (-9223372036854775808_isize) as u64;
RET = _1;
_17 = _10;
_19.fld1 = 40126298006180990500447666211339657643_i128;
_23 = _10 as u16;
_2 = _5;
_11.0 = [true,false];
_19.fld2.1.0 = _10;
_25.0 = core::ptr::addr_of_mut!(_6);
_3 = !_12;
_10 = _17;
_11.0 = _13;
_19.fld1 = -(-31905651436482982092087784844714238325_i128);
_17 = _10 ^ _8;
_5.0 = [true,false];
Goto(bb6)
}
bb14 = {
_34 = [_27,_27];
_36 = !_27;
_5.0 = _11.0;
RET = _28;
RET = _25.2;
_29 = -_21;
_29 = _21;
_5.0 = [_27,_36];
_19.fld2.1.0 = !_9;
_31 = (-1372724485_i32) as f32;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(13_usize, 11_usize, Move(_11), 12_usize, Move(_12), 28_usize, Move(_28), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(13_usize, 24_usize, Move(_24), 3_usize, Move(_3), 18_usize, Move(_18), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(13_usize, 33_usize, Move(_33), 22_usize, Move(_22), 14_usize, Move(_14), 36_usize, Move(_36)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(13_usize, 4_usize, Move(_4), 9_usize, Move(_9), 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: u64,mut _2: *const u64,mut _3: u64,mut _4: *const u64,mut _5: ([bool; 2],),mut _6: [bool; 2]) -> [bool; 2] {
mir! {
type RET = [bool; 2];
let _7: [i64; 4];
let _8: (isize, *const i32, f32, *const i32);
let _9: [i64; 4];
let _10: i32;
let _11: [isize; 8];
let _12: f64;
let _13: i64;
let _14: char;
let _15: *const i64;
let _16: isize;
let _17: [isize; 8];
let _18: *const i32;
let _19: [u32; 6];
let _20: f64;
let _21: Adt63;
let _22: f32;
let _23: (char,);
let _24: f32;
let _25: i64;
let _26: [u32; 6];
let _27: Adt52;
let _28: char;
let _29: [i16; 6];
let _30: ((*mut u64, usize, char), [char; 1], (i8,), [char; 1]);
let _31: u128;
let _32: [u32; 6];
let _33: [i64; 1];
let _34: char;
let _35: f64;
let _36: u8;
let _37: (u64,);
let _38: [bool; 2];
let _39: Adt48;
let _40: [char; 1];
let _41: bool;
let _42: (*const u64,);
let _43: f32;
let _44: bool;
let _45: ();
let _46: ();
{
(*_2) = !_1;
_1 = (*_2) | (*_2);
RET = [false,false];
_4 = _2;
_9 = [3672569685606014546_i64,(-3134759207573966265_i64),(-8981120493049578787_i64),3781484894719589162_i64];
_3 = !(*_2);
(*_4) = _1;
_4 = core::ptr::addr_of!(_3);
_8.0 = -(-9223372036854775808_isize);
_14 = '\u{63316}';
(*_4) = (*_2);
_13 = (-6384225809360322988_i64) | (-382469724859010316_i64);
_3 = _1 / 5257282485777646343_u64;
_8.2 = 2573659095_u32 as f32;
Goto(bb1)
}
bb1 = {
(*_2) = 1314917853_u32 as u64;
_11 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
(*_4) = !(*_2);
(*_4) = !_1;
Goto(bb2)
}
bb2 = {
_12 = 1999845776_u32 as f64;
_7 = _9;
_5 = (_6,);
_8.0 = (-9223372036854775808_isize) + (-10_isize);
_8.0 = 105_isize;
_3 = _1 & (*_2);
_13 = -(-3714190211012619359_i64);
(*_2) = !_1;
_10 = 1625120516_i32 >> (*_4);
RET = [true,false];
_4 = core::ptr::addr_of!(_1);
_8.2 = (*_2) as f32;
_9 = _7;
_8.1 = core::ptr::addr_of!(_10);
_12 = 39068754280480720842080206657315334271_i128 as f64;
_8.3 = core::ptr::addr_of!(_10);
_16 = !_8.0;
_14 = '\u{108b31}';
_5 = (_6,);
_16 = _8.0;
_5.0 = _6;
_5.0 = [false,false];
(*_4) = (*_2);
_3 = (*_2);
_8.1 = _8.3;
_1 = 181_u8 as u64;
_8.2 = 7134935758400581333_usize as f32;
Goto(bb3)
}
bb3 = {
_2 = _4;
_8.3 = core::ptr::addr_of!(_10);
_9 = [_13,_13,_13,_13];
_4 = _2;
_7 = [_13,_13,_13,_13];
_16 = _8.0 << _1;
(*_4) = true as u64;
_17 = [_8.0,_16,_8.0,_16,_16,_8.0,_16,_8.0];
_6 = _5.0;
_9 = _7;
_9 = [_13,_13,_13,_13];
_2 = _4;
_10 = -(-2028716550_i32);
_5.0 = [true,true];
(*_4) = _3 + _3;
_3 = !(*_4);
_5.0 = _6;
_5.0 = _6;
RET = [true,false];
match _8.0 {
0 => bb4,
105 => bb6,
_ => bb5
}
}
bb4 = {
_12 = 1999845776_u32 as f64;
_7 = _9;
_5 = (_6,);
_8.0 = (-9223372036854775808_isize) + (-10_isize);
_8.0 = 105_isize;
_3 = _1 & (*_2);
_13 = -(-3714190211012619359_i64);
(*_2) = !_1;
_10 = 1625120516_i32 >> (*_4);
RET = [true,false];
_4 = core::ptr::addr_of!(_1);
_8.2 = (*_2) as f32;
_9 = _7;
_8.1 = core::ptr::addr_of!(_10);
_12 = 39068754280480720842080206657315334271_i128 as f64;
_8.3 = core::ptr::addr_of!(_10);
_16 = !_8.0;
_14 = '\u{108b31}';
_5 = (_6,);
_16 = _8.0;
_5.0 = _6;
_5.0 = [false,false];
(*_4) = (*_2);
_3 = (*_2);
_8.1 = _8.3;
_1 = 181_u8 as u64;
_8.2 = 7134935758400581333_usize as f32;
Goto(bb3)
}
bb5 = {
(*_2) = 1314917853_u32 as u64;
_11 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
(*_4) = !(*_2);
(*_4) = !_1;
Goto(bb2)
}
bb6 = {
_21.fld2.fld1 = (68_i8,);
_1 = _10 as u64;
_21.fld2.fld2 = [true,true,false,true,false];
_15 = core::ptr::addr_of!(_13);
_21.fld2.fld4.0 = !_3;
_21.fld2.fld4.0 = !_3;
_21.fld2.fld0 = 29213_u16;
(*_2) = _3 ^ _3;
_4 = _2;
(*_4) = !_3;
_8.1 = core::ptr::addr_of!(_10);
_8.2 = 181540367934900020224621547158134467265_u128 as f32;
_6 = [false,false];
Goto(bb7)
}
bb7 = {
_11 = _17;
_21.fld1.0.0 = 82730115996613558788963630386025243981_i128 as i8;
RET = [true,false];
_4 = _2;
_19 = [1210404704_u32,2887676552_u32,4149014220_u32,2031752332_u32,1786502848_u32,1809355955_u32];
_23.0 = _14;
_11 = [_8.0,_8.0,_8.0,_16,_8.0,_16,_16,_16];
(*_2) = _21.fld2.fld4.0;
RET = [true,true];
_21.fld2.fld2 = [true,true,true,true,true];
_21.fld2.fld0 = !4939_u16;
_8.2 = _10 as f32;
_21.fld1.0 = (_21.fld2.fld1.0,);
_30.0.2 = _14;
_13 = !4051460619237992144_i64;
(*_2) = _21.fld2.fld4.0 * _21.fld2.fld4.0;
match _8.0 {
0 => bb6,
1 => bb8,
2 => bb9,
105 => bb11,
_ => bb10
}
}
bb8 = {
_2 = _4;
_8.3 = core::ptr::addr_of!(_10);
_9 = [_13,_13,_13,_13];
_4 = _2;
_7 = [_13,_13,_13,_13];
_16 = _8.0 << _1;
(*_4) = true as u64;
_17 = [_8.0,_16,_8.0,_16,_16,_8.0,_16,_8.0];
_6 = _5.0;
_9 = _7;
_9 = [_13,_13,_13,_13];
_2 = _4;
_10 = -(-2028716550_i32);
_5.0 = [true,true];
(*_4) = _3 + _3;
_3 = !(*_4);
_5.0 = _6;
_5.0 = _6;
RET = [true,false];
match _8.0 {
0 => bb4,
105 => bb6,
_ => bb5
}
}
bb9 = {
(*_2) = 1314917853_u32 as u64;
_11 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
(*_4) = !(*_2);
(*_4) = !_1;
Goto(bb2)
}
bb10 = {
_12 = 1999845776_u32 as f64;
_7 = _9;
_5 = (_6,);
_8.0 = (-9223372036854775808_isize) + (-10_isize);
_8.0 = 105_isize;
_3 = _1 & (*_2);
_13 = -(-3714190211012619359_i64);
(*_2) = !_1;
_10 = 1625120516_i32 >> (*_4);
RET = [true,false];
_4 = core::ptr::addr_of!(_1);
_8.2 = (*_2) as f32;
_9 = _7;
_8.1 = core::ptr::addr_of!(_10);
_12 = 39068754280480720842080206657315334271_i128 as f64;
_8.3 = core::ptr::addr_of!(_10);
_16 = !_8.0;
_14 = '\u{108b31}';
_5 = (_6,);
_16 = _8.0;
_5.0 = _6;
_5.0 = [false,false];
(*_4) = (*_2);
_3 = (*_2);
_8.1 = _8.3;
_1 = 181_u8 as u64;
_8.2 = 7134935758400581333_usize as f32;
Goto(bb3)
}
bb11 = {
_28 = _14;
Call(_27.fld0 = fn15(_30.0.2, _28, _1, _21.fld1.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_8.0 = _16;
_21.fld1.0 = _21.fld2.fld1;
_21.fld1 = (_21.fld2.fld1,);
_22 = _8.2;
_2 = _4;
_31 = !167689807170112521952308418507682333312_u128;
_30.0.0 = core::ptr::addr_of_mut!(_21.fld2.fld4.0);
_6 = [false,true];
_33 = [_13];
_30.0.1 = 3_usize ^ 15315102145601869365_usize;
_18 = core::ptr::addr_of!(_10);
_8.2 = 73595815954859884991621102032139123658_i128 as f32;
_2 = _4;
match _21.fld2.fld1.0 {
68 => bb13,
_ => bb1
}
}
bb13 = {
_20 = _12;
_2 = core::ptr::addr_of!(_37.0);
_19 = [950701638_u32,2036386130_u32,2718277210_u32,355249447_u32,2413491146_u32,2019921268_u32];
_30.1 = [_28];
_20 = _12 / (-0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001676826913444502_f64);
_39.fld2 = _4;
_39.fld3 = [true,true,true,false,false];
(*_15) = 1274477070037080544_i64 - (-3156037761537328861_i64);
_32 = [428782967_u32,297132474_u32,4117740208_u32,3701813807_u32,1033838930_u32,1349149516_u32];
_30.3 = [_14];
_7 = [(*_15),(*_15),_13,_13];
RET = [true,true];
(*_2) = _3 | (*_4);
_21.fld2.fld0 = _14 as u16;
_21.fld0 = core::ptr::addr_of_mut!(_31);
(*_15) = _20 as i64;
_4 = core::ptr::addr_of!(_3);
_10 = (-972695741_i32);
_36 = !168_u8;
Call(_43 = core::intrinsics::transmute(_30.3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_40 = _30.1;
_42.0 = _2;
_21.fld2.fld4 = (_37.0,);
_21.fld2 = Adt53 { fld0: 43451_u16,fld1: _21.fld1.0,fld2: _39.fld3,fld3: _15,fld4: _37 };
_8 = (_16, _18, _43, _18);
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(14_usize, 1_usize, Move(_1), 36_usize, Move(_36), 23_usize, Move(_23), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(14_usize, 6_usize, Move(_6), 9_usize, Move(_9), 5_usize, Move(_5), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(14_usize, 33_usize, Move(_33), 28_usize, Move(_28), 46_usize, _46, 46_usize, _46), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: char,mut _2: char,mut _3: u64,mut _4: (i8,)) -> [char; 1] {
mir! {
type RET = [char; 1];
let _5: *mut (*const u64, [bool; 2], *const i64, u32);
let _6: i16;
let _7: char;
let _8: i64;
let _9: [u32; 6];
let _10: ((i8,),);
let _11: ([isize; 8], (u64,));
let _12: isize;
let _13: f64;
let _14: f32;
let _15: Adt61;
let _16: bool;
let _17: u8;
let _18: isize;
let _19: char;
let _20: f64;
let _21: isize;
let _22: ();
let _23: ();
{
RET = [_1];
RET = [_1];
RET = [_2];
_1 = _2;
RET = [_2];
_6 = _3 as i16;
_6 = 10160_i16;
RET = [_2];
_3 = 133_u8 as u64;
_4.0 = !35_i8;
_2 = _1;
_2 = _1;
_6 = 9486_i16 & (-27773_i16);
_7 = _2;
_6 = 13943_i16 + (-23999_i16);
_3 = !5491194823322307395_u64;
_4.0 = -(-12_i8);
_6 = 3586045537556944514_i64 as i16;
_4.0 = -19_i8;
Call(_6 = core::intrinsics::bswap((-11303_i16)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.0 = (-120_i8) << _6;
Call(_8 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = (54_i8,);
RET = [_1];
Goto(bb3)
}
bb3 = {
_10.0 = (_4.0,);
_8 = 7046633671718337636_i64;
_7 = _1;
_4 = (_10.0.0,);
_10.0 = (_4.0,);
match _4.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
54 => bb9,
_ => bb8
}
}
bb4 = {
_4 = (54_i8,);
RET = [_1];
Goto(bb3)
}
bb5 = {
_4.0 = (-120_i8) << _6;
Call(_8 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
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
_6 = 9806_i16;
_9 = [3739900592_u32,1343951397_u32,1896228856_u32,3947128542_u32,580717239_u32,4214248606_u32];
_10 = (_4,);
_12 = _8 as isize;
_10 = (_4,);
_8 = 3451098594726116547_i64;
_11.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_11.1 = (_3,);
_13 = _3 as f64;
_10 = (_4,);
_4 = _10.0;
_6 = 3209017827_u32 as i16;
_10 = (_4,);
_14 = 184_u8 as f32;
_11.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_10 = (_4,);
_3 = _11.1.0 & _11.1.0;
_3 = _11.1.0;
_6 = 22510_i16 * (-27190_i16);
_14 = (-86463988784626881656802008013805371723_i128) as f32;
match _8 {
0 => bb4,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
3451098594726116547 => bb17,
_ => bb16
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
_4.0 = (-120_i8) << _6;
Call(_8 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_4 = (54_i8,);
RET = [_1];
Goto(bb3)
}
bb15 = {
_4.0 = (-120_i8) << _6;
Call(_8 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_4 = (54_i8,);
RET = [_1];
Goto(bb3)
}
bb17 = {
_7 = _1;
RET = [_1];
RET = [_2];
_15.fld7.fld7.0 = _11.0;
_15.fld7.fld2 = [_2];
_11.1.0 = !_3;
_11.0 = [_12,_12,_12,_12,_12,_12,_12,_12];
_15.fld7.fld7.1 = (_11.1.0,);
_14 = 92_u8 as f32;
_18 = 1097291829_i32 as isize;
_15.fld5 = 1926738894_i32 - (-1774066664_i32);
_15.fld7.fld1 = 13315164293759927550_usize;
_15.fld7.fld2 = [_1];
_6 = 23971881107080159339180519818641789293_u128 as i16;
_15.fld4.0 = _10.0.0 as isize;
_11 = (_15.fld7.fld7.0, _15.fld7.fld7.1);
_18 = !_12;
_15.fld7.fld3 = _10.0.0 >> _15.fld5;
_15.fld4.2 = _14;
_16 = _1 != _2;
_15.fld4.1 = core::ptr::addr_of!(_15.fld5);
_19 = _2;
_15.fld3 = _15.fld7.fld3 >> _15.fld5;
_17 = 202_u8 + 97_u8;
_15.fld1 = [_15.fld7.fld1];
Goto(bb18)
}
bb18 = {
Call(_22 = dump_var(15_usize, 8_usize, Move(_8), 9_usize, Move(_9), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_22 = dump_var(15_usize, 17_usize, Move(_17), 7_usize, Move(_7), 19_usize, Move(_19), 23_usize, _23), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: char,mut _2: char,mut _3: i8,mut _4: (u64,),mut _5: isize,mut _6: ([isize; 8], (u64,))) -> i8 {
mir! {
type RET = i8;
let _7: Adt60;
let _8: (u64,);
let _9: (u64,);
let _10: bool;
let _11: u64;
let _12: [char; 1];
let _13: (u64,);
let _14: Adt54;
let _15: i32;
let _16: [bool; 2];
let _17: (u64,);
let _18: u8;
let _19: Adt63;
let _20: u8;
let _21: u128;
let _22: *const *const u64;
let _23: (char,);
let _24: [usize; 1];
let _25: u128;
let _26: (*mut u64, usize, char);
let _27: char;
let _28: i64;
let _29: isize;
let _30: isize;
let _31: Adt52;
let _32: ([bool; 2], *const i32, i128, (isize, *const i32, f32, *const i32), isize, *mut u64);
let _33: i128;
let _34: ();
let _35: ();
{
_1 = _2;
_1 = _2;
_6.0 = [_5,_5,_5,_5,_5,_5,_5,_5];
_5 = 9223372036854775807_isize & (-9223372036854775808_isize);
RET = true as i8;
_4 = (_6.1.0,);
_7.fld6 = 632491975195521918_i64 >> _6.1.0;
RET = _3;
_3 = _6.1.0 as i8;
_6.0 = [_5,_5,_5,_5,_5,_5,_5,_5];
Goto(bb1)
}
bb1 = {
_3 = !(-75_i8);
_7.fld2 = !_5;
_7.fld3.0.0 = 6875_u16 as i8;
_4 = (_6.1.0,);
_4.0 = _6.1.0 | _6.1.0;
RET = -_7.fld3.0.0;
_2 = _1;
_7.fld4 = [3054981627622073198_usize];
_7.fld7.0 = core::ptr::addr_of!(_7.fld0);
_7.fld5 = [_1];
_5 = _7.fld2 << _4.0;
_7.fld3.0 = (_3,);
_1 = _2;
_7.fld1 = [(-14557_i16),(-27905_i16),(-384_i16),(-436_i16),(-27823_i16),(-22304_i16)];
_7.fld3.0 = (_3,);
_1 = _2;
_4 = _6.1;
_2 = _1;
_6.1.0 = _4.0 + _4.0;
Goto(bb2)
}
bb2 = {
_9.0 = !_4.0;
RET = _3;
_9 = (_6.1.0,);
_12 = _7.fld5;
_7.fld7.0 = core::ptr::addr_of!(_6.1.0);
_7.fld2 = _5 - _5;
_8 = _9;
_7.fld2 = -_5;
_7.fld2 = -_5;
_13 = (_9.0,);
_4.0 = 322233072_u32 as u64;
_2 = _1;
_14.fld3.3.2 = _7.fld6 as f32;
_14.fld0.0 = core::ptr::addr_of!(_9.0);
_14.fld3.2 = 39233115504944676928662016563556014841_i128 & 37386128034561689004583273945184840189_i128;
_7.fld3.0.0 = 4827257682110810801578352880012908876_u128 as i8;
_4.0 = !_8.0;
_14.fld0.0 = _7.fld7.0;
_1 = _2;
_14.fld2 = !(-9753_i16);
RET = !_7.fld3.0.0;
_11 = _4.0 / 13363786292007116605_u64;
_4.0 = 1397050966_u32 as u64;
_14.fld3.4 = _7.fld2;
_9.0 = _6.1.0;
Goto(bb3)
}
bb3 = {
_5 = _14.fld3.4 & _7.fld2;
_14.fld3.0 = [true,false];
_14.fld1 = _6;
_12 = [_1];
_14.fld3.2 = _7.fld6 as i128;
_14.fld3.3.2 = 36496_u16 as f32;
_15 = 1342090905_i32;
match _15 {
0 => bb4,
1 => bb5,
1342090905 => bb7,
_ => bb6
}
}
bb4 = {
_9.0 = !_4.0;
RET = _3;
_9 = (_6.1.0,);
_12 = _7.fld5;
_7.fld7.0 = core::ptr::addr_of!(_6.1.0);
_7.fld2 = _5 - _5;
_8 = _9;
_7.fld2 = -_5;
_7.fld2 = -_5;
_13 = (_9.0,);
_4.0 = 322233072_u32 as u64;
_2 = _1;
_14.fld3.3.2 = _7.fld6 as f32;
_14.fld0.0 = core::ptr::addr_of!(_9.0);
_14.fld3.2 = 39233115504944676928662016563556014841_i128 & 37386128034561689004583273945184840189_i128;
_7.fld3.0.0 = 4827257682110810801578352880012908876_u128 as i8;
_4.0 = !_8.0;
_14.fld0.0 = _7.fld7.0;
_1 = _2;
_14.fld2 = !(-9753_i16);
RET = !_7.fld3.0.0;
_11 = _4.0 / 13363786292007116605_u64;
_4.0 = 1397050966_u32 as u64;
_14.fld3.4 = _7.fld2;
_9.0 = _6.1.0;
Goto(bb3)
}
bb5 = {
_3 = !(-75_i8);
_7.fld2 = !_5;
_7.fld3.0.0 = 6875_u16 as i8;
_4 = (_6.1.0,);
_4.0 = _6.1.0 | _6.1.0;
RET = -_7.fld3.0.0;
_2 = _1;
_7.fld4 = [3054981627622073198_usize];
_7.fld7.0 = core::ptr::addr_of!(_7.fld0);
_7.fld5 = [_1];
_5 = _7.fld2 << _4.0;
_7.fld3.0 = (_3,);
_1 = _2;
_7.fld1 = [(-14557_i16),(-27905_i16),(-384_i16),(-436_i16),(-27823_i16),(-22304_i16)];
_7.fld3.0 = (_3,);
_1 = _2;
_4 = _6.1;
_2 = _1;
_6.1.0 = _4.0 + _4.0;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_7.fld0 = !_9.0;
_14.fld3.3.0 = _5;
_19.fld2.fld0 = 962459445_u32 as u16;
_14.fld3.5 = core::ptr::addr_of_mut!(_4.0);
_14.fld3.1 = core::ptr::addr_of!(_15);
_4 = _8;
_2 = _1;
_20 = !171_u8;
Goto(bb8)
}
bb8 = {
_19.fld2.fld0 = 25675_u16 + 19477_u16;
_19.fld2.fld1 = (_7.fld3.0.0,);
_7.fld0 = !_11;
_14.fld0 = (_7.fld7.0,);
match _15 {
1342090905 => bb9,
_ => bb5
}
}
bb9 = {
RET = _7.fld3.0.0;
_19.fld1.0.0 = _14.fld3.3.2 as i8;
_9.0 = !_11;
_17.0 = !_11;
_7.fld3 = (_19.fld1.0,);
_19.fld2.fld0 = 55028_u16;
_21 = false as u128;
_19.fld2.fld4 = (_17.0,);
_19.fld0 = core::ptr::addr_of_mut!(_21);
_22 = core::ptr::addr_of!(_14.fld0.0);
_14.fld1.0 = [_14.fld3.3.0,_14.fld3.4,_14.fld3.4,_7.fld2,_14.fld3.4,_5,_5,_7.fld2];
_4 = (_19.fld2.fld4.0,);
_9 = _19.fld2.fld4;
_14.fld3.3.3 = _14.fld3.1;
_7.fld6 = !8355488738700762030_i64;
_21 = 58986962897530962277461897717425397867_u128 * 124775153026677193481557260235207840316_u128;
_14.fld3.2 = _14.fld2 as i128;
_9 = (_6.1.0,);
_19.fld2.fld1.0 = _19.fld2.fld0 as i8;
_7.fld6 = -823941938351687588_i64;
_19.fld2.fld3 = core::ptr::addr_of!(_7.fld6);
_14.fld0.0 = _7.fld7.0;
match _19.fld2.fld0 {
0 => bb3,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
55028 => bb16,
_ => bb15
}
}
bb10 = {
_9.0 = !_4.0;
RET = _3;
_9 = (_6.1.0,);
_12 = _7.fld5;
_7.fld7.0 = core::ptr::addr_of!(_6.1.0);
_7.fld2 = _5 - _5;
_8 = _9;
_7.fld2 = -_5;
_7.fld2 = -_5;
_13 = (_9.0,);
_4.0 = 322233072_u32 as u64;
_2 = _1;
_14.fld3.3.2 = _7.fld6 as f32;
_14.fld0.0 = core::ptr::addr_of!(_9.0);
_14.fld3.2 = 39233115504944676928662016563556014841_i128 & 37386128034561689004583273945184840189_i128;
_7.fld3.0.0 = 4827257682110810801578352880012908876_u128 as i8;
_4.0 = !_8.0;
_14.fld0.0 = _7.fld7.0;
_1 = _2;
_14.fld2 = !(-9753_i16);
RET = !_7.fld3.0.0;
_11 = _4.0 / 13363786292007116605_u64;
_4.0 = 1397050966_u32 as u64;
_14.fld3.4 = _7.fld2;
_9.0 = _6.1.0;
Goto(bb3)
}
bb11 = {
_7.fld0 = !_9.0;
_14.fld3.3.0 = _5;
_19.fld2.fld0 = 962459445_u32 as u16;
_14.fld3.5 = core::ptr::addr_of_mut!(_4.0);
_14.fld3.1 = core::ptr::addr_of!(_15);
_4 = _8;
_2 = _1;
_20 = !171_u8;
Goto(bb8)
}
bb12 = {
Return()
}
bb13 = {
_3 = !(-75_i8);
_7.fld2 = !_5;
_7.fld3.0.0 = 6875_u16 as i8;
_4 = (_6.1.0,);
_4.0 = _6.1.0 | _6.1.0;
RET = -_7.fld3.0.0;
_2 = _1;
_7.fld4 = [3054981627622073198_usize];
_7.fld7.0 = core::ptr::addr_of!(_7.fld0);
_7.fld5 = [_1];
_5 = _7.fld2 << _4.0;
_7.fld3.0 = (_3,);
_1 = _2;
_7.fld1 = [(-14557_i16),(-27905_i16),(-384_i16),(-436_i16),(-27823_i16),(-22304_i16)];
_7.fld3.0 = (_3,);
_1 = _2;
_4 = _6.1;
_2 = _1;
_6.1.0 = _4.0 + _4.0;
Goto(bb2)
}
bb14 = {
_9.0 = !_4.0;
RET = _3;
_9 = (_6.1.0,);
_12 = _7.fld5;
_7.fld7.0 = core::ptr::addr_of!(_6.1.0);
_7.fld2 = _5 - _5;
_8 = _9;
_7.fld2 = -_5;
_7.fld2 = -_5;
_13 = (_9.0,);
_4.0 = 322233072_u32 as u64;
_2 = _1;
_14.fld3.3.2 = _7.fld6 as f32;
_14.fld0.0 = core::ptr::addr_of!(_9.0);
_14.fld3.2 = 39233115504944676928662016563556014841_i128 & 37386128034561689004583273945184840189_i128;
_7.fld3.0.0 = 4827257682110810801578352880012908876_u128 as i8;
_4.0 = !_8.0;
_14.fld0.0 = _7.fld7.0;
_1 = _2;
_14.fld2 = !(-9753_i16);
RET = !_7.fld3.0.0;
_11 = _4.0 / 13363786292007116605_u64;
_4.0 = 1397050966_u32 as u64;
_14.fld3.4 = _7.fld2;
_9.0 = _6.1.0;
Goto(bb3)
}
bb15 = {
_5 = _14.fld3.4 & _7.fld2;
_14.fld3.0 = [true,false];
_14.fld1 = _6;
_12 = [_1];
_14.fld3.2 = _7.fld6 as i128;
_14.fld3.3.2 = 36496_u16 as f32;
_15 = 1342090905_i32;
match _15 {
0 => bb4,
1 => bb5,
1342090905 => bb7,
_ => bb6
}
}
bb16 = {
(*_22) = core::ptr::addr_of!(_6.1.0);
RET = _7.fld3.0.0;
_7.fld4 = [0_usize];
_19.fld1.0.0 = !_7.fld3.0.0;
_14.fld3.3.3 = core::ptr::addr_of!(_15);
_26 = (_14.fld3.5, 0_usize, _1);
_14.fld3.1 = core::ptr::addr_of!(_15);
_3 = _19.fld2.fld0 as i8;
_15 = 1008900052_i32;
_14.fld3.2 = 101258615117156271405653836626209686120_i128;
(*_22) = _7.fld7.0;
_8.0 = !_4.0;
_30 = true as isize;
_14.fld3.3.1 = core::ptr::addr_of!(_15);
_27 = _26.2;
_19.fld2.fld1 = (_19.fld1.0.0,);
_9 = (_19.fld2.fld4.0,);
_29 = _14.fld3.4;
_21 = !144780189175664187817389510400073869739_u128;
_2 = _26.2;
_7.fld0 = _7.fld6 as u64;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(16_usize, 5_usize, Move(_5), 20_usize, Move(_20), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(16_usize, 17_usize, Move(_17), 3_usize, Move(_3), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(16_usize, 6_usize, Move(_6), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: u64,mut _2: (*const u64, [bool; 2], *const i64, u32),mut _3: i128,mut _4: u128,mut _5: Adt60,mut _6: f32,mut _7: *const u64,mut _8: i8,mut _9: [bool; 2],mut _10: *mut u128,mut _11: u64) -> isize {
mir! {
type RET = isize;
let _12: (*const u64, [bool; 2], *const i64, u32);
let _13: f64;
let _14: ((i8,),);
let _15: u8;
let _16: ();
let _17: ();
{
_5.fld7 = (_7,);
(*_7) = _5.fld0;
_12.0 = _5.fld7.0;
_1 = !(*_7);
_12.1 = [true,true];
RET = -_5.fld2;
_2.1 = [true,false];
_12.0 = core::ptr::addr_of!(_5.fld0);
_5.fld3.0 = (_8,);
(*_7) = !_1;
_5.fld7.0 = core::ptr::addr_of!((*_7));
_5.fld0 = !_1;
_6 = 1_usize as f32;
_7 = _5.fld7.0;
_8 = -_5.fld3.0.0;
_4 = 61458391753047661130349996511592694872_u128;
_3 = (-32826104952212925266220660770875872569_i128) & (-22938031189580074725981455288645670771_i128);
(*_10) = _4;
_12.2 = core::ptr::addr_of!(_5.fld6);
_3 = _2.3 as i128;
_14.0.0 = _5.fld3.0.0 + _8;
_10 = core::ptr::addr_of_mut!((*_10));
_12 = _2;
_5.fld1 = [6578_i16,(-15425_i16),13478_i16,29063_i16,(-15011_i16),(-12834_i16)];
_3 = (-35370897850109663727776700468069729447_i128);
_4 = (*_10);
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(17_usize, 8_usize, Move(_8), 14_usize, Move(_14), 4_usize, Move(_4), 17_usize, _17), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: *mut u64,mut _2: char,mut _3: Adt52,mut _4: i8,mut _5: u64,mut _6: char,mut _7: i128,mut _8: char,mut _9: char) -> i32 {
mir! {
type RET = i32;
let _10: isize;
let _11: f64;
let _12: [isize; 8];
let _13: i8;
let _14: i128;
let _15: Adt47;
let _16: [i64; 1];
let _17: usize;
let _18: Adt53;
let _19: f32;
let _20: i8;
let _21: f32;
let _22: isize;
let _23: ([bool; 2],);
let _24: i16;
let _25: ();
let _26: ();
{
_9 = _6;
RET = (-9223372036854775808_isize) as i32;
_4 = 9223372036854775807_isize as i8;
_8 = _9;
_4 = (-70_i8) - 102_i8;
RET = 3715490900751259173_i64 as i32;
_10 = _4 as isize;
(*_1) = !_5;
(*_1) = _5;
RET = 1883717250_i32 * (-450574736_i32);
Goto(bb1)
}
bb1 = {
_9 = _2;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_2 = _8;
_6 = _2;
_2 = _9;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_3.fld0 = [_9];
RET = 1604279964_i32 & 997725929_i32;
_5 = (-9340_i16) as u64;
_10 = (-9223372036854775808_isize);
_15.fld0 = 220314612961684240137959761682754922131_u128 % 166683684430815914966598544090420922127_u128;
_1 = core::ptr::addr_of_mut!((*_1));
RET = -(-278477078_i32);
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454151235394913435648 => bb8,
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
_15.fld2.1 = ((*_1),);
_11 = _15.fld2.1.0 as f64;
_15.fld0 = 280468750848061695238679274158341415566_u128;
_8 = _9;
_15.fld2.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_17 = !9589435343425708654_usize;
_13 = _4;
_16 = [(-1951671944257856986_i64)];
_16 = [(-9149919648594632401_i64)];
_2 = _9;
_14 = -_7;
Goto(bb9)
}
bb9 = {
_7 = _14;
RET = (-155949851_i32) ^ 600660123_i32;
_2 = _8;
_2 = _9;
_16 = [6168709786317849515_i64];
(*_1) = _15.fld2.1.0;
(*_1) = _10 as u64;
RET = _17 as i32;
_16 = [(-8456188599695875701_i64)];
_15.fld1 = _7;
_17 = 8308_i16 as usize;
_6 = _2;
_4 = _15.fld0 as i8;
_7 = -_15.fld1;
_18.fld1.0 = _4 >> _5;
_6 = _9;
_10 = !(-9223372036854775808_isize);
_18.fld2 = [false,false,false,true,false];
(*_1) = !_5;
_19 = _18.fld1.0 as f32;
_20 = _18.fld1.0;
_15.fld2.1.0 = (*_1);
match _15.fld0 {
0 => bb2,
1 => bb10,
2 => bb11,
3 => bb12,
280468750848061695238679274158341415566 => bb14,
_ => bb13
}
}
bb10 = {
_15.fld2.1 = ((*_1),);
_11 = _15.fld2.1.0 as f64;
_15.fld0 = 280468750848061695238679274158341415566_u128;
_8 = _9;
_15.fld2.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_17 = !9589435343425708654_usize;
_13 = _4;
_16 = [(-1951671944257856986_i64)];
_16 = [(-9149919648594632401_i64)];
_2 = _9;
_14 = -_7;
Goto(bb9)
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
_15.fld2.0 = _12;
RET = !376217021_i32;
_18.fld4 = _15.fld2.1;
_18.fld0 = !43614_u16;
RET = _11 as i32;
_4 = !_18.fld1.0;
_21 = _15.fld0 as f32;
_15.fld2.1 = _18.fld4;
_18.fld2 = [true,true,false,false,true];
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_7 = 193189484_i32 as i128;
_15.fld1 = _7 | _14;
_3.fld0 = [_9];
_10 = false as isize;
_18.fld2 = [false,true,false,true,true];
_15.fld2 = (_12, _18.fld4);
_20 = !_4;
_8 = _6;
_15.fld2 = (_12, _18.fld4);
(*_1) = _18.fld4.0 | _15.fld2.1.0;
_2 = _9;
_9 = _6;
_6 = _2;
_17 = 4216932395211132690_usize | 4999032247745844428_usize;
_11 = (-4213965801000590207_i64) as f64;
_3.fld0 = [_2];
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(18_usize, 9_usize, Move(_9), 20_usize, Move(_20), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(18_usize, 7_usize, Move(_7), 12_usize, Move(_12), 4_usize, Move(_4), 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                println!("{:?}", fn0(std::hint::black_box(49490_u16), std::hint::black_box('\u{4045b}'), std::hint::black_box(27164_i16), std::hint::black_box(2_u8)));
                
            }
#[derive(Debug)]
pub struct Adt47 {
fld0: u128,
fld1: i128,
fld2: ([isize; 8], (u64,)),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt48 {
fld0: (i8,),
fld1: (*const u64,),
fld2: *const u64,
fld3: [bool; 5],
fld4: *mut (*const u64, [bool; 2], *const i64, u32),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt49 {
fld0: *mut u64,
fld1: (u64,),
fld2: *mut u128,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt50 {
fld0: (char,),
fld1: (i8,),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt51 {
fld0: [u32; 6],
fld1: usize,
fld2: [char; 1],
fld3: i8,
fld4: *const i32,
fld5: u8,
fld6: [i64; 4],
fld7: ([isize; 8], (u64,)),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt52 {
fld0: [char; 1],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt53 {
fld0: u16,
fld1: (i8,),
fld2: [bool; 5],
fld3: *const i64,
fld4: (u64,),
}
#[derive(Debug)]
pub struct Adt54 {
fld0: (*const u64,),
fld1: ([isize; 8], (u64,)),
fld2: i16,
fld3: ([bool; 2], *const i32, i128, (isize, *const i32, f32, *const i32), isize, *mut u64),
}
#[derive(Debug)]
pub struct Adt55 {
fld0: u8,
fld1: *mut (*const u64, [bool; 2], *const i64, u32),
fld2: [i64; 4],
fld3: (*const u64, [bool; 2], *const i64, u32),
fld4: [u32; 6],
fld5: ([bool; 2], *const i32, i128, (isize, *const i32, f32, *const i32), isize, *mut u64),
fld6: [bool; 5],
fld7: Adt49,
}
#[derive(Debug)]
pub struct Adt56 {
fld0: i128,
fld1: *const *const u64,
fld2: isize,
fld3: usize,
fld4: [i64; 4],
fld5: Adt47,
fld6: [i64; 1],
}
#[derive(Debug,Copy,Clone)]
pub struct Adt57 {
fld0: f32,
fld1: u64,
fld2: ([usize; 1], *mut (*const u64, [bool; 2], *const i64, u32), i64),
}
#[derive(Debug)]
pub struct Adt58 {
fld0: *const *const u64,
fld1: i64,
fld2: (i8,),
}
#[derive(Debug)]
pub struct Adt59 {
fld0: (isize, *const i32, f32, *const i32),
fld1: i64,
fld2: ([bool; 2], *const i32, i128, (isize, *const i32, f32, *const i32), isize, *mut u64),
fld3: Adt48,
fld4: ([isize; 8], (u64,)),
}
#[derive(Debug)]
pub struct Adt60 {
fld0: u64,
fld1: [i16; 6],
fld2: isize,
fld3: ((i8,),),
fld4: [usize; 1],
fld5: [char; 1],
fld6: i64,
fld7: (*const u64,),
}
#[derive(Debug,Copy,Clone)]
pub struct Adt61 {
fld0: *mut (*const u64, [bool; 2], *const i64, u32),
fld1: [usize; 1],
fld2: [bool; 2],
fld3: i8,
fld4: (isize, *const i32, f32, *const i32),
fld5: i32,
fld6: *const *const u64,
fld7: Adt51,
}
#[derive(Debug)]
pub struct Adt62 {
fld0: u64,
fld1: char,
fld2: i128,
fld3: i8,
fld4: i16,
fld5: u128,
fld6: Adt54,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt63 {
fld0: *mut u128,
fld1: ((i8,),),
fld2: Adt53,
}

