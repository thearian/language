pub fn statements() -> Vec<[&'static str; 2]> {
    vec!(
        [r"remember that (?P<n>\w+) is (?P<d>.+) and gets (?P<p>.+)\r\n","def $n($p):\n\t$d"],
        [r"remember that (?P<n>\w+) is (?P<d>.+)","def $n():\n\t$d"],
        [r"get (?P<n>.+)\r\n", "input($n)\n"],
        [r" (?P<a>.\w+) to (?P<b>\w+)"," range($a,$b)"],
    )
}