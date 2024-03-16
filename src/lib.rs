pub fn twofer(name: &str) -> String {

    let fer_two = match name {
        "" => String::from("One for you, one for me."),
        _ => format!("One for {}, one for me.", name),
    };
    fer_two
}