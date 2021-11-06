use super::*;

#[test]

fn fn_str_test() {
    let args = vec!["good".to_string(), " ".to_string(), "life".to_string()];
    assert_eq!(Str::call(&args), "good life");
}