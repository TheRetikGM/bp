use music_sheet_gen::lsystem::{
    l_rewriter::{CSSLRewriter, LRewriter},
    l_rule::ToCSSLRule,
    l_rule_set::CSSLRuleSet,
};

fn create_rewriter_simple() -> CSSLRewriter {
    CSSLRewriter::new(CSSLRuleSet::new(
        ["a -> 1 % 1/1", "b -> 2 % 1/1", "d -> 3 % 1/1"]
            .iter()
            .map(|r| r.to_csslrule().unwrap())
            .collect(),
    ))
}

fn create_rewriter_complex() -> CSSLRewriter {
    CSSLRewriter::new(CSSLRuleSet::new(
        [
            "def -> 11 % 1/1",
            "bcd -> 22 % 1/1",
            " bc -> 33 % 1/1",
            " ab -> 44 % 1/1",
            "  a -> 55 % 1/1",
        ]
        .iter()
        .map(|r| r.to_csslrule().unwrap())
        .collect(),
    ))
}

#[test]
fn rewrite_simple() {
    // Arrange
    let re = create_rewriter_simple();

    // Act
    let res = re.rewrite("abcdef");

    // Assert
    assert_eq!("12c3ef", res);
}

#[test]
fn rewrite_complex() {
    // Arrange
    let re = create_rewriter_complex();

    // Act
    let res = re.rewrite("abcdef");

    // Assert
    assert_eq!("553311", res);
}
