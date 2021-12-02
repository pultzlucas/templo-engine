use super::*;

#[test]

fn fn_str_test() {
    let args = vec!["good".to_string(), " ".to_string(), "life".to_string()];
    assert_eq!(Str::call(&args), "good life");
}

#[test]

fn fn_getchar_test() {
    let args = vec!["rust".to_string(), "1".to_string()];
    assert_eq!(GetChar::call(&args), "u");
}