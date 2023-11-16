fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: tiny-serve PORT DIR");
        return;
    }
    let mut server = livid_server::Server::new(args[1].parse().unwrap());
    server.serve_dir(&args[2]);
    server.serve();
}