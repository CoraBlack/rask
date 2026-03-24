pub fn error_and_exit(msg: String) -> ! {
    std::eprintln!("{}", msg);
    std::process::exit(-1);
}
