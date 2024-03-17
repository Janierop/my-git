#[cfg(test)]
mod test {
    use std::process::Command;

    const GIT: &str = "git";
    const MYGIT_PATH: &str = "/home/jaden/projects/codecrafters-git-rust/your_git.sh";
    
    #[test]
    fn test_git_cat_file_print() {
        let object = "f0a4aabb9a7241e55224250a6650f12d597f7912"; // Or any object hash that you want to test

        let git_output = Command::new(GIT)
            .args(["cat-file", "-p",object])
            .output()
            .expect("Failed to execute command");

        let my_git_output = Command::new(MYGIT_PATH)
            .args(["cat-file","-p",object])
            .output()
            .expect("Failed to execute command");
    
        // Compare the outputs
        assert_eq!(my_git_output.stdout, git_output.stdout, "Outputs do not match");
        assert_eq!(my_git_output.stderr, git_output.stderr, "Errors do not match");
        assert_eq!(my_git_output.status.success(), git_output.status.success(), "Status codes do not match");
    }

    #[test]
    fn test_git_cat_file_size() {
        let object = "f0a4aabb9a7241e55224250a6650f12d597f7912"; // Or any object hash that you want to test

        let git_output = Command::new(GIT)
            .args(["cat-file", "-s", object])
            .output()
            .expect("Failed to execute command");

        let my_git_output = Command::new(MYGIT_PATH)
            .args(["cat-file","-s",object])
            .output()
            .expect("Failed to execute command");
    
        // Compare the outputs
        assert_eq!(my_git_output.stdout, git_output.stdout, "Outputs do not match");
        assert_eq!(my_git_output.stderr, git_output.stderr, "Errors do not match");
        assert_eq!(my_git_output.status.success(), git_output.status.success(), "Status codes do not match");
    }

    #[test]
    fn test_git_cat_file_type() {
        let object = "f0a4aabb9a7241e55224250a6650f12d597f7912"; // Or any object hash that you want to test

        let git_output = Command::new(GIT)
            .args(["cat-file", "-t", object])
            .output()
            .expect("Failed to execute command");

        let my_git_output = Command::new(MYGIT_PATH)
            .args(["cat-file","-t",object])
            .output()
            .expect("Failed to execute command");
    
        // Compare the outputs
        assert_eq!(my_git_output.stdout, git_output.stdout, "Outputs do not match");
        assert_eq!(my_git_output.stderr, git_output.stderr, "Errors do not match");
        assert_eq!(my_git_output.status.success(), git_output.status.success(), "Status codes do not match");
    }
}
