use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const LEETCODE_GRAPHQL_URL: &str = "https://leetcode.com/graphql";

#[derive(Debug, Serialize)]
struct GraphQLQuery {
    query: String,
    variables: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct DailyChallenge {
    pub date: String,
    pub link: String,
    pub question: Question,
}

#[derive(Debug, Deserialize)]
pub struct TopicTag {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct CodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    pub title: String,
    pub difficulty: String,
    pub content: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    #[serde(rename = "topicTags")]
    pub topic_tags: Vec<TopicTag>,
    #[serde(rename = "questionFrontendId")]
    pub id: String,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippet>,
}

pub async fn fetch_daily_challenge() -> Result<DailyChallenge, Box<dyn std::error::Error>> {
    let query = r#"
    query questionOfToday {
        activeDailyCodingChallengeQuestion {
            date
            link
            question {
                title
                titleSlug
                difficulty
                content
                questionFrontendId
                topicTags {
                    name
                    slug
                }
                codeSnippets {
                    lang
                    langSlug
                    code
                }
            }
        }
    }"#;

    let client = reqwest::Client::new();
    let response = client
        .post(LEETCODE_GRAPHQL_URL)
        .json(&GraphQLQuery {
            query: query.to_string(),
            variables: None,
        })
        .send()
        .await?;

    let data: Value = response.json().await?;
    let challenge = data["data"]["activeDailyCodingChallengeQuestion"]
        .as_object()
        .ok_or("Invalid response format")?;

    Ok(serde_json::from_value(serde_json::to_value(challenge)?)?)
}
