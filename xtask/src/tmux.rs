use std::env::vars;

pub fn runs() -> bool {
    vars().find(|(key, _)| key == "TMUX").is_some()
}
