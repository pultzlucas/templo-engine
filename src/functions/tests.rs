use super::*;

#[test]

fn fn_getchar_test() {
    let args = vec!["1".to_string(), "rust".to_string()];
    assert_eq!(GetChar.call(&args), "u");
}

#[test]

fn fn_getindex_test() {
    let args = vec!["rust".to_string(), "r".to_string()];
    assert_eq!(GetIndex.call(&args), "0".to_string())
}
