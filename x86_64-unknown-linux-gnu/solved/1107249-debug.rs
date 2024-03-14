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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u128) -> Adt51 {
mir! {
type RET = Adt51;
let _14: [u8; 4];
let _15: f64;
let _16: i16;
let _17: f64;
let _18: i16;
let _19: [u8; 4];
let _20: u8;
let _21: [char; 2];
let _22: bool;
let _23: isize;
let _24: *mut (*const char,);
let _25: f64;
let _26: [i32; 6];
let _27: char;
let _28: isize;
let _29: f64;
let _30: isize;
let _31: ([u8; 4], i32, char);
let _32: i8;
let _33: isize;
let _34: u8;
let _35: [char; 2];
let _36: isize;
let _37: Adt46;
let _38: u128;
let _39: f64;
let _40: (i32, usize, *mut [char; 2]);
let _41: *const (*const i128, i8);
let _42: Adt48;
let _43: (*const i128, i8);
let _44: Adt43;
let _45: [i32; 6];
let _46: i8;
let _47: Adt46;
let _48: i8;
let _49: isize;
let _50: [i64; 2];
let _51: char;
let _52: f64;
let _53: u16;
let _54: [char; 2];
let _55: Adt46;
let _56: char;
let _57: (i32, usize, *mut [char; 2]);
let _58: Adt46;
let _59: i32;
let _60: f32;
let _61: f64;
let _62: char;
let _63: [i64; 2];
let _64: [char; 2];
let _65: (*const f64, (*const i128, i8), f32, usize);
let _66: Adt54;
let _67: isize;
let _68: (*const char,);
let _69: bool;
let _70: isize;
let _71: [i64; 2];
let _72: (*const char,);
let _73: i8;
let _74: [char; 2];
let _75: *mut f32;
let _76: Adt55;
let _77: bool;
let _78: [u8; 4];
let _79: *mut f32;
let _80: isize;
let _81: f64;
let _82: [char; 2];
let _83: f64;
let _84: isize;
let _85: (i64, isize, i128);
let _86: [u8; 4];
let _87: u128;
let _88: i16;
let _89: (i64, isize, i128);
let _90: [i32; 6];
let _91: isize;
let _92: f32;
let _93: u128;
let _94: [i64; 2];
let _95: i128;
let _96: [i32; 6];
let _97: u8;
let _98: f64;
let _99: Adt49;
let _100: Adt48;
let _101: usize;
let _102: i32;
let _103: [i32; 6];
let _104: [i64; 2];
let _105: Adt44;
let _106: [char; 2];
let _107: bool;
let _108: f32;
let _109: isize;
let _110: f64;
let _111: ();
let _112: ();
{
_7 = !5033741175153327488_i64;
_9 = 2_usize ^ 6_usize;
_6 = 253_u8 as i32;
_14 = [45_u8,84_u8,87_u8,198_u8];
_1 = true;
_2 = '\u{73ad5}';
_10 = 134_u8;
_6 = _10 as i32;
_13 = _7 as u128;
_8 = (-2280611077813571106699329302617388549_i128) & 99563983674117278514053328496801761725_i128;
_6 = (-1870272422_i32);
_12 = 1167313160_u32 >> _8;
_12 = 2773081001_u32 << _9;
_6 = _8 as i32;
_16 = -27439_i16;
_15 = 15154178907205377239_u64 as f64;
_6 = !1792031055_i32;
_5 = -_16;
_1 = true;
_11 = 59198_u16 * 32794_u16;
_4 = !(-52_i8);
_1 = _13 > _13;
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
134 => bb9,
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
_4 = 100_i8;
_3 = !9223372036854775807_isize;
_2 = '\u{9f9bc}';
_7 = -(-561767349450291291_i64);
_8 = -158447261234942276726354468280296756311_i128;
_14 = [_10,_10,_10,_10];
Goto(bb10)
}
bb10 = {
_1 = true;
Goto(bb11)
}
bb11 = {
_10 = !75_u8;
_3 = !111_isize;
_11 = 53293_u16 & 36818_u16;
_6 = (-1747688325_i32) ^ 138268493_i32;
_8 = _11 as i128;
_5 = _16 + _16;
_3 = (-81_isize);
_11 = !36578_u16;
_18 = -_5;
_12 = 1763027693_u32;
_16 = _3 as i16;
_12 = 964319002_u32;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb6,
6 => bb12,
964319002 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
_4 = 100_i8;
_3 = !9223372036854775807_isize;
_2 = '\u{9f9bc}';
_7 = -(-561767349450291291_i64);
_8 = -158447261234942276726354468280296756311_i128;
_14 = [_10,_10,_10,_10];
Goto(bb10)
}
bb14 = {
_17 = _15 + _15;
_14 = [_10,_10,_10,_10];
_14 = [_10,_10,_10,_10];
_3 = (-40_isize);
_7 = (-7735861126079777492_i64) & (-1019092273553381214_i64);
_10 = 213_u8;
_20 = _10;
_12 = 942057802_u32 | 3160473371_u32;
_4 = 0_i8;
_19 = _14;
_3 = (-9223372036854775808_isize);
_20 = !_10;
_18 = _5 + _5;
_19 = [_10,_10,_10,_10];
_1 = !false;
_16 = !_18;
_8 = 95513437520291756918682325169321187662_i128 & 38315161094736202212710108363256862143_i128;
_13 = _18 as u128;
_8 = (-139648743932917953386652677482623079937_i128);
_12 = 586931886_u32;
_6 = 1181736126_i32 * 1328284314_i32;
_8 = -(-73158174822256535337652442238642989233_i128);
_23 = !_3;
_8 = (-158577432585441008509431424723808094219_i128) & 103915453228450965584156776254437045296_i128;
_17 = _15;
Goto(bb15)
}
bb15 = {
_22 = _15 != _17;
_21 = [_2,_2];
_22 = _3 >= _3;
_21 = [_2,_2];
_7 = (-3173399165497407043_i64) << _9;
_13 = _23 as u128;
_25 = -_17;
_9 = !15028818161655926011_usize;
_19 = _14;
_1 = !_22;
_19 = [_20,_20,_20,_20];
_2 = '\u{1bfb2}';
_3 = _23 >> _16;
_12 = 256839762_u32;
_26 = [_6,_6,_6,_6,_6,_6];
_13 = 175340144011891535698701905633511138557_u128 & 194717263216470623711665078202658954101_u128;
_7 = (-5399878892377174530_i64) - (-3556025236395007324_i64);
_27 = _2;
_17 = _13 as f64;
_4 = _2 as i8;
_17 = _15;
Goto(bb16)
}
bb16 = {
_1 = _22;
_10 = _20 + _20;
_12 = _9 as u32;
_14 = _19;
_3 = _23 | _23;
_25 = _15;
_13 = !239595465380449550569218897183857604638_u128;
_3 = !_23;
_27 = _2;
_26 = [_6,_6,_6,_6,_6,_6];
_28 = _23;
_5 = !_18;
_4 = (-51_i8);
_17 = _8 as f64;
_29 = _10 as f64;
_21 = [_2,_2];
_12 = 4068396132_u32 - 2412017114_u32;
_28 = _23;
_7 = (-6461607149965060213_i64);
_4 = _11 as i8;
_18 = !_5;
_12 = !3017377067_u32;
_26 = [_6,_6,_6,_6,_6,_6];
_6 = !2093631245_i32;
_21 = [_27,_2];
_23 = _3 | _3;
_18 = _16 - _5;
Call(_7 = fn1(_5, _29, _22, _9, _23, _5, _28, _22, _5, _27, _12, _3, _28, _9, _17, _15), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_29 = _25;
_14 = [_10,_10,_10,_20];
_19 = _14;
_12 = !93243827_u32;
_9 = _28 as usize;
_23 = _3;
_2 = _27;
_22 = !_1;
_32 = -_4;
_29 = _17;
_15 = _29;
_31.2 = _27;
_35 = [_2,_2];
_21 = _35;
_12 = 1775371688_u32;
_20 = !_10;
_31 = (_14, _6, _27);
_11 = _7 as u16;
_6 = _2 as i32;
_10 = _20 >> _5;
_9 = _12 as usize;
_31.0 = _19;
_36 = _3;
_26 = [_6,_6,_31.1,_31.1,_6,_31.1];
match _12 {
0 => bb18,
1 => bb19,
2 => bb20,
1775371688 => bb22,
_ => bb21
}
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
_4 = 100_i8;
_3 = !9223372036854775807_isize;
_2 = '\u{9f9bc}';
_7 = -(-561767349450291291_i64);
_8 = -158447261234942276726354468280296756311_i128;
_14 = [_10,_10,_10,_10];
Goto(bb10)
}
bb22 = {
_15 = _8 as f64;
_34 = !_20;
_35 = [_2,_31.2];
_26 = [_6,_31.1,_6,_31.1,_31.1,_31.1];
_7 = -937401696311634135_i64;
_40.2 = core::ptr::addr_of_mut!(_35);
_31 = (_19, _6, _27);
_36 = !_28;
_6 = _3 as i32;
_19 = _14;
_24 = core::ptr::addr_of_mut!(_42.fld1);
_14 = _31.0;
match _12 {
1775371688 => bb24,
_ => bb23
}
}
bb23 = {
Return()
}
bb24 = {
_42.fld5 = [_31.2,_2];
_30 = _23;
_11 = !40209_u16;
_42.fld2 = (_7, _28, _8);
_16 = -_18;
_6 = _15 as i32;
_30 = _23;
_40.1 = _42.fld2.2 as usize;
_42.fld2.2 = _13 as i128;
_40.0 = _6;
_40.0 = _40.1 as i32;
_4 = !_32;
_20 = !_10;
_42.fld2 = (_7, _36, _8);
_1 = _18 > _18;
_14 = _31.0;
_18 = _5 | _16;
_31.2 = _27;
_11 = 3948_u16 | 5467_u16;
_7 = _42.fld2.0;
Goto(bb25)
}
bb25 = {
_29 = _15;
_31.1 = _40.0 - _6;
_42.fld1.0 = core::ptr::addr_of!(_2);
_42.fld5 = [_27,_31.2];
_16 = -_5;
_42.fld3 = Adt42::Variant1 { fld0: _40.2 };
_10 = _20;
_17 = _29 * _15;
_31.2 = _2;
_8 = !_42.fld2.2;
Goto(bb26)
}
bb26 = {
_5 = !_18;
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).0 = _42.fld2.0;
place!(Field::<*mut u32>(Variant(_42.fld3, 2), 0)) = core::ptr::addr_of_mut!(_12);
_11 = 33922_u16;
_43.0 = core::ptr::addr_of!(_42.fld2.2);
place!(Field::<char>(Variant(_42.fld3, 2), 1)) = _2;
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)) = _42.fld2;
_19 = [_10,_34,_20,_34];
_8 = -Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).2;
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).0 = _42.fld2.0 | _42.fld2.0;
_18 = _16;
_1 = _22;
_31 = (_14, _40.0, _27);
Goto(bb27)
}
bb27 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).3 = _40.1;
_42.fld6 = _43.0;
_15 = _17;
_42.fld0 = _22;
_4 = _32;
_31.2 = _27;
_38 = _13;
_42.fld1.0 = core::ptr::addr_of!(place!(Field::<char>(Variant(_42.fld3, 2), 1)));
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).2 = _5 as f32;
_18 = _5 << Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).2;
_25 = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).2 as f64;
_42.fld2 = (_7, _30, _8);
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1.1 = _32;
_41 = core::ptr::addr_of!(_43);
_33 = -_36;
_48 = _32 << _5;
_55.fld0 = [Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0,_42.fld2.0];
_49 = Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).1 >> _7;
_43.1 = _8 as i8;
_35 = [Field::<char>(Variant(_42.fld3, 2), 1),Field::<char>(Variant(_42.fld3, 2), 1)];
_40.2 = core::ptr::addr_of_mut!(_54);
_52 = _25;
_48 = _12 as i8;
_33 = _49 | _42.fld2.1;
_31.0 = [_10,_10,_20,_10];
place!(Field::<[u8; 4]>(Variant(_42.fld3, 2), 4)) = [_20,_20,_10,_20];
match _12 {
0 => bb26,
1 => bb11,
2 => bb7,
3 => bb25,
4 => bb24,
1775371688 => bb28,
_ => bb23
}
}
bb28 = {
_11 = 681_u16 - 5391_u16;
_42.fld4 = [_34,_20,_34,_20];
_3 = _36 << _42.fld2.2;
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).0 = _42.fld2.0 - _42.fld2.0;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1.1 = !_43.1;
_12 = 2003557921_u32 >> Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).2;
Goto(bb29)
}
bb29 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).0 = core::ptr::addr_of!(_17);
_50 = [Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0,_42.fld2.0];
_11 = Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0 as u16;
_51 = Field::<char>(Variant(_42.fld3, 2), 1);
_47 = Adt46 { fld0: _55.fld0 };
_52 = _11 as f64;
_33 = _3;
_52 = -_15;
_28 = !_42.fld2.1;
place!(Field::<*const (*const i128, i8)>(Variant(_42.fld3, 2), 3)) = core::ptr::addr_of!(_43);
_48 = !Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).1.1;
_16 = -_5;
_54 = [Field::<char>(Variant(_42.fld3, 2), 1),_31.2];
_31 = (_42.fld4, _6, Field::<char>(Variant(_42.fld3, 2), 1));
_41 = Field::<*const (*const i128, i8)>(Variant(_42.fld3, 2), 3);
_37.fld0 = [_7,_7];
_41 = core::ptr::addr_of!(place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1);
_57.2 = core::ptr::addr_of_mut!(_21);
Goto(bb30)
}
bb30 = {
_16 = _5;
_17 = _52 * _52;
_55 = _37;
_32 = (*_41).1 << _16;
_53 = _12 as u16;
_54 = [_31.2,Field::<char>(Variant(_42.fld3, 2), 1)];
_5 = !_18;
_12 = !716691000_u32;
_54 = [_27,_51];
_58 = _55;
_31.1 = _40.0;
_42.fld6 = core::ptr::addr_of!(_42.fld2.2);
_58 = _47;
_6 = _31.1;
_26 = [_40.0,_40.0,_31.1,_31.1,_31.1,_31.1];
_55.fld0 = [_42.fld2.0,_42.fld2.0];
place!(Field::<*const (*const i128, i8)>(Variant(_42.fld3, 2), 3)) = _41;
_61 = -_17;
_33 = !_3;
_57.1 = _40.1 * _9;
_31.1 = !_40.0;
_60 = _57.1 as f32;
Goto(bb31)
}
bb31 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1.0 = core::ptr::addr_of!(_8);
_15 = _17;
_57.0 = !_40.0;
_62 = _27;
_43.1 = -_32;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).3 = _9 >> _32;
_63 = [_42.fld2.0,_42.fld2.0];
_49 = -_23;
_55 = Adt46 { fld0: _50 };
_17 = _25 * _15;
_42.fld2.0 = _7 + Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0;
_42.fld1.0 = core::ptr::addr_of!(_56);
_54 = [_62,_51];
_64 = [_27,_27];
_31 = (_19, _6, Field::<char>(Variant(_42.fld3, 2), 1));
_40.2 = _57.2;
_43.0 = core::ptr::addr_of!(_8);
_21 = [_51,_62];
_72.0 = core::ptr::addr_of!(_62);
place!(Field::<[i32; 6]>(Variant(_42.fld3, 2), 6)) = [_40.0,_31.1,_57.0,_6,_31.1,_31.1];
Goto(bb32)
}
bb32 = {
_59 = _1 as i32;
_51 = _31.2;
_54 = _21;
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).1 = _23;
_72.0 = core::ptr::addr_of!(_56);
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1.0 = _43.0;
_51 = _62;
_43.1 = Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0 as i8;
_6 = _40.0 & _57.0;
_67 = _16 as isize;
_8 = _42.fld2.2;
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).1 = !_67;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1.0 = _43.0;
_57.2 = _40.2;
_45 = _26;
_43.1 = _32 - (*_41).1;
_43.1 = _7 as i8;
Goto(bb33)
}
bb33 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1 = (_42.fld6, _32);
_78 = [_10,_10,_20,_20];
_23 = _53 as isize;
_68.0 = core::ptr::addr_of!(_31.2);
_69 = !_42.fld0;
_58 = Adt46 { fld0: _47.fld0 };
Call(place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1.1 = fn19(_18, _67, Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).0, _4, _15, _6, Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).3, _18, _67, _58, Field::<[u8; 4]>(Variant(_42.fld3, 2), 4), _37, Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).2, _58.fld0), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).2 = -_60;
_11 = !_53;
_65.2 = -Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).2;
Goto(bb35)
}
bb35 = {
_65.0 = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).0;
_57.0 = !_59;
_48 = _32;
_65.1 = (_42.fld6, _32);
_81 = -_61;
_25 = _15 + _52;
_20 = _10 - _10;
Goto(bb36)
}
bb36 = {
_40.0 = _6 - _6;
RET = Adt51::Variant0 { fld0: Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).3,fld1: Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).0,fld2: _28,fld3: _31,fld4: _40 };
_73 = !_48;
_30 = _65.1.1 as isize;
_69 = _1 | _1;
place!(Field::<*const f64>(Variant(RET, 0), 1)) = core::ptr::addr_of!(_52);
SetDiscriminant(RET, 1);
place!(Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4)).2 = core::ptr::addr_of_mut!(_21);
_17 = _81;
place!(Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4)).0 = _65.1.1 as i32;
Goto(bb37)
}
bb37 = {
_40.0 = Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4).0 | Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4).0;
place!(Field::<(f32, usize, *const f64, *const char)>(Variant(RET, 1), 2)).1 = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).3 | Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).3;
place!(Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4)) = _40;
_31 = (_19, Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4).0, _2);
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)) = (_42.fld2.0, _30, _42.fld2.2);
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).1 = _30;
_42.fld5 = [_2,_2];
_56 = _51;
_68.0 = _72.0;
place!(Field::<([u8; 4], i32, char)>(Variant(RET, 1), 1)) = (Field::<[u8; 4]>(Variant(_42.fld3, 2), 4), _59, _27);
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).2 = _42.fld2.2;
_2 = _27;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).0 = core::ptr::addr_of!(_15);
place!(Field::<(f32, usize, *const f64, *const char)>(Variant(RET, 1), 2)).3 = core::ptr::addr_of!(_2);
_53 = _10 as u16;
_35 = [_56,Field::<char>(Variant(_42.fld3, 2), 1)];
_31 = Field::<([u8; 4], i32, char)>(Variant(RET, 1), 1);
_50 = _47.fld0;
place!(Field::<*mut (*const char,)>(Variant(RET, 1), 3)) = core::ptr::addr_of_mut!(_42.fld1);
_65 = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5);
Goto(bb38)
}
bb38 = {
_61 = -_15;
_85.0 = _12 as i64;
_68 = ((*_24).0,);
_29 = -_17;
place!(Field::<([u8; 4], i32, char)>(Variant(RET, 1), 1)).1 = Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4).0;
_63 = [_7,_85.0];
_46 = _18 as i8;
_42.fld2.2 = _8;
_53 = _12 as u16;
_60 = Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).2 as f32;
_42.fld6 = core::ptr::addr_of!(_42.fld2.2);
_42.fld2.0 = Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0 >> _10;
_80 = !_23;
place!(Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4)).0 = _38 as i32;
_48 = _62 as i8;
Goto(bb39)
}
bb39 = {
_3 = _30;
_42.fld2.2 = Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0 as i128;
_69 = _42.fld0;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).3 = !Field::<(f32, usize, *const f64, *const char)>(Variant(RET, 1), 2).1;
Goto(bb40)
}
bb40 = {
_13 = _12 as u128;
place!(Field::<(f32, usize, *const f64, *const char)>(Variant(RET, 1), 2)) = (_65.2, Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).3, _65.0, _72.0);
_79 = core::ptr::addr_of_mut!(place!(Field::<(f32, usize, *const f64, *const char)>(Variant(RET, 1), 2)).0);
_57 = (Field::<([u8; 4], i32, char)>(Variant(RET, 1), 1).1, Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).3, Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4).2);
_19 = [_10,_20,_10,_20];
SetDiscriminant(_42.fld3, 2);
_80 = _33;
_86 = [_20,_20,_20,_20];
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).1 = _3 + _67;
_42.fld0 = !_1;
_85 = (_42.fld2.0, Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).1, _8);
_88 = _56 as i16;
_47.fld0 = [_42.fld2.0,_42.fld2.0];
place!(Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 1), 4)) = (_59, _65.3, _57.2);
_40.1 = !Field::<(f32, usize, *const f64, *const char)>(Variant(RET, 1), 2).1;
RET = Adt51::Variant0 { fld0: _40.1,fld1: _65.0,fld2: Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).1,fld3: _31,fld4: _40 };
_78 = [_10,_20,_20,_34];
SetDiscriminant(RET, 2);
_42.fld1 = _68;
_23 = _42.fld0 as isize;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).0 = _65.0;
Goto(bb41)
}
bb41 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0)).1.1 = _46;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0)).0 = _65.0;
_81 = _25 + _25;
_52 = _40.0 as f64;
_65.2 = _11 as f32;
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)) = (_42.fld2.0, _85.1, _42.fld2.2);
_91 = _85.1 * _30;
place!(Field::<*const (*const i128, i8)>(Variant(RET, 2), 7)) = core::ptr::addr_of!(_43);
place!(Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 2), 4)).1 = _40.1;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0)) = _65;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0)).0 = core::ptr::addr_of!(_15);
_49 = _91 << _57.0;
_19 = _78;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0)).3 = Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 2), 4).1 << _49;
_72.0 = core::ptr::addr_of!(_2);
_95 = _42.fld2.2 - _85.2;
_66 = Adt54::Variant1 { fld0: _52 };
place!(Field::<*const i128>(Variant(RET, 2), 3)) = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0).1.0;
place!(Field::<*const (*const i128, i8)>(Variant(_42.fld3, 2), 3)) = Field::<*const (*const i128, i8)>(Variant(RET, 2), 7);
place!(Field::<Adt50>(Variant(_66, 0), 4)).fld2.fld2.1 = _42.fld2.0 as isize;
Goto(bb42)
}
bb42 = {
place!(Field::<f32>(Variant(RET, 2), 5)) = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0).2 - _65.2;
_94 = [_42.fld2.0,_42.fld2.0];
_47 = Adt46 { fld0: _94 };
Goto(bb43)
}
bb43 = {
place!(Field::<*const i128>(Variant(RET, 2), 3)) = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0).1.0;
_74 = _54;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).3 = !_40.1;
_43.1 = _32;
_57.2 = _40.2;
_13 = _38 * _38;
_82 = [_31.2,_2];
Call(_83 = core::intrinsics::transmute(_91), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).2 = _42.fld0 as i128;
_43.1 = _53 as i8;
place!(Field::<*const (*const i128, i8)>(Variant(RET, 2), 7)) = _41;
_42.fld0 = _69 & _1;
_58.fld0 = [_42.fld2.0,_42.fld2.0];
place!(Field::<f32>(Variant(RET, 2), 5)) = _60;
_42.fld4 = _19;
_42.fld1.0 = _72.0;
_34 = _20 ^ _10;
place!(Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 2), 4)).0 = _57.0 | _57.0;
place!(Field::<Adt50>(Variant(_66, 0), 4)).fld2.fld4 = _19;
_54 = [_56,_51];
place!(Field::<*const (*const i128, i8)>(Variant(RET, 2), 7)) = core::ptr::addr_of!(place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).1);
_94 = _63;
_78 = _86;
place!(Field::<(i64, isize, i128)>(Variant(_66, 0), 0)).0 = Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0 << Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 2), 4).1;
_57.2 = core::ptr::addr_of_mut!(_42.fld5);
Goto(bb45)
}
bb45 = {
_77 = _15 <= _83;
_21 = _35;
_89.2 = _8 - _42.fld2.2;
place!(Field::<Adt50>(Variant(_66, 0), 4)).fld2.fld2.1 = _25 as isize;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0)).1 = (_43.0, _73);
_65.1 = (Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0).1.0, Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0).1.1);
_14 = [_20,_10,_20,_34];
_1 = _69;
_83 = _52 - _25;
_31.0 = _19;
Goto(bb46)
}
bb46 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)) = (Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0).0, _65.1, _65.2, _57.1);
_12 = _38 as u32;
_98 = _29;
place!(Field::<Adt50>(Variant(_66, 0), 4)).fld2.fld2.0 = Field::<(i64, isize, i128)>(Variant(_66, 0), 0).0 + Field::<(i64, isize, i128)>(Variant(_66, 0), 0).0;
place!(Field::<[i32; 6]>(Variant(_42.fld3, 2), 6)) = [_57.0,_40.0,_40.0,Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 2), 4).0,_40.0,Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 2), 4).0];
_80 = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0).1.1 as isize;
place!(Field::<f32>(Variant(RET, 2), 5)) = -Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0).2;
_73 = _32 ^ _65.1.1;
_67 = _5 as isize;
place!(Field::<char>(Variant(_66, 0), 1)) = _2;
place!(Field::<*const char>(Variant(RET, 2), 2)) = (*_24).0;
_22 = !_77;
_92 = _34 as f32;
_26 = _45;
_31.0 = _14;
_88 = _5 + _16;
_82 = [_62,_56];
_91 = -_67;
Goto(bb47)
}
bb47 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)) = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0);
_37 = Adt46 { fld0: _58.fld0 };
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).0 = _65.0;
_89 = (Field::<(i64, isize, i128)>(Variant(_66, 0), 0).0, _49, _85.2);
Goto(bb48)
}
bb48 = {
_78 = [_20,_34,_34,_10];
place!(Field::<Adt50>(Variant(_66, 0), 4)).fld0 = _38 - _13;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0)).1.1 = _11 as i8;
_100.fld6 = _65.1.0;
_70 = -_91;
_89.2 = _95;
_97 = _52 as u8;
_100.fld1 = (_68.0,);
_100.fld4 = [_97,_34,_34,_20];
_40.2 = _57.2;
place!(Field::<*const (*const i128, i8)>(Variant(_42.fld3, 2), 3)) = Field::<*const (*const i128, i8)>(Variant(RET, 2), 7);
Goto(bb49)
}
bb49 = {
place!(Field::<(i64, isize, i128)>(Variant(_66, 0), 0)) = (Field::<Adt50>(Variant(_66, 0), 4).fld2.fld2.0, _91, Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).2);
_94 = [Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0,_85.0];
_95 = _12 as i128;
place!(Field::<*const i128>(Variant(RET, 2), 3)) = _100.fld6;
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5)).3 = Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 2), 4).1 | _65.3;
_4 = -_65.1.1;
_42.fld1 = _100.fld1;
Goto(bb50)
}
bb50 = {
place!(Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(RET, 2), 0)).1 = Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).1;
_100.fld2.1 = _3;
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)) = (Field::<Adt50>(Variant(_66, 0), 4).fld2.fld2.0, _36, _42.fld2.2);
place!(Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2)).1 = _89.1 ^ _49;
place!(Field::<*const i128>(Variant(RET, 2), 3)) = _100.fld6;
_73 = -_4;
place!(Field::<(i32, usize, *mut [char; 2])>(Variant(RET, 2), 4)).2 = _40.2;
place!(Field::<u128>(Variant(_66, 0), 3)) = _12 as u128;
place!(Field::<Adt50>(Variant(_66, 0), 4)).fld3 = _40.2;
Goto(bb51)
}
bb51 = {
place!(Field::<Adt50>(Variant(_66, 0), 4)).fld0 = _53 as u128;
_69 = _22;
_40.2 = core::ptr::addr_of_mut!(_64);
place!(Field::<(i64, isize, i128)>(Variant(_66, 0), 0)) = (Field::<Adt50>(Variant(_66, 0), 4).fld2.fld2.0, Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).1, _8);
place!(Field::<(i64, isize, i128)>(Variant(_66, 0), 0)).2 = !Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).2;
_14 = _42.fld4;
_93 = _13 - _38;
_107 = _69;
_100.fld2.0 = Field::<(i64, isize, i128)>(Variant(_42.fld3, 2), 2).0;
_64 = [_51,_62];
place!(Field::<Adt50>(Variant(_66, 0), 4)).fld2.fld1.0 = core::ptr::addr_of!(_31.2);
_12 = !1717933692_u32;
_52 = Field::<(i64, isize, i128)>(Variant(_66, 0), 0).0 as f64;
RET = Adt51::Variant0 { fld0: _40.1,fld1: Field::<(*const f64, (*const i128, i8), f32, usize)>(Variant(_42.fld3, 2), 5).0,fld2: _49,fld3: _31,fld4: _57 };
_74 = [_27,_27];
Goto(bb52)
}
bb52 = {
Call(_111 = dump_var(0_usize, 50_usize, Move(_50), 85_usize, Move(_85), 56_usize, Move(_56), 88_usize, Move(_88)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_111 = dump_var(0_usize, 94_usize, Move(_94), 80_usize, Move(_80), 18_usize, Move(_18), 73_usize, Move(_73)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_111 = dump_var(0_usize, 20_usize, Move(_20), 19_usize, Move(_19), 63_usize, Move(_63), 27_usize, Move(_27)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_111 = dump_var(0_usize, 97_usize, Move(_97), 30_usize, Move(_30), 64_usize, Move(_64), 69_usize, Move(_69)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_111 = dump_var(0_usize, 6_usize, Move(_6), 10_usize, Move(_10), 49_usize, Move(_49), 5_usize, Move(_5)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_111 = dump_var(0_usize, 77_usize, Move(_77), 51_usize, Move(_51), 67_usize, Move(_67), 33_usize, Move(_33)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_111 = dump_var(0_usize, 62_usize, Move(_62), 9_usize, Move(_9), 38_usize, Move(_38), 59_usize, Move(_59)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_111 = dump_var(0_usize, 3_usize, Move(_3), 74_usize, Move(_74), 53_usize, Move(_53), 4_usize, Move(_4)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i16,mut _2: f64,mut _3: bool,mut _4: usize,mut _5: isize,mut _6: i16,mut _7: isize,mut _8: bool,mut _9: i16,mut _10: char,mut _11: u32,mut _12: isize,mut _13: isize,mut _14: usize,mut _15: f64,mut _16: f64) -> i64 {
mir! {
type RET = i64;
let _17: bool;
let _18: [i64; 2];
let _19: (i64, isize, i128);
let _20: *mut [char; 2];
let _21: u128;
let _22: f64;
let _23: Adt46;
let _24: i8;
let _25: [i32; 6];
let _26: bool;
let _27: [i32; 6];
let _28: Adt43;
let _29: f64;
let _30: [u8; 4];
let _31: (*const i128, i8);
let _32: u32;
let _33: bool;
let _34: i16;
let _35: [i64; 2];
let _36: char;
let _37: f64;
let _38: u8;
let _39: i8;
let _40: [char; 2];
let _41: i128;
let _42: [i32; 6];
let _43: char;
let _44: Adt46;
let _45: f64;
let _46: ();
let _47: ();
{
_7 = _13;
RET = (-8199589850774965163_i64);
_2 = _16;
_7 = _13 * _5;
_4 = !_14;
_11 = 2834556329_u32;
_17 = _8 >= _8;
_19.2 = _1 as i128;
_8 = _17 ^ _17;
_19.1 = _13;
_19.2 = _4 as i128;
_7 = _13 >> _6;
_1 = _14 as i16;
RET = _16 as i64;
match _11 {
0 => bb1,
1 => bb2,
2834556329 => bb4,
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
_21 = 42385068860746496537962634393276111682_u128 >> _13;
_19.1 = -_12;
_11 = 1439690324_u32 << _7;
_3 = _8 & _8;
_19.0 = RET;
_11 = 4168311062_u32 ^ 1303214082_u32;
_19.0 = !RET;
RET = _19.0;
_4 = !_14;
_22 = 198893689_i32 as f64;
_22 = -_2;
_12 = -_13;
_2 = -_16;
_18 = [RET,_19.0];
_13 = 1700351951120302669_u64 as isize;
_23.fld0 = [RET,_19.0];
_18 = [_19.0,_19.0];
_19.1 = _11 as isize;
_6 = _10 as i16;
_4 = 1160414213_i32 as usize;
_19.0 = RET;
_23 = Adt46 { fld0: _18 };
Call(RET = fn2(_19.1, _21, _3, _9, _1, _10), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_24 = (-118_i8);
_2 = _22 + _22;
_23.fld0 = _18;
RET = _11 as i64;
_22 = _15 * _2;
_7 = _5;
_25 = [(-829053731_i32),(-992683248_i32),(-1741721759_i32),(-1987871518_i32),(-610873120_i32),(-1838143247_i32)];
_26 = _3;
_23 = Adt46 { fld0: _18 };
Goto(bb6)
}
bb6 = {
_11 = 684331387_u32 & 3701822485_u32;
_19 = (RET, _5, 135377869582378849423328766358782538502_i128);
_11 = !1537436311_u32;
_21 = !306010517475168347887571735145088640968_u128;
_29 = -_22;
_27 = _25;
_19.1 = _13;
_4 = !_14;
RET = _19.0 >> _24;
_24 = (-121_i8);
_2 = _15 + _22;
_29 = -_15;
_19.0 = _19.2 as i64;
_16 = _11 as f64;
_13 = _19.1 << RET;
_19.0 = RET;
_24 = 3_i8;
_31.1 = 11623564124551650922_u64 as i8;
_4 = _14 | _14;
_19.1 = _7 >> _19.0;
_30 = [241_u8,226_u8,115_u8,254_u8];
_27 = [197986386_i32,841829960_i32,(-1371874957_i32),(-658852750_i32),(-1626686407_i32),(-1797688111_i32)];
_15 = -_2;
_18 = [_19.0,RET];
match _19.2 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
135377869582378849423328766358782538502 => bb12,
_ => bb11
}
}
bb7 = {
_24 = (-118_i8);
_2 = _22 + _22;
_23.fld0 = _18;
RET = _11 as i64;
_22 = _15 * _2;
_7 = _5;
_25 = [(-829053731_i32),(-992683248_i32),(-1741721759_i32),(-1987871518_i32),(-610873120_i32),(-1838143247_i32)];
_26 = _3;
_23 = Adt46 { fld0: _18 };
Goto(bb6)
}
bb8 = {
_21 = 42385068860746496537962634393276111682_u128 >> _13;
_19.1 = -_12;
_11 = 1439690324_u32 << _7;
_3 = _8 & _8;
_19.0 = RET;
_11 = 4168311062_u32 ^ 1303214082_u32;
_19.0 = !RET;
RET = _19.0;
_4 = !_14;
_22 = 198893689_i32 as f64;
_22 = -_2;
_12 = -_13;
_2 = -_16;
_18 = [RET,_19.0];
_13 = 1700351951120302669_u64 as isize;
_23.fld0 = [RET,_19.0];
_18 = [_19.0,_19.0];
_19.1 = _11 as isize;
_6 = _10 as i16;
_4 = 1160414213_i32 as usize;
_19.0 = RET;
_23 = Adt46 { fld0: _18 };
Call(RET = fn2(_19.1, _21, _3, _9, _1, _10), ReturnTo(bb5), UnwindUnreachable())
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
_22 = _29;
_4 = (-188162911_i32) as usize;
_19.2 = 131651828278605740323834793409285638305_i128 | 113892102340998611261969996273790001550_i128;
_2 = _15 + _15;
_9 = _6 - _6;
_23 = Adt46 { fld0: _18 };
_18 = [_19.0,RET];
_29 = -_2;
_35 = [RET,_19.0];
_36 = _10;
_16 = _15;
_32 = _11 >> _19.2;
_19.1 = _7 >> _13;
_22 = -_15;
_31.1 = !_24;
_38 = 194_u8 << RET;
_5 = 694051533_i32 as isize;
Goto(bb13)
}
bb13 = {
_19.1 = _13 ^ _12;
_27 = _25;
_33 = _8;
_4 = _22 as usize;
_42 = [228656939_i32,(-602129934_i32),(-230300674_i32),613796733_i32,(-1163915566_i32),(-33248521_i32)];
_40 = [_36,_10];
_18 = [_19.0,RET];
_20 = core::ptr::addr_of_mut!(_40);
_40 = [_10,_36];
_26 = _33;
_5 = _38 as isize;
_9 = !_6;
Goto(bb14)
}
bb14 = {
_32 = _11;
_41 = _19.2;
_5 = !_7;
_10 = _36;
_24 = _31.1;
_5 = _31.1 as isize;
_34 = 15451_u16 as i16;
_41 = _19.2 << _7;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(1_usize, 36_usize, Move(_36), 18_usize, Move(_18), 10_usize, Move(_10), 30_usize, Move(_30)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(1_usize, 14_usize, Move(_14), 1_usize, Move(_1), 34_usize, Move(_34), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(1_usize, 41_usize, Move(_41), 42_usize, Move(_42), 33_usize, Move(_33), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(1_usize, 4_usize, Move(_4), 38_usize, Move(_38), 24_usize, Move(_24), 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: u128,mut _3: bool,mut _4: i16,mut _5: i16,mut _6: char) -> i64 {
mir! {
type RET = i64;
let _7: Adt46;
let _8: i32;
let _9: u32;
let _10: Adt56;
let _11: *mut usize;
let _12: (i64, isize, i128);
let _13: ([u8; 4], i32, char);
let _14: char;
let _15: *mut usize;
let _16: ([u8; 4], i32, char);
let _17: u16;
let _18: Adt47;
let _19: i32;
let _20: f32;
let _21: i16;
let _22: char;
let _23: usize;
let _24: (i64, isize, i128);
let _25: i32;
let _26: Adt41;
let _27: Adt46;
let _28: bool;
let _29: [i32; 6];
let _30: Adt49;
let _31: [i64; 2];
let _32: Adt56;
let _33: Adt46;
let _34: f64;
let _35: ();
let _36: ();
{
RET = -6956784631879246755_i64;
_2 = !312012548720306729723049183807063553822_u128;
RET = 4826217580158148201_i64 - 6773620800237400659_i64;
RET = 8000805387437505127_i64;
RET = (-4738051816418639463_i64) | (-6997255628262575955_i64);
RET = !1862810772160275421_i64;
_4 = _5 & _5;
_1 = -(-9223372036854775808_isize);
_2 = 52414057988640189674928398486774443568_u128 ^ 335018691492145000776608729483284644077_u128;
_2 = 16485064609846192312751475411476164597_u128 << _1;
_3 = !true;
RET = _6 as i64;
_4 = _5 & _5;
RET = _6 as i64;
RET = 844993861999110971_i64 - (-3901250227036139058_i64);
RET = (-5108471797443227024_i64) + 3324312158086335321_i64;
Goto(bb1)
}
bb1 = {
_7.fld0 = [RET,RET];
RET = 39_u8 as i64;
_4 = _1 as i16;
_8 = -(-1343398750_i32);
RET = 7907919824913086310_i64 * (-894098068020821909_i64);
_5 = _3 as i16;
RET = !7478291159985090873_i64;
RET = 3468804033572239114_i64;
RET = -(-6658727787781328230_i64);
_2 = 82669721154478152933125725989644901571_u128 & 239459886633641189244781294759742732889_u128;
_7.fld0 = [RET,RET];
Call(_7 = fn3(_6, _1, _8, _8, RET, _4, _1, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 4192624213304737365_i64;
_3 = true;
_13.0 = [162_u8,120_u8,154_u8,216_u8];
_13.2 = _6;
_14 = _13.2;
_9 = 3290595116_u32 + 2340659399_u32;
_12.0 = RET << RET;
_12.1 = _1;
_8 = _9 as i32;
_14 = _13.2;
RET = _12.0 >> _1;
_3 = !false;
_13.1 = -_8;
_14 = _13.2;
RET = !_12.0;
_12.2 = (-1064278380394397348992497176369243597_i128) >> _5;
Call(_6 = fn11(RET, _13.2, _9, _13, _1, _1, _14, _1, _13.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = !_13.1;
RET = -_12.0;
_16.2 = _13.2;
_1 = -_12.1;
_2 = 42874288965381928723793729254990600652_u128;
_13.0 = [118_u8,239_u8,17_u8,59_u8];
_5 = 222_u8 as i16;
RET = !_12.0;
_2 = !232806607750499976321698511159271721124_u128;
_9 = !3063218279_u32;
_4 = !_5;
_13.2 = _16.2;
_13.1 = !_8;
_12.0 = RET & RET;
_16.1 = _8 >> _13.1;
Call(_6 = fn12(_16.2, _2, _12, _13.2, _13.0, _13.0, _13, _12, _14, _13, _12, _16.1, _13.0, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12.0 = RET;
_16.0 = [61_u8,197_u8,239_u8,221_u8];
_20 = 46_u8 as f32;
_4 = _5 | _5;
_12 = (RET, _1, 128364806532140337581085636530412651156_i128);
_13.2 = _16.2;
_12 = (RET, _1, (-168972458044001727973810686935029867889_i128));
_13.0 = [52_u8,61_u8,166_u8,218_u8];
_17 = 24698_u16;
_13.1 = _16.1;
_7.fld0 = [_12.0,_12.0];
_16.0 = [179_u8,74_u8,211_u8,235_u8];
_6 = _16.2;
_21 = _4 | _4;
_5 = _21;
_1 = !_12.1;
_24.0 = _12.2 as i64;
_13.2 = _16.2;
_6 = _16.2;
_18 = Adt47::Variant0 { fld0: _12,fld1: _6,fld2: _13.1,fld3: _21 };
place!(Field::<char>(Variant(_18, 0), 1)) = _16.2;
match Field::<(i64, isize, i128)>(Variant(_18, 0), 0).2 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
171309908876936735489563920496738343567 => bb11,
_ => bb10
}
}
bb5 = {
_8 = !_13.1;
RET = -_12.0;
_16.2 = _13.2;
_1 = -_12.1;
_2 = 42874288965381928723793729254990600652_u128;
_13.0 = [118_u8,239_u8,17_u8,59_u8];
_5 = 222_u8 as i16;
RET = !_12.0;
_2 = !232806607750499976321698511159271721124_u128;
_9 = !3063218279_u32;
_4 = !_5;
_13.2 = _16.2;
_13.1 = !_8;
_12.0 = RET & RET;
_16.1 = _8 >> _13.1;
Call(_6 = fn12(_16.2, _2, _12, _13.2, _13.0, _13.0, _13, _12, _14, _13, _12, _16.1, _13.0, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
RET = 4192624213304737365_i64;
_3 = true;
_13.0 = [162_u8,120_u8,154_u8,216_u8];
_13.2 = _6;
_14 = _13.2;
_9 = 3290595116_u32 + 2340659399_u32;
_12.0 = RET << RET;
_12.1 = _1;
_8 = _9 as i32;
_14 = _13.2;
RET = _12.0 >> _1;
_3 = !false;
_13.1 = -_8;
_14 = _13.2;
RET = !_12.0;
_12.2 = (-1064278380394397348992497176369243597_i128) >> _5;
Call(_6 = fn11(RET, _13.2, _9, _13, _1, _1, _14, _1, _13.1), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_7.fld0 = [RET,RET];
RET = 39_u8 as i64;
_4 = _1 as i16;
_8 = -(-1343398750_i32);
RET = 7907919824913086310_i64 * (-894098068020821909_i64);
_5 = _3 as i16;
RET = !7478291159985090873_i64;
RET = 3468804033572239114_i64;
RET = -(-6658727787781328230_i64);
_2 = 82669721154478152933125725989644901571_u128 & 239459886633641189244781294759742732889_u128;
_7.fld0 = [RET,RET];
Call(_7 = fn3(_6, _1, _8, _8, RET, _4, _1, _5, _2), ReturnTo(bb2), UnwindUnreachable())
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
_1 = -Field::<(i64, isize, i128)>(Variant(_18, 0), 0).1;
_1 = Field::<(i64, isize, i128)>(Variant(_18, 0), 0).1;
_9 = 358699997_u32;
_19 = _8;
_25 = _13.1;
_5 = -Field::<i16>(Variant(_18, 0), 3);
place!(Field::<i16>(Variant(_18, 0), 3)) = Field::<char>(Variant(_18, 0), 1) as i16;
_27.fld0 = [_24.0,Field::<(i64, isize, i128)>(Variant(_18, 0), 0).0];
_7 = Adt46 { fld0: _27.fld0 };
match _12.2 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb12,
171309908876936735489563920496738343567 => bb14,
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
_11 = core::ptr::addr_of_mut!(_23);
place!(Field::<i16>(Variant(_18, 0), 3)) = !_5;
_24.2 = !Field::<(i64, isize, i128)>(Variant(_18, 0), 0).2;
_12.1 = _1;
_13.0 = [191_u8,52_u8,5_u8,133_u8];
SetDiscriminant(_18, 1);
_15 = core::ptr::addr_of_mut!(_23);
_2 = 286360616990829912198541146062926808560_u128;
_15 = core::ptr::addr_of_mut!((*_15));
_31 = [RET,_12.0];
_1 = _12.1 - _12.1;
_27.fld0 = [_12.0,_12.0];
_24.2 = _3 as i128;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(2_usize, 19_usize, Move(_19), 4_usize, Move(_4), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(2_usize, 16_usize, Move(_16), 9_usize, Move(_9), 21_usize, Move(_21), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: char,mut _2: isize,mut _3: i32,mut _4: i32,mut _5: i64,mut _6: i16,mut _7: isize,mut _8: i16,mut _9: u128) -> Adt46 {
mir! {
type RET = Adt46;
let _10: Adt56;
let _11: u32;
let _12: f64;
let _13: [i64; 2];
let _14: u8;
let _15: char;
let _16: [i32; 6];
let _17: [i32; 6];
let _18: u8;
let _19: u32;
let _20: isize;
let _21: Adt43;
let _22: i32;
let _23: u128;
let _24: isize;
let _25: isize;
let _26: (i64, isize, i128);
let _27: isize;
let _28: isize;
let _29: f32;
let _30: i32;
let _31: ([u8; 4], i32, char);
let _32: f32;
let _33: ();
let _34: ();
{
_5 = !(-8437424242593042860_i64);
_6 = _8 + _8;
RET.fld0 = [_5,_5];
_4 = _3 * _3;
RET.fld0 = [_5,_5];
_3 = -_4;
_6 = 63909_u16 as i16;
_2 = 4871176206732558279_u64 as isize;
RET.fld0 = [_5,_5];
_9 = 72995442756827204520291288847724460532_u128;
_9 = 204186343159944540480351149024369740843_u128 + 3935358121853780707555353802409087735_u128;
_4 = _3;
_4 = _3;
_12 = _6 as f64;
_3 = 58157_u16 as i32;
_12 = _9 as f64;
_12 = 19373339798126064349333516611146112320_i128 as f64;
_4 = _3 | _3;
Goto(bb1)
}
bb1 = {
_2 = _1 as isize;
_11 = !2664286216_u32;
_9 = !136858199587681237992267730288315190765_u128;
_2 = -_7;
_8 = !_6;
_11 = !3916457660_u32;
_1 = '\u{32c4f}';
_5 = 5_usize as i64;
_3 = -_4;
_6 = _8 - _8;
_8 = _7 as i16;
_9 = 132800371837481358257966190971621505645_u128 + 206188661831307556605494592699143317291_u128;
RET.fld0 = [_5,_5];
_8 = _6;
RET.fld0 = [_5,_5];
RET.fld0 = [_5,_5];
_7 = _2 | _2;
_9 = 164_u8 as u128;
_2 = _7;
_2 = -_7;
RET.fld0 = [_5,_5];
_9 = 317265903455150677932056853752939454954_u128;
_3 = -_4;
_8 = -_6;
Call(_5 = fn4(_9, _7, _2, RET, _7, _1, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = RET.fld0;
_11 = 2567954998_u32 + 1585171648_u32;
RET.fld0 = [_5,_5];
_8 = !_6;
_1 = '\u{105bb}';
_16 = [_3,_3,_4,_4,_4,_4];
_1 = '\u{7c8f3}';
_7 = 63_i8 as isize;
_17 = [_3,_4,_4,_4,_4,_3];
_4 = 222_u8 as i32;
_16 = _17;
RET.fld0 = [_5,_5];
RET = Adt46 { fld0: _13 };
_6 = !_8;
_14 = 99_u8;
_13 = RET.fld0;
Goto(bb3)
}
bb3 = {
RET.fld0 = [_5,_5];
_7 = _14 as isize;
_8 = 14370102417012665854_usize as i16;
RET = Adt46 { fld0: _13 };
RET.fld0 = [_5,_5];
_3 = _4;
_2 = _7 << _4;
_12 = _14 as f64;
_15 = _1;
match _14 {
99 => bb5,
_ => bb4
}
}
bb4 = {
_13 = RET.fld0;
_11 = 2567954998_u32 + 1585171648_u32;
RET.fld0 = [_5,_5];
_8 = !_6;
_1 = '\u{105bb}';
_16 = [_3,_3,_4,_4,_4,_4];
_1 = '\u{7c8f3}';
_7 = 63_i8 as isize;
_17 = [_3,_4,_4,_4,_4,_3];
_4 = 222_u8 as i32;
_16 = _17;
RET.fld0 = [_5,_5];
RET = Adt46 { fld0: _13 };
_6 = !_8;
_14 = 99_u8;
_13 = RET.fld0;
Goto(bb3)
}
bb5 = {
_2 = !_7;
_12 = _11 as f64;
_17 = [_3,_4,_4,_3,_4,_3];
_20 = _2 << _6;
_8 = _6 ^ _6;
_16 = [_4,_4,_3,_3,_4,_4];
_20 = !_7;
_6 = _8 ^ _8;
_15 = _1;
_13 = RET.fld0;
match _14 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
99 => bb10,
_ => bb9
}
}
bb6 = {
_13 = RET.fld0;
_11 = 2567954998_u32 + 1585171648_u32;
RET.fld0 = [_5,_5];
_8 = !_6;
_1 = '\u{105bb}';
_16 = [_3,_3,_4,_4,_4,_4];
_1 = '\u{7c8f3}';
_7 = 63_i8 as isize;
_17 = [_3,_4,_4,_4,_4,_3];
_4 = 222_u8 as i32;
_16 = _17;
RET.fld0 = [_5,_5];
RET = Adt46 { fld0: _13 };
_6 = !_8;
_14 = 99_u8;
_13 = RET.fld0;
Goto(bb3)
}
bb7 = {
RET.fld0 = [_5,_5];
_7 = _14 as isize;
_8 = 14370102417012665854_usize as i16;
RET = Adt46 { fld0: _13 };
RET.fld0 = [_5,_5];
_3 = _4;
_2 = _7 << _4;
_12 = _14 as f64;
_15 = _1;
match _14 {
99 => bb5,
_ => bb4
}
}
bb8 = {
_13 = RET.fld0;
_11 = 2567954998_u32 + 1585171648_u32;
RET.fld0 = [_5,_5];
_8 = !_6;
_1 = '\u{105bb}';
_16 = [_3,_3,_4,_4,_4,_4];
_1 = '\u{7c8f3}';
_7 = 63_i8 as isize;
_17 = [_3,_4,_4,_4,_4,_3];
_4 = 222_u8 as i32;
_16 = _17;
RET.fld0 = [_5,_5];
RET = Adt46 { fld0: _13 };
_6 = !_8;
_14 = 99_u8;
_13 = RET.fld0;
Goto(bb3)
}
bb9 = {
_2 = _1 as isize;
_11 = !2664286216_u32;
_9 = !136858199587681237992267730288315190765_u128;
_2 = -_7;
_8 = !_6;
_11 = !3916457660_u32;
_1 = '\u{32c4f}';
_5 = 5_usize as i64;
_3 = -_4;
_6 = _8 - _8;
_8 = _7 as i16;
_9 = 132800371837481358257966190971621505645_u128 + 206188661831307556605494592699143317291_u128;
RET.fld0 = [_5,_5];
_8 = _6;
RET.fld0 = [_5,_5];
RET.fld0 = [_5,_5];
_7 = _2 | _2;
_9 = 164_u8 as u128;
_2 = _7;
_2 = -_7;
RET.fld0 = [_5,_5];
_9 = 317265903455150677932056853752939454954_u128;
_3 = -_4;
_8 = -_6;
Call(_5 = fn4(_9, _7, _2, RET, _7, _1, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_22 = 12904_u16 as i32;
_16 = _17;
_4 = _5 as i32;
_18 = _14 << _8;
_6 = _8 << _14;
_8 = _6 & _6;
_2 = _7;
RET = Adt46 { fld0: _13 };
_6 = _11 as i16;
_23 = 8888022567821145063_u64 as u128;
_19 = _4 as u32;
_24 = _7 >> _8;
_27 = -_24;
_25 = _24 ^ _24;
_7 = _27;
_19 = _11;
_14 = _18 + _18;
_16 = [_4,_22,_22,_4,_3,_22];
_26.1 = _27;
_19 = _11;
_22 = !_4;
Goto(bb11)
}
bb11 = {
_1 = _15;
_9 = true as u128;
_8 = _6 - _6;
_17 = [_3,_4,_22,_22,_22,_22];
_26.2 = 11735183369661387846975354887782340556_i128 - (-146454701415282845112216294050832226452_i128);
_28 = -_25;
_26.1 = _25 | _25;
_29 = _8 as f32;
_26.1 = 6878818692378829791_u64 as isize;
_26 = (_5, _28, (-15451106868807460430407692162971484076_i128));
match _26.2 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb10,
6 => bb12,
324831260052131003032966915268796727380 => bb14,
_ => bb13
}
}
bb12 = {
_2 = _1 as isize;
_11 = !2664286216_u32;
_9 = !136858199587681237992267730288315190765_u128;
_2 = -_7;
_8 = !_6;
_11 = !3916457660_u32;
_1 = '\u{32c4f}';
_5 = 5_usize as i64;
_3 = -_4;
_6 = _8 - _8;
_8 = _7 as i16;
_9 = 132800371837481358257966190971621505645_u128 + 206188661831307556605494592699143317291_u128;
RET.fld0 = [_5,_5];
_8 = _6;
RET.fld0 = [_5,_5];
RET.fld0 = [_5,_5];
_7 = _2 | _2;
_9 = 164_u8 as u128;
_2 = _7;
_2 = -_7;
RET.fld0 = [_5,_5];
_9 = 317265903455150677932056853752939454954_u128;
_3 = -_4;
_8 = -_6;
Call(_5 = fn4(_9, _7, _2, RET, _7, _1, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_13 = RET.fld0;
_11 = 2567954998_u32 + 1585171648_u32;
RET.fld0 = [_5,_5];
_8 = !_6;
_1 = '\u{105bb}';
_16 = [_3,_3,_4,_4,_4,_4];
_1 = '\u{7c8f3}';
_7 = 63_i8 as isize;
_17 = [_3,_4,_4,_4,_4,_3];
_4 = 222_u8 as i32;
_16 = _17;
RET.fld0 = [_5,_5];
RET = Adt46 { fld0: _13 };
_6 = !_8;
_14 = 99_u8;
_13 = RET.fld0;
Goto(bb3)
}
bb14 = {
_17 = [_4,_22,_3,_4,_3,_22];
_9 = !_23;
_4 = _22;
_18 = _14 << _26.2;
_31.1 = -_3;
_14 = _18 * _18;
_27 = _26.1;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(3_usize, 3_usize, Move(_3), 9_usize, Move(_9), 23_usize, Move(_23), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(3_usize, 17_usize, Move(_17), 1_usize, Move(_1), 8_usize, Move(_8), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(3_usize, 13_usize, Move(_13), 11_usize, Move(_11), 5_usize, Move(_5), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u128,mut _2: isize,mut _3: isize,mut _4: Adt46,mut _5: isize,mut _6: char,mut _7: i16) -> i64 {
mir! {
type RET = i64;
let _8: isize;
let _9: Adt49;
let _10: isize;
let _11: i8;
let _12: isize;
let _13: Adt49;
let _14: u64;
let _15: ([u8; 4], i32, char);
let _16: (i64, isize, i128);
let _17: f32;
let _18: u128;
let _19: (*const i128, i8);
let _20: i16;
let _21: i16;
let _22: Adt55;
let _23: f64;
let _24: usize;
let _25: u32;
let _26: Adt49;
let _27: u32;
let _28: [i32; 6];
let _29: bool;
let _30: i16;
let _31: i64;
let _32: u8;
let _33: i32;
let _34: ([u8; 4], i32, char);
let _35: [u8; 4];
let _36: u64;
let _37: Adt52;
let _38: u64;
let _39: i8;
let _40: [char; 2];
let _41: Adt57;
let _42: bool;
let _43: isize;
let _44: isize;
let _45: isize;
let _46: ();
let _47: ();
{
_7 = 775_i16;
_8 = _2;
_1 = !295056044655221513200080658737043181805_u128;
_1 = 151508557482828577480565789527835721346_u128 & 61050193233566229709998935273709856089_u128;
RET = 6412305173602894580_i64;
_3 = _5;
_6 = '\u{10dbc8}';
_4.fld0 = [RET,RET];
RET = (-6210877370435782513_i64);
_5 = _3 * _3;
_7 = 6359_i16 ^ 1738_i16;
_7 = !(-3300_i16);
_8 = !_5;
_2 = -_5;
_2 = !_8;
_2 = _3;
_7 = !(-27955_i16);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463457163730061332428943 => bb5,
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
_10 = 3474051207917807641_u64 as isize;
_10 = _5;
_10 = _2 & _8;
_8 = (-91_i8) as isize;
_11 = 86_i8;
_3 = _5;
_7 = 27382_i16;
_4.fld0 = [RET,RET];
_2 = _10;
_12 = _6 as isize;
RET = (-302170061798082935_i64) * 5179724583586599181_i64;
_10 = (-159184951854779854502558154139373562798_i128) as isize;
RET = 3854446811_u32 as i64;
_12 = !_2;
_10 = _3;
_10 = _5;
_2 = _11 as isize;
_7 = (-17701_i16);
_3 = -_12;
_7 = !(-16008_i16);
_3 = 172_u8 as isize;
Goto(bb6)
}
bb6 = {
_14 = 2604363498801466883_u64 >> _10;
_16.0 = !RET;
_11 = 26960_u16 as i8;
_16.1 = _1 as isize;
Call(_6 = fn5(_12, _2, _12, _12, _5, _5, _5, _14), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15.1 = 639622388_i32 >> _5;
_12 = _5;
_5 = _2;
_6 = '\u{93326}';
_15.0 = [164_u8,226_u8,252_u8,182_u8];
_11 = 56_i8;
_15.2 = _6;
_14 = !11575063130927419302_u64;
RET = _1 as i64;
_14 = 17190045788890636825_u64 * 6709054603610626510_u64;
_23 = 223_u8 as f64;
_24 = !13018680715227846613_usize;
_7 = !9951_i16;
_18 = _1;
_19.1 = _11 - _11;
_15.2 = _6;
_20 = _12 as i16;
_5 = 3753532112_u32 as isize;
_3 = _10 ^ _10;
_24 = !16141266083743285781_usize;
match _11 {
56 => bb9,
_ => bb8
}
}
bb8 = {
_14 = 2604363498801466883_u64 >> _10;
_16.0 = !RET;
_11 = 26960_u16 as i8;
_16.1 = _1 as isize;
Call(_6 = fn5(_12, _2, _12, _12, _5, _5, _5, _14), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_16 = (RET, _3, 168788675928902176908732754393232610737_i128);
_27 = !335999378_u32;
_16.1 = _8;
_25 = !_27;
_15.1 = _14 as i32;
_11 = -_19.1;
_6 = _15.2;
_16.2 = RET as i128;
_15.2 = _6;
_21 = _7 * _20;
_28 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
RET = _16.0 ^ _16.0;
_16 = (RET, _3, 107389861406510234512301608379529928064_i128);
_6 = _15.2;
_18 = 5590_u16 as u128;
RET = _16.0 & _16.0;
_2 = _16.1;
_6 = _15.2;
_29 = false;
RET = _16.0 + _16.0;
_1 = !_18;
_16 = (RET, _10, 103893832068402450960032873109203746297_i128);
_2 = -_12;
_24 = _3 as usize;
_7 = _20;
Call(_24 = fn8(_16.0, _16.2, _16, _15, _1, _15.0, RET, _8, _16, _25, _16, RET, _15.2, RET, _7, _3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_8 = _3 ^ _3;
_16 = (RET, _3, 82454715017452893200990558415673846682_i128);
_24 = 9192945371139284042_usize;
_24 = 10512179623806066317_usize ^ 2_usize;
_31 = !RET;
_17 = _14 as f32;
_25 = !_27;
_19.1 = _14 as i8;
_29 = !true;
_19.0 = core::ptr::addr_of!(_16.2);
_1 = _18 + _18;
_1 = _18 | _18;
_18 = _1 * _1;
_34.0 = [36_u8,82_u8,17_u8,114_u8];
Goto(bb11)
}
bb11 = {
_18 = !_1;
RET = !_31;
_17 = _14 as f32;
_35 = [129_u8,50_u8,203_u8,43_u8];
RET = -_16.0;
_34 = (_35, _15.1, _6);
_34 = (_15.0, _15.1, _6);
_19.1 = _11;
_25 = _27 ^ _27;
_33 = _34.1;
_21 = _18 as i16;
_30 = -_20;
Call(_18 = fn10(_19.0, _19.0, _16.0, _3, _16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_10 = -_3;
_31 = !_16.0;
_18 = _12 as u128;
_4.fld0 = [_31,RET];
_19.1 = -_11;
_41.fld1 = _15.0;
_19.1 = _11;
match _16.2 {
0 => bb3,
82454715017452893200990558415673846682 => bb14,
_ => bb13
}
}
bb13 = {
_18 = !_1;
RET = !_31;
_17 = _14 as f32;
_35 = [129_u8,50_u8,203_u8,43_u8];
RET = -_16.0;
_34 = (_35, _15.1, _6);
_34 = (_15.0, _15.1, _6);
_19.1 = _11;
_25 = _27 ^ _27;
_33 = _34.1;
_21 = _18 as i16;
_30 = -_20;
Call(_18 = fn10(_19.0, _19.0, _16.0, _3, _16), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_40 = [_15.2,_6];
_5 = _8;
RET = _27 as i64;
_36 = _23 as u64;
_41.fld0.0 = core::ptr::addr_of!(_23);
_41.fld0.1.1 = _29 as i8;
_16 = (_31, _10, (-132496870369794118169019636558560510458_i128));
_19.0 = core::ptr::addr_of!(_16.2);
_16.1 = _5;
_5 = _10 | _3;
_35 = [167_u8,156_u8,186_u8,103_u8];
_41.fld0.2 = _17;
_41.fld0.3 = _24 * _24;
_15.0 = [250_u8,177_u8,204_u8,46_u8];
_21 = !_30;
_16.1 = _5 << _16.2;
_6 = _34.2;
_15 = (_35, _33, _6);
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(4_usize, 20_usize, Move(_20), 36_usize, Move(_36), 3_usize, Move(_3), 40_usize, Move(_40)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(4_usize, 16_usize, Move(_16), 30_usize, Move(_30), 10_usize, Move(_10), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(4_usize, 24_usize, Move(_24), 18_usize, Move(_18), 6_usize, Move(_6), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(4_usize, 34_usize, Move(_34), 27_usize, Move(_27), 47_usize, _47, 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: u64) -> char {
mir! {
type RET = char;
let _9: *mut u32;
let _10: Adt46;
let _11: u32;
let _12: isize;
let _13: [i32; 6];
let _14: ([u8; 4], i32, char);
let _15: bool;
let _16: f32;
let _17: ();
let _18: ();
{
_8 = 5952561546360430878_u64;
_3 = 2893044671582406359_usize as isize;
Call(_7 = fn6(_4, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10.fld0 = [(-710040498146571934_i64),2613686313597820109_i64];
_5 = !_2;
RET = '\u{103b2d}';
_3 = 138_u8 as isize;
RET = '\u{3fa5}';
_10.fld0 = [(-1052367079574972851_i64),(-8479446384998587223_i64)];
_3 = _4;
_1 = 100_i8 as isize;
_1 = !_7;
_1 = !_7;
_7 = RET as isize;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
5952561546360430878 => bb10,
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
_1 = -_7;
RET = '\u{e6919}';
_3 = -_6;
_1 = _6;
_3 = -_5;
_8 = !13465172037733165633_u64;
_7 = _2;
_11 = !1512420107_u32;
RET = '\u{d70e6}';
_2 = _4 >> _4;
Call(_6 = core::intrinsics::bswap(_4), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_8 = RET as u64;
Call(_12 = fn7(_2, RET, _6, _6, _6, _4, _2, _2, _4, _1, _4, _4, _2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_10.fld0 = [2954112727384346356_i64,(-581918563444511921_i64)];
_10.fld0 = [(-465105348358267867_i64),5592145493678496764_i64];
_4 = _11 as isize;
_3 = _2;
_9 = core::ptr::addr_of_mut!(_11);
_5 = _12 * _2;
_2 = 65773826430338118898797331858237142497_i128 as isize;
_10.fld0 = [(-89331166555157212_i64),(-7118324320124505905_i64)];
_4 = 5_usize as isize;
_5 = _6;
_6 = _12;
_6 = -_5;
_11 = 605422691_u32 | 1006773198_u32;
_10.fld0 = [(-5064860338461976731_i64),980347907645276741_i64];
_7 = (-4338743315716615099_i64) as isize;
_4 = 131933756098527327088941962823462047738_u128 as isize;
_10.fld0 = [6932706493019960342_i64,(-8544721241426754717_i64)];
Goto(bb13)
}
bb13 = {
_10.fld0 = [7158569132311107629_i64,7152188815355512093_i64];
_2 = _5 + _5;
_10.fld0 = [561996059520028030_i64,260648608682837348_i64];
_1 = _11 as isize;
_13 = [(-660424544_i32),(-1387058088_i32),1567704803_i32,(-1559511108_i32),83877814_i32,(-1546446157_i32)];
_5 = _2;
_6 = _5;
_8 = true as u64;
_7 = _12 | _2;
_6 = _4;
_14.0 = [13_u8,179_u8,97_u8,156_u8];
RET = '\u{a3c65}';
_15 = (*_9) >= (*_9);
_14.1 = 7732087951385208662_i64 as i32;
_12 = _3;
_4 = _7 - _1;
_14.1 = (-1259224016_i32);
Goto(bb14)
}
bb14 = {
_10.fld0 = [(-6726586199716696896_i64),(-4041153094059290847_i64)];
_11 = 2449168724_u32 + 827491245_u32;
_11 = _3 as u32;
_14.2 = RET;
_1 = !_7;
_6 = 292146729773163266998120176278946840292_u128 as isize;
_14.1 = !1173515226_i32;
_14.0 = [213_u8,67_u8,100_u8,221_u8];
_1 = _4;
_15 = true;
_12 = _3 + _5;
RET = _14.2;
RET = _14.2;
_9 = core::ptr::addr_of_mut!((*_9));
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(5_usize, 8_usize, Move(_8), 7_usize, Move(_7), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_17 = dump_var(5_usize, 12_usize, Move(_12), 6_usize, Move(_6), 18_usize, _18, 18_usize, _18), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: isize,mut _3: isize) -> isize {
mir! {
type RET = isize;
let _4: ([u8; 4], i32, char);
let _5: (i64, isize, i128);
let _6: *mut *mut usize;
let _7: bool;
let _8: Adt52;
let _9: Adt57;
let _10: isize;
let _11: Adt46;
let _12: ([u8; 4], i32, char);
let _13: f32;
let _14: ();
let _15: ();
{
_1 = _3;
_4.2 = '\u{1a14f}';
_3 = !_2;
_4.0 = [110_u8,172_u8,155_u8,175_u8];
RET = _2 * _2;
RET = 7883166890481291859_usize as isize;
_3 = _1 * _1;
Goto(bb1)
}
bb1 = {
_5.1 = 84_u8 as isize;
_4.1 = 368338714_i32;
_3 = _4.2 as isize;
_7 = !false;
_1 = _2;
_5.0 = 5951608858317745386_i64;
_4.1 = _4.2 as i32;
RET = !_2;
_7 = true;
_3 = !_2;
Goto(bb2)
}
bb2 = {
_10 = 4259928230_u32 as isize;
_9.fld0.2 = (-37153490670094959803278614171851142858_i128) as f32;
RET = -_5.1;
_10 = _1 * _3;
_9.fld0.1.1 = -56_i8;
RET = -_2;
_7 = !false;
_11.fld0 = [_5.0,_5.0];
_4.0 = [18_u8,251_u8,58_u8,160_u8];
_9.fld1 = [165_u8,71_u8,165_u8,184_u8];
_2 = _1;
_7 = !true;
_9.fld0.3 = 220_u8 as usize;
match _5.0 {
0 => bb1,
1 => bb3,
5951608858317745386 => bb5,
_ => bb4
}
}
bb3 = {
_5.1 = 84_u8 as isize;
_4.1 = 368338714_i32;
_3 = _4.2 as isize;
_7 = !false;
_1 = _2;
_5.0 = 5951608858317745386_i64;
_4.1 = _4.2 as i32;
RET = !_2;
_7 = true;
_3 = !_2;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_9.fld1 = _4.0;
_5.1 = (-26964290773574789074304022239540252423_i128) as isize;
_9.fld0.3 = 6521540156006985357_usize | 4_usize;
_11.fld0 = [_5.0,_5.0];
_9.fld0.1.0 = core::ptr::addr_of!(_5.2);
RET = _1 | _2;
_4 = (_9.fld1, 1985318794_i32, '\u{d099}');
_9.fld0.2 = _4.1 as f32;
Goto(bb6)
}
bb6 = {
Call(_14 = dump_var(6_usize, 4_usize, Move(_4), 7_usize, Move(_7), 3_usize, Move(_3), 15_usize, _15), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: char,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize) -> isize {
mir! {
type RET = isize;
let _14: isize;
let _15: i16;
let _16: i128;
let _17: f64;
let _18: u8;
let _19: [i64; 2];
let _20: ();
let _21: ();
{
RET = _7;
_15 = 2547_i16;
RET = _1;
_4 = _13 & _1;
_1 = _5 << _13;
_14 = _6;
_3 = _14 ^ _1;
_9 = _3 << _4;
_6 = _11 | _9;
_14 = 3482115202347471357_i64 as isize;
_6 = _15 as isize;
_10 = _13 + _1;
_16 = (-61450254676752634730062436750798376103_i128);
_5 = RET - _13;
_11 = !_7;
_16 = !(-1110748536161919007157006189180000822_i128);
_3 = RET;
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(7_usize, 15_usize, Move(_15), 7_usize, Move(_7), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(7_usize, 6_usize, Move(_6), 13_usize, Move(_13), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i64,mut _2: i128,mut _3: (i64, isize, i128),mut _4: ([u8; 4], i32, char),mut _5: u128,mut _6: [u8; 4],mut _7: i64,mut _8: isize,mut _9: (i64, isize, i128),mut _10: u32,mut _11: (i64, isize, i128),mut _12: i64,mut _13: char,mut _14: i64,mut _15: i16,mut _16: isize) -> usize {
mir! {
type RET = usize;
let _17: [char; 2];
let _18: [u8; 4];
let _19: ([u8; 4], i32, char);
let _20: i16;
let _21: f64;
let _22: [char; 2];
let _23: u32;
let _24: bool;
let _25: f32;
let _26: usize;
let _27: bool;
let _28: Adt56;
let _29: bool;
let _30: ([u8; 4], i32, char);
let _31: Adt46;
let _32: f32;
let _33: usize;
let _34: f32;
let _35: Adt46;
let _36: Adt46;
let _37: i128;
let _38: ();
let _39: ();
{
_15 = !(-4240_i16);
_3.2 = _13 as i128;
_9 = (_7, _16, _3.2);
_4.1 = (-1669765428_i32);
_6 = [113_u8,48_u8,91_u8,134_u8];
_9.0 = !_14;
_16 = -_9.1;
_3 = (_1, _11.1, _11.2);
_9.2 = 32459_u16 as i128;
_9.0 = _12 << _15;
_18 = [239_u8,126_u8,7_u8,123_u8];
_14 = _12 >> _16;
Goto(bb1)
}
bb1 = {
_3 = _9;
_3 = (_11.0, _11.1, _11.2);
_11.2 = _2 << _9.0;
_17 = [_13,_4.2];
_5 = 93428729431178621985267117749695279075_u128;
_11.2 = _2 & _3.2;
_12 = _9.0;
_9 = _11;
RET = _10 as usize;
_17 = [_13,_13];
_12 = _1 & _14;
_2 = _11.2 * _9.2;
RET = _13 as usize;
_18 = _6;
_19.1 = _10 as i32;
_18 = _6;
RET = 6530908939471818965_usize;
_6 = [64_u8,81_u8,255_u8,132_u8];
_1 = _12;
_19.0 = [216_u8,198_u8,84_u8,231_u8];
_19.2 = _4.2;
_4.2 = _13;
Call(_3 = fn9(_9.2, _12, _2, _1, _2, _14, _11.0, _2, _4.2, _9.2, _2, _11, _11, _1, _12, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 9695611713582919382_usize | 3_usize;
_11.2 = -_2;
_9 = (_3.0, _11.1, _2);
_9 = (_3.0, _11.1, _2);
_19.2 = _4.2;
RET = !609985918223370074_usize;
_11 = (_9.0, _16, _3.2);
_21 = 64_u8 as f64;
_10 = !110439482_u32;
_19 = _4;
_22 = [_19.2,_19.2];
_5 = !319993926633870623268782471820780402736_u128;
_23 = _5 as u32;
_12 = -_3.0;
_4.1 = 32_i8 as i32;
_4 = (_6, _19.1, _13);
_4.0 = [65_u8,64_u8,6_u8,22_u8];
_20 = _15 >> _12;
_11.0 = _9.0 >> _9.0;
_9.0 = _12;
RET = 2512521521641363952_usize;
_11.0 = _9.1 as i64;
_25 = _11.2 as f32;
_14 = -_3.0;
Goto(bb3)
}
bb3 = {
_26 = !RET;
_15 = 164_u8 as i16;
_11 = (_9.0, _9.1, _9.2);
_16 = _11.1 << _3.2;
_3.2 = _4.1 as i128;
_15 = _20 + _20;
_4.2 = _19.2;
_24 = false;
_14 = !_9.0;
_3.1 = _20 as isize;
_4 = _19;
_11.1 = _16 >> _15;
_30.2 = _4.2;
_19.2 = _4.2;
_14 = _10 as i64;
_26 = !RET;
_11.0 = _4.2 as i64;
_11.0 = RET as i64;
_11.2 = _2 >> _20;
_13 = _30.2;
_9.2 = _11.2 - _11.2;
match _19.1 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607430098446028 => bb12,
_ => bb11
}
}
bb4 = {
RET = 9695611713582919382_usize | 3_usize;
_11.2 = -_2;
_9 = (_3.0, _11.1, _2);
_9 = (_3.0, _11.1, _2);
_19.2 = _4.2;
RET = !609985918223370074_usize;
_11 = (_9.0, _16, _3.2);
_21 = 64_u8 as f64;
_10 = !110439482_u32;
_19 = _4;
_22 = [_19.2,_19.2];
_5 = !319993926633870623268782471820780402736_u128;
_23 = _5 as u32;
_12 = -_3.0;
_4.1 = 32_i8 as i32;
_4 = (_6, _19.1, _13);
_4.0 = [65_u8,64_u8,6_u8,22_u8];
_20 = _15 >> _12;
_11.0 = _9.0 >> _9.0;
_9.0 = _12;
RET = 2512521521641363952_usize;
_11.0 = _9.1 as i64;
_25 = _11.2 as f32;
_14 = -_3.0;
Goto(bb3)
}
bb5 = {
_3 = _9;
_3 = (_11.0, _11.1, _11.2);
_11.2 = _2 << _9.0;
_17 = [_13,_4.2];
_5 = 93428729431178621985267117749695279075_u128;
_11.2 = _2 & _3.2;
_12 = _9.0;
_9 = _11;
RET = _10 as usize;
_17 = [_13,_13];
_12 = _1 & _14;
_2 = _11.2 * _9.2;
RET = _13 as usize;
_18 = _6;
_19.1 = _10 as i32;
_18 = _6;
RET = 6530908939471818965_usize;
_6 = [64_u8,81_u8,255_u8,132_u8];
_1 = _12;
_19.0 = [216_u8,198_u8,84_u8,231_u8];
_19.2 = _4.2;
_4.2 = _13;
Call(_3 = fn9(_9.2, _12, _2, _1, _2, _14, _11.0, _2, _4.2, _9.2, _2, _11, _11, _1, _12, _11), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb12 = {
_31.fld0 = [_12,_9.0];
_2 = _11.2;
_9.0 = -_3.0;
_19.1 = _4.1;
_8 = _16;
_29 = !_24;
_30.0 = [170_u8,149_u8,147_u8,189_u8];
_26 = _3.0 as usize;
_30 = (_18, _4.1, _13);
_25 = 37481_u16 as f32;
_19.2 = _13;
_11.1 = _8;
_35 = _31;
_18 = [27_u8,119_u8,177_u8,95_u8];
_25 = _21 as f32;
_34 = -_25;
_10 = _25 as u32;
_30.2 = _4.2;
_1 = -_9.0;
RET = _26 | _26;
_30 = (_18, _19.1, _13);
_9.0 = _5 as i64;
_33 = RET;
_35.fld0 = [_1,_1];
_32 = _9.2 as f32;
Goto(bb13)
}
bb13 = {
Call(_38 = dump_var(8_usize, 24_usize, Move(_24), 18_usize, Move(_18), 17_usize, Move(_17), 22_usize, Move(_22)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_38 = dump_var(8_usize, 5_usize, Move(_5), 26_usize, Move(_26), 16_usize, Move(_16), 33_usize, Move(_33)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_38 = dump_var(8_usize, 9_usize, Move(_9), 13_usize, Move(_13), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(8_usize, 14_usize, Move(_14), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i128,mut _2: i64,mut _3: i128,mut _4: i64,mut _5: i128,mut _6: i64,mut _7: i64,mut _8: i128,mut _9: char,mut _10: i128,mut _11: i128,mut _12: (i64, isize, i128),mut _13: (i64, isize, i128),mut _14: i64,mut _15: i64,mut _16: (i64, isize, i128)) -> (i64, isize, i128) {
mir! {
type RET = (i64, isize, i128);
let _17: [u8; 4];
let _18: isize;
let _19: [u8; 4];
let _20: ();
let _21: ();
{
_16.2 = -_10;
RET = (_14, _13.1, _1);
RET.0 = _4;
RET = _16;
RET.2 = _8 << _15;
_16.0 = _15;
_13.1 = RET.1;
_13 = (_2, RET.1, RET.2);
RET.2 = _12.2 >> _11;
_16.2 = RET.2;
_10 = !RET.2;
RET.0 = _13.0 << _13.2;
RET.1 = true as isize;
_12 = RET;
_13.1 = _12.1 & _16.1;
RET.1 = -_13.1;
_14 = _12.0;
_12.0 = -_6;
_17 = [55_u8,197_u8,162_u8,54_u8];
_15 = -_2;
_17 = [150_u8,61_u8,189_u8,220_u8];
_11 = !RET.2;
_18 = _16.1 * RET.1;
_2 = (-2002645511_i32) as i64;
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(9_usize, 14_usize, Move(_14), 2_usize, Move(_2), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(9_usize, 8_usize, Move(_8), 9_usize, Move(_9), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(9_usize, 10_usize, Move(_10), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *const i128,mut _2: *const i128,mut _3: i64,mut _4: isize,mut _5: (i64, isize, i128)) -> u128 {
mir! {
type RET = u128;
let _6: char;
let _7: u8;
let _8: Adt42;
let _9: ([u8; 4], i32, char);
let _10: [i32; 6];
let _11: bool;
let _12: Adt52;
let _13: ();
let _14: ();
{
_4 = _5.1;
_5.1 = _4;
_5.2 = 7_u8 as i128;
_6 = '\u{29f16}';
_7 = 182575577_i32 as u8;
_5.0 = _5.2 as i64;
RET = 197749307842280594457075535055122018436_u128;
_7 = 7891206749560717834_usize as u8;
_6 = '\u{206e9}';
_9.2 = _6;
_1 = core::ptr::addr_of!(_5.2);
_2 = core::ptr::addr_of!((*_1));
_2 = core::ptr::addr_of!((*_2));
Goto(bb1)
}
bb1 = {
_9.1 = 1704356020_i32 * (-1312292645_i32);
_9.2 = _6;
_5.1 = _4;
Goto(bb2)
}
bb2 = {
_10 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_5.0 = _3 << (*_1);
_9.2 = _6;
_4 = RET as isize;
RET = _9.1 as u128;
RET = 75228607337170712283561188003612698253_u128;
_3 = -_5.0;
RET = 74382336789518894969206372667352053449_u128;
_9.0 = [_7,_7,_7,_7];
_11 = true & true;
_9.2 = _6;
_5 = (_3, _4, 112729635727318228787585350523799773971_i128);
_10 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_2 = _1;
_10 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_6 = _9.2;
_10 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_10 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_2 = core::ptr::addr_of!(_5.2);
_9.1 = 1741221693_i32;
RET = 146756992973147088440666586714326715730_u128 >> (*_2);
Goto(bb3)
}
bb3 = {
Call(_13 = dump_var(10_usize, 10_usize, Move(_10), 7_usize, Move(_7), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i64,mut _2: char,mut _3: u32,mut _4: ([u8; 4], i32, char),mut _5: isize,mut _6: isize,mut _7: char,mut _8: isize,mut _9: i32) -> char {
mir! {
type RET = char;
let _10: isize;
let _11: char;
let _12: char;
let _13: isize;
let _14: u8;
let _15: i64;
let _16: isize;
let _17: [char; 2];
let _18: i8;
let _19: &'static f32;
let _20: f32;
let _21: isize;
let _22: f64;
let _23: ([u8; 4], i32, char);
let _24: Adt51;
let _25: ([u8; 4], i32, char);
let _26: *mut [char; 2];
let _27: bool;
let _28: Adt48;
let _29: Adt54;
let _30: f32;
let _31: u8;
let _32: i64;
let _33: f32;
let _34: *const char;
let _35: i64;
let _36: ([u8; 4], i32, char);
let _37: isize;
let _38: Adt54;
let _39: bool;
let _40: f64;
let _41: Adt52;
let _42: isize;
let _43: isize;
let _44: Adt46;
let _45: ();
let _46: ();
{
_5 = _8 + _8;
_9 = _4.1 + _4.1;
RET = _4.2;
_5 = _1 as isize;
RET = _2;
_8 = -_6;
_7 = _4.2;
_2 = _7;
_6 = _3 as isize;
_9 = _4.1 + _4.1;
_6 = _8 ^ _5;
_2 = _7;
_3 = 228706351_u32;
_4.1 = -_9;
_1 = 7157141036153618283_i64 << _9;
_7 = _2;
_6 = _5;
_4.2 = RET;
_9 = -_4.1;
_9 = (-82997329246524540929748859855939633037_i128) as i32;
_11 = RET;
Goto(bb1)
}
bb1 = {
_7 = _2;
_7 = _11;
_6 = _5 - _5;
RET = _7;
_13 = _6;
_2 = _11;
_8 = (-15119_i16) as isize;
_8 = -_13;
_11 = _2;
_14 = 16201_i16 as u8;
_10 = -_6;
_9 = !_4.1;
_4.2 = RET;
_8 = _6;
RET = _7;
_9 = -_4.1;
_7 = RET;
_2 = _7;
_2 = _11;
Goto(bb2)
}
bb2 = {
_15 = _1 + _1;
_4.0 = [_14,_14,_14,_14];
_8 = _6;
_15 = _1 ^ _1;
_10 = !_5;
RET = _11;
_2 = _7;
_7 = _2;
_12 = _2;
_4.0 = [_14,_14,_14,_14];
RET = _4.2;
_11 = _2;
_7 = _4.2;
_14 = 220_u8 >> _6;
_6 = _13 * _8;
_2 = _7;
_14 = true as u8;
_13 = _1 as isize;
RET = _7;
_7 = RET;
_4.1 = _9;
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
228706351 => bb6,
_ => bb5
}
}
bb3 = {
_7 = _2;
_7 = _11;
_6 = _5 - _5;
RET = _7;
_13 = _6;
_2 = _11;
_8 = (-15119_i16) as isize;
_8 = -_13;
_11 = _2;
_14 = 16201_i16 as u8;
_10 = -_6;
_9 = !_4.1;
_4.2 = RET;
_8 = _6;
RET = _7;
_9 = -_4.1;
_7 = RET;
_2 = _7;
_2 = _11;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_1 = -_15;
_14 = _6 as u8;
_4.2 = _2;
_4.0 = [_14,_14,_14,_14];
_14 = 157_u8;
_16 = _6;
_13 = _6;
_10 = -_13;
_3 = 3106869794_u32 & 4025010850_u32;
_7 = _12;
_18 = 89_i8;
_4.1 = !_9;
_17 = [_2,RET];
_13 = _3 as isize;
Goto(bb7)
}
bb7 = {
_20 = (-101830002344683525298915982950246880963_i128) as f32;
_16 = _8 * _6;
_16 = -_13;
_21 = _15 as isize;
_14 = 5_u8 - 180_u8;
_15 = true as i64;
_2 = _7;
_3 = 4089461807_u32;
_23.2 = _7;
_14 = !160_u8;
_16 = _1 as isize;
_9 = _4.1 + _4.1;
_11 = _7;
_23.1 = _16 as i32;
_4.0 = [_14,_14,_14,_14];
match _3 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb9,
4089461807 => bb11,
_ => bb10
}
}
bb8 = {
_1 = -_15;
_14 = _6 as u8;
_4.2 = _2;
_4.0 = [_14,_14,_14,_14];
_14 = 157_u8;
_16 = _6;
_13 = _6;
_10 = -_13;
_3 = 3106869794_u32 & 4025010850_u32;
_7 = _12;
_18 = 89_i8;
_4.1 = !_9;
_17 = [_2,RET];
_13 = _3 as isize;
Goto(bb7)
}
bb9 = {
_7 = _2;
_7 = _11;
_6 = _5 - _5;
RET = _7;
_13 = _6;
_2 = _11;
_8 = (-15119_i16) as isize;
_8 = -_13;
_11 = _2;
_14 = 16201_i16 as u8;
_10 = -_6;
_9 = !_4.1;
_4.2 = RET;
_8 = _6;
RET = _7;
_9 = -_4.1;
_7 = RET;
_2 = _7;
_2 = _11;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_12 = _2;
_4.2 = RET;
RET = _4.2;
_23.0 = [_14,_14,_14,_14];
_4.0 = [_14,_14,_14,_14];
_22 = _3 as f64;
_6 = _23.1 as isize;
_12 = _4.2;
_23.2 = _4.2;
_9 = _4.1 & _23.1;
_27 = !true;
_7 = _12;
_22 = (-160615307483677791414842791002103607098_i128) as f64;
_11 = _23.2;
_25.2 = _11;
_18 = (-48_i8) ^ (-3_i8);
_23.2 = _2;
_28.fld6 = core::ptr::addr_of!(_28.fld2.2);
_19 = &_30;
_7 = RET;
Goto(bb12)
}
bb12 = {
_6 = -_16;
_20 = 164841851840156385008149949448517905309_u128 as f32;
_16 = -_21;
_6 = -_16;
_25.0 = _4.0;
_29 = Adt54::Variant1 { fld0: _22 };
_28.fld2.2 = _3 as i128;
_23.0 = _25.0;
_27 = !true;
_16 = _8;
_23.0 = _25.0;
_12 = _25.2;
_25.1 = (-21516_i16) as i32;
_36.2 = _2;
_16 = !_13;
SetDiscriminant(_29, 2);
_11 = _7;
_29 = Adt54::Variant1 { fld0: _22 };
_28.fld4 = _23.0;
_40 = Field::<f64>(Variant(_29, 1), 0);
Goto(bb13)
}
bb13 = {
_28.fld1.0 = core::ptr::addr_of!(_2);
_23.2 = _11;
_28.fld2 = (_15, _10, (-7143907497044916418868472607639144417_i128));
_16 = _13;
_10 = _8 - _6;
SetDiscriminant(_29, 1);
_2 = _7;
_23.2 = _4.2;
_3 = 4148157944_u32;
place!(Field::<f64>(Variant(_29, 1), 0)) = -_22;
_17 = [_2,_4.2];
_36.2 = _25.2;
_32 = -_28.fld2.0;
_3 = !869757427_u32;
_25 = (_4.0, _9, _36.2);
_28.fld2 = (_1, _10, (-57368433772594594617943397078606277427_i128));
_22 = Field::<f64>(Variant(_29, 1), 0) * _40;
_36.1 = _28.fld2.1 as i32;
_28.fld4 = _25.0;
_36.1 = _9 * _25.1;
_25.1 = _4.1 - _23.1;
_30 = _20;
_4.0 = [_14,_14,_14,_14];
match _28.fld2.2 {
0 => bb2,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
282913933148343868845431210353161934029 => bb19,
_ => bb18
}
}
bb14 = {
_7 = _2;
_7 = _11;
_6 = _5 - _5;
RET = _7;
_13 = _6;
_2 = _11;
_8 = (-15119_i16) as isize;
_8 = -_13;
_11 = _2;
_14 = 16201_i16 as u8;
_10 = -_6;
_9 = !_4.1;
_4.2 = RET;
_8 = _6;
RET = _7;
_9 = -_4.1;
_7 = RET;
_2 = _7;
_2 = _11;
Goto(bb2)
}
bb15 = {
_12 = _2;
_4.2 = RET;
RET = _4.2;
_23.0 = [_14,_14,_14,_14];
_4.0 = [_14,_14,_14,_14];
_22 = _3 as f64;
_6 = _23.1 as isize;
_12 = _4.2;
_23.2 = _4.2;
_9 = _4.1 & _23.1;
_27 = !true;
_7 = _12;
_22 = (-160615307483677791414842791002103607098_i128) as f64;
_11 = _23.2;
_25.2 = _11;
_18 = (-48_i8) ^ (-3_i8);
_23.2 = _2;
_28.fld6 = core::ptr::addr_of!(_28.fld2.2);
_19 = &_30;
_7 = RET;
Goto(bb12)
}
bb16 = {
_7 = _2;
_7 = _11;
_6 = _5 - _5;
RET = _7;
_13 = _6;
_2 = _11;
_8 = (-15119_i16) as isize;
_8 = -_13;
_11 = _2;
_14 = 16201_i16 as u8;
_10 = -_6;
_9 = !_4.1;
_4.2 = RET;
_8 = _6;
RET = _7;
_9 = -_4.1;
_7 = RET;
_2 = _7;
_2 = _11;
Goto(bb2)
}
bb17 = {
Return()
}
bb18 = {
_1 = -_15;
_14 = _6 as u8;
_4.2 = _2;
_4.0 = [_14,_14,_14,_14];
_14 = 157_u8;
_16 = _6;
_13 = _6;
_10 = -_13;
_3 = 3106869794_u32 & 4025010850_u32;
_7 = _12;
_18 = 89_i8;
_4.1 = !_9;
_17 = [_2,RET];
_13 = _3 as isize;
Goto(bb7)
}
bb19 = {
_42 = !_6;
_42 = _16;
_23.2 = _12;
_23.1 = _9 << _8;
_25.0 = [_14,_14,_14,_14];
SetDiscriminant(_29, 2);
_28.fld2.2 = (-37218798890378429298469063028433525203_i128);
_33 = _20;
_28.fld2.2 = !(-136547688492763888899327285266879368319_i128);
_40 = _22 * _22;
_6 = !_10;
_44.fld0 = [_32,_28.fld2.0];
Goto(bb20)
}
bb20 = {
Call(_45 = dump_var(11_usize, 17_usize, Move(_17), 42_usize, Move(_42), 18_usize, Move(_18), 15_usize, Move(_15)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_45 = dump_var(11_usize, 8_usize, Move(_8), 6_usize, Move(_6), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_45 = dump_var(11_usize, 23_usize, Move(_23), 32_usize, Move(_32), 9_usize, Move(_9), 21_usize, Move(_21)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: char,mut _2: u128,mut _3: (i64, isize, i128),mut _4: char,mut _5: [u8; 4],mut _6: [u8; 4],mut _7: ([u8; 4], i32, char),mut _8: (i64, isize, i128),mut _9: char,mut _10: ([u8; 4], i32, char),mut _11: (i64, isize, i128),mut _12: i32,mut _13: [u8; 4],mut _14: i16) -> char {
mir! {
type RET = char;
let _15: [i64; 2];
let _16: *mut u32;
let _17: Adt46;
let _18: *const (*const i128, i8);
let _19: i64;
let _20: *const (*const char,);
let _21: u8;
let _22: f32;
let _23: ([u8; 4], i32, char);
let _24: (*const i128, i8);
let _25: Adt51;
let _26: f64;
let _27: char;
let _28: Adt46;
let _29: [u8; 4];
let _30: ([u8; 4], i32, char);
let _31: *const i128;
let _32: f32;
let _33: Adt46;
let _34: Adt46;
let _35: Adt57;
let _36: *mut u32;
let _37: (i64, isize, i128);
let _38: i8;
let _39: [i32; 6];
let _40: isize;
let _41: [u8; 4];
let _42: f64;
let _43: *const f64;
let _44: ();
let _45: ();
{
_15 = [_11.0,_11.0];
_10 = (_7.0, _12, _9);
_4 = _7.2;
_12 = _10.1 - _10.1;
_14 = _10.2 as i16;
_11.2 = -_3.2;
_6 = [146_u8,133_u8,169_u8,118_u8];
_10.1 = _7.1 ^ _12;
_5 = _10.0;
_15 = [_3.0,_3.0];
_6 = _13;
_3 = (_8.0, _8.1, _8.2);
_3.1 = _11.1;
_8.1 = _3.1;
_15 = [_11.0,_11.0];
_10 = (_7.0, _7.1, _4);
RET = _7.2;
_11.1 = _3.1;
Goto(bb1)
}
bb1 = {
_11.0 = _3.0 - _3.0;
_8.1 = 7986072712865328044_usize as isize;
_8 = _11;
_13 = [22_u8,130_u8,23_u8,162_u8];
_7.0 = [85_u8,187_u8,20_u8,162_u8];
_19 = _3.0 >> _12;
_17 = Adt46 { fld0: _15 };
_12 = 1166222614_u32 as i32;
_9 = RET;
_3 = _8;
_10 = (_7.0, _7.1, _9);
_6 = [231_u8,158_u8,235_u8,24_u8];
_17.fld0 = [_19,_8.0];
_17.fld0 = [_3.0,_19];
_12 = _7.1;
_3.2 = _14 as i128;
_10 = _7;
_5 = [158_u8,174_u8,181_u8,244_u8];
_3 = (_19, _11.1, _8.2);
_11 = (_19, _3.1, _8.2);
_3.2 = _11.0 as i128;
_1 = _7.2;
RET = _1;
_11.2 = _3.2 << _8.0;
_10.2 = _7.2;
_11.2 = _19 as i128;
_1 = RET;
_3 = _11;
Goto(bb2)
}
bb2 = {
_3.2 = !_11.2;
_12 = -_10.1;
_11.0 = RET as i64;
_15 = [_19,_19];
_4 = _10.2;
_7.2 = _4;
_17 = Adt46 { fld0: _15 };
_13 = _7.0;
_7 = _10;
_4 = _7.2;
_3 = (_8.0, _11.1, _11.2);
_8 = _11;
_23.2 = _10.2;
Goto(bb3)
}
bb3 = {
_23.1 = _11.0 as i32;
_24.0 = core::ptr::addr_of!(_3.2);
Goto(bb4)
}
bb4 = {
_23.1 = 4076274725_u32 as i32;
_8.2 = -_11.2;
_11.0 = _10.2 as i64;
_12 = !_10.1;
_4 = _23.2;
RET = _7.2;
_7.1 = _10.1 & _10.1;
_23 = _7;
_18 = core::ptr::addr_of!(_24);
_17.fld0 = _15;
_2 = 318309535302187564340861740144253849961_u128;
_7.2 = _10.2;
_6 = [40_u8,185_u8,230_u8,12_u8];
_10.0 = [219_u8,154_u8,105_u8,198_u8];
_17 = Adt46 { fld0: _15 };
_17.fld0 = [_3.0,_3.0];
_10.0 = [5_u8,24_u8,201_u8,229_u8];
_24.0 = core::ptr::addr_of!(_8.2);
_11.2 = !_3.2;
_21 = 143_u8 * 66_u8;
_7 = (_23.0, _23.1, _23.2);
_10 = (_13, _7.1, _1);
_15 = _17.fld0;
_11.1 = _8.1;
_11.0 = _19 & _19;
_3.0 = -_19;
_27 = _23.2;
Call(_23.1 = fn13(_3.0, _11.0, _11.0, _3.1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_28 = _17;
_23.2 = RET;
_8 = (_3.0, _3.1, _3.2);
_8.0 = -_11.0;
_30.2 = RET;
RET = _7.2;
_23.1 = _21 as i32;
_12 = -_23.1;
_14 = _8.2 as i16;
_32 = _11.2 as f32;
_30 = (_6, _10.1, _7.2);
match _2 {
0 => bb3,
1 => bb2,
2 => bb6,
318309535302187564340861740144253849961 => bb8,
_ => bb7
}
}
bb6 = {
_23.1 = 4076274725_u32 as i32;
_8.2 = -_11.2;
_11.0 = _10.2 as i64;
_12 = !_10.1;
_4 = _23.2;
RET = _7.2;
_7.1 = _10.1 & _10.1;
_23 = _7;
_18 = core::ptr::addr_of!(_24);
_17.fld0 = _15;
_2 = 318309535302187564340861740144253849961_u128;
_7.2 = _10.2;
_6 = [40_u8,185_u8,230_u8,12_u8];
_10.0 = [219_u8,154_u8,105_u8,198_u8];
_17 = Adt46 { fld0: _15 };
_17.fld0 = [_3.0,_3.0];
_10.0 = [5_u8,24_u8,201_u8,229_u8];
_24.0 = core::ptr::addr_of!(_8.2);
_11.2 = !_3.2;
_21 = 143_u8 * 66_u8;
_7 = (_23.0, _23.1, _23.2);
_10 = (_13, _7.1, _1);
_15 = _17.fld0;
_11.1 = _8.1;
_11.0 = _19 & _19;
_3.0 = -_19;
_27 = _23.2;
Call(_23.1 = fn13(_3.0, _11.0, _11.0, _3.1), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_23.1 = _11.0 as i32;
_24.0 = core::ptr::addr_of!(_3.2);
Goto(bb4)
}
bb8 = {
_30.2 = RET;
_17 = Adt46 { fld0: _28.fld0 };
_15 = [_11.0,_19];
_8.1 = _3.1;
_23 = (_10.0, _30.1, RET);
_33 = _28;
_22 = -_32;
_21 = !136_u8;
_22 = _32 + _32;
_30.1 = _14 as i32;
_10.0 = [_21,_21,_21,_21];
_2 = 229589712178195758812648817102946699391_u128 & 226665585373459720507598271754507369165_u128;
_35.fld0.1.1 = -(-116_i8);
_35.fld0.1.0 = core::ptr::addr_of!(_37.2);
Call(_8.2 = core::intrinsics::bswap(_11.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_13 = _30.0;
_37.1 = _3.0 as isize;
_6 = [_21,_21,_21,_21];
_11.1 = _37.1 | _37.1;
_35.fld1 = [_21,_21,_21,_21];
_9 = _10.2;
_7 = _30;
_1 = _23.2;
_35.fld0.0 = core::ptr::addr_of!(_26);
_37.2 = _8.2;
_23.2 = _10.2;
_15 = [_11.0,_19];
_13 = [_21,_21,_21,_21];
_12 = _30.1 >> _11.1;
RET = _1;
_8.2 = -_3.2;
_15 = _28.fld0;
_32 = _11.1 as f32;
_35.fld0.1 = ((*_18).0, (-113_i8));
_8 = _11;
_34.fld0 = [_8.0,_8.0];
_3.2 = _11.2 | _11.2;
_8 = (_11.0, _11.1, _11.2);
_35.fld0.3 = 9617677293002814377_usize ^ 2_usize;
_26 = 58083_u16 as f64;
_30.1 = _12;
_11.0 = _8.0 + _8.0;
match _35.fld0.1.1 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
340282366920938463463374607431768211343 => bb15,
_ => bb14
}
}
bb10 = {
_3.2 = !_11.2;
_12 = -_10.1;
_11.0 = RET as i64;
_15 = [_19,_19];
_4 = _10.2;
_7.2 = _4;
_17 = Adt46 { fld0: _15 };
_13 = _7.0;
_7 = _10;
_4 = _7.2;
_3 = (_8.0, _11.1, _11.2);
_8 = _11;
_23.2 = _10.2;
Goto(bb3)
}
bb11 = {
_23.1 = _11.0 as i32;
_24.0 = core::ptr::addr_of!(_3.2);
Goto(bb4)
}
bb12 = {
_23.1 = 4076274725_u32 as i32;
_8.2 = -_11.2;
_11.0 = _10.2 as i64;
_12 = !_10.1;
_4 = _23.2;
RET = _7.2;
_7.1 = _10.1 & _10.1;
_23 = _7;
_18 = core::ptr::addr_of!(_24);
_17.fld0 = _15;
_2 = 318309535302187564340861740144253849961_u128;
_7.2 = _10.2;
_6 = [40_u8,185_u8,230_u8,12_u8];
_10.0 = [219_u8,154_u8,105_u8,198_u8];
_17 = Adt46 { fld0: _15 };
_17.fld0 = [_3.0,_3.0];
_10.0 = [5_u8,24_u8,201_u8,229_u8];
_24.0 = core::ptr::addr_of!(_8.2);
_11.2 = !_3.2;
_21 = 143_u8 * 66_u8;
_7 = (_23.0, _23.1, _23.2);
_10 = (_13, _7.1, _1);
_15 = _17.fld0;
_11.1 = _8.1;
_11.0 = _19 & _19;
_3.0 = -_19;
_27 = _23.2;
Call(_23.1 = fn13(_3.0, _11.0, _11.0, _3.1), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_28 = _17;
_23.2 = RET;
_8 = (_3.0, _3.1, _3.2);
_8.0 = -_11.0;
_30.2 = RET;
RET = _7.2;
_23.1 = _21 as i32;
_12 = -_23.1;
_14 = _8.2 as i16;
_32 = _11.2 as f32;
_30 = (_6, _10.1, _7.2);
match _2 {
0 => bb3,
1 => bb2,
2 => bb6,
318309535302187564340861740144253849961 => bb8,
_ => bb7
}
}
bb14 = {
_23.1 = 4076274725_u32 as i32;
_8.2 = -_11.2;
_11.0 = _10.2 as i64;
_12 = !_10.1;
_4 = _23.2;
RET = _7.2;
_7.1 = _10.1 & _10.1;
_23 = _7;
_18 = core::ptr::addr_of!(_24);
_17.fld0 = _15;
_2 = 318309535302187564340861740144253849961_u128;
_7.2 = _10.2;
_6 = [40_u8,185_u8,230_u8,12_u8];
_10.0 = [219_u8,154_u8,105_u8,198_u8];
_17 = Adt46 { fld0: _15 };
_17.fld0 = [_3.0,_3.0];
_10.0 = [5_u8,24_u8,201_u8,229_u8];
_24.0 = core::ptr::addr_of!(_8.2);
_11.2 = !_3.2;
_21 = 143_u8 * 66_u8;
_7 = (_23.0, _23.1, _23.2);
_10 = (_13, _7.1, _1);
_15 = _17.fld0;
_11.1 = _8.1;
_11.0 = _19 & _19;
_3.0 = -_19;
_27 = _23.2;
Call(_23.1 = fn13(_3.0, _11.0, _11.0, _3.1), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_37.2 = -_3.2;
_23.2 = _30.2;
_23.1 = _26 as i32;
_2 = 35767709435297886155964059342044108855_u128 | 37805883529580426677149729229189815151_u128;
_39 = [_12,_30.1,_7.1,_12,_30.1,_30.1];
_8 = _11;
_8.1 = _11.1;
_35.fld0.3 = 6_usize | 5_usize;
_35.fld0.1.0 = core::ptr::addr_of!(_8.2);
_9 = RET;
_32 = _22;
_4 = _9;
_11.0 = _19;
_22 = _14 as f32;
_27 = _23.2;
_30.1 = _35.fld0.1.1 as i32;
_4 = _10.2;
_4 = _1;
_10.1 = -_7.1;
_23.2 = _30.2;
_35.fld0.3 = 3_usize ^ 1_usize;
_33.fld0 = [_8.0,_19];
_35.fld1 = [_21,_21,_21,_21];
_24.1 = _19 as i8;
_42 = -_26;
_38 = _11.1 as i8;
_17 = Adt46 { fld0: _28.fld0 };
_29 = [_21,_21,_21,_21];
Goto(bb16)
}
bb16 = {
Call(_44 = dump_var(12_usize, 27_usize, Move(_27), 9_usize, Move(_9), 3_usize, Move(_3), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(12_usize, 39_usize, Move(_39), 10_usize, Move(_10), 14_usize, Move(_14), 29_usize, Move(_29)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(12_usize, 13_usize, Move(_13), 6_usize, Move(_6), 4_usize, Move(_4), 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: isize) -> i32 {
mir! {
type RET = i32;
let _5: Adt46;
let _6: Adt46;
let _7: [i32; 6];
let _8: *const f64;
let _9: u64;
let _10: u128;
let _11: char;
let _12: bool;
let _13: [i32; 6];
let _14: Adt45;
let _15: [i64; 2];
let _16: [char; 2];
let _17: [char; 2];
let _18: bool;
let _19: (*const i128, i8);
let _20: i16;
let _21: Adt46;
let _22: [u8; 4];
let _23: u16;
let _24: char;
let _25: [i64; 2];
let _26: ([u8; 4], i32, char);
let _27: Adt46;
let _28: (i64, isize, i128);
let _29: f64;
let _30: usize;
let _31: bool;
let _32: isize;
let _33: *mut isize;
let _34: Adt46;
let _35: (i64, isize, i128);
let _36: (i64, isize, i128);
let _37: Adt49;
let _38: f32;
let _39: isize;
let _40: [char; 2];
let _41: *mut usize;
let _42: ();
let _43: ();
{
_4 = 9223372036854775807_isize;
RET = 27692_i16 as i32;
_5.fld0 = [_3,_2];
_6 = Adt46 { fld0: _5.fld0 };
_4 = (-9223372036854775808_isize) << _3;
_7 = [RET,RET,RET,RET,RET,RET];
_6 = _5;
_1 = !_3;
_6 = _5;
_6 = Adt46 { fld0: _5.fld0 };
_1 = _4 as i64;
_7 = [RET,RET,RET,RET,RET,RET];
_6 = Adt46 { fld0: _5.fld0 };
_4 = 9223372036854775807_isize;
_5.fld0 = _6.fld0;
_9 = RET as u64;
_5 = Adt46 { fld0: _6.fld0 };
_11 = '\u{fc0c4}';
Goto(bb1)
}
bb1 = {
_6 = Adt46 { fld0: _5.fld0 };
_6.fld0 = _5.fld0;
_5 = _6;
_3 = _1 - _1;
RET = 111128137890753413893495919404379326105_u128 as i32;
_1 = -_3;
_7 = [RET,RET,RET,RET,RET,RET];
RET = (-1991445508_i32);
_10 = 893467977_u32 as u128;
_12 = true;
_4 = (-9223372036854775808_isize);
RET = 1018525249_u32 as i32;
RET = 1952147530_i32 * 605793143_i32;
_9 = !1959677756917731568_u64;
_5.fld0 = [_2,_3];
_1 = _9 as i64;
_5 = Adt46 { fld0: _6.fld0 };
_6 = Adt46 { fld0: _5.fld0 };
Call(_8 = fn14(_5, _3, _3, _3, _5, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = [RET,RET,RET,RET,RET,RET];
_7 = _13;
_2 = _9 as i64;
_13 = [RET,RET,RET,RET,RET,RET];
RET = 972730501_i32;
_16 = [_11,_11];
_2 = _3;
_2 = _3;
RET = (-1497571837_i32) & (-185488782_i32);
_15 = [_2,_3];
_12 = false;
_6 = _5;
_13 = [RET,RET,RET,RET,RET,RET];
_7 = [RET,RET,RET,RET,RET,RET];
_16 = [_11,_11];
_9 = 1954760573967056471_u64 + 5736137535586016009_u64;
_6 = Adt46 { fld0: _15 };
_15 = _5.fld0;
_4 = _11 as isize;
_6.fld0 = _5.fld0;
_12 = !true;
Goto(bb3)
}
bb3 = {
_11 = '\u{17872}';
_13 = [RET,RET,RET,RET,RET,RET];
_18 = _12;
_6.fld0 = _15;
RET = _18 as i32;
_5 = Adt46 { fld0: _6.fld0 };
_10 = 138100344745477948887091177725654203274_u128;
_19.1 = 64_i8 | (-37_i8);
_16 = [_11,_11];
_13 = [RET,RET,RET,RET,RET,RET];
_17 = [_11,_11];
_15 = _5.fld0;
_16 = [_11,_11];
_13 = [RET,RET,RET,RET,RET,RET];
RET = 642282463_i32 ^ (-2044605657_i32);
RET = _9 as i32;
_7 = [RET,RET,RET,RET,RET,RET];
Goto(bb4)
}
bb4 = {
_2 = _3 << _3;
_9 = !3106843700920117140_u64;
_16 = [_11,_11];
_6 = Adt46 { fld0: _5.fld0 };
_9 = 4028980492333683672_u64 * 14643009222661264680_u64;
_20 = 9541_i16;
_23 = 65459_u16;
_2 = _20 as i64;
_5.fld0 = [_3,_3];
_11 = '\u{c70bd}';
_9 = 16156175466451873508_u64;
Goto(bb5)
}
bb5 = {
_21.fld0 = [_3,_3];
_9 = 140_u8 as u64;
_2 = _3 ^ _3;
_6.fld0 = [_3,_3];
_2 = 3759602108_u32 as i64;
_3 = _2;
_13 = _7;
_17 = [_11,_11];
_15 = [_2,_1];
_20 = _11 as i16;
RET = (-1765627103_i32);
_22 = [108_u8,115_u8,225_u8,2_u8];
_18 = _12;
_26 = (_22, RET, _11);
_20 = _1 as i16;
_19.1 = 77_i8 >> _20;
_23 = !39996_u16;
_15 = _5.fld0;
_4 = 9223372036854775807_isize;
_25 = _6.fld0;
_19.1 = (-67_i8);
_28.2 = !83017892573372592151513251394208222936_i128;
_28.1 = _4 ^ _4;
_22 = [87_u8,254_u8,84_u8,41_u8];
_5.fld0 = [_1,_1];
_19.0 = core::ptr::addr_of!(_28.2);
_27 = Adt46 { fld0: _21.fld0 };
Goto(bb6)
}
bb6 = {
_3 = _4 as i64;
RET = _26.1;
_26.2 = _11;
_6.fld0 = _15;
_28.1 = 16131697273293127621_usize as isize;
_19.1 = 32_i8 * (-110_i8);
_26 = (_22, RET, _11);
_29 = _9 as f64;
_12 = _18;
match RET {
0 => bb7,
340282366920938463463374607430002584353 => bb9,
_ => bb8
}
}
bb7 = {
_13 = [RET,RET,RET,RET,RET,RET];
_7 = _13;
_2 = _9 as i64;
_13 = [RET,RET,RET,RET,RET,RET];
RET = 972730501_i32;
_16 = [_11,_11];
_2 = _3;
_2 = _3;
RET = (-1497571837_i32) & (-185488782_i32);
_15 = [_2,_3];
_12 = false;
_6 = _5;
_13 = [RET,RET,RET,RET,RET,RET];
_7 = [RET,RET,RET,RET,RET,RET];
_16 = [_11,_11];
_9 = 1954760573967056471_u64 + 5736137535586016009_u64;
_6 = Adt46 { fld0: _15 };
_15 = _5.fld0;
_4 = _11 as isize;
_6.fld0 = _5.fld0;
_12 = !true;
Goto(bb3)
}
bb8 = {
_11 = '\u{17872}';
_13 = [RET,RET,RET,RET,RET,RET];
_18 = _12;
_6.fld0 = _15;
RET = _18 as i32;
_5 = Adt46 { fld0: _6.fld0 };
_10 = 138100344745477948887091177725654203274_u128;
_19.1 = 64_i8 | (-37_i8);
_16 = [_11,_11];
_13 = [RET,RET,RET,RET,RET,RET];
_17 = [_11,_11];
_15 = _5.fld0;
_16 = [_11,_11];
_13 = [RET,RET,RET,RET,RET,RET];
RET = 642282463_i32 ^ (-2044605657_i32);
RET = _9 as i32;
_7 = [RET,RET,RET,RET,RET,RET];
Goto(bb4)
}
bb9 = {
_3 = _1;
_4 = -_28.1;
_5.fld0 = _6.fld0;
_30 = !2_usize;
_22 = _26.0;
_5 = Adt46 { fld0: _21.fld0 };
_25 = [_3,_1];
Call(RET = fn18(_27.fld0, _27, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_23 = !49470_u16;
_21 = Adt46 { fld0: _5.fld0 };
_26.0 = [51_u8,196_u8,98_u8,90_u8];
_24 = _26.2;
_3 = _1 >> _19.1;
_25 = [_1,_3];
_28 = (_2, _4, (-60791340220905957763068604433603484734_i128));
_32 = _28.1;
RET = _26.1;
_11 = _24;
_22 = _26.0;
Goto(bb11)
}
bb11 = {
RET = _26.1;
_33 = core::ptr::addr_of_mut!(_4);
_25 = [_3,_3];
_13 = _7;
_27.fld0 = [_28.0,_3];
_3 = _1;
_28.0 = _2;
_11 = _24;
_27.fld0 = _6.fld0;
match _28.2 {
0 => bb1,
1 => bb8,
2 => bb7,
3 => bb10,
279491026700032505700306002998164726722 => bb13,
_ => bb12
}
}
bb12 = {
_11 = '\u{17872}';
_13 = [RET,RET,RET,RET,RET,RET];
_18 = _12;
_6.fld0 = _15;
RET = _18 as i32;
_5 = Adt46 { fld0: _6.fld0 };
_10 = 138100344745477948887091177725654203274_u128;
_19.1 = 64_i8 | (-37_i8);
_16 = [_11,_11];
_13 = [RET,RET,RET,RET,RET,RET];
_17 = [_11,_11];
_15 = _5.fld0;
_16 = [_11,_11];
_13 = [RET,RET,RET,RET,RET,RET];
RET = 642282463_i32 ^ (-2044605657_i32);
RET = _9 as i32;
_7 = [RET,RET,RET,RET,RET,RET];
Goto(bb4)
}
bb13 = {
_25 = [_3,_1];
_4 = _30 as isize;
_31 = !_18;
_29 = _23 as f64;
RET = _9 as i32;
_21.fld0 = [_2,_2];
_31 = _12 & _18;
_34 = _6;
_20 = (-18295_i16);
_32 = _1 as isize;
_28.1 = _26.1 as isize;
_35.1 = (*_33);
match _20 {
0 => bb8,
1 => bb10,
2 => bb9,
340282366920938463463374607431768193161 => bb15,
_ => bb14
}
}
bb14 = {
_13 = [RET,RET,RET,RET,RET,RET];
_7 = _13;
_2 = _9 as i64;
_13 = [RET,RET,RET,RET,RET,RET];
RET = 972730501_i32;
_16 = [_11,_11];
_2 = _3;
_2 = _3;
RET = (-1497571837_i32) & (-185488782_i32);
_15 = [_2,_3];
_12 = false;
_6 = _5;
_13 = [RET,RET,RET,RET,RET,RET];
_7 = [RET,RET,RET,RET,RET,RET];
_16 = [_11,_11];
_9 = 1954760573967056471_u64 + 5736137535586016009_u64;
_6 = Adt46 { fld0: _15 };
_15 = _5.fld0;
_4 = _11 as isize;
_6.fld0 = _5.fld0;
_12 = !true;
Goto(bb3)
}
bb15 = {
_30 = 240_u8 as usize;
_23 = 7324_u16;
_38 = 52_u8 as f32;
_27 = Adt46 { fld0: _15 };
_28 = (_2, (*_33), (-10104525320536624410336176162482378880_i128));
_36 = (_2, (*_33), _28.2);
_21 = Adt46 { fld0: _5.fld0 };
_7 = _13;
_27 = Adt46 { fld0: _15 };
_36 = _28;
_12 = (*_33) == _35.1;
_25 = [_28.0,_3];
_35.0 = !_3;
_34 = Adt46 { fld0: _15 };
_19.0 = core::ptr::addr_of!(_28.2);
_25 = [_28.0,_36.0];
_35 = _36;
_5.fld0 = [_35.0,_2];
_30 = !4102388905000188722_usize;
_28.1 = (*_33);
_27.fld0 = _21.fld0;
_35 = _28;
_28.1 = _36.1;
_21 = _27;
_26.0 = _22;
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(13_usize, 31_usize, Move(_31), 13_usize, Move(_13), 11_usize, Move(_11), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(13_usize, 20_usize, Move(_20), 2_usize, Move(_2), 25_usize, Move(_25), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(13_usize, 15_usize, Move(_15), 36_usize, Move(_36), 26_usize, Move(_26), 16_usize, Move(_16)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(13_usize, 24_usize, Move(_24), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: Adt46,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: Adt46,mut _6: i64) -> *const f64 {
mir! {
type RET = *const f64;
let _7: isize;
let _8: u128;
let _9: *const (*const char,);
let _10: u64;
let _11: Adt46;
let _12: [i64; 2];
let _13: isize;
let _14: char;
let _15: isize;
let _16: Adt42;
let _17: u16;
let _18: Adt46;
let _19: u32;
let _20: isize;
let _21: i64;
let _22: ([u8; 4], i32, char);
let _23: [i64; 2];
let _24: [i32; 6];
let _25: (i64, isize, i128);
let _26: isize;
let _27: *const (*const i128, i8);
let _28: bool;
let _29: Adt46;
let _30: u32;
let _31: ([u8; 4], i32, char);
let _32: u16;
let _33: [char; 2];
let _34: isize;
let _35: *mut *mut usize;
let _36: *const i128;
let _37: f64;
let _38: u64;
let _39: (*const char,);
let _40: f32;
let _41: Adt46;
let _42: [i64; 2];
let _43: [u8; 4];
let _44: usize;
let _45: Adt44;
let _46: [i64; 2];
let _47: ();
let _48: ();
{
_3 = !_2;
_5 = _1;
_2 = 26141_u16 as i64;
_1.fld0 = [_4,_6];
_1 = Adt46 { fld0: _5.fld0 };
_7 = _4 as isize;
_6 = _4;
_3 = _4;
_5.fld0 = [_4,_4];
_6 = -_3;
_2 = _7 as i64;
_2 = _4;
_1.fld0 = [_6,_6];
_5.fld0 = [_4,_3];
_7 = 9223372036854775807_isize + 9223372036854775807_isize;
_4 = _2 * _2;
_8 = 120_i8 as u128;
_5 = _1;
Goto(bb1)
}
bb1 = {
_5 = Adt46 { fld0: _1.fld0 };
_4 = _6 | _3;
_5.fld0 = [_6,_6];
_6 = !_2;
_3 = 68_u8 as i64;
_5.fld0 = [_4,_4];
_6 = !_4;
_3 = _4;
_6 = !_2;
_3 = _4;
_3 = _7 as i64;
_5 = _1;
_8 = 300414109930650072821740576689953151753_u128;
_5 = Adt46 { fld0: _1.fld0 };
_1.fld0 = [_4,_2];
_4 = _6;
_11.fld0 = _1.fld0;
_5 = Adt46 { fld0: _11.fld0 };
_2 = _4;
_1.fld0 = _11.fld0;
_4 = _6;
_4 = -_2;
_4 = 1849246549321353700_u64 as i64;
_4 = _6;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
300414109930650072821740576689953151753 => bb8,
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
_7 = -9223372036854775807_isize;
_12 = _1.fld0;
_5 = Adt46 { fld0: _11.fld0 };
_5.fld0 = [_6,_6];
_11.fld0 = [_2,_2];
_1 = _11;
_12 = _5.fld0;
_5 = Adt46 { fld0: _11.fld0 };
_6 = _2;
_7 = (-9223372036854775808_isize);
_10 = 44515080018429693630080008290444156988_i128 as u64;
_6 = !_2;
_8 = !69216024067771348513954123232933099892_u128;
_11 = _1;
_3 = 163585862852176556382870213631956782220_i128 as i64;
_10 = 16454728706473049122_u64 * 16153692168316097449_u64;
_1.fld0 = [_6,_4];
_3 = _4;
_14 = '\u{ed6ad}';
_15 = (-62_i8) as isize;
_13 = _15 + _15;
_1.fld0 = [_4,_4];
_15 = _13;
_10 = !12546029476736978270_u64;
Call(_6 = core::intrinsics::transmute(_3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2 = _8 as i64;
_11.fld0 = [_4,_3];
_17 = 38034_u16;
_11 = Adt46 { fld0: _5.fld0 };
_1 = Adt46 { fld0: _12 };
_7 = 2499187900_u32 as isize;
_18.fld0 = [_6,_4];
_14 = '\u{3718c}';
_1 = Adt46 { fld0: _12 };
_19 = !331394716_u32;
_5 = Adt46 { fld0: _12 };
_11 = Adt46 { fld0: _18.fld0 };
_7 = _10 as isize;
_13 = _15 ^ _7;
_5.fld0 = _11.fld0;
_1 = Adt46 { fld0: _5.fld0 };
_11.fld0 = [_6,_3];
_6 = _4;
_2 = _6 + _4;
_6 = _8 as i64;
_3 = true as i64;
_11.fld0 = [_4,_4];
_5.fld0 = [_2,_2];
_11.fld0 = _5.fld0;
_1.fld0 = [_2,_2];
_4 = !_2;
match _17 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb8,
38034 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_1.fld0 = _11.fld0;
_3 = _4;
_14 = '\u{bfeee}';
_12 = [_3,_4];
_18.fld0 = [_2,_3];
_20 = _7 * _15;
_18.fld0 = [_4,_2];
_2 = _3;
_17 = (-148196012431893358980339234307139896785_i128) as u16;
_10 = !13206179115879155676_u64;
_8 = 184310830828183461036664676750106900253_u128;
_13 = _20;
_17 = 34514_u16 + 17260_u16;
_15 = _8 as isize;
_5.fld0 = [_2,_4];
_23 = _18.fld0;
_5.fld0 = [_3,_3];
_10 = 8329498917290291479_u64;
_3 = !_2;
_18 = _1;
Goto(bb12)
}
bb12 = {
_19 = 3376628151_u32 >> _2;
_6 = _4;
_4 = (-112_i8) as i64;
_8 = !55606176990022446932639594201860385902_u128;
_23 = [_3,_2];
_23 = _1.fld0;
_7 = _10 as isize;
_22.1 = (-816524377_i32) & (-160683398_i32);
_12 = _5.fld0;
_18.fld0 = [_6,_2];
_11 = _18;
_22.2 = _14;
_20 = _14 as isize;
_11 = Adt46 { fld0: _5.fld0 };
_24 = [_22.1,_22.1,_22.1,_22.1,_22.1,_22.1];
_21 = !_2;
_14 = _22.2;
_11.fld0 = _23;
_22.0 = [159_u8,111_u8,143_u8,21_u8];
_7 = _20 | _20;
_2 = _21;
_23 = [_21,_2];
Call(_12 = fn15(_2, _23, _18, _18.fld0, _5), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_12 = [_21,_3];
_4 = -_3;
_1.fld0 = [_3,_2];
_5.fld0 = _1.fld0;
_20 = _7 << _3;
_8 = !257492656875476332301291972423323773663_u128;
_22.0 = [182_u8,123_u8,109_u8,113_u8];
_18.fld0 = _1.fld0;
_22.2 = _14;
_1 = Adt46 { fld0: _12 };
_10 = 15089534419368471611_u64;
_1.fld0 = _18.fld0;
_26 = 1_usize as isize;
_22.1 = false as i32;
_1.fld0 = [_21,_6];
_22.2 = _14;
_21 = _4;
_12 = _1.fld0;
_25 = (_3, _20, (-36434340240510060166471386866680215398_i128));
Call(_19 = fn16(_5, _11.fld0, _25, _23, _11.fld0, _1.fld0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22.0 = [240_u8,47_u8,67_u8,120_u8];
_25.0 = _22.2 as i64;
_11 = Adt46 { fld0: _23 };
_22.2 = _14;
_1 = Adt46 { fld0: _23 };
_7 = !_25.1;
Goto(bb15)
}
bb15 = {
_20 = _25.1 >> _3;
_15 = _7;
_4 = 12_i8 as i64;
_31.1 = _22.1 + _22.1;
_31.1 = _22.1 >> _3;
_31.2 = _22.2;
_24 = [_31.1,_31.1,_31.1,_31.1,_31.1,_31.1];
_23 = [_3,_21];
match _25.2 {
0 => bb16,
1 => bb17,
303848026680428403296903220565087996058 => bb19,
_ => bb18
}
}
bb16 = {
Return()
}
bb17 = {
_12 = [_21,_3];
_4 = -_3;
_1.fld0 = [_3,_2];
_5.fld0 = _1.fld0;
_20 = _7 << _3;
_8 = !257492656875476332301291972423323773663_u128;
_22.0 = [182_u8,123_u8,109_u8,113_u8];
_18.fld0 = _1.fld0;
_22.2 = _14;
_1 = Adt46 { fld0: _12 };
_10 = 15089534419368471611_u64;
_1.fld0 = _18.fld0;
_26 = 1_usize as isize;
_22.1 = false as i32;
_1.fld0 = [_21,_6];
_22.2 = _14;
_21 = _4;
_12 = _1.fld0;
_25 = (_3, _20, (-36434340240510060166471386866680215398_i128));
Call(_19 = fn16(_5, _11.fld0, _25, _23, _11.fld0, _1.fld0), ReturnTo(bb14), UnwindUnreachable())
}
bb18 = {
_7 = -9223372036854775807_isize;
_12 = _1.fld0;
_5 = Adt46 { fld0: _11.fld0 };
_5.fld0 = [_6,_6];
_11.fld0 = [_2,_2];
_1 = _11;
_12 = _5.fld0;
_5 = Adt46 { fld0: _11.fld0 };
_6 = _2;
_7 = (-9223372036854775808_isize);
_10 = 44515080018429693630080008290444156988_i128 as u64;
_6 = !_2;
_8 = !69216024067771348513954123232933099892_u128;
_11 = _1;
_3 = 163585862852176556382870213631956782220_i128 as i64;
_10 = 16454728706473049122_u64 * 16153692168316097449_u64;
_1.fld0 = [_6,_4];
_3 = _4;
_14 = '\u{ed6ad}';
_15 = (-62_i8) as isize;
_13 = _15 + _15;
_1.fld0 = [_4,_4];
_15 = _13;
_10 = !12546029476736978270_u64;
Call(_6 = core::intrinsics::transmute(_3), ReturnTo(bb9), UnwindUnreachable())
}
bb19 = {
_31.0 = _22.0;
_33 = [_14,_22.2];
_5.fld0 = [_3,_21];
_18 = Adt46 { fld0: _23 };
_11.fld0 = _5.fld0;
_22.0 = _31.0;
_25.0 = !_3;
_28 = !false;
_29 = Adt46 { fld0: _11.fld0 };
_10 = !1466156360092903453_u64;
_22 = (_31.0, _31.1, _14);
_3 = _6 >> _2;
_21 = 5_usize as i64;
_30 = !_19;
match _25.2 {
0 => bb12,
1 => bb2,
2 => bb8,
3 => bb10,
4 => bb17,
5 => bb16,
303848026680428403296903220565087996058 => bb21,
_ => bb20
}
}
bb20 = {
_22.0 = [240_u8,47_u8,67_u8,120_u8];
_25.0 = _22.2 as i64;
_11 = Adt46 { fld0: _23 };
_22.2 = _14;
_1 = Adt46 { fld0: _23 };
_7 = !_25.1;
Goto(bb15)
}
bb21 = {
_1 = _11;
_32 = _19 as u16;
_7 = _19 as isize;
_18 = _29;
_12 = [_25.0,_25.0];
_25.2 = (-129415261215378815243665832393944972586_i128);
_11.fld0 = [_2,_2];
_18 = Adt46 { fld0: _23 };
_29.fld0 = _23;
_1 = _5;
_31.2 = _22.2;
_11.fld0 = _23;
_4 = !_3;
_1.fld0 = _11.fld0;
_12 = [_3,_4];
_6 = _4 - _2;
_18 = Adt46 { fld0: _11.fld0 };
_5 = Adt46 { fld0: _23 };
_29.fld0 = [_2,_25.0];
_1 = Adt46 { fld0: _18.fld0 };
_7 = _15 - _20;
_29 = Adt46 { fld0: _11.fld0 };
_18 = _29;
_28 = !false;
_22 = (_31.0, _31.1, _31.2);
_36 = core::ptr::addr_of!(_25.2);
match (*_36) {
210867105705559648219708775037823238870 => bb22,
_ => bb2
}
}
bb22 = {
_19 = !_30;
_9 = core::ptr::addr_of!(_39);
_9 = core::ptr::addr_of!(_39);
_3 = _6 >> _25.1;
_12 = _29.fld0;
_36 = core::ptr::addr_of!(_25.2);
_29 = _1;
_4 = _6;
_40 = _20 as f32;
_5.fld0 = [_25.0,_3];
match _25.2 {
0 => bb23,
1 => bb24,
2 => bb25,
3 => bb26,
210867105705559648219708775037823238870 => bb28,
_ => bb27
}
}
bb23 = {
_5 = Adt46 { fld0: _1.fld0 };
_4 = _6 | _3;
_5.fld0 = [_6,_6];
_6 = !_2;
_3 = 68_u8 as i64;
_5.fld0 = [_4,_4];
_6 = !_4;
_3 = _4;
_6 = !_2;
_3 = _4;
_3 = _7 as i64;
_5 = _1;
_8 = 300414109930650072821740576689953151753_u128;
_5 = Adt46 { fld0: _1.fld0 };
_1.fld0 = [_4,_2];
_4 = _6;
_11.fld0 = _1.fld0;
_5 = Adt46 { fld0: _11.fld0 };
_2 = _4;
_1.fld0 = _11.fld0;
_4 = _6;
_4 = -_2;
_4 = 1849246549321353700_u64 as i64;
_4 = _6;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
300414109930650072821740576689953151753 => bb8,
_ => bb7
}
}
bb24 = {
Return()
}
bb25 = {
_7 = -9223372036854775807_isize;
_12 = _1.fld0;
_5 = Adt46 { fld0: _11.fld0 };
_5.fld0 = [_6,_6];
_11.fld0 = [_2,_2];
_1 = _11;
_12 = _5.fld0;
_5 = Adt46 { fld0: _11.fld0 };
_6 = _2;
_7 = (-9223372036854775808_isize);
_10 = 44515080018429693630080008290444156988_i128 as u64;
_6 = !_2;
_8 = !69216024067771348513954123232933099892_u128;
_11 = _1;
_3 = 163585862852176556382870213631956782220_i128 as i64;
_10 = 16454728706473049122_u64 * 16153692168316097449_u64;
_1.fld0 = [_6,_4];
_3 = _4;
_14 = '\u{ed6ad}';
_15 = (-62_i8) as isize;
_13 = _15 + _15;
_1.fld0 = [_4,_4];
_15 = _13;
_10 = !12546029476736978270_u64;
Call(_6 = core::intrinsics::transmute(_3), ReturnTo(bb9), UnwindUnreachable())
}
bb26 = {
Return()
}
bb27 = {
Return()
}
bb28 = {
_25 = (_4, _15, (-114523999572066230913572939450618847158_i128));
_28 = false ^ true;
_39.0 = core::ptr::addr_of!(_22.2);
_7 = _40 as isize;
_11.fld0 = [_3,_3];
_39.0 = core::ptr::addr_of!(_31.2);
_10 = !3949070173593907296_u64;
_5 = Adt46 { fld0: _1.fld0 };
_42 = _18.fld0;
_10 = 13325479849032387946_u64;
_22 = (_31.0, _31.1, _31.2);
_36 = core::ptr::addr_of!(_25.2);
_31.2 = _22.2;
_29.fld0 = _23;
_12 = [_3,_25.0];
_12 = [_3,_6];
_23 = [_4,_6];
_25.0 = -_4;
_25 = (_4, _7, (-154984316700310765757271342499399365170_i128));
_22 = _31;
_31 = (_22.0, _22.1, _22.2);
Goto(bb29)
}
bb29 = {
_22 = (_31.0, _31.1, _31.2);
_43 = _22.0;
_38 = _10;
_38 = !_10;
_32 = _17;
RET = core::ptr::addr_of!(_37);
_5.fld0 = _11.fld0;
_1 = _18;
_24 = [_22.1,_31.1,_22.1,_22.1,_22.1,_22.1];
_12 = _1.fld0;
_8 = 249236326184768066086473630928911053860_u128;
_44 = _38 as usize;
_9 = core::ptr::addr_of!(_39);
_18 = Adt46 { fld0: _1.fld0 };
Goto(bb30)
}
bb30 = {
Call(_47 = dump_var(14_usize, 31_usize, Move(_31), 21_usize, Move(_21), 22_usize, Move(_22), 23_usize, Move(_23)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_47 = dump_var(14_usize, 8_usize, Move(_8), 25_usize, Move(_25), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_47 = dump_var(14_usize, 13_usize, Move(_13), 33_usize, Move(_33), 14_usize, Move(_14), 44_usize, Move(_44)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_47 = dump_var(14_usize, 10_usize, Move(_10), 30_usize, Move(_30), 48_usize, _48, 48_usize, _48), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i64,mut _2: [i64; 2],mut _3: Adt46,mut _4: [i64; 2],mut _5: Adt46) -> [i64; 2] {
mir! {
type RET = [i64; 2];
let _6: isize;
let _7: ([u8; 4], i32, char);
let _8: ();
let _9: ();
{
RET = [_1,_1];
RET = _2;
_6 = 9223372036854775807_isize << _1;
_3.fld0 = [_1,_1];
_7.2 = '\u{bcd01}';
_4 = RET;
_7.0 = [219_u8,229_u8,152_u8,89_u8];
_5.fld0 = [_1,_1];
_7.1 = 1460645603_i32;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(15_usize, 1_usize, Move(_1), 4_usize, Move(_4), 9_usize, _9, 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: Adt46,mut _2: [i64; 2],mut _3: (i64, isize, i128),mut _4: [i64; 2],mut _5: [i64; 2],mut _6: [i64; 2]) -> u32 {
mir! {
type RET = u32;
let _7: ([u8; 4], i32, char);
let _8: [i64; 2];
let _9: isize;
let _10: (i64, isize, i128);
let _11: &'static f32;
let _12: bool;
let _13: u128;
let _14: [u8; 4];
let _15: (i64, isize, i128);
let _16: f32;
let _17: bool;
let _18: Adt46;
let _19: ([u8; 4], i32, char);
let _20: i128;
let _21: f32;
let _22: [i64; 2];
let _23: Adt46;
let _24: ();
let _25: ();
{
_1.fld0 = [_3.0,_3.0];
_3.2 = 2228929384562282774984864011593025371_i128 + (-85150527261084914428277417400235043137_i128);
Call(_2 = fn17(_3, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = (6940061593954096538_i64, (-9223372036854775808_isize), (-66887136575396597050964059450880317762_i128));
_3 = (225658382636878447_i64, (-9223372036854775808_isize), 133569144049342045428316279128360329439_i128);
_3 = (4635372507680997333_i64, 45_isize, 94232486558667962967818904897491061647_i128);
_7.1 = (-1869768957_i32) << _3.0;
match _3.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
94232486558667962967818904897491061647 => bb7,
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
RET = !3052086330_u32;
_1.fld0 = [_3.0,_3.0];
_7.2 = '\u{37fc0}';
_6 = _5;
RET = 2033601620_u32 - 4031045389_u32;
_3.0 = -3201782220401160646_i64;
_8 = _2;
_7.1 = 8676_u16 as i32;
_2 = [_3.0,_3.0];
RET = !418064302_u32;
_1.fld0 = [_3.0,_3.0];
_7.0 = [39_u8,80_u8,42_u8,164_u8];
_6 = [_3.0,_3.0];
_3 = ((-6801687356464681780_i64), (-9223372036854775808_isize), 115184332894428262379751838639956176942_i128);
_3.1 = _7.2 as isize;
_10.0 = _3.0 & _3.0;
_10.2 = _3.2 & _3.2;
_10.1 = _3.1 - _3.1;
_10.0 = _3.0 << _10.2;
match _3.2 {
0 => bb4,
1 => bb2,
2 => bb3,
115184332894428262379751838639956176942 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_3 = _10;
_7.2 = '\u{95936}';
_3.2 = _10.2;
_3.1 = _10.1 * _10.1;
_8 = [_3.0,_3.0];
_8 = [_10.0,_10.0];
Goto(bb10)
}
bb10 = {
RET = _3.2 as u32;
_7.1 = _3.2 as i32;
_3.2 = _10.2 - _10.2;
_9 = _7.1 as isize;
_10.0 = _7.1 as i64;
_3.2 = _10.2 & _10.2;
_1 = Adt46 { fld0: _5 };
_7.1 = !(-1788261639_i32);
_9 = !_3.1;
_8 = _1.fld0;
_10 = (_3.0, _9, _3.2);
_5 = _4;
_10.0 = _3.0 * _3.0;
_6 = [_3.0,_10.0];
_10.0 = 13535881291114773424_u64 as i64;
_10.0 = _3.0 - _3.0;
_9 = _10.1;
_5 = [_3.0,_3.0];
_10.1 = _9 | _9;
_2 = _6;
RET = !3118042179_u32;
_14 = _7.0;
_8 = [_3.0,_10.0];
_15 = _10;
_15.2 = 90447884033871082695943698229645328212_u128 as i128;
Goto(bb11)
}
bb11 = {
_7.0 = [91_u8,21_u8,132_u8,222_u8];
_12 = true;
_10.2 = 9765842783321200670_usize as i128;
_18.fld0 = [_15.0,_10.0];
_13 = 259286784639016541509198766584220671951_u128;
_7 = (_14, (-1206514441_i32), '\u{36646}');
_3 = _15;
_4 = _1.fld0;
RET = 2754818401_u32;
_14 = [18_u8,51_u8,38_u8,235_u8];
Goto(bb12)
}
bb12 = {
_3.0 = 86_u8 as i64;
_12 = !true;
_11 = &_16;
RET = _7.2 as u32;
_7.2 = '\u{18d3d}';
_7.1 = _12 as i32;
_10 = (_15.0, _15.1, _3.2);
_3 = (_15.0, _10.1, _10.2);
_16 = 182_u8 as f32;
_3.1 = _15.1;
_17 = !_12;
match _13 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb10,
4 => bb7,
259286784639016541509198766584220671951 => bb13,
_ => bb6
}
}
bb13 = {
_15.2 = _16 as i128;
_6 = [_15.0,_3.0];
_3.1 = -_15.1;
_7.0 = [149_u8,228_u8,109_u8,158_u8];
_19.2 = _7.2;
_19.0 = _7.0;
_16 = _13 as f32;
_19 = _7;
_10.2 = !_15.2;
_3.2 = -_10.2;
_3.0 = _15.0 >> _15.0;
_3.1 = _15.1;
_15.2 = _3.2 | _3.2;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb7,
5 => bb6,
6 => bb14,
259286784639016541509198766584220671951 => bb16,
_ => bb15
}
}
bb14 = {
RET = _3.2 as u32;
_7.1 = _3.2 as i32;
_3.2 = _10.2 - _10.2;
_9 = _7.1 as isize;
_10.0 = _7.1 as i64;
_3.2 = _10.2 & _10.2;
_1 = Adt46 { fld0: _5 };
_7.1 = !(-1788261639_i32);
_9 = !_3.1;
_8 = _1.fld0;
_10 = (_3.0, _9, _3.2);
_5 = _4;
_10.0 = _3.0 * _3.0;
_6 = [_3.0,_10.0];
_10.0 = 13535881291114773424_u64 as i64;
_10.0 = _3.0 - _3.0;
_9 = _10.1;
_5 = [_3.0,_3.0];
_10.1 = _9 | _9;
_2 = _6;
RET = !3118042179_u32;
_14 = _7.0;
_8 = [_3.0,_10.0];
_15 = _10;
_15.2 = 90447884033871082695943698229645328212_u128 as i128;
Goto(bb11)
}
bb15 = {
_3 = _10;
_7.2 = '\u{95936}';
_3.2 = _10.2;
_3.1 = _10.1 * _10.1;
_8 = [_3.0,_3.0];
_8 = [_10.0,_10.0];
Goto(bb10)
}
bb16 = {
_7.1 = !_19.1;
_17 = _12 | _12;
_11 = &_21;
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(16_usize, 3_usize, Move(_3), 10_usize, Move(_10), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(16_usize, 7_usize, Move(_7), 17_usize, Move(_17), 5_usize, Move(_5), 25_usize, _25), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: (i64, isize, i128),mut _2: [i64; 2]) -> [i64; 2] {
mir! {
type RET = [i64; 2];
let _3: (i64, isize, i128);
let _4: (*const i128, i8);
let _5: u64;
let _6: isize;
let _7: ();
let _8: ();
{
_2 = [_1.0,_1.0];
_1.2 = 134990474380292478290105601830825442066_i128;
Call(_1.0 = core::intrinsics::transmute(_1.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_1.0,_1.0];
_1.2 = !74814750816060718886664559882249504203_i128;
_1.0 = 7131156093089522748_i64;
RET = _2;
RET = [_1.0,_1.0];
RET = _2;
_1.0 = (-8251691397783616610_i64) & 7442993126377218252_i64;
_1.2 = !16818552375524465390180337740898817537_i128;
_3 = (_1.0, _1.1, _1.2);
_1.2 = _3.2 * _3.2;
_1.2 = -_3.2;
_3 = (_1.0, _1.1, _1.2);
_3.2 = _1.2 - _1.2;
_4.0 = core::ptr::addr_of!(_3.2);
_3.1 = _1.0 as isize;
_1.2 = -_3.2;
_4.1 = !(-118_i8);
_6 = _1.1 - _1.1;
_3.2 = _1.2;
_1.0 = _3.0 ^ _3.0;
_3.1 = -_1.1;
_5 = !742040026501275638_u64;
Goto(bb2)
}
bb2 = {
Call(_7 = dump_var(17_usize, 1_usize, Move(_1), 3_usize, Move(_3), 8_usize, _8, 8_usize, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [i64; 2],mut _2: Adt46,mut _3: Adt46) -> i32 {
mir! {
type RET = i32;
let _4: (i32, usize, *mut [char; 2]);
let _5: u64;
let _6: f64;
let _7: isize;
let _8: i32;
let _9: *const i128;
let _10: u8;
let _11: [u8; 4];
let _12: u64;
let _13: bool;
let _14: u8;
let _15: u128;
let _16: (i64, isize, i128);
let _17: bool;
let _18: ([u8; 4], i32, char);
let _19: Adt41;
let _20: ();
let _21: ();
{
_1 = [(-2845114225682113968_i64),(-7892554179823561985_i64)];
_1 = _3.fld0;
_4.0 = (-935827862_i32) | 1851325724_i32;
RET = _4.0 - _4.0;
_2.fld0 = _3.fld0;
_4.1 = 3_usize * 14447007095114152964_usize;
_6 = 323282228182000801493147219558662215567_u128 as f64;
_2 = Adt46 { fld0: _1 };
_6 = (-48_i8) as f64;
_3.fld0 = [2389210187976398184_i64,7715452198211208869_i64];
_4.0 = RET;
_2.fld0 = _1;
_4.1 = 2333859481600725195_i64 as usize;
_3 = Adt46 { fld0: _1 };
_10 = 1398201036_u32 as u8;
RET = _4.0;
_4.1 = 8919808829076387858_usize;
_7 = !35_isize;
_5 = !12038808095137480583_u64;
_5 = !3681553328217587417_u64;
RET = _4.0 ^ _4.0;
match _4.1 {
0 => bb1,
1 => bb2,
2 => bb3,
8919808829076387858 => bb5,
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
_3 = _2;
_3.fld0 = _2.fld0;
_12 = !_5;
_6 = 5095_u16 as f64;
_1 = [(-6625035269523378868_i64),(-2175044641458256162_i64)];
_4.0 = RET;
_14 = !_10;
_7 = 96576452906887867659380857889199845088_i128 as isize;
_16 = ((-8686517120685887726_i64), _7, 93169171360560309880697310303255373135_i128);
RET = (-10_i8) as i32;
_13 = !true;
RET = _4.0;
match _16.2 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
93169171360560309880697310303255373135 => bb13,
_ => bb12
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
Return()
}
bb12 = {
Return()
}
bb13 = {
_15 = _5 as u128;
_18.0 = [_10,_14,_10,_14];
_17 = _13;
_10 = _14;
Goto(bb14)
}
bb14 = {
_16.1 = !_7;
_6 = RET as f64;
_10 = '\u{faee3}' as u8;
_11 = [_10,_10,_10,_14];
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(18_usize, 5_usize, Move(_5), 13_usize, Move(_13), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(18_usize, 17_usize, Move(_17), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i16,mut _2: isize,mut _3: *const f64,mut _4: i8,mut _5: f64,mut _6: i32,mut _7: usize,mut _8: i16,mut _9: isize,mut _10: Adt46,mut _11: [u8; 4],mut _12: Adt46,mut _13: i128,mut _14: [i64; 2]) -> i8 {
mir! {
type RET = i8;
let _15: *mut isize;
let _16: (*const i128, i8);
let _17: (*const f64, (*const i128, i8), f32, usize);
let _18: *mut f32;
let _19: [i32; 6];
let _20: ([u8; 4], i32, char);
let _21: *mut usize;
let _22: [char; 2];
let _23: f32;
let _24: f32;
let _25: &'static f32;
let _26: Adt49;
let _27: [i32; 6];
let _28: [i64; 2];
let _29: Adt49;
let _30: Adt49;
let _31: i128;
let _32: [i64; 2];
let _33: [i64; 2];
let _34: (i64, isize, i128);
let _35: Adt48;
let _36: ();
let _37: ();
{
RET = 1874621206_u32 as i8;
_6 = 1838521816_i32 - (-1670054827_i32);
_8 = -_1;
_4 = RET ^ RET;
_1 = 2703876747304733617_i64 as i16;
_15 = core::ptr::addr_of_mut!(_9);
_15 = core::ptr::addr_of_mut!((*_15));
_6 = 985744941_i32;
_4 = RET;
_8 = '\u{c1382}' as i16;
_10.fld0 = [(-1093871866823902572_i64),2322513049114163944_i64];
_10.fld0 = _14;
_10.fld0 = [(-8987344717348298199_i64),3175774916731167233_i64];
_7 = 7964687112841023171_usize * 4_usize;
_13 = -(-110347240773688502151688981892161987843_i128);
_9 = !_2;
_2 = !(*_15);
_19 = [_6,_6,_6,_6,_6,_6];
_2 = _9;
Goto(bb1)
}
bb1 = {
_12 = Adt46 { fld0: _10.fld0 };
_17.1.1 = 2853004030_u32 as i8;
_5 = 190324780386326238692736567027927801718_u128 as f64;
_11 = [226_u8,95_u8,182_u8,75_u8];
_3 = core::ptr::addr_of!(_5);
_10 = Adt46 { fld0: _14 };
_15 = core::ptr::addr_of_mut!((*_15));
_8 = 16112975858970734291_u64 as i16;
_17.1.0 = core::ptr::addr_of!(_13);
_10 = Adt46 { fld0: _14 };
_10 = _12;
_20.0 = _11;
_17.0 = core::ptr::addr_of!((*_3));
Call(_16.1 = core::intrinsics::transmute(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_20.2 = '\u{5362a}';
_18 = core::ptr::addr_of_mut!(_17.2);
match _6 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
985744941 => bb8,
_ => bb7
}
}
bb3 = {
_12 = Adt46 { fld0: _10.fld0 };
_17.1.1 = 2853004030_u32 as i8;
_5 = 190324780386326238692736567027927801718_u128 as f64;
_11 = [226_u8,95_u8,182_u8,75_u8];
_3 = core::ptr::addr_of!(_5);
_10 = Adt46 { fld0: _14 };
_15 = core::ptr::addr_of_mut!((*_15));
_8 = 16112975858970734291_u64 as i16;
_17.1.0 = core::ptr::addr_of!(_13);
_10 = Adt46 { fld0: _14 };
_10 = _12;
_20.0 = _11;
_17.0 = core::ptr::addr_of!((*_3));
Call(_16.1 = core::intrinsics::transmute(_4), ReturnTo(bb2), UnwindUnreachable())
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
_3 = _17.0;
_20.2 = '\u{781ad}';
_2 = 36201_u16 as isize;
_1 = -_8;
_6 = -(-365199239_i32);
_8 = _1;
_13 = 141613913708363995017033416093385505468_i128;
_14 = [6507066842599431842_i64,600005738192534361_i64];
_10.fld0 = [(-833332746272204917_i64),(-4988901915097914491_i64)];
_20.2 = '\u{1ef4a}';
_17.2 = _13 as f32;
_7 = !16808215112286423089_usize;
_22 = [_20.2,_20.2];
_16.0 = core::ptr::addr_of!(_13);
_6 = 736365036_i32;
_5 = 298526810646584215869355289479573591472_u128 as f64;
_12.fld0 = [(-192422687737591788_i64),(-3807319775900591709_i64)];
_18 = core::ptr::addr_of_mut!(_17.2);
Goto(bb9)
}
bb9 = {
_17.0 = core::ptr::addr_of!(_5);
_12.fld0 = [(-1135237326188142968_i64),2636946878806241587_i64];
_21 = core::ptr::addr_of_mut!(_7);
_2 = !(*_15);
_4 = 33414_u16 as i8;
RET = _17.1.1;
_12 = Adt46 { fld0: _10.fld0 };
_21 = core::ptr::addr_of_mut!((*_21));
_20 = (_11, _6, '\u{52bd6}');
_12.fld0 = [3776684778385352388_i64,(-8707270833038734434_i64)];
_18 = core::ptr::addr_of_mut!((*_18));
_5 = _1 as f64;
_20.2 = '\u{5b1f1}';
RET = _17.1.1;
Goto(bb10)
}
bb10 = {
Goto(bb11)
}
bb11 = {
_3 = _17.0;
_3 = core::ptr::addr_of!(_5);
Goto(bb12)
}
bb12 = {
_3 = _17.0;
_3 = core::ptr::addr_of!(_5);
_15 = core::ptr::addr_of_mut!((*_15));
_24 = _17.2;
_19 = [_20.1,_20.1,_20.1,_20.1,_6,_20.1];
_21 = core::ptr::addr_of_mut!((*_21));
_13 = 52747805599815651622673253309631873417_i128;
_16.0 = core::ptr::addr_of!(_13);
_10 = Adt46 { fld0: _14 };
_13 = !31727714366458021173778632680560730420_i128;
_7 = !17781468438411188100_usize;
_24 = -(*_18);
_9 = !_2;
_12.fld0 = _14;
Goto(bb13)
}
bb13 = {
_17.3 = (*_21);
_18 = core::ptr::addr_of_mut!(_17.2);
_6 = -_20.1;
_3 = core::ptr::addr_of!((*_3));
_7 = !_17.3;
_4 = _16.1 << _13;
_9 = _2 << _8;
_1 = 35_u8 as i16;
_20 = (_11, _6, '\u{8a5f8}');
_25 = &_23;
_17.3 = !_7;
_12.fld0 = [277316153159096910_i64,8150993498289293569_i64];
_17 = (_3, _16, _24, (*_21));
_29 = Adt49::Variant0 { fld0: _17.0,fld1: _22 };
_23 = _13 as f32;
_16 = (_17.1.0, _4);
_30 = Move(_29);
Goto(bb14)
}
bb14 = {
_4 = !RET;
_21 = core::ptr::addr_of_mut!((*_21));
_12 = Adt46 { fld0: _10.fld0 };
_25 = &_17.2;
(*_18) = _23 + _24;
_17.3 = RET as usize;
SetDiscriminant(_30, 0);
_25 = &(*_18);
_4 = 330579185619670161776103698419096371453_u128 as i8;
_30 = Adt49::Variant0 { fld0: _17.0,fld1: _22 };
_27 = _19;
_7 = _17.3;
_17.1.0 = _16.0;
_8 = -_1;
_20.1 = _6 << _9;
_10.fld0 = [8732137580049328355_i64,4455727724227670623_i64];
_31 = _2 as i128;
_34.1 = -_2;
_23 = _17.2 - _17.2;
place!(Field::<*const f64>(Variant(_30, 0), 0)) = _3;
_35.fld0 = false | false;
_35.fld6 = _17.1.0;
_35.fld5 = [_20.2,_20.2];
place!(Field::<*const f64>(Variant(_30, 0), 0)) = _17.0;
_30 = Adt49::Variant0 { fld0: _17.0,fld1: _35.fld5 };
RET = -_16.1;
_16.1 = _17.1.1;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(19_usize, 19_usize, Move(_19), 14_usize, Move(_14), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(19_usize, 31_usize, Move(_31), 13_usize, Move(_13), 27_usize, Move(_27), 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{17d9f}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(3_i8), std::hint::black_box((-2970_i16)), std::hint::black_box((-56399909_i32)), std::hint::black_box((-6905185767555406820_i64)), std::hint::black_box((-30167336284884856668204160346251307440_i128)), std::hint::black_box(6_usize), std::hint::black_box(213_u8), std::hint::black_box(12984_u16), std::hint::black_box(1577140680_u32), std::hint::black_box(192375825972710764488781698742469813933_u128));
                
            }
#[derive(Debug)]
pub enum Adt41 {
Variant0{
fld0: (i32, usize, *mut [char; 2]),
fld1: *const i128,
fld2: *const f64,

},
Variant1{
fld0: *const (*const i128, i8),
fld1: ([u8; 4], i32, char),

},
Variant2{
fld0: *mut f32,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt42 {
Variant0{
fld0: *const (*const i128, i8),
fld1: *mut *mut usize,

},
Variant1{
fld0: *mut [char; 2],

},
Variant2{
fld0: *mut u32,
fld1: char,
fld2: (i64, isize, i128),
fld3: *const (*const i128, i8),
fld4: [u8; 4],
fld5: (*const f64, (*const i128, i8), f32, usize),
fld6: [i32; 6],

},
Variant3{
fld0: bool,
fld1: *const (*const char,),
fld2: u8,
fld3: usize,
fld4: i16,
fld5: (i64, isize, i128),
fld6: i64,

}}
#[derive(Debug)]
pub enum Adt43 {
Variant0{
fld0: *mut (*const char,),
fld1: *const i128,
fld2: *mut usize,
fld3: i32,

},
Variant1{
fld0: u128,
fld1: *mut *mut usize,
fld2: (f32, usize, *const f64, *const char),
fld3: u64,

}}
#[derive(Debug)]
pub enum Adt44 {
Variant0{
fld0: isize,
fld1: i32,

},
Variant1{
fld0: Adt43,
fld1: u8,
fld2: f32,
fld3: *mut f32,
fld4: *mut usize,
fld5: [i64; 2],

}}
#[derive(Debug)]
pub enum Adt45 {
Variant0{
fld0: bool,
fld1: [char; 2],
fld2: *const (*const i128, i8),
fld3: u128,
fld4: i16,
fld5: Adt43,
fld6: (*const i128, i8),

},
Variant1{
fld0: bool,
fld1: Adt44,
fld2: isize,
fld3: (i32, usize, *mut [char; 2]),

},
Variant2{
fld0: (*const char,),
fld1: char,
fld2: *const (*const char,),
fld3: *mut [char; 2],
fld4: [u8; 4],
fld5: usize,
fld6: u16,
fld7: (*const i128, i8),

},
Variant3{
fld0: u128,
fld1: char,
fld2: [i64; 2],

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt46 {
fld0: [i64; 2],
}
#[derive(Debug)]
pub enum Adt47 {
Variant0{
fld0: (i64, isize, i128),
fld1: char,
fld2: i32,
fld3: i16,

},
Variant1{
fld0: bool,
fld1: [i32; 6],
fld2: *mut f32,
fld3: i128,
fld4: u16,

}}
#[derive(Debug)]
pub struct Adt48 {
fld0: bool,
fld1: (*const char,),
fld2: (i64, isize, i128),
fld3: Adt42,
fld4: [u8; 4],
fld5: [char; 2],
fld6: *const i128,
}
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: *const f64,
fld1: [char; 2],

},
Variant1{
fld0: u64,
fld1: *mut [char; 2],
fld2: *mut isize,
fld3: Adt43,
fld4: *mut u32,
fld5: *mut *mut usize,

}}
#[derive(Debug)]
pub struct Adt50 {
fld0: u128,
fld1: Adt49,
fld2: Adt48,
fld3: *mut [char; 2],
fld4: Adt46,
}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: usize,
fld1: *const f64,
fld2: isize,
fld3: ([u8; 4], i32, char),
fld4: (i32, usize, *mut [char; 2]),

},
Variant1{
fld0: Adt49,
fld1: ([u8; 4], i32, char),
fld2: (f32, usize, *const f64, *const char),
fld3: *mut (*const char,),
fld4: (i32, usize, *mut [char; 2]),
fld5: i32,

},
Variant2{
fld0: (*const f64, (*const i128, i8), f32, usize),
fld1: Adt43,
fld2: *const char,
fld3: *const i128,
fld4: (i32, usize, *mut [char; 2]),
fld5: f32,
fld6: [i32; 6],
fld7: *const (*const i128, i8),

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: Adt45,
fld1: u64,
fld2: i64,
fld3: Adt48,

},
Variant1{
fld0: (i64, isize, i128),
fld1: Adt42,
fld2: u32,
fld3: *mut *mut usize,

}}
#[derive(Debug)]
pub struct Adt53 {
fld0: ([u8; 4], i32, char),
fld1: u128,
fld2: [i32; 6],
fld3: Adt51,
fld4: *const (*const char,),
}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: (i64, isize, i128),
fld1: char,
fld2: usize,
fld3: u128,
fld4: Adt50,

},
Variant1{
fld0: f64,

},
Variant2{
fld0: Adt42,
fld1: u64,
fld2: isize,

}}
#[derive(Debug)]
pub enum Adt55 {
Variant0{
fld0: Adt48,
fld1: Adt50,
fld2: Adt47,
fld3: i8,
fld4: Adt51,
fld5: *mut f32,

},
Variant1{
fld0: Adt44,
fld1: *const (*const char,),
fld2: *mut [char; 2],
fld3: (i64, isize, i128),
fld4: [i64; 2],

},
Variant2{
fld0: i16,
fld1: [i64; 2],
fld2: u64,
fld3: *mut usize,

}}
#[derive(Debug)]
pub enum Adt56 {
Variant0{
fld0: *mut isize,
fld1: f64,
fld2: (i64, isize, i128),
fld3: u8,
fld4: usize,

},
Variant1{
fld0: i32,
fld1: *mut [char; 2],

},
Variant2{
fld0: Adt42,
fld1: (i64, isize, i128),
fld2: isize,
fld3: usize,
fld4: u8,
fld5: Adt46,
fld6: Adt43,
fld7: i128,

}}
#[derive(Debug)]
pub struct Adt57 {
fld0: (*const f64, (*const i128, i8), f32, usize),
fld1: [u8; 4],
fld2: Adt41,
fld3: Adt44,
}

