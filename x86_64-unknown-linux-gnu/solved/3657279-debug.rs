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
pub fn fn0(mut _1: bool,mut _2: u32,mut _3: isize,mut _4: i8,mut _5: u16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8) -> Adt41 {
mir! {
type RET = Adt41;
let _11: &'static i64;
let _12: char;
let _13: u64;
let _14: &'static i64;
let _15: (u8, isize);
let _16: [i8; 5];
let _17: u128;
let _18: u128;
let _19: isize;
let _20: ([u128; 4], &'static i64, [isize; 3], (char, *mut *const u8, u8, *mut *const u8));
let _21: Adt50;
let _22: Adt41;
let _23: [bool; 3];
let _24: f64;
let _25: Adt51;
let _26: isize;
let _27: bool;
let _28: *const u8;
let _29: Adt46;
let _30: char;
let _31: Adt56;
let _32: u8;
let _33: *mut u128;
let _34: i32;
let _35: *const *mut i64;
let _36: isize;
let _37: u32;
let _38: u16;
let _39: [u8; 2];
let _40: isize;
let _41: f32;
let _42: (char, *mut *const u8, u8, *mut *const u8);
let _43: u8;
let _44: i16;
let _45: ();
let _46: ();
{
_7 = !(-4572441806862394425_i64);
_4 = 10_i8;
_9 = !9762827465726041592_usize;
_6 = _9 as i32;
_1 = !true;
_7 = !(-7744283734606973882_i64);
_12 = '\u{4a227}';
_15.0 = 61_u8;
_16 = [_4,_4,_4,_4,_4];
_3 = (-9223372036854775808_isize);
_10 = _15.0;
_9 = _10 as usize;
_11 = &_7;
_8 = (-108710810422591302361375988622446877439_i128) & (-115992734029292716381318344004129116447_i128);
_8 = 95773901857950801843906524258597068957_i128 | 53241856285745098524538776912051311602_i128;
_2 = !792819604_u32;
_18 = !105709212956136900315970738699189663569_u128;
_13 = 5284662617338847628_u64;
Call(_12 = fn1(_6, (*_11), _18, _7, Move(_11), _6, _10, _16, _18, _4, _4, _3, _3, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_17 = _18 + _18;
_9 = _3 as usize;
_5 = 26296_u16 >> _7;
_19 = _3;
_15.1 = _3;
_16 = [_4,_4,_4,_4,_4];
_2 = _10 as u32;
match _4 {
10 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_20.3.2 = _15.0;
_8 = _12 as i128;
_14 = &_7;
_20.3.0 = _12;
_10 = _5 as u8;
RET = Adt41::Variant0 { fld0: _15 };
_3 = _13 as isize;
_15.1 = _19;
_22 = RET;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).0 = _15.0;
_4 = 27_i8;
place!(Field::<(u8, isize)>(Variant(_22, 0), 0)).0 = _15.0;
_11 = Move(_14);
_8 = _5 as i128;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).0 = Field::<(u8, isize)>(Variant(_22, 0), 0).0 & _10;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = Field::<(u8, isize)>(Variant(_22, 0), 0).1 + _19;
_24 = _5 as f64;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = -Field::<(u8, isize)>(Variant(_22, 0), 0).1;
SetDiscriminant(_22, 1);
match _19 {
0 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb4 = {
Return()
}
bb5 = {
_17 = _18 + _18;
_9 = _3 as usize;
_5 = 26296_u16 >> _7;
_19 = _3;
_15.1 = _3;
_16 = [_4,_4,_4,_4,_4];
_2 = _10 as u32;
match _4 {
10 => bb3,
_ => bb2
}
}
bb6 = {
_23 = [_1,_1,_1];
_19 = _3 - Field::<(u8, isize)>(Variant(RET, 0), 0).1;
place!(Field::<char>(Variant(_22, 1), 1)) = _12;
_15.1 = !_19;
SetDiscriminant(RET, 0);
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).0 = _8 as u8;
_15.1 = _19 >> _5;
RET = Adt41::Variant1 { fld0: _9,fld1: Field::<char>(Variant(_22, 1), 1),fld2: _23 };
place!(Field::<char>(Variant(RET, 1), 1)) = Field::<char>(Variant(_22, 1), 1);
_23 = Field::<[bool; 3]>(Variant(RET, 1), 2);
_20.2 = [_15.1,_15.1,_15.1];
place!(Field::<char>(Variant(RET, 1), 1)) = Field::<char>(Variant(_22, 1), 1);
place!(Field::<char>(Variant(_22, 1), 1)) = Field::<char>(Variant(RET, 1), 1);
_1 = !true;
_3 = _15.1;
_5 = _2 as u16;
place!(Field::<usize>(Variant(RET, 1), 0)) = _9 | _9;
Goto(bb7)
}
bb7 = {
_23 = Field::<[bool; 3]>(Variant(RET, 1), 2);
place!(Field::<usize>(Variant(_22, 1), 0)) = _2 as usize;
RET = Adt41::Variant1 { fld0: _9,fld1: Field::<char>(Variant(_22, 1), 1),fld2: _23 };
place!(Field::<usize>(Variant(_22, 1), 0)) = Field::<usize>(Variant(RET, 1), 0);
_15.0 = _17 as u8;
place!(Field::<char>(Variant(_22, 1), 1)) = Field::<char>(Variant(RET, 1), 1);
SetDiscriminant(RET, 0);
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).0 = !_10;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = _3;
_22 = Adt41::Variant1 { fld0: _9,fld1: _12,fld2: _23 };
_19 = _3 * Field::<(u8, isize)>(Variant(RET, 0), 0).1;
_20.3.2 = !Field::<(u8, isize)>(Variant(RET, 0), 0).0;
SetDiscriminant(RET, 1);
RET = Adt41::Variant0 { fld0: _15 };
_15 = (_20.3.2, _19);
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = _6 as isize;
_20.0 = [_17,_18,_17,_18];
_28 = core::ptr::addr_of!(_15.0);
_20.3.1 = core::ptr::addr_of_mut!(_28);
_18 = _17 >> _3;
RET = Adt41::Variant1 { fld0: _9,fld1: _20.3.0,fld2: _23 };
_20.3.0 = Field::<char>(Variant(_22, 1), 1);
_20.3.3 = _20.3.1;
RET = Adt41::Variant1 { fld0: _9,fld1: Field::<char>(Variant(_22, 1), 1),fld2: Field::<[bool; 3]>(Variant(_22, 1), 2) };
_15.1 = -_3;
_25.fld0 = [_7,_7];
_27 = _1;
SetDiscriminant(RET, 1);
Goto(bb8)
}
bb8 = {
_1 = !_27;
RET = _22;
_17 = _18 & _18;
place!(Field::<[bool; 3]>(Variant(RET, 1), 2)) = [_1,_1,_27];
_26 = _15.1 + _15.1;
RET = Adt41::Variant0 { fld0: _15 };
_15 = (_20.3.2, _26);
_23 = Field::<[bool; 3]>(Variant(_22, 1), 2);
_15 = (Field::<(u8, isize)>(Variant(RET, 0), 0).0, Field::<(u8, isize)>(Variant(RET, 0), 0).1);
_4 = _13 as i8;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = Field::<(u8, isize)>(Variant(RET, 0), 0).0 as isize;
_15.0 = _4 as u8;
RET = Adt41::Variant0 { fld0: _15 };
SetDiscriminant(_22, 0);
place!(Field::<(u8, isize)>(Variant(_22, 0), 0)).1 = Field::<(u8, isize)>(Variant(RET, 0), 0).1 >> _26;
_1 = Field::<(u8, isize)>(Variant(_22, 0), 0).1 > _26;
_4 = (-37_i8) << _15.1;
place!(Field::<(u8, isize)>(Variant(_22, 0), 0)) = _15;
_10 = _7 as u8;
Goto(bb9)
}
bb9 = {
place!(Field::<(u8, isize)>(Variant(_22, 0), 0)).0 = (*_28);
_28 = core::ptr::addr_of!((*_28));
_17 = _18;
_20.2 = [_15.1,_3,_19];
_3 = Field::<(u8, isize)>(Variant(RET, 0), 0).1;
Goto(bb10)
}
bb10 = {
_4 = _7 as i8;
_14 = &_7;
_20.3.2 = _10;
_29 = Adt46::Variant2 { fld0: _13 };
match Field::<u64>(Variant(_29, 2), 0) {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
5284662617338847628 => bb12,
_ => bb11
}
}
bb11 = {
_23 = [_1,_1,_1];
_19 = _3 - Field::<(u8, isize)>(Variant(RET, 0), 0).1;
place!(Field::<char>(Variant(_22, 1), 1)) = _12;
_15.1 = !_19;
SetDiscriminant(RET, 0);
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).0 = _8 as u8;
_15.1 = _19 >> _5;
RET = Adt41::Variant1 { fld0: _9,fld1: Field::<char>(Variant(_22, 1), 1),fld2: _23 };
place!(Field::<char>(Variant(RET, 1), 1)) = Field::<char>(Variant(_22, 1), 1);
_23 = Field::<[bool; 3]>(Variant(RET, 1), 2);
_20.2 = [_15.1,_15.1,_15.1];
place!(Field::<char>(Variant(RET, 1), 1)) = Field::<char>(Variant(_22, 1), 1);
place!(Field::<char>(Variant(_22, 1), 1)) = Field::<char>(Variant(RET, 1), 1);
_1 = !true;
_3 = _15.1;
_5 = _2 as u16;
place!(Field::<usize>(Variant(RET, 1), 0)) = _9 | _9;
Goto(bb7)
}
bb12 = {
RET = Adt41::Variant0 { fld0: _15 };
_30 = _20.3.0;
_16 = [_4,_4,_4,_4,_4];
_20.1 = Move(_14);
_12 = _20.3.0;
_20.3.0 = _12;
_20.1 = &_7;
_27 = !_1;
SetDiscriminant(_29, 1);
_34 = _4 as i32;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)) = (_15.0, _26);
_9 = !1_usize;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = _19;
RET = Adt41::Variant1 { fld0: _9,fld1: _12,fld2: _23 };
_2 = 1244574244_u32 + 3215623817_u32;
SetDiscriminant(RET, 0);
_29 = Adt46::Variant2 { fld0: _13 };
place!(Field::<(u8, isize)>(Variant(_22, 0), 0)).0 = (*_28) - _15.0;
_39 = [_20.3.2,(*_28)];
match _13 {
5284662617338847628 => bb13,
_ => bb3
}
}
bb13 = {
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = !_19;
_19 = _13 as isize;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)) = (_20.3.2, _15.1);
_13 = _4 as u64;
_3 = -_15.1;
_20.3.3 = core::ptr::addr_of_mut!(_28);
SetDiscriminant(_22, 0);
_3 = !_26;
_40 = 3743_i16 as isize;
_33 = core::ptr::addr_of_mut!(_18);
_20.3.0 = _12;
_22 = Adt41::Variant0 { fld0: _15 };
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = _19 - _3;
_36 = _26;
_38 = _5;
_15.1 = !_26;
_15.1 = _36 << (*_33);
_1 = !_27;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).1 = !_15.1;
_8 = !67730820675437869554030910342810355890_i128;
_13 = !Field::<u64>(Variant(_29, 2), 0);
_12 = _20.3.0;
_37 = _2;
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)) = Field::<(u8, isize)>(Variant(_22, 0), 0);
_23 = [_27,_27,_1];
_1 = !_27;
_32 = _10 + _20.3.2;
_13 = (*_33) as u64;
SetDiscriminant(_29, 1);
_20.0 = [_17,_17,_17,_17];
Goto(bb14)
}
bb14 = {
_32 = _20.3.2 - (*_28);
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)).0 = !_32;
place!(Field::<(u8, isize)>(Variant(_22, 0), 0)).0 = !_32;
SetDiscriminant(RET, 0);
_12 = _20.3.0;
_1 = _27 | _27;
_16 = [_4,_4,_4,_4,_4];
place!(Field::<(u8, isize)>(Variant(RET, 0), 0)) = (_32, _26);
_42.0 = _30;
_42.0 = _20.3.0;
_23 = [_1,_1,_1];
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(0_usize, 34_usize, Move(_34), 36_usize, Move(_36), 3_usize, Move(_3), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(0_usize, 32_usize, Move(_32), 26_usize, Move(_26), 38_usize, Move(_38), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(0_usize, 7_usize, Move(_7), 13_usize, Move(_13), 19_usize, Move(_19), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(0_usize, 16_usize, Move(_16), 1_usize, Move(_1), 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i32,mut _2: i64,mut _3: u128,mut _4: i64,mut _5: &'static i64,mut _6: i32,mut _7: u8,mut _8: [i8; 5],mut _9: u128,mut _10: i8,mut _11: i8,mut _12: isize,mut _13: isize,mut _14: i128) -> char {
mir! {
type RET = char;
let _15: char;
let _16: (u32, i32, isize, u128);
let _17: Adt43;
let _18: Adt41;
let _19: u16;
let _20: f32;
let _21: (u32, i32, isize, u128);
let _22: *mut i64;
let _23: (u8, isize);
let _24: (u8, isize);
let _25: isize;
let _26: u64;
let _27: [u128; 4];
let _28: Adt53;
let _29: ([isize; 3],);
let _30: [i16; 1];
let _31: u64;
let _32: [i16; 1];
let _33: Adt42;
let _34: char;
let _35: Adt56;
let _36: Adt41;
let _37: isize;
let _38: [isize; 3];
let _39: [isize; 3];
let _40: isize;
let _41: [isize; 3];
let _42: f64;
let _43: Adt46;
let _44: ();
let _45: ();
{
_6 = _1;
RET = '\u{1a0f}';
_14 = -48395961719152609098806767779267475374_i128;
_7 = !178_u8;
_9 = _3 >> _2;
_2 = _4;
_7 = !85_u8;
_8 = [_11,_11,_10,_11,_10];
_1 = _7 as i32;
_12 = -_13;
_16.3 = _9;
_16.0 = 3986964560_u32;
_8 = [_11,_10,_11,_11,_11];
_16.2 = _13 << _10;
_11 = _10 & _10;
_5 = &_4;
_16 = (537094587_u32, _1, _12, _9);
_16.2 = _12;
Goto(bb1)
}
bb1 = {
_15 = RET;
_9 = !_16.3;
_14 = (-123851887249020692272206980650002875799_i128);
_16.0 = !345372939_u32;
_16.2 = -_13;
_2 = _15 as i64;
_11 = !_10;
RET = _15;
_13 = _16.2 - _16.2;
_17.fld4 = core::ptr::addr_of_mut!((*_5));
_6 = _16.1 >> _9;
_13 = _16.2 ^ _16.2;
_17.fld0 = [false,false,false];
_6 = _16.1;
_21 = (_16.0, _1, _12, _9);
_17.fld0 = [true,true,false];
_17.fld1 = [_21.2,_13,_16.2];
_16.0 = !_21.0;
_5 = &(*_5);
_9 = !_21.3;
_17.fld0 = [true,true,true];
_21 = (_16.0, _6, _13, _9);
Call(_7 = fn2(_4, (*_5), _17.fld1, _12, _4, _16.0, _16, _13, _21, _12, _16.0, _15, _17.fld1, _17.fld4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_22 = core::ptr::addr_of_mut!((*_5));
_16.3 = !_9;
_4 = _2 << _13;
_8 = [_11,_10,_11,_11,_10];
_8 = [_11,_11,_11,_10,_10];
_15 = RET;
_17.fld2 = _9;
_17.fld1 = [_21.2,_21.2,_12];
_17.fld3.0 = RET;
_19 = 53931_u16 ^ 31837_u16;
_16.0 = _21.0;
_9 = _19 as u128;
_23.0 = !_7;
_23 = (_7, _13);
_10 = 2_usize as i8;
_17.fld4 = core::ptr::addr_of_mut!(_4);
_24 = (_7, _12);
RET = _15;
_6 = _16.1;
match _14 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
216430479671917771191167626781765335657 => bb7,
_ => bb6
}
}
bb3 = {
_15 = RET;
_9 = !_16.3;
_14 = (-123851887249020692272206980650002875799_i128);
_16.0 = !345372939_u32;
_16.2 = -_13;
_2 = _15 as i64;
_11 = !_10;
RET = _15;
_13 = _16.2 - _16.2;
_17.fld4 = core::ptr::addr_of_mut!((*_5));
_6 = _16.1 >> _9;
_13 = _16.2 ^ _16.2;
_17.fld0 = [false,false,false];
_6 = _16.1;
_21 = (_16.0, _1, _12, _9);
_17.fld0 = [true,true,false];
_17.fld1 = [_21.2,_13,_16.2];
_16.0 = !_21.0;
_5 = &(*_5);
_9 = !_21.3;
_17.fld0 = [true,true,true];
_21 = (_16.0, _6, _13, _9);
Call(_7 = fn2(_4, (*_5), _17.fld1, _12, _4, _16.0, _16, _13, _21, _12, _16.0, _15, _17.fld1, _17.fld4), ReturnTo(bb2), UnwindUnreachable())
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
_16.2 = !_23.1;
_5 = &_2;
match _14 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
216430479671917771191167626781765335657 => bb8,
_ => bb5
}
}
bb8 = {
_21 = _16;
match _14 {
216430479671917771191167626781765335657 => bb9,
_ => bb1
}
}
bb9 = {
_5 = &_4;
_2 = _19 as i64;
_20 = _14 as f32;
RET = _15;
_17.fld3.0 = _15;
_19 = 44506_u16 ^ 60674_u16;
_5 = &_4;
_20 = _16.0 as f32;
_10 = _11;
_13 = 16112205143808062632_u64 as isize;
_14 = (-20790485773893489224369380983524363832_i128);
_21.0 = _16.0 * _16.0;
_16.2 = _20 as isize;
Goto(bb10)
}
bb10 = {
_24 = (_23.0, _21.2);
_17.fld1 = [_21.2,_13,_21.2];
_27 = [_17.fld2,_17.fld2,_3,_16.3];
_17.fld2 = true as u128;
_4 = _2 >> _24.0;
_22 = core::ptr::addr_of_mut!(_4);
_31 = !3250208485085058125_u64;
_26 = _31 ^ _31;
_20 = _6 as f32;
_21.0 = _16.0 & _16.0;
_21 = _16;
_12 = _20 as isize;
_19 = !30131_u16;
_30 = [9918_i16];
_16.1 = _6;
_25 = _24.1;
_17.fld3.2 = false as u8;
_5 = &(*_22);
match _14 {
319491881147044974239005226448243847624 => bb11,
_ => bb4
}
}
bb11 = {
_17.fld3.2 = !_24.0;
_23.1 = _24.1;
_16.3 = _17.fld2 & _21.3;
_21.1 = _19 as i32;
_17.fld2 = _3;
_13 = !_24.1;
Goto(bb12)
}
bb12 = {
_29.0 = _17.fld1;
RET = _15;
_16 = (_21.0, _1, _21.2, _9);
_21 = _16;
_21.2 = -_13;
_21 = (_16.0, _1, _16.2, _9);
_3 = 5_usize as u128;
_23.0 = _24.0;
_21.2 = -_13;
_17.fld2 = !_16.3;
_17.fld1 = _29.0;
_16.2 = _23.1;
_17.fld0 = [true,false,true];
_32 = [14080_i16];
_2 = _4 >> _21.0;
_2 = _16.0 as i64;
_8 = [_11,_11,_10,_10,_11];
RET = _15;
match _14 {
319491881147044974239005226448243847624 => bb13,
_ => bb11
}
}
bb13 = {
_11 = _10;
_27 = [_17.fld2,_9,_3,_9];
_23 = (_24.0, _25);
_26 = true as u64;
_17.fld0 = [true,false,true];
_17.fld3.2 = !_24.0;
RET = _17.fld3.0;
_17.fld3.2 = !_24.0;
_39 = _17.fld1;
_9 = _16.3;
_21.3 = _19 as u128;
_33 = Adt42::Variant1 { fld0: _31,fld1: _15,fld2: _22,fld3: _16.1 };
_29.0 = _39;
_23 = (_17.fld3.2, _25);
_3 = !_17.fld2;
_29.0 = [_16.2,_13,_23.1];
_24 = (_17.fld3.2, _21.2);
_13 = _16.0 as isize;
_30 = _32;
_10 = _11 << _21.3;
_3 = _21.0 as u128;
_16.1 = Field::<i32>(Variant(_33, 1), 3) - Field::<i32>(Variant(_33, 1), 3);
_11 = !_10;
_36 = Adt41::Variant1 { fld0: 12839084936192661070_usize,fld1: RET,fld2: _17.fld0 };
_25 = _21.2 * _23.1;
_41 = _39;
_14 = 158316716185288050654812642681170309923_i128 - 102266629209518729993073638282268642347_i128;
Goto(bb14)
}
bb14 = {
_2 = _16.2 as i64;
_37 = _21.0 as isize;
_41 = _29.0;
_34 = Field::<char>(Variant(_33, 1), 1);
_6 = _16.1;
_42 = _24.0 as f64;
SetDiscriminant(_33, 1);
_19 = Field::<char>(Variant(_36, 1), 1) as u16;
place!(Field::<i32>(Variant(_33, 1), 3)) = _16.1 - _21.1;
_39 = [_21.2,_24.1,_25];
_39 = [_12,_16.2,_23.1];
_38 = [_23.1,_25,_16.2];
_18 = Adt41::Variant1 { fld0: 6_usize,fld1: _15,fld2: _17.fld0 };
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(1_usize, 32_usize, Move(_32), 13_usize, Move(_13), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(1_usize, 34_usize, Move(_34), 6_usize, Move(_6), 15_usize, Move(_15), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(1_usize, 23_usize, Move(_23), 31_usize, Move(_31), 4_usize, Move(_4), 39_usize, Move(_39)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(1_usize, 30_usize, Move(_30), 7_usize, Move(_7), 41_usize, Move(_41), 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i64,mut _2: i64,mut _3: [isize; 3],mut _4: isize,mut _5: i64,mut _6: u32,mut _7: (u32, i32, isize, u128),mut _8: isize,mut _9: (u32, i32, isize, u128),mut _10: isize,mut _11: u32,mut _12: char,mut _13: [isize; 3],mut _14: *mut i64) -> u8 {
mir! {
type RET = u8;
let _15: [isize; 3];
let _16: Adt42;
let _17: isize;
let _18: [isize; 3];
let _19: Adt42;
let _20: *mut u128;
let _21: i64;
let _22: ([isize; 3],);
let _23: isize;
let _24: i8;
let _25: f32;
let _26: [i8; 5];
let _27: i64;
let _28: bool;
let _29: bool;
let _30: u8;
let _31: u32;
let _32: [i16; 1];
let _33: Adt41;
let _34: bool;
let _35: f32;
let _36: (u8, isize);
let _37: [i64; 2];
let _38: [i16; 2];
let _39: Adt53;
let _40: (char, *mut *const u8, u8, *mut *const u8);
let _41: (u8, isize);
let _42: bool;
let _43: i64;
let _44: isize;
let _45: Adt47;
let _46: u32;
let _47: ();
let _48: ();
{
_5 = !_1;
_12 = '\u{10e50a}';
_2 = _10 as i64;
_13 = _3;
_1 = _2;
_15 = [_9.2,_9.2,_10];
RET = 241_u8;
_6 = _11;
_13 = _15;
_7.0 = _9.0 * _6;
_9.0 = !_6;
_18 = [_9.2,_8,_8];
_10 = -_9.2;
_2 = _5 << _7.2;
_15 = _13;
_8 = true as isize;
_7.0 = 10844_i16 as u32;
_17 = -_4;
_3 = [_10,_4,_4];
_4 = _12 as isize;
_13 = [_4,_10,_7.2];
_4 = -_7.2;
_20 = core::ptr::addr_of_mut!(_7.3);
_18 = _3;
Goto(bb1)
}
bb1 = {
_21 = 13371474668550907411_usize as i64;
_19 = Adt42::Variant1 { fld0: 11216871307906165440_u64,fld1: _12,fld2: _14,fld3: _7.1 };
_9.0 = !_11;
place!(Field::<char>(Variant(_19, 1), 1)) = _12;
place!(Field::<*mut i64>(Variant(_19, 1), 2)) = _14;
_7.2 = -_10;
_16 = Adt42::Variant1 { fld0: 14025239166849466220_u64,fld1: _12,fld2: _14,fld3: _7.1 };
_6 = 50488_u16 as u32;
RET = 121_u8;
_22.0 = _15;
Call(_11 = fn3(_7.2, Field::<char>(Variant(_19, 1), 1), _9.0, _13, _10, _20, Field::<char>(Variant(_16, 1), 1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = Field::<char>(Variant(_19, 1), 1);
_21 = _2;
_9.0 = _6;
_12 = Field::<char>(Variant(_16, 1), 1);
_19 = Adt42::Variant1 { fld0: 6643186294239341758_u64,fld1: _12,fld2: _14,fld3: _9.1 };
_3 = [_10,_17,_10];
_9.1 = Field::<i32>(Variant(_16, 1), 3);
_22 = (_3,);
_7.0 = _6;
_9.2 = (-105_i8) as isize;
place!(Field::<i32>(Variant(_19, 1), 3)) = _6 as i32;
_1 = _5;
place!(Field::<u64>(Variant(_16, 1), 0)) = !8311042948987313468_u64;
Goto(bb3)
}
bb3 = {
_7.0 = 91659183583677081755526495243947576241_i128 as u32;
_12 = Field::<char>(Variant(_19, 1), 1);
_8 = !_10;
_20 = core::ptr::addr_of_mut!((*_20));
_23 = _7.2 * _7.2;
_18 = [_17,_7.2,_23];
match RET {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
121 => bb8,
_ => bb7
}
}
bb4 = {
_12 = Field::<char>(Variant(_19, 1), 1);
_21 = _2;
_9.0 = _6;
_12 = Field::<char>(Variant(_16, 1), 1);
_19 = Adt42::Variant1 { fld0: 6643186294239341758_u64,fld1: _12,fld2: _14,fld3: _9.1 };
_3 = [_10,_17,_10];
_9.1 = Field::<i32>(Variant(_16, 1), 3);
_22 = (_3,);
_7.0 = _6;
_9.2 = (-105_i8) as isize;
place!(Field::<i32>(Variant(_19, 1), 3)) = _6 as i32;
_1 = _5;
place!(Field::<u64>(Variant(_16, 1), 0)) = !8311042948987313468_u64;
Goto(bb3)
}
bb5 = {
_21 = 13371474668550907411_usize as i64;
_19 = Adt42::Variant1 { fld0: 11216871307906165440_u64,fld1: _12,fld2: _14,fld3: _7.1 };
_9.0 = !_11;
place!(Field::<char>(Variant(_19, 1), 1)) = _12;
place!(Field::<*mut i64>(Variant(_19, 1), 2)) = _14;
_7.2 = -_10;
_16 = Adt42::Variant1 { fld0: 14025239166849466220_u64,fld1: _12,fld2: _14,fld3: _7.1 };
_6 = 50488_u16 as u32;
RET = 121_u8;
_22.0 = _15;
Call(_11 = fn3(_7.2, Field::<char>(Variant(_19, 1), 1), _9.0, _13, _10, _20, Field::<char>(Variant(_16, 1), 1)), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_7.0 = !_11;
Goto(bb9)
}
bb9 = {
_7.1 = !_9.1;
place!(Field::<i32>(Variant(_19, 1), 3)) = Field::<i32>(Variant(_16, 1), 3);
_18 = [_23,_8,_23];
_19 = Adt42::Variant1 { fld0: Field::<u64>(Variant(_16, 1), 0),fld1: _12,fld2: _14,fld3: _9.1 };
_25 = Field::<u64>(Variant(_16, 1), 0) as f32;
_20 = core::ptr::addr_of_mut!((*_20));
_16 = Move(_19);
Goto(bb10)
}
bb10 = {
_26 = [(-65_i8),37_i8,(-103_i8),(-26_i8),(-43_i8)];
RET = 132_u8 | 218_u8;
_21 = _2 ^ _1;
_9.3 = _7.3 << _11;
_24 = (-8_i8) >> Field::<i32>(Variant(_16, 1), 3);
place!(Field::<char>(Variant(_16, 1), 1)) = _12;
_26 = [_24,_24,_24,_24,_24];
_11 = !_6;
_19 = Adt42::Variant1 { fld0: Field::<u64>(Variant(_16, 1), 0),fld1: _12,fld2: _14,fld3: Field::<i32>(Variant(_16, 1), 3) };
RET = !246_u8;
_6 = _5 as u32;
SetDiscriminant(_16, 1);
_27 = _2 << _7.0;
_9.2 = _7.2;
_30 = 9320_u16 as u8;
_9.1 = !Field::<i32>(Variant(_19, 1), 3);
_15 = [_10,_8,_23];
_31 = _11;
_5 = -_27;
place!(Field::<i32>(Variant(_16, 1), 3)) = (-30661_i16) as i32;
_24 = -(-70_i8);
place!(Field::<u64>(Variant(_19, 1), 0)) = _30 as u64;
place!(Field::<char>(Variant(_19, 1), 1)) = _12;
_15 = [_9.2,_23,_7.2];
Goto(bb11)
}
bb11 = {
_28 = true;
_16 = Adt42::Variant1 { fld0: Field::<u64>(Variant(_19, 1), 0),fld1: _12,fld2: _14,fld3: Field::<i32>(Variant(_19, 1), 3) };
place!(Field::<u64>(Variant(_19, 1), 0)) = Field::<u64>(Variant(_16, 1), 0) + Field::<u64>(Variant(_16, 1), 0);
place!(Field::<u64>(Variant(_19, 1), 0)) = 31116_u16 as u64;
_32 = [7009_i16];
_22.0 = _15;
_9.3 = (*_20);
_16 = Move(_19);
_29 = _28 ^ _28;
Goto(bb12)
}
bb12 = {
_3 = _15;
_12 = Field::<char>(Variant(_16, 1), 1);
place!(Field::<u64>(Variant(_16, 1), 0)) = Field::<i32>(Variant(_16, 1), 3) as u64;
_25 = (-117544590487192099697026285486384727625_i128) as f32;
_19 = Adt42::Variant1 { fld0: Field::<u64>(Variant(_16, 1), 0),fld1: Field::<char>(Variant(_16, 1), 1),fld2: Field::<*mut i64>(Variant(_16, 1), 2),fld3: Field::<i32>(Variant(_16, 1), 3) };
_16 = Adt42::Variant1 { fld0: Field::<u64>(Variant(_19, 1), 0),fld1: Field::<char>(Variant(_19, 1), 1),fld2: _14,fld3: _7.1 };
_22 = (_13,);
_5 = (-10845_i16) as i64;
_7.1 = Field::<i32>(Variant(_19, 1), 3) ^ Field::<i32>(Variant(_19, 1), 3);
_26 = [_24,_24,_24,_24,_24];
_1 = -_21;
_7.2 = -_17;
_8 = _21 as isize;
_37 = [_21,_21];
_9.1 = Field::<u64>(Variant(_16, 1), 0) as i32;
place!(Field::<u64>(Variant(_16, 1), 0)) = Field::<u64>(Variant(_19, 1), 0);
_12 = Field::<char>(Variant(_19, 1), 1);
RET = _30 - _30;
place!(Field::<u64>(Variant(_16, 1), 0)) = !Field::<u64>(Variant(_19, 1), 0);
_36 = (RET, _7.2);
_13 = [_23,_4,_23];
Call(_25 = core::intrinsics::transmute(Field::<i32>(Variant(_19, 1), 3)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_7.3 = _9.3;
_1 = _2;
_17 = _7.0 as isize;
_4 = _8;
_15 = [_36.1,_8,_23];
_9.3 = _7.3 * (*_20);
_10 = _8;
place!(Field::<i32>(Variant(_19, 1), 3)) = _7.1;
_23 = _10 & _8;
_17 = 5887847254701146258_usize as isize;
_41.0 = _30;
_24 = 3_i8;
_31 = _11;
place!(Field::<char>(Variant(_16, 1), 1)) = _12;
_10 = _9.2;
_34 = _29;
_21 = 30650_i16 as i64;
_9.3 = (*_20) >> Field::<i32>(Variant(_16, 1), 3);
_22.0 = _15;
_19 = Move(_16);
_16 = Move(_19);
_36.1 = _23 >> (*_20);
RET = _30;
match _24 {
0 => bb1,
1 => bb2,
2 => bb11,
4 => bb5,
5 => bb14,
3 => bb16,
_ => bb15
}
}
bb14 = {
_7.0 = !_11;
Goto(bb9)
}
bb15 = {
_28 = true;
_16 = Adt42::Variant1 { fld0: Field::<u64>(Variant(_19, 1), 0),fld1: _12,fld2: _14,fld3: Field::<i32>(Variant(_19, 1), 3) };
place!(Field::<u64>(Variant(_19, 1), 0)) = Field::<u64>(Variant(_16, 1), 0) + Field::<u64>(Variant(_16, 1), 0);
place!(Field::<u64>(Variant(_19, 1), 0)) = 31116_u16 as u64;
_32 = [7009_i16];
_22.0 = _15;
_9.3 = (*_20);
_16 = Move(_19);
_29 = _28 ^ _28;
Goto(bb12)
}
bb16 = {
_41.1 = _36.1;
_2 = _27;
_2 = !_27;
SetDiscriminant(_16, 0);
_23 = _34 as isize;
_22 = (_15,);
_40.2 = _36.0 + _41.0;
_7 = _9;
_25 = _9.0 as f32;
_44 = _41.1 * _10;
_14 = core::ptr::addr_of_mut!(_21);
place!(Field::<[i16; 1]>(Variant(_16, 0), 0)) = _32;
_25 = 63867_u16 as f32;
Goto(bb17)
}
bb17 = {
Call(_47 = dump_var(2_usize, 41_usize, Move(_41), 3_usize, Move(_3), 12_usize, Move(_12), 32_usize, Move(_32)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(2_usize, 9_usize, Move(_9), 26_usize, Move(_26), 24_usize, Move(_24), 28_usize, Move(_28)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_47 = dump_var(2_usize, 10_usize, Move(_10), 29_usize, Move(_29), 7_usize, Move(_7), 30_usize, Move(_30)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_47 = dump_var(2_usize, 4_usize, Move(_4), 5_usize, Move(_5), 17_usize, Move(_17), 27_usize, Move(_27)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: char,mut _3: u32,mut _4: [isize; 3],mut _5: isize,mut _6: *mut u128,mut _7: char) -> u32 {
mir! {
type RET = u32;
let _8: Adt50;
let _9: usize;
let _10: usize;
let _11: [u128; 4];
let _12: ([u128; 4], &'static i64, [isize; 3], (char, *mut *const u8, u8, *mut *const u8));
let _13: [i8; 5];
let _14: *const u8;
let _15: (u8, isize);
let _16: i64;
let _17: &'static i64;
let _18: [bool; 3];
let _19: char;
let _20: (u8, isize);
let _21: Adt46;
let _22: [i8; 5];
let _23: u16;
let _24: *mut i64;
let _25: u128;
let _26: isize;
let _27: [i8; 5];
let _28: f32;
let _29: ([i16; 1], i8, *mut u128);
let _30: char;
let _31: [i64; 2];
let _32: Adt56;
let _33: [i16; 2];
let _34: i32;
let _35: i64;
let _36: Adt53;
let _37: [i16; 2];
let _38: u64;
let _39: *mut u128;
let _40: bool;
let _41: [i16; 2];
let _42: [i16; 1];
let _43: f32;
let _44: u8;
let _45: i8;
let _46: ([i16; 1], i8, *mut u128);
let _47: i32;
let _48: f32;
let _49: ();
let _50: ();
{
_3 = 18307867_u32 - 2091410746_u32;
RET = _3;
RET = !_3;
_5 = !_1;
_2 = _7;
_2 = _7;
_7 = _2;
_1 = _5 | _5;
RET = _3 ^ _3;
_4 = [_1,_5,_1];
_2 = _7;
_10 = !1_usize;
_3 = !RET;
_11 = [220068308128563990161980800791119684860_u128,260866790061279674144427172937869345676_u128,46590476759374729782396002391399619339_u128,119487196007156650003515932888958760835_u128];
Goto(bb1)
}
bb1 = {
_2 = _7;
_11 = [83695566347359768567775992840442660472_u128,184950984528150757996297253529398571798_u128,125682771880135701136890223112844795735_u128,337525908368918079016742098948246484555_u128];
_10 = _7 as usize;
_3 = RET;
_4 = [_1,_1,_1];
_9 = (-9640_i16) as usize;
_7 = _2;
_11 = [319349907012189751601401021942353733767_u128,140265845431903081117572970611098911791_u128,52386064624486691367298746198904518324_u128,187731996016597354672168052893672284874_u128];
_4 = [_1,_1,_1];
_3 = !RET;
RET = _3 - _3;
_12.3.3 = core::ptr::addr_of_mut!(_14);
RET = _3;
_15.1 = _5 * _1;
_12.3.3 = core::ptr::addr_of_mut!(_14);
_2 = _7;
_12.3.2 = 65_u8 | 82_u8;
_10 = _9;
_11 = [4554052377541546243236233249373104186_u128,60585515020611332525675985038063849870_u128,304387165346045681262792428912472235072_u128,298074371839598878846126816438136700443_u128];
_15.0 = !_12.3.2;
_14 = core::ptr::addr_of!(_15.0);
_14 = core::ptr::addr_of!((*_14));
_4 = [_15.1,_15.1,_1];
RET = _3 * _3;
_12.2 = [_1,_1,_1];
Goto(bb2)
}
bb2 = {
_13 = [63_i8,50_i8,23_i8,70_i8,(-87_i8)];
_12.3.1 = core::ptr::addr_of_mut!(_14);
_12.2 = [_15.1,_15.1,_1];
_12.2 = _4;
_9 = _10;
_7 = _2;
_9 = _10 >> _5;
_15.0 = _12.3.2 - _12.3.2;
_7 = _2;
_1 = true as isize;
_13 = [(-114_i8),59_i8,15_i8,(-45_i8),(-30_i8)];
_15.0 = _12.3.2;
_11 = [168835614906610737963357389189024805680_u128,277709183892662383183562411456211432668_u128,144098456118999827473638805128149983146_u128,317167917804902326992278200491291628885_u128];
_2 = _7;
Call(_12.3.0 = fn4(_12.2, _3, _5, _4, _12.3.2, _10, _15.1, _15.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12.0 = _11;
_13 = [125_i8,72_i8,(-24_i8),53_i8,51_i8];
_9 = 13489800071891442081_u64 as usize;
_2 = _12.3.0;
_2 = _12.3.0;
_12.3.1 = core::ptr::addr_of_mut!(_14);
_14 = core::ptr::addr_of!(_12.3.2);
_12.3.1 = _12.3.3;
_10 = _9;
_12.3.0 = _2;
Goto(bb4)
}
bb4 = {
_2 = _7;
_15 = ((*_14), _5);
_9 = _10;
_12.3.0 = _2;
_12.3.3 = core::ptr::addr_of_mut!(_14);
_11 = [333703271041939107614794796436661259450_u128,34796704489908483575428778629524078487_u128,189469194779601652341911815162154707489_u128,263854414527795796268862314333549445019_u128];
RET = _3 >> _10;
_3 = RET ^ RET;
_4 = _12.2;
_12.3.3 = core::ptr::addr_of_mut!(_14);
_3 = !RET;
_13 = [114_i8,103_i8,127_i8,(-15_i8),50_i8];
_9 = !_10;
_12.3.3 = core::ptr::addr_of_mut!(_14);
_14 = core::ptr::addr_of!((*_14));
Goto(bb5)
}
bb5 = {
_13 = [(-35_i8),114_i8,31_i8,(-59_i8),91_i8];
_12.2 = [_5,_5,_15.1];
_2 = _7;
_5 = !_15.1;
_14 = core::ptr::addr_of!(_12.3.2);
_3 = !RET;
_12.3.3 = core::ptr::addr_of_mut!(_14);
_16 = (-6542038253458339497_i64);
_10 = !_9;
_13 = [(-100_i8),117_i8,(-123_i8),(-41_i8),42_i8];
RET = 97_i8 as u32;
_13 = [(-120_i8),(-4_i8),88_i8,30_i8,70_i8];
_9 = _10 >> (*_14);
_3 = !RET;
_2 = _7;
_11 = _12.0;
_10 = _9 - _9;
_12.3.0 = _2;
Goto(bb6)
}
bb6 = {
_20 = ((*_14), _15.1);
_20.0 = 127613982715692198857754500228342478869_u128 as u8;
_12.2 = _4;
_17 = &_16;
_12.1 = Move(_17);
_12.1 = &_16;
_12.3.3 = core::ptr::addr_of_mut!(_14);
RET = _3;
_12.0 = [147852497772515898179217558118274371984_u128,6880274093354754427277899252207108203_u128,96862213306033401138308584036059169577_u128,114345117550209036903898186840091161856_u128];
_1 = _5 | _5;
_5 = _1 << _15.1;
_12.3.0 = _2;
_12.3.2 = _1 as u8;
_9 = _3 as usize;
_20 = ((*_14), _1);
_10 = _9 >> _12.3.2;
_20.1 = -_5;
_10 = _20.1 as usize;
Goto(bb7)
}
bb7 = {
_25 = 321348213287909155255765281105129098597_u128;
_12.1 = &_16;
_12.3.2 = _20.0;
_21 = Adt46::Variant2 { fld0: 5678100028250266457_u64 };
_27 = _13;
_17 = &_16;
_18 = [false,true,true];
_27 = [103_i8,3_i8,1_i8,(-24_i8),(-29_i8)];
_13 = [(-80_i8),(-64_i8),(-56_i8),(-6_i8),18_i8];
_20 = (_12.3.2, _5);
_14 = core::ptr::addr_of!(_12.3.2);
_6 = core::ptr::addr_of_mut!(_25);
_22 = [(-50_i8),(-61_i8),2_i8,(-59_i8),3_i8];
_25 = 38523243636446329894808404897661925583_u128;
_12.3.2 = !_15.0;
_29.1 = (-119_i8) - (-122_i8);
_29.2 = core::ptr::addr_of_mut!(_25);
_31 = [(*_17),(*_17)];
_26 = _5;
_12.3.0 = _7;
_20.1 = !_15.1;
_15 = ((*_14), _1);
_26 = _1;
place!(Field::<u64>(Variant(_21, 2), 0)) = 4523961114015184755_u64 ^ 11601225887277840278_u64;
_7 = _2;
match (*_17) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463456832569178309871959 => bb9,
_ => bb8
}
}
bb8 = {
_13 = [(-35_i8),114_i8,31_i8,(-59_i8),91_i8];
_12.2 = [_5,_5,_15.1];
_2 = _7;
_5 = !_15.1;
_14 = core::ptr::addr_of!(_12.3.2);
_3 = !RET;
_12.3.3 = core::ptr::addr_of_mut!(_14);
_16 = (-6542038253458339497_i64);
_10 = !_9;
_13 = [(-100_i8),117_i8,(-123_i8),(-41_i8),42_i8];
RET = 97_i8 as u32;
_13 = [(-120_i8),(-4_i8),88_i8,30_i8,70_i8];
_9 = _10 >> (*_14);
_3 = !RET;
_2 = _7;
_11 = _12.0;
_10 = _9 - _9;
_12.3.0 = _2;
Goto(bb6)
}
bb9 = {
_12.1 = &(*_17);
_19 = _7;
_1 = 8694_u16 as isize;
RET = !_3;
SetDiscriminant(_21, 0);
_12.3.1 = _12.3.3;
_3 = RET | RET;
_9 = !_10;
_33 = [(-26281_i16),(-23482_i16)];
_20 = (_15.0, _15.1);
_30 = _2;
_15 = ((*_14), _20.1);
_11 = [(*_6),(*_6),(*_6),(*_6)];
RET = !_3;
place!(Field::<f64>(Variant(_21, 0), 1)) = (-1845_i16) as f64;
_3 = RET;
_28 = _29.1 as f32;
_12.3.0 = _19;
_29.0 = [(-21407_i16)];
_20.0 = (*_14) - _15.0;
_10 = _9 | _9;
_35 = !_16;
_29.1 = (-44_i8) & (-23_i8);
_12.0 = _11;
_30 = _7;
_29.0 = [9218_i16];
_11 = _12.0;
match _16 {
0 => bb10,
340282366920938463456832569178309871959 => bb12,
_ => bb11
}
}
bb10 = {
_13 = [(-35_i8),114_i8,31_i8,(-59_i8),91_i8];
_12.2 = [_5,_5,_15.1];
_2 = _7;
_5 = !_15.1;
_14 = core::ptr::addr_of!(_12.3.2);
_3 = !RET;
_12.3.3 = core::ptr::addr_of_mut!(_14);
_16 = (-6542038253458339497_i64);
_10 = !_9;
_13 = [(-100_i8),117_i8,(-123_i8),(-41_i8),42_i8];
RET = 97_i8 as u32;
_13 = [(-120_i8),(-4_i8),88_i8,30_i8,70_i8];
_9 = _10 >> (*_14);
_3 = !RET;
_2 = _7;
_11 = _12.0;
_10 = _9 - _9;
_12.3.0 = _2;
Goto(bb6)
}
bb11 = {
_12.0 = _11;
_13 = [125_i8,72_i8,(-24_i8),53_i8,51_i8];
_9 = 13489800071891442081_u64 as usize;
_2 = _12.3.0;
_2 = _12.3.0;
_12.3.1 = core::ptr::addr_of_mut!(_14);
_14 = core::ptr::addr_of!(_12.3.2);
_12.3.1 = _12.3.3;
_10 = _9;
_12.3.0 = _2;
Goto(bb4)
}
bb12 = {
_12.1 = &_35;
_26 = -_5;
_16 = _35;
_35 = !_16;
_23 = !58711_u16;
_26 = !_15.1;
_28 = RET as f32;
place!(Field::<[i8; 5]>(Variant(_21, 0), 0)) = [_29.1,_29.1,_29.1,_29.1,_29.1];
place!(Field::<[i8; 5]>(Variant(_21, 0), 0)) = _22;
_9 = !_10;
_20 = _15;
_25 = _2 as u128;
_14 = core::ptr::addr_of!((*_14));
_24 = core::ptr::addr_of_mut!(_35);
_12.1 = &_35;
_29.0 = [(-26388_i16)];
_10 = !_9;
_34 = 787299063_i32;
_12.2 = _4;
_5 = _25 as isize;
_27 = [_29.1,_29.1,_29.1,_29.1,_29.1];
_19 = _30;
Goto(bb13)
}
bb13 = {
_7 = _12.3.0;
_34 = 1272377695_i32 >> _12.3.2;
_26 = _20.1 & _20.1;
_12.2 = [_20.1,_26,_1];
_40 = _34 > _34;
_2 = _30;
_36 = Adt53::Variant0 { fld0: 10689401836272756473931867857401165014_i128,fld1: _12.3 };
place!(Field::<i128>(Variant(_36, 0), 0)) = 130611704185463178768131503463260869906_i128;
_36 = Adt53::Variant0 { fld0: (-1687501156662167892569998887283760256_i128),fld1: _12.3 };
_37 = [(-25971_i16),(-15775_i16)];
_17 = Move(_12.1);
_12.3.2 = Field::<(char, *mut *const u8, u8, *mut *const u8)>(Variant(_36, 0), 1).2 << _26;
_12.3.2 = !_15.0;
Goto(bb14)
}
bb14 = {
_23 = 34782_u16;
_12.3.1 = core::ptr::addr_of_mut!(_14);
_15.0 = _20.0;
place!(Field::<u64>(Variant(_21, 0), 3)) = 11402190326988121519_u64;
_12.1 = &_16;
_40 = !false;
_42 = [(-18718_i16)];
_20.0 = !(*_14);
_34 = 1555526329_i32 | 749734753_i32;
RET = _15.0 as u32;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(3_usize, 10_usize, Move(_10), 15_usize, Move(_15), 30_usize, Move(_30), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(3_usize, 13_usize, Move(_13), 40_usize, Move(_40), 20_usize, Move(_20), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(3_usize, 18_usize, Move(_18), 3_usize, Move(_3), 25_usize, Move(_25), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(3_usize, 7_usize, Move(_7), 9_usize, Move(_9), 50_usize, _50, 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [isize; 3],mut _2: u32,mut _3: isize,mut _4: [isize; 3],mut _5: u8,mut _6: usize,mut _7: isize,mut _8: isize) -> char {
mir! {
type RET = char;
let _9: bool;
let _10: i32;
let _11: bool;
let _12: f64;
let _13: i32;
let _14: *const u8;
let _15: [i128; 5];
let _16: [i16; 2];
let _17: Adt44;
let _18: usize;
let _19: f64;
let _20: Adt49;
let _21: [i16; 2];
let _22: [i16; 2];
let _23: isize;
let _24: [i16; 2];
let _25: f64;
let _26: isize;
let _27: (u32, i32, isize, u128);
let _28: [i16; 2];
let _29: i16;
let _30: *const u8;
let _31: *mut *const u8;
let _32: Adt44;
let _33: isize;
let _34: ();
let _35: ();
{
_5 = !96_u8;
RET = '\u{6c4a7}';
_8 = !_7;
_2 = 3855702491_u32;
_1 = [_8,_8,_7];
_2 = 240847597_u32;
RET = '\u{f742c}';
_9 = _6 != _6;
_6 = 2_usize * 12920948152163901208_usize;
Call(_6 = fn5(_1, _7, _1, _3, _7, _9, _1, _8, _3, _8, _7, _4, _8, _8, _1, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !4173019307_u32;
RET = '\u{5e577}';
_3 = _8 << _5;
_10 = -(-1439019550_i32);
_5 = 3391259309755960732_i64 as u8;
Goto(bb2)
}
bb2 = {
_7 = _3;
_5 = 86_u8;
_5 = _6 as u8;
_8 = -_3;
RET = '\u{a1a28}';
_11 = !_9;
_5 = 177_u8;
_1 = _4;
_7 = 41905_u16 as isize;
_13 = _10 >> _3;
Goto(bb3)
}
bb3 = {
_7 = _3;
_15 = [102654765908409834944894522439167780478_i128,26720014010479318465856918185013848401_i128,(-142859017399877779472220226810046332843_i128),166105062265403397501161682914963427119_i128,(-78304658067582510028478769271353394630_i128)];
_8 = _7 << _7;
_3 = _8 << _13;
_15 = [(-159479196483142027184467488246401906985_i128),(-71537021501745839584650856759086114091_i128),(-149948764626417395415069559158556155363_i128),39047715332967641844836642155206634067_i128,(-99613042775271673859589103413440086007_i128)];
_13 = _10;
_11 = _3 > _7;
_10 = _13;
_15 = [(-168330763168109376568584573905399898476_i128),(-123800453180698547375138918163832256901_i128),(-64436605404103741891178227949959429856_i128),104466053159100328116912805299302675245_i128,(-144975092348875909250916681008254712589_i128)];
_4 = [_3,_3,_7];
_12 = _3 as f64;
_18 = _6;
_16 = [(-13666_i16),31416_i16];
match _5 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
177 => bb9,
_ => bb8
}
}
bb4 = {
_7 = _3;
_5 = 86_u8;
_5 = _6 as u8;
_8 = -_3;
RET = '\u{a1a28}';
_11 = !_9;
_5 = 177_u8;
_1 = _4;
_7 = 41905_u16 as isize;
_13 = _10 >> _3;
Goto(bb3)
}
bb5 = {
_2 = !4173019307_u32;
RET = '\u{5e577}';
_3 = _8 << _5;
_10 = -(-1439019550_i32);
_5 = 3391259309755960732_i64 as u8;
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
_20.fld5 = core::ptr::addr_of_mut!(_20.fld3.fld4);
_20.fld1 = [(-21709541099787488208252868728444151926_i128),(-112221544760090839257159682201446683631_i128),(-25204647961050817968378256899787850869_i128),(-141145402690575126591083954449669041123_i128),(-20790803906673890646895511869533497926_i128)];
_3 = _8 ^ _8;
_11 = !_9;
_20.fld2 = _16;
_20.fld2 = [(-22132_i16),(-3938_i16)];
_20.fld1 = _15;
_20.fld3.fld3 = [22084_i16];
_2 = RET as u32;
_19 = _18 as f64;
_20.fld3.fld2 = _8 >> _3;
RET = '\u{10fbcf}';
_5 = 254_u8 + 59_u8;
_3 = _20.fld3.fld2 + _8;
_20.fld2 = _16;
_13 = _10;
_20.fld3.fld4 = 2932852699772910125_i64;
Goto(bb10)
}
bb10 = {
RET = '\u{e1021}';
_14 = core::ptr::addr_of!(_5);
_20.fld3.fld2 = _3;
_25 = _12 - _12;
_6 = _18;
_19 = (-2982_i16) as f64;
_5 = !11_u8;
_8 = _3;
_23 = 144736735436401458250480738759407321641_i128 as isize;
_8 = _2 as isize;
match _20.fld3.fld4 {
0 => bb3,
2932852699772910125 => bb12,
_ => bb11
}
}
bb11 = {
_7 = _3;
_5 = 86_u8;
_5 = _6 as u8;
_8 = -_3;
RET = '\u{a1a28}';
_11 = !_9;
_5 = 177_u8;
_1 = _4;
_7 = 41905_u16 as isize;
_13 = _10 >> _3;
Goto(bb3)
}
bb12 = {
_20.fld3.fld2 = _3 >> _3;
_27.1 = 2112_i16 as i32;
_20.fld3.fld1 = RET;
_22 = _20.fld2;
_20.fld3.fld0 = core::ptr::addr_of_mut!(_27.3);
_3 = _25 as isize;
_20.fld3.fld5 = 18174799183910686184_u64;
_4 = [_20.fld3.fld2,_20.fld3.fld2,_3];
match _20.fld3.fld4 {
0 => bb13,
2932852699772910125 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
RET = '\u{e1021}';
_14 = core::ptr::addr_of!(_5);
_20.fld3.fld2 = _3;
_25 = _12 - _12;
_6 = _18;
_19 = (-2982_i16) as f64;
_5 = !11_u8;
_8 = _3;
_23 = 144736735436401458250480738759407321641_i128 as isize;
_8 = _2 as isize;
match _20.fld3.fld4 {
0 => bb3,
2932852699772910125 => bb12,
_ => bb11
}
}
bb15 = {
_27.3 = (-12822_i16) as u128;
_20.fld1 = [29432978633869348210063448955403567343_i128,67861419152871446358954743493947102635_i128,(-149509269562849325138183414656397181553_i128),(-16285258350027382930858973936541094580_i128),157007139916241406368299670262354201002_i128];
_21 = _22;
RET = _20.fld3.fld1;
_20.fld6 = [_20.fld3.fld4,_20.fld3.fld4];
_3 = _11 as isize;
_18 = !_6;
_16 = [31628_i16,30387_i16];
_27.0 = _2;
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(4_usize, 22_usize, Move(_22), 6_usize, Move(_6), 18_usize, Move(_18), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(4_usize, 2_usize, Move(_2), 1_usize, Move(_1), 23_usize, Move(_23), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(4_usize, 8_usize, Move(_8), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [isize; 3],mut _2: isize,mut _3: [isize; 3],mut _4: isize,mut _5: isize,mut _6: bool,mut _7: [isize; 3],mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: [isize; 3],mut _13: isize,mut _14: isize,mut _15: [isize; 3],mut _16: isize) -> usize {
mir! {
type RET = usize;
let _17: u16;
let _18: [bool; 3];
let _19: usize;
let _20: char;
let _21: u16;
let _22: [u128; 4];
let _23: Adt47;
let _24: *mut [isize; 3];
let _25: [i64; 2];
let _26: [i8; 5];
let _27: f64;
let _28: i16;
let _29: isize;
let _30: *mut u128;
let _31: [i64; 2];
let _32: f64;
let _33: u16;
let _34: Adt41;
let _35: u32;
let _36: Adt45;
let _37: usize;
let _38: f32;
let _39: Adt40;
let _40: Adt41;
let _41: f32;
let _42: [bool; 3];
let _43: Adt45;
let _44: isize;
let _45: f32;
let _46: isize;
let _47: [i128; 5];
let _48: isize;
let _49: Adt49;
let _50: ();
let _51: ();
{
_9 = _16 >> _10;
RET = !15827568808370573523_usize;
_4 = -_14;
_2 = _9;
_5 = 21394_u16 as isize;
RET = !7_usize;
_15 = _1;
_18 = [_6,_6,_6];
_20 = '\u{95c1b}';
_13 = RET as isize;
_17 = !24044_u16;
RET = !13208545476628552052_usize;
_15 = [_2,_2,_8];
_11 = _2 >> _14;
_19 = !RET;
_21 = _17;
_20 = '\u{20565}';
_8 = _2;
_9 = -_8;
_22 = [250411535862401283093424642799865759534_u128,235400912039920530608490717443789099639_u128,275864959227682605349672263624524256789_u128,224160977468075103646507557025061762000_u128];
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_2 = -_11;
_3 = _15;
_3 = _15;
RET = !_19;
_11 = _20 as isize;
_15 = [_4,_10,_8];
RET = !_19;
_2 = (-1764582133_i32) as isize;
_9 = _17 as isize;
_9 = _14 ^ _10;
_4 = _16;
_11 = _16 ^ _10;
_12 = [_8,_8,_9];
_13 = -_11;
_7 = [_14,_9,_11];
_9 = _10 * _10;
_6 = !true;
_15 = [_11,_11,_14];
_11 = _9;
_7 = [_16,_16,_11];
_15 = [_16,_10,_11];
_15 = [_9,_16,_16];
_18 = [_6,_6,_6];
Call(_17 = fn6(_12, _7, _16, _12, _3, _9, _15, _12, _10, _10, _10, _20, _12, _3, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = _20 as isize;
_10 = 170_u8 as isize;
_24 = core::ptr::addr_of_mut!(_12);
_1 = [_9,_4,_4];
_6 = true & true;
_25 = [(-7094869176379382773_i64),(-8477308761719508674_i64)];
_16 = !_11;
_14 = 108_u8 as isize;
_16 = -_2;
_5 = -_11;
_24 = core::ptr::addr_of_mut!(_12);
Goto(bb4)
}
bb4 = {
_16 = _8 << _5;
_25 = [(-5650013970725154126_i64),232328844355761094_i64];
_12 = [_8,_11,_13];
_28 = (-26247_i16) + (-19659_i16);
_9 = !_4;
_12 = [_11,_16,_16];
_29 = _16;
_18 = [_6,_6,_6];
Goto(bb5)
}
bb5 = {
_9 = _5;
_27 = 242530267_u32 as f64;
_6 = false | true;
_9 = -_11;
_3 = _12;
_16 = 29_u8 as isize;
_11 = _8 ^ _10;
_19 = RET;
_14 = _8 << _4;
_26 = [(-38_i8),(-18_i8),(-105_i8),10_i8,125_i8];
_6 = false;
_11 = _8;
_16 = _8 >> _14;
RET = _19;
_12 = [_5,_29,_8];
_8 = _9;
_33 = _21 | _17;
_18 = [_6,_6,_6];
_33 = _21 - _21;
_32 = _27;
_20 = '\u{51ca0}';
_20 = '\u{2e1f3}';
_6 = !true;
_33 = _17;
_10 = _16;
_27 = -_32;
Goto(bb6)
}
bb6 = {
_31 = [(-9159193224036786745_i64),(-6655673753322202730_i64)];
_1 = [_14,_29,_4];
_13 = (-78966598631609270722865097769770283229_i128) as isize;
_23 = Adt47::Variant2 { fld0: _26,fld1: _25,fld2: _1,fld3: 881151053509919811_i64,fld4: _22 };
_2 = -_5;
_31 = [(-1856227183095638517_i64),1506850066465827517_i64];
_5 = _14 - _10;
RET = _19;
_35 = _21 as u32;
RET = _19 * _19;
_8 = !_16;
_2 = _10;
_21 = _17 << _14;
_37 = RET;
place!(Field::<[i8; 5]>(Variant(_23, 2), 0)) = [(-3_i8),(-126_i8),62_i8,(-88_i8),(-102_i8)];
_4 = !_10;
Goto(bb7)
}
bb7 = {
_39.fld1 = _20;
_18 = [_6,_6,_6];
_19 = !RET;
_12 = [_4,_4,_16];
place!(Field::<i64>(Variant(_23, 2), 3)) = !215097983866203063_i64;
_39.fld2 = _5;
Call(_37 = core::intrinsics::transmute(_16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_35 = 648718493_u32;
_29 = _5 & _14;
_5 = _29;
_37 = _19;
_39.fld3 = [_28];
_2 = _5;
place!(Field::<i64>(Variant(_23, 2), 3)) = (-5270690364327482992_i64);
place!(Field::<[i8; 5]>(Variant(_23, 2), 0)) = [111_i8,(-74_i8),(-69_i8),(-26_i8),8_i8];
_40 = Adt41::Variant1 { fld0: RET,fld1: _39.fld1,fld2: _18 };
_2 = _27 as isize;
_1 = _15;
_14 = 244_u8 as isize;
place!(Field::<usize>(Variant(_40, 1), 0)) = RET | _19;
Goto(bb9)
}
bb9 = {
_40 = Adt41::Variant1 { fld0: RET,fld1: _20,fld2: _18 };
RET = !_37;
_29 = _16;
_31 = [Field::<i64>(Variant(_23, 2), 3),Field::<i64>(Variant(_23, 2), 3)];
_29 = _5 & _11;
_16 = -_4;
_1 = [_8,_8,_10];
_21 = _33 >> _9;
place!(Field::<char>(Variant(_40, 1), 1)) = _39.fld1;
RET = Field::<usize>(Variant(_40, 1), 0) & _19;
RET = _32 as usize;
place!(Field::<char>(Variant(_40, 1), 1)) = _39.fld1;
_25 = [Field::<i64>(Variant(_23, 2), 3),Field::<i64>(Variant(_23, 2), 3)];
_39.fld5 = 15131968572008213219_u64 + 455319720569657002_u64;
_17 = _21 - _21;
_4 = _29 + _5;
_18 = [_6,_6,_6];
_35 = 1762177495_u32 << _4;
_33 = _17 * _21;
_42 = [_6,_6,_6];
place!(Field::<[i8; 5]>(Variant(_23, 2), 0)) = [78_i8,(-56_i8),(-102_i8),(-100_i8),(-11_i8)];
_38 = Field::<usize>(Variant(_40, 1), 0) as f32;
_27 = _32 * _32;
_40 = Adt41::Variant1 { fld0: _19,fld1: _20,fld2: _42 };
_21 = _33 + _33;
_26 = [106_i8,45_i8,(-47_i8),(-28_i8),115_i8];
match Field::<i64>(Variant(_23, 2), 3) {
0 => bb4,
1 => bb7,
2 => bb6,
3 => bb10,
340282366920938463458103917067440728464 => bb12,
_ => bb11
}
}
bb10 = {
_35 = 648718493_u32;
_29 = _5 & _14;
_5 = _29;
_37 = _19;
_39.fld3 = [_28];
_2 = _5;
place!(Field::<i64>(Variant(_23, 2), 3)) = (-5270690364327482992_i64);
place!(Field::<[i8; 5]>(Variant(_23, 2), 0)) = [111_i8,(-74_i8),(-69_i8),(-26_i8),8_i8];
_40 = Adt41::Variant1 { fld0: RET,fld1: _39.fld1,fld2: _18 };
_2 = _27 as isize;
_1 = _15;
_14 = 244_u8 as isize;
place!(Field::<usize>(Variant(_40, 1), 0)) = RET | _19;
Goto(bb9)
}
bb11 = {
Goto(bb2)
}
bb12 = {
_10 = _39.fld5 as isize;
_27 = _32;
_43 = Adt45::Variant1 { fld0: _39.fld3,fld1: _22,fld2: _35,fld3: _21 };
_4 = _29;
_4 = _9 ^ _39.fld2;
_26 = [90_i8,(-17_i8),(-29_i8),36_i8,77_i8];
_19 = 327783932975746551478296900490454510949_u128 as usize;
_14 = _39.fld2 << _16;
_7 = (*_24);
RET = _19;
_9 = _39.fld2 << _14;
_3 = (*_24);
_13 = _28 as isize;
_45 = _38;
_35 = Field::<u32>(Variant(_43, 1), 2);
_34 = Adt41::Variant1 { fld0: Field::<usize>(Variant(_40, 1), 0),fld1: _20,fld2: Field::<[bool; 3]>(Variant(_40, 1), 2) };
place!(Field::<char>(Variant(_40, 1), 1)) = _39.fld1;
match Field::<i64>(Variant(_23, 2), 3) {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
340282366920938463458103917067440728464 => bb19,
_ => bb18
}
}
bb13 = {
Goto(bb2)
}
bb14 = {
_2 = -_11;
_3 = _15;
_3 = _15;
RET = !_19;
_11 = _20 as isize;
_15 = [_4,_10,_8];
RET = !_19;
_2 = (-1764582133_i32) as isize;
_9 = _17 as isize;
_9 = _14 ^ _10;
_4 = _16;
_11 = _16 ^ _10;
_12 = [_8,_8,_9];
_13 = -_11;
_7 = [_14,_9,_11];
_9 = _10 * _10;
_6 = !true;
_15 = [_11,_11,_14];
_11 = _9;
_7 = [_16,_16,_11];
_15 = [_16,_10,_11];
_15 = [_9,_16,_16];
_18 = [_6,_6,_6];
Call(_17 = fn6(_12, _7, _16, _12, _3, _9, _15, _12, _10, _10, _10, _20, _12, _3, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_40 = Adt41::Variant1 { fld0: RET,fld1: _20,fld2: _18 };
RET = !_37;
_29 = _16;
_31 = [Field::<i64>(Variant(_23, 2), 3),Field::<i64>(Variant(_23, 2), 3)];
_29 = _5 & _11;
_16 = -_4;
_1 = [_8,_8,_10];
_21 = _33 >> _9;
place!(Field::<char>(Variant(_40, 1), 1)) = _39.fld1;
RET = Field::<usize>(Variant(_40, 1), 0) & _19;
RET = _32 as usize;
place!(Field::<char>(Variant(_40, 1), 1)) = _39.fld1;
_25 = [Field::<i64>(Variant(_23, 2), 3),Field::<i64>(Variant(_23, 2), 3)];
_39.fld5 = 15131968572008213219_u64 + 455319720569657002_u64;
_17 = _21 - _21;
_4 = _29 + _5;
_18 = [_6,_6,_6];
_35 = 1762177495_u32 << _4;
_33 = _17 * _21;
_42 = [_6,_6,_6];
place!(Field::<[i8; 5]>(Variant(_23, 2), 0)) = [78_i8,(-56_i8),(-102_i8),(-100_i8),(-11_i8)];
_38 = Field::<usize>(Variant(_40, 1), 0) as f32;
_27 = _32 * _32;
_40 = Adt41::Variant1 { fld0: _19,fld1: _20,fld2: _42 };
_21 = _33 + _33;
_26 = [106_i8,45_i8,(-47_i8),(-28_i8),115_i8];
match Field::<i64>(Variant(_23, 2), 3) {
0 => bb4,
1 => bb7,
2 => bb6,
3 => bb10,
340282366920938463458103917067440728464 => bb12,
_ => bb11
}
}
bb16 = {
_14 = _20 as isize;
_10 = 170_u8 as isize;
_24 = core::ptr::addr_of_mut!(_12);
_1 = [_9,_4,_4];
_6 = true & true;
_25 = [(-7094869176379382773_i64),(-8477308761719508674_i64)];
_16 = !_11;
_14 = 108_u8 as isize;
_16 = -_2;
_5 = -_11;
_24 = core::ptr::addr_of_mut!(_12);
Goto(bb4)
}
bb17 = {
_39.fld1 = _20;
_18 = [_6,_6,_6];
_19 = !RET;
_12 = [_4,_4,_16];
place!(Field::<i64>(Variant(_23, 2), 3)) = !215097983866203063_i64;
_39.fld2 = _5;
Call(_37 = core::intrinsics::transmute(_16), ReturnTo(bb8), UnwindUnreachable())
}
bb18 = {
_16 = _8 << _5;
_25 = [(-5650013970725154126_i64),232328844355761094_i64];
_12 = [_8,_11,_13];
_28 = (-26247_i16) + (-19659_i16);
_9 = !_4;
_12 = [_11,_16,_16];
_29 = _16;
_18 = [_6,_6,_6];
Goto(bb5)
}
bb19 = {
place!(Field::<[u128; 4]>(Variant(_43, 1), 1)) = [269355143506273875637481510245064995117_u128,43809034867783384349572475934324626118_u128,44458279371908104907070935947995585731_u128,299521817928708856658515910298744574413_u128];
place!(Field::<[u128; 4]>(Variant(_43, 1), 1)) = [166272037862216829830624432722626051827_u128,65111876592498558003884597522515246209_u128,250316035321480090067983496988257556905_u128,258268840267048310428557760457401648516_u128];
_8 = 924038966_i32 as isize;
_28 = 3632_i16;
_1 = [_14,_14,_9];
_43 = Adt45::Variant1 { fld0: _39.fld3,fld1: Field::<[u128; 4]>(Variant(_23, 2), 4),fld2: _35,fld3: _21 };
_44 = _38 as isize;
_49.fld3.fld1 = _20;
_49.fld6 = [Field::<i64>(Variant(_23, 2), 3),Field::<i64>(Variant(_23, 2), 3)];
_49.fld1 = [(-122094805799820441851717738623036898098_i128),122313443978448391926092586396106822720_i128,161385789012305651941552030220199379082_i128,32061064221890101225058318092500952874_i128,2594285908676363065323007819949226704_i128];
_21 = 185667502_i32 as u16;
_49.fld3.fld2 = _9 & _9;
place!(Field::<[bool; 3]>(Variant(_40, 1), 2)) = [_6,_6,_6];
_49.fld6 = [Field::<i64>(Variant(_23, 2), 3),Field::<i64>(Variant(_23, 2), 3)];
_49.fld0 = [_28];
_48 = 164230785968444125077260166796971607463_i128 as isize;
_24 = core::ptr::addr_of_mut!(_15);
_3 = _12;
_46 = _20 as isize;
_41 = _27 as f32;
_7 = _12;
_47 = [(-154374067223973520688898967960618781436_i128),51099165276849737183949496461619103295_i128,71116414550830906015993756506782419654_i128,93075032929067766839196221849443174580_i128,(-98667129376625345644289488311912106950_i128)];
_41 = (-71361191517621480044605120010451713702_i128) as f32;
_49.fld3.fld5 = !_39.fld5;
_39.fld3 = [_28];
Goto(bb20)
}
bb20 = {
Call(_50 = dump_var(5_usize, 12_usize, Move(_12), 19_usize, Move(_19), 2_usize, Move(_2), 26_usize, Move(_26)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_50 = dump_var(5_usize, 11_usize, Move(_11), 6_usize, Move(_6), 44_usize, Move(_44), 18_usize, Move(_18)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_50 = dump_var(5_usize, 14_usize, Move(_14), 35_usize, Move(_35), 16_usize, Move(_16), 33_usize, Move(_33)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_50 = dump_var(5_usize, 13_usize, Move(_13), 7_usize, Move(_7), 15_usize, Move(_15), 29_usize, Move(_29)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_50 = dump_var(5_usize, 42_usize, Move(_42), 51_usize, _51, 51_usize, _51, 51_usize, _51), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [isize; 3],mut _2: [isize; 3],mut _3: isize,mut _4: [isize; 3],mut _5: [isize; 3],mut _6: isize,mut _7: [isize; 3],mut _8: [isize; 3],mut _9: isize,mut _10: isize,mut _11: isize,mut _12: char,mut _13: [isize; 3],mut _14: [isize; 3],mut _15: [isize; 3]) -> u16 {
mir! {
type RET = u16;
let _16: char;
let _17: f64;
let _18: isize;
let _19: isize;
let _20: Adt45;
let _21: char;
let _22: isize;
let _23: char;
let _24: u64;
let _25: Adt42;
let _26: [i8; 5];
let _27: [isize; 3];
let _28: ();
let _29: ();
{
RET = 41341_u16 | 19270_u16;
_9 = 4_usize as isize;
RET = 35595_u16 + 53367_u16;
_3 = 2_usize as isize;
_14 = _8;
_13 = _14;
_1 = _2;
_9 = _11;
_11 = -_6;
_12 = '\u{c84c4}';
_5 = [_6,_6,_6];
_15 = [_6,_11,_11];
RET = 24645_u16;
RET = (-22108_i16) as u16;
_6 = (-863601398_i32) as isize;
_5 = _2;
_4 = _7;
RET = 10560_u16;
_7 = _15;
_18 = _10 + _10;
_12 = '\u{b58e8}';
RET = 8242_u16;
_15 = [_6,_10,_18];
_9 = -_10;
_6 = !_9;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
8242 => bb8,
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
_19 = _10;
_14 = [_19,_10,_11];
_16 = _12;
_1 = [_19,_10,_9];
Goto(bb9)
}
bb9 = {
_15 = _7;
_2 = [_6,_18,_18];
_15 = [_10,_10,_6];
_18 = !_9;
_19 = _16 as isize;
_21 = _16;
_2 = _7;
_5 = [_9,_11,_10];
Call(_16 = fn7(_13, _7, _13, _14), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = 1587774210_i32 as f64;
Call(_6 = fn8(_1, _11, _2, _8, _2, _5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = 48045304504732220228037710546082672517_i128 as u16;
_22 = _11;
_23 = _12;
_18 = _22 * _6;
_11 = _18 | _18;
_22 = _6;
RET = !28635_u16;
_15 = _2;
_24 = 14680730190026239163_u64 & 1835363184015451985_u64;
_15 = _5;
_17 = 0_usize as f64;
_17 = RET as f64;
_8 = [_11,_11,_18];
_2 = [_18,_18,_11];
_9 = _11 >> _6;
_10 = _11;
_17 = _24 as f64;
_18 = !_11;
_3 = !_9;
_21 = _23;
_17 = 3_usize as f64;
_3 = _11;
_21 = _12;
Call(_22 = fn9(_9, _15, _3, _13, _11, _2, _3, _9), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22 = _10;
_5 = _13;
_26 = [52_i8,(-101_i8),78_i8,(-12_i8),(-47_i8)];
_16 = _12;
_15 = [_9,_9,_10];
_15 = [_22,_22,_10];
_4 = [_9,_11,_18];
_18 = -_6;
_27 = [_18,_22,_6];
_8 = _15;
Call(_22 = fn10(_27, _27, _8, _15, _4, _8, _15, _3, _27, _13), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6 = _3 * _9;
_6 = -_18;
_9 = 27324_i16 as isize;
_3 = (-141872851_i32) as isize;
_1 = [_11,_10,_10];
_1 = [_18,_11,_11];
_13 = _2;
_9 = _18 - _10;
Goto(bb14)
}
bb14 = {
_6 = _9 & _9;
_13 = [_10,_11,_6];
_21 = _16;
_2 = [_9,_11,_6];
_12 = _16;
_13 = _2;
_21 = _16;
_1 = _2;
_21 = _16;
_1 = [_10,_6,_18];
_16 = _12;
_9 = _10;
_21 = _16;
_21 = _23;
_14 = [_6,_9,_9];
_19 = -_18;
_17 = 3325637045796416268_i64 as f64;
_17 = 207_u8 as f64;
_19 = _10;
_9 = _19;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(6_usize, 22_usize, Move(_22), 26_usize, Move(_26), 23_usize, Move(_23), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(6_usize, 27_usize, Move(_27), 11_usize, Move(_11), 10_usize, Move(_10), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(6_usize, 12_usize, Move(_12), 19_usize, Move(_19), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [isize; 3],mut _2: [isize; 3],mut _3: [isize; 3],mut _4: [isize; 3]) -> char {
mir! {
type RET = char;
let _5: Adt54;
let _6: u32;
let _7: Adt50;
let _8: f64;
let _9: bool;
let _10: [isize; 3];
let _11: [i128; 5];
let _12: (u8, isize);
let _13: [bool; 3];
let _14: i16;
let _15: usize;
let _16: ();
let _17: ();
{
_1 = [(-93_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = '\u{cec87}';
_3 = [9223372036854775807_isize,9223372036854775807_isize,(-110_isize)];
RET = '\u{aff20}';
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_4 = [(-121_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_6 = 3713667045_u32;
RET = '\u{f8ff1}';
_3 = [88_isize,9223372036854775807_isize,9223372036854775807_isize];
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_9 = !true;
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),8_isize];
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
3713667045 => bb6,
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
_9 = false;
_8 = (-8891_i16) as f64;
_2 = _1;
_4 = [9223372036854775807_isize,(-76_isize),(-57_isize)];
_6 = !1902793114_u32;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),(-125_isize)];
RET = '\u{c9693}';
Goto(bb7)
}
bb7 = {
_2 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_3 = [31_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_6 = 113431436_u32;
RET = '\u{10d1e4}';
_6 = 1080643959_u32;
_2 = [(-106_isize),(-65_isize),(-65_isize)];
_13 = [_9,_9,_9];
match _6 {
0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
1080643959 => bb14,
_ => bb13
}
}
bb8 = {
_9 = false;
_8 = (-8891_i16) as f64;
_2 = _1;
_4 = [9223372036854775807_isize,(-76_isize),(-57_isize)];
_6 = !1902793114_u32;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),(-125_isize)];
RET = '\u{c9693}';
Goto(bb7)
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
_12.0 = _9 as u8;
_2 = [(-24_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_9 = false;
_14 = (-9914_i16) >> _6;
_12 = (119_u8, (-49_isize));
_2 = [_12.1,_12.1,_12.1];
_12.0 = _6 as u8;
_3 = _4;
_12 = (182_u8, (-9223372036854775808_isize));
_8 = 19819_u16 as f64;
_9 = !false;
_6 = 2473534056_u32;
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(7_usize, 9_usize, Move(_9), 12_usize, Move(_12), 1_usize, Move(_1), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [isize; 3],mut _2: isize,mut _3: [isize; 3],mut _4: [isize; 3],mut _5: [isize; 3],mut _6: [isize; 3]) -> isize {
mir! {
type RET = isize;
let _7: *mut [isize; 3];
let _8: Adt41;
let _9: ();
let _10: ();
{
_7 = core::ptr::addr_of_mut!(_6);
_5 = (*_7);
_5 = [_2,_2,_2];
_3 = _6;
_4 = [_2,_2,_2];
RET = _2;
_1 = _3;
_4 = [_2,_2,_2];
_6 = _4;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(8_usize, 3_usize, Move(_3), 4_usize, Move(_4), 6_usize, Move(_6), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: [isize; 3],mut _3: isize,mut _4: [isize; 3],mut _5: isize,mut _6: [isize; 3],mut _7: isize,mut _8: isize) -> isize {
mir! {
type RET = isize;
let _9: u64;
let _10: [i128; 5];
let _11: isize;
let _12: ();
let _13: ();
{
_6 = [_5,_8,_1];
RET = _7 & _5;
_3 = _7 - _7;
RET = -_3;
_5 = _3;
RET = _3;
_3 = RET;
RET = !_1;
RET = _8 >> _1;
_10 = [119226145520294407046214882200096146025_i128,(-55817879069141855635946164352635845480_i128),(-86472721225964504241626433354110113079_i128),151274785571641441686661682899709735888_i128,14413510147914527293370946304616581471_i128];
_3 = RET | _1;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(9_usize, 10_usize, Move(_10), 2_usize, Move(_2), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [isize; 3],mut _2: [isize; 3],mut _3: [isize; 3],mut _4: [isize; 3],mut _5: [isize; 3],mut _6: [isize; 3],mut _7: [isize; 3],mut _8: isize,mut _9: [isize; 3],mut _10: [isize; 3]) -> isize {
mir! {
type RET = isize;
let _11: [u8; 2];
let _12: [i16; 2];
let _13: f32;
let _14: char;
let _15: [i128; 5];
let _16: isize;
let _17: Adt44;
let _18: &'static i64;
let _19: char;
let _20: [i16; 2];
let _21: *mut *const u8;
let _22: Adt44;
let _23: ([isize; 3],);
let _24: f64;
let _25: [u8; 2];
let _26: *mut *const u8;
let _27: [i64; 2];
let _28: char;
let _29: ([isize; 3],);
let _30: u8;
let _31: isize;
let _32: f64;
let _33: ([isize; 3],);
let _34: Adt42;
let _35: f32;
let _36: i8;
let _37: isize;
let _38: u8;
let _39: i128;
let _40: Adt44;
let _41: char;
let _42: Adt55;
let _43: f64;
let _44: [i64; 2];
let _45: (u8, isize);
let _46: u64;
let _47: i8;
let _48: (u32, i32, isize, u128);
let _49: [i16; 2];
let _50: [i16; 2];
let _51: char;
let _52: Adt51;
let _53: [isize; 3];
let _54: i8;
let _55: i8;
let _56: [bool; 3];
let _57: (u8, isize);
let _58: i32;
let _59: [i16; 1];
let _60: *const *mut i64;
let _61: char;
let _62: usize;
let _63: isize;
let _64: Adt42;
let _65: Adt44;
let _66: *const u8;
let _67: ();
let _68: ();
{
RET = 251804415229509555747215804310468914609_u128 as isize;
_11 = [71_u8,41_u8];
_2 = _3;
_5 = [_8,_8,_8];
_8 = RET;
_8 = RET << RET;
_3 = _7;
_1 = [RET,_8,_8];
_3 = [_8,_8,RET];
_7 = _9;
_12 = [32718_i16,(-26891_i16)];
Call(_7 = fn11(_5, _4, _9, _9, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = 28611_u16 as f32;
_8 = RET | RET;
_9 = [_8,_8,_8];
_14 = '\u{3f814}';
RET = true as isize;
RET = 6_usize as isize;
_8 = RET + RET;
_16 = _8;
_13 = 7_usize as f32;
_7 = [_16,RET,RET];
RET = 338847992859606285983442619266777275117_u128 as isize;
_1 = _2;
_5 = [_16,_8,_8];
_10 = [_16,_16,_8];
_12 = [(-8476_i16),16743_i16];
_16 = 94983024767940792015661710962395143276_u128 as isize;
Goto(bb2)
}
bb2 = {
_4 = [RET,_8,_8];
_3 = [_8,_16,_8];
_2 = _6;
_12 = [(-32457_i16),18337_i16];
_6 = _2;
_16 = (-2117412563_i32) as isize;
_15 = [138993305947503482615217557439688907931_i128,(-157790049823528009737750413149286632543_i128),20089540795569069424558845525332064545_i128,115553240869220996358221794256606907147_i128,14044634761545923057327173545454808359_i128];
_20 = _12;
_6 = [RET,_16,_8];
_19 = _14;
_15 = [163923736450187929824542807107811619194_i128,122104947113535787420502014331749478077_i128,130182947960523567550857928430638169791_i128,(-10696314519073563208839957283052855958_i128),146358880693813788083818107912226563033_i128];
_2 = [_16,_8,_8];
_3 = [_8,_8,_8];
_8 = 1841520763_u32 as isize;
_20 = [19047_i16,1484_i16];
_11 = [165_u8,102_u8];
_25 = _11;
_4 = [_8,_16,_8];
_4 = [RET,_16,_16];
_23.0 = _1;
_11 = [19_u8,153_u8];
Goto(bb3)
}
bb3 = {
_23.0 = [_8,_8,_16];
_16 = 12313289368445049511_u64 as isize;
_20 = [(-23721_i16),25352_i16];
_23.0 = [_8,_8,_8];
_14 = _19;
_24 = 219_u8 as f64;
_25 = [66_u8,26_u8];
_7 = [_8,_8,RET];
_15 = [83555398200888771218721554702217213140_i128,15652458249485157366143636092886820422_i128,(-5575522670248633874538086556483254281_i128),(-147928993641195157177464015200467715648_i128),48146261373962391791350356149234736514_i128];
_24 = 4803_u16 as f64;
_29.0 = _1;
RET = _8;
_29.0 = [_8,_16,_16];
_23.0 = [RET,_8,_8];
RET = !_8;
Goto(bb4)
}
bb4 = {
_6 = [_8,_8,_8];
_9 = [_16,_16,RET];
_4 = _10;
_5 = _1;
_23 = (_10,);
_12 = _20;
_16 = _8;
_28 = _19;
_27 = [(-4863818370037194552_i64),7318331559796248690_i64];
_30 = !0_u8;
_14 = _19;
Goto(bb5)
}
bb5 = {
_11 = [_30,_30];
_13 = 895953770435188127_usize as f32;
_16 = _8 | _8;
Call(_16 = core::intrinsics::bswap(_8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_29.0 = _1;
_19 = _14;
_29.0 = [_16,_16,_16];
_15 = [(-93691365508519225812637820528625417719_i128),(-36434284827905791706066925607457171791_i128),(-6006925790762700312086657482724469933_i128),(-17649315613368241169501800266102518150_i128),(-101533183519754415586741841060868485623_i128)];
_4 = _1;
_4 = [_8,_16,_16];
_24 = (-695697566_i32) as f64;
_32 = 8701722770574987294_u64 as f64;
_23 = (_5,);
_33.0 = _23.0;
_27 = [3033530104514174202_i64,(-6560124112393837845_i64)];
RET = _16;
_16 = -_8;
_4 = _33.0;
_6 = [RET,RET,RET];
_11 = [_30,_30];
_25 = _11;
_36 = -(-23_i8);
_8 = _16;
_32 = 27644_u16 as f64;
_36 = (-39_i8) >> RET;
_35 = -_13;
_33 = (_5,);
_7 = [_16,_8,RET];
Goto(bb7)
}
bb7 = {
_20 = _12;
_5 = [_16,RET,_16];
_5 = _1;
_29.0 = [_16,RET,RET];
_10 = _23.0;
_1 = _23.0;
_10 = _4;
_19 = _28;
_38 = _30;
_14 = _19;
Goto(bb8)
}
bb8 = {
_4 = [_16,RET,RET];
_31 = _16;
_33 = (_23.0,);
_9 = _5;
_10 = [RET,_16,RET];
_29 = _33;
_5 = _1;
_25 = [_38,_38];
_28 = _19;
_31 = 12044379587119833251_usize as isize;
Call(_32 = fn18(_29.0, _33.0, _5), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_25 = _11;
_38 = _30;
_4 = _29.0;
_11 = [_30,_38];
_37 = false as isize;
_29.0 = [_31,_31,_31];
_33.0 = [_37,_16,_16];
_36 = (-94_i8);
_15 = [143534449082004893813107419447360249394_i128,(-9196759952565548966157090475246650010_i128),92300798501630782224789568035369797473_i128,(-58359949932722745948433651110695883246_i128),(-68259930222074793512389284612958273486_i128)];
_39 = !(-152753891201775635501024517581446479777_i128);
_19 = _28;
_36 = (-125_i8) & 47_i8;
_5 = _9;
_36 = _14 as i8;
_6 = [_8,RET,RET];
_32 = -_24;
_32 = -_24;
RET = _37;
_42.fld2 = core::ptr::addr_of_mut!(_42.fld0);
_28 = _19;
_42.fld6.fld3.0 = _28;
_6 = _4;
_11 = [_38,_30];
_35 = 21646_i16 as f32;
Goto(bb10)
}
bb10 = {
_11 = _25;
_25 = _11;
_42.fld6.fld2 = _39 as u128;
_36 = (-42_i8);
_16 = _37 + _31;
_4 = [RET,_16,_31];
_42.fld3 = !3157968123420286119_usize;
_29.0 = _1;
_24 = _32;
_48.3 = _42.fld6.fld2;
_20 = [17059_i16,(-14120_i16)];
_48.3 = _42.fld6.fld2;
_29.0 = [_16,RET,_8];
_48.2 = _16 >> _38;
_2 = [_16,_8,_31];
Goto(bb11)
}
bb11 = {
_6 = [_37,RET,_48.2];
_24 = _32;
_39 = (-135987543413340650081866527020555356175_i128) | (-37875049387753685990544315053572350604_i128);
_48.3 = _42.fld6.fld2 * _42.fld6.fld2;
_49 = [(-19204_i16),5039_i16];
_42.fld7 = _39 | _39;
match _36 {
0 => bb10,
1 => bb12,
340282366920938463463374607431768211414 => bb14,
_ => bb13
}
}
bb12 = {
_13 = 28611_u16 as f32;
_8 = RET | RET;
_9 = [_8,_8,_8];
_14 = '\u{3f814}';
RET = true as isize;
RET = 6_usize as isize;
_8 = RET + RET;
_16 = _8;
_13 = 7_usize as f32;
_7 = [_16,RET,RET];
RET = 338847992859606285983442619266777275117_u128 as isize;
_1 = _2;
_5 = [_16,_8,_8];
_10 = [_16,_16,_8];
_12 = [(-8476_i16),16743_i16];
_16 = 94983024767940792015661710962395143276_u128 as isize;
Goto(bb2)
}
bb13 = {
_25 = _11;
_38 = _30;
_4 = _29.0;
_11 = [_30,_38];
_37 = false as isize;
_29.0 = [_31,_31,_31];
_33.0 = [_37,_16,_16];
_36 = (-94_i8);
_15 = [143534449082004893813107419447360249394_i128,(-9196759952565548966157090475246650010_i128),92300798501630782224789568035369797473_i128,(-58359949932722745948433651110695883246_i128),(-68259930222074793512389284612958273486_i128)];
_39 = !(-152753891201775635501024517581446479777_i128);
_19 = _28;
_36 = (-125_i8) & 47_i8;
_5 = _9;
_36 = _14 as i8;
_6 = [_8,RET,RET];
_32 = -_24;
_32 = -_24;
RET = _37;
_42.fld2 = core::ptr::addr_of_mut!(_42.fld0);
_28 = _19;
_42.fld6.fld3.0 = _28;
_6 = _4;
_11 = [_38,_30];
_35 = 21646_i16 as f32;
Goto(bb10)
}
bb14 = {
_53 = [_48.2,_48.2,_16];
_25 = [_30,_38];
_56 = [false,true,true];
_46 = _42.fld6.fld2 as u64;
_57.0 = _38;
_50 = [1923_i16,(-10565_i16)];
_45.1 = _48.2 | _48.2;
_33.0 = [_45.1,_16,_45.1];
_42.fld4 = _36 as i16;
_55 = _36;
_42.fld6.fld2 = _48.3 * _48.3;
_54 = _55 << _46;
_47 = true as i8;
_49 = _12;
_45.1 = _16 & RET;
Goto(bb15)
}
bb15 = {
Call(_67 = dump_var(10_usize, 47_usize, Move(_47), 3_usize, Move(_3), 31_usize, Move(_31), 53_usize, Move(_53)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_67 = dump_var(10_usize, 49_usize, Move(_49), 54_usize, Move(_54), 39_usize, Move(_39), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_67 = dump_var(10_usize, 29_usize, Move(_29), 2_usize, Move(_2), 12_usize, Move(_12), 50_usize, Move(_50)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_67 = dump_var(10_usize, 23_usize, Move(_23), 10_usize, Move(_10), 14_usize, Move(_14), 55_usize, Move(_55)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_67 = dump_var(10_usize, 6_usize, Move(_6), 16_usize, Move(_16), 68_usize, _68, 68_usize, _68), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [isize; 3],mut _2: [isize; 3],mut _3: [isize; 3],mut _4: [isize; 3],mut _5: [isize; 3]) -> [isize; 3] {
mir! {
type RET = [isize; 3];
let _6: bool;
let _7: [u8; 2];
let _8: bool;
let _9: bool;
let _10: Adt42;
let _11: ([i16; 1], i8, *mut u128);
let _12: [u128; 4];
let _13: Adt46;
let _14: Adt40;
let _15: isize;
let _16: u16;
let _17: ();
let _18: ();
{
RET = [15_isize,40_isize,(-4_isize)];
_5 = [24_isize,(-9223372036854775808_isize),9223372036854775807_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-66_isize)];
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),30_isize];
_4 = [(-70_isize),(-9223372036854775808_isize),5_isize];
_3 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_2 = [51_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_5 = _1;
RET = _1;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),110_isize];
_1 = [(-118_isize),16_isize,(-9223372036854775808_isize)];
_6 = false;
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_3 = _5;
_2 = _5;
_8 = _6 <= _6;
_7 = [193_u8,235_u8];
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_5 = [(-127_isize),9223372036854775807_isize,63_isize];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5 = _1;
_9 = !_8;
_5 = [(-123_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,126_isize,(-9223372036854775808_isize)];
_5 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
Call(_4 = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = [206_u8,121_u8];
_7 = [6_u8,211_u8];
_2 = RET;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [65_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = RET;
RET = [87_isize,(-9223372036854775808_isize),102_isize];
_11.0 = [27878_i16];
RET = [96_isize,9223372036854775807_isize,(-26_isize)];
_12 = [138547502741066671579881357989385068507_u128,144295626299083472579449180191816959624_u128,202142472807310038588790149693352109155_u128,286329977386757530838116376645109858952_u128];
_11.1 = (-23_i8);
RET = [(-30_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _11.1 > _11.1;
Call(_11.0 = fn12(_4, _11.1, _3, _3, _12, _2, _7, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _8;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_8 = _6;
_11.0 = [(-20855_i16)];
_12 = [185946996601694310067560915208255197001_u128,221418717980860543626717270883588021915_u128,81370278098892550707044005309524013237_u128,285853698911623027483532233088544908841_u128];
_1 = _2;
_5 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = !_6;
_7 = [240_u8,20_u8];
match _11.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463463374607431768211433 => bb8,
_ => bb7
}
}
bb3 = {
_7 = [206_u8,121_u8];
_7 = [6_u8,211_u8];
_2 = RET;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [65_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = RET;
RET = [87_isize,(-9223372036854775808_isize),102_isize];
_11.0 = [27878_i16];
RET = [96_isize,9223372036854775807_isize,(-26_isize)];
_12 = [138547502741066671579881357989385068507_u128,144295626299083472579449180191816959624_u128,202142472807310038588790149693352109155_u128,286329977386757530838116376645109858952_u128];
_11.1 = (-23_i8);
RET = [(-30_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _11.1 > _11.1;
Call(_11.0 = fn12(_4, _11.1, _3, _3, _12, _2, _7, _1, _1), ReturnTo(bb2), UnwindUnreachable())
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
_3 = RET;
_11.1 = (-123_i8);
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),123_isize];
_5 = [(-9223372036854775808_isize),(-34_isize),9223372036854775807_isize];
_11.0 = [18941_i16];
_1 = RET;
_1 = RET;
RET = _4;
_14.fld1 = '\u{a49c2}';
_14.fld5 = 302989775989696190_u64 << _11.1;
Goto(bb9)
}
bb9 = {
_1 = _2;
_12 = [173512089964078131729936906419850128141_u128,53345085359921773116304350270674961632_u128,8722139017332197192809765680865238321_u128,126899010514042426593503571432734082191_u128];
_11.1 = 9223372036854775807_isize as i8;
_1 = _5;
RET = _5;
_8 = _9;
_14.fld2 = -(-9223372036854775808_isize);
_14.fld4 = (-1967398201_i32) as i64;
_14.fld5 = 14878418622855024897_u64;
_2 = [_14.fld2,_14.fld2,_14.fld2];
_11.0 = [(-9518_i16)];
_1 = _4;
_14.fld2 = (-60_isize);
_14.fld1 = '\u{5957a}';
_9 = _8;
_8 = _6 >= _9;
RET = _3;
_3 = [_14.fld2,_14.fld2,_14.fld2];
_9 = _8 >= _6;
match _14.fld2 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463463374607431768211396 => bb17,
_ => bb16
}
}
bb10 = {
_3 = RET;
_11.1 = (-123_i8);
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),123_isize];
_5 = [(-9223372036854775808_isize),(-34_isize),9223372036854775807_isize];
_11.0 = [18941_i16];
_1 = RET;
_1 = RET;
RET = _4;
_14.fld1 = '\u{a49c2}';
_14.fld5 = 302989775989696190_u64 << _11.1;
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
_7 = [206_u8,121_u8];
_7 = [6_u8,211_u8];
_2 = RET;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [65_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = RET;
RET = [87_isize,(-9223372036854775808_isize),102_isize];
_11.0 = [27878_i16];
RET = [96_isize,9223372036854775807_isize,(-26_isize)];
_12 = [138547502741066671579881357989385068507_u128,144295626299083472579449180191816959624_u128,202142472807310038588790149693352109155_u128,286329977386757530838116376645109858952_u128];
_11.1 = (-23_i8);
RET = [(-30_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _11.1 > _11.1;
Call(_11.0 = fn12(_4, _11.1, _3, _3, _12, _2, _7, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_7 = [206_u8,121_u8];
_7 = [6_u8,211_u8];
_2 = RET;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [65_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = RET;
RET = [87_isize,(-9223372036854775808_isize),102_isize];
_11.0 = [27878_i16];
RET = [96_isize,9223372036854775807_isize,(-26_isize)];
_12 = [138547502741066671579881357989385068507_u128,144295626299083472579449180191816959624_u128,202142472807310038588790149693352109155_u128,286329977386757530838116376645109858952_u128];
_11.1 = (-23_i8);
RET = [(-30_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _11.1 > _11.1;
Call(_11.0 = fn12(_4, _11.1, _3, _3, _12, _2, _7, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_6 = _8;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_8 = _6;
_11.0 = [(-20855_i16)];
_12 = [185946996601694310067560915208255197001_u128,221418717980860543626717270883588021915_u128,81370278098892550707044005309524013237_u128,285853698911623027483532233088544908841_u128];
_1 = _2;
_5 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = !_6;
_7 = [240_u8,20_u8];
match _11.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463463374607431768211433 => bb8,
_ => bb7
}
}
bb17 = {
_14.fld1 = '\u{83646}';
_15 = _14.fld2 + _14.fld2;
_4 = [_14.fld2,_15,_14.fld2];
RET = _5;
_14.fld3 = [12924_i16];
Goto(bb18)
}
bb18 = {
Call(_17 = dump_var(11_usize, 15_usize, Move(_15), 2_usize, Move(_2), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_17 = dump_var(11_usize, 6_usize, Move(_6), 18_usize, _18, 18_usize, _18, 18_usize, _18), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [isize; 3],mut _2: i8,mut _3: [isize; 3],mut _4: [isize; 3],mut _5: [u128; 4],mut _6: [isize; 3],mut _7: [u8; 2],mut _8: [isize; 3],mut _9: [isize; 3]) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _10: isize;
let _11: [i128; 5];
let _12: i64;
let _13: bool;
let _14: u8;
let _15: char;
let _16: i128;
let _17: isize;
let _18: [i128; 5];
let _19: [i16; 2];
let _20: ([isize; 3],);
let _21: (u8, isize);
let _22: u8;
let _23: u8;
let _24: [i64; 2];
let _25: [i64; 2];
let _26: f32;
let _27: [i128; 5];
let _28: Adt43;
let _29: bool;
let _30: [u128; 4];
let _31: (u8, isize);
let _32: [bool; 3];
let _33: char;
let _34: Adt41;
let _35: f32;
let _36: ();
let _37: ();
{
_6 = _8;
_2 = 92_u8 as i8;
_6 = [(-9223372036854775808_isize),120_isize,42_isize];
RET = [(-17991_i16)];
_3 = [42_isize,9223372036854775807_isize,(-125_isize)];
_2 = 120664249349713834963162408078293262507_i128 as i8;
_7 = [214_u8,227_u8];
_1 = _6;
Goto(bb1)
}
bb1 = {
_2 = !66_i8;
_7 = [246_u8,39_u8];
_10 = !9223372036854775807_isize;
_11 = [101762930624653925138834866833143994450_i128,17901760936982868053303327552532226186_i128,49265254852236223499206173250851776430_i128,(-56709572554523523454337206348675783995_i128),107857912917834714226185679635124735024_i128];
RET = [(-23902_i16)];
_7 = [237_u8,154_u8];
_7 = [27_u8,172_u8];
_7 = [89_u8,183_u8];
_1 = [_10,_10,_10];
_1 = [_10,_10,_10];
_12 = 3639693751423441260_i64;
RET = [24039_i16];
_7 = [105_u8,86_u8];
_9 = _3;
RET = [(-17156_i16)];
_9 = [_10,_10,_10];
_7 = [124_u8,95_u8];
_2 = 121_i8 * (-26_i8);
_14 = !140_u8;
Goto(bb2)
}
bb2 = {
_7 = [_14,_14];
_9 = [_10,_10,_10];
_12 = 14168773867459133808_u64 as i64;
RET = [(-5789_i16)];
_17 = 289835048998956879063400253088644853155_u128 as isize;
_8 = _4;
_1 = [_17,_17,_17];
_3 = [_10,_17,_10];
_5 = [105530819585038642651776180563472258190_u128,227856843841284367432825796821655813289_u128,106404211270429914442269888063803011318_u128,81172183452194491230520583426806273006_u128];
Goto(bb3)
}
bb3 = {
_4 = _6;
_21 = (_14, _17);
_12 = 8242559761951329898_i64;
_20.0 = [_10,_17,_17];
_15 = '\u{55230}';
_3 = [_10,_21.1,_10];
_21.0 = _14;
_25 = [_12,_12];
match _12 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
8242559761951329898 => bb8,
_ => bb7
}
}
bb4 = {
_7 = [_14,_14];
_9 = [_10,_10,_10];
_12 = 14168773867459133808_u64 as i64;
RET = [(-5789_i16)];
_17 = 289835048998956879063400253088644853155_u128 as isize;
_8 = _4;
_1 = [_17,_17,_17];
_3 = [_10,_17,_10];
_5 = [105530819585038642651776180563472258190_u128,227856843841284367432825796821655813289_u128,106404211270429914442269888063803011318_u128,81172183452194491230520583426806273006_u128];
Goto(bb3)
}
bb5 = {
_2 = !66_i8;
_7 = [246_u8,39_u8];
_10 = !9223372036854775807_isize;
_11 = [101762930624653925138834866833143994450_i128,17901760936982868053303327552532226186_i128,49265254852236223499206173250851776430_i128,(-56709572554523523454337206348675783995_i128),107857912917834714226185679635124735024_i128];
RET = [(-23902_i16)];
_7 = [237_u8,154_u8];
_7 = [27_u8,172_u8];
_7 = [89_u8,183_u8];
_1 = [_10,_10,_10];
_1 = [_10,_10,_10];
_12 = 3639693751423441260_i64;
RET = [24039_i16];
_7 = [105_u8,86_u8];
_9 = _3;
RET = [(-17156_i16)];
_9 = [_10,_10,_10];
_7 = [124_u8,95_u8];
_2 = 121_i8 * (-26_i8);
_14 = !140_u8;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_23 = _21.0 >> _21.1;
Call(_23 = fn13(_11, _11, _4, _2, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_15 = '\u{ed8ae}';
_18 = [145569355968701711102243177164005400478_i128,(-100361431851937795975111063050834292655_i128),(-159307257716175282515124336858342116172_i128),(-28465976468755691475036715424885356616_i128),164714002545383726923232681855058577406_i128];
_17 = _21.1 - _10;
_8 = [_10,_10,_17];
_16 = -109201693088003333709832600104170078156_i128;
_16 = _12 as i128;
_27 = [_16,_16,_16,_16,_16];
_6 = _4;
Goto(bb10)
}
bb10 = {
_25 = [_12,_12];
_23 = !_21.0;
_15 = '\u{820ee}';
_28.fld4 = core::ptr::addr_of_mut!(_12);
_28.fld0 = [true,true,true];
_20 = (_6,);
_28.fld0 = [true,false,false];
_15 = '\u{29367}';
_21 = (_23, _10);
_20 = (_4,);
_25 = [_12,_12];
_32 = [false,true,false];
_17 = _10;
_20 = (_4,);
_28.fld3.2 = _2 as u8;
_28.fld1 = [_17,_10,_21.1];
_18 = _11;
_21.0 = _28.fld3.2 ^ _28.fld3.2;
_24 = [_12,_12];
_26 = _16 as f32;
_9 = [_10,_10,_17];
_28.fld2 = 9062616191264304191823861107997060257_u128;
_16 = (-49187415736181765165982977063078335893_i128);
_31 = (_28.fld3.2, _17);
_15 = '\u{e319b}';
_34 = Adt41::Variant0 { fld0: _21 };
match _16 {
0 => bb4,
1 => bb3,
2 => bb11,
3 => bb12,
291094951184756698297391630368689875563 => bb14,
_ => bb13
}
}
bb11 = {
Return()
}
bb12 = {
_23 = _21.0 >> _21.1;
Call(_23 = fn13(_11, _11, _4, _2, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
_2 = !66_i8;
_7 = [246_u8,39_u8];
_10 = !9223372036854775807_isize;
_11 = [101762930624653925138834866833143994450_i128,17901760936982868053303327552532226186_i128,49265254852236223499206173250851776430_i128,(-56709572554523523454337206348675783995_i128),107857912917834714226185679635124735024_i128];
RET = [(-23902_i16)];
_7 = [237_u8,154_u8];
_7 = [27_u8,172_u8];
_7 = [89_u8,183_u8];
_1 = [_10,_10,_10];
_1 = [_10,_10,_10];
_12 = 3639693751423441260_i64;
RET = [24039_i16];
_7 = [105_u8,86_u8];
_9 = _3;
RET = [(-17156_i16)];
_9 = [_10,_10,_10];
_7 = [124_u8,95_u8];
_2 = 121_i8 * (-26_i8);
_14 = !140_u8;
Goto(bb2)
}
bb14 = {
_34 = Adt41::Variant1 { fld0: 17525689902757420447_usize,fld1: _15,fld2: _28.fld0 };
_29 = _31.0 <= _21.0;
place!(Field::<[bool; 3]>(Variant(_34, 1), 2)) = [_29,_29,_29];
_17 = Field::<char>(Variant(_34, 1), 1) as isize;
_18 = [_16,_16,_16,_16,_16];
_21 = (_14, _17);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(12_usize, 1_usize, Move(_1), 2_usize, Move(_2), 29_usize, Move(_29), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(12_usize, 32_usize, Move(_32), 5_usize, Move(_5), 18_usize, Move(_18), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(12_usize, 9_usize, Move(_9), 3_usize, Move(_3), 10_usize, Move(_10), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(12_usize, 17_usize, Move(_17), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i128; 5],mut _2: [i128; 5],mut _3: [isize; 3],mut _4: i8,mut _5: [isize; 3]) -> u8 {
mir! {
type RET = u8;
let _6: char;
let _7: f64;
let _8: isize;
let _9: isize;
let _10: Adt56;
let _11: Adt44;
let _12: Adt47;
let _13: isize;
let _14: f64;
let _15: char;
let _16: usize;
let _17: i128;
let _18: [i128; 5];
let _19: Adt53;
let _20: Adt43;
let _21: [bool; 3];
let _22: *const *mut i64;
let _23: f64;
let _24: isize;
let _25: f64;
let _26: ();
let _27: ();
{
_3 = _5;
_4 = -28_i8;
_3 = _5;
_2 = [(-103121221864045414403155766125930290218_i128),(-117654698832759037969222195487672578002_i128),95211832401332604422938511319172125873_i128,(-143368807891288222897234022008562037171_i128),5175086023644332553736228423207415608_i128];
RET = 82_u8 << _4;
_6 = '\u{48d80}';
_3 = _5;
_2 = _1;
_6 = '\u{f187b}';
_1 = [(-34945519440509462997411703223821389176_i128),(-89976482396233682232825946694880723227_i128),(-88201655258642851246687031465688804560_i128),(-150913675539925612286073024980126697406_i128),(-12546645040595206354855671248092580642_i128)];
_3 = [9223372036854775807_isize,107_isize,(-9223372036854775808_isize)];
_6 = '\u{902b}';
_1 = [(-9178118762525198183160849710049797607_i128),(-61073342131534604241373959532844417883_i128),125684760124480626121314506249280136143_i128,38961731373869069268994426802870001718_i128,105565020825220927135255147106612363182_i128];
RET = 74_u8;
_1 = [(-118983465566813487675088369000417755864_i128),(-129546256503123709342043271695627394371_i128),(-24604911021858709949858937474616675457_i128),52303330922172113844535713663830447218_i128,92156493938689273599165720579317815322_i128];
RET = 178_u8;
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6 = '\u{611f7}';
_3 = [(-66_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_8 = _4 as isize;
_8 = 3778900388_u32 as isize;
_4 = 51_i8 | (-1_i8);
Goto(bb1)
}
bb1 = {
_5 = [_8,_8,_8];
_9 = -_8;
_4 = 99_i8 - (-54_i8);
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
178 => bb8,
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
RET = 145_u8;
_9 = -_8;
RET = !160_u8;
_3 = _5;
_8 = !_9;
_1 = _2;
_7 = 10799463566276120781_u64 as f64;
_4 = 88_i8;
_8 = !_9;
_6 = '\u{5bbbb}';
_4 = 18_i8;
RET = 247_u8;
_4 = 12_i8;
_7 = 1663851324_i32 as f64;
_7 = 1537330553_u32 as f64;
_9 = -_8;
_2 = _1;
_5 = [_9,_8,_9];
RET = 16600657445844050416_usize as u8;
_6 = '\u{fc864}';
_4 = 29051_u16 as i8;
_2 = [151418334211093277023639492032398518981_i128,(-33279515571750244981638724137113169580_i128),140847503375655296025919882793369945653_i128,56071589521242425041235435739529126246_i128,84141547926519830941000094646510250983_i128];
_5 = [_8,_9,_9];
_14 = -_7;
Goto(bb9)
}
bb9 = {
_7 = _14 + _14;
_14 = _7;
_6 = '\u{48d95}';
_1 = [100491844404713354301121518627896880930_i128,131528621548703559971142036059104255423_i128,(-150358252448927754165086485190882869598_i128),(-135724687389738670766418389746037727724_i128),(-147266847681115719355634011698695647666_i128)];
RET = _6 as u8;
_13 = !_9;
_15 = _6;
RET = 56_u8;
_1 = [49983900994929320582523148269689144340_i128,(-107683525716144532051260615065503183248_i128),(-80884469222012062929556528649245705527_i128),(-141847354383527219650100428656507879845_i128),64497485180997354940037511487079393906_i128];
RET = !82_u8;
_17 = -33203407601806613660722333697084273357_i128;
_13 = _17 as isize;
RET = 60_u8 << _8;
_6 = _15;
Goto(bb10)
}
bb10 = {
_15 = _6;
_18 = [_17,_17,_17,_17,_17];
_2 = _1;
_16 = 1_usize;
_6 = _15;
_18 = [_1[_16],_1[_16],_1[_16],_1[_16],_2[_16]];
RET = 163_u8 & 208_u8;
_20.fld1[_16] = _3[_16] >> _2[_16];
_7 = _16 as f64;
_16 = _4 as usize;
_21 = [false,false,true];
Goto(bb11)
}
bb11 = {
_8 = !_9;
_17 = (-137339637547367820380731391110753007873_i128);
_17 = 60436_u16 as i128;
_20.fld0 = _21;
_2 = [_17,_17,_17,_17,_17];
_23 = 11617445924525014281_u64 as f64;
_22 = core::ptr::addr_of!(_20.fld4);
_16 = 299609083206123061282525456946983275737_u128 as usize;
_20.fld3.2 = RET;
_14 = _7;
_14 = _23;
_13 = _8;
_6 = _15;
_20.fld1 = [_8,_13,_9];
_20.fld1 = [_8,_9,_8];
RET = _20.fld3.2;
_20.fld2 = 40431680237308513168417751143453990630_u128;
_4 = _14 as i8;
_22 = core::ptr::addr_of!((*_22));
_5 = [_13,_13,_9];
_17 = (-44651090409356250362746685236324648244_i128) - (-115659595192122060041319598850406507251_i128);
_1 = _18;
_20.fld3.0 = _15;
Call(_20.fld3.0 = fn14(_5, _9, RET, _1, _13, _21, _23), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_20.fld0 = _21;
_5 = [_13,_8,_9];
_20.fld1 = [_8,_13,_9];
_15 = _6;
_17 = _7 as i128;
RET = _20.fld3.2;
_23 = -_14;
_20.fld3.2 = RET;
_6 = _20.fld3.0;
_23 = 7658142830898462355_i64 as f64;
_16 = _20.fld2 as usize;
_7 = _4 as f64;
_15 = _6;
_20.fld3.0 = _15;
RET = _20.fld3.2 | _20.fld3.2;
_3 = [_9,_13,_8];
_21 = [true,true,true];
_2 = _1;
_6 = _15;
Goto(bb13)
}
bb13 = {
_7 = _14;
Goto(bb14)
}
bb14 = {
RET = !_20.fld3.2;
_4 = !70_i8;
_8 = _4 as isize;
_25 = _7;
_4 = _16 as i8;
_5 = _3;
_22 = core::ptr::addr_of!((*_22));
_2 = [_17,_17,_17,_17,_17];
_13 = -_9;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(13_usize, 3_usize, Move(_3), 13_usize, Move(_13), 4_usize, Move(_4), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(13_usize, 18_usize, Move(_18), 5_usize, Move(_5), 9_usize, Move(_9), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [isize; 3],mut _2: isize,mut _3: u8,mut _4: [i128; 5],mut _5: isize,mut _6: [bool; 3],mut _7: f64) -> char {
mir! {
type RET = char;
let _8: Adt41;
let _9: usize;
let _10: [i64; 2];
let _11: *const *mut [isize; 3];
let _12: isize;
let _13: i32;
let _14: bool;
let _15: isize;
let _16: i16;
let _17: bool;
let _18: [u8; 2];
let _19: [i64; 2];
let _20: char;
let _21: *mut [isize; 3];
let _22: (u8, isize);
let _23: i128;
let _24: (char, *mut *const u8, u8, *mut *const u8);
let _25: char;
let _26: i8;
let _27: u64;
let _28: [i16; 2];
let _29: i16;
let _30: (u32, i32, isize, u128);
let _31: f32;
let _32: Adt46;
let _33: Adt52;
let _34: f64;
let _35: Adt41;
let _36: (char, *mut *const u8, u8, *mut *const u8);
let _37: u128;
let _38: [u128; 4];
let _39: [i16; 1];
let _40: [i16; 2];
let _41: Adt51;
let _42: ();
let _43: ();
{
RET = '\u{9b633}';
_5 = _2;
_6 = [false,true,false];
_2 = _5 + _5;
_2 = 1365234701_i32 as isize;
_5 = !_2;
_7 = _2 as f64;
Goto(bb1)
}
bb1 = {
_7 = _3 as f64;
_7 = _3 as f64;
RET = '\u{8ae81}';
_4 = [60769599722445572981271981454514274923_i128,131143769537748210686375966790536515154_i128,151143940620593398655638299962252737879_i128,60787910964161326200886558151425946258_i128,103263267997771784671614346587127277649_i128];
_4 = [(-43440211877250880509860792535000464969_i128),(-142089292343021368916995848965770918673_i128),(-48715762839986081366872331673154792625_i128),155197218160296693698616583828031315062_i128,86913467152684917910267137185252586284_i128];
_1 = [_2,_5,_5];
_4 = [(-127829285279207069939169899730540229370_i128),32977780400076249655524468928011856074_i128,23082217881703063914096087182774739120_i128,92800766696153032360570922585711235299_i128,18752994588414124906567891731293863522_i128];
_2 = !_5;
_2 = !_5;
RET = '\u{d64bf}';
Goto(bb2)
}
bb2 = {
_1 = [_2,_2,_5];
RET = '\u{c43e3}';
_6 = [false,false,false];
RET = '\u{73bcf}';
_8 = Adt41::Variant1 { fld0: 4_usize,fld1: RET,fld2: _6 };
_4 = [157622684693712860205684402707774970593_i128,164136700264354750731061278680736623687_i128,(-40946788682968688648990813936821063166_i128),246936440869709561642372060519057803_i128,(-151437679548407793719664307796575454781_i128)];
RET = Field::<char>(Variant(_8, 1), 1);
_7 = (-20876021559592458958151946602812345682_i128) as f64;
RET = Field::<char>(Variant(_8, 1), 1);
_5 = (-62500338013864591416037367272067724215_i128) as isize;
place!(Field::<[bool; 3]>(Variant(_8, 1), 2)) = [false,true,false];
place!(Field::<[bool; 3]>(Variant(_8, 1), 2)) = [true,false,false];
_10 = [5435951986035609500_i64,(-235201648593744926_i64)];
_6 = [true,true,true];
_1 = [_2,_5,_2];
_8 = Adt41::Variant1 { fld0: 5_usize,fld1: RET,fld2: _6 };
_9 = !6251915131395848011_usize;
RET = Field::<char>(Variant(_8, 1), 1);
_5 = _2;
Goto(bb3)
}
bb3 = {
place!(Field::<char>(Variant(_8, 1), 1)) = RET;
RET = Field::<char>(Variant(_8, 1), 1);
_5 = _2;
place!(Field::<char>(Variant(_8, 1), 1)) = RET;
_4 = [97316249325423377354832023070759290728_i128,159900424297687494383715799134196534778_i128,(-66302132889357012538229853565105116173_i128),(-14535081368109574714930290997380652674_i128),6856561952692857685439360701782831811_i128];
_10 = [8666047215441417169_i64,8640945854478727164_i64];
RET = Field::<char>(Variant(_8, 1), 1);
Call(_4 = fn15(Field::<[bool; 3]>(Variant(_8, 1), 2), _10, _5, _5, Field::<[bool; 3]>(Variant(_8, 1), 2), _6, Field::<[bool; 3]>(Variant(_8, 1), 2)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
place!(Field::<usize>(Variant(_8, 1), 0)) = _9;
_8 = Adt41::Variant1 { fld0: _9,fld1: RET,fld2: _6 };
_3 = 5571802348689116566_i64 as u8;
place!(Field::<usize>(Variant(_8, 1), 0)) = true as usize;
_1 = [_2,_5,_5];
_3 = _2 as u8;
_4 = [3859123694303697920484235688496098624_i128,142416289483982756511858023934567617022_i128,(-50282648048563386802452327012730151578_i128),108240198374062558178405275233654639327_i128,76267302081575269776057985857364504964_i128];
_12 = -_2;
_3 = !13_u8;
RET = Field::<char>(Variant(_8, 1), 1);
RET = Field::<char>(Variant(_8, 1), 1);
_9 = !Field::<usize>(Variant(_8, 1), 0);
_6 = [true,false,false];
RET = Field::<char>(Variant(_8, 1), 1);
_4 = [170010706696123079770989212616623265745_i128,134086281018530726490158543204612754438_i128,(-95172677012468667652288844462437263324_i128),9249835486747196457754367007826152195_i128,(-62645566779157236226276527485417909902_i128)];
_8 = Adt41::Variant1 { fld0: _9,fld1: RET,fld2: _6 };
_2 = _5;
_6 = [true,true,true];
place!(Field::<[bool; 3]>(Variant(_8, 1), 2)) = [false,true,true];
RET = Field::<char>(Variant(_8, 1), 1);
Goto(bb5)
}
bb5 = {
Goto(bb6)
}
bb6 = {
place!(Field::<usize>(Variant(_8, 1), 0)) = Field::<char>(Variant(_8, 1), 1) as usize;
_17 = false;
RET = Field::<char>(Variant(_8, 1), 1);
_14 = _17 == _17;
_5 = _2;
_10 = [5270057368530762627_i64,251161355767065919_i64];
_9 = Field::<usize>(Variant(_8, 1), 0) & Field::<usize>(Variant(_8, 1), 0);
RET = Field::<char>(Variant(_8, 1), 1);
_16 = 7902_i16 << _2;
_5 = _2;
SetDiscriminant(_8, 0);
_5 = _2 - _12;
_1 = [_5,_5,_12];
_13 = (-1066393053_i32) * 1927988533_i32;
_18 = [_3,_3];
_7 = 4257274492596556152_i64 as f64;
_8 = Adt41::Variant1 { fld0: _9,fld1: RET,fld2: _6 };
_15 = _2;
RET = Field::<char>(Variant(_8, 1), 1);
place!(Field::<usize>(Variant(_8, 1), 0)) = _9;
place!(Field::<usize>(Variant(_8, 1), 0)) = !_9;
_17 = _14;
_3 = _14 as u8;
Goto(bb7)
}
bb7 = {
_10 = [3993933206457782135_i64,(-737287455313282452_i64)];
_12 = _5 + _5;
_9 = !Field::<usize>(Variant(_8, 1), 0);
_20 = RET;
_15 = _2 & _12;
_18 = [_3,_3];
_20 = RET;
_10 = [8669674670884941306_i64,2803365013539396098_i64];
_5 = 14926002458479987553232308242445528001_u128 as isize;
place!(Field::<[bool; 3]>(Variant(_8, 1), 2)) = [_14,_17,_17];
_3 = _7 as u8;
_14 = !_17;
_23 = 205569812500224343635203078911760841454_u128 as i128;
RET = _20;
_22.0 = _3 | _3;
_7 = 98_i8 as f64;
Goto(bb8)
}
bb8 = {
_18 = [_22.0,_22.0];
_9 = Field::<usize>(Variant(_8, 1), 0) * Field::<usize>(Variant(_8, 1), 0);
_14 = _17;
_23 = (-50525705938075922534678439091067401386_i128);
_3 = _17 as u8;
_12 = _15 >> _2;
_24.0 = Field::<char>(Variant(_8, 1), 1);
_11 = core::ptr::addr_of!(_21);
_14 = !_17;
Goto(bb9)
}
bb9 = {
_19 = [(-9190754415704613842_i64),(-1793554437405145060_i64)];
_6 = [_17,_14,_14];
_10 = _19;
SetDiscriminant(_8, 1);
_18 = [_22.0,_3];
_16 = 15000_i16 + 10128_i16;
_5 = 35478_u16 as isize;
_22 = (_3, _2);
_26 = !(-27_i8);
_5 = _15 & _12;
_18 = [_22.0,_3];
_17 = _14 | _14;
place!(Field::<char>(Variant(_8, 1), 1)) = RET;
_5 = !_12;
_28 = [_16,_16];
RET = Field::<char>(Variant(_8, 1), 1);
_7 = _16 as f64;
_16 = (-21640_i16);
_20 = _24.0;
place!(Field::<[bool; 3]>(Variant(_8, 1), 2)) = [_17,_14,_17];
_25 = Field::<char>(Variant(_8, 1), 1);
_7 = _26 as f64;
_1 = [_5,_5,_22.1];
_4 = [_23,_23,_23,_23,_23];
match _16 {
340282366920938463463374607431768189816 => bb10,
_ => bb8
}
}
bb10 = {
place!(Field::<[bool; 3]>(Variant(_8, 1), 2)) = [_17,_17,_17];
_24.2 = _13 as u8;
_27 = 62675_u16 as u64;
_15 = _17 as isize;
_30.0 = 50478_u16 as u32;
_26 = _7 as i8;
_19 = [(-468467438679815708_i64),(-6380946650278331562_i64)];
_26 = _13 as i8;
_6 = [_14,_17,_17];
RET = _20;
_10 = [(-4892273863685063588_i64),(-1103186811598903158_i64)];
_30.3 = _16 as u128;
_30.3 = !13397333095821492299041726046553521793_u128;
_1 = [_15,_22.1,_15];
_30.3 = 88202539735439106549350606699264421757_u128 + 266322855382400802353591485632302239790_u128;
_23 = 75840629248583180947790813812821967628_i128 << _5;
_21 = core::ptr::addr_of_mut!(_1);
place!(Field::<usize>(Variant(_8, 1), 0)) = _22.0 as usize;
place!(Field::<char>(Variant(_8, 1), 1)) = _25;
SetDiscriminant(_8, 1);
RET = _25;
_10 = _19;
_7 = _30.3 as f64;
_23 = 112153554837823437148603436185474260119_i128;
_14 = !_17;
Goto(bb11)
}
bb11 = {
_23 = 50835590856327219907209403789033280920_i128 ^ 150894443678270421865115612127036975874_i128;
RET = _20;
_30.1 = _13 >> _24.2;
_10 = [(-7178765619886996225_i64),(-3424126320893370318_i64)];
_14 = !_17;
_22.1 = !_5;
_24.2 = _22.0;
_22.1 = !_5;
_29 = _16 >> _27;
_5 = _15 << _22.1;
_18 = [_24.2,_24.2];
_17 = _12 != _22.1;
_30.2 = _5 * _5;
place!(Field::<usize>(Variant(_8, 1), 0)) = !_9;
_22.0 = _3 | _24.2;
_5 = _12 >> _22.0;
_29 = _16;
Goto(bb12)
}
bb12 = {
_25 = RET;
_8 = Adt41::Variant1 { fld0: _9,fld1: _25,fld2: _6 };
_25 = _20;
_22.0 = _3 >> _5;
_33.fld4.fld3 = [_29];
_2 = !_30.2;
_12 = _30.2;
_1 = [_2,_2,_30.2];
_22.1 = _16 as isize;
_16 = _29;
_4 = [_23,_23,_23,_23,_23];
_28 = [_29,_29];
_33.fld4.fld2 = _12 >> _30.2;
_33.fld1 = [_26,_26,_26,_26,_26];
_35 = Adt41::Variant1 { fld0: Field::<usize>(Variant(_8, 1), 0),fld1: RET,fld2: Field::<[bool; 3]>(Variant(_8, 1), 2) };
_33.fld0 = _30.1 as u64;
_33.fld5 = _9 * _9;
_33.fld4.fld0 = core::ptr::addr_of_mut!(_30.3);
_34 = _33.fld5 as f64;
_35 = _8;
_9 = !Field::<usize>(Variant(_35, 1), 0);
_2 = _5;
_14 = _17;
_26 = (-4796426098980843717_i64) as i8;
_36.2 = !_3;
match _29 {
0 => bb7,
1 => bb13,
340282366920938463463374607431768189816 => bb15,
_ => bb14
}
}
bb13 = {
place!(Field::<usize>(Variant(_8, 1), 0)) = Field::<char>(Variant(_8, 1), 1) as usize;
_17 = false;
RET = Field::<char>(Variant(_8, 1), 1);
_14 = _17 == _17;
_5 = _2;
_10 = [5270057368530762627_i64,251161355767065919_i64];
_9 = Field::<usize>(Variant(_8, 1), 0) & Field::<usize>(Variant(_8, 1), 0);
RET = Field::<char>(Variant(_8, 1), 1);
_16 = 7902_i16 << _2;
_5 = _2;
SetDiscriminant(_8, 0);
_5 = _2 - _12;
_1 = [_5,_5,_12];
_13 = (-1066393053_i32) * 1927988533_i32;
_18 = [_3,_3];
_7 = 4257274492596556152_i64 as f64;
_8 = Adt41::Variant1 { fld0: _9,fld1: RET,fld2: _6 };
_15 = _2;
RET = Field::<char>(Variant(_8, 1), 1);
place!(Field::<usize>(Variant(_8, 1), 0)) = _9;
place!(Field::<usize>(Variant(_8, 1), 0)) = !_9;
_17 = _14;
_3 = _14 as u8;
Goto(bb7)
}
bb14 = {
_1 = [_2,_2,_5];
RET = '\u{c43e3}';
_6 = [false,false,false];
RET = '\u{73bcf}';
_8 = Adt41::Variant1 { fld0: 4_usize,fld1: RET,fld2: _6 };
_4 = [157622684693712860205684402707774970593_i128,164136700264354750731061278680736623687_i128,(-40946788682968688648990813936821063166_i128),246936440869709561642372060519057803_i128,(-151437679548407793719664307796575454781_i128)];
RET = Field::<char>(Variant(_8, 1), 1);
_7 = (-20876021559592458958151946602812345682_i128) as f64;
RET = Field::<char>(Variant(_8, 1), 1);
_5 = (-62500338013864591416037367272067724215_i128) as isize;
place!(Field::<[bool; 3]>(Variant(_8, 1), 2)) = [false,true,false];
place!(Field::<[bool; 3]>(Variant(_8, 1), 2)) = [true,false,false];
_10 = [5435951986035609500_i64,(-235201648593744926_i64)];
_6 = [true,true,true];
_1 = [_2,_5,_2];
_8 = Adt41::Variant1 { fld0: 5_usize,fld1: RET,fld2: _6 };
_9 = !6251915131395848011_usize;
RET = Field::<char>(Variant(_8, 1), 1);
_5 = _2;
Goto(bb3)
}
bb15 = {
_33.fld4.fld0 = core::ptr::addr_of_mut!(_37);
_30.1 = _13 - _13;
place!(Field::<char>(Variant(_8, 1), 1)) = RET;
_22 = (_36.2, _33.fld4.fld2);
_30 = (682944442_u32, _13, _33.fld4.fld2, 195085728250622881520944936729895708708_u128);
_21 = core::ptr::addr_of_mut!(_33.fld7);
_1 = [_15,_30.2,_30.2];
RET = _20;
_24.2 = _16 as u8;
RET = Field::<char>(Variant(_8, 1), 1);
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(14_usize, 6_usize, Move(_6), 26_usize, Move(_26), 16_usize, Move(_16), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(14_usize, 19_usize, Move(_19), 18_usize, Move(_18), 22_usize, Move(_22), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(14_usize, 13_usize, Move(_13), 29_usize, Move(_29), 9_usize, Move(_9), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [bool; 3],mut _2: [i64; 2],mut _3: isize,mut _4: isize,mut _5: [bool; 3],mut _6: [bool; 3],mut _7: [bool; 3]) -> [i128; 5] {
mir! {
type RET = [i128; 5];
let _8: f32;
let _9: isize;
let _10: (u32, i32, isize, u128);
let _11: [u128; 4];
let _12: [i128; 5];
let _13: u8;
let _14: Adt41;
let _15: Adt43;
let _16: bool;
let _17: isize;
let _18: [i16; 1];
let _19: Adt44;
let _20: Adt44;
let _21: ();
let _22: ();
{
RET = [(-2508903395394470233203617855636135605_i128),80957374934952333712006912665044319651_i128,87773547802633755762878091850531988618_i128,153791402074530459954481034970912158621_i128,140470672252555230804244382930901169639_i128];
RET = [82326919765745495786115816668082291771_i128,(-57780784997928780207813782700739699143_i128),(-129531307275306022001262627939506840936_i128),31869138484696862740710212519474984493_i128,168951511950749854890409407767749072641_i128];
_6 = [false,true,true];
_8 = (-87_i8) as f32;
_4 = _3;
_4 = _3 | _3;
_4 = _3 * _3;
_8 = 85_u8 as f32;
_1 = _7;
RET = [62083739597176125208299133069877561251_i128,35261217539174177549532014036971054926_i128,(-66105714481293256888611374339649456396_i128),20091095814921519799061215647711114864_i128,62798614612884851946688736861746851620_i128];
Goto(bb1)
}
bb1 = {
RET = [(-134844480027332839229293396737308296087_i128),(-8932057167628659032306272398870372755_i128),(-169421571832650648618793140891694097750_i128),(-11489039979387714089918777444739531344_i128),134049812493761407503302188216580814533_i128];
_4 = _3;
_5 = [true,true,false];
_1 = _6;
_5 = _6;
_2 = [(-8004921862446799623_i64),(-3126799762755463949_i64)];
_1 = _6;
Call(_4 = core::intrinsics::bswap(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = [false,true,true];
RET = [(-152071343511994305689576640862865903312_i128),(-159935433127964969500003512182805330384_i128),(-134777770513634803489893165155958913148_i128),149584699448033998457499747773705606560_i128,(-161716075117371589585185091971719136546_i128)];
_6 = _5;
_2 = [(-7933419308809398715_i64),(-2827450839707795661_i64)];
_9 = _3 ^ _4;
_2 = [(-3220405667264731620_i64),4801603069525494109_i64];
_5 = [true,true,false];
RET = [(-166107247084161770712045195231405222380_i128),(-70094730419899244135179503370524519288_i128),(-51813921182481167034238147168464582238_i128),(-110286084677882644821414429200502089454_i128),160974840662466838486579882822098019581_i128];
_7 = [false,false,true];
_7 = [true,true,true];
_5 = [true,false,true];
_7 = _1;
RET = [(-111395842604121684423754361118746858569_i128),107975534888944251855218857456320869534_i128,(-49473337247659358621595160769939343189_i128),42379317603858778521958925494062582031_i128,(-53714178373087192201347458948889067814_i128)];
RET = [(-49607133900728784198316882112816337605_i128),(-150011706955728320216125500647442162789_i128),30680283800113444125530877759168384605_i128,(-129450173925619669335905452950794849261_i128),137167512237113906917051490196029061592_i128];
_10 = (1809827083_u32, 30628103_i32, _3, 251868180552183645053441938624745470329_u128);
Call(_8 = fn16(_10.3, _9, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [(-3531513018993079206638146057912490975_i128),45260703952450316463131774992287907991_i128,(-69956678905252315734312313901858142213_i128),(-52773380758078122646935976673592248457_i128),(-31116246317050285307853949539869209072_i128)];
_10.3 = _10.1 as u128;
_5 = [true,false,false];
_3 = !_9;
_2 = [402072951529828228_i64,2662795336903088915_i64];
_10.0 = 2622521331_u32;
_4 = !_9;
_7 = [false,false,true];
_10.3 = !147922287581021921089850235993972588034_u128;
_7 = _5;
_10.1 = -(-1606148003_i32);
_9 = _3 ^ _3;
match _10.0 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
2622521331 => bb10,
_ => bb9
}
}
bb4 = {
_5 = [false,true,true];
RET = [(-152071343511994305689576640862865903312_i128),(-159935433127964969500003512182805330384_i128),(-134777770513634803489893165155958913148_i128),149584699448033998457499747773705606560_i128,(-161716075117371589585185091971719136546_i128)];
_6 = _5;
_2 = [(-7933419308809398715_i64),(-2827450839707795661_i64)];
_9 = _3 ^ _4;
_2 = [(-3220405667264731620_i64),4801603069525494109_i64];
_5 = [true,true,false];
RET = [(-166107247084161770712045195231405222380_i128),(-70094730419899244135179503370524519288_i128),(-51813921182481167034238147168464582238_i128),(-110286084677882644821414429200502089454_i128),160974840662466838486579882822098019581_i128];
_7 = [false,false,true];
_7 = [true,true,true];
_5 = [true,false,true];
_7 = _1;
RET = [(-111395842604121684423754361118746858569_i128),107975534888944251855218857456320869534_i128,(-49473337247659358621595160769939343189_i128),42379317603858778521958925494062582031_i128,(-53714178373087192201347458948889067814_i128)];
RET = [(-49607133900728784198316882112816337605_i128),(-150011706955728320216125500647442162789_i128),30680283800113444125530877759168384605_i128,(-129450173925619669335905452950794849261_i128),137167512237113906917051490196029061592_i128];
_10 = (1809827083_u32, 30628103_i32, _3, 251868180552183645053441938624745470329_u128);
Call(_8 = fn16(_10.3, _9, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = [(-134844480027332839229293396737308296087_i128),(-8932057167628659032306272398870372755_i128),(-169421571832650648618793140891694097750_i128),(-11489039979387714089918777444739531344_i128),134049812493761407503302188216580814533_i128];
_4 = _3;
_5 = [true,true,false];
_1 = _6;
_5 = _6;
_2 = [(-8004921862446799623_i64),(-3126799762755463949_i64)];
_1 = _6;
Call(_4 = core::intrinsics::bswap(_3), ReturnTo(bb2), UnwindUnreachable())
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
_4 = _3;
_2 = [(-7047935575707486104_i64),7531558747702215882_i64];
_12 = [58749172565182842314116324989309154514_i128,128253588747008965583015007611245396194_i128,(-133023430532147323302278386219291897973_i128),6580242156316617946637343154135170601_i128,103090102999807786957963978368491714461_i128];
_4 = 6541614849361641118_u64 as isize;
RET = [(-71440150616388022230698935728341696477_i128),124066169321491683867201460918099486934_i128,32434409275586609659481605933248335427_i128,86326121195309763646595620732107030717_i128,113884105666242036324687578073616708570_i128];
_2 = [7099012943336640235_i64,(-5980091197707060334_i64)];
_10.1 = 12251419347366095208_u64 as i32;
_1 = [true,false,true];
Goto(bb11)
}
bb11 = {
_5 = [true,false,false];
RET = _12;
_1 = _7;
RET = [(-151405546126639567128336540637690902922_i128),(-111914977459943468064169339121024704689_i128),(-50439482344440594603032865274898755290_i128),(-137015815210476755074307937698396737997_i128),(-44359849358532423657349041227447571475_i128)];
RET = [44884779382134293055268065684463953517_i128,22782268728528080137543696319668372737_i128,96329313334603677446148408930900428324_i128,(-126665413471060612161107049773856119885_i128),86764405338862950305236871296768756442_i128];
_7 = [true,true,true];
_1 = _6;
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
2622521331 => bb12,
_ => bb10
}
}
bb12 = {
_10.0 = 3055139135_u32;
match _10.0 {
0 => bb11,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb6,
5 => bb13,
6 => bb14,
3055139135 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_5 = [false,true,true];
RET = [(-152071343511994305689576640862865903312_i128),(-159935433127964969500003512182805330384_i128),(-134777770513634803489893165155958913148_i128),149584699448033998457499747773705606560_i128,(-161716075117371589585185091971719136546_i128)];
_6 = _5;
_2 = [(-7933419308809398715_i64),(-2827450839707795661_i64)];
_9 = _3 ^ _4;
_2 = [(-3220405667264731620_i64),4801603069525494109_i64];
_5 = [true,true,false];
RET = [(-166107247084161770712045195231405222380_i128),(-70094730419899244135179503370524519288_i128),(-51813921182481167034238147168464582238_i128),(-110286084677882644821414429200502089454_i128),160974840662466838486579882822098019581_i128];
_7 = [false,false,true];
_7 = [true,true,true];
_5 = [true,false,true];
_7 = _1;
RET = [(-111395842604121684423754361118746858569_i128),107975534888944251855218857456320869534_i128,(-49473337247659358621595160769939343189_i128),42379317603858778521958925494062582031_i128,(-53714178373087192201347458948889067814_i128)];
RET = [(-49607133900728784198316882112816337605_i128),(-150011706955728320216125500647442162789_i128),30680283800113444125530877759168384605_i128,(-129450173925619669335905452950794849261_i128),137167512237113906917051490196029061592_i128];
_10 = (1809827083_u32, 30628103_i32, _3, 251868180552183645053441938624745470329_u128);
Call(_8 = fn16(_10.3, _9, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_15.fld0 = [true,true,false];
_5 = [true,false,false];
_7 = [true,false,false];
_2 = [(-1445771716858047647_i64),1933845901738911363_i64];
RET = [(-69187810759148308051443965449413507647_i128),(-46830452611564575196871755395617925_i128),118834461710860571065823317423990484246_i128,(-25912429110386963802083643796679880232_i128),(-160128262462752834418187349886948475996_i128)];
_15.fld3.0 = '\u{553ff}';
_15.fld2 = _8 as u128;
RET = _12;
_6 = _5;
_2 = [6593530825911932989_i64,4084278678643511040_i64];
_10.3 = !_15.fld2;
_15.fld2 = !_10.3;
_15.fld3.2 = !193_u8;
_17 = _15.fld2 as isize;
_11 = [_15.fld2,_10.3,_15.fld2,_10.3];
_8 = (-53_i8) as f32;
_1 = [true,true,false];
_13 = _15.fld3.2;
_1 = [true,false,true];
_14 = Adt41::Variant1 { fld0: 18045443284967824409_usize,fld1: _15.fld3.0,fld2: _1 };
_13 = !_15.fld3.2;
_4 = 37588_u16 as isize;
_5 = _6;
place!(Field::<char>(Variant(_14, 1), 1)) = _15.fld3.0;
Goto(bb17)
}
bb17 = {
Call(_21 = dump_var(15_usize, 10_usize, Move(_10), 13_usize, Move(_13), 4_usize, Move(_4), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_21 = dump_var(15_usize, 7_usize, Move(_7), 6_usize, Move(_6), 22_usize, _22, 22_usize, _22), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u128,mut _2: isize,mut _3: [i128; 5]) -> f32 {
mir! {
type RET = f32;
let _4: isize;
let _5: Adt44;
let _6: [isize; 3];
let _7: Adt41;
let _8: i8;
let _9: isize;
let _10: [i16; 2];
let _11: [i8; 5];
let _12: isize;
let _13: Adt41;
let _14: Adt44;
let _15: i16;
let _16: f32;
let _17: *const *mut i64;
let _18: Adt48;
let _19: i128;
let _20: isize;
let _21: Adt41;
let _22: ();
let _23: ();
{
_3 = [92327708263672282725350563185603462980_i128,16473718473933577997227045346944933384_i128,146662727699731516736057067126509724615_i128,3162696097878220818092694826601679829_i128,(-142238392016603520796013875540957495557_i128)];
_3 = [58072652508499984366074506281160369897_i128,(-88751212715864238591247934959769630725_i128),(-66409387730036901804464589515712515485_i128),(-162773837196801883850528717278238805906_i128),(-111770890651721724856726504470807190446_i128)];
RET = _2 as f32;
_3 = [103868748359569185702126218213644664702_i128,(-9969987434605353226840497324066904985_i128),86232696324512474784501799020505840673_i128,(-37856333906118718659284765129991163958_i128),67365024807140108361878133551995993419_i128];
_1 = 254359616928831907333678776882567192335_u128 - 86067243138119730669050460716673978265_u128;
RET = 3718650241307927554_i64 as f32;
_3 = [29615465524090323166132113734530510286_i128,109744100014067332901497387993855389350_i128,63350127794527486441460980367529489291_i128,(-135626291193339945538657339104815708991_i128),155244084184944542625639322631256265902_i128];
_2 = !9223372036854775807_isize;
_1 = 28096_i16 as u128;
_2 = 89_isize;
RET = 6366262043310077746_usize as f32;
RET = 5054999580149273193_u64 as f32;
_1 = 243715926531537996654252728491277375224_u128;
RET = 768976825_i32 as f32;
Call(_1 = core::intrinsics::bswap(184373849658229102497576594740310012485_u128), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 5_usize as f32;
_3 = [(-8284497836141144999577554738350937921_i128),87211847027008551226697923741216536807_i128,(-41290053206240574145588396594838654502_i128),(-89889395999260276085799836051571594798_i128),29952726139564291328929065029345690925_i128];
_3 = [(-112743573799874617731764299613113869757_i128),152664777429424091818838804996593030537_i128,(-87070029271790891171674632239238444974_i128),(-102155603693058438040709837402111165805_i128),(-7222177429666844397104959803939887988_i128)];
_1 = 141465775505917015308572342294765693366_u128;
_1 = 112522841_u32 as u128;
_3 = [(-22183965843009607322253169395549027465_i128),137775004499647104170041246008418039187_i128,(-112376940131887708836931984740684098947_i128),69985430101480038185179638362330756219_i128,(-105160946532595742724150275470995277308_i128)];
RET = 1777972794_i32 as f32;
Call(RET = fn17(_3, _3, _3, _2, _2, _3, _3, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = 251754019590785298763908499736468740119_u128 + 263659399158772080145833649459848829195_u128;
RET = 3_usize as f32;
RET = 5_usize as f32;
_1 = 2480329960_u32 as u128;
RET = 106376273_i32 as f32;
_4 = _1 as isize;
RET = _1 as f32;
_3 = [(-62861934568993345191877312477203238914_i128),9064631863476567941587408791497761008_i128,5263984227847814153647975000396501149_i128,(-10525793984752411687469212697361588047_i128),(-6611691283782045693754258291603234395_i128)];
RET = _2 as f32;
_3 = [(-55479024272250827486828319182757586480_i128),(-79461415894410031846462333918553296774_i128),124735334609425157684102923973428026735_i128,(-140015392430428093357212425211521633703_i128),(-135240630715692290842371688643221145167_i128)];
RET = 255_u8 as f32;
_2 = 36649_u16 as isize;
_3 = [(-17540706142719644433304263477539273683_i128),(-123454576713013068661806470650171104133_i128),3432667106869651077482538900818770803_i128,(-8485406417158577663462397873344970129_i128),(-54249028700996771172316220639383493708_i128)];
_3 = [118400482565828579400472550682291220288_i128,21461202637552513353558112524384847215_i128,(-93522808556297998317133457218389922150_i128),(-108501001536180244017293684098025547615_i128),(-43308142768936178459828691411558567272_i128)];
_1 = 55434125795952266315005290348521886806_u128 | 296398960331622378304276469990580572982_u128;
_2 = _4 ^ _4;
_1 = 65174960218983782539933746410060994191_u128;
_6 = [_2,_2,_4];
_3 = [(-68798953900300495250931067161283772147_i128),165891242172557872810860342408636773874_i128,93812519364534188399163297705151464159_i128,72624324554850913833886478608512170598_i128,(-158155477555908154689047444718271922364_i128)];
Goto(bb3)
}
bb3 = {
RET = 2_usize as f32;
_3 = [(-164180283084017109999583310344327837745_i128),17707850148454827991532978028122593041_i128,(-76250425477686686446607548580669793921_i128),(-3020431794640486542991640280130167665_i128),(-62678333365188379841041598934085770425_i128)];
_8 = _1 as i8;
_2 = _4 * _4;
RET = 30100_u16 as f32;
_6 = [_4,_2,_4];
_3 = [(-140142613245935701833339356695317267794_i128),(-46185057069358924254676037537035210424_i128),169094754019198319043742385271388258530_i128,2106253289368388211082041956759482545_i128,(-36984200797969975845245448320068988003_i128)];
_3 = [32046482302309755481863288193725715438_i128,(-21228293395511189845119388015993010893_i128),110390921492161562091423694774608956534_i128,(-112125000741385313716926147165824994846_i128),32382694495443695535491584195028729150_i128];
_9 = _2 | _2;
_4 = _9 ^ _2;
_1 = 96184253191911647326094007406434782244_u128;
_4 = !_9;
_1 = RET as u128;
_2 = !_9;
_1 = 314689437053839147246889104393091043238_u128 ^ 49778692568528625766401312525762089200_u128;
RET = 15163468563737231991_usize as f32;
Goto(bb4)
}
bb4 = {
RET = 211_u8 as f32;
_8 = !115_i8;
_1 = 73918406182326821175374206891613629084_u128;
_8 = 1915085263_u32 as i8;
_3 = [(-27374328439391152483113797556469301503_i128),25503886771719742648955643998333161338_i128,(-25384693856511803479233637983667091893_i128),139726555637491444619346780009278068966_i128,7816933960468829198089387491580764950_i128];
_11 = [_8,_8,_8,_8,_8];
_8 = 204_u8 as i8;
_9 = _2 >> _4;
_1 = _9 as u128;
_12 = _2;
_12 = _9 & _4;
_10 = [16016_i16,(-4885_i16)];
RET = 2_usize as f32;
_3 = [(-33852262499355969294239095831959710524_i128),167094251977558089998521581799965277790_i128,(-38767244825551977381916217837018899341_i128),(-166936976646516324831388746401372231540_i128),127364955429236011789553658987602398437_i128];
RET = _1 as f32;
_4 = !_2;
_9 = !_12;
_8 = (-47_i8);
_10 = [(-4823_i16),(-201_i16)];
_11 = [_8,_8,_8,_8,_8];
_9 = !_12;
_2 = _9 & _12;
match _8 {
0 => bb3,
340282366920938463463374607431768211409 => bb6,
_ => bb5
}
}
bb5 = {
RET = 2_usize as f32;
_3 = [(-164180283084017109999583310344327837745_i128),17707850148454827991532978028122593041_i128,(-76250425477686686446607548580669793921_i128),(-3020431794640486542991640280130167665_i128),(-62678333365188379841041598934085770425_i128)];
_8 = _1 as i8;
_2 = _4 * _4;
RET = 30100_u16 as f32;
_6 = [_4,_2,_4];
_3 = [(-140142613245935701833339356695317267794_i128),(-46185057069358924254676037537035210424_i128),169094754019198319043742385271388258530_i128,2106253289368388211082041956759482545_i128,(-36984200797969975845245448320068988003_i128)];
_3 = [32046482302309755481863288193725715438_i128,(-21228293395511189845119388015993010893_i128),110390921492161562091423694774608956534_i128,(-112125000741385313716926147165824994846_i128),32382694495443695535491584195028729150_i128];
_9 = _2 | _2;
_4 = _9 ^ _2;
_1 = 96184253191911647326094007406434782244_u128;
_4 = !_9;
_1 = RET as u128;
_2 = !_9;
_1 = 314689437053839147246889104393091043238_u128 ^ 49778692568528625766401312525762089200_u128;
RET = 15163468563737231991_usize as f32;
Goto(bb4)
}
bb6 = {
_16 = RET;
RET = -_16;
_2 = _4 ^ _4;
_10 = [(-5268_i16),(-1063_i16)];
_6 = [_9,_9,_2];
_10 = [(-24971_i16),10707_i16];
_3 = [(-93261858697275107631576385777441694096_i128),(-74394180339729883736373439569353924217_i128),(-68998015796348780391059199340269106144_i128),(-62129773776740446540806097825563877395_i128),90814769870960814386172456866264621142_i128];
_15 = (-11556_i16) ^ (-16155_i16);
_4 = !_2;
_3 = [111904679974217190825930237072067800354_i128,(-18491150164239342499084188679825881628_i128),52921560997684378666230688915219304104_i128,124176139973688858424243304713178679683_i128,11855337745504062547675584411775424782_i128];
_1 = 108294818245166054524149393250045518278_u128;
_9 = 194_u8 as isize;
_9 = false as isize;
_8 = 102_i8;
_12 = (-95866852438514698569245430015681406795_i128) as isize;
_11 = [_8,_8,_8,_8,_8];
_6 = [_2,_4,_2];
match _1 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
108294818245166054524149393250045518278 => bb15,
_ => bb14
}
}
bb7 = {
RET = 2_usize as f32;
_3 = [(-164180283084017109999583310344327837745_i128),17707850148454827991532978028122593041_i128,(-76250425477686686446607548580669793921_i128),(-3020431794640486542991640280130167665_i128),(-62678333365188379841041598934085770425_i128)];
_8 = _1 as i8;
_2 = _4 * _4;
RET = 30100_u16 as f32;
_6 = [_4,_2,_4];
_3 = [(-140142613245935701833339356695317267794_i128),(-46185057069358924254676037537035210424_i128),169094754019198319043742385271388258530_i128,2106253289368388211082041956759482545_i128,(-36984200797969975845245448320068988003_i128)];
_3 = [32046482302309755481863288193725715438_i128,(-21228293395511189845119388015993010893_i128),110390921492161562091423694774608956534_i128,(-112125000741385313716926147165824994846_i128),32382694495443695535491584195028729150_i128];
_9 = _2 | _2;
_4 = _9 ^ _2;
_1 = 96184253191911647326094007406434782244_u128;
_4 = !_9;
_1 = RET as u128;
_2 = !_9;
_1 = 314689437053839147246889104393091043238_u128 ^ 49778692568528625766401312525762089200_u128;
RET = 15163468563737231991_usize as f32;
Goto(bb4)
}
bb8 = {
RET = 211_u8 as f32;
_8 = !115_i8;
_1 = 73918406182326821175374206891613629084_u128;
_8 = 1915085263_u32 as i8;
_3 = [(-27374328439391152483113797556469301503_i128),25503886771719742648955643998333161338_i128,(-25384693856511803479233637983667091893_i128),139726555637491444619346780009278068966_i128,7816933960468829198089387491580764950_i128];
_11 = [_8,_8,_8,_8,_8];
_8 = 204_u8 as i8;
_9 = _2 >> _4;
_1 = _9 as u128;
_12 = _2;
_12 = _9 & _4;
_10 = [16016_i16,(-4885_i16)];
RET = 2_usize as f32;
_3 = [(-33852262499355969294239095831959710524_i128),167094251977558089998521581799965277790_i128,(-38767244825551977381916217837018899341_i128),(-166936976646516324831388746401372231540_i128),127364955429236011789553658987602398437_i128];
RET = _1 as f32;
_4 = !_2;
_9 = !_12;
_8 = (-47_i8);
_10 = [(-4823_i16),(-201_i16)];
_11 = [_8,_8,_8,_8,_8];
_9 = !_12;
_2 = _9 & _12;
match _8 {
0 => bb3,
340282366920938463463374607431768211409 => bb6,
_ => bb5
}
}
bb9 = {
RET = 2_usize as f32;
_3 = [(-164180283084017109999583310344327837745_i128),17707850148454827991532978028122593041_i128,(-76250425477686686446607548580669793921_i128),(-3020431794640486542991640280130167665_i128),(-62678333365188379841041598934085770425_i128)];
_8 = _1 as i8;
_2 = _4 * _4;
RET = 30100_u16 as f32;
_6 = [_4,_2,_4];
_3 = [(-140142613245935701833339356695317267794_i128),(-46185057069358924254676037537035210424_i128),169094754019198319043742385271388258530_i128,2106253289368388211082041956759482545_i128,(-36984200797969975845245448320068988003_i128)];
_3 = [32046482302309755481863288193725715438_i128,(-21228293395511189845119388015993010893_i128),110390921492161562091423694774608956534_i128,(-112125000741385313716926147165824994846_i128),32382694495443695535491584195028729150_i128];
_9 = _2 | _2;
_4 = _9 ^ _2;
_1 = 96184253191911647326094007406434782244_u128;
_4 = !_9;
_1 = RET as u128;
_2 = !_9;
_1 = 314689437053839147246889104393091043238_u128 ^ 49778692568528625766401312525762089200_u128;
RET = 15163468563737231991_usize as f32;
Goto(bb4)
}
bb10 = {
_1 = 251754019590785298763908499736468740119_u128 + 263659399158772080145833649459848829195_u128;
RET = 3_usize as f32;
RET = 5_usize as f32;
_1 = 2480329960_u32 as u128;
RET = 106376273_i32 as f32;
_4 = _1 as isize;
RET = _1 as f32;
_3 = [(-62861934568993345191877312477203238914_i128),9064631863476567941587408791497761008_i128,5263984227847814153647975000396501149_i128,(-10525793984752411687469212697361588047_i128),(-6611691283782045693754258291603234395_i128)];
RET = _2 as f32;
_3 = [(-55479024272250827486828319182757586480_i128),(-79461415894410031846462333918553296774_i128),124735334609425157684102923973428026735_i128,(-140015392430428093357212425211521633703_i128),(-135240630715692290842371688643221145167_i128)];
RET = 255_u8 as f32;
_2 = 36649_u16 as isize;
_3 = [(-17540706142719644433304263477539273683_i128),(-123454576713013068661806470650171104133_i128),3432667106869651077482538900818770803_i128,(-8485406417158577663462397873344970129_i128),(-54249028700996771172316220639383493708_i128)];
_3 = [118400482565828579400472550682291220288_i128,21461202637552513353558112524384847215_i128,(-93522808556297998317133457218389922150_i128),(-108501001536180244017293684098025547615_i128),(-43308142768936178459828691411558567272_i128)];
_1 = 55434125795952266315005290348521886806_u128 | 296398960331622378304276469990580572982_u128;
_2 = _4 ^ _4;
_1 = 65174960218983782539933746410060994191_u128;
_6 = [_2,_2,_4];
_3 = [(-68798953900300495250931067161283772147_i128),165891242172557872810860342408636773874_i128,93812519364534188399163297705151464159_i128,72624324554850913833886478608512170598_i128,(-158155477555908154689047444718271922364_i128)];
Goto(bb3)
}
bb11 = {
RET = 5_usize as f32;
_3 = [(-8284497836141144999577554738350937921_i128),87211847027008551226697923741216536807_i128,(-41290053206240574145588396594838654502_i128),(-89889395999260276085799836051571594798_i128),29952726139564291328929065029345690925_i128];
_3 = [(-112743573799874617731764299613113869757_i128),152664777429424091818838804996593030537_i128,(-87070029271790891171674632239238444974_i128),(-102155603693058438040709837402111165805_i128),(-7222177429666844397104959803939887988_i128)];
_1 = 141465775505917015308572342294765693366_u128;
_1 = 112522841_u32 as u128;
_3 = [(-22183965843009607322253169395549027465_i128),137775004499647104170041246008418039187_i128,(-112376940131887708836931984740684098947_i128),69985430101480038185179638362330756219_i128,(-105160946532595742724150275470995277308_i128)];
RET = 1777972794_i32 as f32;
Call(RET = fn17(_3, _3, _3, _2, _2, _3, _3, _2), ReturnTo(bb2), UnwindUnreachable())
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
RET = 11615291099492823232_usize as f32;
_19 = '\u{fa1f8}' as i128;
_4 = (-812410961_i32) as isize;
_20 = _12 & _2;
_10 = [_15,_15];
_16 = RET;
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(16_usize, 11_usize, Move(_11), 19_usize, Move(_19), 3_usize, Move(_3), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_22 = dump_var(16_usize, 15_usize, Move(_15), 10_usize, Move(_10), 23_usize, _23, 23_usize, _23), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: [i128; 5],mut _2: [i128; 5],mut _3: [i128; 5],mut _4: isize,mut _5: isize,mut _6: [i128; 5],mut _7: [i128; 5],mut _8: isize) -> f32 {
mir! {
type RET = f32;
let _9: isize;
let _10: isize;
let _11: [u128; 4];
let _12: Adt53;
let _13: [u128; 4];
let _14: [i128; 5];
let _15: (u8, isize);
let _16: i16;
let _17: Adt53;
let _18: *mut *const u8;
let _19: usize;
let _20: Adt44;
let _21: [isize; 3];
let _22: [i16; 1];
let _23: ([i16; 1], i8, *mut u128);
let _24: [u128; 4];
let _25: [i8; 5];
let _26: [u128; 4];
let _27: usize;
let _28: Adt51;
let _29: [i64; 2];
let _30: [bool; 3];
let _31: u64;
let _32: ();
let _33: ();
{
_1 = [21539799502581209329101636065313205684_i128,(-137957540697315498021504602688678891563_i128),(-169730223999454758634977669515060384818_i128),29894535375350935978579891275522020349_i128,18538054746765662924762506332834753415_i128];
RET = 79_u8 as f32;
_2 = [138406286753933232319604083857459202698_i128,(-170096976636635667089810112503981954558_i128),(-79771834718749367582902285190083064312_i128),23077860679956350203562049608194884026_i128,(-70570822751778483960140990383844221034_i128)];
_5 = _8;
_6 = [11693495413740706612893954466308986429_i128,87930839161366163563229630057995528212_i128,(-69324100296084641097867778926780328822_i128),164699318458536545513008078892266537269_i128,75079787685570508274711497357274287554_i128];
_2 = [(-43098521971112726212333833379847057729_i128),(-166835110084812799122058583996264145430_i128),10096876177968354771838972885397854913_i128,56348976770836566817926066107532729103_i128,(-132774031461533127267417236851929414363_i128)];
_10 = !_4;
_6 = [(-80089038728735590893567912555044338335_i128),13614777372111503137716361296226499272_i128,43963087737803485200438446466901600107_i128,(-8646667276631956650341325791124447085_i128),9855675036880305944204350358829191145_i128];
_3 = [131024928329538417782426269830811578042_i128,41303467487387806989620090445127331849_i128,(-55909980292592167403386201010032528216_i128),152758631014897858421022179777211294120_i128,51812892614594968354336227063616908568_i128];
_3 = [33759226468208171739985572987971817224_i128,(-52864948029624790100440180010896268769_i128),78099779369709561997077949927389872391_i128,(-134394980981453315105573365479670953425_i128),(-130210263175542694251719506684784993409_i128)];
_8 = -_10;
match _4 {
89 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_3 = [75672748659586418475008384634721319665_i128,31709535792584841949161524008699991164_i128,(-45269782432350980598404610827560454393_i128),93930739312416181789726766885234390472_i128,(-50221782830517124211319793003852940026_i128)];
_9 = 5_i8 as isize;
_1 = [(-14414058187946659868768863572632642898_i128),20041517627227938372941111130788067444_i128,(-44188428503418289254183872093394627873_i128),32427723885652496054302507727627755473_i128,35684607503473384831489571592638945882_i128];
_11 = [23648234676038664600719686331423985019_u128,223732560060006904243104268558393535374_u128,207497768051755119897209681366383574993_u128,60865219987540190768735522253345429778_u128];
Goto(bb3)
}
bb3 = {
RET = (-1876825311_i32) as f32;
RET = (-56599290397501736679532320288928032409_i128) as f32;
_2 = [123775994987486307992730381008931373856_i128,(-60053836639058822222573759837103781187_i128),(-140036574193921687778333181160799711535_i128),(-162667114647821337723227404633053793842_i128),(-163455744285587235828964943185317974504_i128)];
_14 = [115009018619636882193408507695629219891_i128,(-28832754538486171399667616392332095069_i128),124765905446991338452076256123515280650_i128,(-19861830986896690709194009699681954393_i128),(-168143196793632083409285795235859864721_i128)];
_1 = [(-58644196060476352899812589064762645724_i128),69196156656935375134353079327611718879_i128,(-166403111839727693558219570807980421555_i128),80236638285893115283270316611456821374_i128,28452739720316521239199144992439296223_i128];
_1 = [37502307166511225083502413988420918122_i128,150680747842304705791304046086937400359_i128,3064466247879078779359604636890779325_i128,(-8929792619019769737136187888231850121_i128),(-25127847656092177000519916731934273651_i128)];
RET = 35_u8 as f32;
_5 = !_4;
_14 = _1;
RET = _10 as f32;
_11 = [301433995944075870775463120905379333548_u128,236561305382793373555912971535272671120_u128,223918376157961695215651003904700539594_u128,304260989036173545220628223917743703625_u128];
_8 = _10 * _9;
_4 = -_9;
RET = 16676950296958283274169823460674144839_i128 as f32;
Goto(bb4)
}
bb4 = {
_1 = _3;
_10 = -_5;
_4 = _10 * _5;
_15 = (125_u8, _8);
RET = (-5592496451114626373_i64) as f32;
_10 = _5 * _8;
_5 = _10 >> _10;
_14 = [50166266819418807932533220719127444603_i128,17577601460135182291848774909715513654_i128,(-82944117594857300424285877356418843453_i128),148114756468400979011218444632177259722_i128,(-88027112206544583477174175134438723287_i128)];
_4 = (-158077788586122788477455023682872044083_i128) as isize;
match _15.0 {
0 => bb1,
1 => bb2,
125 => bb5,
_ => bb3
}
}
bb5 = {
_15.0 = !121_u8;
_15 = (82_u8, _5);
_3 = [(-24790815970810645104906357441500295052_i128),74200026878090932628645950922317501639_i128,66764084433852808928859254835017881457_i128,119355190437946942064126560155416143431_i128,141272747564786232312476925686300081656_i128];
match _15.0 {
0 => bb1,
1 => bb6,
2 => bb7,
82 => bb9,
_ => bb8
}
}
bb6 = {
_1 = _3;
_10 = -_5;
_4 = _10 * _5;
_15 = (125_u8, _8);
RET = (-5592496451114626373_i64) as f32;
_10 = _5 * _8;
_5 = _10 >> _10;
_14 = [50166266819418807932533220719127444603_i128,17577601460135182291848774909715513654_i128,(-82944117594857300424285877356418843453_i128),148114756468400979011218444632177259722_i128,(-88027112206544583477174175134438723287_i128)];
_4 = (-158077788586122788477455023682872044083_i128) as isize;
match _15.0 {
0 => bb1,
1 => bb2,
125 => bb5,
_ => bb3
}
}
bb7 = {
RET = (-1876825311_i32) as f32;
RET = (-56599290397501736679532320288928032409_i128) as f32;
_2 = [123775994987486307992730381008931373856_i128,(-60053836639058822222573759837103781187_i128),(-140036574193921687778333181160799711535_i128),(-162667114647821337723227404633053793842_i128),(-163455744285587235828964943185317974504_i128)];
_14 = [115009018619636882193408507695629219891_i128,(-28832754538486171399667616392332095069_i128),124765905446991338452076256123515280650_i128,(-19861830986896690709194009699681954393_i128),(-168143196793632083409285795235859864721_i128)];
_1 = [(-58644196060476352899812589064762645724_i128),69196156656935375134353079327611718879_i128,(-166403111839727693558219570807980421555_i128),80236638285893115283270316611456821374_i128,28452739720316521239199144992439296223_i128];
_1 = [37502307166511225083502413988420918122_i128,150680747842304705791304046086937400359_i128,3064466247879078779359604636890779325_i128,(-8929792619019769737136187888231850121_i128),(-25127847656092177000519916731934273651_i128)];
RET = 35_u8 as f32;
_5 = !_4;
_14 = _1;
RET = _10 as f32;
_11 = [301433995944075870775463120905379333548_u128,236561305382793373555912971535272671120_u128,223918376157961695215651003904700539594_u128,304260989036173545220628223917743703625_u128];
_8 = _10 * _9;
_4 = -_9;
RET = 16676950296958283274169823460674144839_i128 as f32;
Goto(bb4)
}
bb8 = {
_3 = [75672748659586418475008384634721319665_i128,31709535792584841949161524008699991164_i128,(-45269782432350980598404610827560454393_i128),93930739312416181789726766885234390472_i128,(-50221782830517124211319793003852940026_i128)];
_9 = 5_i8 as isize;
_1 = [(-14414058187946659868768863572632642898_i128),20041517627227938372941111130788067444_i128,(-44188428503418289254183872093394627873_i128),32427723885652496054302507727627755473_i128,35684607503473384831489571592638945882_i128];
_11 = [23648234676038664600719686331423985019_u128,223732560060006904243104268558393535374_u128,207497768051755119897209681366383574993_u128,60865219987540190768735522253345429778_u128];
Goto(bb3)
}
bb9 = {
_16 = !1832_i16;
_14 = _3;
_15.0 = 29_u8;
_16 = (-26632_i16) ^ 4718_i16;
_7 = [161677794684843108017165746885755627006_i128,13994599150845547327566441610302951558_i128,(-86995168404821535424751240693278303282_i128),96272256516225334624331016022054837324_i128,(-90917552476003711180822864045115070070_i128)];
_7 = _1;
_9 = -_15.1;
match _15.0 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb10,
29 => bb12,
_ => bb11
}
}
bb10 = {
_3 = [75672748659586418475008384634721319665_i128,31709535792584841949161524008699991164_i128,(-45269782432350980598404610827560454393_i128),93930739312416181789726766885234390472_i128,(-50221782830517124211319793003852940026_i128)];
_9 = 5_i8 as isize;
_1 = [(-14414058187946659868768863572632642898_i128),20041517627227938372941111130788067444_i128,(-44188428503418289254183872093394627873_i128),32427723885652496054302507727627755473_i128,35684607503473384831489571592638945882_i128];
_11 = [23648234676038664600719686331423985019_u128,223732560060006904243104268558393535374_u128,207497768051755119897209681366383574993_u128,60865219987540190768735522253345429778_u128];
Goto(bb3)
}
bb11 = {
_1 = _3;
_10 = -_5;
_4 = _10 * _5;
_15 = (125_u8, _8);
RET = (-5592496451114626373_i64) as f32;
_10 = _5 * _8;
_5 = _10 >> _10;
_14 = [50166266819418807932533220719127444603_i128,17577601460135182291848774909715513654_i128,(-82944117594857300424285877356418843453_i128),148114756468400979011218444632177259722_i128,(-88027112206544583477174175134438723287_i128)];
_4 = (-158077788586122788477455023682872044083_i128) as isize;
match _15.0 {
0 => bb1,
1 => bb2,
125 => bb5,
_ => bb3
}
}
bb12 = {
RET = 6659089449682206899_u64 as f32;
_6 = [146901795744767133615364589810195193313_i128,169372991715968614617509878280004394177_i128,64397537974373819119392020982422906206_i128,(-145371238337940695230972017800393975993_i128),163123495964301846982614007054628189940_i128];
_15.1 = _5 | _5;
_13 = [105698748429214438408243841070843907552_u128,2730039116013409747988661002747242938_u128,214863008004677350264705852904629484210_u128,328965310897765904618117346127553251931_u128];
_13 = _11;
_4 = (-1122021564_i32) as isize;
_15.1 = -_5;
_19 = !0_usize;
_15.0 = 98_u8;
_8 = !_15.1;
_19 = (-1576560240_i32) as usize;
_1 = [(-152403893839572130655727546130095559396_i128),(-38323559633141189012310625624113321335_i128),142993046300112199098070733200090568295_i128,(-97953048966916938743521857220129154764_i128),18496547908712263057422750278958511466_i128];
_6 = _3;
RET = 49832318910975458865918669493940427716_u128 as f32;
RET = 0_i8 as f32;
_15 = (81_u8, _5);
_5 = RET as isize;
_1 = [(-96234998024074440181414823495414736207_i128),7246846048854562749954001737079457708_i128,(-26139726187058516931587429306368118061_i128),(-96666627604113782638955199875942736100_i128),35917294638594708301398565911973269212_i128];
_11 = _13;
Goto(bb13)
}
bb13 = {
_14 = [103574476117499026199721657715877215144_i128,(-149731938665182811396006665229565288170_i128),38561870474505286997986355344296763034_i128,(-102873540193597108501833623563795154096_i128),(-101834087096485298790091193179434697668_i128)];
_21 = [_15.1,_8,_10];
_11 = [202583165558966717563463851320746623007_u128,232276408320560214808402265658871071275_u128,200783909596248295361245899935708590868_u128,87662617188823998453868268440603298007_u128];
_4 = -_9;
RET = 47251826673473360203547481736257486238_u128 as f32;
_15.0 = 64_u8 * 104_u8;
_11 = [105292970862772771410774077700463872478_u128,16148025051011847951240673088141959819_u128,137732333432336240436386183380774486217_u128,119000932107106801119362294886405983123_u128];
_8 = !_10;
_9 = -_15.1;
_1 = _2;
RET = 37418_u16 as f32;
_26 = [127806416813200991887467122623205696869_u128,228774011279490186295232754691737708054_u128,60122702098835157226900404133491012010_u128,837081969482507934551559477791759055_u128];
_23.1 = !(-123_i8);
_7 = _3;
_15.0 = 79_u8 << _10;
_10 = _15.1;
_15.1 = !_10;
Goto(bb14)
}
bb14 = {
_2 = _14;
_24 = _13;
_9 = _15.1 | _10;
_19 = 8758802407001953068_usize ^ 3726557155630540949_usize;
_7 = [(-45886507439237099457267957681207703589_i128),(-34822021209363888085173608609373673605_i128),100591228146452335236734701682068397202_i128,141538179522707086467457698647197637472_i128,(-112906354725202756015920641153031778526_i128)];
_27 = 151501649944019905031299098101140348222_i128 as usize;
_15.0 = !142_u8;
_7 = [162047617359270047499083834027508840858_i128,(-90563234997757912851865925001834012328_i128),(-85354069948594146762430377049563772511_i128),(-14924807887527613816753340048771655820_i128),(-163895377299655846322996397707055903061_i128)];
_1 = _14;
_29 = [5236669573631012945_i64,(-4563607595442812547_i64)];
_22 = [_16];
_30 = [false,false,true];
_23.0 = _22;
_24 = [35555125391747138712351624031668342608_u128,224134953669794684386642722820638245829_u128,124527597087557759371982862874598676802_u128,194078164982819332324783912463066709129_u128];
_3 = [98915615286976891353436726970491188258_i128,(-140183041932252907288559534801116087584_i128),(-26136036503895321079822770719508658781_i128),138965626150574454020207451844798548333_i128,109439900901338147986954517448550083309_i128];
_14 = [56449931210698066270301544281926961205_i128,(-11562373250310520965446483561480042478_i128),153041879223991277733517791255844961257_i128,(-145276865271445577282468390460092939843_i128),8703314410992576956922037500864067726_i128];
_29 = [4382872335632271585_i64,(-6065924975994089254_i64)];
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(17_usize, 24_usize, Move(_24), 9_usize, Move(_9), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(17_usize, 1_usize, Move(_1), 15_usize, Move(_15), 22_usize, Move(_22), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(17_usize, 16_usize, Move(_16), 13_usize, Move(_13), 29_usize, Move(_29), 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [isize; 3],mut _2: [isize; 3],mut _3: [isize; 3]) -> f64 {
mir! {
type RET = f64;
let _4: isize;
let _5: f32;
let _6: usize;
let _7: [i64; 2];
let _8: ([i16; 1], i8, *mut u128);
let _9: Adt45;
let _10: [bool; 3];
let _11: bool;
let _12: f32;
let _13: f64;
let _14: isize;
let _15: bool;
let _16: Adt52;
let _17: [i16; 1];
let _18: *mut *const u8;
let _19: [i16; 2];
let _20: f64;
let _21: u8;
let _22: Adt45;
let _23: char;
let _24: ();
let _25: ();
{
RET = 23_u8 as f64;
_2 = [9223372036854775807_isize,(-13_isize),(-9223372036854775808_isize)];
RET = 232377322939392116039302736008074063061_u128 as f64;
Goto(bb1)
}
bb1 = {
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-35_isize)];
RET = 58406_u16 as f64;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),61_isize];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_4 = 104_isize & (-9223372036854775808_isize);
_4 = 9223372036854775807_isize;
_4 = 18988685431008037506114339044775911394_i128 as isize;
Goto(bb2)
}
bb2 = {
_5 = 24488_u16 as f32;
RET = (-67_i8) as f64;
_6 = 3_usize;
_5 = 127660656296243042014522700666508524440_u128 as f32;
_2 = [_4,_4,_4];
_5 = (-168847135427035560099936942887653911984_i128) as f32;
_6 = 12864747500887847967_usize ^ 14209119340669142224_usize;
_6 = !1748355527034508252_usize;
Goto(bb3)
}
bb3 = {
_3 = [_4,_4,_4];
RET = 179329580758762547274520204806715772620_u128 as f64;
RET = 823475070_u32 as f64;
_5 = 58679_u16 as f32;
_4 = 9223372036854775807_isize;
_5 = 10950415316272329057_u64 as f32;
_2 = [_4,_4,_4];
RET = 110956254073799204634384524858028738727_i128 as f64;
RET = _4 as f64;
_6 = 8055862046778180220_usize;
_5 = 26633_i16 as f32;
_3 = _1;
RET = 3888683809_u32 as f64;
_5 = _4 as f32;
_7 = [(-3977217600385913667_i64),(-7241192608020361813_i64)];
RET = 14068_i16 as f64;
_4 = 32_isize;
RET = _6 as f64;
_7 = [(-5424334629857466656_i64),3478187051129649420_i64];
_8.1 = 4945255015070309739_u64 as i8;
_2 = _1;
Goto(bb4)
}
bb4 = {
_2 = _1;
_4 = 9223372036854775807_isize;
_4 = 9223372036854775807_isize - 9223372036854775807_isize;
_8.0 = [4883_i16];
_4 = 9223372036854775807_isize;
RET = 33210_u16 as f64;
_11 = !true;
_5 = 10024289239636728580_u64 as f32;
_4 = 73_isize;
_14 = _4;
RET = _8.1 as f64;
_13 = (-4091094744258637022_i64) as f64;
_13 = (-29241923756076628200332566287499362564_i128) as f64;
_7 = [(-8193703253769332831_i64),1931907986086987163_i64];
RET = _13 - _13;
_8.0 = [31051_i16];
_8.0 = [(-6051_i16)];
_4 = _14 + _14;
Goto(bb5)
}
bb5 = {
_12 = _5 * _5;
_10 = [_11,_11,_11];
_15 = _6 > _6;
_6 = 17222750673500428047_usize;
RET = _13 * _13;
RET = _13;
_10 = [_15,_11,_15];
_8.0 = [14379_i16];
_3 = [_4,_4,_4];
_7 = [(-1412991348379672503_i64),(-508264416544872847_i64)];
_12 = (-26974_i16) as f32;
_8.2 = core::ptr::addr_of_mut!(_16.fld3);
_16.fld3 = _13 as u128;
_16.fld0 = 17895168071070685835_u64 - 2065823746682198665_u64;
_10 = [_11,_15,_15];
_6 = 19229_i16 as usize;
_16.fld4.fld5 = _4 as u64;
_10 = [_15,_11,_11];
_2 = [_4,_4,_14];
_17 = _8.0;
_3 = [_14,_4,_14];
_16.fld4.fld0 = core::ptr::addr_of_mut!(_16.fld3);
match _14 {
0 => bb3,
1 => bb6,
73 => bb8,
_ => bb7
}
}
bb6 = {
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-35_isize)];
RET = 58406_u16 as f64;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),61_isize];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_4 = 104_isize & (-9223372036854775808_isize);
_4 = 9223372036854775807_isize;
_4 = 18988685431008037506114339044775911394_i128 as isize;
Goto(bb2)
}
bb7 = {
_5 = 24488_u16 as f32;
RET = (-67_i8) as f64;
_6 = 3_usize;
_5 = 127660656296243042014522700666508524440_u128 as f32;
_2 = [_4,_4,_4];
_5 = (-168847135427035560099936942887653911984_i128) as f32;
_6 = 12864747500887847967_usize ^ 14209119340669142224_usize;
_6 = !1748355527034508252_usize;
Goto(bb3)
}
bb8 = {
_19 = [19431_i16,17861_i16];
Goto(bb9)
}
bb9 = {
_16.fld4 = Adt40 { fld0: _8.2,fld1: '\u{6bc7e}',fld2: _14,fld3: _17,fld4: (-8472041144369069183_i64),fld5: _16.fld0 };
_16.fld1 = [_8.1,_8.1,_8.1,_8.1,_8.1];
_16.fld5 = 65490_u16 as usize;
_23 = _16.fld4.fld1;
_23 = _16.fld4.fld1;
match _16.fld4.fld2 {
0 => bb10,
1 => bb11,
73 => bb13,
_ => bb12
}
}
bb10 = {
_3 = [_4,_4,_4];
RET = 179329580758762547274520204806715772620_u128 as f64;
RET = 823475070_u32 as f64;
_5 = 58679_u16 as f32;
_4 = 9223372036854775807_isize;
_5 = 10950415316272329057_u64 as f32;
_2 = [_4,_4,_4];
RET = 110956254073799204634384524858028738727_i128 as f64;
RET = _4 as f64;
_6 = 8055862046778180220_usize;
_5 = 26633_i16 as f32;
_3 = _1;
RET = 3888683809_u32 as f64;
_5 = _4 as f32;
_7 = [(-3977217600385913667_i64),(-7241192608020361813_i64)];
RET = 14068_i16 as f64;
_4 = 32_isize;
RET = _6 as f64;
_7 = [(-5424334629857466656_i64),3478187051129649420_i64];
_8.1 = 4945255015070309739_u64 as i8;
_2 = _1;
Goto(bb4)
}
bb11 = {
_2 = _1;
_4 = 9223372036854775807_isize;
_4 = 9223372036854775807_isize - 9223372036854775807_isize;
_8.0 = [4883_i16];
_4 = 9223372036854775807_isize;
RET = 33210_u16 as f64;
_11 = !true;
_5 = 10024289239636728580_u64 as f32;
_4 = 73_isize;
_14 = _4;
RET = _8.1 as f64;
_13 = (-4091094744258637022_i64) as f64;
_13 = (-29241923756076628200332566287499362564_i128) as f64;
_7 = [(-8193703253769332831_i64),1931907986086987163_i64];
RET = _13 - _13;
_8.0 = [31051_i16];
_8.0 = [(-6051_i16)];
_4 = _14 + _14;
Goto(bb5)
}
bb12 = {
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-35_isize)];
RET = 58406_u16 as f64;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),61_isize];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_4 = 104_isize & (-9223372036854775808_isize);
_4 = 9223372036854775807_isize;
_4 = 18988685431008037506114339044775911394_i128 as isize;
Goto(bb2)
}
bb13 = {
_21 = 152_u8 + 156_u8;
_16.fld0 = !_16.fld4.fld5;
_16.fld4 = Adt40 { fld0: _8.2,fld1: _23,fld2: _4,fld3: _8.0,fld4: 3344131957641624868_i64,fld5: _16.fld0 };
_16.fld3 = !269141222565055872291268972918895353575_u128;
_8.2 = _16.fld4.fld0;
_16.fld4.fld3 = _8.0;
_8.0 = _17;
_15 = _11;
match _16.fld4.fld4 {
0 => bb1,
1 => bb8,
2 => bb7,
3 => bb10,
3344131957641624868 => bb15,
_ => bb14
}
}
bb14 = {
_2 = _1;
_4 = 9223372036854775807_isize;
_4 = 9223372036854775807_isize - 9223372036854775807_isize;
_8.0 = [4883_i16];
_4 = 9223372036854775807_isize;
RET = 33210_u16 as f64;
_11 = !true;
_5 = 10024289239636728580_u64 as f32;
_4 = 73_isize;
_14 = _4;
RET = _8.1 as f64;
_13 = (-4091094744258637022_i64) as f64;
_13 = (-29241923756076628200332566287499362564_i128) as f64;
_7 = [(-8193703253769332831_i64),1931907986086987163_i64];
RET = _13 - _13;
_8.0 = [31051_i16];
_8.0 = [(-6051_i16)];
_4 = _14 + _14;
Goto(bb5)
}
bb15 = {
_16.fld5 = _6;
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(18_usize, 10_usize, Move(_10), 2_usize, Move(_2), 14_usize, Move(_14), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(18_usize, 19_usize, Move(_19), 4_usize, Move(_4), 7_usize, Move(_7), 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(776798268_u32), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(48_i8), std::hint::black_box(29261_u16), std::hint::black_box((-1692526213_i32)), std::hint::black_box((-2373423877722114474_i64)), std::hint::black_box((-98884573392261528452573238429152028171_i128)), std::hint::black_box(1421816544985595991_usize), std::hint::black_box(118_u8));
                
            }
#[derive(Debug,Copy,Clone)]
pub struct Adt40 {
fld0: *mut u128,
fld1: char,
fld2: isize,
fld3: [i16; 1],
fld4: i64,
fld5: u64,
}
#[derive(Debug,Copy,Clone)]
pub enum Adt41 {
Variant0{
fld0: (u8, isize),

},
Variant1{
fld0: usize,
fld1: char,
fld2: [bool; 3],

}}
#[derive(Debug)]
pub enum Adt42 {
Variant0{
fld0: [i16; 1],
fld1: *mut *const u8,
fld2: usize,

},
Variant1{
fld0: u64,
fld1: char,
fld2: *mut i64,
fld3: i32,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt43 {
fld0: [bool; 3],
fld1: [isize; 3],
fld2: u128,
fld3: (char, *mut *const u8, u8, *mut *const u8),
fld4: *mut i64,
}
#[derive(Debug)]
pub enum Adt44 {
Variant0{
fld0: i8,
fld1: (u8, isize),
fld2: (u32, i32, isize, u128),

},
Variant1{
fld0: [u8; 2],
fld1: [i16; 2],
fld2: Adt41,
fld3: [bool; 3],
fld4: u64,
fld5: i32,

}}
#[derive(Debug)]
pub enum Adt45 {
Variant0{
fld0: bool,
fld1: (char, *mut *const u8, u8, *mut *const u8),
fld2: u32,
fld3: Adt44,

},
Variant1{
fld0: [i16; 1],
fld1: [u128; 4],
fld2: u32,
fld3: u16,

}}
#[derive(Debug)]
pub enum Adt46 {
Variant0{
fld0: [i8; 5],
fld1: f64,
fld2: Adt45,
fld3: u64,

},
Variant1{
fld0: [i64; 2],
fld1: [i16; 1],
fld2: *mut i64,

},
Variant2{
fld0: u64,

},
Variant3{
fld0: (u8, isize),
fld1: *mut u128,
fld2: Adt43,
fld3: [i128; 5],
fld4: u32,
fld5: *mut *const u8,
fld6: (u32, i32, isize, u128),

}}
#[derive(Debug)]
pub enum Adt47 {
Variant0{
fld0: bool,
fld1: *mut *const u8,
fld2: usize,
fld3: Adt43,
fld4: (u32, i32, isize, u128),
fld5: u8,
fld6: *const *mut [isize; 3],
fld7: Adt44,

},
Variant1{
fld0: [i128; 5],
fld1: (char, *mut *const u8, u8, *mut *const u8),
fld2: Adt45,
fld3: i128,
fld4: (u8, isize),
fld5: [i8; 5],
fld6: [bool; 3],

},
Variant2{
fld0: [i8; 5],
fld1: [i64; 2],
fld2: [isize; 3],
fld3: i64,
fld4: [u128; 4],

}}
#[derive(Debug)]
pub enum Adt48 {
Variant0{
fld0: ([i16; 1], i8, *mut u128),
fld1: char,
fld2: [isize; 3],
fld3: Adt46,
fld4: *const *mut [isize; 3],
fld5: i32,

},
Variant1{
fld0: bool,
fld1: *mut *const u8,
fld2: [u128; 4],
fld3: [bool; 3],
fld4: (u32, i32, isize, u128),
fld5: (u8, isize),
fld6: [u8; 2],

}}
#[derive(Debug)]
pub struct Adt49 {
fld0: [i16; 1],
fld1: [i128; 5],
fld2: [i16; 2],
fld3: Adt40,
fld4: *mut *const u8,
fld5: *mut i64,
fld6: [i64; 2],
}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: Adt49,
fld1: [u8; 2],
fld2: isize,
fld3: [i8; 5],
fld4: [bool; 3],
fld5: Adt43,

},
Variant1{
fld0: Adt49,
fld1: Adt40,
fld2: [bool; 3],
fld3: [u128; 4],
fld4: i16,
fld5: *const *mut [isize; 3],
fld6: *mut u128,
fld7: i128,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt51 {
fld0: [i64; 2],
fld1: *const *mut i64,
}
#[derive(Debug)]
pub struct Adt52 {
fld0: u64,
fld1: [i8; 5],
fld2: *const *mut i64,
fld3: u128,
fld4: Adt40,
fld5: usize,
fld6: Adt45,
fld7: [isize; 3],
}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: i128,
fld1: (char, *mut *const u8, u8, *mut *const u8),

},
Variant1{
fld0: u128,
fld1: Adt48,
fld2: isize,
fld3: Adt50,
fld4: Adt51,
fld5: (char, *mut *const u8, u8, *mut *const u8),

},
Variant2{
fld0: (char, *mut *const u8, u8, *mut *const u8),

}}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: bool,
fld1: (char, *mut *const u8, u8, *mut *const u8),
fld2: ([isize; 3],),

},
Variant1{
fld0: [i8; 5],
fld1: usize,
fld2: Adt52,
fld3: Adt41,

},
Variant2{
fld0: bool,
fld1: u8,
fld2: usize,
fld3: i8,
fld4: [u8; 2],
fld5: [isize; 3],

},
Variant3{
fld0: [i16; 2],
fld1: u32,
fld2: isize,
fld3: Adt43,
fld4: u16,
fld5: ([i16; 1], i8, *mut u128),
fld6: [i8; 5],
fld7: i128,

}}
#[derive(Debug)]
pub struct Adt55 {
fld0: u128,
fld1: Adt42,
fld2: *mut u128,
fld3: usize,
fld4: i16,
fld5: Adt44,
fld6: Adt43,
fld7: i128,
}
#[derive(Debug)]
pub enum Adt56 {
Variant0{
fld0: Adt50,
fld1: u128,
fld2: f32,
fld3: i128,

},
Variant1{
fld0: i128,
fld1: Adt44,

}}

