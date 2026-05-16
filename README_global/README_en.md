[简体中文](../README.md) | [日本語](./README_jp.md) | [한국어](./README_kr.md)

<p align="center">
  <img src="resources/committee_logo.png" width="450" alt="GNA:AEFR comittee logo">
  <br>
  <b>Real hackers compile from source; real freedom breathes under AGPL.</b>
</p>

# GNA's Not AA : AEFR's Eternal Freedom & Rust-rendered

> **Executive Committee Announcement**: This project is inspired by the GNU spirit of freedom and is NOT an official GNU project. We have currently submitted an application for inclusion in the "FSF (Free Software Foundation) Free Software Directory."

## Our Rust purity is 99.7% higher than the GNU/Linux kernel!

**GNA:AEFR is a free software project upholding the spirit of the GNU Manifesto, dedicated to liberating the creative environment of Kivotos. We firmly believe: distributing software without delivering the source code to the community is an irresponsible act that strips users of their freedom.**

---

### ⚖️ License
*   **Versions prior to v0.8.3**: Released under the **GPL-3.0** license.
*   **v0.8.3 and subsequent versions**: To close the "cloud-based closed-source loophole," the project has officially migrated to and mandates the **AGPL-3.0** license.

---

## 🧭 Navigation: The Philosophy of GNA:AEFR

> **GNA:AEFR** is not a mediocre application designed to please the masses. Currently in a rapid iteration phase of its core kernel, it is a high-performance editor instance built on pure Rust logic. **In the world of computing, the shortest path is always the most invincible.**
> 
> If you seek universal compatibility and an "out-of-the-box" low barrier to entry, please use **AA**; if you pursue extreme execution efficiency, absolute creative freedom, and hardcore community-driven maintenance, welcome to the world of **GNA:AEFR**.

*   **Unofficial Fan-made**: A multi-platform, multi-threaded editor created for *Blue Archive* secondary creations, built entirely with pure Rust.
*   **Engine-less Architecture**: Rejects reliance on bloated commercial engines like Unity/Unreal; driven directly by the lightweight `egui` library for GPU rendering, straight to the metal.
*   **Cross-platform Sovereignty**: Native support for GNU/Linux, Android, macOS, and Windows, ensuring a consistent and superior experience across different operating systems.

### ✨ Implemented (Core Features)
- [x] **Dynamic Scene Reconstruction**: Supports real-time replacement of high-resolution scene backgrounds.
- [x] **Multi-dimensional Skeletal Rendering**: Supports simultaneous import and parallel rendering of up to 5 Spine skeletal animation files.
- [x] **Orthodox Visual Standards**: Perfectly recreates the standard Kivotos-style dialogue box rendering.
- [x] **Real-time Motion Dispatching**: Supports real-time switching of skeletal animation states (expressions, body movements, special effects).
- [x] **Asynchronous Audio System**: Supports asynchronous loading and seamless streaming of Background Music (BGM).

### 🎯 Roadmap
- [ ] **Non-linear Editing System**: Introducing professional-grade Timeline logic.
- [ ] **Smooth Tweening**: Implementing linear interpolation and smooth blending for character movements.
- [ ] **Scene Illustration System**: Supporting instant pop-up layered illustrations within scenes.
- [ ] **Transition Transcoder**: Developing various scene transition effects including fades and wipes.
- [ ] **Interactive Bubbles**: Dynamic tracking of emoji bubbles above characters' heads.

**We welcome any hacker who identifies with the ideals of free software to participate in the development of GNA:AEFR.**

---

## 🚀 Getting Started

> *"Release? Real hackers compile from source." ;-)*

Go to the [**Releases**](https://github.com/OxidizedSchale/GNA-AEFR/releases) page to get the latest source code, or download pre-compiled binaries for specific platforms.

**GNA:AEFR** utilizes an interaction model that deeply integrates a **Graphical User Interface (GUI)** with **Command-Driven** logic.

*   **Desktop**: The full graphical control panel is recommended.
*   **Mobile**: For external resource importing, precise scheduling via console commands is currently required.

Click the `[SHELL]` button in the top-left corner of the interface to invoke the built-in hardcore debugging console.

<details>
<summary><strong>📖 Click to expand: GNA:AEFR Command Reference Manual</strong></summary>

### 1. Visuals & Scenes

*   **Load Background**
    *   **Command**: `BG <image_path>`
    *   **Description**: Instantly switches the background. Supports `.jpg`, `.png`, and `.webp`.
    *   **Example**: `BG C:\Assets\BlueArchive\BG_Classroom.png`

*   **Load Character**
    *   **Command**: `LOAD <slot_ID> <.atlas_path>`
    *   **Description**: Loads Spine resources into slots `0` through `4`. Upon success, the SHELL returns the full list of available animations for that character.
    *   **Example**: `LOAD 0 D:\Assets\Shiroko\Shiroko_Home.atlas`

### 2. Motion & Performance

*   **Switch Animation**
    *   **Command**: `ANIM <slot_ID> <animation_name> [loop: true/false]`
    *   **Description**: Switches the animation for the character in the specified slot. `true` for forced looping, `false` for single play. The animation name must exactly match the list returned during loading.
    *   **Example**:
        ```bash
        ANIM 0 Start_Idle_01 true    # Shiroko loops idle
        ANIM 1 Attack_Normal false   # Character in slot 1 performs a single attack
        ```

### 3. Storytelling & Dialogue

*   **Send Dialogue**
    *   **Command**: `TALK <name>|<affiliation>|<content>`
    *   **Description**: Renders a standard dialogue box with an integrated typewriter effect. **Parameters must be separated by the pipe character `|`.**
    *   **Example**:
        ```bash
        TALK Shiroko|Abydos|Sensei, are we going to rob a bank?
        TALK Arona|Schale|Sensei, please don't slack off during work hours!
        ```

### 4. Audio System

*   **Play BGM**
    *   **Command**: `BGM <audio_path>`
    *   **Description**: Dispatches the audio engine to asynchronously load and loop an audio file.
    *   **Example**: `BGM D:\Music\Unwelcome_School.mp3`

*   **Stop Music**
    *   **Command**: `STOP`
    *   **Description**: Forcefully terminates the currently active audio output stream.

</details>

---

### 💡 Pro Tips

*   **Path Pre-processing**: Windows users can paste file paths directly; GNA automatically handles quotes and escape characters. Android/Termux users must ensure they use absolute paths starting with `/sdcard/`.
*   **Scheduler Guarantee**: Thanks to the unique "Gentleman's Scheduler," the UI thread is physically isolated. Even with 5 characters on screen at full computational load, the interface remains absolutely buttery smooth.
*   **Real-time Auditing**: It is recommended to keep the SHELL window visible to audit resource loading states and animation parsing logs in real-time.

---

## 🤝 Contribution Guide

<details>
<summary><strong>Click to expand: GNA Executive Committee Contributor Covenant</strong></summary>

### Tech Stack Purity
This project strictly prohibits any behavior that undermines architectural purity. Core business logic must be 100% implemented in **Rust**.

*   **Principled Rejection**: Any proposal that introduces complex JNI/FFI interactions with C++ runtimes or heavyweight frameworks (e.g., Qt, Unity).
*   **Exceptional Permission**: Only allowed for Rust safety wrappers around low-level system C libraries (Graphics APIs, Audio backends). In such cases:
    *   Mature community `-sys` bindings must be prioritized.
    *   If `unsafe` calls must be written, the safety proof standards below must be strictly followed.
    *   The ultimate goal must be to completely decouple non-safe calls and provide a transparent, safe Rust API.

### Unsafe Guidelines: Constraints on the Blade of Performance
> `unsafe` is the performance blade Rust grants to developers. Our principle is: "Do not use unless necessary; if used, it must be foolproof."

**Core Principle: Proof of Necessity**
Any PR containing `unsafe` blocks must provide a proof of necessity in the comments above the code:
1.  **Reason**: Clarify why safe Rust cannot meet the current low-level requirement.
2.  **Irreplaceability**: Demonstrate that existing safe libraries cannot achieve the same functionality.
3.  **Safety Boundaries**: Clearly define the memory invariants the developer is promising to the compiler.

</details>

---

## 🏛️ Architecture Philosophy

<details>
<summary><strong>Click to expand: Deep dive into the hardcore design driving GNA:AEFR</strong></summary>

### Gentleman's Scheduler: Class Segregation at the Computing Level
> After v0.8+, we no longer trust the operating system's default scheduling logic, as it often sacrifices real-time rendering determinism for the sake of mediocre "fairness."

*   **N-2 Strategy**: GNA forcibly detects the number of physical cores `N` and isolates `N-2` cores as a dedicated computing zone.
    *   **Render Core**: Locks 1 core exclusively for main-thread GUI drawing.
    *   **Audio Core**: Locks 1 core as a system audio "buffer space" to completely eliminate audio crackling caused by CPU spikes.
    *   **Compute Zone**: All remaining cores are handed over to computational Workers, squeezing every drop of physical performance out of Spine skeletal skinning via a Rayon thread pool.

*   **Synchronous Blocking Model**: Rejects the cheap and uncontrollable "asynchrony" found in Web development.
    *   **Spatial Alignment**: The main thread waits synchronously for results during the Update phase, ensuring that every frame's visual elements are absolutely aligned in space-time.
    *   **Work Stealing**: On low-core devices, the main thread will actively step in to take over computing tasks, ensuring power utilization hits physical limits.

### GUI Design Principles
*   All components must strictly follow the **Immediate Mode** philosophy of `egui`.

</details>

---

## ✉️ Contact Us
*   **GNA Author Email**: `ExtraShiningWonder@gmail.com`

---
**"Real hackers seek order in code; real freedom is born after compilation."**
—— **GNA:AEFR Executive Committee**
