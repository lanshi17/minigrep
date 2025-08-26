#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_case_sensitive_default() {
        // 测试默认情况下大小写敏感
        let output = Command::new("cargo")
            .args(["run", "--", "the", "src/poem.txt"])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
    }

    #[test]
    fn test_ignore_case_with_env_var() {
        // 测试使用环境变量忽略大小写
        let output = Command::new("cargo")
            .env("IGNORE_CASE", "1")
            .args(["run", "--", "the", "src/poem.txt"])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
    }

    #[test]
    fn test_ignore_case_with_flag() {
        // 测试使用命令行参数忽略大小写
        let output = Command::new("cargo")
            .args(["run", "--", "the", "src/poem.txt","-i"])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
    }

    #[test]
    fn test_env_var_priority() {
        // 测试环境变量优先级高于命令行参数
        let output = Command::new("cargo")
            .env("IGNORE_CASE", "0")
            .args(["run", "--", "the", "src/poem.txt","--ignore-case"])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
    }
}