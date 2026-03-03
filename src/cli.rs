use clap::Parser;
use std::path::PathBuf;

/// 热点新闻聚合器 - DDD 学习项目
#[derive(Parser, Debug)]
#[command(name = "trendarc")]
#[command(author = "TrendArc Team")]
#[command(version = "0.1.0")]
#[command(about = "从多个数据源抓取并聚合热点新闻", long_about = None)]
pub struct Cli {
    /// 是否保存到数据库
    #[arg(short, long, action)]
    pub save: bool,

    /// 是否从数据库加载（而不是抓取）
    #[arg(short, long, action)]
    pub load: bool,

    /// 新闻数量限制
    #[arg(short, long, default_value_t = 10)]
    pub limit: usize,

    /// 数据库文件路径
    #[arg(long, default_value = "trendarc.db")]
    pub database: String,

    /// 指定领域过滤（ai, block, social）
    #[arg(short = 'd', long)]
    pub domain: Option<String>,

    /// 显示统计信息
    #[arg(long, action)]
    pub stats: bool,
}

impl Cli {
    /// 解析命令行参数
    pub fn parse_args() -> Self {
        Cli::parse()
    }

    /// 验证参数
    pub fn validate(&self) -> Result<(), String> {
        // 不能同时使用 --save 和 --load
        if self.save && self.load {
            return Err("不能同时使用 --save 和 --load".to_string());
        }

        // 验证 domain 参数
        if let Some(ref domain) = self.domain {
            let valid_domains = vec!["ai", "block", "social"];
            if !valid_domains.contains(&domain.to_lowercase().as_str()) {
                return Err(format!(
                    "无效的领域: {}. 有效值: ai, block, social",
                    domain
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_success() {
        let cli = Cli {
            save: false,
            load: false,
            limit: 10,
            database: "test.db".to_string(),
            domain: None,
            stats: false,
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_save_and_load_conflict() {
        let cli = Cli {
            save: true,
            load: true,
            limit: 10,
            database: "test.db".to_string(),
            domain: None,
            stats: false,
        };
        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_domain() {
        let cli = Cli {
            save: false,
            load: false,
            limit: 10,
            database: "test.db".to_string(),
            domain: Some("invalid".to_string()),
            stats: false,
        };
        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_validate_valid_domain() {
        let cli = Cli {
            save: false,
            load: false,
            limit: 10,
            database: "test.db".to_string(),
            domain: Some("ai".to_string()),
            stats: false,
        };
        assert!(cli.validate().is_ok());
    }
}