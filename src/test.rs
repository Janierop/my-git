#[cfg(test)]
mod test {
    use std::process::Command;

    const GIT: &str = "git";
    const MYGIT_PATH: &str = "/home/jaden/projects/codecrafters-git-rust/your_git.sh";
    const REPO_PATH: &str = "./test-repo/";
    
    #[test]
    fn test_git_cat_file_print() {
        let object = "f0a4aabb9a7241e55224250a6650f12d597f7912"; // Or any object hash that you want to test

        let git_output = Command::new(GIT)
            .args(["-C",REPO_PATH,"cat-file","-p",object])
            .output()
            .expect("Failed to execute command");

        let my_git_output = Command::new(MYGIT_PATH)
            .args(["--","-C",REPO_PATH,"cat-file","-p",object])
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
            .args(["-C",REPO_PATH,"cat-file", "-s", object])
            .output()
            .expect("Failed to execute command");

        let my_git_output = Command::new(MYGIT_PATH)
            .args(["--","-C",REPO_PATH,"cat-file","-s",object])
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
            .args(["-C",REPO_PATH,"cat-file", "-t", object])
            .output()
            .expect("Failed to execute command");

        let my_git_output = Command::new(MYGIT_PATH)
            .args(["--","-C",REPO_PATH,"cat-file","-t",object])
            .output()
            .expect("Failed to execute command");
    
        // Compare the outputs
        assert_eq!(my_git_output.stdout, git_output.stdout, "Outputs do not match");
        assert_eq!(my_git_output.stderr, git_output.stderr, "Errors do not match");
        assert_eq!(my_git_output.status.success(), git_output.status.success(), "Status codes do not match");
    }

    #[test]
    fn test_git_hash_object() {
        let file = "./deleteme"; // Or any object hash that you want to test

        let git_output = Command::new(GIT)
            .args(["-C",REPO_PATH,"hash-object", file])
            .output()
            .expect("Failed to execute command");

        let my_git_output = Command::new(MYGIT_PATH)
            .args(["--","-C",REPO_PATH,"hash-object", file])
            .output()
            .expect("Failed to execute command");

        dbg!(String::from_utf8_lossy(&git_output.stdout[..]));
    
        // Compare the outputs
        assert_eq!(my_git_output.stdout, git_output.stdout, "Outputs do not match");
        assert_eq!(my_git_output.stderr, git_output.stderr, "Errors do not match");
        assert_eq!(my_git_output.status.success(), git_output.status.success(), "Status codes do not match");
    }

}
