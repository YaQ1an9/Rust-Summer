use std::process;

pub fn use_help() {
    eprintln!("使用方式: mini_Find <目标目录> <要搜索的正则表达式>");
    eprintln!("或者: mini_Find -v/--verbose <目标目录> <要搜索的正则表达式>");
    process::exit(1);
}