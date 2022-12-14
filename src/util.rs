pub fn next_line_prefixed<'a, I>(lines: &mut I, prefix: &str) -> &'a str
where
    I: Iterator<Item = &'a str>,
{
    lines.next().unwrap().strip_prefix(prefix).unwrap()
}
