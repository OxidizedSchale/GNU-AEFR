[简体中文](../README.md) | [日本語](./README_jp.md) | [한국어](./README_kr.md)

# GNA's Not AA : AEFR's Eternal Freedom & Rust-rendered

> Inspired by GNU, but not an official GNU project. We have applied for inclusion in the "Free Software Foundation Free Software Directory."

## Our Rust purity is 99.7% higher than the GNU/Linux kernel!

**GNA:AEFR is a free software project upholding the spirit of the GNU Manifesto, dedicated to liberating the creative environment of Kivotos! Distributing software without its source code is an irresponsible act!**

---

### ​⚖️ License
This project is and will only be released under the **GPL-3.0** license (for versions prior to 0.8.3).

For version 0.8.3 and beyond, the **AGPL-3.0** license must be used.

---

## 🧭 Navigation: The Philosophy of GNA:AEFR

> GNA:AEFR is not currently a complex application designed to please everyone, as a complete version has yet to be finalized. It is simply a high-performance editor instance built entirely in pure Rust. **In the world of computing, the shortest path is always the most invincible!** If you seek current compatibility and ease of use, please use AA; if you pursue ultimate freedom, performance, and community-driven maintenance, welcome to **GNA:AEFR**.

*   **Unofficial Fan-made**: A high-performance, multi-platform, multi-threaded "Blue Archive" secondary creation editor built with pure Rust.
*   **No Game Engine**: We do not rely on Unity/Unreal; the graphical interface is driven directly by the lightweight `egui` library.
*   **Cross-platform Dominance**: Native support for GNU/Linux, Android, macOS, and Windows.

### ✨ Currently Implemented
- [x] Dynamic background scene switching.
- [x] Concurrent import and rendering of up to 5 Spine skeletal animation files.
- [x] Support for standard Kivotos-style dialog box rendering.
- [x] Real-time animation switching (e.g., expressions, actions).
- [x] Asynchronous loading and playback of Background Music (BGM).

### 🎯 Future Plans
- [ ] Linear editing system (Timeline).
- [ ] Smooth transitions and blending of character actions.
- [ ] In-scene image pop-ups (e.g., illustrations/CG).
- [ ] Scene transition effects (fade-in/out, wipes, etc.).
- [ ] Character overhead emote bubbles.

**We welcome any aspiring developers to join the development of GNA:AEFR!**

---

## 🚀 Getting Started

> "Release? Real hackers compile from source." ;-)

Go to the [**Releases**](https://github.com/OxidizedSchale/GNA-AEFR/releases) page to download the source code, or directly obtain the pre-compiled binaries (without instruction set optimization) for your platform.

**GNA:AEFR** employs a combination of **Graphical User Interface (GUI)** and **Command-Driven** interaction.

*   **Desktop**: Graphical interface is recommended.
*   **Mobile**: For file imports, command-driven interaction is currently required.

Click the `[CMD]` button in the top-left corner of the interface to open the built-in debug console (with GUI support).

---

## 📖 Command Reference Manual

### 1. Scene and Visuals

*   **Load Background**
    *   **Command**: `BG <image_path>`
    *   **Description**: Instantly switch the background image. Supports `.jpg`, `.png`, and `.webp`.
    *   **Example**: `BG C:\Assets\BlueArchive\BG_Classroom.png`

*   **Load Character**
    *   **Command**: `LOAD <slot_ID> <.atlas_path>`
    *   **Description**: Load a character into slots `0` through `4` (5 slots total). Supports automatic layout. Upon success, the console will print a list of available animations for that character.
    *   **Example**: `LOAD 0 D:\Assets\Shiroko\Shiroko_Home.atlas`

### 2. Animation and Performance

*   **Switch Animation**
    *   **Command**: `ANIM <slot_ID> <animation_name> [loop: true/false]`
    *   **Description**: `true` for looping, `false` for a single playback. Animation names must match exactly (refer to the list printed upon loading).
    *   **Example**:
        ```bash
        ANIM 0 Start_Idle_01 true    # Starts Shiroko's idle loop
        ANIM 1 Attack_Normal false   # Slot 1 character attacks once
        ```

### 3. Script and Dialog

*   **Send Dialog**
    *   **Command**: `TALK <name>|<affiliation>|<content>`
    *   **Description**: Renders a standard Kivotos-style dialog box with a typewriter effect (click the dialog box to skip). **Parameters must be separated by the vertical bar `|`.**
    *   **Example**:
        ```bash
        TALK Shiroko|Abydos|Sensei, are we going to rob a bank?
        TALK Arona|Schale|Sensei, please do not slack off during work hours!
        ```

### 4. Audio System

*   **Play BGM**
    *   **Command**: `BGM <audio_path>`
    *   **Description**: Asynchronously loads and plays background music with seamless switching.
    *   **Example**: `BGM D:\Music\Unwelcome_School.mp3`

*   **Stop Music**
    *   **Command**: `STOP`
    *   **Description**: Immediately stops the currently playing BGM.

---

### 💡 Geek Tips

*   **Path Issues**: On Windows, it is recommended to copy the file path directly; AEFR automatically handles quotes (e.g., `"C:\Path"`). On Android/Termux, please use absolute paths, such as `/sdcard/Download/bg.png`.
*   **Performance Monitoring**: Thanks to the "Gentleman Scheduler," the UI thread remains silky smooth even when all 5 slots are occupied and BGM is playing. Feel free to multitask.
*   **Log Feedback**: All operation results (success/failure/available animations) are printed in real-time in the Log area of the console.

---

## 🤝 Contribution Guidelines

<details>
<summary><strong>Click to expand: Mandatory rules for contributing to GNA:AEFR</strong></summary>

### Tech Stack Purity
This project insists that core business logic and architecture be implemented 100% in **Rust**.

*   **Rejection of C++ Interop**: We generally refuse Pull Requests that introduce complex interactions (JNI/Complex FFI) with C++ runtimes or frameworks (e.g., Qt, Unity, Unreal) to maintain architectural purity and maintainability.
*   **Exceptions**: Safe Rust wrappers for existing, functional, system-level C libraries (graphics, audio, filesystem foundations) are permitted. In such cases:
    *   Community-maintained, mature `-sys` binding libraries must be prioritized.
    *   If `unsafe` code for FFI calls is necessary, it must strictly adhere to the `unsafe` guidelines below and its necessity must be proven.
    *   The ultimate goal is to encapsulate all `unsafe` calls within safe Rust APIs, making them transparent to the upper application layer.

### Unsafe Guidelines: The Blade of Performance Above Safety
> The `unsafe` keyword is the power granted by Rust to developers to perform necessary low-level operations outside the compiler's safety rules. This project views it as a "Blade of Performance Above Safety." The principle is: "Do not use it unless necessary; if used, it must be foolproof."

**Core Principle: Proof of Necessity**

The existence of any `unsafe` block must be justified by a valid reason that cannot be achieved through safe Rust. In a PR, you must provide a concise "Proof of Necessity" in the form of a comment above the `unsafe` block, including:

1.  **Reason**: Explicitly state why `unsafe` is required (e.g., calling specific C FFI functions, deterministic memory layout conversion, implementing self-referential structures, etc.).
2.  **Irreplaceability**: Argue why the same functionality cannot be achieved using the Rust standard library or existing community crates.
3.  **Safety Boundary**: Clearly define the invariants that this `unsafe` block promises to maintain. As a developer, what conditions are you "promising" the compiler are true to make this code logically safe?

*Specific Requirement: Documentation Comments*
*   Every `unsafe` function, method, or block must be preceded by a safety comment.

</details>

---

## 🏛️ Architectural Philosophy

<details>
<summary><strong>Click to expand: The hardcore low-level design driving GNA:AEFR</strong></summary>

### Gentleman Scheduler: Class Isolation at the Computing Level
> In GNA:AEFR v0.8+, we introduced our self-developed "Gentleman Scheduler." We do not trust the default scheduling of operating systems, as they often sacrifice the determinism of real-time rendering for so-called "fairness."

*   **Core Guideline: N-2 Strategy**
    *   GNA:AEFR refuses to be a "CPU-hogging beast." The scheduler automatically detects the number of physical cores `N` and forcibly isolates `N-2` cores as a dedicated computing zone (if only two cores/threads are available, AEFR will occupy one).
    *   **1 Core** Reserved for UI/Render (Main Thread): Ensures UI remains smooth even during heavy computation.
    *   **1 Core** Reserved for OS/Audio (Backstage): Acts as a "buffer zone" for the underlying system, completely eliminating audio crackling or stuttering caused by CPU saturation.
    *   Remaining cores are allocated to **GNA:AEFR** Workers: Utilizing the Rayon thread pool for Spine bone skinning and physics calculations.

*   **Why Insist on a "Synchronous Blocking Model"?**
    > Do not try to sell us the cheap "non-blocking async" concepts common in Web development. In the philosophy of GNA:AEFR, "Synchronous" is "Order."

    *   **Zero Screen Tearing**: The main thread synchronously waits for calculation results during the Update phase. This ensures that bone positions, expressions, and physics effects are perfectly aligned in space for every frame.
    *   **Work Stealing Mechanism**: Thanks to Rayon's hardcore implementation, in extreme cases with few cores (e.g., dual-core machines), the main thread will actively participate in calculations to ensure power utilization reaches 100% of the physical limit.
    *   **Scheduler Warning**: Using `std::thread::spawn` for any computation-intensive task is strictly prohibited. All parallel tasks must be distributed via the `AefrScheduler`. Violators cause the system scheduling to fall into unordered competition—a betrayal of the GNA:AEFR Rust aesthetic.

### GUI Improvement Rules
*   UI components must follow the **Immediate Mode** philosophy of `egui`. Proposals following `React`-style declarative state management are not accepted.

</details>

---

## ✉️ Contact Us
*   **Author Email**: ExtraShiningWonder@gmail.com
