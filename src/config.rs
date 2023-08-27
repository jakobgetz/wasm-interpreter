pub struct Config {
    pub binary_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() > 2 {
            return Err("Too many params");
        }
        if args.len() < 2 {
            return Err("Not enough params");
        }
        let binary_path = args[1].clone();
        Ok(Self { binary_path })
    }
}