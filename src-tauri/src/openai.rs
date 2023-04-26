use anyhow::{anyhow, Result};
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::result::Result::Ok;

use crate::entry::{self, Meanings};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatGPTQuery {
    model: String,
    messages: Vec<Message>,
}

fn escape(src: &str) -> String {
    use std::fmt::Write;
    let mut escaped = String::with_capacity(src.len());
    for c in src.chars() {
        match c {
            '\x08' => escaped += "\\b",
            '\x0c' => escaped += "\\f",
            '\n' => escaped += "\\n",
            '\r' => escaped += "\\r",
            '\t' => escaped += "\\t",
            '"' => escaped += "\\\"",
            '\\' => escaped += "\\",
            _ => escaped.push(c),
        }
    }

    escaped
}

fn sample_chinese_query() -> String {
    let e = entry::Entry {
        query: "sentence".to_string(),
        meanings: vec![entry::Meaning {
            pos: "noun".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("A set of words that is complete in itself, typically containing a subject and predicate, conveying a statement, question, exclamation, or command, and consisting of a main clause and sometimes one or more subordinate clauses.".to_string()),
                entry::Lang::Chinese("一组单词".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("The teacher ask each student to make a sentence.".to_string()),
                entry::Lang::Chinese("老师让每个学生造一个句子。".to_string()),
            ]]},
            entry::Item {
                meaning: vec![
                    entry::Lang::English("The punishment assigned to a defendant found guilty by a court, or fixed by law for a particular offense.".to_string()),
                    entry::Lang::Chinese("法庭针对有罪的被告的惩罚。或是依法规定的惩罚。".to_string())
                ],
                examples: vec![vec![
                    entry::Lang::English("Her husband is serving a three-year sentence for fraud.".to_string()),
                    entry::Lang::Chinese("她丈夫增在为三年刑期服刑。".to_string()),
                ]]
            }]
        },
        entry::Meaning {
            pos: "verb".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("Declare the punishment decided for (an offender).".to_string()),
                entry::Lang::Chinese("对罪犯进行判决。".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                entry::Lang::Chinese("十位军官被判处死刑。".to_string()),
            ]]}]
        }],
    };

    let res = serde_json::to_string(&e.meanings).unwrap();

    res
}

fn assemble_query(query: &str) -> String {
    let query = ChatGPTQuery {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
                role: "system".to_string(),
                content: "You are a dictionary bot. Given a query, reply its meaning and sample sentences in English and Chinese in JSON format.".to_string()
        },
        Message {
            role: "user".to_string(),
            content: "sentence".to_string()
          },
         Message        {
            role: "assistant".to_string(),
            content:sample_chinese_query()
        } ,
        Message     {
            role: "user".to_string(),
            content: query.to_string()
          }],
    };
    let res = serde_json::to_string(&query).unwrap();

    res
}

pub async fn search(query: &str, auth_token: &str) -> Result<entry::Entry> {
    let bearer_auth = format!("Bearer {}", auth_token);

    let data = assemble_query(query);

    println!("{}", data);

    let url = "https://api.openai.com/v1/chat/completions".to_string();
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(ACCEPT, "*/*")
        .header(AUTHORIZATION, &bearer_auth)
        .header(CONTENT_TYPE, "application/json")
        .body(data)
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Root>().await {
                Ok(parsed) => {
                    println!("🔥 Success!");
                    println!("💬 Response: {}", parsed.choices[0].message.content);

                    match serde_json::from_str(&parsed.choices[0].message.content) {
                        Ok(meanings) => {
                            println!("{:?}", meanings);
                            return Ok(entry::Entry {
                                query: query.to_string(),
                                meanings: meanings,
                            });
                        }
                        Err(message) => {
                            return Err(anyhow!(format!(
                                "{} : {}",
                                message.to_string(),
                                parsed.choices[0].message.content
                            )));
                        }
                    }
                }
                Err(_) => {
                    println!("🛑 Hm, the response didn't match the shape we expected.");
                    return Err(anyhow!(
                        "🛑 Hm, the response didn't match the shape we expected."
                    ));
                }
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("🛑 Status: UNAUTHORIZED - Need to grab a new token");
            return Err(anyhow!("Status: UNAUTHORIZED - Need to grab a new token"));
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            println!("🛑 Status: 429 - Too many requests");
            return Err(anyhow!("Status: 429 - Too many requests, this may happend if your API token was generated not too long ago. Please try again later."));
        }
        other => {
            return Err(anyhow!(format!(
                "🛑 Uh oh! Something unexpected happened: [{:#?} {:?}]",
                other,
                response.text().await
            )));
        }
    };
}
