[README.md](https://github.com/user-attachments/files/33506064/README.md)
# Akasa · 阿卡夏

**Enter a world. Make a choice. Let the story answer.**  
**进入一个世界，让每次选择都有回应。**

Akasa is an AI-native interactive storytelling product for people who want to explore a world, inhabit a character, and see their actions become part of a story. It brings together authored worlds, player choices, free-form actions, and AI-assisted narrative creation.

This repository shares a browser-playable **Snow Empress / 白雪女帝** prototype and a **Rust source module for character and world creation**. The broader Akasa product is available at [akasa.fun](https://akasa.fun/); the scope of this repository is described below.

[Product website](https://akasa.fun/) · [30-second video](./akasa-demo.mp4) · [中文说明](#中文说明) · [Product & business overview / BP](./阿卡夏_AI原生产品与商业叙事.pdf) · [Feedback](https://github.com/xlmsyq/Akasa-demo-9-2-2026/issues)

![Artwork used in the Snow Empress demo](./assets/snow-empress-mirror.png)

## Why Akasa

A story can offer a place to relax, express an idea, and try another way of being. Akasa explores how a story world can respond to the person inside it: a decision changes what happens next, a character carries a history, and a past experience becomes something to revisit and continue.

The product combines curated story worlds with user-created premises. Its product materials describe character creation, story choices, free-form actions, illustrated chapters, saved journeys, and shared branches. This repository provides a small, inspectable prototype of parts of that experience.

## What is in this repository

| Component | What it contains | Current scope |
| --- | --- | --- |
| [`index.html`](./index.html) | HTML, CSS, and JavaScript for the Snow Empress story | Browser demo with authored scenes and rule-based outcomes |
| [`assets/`](./assets/) | Scene artwork and character portraits | Visual assets used by the demo |
| [`mod.rs`](./mod.rs) | Rust routines for AI-assisted character/world drafts, parsing, validation, and trait normalization | Source module from a larger backend; not a standalone Rust application |
| [`akasa-demo.mp4`](./akasa-demo.mp4) | Approximately 30 seconds of mobile product footage | Walkthrough of the broader Akasa product |
| Product & business overview PDF | Product journey, design approach, reported usage, and business hypotheses | Context for the broader Akasa product |

### Browser demo

- **Ten story acts:** choices change agency, public support, power, and bonds, which determine the final outcome.
- **Visual storytelling:** scene artwork, character portraits, dialogue controls, and immediate choice feedback.
- **Solo play:** start a room and explore the story on your own.
- **Local room prototype:** room state can synchronize across tabs in the same browser profile and origin through browser storage and `BroadcastChannel`.
- **Responsive layout:** styles adapt the interface for smaller screens.

The browser demo uses authored story content and local JavaScript. It does not call an AI service or connect to the Rust module. Its room implementation is local to the browser; sharing a room URL does not provide multiplayer synchronization between different devices.

## Run the browser demo

**Requirements:** a modern browser. Python 3 is one simple option for serving the files locally. No API key or Rust installation is needed for this demo.

1. Download the repository using **Code → Download ZIP** and extract it, or clone it:

   ```bash
   git clone https://github.com/xlmsyq/Akasa-demo-9-2-2026.git
   cd Akasa-demo-9-2-2026
   ```

2. Open a terminal in the directory containing `index.html` and `assets`, then start a local server:

   **macOS / Linux**

   ```bash
   python3 -m http.server 8000 --bind 127.0.0.1
   ```

   **Windows with the Python launcher**

   ```powershell
   py -m http.server 8000 --bind 127.0.0.1
   ```

3. Open <http://127.0.0.1:8000> and select **单人试玩** to begin.
4. Choose actions and watch the character values and story record change. Try different decisions to explore the endings.

To explore the local room prototype, open the same local address in another tab in the same browser profile and join using the same five-digit room code. Keep both tabs on the same origin. Stop the local server with `Ctrl+C` when finished.

## AI creation module

The Rust source in [`mod.rs`](./mod.rs) illustrates part of Akasa's character and world creation workflow:

- Build generation prompts using the current character and world context.
- Request and parse structured JSON output from a model.
- Preserve character identity fields already specified by the user.
- Validate required fields and normalize six character traits to bounded values with a fixed total.
- Include unit-test definitions for parsing, normalization, and prompt constraints.

The module references surrounding DTOs, application errors, and the `agent` and `story_engine` crates. Those components and a `Cargo.toml` are not included in this repository snapshot. Consequently, the module and its tests cannot be built or run independently from this snapshot. The HTML demo can be explored separately using the instructions above.

## Product usage reported in the BP

The [product & business overview](./阿卡夏_AI原生产品与商业叙事.pdf), page 6, reports the following **seven-day product snapshot**:

| Metric / 指标 | Reported count / 报告数值 |
| --- | ---: |
| Unique users / 独立用户 | 914 |
| Game sessions / 游戏会话 | 621 |
| Choice submissions / 选择提交 | 2,615 |
| Shared-copy creations / 分享副本创建 | 122 |
| Intuition previews / 直觉预览 | 483 |
| Free-form action submissions / 自由行动提交 | 392 |
| Graph-template submissions / 图谱模板提交 | 81 |

These figures are team-reported activity for the broader Akasa product, not downloads or users of this GitHub repository. The deck labels the window as seven days but does not specify its calendar start and end dates; the figures are not presented as current totals. Retention, subscription conversion, and profitability remain separate questions to validate.

## Product materials

**[Watch the 30-second mobile walkthrough](./akasa-demo.mp4)** — approximately 3.3 MB. The recording shows the product's exploration interface, character cards, and world-creation entry. It illustrates the broader hosted product; the browser prototype in this repository has the scope documented above.

Read [阿卡夏：AI 原生产品与商业叙事](./阿卡夏_AI原生产品与商业叙事.pdf) for the product walkthrough, world and character design, reported activity, and proposed business model.

The deck distinguishes the current product experience from future validation work. Subscription pricing, video integration, retention, and unit economics are described as areas for further testing or development. Product-level descriptions in the deck should be read alongside the repository scope above.

## Development directions

The following are proposed next steps, rather than features already included in this repository:

- Package a runnable character/world creation example with its schemas, dependencies, configuration, and tests.
- Add reusable examples of world rules, character state, and choice consequences.
- Evaluate narrative continuity, character consistency, generation failures, latency, and cost.
- Extend the browser demo with English story content and accessibility improvements.
- Explore a backend room service for play across devices.

## Feedback and contributions

Play the demo, explore a different ending, and tell us where the experience works or breaks. Feedback about unclear choices, story continuity, mobile layouts, and setup instructions is especially useful.

Please [open an issue](https://github.com/xlmsyq/Akasa-demo-9-2-2026/issues) with your browser/device, steps to reproduce, expected behavior, and what happened. Documentation improvements and focused bug reports are welcome. If the project interests you, a GitHub Star helps others discover it.

## License and materials

A repository-wide open-source license has not yet been published in this snapshot. Code, artwork, story text, and business materials may have separate terms. Please contact the repository maintainer about reuse until the applicable licenses are documented.

---

## 中文说明

### 阿卡夏是什么

阿卡夏是一个 AI 原生的游戏化互动内容产品：进入一个喜欢的世界，以自己的身份做出选择，让文字、人物和画面回应你的行动。

产品围绕“放松、表达、沉浸”展开，将作者设计的世界规则与用户的自由行动结合起来。完整产品材料展示了世界与角色创建、选项与自由表达、图文故事、存档续读和分支分享等体验。

本仓库目前公开的是 **《白雪女帝》浏览器演示，以及角色与世界生成的 Rust 源码模块**。产品官网与本仓库的功能范围不同，具体以本文列出的文件和说明为准。

### 现在可以体验什么

| 内容 | 当前实现 |
| --- | --- |
| 互动故事 | 十幕预设剧情，选择影响独立、民心、权力、羁绊，并导向不同结局 |
| 视觉与反馈 | 场景图、人物立绘、逐句对话、行动反馈与故事记录 |
| 单人试玩 | 在浏览器中开始并完成一段故事 |
| 本地房间 | 同一浏览器、同一来源下的多标签页状态同步；尚无跨设备房间服务 |
| AI 创作源码 | 角色与世界草稿的提示构建、JSON 解析、字段校验和属性归一化；需原项目配套模块才能运行 |

网页演示使用预设故事和本地规则，不需要 API 密钥，也没有接入仓库中的 Rust 模块。官网上的完整 AI 互动体验请访问 [akasa.fun](https://akasa.fun/)。

### 如何运行

下载并解压整个仓库，保留 `index.html` 与 `assets` 的相对位置。在该目录打开终端，使用上文对应系统的 Python 命令启动服务，然后在浏览器访问 `http://127.0.0.1:8000`，点击“单人试玩”。

### 产品与商业资料

[观看约 30 秒产品演示视频](./akasa-demo.mp4)（约 3.3 MB），查看移动端故事探索、人物卡片和世界创建入口。这段录屏展示的是完整产品的界面，本仓库中的网页原型范围以上文说明为准。

[阅读《阿卡夏_AI原生产品与商业叙事》PDF](./阿卡夏_AI原生产品与商业叙事.pdf)，了解产品体验流程、世界架构、用户活动数据与商业验证方向。

上方数据表转述 BP 第 6 页的七日产品数据，属于团队提供的产品活动快照。BP 未标明该窗口的具体起止日期；这些数字不代表本仓库的使用量，也不作为当前累计数据或留存、收入结论。

### 一起改进

欢迎试玩、反馈问题，或提出你想探索的故事机制。提交问题时，请注明浏览器或设备、操作步骤、预期表现与实际结果。当前优先方向包括：补齐可运行的生成模块示例、改进人物与世界一致性评估、英文故事内容、移动端体验及跨设备互动。

代码、故事、图片与 BP 的使用许可仍需分别明确；当前仓库尚未发布统一的开源许可证。
