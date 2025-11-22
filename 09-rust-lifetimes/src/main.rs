// 'a is a lifetime annotation ('a is convention e.g.: 'my_lifetime would also be valid)
// lifetime annotations must be used used in functions that require two or more references
// as arguments and that also return references.
// this is needed in order to specify Rust, from which one of the arguments
// the reference is likely to come from (what type of reference it is)
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }
    languages.last().unwrap()
}

// the function takes two references and returns a reference
// but there's no way of knowing from which one of the
// arguments the return comes from 
// both of them must be have the same lifetime annotation
fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() > lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

// the function takes one reference argument and returns a reference
// no lifetime annotation is needed here
// this would also be the case if the function would take more non-reference arguments
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

// the function takes three references but doesn't return a reference
// no lifetime annotation needed here
fn get_language_or(languages: &[String], search: &str, fallback: &str) -> String {
    for lang in languages {
        if lang.contains(search) {
            return String::from(lang);
        }
        
    }

    return fallback.to_string();
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("python"),
        String::from("javascript")
    ];

    let result = next_language(&languages, "go");
    println!("Next: {}", result);

    let longest = longest_language("rust", "go");
    println!("Longest: {}", longest);

    let last = last_language(&languages);
    println!("Last: {}", last);

    let language = get_language_or(&languages, "ty", "Typescript");
    println!("Found: {}", language);

}
