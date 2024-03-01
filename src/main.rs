mod init;
mod cli;
mod cat_file;
mod hash_object;

fn main() {
    cli::parse()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        
    }
}