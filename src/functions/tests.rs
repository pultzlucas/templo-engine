use super::*;

// #[test]

// fn fn_str_test() {
//     let args = vec!["good".to_string(), " ".to_string(), "life".to_string()];
//     assert_eq!(Str::call(&args), "good life");
// }

// #[test]

// fn fn_getchar_test() {
//     let args = vec!["rust".to_string(), "1".to_string()];
//     assert_eq!(GetChar::call(&args), "u");
// }

#[test]

fn fn_getindex_test() {
    let args = vec!["rust".to_string(), "r".to_string()];
    let get_index = GetIndex{};
    assert_eq!(get_index.call(&args), "0".to_string())
}
