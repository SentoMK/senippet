// src/commands/list.rs
use crate::storage::load_prompts;
use clap::Parser;
use colored::*;
use prettytable::{color, format, Attr, Cell, Row, Table};

#[derive(Parser, Debug)]
#[clap(about = "List all prompts")]
pub struct ListArgs {}

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let prompts = load_prompts();

    if prompts.is_empty() {
        println!("{}", "No prompts found.".yellow());
        return Ok(());
    }

    // 创建一个表格
    let mut table = Table::new();
    // 定义表格的格式
    let format = format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('─', '┼', '├', '┤'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);
    // 添加表头
    table.add_row(Row::new(vec![
        Cell::new("ID").with_style(Attr::Bold),
        Cell::new("Name").with_style(Attr::Bold),
        Cell::new("Tags").with_style(Attr::Bold),
    ]));
    // 添加数据行
    for prompt in prompts {
        let tags_str = prompt.tags.join(", ");
        table.add_row(Row::new(vec![
            Cell::new(&prompt.short_id).with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new(&prompt.title),
            Cell::new(&tags_str),
        ]));
    }
    // 打印表格
    table.printstd();

    Ok(())
}
