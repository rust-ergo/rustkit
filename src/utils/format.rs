/// Remove quotes from a string
pub fn remove_quotes(i_str: String) -> String {
    let n_str: String = i_str.replace('"', "");
    n_str
}