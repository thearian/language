pub fn statements() -> Vec<[&'static str; 2]> {
    vec!(
        [r"untill (?P<t>.+)\r\n", "while not $t:\n\t"],
        [r"loop in (?P<t>.+)\r\n", "for index in $t:\n\t"],
        [r"do (?P<t>.+) forever", "while True:\n\t$t"],
    )
}