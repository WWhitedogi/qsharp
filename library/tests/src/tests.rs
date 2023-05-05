// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::run_stdlib_test;
use num_bigint::BigInt;
use qsc::interpret::Value;

//
// Sign, Abs, Min, Max, etc.
//

#[test]
fn check_sign_i() {
    run_stdlib_test("Microsoft.Quantum.Math.SignI(0)", &Value::Int(0));
    run_stdlib_test("Microsoft.Quantum.Math.SignI(1000)", &Value::Int(1));
    run_stdlib_test("Microsoft.Quantum.Math.SignI(-1000)", &Value::Int(-1));
}

#[test]
fn check_sign_d() {
    run_stdlib_test("Microsoft.Quantum.Math.SignD(0.0)", &Value::Int(0));
    run_stdlib_test("Microsoft.Quantum.Math.SignD(0.005)", &Value::Int(1));
    run_stdlib_test("Microsoft.Quantum.Math.SignD(-0.005)", &Value::Int(-1));
}

#[test]
fn check_sign_l() {
    run_stdlib_test("Microsoft.Quantum.Math.SignL(0L)", &Value::Int(0));
    run_stdlib_test(
        "Microsoft.Quantum.Math.SignL(9999999999999999999999999999999999999999L)",
        &Value::Int(1),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.SignL(-9999999999999999999999999999999999999999L)",
        &Value::Int(-1),
    );
}

#[test]
fn check_abs_i() {
    run_stdlib_test("Microsoft.Quantum.Math.AbsI(0)", &Value::Int(0));
    run_stdlib_test("Microsoft.Quantum.Math.AbsI(1000)", &Value::Int(1000));
    run_stdlib_test("Microsoft.Quantum.Math.AbsI(-1000)", &Value::Int(1000));
}

#[test]
fn check_abs_d() {
    run_stdlib_test("Microsoft.Quantum.Math.AbsD(0.0)", &Value::Double(0.0));
    run_stdlib_test("Microsoft.Quantum.Math.AbsD(0.005)", &Value::Double(0.005));
    run_stdlib_test("Microsoft.Quantum.Math.AbsD(-0.005)", &Value::Double(0.005));
}

#[test]
fn check_abs_l() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.AbsL(0L)",
        &Value::BigInt(BigInt::from(0)),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.AbsL(9999L)",
        &Value::BigInt(BigInt::from(9999)),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.AbsL(-9999L)",
        &Value::BigInt(BigInt::from(9999)),
    );
}

#[test]
fn check_max_i() {
    run_stdlib_test("Microsoft.Quantum.Math.MaxI(-5,7)", &Value::Int(7));
    run_stdlib_test("Microsoft.Quantum.Math.MaxI(-7,0)", &Value::Int(0));
}

#[test]
fn check_max_d() {
    run_stdlib_test("Microsoft.Quantum.Math.MaxD(-5.0,7.0)", &Value::Double(7.0));
    run_stdlib_test("Microsoft.Quantum.Math.MaxD(-7.0,0.0)", &Value::Double(0.0));
}

#[test]
fn check_max_l() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.MaxL(-5L,7L)",
        &Value::BigInt(BigInt::from(7)),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.MaxL(-7L,0L)",
        &Value::BigInt(BigInt::from(0)),
    );
}

#[test]
fn check_min_i() {
    run_stdlib_test("Microsoft.Quantum.Math.MinI(-5,7)", &Value::Int(-5));
    run_stdlib_test("Microsoft.Quantum.Math.MinI(-7,0)", &Value::Int(-7));
}

#[test]
fn check_min_d() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.MinD(-5.0,7.0)",
        &Value::Double(-5.0),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.MinD(-7.0,0.0)",
        &Value::Double(-7.0),
    );
}

#[test]
fn check_min_l() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.MinL(-5L,7L)",
        &Value::BigInt(BigInt::from(-5)),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.MinL(-7L,0L)",
        &Value::BigInt(BigInt::from(-7)),
    );
}

//
// Trigonometric functions
//

#[test]
fn check_pi() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.PI()",
        &Value::Double(std::f64::consts::PI),
    );
}

#[test]
fn check_e() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.E()",
        &Value::Double(std::f64::consts::E),
    );
}

#[test]
fn check_arccosh() {
    run_stdlib_test("Microsoft.Quantum.Math.ArcCosh(1.0)", &Value::Double(0.0));
}

#[test]
fn check_arcsinh() {
    run_stdlib_test("Microsoft.Quantum.Math.ArcSinh(0.0)", &Value::Double(0.0));
}

#[test]
fn check_arctanh() {
    run_stdlib_test("Microsoft.Quantum.Math.ArcTanh(0.0)", &Value::Double(0.0));
}

//
// Sqrt, Log, exp, etc.
//

#[test]
fn check_log10() {
    run_stdlib_test("Microsoft.Quantum.Math.Log10(1.0)", &Value::Double(0.0));
    run_stdlib_test("Microsoft.Quantum.Math.Log10(10.0)", &Value::Double(1.0));
}

#[test]
fn check_lg() {
    run_stdlib_test("Microsoft.Quantum.Math.Lg(1.0)", &Value::Double(0.0));
    run_stdlib_test("Microsoft.Quantum.Math.Lg(2.0)", &Value::Double(1.0));
}

//
// Modular arithmetic
//

#[test]
fn check_modulus_i() {
    run_stdlib_test("Microsoft.Quantum.Math.ModulusI(20, 3)", &Value::Int(2));
    run_stdlib_test("Microsoft.Quantum.Math.ModulusI(-20, 3)", &Value::Int(1));
}

#[test]
fn check_modulus_l() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.ModulusL(20L, 3L)",
        &Value::BigInt(BigInt::from(2)),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.ModulusL(-20L, 3L)",
        &Value::BigInt(BigInt::from(1)),
    );
}

#[test]
fn check_exp_mod_i() {
    run_stdlib_test("Microsoft.Quantum.Math.ExpModI(1,10,10)", &Value::Int(1));
    run_stdlib_test("Microsoft.Quantum.Math.ExpModI(10,0,10)", &Value::Int(1));
    run_stdlib_test("Microsoft.Quantum.Math.ExpModI(2,10,10)", &Value::Int(4));
}

#[test]
fn check_exp_mod_l() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.ExpModL(1L,10L,10L)",
        &Value::BigInt(BigInt::from(1)),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.ExpModL(10L,0L,10L)",
        &Value::BigInt(BigInt::from(1)),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.ExpModL(2L,10L,10L)",
        &Value::BigInt(BigInt::from(4)),
    );
}

//
// GCD, etc.
//
#[test]
fn check_gcd_i() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.GreatestCommonDivisorI(0,0)",
        &Value::Int(0),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.GreatestCommonDivisorI(2*3*5,2*3*7)",
        &Value::Int(2 * 3),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.GreatestCommonDivisorI(39088169,63245986)",
        &Value::Int(1),
    );
}

#[test]
fn check_gcd_l() {
    run_stdlib_test(
        "Microsoft.Quantum.Math.GreatestCommonDivisorL(0L,0L)",
        &Value::BigInt(BigInt::from(0)),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Math.GreatestCommonDivisorL(2L*3L*5L,2L*3L*7L)",
        &Value::BigInt(BigInt::from(2 * 3)),
    );
    run_stdlib_test("Microsoft.Quantum.Math.GreatestCommonDivisorL(222232244629420445529739893461909967206666939096499764990979600L,359579325206583560961765665172189099052367214309267232255589801L)", &Value::BigInt(
        BigInt::from(1)));
}

#[test]
fn check_fst_snd() {
    run_stdlib_test("Fst(7,6)", &Value::Int(7));
    run_stdlib_test("Snd(7,6)", &Value::Int(6));
}

#[test]
fn check_index_range() {
    run_stdlib_test(
        "Microsoft.Quantum.Arrays.IndexRange([7,6,5,4])::Start",
        &Value::Int(0),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Arrays.IndexRange([7,6,5,4])::Step",
        &Value::Int(1),
    );
    run_stdlib_test(
        "Microsoft.Quantum.Arrays.IndexRange([7,6,5,4])::End",
        &Value::Int(3),
    );
}

#[test]
fn check_bitsize_i() {
    run_stdlib_test("Microsoft.Quantum.Math.BitSizeI(0)", &Value::Int(0));
    run_stdlib_test("Microsoft.Quantum.Math.BitSizeI(1)", &Value::Int(1));
    run_stdlib_test("Microsoft.Quantum.Math.BitSizeI(2)", &Value::Int(2));
    run_stdlib_test("Microsoft.Quantum.Math.BitSizeI(3)", &Value::Int(2));
    run_stdlib_test(
        "Microsoft.Quantum.Math.BitSizeI(0x7FFFFFFFFFFFFFFF)",
        &Value::Int(63),
    );
}