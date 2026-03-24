pub fn error_and_exit(msg: String) -> ! {
    std::eprintln!("{}", msg);
    std::process::exit(-1);
}

pub fn get_arg() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.is_empty() || args.len() < 2 {
        return None;
    }

    let arg = args[1].clone();
    Some(arg.clone())
}
