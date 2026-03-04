use crate::domain::services::{InferenceResult, NewsInferenceService};
use crate::domain::{Domain, NewsItem};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tracing::{debug, error, info};

/// Ollama 请求结构
#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    format: String, // 强制生成 JSON
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// Ollama 响应结构
#[derive(Deserialize)]
struct OllamaChatResponse {
    message: ChatMessageResponse,
}

#[derive(Deserialize)]
struct ChatMessageResponse {
    content: String,
}

/// AI 返回的 JSON 格式定义
#[derive(Deserialize)]
struct AIClassification {
    is_relevant: bool,
    domain: Option<String>,
    confidence: f32,
    reason: Option<String>,
    suggested_keywords: Vec<String>,
}

pub struct OllamaInferenceService {
    base_url: String,
    model_name: String,
    client: reqwest::Client,
}

impl OllamaInferenceService {
    /// 创建服务实例。
    ///
    /// 优先级（高→低）：
    /// 1. 环境变量 `OLLAMA_BASE_URL` / `OLLAMA_MODEL`
    /// 2. 传入的 `model_name` 参数（作为模型的代码默认值）
    pub fn new(model_name: &str) -> Self {
        let base_url = std::env::var("OLLAMA_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:11434/api/chat".to_string());
        let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| model_name.to_string());

        Self {
            base_url,
            model_name: model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NewsInferenceService for OllamaInferenceService {
    async fn infer(
        &self,
        news: &NewsItem,
    ) -> Result<InferenceResult, Box<dyn Error + Send + Sync>> {
        let content_preview = news.content.as_deref().unwrap_or("No content provided.");
        let limit = 2000;
        let truncated_content = if content_preview.len() > limit {
            let mut end = limit;
            while !content_preview.is_char_boundary(end) && end > 0 {
                end -= 1;
            }
            &content_preview[..end]
        } else {
            content_preview
        };

        let sys_prompt = r#"You are a professional news classifier. 
Your goal is to determine if a news item is related to our target focus: AI, Blockchain, or Social Platforms.

AVAILABLE DOMAINS:
- AI: Artificial intelligence, LLMs, Neural Networks, Robotics, etc.
- Block: Cryptocurrency, Web3, DeFi, Smart Contracts, etc.
- Social: Social Media platforms (Twitter/X, Meta, Tiktok, etc.), tech platform news.

OUTPUT FORMAT (JSON):
{
  "is_relevant": boolean,
  "domain": "AI" | "Block" | "Social",
  "confidence": float (0.0-1.0),
  "reason": "short explanation in Chinese",
  "suggested_keywords": ["extracted_keyword1", "extracted_keyword2"]
}
"#;

        let user_input = format!(
            "Title: {}\nSource: {}\nContent Snippet: {}\n",
            news.title, news.source, truncated_content
        );

        let request = OllamaChatRequest {
            model: self.model_name.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: sys_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_input.clone(),
                },
            ],
            stream: false,
            format: "json".to_string(),
        };

        debug!("Sending request to Ollama with input:\n{}", user_input);
        let response = self
            .client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let err_body = response.text().await?;
            error!("Ollama error response: {}", err_body);
            return Err(format!("Ollama API returned error: {}", err_body).into());
        }

        let body: OllamaChatResponse = response.json().await?;
        info!("🤖 AI Raw Response: {}", body.message.content);
        let ai_result: AIClassification = serde_json::from_str(&body.message.content)?;

        let final_domain = match ai_result.domain.as_deref() {
            Some("AI") => Some(Domain::AI),
            Some("Block") => Some(Domain::Block),
            Some("Social") => Some(Domain::Social),
            _ => None,
        };

        Ok(InferenceResult {
            is_relevant: ai_result.is_relevant && final_domain.is_some(),
            domain: final_domain,
            confidence: ai_result.confidence,
            reason: ai_result.reason.unwrap_or_else(|| "No reason provided".to_string()),
            suggested_keywords: ai_result.suggested_keywords,
        })
    }

    fn name(&self) -> &str {
        &self.model_name
    }
}
