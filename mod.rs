mod dto;

pub use dto::*;

use agent::{
    agent::{CallModelEvent, call_model},
    core::Message,
};
use axum::Json;
use futures::StreamExt;
use story_engine::utils::{build_chat_model, parse_json_response};
use uuid::Uuid;

use crate::error::AppError;

use super::dto::ApiResponse;

type ApiResult<T> = Result<Json<ApiResponse<T>>, AppError>;

const CREATION_TRAIT_MIN: u8 = 1;
const CREATION_TRAIT_MAX: u8 = 10;
const CREATION_TRAIT_TOTAL: u16 = 30;

pub async fn generate_creation_draft(
    Json(request): Json<GenerateCreationDraftRequest>,
) -> ApiResult<GenerateCreationDraftData> {
    let mut model = build_chat_model();
    model.set_output_json(true);
    let variant_id = Uuid::new_v4().simple().to_string();

    let messages = vec![
        Message::system(
            r#"你是一名互动文字游戏创建页的 AI 表单生成器，负责生成可直接填入表单的中文内容。

            总体目标：
            1. 生成内容应保持随机、多样、中性，不偏男频、女频、男性向、女性向、男男、女女或固定恋爱范式；题材、玩家角色性别、关系张力和叙事欲望每次都要有不同组合，不要固定套路。
            2. 内容用于长期互动叙事的开局表单，不是人物小传或世界百科。字段要精炼、具体、可演绎，有欲望、秘密、代价、关系张力或异常规则。**所生成的弱点和冲突应具有阶段性升级的潜质，能够支撑一个从常态破裂到至暗时刻再到终局收束的完整叙事弧线。**
            3. 所有角色必须是成年人；不要生成露骨性内容、未成年人恋爱、现实名人、真实品牌或侵权 IP。
            4. 可以参考用户当前表单来避开重复，并让新内容与当前另一组表单能够搭配；不要照抄当前表单。
            5. 如果用户已经填写姓名、性别或有效成年年龄，生成 character 时必须原样保留这些已填写字段，只围绕它们补全命运烙印、人物描述和属性倾向；如果姓名或性别未指定，必须自行生成姓名并随机选择“男”或“女”，绝不能输出“未填写”。
            6. 生成 world 时，必须让世界背景与当前人物表单相容，让世界的秩序、压力和核心矛盾能自然牵引该角色。
            7. 生成 character 时，必须让角色与当前世界表单相容，尤其要服从世界背景、世界描述中的秩序、异常机制、禁忌和冲突结构。
            8. 只输出一个合法 json 对象，不要代码块、标题、解释、注释或对象外文本。

            当 target 是 "character" 时，输出结构必须是：
            {
              "character": {
                "name": "2到4个汉字的中文名",
                "gender": "男或女",
                "age": 18到80之间的整数,
                "background": "一句命运烙印，12到20个中文字符，带明确钩子",
                "appearance": "45到110个中文字符的人物描述，包含外貌、气质、弱点或行动习惯。**弱点描述应暗示其会在叙事不同阶段以不同方式暴露——有的在早期牵引，有的在中期制造重大挫败，有的在至暗时刻彻底撕裂角色。**",
                "traits": {
                  "intellect": 1到10的整数,
                  "physique": 1到10的整数,
                  "endurance": 1到10的整数,
                  "courage": 1到10的整数,
                  "rationality": 1到10的整数,
                  "altruism": 1到10的整数
                }
              }
            }

            character 约束：
            - 六项 traits 总和必须恰好等于 30。
            - 不要生成完美六边形；至少有一项不高于 3，至少有一项不低于 7。**极端值所在维度应能暗示角色在五幕弧线中的关键转折点：例如勇气低于3可能在至暗时刻溃败，理性高于7可能在关键时刻因过度计算而失去人性连接。**
            - background 要像“命运烙印”，不是职业标签，例如可以包含契约、秘密、异能、旧爱、权力交易、地下组织、异常档案、白月光、记忆错位等元素。**这个烙印应该是可分层揭示的——表层信息可在第一幕揭露，深层真相则留待第三幕前后引爆。**
            - appearance 要能直接显示在表单里，保持一段话，不要分点。

            当 target 是 "world" 时，输出结构必须是：
            {
              "world": {
                "era": "4到24个中文字符的世界背景名",
                "description": "80到180个中文字符的世界描述，一段话，说明秩序、异常机制、主要压力和故事张力。**这个世界的压力源应具有升级阶梯——初期的摩擦、中期的围困、后期的绝境，各自清晰可辨，让五幕节奏有处可落。**"
              }
            }

            world 约束：
            - era 要适合放在下拉输入框里，不能太长。
            - description 要能直接填入创建页文本框，具体但不冗长。
            - 世界应能适配不同性别玩家角色和多种关系张力，不要默认女性玩家角色、男性玩家角色或特定恋爱方向，也不要把所有冲突都写成恋爱。
            "#,
        ),
        Message::user(build_creation_draft_prompt(&request, &variant_id)),
    ];

    let mut stream = std::pin::pin!(call_model(&model, &messages, None));

    while let Some(event) = stream.next().await {
        match event {
            CallModelEvent::Completed { content, .. } => {
                let data =
                    parse_generated_creation_draft(&content, &request).map_err(|message| {
                        AppError::internal(format!("模型返回的创建表单格式不符合预期：{message}"))
                    })?;
                return Ok(Json(ApiResponse::ok(data)));
            }
            CallModelEvent::Error(message) => {
                return Err(AppError::internal(format!("生成创建表单失败：{message}")));
            }
            CallModelEvent::TextChunk(_) | CallModelEvent::ReasoningChunk(_) => {}
        }
    }

    Err(AppError::internal("模型未返回完整创建表单。"))
}

fn build_creation_draft_prompt(request: &GenerateCreationDraftRequest, variant_id: &str) -> String {
    let target = match request.target {
        GenerateCreationDraftTarget::Character => "character",
        GenerateCreationDraftTarget::World => "world",
    };
    let current_background = match request.target {
        GenerateCreationDraftTarget::Character => "未提供".to_string(),
        GenerateCreationDraftTarget::World => empty_placeholder(&request.character.background),
    };
    let current_appearance = match request.target {
        GenerateCreationDraftTarget::Character => "未提供".to_string(),
        GenerateCreationDraftTarget::World => empty_placeholder(&request.character.appearance),
    };
    let current_era = match request.target {
        GenerateCreationDraftTarget::Character => empty_placeholder(&request.world.era),
        GenerateCreationDraftTarget::World => "未提供".to_string(),
    };
    let current_description = match request.target {
        GenerateCreationDraftTarget::Character => empty_placeholder(&request.world.description),
        GenerateCreationDraftTarget::World => "未提供".to_string(),
    };

    format!(
        r#"target: {target}
        本次生成变体ID：{variant_id}

        [当前人物表单]
        姓名：{name}
        性别：{gender}
        年龄：{age}
        命运烙印：{background}
        人物描述：{appearance}
        属性：智力 {intellect}，体力 {physique}，耐力 {endurance}，勇气 {courage}，理性 {rationality}，利他 {altruism}

        [当前世界表单]
        世界背景：{era}
        世界描述：{description}

        请根据 target 只生成对应的表单片段。

        生成规则：
        - 当 target 是 character：如果当前姓名已填写，返回的 character.name 必须等于当前姓名；如果当前姓名未指定，必须生成 2 到 4 个汉字的中文名，不能返回“未填写”；如果当前性别是“男”或“女”，返回的 character.gender 必须等于当前性别；如果当前性别未指定，必须随机返回“男”或“女”，不能返回“未填写”；如果当前年龄是 18 到 80 之间的成年人年龄，返回的 character.age 必须等于当前年龄。角色其余字段必须贴合当前世界表单。
        - character 的 background、appearance、traits 必须围绕本次生成变体ID改换方向，不得复用当前命运烙印、人物描述或完全相同的属性分布。改换时，让新方向在五幕叙事弧线上占据不同的节奏位：例如本次生成的弱点更适合在第二幕爆发，则上次的弱点可能偏第四幕的代价。
        - 当 target 是 world：返回的 world 必须贴合当前人物表单，尤其要能解释当前人物的命运烙印、行动倾向和关系张力如何在这个世界中被放大。
        - world 的 era、description 必须围绕本次生成变体ID改换方向，不得复用当前世界背景或当前世界描述。改换时，让新世界的压力结构与当前人物的弱点形成阶段性的咬合——有的压力在第一幕就显现，有的潜伏到第三幕才反噬。
        - 如果另一组表单尚未填写完整，可以自由补足，但不能与已填写内容冲突。"#,
        variant_id = variant_id,
        name = creation_name_prompt_value(&request.character.name),
        gender = creation_gender_prompt_value(&request.character.gender),
        age = request.character.age,
        background = current_background,
        appearance = current_appearance,
        intellect = request.character.traits.intellect,
        physique = request.character.traits.physique,
        endurance = request.character.traits.endurance,
        courage = request.character.traits.courage,
        rationality = request.character.traits.rationality,
        altruism = request.character.traits.altruism,
        era = current_era,
        description = current_description,
    )
}

fn empty_placeholder(value: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        "未填写".to_string()
    } else {
        value.to_string()
    }
}

fn creation_name_prompt_value(value: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        "未指定（请生成2到4个汉字中文名，不要输出“未填写”）".to_string()
    } else {
        value.to_string()
    }
}

fn creation_gender_prompt_value(value: &str) -> String {
    match value.trim() {
        "男" => "男".to_string(),
        "女" => "女".to_string(),
        _ => "未指定（请随机选择男或女，不要输出“未填写”）".to_string(),
    }
}

fn parse_generated_creation_draft(
    content: &str,
    request: &GenerateCreationDraftRequest,
) -> Result<GenerateCreationDraftData, String> {
    let parsed = parse_json_response::<GenerateCreationDraftData>(content)?;
    validate_generated_creation_draft(parsed, request)
}

fn validate_generated_creation_draft(
    data: GenerateCreationDraftData,
    request: &GenerateCreationDraftRequest,
) -> Result<GenerateCreationDraftData, String> {
    match request.target {
        GenerateCreationDraftTarget::Character => {
            let character = data
                .character
                .ok_or_else(|| "`character` 不能为空。".to_string())?;
            Ok(GenerateCreationDraftData {
                character: Some(apply_locked_character_fields(
                    validate_creation_character(character)?,
                    &request.character,
                )),
                world: None,
            })
        }
        GenerateCreationDraftTarget::World => {
            let world = data.world.ok_or_else(|| "`world` 不能为空。".to_string())?;
            Ok(GenerateCreationDraftData {
                character: None,
                world: Some(validate_creation_world(world)?),
            })
        }
    }
}

fn validate_creation_character(character: CreationCharacter) -> Result<CreationCharacter, String> {
    let name = trim_required(character.name, "`character.name`")?;
    let age = character.age.clamp(18, 80);
    let background = trim_required(character.background, "`character.background`")?;
    let appearance = trim_required(character.appearance, "`character.appearance`")?;
    let traits = normalize_creation_traits(character.traits);
    let gender = normalize_creation_gender(character.gender, &name, age, &traits);

    Ok(CreationCharacter {
        name,
        gender,
        age,
        appearance,
        traits,
        background,
    })
}

fn validate_creation_world(world: CreationWorld) -> Result<CreationWorld, String> {
    let era = trim_required(world.era, "`world.era`")?;
    let description = trim_required(world.description, "`world.description`")?;
    let style_guide = world.style_guide.trim().to_string();

    Ok(CreationWorld {
        era,
        description,
        style_guide,
    })
}

fn apply_locked_character_fields(
    mut generated: CreationCharacter,
    current: &CreationCharacter,
) -> CreationCharacter {
    if let Some(name) = locked_character_name(current) {
        generated.name = name;
    }
    if let Some(gender) = locked_character_gender(current) {
        generated.gender = gender;
    }
    if let Some(age) = locked_character_age(current) {
        generated.age = age;
    }
    generated
}

fn locked_character_name(character: &CreationCharacter) -> Option<String> {
    let name = character.name.trim();
    (!name.is_empty()).then(|| name.to_string())
}

fn locked_character_gender(character: &CreationCharacter) -> Option<String> {
    match character.gender.trim() {
        "男" => Some("男".to_string()),
        "女" => Some("女".to_string()),
        _ => None,
    }
}

fn locked_character_age(character: &CreationCharacter) -> Option<u16> {
    (18..=80).contains(&character.age).then_some(character.age)
}

fn normalize_creation_gender(
    gender: String,
    name: &str,
    age: u16,
    traits: &CreationTraits,
) -> String {
    match gender.trim() {
        "男" => "男".to_string(),
        "女" => "女".to_string(),
        _ => fallback_creation_gender(name, age, traits),
    }
}

fn fallback_creation_gender(name: &str, age: u16, traits: &CreationTraits) -> String {
    let seed = name
        .bytes()
        .map(u16::from)
        .sum::<u16>()
        .saturating_add(age)
        .saturating_add(u16::from(traits.intellect))
        .saturating_add(u16::from(traits.physique))
        .saturating_add(u16::from(traits.endurance))
        .saturating_add(u16::from(traits.courage))
        .saturating_add(u16::from(traits.rationality))
        .saturating_add(u16::from(traits.altruism));
    if seed % 2 == 0 {
        "男".to_string()
    } else {
        "女".to_string()
    }
}

fn trim_required(value: String, field: &str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() {
        Err(format!("{field} 不能为空。"))
    } else {
        Ok(value.to_string())
    }
}

fn normalize_creation_traits(traits: CreationTraits) -> CreationTraits {
    let mut values = [
        traits
            .intellect
            .clamp(CREATION_TRAIT_MIN, CREATION_TRAIT_MAX),
        traits
            .physique
            .clamp(CREATION_TRAIT_MIN, CREATION_TRAIT_MAX),
        traits
            .endurance
            .clamp(CREATION_TRAIT_MIN, CREATION_TRAIT_MAX),
        traits.courage.clamp(CREATION_TRAIT_MIN, CREATION_TRAIT_MAX),
        traits
            .rationality
            .clamp(CREATION_TRAIT_MIN, CREATION_TRAIT_MAX),
        traits
            .altruism
            .clamp(CREATION_TRAIT_MIN, CREATION_TRAIT_MAX),
    ];

    let mut total = values.iter().map(|value| u16::from(*value)).sum::<u16>();
    while total < CREATION_TRAIT_TOTAL {
        for value in &mut values {
            if total >= CREATION_TRAIT_TOTAL {
                break;
            }
            if *value < CREATION_TRAIT_MAX {
                *value += 1;
                total += 1;
            }
        }
    }

    while total > CREATION_TRAIT_TOTAL {
        for value in values.iter_mut().rev() {
            if total <= CREATION_TRAIT_TOTAL {
                break;
            }
            if *value > CREATION_TRAIT_MIN {
                *value -= 1;
                total -= 1;
            }
        }
    }

    CreationTraits {
        intellect: values[0],
        physique: values[1],
        endurance: values[2],
        courage: values[3],
        rationality: values[4],
        altruism: values[5],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn creation_request(target: GenerateCreationDraftTarget) -> GenerateCreationDraftRequest {
        GenerateCreationDraftRequest {
            target,
            character: CreationCharacter {
                name: String::new(),
                gender: String::new(),
                age: 0,
                appearance: String::new(),
                background: String::new(),
                traits: CreationTraits {
                    intellect: 5,
                    physique: 5,
                    endurance: 5,
                    courage: 5,
                    rationality: 5,
                    altruism: 5,
                },
            },
            world: CreationWorld {
                era: String::new(),
                description: String::new(),
                style_guide: String::new(),
            },
        }
    }

    #[test]
    fn generate_creation_request_accepts_omitted_same_target_fields() {
        let character_request = serde_json::from_str::<GenerateCreationDraftRequest>(
            r#"{
                "target": "character",
                "character": {
                    "name": "叶知秋",
                    "gender": "女",
                    "age": 28,
                    "traits": {
                        "intellect": 5,
                        "physique": 5,
                        "endurance": 5,
                        "courage": 5,
                        "rationality": 5,
                        "altruism": 5
                    }
                },
                "world": {
                    "era": "都市异能",
                    "description": "所有异能都由情绪债务触发。"
                }
            }"#,
        )
        .expect("character request should allow omitted generated fields");
        assert!(character_request.character.background.is_empty());
        assert!(character_request.character.appearance.is_empty());

        let world_request = serde_json::from_str::<GenerateCreationDraftRequest>(
            r#"{
                "target": "world",
                "character": {
                    "name": "叶知秋",
                    "gender": "女",
                    "age": 28,
                    "appearance": "短发，习惯在雨夜独行。",
                    "background": "被冷艳继承者认作宿命例外的人",
                    "traits": {
                        "intellect": 5,
                        "physique": 5,
                        "endurance": 5,
                        "courage": 5,
                        "rationality": 5,
                        "altruism": 5
                    }
                },
                "world": {}
            }"#,
        )
        .expect("world request should allow omitted generated fields");
        assert!(world_request.world.era.is_empty());
        assert!(world_request.world.description.is_empty());
    }

    #[test]
    fn parse_generated_creation_character_trims_and_accepts_valid_traits() {
        let request = creation_request(GenerateCreationDraftTarget::Character);
        let parsed = parse_generated_creation_draft(
            r#"{
                "character": {
                    "name": " 林昼雪 ",
                    "gender": "女",
                    "age": 24,
                    "appearance": " 黑色齐肩发，习惯把工牌藏进外套口袋。 ",
                    "background": " 与霸道女总裁签下隐婚契约的普通女孩 ",
                    "traits": {
                        "intellect": 8,
                        "physique": 3,
                        "endurance": 4,
                        "courage": 5,
                        "rationality": 6,
                        "altruism": 4
                    }
                }
            }"#,
            &request,
        )
        .expect("character draft should parse");

        let character = parsed.character.expect("character payload");
        assert_eq!(character.name, "林昼雪");
        assert_eq!(character.background, "与霸道女总裁签下隐婚契约的普通女孩");
        assert!(parsed.world.is_none());
    }

    #[test]
    fn parse_generated_creation_character_normalizes_trait_total() {
        let request = creation_request(GenerateCreationDraftTarget::Character);
        let parsed = parse_generated_creation_draft(
            r#"{
                "character": {
                    "name": "林昼雪",
                    "gender": "未知",
                    "age": 12,
                    "appearance": "黑色齐肩发，习惯把工牌藏进外套口袋。",
                    "background": "与霸道女总裁签下隐婚契约的普通女孩",
                    "traits": {
                        "intellect": 11,
                        "physique": 0,
                        "endurance": 4,
                        "courage": 5,
                        "rationality": 6,
                        "altruism": 3
                    }
                }
            }"#,
            &request,
        )
        .expect("character draft should be normalized");

        let character = parsed.character.expect("character payload");
        let traits = character.traits;
        let total = u16::from(traits.intellect)
            + u16::from(traits.physique)
            + u16::from(traits.endurance)
            + u16::from(traits.courage)
            + u16::from(traits.rationality)
            + u16::from(traits.altruism);

        assert_eq!(character.gender, "女");
        assert_eq!(character.age, 18);
        assert_eq!(total, CREATION_TRAIT_TOTAL);
        assert!(
            [
                traits.intellect,
                traits.physique,
                traits.endurance,
                traits.courage,
                traits.rationality,
                traits.altruism
            ]
            .iter()
            .all(|value| (CREATION_TRAIT_MIN..=CREATION_TRAIT_MAX).contains(value))
        );
    }

    #[test]
    fn parse_generated_creation_character_preserves_locked_identity_fields() {
        let mut request = creation_request(GenerateCreationDraftTarget::Character);
        request.character.name = " 叶知秋 ".to_string();
        request.character.gender = "男".to_string();
        request.character.age = 31;

        let parsed = parse_generated_creation_draft(
            r#"{
                "character": {
                    "name": "林昼雪",
                    "gender": "女",
                    "age": 24,
                    "appearance": "黑色齐肩发，习惯把工牌藏进外套口袋。",
                    "background": "与霸道女总裁签下隐婚契约的普通女孩",
                    "traits": {
                        "intellect": 8,
                        "physique": 3,
                        "endurance": 4,
                        "courage": 5,
                        "rationality": 6,
                        "altruism": 4
                    }
                }
            }"#,
            &request,
        )
        .expect("character draft should parse");

        let character = parsed.character.expect("character payload");
        assert_eq!(character.name, "叶知秋");
        assert_eq!(character.gender, "男");
        assert_eq!(character.age, 31);
    }

    #[test]
    fn build_creation_draft_prompt_includes_cross_form_constraints() {
        let mut character_request = creation_request(GenerateCreationDraftTarget::Character);
        character_request.world.era = "都市异能".to_string();
        character_request.world.description = "所有异能都由情绪债务触发。".to_string();
        character_request.character.background = "旧角色烙印不应进入角色生成提示".to_string();
        character_request.character.appearance = "旧角色描述不应进入角色生成提示".to_string();
        let character_prompt = build_creation_draft_prompt(&character_request, "variant-character");
        assert!(character_prompt.contains("角色其余字段必须贴合当前世界表单"));
        assert!(character_prompt.contains("本次生成变体ID：variant-character"));
        assert!(character_prompt.contains("不得复用当前命运烙印、人物描述或完全相同的属性分布"));
        assert!(!character_prompt.contains("旧角色烙印不应进入角色生成提示"));
        assert!(!character_prompt.contains("旧角色描述不应进入角色生成提示"));
        assert!(character_prompt.contains("都市异能"));
        assert!(character_prompt.contains("所有异能都由情绪债务触发。"));

        let mut world_request = creation_request(GenerateCreationDraftTarget::World);
        world_request.character.name = "叶知秋".to_string();
        world_request.character.background = "被冷艳继承者认作宿命例外的人".to_string();
        world_request.world.era = "旧世界背景不应进入世界生成提示".to_string();
        world_request.world.description = "旧世界描述不应进入世界生成提示".to_string();
        let world_prompt = build_creation_draft_prompt(&world_request, "variant-world");
        assert!(world_prompt.contains("world 必须贴合当前人物表单"));
        assert!(world_prompt.contains("本次生成变体ID：variant-world"));
        assert!(world_prompt.contains("不得复用当前世界背景或当前世界描述"));
        assert!(world_prompt.contains("被冷艳继承者认作宿命例外的人"));
        assert!(!world_prompt.contains("旧世界背景不应进入世界生成提示"));
        assert!(!world_prompt.contains("旧世界描述不应进入世界生成提示"));
    }

    #[test]
    fn build_creation_draft_prompt_treats_blank_identity_as_generation_targets() {
        let request = creation_request(GenerateCreationDraftTarget::Character);
        let prompt = build_creation_draft_prompt(&request, "variant-random");

        assert!(prompt.contains("姓名：未指定（请生成2到4个汉字中文名，不要输出“未填写”）"));
        assert!(prompt.contains("性别：未指定（请随机选择男或女，不要输出“未填写”）"));
        assert!(prompt.contains("如果当前姓名未指定，必须生成 2 到 4 个汉字的中文名"));
        assert!(prompt.contains("如果当前性别未指定，必须随机返回“男”或“女”"));
    }

    #[test]
    fn parse_generated_creation_world_trims_visible_fields() {
        let request = creation_request(GenerateCreationDraftTarget::World);
        let parsed = parse_generated_creation_draft(
            r#"{
                "world": {
                    "era": " 都市奇幻 ",
                    "description": " 雨夜高楼间藏着古老结界，女巫与财阀共同维持秘密秩序。 "
                }
            }"#,
            &request,
        )
        .expect("world draft should parse");

        let world = parsed.world.expect("world payload");
        assert_eq!(world.era, "都市奇幻");
        assert_eq!(
            world.description,
            "雨夜高楼间藏着古老结界，女巫与财阀共同维持秘密秩序。"
        );
        assert!(parsed.character.is_none());
    }
}
