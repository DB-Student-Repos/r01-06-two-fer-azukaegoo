pub fn twofer(name: &str) -> String {
    let fer = name;

    let fer_two = match fer {
        "" => String::from("One for you, one for me."),
        name => format!("One for {}, one for me.", name),
    };

    fer_two
}
