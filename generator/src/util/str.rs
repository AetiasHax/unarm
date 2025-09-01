pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    let Some(c) = chars.next() else {
        return String::new();
    };
    c.to_uppercase().chain(chars).collect()
}

pub fn snake_to_pascal_case(s: &str) -> String {
    // s.split('_').map(|w| capitalize(w)).collect::<Vec<_>>().join("")
    let mut result = String::new();
    let mut capitalized = false;
    for c in s.chars() {
        match c {
            'a'..='z' => {
                if capitalized {
                    result.push(c);
                } else {
                    result.push(c.to_ascii_uppercase());
                    capitalized = true;
                }
            }
            'A'..='Z' => {
                result.push(c);
                capitalized = true;
            }
            '_' => capitalized = false,
            '0'..='9' => {
                result.push(c);
                capitalized = false;
            }
            _ => result.push(c),
        }
    }
    result
}
