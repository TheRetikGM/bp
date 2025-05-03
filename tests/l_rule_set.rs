use std::rc::Rc;

use music_sheet_gen::lsystem::*;

fn create_basic_set() -> CSSLRuleSet {
    CSSLRuleSet::new(vec![
        CSSLRule::new("a", "b", 1. / 2.),
        CSSLRule::new("a", "c", 1. / 2.),
        CSSLRule::new("b", "d", 1.),
    ])
}

fn create_context_set() -> CSSLRuleSet {
    CSSLRuleSet::new(vec![
        CSSLRule::new("F", "AA", 1. / 4.),
        CSSLRule::new("+F", "BB", 1. / 4.),
        CSSLRule::new("F+F", "CC", 1. / 4.),
        CSSLRule::new("F-F", "DD", 1. / 4.),
    ])
}

fn assert_rule_eq(rule: &Rc<impl LRule>, left: &str, right: &str) {
    assert_eq!(rule.left(), left);
    assert_eq!(rule.right(), right);
}

fn assert_rule_any(rule: &Rc<impl LRule>, rules: &[(&str, &str)]) {
    assert!(rules
        .iter()
        .any(|r| rule.left() == r.0 && rule.right() == r.1));
}

fn assert_many_times<F: Fn()>(times: i32, func: F) {
    for _ in 0..times {
        func()
    }
}

#[test]
pub fn select_simple1() {
    // Arrange
    let set = create_basic_set();

    // Act
    let rule1 = set.select("a");
    let rule2 = set.select("b");
    let rule3 = set.select("ab");
    let rule4 = set.select("ba");
    let rule5 = set.select("abc");

    // Assert
    assert_many_times(5, || {
        assert_rule_any(rule1.unwrap(), &[("a", "b"), ("a", "c")])
    });
    assert_rule_eq(rule2.unwrap(), "b", "d");
    assert_rule_eq(rule3.unwrap(), "b", "d");
    assert_many_times(5, || {
        assert_rule_any(rule4.unwrap(), &[("a", "b"), ("a", "c")])
    });
    assert!(rule5.is_none());
}

#[test]
fn select_simple2() {
    // Arrange
    let set = CSSLRuleSet::new(
        ["a -> 1 % 1/1", "b -> 2 % 1/1", "d -> 3 % 1/1"]
            .iter()
            .map(|r| music_sheet_gen::lsystem::l_rule::ToCSSLRule::to_csslrule(&r).unwrap())
            .collect(),
    );

    // Act
    let res1 = set.select("bac");
    let res2 = set.select("aslkdbca");
    let res3 = set.select("ca");
    let res4 = set.select("a");
    let res5 = set.select("bad");

    // Assert
    assert!(res1.is_none());
    assert_rule_eq(res2.unwrap(), "a", "1");
    assert_rule_eq(res3.unwrap(), "a", "1");
    assert_rule_eq(res4.unwrap(), "a", "1");
    assert_rule_eq(res5.unwrap(), "d", "3");
}

#[test]
pub fn select_context() {
    // Arrange
    let set = create_context_set();

    // Act && Assert
    assert_many_times(10, || assert_rule_eq(set.select("FFF").unwrap(), "F", "AA"));
    assert_many_times(10, || {
        assert_rule_any(
            set.select("F-F+F").unwrap(),
            &[("F", "AA"), ("+F", "BB"), ("F+F", "CC")],
        )
    });
    assert_many_times(10, || {
        assert_rule_any(set.select("F-F").unwrap(), &[("F", "AA"), ("F-F", "DD")])
    });
    assert_many_times(10, || {
        assert_rule_any(set.select("--F").unwrap(), &[("F", "AA")])
    });
    assert_many_times(10, || {
        assert_rule_any(set.select("++F").unwrap(), &[("F", "AA"), ("+F", "BB")])
    });
}
