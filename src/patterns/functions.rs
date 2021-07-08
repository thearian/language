pub fn statements() -> Vec<[&'static str; 2]> {
    vec!(
        [r"get (?P<n>.+)", "input($n)"],
        [r"remember that (?P<n>.+) is (?P<d>.+)","def $n():\n\t$d"],
    )
}