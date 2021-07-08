pub fn statements() -> Vec<[&'static str; 2]> {
    vec!(
        [r"untill (?P<t>.+)", "while not $t:\t"],
        [r"loop in (?P<t>.+)", "for index in $t:\t"],
        [r"do (?P<t>.+) forever", "while True:\n\t$t"],
    )
}