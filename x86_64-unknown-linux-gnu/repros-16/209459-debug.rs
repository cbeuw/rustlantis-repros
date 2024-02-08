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
pub fn fn0(mut _1: bool,mut _2: u16,mut _3: isize,mut _4: i8,mut _5: u128,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u32) -> Adt42 {
mir! {
type RET = Adt42;
let _11: (u32,);
let _12: f64;
let _13: Adt41;
let _14: Adt56;
let _15: bool;
let _16: Adt45;
let _17: i128;
let _18: Adt44;
let _19: *mut [u64; 4];
let _20: char;
let _21: [isize; 1];
let _22: ([i128; 5],);
let _23: Adt56;
let _24: Adt55;
let _25: *mut *mut bool;
let _26: Adt56;
let _27: Adt51;
let _28: bool;
let _29: [u8; 7];
let _30: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize);
let _31: Adt48;
let _32: isize;
let _33: f32;
let _34: ();
let _35: ();
{
RET.fld0.1 = (-13715683045777954665010252928549615829_i128);
RET.fld3.0 = [RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1];
_10 = 2239753443_u32;
_6 = 1970529044_i32 * 835270807_i32;
RET.fld0 = (4500_i16, 89331256633551193529180960881328969338_i128, 5867590006123387907_u64);
_1 = false;
RET.fld2.0 = 9223372036854775807_isize - (-9223372036854775808_isize);
RET.fld3.0 = [RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1];
_3 = RET.fld0.1 as isize;
_11 = (_10,);
_4 = 68_i8;
_1 = !false;
_8 = 100743206609033613763406605490787042358_u128 as i128;
_4 = !51_i8;
RET.fld3.0 = [RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,_8];
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
2239753443 => bb7,
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
RET.fld0.2 = (-3561999628345847604_i64) as u64;
RET.fld3.0 = [RET.fld0.1,RET.fld0.1,RET.fld0.1,RET.fld0.1,_8];
_6 = 302660826713591530145973858170021080734_u128 as i32;
RET.fld0.2 = 4103728050653200847_u64;
RET.fld2.0 = _3 ^ _3;
RET.fld0.1 = _8 * _8;
_14.fld4.fld3.0 = [_8,RET.fld0.1,RET.fld0.1,_8,RET.fld0.1];
RET.fld0.1 = 16646_u16 as i128;
RET.fld2 = (_3,);
_14.fld4.fld2 = RET.fld2;
RET.fld0.0 = (-27085_i16);
_14.fld4.fld0.1 = !_8;
RET.fld3.0 = _14.fld4.fld3.0;
RET.fld1 = _1 as u16;
RET.fld0.0 = (-29209_i16) * 8772_i16;
_14.fld0 = (_14.fld4.fld2.0,);
_14.fld4.fld0.2 = 82809908710294267603815128446745528452_u128 as u64;
_1 = false;
_14.fld4.fld1 = (-7226695344011911117_i64) as u16;
_14.fld4.fld1 = RET.fld1;
RET.fld0.1 = _8 >> _14.fld4.fld1;
_14.fld4.fld3 = (RET.fld3.0,);
match _10 {
2239753443 => bb8,
_ => bb5
}
}
bb8 = {
Goto(bb9)
}
bb9 = {
_7 = -8575516741499042007_i64;
_1 = !false;
_14.fld4.fld3.0 = [RET.fld0.1,RET.fld0.1,_8,RET.fld0.1,RET.fld0.1];
RET.fld0.2 = !_14.fld4.fld0.2;
_14.fld4.fld0 = (RET.fld0.0, _8, RET.fld0.2);
_11.0 = _10 << _14.fld4.fld0.1;
_14.fld4.fld3.0 = [_8,_14.fld4.fld0.1,_8,_8,_14.fld4.fld0.1];
_14.fld4.fld0.1 = _10 as i128;
RET.fld2.0 = !_14.fld0.0;
RET = Adt42 { fld0: _14.fld4.fld0,fld1: _14.fld4.fld1,fld2: _14.fld0,fld3: _14.fld4.fld3 };
Call(_7 = fn1(_3, RET.fld1, _3, RET.fld2.0, _14.fld4.fld2.0, Move(RET), _1, _4, _14.fld0, _14.fld4.fld0.1, _14.fld4.fld0.1, _14.fld0.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_14.fld4.fld0.2 = !6957400780741579150_u64;
RET.fld0.1 = _3 as i128;
RET.fld3 = _14.fld4.fld3;
Call(_14.fld4.fld3.0 = fn3(RET.fld0.1, _10, _6, _14.fld4.fld2.0, _10, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11.0 = _6 as u32;
_23.fld2 = _14.fld4.fld1;
RET.fld0 = (_14.fld4.fld0.0, _8, _14.fld4.fld0.2);
_14.fld0.0 = -_3;
_23.fld4.fld1 = _4 as u16;
_2 = !_14.fld4.fld1;
_23.fld4 = Adt42 { fld0: RET.fld0,fld1: _2,fld2: _14.fld4.fld2,fld3: _14.fld4.fld3 };
_14.fld1 = '\u{5509e}';
_7 = -(-6971283052560886476_i64);
_23.fld0 = (_14.fld0.0,);
_8 = RET.fld0.1;
RET.fld0.0 = _14.fld4.fld0.0;
_26.fld4.fld2 = (_14.fld4.fld2.0,);
_23.fld4.fld0.2 = RET.fld0.2;
_20 = _14.fld1;
RET.fld2.0 = _14.fld4.fld2.0 | _3;
_14.fld0 = (RET.fld2.0,);
_14.fld3 = [_3];
_3 = _14.fld0.0;
RET.fld1 = _23.fld4.fld1;
_12 = RET.fld2.0 as f64;
_23 = Adt56 { fld0: _14.fld4.fld2,fld1: _20,fld2: _14.fld4.fld1,fld3: _14.fld3,fld4: Move(_14.fld4) };
Goto(bb12)
}
bb12 = {
_12 = 262985872131733634417403295831155393393_u128 as f64;
RET.fld3.0 = _23.fld4.fld3.0;
_21 = [_3];
_26.fld4.fld0 = (_23.fld4.fld0.0, RET.fld0.1, RET.fld0.2);
_17 = _7 as i128;
_9 = 0_usize << _7;
_14.fld4.fld3 = (_23.fld4.fld3.0,);
Goto(bb13)
}
bb13 = {
Call(_34 = dump_var(0_usize, 17_usize, Move(_17), 1_usize, Move(_1), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_34 = dump_var(0_usize, 9_usize, Move(_9), 7_usize, Move(_7), 35_usize, _35, 35_usize, _35), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: u16,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: Adt42,mut _7: bool,mut _8: i8,mut _9: (isize,),mut _10: i128,mut _11: i128,mut _12: isize) -> i64 {
mir! {
type RET = i64;
let _13: bool;
let _14: *mut bool;
let _15: f32;
let _16: i16;
let _17: u8;
let _18: u16;
let _19: u64;
let _20: Adt42;
let _21: (i16, i128, u64);
let _22: Adt52;
let _23: Adt41;
let _24: i128;
let _25: f64;
let _26: ();
let _27: ();
{
_6.fld0.2 = !9516328768331275757_u64;
_11 = 2881740070_u32 as i128;
RET = '\u{f8430}' as i64;
_9.0 = _12 - _4;
_14 = core::ptr::addr_of_mut!(_7);
_13 = _4 <= _1;
_9 = (_5,);
_3 = -_5;
_14 = core::ptr::addr_of_mut!(_7);
_13 = (*_14);
_15 = 371475049_i32 as f32;
RET = 3465310068480478048_i64;
_9 = (_1,);
_12 = _1 >> _9.0;
_13 = !_7;
_1 = 220_u8 as isize;
_6.fld2.0 = _12;
_6.fld1 = 803820895_i32 as u16;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
3465310068480478048 => bb7,
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
_6.fld0.1 = !_10;
_17 = !56_u8;
_16 = -_6.fld0.0;
_5 = 3140988000_u32 as isize;
_19 = _8 as u64;
RET = 7672216924431137844_i64;
_20.fld0 = (_16, _10, _6.fld0.2);
_17 = 176_u8 | 137_u8;
_19 = !_20.fld0.2;
_6.fld0 = (_20.fld0.0, _20.fld0.1, _20.fld0.2);
Goto(bb8)
}
bb8 = {
_6.fld0.0 = _20.fld0.0;
_20 = Move(_6);
_4 = _3;
_9.0 = _20.fld2.0 * _12;
_18 = !_20.fld1;
_20.fld0.1 = !_11;
_15 = _20.fld1 as f32;
match RET {
0 => bb9,
1 => bb10,
7672216924431137844 => bb12,
_ => bb11
}
}
bb9 = {
_6.fld0.1 = !_10;
_17 = !56_u8;
_16 = -_6.fld0.0;
_5 = 3140988000_u32 as isize;
_19 = _8 as u64;
RET = 7672216924431137844_i64;
_20.fld0 = (_16, _10, _6.fld0.2);
_17 = 176_u8 | 137_u8;
_19 = !_20.fld0.2;
_6.fld0 = (_20.fld0.0, _20.fld0.1, _20.fld0.2);
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_7 = _11 < _11;
Goto(bb13)
}
bb13 = {
_6.fld0.2 = !_20.fld0.2;
_21.1 = -_10;
_10 = _21.1;
_6.fld3.0 = [_11,_11,_10,_21.1,_10];
_20.fld3.0 = [_20.fld0.1,_20.fld0.1,_21.1,_21.1,_21.1];
_5 = '\u{10a48f}' as isize;
_6.fld0.0 = -_20.fld0.0;
_20.fld0.2 = RET as u64;
_1 = _9.0;
_7 = _13;
_6 = Adt42 { fld0: _20.fld0,fld1: _20.fld1,fld2: _20.fld2,fld3: _20.fld3 };
_6.fld2.0 = _8 as isize;
_21.2 = !_19;
_18 = _2;
_11 = _6.fld0.1;
_9 = (_4,);
_21.0 = _17 as i16;
_17 = !218_u8;
_1 = _9.0 ^ _3;
_4 = _11 as isize;
_20.fld0 = (_16, _11, _6.fld0.2);
_6.fld2 = (_20.fld2.0,);
_18 = _2 ^ _2;
_6.fld0.0 = _12 as i16;
_1 = _9.0 & _5;
_20.fld0.0 = !_6.fld0.0;
_6.fld3.0 = [_11,_6.fld0.1,_20.fld0.1,_11,_20.fld0.1];
_1 = (-1515575803_i32) as isize;
_21.1 = _11 & _11;
Call(_6.fld0 = fn2(_12, _6.fld2.0, Move(_20), _9.0, _21.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_6.fld0.2 = _21.2 | _19;
_8 = (-25_i8);
_9.0 = -_6.fld2.0;
_18 = _6.fld1 | _2;
_20.fld0 = (_16, _6.fld0.1, _6.fld0.2);
_17 = 143_u8 >> _6.fld0.0;
_14 = core::ptr::addr_of_mut!(_13);
_21.2 = !_6.fld0.2;
_14 = core::ptr::addr_of_mut!((*_14));
_6.fld3.0 = [_20.fld0.1,_6.fld0.1,_6.fld0.1,_6.fld0.1,_20.fld0.1];
_20.fld1 = _18;
_20.fld2 = _6.fld2;
_21.2 = !_20.fld0.2;
RET = 9201299660344316557_i64;
_20.fld2 = _9;
_25 = _15 as f64;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(1_usize, 11_usize, Move(_11), 18_usize, Move(_18), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(1_usize, 3_usize, Move(_3), 10_usize, Move(_10), 9_usize, Move(_9), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: Adt42,mut _4: isize,mut _5: i128) -> (i16, i128, u64) {
mir! {
type RET = (i16, i128, u64);
let _6: (u32,);
let _7: f32;
let _8: [i128; 5];
let _9: isize;
let _10: char;
let _11: u8;
let _12: [i8; 1];
let _13: (u32,);
let _14: u64;
let _15: f32;
let _16: (u32,);
let _17: Adt56;
let _18: ();
let _19: ();
{
RET.1 = 5_usize as i128;
_3.fld2.0 = _2;
RET.0 = _3.fld0.0;
_3.fld0 = (RET.0, _5, 6787660839605253114_u64);
_7 = 24_i8 as f32;
_3.fld0 = (RET.0, _5, 2614988056554778044_u64);
RET = (_3.fld0.0, _3.fld0.1, _3.fld0.2);
_3.fld0.1 = _5;
_6.0 = !937035347_u32;
RET.0 = (-534042360_i32) as i16;
_4 = -_2;
Goto(bb1)
}
bb1 = {
RET.1 = _3.fld0.1;
RET = _3.fld0;
_3.fld0 = RET;
_5 = !RET.1;
_1 = _3.fld2.0;
_3.fld2.0 = !_4;
RET = (_3.fld0.0, _5, _3.fld0.2);
_3.fld1 = (-2628957304307417239_i64) as u16;
_4 = _1 >> RET.1;
_1 = _4;
_3.fld0.2 = RET.2 + RET.2;
match RET.2 {
2614988056554778044 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET.1 = _3.fld0.1 >> _3.fld2.0;
_8 = _3.fld3.0;
_3.fld3 = (_8,);
_13.0 = !_6.0;
_3.fld0.2 = _7 as u64;
_3.fld0.2 = RET.2 >> _1;
match RET.2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
2614988056554778044 => bb11,
_ => bb10
}
}
bb4 = {
Return()
}
bb5 = {
RET.1 = _3.fld0.1;
RET = _3.fld0;
_3.fld0 = RET;
_5 = !RET.1;
_1 = _3.fld2.0;
_3.fld2.0 = !_4;
RET = (_3.fld0.0, _5, _3.fld0.2);
_3.fld1 = (-2628957304307417239_i64) as u16;
_4 = _1 >> RET.1;
_1 = _4;
_3.fld0.2 = RET.2 + RET.2;
match RET.2 {
2614988056554778044 => bb3,
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
_3.fld3 = (_8,);
_9 = 962106393251464039_i64 as isize;
_12 = [17_i8];
_3.fld0.2 = RET.2 * RET.2;
RET.0 = _3.fld0.0 << RET.1;
_6.0 = !_13.0;
_13 = _6;
_5 = RET.1 + _3.fld0.1;
_14 = _3.fld0.2;
RET = _3.fld0;
RET = (_3.fld0.0, _5, _3.fld0.2);
_13.0 = _6.0 | _6.0;
_5 = RET.1 << _3.fld2.0;
RET.1 = _5;
_3.fld1 = 142457171361230657059244571949982375522_u128 as u16;
_14 = _3.fld0.2;
_3.fld2 = (_4,);
_15 = _7 + _7;
_10 = '\u{67db0}';
_17.fld4.fld1 = _3.fld1 + _3.fld1;
RET.0 = -_3.fld0.0;
_16 = (_6.0,);
_17.fld0.0 = (-6511663236905446816_i64) as isize;
Goto(bb12)
}
bb12 = {
Call(_18 = dump_var(2_usize, 12_usize, Move(_12), 8_usize, Move(_8), 14_usize, Move(_14), 16_usize, Move(_16)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_18 = dump_var(2_usize, 6_usize, Move(_6), 5_usize, Move(_5), 19_usize, _19, 19_usize, _19), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i128,mut _2: u32,mut _3: i32,mut _4: isize,mut _5: u32,mut _6: isize) -> [i128; 5] {
mir! {
type RET = [i128; 5];
let _7: isize;
let _8: u128;
let _9: Adt54;
let _10: Adt56;
let _11: u128;
let _12: isize;
let _13: i16;
let _14: isize;
let _15: Adt44;
let _16: Adt47;
let _17: f32;
let _18: f32;
let _19: Adt44;
let _20: isize;
let _21: [u64; 4];
let _22: f32;
let _23: isize;
let _24: usize;
let _25: u128;
let _26: f32;
let _27: Adt42;
let _28: Adt41;
let _29: isize;
let _30: ([i128; 1], [i128; 1]);
let _31: [isize; 1];
let _32: f32;
let _33: ();
let _34: ();
{
_4 = _6;
RET = [_1,_1,_1,_1,_1];
_3 = '\u{4b203}' as i32;
_5 = !_2;
RET = [_1,_1,_1,_1,_1];
_5 = _2 >> _4;
RET = [_1,_1,_1,_1,_1];
_7 = _6;
_2 = !_5;
RET = [_1,_1,_1,_1,_1];
_7 = _2 as isize;
Goto(bb1)
}
bb1 = {
_10.fld4.fld0.2 = 10917192801660885225_u64;
_10.fld4.fld1 = 14210_u16 + 55139_u16;
_4 = 249_u8 as isize;
_6 = _7 + _7;
RET = [_1,_1,_1,_1,_1];
_10.fld0.0 = '\u{c003a}' as isize;
_4 = _3 as isize;
_10.fld0 = (_6,);
_10.fld4.fld0.1 = _10.fld4.fld1 as i128;
_8 = '\u{bbe7b}' as u128;
_10.fld4.fld0 = ((-22702_i16), _1, 9295416366977261527_u64);
_10.fld1 = '\u{91214}';
_10.fld2 = _10.fld4.fld1;
_10.fld4.fld0.1 = _1 - _1;
_10.fld4.fld2.0 = -_6;
RET = [_10.fld4.fld0.1,_10.fld4.fld0.1,_1,_10.fld4.fld0.1,_10.fld4.fld0.1];
RET = [_10.fld4.fld0.1,_1,_10.fld4.fld0.1,_10.fld4.fld0.1,_10.fld4.fld0.1];
_5 = !_2;
_10.fld4.fld3.0 = [_10.fld4.fld0.1,_10.fld4.fld0.1,_10.fld4.fld0.1,_10.fld4.fld0.1,_10.fld4.fld0.1];
_7 = 4_u8 as isize;
_2 = !_5;
_10.fld4.fld0 = ((-10246_i16), _1, 11380038416831763680_u64);
_10.fld0 = (_7,);
_10.fld4.fld2 = (_6,);
Call(_9 = fn4(_6, Move(_10.fld4), _6, _6, _6, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10.fld0.0 = _10.fld2 as isize;
SetDiscriminant(_9, 0);
Goto(bb3)
}
bb3 = {
_10.fld4.fld0.2 = _10.fld1 as u64;
_10.fld4.fld2.0 = !_6;
_10.fld4.fld3 = (RET,);
_13 = !(-22974_i16);
place!(Field::<[i128; 1]>(Variant(_9, 0), 1)) = [_1];
_10.fld4.fld0.1 = !_1;
_14 = _13 as isize;
_10.fld4.fld0.0 = !_13;
place!(Field::<i128>(Variant(_9, 0), 0)) = _8 as i128;
Goto(bb4)
}
bb4 = {
RET = _10.fld4.fld3.0;
place!(Field::<[i128; 5]>(Variant(_9, 0), 2)) = [_1,_1,_1,_1,_1];
_8 = 139618311372238585882258053561623226916_u128 | 96351655379089662940815785754224098024_u128;
_10.fld4.fld1 = _10.fld2 * _10.fld2;
_12 = _6;
_10.fld4.fld0.0 = _8 as i16;
SetDiscriminant(_9, 0);
RET = [_1,_10.fld4.fld0.1,_1,_1,_1];
_10.fld4.fld0.2 = 13218963043472592017_u64;
_17 = _10.fld4.fld0.0 as f32;
_18 = _17;
Call(place!(Field::<i128>(Variant(_9, 0), 0)) = fn8(_10.fld4.fld2, _10.fld0, _10.fld4.fld3, _1, _10.fld4.fld0, _10.fld0, _10.fld4.fld2.0, _13, _6, _6, _12, _6, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_10.fld2 = _10.fld4.fld0.0 as u16;
_10.fld4.fld1 = _10.fld2 | _10.fld2;
_6 = true as isize;
_11 = _8;
_10.fld4.fld0 = (_13, Field::<i128>(Variant(_9, 0), 0), 3230469614863393797_u64);
_10.fld4.fld2 = _10.fld0;
_7 = _10.fld4.fld2.0;
_2 = _5 + _5;
_10.fld4.fld0.0 = 5703732128772556517_usize as i16;
_15 = Adt44::Variant1 { fld0: _17,fld1: _5 };
_18 = _10.fld4.fld0.0 as f32;
_1 = _11 as i128;
_2 = 16_i8 as u32;
SetDiscriminant(_15, 1);
_14 = _7 + _10.fld4.fld2.0;
_10.fld3 = [_4];
_10.fld4.fld0.1 = (-109_i8) as i128;
_6 = 13447971949414391327_usize as isize;
_10.fld4.fld1 = _10.fld2;
_10.fld4.fld0 = (_13, Field::<i128>(Variant(_9, 0), 0), 2502841776490164120_u64);
place!(Field::<[i128; 1]>(Variant(_9, 0), 1)) = [_10.fld4.fld0.1];
Goto(bb6)
}
bb6 = {
_10.fld4.fld2 = (_12,);
_13 = _10.fld4.fld0.0 + _10.fld4.fld0.0;
_10.fld4.fld0 = (_13, Field::<i128>(Variant(_9, 0), 0), 13900970789757490719_u64);
_10.fld4.fld2 = (_12,);
_8 = true as u128;
_17 = 98_u8 as f32;
_2 = _5;
_10.fld4.fld3.0 = RET;
_5 = !_2;
_10.fld4.fld3.0 = [Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0),_10.fld4.fld0.1,_10.fld4.fld0.1,Field::<i128>(Variant(_9, 0), 0)];
_5 = _2;
_10.fld4.fld0.2 = 6928407660735442838_u64 - 242374247982940697_u64;
_10.fld4.fld1 = !_10.fld2;
place!(Field::<[i128; 5]>(Variant(_9, 0), 2)) = _10.fld4.fld3.0;
place!(Field::<f32>(Variant(_15, 1), 0)) = _10.fld4.fld1 as f32;
_10.fld4.fld3 = (Field::<[i128; 5]>(Variant(_9, 0), 2),);
Goto(bb7)
}
bb7 = {
_10.fld2 = _10.fld1 as u16;
_10.fld4.fld2 = (_12,);
SetDiscriminant(_9, 0);
place!(Field::<u32>(Variant(_15, 1), 1)) = !_2;
_5 = _2;
_6 = !_10.fld4.fld2.0;
SetDiscriminant(_15, 0);
place!(Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3)).5 = (_10.fld4.fld3.0,);
_22 = _17;
_10.fld4.fld2 = (_12,);
_13 = !_10.fld4.fld0.0;
Call(place!(Field::<*mut *mut bool>(Variant(_15, 0), 2)) = fn10(_10.fld4.fld2, _10.fld4.fld0, _10.fld4.fld2.0, Move(_10)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10.fld4.fld2 = (_6,);
place!(Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3)).3.2 = 2624196492771842808_u64 - 15745650104826253339_u64;
place!(Field::<u128>(Variant(_15, 0), 0)) = _11;
place!(Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3)).4 = [true,false,true,false,false,true];
place!(Field::<i128>(Variant(_9, 0), 0)) = _11 as i128;
_10.fld1 = '\u{b58c7}';
_14 = (-3656663809783722144_i64) as isize;
Goto(bb9)
}
bb9 = {
_27.fld2 = (_12,);
RET = Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3).5.0;
_10.fld3 = [_27.fld2.0];
place!(Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3)).3.2 = !16324336702976315439_u64;
_10.fld4.fld3.0 = [Field::<i128>(Variant(_9, 0), 0),_1,Field::<i128>(Variant(_9, 0), 0),_1,_1];
_27.fld3.0 = [Field::<i128>(Variant(_9, 0), 0),_1,_1,Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0)];
RET = [_1,Field::<i128>(Variant(_9, 0), 0),Field::<i128>(Variant(_9, 0), 0),_1,_1];
place!(Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3)).1 = [true,false,false,false,false,false];
place!(Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3)).0 = [_2,_5];
place!(Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3)).3.0 = _13;
place!(Field::<[i128; 1]>(Variant(_9, 0), 1)) = [_1];
_10.fld4.fld1 = _6 as u16;
_10.fld1 = '\u{1448d}';
place!(Field::<[i128; 1]>(Variant(_9, 0), 1)) = [_1];
RET = Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3).5.0;
_27.fld2 = (_6,);
_10.fld4.fld3 = (Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3).5.0,);
_27.fld0.1 = Field::<i128>(Variant(_9, 0), 0);
_17 = _22;
_24 = !3_usize;
place!(Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3)).0 = [_5,_2];
_25 = _11 + _8;
_27.fld3 = Field::<([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize)>(Variant(_15, 0), 3).5;
_10.fld2 = _10.fld4.fld1;
_26 = 8_i8 as f32;
Goto(bb10)
}
bb10 = {
Call(_33 = dump_var(3_usize, 12_usize, Move(_12), 8_usize, Move(_8), 13_usize, Move(_13), 1_usize, Move(_1)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_33 = dump_var(3_usize, 5_usize, Move(_5), 25_usize, Move(_25), 4_usize, Move(_4), 34_usize, _34), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: Adt42,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [i128; 5]) -> Adt54 {
mir! {
type RET = Adt54;
let _7: [i128; 5];
let _8: u128;
let _9: char;
let _10: bool;
let _11: u64;
let _12: ([i128; 5],);
let _13: [i128; 1];
let _14: Adt50;
let _15: Adt40;
let _16: bool;
let _17: ();
let _18: ();
{
_6 = _2.fld3.0;
_2.fld3 = (_6,);
_2.fld3.0 = [_2.fld0.1,_2.fld0.1,_2.fld0.1,_2.fld0.1,_2.fld0.1];
_3 = _2.fld0.0 as isize;
_3 = -_4;
Call(_5 = fn5(_2.fld3, _4, _2.fld0, _4, _1, _2.fld2.0, _6, _2.fld0.2, _1, _1, _6, _3, _2.fld2, _2.fld2.0, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _3 - _4;
_2.fld1 = 15001200649726075794_usize as u16;
_2.fld2.0 = _5 - _4;
_2.fld1 = 254617519933397009026791995305582207918_u128 as u16;
Goto(bb2)
}
bb2 = {
_2.fld1 = _2.fld0.2 as u16;
Call(_4 = fn6(_2.fld2, _2.fld2, _2.fld2.0, _5, _2.fld2, _2.fld2, Move(_2), _3, _1, _5, _5, _3, _3, _1, _5, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2.fld0 = (32412_i16, 85398102239813832603241534662077540001_i128, 547501938627721650_u64);
_9 = '\u{1245f}';
_2.fld2 = (_4,);
_10 = !true;
_1 = -_4;
_5 = -_2.fld2.0;
_10 = false;
_8 = !221641788181118598041032990362369494549_u128;
_7 = [_2.fld0.1,_2.fld0.1,_2.fld0.1,_2.fld0.1,_2.fld0.1];
_7 = [_2.fld0.1,_2.fld0.1,_2.fld0.1,_2.fld0.1,_2.fld0.1];
_8 = !280121017238175867201658839084785961900_u128;
_8 = 23860125758941962700983991072693291219_u128 & 262194470115159593978850125597006292527_u128;
_2.fld1 = _2.fld0.2 as u16;
_2.fld1 = !53707_u16;
_3 = !_2.fld2.0;
match _2.fld0.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
85398102239813832603241534662077540001 => bb8,
_ => bb7
}
}
bb4 = {
_2.fld1 = _2.fld0.2 as u16;
Call(_4 = fn6(_2.fld2, _2.fld2, _2.fld2.0, _5, _2.fld2, _2.fld2, Move(_2), _3, _1, _5, _5, _3, _3, _1, _5, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5 = _3 - _4;
_2.fld1 = 15001200649726075794_usize as u16;
_2.fld2.0 = _5 - _4;
_2.fld1 = 254617519933397009026791995305582207918_u128 as u16;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_2.fld3 = (_7,);
_2.fld0 = ((-7675_i16), 94136516948476017046050450551315200631_i128, 15848834368858212749_u64);
_2.fld1 = !22727_u16;
_2.fld0.2 = !9903939471051334570_u64;
_12.0 = _2.fld3.0;
_2.fld2.0 = _3;
_2.fld1 = !26_u16;
_2.fld2 = (_4,);
_8 = !180849679530900699506282368134934383682_u128;
_12.0 = _6;
_6 = [_2.fld0.1,_2.fld0.1,_2.fld0.1,_2.fld0.1,_2.fld0.1];
_7 = _12.0;
_2.fld0.2 = 15528045181113532529_u64;
_2.fld2 = (_4,);
_5 = 4_usize as isize;
_5 = _2.fld2.0 << _3;
_11 = 1_usize as u64;
_12 = (_6,);
_2.fld0 = ((-9260_i16), (-49753319773004017266603383928033099346_i128), _11);
_9 = '\u{3f7a8}';
Call(_11 = fn7(_3, _2.fld2, _5, _4, _5, _5, Move(_2), _1, _5, _4, _4, _5), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2.fld3 = _12;
_12 = _2.fld3;
_2.fld2.0 = _5;
_3 = _9 as isize;
_2.fld1 = 36026_u16;
_1 = _8 as isize;
_13 = [72280875414369743223230399352840129782_i128];
_2.fld2.0 = (-21955_i16) as isize;
_2.fld0 = (9940_i16, (-83016105003832175651833913561770781262_i128), _11);
_2.fld2.0 = _2.fld1 as isize;
RET = Adt54::Variant0 { fld0: _2.fld0.1,fld1: _13,fld2: _7 };
place!(Field::<[i128; 5]>(Variant(RET, 0), 2)) = [_2.fld0.1,Field::<i128>(Variant(RET, 0), 0),_2.fld0.1,_2.fld0.1,_2.fld0.1];
place!(Field::<[i128; 5]>(Variant(RET, 0), 2)) = _6;
place!(Field::<[i128; 1]>(Variant(RET, 0), 1)) = [Field::<i128>(Variant(RET, 0), 0)];
_11 = _2.fld0.2;
place!(Field::<i128>(Variant(RET, 0), 0)) = 5_usize as i128;
Goto(bb10)
}
bb10 = {
Call(_17 = dump_var(4_usize, 4_usize, Move(_4), 12_usize, Move(_12), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_17 = dump_var(4_usize, 7_usize, Move(_7), 9_usize, Move(_9), 18_usize, _18, 18_usize, _18), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: ([i128; 5],),mut _2: isize,mut _3: (i16, i128, u64),mut _4: isize,mut _5: isize,mut _6: isize,mut _7: [i128; 5],mut _8: u64,mut _9: isize,mut _10: isize,mut _11: [i128; 5],mut _12: isize,mut _13: (isize,),mut _14: isize,mut _15: isize) -> isize {
mir! {
type RET = isize;
let _16: Adt40;
let _17: i16;
let _18: char;
let _19: char;
let _20: ();
let _21: ();
{
_14 = !_6;
_3 = ((-6606_i16), 167520713822105905461986453719633874163_i128, _8);
RET = _4;
_12 = RET * _10;
_15 = _4;
_17 = _3.0;
_13 = (_4,);
_15 = _10;
_11 = [_3.1,_3.1,_3.1,_3.1,_3.1];
_6 = _5;
_17 = 7236492082974327199_i64 as i16;
_8 = _3.1 as u64;
_3.2 = _8 >> _15;
_12 = _3.2 as isize;
_3 = (_17, (-154207158885896484044515298864588115279_i128), _8);
_1.0 = [_3.1,_3.1,_3.1,_3.1,_3.1];
RET = !_9;
_15 = '\u{17085}' as isize;
_9 = -_12;
_18 = '\u{774c2}';
_17 = -_3.0;
_2 = -_12;
RET = _14 & _12;
_19 = _18;
_9 = !_12;
_10 = _13.0 & _9;
_5 = _10 << _6;
_13 = (_5,);
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(5_usize, 2_usize, Move(_2), 18_usize, Move(_18), 14_usize, Move(_14), 19_usize, Move(_19)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(5_usize, 8_usize, Move(_8), 17_usize, Move(_17), 15_usize, Move(_15), 4_usize, Move(_4)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(5_usize, 12_usize, Move(_12), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: (isize,),mut _2: (isize,),mut _3: isize,mut _4: isize,mut _5: (isize,),mut _6: (isize,),mut _7: Adt42,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize,mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: *mut bool;
let _18: ();
let _19: ();
{
_13 = _3 * _5.0;
_7.fld2.0 = _11;
_12 = _6.0;
match _7.fld0.0 {
340282366920938463463374607431768201210 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET = -_2.0;
_7.fld3.0 = [_7.fld0.1,_7.fld0.1,_7.fld0.1,_7.fld0.1,_7.fld0.1];
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(6_usize, 8_usize, Move(_8), 16_usize, Move(_16), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_18 = dump_var(6_usize, 1_usize, Move(_1), 4_usize, Move(_4), 12_usize, Move(_12), 19_usize, _19), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: (isize,),mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: Adt42,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> u64 {
mir! {
type RET = u64;
let _13: f32;
let _14: Adt43;
let _15: [u32; 2];
let _16: Adt53;
let _17: ();
let _18: ();
{
match _7.fld0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768202196 => bb7,
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
_7.fld1 = 48787_u16 | 50453_u16;
_6 = _5;
_9 = 285280366730818998000638992628823031154_u128 as isize;
_2 = _7.fld2;
_7.fld1 = !35418_u16;
_7.fld0.2 = 15325051616210437903_u64 ^ 6759646596671832931_u64;
_7.fld0 = ((-4423_i16), 33638916699382880664307597158410219605_i128, 3587825049631103864_u64);
_2.0 = 7_usize as isize;
_12 = !_7.fld2.0;
RET = _7.fld0.2 + _7.fld0.2;
_3 = _6 + _10;
_7.fld0.0 = 636086801_u32 as i16;
_2 = (_12,);
_7.fld0.2 = RET;
_6 = !_5;
_4 = -_3;
_13 = 2391469023_u32 as f32;
_7.fld0 = (1626_i16, 131550637112263229093687318171390296473_i128, RET);
_7.fld2.0 = _3;
_7.fld1 = RET as u16;
_7.fld3.0 = [_7.fld0.1,_7.fld0.1,_7.fld0.1,_7.fld0.1,_7.fld0.1];
_7.fld0.1 = -76232697800822992501929432667587888779_i128;
match _7.fld0.0 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb8,
4 => bb9,
5 => bb10,
1626 => bb12,
_ => bb11
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
_7.fld0.2 = RET | RET;
_5 = 6909312245294338677_i64 as isize;
_2.0 = !_4;
_1 = _12 * _5;
_7.fld1 = !61226_u16;
_7.fld1 = 89998106_u32 as u16;
_15 = [304968819_u32,4127065603_u32];
_15 = [3467600281_u32,3593372505_u32];
_10 = _7.fld1 as isize;
_7.fld0.2 = RET * RET;
_4 = _3 + _2.0;
_7.fld0.1 = !(-81139742704100037353200987412187071871_i128);
RET = !_7.fld0.2;
_12 = -_3;
_7.fld1 = !32677_u16;
Goto(bb13)
}
bb13 = {
Call(_17 = dump_var(7_usize, 10_usize, Move(_10), 11_usize, Move(_11), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_17 = dump_var(7_usize, 2_usize, Move(_2), 1_usize, Move(_1), 18_usize, _18, 18_usize, _18), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: (isize,),mut _2: (isize,),mut _3: ([i128; 5],),mut _4: i128,mut _5: (i16, i128, u64),mut _6: (isize,),mut _7: isize,mut _8: i16,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize) -> i128 {
mir! {
type RET = i128;
let _14: [bool; 6];
let _15: [u64; 4];
let _16: char;
let _17: bool;
let _18: f32;
let _19: ();
let _20: ();
{
_2.0 = 11052080905219134357_usize as isize;
Call(_9 = fn9(_1.0, _10, _11, _1.0, _12, _5, _1.0, _10, _3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.0 = 5_usize as isize;
_7 = -_12;
_2 = _1;
_14 = [true,false,false,true,false,false];
_1 = (_10,);
_8 = !_5.0;
_2 = _1;
_6.0 = _7 | _13;
match _5.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
13218963043472592017 => bb8,
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
_14 = [false,true,false,false,false,false];
_4 = -_5.1;
_8 = 208041519_i32 as i16;
_6 = _1;
_5 = (_8, _4, 15722229868963013992_u64);
_12 = _1.0 >> _5.0;
_12 = -_11;
RET = _5.1;
_6 = (_11,);
_2.0 = true as isize;
_1 = (_9,);
_8 = _5.0 + _5.0;
RET = _5.1 ^ _4;
_7 = -_10;
_14 = [false,true,true,false,true,false];
_6.0 = _1.0;
_15 = [_5.2,_5.2,_5.2,_5.2];
_15 = [_5.2,_5.2,_5.2,_5.2];
_5.1 = RET;
_11 = _10 & _6.0;
_6.0 = _13 << _5.0;
_14 = [true,false,true,false,false,true];
_2.0 = _11;
_2 = (_9,);
Goto(bb9)
}
bb9 = {
RET = 291278165455677170452568182248361843416_u128 as i128;
_6.0 = 170379484289185097004639855711567335827_u128 as isize;
_5.2 = 15419808274302322966_u64 >> _9;
_10 = !_1.0;
_5.1 = _4;
RET = _5.1 >> _2.0;
Goto(bb10)
}
bb10 = {
Call(_19 = dump_var(8_usize, 8_usize, Move(_8), 7_usize, Move(_7), 12_usize, Move(_12), 11_usize, Move(_11)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_19 = dump_var(8_usize, 15_usize, Move(_15), 9_usize, Move(_9), 1_usize, Move(_1), 20_usize, _20), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: (i16, i128, u64),mut _7: isize,mut _8: isize,mut _9: ([i128; 5],),mut _10: isize) -> isize {
mir! {
type RET = isize;
let _11: i8;
let _12: Adt52;
let _13: i32;
let _14: f64;
let _15: Adt52;
let _16: ();
let _17: ();
{
_10 = _3;
_6 = ((-19706_i16), (-74927362225612569199346821676287247732_i128), 8613786897537884619_u64);
_6.1 = (-114970357948272539438733163355775896138_i128);
_1 = 63259_u16 as isize;
_6 = ((-8883_i16), (-83749097259744386168281570959275122646_i128), 7595907772456918650_u64);
RET = _5;
_6.2 = !14956107255106188547_u64;
_6 = (5910_i16, 147602472019113181723567089827943605464_i128, 18205716276122431714_u64);
Goto(bb1)
}
bb1 = {
_1 = RET;
_4 = _8 + _3;
_6 = (3091_i16, (-11628694029974624652741441333558271820_i128), 10303138652669915780_u64);
_6.0 = (-5193_i16) << _8;
_7 = _2;
_9.0 = [_6.1,_6.1,_6.1,_6.1,_6.1];
_1 = (-795347189_i32) as isize;
_6 = ((-5075_i16), (-69983657260299250516679781122944977695_i128), 14367909228342848278_u64);
_6 = (12612_i16, 95015151162850158255465828450381445372_i128, 125756481779515199_u64);
_1 = (-439064884_i32) as isize;
_3 = _4 - RET;
_6 = (23908_i16, 45311687329773137470991129353838762803_i128, 7287642206901184733_u64);
match _6.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
7287642206901184733 => bb10,
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
_13 = (-1858395143_i32) ^ (-74842348_i32);
_2 = _10 - _5;
_6.2 = !5050018132978664251_u64;
_5 = _8 ^ _3;
RET = -_3;
Goto(bb11)
}
bb11 = {
Call(_16 = dump_var(9_usize, 9_usize, Move(_9), 2_usize, Move(_2), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_16 = dump_var(9_usize, 7_usize, Move(_7), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (isize,),mut _2: (i16, i128, u64),mut _3: isize,mut _4: Adt56) -> *mut *mut bool {
mir! {
type RET = *mut *mut bool;
let _5: *mut u128;
let _6: i32;
let _7: *mut u128;
let _8: u32;
let _9: i16;
let _10: (i16, i128, u64);
let _11: (isize,);
let _12: [i128; 1];
let _13: Adt42;
let _14: *mut [u64; 4];
let _15: [isize; 1];
let _16: Adt52;
let _17: *const *mut bool;
let _18: i32;
let _19: u8;
let _20: f32;
let _21: Adt53;
let _22: Adt47;
let _23: &'static i64;
let _24: char;
let _25: bool;
let _26: i16;
let _27: Adt49;
let _28: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize);
let _29: isize;
let _30: [i128; 5];
let _31: u8;
let _32: Adt41;
let _33: u128;
let _34: isize;
let _35: isize;
let _36: u32;
let _37: *mut *mut bool;
let _38: isize;
let _39: [u8; 4];
let _40: Adt47;
let _41: i64;
let _42: f64;
let _43: u32;
let _44: u8;
let _45: Adt43;
let _46: char;
let _47: Adt45;
let _48: Adt43;
let _49: Adt40;
let _50: Adt46;
let _51: f64;
let _52: char;
let _53: [u32; 2];
let _54: [i8; 5];
let _55: ();
let _56: ();
{
_2 = _4.fld4.fld0;
_4.fld4.fld0.1 = _2.1;
_4.fld4.fld3.0 = [_4.fld4.fld0.1,_2.1,_4.fld4.fld0.1,_4.fld4.fld0.1,_4.fld4.fld0.1];
_4.fld2 = 7_usize as u16;
_4.fld4.fld1 = _4.fld2;
_2.2 = _4.fld4.fld0.2;
_8 = 1025299997_u32 >> _4.fld4.fld0.1;
_4.fld0 = (_1.0,);
_1 = (_3,);
_4.fld4.fld0.1 = 6249452314043523903_i64 as i128;
_4.fld4.fld2.0 = -_1.0;
_4.fld0.0 = _3;
_4.fld4.fld0.1 = _2.1 | _2.1;
_2.2 = _4.fld4.fld0.2;
_10.0 = _4.fld4.fld0.0;
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_1.0 = _3 & _3;
_11.0 = _1.0;
_4.fld4.fld0.2 = 0_usize as u64;
_10.2 = _2.2;
_2 = _4.fld4.fld0;
_9 = !_2.0;
_4.fld4.fld0.1 = -_2.1;
_11 = (_4.fld0.0,);
_13.fld2 = _4.fld4.fld2;
_4.fld4.fld1 = !_4.fld2;
_13.fld3 = (_4.fld4.fld3.0,);
_13.fld3.0 = [_2.1,_2.1,_2.1,_2.1,_2.1];
Goto(bb3)
}
bb3 = {
_4.fld3 = [_4.fld0.0];
_13.fld0 = (_2.0, _2.1, _10.2);
_12 = [_13.fld0.1];
_2.1 = _4.fld4.fld0.1 >> _4.fld4.fld0.1;
_2 = (_13.fld0.0, _13.fld0.1, _4.fld4.fld0.2);
_10 = (_4.fld4.fld0.0, _13.fld0.1, _13.fld0.2);
_4.fld4.fld2 = (_1.0,);
_15 = [_13.fld2.0];
_1 = _4.fld4.fld2;
_4.fld0 = (_1.0,);
_13.fld0.1 = 307198417817180675159309201744916301208_u128 as i128;
_3 = _1.0;
_4.fld4.fld3 = _13.fld3;
_9 = !_13.fld0.0;
_4.fld2 = _4.fld4.fld1 >> _2.1;
_11 = (_4.fld4.fld2.0,);
_11 = (_1.0,);
_11.0 = (-1291152530_i32) as isize;
_13 = Move(_4.fld4);
_4.fld4.fld0 = (_13.fld0.0, _13.fld0.1, _10.2);
_1.0 = 9053951048774134553_i64 as isize;
Call(_2.0 = fn11(Move(_13), _4.fld3, _4.fld4.fld0, _4.fld4.fld0, _10.2, _1.0, _8, _2.1, _4.fld2, _4.fld1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10 = (_9, _4.fld4.fld0.1, _2.2);
_13.fld0.1 = _10.1;
_9 = _10.0;
_10.0 = _9 * _4.fld4.fld0.0;
_3 = _4.fld0.0 << _4.fld2;
RET = core::ptr::addr_of_mut!(_21.fld6.fld2);
_4.fld4.fld1 = true as u16;
_21.fld6.fld6 = (-15_i8) as f64;
_2 = (_9, _10.1, _4.fld4.fld0.2);
_13.fld0.0 = !_10.0;
_21.fld6.fld5 = [_2.1,_13.fld0.1,_13.fld0.1,_2.1,_13.fld0.1];
_18 = (-240965930_i32) + 2096825184_i32;
_6 = _18;
_4.fld3 = [_3];
_17 = core::ptr::addr_of!(_21.fld0);
_13.fld0.1 = _4.fld4.fld0.1;
_4.fld4.fld3.0 = [_4.fld4.fld0.1,_2.1,_13.fld0.1,_4.fld4.fld0.1,_13.fld0.1];
Goto(bb5)
}
bb5 = {
Goto(bb6)
}
bb6 = {
_4.fld4.fld3 = (_21.fld6.fld5,);
_13.fld0.0 = !_9;
_4.fld4.fld2 = (_3,);
_2 = _4.fld4.fld0;
_4.fld4.fld2.0 = 2_usize as isize;
_13.fld1 = false as u16;
_21.fld6.fld3 = [16_u8,218_u8,217_u8,157_u8];
_28.2 = _21.fld6.fld6 * _21.fld6.fld6;
_28.5.0 = [_4.fld4.fld0.1,_10.1,_4.fld4.fld0.1,_2.1,_2.1];
_6 = _4.fld2 as i32;
_21.fld6.fld2 = core::ptr::addr_of_mut!(_25);
_21.fld6.fld4 = (_4.fld0.0,);
_28.3.1 = _13.fld0.1 | _4.fld4.fld0.1;
_21.fld1 = _4.fld1;
_21.fld4 = Adt45::Variant0 { fld0: 263406156719709216154019630320761909955_u128 };
_4.fld4.fld2.0 = -_3;
_20 = _4.fld4.fld0.2 as f32;
_11 = (_3,);
_29 = 4032068562276595549_i64 as isize;
_24 = _21.fld1;
Goto(bb7)
}
bb7 = {
_28.5.0 = [_13.fld0.1,_13.fld0.1,_13.fld0.1,_28.3.1,_13.fld0.1];
_10 = _2;
_21.fld6.fld5 = [_28.3.1,_4.fld4.fld0.1,_4.fld4.fld0.1,_10.1,_13.fld0.1];
_4.fld4.fld3 = (_21.fld6.fld5,);
_26 = _10.0;
_13.fld2 = (_29,);
_3 = -_11.0;
_28.3 = _10;
_25 = true;
_12 = [_2.1];
_25 = false;
_4.fld4.fld2.0 = _29 >> _4.fld2;
_21.fld6.fld2 = core::ptr::addr_of_mut!(_25);
_4.fld4.fld1 = _4.fld2;
Goto(bb8)
}
bb8 = {
place!(Field::<u128>(Variant(_21.fld4, 0), 0)) = 67801113677349792160749328963265116675_u128 & 139990231940365573731347020242251294006_u128;
_13.fld0.2 = _10.2;
_36 = _8;
_15 = [_3];
SetDiscriminant(_21.fld4, 0);
_3 = _11.0;
_10 = (_4.fld4.fld0.0, _2.1, _2.2);
_28.2 = _21.fld6.fld6 * _21.fld6.fld6;
_19 = _4.fld2 as u8;
_28.0 = [_8,_36];
_13.fld3.0 = _21.fld6.fld5;
_4.fld4.fld1 = !_4.fld2;
_10 = _4.fld4.fld0;
_28.5 = (_21.fld6.fld5,);
_4.fld4.fld0 = (_13.fld0.0, _13.fld0.1, _13.fld0.2);
_21.fld6.fld4 = (_3,);
_13.fld0.2 = _2.2 | _10.2;
_13.fld0.2 = _10.2 | _4.fld4.fld0.2;
_1 = (_3,);
_24 = _4.fld1;
_21.fld0 = (*RET);
place!(Field::<u128>(Variant(_21.fld4, 0), 0)) = 317604012926698736525463044617324619618_u128 | 241758059111753032219326045724811577364_u128;
_16 = Adt52::Variant1 { fld0: _15 };
_28.3 = (_10.0, _2.1, _10.2);
_13.fld3.0 = [_4.fld4.fld0.1,_4.fld4.fld0.1,_28.3.1,_10.1,_4.fld4.fld0.1];
_10 = (_2.0, _2.1, _4.fld4.fld0.2);
_28.3 = _13.fld0;
Goto(bb9)
}
bb9 = {
_28.0 = [_36,_36];
_21.fld5 = Adt49::Variant0 { fld0: _12,fld1: 2304555015987805422_usize,fld2: _13.fld0.2 };
_31 = !_19;
_28.4 = [_25,_25,_25,_25,_25,_25];
_30 = _4.fld4.fld3.0;
_30 = _28.5.0;
_21.fld2 = [_31,_31,_19,_31,_19,_19,_31];
place!(Field::<[isize; 1]>(Variant(_16, 1), 0)) = _15;
place!(Field::<usize>(Variant(_21.fld5, 0), 1)) = _4.fld4.fld1 as usize;
_39 = _21.fld6.fld3;
_3 = _11.0 >> _1.0;
_18 = _6 >> _6;
_4.fld4.fld0 = (_9, _13.fld0.1, Field::<u64>(Variant(_21.fld5, 0), 2));
place!(Field::<[i128; 1]>(Variant(_21.fld5, 0), 0)) = _12;
_13.fld3.0 = [_2.1,_10.1,_28.3.1,_13.fld0.1,_28.3.1];
_35 = 42_i8 as isize;
_13.fld3.0 = [_28.3.1,_28.3.1,_28.3.1,_13.fld0.1,_28.3.1];
_41 = 6152862423470837235_i64 * (-1300312249375888314_i64);
Call(_34 = fn17(_4.fld4.fld3, _18), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19 = _31 ^ _31;
_11.0 = _4.fld4.fld2.0;
_11.0 = _13.fld0.0 as isize;
_4.fld4.fld0.1 = !_13.fld0.1;
_21.fld4 = Adt45::Variant0 { fld0: 19817964879636847036080842344509257604_u128 };
_21.fld6.fld1 = Adt45::Variant0 { fld0: 336388205497624455568810617971981880216_u128 };
_21.fld6.fld5 = [_4.fld4.fld0.1,_2.1,_10.1,_28.3.1,_4.fld4.fld0.1];
Goto(bb11)
}
bb11 = {
_37 = core::ptr::addr_of_mut!((*RET));
_46 = _24;
place!(Field::<u128>(Variant(_21.fld6.fld1, 0), 0)) = _9 as u128;
_42 = Field::<usize>(Variant(_21.fld5, 0), 1) as f64;
_9 = _20 as i16;
_30 = [_10.1,_13.fld0.1,_4.fld4.fld0.1,_10.1,_13.fld0.1];
place!(Field::<u128>(Variant(_21.fld4, 0), 0)) = Field::<u128>(Variant(_21.fld6.fld1, 0), 0) - Field::<u128>(Variant(_21.fld6.fld1, 0), 0);
_28.1 = _28.4;
_4.fld0.0 = _21.fld6.fld4.0;
_1 = (_4.fld4.fld2.0,);
_33 = Field::<u128>(Variant(_21.fld4, 0), 0) & Field::<u128>(Variant(_21.fld4, 0), 0);
_13.fld0.0 = _4.fld4.fld0.0 + _10.0;
_28.3.1 = _41 as i128;
_30 = _13.fld3.0;
_4.fld4 = Adt42 { fld0: _13.fld0,fld1: _4.fld2,fld2: _21.fld6.fld4,fld3: _28.5 };
_13.fld0.0 = -_9;
Goto(bb12)
}
bb12 = {
_4.fld3 = _15;
_15 = [_3];
_2.1 = -_10.1;
_4.fld4.fld0.1 = -_2.1;
_12 = [_2.1];
_2.0 = -_26;
_28.4 = _28.1;
place!(Field::<u128>(Variant(_21.fld4, 0), 0)) = !_33;
SetDiscriminant(_21.fld4, 2);
SetDiscriminant(_21.fld6.fld1, 2);
_21.fld6.fld0 = [_19,_19,_19,_19,_19,_19,_19];
_28.2 = _42 - _42;
place!(Field::<(i64, *mut u128, u64)>(Variant(_21.fld4, 2), 1)).2 = !_4.fld4.fld0.2;
_21.fld6.fld0 = [_19,_31,_19,_19,_19,_31,_19];
SetDiscriminant(_16, 1);
RET = _37;
place!(Field::<(i64, *mut u128, u64)>(Variant(_21.fld6.fld1, 2), 1)).1 = core::ptr::addr_of_mut!(_33);
_25 = _8 == _8;
RET = _37;
_22 = Adt47::Variant0 { fld0: _25,fld1: _39,fld2: Move(_4.fld4) };
place!(Field::<(isize,)>(Variant(_21.fld6.fld1, 2), 3)) = (_34,);
_21.fld1 = _4.fld1;
_22 = Adt47::Variant0 { fld0: _25,fld1: _21.fld6.fld3,fld2: Move(_13) };
_10 = (_2.0, _2.1, _2.2);
place!(Field::<(i64, *mut u128, u64)>(Variant(_21.fld4, 2), 1)).0 = _20 as i64;
_4.fld4.fld0.1 = _2.1;
place!(Field::<u64>(Variant(_21.fld5, 0), 2)) = Field::<Adt42>(Variant(_22, 0), 2).fld0.2 + Field::<(i64, *mut u128, u64)>(Variant(_21.fld4, 2), 1).2;
_4.fld4.fld3 = (_28.5.0,);
_13.fld0.1 = _10.1;
Goto(bb13)
}
bb13 = {
_4.fld4.fld0.2 = _10.2 << _10.1;
_28.5.0 = _4.fld4.fld3.0;
_4 = Adt56 { fld0: _21.fld6.fld4,fld1: _46,fld2: Field::<Adt42>(Variant(_22, 0), 2).fld1,fld3: _15,fld4: Move(Field::<Adt42>(Variant(_22, 0), 2)) };
_21.fld0 = (*_37);
_19 = _41 as u8;
_30 = _28.5.0;
_8 = _36;
place!(Field::<isize>(Variant(_21.fld4, 2), 2)) = _3 ^ _1.0;
place!(Field::<usize>(Variant(_21.fld5, 0), 1)) = 10251223257103687716_usize >> Field::<isize>(Variant(_21.fld4, 2), 2);
SetDiscriminant(_21.fld5, 1);
place!(Field::<Adt42>(Variant(_22, 0), 2)).fld0.2 = !_2.2;
_10.1 = !_13.fld0.1;
_50.fld4 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(_21.fld4, 2), 0)));
_4.fld4.fld0.0 = _2.0;
_13.fld0 = (_26, _2.1, _4.fld4.fld0.2);
_50.fld0 = _10.1;
_1.0 = _3 - Field::<isize>(Variant(_21.fld4, 2), 2);
place!(Field::<bool>(Variant(_21.fld6.fld1, 2), 0)) = _10.1 == _10.1;
_50.fld1 = [_36,_36];
Goto(bb14)
}
bb14 = {
_19 = !_31;
place!(Field::<(isize,)>(Variant(_21.fld4, 2), 3)) = _4.fld0;
place!(Field::<Adt42>(Variant(_22, 0), 2)).fld0.0 = !_9;
_4.fld4.fld1 = _4.fld2 - _4.fld2;
_13.fld0.2 = Field::<(i64, *mut u128, u64)>(Variant(_21.fld4, 2), 1).2 | _4.fld4.fld0.2;
_51 = _28.2 * _28.2;
_4.fld4.fld1 = !_4.fld2;
_21.fld1 = _4.fld1;
_21.fld6.fld0 = _21.fld2;
_15 = [_34];
_13.fld0.0 = !_9;
_28.3 = (Field::<Adt42>(Variant(_22, 0), 2).fld0.0, _13.fld0.1, _13.fld0.2);
place!(Field::<(i64, *mut u128, u64)>(Variant(_21.fld4, 2), 1)).2 = _13.fld0.2 - _28.3.2;
_4.fld4.fld0 = (_26, _28.3.1, Field::<(i64, *mut u128, u64)>(Variant(_21.fld4, 2), 1).2);
Goto(bb15)
}
bb15 = {
Call(_55 = dump_var(10_usize, 41_usize, Move(_41), 15_usize, Move(_15), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_55 = dump_var(10_usize, 10_usize, Move(_10), 30_usize, Move(_30), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_55 = dump_var(10_usize, 26_usize, Move(_26), 12_usize, Move(_12), 9_usize, Move(_9), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: Adt42,mut _2: [isize; 1],mut _3: (i16, i128, u64),mut _4: (i16, i128, u64),mut _5: u64,mut _6: isize,mut _7: u32,mut _8: i128,mut _9: u16,mut _10: char) -> i16 {
mir! {
type RET = i16;
let _11: *mut u128;
let _12: u16;
let _13: isize;
let _14: char;
let _15: i32;
let _16: u64;
let _17: [i8; 5];
let _18: [i128; 5];
let _19: u128;
let _20: isize;
let _21: bool;
let _22: Adt48;
let _23: Adt42;
let _24: char;
let _25: [u64; 4];
let _26: char;
let _27: isize;
let _28: i32;
let _29: f32;
let _30: char;
let _31: u128;
let _32: ();
let _33: ();
{
_3.2 = _5;
RET = _4.0;
Call(_1.fld3 = fn12(_1.fld0.1, _4.1, _8, _4, _1.fld0, _7, _9, _1.fld0, _3, _1.fld0.1, _1.fld0.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.fld2 = (_6,);
_9 = _7 as u16;
Goto(bb2)
}
bb2 = {
_9 = (-2712594108021153034_i64) as u16;
RET = -_3.0;
Call(_4.1 = core::intrinsics::transmute(_8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1.fld2.0 = -_6;
_1.fld0.2 = _4.2;
_1.fld0.1 = -_8;
_1.fld1 = !_9;
_13 = _9 as isize;
Goto(bb4)
}
bb4 = {
_1.fld1 = _9 >> _8;
RET = _3.0;
_3.2 = _3.1 as u64;
Goto(bb5)
}
bb5 = {
_8 = _3.1 << _4.1;
_6 = _13 ^ _13;
Call(_11 = fn13(_3.2, _1.fld3, Move(_1), _4, _3.1, _3.1, _3, _8, _8, _8, _3.2, _4.1, _3.2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = (RET, _8, _3.2);
_3.0 = RET;
_1.fld0.1 = _4.1;
_15 = (-536261064_i32);
_3.1 = _8 | _1.fld0.1;
match _15 {
0 => bb3,
340282366920938463463374607431231950392 => bb7,
_ => bb2
}
}
bb7 = {
_1.fld2 = (_6,);
_15 = 1646004439_i32;
_16 = _3.2 - _3.2;
_1.fld0.2 = _4.2 >> _1.fld0.1;
_6 = _1.fld2.0;
_4.1 = !_8;
_12 = _9 & _9;
_14 = _10;
_4.1 = (-487498323779239258_i64) as i128;
_4.1 = _3.1 ^ _8;
_17 = [(-15_i8),126_i8,32_i8,90_i8,61_i8];
_2 = [_1.fld2.0];
RET = !_4.0;
_16 = !_3.2;
_4 = _3;
_1.fld3.0 = [_8,_8,_4.1,_1.fld0.1,_8];
_20 = _6;
RET = _3.0 ^ _4.0;
Call(_6 = core::intrinsics::transmute(_1.fld0.2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_9 = _12 + _12;
_18 = _1.fld3.0;
_10 = _14;
_1.fld1 = !_9;
_1.fld3 = (_18,);
_22.fld3 = [28_u8,199_u8,228_u8,232_u8];
_3.1 = _1.fld0.1 + _1.fld0.1;
_16 = _8 as u64;
_1.fld3.0 = [_3.1,_8,_1.fld0.1,_8,_1.fld0.1];
_20 = _13 + _6;
_4 = (_3.0, _1.fld0.1, _16);
_19 = 324135410073576611906134767628366662718_u128 >> _7;
_1.fld2 = (_20,);
_3 = (RET, _1.fld0.1, _16);
_22.fld2 = core::ptr::addr_of_mut!(_21);
_1.fld0 = (RET, _3.1, _5);
Goto(bb9)
}
bb9 = {
_13 = _1.fld2.0;
_22.fld5 = [_4.1,_1.fld0.1,_8,_3.1,_3.1];
_21 = _20 <= _6;
_9 = _3.0 as u16;
_10 = _14;
_1.fld0.1 = _21 as i128;
Goto(bb10)
}
bb10 = {
_12 = !_1.fld1;
_1.fld1 = 39_i8 as u16;
_22.fld0 = [33_u8,149_u8,51_u8,233_u8,196_u8,149_u8,2_u8];
_12 = _9;
_22.fld6 = 115_i8 as f64;
_23.fld0.1 = -_4.1;
_23 = Adt42 { fld0: _1.fld0,fld1: _12,fld2: _1.fld2,fld3: _1.fld3 };
_4 = (RET, _23.fld0.1, _3.2);
_23.fld2 = (_20,);
Goto(bb11)
}
bb11 = {
_4.1 = _3.1 * _1.fld0.1;
_23 = Adt42 { fld0: _3,fld1: _9,fld2: _1.fld2,fld3: _1.fld3 };
_4.0 = _3.0;
_22.fld6 = 10490470682129640067_usize as f64;
_1.fld0.1 = !_23.fld0.1;
_23.fld1 = _12;
_22.fld0 = [169_u8,57_u8,83_u8,205_u8,69_u8,223_u8,186_u8];
_1.fld3 = (_18,);
RET = _1.fld0.0 + _23.fld0.0;
_9 = !_23.fld1;
_3 = (_1.fld0.0, _1.fld0.1, _23.fld0.2);
_4.2 = 61_i8 as u64;
_11 = core::ptr::addr_of_mut!(_19);
_23.fld0 = _4;
_22.fld5 = _18;
Goto(bb12)
}
bb12 = {
_22.fld4 = _1.fld2;
_23.fld3.0 = _22.fld5;
_4.2 = _5;
_4.0 = _3.0;
_23.fld1 = _12;
_1.fld0.2 = _3.2 >> _16;
Call(_22.fld4.0 = core::intrinsics::transmute(_20), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_23.fld2.0 = _6 - _13;
_19 = !245727208265924890616353930380860840408_u128;
_23 = Adt42 { fld0: _1.fld0,fld1: _9,fld2: _1.fld2,fld3: _1.fld3 };
_3.2 = !_23.fld0.2;
_27 = _23.fld2.0 & _20;
_1.fld2 = (_23.fld2.0,);
RET = _23.fld0.0 - _1.fld0.0;
_23.fld3.0 = [_8,_8,_1.fld0.1,_3.1,_23.fld0.1];
_3 = (_4.0, _8, _23.fld0.2);
_1.fld2 = (_20,);
_23.fld0.2 = !_3.2;
_10 = _14;
_25 = [_1.fld0.2,_1.fld0.2,_16,_1.fld0.2];
_1.fld0 = (_4.0, _8, _3.2);
RET = _14 as i16;
_23 = Adt42 { fld0: _4,fld1: _1.fld1,fld2: _1.fld2,fld3: _1.fld3 };
RET = _23.fld0.0 ^ _1.fld0.0;
_1.fld0.1 = -_8;
_1.fld3 = (_18,);
_1.fld0 = (RET, _8, _3.2);
_4 = (_1.fld0.0, _23.fld0.1, _3.2);
_5 = _4.2;
_28 = _15;
Goto(bb14)
}
bb14 = {
_28 = _15;
_5 = !_16;
_11 = core::ptr::addr_of_mut!((*_11));
_22.fld0 = [67_u8,96_u8,151_u8,82_u8,106_u8,24_u8,38_u8];
RET = 100_i8 as i16;
_4 = (_23.fld0.0, _3.1, _1.fld0.2);
_26 = _10;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(11_usize, 5_usize, Move(_5), 18_usize, Move(_18), 17_usize, Move(_17), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(11_usize, 15_usize, Move(_15), 2_usize, Move(_2), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(11_usize, 6_usize, Move(_6), 25_usize, Move(_25), 13_usize, Move(_13), 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: i128,mut _2: i128,mut _3: i128,mut _4: (i16, i128, u64),mut _5: (i16, i128, u64),mut _6: u32,mut _7: u16,mut _8: (i16, i128, u64),mut _9: (i16, i128, u64),mut _10: i128,mut _11: i128) -> ([i128; 5],) {
mir! {
type RET = ([i128; 5],);
let _12: (isize,);
let _13: isize;
let _14: f32;
let _15: isize;
let _16: Adt47;
let _17: ();
let _18: ();
{
_4 = _8;
_12.0 = (-74_isize);
_3 = -_10;
_13 = _12.0;
_7 = 43792_u16 & 37073_u16;
_7 = 28166_u16;
_4.0 = _8.0;
_12.0 = !_13;
_2 = _8.1 | _1;
_1 = _8.1;
_14 = 63_u8 as f32;
_5.2 = _4.2;
_8.0 = _5.0;
_14 = _6 as f32;
_11 = true as i128;
_1 = _9.1 * _9.1;
_11 = 8816356899870222445_i64 as i128;
_9 = _8;
RET.0 = [_9.1,_10,_1,_8.1,_2];
_9.2 = _5.2 & _4.2;
_11 = _2 + _4.1;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(12_usize, 9_usize, Move(_9), 2_usize, Move(_2), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(12_usize, 13_usize, Move(_13), 3_usize, Move(_3), 18_usize, _18, 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: u64,mut _2: ([i128; 5],),mut _3: Adt42,mut _4: (i16, i128, u64),mut _5: i128,mut _6: i128,mut _7: (i16, i128, u64),mut _8: i128,mut _9: i128,mut _10: i128,mut _11: u64,mut _12: i128,mut _13: u64) -> *mut u128 {
mir! {
type RET = *mut u128;
let _14: (i16, i128, u64);
let _15: usize;
let _16: u64;
let _17: isize;
let _18: &'static i64;
let _19: Adt53;
let _20: i16;
let _21: (i16, i128, u64);
let _22: f32;
let _23: Adt40;
let _24: usize;
let _25: Adt50;
let _26: (isize,);
let _27: [u8; 4];
let _28: u32;
let _29: i32;
let _30: ([i128; 5],);
let _31: Adt48;
let _32: [bool; 6];
let _33: Adt56;
let _34: ([i128; 5],);
let _35: f32;
let _36: Adt42;
let _37: i64;
let _38: [bool; 6];
let _39: f64;
let _40: [u8; 7];
let _41: f32;
let _42: [i128; 1];
let _43: i16;
let _44: [i128; 5];
let _45: char;
let _46: f64;
let _47: [u8; 4];
let _48: isize;
let _49: Adt56;
let _50: ();
let _51: ();
{
_4.1 = -_5;
_9 = -_12;
_12 = _6 * _8;
_6 = _9;
_4 = (_3.fld0.0, _9, _1);
_7.2 = !_13;
_2.0 = [_5,_12,_3.fld0.1,_10,_3.fld0.1];
_7.0 = 131579620_i32 as i16;
_2 = _3.fld3;
_7.2 = _11;
_3.fld0.2 = 0_usize as u64;
_13 = _3.fld2.0 as u64;
_2.0 = [_8,_8,_9,_10,_10];
_3.fld1 = !1740_u16;
_3.fld2.0 = -9223372036854775807_isize;
_7.2 = 92_u8 as u64;
_3.fld0.1 = _12;
_16 = _11;
_4 = (_7.0, _3.fld0.1, _16);
_3.fld0.2 = _16;
Goto(bb1)
}
bb1 = {
_17 = !_3.fld2.0;
_3.fld0.2 = _4.2 & _11;
_3.fld0.1 = 7646062970975337935_usize as i128;
_4.2 = _1 ^ _11;
_3.fld0.1 = !_12;
_14.0 = _3.fld0.0 + _3.fld0.0;
_7 = _3.fld0;
_7.0 = _3.fld0.0 & _14.0;
_14 = (_7.0, _12, _11);
_5 = !_14.1;
_15 = 4_usize;
_7.1 = _2.0[_15] >> _11;
_4.2 = _16 - _7.2;
_16 = 65_i8 as u64;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
5 => bb7,
6 => bb8,
4 => bb10,
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
_3.fld0 = (_14.0, _3.fld3.0[_15], _7.2);
_19.fld2[_15] = 1552203217_i32 as u8;
_5 = _9 & _3.fld0.1;
_12 = (-1_i8) as i128;
_19.fld6.fld5 = [_14.1,_10,_3.fld3.0[_15],_9,_9];
_19.fld2 = [58_u8,168_u8,164_u8,202_u8,198_u8,120_u8,70_u8];
_3.fld3 = (_2.0,);
_7 = _14;
_15 = !11161047740661385303_usize;
_10 = _14.1 * _5;
_19.fld6.fld6 = 110_u8 as f64;
_8 = _7.0 as i128;
_4.2 = _3.fld0.0 as u64;
_3.fld2 = (_17,);
Goto(bb11)
}
bb11 = {
_4.1 = _3.fld0.1;
_10 = _1 as i128;
_3.fld1 = 48635_u16 & 5998_u16;
Call(_11 = core::intrinsics::transmute(_14.2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14.1 = _3.fld0.0 as i128;
_20 = _14.0;
_14.0 = -_20;
_3.fld0.2 = _15 as u64;
_3.fld2 = (_17,);
_19.fld6.fld4.0 = !_3.fld2.0;
_3.fld0.1 = _4.1;
_19.fld6.fld6 = (-66_i8) as f64;
_28 = 537722159_u32 ^ 3645230626_u32;
Call(_22 = fn14(_7.1, Move(_3), _2, _4, _8, _4, _10, _19.fld6.fld5, _4, _2, _4, _10, _5, _2, _14), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3 = Adt42 { fld0: _4,fld1: 40179_u16,fld2: _19.fld6.fld4,fld3: _2 };
_19.fld6.fld5 = _2.0;
_26.0 = -_19.fld6.fld4.0;
_7 = _3.fld0;
_14 = (_20, _3.fld0.1, _1);
_14.0 = _20;
_12 = !_6;
_19.fld6.fld0 = [50_u8,154_u8,144_u8,119_u8,225_u8,226_u8,130_u8];
_19.fld1 = '\u{6f725}';
_27 = [38_u8,185_u8,107_u8,220_u8];
_21.2 = _14.2;
match _3.fld1 {
0 => bb14,
40179 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_30 = _2;
_27 = [4_u8,127_u8,103_u8,144_u8];
_31.fld1 = Adt45::Variant0 { fld0: 17870680214803135685758296568635215579_u128 };
_27 = [129_u8,112_u8,170_u8,66_u8];
_2 = (_30.0,);
_2.0 = _3.fld3.0;
_21.1 = _6 >> _3.fld0.1;
_19.fld2 = [182_u8,232_u8,17_u8,49_u8,111_u8,124_u8,215_u8];
_21.0 = _14.0;
_33.fld2 = _28 as u16;
_2.0 = _30.0;
Goto(bb17)
}
bb17 = {
_20 = _21.0 + _7.0;
_31.fld0 = [195_u8,189_u8,78_u8,208_u8,103_u8,15_u8,138_u8];
_14 = (_20, _5, _11);
place!(Field::<u128>(Variant(_31.fld1, 0), 0)) = 328089666032240922111257305188835230680_u128;
match _3.fld1 {
0 => bb18,
40179 => bb20,
_ => bb19
}
}
bb18 = {
_14.1 = _3.fld0.0 as i128;
_20 = _14.0;
_14.0 = -_20;
_3.fld0.2 = _15 as u64;
_3.fld2 = (_17,);
_19.fld6.fld4.0 = !_3.fld2.0;
_3.fld0.1 = _4.1;
_19.fld6.fld6 = (-66_i8) as f64;
_28 = 537722159_u32 ^ 3645230626_u32;
Call(_22 = fn14(_7.1, Move(_3), _2, _4, _8, _4, _10, _19.fld6.fld5, _4, _2, _4, _10, _5, _2, _14), ReturnTo(bb13), UnwindUnreachable())
}
bb19 = {
_3 = Adt42 { fld0: _4,fld1: 40179_u16,fld2: _19.fld6.fld4,fld3: _2 };
_19.fld6.fld5 = _2.0;
_26.0 = -_19.fld6.fld4.0;
_7 = _3.fld0;
_14 = (_20, _3.fld0.1, _1);
_14.0 = _20;
_12 = !_6;
_19.fld6.fld0 = [50_u8,154_u8,144_u8,119_u8,225_u8,226_u8,130_u8];
_19.fld1 = '\u{6f725}';
_27 = [38_u8,185_u8,107_u8,220_u8];
_21.2 = _14.2;
match _3.fld1 {
0 => bb14,
40179 => bb16,
_ => bb15
}
}
bb20 = {
_33.fld4.fld2 = (_19.fld6.fld4.0,);
_33.fld4.fld0.1 = _21.1;
_36.fld3.0 = [_14.1,_12,_5,_14.1,_5];
_14 = (_3.fld0.0, _33.fld4.fld0.1, _21.2);
Call(_3.fld0.0 = core::intrinsics::transmute(_3.fld1), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_3.fld1 = true as u16;
Goto(bb22)
}
bb22 = {
_27 = [161_u8,6_u8,118_u8,224_u8];
_3.fld3.0 = [_21.1,_12,_5,_21.1,_5];
SetDiscriminant(_31.fld1, 1);
_35 = _22;
_36.fld3.0 = _3.fld3.0;
_22 = _35 * _35;
_3.fld0 = _7;
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld2 = _3.fld2;
_34 = (_19.fld6.fld5,);
_39 = -_19.fld6.fld6;
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld2 = (_17,);
_36 = Adt42 { fld0: _3.fld0,fld1: _33.fld2,fld2: _26,fld3: _30 };
place!(Field::<(isize,)>(Variant(_31.fld1, 1), 5)) = _33.fld4.fld2;
_7 = (_4.0, _33.fld4.fld0.1, _11);
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld2.0 = _33.fld2 as isize;
_31.fld3 = [29_u8,48_u8,77_u8,167_u8];
_21.0 = _4.0 * _20;
place!(Field::<[i128; 5]>(Variant(_31.fld1, 1), 4)) = [_10,_5,_5,_33.fld4.fld0.1,_21.1];
_33.fld4 = Move(_3);
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld3 = _33.fld4.fld3;
_3.fld0.1 = _19.fld1 as i128;
_37 = 3330904074864028194_i64 << _4.1;
_19.fld2 = [19_u8,204_u8,22_u8,48_u8,182_u8,31_u8,210_u8];
_28 = (-116_i8) as u32;
Goto(bb23)
}
bb23 = {
_36.fld0.0 = _33.fld4.fld0.0;
_38 = [true,false,true,false,false,false];
_24 = _15 >> _37;
_31.fld4.0 = _19.fld6.fld4.0;
_11 = _7.2;
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld0.1 = _14.1;
_24 = _37 as usize;
_33.fld4.fld2.0 = _26.0 * _19.fld6.fld4.0;
_33.fld0.0 = _37 as isize;
_42 = [_21.1];
_43 = _20;
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld2 = _33.fld0;
_3.fld2 = _33.fld0;
place!(Field::<(i64, *mut u128, u64)>(Variant(_31.fld1, 1), 1)).2 = !_1;
_31.fld5 = [_36.fld0.1,_6,Field::<Adt42>(Variant(_31.fld1, 1), 2).fld0.1,Field::<Adt42>(Variant(_31.fld1, 1), 2).fld0.1,_21.1];
_33.fld3 = [Field::<Adt42>(Variant(_31.fld1, 1), 2).fld2.0];
_33.fld4.fld0.1 = _19.fld6.fld4.0 as i128;
_7.0 = _14.0 - _20;
_19.fld6.fld5 = _34.0;
_19.fld6.fld3 = [78_u8,59_u8,252_u8,17_u8];
_7.2 = (-32_i8) as u64;
_37 = !(-4138993642266021783_i64);
Call(_1 = fn15(_7.1, _33.fld0, _36.fld3.0, _5, _21.1, _24, _34.0, _14.1, _2.0, _3.fld2.0), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_29 = _14.1 as i32;
_19.fld4 = Adt45::Variant0 { fld0: 157031090828647069087177714315839551274_u128 };
_33.fld4.fld0.1 = !_21.1;
_4.2 = _24 as u64;
_4.0 = _14.1 as i16;
_33.fld4.fld2.0 = _33.fld0.0;
_46 = _19.fld6.fld6 + _39;
_38 = [false,true,false,false,false,false];
_49.fld4.fld1 = _33.fld2 ^ _36.fld1;
_49.fld0.0 = _4.1 as isize;
_31.fld6 = _46 * _46;
_21.1 = _33.fld4.fld0.1;
_49.fld4.fld0.1 = Field::<Adt42>(Variant(_31.fld1, 1), 2).fld0.1 | _8;
_33.fld1 = _19.fld1;
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld0.0 = _4.0;
_3 = Adt42 { fld0: _21,fld1: _49.fld4.fld1,fld2: _33.fld0,fld3: _36.fld3 };
_44 = _31.fld5;
Goto(bb25)
}
bb25 = {
_45 = _33.fld1;
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld0.2 = _37 as u64;
_44 = [_7.1,_4.1,_9,_36.fld0.1,_6];
place!(Field::<Adt42>(Variant(_31.fld1, 1), 2)).fld1 = _3.fld1 + _33.fld4.fld1;
_49.fld4.fld2.0 = _49.fld0.0 | _3.fld2.0;
Goto(bb26)
}
bb26 = {
_33.fld4.fld0.1 = _49.fld4.fld0.1 ^ _14.1;
_31.fld0 = [103_u8,65_u8,20_u8,151_u8,172_u8,155_u8,91_u8];
_33.fld4.fld0.1 = _6 & _14.1;
_49.fld4.fld3 = (_36.fld3.0,);
Goto(bb27)
}
bb27 = {
place!(Field::<[i8; 5]>(Variant(_31.fld1, 1), 0)) = [(-81_i8),9_i8,(-109_i8),92_i8,30_i8];
_40 = [239_u8,64_u8,125_u8,134_u8,133_u8,136_u8,84_u8];
RET = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_19.fld4, 0), 0)));
_49 = Move(_33);
_46 = -_31.fld6;
_3.fld0.2 = _24 as u64;
_46 = _31.fld6;
Goto(bb28)
}
bb28 = {
Call(_50 = dump_var(13_usize, 43_usize, Move(_43), 7_usize, Move(_7), 28_usize, Move(_28), 14_usize, Move(_14)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_50 = dump_var(13_usize, 30_usize, Move(_30), 16_usize, Move(_16), 42_usize, Move(_42), 37_usize, Move(_37)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_50 = dump_var(13_usize, 12_usize, Move(_12), 26_usize, Move(_26), 29_usize, Move(_29), 6_usize, Move(_6)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_50 = dump_var(13_usize, 9_usize, Move(_9), 5_usize, Move(_5), 8_usize, Move(_8), 20_usize, Move(_20)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i128,mut _2: Adt42,mut _3: ([i128; 5],),mut _4: (i16, i128, u64),mut _5: i128,mut _6: (i16, i128, u64),mut _7: i128,mut _8: [i128; 5],mut _9: (i16, i128, u64),mut _10: ([i128; 5],),mut _11: (i16, i128, u64),mut _12: i128,mut _13: i128,mut _14: ([i128; 5],),mut _15: (i16, i128, u64)) -> f32 {
mir! {
type RET = f32;
let _16: i8;
let _17: u64;
let _18: (isize,);
let _19: ([i128; 1], [i128; 1]);
let _20: ([i128; 1], [i128; 1]);
let _21: ();
let _22: ();
{
_14.0 = _8;
_9.2 = !_15.2;
_4.2 = _15.2 - _9.2;
_15.0 = _4.0 & _2.fld0.0;
_12 = _11.1 ^ _13;
_9.1 = _6.1 * _6.1;
_9.0 = _4.0 ^ _15.0;
_15 = (_9.0, _1, _4.2);
_4 = (_9.0, _13, _15.2);
_16 = 100_i8 | (-28_i8);
_15.2 = !_9.2;
_10.0 = [_4.1,_11.1,_12,_12,_11.1];
_2.fld0.0 = (-2027845371_i32) as i16;
_14.0 = [_2.fld0.1,_4.1,_13,_4.1,_9.1];
_2.fld0.0 = _4.0;
_3.0 = _14.0;
_15.0 = _6.0;
_10.0 = [_6.1,_1,_7,_7,_1];
Goto(bb1)
}
bb1 = {
_5 = 6640741838522494626874073105530181648_u128 as i128;
_3 = _10;
_14.0 = [_13,_11.1,_9.1,_13,_15.1];
_11.0 = true as i16;
_9 = (_2.fld0.0, _7, _4.2);
_16 = 103_i8;
_2.fld0 = (_9.0, _11.1, _9.2);
_1 = _6.1;
_10 = (_8,);
_9 = _11;
_19.0 = [_12];
_18.0 = !_2.fld2.0;
RET = _2.fld0.2 as f32;
_15.2 = !_2.fld0.2;
_2.fld3 = (_8,);
_20 = (_19.0, _19.0);
_14.0 = [_9.1,_9.1,_9.1,_4.1,_13];
_2.fld0 = _15;
_9.2 = !_15.2;
Goto(bb2)
}
bb2 = {
Call(_21 = dump_var(14_usize, 5_usize, Move(_5), 10_usize, Move(_10), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_21 = dump_var(14_usize, 14_usize, Move(_14), 8_usize, Move(_8), 4_usize, Move(_4), 11_usize, Move(_11)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i128,mut _2: (isize,),mut _3: [i128; 5],mut _4: i128,mut _5: i128,mut _6: usize,mut _7: [i128; 5],mut _8: i128,mut _9: [i128; 5],mut _10: isize) -> u64 {
mir! {
type RET = u64;
let _11: isize;
let _12: (i64, *mut u128, u64);
let _13: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize);
let _14: f64;
let _15: isize;
let _16: char;
let _17: i32;
let _18: i128;
let _19: isize;
let _20: u128;
let _21: *mut *mut bool;
let _22: bool;
let _23: ([i128; 5],);
let _24: u16;
let _25: Adt56;
let _26: Adt50;
let _27: i64;
let _28: (i64, *mut u128, u64);
let _29: i32;
let _30: bool;
let _31: bool;
let _32: isize;
let _33: i128;
let _34: Adt52;
let _35: ();
let _36: ();
{
_2 = (_10,);
RET = 195397933781613907466581437235974644414_u128 as u64;
_8 = 170_u8 as i128;
_4 = _1 & _1;
_6 = 1_usize;
_3 = _9;
_11 = _2.0;
_3[_6] = _4 + _1;
_11 = _10;
_7 = [_4,_4,_1,_5,_1];
_1 = -_4;
_2.0 = _11;
_5 = _4;
_3[_6] = _1;
_5 = !_7[_6];
RET = 8971_i16 as u64;
Call(_9[_6] = core::intrinsics::bswap(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = [_5,_7[_6],_7[_6],_7[_6],_3[_6]];
_5 = _4;
_9[_6] = _10 as i128;
_4 = !_1;
_2.0 = _6 as isize;
_2 = (_10,);
_4 = _5 + _9[_6];
RET = _6 as u64;
_6 = !0_usize;
_12.2 = RET;
_12.0 = -(-6402103338616088033_i64);
_1 = _5 * _4;
_3 = [_1,_1,_1,_4,_4];
_5 = 14246_u16 as i128;
_6 = !15517525283020756525_usize;
_1 = -_5;
_3 = [_4,_4,_4,_4,_4];
_2 = (_11,);
_2.0 = _11 + _11;
_9 = [_4,_4,_4,_4,_4];
_8 = 59012_u16 as i128;
_5 = _4;
_10 = _2.0 + _2.0;
_10 = _12.0 as isize;
_12.0 = !(-2911641658447147915_i64);
Goto(bb2)
}
bb2 = {
_11 = !_2.0;
_13.6 = _11 ^ _2.0;
_13.3.0 = !(-23856_i16);
_13.5 = (_9,);
_13.5.0 = _3;
_13.1 = [true,true,false,false,false,true];
_17 = 491111425_i32;
_12.2 = RET;
_1 = -_5;
_5 = _1;
_2.0 = _6 as isize;
_13.0 = [2971575766_u32,4003748832_u32];
_2 = (_11,);
_15 = 22632_u16 as isize;
match _17 {
491111425 => bb3,
_ => bb1
}
}
bb3 = {
_12.0 = 2391301158737935276_i64 * (-5928769375519075859_i64);
_13.4 = [false,true,true,true,false,true];
_13.2 = _17 as f64;
_13.3.1 = 165869793881444572634348379577377416034_u128 as i128;
_4 = !_1;
_12.2 = (-33_i8) as u64;
_13.2 = RET as f64;
_16 = '\u{52b8e}';
_17 = !(-224400311_i32);
_7 = _13.5.0;
_13.0 = [424214217_u32,1509441013_u32];
_13.2 = 236200038595430252125006893978071919824_u128 as f64;
_13.5.0 = [_4,_5,_1,_5,_4];
Goto(bb4)
}
bb4 = {
_7 = [_5,_5,_4,_5,_5];
Call(_19 = core::intrinsics::bswap(_2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13.3.0 = _6 as i16;
_13.5 = (_7,);
_2 = (_11,);
_13.3 = (27869_i16, _1, RET);
_7 = [_13.3.1,_13.3.1,_5,_13.3.1,_13.3.1];
_6 = 17994470807017481460_usize >> _5;
_12.2 = RET;
_12.2 = _13.3.2 ^ _13.3.2;
_22 = !true;
_4 = _16 as i128;
_2 = (_11,);
RET = _13.6 as u64;
_13.5.0 = [_1,_13.3.1,_1,_1,_1];
RET = _13.2 as u64;
_10 = 30703_u16 as isize;
_25.fld4.fld0.0 = _13.3.0;
_2 = (_13.6,);
_25.fld0.0 = 36_u8 as isize;
_25.fld0 = (_11,);
match _25.fld4.fld0.0 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
27869 => bb12,
_ => bb11
}
}
bb6 = {
_7 = [_5,_5,_4,_5,_5];
Call(_19 = core::intrinsics::bswap(_2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_12.0 = 2391301158737935276_i64 * (-5928769375519075859_i64);
_13.4 = [false,true,true,true,false,true];
_13.2 = _17 as f64;
_13.3.1 = 165869793881444572634348379577377416034_u128 as i128;
_4 = !_1;
_12.2 = (-33_i8) as u64;
_13.2 = RET as f64;
_16 = '\u{52b8e}';
_17 = !(-224400311_i32);
_7 = _13.5.0;
_13.0 = [424214217_u32,1509441013_u32];
_13.2 = 236200038595430252125006893978071919824_u128 as f64;
_13.5.0 = [_4,_5,_1,_5,_4];
Goto(bb4)
}
bb8 = {
_11 = !_2.0;
_13.6 = _11 ^ _2.0;
_13.3.0 = !(-23856_i16);
_13.5 = (_9,);
_13.5.0 = _3;
_13.1 = [true,true,false,false,false,true];
_17 = 491111425_i32;
_12.2 = RET;
_1 = -_5;
_5 = _1;
_2.0 = _6 as isize;
_13.0 = [2971575766_u32,4003748832_u32];
_2 = (_11,);
_15 = 22632_u16 as isize;
match _17 {
491111425 => bb3,
_ => bb1
}
}
bb9 = {
_9 = [_5,_7[_6],_7[_6],_7[_6],_3[_6]];
_5 = _4;
_9[_6] = _10 as i128;
_4 = !_1;
_2.0 = _6 as isize;
_2 = (_10,);
_4 = _5 + _9[_6];
RET = _6 as u64;
_6 = !0_usize;
_12.2 = RET;
_12.0 = -(-6402103338616088033_i64);
_1 = _5 * _4;
_3 = [_1,_1,_1,_4,_4];
_5 = 14246_u16 as i128;
_6 = !15517525283020756525_usize;
_1 = -_5;
_3 = [_4,_4,_4,_4,_4];
_2 = (_11,);
_2.0 = _11 + _11;
_9 = [_4,_4,_4,_4,_4];
_8 = 59012_u16 as i128;
_5 = _4;
_10 = _2.0 + _2.0;
_10 = _12.0 as isize;
_12.0 = !(-2911641658447147915_i64);
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_25.fld2 = 20340_u16;
_23 = (_7,);
_25.fld3 = [_2.0];
_25.fld4 = Adt42 { fld0: _13.3,fld1: _25.fld2,fld2: _2,fld3: _13.5 };
_25.fld2 = _25.fld4.fld1;
_13.3.2 = RET * _12.2;
_14 = 40693744468285525038014548164131550372_u128 as f64;
_13.3.0 = !_25.fld4.fld0.0;
_23.0 = [_25.fld4.fld0.1,_13.3.1,_5,_1,_13.3.1];
_13.3.2 = _12.0 as u64;
_25.fld3 = [_25.fld0.0];
Goto(bb13)
}
bb13 = {
_25.fld4.fld2.0 = !_11;
_25.fld4.fld0.0 = !_13.3.0;
_2.0 = _22 as isize;
_12.1 = core::ptr::addr_of_mut!(_20);
_8 = _5;
_20 = 114696260178166589059990519278483797368_u128 & 152584141986957800470229956843990262839_u128;
_27 = -_12.0;
_17 = (-1410397118_i32) << _6;
_13.2 = _14;
_25.fld0.0 = 515701531_u32 as isize;
_29 = !_17;
_26 = Adt50::Variant0 { fld0: _13.2 };
_25.fld4.fld0 = (_13.3.0, _8, RET);
_25.fld4.fld2 = (_11,);
_25.fld2 = !_25.fld4.fld1;
_12.0 = !_27;
_25.fld4.fld0.2 = !_12.2;
_13.3.1 = _25.fld4.fld0.1 - _25.fld4.fld0.1;
_8 = _25.fld2 as i128;
_3 = [_25.fld4.fld0.1,_5,_13.3.1,_5,_25.fld4.fld0.1];
Call(_5 = fn16(_13, _25.fld3, _25.fld3, _25.fld4.fld0, _25.fld4.fld0.0, _25.fld4.fld3.0, _7, _25.fld3, _13, _23, _9, _25.fld4.fld0.1, Move(_25.fld4), _13.5), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_25.fld4.fld0 = _13.3;
_24 = _25.fld2;
_12.0 = _27 & _27;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(15_usize, 20_usize, Move(_20), 15_usize, Move(_15), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(15_usize, 3_usize, Move(_3), 16_usize, Move(_16), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(15_usize, 17_usize, Move(_17), 27_usize, Move(_27), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize),mut _2: [isize; 1],mut _3: [isize; 1],mut _4: (i16, i128, u64),mut _5: i16,mut _6: [i128; 5],mut _7: [i128; 5],mut _8: [isize; 1],mut _9: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize),mut _10: ([i128; 5],),mut _11: [i128; 5],mut _12: i128,mut _13: Adt42,mut _14: ([i128; 5],)) -> i128 {
mir! {
type RET = i128;
let _15: isize;
let _16: Adt43;
let _17: u32;
let _18: u32;
let _19: ();
let _20: ();
{
_9.3.1 = 6_usize as i128;
_14 = _13.fld3;
_9.3.2 = !_4.2;
_4 = (_9.3.0, _12, _13.fld0.2);
_12 = _4.1;
_14 = _9.5;
_9.2 = _9.3.2 as f64;
_12 = _13.fld2.0 as i128;
_1.3.0 = 119_i8 as i16;
_12 = true as i128;
_13.fld1 = 19412_u16 | 44753_u16;
_13.fld2.0 = (-1860635623_i32) as isize;
_14.0 = [_1.3.1,_13.fld0.1,_4.1,_13.fld0.1,_4.1];
_13.fld1 = 112_u8 as u16;
Goto(bb1)
}
bb1 = {
_3 = [_9.6];
RET = -_13.fld0.1;
_1.3.0 = _4.0 << _4.1;
_9.4 = [false,true,true,true,false,true];
_4.0 = _9.2 as i16;
_1.4 = _9.4;
_12 = !_4.1;
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(16_usize, 4_usize, Move(_4), 8_usize, Move(_8), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_19 = dump_var(16_usize, 11_usize, Move(_11), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: ([i128; 5],),mut _2: i32) -> isize {
mir! {
type RET = isize;
let _3: i16;
let _4: Adt55;
let _5: isize;
let _6: [i8; 5];
let _7: u8;
let _8: i8;
let _9: [isize; 1];
let _10: (isize,);
let _11: Adt45;
let _12: isize;
let _13: [bool; 6];
let _14: Adt52;
let _15: *mut u128;
let _16: Adt44;
let _17: (i64, *mut u128, u64);
let _18: [i128; 1];
let _19: isize;
let _20: ();
let _21: ();
{
RET = false as isize;
_1.0 = [13419474609880550313018055535846160817_i128,(-113172066849509800203179056881517227607_i128),89981903967500206492981927214172101977_i128,(-1112240304384969280435942533283072726_i128),(-24388511654920422518854292016490939730_i128)];
_1.0 = [(-145419473172216650548418458603705364512_i128),126892761357103338199026445525642491594_i128,56900611447282303857701448388130108229_i128,(-160871192705126207946619195129782657534_i128),165710224988232525509182236263638857533_i128];
_1.0 = [100534072434417381525457206959606983163_i128,(-61435943947786649208595118841692325727_i128),(-11966702262699716706394350018183618300_i128),(-26596734149702071741307519150331915635_i128),(-113574975160622021642469290887456802060_i128)];
_3 = _2 as i16;
RET = 99_isize >> _2;
_2 = !(-1448557731_i32);
_1.0 = [40390141653857906951963061565400768893_i128,(-26852187692161814554431281574059258229_i128),(-66051318248311449739417694820642149240_i128),47744877507984980819457460712611558448_i128,82149809883767271097587056894230739610_i128];
RET = 9223372036854775807_isize;
_3 = 14643_i16 << _2;
_3 = 20122_i16;
_2 = 3029848749607750323_i64 as i32;
_1.0 = [(-40115068000040771226910562226997401107_i128),27495132610713282461945643994851882206_i128,147237472709337330772687555368122168552_i128,53697316739619897708997518273313717185_i128,(-62013580771959351921392216854181118861_i128)];
_1.0 = [150814180236354498173649184067492781478_i128,(-151507565541441547598247947853740175421_i128),137331570410052899714318018220817618886_i128,(-67671999374738780107818555761878046202_i128),(-2263798450331590653105283861502574111_i128)];
RET = (-49_isize);
_2 = (-1339241482_i32);
_1.0 = [(-157002747190736671611079429605961778400_i128),(-30970804670888655672067156214523982423_i128),79275393413933000889511373873584023181_i128,(-70430420751446898583169663066540498471_i128),166265440586744213127785993296051324038_i128];
_1.0 = [151431603613298716465218720554689635282_i128,61107639884156175257476159359956500363_i128,(-109389378608456064840579410676510335642_i128),115923961531002846471710313772493956943_i128,(-30479274833651545920727985856840095896_i128)];
_3 = (-11450_i16);
_3 = 119_u8 as i16;
_5 = RET;
_2 = 461770890_i32;
RET = _5 ^ _5;
_2 = 309718171_i32 * (-546155972_i32);
_5 = 277651229824414001833518593103728575043_u128 as isize;
Goto(bb1)
}
bb1 = {
RET = _5 - _5;
RET = _5 + _5;
_1.0 = [112140303918875338654523016643815083520_i128,(-116992582167298165446609013049987894076_i128),134711087010731290183943286820072462379_i128,(-73393900435370308932926980225498852713_i128),(-168586852665070140745996198392920642920_i128)];
Goto(bb2)
}
bb2 = {
_3 = false as i16;
_7 = (-72625382478863005903628996264641645826_i128) as u8;
_1.0 = [67510965227497689510391243695682760441_i128,(-167672620717353021887990623800287340438_i128),(-25360683791404914733969582732199801559_i128),8132833047505960220719819793606514325_i128,(-69125834726074361684036896288984323403_i128)];
_7 = false as u8;
_7 = '\u{e6624}' as u8;
_3 = -(-20870_i16);
_3 = false as i16;
_3 = 62106_u16 as i16;
_10.0 = RET & RET;
_12 = _10.0 + _10.0;
Call(_5 = core::intrinsics::bswap(_10.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10.0 = !_12;
_5 = !_12;
_3 = _7 as i16;
_8 = -(-56_i8);
_8 = 78_i8 * (-92_i8);
RET = -_10.0;
_7 = _10.0 as u8;
Goto(bb4)
}
bb4 = {
_10.0 = _5 | _5;
_12 = _7 as isize;
_10 = (RET,);
RET = 138090914691926231754578101808889715401_u128 as isize;
Call(RET = fn18(_5, _1, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17.0 = (-170139886595776699157512941423528865414_i128) as i64;
_11 = Adt45::Variant0 { fld0: 52933453090123123216967333468523524521_u128 };
_10 = (RET,);
_17.1 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_11, 0), 0)));
_17.0 = !3251211228463236427_i64;
_17.2 = false as u64;
_13 = [false,true,false,true,false,true];
place!(Field::<u128>(Variant(_11, 0), 0)) = !273930720544126658466615710450585644569_u128;
_2 = (-1269890874_i32);
_15 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_11, 0), 0)));
Goto(bb6)
}
bb6 = {
Call(_20 = dump_var(17_usize, 10_usize, Move(_10), 2_usize, Move(_2), 8_usize, Move(_8), 12_usize, Move(_12)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: ([i128; 5],),mut _3: isize) -> isize {
mir! {
type RET = isize;
let _4: Adt54;
let _5: i128;
let _6: ();
let _7: ();
{
RET = -_3;
_3 = _1;
RET = !_3;
_1 = RET;
RET = 59_i8 as isize;
_3 = -_1;
_1 = _3 >> _3;
RET = _1 + _1;
_1 = 566969758855649823_u64 as isize;
_3 = !RET;
_1 = -RET;
RET = _1;
_1 = !RET;
_5 = -134650329016356731965003441312207002126_i128;
_5 = -(-30910678025143355024047243982722056210_i128);
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(18_usize, 1_usize, Move(_1), 2_usize, Move(_2), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(40580_u16), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-111_i8)), std::hint::black_box(222907118429431418451894257681146158864_u128), std::hint::black_box(698375955_i32), std::hint::black_box((-2275319634249139616_i64)), std::hint::black_box(90152401076897770078961153887669501329_i128), std::hint::black_box(3369880350782338690_usize), std::hint::black_box(3631427291_u32));
                
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt40 {
Variant0{
fld0: u8,
fld1: *mut u128,
fld2: isize,
fld3: [bool; 6],

},
Variant1{
fld0: bool,
fld1: *mut *mut bool,
fld2: i32,
fld3: f64,

},
Variant2{
fld0: bool,
fld1: *mut u128,
fld2: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize),
fld3: [u8; 7],

},
Variant3{
fld0: ([i128; 1], [i128; 1]),
fld1: (i64, *mut u128, u64),
fld2: [u32; 2],
fld3: i8,
fld4: f64,
fld5: f32,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt41 {
Variant0{
fld0: (isize,),

},
Variant1{
fld0: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize),

},
Variant2{
fld0: u128,
fld1: u32,
fld2: *mut [u64; 4],
fld3: usize,

},
Variant3{
fld0: (i16, i128, u64),

}}
#[derive(Debug)]
pub struct Adt42 {
fld0: (i16, i128, u64),
fld1: u16,
fld2: (isize,),
fld3: ([i128; 5],),
}
#[derive(Debug)]
pub enum Adt43 {
Variant0{
fld0: Adt42,
fld1: i128,
fld2: u128,

},
Variant1{
fld0: *mut u128,
fld1: char,
fld2: [isize; 1],
fld3: Adt40,
fld4: u8,
fld5: [u8; 4],
fld6: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize),
fld7: [i128; 1],

},
Variant2{
fld0: [isize; 1],
fld1: *mut *mut bool,

},
Variant3{
fld0: *mut *mut bool,
fld1: char,
fld2: (i16, i128, u64),
fld3: ([i128; 1], [i128; 1]),
fld4: *mut u128,
fld5: [u8; 7],
fld6: [i128; 5],
fld7: i128,

}}
#[derive(Debug)]
pub enum Adt44 {
Variant0{
fld0: u128,
fld1: u16,
fld2: *mut *mut bool,
fld3: ([u32; 2], [bool; 6], f64, (i16, i128, u64), [bool; 6], ([i128; 5],), isize),

},
Variant1{
fld0: f32,
fld1: u32,

}}
#[derive(Debug)]
pub enum Adt45 {
Variant0{
fld0: u128,

},
Variant1{
fld0: [i8; 5],
fld1: (i64, *mut u128, u64),
fld2: Adt42,
fld3: [i8; 1],
fld4: [i128; 5],
fld5: (isize,),

},
Variant2{
fld0: bool,
fld1: (i64, *mut u128, u64),
fld2: isize,
fld3: (isize,),
fld4: *mut [u64; 4],

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt46 {
fld0: i128,
fld1: [u32; 2],
fld2: [i8; 1],
fld3: *mut *mut bool,
fld4: *mut bool,
fld5: [isize; 1],
}
#[derive(Debug)]
pub enum Adt47 {
Variant0{
fld0: bool,
fld1: [u8; 4],
fld2: Adt42,

},
Variant1{
fld0: [i8; 5],

}}
#[derive(Debug)]
pub struct Adt48 {
fld0: [u8; 7],
fld1: Adt45,
fld2: *mut bool,
fld3: [u8; 4],
fld4: (isize,),
fld5: [i128; 5],
fld6: f64,
fld7: Adt43,
}
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: [i128; 1],
fld1: usize,
fld2: u64,

},
Variant1{
fld0: bool,
fld1: [u64; 4],
fld2: u16,
fld3: u64,
fld4: Adt45,
fld5: i32,

},
Variant2{
fld0: bool,
fld1: [u64; 4],
fld2: u32,
fld3: u128,
fld4: u8,
fld5: *mut *mut bool,
fld6: Adt48,
fld7: Adt47,

},
Variant3{
fld0: Adt45,
fld1: [u64; 4],
fld2: *mut u128,
fld3: Adt42,
fld4: *const *mut bool,
fld5: Adt40,
fld6: [i128; 5],
fld7: Adt46,

}}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: f64,

},
Variant1{
fld0: Adt44,
fld1: u32,
fld2: usize,
fld3: (isize,),
fld4: u64,
fld5: *mut [u64; 4],
fld6: i64,
fld7: Adt40,

},
Variant2{
fld0: [i8; 1],
fld1: u128,
fld2: isize,
fld3: Adt47,
fld4: [i128; 1],
fld5: i32,
fld6: *mut [u64; 4],

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: (u32,),
fld1: (i16, i128, u64),
fld2: f64,

},
Variant1{
fld0: [i128; 5],

},
Variant2{
fld0: Adt48,
fld1: Adt44,

},
Variant3{
fld0: isize,

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: Adt49,
fld1: [bool; 6],
fld2: ([i128; 1], [i128; 1]),
fld3: Adt46,
fld4: i16,

},
Variant1{
fld0: [isize; 1],

},
Variant2{
fld0: f32,
fld1: u64,
fld2: *mut [u64; 4],
fld3: (i64, *mut u128, u64),
fld4: Adt41,
fld5: Adt50,

}}
#[derive(Debug)]
pub struct Adt53 {
fld0: *mut bool,
fld1: char,
fld2: [u8; 7],
fld3: *mut [u64; 4],
fld4: Adt45,
fld5: Adt49,
fld6: Adt48,
}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: i128,
fld1: [i128; 1],
fld2: [i128; 5],

},
Variant1{
fld0: [i128; 5],
fld1: i64,
fld2: [i128; 1],
fld3: (i16, i128, u64),
fld4: (isize,),

},
Variant2{
fld0: Adt46,
fld1: (isize,),

},
Variant3{
fld0: bool,
fld1: u32,
fld2: [isize; 1],
fld3: [u8; 4],
fld4: i16,
fld5: Adt47,
fld6: ([i128; 1], [i128; 1]),
fld7: (i64, *mut u128, u64),

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: Adt42,
fld1: [bool; 6],
fld2: Adt52,
fld3: u64,
fld4: u8,
fld5: [i128; 5],
fld6: u16,

},
Variant1{
fld0: u128,
fld1: Adt52,
fld2: Adt48,
fld3: Adt47,
fld4: *mut *mut bool,
fld5: i32,
fld6: *mut [u64; 4],
fld7: u8,

},
Variant2{
fld0: *mut bool,
fld1: Adt48,

},
Variant3{
fld0: Adt50,
fld1: usize,
fld2: isize,
fld3: Adt41,
fld4: [i128; 1],
fld5: Adt52,
fld6: [u8; 7],
fld7: Adt48,

}}
#[derive(Debug)]
pub struct Adt56 {
fld0: (isize,),
fld1: char,
fld2: u16,
fld3: [isize; 1],
fld4: Adt42,
}

