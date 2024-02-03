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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> i128 {
mir! {
type RET = i128;
let _15: Adt47;
let _16: [u32; 5];
let _17: Adt47;
let _18: u32;
let _19: u128;
let _20: (u16,);
let _21: *const *mut (u16,);
let _22: Adt49;
let _23: [u32; 5];
let _24: Adt47;
let _25: Adt43;
let _26: i8;
let _27: f64;
let _28: [usize; 7];
let _29: ([usize; 8],);
let _30: ([usize; 8],);
let _31: [i32; 3];
let _32: i8;
let _33: i64;
let _34: [i32; 3];
let _35: Adt47;
let _36: ();
let _37: ();
{
_10 = 181_u8 - 53_u8;
_3 = 13_isize - 99_isize;
_1 = true;
_9 = 15601167427024735064_u64 as usize;
_8 = !135581680942914817299211264421239592870_i128;
_13 = _10 as u64;
_6 = 62685_u16 as i32;
_2 = '\u{8d53e}';
RET = -_8;
_4 = _6 as i8;
_14 = 59338985791251260691492170101774048418_u128;
_13 = _10 as u64;
_8 = !RET;
_5 = 4459411631638452017_i64 as i16;
_10 = 139_u8;
_13 = 8302640954329844823_u64 ^ 4452687048859617230_u64;
_11 = 18169_u16 - 50557_u16;
_15 = Adt47::Variant1 { fld0: _14 };
_8 = _1 as i128;
_16 = [1491101154_u32,4204299693_u32,2014670837_u32,3741918388_u32,2239832198_u32];
_5 = 9619_i16;
_17 = Move(_15);
_16 = [1377213921_u32,1905058599_u32,2958107869_u32,3839200949_u32,469914976_u32];
_13 = 17411582691020567704_u64 + 15271336224152003178_u64;
_6 = _10 as i32;
_8 = _2 as i128;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
9619 => bb6,
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
place!(Field::<u128>(Variant(_17, 1), 0)) = _14;
RET = -_8;
_6 = -314240681_i32;
SetDiscriminant(_17, 0);
_11 = !39571_u16;
_6 = (-321580636_i32) - 1621398287_i32;
_11 = !57516_u16;
_15 = Adt47::Variant1 { fld0: _14 };
_17 = Adt47::Variant1 { fld0: _14 };
_9 = !6820037998739343234_usize;
_10 = _13 as u8;
_19 = _9 as u128;
_20.0 = !_11;
_19 = !Field::<u128>(Variant(_17, 1), 0);
match _5 {
0 => bb7,
1 => bb8,
9619 => bb10,
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
place!(Field::<u128>(Variant(_15, 1), 0)) = !_14;
_9 = !3_usize;
RET = 3329688042_u32 as i128;
_18 = 3793747338_u32 & 1723040736_u32;
_13 = !9942016605184546129_u64;
_8 = _4 as i128;
_5 = !13103_i16;
_7 = (-450964159097978553_i64) ^ 9292599873333552_i64;
_6 = _4 as i32;
place!(Field::<u128>(Variant(_15, 1), 0)) = _4 as u128;
_13 = !16521197411527133080_u64;
_7 = 6674218041834673650_i64;
_20.0 = !_11;
_24 = Adt47::Variant0 { fld0: _20.0 };
place!(Field::<u16>(Variant(_24, 0), 0)) = _10 as u16;
_3 = Field::<u16>(Variant(_24, 0), 0) as isize;
_5 = (-1071_i16);
_12 = _18;
place!(Field::<u128>(Variant(_17, 1), 0)) = Field::<u128>(Variant(_15, 1), 0);
_6 = (-879004119_i32);
_7 = 8793437390596582318_i64 - (-2296084582746549083_i64);
_11 = !Field::<u16>(Variant(_24, 0), 0);
_24 = Adt47::Variant0 { fld0: _11 };
_10 = 212_u8 + 133_u8;
_3 = !(-9223372036854775808_isize);
_25.fld4.fld4 = core::ptr::addr_of!(_20);
Call(_25.fld4.fld0 = fn1(Field::<u16>(Variant(_24, 0), 0), _7, _1, _16, _12, _3, Field::<u16>(Variant(_24, 0), 0), Field::<u128>(Variant(_17, 1), 0), _19), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_24 = Adt47::Variant0 { fld0: _11 };
_7 = Field::<u128>(Variant(_17, 1), 0) as i64;
_25.fld3 = [_13,_13,_13,_13,_13,_13,_13,_13];
Call(_20.0 = core::intrinsics::bswap(Field::<u16>(Variant(_24, 0), 0)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_2 = '\u{10cf88}';
_7 = (-6026501647072910992_i64);
_25.fld6 = _6 as u16;
_17 = Adt47::Variant0 { fld0: _11 };
_25.fld4.fld3.0 = RET << _18;
_7 = -(-6079456099057625174_i64);
_25.fld0 = Adt40::Variant3 { fld0: _25.fld4.fld4 };
_15 = Adt47::Variant0 { fld0: _25.fld6 };
RET = Field::<u16>(Variant(_24, 0), 0) as i128;
_6 = (-1209673672_i32);
_7 = (-748193196110965404_i64);
_19 = Field::<u16>(Variant(_24, 0), 0) as u128;
_25.fld7 = !_13;
_28 = [_9,_9,_9,_9,_9,_9,_9];
_14 = !_19;
_7 = 6938998971890418343_i64;
_12 = _3 as u32;
_9 = !0_usize;
_5 = (-27226_i16);
_29.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10 = !37_u8;
_25.fld5.0 = _29.0;
_16 = [_18,_12,_18,_18,_12];
_12 = _18;
Goto(bb13)
}
bb13 = {
RET = !_25.fld4.fld3.0;
_12 = _18;
_23 = [_18,_12,_18,_12,_12];
_15 = Move(_17);
_30 = (_25.fld5.0,);
place!(Field::<*const (u16,)>(Variant(_25.fld0, 3), 0)) = _25.fld4.fld4;
SetDiscriminant(_15, 0);
Goto(bb14)
}
bb14 = {
_10 = !116_u8;
SetDiscriminant(_25.fld0, 2);
_25.fld6 = _11 ^ Field::<u16>(Variant(_24, 0), 0);
place!(Field::<i64>(Variant(_25.fld0, 2), 0)) = !_7;
_24 = Adt47::Variant0 { fld0: _25.fld6 };
_29.0 = _30.0;
_13 = _25.fld7 << RET;
place!(Field::<u16>(Variant(_24, 0), 0)) = !_25.fld6;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(0_usize, 1_usize, Move(_1), 11_usize, Move(_11), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(0_usize, 4_usize, Move(_4), 14_usize, Move(_14), 16_usize, Move(_16), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(0_usize, 7_usize, Move(_7), 12_usize, Move(_12), 30_usize, Move(_30), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u16,mut _2: i64,mut _3: bool,mut _4: [u32; 5],mut _5: u32,mut _6: isize,mut _7: u16,mut _8: u128,mut _9: u128) -> *const bool {
mir! {
type RET = *const bool;
let _10: [i64; 5];
let _11: u32;
let _12: u128;
let _13: [u64; 8];
let _14: u128;
let _15: i128;
let _16: isize;
let _17: [u64; 8];
let _18: [u64; 8];
let _19: *mut (*mut (u16,), u8, char);
let _20: Adt53;
let _21: Adt49;
let _22: (u16,);
let _23: [i32; 3];
let _24: [usize; 7];
let _25: bool;
let _26: [u32; 5];
let _27: Adt55;
let _28: [u64; 8];
let _29: ();
let _30: ();
{
RET = core::ptr::addr_of!(_3);
_9 = _8;
_3 = false;
_9 = _6 as u128;
_7 = '\u{441a3}' as u16;
_2 = 2369988011610998773_i64;
RET = core::ptr::addr_of!((*RET));
_10 = [_2,_2,_2,_2,_2];
_5 = 2143355863_u32 | 1654643795_u32;
_8 = _9;
_5 = 167494242_u32;
_6 = 9223372036854775807_isize;
_1 = _2 as u16;
_7 = !_1;
_11 = 6_usize as u32;
_6 = 10887_i16 as isize;
_10 = [_2,_2,_2,_2,_2];
_8 = (-144206597612592589672054401297566823345_i128) as u128;
_6 = _7 as isize;
_11 = !_5;
_2 = (-112_i8) as i64;
_3 = false | true;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
167494242 => bb8,
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
_8 = !_9;
_14 = _8 ^ _9;
_15 = !(-108041734581870484917010746374161007145_i128);
_6 = 3_usize as isize;
_7 = !_1;
Call(_2 = fn2(_14, (*RET), _6, (*RET), _15, RET, _5, _11), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = core::ptr::addr_of!((*RET));
_11 = !_5;
_15 = 30_i8 as i128;
_10 = [_2,_2,_2,_2,_2];
_4 = [_11,_5,_11,_5,_11];
_8 = _14 - _9;
_5 = _11 * _11;
_16 = (-74_i8) as isize;
_11 = _1 as u32;
_11 = _5;
_17 = [8241317078534739500_u64,10659746630596140554_u64,15613495744805947672_u64,15223919152800584021_u64,9649897440742247771_u64,16263299193443415343_u64,10049491766470777941_u64,8628135200288915310_u64];
_11 = (-2979_i16) as u32;
_13 = _17;
_7 = _1;
RET = core::ptr::addr_of!((*RET));
_3 = true;
_6 = 15_i8 as isize;
Goto(bb10)
}
bb10 = {
_1 = !_7;
_5 = _11;
_22 = (_1,);
_13 = _17;
_12 = _14 << _5;
_16 = _6 | _6;
RET = core::ptr::addr_of!((*RET));
_18 = [5419106185705827694_u64,4080005525714140955_u64,15150130004620025784_u64,2685103837570935929_u64,12603960390754977689_u64,17729104620424710122_u64,14992638369528717196_u64,3626284615516283831_u64];
_17 = [12897269501303534418_u64,1039107779752156550_u64,5539622783977355157_u64,2451153027293809569_u64,6006748059840045276_u64,3437207596860762092_u64,11539385527901152579_u64,10405967900251661318_u64];
_22.0 = 5717878730320812722_usize as u16;
_10 = [_2,_2,_2,_2,_2];
_9 = _1 as u128;
_9 = _12;
RET = core::ptr::addr_of!((*RET));
_3 = _8 != _9;
_2 = (-1005003485251115245_i64);
_11 = _5;
match _2 {
340282366920938463462369603946517096211 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_6 = _16 - _16;
_24 = [16196731222020349140_usize,7_usize,7_usize,15799032415949216486_usize,4_usize,8188353673415062922_usize,4_usize];
_15 = (-38033860475378990263144560242090830053_i128);
_26 = [_11,_5,_5,_11,_5];
RET = core::ptr::addr_of!(_3);
_22.0 = !_7;
_8 = 0_usize as u128;
_15 = -(-115759902498683967975466097577112053007_i128);
_22 = (_7,);
_17 = _13;
_27.fld0 = -716414953_i32;
_25 = _3 > (*RET);
RET = core::ptr::addr_of!((*RET));
_5 = _11;
RET = core::ptr::addr_of!((*RET));
_5 = _15 as u32;
_12 = !_8;
Goto(bb13)
}
bb13 = {
_22 = (_1,);
_12 = !_9;
_25 = (*RET);
_6 = _16 | _16;
_17 = [9387743060251128025_u64,8493130838371055211_u64,11528984040973784087_u64,12554857780611180017_u64,8091889216328941402_u64,5239281126996076888_u64,8486224469935803814_u64,6006912027375332678_u64];
match _2 {
340282366920938463462369603946517096211 => bb14,
_ => bb4
}
}
bb14 = {
_2 = !1937286411544103576_i64;
_4 = _26;
_3 = _25;
_22.0 = !_7;
_3 = !_25;
_1 = _22.0;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(1_usize, 11_usize, Move(_11), 26_usize, Move(_26), 17_usize, Move(_17), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(1_usize, 24_usize, Move(_24), 4_usize, Move(_4), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(1_usize, 25_usize, Move(_25), 5_usize, Move(_5), 13_usize, Move(_13), 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u128,mut _2: bool,mut _3: isize,mut _4: bool,mut _5: i128,mut _6: *const bool,mut _7: u32,mut _8: u32) -> i64 {
mir! {
type RET = i64;
let _9: isize;
let _10: [u64; 8];
let _11: f32;
let _12: [u32; 5];
let _13: Adt47;
let _14: Adt52;
let _15: (i128,);
let _16: i32;
let _17: isize;
let _18: Adt56;
let _19: bool;
let _20: i32;
let _21: Adt44;
let _22: [i64; 5];
let _23: isize;
let _24: usize;
let _25: ();
let _26: ();
{
RET = 15838975076391634627_u64 as i64;
_2 = !_4;
_3 = 65_u8 as isize;
RET = 6175544461046468469_i64;
_6 = core::ptr::addr_of!(_2);
_6 = core::ptr::addr_of!((*_6));
_3 = 1241406759555231461_usize as isize;
_4 = (*_6);
RET = 9213155721712628103_i64 >> _1;
_3 = (-9223372036854775808_isize);
_6 = core::ptr::addr_of!((*_6));
_4 = (*_6) ^ (*_6);
_7 = '\u{1027cd}' as u32;
RET = 4860525859740940466_usize as i64;
_4 = !(*_6);
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463454151235394913435648 => bb8,
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
_9 = _3 >> _1;
_9 = 24558_i16 as isize;
_9 = _3;
_4 = (*_6);
_2 = !_4;
_1 = 68984724902562439652906646153071118564_u128 | 261642822206128200681847563322776370457_u128;
RET = (-1811223719961532439_i64) ^ 8425974213353469757_i64;
_1 = (-124_i8) as u128;
_3 = _8 as isize;
_10 = [1471817956065522596_u64,9866717794501884422_u64,12294422992037561791_u64,15502022051003924391_u64,5440729524223130023_u64,2592066975649058715_u64,10674804968867955456_u64,11604425163907205441_u64];
_8 = _7;
_3 = _9 | _9;
_8 = _7 << _1;
_10 = [9923171403762542284_u64,13270378599666045278_u64,4082048417361329268_u64,6413984468365658583_u64,3314369463578103308_u64,11948080128560742933_u64,2978820353671372425_u64,13690654724056099249_u64];
RET = 6395449519069412344_u64 as i64;
_1 = _3 as u128;
_5 = RET as i128;
_4 = !(*_6);
_8 = !_7;
Goto(bb9)
}
bb9 = {
_12 = [_7,_7,_8,_8,_7];
_7 = _8 - _8;
_5 = -(-170051767633325613078574061584200023239_i128);
_7 = _8;
_8 = RET as u32;
_11 = 6967_u16 as f32;
_6 = core::ptr::addr_of!(_4);
_5 = -(-125762961601704644961661251849200991271_i128);
RET = -(-3885070801022668992_i64);
_3 = _8 as isize;
_3 = !_9;
_7 = _8 | _8;
RET = 377313482727425313_i64 | (-7374808815082424401_i64);
_14 = Adt52 { fld0: _5,fld1: '\u{2009e}',fld2: (-13612_i16) };
_9 = _11 as isize;
_13 = Adt47::Variant1 { fld0: _1 };
SetDiscriminant(_13, 1);
_6 = core::ptr::addr_of!(_4);
_13 = Adt47::Variant0 { fld0: 639_u16 };
_14.fld0 = !_5;
place!(Field::<u16>(Variant(_13, 0), 0)) = _2 as u16;
RET = 4563484432160013761_i64 ^ (-9195857001277966952_i64);
_1 = 309575633632370567866054155256809565569_u128;
_16 = !(-460247661_i32);
_15.0 = !_5;
Goto(bb10)
}
bb10 = {
RET = (-2457003206236674539_i64);
_9 = !_3;
_14.fld2 = _11 as i16;
_6 = core::ptr::addr_of!((*_6));
_15 = (_5,);
_18.fld6 = [_16,_16,_16];
_18.fld2 = _1;
RET = 2610323588334078052_i64;
Goto(bb11)
}
bb11 = {
SetDiscriminant(_13, 1);
_14.fld0 = _5;
_1 = !_18.fld2;
_18.fld1 = _14.fld1;
_21.fld1 = _6;
_21.fld0 = _16 - _16;
Call(_18.fld4 = fn3(_8, (*_6), _7, _10, _18.fld6, (*_6), _10, _14.fld1, _2, _21.fld1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = (-5308250281455890935_i64) * 3156813280804237115_i64;
_18.fld3.fld2 = !_3;
_18.fld3.fld0 = Adt40::Variant2 { fld0: RET,fld1: _7,fld2: _10,fld3: _21.fld1 };
SetDiscriminant(_18.fld3.fld0, 2);
place!(Field::<[u64; 8]>(Variant(_18.fld3.fld0, 2), 2)) = _10;
_21.fld2 = core::ptr::addr_of!(_15);
Goto(bb13)
}
bb13 = {
RET = 3536423144344403054_i64 - (-2300323111491943801_i64);
Goto(bb14)
}
bb14 = {
_18.fld3.fld6 = !32330_u16;
_16 = _21.fld0;
place!(Field::<u128>(Variant(_13, 1), 0)) = _1 - _1;
_18.fld3.fld4.fld2 = core::ptr::addr_of!(_21.fld3);
_18.fld3.fld2 = 16349503631083453601_u64 as isize;
_18.fld3.fld2 = _3 << _5;
_14.fld1 = _18.fld1;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(2_usize, 2_usize, Move(_2), 15_usize, Move(_15), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(2_usize, 8_usize, Move(_8), 10_usize, Move(_10), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: u32,mut _2: bool,mut _3: u32,mut _4: [u64; 8],mut _5: [i32; 3],mut _6: bool,mut _7: [u64; 8],mut _8: char,mut _9: bool,mut _10: *const bool) -> *const *mut (u16,) {
mir! {
type RET = *const *mut (u16,);
let _11: Adt56;
let _12: Adt47;
let _13: i8;
let _14: [usize; 7];
let _15: i128;
let _16: bool;
let _17: f64;
let _18: u16;
let _19: ([usize; 8],);
let _20: ([usize; 8],);
let _21: [u64; 8];
let _22: (*mut (u16,), u8, char);
let _23: isize;
let _24: u8;
let _25: Adt52;
let _26: *const (i128,);
let _27: [u32; 5];
let _28: bool;
let _29: [i32; 3];
let _30: ();
let _31: ();
{
_4 = [12180984693802662790_u64,16828708235271278449_u64,6862045834513596501_u64,2336951010660592245_u64,6713836015148036087_u64,16894104371645044221_u64,1616872087879494261_u64,11161546731189650620_u64];
_11.fld3.fld4.fld3 = (31197417802756498032939947734924848813_i128,);
_3 = !_1;
_11.fld1 = _8;
_11.fld3.fld3 = [12742297319302057062_u64,7218304611735750941_u64,9037514550193156485_u64,17794826733039038957_u64,9604640554254212538_u64,11176226111992030779_u64,7542416945558514573_u64,16676681030904943892_u64];
_8 = _11.fld1;
_11.fld3.fld4.fld0 = _10;
_11.fld3.fld7 = !15377193424725640325_u64;
_10 = core::ptr::addr_of!(_6);
_5 = [588498258_i32,489289430_i32,327554872_i32];
_12 = Adt47::Variant1 { fld0: 327412461349267534139260460126395419834_u128 };
_3 = _11.fld3.fld4.fld3.0 as u32;
_12 = Adt47::Variant1 { fld0: 60836881846103246027326083536502837422_u128 };
_13 = (-43_i8);
Goto(bb1)
}
bb1 = {
_11.fld3.fld4.fld3.0 = 70133081634716100210230736489141380307_i128;
_9 = (*_10);
_11.fld3.fld4.fld3 = (100019826045300300049557767889152795501_i128,);
_11.fld1 = _8;
_11.fld3.fld5.0 = [6_usize,5244086944421581332_usize,7_usize,13087919210454041353_usize,681083630518029718_usize,0_usize,0_usize,15522636364641907083_usize];
_3 = !_1;
_11.fld3.fld4.fld1 = [5_usize,4498273510353295872_usize,4319921429438049365_usize,7_usize,3_usize,4_usize,11768755991106339690_usize,13881675249952275532_usize];
_11.fld2 = !298149814356552425193800628172581687729_u128;
_11.fld3.fld0 = Adt40::Variant0 { fld0: _11.fld2,fld1: _1 };
_11.fld3.fld2 = (-7074908973378556677_i64) as isize;
_11.fld3.fld7 = !657377060501785550_u64;
_11.fld3.fld0 = Adt40::Variant2 { fld0: 273797698374384834_i64,fld1: _3,fld2: _11.fld3.fld3,fld3: _10 };
_12 = Adt47::Variant1 { fld0: _11.fld2 };
_11.fld3.fld5.0 = [1_usize,17403474736066611431_usize,0_usize,2_usize,0_usize,7632415830604195451_usize,4_usize,14420909617159203308_usize];
_11.fld3.fld5.0 = _11.fld3.fld4.fld1;
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
_11.fld2 = Field::<u128>(Variant(_12, 1), 0) & Field::<u128>(Variant(_12, 1), 0);
_14 = [6_usize,3_usize,5_usize,2_usize,7481276092247411299_usize,11906340679171103802_usize,0_usize];
_11.fld3.fld4.fld0 = core::ptr::addr_of!((*_10));
Call(_11.fld3.fld5 = fn4(_8, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11.fld3.fld3 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld3.fld3 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld2 = !Field::<u128>(Variant(_12, 1), 0);
_10 = core::ptr::addr_of!(_2);
_11.fld3.fld6 = 57745_u16 + 7947_u16;
_8 = _11.fld1;
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
Goto(bb3)
}
bb3 = {
_11.fld3.fld5 = (_11.fld3.fld4.fld1,);
place!(Field::<u32>(Variant(_11.fld3.fld0, 2), 1)) = (-14991_i16) as u32;
_4 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_13 = (-123_i8);
SetDiscriminant(_12, 1);
_11.fld3.fld5.0 = [6_usize,13789345260322378261_usize,0_usize,10208843470068621765_usize,14592371696120910415_usize,0_usize,8042689743412804814_usize,5_usize];
_5 = [(-1515633572_i32),(-577593119_i32),347985756_i32];
_5 = [2027319966_i32,679924907_i32,(-1641874647_i32)];
_12 = Adt47::Variant0 { fld0: _11.fld3.fld6 };
_11.fld3.fld4.fld3 = (108275229767384427251456066945113596393_i128,);
_11.fld3.fld7 = !15185679094285384935_u64;
place!(Field::<u32>(Variant(_11.fld3.fld0, 2), 1)) = _1 & _1;
Goto(bb4)
}
bb4 = {
_5 = [(-327048837_i32),1623964069_i32,(-534018017_i32)];
Goto(bb5)
}
bb5 = {
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)) = 3_usize as i64;
_12 = Adt47::Variant0 { fld0: _11.fld3.fld6 };
place!(Field::<[u64; 8]>(Variant(_11.fld3.fld0, 2), 2)) = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_16 = _9;
place!(Field::<u16>(Variant(_12, 0), 0)) = !_11.fld3.fld6;
_11.fld3.fld5.0 = [2_usize,0_usize,5_usize,9365498649729793127_usize,13121097907514163731_usize,1_usize,14469703211561921080_usize,4961225813828661161_usize];
place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)) = 497421501770075470_i64;
_8 = _11.fld1;
_11.fld6 = [1699797366_i32,182574367_i32,(-540987084_i32)];
_11.fld3.fld4.fld0 = _10;
_11.fld3.fld5 = (_11.fld3.fld4.fld1,);
_9 = !_6;
_11.fld3.fld5 = (_11.fld3.fld4.fld1,);
_3 = !Field::<u32>(Variant(_11.fld3.fld0, 2), 1);
_8 = _11.fld1;
match _11.fld3.fld4.fld3.0 {
0 => bb6,
1 => bb7,
2 => bb8,
108275229767384427251456066945113596393 => bb10,
_ => bb9
}
}
bb6 = {
_5 = [(-327048837_i32),1623964069_i32,(-534018017_i32)];
Goto(bb5)
}
bb7 = {
_11.fld3.fld5 = (_11.fld3.fld4.fld1,);
place!(Field::<u32>(Variant(_11.fld3.fld0, 2), 1)) = (-14991_i16) as u32;
_4 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_13 = (-123_i8);
SetDiscriminant(_12, 1);
_11.fld3.fld5.0 = [6_usize,13789345260322378261_usize,0_usize,10208843470068621765_usize,14592371696120910415_usize,0_usize,8042689743412804814_usize,5_usize];
_5 = [(-1515633572_i32),(-577593119_i32),347985756_i32];
_5 = [2027319966_i32,679924907_i32,(-1641874647_i32)];
_12 = Adt47::Variant0 { fld0: _11.fld3.fld6 };
_11.fld3.fld4.fld3 = (108275229767384427251456066945113596393_i128,);
_11.fld3.fld7 = !15185679094285384935_u64;
place!(Field::<u32>(Variant(_11.fld3.fld0, 2), 1)) = _1 & _1;
Goto(bb4)
}
bb8 = {
_11.fld3.fld3 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld3.fld3 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld2 = !Field::<u128>(Variant(_12, 1), 0);
_10 = core::ptr::addr_of!(_2);
_11.fld3.fld6 = 57745_u16 + 7947_u16;
_8 = _11.fld1;
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
Goto(bb3)
}
bb9 = {
_11.fld3.fld4.fld3.0 = 70133081634716100210230736489141380307_i128;
_9 = (*_10);
_11.fld3.fld4.fld3 = (100019826045300300049557767889152795501_i128,);
_11.fld1 = _8;
_11.fld3.fld5.0 = [6_usize,5244086944421581332_usize,7_usize,13087919210454041353_usize,681083630518029718_usize,0_usize,0_usize,15522636364641907083_usize];
_3 = !_1;
_11.fld3.fld4.fld1 = [5_usize,4498273510353295872_usize,4319921429438049365_usize,7_usize,3_usize,4_usize,11768755991106339690_usize,13881675249952275532_usize];
_11.fld2 = !298149814356552425193800628172581687729_u128;
_11.fld3.fld0 = Adt40::Variant0 { fld0: _11.fld2,fld1: _1 };
_11.fld3.fld2 = (-7074908973378556677_i64) as isize;
_11.fld3.fld7 = !657377060501785550_u64;
_11.fld3.fld0 = Adt40::Variant2 { fld0: 273797698374384834_i64,fld1: _3,fld2: _11.fld3.fld3,fld3: _10 };
_12 = Adt47::Variant1 { fld0: _11.fld2 };
_11.fld3.fld5.0 = [1_usize,17403474736066611431_usize,0_usize,2_usize,0_usize,7632415830604195451_usize,4_usize,14420909617159203308_usize];
_11.fld3.fld5.0 = _11.fld3.fld4.fld1;
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
_11.fld2 = Field::<u128>(Variant(_12, 1), 0) & Field::<u128>(Variant(_12, 1), 0);
_14 = [6_usize,3_usize,5_usize,2_usize,7481276092247411299_usize,11906340679171103802_usize,0_usize];
_11.fld3.fld4.fld0 = core::ptr::addr_of!((*_10));
Call(_11.fld3.fld5 = fn4(_8, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_11.fld3.fld5.0 = [6_usize,7_usize,3_usize,16709673178866709221_usize,1_usize,1_usize,1_usize,10764526035657724946_usize];
place!(Field::<*const bool>(Variant(_11.fld3.fld0, 2), 3)) = _11.fld3.fld4.fld0;
_17 = 3_usize as f64;
_3 = Field::<u32>(Variant(_11.fld3.fld0, 2), 1) << _11.fld3.fld6;
_2 = _16;
_16 = !(*_10);
_11.fld3.fld4.fld3 = ((-74790926599222419501320790980220902977_i128),);
_11.fld3.fld5 = (_11.fld3.fld4.fld1,);
_4 = Field::<[u64; 8]>(Variant(_11.fld3.fld0, 2), 2);
_11.fld3.fld7 = _17 as u64;
_4 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_4 = _7;
SetDiscriminant(_11.fld3.fld0, 3);
_18 = Field::<u16>(Variant(_12, 0), 0) >> _11.fld3.fld4.fld3.0;
_9 = !(*_10);
_11.fld3.fld4.fld3 = ((-159257937717795910244204355894538469401_i128),);
SetDiscriminant(_12, 1);
_12 = Adt47::Variant1 { fld0: _11.fld2 };
_13 = (-94_i8);
_17 = _11.fld3.fld2 as f64;
match _13 {
0 => bb7,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
340282366920938463463374607431768211362 => bb16,
_ => bb15
}
}
bb11 = {
_11.fld3.fld5 = (_11.fld3.fld4.fld1,);
place!(Field::<u32>(Variant(_11.fld3.fld0, 2), 1)) = (-14991_i16) as u32;
_4 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_13 = (-123_i8);
SetDiscriminant(_12, 1);
_11.fld3.fld5.0 = [6_usize,13789345260322378261_usize,0_usize,10208843470068621765_usize,14592371696120910415_usize,0_usize,8042689743412804814_usize,5_usize];
_5 = [(-1515633572_i32),(-577593119_i32),347985756_i32];
_5 = [2027319966_i32,679924907_i32,(-1641874647_i32)];
_12 = Adt47::Variant0 { fld0: _11.fld3.fld6 };
_11.fld3.fld4.fld3 = (108275229767384427251456066945113596393_i128,);
_11.fld3.fld7 = !15185679094285384935_u64;
place!(Field::<u32>(Variant(_11.fld3.fld0, 2), 1)) = _1 & _1;
Goto(bb4)
}
bb12 = {
_11.fld3.fld3 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld3.fld3 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld2 = !Field::<u128>(Variant(_12, 1), 0);
_10 = core::ptr::addr_of!(_2);
_11.fld3.fld6 = 57745_u16 + 7947_u16;
_8 = _11.fld1;
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
Goto(bb3)
}
bb13 = {
_11.fld3.fld5 = (_11.fld3.fld4.fld1,);
place!(Field::<u32>(Variant(_11.fld3.fld0, 2), 1)) = (-14991_i16) as u32;
_4 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_13 = (-123_i8);
SetDiscriminant(_12, 1);
_11.fld3.fld5.0 = [6_usize,13789345260322378261_usize,0_usize,10208843470068621765_usize,14592371696120910415_usize,0_usize,8042689743412804814_usize,5_usize];
_5 = [(-1515633572_i32),(-577593119_i32),347985756_i32];
_5 = [2027319966_i32,679924907_i32,(-1641874647_i32)];
_12 = Adt47::Variant0 { fld0: _11.fld3.fld6 };
_11.fld3.fld4.fld3 = (108275229767384427251456066945113596393_i128,);
_11.fld3.fld7 = !15185679094285384935_u64;
place!(Field::<u32>(Variant(_11.fld3.fld0, 2), 1)) = _1 & _1;
Goto(bb4)
}
bb14 = {
_5 = [(-327048837_i32),1623964069_i32,(-534018017_i32)];
Goto(bb5)
}
bb15 = {
_11.fld3.fld3 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld3.fld3 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld2 = !Field::<u128>(Variant(_12, 1), 0);
_10 = core::ptr::addr_of!(_2);
_11.fld3.fld6 = 57745_u16 + 7947_u16;
_8 = _11.fld1;
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
_11.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_11.fld3.fld0, 2), 0)));
Goto(bb3)
}
bb16 = {
_11.fld3.fld4.fld1 = _11.fld3.fld5.0;
_14 = [0_usize,7804855510931718109_usize,7_usize,2_usize,17192482164845978396_usize,3712294697282293665_usize,0_usize];
Call(_19.0 = core::intrinsics::transmute(_7), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_20.0 = _11.fld3.fld4.fld1;
_8 = _11.fld1;
_11.fld3.fld5.0 = [6_usize,5_usize,0_usize,2_usize,6_usize,9021839111436929053_usize,742139082182793847_usize,3_usize];
_5 = _11.fld6;
Goto(bb18)
}
bb18 = {
_18 = !_11.fld3.fld6;
_8 = _11.fld1;
_13 = -(-26_i8);
_11.fld3.fld4.fld0 = _10;
_11.fld3.fld4.fld2 = core::ptr::addr_of!(_22.0);
Goto(bb19)
}
bb19 = {
_24 = _3 as u8;
_11.fld3.fld0 = Adt40::Variant0 { fld0: Field::<u128>(Variant(_12, 1), 0),fld1: _3 };
RET = core::ptr::addr_of!(_22.0);
_19.0 = [14537530575902212344_usize,2_usize,6_usize,7_usize,3_usize,15645566469915396119_usize,12649228504747328201_usize,8196375838963872391_usize];
_11.fld1 = _8;
_21 = _7;
_26 = core::ptr::addr_of!(_11.fld3.fld4.fld3);
_19.0 = _20.0;
SetDiscriminant(_12, 0);
_19.0 = _11.fld3.fld4.fld1;
_11.fld3.fld4.fld3 = ((-153422839345815764158195643400027727152_i128),);
_6 = !_16;
_11.fld3.fld2 = (*_26).0 as isize;
_13 = 66_i8;
_22.2 = _11.fld1;
_26 = core::ptr::addr_of!((*_26));
_15 = _18 as i128;
_17 = 1017173731_i32 as f64;
_25.fld0 = -_11.fld3.fld4.fld3.0;
_11.fld3.fld5 = (_20.0,);
_29 = [835238906_i32,(-1070189851_i32),2091191021_i32];
_11.fld1 = _22.2;
_14 = [17733252584407614145_usize,4_usize,13906788847672597462_usize,3_usize,3264910067354793487_usize,17428120394734187955_usize,0_usize];
_7 = [_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7,_11.fld3.fld7];
_11.fld1 = _8;
Goto(bb20)
}
bb20 = {
Call(_30 = dump_var(3_usize, 1_usize, Move(_1), 13_usize, Move(_13), 21_usize, Move(_21), 2_usize, Move(_2)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_30 = dump_var(3_usize, 16_usize, Move(_16), 18_usize, Move(_18), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_30 = dump_var(3_usize, 7_usize, Move(_7), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: char,mut _2: [i32; 3]) -> ([usize; 8],) {
mir! {
type RET = ([usize; 8],);
let _3: char;
let _4: *const i64;
let _5: Adt52;
let _6: f32;
let _7: i16;
let _8: Adt43;
let _9: Adt44;
let _10: Adt56;
let _11: [usize; 7];
let _12: i16;
let _13: &'static bool;
let _14: isize;
let _15: &'static bool;
let _16: usize;
let _17: [usize; 8];
let _18: i16;
let _19: u128;
let _20: f32;
let _21: f64;
let _22: (i128,);
let _23: f64;
let _24: Adt48;
let _25: i128;
let _26: [i32; 3];
let _27: bool;
let _28: [usize; 7];
let _29: ([usize; 8],);
let _30: u8;
let _31: char;
let _32: (i128,);
let _33: ();
let _34: ();
{
RET.0 = [11545764264481228257_usize,16978143781465108910_usize,2389475373485009496_usize,6_usize,7_usize,1068712697368917231_usize,6_usize,1434011022063888464_usize];
_3 = _1;
_2 = [324158521_i32,117265742_i32,900818346_i32];
_5 = Adt52 { fld0: 36943921468296419611765994237095943479_i128,fld1: _1,fld2: 19381_i16 };
_3 = _5.fld1;
_5.fld1 = _1;
RET.0 = [12000580492543607191_usize,17350383078138984899_usize,7_usize,7426156907214520735_usize,2_usize,0_usize,15507800856726502129_usize,6_usize];
_5 = Adt52 { fld0: 54467280169332594527108758946538523755_i128,fld1: _3,fld2: 15085_i16 };
Goto(bb1)
}
bb1 = {
RET.0 = [7_usize,1_usize,2_usize,9020601390778334186_usize,4_usize,7_usize,5_usize,15848923151891421701_usize];
_5.fld1 = _3;
_5.fld2 = 5_u8 as i16;
_2 = [(-923921534_i32),(-1893154789_i32),(-1154111090_i32)];
_8.fld4.fld3.0 = _5.fld0 - _5.fld0;
_6 = 9223372036854775807_isize as f32;
_8.fld5 = (RET.0,);
_8.fld2 = !(-9223372036854775808_isize);
_8.fld5 = RET;
RET.0 = _8.fld5.0;
_7 = _5.fld2 - _5.fld2;
_5 = Adt52 { fld0: _8.fld4.fld3.0,fld1: _3,fld2: _7 };
_10.fld3.fld6 = 32854_u16 - 60786_u16;
_10.fld3.fld6 = 59816_u16 & 32706_u16;
_8.fld3 = [720837102512404351_u64,8448867268860909511_u64,4394041378941269941_u64,3175223700465117272_u64,8460724573763779401_u64,8696801971527850705_u64,4628020155252643031_u64,6494527940346865457_u64];
_10.fld6 = [(-1581549464_i32),(-1198100469_i32),146096063_i32];
_10.fld3.fld2 = _8.fld2 << _8.fld4.fld3.0;
_8.fld4.fld3.0 = _5.fld0;
_10.fld3.fld4.fld3 = _8.fld4.fld3;
_8.fld4.fld1 = [2_usize,0_usize,2_usize,5_usize,12643617965869187849_usize,6_usize,4356610679131395528_usize,12763656789947603771_usize];
Call(_10.fld3.fld4.fld0 = fn5(_8.fld5, _10.fld3.fld2, _10.fld3.fld6, _5.fld1, _10.fld3.fld2, _5, _10.fld3.fld6, _8.fld4.fld1, RET, RET.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10.fld3.fld4.fld1 = _8.fld5.0;
_8.fld7 = 586059619977322585_u64 * 9148896005569093787_u64;
_9.fld1 = _10.fld3.fld4.fld0;
_5.fld0 = -_8.fld4.fld3.0;
_1 = _5.fld1;
_8.fld4.fld2 = core::ptr::addr_of!(_9.fld3);
_5.fld1 = _1;
_9.fld0 = !(-1654099421_i32);
_2 = [_9.fld0,_9.fld0,_9.fld0];
_8.fld3 = [_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7];
RET = (_8.fld4.fld1,);
_10.fld2 = _7 as u128;
_8.fld0 = Adt40::Variant2 { fld0: (-5863858554537973441_i64),fld1: 3200255865_u32,fld2: _8.fld3,fld3: _9.fld1 };
_10.fld3.fld6 = 30129_u16;
_10.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_8.fld0, 2), 0)));
Goto(bb3)
}
bb3 = {
_10.fld3.fld5.0 = [4_usize,14682733436718717940_usize,2736585966092554508_usize,4_usize,0_usize,7622556415275865914_usize,7_usize,2_usize];
_7 = !_5.fld2;
RET.0 = _8.fld5.0;
_10.fld3.fld2 = -_8.fld2;
_8.fld6 = !_10.fld3.fld6;
_11 = [2743265892456942465_usize,5_usize,1394781971631021949_usize,5462453215531150711_usize,0_usize,5_usize,7_usize];
_4 = _10.fld0;
_10.fld3.fld5 = (_10.fld3.fld4.fld1,);
_5.fld2 = _7 << _5.fld0;
_9.fld2 = core::ptr::addr_of!(_10.fld3.fld4.fld3);
_8.fld4.fld1 = _10.fld3.fld4.fld1;
_4 = _10.fld0;
_10.fld1 = _1;
place!(Field::<i64>(Variant(_8.fld0, 2), 0)) = _9.fld0 as i64;
_10.fld0 = _4;
_11 = [3_usize,0_usize,2_usize,7_usize,2317228973878515178_usize,3800317289410708141_usize,13579572314401985339_usize];
_17 = _8.fld4.fld1;
_8.fld7 = !14067828122689773834_u64;
RET.0 = [7_usize,11461423619151518267_usize,14648729536208001537_usize,6_usize,1244323704156201228_usize,1_usize,3226649268046668096_usize,3_usize];
_16 = 1_usize;
_10.fld6[_16] = _1 as i32;
_10.fld3.fld5.0 = [_17[_16],_17[_16],_8.fld4.fld1[_16],_8.fld5.0[_16],_8.fld5.0[_16],_8.fld4.fld1[_16],_17[_16],_17[_16]];
_10.fld3.fld0 = Adt40::Variant2 { fld0: Field::<i64>(Variant(_8.fld0, 2), 0),fld1: 3871845131_u32,fld2: Field::<[u64; 8]>(Variant(_8.fld0, 2), 2),fld3: _10.fld3.fld4.fld0 };
Call(_7 = core::intrinsics::bswap(_5.fld2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10.fld3.fld5 = RET;
place!(Field::<[u64; 8]>(Variant(_10.fld3.fld0, 2), 2))[_16] = !Field::<[u64; 8]>(Variant(_8.fld0, 2), 2)[_16];
_8.fld4.fld1[_16] = _6 as usize;
_10.fld3.fld2 = !_8.fld2;
_19 = _10.fld2;
RET.0 = [_11[_16],_10.fld3.fld5.0[_16],_8.fld5.0[_16],_8.fld5.0[_16],_10.fld3.fld4.fld1[_16],_8.fld5.0[_16],_17[_16],_10.fld3.fld4.fld1[_16]];
_10.fld3.fld6 = !_8.fld6;
place!(Field::<*const bool>(Variant(_8.fld0, 2), 3)) = Field::<*const bool>(Variant(_10.fld3.fld0, 2), 3);
_8.fld0 = Adt40::Variant0 { fld0: _19,fld1: 1315276410_u32 };
Goto(bb5)
}
bb5 = {
_8.fld4.fld1 = [_11[_16],_10.fld3.fld4.fld1[_16],_11[_16],_16,_17[_16],_17[_16],RET.0[_16],RET.0[_16]];
place!(Field::<u32>(Variant(_10.fld3.fld0, 2), 1)) = 661215045_u32 ^ 753607962_u32;
_21 = _5.fld2 as f64;
RET = _10.fld3.fld5;
_22.0 = _10.fld2 as i128;
_22.0 = _5.fld0 >> _8.fld4.fld3.0;
_20 = -_6;
match _10.fld3.fld5.0[_16] {
0 => bb6,
1 => bb7,
2 => bb8,
11461423619151518267 => bb10,
_ => bb9
}
}
bb6 = {
_10.fld3.fld5 = RET;
place!(Field::<[u64; 8]>(Variant(_10.fld3.fld0, 2), 2))[_16] = !Field::<[u64; 8]>(Variant(_8.fld0, 2), 2)[_16];
_8.fld4.fld1[_16] = _6 as usize;
_10.fld3.fld2 = !_8.fld2;
_19 = _10.fld2;
RET.0 = [_11[_16],_10.fld3.fld5.0[_16],_8.fld5.0[_16],_8.fld5.0[_16],_10.fld3.fld4.fld1[_16],_8.fld5.0[_16],_17[_16],_10.fld3.fld4.fld1[_16]];
_10.fld3.fld6 = !_8.fld6;
place!(Field::<*const bool>(Variant(_8.fld0, 2), 3)) = Field::<*const bool>(Variant(_10.fld3.fld0, 2), 3);
_8.fld0 = Adt40::Variant0 { fld0: _19,fld1: 1315276410_u32 };
Goto(bb5)
}
bb7 = {
_10.fld3.fld5.0 = [4_usize,14682733436718717940_usize,2736585966092554508_usize,4_usize,0_usize,7622556415275865914_usize,7_usize,2_usize];
_7 = !_5.fld2;
RET.0 = _8.fld5.0;
_10.fld3.fld2 = -_8.fld2;
_8.fld6 = !_10.fld3.fld6;
_11 = [2743265892456942465_usize,5_usize,1394781971631021949_usize,5462453215531150711_usize,0_usize,5_usize,7_usize];
_4 = _10.fld0;
_10.fld3.fld5 = (_10.fld3.fld4.fld1,);
_5.fld2 = _7 << _5.fld0;
_9.fld2 = core::ptr::addr_of!(_10.fld3.fld4.fld3);
_8.fld4.fld1 = _10.fld3.fld4.fld1;
_4 = _10.fld0;
_10.fld1 = _1;
place!(Field::<i64>(Variant(_8.fld0, 2), 0)) = _9.fld0 as i64;
_10.fld0 = _4;
_11 = [3_usize,0_usize,2_usize,7_usize,2317228973878515178_usize,3800317289410708141_usize,13579572314401985339_usize];
_17 = _8.fld4.fld1;
_8.fld7 = !14067828122689773834_u64;
RET.0 = [7_usize,11461423619151518267_usize,14648729536208001537_usize,6_usize,1244323704156201228_usize,1_usize,3226649268046668096_usize,3_usize];
_16 = 1_usize;
_10.fld6[_16] = _1 as i32;
_10.fld3.fld5.0 = [_17[_16],_17[_16],_8.fld4.fld1[_16],_8.fld5.0[_16],_8.fld5.0[_16],_8.fld4.fld1[_16],_17[_16],_17[_16]];
_10.fld3.fld0 = Adt40::Variant2 { fld0: Field::<i64>(Variant(_8.fld0, 2), 0),fld1: 3871845131_u32,fld2: Field::<[u64; 8]>(Variant(_8.fld0, 2), 2),fld3: _10.fld3.fld4.fld0 };
Call(_7 = core::intrinsics::bswap(_5.fld2), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_10.fld3.fld4.fld1 = _8.fld5.0;
_8.fld7 = 586059619977322585_u64 * 9148896005569093787_u64;
_9.fld1 = _10.fld3.fld4.fld0;
_5.fld0 = -_8.fld4.fld3.0;
_1 = _5.fld1;
_8.fld4.fld2 = core::ptr::addr_of!(_9.fld3);
_5.fld1 = _1;
_9.fld0 = !(-1654099421_i32);
_2 = [_9.fld0,_9.fld0,_9.fld0];
_8.fld3 = [_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7];
RET = (_8.fld4.fld1,);
_10.fld2 = _7 as u128;
_8.fld0 = Adt40::Variant2 { fld0: (-5863858554537973441_i64),fld1: 3200255865_u32,fld2: _8.fld3,fld3: _9.fld1 };
_10.fld3.fld6 = 30129_u16;
_10.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_8.fld0, 2), 0)));
Goto(bb3)
}
bb9 = {
RET.0 = [7_usize,1_usize,2_usize,9020601390778334186_usize,4_usize,7_usize,5_usize,15848923151891421701_usize];
_5.fld1 = _3;
_5.fld2 = 5_u8 as i16;
_2 = [(-923921534_i32),(-1893154789_i32),(-1154111090_i32)];
_8.fld4.fld3.0 = _5.fld0 - _5.fld0;
_6 = 9223372036854775807_isize as f32;
_8.fld5 = (RET.0,);
_8.fld2 = !(-9223372036854775808_isize);
_8.fld5 = RET;
RET.0 = _8.fld5.0;
_7 = _5.fld2 - _5.fld2;
_5 = Adt52 { fld0: _8.fld4.fld3.0,fld1: _3,fld2: _7 };
_10.fld3.fld6 = 32854_u16 - 60786_u16;
_10.fld3.fld6 = 59816_u16 & 32706_u16;
_8.fld3 = [720837102512404351_u64,8448867268860909511_u64,4394041378941269941_u64,3175223700465117272_u64,8460724573763779401_u64,8696801971527850705_u64,4628020155252643031_u64,6494527940346865457_u64];
_10.fld6 = [(-1581549464_i32),(-1198100469_i32),146096063_i32];
_10.fld3.fld2 = _8.fld2 << _8.fld4.fld3.0;
_8.fld4.fld3.0 = _5.fld0;
_10.fld3.fld4.fld3 = _8.fld4.fld3;
_8.fld4.fld1 = [2_usize,0_usize,2_usize,5_usize,12643617965869187849_usize,6_usize,4356610679131395528_usize,12763656789947603771_usize];
Call(_10.fld3.fld4.fld0 = fn5(_8.fld5, _10.fld3.fld2, _10.fld3.fld6, _5.fld1, _10.fld3.fld2, _5, _10.fld3.fld6, _8.fld4.fld1, RET, RET.0), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_8.fld4.fld3.0 = _22.0;
match _10.fld3.fld5.0[_16] {
0 => bb1,
1 => bb6,
2 => bb4,
11461423619151518267 => bb12,
_ => bb11
}
}
bb11 = {
RET.0 = [7_usize,1_usize,2_usize,9020601390778334186_usize,4_usize,7_usize,5_usize,15848923151891421701_usize];
_5.fld1 = _3;
_5.fld2 = 5_u8 as i16;
_2 = [(-923921534_i32),(-1893154789_i32),(-1154111090_i32)];
_8.fld4.fld3.0 = _5.fld0 - _5.fld0;
_6 = 9223372036854775807_isize as f32;
_8.fld5 = (RET.0,);
_8.fld2 = !(-9223372036854775808_isize);
_8.fld5 = RET;
RET.0 = _8.fld5.0;
_7 = _5.fld2 - _5.fld2;
_5 = Adt52 { fld0: _8.fld4.fld3.0,fld1: _3,fld2: _7 };
_10.fld3.fld6 = 32854_u16 - 60786_u16;
_10.fld3.fld6 = 59816_u16 & 32706_u16;
_8.fld3 = [720837102512404351_u64,8448867268860909511_u64,4394041378941269941_u64,3175223700465117272_u64,8460724573763779401_u64,8696801971527850705_u64,4628020155252643031_u64,6494527940346865457_u64];
_10.fld6 = [(-1581549464_i32),(-1198100469_i32),146096063_i32];
_10.fld3.fld2 = _8.fld2 << _8.fld4.fld3.0;
_8.fld4.fld3.0 = _5.fld0;
_10.fld3.fld4.fld3 = _8.fld4.fld3;
_8.fld4.fld1 = [2_usize,0_usize,2_usize,5_usize,12643617965869187849_usize,6_usize,4356610679131395528_usize,12763656789947603771_usize];
Call(_10.fld3.fld4.fld0 = fn5(_8.fld5, _10.fld3.fld2, _10.fld3.fld6, _5.fld1, _10.fld3.fld2, _5, _10.fld3.fld6, _8.fld4.fld1, RET, RET.0), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_8.fld7 = Field::<[u64; 8]>(Variant(_10.fld3.fld0, 2), 2)[_16] | _8.fld3[_16];
_10.fld3.fld5 = (_17,);
_24 = Adt48::Variant0 { fld0: _4,fld1: _10.fld1,fld2: RET.0 };
_27 = Field::<u32>(Variant(_10.fld3.fld0, 2), 1) <= Field::<u32>(Variant(_10.fld3.fld0, 2), 1);
_8.fld4.fld2 = core::ptr::addr_of!(_9.fld3);
_10.fld3.fld4.fld3 = _22;
_8.fld4.fld1[_16] = !_10.fld3.fld4.fld1[_16];
_10.fld3.fld0 = Adt40::Variant1 { fld0: _9.fld2 };
SetDiscriminant(_24, 1);
_17 = [RET.0[_16],_8.fld4.fld1[_16],_10.fld3.fld4.fld1[_16],RET.0[_16],_10.fld3.fld4.fld1[_16],_8.fld5.0[_16],_11[_16],RET.0[_16]];
_8.fld3 = [_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7];
_8.fld5 = RET;
_5.fld2 = _7 | _7;
_17[_16] = _8.fld2 as usize;
_10.fld3.fld4.fld1 = [_11[_16],_10.fld3.fld5.0[_16],_8.fld4.fld1[_16],RET.0[_16],_8.fld5.0[_16],_8.fld5.0[_16],_10.fld3.fld5.0[_16],_8.fld5.0[_16]];
_13 = &_27;
Goto(bb13)
}
bb13 = {
_4 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_24, 1), 0)));
place!(Field::<u32>(Variant(_8.fld0, 0), 1)) = 1591266697_u32 + 2379913997_u32;
_26[_16] = _10.fld6[_16] & _9.fld0;
_11[_16] = _10.fld3.fld5.0[_16];
_26[_16] = _2[_16] << _8.fld5.0[_16];
_10.fld3.fld5.0 = [_10.fld3.fld4.fld1[_16],_8.fld4.fld1[_16],_10.fld3.fld4.fld1[_16],_10.fld3.fld4.fld1[_16],_10.fld3.fld4.fld1[_16],_10.fld3.fld4.fld1[_16],_8.fld5.0[_16],_10.fld3.fld4.fld1[_16]];
_24 = Adt48::Variant0 { fld0: _4,fld1: _5.fld1,fld2: _8.fld5.0 };
_5.fld0 = _21 as i128;
_26[_16] = Field::<u32>(Variant(_8.fld0, 0), 1) as i32;
_23 = _21;
_10.fld3.fld4.fld3 = (_5.fld0,);
match _10.fld3.fld4.fld1[_16] {
0 => bb14,
2 => bb16,
3 => bb17,
4 => bb18,
1 => bb20,
_ => bb19
}
}
bb14 = {
_10.fld3.fld5 = RET;
place!(Field::<[u64; 8]>(Variant(_10.fld3.fld0, 2), 2))[_16] = !Field::<[u64; 8]>(Variant(_8.fld0, 2), 2)[_16];
_8.fld4.fld1[_16] = _6 as usize;
_10.fld3.fld2 = !_8.fld2;
_19 = _10.fld2;
RET.0 = [_11[_16],_10.fld3.fld5.0[_16],_8.fld5.0[_16],_8.fld5.0[_16],_10.fld3.fld4.fld1[_16],_8.fld5.0[_16],_17[_16],_10.fld3.fld4.fld1[_16]];
_10.fld3.fld6 = !_8.fld6;
place!(Field::<*const bool>(Variant(_8.fld0, 2), 3)) = Field::<*const bool>(Variant(_10.fld3.fld0, 2), 3);
_8.fld0 = Adt40::Variant0 { fld0: _19,fld1: 1315276410_u32 };
Goto(bb5)
}
bb15 = {
_10.fld3.fld5 = RET;
place!(Field::<[u64; 8]>(Variant(_10.fld3.fld0, 2), 2))[_16] = !Field::<[u64; 8]>(Variant(_8.fld0, 2), 2)[_16];
_8.fld4.fld1[_16] = _6 as usize;
_10.fld3.fld2 = !_8.fld2;
_19 = _10.fld2;
RET.0 = [_11[_16],_10.fld3.fld5.0[_16],_8.fld5.0[_16],_8.fld5.0[_16],_10.fld3.fld4.fld1[_16],_8.fld5.0[_16],_17[_16],_10.fld3.fld4.fld1[_16]];
_10.fld3.fld6 = !_8.fld6;
place!(Field::<*const bool>(Variant(_8.fld0, 2), 3)) = Field::<*const bool>(Variant(_10.fld3.fld0, 2), 3);
_8.fld0 = Adt40::Variant0 { fld0: _19,fld1: 1315276410_u32 };
Goto(bb5)
}
bb16 = {
_10.fld3.fld5.0 = [4_usize,14682733436718717940_usize,2736585966092554508_usize,4_usize,0_usize,7622556415275865914_usize,7_usize,2_usize];
_7 = !_5.fld2;
RET.0 = _8.fld5.0;
_10.fld3.fld2 = -_8.fld2;
_8.fld6 = !_10.fld3.fld6;
_11 = [2743265892456942465_usize,5_usize,1394781971631021949_usize,5462453215531150711_usize,0_usize,5_usize,7_usize];
_4 = _10.fld0;
_10.fld3.fld5 = (_10.fld3.fld4.fld1,);
_5.fld2 = _7 << _5.fld0;
_9.fld2 = core::ptr::addr_of!(_10.fld3.fld4.fld3);
_8.fld4.fld1 = _10.fld3.fld4.fld1;
_4 = _10.fld0;
_10.fld1 = _1;
place!(Field::<i64>(Variant(_8.fld0, 2), 0)) = _9.fld0 as i64;
_10.fld0 = _4;
_11 = [3_usize,0_usize,2_usize,7_usize,2317228973878515178_usize,3800317289410708141_usize,13579572314401985339_usize];
_17 = _8.fld4.fld1;
_8.fld7 = !14067828122689773834_u64;
RET.0 = [7_usize,11461423619151518267_usize,14648729536208001537_usize,6_usize,1244323704156201228_usize,1_usize,3226649268046668096_usize,3_usize];
_16 = 1_usize;
_10.fld6[_16] = _1 as i32;
_10.fld3.fld5.0 = [_17[_16],_17[_16],_8.fld4.fld1[_16],_8.fld5.0[_16],_8.fld5.0[_16],_8.fld4.fld1[_16],_17[_16],_17[_16]];
_10.fld3.fld0 = Adt40::Variant2 { fld0: Field::<i64>(Variant(_8.fld0, 2), 0),fld1: 3871845131_u32,fld2: Field::<[u64; 8]>(Variant(_8.fld0, 2), 2),fld3: _10.fld3.fld4.fld0 };
Call(_7 = core::intrinsics::bswap(_5.fld2), ReturnTo(bb4), UnwindUnreachable())
}
bb17 = {
_8.fld4.fld1 = [_11[_16],_10.fld3.fld4.fld1[_16],_11[_16],_16,_17[_16],_17[_16],RET.0[_16],RET.0[_16]];
place!(Field::<u32>(Variant(_10.fld3.fld0, 2), 1)) = 661215045_u32 ^ 753607962_u32;
_21 = _5.fld2 as f64;
RET = _10.fld3.fld5;
_22.0 = _10.fld2 as i128;
_22.0 = _5.fld0 >> _8.fld4.fld3.0;
_20 = -_6;
match _10.fld3.fld5.0[_16] {
0 => bb6,
1 => bb7,
2 => bb8,
11461423619151518267 => bb10,
_ => bb9
}
}
bb18 = {
_10.fld3.fld4.fld1 = _8.fld5.0;
_8.fld7 = 586059619977322585_u64 * 9148896005569093787_u64;
_9.fld1 = _10.fld3.fld4.fld0;
_5.fld0 = -_8.fld4.fld3.0;
_1 = _5.fld1;
_8.fld4.fld2 = core::ptr::addr_of!(_9.fld3);
_5.fld1 = _1;
_9.fld0 = !(-1654099421_i32);
_2 = [_9.fld0,_9.fld0,_9.fld0];
_8.fld3 = [_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7,_8.fld7];
RET = (_8.fld4.fld1,);
_10.fld2 = _7 as u128;
_8.fld0 = Adt40::Variant2 { fld0: (-5863858554537973441_i64),fld1: 3200255865_u32,fld2: _8.fld3,fld3: _9.fld1 };
_10.fld3.fld6 = 30129_u16;
_10.fld0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_8.fld0, 2), 0)));
Goto(bb3)
}
bb19 = {
_10.fld3.fld5.0 = [4_usize,14682733436718717940_usize,2736585966092554508_usize,4_usize,0_usize,7622556415275865914_usize,7_usize,2_usize];
_7 = !_5.fld2;
RET.0 = _8.fld5.0;
_10.fld3.fld2 = -_8.fld2;
_8.fld6 = !_10.fld3.fld6;
_11 = [2743265892456942465_usize,5_usize,1394781971631021949_usize,5462453215531150711_usize,0_usize,5_usize,7_usize];
_4 = _10.fld0;
_10.fld3.fld5 = (_10.fld3.fld4.fld1,);
_5.fld2 = _7 << _5.fld0;
_9.fld2 = core::ptr::addr_of!(_10.fld3.fld4.fld3);
_8.fld4.fld1 = _10.fld3.fld4.fld1;
_4 = _10.fld0;
_10.fld1 = _1;
place!(Field::<i64>(Variant(_8.fld0, 2), 0)) = _9.fld0 as i64;
_10.fld0 = _4;
_11 = [3_usize,0_usize,2_usize,7_usize,2317228973878515178_usize,3800317289410708141_usize,13579572314401985339_usize];
_17 = _8.fld4.fld1;
_8.fld7 = !14067828122689773834_u64;
RET.0 = [7_usize,11461423619151518267_usize,14648729536208001537_usize,6_usize,1244323704156201228_usize,1_usize,3226649268046668096_usize,3_usize];
_16 = 1_usize;
_10.fld6[_16] = _1 as i32;
_10.fld3.fld5.0 = [_17[_16],_17[_16],_8.fld4.fld1[_16],_8.fld5.0[_16],_8.fld5.0[_16],_8.fld4.fld1[_16],_17[_16],_17[_16]];
_10.fld3.fld0 = Adt40::Variant2 { fld0: Field::<i64>(Variant(_8.fld0, 2), 0),fld1: 3871845131_u32,fld2: Field::<[u64; 8]>(Variant(_8.fld0, 2), 2),fld3: _10.fld3.fld4.fld0 };
Call(_7 = core::intrinsics::bswap(_5.fld2), ReturnTo(bb4), UnwindUnreachable())
}
bb20 = {
_29.0[_16] = !_10.fld3.fld4.fld1[_16];
_11[_16] = !_8.fld4.fld1[_16];
_2 = [_26[_16],_26[_16],_26[_16]];
_5 = Adt52 { fld0: _10.fld3.fld4.fld3.0,fld1: _3,fld2: _7 };
_8.fld0 = Adt40::Variant2 { fld0: (-92160176706157593_i64),fld1: 4280819108_u32,fld2: _8.fld3,fld3: _10.fld3.fld4.fld0 };
_30 = 229_u8;
_29 = (RET.0,);
place!(Field::<i64>(Variant(_8.fld0, 2), 0)) = !4698058968141826215_i64;
_10.fld3.fld3[_16] = Field::<[u64; 8]>(Variant(_8.fld0, 2), 2)[_16] >> _29.0[_16];
_10.fld3.fld7 = _21 as u64;
_8.fld4.fld1 = [_10.fld3.fld4.fld1[_16],Field::<[usize; 8]>(Variant(_24, 0), 2)[_16],_8.fld5.0[_16],_29.0[_16],Field::<[usize; 8]>(Variant(_24, 0), 2)[_16],Field::<[usize; 8]>(Variant(_24, 0), 2)[_16],_10.fld3.fld5.0[_16],_10.fld3.fld5.0[_16]];
_29.0 = [_8.fld5.0[_16],_10.fld3.fld5.0[_16],_10.fld3.fld4.fld1[_16],_8.fld5.0[_16],_10.fld3.fld4.fld1[_16],RET.0[_16],RET.0[_16],RET.0[_16]];
_10.fld3.fld5.0[_16] = _10.fld3.fld4.fld1[_16] / RET.0[_16];
_8.fld6 = _10.fld3.fld6;
_15 = Move(_13);
_5 = Adt52 { fld0: _10.fld3.fld4.fld3.0,fld1: Field::<char>(Variant(_24, 0), 1),fld2: _7 };
_10.fld3.fld4.fld3.0 = _22.0 + _8.fld4.fld3.0;
_22 = (_8.fld4.fld3.0,);
_8.fld7 = !_10.fld3.fld3[_16];
_2[_16] = _30 as i32;
_27 = true;
_8.fld0 = Adt40::Variant1 { fld0: _9.fld2 };
place!(Field::<*const (i128,)>(Variant(_10.fld3.fld0, 1), 0)) = core::ptr::addr_of!(_8.fld4.fld3);
place!(Field::<*const (i128,)>(Variant(_8.fld0, 1), 0)) = core::ptr::addr_of!(_10.fld3.fld4.fld3);
Goto(bb21)
}
bb21 = {
Call(_33 = dump_var(4_usize, 17_usize, Move(_17), 2_usize, Move(_2), 19_usize, Move(_19), 16_usize, Move(_16)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_33 = dump_var(4_usize, 29_usize, Move(_29), 22_usize, Move(_22), 34_usize, _34, 34_usize, _34), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: ([usize; 8],),mut _2: isize,mut _3: u16,mut _4: char,mut _5: isize,mut _6: Adt52,mut _7: u16,mut _8: [usize; 8],mut _9: ([usize; 8],),mut _10: [usize; 8]) -> *const bool {
mir! {
type RET = *const bool;
let _11: u8;
let _12: *mut [i64; 5];
let _13: f64;
let _14: isize;
let _15: [i32; 3];
let _16: Adt47;
let _17: char;
let _18: char;
let _19: u8;
let _20: Adt44;
let _21: isize;
let _22: u128;
let _23: (*mut (u16,), u8, char);
let _24: char;
let _25: isize;
let _26: u32;
let _27: ();
let _28: ();
{
_5 = -_2;
_7 = _3 | _3;
_7 = _3;
_6.fld0 = (-11199146064572612432832635954881158146_i128) - (-132086106365235707043692102281939622604_i128);
_4 = _6.fld1;
_6.fld2 = 9312_i16 + (-8744_i16);
_1 = (_10,);
_10 = [10878616996708230640_usize,5_usize,15743657965143106678_usize,4_usize,6259507029975142489_usize,0_usize,1_usize,3521963803559110767_usize];
_7 = _3 & _3;
_6.fld1 = _4;
_6.fld0 = 4154001445384088161829469663614379864_i128 ^ (-87871874432318134573806875689429146038_i128);
_9 = (_10,);
_11 = 153_u8 + 61_u8;
_4 = _6.fld1;
_6.fld2 = 4429373811510187598_usize as i16;
_7 = _3;
_2 = _5;
_6 = Adt52 { fld0: 55973327155703950149846304934147899329_i128,fld1: _4,fld2: (-4400_i16) };
_7 = _3 - _3;
Goto(bb1)
}
bb1 = {
_7 = !_3;
_10 = [4_usize,1_usize,752211504003721480_usize,7_usize,1536619400799185455_usize,12808075225465269907_usize,7_usize,4_usize];
_8 = [5_usize,4452364403897876345_usize,1_usize,2_usize,8149496611272645906_usize,6759444931646094995_usize,12476556623468276698_usize,12482995950303643494_usize];
_14 = _5;
_9 = (_10,);
_6.fld0 = (-39416389255206062153769041130767152680_i128) ^ 13059827985015369562514905094545221474_i128;
Goto(bb2)
}
bb2 = {
_6 = Adt52 { fld0: (-78521970413956921817254698528785534443_i128),fld1: _4,fld2: (-26669_i16) };
_3 = _7;
_1.0 = [5_usize,12619363599732456450_usize,12991280779650239340_usize,2719395135880729383_usize,6_usize,15770287811561708294_usize,4_usize,532613873273035022_usize];
_6 = Adt52 { fld0: (-72397033204371728693049707395979725980_i128),fld1: _4,fld2: (-21228_i16) };
_6 = Adt52 { fld0: 131061175811928436395445402622565168959_i128,fld1: _4,fld2: (-1662_i16) };
_1.0 = [0_usize,2558622378281837923_usize,17070219442305097567_usize,6040529262699462631_usize,16954449119246404972_usize,7_usize,4386684193501695916_usize,0_usize];
_6.fld1 = _4;
_6 = Adt52 { fld0: 147476257125720399194313660870212651561_i128,fld1: _4,fld2: (-15432_i16) };
_4 = _6.fld1;
_11 = 132_u8;
_5 = 1564299320_u32 as isize;
match _11 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
132 => bb10,
_ => bb9
}
}
bb3 = {
_7 = !_3;
_10 = [4_usize,1_usize,752211504003721480_usize,7_usize,1536619400799185455_usize,12808075225465269907_usize,7_usize,4_usize];
_8 = [5_usize,4452364403897876345_usize,1_usize,2_usize,8149496611272645906_usize,6759444931646094995_usize,12476556623468276698_usize,12482995950303643494_usize];
_14 = _5;
_9 = (_10,);
_6.fld0 = (-39416389255206062153769041130767152680_i128) ^ 13059827985015369562514905094545221474_i128;
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
Return()
}
bb10 = {
_13 = _11 as f64;
_14 = _2;
_6 = Adt52 { fld0: 116260390922899929952743192456121740366_i128,fld1: _4,fld2: (-28751_i16) };
_4 = _6.fld1;
_6 = Adt52 { fld0: (-139846528067009754064790170939208324175_i128),fld1: _4,fld2: 23021_i16 };
_9 = (_10,);
_17 = _6.fld1;
_17 = _4;
_6.fld0 = !(-48707226825770647736404208281056452381_i128);
match _6.fld2 {
0 => bb5,
1 => bb11,
2 => bb12,
23021 => bb14,
_ => bb13
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_7 = !_3;
_10 = [4_usize,1_usize,752211504003721480_usize,7_usize,1536619400799185455_usize,12808075225465269907_usize,7_usize,4_usize];
_8 = [5_usize,4452364403897876345_usize,1_usize,2_usize,8149496611272645906_usize,6759444931646094995_usize,12476556623468276698_usize,12482995950303643494_usize];
_14 = _5;
_9 = (_10,);
_6.fld0 = (-39416389255206062153769041130767152680_i128) ^ 13059827985015369562514905094545221474_i128;
Goto(bb2)
}
bb14 = {
_10 = [1_usize,6_usize,4_usize,1_usize,7694993024697644373_usize,8952399921219376953_usize,3_usize,4_usize];
_4 = _6.fld1;
_11 = !116_u8;
_9 = (_1.0,);
_8 = [6_usize,2_usize,16236472879815247872_usize,0_usize,1_usize,5_usize,7_usize,17997448737574456330_usize];
_10 = [8183517772872789649_usize,164835617868813502_usize,1_usize,8692671228602099356_usize,6_usize,7_usize,2_usize,6166393993556718577_usize];
_2 = -_14;
_4 = _17;
_11 = !165_u8;
_11 = !198_u8;
_15 = [1935706978_i32,(-483463718_i32),664080158_i32];
_16 = Adt47::Variant1 { fld0: 254845126845651970558866016362402833230_u128 };
_4 = _6.fld1;
_9.0 = _8;
_14 = _2;
Goto(bb15)
}
bb15 = {
_2 = _14 | _14;
_18 = _6.fld1;
_6 = Adt52 { fld0: (-80605362925845065770104841803484997350_i128),fld1: _17,fld2: (-7710_i16) };
_13 = 77_i8 as f64;
_3 = !_7;
_19 = _11;
_4 = _6.fld1;
Goto(bb16)
}
bb16 = {
_9.0 = [6721711981171113310_usize,9620833235956573494_usize,15006421326199779036_usize,16451521360874226698_usize,55351179064506448_usize,5638929172777150688_usize,5698228241443588999_usize,1_usize];
_7 = _3;
_11 = 5_usize as u8;
_6 = Adt52 { fld0: 9782994585303804778026468554927772387_i128,fld1: _4,fld2: 32394_i16 };
_18 = _6.fld1;
_20.fld0 = 493458873_i32 | (-51368231_i32);
_2 = _14 & _14;
_11 = _2 as u8;
_13 = _6.fld2 as f64;
_6 = Adt52 { fld0: (-99472564523194005617467797619376354379_i128),fld1: _4,fld2: 31191_i16 };
_22 = !292856402127621757275753375774619668778_u128;
_10 = _8;
_9.0 = _1.0;
_18 = _17;
_20.fld0 = _7 as i32;
_1 = (_8,);
_1 = (_10,);
_9 = (_1.0,);
Call(_13 = fn6(_11, _9, _6.fld0, _14, _18, _4, _2, _11, _17, _9.0), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
place!(Field::<u128>(Variant(_16, 1), 0)) = _22 ^ _22;
_14 = _2;
_18 = _17;
_11 = !_19;
place!(Field::<u128>(Variant(_16, 1), 0)) = _3 as u128;
_21 = _13 as isize;
_15 = [_20.fld0,_20.fld0,_20.fld0];
_25 = _2;
_14 = 13451152936277971261_usize as isize;
_25 = Field::<u128>(Variant(_16, 1), 0) as isize;
_6.fld2 = 19670_i16;
SetDiscriminant(_16, 0);
Call(RET = fn7(_21, _13, _13, _6, _21, _2, _1.0), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_2 = _21 << _21;
_23.2 = _6.fld1;
_6.fld1 = _23.2;
_26 = 551849228_u32 << _2;
place!(Field::<u16>(Variant(_16, 0), 0)) = _7 + _3;
_26 = !4171955224_u32;
_13 = _7 as f64;
_6.fld1 = _17;
_24 = _6.fld1;
Goto(bb19)
}
bb19 = {
Call(_27 = dump_var(5_usize, 7_usize, Move(_7), 5_usize, Move(_5), 26_usize, Move(_26), 18_usize, Move(_18)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_27 = dump_var(5_usize, 24_usize, Move(_24), 15_usize, Move(_15), 10_usize, Move(_10), 14_usize, Move(_14)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_27 = dump_var(5_usize, 11_usize, Move(_11), 25_usize, Move(_25), 28_usize, _28, 28_usize, _28), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u8,mut _2: ([usize; 8],),mut _3: i128,mut _4: isize,mut _5: char,mut _6: char,mut _7: isize,mut _8: u8,mut _9: char,mut _10: [usize; 8]) -> f64 {
mir! {
type RET = f64;
let _11: Adt40;
let _12: (u16,);
let _13: [i32; 3];
let _14: Adt54;
let _15: bool;
let _16: u32;
let _17: [i64; 5];
let _18: char;
let _19: f32;
let _20: Adt52;
let _21: [usize; 7];
let _22: ();
let _23: ();
{
_4 = _7;
RET = 27132_u16 as f64;
_1 = _8 * _8;
_5 = _9;
_1 = _8 * _8;
RET = (-2_i8) as f64;
_4 = _7 ^ _7;
_9 = _5;
RET = 179705100880887042352728398957032476192_u128 as f64;
_2.0 = [11961901578483519551_usize,0_usize,0_usize,0_usize,3225182111280963779_usize,5084256022053321842_usize,5265085918925489705_usize,11193636590098342573_usize];
_1 = _8 + _8;
Goto(bb1)
}
bb1 = {
_6 = _5;
RET = (-6768057511374315565_i64) as f64;
RET = 1909749418_i32 as f64;
_6 = _5;
_6 = _9;
Goto(bb2)
}
bb2 = {
_7 = -_4;
_12.0 = 54948_u16;
_14.fld3 = Adt52 { fld0: _3,fld1: _9,fld2: (-156_i16) };
RET = 7623993719675189_i64 as f64;
_14.fld3.fld1 = _9;
_5 = _9;
Goto(bb3)
}
bb3 = {
_7 = RET as isize;
_14.fld1 = (_10,);
_3 = _14.fld3.fld0;
_14.fld0 = [5128052624194719200_i64,(-4275715536130613103_i64),8650326668392075889_i64,5798052514320674549_i64,4583590426334869651_i64];
RET = _1 as f64;
_9 = _14.fld3.fld1;
RET = _1 as f64;
_17 = [(-6767282039462924626_i64),(-4026004596176186382_i64),(-6016616899188446580_i64),2528079902868799088_i64,(-5817271069745388368_i64)];
_2.0 = [7_usize,14375911989606724741_usize,9916064659964286615_usize,4_usize,1_usize,15072779157117897412_usize,9485416215753204617_usize,18192825092814829698_usize];
_14.fld0 = _17;
_14.fld0 = [(-168078844620146017_i64),126944538393747339_i64,2877818773261684973_i64,6908574777261997381_i64,5383802493655340092_i64];
_9 = _14.fld3.fld1;
_6 = _5;
_16 = !1464236547_u32;
_19 = _14.fld3.fld0 as f32;
Goto(bb4)
}
bb4 = {
Call(_22 = dump_var(6_usize, 5_usize, Move(_5), 17_usize, Move(_17), 1_usize, Move(_1), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_22 = dump_var(6_usize, 12_usize, Move(_12), 16_usize, Move(_16), 23_usize, _23, 23_usize, _23), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: f64,mut _3: f64,mut _4: Adt52,mut _5: isize,mut _6: isize,mut _7: [usize; 8]) -> *const bool {
mir! {
type RET = *const bool;
let _8: bool;
let _9: (i128,);
let _10: [u64; 8];
let _11: char;
let _12: [usize; 7];
let _13: [usize; 7];
let _14: Adt44;
let _15: Adt40;
let _16: *mut (*mut (u16,), u8, char);
let _17: [usize; 7];
let _18: char;
let _19: i128;
let _20: i64;
let _21: ([usize; 8],);
let _22: [usize; 8];
let _23: Adt56;
let _24: isize;
let _25: [i32; 3];
let _26: Adt50;
let _27: [u32; 5];
let _28: *mut [i64; 5];
let _29: Adt51;
let _30: isize;
let _31: isize;
let _32: char;
let _33: Adt46;
let _34: u16;
let _35: Adt43;
let _36: [i32; 3];
let _37: ();
let _38: ();
{
_6 = 91070258389164556327377626697849977418_u128 as isize;
RET = core::ptr::addr_of!(_8);
_5 = _1 >> _1;
_5 = 3660187137_u32 as isize;
RET = core::ptr::addr_of!((*RET));
_4 = Adt52 { fld0: (-113377188891301279630562071189746310746_i128),fld1: '\u{6b09b}',fld2: 1546_i16 };
_1 = !_5;
_4.fld2 = (-6571_i16);
_2 = 21_i8 as f64;
_2 = -_3;
_10 = [9650163790292883882_u64,16836070912647955576_u64,6978924034405846724_u64,12355281932445653537_u64,6769485751382341002_u64,5816711036240046764_u64,7860387998905200004_u64,10693641310555542774_u64];
_9 = (_4.fld0,);
RET = core::ptr::addr_of!((*RET));
_9.0 = _4.fld0;
_6 = _1;
_4 = Adt52 { fld0: _9.0,fld1: '\u{9ff64}',fld2: (-24718_i16) };
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
226905178029637183832812536242021900710 => bb8,
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
_8 = _1 < _5;
_12 = [6568655734536644287_usize,6_usize,7_usize,16528842612909507309_usize,8972503969933950271_usize,4_usize,6097072383206881317_usize];
_7 = [0_usize,8458331240133085961_usize,6575627671576261884_usize,14350295966238368346_usize,409937407317006726_usize,5002609132728300267_usize,10310001071828968138_usize,17786842984925565133_usize];
_9.0 = -_4.fld0;
RET = core::ptr::addr_of!((*RET));
_12 = [8769028637581953879_usize,2_usize,3610432009246174599_usize,3_usize,5_usize,5_usize,5_usize];
_3 = 629985823_u32 as f64;
_10 = [2000982652750133463_u64,11874073580899162262_u64,15664555935542980778_u64,5144395247054238107_u64,13251911966260876001_u64,9536391394602957155_u64,13322497191534027588_u64,8294927442107936112_u64];
_5 = !_6;
_11 = _4.fld1;
_3 = _2;
_4.fld0 = _9.0;
_1 = -_6;
_8 = false;
_14.fld1 = core::ptr::addr_of!((*RET));
_9.0 = !_4.fld0;
_4.fld2 = -(-7125_i16);
_11 = _4.fld1;
Goto(bb9)
}
bb9 = {
_9.0 = _4.fld0 << _6;
_14.fld0 = 168_u8 as i32;
_2 = _4.fld2 as f64;
_9.0 = _4.fld2 as i128;
_9.0 = _4.fld0;
_3 = -_2;
_10 = [18148624921007411810_u64,7411930533421235798_u64,10873821955313154421_u64,14855484606741944515_u64,9808090683740259406_u64,3340753672721848893_u64,12802005108651750621_u64,12434380945165498905_u64];
_10 = [1546655888375678114_u64,4133325681945676896_u64,2029286155104431076_u64,7182701523953183215_u64,18087954865800682996_u64,11615059241284313886_u64,10257909087263592217_u64,7072639822466989169_u64];
_11 = _4.fld1;
_14.fld2 = core::ptr::addr_of!(_9);
_13 = _12;
_2 = _3;
_3 = _2;
_6 = 7705978769743736062_i64 as isize;
_2 = -_3;
_9.0 = _4.fld0;
_1 = _6 + _6;
_18 = _4.fld1;
_14.fld0 = 811925143_i32 & (-1351622969_i32);
Call(_3 = fn8(_12, _11, _11, _9.0, _1, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_14.fld0 = !(-1992055022_i32);
_19 = 13446888539333065699_u64 as i128;
_4.fld2 = _3 as i16;
_9.0 = _4.fld0;
_13 = [6091564817776249104_usize,1718756397510891013_usize,14193243230431142214_usize,7_usize,1_usize,2_usize,4_usize];
_7 = [13964117724081579137_usize,5883586282585046650_usize,3_usize,4_usize,0_usize,4_usize,18109442210150300584_usize,3_usize];
_9 = (_4.fld0,);
_7 = [6_usize,3_usize,3337061199049829479_usize,9364137807229736930_usize,9110715843736227910_usize,8403668079959421020_usize,4_usize,7_usize];
_10 = [4278696715403878567_u64,1834690177943907389_u64,17510270716829828986_u64,14827611007346134750_u64,8543892843706011091_u64,5599147070224419152_u64,12553857669031552444_u64,10793408314502976611_u64];
_12 = [10982413694198917122_usize,4024390844254113406_usize,2534077315645347109_usize,3_usize,5_usize,14036409180150039714_usize,4231396988576667225_usize];
_3 = (-59_i8) as f64;
_20 = (-1846787562007043485_i64);
_13 = [1_usize,3_usize,6_usize,18314458760734845754_usize,2_usize,18152371893176483975_usize,4_usize];
_10 = [18197593629465365747_u64,12853086710595783358_u64,7575526969993722367_u64,6134231198629551264_u64,5109759244316098733_u64,15045860343726842102_u64,18190459167820690868_u64,18340540927255301996_u64];
_18 = _4.fld1;
match _20 {
0 => bb1,
340282366920938463461527819869761167971 => bb11,
_ => bb3
}
}
bb11 = {
_1 = _6;
_17 = _12;
_21 = (_7,);
_13 = _12;
_23.fld1 = _18;
RET = core::ptr::addr_of!(_8);
_2 = _14.fld0 as f64;
_23.fld3.fld3 = [2306149989306294950_u64,1667093914711110725_u64,630548397953396511_u64,16161856246722688368_u64,16938484576768899099_u64,10218004769867082229_u64,7028180097281916269_u64,13099270471495759245_u64];
_23.fld3.fld0 = Adt40::Variant1 { fld0: _14.fld2 };
_23.fld3.fld4.fld1 = _7;
_23.fld3.fld3 = [6840190574751180076_u64,11185244011033799267_u64,2280140942912912321_u64,6770475741686656237_u64,7620899970718173709_u64,2317208182317820088_u64,6514963393075963923_u64,8039808700221723184_u64];
_23.fld4 = core::ptr::addr_of!(_14.fld3);
_23.fld3.fld4.fld1 = [10911911259416566495_usize,4_usize,4_usize,14876017799798599379_usize,15177042583869747499_usize,0_usize,16471763334841520096_usize,5_usize];
_9 = (_19,);
_23.fld3.fld4.fld2 = core::ptr::addr_of!(_14.fld3);
_17 = [4_usize,11035694999391840977_usize,5_usize,5673329690373661844_usize,4_usize,13490195614166402331_usize,17225833951837420522_usize];
_23.fld3.fld5 = _21;
_4.fld0 = 332785689037681078922789621967590816141_u128 as i128;
Goto(bb12)
}
bb12 = {
_23.fld2 = !15539633742479194464032973907843539663_u128;
_26 = Adt50::Variant0 { fld0: _23.fld3.fld5 };
_23.fld3.fld4.fld3 = (_19,);
_23.fld1 = _4.fld1;
place!(Field::<([usize; 8],)>(Variant(_26, 0), 0)).0 = _7;
_23.fld3.fld2 = -_5;
_23.fld6 = [_14.fld0,_14.fld0,_14.fld0];
_23.fld3.fld4.fld3 = _9;
_25 = [_14.fld0,_14.fld0,_14.fld0];
_7 = _23.fld3.fld4.fld1;
_26 = Adt50::Variant0 { fld0: _21 };
_30 = _6 * _5;
_25 = _23.fld6;
_15 = Move(_23.fld3.fld0);
Goto(bb13)
}
bb13 = {
_23.fld1 = _4.fld1;
_10 = [15829248643467844002_u64,9874165683599189078_u64,2739692319383814699_u64,8654019168350927401_u64,10943081055141687476_u64,1726719693877794085_u64,5463296363400477451_u64,5443768622099318247_u64];
_23.fld3.fld5.0 = Field::<([usize; 8],)>(Variant(_26, 0), 0).0;
_23.fld3.fld0 = Move(_15);
_23.fld3.fld5 = (_21.0,);
_1 = 14559403398248761485_usize as isize;
place!(Field::<*const (i128,)>(Variant(_23.fld3.fld0, 1), 0)) = _14.fld2;
SetDiscriminant(_26, 0);
_4.fld1 = _23.fld1;
place!(Field::<*const (i128,)>(Variant(_23.fld3.fld0, 1), 0)) = _14.fld2;
_23.fld0 = core::ptr::addr_of!(_20);
_18 = _4.fld1;
_20 = 0_usize as i64;
place!(Field::<([usize; 8],)>(Variant(_26, 0), 0)).0 = _21.0;
Call(_29 = fn10(_4.fld1, Move(_26), _11, _10, _23.fld3.fld2, _23.fld3.fld5, _20, _23.fld3.fld5.0, _4, _12, _12, _12), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_36 = [_14.fld0,_14.fld0,_14.fld0];
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(7_usize, 6_usize, Move(_6), 17_usize, Move(_17), 7_usize, Move(_7), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(7_usize, 25_usize, Move(_25), 1_usize, Move(_1), 13_usize, Move(_13), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(7_usize, 19_usize, Move(_19), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [usize; 7],mut _2: char,mut _3: char,mut _4: i128,mut _5: isize,mut _6: (i128,)) -> f64 {
mir! {
type RET = f64;
let _7: u16;
let _8: Adt42;
let _9: Adt53;
let _10: (i128,);
let _11: Adt48;
let _12: [u64; 8];
let _13: (u16,);
let _14: [u64; 8];
let _15: Adt47;
let _16: (i128,);
let _17: Adt44;
let _18: char;
let _19: (i128,);
let _20: [i64; 5];
let _21: f32;
let _22: [u32; 5];
let _23: f64;
let _24: isize;
let _25: [i32; 3];
let _26: Adt42;
let _27: Adt52;
let _28: i64;
let _29: f64;
let _30: *const *const *mut (u16,);
let _31: f64;
let _32: *const *const *mut (u16,);
let _33: u128;
let _34: [u64; 8];
let _35: *const (u16,);
let _36: bool;
let _37: f64;
let _38: Adt52;
let _39: i32;
let _40: ([usize; 8],);
let _41: ();
let _42: ();
{
_2 = _3;
RET = 173_u8 as f64;
_1 = [17051448171827436041_usize,13969924276872753439_usize,12520368788186406546_usize,7_usize,15396996933422790481_usize,17031617631382096327_usize,4_usize];
RET = _4 as f64;
_2 = _3;
RET = 102_u8 as f64;
_4 = !_6.0;
_4 = _5 as i128;
_7 = 11576_u16 | 5494_u16;
RET = _7 as f64;
Call(_4 = core::intrinsics::transmute(_6.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _3;
_6.0 = -_4;
RET = 1537328027_u32 as f64;
RET = 98_i8 as f64;
_1 = [7_usize,3_usize,5429894346928386939_usize,1_usize,3_usize,1742360522312852229_usize,7_usize];
_2 = _3;
_6 = (_4,);
_2 = _3;
RET = 2669783519_u32 as f64;
_4 = !_6.0;
Goto(bb2)
}
bb2 = {
_2 = _3;
_4 = !_6.0;
_8.fld3.0 = _4 ^ _6.0;
RET = 472568823_u32 as f64;
_3 = _2;
_10.0 = 322269807304444738919897432422475969402_u128 as i128;
_8.fld3.0 = _4 * _10.0;
_6 = _8.fld3;
RET = 32_u8 as f64;
_8.fld3 = (_6.0,);
_10 = _8.fld3;
_3 = _2;
_8.fld3.0 = 293143671883016725736706998403746131639_u128 as i128;
Goto(bb3)
}
bb3 = {
_8.fld3 = (_10.0,);
_6 = _10;
_1 = [0_usize,17515099086141879414_usize,7_usize,4424863214682075822_usize,6_usize,10682780200269228654_usize,7_usize];
_8.fld3 = (_6.0,);
_6 = (_10.0,);
_2 = _3;
_7 = 2868_u16;
_8.fld3.0 = 100955702446830266303927900290090987329_u128 as i128;
RET = _7 as f64;
RET = _5 as f64;
_1 = [4920689295593428697_usize,12272791705453000035_usize,0_usize,10272070726363361444_usize,6270210711506964950_usize,0_usize,7_usize];
Goto(bb4)
}
bb4 = {
_6 = (_8.fld3.0,);
_8.fld3.0 = _10.0 - _4;
_10.0 = !_8.fld3.0;
_13.0 = _7;
RET = (-8801704358196594611_i64) as f64;
_1 = [0_usize,6_usize,5_usize,3_usize,9777241475789830396_usize,4_usize,2_usize];
_10 = (_8.fld3.0,);
_7 = true as u16;
_14 = [4630975495369966918_u64,9097123619196883060_u64,9082619160593410695_u64,11293013238673117020_u64,15498631047697482021_u64,10630429162478010932_u64,13987625749323212680_u64,6565522932408960258_u64];
_3 = _2;
_13 = (_7,);
_3 = _2;
_6.0 = _13.0 as i128;
_8.fld4 = core::ptr::addr_of!(_13);
_3 = _2;
_13.0 = !_7;
_7 = 15303124222008719625_u64 as u16;
_4 = _8.fld3.0 + _10.0;
RET = _7 as f64;
_8.fld3.0 = !_4;
Call(_16 = fn9(_1, _4, _6.0, _8.fld3, _2, _8.fld3.0, _8.fld3, _8.fld3, _5, _3, _8.fld3, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12 = _14;
_6.0 = _16.0 >> _10.0;
Goto(bb6)
}
bb6 = {
_18 = _2;
_6 = _10;
_4 = _16.0;
_17.fld3 = core::ptr::addr_of_mut!(_13);
_8.fld4 = core::ptr::addr_of!(_13);
_18 = _2;
_18 = _2;
_4 = _6.0;
_20 = [(-7159507294689413152_i64),938166564516947130_i64,(-3415715134743842215_i64),2590866162885967031_i64,4832354127150485961_i64];
_13 = (_7,);
_12 = [5795123195657250395_u64,16127884109615882581_u64,12449279540711558902_u64,16862450419430386007_u64,8720457898572252716_u64,4474110068267125878_u64,7812038598259452330_u64,5609612635005637367_u64];
_19 = (_16.0,);
_8.fld1 = [7750347042749070202_usize,6019684369774153250_usize,1_usize,3_usize,7_usize,12981093476877052923_usize,10969478389454397311_usize,0_usize];
_2 = _18;
_20 = [(-6752617888224341449_i64),(-5531874829938209217_i64),(-5360208937173638498_i64),(-4002153686800459176_i64),5435853910179333733_i64];
_17.fld2 = core::ptr::addr_of!(_10);
_8.fld2 = core::ptr::addr_of!(_17.fld3);
_16.0 = !_19.0;
_20 = [3057339561104450556_i64,3319157179665918607_i64,(-3634518526288567920_i64),2520687782276297828_i64,(-7949089609895057359_i64)];
Goto(bb7)
}
bb7 = {
_4 = _19.0;
_22 = [2993074994_u32,3337446002_u32,3985131912_u32,3998218087_u32,88744472_u32];
RET = (-12781_i16) as f64;
_16 = (_19.0,);
_14 = _12;
_16 = (_8.fld3.0,);
_6.0 = RET as i128;
_18 = _2;
Goto(bb8)
}
bb8 = {
_25 = [710210833_i32,607463422_i32,(-812359728_i32)];
_26.fld2 = core::ptr::addr_of!(_17.fld3);
_24 = 3308227017_u32 as isize;
_26.fld1 = _8.fld1;
_26.fld1 = [6493993315063846510_usize,5_usize,16460043034777877787_usize,3300900159678534774_usize,4_usize,6936575711056383752_usize,9318224315811979577_usize,16611269849266284124_usize];
_17.fld3 = core::ptr::addr_of_mut!(_13);
_26.fld3 = (_19.0,);
_15 = Adt47::Variant1 { fld0: 159147996713252430586685661757302470429_u128 };
_23 = RET - RET;
_17.fld0 = 193225124_i32 << _26.fld3.0;
_26.fld4 = core::ptr::addr_of!(_13);
_14 = [13791918849637204176_u64,1814467487135630623_u64,10670587360616125365_u64,11252868434752211072_u64,6057757509011644348_u64,5378399403467886443_u64,14757152703220800751_u64,15867623978888674180_u64];
_26.fld2 = core::ptr::addr_of!(_17.fld3);
_13 = (_7,);
_27.fld1 = _2;
_8.fld3 = (_26.fld3.0,);
Goto(bb9)
}
bb9 = {
_25 = [_17.fld0,_17.fld0,_17.fld0];
_26.fld2 = core::ptr::addr_of!(_17.fld3);
_24 = _5;
_27 = Adt52 { fld0: _16.0,fld1: _18,fld2: 16734_i16 };
_8.fld3 = (_26.fld3.0,);
_27 = Adt52 { fld0: _19.0,fld1: _2,fld2: (-18838_i16) };
_15 = Adt47::Variant1 { fld0: 15012601557828200195525960338130322196_u128 };
_20 = [(-1364928288809000935_i64),6145878925460917444_i64,(-5076808930800301997_i64),(-7977326596394287021_i64),(-740208698176201206_i64)];
_8.fld1 = [7_usize,4_usize,6315377141572642955_usize,3_usize,9894925650916126155_usize,1_usize,6_usize,0_usize];
_6.0 = _19.0;
_23 = -RET;
_26.fld3 = _19;
Goto(bb10)
}
bb10 = {
_19 = (_26.fld3.0,);
_29 = -RET;
_26.fld3.0 = !_27.fld0;
_26.fld3.0 = _10.0;
_5 = _24 * _24;
RET = 169_u8 as f64;
_26.fld4 = _8.fld4;
_8.fld1 = [16699845427719849536_usize,15087667613689496705_usize,7_usize,2_usize,5_usize,7_usize,866371345379540065_usize,7_usize];
_8.fld4 = _26.fld4;
_26.fld3 = (_8.fld3.0,);
_20 = [(-8525814213179823829_i64),(-4403718149491131153_i64),(-4836018803566582509_i64),(-5150412270646499_i64),(-1092198269708814647_i64)];
_8.fld3.0 = _2 as i128;
_30 = core::ptr::addr_of!(_8.fld2);
_13 = (_7,);
_16.0 = -_6.0;
_18 = _2;
RET = _23;
Goto(bb11)
}
bb11 = {
_25 = [_17.fld0,_17.fld0,_17.fld0];
_20 = [(-3564794777576858259_i64),3235917031232732619_i64,6655944021730152791_i64,1901000283574205243_i64,(-2088685787464778391_i64)];
_27.fld1 = _3;
_25 = [_17.fld0,_17.fld0,_17.fld0];
_17.fld3 = core::ptr::addr_of_mut!(_13);
_32 = _30;
_26.fld2 = core::ptr::addr_of!(_17.fld3);
_21 = _26.fld3.0 as f32;
place!(Field::<u128>(Variant(_15, 1), 0)) = _13.0 as u128;
_16.0 = -_27.fld0;
_18 = _2;
_17.fld0 = (-489003045_i32);
Call(_28 = core::intrinsics::transmute(_24), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_30 = core::ptr::addr_of!((*_30));
_8.fld3 = _16;
_20 = [_28,_28,_28,_28,_28];
_15 = Adt47::Variant1 { fld0: 162966756919395941484998712812283364563_u128 };
_3 = _27.fld1;
match _27.fld2 {
0 => bb9,
1 => bb13,
340282366920938463463374607431768192618 => bb15,
_ => bb14
}
}
bb13 = {
_4 = _19.0;
_22 = [2993074994_u32,3337446002_u32,3985131912_u32,3998218087_u32,88744472_u32];
RET = (-12781_i16) as f64;
_16 = (_19.0,);
_14 = _12;
_16 = (_8.fld3.0,);
_6.0 = RET as i128;
_18 = _2;
Goto(bb8)
}
bb14 = {
_25 = [710210833_i32,607463422_i32,(-812359728_i32)];
_26.fld2 = core::ptr::addr_of!(_17.fld3);
_24 = 3308227017_u32 as isize;
_26.fld1 = _8.fld1;
_26.fld1 = [6493993315063846510_usize,5_usize,16460043034777877787_usize,3300900159678534774_usize,4_usize,6936575711056383752_usize,9318224315811979577_usize,16611269849266284124_usize];
_17.fld3 = core::ptr::addr_of_mut!(_13);
_26.fld3 = (_19.0,);
_15 = Adt47::Variant1 { fld0: 159147996713252430586685661757302470429_u128 };
_23 = RET - RET;
_17.fld0 = 193225124_i32 << _26.fld3.0;
_26.fld4 = core::ptr::addr_of!(_13);
_14 = [13791918849637204176_u64,1814467487135630623_u64,10670587360616125365_u64,11252868434752211072_u64,6057757509011644348_u64,5378399403467886443_u64,14757152703220800751_u64,15867623978888674180_u64];
_26.fld2 = core::ptr::addr_of!(_17.fld3);
_13 = (_7,);
_27.fld1 = _2;
_8.fld3 = (_26.fld3.0,);
Goto(bb9)
}
bb15 = {
_27 = Adt52 { fld0: _19.0,fld1: _18,fld2: (-14530_i16) };
_31 = -_29;
_26.fld3 = (_4,);
_21 = 1698726096_u32 as f32;
_33 = 143778646174989406483942299857400960362_u128 * 275939119736885748455645469870699382660_u128;
_13 = (_7,);
_24 = _5 | _5;
_30 = _32;
_19 = (_4,);
RET = _29;
RET = -_31;
_8.fld1 = [4_usize,2350389822560869961_usize,14356493003449989813_usize,7289764621481264122_usize,7_usize,9131345984735184886_usize,4_usize,4503376015471348555_usize];
_2 = _27.fld1;
_8.fld0 = core::ptr::addr_of!(_36);
_10 = (_4,);
_35 = _8.fld4;
Goto(bb16)
}
bb16 = {
Call(_41 = dump_var(8_usize, 1_usize, Move(_1), 4_usize, Move(_4), 12_usize, Move(_12), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(8_usize, 19_usize, Move(_19), 28_usize, Move(_28), 24_usize, Move(_24), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(8_usize, 3_usize, Move(_3), 16_usize, Move(_16), 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [usize; 7],mut _2: i128,mut _3: i128,mut _4: (i128,),mut _5: char,mut _6: i128,mut _7: (i128,),mut _8: (i128,),mut _9: isize,mut _10: char,mut _11: (i128,),mut _12: char) -> (i128,) {
mir! {
type RET = (i128,);
let _13: u128;
let _14: [u32; 5];
let _15: [i64; 5];
let _16: Adt47;
let _17: i64;
let _18: ();
let _19: ();
{
_6 = _11.0 + _4.0;
_7 = (_8.0,);
_13 = !222003741257468259503209868806629896471_u128;
_10 = _12;
_4.0 = _5 as i128;
_11.0 = _2 + _6;
RET = (_11.0,);
RET = (_6,);
_11.0 = true as i128;
_10 = _5;
_8 = (_2,);
_11 = (_6,);
RET.0 = 54_u8 as i128;
_8 = (_2,);
_3 = _8.0;
RET.0 = _6 >> _11.0;
_1 = [5159661999441549098_usize,7577886671887120214_usize,3_usize,2_usize,6292091215833629655_usize,1_usize,1235285598151434130_usize];
_1 = [8853934951041567526_usize,2_usize,4_usize,4_usize,7_usize,4504501786714215165_usize,5_usize];
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(9_usize, 12_usize, Move(_12), 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(9_usize, 2_usize, Move(_2), 9_usize, Move(_9), 19_usize, _19, 19_usize, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: char,mut _2: Adt50,mut _3: char,mut _4: [u64; 8],mut _5: isize,mut _6: ([usize; 8],),mut _7: i64,mut _8: [usize; 8],mut _9: Adt52,mut _10: [usize; 7],mut _11: [usize; 7],mut _12: [usize; 7]) -> Adt51 {
mir! {
type RET = Adt51;
let _13: u8;
let _14: f32;
let _15: usize;
let _16: isize;
let _17: bool;
let _18: f64;
let _19: [i64; 5];
let _20: (&'static bool, char);
let _21: u128;
let _22: f64;
let _23: Adt44;
let _24: [usize; 7];
let _25: u32;
let _26: [u32; 5];
let _27: ([usize; 8],);
let _28: char;
let _29: [usize; 8];
let _30: Adt43;
let _31: [u64; 8];
let _32: ([usize; 8],);
let _33: f64;
let _34: Adt47;
let _35: usize;
let _36: [usize; 8];
let _37: *mut [i64; 5];
let _38: isize;
let _39: *mut (*mut (u16,), u8, char);
let _40: isize;
let _41: Adt41;
let _42: Adt43;
let _43: isize;
let _44: isize;
let _45: [i32; 3];
let _46: [usize; 8];
let _47: (u16,);
let _48: Adt52;
let _49: isize;
let _50: u64;
let _51: u16;
let _52: [u64; 8];
let _53: Adt55;
let _54: [u64; 8];
let _55: isize;
let _56: *const *const *mut (u16,);
let _57: bool;
let _58: f64;
let _59: f64;
let _60: isize;
let _61: ();
let _62: ();
{
_9.fld0 = !104730130219488146018180985974319025129_i128;
_3 = _9.fld1;
_9.fld2 = (-3381_i16);
_9 = Adt52 { fld0: (-6391236618297636484619659138943951178_i128),fld1: _1,fld2: 19803_i16 };
_11 = [6_usize,3129646544589917079_usize,11984674527600288586_usize,5_usize,8511923355000355011_usize,1_usize,6367774278079515706_usize];
match _9.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
19803 => bb5,
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
_1 = _3;
_12 = [2_usize,3_usize,2_usize,10396693557569250727_usize,5_usize,11805499668261175884_usize,10066353352927395970_usize];
_6.0 = [1817933742985012842_usize,2_usize,17249006870245771831_usize,1278081238179828656_usize,5_usize,2_usize,6879375294262479670_usize,2_usize];
_9.fld2 = (-16002_i16);
_12 = [12408510901408439673_usize,0_usize,6_usize,7_usize,2_usize,0_usize,0_usize];
_3 = _9.fld1;
_7 = 9067875795554212145_i64;
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)) = _6;
_14 = _5 as f32;
_9.fld0 = 83900995744860788098497802100705477616_i128 * 162634416866752138789445472894639481033_i128;
_13 = 35_u8 + 64_u8;
_9.fld0 = _9.fld1 as i128;
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)).0 = [2_usize,15827305987049741587_usize,3_usize,7_usize,0_usize,2_usize,11442220231483503594_usize,4_usize];
_12 = [2342015752797228302_usize,6_usize,4_usize,4624362366235173809_usize,4_usize,2_usize,4132777534161773449_usize];
_7 = !5485329712262686852_i64;
_15 = 5085724046310583441_usize;
_9.fld1 = _1;
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)) = (_8,);
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)).0 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9.fld0 = (-93872695543208621606730401210158838158_i128);
_9 = Adt52 { fld0: (-19093233211251578309103307229852842276_i128),fld1: _1,fld2: (-32260_i16) };
SetDiscriminant(_2, 1);
match _9.fld0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
321189133709686885154271300201915369180 => bb9,
_ => bb8
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
_12 = [_15,_15,_15,_15,_15,_15,_15];
_8 = _6.0;
_5 = 10633_u16 as isize;
Call(place!(Field::<[u64; 8]>(Variant(_2, 1), 1)) = fn11(_1, _1, _11, _8, _6, _9.fld0, _1, _9.fld0, _8), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1 = _9.fld1;
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = core::ptr::addr_of!(_17);
_9.fld1 = _1;
_4 = [4670558801851210463_u64,8156788136057998918_u64,7149096329094374178_u64,3958063654535151502_u64,5038127106778782248_u64,76426784013117710_u64,1728694339014301994_u64,4145008592116455919_u64];
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld0 = (-1444959862_i32) + (-1576522528_i32);
_2 = Adt50::Variant0 { fld0: _6 };
_12 = [_15,_15,_15,_15,_15,_15,_15];
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)).0 = [_15,_15,_15,_15,_15,_15,_15,_15];
_13 = !251_u8;
_2 = Adt50::Variant0 { fld0: _6 };
_1 = _9.fld1;
_12 = _11;
_11 = [_15,_15,_15,_15,_15,_15,_15];
_18 = (-69_i8) as f64;
_12 = [_15,_15,_15,_15,_15,_15,_15];
_17 = !true;
_10 = [_15,_15,_15,_15,_15,_15,_15];
_11 = _10;
_10 = [_15,_15,_15,_15,_15,_15,_15];
_9 = Adt52 { fld0: (-163708497416222861969441956394769368872_i128),fld1: _3,fld2: (-7081_i16) };
Goto(bb11)
}
bb11 = {
_9 = Adt52 { fld0: (-63274098500190782014539579735920325236_i128),fld1: _3,fld2: 3105_i16 };
_21 = !327832124069392527658127585339763317397_u128;
_6 = (_8,);
_17 = _9.fld2 < _9.fld2;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_10 = _11;
SetDiscriminant(_2, 1);
_9.fld2 = 11644_i16 * 6305_i16;
_3 = _1;
_16 = !_5;
place!(Field::<[u64; 8]>(Variant(_2, 1), 1)) = [7816623593393613537_u64,9800360693370734193_u64,14720474535775201068_u64,11613522041695289158_u64,17058425033068953957_u64,3869178427707350889_u64,4399499278524581542_u64,8027025037505326213_u64];
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = core::ptr::addr_of!(_17);
_20.1 = _1;
_7 = 67_i8 as i64;
place!(Field::<[u64; 8]>(Variant(_2, 1), 1)) = [3224184679659328621_u64,10903686161768700707_u64,16533635319820630315_u64,2048221245450098286_u64,18338994927255741575_u64,15049426160403220818_u64,17965980681906917449_u64,7421078514573001952_u64];
_6.0 = _8;
_3 = _9.fld1;
_19 = [_7,_7,_7,_7,_7];
_9.fld2 = _13 as i16;
_17 = !false;
_19 = [_7,_7,_7,_7,_7];
place!(Field::<u8>(Variant(_2, 1), 2)) = _13 | _13;
_6 = (_8,);
_4 = [3994246564355559419_u64,8991750813765095812_u64,16895907114057995097_u64,6141732976187467613_u64,3568505188924278314_u64,9713611346566462333_u64,1054287996013433068_u64,14470061364518343103_u64];
_6 = (_8,);
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld0 = 239602494_i32;
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = core::ptr::addr_of!(_17);
_13 = _18 as u8;
match _9.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
277008268420747681448835027695847886220 => bb12,
_ => bb8
}
}
bb12 = {
_17 = _9.fld0 >= _9.fld0;
match _9.fld0 {
0 => bb7,
1 => bb10,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
277008268420747681448835027695847886220 => bb19,
_ => bb18
}
}
bb13 = {
_9 = Adt52 { fld0: (-63274098500190782014539579735920325236_i128),fld1: _3,fld2: 3105_i16 };
_21 = !327832124069392527658127585339763317397_u128;
_6 = (_8,);
_17 = _9.fld2 < _9.fld2;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_10 = _11;
SetDiscriminant(_2, 1);
_9.fld2 = 11644_i16 * 6305_i16;
_3 = _1;
_16 = !_5;
place!(Field::<[u64; 8]>(Variant(_2, 1), 1)) = [7816623593393613537_u64,9800360693370734193_u64,14720474535775201068_u64,11613522041695289158_u64,17058425033068953957_u64,3869178427707350889_u64,4399499278524581542_u64,8027025037505326213_u64];
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = core::ptr::addr_of!(_17);
_20.1 = _1;
_7 = 67_i8 as i64;
place!(Field::<[u64; 8]>(Variant(_2, 1), 1)) = [3224184679659328621_u64,10903686161768700707_u64,16533635319820630315_u64,2048221245450098286_u64,18338994927255741575_u64,15049426160403220818_u64,17965980681906917449_u64,7421078514573001952_u64];
_6.0 = _8;
_3 = _9.fld1;
_19 = [_7,_7,_7,_7,_7];
_9.fld2 = _13 as i16;
_17 = !false;
_19 = [_7,_7,_7,_7,_7];
place!(Field::<u8>(Variant(_2, 1), 2)) = _13 | _13;
_6 = (_8,);
_4 = [3994246564355559419_u64,8991750813765095812_u64,16895907114057995097_u64,6141732976187467613_u64,3568505188924278314_u64,9713611346566462333_u64,1054287996013433068_u64,14470061364518343103_u64];
_6 = (_8,);
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld0 = 239602494_i32;
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = core::ptr::addr_of!(_17);
_13 = _18 as u8;
match _9.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
277008268420747681448835027695847886220 => bb12,
_ => bb8
}
}
bb14 = {
_1 = _3;
_12 = [2_usize,3_usize,2_usize,10396693557569250727_usize,5_usize,11805499668261175884_usize,10066353352927395970_usize];
_6.0 = [1817933742985012842_usize,2_usize,17249006870245771831_usize,1278081238179828656_usize,5_usize,2_usize,6879375294262479670_usize,2_usize];
_9.fld2 = (-16002_i16);
_12 = [12408510901408439673_usize,0_usize,6_usize,7_usize,2_usize,0_usize,0_usize];
_3 = _9.fld1;
_7 = 9067875795554212145_i64;
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)) = _6;
_14 = _5 as f32;
_9.fld0 = 83900995744860788098497802100705477616_i128 * 162634416866752138789445472894639481033_i128;
_13 = 35_u8 + 64_u8;
_9.fld0 = _9.fld1 as i128;
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)).0 = [2_usize,15827305987049741587_usize,3_usize,7_usize,0_usize,2_usize,11442220231483503594_usize,4_usize];
_12 = [2342015752797228302_usize,6_usize,4_usize,4624362366235173809_usize,4_usize,2_usize,4132777534161773449_usize];
_7 = !5485329712262686852_i64;
_15 = 5085724046310583441_usize;
_9.fld1 = _1;
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)) = (_8,);
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)).0 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9.fld0 = (-93872695543208621606730401210158838158_i128);
_9 = Adt52 { fld0: (-19093233211251578309103307229852842276_i128),fld1: _1,fld2: (-32260_i16) };
SetDiscriminant(_2, 1);
match _9.fld0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
321189133709686885154271300201915369180 => bb9,
_ => bb8
}
}
bb15 = {
_12 = [_15,_15,_15,_15,_15,_15,_15];
_8 = _6.0;
_5 = 10633_u16 as isize;
Call(place!(Field::<[u64; 8]>(Variant(_2, 1), 1)) = fn11(_1, _1, _11, _8, _6, _9.fld0, _1, _9.fld0, _8), ReturnTo(bb10), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_6 = (_8,);
_20.0 = &_17;
_9.fld0 = !(-147887236043631116941383428796290867099_i128);
_14 = _9.fld2 as f32;
_5 = -_16;
_6 = (_8,);
_25 = 3571226282_u32 + 3182940096_u32;
_18 = Field::<Adt44>(Variant(_2, 1), 0).fld0 as f64;
_4 = [10455906317448410172_u64,1878162225295014695_u64,8564502631666596086_u64,182770683362784872_u64,2839461855785011758_u64,6966187304810644593_u64,9967827360910438590_u64,9400238286343935800_u64];
_26 = [_25,_25,_25,_25,_25];
_19 = [_7,_7,_7,_7,_7];
_8 = _6.0;
_6 = (_8,);
_3 = _9.fld1;
_3 = _1;
match _15 {
0 => bb16,
1 => bb20,
5085724046310583441 => bb22,
_ => bb21
}
}
bb20 = {
_1 = _9.fld1;
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = core::ptr::addr_of!(_17);
_9.fld1 = _1;
_4 = [4670558801851210463_u64,8156788136057998918_u64,7149096329094374178_u64,3958063654535151502_u64,5038127106778782248_u64,76426784013117710_u64,1728694339014301994_u64,4145008592116455919_u64];
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld0 = (-1444959862_i32) + (-1576522528_i32);
_2 = Adt50::Variant0 { fld0: _6 };
_12 = [_15,_15,_15,_15,_15,_15,_15];
place!(Field::<([usize; 8],)>(Variant(_2, 0), 0)).0 = [_15,_15,_15,_15,_15,_15,_15,_15];
_13 = !251_u8;
_2 = Adt50::Variant0 { fld0: _6 };
_1 = _9.fld1;
_12 = _11;
_11 = [_15,_15,_15,_15,_15,_15,_15];
_18 = (-69_i8) as f64;
_12 = [_15,_15,_15,_15,_15,_15,_15];
_17 = !true;
_10 = [_15,_15,_15,_15,_15,_15,_15];
_11 = _10;
_10 = [_15,_15,_15,_15,_15,_15,_15];
_9 = Adt52 { fld0: (-163708497416222861969441956394769368872_i128),fld1: _3,fld2: (-7081_i16) };
Goto(bb11)
}
bb21 = {
Return()
}
bb22 = {
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld0 = _7 as i32;
_29 = _8;
_23.fld1 = Field::<Adt44>(Variant(_2, 1), 0).fld1;
_29 = _8;
_6 = (_8,);
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = _23.fld1;
_22 = _18 - _18;
_4 = [11495620200421991940_u64,8979527041674510261_u64,17617043665235038019_u64,1500083466538466492_u64,8597045335771023678_u64,6366888665904696360_u64,8965723245455915883_u64,7873626776883939327_u64];
_12 = [_15,_15,_15,_15,_15,_15,_15];
_30.fld4.fld0 = _23.fld1;
_30.fld4.fld3.0 = -_9.fld0;
_9.fld1 = _20.1;
_30.fld4.fld2 = core::ptr::addr_of!(place!(Field::<Adt44>(Variant(_2, 1), 0)).fld3);
_30.fld5.0 = [_15,_15,_15,_15,_15,_15,_15,_15];
_18 = 30630_u16 as f64;
_27.0 = _6.0;
_17 = !true;
_13 = Field::<u8>(Variant(_2, 1), 2) << Field::<u8>(Variant(_2, 1), 2);
match _15 {
5085724046310583441 => bb23,
_ => bb7
}
}
bb23 = {
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = core::ptr::addr_of!(_17);
_3 = _1;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_24 = [_15,_15,_15,_15,_15,_15,_15];
place!(Field::<u8>(Variant(_2, 1), 2)) = !_13;
_9 = Adt52 { fld0: _30.fld4.fld3.0,fld1: _3,fld2: (-16837_i16) };
_30.fld4.fld1 = [_15,_15,_15,_15,_15,_15,_15,_15];
_33 = _14 as f64;
_9 = Adt52 { fld0: _30.fld4.fld3.0,fld1: _20.1,fld2: 17701_i16 };
_3 = _20.1;
_15 = 12786588338230800348_usize;
_2 = Adt50::Variant0 { fld0: _30.fld5 };
_32.0 = [_15,_15,_15,_15,_15,_15,_15,_15];
_10 = _24;
_30.fld6 = !64421_u16;
_33 = _22 * _22;
_20.0 = &_17;
_25 = _30.fld4.fld3.0 as u32;
_21 = !114291044167091086930760997806099846368_u128;
_9.fld1 = _1;
Goto(bb24)
}
bb24 = {
_1 = _20.1;
_31 = [6144402461358386729_u64,11809322785371157766_u64,400376181781335777_u64,5703056964094084542_u64,6904951195052195703_u64,6595942534564634293_u64,12869850217555113576_u64,9768180780125417184_u64];
SetDiscriminant(_2, 1);
_35 = _15 - _15;
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = _30.fld4.fld0;
_30.fld4.fld3 = (_9.fld0,);
_6.0 = [_35,_35,_35,_35,_35,_15,_35,_15];
_30.fld2 = -_16;
_9.fld1 = _1;
_37 = core::ptr::addr_of_mut!(_19);
_30.fld0 = Adt40::Variant2 { fld0: _7,fld1: _25,fld2: _31,fld3: _23.fld1 };
_15 = _35 + _35;
_30.fld5 = (_6.0,);
_36 = _30.fld5.0;
_29 = [_15,_15,_35,_15,_15,_15,_15,_35];
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld1 = _30.fld4.fld0;
_30.fld3 = Field::<[u64; 8]>(Variant(_30.fld0, 2), 2);
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld2 = core::ptr::addr_of!(_42.fld4.fld3);
_42.fld5 = _30.fld5;
_42.fld3 = [3105041526151356821_u64,15324493104392626288_u64,15460757768973042203_u64,865245520673675906_u64,16775797571242422671_u64,16164800365147437614_u64,15682230720863934969_u64,11722525707227634049_u64];
_38 = !_16;
_42.fld4.fld2 = core::ptr::addr_of!(place!(Field::<Adt44>(Variant(_2, 1), 0)).fld3);
_26 = [Field::<u32>(Variant(_30.fld0, 2), 1),_25,Field::<u32>(Variant(_30.fld0, 2), 1),_25,_25];
SetDiscriminant(_30.fld0, 3);
Goto(bb25)
}
bb25 = {
_31 = _42.fld3;
_23.fld2 = core::ptr::addr_of!(_42.fld4.fld3);
_27 = (_6.0,);
_34 = Adt47::Variant1 { fld0: _21 };
place!(Field::<u128>(Variant(_34, 1), 0)) = _21 ^ _21;
_42.fld3 = [11051314572344295612_u64,1515872972297050845_u64,5119941989773851499_u64,1707075346119296656_u64,5837296214913309362_u64,7514237833485161548_u64,2553402466423563258_u64,15553124629797057807_u64];
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld0 = Field::<u128>(Variant(_34, 1), 0) as i32;
_22 = _33 - _33;
_42.fld0 = Adt40::Variant2 { fld0: _7,fld1: _25,fld2: _30.fld3,fld3: _30.fld4.fld0 };
Goto(bb26)
}
bb26 = {
_9.fld0 = _30.fld4.fld3.0 ^ _30.fld4.fld3.0;
_18 = _33;
_1 = _9.fld1;
_42.fld4.fld0 = _23.fld1;
_3 = _9.fld1;
_42.fld2 = _5;
SetDiscriminant(_34, 0);
SetDiscriminant(_42.fld0, 0);
_7 = 8521191030599711703_u64 as i64;
_13 = 78_u8;
_24 = [_35,_15,_15,_15,_35,_35,_15];
_42.fld5 = (_29,);
_42.fld5 = _30.fld5;
_45 = [Field::<Adt44>(Variant(_2, 1), 0).fld0,Field::<Adt44>(Variant(_2, 1), 0).fld0,Field::<Adt44>(Variant(_2, 1), 0).fld0];
_28 = _1;
_42.fld0 = Adt40::Variant0 { fld0: _21,fld1: _25 };
_24 = [_35,_15,_35,_35,_15,_15,_15];
_40 = _42.fld2;
_30.fld4.fld1 = [_15,_35,_35,_15,_15,_35,_15,_35];
_42.fld5 = (_30.fld5.0,);
place!(Field::<*const (u16,)>(Variant(_30.fld0, 3), 0)) = core::ptr::addr_of!(_47);
_11 = [_15,_15,_15,_35,_15,_35,_15];
_32 = _6;
place!(Field::<u8>(Variant(_2, 1), 2)) = _13 & _13;
_9.fld1 = _20.1;
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld0 = (-1673468932_i32) - 591716316_i32;
_30.fld4.fld3.0 = _9.fld0;
Goto(bb27)
}
bb27 = {
_44 = -_42.fld2;
_43 = _30.fld2;
_29 = [_35,_15,_15,_15,_15,_15,_35,_35];
_31 = [16264435912818502852_u64,8414058728919828827_u64,16056280827007831135_u64,9252533826533010673_u64,16240390883704084209_u64,1891754801746950975_u64,3527443523815743159_u64,2170011056486092090_u64];
place!(Field::<[u64; 8]>(Variant(_2, 1), 1)) = _30.fld3;
_30.fld7 = 3111989891636193941_u64;
place!(Field::<Adt44>(Variant(_2, 1), 0)).fld2 = core::ptr::addr_of!(_30.fld4.fld3);
_30.fld7 = 1212201774456531887_u64 + 10951225352324847457_u64;
_40 = _42.fld2;
_46 = [_35,_35,_35,_15,_35,_15,_15,_35];
_30.fld2 = _43 ^ _44;
_36 = [_15,_15,_15,_15,_35,_35,_35,_15];
_42.fld3 = [_30.fld7,_30.fld7,_30.fld7,_30.fld7,_30.fld7,_30.fld7,_30.fld7,_30.fld7];
_5 = _42.fld2 + _42.fld2;
_38 = !_44;
_30.fld4.fld0 = Field::<Adt44>(Variant(_2, 1), 0).fld1;
_14 = _30.fld4.fld3.0 as f32;
_48.fld1 = _28;
RET = Adt51::Variant0 { fld0: _30.fld3,fld1: _24,fld2: _42.fld4.fld2,fld3: Move(_30.fld0),fld4: Field::<Adt44>(Variant(_2, 1), 0).fld2 };
_42.fld7 = _30.fld7 * _30.fld7;
SetDiscriminant(Field::<Adt40>(Variant(RET, 0), 3), 1);
_42.fld4.fld3 = (_30.fld4.fld3.0,);
_25 = Field::<u32>(Variant(_42.fld0, 0), 1) - Field::<u32>(Variant(_42.fld0, 0), 1);
_30.fld5.0 = _27.0;
Goto(bb28)
}
bb28 = {
RET = Adt51::Variant0 { fld0: _4,fld1: _24,fld2: _30.fld4.fld2,fld3: Move(_42.fld0),fld4: _23.fld2 };
_27 = (_32.0,);
_24 = _12;
_12 = _24;
_42.fld6 = !_30.fld6;
_47 = (_30.fld6,);
_27 = _30.fld5;
_48 = Adt52 { fld0: _9.fld0,fld1: _20.1,fld2: _9.fld2 };
_24 = [_15,_35,_35,_15,_15,_15,_15];
_8 = [_15,_35,_35,_15,_15,_15,_15,_15];
_42.fld7 = _48.fld2 as u64;
_30.fld3 = [_42.fld7,_42.fld7,_42.fld7,_42.fld7,_30.fld7,_42.fld7,_42.fld7,_30.fld7];
_42.fld7 = _30.fld7 << _9.fld0;
place!(Field::<u128>(Variant(place!(Field::<Adt40>(Variant(RET, 0), 3)), 0), 0)) = !_21;
_11 = Field::<[usize; 7]>(Variant(RET, 0), 1);
SetDiscriminant(Field::<Adt40>(Variant(RET, 0), 3), 0);
place!(Field::<u32>(Variant(place!(Field::<Adt40>(Variant(RET, 0), 3)), 0), 1)) = _25;
_51 = !_30.fld6;
place!(Field::<[u64; 8]>(Variant(RET, 0), 0)) = [_42.fld7,_30.fld7,_42.fld7,_42.fld7,_42.fld7,_42.fld7,_42.fld7,_30.fld7];
_30.fld4.fld0 = _42.fld4.fld0;
place!(Field::<[usize; 7]>(Variant(RET, 0), 1)) = [_15,_15,_15,_35,_15,_15,_35];
Goto(bb29)
}
bb29 = {
_21 = 157053573276814276763738975009981449145_u128;
_47.0 = _14 as u16;
_42.fld4.fld4 = core::ptr::addr_of!(_47);
Goto(bb30)
}
bb30 = {
_30.fld6 = _42.fld6;
_56 = core::ptr::addr_of!(place!(Field::<*const *mut (u16,)>(Variant(RET, 0), 2)));
_53 = Adt55 { fld0: Field::<Adt44>(Variant(_2, 1), 0).fld0,fld1: Field::<u32>(Variant(Field::<Adt40>(Variant(RET, 0), 3), 0), 1),fld2: Field::<Adt44>(Variant(_2, 1), 0).fld2,fld3: _30.fld5 };
place!(Field::<*const (i128,)>(Variant(RET, 0), 4)) = core::ptr::addr_of!(_30.fld4.fld3);
_41 = Adt41::Variant0 { fld0: _17,fld1: _51,fld2: _11,fld3: _42.fld4.fld3,fld4: _15,fld5: _14,fld6: _42.fld4.fld4 };
_42.fld0 = Adt40::Variant1 { fld0: Field::<*const (i128,)>(Variant(RET, 0), 4) };
_53 = Adt55 { fld0: Field::<Adt44>(Variant(_2, 1), 0).fld0,fld1: _25,fld2: _23.fld2,fld3: _30.fld5 };
place!(Field::<usize>(Variant(_41, 0), 4)) = !_15;
_2 = Adt50::Variant0 { fld0: _30.fld5 };
_18 = -_33;
place!(Field::<(i128,)>(Variant(_41, 0), 3)).0 = _17 as i128;
place!(Field::<u16>(Variant(_34, 0), 0)) = !_51;
place!(Field::<u32>(Variant(place!(Field::<Adt40>(Variant(RET, 0), 3)), 0), 1)) = _48.fld2 as u32;
_47 = (Field::<u16>(Variant(_41, 0), 1),);
_1 = _3;
_2 = Adt50::Variant0 { fld0: _32 };
_30.fld6 = _51 + _42.fld6;
_10 = [Field::<usize>(Variant(_41, 0), 4),Field::<usize>(Variant(_41, 0), 4),_35,_15,Field::<usize>(Variant(_41, 0), 4),_15,_15];
place!(Field::<u128>(Variant(place!(Field::<Adt40>(Variant(RET, 0), 3)), 0), 0)) = _42.fld7 as u128;
_23.fld3 = core::ptr::addr_of_mut!(_47);
_24 = [_15,Field::<usize>(Variant(_41, 0), 4),Field::<usize>(Variant(_41, 0), 4),Field::<usize>(Variant(_41, 0), 4),Field::<usize>(Variant(_41, 0), 4),_15,_35];
_23.fld2 = core::ptr::addr_of!(place!(Field::<(i128,)>(Variant(_41, 0), 3)));
_30.fld4 = Adt42 { fld0: _23.fld1,fld1: _6.0,fld2: (*_56),fld3: _42.fld4.fld3,fld4: _42.fld4.fld4 };
_11 = [_15,_15,_35,_35,Field::<usize>(Variant(_41, 0), 4),_35,_15];
_16 = _48.fld1 as isize;
_20.1 = _48.fld1;
_18 = -_22;
place!(Field::<[u64; 8]>(Variant(RET, 0), 0)) = [_42.fld7,_30.fld7,_42.fld7,_30.fld7,_42.fld7,_30.fld7,_42.fld7,_42.fld7];
Goto(bb31)
}
bb31 = {
Call(_61 = dump_var(10_usize, 36_usize, Move(_36), 26_usize, Move(_26), 29_usize, Move(_29), 13_usize, Move(_13)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_61 = dump_var(10_usize, 40_usize, Move(_40), 47_usize, Move(_47), 28_usize, Move(_28), 5_usize, Move(_5)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_61 = dump_var(10_usize, 11_usize, Move(_11), 44_usize, Move(_44), 10_usize, Move(_10), 8_usize, Move(_8)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_61 = dump_var(10_usize, 12_usize, Move(_12), 19_usize, Move(_19), 27_usize, Move(_27), 35_usize, Move(_35)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_61 = dump_var(10_usize, 24_usize, Move(_24), 62_usize, _62, 62_usize, _62, 62_usize, _62), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: char,mut _2: char,mut _3: [usize; 7],mut _4: [usize; 8],mut _5: ([usize; 8],),mut _6: i128,mut _7: char,mut _8: i128,mut _9: [usize; 8]) -> [u64; 8] {
mir! {
type RET = [u64; 8];
let _10: usize;
let _11: f32;
let _12: *const *mut (u16,);
let _13: (*mut (u16,), u8, char);
let _14: isize;
let _15: char;
let _16: i8;
let _17: Adt47;
let _18: u32;
let _19: isize;
let _20: Adt52;
let _21: (i128,);
let _22: f32;
let _23: &'static bool;
let _24: [i64; 5];
let _25: isize;
let _26: isize;
let _27: isize;
let _28: char;
let _29: bool;
let _30: (u16,);
let _31: Adt43;
let _32: [i32; 3];
let _33: ();
let _34: ();
{
RET = [8669459271220597178_u64,440847412928152335_u64,15614177069877279086_u64,16411645681402032905_u64,14053229950264923246_u64,6088755900697917755_u64,10463213125605947177_u64,15972349928115072662_u64];
Goto(bb1)
}
bb1 = {
RET = [5687608618890450833_u64,194001005575037999_u64,1548439382913296678_u64,16715480995837442323_u64,8474022596603144434_u64,6172965638130710922_u64,17008400390481373567_u64,9406698009340193558_u64];
_2 = _1;
_8 = _6;
_5 = (_4,);
_4 = [17503349800242067526_usize,1_usize,2034170244520871247_usize,3_usize,10202637688387331266_usize,927572683289478332_usize,5_usize,3_usize];
_3 = [17781725982588984996_usize,0_usize,6614601896343961838_usize,2_usize,9097587622249052427_usize,5080728079964348210_usize,10013086087333429363_usize];
_10 = 9156160340818334477_usize;
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_10 = !2_usize;
_10 = 2775810905_u32 as usize;
_5.0 = _9;
_1 = _2;
_5 = (_9,);
_5 = (_9,);
_1 = _2;
_8 = -_6;
match _6 {
0 => bb2,
1 => bb3,
321189133709686885154271300201915369180 => bb5,
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
_8 = _7 as i128;
_11 = _10 as f32;
RET = [1854858348185273899_u64,17436549141507597331_u64,4299891807195721788_u64,8652813039264281167_u64,10879720996748330877_u64,16696585236796115426_u64,1831496589459975803_u64,9705199299498973803_u64];
RET = [13610610747106891567_u64,5881120186223033781_u64,3269606871557943866_u64,14349122369415729999_u64,11282126875858966028_u64,16006077300335249765_u64,7514487116861367135_u64,18181919262787439073_u64];
_4 = [_10,_10,_10,_10,_10,_10,_10,_10];
_1 = _7;
_3 = [_10,_10,_10,_10,_10,_10,_10];
_3 = [_10,_10,_10,_10,_10,_10,_10];
RET = [11231451877601495650_u64,9603905626820708399_u64,14220116270732926676_u64,1367385036251960541_u64,18317301389740678604_u64,16228342373435222194_u64,14897602021219233894_u64,11328604909535566661_u64];
_6 = _8;
_9 = _5.0;
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
Call(_7 = fn12(_3, _2, _1, _1, _6, _2, _10), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13.2 = _1;
_9 = [_10,_10,_10,_10,_10,_10,_10,_10];
_8 = _6 << _6;
_7 = _13.2;
_7 = _2;
_7 = _1;
_14 = 9223372036854775807_isize;
_14 = (-105_isize);
_5.0 = _4;
RET = [6446640227684726410_u64,2490258009616138377_u64,7666381713282079847_u64,5349966154397458972_u64,7841030538401734611_u64,10072549817440462680_u64,7744946675643014194_u64,7569806123798383366_u64];
_2 = _13.2;
Call(_2 = fn13(_1, _6, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_16 = 58_i8;
_16 = (-106_i8);
Goto(bb8)
}
bb8 = {
RET = [18024961495832194828_u64,4361380863569278820_u64,2559339563253188075_u64,6189827384735079026_u64,4726120484773848330_u64,6077431162307792554_u64,4779359440669477240_u64,13860610720884990209_u64];
_13.2 = _2;
_6 = _8 ^ _8;
_13.1 = !74_u8;
_15 = _2;
_11 = 203881034949059749925361887399633973976_u128 as f32;
_17 = Adt47::Variant1 { fld0: 50015939667142690802977536262782863405_u128 };
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_18 = (-18960_i16) as u32;
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
place!(Field::<u128>(Variant(_17, 1), 0)) = !24021306773295293062403712728128677130_u128;
_12 = core::ptr::addr_of!(_13.0);
_16 = 27_i8;
_7 = _15;
_15 = _13.2;
_12 = core::ptr::addr_of!((*_12));
SetDiscriminant(_17, 0);
Goto(bb9)
}
bb9 = {
_20.fld0 = _6 - _8;
_21 = (_6,);
_20.fld2 = !(-19037_i16);
_24 = [(-1799757231283592999_i64),6973925538243380712_i64,539165217850349875_i64,(-6129396471731690954_i64),(-5916027583660942677_i64)];
_26 = _11 as isize;
_18 = _20.fld2 as u32;
_5 = (_4,);
match _14 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211351 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_21.0 = _6;
Call(_16 = fn15(_20.fld0, _13.2, _14, _13.2, _24, _21, _6, _21.0, _2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_13.2 = _2;
_4 = _5.0;
_21.0 = _8;
match _14 {
0 => bb3,
340282366920938463463374607431768211351 => bb13,
_ => bb10
}
}
bb13 = {
_24 = [7412541218229157898_i64,(-3489569242655583756_i64),(-4010201677611311044_i64),(-6298690424347868950_i64),(-3222485653227960125_i64)];
_25 = _1 as isize;
RET = [15007093447355355058_u64,235236062308963081_u64,6523398538944496272_u64,11974101874964558436_u64,2205230379620213771_u64,7255405396027970986_u64,15228656672638756338_u64,10117081584649133944_u64];
match _14 {
0 => bb11,
1 => bb14,
2 => bb15,
3 => bb16,
340282366920938463463374607431768211351 => bb18,
_ => bb17
}
}
bb14 = {
RET = [18024961495832194828_u64,4361380863569278820_u64,2559339563253188075_u64,6189827384735079026_u64,4726120484773848330_u64,6077431162307792554_u64,4779359440669477240_u64,13860610720884990209_u64];
_13.2 = _2;
_6 = _8 ^ _8;
_13.1 = !74_u8;
_15 = _2;
_11 = 203881034949059749925361887399633973976_u128 as f32;
_17 = Adt47::Variant1 { fld0: 50015939667142690802977536262782863405_u128 };
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_18 = (-18960_i16) as u32;
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
place!(Field::<u128>(Variant(_17, 1), 0)) = !24021306773295293062403712728128677130_u128;
_12 = core::ptr::addr_of!(_13.0);
_16 = 27_i8;
_7 = _15;
_15 = _13.2;
_12 = core::ptr::addr_of!((*_12));
SetDiscriminant(_17, 0);
Goto(bb9)
}
bb15 = {
RET = [5687608618890450833_u64,194001005575037999_u64,1548439382913296678_u64,16715480995837442323_u64,8474022596603144434_u64,6172965638130710922_u64,17008400390481373567_u64,9406698009340193558_u64];
_2 = _1;
_8 = _6;
_5 = (_4,);
_4 = [17503349800242067526_usize,1_usize,2034170244520871247_usize,3_usize,10202637688387331266_usize,927572683289478332_usize,5_usize,3_usize];
_3 = [17781725982588984996_usize,0_usize,6614601896343961838_usize,2_usize,9097587622249052427_usize,5080728079964348210_usize,10013086087333429363_usize];
_10 = 9156160340818334477_usize;
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_10 = !2_usize;
_10 = 2775810905_u32 as usize;
_5.0 = _9;
_1 = _2;
_5 = (_9,);
_5 = (_9,);
_1 = _2;
_8 = -_6;
match _6 {
0 => bb2,
1 => bb3,
321189133709686885154271300201915369180 => bb5,
_ => bb4
}
}
bb16 = {
Return()
}
bb17 = {
_20.fld0 = _6 - _8;
_21 = (_6,);
_20.fld2 = !(-19037_i16);
_24 = [(-1799757231283592999_i64),6973925538243380712_i64,539165217850349875_i64,(-6129396471731690954_i64),(-5916027583660942677_i64)];
_26 = _11 as isize;
_18 = _20.fld2 as u32;
_5 = (_4,);
match _14 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211351 => bb11,
_ => bb10
}
}
bb18 = {
_27 = _25;
_19 = _25 >> _20.fld0;
_20 = Adt52 { fld0: _21.0,fld1: _2,fld2: 10336_i16 };
_16 = 934674615204669949_i64 as i8;
_27 = -_19;
_24 = [(-602134562616749560_i64),6383087510901171499_i64,(-6812050294875385412_i64),6979874064634875315_i64,7406243567900498311_i64];
_8 = _20.fld0 ^ _6;
RET = [15031755579408689422_u64,15045487054528281367_u64,14412884743049864840_u64,8601658621939428842_u64,10630264734626489025_u64,9750151699544262977_u64,14399921024648728584_u64,14971821513667556645_u64];
_21.0 = -_6;
_21.0 = 218429725157638653542331437764290534469_u128 as i128;
_2 = _1;
_13.0 = core::ptr::addr_of_mut!(_30);
_18 = 7995743258710786774_i64 as u32;
_20.fld0 = 9346059096535569502_u64 as i128;
_25 = _20.fld2 as isize;
_31.fld3 = [17733074416265592271_u64,8352387332480968200_u64,8389084546579956722_u64,9075872629588929909_u64,5406614995162028563_u64,13125016192007168977_u64,10078548755662626706_u64,14085949308616723873_u64];
_2 = _15;
_30.0 = (-7009080670053466532_i64) as u16;
_22 = _11;
Goto(bb19)
}
bb19 = {
Call(_33 = dump_var(11_usize, 16_usize, Move(_16), 30_usize, Move(_30), 5_usize, Move(_5), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_33 = dump_var(11_usize, 1_usize, Move(_1), 2_usize, Move(_2), 18_usize, Move(_18), 26_usize, Move(_26)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_33 = dump_var(11_usize, 9_usize, Move(_9), 24_usize, Move(_24), 34_usize, _34, 34_usize, _34), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [usize; 7],mut _2: char,mut _3: char,mut _4: char,mut _5: i128,mut _6: char,mut _7: usize) -> char {
mir! {
type RET = char;
let _8: isize;
let _9: isize;
let _10: (i128,);
let _11: f32;
let _12: [usize; 8];
let _13: Adt52;
let _14: f64;
let _15: char;
let _16: f64;
let _17: f64;
let _18: [i64; 5];
let _19: u8;
let _20: u8;
let _21: [i64; 5];
let _22: isize;
let _23: *mut (*mut (u16,), u8, char);
let _24: *mut [i64; 5];
let _25: bool;
let _26: [usize; 8];
let _27: char;
let _28: u16;
let _29: (i128,);
let _30: [i64; 5];
let _31: bool;
let _32: f32;
let _33: *const u32;
let _34: Adt52;
let _35: u128;
let _36: i8;
let _37: isize;
let _38: i16;
let _39: f64;
let _40: *const *mut (u16,);
let _41: i64;
let _42: *mut (*mut (u16,), u8, char);
let _43: f64;
let _44: *const u32;
let _45: Adt47;
let _46: u16;
let _47: ();
let _48: ();
{
_6 = _3;
_7 = 17535776819708395072_usize | 2_usize;
_2 = _4;
RET = _2;
_5 = !(-124790198387265555993007585447489420310_i128);
_5 = 140207307656171671602689332365191275828_i128 - (-91643489485292380555185089356340140023_i128);
_6 = RET;
_6 = RET;
_7 = !8124603198798532100_usize;
_5 = !(-159567759838337008786384867194264226736_i128);
_8 = (-15_isize);
RET = _3;
_8 = !9223372036854775807_isize;
_5 = 92734762759627357459237371058130158063_i128 ^ 166846872289300415851324428343117713963_i128;
_5 = 6679_i16 as i128;
_10 = (_5,);
_6 = _4;
_3 = _6;
_7 = 89_u8 as usize;
Call(_9 = core::intrinsics::bswap(_8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = 334073703020573096076345071978145481289_u128 as f32;
_7 = 13794_i16 as usize;
RET = _3;
_7 = 3734928093226252855_usize * 5_usize;
_10.0 = _5;
_11 = (-2385_i16) as f32;
_11 = 8996068701872936491_u64 as f32;
RET = _6;
_1 = [_7,_7,_7,_7,_7,_7,_7];
_10.0 = _5;
_6 = _4;
_9 = _8;
_10.0 = !_5;
_10 = (_5,);
_3 = _2;
Goto(bb2)
}
bb2 = {
_7 = 49387_u16 as usize;
_5 = 128_u8 as i128;
_4 = _6;
_11 = (-3894560724045441420_i64) as f32;
RET = _3;
RET = _4;
_4 = _2;
_8 = 6820970866605712192_u64 as isize;
_4 = _3;
_13.fld2 = 31317_i16;
_1 = [_7,_7,_7,_7,_7,_7,_7];
RET = _2;
_13.fld1 = _4;
_13.fld0 = _10.0;
_13.fld1 = RET;
_13.fld1 = _3;
_11 = (-53_i8) as f32;
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_9 = !_8;
_13.fld2 = 1941790201_i32 as i16;
_9 = _13.fld2 as isize;
_1 = [_7,_7,_7,_7,_7,_7,_7];
_9 = !_8;
_13 = Adt52 { fld0: _5,fld1: _3,fld2: (-8803_i16) };
match _13.fld2 {
0 => bb1,
340282366920938463463374607431768202653 => bb4,
_ => bb3
}
}
bb3 = {
_11 = 334073703020573096076345071978145481289_u128 as f32;
_7 = 13794_i16 as usize;
RET = _3;
_7 = 3734928093226252855_usize * 5_usize;
_10.0 = _5;
_11 = (-2385_i16) as f32;
_11 = 8996068701872936491_u64 as f32;
RET = _6;
_1 = [_7,_7,_7,_7,_7,_7,_7];
_10.0 = _5;
_6 = _4;
_9 = _8;
_10.0 = !_5;
_10 = (_5,);
_3 = _2;
Goto(bb2)
}
bb4 = {
_9 = _8 * _8;
_10.0 = _13.fld0;
_14 = _13.fld2 as f64;
_15 = RET;
_5 = -_10.0;
_10 = (_13.fld0,);
_10 = (_5,);
Goto(bb5)
}
bb5 = {
_16 = -_14;
_5 = -_10.0;
_15 = RET;
_16 = _14 * _14;
_13 = Adt52 { fld0: _5,fld1: _2,fld2: (-31095_i16) };
_16 = _14 * _14;
_2 = _13.fld1;
_10.0 = _13.fld0 << _13.fld0;
_13.fld2 = (-29678_i16) + (-2640_i16);
Call(_9 = core::intrinsics::bswap(_8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = (_13.fld0,);
_13.fld0 = !_5;
_2 = _15;
RET = _15;
_17 = _16 * _16;
_2 = _3;
_9 = _8 & _8;
_13 = Adt52 { fld0: _5,fld1: _15,fld2: (-25401_i16) };
_14 = _17;
_16 = _14;
_17 = (-57_i8) as f64;
_3 = _13.fld1;
RET = _3;
_10 = (_13.fld0,);
Goto(bb7)
}
bb7 = {
_1 = [_7,_7,_7,_7,_7,_7,_7];
_13 = Adt52 { fld0: _5,fld1: RET,fld2: 28301_i16 };
_7 = _11 as usize;
RET = _6;
_11 = 12376027225877388334_u64 as f32;
_1 = [_7,_7,_7,_7,_7,_7,_7];
_10 = (_5,);
_15 = _3;
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_9 = 335052006847831942822367804127708972034_u128 as isize;
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
RET = _3;
_13.fld1 = _6;
_10 = (_13.fld0,);
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_13 = Adt52 { fld0: _10.0,fld1: _15,fld2: (-31655_i16) };
Goto(bb8)
}
bb8 = {
_21 = [1813079234886957175_i64,2977111834210136255_i64,2690519910668160897_i64,7396737400516425539_i64,5137236126650674440_i64];
_24 = core::ptr::addr_of_mut!(_21);
_13.fld0 = _10.0 & _10.0;
_1 = [_7,_7,_7,_7,_7,_7,_7];
RET = _2;
_25 = !false;
_25 = true;
_6 = _3;
_5 = _13.fld2 as i128;
_14 = _16 * _16;
_17 = _14 + _14;
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_26 = [_7,_7,_7,_7,_7,_7,_7,_7];
_4 = _15;
_25 = _14 != _14;
_11 = _7 as f32;
_19 = 124_u8;
_22 = 20270_u16 as isize;
_9 = _8 | _22;
_16 = -_14;
_4 = _15;
_2 = RET;
RET = _2;
match _13.fld2 {
340282366920938463463374607431768179801 => bb9,
_ => bb5
}
}
bb9 = {
_7 = 9022155429939977050_u64 as usize;
_4 = _15;
_12 = _26;
_25 = true;
_5 = _4 as i128;
_18 = _21;
_13.fld1 = _6;
_16 = _11 as f64;
_5 = -_13.fld0;
_6 = _15;
_29 = _10;
_24 = core::ptr::addr_of_mut!((*_24));
_15 = _4;
_8 = _9 + _9;
_24 = core::ptr::addr_of_mut!(_18);
_30 = [1461878668127584507_i64,(-8060685070399978938_i64),4971567120621478567_i64,(-8191098030037050947_i64),(-6719767351936561942_i64)];
RET = _13.fld1;
_32 = _11;
_6 = _2;
Goto(bb10)
}
bb10 = {
_11 = _32 - _32;
_20 = _19 % _19;
_17 = -_14;
_29.0 = _5 | _5;
_13.fld2 = (-7165_i16);
_31 = !_25;
_20 = !_19;
_16 = _17 - _14;
_1 = [_7,_7,_7,_7,_7,_7,_7];
_24 = core::ptr::addr_of_mut!(_30);
_4 = _13.fld1;
_19 = _20 >> _5;
RET = _4;
_32 = -_11;
_34 = Adt52 { fld0: _10.0,fld1: _3,fld2: _13.fld2 };
_32 = _11;
_22 = 1242683908_i32 as isize;
_29 = _10;
_9 = -_22;
_3 = _13.fld1;
_5 = _34.fld0 | _13.fld0;
_13.fld0 = 18320541336935972592_u64 as i128;
_16 = _14;
_27 = _3;
RET = _27;
_5 = 8420030622130275495_i64 as i128;
_3 = _2;
_27 = _34.fld1;
Goto(bb11)
}
bb11 = {
RET = _6;
_13 = _34;
_29.0 = !_10.0;
_5 = -_29.0;
_9 = !_8;
_9 = _8;
_30 = [2834516979562625762_i64,2532906087428289975_i64,7529481388206950306_i64,3159307834158412208_i64,3673482573121327443_i64];
_35 = 265687915307153080681232948311626665345_u128 * 13905793521909547509908796881956719972_u128;
_34.fld1 = RET;
_36 = (-30_i8);
_37 = _8;
_39 = _14;
_3 = _4;
Goto(bb12)
}
bb12 = {
_38 = -_13.fld2;
RET = _3;
_13.fld1 = RET;
_24 = core::ptr::addr_of_mut!((*_24));
_39 = -_16;
_15 = _2;
_34 = _13;
_22 = _36 as isize;
_18 = [(-936758722755710394_i64),(-2115004865982294255_i64),4344433433960494568_i64,7930807990156405585_i64,3315606210865941317_i64];
_11 = _32 * _32;
_26 = [_7,_7,_7,_7,_7,_7,_7,_7];
_5 = -_29.0;
_31 = _25;
_31 = _15 < _15;
_21 = [(-7814409729565199712_i64),7954289052284191079_i64,4700444762074048279_i64,(-9116865253602926797_i64),(-5217296484103985271_i64)];
_6 = _15;
_10 = _29;
_13 = _34;
_32 = _11;
_28 = 2730148235_u32 as u16;
_25 = _5 <= _5;
_32 = _11 + _11;
_36 = -(-98_i8);
_34.fld0 = _13.fld0 & _5;
RET = _13.fld1;
_10 = (_29.0,);
_7 = 13225331043510123370_usize;
_1 = [_7,_7,_7,_7,_7,_7,_7];
_10 = (_29.0,);
Goto(bb13)
}
bb13 = {
_26 = _12;
RET = _2;
_29.0 = _34.fld0 << _8;
_14 = _28 as f64;
_38 = _34.fld2;
RET = _13.fld1;
Goto(bb14)
}
bb14 = {
_13.fld1 = _2;
_3 = _34.fld1;
_29 = (_13.fld0,);
_21 = [(-5903728910907894563_i64),(-5769765878365416089_i64),6313111822561637265_i64,6616704358960563178_i64,(-7696210878785290380_i64)];
_36 = _28 as i8;
_25 = !_31;
_31 = _14 < _17;
_21 = _18;
_10.0 = -_13.fld0;
_16 = _17;
_34.fld0 = _13.fld0 ^ _5;
_5 = !_34.fld0;
_4 = RET;
_16 = 1223383172_i32 as f64;
_34 = _13;
_43 = _39;
_11 = (-265407394_i32) as f32;
_12 = [_7,_7,_7,_7,_7,_7,_7,_7];
_19 = _20;
_20 = _38 as u8;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(12_usize, 19_usize, Move(_19), 7_usize, Move(_7), 38_usize, Move(_38), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(12_usize, 1_usize, Move(_1), 20_usize, Move(_20), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(12_usize, 9_usize, Move(_9), 10_usize, Move(_10), 35_usize, Move(_35), 31_usize, Move(_31)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(12_usize, 22_usize, Move(_22), 29_usize, Move(_29), 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: char,mut _2: i128,mut _3: i128) -> char {
mir! {
type RET = char;
let _4: i8;
let _5: ((u16,), isize, (&'static bool, char), [usize; 8], usize);
let _6: f32;
let _7: isize;
let _8: [i64; 5];
let _9: *const (u16,);
let _10: [i32; 3];
let _11: f32;
let _12: isize;
let _13: *const i64;
let _14: isize;
let _15: [usize; 8];
let _16: Adt53;
let _17: i16;
let _18: *const i64;
let _19: isize;
let _20: u64;
let _21: i16;
let _22: f32;
let _23: u64;
let _24: f64;
let _25: Adt40;
let _26: (*mut (u16,), u8, char);
let _27: char;
let _28: ();
let _29: ();
{
RET = _1;
_3 = _2 | _2;
RET = _1;
_1 = RET;
RET = _1;
RET = _1;
_2 = _3 >> _3;
_1 = RET;
_3 = -_2;
_3 = _2;
_2 = -_3;
_3 = !_2;
_4 = (-107_i8);
RET = _1;
_5.1 = !44_isize;
_5.2.1 = RET;
_5.0 = (1656_u16,);
_5.4 = 2_usize + 0_usize;
_5.2.1 = _1;
Goto(bb1)
}
bb1 = {
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
RET = _5.2.1;
_2 = -_3;
RET = _5.2.1;
_5.0.0 = 21567_u16 << _5.4;
_2 = _3 & _3;
_10 = [1865843005_i32,(-2128135920_i32),(-1597046573_i32)];
_5.0.0 = 22709_u16 << _3;
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_10 = [(-1434005626_i32),(-999128332_i32),1959411037_i32];
_1 = _5.2.1;
_6 = 54_u8 as f32;
_11 = _3 as f32;
Call(_7 = fn14(_3, _5.0.0, _1, _5.0.0, _3, RET, _2, _5.0.0, _11, _5.4, _2, _3, _5.0.0, _5.0, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = [(-551993767_i32),37340301_i32,959647557_i32];
_5.0.0 = 47830_u16 & 20917_u16;
_5.2.1 = _1;
_5.2.1 = RET;
_8 = [(-9064638765562575569_i64),5296473960835134604_i64,8566373906431194387_i64,3773254304812315989_i64,2109640994044516755_i64];
_9 = core::ptr::addr_of!(_5.0);
_4 = 115_u8 as i8;
_12 = !_7;
_7 = 1660516442_i32 as isize;
Goto(bb3)
}
bb3 = {
_8 = [1670501634658702697_i64,8232847977348336583_i64,7587189514645046348_i64,(-6644866410117645581_i64),(-8428472257043685833_i64)];
_12 = !_7;
_8 = [4557851873643121050_i64,(-8154648081953079959_i64),(-8550186661007698348_i64),(-5590905340866650064_i64),5189633283001339124_i64];
_5.1 = (-15339_i16) as isize;
_5.0.0 = 42557_u16 << _2;
_3 = _2 & _2;
_14 = _5.4 as isize;
Goto(bb4)
}
bb4 = {
_1 = RET;
_5.2.1 = _1;
_3 = _2 + _2;
_6 = 16943949019060555426_u64 as f32;
_15 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_1 = RET;
_15 = _5.3;
_2 = 968150734_u32 as i128;
_3 = 78_u8 as i128;
RET = _1;
_11 = _4 as f32;
_6 = -_11;
_6 = -_11;
_2 = _5.2.1 as i128;
Goto(bb5)
}
bb5 = {
_5.1 = _12;
_11 = _4 as f32;
_7 = _12 | _14;
_12 = (*_9).0 as isize;
_2 = _3 + _3;
_3 = _2 + _2;
RET = _1;
_10 = [1862850723_i32,557486591_i32,(-1342736185_i32)];
_5.4 = 282143221243594072341276281329584734013_u128 as usize;
_7 = 1455646017_i32 as isize;
_5.4 = !13234896237467780998_usize;
_7 = 1029835867_i32 as isize;
_9 = core::ptr::addr_of!(_5.0);
RET = _5.2.1;
_4 = _1 as i8;
_1 = RET;
Goto(bb6)
}
bb6 = {
_5.1 = _3 as isize;
_1 = RET;
_2 = !_3;
_7 = _12 * _5.1;
_9 = core::ptr::addr_of!(_5.0);
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_5.2.1 = _1;
RET = _5.2.1;
_4 = 112_i8;
_14 = _5.2.1 as isize;
_2 = _3;
_2 = !_3;
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_6 = -_11;
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_19 = _12 - _14;
_8 = [5562304291436401444_i64,(-5889741340485029996_i64),1617876846404795989_i64,(-1127327317911473673_i64),(-8258862873741565802_i64)];
_17 = _11 as i16;
_2 = _3;
_9 = core::ptr::addr_of!((*_9));
_5.2.1 = _1;
_1 = RET;
_7 = !_19;
_17 = !20260_i16;
_5.4 = !1_usize;
_2 = !_3;
_3 = _2;
Goto(bb7)
}
bb7 = {
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_7 = _4 as isize;
_2 = (*_9).0 as i128;
_6 = _11;
_5.3 = _15;
_23 = 451104444436456031_u64 + 1572357374740889288_u64;
_5.0.0 = 61975_u16;
_26.1 = false as u8;
_12 = -_19;
_24 = _12 as f64;
_27 = RET;
match (*_9).0 {
0 => bb1,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
61975 => bb15,
_ => bb14
}
}
bb8 = {
_5.1 = _3 as isize;
_1 = RET;
_2 = !_3;
_7 = _12 * _5.1;
_9 = core::ptr::addr_of!(_5.0);
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_5.2.1 = _1;
RET = _5.2.1;
_4 = 112_i8;
_14 = _5.2.1 as isize;
_2 = _3;
_2 = !_3;
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_6 = -_11;
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_19 = _12 - _14;
_8 = [5562304291436401444_i64,(-5889741340485029996_i64),1617876846404795989_i64,(-1127327317911473673_i64),(-8258862873741565802_i64)];
_17 = _11 as i16;
_2 = _3;
_9 = core::ptr::addr_of!((*_9));
_5.2.1 = _1;
_1 = RET;
_7 = !_19;
_17 = !20260_i16;
_5.4 = !1_usize;
_2 = !_3;
_3 = _2;
Goto(bb7)
}
bb9 = {
_5.1 = _12;
_11 = _4 as f32;
_7 = _12 | _14;
_12 = (*_9).0 as isize;
_2 = _3 + _3;
_3 = _2 + _2;
RET = _1;
_10 = [1862850723_i32,557486591_i32,(-1342736185_i32)];
_5.4 = 282143221243594072341276281329584734013_u128 as usize;
_7 = 1455646017_i32 as isize;
_5.4 = !13234896237467780998_usize;
_7 = 1029835867_i32 as isize;
_9 = core::ptr::addr_of!(_5.0);
RET = _5.2.1;
_4 = _1 as i8;
_1 = RET;
Goto(bb6)
}
bb10 = {
_1 = RET;
_5.2.1 = _1;
_3 = _2 + _2;
_6 = 16943949019060555426_u64 as f32;
_15 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_1 = RET;
_15 = _5.3;
_2 = 968150734_u32 as i128;
_3 = 78_u8 as i128;
RET = _1;
_11 = _4 as f32;
_6 = -_11;
_6 = -_11;
_2 = _5.2.1 as i128;
Goto(bb5)
}
bb11 = {
_8 = [1670501634658702697_i64,8232847977348336583_i64,7587189514645046348_i64,(-6644866410117645581_i64),(-8428472257043685833_i64)];
_12 = !_7;
_8 = [4557851873643121050_i64,(-8154648081953079959_i64),(-8550186661007698348_i64),(-5590905340866650064_i64),5189633283001339124_i64];
_5.1 = (-15339_i16) as isize;
_5.0.0 = 42557_u16 << _2;
_3 = _2 & _2;
_14 = _5.4 as isize;
Goto(bb4)
}
bb12 = {
_10 = [(-551993767_i32),37340301_i32,959647557_i32];
_5.0.0 = 47830_u16 & 20917_u16;
_5.2.1 = _1;
_5.2.1 = RET;
_8 = [(-9064638765562575569_i64),5296473960835134604_i64,8566373906431194387_i64,3773254304812315989_i64,2109640994044516755_i64];
_9 = core::ptr::addr_of!(_5.0);
_4 = 115_u8 as i8;
_12 = !_7;
_7 = 1660516442_i32 as isize;
Goto(bb3)
}
bb13 = {
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
RET = _5.2.1;
_2 = -_3;
RET = _5.2.1;
_5.0.0 = 21567_u16 << _5.4;
_2 = _3 & _3;
_10 = [1865843005_i32,(-2128135920_i32),(-1597046573_i32)];
_5.0.0 = 22709_u16 << _3;
_5.3 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
_10 = [(-1434005626_i32),(-999128332_i32),1959411037_i32];
_1 = _5.2.1;
_6 = 54_u8 as f32;
_11 = _3 as f32;
Call(_7 = fn14(_3, _5.0.0, _1, _5.0.0, _3, RET, _2, _5.0.0, _11, _5.4, _2, _3, _5.0.0, _5.0, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_19 = _5.4 as isize;
_5.2.1 = _27;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(13_usize, 8_usize, Move(_8), 10_usize, Move(_10), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(13_usize, 19_usize, Move(_19), 4_usize, Move(_4), 1_usize, Move(_1), 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i128,mut _2: u16,mut _3: char,mut _4: u16,mut _5: i128,mut _6: char,mut _7: i128,mut _8: u16,mut _9: f32,mut _10: usize,mut _11: i128,mut _12: i128,mut _13: u16,mut _14: (u16,),mut _15: char) -> isize {
mir! {
type RET = isize;
let _16: *mut [i64; 5];
let _17: ([usize; 8],);
let _18: (&'static bool, char);
let _19: (u16,);
let _20: *const bool;
let _21: (i128,);
let _22: Adt52;
let _23: Adt55;
let _24: f64;
let _25: u8;
let _26: Adt55;
let _27: [usize; 7];
let _28: (u16,);
let _29: Adt40;
let _30: f32;
let _31: *const *const *mut (u16,);
let _32: f64;
let _33: ();
let _34: ();
{
_7 = _12 >> _1;
_12 = _5;
RET = 3007992301_u32 as isize;
RET = (-977282615_i32) as isize;
_3 = _15;
_7 = _11;
_15 = _6;
_13 = 12646042415293632053_u64 as u16;
_14 = (_2,);
_2 = _4;
_13 = !_8;
_13 = _14.0;
_6 = _15;
_14.0 = _2 >> _4;
_8 = !_4;
_14.0 = _13;
RET = 2768733181673376279_i64 as isize;
_4 = _8;
_11 = (-9187_i16) as i128;
_7 = _12;
_6 = _3;
_19 = (_4,);
_22 = Adt52 { fld0: _12,fld1: _3,fld2: (-2288_i16) };
_5 = _1 - _12;
Goto(bb1)
}
bb1 = {
_12 = (-1014258371834633464_i64) as i128;
_14 = (_8,);
_26.fld2 = core::ptr::addr_of!(_21);
_26.fld1 = _6 as u32;
_23.fld3.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_22.fld2 = 3050_i16 + (-6584_i16);
RET = (-9223372036854775808_isize);
_27 = [_10,_10,_10,_10,_10,_10,_10];
_23.fld3.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_23.fld1 = _26.fld1 - _26.fld1;
_17.0 = _23.fld3.0;
_22 = Adt52 { fld0: _1,fld1: _3,fld2: (-12931_i16) };
RET = _8 as isize;
_23.fld1 = _26.fld1 >> _5;
_6 = _3;
Goto(bb2)
}
bb2 = {
Call(_33 = dump_var(14_usize, 7_usize, Move(_7), 2_usize, Move(_2), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_33 = dump_var(14_usize, 10_usize, Move(_10), 8_usize, Move(_8), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i128,mut _2: char,mut _3: isize,mut _4: char,mut _5: [i64; 5],mut _6: (i128,),mut _7: i128,mut _8: i128,mut _9: char) -> i8 {
mir! {
type RET = i8;
let _10: *const *mut (u16,);
let _11: ([usize; 8],);
let _12: f32;
let _13: [u64; 8];
let _14: ([usize; 8],);
let _15: bool;
let _16: bool;
let _17: (i128,);
let _18: u32;
let _19: Adt54;
let _20: [usize; 8];
let _21: bool;
let _22: Adt46;
let _23: isize;
let _24: i32;
let _25: char;
let _26: [usize; 7];
let _27: bool;
let _28: Adt48;
let _29: [u32; 5];
let _30: [u32; 5];
let _31: i32;
let _32: isize;
let _33: [usize; 7];
let _34: i8;
let _35: *const (u16,);
let _36: [u32; 5];
let _37: ();
let _38: ();
{
_9 = _2;
_2 = _4;
_1 = (-20_i8) as i128;
_5 = [356070073545297042_i64,6395611108893740529_i64,3131640593292000421_i64,(-4635354632737373637_i64),5512143889586166655_i64];
_9 = _2;
RET = !(-93_i8);
_9 = _2;
_6 = (_7,);
_4 = _9;
_4 = _9;
_4 = _2;
_6.0 = _8 ^ _8;
_5 = [(-1820363151924970021_i64),(-7363205373346983472_i64),(-5234329682729180644_i64),(-6459507264435453143_i64),5007715887538908557_i64];
_9 = _2;
_4 = _2;
_4 = _9;
_8 = _6.0;
_1 = !_8;
Call(_1 = core::intrinsics::transmute(_7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = 3116_u16 as f32;
_11.0 = [3_usize,15135291951054904748_usize,4_usize,0_usize,2_usize,5622008463105948499_usize,16483966852665234620_usize,16020561980127391496_usize];
RET = _3 as i8;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463463374607431768211351 => bb6,
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
RET = 36_i8;
_5 = [5963426927013168374_i64,967458885712029224_i64,3085971071648031579_i64,7903217907412531512_i64,(-4224498697703276314_i64)];
_2 = _4;
_11.0 = [6122364417223890896_usize,1_usize,3_usize,8589281188712957751_usize,10724527994708237738_usize,7_usize,5_usize,4_usize];
_12 = 2065561404_i32 as f32;
_2 = _4;
_9 = _2;
_14.0 = _11.0;
RET = 1514910278455079347_i64 as i8;
_16 = true ^ false;
_8 = !_1;
_14.0 = _11.0;
_8 = 20180_u16 as i128;
RET = 7074238548450946065_i64 as i8;
_4 = _2;
_17 = (_7,);
_19.fld3.fld2 = _2 as i16;
_6 = (_1,);
_19.fld3.fld0 = !_1;
_17 = _6;
_8 = -_1;
match _3 {
0 => bb4,
1 => bb5,
340282366920938463463374607431768211351 => bb7,
_ => bb3
}
}
bb7 = {
_8 = _17.0;
_1 = 0_usize as i128;
_19.fld1 = _14;
RET = 8_i8 + (-111_i8);
_11.0 = [5_usize,1_usize,12960787245968552231_usize,418081736838154486_usize,2_usize,7843121121477826378_usize,6_usize,4104141555330567952_usize];
_17.0 = (-871571968_i32) as i128;
RET = (-115_i8);
_3 = !9223372036854775807_isize;
match RET {
340282366920938463463374607431768211341 => bb8,
_ => bb4
}
}
bb8 = {
_19.fld2 = core::ptr::addr_of!(_18);
_18 = _4 as u32;
_26 = [6920023334849335221_usize,0_usize,3_usize,3267904084000774556_usize,9338716474317419211_usize,2_usize,0_usize];
_17.0 = _19.fld3.fld0;
_19.fld3 = Adt52 { fld0: _6.0,fld1: _9,fld2: (-21398_i16) };
_24 = -1924424409_i32;
RET = 39_i8;
_27 = _16;
_15 = _7 <= _6.0;
_25 = _4;
_23 = _3 << _6.0;
_18 = 2393846424_u32 ^ 3023128014_u32;
_13 = [5976545618969645821_u64,10453560721758742457_u64,9329827292624225913_u64,1269097165447889823_u64,3972138712970638523_u64,15274938492169007321_u64,6637293116920825860_u64,17757420592748971266_u64];
_21 = _23 <= _23;
_7 = _6.0;
_14.0 = _19.fld1.0;
_19.fld0 = _5;
_17 = (_8,);
_30 = [_18,_18,_18,_18,_18];
_29 = [_18,_18,_18,_18,_18];
_8 = -_7;
_14.0 = [7671460742314183727_usize,11205939031833841356_usize,17740762758584914978_usize,4_usize,4569599849626772885_usize,14679527293683437930_usize,2_usize,1_usize];
Call(_32 = fn16(_21, Move(_19), _26, _25, _26, _7, _14.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_26 = [4804233422741703227_usize,8371862759651385453_usize,208387462777938852_usize,4831833253379936819_usize,12487938683002883546_usize,2_usize,5_usize];
_21 = !_15;
_26 = [5_usize,12485594103467207418_usize,0_usize,6875227701453450133_usize,1_usize,156257168898802607_usize,15606722449641845351_usize];
_19.fld1.0 = [13001431900281504808_usize,2698233484662262930_usize,4_usize,5_usize,6_usize,13678304824001120649_usize,6_usize,15762820696867446433_usize];
_17 = _6;
_19.fld0 = [5483512736549406226_i64,(-7971518949542716866_i64),(-8338630294514901301_i64),(-6611405270280388107_i64),(-2395431565628136397_i64)];
_31 = !_24;
_13 = [3100973691657082374_u64,7533929453208168736_u64,8469656833446881575_u64,10917856922394245967_u64,15413335037518831756_u64,10450612301032744473_u64,17402686808956350877_u64,8864260432817781277_u64];
_19.fld3.fld1 = _9;
_17.0 = _7;
_26 = [0_usize,7063710234283266906_usize,7070434766302976247_usize,7617286713329203966_usize,2_usize,12454971742509053386_usize,5443093099654578596_usize];
RET = 29_i8 ^ (-21_i8);
_29 = [_18,_18,_18,_18,_18];
_15 = _32 <= _32;
_20 = _11.0;
_33 = [9141274002517858144_usize,6_usize,3_usize,7_usize,9406224451488786190_usize,783334478694043769_usize,3445747902279526818_usize];
_19.fld3 = Adt52 { fld0: _17.0,fld1: _2,fld2: (-13677_i16) };
_6 = _17;
_2 = _4;
RET = _19.fld3.fld2 as i8;
Goto(bb10)
}
bb10 = {
Call(_37 = dump_var(15_usize, 27_usize, Move(_27), 14_usize, Move(_14), 30_usize, Move(_30), 13_usize, Move(_13)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_37 = dump_var(15_usize, 21_usize, Move(_21), 25_usize, Move(_25), 9_usize, Move(_9), 23_usize, Move(_23)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_37 = dump_var(15_usize, 4_usize, Move(_4), 20_usize, Move(_20), 18_usize, Move(_18), 11_usize, Move(_11)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_37 = dump_var(15_usize, 15_usize, Move(_15), 8_usize, Move(_8), 38_usize, _38, 38_usize, _38), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: Adt54,mut _3: [usize; 7],mut _4: char,mut _5: [usize; 7],mut _6: i128,mut _7: [usize; 8]) -> isize {
mir! {
type RET = isize;
let _8: [usize; 8];
let _9: Adt52;
let _10: ([usize; 8],);
let _11: (*mut (u16,), u8, char);
let _12: u64;
let _13: [u64; 8];
let _14: [usize; 8];
let _15: Adt52;
let _16: (u16,);
let _17: Adt46;
let _18: f64;
let _19: Adt52;
let _20: isize;
let _21: [i64; 5];
let _22: (u16,);
let _23: *mut (u16,);
let _24: Adt47;
let _25: (i128,);
let _26: Adt41;
let _27: u128;
let _28: ();
let _29: ();
{
RET = 15587916689774250473_u64 as isize;
_2.fld1.0 = [15238790040158124477_usize,7_usize,1_usize,117887784023902055_usize,7_usize,5_usize,6_usize,17204764425813269695_usize];
_4 = _2.fld3.fld1;
_9.fld0 = _6;
_2.fld3.fld1 = _4;
_10.0 = [16637939938653444032_usize,4_usize,0_usize,3_usize,10244587277078824035_usize,7_usize,15357735081549556738_usize,3_usize];
_3 = [11382719913078879538_usize,3429267802041429600_usize,10584650987394548362_usize,1_usize,451999326584002568_usize,7_usize,1_usize];
RET = !9223372036854775807_isize;
_2.fld3 = Adt52 { fld0: _6,fld1: _4,fld2: (-25330_i16) };
_12 = 1_usize as u64;
_8 = _2.fld1.0;
_9.fld1 = _4;
_15.fld0 = _9.fld0;
_2.fld3.fld1 = _4;
_5 = [5_usize,14345746377586989260_usize,0_usize,0_usize,4713350988842139632_usize,2856603556068212965_usize,6884131931338777794_usize];
_4 = _9.fld1;
_2.fld3.fld2 = !20192_i16;
_6 = 55023_u16 as i128;
_15.fld0 = _1 as i128;
_15.fld2 = _2.fld3.fld2 - _2.fld3.fld2;
RET = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_8 = _7;
Goto(bb1)
}
bb1 = {
_16 = (33201_u16,);
Goto(bb2)
}
bb2 = {
_11.1 = 117_u8;
_11.2 = _4;
Goto(bb3)
}
bb3 = {
_10 = _2.fld1;
_2.fld1 = (_7,);
_2.fld3.fld2 = _15.fld2 ^ _15.fld2;
_9.fld0 = 10330497595805620437_usize as i128;
RET = -9223372036854775807_isize;
_9 = Adt52 { fld0: _15.fld0,fld1: _2.fld3.fld1,fld2: _2.fld3.fld2 };
_16.0 = 39724_u16 & 64350_u16;
_3 = _5;
_15.fld1 = _4;
_4 = _2.fld3.fld1;
_11.0 = core::ptr::addr_of_mut!(_16);
_19 = _9;
_9.fld2 = 1244673436_i32 as i16;
Call(RET = fn17(_11, Move(_2), _9.fld0, _8, _9, _9, _19, _9, _15, _19, _9.fld0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9 = _15;
_15.fld0 = 200538631219486069327044427184316174034_u128 as i128;
_23 = _11.0;
_1 = !false;
_13 = [_12,_12,_12,_12,_12,_12,_12,_12];
_10.0 = _7;
_13 = [_12,_12,_12,_12,_12,_12,_12,_12];
_11.2 = _15.fld1;
_1 = RET == RET;
_25 = (_19.fld0,);
_27 = 337243150348578862139267497593935092156_u128;
_16 = (14482_u16,);
_2.fld3.fld2 = _9.fld2 & _9.fld2;
_2.fld1.0 = _7;
_19.fld1 = _4;
_4 = _15.fld1;
_12 = !1087893484082441084_u64;
_11.0 = core::ptr::addr_of_mut!(_22);
_2.fld3 = _19;
_25 = (_19.fld0,);
_20 = -RET;
_17 = Adt46::Variant2 { fld0: _11 };
Goto(bb5)
}
bb5 = {
Call(_28 = dump_var(16_usize, 12_usize, Move(_12), 25_usize, Move(_25), 7_usize, Move(_7), 20_usize, Move(_20)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_28 = dump_var(16_usize, 16_usize, Move(_16), 13_usize, Move(_13), 3_usize, Move(_3), 29_usize, _29), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: (*mut (u16,), u8, char),mut _2: Adt54,mut _3: i128,mut _4: [usize; 8],mut _5: Adt52,mut _6: Adt52,mut _7: Adt52,mut _8: Adt52,mut _9: Adt52,mut _10: Adt52,mut _11: i128) -> isize {
mir! {
type RET = isize;
let _12: isize;
let _13: [usize; 7];
let _14: *const *const *mut (u16,);
let _15: [usize; 8];
let _16: f64;
let _17: (u16,);
let _18: Adt41;
let _19: Adt50;
let _20: u128;
let _21: isize;
let _22: Adt43;
let _23: *mut [i64; 5];
let _24: f32;
let _25: isize;
let _26: *mut [i64; 5];
let _27: *const bool;
let _28: u8;
let _29: [u64; 8];
let _30: i16;
let _31: Adt55;
let _32: usize;
let _33: (&'static bool, char);
let _34: isize;
let _35: Adt44;
let _36: ();
let _37: ();
{
_9 = Adt52 { fld0: _5.fld0,fld1: _6.fld1,fld2: _6.fld2 };
_5 = Adt52 { fld0: _11,fld1: _2.fld3.fld1,fld2: _7.fld2 };
_10.fld1 = _7.fld1;
_6.fld1 = _8.fld1;
_1.1 = !55_u8;
_7.fld1 = _9.fld1;
_13 = [5_usize,17119128598727052256_usize,4_usize,10812842639908231558_usize,500680291020033858_usize,8977607208459539424_usize,8720211107024526695_usize];
_2.fld3.fld2 = _6.fld2 & _10.fld2;
_7 = Adt52 { fld0: _8.fld0,fld1: _5.fld1,fld2: _5.fld2 };
RET = 593591046644262923_u64 as isize;
_2.fld3.fld2 = true as i16;
_9.fld1 = _5.fld1;
_5 = Adt52 { fld0: _9.fld0,fld1: _1.2,fld2: _2.fld3.fld2 };
_9.fld1 = _8.fld1;
_4 = [12606444086495862332_usize,2008651916933573665_usize,5_usize,4499650239341719962_usize,5_usize,2_usize,0_usize,3_usize];
Goto(bb1)
}
bb1 = {
_1.2 = _2.fld3.fld1;
_5 = _7;
_2.fld3.fld1 = _1.2;
_7.fld0 = _8.fld1 as i128;
_8.fld0 = 59033_u16 as i128;
_8.fld1 = _6.fld1;
_6.fld1 = _10.fld1;
_2.fld0 = [(-2412928885777623338_i64),6692383154035578309_i64,(-7596873925485520556_i64),(-1179533590912090128_i64),(-3003607966911166102_i64)];
_9.fld0 = !_5.fld0;
_10.fld2 = 96_i8 as i16;
_7.fld2 = _6.fld2;
_10.fld1 = _5.fld1;
_8.fld0 = _11;
_10.fld0 = _5.fld0;
_6.fld0 = !_10.fld0;
_2.fld3.fld1 = _6.fld1;
_7.fld2 = _9.fld2 * _10.fld2;
_1.2 = _2.fld3.fld1;
_6.fld2 = _7.fld2 ^ _2.fld3.fld2;
_2.fld3.fld0 = _6.fld2 as i128;
_4 = [12480331007867880592_usize,16695686752879177435_usize,12969487583812325751_usize,9011685085843738511_usize,2_usize,3_usize,12610357000520006466_usize,6_usize];
_9.fld2 = -_2.fld3.fld2;
_10.fld0 = _1.1 as i128;
_4 = _2.fld1.0;
Goto(bb2)
}
bb2 = {
_1.2 = _6.fld1;
Goto(bb3)
}
bb3 = {
_2.fld3.fld1 = _8.fld1;
_7.fld2 = _6.fld2;
_1.1 = !149_u8;
_3 = -_8.fld0;
_2.fld3.fld2 = _8.fld2;
_10 = Adt52 { fld0: _9.fld0,fld1: _8.fld1,fld2: _8.fld2 };
_1.1 = 1639_u16 as u8;
_7.fld2 = _6.fld2;
_2.fld0 = [(-2886994029641871227_i64),(-6634331080403503529_i64),3948404357927651217_i64,(-4548677389727960443_i64),6090401756751570024_i64];
_11 = 1687666348_u32 as i128;
_2.fld3.fld1 = _10.fld1;
_2.fld1.0 = [13598110447377734420_usize,7_usize,2_usize,4_usize,7_usize,0_usize,3_usize,2_usize];
_10 = Adt52 { fld0: _3,fld1: _5.fld1,fld2: _8.fld2 };
_6.fld2 = 1553483_u32 as i16;
_15 = [6_usize,12525875935456386501_usize,1_usize,14648766176438386996_usize,1962462695838131019_usize,3_usize,4314430676255821849_usize,2_usize];
_5 = Adt52 { fld0: _10.fld0,fld1: _10.fld1,fld2: _2.fld3.fld2 };
_8 = _2.fld3;
_7.fld1 = _10.fld1;
_8.fld0 = _9.fld0 << _10.fld0;
Goto(bb4)
}
bb4 = {
_2.fld3 = Adt52 { fld0: _5.fld0,fld1: _7.fld1,fld2: _6.fld2 };
Call(_3 = fn18(_5, _8, _6, _5, Move(_2), _9.fld0, _5.fld0, _9.fld1, _5.fld0, _10, _9, _9.fld0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_1.0 = core::ptr::addr_of_mut!(_17);
_2.fld3.fld2 = _7.fld2 >> _6.fld0;
_2.fld1 = (_15,);
_20 = !23023660170958551707639527832589702289_u128;
_6.fld0 = 3341407349_u32 as i128;
_9.fld0 = -_8.fld0;
_6.fld1 = _7.fld1;
_13 = [16307122742840418361_usize,0_usize,1_usize,16415665643664607406_usize,16367480251698517673_usize,0_usize,2_usize];
_22.fld4.fld2 = core::ptr::addr_of!(_1.0);
_20 = 92149403203347435393831339462168464536_u128;
match _20 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
92149403203347435393831339462168464536 => bb10,
_ => bb9
}
}
bb6 = {
_2.fld3 = Adt52 { fld0: _5.fld0,fld1: _7.fld1,fld2: _6.fld2 };
Call(_3 = fn18(_5, _8, _6, _5, Move(_2), _9.fld0, _5.fld0, _9.fld1, _5.fld0, _10, _9, _9.fld0), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_2.fld3.fld1 = _8.fld1;
_7.fld2 = _6.fld2;
_1.1 = !149_u8;
_3 = -_8.fld0;
_2.fld3.fld2 = _8.fld2;
_10 = Adt52 { fld0: _9.fld0,fld1: _8.fld1,fld2: _8.fld2 };
_1.1 = 1639_u16 as u8;
_7.fld2 = _6.fld2;
_2.fld0 = [(-2886994029641871227_i64),(-6634331080403503529_i64),3948404357927651217_i64,(-4548677389727960443_i64),6090401756751570024_i64];
_11 = 1687666348_u32 as i128;
_2.fld3.fld1 = _10.fld1;
_2.fld1.0 = [13598110447377734420_usize,7_usize,2_usize,4_usize,7_usize,0_usize,3_usize,2_usize];
_10 = Adt52 { fld0: _3,fld1: _5.fld1,fld2: _8.fld2 };
_6.fld2 = 1553483_u32 as i16;
_15 = [6_usize,12525875935456386501_usize,1_usize,14648766176438386996_usize,1962462695838131019_usize,3_usize,4314430676255821849_usize,2_usize];
_5 = Adt52 { fld0: _10.fld0,fld1: _10.fld1,fld2: _2.fld3.fld2 };
_8 = _2.fld3;
_7.fld1 = _10.fld1;
_8.fld0 = _9.fld0 << _10.fld0;
Goto(bb4)
}
bb8 = {
_1.2 = _6.fld1;
Goto(bb3)
}
bb9 = {
_1.2 = _2.fld3.fld1;
_5 = _7;
_2.fld3.fld1 = _1.2;
_7.fld0 = _8.fld1 as i128;
_8.fld0 = 59033_u16 as i128;
_8.fld1 = _6.fld1;
_6.fld1 = _10.fld1;
_2.fld0 = [(-2412928885777623338_i64),6692383154035578309_i64,(-7596873925485520556_i64),(-1179533590912090128_i64),(-3003607966911166102_i64)];
_9.fld0 = !_5.fld0;
_10.fld2 = 96_i8 as i16;
_7.fld2 = _6.fld2;
_10.fld1 = _5.fld1;
_8.fld0 = _11;
_10.fld0 = _5.fld0;
_6.fld0 = !_10.fld0;
_2.fld3.fld1 = _6.fld1;
_7.fld2 = _9.fld2 * _10.fld2;
_1.2 = _2.fld3.fld1;
_6.fld2 = _7.fld2 ^ _2.fld3.fld2;
_2.fld3.fld0 = _6.fld2 as i128;
_4 = [12480331007867880592_usize,16695686752879177435_usize,12969487583812325751_usize,9011685085843738511_usize,2_usize,3_usize,12610357000520006466_usize,6_usize];
_9.fld2 = -_2.fld3.fld2;
_10.fld0 = _1.1 as i128;
_4 = _2.fld1.0;
Goto(bb2)
}
bb10 = {
_2.fld3.fld1 = _10.fld1;
_2.fld3 = Adt52 { fld0: _8.fld0,fld1: _8.fld1,fld2: _9.fld2 };
_9.fld2 = _10.fld0 as i16;
_23 = core::ptr::addr_of_mut!(_2.fld0);
_22.fld5 = (_2.fld1.0,);
_21 = _9.fld0 as isize;
_9 = Adt52 { fld0: _5.fld0,fld1: _5.fld1,fld2: _7.fld2 };
_22.fld5 = (_15,);
_2.fld0 = [(-2633707488206904110_i64),1408830014359744857_i64,3338650011252557166_i64,(-1894777775325947812_i64),(-1939322882711807607_i64)];
_16 = _20 as f64;
Goto(bb11)
}
bb11 = {
_20 = 64702804165604197691943701331504909584_u128;
_22.fld7 = 7408410165789535657_u64;
_6.fld2 = 3172337676_u32 as i16;
_5.fld2 = _2.fld3.fld2 ^ _7.fld2;
_17.0 = !19811_u16;
_22.fld2 = !_21;
_6.fld2 = _1.1 as i16;
_5.fld1 = _8.fld1;
_19 = Adt50::Variant0 { fld0: _22.fld5 };
SetDiscriminant(_19, 1);
_4 = [5_usize,455110785296920286_usize,7_usize,7_usize,0_usize,5_usize,0_usize,2_usize];
_8.fld0 = _3 >> _21;
_22.fld4.fld3.0 = !_9.fld0;
_8.fld0 = _3;
RET = (-25_i8) as isize;
_11 = _20 as i128;
_22.fld6 = _17.0;
_12 = _22.fld2 ^ _22.fld2;
_22.fld6 = _17.0;
_22.fld4.fld1 = [5_usize,11844269980082917891_usize,4_usize,7_usize,4_usize,0_usize,5_usize,0_usize];
_26 = core::ptr::addr_of_mut!((*_23));
_25 = (-9131204046883348311_i64) as isize;
_5 = Adt52 { fld0: _11,fld1: _10.fld1,fld2: _2.fld3.fld2 };
_24 = _21 as f32;
_21 = !_22.fld2;
match _22.fld7 {
7408410165789535657 => bb12,
_ => bb5
}
}
bb12 = {
place!(Field::<Adt44>(Variant(_19, 1), 0)).fld3 = core::ptr::addr_of_mut!(_17);
_13 = [17828811112427747228_usize,3_usize,3_usize,4_usize,15993797926703853358_usize,5043235662930209291_usize,3_usize];
_3 = _2.fld3.fld0;
_22.fld5.0 = _4;
_19 = Adt50::Variant0 { fld0: _22.fld5 };
_3 = _1.1 as i128;
SetDiscriminant(_19, 0);
_7.fld0 = _8.fld0;
_10.fld1 = _7.fld1;
_22.fld1 = core::ptr::addr_of_mut!(_1);
_6.fld0 = _2.fld3.fld0 & _7.fld0;
_11 = !_7.fld0;
place!(Field::<([usize; 8],)>(Variant(_19, 0), 0)).0 = [7_usize,7_usize,4_usize,17118582705126396358_usize,2_usize,15253068239308490538_usize,4_usize,7_usize];
_6.fld0 = _22.fld4.fld3.0 & _10.fld0;
_22.fld4.fld4 = core::ptr::addr_of!(_17);
_12 = _22.fld2 >> _7.fld0;
SetDiscriminant(_19, 1);
_22.fld4.fld3 = (_9.fld0,);
Goto(bb13)
}
bb13 = {
_11 = _8.fld0 >> _10.fld0;
_22.fld3 = [_22.fld7,_22.fld7,_22.fld7,_22.fld7,_22.fld7,_22.fld7,_22.fld7,_22.fld7];
_10.fld1 = _2.fld3.fld1;
_5 = Adt52 { fld0: _10.fld0,fld1: _1.2,fld2: _2.fld3.fld2 };
_6.fld2 = false as i16;
_7.fld1 = _1.2;
_2.fld1 = (_22.fld4.fld1,);
_16 = 9749579651469486661_usize as f64;
_26 = core::ptr::addr_of_mut!((*_26));
_26 = core::ptr::addr_of_mut!((*_26));
_2.fld1 = (_22.fld4.fld1,);
_14 = core::ptr::addr_of!(_22.fld4.fld2);
_2.fld3.fld1 = _8.fld1;
_5.fld0 = _6.fld0 | _8.fld0;
_2.fld0 = [4140141989293275953_i64,(-5369914070394803321_i64),6894316962484535485_i64,(-7652662985281230286_i64),(-8477184385011451983_i64)];
_32 = _1.1 as usize;
Goto(bb14)
}
bb14 = {
_2.fld1 = _22.fld5;
_31.fld2 = core::ptr::addr_of!(_22.fld4.fld3);
RET = !_22.fld2;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(17_usize, 21_usize, Move(_21), 25_usize, Move(_25), 12_usize, Move(_12), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(17_usize, 11_usize, Move(_11), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: Adt52,mut _2: Adt52,mut _3: Adt52,mut _4: Adt52,mut _5: Adt54,mut _6: i128,mut _7: i128,mut _8: char,mut _9: i128,mut _10: Adt52,mut _11: Adt52,mut _12: i128) -> i128 {
mir! {
type RET = i128;
let _13: isize;
let _14: Adt48;
let _15: ((u16,), isize, (&'static bool, char), [usize; 8], usize);
let _16: i32;
let _17: i8;
let _18: isize;
let _19: usize;
let _20: Adt44;
let _21: [u64; 8];
let _22: u32;
let _23: Adt52;
let _24: Adt50;
let _25: u32;
let _26: u16;
let _27: char;
let _28: f64;
let _29: ();
let _30: ();
{
_5.fld3.fld0 = -_12;
_3.fld1 = _8;
_4.fld1 = _11.fld1;
_10 = Adt52 { fld0: _11.fld0,fld1: _2.fld1,fld2: _2.fld2 };
_3 = _2;
Goto(bb1)
}
bb1 = {
_1.fld2 = 9223372036854775807_isize as i16;
_13 = 111_isize;
_5.fld3.fld2 = !_11.fld2;
_16 = 2129258839_i32;
_12 = 3866175217_u32 as i128;
_11 = _2;
_11.fld2 = _5.fld3.fld2;
_5.fld3.fld0 = _2.fld0 | _3.fld0;
_10.fld2 = _1.fld0 as i16;
_5.fld3 = Adt52 { fld0: _6,fld1: _1.fld1,fld2: _10.fld2 };
_16 = (-693965774_i32);
_15.0 = (20729_u16,);
_10.fld2 = -_5.fld3.fld2;
_15.1 = -_13;
_4 = Adt52 { fld0: _10.fld0,fld1: _5.fld3.fld1,fld2: _10.fld2 };
_15.1 = 558961012446318657_u64 as isize;
_4 = Adt52 { fld0: _2.fld0,fld1: _5.fld3.fld1,fld2: _5.fld3.fld2 };
_5.fld1.0 = [7_usize,7_usize,1_usize,0_usize,1_usize,13591378653189537097_usize,2_usize,6_usize];
match _13 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
111 => bb8,
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
_3.fld1 = _10.fld1;
_10 = Adt52 { fld0: _11.fld0,fld1: _1.fld1,fld2: _5.fld3.fld2 };
_18 = -_13;
_2.fld1 = _5.fld3.fld1;
_2.fld1 = _10.fld1;
_10.fld2 = 253_u8 as i16;
_3.fld2 = !_5.fld3.fld2;
_6 = !_10.fld0;
_3.fld1 = _2.fld1;
_10 = _3;
_3 = Adt52 { fld0: _2.fld0,fld1: _2.fld1,fld2: _10.fld2 };
_3.fld0 = _2.fld0;
_19 = !15266407908609875230_usize;
_10.fld0 = -_1.fld0;
_17 = !(-91_i8);
_1.fld0 = _4.fld0 << _3.fld0;
_1.fld2 = _4.fld2;
_11.fld1 = _4.fld1;
_1.fld2 = _9 as i16;
Goto(bb9)
}
bb9 = {
_1.fld0 = _3.fld0 >> _2.fld0;
_19 = 1_usize;
_9 = _5.fld0[_19] as i128;
_10.fld2 = true as i16;
_15.3 = _5.fld1.0;
_8 = _2.fld1;
_5.fld3.fld1 = _2.fld1;
_1 = Adt52 { fld0: _4.fld0,fld1: _2.fld1,fld2: _5.fld3.fld2 };
_1 = Adt52 { fld0: _11.fld0,fld1: _10.fld1,fld2: _4.fld2 };
_9 = _6;
_20.fld3 = core::ptr::addr_of_mut!(_15.0);
_12 = -_9;
_8 = _4.fld1;
_20.fld3 = core::ptr::addr_of_mut!(_15.0);
_11 = Adt52 { fld0: _4.fld0,fld1: _3.fld1,fld2: _1.fld2 };
_25 = _15.0.0 as u32;
_20.fld3 = core::ptr::addr_of_mut!(_15.0);
_11.fld2 = _15.0.0 as i16;
Goto(bb10)
}
bb10 = {
_15.4 = _15.3[_19] / _5.fld1.0[_19];
_8 = _11.fld1;
_24 = Adt50::Variant0 { fld0: _5.fld1 };
_5.fld2 = core::ptr::addr_of!(_22);
RET = -_2.fld0;
_5.fld1.0 = _15.3;
_22 = _5.fld0[_19] as u32;
_23 = Adt52 { fld0: _10.fld0,fld1: _5.fld3.fld1,fld2: _5.fld3.fld2 };
_1.fld0 = -_11.fld0;
_10 = _1;
_24 = Adt50::Variant0 { fld0: _5.fld1 };
_5.fld0 = [(-6662397072785790004_i64),7314214183182718860_i64,(-1043191515369338388_i64),7348494232984191108_i64,(-2162513802611141547_i64)];
_23.fld0 = _2.fld0 + _1.fld0;
_2 = _11;
Goto(bb11)
}
bb11 = {
Call(_29 = dump_var(18_usize, 18_usize, Move(_18), 9_usize, Move(_9), 17_usize, Move(_17), 13_usize, Move(_13)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_29 = dump_var(18_usize, 6_usize, Move(_6), 7_usize, Move(_7), 30_usize, _30, 30_usize, _30), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{c0212}'), std::hint::black_box(116_isize), std::hint::black_box((-109_i8)), std::hint::black_box((-9730_i16)), std::hint::black_box(843520192_i32), std::hint::black_box((-7865077812454448874_i64)), std::hint::black_box(67496015345726190589207867173410458958_i128), std::hint::black_box(4673021256627711343_usize), std::hint::black_box(174_u8), std::hint::black_box(20380_u16), std::hint::black_box(1969283785_u32), std::hint::black_box(192760066050155378_u64), std::hint::black_box(57014491035199804320372522986387536968_u128));
                
            }
#[derive(Debug)]
pub enum Adt40 {
Variant0{
fld0: u128,
fld1: u32,

},
Variant1{
fld0: *const (i128,),

},
Variant2{
fld0: i64,
fld1: u32,
fld2: [u64; 8],
fld3: *const bool,

},
Variant3{
fld0: *const (u16,),

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt41 {
Variant0{
fld0: bool,
fld1: u16,
fld2: [usize; 7],
fld3: (i128,),
fld4: usize,
fld5: f32,
fld6: *const (u16,),

},
Variant1{
fld0: *const (u16,),

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt42 {
fld0: *const bool,
fld1: [usize; 8],
fld2: *const *mut (u16,),
fld3: (i128,),
fld4: *const (u16,),
}
#[derive(Debug)]
pub struct Adt43 {
fld0: Adt40,
fld1: *mut (*mut (u16,), u8, char),
fld2: isize,
fld3: [u64; 8],
fld4: Adt42,
fld5: ([usize; 8],),
fld6: u16,
fld7: u64,
}
#[derive(Debug,Copy,Clone)]
pub struct Adt44 {
fld0: i32,
fld1: *const bool,
fld2: *const (i128,),
fld3: *mut (u16,),
}
#[derive(Debug)]
pub enum Adt45 {
Variant0{
fld0: isize,
fld1: *mut [i64; 5],

},
Variant1{
fld0: bool,
fld1: *const bool,
fld2: isize,
fld3: [usize; 7],

},
Variant2{
fld0: [usize; 8],
fld1: Adt43,
fld2: Adt42,
fld3: Adt44,
fld4: u32,
fld5: *const i64,

}}
#[derive(Debug)]
pub enum Adt46 {
Variant0{
fld0: bool,
fld1: usize,
fld2: *const *const *mut (u16,),
fld3: u32,
fld4: [i64; 5],

},
Variant1{
fld0: u16,
fld1: Adt43,
fld2: [u32; 5],
fld3: *const i64,
fld4: (*mut (u16,), u8, char),

},
Variant2{
fld0: (*mut (u16,), u8, char),

}}
#[derive(Debug)]
pub enum Adt47 {
Variant0{
fld0: u16,

},
Variant1{
fld0: u128,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt48 {
Variant0{
fld0: *const i64,
fld1: char,
fld2: [usize; 8],

},
Variant1{
fld0: i64,
fld1: f32,
fld2: Adt44,
fld3: Adt42,

}}
#[derive(Debug)]
pub enum Adt49 {
Variant0{
fld0: u8,
fld1: u128,
fld2: [u64; 8],
fld3: i8,
fld4: u32,
fld5: (*mut (u16,), u8, char),
fld6: *const bool,

},
Variant1{
fld0: *const i64,
fld1: *const *const *mut (u16,),
fld2: (i128,),

},
Variant2{
fld0: [u64; 8],
fld1: [usize; 7],
fld2: isize,
fld3: i8,
fld4: u32,
fld5: u128,
fld6: [u32; 5],
fld7: f32,

}}
#[derive(Debug)]
pub enum Adt50 {
Variant0{
fld0: ([usize; 8],),

},
Variant1{
fld0: Adt44,
fld1: [u64; 8],
fld2: u8,

}}
#[derive(Debug)]
pub enum Adt51 {
Variant0{
fld0: [u64; 8],
fld1: [usize; 7],
fld2: *const *mut (u16,),
fld3: Adt40,
fld4: *const (i128,),

},
Variant1{
fld0: Adt40,
fld1: f32,
fld2: (*mut (u16,), u8, char),
fld3: *mut [i64; 5],

},
Variant2{
fld0: Adt45,
fld1: Adt48,
fld2: isize,
fld3: *mut [i64; 5],
fld4: *const u32,
fld5: [usize; 7],
fld6: i64,
fld7: u64,

}}
#[derive(Debug,Copy,Clone)]
pub struct Adt52 {
fld0: i128,
fld1: char,
fld2: i16,
}
#[derive(Debug)]
pub enum Adt53 {
Variant0{
fld0: Adt50,
fld1: Adt41,
fld2: u32,
fld3: (*mut (u16,), u8, char),
fld4: Adt46,
fld5: u8,

},
Variant1{
fld0: ([usize; 8],),
fld1: [usize; 8],
fld2: Adt51,
fld3: u8,
fld4: Adt47,
fld5: (u16,),
fld6: [u64; 8],

}}
#[derive(Debug)]
pub struct Adt54 {
fld0: [i64; 5],
fld1: ([usize; 8],),
fld2: *const u32,
fld3: Adt52,
}
#[derive(Debug)]
pub struct Adt55 {
fld0: i32,
fld1: u32,
fld2: *const (i128,),
fld3: ([usize; 8],),
}
#[derive(Debug)]
pub struct Adt56 {
fld0: *const i64,
fld1: char,
fld2: u128,
fld3: Adt43,
fld4: *const *mut (u16,),
fld5: Adt41,
fld6: [i32; 3],
}

