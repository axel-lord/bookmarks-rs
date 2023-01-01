use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_command(line: &str) -> Option<Vec<String>> {
    lazy_static! {
        static ref ARG_RE: Regex =
            Regex::new(r#"\s*"(.*?)"\s*|$"#).expect("failure to compile ARG_RE regex");
    }

    let mut next_start = 0;
    let mut arg_vec = Vec::new();

    for capture in ARG_RE.captures_iter(line) {
        // if capture exists so should whole capture
        let whole_capture = capture
            .get(0)
            .expect("could not get whole capture of regex (should never happen)");

        // used when parsing no quoted arguments
        let before = &line[next_start..whole_capture.start()];
        next_start = whole_capture.end();

        // iterate over quoted arguments appearing before captured end or quoted argument
        // and add them to arg vector
        arg_vec.extend(before.split_whitespace().map(String::from));

        // if a quoted argument was captured add it
        let Some(quoted_arg) = capture.get(1) else {
            continue;
        };

        arg_vec.push(quoted_arg.as_str().into());
    }

    Some(arg_vec)
}
