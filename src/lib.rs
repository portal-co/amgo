#![no_std]
pub const AMOGUS: &'static str = "ඞ";
pub const TAG: u32 = 0xc0;
pub fn amgo<'a>(a: &'a str, key: &str) -> Option<impl Iterator<Item = u8> + use<'a>> {
    let (_, a) = a.split_once(AMOGUS)?;
    let a = a.strip_prefix(key)?;
    let a = a.strip_prefix(AMOGUS)?;
    Some(a.chars().filter_map(|c| match c as u32 {
        a if a > TAG => Some(((a - TAG) & 0xff) as u8),
        _ => None,
    }))
}
pub fn demgo(key: &str, a: impl Iterator<Item = u8>) -> impl Iterator<Item = char> {
    AMOGUS
        .chars()
        .chain(key.chars())
        .chain(AMOGUS.chars())
        .chain(a.filter_map(|a| char::from_u32((a as u32) + TAG)))
}
