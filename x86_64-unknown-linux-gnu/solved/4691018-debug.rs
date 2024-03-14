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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> Adt51 {
mir! {
type RET = Adt51;
let _15: *mut f64;
let _16: i128;
let _17: Adt37;
let _18: [u8; 3];
let _19: (i64, [i64; 1], [i64; 1], u16);
let _20: [u128; 1];
let _21: ([u16; 8],);
let _22: f64;
let _23: ([u16; 8],);
let _24: f32;
let _25: [u8; 3];
let _26: isize;
let _27: isize;
let _28: [u32; 8];
let _29: (i64, [i64; 1], [i64; 1], u16);
let _30: Adt39;
let _31: Adt52;
let _32: Adt38;
let _33: i32;
let _34: isize;
let _35: [u128; 1];
let _36: ([u16; 8],);
let _37: usize;
let _38: u128;
let _39: *const *mut usize;
let _40: u16;
let _41: Adt41;
let _42: (i64, [i64; 1], [i64; 1], u16);
let _43: Adt39;
let _44: [u128; 2];
let _45: [u16; 6];
let _46: isize;
let _47: [u128; 2];
let _48: (i64, [i64; 1], [i64; 1], u16);
let _49: u128;
let _50: [u128; 2];
let _51: ();
let _52: ();
{
_5 = 4909_u16 as i16;
_2 = '\u{c3ba1}';
_1 = true & true;
_12 = 3661663870_u32;
_7 = _2 as i64;
_5 = 1726_i16;
_2 = '\u{b377e}';
_3 = 222_u8 as isize;
_4 = 114_i8 << _5;
_14 = !83602544528564417350931872383799639271_u128;
_7 = (-341455613_i32) as i64;
Goto(bb1)
}
bb1 = {
_11 = 49316_u16;
_5 = (-23418_i16) - 6734_i16;
_9 = !15695267483308240847_usize;
_13 = 12653630119627279421_u64;
_11 = _9 as u16;
_2 = '\u{5094d}';
_10 = 236_u8 >> _11;
_9 = 6_usize - 13863984590242870775_usize;
_8 = -65009815644955775681963012910808844281_i128;
_3 = 9223372036854775807_isize ^ 127_isize;
_6 = 1296635987_i32 >> _10;
_14 = _4 as u128;
_19.3 = _11 * _11;
_19.0 = _7;
Goto(bb2)
}
bb2 = {
_6 = _7 as i32;
_16 = _3 as i128;
_10 = _19.0 as u8;
_8 = _16 + _16;
_4 = -51_i8;
_19.2 = [_7];
RET = Adt51::Variant0 { fld0: _12 };
RET = Adt51::Variant0 { fld0: _12 };
_7 = _19.0;
_19.2 = [_19.0];
_2 = '\u{fd839}';
_14 = Field::<u32>(Variant(RET, 0), 0) as u128;
_8 = _16 + _16;
_4 = (-9_i8) + 31_i8;
_12 = _11 as u32;
_12 = Field::<u32>(Variant(RET, 0), 0) - Field::<u32>(Variant(RET, 0), 0);
_10 = !138_u8;
_20 = [_14];
_20 = [_14];
_19.2 = [_19.0];
place!(Field::<u32>(Variant(RET, 0), 0)) = _12 * _12;
SetDiscriminant(RET, 2);
_5 = 13475_i16;
_22 = _10 as f64;
_8 = _16 & _16;
match _13 {
12653630119627279421 => bb4,
_ => bb3
}
}
bb3 = {
_11 = 49316_u16;
_5 = (-23418_i16) - 6734_i16;
_9 = !15695267483308240847_usize;
_13 = 12653630119627279421_u64;
_11 = _9 as u16;
_2 = '\u{5094d}';
_10 = 236_u8 >> _11;
_9 = 6_usize - 13863984590242870775_usize;
_8 = -65009815644955775681963012910808844281_i128;
_3 = 9223372036854775807_isize ^ 127_isize;
_6 = 1296635987_i32 >> _10;
_14 = _4 as u128;
_19.3 = _11 * _11;
_19.0 = _7;
Goto(bb2)
}
bb4 = {
_7 = _19.0;
_11 = _19.3;
place!(Field::<u64>(Variant(RET, 2), 1)) = _14 as u64;
_9 = 6_usize;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld0 = _9 >> _11;
_17 = Adt37::Variant1 { fld0: Field::<Adt46>(Variant(RET, 2), 0).fld0,fld1: _13,fld2: _22,fld3: _10 };
_23.0[_9] = Field::<f64>(Variant(_17, 1), 2) as u16;
_19.1 = _19.2;
_23.0 = [_19.3,_19.3,_19.3,_19.3,_11,_11,_11,_19.3];
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
place!(Field::<u8>(Variant(_17, 1), 3)) = !_10;
_13 = Field::<u64>(Variant(_17, 1), 1);
_7 = Field::<u64>(Variant(_17, 1), 1) as i64;
_12 = _1 as u32;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
SetDiscriminant(_17, 0);
Goto(bb5)
}
bb5 = {
_10 = _5 as u8;
_16 = _8 << _8;
_20 = [_14];
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld1 = [_11];
_26 = -_3;
place!(Field::<[i64; 1]>(Variant(_17, 0), 0)) = [_7];
_27 = _3 | _3;
place!(Field::<*mut f32>(Variant(_17, 0), 1)) = core::ptr::addr_of_mut!(_24);
_28 = [_12,_12,_12,_12,_12,_12,_12,_12];
place!(Field::<*mut f32>(Variant(_17, 0), 1)) = core::ptr::addr_of_mut!(_24);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
_29.1 = [_19.0];
_27 = _3 + _3;
_29.1 = _19.2;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld1 = [_23.0[_9]];
_19.1 = [_7];
_6 = _1 as i32;
Goto(bb6)
}
bb6 = {
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
_30 = Adt39::Variant2 { fld0: Field::<Adt46>(Variant(RET, 2), 0).fld1,fld1: _2,fld2: _28[_9] };
_29 = (_19.0, _19.2, _19.1, _23.0[_9]);
_9 = !Field::<Adt46>(Variant(RET, 2), 0).fld0;
place!(Field::<i32>(Variant(_17, 0), 5)) = -_6;
place!(Field::<isize>(Variant(_17, 0), 2)) = _3 - _3;
_19.2 = [_19.0];
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
place!(Field::<[u16; 1]>(Variant(_17, 0), 4)) = Field::<[u16; 1]>(Variant(_30, 2), 0);
Goto(bb7)
}
bb7 = {
_21 = (_23.0,);
place!(Field::<char>(Variant(_30, 2), 1)) = _2;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld2 = _30;
place!(Field::<[u16; 1]>(Variant(_30, 2), 0)) = Field::<[u16; 1]>(Variant(_17, 0), 4);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld1 = [_19.3];
_24 = _9 as f32;
_2 = Field::<char>(Variant(Field::<Adt46>(Variant(RET, 2), 0).fld2, 2), 1);
place!(Field::<[u16; 1]>(Variant(_17, 0), 4)) = Field::<Adt46>(Variant(RET, 2), 0).fld1;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld0 = _9;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
SetDiscriminant(Field::<Adt46>(Variant(RET, 2), 0).fld2, 2);
_17 = Adt37::Variant1 { fld0: Field::<Adt46>(Variant(RET, 2), 0).fld0,fld1: Field::<u64>(Variant(RET, 2), 1),fld2: _22,fld3: _10 };
_27 = _3;
_20 = [_14];
place!(Field::<char>(Variant(place!(Field::<Adt46>(Variant(RET, 2), 0)).fld2, 2), 1)) = _2;
_19.1 = _29.1;
_35 = [_14];
_25 = [Field::<u8>(Variant(_17, 1), 3),Field::<u8>(Variant(_17, 1), 3),_10];
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld1 = [_19.3];
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld2 = _30;
_19 = (_29.0, _29.1, _29.2, _11);
_32 = Adt38::Variant2 { fld0: _1,fld1: _17,fld2: _16,fld3: _25,fld4: _29.3,fld5: _6 };
_18 = [Field::<u8>(Variant(Field::<Adt37>(Variant(_32, 2), 1), 1), 3),Field::<u8>(Variant(_17, 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_32, 2), 1), 1), 3)];
_19 = (_29.0, _29.1, _29.2, _29.3);
place!(Field::<u64>(Variant(RET, 2), 1)) = !Field::<u64>(Variant(Field::<Adt37>(Variant(_32, 2), 1), 1), 1);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
Call(place!(Field::<u64>(Variant(RET, 2), 1)) = fn1(Move(_32), Field::<Adt46>(Variant(RET, 2), 0), _30, Field::<Adt46>(Variant(RET, 2), 0).fld2, _27, Field::<Adt46>(Variant(RET, 2), 0), Field::<u32>(Variant(_30, 2), 2), _17, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_22 = -Field::<f64>(Variant(_17, 1), 2);
_19.2 = [_19.0];
_32 = Adt38::Variant3 { fld0: _7,fld1: _19.3 };
_33 = _6 + _6;
_23 = (_21.0,);
_19.1 = _29.1;
place!(Field::<f64>(Variant(_17, 1), 2)) = _26 as f64;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
_21.0 = _23.0;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld0 = _1 as usize;
place!(Field::<char>(Variant(place!(Field::<Adt46>(Variant(RET, 2), 0)).fld2, 2), 1)) = _2;
_19.2 = [_29.0];
place!(Field::<i64>(Variant(_32, 3), 0)) = _7;
_33 = _6 | _6;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
12653630119627279421 => bb10,
_ => bb9
}
}
bb9 = {
_11 = 49316_u16;
_5 = (-23418_i16) - 6734_i16;
_9 = !15695267483308240847_usize;
_13 = 12653630119627279421_u64;
_11 = _9 as u16;
_2 = '\u{5094d}';
_10 = 236_u8 >> _11;
_9 = 6_usize - 13863984590242870775_usize;
_8 = -65009815644955775681963012910808844281_i128;
_3 = 9223372036854775807_isize ^ 127_isize;
_6 = 1296635987_i32 >> _10;
_14 = _4 as u128;
_19.3 = _11 * _11;
_19.0 = _7;
Goto(bb2)
}
bb10 = {
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
_37 = Field::<u64>(Variant(RET, 2), 1) as usize;
_21 = (_23.0,);
_29 = (_7, _19.2, _19.1, _19.3);
RET = Adt51::Variant0 { fld0: _12 };
_19.2 = [Field::<i64>(Variant(_32, 3), 0)];
place!(Field::<[u16; 1]>(Variant(_30, 2), 0)) = [_29.3];
_36 = (_21.0,);
_33 = -_6;
_10 = Field::<u8>(Variant(_17, 1), 3) << Field::<u16>(Variant(_32, 3), 1);
_19.3 = Field::<u16>(Variant(_32, 3), 1);
_34 = _3 >> _3;
_29.0 = _7 - Field::<i64>(Variant(_32, 3), 0);
place!(Field::<[u16; 1]>(Variant(_30, 2), 0)) = [_19.3];
_15 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_17, 1), 2)));
place!(Field::<u32>(Variant(_30, 2), 2)) = _12;
SetDiscriminant(RET, 2);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld0 = Field::<usize>(Variant(_17, 1), 0);
SetDiscriminant(_30, 2);
place!(Field::<u32>(Variant(_30, 2), 2)) = !_12;
place!(Field::<u64>(Variant(_17, 1), 1)) = !_13;
place!(Field::<u64>(Variant(RET, 2), 1)) = _13 ^ Field::<u64>(Variant(_17, 1), 1);
_10 = _11 as u8;
place!(Field::<i64>(Variant(_32, 3), 0)) = _19.0 * _7;
_35 = [_14];
Call(_10 = core::intrinsics::transmute(_1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
_4 = !7_i8;
_19.3 = _13 as u16;
_42 = (Field::<i64>(Variant(_32, 3), 0), _19.1, _29.2, _29.3);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld1 = [_29.3];
_19 = (_42.0, _42.2, _42.1, _42.3);
_29 = (_19.0, _19.1, _42.1, _42.3);
Goto(bb12)
}
bb12 = {
place!(Field::<char>(Variant(_30, 2), 1)) = _2;
SetDiscriminant(_32, 0);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
_42.3 = _29.3 | _11;
place!(Field::<u8>(Variant(_17, 1), 3)) = Field::<Adt46>(Variant(RET, 2), 0).fld0 as u8;
_21 = _23;
_44 = [_14,_14];
_42.0 = _12 as i64;
_19.2 = [_42.0];
place!(Field::<isize>(Variant(_32, 0), 2)) = _4 as isize;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld0 = Field::<usize>(Variant(_17, 1), 0) + _9;
_6 = _4 as i32;
Goto(bb13)
}
bb13 = {
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
place!(Field::<[u128; 8]>(Variant(_32, 0), 1)) = [_14,_14,_14,_14,_14,_14,_14,_14];
_17 = Adt37::Variant1 { fld0: _9,fld1: Field::<u64>(Variant(RET, 2), 1),fld2: _22,fld3: _10 };
_5 = (-11147_i16) << _34;
place!(Field::<i32>(Variant(_32, 0), 5)) = !_33;
_14 = 314058765558344556193794326877455582272_u128;
place!(Field::<[u16; 8]>(Variant(_32, 0), 6)) = _21.0;
place!(Field::<[u128; 8]>(Variant(_32, 0), 1)) = [_14,_14,_14,_14,_14,_14,_14,_14];
_19.3 = _37 as u16;
_42.1 = [_42.0];
_27 = _7 as isize;
_47 = [_14,_14];
SetDiscriminant(_17, 1);
_42.3 = _11 << _34;
_21.0 = Field::<[u16; 8]>(Variant(_32, 0), 6);
match _13 {
0 => bb1,
1 => bb6,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
12653630119627279421 => bb19,
_ => bb18
}
}
bb14 = {
place!(Field::<char>(Variant(_30, 2), 1)) = _2;
SetDiscriminant(_32, 0);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
_42.3 = _29.3 | _11;
place!(Field::<u8>(Variant(_17, 1), 3)) = Field::<Adt46>(Variant(RET, 2), 0).fld0 as u8;
_21 = _23;
_44 = [_14,_14];
_42.0 = _12 as i64;
_19.2 = [_42.0];
place!(Field::<isize>(Variant(_32, 0), 2)) = _4 as isize;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld0 = Field::<usize>(Variant(_17, 1), 0) + _9;
_6 = _4 as i32;
Goto(bb13)
}
bb15 = {
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
_4 = !7_i8;
_19.3 = _13 as u16;
_42 = (Field::<i64>(Variant(_32, 3), 0), _19.1, _29.2, _29.3);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld1 = [_29.3];
_19 = (_42.0, _42.2, _42.1, _42.3);
_29 = (_19.0, _19.1, _42.1, _42.3);
Goto(bb12)
}
bb16 = {
_11 = 49316_u16;
_5 = (-23418_i16) - 6734_i16;
_9 = !15695267483308240847_usize;
_13 = 12653630119627279421_u64;
_11 = _9 as u16;
_2 = '\u{5094d}';
_10 = 236_u8 >> _11;
_9 = 6_usize - 13863984590242870775_usize;
_8 = -65009815644955775681963012910808844281_i128;
_3 = 9223372036854775807_isize ^ 127_isize;
_6 = 1296635987_i32 >> _10;
_14 = _4 as u128;
_19.3 = _11 * _11;
_19.0 = _7;
Goto(bb2)
}
bb17 = {
_10 = _5 as u8;
_16 = _8 << _8;
_20 = [_14];
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld1 = [_11];
_26 = -_3;
place!(Field::<[i64; 1]>(Variant(_17, 0), 0)) = [_7];
_27 = _3 | _3;
place!(Field::<*mut f32>(Variant(_17, 0), 1)) = core::ptr::addr_of_mut!(_24);
_28 = [_12,_12,_12,_12,_12,_12,_12,_12];
place!(Field::<*mut f32>(Variant(_17, 0), 1)) = core::ptr::addr_of_mut!(_24);
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld4 = [_14,_14];
_29.1 = [_19.0];
_27 = _3 + _3;
_29.1 = _19.2;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld1 = [_23.0[_9]];
_19.1 = [_7];
_6 = _1 as i32;
Goto(bb6)
}
bb18 = {
_22 = -Field::<f64>(Variant(_17, 1), 2);
_19.2 = [_19.0];
_32 = Adt38::Variant3 { fld0: _7,fld1: _19.3 };
_33 = _6 + _6;
_23 = (_21.0,);
_19.1 = _29.1;
place!(Field::<f64>(Variant(_17, 1), 2)) = _26 as f64;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld3.0 = core::ptr::addr_of!(_15);
_21.0 = _23.0;
place!(Field::<Adt46>(Variant(RET, 2), 0)).fld0 = _1 as usize;
place!(Field::<char>(Variant(place!(Field::<Adt46>(Variant(RET, 2), 0)).fld2, 2), 1)) = _2;
_19.2 = [_29.0];
place!(Field::<i64>(Variant(_32, 3), 0)) = _7;
_33 = _6 | _6;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
12653630119627279421 => bb10,
_ => bb9
}
}
bb19 = {
_41.fld0 = core::ptr::addr_of!(_1);
place!(Field::<char>(Variant(_30, 2), 1)) = _2;
_29.2 = [_29.0];
_5 = -(-25695_i16);
_29.3 = _19.3;
place!(Field::<[u16; 1]>(Variant(_30, 2), 0)) = [_11];
place!(Field::<usize>(Variant(_17, 1), 0)) = Field::<Adt46>(Variant(RET, 2), 0).fld0 - Field::<Adt46>(Variant(RET, 2), 0).fld0;
RET = Adt51::Variant0 { fld0: Field::<u32>(Variant(_30, 2), 2) };
_46 = !_3;
_45 = [_42.3,_42.3,_42.3,_29.3,_19.3,_42.3];
_46 = _3 + Field::<isize>(Variant(_32, 0), 2);
place!(Field::<f64>(Variant(_17, 1), 2)) = _22 + _22;
_36 = (Field::<[u16; 8]>(Variant(_32, 0), 6),);
Goto(bb20)
}
bb20 = {
Call(_51 = dump_var(0_usize, 33_usize, Move(_33), 6_usize, Move(_6), 9_usize, Move(_9), 35_usize, Move(_35)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_51 = dump_var(0_usize, 5_usize, Move(_5), 14_usize, Move(_14), 27_usize, Move(_27), 21_usize, Move(_21)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_51 = dump_var(0_usize, 20_usize, Move(_20), 46_usize, Move(_46), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_51 = dump_var(0_usize, 13_usize, Move(_13), 7_usize, Move(_7), 8_usize, Move(_8), 12_usize, Move(_12)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_51 = dump_var(0_usize, 19_usize, Move(_19), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: Adt38,mut _2: Adt46,mut _3: Adt39,mut _4: Adt39,mut _5: isize,mut _6: Adt46,mut _7: u32,mut _8: Adt37,mut _9: u8) -> u64 {
mir! {
type RET = u64;
let _10: isize;
let _11: bool;
let _12: f64;
let _13: Adt42;
let _14: u8;
let _15: Adt52;
let _16: u8;
let _17: [u32; 8];
let _18: *const *mut f64;
let _19: [u128; 2];
let _20: Adt44;
let _21: *const *mut f64;
let _22: f64;
let _23: Adt47;
let _24: u8;
let _25: u32;
let _26: [u128; 2];
let _27: bool;
let _28: *const bool;
let _29: [u8; 3];
let _30: [u16; 8];
let _31: char;
let _32: usize;
let _33: u32;
let _34: i128;
let _35: isize;
let _36: f32;
let _37: usize;
let _38: i32;
let _39: [u128; 8];
let _40: f32;
let _41: [u128; 2];
let _42: *mut f32;
let _43: Adt48;
let _44: isize;
let _45: Adt43;
let _46: *mut usize;
let _47: ();
let _48: ();
{
place!(Field::<[u16; 1]>(Variant(_4, 2), 0)) = [Field::<u16>(Variant(_1, 2), 4)];
place!(Field::<u32>(Variant(_6.fld2, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
_9 = Field::<u8>(Variant(Field::<Adt37>(Variant(_1, 2), 1), 1), 3) | Field::<u8>(Variant(Field::<Adt37>(Variant(_1, 2), 1), 1), 3);
_2 = _6;
place!(Field::<u64>(Variant(place!(Field::<Adt37>(Variant(_1, 2), 1)), 1), 1)) = !Field::<u64>(Variant(_8, 1), 1);
_10 = _5;
_2.fld4 = [291610906625453175792836765904365889903_u128,324088280149523049605440085765354399935_u128];
place!(Field::<u8>(Variant(_8, 1), 3)) = !_9;
place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_1, 2), 1)), 1), 0)) = Field::<usize>(Variant(_8, 1), 0);
_11 = !Field::<bool>(Variant(_1, 2), 0);
place!(Field::<Adt37>(Variant(_1, 2), 1)) = _8;
place!(Field::<u32>(Variant(_2.fld2, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
place!(Field::<Adt37>(Variant(_1, 2), 1)) = _8;
place!(Field::<u8>(Variant(_8, 1), 3)) = Field::<u8>(Variant(Field::<Adt37>(Variant(_1, 2), 1), 1), 3) + _9;
place!(Field::<f64>(Variant(_8, 1), 2)) = 6579_i16 as f64;
place!(Field::<f64>(Variant(_8, 1), 2)) = Field::<f64>(Variant(Field::<Adt37>(Variant(_1, 2), 1), 1), 2);
place!(Field::<bool>(Variant(_1, 2), 0)) = _11;
_2.fld0 = Field::<usize>(Variant(_8, 1), 0) >> _9;
Goto(bb1)
}
bb1 = {
_11 = Field::<bool>(Variant(_1, 2), 0);
place!(Field::<f64>(Variant(_8, 1), 2)) = Field::<f64>(Variant(Field::<Adt37>(Variant(_1, 2), 1), 1), 2) + Field::<f64>(Variant(Field::<Adt37>(Variant(_1, 2), 1), 1), 2);
SetDiscriminant(_6.fld2, 0);
_6.fld0 = Field::<usize>(Variant(Field::<Adt37>(Variant(_1, 2), 1), 1), 0) | _2.fld0;
RET = 4003429763410642403_i64 as u64;
_12 = Field::<f64>(Variant(_8, 1), 2) + Field::<f64>(Variant(_8, 1), 2);
_2.fld4 = [140893304954316755579287547776762048052_u128,168350786386990838633279453128810468154_u128];
_6.fld2 = _3;
place!(Field::<i128>(Variant(_1, 2), 2)) = (-15183121755851000845790649411181249737_i128);
_2.fld2 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_3, 2), 0),fld1: Field::<char>(Variant(_3, 2), 1),fld2: Field::<u32>(Variant(_6.fld2, 2), 2) };
_14 = !Field::<u8>(Variant(_8, 1), 3);
_6 = _2;
place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_1, 2), 1)), 1), 0)) = 186424783486111163633305732578429854046_u128 as usize;
_2.fld0 = _6.fld0;
place!(Field::<char>(Variant(_6.fld2, 2), 1)) = Field::<char>(Variant(_2.fld2, 2), 1);
_2.fld0 = _6.fld0;
place!(Field::<char>(Variant(_2.fld2, 2), 1)) = Field::<char>(Variant(_3, 2), 1);
_13.fld1 = _4;
place!(Field::<[u16; 1]>(Variant(_4, 2), 0)) = Field::<[u16; 1]>(Variant(_2.fld2, 2), 0);
_10 = _5;
place!(Field::<u64>(Variant(place!(Field::<Adt37>(Variant(_1, 2), 1)), 1), 1)) = !RET;
_12 = Field::<f64>(Variant(_8, 1), 2) + Field::<f64>(Variant(Field::<Adt37>(Variant(_1, 2), 1), 1), 2);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [Field::<u16>(Variant(_1, 2), 4)];
_3 = _13.fld1;
place!(Field::<u32>(Variant(_13.fld1, 2), 2)) = Field::<i32>(Variant(_1, 2), 5) as u32;
Call(place!(Field::<[u16; 1]>(Variant(_13.fld1, 2), 0)) = fn2(_11, Field::<u32>(Variant(_3, 2), 2), RET, Move(_1), Field::<u32>(Variant(_4, 2), 2), _2.fld0, _6, _3, _6, _6.fld2, Field::<u8>(Variant(_8, 1), 3), _2.fld2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _13.fld1;
_8 = Adt37::Variant1 { fld0: _6.fld0,fld1: RET,fld2: _12,fld3: _9 };
_2.fld1 = Field::<[u16; 1]>(Variant(_3, 2), 0);
_6.fld2 = _3;
_13.fld1 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_3, 2), 0),fld1: Field::<char>(Variant(_3, 2), 1),fld2: _7 };
place!(Field::<char>(Variant(_2.fld2, 2), 1)) = Field::<char>(Variant(_13.fld1, 2), 1);
place!(Field::<char>(Variant(_2.fld2, 2), 1)) = Field::<char>(Variant(_3, 2), 1);
_6 = _2;
place!(Field::<char>(Variant(_4, 2), 1)) = Field::<char>(Variant(_3, 2), 1);
place!(Field::<u8>(Variant(_8, 1), 3)) = !_14;
_5 = _11 as isize;
place!(Field::<usize>(Variant(_8, 1), 0)) = _2.fld0;
place!(Field::<[u16; 1]>(Variant(_2.fld2, 2), 0)) = [65232_u16];
_6.fld0 = Field::<usize>(Variant(_8, 1), 0) * Field::<usize>(Variant(_8, 1), 0);
place!(Field::<[u16; 1]>(Variant(_13.fld1, 2), 0)) = _2.fld1;
_6.fld0 = Field::<usize>(Variant(_8, 1), 0) * _2.fld0;
RET = (-110_i8) as u64;
_8 = Adt37::Variant1 { fld0: _6.fld0,fld1: RET,fld2: _12,fld3: _9 };
_9 = _14 >> _6.fld0;
_16 = _14;
_13.fld0 = [43144_u16,11991_u16,22883_u16,7125_u16,31510_u16,53636_u16];
_13.fld0 = [19912_u16,13891_u16,30679_u16,54804_u16,30965_u16,47677_u16];
_6 = _2;
place!(Field::<u32>(Variant(_13.fld1, 2), 2)) = _7 ^ Field::<u32>(Variant(_3, 2), 2);
_3 = _2.fld2;
place!(Field::<u32>(Variant(_13.fld1, 2), 2)) = Field::<u32>(Variant(_3, 2), 2) - _7;
_6.fld2 = _3;
place!(Field::<char>(Variant(_4, 2), 1)) = Field::<char>(Variant(_6.fld2, 2), 1);
_9 = _16;
Call(_5 = fn17(_2, _6, _2, _13, Field::<f64>(Variant(_8, 1), 2), _2, _6.fld2, _2.fld1, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = Field::<usize>(Variant(_8, 1), 0) as u32;
_9 = _16 >> Field::<usize>(Variant(_8, 1), 0);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [12759_u16];
_9 = _14;
place!(Field::<char>(Variant(_3, 2), 1)) = Field::<char>(Variant(_4, 2), 1);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [43011_u16];
place!(Field::<[u16; 1]>(Variant(_4, 2), 0)) = [7992_u16];
_4 = _6.fld2;
SetDiscriminant(_13.fld1, 2);
_2.fld0 = _6.fld0 << _10;
RET = Field::<u64>(Variant(_8, 1), 1) + Field::<u64>(Variant(_8, 1), 1);
_6 = _2;
_8 = Adt37::Variant1 { fld0: _6.fld0,fld1: RET,fld2: _12,fld3: _9 };
_14 = Field::<char>(Variant(_6.fld2, 2), 1) as u8;
_12 = _9 as f64;
place!(Field::<[u8; 3]>(Variant(_3, 1), 7)) = [_9,_9,Field::<u8>(Variant(_8, 1), 3)];
place!(Field::<[u16; 1]>(Variant(_2.fld2, 2), 0)) = _6.fld1;
_9 = RET as u8;
place!(Field::<u64>(Variant(_3, 1), 4)) = Field::<u64>(Variant(_8, 1), 1) ^ RET;
place!(Field::<usize>(Variant(_8, 1), 0)) = _2.fld0;
_12 = (-2165579480376077768_i64) as f64;
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_4, 2), 1);
_7 = 62707819044225086705960886269791613470_i128 as u32;
_26 = _2.fld4;
Goto(bb4)
}
bb4 = {
_22 = _12;
_13.fld1 = _2.fld2;
place!(Field::<[u16; 1]>(Variant(_6.fld2, 2), 0)) = [37113_u16];
place!(Field::<u32>(Variant(_4, 2), 2)) = Field::<u32>(Variant(_6.fld2, 2), 2) >> RET;
Goto(bb5)
}
bb5 = {
_7 = Field::<u32>(Variant(_4, 2), 2);
_25 = _7 & Field::<u32>(Variant(_6.fld2, 2), 2);
_4 = _13.fld1;
Goto(bb6)
}
bb6 = {
place!(Field::<[u16; 1]>(Variant(_13.fld1, 2), 0)) = _2.fld1;
_5 = -_10;
_28 = core::ptr::addr_of!(_27);
_6.fld0 = !_2.fld0;
place!(Field::<u8>(Variant(_8, 1), 3)) = (-152541321_i32) as u8;
_25 = Field::<u32>(Variant(_4, 2), 2);
_26 = _6.fld4;
place!(Field::<char>(Variant(_13.fld1, 2), 1)) = Field::<char>(Variant(_3, 1), 1);
RET = Field::<u64>(Variant(_3, 1), 4);
_27 = !_11;
_16 = !_14;
place!(Field::<bool>(Variant(_6.fld2, 1), 0)) = _11;
Goto(bb7)
}
bb7 = {
_6.fld1 = [231_u16];
place!(Field::<*mut usize>(Variant(_6.fld2, 1), 6)) = core::ptr::addr_of_mut!(_2.fld0);
_24 = _9 | _9;
_14 = _9;
place!(Field::<[u32; 8]>(Variant(_6.fld2, 1), 5)) = [Field::<u32>(Variant(_2.fld2, 2), 2),_7,Field::<u32>(Variant(_2.fld2, 2), 2),_7,_7,_25,_7,_25];
_17 = Field::<[u32; 8]>(Variant(_6.fld2, 1), 5);
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)).0 = [15934_u16,63898_u16,1723_u16,64537_u16,6214_u16,54190_u16,38621_u16,46582_u16];
Goto(bb8)
}
bb8 = {
place!(Field::<u64>(Variant(_6.fld2, 1), 4)) = Field::<u64>(Variant(_3, 1), 4) << Field::<u32>(Variant(_2.fld2, 2), 2);
_2.fld4 = [139042017003710145959315604574232446947_u128,309413744182982464177891654286846830414_u128];
_32 = _2.fld0;
_33 = !_25;
Goto(bb9)
}
bb9 = {
_15 = Adt52::Variant3 { fld0: 9603_u16,fld1: 1764639858_i32 };
_4 = _2.fld2;
_24 = !Field::<u8>(Variant(_8, 1), 3);
place!(Field::<Adt37>(Variant(_6.fld2, 1), 3)) = _8;
_34 = (-68127998992859274527948760410282547841_i128) * (-47116552005522033795869540929038606152_i128);
place!(Field::<([u16; 8],)>(Variant(_6.fld2, 1), 2)).0 = [32328_u16,14827_u16,12047_u16,13642_u16,45394_u16,12364_u16,24431_u16,46095_u16];
_6 = _2;
_26 = _2.fld4;
_31 = Field::<char>(Variant(_4, 2), 1);
_36 = 25622_i16 as f32;
_38 = (-907295213_i32) >> Field::<u32>(Variant(_13.fld1, 2), 2);
place!(Field::<f64>(Variant(_8, 1), 2)) = _36 as f64;
_29 = Field::<[u8; 3]>(Variant(_3, 1), 7);
_10 = 6980685939651063146_i64 as isize;
_3 = _6.fld2;
_33 = _7;
Goto(bb10)
}
bb10 = {
SetDiscriminant(_8, 1);
_6.fld4 = [324649329975312801512467035981481123875_u128,240883565133436315819836824877934913760_u128];
place!(Field::<[u16; 1]>(Variant(_13.fld1, 2), 0)) = _6.fld1;
place!(Field::<u64>(Variant(_8, 1), 1)) = _14 as u64;
_35 = _5;
RET = Field::<u64>(Variant(_8, 1), 1) * Field::<u64>(Variant(_8, 1), 1);
_37 = _35 as usize;
_6 = _2;
_13.fld0 = [37670_u16,13539_u16,5622_u16,54672_u16,17558_u16,2196_u16];
_40 = -_36;
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [50566_u16];
_19 = [316083505815511545917924845182716710279_u128,301826478211827351996767419793541762695_u128];
place!(Field::<u32>(Variant(_2.fld2, 2), 2)) = _33;
place!(Field::<u32>(Variant(_6.fld2, 2), 2)) = _7 * _33;
_13.fld0 = [1843_u16,31748_u16,34271_u16,15303_u16,24515_u16,49189_u16];
_36 = RET as f32;
_40 = -_36;
_5 = _10 & _35;
place!(Field::<u32>(Variant(_13.fld1, 2), 2)) = Field::<u32>(Variant(_4, 2), 2);
_39 = [193576468874577016318238125145204569262_u128,3926452755281888457699023152553079587_u128,146230522581369313838005371363643215034_u128,15276535333575052015849410235158984977_u128,235744162296153066895242496020248012977_u128,204068008792739407875992714282962274217_u128,160547360407869703333340001834058181069_u128,279051178767608204299094887248091755984_u128];
place!(Field::<char>(Variant(_13.fld1, 2), 1)) = _31;
Goto(bb11)
}
bb11 = {
place!(Field::<u16>(Variant(_15, 3), 0)) = 22182_u16;
place!(Field::<i32>(Variant(_15, 3), 1)) = _38 & _38;
place!(Field::<char>(Variant(_4, 2), 1)) = _31;
_2.fld2 = _6.fld2;
_34 = -106557386156564239867999744473836514831_i128;
_30 = [Field::<u16>(Variant(_15, 3), 0),Field::<u16>(Variant(_15, 3), 0),Field::<u16>(Variant(_15, 3), 0),Field::<u16>(Variant(_15, 3), 0),Field::<u16>(Variant(_15, 3), 0),Field::<u16>(Variant(_15, 3), 0),Field::<u16>(Variant(_15, 3), 0),Field::<u16>(Variant(_15, 3), 0)];
_29 = [_24,_16,_9];
_14 = _16;
place!(Field::<*mut usize>(Variant(_13.fld1, 1), 6)) = core::ptr::addr_of_mut!(_2.fld0);
_27 = !_11;
match Field::<u16>(Variant(_15, 3), 0) {
0 => bb5,
1 => bb7,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
22182 => bb18,
_ => bb17
}
}
bb12 = {
_7 = Field::<usize>(Variant(_8, 1), 0) as u32;
_9 = _16 >> Field::<usize>(Variant(_8, 1), 0);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [12759_u16];
_9 = _14;
place!(Field::<char>(Variant(_3, 2), 1)) = Field::<char>(Variant(_4, 2), 1);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [43011_u16];
place!(Field::<[u16; 1]>(Variant(_4, 2), 0)) = [7992_u16];
_4 = _6.fld2;
SetDiscriminant(_13.fld1, 2);
_2.fld0 = _6.fld0 << _10;
RET = Field::<u64>(Variant(_8, 1), 1) + Field::<u64>(Variant(_8, 1), 1);
_6 = _2;
_8 = Adt37::Variant1 { fld0: _6.fld0,fld1: RET,fld2: _12,fld3: _9 };
_14 = Field::<char>(Variant(_6.fld2, 2), 1) as u8;
SetDiscriminant(_3, 1);
_12 = _9 as f64;
place!(Field::<[u8; 3]>(Variant(_3, 1), 7)) = [_9,_9,Field::<u8>(Variant(_8, 1), 3)];
place!(Field::<[u16; 1]>(Variant(_2.fld2, 2), 0)) = _6.fld1;
_9 = RET as u8;
place!(Field::<u64>(Variant(_3, 1), 4)) = Field::<u64>(Variant(_8, 1), 1) ^ RET;
place!(Field::<usize>(Variant(_8, 1), 0)) = _2.fld0;
_12 = (-2165579480376077768_i64) as f64;
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_4, 2), 1);
_7 = 62707819044225086705960886269791613470_i128 as u32;
_26 = _2.fld4;
Goto(bb4)
}
bb13 = {
_3 = _13.fld1;
_8 = Adt37::Variant1 { fld0: _6.fld0,fld1: RET,fld2: _12,fld3: _9 };
_2.fld1 = Field::<[u16; 1]>(Variant(_3, 2), 0);
_6.fld2 = _3;
_13.fld1 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_3, 2), 0),fld1: Field::<char>(Variant(_3, 2), 1),fld2: _7 };
place!(Field::<char>(Variant(_2.fld2, 2), 1)) = Field::<char>(Variant(_13.fld1, 2), 1);
place!(Field::<char>(Variant(_2.fld2, 2), 1)) = Field::<char>(Variant(_3, 2), 1);
_6 = _2;
place!(Field::<char>(Variant(_4, 2), 1)) = Field::<char>(Variant(_3, 2), 1);
place!(Field::<u8>(Variant(_8, 1), 3)) = !_14;
_5 = _11 as isize;
place!(Field::<usize>(Variant(_8, 1), 0)) = _2.fld0;
place!(Field::<[u16; 1]>(Variant(_2.fld2, 2), 0)) = [65232_u16];
_6.fld0 = Field::<usize>(Variant(_8, 1), 0) * Field::<usize>(Variant(_8, 1), 0);
place!(Field::<[u16; 1]>(Variant(_13.fld1, 2), 0)) = _2.fld1;
_6.fld0 = Field::<usize>(Variant(_8, 1), 0) * _2.fld0;
RET = (-110_i8) as u64;
_8 = Adt37::Variant1 { fld0: _6.fld0,fld1: RET,fld2: _12,fld3: _9 };
_9 = _14 >> _6.fld0;
_16 = _14;
_13.fld0 = [43144_u16,11991_u16,22883_u16,7125_u16,31510_u16,53636_u16];
_13.fld0 = [19912_u16,13891_u16,30679_u16,54804_u16,30965_u16,47677_u16];
_6 = _2;
place!(Field::<u32>(Variant(_13.fld1, 2), 2)) = _7 ^ Field::<u32>(Variant(_3, 2), 2);
_3 = _2.fld2;
place!(Field::<u32>(Variant(_13.fld1, 2), 2)) = Field::<u32>(Variant(_3, 2), 2) - _7;
_6.fld2 = _3;
place!(Field::<char>(Variant(_4, 2), 1)) = Field::<char>(Variant(_6.fld2, 2), 1);
_9 = _16;
Call(_5 = fn17(_2, _6, _2, _13, Field::<f64>(Variant(_8, 1), 2), _2, _6.fld2, _2.fld1, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
place!(Field::<u64>(Variant(_6.fld2, 1), 4)) = Field::<u64>(Variant(_3, 1), 4) << Field::<u32>(Variant(_2.fld2, 2), 2);
_2.fld4 = [139042017003710145959315604574232446947_u128,309413744182982464177891654286846830414_u128];
_32 = _2.fld0;
_33 = !_25;
Goto(bb9)
}
bb15 = {
_6.fld1 = [231_u16];
place!(Field::<*mut usize>(Variant(_6.fld2, 1), 6)) = core::ptr::addr_of_mut!(_2.fld0);
_24 = _9 | _9;
SetDiscriminant(_4, 1);
_14 = _9;
place!(Field::<[u32; 8]>(Variant(_6.fld2, 1), 5)) = [Field::<u32>(Variant(_2.fld2, 2), 2),_7,Field::<u32>(Variant(_2.fld2, 2), 2),_7,_7,_25,_7,_25];
_17 = Field::<[u32; 8]>(Variant(_6.fld2, 1), 5);
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)).0 = [15934_u16,63898_u16,1723_u16,64537_u16,6214_u16,54190_u16,38621_u16,46582_u16];
Goto(bb8)
}
bb16 = {
place!(Field::<[u16; 1]>(Variant(_13.fld1, 2), 0)) = _2.fld1;
_5 = -_10;
_28 = core::ptr::addr_of!(_27);
_6.fld0 = !_2.fld0;
place!(Field::<u8>(Variant(_8, 1), 3)) = (-152541321_i32) as u8;
SetDiscriminant(_6.fld2, 1);
_25 = Field::<u32>(Variant(_4, 2), 2);
_26 = _6.fld4;
place!(Field::<char>(Variant(_13.fld1, 2), 1)) = Field::<char>(Variant(_3, 1), 1);
RET = Field::<u64>(Variant(_3, 1), 4);
_27 = !_11;
_16 = !_14;
place!(Field::<bool>(Variant(_6.fld2, 1), 0)) = _11;
Goto(bb7)
}
bb17 = {
_22 = _12;
_13.fld1 = _2.fld2;
place!(Field::<[u16; 1]>(Variant(_6.fld2, 2), 0)) = [37113_u16];
place!(Field::<u32>(Variant(_4, 2), 2)) = Field::<u32>(Variant(_6.fld2, 2), 2) >> RET;
Goto(bb5)
}
bb18 = {
place!(Field::<bool>(Variant(_13.fld1, 1), 0)) = !(*_28);
_27 = !_11;
_12 = _22;
SetDiscriminant(_15, 1);
place!(Field::<*const *mut usize>(Variant(_15, 1), 1)) = core::ptr::addr_of!(place!(Field::<*mut usize>(Variant(_13.fld1, 1), 6)));
place!(Field::<[u32; 8]>(Variant(_15, 1), 6)) = _17;
_5 = -_35;
SetDiscriminant(_3, 0);
place!(Field::<Adt38>(Variant(_15, 1), 2)) = Adt38::Variant0 { fld0: Field::<u64>(Variant(_8, 1), 1),fld1: _39,fld2: _10,fld3: 30_i8,fld4: _28,fld5: _38,fld6: _30,fld7: _34 };
place!(Field::<i32>(Variant(place!(Field::<Adt38>(Variant(_15, 1), 2)), 0), 5)) = _16 as i32;
_41 = [243674331453698762033598434418335676526_u128,251866744217586208164312178600711546838_u128];
_10 = 30313_u16 as isize;
place!(Field::<([u16; 8],)>(Variant(_13.fld1, 1), 2)).0 = Field::<[u16; 8]>(Variant(Field::<Adt38>(Variant(_15, 1), 2), 0), 6);
_20 = Adt44::Variant1 { fld0: _40,fld1: 3824_u16,fld2: 194439424186838114875397551349576051770_u128 };
_19 = [172585410808605629400475762653209362570_u128,170679429255869410099186232311205860946_u128];
place!(Field::<bool>(Variant(_13.fld1, 1), 0)) = (*_28);
place!(Field::<Adt46>(Variant(_15, 1), 5)).fld1 = [29306_u16];
_42 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_20, 1), 0)));
_19 = _41;
_34 = _9 as i128;
place!(Field::<*const *mut usize>(Variant(_15, 1), 1)) = core::ptr::addr_of!(place!(Field::<*mut usize>(Variant(_13.fld1, 1), 6)));
_24 = 8074324946706802485_i64 as u8;
place!(Field::<Adt46>(Variant(_15, 1), 5)).fld0 = 24526_u16 as usize;
SetDiscriminant(_4, 2);
place!(Field::<Adt46>(Variant(_15, 1), 5)) = _2;
place!(Field::<Adt46>(Variant(_15, 1), 5)).fld0 = _37;
place!(Field::<i8>(Variant(place!(Field::<Adt38>(Variant(_15, 1), 2)), 0), 3)) = 112_i8 & (-41_i8);
Goto(bb19)
}
bb19 = {
Call(_47 = dump_var(1_usize, 33_usize, Move(_33), 27_usize, Move(_27), 26_usize, Move(_26), 32_usize, Move(_32)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_47 = dump_var(1_usize, 14_usize, Move(_14), 7_usize, Move(_7), 41_usize, Move(_41), 30_usize, Move(_30)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_47 = dump_var(1_usize, 34_usize, Move(_34), 29_usize, Move(_29), 35_usize, Move(_35), 37_usize, Move(_37)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: bool,mut _2: u32,mut _3: u64,mut _4: Adt38,mut _5: u32,mut _6: usize,mut _7: Adt46,mut _8: Adt39,mut _9: Adt46,mut _10: Adt39,mut _11: u8,mut _12: Adt39,mut _13: Adt39) -> [u16; 1] {
mir! {
type RET = [u16; 1];
let _14: (i64, [i64; 1], [i64; 1], u16);
let _15: f64;
let _16: usize;
let _17: [u16; 6];
let _18: f32;
let _19: u16;
let _20: i64;
let _21: f64;
let _22: usize;
let _23: i32;
let _24: u32;
let _25: isize;
let _26: i8;
let _27: f64;
let _28: Adt50;
let _29: u128;
let _30: *const *mut f64;
let _31: u16;
let _32: bool;
let _33: [u16; 8];
let _34: [u128; 1];
let _35: [u16; 1];
let _36: i128;
let _37: i128;
let _38: [i64; 1];
let _39: Adt42;
let _40: u32;
let _41: ();
let _42: ();
{
_9.fld1 = [Field::<u16>(Variant(_4, 2), 4)];
_9.fld1 = Field::<[u16; 1]>(Variant(_12, 2), 0);
_1 = Field::<bool>(Variant(_4, 2), 0);
_10 = _9.fld2;
_7.fld2 = _13;
place!(Field::<[u16; 1]>(Variant(_12, 2), 0)) = [Field::<u16>(Variant(_4, 2), 4)];
_3 = !Field::<u64>(Variant(Field::<Adt37>(Variant(_4, 2), 1), 1), 1);
place!(Field::<[u16; 1]>(Variant(_10, 2), 0)) = [Field::<u16>(Variant(_4, 2), 4)];
place!(Field::<u32>(Variant(_8, 2), 2)) = Field::<u32>(Variant(_10, 2), 2);
_11 = !Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 2), 1), 1), 3);
place!(Field::<[u16; 1]>(Variant(_7.fld2, 2), 0)) = [Field::<u16>(Variant(_4, 2), 4)];
_9.fld4 = _7.fld4;
_14.0 = -(-9176586998883119632_i64);
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_4, 2), 1)), 1), 2)) = _11 as f64;
place!(Field::<i32>(Variant(_4, 2), 5)) = -1433885516_i32;
place!(Field::<i128>(Variant(_4, 2), 2)) = -44164692184389611344179276123358087092_i128;
place!(Field::<[u16; 1]>(Variant(_9.fld2, 2), 0)) = Field::<[u16; 1]>(Variant(_12, 2), 0);
_8 = _10;
place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_4, 2), 1)), 1), 0)) = Field::<i128>(Variant(_4, 2), 2) as usize;
place!(Field::<u32>(Variant(_7.fld2, 2), 2)) = (-9223372036854775808_isize) as u32;
_2 = Field::<u32>(Variant(_7.fld2, 2), 2) & Field::<u32>(Variant(_8, 2), 2);
RET = [Field::<u16>(Variant(_4, 2), 4)];
_9.fld4 = [183057002610231185263626797805686646849_u128,243380594084085884964902870866091234921_u128];
place!(Field::<i32>(Variant(_4, 2), 5)) = Field::<char>(Variant(_9.fld2, 2), 1) as i32;
place!(Field::<u16>(Variant(_4, 2), 4)) = Field::<bool>(Variant(_4, 2), 0) as u16;
place!(Field::<u32>(Variant(_9.fld2, 2), 2)) = Field::<u32>(Variant(_8, 2), 2);
Call(place!(Field::<Adt37>(Variant(_4, 2), 1)) = fn3(_7.fld2, _9.fld2, Field::<[u16; 1]>(Variant(_12, 2), 0), _7.fld2, _9.fld2, _10, RET, _7.fld4, _9.fld0, _12, _7, Field::<char>(Variant(_7.fld2, 2), 1), _7.fld0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<char>(Variant(_10, 2), 1)) = Field::<char>(Variant(_9.fld2, 2), 1);
place!(Field::<bool>(Variant(_4, 2), 0)) = _1;
_16 = _14.0 as usize;
place!(Field::<[u16; 1]>(Variant(_7.fld2, 2), 0)) = Field::<[u16; 1]>(Variant(_12, 2), 0);
_14.2 = [_14.0];
_18 = _11 as f32;
match Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 2), 1), 1), 3) {
0 => bb2,
1 => bb3,
149 => bb5,
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
place!(Field::<[u16; 1]>(Variant(_8, 2), 0)) = [Field::<u16>(Variant(_4, 2), 4)];
place!(Field::<char>(Variant(_9.fld2, 2), 1)) = Field::<char>(Variant(_12, 2), 1);
_7 = _9;
_7.fld0 = !Field::<usize>(Variant(Field::<Adt37>(Variant(_4, 2), 1), 1), 0);
_9.fld2 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_13, 2), 0),fld1: Field::<char>(Variant(_10, 2), 1),fld2: Field::<u32>(Variant(_10, 2), 2) };
_7 = _9;
_15 = -Field::<f64>(Variant(Field::<Adt37>(Variant(_4, 2), 1), 1), 2);
_17 = [Field::<u16>(Variant(_4, 2), 4),Field::<u16>(Variant(_4, 2), 4),Field::<u16>(Variant(_4, 2), 4),Field::<u16>(Variant(_4, 2), 4),Field::<u16>(Variant(_4, 2), 4),Field::<u16>(Variant(_4, 2), 4)];
RET = _7.fld1;
_10 = _8;
_9.fld2 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_10, 2), 0),fld1: Field::<char>(Variant(_12, 2), 1),fld2: Field::<u32>(Variant(_13, 2), 2) };
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_4, 2), 1)), 1), 2)) = _15;
place!(Field::<char>(Variant(_12, 2), 1)) = Field::<char>(Variant(_10, 2), 1);
SetDiscriminant(_10, 0);
place!(Field::<[u16; 1]>(Variant(_10, 0), 3)) = Field::<[u16; 1]>(Variant(_9.fld2, 2), 0);
_15 = Field::<f64>(Variant(Field::<Adt37>(Variant(_4, 2), 1), 1), 2);
_6 = Field::<usize>(Variant(Field::<Adt37>(Variant(_4, 2), 1), 1), 0);
place!(Field::<char>(Variant(_8, 2), 1)) = Field::<char>(Variant(_13, 2), 1);
_4 = Adt38::Variant3 { fld0: _14.0,fld1: 41336_u16 };
_14.2 = [Field::<i64>(Variant(_4, 3), 0)];
_5 = (-132684641142054111055881960284522359022_i128) as u32;
_5 = !_2;
_24 = Field::<u32>(Variant(_8, 2), 2);
Call(place!(Field::<[u16; 1]>(Variant(_7.fld2, 2), 0)) = core::intrinsics::transmute(Field::<[u16; 1]>(Variant(_13, 2), 0)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19 = 26350_u16 >> _6;
_14.1 = [_14.0];
_22 = !_6;
_21 = _15 - _15;
_7.fld0 = _9.fld0;
_1 = _19 < _19;
place!(Field::<char>(Variant(_9.fld2, 2), 1)) = Field::<char>(Variant(_13, 2), 1);
place!(Field::<u32>(Variant(_8, 2), 2)) = !Field::<u32>(Variant(_7.fld2, 2), 2);
_21 = (-98584125_i32) as f64;
place!(Field::<([u16; 8],)>(Variant(_10, 0), 2)).0 = [_19,_19,_19,_19,_19,_19,_19,_19];
_6 = !_22;
place!(Field::<char>(Variant(_13, 1), 1)) = Field::<char>(Variant(_8, 2), 1);
SetDiscriminant(_8, 0);
SetDiscriminant(_12, 2);
place!(Field::<u32>(Variant(_12, 2), 2)) = 9223372036854775807_isize as u32;
_21 = _15 - _15;
place!(Field::<u32>(Variant(_12, 2), 2)) = _24 + _2;
_22 = _6;
_20 = Field::<i64>(Variant(_4, 3), 0) - Field::<i64>(Variant(_4, 3), 0);
_22 = !_6;
_9 = _7;
_14.3 = !_19;
_14.1 = [Field::<i64>(Variant(_4, 3), 0)];
_25 = (-99_isize);
place!(Field::<u16>(Variant(_4, 3), 1)) = _14.3 | _19;
place!(Field::<bool>(Variant(_13, 1), 0)) = _19 == Field::<u16>(Variant(_4, 3), 1);
place!(Field::<[u128; 2]>(Variant(_10, 0), 0)) = [160678015098303237400626228312929008636_u128,186769840876639621685345390989601848305_u128];
match _25 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211357 => bb7,
_ => bb5
}
}
bb7 = {
_1 = _21 > _21;
place!(Field::<*mut usize>(Variant(_13, 1), 6)) = core::ptr::addr_of_mut!(_7.fld0);
_6 = !_22;
place!(Field::<([u16; 8],)>(Variant(_13, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_10, 0), 2).0;
place!(Field::<[u32; 8]>(Variant(_13, 1), 5)) = [_5,Field::<u32>(Variant(_12, 2), 2),_5,_2,Field::<u32>(Variant(_12, 2), 2),Field::<u32>(Variant(_9.fld2, 2), 2),Field::<u32>(Variant(_12, 2), 2),_5];
_12 = _9.fld2;
_13 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_7.fld2, 2), 0),fld1: Field::<char>(Variant(_12, 2), 1),fld2: _5 };
_26 = _25 as i8;
place!(Field::<[u128; 2]>(Variant(_8, 0), 0)) = [108643341989853358283708559492824435765_u128,266440292178423986508205877130139985901_u128];
_29 = 131485403395980007062993691239260333245_u128;
_9.fld4 = [_29,_29];
_7.fld0 = (-167697627330480145695927975503022441186_i128) as usize;
_22 = _6;
Goto(bb8)
}
bb8 = {
_9.fld4 = _7.fld4;
_25 = 56_isize;
_35 = [_19];
_27 = _21;
_33 = [_19,_14.3,Field::<u16>(Variant(_4, 3), 1),_14.3,Field::<u16>(Variant(_4, 3), 1),Field::<u16>(Variant(_4, 3), 1),Field::<u16>(Variant(_4, 3), 1),_14.3];
_34 = [_29];
_31 = _14.3 << _6;
_20 = Field::<i64>(Variant(_4, 3), 0) | _14.0;
_5 = Field::<u32>(Variant(_9.fld2, 2), 2);
_20 = !_14.0;
place!(Field::<u32>(Variant(_12, 2), 2)) = _24 * Field::<u32>(Variant(_13, 2), 2);
_8 = _9.fld2;
_28.fld1.fld1 = _8;
_9.fld1 = [Field::<u16>(Variant(_4, 3), 1)];
_28.fld2 = Adt43::Variant0 { fld0: _14.1,fld1: Field::<char>(Variant(_12, 2), 1) };
_32 = _1;
_32 = _1;
match _25 {
0 => bb9,
1 => bb10,
56 => bb12,
_ => bb11
}
}
bb9 = {
_1 = _21 > _21;
place!(Field::<*mut usize>(Variant(_13, 1), 6)) = core::ptr::addr_of_mut!(_7.fld0);
_6 = !_22;
place!(Field::<([u16; 8],)>(Variant(_13, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_10, 0), 2).0;
place!(Field::<[u32; 8]>(Variant(_13, 1), 5)) = [_5,Field::<u32>(Variant(_12, 2), 2),_5,_2,Field::<u32>(Variant(_12, 2), 2),Field::<u32>(Variant(_9.fld2, 2), 2),Field::<u32>(Variant(_12, 2), 2),_5];
_12 = _9.fld2;
_13 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_7.fld2, 2), 0),fld1: Field::<char>(Variant(_12, 2), 1),fld2: _5 };
_26 = _25 as i8;
place!(Field::<[u128; 2]>(Variant(_8, 0), 0)) = [108643341989853358283708559492824435765_u128,266440292178423986508205877130139985901_u128];
_29 = 131485403395980007062993691239260333245_u128;
_9.fld4 = [_29,_29];
_7.fld0 = (-167697627330480145695927975503022441186_i128) as usize;
_22 = _6;
Goto(bb8)
}
bb10 = {
place!(Field::<char>(Variant(_10, 2), 1)) = Field::<char>(Variant(_9.fld2, 2), 1);
place!(Field::<bool>(Variant(_4, 2), 0)) = _1;
_16 = _14.0 as usize;
place!(Field::<[u16; 1]>(Variant(_7.fld2, 2), 0)) = Field::<[u16; 1]>(Variant(_12, 2), 0);
_14.2 = [_14.0];
_18 = _11 as f32;
match Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 2), 1), 1), 3) {
0 => bb2,
1 => bb3,
149 => bb5,
_ => bb4
}
}
bb11 = {
Return()
}
bb12 = {
place!(Field::<[u16; 1]>(Variant(_8, 2), 0)) = Field::<[u16; 1]>(Variant(_10, 0), 3);
_9.fld2 = _8;
_9.fld0 = _6 & _6;
_19 = !Field::<u16>(Variant(_4, 3), 1);
_3 = !16613642073406686243_u64;
_3 = !6899807522226054657_u64;
_8 = _7.fld2;
_26 = _2 as i8;
_27 = -_21;
_37 = (-148695245608503625960380561772788301168_i128);
place!(Field::<[u16; 1]>(Variant(_10, 0), 3)) = [Field::<u16>(Variant(_4, 3), 1)];
place!(Field::<[u16; 1]>(Variant(_9.fld2, 2), 0)) = Field::<[u16; 1]>(Variant(_10, 0), 3);
_28.fld3 = _3;
RET = [Field::<u16>(Variant(_4, 3), 1)];
_17 = [_31,_14.3,_19,_19,_31,Field::<u16>(Variant(_4, 3), 1)];
_22 = _9.fld0;
_16 = _6;
_13 = _8;
_36 = _37 & _37;
_14.2 = [Field::<i64>(Variant(_4, 3), 0)];
place!(Field::<u32>(Variant(_13, 2), 2)) = _11 as u32;
_17 = [_31,_31,_19,_19,_14.3,_14.3];
place!(Field::<u32>(Variant(_13, 2), 2)) = _2;
_29 = 65474761601118433532716933666928203547_u128;
Goto(bb13)
}
bb13 = {
Call(_41 = dump_var(2_usize, 37_usize, Move(_37), 31_usize, Move(_31), 20_usize, Move(_20), 17_usize, Move(_17)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_41 = dump_var(2_usize, 29_usize, Move(_29), 33_usize, Move(_33), 36_usize, Move(_36), 1_usize, Move(_1)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_41 = dump_var(2_usize, 2_usize, Move(_2), 11_usize, Move(_11), 32_usize, Move(_32), 42_usize, _42), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: Adt39,mut _2: Adt39,mut _3: [u16; 1],mut _4: Adt39,mut _5: Adt39,mut _6: Adt39,mut _7: [u16; 1],mut _8: [u128; 2],mut _9: usize,mut _10: Adt39,mut _11: Adt46,mut _12: char,mut _13: usize) -> Adt37 {
mir! {
type RET = Adt37;
let _14: [u16; 8];
let _15: isize;
let _16: i32;
let _17: i8;
let _18: i64;
let _19: isize;
let _20: *mut [u8; 3];
let _21: isize;
let _22: *const *mut usize;
let _23: ([u16; 8],);
let _24: i8;
let _25: usize;
let _26: u128;
let _27: [u32; 8];
let _28: *const bool;
let _29: i32;
let _30: [u16; 8];
let _31: (*const *mut f64,);
let _32: f64;
let _33: [u16; 1];
let _34: bool;
let _35: u8;
let _36: Adt49;
let _37: Adt53;
let _38: i8;
let _39: f64;
let _40: f32;
let _41: char;
let _42: usize;
let _43: f64;
let _44: [u128; 1];
let _45: Adt37;
let _46: isize;
let _47: f64;
let _48: ();
let _49: ();
{
_9 = _11.fld0;
SetDiscriminant(_6, 2);
Call(place!(Field::<u32>(Variant(_2, 2), 2)) = fn4(_9, _5, _10, _4, _1, _11.fld2, _11, _11, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _4;
_1 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_5, 2), 0),fld1: _12,fld2: Field::<u32>(Variant(_10, 2), 2) };
place!(Field::<u32>(Variant(_2, 2), 2)) = _11.fld0 as u32;
place!(Field::<u32>(Variant(_1, 2), 2)) = Field::<u32>(Variant(_2, 2), 2) - Field::<u32>(Variant(_2, 2), 2);
_14 = [53493_u16,57619_u16,21470_u16,38189_u16,6924_u16,50927_u16,27087_u16,3865_u16];
place!(Field::<u32>(Variant(_1, 2), 2)) = !Field::<u32>(Variant(_2, 2), 2);
place!(Field::<[u16; 1]>(Variant(_5, 2), 0)) = [14773_u16];
place!(Field::<[u16; 1]>(Variant(_5, 2), 0)) = Field::<[u16; 1]>(Variant(_4, 2), 0);
place!(Field::<[u16; 1]>(Variant(_11.fld2, 2), 0)) = Field::<[u16; 1]>(Variant(_10, 2), 0);
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_6, 2), 1);
Goto(bb2)
}
bb2 = {
_4 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_11.fld2, 2), 0),fld1: Field::<char>(Variant(_6, 2), 1),fld2: Field::<u32>(Variant(_10, 2), 2) };
_11.fld1 = Field::<[u16; 1]>(Variant(_4, 2), 0);
_1 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_5, 2), 0),fld1: Field::<char>(Variant(_2, 2), 1),fld2: Field::<u32>(Variant(_10, 2), 2) };
SetDiscriminant(_6, 0);
_11.fld4 = [35544215510110360269703779477394246518_u128,139086326826137337829320034795751033201_u128];
_6 = _5;
_7 = [7844_u16];
_18 = !(-6996921928219047631_i64);
place!(Field::<u32>(Variant(_4, 2), 2)) = Field::<u32>(Variant(_2, 2), 2) - Field::<u32>(Variant(_2, 2), 2);
_16 = 301675701_i32;
SetDiscriminant(_1, 3);
Goto(bb3)
}
bb3 = {
_5 = _2;
_2 = _11.fld2;
_15 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_12 = Field::<char>(Variant(_10, 2), 1);
place!(Field::<char>(Variant(_6, 2), 1)) = _12;
_11.fld0 = 96_u8 as usize;
_15 = !9223372036854775807_isize;
_15 = (-125_isize) + (-106_isize);
_17 = 85_i8;
_8 = [338660327511728145142796021746431456034_u128,327838334386500795090506703264740175421_u128];
place!(Field::<u32>(Variant(_11.fld2, 2), 2)) = Field::<u32>(Variant(_6, 2), 2);
place!(Field::<[u16; 1]>(Variant(_2, 2), 0)) = [3079_u16];
_15 = (-67_isize);
_5 = _4;
place!(Field::<u32>(Variant(_6, 2), 2)) = Field::<u32>(Variant(_5, 2), 2) - Field::<u32>(Variant(_2, 2), 2);
_1 = _2;
_3 = Field::<[u16; 1]>(Variant(_1, 2), 0);
place!(Field::<u32>(Variant(_2, 2), 2)) = Field::<u32>(Variant(_4, 2), 2) * Field::<u32>(Variant(_5, 2), 2);
_12 = Field::<char>(Variant(_11.fld2, 2), 1);
_6 = _4;
match _15 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607431768211389 => bb9,
_ => bb8
}
}
bb4 = {
_4 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_11.fld2, 2), 0),fld1: Field::<char>(Variant(_6, 2), 1),fld2: Field::<u32>(Variant(_10, 2), 2) };
_11.fld1 = Field::<[u16; 1]>(Variant(_4, 2), 0);
_1 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_5, 2), 0),fld1: Field::<char>(Variant(_2, 2), 1),fld2: Field::<u32>(Variant(_10, 2), 2) };
SetDiscriminant(_6, 0);
_11.fld4 = [35544215510110360269703779477394246518_u128,139086326826137337829320034795751033201_u128];
_6 = _5;
_7 = [7844_u16];
_18 = !(-6996921928219047631_i64);
place!(Field::<u32>(Variant(_4, 2), 2)) = Field::<u32>(Variant(_2, 2), 2) - Field::<u32>(Variant(_2, 2), 2);
_16 = 301675701_i32;
SetDiscriminant(_1, 3);
Goto(bb3)
}
bb5 = {
_6 = _4;
_1 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_5, 2), 0),fld1: _12,fld2: Field::<u32>(Variant(_10, 2), 2) };
place!(Field::<u32>(Variant(_2, 2), 2)) = _11.fld0 as u32;
place!(Field::<u32>(Variant(_1, 2), 2)) = Field::<u32>(Variant(_2, 2), 2) - Field::<u32>(Variant(_2, 2), 2);
_14 = [53493_u16,57619_u16,21470_u16,38189_u16,6924_u16,50927_u16,27087_u16,3865_u16];
place!(Field::<u32>(Variant(_1, 2), 2)) = !Field::<u32>(Variant(_2, 2), 2);
place!(Field::<[u16; 1]>(Variant(_5, 2), 0)) = [14773_u16];
place!(Field::<[u16; 1]>(Variant(_5, 2), 0)) = Field::<[u16; 1]>(Variant(_4, 2), 0);
place!(Field::<[u16; 1]>(Variant(_11.fld2, 2), 0)) = Field::<[u16; 1]>(Variant(_10, 2), 0);
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_6, 2), 1);
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
_11.fld4 = [15478008540061116366398782762873042163_u128,56883502954594078091317520304568941715_u128];
_17 = 91_i8 * (-42_i8);
_1 = _2;
_23.0 = _14;
_2 = _1;
_10 = _2;
_6 = _2;
_23.0 = [53067_u16,21272_u16,36315_u16,19785_u16,20068_u16,18161_u16,49521_u16,6767_u16];
_19 = _15;
SetDiscriminant(_10, 3);
_11.fld1 = [63548_u16];
_24 = _17;
place!(Field::<[u16; 1]>(Variant(_4, 2), 0)) = [23348_u16];
_11.fld4 = _8;
_26 = 13193615480724470852406651172320633606_u128 & 12122863386712142924164440057124604651_u128;
match _19 {
340282366920938463463374607431768211389 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_2, 2), 1);
_15 = false as isize;
_21 = -_19;
place!(Field::<[u16; 1]>(Variant(_2, 2), 0)) = [51948_u16];
SetDiscriminant(_2, 0);
_18 = 6083654317413103657_i64 & 8314610407064367875_i64;
place!(Field::<[u16; 1]>(Variant(_11.fld2, 2), 0)) = [16561_u16];
_2 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_5, 2), 0),fld1: Field::<char>(Variant(_1, 2), 1),fld2: Field::<u32>(Variant(_11.fld2, 2), 2) };
_18 = (-7640576207495801363_i64);
_4 = _6;
_5 = _6;
_5 = _4;
place!(Field::<u32>(Variant(_5, 2), 2)) = 57884_u16 as u32;
place!(Field::<u32>(Variant(_5, 2), 2)) = !Field::<u32>(Variant(_1, 2), 2);
_10 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_4, 2), 0),fld1: _12,fld2: Field::<u32>(Variant(_6, 2), 2) };
_18 = (-4657035605632196522_i64);
SetDiscriminant(_2, 2);
_23.0 = _14;
place!(Field::<char>(Variant(_6, 2), 1)) = Field::<char>(Variant(_4, 2), 1);
place!(Field::<u32>(Variant(_4, 2), 2)) = 14322485501603130693_u64 as u32;
_14 = _23.0;
_11.fld4 = [_26,_26];
_17 = _24;
_23.0 = [60420_u16,20426_u16,13769_u16,35994_u16,47488_u16,65329_u16,17150_u16,63481_u16];
_7 = [25214_u16];
Call(_26 = core::intrinsics::transmute(_14), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<char>(Variant(_4, 2), 1)) = _12;
_7 = _3;
_4 = _6;
place!(Field::<char>(Variant(_1, 2), 1)) = Field::<char>(Variant(_5, 2), 1);
_11.fld1 = [55535_u16];
_11.fld4 = _8;
place!(Field::<char>(Variant(_10, 2), 1)) = Field::<char>(Variant(_11.fld2, 2), 1);
_10 = _5;
_29 = _16 | _16;
_7 = Field::<[u16; 1]>(Variant(_4, 2), 0);
_9 = _13;
_32 = 31290520673374910422943461190617199995_i128 as f64;
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_1, 2), 1);
place!(Field::<[u16; 1]>(Variant(_10, 2), 0)) = [13871_u16];
place!(Field::<char>(Variant(_1, 2), 1)) = Field::<char>(Variant(_6, 2), 1);
place!(Field::<[u16; 1]>(Variant(_2, 2), 0)) = [41879_u16];
place!(Field::<[u16; 1]>(Variant(_1, 2), 0)) = [1152_u16];
_1 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_2, 2), 0),fld1: Field::<char>(Variant(_4, 2), 1),fld2: Field::<u32>(Variant(_6, 2), 2) };
_2 = Adt39::Variant2 { fld0: _3,fld1: Field::<char>(Variant(_4, 2), 1),fld2: Field::<u32>(Variant(_5, 2), 2) };
_32 = _24 as f64;
_36.fld5 = _9;
_2 = _4;
_17 = _24 & _24;
place!(Field::<u32>(Variant(_5, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
SetDiscriminant(_1, 0);
place!(Field::<[u16; 1]>(Variant(_1, 0), 3)) = [62609_u16];
Goto(bb13)
}
bb13 = {
place!(Field::<[u16; 1]>(Variant(_1, 0), 3)) = [54552_u16];
place!(Field::<char>(Variant(_4, 2), 1)) = _12;
_36.fld0 = -_32;
_11.fld1 = [18006_u16];
_35 = 149_u8;
_8 = [_26,_26];
_26 = 222935313484380529220337254844324958606_u128;
_18 = !8488664597009751982_i64;
_30 = [524_u16,31668_u16,3277_u16,8848_u16,51232_u16,896_u16,38869_u16,5887_u16];
_28 = core::ptr::addr_of!(_34);
_41 = Field::<char>(Variant(_4, 2), 1);
_22 = core::ptr::addr_of!(_36.fld4);
place!(Field::<[u16; 1]>(Variant(_4, 2), 0)) = [4123_u16];
place!(Field::<[u16; 1]>(Variant(_11.fld2, 2), 0)) = Field::<[u16; 1]>(Variant(_5, 2), 0);
place!(Field::<[u16; 1]>(Variant(_11.fld2, 2), 0)) = [27375_u16];
_32 = _26 as f64;
_38 = _35 as i8;
_11.fld4 = [_26,_26];
_5 = _11.fld2;
_9 = !_13;
_11.fld1 = [49435_u16];
_42 = !_36.fld5;
_2 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_10, 2), 0),fld1: Field::<char>(Variant(_10, 2), 1),fld2: Field::<u32>(Variant(_6, 2), 2) };
Goto(bb14)
}
bb14 = {
_27 = [Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_6, 2), 2),Field::<u32>(Variant(_6, 2), 2),Field::<u32>(Variant(_6, 2), 2),Field::<u32>(Variant(_10, 2), 2),Field::<u32>(Variant(_10, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_6, 2), 2)];
_26 = Field::<char>(Variant(_6, 2), 1) as u128;
_36.fld6 = Field::<u32>(Variant(_4, 2), 2) >> Field::<u32>(Variant(_6, 2), 2);
_1 = _6;
place!(Field::<[u16; 1]>(Variant(_6, 2), 0)) = [6052_u16];
place!(Field::<[u16; 1]>(Variant(_10, 2), 0)) = _3;
_9 = _42 * _13;
_23 = (_14,);
_42 = _9;
RET = Adt37::Variant1 { fld0: _42,fld1: 838308552872852358_u64,fld2: _36.fld0,fld3: _35 };
place!(Field::<u64>(Variant(RET, 1), 1)) = !3179499761933350537_u64;
_6 = _10;
_9 = _36.fld5 ^ _36.fld5;
_11.fld2 = _10;
_24 = _38 * _38;
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_11.fld2, 2), 1);
_44 = [_26];
place!(Field::<f64>(Variant(RET, 1), 2)) = _36.fld0;
place!(Field::<char>(Variant(_4, 2), 1)) = Field::<char>(Variant(_2, 2), 1);
_43 = -Field::<f64>(Variant(RET, 1), 2);
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(3_usize, 7_usize, Move(_7), 12_usize, Move(_12), 30_usize, Move(_30), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(3_usize, 14_usize, Move(_14), 3_usize, Move(_3), 16_usize, Move(_16), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(3_usize, 13_usize, Move(_13), 41_usize, Move(_41), 38_usize, Move(_38), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: usize,mut _2: Adt39,mut _3: Adt39,mut _4: Adt39,mut _5: Adt39,mut _6: Adt39,mut _7: Adt46,mut _8: Adt46,mut _9: Adt39) -> u32 {
mir! {
type RET = u32;
let _10: u32;
let _11: isize;
let _12: (i64, [i64; 1], [i64; 1], u16);
let _13: isize;
let _14: ([u16; 8],);
let _15: usize;
let _16: (*const *mut f64,);
let _17: Adt44;
let _18: isize;
let _19: f64;
let _20: *const bool;
let _21: &'static [u16; 1];
let _22: [u16; 8];
let _23: u16;
let _24: [u128; 8];
let _25: Adt44;
let _26: f64;
let _27: [u128; 1];
let _28: f64;
let _29: i16;
let _30: ();
let _31: ();
{
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_4, 2), 1);
_9 = _3;
place!(Field::<[u16; 1]>(Variant(_5, 2), 0)) = Field::<[u16; 1]>(Variant(_7.fld2, 2), 0);
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_6, 2), 1);
place!(Field::<u32>(Variant(_9, 2), 2)) = 630385826370982737_u64 as u32;
_1 = 742_u16 as usize;
_8 = _7;
SetDiscriminant(_4, 0);
place!(Field::<char>(Variant(_3, 2), 1)) = Field::<char>(Variant(_2, 2), 1);
_12.1 = [(-8496895701440691167_i64)];
place!(Field::<u32>(Variant(_5, 2), 2)) = Field::<u32>(Variant(_7.fld2, 2), 2) >> _8.fld0;
place!(Field::<char>(Variant(_2, 2), 1)) = Field::<char>(Variant(_9, 2), 1);
_8.fld4 = _7.fld4;
_6 = _9;
_4 = _7.fld2;
_7.fld2 = _4;
_7.fld1 = [12543_u16];
_8 = _7;
Call(_14 = fn5(_7.fld0, _6, _7.fld2, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<char>(Variant(_6, 2), 1)) = Field::<char>(Variant(_5, 2), 1);
_7.fld2 = _4;
RET = (-33_isize) as u32;
place!(Field::<char>(Variant(_3, 2), 1)) = Field::<char>(Variant(_4, 2), 1);
place!(Field::<char>(Variant(_4, 2), 1)) = Field::<char>(Variant(_6, 2), 1);
_2 = _6;
_12.2 = _12.1;
place!(Field::<[u16; 1]>(Variant(_2, 2), 0)) = [5744_u16];
place!(Field::<[u16; 1]>(Variant(_6, 2), 0)) = [43041_u16];
place!(Field::<u32>(Variant(_9, 2), 2)) = 210941187178061458258259424998774349326_u128 as u32;
_1 = _8.fld0 + _8.fld0;
_8.fld0 = !_1;
place!(Field::<u32>(Variant(_3, 2), 2)) = (-9223372036854775808_isize) as u32;
_6 = _9;
_8.fld1 = Field::<[u16; 1]>(Variant(_5, 2), 0);
Goto(bb2)
}
bb2 = {
place!(Field::<u32>(Variant(_5, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [16893_u16];
place!(Field::<u32>(Variant(_7.fld2, 2), 2)) = RET - Field::<u32>(Variant(_9, 2), 2);
_14.0 = [3272_u16,39249_u16,6236_u16,4185_u16,40554_u16,12836_u16,49241_u16,47218_u16];
_11 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
place!(Field::<[u16; 1]>(Variant(_7.fld2, 2), 0)) = [2014_u16];
_12.0 = (-4996879469700088558_i64);
_4 = _7.fld2;
_10 = Field::<u32>(Variant(_7.fld2, 2), 2);
place!(Field::<[u16; 1]>(Variant(_6, 2), 0)) = [41270_u16];
RET = Field::<u32>(Variant(_7.fld2, 2), 2) >> _8.fld0;
place!(Field::<u32>(Variant(_2, 2), 2)) = Field::<u32>(Variant(_4, 2), 2) + RET;
place!(Field::<[u16; 1]>(Variant(_6, 2), 0)) = [28017_u16];
_7 = _8;
place!(Field::<char>(Variant(_2, 2), 1)) = Field::<char>(Variant(_5, 2), 1);
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_9, 2), 1);
SetDiscriminant(_9, 2);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = Field::<[u16; 1]>(Variant(_8.fld2, 2), 0);
_9 = _2;
RET = !Field::<u32>(Variant(_7.fld2, 2), 2);
SetDiscriminant(_8.fld2, 3);
match _12.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463458377727962068122898 => bb8,
_ => bb7
}
}
bb3 = {
place!(Field::<char>(Variant(_6, 2), 1)) = Field::<char>(Variant(_5, 2), 1);
_7.fld2 = _4;
RET = (-33_isize) as u32;
place!(Field::<char>(Variant(_3, 2), 1)) = Field::<char>(Variant(_4, 2), 1);
place!(Field::<char>(Variant(_4, 2), 1)) = Field::<char>(Variant(_6, 2), 1);
SetDiscriminant(_2, 1);
_2 = _6;
_12.2 = _12.1;
place!(Field::<[u16; 1]>(Variant(_2, 2), 0)) = [5744_u16];
place!(Field::<[u16; 1]>(Variant(_6, 2), 0)) = [43041_u16];
place!(Field::<u32>(Variant(_9, 2), 2)) = 210941187178061458258259424998774349326_u128 as u32;
_1 = _8.fld0 + _8.fld0;
_8.fld0 = !_1;
place!(Field::<u32>(Variant(_3, 2), 2)) = (-9223372036854775808_isize) as u32;
_6 = _9;
_8.fld1 = Field::<[u16; 1]>(Variant(_5, 2), 0);
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
_12.2 = [_12.0];
place!(Field::<u32>(Variant(_7.fld2, 2), 2)) = Field::<u32>(Variant(_9, 2), 2) * Field::<u32>(Variant(_2, 2), 2);
_12.2 = [_12.0];
place!(Field::<[u16; 1]>(Variant(_2, 2), 0)) = [64592_u16];
_19 = _8.fld0 as f64;
_1 = !_8.fld0;
_3 = Adt39::Variant2 { fld0: _7.fld1,fld1: Field::<char>(Variant(_5, 2), 1),fld2: Field::<u32>(Variant(_7.fld2, 2), 2) };
_11 = (-9223372036854775808_isize) & 14_isize;
_8.fld0 = _1;
_5 = _4;
_12.3 = 37523_u16;
_18 = -_11;
place!(Field::<Adt37>(Variant(_8.fld2, 3), 1)) = Adt37::Variant1 { fld0: _7.fld0,fld1: 15267055289801380579_u64,fld2: _19,fld3: 57_u8 };
_10 = Field::<u32>(Variant(_7.fld2, 2), 2) ^ Field::<u32>(Variant(_9, 2), 2);
place!(Field::<u128>(Variant(_8.fld2, 3), 2)) = _12.3 as u128;
_12.3 = 46034_u16 - 19102_u16;
place!(Field::<u32>(Variant(_2, 2), 2)) = !Field::<u32>(Variant(_3, 2), 2);
_8.fld0 = _7.fld0;
SetDiscriminant(_9, 0);
_24 = [Field::<u128>(Variant(_8.fld2, 3), 2),Field::<u128>(Variant(_8.fld2, 3), 2),Field::<u128>(Variant(_8.fld2, 3), 2),Field::<u128>(Variant(_8.fld2, 3), 2),Field::<u128>(Variant(_8.fld2, 3), 2),Field::<u128>(Variant(_8.fld2, 3), 2),Field::<u128>(Variant(_8.fld2, 3), 2),Field::<u128>(Variant(_8.fld2, 3), 2)];
match _12.0 {
0 => bb6,
1 => bb7,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
340282366920938463458377727962068122898 => bb14,
_ => bb13
}
}
bb9 = {
Return()
}
bb10 = {
place!(Field::<u32>(Variant(_5, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [16893_u16];
place!(Field::<u32>(Variant(_7.fld2, 2), 2)) = RET - Field::<u32>(Variant(_9, 2), 2);
_14.0 = [3272_u16,39249_u16,6236_u16,4185_u16,40554_u16,12836_u16,49241_u16,47218_u16];
_11 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
place!(Field::<[u16; 1]>(Variant(_7.fld2, 2), 0)) = [2014_u16];
_12.0 = (-4996879469700088558_i64);
_4 = _7.fld2;
_10 = Field::<u32>(Variant(_7.fld2, 2), 2);
place!(Field::<[u16; 1]>(Variant(_6, 2), 0)) = [41270_u16];
RET = Field::<u32>(Variant(_7.fld2, 2), 2) >> _8.fld0;
place!(Field::<u32>(Variant(_2, 2), 2)) = Field::<u32>(Variant(_4, 2), 2) + RET;
place!(Field::<[u16; 1]>(Variant(_6, 2), 0)) = [28017_u16];
_7 = _8;
place!(Field::<char>(Variant(_2, 2), 1)) = Field::<char>(Variant(_5, 2), 1);
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_9, 2), 1);
SetDiscriminant(_9, 2);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = Field::<[u16; 1]>(Variant(_8.fld2, 2), 0);
_9 = _2;
RET = !Field::<u32>(Variant(_7.fld2, 2), 2);
SetDiscriminant(_8.fld2, 3);
match _12.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463458377727962068122898 => bb8,
_ => bb7
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
place!(Field::<char>(Variant(_6, 2), 1)) = Field::<char>(Variant(_5, 2), 1);
_7.fld2 = _4;
RET = (-33_isize) as u32;
place!(Field::<char>(Variant(_3, 2), 1)) = Field::<char>(Variant(_4, 2), 1);
place!(Field::<char>(Variant(_4, 2), 1)) = Field::<char>(Variant(_6, 2), 1);
SetDiscriminant(_2, 1);
_2 = _6;
_12.2 = _12.1;
place!(Field::<[u16; 1]>(Variant(_2, 2), 0)) = [5744_u16];
place!(Field::<[u16; 1]>(Variant(_6, 2), 0)) = [43041_u16];
place!(Field::<u32>(Variant(_9, 2), 2)) = 210941187178061458258259424998774349326_u128 as u32;
_1 = _8.fld0 + _8.fld0;
_8.fld0 = !_1;
place!(Field::<u32>(Variant(_3, 2), 2)) = (-9223372036854775808_isize) as u32;
_6 = _9;
_8.fld1 = Field::<[u16; 1]>(Variant(_5, 2), 0);
Goto(bb2)
}
bb14 = {
_12.0 = !(-9169563226201733596_i64);
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_8.fld2, 3), 1)), 1), 3)) = 188_u8;
_1 = _8.fld0 - Field::<usize>(Variant(Field::<Adt37>(Variant(_8.fld2, 3), 1), 1), 0);
_9 = _7.fld2;
_23 = _12.3 - _12.3;
RET = Field::<u32>(Variant(_3, 2), 2);
place!(Field::<char>(Variant(_6, 2), 1)) = Field::<char>(Variant(_5, 2), 1);
_7.fld0 = Field::<usize>(Variant(Field::<Adt37>(Variant(_8.fld2, 3), 1), 1), 0) & _1;
_2 = _5;
place!(Field::<[u16; 1]>(Variant(_7.fld2, 2), 0)) = [_23];
_8.fld4 = _7.fld4;
place!(Field::<char>(Variant(_6, 2), 1)) = Field::<char>(Variant(_9, 2), 1);
place!(Field::<char>(Variant(_5, 2), 1)) = Field::<char>(Variant(_7.fld2, 2), 1);
_14.0 = [_12.3,_23,_23,_23,_12.3,_12.3,_23,_12.3];
_10 = Field::<u32>(Variant(_6, 2), 2);
_8 = _7;
_28 = _19;
_13 = -_18;
_14.0 = [_23,_12.3,_12.3,_12.3,_12.3,_23,_12.3,_12.3];
place!(Field::<[u16; 1]>(Variant(_4, 2), 0)) = _7.fld1;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(4_usize, 1_usize, Move(_1), 14_usize, Move(_14), 10_usize, Move(_10), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: usize,mut _2: Adt39,mut _3: Adt39,mut _4: Adt39) -> ([u16; 8],) {
mir! {
type RET = ([u16; 8],);
let _5: i128;
let _6: ([u16; 8],);
let _7: u128;
let _8: [u16; 8];
let _9: (i64, [i64; 1], [i64; 1], u16);
let _10: [u16; 1];
let _11: Adt42;
let _12: u16;
let _13: char;
let _14: bool;
let _15: u16;
let _16: Adt51;
let _17: [u32; 8];
let _18: f32;
let _19: [u8; 3];
let _20: Adt40;
let _21: bool;
let _22: u8;
let _23: bool;
let _24: u8;
let _25: [u16; 6];
let _26: *mut usize;
let _27: bool;
let _28: isize;
let _29: i128;
let _30: [u16; 8];
let _31: u8;
let _32: i8;
let _33: [u128; 1];
let _34: [u16; 1];
let _35: (i64, [i64; 1], [i64; 1], u16);
let _36: u32;
let _37: i64;
let _38: ();
let _39: ();
{
place!(Field::<u32>(Variant(_3, 2), 2)) = !Field::<u32>(Variant(_2, 2), 2);
place!(Field::<[u16; 1]>(Variant(_3, 2), 0)) = [51333_u16];
place!(Field::<u32>(Variant(_4, 2), 2)) = Field::<u32>(Variant(_3, 2), 2) << Field::<u32>(Variant(_2, 2), 2);
RET.0 = [37656_u16,3665_u16,41532_u16,10948_u16,23512_u16,42022_u16,32576_u16,49373_u16];
_4 = _3;
place!(Field::<u32>(Variant(_3, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
place!(Field::<u32>(Variant(_4, 2), 2)) = _1 as u32;
place!(Field::<char>(Variant(_3, 2), 1)) = Field::<char>(Variant(_2, 2), 1);
_6 = (RET.0,);
RET.0 = [50321_u16,51111_u16,42948_u16,55351_u16,62300_u16,12269_u16,20228_u16,16793_u16];
place!(Field::<u32>(Variant(_3, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
_6.0 = [1886_u16,58155_u16,9931_u16,62658_u16,44751_u16,65479_u16,5310_u16,19166_u16];
RET = (_6.0,);
_1 = (-9223372036854775808_isize) as usize;
_5 = !(-101082525827683588644234300644073999214_i128);
Goto(bb1)
}
bb1 = {
_1 = !1_usize;
_5 = !(-159754184149520304549595059111060375694_i128);
RET.0 = [34974_u16,63965_u16,3742_u16,56257_u16,27262_u16,59769_u16,22502_u16,31095_u16];
_6 = (RET.0,);
_5 = 93867354_i32 as i128;
place!(Field::<[u32; 8]>(Variant(_3, 1), 5)) = [Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_4, 2), 2),Field::<u32>(Variant(_4, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_4, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_4, 2), 2)];
place!(Field::<u32>(Variant(_2, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
_2 = _4;
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)) = _6;
place!(Field::<[u32; 8]>(Variant(_3, 1), 5)) = [Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2)];
place!(Field::<[u32; 8]>(Variant(_3, 1), 5)) = [Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2)];
place!(Field::<[u8; 3]>(Variant(_4, 1), 7)) = [120_u8,146_u8,108_u8];
_7 = false as u128;
place!(Field::<([u16; 8],)>(Variant(_2, 1), 2)).0 = [2193_u16,19031_u16,30244_u16,4922_u16,40480_u16,64667_u16,53467_u16,57573_u16];
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)).0 = [41954_u16,27985_u16,18447_u16,46657_u16,27522_u16,30052_u16,30847_u16,27794_u16];
place!(Field::<*mut usize>(Variant(_2, 1), 6)) = core::ptr::addr_of_mut!(_1);
RET = (Field::<([u16; 8],)>(Variant(_2, 1), 2).0,);
_5 = (-43289369099797351704160748171698396658_i128) << _1;
_6 = Field::<([u16; 8],)>(Variant(_3, 1), 2);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_2, 1), 2).0;
place!(Field::<bool>(Variant(_4, 1), 0)) = true;
place!(Field::<char>(Variant(_4, 1), 1)) = '\u{3fba5}';
place!(Field::<[u32; 8]>(Variant(_2, 1), 5)) = [1307661351_u32,994111125_u32,3562665860_u32,1748622408_u32,305980351_u32,1924626199_u32,4194771783_u32,3614761578_u32];
place!(Field::<char>(Variant(_4, 1), 1)) = '\u{2d5b8}';
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = core::ptr::addr_of_mut!(_1);
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_4, 1), 1);
Goto(bb2)
}
bb2 = {
_8 = [13053_u16,27242_u16,60489_u16,14318_u16,59944_u16,57537_u16,19113_u16,54064_u16];
place!(Field::<bool>(Variant(_3, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<[u8; 3]>(Variant(_2, 1), 7)) = [188_u8,237_u8,79_u8];
place!(Field::<u64>(Variant(_4, 1), 4)) = 16369603026938400436_u64 | 3370595571892137506_u64;
place!(Field::<bool>(Variant(_2, 1), 0)) = _1 < _1;
place!(Field::<[u8; 3]>(Variant(_3, 1), 7)) = [66_u8,223_u8,133_u8];
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)) = (Field::<([u16; 8],)>(Variant(_4, 1), 2).0,);
RET = Field::<([u16; 8],)>(Variant(_3, 1), 2);
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<bool>(Variant(_4, 1), 0)) = Field::<bool>(Variant(_3, 1), 0) ^ Field::<bool>(Variant(_2, 1), 0);
_9.0 = 2340370875_u32 as i64;
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = core::ptr::addr_of_mut!(_1);
RET = Field::<([u16; 8],)>(Variant(_2, 1), 2);
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_4, 1), 1);
place!(Field::<bool>(Variant(_2, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<[u8; 3]>(Variant(_4, 1), 7)) = [0_u8,55_u8,2_u8];
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(_4, 1), 4);
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(_4, 1), 4);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = [61745_u16,41679_u16,7096_u16,49809_u16,48753_u16,15115_u16,13951_u16,15379_u16];
place!(Field::<u64>(Variant(_4, 1), 4)) = Field::<u64>(Variant(_3, 1), 4) << _9.0;
_1 = 16180971547395948915_usize;
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_3, 1), 6);
_11.fld0 = [31143_u16,48863_u16,58986_u16,36191_u16,6648_u16,34583_u16];
Call(place!(Field::<Adt37>(Variant(_2, 1), 3)) = fn6(Field::<([u16; 8],)>(Variant(_4, 1), 2).0, Field::<([u16; 8],)>(Variant(_4, 1), 2).0, Field::<bool>(Variant(_2, 1), 0), Field::<([u16; 8],)>(Variant(_2, 1), 2).0, _1, _6.0, Field::<([u16; 8],)>(Variant(_2, 1), 2), Field::<([u16; 8],)>(Variant(_3, 1), 2).0, Field::<([u16; 8],)>(Variant(_2, 1), 2).0, RET, _6.0, Field::<([u16; 8],)>(Variant(_3, 1), 2).0, _11.fld0, Field::<[u32; 8]>(Variant(_2, 1), 5), _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9.1 = [_9.0];
_15 = 27530_u16 - 53925_u16;
place!(Field::<bool>(Variant(_4, 1), 0)) = Field::<bool>(Variant(_2, 1), 0);
_9.3 = _15 - _15;
_9.1 = [_9.0];
_11.fld1 = Adt39::Variant1 { fld0: Field::<bool>(Variant(_4, 1), 0),fld1: Field::<char>(Variant(_4, 1), 1),fld2: RET,fld3: Field::<Adt37>(Variant(_2, 1), 3),fld4: Field::<u64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 1),fld5: Field::<[u32; 8]>(Variant(_3, 1), 5),fld6: Field::<*mut usize>(Variant(_3, 1), 6),fld7: Field::<[u8; 3]>(Variant(_3, 1), 7) };
place!(Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2)) = Field::<([u16; 8],)>(Variant(_2, 1), 2);
_4 = _11.fld1;
place!(Field::<*mut usize>(Variant(_2, 1), 6)) = Field::<*mut usize>(Variant(_4, 1), 6);
_6 = Field::<([u16; 8],)>(Variant(_2, 1), 2);
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 1), 3)) = Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3) << Field::<u64>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 1);
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)), 1), 2)) = Field::<f64>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 2);
RET.0 = [_9.3,_9.3,_15,_15,_9.3,_15,_9.3,_9.3];
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 1);
place!(Field::<[u32; 8]>(Variant(_2, 1), 5)) = Field::<[u32; 8]>(Variant(_11.fld1, 1), 5);
place!(Field::<Adt37>(Variant(_3, 1), 3)) = Adt37::Variant1 { fld0: Field::<usize>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 0),fld1: Field::<u64>(Variant(_11.fld1, 1), 4),fld2: Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2),fld3: Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3) };
SetDiscriminant(Field::<Adt37>(Variant(_4, 1), 3), 0);
place!(Field::<([u16; 8],)>(Variant(_2, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2).0;
_2 = _11.fld1;
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = [_15,_15,_9.3,_9.3,_9.3,_9.3,_15,_15];
place!(Field::<[u32; 8]>(Variant(_11.fld1, 1), 5)) = Field::<[u32; 8]>(Variant(_3, 1), 5);
_15 = !_9.3;
place!(Field::<*mut usize>(Variant(_11.fld1, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = core::ptr::addr_of_mut!(_1);
place!(Field::<[u32; 8]>(Variant(_4, 1), 5)) = [84165667_u32,76809181_u32,1663492426_u32,2812027400_u32,3623041572_u32,3758801421_u32,2428714173_u32,3251484980_u32];
_12 = _5 as u16;
place!(Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2)) = (RET.0,);
_4 = _2;
Goto(bb4)
}
bb4 = {
place!(Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2)).0 = [_15,_12,_9.3,_9.3,_9.3,_9.3,_9.3,_15];
_9.0 = (-7779410110766678058_i64) >> Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1);
_14 = Field::<u64>(Variant(_11.fld1, 1), 4) < Field::<u64>(Variant(_11.fld1, 1), 4);
RET.0 = [_9.3,_9.3,_9.3,_9.3,_9.3,_15,_15,_15];
place!(Field::<[u8; 3]>(Variant(_2, 1), 7)) = [Field::<u8>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 3)];
place!(Field::<u64>(Variant(_2, 1), 4)) = !Field::<u64>(Variant(_11.fld1, 1), 4);
place!(Field::<[u32; 8]>(Variant(_11.fld1, 1), 5)) = [346113777_u32,280830532_u32,2419127982_u32,304430029_u32,2286562178_u32,336585586_u32,3178392348_u32,1587423495_u32];
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2).0;
_9.0 = Field::<char>(Variant(_3, 1), 1) as i64;
_16 = Adt51::Variant0 { fld0: 3393448325_u32 };
RET = (Field::<([u16; 8],)>(Variant(_3, 1), 2).0,);
_9.1 = [_9.0];
place!(Field::<bool>(Variant(_2, 1), 0)) = !_14;
RET = (_8,);
RET = (_6.0,);
_8 = [_9.3,_12,_15,_15,_15,_15,_12,_15];
place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_2, 1), 3)), 1), 0)) = !Field::<usize>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 0);
place!(Field::<Adt37>(Variant(_4, 1), 3)) = Field::<Adt37>(Variant(_3, 1), 3);
_11.fld1 = _3;
Goto(bb5)
}
bb5 = {
place!(Field::<*mut usize>(Variant(_2, 1), 6)) = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_3, 1), 3)), 1), 0)));
SetDiscriminant(Field::<Adt37>(Variant(_2, 1), 3), 1);
_7 = 179869144854884706783897621086274335916_u128;
SetDiscriminant(Field::<Adt37>(Variant(_3, 1), 3), 1);
place!(Field::<Adt37>(Variant(_3, 1), 3)) = Field::<Adt37>(Variant(_4, 1), 3);
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_2, 1), 3)), 1), 3)) = 4872_i16 as u8;
_9.3 = _15;
_8 = [_15,_15,_12,_12,_15,_9.3,_9.3,_9.3];
_2 = _3;
place!(Field::<char>(Variant(_11.fld1, 1), 1)) = Field::<char>(Variant(_3, 1), 1);
_6.0 = [_15,_15,_15,_15,_15,_12,_9.3,_9.3];
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_2, 1), 3)), 1), 3)) = 2970074801_u32 as u8;
SetDiscriminant(Field::<Adt37>(Variant(_4, 1), 3), 0);
_22 = !Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3);
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_3, 1), 3)), 1), 2)) = Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2) * Field::<f64>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 2);
place!(Field::<u64>(Variant(place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)), 1), 1)) = Field::<u64>(Variant(_2, 1), 4) * Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1);
_16 = Adt51::Variant0 { fld0: 1694328573_u32 };
_8 = [_15,_12,_15,_9.3,_9.3,_9.3,_12,_15];
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)), 1), 3)) = Field::<u8>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 3) | _22;
Call(place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)), 1), 2)) = fn16(Field::<u64>(Variant(_11.fld1, 1), 4), Field::<Adt37>(Variant(_3, 1), 3), Field::<u64>(Variant(_4, 1), 4), Field::<Adt37>(Variant(_2, 1), 3), Field::<usize>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 0), _3, _2, Field::<u64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 1), Field::<[u32; 8]>(Variant(_2, 1), 5), _2, Field::<Adt37>(Variant(_2, 1), 3), Field::<Adt37>(Variant(_3, 1), 3), Field::<Adt37>(Variant(_3, 1), 3), Field::<u64>(Variant(_3, 1), 4), _3, Field::<u64>(Variant(_3, 1), 4)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_2, 1), 3)), 1), 2)) = Field::<f64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 2) * Field::<f64>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 2);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = [_15,_15,_15,_15,_9.3,_9.3,_15,_9.3];
place!(Field::<Adt37>(Variant(_4, 1), 3)) = Adt37::Variant1 { fld0: Field::<usize>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 0),fld1: Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1),fld2: Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2),fld3: Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3) };
place!(Field::<u64>(Variant(_2, 1), 4)) = Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1) - Field::<u64>(Variant(_3, 1), 4);
_19 = [Field::<u8>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 3),_22];
place!(Field::<bool>(Variant(_4, 1), 0)) = Field::<u64>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 1) != Field::<u64>(Variant(_2, 1), 4);
_6.0 = Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2).0;
match _1 {
0 => bb5,
1 => bb3,
16180971547395948915 => bb8,
_ => bb7
}
}
bb7 = {
place!(Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2)).0 = [_15,_12,_9.3,_9.3,_9.3,_9.3,_9.3,_15];
_9.0 = (-7779410110766678058_i64) >> Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1);
_14 = Field::<u64>(Variant(_11.fld1, 1), 4) < Field::<u64>(Variant(_11.fld1, 1), 4);
RET.0 = [_9.3,_9.3,_9.3,_9.3,_9.3,_15,_15,_15];
place!(Field::<[u8; 3]>(Variant(_2, 1), 7)) = [Field::<u8>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 3)];
place!(Field::<u64>(Variant(_2, 1), 4)) = !Field::<u64>(Variant(_11.fld1, 1), 4);
place!(Field::<[u32; 8]>(Variant(_11.fld1, 1), 5)) = [346113777_u32,280830532_u32,2419127982_u32,304430029_u32,2286562178_u32,336585586_u32,3178392348_u32,1587423495_u32];
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2).0;
_9.0 = Field::<char>(Variant(_3, 1), 1) as i64;
_16 = Adt51::Variant0 { fld0: 3393448325_u32 };
RET = (Field::<([u16; 8],)>(Variant(_3, 1), 2).0,);
_9.1 = [_9.0];
place!(Field::<bool>(Variant(_2, 1), 0)) = !_14;
RET = (_8,);
RET = (_6.0,);
_8 = [_9.3,_12,_15,_15,_15,_15,_12,_15];
place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_2, 1), 3)), 1), 0)) = !Field::<usize>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 0);
place!(Field::<Adt37>(Variant(_4, 1), 3)) = Field::<Adt37>(Variant(_3, 1), 3);
_11.fld1 = _3;
Goto(bb5)
}
bb8 = {
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)), 1), 3)) = !Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 3);
RET.0 = [_9.3,_9.3,_15,_9.3,_15,_9.3,_15,_12];
_1 = Field::<u64>(Variant(_2, 1), 4) as usize;
place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)) = Adt37::Variant1 { fld0: _1,fld1: Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1),fld2: Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2),fld3: Field::<u8>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 3) };
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 1), 2)) = -Field::<f64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 2);
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_3, 1), 3)), 1), 2)) = -Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2);
place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)) = Field::<Adt37>(Variant(_3, 1), 3);
_10 = [_9.3];
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 1), 3)) = _22 ^ Field::<u8>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 3);
_9.3 = _15;
place!(Field::<[u32; 8]>(Variant(_11.fld1, 1), 5)) = [1779722401_u32,3732342419_u32,1159575001_u32,3032249743_u32,955228416_u32,434859830_u32,2959694519_u32,1882365942_u32];
place!(Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2)).0 = [_9.3,_15,_9.3,_12,_15,_9.3,_15,_12];
_9.3 = _15;
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)), 1), 2)) = Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2) + Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2);
place!(Field::<[u32; 8]>(Variant(_11.fld1, 1), 5)) = [4065693881_u32,3286708995_u32,786187394_u32,1085844469_u32,1467769754_u32,2210415226_u32,1972315688_u32,403809340_u32];
_9.3 = _15;
place!(Field::<([u16; 8],)>(Variant(_2, 1), 2)).0 = [_15,_15,_12,_15,_9.3,_9.3,_9.3,_15];
_9.2 = [_9.0];
_25 = _11.fld0;
place!(Field::<char>(Variant(_4, 1), 1)) = Field::<char>(Variant(_11.fld1, 1), 1);
_15 = !_9.3;
_3 = _11.fld1;
place!(Field::<u64>(Variant(_2, 1), 4)) = 1463144769_i32 as u64;
place!(Field::<[u32; 8]>(Variant(_3, 1), 5)) = Field::<[u32; 8]>(Variant(_2, 1), 5);
RET.0 = Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2).0;
match _7 {
179869144854884706783897621086274335916 => bb9,
_ => bb7
}
}
bb9 = {
place!(Field::<u64>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 1), 1)) = !Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1);
_17 = [1862767683_u32,3081286137_u32,2858120402_u32,2535168678_u32,3361985010_u32,332208567_u32,145353461_u32,4227231261_u32];
_20 = Adt40::Variant0 { fld0: _11.fld1 };
place!(Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2)).0 = [_9.3,_9.3,_12,_9.3,_15,_9.3,_15,_15];
place!(Field::<Adt37>(Variant(_2, 1), 3)) = Field::<Adt37>(Variant(_3, 1), 3);
place!(Field::<*mut usize>(Variant(_11.fld1, 1), 6)) = Field::<*mut usize>(Variant(_4, 1), 6);
place!(Field::<Adt39>(Variant(_20, 0), 0)) = _2;
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 1), 3)) = _9.0 as u8;
_9.0 = 6749072308074865843_i64 + 6742656613831360793_i64;
place!(Field::<[u8; 3]>(Variant(_4, 1), 7)) = [Field::<u8>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3)];
place!(Field::<([u16; 8],)>(Variant(place!(Field::<Adt39>(Variant(_20, 0), 0)), 1), 2)) = (_6.0,);
place!(Field::<*mut usize>(Variant(place!(Field::<Adt39>(Variant(_20, 0), 0)), 1), 6)) = Field::<*mut usize>(Variant(_3, 1), 6);
SetDiscriminant(Field::<Adt37>(Variant(Field::<Adt39>(Variant(_20, 0), 0), 1), 3), 0);
place!(Field::<u8>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 1), 3)) = _22;
RET = (Field::<([u16; 8],)>(Variant(_3, 1), 2).0,);
_23 = !Field::<bool>(Variant(_4, 1), 0);
_13 = Field::<char>(Variant(_3, 1), 1);
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 1), 0)));
place!(Field::<[u32; 8]>(Variant(_11.fld1, 1), 5)) = [2144029468_u32,190532085_u32,805024360_u32,3096373619_u32,964551534_u32,3086771710_u32,2473312224_u32,591030605_u32];
SetDiscriminant(Field::<Adt37>(Variant(_4, 1), 3), 0);
place!(Field::<[u16; 1]>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 0), 4)) = [_9.3];
place!(Field::<Adt37>(Variant(_11.fld1, 1), 3)) = Adt37::Variant1 { fld0: _1,fld1: Field::<u64>(Variant(_3, 1), 4),fld2: Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2),fld3: Field::<u8>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 3) };
Goto(bb10)
}
bb10 = {
_5 = 22525047620282088998497622742814913534_i128;
_12 = _9.3;
RET.0 = [_15,_9.3,_12,_9.3,_12,_15,_15,_9.3];
match _7 {
0 => bb5,
179869144854884706783897621086274335916 => bb12,
_ => bb11
}
}
bb11 = {
_8 = [13053_u16,27242_u16,60489_u16,14318_u16,59944_u16,57537_u16,19113_u16,54064_u16];
place!(Field::<bool>(Variant(_3, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<[u8; 3]>(Variant(_2, 1), 7)) = [188_u8,237_u8,79_u8];
place!(Field::<u64>(Variant(_4, 1), 4)) = 16369603026938400436_u64 | 3370595571892137506_u64;
place!(Field::<bool>(Variant(_2, 1), 0)) = _1 < _1;
place!(Field::<[u8; 3]>(Variant(_3, 1), 7)) = [66_u8,223_u8,133_u8];
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)) = (Field::<([u16; 8],)>(Variant(_4, 1), 2).0,);
RET = Field::<([u16; 8],)>(Variant(_3, 1), 2);
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<bool>(Variant(_4, 1), 0)) = Field::<bool>(Variant(_3, 1), 0) ^ Field::<bool>(Variant(_2, 1), 0);
_9.0 = 2340370875_u32 as i64;
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = core::ptr::addr_of_mut!(_1);
RET = Field::<([u16; 8],)>(Variant(_2, 1), 2);
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_4, 1), 1);
place!(Field::<bool>(Variant(_2, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<[u8; 3]>(Variant(_4, 1), 7)) = [0_u8,55_u8,2_u8];
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(_4, 1), 4);
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(_4, 1), 4);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = [61745_u16,41679_u16,7096_u16,49809_u16,48753_u16,15115_u16,13951_u16,15379_u16];
place!(Field::<u64>(Variant(_4, 1), 4)) = Field::<u64>(Variant(_3, 1), 4) << _9.0;
_1 = 16180971547395948915_usize;
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_3, 1), 6);
_11.fld0 = [31143_u16,48863_u16,58986_u16,36191_u16,6648_u16,34583_u16];
Call(place!(Field::<Adt37>(Variant(_2, 1), 3)) = fn6(Field::<([u16; 8],)>(Variant(_4, 1), 2).0, Field::<([u16; 8],)>(Variant(_4, 1), 2).0, Field::<bool>(Variant(_2, 1), 0), Field::<([u16; 8],)>(Variant(_2, 1), 2).0, _1, _6.0, Field::<([u16; 8],)>(Variant(_2, 1), 2), Field::<([u16; 8],)>(Variant(_3, 1), 2).0, Field::<([u16; 8],)>(Variant(_2, 1), 2).0, RET, _6.0, Field::<([u16; 8],)>(Variant(_3, 1), 2).0, _11.fld0, Field::<[u32; 8]>(Variant(_2, 1), 5), _5), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
SetDiscriminant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 0);
place!(Field::<char>(Variant(_11.fld1, 1), 1)) = Field::<char>(Variant(_4, 1), 1);
_28 = (-90_isize) >> Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1);
match _7 {
0 => bb9,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
179869144854884706783897621086274335916 => bb19,
_ => bb18
}
}
bb13 = {
_8 = [13053_u16,27242_u16,60489_u16,14318_u16,59944_u16,57537_u16,19113_u16,54064_u16];
place!(Field::<bool>(Variant(_3, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<[u8; 3]>(Variant(_2, 1), 7)) = [188_u8,237_u8,79_u8];
place!(Field::<u64>(Variant(_4, 1), 4)) = 16369603026938400436_u64 | 3370595571892137506_u64;
place!(Field::<bool>(Variant(_2, 1), 0)) = _1 < _1;
place!(Field::<[u8; 3]>(Variant(_3, 1), 7)) = [66_u8,223_u8,133_u8];
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)) = (Field::<([u16; 8],)>(Variant(_4, 1), 2).0,);
RET = Field::<([u16; 8],)>(Variant(_3, 1), 2);
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<bool>(Variant(_4, 1), 0)) = Field::<bool>(Variant(_3, 1), 0) ^ Field::<bool>(Variant(_2, 1), 0);
_9.0 = 2340370875_u32 as i64;
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = core::ptr::addr_of_mut!(_1);
RET = Field::<([u16; 8],)>(Variant(_2, 1), 2);
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_4, 1), 1);
place!(Field::<bool>(Variant(_2, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<[u8; 3]>(Variant(_4, 1), 7)) = [0_u8,55_u8,2_u8];
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(_4, 1), 4);
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(_4, 1), 4);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = [61745_u16,41679_u16,7096_u16,49809_u16,48753_u16,15115_u16,13951_u16,15379_u16];
place!(Field::<u64>(Variant(_4, 1), 4)) = Field::<u64>(Variant(_3, 1), 4) << _9.0;
_1 = 16180971547395948915_usize;
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_3, 1), 6);
_11.fld0 = [31143_u16,48863_u16,58986_u16,36191_u16,6648_u16,34583_u16];
Call(place!(Field::<Adt37>(Variant(_2, 1), 3)) = fn6(Field::<([u16; 8],)>(Variant(_4, 1), 2).0, Field::<([u16; 8],)>(Variant(_4, 1), 2).0, Field::<bool>(Variant(_2, 1), 0), Field::<([u16; 8],)>(Variant(_2, 1), 2).0, _1, _6.0, Field::<([u16; 8],)>(Variant(_2, 1), 2), Field::<([u16; 8],)>(Variant(_3, 1), 2).0, Field::<([u16; 8],)>(Variant(_2, 1), 2).0, RET, _6.0, Field::<([u16; 8],)>(Variant(_3, 1), 2).0, _11.fld0, Field::<[u32; 8]>(Variant(_2, 1), 5), _5), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_5 = 22525047620282088998497622742814913534_i128;
_12 = _9.3;
RET.0 = [_15,_9.3,_12,_9.3,_12,_15,_15,_9.3];
match _7 {
0 => bb5,
179869144854884706783897621086274335916 => bb12,
_ => bb11
}
}
bb15 = {
_8 = [13053_u16,27242_u16,60489_u16,14318_u16,59944_u16,57537_u16,19113_u16,54064_u16];
place!(Field::<bool>(Variant(_3, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<[u8; 3]>(Variant(_2, 1), 7)) = [188_u8,237_u8,79_u8];
place!(Field::<u64>(Variant(_4, 1), 4)) = 16369603026938400436_u64 | 3370595571892137506_u64;
place!(Field::<bool>(Variant(_2, 1), 0)) = _1 < _1;
place!(Field::<[u8; 3]>(Variant(_3, 1), 7)) = [66_u8,223_u8,133_u8];
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)) = (Field::<([u16; 8],)>(Variant(_4, 1), 2).0,);
RET = Field::<([u16; 8],)>(Variant(_3, 1), 2);
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<bool>(Variant(_4, 1), 0)) = Field::<bool>(Variant(_3, 1), 0) ^ Field::<bool>(Variant(_2, 1), 0);
_9.0 = 2340370875_u32 as i64;
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = core::ptr::addr_of_mut!(_1);
RET = Field::<([u16; 8],)>(Variant(_2, 1), 2);
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_4, 1), 1);
place!(Field::<bool>(Variant(_2, 1), 0)) = Field::<bool>(Variant(_4, 1), 0);
place!(Field::<[u8; 3]>(Variant(_4, 1), 7)) = [0_u8,55_u8,2_u8];
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(_4, 1), 4);
place!(Field::<u64>(Variant(_3, 1), 4)) = !Field::<u64>(Variant(_4, 1), 4);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = [61745_u16,41679_u16,7096_u16,49809_u16,48753_u16,15115_u16,13951_u16,15379_u16];
place!(Field::<u64>(Variant(_4, 1), 4)) = Field::<u64>(Variant(_3, 1), 4) << _9.0;
_1 = 16180971547395948915_usize;
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = Field::<*mut usize>(Variant(_3, 1), 6);
_11.fld0 = [31143_u16,48863_u16,58986_u16,36191_u16,6648_u16,34583_u16];
Call(place!(Field::<Adt37>(Variant(_2, 1), 3)) = fn6(Field::<([u16; 8],)>(Variant(_4, 1), 2).0, Field::<([u16; 8],)>(Variant(_4, 1), 2).0, Field::<bool>(Variant(_2, 1), 0), Field::<([u16; 8],)>(Variant(_2, 1), 2).0, _1, _6.0, Field::<([u16; 8],)>(Variant(_2, 1), 2), Field::<([u16; 8],)>(Variant(_3, 1), 2).0, Field::<([u16; 8],)>(Variant(_2, 1), 2).0, RET, _6.0, Field::<([u16; 8],)>(Variant(_3, 1), 2).0, _11.fld0, Field::<[u32; 8]>(Variant(_2, 1), 5), _5), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
place!(Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2)).0 = [_15,_12,_9.3,_9.3,_9.3,_9.3,_9.3,_15];
_9.0 = (-7779410110766678058_i64) >> Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1);
_14 = Field::<u64>(Variant(_11.fld1, 1), 4) < Field::<u64>(Variant(_11.fld1, 1), 4);
RET.0 = [_9.3,_9.3,_9.3,_9.3,_9.3,_15,_15,_15];
place!(Field::<[u8; 3]>(Variant(_2, 1), 7)) = [Field::<u8>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 3)];
place!(Field::<u64>(Variant(_2, 1), 4)) = !Field::<u64>(Variant(_11.fld1, 1), 4);
place!(Field::<[u32; 8]>(Variant(_11.fld1, 1), 5)) = [346113777_u32,280830532_u32,2419127982_u32,304430029_u32,2286562178_u32,336585586_u32,3178392348_u32,1587423495_u32];
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2).0;
_9.0 = Field::<char>(Variant(_3, 1), 1) as i64;
_16 = Adt51::Variant0 { fld0: 3393448325_u32 };
RET = (Field::<([u16; 8],)>(Variant(_3, 1), 2).0,);
_9.1 = [_9.0];
place!(Field::<bool>(Variant(_2, 1), 0)) = !_14;
RET = (_8,);
RET = (_6.0,);
_8 = [_9.3,_12,_15,_15,_15,_15,_12,_15];
place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_2, 1), 3)), 1), 0)) = !Field::<usize>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 0);
place!(Field::<Adt37>(Variant(_4, 1), 3)) = Field::<Adt37>(Variant(_3, 1), 3);
_11.fld1 = _3;
Goto(bb5)
}
bb17 = {
place!(Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2)).0 = [_15,_12,_9.3,_9.3,_9.3,_9.3,_9.3,_15];
_9.0 = (-7779410110766678058_i64) >> Field::<u64>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 1);
_14 = Field::<u64>(Variant(_11.fld1, 1), 4) < Field::<u64>(Variant(_11.fld1, 1), 4);
RET.0 = [_9.3,_9.3,_9.3,_9.3,_9.3,_15,_15,_15];
place!(Field::<[u8; 3]>(Variant(_2, 1), 7)) = [Field::<u8>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_11.fld1, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 3)];
place!(Field::<u64>(Variant(_2, 1), 4)) = !Field::<u64>(Variant(_11.fld1, 1), 4);
place!(Field::<[u32; 8]>(Variant(_11.fld1, 1), 5)) = [346113777_u32,280830532_u32,2419127982_u32,304430029_u32,2286562178_u32,336585586_u32,3178392348_u32,1587423495_u32];
place!(Field::<*mut usize>(Variant(_3, 1), 6)) = Field::<*mut usize>(Variant(_2, 1), 6);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_11.fld1, 1), 2).0;
_9.0 = Field::<char>(Variant(_3, 1), 1) as i64;
_16 = Adt51::Variant0 { fld0: 3393448325_u32 };
RET = (Field::<([u16; 8],)>(Variant(_3, 1), 2).0,);
_9.1 = [_9.0];
place!(Field::<bool>(Variant(_2, 1), 0)) = !_14;
RET = (_8,);
RET = (_6.0,);
_8 = [_9.3,_12,_15,_15,_15,_15,_12,_15];
place!(Field::<usize>(Variant(place!(Field::<Adt37>(Variant(_2, 1), 3)), 1), 0)) = !Field::<usize>(Variant(Field::<Adt37>(Variant(_3, 1), 3), 1), 0);
place!(Field::<Adt37>(Variant(_4, 1), 3)) = Field::<Adt37>(Variant(_3, 1), 3);
_11.fld1 = _3;
Goto(bb5)
}
bb18 = {
_1 = !1_usize;
_5 = !(-159754184149520304549595059111060375694_i128);
RET.0 = [34974_u16,63965_u16,3742_u16,56257_u16,27262_u16,59769_u16,22502_u16,31095_u16];
_6 = (RET.0,);
SetDiscriminant(_3, 1);
_5 = 93867354_i32 as i128;
place!(Field::<[u32; 8]>(Variant(_3, 1), 5)) = [Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_4, 2), 2),Field::<u32>(Variant(_4, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_4, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_4, 2), 2)];
place!(Field::<u32>(Variant(_2, 2), 2)) = !Field::<u32>(Variant(_4, 2), 2);
_2 = _4;
SetDiscriminant(_4, 1);
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)) = _6;
place!(Field::<[u32; 8]>(Variant(_3, 1), 5)) = [Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2)];
place!(Field::<[u32; 8]>(Variant(_3, 1), 5)) = [Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2),Field::<u32>(Variant(_2, 2), 2)];
place!(Field::<[u8; 3]>(Variant(_4, 1), 7)) = [120_u8,146_u8,108_u8];
SetDiscriminant(_2, 1);
_7 = false as u128;
place!(Field::<([u16; 8],)>(Variant(_2, 1), 2)).0 = [2193_u16,19031_u16,30244_u16,4922_u16,40480_u16,64667_u16,53467_u16,57573_u16];
place!(Field::<([u16; 8],)>(Variant(_3, 1), 2)).0 = [41954_u16,27985_u16,18447_u16,46657_u16,27522_u16,30052_u16,30847_u16,27794_u16];
place!(Field::<*mut usize>(Variant(_2, 1), 6)) = core::ptr::addr_of_mut!(_1);
RET = (Field::<([u16; 8],)>(Variant(_2, 1), 2).0,);
_5 = (-43289369099797351704160748171698396658_i128) << _1;
_6 = Field::<([u16; 8],)>(Variant(_3, 1), 2);
place!(Field::<([u16; 8],)>(Variant(_4, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_2, 1), 2).0;
place!(Field::<bool>(Variant(_4, 1), 0)) = true;
place!(Field::<char>(Variant(_4, 1), 1)) = '\u{3fba5}';
place!(Field::<[u32; 8]>(Variant(_2, 1), 5)) = [1307661351_u32,994111125_u32,3562665860_u32,1748622408_u32,305980351_u32,1924626199_u32,4194771783_u32,3614761578_u32];
place!(Field::<char>(Variant(_4, 1), 1)) = '\u{2d5b8}';
place!(Field::<*mut usize>(Variant(_4, 1), 6)) = core::ptr::addr_of_mut!(_1);
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_4, 1), 1);
Goto(bb2)
}
bb19 = {
_31 = Field::<char>(Variant(_2, 1), 1) as u8;
_18 = Field::<u64>(Variant(_11.fld1, 1), 4) as f32;
_7 = _28 as u128;
_14 = _23;
place!(Field::<u32>(Variant(_16, 0), 0)) = 472398142_u32;
place!(Field::<char>(Variant(_4, 1), 1)) = _13;
_11.fld1 = _3;
place!(Field::<isize>(Variant(place!(Field::<Adt37>(Variant(_4, 1), 3)), 0), 2)) = _28;
place!(Field::<Adt39>(Variant(_20, 0), 0)) = _2;
place!(Field::<[u8; 3]>(Variant(_3, 1), 7)) = [_22,_22,_22];
_32 = 39_i8;
_2 = Field::<Adt39>(Variant(_20, 0), 0);
place!(Field::<bool>(Variant(_4, 1), 0)) = _14 != _23;
_4 = _11.fld1;
place!(Field::<[u8; 3]>(Variant(_11.fld1, 1), 7)) = [Field::<u8>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 3),Field::<u8>(Variant(Field::<Adt37>(Variant(_4, 1), 3), 1), 3),_22];
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(place!(Field::<Adt39>(Variant(_20, 0), 0)), 1), 3)), 1), 2)) = -Field::<f64>(Variant(Field::<Adt37>(Variant(_2, 1), 3), 1), 2);
_28 = !9223372036854775807_isize;
place!(Field::<char>(Variant(_4, 1), 1)) = Field::<char>(Variant(_2, 1), 1);
_13 = Field::<char>(Variant(_2, 1), 1);
_23 = _14;
_21 = _23 | _23;
Goto(bb20)
}
bb20 = {
Call(_38 = dump_var(5_usize, 21_usize, Move(_21), 10_usize, Move(_10), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_38 = dump_var(5_usize, 23_usize, Move(_23), 9_usize, Move(_9), 31_usize, Move(_31), 32_usize, Move(_32)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_38 = dump_var(5_usize, 15_usize, Move(_15), 5_usize, Move(_5), 39_usize, _39, 39_usize, _39), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [u16; 8],mut _2: [u16; 8],mut _3: bool,mut _4: [u16; 8],mut _5: usize,mut _6: [u16; 8],mut _7: ([u16; 8],),mut _8: [u16; 8],mut _9: [u16; 8],mut _10: ([u16; 8],),mut _11: [u16; 8],mut _12: [u16; 8],mut _13: [u16; 6],mut _14: [u32; 8],mut _15: i128) -> Adt37 {
mir! {
type RET = Adt37;
let _16: isize;
let _17: u64;
let _18: ([u16; 8],);
let _19: [u128; 2];
let _20: f64;
let _21: [u128; 1];
let _22: *mut usize;
let _23: Adt45;
let _24: *mut f64;
let _25: i32;
let _26: [u16; 1];
let _27: isize;
let _28: [u16; 6];
let _29: char;
let _30: bool;
let _31: ();
let _32: ();
{
_8 = _10.0;
_9 = _4;
_16 = 9223372036854775807_isize;
_13 = [39325_u16,35672_u16,27528_u16,29708_u16,769_u16,62_u16];
_18 = (_8,);
_4 = [63735_u16,42053_u16,56817_u16,49126_u16,961_u16,36297_u16,19793_u16,61113_u16];
_15 = _5 as i128;
match _16 {
0 => bb1,
1 => bb2,
2 => bb3,
9223372036854775807 => bb5,
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
_17 = 978764288_u32 as u64;
_12 = _7.0;
_19 = [285039116323453954707179403420997491971_u128,27096044300507341093219695528105534491_u128];
_6 = [56275_u16,26303_u16,3838_u16,27897_u16,32162_u16,58438_u16,60294_u16,45690_u16];
_12 = [21421_u16,21037_u16,8177_u16,28638_u16,24453_u16,45389_u16,38653_u16,46855_u16];
_19 = [231816143306156052861053859326154889307_u128,264210543652610173681739208512279030722_u128];
_20 = _15 as f64;
_13 = [55685_u16,26334_u16,7827_u16,42010_u16,39970_u16,16123_u16];
_19 = [79610509236119027146079700603775331045_u128,273303345941514812735858583209142815152_u128];
_13 = [31827_u16,29644_u16,65039_u16,24900_u16,55908_u16,568_u16];
_21 = [123881214333171223121655528739336061862_u128];
Goto(bb6)
}
bb6 = {
_18 = (_11,);
_12 = [52741_u16,22229_u16,1719_u16,52889_u16,56726_u16,22902_u16,21853_u16,51655_u16];
_22 = core::ptr::addr_of_mut!(_5);
_25 = (-1145358690_i32);
RET = Adt37::Variant1 { fld0: (*_22),fld1: _17,fld2: _20,fld3: 231_u8 };
place!(Field::<u8>(Variant(RET, 1), 3)) = 5480360306806768645_i64 as u8;
_16 = -(-73_isize);
place!(Field::<u8>(Variant(RET, 1), 3)) = !64_u8;
_26 = [46176_u16];
SetDiscriminant(RET, 1);
place!(Field::<usize>(Variant(RET, 1), 0)) = !_5;
_11 = _7.0;
match (*_22) {
16180971547395948915 => bb7,
_ => bb2
}
}
bb7 = {
_22 = core::ptr::addr_of_mut!(_5);
_5 = Field::<usize>(Variant(RET, 1), 0) - Field::<usize>(Variant(RET, 1), 0);
place!(Field::<u8>(Variant(RET, 1), 3)) = 147_u8 | 66_u8;
_18 = _7;
_9 = _4;
place!(Field::<u64>(Variant(RET, 1), 1)) = !_17;
_29 = '\u{92b10}';
Call(RET = fn7(_18, _2, _18, _18.0, _1, _18.0, _14, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Goto(bb9)
}
bb9 = {
Call(_31 = dump_var(6_usize, 4_usize, Move(_4), 7_usize, Move(_7), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_31 = dump_var(6_usize, 9_usize, Move(_9), 2_usize, Move(_2), 14_usize, Move(_14), 17_usize, Move(_17)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_31 = dump_var(6_usize, 5_usize, Move(_5), 19_usize, Move(_19), 11_usize, Move(_11), 32_usize, _32), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: ([u16; 8],),mut _2: [u16; 8],mut _3: ([u16; 8],),mut _4: [u16; 8],mut _5: [u16; 8],mut _6: [u16; 8],mut _7: [u32; 8],mut _8: usize) -> Adt37 {
mir! {
type RET = Adt37;
let _9: char;
let _10: [u16; 6];
let _11: char;
let _12: *mut [u8; 3];
let _13: (i64, [i64; 1], [i64; 1], u16);
let _14: i8;
let _15: [u8; 3];
let _16: i32;
let _17: isize;
let _18: [u32; 8];
let _19: u8;
let _20: i128;
let _21: isize;
let _22: [u128; 1];
let _23: [u128; 1];
let _24: i16;
let _25: bool;
let _26: usize;
let _27: isize;
let _28: Adt41;
let _29: char;
let _30: i128;
let _31: Adt44;
let _32: [u128; 8];
let _33: [u128; 8];
let _34: [u16; 8];
let _35: usize;
let _36: isize;
let _37: isize;
let _38: usize;
let _39: char;
let _40: [u128; 2];
let _41: u8;
let _42: i32;
let _43: [u16; 8];
let _44: u128;
let _45: f32;
let _46: [u16; 8];
let _47: isize;
let _48: u8;
let _49: (i64, [i64; 1], [i64; 1], u16);
let _50: u16;
let _51: Adt44;
let _52: [u16; 6];
let _53: Adt53;
let _54: isize;
let _55: isize;
let _56: [i64; 1];
let _57: i16;
let _58: [u128; 2];
let _59: [u16; 8];
let _60: [u8; 3];
let _61: *const *mut f64;
let _62: *const bool;
let _63: [u16; 6];
let _64: char;
let _65: isize;
let _66: isize;
let _67: Adt47;
let _68: *mut f32;
let _69: i128;
let _70: [u16; 1];
let _71: u32;
let _72: isize;
let _73: Adt37;
let _74: [u128; 2];
let _75: [u16; 8];
let _76: [u128; 1];
let _77: i8;
let _78: &'static [u16; 1];
let _79: (i64, [i64; 1], [i64; 1], u16);
let _80: f64;
let _81: u128;
let _82: f32;
let _83: f64;
let _84: [u128; 2];
let _85: [u128; 8];
let _86: char;
let _87: f64;
let _88: u128;
let _89: Adt48;
let _90: char;
let _91: *mut f32;
let _92: [u8; 3];
let _93: i64;
let _94: i32;
let _95: [i64; 1];
let _96: f64;
let _97: [u128; 1];
let _98: f32;
let _99: *const *mut f64;
let _100: Adt38;
let _101: [u128; 2];
let _102: bool;
let _103: u32;
let _104: [u128; 2];
let _105: ([u16; 8],);
let _106: u8;
let _107: u64;
let _108: f32;
let _109: [u16; 6];
let _110: u32;
let _111: Adt37;
let _112: usize;
let _113: ();
let _114: ();
{
_1 = _3;
_7 = [3976439892_u32,3838062382_u32,2823541665_u32,257663140_u32,3010339023_u32,3164312101_u32,1359171720_u32,4063314208_u32];
_1 = _3;
_3 = (_2,);
_3.0 = _6;
_8 = 9643549846908055588_usize ^ 0_usize;
_8 = 855757322_i32 as usize;
_1.0 = [37218_u16,52201_u16,55642_u16,53256_u16,33643_u16,37262_u16,40117_u16,17461_u16];
_9 = '\u{141c}';
_8 = 5_usize;
_1.0 = [_3.0[_8],_6[_8],_4[_8],_3.0[_8],_2[_8],_4[_8],_2[_8],_3.0[_8]];
_7[_8] = 2492078248_u32;
_6 = [_2[_8],_3.0[_8],_5[_8],_3.0[_8],_5[_8],_3.0[_8],_5[_8],_3.0[_8]];
_6 = _3.0;
_7[_8] = 3660331571_u32 + 4181679411_u32;
_1 = (_3.0,);
Goto(bb1)
}
bb1 = {
_2[_8] = _4[_8];
_4[_8] = _5[_8];
_3.0[_8] = _2[_8];
_10 = [_3.0[_8],_4[_8],_1.0[_8],_3.0[_8],_3.0[_8],_1.0[_8]];
Call(_5 = fn8(_4[_8], _1.0[_8], _3, _6[_8], _9, _3.0[_8], _3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.0 = [9126_u16,27788_u16,60938_u16,59480_u16,31139_u16,34635_u16,31860_u16,50872_u16];
_3.0 = _5;
_11 = _9;
_3.0 = _5;
_9 = _11;
_1.0 = _3.0;
_3 = (_5,);
_6 = _5;
_8 = 2890329087_u32 as usize;
_8 = 18_u8 as usize;
_7 = [760793062_u32,2228995379_u32,1833569676_u32,2656355993_u32,1371690884_u32,3528114710_u32,3281640141_u32,2044161854_u32];
_1 = (_5,);
_7 = [1864053148_u32,261783831_u32,1382911615_u32,290135370_u32,1210548162_u32,2870799147_u32,3630033278_u32,3962332152_u32];
_10 = [53948_u16,31915_u16,44882_u16,15940_u16,52766_u16,990_u16];
_10 = [15205_u16,37571_u16,21747_u16,26055_u16,52364_u16,2404_u16];
_2 = [47863_u16,5421_u16,426_u16,47607_u16,13841_u16,32545_u16,42854_u16,37928_u16];
_3.0 = [853_u16,10263_u16,31456_u16,31070_u16,51465_u16,37557_u16,767_u16,51952_u16];
_1 = (_6,);
_13.3 = 4808404238256422061_i64 as u16;
_5 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_2 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_7 = [2457658403_u32,2707276455_u32,2729317155_u32,333521517_u32,1062940853_u32,2383733572_u32,3861362598_u32,3365229194_u32];
_13.3 = 57348_u16;
Goto(bb3)
}
bb3 = {
_8 = !2_usize;
_5 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_6 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_5 = _1.0;
_13.1 = [(-4733576225882276046_i64)];
_8 = 7_usize;
_8 = 12945661795188774765_usize & 7_usize;
_13.2 = [3385183976126118962_i64];
_7 = [924489307_u32,1870152633_u32,3058218530_u32,3459414791_u32,3263307495_u32,2212817853_u32,2178400394_u32,1460980247_u32];
_14 = false as i8;
_13.2 = [(-3757050979495045471_i64)];
match _13.3 {
0 => bb2,
1 => bb4,
57348 => bb6,
_ => bb5
}
}
bb4 = {
_1.0 = [9126_u16,27788_u16,60938_u16,59480_u16,31139_u16,34635_u16,31860_u16,50872_u16];
_3.0 = _5;
_11 = _9;
_3.0 = _5;
_9 = _11;
_1.0 = _3.0;
_3 = (_5,);
_6 = _5;
_8 = 2890329087_u32 as usize;
_8 = 18_u8 as usize;
_7 = [760793062_u32,2228995379_u32,1833569676_u32,2656355993_u32,1371690884_u32,3528114710_u32,3281640141_u32,2044161854_u32];
_1 = (_5,);
_7 = [1864053148_u32,261783831_u32,1382911615_u32,290135370_u32,1210548162_u32,2870799147_u32,3630033278_u32,3962332152_u32];
_10 = [53948_u16,31915_u16,44882_u16,15940_u16,52766_u16,990_u16];
_10 = [15205_u16,37571_u16,21747_u16,26055_u16,52364_u16,2404_u16];
_2 = [47863_u16,5421_u16,426_u16,47607_u16,13841_u16,32545_u16,42854_u16,37928_u16];
_3.0 = [853_u16,10263_u16,31456_u16,31070_u16,51465_u16,37557_u16,767_u16,51952_u16];
_1 = (_6,);
_13.3 = 4808404238256422061_i64 as u16;
_5 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_2 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_7 = [2457658403_u32,2707276455_u32,2729317155_u32,333521517_u32,1062940853_u32,2383733572_u32,3861362598_u32,3365229194_u32];
_13.3 = 57348_u16;
Goto(bb3)
}
bb5 = {
_2[_8] = _4[_8];
_4[_8] = _5[_8];
_3.0[_8] = _2[_8];
_10 = [_3.0[_8],_4[_8],_1.0[_8],_3.0[_8],_3.0[_8],_1.0[_8]];
Call(_5 = fn8(_4[_8], _1.0[_8], _3, _6[_8], _9, _3.0[_8], _3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_8 = 2_usize;
_7 = [3682512859_u32,895163007_u32,4040261181_u32,2376509573_u32,2456059635_u32,293460653_u32,3318760773_u32,2441794832_u32];
_15[_8] = !253_u8;
_4[_8] = _1.0[_8] + _5[_8];
_4[_8] = (-9223372036854775808_isize) as u16;
_1 = (_3.0,);
_7 = [284971602_u32,1200045671_u32,2286945083_u32,881475354_u32,1371738404_u32,874709716_u32,2138335330_u32,750726367_u32];
_1.0 = [_5[_8],_5[_8],_5[_8],_5[_8],_5[_8],_5[_8],_5[_8],_5[_8]];
_3.0[_8] = _7[_8] as u16;
_18 = [_7[_8],_7[_8],_7[_8],_7[_8],_7[_8],_7[_8],_7[_8],_7[_8]];
_18[_8] = _7[_8] % _7[_8];
_16 = 211212860_i32 * (-745523850_i32);
_15 = [226_u8,139_u8,218_u8];
Call(_3.0[_8] = core::intrinsics::bswap(_5[_8]), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_20 = (-93854860774329981398219775475468600813_i128);
_7[_8] = _18[_8] | _18[_8];
_22 = [34016965254624883780463190880690348928_u128];
_10[_8] = false as u16;
_21 = (-9223372036854775808_isize);
_2[_8] = _6[_8];
_17 = !_21;
_3.0[_8] = !_1.0[_8];
_3.0[_8] = _1.0[_8];
_22 = [305985895330508569920910911439162406499_u128];
_3.0 = _1.0;
_13.3 = _3.0[_8] & _5[_8];
_1.0[_8] = _5[_8];
_9 = _11;
_15 = [178_u8,81_u8,138_u8];
_22 = [220711634722580560118060193205230692742_u128];
_17 = _21 & _21;
_14 = _17 as i8;
_13.0 = !(-7073263628579381195_i64);
_13.2 = [_13.0];
_15[_8] = 13_u8 & 132_u8;
_24 = 26568_i16;
_20 = -99412155035971748556940629985772649239_i128;
_10 = [_5[_8],_1.0[_8],_1.0[_8],_5[_8],_13.3,_13.3];
_19 = _14 as u8;
_10[_8] = !_5[_8];
_7[_8] = _18[_8];
Goto(bb8)
}
bb8 = {
_3.0[_8] = _13.3;
_7[_8] = _18[_8];
_4[_8] = _18[_8] as u16;
_11 = _9;
_3.0[_8] = _10[_8] - _13.3;
_20 = !46659875859872445430651867988287464042_i128;
_10[_8] = !_3.0[_8];
_2[_8] = _13.3;
_12 = core::ptr::addr_of_mut!(_15);
_5 = [_3.0[_8],_10[_8],_3.0[_8],_3.0[_8],_10[_8],_3.0[_8],_13.3,_2[_8]];
Goto(bb9)
}
bb9 = {
_11 = _9;
_21 = !_17;
_13.1 = _13.2;
_19 = !_15[_8];
_15[_8] = !_19;
_10[_8] = !_5[_8];
_13.2 = _13.1;
_3.0[_8] = _2[_8] - _2[_8];
_10[_8] = !_13.3;
_14 = _24 as i8;
Call(_17 = fn15(_10[_8], _10, _10[_8], _1, _3.0, _13.3, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_28.fld0 = core::ptr::addr_of!(_25);
_21 = _17 | _17;
_18 = [344365180_u32,764639658_u32,1201882103_u32,3848491235_u32,3174405525_u32,2899981430_u32,2145654778_u32,4181631823_u32];
_26 = _8;
_8 = !_26;
_2[_26] = !_1.0[_26];
_27 = true as isize;
_5 = [_10[_26],_3.0[_26],_2[_26],_3.0[_26],_3.0[_26],_3.0[_26],_3.0[_26],_1.0[_26]];
_22 = [304999967493103485426468642755864889463_u128];
_30 = _20 * _20;
_17 = _21 & _21;
_3 = (_1.0,);
_11 = _9;
_31 = Adt44::Variant0 { fld0: 12297238000351177985_u64,fld1: _12 };
_18[_26] = _7[_26] - _7[_26];
_3.0[_26] = _2[_26];
_5[_26] = _3.0[_26];
_3 = (_1.0,);
_2 = _3.0;
_9 = _11;
_11 = _9;
_23 = [316973765853758340487389540184479011011_u128];
_1 = (_5,);
Goto(bb11)
}
bb11 = {
_5 = [_13.3,_4[_26],_2[_26],_13.3,_4[_26],_10[_26],_1.0[_26],_13.3];
_10 = [_3.0[_26],_5[_26],_3.0[_26],_5[_26],_13.3,_3.0[_26]];
_3 = (_5,);
_3.0[_26] = _13.3 | _13.3;
_16 = 139823147_i32 ^ (-1770563448_i32);
_25 = false;
_3.0 = _2;
_28.fld0 = core::ptr::addr_of!(_25);
_27 = _21;
_32 = [63468465250896308826194763540649629218_u128,328013454616293684416706314554742828304_u128,89274569644414723620626404169119316218_u128,172419567861260094246543097293989692514_u128,57662171246509350355668800806028727013_u128,240508316372547308407038179884185601817_u128,93973199086062023556513748227599711854_u128,66388533201675396515814617909802757609_u128];
_10[_26] = _3.0[_26];
_33[_26] = _32[_26] + _32[_26];
_28.fld0 = core::ptr::addr_of!(_25);
_12 = core::ptr::addr_of_mut!(_15);
_9 = _11;
_34 = [_10[_26],_5[_26],_5[_26],_1.0[_26],_1.0[_26],_13.3,_3.0[_26],_1.0[_26]];
_9 = _11;
_3 = (_4,);
_32 = [_33[_26],_33[_26],_33[_26],_33[_26],_33[_26],_33[_26],_33[_26],_33[_26]];
_7[_26] = _32[_26] as u32;
_35 = 2079326018520150549_u64 as usize;
_5[_26] = !_34[_26];
_13.2 = [_13.0];
_28.fld0 = core::ptr::addr_of!(_25);
_37 = _17 >> _17;
Goto(bb12)
}
bb12 = {
place!(Field::<*mut [u8; 3]>(Variant(_31, 0), 1)) = _12;
_9 = _11;
_7 = [_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26]];
_3 = _1;
_1.0 = [_3.0[_26],_5[_26],_3.0[_26],_13.3,_3.0[_26],_10[_26],_5[_26],_13.3];
Goto(bb13)
}
bb13 = {
_14 = !88_i8;
_33[_26] = _32[_26] & _32[_26];
_1.0[_26] = _5[_26];
_15[_26] = !_19;
_1.0[_26] = _2[_26];
_15 = [_19,_19,_19];
_2[_26] = _5[_26];
_11 = _9;
_13.2 = [_13.0];
_32[_26] = _33[_26] + _33[_26];
_38 = _35 - _8;
_15 = [_19,_19,_19];
_23 = [_33[_26]];
Goto(bb14)
}
bb14 = {
_30 = _7[_26] as i128;
_3.0[_26] = !_34[_26];
_10[_26] = _4[_26];
_35 = _26 - _26;
_15[_26] = !_19;
_33 = [_32[_26],_32[_26],_32[_26],_32[_26],_32[_26],_32[_26],_32[_26],_32[_26]];
_20 = _30;
_27 = _17 ^ _17;
_41 = !_19;
_22 = [_32[_26]];
_5 = [_1.0[_26],_2[_26],_1.0[_26],_3.0[_26],_1.0[_26],_2[_26],_3.0[_26],_3.0[_26]];
_23 = [_33[_26]];
_18[_26] = _7[_26] | _7[_26];
_18 = _7;
_29 = _9;
_10[_26] = !_1.0[_26];
_41 = (*_12)[_26] & _15[_26];
Goto(bb15)
}
bb15 = {
_29 = _11;
_36 = -_21;
_19 = _41;
_30 = _20 ^ _20;
_5[_26] = _14 as u16;
_38 = _35;
_38 = _27 as usize;
_34[_26] = _2[_26] % _6[_26];
_4[_26] = _1.0[_26] & _2[_26];
_6 = _1.0;
_43[_26] = _16 as u16;
_23 = [_32[_26]];
_13.2 = [_13.0];
_28.fld0 = core::ptr::addr_of!(_25);
_11 = _29;
_19 = _15[_26] - _41;
_39 = _9;
_20 = _33[_26] as i128;
_40 = [_33[_26],_32[_26]];
_43 = [_13.3,_34[_26],_4[_26],_3.0[_26],_1.0[_26],_34[_26],_13.3,_1.0[_26]];
_14 = _36 as i8;
_1 = (_6,);
_20 = _30 & _30;
_42 = _16 << _6[_26];
_11 = _9;
_3.0 = [_4[_26],_4[_26],_6[_26],_4[_26],_34[_26],_1.0[_26],_13.3,_1.0[_26]];
_9 = _29;
match _24 {
0 => bb9,
1 => bb16,
2 => bb17,
26568 => bb19,
_ => bb18
}
}
bb16 = {
_28.fld0 = core::ptr::addr_of!(_25);
_21 = _17 | _17;
_18 = [344365180_u32,764639658_u32,1201882103_u32,3848491235_u32,3174405525_u32,2899981430_u32,2145654778_u32,4181631823_u32];
_26 = _8;
_8 = !_26;
_2[_26] = !_1.0[_26];
_27 = true as isize;
_5 = [_10[_26],_3.0[_26],_2[_26],_3.0[_26],_3.0[_26],_3.0[_26],_3.0[_26],_1.0[_26]];
_22 = [304999967493103485426468642755864889463_u128];
_30 = _20 * _20;
_17 = _21 & _21;
_3 = (_1.0,);
_11 = _9;
_31 = Adt44::Variant0 { fld0: 12297238000351177985_u64,fld1: _12 };
_18[_26] = _7[_26] - _7[_26];
_3.0[_26] = _2[_26];
_5[_26] = _3.0[_26];
_3 = (_1.0,);
_2 = _3.0;
_9 = _11;
_11 = _9;
_23 = [316973765853758340487389540184479011011_u128];
_1 = (_5,);
Goto(bb11)
}
bb17 = {
_14 = !88_i8;
_33[_26] = _32[_26] & _32[_26];
_1.0[_26] = _5[_26];
_15[_26] = !_19;
_1.0[_26] = _2[_26];
_15 = [_19,_19,_19];
_2[_26] = _5[_26];
_11 = _9;
_13.2 = [_13.0];
_32[_26] = _33[_26] + _33[_26];
_38 = _35 - _8;
_15 = [_19,_19,_19];
_23 = [_33[_26]];
Goto(bb14)
}
bb18 = {
place!(Field::<*mut [u8; 3]>(Variant(_31, 0), 1)) = _12;
_9 = _11;
_7 = [_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26]];
_3 = _1;
_1.0 = [_3.0[_26],_5[_26],_3.0[_26],_13.3,_3.0[_26],_10[_26],_5[_26],_13.3];
Goto(bb13)
}
bb19 = {
Goto(bb20)
}
bb20 = {
_5[_26] = !_2[_26];
match _26 {
0 => bb15,
1 => bb2,
3 => bb9,
4 => bb5,
5 => bb10,
6 => bb19,
2 => bb22,
_ => bb21
}
}
bb21 = {
_1.0 = [9126_u16,27788_u16,60938_u16,59480_u16,31139_u16,34635_u16,31860_u16,50872_u16];
_3.0 = _5;
_11 = _9;
_3.0 = _5;
_9 = _11;
_1.0 = _3.0;
_3 = (_5,);
_6 = _5;
_8 = 2890329087_u32 as usize;
_8 = 18_u8 as usize;
_7 = [760793062_u32,2228995379_u32,1833569676_u32,2656355993_u32,1371690884_u32,3528114710_u32,3281640141_u32,2044161854_u32];
_1 = (_5,);
_7 = [1864053148_u32,261783831_u32,1382911615_u32,290135370_u32,1210548162_u32,2870799147_u32,3630033278_u32,3962332152_u32];
_10 = [53948_u16,31915_u16,44882_u16,15940_u16,52766_u16,990_u16];
_10 = [15205_u16,37571_u16,21747_u16,26055_u16,52364_u16,2404_u16];
_2 = [47863_u16,5421_u16,426_u16,47607_u16,13841_u16,32545_u16,42854_u16,37928_u16];
_3.0 = [853_u16,10263_u16,31456_u16,31070_u16,51465_u16,37557_u16,767_u16,51952_u16];
_1 = (_6,);
_13.3 = 4808404238256422061_i64 as u16;
_5 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_2 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_7 = [2457658403_u32,2707276455_u32,2729317155_u32,333521517_u32,1062940853_u32,2383733572_u32,3861362598_u32,3365229194_u32];
_13.3 = 57348_u16;
Goto(bb3)
}
bb22 = {
_33[_26] = _32[_26];
_25 = false;
_31 = Adt44::Variant0 { fld0: 10827919762376424884_u64,fld1: _12 };
_4 = [_13.3,_2[_26],_1.0[_26],_43[_26],_13.3,_34[_26],_3.0[_26],_13.3];
_42 = -_16;
_1 = _3;
_16 = -_42;
_37 = -_36;
_13.1 = _13.2;
_2 = _1.0;
_13.0 = !(-115497055014598682_i64);
_10[_26] = !_2[_26];
_22 = [_33[_26]];
_18 = _7;
Goto(bb23)
}
bb23 = {
_5 = [_4[_26],_1.0[_26],_1.0[_26],_1.0[_26],_1.0[_26],_34[_26],_43[_26],_1.0[_26]];
_16 = _42 + _42;
_44 = !_32[_26];
_3.0[_26] = _10[_26];
_41 = (*_12)[_26] + (*_12)[_26];
place!(Field::<*mut [u8; 3]>(Variant(_31, 0), 1)) = _12;
_27 = _38 as isize;
_48 = _19 + _15[_26];
_7[_26] = !_18[_26];
_3 = _1;
_7 = [_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26]];
_1.0 = [_4[_26],_10[_26],_4[_26],_2[_26],_6[_26],_4[_26],_3.0[_26],_13.3];
_13.3 = _25 as u16;
_5 = [_43[_26],_6[_26],_43[_26],_2[_26],_3.0[_26],_10[_26],_2[_26],_6[_26]];
_11 = _29;
place!(Field::<u64>(Variant(_31, 0), 0)) = 6485982002130084224_u64;
_5[_26] = _43[_26];
_46 = _1.0;
_27 = _36;
_13.2 = _13.1;
_3.0 = [_43[_26],_43[_26],_6[_26],_34[_26],_34[_26],_10[_26],_1.0[_26],_10[_26]];
_38 = _8;
_33[_26] = _29 as u128;
_34[_26] = _32[_26] as u16;
_38 = _29 as usize;
_27 = -_37;
Goto(bb24)
}
bb24 = {
_32[_26] = _44 >> _5[_26];
_4[_26] = _2[_26] - _6[_26];
_49.2 = [_13.0];
_10[_26] = _5[_26] & _2[_26];
_35 = _13.0 as usize;
_5 = [_4[_26],_46[_26],_1.0[_26],_6[_26],_3.0[_26],_1.0[_26],_4[_26],_6[_26]];
_47 = _21 - _37;
_1.0 = [_6[_26],_46[_26],_6[_26],_43[_26],_6[_26],_34[_26],_46[_26],_46[_26]];
_9 = _39;
_32[_26] = _44;
_32[_26] = _44;
_10 = [_46[_26],_6[_26],_5[_26],_3.0[_26],_4[_26],_43[_26]];
_5 = [_1.0[_26],_4[_26],_4[_26],_34[_26],_4[_26],_1.0[_26],_34[_26],_4[_26]];
_32 = _33;
_28.fld0 = core::ptr::addr_of!(_25);
SetDiscriminant(_31, 1);
place!(Field::<u128>(Variant(_31, 1), 2)) = _30 as u128;
Goto(bb25)
}
bb25 = {
_48 = Field::<u128>(Variant(_31, 1), 2) as u8;
_28.fld0 = core::ptr::addr_of!(_25);
_15 = [_19,_19,_48];
_45 = _20 as f32;
_16 = !_42;
_49.3 = _2[_26] - _4[_26];
_42 = _16 & _16;
_44 = _18[_26] as u128;
_3.0 = _46;
_28.fld0 = core::ptr::addr_of!(_25);
_30 = -_20;
_52[_26] = _13.0 as u16;
_21 = _36;
_42 = !_16;
_49.0 = 18351317708908963813_u64 as i64;
_26 = _35;
_49.1 = [_49.0];
_58 = [_44,_44];
_33 = [_44,Field::<u128>(Variant(_31, 1), 2),Field::<u128>(Variant(_31, 1), 2),Field::<u128>(Variant(_31, 1), 2),Field::<u128>(Variant(_31, 1), 2),Field::<u128>(Variant(_31, 1), 2),Field::<u128>(Variant(_31, 1), 2),Field::<u128>(Variant(_31, 1), 2)];
place!(Field::<f32>(Variant(_31, 1), 0)) = _45 + _45;
_3.0 = [_49.3,_49.3,_49.3,_49.3,_49.3,_49.3,_49.3,_49.3];
_23 = _22;
_2 = _3.0;
match _24 {
0 => bb15,
1 => bb26,
26568 => bb28,
_ => bb27
}
}
bb26 = {
_8 = !2_usize;
_5 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_6 = [_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3,_13.3];
_5 = _1.0;
_13.1 = [(-4733576225882276046_i64)];
_8 = 7_usize;
_8 = 12945661795188774765_usize & 7_usize;
_13.2 = [3385183976126118962_i64];
_7 = [924489307_u32,1870152633_u32,3058218530_u32,3459414791_u32,3263307495_u32,2212817853_u32,2178400394_u32,1460980247_u32];
_14 = false as i8;
_13.2 = [(-3757050979495045471_i64)];
match _13.3 {
0 => bb2,
1 => bb4,
57348 => bb6,
_ => bb5
}
}
bb27 = {
_29 = _11;
_36 = -_21;
_19 = _41;
_30 = _20 ^ _20;
_5[_26] = _14 as u16;
_38 = _35;
_38 = _27 as usize;
_34[_26] = _2[_26] % _6[_26];
_4[_26] = _1.0[_26] & _2[_26];
_6 = _1.0;
_43[_26] = _16 as u16;
_23 = [_32[_26]];
_13.2 = [_13.0];
_28.fld0 = core::ptr::addr_of!(_25);
_11 = _29;
_19 = _15[_26] - _41;
_39 = _9;
_20 = _33[_26] as i128;
_40 = [_33[_26],_32[_26]];
_43 = [_13.3,_34[_26],_4[_26],_3.0[_26],_1.0[_26],_34[_26],_13.3,_1.0[_26]];
_14 = _36 as i8;
_1 = (_6,);
_20 = _30 & _30;
_42 = _16 << _6[_26];
_11 = _9;
_3.0 = [_4[_26],_4[_26],_6[_26],_4[_26],_34[_26],_1.0[_26],_13.3,_1.0[_26]];
_9 = _29;
match _24 {
0 => bb9,
1 => bb16,
2 => bb17,
26568 => bb19,
_ => bb18
}
}
bb28 = {
_15 = [_48,_48,_48];
_55 = _13.0 as isize;
_24 = 9298_i16;
_43 = [_49.3,_13.3,_49.3,_49.3,_49.3,_49.3,_49.3,_49.3];
_8 = !_35;
_28.fld0 = core::ptr::addr_of!(_25);
_21 = _37;
_50 = !_49.3;
_36 = _47 | _47;
_9 = _11;
_64 = _39;
_16 = !_42;
_26 = Field::<u128>(Variant(_31, 1), 2) as usize;
_28.fld0 = core::ptr::addr_of!(_25);
_12 = core::ptr::addr_of_mut!((*_12));
_54 = -_36;
_11 = _29;
_31 = Adt44::Variant0 { fld0: 3455077625287018643_u64,fld1: _12 };
_1.0 = [_50,_50,_49.3,_50,_49.3,_50,_49.3,_50];
Goto(bb29)
}
bb29 = {
place!(Field::<*mut [u8; 3]>(Variant(_31, 0), 1)) = _12;
_16 = _26 as i32;
_7 = [101395368_u32,3214214141_u32,1627020845_u32,4078943876_u32,1596379127_u32,2829565120_u32,1909282632_u32,688289501_u32];
_45 = 3218745855_u32 as f32;
_13 = _49;
_28.fld0 = core::ptr::addr_of!(_25);
_57 = -_24;
_3.0 = _6;
_33 = [_44,_44,_44,_44,_44,_44,_44,_44];
_40 = [_44,_44];
_65 = !_27;
Goto(bb30)
}
bb30 = {
_46 = [_13.3,_50,_49.3,_50,_49.3,_50,_50,_13.3];
_35 = _26;
_66 = _9 as isize;
_22 = [_44];
_31 = Adt44::Variant0 { fld0: 6017461726313374227_u64,fld1: _12 };
_25 = true;
_28.fld0 = core::ptr::addr_of!(_25);
_6 = [_13.3,_49.3,_50,_13.3,_49.3,_50,_13.3,_13.3];
_60 = _15;
_27 = _65 ^ _17;
_72 = _27;
_49 = _13;
_52 = _10;
_31 = Adt44::Variant0 { fld0: 8388424282411112798_u64,fld1: _12 };
_31 = Adt44::Variant0 { fld0: 3794783250699315899_u64,fld1: _12 };
_13 = (_49.0, _49.1, _49.2, _50);
_25 = true;
_2 = _4;
match _24 {
9298 => bb32,
_ => bb31
}
}
bb31 = {
_5 = [_4[_26],_1.0[_26],_1.0[_26],_1.0[_26],_1.0[_26],_34[_26],_43[_26],_1.0[_26]];
_16 = _42 + _42;
_44 = !_32[_26];
_3.0[_26] = _10[_26];
_41 = (*_12)[_26] + (*_12)[_26];
place!(Field::<*mut [u8; 3]>(Variant(_31, 0), 1)) = _12;
_27 = _38 as isize;
_48 = _19 + _15[_26];
_7[_26] = !_18[_26];
_3 = _1;
_7 = [_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26],_18[_26]];
_1.0 = [_4[_26],_10[_26],_4[_26],_2[_26],_6[_26],_4[_26],_3.0[_26],_13.3];
_13.3 = _25 as u16;
_5 = [_43[_26],_6[_26],_43[_26],_2[_26],_3.0[_26],_10[_26],_2[_26],_6[_26]];
_11 = _29;
place!(Field::<u64>(Variant(_31, 0), 0)) = 6485982002130084224_u64;
_5[_26] = _43[_26];
_46 = _1.0;
_27 = _36;
_13.2 = _13.1;
_3.0 = [_43[_26],_43[_26],_6[_26],_34[_26],_34[_26],_10[_26],_1.0[_26],_10[_26]];
_38 = _8;
_33[_26] = _29 as u128;
_34[_26] = _32[_26] as u16;
_38 = _29 as usize;
_27 = -_37;
Goto(bb24)
}
bb32 = {
_24 = _57 >> _17;
_63 = _52;
place!(Field::<u64>(Variant(_31, 0), 0)) = !7950382334723228677_u64;
_63 = [_13.3,_50,_49.3,_13.3,_50,_50];
_8 = _25 as usize;
SetDiscriminant(_31, 0);
_4 = [_49.3,_13.3,_50,_50,_13.3,_13.3,_50,_49.3];
_44 = 197954369707409091522125922925595627277_u128;
_7 = [1766962497_u32,2014689128_u32,1444243561_u32,1980553553_u32,1302840923_u32,443428198_u32,160820214_u32,528851638_u32];
_49.1 = [_13.0];
_71 = !4049231707_u32;
_13.1 = [_49.0];
_63 = [_50,_50,_13.3,_13.3,_13.3,_49.3];
_40 = _58;
_68 = core::ptr::addr_of_mut!(_45);
_17 = _44 as isize;
_7 = [_71,_71,_71,_71,_71,_71,_71,_71];
_41 = _48;
_71 = !3111959687_u32;
match _44 {
0 => bb5,
1 => bb4,
197954369707409091522125922925595627277 => bb34,
_ => bb33
}
}
bb33 = {
_11 = _9;
_21 = !_17;
_13.1 = _13.2;
_19 = !_15[_8];
_15[_8] = !_19;
_10[_8] = !_5[_8];
_13.2 = _13.1;
_3.0[_8] = _2[_8] - _2[_8];
_10[_8] = !_13.3;
_14 = _24 as i8;
Call(_17 = fn15(_10[_8], _10, _10[_8], _1, _3.0, _13.3, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb34 = {
_79.0 = _49.0 * _49.0;
_13.0 = _79.0;
Goto(bb35)
}
bb35 = {
_29 = _39;
_79 = (_13.0, _13.1, _49.2, _50);
_74 = [_44,_44];
_39 = _9;
_21 = _13.0 as isize;
_56 = [_79.0];
_42 = _16 + _16;
_41 = !_19;
_81 = _13.0 as u128;
place!(Field::<*mut [u8; 3]>(Variant(_31, 0), 1)) = _12;
_41 = _48;
match _44 {
0 => bb26,
1 => bb15,
2 => bb22,
197954369707409091522125922925595627277 => bb37,
_ => bb36
}
}
bb36 = {
_11 = _9;
_21 = !_17;
_13.1 = _13.2;
_19 = !_15[_8];
_15[_8] = !_19;
_10[_8] = !_5[_8];
_13.2 = _13.1;
_3.0[_8] = _2[_8] - _2[_8];
_10[_8] = !_13.3;
_14 = _24 as i8;
Call(_17 = fn15(_10[_8], _10, _10[_8], _1, _3.0, _13.3, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb37 = {
_75 = _5;
_7 = _18;
_28.fld0 = core::ptr::addr_of!(_25);
_70 = [_13.3];
_45 = _42 as f32;
_76 = [_44];
_49.0 = _79.0;
_77 = !_14;
_52 = [_13.3,_79.3,_50,_49.3,_79.3,_79.3];
_84 = _40;
_62 = _28.fld0;
_28 = Adt41 { fld0: _62 };
_18 = _7;
_62 = _28.fld0;
_82 = (*_68) * _45;
_33 = [_81,_81,_44,_81,_44,_44,_81,_81];
_31 = Adt44::Variant0 { fld0: 13146634315157944653_u64,fld1: _12 };
Goto(bb38)
}
bb38 = {
_31 = Adt44::Variant1 { fld0: (*_68),fld1: _13.3,fld2: _81 };
_86 = _11;
_15 = [_41,_48,_19];
_11 = _9;
_3.0 = [_50,_79.3,_50,_49.3,_49.3,Field::<u16>(Variant(_31, 1), 1),_50,Field::<u16>(Variant(_31, 1), 1)];
_60 = (*_12);
_86 = _11;
_49 = (_79.0, _79.2, _56, _79.3);
_78 = &_70;
_13.0 = !_79.0;
_83 = _44 as f64;
_86 = _11;
SetDiscriminant(_31, 0);
_32 = _33;
_79.3 = _50 * _50;
_26 = _35;
_78 = &(*_78);
_4 = [_13.3,_79.3,_50,_79.3,_49.3,_50,_50,_50];
_56 = [_49.0];
_22 = [_81];
match _44 {
0 => bb16,
1 => bb32,
2 => bb10,
197954369707409091522125922925595627277 => bb40,
_ => bb39
}
}
bb39 = {
_28.fld0 = core::ptr::addr_of!(_25);
_21 = _17 | _17;
_18 = [344365180_u32,764639658_u32,1201882103_u32,3848491235_u32,3174405525_u32,2899981430_u32,2145654778_u32,4181631823_u32];
_26 = _8;
_8 = !_26;
_2[_26] = !_1.0[_26];
_27 = true as isize;
_5 = [_10[_26],_3.0[_26],_2[_26],_3.0[_26],_3.0[_26],_3.0[_26],_3.0[_26],_1.0[_26]];
_22 = [304999967493103485426468642755864889463_u128];
_30 = _20 * _20;
_17 = _21 & _21;
_3 = (_1.0,);
_11 = _9;
_31 = Adt44::Variant0 { fld0: 12297238000351177985_u64,fld1: _12 };
_18[_26] = _7[_26] - _7[_26];
_3.0[_26] = _2[_26];
_5[_26] = _3.0[_26];
_3 = (_1.0,);
_2 = _3.0;
_9 = _11;
_11 = _9;
_23 = [316973765853758340487389540184479011011_u128];
_1 = (_5,);
Goto(bb11)
}
bb40 = {
_78 = &(*_78);
_91 = core::ptr::addr_of_mut!((*_68));
_74 = [_81,_44];
_38 = _35;
_43 = [_13.3,_13.3,_49.3,_50,_50,_79.3,_49.3,_49.3];
_27 = _24 as isize;
_34 = _3.0;
_79.3 = !_13.3;
_26 = _13.3 as usize;
_31 = Adt44::Variant0 { fld0: 2529875774290968166_u64,fld1: _12 };
_43 = [_79.3,_50,_13.3,_50,_13.3,_50,_50,_79.3];
_57 = _24;
_3.0 = _6;
place!(Field::<u64>(Variant(_31, 0), 0)) = _83 as u64;
SetDiscriminant(_31, 1);
_35 = _13.0 as usize;
_14 = _77;
_95 = [_49.0];
_12 = core::ptr::addr_of_mut!((*_12));
_58 = _84;
_82 = -(*_91);
_85 = _33;
_83 = _42 as f64;
match _44 {
0 => bb1,
1 => bb18,
197954369707409091522125922925595627277 => bb41,
_ => bb31
}
}
bb41 = {
_79.2 = _49.2;
_95 = [_79.0];
RET = Adt37::Variant1 { fld0: _38,fld1: 13702705246521540716_u64,fld2: _83,fld3: _48 };
_65 = _41 as isize;
place!(Field::<f64>(Variant(RET, 1), 2)) = -_83;
_35 = !Field::<usize>(Variant(RET, 1), 0);
_31 = Adt44::Variant1 { fld0: (*_68),fld1: _50,fld2: _81 };
_49.2 = _13.2;
_14 = _77 | _77;
_5 = _3.0;
_46 = [_50,Field::<u16>(Variant(_31, 1), 1),_79.3,_13.3,_50,_13.3,_79.3,_79.3];
_72 = Field::<usize>(Variant(RET, 1), 0) as isize;
_33 = [_44,_81,_81,_81,Field::<u128>(Variant(_31, 1), 2),_81,Field::<u128>(Variant(_31, 1), 2),_81];
place!(Field::<f64>(Variant(RET, 1), 2)) = _83;
_1 = _3;
_97 = [Field::<u128>(Variant(_31, 1), 2)];
_97 = [_44];
_81 = !_44;
_56 = [_79.0];
Goto(bb42)
}
bb42 = {
_12 = core::ptr::addr_of_mut!(_92);
_27 = _79.3 as isize;
_35 = !_26;
_14 = _77;
_3 = (_46,);
place!(Field::<u128>(Variant(_31, 1), 2)) = _44 / _44;
_103 = _71;
_94 = _16;
_17 = _36;
Goto(bb43)
}
bb43 = {
_81 = _44;
_106 = _41;
_74 = _40;
_1 = (_43,);
_79.1 = _49.2;
_20 = -_30;
_13.1 = [_49.0];
_78 = &(*_78);
_7 = [_103,_71,_71,_103,_103,_71,_103,_71];
_54 = _36 >> Field::<usize>(Variant(RET, 1), 0);
_29 = _9;
_66 = _54 + _72;
_48 = _106;
_28.fld0 = _62;
SetDiscriminant(_31, 0);
_107 = _35 as u64;
_47 = !_37;
_8 = _79.0 as usize;
_13.1 = _95;
_26 = !_38;
_26 = !_38;
_110 = _71 - _71;
place!(Field::<u64>(Variant(RET, 1), 1)) = Field::<usize>(Variant(RET, 1), 0) as u64;
_3 = _1;
Goto(bb44)
}
bb44 = {
Call(_113 = dump_var(7_usize, 75_usize, Move(_75), 9_usize, Move(_9), 65_usize, Move(_65), 71_usize, Move(_71)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_113 = dump_var(7_usize, 55_usize, Move(_55), 84_usize, Move(_84), 27_usize, Move(_27), 30_usize, Move(_30)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_113 = dump_var(7_usize, 97_usize, Move(_97), 95_usize, Move(_95), 1_usize, Move(_1), 23_usize, Move(_23)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_113 = dump_var(7_usize, 77_usize, Move(_77), 50_usize, Move(_50), 86_usize, Move(_86), 17_usize, Move(_17)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_113 = dump_var(7_usize, 47_usize, Move(_47), 19_usize, Move(_19), 39_usize, Move(_39), 35_usize, Move(_35)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_113 = dump_var(7_usize, 14_usize, Move(_14), 42_usize, Move(_42), 56_usize, Move(_56), 60_usize, Move(_60)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_113 = dump_var(7_usize, 16_usize, Move(_16), 103_usize, Move(_103), 66_usize, Move(_66), 107_usize, Move(_107)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_113 = dump_var(7_usize, 94_usize, Move(_94), 4_usize, Move(_4), 58_usize, Move(_58), 37_usize, Move(_37)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_113 = dump_var(7_usize, 20_usize, Move(_20), 44_usize, Move(_44), 76_usize, Move(_76), 2_usize, Move(_2)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_113 = dump_var(7_usize, 41_usize, Move(_41), 7_usize, Move(_7), 114_usize, _114, 114_usize, _114), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: u16,mut _2: u16,mut _3: ([u16; 8],),mut _4: u16,mut _5: char,mut _6: u16,mut _7: [u16; 8]) -> [u16; 8] {
mir! {
type RET = [u16; 8];
let _8: [i64; 1];
let _9: f64;
let _10: i128;
let _11: Adt51;
let _12: [u128; 1];
let _13: u8;
let _14: [u128; 2];
let _15: ();
let _16: ();
{
_2 = _4 >> _6;
RET = [_6,_6,_2,_2,_2,_1,_4,_2];
_3 = (RET,);
_8 = [367854800616523800_i64];
_5 = '\u{7e89e}';
_3 = (RET,);
_5 = '\u{92e17}';
_9 = 4_u8 as f64;
_3.0 = _7;
RET = _3.0;
_5 = '\u{7bd3a}';
_8 = [1536064560616894173_i64];
_6 = _2 >> _1;
RET = [_1,_6,_2,_6,_1,_2,_2,_2];
_3.0 = RET;
_10 = 15615158522386308787_usize as i128;
_5 = '\u{aa7a2}';
_3.0 = [_6,_6,_6,_2,_2,_6,_6,_6];
_3 = (RET,);
_2 = !_6;
_1 = _9 as u16;
_4 = 211_u8 as u16;
_9 = 3_usize as f64;
_10 = (-84318597812201135085303909362483855734_i128);
_8 = [(-235779578955672912_i64)];
_3 = (_7,);
RET = _7;
Call(_8 = fn9(_6, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _3.0;
_3.0 = [_6,_2,_6,_6,_6,_2,_6,_2];
_4 = !_2;
_9 = _10 as f64;
_6 = _2 * _4;
_10 = 95328766179242066613178874676465039272_i128 ^ 161311832805476837897667209708092523593_i128;
_3.0 = [_6,_6,_4,_2,_4,_4,_4,_6];
_1 = _2;
_2 = _6 & _6;
_4 = _2;
_6 = !_4;
_7 = _3.0;
_10 = 2127333494_i32 as i128;
_11 = Adt51::Variant0 { fld0: 478235457_u32 };
RET = _7;
_12 = [57498220988246822899384961221932825269_u128];
place!(Field::<u32>(Variant(_11, 0), 0)) = 2024206129_u32;
_8 = [(-3119965917955448490_i64)];
_3 = (RET,);
RET = [_2,_2,_2,_6,_2,_4,_6,_6];
_12 = [46247992495461835860216380227304227306_u128];
SetDiscriminant(_11, 1);
_1 = _2;
place!(Field::<Adt37>(Variant(_11, 1), 3)) = Adt37::Variant1 { fld0: 13712788248608620868_usize,fld1: 12995995565967508149_u64,fld2: _9,fld3: 175_u8 };
_4 = _2;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(8_usize, 4_usize, Move(_4), 2_usize, Move(_2), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_15 = dump_var(8_usize, 12_usize, Move(_12), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: u16,mut _2: [u16; 8]) -> [i64; 1] {
mir! {
type RET = [i64; 1];
let _3: bool;
let _4: [i64; 1];
let _5: Adt49;
let _6: u8;
let _7: *const *mut f64;
let _8: Adt50;
let _9: [u16; 1];
let _10: [u128; 2];
let _11: u128;
let _12: char;
let _13: Adt42;
let _14: u8;
let _15: [u32; 8];
let _16: Adt48;
let _17: [u128; 2];
let _18: Adt52;
let _19: f64;
let _20: Adt45;
let _21: i16;
let _22: isize;
let _23: [u128; 1];
let _24: (i64, [i64; 1], [i64; 1], u16);
let _25: ();
let _26: ();
{
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = [2870229321495119705_i64];
_3 = _1 <= _1;
RET = [7154794415603209894_i64];
RET = [(-4667774885752835755_i64)];
_4 = RET;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_3 = _1 < _1;
_1 = 59054_u16 | 56000_u16;
Goto(bb1)
}
bb1 = {
_5.fld3 = core::ptr::addr_of!(_3);
RET = [(-1952333633844828282_i64)];
_1 = 55271_u16 >> 1736459309_i32;
_4 = [10863654298952456_i64];
_6 = (-527302574_i32) as u8;
_5.fld5 = !4306579749954069100_usize;
_5.fld3 = core::ptr::addr_of!(_3);
Goto(bb2)
}
bb2 = {
_5.fld0 = _1 as f64;
_1 = 33622997468860146813191273777912274862_u128 as u16;
_4 = [1107101556297914739_i64];
Goto(bb3)
}
bb3 = {
_5.fld0 = 10058653667905069507_u64 as f64;
_1 = !13684_u16;
_5.fld0 = 28017_i16 as f64;
_1 = 99794233899715288253010842723930165952_u128 as u16;
RET = [7445770324400429362_i64];
_8.fld3 = !16975730915136377428_u64;
_8.fld1.fld0 = [_1,_1,_1,_1,_1,_1];
_5.fld3 = core::ptr::addr_of!(_3);
_5.fld4 = core::ptr::addr_of_mut!(_5.fld5);
_4 = [(-2108470403337345562_i64)];
_5.fld6 = _5.fld0 as u32;
_5.fld4 = core::ptr::addr_of_mut!(_5.fld5);
_6 = (-23911_i16) as u8;
Call(_8.fld3 = fn10(_5.fld3, _3, _2, _5.fld3, _5.fld3, _5.fld3, _5.fld3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5.fld6 = 3451641244_u32;
_8.fld3 = !4280071969334543764_u64;
_10 = [156832416550420800053621405535410836694_u128,217781436406779630008048016649618340022_u128];
_8.fld2 = Adt43::Variant0 { fld0: RET,fld1: '\u{42df7}' };
_10 = [308570591290078780299188263798241970962_u128,1183372838181979900099713483925083352_u128];
_3 = !false;
_10 = [212134997204061276039679033255258928817_u128,263614915267449660531513749659631039374_u128];
_3 = !true;
_8.fld1.fld0 = [_1,_1,_1,_1,_1,_1];
_3 = false ^ true;
RET = [7167673431144488472_i64];
_3 = true;
_3 = !false;
match _5.fld6 {
0 => bb3,
1 => bb5,
3451641244 => bb7,
_ => bb6
}
}
bb5 = {
_5.fld0 = 10058653667905069507_u64 as f64;
_1 = !13684_u16;
_5.fld0 = 28017_i16 as f64;
_1 = 99794233899715288253010842723930165952_u128 as u16;
RET = [7445770324400429362_i64];
_8.fld3 = !16975730915136377428_u64;
_8.fld1.fld0 = [_1,_1,_1,_1,_1,_1];
_5.fld3 = core::ptr::addr_of!(_3);
_5.fld4 = core::ptr::addr_of_mut!(_5.fld5);
_4 = [(-2108470403337345562_i64)];
_5.fld6 = _5.fld0 as u32;
_5.fld4 = core::ptr::addr_of_mut!(_5.fld5);
_6 = (-23911_i16) as u8;
Call(_8.fld3 = fn10(_5.fld3, _3, _2, _5.fld3, _5.fld3, _5.fld3, _5.fld3), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_5.fld0 = _1 as f64;
_1 = 33622997468860146813191273777912274862_u128 as u16;
_4 = [1107101556297914739_i64];
Goto(bb3)
}
bb7 = {
_5.fld5 = !7_usize;
_8.fld2 = Adt43::Variant0 { fld0: _4,fld1: '\u{536d1}' };
_6 = !71_u8;
RET = [(-7351926135813247738_i64)];
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = Field::<[i64; 1]>(Variant(_8.fld2, 0), 0);
Goto(bb8)
}
bb8 = {
_5.fld0 = 63874525786854792429831996962196224153_u128 as f64;
_1 = !54043_u16;
place!(Field::<char>(Variant(_8.fld2, 0), 1)) = '\u{a7c27}';
_9 = [_1];
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_12 = Field::<char>(Variant(_8.fld2, 0), 1);
place!(Field::<[i64; 1]>(Variant(_8.fld2, 0), 0)) = [(-6454390359377160157_i64)];
_10 = [135407243322781975493603022010320748617_u128,120415175195279043570748863768690508424_u128];
_6 = 85_u8;
_9 = [_1];
place!(Field::<char>(Variant(_8.fld2, 0), 1)) = _12;
_5.fld5 = _1 as usize;
_5.fld4 = core::ptr::addr_of_mut!(_5.fld5);
_4 = Field::<[i64; 1]>(Variant(_8.fld2, 0), 0);
_6 = 26_u8;
_13.fld1 = Adt39::Variant2 { fld0: _9,fld1: Field::<char>(Variant(_8.fld2, 0), 1),fld2: _5.fld6 };
_13.fld0 = [_1,_1,_1,_1,_1,_1];
_11 = 65974967940854616604896345500057118126_u128 & 141593401386669326263380675590068834418_u128;
place!(Field::<char>(Variant(_8.fld2, 0), 1)) = _12;
_8.fld2 = Adt43::Variant0 { fld0: _4,fld1: Field::<char>(Variant(_13.fld1, 2), 1) };
_8.fld1.fld1 = _13.fld1;
place!(Field::<char>(Variant(_13.fld1, 2), 1)) = Field::<char>(Variant(_8.fld2, 0), 1);
_8.fld3 = !10997063576571266476_u64;
match Field::<u32>(Variant(_8.fld1.fld1, 2), 2) {
0 => bb7,
1 => bb4,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
3451641244 => bb15,
_ => bb14
}
}
bb9 = {
_5.fld5 = !7_usize;
_8.fld2 = Adt43::Variant0 { fld0: _4,fld1: '\u{536d1}' };
_6 = !71_u8;
RET = [(-7351926135813247738_i64)];
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = Field::<[i64; 1]>(Variant(_8.fld2, 0), 0);
Goto(bb8)
}
bb10 = {
_5.fld0 = _1 as f64;
_1 = 33622997468860146813191273777912274862_u128 as u16;
_4 = [1107101556297914739_i64];
Goto(bb3)
}
bb11 = {
_5.fld0 = 10058653667905069507_u64 as f64;
_1 = !13684_u16;
_5.fld0 = 28017_i16 as f64;
_1 = 99794233899715288253010842723930165952_u128 as u16;
RET = [7445770324400429362_i64];
_8.fld3 = !16975730915136377428_u64;
_8.fld1.fld0 = [_1,_1,_1,_1,_1,_1];
_5.fld3 = core::ptr::addr_of!(_3);
_5.fld4 = core::ptr::addr_of_mut!(_5.fld5);
_4 = [(-2108470403337345562_i64)];
_5.fld6 = _5.fld0 as u32;
_5.fld4 = core::ptr::addr_of_mut!(_5.fld5);
_6 = (-23911_i16) as u8;
Call(_8.fld3 = fn10(_5.fld3, _3, _2, _5.fld3, _5.fld3, _5.fld3, _5.fld3), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_5.fld6 = 3451641244_u32;
_8.fld3 = !4280071969334543764_u64;
_10 = [156832416550420800053621405535410836694_u128,217781436406779630008048016649618340022_u128];
_8.fld2 = Adt43::Variant0 { fld0: RET,fld1: '\u{42df7}' };
_10 = [308570591290078780299188263798241970962_u128,1183372838181979900099713483925083352_u128];
_3 = !false;
_10 = [212134997204061276039679033255258928817_u128,263614915267449660531513749659631039374_u128];
_3 = !true;
_8.fld1.fld0 = [_1,_1,_1,_1,_1,_1];
_3 = false ^ true;
RET = [7167673431144488472_i64];
_3 = true;
_3 = !false;
match _5.fld6 {
0 => bb3,
1 => bb5,
3451641244 => bb7,
_ => bb6
}
}
bb13 = {
_5.fld3 = core::ptr::addr_of!(_3);
RET = [(-1952333633844828282_i64)];
_1 = 55271_u16 >> 1736459309_i32;
_4 = [10863654298952456_i64];
_6 = (-527302574_i32) as u8;
_5.fld5 = !4306579749954069100_usize;
_5.fld3 = core::ptr::addr_of!(_3);
Goto(bb2)
}
bb14 = {
_5.fld0 = _1 as f64;
_1 = 33622997468860146813191273777912274862_u128 as u16;
_4 = [1107101556297914739_i64];
Goto(bb3)
}
bb15 = {
_5.fld6 = Field::<u32>(Variant(_13.fld1, 2), 2);
place!(Field::<[u16; 1]>(Variant(_8.fld1.fld1, 2), 0)) = [_1];
_8.fld2 = Adt43::Variant0 { fld0: RET,fld1: Field::<char>(Variant(_8.fld1.fld1, 2), 1) };
_15 = [Field::<u32>(Variant(_8.fld1.fld1, 2), 2),Field::<u32>(Variant(_13.fld1, 2), 2),Field::<u32>(Variant(_8.fld1.fld1, 2), 2),_5.fld6,Field::<u32>(Variant(_8.fld1.fld1, 2), 2),Field::<u32>(Variant(_8.fld1.fld1, 2), 2),_5.fld6,Field::<u32>(Variant(_13.fld1, 2), 2)];
place!(Field::<u32>(Variant(_13.fld1, 2), 2)) = Field::<u32>(Variant(_8.fld1.fld1, 2), 2) >> _8.fld3;
place!(Field::<char>(Variant(_8.fld2, 0), 1)) = Field::<char>(Variant(_8.fld1.fld1, 2), 1);
_8.fld3 = 7254835237031321053_u64;
place!(Field::<[i64; 1]>(Variant(_8.fld2, 0), 0)) = RET;
_3 = !true;
_13 = _8.fld1;
_12 = Field::<char>(Variant(_13.fld1, 2), 1);
_11 = 103591805707599640622694467136698635332_u128 << _6;
SetDiscriminant(_8.fld1.fld1, 0);
_6 = !79_u8;
_5.fld3 = core::ptr::addr_of!(_3);
_17 = [_11,_11];
_5.fld6 = Field::<u32>(Variant(_13.fld1, 2), 2) << _5.fld5;
place!(Field::<[u128; 2]>(Variant(_8.fld1.fld1, 0), 0)) = _10;
_8.fld1.fld0 = _13.fld0;
RET = [5306212649854819445_i64];
place!(Field::<char>(Variant(_8.fld2, 0), 1)) = _12;
_1 = !59105_u16;
_21 = _8.fld3 as i16;
Goto(bb16)
}
bb16 = {
Call(_25 = dump_var(9_usize, 15_usize, Move(_15), 17_usize, Move(_17), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_25 = dump_var(9_usize, 10_usize, Move(_10), 9_usize, Move(_9), 26_usize, _26, 26_usize, _26), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *const bool,mut _2: bool,mut _3: [u16; 8],mut _4: *const bool,mut _5: *const bool,mut _6: *const bool,mut _7: *const bool) -> u64 {
mir! {
type RET = u64;
let _8: char;
let _9: *mut usize;
let _10: [u16; 8];
let _11: bool;
let _12: f64;
let _13: [u16; 1];
let _14: [u128; 2];
let _15: f32;
let _16: *mut f64;
let _17: [u16; 1];
let _18: f32;
let _19: [u8; 3];
let _20: u64;
let _21: Adt44;
let _22: [u16; 8];
let _23: *const *mut usize;
let _24: f32;
let _25: f32;
let _26: [u128; 1];
let _27: [u128; 2];
let _28: u32;
let _29: Adt49;
let _30: f32;
let _31: bool;
let _32: u32;
let _33: ();
let _34: ();
{
_10 = _3;
_3 = [33713_u16,59039_u16,3912_u16,12803_u16,14500_u16,17890_u16,10477_u16,16188_u16];
_7 = _1;
_5 = core::ptr::addr_of!(_2);
_10 = [37556_u16,17257_u16,13477_u16,18345_u16,6545_u16,58804_u16,20092_u16,55095_u16];
_11 = (*_5);
RET = 2389929053132546246_u64 >> 10734_u16;
_8 = '\u{c8ac4}';
_12 = 3_usize as f64;
_12 = 60175_u16 as f64;
_8 = '\u{fa835}';
_4 = core::ptr::addr_of!(_11);
_7 = _1;
_11 = (*_5) <= (*_5);
_8 = '\u{56e5d}';
_13 = [36450_u16];
_7 = core::ptr::addr_of!((*_5));
Call(_5 = fn11((*_7), _11, _7, (*_4), _10, (*_4), _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [14492_u16,8350_u16,63871_u16,16933_u16,38449_u16,46145_u16,7038_u16,30840_u16];
_12 = 12936078683814150981_usize as f64;
Goto(bb2)
}
bb2 = {
_1 = core::ptr::addr_of!((*_4));
_14 = [144289180597973100977243410117936055595_u128,141721789862314071638387773310447567163_u128];
_16 = core::ptr::addr_of_mut!(_12);
_5 = _4;
RET = 15135682216423853484_u64;
_3 = [46630_u16,8826_u16,55179_u16,32811_u16,27322_u16,62678_u16,18587_u16,50334_u16];
_15 = 4_usize as f32;
_6 = _7;
RET = !1265447842711573122_u64;
_7 = _4;
_6 = _4;
_18 = -_15;
Call(_5 = core::intrinsics::arith_offset(_7, 9223372036854775807_isize), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = core::ptr::addr_of!((*_1));
_11 = _2 & _2;
_3 = [58571_u16,23106_u16,12801_u16,22447_u16,48045_u16,48788_u16,217_u16,10078_u16];
_6 = _7;
_2 = (*_7) >= (*_4);
_18 = _15;
Call(_2 = fn12(_5, _5, _1, _5, _5, _5, _5, _5, _5, _5, (*_4), _5, _5, _5, _6, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16 = core::ptr::addr_of_mut!(_12);
_16 = core::ptr::addr_of_mut!((*_16));
_6 = core::ptr::addr_of!(_2);
_2 = (*_4);
_2 = (*_1);
RET = (-85_isize) as u64;
_6 = core::ptr::addr_of!((*_7));
_7 = _5;
_2 = !(*_1);
_14 = [128335237696634379664155327765341160847_u128,273323070856693534665897794027077739548_u128];
_13 = [34328_u16];
_14 = [327815931939115600201022053323668603766_u128,49728064307030526505534439930170991256_u128];
_14 = [281818809370585842347405798854894791652_u128,326194180991651400962895618432445900610_u128];
_12 = 7_u8 as f64;
_8 = '\u{103297}';
_4 = core::ptr::addr_of!(_11);
_20 = RET;
Goto(bb5)
}
bb5 = {
_17 = _13;
_2 = !(*_4);
_6 = _7;
_13 = _17;
_8 = '\u{160ba}';
_10 = [58424_u16,34562_u16,37539_u16,44903_u16,40344_u16,23725_u16,13239_u16,57808_u16];
_15 = _18;
_22 = [1949_u16,5316_u16,22735_u16,47208_u16,13697_u16,4636_u16,36288_u16,54949_u16];
_4 = core::ptr::addr_of!(_2);
Call(_4 = core::intrinsics::arith_offset(_5, (-9223372036854775807_isize)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = 4190644033_u32 as u64;
_1 = core::ptr::addr_of!(_11);
_1 = core::ptr::addr_of!((*_1));
_22 = [33252_u16,37523_u16,56650_u16,21395_u16,34999_u16,63030_u16,56986_u16,33020_u16];
_8 = '\u{5b1d7}';
_14 = [281655684885229251036953884845852953842_u128,48304183984603080245317598650341336605_u128];
_23 = core::ptr::addr_of!(_9);
_23 = core::ptr::addr_of!((*_23));
_2 = !(*_1);
_18 = -_15;
_17 = [48683_u16];
RET = _20;
_23 = core::ptr::addr_of!((*_23));
_22 = _10;
_1 = core::ptr::addr_of!((*_1));
_15 = _18 * _18;
_14 = [59368445570375443450006126766834098519_u128,189730826960361074964477328181512078291_u128];
_11 = _2;
_23 = core::ptr::addr_of!((*_23));
RET = _15 as u64;
_24 = _15;
Goto(bb7)
}
bb7 = {
_8 = '\u{37d8d}';
_21 = Adt44::Variant1 { fld0: _24,fld1: 13800_u16,fld2: 316926224153698814273668590443567723976_u128 };
RET = !_20;
_23 = core::ptr::addr_of!((*_23));
_17 = _13;
place!(Field::<u16>(Variant(_21, 1), 1)) = 17390_u16;
_14 = [69922401549426424517385337032045420588_u128,162297913382572246395308261535472653882_u128];
match Field::<u16>(Variant(_21, 1), 1) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb9,
17390 => bb11,
_ => bb10
}
}
bb8 = {
RET = 4190644033_u32 as u64;
_1 = core::ptr::addr_of!(_11);
_1 = core::ptr::addr_of!((*_1));
_22 = [33252_u16,37523_u16,56650_u16,21395_u16,34999_u16,63030_u16,56986_u16,33020_u16];
_8 = '\u{5b1d7}';
_14 = [281655684885229251036953884845852953842_u128,48304183984603080245317598650341336605_u128];
_23 = core::ptr::addr_of!(_9);
_23 = core::ptr::addr_of!((*_23));
_2 = !(*_1);
_18 = -_15;
_17 = [48683_u16];
RET = _20;
_23 = core::ptr::addr_of!((*_23));
_22 = _10;
_1 = core::ptr::addr_of!((*_1));
_15 = _18 * _18;
_14 = [59368445570375443450006126766834098519_u128,189730826960361074964477328181512078291_u128];
_11 = _2;
_23 = core::ptr::addr_of!((*_23));
RET = _15 as u64;
_24 = _15;
Goto(bb7)
}
bb9 = {
_17 = _13;
_2 = !(*_4);
_6 = _7;
_13 = _17;
_8 = '\u{160ba}';
_10 = [58424_u16,34562_u16,37539_u16,44903_u16,40344_u16,23725_u16,13239_u16,57808_u16];
_15 = _18;
_22 = [1949_u16,5316_u16,22735_u16,47208_u16,13697_u16,4636_u16,36288_u16,54949_u16];
_4 = core::ptr::addr_of!(_2);
Call(_4 = core::intrinsics::arith_offset(_5, (-9223372036854775807_isize)), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_7 = core::ptr::addr_of!((*_1));
_11 = _2 & _2;
_3 = [58571_u16,23106_u16,12801_u16,22447_u16,48045_u16,48788_u16,217_u16,10078_u16];
_6 = _7;
_2 = (*_7) >= (*_4);
_18 = _15;
Call(_2 = fn12(_5, _5, _1, _5, _5, _5, _5, _5, _5, _5, (*_4), _5, _5, _5, _6, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_17 = _13;
RET = !_20;
_22 = [Field::<u16>(Variant(_21, 1), 1),Field::<u16>(Variant(_21, 1), 1),Field::<u16>(Variant(_21, 1), 1),Field::<u16>(Variant(_21, 1), 1),Field::<u16>(Variant(_21, 1), 1),Field::<u16>(Variant(_21, 1), 1),Field::<u16>(Variant(_21, 1), 1),Field::<u16>(Variant(_21, 1), 1)];
_23 = core::ptr::addr_of!(_9);
place!(Field::<u128>(Variant(_21, 1), 2)) = 225919797978927763869394315762460397710_u128;
_19 = [109_u8,146_u8,4_u8];
_15 = _18 - _18;
_19 = [181_u8,66_u8,214_u8];
_18 = (-27870_i16) as f32;
_19 = [179_u8,152_u8,33_u8];
_2 = (*_1) < (*_1);
RET = _20;
_3 = _10;
_11 = _2;
_25 = -_18;
place!(Field::<u16>(Variant(_21, 1), 1)) = !21639_u16;
Goto(bb12)
}
bb12 = {
_17 = [Field::<u16>(Variant(_21, 1), 1)];
_18 = (*_16) as f32;
_3 = _10;
_12 = Field::<u16>(Variant(_21, 1), 1) as f64;
_30 = _15 - Field::<f32>(Variant(_21, 1), 0);
Call(_18 = fn13(_4, _4, _4, _4, _4, _4, _5, _4, _6, _4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_10 = _3;
Goto(bb14)
}
bb14 = {
_29.fld6 = !3842277426_u32;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(10_usize, 17_usize, Move(_17), 14_usize, Move(_14), 10_usize, Move(_10), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(10_usize, 22_usize, Move(_22), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: bool,mut _2: bool,mut _3: *const bool,mut _4: bool,mut _5: [u16; 8],mut _6: bool,mut _7: *const bool) -> *const bool {
mir! {
type RET = *const bool;
let _8: f32;
let _9: usize;
let _10: (i64, [i64; 1], [i64; 1], u16);
let _11: [u16; 1];
let _12: ();
let _13: ();
{
_1 = _6 >= _4;
_1 = _6 | _4;
_7 = core::ptr::addr_of!(_2);
RET = _7;
_4 = !(*_7);
_2 = _6;
_2 = _4;
RET = core::ptr::addr_of!((*_7));
RET = core::ptr::addr_of!((*_7));
_3 = RET;
_7 = RET;
_1 = !(*RET);
_1 = _4 < (*RET);
_5 = [2969_u16,26548_u16,50498_u16,43101_u16,11204_u16,17477_u16,65409_u16,9294_u16];
_8 = 14129745302792137986_u64 as f32;
_9 = 10783296347423931699_usize;
_10.1 = [4945031987987401051_i64];
_4 = !_2;
_3 = core::ptr::addr_of!((*RET));
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(11_usize, 5_usize, Move(_5), 6_usize, Move(_6), 4_usize, Move(_4), 13_usize, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool,mut _4: *const bool,mut _5: *const bool,mut _6: *const bool,mut _7: *const bool,mut _8: *const bool,mut _9: *const bool,mut _10: *const bool,mut _11: bool,mut _12: *const bool,mut _13: *const bool,mut _14: *const bool,mut _15: *const bool,mut _16: *const bool) -> bool {
mir! {
type RET = bool;
let _17: i8;
let _18: char;
let _19: Adt51;
let _20: ();
let _21: ();
{
_17 = 145325460849509219814559398229411510393_u128 as i8;
RET = _11 | _11;
_3 = core::ptr::addr_of!(_11);
_17 = (*_3) as i8;
_16 = core::ptr::addr_of!((*_3));
_15 = core::ptr::addr_of!(_11);
RET = (*_3);
_3 = core::ptr::addr_of!((*_16));
_15 = core::ptr::addr_of!((*_16));
_16 = core::ptr::addr_of!((*_15));
_16 = _6;
RET = !(*_15);
_17 = !(-39_i8);
RET = (*_3) != (*_15);
_18 = '\u{b2a18}';
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(12_usize, 18_usize, Move(_18), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool,mut _4: *const bool,mut _5: *const bool,mut _6: *const bool,mut _7: *const bool,mut _8: *const bool,mut _9: *const bool,mut _10: *const bool) -> f32 {
mir! {
type RET = f32;
let _11: [u128; 2];
let _12: bool;
let _13: bool;
let _14: i32;
let _15: char;
let _16: f64;
let _17: [u8; 3];
let _18: Adt40;
let _19: [i64; 1];
let _20: f32;
let _21: Adt45;
let _22: [u8; 3];
let _23: isize;
let _24: isize;
let _25: f32;
let _26: char;
let _27: f64;
let _28: Adt51;
let _29: f32;
let _30: ([u16; 8],);
let _31: [u128; 8];
let _32: i16;
let _33: [u16; 6];
let _34: (i64, [i64; 1], [i64; 1], u16);
let _35: *const *mut usize;
let _36: char;
let _37: Adt39;
let _38: [u128; 1];
let _39: (i64, [i64; 1], [i64; 1], u16);
let _40: *mut usize;
let _41: [u16; 6];
let _42: char;
let _43: [u32; 8];
let _44: f32;
let _45: [u8; 3];
let _46: [u16; 1];
let _47: u32;
let _48: [i64; 1];
let _49: char;
let _50: (i64, [i64; 1], [i64; 1], u16);
let _51: Adt43;
let _52: u64;
let _53: char;
let _54: bool;
let _55: [u16; 1];
let _56: ();
let _57: ();
{
_11 = [105326705684142790610280291125241176750_u128,145517126044060105836783524307107599191_u128];
_11 = [323569818302062947283816563009281718537_u128,309734885193831275020683293913433836678_u128];
RET = 71082598673998115526330287262922111128_i128 as f32;
RET = 8523_u16 as f32;
RET = (-58054916938885802843654169654699190593_i128) as f32;
_11 = [135004598234769597288467357256693794764_u128,292924806087090158881108533499361257334_u128];
_12 = true;
_11 = [108100644285118334725334720757042475316_u128,335251180701623108738497007813653878749_u128];
_11 = [3383902546750862440601077099130809298_u128,46561020106776254811283295163871553197_u128];
RET = 6430452346288805467_i64 as f32;
RET = (-96_i8) as f32;
RET = 5782179743547264062_i64 as f32;
_12 = RET > RET;
_11 = [244845352838209680059876087435920050170_u128,75965906631072648769101816923034130824_u128];
_14 = 617275124_i32 + (-1537159248_i32);
RET = _14 as f32;
RET = (-9223372036854775808_isize) as f32;
_14 = !1469220517_i32;
_13 = _12;
_12 = _13 != _13;
RET = 25_i8 as f32;
_12 = !_13;
_14 = (-37433047006198804802669748110727153551_i128) as i32;
_13 = _12;
Goto(bb1)
}
bb1 = {
_17 = [139_u8,13_u8,212_u8];
_15 = '\u{2ac27}';
_12 = _13;
_13 = !_12;
_12 = _13;
_17 = [199_u8,84_u8,39_u8];
_11 = [42749272643452616086583731446644923614_u128,102459831837123681337900639951037099960_u128];
_11 = [206163555989856240109791396155957467974_u128,56278713879561334386058430559246016040_u128];
_16 = 47_u8 as f64;
_14 = 30_i8 as i32;
_15 = '\u{b1083}';
_15 = '\u{61a74}';
_11 = [31834629716563353581058674289358508008_u128,331013806869455280251652159804780634106_u128];
RET = 55044_u16 as f32;
_17 = [56_u8,247_u8,205_u8];
_13 = !_12;
_11 = [121691503241459564346268254346966007218_u128,17050515725932857374684563676980150678_u128];
Goto(bb2)
}
bb2 = {
_13 = !_12;
_11 = [196746955903667932962414902017048827900_u128,190802974946862224975647495518161939639_u128];
_16 = 168197482_u32 as f64;
RET = 118_u8 as f32;
_12 = !_13;
_17 = [82_u8,82_u8,186_u8];
_11 = [101398789397609636668749817364882959222_u128,79695173621111408181519953938233567599_u128];
_22 = [248_u8,1_u8,93_u8];
_11 = [137273223914710967546978030217537579738_u128,125431290619562125259243922648237912779_u128];
RET = 16432_u16 as f32;
_12 = !_13;
RET = 118_i8 as f32;
_14 = 1499406583_i32;
_19 = [(-3203721159821349462_i64)];
_22 = [154_u8,212_u8,221_u8];
Goto(bb3)
}
bb3 = {
_20 = (-88306099624313572359482371958528510548_i128) as f32;
_19 = [7280803765423821958_i64];
_23 = (-127_isize);
_22 = _17;
RET = _20;
Goto(bb4)
}
bb4 = {
RET = -_20;
_16 = 35135_u16 as f64;
_22 = [183_u8,137_u8,5_u8];
_25 = (-107_i8) as f32;
RET = _16 as f32;
_19 = [(-7109407979667444136_i64)];
_14 = 50306_u16 as i32;
_12 = _13 & _13;
_26 = _15;
_24 = _20 as isize;
_17 = _22;
_20 = -_25;
_16 = 27694_i16 as f64;
_19 = [4363506109952152023_i64];
Call(_12 = fn14(_2, _4, _10), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = -_25;
match _23 {
340282366920938463463374607431768211329 => bb7,
_ => bb6
}
}
bb6 = {
RET = -_20;
_16 = 35135_u16 as f64;
_22 = [183_u8,137_u8,5_u8];
_25 = (-107_i8) as f32;
RET = _16 as f32;
_19 = [(-7109407979667444136_i64)];
_14 = 50306_u16 as i32;
_12 = _13 & _13;
_26 = _15;
_24 = _20 as isize;
_17 = _22;
_20 = -_25;
_16 = 27694_i16 as f64;
_19 = [4363506109952152023_i64];
Call(_12 = fn14(_2, _4, _10), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_26 = _15;
_22 = [88_u8,55_u8,221_u8];
Goto(bb8)
}
bb8 = {
_22 = [79_u8,210_u8,11_u8];
_26 = _15;
_11 = [274301109772477515880519185601391704895_u128,56723352589066013193546422301026591620_u128];
Goto(bb9)
}
bb9 = {
_13 = _12;
_19 = [(-5076805991810283657_i64)];
_29 = (-32022_i16) as f32;
_15 = _26;
_26 = _15;
_26 = _15;
_23 = _24;
_13 = _12 ^ _12;
_33 = [21983_u16,21541_u16,5051_u16,9630_u16,58025_u16,24478_u16];
_30.0 = [34974_u16,46957_u16,10393_u16,41988_u16,58107_u16,10197_u16,37096_u16,18727_u16];
_20 = _14 as f32;
_20 = _24 as f32;
_17 = [68_u8,73_u8,194_u8];
_32 = _20 as i16;
_31 = [38276061540178529142175779080691928279_u128,157467301105565959649754704106546691917_u128,102865830316232372465640545842383092167_u128,88943514816008858169514645540460457026_u128,201441367631153020412948965056448511781_u128,210621582964555680228098581253782351336_u128,307244104353826705948456619169439041158_u128,9996629518569567521153205964467084040_u128];
_34.0 = -1509401264685597003_i64;
_31 = [42852821656243276761419325656490988351_u128,176699956338105368949602965241054950881_u128,116162154301781091645301502184189418038_u128,111012785449448509966299105407977277173_u128,255891373931051151445911477318503281592_u128,182976496211381838108526168932856910318_u128,86043361862794953292022481235191736174_u128,245016344854545310828809050175044458884_u128];
_24 = _14 as isize;
_20 = -_25;
_19 = [_34.0];
_12 = !_13;
_16 = 9837888942560759337_u64 as f64;
_36 = _15;
Goto(bb10)
}
bb10 = {
_31 = [113153788531376935236694230008914039823_u128,323790070421554741166282989358152570964_u128,302875451502927621251640522295659234102_u128,45403403944497178979981434858840436625_u128,241874849996174379103693922473454117491_u128,40416762515840650701756430934934732111_u128,273521363445893434277656493003837732550_u128,336305594698977183178324139153806682298_u128];
_34.0 = 8849873393685801637_i64;
_16 = _32 as f64;
_15 = _26;
_24 = _23 ^ _23;
_34.1 = [_34.0];
_34 = ((-812931221468148806_i64), _19, _19, 31035_u16);
_23 = _24;
_12 = _13;
_23 = -_24;
_38 = [58661864738768121450184946220348732536_u128];
_24 = _23 & _23;
_20 = _24 as f32;
_39.2 = [_34.0];
_33 = [_34.3,_34.3,_34.3,_34.3,_34.3,_34.3];
_34.0 = _14 as i64;
_39.1 = [_34.0];
_17 = _22;
_11 = [111171614181693361727282047373126655354_u128,214381459763705143217509081517852939403_u128];
Goto(bb11)
}
bb11 = {
RET = _25;
_29 = -_20;
_28 = Adt51::Variant0 { fld0: 1575173557_u32 };
_28 = Adt51::Variant0 { fld0: 965709300_u32 };
match _34.3 {
0 => bb7,
1 => bb5,
2 => bb6,
31035 => bb12,
_ => bb8
}
}
bb12 = {
_39 = _34;
_34.2 = [_39.0];
_26 = _15;
RET = _29;
_12 = _13;
_17 = [83_u8,211_u8,210_u8];
_12 = _13;
RET = _29;
_39.0 = _34.0 & _34.0;
_36 = _26;
_39.3 = _34.3 / _34.3;
Call(_34.0 = core::intrinsics::bswap(_39.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_16 = _24 as f64;
_20 = _29;
_19 = [_39.0];
_31 = [240253754214633728891750855617268519214_u128,241942349215054646973592245090560117364_u128,241722757431343003086780214744845307389_u128,131561041683092784099197974747373224840_u128,339893745149140233838757353764007755305_u128,21113973973578652909673851772043173609_u128,154177177541357656749834379464649081595_u128,70510765106050011706613121293305354203_u128];
_41 = [_39.3,_39.3,_34.3,_39.3,_39.3,_39.3];
RET = _25 + _20;
_39.2 = _39.1;
_41 = _33;
_30.0 = [_39.3,_39.3,_34.3,_39.3,_39.3,_39.3,_39.3,_39.3];
_31 = [253398435653105189016537189166360711119_u128,36989871417252789813167533437677321866_u128,246909714838736309262797415679229169916_u128,148202758231204746530725117736939492628_u128,27049858732913819170667622464775347194_u128,233306060740913336049230849745734745599_u128,18993088197588468761853592077046449072_u128,98441046461721008724438566790249678591_u128];
_34.0 = _39.0;
_44 = _24 as f32;
_39.1 = [_39.0];
_33 = [_39.3,_39.3,_34.3,_34.3,_39.3,_39.3];
_15 = _36;
place!(Field::<u32>(Variant(_28, 0), 0)) = 1223055192_u32;
_43 = [Field::<u32>(Variant(_28, 0), 0),Field::<u32>(Variant(_28, 0), 0),Field::<u32>(Variant(_28, 0), 0),Field::<u32>(Variant(_28, 0), 0),Field::<u32>(Variant(_28, 0), 0),Field::<u32>(Variant(_28, 0), 0),Field::<u32>(Variant(_28, 0), 0),Field::<u32>(Variant(_28, 0), 0)];
_33 = _41;
SetDiscriminant(_28, 2);
_39.0 = 226_u8 as i64;
_17 = [219_u8,181_u8,199_u8];
_35 = core::ptr::addr_of!(_40);
_23 = _24;
match _34.3 {
0 => bb11,
1 => bb10,
2 => bb12,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb9,
31035 => bb15,
_ => bb14
}
}
bb14 = {
RET = -_20;
_16 = 35135_u16 as f64;
_22 = [183_u8,137_u8,5_u8];
_25 = (-107_i8) as f32;
RET = _16 as f32;
_19 = [(-7109407979667444136_i64)];
_14 = 50306_u16 as i32;
_12 = _13 & _13;
_26 = _15;
_24 = _20 as isize;
_17 = _22;
_20 = -_25;
_16 = 27694_i16 as f64;
_19 = [4363506109952152023_i64];
Call(_12 = fn14(_2, _4, _10), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_42 = _36;
_34.3 = !_39.3;
_16 = 69354956154964588141279924404083299162_i128 as f64;
_45 = [241_u8,246_u8,89_u8];
_41 = [_39.3,_39.3,_39.3,_39.3,_34.3,_34.3];
_34.0 = _39.0 & _39.0;
_49 = _42;
_39.1 = [_39.0];
place!(Field::<Adt46>(Variant(_28, 2), 0)).fld1 = [_39.3];
place!(Field::<Adt46>(Variant(_28, 2), 0)).fld1 = [_34.3];
_21 = Adt45::Variant0 { fld0: _13,fld1: 6_usize };
_54 = !Field::<bool>(Variant(_21, 0), 0);
_34 = (_39.0, _39.1, _39.2, _39.3);
_53 = _49;
_48 = _34.2;
Goto(bb16)
}
bb16 = {
Call(_56 = dump_var(13_usize, 33_usize, Move(_33), 26_usize, Move(_26), 38_usize, Move(_38), 54_usize, Move(_54)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_56 = dump_var(13_usize, 17_usize, Move(_17), 12_usize, Move(_12), 22_usize, Move(_22), 45_usize, Move(_45)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_56 = dump_var(13_usize, 24_usize, Move(_24), 49_usize, Move(_49), 19_usize, Move(_19), 15_usize, Move(_15)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(13_usize, 48_usize, Move(_48), 57_usize, _57, 57_usize, _57, 57_usize, _57), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool) -> bool {
mir! {
type RET = bool;
let _4: u32;
let _5: Adt43;
let _6: [u16; 1];
let _7: f32;
let _8: i16;
let _9: [u16; 1];
let _10: Adt49;
let _11: i16;
let _12: [u32; 8];
let _13: Adt37;
let _14: f32;
let _15: i16;
let _16: (i64, [i64; 1], [i64; 1], u16);
let _17: isize;
let _18: isize;
let _19: usize;
let _20: Adt41;
let _21: u8;
let _22: (i64, [i64; 1], [i64; 1], u16);
let _23: ();
let _24: ();
{
RET = !false;
RET = 52136_u16 == 132_u16;
RET = false;
_4 = 2250604166_u32 * 75152729_u32;
_4 = 111367847390254525201543460619681884377_u128 as u32;
_4 = 1964097719_u32 >> 11050092726212699798_u64;
RET = true;
RET = false;
_4 = 4009551832_u32;
RET = _4 <= _4;
_4 = 5242788810856054045_u64 as u32;
RET = _4 != _4;
_4 = 58_u8 as u32;
_4 = 1187359289_u32;
RET = !false;
RET = !true;
_7 = (-1037587077_i32) as f32;
Goto(bb1)
}
bb1 = {
_6 = [23825_u16];
_6 = [12045_u16];
_6 = [8490_u16];
RET = _7 <= _7;
RET = !false;
_7 = 19647884532247841589610769712270632380_i128 as f32;
RET = false ^ true;
_7 = 4_usize as f32;
_7 = 54271_u16 as f32;
_6 = [43939_u16];
RET = false;
RET = !true;
_4 = _7 as u32;
_7 = 8_u8 as f32;
RET = false | false;
RET = true;
_8 = 27366_i16;
RET = _7 != _7;
RET = !false;
RET = _8 < _8;
_6 = [39901_u16];
_6 = [36381_u16];
_7 = 225738386842103121211900110474326144775_u128 as f32;
RET = !false;
match _8 {
0 => bb2,
27366 => bb4,
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
_10.fld4 = core::ptr::addr_of_mut!(_10.fld5);
_4 = _8 as u32;
_10.fld4 = core::ptr::addr_of_mut!(_10.fld5);
_10.fld4 = core::ptr::addr_of_mut!(_10.fld5);
_9 = [37076_u16];
_10.fld0 = _4 as f64;
_10.fld3 = _1;
_10.fld5 = _7 as usize;
_9 = _6;
match _8 {
0 => bb2,
27366 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
_10.fld4 = core::ptr::addr_of_mut!(_10.fld5);
_6 = [57357_u16];
_10.fld6 = _4;
_10.fld0 = 141_u8 as f64;
_10.fld0 = _4 as f64;
_11 = _8;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb8,
27366 => bb10,
_ => bb9
}
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
_6 = [29506_u16];
_7 = _10.fld6 as f32;
_10.fld0 = _7 as f64;
RET = !false;
_7 = (-3846212130245434632_i64) as f32;
_10.fld0 = 21145_u16 as f64;
_10.fld4 = core::ptr::addr_of_mut!(_10.fld5);
_6 = [39095_u16];
_10.fld5 = RET as usize;
match _11 {
0 => bb5,
1 => bb6,
2 => bb3,
3 => bb4,
27366 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_12 = [_10.fld6,_10.fld6,_10.fld6,_10.fld6,_10.fld6,_4,_10.fld6,_10.fld6];
_7 = _4 as f32;
_10.fld4 = core::ptr::addr_of_mut!(_10.fld5);
_9 = _6;
_12 = [_4,_10.fld6,_10.fld6,_10.fld6,_10.fld6,_4,_10.fld6,_10.fld6];
_11 = -_8;
_9 = [47822_u16];
_4 = _10.fld6 * _10.fld6;
_15 = !_11;
_16.3 = 45295_u16;
_16.2 = [(-2418595917187367152_i64)];
_16.1 = _16.2;
_16.0 = 332830697635501543753255081665408132724_u128 as i64;
_16.2 = [_16.0];
_16.1 = [_16.0];
_5 = Adt43::Variant0 { fld0: _16.2,fld1: '\u{a9ba1}' };
Goto(bb13)
}
bb13 = {
_10.fld4 = core::ptr::addr_of_mut!(_10.fld5);
_6 = _9;
match _8 {
0 => bb1,
1 => bb2,
2 => bb7,
27366 => bb14,
_ => bb6
}
}
bb14 = {
_16.2 = [_16.0];
_16.3 = 1439556755_i32 as u16;
_14 = _7;
_18 = !14_isize;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(14_usize, 16_usize, Move(_16), 15_usize, Move(_15), 8_usize, Move(_8), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: u16,mut _2: [u16; 6],mut _3: u16,mut _4: ([u16; 8],),mut _5: [u16; 8],mut _6: u16,mut _7: [u16; 8]) -> isize {
mir! {
type RET = isize;
let _8: u32;
let _9: u128;
let _10: [u128; 8];
let _11: [u128; 1];
let _12: [u16; 8];
let _13: [u16; 6];
let _14: [u16; 6];
let _15: [u128; 2];
let _16: [u8; 3];
let _17: [u32; 8];
let _18: Adt40;
let _19: isize;
let _20: Adt41;
let _21: Adt48;
let _22: char;
let _23: [u16; 1];
let _24: Adt52;
let _25: [u128; 2];
let _26: isize;
let _27: f32;
let _28: Adt52;
let _29: [u16; 1];
let _30: [u16; 1];
let _31: Adt51;
let _32: u32;
let _33: *const *mut f64;
let _34: Adt37;
let _35: [i64; 1];
let _36: f64;
let _37: Adt46;
let _38: ();
let _39: ();
{
RET = !9223372036854775807_isize;
_4.0 = _7;
_6 = 13420310617721964222_u64 as u16;
_8 = '\u{ef9c4}' as u32;
_4.0 = [_1,_3,_3,_3,_3,_3,_1,_1];
RET = 40_isize * 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
RET = 64_isize;
RET = -(-9223372036854775808_isize);
RET = (-9223372036854775808_isize) >> _1;
RET = -9223372036854775807_isize;
RET = (-9223372036854775808_isize) << _1;
RET = -(-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
RET = 377305571_i32 as isize;
_3 = _1 ^ _1;
_4 = (_5,);
_2 = [_3,_1,_1,_3,_1,_3];
_2 = [_3,_1,_3,_3,_3,_1];
_5 = [_3,_3,_3,_1,_3,_3,_3,_1];
_3 = 16_u8 as u16;
_10 = [236126167729187299206748218529692307875_u128,57744240176500492845978406914002494804_u128,217195351971625375343558116716605829065_u128,129236186980423803062447017530737196399_u128,294252675172112487279666391323908173110_u128,100004957164200737726496457606853074263_u128,186658234599640892716092426649165436290_u128,82386862456014159043158439136048207749_u128];
_10 = [303953786207668455853558883044047804482_u128,222188813172231871096817294273880309873_u128,92314829277115406482746089482401821124_u128,128350318280940794234492852227236035629_u128,171962724457295973279072481076430840947_u128,30563644578993243611887128426605684335_u128,208247002887529378508218545518713999631_u128,22881954166754349419424333866837341490_u128];
_8 = !2600083208_u32;
_13 = [_1,_1,_1,_1,_1,_6];
RET = (-9223372036854775808_isize) * (-39_isize);
RET = (-36_i8) as isize;
_12 = _4.0;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
Call(_11 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = _13;
_9 = !174380574415157249486169039403202854517_u128;
_11 = [_9];
RET = (-51_isize);
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_13 = _2;
RET = 9223372036854775807_isize;
_14 = _13;
_4.0 = [_1,_1,_1,_1,_1,_1,_1,_1];
_6 = _1 & _1;
_4.0 = [_6,_6,_6,_1,_1,_6,_1,_6];
_15 = [_9,_9];
Goto(bb4)
}
bb4 = {
_9 = 304823017883869623624955536051085495697_u128 << _6;
_3 = _6 | _1;
_2 = _14;
_2 = _14;
_15 = [_9,_9];
_7 = [_3,_6,_3,_1,_1,_6,_6,_1];
_4 = (_12,);
_2 = _14;
_10 = [_9,_9,_9,_9,_9,_9,_9,_9];
_16 = [140_u8,217_u8,5_u8];
_14 = [_6,_6,_3,_3,_6,_3];
_5 = _4.0;
_1 = 144094863107395409147121917630426792631_i128 as u16;
_12 = [_6,_3,_3,_3,_6,_6,_3,_3];
_1 = _9 as u16;
_11 = [_9];
RET = -(-9223372036854775808_isize);
_5 = [_3,_3,_1,_3,_3,_3,_3,_3];
_4 = (_12,);
Goto(bb5)
}
bb5 = {
_8 = 3673947359_u32;
_10 = [_9,_9,_9,_9,_9,_9,_9,_9];
_8 = !285385196_u32;
_2 = _14;
_7 = _12;
_12 = _5;
_4 = (_12,);
_2 = [_1,_6,_3,_6,_3,_1];
_8 = 3943857896_u32;
_17 = [_8,_8,_8,_8,_8,_8,_8,_8];
_7 = [_6,_3,_1,_6,_1,_6,_3,_6];
_8 = !3902268650_u32;
_15 = [_9,_9];
_10 = [_9,_9,_9,_9,_9,_9,_9,_9];
_19 = RET;
_8 = 1950103518_u32;
Goto(bb6)
}
bb6 = {
_17 = [_8,_8,_8,_8,_8,_8,_8,_8];
_15 = [_9,_9];
RET = -_19;
_4 = (_12,);
_19 = _1 as isize;
_13 = [_1,_3,_3,_6,_3,_1];
_22 = '\u{207fe}';
_11 = [_9];
_16 = [149_u8,89_u8,180_u8];
_11 = [_9];
_12 = [_3,_3,_6,_1,_6,_6,_3,_1];
_4 = (_12,);
_2 = _13;
_15 = [_9,_9];
_4.0 = [_1,_3,_3,_1,_3,_3,_3,_3];
_13 = _14;
_16 = [180_u8,98_u8,38_u8];
_22 = '\u{bb2d0}';
_10 = [_9,_9,_9,_9,_9,_9,_9,_9];
_23 = [_1];
_14 = [_1,_1,_1,_6,_1,_3];
_3 = 6190461288097007720_i64 as u16;
_23 = [_1];
_5 = [_1,_1,_6,_6,_6,_6,_6,_1];
match _8 {
0 => bb3,
1950103518 => bb7,
_ => bb2
}
}
bb7 = {
_13 = _14;
_2 = [_3,_6,_1,_6,_6,_1];
_16 = [115_u8,201_u8,215_u8];
_26 = _19;
_22 = '\u{71882}';
_23 = [_6];
_1 = _6;
_4.0 = [_6,_1,_6,_6,_1,_1,_6,_1];
_28 = Adt52::Variant3 { fld0: _6,fld1: 1469507344_i32 };
_17 = [_8,_8,_8,_8,_8,_8,_8,_8];
_13 = _14;
_6 = !_1;
_7 = _5;
_25 = _15;
_2 = [Field::<u16>(Variant(_28, 3), 0),_6,Field::<u16>(Variant(_28, 3), 0),_1,_6,_1];
_4 = (_7,);
_30 = [_1];
Goto(bb8)
}
bb8 = {
_13 = [_6,_6,Field::<u16>(Variant(_28, 3), 0),_6,Field::<u16>(Variant(_28, 3), 0),_1];
_31 = Adt51::Variant0 { fld0: _8 };
_13 = [_6,Field::<u16>(Variant(_28, 3), 0),_1,_1,Field::<u16>(Variant(_28, 3), 0),_1];
_11 = [_9];
_19 = _26 * _26;
SetDiscriminant(_31, 1);
place!(Field::<char>(Variant(_31, 1), 1)) = _22;
_25 = [_9,_9];
_25 = _15;
RET = !_19;
_14 = [_6,Field::<u16>(Variant(_28, 3), 0),_6,Field::<u16>(Variant(_28, 3), 0),Field::<u16>(Variant(_28, 3), 0),_6];
place!(Field::<i32>(Variant(_28, 3), 1)) = 873445467_i32 & 1111908573_i32;
SetDiscriminant(_28, 3);
place!(Field::<[u128; 1]>(Variant(_31, 1), 6)) = _11;
_35 = [4409667806087345730_i64];
_23 = [_6];
_1 = _6 >> _6;
_25 = _15;
place!(Field::<char>(Variant(_31, 1), 1)) = _22;
Goto(bb9)
}
bb9 = {
Call(_38 = dump_var(15_usize, 17_usize, Move(_17), 26_usize, Move(_26), 25_usize, Move(_25), 10_usize, Move(_10)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_38 = dump_var(15_usize, 7_usize, Move(_7), 2_usize, Move(_2), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_38 = dump_var(15_usize, 6_usize, Move(_6), 23_usize, Move(_23), 12_usize, Move(_12), 22_usize, Move(_22)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: u64,mut _2: Adt37,mut _3: u64,mut _4: Adt37,mut _5: usize,mut _6: Adt39,mut _7: Adt39,mut _8: u64,mut _9: [u32; 8],mut _10: Adt39,mut _11: Adt37,mut _12: Adt37,mut _13: Adt37,mut _14: u64,mut _15: Adt39,mut _16: u64) -> f64 {
mir! {
type RET = f64;
let _17: Adt43;
let _18: i64;
let _19: [u8; 3];
let _20: *mut f32;
let _21: ();
let _22: ();
{
place!(Field::<u8>(Variant(_2, 1), 3)) = Field::<char>(Variant(_7, 1), 1) as u8;
place!(Field::<char>(Variant(_6, 1), 1)) = Field::<char>(Variant(_10, 1), 1);
_2 = Field::<Adt37>(Variant(_6, 1), 3);
place!(Field::<u64>(Variant(_10, 1), 4)) = Field::<u64>(Variant(_15, 1), 4);
place!(Field::<f64>(Variant(place!(Field::<Adt37>(Variant(_10, 1), 3)), 1), 2)) = Field::<f64>(Variant(Field::<Adt37>(Variant(_6, 1), 3), 1), 2);
place!(Field::<([u16; 8],)>(Variant(_15, 1), 2)).0 = Field::<([u16; 8],)>(Variant(_6, 1), 2).0;
place!(Field::<([u16; 8],)>(Variant(_10, 1), 2)) = (Field::<([u16; 8],)>(Variant(_15, 1), 2).0,);
place!(Field::<f64>(Variant(_11, 1), 2)) = Field::<f64>(Variant(Field::<Adt37>(Variant(_10, 1), 3), 1), 2) * Field::<f64>(Variant(Field::<Adt37>(Variant(_7, 1), 3), 1), 2);
_18 = (-8918098886424280433_i64) * 1977044000479706219_i64;
place!(Field::<u64>(Variant(_7, 1), 4)) = _5 as u64;
RET = Field::<f64>(Variant(_12, 1), 2);
place!(Field::<[u32; 8]>(Variant(_6, 1), 5)) = Field::<[u32; 8]>(Variant(_15, 1), 5);
place!(Field::<char>(Variant(_15, 1), 1)) = Field::<char>(Variant(_7, 1), 1);
place!(Field::<bool>(Variant(_10, 1), 0)) = Field::<u8>(Variant(_2, 1), 3) != Field::<u8>(Variant(_12, 1), 3);
_13 = Field::<Adt37>(Variant(_15, 1), 3);
SetDiscriminant(Field::<Adt37>(Variant(_10, 1), 3), 1);
_16 = Field::<u64>(Variant(_4, 1), 1) ^ _14;
place!(Field::<char>(Variant(_7, 1), 1)) = Field::<char>(Variant(_10, 1), 1);
Goto(bb1)
}
bb1 = {
Call(_21 = dump_var(16_usize, 1_usize, Move(_1), 9_usize, Move(_9), 3_usize, Move(_3), 16_usize, Move(_16)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: Adt46,mut _2: Adt46,mut _3: Adt46,mut _4: Adt42,mut _5: f64,mut _6: Adt46,mut _7: Adt39,mut _8: [u16; 1],mut _9: Adt46) -> isize {
mir! {
type RET = isize;
let _10: Adt41;
let _11: isize;
let _12: i32;
let _13: [u32; 8];
let _14: u128;
let _15: Adt40;
let _16: (i64, [i64; 1], [i64; 1], u16);
let _17: [u16; 1];
let _18: [u32; 8];
let _19: [u8; 3];
let _20: Adt37;
let _21: [u16; 6];
let _22: Adt52;
let _23: i32;
let _24: f64;
let _25: [u8; 3];
let _26: [u16; 1];
let _27: isize;
let _28: u64;
let _29: [i64; 1];
let _30: f64;
let _31: f64;
let _32: u32;
let _33: [u128; 2];
let _34: isize;
let _35: *mut [u8; 3];
let _36: [u128; 8];
let _37: u16;
let _38: *mut usize;
let _39: usize;
let _40: Adt52;
let _41: [u128; 2];
let _42: [u128; 8];
let _43: [u16; 6];
let _44: f64;
let _45: [u16; 8];
let _46: Adt43;
let _47: i16;
let _48: isize;
let _49: bool;
let _50: isize;
let _51: bool;
let _52: Adt51;
let _53: ();
let _54: ();
{
_3.fld2 = _4.fld1;
place!(Field::<u32>(Variant(_9.fld2, 2), 2)) = !Field::<u32>(Variant(_1.fld2, 2), 2);
place!(Field::<u32>(Variant(_1.fld2, 2), 2)) = Field::<u32>(Variant(_6.fld2, 2), 2) + Field::<u32>(Variant(_2.fld2, 2), 2);
_4.fld0 = [23757_u16,50284_u16,61232_u16,51027_u16,31561_u16,1079_u16];
SetDiscriminant(_1.fld2, 0);
place!(Field::<u32>(Variant(_7, 2), 2)) = Field::<u32>(Variant(_9.fld2, 2), 2) + Field::<u32>(Variant(_3.fld2, 2), 2);
_9.fld1 = [16428_u16];
place!(Field::<char>(Variant(_4.fld1, 2), 1)) = Field::<char>(Variant(_6.fld2, 2), 1);
place!(Field::<u32>(Variant(_7, 2), 2)) = Field::<u32>(Variant(_4.fld1, 2), 2) & Field::<u32>(Variant(_3.fld2, 2), 2);
Goto(bb1)
}
bb1 = {
_7 = _4.fld1;
_1.fld1 = _8;
place!(Field::<[u16; 1]>(Variant(_2.fld2, 2), 0)) = [62284_u16];
_13 = [Field::<u32>(Variant(_4.fld1, 2), 2),Field::<u32>(Variant(_4.fld1, 2), 2),Field::<u32>(Variant(_4.fld1, 2), 2),Field::<u32>(Variant(_7, 2), 2),Field::<u32>(Variant(_3.fld2, 2), 2),Field::<u32>(Variant(_6.fld2, 2), 2),Field::<u32>(Variant(_3.fld2, 2), 2),Field::<u32>(Variant(_7, 2), 2)];
place!(Field::<char>(Variant(_9.fld2, 2), 1)) = Field::<char>(Variant(_2.fld2, 2), 1);
_2 = _6;
Goto(bb2)
}
bb2 = {
_3.fld4 = _2.fld4;
_2.fld0 = _1.fld0;
_6.fld2 = _3.fld2;
SetDiscriminant(_6.fld2, 3);
place!(Field::<[u16; 1]>(Variant(_4.fld1, 2), 0)) = [15269_u16];
place!(Field::<[u16; 1]>(Variant(_1.fld2, 0), 3)) = [15905_u16];
_14 = 194952063389537089847592910825888078112_u128 << Field::<u32>(Variant(_2.fld2, 2), 2);
_6 = _3;
place!(Field::<[u16; 1]>(Variant(_7, 2), 0)) = [34105_u16];
_14 = (-12017_i16) as u128;
place!(Field::<char>(Variant(_9.fld2, 2), 1)) = Field::<char>(Variant(_6.fld2, 2), 1);
_4.fld1 = _7;
_12 = -(-1195263082_i32);
RET = !93_isize;
place!(Field::<[u16; 1]>(Variant(_3.fld2, 2), 0)) = [16179_u16];
_4.fld0 = [32503_u16,12926_u16,46097_u16,10710_u16,14361_u16,13551_u16];
Goto(bb3)
}
bb3 = {
place!(Field::<u32>(Variant(_2.fld2, 2), 2)) = Field::<u32>(Variant(_6.fld2, 2), 2) | Field::<u32>(Variant(_7, 2), 2);
_8 = [33787_u16];
_1.fld1 = [10850_u16];
_11 = false as isize;
place!(Field::<([u16; 8],)>(Variant(_1.fld2, 0), 2)).0 = [18135_u16,44148_u16,1323_u16,56663_u16,7733_u16,64602_u16,45499_u16,45495_u16];
_3 = _2;
place!(Field::<[u16; 1]>(Variant(_3.fld2, 2), 0)) = [24411_u16];
_1 = _2;
_4.fld0 = [61646_u16,22009_u16,61227_u16,22940_u16,234_u16,52595_u16];
RET = _11 + _11;
_2.fld2 = _6.fld2;
_3.fld1 = _6.fld1;
RET = -_11;
Goto(bb4)
}
bb4 = {
_2.fld0 = _3.fld0;
place!(Field::<char>(Variant(_3.fld2, 2), 1)) = Field::<char>(Variant(_4.fld1, 2), 1);
_17 = [18835_u16];
_3.fld2 = _6.fld2;
_2.fld1 = [46459_u16];
_3.fld1 = [40982_u16];
place!(Field::<char>(Variant(_7, 2), 1)) = Field::<char>(Variant(_4.fld1, 2), 1);
_6.fld2 = _7;
_3.fld1 = [5371_u16];
_9.fld4 = [_14,_14];
_3.fld2 = _6.fld2;
_17 = [254_u16];
_7 = _2.fld2;
place!(Field::<u32>(Variant(_7, 2), 2)) = !Field::<u32>(Variant(_4.fld1, 2), 2);
_2.fld2 = _3.fld2;
_1.fld2 = _2.fld2;
_8 = Field::<[u16; 1]>(Variant(_7, 2), 0);
_9.fld1 = [22646_u16];
_2.fld2 = _4.fld1;
_9.fld0 = _3.fld0;
_2.fld4 = [_14,_14];
_2.fld2 = _7;
_9.fld2 = _7;
Goto(bb5)
}
bb5 = {
place!(Field::<u32>(Variant(_4.fld1, 2), 2)) = RET as u32;
_1 = _9;
_12 = 1312400603_i32 << Field::<u32>(Variant(_3.fld2, 2), 2);
_13 = [Field::<u32>(Variant(_3.fld2, 2), 2),Field::<u32>(Variant(_2.fld2, 2), 2),Field::<u32>(Variant(_2.fld2, 2), 2),Field::<u32>(Variant(_1.fld2, 2), 2),Field::<u32>(Variant(_1.fld2, 2), 2),Field::<u32>(Variant(_1.fld2, 2), 2),Field::<u32>(Variant(_6.fld2, 2), 2),Field::<u32>(Variant(_3.fld2, 2), 2)];
_16.0 = 578628252518248472_i64 << Field::<u32>(Variant(_3.fld2, 2), 2);
_8 = [4564_u16];
place!(Field::<char>(Variant(_4.fld1, 2), 1)) = Field::<char>(Variant(_6.fld2, 2), 1);
_19 = [213_u8,31_u8,124_u8];
place!(Field::<[u16; 1]>(Variant(_2.fld2, 2), 0)) = [51147_u16];
_12 = -1668246751_i32;
SetDiscriminant(_7, 2);
_16.2 = [_16.0];
_16.1 = _16.2;
_2.fld2 = _6.fld2;
place!(Field::<u32>(Variant(_7, 2), 2)) = !Field::<u32>(Variant(_6.fld2, 2), 2);
_21 = _4.fld0;
_20 = Adt37::Variant1 { fld0: _2.fld0,fld1: 17135780764056511201_u64,fld2: _5,fld3: 182_u8 };
place!(Field::<u8>(Variant(_20, 1), 3)) = true as u8;
_6.fld2 = _9.fld2;
_9.fld4 = _1.fld4;
place!(Field::<[u16; 1]>(Variant(_1.fld2, 2), 0)) = Field::<[u16; 1]>(Variant(_9.fld2, 2), 0);
place!(Field::<char>(Variant(_7, 2), 1)) = Field::<char>(Variant(_1.fld2, 2), 1);
_11 = RET << _2.fld0;
Goto(bb6)
}
bb6 = {
RET = _11 << Field::<u32>(Variant(_3.fld2, 2), 2);
_16.3 = 37217_u16 << _2.fld0;
_21 = _4.fld0;
Goto(bb7)
}
bb7 = {
_16.0 = 5279122371267154040_i64;
place!(Field::<u32>(Variant(_3.fld2, 2), 2)) = (-13999_i16) as u32;
_2.fld0 = _3.fld0 - _9.fld0;
_12 = -2105143787_i32;
_3.fld0 = _9.fld0;
Goto(bb8)
}
bb8 = {
_15 = Adt40::Variant0 { fld0: _3.fld2 };
_3 = _1;
_6.fld0 = !Field::<usize>(Variant(_20, 1), 0);
_7 = _9.fld2;
_24 = _5;
place!(Field::<char>(Variant(_2.fld2, 2), 1)) = Field::<char>(Variant(_4.fld1, 2), 1);
_22 = Adt52::Variant3 { fld0: _16.3,fld1: _12 };
_4.fld0 = _21;
place!(Field::<[u16; 1]>(Variant(_9.fld2, 2), 0)) = [_16.3];
_6.fld2 = _2.fld2;
_25 = [Field::<u8>(Variant(_20, 1), 3),Field::<u8>(Variant(_20, 1), 3),Field::<u8>(Variant(_20, 1), 3)];
place!(Field::<[u16; 1]>(Variant(_6.fld2, 2), 0)) = [Field::<u16>(Variant(_22, 3), 0)];
SetDiscriminant(_3.fld2, 3);
_14 = 50514770802107511576152619290635093788_i128 as u128;
_4.fld0 = _21;
SetDiscriminant(_7, 3);
_3.fld0 = _9.fld0 * Field::<usize>(Variant(_20, 1), 0);
Goto(bb9)
}
bb9 = {
_1.fld1 = [Field::<u16>(Variant(_22, 3), 0)];
_16.0 = 5220493074955922983_i64;
_14 = 16637591932244861707958158282988677472_u128 + 37353915380583483839152523711048565059_u128;
place!(Field::<u64>(Variant(_20, 1), 1)) = 3022744728828130203_u64;
_25 = [Field::<u8>(Variant(_20, 1), 3),Field::<u8>(Variant(_20, 1), 3),Field::<u8>(Variant(_20, 1), 3)];
place!(Field::<u32>(Variant(_9.fld2, 2), 2)) = !Field::<u32>(Variant(_2.fld2, 2), 2);
_9.fld0 = _6.fld0 >> _16.3;
_2 = _9;
_1 = _6;
place!(Field::<f64>(Variant(_20, 1), 2)) = _24;
_1.fld0 = _9.fld0 ^ _2.fld0;
_19 = [Field::<u8>(Variant(_20, 1), 3),Field::<u8>(Variant(_20, 1), 3),Field::<u8>(Variant(_20, 1), 3)];
_3.fld4 = _6.fld4;
place!(Field::<[u16; 1]>(Variant(_1.fld2, 2), 0)) = [Field::<u16>(Variant(_22, 3), 0)];
_3.fld2 = Adt39::Variant2 { fld0: _1.fld1,fld1: Field::<char>(Variant(_4.fld1, 2), 1),fld2: Field::<u32>(Variant(_1.fld2, 2), 2) };
_26 = _1.fld1;
SetDiscriminant(_3.fld2, 0);
_27 = !RET;
_6.fld0 = _1.fld0 << _1.fld0;
_9.fld4 = _3.fld4;
_16.2 = [_16.0];
place!(Field::<char>(Variant(_6.fld2, 2), 1)) = Field::<char>(Variant(Field::<Adt39>(Variant(_15, 0), 0), 2), 1);
Goto(bb10)
}
bb10 = {
_18 = [Field::<u32>(Variant(_2.fld2, 2), 2),Field::<u32>(Variant(_2.fld2, 2), 2),Field::<u32>(Variant(_2.fld2, 2), 2),Field::<u32>(Variant(_2.fld2, 2), 2),Field::<u32>(Variant(_1.fld2, 2), 2),Field::<u32>(Variant(_6.fld2, 2), 2),Field::<u32>(Variant(_1.fld2, 2), 2),Field::<u32>(Variant(_6.fld2, 2), 2)];
_23 = !Field::<i32>(Variant(_22, 3), 1);
_16.1 = [_16.0];
_9.fld0 = _1.fld0;
_23 = !_12;
_30 = -_24;
place!(Field::<[u16; 1]>(Variant(_3.fld2, 0), 3)) = [_16.3];
place!(Field::<u32>(Variant(_9.fld2, 2), 2)) = Field::<u32>(Variant(_6.fld2, 2), 2);
place!(Field::<char>(Variant(_1.fld2, 2), 1)) = Field::<char>(Variant(_4.fld1, 2), 1);
_25 = _19;
_4.fld1 = _6.fld2;
RET = -_27;
_5 = Field::<u8>(Variant(_20, 1), 3) as f64;
_3.fld1 = [Field::<u16>(Variant(_22, 3), 0)];
place!(Field::<u32>(Variant(_4.fld1, 2), 2)) = Field::<u32>(Variant(_6.fld2, 2), 2) << Field::<u32>(Variant(_9.fld2, 2), 2);
_1.fld2 = Adt39::Variant2 { fld0: Field::<[u16; 1]>(Variant(_3.fld2, 0), 3),fld1: Field::<char>(Variant(_2.fld2, 2), 1),fld2: Field::<u32>(Variant(_4.fld1, 2), 2) };
_21 = [_16.3,Field::<u16>(Variant(_22, 3), 0),_16.3,_16.3,_16.3,_16.3];
place!(Field::<[u16; 1]>(Variant(_3.fld2, 0), 3)) = [_16.3];
place!(Field::<[u128; 2]>(Variant(_3.fld2, 0), 0)) = [_14,_14];
SetDiscriminant(_9.fld2, 3);
Call(_31 = core::intrinsics::transmute(_6.fld0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = _1;
_3.fld1 = _26;
place!(Field::<([u16; 8],)>(Variant(_3.fld2, 0), 2)).0 = [Field::<u16>(Variant(_22, 3), 0),_16.3,Field::<u16>(Variant(_22, 3), 0),_16.3,_16.3,Field::<u16>(Variant(_22, 3), 0),_16.3,_16.3];
_9.fld0 = _6.fld0 & _3.fld0;
place!(Field::<Adt37>(Variant(_7, 3), 1)) = Adt37::Variant1 { fld0: _1.fld0,fld1: Field::<u64>(Variant(_20, 1), 1),fld2: _31,fld3: Field::<u8>(Variant(_20, 1), 3) };
SetDiscriminant(_20, 0);
_32 = Field::<u32>(Variant(_4.fld1, 2), 2);
_32 = !Field::<u32>(Variant(_2.fld2, 2), 2);
place!(Field::<u32>(Variant(_1.fld2, 2), 2)) = Field::<u32>(Variant(_2.fld2, 2), 2) << _1.fld0;
_17 = [_16.3];
_9.fld4 = [_14,_14];
SetDiscriminant(_1.fld2, 2);
place!(Field::<i32>(Variant(_20, 0), 5)) = _23;
_9.fld2 = Field::<Adt39>(Variant(_15, 0), 0);
SetDiscriminant(_15, 2);
_28 = Field::<u64>(Variant(Field::<Adt37>(Variant(_7, 3), 1), 1), 1) + Field::<u64>(Variant(Field::<Adt37>(Variant(_7, 3), 1), 1), 1);
_3.fld2 = _9.fld2;
place!(Field::<u32>(Variant(_9.fld2, 2), 2)) = Field::<char>(Variant(_9.fld2, 2), 1) as u32;
place!(Field::<char>(Variant(_1.fld2, 2), 1)) = Field::<char>(Variant(_2.fld2, 2), 1);
_16.3 = Field::<u16>(Variant(_22, 3), 0) - Field::<u16>(Variant(_22, 3), 0);
_5 = -_31;
Goto(bb12)
}
bb12 = {
_7 = _2.fld2;
SetDiscriminant(_7, 0);
place!(Field::<[u16; 8]>(Variant(_15, 2), 0)) = [_16.3,Field::<u16>(Variant(_22, 3), 0),_16.3,_16.3,_16.3,_16.3,Field::<u16>(Variant(_22, 3), 0),_16.3];
place!(Field::<u32>(Variant(_1.fld2, 2), 2)) = !Field::<u32>(Variant(_6.fld2, 2), 2);
place!(Field::<i32>(Variant(_22, 3), 1)) = (-32175_i16) as i32;
_34 = !_27;
_18 = [Field::<u32>(Variant(_4.fld1, 2), 2),Field::<u32>(Variant(_1.fld2, 2), 2),Field::<u32>(Variant(_6.fld2, 2), 2),Field::<u32>(Variant(_6.fld2, 2), 2),Field::<u32>(Variant(_2.fld2, 2), 2),Field::<u32>(Variant(_1.fld2, 2), 2),_32,Field::<u32>(Variant(_4.fld1, 2), 2)];
_20 = Adt37::Variant1 { fld0: _3.fld0,fld1: _28,fld2: _31,fld3: 155_u8 };
_9.fld1 = [Field::<u16>(Variant(_22, 3), 0)];
_34 = _27;
place!(Field::<f64>(Variant(_20, 1), 2)) = 158_u8 as f64;
place!(Field::<char>(Variant(_3.fld2, 2), 1)) = Field::<char>(Variant(_6.fld2, 2), 1);
_39 = !Field::<usize>(Variant(_20, 1), 0);
_4.fld1 = _2.fld2;
_6.fld4 = [_14,_14];
place!(Field::<u32>(Variant(_1.fld2, 2), 2)) = Field::<u32>(Variant(_6.fld2, 2), 2) ^ Field::<u32>(Variant(_6.fld2, 2), 2);
place!(Field::<u32>(Variant(_1.fld2, 2), 2)) = !Field::<u32>(Variant(_6.fld2, 2), 2);
_3.fld2 = _2.fld2;
_26 = [_16.3];
_1 = _9;
_9.fld0 = 19655_i16 as usize;
_6.fld0 = _1.fld0;
_3.fld1 = [_16.3];
_9.fld0 = _6.fld0 * _2.fld0;
_18 = _13;
_37 = Field::<u16>(Variant(_22, 3), 0) ^ _16.3;
_4.fld1 = Adt39::Variant2 { fld0: _6.fld1,fld1: Field::<char>(Variant(_1.fld2, 2), 1),fld2: Field::<u32>(Variant(_6.fld2, 2), 2) };
match _16.0 {
5220493074955922983 => bb13,
_ => bb9
}
}
bb13 = {
_6.fld2 = _3.fld2;
_13 = [Field::<u32>(Variant(_2.fld2, 2), 2),_32,Field::<u32>(Variant(_6.fld2, 2), 2),_32,Field::<u32>(Variant(_4.fld1, 2), 2),Field::<u32>(Variant(_4.fld1, 2), 2),Field::<u32>(Variant(_4.fld1, 2), 2),Field::<u32>(Variant(_3.fld2, 2), 2)];
_1 = _9;
place!(Field::<char>(Variant(_4.fld1, 2), 1)) = Field::<char>(Variant(_3.fld2, 2), 1);
SetDiscriminant(_4.fld1, 2);
_10.fld0 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_3.fld2, 1), 0)));
SetDiscriminant(_1.fld2, 2);
_37 = _16.3;
_34 = (-139173209036262097509431036545950576455_i128) as isize;
place!(Field::<Adt37>(Variant(_3.fld2, 1), 3)) = Adt37::Variant1 { fld0: _1.fld0,fld1: _28,fld2: _31,fld3: 53_u8 };
match _16.0 {
5220493074955922983 => bb15,
_ => bb14
}
}
bb14 = {
_15 = Adt40::Variant0 { fld0: _3.fld2 };
_3 = _1;
_6.fld0 = !Field::<usize>(Variant(_20, 1), 0);
_7 = _9.fld2;
_24 = _5;
place!(Field::<char>(Variant(_2.fld2, 2), 1)) = Field::<char>(Variant(_4.fld1, 2), 1);
_22 = Adt52::Variant3 { fld0: _16.3,fld1: _12 };
_4.fld0 = _21;
place!(Field::<[u16; 1]>(Variant(_9.fld2, 2), 0)) = [_16.3];
_6.fld2 = _2.fld2;
_25 = [Field::<u8>(Variant(_20, 1), 3),Field::<u8>(Variant(_20, 1), 3),Field::<u8>(Variant(_20, 1), 3)];
place!(Field::<[u16; 1]>(Variant(_6.fld2, 2), 0)) = [Field::<u16>(Variant(_22, 3), 0)];
SetDiscriminant(_3.fld2, 3);
_14 = 50514770802107511576152619290635093788_i128 as u128;
_4.fld0 = _21;
SetDiscriminant(_7, 3);
_3.fld0 = _9.fld0 * Field::<usize>(Variant(_20, 1), 0);
Goto(bb9)
}
bb15 = {
place!(Field::<([u16; 8],)>(Variant(_7, 0), 2)) = (Field::<[u16; 8]>(Variant(_15, 2), 0),);
_37 = Field::<u16>(Variant(_22, 3), 0) * _16.3;
_42 = [_14,_14,_14,_14,_14,_14,_14,_14];
place!(Field::<[u128; 1]>(Variant(_15, 2), 1)) = [_14];
_29 = [_16.0];
place!(Field::<char>(Variant(_4.fld1, 2), 1)) = Field::<char>(Variant(_9.fld2, 2), 1);
_9 = _2;
place!(Field::<u32>(Variant(_6.fld2, 2), 2)) = _14 as u32;
place!(Field::<f64>(Variant(_20, 1), 2)) = -_31;
_4.fld0 = [_37,Field::<u16>(Variant(_22, 3), 0),_37,_37,_37,_16.3];
_32 = !Field::<u32>(Variant(_2.fld2, 2), 2);
_20 = Adt37::Variant1 { fld0: _9.fld0,fld1: _28,fld2: _31,fld3: 228_u8 };
_6.fld1 = [_16.3];
place!(Field::<u32>(Variant(_6.fld2, 2), 2)) = Field::<u64>(Variant(_20, 1), 1) as u32;
_3.fld2 = Adt39::Variant2 { fld0: _3.fld1,fld1: Field::<char>(Variant(_6.fld2, 2), 1),fld2: Field::<u32>(Variant(_9.fld2, 2), 2) };
_43 = [_16.3,_16.3,_37,_37,_16.3,_37];
place!(Field::<[u16; 1]>(Variant(_7, 0), 3)) = _3.fld1;
_49 = !true;
Goto(bb16)
}
bb16 = {
Call(_53 = dump_var(17_usize, 17_usize, Move(_17), 11_usize, Move(_11), 25_usize, Move(_25), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(17_usize, 23_usize, Move(_23), 21_usize, Move(_21), 13_usize, Move(_13), 49_usize, Move(_49)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(17_usize, 37_usize, Move(_37), 19_usize, Move(_19), 12_usize, Move(_12), 54_usize, _54), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{333b}'), std::hint::black_box(49_isize), std::hint::black_box((-12_i8)), std::hint::black_box((-19542_i16)), std::hint::black_box((-1857063314_i32)), std::hint::black_box((-2754304518954427776_i64)), std::hint::black_box(66747116518378922803192968367499192769_i128), std::hint::black_box(13959198652434362204_usize), std::hint::black_box(2_u8), std::hint::black_box(11237_u16), std::hint::black_box(907259920_u32), std::hint::black_box(5275313612477253442_u64), std::hint::black_box(136797095152000877788807884157554552909_u128));
                
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt37 {
Variant0{
fld0: [i64; 1],
fld1: *mut f32,
fld2: isize,
fld3: f64,
fld4: [u16; 1],
fld5: i32,

},
Variant1{
fld0: usize,
fld1: u64,
fld2: f64,
fld3: u8,

}}
#[derive(Debug)]
pub enum Adt38 {
Variant0{
fld0: u64,
fld1: [u128; 8],
fld2: isize,
fld3: i8,
fld4: *const bool,
fld5: i32,
fld6: [u16; 8],
fld7: i128,

},
Variant1{
fld0: u32,
fld1: char,
fld2: u128,
fld3: Adt37,
fld4: ([u16; 8],),
fld5: [u128; 8],
fld6: [u128; 1],
fld7: i128,

},
Variant2{
fld0: bool,
fld1: Adt37,
fld2: i128,
fld3: [u8; 3],
fld4: u16,
fld5: i32,

},
Variant3{
fld0: i64,
fld1: u16,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt39 {
Variant0{
fld0: [u128; 2],
fld1: (*const *mut f64,),
fld2: ([u16; 8],),
fld3: [u16; 1],

},
Variant1{
fld0: bool,
fld1: char,
fld2: ([u16; 8],),
fld3: Adt37,
fld4: u64,
fld5: [u32; 8],
fld6: *mut usize,
fld7: [u8; 3],

},
Variant2{
fld0: [u16; 1],
fld1: char,
fld2: u32,

},
Variant3{
fld0: *mut f32,
fld1: Adt37,
fld2: u128,

}}
#[derive(Debug)]
pub enum Adt40 {
Variant0{
fld0: Adt39,

},
Variant1{
fld0: i128,
fld1: *mut [u8; 3],
fld2: isize,
fld3: (*const *mut f64,),
fld4: i16,

},
Variant2{
fld0: [u16; 8],
fld1: [u128; 1],
fld2: *const *mut usize,
fld3: *mut [u8; 3],
fld4: u32,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt41 {
fld0: *const bool,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt42 {
fld0: [u16; 6],
fld1: Adt39,
}
#[derive(Debug)]
pub enum Adt43 {
Variant0{
fld0: [i64; 1],
fld1: char,

},
Variant1{
fld0: bool,
fld1: (i64, [i64; 1], [i64; 1], u16),
fld2: Adt37,
fld3: u64,
fld4: [u128; 8],

}}
#[derive(Debug)]
pub enum Adt44 {
Variant0{
fld0: u64,
fld1: *mut [u8; 3],

},
Variant1{
fld0: f32,
fld1: u16,
fld2: u128,

}}
#[derive(Debug)]
pub enum Adt45 {
Variant0{
fld0: bool,
fld1: usize,

},
Variant1{
fld0: *mut f64,
fld1: [u128; 1],
fld2: i16,

},
Variant2{
fld0: Adt38,
fld1: f64,
fld2: [u16; 8],
fld3: [u16; 6],

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt46 {
fld0: usize,
fld1: [u16; 1],
fld2: Adt39,
fld3: (*const *mut f64,),
fld4: [u128; 2],
}
#[derive(Debug,Copy,Clone)]
pub enum Adt47 {
Variant0{
fld0: *mut [u8; 3],
fld1: [u8; 3],
fld2: i64,

},
Variant1{
fld0: (i64, [i64; 1], [i64; 1], u16),
fld1: u8,
fld2: isize,
fld3: *const *mut usize,
fld4: i16,
fld5: u128,
fld6: f32,

},
Variant2{
fld0: [u8; 3],
fld1: *mut [u8; 3],
fld2: Adt41,
fld3: i8,
fld4: i16,
fld5: u128,
fld6: *const *mut f64,

}}
#[derive(Debug)]
pub enum Adt48 {
Variant0{
fld0: Adt40,
fld1: (*const *mut f64,),
fld2: *mut f64,
fld3: f32,
fld4: *const *mut usize,
fld5: Adt37,
fld6: *const bool,

},
Variant1{
fld0: bool,
fld1: Adt44,
fld2: *const *mut f64,
fld3: Adt38,
fld4: [u32; 8],
fld5: *mut usize,
fld6: i64,

},
Variant2{
fld0: f64,
fld1: Adt40,
fld2: [u8; 3],
fld3: u32,
fld4: i16,

}}
#[derive(Debug)]
pub struct Adt49 {
fld0: f64,
fld1: Adt47,
fld2: Adt40,
fld3: *const bool,
fld4: *mut usize,
fld5: usize,
fld6: u32,
}
#[derive(Debug)]
pub struct Adt50 {
fld0: *mut f32,
fld1: Adt42,
fld2: Adt43,
fld3: u64,
}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: u32,

},
Variant1{
fld0: u8,
fld1: char,
fld2: Adt44,
fld3: Adt37,
fld4: u32,
fld5: i32,
fld6: [u128; 1],
fld7: [u128; 2],

},
Variant2{
fld0: Adt46,
fld1: u64,

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: *mut f64,
fld1: [u128; 1],
fld2: Adt48,
fld3: Adt38,
fld4: Adt42,

},
Variant1{
fld0: u64,
fld1: *const *mut usize,
fld2: Adt38,
fld3: i8,
fld4: i16,
fld5: Adt46,
fld6: [u32; 8],

},
Variant2{
fld0: Adt49,
fld1: *const bool,
fld2: isize,
fld3: i64,
fld4: Adt50,
fld5: u32,

},
Variant3{
fld0: u16,
fld1: i32,

}}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: Adt37,
fld1: Adt39,
fld2: Adt49,
fld3: f32,
fld4: u64,
fld5: i32,
fld6: (*const *mut f64,),

},
Variant1{
fld0: ([u16; 8],),
fld1: (i64, [i64; 1], [i64; 1], u16),
fld2: isize,
fld3: f64,
fld4: *mut [u8; 3],
fld5: i64,

},
Variant2{
fld0: bool,
fld1: Adt38,
fld2: Adt40,
fld3: i8,
fld4: (*const *mut f64,),
fld5: [i64; 1],
fld6: [u16; 6],
fld7: u8,

},
Variant3{
fld0: Adt50,
fld1: Adt37,

}}

