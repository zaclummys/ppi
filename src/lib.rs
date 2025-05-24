/**
 * Transformations of boolean algebra to restriction of a linear programming problem.
 * 
 * Rules:
 * - Only integer values from 0 to 1 should be used as inputs.
 * - Only boolean values should be allowed as outputs.
 * - The allowed operators are:
 *  - Logical AND (&&)
 *  - Equal (==)
 *  - Less Than (<)
 *  - Less Than or Equal (<=)
 *  - Greater Than (>)
 *  - Greater Than or Equal (>=)
 *  - Addition (+)
 *  - Subtraction (-)
 * - It cannot use:
 *  - boolean values directly (true, false).
 *  - bitwise operators.
 *  - arithmetic multiplication.
 *  - arithmetic division.
 *  - boolean operators as OR (||) and NOT(!) directly.
 *  - unequal operator (!=).
 *  - Expressions can be expressed as a combination of another expressions.
 *  - casting to boolean or integer.
 */

// ¬ a
pub fn not (a: u8) -> bool {
    a == 0
}

// a ∧ b
pub fn and (a: u8, b: u8) -> bool {
    a == 1 && b == 1
}

// a ∨ b
pub fn or (a: u8, b: u8) -> bool {
    a + b >= 1
}

// a ⊕ b
pub fn xor (a: u8, b: u8) -> bool {
    a + b == 1
}

// a → b
pub fn implication (a: u8, b: u8) -> bool {
    a <= b
}

// a → (b ∧ c)
pub fn implication_with_double_consequent_and (a: u8, b: u8, c: u8) -> bool {
    a <= b && a <= c
}

// a → (b ∨ c)
pub fn implication_with_double_consequent_or (a: u8, b: u8, c: u8) -> bool {
    a <= b + c
}

// (a ∧ b) → c
pub fn implication_with_double_antecedent_and (a: u8, b: u8, c: u8) -> bool {
    a + b <= c + 1
}

// (a ∨ b) → c
pub fn implication_with_double_antecedent_or (a: u8, b: u8, c: u8) -> bool {
    a <= c && b <= c
}

// a ↔ b
pub fn equivalence (a: u8, b: u8) -> bool {
    a == b
}

// ¬ (a ∧ b)
pub fn nand (a: u8, b: u8) -> bool {
    a + b <= 1
}

// ¬ (a ∨ b)
pub fn nor (a: u8, b: u8) -> bool {
    a + b == 0
}

// a ∧ ¬ b
pub fn a_and_not_b (a: u8, b: u8) -> bool {
    a > b
}

// a ∨ ¬ b
pub fn a_or_not_b (a: u8, b: u8) -> bool {
    b <= a
}

#[test]
fn test_not () {
    for (a, expected) in [
        (0, true),
        (1, false),
    ] {
        let actual = not(a);

        assert_eq!(actual, expected, "a: {}", a);
    }
}

#[test]
fn test_and () {
    for (a, b, expected) in [
        (0, 0, false),
        (0, 1, false),
        (1, 0, false),
        (1, 1, true),
    ] {
        let actual = and(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}

#[test]
fn test_or () {
    for (a, b, expected) in [
        (0, 0, false),
        (0, 1, true),
        (1, 0, true),
        (1, 1, true),
    ] {
        let actual = or(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}

#[test]
fn test_xor () {
    for (a, b, expected) in [
        (0, 0, false),
        (0, 1, true),
        (1, 0, true),
        (1, 1, false),
    ] {
        let actual = xor(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}

#[test]
fn test_implication () {
    for (a, b, expected) in [
        (0, 0, true),
        (0, 1, true),
        (1, 0, false),
        (1, 1, true),
    ] {
        let actual = implication(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}

#[test]
fn test_equivalence () {
    for (a, b, expected) in [
        (0, 0, true),
        (0, 1, false),
        (1, 0, false),
        (1, 1, true),
    ] {
        let actual = equivalence(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}

#[test]
fn test_nand () {
    for (a, b, expected) in [
        (0, 0, true),
        (0, 1, true),
        (1, 0, true),
        (1, 1, false),
    ] {
        let actual = nand(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}

#[test]
fn test_nor () {
    for (a, b, expected) in [
        (0, 0, true),
        (0, 1, false),
        (1, 0, false),
        (1, 1, false),
    ] {
        let actual = nor(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}

#[test]
fn test_if_a_then_b_and_c () {
    for (a, b, c, expected) in [
        (0, 0, 0, true),
        (0, 0, 1, true),
        (0, 1, 0, true),
        (0, 1, 1, true),
        (1, 0, 0, false),
        (1, 0, 1, false),
        (1, 1, 0, false),
        (1, 1, 1, true),
    ] {
        let actual = implication_with_double_consequent_and(a, b, c);

        assert_eq!(actual, expected, "a: {}, b: {}, c: {}", a, b, c);
    }
}

#[test]
fn test_if_a_then_b_or_c () {
    for (a, b, c, expected) in [
        (0, 0, 0, true),
        (0, 0, 1, true),
        (0, 1, 0, true),
        (0, 1, 1, true),
        (1, 0, 0, false),
        (1, 0, 1, true),
        (1, 1, 0, true),
        (1, 1, 1, true),
    ] {
        let actual = implication_with_double_consequent_or(a, b, c);

        assert_eq!(actual, expected, "a: {}, b: {}, c: {}", a, b, c);
    }
}

#[test]
fn test_if_a_and_b_then_c () {
    for (a, b, c, expected) in [
        (0, 0, 0, true),
        (0, 0, 1, true),
        (0, 1, 0, true),
        (0, 1, 1, true),
        (1, 0, 0, true),
        (1, 0, 1, true),
        (1, 1, 0, false),
        (1, 1, 1, true),
    ] {
        let actual = implication_with_double_antecedent_and(a, b, c);

        assert_eq!(actual, expected, "a: {}, b: {}, c: {}", a, b, c);
    }
}

#[test]
fn test_if_a_or_b_then_c () {
    for (a, b, c, expected) in [
        (0, 0, 0, true),
        (0, 0, 1, true),
        (0, 1, 0, false),
        (0, 1, 1, true),
        (1, 0, 0, false),
        (1, 0, 1, true),
        (1, 1, 0, false),
        (1, 1, 1, true),
    ] {
        let actual = implication_with_double_antecedent_or(a, b, c);

        assert_eq!(actual, expected, "a: {}, b: {}, c: {}", a, b, c);
    }
}

#[test]
fn test_a_and_not_b () {
    for (a, b, expected) in [
        (0, 0, false),
        (0, 1, false),
        (1, 0, true),
        (1, 1, false),
    ] {
        let actual = a_and_not_b(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}

#[test]
fn test_a_or_not_b () {
    for (a, b, expected) in [
        (0, 0, true),
        (0, 1, false),
        (1, 0, true),
        (1, 1, true),
    ] {
        let actual = a_or_not_b(a, b);

        assert_eq!(actual, expected, "a: {}, b: {}", a, b);
    }
}