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
pub fn fn0(mut _1: i32,mut _2: char,mut _3: u16,mut _4: i16) -> Adt50 {
mir! {
type RET = Adt50;
let _5: bool;
let _6: bool;
let _7: isize;
let _8: [u32; 7];
let _9: Adt41;
let _10: f32;
let _11: i32;
let _12: char;
let _13: isize;
let _14: ([i128; 5], i16);
let _15: *const &'static isize;
let _16: Adt43;
let _17: Adt41;
let _18: u32;
let _19: isize;
let _20: f32;
let _21: ([i8; 2], [i128; 5]);
let _22: [i128; 5];
let _23: char;
let _24: Adt51;
let _25: bool;
let _26: Adt44;
let _27: usize;
let _28: (([i128; 5], i16), ([u16; 5],));
let _29: u16;
let _30: isize;
let _31: usize;
let _32: ();
let _33: ();
{
_4 = 9223372036854775807_isize as i16;
_4 = -9892_i16;
_2 = '\u{44a9b}';
_1 = (-34705693_i32);
_2 = '\u{102705}';
_5 = !true;
_4 = 16960_i16;
_3 = 55843_u16;
_6 = _1 != _1;
_2 = '\u{4a571}';
_2 = '\u{a49f9}';
_3 = !16412_u16;
_5 = !_6;
_7 = 17974378689866730946_usize as isize;
_6 = _5 ^ _5;
Goto(bb1)
}
bb1 = {
_5 = _6;
_3 = !17575_u16;
_9.fld2 = 222631876495275621686912584092131260179_u128 as f64;
_9.fld4.0 = _2;
_2 = _9.fld4.0;
_9.fld4.2 = _4;
_9.fld4.2 = 59083350046544130208712851797516503650_i128 as i16;
_9.fld4.1 = _9.fld4.0;
_5 = !_6;
_9.fld4.2 = -_4;
_9.fld2 = 127640405910583462433938952083463032155_u128 as f64;
_9.fld2 = (-79_i8) as f64;
_9.fld4.3 = !_3;
_9.fld2 = 243339050584399337882157328358210718883_u128 as f64;
_9.fld4.1 = _9.fld4.0;
_9.fld3.0 = [_9.fld4.3,_9.fld4.3,_3,_9.fld4.3,_9.fld4.3];
_5 = _6 ^ _6;
_9.fld4.2 = -_4;
_9.fld3.0 = [_3,_3,_9.fld4.3,_9.fld4.3,_9.fld4.3];
_4 = 5790199007677171164_u64 as i16;
_10 = 8354012331602934640_i64 as f32;
_9.fld3.0 = [_3,_3,_9.fld4.3,_3,_3];
_6 = _5;
_4 = _9.fld4.2;
_5 = _6 <= _6;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431733505763 => bb7,
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
_9.fld4.2 = _4;
_9.fld2 = 110361806622615164168118653500739837006_i128 as f64;
_2 = _9.fld4.1;
_11 = _1 >> _3;
_1 = _11 - _11;
_9.fld0 = _5;
_8 = [283217448_u32,2539630483_u32,3573446560_u32,560853073_u32,4134096899_u32,2209910555_u32,1401388460_u32];
_12 = _2;
_9.fld2 = 6691870624916594397_i64 as f64;
_9.fld0 = _6;
_9.fld4.4 = _9.fld2;
Goto(bb8)
}
bb8 = {
_6 = _9.fld0 != _5;
_10 = _3 as f32;
_9.fld1 = _2;
_9.fld0 = !_6;
_11 = _1;
_8 = [3683301680_u32,4106129815_u32,2319676455_u32,2380170456_u32,133119441_u32,1161123211_u32,1887283763_u32];
_9.fld1 = _12;
Goto(bb9)
}
bb9 = {
_13 = _7 << _11;
_17.fld4.2 = -_9.fld4.2;
_12 = _2;
_9.fld4 = (_9.fld1, _12, _17.fld4.2, _3, _9.fld2);
_7 = 17639259553472466179_u64 as isize;
_17.fld4.0 = _9.fld1;
_8 = [959392229_u32,3934258739_u32,303705558_u32,2557541727_u32,3734430752_u32,2565365538_u32,2399054631_u32];
_17.fld0 = !_6;
_17.fld3.0 = _9.fld3.0;
_14.1 = !_9.fld4.2;
_17.fld4 = (_12, _2, _9.fld4.2, _9.fld4.3, _9.fld4.4);
_10 = (-4116437109181348195_i64) as f32;
_17.fld2 = _9.fld4.4 - _9.fld2;
_9.fld1 = _12;
_9.fld0 = _5;
_9.fld4.0 = _2;
_9 = Adt41 { fld0: _6,fld1: _12,fld2: _17.fld2,fld3: _17.fld3,fld4: _17.fld4 };
_9 = Adt41 { fld0: _6,fld1: _2,fld2: _17.fld2,fld3: _17.fld3,fld4: _17.fld4 };
_17.fld4.2 = !_14.1;
_17.fld4.2 = (-77_i8) as i16;
_14.0 = [109362665954294039619033486687767059024_i128,(-160890654436771980304839390139164923294_i128),25901930226879481565899149541934539226_i128,28290155317060098708601402052107464102_i128,(-74433322333175627806634725476037421501_i128)];
_18 = 1907789784_u32 | 2810755047_u32;
_14.0 = [51881685122339022273540761910759323824_i128,(-22749784883305783520047478240513154772_i128),(-133271308674408751849195291643798690272_i128),(-87258825101480109468458015908553673618_i128),(-11699315237088961345337908207063476418_i128)];
_9.fld4 = (_2, _17.fld4.1, _4, _3, _17.fld2);
_5 = _9.fld0;
Call(_17.fld3.0 = core::intrinsics::transmute(_9.fld3.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = !_9.fld4.3;
_9.fld4.1 = _2;
_17 = Adt41 { fld0: _5,fld1: _2,fld2: _9.fld4.4,fld3: _9.fld3,fld4: _9.fld4 };
_9.fld4 = (_12, _17.fld1, _14.1, _3, _17.fld4.4);
_12 = _17.fld4.0;
_6 = _17.fld0;
_9.fld2 = _17.fld4.4 - _17.fld4.4;
_9.fld4.1 = _17.fld4.1;
_17.fld4.3 = !_3;
_9.fld1 = _9.fld4.1;
_8 = [_18,_18,_18,_18,_18,_18,_18];
_9.fld4.4 = _9.fld2;
_11 = -_1;
_14.1 = -_9.fld4.2;
_11 = _1 ^ _1;
_17.fld4.3 = _3;
Call(_17.fld4.1 = fn1(_5, _11, _9.fld4.4, Move(_9), _6, _5, _5, _5, _6, _5, _11, _18, _5, _14.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9.fld1 = _17.fld4.1;
_9.fld4.0 = _12;
_1 = _11;
_9.fld2 = _17.fld2 * _17.fld4.4;
_9.fld3.0 = [_3,_17.fld4.3,_17.fld4.3,_3,_3];
_19 = _13;
_9 = Adt41 { fld0: _17.fld0,fld1: _2,fld2: _17.fld4.4,fld3: _17.fld3,fld4: _17.fld4 };
_20 = -_10;
_9.fld4.2 = (-8419783003283398622_i64) as i16;
_21.0 = [(-58_i8),(-44_i8)];
_17.fld1 = _17.fld4.0;
_17.fld1 = _9.fld4.1;
Goto(bb12)
}
bb12 = {
_17.fld2 = -_17.fld4.4;
_17 = Adt41 { fld0: _5,fld1: _9.fld4.0,fld2: _9.fld2,fld3: _9.fld3,fld4: _9.fld4 };
RET = Adt50::Variant1 { fld0: _19 };
_9.fld4.2 = !_14.1;
_9.fld4.1 = _17.fld4.0;
_14.1 = _4;
_11 = _1 * _1;
SetDiscriminant(RET, 2);
_19 = !_13;
place!(Field::<u64>(Variant(RET, 2), 0)) = _20 as u64;
_22 = [30720168926570880478667794531307271404_i128,(-111691312605908712057259218572230229242_i128),(-19229725777951445580315323505833742362_i128),23908117523318796567402904294620175688_i128,95834427798808128926887110661254687664_i128];
_5 = _17.fld0 != _17.fld0;
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(RET, 2), 2)) = core::ptr::addr_of!(_9.fld4);
_12 = _2;
Goto(bb13)
}
bb13 = {
_9.fld4.3 = _20 as u16;
place!(Field::<(usize, [i8; 2], bool)>(Variant(RET, 2), 1)).1 = [76_i8,(-101_i8)];
_19 = _17.fld2 as isize;
_17.fld2 = -_9.fld4.4;
place!(Field::<(usize, [i8; 2], bool)>(Variant(RET, 2), 1)).0 = !5111090974817237133_usize;
place!(Field::<(usize, [i8; 2], bool)>(Variant(RET, 2), 1)) = (18039848435729418670_usize, _21.0, _5);
_7 = _13;
_14 = (_22, _17.fld4.2);
_20 = _19 as f32;
SetDiscriminant(RET, 2);
_14 = (_22, _4);
_14.1 = -_9.fld4.2;
_14.1 = _9.fld4.2;
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(RET, 2), 2)) = core::ptr::addr_of!(_17.fld4);
_25 = _17.fld2 <= _17.fld4.4;
place!(Field::<(usize, [i8; 2], bool)>(Variant(RET, 2), 1)).0 = _9.fld4.3 as usize;
Goto(bb14)
}
bb14 = {
_4 = !_9.fld4.2;
_29 = (-91739463494047136376733052099186628220_i128) as u16;
_14.1 = _4;
_25 = _17.fld1 <= _9.fld4.1;
_6 = !_5;
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(RET, 2), 2)) = core::ptr::addr_of!(_9.fld4);
place!(Field::<u64>(Variant(RET, 2), 0)) = !5675050809627787885_u64;
_10 = -_20;
_6 = !_5;
RET = Adt50::Variant1 { fld0: _7 };
_14 = (_22, _17.fld4.2);
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(0_usize, 29_usize, Move(_29), 8_usize, Move(_8), 25_usize, Move(_25), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(0_usize, 22_usize, Move(_22), 3_usize, Move(_3), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: bool,mut _2: i32,mut _3: f64,mut _4: Adt41,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: i32,mut _12: u32,mut _13: bool,mut _14: [i128; 5]) -> char {
mir! {
type RET = char;
let _15: [u16; 5];
let _16: i8;
let _17: char;
let _18: ([u16; 5],);
let _19: (char, char, i16, u16, f64);
let _20: usize;
let _21: i16;
let _22: Adt55;
let _23: Adt43;
let _24: ([u16; 5],);
let _25: &'static isize;
let _26: bool;
let _27: i32;
let _28: (([i128; 5], i16), ([u16; 5],));
let _29: [u128; 3];
let _30: Adt42;
let _31: char;
let _32: Adt46;
let _33: [isize; 4];
let _34: char;
let _35: *mut usize;
let _36: (usize, i8, i128, i128);
let _37: (char, char, i16, u16, f64);
let _38: f64;
let _39: i64;
let _40: Adt46;
let _41: char;
let _42: u16;
let _43: (usize, i8, i128, i128);
let _44: ([u16; 5],);
let _45: char;
let _46: isize;
let _47: isize;
let _48: bool;
let _49: [u128; 1];
let _50: &'static isize;
let _51: (char, char, i16, u16, f64);
let _52: [u128; 1];
let _53: i128;
let _54: f32;
let _55: (([i128; 5], i16), ([u16; 5],));
let _56: ([i8; 2], [i128; 5]);
let _57: f32;
let _58: isize;
let _59: isize;
let _60: Adt54;
let _61: ();
let _62: ();
{
_4.fld4.4 = _3 - _4.fld2;
_5 = !_4.fld0;
_4.fld1 = _4.fld4.1;
_7 = _10;
_8 = _1;
_14 = [(-130233526026014793404830362538416936512_i128),97125507807730165767788214605213311925_i128,(-95902003677682005440419042268030696146_i128),(-42326323501491735638980670997461067441_i128),(-80431970716342925443465932338100410394_i128)];
_13 = _6 & _7;
_2 = _11 & _11;
_6 = !_13;
_4.fld4.1 = _4.fld4.0;
_3 = _4.fld4.4;
_16 = (-33_i8) ^ 39_i8;
_5 = !_4.fld0;
RET = _4.fld4.0;
_19.2 = -_4.fld4.2;
Call(_4.fld4.0 = fn2(_13, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _4.fld4.0;
_4.fld4.3 = (-85944152900151700054734144734168104766_i128) as u16;
_18 = (_4.fld3.0,);
_20 = 33643878707955059543280602521850240054_u128 as usize;
_19.1 = RET;
_21 = _19.2;
_4.fld3 = (_18.0,);
_10 = _5;
_5 = _4.fld0 & _4.fld0;
RET = _19.1;
_15 = [_4.fld4.3,_4.fld4.3,_4.fld4.3,_4.fld4.3,_4.fld4.3];
_17 = _4.fld4.1;
_4.fld4.1 = RET;
_6 = !_1;
_22.fld1 = Adt45 { fld0: _4.fld4.3 };
_24 = (_4.fld3.0,);
_1 = _5 & _4.fld0;
_18.0 = [_4.fld4.3,_4.fld4.3,_22.fld1.fld0,_4.fld4.3,_22.fld1.fld0];
_15 = _4.fld3.0;
_17 = RET;
_9 = _13;
_20 = 56_isize as usize;
_4.fld3.0 = [_22.fld1.fld0,_22.fld1.fld0,_22.fld1.fld0,_4.fld4.3,_22.fld1.fld0];
Goto(bb2)
}
bb2 = {
_4.fld3 = _18;
_22.fld4 = !(-51276700547223697846149759319347264104_i128);
_19.2 = -_4.fld4.2;
_5 = _6 >= _1;
_19.1 = _4.fld4.0;
_19 = (_17, _4.fld1, _4.fld4.2, _22.fld1.fld0, _3);
_20 = 7_usize + 16278687426148893771_usize;
_4.fld4.4 = _19.4;
_4.fld0 = _13;
_22.fld4 = _19.4 as i128;
_4.fld4.0 = _17;
_4.fld4.0 = _4.fld4.1;
_4.fld4 = (_4.fld1, _19.0, _19.2, _19.3, _4.fld2);
_22.fld5 = _2;
RET = _17;
_19.0 = _4.fld4.0;
_19 = (RET, _4.fld4.0, _4.fld4.2, _4.fld4.3, _3);
_4.fld4.4 = -_3;
Goto(bb3)
}
bb3 = {
_14 = [_22.fld4,_22.fld4,_22.fld4,_22.fld4,_22.fld4];
_4.fld2 = _19.4 - _4.fld4.4;
Goto(bb4)
}
bb4 = {
_4.fld3.0 = _18.0;
_27 = 10413390076423650551_u64 as i32;
_26 = !_5;
_28.0 = (_14, _4.fld4.2);
_4.fld2 = _21 as f64;
_7 = !_9;
_22.fld1.fld0 = !_4.fld4.3;
_18.0 = [_19.3,_19.3,_22.fld1.fld0,_22.fld1.fld0,_19.3];
_34 = _4.fld1;
_12 = !3992125977_u32;
_28.1 = (_15,);
_26 = _3 == _3;
_21 = !_28.0.1;
_19.3 = _4.fld4.3 << _2;
_4.fld2 = _19.4 + _4.fld4.4;
_36.0 = !_20;
_36.2 = _22.fld4 & _22.fld4;
_12 = _28.0.1 as u32;
_37.2 = _28.0.1;
Goto(bb5)
}
bb5 = {
_22.fld1.fld0 = !_19.3;
_4 = Adt41 { fld0: _13,fld1: _34,fld2: _3,fld3: _24,fld4: _19 };
_36.3 = !_22.fld4;
_38 = -_3;
_28.1 = _18;
_12 = 3058705107_u32 >> _19.3;
Goto(bb6)
}
bb6 = {
_31 = _17;
_4.fld4.2 = _22.fld1.fld0 as i16;
_7 = !_9;
_37.4 = _4.fld2;
_22.fld1 = Adt45 { fld0: _4.fld4.3 };
_36.1 = _16 + _16;
Goto(bb7)
}
bb7 = {
RET = _31;
_38 = -_4.fld2;
_8 = _38 < _37.4;
_37.1 = _19.1;
_28.0.0 = [_22.fld4,_36.2,_36.3,_36.2,_36.2];
_18 = _28.1;
_31 = _37.1;
_27 = _11 ^ _2;
_10 = _13;
_28.1 = (_15,);
_15 = [_19.3,_22.fld1.fld0,_4.fld4.3,_22.fld1.fld0,_19.3];
_8 = _7;
_4.fld3.0 = [_22.fld1.fld0,_4.fld4.3,_22.fld1.fld0,_19.3,_22.fld1.fld0];
_12 = (-5318498208808522932_i64) as u32;
_37 = (RET, _4.fld4.1, _4.fld4.2, _22.fld1.fld0, _4.fld2);
_27 = !_22.fld5;
_19 = (_37.0, _37.1, _37.2, _37.3, _4.fld2);
_35 = core::ptr::addr_of_mut!(_36.0);
_17 = _4.fld4.1;
_28.0 = (_14, _19.2);
Call(_19 = fn8(_13, _1, _4.fld4.2, _4.fld0, _36, _18, _8, _37.3, _22.fld1, _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_22.fld2 = Adt54::Variant1 { fld0: _36,fld1: _37.0 };
_36.1 = (-118_isize) as i8;
_39 = _12 as i64;
_33 = [125_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_14 = [Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).3,_36.2,Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).2,Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).3,_22.fld4];
RET = _4.fld4.0;
_44.0 = [_4.fld4.3,_22.fld1.fld0,_19.3,_19.3,_37.3];
_3 = 202_u8 as f64;
_33 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = _31;
_4.fld4 = (_4.fld1, _31, _19.2, _19.3, _38);
_22.fld5 = _11 >> _2;
_28.0 = (_14, _19.2);
_28.0.0 = [Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).2,_36.3,Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).2,_22.fld4,Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).3];
_28.0.0 = [Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).2,_22.fld4,_36.2,Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).3,_36.3];
Call(_22.fld4 = core::intrinsics::transmute(_36.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_36.1 = _11 as i8;
_25 = &_46;
_24.0 = _44.0;
_22.fld4 = _36.1 as i128;
SetDiscriminant(_22.fld2, 1);
_32 = Adt46::Variant2 { fld0: _28 };
_9 = _10;
_22.fld4 = _22.fld5 as i128;
_4.fld3 = (_44.0,);
_37.2 = _19.2 & _19.2;
_14 = [_22.fld4,_22.fld4,_22.fld4,_22.fld4,_36.2];
_11 = _22.fld5 ^ _2;
_4.fld2 = _4.fld4.4 - _37.4;
_24.0 = [_4.fld4.3,_37.3,_4.fld4.3,_19.3,_4.fld4.3];
_37.0 = _4.fld1;
_36.2 = _36.3;
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_32, 2), 0)).0.0 = _14;
_22.fld1 = Adt45 { fld0: _4.fld4.3 };
_22.fld4 = !_36.2;
Goto(bb10)
}
bb10 = {
_8 = _1;
_41 = RET;
_36 = (_20, _16, _22.fld4, _22.fld4);
_43.1 = !_36.1;
place!(Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0)).2 = _22.fld4;
_4.fld1 = _17;
_45 = _19.0;
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_32, 2), 0)).1.0 = [_19.3,_22.fld1.fld0,_22.fld1.fld0,_22.fld1.fld0,_22.fld1.fld0];
Goto(bb11)
}
bb11 = {
_22.fld0 = Adt52::Variant1 { fld0: _4.fld3.0,fld1: _4.fld4,fld2: (-9223372036854775808_isize),fld3: _35,fld4: Move(_32) };
_37.4 = _38 - _4.fld2;
place!(Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0)).0 = (*_35);
_37 = (Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).1, _34, _19.2, Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).3, _4.fld2);
_39 = (-703061695010255786_i64) | 5687993740855904344_i64;
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(place!(Field::<Adt46>(Variant(_22.fld0, 1), 4)), 2), 0)).1 = _4.fld3;
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1)).4 = _4.fld4.4 - _38;
_22.fld1.fld0 = _36.1 as u16;
_10 = _4.fld0;
_47 = (-20_isize) + 14_isize;
_51.3 = !_37.3;
_18.0 = [_51.3,_4.fld4.3,Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).3,_51.3,_51.3];
RET = Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).1;
SetDiscriminant(Field::<Adt46>(Variant(_22.fld0, 1), 4), 3);
place!(Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0)).1 = _36.1;
_51.3 = _19.3;
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(place!(Field::<Adt46>(Variant(_22.fld0, 1), 4)), 3), 3)) = core::ptr::addr_of!(_19);
_51 = _19;
Goto(bb12)
}
bb12 = {
_17 = RET;
place!(Field::<i64>(Variant(place!(Field::<Adt46>(Variant(_22.fld0, 1), 4)), 3), 0)) = _39 & _39;
_44 = _24;
_28.1.0 = [_37.3,_19.3,_37.3,_19.3,Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).3];
_4.fld4.4 = -Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).4;
place!(Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0)).0 = (*_35) & (*_35);
_18.0 = [_19.3,_4.fld4.3,_4.fld4.3,_37.3,_19.3];
_37.3 = _4.fld4.3 * _4.fld4.3;
_44 = (_4.fld3.0,);
Goto(bb13)
}
bb13 = {
place!(Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0)).3 = _36.2;
_49 = [254976515111622418020197803776245599016_u128];
_7 = _13;
_4.fld4 = (_51.0, _51.0, _19.2, _37.3, _37.4);
_46 = _47;
_37.0 = _4.fld4.1;
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1)).1 = _37.0;
_4.fld4.0 = _37.0;
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1)).4 = -_37.4;
_16 = -_36.1;
_32 = Adt46::Variant0 { fld0: _1,fld1: _35 };
_39 = Field::<i64>(Variant(Field::<Adt46>(Variant(_22.fld0, 1), 4), 3), 0) - Field::<i64>(Variant(Field::<Adt46>(Variant(_22.fld0, 1), 4), 3), 0);
Call(_37.1 = fn9(Move(_32), _28, _8, Move(_4), _19, _13, _44, _51.2, Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1), _19, Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).1, Field::<[u16; 5]>(Variant(_22.fld0, 1), 0), Field::<*const (char, char, i16, u16, f64)>(Variant(Field::<Adt46>(Variant(_22.fld0, 1), 4), 3), 3), _18, _28.0.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_55.1.0 = _15;
_4.fld4.0 = _51.1;
_43.2 = _36.2 + Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).2;
_14 = _28.0.0;
_4.fld4.2 = Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0).3 as i16;
_4.fld3.0 = [_51.3,Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).3,_51.3,_37.3,_51.3];
place!(Field::<*mut usize>(Variant(_22.fld0, 1), 3)) = core::ptr::addr_of_mut!(place!(Field::<(usize, [i8; 2], bool)>(Variant(place!(Field::<Adt46>(Variant(_22.fld0, 1), 4)), 3), 1)).0);
_4.fld4.4 = -Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).4;
_4.fld4.1 = _34;
_54 = 18848874464971533721290526195040140737_u128 as f32;
_10 = _1;
_4.fld1 = _51.0;
_24 = (_18.0,);
_28.0.0 = _14;
place!(Field::<isize>(Variant(_22.fld0, 1), 2)) = _46 << _19.3;
_17 = _51.0;
place!(Field::<char>(Variant(_22.fld2, 1), 1)) = _41;
_58 = Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1).1 as isize;
_22.fld5 = _11 << _51.2;
place!(Field::<(usize, i8, i128, i128)>(Variant(_22.fld2, 1), 0)) = ((*_35), _43.1, _22.fld4, _43.2);
_24 = (Field::<[u16; 5]>(Variant(_22.fld0, 1), 0),);
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22.fld0, 1), 1)).1 = Field::<char>(Variant(_22.fld2, 1), 1);
Goto(bb15)
}
bb15 = {
Call(_61 = dump_var(1_usize, 15_usize, Move(_15), 27_usize, Move(_27), 6_usize, Move(_6), 45_usize, Move(_45)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_61 = dump_var(1_usize, 14_usize, Move(_14), 39_usize, Move(_39), 13_usize, Move(_13), 41_usize, Move(_41)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_61 = dump_var(1_usize, 16_usize, Move(_16), 46_usize, Move(_46), 28_usize, Move(_28), 44_usize, Move(_44)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_61 = dump_var(1_usize, 34_usize, Move(_34), 20_usize, Move(_20), 5_usize, Move(_5), 36_usize, Move(_36)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_61 = dump_var(1_usize, 8_usize, Move(_8), 62_usize, _62, 62_usize, _62, 62_usize, _62), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: bool,mut _2: bool) -> char {
mir! {
type RET = char;
let _3: char;
let _4: Adt55;
let _5: f64;
let _6: &'static isize;
let _7: ([i128; 5], i16);
let _8: isize;
let _9: f64;
let _10: u64;
let _11: [isize; 4];
let _12: u32;
let _13: i8;
let _14: bool;
let _15: f64;
let _16: [i128; 5];
let _17: f32;
let _18: ();
let _19: ();
{
_2 = _1;
RET = '\u{3b481}';
_1 = _2;
RET = '\u{b33d5}';
Goto(bb1)
}
bb1 = {
RET = '\u{31a25}';
RET = '\u{4ab12}';
_3 = RET;
_2 = !_1;
_5 = (-1401775393543799946_i64) as f64;
_1 = _2 > _2;
RET = _3;
_5 = 255_u8 as f64;
_7.0 = [(-88020900860705048012551443910385141538_i128),46124583785954652293896738066819242295_i128,(-101749805620365588502680946813246327569_i128),(-1779460054210723221288084011425119828_i128),(-62139378592894502816557086728056473553_i128)];
_2 = !_1;
_2 = _1 | _1;
_5 = 1913792842_i32 as f64;
Call(_4.fld4 = fn3(_1, _2, _1, _2, _1, _1, _1, _2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.0 = [_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4];
RET = _3;
Call(_4.fld1.fld0 = core::intrinsics::bswap(38712_u16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = !_2;
RET = _3;
_7.1 = !16533_i16;
_4.fld4 = 133060044137223024644066278004670531856_i128;
_4.fld4 = (-138637477330112973569312375427985690969_i128);
_4.fld5 = 252525980_i32 | 1151232403_i32;
_1 = _2 & _2;
_4.fld1 = Adt45 { fld0: 56461_u16 };
_7.1 = !21132_i16;
_7.1 = _4.fld5 as i16;
_3 = RET;
_6 = &_8;
_3 = RET;
_8 = 309722790966166719_u64 as isize;
_4.fld5 = !862826246_i32;
_7.0 = [_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4];
RET = _3;
_3 = RET;
_4.fld4 = (-157262662407672950322911076197464353463_i128) ^ 1150015965947224605120960691014802136_i128;
_4.fld1 = Adt45 { fld0: 54442_u16 };
_9 = _5 * _5;
_4.fld5 = 565546695_i32;
_8 = 9223372036854775807_isize >> _4.fld4;
_7.0 = [_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4];
_2 = _1;
match _4.fld5 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
565546695 => bb12,
_ => bb11
}
}
bb4 = {
_7.0 = [_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4];
RET = _3;
Call(_4.fld1.fld0 = core::intrinsics::bswap(38712_u16), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = '\u{31a25}';
RET = '\u{4ab12}';
_3 = RET;
_2 = !_1;
_5 = (-1401775393543799946_i64) as f64;
_1 = _2 > _2;
RET = _3;
_5 = 255_u8 as f64;
_7.0 = [(-88020900860705048012551443910385141538_i128),46124583785954652293896738066819242295_i128,(-101749805620365588502680946813246327569_i128),(-1779460054210723221288084011425119828_i128),(-62139378592894502816557086728056473553_i128)];
_2 = !_1;
_2 = _1 | _1;
_5 = 1913792842_i32 as f64;
Call(_4.fld4 = fn3(_1, _2, _1, _2, _1, _1, _1, _2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
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
_10 = 237645919358492934_u64;
_6 = &_8;
_4.fld5 = (-1486518528_i32);
_11 = [_8,(*_6),_8,_8];
match _4.fld5 {
0 => bb9,
1 => bb2,
340282366920938463463374607430281692928 => bb13,
_ => bb8
}
}
bb13 = {
_4.fld1 = Adt45 { fld0: 12779_u16 };
RET = _3;
_3 = RET;
_13 = _7.1 as i8;
Goto(bb14)
}
bb14 = {
_4.fld4 = 4_usize as i128;
RET = _3;
_4.fld4 = (-41548975833632189278390717817408020683_i128) << _4.fld5;
_8 = _7.1 as isize;
_6 = &_8;
_7.1 = RET as i16;
RET = _3;
_3 = RET;
_4.fld1 = Adt45 { fld0: 25376_u16 };
_10 = _4.fld4 as u64;
_6 = &(*_6);
_4.fld1.fld0 = 17333_u16 << _10;
_14 = !_2;
RET = _3;
_7.0 = [_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4];
_4.fld4 = _13 as i128;
_12 = 1214934117_u32 & 2879067620_u32;
_6 = &_8;
_12 = _9 as u32;
RET = _3;
_6 = &_8;
_1 = _2;
_4.fld1 = Adt45 { fld0: 50116_u16 };
_4.fld1.fld0 = _4.fld4 as u16;
_10 = 11247072800314490003_u64 + 15471142050623968833_u64;
_3 = RET;
_8 = 9223372036854775807_isize;
_17 = _7.1 as f32;
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(2_usize, 3_usize, Move(_3), 10_usize, Move(_10), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_18 = dump_var(2_usize, 13_usize, Move(_13), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool) -> i128 {
mir! {
type RET = i128;
let _11: f32;
let _12: u8;
let _13: u8;
let _14: ([i8; 2], [i128; 5]);
let _15: ();
let _16: ();
{
_8 = _9;
_8 = _7 < _4;
_9 = _7 != _3;
_10 = _2;
_9 = _3;
_7 = !_3;
Call(RET = fn4(_4, _5, _3, _6, _3, _10, _3, _1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = !_1;
_3 = !_1;
_9 = _5;
_3 = _4 <= _4;
_7 = _9 > _10;
_12 = !215_u8;
_2 = !_7;
_3 = _8 >= _8;
_8 = _7;
_13 = _12;
_4 = _5 & _2;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(3_usize, 8_usize, Move(_8), 5_usize, Move(_5), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_15 = dump_var(3_usize, 4_usize, Move(_4), 9_usize, Move(_9), 16_usize, _16, 16_usize, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool) -> i128 {
mir! {
type RET = i128;
let _10: isize;
let _11: f32;
let _12: f64;
let _13: f32;
let _14: (([i128; 5], i16), ([u16; 5],));
let _15: [u128; 1];
let _16: Adt52;
let _17: f32;
let _18: ([i8; 2], [i128; 5]);
let _19: [u128; 3];
let _20: u32;
let _21: usize;
let _22: ();
let _23: ();
{
RET = (-166813005854485467510090495185135824807_i128) * 147155056986427180018746723569376362584_i128;
RET = 118192064163282244696130383298040060834_i128;
_7 = _3 | _4;
_3 = _5 ^ _4;
_4 = !_9;
RET = 3171563429762089226431078661483329665_i128 << (-200252298_i32);
_9 = _7 | _3;
_10 = (-12_isize) - 9223372036854775807_isize;
_8 = !_5;
_2 = _3;
_13 = 3531041608_u32 as f32;
_1 = !_4;
_12 = 5_usize as f64;
_1 = _7 ^ _3;
_8 = _3 | _3;
_2 = _9 >= _4;
_7 = !_4;
_3 = _7 & _1;
_14.1.0 = [18418_u16,9653_u16,9301_u16,6781_u16,55789_u16];
_5 = !_2;
_1 = !_3;
_3 = _4 ^ _8;
Call(_2 = fn5(_4, _7, _3, _5, _5, _4, _5, _9, _5, _4, _3, _6, _1, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _9 <= _7;
_4 = _9;
_11 = _13;
_11 = _13 + _13;
Goto(bb2)
}
bb2 = {
_1 = _9 == _9;
_12 = 1249375108658230911_i64 as f64;
_14.1.0 = [31955_u16,15455_u16,17981_u16,4022_u16,34119_u16];
_6 = !_4;
_2 = !_1;
_7 = _9 > _4;
_14.0.0 = [RET,RET,RET,RET,RET];
_7 = !_8;
RET = (-31277899411323565872817479567461288206_i128);
_5 = _4;
_17 = 27_u8 as f32;
_11 = _13 * _17;
_15 = [207999010996320915654998901369397080554_u128];
_5 = _4;
_11 = _17;
_13 = _17 + _11;
_3 = _9 < _8;
_3 = !_8;
_18.1 = _14.0.0;
_4 = _5 != _8;
Goto(bb3)
}
bb3 = {
_14.1.0 = [20301_u16,25064_u16,40809_u16,41701_u16,1358_u16];
_18.0 = [43_i8,(-60_i8)];
Call(_16 = fn6(_6, _9, _5, _5, _5, _2, _5, _5, _8, _9, _5, _9, _8, _9), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = Field::<i128>(Variant(_16, 0), 0);
_16 = Adt52::Variant0 { fld0: RET };
_12 = 2690090857374565594_i64 as f64;
_14.1.0 = [11750_u16,63840_u16,46886_u16,64850_u16,45425_u16];
_12 = _10 as f64;
_14.0.1 = 54_i8 as i16;
_9 = _1;
_8 = _3 != _5;
_18.0 = [88_i8,74_i8];
_10 = _13 as isize;
SetDiscriminant(_16, 0);
_3 = !_9;
_3 = !_5;
_3 = !_1;
_14.0 = (_18.1, 379_i16);
Goto(bb5)
}
bb5 = {
Call(_22 = dump_var(4_usize, 1_usize, Move(_1), 2_usize, Move(_2), 5_usize, Move(_5), 8_usize, Move(_8)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_22 = dump_var(4_usize, 18_usize, Move(_18), 15_usize, Move(_15), 23_usize, _23, 23_usize, _23), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool) -> bool {
mir! {
type RET = bool;
let _15: f32;
let _16: f64;
let _17: Adt53;
let _18: ();
let _19: ();
{
_3 = _6;
_2 = !_12;
_11 = _2;
_7 = _11 < _5;
_4 = !_3;
RET = _4;
_12 = !_8;
_13 = _6 ^ _11;
_6 = _8 <= _14;
_11 = !_3;
RET = !_11;
_10 = _12;
_2 = !_1;
_5 = !_13;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(5_usize, 3_usize, Move(_3), 2_usize, Move(_2), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(5_usize, 14_usize, Move(_14), 1_usize, Move(_1), 6_usize, Move(_6), 19_usize, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool) -> Adt52 {
mir! {
type RET = Adt52;
let _15: usize;
let _16: u32;
let _17: *const (char, char, i16, u16, f64);
let _18: (char, char, i16, u16, f64);
let _19: Adt51;
let _20: *const *const &'static isize;
let _21: isize;
let _22: i128;
let _23: [u32; 4];
let _24: bool;
let _25: f64;
let _26: ();
let _27: ();
{
_2 = _8 & _7;
_4 = _1 != _2;
_8 = !_12;
_9 = _5;
_15 = 5225165614612590488_usize;
RET = Adt52::Variant0 { fld0: 101255025930192994863830847331621585142_i128 };
place!(Field::<i128>(Variant(RET, 0), 0)) = 134028227002239095288973048324156941066_i128;
_5 = !_13;
_11 = !_4;
_15 = 8759778616042275851_u64 as usize;
_3 = _4 > _13;
_15 = 6_usize;
_15 = (-8272677430983217487_i64) as usize;
_14 = _7;
_9 = _8;
_6 = !_5;
place!(Field::<i128>(Variant(RET, 0), 0)) = !(-57732320404731468029701690783203914059_i128);
Goto(bb1)
}
bb1 = {
_1 = _11;
_2 = _13 ^ _11;
_7 = _9 != _3;
_4 = _12 >= _3;
Call(_8 = fn7(_1, _5, _1, _10, _7, _12, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<i128>(Variant(RET, 0), 0)) = 9223372036854775807_isize as i128;
_14 = !_6;
_13 = _4;
_5 = !_8;
_14 = _11;
_5 = _7;
Goto(bb3)
}
bb3 = {
place!(Field::<i128>(Variant(RET, 0), 0)) = 36951154947779465554522753478491551189_i128 + (-120409976238923991146747345467508119316_i128);
RET = Adt52::Variant0 { fld0: (-82909551816595760715829712407771949726_i128) };
RET = Adt52::Variant0 { fld0: 153214213351152981698667662733508856373_i128 };
_6 = _9 != _11;
_7 = _5 > _5;
_16 = 204034474566390099_i64 as u32;
_18.2 = 20948_i16;
_18.1 = '\u{e0b36}';
_6 = !_13;
_4 = !_14;
place!(Field::<i128>(Variant(RET, 0), 0)) = (-128647387899051850145376713476251058687_i128) - 54867039076805841029000395114659618118_i128;
_10 = !_7;
RET = Adt52::Variant0 { fld0: (-100952789844253449949065515402882652915_i128) };
_10 = _8;
_3 = _4 > _14;
Goto(bb4)
}
bb4 = {
_12 = !_6;
_17 = core::ptr::addr_of!(_18);
_3 = _12 | _13;
_11 = _13 & _12;
_18.4 = (-81_i8) as f64;
_6 = _8 == _4;
_18.0 = (*_17).1;
_7 = !_5;
_16 = 4196692210_u32 << (*_17).2;
_5 = _2;
_18.3 = 29531_u16 & 32590_u16;
Goto(bb5)
}
bb5 = {
_18.2 = -(-14884_i16);
_18.1 = _18.0;
_11 = _14;
_6 = _11 >= _14;
_6 = _5;
_6 = _12;
RET = Adt52::Variant0 { fld0: 49885974399661596374388022144262123602_i128 };
_21 = 43_isize >> _16;
_3 = !_7;
_18.3 = !28039_u16;
place!(Field::<i128>(Variant(RET, 0), 0)) = (-122834305238633044665647457336272920169_i128) ^ (-108230097691706437764049045676042703071_i128);
_2 = _11 | _8;
RET = Adt52::Variant0 { fld0: 119015230965499333026932755191506306504_i128 };
Goto(bb6)
}
bb6 = {
_21 = (-5549988631231648315_i64) as isize;
_18.3 = 47349_u16 - 19486_u16;
_14 = _7;
_23 = [_16,_16,_16,_16];
place!(Field::<i128>(Variant(RET, 0), 0)) = _14 as i128;
_21 = 126_isize;
Goto(bb7)
}
bb7 = {
Call(_26 = dump_var(6_usize, 21_usize, Move(_21), 6_usize, Move(_6), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_26 = dump_var(6_usize, 5_usize, Move(_5), 2_usize, Move(_2), 14_usize, Move(_14), 4_usize, Move(_4)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_26 = dump_var(6_usize, 9_usize, Move(_9), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool) -> bool {
mir! {
type RET = bool;
let _8: [i8; 2];
let _9: Adt48;
let _10: Adt45;
let _11: i32;
let _12: [isize; 4];
let _13: i8;
let _14: ();
let _15: ();
{
_5 = !_7;
_2 = !_4;
_4 = _2;
_3 = !_6;
Goto(bb1)
}
bb1 = {
_3 = !_2;
RET = !_3;
_7 = !_1;
_2 = !RET;
_9.fld2 = !218_u8;
_10 = Adt45 { fld0: 518_u16 };
_9.fld0 = _9.fld2 as i128;
_9.fld1 = [34_isize,81_isize,(-78_isize),(-9223372036854775808_isize)];
_10 = Adt45 { fld0: 53272_u16 };
_9.fld1 = [(-9223372036854775808_isize),9223372036854775807_isize,(-122_isize),56_isize];
_7 = !_5;
_11 = 384920174_i32;
_5 = !_1;
_6 = !_1;
_9.fld2 = (-122_isize) as u8;
_12 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,55_isize];
_10 = Adt45 { fld0: 32738_u16 };
_12 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_8 = [(-87_i8),32_i8];
_10.fld0 = 4_usize as u16;
_10.fld0 = 320_u16;
_9.fld1 = _12;
_13 = _10.fld0 as i8;
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(7_usize, 12_usize, Move(_12), 11_usize, Move(_11), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_14 = dump_var(7_usize, 2_usize, Move(_2), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: bool,mut _2: bool,mut _3: i16,mut _4: bool,mut _5: (usize, i8, i128, i128),mut _6: ([u16; 5],),mut _7: bool,mut _8: u16,mut _9: Adt45,mut _10: bool) -> (char, char, i16, u16, f64) {
mir! {
type RET = (char, char, i16, u16, f64);
let _11: [u128; 3];
let _12: Adt45;
let _13: bool;
let _14: bool;
let _15: (char, char, i16, u16, f64);
let _16: bool;
let _17: [u128; 1];
let _18: f32;
let _19: ();
let _20: ();
{
_5.3 = _5.2;
_5.1 = -35_i8;
_8 = _9.fld0;
_6.0 = [_9.fld0,_9.fld0,_8,_9.fld0,_9.fld0];
_2 = _7;
RET.0 = '\u{3c37b}';
_1 = _4;
_2 = _7 > _10;
_5.1 = -37_i8;
RET.2 = _3 ^ _3;
_11 = [268946481996435718602150402897262145591_u128,271435496853140728488658414392656796995_u128,155284500366856561967781722438063106252_u128];
RET.0 = '\u{20534}';
RET.0 = '\u{faeaa}';
_3 = RET.2;
RET.3 = !_8;
_5 = (6_usize, 113_i8, 79368590569536392944307961347278590338_i128, (-81268335395351718492304439031017884459_i128));
_11 = [275381985105477468988707837838950753388_u128,199751487662503842326436565576065922661_u128,131881385723522691428660196427243374598_u128];
match _5.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
6 => bb6,
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
_12.fld0 = RET.3 - _8;
_5.2 = !_5.3;
_14 = _10 ^ _4;
RET.3 = _12.fld0 | _8;
RET.4 = (-56_isize) as f64;
RET.0 = '\u{f90d3}';
RET.0 = '\u{39cb6}';
_10 = _14;
RET.0 = '\u{286e3}';
_10 = _1;
_15.3 = _3 as u16;
_3 = RET.2;
_6.0 = [RET.3,_15.3,RET.3,RET.3,_15.3];
RET.1 = RET.0;
RET.1 = RET.0;
_5.2 = _5.3 ^ _5.3;
_5 = (1385530464839203750_usize, (-72_i8), 94018622240784837727311412416095420818_i128, 131767192177523202968276651437492866634_i128);
_17 = [121091268690010529229069212159833164639_u128];
_4 = _7 != _14;
_9 = Adt45 { fld0: _15.3 };
_11 = [4814345575607012945099793993942508303_u128,271812094136573909222808034638732876819_u128,357063328033870158234024454354769605_u128];
_15.2 = !RET.2;
Goto(bb7)
}
bb7 = {
Call(_19 = dump_var(8_usize, 11_usize, Move(_11), 7_usize, Move(_7), 1_usize, Move(_1), 14_usize, Move(_14)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_19 = dump_var(8_usize, 5_usize, Move(_5), 4_usize, Move(_4), 20_usize, _20, 20_usize, _20), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: Adt46,mut _2: (([i128; 5], i16), ([u16; 5],)),mut _3: bool,mut _4: Adt41,mut _5: (char, char, i16, u16, f64),mut _6: bool,mut _7: ([u16; 5],),mut _8: i16,mut _9: (char, char, i16, u16, f64),mut _10: (char, char, i16, u16, f64),mut _11: i8,mut _12: [u16; 5],mut _13: *const (char, char, i16, u16, f64),mut _14: ([u16; 5],),mut _15: i16) -> char {
mir! {
type RET = char;
let _16: &'static isize;
let _17: (usize, i8, i128, i128);
let _18: ([i128; 5], i16);
let _19: f64;
let _20: f64;
let _21: (char, char, i16, u16, f64);
let _22: f64;
let _23: isize;
let _24: bool;
let _25: [i8; 2];
let _26: *mut (char, char, i16, u16, f64);
let _27: f64;
let _28: [u128; 3];
let _29: Adt52;
let _30: isize;
let _31: *const (char, char, i16, u16, f64);
let _32: f32;
let _33: [isize; 4];
let _34: (usize, i8, i128, i128);
let _35: u16;
let _36: [i8; 2];
let _37: char;
let _38: [u32; 7];
let _39: [i128; 5];
let _40: Adt52;
let _41: [u16; 5];
let _42: Adt42;
let _43: i64;
let _44: Adt49;
let _45: u128;
let _46: Adt52;
let _47: Adt52;
let _48: (usize, [i8; 2], bool);
let _49: Adt53;
let _50: ();
let _51: ();
{
_6 = _4.fld4.1 == _10.0;
_2.0.1 = !_5.2;
_10.1 = _9.0;
_15 = !_5.2;
_4.fld4.0 = _10.1;
_9.1 = _4.fld4.1;
_13 = core::ptr::addr_of!(_5);
_10.1 = _9.1;
RET = (*_13).0;
_4.fld1 = RET;
_9 = _4.fld4;
_4.fld2 = -_9.4;
_10 = (*_13);
_9.3 = !_5.3;
_4.fld4 = ((*_13).0, _10.0, _2.0.1, (*_13).3, _4.fld2);
_10.0 = _9.0;
_14 = (_4.fld3.0,);
_4.fld4.1 = _10.0;
RET = _10.1;
_17.3 = (-86207973450219275360128210821554728972_i128);
_17.1 = _11 << _4.fld4.2;
Call(_2.0 = fn10(_10, Move(_1)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10.3 = !_5.3;
_9.3 = (*_13).2 as u16;
_4.fld4.3 = _10.3 + _9.3;
_2.1.0 = [(*_13).3,(*_13).3,_4.fld4.3,_4.fld4.3,(*_13).3];
_9.2 = (*_13).1 as i16;
_7 = _2.1;
Call(_17.0 = core::intrinsics::bswap(8118664942030624389_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18 = _2.0;
_4.fld1 = _5.0;
_7 = (_4.fld3.0,);
_10.2 = _18.1;
_10 = (*_13);
_9.3 = !_10.3;
_14.0 = [_4.fld4.3,(*_13).3,_5.3,(*_13).3,(*_13).3];
_4.fld4.1 = (*_13).1;
_4.fld4.2 = _10.2;
_10.4 = _4.fld2 - _4.fld2;
_10 = ((*_13).1, _9.0, _18.1, _9.3, _4.fld2);
_4.fld4.1 = (*_13).0;
_9.1 = RET;
_6 = _4.fld4.2 == _5.2;
_5.4 = -_4.fld4.4;
_4.fld3 = (_14.0,);
Goto(bb3)
}
bb3 = {
_21.4 = (*_13).4;
_17.2 = _17.3;
_9.1 = RET;
_21 = ((*_13).1, RET, _8, _9.3, (*_13).4);
_21.4 = _4.fld2;
_7 = _4.fld3;
RET = _5.1;
_1 = Adt46::Variant2 { fld0: _2 };
_21.4 = _9.4;
SetDiscriminant(_1, 1);
Goto(bb4)
}
bb4 = {
_19 = _10.4;
_4.fld0 = !_6;
place!(Field::<u128>(Variant(_1, 1), 0)) = 110716163167684568172600667078712442192_u128;
_17.0 = 4_usize;
_5.1 = _10.0;
_2.0 = (_18.0, _8);
place!(Field::<f32>(Variant(_1, 1), 1)) = 3552083896_u32 as f32;
_10.4 = _19 * _5.4;
_5.3 = _21.3 * _9.3;
_18.0 = _2.0.0;
_5.4 = _17.0 as f64;
_23 = !94_isize;
place!(Field::<(char, char, i16, u16, f64)>(Variant(_1, 1), 2)).4 = _10.4 * _19;
place!(Field::<(char, char, i16, u16, f64)>(Variant(_1, 1), 2)).4 = _19;
_1 = Adt46::Variant2 { fld0: _2 };
_9.2 = !_4.fld4.2;
_9.3 = 16366741888034201292_u64 as u16;
_8 = -_5.2;
_9.2 = 744336932_u32 as i16;
_22 = (-624745659_i32) as f64;
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_1, 2), 0)) = (_2.0, _7);
_14.0 = _4.fld3.0;
_2.0 = (Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_1, 2), 0).0.0, _18.1);
_25 = [_17.1,_17.1];
match _17.3 {
0 => bb3,
254074393470719188103246396610213482484 => bb5,
_ => bb2
}
}
bb5 = {
SetDiscriminant(_1, 3);
RET = _4.fld4.0;
_10.0 = _4.fld4.1;
_2.1 = (_7.0,);
_10.0 = (*_13).0;
_4.fld3 = _2.1;
_4.fld4.0 = (*_13).1;
place!(Field::<i64>(Variant(_1, 3), 0)) = _17.1 as i64;
_4.fld2 = -_21.4;
_30 = !_23;
_4 = Adt41 { fld0: _3,fld1: (*_13).0,fld2: _21.4,fld3: _7,fld4: (*_13) };
_5.0 = _4.fld4.1;
_10.2 = _2.0.1 + _4.fld4.2;
_25 = [_17.1,_17.1];
_9.3 = 27471510082379920640904019028197659427_u128 as u16;
_4.fld4 = (*_13);
_26 = core::ptr::addr_of_mut!(_5);
Call(_9.4 = core::intrinsics::transmute(Field::<i64>(Variant(_1, 3), 0)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_20 = _9.4;
_9.3 = _5.3;
_4.fld3 = (_12,);
_2.0.1 = _18.1 + _4.fld4.2;
_16 = &_30;
Call(_27 = core::intrinsics::transmute(Field::<i64>(Variant(_1, 3), 0)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_23 = _17.1 as isize;
_24 = _4.fld0 & _4.fld0;
_17.0 = _17.1 as usize;
_2.0 = _18;
_17 = (2216999878868527576_usize, _11, (-51241118383448176576393897790018876614_i128), (-98196913275385914915771953220461195395_i128));
_10.1 = _9.0;
_17.3 = _17.2;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_1, 3), 1)).1 = [_17.1,_11];
_34.3 = _17.3;
_21 = (*_26);
RET = (*_26).1;
_36 = _25;
_34.0 = !_17.0;
_12 = [_5.3,(*_26).3,(*_13).3,_5.3,_4.fld4.3];
_10 = ((*_13).1, (*_26).0, (*_26).2, _9.3, _20);
_16 = &(*_16);
_15 = -(*_13).2;
_4.fld4.2 = (*_26).2;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_1, 3), 1)) = (_17.0, _25, _3);
Call(_28 = fn11((*_13), _13, _2, _21.3, _13, _4.fld3, _26, (*_26), Field::<i64>(Variant(_1, 3), 0), _2.1.0, _14.0, (*_26).3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4.fld0 = _3;
_34.0 = _10.4 as usize;
_27 = -_20;
_3 = _6;
_10.4 = _20;
_38 = [3877474749_u32,3633978245_u32,3569555790_u32,895060415_u32,1291003986_u32,1824722756_u32,4041507601_u32];
_21 = (_10.0, _4.fld1, (*_26).2, (*_13).3, _27);
_9.1 = (*_13).1;
_4.fld4.2 = _4.fld4.3 as i16;
_2.0.0 = _18.0;
_34.3 = !_17.2;
_21.1 = _5.1;
_4.fld4.4 = _10.4 - _27;
_4.fld4.2 = (*_13).2 & _5.2;
_43 = -Field::<i64>(Variant(_1, 3), 0);
_34.2 = _21.1 as i128;
_30 = _23 << _4.fld4.2;
_21 = _5;
_43 = _20 as i64;
_2.0.1 = _30 as i16;
_39 = [_17.2,_34.3,_34.3,_34.3,_34.3];
_10 = (_9.1, (*_26).1, _8, _5.3, _20);
_34 = (_17.0, _17.1, _17.2, _17.2);
_45 = !133608274590894906163260284699670910135_u128;
Goto(bb9)
}
bb9 = {
_27 = _34.1 as f64;
_2 = (_18, _14);
_34 = (_17.0, _17.1, _17.3, _17.3);
_4.fld4.3 = _45 as u16;
_46 = Adt52::Variant0 { fld0: _17.2 };
_13 = core::ptr::addr_of!(_4.fld4);
_43 = _10.4 as i64;
_5.2 = _4.fld4.2 * _8;
_29 = Move(_46);
_9 = (RET, _5.0, (*_26).2, (*_26).3, _4.fld4.4);
_21.1 = _5.0;
_31 = core::ptr::addr_of!(_4.fld4);
_4.fld3.0 = _2.1.0;
_2.0.1 = (*_26).2 - _10.2;
_21 = ((*_26).0, (*_31).0, _10.2, _5.3, _4.fld4.4);
_2.0.0 = _18.0;
_24 = _4.fld0 & Field::<(usize, [i8; 2], bool)>(Variant(_1, 3), 1).2;
_38 = [4068864891_u32,439362974_u32,962824860_u32,1136522949_u32,3184884046_u32,1148925300_u32,3807778154_u32];
_7.0 = [_21.3,_21.3,(*_26).3,_5.3,_21.3];
_21.0 = (*_13).0;
_35 = !_5.3;
Goto(bb10)
}
bb10 = {
_18.1 = !(*_13).2;
_28 = [_45,_45,_45];
_41 = [_5.3,(*_26).3,(*_26).3,_21.3,(*_26).3];
_9.2 = !(*_31).2;
_40 = Adt52::Variant0 { fld0: Field::<i128>(Variant(_29, 0), 0) };
Call(_25 = core::intrinsics::transmute(_8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_34.1 = _23 as i8;
_21.4 = 613280588_i32 as f64;
_10.0 = (*_13).0;
_32 = 230_u8 as f32;
_41 = [_10.3,_10.3,_35,_35,_35];
_5 = (*_31);
_2 = (_18, _4.fld3);
_24 = !_3;
_34.1 = 146_u8 as i8;
_5.4 = 2753983869_u32 as f64;
RET = (*_26).0;
_16 = &_30;
_39 = [Field::<i128>(Variant(_29, 0), 0),_17.2,Field::<i128>(Variant(_29, 0), 0),_17.2,_17.2];
_39 = _2.0.0;
_48.0 = _23 as usize;
Goto(bb12)
}
bb12 = {
_48.2 = _6;
_19 = (*_13).4 + _20;
_37 = (*_26).0;
_4.fld3.0 = [_21.3,_10.3,_9.3,_9.3,_35];
_4.fld4 = ((*_26).0, _10.0, _18.1, _9.3, _10.4);
place!(Field::<(usize, [i8; 2], bool)>(Variant(_1, 3), 1)) = (_48.0, _25, _3);
_24 = !_3;
_21 = ((*_13).0, _4.fld1, (*_26).2, (*_31).3, _4.fld4.4);
match _34.0 {
0 => bb7,
1 => bb10,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb13,
6 => bb14,
2216999878868527576 => bb16,
_ => bb15
}
}
bb13 = {
_34.1 = _23 as i8;
_21.4 = 613280588_i32 as f64;
_10.0 = (*_13).0;
_32 = 230_u8 as f32;
_41 = [_10.3,_10.3,_35,_35,_35];
_5 = (*_31);
_2 = (_18, _4.fld3);
_24 = !_3;
_34.1 = 146_u8 as i8;
_5.4 = 2753983869_u32 as f64;
RET = (*_26).0;
_16 = &_30;
_39 = [Field::<i128>(Variant(_29, 0), 0),_17.2,Field::<i128>(Variant(_29, 0), 0),_17.2,_17.2];
_39 = _2.0.0;
_48.0 = _23 as usize;
Goto(bb12)
}
bb14 = {
_10.3 = !_5.3;
_9.3 = (*_13).2 as u16;
_4.fld4.3 = _10.3 + _9.3;
_2.1.0 = [(*_13).3,(*_13).3,_4.fld4.3,_4.fld4.3,(*_13).3];
_9.2 = (*_13).1 as i16;
_7 = _2.1;
Call(_17.0 = core::intrinsics::bswap(8118664942030624389_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_27 = _34.1 as f64;
_2 = (_18, _14);
_34 = (_17.0, _17.1, _17.3, _17.3);
_4.fld4.3 = _45 as u16;
_46 = Adt52::Variant0 { fld0: _17.2 };
_13 = core::ptr::addr_of!(_4.fld4);
_43 = _10.4 as i64;
_5.2 = _4.fld4.2 * _8;
_29 = Move(_46);
_9 = (RET, _5.0, (*_26).2, (*_26).3, _4.fld4.4);
_21.1 = _5.0;
_31 = core::ptr::addr_of!(_4.fld4);
_4.fld3.0 = _2.1.0;
_2.0.1 = (*_26).2 - _10.2;
_21 = ((*_26).0, (*_31).0, _10.2, _5.3, _4.fld4.4);
_2.0.0 = _18.0;
_24 = _4.fld0 & Field::<(usize, [i8; 2], bool)>(Variant(_1, 3), 1).2;
_38 = [4068864891_u32,439362974_u32,962824860_u32,1136522949_u32,3184884046_u32,1148925300_u32,3807778154_u32];
_7.0 = [_21.3,_21.3,(*_26).3,_5.3,_21.3];
_21.0 = (*_13).0;
_35 = !_5.3;
Goto(bb10)
}
bb16 = {
place!(Field::<(usize, [i8; 2], bool)>(Variant(_1, 3), 1)).2 = !_24;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_1, 3), 1)) = (_48.0, _36, _6);
_2.1 = (_4.fld3.0,);
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(_1, 3), 3)) = _13;
_21.0 = _4.fld1;
_5.4 = _4.fld4.4;
_27 = _21.4;
_17.1 = -_11;
Goto(bb17)
}
bb17 = {
Call(_50 = dump_var(9_usize, 3_usize, Move(_3), 8_usize, Move(_8), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(9_usize, 18_usize, Move(_18), 30_usize, Move(_30), 25_usize, Move(_25), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_50 = dump_var(9_usize, 6_usize, Move(_6), 45_usize, Move(_45), 28_usize, Move(_28), 37_usize, Move(_37)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: (char, char, i16, u16, f64),mut _2: Adt46) -> ([i128; 5], i16) {
mir! {
type RET = ([i128; 5], i16);
let _3: f64;
let _4: ();
let _5: ();
{
RET.0 = [(-8334212272592801604031452611715679355_i128),44359076095267885676489337904583331394_i128,(-42167188511296581396222605058058925060_i128),(-8932232190755491251066651678537490247_i128),(-69916121982884442175369374446144431478_i128)];
RET.0 = [28901469241405819870597702667288845479_i128,142083205141060768943331208931354792415_i128,76259054018600544915849481996424577947_i128,(-90468250837214936969736238130286364568_i128),37592613229912683757336738769171939774_i128];
RET.1 = -_1.2;
RET.0 = [(-5634104056413641432782855481718867093_i128),(-130851235680241445397047539299118972179_i128),(-30223382114146051048677950362388863953_i128),142177833541892125382185026378501104367_i128,83046981299607712512862702027889365665_i128];
_1.4 = (-1339007252_i32) as f64;
place!(Field::<bool>(Variant(_2, 0), 0)) = _1.2 > _1.2;
_1.4 = _1.2 as f64;
_1.3 = 53277_u16 ^ 7231_u16;
RET.0 = [100184326350290341169269336031461548615_i128,3386047995635525079285032890938481815_i128,(-114812046616220980935371318636948523427_i128),(-111386423180772424481550678585039650812_i128),4718467941400122165898980456668003309_i128];
_1.2 = !RET.1;
_1.0 = _1.1;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: (char, char, i16, u16, f64),mut _2: *const (char, char, i16, u16, f64),mut _3: (([i128; 5], i16), ([u16; 5],)),mut _4: u16,mut _5: *const (char, char, i16, u16, f64),mut _6: ([u16; 5],),mut _7: *mut (char, char, i16, u16, f64),mut _8: (char, char, i16, u16, f64),mut _9: i64,mut _10: [u16; 5],mut _11: [u16; 5],mut _12: u16) -> [u128; 3] {
mir! {
type RET = [u128; 3];
let _13: u128;
let _14: bool;
let _15: u8;
let _16: char;
let _17: [u32; 4];
let _18: [i8; 2];
let _19: *mut (char, char, i16, u16, f64);
let _20: [u128; 1];
let _21: [u16; 5];
let _22: Adt47;
let _23: Adt47;
let _24: Adt51;
let _25: char;
let _26: char;
let _27: [u16; 5];
let _28: f32;
let _29: [u128; 1];
let _30: Adt50;
let _31: u32;
let _32: *const (char, char, i16, u16, f64);
let _33: u128;
let _34: [u32; 4];
let _35: isize;
let _36: u16;
let _37: ();
let _38: ();
{
_8.3 = !_12;
_6.0 = _10;
_13 = 19539522186329103885432651511333961024_u128;
_8.1 = _1.1;
RET = [_13,_13,_13];
_8.4 = _1.4 + _1.4;
_12 = _8.3;
_8.1 = _8.0;
RET = [_13,_13,_13];
RET = [_13,_13,_13];
_8.4 = _1.4 * _1.4;
_8.2 = -_1.2;
_15 = _4 as u8;
_1.2 = !_3.0.1;
_5 = _2;
_1.1 = _8.1;
RET = [_13,_13,_13];
_7 = core::ptr::addr_of_mut!(_1);
_18 = [(-107_i8),1_i8];
_1.1 = (*_7).0;
_10 = [(*_7).3,_12,_1.3,(*_7).3,(*_7).3];
Call(_17 = fn12((*_7), _5, _3.1.0, (*_7), _3.1.0, _2, (*_7).3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = _4 == (*_7).3;
_13 = 269156691197574905020541969826902786491_u128;
_3.0.1 = (*_7).2 - (*_7).2;
_12 = _14 as u16;
_3.1.0 = [(*_7).3,_12,(*_7).3,(*_7).3,_8.3];
_1.3 = _4;
_16 = (*_7).1;
_1.4 = -_8.4;
_7 = core::ptr::addr_of_mut!(_8);
_7 = core::ptr::addr_of_mut!((*_7));
_12 = (*_7).3;
_8.4 = -_1.4;
_1.0 = _1.1;
_13 = 56_i8 as u128;
_8.2 = _3.0.1;
_1.1 = (*_7).1;
_10 = [_1.3,_12,_4,_8.3,_12];
_8.2 = _3.0.1;
_3.1 = _6;
_21 = [_12,_12,_8.3,(*_7).3,_4];
_18 = [22_i8,1_i8];
_13 = !279723528462642743030369047114854840239_u128;
_1.2 = (*_7).2 >> _8.3;
_8.3 = _4 ^ _12;
_1 = ((*_7).1, (*_7).0, _8.2, (*_7).3, _8.4);
Goto(bb2)
}
bb2 = {
_19 = _7;
_8 = _1;
_3.0.1 = !(*_7).2;
_21 = [_8.3,_12,(*_7).3,_12,(*_7).3];
_8.1 = _1.1;
_17 = [2010087948_u32,1272492868_u32,199353478_u32,2556864926_u32];
_19 = core::ptr::addr_of_mut!((*_7));
_3.0.1 = _1.2 << _1.2;
RET = [_13,_13,_13];
_3.1 = (_6.0,);
_6.0 = [_8.3,(*_19).3,_12,_8.3,(*_7).3];
_23 = Adt47::Variant2 { fld0: _3.1 };
_5 = _2;
_1.1 = (*_7).0;
_11 = _10;
_22 = Move(_23);
_8.2 = _3.0.1 & _3.0.1;
_3.1 = _6;
_23 = Adt47::Variant2 { fld0: _3.1 };
_1.4 = -(*_7).4;
place!(Field::<([u16; 5],)>(Variant(_23, 2), 0)).0 = [_1.3,(*_19).3,(*_7).3,_8.3,(*_19).3];
SetDiscriminant(_22, 1);
place!(Field::<([i128; 5], i16)>(Variant(_22, 1), 5)) = (_3.0.0, _3.0.1);
_25 = (*_19).1;
place!(Field::<f32>(Variant(_22, 1), 1)) = _3.0.1 as f32;
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4)).0 = _1.1;
_20 = [_13];
_5 = _2;
_3.0.1 = (*_7).4 as i16;
Goto(bb3)
}
bb3 = {
place!(Field::<([u16; 5],)>(Variant(_22, 1), 3)) = (_3.1.0,);
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4)).2 = (*_7).2 << (*_7).2;
place!(Field::<([u16; 5],)>(Variant(_23, 2), 0)).0 = [_12,_4,(*_19).3,_1.3,(*_7).3];
_11 = _21;
place!(Field::<*mut i16>(Variant(_22, 1), 2)) = core::ptr::addr_of_mut!((*_7).2);
_4 = _8.3;
SetDiscriminant(_23, 0);
place!(Field::<*mut i16>(Variant(_22, 1), 2)) = core::ptr::addr_of_mut!((*_7).2);
place!(Field::<([i128; 5], i16)>(Variant(_22, 1), 5)).1 = Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4).2;
_23 = Adt47::Variant2 { fld0: _6 };
_20 = [_13];
_25 = (*_19).0;
_5 = core::ptr::addr_of!((*_7));
_1.2 = Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4).2;
Goto(bb4)
}
bb4 = {
place!(Field::<([i128; 5], i16)>(Variant(_22, 1), 5)).0 = _3.0.0;
_1.1 = (*_7).0;
place!(Field::<*mut i16>(Variant(_22, 1), 2)) = core::ptr::addr_of_mut!((*_19).2);
_9 = 1581376606215213469_u64 as i64;
place!(Field::<u64>(Variant(_22, 1), 0)) = 1978839677206037616_u64;
_18 = [(-6_i8),36_i8];
_4 = (*_19).4 as u16;
_3.1 = (Field::<([u16; 5],)>(Variant(_22, 1), 3).0,);
_13 = 7_usize as u128;
_15 = 115_u8;
_21 = [(*_19).3,(*_5).3,(*_19).3,_1.3,(*_5).3];
SetDiscriminant(_23, 2);
place!(Field::<*mut i16>(Variant(_22, 1), 2)) = core::ptr::addr_of_mut!(_8.2);
Call(_24 = fn18(_10, Field::<([u16; 5],)>(Variant(_22, 1), 3), _7, (*_7).2, (*_19).2, (*_19).2, (*_19), Field::<f32>(Variant(_22, 1), 1), _7, (*_7), _7, _8, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4)).0 = (*_7).1;
_5 = _2;
place!(Field::<([u16; 5],)>(Variant(_23, 2), 0)) = Field::<([u16; 5],)>(Variant(_22, 1), 3);
_15 = 29_u8 * 187_u8;
_16 = (*_19).1;
_8.2 = _1.2;
match Field::<u64>(Variant(_22, 1), 0) {
0 => bb4,
1 => bb3,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
1978839677206037616 => bb12,
_ => bb11
}
}
bb6 = {
place!(Field::<([i128; 5], i16)>(Variant(_22, 1), 5)).0 = _3.0.0;
_1.1 = (*_7).0;
place!(Field::<*mut i16>(Variant(_22, 1), 2)) = core::ptr::addr_of_mut!((*_19).2);
_9 = 1581376606215213469_u64 as i64;
place!(Field::<u64>(Variant(_22, 1), 0)) = 1978839677206037616_u64;
_18 = [(-6_i8),36_i8];
_4 = (*_19).4 as u16;
_3.1 = (Field::<([u16; 5],)>(Variant(_22, 1), 3).0,);
_13 = 7_usize as u128;
_15 = 115_u8;
_21 = [(*_19).3,(*_5).3,(*_19).3,_1.3,(*_5).3];
SetDiscriminant(_23, 2);
place!(Field::<*mut i16>(Variant(_22, 1), 2)) = core::ptr::addr_of_mut!(_8.2);
Call(_24 = fn18(_10, Field::<([u16; 5],)>(Variant(_22, 1), 3), _7, (*_7).2, (*_19).2, (*_19).2, (*_19), Field::<f32>(Variant(_22, 1), 1), _7, (*_7), _7, _8, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
place!(Field::<([u16; 5],)>(Variant(_22, 1), 3)) = (_3.1.0,);
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4)).2 = (*_7).2 << (*_7).2;
place!(Field::<([u16; 5],)>(Variant(_23, 2), 0)).0 = [_12,_4,(*_19).3,_1.3,(*_7).3];
_11 = _21;
place!(Field::<*mut i16>(Variant(_22, 1), 2)) = core::ptr::addr_of_mut!((*_7).2);
_4 = _8.3;
SetDiscriminant(_23, 0);
place!(Field::<*mut i16>(Variant(_22, 1), 2)) = core::ptr::addr_of_mut!((*_7).2);
place!(Field::<([i128; 5], i16)>(Variant(_22, 1), 5)).1 = Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4).2;
_23 = Adt47::Variant2 { fld0: _6 };
_20 = [_13];
_25 = (*_19).0;
_5 = core::ptr::addr_of!((*_7));
_1.2 = Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4).2;
Goto(bb4)
}
bb8 = {
_19 = _7;
_8 = _1;
_3.0.1 = !(*_7).2;
_21 = [_8.3,_12,(*_7).3,_12,(*_7).3];
_8.1 = _1.1;
_17 = [2010087948_u32,1272492868_u32,199353478_u32,2556864926_u32];
_19 = core::ptr::addr_of_mut!((*_7));
_3.0.1 = _1.2 << _1.2;
RET = [_13,_13,_13];
_3.1 = (_6.0,);
_6.0 = [_8.3,(*_19).3,_12,_8.3,(*_7).3];
_23 = Adt47::Variant2 { fld0: _3.1 };
_5 = _2;
_1.1 = (*_7).0;
_11 = _10;
_22 = Move(_23);
_8.2 = _3.0.1 & _3.0.1;
_3.1 = _6;
_23 = Adt47::Variant2 { fld0: _3.1 };
_1.4 = -(*_7).4;
place!(Field::<([u16; 5],)>(Variant(_23, 2), 0)).0 = [_1.3,(*_19).3,(*_7).3,_8.3,(*_19).3];
SetDiscriminant(_22, 1);
place!(Field::<([i128; 5], i16)>(Variant(_22, 1), 5)) = (_3.0.0, _3.0.1);
_25 = (*_19).1;
place!(Field::<f32>(Variant(_22, 1), 1)) = _3.0.1 as f32;
place!(Field::<(char, char, i16, u16, f64)>(Variant(_22, 1), 4)).0 = _1.1;
_20 = [_13];
_5 = _2;
_3.0.1 = (*_7).4 as i16;
Goto(bb3)
}
bb9 = {
_14 = _4 == (*_7).3;
_13 = 269156691197574905020541969826902786491_u128;
_3.0.1 = (*_7).2 - (*_7).2;
_12 = _14 as u16;
_3.1.0 = [(*_7).3,_12,(*_7).3,(*_7).3,_8.3];
_1.3 = _4;
_16 = (*_7).1;
_1.4 = -_8.4;
_7 = core::ptr::addr_of_mut!(_8);
_7 = core::ptr::addr_of_mut!((*_7));
_12 = (*_7).3;
_8.4 = -_1.4;
_1.0 = _1.1;
_13 = 56_i8 as u128;
_8.2 = _3.0.1;
_1.1 = (*_7).1;
_10 = [_1.3,_12,_4,_8.3,_12];
_8.2 = _3.0.1;
_3.1 = _6;
_21 = [_12,_12,_8.3,(*_7).3,_4];
_18 = [22_i8,1_i8];
_13 = !279723528462642743030369047114854840239_u128;
_1.2 = (*_7).2 >> _8.3;
_8.3 = _4 ^ _12;
_1 = ((*_7).1, (*_7).0, _8.2, (*_7).3, _8.4);
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
SetDiscriminant(_23, 1);
place!(Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4)).1 = (*_7).1;
_1 = ((*_7).1, _16, (*_7).2, (*_7).3, (*_19).4);
place!(Field::<[i8; 2]>(Variant(_24, 2), 1)) = [55_i8,59_i8];
_15 = 104_u8 >> (*_19).3;
_6.0 = [_8.3,(*_19).3,(*_7).3,(*_7).3,_8.3];
SetDiscriminant(_24, 2);
Call(_28 = core::intrinsics::transmute((*_19).1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_18 = [(-17_i8),(-68_i8)];
place!(Field::<([u16; 5],)>(Variant(_22, 1), 3)) = _6;
_22 = Adt47::Variant2 { fld0: _6 };
_12 = _9 as u16;
place!(Field::<([u16; 5],)>(Variant(_23, 1), 3)) = Field::<([u16; 5],)>(Variant(_22, 2), 0);
_6.0 = [(*_7).3,(*_7).3,(*_7).3,(*_19).3,_8.3];
_24 = Adt51::Variant2 { fld0: _17,fld1: _18,fld2: _13 };
_28 = 1238310282_u32 as f32;
Goto(bb14)
}
bb14 = {
place!(Field::<([u16; 5],)>(Variant(_23, 1), 3)).0 = _11;
_13 = 6729241222912126487_u64 as u128;
RET = [Field::<u128>(Variant(_24, 2), 2),Field::<u128>(Variant(_24, 2), 2),Field::<u128>(Variant(_24, 2), 2)];
_5 = _2;
place!(Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4)).0 = _1.1;
_1.3 = Field::<u128>(Variant(_24, 2), 2) as u16;
place!(Field::<([i128; 5], i16)>(Variant(_23, 1), 5)).0 = [25458068395836449393817096062309778937_i128,(-89197846979994826189875876878956366513_i128),(-141524171415758660394137624645390041219_i128),149016305033806377831359928891515244858_i128,63208202352811545743279411171327231352_i128];
place!(Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4)) = ((*_19).1, (*_19).0, (*_7).2, _8.3, (*_7).4);
place!(Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4)).4 = _15 as f64;
_1 = ((*_19).0, (*_7).1, Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4).2, _8.3, Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4).4);
_30 = Adt50::Variant1 { fld0: 9223372036854775807_isize };
_33 = _13 * _13;
place!(Field::<[i8; 2]>(Variant(_24, 2), 1)) = _18;
_32 = core::ptr::addr_of!(_8);
_8 = (Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4).1, _1.1, _1.2, Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4).3, Field::<(char, char, i16, u16, f64)>(Variant(_23, 1), 4).4);
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(11_usize, 4_usize, Move(_4), 16_usize, Move(_16), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(11_usize, 14_usize, Move(_14), 13_usize, Move(_13), 15_usize, Move(_15), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: (char, char, i16, u16, f64),mut _2: *const (char, char, i16, u16, f64),mut _3: [u16; 5],mut _4: (char, char, i16, u16, f64),mut _5: [u16; 5],mut _6: *const (char, char, i16, u16, f64),mut _7: u16) -> [u32; 4] {
mir! {
type RET = [u32; 4];
let _8: [u32; 4];
let _9: Adt45;
let _10: Adt55;
let _11: [i128; 5];
let _12: [u128; 1];
let _13: i128;
let _14: bool;
let _15: isize;
let _16: i64;
let _17: (([i128; 5], i16), ([u16; 5],));
let _18: *const &'static isize;
let _19: i16;
let _20: (([i128; 5], i16), ([u16; 5],));
let _21: Adt44;
let _22: [u32; 4];
let _23: Adt53;
let _24: char;
let _25: (usize, i8, i128, i128);
let _26: i8;
let _27: (usize, [i8; 2], bool);
let _28: [i8; 2];
let _29: (char, char, i16, u16, f64);
let _30: isize;
let _31: ([i8; 2], [i128; 5]);
let _32: bool;
let _33: bool;
let _34: (([i128; 5], i16), ([u16; 5],));
let _35: Adt56;
let _36: [u128; 1];
let _37: Adt41;
let _38: *const *mut f64;
let _39: (([i128; 5], i16), ([u16; 5],));
let _40: Adt47;
let _41: Adt54;
let _42: *mut f64;
let _43: ([i8; 2], [i128; 5]);
let _44: ([i8; 2], [i128; 5]);
let _45: Adt54;
let _46: ([i8; 2], [i128; 5]);
let _47: u128;
let _48: bool;
let _49: ();
let _50: ();
{
_4.3 = !_1.3;
_4.3 = _1.3;
RET = [4211335791_u32,3768405365_u32,3209328904_u32,1488287537_u32];
RET = [308049282_u32,1660607356_u32,2463292204_u32,64803893_u32];
RET = [3258535659_u32,2516144244_u32,1869968868_u32,3424765980_u32];
_1 = _4;
_4.3 = _7;
_1.3 = !_7;
_1.3 = !_4.3;
RET = [3497127210_u32,549557225_u32,1541754866_u32,423501645_u32];
RET = [4239188088_u32,2100255595_u32,2450640662_u32,349254495_u32];
_4 = (_1.0, _1.0, _1.2, _7, _1.4);
_4.4 = -_1.4;
_3 = _5;
Goto(bb1)
}
bb1 = {
_9 = Adt45 { fld0: _7 };
_4.3 = _1.4 as u16;
_4.0 = _1.0;
_1.2 = 239_u8 as i16;
_1.4 = -_4.4;
_11 = [98314875393457993470086847327850968841_i128,(-139360995327374222873499508344054362030_i128),(-14412810712312006017347765063762954136_i128),(-27591190608919557561243615602610743995_i128),130035425193418725005951446203539647629_i128];
_1.3 = 47138270321401746829855657977727593055_i128 as u16;
_10.fld1.fld0 = _9.fld0;
_5 = [_7,_10.fld1.fld0,_9.fld0,_9.fld0,_10.fld1.fld0];
RET = [3086879248_u32,4140330265_u32,2247980287_u32,540818659_u32];
Call(_10.fld3 = fn13(_2, _10.fld1.fld0, _10.fld1, _7, _10.fld1, _2, _7, _4.2, _5, _7, _9, _10.fld1.fld0, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.2 = -_4.2;
_10.fld5 = 9223372036854775807_isize as i32;
place!(Field::<u128>(Variant(_10.fld3, 2), 2)) = 142637018200191945366075084925085156890_u128 >> _9.fld0;
SetDiscriminant(_10.fld3, 2);
_3 = _5;
_3 = [_7,_7,_10.fld1.fld0,_7,_7];
_5 = [_9.fld0,_10.fld1.fld0,_10.fld1.fld0,_9.fld0,_10.fld1.fld0];
place!(Field::<[u32; 4]>(Variant(_10.fld3, 2), 0)) = [1836177383_u32,4134848541_u32,807268878_u32,172896468_u32];
_1.1 = _4.1;
place!(Field::<[u32; 4]>(Variant(_10.fld3, 2), 0)) = RET;
_17.0 = (_11, _4.2);
_10.fld1.fld0 = _4.4 as u16;
_5 = [_7,_7,_7,_7,_9.fld0];
place!(Field::<[i8; 2]>(Variant(_10.fld3, 2), 1)) = [(-65_i8),77_i8];
_17.1 = (_5,);
_10.fld1 = Adt45 { fld0: _7 };
_12 = [16959408943660049509772928250047274590_u128];
place!(Field::<[i8; 2]>(Variant(_10.fld3, 2), 1)) = [(-114_i8),(-72_i8)];
RET = [3689007580_u32,4026182013_u32,3543445362_u32,3105251037_u32];
_6 = _2;
place!(Field::<u128>(Variant(_10.fld3, 2), 2)) = 239_u8 as u128;
Goto(bb3)
}
bb3 = {
_1.0 = _4.0;
_5 = _3;
_1.3 = _9.fld0;
_8 = [270737923_u32,1652209351_u32,3160102159_u32,1917445387_u32];
_8 = [2888312272_u32,739642218_u32,2023801615_u32,1977483944_u32];
_15 = (-9223372036854775808_isize) >> _10.fld1.fld0;
Call(_12 = core::intrinsics::transmute(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_22 = [2369399107_u32,3849039451_u32,2530929945_u32,3653329383_u32];
SetDiscriminant(_10.fld3, 0);
_13 = _10.fld5 as i128;
_10.fld5 = -701933719_i32;
_4 = _1;
_15 = -9223372036854775807_isize;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = !true;
_17.1 = (_5,);
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).1 = _17.0.0;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = false & false;
place!(Field::<[u32; 4]>(Variant(_10.fld3, 0), 4)) = [3927791820_u32,1556415001_u32,735680515_u32,3843566553_u32];
_20.0.0 = [_13,_13,_13,_13,_13];
RET = [2829101919_u32,1075094496_u32,2513386355_u32,2684858014_u32];
_9 = _10.fld1;
_9 = _10.fld1;
_9.fld0 = _10.fld5 as u16;
_20.1.0 = _3;
_20 = (_17.0, _17.1);
_25.3 = _13 - _13;
_1.1 = _4.1;
place!(Field::<u32>(Variant(_10.fld3, 0), 3)) = 4161333912_u32;
match Field::<u32>(Variant(_10.fld3, 0), 3) {
0 => bb1,
4161333912 => bb5,
_ => bb3
}
}
bb5 = {
_9 = Adt45 { fld0: _7 };
_24 = _4.0;
_4.1 = _24;
_25.2 = _25.3 & _25.3;
_26 = 110_i8 * (-18_i8);
_27.0 = 1484294429227364067_usize | 5_usize;
_27.1 = [_26,_26];
_20.0.0 = [_25.2,_25.3,_25.3,_25.2,_25.3];
_1.2 = _1.4 as i16;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).0 = _27.1;
_32 = Field::<bool>(Variant(_10.fld3, 0), 0);
_25 = (_27.0, _26, _13, _13);
_10.fld4 = _25.0 as i128;
place!(Field::<[u32; 4]>(Variant(_10.fld3, 0), 4)) = [Field::<u32>(Variant(_10.fld3, 0), 3),Field::<u32>(Variant(_10.fld3, 0), 3),Field::<u32>(Variant(_10.fld3, 0), 3),Field::<u32>(Variant(_10.fld3, 0), 3)];
_11 = [_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4];
match Field::<u32>(Variant(_10.fld3, 0), 3) {
0 => bb1,
4161333912 => bb6,
_ => bb2
}
}
bb6 = {
_4.2 = _17.0.1 + _20.0.1;
_34 = (_17.0, _20.1);
_34.0.0 = [_13,_25.3,_25.2,_10.fld4,_10.fld4];
_29.0 = _4.1;
_27 = (_25.0, Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2).0, Field::<bool>(Variant(_10.fld3, 0), 0));
_17.1 = (_34.1.0,);
_31.0 = [_26,_26];
_25.1 = !_26;
_10.fld1 = Adt45 { fld0: _9.fld0 };
_3 = _17.1.0;
_12 = [125484959531363561004806183262555531607_u128];
_25.3 = _25.2;
_19 = _34.0.1 + _20.0.1;
_12 = [178287699744632910838552393237879196500_u128];
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)) = (_31.0, _11);
match Field::<u32>(Variant(_10.fld3, 0), 3) {
0 => bb4,
4161333912 => bb7,
_ => bb2
}
}
bb7 = {
_33 = !_32;
_28 = _31.0;
_37.fld4.3 = _1.3;
_29 = _4;
_16 = _19 as i64;
_37.fld3.0 = _20.1.0;
_20 = (_34.0, _17.1);
_29 = _1;
_39.1.0 = [_37.fld4.3,_7,_9.fld0,_10.fld1.fld0,_9.fld0];
_7 = _10.fld1.fld0;
_10.fld5 = (-2031469052_i32);
_30 = -_15;
_9.fld0 = !_10.fld1.fld0;
_37.fld4.2 = !_20.0.1;
_25.3 = -_25.2;
_24 = _1.1;
_32 = _33;
_10.fld1 = _9;
_31 = Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2);
match _10.fld5 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
340282366920938463463374607429736742404 => bb10,
_ => bb9
}
}
bb8 = {
_1.2 = -_4.2;
_10.fld5 = 9223372036854775807_isize as i32;
place!(Field::<u128>(Variant(_10.fld3, 2), 2)) = 142637018200191945366075084925085156890_u128 >> _9.fld0;
SetDiscriminant(_10.fld3, 2);
_3 = _5;
_3 = [_7,_7,_10.fld1.fld0,_7,_7];
_5 = [_9.fld0,_10.fld1.fld0,_10.fld1.fld0,_9.fld0,_10.fld1.fld0];
place!(Field::<[u32; 4]>(Variant(_10.fld3, 2), 0)) = [1836177383_u32,4134848541_u32,807268878_u32,172896468_u32];
_1.1 = _4.1;
place!(Field::<[u32; 4]>(Variant(_10.fld3, 2), 0)) = RET;
_17.0 = (_11, _4.2);
_10.fld1.fld0 = _4.4 as u16;
_5 = [_7,_7,_7,_7,_9.fld0];
place!(Field::<[i8; 2]>(Variant(_10.fld3, 2), 1)) = [(-65_i8),77_i8];
_17.1 = (_5,);
_10.fld1 = Adt45 { fld0: _7 };
_12 = [16959408943660049509772928250047274590_u128];
place!(Field::<[i8; 2]>(Variant(_10.fld3, 2), 1)) = [(-114_i8),(-72_i8)];
RET = [3689007580_u32,4026182013_u32,3543445362_u32,3105251037_u32];
_6 = _2;
place!(Field::<u128>(Variant(_10.fld3, 2), 2)) = 239_u8 as u128;
Goto(bb3)
}
bb9 = {
_22 = [2369399107_u32,3849039451_u32,2530929945_u32,3653329383_u32];
SetDiscriminant(_10.fld3, 0);
_13 = _10.fld5 as i128;
_10.fld5 = -701933719_i32;
_4 = _1;
_15 = -9223372036854775807_isize;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = !true;
_17.1 = (_5,);
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).1 = _17.0.0;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = false & false;
place!(Field::<[u32; 4]>(Variant(_10.fld3, 0), 4)) = [3927791820_u32,1556415001_u32,735680515_u32,3843566553_u32];
_20.0.0 = [_13,_13,_13,_13,_13];
RET = [2829101919_u32,1075094496_u32,2513386355_u32,2684858014_u32];
_9 = _10.fld1;
_9 = _10.fld1;
_9.fld0 = _10.fld5 as u16;
_20.1.0 = _3;
_20 = (_17.0, _17.1);
_25.3 = _13 - _13;
_1.1 = _4.1;
place!(Field::<u32>(Variant(_10.fld3, 0), 3)) = 4161333912_u32;
match Field::<u32>(Variant(_10.fld3, 0), 3) {
0 => bb1,
4161333912 => bb5,
_ => bb3
}
}
bb10 = {
_4 = _29;
_33 = _32;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).1 = [_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4];
_39.1.0 = _37.fld3.0;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = _32;
_29.2 = _10.fld4 as i16;
_9 = _10.fld1;
_4.1 = _1.0;
match _10.fld5 {
340282366920938463463374607429736742404 => bb11,
_ => bb1
}
}
bb11 = {
_37.fld3.0 = [_9.fld0,_9.fld0,_7,_37.fld4.3,_29.3];
_31 = Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2);
_4.0 = _29.1;
_43.1 = _31.1;
_43 = (_28, _20.0.0);
_39.0 = _20.0;
_32 = Field::<bool>(Variant(_10.fld3, 0), 0);
_4.0 = _1.1;
_1.0 = _29.1;
_1 = (_29.0, _29.0, _20.0.1, _4.3, _4.4);
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).1 = _20.0.0;
_1.1 = _1.0;
_29.0 = _4.0;
_32 = !_33;
_27.0 = _25.0 << _1.3;
_1.4 = _29.4 * _29.4;
_1 = _4;
_44 = (Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2).0, _11);
_4.0 = _24;
_29.1 = _4.0;
_29.0 = _4.0;
_39 = (_34.0, _17.1);
_39.0.1 = _19 << _37.fld4.2;
_37.fld4.1 = _4.0;
_28 = [_25.1,_25.1];
_17 = _39;
Goto(bb12)
}
bb12 = {
_20.0.1 = -_39.0.1;
_27.2 = _27.0 > _27.0;
_1.1 = _24;
_20.0.0 = [_13,_25.2,_13,_10.fld4,_13];
_34 = _39;
_12 = [111489846790178414380097889442716628554_u128];
_20.1.0 = _39.1.0;
_1.1 = _4.1;
_46.1 = _44.1;
place!(Field::<i32>(Variant(_10.fld3, 0), 5)) = _10.fld5;
_37.fld4.0 = _29.0;
_34.1.0 = [_29.3,_4.3,_29.3,_37.fld4.3,_9.fld0];
_37.fld3.0 = [_10.fld1.fld0,_1.3,_4.3,_10.fld1.fld0,_7];
_48 = !_27.2;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).0 = [_26,_25.1];
_43.0 = _27.1;
_39 = (_34.0, _34.1);
_37.fld3 = (_5,);
_24 = _37.fld4.0;
place!(Field::<[u32; 4]>(Variant(_10.fld3, 0), 4)) = [Field::<u32>(Variant(_10.fld3, 0), 3),Field::<u32>(Variant(_10.fld3, 0), 3),Field::<u32>(Variant(_10.fld3, 0), 3),Field::<u32>(Variant(_10.fld3, 0), 3)];
match Field::<i32>(Variant(_10.fld3, 0), 5) {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
340282366920938463463374607429736742404 => bb18,
_ => bb17
}
}
bb13 = {
_22 = [2369399107_u32,3849039451_u32,2530929945_u32,3653329383_u32];
SetDiscriminant(_10.fld3, 0);
_13 = _10.fld5 as i128;
_10.fld5 = -701933719_i32;
_4 = _1;
_15 = -9223372036854775807_isize;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = !true;
_17.1 = (_5,);
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).1 = _17.0.0;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = false & false;
place!(Field::<[u32; 4]>(Variant(_10.fld3, 0), 4)) = [3927791820_u32,1556415001_u32,735680515_u32,3843566553_u32];
_20.0.0 = [_13,_13,_13,_13,_13];
RET = [2829101919_u32,1075094496_u32,2513386355_u32,2684858014_u32];
_9 = _10.fld1;
_9 = _10.fld1;
_9.fld0 = _10.fld5 as u16;
_20.1.0 = _3;
_20 = (_17.0, _17.1);
_25.3 = _13 - _13;
_1.1 = _4.1;
place!(Field::<u32>(Variant(_10.fld3, 0), 3)) = 4161333912_u32;
match Field::<u32>(Variant(_10.fld3, 0), 3) {
0 => bb1,
4161333912 => bb5,
_ => bb3
}
}
bb14 = {
_4 = _29;
_33 = _32;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).1 = [_10.fld4,_10.fld4,_10.fld4,_10.fld4,_10.fld4];
_39.1.0 = _37.fld3.0;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = _32;
_29.2 = _10.fld4 as i16;
_9 = _10.fld1;
_4.1 = _1.0;
match _10.fld5 {
340282366920938463463374607429736742404 => bb11,
_ => bb1
}
}
bb15 = {
_22 = [2369399107_u32,3849039451_u32,2530929945_u32,3653329383_u32];
SetDiscriminant(_10.fld3, 0);
_13 = _10.fld5 as i128;
_10.fld5 = -701933719_i32;
_4 = _1;
_15 = -9223372036854775807_isize;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = !true;
_17.1 = (_5,);
place!(Field::<([i8; 2], [i128; 5])>(Variant(_10.fld3, 0), 2)).1 = _17.0.0;
place!(Field::<bool>(Variant(_10.fld3, 0), 0)) = false & false;
place!(Field::<[u32; 4]>(Variant(_10.fld3, 0), 4)) = [3927791820_u32,1556415001_u32,735680515_u32,3843566553_u32];
_20.0.0 = [_13,_13,_13,_13,_13];
RET = [2829101919_u32,1075094496_u32,2513386355_u32,2684858014_u32];
_9 = _10.fld1;
_9 = _10.fld1;
_9.fld0 = _10.fld5 as u16;
_20.1.0 = _3;
_20 = (_17.0, _17.1);
_25.3 = _13 - _13;
_1.1 = _4.1;
place!(Field::<u32>(Variant(_10.fld3, 0), 3)) = 4161333912_u32;
match Field::<u32>(Variant(_10.fld3, 0), 3) {
0 => bb1,
4161333912 => bb5,
_ => bb3
}
}
bb16 = {
_1.0 = _4.0;
_5 = _3;
_1.3 = _9.fld0;
_8 = [270737923_u32,1652209351_u32,3160102159_u32,1917445387_u32];
_8 = [2888312272_u32,739642218_u32,2023801615_u32,1977483944_u32];
_15 = (-9223372036854775808_isize) >> _10.fld1.fld0;
Call(_12 = core::intrinsics::transmute(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb17 = {
_9 = Adt45 { fld0: _7 };
_4.3 = _1.4 as u16;
_4.0 = _1.0;
_1.2 = 239_u8 as i16;
_1.4 = -_4.4;
_11 = [98314875393457993470086847327850968841_i128,(-139360995327374222873499508344054362030_i128),(-14412810712312006017347765063762954136_i128),(-27591190608919557561243615602610743995_i128),130035425193418725005951446203539647629_i128];
_1.3 = 47138270321401746829855657977727593055_i128 as u16;
_10.fld1.fld0 = _9.fld0;
_5 = [_7,_10.fld1.fld0,_9.fld0,_9.fld0,_10.fld1.fld0];
RET = [3086879248_u32,4140330265_u32,2247980287_u32,540818659_u32];
Call(_10.fld3 = fn13(_2, _10.fld1.fld0, _10.fld1, _7, _10.fld1, _2, _7, _4.2, _5, _7, _9, _10.fld1.fld0, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_20.1.0 = _5;
Goto(bb19)
}
bb19 = {
Call(_49 = dump_var(12_usize, 22_usize, Move(_22), 31_usize, Move(_31), 7_usize, Move(_7), 28_usize, Move(_28)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_49 = dump_var(12_usize, 44_usize, Move(_44), 30_usize, Move(_30), 20_usize, Move(_20), 17_usize, Move(_17)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_49 = dump_var(12_usize, 39_usize, Move(_39), 15_usize, Move(_15), 25_usize, Move(_25), 43_usize, Move(_43)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_49 = dump_var(12_usize, 11_usize, Move(_11), 50_usize, _50, 50_usize, _50, 50_usize, _50), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *const (char, char, i16, u16, f64),mut _2: u16,mut _3: Adt45,mut _4: u16,mut _5: Adt45,mut _6: *const (char, char, i16, u16, f64),mut _7: u16,mut _8: i16,mut _9: [u16; 5],mut _10: u16,mut _11: Adt45,mut _12: u16,mut _13: *const (char, char, i16, u16, f64)) -> Adt51 {
mir! {
type RET = Adt51;
let _14: Adt54;
let _15: u64;
let _16: Adt41;
let _17: Adt49;
let _18: isize;
let _19: Adt53;
let _20: Adt56;
let _21: [isize; 4];
let _22: isize;
let _23: *mut f64;
let _24: Adt45;
let _25: [u16; 5];
let _26: f64;
let _27: u64;
let _28: i128;
let _29: u16;
let _30: isize;
let _31: u64;
let _32: Adt47;
let _33: Adt42;
let _34: usize;
let _35: u128;
let _36: isize;
let _37: *mut usize;
let _38: f64;
let _39: Adt45;
let _40: (usize, i8, i128, i128);
let _41: *mut f64;
let _42: i16;
let _43: Adt54;
let _44: (char, char, i16, u16, f64);
let _45: bool;
let _46: f64;
let _47: Adt48;
let _48: isize;
let _49: f32;
let _50: isize;
let _51: (char, char, i16, u16, f64);
let _52: [u128; 3];
let _53: Adt43;
let _54: Adt48;
let _55: [u16; 5];
let _56: Adt50;
let _57: f32;
let _58: u128;
let _59: Adt52;
let _60: (usize, i8, i128, i128);
let _61: [u32; 4];
let _62: Adt44;
let _63: isize;
let _64: u32;
let _65: u128;
let _66: (usize, i8, i128, i128);
let _67: Adt50;
let _68: [i8; 2];
let _69: f64;
let _70: isize;
let _71: Adt56;
let _72: [u32; 4];
let _73: (char, char, i16, u16, f64);
let _74: &'static isize;
let _75: (usize, [i8; 2], bool);
let _76: bool;
let _77: bool;
let _78: *const *const &'static isize;
let _79: isize;
let _80: f64;
let _81: Adt43;
let _82: ();
let _83: ();
{
_5 = Adt45 { fld0: _12 };
_2 = true as u16;
_4 = 6907996878630579646_u64 as u16;
_6 = _1;
_11.fld0 = _12 + _3.fld0;
_7 = !_10;
_12 = _3.fld0 * _3.fld0;
_12 = _7;
_8 = (-30851_i16) >> _11.fld0;
_3 = Adt45 { fld0: _12 };
_11 = Adt45 { fld0: _5.fld0 };
_2 = _10;
Goto(bb1)
}
bb1 = {
_5 = _11;
_5 = Adt45 { fld0: _3.fld0 };
_12 = _5.fld0 + _3.fld0;
_3.fld0 = _7 ^ _2;
_5.fld0 = _10 & _2;
_2 = true as u16;
_1 = _13;
_11 = Adt45 { fld0: _5.fld0 };
_6 = _13;
_3.fld0 = _7;
_2 = 9223372036854775807_isize as u16;
_9 = [_3.fld0,_10,_7,_10,_7];
_6 = _13;
_2 = _10;
_10 = !_3.fld0;
_11 = _5;
_3 = _5;
_13 = _1;
_4 = !_3.fld0;
_12 = _7 - _2;
_11.fld0 = _5.fld0 << _4;
_5.fld0 = !_4;
_16.fld4.1 = '\u{2e3b6}';
_16.fld4.2 = -_8;
_6 = core::ptr::addr_of!(_16.fld4);
_2 = _3.fld0 + _10;
_16.fld0 = _5.fld0 != _5.fld0;
_11 = Adt45 { fld0: _10 };
Goto(bb2)
}
bb2 = {
_16.fld4.2 = 1312680802_u32 as i16;
_16.fld3.0 = [_12,_3.fld0,_4,_2,_2];
_16.fld4.0 = (*_6).1;
_16.fld4.3 = !_10;
_16.fld0 = !false;
_1 = _13;
_16.fld4.3 = _12 ^ _5.fld0;
_4 = (-1865757816_i32) as u16;
_8 = (*_6).2 ^ (*_6).2;
_8 = _16.fld4.2 - (*_6).2;
_16.fld4.1 = (*_6).0;
_10 = (*_6).3 * _2;
_11.fld0 = _16.fld4.3 >> _5.fld0;
_16.fld4.4 = 11656308865437106066_u64 as f64;
_9 = _16.fld3.0;
_5.fld0 = (*_6).3;
_16.fld3 = (_9,);
_16.fld4.0 = (*_6).1;
_16.fld1 = (*_6).1;
_16.fld3.0 = [(*_6).3,_12,(*_6).3,_7,(*_6).3];
_8 = (*_6).2 & (*_6).2;
_6 = _1;
_10 = _16.fld4.3 * _3.fld0;
_22 = (-125_isize);
Goto(bb3)
}
bb3 = {
_16.fld4.0 = _16.fld4.1;
_11.fld0 = _3.fld0 >> _16.fld4.3;
_15 = 3347324127_u32 as u64;
_10 = 118_u8 as u16;
_16.fld4.3 = _3.fld0;
_11.fld0 = !_3.fld0;
_24 = _3;
_16.fld3 = (_9,);
match _22 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211331 => bb9,
_ => bb8
}
}
bb4 = {
_16.fld4.2 = 1312680802_u32 as i16;
_16.fld3.0 = [_12,_3.fld0,_4,_2,_2];
_16.fld4.0 = (*_6).1;
_16.fld4.3 = !_10;
_16.fld0 = !false;
_1 = _13;
_16.fld4.3 = _12 ^ _5.fld0;
_4 = (-1865757816_i32) as u16;
_8 = (*_6).2 ^ (*_6).2;
_8 = _16.fld4.2 - (*_6).2;
_16.fld4.1 = (*_6).0;
_10 = (*_6).3 * _2;
_11.fld0 = _16.fld4.3 >> _5.fld0;
_16.fld4.4 = 11656308865437106066_u64 as f64;
_9 = _16.fld3.0;
_5.fld0 = (*_6).3;
_16.fld3 = (_9,);
_16.fld4.0 = (*_6).1;
_16.fld1 = (*_6).1;
_16.fld3.0 = [(*_6).3,_12,(*_6).3,_7,(*_6).3];
_8 = (*_6).2 & (*_6).2;
_6 = _1;
_10 = _16.fld4.3 * _3.fld0;
_22 = (-125_isize);
Goto(bb3)
}
bb5 = {
_5 = _11;
_5 = Adt45 { fld0: _3.fld0 };
_12 = _5.fld0 + _3.fld0;
_3.fld0 = _7 ^ _2;
_5.fld0 = _10 & _2;
_2 = true as u16;
_1 = _13;
_11 = Adt45 { fld0: _5.fld0 };
_6 = _13;
_3.fld0 = _7;
_2 = 9223372036854775807_isize as u16;
_9 = [_3.fld0,_10,_7,_10,_7];
_6 = _13;
_2 = _10;
_10 = !_3.fld0;
_11 = _5;
_3 = _5;
_13 = _1;
_4 = !_3.fld0;
_12 = _7 - _2;
_11.fld0 = _5.fld0 << _4;
_5.fld0 = !_4;
_16.fld4.1 = '\u{2e3b6}';
_16.fld4.2 = -_8;
_6 = core::ptr::addr_of!(_16.fld4);
_2 = _3.fld0 + _10;
_16.fld0 = _5.fld0 != _5.fld0;
_11 = Adt45 { fld0: _10 };
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
_3.fld0 = _24.fld0 ^ _5.fld0;
_15 = 219_u8 as u64;
Call(_22 = fn14(_13, _11.fld0, _16.fld3.0, _16.fld4.3, _5, _24), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16.fld4.4 = _15 as f64;
_21 = [_22,_22,_22,_22];
_16.fld4.1 = _16.fld4.0;
_26 = _16.fld4.4 * _16.fld4.4;
_16.fld4.4 = _26;
_16.fld4.3 = !_24.fld0;
_21 = [_22,_22,_22,_22];
_9 = _16.fld3.0;
Call(_8 = fn15(_16.fld3, _6, _9), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_24.fld0 = _2;
_11 = Adt45 { fld0: _16.fld4.3 };
_5 = _11;
_15 = 8537281965706770852_u64 << _5.fld0;
_27 = 35204111325590353800983986327115638054_i128 as u64;
_25 = _16.fld3.0;
_9 = [_7,_7,_12,_12,_16.fld4.3];
_11.fld0 = _5.fld0 >> _15;
_12 = _11.fld0 + _2;
_30 = _22 << _2;
_13 = core::ptr::addr_of!(_16.fld4);
_16.fld4.2 = -_8;
_20 = Adt56::Variant0 { fld0: _21 };
_7 = 203_u8 as u16;
_7 = !_11.fld0;
_24 = Adt45 { fld0: _3.fld0 };
_20 = Adt56::Variant0 { fld0: _21 };
_11 = Adt45 { fld0: _7 };
_9 = [_5.fld0,_3.fld0,(*_13).3,_5.fld0,_24.fld0];
_22 = _30 * _30;
_16.fld4.4 = -_26;
_21 = [_30,_22,_30,_22];
Call(_16.fld2 = fn16(_3.fld0, _30, _11.fld0, _11.fld0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<[isize; 4]>(Variant(_20, 0), 0)) = _21;
_26 = 2352363878_u32 as f64;
_7 = _15 as u16;
_16.fld4 = (_16.fld1, _16.fld1, _8, _11.fld0, _16.fld2);
_5.fld0 = !_3.fld0;
_16.fld0 = false;
_16.fld4.1 = _16.fld4.0;
_26 = _16.fld4.4 + _16.fld2;
SetDiscriminant(_20, 2);
_2 = !_7;
_15 = 5_usize as u64;
Call(place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0 = core::intrinsics::bswap(18331448745197333732_usize), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_29 = _3.fld0 - _11.fld0;
_30 = _26 as isize;
place!(Field::<usize>(Variant(_20, 2), 1)) = Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0 | Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0 = (*_13).2 as usize;
_16.fld1 = _16.fld4.0;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).2 = _16.fld4.4 < _16.fld4.4;
_28 = -62676998500347445774901029696120823623_i128;
place!(Field::<*mut (char, char, i16, u16, f64)>(Variant(_20, 2), 3)) = core::ptr::addr_of_mut!(_16.fld4);
place!(Field::<*mut (char, char, i16, u16, f64)>(Variant(_20, 2), 3)) = core::ptr::addr_of_mut!((*_13));
place!(Field::<u32>(Variant(_20, 2), 0)) = 90813027656502606572927470201130025251_u128 as u32;
_34 = Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0 + Field::<usize>(Variant(_20, 2), 1);
place!(Field::<usize>(Variant(_20, 2), 1)) = _16.fld4.3 as usize;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).2 = !_16.fld0;
place!(Field::<u32>(Variant(_20, 2), 0)) = 1398557737_u32 >> _2;
_16.fld4 = (_16.fld1, _16.fld1, _8, _5.fld0, _26);
place!(Field::<f64>(Variant(_20, 2), 7)) = _16.fld4.4;
_8 = (*_13).2 - _16.fld4.2;
_16.fld4.1 = _16.fld1;
Goto(bb14)
}
bb14 = {
_37 = core::ptr::addr_of_mut!(place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0);
_38 = _16.fld2;
_24.fld0 = 282895438594664623065798556786669331077_u128 as u16;
_3.fld0 = _2 ^ (*_13).3;
_24.fld0 = _16.fld4.3 << (*_13).3;
_7 = _12;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).1 = [(-63_i8),(-31_i8)];
place!(Field::<usize>(Variant(_20, 2), 1)) = (*_37) << _29;
_1 = _13;
_16.fld4.0 = (*_13).1;
_26 = (*_13).4 * Field::<f64>(Variant(_20, 2), 7);
_11 = _24;
_18 = !_22;
_16.fld2 = _16.fld4.4 - (*_13).4;
_8 = -_16.fld4.2;
_40.3 = _28;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).1 = [79_i8,4_i8];
_31 = _15;
_40.0 = Field::<usize>(Variant(_20, 2), 1);
_16.fld2 = -_26;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0 = _40.0;
_10 = !_5.fld0;
_16.fld3.0 = [_10,_16.fld4.3,_24.fld0,(*_13).3,(*_13).3];
_34 = Field::<usize>(Variant(_20, 2), 1) + Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0;
Goto(bb15)
}
bb15 = {
_16.fld1 = (*_13).0;
_7 = Field::<usize>(Variant(_20, 2), 1) as u16;
_20 = Adt56::Variant0 { fld0: _21 };
SetDiscriminant(_20, 0);
_24.fld0 = !_2;
_39 = _24;
_18 = _15 as isize;
_40.1 = (-1561207778737887837_i64) as i8;
Call(_40.0 = core::intrinsics::transmute(_22), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_44.1 = _16.fld4.0;
_47.fld3 = (_25,);
_16.fld1 = (*_13).0;
_16.fld4.4 = (*_13).3 as f64;
_44.2 = -(*_13).2;
_44.0 = _16.fld1;
_46 = (*_13).4;
_44.1 = (*_13).0;
_11.fld0 = 3871257806983029400_i64 as u16;
_24 = Adt45 { fld0: _7 };
_11 = _39;
_40 = (_34, 116_i8, _28, _28);
_15 = !_31;
_28 = !_40.3;
_25 = [(*_13).3,_29,_24.fld0,_12,_3.fld0];
_24.fld0 = _3.fld0 * (*_13).3;
_39 = Adt45 { fld0: _12 };
_47.fld1 = [_22,_22,_30,_30];
_50 = (*_13).1 as isize;
place!(Field::<[isize; 4]>(Variant(_20, 0), 0)) = [_30,_22,_30,_22];
_49 = _40.1 as f32;
_22 = _49 as isize;
_16.fld1 = _44.0;
_34 = _40.0 & _40.0;
match _40.1 {
116 => bb17,
_ => bb14
}
}
bb17 = {
_27 = _31;
_1 = _6;
_54.fld3 = (_16.fld3.0,);
_26 = _46;
_30 = -_22;
_30 = -_22;
_35 = 135844005229858286093590472607814352782_u128 << _10;
_47.fld3 = (_25,);
_40.0 = _34 ^ _34;
_16.fld4.1 = _44.0;
_40 = (_34, 100_i8, _28, _28);
_18 = _22;
_36 = _30;
_30 = _22 >> _11.fld0;
_37 = core::ptr::addr_of_mut!(_34);
_42 = (*_13).2 - _16.fld4.2;
_1 = _13;
_16.fld4 = (_44.0, _16.fld1, _44.2, _11.fld0, _16.fld2);
_48 = !_22;
Goto(bb18)
}
bb18 = {
_48 = _15 as isize;
_47.fld0 = -_28;
_61 = [3405788670_u32,2736544583_u32,3569657062_u32,2998678962_u32];
_40.0 = (*_37);
_16.fld3 = (_9,);
SetDiscriminant(_20, 2);
_44.3 = !_12;
_5 = _24;
_60.3 = _44.0 as i128;
_40.0 = !_34;
_47 = Adt48 { fld0: _60.3,fld1: _21,fld2: 113_u8,fld3: _54.fld3 };
_45 = !_16.fld0;
_7 = !(*_13).3;
_45 = !_16.fld0;
_21 = [_22,_22,_30,_18];
_18 = _36 ^ _36;
_42 = _16.fld0 as i16;
_13 = core::ptr::addr_of!((*_13));
Call(_42 = core::intrinsics::transmute(_10), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_5.fld0 = _44.3;
_28 = _40.2 * _40.3;
_40.1 = -(-120_i8);
_44.4 = -_16.fld2;
_42 = _31 as i16;
_1 = core::ptr::addr_of!(_51);
_29 = !_5.fld0;
_29 = 508552287_i32 as u16;
_51.3 = _44.3 - (*_13).3;
_47 = Adt48 { fld0: _40.3,fld1: _21,fld2: 56_u8,fld3: _54.fld3 };
_34 = _40.0 >> (*_1).3;
_1 = core::ptr::addr_of!((*_13));
_16.fld4.3 = 23230499_u32 as u16;
_66.2 = (*_13).1 as i128;
_47.fld3 = _54.fld3;
_51.3 = _5.fld0;
_51 = (*_1);
Goto(bb20)
}
bb20 = {
place!(Field::<usize>(Variant(_20, 2), 1)) = _40.0 & _40.0;
_53 = Adt43::Variant0 { fld0: 891330878_u32,fld1: _61,fld2: (*_13).4 };
_23 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_53, 0), 2)));
_60.1 = !_40.1;
_56 = Adt50::Variant1 { fld0: _30 };
_58 = (*_1).4 as u128;
_66.0 = Field::<usize>(Variant(_20, 2), 1);
_60 = (_66.0, _40.1, _40.3, _28);
match _47.fld2 {
0 => bb21,
1 => bb22,
2 => bb23,
3 => bb24,
56 => bb26,
_ => bb25
}
}
bb21 = {
Return()
}
bb22 = {
_29 = _3.fld0 - _11.fld0;
_30 = _26 as isize;
place!(Field::<usize>(Variant(_20, 2), 1)) = Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0 | Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0 = (*_13).2 as usize;
_16.fld1 = _16.fld4.0;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).2 = _16.fld4.4 < _16.fld4.4;
_28 = -62676998500347445774901029696120823623_i128;
place!(Field::<*mut (char, char, i16, u16, f64)>(Variant(_20, 2), 3)) = core::ptr::addr_of_mut!(_16.fld4);
place!(Field::<*mut (char, char, i16, u16, f64)>(Variant(_20, 2), 3)) = core::ptr::addr_of_mut!((*_13));
place!(Field::<u32>(Variant(_20, 2), 0)) = 90813027656502606572927470201130025251_u128 as u32;
_34 = Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0 + Field::<usize>(Variant(_20, 2), 1);
place!(Field::<usize>(Variant(_20, 2), 1)) = _16.fld4.3 as usize;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).2 = !_16.fld0;
place!(Field::<u32>(Variant(_20, 2), 0)) = 1398557737_u32 >> _2;
_16.fld4 = (_16.fld1, _16.fld1, _8, _5.fld0, _26);
place!(Field::<f64>(Variant(_20, 2), 7)) = _16.fld4.4;
_8 = (*_13).2 - _16.fld4.2;
_16.fld4.1 = _16.fld1;
Goto(bb14)
}
bb23 = {
_5 = _11;
_5 = Adt45 { fld0: _3.fld0 };
_12 = _5.fld0 + _3.fld0;
_3.fld0 = _7 ^ _2;
_5.fld0 = _10 & _2;
_2 = true as u16;
_1 = _13;
_11 = Adt45 { fld0: _5.fld0 };
_6 = _13;
_3.fld0 = _7;
_2 = 9223372036854775807_isize as u16;
_9 = [_3.fld0,_10,_7,_10,_7];
_6 = _13;
_2 = _10;
_10 = !_3.fld0;
_11 = _5;
_3 = _5;
_13 = _1;
_4 = !_3.fld0;
_12 = _7 - _2;
_11.fld0 = _5.fld0 << _4;
_5.fld0 = !_4;
_16.fld4.1 = '\u{2e3b6}';
_16.fld4.2 = -_8;
_6 = core::ptr::addr_of!(_16.fld4);
_2 = _3.fld0 + _10;
_16.fld0 = _5.fld0 != _5.fld0;
_11 = Adt45 { fld0: _10 };
Goto(bb2)
}
bb24 = {
_44.1 = _16.fld4.0;
_47.fld3 = (_25,);
_16.fld1 = (*_13).0;
_16.fld4.4 = (*_13).3 as f64;
_44.2 = -(*_13).2;
_44.0 = _16.fld1;
_46 = (*_13).4;
_44.1 = (*_13).0;
_11.fld0 = 3871257806983029400_i64 as u16;
_24 = Adt45 { fld0: _7 };
_11 = _39;
_40 = (_34, 116_i8, _28, _28);
_15 = !_31;
_28 = !_40.3;
_25 = [(*_13).3,_29,_24.fld0,_12,_3.fld0];
_24.fld0 = _3.fld0 * (*_13).3;
_39 = Adt45 { fld0: _12 };
_47.fld1 = [_22,_22,_30,_30];
_50 = (*_13).1 as isize;
place!(Field::<[isize; 4]>(Variant(_20, 0), 0)) = [_30,_22,_30,_22];
_49 = _40.1 as f32;
_22 = _49 as isize;
_16.fld1 = _44.0;
_34 = _40.0 & _40.0;
match _40.1 {
116 => bb17,
_ => bb14
}
}
bb25 = {
_37 = core::ptr::addr_of_mut!(place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0);
_38 = _16.fld2;
_24.fld0 = 282895438594664623065798556786669331077_u128 as u16;
_3.fld0 = _2 ^ (*_13).3;
_24.fld0 = _16.fld4.3 << (*_13).3;
_7 = _12;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).1 = [(-63_i8),(-31_i8)];
place!(Field::<usize>(Variant(_20, 2), 1)) = (*_37) << _29;
_1 = _13;
_16.fld4.0 = (*_13).1;
_26 = (*_13).4 * Field::<f64>(Variant(_20, 2), 7);
_11 = _24;
_18 = !_22;
_16.fld2 = _16.fld4.4 - (*_13).4;
_8 = -_16.fld4.2;
_40.3 = _28;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).1 = [79_i8,4_i8];
_31 = _15;
_40.0 = Field::<usize>(Variant(_20, 2), 1);
_16.fld2 = -_26;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0 = _40.0;
_10 = !_5.fld0;
_16.fld3.0 = [_10,_16.fld4.3,_24.fld0,(*_13).3,(*_13).3];
_34 = Field::<usize>(Variant(_20, 2), 1) + Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0;
Goto(bb15)
}
bb26 = {
_16.fld4.2 = _8 << _39.fld0;
match _47.fld2 {
0 => bb19,
1 => bb7,
2 => bb25,
3 => bb4,
4 => bb9,
56 => bb28,
_ => bb27
}
}
bb27 = {
_27 = _31;
_1 = _6;
_54.fld3 = (_16.fld3.0,);
_26 = _46;
_30 = -_22;
_30 = -_22;
_35 = 135844005229858286093590472607814352782_u128 << _10;
_47.fld3 = (_25,);
_40.0 = _34 ^ _34;
_16.fld4.1 = _44.0;
_40 = (_34, 100_i8, _28, _28);
_18 = _22;
_36 = _30;
_30 = _22 >> _11.fld0;
_37 = core::ptr::addr_of_mut!(_34);
_42 = (*_13).2 - _16.fld4.2;
_1 = _13;
_16.fld4 = (_44.0, _16.fld1, _44.2, _11.fld0, _16.fld2);
_48 = !_22;
Goto(bb18)
}
bb28 = {
place!(Field::<f64>(Variant(_20, 2), 7)) = -_46;
_16 = Adt41 { fld0: _45,fld1: _44.0,fld2: _46,fld3: _54.fld3,fld4: _44 };
_12 = _10;
_55 = [_12,_16.fld4.3,_2,_2,_11.fld0];
_51.3 = !_5.fld0;
_64 = !1414880456_u32;
_12 = _47.fld2 as u16;
_60.3 = _16.fld4.1 as i128;
place!(Field::<*mut (char, char, i16, u16, f64)>(Variant(_20, 2), 3)) = core::ptr::addr_of_mut!((*_13));
_44 = ((*_13).1, _16.fld4.1, (*_13).2, _7, Field::<f64>(Variant(_53, 0), 2));
_18 = _22 | _36;
_23 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_53, 0), 2)));
_69 = Field::<f64>(Variant(_53, 0), 2) + _26;
_54.fld3 = (_25,);
place!(Field::<*mut (char, char, i16, u16, f64)>(Variant(_20, 2), 3)) = core::ptr::addr_of_mut!((*_13));
_60.3 = _45 as i128;
_68 = [_60.1,_60.1];
_41 = _23;
_64 = 192409365_u32;
_37 = core::ptr::addr_of_mut!((*_37));
SetDiscriminant(_56, 1);
place!(Field::<u32>(Variant(_53, 0), 0)) = !_64;
_51.0 = (*_13).1;
_51.1 = _16.fld4.0;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)) = (_40.0, _68, _16.fld0);
_4 = _40.1 as u16;
_66.3 = !_47.fld0;
_48 = (*_13).3 as isize;
_16.fld4.0 = (*_13).1;
_34 = 3097739582717126447_i64 as usize;
match _47.fld2 {
0 => bb29,
1 => bb30,
2 => bb31,
3 => bb32,
4 => bb33,
5 => bb34,
6 => bb35,
56 => bb37,
_ => bb36
}
}
bb29 = {
_16.fld4.0 = _16.fld4.1;
_11.fld0 = _3.fld0 >> _16.fld4.3;
_15 = 3347324127_u32 as u64;
_10 = 118_u8 as u16;
_16.fld4.3 = _3.fld0;
_11.fld0 = !_3.fld0;
_24 = _3;
_16.fld3 = (_9,);
match _22 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211331 => bb9,
_ => bb8
}
}
bb30 = {
_16.fld4.2 = _8 << _39.fld0;
match _47.fld2 {
0 => bb19,
1 => bb7,
2 => bb25,
3 => bb4,
4 => bb9,
56 => bb28,
_ => bb27
}
}
bb31 = {
_29 = _3.fld0 - _11.fld0;
_30 = _26 as isize;
place!(Field::<usize>(Variant(_20, 2), 1)) = Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0 | Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0 = (*_13).2 as usize;
_16.fld1 = _16.fld4.0;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).2 = _16.fld4.4 < _16.fld4.4;
_28 = -62676998500347445774901029696120823623_i128;
place!(Field::<*mut (char, char, i16, u16, f64)>(Variant(_20, 2), 3)) = core::ptr::addr_of_mut!(_16.fld4);
place!(Field::<*mut (char, char, i16, u16, f64)>(Variant(_20, 2), 3)) = core::ptr::addr_of_mut!((*_13));
place!(Field::<u32>(Variant(_20, 2), 0)) = 90813027656502606572927470201130025251_u128 as u32;
_34 = Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0 + Field::<usize>(Variant(_20, 2), 1);
place!(Field::<usize>(Variant(_20, 2), 1)) = _16.fld4.3 as usize;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).2 = !_16.fld0;
place!(Field::<u32>(Variant(_20, 2), 0)) = 1398557737_u32 >> _2;
_16.fld4 = (_16.fld1, _16.fld1, _8, _5.fld0, _26);
place!(Field::<f64>(Variant(_20, 2), 7)) = _16.fld4.4;
_8 = (*_13).2 - _16.fld4.2;
_16.fld4.1 = _16.fld1;
Goto(bb14)
}
bb32 = {
_37 = core::ptr::addr_of_mut!(place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0);
_38 = _16.fld2;
_24.fld0 = 282895438594664623065798556786669331077_u128 as u16;
_3.fld0 = _2 ^ (*_13).3;
_24.fld0 = _16.fld4.3 << (*_13).3;
_7 = _12;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).1 = [(-63_i8),(-31_i8)];
place!(Field::<usize>(Variant(_20, 2), 1)) = (*_37) << _29;
_1 = _13;
_16.fld4.0 = (*_13).1;
_26 = (*_13).4 * Field::<f64>(Variant(_20, 2), 7);
_11 = _24;
_18 = !_22;
_16.fld2 = _16.fld4.4 - (*_13).4;
_8 = -_16.fld4.2;
_40.3 = _28;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).1 = [79_i8,4_i8];
_31 = _15;
_40.0 = Field::<usize>(Variant(_20, 2), 1);
_16.fld2 = -_26;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0 = _40.0;
_10 = !_5.fld0;
_16.fld3.0 = [_10,_16.fld4.3,_24.fld0,(*_13).3,(*_13).3];
_34 = Field::<usize>(Variant(_20, 2), 1) + Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0;
Goto(bb15)
}
bb33 = {
_5 = _11;
_5 = Adt45 { fld0: _3.fld0 };
_12 = _5.fld0 + _3.fld0;
_3.fld0 = _7 ^ _2;
_5.fld0 = _10 & _2;
_2 = true as u16;
_1 = _13;
_11 = Adt45 { fld0: _5.fld0 };
_6 = _13;
_3.fld0 = _7;
_2 = 9223372036854775807_isize as u16;
_9 = [_3.fld0,_10,_7,_10,_7];
_6 = _13;
_2 = _10;
_10 = !_3.fld0;
_11 = _5;
_3 = _5;
_13 = _1;
_4 = !_3.fld0;
_12 = _7 - _2;
_11.fld0 = _5.fld0 << _4;
_5.fld0 = !_4;
_16.fld4.1 = '\u{2e3b6}';
_16.fld4.2 = -_8;
_6 = core::ptr::addr_of!(_16.fld4);
_2 = _3.fld0 + _10;
_16.fld0 = _5.fld0 != _5.fld0;
_11 = Adt45 { fld0: _10 };
Goto(bb2)
}
bb34 = {
Return()
}
bb35 = {
_16.fld1 = (*_13).0;
_7 = Field::<usize>(Variant(_20, 2), 1) as u16;
_20 = Adt56::Variant0 { fld0: _21 };
SetDiscriminant(_20, 0);
_24.fld0 = !_2;
_39 = _24;
_18 = _15 as isize;
_40.1 = (-1561207778737887837_i64) as i8;
Call(_40.0 = core::intrinsics::transmute(_22), ReturnTo(bb16), UnwindUnreachable())
}
bb36 = {
_16.fld4.2 = 1312680802_u32 as i16;
_16.fld3.0 = [_12,_3.fld0,_4,_2,_2];
_16.fld4.0 = (*_6).1;
_16.fld4.3 = !_10;
_16.fld0 = !false;
_1 = _13;
_16.fld4.3 = _12 ^ _5.fld0;
_4 = (-1865757816_i32) as u16;
_8 = (*_6).2 ^ (*_6).2;
_8 = _16.fld4.2 - (*_6).2;
_16.fld4.1 = (*_6).0;
_10 = (*_6).3 * _2;
_11.fld0 = _16.fld4.3 >> _5.fld0;
_16.fld4.4 = 11656308865437106066_u64 as f64;
_9 = _16.fld3.0;
_5.fld0 = (*_6).3;
_16.fld3 = (_9,);
_16.fld4.0 = (*_6).1;
_16.fld1 = (*_6).1;
_16.fld3.0 = [(*_6).3,_12,(*_6).3,_7,(*_6).3];
_8 = (*_6).2 & (*_6).2;
_6 = _1;
_10 = _16.fld4.3 * _3.fld0;
_22 = (-125_isize);
Goto(bb3)
}
bb37 = {
_11.fld0 = Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).2 as u16;
match _47.fld2 {
0 => bb38,
1 => bb39,
56 => bb41,
_ => bb40
}
}
bb38 = {
_37 = core::ptr::addr_of_mut!(place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0);
_38 = _16.fld2;
_24.fld0 = 282895438594664623065798556786669331077_u128 as u16;
_3.fld0 = _2 ^ (*_13).3;
_24.fld0 = _16.fld4.3 << (*_13).3;
_7 = _12;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).1 = [(-63_i8),(-31_i8)];
place!(Field::<usize>(Variant(_20, 2), 1)) = (*_37) << _29;
_1 = _13;
_16.fld4.0 = (*_13).1;
_26 = (*_13).4 * Field::<f64>(Variant(_20, 2), 7);
_11 = _24;
_18 = !_22;
_16.fld2 = _16.fld4.4 - (*_13).4;
_8 = -_16.fld4.2;
_40.3 = _28;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).1 = [79_i8,4_i8];
_31 = _15;
_40.0 = Field::<usize>(Variant(_20, 2), 1);
_16.fld2 = -_26;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).0 = _40.0;
_10 = !_5.fld0;
_16.fld3.0 = [_10,_16.fld4.3,_24.fld0,(*_13).3,(*_13).3];
_34 = Field::<usize>(Variant(_20, 2), 1) + Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4).0;
Goto(bb15)
}
bb39 = {
Return()
}
bb40 = {
_48 = _15 as isize;
_47.fld0 = -_28;
_61 = [3405788670_u32,2736544583_u32,3569657062_u32,2998678962_u32];
_40.0 = (*_37);
_16.fld3 = (_9,);
SetDiscriminant(_20, 2);
_44.3 = !_12;
_5 = _24;
_60.3 = _44.0 as i128;
_40.0 = !_34;
_47 = Adt48 { fld0: _60.3,fld1: _21,fld2: 113_u8,fld3: _54.fld3 };
_45 = !_16.fld0;
_7 = !(*_13).3;
_45 = !_16.fld0;
_21 = [_22,_22,_30,_18];
_18 = _36 ^ _36;
_42 = _16.fld0 as i16;
_13 = core::ptr::addr_of!((*_13));
Call(_42 = core::intrinsics::transmute(_10), ReturnTo(bb19), UnwindUnreachable())
}
bb41 = {
place!(Field::<Adt54>(Variant(_20, 2), 5)) = Adt54::Variant1 { fld0: _60,fld1: (*_13).0 };
_40 = (_60.0, _60.1, _66.2, _60.3);
_54.fld3.0 = [_44.3,_3.fld0,_2,_3.fld0,_16.fld4.3];
place!(Field::<usize>(Variant(_20, 2), 1)) = Field::<(usize, i8, i128, i128)>(Variant(Field::<Adt54>(Variant(_20, 2), 5), 1), 0).0 * Field::<(usize, i8, i128, i128)>(Variant(Field::<Adt54>(Variant(_20, 2), 5), 1), 0).0;
place!(Field::<isize>(Variant(_56, 1), 0)) = -_48;
_32 = Adt47::Variant2 { fld0: _54.fld3 };
_16.fld4.3 = _44.3 ^ _24.fld0;
_51.2 = !(*_1).2;
_73.3 = (*_1).3 >> _44.3;
_75.2 = _45 ^ _45;
_73 = ((*_1).0, Field::<char>(Variant(Field::<Adt54>(Variant(_20, 2), 5), 1), 1), (*_1).2, _3.fld0, (*_13).4);
_39 = Adt45 { fld0: _2 };
match _47.fld2 {
56 => bb42,
_ => bb23
}
}
bb42 = {
place!(Field::<isize>(Variant(_56, 1), 0)) = _22;
_27 = _31 >> _39.fld0;
_54 = Adt48 { fld0: _60.2,fld1: _47.fld1,fld2: _47.fld2,fld3: _47.fld3 };
_16.fld4.2 = _8 & _8;
_15 = _16.fld4.1 as u64;
_73.4 = _44.4;
_51 = ((*_13).0, _73.0, _16.fld4.2, _39.fld0, (*_1).4);
place!(Field::<[u32; 7]>(Variant(_20, 2), 2)) = [Field::<u32>(Variant(_53, 0), 0),Field::<u32>(Variant(_53, 0), 0),Field::<u32>(Variant(_53, 0), 0),Field::<u32>(Variant(_53, 0), 0),Field::<u32>(Variant(_53, 0), 0),Field::<u32>(Variant(_53, 0), 0),Field::<u32>(Variant(_53, 0), 0)];
Goto(bb43)
}
bb43 = {
_51.2 = (*_13).2 + _73.2;
_4 = _7;
_16.fld4 = (_73.1, _73.1, _51.2, _2, _16.fld2);
_74 = &_70;
_39 = Adt45 { fld0: (*_13).3 };
_1 = core::ptr::addr_of!(_51);
_70 = _18 & _22;
_27 = _15;
_46 = Field::<f64>(Variant(_53, 0), 2) - _16.fld2;
_18 = _48 >> _48;
_6 = _1;
place!(Field::<u32>(Variant(_20, 2), 0)) = _64 - Field::<u32>(Variant(_53, 0), 0);
_11 = _3;
place!(Field::<([u16; 5],)>(Variant(_32, 2), 0)).0 = _25;
_75.0 = _60.0;
_65 = (*_13).2 as u128;
_73 = (*_13);
_77 = _45;
_75.0 = !_66.0;
_51.3 = _5.fld0 << _54.fld2;
_44.3 = !_24.fld0;
_75 = Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4);
place!(Field::<(usize, [i8; 2], bool)>(Variant(_20, 2), 4)).2 = _24.fld0 >= _73.3;
_54.fld2 = !_47.fld2;
_68 = [Field::<(usize, i8, i128, i128)>(Variant(Field::<Adt54>(Variant(_20, 2), 5), 1), 0).1,_60.1];
match _47.fld2 {
0 => bb25,
1 => bb20,
2 => bb44,
3 => bb45,
56 => bb47,
_ => bb46
}
}
bb44 = {
Return()
}
bb45 = {
_3.fld0 = _24.fld0 ^ _5.fld0;
_15 = 219_u8 as u64;
Call(_22 = fn14(_13, _11.fld0, _16.fld3.0, _16.fld4.3, _5, _24), ReturnTo(bb10), UnwindUnreachable())
}
bb46 = {
_16.fld4.2 = 1312680802_u32 as i16;
_16.fld3.0 = [_12,_3.fld0,_4,_2,_2];
_16.fld4.0 = (*_6).1;
_16.fld4.3 = !_10;
_16.fld0 = !false;
_1 = _13;
_16.fld4.3 = _12 ^ _5.fld0;
_4 = (-1865757816_i32) as u16;
_8 = (*_6).2 ^ (*_6).2;
_8 = _16.fld4.2 - (*_6).2;
_16.fld4.1 = (*_6).0;
_10 = (*_6).3 * _2;
_11.fld0 = _16.fld4.3 >> _5.fld0;
_16.fld4.4 = 11656308865437106066_u64 as f64;
_9 = _16.fld3.0;
_5.fld0 = (*_6).3;
_16.fld3 = (_9,);
_16.fld4.0 = (*_6).1;
_16.fld1 = (*_6).1;
_16.fld3.0 = [(*_6).3,_12,(*_6).3,_7,(*_6).3];
_8 = (*_6).2 & (*_6).2;
_6 = _1;
_10 = _16.fld4.3 * _3.fld0;
_22 = (-125_isize);
Goto(bb3)
}
bb47 = {
_51.1 = _16.fld4.0;
_16.fld4.3 = _12;
_79 = _18 ^ _70;
place!(Field::<[u32; 7]>(Variant(_20, 2), 2)) = [Field::<u32>(Variant(_20, 2), 0),_64,Field::<u32>(Variant(_53, 0), 0),Field::<u32>(Variant(_20, 2), 0),Field::<u32>(Variant(_20, 2), 0),Field::<u32>(Variant(_53, 0), 0),Field::<u32>(Variant(_20, 2), 0)];
_75.1 = [Field::<(usize, i8, i128, i128)>(Variant(Field::<Adt54>(Variant(_20, 2), 5), 1), 0).1,_40.1];
_66 = (_75.0, Field::<(usize, i8, i128, i128)>(Variant(Field::<Adt54>(Variant(_20, 2), 5), 1), 0).1, _60.2, _47.fld0);
_52 = [_58,_35,_58];
RET = Adt51::Variant2 { fld0: _61,fld1: _75.1,fld2: _35 };
_54.fld3.0 = [_39.fld0,_7,_24.fld0,_10,(*_1).3];
Goto(bb48)
}
bb48 = {
Call(_82 = dump_var(13_usize, 30_usize, Move(_30), 70_usize, Move(_70), 50_usize, Move(_50), 40_usize, Move(_40)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_82 = dump_var(13_usize, 34_usize, Move(_34), 68_usize, Move(_68), 45_usize, Move(_45), 64_usize, Move(_64)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_82 = dump_var(13_usize, 36_usize, Move(_36), 31_usize, Move(_31), 15_usize, Move(_15), 12_usize, Move(_12)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_82 = dump_var(13_usize, 52_usize, Move(_52), 9_usize, Move(_9), 28_usize, Move(_28), 66_usize, Move(_66)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_82 = dump_var(13_usize, 29_usize, Move(_29), 7_usize, Move(_7), 35_usize, Move(_35), 83_usize, _83), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *const (char, char, i16, u16, f64),mut _2: u16,mut _3: [u16; 5],mut _4: u16,mut _5: Adt45,mut _6: Adt45) -> isize {
mir! {
type RET = isize;
let _7: char;
let _8: f64;
let _9: u16;
let _10: ([i8; 2], [i128; 5]);
let _11: [isize; 4];
let _12: *mut (char, char, i16, u16, f64);
let _13: bool;
let _14: Adt52;
let _15: char;
let _16: [u32; 7];
let _17: i8;
let _18: ([u16; 5],);
let _19: isize;
let _20: ([i128; 5], i16);
let _21: Adt51;
let _22: (usize, [i8; 2], bool);
let _23: u8;
let _24: Adt52;
let _25: f64;
let _26: u8;
let _27: Adt41;
let _28: char;
let _29: char;
let _30: (usize, [i8; 2], bool);
let _31: [u128; 1];
let _32: isize;
let _33: f64;
let _34: Adt53;
let _35: (char, char, i16, u16, f64);
let _36: isize;
let _37: ();
let _38: ();
{
_6.fld0 = !_4;
_5 = Adt45 { fld0: _4 };
RET = 16269_i16 as isize;
_5.fld0 = _4;
_2 = _5.fld0 | _6.fld0;
RET = (-9223372036854775808_isize) >> _4;
RET = 2011697446_u32 as isize;
Goto(bb1)
}
bb1 = {
_4 = _6.fld0 - _6.fld0;
RET = (-29370_i16) as isize;
_2 = !_6.fld0;
_3 = [_4,_5.fld0,_4,_2,_6.fld0];
_6.fld0 = _2;
_4 = _5.fld0;
_4 = _2 ^ _2;
Goto(bb2)
}
bb2 = {
RET = (-73_isize) * (-9223372036854775808_isize);
_8 = RET as f64;
_8 = 1_usize as f64;
_10.0 = [(-123_i8),(-36_i8)];
RET = 9223372036854775807_isize ^ (-126_isize);
Goto(bb3)
}
bb3 = {
_10.0 = [(-101_i8),(-42_i8)];
_8 = RET as f64;
_8 = _6.fld0 as f64;
_6.fld0 = (-6707739809241125173_i64) as u16;
Goto(bb4)
}
bb4 = {
_11 = [RET,RET,RET,RET];
_9 = _5.fld0 ^ _4;
_9 = (-144426117310700769490435893393709017941_i128) as u16;
_6 = _5;
_4 = !_6.fld0;
RET = 9223372036854775807_isize;
_5 = Adt45 { fld0: _6.fld0 };
_3 = [_2,_4,_5.fld0,_5.fld0,_6.fld0];
_10.1 = [(-149458130463092025231067909557454225182_i128),87813060490474290945536035845163694509_i128,(-151721096249795348322326144577800905291_i128),(-7229809267279029684251748414901207921_i128),(-155280802271723850898965336353666994123_i128)];
_14 = Adt52::Variant0 { fld0: (-58057589243045037617746107814029692465_i128) };
_2 = _4 * _6.fld0;
_2 = 9177110113349689330_i64 as u16;
Goto(bb5)
}
bb5 = {
_13 = false | true;
_15 = '\u{103ce5}';
_2 = _4 - _5.fld0;
_7 = _15;
_5.fld0 = _6.fld0;
_6.fld0 = 6596_i16 as u16;
_19 = 0_usize as isize;
_20.1 = 130_i16 * (-1835_i16);
_18 = (_3,);
_16 = [1073026108_u32,4123241167_u32,1855312780_u32,143194782_u32,2015241387_u32,2198166138_u32,777907248_u32];
_18 = (_3,);
_18 = (_3,);
_16 = [769249005_u32,3377951954_u32,2973892226_u32,750520860_u32,2700839722_u32,2815157637_u32,828225176_u32];
_20.0 = [(-134979071688780644326565905011853532_i128),126624396292773660284668435845533288649_i128,(-111281999854243135537160022210723800938_i128),(-96708406137451790258240076755723422574_i128),113063859440535170590564838101908328493_i128];
place!(Field::<i128>(Variant(_14, 0), 0)) = 110926128872826143606627973154882306693_i128;
_2 = 47152706777482896534731488251777518554_u128 as u16;
_17 = _4 as i8;
_17 = 21113253_u32 as i8;
_19 = RET;
match _19 {
9223372036854775807 => bb6,
_ => bb3
}
}
bb6 = {
_5 = Adt45 { fld0: _4 };
_10.1 = [Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0)];
place!(Field::<i128>(Variant(_14, 0), 0)) = (-83911551795624262317636362200877891677_i128);
_20.1 = 5383_i16 * (-18417_i16);
_5 = Adt45 { fld0: _4 };
SetDiscriminant(_14, 1);
place!(Field::<(char, char, i16, u16, f64)>(Variant(_14, 1), 1)).2 = _20.1 - _20.1;
_12 = core::ptr::addr_of_mut!(place!(Field::<(char, char, i16, u16, f64)>(Variant(_14, 1), 1)));
place!(Field::<[u16; 5]>(Variant(_14, 1), 0)) = [_4,_5.fld0,_5.fld0,_5.fld0,_5.fld0];
_22.1 = _10.0;
place!(Field::<isize>(Variant(_14, 1), 2)) = _13 as isize;
place!(Field::<*mut usize>(Variant(_14, 1), 3)) = core::ptr::addr_of_mut!(_22.0);
_19 = Field::<isize>(Variant(_14, 1), 2);
_14 = Adt52::Variant0 { fld0: (-20951682912046249880393051380768622770_i128) };
_3 = _18.0;
_22.1 = [_17,_17];
place!(Field::<i128>(Variant(_14, 0), 0)) = 23891545470406885580765995959715938233_i128;
_20.0 = [Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0)];
_22.1 = _10.0;
_20 = (_10.1, 17499_i16);
Goto(bb7)
}
bb7 = {
place!(Field::<i128>(Variant(_14, 0), 0)) = (-47384600468835072470216695886914166691_i128) - (-97275378546910814711241587739823399687_i128);
_22.0 = !4147024734905307651_usize;
_5.fld0 = 2846842988_u32 as u16;
_22.2 = _4 >= _4;
_22 = (10569712926508311108_usize, _10.0, _13);
_10.1 = [Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0)];
_19 = _13 as isize;
_4 = 5335244611355865237_i64 as u16;
_11 = [RET,RET,_19,RET];
_22.1 = [_17,_17];
_22.1 = [_17,_17];
_18 = (_3,);
_23 = 90_u8;
_25 = -_8;
_26 = (-1557345249_i32) as u8;
_2 = _6.fld0;
_26 = _23 & _23;
_27.fld4.1 = _15;
_16 = [4016957866_u32,54868533_u32,4075598589_u32,4164511554_u32,3215657110_u32,3011943372_u32,2621990419_u32];
_27.fld4.3 = _17 as u16;
_9 = _20.1 as u16;
_6 = _5;
match _22.0 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
10569712926508311108 => bb9,
_ => bb8
}
}
bb8 = {
_4 = _6.fld0 - _6.fld0;
RET = (-29370_i16) as isize;
_2 = !_6.fld0;
_3 = [_4,_5.fld0,_4,_2,_6.fld0];
_6.fld0 = _2;
_4 = _5.fld0;
_4 = _2 ^ _2;
Goto(bb2)
}
bb9 = {
SetDiscriminant(_14, 2);
_17 = (-90_i8) * 39_i8;
_27.fld3 = (_18.0,);
_6.fld0 = _5.fld0 + _9;
_27.fld4.4 = 9626713906893651885_u64 as f64;
_25 = _8 - _8;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_14, 2), 1)).1 = _10.1;
_7 = _27.fld4.1;
_20 = (_10.1, (-7658_i16));
place!(Field::<i128>(Variant(_14, 2), 3)) = (-1947594857_i32) as i128;
place!(Field::<*mut usize>(Variant(_14, 2), 0)) = core::ptr::addr_of_mut!(_22.0);
_27.fld2 = Field::<i128>(Variant(_14, 2), 3) as f64;
_10.1 = [Field::<i128>(Variant(_14, 2), 3),Field::<i128>(Variant(_14, 2), 3),Field::<i128>(Variant(_14, 2), 3),Field::<i128>(Variant(_14, 2), 3),Field::<i128>(Variant(_14, 2), 3)];
place!(Field::<u16>(Variant(_14, 2), 4)) = _4;
_20 = (Field::<([i8; 2], [i128; 5])>(Variant(_14, 2), 1).1, (-2568_i16));
Goto(bb10)
}
bb10 = {
place!(Field::<*mut usize>(Variant(_14, 2), 0)) = core::ptr::addr_of_mut!(_22.0);
_13 = _8 <= _8;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_14, 2), 1)) = _10;
_27.fld1 = _7;
_27.fld0 = _13;
_15 = _7;
_15 = _27.fld1;
_7 = _27.fld4.1;
_27.fld4.1 = _7;
_30.1 = [_17,_17];
_17 = !(-34_i8);
_27.fld4.3 = !_6.fld0;
_35.1 = _27.fld1;
match _22.0 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb9,
4 => bb11,
5 => bb12,
6 => bb13,
10569712926508311108 => bb15,
_ => bb14
}
}
bb11 = {
SetDiscriminant(_14, 2);
_17 = (-90_i8) * 39_i8;
_27.fld3 = (_18.0,);
_6.fld0 = _5.fld0 + _9;
_27.fld4.4 = 9626713906893651885_u64 as f64;
_25 = _8 - _8;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_14, 2), 1)).1 = _10.1;
_7 = _27.fld4.1;
_20 = (_10.1, (-7658_i16));
place!(Field::<i128>(Variant(_14, 2), 3)) = (-1947594857_i32) as i128;
place!(Field::<*mut usize>(Variant(_14, 2), 0)) = core::ptr::addr_of_mut!(_22.0);
_27.fld2 = Field::<i128>(Variant(_14, 2), 3) as f64;
_10.1 = [Field::<i128>(Variant(_14, 2), 3),Field::<i128>(Variant(_14, 2), 3),Field::<i128>(Variant(_14, 2), 3),Field::<i128>(Variant(_14, 2), 3),Field::<i128>(Variant(_14, 2), 3)];
place!(Field::<u16>(Variant(_14, 2), 4)) = _4;
_20 = (Field::<([i8; 2], [i128; 5])>(Variant(_14, 2), 1).1, (-2568_i16));
Goto(bb10)
}
bb12 = {
_11 = [RET,RET,RET,RET];
_9 = _5.fld0 ^ _4;
_9 = (-144426117310700769490435893393709017941_i128) as u16;
_6 = _5;
_4 = !_6.fld0;
RET = 9223372036854775807_isize;
_5 = Adt45 { fld0: _6.fld0 };
_3 = [_2,_4,_5.fld0,_5.fld0,_6.fld0];
_10.1 = [(-149458130463092025231067909557454225182_i128),87813060490474290945536035845163694509_i128,(-151721096249795348322326144577800905291_i128),(-7229809267279029684251748414901207921_i128),(-155280802271723850898965336353666994123_i128)];
_14 = Adt52::Variant0 { fld0: (-58057589243045037617746107814029692465_i128) };
_2 = _4 * _6.fld0;
_2 = 9177110113349689330_i64 as u16;
Goto(bb5)
}
bb13 = {
place!(Field::<i128>(Variant(_14, 0), 0)) = (-47384600468835072470216695886914166691_i128) - (-97275378546910814711241587739823399687_i128);
_22.0 = !4147024734905307651_usize;
_5.fld0 = 2846842988_u32 as u16;
_22.2 = _4 >= _4;
_22 = (10569712926508311108_usize, _10.0, _13);
_10.1 = [Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0)];
_19 = _13 as isize;
_4 = 5335244611355865237_i64 as u16;
_11 = [RET,RET,_19,RET];
_22.1 = [_17,_17];
_22.1 = [_17,_17];
_18 = (_3,);
_23 = 90_u8;
_25 = -_8;
_26 = (-1557345249_i32) as u8;
_2 = _6.fld0;
_26 = _23 & _23;
_27.fld4.1 = _15;
_16 = [4016957866_u32,54868533_u32,4075598589_u32,4164511554_u32,3215657110_u32,3011943372_u32,2621990419_u32];
_27.fld4.3 = _17 as u16;
_9 = _20.1 as u16;
_6 = _5;
match _22.0 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
10569712926508311108 => bb9,
_ => bb8
}
}
bb14 = {
_5 = Adt45 { fld0: _4 };
_10.1 = [Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0)];
place!(Field::<i128>(Variant(_14, 0), 0)) = (-83911551795624262317636362200877891677_i128);
_20.1 = 5383_i16 * (-18417_i16);
_5 = Adt45 { fld0: _4 };
SetDiscriminant(_14, 1);
place!(Field::<(char, char, i16, u16, f64)>(Variant(_14, 1), 1)).2 = _20.1 - _20.1;
_12 = core::ptr::addr_of_mut!(place!(Field::<(char, char, i16, u16, f64)>(Variant(_14, 1), 1)));
place!(Field::<[u16; 5]>(Variant(_14, 1), 0)) = [_4,_5.fld0,_5.fld0,_5.fld0,_5.fld0];
_22.1 = _10.0;
place!(Field::<isize>(Variant(_14, 1), 2)) = _13 as isize;
place!(Field::<*mut usize>(Variant(_14, 1), 3)) = core::ptr::addr_of_mut!(_22.0);
_19 = Field::<isize>(Variant(_14, 1), 2);
_14 = Adt52::Variant0 { fld0: (-20951682912046249880393051380768622770_i128) };
_3 = _18.0;
_22.1 = [_17,_17];
place!(Field::<i128>(Variant(_14, 0), 0)) = 23891545470406885580765995959715938233_i128;
_20.0 = [Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0),Field::<i128>(Variant(_14, 0), 0)];
_22.1 = _10.0;
_20 = (_10.1, 17499_i16);
Goto(bb7)
}
bb15 = {
_11 = [_19,RET,_19,_19];
_27.fld3 = (_18.0,);
_19 = -RET;
_27.fld3 = _18;
RET = _20.1 as isize;
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(14_usize, 23_usize, Move(_23), 7_usize, Move(_7), 22_usize, Move(_22), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(14_usize, 19_usize, Move(_19), 10_usize, Move(_10), 3_usize, Move(_3), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: ([u16; 5],),mut _2: *const (char, char, i16, u16, f64),mut _3: [u16; 5]) -> i16 {
mir! {
type RET = i16;
let _4: Adt47;
let _5: Adt42;
let _6: u32;
let _7: Adt52;
let _8: Adt50;
let _9: Adt44;
let _10: Adt46;
let _11: ([i8; 2], [i128; 5]);
let _12: i8;
let _13: isize;
let _14: f64;
let _15: isize;
let _16: isize;
let _17: *mut (char, char, i16, u16, f64);
let _18: (usize, [i8; 2], bool);
let _19: *mut usize;
let _20: isize;
let _21: *mut i16;
let _22: [u32; 4];
let _23: char;
let _24: Adt55;
let _25: *const (char, char, i16, u16, f64);
let _26: ();
let _27: ();
{
RET = !22383_i16;
_1 = (_3,);
_1 = (_3,);
_1 = (_3,);
RET = 13374_i16 & 25556_i16;
RET = (-19879_i16);
_1.0 = _3;
RET = -29310_i16;
RET = 22856_i16 | 10394_i16;
RET = 29108_i16 - (-20423_i16);
_1 = (_3,);
_4 = Adt47::Variant2 { fld0: _1 };
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (_3,);
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (_3,);
RET = 26637_i16 & (-20055_i16);
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)).0 = _1.0;
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)).0 = [62628_u16,10544_u16,50860_u16,46973_u16,1920_u16];
_1 = (_3,);
_1 = Field::<([u16; 5],)>(Variant(_4, 2), 0);
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (_3,);
_4 = Adt47::Variant2 { fld0: _1 };
_3 = [32695_u16,61339_u16,32915_u16,52609_u16,25831_u16];
Call(RET = core::intrinsics::bswap((-24985_i16)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-11761_i16) | (-27261_i16);
_1 = (Field::<([u16; 5],)>(Variant(_4, 2), 0).0,);
RET = 137_u8 as i16;
_1.0 = [9535_u16,52478_u16,334_u16,50118_u16,53449_u16];
_5 = Adt42::Variant3 { fld0: Field::<([u16; 5],)>(Variant(_4, 2), 0).0,fld1: _2 };
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)).0 = [6388_u16,58408_u16,34543_u16,50652_u16,57765_u16];
place!(Field::<[u16; 5]>(Variant(_5, 3), 0)) = Field::<([u16; 5],)>(Variant(_4, 2), 0).0;
_7 = Adt52::Variant0 { fld0: (-154056764358434970910336302180696147983_i128) };
_3 = [14834_u16,6740_u16,18830_u16,45751_u16,52964_u16];
Goto(bb2)
}
bb2 = {
_3 = [7568_u16,36138_u16,63481_u16,28433_u16,10398_u16];
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)).0 = [26381_u16,54594_u16,26753_u16,25798_u16,37954_u16];
_7 = Adt52::Variant0 { fld0: 89780322683766092129147790368504397905_i128 };
place!(Field::<i128>(Variant(_7, 0), 0)) = 9590177459633697771170031090334152037_i128;
_6 = 4054830771_u32;
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (_1.0,);
SetDiscriminant(_7, 2);
_5 = Adt42::Variant3 { fld0: Field::<([u16; 5],)>(Variant(_4, 2), 0).0,fld1: _2 };
place!(Field::<u64>(Variant(_7, 2), 2)) = !21591377559804938_u64;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1)).0 = [(-79_i8),(-49_i8)];
SetDiscriminant(_4, 2);
_1.0 = Field::<[u16; 5]>(Variant(_5, 3), 0);
SetDiscriminant(_5, 3);
_11.1 = [144964314514219965001674000984173481913_i128,(-82554039946829366799166361453600132812_i128),(-68837537020501285749642775291331605022_i128),26328331279247401899526875673312641753_i128,56300334908711338062029389068973723203_i128];
_11.1 = [90538598443524000157249890125921741251_i128,(-86866265595958435140018281279460298347_i128),84148431729948494246042724061411935082_i128,65542081114351447768314390291152179822_i128,(-35570001009189174721945850571242147491_i128)];
_4 = Adt47::Variant2 { fld0: _1 };
place!(Field::<[u16; 5]>(Variant(_5, 3), 0)) = [17881_u16,24783_u16,39066_u16,63878_u16,58353_u16];
_1 = (Field::<[u16; 5]>(Variant(_5, 3), 0),);
_11.1 = [(-150452415157713279590623403449881783421_i128),(-125444787975567077471110788060638856951_i128),72987762835344023424544035122305713905_i128,46447871290014928056545218069630097957_i128,(-156200060536645175091287480171790005259_i128)];
RET = (-4638_i16);
place!(Field::<i128>(Variant(_7, 2), 3)) = 86223902748401405884539914467769848173_i128;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1)).1 = _11.1;
_13 = 9223372036854775807_isize;
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (Field::<[u16; 5]>(Variant(_5, 3), 0),);
place!(Field::<u16>(Variant(_7, 2), 4)) = 1354708050_i32 as u16;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1)).0 = [(-115_i8),26_i8];
Goto(bb3)
}
bb3 = {
_14 = Field::<u64>(Variant(_7, 2), 2) as f64;
_1 = (_3,);
_3 = [Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4)];
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(_5, 3), 1)) = _2;
_11 = (Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1).0, Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1).1);
_15 = !_13;
_14 = 106_u8 as f64;
_1.0 = Field::<([u16; 5],)>(Variant(_4, 2), 0).0;
Goto(bb4)
}
bb4 = {
_4 = Adt47::Variant2 { fld0: _1 };
_1 = (Field::<[u16; 5]>(Variant(_5, 3), 0),);
place!(Field::<i128>(Variant(_7, 2), 3)) = 128699158419281544262750398617653981175_i128;
_12 = (-87_i8);
_6 = 3254465204_u32 ^ 3107579961_u32;
place!(Field::<i128>(Variant(_7, 2), 3)) = 157438003405103078077534261536901537963_i128;
_18.1 = [_12,_12];
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(_5, 3), 1)) = _2;
_6 = !2620945723_u32;
_11.0 = _18.1;
_11 = Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1);
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (_1.0,);
_11.0 = [_12,_12];
_6 = 1742213116_u32;
_18.2 = !true;
_18.1 = [_12,_12];
_12 = _13 as i8;
Call(_20 = core::intrinsics::transmute(_13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20 = Field::<u16>(Variant(_7, 2), 4) as isize;
RET = Field::<u16>(Variant(_7, 2), 4) as i16;
_19 = core::ptr::addr_of_mut!(_18.0);
_1.0 = _3;
_11.1 = Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1).1;
_16 = _18.2 as isize;
SetDiscriminant(_5, 2);
_12 = 21_i8;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_5, 2), 3)).0 = [_12,_12];
_1.0 = [Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4)];
_12 = !51_i8;
place!(Field::<f64>(Variant(_5, 2), 4)) = _14 * _14;
_20 = !_13;
place!(Field::<*mut usize>(Variant(_7, 2), 0)) = core::ptr::addr_of_mut!(_18.0);
SetDiscriminant(_7, 2);
_14 = -Field::<f64>(Variant(_5, 2), 4);
place!(Field::<([i8; 2], [i128; 5])>(Variant(_5, 2), 3)) = (_18.1, _11.1);
_22 = [_6,_6,_6,_6];
place!(Field::<([i8; 2], [i128; 5])>(Variant(_5, 2), 3)).0 = [_12,_12];
_11.0 = [_12,_12];
_18 = (2_usize, _11.0, false);
place!(Field::<[u128; 3]>(Variant(_5, 2), 0)) = [150888580156745807657770834158493451727_u128,67442661713406730792358562407104578420_u128,223236655101183101475757611744684233031_u128];
place!(Field::<*mut usize>(Variant(_7, 2), 0)) = core::ptr::addr_of_mut!((*_19));
_1 = (Field::<([u16; 5],)>(Variant(_4, 2), 0).0,);
match (*_19) {
0 => bb6,
1 => bb7,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
2 => bb14,
_ => bb13
}
}
bb6 = {
_4 = Adt47::Variant2 { fld0: _1 };
_1 = (Field::<[u16; 5]>(Variant(_5, 3), 0),);
place!(Field::<i128>(Variant(_7, 2), 3)) = 128699158419281544262750398617653981175_i128;
_12 = (-87_i8);
_6 = 3254465204_u32 ^ 3107579961_u32;
place!(Field::<i128>(Variant(_7, 2), 3)) = 157438003405103078077534261536901537963_i128;
_18.1 = [_12,_12];
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(_5, 3), 1)) = _2;
_6 = !2620945723_u32;
_11.0 = _18.1;
_11 = Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1);
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (_1.0,);
_11.0 = [_12,_12];
_6 = 1742213116_u32;
_18.2 = !true;
_18.1 = [_12,_12];
_12 = _13 as i8;
Call(_20 = core::intrinsics::transmute(_13), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_14 = Field::<u64>(Variant(_7, 2), 2) as f64;
_1 = (_3,);
_3 = [Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4),Field::<u16>(Variant(_7, 2), 4)];
place!(Field::<*const (char, char, i16, u16, f64)>(Variant(_5, 3), 1)) = _2;
_11 = (Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1).0, Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1).1);
_15 = !_13;
_14 = 106_u8 as f64;
_1.0 = Field::<([u16; 5],)>(Variant(_4, 2), 0).0;
Goto(bb4)
}
bb8 = {
_3 = [7568_u16,36138_u16,63481_u16,28433_u16,10398_u16];
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)).0 = [26381_u16,54594_u16,26753_u16,25798_u16,37954_u16];
_7 = Adt52::Variant0 { fld0: 89780322683766092129147790368504397905_i128 };
place!(Field::<i128>(Variant(_7, 0), 0)) = 9590177459633697771170031090334152037_i128;
_6 = 4054830771_u32;
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (_1.0,);
SetDiscriminant(_7, 2);
_5 = Adt42::Variant3 { fld0: Field::<([u16; 5],)>(Variant(_4, 2), 0).0,fld1: _2 };
place!(Field::<u64>(Variant(_7, 2), 2)) = !21591377559804938_u64;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1)).0 = [(-79_i8),(-49_i8)];
SetDiscriminant(_4, 2);
_1.0 = Field::<[u16; 5]>(Variant(_5, 3), 0);
SetDiscriminant(_5, 3);
_11.1 = [144964314514219965001674000984173481913_i128,(-82554039946829366799166361453600132812_i128),(-68837537020501285749642775291331605022_i128),26328331279247401899526875673312641753_i128,56300334908711338062029389068973723203_i128];
_11.1 = [90538598443524000157249890125921741251_i128,(-86866265595958435140018281279460298347_i128),84148431729948494246042724061411935082_i128,65542081114351447768314390291152179822_i128,(-35570001009189174721945850571242147491_i128)];
_4 = Adt47::Variant2 { fld0: _1 };
place!(Field::<[u16; 5]>(Variant(_5, 3), 0)) = [17881_u16,24783_u16,39066_u16,63878_u16,58353_u16];
_1 = (Field::<[u16; 5]>(Variant(_5, 3), 0),);
_11.1 = [(-150452415157713279590623403449881783421_i128),(-125444787975567077471110788060638856951_i128),72987762835344023424544035122305713905_i128,46447871290014928056545218069630097957_i128,(-156200060536645175091287480171790005259_i128)];
RET = (-4638_i16);
place!(Field::<i128>(Variant(_7, 2), 3)) = 86223902748401405884539914467769848173_i128;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1)).1 = _11.1;
_13 = 9223372036854775807_isize;
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (Field::<[u16; 5]>(Variant(_5, 3), 0),);
place!(Field::<u16>(Variant(_7, 2), 4)) = 1354708050_i32 as u16;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1)).0 = [(-115_i8),26_i8];
Goto(bb3)
}
bb9 = {
RET = (-11761_i16) | (-27261_i16);
_1 = (Field::<([u16; 5],)>(Variant(_4, 2), 0).0,);
RET = 137_u8 as i16;
_1.0 = [9535_u16,52478_u16,334_u16,50118_u16,53449_u16];
_5 = Adt42::Variant3 { fld0: Field::<([u16; 5],)>(Variant(_4, 2), 0).0,fld1: _2 };
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)).0 = [6388_u16,58408_u16,34543_u16,50652_u16,57765_u16];
place!(Field::<[u16; 5]>(Variant(_5, 3), 0)) = Field::<([u16; 5],)>(Variant(_4, 2), 0).0;
_7 = Adt52::Variant0 { fld0: (-154056764358434970910336302180696147983_i128) };
_3 = [14834_u16,6740_u16,18830_u16,45751_u16,52964_u16];
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
_11.0 = [_12,_12];
place!(Field::<([i8; 2], [i128; 5])>(Variant(_5, 2), 3)).0 = _18.1;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_5, 2), 3)).0 = [_12,_12];
_16 = -_13;
SetDiscriminant(_4, 2);
place!(Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1)).0 = [_12,_12];
RET = (-9643_i16) * 31320_i16;
_22 = [_6,_6,_6,_6];
place!(Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1)).1 = [(-84868657623114290617870756426591798580_i128),128356024355816425115587127993536685620_i128,(-71882406973261817988124876522058550185_i128),155405109595865194963407168646890897132_i128,141126068826465428250379716498575193862_i128];
place!(Field::<([u16; 5],)>(Variant(_4, 2), 0)) = (_3,);
place!(Field::<u64>(Variant(_7, 2), 2)) = !16926541300237789270_u64;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_5, 2), 3)) = (_11.0, Field::<([i8; 2], [i128; 5])>(Variant(_7, 2), 1).1);
_1.0 = [18541_u16,49215_u16,19676_u16,38326_u16,27847_u16];
place!(Field::<*mut usize>(Variant(_7, 2), 0)) = _19;
_16 = '\u{fd7d5}' as isize;
_7 = Adt52::Variant2 { fld0: _19,fld1: Field::<([i8; 2], [i128; 5])>(Variant(_5, 2), 3),fld2: 18043790124507628302_u64,fld3: 4433443901245394670353854565497359405_i128,fld4: 64584_u16 };
_24.fld5 = 314099172_i32 | (-192630625_i32);
place!(Field::<u64>(Variant(_7, 2), 2)) = 9854167556969991929_u64;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(15_usize, 20_usize, Move(_20), 15_usize, Move(_15), 22_usize, Move(_22), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(15_usize, 1_usize, Move(_1), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u16,mut _2: isize,mut _3: u16,mut _4: u16) -> f64 {
mir! {
type RET = f64;
let _5: f64;
let _6: isize;
let _7: f64;
let _8: usize;
let _9: (usize, [i8; 2], bool);
let _10: bool;
let _11: u32;
let _12: i64;
let _13: char;
let _14: f64;
let _15: bool;
let _16: (usize, i8, i128, i128);
let _17: i8;
let _18: Adt54;
let _19: Adt44;
let _20: u8;
let _21: u64;
let _22: [i8; 2];
let _23: [u32; 7];
let _24: ();
let _25: ();
{
_2 = !(-9223372036854775808_isize);
RET = 7_usize as f64;
_2 = '\u{55936}' as isize;
_5 = 5_usize as f64;
_2 = '\u{feb}' as isize;
_4 = _3;
RET = -_5;
_2 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_2 = (-26382_i16) as isize;
_2 = (-9223372036854775808_isize) + (-83_isize);
_5 = -RET;
_1 = !_3;
_6 = !_2;
_5 = RET + RET;
_3 = !_1;
_8 = 1_usize << _4;
RET = _5;
RET = -_5;
_1 = _3 * _3;
_4 = 12547375023976103764_u64 as u16;
_2 = _6 & _6;
_3 = _1 << _1;
Goto(bb1)
}
bb1 = {
_1 = _3 & _3;
_7 = RET - _5;
_1 = (-69_i8) as u16;
_9.2 = true;
_1 = !_3;
Call(_8 = fn17(_1, _3, _3, _3, _1, _9.2, _3, _2, _3, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _7;
_10 = _9.2;
_1 = 2058977326_i32 as u16;
_13 = '\u{cbd2}';
_6 = _3 as isize;
_3 = _4 - _1;
_3 = _4;
_12 = (-3150539183775656687_i64) << _6;
_4 = !_1;
_5 = -RET;
_10 = _9.2;
_15 = !_9.2;
_9.0 = _8 * _8;
Call(_16.2 = core::intrinsics::bswap((-35949345138504087525883279204468838478_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = -_7;
Goto(bb4)
}
bb4 = {
_17 = 253119228_i32 as i8;
_16 = (_9.0, _17, 166275907431668648736110293500003320695_i128, (-25748872830316541005274521451520366086_i128));
_9.1 = [_17,_17];
Goto(bb5)
}
bb5 = {
RET = _5 + _7;
_16.3 = _16.2 | _16.2;
_11 = !3121478040_u32;
match _16.2 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
166275907431668648736110293500003320695 => bb9,
_ => bb8
}
}
bb6 = {
_17 = 253119228_i32 as i8;
_16 = (_9.0, _17, 166275907431668648736110293500003320695_i128, (-25748872830316541005274521451520366086_i128));
_9.1 = [_17,_17];
Goto(bb5)
}
bb7 = {
RET = -_7;
Goto(bb4)
}
bb8 = {
RET = _7;
_10 = _9.2;
_1 = 2058977326_i32 as u16;
_13 = '\u{cbd2}';
_6 = _3 as isize;
_3 = _4 - _1;
_3 = _4;
_12 = (-3150539183775656687_i64) << _6;
_4 = !_1;
_5 = -RET;
_10 = _9.2;
_15 = !_9.2;
_9.0 = _8 * _8;
Call(_16.2 = core::intrinsics::bswap((-35949345138504087525883279204468838478_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
RET = _7 - _5;
_7 = _5 - RET;
_16.0 = _9.0;
_10 = _9.2;
_10 = _15 & _15;
_20 = 64_u8 + 231_u8;
_4 = !_3;
RET = _7 + _5;
_4 = _1;
_14 = RET;
_13 = '\u{622f4}';
_16.2 = _16.3;
RET = -_14;
_7 = _17 as f64;
_15 = _10;
_9.0 = _16.0 << _6;
_16.0 = _9.0 ^ _9.0;
Goto(bb10)
}
bb10 = {
Call(_24 = dump_var(16_usize, 6_usize, Move(_6), 4_usize, Move(_4), 13_usize, Move(_13), 16_usize, Move(_16)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_24 = dump_var(16_usize, 11_usize, Move(_11), 9_usize, Move(_9), 20_usize, Move(_20), 25_usize, _25), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: bool,mut _7: u16,mut _8: isize,mut _9: u16,mut _10: u16,mut _11: u16) -> usize {
mir! {
type RET = usize;
let _12: isize;
let _13: isize;
let _14: [u32; 7];
let _15: u64;
let _16: (usize, [i8; 2], bool);
let _17: Adt50;
let _18: Adt55;
let _19: Adt52;
let _20: Adt57;
let _21: (usize, [i8; 2], bool);
let _22: ([i128; 5], i16);
let _23: isize;
let _24: (([i128; 5], i16), ([u16; 5],));
let _25: f32;
let _26: isize;
let _27: (usize, [i8; 2], bool);
let _28: i32;
let _29: bool;
let _30: (char, char, i16, u16, f64);
let _31: usize;
let _32: i32;
let _33: char;
let _34: bool;
let _35: *mut (char, char, i16, u16, f64);
let _36: [isize; 4];
let _37: char;
let _38: [u32; 4];
let _39: Adt50;
let _40: ();
let _41: ();
{
_1 = _4 & _11;
_1 = 3932054977_u32 as u16;
_6 = !false;
_8 = 9223372036854775807_isize;
_3 = !_9;
_10 = !_3;
_1 = !_3;
_3 = 7069410343191259151_i64 as u16;
_3 = 2_usize as u16;
_3 = _2;
_4 = _8 as u16;
_11 = _2 + _5;
_3 = _2 ^ _1;
_12 = 95_i8 as isize;
_13 = _8;
_5 = _7;
RET = !2_usize;
_14 = [201137672_u32,2459758027_u32,215775642_u32,4019776169_u32,982098096_u32,4135367076_u32,3141212370_u32];
_4 = !_10;
RET = !6_usize;
_6 = _4 == _10;
Call(RET = core::intrinsics::transmute(_13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = (-15213_i16) as u16;
Goto(bb2)
}
bb2 = {
_15 = 5436531974642314570_u64 ^ 10107111350027052102_u64;
_4 = _1 << _3;
_12 = _13;
_8 = _13;
_8 = _12;
_11 = _4;
_14 = [4238082511_u32,4231999866_u32,1600412716_u32,1597941857_u32,480343020_u32,1868409799_u32,2793020563_u32];
_12 = _13 + _13;
RET = 1264252378379884114_usize;
_1 = _10;
_15 = (-9080871170332504435_i64) as u64;
_13 = 945827859_u32 as isize;
_3 = '\u{6f376}' as u16;
_15 = 16572026320452400341_u64;
_16.1 = [(-21_i8),(-42_i8)];
_9 = _5 >> _1;
match RET {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
1264252378379884114 => bb8,
_ => bb7
}
}
bb3 = {
_7 = (-15213_i16) as u16;
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
_3 = _6 as u16;
_6 = !true;
_4 = !_5;
_13 = (-462293760365973592_i64) as isize;
_3 = _1;
_10 = _1;
RET = _8 as usize;
_18.fld4 = 3460680175_u32 as i128;
_14 = [1423060545_u32,1388152995_u32,12067775_u32,2319485701_u32,214103980_u32,3935272690_u32,750256427_u32];
_18.fld1.fld0 = 8998303872982578212438959903641856723_u128 as u16;
_18.fld4 = (-117288635057163531453844649372847564374_i128);
_5 = _6 as u16;
_18.fld1 = Adt45 { fld0: _4 };
_20.fld6 = (RET, 8_i8, _18.fld4, _18.fld4);
_1 = !_3;
_20.fld2 = [335427295748650679583875562306160322328_u128];
_2 = _18.fld1.fld0;
_16.0 = RET;
_21.2 = _6;
_18.fld1 = Adt45 { fld0: _3 };
_17 = Adt50::Variant1 { fld0: _12 };
match _20.fld6.1 {
8 => bb9,
_ => bb5
}
}
bb9 = {
_18.fld2 = Adt54::Variant1 { fld0: _20.fld6,fld1: '\u{31732}' };
_9 = _20.fld6.3 as u16;
place!(Field::<char>(Variant(_18.fld2, 1), 1)) = '\u{ab649}';
_27.2 = !_6;
place!(Field::<isize>(Variant(_17, 1), 0)) = _8 + _8;
_18.fld4 = Field::<(usize, i8, i128, i128)>(Variant(_18.fld2, 1), 0).2 & _20.fld6.3;
place!(Field::<isize>(Variant(_17, 1), 0)) = _27.2 as isize;
_26 = 135_u8 as isize;
_27.1 = _16.1;
place!(Field::<(usize, i8, i128, i128)>(Variant(_18.fld2, 1), 0)).1 = 6_u8 as i8;
_20.fld5 = 790448289_i32;
_7 = _4 >> _10;
_16.2 = !_6;
RET = _20.fld6.0;
_20.fld4 = [204670469336158784496618484519449157160_u128,18228694408621273023648422559774548226_u128,28698452548765826111078729447232571259_u128];
SetDiscriminant(_17, 0);
_21.0 = _16.0 + Field::<(usize, i8, i128, i128)>(Variant(_18.fld2, 1), 0).0;
_30.1 = Field::<char>(Variant(_18.fld2, 1), 1);
place!(Field::<[u16; 5]>(Variant(_17, 0), 1)) = [_4,_2,_11,_7,_10];
_23 = _8 ^ _13;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4)).1 = [Field::<(usize, i8, i128, i128)>(Variant(_18.fld2, 1), 0).1,_20.fld6.1];
_30.1 = Field::<char>(Variant(_18.fld2, 1), 1);
_22.0 = [_20.fld6.3,_20.fld6.2,_20.fld6.3,_20.fld6.2,_18.fld4];
_30.2 = -(-2436_i16);
SetDiscriminant(_18.fld2, 2);
_30.1 = '\u{bfcae}';
match _20.fld6.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
8 => bb10,
_ => bb7
}
}
bb10 = {
place!(Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4)) = _16;
place!(Field::<[u32; 4]>(Variant(_17, 0), 0)) = [1671561887_u32,1308473646_u32,3251728834_u32,3926795614_u32];
_24.0.1 = _30.2 | _30.2;
_21.1 = [_20.fld6.1,_20.fld6.1];
_20.fld7 = [_10,_10,_4,_10,_7];
_30.0 = _30.1;
_23 = -_26;
_18.fld4 = _20.fld6.3 * _20.fld6.3;
_20.fld0.1.0 = Field::<[u16; 5]>(Variant(_17, 0), 1);
_28 = !_20.fld5;
_20.fld1 = Adt51::Variant2 { fld0: Field::<[u32; 4]>(Variant(_17, 0), 0),fld1: Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4).1,fld2: 22291844758271251440502345998045064721_u128 };
place!(Field::<[u32; 4]>(Variant(_17, 0), 0)) = Field::<[u32; 4]>(Variant(_20.fld1, 2), 0);
_30.4 = _15 as f64;
_16.1 = Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4).1;
_15 = 935623257296442440_u64 | 13077719479981006071_u64;
_20.fld7 = [_10,_2,_11,_1,_11];
_20.fld0.0.1 = _24.0.1 - _24.0.1;
_22.0 = [_20.fld6.3,_18.fld4,_18.fld4,_20.fld6.3,_18.fld4];
_10 = !_3;
_28 = -_20.fld5;
_21 = (RET, Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4).1, _27.2);
_26 = !_12;
_18.fld3 = Adt51::Variant2 { fld0: Field::<[u32; 4]>(Variant(_17, 0), 0),fld1: Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4).1,fld2: 321554359061140001032291788743482318889_u128 };
_20.fld0.0 = (_22.0, _24.0.1);
_24.0 = (_20.fld0.0.0, _20.fld0.0.1);
_14 = [360438024_u32,423139161_u32,3432851775_u32,138240106_u32,3075775846_u32,2918041271_u32,3270744292_u32];
Goto(bb11)
}
bb11 = {
_20.fld5 = _28 * _28;
_31 = _20.fld6.0 ^ RET;
place!(Field::<[u32; 4]>(Variant(_18.fld3, 2), 0)) = Field::<[u32; 4]>(Variant(_20.fld1, 2), 0);
_20.fld6.1 = -(-36_i8);
_20.fld0.1.0 = Field::<[u16; 5]>(Variant(_17, 0), 1);
_20.fld6 = (_21.0, 93_i8, _18.fld4, _18.fld4);
place!(Field::<i32>(Variant(_18.fld2, 2), 2)) = _20.fld6.1 as i32;
_3 = _30.4 as u16;
_13 = _26;
place!(Field::<i8>(Variant(_18.fld2, 2), 3)) = _20.fld6.1 | _20.fld6.1;
_18.fld5 = 3776977956727846816_i64 as i32;
_10 = _7 + _1;
_13 = _31 as isize;
_24.1 = _20.fld0.1;
place!(Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4)).2 = !_27.2;
_22.1 = _24.0.1;
_18.fld1 = Adt45 { fld0: _10 };
_7 = 172_u8 as u16;
_20.fld1 = Adt51::Variant2 { fld0: Field::<[u32; 4]>(Variant(_18.fld3, 2), 0),fld1: Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4).1,fld2: 49134208010224482198736320514767193618_u128 };
_20.fld6 = (_31, Field::<i8>(Variant(_18.fld2, 2), 3), _18.fld4, _18.fld4);
place!(Field::<u128>(Variant(_20.fld1, 2), 2)) = 108499413996096285685631444588100107046_u128 >> _2;
SetDiscriminant(_20.fld1, 0);
place!(Field::<Adt46>(Variant(_17, 0), 2)) = Adt46::Variant2 { fld0: _20.fld0 };
place!(Field::<u128>(Variant(_18.fld3, 2), 2)) = 338437166555592076888573844042661753538_u128;
_7 = _10;
Goto(bb12)
}
bb12 = {
place!(Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4)) = _21;
_30.0 = _30.1;
_34 = _6;
SetDiscriminant(Field::<Adt46>(Variant(_17, 0), 2), 0);
SetDiscriminant(_18.fld3, 2);
_6 = !_27.2;
match _8 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb10,
4 => bb5,
5 => bb6,
6 => bb7,
9223372036854775807 => bb13,
_ => bb8
}
}
bb13 = {
place!(Field::<([i8; 2], [i128; 5])>(Variant(_20.fld1, 0), 2)).0 = [Field::<i8>(Variant(_18.fld2, 2), 3),Field::<i8>(Variant(_18.fld2, 2), 3)];
_24.0.0 = _20.fld0.0.0;
place!(Field::<[u32; 4]>(Variant(_18.fld3, 2), 0)) = [2435454029_u32,1635301494_u32,3973161683_u32,2456572518_u32];
_20.fld0.1.0 = [_10,_2,_18.fld1.fld0,_7,_10];
_22 = _24.0;
place!(Field::<i8>(Variant(_18.fld2, 2), 3)) = _20.fld6.1;
_16 = (_31, Field::<([i8; 2], [i128; 5])>(Variant(_20.fld1, 0), 2).0, _21.2);
place!(Field::<u128>(Variant(_18.fld3, 2), 2)) = Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4).0 as u128;
_31 = _30.0 as usize;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_20.fld1, 0), 2)) = (Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4).1, _24.0.0);
RET = _21.0;
_31 = _12 as usize;
_1 = _7;
_20.fld6.2 = _18.fld4 | _20.fld6.3;
_14 = [818587410_u32,868671153_u32,2641668438_u32,3608893415_u32,449786316_u32,2981940507_u32,2641391262_u32];
place!(Field::<(usize, [i8; 2], bool)>(Variant(_17, 0), 4)).2 = _34;
match _8 {
0 => bb14,
9223372036854775807 => bb16,
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
_12 = _26;
_20.fld0.0.1 = _22.1;
place!(Field::<[u32; 4]>(Variant(_20.fld1, 0), 4)) = [2668126110_u32,3138685156_u32,910076373_u32,2490714515_u32];
place!(Field::<[u32; 7]>(Variant(_18.fld2, 2), 4)) = [2389130025_u32,2081355903_u32,617461652_u32,2573806314_u32,2696615491_u32,1017552839_u32,3985523614_u32];
place!(Field::<[u16; 5]>(Variant(_17, 0), 1)) = _20.fld7;
place!(Field::<([i8; 2], [i128; 5])>(Variant(_20.fld1, 0), 2)).0 = [Field::<i8>(Variant(_18.fld2, 2), 3),_20.fld6.1];
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(17_usize, 10_usize, Move(_10), 14_usize, Move(_14), 2_usize, Move(_2), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(17_usize, 12_usize, Move(_12), 3_usize, Move(_3), 28_usize, Move(_28), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(17_usize, 16_usize, Move(_16), 13_usize, Move(_13), 11_usize, Move(_11), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [u16; 5],mut _2: ([u16; 5],),mut _3: *mut (char, char, i16, u16, f64),mut _4: i16,mut _5: i16,mut _6: i16,mut _7: (char, char, i16, u16, f64),mut _8: f32,mut _9: *mut (char, char, i16, u16, f64),mut _10: (char, char, i16, u16, f64),mut _11: *mut (char, char, i16, u16, f64),mut _12: (char, char, i16, u16, f64),mut _13: (char, char, i16, u16, f64)) -> Adt51 {
mir! {
type RET = Adt51;
let _14: [u128; 1];
let _15: Adt41;
let _16: Adt41;
let _17: f64;
let _18: ([i128; 5], i16);
let _19: ([i8; 2], [i128; 5]);
let _20: [isize; 4];
let _21: (([i128; 5], i16), ([u16; 5],));
let _22: u32;
let _23: bool;
let _24: [u128; 1];
let _25: bool;
let _26: ();
let _27: ();
{
_14 = [266289410498333136886765280699254842579_u128];
_4 = _7.2;
_7.3 = _10.3;
_15.fld4.1 = _12.1;
_12.3 = _13.3;
_9 = _3;
_13.2 = !_6;
_16.fld4 = _10;
_15.fld4 = _12;
_15 = Adt41 { fld0: true,fld1: _10.1,fld2: _16.fld4.4,fld3: _2,fld4: _16.fld4 };
_12.4 = _15.fld4.4;
_15.fld4.0 = _12.1;
_10 = (_16.fld4.0, _13.1, _15.fld4.2, _12.3, _12.4);
_12.4 = _15.fld4.4 * _16.fld4.4;
_15 = Adt41 { fld0: true,fld1: _7.0,fld2: _12.4,fld3: _2,fld4: _10 };
_3 = core::ptr::addr_of_mut!(_12);
_12.4 = 6_usize as f64;
_14 = [29539090011329564218839082996012934128_u128];
Goto(bb1)
}
bb1 = {
_7.3 = _15.fld4.3 | (*_3).3;
_5 = _10.2 ^ _10.2;
_16.fld3 = (_1,);
_15.fld3.0 = [(*_3).3,_15.fld4.3,_13.3,_10.3,(*_3).3];
_15 = Adt41 { fld0: true,fld1: _7.1,fld2: _13.4,fld3: _2,fld4: _12 };
Goto(bb2)
}
bb2 = {
_16.fld3.0 = _1;
_17 = _13.4;
_7.2 = !(*_3).2;
_16.fld4.2 = (*_3).2 << _7.3;
_12.4 = _17;
_16.fld4.4 = _10.4 + _17;
_18.1 = _7.2;
_11 = core::ptr::addr_of_mut!(_7);
_18.0 = [(-670943950197382143273857454824237130_i128),(-76995541787144020527673342673570952111_i128),92195061073291109276187662065189253264_i128,4927921664786575482863922851568626667_i128,(-89563930289398105696714436738384387812_i128)];
_18.1 = _6 + _4;
_4 = (*_3).2;
_13.1 = _15.fld1;
_7.3 = _10.4 as u16;
Call(RET = fn19(_13.2, _11, _2, _7.2, _12.2, _2.0, _10.3, _4, _15.fld3, _15.fld3.0, _15.fld4.3, _15.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_20 = [9223372036854775807_isize,112_isize,(-9223372036854775808_isize),44_isize];
_16.fld4.4 = (*_11).4;
_21 = (_18, _16.fld3);
_16.fld4.0 = (*_11).0;
_21.0.0 = [7773594828652055228571753325447487790_i128,67489360252400100611600131622052863177_i128,(-106603649117378808585713155929252605244_i128),130294389864853008379433356752306096416_i128,(-148428736962099522784212951320699753291_i128)];
_12.4 = -_10.4;
_3 = _11;
_15.fld4 = (_12.0, _16.fld4.1, _13.2, _12.3, (*_11).4);
_13.4 = _7.4 - _17;
_7.1 = _12.0;
_14 = [Field::<u128>(Variant(RET, 2), 2)];
_7 = (_13.0, _10.1, _5, _15.fld4.3, _17);
place!(Field::<u128>(Variant(RET, 2), 2)) = 333349025947574746531763580628861939627_u128 | 12863775117049975653274075939848826633_u128;
_21.1.0 = _15.fld3.0;
_7.1 = (*_11).0;
place!(Field::<[u32; 4]>(Variant(RET, 2), 0)) = [3880783302_u32,219259094_u32,2780200763_u32,64654880_u32];
_21.0 = (_18.0, (*_11).2);
_21.1 = (_15.fld3.0,);
_22 = 4280307636_u32;
_12.4 = _15.fld2;
_12.0 = _15.fld1;
_15.fld4.4 = (-2016026688_i32) as f64;
_10.1 = _12.1;
place!(Field::<u128>(Variant(RET, 2), 2)) = (*_11).2 as u128;
_21.0.1 = _5 + _10.2;
Goto(bb4)
}
bb4 = {
Call(_26 = dump_var(18_usize, 18_usize, Move(_18), 20_usize, Move(_20), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_26 = dump_var(18_usize, 21_usize, Move(_21), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i16,mut _2: *mut (char, char, i16, u16, f64),mut _3: ([u16; 5],),mut _4: i16,mut _5: i16,mut _6: [u16; 5],mut _7: u16,mut _8: i16,mut _9: ([u16; 5],),mut _10: [u16; 5],mut _11: u16,mut _12: (char, char, i16, u16, f64)) -> Adt51 {
mir! {
type RET = Adt51;
let _13: i8;
let _14: i32;
let _15: bool;
let _16: isize;
let _17: u128;
let _18: bool;
let _19: (([i128; 5], i16), ([u16; 5],));
let _20: (char, char, i16, u16, f64);
let _21: Adt44;
let _22: (([i128; 5], i16), ([u16; 5],));
let _23: [u32; 7];
let _24: (usize, [i8; 2], bool);
let _25: [i8; 2];
let _26: u32;
let _27: bool;
let _28: u32;
let _29: ([u16; 5],);
let _30: *mut i16;
let _31: isize;
let _32: char;
let _33: (([i128; 5], i16), ([u16; 5],));
let _34: [u32; 4];
let _35: [u16; 5];
let _36: Adt48;
let _37: i128;
let _38: f32;
let _39: [i8; 2];
let _40: i64;
let _41: Adt54;
let _42: char;
let _43: bool;
let _44: bool;
let _45: Adt53;
let _46: [u32; 4];
let _47: bool;
let _48: Adt42;
let _49: (usize, i8, i128, i128);
let _50: (usize, [i8; 2], bool);
let _51: f64;
let _52: i8;
let _53: ();
let _54: ();
{
_11 = !_7;
_10 = _6;
_13 = 46_i8 ^ 54_i8;
_6 = _3.0;
_12.3 = _11;
_12.3 = !_11;
_16 = 1_isize - 9223372036854775807_isize;
_17 = 55773531690909849840453526777258643607_u128;
_11 = _12.3 << _8;
_3.0 = [_12.3,_12.3,_11,_12.3,_11];
_3 = (_10,);
_7 = 378055487_i32 as u16;
_3.0 = [_12.3,_12.3,_11,_12.3,_11];
_2 = core::ptr::addr_of_mut!(_12);
_17 = 2746504207_u32 as u128;
Goto(bb1)
}
bb1 = {
_12.1 = (*_2).0;
_6 = [_11,(*_2).3,(*_2).3,_11,_12.3];
Goto(bb2)
}
bb2 = {
_13 = !(-126_i8);
_14 = 282992182_i32;
_2 = core::ptr::addr_of_mut!((*_2));
_5 = (*_2).2;
_12.3 = !_11;
_12.0 = (*_2).1;
_3 = (_9.0,);
_18 = _8 > _5;
_6 = [_11,(*_2).3,_12.3,(*_2).3,(*_2).3];
_17 = 149075235393661658669147262671626689655_u128;
_19.0.0 = [(-21809917433636149101429361227960778412_i128),(-94343953287851119225835761190035757139_i128),(-31169077137663025186699004576350145835_i128),(-82621477910900531456315138206760850673_i128),147014733243860158040960713390506273912_i128];
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb3)
}
bb3 = {
_3.0 = _10;
_2 = core::ptr::addr_of_mut!((*_2));
_2 = core::ptr::addr_of_mut!((*_2));
_20.4 = (-17239926875132474344189348635929118160_i128) as f64;
_19.1 = _3;
_9 = _19.1;
_18 = !false;
_20 = ((*_2).0, (*_2).1, _4, _11, (*_2).4);
_9.0 = [_20.3,(*_2).3,_11,_12.3,_20.3];
_12.0 = (*_2).1;
_8 = (*_2).2 ^ (*_2).2;
_19.0.1 = _5 >> (*_2).2;
_12.1 = _20.1;
_20.2 = _14 as i16;
_13 = -(-66_i8);
_12 = (_20.0, _20.0, _5, _11, _20.4);
_10 = [(*_2).3,_20.3,(*_2).3,_12.3,_12.3];
_20 = ((*_2).0, (*_2).0, _19.0.1, _12.3, _12.4);
_24.0 = _20.0 as usize;
_12.3 = _13 as u16;
_8 = _1 - _12.2;
_22.1.0 = _3.0;
_24.2 = _18;
_19.0.0 = [(-106240563402097662714121254591087987267_i128),162363394001019975086907925583905477374_i128,156275807982227245399466124561802983752_i128,104762749302337776902632187442185103535_i128,(-36397085092178795593297167955848669128_i128)];
Goto(bb4)
}
bb4 = {
_22.0.0 = [(-52229236046963917757229500289342270630_i128),(-153099149070971505602091642731755662020_i128),99400920847435081481672805273175650457_i128,(-85103655658417817284536892215460859756_i128),(-124495920768896257639583954711747610862_i128)];
_17 = !256350574320977954944321170387100439245_u128;
_19.1 = _3;
_12.1 = (*_2).0;
_6 = _19.1.0;
_21 = Adt44::Variant1 { fld0: 247_u8 };
_21 = Adt44::Variant1 { fld0: 171_u8 };
match _14 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
282992182 => bb10,
_ => bb9
}
}
bb5 = {
_3.0 = _10;
_2 = core::ptr::addr_of_mut!((*_2));
_2 = core::ptr::addr_of_mut!((*_2));
_20.4 = (-17239926875132474344189348635929118160_i128) as f64;
_19.1 = _3;
_9 = _19.1;
_18 = !false;
_20 = ((*_2).0, (*_2).1, _4, _11, (*_2).4);
_9.0 = [_20.3,(*_2).3,_11,_12.3,_20.3];
_12.0 = (*_2).1;
_8 = (*_2).2 ^ (*_2).2;
_19.0.1 = _5 >> (*_2).2;
_12.1 = _20.1;
_20.2 = _14 as i16;
_13 = -(-66_i8);
_12 = (_20.0, _20.0, _5, _11, _20.4);
_10 = [(*_2).3,_20.3,(*_2).3,_12.3,_12.3];
_20 = ((*_2).0, (*_2).0, _19.0.1, _12.3, _12.4);
_24.0 = _20.0 as usize;
_12.3 = _13 as u16;
_8 = _1 - _12.2;
_22.1.0 = _3.0;
_24.2 = _18;
_19.0.0 = [(-106240563402097662714121254591087987267_i128),162363394001019975086907925583905477374_i128,156275807982227245399466124561802983752_i128,104762749302337776902632187442185103535_i128,(-36397085092178795593297167955848669128_i128)];
Goto(bb4)
}
bb6 = {
_13 = !(-126_i8);
_14 = 282992182_i32;
_2 = core::ptr::addr_of_mut!((*_2));
_5 = (*_2).2;
_12.3 = !_11;
_12.0 = (*_2).1;
_3 = (_9.0,);
_18 = _8 > _5;
_6 = [_11,(*_2).3,_12.3,(*_2).3,(*_2).3];
_17 = 149075235393661658669147262671626689655_u128;
_19.0.0 = [(-21809917433636149101429361227960778412_i128),(-94343953287851119225835761190035757139_i128),(-31169077137663025186699004576350145835_i128),(-82621477910900531456315138206760850673_i128),147014733243860158040960713390506273912_i128];
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb3)
}
bb7 = {
_12.1 = (*_2).0;
_6 = [_11,(*_2).3,(*_2).3,_11,_12.3];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_22 = (_19.0, _9);
_22.0 = (_19.0.0, (*_2).2);
place!(Field::<u8>(Variant(_21, 1), 0)) = 87_u8 << _11;
_22.1 = (_19.1.0,);
_16 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_8 = !(*_2).2;
_19.0.1 = _5;
SetDiscriminant(_21, 0);
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 0), 1)) = _19;
_22.0 = _19.0;
_22.1 = _3;
_28 = (*_2).4 as u32;
_24.2 = _18;
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 0), 1)).0.1 = (-6845011780553394860_i64) as i16;
_7 = _28 as u16;
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 0), 1)).1.0 = _19.1.0;
_22.1.0 = _6;
_8 = _5;
_22.0.0 = [61723024659678471618831899007327060454_i128,46566178832958848352121518435957249557_i128,158172323583442418933356195418851189478_i128,(-168420172270775386881784139960469869094_i128),(-116776789925296030493884272796599090949_i128)];
_31 = _16 >> _22.0.1;
_16 = !_31;
_29.0 = _9.0;
_20.2 = _20.3 as i16;
_12.3 = _24.0 as u16;
_19.0.1 = (*_2).2;
match _14 {
0 => bb4,
1 => bb3,
2 => bb11,
3 => bb12,
4 => bb13,
282992182 => bb15,
_ => bb14
}
}
bb11 = {
_13 = !(-126_i8);
_14 = 282992182_i32;
_2 = core::ptr::addr_of_mut!((*_2));
_5 = (*_2).2;
_12.3 = !_11;
_12.0 = (*_2).1;
_3 = (_9.0,);
_18 = _8 > _5;
_6 = [_11,(*_2).3,_12.3,(*_2).3,(*_2).3];
_17 = 149075235393661658669147262671626689655_u128;
_19.0.0 = [(-21809917433636149101429361227960778412_i128),(-94343953287851119225835761190035757139_i128),(-31169077137663025186699004576350145835_i128),(-82621477910900531456315138206760850673_i128),147014733243860158040960713390506273912_i128];
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb3)
}
bb12 = {
Return()
}
bb13 = {
_12.1 = (*_2).0;
_6 = [_11,(*_2).3,(*_2).3,_11,_12.3];
Goto(bb2)
}
bb14 = {
_13 = !(-126_i8);
_14 = 282992182_i32;
_2 = core::ptr::addr_of_mut!((*_2));
_5 = (*_2).2;
_12.3 = !_11;
_12.0 = (*_2).1;
_3 = (_9.0,);
_18 = _8 > _5;
_6 = [_11,(*_2).3,_12.3,(*_2).3,(*_2).3];
_17 = 149075235393661658669147262671626689655_u128;
_19.0.0 = [(-21809917433636149101429361227960778412_i128),(-94343953287851119225835761190035757139_i128),(-31169077137663025186699004576350145835_i128),(-82621477910900531456315138206760850673_i128),147014733243860158040960713390506273912_i128];
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb3)
}
bb15 = {
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 0), 1)).0.1 = _1;
_21 = Adt44::Variant1 { fld0: 142_u8 };
_22.0 = (_19.0.0, _1);
_28 = !13062785_u32;
_27 = _18;
Goto(bb16)
}
bb16 = {
_20 = ((*_2).0, _12.0, (*_2).2, _11, (*_2).4);
_14 = -1776836568_i32;
_24.2 = !_27;
_28 = _20.3 as u32;
_31 = _16;
_9 = (_19.1.0,);
_19.0.1 = (-167660637746735081835185072086152094483_i128) as i16;
_12.3 = _18 as u16;
_25 = [_13,_13];
_25 = [_13,_13];
_14 = !1547033855_i32;
_12.2 = -_20.2;
place!(Field::<u8>(Variant(_21, 1), 0)) = !27_u8;
_30 = core::ptr::addr_of_mut!(_8);
Goto(bb17)
}
bb17 = {
_24 = (15206438001123763459_usize, _25, _18);
_23 = [_28,_28,_28,_28,_28,_28,_28];
_9 = (_6,);
_33.0 = _22.0;
_24.2 = _18 & _27;
_27 = _20.3 < _11;
_33.0 = _19.0;
SetDiscriminant(_21, 2);
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)).0.1 = -_8;
_33 = _22;
_24.1 = [_13,_13];
_29 = (_22.1.0,);
_31 = _16;
_33.1.0 = [_20.3,_11,_11,_11,_20.3];
Goto(bb18)
}
bb18 = {
_19.1.0 = [_11,_11,_20.3,_12.3,_11];
_6 = [_11,_11,_20.3,_20.3,_20.3];
_24.1 = _25;
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)).0 = (_22.0.0, _4);
_31 = _16 >> _33.0.1;
_20.4 = (*_2).4;
_3.0 = [_11,_11,_11,_20.3,_11];
place!(Field::<u8>(Variant(_21, 2), 4)) = 180_u8;
_7 = _20.3 * _20.3;
_36.fld1 = [_16,_31,_16,_16];
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)).0.0 = [(-73134580359967809636966171601801288108_i128),(-116291951225993045746444512097701829634_i128),146593263156267173799957600891014989233_i128,(-123282342230016195628514939071130622308_i128),(-167562471654197550165526224622601174248_i128)];
_12.4 = -_20.4;
_24 = (16598665270129470323_usize, _25, _27);
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)) = (_33.0, _3);
_4 = _1 + (*_30);
_19.1.0 = Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6).1.0;
_12.1 = _12.0;
_25 = _24.1;
_8 = 17534106118512444940_u64 as i16;
_36.fld3.0 = [_11,_20.3,_11,_11,_7];
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)) = _22;
place!(Field::<u16>(Variant(_21, 2), 3)) = _27 as u16;
_12.2 = _28 as i16;
Call(_11 = core::intrinsics::bswap(Field::<u16>(Variant(_21, 2), 3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_22.1.0 = [_11,Field::<u16>(Variant(_21, 2), 3),Field::<u16>(Variant(_21, 2), 3),_11,_20.3];
place!(Field::<u8>(Variant(_21, 2), 4)) = 211_u8;
Goto(bb20)
}
bb20 = {
_19.0.0 = [94486380673433244301386487250097277840_i128,159139416769299134642704093716851989619_i128,(-131915702562361448111607162521989458391_i128),30735468678653467510842215333590454625_i128,118691369377181827654987780290929420606_i128];
match _24.0 {
0 => bb14,
1 => bb15,
2 => bb13,
3 => bb4,
16598665270129470323 => bb22,
_ => bb21
}
}
bb21 = {
_12.1 = (*_2).0;
_6 = [_11,(*_2).3,(*_2).3,_11,_12.3];
Goto(bb2)
}
bb22 = {
_3.0 = _36.fld3.0;
_15 = _27;
_30 = core::ptr::addr_of_mut!(_22.0.1);
_20.4 = -(*_2).4;
_40 = -(-3394236730709074199_i64);
_37 = 155551582036907711883804325537773795843_i128 + 133886988276922792832505440775146718559_i128;
_33.1 = _36.fld3;
_3 = (Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6).1.0,);
_3.0 = [_20.3,Field::<u16>(Variant(_21, 2), 3),_11,Field::<u16>(Variant(_21, 2), 3),Field::<u16>(Variant(_21, 2), 3)];
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)).0 = (_22.0.0, _1);
_18 = _20.2 < _20.2;
_7 = !_20.3;
_16 = !_31;
_22.0.0 = Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6).0.0;
place!(Field::<u16>(Variant(_21, 2), 3)) = _11;
_20.2 = (*_2).2;
_9 = _19.1;
_20.0 = (*_2).0;
Goto(bb23)
}
bb23 = {
_6 = _10;
_5 = !(*_30);
place!(Field::<([i8; 2], [i128; 5])>(Variant(_21, 2), 5)).0 = [_13,_13];
_37 = (-142608738194725461400875321608942695052_i128);
_21 = Adt44::Variant1 { fld0: 117_u8 };
_23 = [_28,_28,_28,_28,_28,_28,_28];
_12.0 = _20.1;
_36.fld0 = _37;
_46 = [_28,_28,_28,_28];
_43 = !_18;
_37 = _36.fld0;
_19.1.0 = [_7,_20.3,_11,_20.3,_11];
_17 = 250587631306891220039450846231795594227_u128;
match _24.0 {
0 => bb20,
1 => bb24,
2 => bb25,
3 => bb26,
16598665270129470323 => bb28,
_ => bb27
}
}
bb24 = {
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 0), 1)).0.1 = _1;
_21 = Adt44::Variant1 { fld0: 142_u8 };
_22.0 = (_19.0.0, _1);
_28 = !13062785_u32;
_27 = _18;
Goto(bb16)
}
bb25 = {
_12.1 = (*_2).0;
_6 = [_11,(*_2).3,(*_2).3,_11,_12.3];
Goto(bb2)
}
bb26 = {
_12.1 = (*_2).0;
_6 = [_11,(*_2).3,(*_2).3,_11,_12.3];
Goto(bb2)
}
bb27 = {
_13 = !(-126_i8);
_14 = 282992182_i32;
_2 = core::ptr::addr_of_mut!((*_2));
_5 = (*_2).2;
_12.3 = !_11;
_12.0 = (*_2).1;
_3 = (_9.0,);
_18 = _8 > _5;
_6 = [_11,(*_2).3,_12.3,(*_2).3,(*_2).3];
_17 = 149075235393661658669147262671626689655_u128;
_19.0.0 = [(-21809917433636149101429361227960778412_i128),(-94343953287851119225835761190035757139_i128),(-31169077137663025186699004576350145835_i128),(-82621477910900531456315138206760850673_i128),147014733243860158040960713390506273912_i128];
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb3)
}
bb28 = {
_32 = _12.0;
_22.0.0 = _19.0.0;
_30 = core::ptr::addr_of_mut!((*_2).2);
match _24.0 {
0 => bb29,
1 => bb30,
2 => bb31,
3 => bb32,
16598665270129470323 => bb34,
_ => bb33
}
}
bb29 = {
_3.0 = _10;
_2 = core::ptr::addr_of_mut!((*_2));
_2 = core::ptr::addr_of_mut!((*_2));
_20.4 = (-17239926875132474344189348635929118160_i128) as f64;
_19.1 = _3;
_9 = _19.1;
_18 = !false;
_20 = ((*_2).0, (*_2).1, _4, _11, (*_2).4);
_9.0 = [_20.3,(*_2).3,_11,_12.3,_20.3];
_12.0 = (*_2).1;
_8 = (*_2).2 ^ (*_2).2;
_19.0.1 = _5 >> (*_2).2;
_12.1 = _20.1;
_20.2 = _14 as i16;
_13 = -(-66_i8);
_12 = (_20.0, _20.0, _5, _11, _20.4);
_10 = [(*_2).3,_20.3,(*_2).3,_12.3,_12.3];
_20 = ((*_2).0, (*_2).0, _19.0.1, _12.3, _12.4);
_24.0 = _20.0 as usize;
_12.3 = _13 as u16;
_8 = _1 - _12.2;
_22.1.0 = _3.0;
_24.2 = _18;
_19.0.0 = [(-106240563402097662714121254591087987267_i128),162363394001019975086907925583905477374_i128,156275807982227245399466124561802983752_i128,104762749302337776902632187442185103535_i128,(-36397085092178795593297167955848669128_i128)];
Goto(bb4)
}
bb30 = {
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 0), 1)).0.1 = _1;
_21 = Adt44::Variant1 { fld0: 142_u8 };
_22.0 = (_19.0.0, _1);
_28 = !13062785_u32;
_27 = _18;
Goto(bb16)
}
bb31 = {
_19.1.0 = [_11,_11,_20.3,_12.3,_11];
_6 = [_11,_11,_20.3,_20.3,_20.3];
_24.1 = _25;
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)).0 = (_22.0.0, _4);
_31 = _16 >> _33.0.1;
_20.4 = (*_2).4;
_3.0 = [_11,_11,_11,_20.3,_11];
place!(Field::<u8>(Variant(_21, 2), 4)) = 180_u8;
_7 = _20.3 * _20.3;
_36.fld1 = [_16,_31,_16,_16];
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)).0.0 = [(-73134580359967809636966171601801288108_i128),(-116291951225993045746444512097701829634_i128),146593263156267173799957600891014989233_i128,(-123282342230016195628514939071130622308_i128),(-167562471654197550165526224622601174248_i128)];
_12.4 = -_20.4;
_24 = (16598665270129470323_usize, _25, _27);
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)) = (_33.0, _3);
_4 = _1 + (*_30);
_19.1.0 = Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6).1.0;
_12.1 = _12.0;
_25 = _24.1;
_8 = 17534106118512444940_u64 as i16;
_36.fld3.0 = [_11,_20.3,_11,_11,_7];
place!(Field::<(([i128; 5], i16), ([u16; 5],))>(Variant(_21, 2), 6)) = _22;
place!(Field::<u16>(Variant(_21, 2), 3)) = _27 as u16;
_12.2 = _28 as i16;
Call(_11 = core::intrinsics::bswap(Field::<u16>(Variant(_21, 2), 3)), ReturnTo(bb19), UnwindUnreachable())
}
bb32 = {
_20 = ((*_2).0, _12.0, (*_2).2, _11, (*_2).4);
_14 = -1776836568_i32;
_24.2 = !_27;
_28 = _20.3 as u32;
_31 = _16;
_9 = (_19.1.0,);
_19.0.1 = (-167660637746735081835185072086152094483_i128) as i16;
_12.3 = _18 as u16;
_25 = [_13,_13];
_25 = [_13,_13];
_14 = !1547033855_i32;
_12.2 = -_20.2;
place!(Field::<u8>(Variant(_21, 1), 0)) = !27_u8;
_30 = core::ptr::addr_of_mut!(_8);
Goto(bb17)
}
bb33 = {
_13 = !(-126_i8);
_14 = 282992182_i32;
_2 = core::ptr::addr_of_mut!((*_2));
_5 = (*_2).2;
_12.3 = !_11;
_12.0 = (*_2).1;
_3 = (_9.0,);
_18 = _8 > _5;
_6 = [_11,(*_2).3,_12.3,(*_2).3,(*_2).3];
_17 = 149075235393661658669147262671626689655_u128;
_19.0.0 = [(-21809917433636149101429361227960778412_i128),(-94343953287851119225835761190035757139_i128),(-31169077137663025186699004576350145835_i128),(-82621477910900531456315138206760850673_i128),147014733243860158040960713390506273912_i128];
_2 = core::ptr::addr_of_mut!(_12);
Goto(bb3)
}
bb34 = {
_35 = _6;
_9 = (_22.1.0,);
_47 = (*_2).2 >= _1;
_50.0 = !_24.0;
_22.0.0 = [_37,_36.fld0,_36.fld0,_37,_36.fld0];
_33.1 = (_3.0,);
_26 = _28 & _28;
_39 = _25;
_44 = !_24.2;
_18 = _24.2;
_29.0 = _10;
_22.0.0 = _33.0.0;
_2 = core::ptr::addr_of_mut!((*_2));
_20.0 = _32;
_49.2 = _24.0 as i128;
Goto(bb35)
}
bb35 = {
_14 = !1007150989_i32;
_42 = _20.1;
_38 = _17 as f32;
RET = Adt51::Variant2 { fld0: _46,fld1: _24.1,fld2: _17 };
_27 = _24.0 >= _24.0;
_22.1.0 = [_7,_7,_11,_20.3,_20.3];
_47 = _28 > _26;
_20.4 = _49.2 as f64;
_50.1 = [_13,_13];
_19.1 = (_6,);
_33.0.0 = [_49.2,_49.2,_49.2,_49.2,_49.2];
_25 = [_13,_13];
_19.0.1 = _4 + _1;
_10 = _22.1.0;
_22.0.0 = _33.0.0;
_46 = [_28,_26,_26,_28];
place!(Field::<[i8; 2]>(Variant(RET, 2), 1)) = [_13,_13];
_12.2 = _33.0.1 & _4;
_33 = (_19.0, _36.fld3);
_20.3 = _20.2 as u16;
_31 = -_16;
_36.fld1 = [_31,_31,_31,_16];
_38 = 52_u8 as f32;
_33.0.1 = _12.2;
_49 = (_50.0, _13, _36.fld0, _36.fld0);
_11 = _7 >> _31;
_50.0 = _24.0 - _24.0;
_22 = (_33.0, _19.1);
_14 = 1895170686_i32 >> _4;
Goto(bb36)
}
bb36 = {
Call(_53 = dump_var(19_usize, 40_usize, Move(_40), 4_usize, Move(_4), 17_usize, Move(_17), 1_usize, Move(_1)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_53 = dump_var(19_usize, 13_usize, Move(_13), 49_usize, Move(_49), 43_usize, Move(_43), 24_usize, Move(_24)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_53 = dump_var(19_usize, 14_usize, Move(_14), 47_usize, Move(_47), 10_usize, Move(_10), 19_usize, Move(_19)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_53 = dump_var(19_usize, 9_usize, Move(_9), 22_usize, Move(_22), 37_usize, Move(_37), 46_usize, Move(_46)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_53 = dump_var(19_usize, 35_usize, Move(_35), 16_usize, Move(_16), 15_usize, Move(_15), 54_usize, _54), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(441437153_i32), std::hint::black_box('\u{b8fde}'), std::hint::black_box(64739_u16), std::hint::black_box(27623_i16));
                
            }
#[derive(Debug)]
pub struct Adt41 {
fld0: bool,
fld1: char,
fld2: f64,
fld3: ([u16; 5],),
fld4: (char, char, i16, u16, f64),
}
#[derive(Debug)]
pub enum Adt42 {
Variant0{
fld0: ([i8; 2], [i128; 5]),
fld1: ([i128; 5], i16),
fld2: isize,
fld3: *const *mut f64,
fld4: usize,
fld5: [i128; 5],
fld6: *mut (char, char, i16, u16, f64),

},
Variant1{
fld0: i16,
fld1: *const *mut f64,

},
Variant2{
fld0: [u128; 3],
fld1: [u32; 4],
fld2: ([u16; 5],),
fld3: ([i8; 2], [i128; 5]),
fld4: f64,
fld5: i32,

},
Variant3{
fld0: [u16; 5],
fld1: *const (char, char, i16, u16, f64),

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt43 {
Variant0{
fld0: u32,
fld1: [u32; 4],
fld2: f64,

},
Variant1{
fld0: [isize; 4],
fld1: *mut (char, char, i16, u16, f64),

}}
#[derive(Debug)]
pub enum Adt44 {
Variant0{
fld0: u8,
fld1: (([i128; 5], i16), ([u16; 5],)),
fld2: u128,
fld3: *const *mut f64,
fld4: f32,
fld5: i32,

},
Variant1{
fld0: u8,

},
Variant2{
fld0: bool,
fld1: *mut f64,
fld2: Adt42,
fld3: u16,
fld4: u8,
fld5: ([i8; 2], [i128; 5]),
fld6: (([i128; 5], i16), ([u16; 5],)),

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt45 {
fld0: u16,
}
#[derive(Debug)]
pub enum Adt46 {
Variant0{
fld0: bool,
fld1: *mut usize,

},
Variant1{
fld0: u128,
fld1: f32,
fld2: (char, char, i16, u16, f64),
fld3: Adt43,
fld4: *mut usize,
fld5: Adt44,

},
Variant2{
fld0: (([i128; 5], i16), ([u16; 5],)),

},
Variant3{
fld0: i64,
fld1: (usize, [i8; 2], bool),
fld2: *mut i16,
fld3: *const (char, char, i16, u16, f64),

}}
#[derive(Debug)]
pub enum Adt47 {
Variant0{
fld0: bool,
fld1: *const (char, char, i16, u16, f64),
fld2: [u32; 4],
fld3: [isize; 4],
fld4: ([u16; 5],),
fld5: u128,
fld6: i64,
fld7: [u32; 7],

},
Variant1{
fld0: u64,
fld1: f32,
fld2: *mut i16,
fld3: ([u16; 5],),
fld4: (char, char, i16, u16, f64),
fld5: ([i128; 5], i16),

},
Variant2{
fld0: ([u16; 5],),

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt48 {
fld0: i128,
fld1: [isize; 4],
fld2: u8,
fld3: ([u16; 5],),
}
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: i16,
fld1: ([i128; 5], i16),
fld2: Adt44,

},
Variant1{
fld0: Adt47,
fld1: char,
fld2: [i8; 2],
fld3: [i128; 5],
fld4: usize,
fld5: ([u16; 5],),
fld6: i64,
fld7: Adt43,

}}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: [u32; 4],
fld1: [u16; 5],
fld2: Adt46,
fld3: Adt49,
fld4: (usize, [i8; 2], bool),

},
Variant1{
fld0: isize,

},
Variant2{
fld0: u64,
fld1: (usize, [i8; 2], bool),
fld2: *const (char, char, i16, u16, f64),

},
Variant3{
fld0: Adt43,
fld1: Adt48,
fld2: [i8; 2],
fld3: [i128; 5],
fld4: Adt45,
fld5: ([u16; 5],),

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: bool,
fld1: Adt42,
fld2: ([i8; 2], [i128; 5]),
fld3: u32,
fld4: [u32; 4],
fld5: i32,

},
Variant1{
fld0: *const *mut f64,
fld1: char,
fld2: [u32; 7],
fld3: ([i8; 2], [i128; 5]),
fld4: Adt50,

},
Variant2{
fld0: [u32; 4],
fld1: [i8; 2],
fld2: u128,

}}
#[derive(Debug)]
pub enum Adt52 {
Variant0{
fld0: i128,

},
Variant1{
fld0: [u16; 5],
fld1: (char, char, i16, u16, f64),
fld2: isize,
fld3: *mut usize,
fld4: Adt46,

},
Variant2{
fld0: *mut usize,
fld1: ([i8; 2], [i128; 5]),
fld2: u64,
fld3: i128,
fld4: u16,

},
Variant3{
fld0: isize,
fld1: Adt44,

}}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: *const *mut f64,
fld1: [isize; 4],
fld2: *mut (char, char, i16, u16, f64),
fld3: [u32; 4],
fld4: i16,
fld5: [u16; 5],
fld6: Adt44,
fld7: u64,

},
Variant1{
fld0: (([i128; 5], i16), ([u16; 5],)),
fld1: *mut usize,
fld2: Adt43,
fld3: u128,
fld4: [i8; 2],
fld5: Adt48,
fld6: i64,
fld7: Adt49,

}}
#[derive(Debug)]
pub enum Adt54 {
Variant0{
fld0: bool,
fld1: Adt48,
fld2: (([i128; 5], i16), ([u16; 5],)),
fld3: i64,
fld4: i16,
fld5: u8,

},
Variant1{
fld0: (usize, i8, i128, i128),
fld1: char,

},
Variant2{
fld0: Adt43,
fld1: Adt44,
fld2: i32,
fld3: i8,
fld4: [u32; 7],

}}
#[derive(Debug)]
pub struct Adt55 {
fld0: Adt52,
fld1: Adt45,
fld2: Adt54,
fld3: Adt51,
fld4: i128,
fld5: i32,
}
#[derive(Debug)]
pub enum Adt56 {
Variant0{
fld0: [isize; 4],

},
Variant1{
fld0: Adt48,

},
Variant2{
fld0: u32,
fld1: usize,
fld2: [u32; 7],
fld3: *mut (char, char, i16, u16, f64),
fld4: (usize, [i8; 2], bool),
fld5: Adt54,
fld6: Adt47,
fld7: f64,

},
Variant3{
fld0: bool,
fld1: char,
fld2: *mut (char, char, i16, u16, f64),
fld3: [u128; 1],
fld4: Adt42,
fld5: (([i128; 5], i16), ([u16; 5],)),
fld6: [isize; 4],

}}
#[derive(Debug)]
pub struct Adt57 {
fld0: (([i128; 5], i16), ([u16; 5],)),
fld1: Adt51,
fld2: [u128; 1],
fld3: i8,
fld4: [u128; 3],
fld5: i32,
fld6: (usize, i8, i128, i128),
fld7: [u16; 5],
}

