use anyhow::{anyhow, Result};
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use tauri::utils::config::parse;
use std::result::Result::Ok;

use crate::entry::{self, Meanings};
use crate::state;

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
                entry::Lang::Chinese("一组本身完整的词，通常包含主语和谓语，传达陈述、问题、感叹或命令，并由主句和有时一个或多个从句组成。".to_string()),
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

fn sample_spanish_query() -> String {
    let e = entry::Entry {
        query: "sentence".to_string(),
        meanings: vec![entry::Meaning {
            pos: "noun".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("A set of words that is complete in itself, typically containing a subject and predicate, conveying a statement, question, exclamation, or command, and consisting of a main clause and sometimes one or more subordinate clauses.".to_string()),
                entry::Lang::Spanish("Un conjunto de palabras completo en sí mismo, que normalmente contiene un sujeto y un predicado, transmite una declaración, pregunta, exclamación o comando, y consta de una cláusula principal y, a veces, una o más cláusulas subordinadas.".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("The teacher ask each student to make a sentence.".to_string()),
                entry::Lang::Spanish("El profesor pide a cada estudiante que haga una oración.".to_string()),
            ]]},
            entry::Item {
                meaning: vec![
                    entry::Lang::English("The punishment assigned to a defendant found guilty by a court, or fixed by law for a particular offense.".to_string()),
                    entry::Lang::Spanish("El castigo asignado a un acusado declarado culpable por un tribunal, o fijado por la ley por un delito en particular.".to_string())
                ],
                examples: vec![vec![
                    entry::Lang::English("Her husband is serving a three-year sentence for fraud.".to_string()),
                    entry::Lang::Chinese("Su marido cumple una condena de tres años por fraude.".to_string()),
                ]]
            }]
        },
        entry::Meaning {
            pos: "verb".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("Declare the punishment decided for (an offender).".to_string()),
                entry::Lang::Chinese("Declarar la pena decidida para (un infractor).".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                entry::Lang::Chinese("Diez oficiales del ejército fueron condenados a muerte.".to_string()),
            ]]}]
        }],
    };

    let res = serde_json::to_string(&e.meanings).unwrap();

    res
}

fn sample_japanese_query() -> String {
    let e = entry::Entry {
        query: "sentence".to_string(),
        meanings: vec![entry::Meaning {
            pos: "noun".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("A set of words that is complete in itself, typically containing a subject and predicate, conveying a statement, question, exclamation, or command, and consisting of a main clause and sometimes one or more subordinate clauses.".to_string()),
                entry::Lang::Japanese("それ自体で完全な単語のセットで、通常は主語と述語を含み、ステートメント、質問、感嘆符、または命令を伝え、主節と場合によっては 1 つまたは複数の従属節で構成されます。".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("The teacher ask each student to make a sentence.".to_string()),
                entry::Lang::Japanese("教師は各生徒に文を作るように求めます。".to_string()),
            ]]},
            entry::Item {
                meaning: vec![
                    entry::Lang::English("The punishment assigned to a defendant found guilty by a court, or fixed by law for a particular offense.".to_string()),
                    entry::Lang::Japanese("裁判所によって有罪とされた、または特定の犯罪に対して法律によって定められた被告に割り当てられる刑罰。".to_string())
                ],
                examples: vec![vec![
                    entry::Lang::English("Her husband is serving a three-year sentence for fraud.".to_string()),
                    entry::Lang::Japanese("彼女の夫は、詐欺罪で 3 年の刑に服しています。".to_string()),
                ]]
            }]
        },
        entry::Meaning {
            pos: "verb".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("Declare the punishment decided for (an offender).".to_string()),
                entry::Lang::Japanese("（犯罪者）に対して決定された処罰を宣言します。".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                entry::Lang::Japanese("陸軍将校10人が死刑を宣告された。".to_string()),
            ]]}]
        }],
    };

    let res = serde_json::to_string(&e.meanings).unwrap();

    res
}

fn sample_korean_query() -> String {
    let e = entry::Entry {
        query: "sentence".to_string(),
        meanings: vec![entry::Meaning {
            pos: "noun".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("A set of words that is complete in itself, typically containing a subject and predicate, conveying a statement, question, exclamation, or command, and consisting of a main clause and sometimes one or more subordinate clauses.".to_string()),
                entry::Lang::Korean("일반적으로 주어와 술어를 포함하고 진술, 질문, 느낌표 또는 명령을 전달하고 주절과 때때로 하나 이상의 종속절로 구성되는 그 자체로 완전한 단어 집합입니다.".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("The teacher ask each student to make a sentence.".to_string()),
                entry::Lang::Korean("교사는 각 학생에게 문장을 만들라고 합니다.".to_string()),
            ]]},
            entry::Item {
                meaning: vec![
                    entry::Lang::English("The punishment assigned to a defendant found guilty by a court, or fixed by law for a particular offense.".to_string()),
                    entry::Lang::Korean("법원에서 유죄 판결을 받은 피고인에게 부과되는 형벌 또는 특정 범죄에 대해 법률에 의해 정해진 형벌.".to_string())
                ],
                examples: vec![vec![
                    entry::Lang::English("Her husband is serving a three-year sentence for fraud.".to_string()),
                    entry::Lang::Korean("그녀의 남편은 사기죄로 3년 형을 선고받고 복역하고 있습니다.".to_string()),
                ]]
            }]
        },
        entry::Meaning {
            pos: "verb".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("Declare the punishment decided for (an offender).".to_string()),
                entry::Lang::Korean("(가해자)에 대해 결정된 처벌을 선언합니다.".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                entry::Lang::Korean("육군 장교 10명이 사형을 선고받았다.".to_string()),
            ]]}]
        }],
    };

    let res = serde_json::to_string(&e.meanings).unwrap();

    res
}

fn sample_german_query() -> String {
    let e = entry::Entry {
        query: "sentence".to_string(),
        meanings: vec![entry::Meaning {
            pos: "noun".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("A set of words that is complete in itself, typically containing a subject and predicate, conveying a statement, question, exclamation, or command, and consisting of a main clause and sometimes one or more subordinate clauses.".to_string()),
                entry::Lang::German("Eine in sich abgeschlossene Wortgruppe, die typischerweise ein Subjekt und ein Prädikat enthält, eine Aussage, Frage, einen Ausruf oder einen Befehl übermittelt und aus einem Hauptsatz und manchmal einem oder mehreren Nebensätzen besteht.".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("The teacher ask each student to make a sentence.".to_string()),
                entry::Lang::German("Die Lehrerin bittet jeden Schüler, einen Satz zu bilden.".to_string()),
            ]]},
            entry::Item {
                meaning: vec![
                    entry::Lang::English("The punishment assigned to a defendant found guilty by a court, or fixed by law for a particular offense.".to_string()),
                    entry::Lang::German("Die Strafe, die einem Angeklagten von einem Gericht für schuldig befunden oder gesetzlich für ein bestimmtes Vergehen festgelegt wurde.".to_string())
                ],
                examples: vec![vec![
                    entry::Lang::English("Her husband is serving a three-year sentence for fraud.".to_string()),
                    entry::Lang::German("Ihr Ehemann verbüßt ​​eine dreijährige Haftstrafe wegen Betrugs.".to_string()),
                ]]
            }]
        },
        entry::Meaning {
            pos: "verb".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("Declare the punishment decided for (an offender).".to_string()),
                entry::Lang::German("Deklarieren Sie die Strafe, die für (einen Täter) entschieden wurde.".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                entry::Lang::German("Zehn Offiziere der Armee wurden zum Tode verurteilt.".to_string()),
            ]]}]
        }],
    };

    let res = serde_json::to_string(&e.meanings).unwrap();

    res
}

fn sample_french_query() -> String {
    let e = entry::Entry {
        query: "sentence".to_string(),
        meanings: vec![entry::Meaning {
            pos: "noun".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("A set of words that is complete in itself, typically containing a subject and predicate, conveying a statement, question, exclamation, or command, and consisting of a main clause and sometimes one or more subordinate clauses.".to_string()),
                entry::Lang::French("Un ensemble de mots qui est complet en lui-même, contenant généralement un sujet et un prédicat, véhiculant une déclaration, une question, une exclamation ou une commande, et consistant en une clause principale et parfois une ou plusieurs clauses subordonnées.".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("The teacher ask each student to make a sentence.".to_string()),
                entry::Lang::French("Le professeur demande à chaque étudiant de faire une phrase.".to_string()),
            ]]},
            entry::Item {
                meaning: vec![
                    entry::Lang::English("The punishment assigned to a defendant found guilty by a court, or fixed by law for a particular offense.".to_string()),
                    entry::Lang::French("Peine infligée à un prévenu reconnu coupable par un tribunal ou fixée par la loi pour une infraction particulière.".to_string())
                ],
                examples: vec![vec![
                    entry::Lang::English("Her husband is serving a three-year sentence for fraud.".to_string()),
                    entry::Lang::French("Son mari purge une peine de trois ans d'emprisonnement pour fraude.".to_string()),
                ]]
            }]
        },
        entry::Meaning {
            pos: "verb".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("Declare the punishment decided for (an offender).".to_string()),
                entry::Lang::French("Déclarer la peine décidée pour (un délinquant).".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                entry::Lang::French("Dix officiers militaires ont été condamnés à mort.".to_string()),
            ]]}]
        }],
    };

    let res = serde_json::to_string(&e.meanings).unwrap();

    res
}

fn sample_portuguese_query() -> String {
    let e = entry::Entry {
        query: "sentence".to_string(),
        meanings: vec![entry::Meaning {
            pos: "noun".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("A set of words that is complete in itself, typically containing a subject and predicate, conveying a statement, question, exclamation, or command, and consisting of a main clause and sometimes one or more subordinate clauses.".to_string()),
                entry::Lang::Portuguese("Um conjunto de palavras que é completo em si mesmo, geralmente contendo um sujeito e um predicado, transmitindo uma afirmação, pergunta, exclamação ou comando e consistindo em uma oração principal e, às vezes, uma ou mais orações subordinadas.".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("The teacher ask each student to make a sentence.".to_string()),
                entry::Lang::Portuguese("A professora pede que cada aluno faça uma frase.".to_string()),
            ]]},
            entry::Item {
                meaning: vec![
                    entry::Lang::English("The punishment assigned to a defendant found guilty by a court, or fixed by law for a particular offense.".to_string()),
                    entry::Lang::Portuguese("A punição atribuída a um réu considerado culpado por um tribunal, ou fixado por lei para uma ofensa específica.".to_string())
                ],
                examples: vec![vec![
                    entry::Lang::English("Her husband is serving a three-year sentence for fraud.".to_string()),
                    entry::Lang::Portuguese("O marido dela cumpre uma sentença de três anos por fraude.".to_string()),
                ]]
            }]
        },
        entry::Meaning {
            pos: "verb".to_string(),
            meanings: vec![entry::Item{ meaning: vec![
                entry::Lang::English("Declare the punishment decided for (an offender).".to_string()),
                entry::Lang::Portuguese("Declarar a punição decidida para (um infrator).".to_string()),
            ],
            examples: vec![vec![
                entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                entry::Lang::Portuguese("Dez militares foram condenados à morte.".to_string()),
            ]]}]
        }],
    };

    let res = serde_json::to_string(&e.meanings).unwrap();

    res
}

fn assemble_query(query: &str, target_lang: &state::TargetLang) -> String {
    let language_str = match target_lang {
        state::TargetLang::Chinese => "Chinese",
        state::TargetLang::Spanish => "Spanish",
        state::TargetLang::Japanese => "Japanese",
        state::TargetLang::Korean => "Korean",
        state::TargetLang::German => "German",
        state::TargetLang::French => "French",
        state::TargetLang::Portuguese => "Portuguese",
    };

    let query = ChatGPTQuery {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
                role: "system".to_string(),
                content: format!("You are a dictionary bot. Given a query, reply its meaning and sample sentences in English and {} in JSON format.", language_str).to_string()
        },
        Message {
            role: "user".to_string(),
            content: "sentence".to_string()
          },
         Message        {
            role: "assistant".to_string(),
            content:match target_lang {
                state::TargetLang::Chinese => { sample_chinese_query() }
                state::TargetLang::Spanish => { sample_spanish_query() }
                state::TargetLang::Japanese => {sample_japanese_query()}
                state::TargetLang::Korean => {sample_korean_query()}
                state::TargetLang::German => {sample_german_query()}
                state::TargetLang::French => {sample_french_query()}
                state::TargetLang::Portuguese => {sample_portuguese_query()}
            }
        } ,
        Message     {
            role: "user".to_string(),
            content: query.to_string()
          }],
    };
    let res = serde_json::to_string(&query).unwrap();

    res
}

pub async fn search(
    query: &str,
    auth_token: &str,
    target_lang: &state::TargetLang,
) -> Result<(i64,i64,entry::Entry)> {
    let bearer_auth = format!("Bearer {}", auth_token);

    let data = assemble_query(query, target_lang);

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
                            return Ok((parsed.usage.prompt_tokens, parsed.usage.completion_tokens, entry::Entry {
                                query: query.to_string(),
                                meanings: meanings,
                            }));
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

pub struct SentenceExampleQuery {
    pub query: String,
    pub meaning: String
}

fn assemble_sentence_example_query(
    sentence_query: &SentenceExampleQuery,
    target_lang: &state::TargetLang,
) -> String {
    let language_str = match target_lang {
        state::TargetLang::Chinese => "Chinese",
        state::TargetLang::Spanish => "Spanish",
        state::TargetLang::Japanese => "Japanese",
        state::TargetLang::Korean => "Korean",
        state::TargetLang::German => "German",
        state::TargetLang::French => "French",
        state::TargetLang::Portuguese => "Portuguese",
    };

    let sentence_sample = match target_lang {
        state::TargetLang::Chinese => {
            vec![
                vec![
                    entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                    entry::Lang::Chinese("十位军官被判处死刑。".to_string()),
                ],
                vec![
                    entry::Lang::English(
                        "The judge sentenced her to six months in jail.".to_string(),
                    ),
                    entry::Lang::Chinese("法官判她入狱六个月。".to_string()),
                ],
            ]
        }
        state::TargetLang::Spanish => {
            vec![
                vec![
                    entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                    entry::Lang::Chinese(
                        "Diez oficiales del ejército fueron condenados a muerte.".to_string(),
                    ),
                ],
                vec![
                    entry::Lang::English(
                        "The judge sentenced her to six months in jail.".to_string(),
                    ),
                    entry::Lang::Spanish(
                        "La jueza la sentenció a seis meses de cárcel.".to_string(),
                    ),
                ],
            ]
        }
        state::TargetLang::Japanese => {
            vec![
                vec![
                    entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                    entry::Lang::Japanese("陸軍将校10人が死刑を宣告された。".to_string()),
                ],
                vec![
                    entry::Lang::English(
                        "The judge sentenced her to six months in jail.".to_string(),
                    ),
                    entry::Lang::Japanese("裁判官は彼女に 6 か月の禁錮刑を宣告した。".to_string()),
                ],
            ]
        }
        state::TargetLang::Korean => {
            vec![
                vec![
                    entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                    entry::Lang::Korean("육군 장교 10명이 사형을 선고받았다.".to_string()),
                ],
                vec![
                    entry::Lang::English(
                        "The judge sentenced her to six months in jail.".to_string(),
                    ),
                    entry::Lang::Korean("판사는 그녀에게 6개월의 징역형을 선고했다.".to_string()),
                ],
            ]
        }
        state::TargetLang::German => {
            vec![
                vec![
                    entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                    entry::Lang::German(
                        "Zehn Offiziere der Armee wurden zum Tode verurteilt.".to_string(),
                    ),
                ],
                vec![
                    entry::Lang::English(
                        "The judge sentenced her to six months in jail.".to_string(),
                    ),
                    entry::Lang::German(
                        "Der Richter verurteilte sie zu einer sechsmonatigen Haft.".to_string(),
                    ),
                ],
            ]
        }
        state::TargetLang::French => {
            vec![
                vec![
                    entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                    entry::Lang::French(
                        "Dix officiers militaires ont été condamnés à mort.".to_string(),
                    ),
                ],
                vec![
                    entry::Lang::English(
                        "The judge sentenced her to six months in jail.".to_string(),
                    ),
                    entry::Lang::French(
                        "Le juge l'a condamnée à six mois d'emprisonnement.".to_string(),
                    ),
                ],
            ]
        }
        state::TargetLang::Portuguese => {
            vec![
                vec![
                    entry::Lang::English("Ten army officers were sentenced to death.".to_string()),
                    entry::Lang::Portuguese("Dez militares foram condenados à morte.".to_string()),
                ],
                vec![
                    entry::Lang::English(
                        "The judge sentenced her to six months in jail.".to_string(),
                    ),
                    entry::Lang::Portuguese(
                        "O juiz a sentenciou a seis meses de prisão.".to_string(),
                    ),
                ],
            ]
        }
    };

    let query = ChatGPTQuery {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
                role: "system".to_string(),
                content: format!("You are a dictionary bot. Given a query, reply more sample sentences in English and {} in JSON format.", language_str).to_string()
        },
        Message {
            role: "user".to_string(),
            content: format!("Query: \"sentence\" Meaning: \"Declare the punishment decided for (an offender).\"]")
          },
         Message        {
            role: "assistant".to_string(),
            content:serde_json::to_string(&sentence_sample).unwrap()
        },
        Message     {
            role: "user".to_string(),
            content: format!("Query: \"{}\" Meaning: \"{}\"]", sentence_query.query, sentence_query.meaning)
          }],
    };
    let res = serde_json::to_string(&query).unwrap();

    res
}

pub async fn search_example_sentences(
    search_query: &SentenceExampleQuery,
    auth_token: &str,
    target_lang: &state::TargetLang,
) -> Result<(i64, i64, Vec<Vec<entry::Lang>>)> {
    let bearer_auth = format!("Bearer {}", auth_token);

    let data = assemble_sentence_example_query(search_query, target_lang);

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
                            let result: Vec<Vec<entry::Lang>> = meanings;
                            return Ok((parsed.usage.prompt_tokens, parsed.usage.completion_tokens, result));
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
