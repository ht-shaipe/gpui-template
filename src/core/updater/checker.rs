//! copyright © htui.tech 2026 - present
//! 更新检查器
//! created shaipe by 2026-03-01 09:09:45

use super::version::Version;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;

/// 单例 tokio 运行时句柄
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

/// 如果当前没有 tokio 运行时句柄，则创建一个
fn tokio_handle() -> tokio::runtime::Handle {
    tokio::runtime::Handle::try_current().unwrap_or_else(|_| {
        RUNTIME
            .get_or_init(|| {
                tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(1)
                    .enable_all()
                    .build()
                    .expect("Failed to create tokio runtime for update checker")
            })
            .handle()
            .clone()
    })
}

/// 更新信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    /// 版本号
    pub version: String,
    /// 下载 URL
    pub download_url: String,
    /// 发布说明
    pub release_notes: String,
    /// 发布时间
    pub published_at: String,
    /// 文件大小
    pub file_size: Option<u64>,
}

/// 更新检查结果
#[derive(Debug, Clone)]
pub enum UpdateCheckResult {
    /// 没有更新
    NoUpdate,
    /// 有更新
    UpdateAvailable(UpdateInfo),
    /// 错误
    Error(String),
}

/// 更新检查器
#[derive(Clone)]
pub struct UpdateChecker {
    /// 检查 URL
    check_url: String,
    /// 超时时间
    timeout: Duration,
}

impl UpdateChecker {
    /// 创建新的更新检查器
    pub fn new() -> Self {
        Self {
            check_url: "https://api.github.com/repos/YOUR_USERNAME/{{crate_name}}/releases/latest"
                .to_string(),
            timeout: Duration::from_secs(10),
        }
    }

    /// Safe to call from any async executor (GPUI, tokio, etc.).
    pub async fn check_for_updates(&self) -> UpdateCheckResult {
        let check_url = self.check_url.clone();
        let timeout = self.timeout;

        // 异步检查最新版本
        let fetch_result = tokio_handle()
            .spawn(async move { fetch_latest_release(&check_url, timeout).await })
            .await;

        // 匹配检查结果
        let info = match fetch_result {
            Ok(Ok(info)) => info,
            Ok(Err(e)) => {
                log::error!("Failed to check for updates: {}", e);
                return UpdateCheckResult::Error(e.to_string());
            }
            Err(e) => {
                log::error!("Update check task failed: {}", e);
                return UpdateCheckResult::Error(e.to_string());
            }
        };

        // 当前版本
        let current = Version::current();
        // 匹配版本
        match Version::parse(&info.version) {
            Ok(latest) if latest.is_newer_than(&current) => {
                log::info!("Update available: {} -> {}", current, latest);
                UpdateCheckResult::UpdateAvailable(info)
            }
            Ok(_) => {
                log::info!("No update available (current: {})", current);
                UpdateCheckResult::NoUpdate
            }
            Err(e) => {
                log::error!("Failed to parse remote version: {}", e);
                UpdateCheckResult::Error(format!("Invalid version format: {}", e))
            }
        }
    }
}

impl Default for UpdateChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// 异步获取最新版本
async fn fetch_latest_release(check_url: &str, timeout: Duration) -> Result<UpdateInfo> {
    log::info!("Fetching latest release from: {}", check_url);

    // 创建请求客户端
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .user_agent(format!("AgentStudio/{}", env!("CARGO_PKG_VERSION")))
        .build()?;

    // 发送请求
    let response = client
        .get(check_url)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("GitHub API returned status: {}", response.status()));
    }

    let body = response.text().await?;
    let release: GitHubRelease = serde_json::from_str(&body)?;
    let download_url = find_platform_asset(&release.assets);

    Ok(UpdateInfo {
        version: release.tag_name,
        download_url,
        release_notes: release.body.unwrap_or_default(),
        published_at: release.published_at,
        file_size: release.assets.first().map(|a| a.size),
    })
}

/// 查找平台资产
fn find_platform_asset(assets: &[GitHubAsset]) -> String {
    let patterns: &[&str] = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => &["aarch64-apple-darwin", "arm64-macos", "darwin-arm64"],
        ("macos", "x86_64") => &["x86_64-apple-darwin", "x64-macos", "darwin-x64"],
        ("windows", "x86_64") => &["x86_64-pc-windows", "win64", "windows-x64"],
        ("windows", "aarch64") => &["aarch64-pc-windows", "win-arm64", "windows-arm64"],
        ("linux", "x86_64") => &["x86_64-unknown-linux", "linux-x64", "linux64"],
        ("linux", "aarch64") => &["aarch64-unknown-linux", "linux-arm64"],
        _ => &[],
    };

    // 遍历模式
    for pattern in patterns {
        // 查找资产
        if let Some(asset) = assets.iter().find(|a| {
            a.name.to_lowercase().contains(pattern)
                || a.browser_download_url.to_lowercase().contains(pattern)
        }) {
            // 返回下载 URL
            return asset.browser_download_url.clone();
        }
    }

    // 返回第一个资产的下载 URL
    assets
        .first()
        .map(|a| a.browser_download_url.clone())
        .unwrap_or_default()
}

/// GitHub 发布
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    /// 标签名
    tag_name: String,
    /// 发布说明
    body: Option<String>,
    /// 发布时间
    published_at: String,
    /// 资产
    assets: Vec<GitHubAsset>,
}

/// GitHub 资产
#[derive(Debug, Deserialize)]
struct GitHubAsset {
    /// 名称
    name: String,
    /// 下载 URL
    browser_download_url: String,
    /// 文件大小
    size: u64,
}
