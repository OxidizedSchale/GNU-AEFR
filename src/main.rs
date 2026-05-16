/*
 * GNA:AEFR (GNA's Not AA:AEFR's Eternal Freedom & Rust-rendered)
 * Copyright (C) 2026 OxidizedSchale & The Executive Committee of GNA: AEFR
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License.
 *
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * 
 * GitHub: https://github.com/OxidizedSchale/GNA-AEFR
 *
 * 版权所有 (C) 2026 OxidizedSchale & The Executive Committee of GNA: AEFR
 *
 * 本程序是自由软件：您可以自由分发和/或修改它。
 * 它遵循由自由软件基金会（Free Software Foundation）发布的
 * GNU Affero 通用公共许可证（GNU Affero General Public License）第 3 版。
 * 本程序的 git 仓库应带有 AGPL3 许可证，请自行查看
 */

// 全局禁用 Rust 的大傻逼警告 (后续逐步删去)
#![allow(warnings)]

use eframe::egui;
use egui::{
    epaint::Vertex, Color32, FontData, FontDefinitions, FontFamily, Mesh, Pos2, Rect, Shape,
    TextureHandle, TextureId, Vec2, Stroke, RichText, Rounding, Visuals,
};
use rayon::prelude::*;
use rusty_spine::{
    AnimationState, AnimationStateData, Atlas, Skeleton, SkeletonJson, SkeletonBinary, Slot,
};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use rodio::Source;
use serde::{Serialize, Deserialize};

// ============================================================================
// 常量定义：消除魔法数字，确立排版秩序
// ============================================================================
const BASE_HEIGHT: f32 = 720.0;               
const DIALOGUE_BOX_RATIO: f32 = 0.28;         
const MAX_DT: f32 = 0.033;                    
const TYPEWRITER_INTERVAL: f32 = 0.03;        // 打字机字符间隔
const CHAR_BASE_SCALE: f32 = 0.45;            
const CHAR_X_START_PERCENT: f32 = 0.15;       
const CHAR_X_STEP_PERCENT: f32 = 0.175;       

#[derive(Serialize, Deserialize, Clone, Default)]
struct Scene {
    bg_path: Option<String>,              
    bgm_path: Option<String>,             
    char_paths:[Option<String>; 5],       
    char_anims:[Option<String>; 5],       
    speaker_name: String,                 
    speaker_aff: String,                  
    dialogue_content: String,             
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Scenario {
    scenes: Vec<Scene>,                   
}

// ============================================================================
// 程序入口点
// ============================================================================
#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])      
            .with_title("GNA's Not AA : AEFR's Eternal Freedom & Rust_rendered"),
        vsync: true,                               
        ..Default::default()
    };
    eframe::run_native("AEFR_App", options, Box::new(|cc| Box::new(AefrApp::new(cc))))
}

#[cfg(target_os = "android")]
fn main() -> eframe::Result<()> {
    eframe::run_native("AEFR_App", eframe::NativeOptions::default(), Box::new(|cc| Box::new(AefrApp::new(cc))))
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let _ = eframe::run_native("AEFR_App", eframe::NativeOptions::default(), Box::new(|cc| Box::new(AefrApp::new(cc))));
}

// ============================================================================
// 异步文件选择器 (桌面端隔离 RFD)
// ============================================================================
#[cfg(not(target_os = "android"))]
mod file_picker {
    use super::*;
    pub fn save_scenario(tx: Sender<AppCommand>, json_data: String) {
        thread::spawn(move || {
            if let Some(p) = rfd::FileDialog::new().set_file_name("scenario.json").save_file() {
                if std::fs::write(&p, json_data).is_ok() {
                    let _ = tx.send(AppCommand::Log(format!("[系统] 剧本已保存: {}", p.display())));
                }
            }
        });
    }
    pub fn load_scenario(tx: Sender<AppCommand>) {
        thread::spawn(move || {
            if let Some(p) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
                if let Ok(data) = std::fs::read_to_string(&p) {
                    if let Ok(s) = serde_json::from_str::<Scenario>(&data) {
                        let _ = tx.send(AppCommand::ScenarioLoaded(s));
                    }
                }
            }
        });
    }
    pub fn pick_spine(tx: Sender<AppCommand>, slot: usize) {
        thread::spawn(move || {
            if let Some(p) = rfd::FileDialog::new().add_filter("Atlas", &["atlas"]).pick_file() {
                let _ = tx.send(AppCommand::RequestLoad { slot_idx: slot, path: p.display().to_string() });
            }
        });
    }
    pub fn pick_bg(tx: Sender<AppCommand>) {
        thread::spawn(move || {
            if let Some(p) = rfd::FileDialog::new().add_filter("Images", &["png", "jpg"]).pick_file() {
                let _ = tx.send(AppCommand::LoadBackground(p.display().to_string()));
            }
        });
    }
    pub fn pick_bgm(tx: Sender<AppCommand>) {
        thread::spawn(move || {
            if let Some(p) = rfd::FileDialog::new().add_filter("Audio", &["mp3", "wav", "ogg"]).pick_file() {
                let _ = tx.send(AppCommand::PlayBgm(p.display().to_string()));
            }
        });
    }
    pub fn pick_se(tx: Sender<AppCommand>) {
        thread::spawn(move || {
            if let Some(p) = rfd::FileDialog::new().add_filter("Audio", &["mp3", "wav", "ogg"]).pick_file() {
                let _ = tx.send(AppCommand::PlaySe(p.display().to_string()));
            }
        });
    }
}

#[cfg(target_os = "android")]
mod file_picker {
    use super::*;
    pub fn save_scenario(tx: Sender<AppCommand>, _json_data: String) { let _ = tx.send(AppCommand::Log("[系统] 安卓端请使用指令保存".into())); }
    pub fn load_scenario(tx: Sender<AppCommand>) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓 SAF...".into())); }
    pub fn pick_spine(tx: Sender<AppCommand>, _slot: usize) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓文件选择器...".into())); }
    pub fn pick_bg(tx: Sender<AppCommand>) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓文件选择器...".into())); }
    pub fn pick_bgm(tx: Sender<AppCommand>) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓文件选择器...".into())); }
    pub fn pick_se(tx: Sender<AppCommand>) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓文件选择器...".into())); }
}

// ============================================================================
// 核心架构组件
// ============================================================================
struct AefrScheduler { pool: rayon::ThreadPool }
impl AefrScheduler {
    fn new() -> Self {
        let logic_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        Self { pool: rayon::ThreadPoolBuilder::new().num_threads(if logic_cores > 2 { logic_cores - 2 } else { 1 }).build().unwrap() }
    }
    fn run_parallel<OP>(&self, op: OP) where OP: FnOnce() + Send { self.pool.install(op); }
}

enum AppCommand {
    Dialogue { name: String, affiliation: String, content: String },
    RequestLoad { slot_idx: usize, path: String },
    LoadSuccess(usize, Box<SpineObject>, egui::ColorImage, String, Vec<String>),
    RemoveCharacter(usize),
    LoadBackground(String),
    LoadBackgroundSuccess(egui::ColorImage),
    PlayBgm(String),
    PlaySe(String),
    AudioReady(Vec<u8>, bool),  
    StopBgm,
    SetAnimation { slot_idx: usize, anim_name: String, loop_anim: bool },
    Log(String),
    ScenarioLoaded(Scenario),
}

struct AudioManager {
    _stream: rodio::OutputStream,           
    _stream_handle: rodio::OutputStreamHandle, 
    bgm_sink: rodio::Sink,                  
    se_sink: rodio::Sink,                   
}
impl AudioManager {
    fn new() -> Result<Self, String> {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().map_err(|e| e.to_string())?;
        let bgm_sink = rodio::Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;
        let se_sink = rodio::Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;
        Ok(Self { _stream, _stream_handle: stream_handle, bgm_sink, se_sink })
    }
    fn play_bgm(&self, data: Vec<u8>) {
        if let Ok(source) = rodio::Decoder::new(Cursor::new(data)) {
            self.bgm_sink.stop();  
            self.bgm_sink.append(source.repeat_infinite()); 
            self.bgm_sink.play();
        }
    }
    fn play_se(&self, data: Vec<u8>) {
        if let Ok(source) = rodio::Decoder::new(Cursor::new(data)) { self.se_sink.append(source); self.se_sink.play(); }
    }
    fn stop_bgm(&self) { self.bgm_sink.stop(); }
}

// ============================================================================
// Spine 2D骨骼动画渲染对象
// ============================================================================
pub struct SpineObject {
    pub position: Pos2,                     
    pub scale: f32,                         
    _texture: Option<TextureHandle>,        
    texture_id: Option<TextureId>,          
    world_vertices: Vec<f32>,
    skeleton: Skeleton,                     
    state: AnimationState,                  
    _state_data: Arc<AnimationStateData>,   
    _skeleton_data: Arc<rusty_spine::SkeletonData>, 
    _atlas: Arc<Atlas>,                     
}

// 【必要性证明】: 采用 Arc<Mutex<T>> 包装，确保并行蒙皮计算时数据竞争物理隔绝。
unsafe impl Send for SpineObject {}

impl SpineObject {
    fn load_async_no_gpu(path_str: &str) -> Result<(Self, egui::ColorImage, String, Vec<String>), String> {
        let atlas = Arc::new(Atlas::new_from_file(std::path::Path::new(path_str)).map_err(|e| format!("Atlas Error: {}", e))?);
        let page = atlas.pages().next().ok_or("Atlas has no pages")?;
        let page_name = page.name().to_string();
        let img_path = std::path::Path::new(path_str).parent().ok_or("Invalid path")?.join(&page_name);
        let img = image::open(&img_path).map_err(|e| format!("Image Load Error: {}", e))?;
        let rgba = img.to_rgba8();
        let color_image = egui::ColorImage::from_rgba_unmultiplied([rgba.width() as _, rgba.height() as _], &rgba.into_raw());

        let skel_path = std::path::Path::new(path_str).with_extension("skel");
        let json_path = std::path::Path::new(path_str).with_extension("json");
        let skeleton_data = if skel_path.exists() {
            let skeleton_bin = SkeletonBinary::new(atlas.clone());
            Arc::new(skeleton_bin.read_skeleton_data_file(&skel_path).map_err(|e| format!("Binary load failed: {}", e))?)
        } else {
            let skeleton_json = SkeletonJson::new(atlas.clone());
            Arc::new(skeleton_json.read_skeleton_data_file(&json_path).map_err(|e| format!("JSON load failed: {}", e))?)
        };

        let state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
        let mut state = AnimationState::new(state_data.clone());
        let anim_names: Vec<String> = skeleton_data.animations().map(|a| a.name().to_string()).collect();
        if let Some(anim) = skeleton_data.animations().next() { let _ = state.set_animation(0, &anim, true); }

        Ok((Self { 
            position: Pos2::ZERO, scale: CHAR_BASE_SCALE, _texture: None, texture_id: None, 
            world_vertices: Vec::with_capacity(8192),
            skeleton: Skeleton::new(skeleton_data.clone()), state, _state_data: state_data, _skeleton_data: skeleton_data, _atlas: atlas,
        }, color_image, page_name, anim_names))
    }

    fn get_anim_names(&self) -> Vec<String> { self._skeleton_data.animations().map(|a| a.name().to_string()).collect() }
    fn set_animation_by_name(&mut self, anim_name: &str, loop_anim: bool) -> bool {
        if let Some(anim) = self._skeleton_data.animations().find(|a| a.name() == anim_name) {
            let _ = self.state.set_animation(0, &anim, loop_anim); true
        } else { false }
    }
    fn update_parallel(&mut self, dt: f32) {
        let dt = dt.min(MAX_DT);  
        self.state.update(dt);                     
        self.skeleton.set_to_setup_pose();         
        let _ = self.state.apply(&mut self.skeleton); 
        self.skeleton.update_world_transform();    
        self.skeleton.update_cache();              
    }
    fn paint(&mut self, ui: &mut egui::Ui) {
        let tex_id = match self.texture_id { Some(id) => id, None => return };
        let mut mesh = Mesh::with_texture(tex_id);
        for slot in self.skeleton.draw_order() {
            let attachment = match slot.attachment() { Some(a) => a, None => continue };
            if let Some(region) = attachment.as_region() {
                unsafe {
                    if self.world_vertices.len() < 8 { self.world_vertices.resize(8, 0.0); }
                    region.compute_world_vertices(&slot.bone(), &mut self.world_vertices, 0, 2);
                    self.push_to_mesh(&mut mesh, &self.world_vertices[0..8], &region.uvs(), &[0, 1, 2, 2, 3, 0], &*slot, region.color());
                }
            } else if let Some(mesh_att) = attachment.as_mesh() {
                unsafe {
                    let len = mesh_att.world_vertices_length() as usize;
                    if self.world_vertices.len() < len { self.world_vertices.resize(len, 0.0); }
                    mesh_att.compute_world_vertices(&*slot, 0, len as i32, &mut self.world_vertices, 0, 2);
                    let uvs = std::slice::from_raw_parts(mesh_att.uvs(), len);
                    let tris = std::slice::from_raw_parts(mesh_att.triangles(), mesh_att.triangles_count() as usize);
                    self.push_to_mesh(&mut mesh, &self.world_vertices[0..len], uvs, tris, &*slot, mesh_att.color());
                }
            }
        }
        ui.painter().add(Shape::mesh(mesh));
    }
    fn push_to_mesh(&self, mesh: &mut Mesh, w_v: &[f32], uvs: &[f32], tris: &[u16], slot: &Slot, att_c: rusty_spine::Color) {
        let s_c = slot.color();      
        let a = s_c.a * att_c.a;     
        let (r, g, b) = (s_c.r * att_c.r * a, s_c.g * att_c.g * a, s_c.b * att_c.b * a);
        let final_a = match slot.data().blend_mode() { rusty_spine::BlendMode::Additive => 0.0, _ => a };
        let color = Color32::from_rgba_premultiplied((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (final_a * 255.0) as u8);
        let count = usize::min(uvs.len() / 2, w_v.len() / 2);
        let idx_offset = mesh.vertices.len() as u32;  
        for i in 0..count {
            let pos = Pos2::new(w_v[i*2] * self.scale + self.position.x, -w_v[i*2+1] * self.scale + self.position.y);
            mesh.vertices.push(Vertex { pos, uv: Pos2::new(uvs[i*2], uvs[i*2+1]), color });
        }
        for &idx in tris { mesh.indices.push(idx_offset + idx as u32); }
    }
}

// ============================================================================
// 主应用程序逻辑 (AEFR_App)
// ============================================================================
struct AefrApp {
    scheduler: AefrScheduler,      
    audio_manager: Option<AudioManager>, 
    scenario: Scenario,            
    current_scene_idx: usize,      
    target_chars: Vec<char>,       
    visible_count: usize,          
    type_timer: f32,               
    is_auto_enabled: bool,         
    show_dialogue: bool,           
    console_open: bool,            
    selected_slot: usize,          
    console_input: String,         
    console_logs: Vec<String>,     
    show_anim_preview: bool,       
    preview_anim_idx: usize,       
    characters: Vec<Option<Arc<Mutex<SpineObject>>>>, 
    background: Option<TextureHandle>, 
    tx: Sender<AppCommand>,        
    rx: Receiver<AppCommand>,      
}

impl AefrApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        setup_embedded_font(&cc.egui_ctx);
        setup_custom_theme(&cc.egui_ctx); // 🌟 注入极致高对比度包豪斯主题
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let (tx, rx) = channel();
        let audio_manager = AudioManager::new().ok();
        
        let startup_text = "GNA:AEFR 已启动！\n正在等待指令......";
        let mut first_scene = Scene::default();
        first_scene.speaker_name = "OxidizedSchale".into();
        first_scene.speaker_aff = "The Executive Committee of GNA:AEFR".into();
        first_scene.dialogue_content = startup_text.into();

        Self {
            scheduler: AefrScheduler::new(), is_auto_enabled: true, show_dialogue: true,
            scenario: Scenario { scenes: vec![first_scene] }, current_scene_idx: 0,
            target_chars: startup_text.chars().collect(), visible_count: 0, type_timer: 0.0,
            console_open: false, selected_slot: 0, console_input: String::new(),
            console_logs: vec!["[系统] 编辑器就绪。".into()],
            show_anim_preview: false, preview_anim_idx: 0,
            characters: (0..5).map(|_| None).collect(), background: None,
            audio_manager, tx, rx,
        }
    }

    fn sync_scene_to_ui(&mut self) {
        if let Some(scene) = self.scenario.scenes.get(self.current_scene_idx) {
            self.target_chars = scene.dialogue_content.chars().collect();
            self.visible_count = 0; self.type_timer = 0.0;
        }
    }

    fn parse_and_send_command(&mut self, input: &str) {
        let input_trimmed = input.trim(); if input_trimmed.is_empty() { return; }
        self.console_logs.push(format!("> {}", input_trimmed));
        let tx = self.tx.clone(); let cmd_lower = input_trimmed.to_lowercase();

        if cmd_lower.starts_with("load ") {
            let parts: Vec<&str> = input_trimmed.splitn(2, ' ').collect();
            if parts.len() == 2 { if let Ok(idx) = parts[0][5..].trim().parse::<usize>() { let _ = tx.send(AppCommand::RequestLoad { slot_idx: idx, path: parts[1].replace("\"", "") }); } }
        } else if cmd_lower.starts_with("anim ") {
            let parts: Vec<&str> = input_trimmed.split_whitespace().collect();
            if parts.len() >= 2 { if let Ok(idx) = parts[1].parse::<usize>() { let anim_name = parts[2].to_string(); let loop_anim = parts.get(3).map_or(true, |s| s.to_lowercase() == "true"); let _ = tx.send(AppCommand::SetAnimation { slot_idx: idx, anim_name, loop_anim }); } }
        } else if cmd_lower.starts_with("bgm ") { let _ = tx.send(AppCommand::PlayBgm(input_trimmed[4..].trim().replace("\"", ""))); }
        else if cmd_lower.starts_with("se ") { let _ = tx.send(AppCommand::PlaySe(input_trimmed[3..].trim().replace("\"", ""))); }
        else if cmd_lower == "stop" { let _ = tx.send(AppCommand::StopBgm); }
        else if cmd_lower.starts_with("talk ") {
            let p: Vec<&str> = input_trimmed[5..].split('|').collect();
            if p.len() == 3 { let _ = tx.send(AppCommand::Dialogue { name: p[0].into(), affiliation: p[1].into(), content: p[2].into() }); }
        } else if cmd_lower.starts_with("bg ") { let _ = tx.send(AppCommand::LoadBackground(input_trimmed[3..].trim().replace("\"", ""))); }
    }

    fn handle_async_events(&mut self, ctx: &egui::Context) {
        while let Ok(cmd) = self.rx.try_recv() {
            match cmd {
                AppCommand::Dialogue { name, affiliation, content } => {
                    let scene = &mut self.scenario.scenes[self.current_scene_idx];
                    scene.speaker_name = name; scene.speaker_aff = affiliation; scene.dialogue_content = content;
                    self.sync_scene_to_ui();  
                }
                AppCommand::Log(msg) => self.console_logs.push(msg),
                AppCommand::RequestLoad { slot_idx, path } => {
                    let tx_cb = self.tx.clone(); self.console_logs.push(format!("[解析] {}", path));
                    let path_clone = path.clone();
                    thread::spawn(move || {
                        if let Ok((obj, img, page, anims)) = SpineObject::load_async_no_gpu(&path_clone) { 
                            let _ = tx_cb.send(AppCommand::LoadSuccess(slot_idx, Box::new(obj), img, page, anims)); 
                        }
                    });
                }
                AppCommand::LoadSuccess(idx, obj, img, page, anims) => {
                    if let Some(slot) = self.characters.get_mut(idx) {
                        let mut loaded = *obj; let handle = ctx.load_texture(page, img, egui::TextureOptions::LINEAR);
                        loaded.texture_id = Some(handle.id()); loaded._texture = Some(handle);
                        *slot = Some(Arc::new(Mutex::new(loaded)));
                    }
                }
                AppCommand::RemoveCharacter(idx) => { self.characters[idx] = None; }
                AppCommand::LoadBackground(path) => {
                    let tx_cb = self.tx.clone(); let path_clone = path.clone();
                    thread::spawn(move || { if let Ok(img) = image::open(&path_clone) {
                        let c_img = egui::ColorImage::from_rgba_unmultiplied([img.width() as _, img.height() as _], img.to_rgba8().as_raw());
                        let _ = tx_cb.send(AppCommand::LoadBackgroundSuccess(c_img));
                    }});
                    self.scenario.scenes[self.current_scene_idx].bg_path = Some(path);
                }
                AppCommand::LoadBackgroundSuccess(c_img) => { self.background = Some(ctx.load_texture("bg", c_img, egui::TextureOptions::LINEAR)); }
                AppCommand::SetAnimation { slot_idx, anim_name, loop_anim } => {
                     if let Some(Some(char_arc)) = self.characters.get(slot_idx) { if let Ok(mut char) = char_arc.lock() { let _ = char.set_animation_by_name(&anim_name, loop_anim); } }
                }
                AppCommand::PlayBgm(path) => {
                    let tx_cb = self.tx.clone(); let path_clone = path.clone();
                    thread::spawn(move || { if let Ok(d) = std::fs::read(&path_clone) { let _ = tx_cb.send(AppCommand::AudioReady(d, true)); } });
                    self.scenario.scenes[self.current_scene_idx].bgm_path = Some(path);
                }
                AppCommand::PlaySe(path) => {
                    let tx_cb = self.tx.clone(); let path_clone = path.clone();
                    thread::spawn(move || { if let Ok(d) = std::fs::read(&path_clone) { let _ = tx_cb.send(AppCommand::AudioReady(d, false)); } });
                }
                AppCommand::AudioReady(data, is_bgm) => { if let Some(mgr) = &self.audio_manager { if is_bgm { mgr.play_bgm(data); } else { mgr.play_se(data); } } }
                AppCommand::StopBgm => { if let Some(mgr) = &self.audio_manager { mgr.stop_bgm(); } }
                AppCommand::ScenarioLoaded(s) => { self.scenario = s; self.current_scene_idx = 0; self.sync_scene_to_ui(); self.visible_count = self.target_chars.len(); }
            }
        }
    }
}

impl eframe::App for AefrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_async_events(ctx);
        let dt = ctx.input(|i| i.stable_dt);
        
        if self.show_dialogue && self.visible_count < self.target_chars.len() {
            self.type_timer += dt;
            while self.type_timer >= TYPEWRITER_INTERVAL { self.visible_count += 1; self.type_timer -= TYPEWRITER_INTERVAL; }
        }

        let screen = ctx.screen_rect();
        let scale_factor = screen.height() / BASE_HEIGHT;
        
        for (i, slot) in self.characters.iter().enumerate() {
            if let Some(char_arc) = slot { if let Ok(mut char) = char_arc.lock() {
                char.scale = CHAR_BASE_SCALE * scale_factor;
                char.position = Pos2::new(screen.width() * (CHAR_X_START_PERCENT + (i as f32 * CHAR_X_STEP_PERCENT)), screen.bottom() + (30.0 * scale_factor));
            }}
        }

        self.scheduler.run_parallel(|| { self.characters.par_iter().for_each(|slot| { if let Some(char_arc) = slot { if let Ok(mut char) = char_arc.lock() { char.update_parallel(dt); } } }); });

        egui::CentralPanel::default().frame(egui::Frame::none().fill(Color32::BLACK)).show(ctx, |ui| {
            let rect = ui.max_rect();
            if let Some(bg) = &self.background { let img_size = bg.size_vec2(); let scale = (rect.width() / img_size.x).max(rect.height() / img_size.y);
                ui.painter().image(bg.id(), Rect::from_center_size(rect.center(), img_size * scale), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
            }
            for char_arc in self.characters.iter().flatten() { if let Ok(mut char) = char_arc.lock() { char.paint(ui); } }
            draw_top_right_buttons(ui, rect, &mut self.is_auto_enabled);
            if self.show_dialogue {
                let committed_text: String = self.target_chars.iter().collect();
                if !committed_text.trim().is_empty() {
                    let scene = &self.scenario.scenes[self.current_scene_idx];
                    let text: String = self.target_chars.iter().take(self.visible_count).collect();
                    if draw_ba_dialogue(ui, rect, &scene.speaker_name, &scene.speaker_aff, &text, self.visible_count >= self.target_chars.len()) { self.visible_count = self.target_chars.len(); }
                }
            }
            // 🌟 更名为 "SHELL" 
            if ui.put(Rect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(60.0, 30.0)), egui::Button::new("SHELL")).clicked() { self.console_open = !self.console_open; }
            if self.console_open { draw_creator_panel(ctx, self); }
        });
        ctx.request_repaint();
    }
}

// ============================================================================
// UI 组件函数
// ============================================================================
fn draw_top_right_buttons(ui: &mut egui::Ui, screen: Rect, is_auto: &mut bool) {
    let (btn_w, btn_h, margin) = (90.0, 32.0, 20.0);
    let auto_rect = Rect::from_min_size(Pos2::new(screen.right() - btn_w * 2.0 - margin - 10.0, margin), Vec2::new(btn_w, btn_h));
    if ui.allocate_rect(auto_rect, egui::Sense::click()).clicked() { *is_auto = !*is_auto; }
    ui.painter().rect_filled(auto_rect, 4.0, if *is_auto { Color32::from_rgb(255, 215, 0) } else { Color32::WHITE });
    ui.painter().text(auto_rect.center(), egui::Align2::CENTER_CENTER, "AUTO", egui::FontId::proportional(18.0), Color32::from_rgb(20, 30, 50));
    ui.painter().rect_filled(Rect::from_min_size(Pos2::new(screen.right() - btn_w - margin, margin), Vec2::new(btn_w, btn_h)), 4.0, Color32::WHITE);
    ui.painter().text(Pos2::new(screen.right() - btn_w / 2.0 - margin, margin + btn_h / 2.0), egui::Align2::CENTER_CENTER, "MENU", egui::FontId::proportional(18.0), Color32::from_rgb(20, 30, 50));
}

fn draw_ba_dialogue(ui: &mut egui::Ui, screen: Rect, name: &str, affiliation: &str, content: &str, is_finished: bool) -> bool {
    let box_h = screen.height() * DIALOGUE_BOX_RATIO;
    let box_rect = Rect::from_min_max(Pos2::new(screen.left(), screen.bottom() - box_h), screen.max);
    let line_y = box_rect.top() + (box_h * 0.30);  
    ui.painter().rect_filled(Rect::from_min_max(Pos2::new(screen.left(), line_y), screen.max), 0.0, Color32::from_rgba_unmultiplied(12, 18, 28, 252));
    let gradient_rect = Rect::from_min_max(box_rect.left_top(), Pos2::new(screen.right(), line_y));
    let mut mesh = Mesh::default();
    let (c_bot, c_top) = (Color32::from_rgba_unmultiplied(12, 18, 28, 245), Color32::from_rgba_unmultiplied(12, 18, 28, 0));
    mesh.vertices.push(Vertex { pos: gradient_rect.left_top(), uv: Pos2::ZERO, color: c_top });
    mesh.vertices.push(Vertex { pos: gradient_rect.right_top(), uv: Pos2::ZERO, color: c_top });
    mesh.vertices.push(Vertex { pos: gradient_rect.right_bottom(), uv: Pos2::ZERO, color: c_bot });
    mesh.vertices.push(Vertex { pos: gradient_rect.left_bottom(), uv: Pos2::ZERO, color: c_bot });
    mesh.add_triangle(0, 1, 2); mesh.add_triangle(0, 2, 3);
    ui.painter().add(Shape::mesh(mesh));
    let resp = ui.allocate_rect(box_rect, egui::Sense::click());
    let pad_x = (screen.width() * 0.08).max(100.0);
    ui.painter().line_segment([Pos2::new(pad_x, line_y), Pos2::new(screen.right() - pad_x, line_y)], Stroke::new(1.5, Color32::from_rgb(100, 120, 150)));
    if !name.is_empty() {
        let n_size = (box_h * 0.16).clamp(22.0, 30.0);
        let n_gal = ui.painter().layout_no_wrap(name.into(), egui::FontId::proportional(n_size), Color32::WHITE);
        let n_pos = Pos2::new(box_rect.left() + pad_x, line_y - n_gal.rect.height() - 4.0);
        if !affiliation.is_empty() {
            let aff_gal = ui.painter().layout_no_wrap(affiliation.into(), egui::FontId::proportional(n_size * 0.75), Color32::from_rgb(100, 200, 255));
            ui.painter().galley(n_pos, n_gal.clone(), Color32::WHITE);
            ui.painter().galley(n_pos + Vec2::new(n_gal.rect.width() + 15.0, n_gal.rect.height() - aff_gal.rect.height()), aff_gal, Color32::from_rgb(100, 200, 255));
        } else {
            ui.painter().galley(n_pos, n_gal, Color32::WHITE);
        }
    }
    ui.painter().text(Pos2::new(box_rect.left() + pad_x, line_y + box_h * 0.05), egui::Align2::LEFT_TOP, content, egui::FontId::proportional((box_h * 0.13).clamp(18.0, 25.0)), Color32::WHITE);
    if is_finished {
        let tri_center = Pos2::new(screen.right() - pad_x, screen.bottom() - (box_h * 0.15) + (ui.input(|i| i.time) * 3.0).sin() as f32 * 3.0);
        let ts = box_h * 0.04;  
        ui.painter().add(Shape::convex_polygon(vec![tri_center + Vec2::new(-ts, -ts), tri_center + Vec2::new(ts, -ts), tri_center + Vec2::new(0.0, ts)], Color32::from_rgb(0, 180, 255), Stroke::NONE));
    }
    resp.clicked()
}

fn draw_creator_panel(ctx: &egui::Context, app: &mut AefrApp) {
    let mut cmd_to_send = None;  
    let danger_bg = Color32::from_rgb(220, 38, 38);
    
    egui::Window::new("创作者面板 - GNA:AEFR")
        .default_size([500.0, 600.0])
        .show(ctx, |ui| {
            ui.heading("🎬 剧本幕数管理");
            ui.horizontal(|ui| {
                if ui.button("⬅ 上一幕").clicked() && app.current_scene_idx > 0 {
                    app.current_scene_idx -= 1; app.sync_scene_to_ui(); app.visible_count = app.target_chars.len();  
                }
                ui.label(format!(" 第 {} / {} 幕 ", app.current_scene_idx + 1, app.scenario.scenes.len()));
                if ui.button("下一幕 ➡").clicked() && app.current_scene_idx < app.scenario.scenes.len() - 1 {
                    app.current_scene_idx += 1; app.sync_scene_to_ui(); app.visible_count = app.target_chars.len();
                }
                ui.separator();
                if ui.button("➕ 增加一幕").clicked() {
                    let mut new_scene = app.scenario.scenes[app.current_scene_idx].clone();
                    new_scene.dialogue_content.clear();  
                    app.scenario.scenes.insert(app.current_scene_idx + 1, new_scene);
                    app.current_scene_idx += 1; app.sync_scene_to_ui();
                }
                let btn_text = egui::RichText::new("❌ 删除").color(Color32::WHITE);
                if ui.add(egui::Button::new(btn_text).fill(danger_bg)).clicked() && app.scenario.scenes.len() > 1 {
                    app.scenario.scenes.remove(app.current_scene_idx);
                    app.current_scene_idx = app.current_scene_idx.min(app.scenario.scenes.len() - 1);
                    app.sync_scene_to_ui();
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("跳转:");
                let mut jump = app.current_scene_idx + 1;
                let len = app.scenario.scenes.len();
                if ui.add(egui::DragValue::new(&mut jump).clamp_range(1..=len)).changed() {
                    app.current_scene_idx = jump - 1; app.sync_scene_to_ui(); app.visible_count = app.target_chars.len();
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("💾 保存剧本").clicked() { if let Ok(json_data) = serde_json::to_string_pretty(&app.scenario) { file_picker::save_scenario(app.tx.clone(), json_data); } }
                if ui.button("📂 重载剧本").clicked() { file_picker::load_scenario(app.tx.clone()); }
            });

            ui.separator();
            ui.heading("📂 资源管理");
            ui.horizontal(|ui| {
                ui.label("槽位:");
                for i in 0..5 { if ui.radio_value(&mut app.selected_slot, i, format!("[{}]", i)).clicked() { app.preview_anim_idx = 0; } }
            });
            
            ui.horizontal(|ui| {
                if ui.button("📥 导入 Spine 立绘").clicked() { file_picker::pick_spine(app.tx.clone(), app.selected_slot); }
                if ui.button("🖼 背景").clicked() { file_picker::pick_bg(app.tx.clone()); }
                let btn_text = egui::RichText::new("🗑 立绘移除").color(Color32::WHITE);
                if ui.add(egui::Button::new(btn_text).fill(danger_bg)).clicked() { cmd_to_send = Some(AppCommand::RemoveCharacter(app.selected_slot)); }
                if ui.button("🏃 动作预览").clicked() { app.show_anim_preview = true; }
            });

            ui.separator();
            ui.heading("🎵 音频管理");
            ui.horizontal(|ui| {
                if ui.button("🔁 导入音乐(循环)").clicked() { file_picker::pick_bgm(app.tx.clone()); }
                if ui.button("🔊 音效").clicked() { file_picker::pick_se(app.tx.clone()); }
                let btn_text = egui::RichText::new("⏹ 停止音乐").color(Color32::WHITE);
                if ui.add(egui::Button::new(btn_text).fill(danger_bg)).clicked() { cmd_to_send = Some(AppCommand::StopBgm); }
            });

            ui.separator();
            ui.heading("💬 对话 (当前幕)");
            let scene = &mut app.scenario.scenes[app.current_scene_idx];
            
            ui.horizontal(|ui| {
                ui.label("名称:"); ui.add(egui::TextEdit::singleline(&mut scene.speaker_name).desired_width(80.0));
                ui.label("所属:"); ui.add(egui::TextEdit::singleline(&mut scene.speaker_aff).desired_width(80.0));
            });
            
            ui.add(egui::TextEdit::multiline(&mut scene.dialogue_content).desired_width(f32::INFINITY));
            if ui.button("▶ 发送对话 (TALK)").clicked() { app.sync_scene_to_ui(); }

            ui.separator();
            ui.horizontal(|ui| {
                let res = ui.add(egui::TextEdit::singleline(&mut app.console_input).hint_text("SHELL..."));
                if ui.button("发送").clicked() || (res.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter))) {
                    let input = app.console_input.clone(); app.parse_and_send_command(&input); app.console_input.clear(); res.request_focus();  
                }
            });
            
            egui::ScrollArea::vertical().stick_to_bottom(true).max_height(60.0).show(ui, |ui| { 
                for log in &app.console_logs { ui.label(log); } 
            });
        });

    if app.show_anim_preview {
        egui::Window::new("动作").open(&mut app.show_anim_preview).show(ctx, |ui| {
            if let Some(Some(char_arc)) = app.characters.get(app.selected_slot) {
                if let Ok(char) = char_arc.lock() {
                    let anims = char.get_anim_names();
                    if !anims.is_empty() {
                        if app.preview_anim_idx >= anims.len() { app.preview_anim_idx = 0; }
                        ui.heading(&anims[app.preview_anim_idx]);
                        ui.horizontal(|ui| {
                            if ui.button("⬅").clicked() { 
                                app.preview_anim_idx = (app.preview_anim_idx + anims.len() - 1) % anims.len(); 
                                cmd_to_send = Some(AppCommand::SetAnimation { slot_idx: app.selected_slot, anim_name: anims[app.preview_anim_idx].clone(), loop_anim: true }); 
                            }
                            if ui.button("➡").clicked() { 
                                app.preview_anim_idx = (app.preview_anim_idx + 1) % anims.len(); 
                                cmd_to_send = Some(AppCommand::SetAnimation { slot_idx: app.selected_slot, anim_name: anims[app.preview_anim_idx].clone(), loop_anim: true }); 
                            }
                        });
                    }
                }
            }
        });
    }
    
    if let Some(cmd) = cmd_to_send { let _ = app.tx.send(cmd); }
}

const FONT_DATA: &[u8] = include_bytes!("SarasaTermSCNerd-Regular.ttf");

/// 🌟 极客精神：强制全局使用纯白底色、黑框描边的包豪斯（Bauhaus）风格主题。
/// 清除所有灰色带来的认知模棱两可，提供极致的高对比度与交互响应反馈。
fn setup_custom_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    
    // 全局背景纯白，光标纯黑
    visuals.window_fill = Color32::WHITE; 
    visuals.panel_fill = Color32::WHITE;  
    visuals.extreme_bg_color = Color32::WHITE; 
    visuals.text_cursor = Stroke::new(2.0, Color32::BLACK); 

    let rounding = Rounding::same(4.0); 

    // --- 控件（Widgets）视觉逻辑强覆盖 ---
    
    // 1. 平常状态（未交互）: 白底 + 细黑框 (1.0) + 黑字
    visuals.widgets.inactive.bg_fill = Color32::WHITE;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.inactive.rounding = rounding;

    // 2. 悬停状态（Hovered）: 边框加粗到 1.5 像素
    visuals.widgets.hovered.bg_fill = Color32::WHITE;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, Color32::BLACK);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.hovered.rounding = rounding;

    // 3. 激活/点击状态（Active）: 边框加粗到 2.0 像素
    visuals.widgets.active.bg_fill = Color32::WHITE;
    visuals.widgets.active.bg_stroke = Stroke::new(2.0, Color32::BLACK);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.active.rounding = rounding;

    // 4. 输入框焦点状态 (Open/Focused): 保持黑白分明，边框 1.5，不准变灰！
    visuals.widgets.open.bg_fill = Color32::WHITE;
    visuals.widgets.open.bg_stroke = Stroke::new(1.5, Color32::BLACK);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.open.rounding = rounding;

    // 5. 选中状态与弱文本强化
    visuals.selection.bg_fill = Color32::BLACK; // 选中文本纯黑底
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE); // 选中文本反白
    visuals.faint_bg_color = Color32::WHITE; 
    
    ctx.set_visuals(visuals);
}

fn setup_embedded_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("sarasa_font".to_owned(), FontData::from_static(FONT_DATA));
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "sarasa_font".to_owned());
    fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "sarasa_font".to_owned());
    ctx.set_fonts(fonts);
}
