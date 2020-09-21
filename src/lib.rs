use ansi_term::Colour::{Black, Blue, Cyan, Green, Purple, Red, White, Yellow};
use std::error::Error;
use std::fmt::Write;

fn pinta(color: &str, obj: &str) -> String {
    match color {
        "Black" => Black.paint(obj).to_string(),
        "Red" => Red.paint(obj).to_string(),
        "Green" => Green.paint(obj).to_string(),
        "Yellow" => Yellow.paint(obj).to_string(),
        "Blue" => Blue.paint(obj).to_string(),
        "Purple" => Purple.paint(obj).to_string(),
        "Cyan" => Cyan.paint(obj).to_string(),
        "White" => White.paint(obj).to_string(),
        _ => obj.to_string(),
    }
}

fn printer(colors: Vec<&str>, objs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let mut s = String::new();
    for o in &objs {
        for c in &colors {
            write!(s, "{}", pinta(c, o))?;
        }
        writeln!(s)?;
    }
    Ok(s)
}

pub fn panes(bw: bool) -> String {
    let vec = vec![" ███▄ ", " ████ ", " ████ ", "  ▀▀▀ "];
    let mut colors = vec!["Red", "Green", "Yellow", "Blue", "Purple", "Cyan"];
    if bw {
        colors.insert(0, "Black");
        colors.push("White");
    }
    printer(colors, vec).unwrap()
}

pub fn bars(bw: bool) -> String {
    let vec = vec![" ▬▬▬▬▬"; 2];
    let mut colors = vec!["Red", "Green", "Yellow", "Blue", "Purple", "Cyan"];
    if bw {
        colors.insert(0, "Black");
        colors.push("White");
    }
    printer(colors, vec).unwrap()
}

pub fn blocks1(bw: bool) -> String {
    let vec = vec![" ██ "; 2];
    let mut colors = vec!["Red", "Green", "Yellow", "Blue", "Purple", "Cyan"];
    if bw {
        colors.insert(0, "Black");
        colors.push("White");
    }
    printer(colors, vec).unwrap()
}

pub fn bloks(bw: bool) -> String {
    let vec = vec!["████████ "; 3];
    let mut colors = vec!["Red", "Green", "Yellow", "Blue", "Purple", "Cyan"];
    if bw {
        colors.insert(0, "Black");
        colors.push("White");
    }
    printer(colors, vec).unwrap()
}

pub fn crunch(bw: bool) -> String {
    let vec = vec![
        "   ██  ██   ",
        " ██████████ ",
        "   ██████   ",
        " ██████████ ",
        "   ██  ██   ",
    ];
    let mut colors = vec!["Red", "Green", "Yellow", "Blue", "Purple", "Cyan"];
    if bw {
        colors.insert(0, "Black");
        colors.push("White");
    }
    printer(colors, vec).unwrap()
}

pub fn arch(bw: bool) -> String {
    let vec = vec![
        "        ■      ",
        "       ■■■     ",
        "      ■■■■■    ",
        "     ■(   )■   ",
        "    ■■■■ ■■■■  ",
        "   ■■       ■■ ",
    ];
    let mut colors = vec!["Red", "Green", "Yellow", "Blue", "Purple", "Cyan"];
    if bw {
        colors.insert(0, "Black");
        colors.push("White");
    }
    printer(colors, vec).unwrap()
}

pub fn crunchbang(bw: bool) -> String {
    let vec = vec![
        "   ██  ██   ██ ",
        " ██████████ ██ ",
        "   ██████   ██ ",
        " ██████████    ",
        "   ██  ██   ██ ",
    ];
    let mut colors = vec!["Red", "Green", "Yellow", "Blue", "Purple", "Cyan"];
    if bw {
        colors.insert(0, "Black");
        colors.push("White");
    }
    printer(colors, vec).unwrap()
}
