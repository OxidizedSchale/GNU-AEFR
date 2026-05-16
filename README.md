[English](./README_global/README_en.md) | [日本語](./README_global/README_jp.md) | [한국어](./README_global/README_kr.md)

<p align="center">
  <img src="resources/comittee_logo.png" width="450" alt="GNA:AEFR comittee logo">
  <br>
  <b>真正的黑客从源码编译，真正的自由在 AGPL 下呼吸</b>
</p>

# GNA's Not AA : AEFR's Eternal Freedom & Rust-rendered

> **执委会公告**：本项目受 GNU 自由精神启发而生，并非官方 GNU 项目。目前已向“自由软件基金会 (FSF) 自由软件目录”提交收录申请。

## 我们的 Rust 纯度比 GNU/Linux 内核高 99.7%!

**GNA:AEFR 是一个秉承 GNU 宣言精神的自由软件项目，致力于解放基沃托斯 (Kivotos) 的创作环境。我们坚信：分发软件而不向社区交付源代码，是一种剥夺用户自由的不负责任行为。**

---

### ​⚖️ 许可证协议 (License)
*   **v0.8.3 之前的版本**：采用 **GPL-3.0** 协议发布。
*   **v0.8.3 及后续版本**：为封死云端闭源漏洞，正式迁移并强制采用 **AGPL-3.0** 协议发布。

---

## 🧭 导览：GNA:AEFR 的哲学

> **GNA:AEFR** 并非一个以取悦大众为目的的平庸应用。它目前正处于核心内核的快速迭代期，是一个由纯粹 Rust 逻辑构筑的高性能编辑器实例。**在计算机的世界里，最短的路径永远是最无敌的。** 
> 
> 如果您追求目前的普适兼容性与开箱即用的低门槛，请移步 AA；如果您追求极致的执行效率、绝对的创作自由以及由社区驱动的硬核维护，欢迎来到 **GNA:AEFR** 的世界。

*   **非官方粉丝制作**：完全使用纯 Rust 语言打造，为《蔚蓝档案》二次创作而生的多平台、多线程编辑器。
*   **去引擎化架构**：拒绝依赖 Unity/Unreal 等臃肿的商业引擎，直接基于轻量级 `egui` 库驱动 GPU 渲染，直达底层。
*   **全平台主权**：原生支持 GNU/Linux、Android、macOS 以及 Windows，确保在不同操作系统间拥有一致的卓越体验。

### ✨ 现已实现 (Core Features)
- [x] **场景动态重构**：支持实时更换高分辨率场景背景。
- [x] **多维骨骼渲染**：支持同时导入并并行渲染多达 5 个 Spine 骨骼动画文件。
- [x] **正统视觉标准**：完美还原标准的基沃托斯风格对话框渲染。
- [x] **实时动作调度**：支持实时切换骨骼动画状态（表情、肢体动作、特效）。
- [x] **异步音频系统**：支持背景音乐 (BGM) 的异步加载与无缝流式播放。

### 🎯 未来计划 (Roadmap)
- [ ] **非线性编辑系统**：引入专业级时间轴 (Timeline) 逻辑。
- [ ] **平滑补间动画**：实现角色动作的线性插值与平滑混合切换。
- [ ] **场景插画系统**：支持场景内即时弹出式图片（Illustration）分层。
- [ ] **转场转码器**：开发包括淡入淡出、划变在内的多种场景切换特效。
- [ ] **交互式气泡**：动态追踪角色头顶的表情符号气泡。

**我们欢迎任何认同自由软件理想的黑客参与 GNA:AEFR 的开发。**

---

## 🚀 开始使用 (Getting Started)

> *"Release? Real hackers compile from source." ;-)*

前往 [**Releases**](https://github.com/OxidizedSchale/GNA-AEFR/releases) 页面获取最新源代码，或直接下载针对特定平台预编译的二进制文件。

**GNA:AEFR** 采用 **图形化界面 (GUI)** 与 **指令驱动 (Command-Driven)** 深度融合的交互模型。

*   **桌面端 (Desktop)**：推荐使用完整的图形化控制面板。
*   **移动端 (Mobile)**：涉及外部资源导入时，目前需通过控制台指令进行精确调度。

点击界面左上角的 `[SHELL]` 按钮即可唤起内置的硬核调试控制台。

<details>
<summary><strong>📖 点击展开：GNA:AEFR 指令参考手册 (Manual)</strong></summary>

### 1. 场景与视觉 (Visuals)

*   **加载背景**
    *   **指令**: `BG <image_path>`
    *   **说明**: 瞬时切换背景，支持 `.jpg`, `.png`, `.webp` 格式。
    *   **示例**: `BG C:\Assets\BlueArchive\BG_Classroom.png`

*   **装填角色**
    *   **指令**: `LOAD <slot_ID> <.atlas_path>`
    *   **说明**: 将 Spine 资源装载至 `0`~`4` 号槽位。加载成功后，SHELL 将返回该角色的完整可用动作清单。
    *   **示例**: `LOAD 0 D:\Assets\Shiroko\Shiroko_Home.atlas`

### 2. 动作与演出 (Motion)

*   **切换动作**
    *   **指令**: `ANIM <slot_ID> <animation_name> [loop: true/false]`
    *   **说明**: 指定槽位角色动作切换。`true` 为强制循环，`false` 为单次播放。动作名必须与加载时的返回清单精确匹配。
    *   **示例**:
        ```bash
        ANIM 0 Start_Idle_01 true    # 白子循环待机
        ANIM 1 Attack_Normal false   # 1号位角色执行单次攻击
        ```

### 3. 剧本与对话 (Storytelling)

*   **发送对话**
    *   **指令**: `TALK <name>|<affiliation>|<content>`
    *   **说明**: 渲染标准对话框，集成打字机动效。**参数间必须使用管道符 `|` 进行物理分隔。**
    *   **示例**:
        ```bash
        TALK 砂狼白子|对策委员会|老师，我们要去抢银行吗？
        TALK 阿洛娜|什亭之箱|老师，请不要在工作时间摸鱼！
        ```

### 4. 音频系统 (Audio)

*   **播放 BGM**
    *   **指令**: `BGM <audio_path>`
    *   **说明**: 调度音频引擎异步加载并循环播放音频文件。
    *   **示例**: `BGM D:\Music\Unwelcome_School.mp3`

*   **停止音乐**
    *   **指令**: `STOP`
    *   **说明**: 强制终止当前活跃的音频输出流。

</details>

---

### 💡 极客贴士 (Pro Tips)

*   **路径预处理**：Windows 用户可直接粘贴文件路径，GNA 会自动处理引号与转义符；Android / Termux 用户请确保使用 `/sdcard/` 开头的绝对路径。
*   **调度器保障**：得益于独有的“绅士调度器”，UI 线程被物理隔离，即便在 5 人同屏满载计算时，界面依然能保持绝对丝滑。
*   **实时审计**：建议保持 SHELL 窗口可见，以便实时审计资源加载状态与动作解析日志。

---

## 🤝 贡献指南 (Contribution)

<details>
<summary><strong>点击展开：GNA 执委会贡献者守则</strong></summary>

### 技术栈纯洁性
本项目严禁任何破坏架构纯洁性的行为，核心业务逻辑必须 100% 使用 **Rust** 实现。

*   **原则性拒绝**：任何引入与 C++ 运行时或重量级框架（如 Qt, Unity）进行复杂 JNI/FFI 交互的提议。
*   **例外许可**：仅允许针对底层系统级 C 库（图形 API、音频后端）进行 Rust 安全封装。在此情形下：
    *   必须优先采用成熟的社区 `-sys` 绑定。
    *   若必须编写 `unsafe` 调用，需严格遵循下文的安全性证明标准。
    *   最终目标必须是将非安全调用完全解耦，提供透明的安全 Rust API。

### unsafe 准则：性能之刃的约束
> `unsafe` 是 Rust 赋予开发者的性能利刃。本项目的原则是：“非必要不动用，动用必万无一失。”

**核心原则：必要性证明 (Proof of Necessity)**
提交的 PR 中若包含 `unsafe` 块，必须在代码上方提供注释形式的必要性证明：
1.  **Reason (原因)**：阐明为何安全 Rust 无法满足当前底层需求。
2.  **Irreplaceability (不可替代性)**：论证现有安全库无法实现相同功能。
3.  **Safety Boundaries (安全边界)**：清晰界定开发者向编译器承诺的内存不变量。

</details>

---

## 🏛️ 架构哲学 (Architecture)

<details>
<summary><strong>点击展开：深入了解驱动 GNA:AEFR 的硬核设计</strong></summary>

### 绅士调度器：算力层面的阶级隔离
> 在 v0.8+ 之后，我们不再信任操作系统的默认调度逻辑，因为它往往为了追求平庸的“公平”而牺牲了实时渲染的确定性。

*   **N-2 策略**：GNA 会强制探测物理核心数 `N`，并隔离出 `N-2` 个核心作为专属计算区。
    *   **Render 核心**：锁定 1 核心专门负责主线程 GUI 绘制。
    *   **Audio 核心**：锁定 1 核心作为系统音频避震空间，彻底杜绝 CPU 满载导致的爆音。
    *   **Compute Zone**：剩余核心全部交给计算 Worker，通过 Rayon 线程池榨干 Spine 骨骼蒙皮的物理性能。

*   **同步阻塞模型**：拒绝 Web 开发中那种廉价且不可控的“异步”概念。
    *   **空间对齐**：主线程在 Update 阶段同步等待结果，确保每一帧的视觉元素在时空上绝对对齐。
    *   **Work Stealing**：在低核心设备上，主线程将主动下场接管计算任务，确保算力利用率触及物理极限。

### GUI 设计准则
*   所有组件必须严格遵循 `egui` 的**即时模式 (Immediate Mode)** 哲学。

</details>

---

## ✉️ 联系我们 (Contact)
*   **GNA 作者邮箱**: `ExtraShiningWonder@gmail.com`

---
**“真正的黑客在代码中寻求秩序，真正的自由在编译后诞生。”**
—— **GNA:AEFR 执行委员会**
