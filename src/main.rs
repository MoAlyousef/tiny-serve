struct Opts<'a> {
    port: u16,
    root: &'a str,
}

impl<'a> Default for Opts<'a> {
    fn default() -> Self {
        Self {
            port: 8000,
            root: ".",
        }
    }
}

impl<'a> Opts<'a> {
    pub fn new(args: &'a [String]) -> Self {
        let port = args[0].parse().unwrap_or(8000);
        let root = args[1].as_str();
        Self {
            port, root,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = if args.is_empty() {
        Opts::default()
    } else {
        Opts::new(&args)
    };
    eprintln!("Serving HTTP on 127.0.0.1 port {0} (http://127.0.0.1:{0}/) ...", opts.port);
    let mut server = livid_server::Server::new(opts.port);
    server.serve_dir(&opts.root.to_string());
    server.log_output(Box::new(std::io::stdout()));
    server.serve();
}