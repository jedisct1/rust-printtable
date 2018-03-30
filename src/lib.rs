use std::cmp;
use std::io::{self, Write};
use std::iter::IntoIterator;
use std::marker::Sized;

fn prepare_line<U, V>(line: U, col_widths: &mut Vec<usize>, cell_strs: &mut Vec<String>)
where
    U: IntoIterator<Item = V>,
    V: ToString + Sized,
{
    for (i, cell) in line.into_iter().enumerate() {
        if i >= col_widths.len() {
            col_widths.resize(i + 1, 0);
        }
        let cell_str = cell.to_string();
        col_widths[i] = cmp::max(col_widths[i], cell_str.len());
        cell_strs.push(cell_str);
    }
}

pub fn write<H, I, T, U, V, W>(mut writer: W, header: H, mat: T) -> Result<(), io::Error>
where
    H: IntoIterator<Item = I>,
    I: ToString + Sized,
    T: IntoIterator<Item = U>,
    U: IntoIterator<Item = V>,
    V: ToString + Sized,
    W: Write,
{
    let mut col_widths = vec![];
    let mut cell_strs = vec![];
    prepare_line(header, &mut col_widths, &mut cell_strs);
    for line in mat {
        prepare_line(line, &mut col_widths, &mut cell_strs);
    }
    let mut col = 0;
    let width = col_widths.len();
    let mut in_header = true;
    for cell_str in &cell_strs {
        writer.write_all(format!("{}", cell_str).as_bytes())?;
        for _ in cell_str.len()..col_widths[col] {
            writer.write_all(b" ")?;
        }
        col += 1;
        if col == width {
            col = 0;
            writer.write_all(b"\n")?;
            if in_header {
                for (i, col_width) in (&col_widths).iter().enumerate() {
                    for _ in 0..*col_width {
                        writer.write_all(b"-")?;
                    }
                    if i < width - 1 {
                        writer.write_all(b"-+-")?;
                    }
                }
                writer.write_all(b"\n")?;
                in_header = false
            }
        } else {
            writer.write_all(b" | ")?;
        }
    }
    Ok(())
}
