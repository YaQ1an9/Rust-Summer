use std::env;
use std::process;
use regex::Regex;
mod Find;
mod Help;
use crate::Help::use_help;
use crate::Find::find;
use crate::Find::Mode;
use colored::*;

pub struct Config {
    pub mode: Mode,
    pub root: String,
    pub pattern: Vec<String>,// 用Vec以处理多个正则式与多个Path
    pub regex: Vec<Regex>,   // Vec<Regex> 与 Vec<&Regex> ?
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode = Mode::Normal;

    if args.len() < 3 {
        eprintln!("参数过少");
        use_help();
    } else if args.len() == 3 {
        mode = Mode::Normal;
    } else if args.len() == 4 {
        if args[1] != "-v" && args[1] != "--verbose" {
            eprintln!("参数错误: {}", args[1]);
            use_help();
        }
        mode = Mode::Verbose;
    } else if args.len() > 4 {
        eprintln!("参数过多");
        use_help();
    }

    let pattern = &args[args.len() - 1];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("正则表达式错误: '{}': {}", pattern, err);
            process::exit(1);
        }
    };
    match find(&args[args.len() - 2], &regex, &mode) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("{}", "未找到匹配项".to_string().red());
            } else {
                println!("{}", "匹配项".to_string().bright_blue());
                for file in matches {
                    println!("{}", file.green());
                }
            }
        }
        Err(error) => {
            eprintln!("应用程序错误: {}", error);
            process::exit(1);
        }
    }
}
/* 处理命令行参数
 * 使得 "mini_Find -v <目标目录> <要搜索的正则表达式>" 
 * 与 "mini_Find <目标目录> -v <要搜索的正则表达式>" 两种命令行参数都能够被正确解析
 * 但是这个函数还没有实现 QaQ
 */
// fn parse_config(args: &[String]) -> Config {
//     let config = Config {
//         mode: Mode::Normal,
//         root: String::from(""),
//         pattern: Vec::new(),
//         regex: Vec::new(),
//     };
//     for arg in args {
//         if arg.starts_with("-") {
//             if arg == "-v" || arg == "--verbose" {
//                 config.mode = Mode::Verbose;
//             } else if arg == "-h" || arg == "--help" {
//                 use_help();
//             }
//         }
//     }
//     config
// }
