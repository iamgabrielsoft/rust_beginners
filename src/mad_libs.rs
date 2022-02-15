/// # Parameters
/// `exclamation` - An exclamation (Waoh, Cool, Shoot)
///
/// `adverb` - An adverb (happily, sadly)
///
/// `noun` - A noun (tree, rock, trash can)
///
/// `adjective0` - An adjective (exciting, kind, brave)
///
/// `adjective1` - A second adjective

//the function returns a string as output
pub fn mad_lib_generator(
    exclamation: &str, 
    adverb: &str, 
    noun: &str, 
    adjective0: &str, 
    adjective1: &str
) -> String {
    let first_letter = adjective1.get(0..1);
    let article = match first_letter {
        Some(f) => {
            if f == "a" || f == "e" || f == "i" || f == "o" || f == "u" {
                "an"
            } else {
                "a"
            }
        }
        None => "b",
    };
    format!(
        "{}! He said {} as he jumped into his convertable {} and drove off with his {} wife. That was {} {} time",
        exclamation, adverb, noun, adjective0, article, "exciting"
    )
}