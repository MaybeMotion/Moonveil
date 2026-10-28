mod bestiary;
mod display;
mod game;

use game::*;
use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound, stop_sound};
use macroquad::prelude::*;

const W: f32 = display::WIDTH;
const H: f32 = display::HEIGHT;
const BG: Color = Color::new(0.055, 0.054, 0.094, 1.0);
const PANEL: Color = Color::new(0.091, 0.086, 0.143, 1.0);
const BORDER: Color = Color::new(0.20, 0.18, 0.28, 1.0);
const TEXT: Color = Color::new(0.95, 0.92, 0.98, 1.0);
const MUTED: Color = Color::new(0.58, 0.56, 0.68, 1.0);
const PINK: Color = Color::new(0.97, 0.65, 0.77, 1.0);
const GOLD: Color = Color::new(0.96, 0.80, 0.50, 1.0);
const TEAL: Color = Color::new(0.48, 0.85, 0.78, 1.0);
const LILAC: Color = Color::new(0.71, 0.63, 0.96, 1.0);

fn config() -> Conf {
    Conf {
        window_title: "Moonveil · Idle Chronicles".into(),
        window_width: 1440,
        window_height: 900,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Adventure,
    Companions,
    Forge,
    Realms,
    Quests,
}

#[derive(Clone, Copy)]
enum Action {
    Strike,
    Skill(usize),
    Companion(usize),
    Upgrade(usize),
    Forge(usize),
    Claim(usize),
    Tab(Tab),
    Realm(usize),
    Settings,
    Help,
    Close,
    Sound,
    Music,
    Particles,
    Ascend,
    ConfirmAscend,
    CancelAscend,
    Save,
    CrystalBoost,
}

struct Fonts {
    ui: Font,
    title: Font,
}
struct Art {
    scenes: Vec<Texture2D>,
    heroine: Texture2D,
    fox: Texture2D,
    portraits: Vec<Texture2D>,
    enemies: Vec<Texture2D>,
    party: Vec<Texture2D>,
}
struct Audio {
    hit: Sound,
    coin: Sound,
    magic: Sound,
    music: Sound,
}
struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    max: f32,
    size: f32,
    color: Color,
}
struct FloatText {
    pos: Vec2,
    text: String,
    color: Color,
    life: f32,
    max: f32,
}
struct Toast {
    title: String,
    detail: String,
    life: f32,
}

struct App {
    game: Game,
    fonts: Fonts,
    art: Art,
    audio: Audio,
    tab: Tab,
    modal: u8,
    ascend_confirm: bool,
    mouse: Vec2,
    clicked: bool,
    hovered: bool,
    particles: Vec<Particle>,
    floats: Vec<FloatText>,
    toasts: Vec<Toast>,
    time: f32,
    last_save: f32,
    auto_tick: f32,
    auto_gold: f32,
    hit_flash: f32,
    strike_anim: f32,
    spawn_anim: f32,
    combo: u32,
    combo_time: f32,
    cooldown: [f32; 3],
    buff: f32,
    skill_flash: f32,
    save_error: bool,
    saved_message: f32,
    offline_gold: f64,
    smoke: bool,
    frame: u64,
}

fn alpha(mut c: Color, a: f32) -> Color {
    c.a = a;
    c
}
fn mix(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
fn rounded(r: Rect, radius: f32, c: Color) {
    let radius = radius.min(r.w / 2.0).min(r.h / 2.0);
    let middle = vec2(r.x + r.w / 2.0, r.y + r.h / 2.0);
    let mut points = Vec::with_capacity(36);
    for (x, y, start) in [
        (r.x + r.w - radius, r.y + radius, -90.0_f32),
        (r.x + r.w - radius, r.y + r.h - radius, 0.0),
        (r.x + radius, r.y + r.h - radius, 90.0),
        (r.x + radius, r.y + radius, 180.0),
    ] {
        for i in 0..=8 {
            let angle = (start + i as f32 * 90.0 / 8.0).to_radians();
            points.push(vec2(x + radius * angle.cos(), y + radius * angle.sin()));
        }
    }
    for i in 0..points.len() {
        draw_triangle(middle, points[i], points[(i + 1) % points.len()], c);
    }
}
fn card(r: Rect, fill: Color) {
    rounded(r, 14.0, BORDER);
    rounded(
        Rect::new(r.x + 1.0, r.y + 1.0, r.w - 2.0, r.h - 2.0),
        13.0,
        fill,
    );
}
fn gradient(r: Rect, top: Color, bottom: Color) {
    draw_mesh(&Mesh {
        vertices: vec![
            Vertex::new(r.x, r.y, 0.0, 0.0, 0.0, top),
            Vertex::new(r.x + r.w, r.y, 0.0, 0.0, 0.0, top),
            Vertex::new(r.x + r.w, r.y + r.h, 0.0, 0.0, 0.0, bottom),
            Vertex::new(r.x, r.y + r.h, 0.0, 0.0, 0.0, bottom),
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: None,
    });
}

fn capture(name: &str) {
    let folder = std::env::var_os("MOONVEIL_CAPTURE_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("work/captures"));
    if let Err(e) = std::fs::create_dir_all(&folder) {
        eprintln!("Could not create capture directory: {e}");
        return;
    }
    get_screen_data().export_png(&folder.join(format!("{name}.png")).to_string_lossy());
}

fn embedded_texture(bytes: &[u8]) -> Texture2D {
    let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    texture.set_filter(FilterMode::Linear);
    texture
}

fn fit_borders(fit: display::DisplayFit) {
    set_default_camera();
    let r = fit.bounds;
    if r.x > 0.0 {
        draw_rectangle(0.0, 0.0, r.x, fit.screen.y, BG);
        draw_rectangle(r.x + r.w, 0.0, fit.screen.x - r.x - r.w, fit.screen.y, BG);
    }
    if r.y > 0.0 {
        draw_rectangle(0.0, 0.0, fit.screen.x, r.y, BG);
        draw_rectangle(0.0, r.y + r.h, fit.screen.x, fit.screen.y - r.y - r.h, BG);
    }
}
fn text(f: &Font, s: &str, x: f32, y: f32, size: f32, c: Color) {
    draw_text_ex(
        s,
        x,
        y,
        TextParams {
            font: Some(f),
            font_size: 32,
            font_scale: size / 32.0,
            color: c,
            ..Default::default()
        },
    );
}
fn center(f: &Font, s: &str, x: f32, y: f32, size: f32, c: Color) {
    let dim = measure_text(s, Some(f), 32, size / 32.0);
    text(f, s, x - dim.width / 2.0, y, size, c);
}
fn right(f: &Font, s: &str, x: f32, y: f32, size: f32, c: Color) {
    let dim = measure_text(s, Some(f), 32, size / 32.0);
    text(f, s, x - dim.width, y, size, c);
}
fn line(x: f32, y: f32, w: f32) {
    draw_line(x, y, x + w, y, 1.0, BORDER);
}
fn diamond(x: f32, y: f32, s: f32, c: Color) {
    draw_triangle(vec2(x, y - s), vec2(x + s * 0.65, y), vec2(x, y + s), c);
    draw_triangle(vec2(x, y - s), vec2(x - s * 0.65, y), vec2(x, y + s), c);
    draw_line(x, y - s, x + s * 0.65, y, 1.0, alpha(WHITE, 0.5));
}
fn star(x: f32, y: f32, s: f32, c: Color) {
    draw_triangle(
        vec2(x, y - s),
        vec2(x + s * 0.2, y - s * 0.2),
        vec2(x - s * 0.2, y + s * 0.2),
        c,
    );
    draw_triangle(
        vec2(x, y + s),
        vec2(x - s * 0.2, y + s * 0.2),
        vec2(x + s * 0.2, y - s * 0.2),
        c,
    );
    draw_triangle(
        vec2(x - s, y),
        vec2(x - s * 0.2, y - s * 0.2),
        vec2(x + s * 0.2, y + s * 0.2),
        c,
    );
    draw_triangle(
        vec2(x + s, y),
        vec2(x + s * 0.2, y + s * 0.2),
        vec2(x - s * 0.2, y - s * 0.2),
        c,
    );
}
fn coin(x: f32, y: f32, s: f32) {
    draw_circle(x, y, s, GOLD);
    draw_circle_lines(x, y, s - 3.0, 1.0, Color::new(0.54, 0.34, 0.13, 1.0));
    diamond(x, y, s * 0.46, Color::new(0.62, 0.41, 0.19, 1.0));
}
fn moon(x: f32, y: f32, s: f32, c: Color, bg: Color) {
    draw_circle(x, y, s, c);
    draw_circle(x + s * 0.42, y - s * 0.27, s * 0.82, bg);
}
fn icon(kind: usize, x: f32, y: f32, s: f32, c: Color) {
    match kind % 10 {
        0 => {
            // crossed blade
            draw_line(x - s * 0.7, y + s * 0.7, x + s * 0.65, y - s * 0.65, 3.0, c);
            draw_triangle(
                vec2(x + s * 0.48, y - s * 0.68),
                vec2(x + s * 0.82, y - s * 0.84),
                vec2(x + s * 0.66, y - s * 0.5),
                c,
            );
            draw_line(
                x - s * 0.65,
                y + s * 0.28,
                x - s * 0.25,
                y + s * 0.7,
                3.0,
                c,
            );
        }
        1 => {
            for (xx, yy) in [(-0.5, 0.2), (0.5, 0.2), (0.0, -0.55)] {
                draw_circle(x + s * xx, y + s * yy, s * 0.25, c);
            }
            draw_arc(x, y + s * 0.4, 12, s * 0.7, 180.0, 3.0, 180.0, c);
        }
        2 => {
            diamond(x, y, s, c);
            diamond(x, y, s * 0.5, PANEL);
        }
        3 => {
            draw_circle_lines(x, y, s * 0.8, 1.5, c);
            draw_ellipse_lines(x, y, s * 0.35, s * 0.8, 0.0, 1.2, c);
            draw_line(x - s * 0.75, y, x + s * 0.75, y, 1.5, c);
        }
        4 => {
            rounded(
                Rect::new(x - s * 0.62, y - s * 0.8, s * 1.24, s * 1.6),
                3.0,
                alpha(c, 0.2),
            );
            draw_rectangle_lines(x - s * 0.62, y - s * 0.8, s * 1.24, s * 1.6, 1.5, c);
            for n in 0..3 {
                draw_line(
                    x - s * 0.32,
                    y - s * 0.35 + n as f32 * s * 0.35,
                    x + s * 0.35,
                    y - s * 0.35 + n as f32 * s * 0.35,
                    1.5,
                    c,
                );
            }
        }
        5 => {
            draw_poly_lines(x, y, 6, s * 0.8, 0.0, 2.0, c);
            draw_circle_lines(x, y, s * 0.28, 2.0, c);
        }
        6 => {
            star(x, y, s, c);
        }
        7 => {
            moon(x, y, s, c, PANEL);
        }
        8 => {
            draw_triangle(
                vec2(x - s * 0.8, y + s * 0.4),
                vec2(x - s * 0.85, y - s * 0.65),
                vec2(x - s * 0.2, y - s * 0.1),
                c,
            );
            draw_triangle(
                vec2(x + s * 0.8, y + s * 0.4),
                vec2(x + s * 0.85, y - s * 0.65),
                vec2(x + s * 0.2, y - s * 0.1),
                c,
            );
            draw_ellipse(x, y + s * 0.2, s * 0.65, s * 0.55, 0.0, c);
            draw_circle(x - s * 0.25, y + s * 0.1, 1.7, PANEL);
            draw_circle(x + s * 0.25, y + s * 0.1, 1.7, PANEL);
        }
        _ => {
            draw_line(x - s * 0.7, y, x + s * 0.7, y, 2.0, c);
            draw_line(x, y - s * 0.7, x, y + s * 0.7, 2.0, c);
        }
    }
}

fn relic_icon(i: usize, x: f32, y: f32, s: f32, c: Color, t: f32) {
    draw_circle(x, y, s * 1.2, alpha(c, 0.04));
    draw_circle_lines(x, y, s * 1.05, 1.0, alpha(c, 0.2));
    match i {
        0 => {
            draw_line(
                x - s * 0.6,
                y + s * 0.8,
                x + s * 0.65,
                y - s * 0.75,
                s * 0.17,
                c,
            );
            draw_line(
                x - s * 0.4,
                y + s * 0.12,
                x + s * 0.08,
                y + s * 0.55,
                s * 0.14,
                GOLD,
            );
            draw_line(
                x - s * 0.63,
                y + s * 0.78,
                x - s * 0.3,
                y + s * 0.42,
                s * 0.2,
                LILAC,
            );
        }
        1 => {
            moon(x, y, s * 0.7, c, PANEL);
            diamond(x + s * 0.25, y + s * 0.55, s * 0.28, GOLD);
        }
        2 => {
            rounded(
                Rect::new(x - s * 0.5, y - s * 0.55, s, s * 1.15),
                s * 0.2,
                alpha(c, 0.5),
            );
            draw_rectangle_lines(x - s * 0.5, y - s * 0.55, s, s * 1.15, 2.0, GOLD);
            star(x, y, s * 0.4, c);
            draw_line(x, y - s * 0.9, x, y - s * 0.55, 2.0, GOLD);
            draw_line(x, y + s * 0.6, x, y + s * 0.95, 2.0, GOLD);
        }
        3 => {
            let r = Rect::new(x - s * 0.45, y - s * 0.75, s * 0.9, s * 1.5);
            rounded(r, 3.0, c);
            for n in 0..3 {
                draw_line(
                    x - s * 0.22,
                    y - s * 0.35 + n as f32 * s * 0.28,
                    x + s * 0.22,
                    y - s * 0.35 + n as f32 * s * 0.28,
                    2.0,
                    PANEL,
                );
            }
        }
        4 => {
            draw_triangle(
                vec2(x - s * 0.9, y - s * 0.5),
                vec2(x - s * 0.7, y + s * 0.45),
                vec2(x, y + s * 0.35),
                c,
            );
            draw_triangle(
                vec2(x, y - s * 0.85),
                vec2(x - s * 0.7, y + s * 0.45),
                vec2(x + s * 0.7, y + s * 0.45),
                c,
            );
            draw_triangle(
                vec2(x + s * 0.9, y - s * 0.5),
                vec2(x + s * 0.7, y + s * 0.45),
                vec2(x, y + s * 0.35),
                c,
            );
            draw_line(
                x - s * 0.65,
                y + s * 0.6,
                x + s * 0.65,
                y + s * 0.6,
                3.0,
                GOLD,
            );
        }
        _ => {
            draw_poly(x, y, 6, s * 0.8, t * 14.0, alpha(c, 0.6));
            diamond(x, y, s * 0.65, c);
            star(x, y, s * 0.3, WHITE);
        }
    }
}

impl App {
    fn toast(&mut self, title: impl Into<String>, detail: impl Into<String>) {
        if self.toasts.len() >= 3 {
            self.toasts.remove(0);
        }
        self.toasts.push(Toast {
            title: title.into(),
            detail: detail.into(),
            life: 4.5,
        });
    }
    fn sfx(&self, kind: usize) {
        if self.game.sound {
            play_sound(
                match kind {
                    0 => &self.audio.hit,
                    1 => &self.audio.coin,
                    _ => &self.audio.magic,
                },
                PlaySoundParams {
                    looped: false,
                    volume: if kind == 0 { 0.18 } else { 0.28 },
                },
            );
        }
    }
    fn button(&mut self, r: Rect, label: &str, enabled: bool, primary: bool) -> bool {
        let over = r.contains(self.mouse) && self.modal == 0;
        if over {
            self.hovered = true;
        }
        let bg = if primary && enabled {
            if over { mix(PINK, WHITE, 0.13) } else { PINK }
        } else if over && enabled {
            Color::new(0.22, 0.18, 0.29, 1.0)
        } else {
            Color::new(0.14, 0.13, 0.20, 1.0)
        };
        rounded(r, 8.0, bg);
        if !primary {
            rounded(
                Rect::new(r.x + 1.0, r.y + 1.0, r.w - 2.0, r.h - 2.0),
                7.0,
                if over && enabled {
                    Color::new(0.21, 0.17, 0.28, 1.0)
                } else {
                    Color::new(0.12, 0.11, 0.18, 1.0)
                },
            );
        }
        center(
            &self.fonts.ui,
            label,
            r.x + r.w / 2.0,
            r.y + r.h / 2.0 + 5.0,
            13.0,
            if primary && enabled {
                BG
            } else if enabled {
                TEXT
            } else {
                MUTED
            },
        );
        over && enabled && self.clicked
    }
    fn burst(&mut self, p: Vec2, color: Color, count: usize) {
        if !self.game.particles {
            return;
        }
        for _ in 0..count.min(100) {
            let a = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(30.0, 160.0);
            let life = rand::gen_range(0.35, 1.0);
            self.particles.push(Particle {
                pos: p,
                vel: vec2(a.cos(), a.sin()) * speed,
                life,
                max: life,
                size: rand::gen_range(1.5, 4.0),
                color,
            });
        }
    }
    fn floating(&mut self, p: Vec2, value: String, color: Color) {
        self.floats.push(FloatText {
            pos: p,
            text: value,
            color,
            life: 1.1,
            max: 1.1,
        });
    }
    fn attack(&mut self, amount: f64, critical: bool, manual: bool) {
        if manual {
            self.hit_flash = 0.16;
            self.strike_anim = 0.26;
            self.floating(
                vec2(rand::gen_range(704.0, 794.0), rand::gen_range(330.0, 375.0)),
                format!("{}{}", if critical { "CRIT  " } else { "" }, number(amount)),
                if critical { GOLD } else { WHITE },
            );
            self.burst(
                vec2(752.0, 420.0),
                if critical { GOLD } else { PINK },
                if critical { 22 } else { 8 },
            );
            self.sfx(0);
        }
        if let Some((gold, boss)) = self.game.damage(amount) {
            self.spawn_anim = 0.4;
            self.floating(vec2(752.0, 390.0), format!("+{} gold", number(gold)), GOLD);
            self.burst(
                vec2(752.0, 420.0),
                if boss { GOLD } else { TEAL },
                if boss { 55 } else { 22 },
            );
            if boss {
                self.toast("Guardian purified", "+1 moon crystal · The path is open");
                self.sfx(2);
            }
            if self.game.stage == 21
                || self.game.stage == 41
                || self.game.stage == 61
                || self.game.stage == 81
            {
                self.toast("A new realm awaits", REALMS[self.game.realm()].0);
            }
        }
    }
    fn handle(&mut self, a: Action) {
        match a {
            Action::Strike => {
                self.game.clicks += 1;
                self.combo = if self.combo_time > 0.0 {
                    self.combo + 1
                } else {
                    1
                };
                self.combo_time = 1.5;
                let critical = rand::gen_range(0.0, 1.0) < self.game.critical_chance();
                let amount = self.game.click_damage()
                    * if critical { 2.5 } else { 1.0 }
                    * if self.buff > 0.0 { 2.0 } else { 1.0 };
                self.attack(amount, critical, true);
            }
            Action::Skill(i) => {
                if self.cooldown[i] > 0.0 {
                    return;
                }
                self.cooldown[i] = [8.0, 30.0, 45.0][i];
                self.skill_flash = 0.65;
                self.sfx(2);
                match i {
                    0 => {
                        self.attack(
                            (self.game.click_damage() * 8.0 + self.game.dps() * 2.0)
                                * if self.buff > 0.0 { 2.0 } else { 1.0 },
                            true,
                            true,
                        );
                        self.toast(
                            "Crescent slash",
                            "A blade of moonlight cuts through the veil",
                        );
                    }
                    1 => {
                        self.buff = 12.0;
                        self.burst(vec2(420.0, 430.0), PINK, 70);
                        self.toast("Sakura storm", "All damage doubled for 12 seconds");
                    }
                    _ => {
                        let gold = (self.game.gold_per_second() * 60.0).max(30.0);
                        self.game.earn(gold);
                        self.toast("Lunar blessing", format!("+{} spirit gold", number(gold)));
                        self.burst(vec2(540.0, 330.0), GOLD, 70);
                    }
                }
            }
            Action::Companion(i) => {
                if self.game.buy_companion(i) {
                    self.sfx(1);
                    self.burst(vec2(540.0, 710.0), TEAL, 18);
                }
            }
            Action::Upgrade(i) => {
                if self.game.buy_upgrade(i) {
                    self.sfx(1);
                }
            }
            Action::Forge(i) => {
                if self.game.forge(i) {
                    self.sfx(2);
                    self.toast(
                        format!("{} · rank {}", RELICS[i].name, self.game.relics[i]),
                        "Equipped automatically. Your power has grown.",
                    );
                    self.burst(vec2(540.0, 420.0), LILAC, 60);
                }
            }
            Action::Claim(i) => {
                if self.game.claim(i) {
                    self.sfx(1);
                    self.toast(
                        "Quest complete",
                        format!(
                            "+{} gold · +{} moon crystals",
                            number(QUESTS[i].2),
                            QUESTS[i].3
                        ),
                    );
                }
            }
            Action::Tab(t) => {
                self.tab = t;
                self.ascend_confirm = false;
            }
            Action::Realm(i) => {
                if self.game.warp(i) {
                    self.tab = Tab::Adventure;
                    self.sfx(2);
                    self.toast("A new chapter", REALMS[i].0);
                }
            }
            Action::Settings => self.modal = 1,
            Action::Help => self.modal = 2,
            Action::Close => {
                self.modal = 0;
                self.ascend_confirm = false;
            }
            Action::Sound => self.game.sound = !self.game.sound,
            Action::Music => {
                self.game.music = !self.game.music;
                if self.game.music {
                    play_sound(
                        &self.audio.music,
                        PlaySoundParams {
                            looped: true,
                            volume: 0.16,
                        },
                    );
                } else {
                    stop_sound(&self.audio.music);
                }
            }
            Action::Particles => self.game.particles = !self.game.particles,
            Action::Save => self.persist(),
            Action::Ascend => self.ascend_confirm = true,
            Action::CancelAscend => self.ascend_confirm = false,
            Action::ConfirmAscend => {
                if self.game.ascend() {
                    self.sfx(2);
                    self.ascend_confirm = false;
                    self.tab = Tab::Adventure;
                    self.toast(
                        "Reborn in moonlight",
                        format!("Permanent damage bonus: +{}%", self.game.ascension * 20),
                    );
                    self.persist();
                }
            }
            Action::CrystalBoost => {
                if self.game.spend_crystals() {
                    self.sfx(2);
                    self.toast(
                        "A gift from the moon",
                        "10 minutes of gold, delivered instantly",
                    );
                }
            }
        }
    }
    fn persist(&mut self) {
        match game::save(&mut self.game) {
            Ok(()) => {
                self.save_error = false;
                self.saved_message = 3.0;
            }
            Err(e) => {
                self.save_error = true;
                self.toast("Could not save", e);
            }
        }
        self.last_save = self.time;
    }
    fn tick(&mut self, dt: f32) {
        self.time += dt;
        self.frame += 1;
        self.game.playtime += dt as f64;
        self.combo_time = (self.combo_time - dt).max(0.0);
        self.strike_anim = (self.strike_anim - dt).max(0.0);
        self.hit_flash = (self.hit_flash - dt).max(0.0);
        self.spawn_anim = (self.spawn_anim - dt).max(0.0);
        self.skill_flash = (self.skill_flash - dt).max(0.0);
        self.buff = (self.buff - dt).max(0.0);
        self.saved_message = (self.saved_message - dt).max(0.0);
        for c in &mut self.cooldown {
            *c = (*c - dt).max(0.0);
        }
        self.auto_tick += dt;
        if self.auto_tick >= 0.10 && !(self.smoke && self.frame >= 380) {
            let elapsed = self.auto_tick;
            self.auto_tick = 0.0;
            self.attack(
                self.game.dps() * elapsed as f64 * if self.buff > 0.0 { 2.0 } else { 1.0 },
                false,
                false,
            );
        }
        self.auto_gold += dt;
        if self.auto_gold >= 0.5 {
            self.game
                .earn(self.game.passive_gold() * self.auto_gold as f64);
            self.auto_gold = 0.0;
        }
        for p in &mut self.particles {
            p.life -= dt;
            p.pos += p.vel * dt;
            p.vel *= 1.0 - dt * 1.2;
        }
        self.particles.retain(|p| p.life > 0.0);
        for f in &mut self.floats {
            f.life -= dt;
            f.pos.y -= 34.0 * dt;
        }
        self.floats.retain(|f| f.life > 0.0);
        for t in &mut self.toasts {
            t.life -= dt;
        }
        self.toasts.retain(|t| t.life > 0.0);
        if self.time - self.last_save > 15.0 && !self.smoke {
            self.persist();
        }
    }

    fn header(&mut self) -> Option<Action> {
        let mut action = None;
        line(0.0, 91.0, W);
        moon(44.0, 42.0, 17.0, PINK, BG);
        star(59.0, 28.0, 4.0, GOLD);
        text(&self.fonts.title, "MOONVEIL", 110.0, 44.0, 35.0, TEXT);
        text(
            &self.fonts.ui,
            "I D L E   C H R O N I C L E S",
            112.0,
            67.0,
            9.0,
            MUTED,
        );
        text(
            &self.fonts.ui,
            "A world worth getting lost in.",
            405.0,
            49.0,
            13.0,
            MUTED,
        );

        let p = Rect::new(864.0, 22.0, 228.0, 49.0);
        card(p, PANEL);
        coin(890.0, 46.0, 11.0);
        text(
            &self.fonts.ui,
            &number(self.game.gold),
            912.0,
            43.0,
            19.0,
            GOLD,
        );
        text(
            &self.fonts.ui,
            &format!("+{} / sec", number(self.game.gold_per_second())),
            913.0,
            61.0,
            10.0,
            MUTED,
        );
        text(&self.fonts.ui, "SPIRIT GOLD", 1012.0, 47.0, 9.0, MUTED);
        card(Rect::new(1104.0, 22.0, 130.0, 49.0), PANEL);
        diamond(1129.0, 46.0, 11.0, LILAC);
        text(
            &self.fonts.ui,
            &self.game.crystals.to_string(),
            1150.0,
            45.0,
            18.0,
            TEXT,
        );
        text(&self.fonts.ui, "MOON", 1150.0, 60.0, 9.0, MUTED);
        for (r, i, a) in [
            (Rect::new(1254.0, 26.0, 40.0, 40.0), 4, Action::Help),
            (Rect::new(1307.0, 26.0, 40.0, 40.0), 5, Action::Settings),
        ] {
            let over = r.contains(self.mouse) && self.modal == 0;
            if over {
                rounded(r, 10.0, PANEL);
                self.hovered = true;
                if self.clicked {
                    action = Some(a);
                }
            }
            icon(
                i,
                r.x + 20.0,
                r.y + 20.0,
                11.0,
                if over { PINK } else { MUTED },
            );
        }
        draw_circle(1385.0, 46.0, 21.0, Color::new(0.24, 0.19, 0.30, 1.0));
        moon(1384.0, 46.0, 10.0, PINK, Color::new(0.24, 0.19, 0.30, 1.0));
        action
    }
    fn sidebar(&mut self) -> Option<Action> {
        let mut action = None;
        draw_line(88.0, 92.0, 88.0, H, 1.0, BORDER);
        for (j, (tab, label)) in [
            (Tab::Adventure, "Battle"),
            (Tab::Companions, "Party"),
            (Tab::Forge, "Forge"),
            (Tab::Realms, "Realms"),
            (Tab::Quests, "Quests"),
        ]
        .iter()
        .enumerate()
        {
            let y = 128.0 + j as f32 * 90.0;
            let r = Rect::new(10.0, y, 68.0, 70.0);
            let over = r.contains(self.mouse) && self.modal == 0;
            if *tab == self.tab {
                rounded(r, 12.0, Color::new(0.19, 0.125, 0.20, 1.0));
                rounded(Rect::new(0.0, y + 22.0, 3.0, 25.0), 1.0, PINK);
            } else if over {
                rounded(r, 12.0, PANEL);
            }
            let c = if *tab == self.tab { PINK } else { MUTED };
            icon(j, 44.0, y + 26.0, 13.0, c);
            center(&self.fonts.ui, label, 44.0, y + 56.0, 10.0, c);
            if over {
                self.hovered = true;
                if self.clicked {
                    action = Some(Action::Tab(*tab));
                }
            }
            if j == 4 && self.game.available_quests() > 0 {
                draw_circle(62.0, y + 13.0, 7.0, PINK);
                center(
                    &self.fonts.ui,
                    &self.game.available_quests().to_string(),
                    62.0,
                    y + 16.5,
                    8.0,
                    BG,
                );
            }
        }
        moon(44.0, 770.0, 12.0, LILAC, BG);
        center(&self.fonts.ui, "LUNAR", 44.0, 799.0, 8.0, MUTED);
        center(
            &self.fonts.ui,
            &format!("+{}%", self.game.ascension * 20),
            44.0,
            818.0,
            12.0,
            LILAC,
        );
        action
    }
    fn heading(&self, kicker: &str, title: &str, sub: &str) {
        text(&self.fonts.ui, kicker, 118.0, 122.0, 9.5, PINK);
        text(&self.fonts.title, title, 116.0, 164.0, 39.0, TEXT);
        text(&self.fonts.ui, sub, 118.0, 188.0, 12.0, MUTED);
    }
    fn training(&mut self) -> Option<Action> {
        let mut action = None;
        let r = Rect::new(998.0, 112.0, 414.0, 734.0);
        card(r, PANEL);
        text(&self.fonts.ui, "CULTIVATION", 1022.0, 144.0, 10.0, PINK);
        text(
            &self.fonts.title,
            "A little stronger",
            1021.0,
            177.0,
            30.0,
            TEXT,
        );
        text(
            &self.fonts.ui,
            "Every small step becomes something great.",
            1022.0,
            200.0,
            11.0,
            MUTED,
        );
        line(1022.0, 219.0, 366.0);
        for (i, upgrade) in UPGRADES.iter().enumerate() {
            let y = 237.0 + i as f32 * 75.0;
            let capped = self.game.upgrades[i] >= MAX_LEVEL;
            let cost = self.game.upgrade_cost(i);
            let affordable = self.game.gold >= cost && !capped;
            rounded(
                Rect::new(1022.0, y + 1.0, 43.0, 43.0),
                10.0,
                Color::new(0.16, 0.13, 0.23, 1.0),
            );
            icon(
                [0, 6, 7, 2, 8, 3][i],
                1043.5,
                y + 22.0,
                12.0,
                [PINK, LILAC, GOLD, TEAL, LILAC, PINK][i],
            );
            text(&self.fonts.ui, upgrade.name, 1077.0, y + 14.0, 13.0, TEXT);
            text(
                &self.fonts.ui,
                &format!("Lv. {}", self.game.upgrades[i]),
                1077.0,
                y + 32.0,
                10.0,
                MUTED,
            );
            text(&self.fonts.ui, upgrade.desc, 1024.0, y + 60.0, 10.0, MUTED);
            let label = if capped { "MAX".into() } else { number(cost) };
            if self.button(
                Rect::new(1293.0, y + 3.0, 95.0, 36.0),
                &label,
                affordable,
                affordable,
            ) {
                action = Some(Action::Upgrade(i));
            }
        }
        line(1022.0, 696.0, 366.0);
        text(&self.fonts.ui, "YOUR POWER", 1023.0, 723.0, 9.0, MUTED);
        text(
            &self.fonts.ui,
            &number(self.game.click_damage()),
            1023.0,
            756.0,
            25.0,
            PINK,
        );
        text(&self.fonts.ui, "per strike", 1024.0, 775.0, 10.0, MUTED);
        draw_line(1192.0, 728.0, 1192.0, 775.0, 1.0, BORDER);
        text(
            &self.fonts.ui,
            &number(self.game.dps()),
            1210.0,
            756.0,
            25.0,
            TEAL,
        );
        text(&self.fonts.ui, "companion DPS", 1211.0, 775.0, 10.0, MUTED);
        rounded(
            Rect::new(1022.0, 798.0, 366.0, 29.0),
            7.0,
            Color::new(0.12, 0.105, 0.18, 1.0),
        );
        star(1038.0, 813.0, 5.0, LILAC);
        text(
            &self.fonts.ui,
            &format!(
                "Critical chance  {:.0}%     ·     Critical damage  250%",
                self.game.critical_chance() * 100.0
            ),
            1051.0,
            817.0,
            9.0,
            MUTED,
        );
        action
    }
    fn battle_party(&self, back_row: bool) {
        // Compact full-body sprites keep every recruited ally visible around Yuki.
        const FORMATION: [(f32, f32, f32); 6] = [
            (180.0, 510.0, 82.0),
            (435.0, 514.0, 151.0),
            (528.0, 510.0, 146.0),
            (190.0, 416.0, 123.0),
            (435.0, 411.0, 123.0),
            (535.0, 414.0, 126.0),
        ];
        const COLORS: [Color; 6] = [PINK, TEAL, TEAL, GOLD, LILAC, GOLD];
        for i in 0..6 {
            if self.game.companions[i] == 0 || (i >= 3) != back_row {
                continue;
            }
            let (x, foot, height) = FORMATION[i];
            let phase = (self.time * 0.72 + i as f32 * 0.19).rem_euclid(1.0);
            let attack = if phase < 0.14 {
                (phase / 0.14 * std::f32::consts::PI).sin()
            } else {
                0.0
            };
            let breath = (self.time * 2.0 + i as f32).sin() * 1.8;
            let sprite = if i == 0 {
                &self.art.fox
            } else {
                &self.art.party[i - 1]
            };
            let width = height * sprite.width() / sprite.height();
            draw_ellipse(
                x,
                foot,
                if i == 0 { 30.0 } else { 22.0 },
                4.0,
                0.0,
                Color::new(0.02, 0.03, 0.09, 0.34),
            );
            if back_row {
                draw_ellipse_lines(x, foot + 1.0, 26.0, 5.0, 0.0, 1.0, alpha(COLORS[i], 0.45));
                star(x + 24.0, foot - 4.0, 3.0, alpha(COLORS[i], 0.7));
            }
            draw_texture_ex(
                sprite,
                x - width / 2.0 + attack * 7.0,
                foot - height + breath - attack * 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(width, height)),
                    flip_x: i == 0,
                    rotation: (self.time + i as f32).sin() * 0.008,
                    ..Default::default()
                },
            );
            center(
                &self.fonts.ui,
                COMPANIONS[i].name,
                x,
                if back_row {
                    foot - height - 6.0
                } else {
                    foot + 13.0
                },
                8.0,
                alpha(TEXT, 0.85),
            );
            if self.game.particles {
                let projectile = (phase / 0.66).min(1.0);
                if phase < 0.66 {
                    let from = vec2(x + 20.0, foot - height * 0.55);
                    for trail in 0..5 {
                        let t = (projectile - trail as f32 * 0.018).max(0.0);
                        let pos = from.lerp(vec2(747.0, 415.0), t)
                            - vec2(0.0, (t * std::f32::consts::PI).sin() * 26.0);
                        let color = alpha(COLORS[i], (1.0 - trail as f32 / 6.0) * 0.75);
                        if i == 1 {
                            draw_line(
                                pos.x - 5.0,
                                pos.y + 2.0,
                                pos.x + 5.0,
                                pos.y - 2.0,
                                1.8,
                                color,
                            );
                        } else {
                            star(pos.x, pos.y, 3.0 - trail as f32 * 0.35, color);
                        }
                    }
                }
            }
        }
    }

    fn adventure(&mut self) -> Option<Action> {
        let mut action = None;
        let realm = self.game.realm();
        self.heading(
            &format!("ADVENTURE  /  CHAPTER {:02}", realm + 1),
            REALMS[realm].0,
            "The moon is quiet. Your adventure is only beginning.",
        );
        let r = Rect::new(118.0, 208.0, 854.0, 414.0);
        rounded(
            Rect::new(r.x - 1.0, r.y - 1.0, r.w + 2.0, r.h + 2.0),
            14.0,
            BORDER,
        );
        draw_texture_ex(
            &self.art.scenes[realm],
            r.x,
            r.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            },
        );
        gradient(
            Rect::new(r.x, r.y, r.w, 130.0),
            Color::new(0.06, 0.06, 0.13, 0.76),
            Color::new(0.06, 0.06, 0.13, 0.0),
        );
        gradient(
            Rect::new(r.x, r.y + 250.0, r.w, 164.0),
            Color::new(0.06, 0.06, 0.13, 0.0),
            Color::new(0.06, 0.06, 0.13, 0.96),
        );
        rounded(
            Rect::new(138.0, 227.0, 136.0, 28.0),
            7.0,
            Color::new(0.06, 0.06, 0.12, 0.8),
        );
        draw_circle(151.0, 241.0, 3.0, TEAL);
        text(
            &self.fonts.ui,
            "AUTO BATTLE ACTIVE",
            161.0,
            245.0,
            8.0,
            TEAL,
        );
        let wave = format!("WAVE {:03}", self.game.stage);
        center(&self.fonts.ui, &wave, 545.0, 245.0, 11.0, TEXT);
        right(
            &self.fonts.ui,
            &format!("BEST  {:03}", self.game.best),
            949.0,
            245.0,
            10.0,
            alpha(TEXT, 0.65),
        );
        for i in 0..10 {
            let x = 490.0 + i as f32 * 12.0;
            draw_circle(
                x,
                263.0,
                2.0,
                if i < (self.game.stage as usize - 1) % 10 {
                    PINK
                } else {
                    alpha(WHITE, 0.3)
                },
            );
        }

        if self.game.particles {
            for i in 0..32 {
                let seed = i as f32;
                let x = r.x + ((seed * 93.17 + self.time * (6.0 + (i % 4) as f32 * 3.0)) % r.w);
                let y = r.y + ((seed * 37.7 + self.time * 8.0) % 350.0);
                let a = 0.18 + 0.24 * (self.time + seed).sin().abs();
                if i % 3 == 0 {
                    draw_ellipse(
                        x,
                        y,
                        3.0,
                        1.3,
                        (self.time * 0.5 + seed).sin() * 0.8,
                        alpha(PINK, a),
                    );
                } else {
                    draw_circle(x, y, 1.4, alpha(TEAL, a));
                }
            }
        }
        self.battle_party(true);
        // Yuki stands between the two ranks of recruited companions.
        let breath = (self.time * 2.1).sin() * 2.4;
        let lunge = if self.strike_anim > 0.0 {
            (self.strike_anim / 0.26 * std::f32::consts::PI).sin() * 34.0
        } else {
            0.0
        };
        draw_ellipse(
            310.0 + lunge,
            519.0,
            71.0,
            10.0,
            0.0,
            Color::new(0.02, 0.03, 0.09, 0.45),
        );
        let hero_h = 264.0 + breath;
        let hero_w = hero_h * self.art.heroine.width() / self.art.heroine.height();
        draw_texture_ex(
            &self.art.heroine,
            310.0 - hero_w * 0.5 + lunge,
            518.0 - hero_h,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(hero_w, hero_h)),
                rotation: (self.time * 1.3).sin() * 0.006,
                ..Default::default()
            },
        );
        self.battle_party(false);
        let boss = self.game.boss();
        let enemy_index = bestiary::enemy_for_wave(self.game.stage);
        let enemy = &self.art.enemies[enemy_index];
        let bob = match enemy_index {
            1 => (self.time * 4.0).sin() * 3.0,
            2 => -(self.time * 3.0).sin().max(0.0) * 8.0,
            3 => (self.time * 3.2).sin() * 10.0,
            4 => (self.time * 2.1).sin() * 7.0,
            5 => (self.time * 1.2).sin() * 1.5,
            6 => (self.time * 1.6).sin() * 2.0,
            7 => (self.time * 1.4).sin() * 6.0,
            _ => (self.time * 2.8).sin() * 8.0,
        };
        let sway = match enemy_index {
            3 => 0.055,
            4 => 0.08,
            5 => 0.003,
            6 => 0.008,
            _ => 0.03,
        };
        let appear = 1.0 - self.spawn_anim * 0.5;
        let sprite_scale = ((if boss { 202.0 } else { 184.0 }) / enemy.height())
            .min(292.0 / enemy.width())
            * appear;
        let foe_h = enemy.height() * sprite_scale;
        let foe_w = enemy.width() * sprite_scale;
        let recoil = if self.hit_flash > 0.0 {
            (self.time * 100.0).sin() * 4.0
        } else {
            0.0
        };
        draw_ellipse(
            756.0,
            512.0,
            72.0,
            9.0,
            0.0,
            Color::new(0.03, 0.02, 0.10, 0.4),
        );
        draw_circle_lines(
            756.0,
            429.0,
            83.0 + (self.time * 2.0).sin() * 3.0,
            1.0,
            alpha(if boss { GOLD } else { LILAC }, 0.18),
        );
        draw_texture_ex(
            enemy,
            756.0 - foe_w / 2.0 + recoil,
            425.0 - foe_h / 2.0 + bob,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(foe_w, foe_h)),
                rotation: (self.time * 1.1).sin() * sway,
                ..Default::default()
            },
        );

        center(
            &self.fonts.ui,
            bestiary::NAMES[enemy_index],
            756.0,
            286.0,
            11.0,
            if boss { GOLD } else { TEXT },
        );
        let hp = (self.game.hp / self.game.max_hp()).clamp(0.0, 1.0) as f32;
        rounded(Rect::new(682.0, 297.0, 148.0, 5.0), 2.5, alpha(BLACK, 0.45));
        if hp > 0.0 {
            rounded(
                Rect::new(682.0, 297.0, 148.0 * hp, 5.0),
                2.5,
                if boss { GOLD } else { PINK },
            );
        }
        center(
            &self.fonts.ui,
            &format!("{} / {}", number(self.game.hp), number(self.game.max_hp())),
            756.0,
            315.0,
            9.0,
            alpha(TEXT, 0.7),
        );
        let target = Rect::new(610.0, 327.0, 292.0, 207.0);
        if target.contains(self.mouse) && self.modal == 0 {
            self.hovered = true;
            draw_circle_lines(756.0, 429.0, 91.0, 1.0, alpha(PINK, 0.4));
            if self.clicked {
                action = Some(Action::Strike);
            }
        }

        if self.strike_anim > 0.0 {
            let t = 1.0 - self.strike_anim / 0.26;
            draw_arc(
                687.0 + t * 58.0,
                414.0,
                35,
                66.0,
                240.0 - t * 70.0,
                5.0 * (1.0 - t) + 1.0,
                130.0,
                alpha(PINK, 1.0 - t),
            );
            draw_arc(
                683.0 + t * 58.0,
                413.0,
                35,
                70.0,
                240.0 - t * 70.0,
                2.0,
                125.0,
                alpha(WHITE, 1.0 - t),
            );
        }
        if self.buff > 0.0 {
            draw_circle_lines(
                310.0,
                421.0,
                100.0 + (self.time * 4.0).sin() * 3.0,
                1.5,
                alpha(PINK, 0.25),
            );
            star(369.0, 327.0, 9.0, PINK);
        }
        if self.skill_flash > 0.0 {
            draw_rectangle(r.x, r.y, r.w, r.h, alpha(LILAC, self.skill_flash * 0.10));
        }
        center(&self.fonts.ui, "Y U K I", 310.0, 537.0, 10.0, TEXT);
        if self.combo > 1 && self.combo_time > 0.0 {
            right(
                &self.fonts.title,
                &format!("{} hit combo", self.combo),
                935.0,
                528.0,
                22.0,
                PINK,
            );
        }
        center(
            &self.fonts.ui,
            "Click the spirit to strike  ·  Companions fight for you",
            548.0,
            555.0,
            9.0,
            alpha(TEXT, 0.65),
        );
        for (i, (title, key, c)) in [
            ("Crescent slash", "Q", PINK),
            ("Sakura storm", "W", LILAC),
            ("Lunar blessing", "E", GOLD),
        ]
        .iter()
        .enumerate()
        {
            let x = 139.0 + i as f32 * 270.0;
            let rr = Rect::new(x, 570.0, 253.0, 35.0);
            let ready = self.cooldown[i] <= 0.0;
            let over = rr.contains(self.mouse) && self.modal == 0;
            rounded(
                rr,
                7.0,
                if over && ready {
                    Color::new(0.22, 0.17, 0.30, 0.97)
                } else {
                    Color::new(0.09, 0.075, 0.15, 0.92)
                },
            );
            rounded(Rect::new(x + 6.0, 576.0, 23.0, 23.0), 5.0, alpha(*c, 0.18));
            center(&self.fonts.ui, key, x + 17.5, 591.0, 10.0, *c);
            text(
                &self.fonts.ui,
                title,
                x + 39.0,
                591.0,
                11.0,
                if ready { TEXT } else { MUTED },
            );
            right(
                &self.fonts.ui,
                &if ready {
                    "READY".into()
                } else {
                    format!("{:.0}s", self.cooldown[i].ceil())
                },
                x + 239.0,
                591.0,
                9.0,
                if ready { *c } else { MUTED },
            );
            if over {
                self.hovered = true;
                if self.clicked && ready {
                    action = Some(Action::Skill(i));
                }
            }
        }
        action
    }
    fn party_strip(&mut self) -> Option<Action> {
        let mut action = None;
        text(
            &self.fonts.title,
            "Better together",
            118.0,
            664.0,
            28.0,
            TEXT,
        );
        text(&self.fonts.ui, "YOUR COMPANIONS", 119.0, 685.0, 9.0, MUTED);
        if self.button(
            Rect::new(843.0, 646.0, 129.0, 31.0),
            "View party  →",
            true,
            false,
        ) {
            action = Some(Action::Tab(Tab::Companions));
        }
        for i in 0..3 {
            let x = 118.0 + i as f32 * 289.0;
            let r = Rect::new(x, 704.0, 276.0, 142.0);
            card(r, PANEL);
            let c = [PINK, TEAL, LILAC][i];
            rounded(Rect::new(x + 15.0, 720.0, 44.0, 44.0), 11.0, alpha(c, 0.12));
            if i == 0 {
                icon(8, x + 37.0, 742.0, 14.0, c);
            } else {
                let portrait = &self.art.portraits[i - 1];
                draw_texture_ex(
                    portrait,
                    x + 16.0,
                    719.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(42.0, 45.0)),
                        ..Default::default()
                    },
                );
            }
            text(
                &self.fonts.ui,
                COMPANIONS[i].name,
                x + 72.0,
                737.0,
                14.0,
                TEXT,
            );
            text(
                &self.fonts.ui,
                COMPANIONS[i].title,
                x + 72.0,
                757.0,
                10.0,
                MUTED,
            );
            text(
                &self.fonts.ui,
                &format!("Lv. {}", self.game.companions[i]),
                x + 16.0,
                791.0,
                11.0,
                c,
            );
            text(
                &self.fonts.ui,
                &format!(
                    "{} base DPS",
                    number(COMPANIONS[i].dps * self.game.companions[i] as f64)
                ),
                x + 16.0,
                815.0,
                10.0,
                MUTED,
            );
            let price = self.game.companion_cost(i);
            let capped = self.game.companions[i] >= MAX_LEVEL;
            let label = if capped {
                "MAX".into()
            } else {
                format!("+1  ·  {}", number(price))
            };
            if self.button(
                Rect::new(x + 147.0, 785.0, 113.0, 39.0),
                &label,
                self.game.gold >= price && !capped,
                true,
            ) {
                action = Some(Action::Companion(i));
            }
        }
        action
    }
    fn companions(&mut self) -> Option<Action> {
        self.heading(
            "COMPANIONS  /  YOUR LITTLE CONSTELLATION",
            "No one journeys alone",
            "Recruit a kindred spirit. Every level adds permanent damage to your party.",
        );
        let mut action = None;
        for (i, c) in COMPANIONS.iter().enumerate() {
            let x = 118.0 + (i % 3) as f32 * 289.0;
            let y = 213.0 + (i / 3) as f32 * 279.0;
            let color = [PINK, TEAL, LILAC, GOLD, LILAC, PINK][i];
            card(Rect::new(x, y, 276.0, 263.0), PANEL);
            text(&self.fonts.ui, c.element, x + 19.0, y + 27.0, 9.0, color);
            right(
                &self.fonts.ui,
                &format!("LV. {}", self.game.companions[i]),
                x + 255.0,
                y + 27.0,
                9.0,
                MUTED,
            );
            let bob = (self.time * 1.8 + i as f32).sin() * 4.0;
            draw_circle(x + 138.0, y + 90.0, 42.0, alpha(color, 0.07));
            draw_circle_lines(x + 138.0, y + 90.0, 41.0, 1.0, alpha(color, 0.18));
            if i == 0 {
                let h = 90.0;
                let w = h * self.art.fox.width() / self.art.fox.height();
                draw_texture_ex(
                    &self.art.fox,
                    x + 138.0 - w / 2.0,
                    y + 42.0 + bob,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(w, h)),
                        ..Default::default()
                    },
                );
            } else {
                let portrait = &self.art.portraits[i - 1];
                let h = 102.0;
                let w = h * portrait.width() / portrait.height();
                draw_texture_ex(
                    portrait,
                    x + 138.0 - w / 2.0,
                    y + 31.0 + bob,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(w, h)),
                        ..Default::default()
                    },
                );
                for j in 0..4 {
                    let a = self.time * 0.5 + j as f32 * 1.57;
                    star(
                        x + 138.0 + a.cos() * 52.0,
                        y + 90.0 + a.sin() * 35.0,
                        3.0,
                        color,
                    );
                }
            }
            center(&self.fonts.title, c.name, x + 138.0, y + 158.0, 28.0, TEXT);
            center(&self.fonts.ui, c.title, x + 138.0, y + 180.0, 10.0, MUTED);
            text(
                &self.fonts.ui,
                &format!("+{} base DPS / level", number(c.dps)),
                x + 19.0,
                y + 211.0,
                10.0,
                color,
            );
            let capped = self.game.companions[i] >= MAX_LEVEL;
            let price = self.game.companion_cost(i);
            let label = if capped {
                "MAX LEVEL".into()
            } else {
                format!(
                    "{}   ·   {} gold",
                    if self.game.companions[i] > 0 {
                        "Level up"
                    } else {
                        "Recruit"
                    },
                    number(price)
                )
            };
            if self.button(
                Rect::new(x + 16.0, y + 224.0, 244.0, 28.0),
                &label,
                self.game.gold >= price && !capped,
                true,
            ) {
                action = Some(Action::Companion(i));
            }
        }
        rounded(
            Rect::new(118.0, 789.0, 854.0, 57.0),
            10.0,
            Color::new(0.115, 0.10, 0.17, 1.0),
        );
        star(142.0, 818.0, 8.0, TEAL);
        text(
            &self.fonts.ui,
            "A bond that never sleeps",
            164.0,
            814.0,
            12.0,
            TEXT,
        );
        text(
            &self.fonts.ui,
            "Your companions earn 50% of estimated battle gold while you are away, for up to 8 hours.",
            164.0,
            833.0,
            10.0,
            MUTED,
        );
        action
    }
    fn forge(&mut self) -> Option<Action> {
        self.heading("THE CELESTIAL FORGE  /  MADE FOR YOU","Little things. Legendary power.","Craft a relic, keep its magic. Every relic equips automatically and stacks with your training.");
        let mut action = None;
        for (i, relic) in RELICS.iter().enumerate() {
            let x = 118.0 + (i % 3) as f32 * 289.0;
            let y = 213.0 + (i / 3) as f32 * 279.0;
            let c = match i {
                0 | 3 => TEAL,
                1 | 2 => LILAC,
                4 => GOLD,
                _ => PINK,
            };
            card(Rect::new(x, y, 276.0, 263.0), PANEL);
            text(&self.fonts.ui, relic.rarity, x + 18.0, y + 26.0, 9.0, c);
            right(
                &self.fonts.ui,
                &format!("RANK {}", self.game.relics[i]),
                x + 257.0,
                y + 26.0,
                9.0,
                MUTED,
            );
            relic_icon(i, x + 138.0, y + 86.0, 31.0, c, self.time);
            center(
                &self.fonts.title,
                relic.name,
                x + 138.0,
                y + 155.0,
                26.0,
                TEXT,
            );
            center(&self.fonts.ui, relic.kind, x + 138.0, y + 175.0, 9.0, MUTED);
            center(&self.fonts.ui, relic.desc, x + 138.0, y + 203.0, 10.0, c);
            let capped = self.game.relics[i] >= 25;
            let price = self.game.relic_cost(i);
            let label = if capped {
                "MAX RANK".into()
            } else {
                format!(
                    "{}   ·   {} gold",
                    if self.game.relics[i] > 0 {
                        "Refine"
                    } else {
                        "Forge"
                    },
                    number(price)
                )
            };
            if self.button(
                Rect::new(x + 16.0, y + 223.0, 244.0, 29.0),
                &label,
                self.game.gold >= price && !capped,
                true,
            ) {
                action = Some(Action::Forge(i));
            }
        }
        text(
            &self.fonts.ui,
            "No luck required. No lost materials. Just a little more magic, every time.",
            135.0,
            826.0,
            11.0,
            MUTED,
        );
        action
    }
    fn quests(&mut self) -> Option<Action> {
        self.heading("MOONLIT MISSIONS  /  SMALL WONDERS","Every moment means something","Your first steps deserve a little celebration. Collect gold and moon crystals as you grow.");
        let mut action = None;
        for (i, q) in QUESTS.iter().enumerate() {
            let y = 213.0 + i as f32 * 91.0;
            let claimed = self.game.claimed[i];
            let (a, b) = self.game.quest_progress(i);
            let done = a >= b;
            card(Rect::new(118.0, y, 854.0, 78.0), PANEL);
            rounded(
                Rect::new(135.0, y + 17.0, 43.0, 43.0),
                11.0,
                alpha(if claimed { TEAL } else { LILAC }, 0.1),
            );
            icon(
                if claimed { 6 } else { i },
                156.0,
                y + 39.0,
                12.0,
                if claimed { TEAL } else { LILAC },
            );
            text(&self.fonts.ui, q.0, 193.0, y + 27.0, 13.0, TEXT);
            text(&self.fonts.ui, q.1, 193.0, y + 47.0, 10.0, MUTED);
            rounded(Rect::new(193.0, y + 58.0, 256.0, 3.0), 1.5, BORDER);
            let p = (a / b).min(1.0) as f32;
            if p > 0.0 {
                rounded(
                    Rect::new(193.0, y + 58.0, 256.0 * p, 3.0),
                    1.5,
                    if claimed { TEAL } else { PINK },
                );
            }
            text(
                &self.fonts.ui,
                &format!("{} / {}", number(a.min(b)), number(b)),
                470.0,
                y + 42.0,
                11.0,
                MUTED,
            );
            coin(640.0, y + 32.0, 7.0);
            text(
                &self.fonts.ui,
                &format!("+{}", number(q.2)),
                656.0,
                y + 37.0,
                12.0,
                GOLD,
            );
            if q.3 > 0 {
                diamond(640.0, y + 54.0, 6.0, LILAC);
                text(
                    &self.fonts.ui,
                    &format!("+{} crystals", q.3),
                    656.0,
                    y + 59.0,
                    9.0,
                    LILAC,
                );
            }
            if self.button(
                Rect::new(810.0, y + 22.0, 140.0, 35.0),
                if claimed {
                    "Collected"
                } else if done {
                    "Collect reward"
                } else {
                    "In progress"
                },
                done && !claimed,
                done && !claimed,
            ) {
                action = Some(Action::Claim(i));
            }
        }
        card(
            Rect::new(118.0, 777.0, 854.0, 69.0),
            Color::new(0.12, 0.10, 0.19, 1.0),
        );
        diamond(147.0, 810.0, 13.0, LILAC);
        text(
            &self.fonts.ui,
            "A gift from the moon",
            176.0,
            804.0,
            13.0,
            TEXT,
        );
        text(
            &self.fonts.ui,
            "Exchange 5 moon crystals for 10 minutes of gold (minimum 250).",
            176.0,
            825.0,
            10.0,
            MUTED,
        );
        if self.button(
            Rect::new(774.0, 795.0, 180.0, 34.0),
            "5 crystals  ·  Exchange",
            self.game.crystals >= 5,
            true,
        ) {
            action = Some(Action::CrystalBoost);
        }
        action
    }
    fn realms(&mut self) -> Option<Action> {
        self.heading(
            "THE WORLD BEYOND  /  FOLLOW THE MOON",
            "There is always another horizon",
            "New realms open every 20 waves. Revisit unlocked realms to gather strength.",
        );
        let mut action = None;
        for (i, realm) in REALMS.iter().enumerate() {
            let y = 213.0 + i as f32 * 87.0;
            let unlocked = self.game.run_best >= realm.2;
            let active = self.game.realm() == i;
            let c = [PINK, TEAL, LILAC, GOLD, PINK][i];
            card(
                Rect::new(118.0, y, 854.0, 75.0),
                if active {
                    Color::new(0.15, 0.11, 0.20, 1.0)
                } else {
                    PANEL
                },
            );
            draw_circle(154.0, y + 37.0, 20.0, alpha(c, 0.12));
            icon(
                i + 3,
                154.0,
                y + 37.0,
                11.0,
                if unlocked { c } else { MUTED },
            );
            text(
                &self.fonts.title,
                realm.0,
                191.0,
                y + 33.0,
                25.0,
                if unlocked { TEXT } else { MUTED },
            );
            text(&self.fonts.ui, realm.1, 192.0, y + 54.0, 10.0, MUTED);
            text(
                &self.fonts.ui,
                &format!("WAVE {:03}", realm.2),
                624.0,
                y + 42.0,
                10.0,
                if unlocked { c } else { MUTED },
            );
            if self.button(
                Rect::new(800.0, y + 20.0, 152.0, 35.0),
                if !unlocked {
                    "Locked"
                } else if active {
                    "Revisit"
                } else {
                    "Travel"
                },
                unlocked,
                false,
            ) {
                action = Some(Action::Realm(i));
            }
        }
        let r = Rect::new(118.0, 668.0, 854.0, 178.0);
        card(r, Color::new(0.13, 0.10, 0.19, 1.0));
        moon(158.0, 705.0, 17.0, LILAC, Color::new(0.13, 0.10, 0.19, 1.0));
        text(
            &self.fonts.title,
            "Begin again. Shine a little brighter.",
            192.0,
            711.0,
            28.0,
            TEXT,
        );
        text(
            &self.fonts.ui,
            "Ascend after wave 50 for permanent +20% damage per lunar blessing.",
            141.0,
            745.0,
            11.0,
            MUTED,
        );
        text(
            &self.fonts.ui,
            "Gold, training, party levels and relics reset. Crystals and completed quests stay with you.",
            141.0,
            765.0,
            10.0,
            MUTED,
        );
        let gain = self.game.prestige_gain();
        if self.ascend_confirm {
            text(
                &self.fonts.ui,
                &format!("Reset this run for +{}% permanent damage?", gain * 20),
                141.0,
                808.0,
                11.0,
                PINK,
            );
            if self.button(
                Rect::new(667.0, 789.0, 119.0, 35.0),
                "Keep playing",
                true,
                false,
            ) {
                action = Some(Action::CancelAscend);
            }
            if self.button(
                Rect::new(800.0, 789.0, 152.0, 35.0),
                "Ascend now",
                gain > 0,
                true,
            ) {
                action = Some(Action::ConfirmAscend);
            }
        } else {
            text(
                &self.fonts.ui,
                &format!("CURRENT BLESSING  +{}%", self.game.ascension * 20),
                141.0,
                810.0,
                9.0,
                LILAC,
            );
            if self.button(
                Rect::new(730.0, 790.0, 222.0, 35.0),
                &if gain > 0 {
                    format!("Ascend  ·  +{}% damage", gain * 20)
                } else {
                    "Unlock at wave 50".into()
                },
                gain > 0,
                true,
            ) {
                action = Some(Action::Ascend);
            }
        }
        action
    }
    fn modal(&mut self) -> Option<Action> {
        if self.modal == 0 {
            return None;
        }
        draw_rectangle(0.0, 0.0, W, H, Color::new(0.025, 0.02, 0.06, 0.85));
        let r = Rect::new(420.0, 175.0, 600.0, 548.0);
        card(r, PANEL);
        let mut action = None;
        text(
            &self.fonts.ui,
            if self.modal == 1 {
                "MAKE YOURSELF AT HOME"
            } else {
                "A LITTLE GUIDANCE"
            },
            451.0,
            214.0,
            9.0,
            PINK,
        );
        text(
            &self.fonts.title,
            if self.modal == 1 {
                "Your moonlit corner"
            } else {
                "Welcome to Moonveil"
            },
            450.0,
            255.0,
            37.0,
            TEXT,
        );
        let close = Rect::new(963.0, 195.0, 33.0, 33.0);
        let over = close.contains(self.mouse);
        if over {
            self.hovered = true;
            if self.clicked {
                action = Some(Action::Close);
            }
        }
        text(
            &self.fonts.ui,
            "×",
            974.0,
            218.0,
            23.0,
            if over { PINK } else { MUTED },
        );
        if self.modal == 1 {
            for (i, (title, desc, on, a)) in [
                (
                    "Sound effects",
                    "Soft strikes, tiny rewards, a little magic.",
                    self.game.sound,
                    Action::Sound,
                ),
                (
                    "Moonlight ambience",
                    "An original, gently drifting music loop.",
                    self.game.music,
                    Action::Music,
                ),
                (
                    "Petals & particles",
                    "A little sparkle goes a long way.",
                    self.game.particles,
                    Action::Particles,
                ),
            ]
            .iter()
            .enumerate()
            {
                let y = 301.0 + i as f32 * 88.0;
                text(&self.fonts.ui, title, 452.0, y, 15.0, TEXT);
                text(&self.fonts.ui, desc, 452.0, y + 24.0, 11.0, MUTED);
                let rr = Rect::new(930.0, y - 17.0, 52.0, 27.0);
                rounded(rr, 13.5, if *on { PINK } else { BORDER });
                draw_circle(
                    if *on { rr.x + 38.0 } else { rr.x + 14.0 },
                    rr.y + 13.5,
                    9.5,
                    if *on { BG } else { MUTED },
                );
                if rr.contains(self.mouse) {
                    self.hovered = true;
                    if self.clicked {
                        action = Some(*a);
                    }
                }
            }
            line(452.0, 552.0, 536.0);
            text(
                &self.fonts.ui,
                "Your journey saves automatically every 15 seconds.",
                452.0,
                582.0,
                12.0,
                TEXT,
            );
            text(
                &self.fonts.ui,
                "Offline expeditions: 50% gold efficiency, up to 8 hours.",
                452.0,
                607.0,
                11.0,
                MUTED,
            );
            text(
                &self.fonts.ui,
                "Display: Fit to window  ·  Proportions locked  ·  F11 fullscreen",
                452.0,
                627.0,
                10.0,
                TEAL,
            );
            let rr = Rect::new(452.0, 643.0, 536.0, 43.0);
            rounded(rr, 8.0, PINK);
            center(&self.fonts.ui, "Save my journey", 720.0, 670.0, 13.0, BG);
            if rr.contains(self.mouse) {
                self.hovered = true;
                if self.clicked {
                    action = Some(Action::Save);
                }
            }
        } else {
            for (i, (title, desc)) in [
                (
                    "01   Meet Yuki",
                    "Click the floating spirit or press Space to strike.",
                ),
                (
                    "02   Grow your little constellation",
                    "Recruited companions join Yuki in battle. Level them with gold.",
                ),
                (
                    "03   Make a little magic",
                    "Q: Crescent slash   W: Sakura storm   E: Lunar blessing",
                ),
                (
                    "04   Find something worth keeping",
                    "Train, forge relics, collect quests, and discover new realms.",
                ),
                (
                    "05   Come back to a warmer world",
                    "Your companions gather gold while you're away.",
                ),
            ]
            .iter()
            .enumerate()
            {
                let y = 301.0 + i as f32 * 67.0;
                text(&self.fonts.ui, title, 452.0, y, 13.0, PINK);
                text(&self.fonts.ui, desc, 452.0, y + 23.0, 11.0, MUTED);
            }
            text(
                &self.fonts.ui,
                "Esc closes this panel.  F11 toggles fullscreen.  S saves.",
                452.0,
                670.0,
                11.0,
                MUTED,
            );
        }
        action
    }
    fn render(&mut self) -> Option<Action> {
        clear_background(BG);
        self.hovered = false;
        let mut a = self.header();
        if let Some(x) = self.sidebar() {
            a = Some(x);
        }
        let content = match self.tab {
            Tab::Adventure => {
                let a = self.adventure();
                self.party_strip().or(a)
            }
            Tab::Companions => self.companions(),
            Tab::Forge => self.forge(),
            Tab::Realms => self.realms(),
            Tab::Quests => self.quests(),
        };
        if let Some(x) = content {
            a = Some(x);
        }
        if let Some(x) = self.training() {
            a = Some(x);
        }
        for p in &self.particles {
            star(p.pos.x, p.pos.y, p.size, alpha(p.color, p.life / p.max));
        }
        for f in &self.floats {
            center(
                &self.fonts.ui,
                &f.text,
                f.pos.x,
                f.pos.y,
                if f.text.starts_with("CRIT") {
                    21.0
                } else {
                    16.0
                },
                alpha(f.color, (f.life / f.max * 2.0).min(1.0)),
            );
        }
        line(118.0, 865.0, 1294.0);
        draw_circle(126.0, 883.0, 2.5, if self.save_error { PINK } else { TEAL });
        text(
            &self.fonts.ui,
            if self.save_error {
                "Save unavailable · check Settings"
            } else if self.saved_message > 0.0 {
                "Journey saved"
            } else {
                "Your journey saves automatically"
            },
            137.0,
            887.0,
            9.0,
            MUTED,
        );
        center(
            &self.fonts.ui,
            "Made of moonlight & small beginnings",
            762.0,
            887.0,
            9.0,
            alpha(MUTED, 0.7),
        );
        right(
            &self.fonts.ui,
            &format!("v1.1   ·   {} spirits purified", self.game.kills),
            1411.0,
            887.0,
            9.0,
            MUTED,
        );
        if self.modal > 0 {
            a = None;
            if let Some(x) = self.modal() {
                a = Some(x);
            }
        }
        for (i, t) in self.toasts.iter().enumerate() {
            let y = 100.0 + i as f32 * 73.0;
            let opacity = (t.life * 3.0).min(1.0);
            let rr = Rect::new(1015.0, y, 380.0, 63.0);
            rounded(rr, 10.0, alpha(Color::new(0.22, 0.17, 0.28, 1.0), opacity));
            draw_line(
                rr.x + 1.0,
                y + 12.0,
                rr.x + 1.0,
                y + 51.0,
                2.0,
                alpha(PINK, opacity),
            );
            text(
                &self.fonts.ui,
                &t.title,
                1032.0,
                y + 24.0,
                12.0,
                alpha(TEXT, opacity),
            );
            text(
                &self.fonts.ui,
                &t.detail,
                1032.0,
                y + 45.0,
                9.5,
                alpha(MUTED, opacity),
            );
        }
        if self.hovered {
            macroquad::miniquad::window::set_mouse_cursor(macroquad::miniquad::CursorIcon::Pointer);
        } else {
            macroquad::miniquad::window::set_mouse_cursor(macroquad::miniquad::CursorIcon::Default);
        }
        a
    }
}

#[macroquad::main(config)]
async fn main() {
    rand::srand(game::now());
    let fonts = Fonts {
        ui: load_ttf_font_from_bytes(include_bytes!("../assets/Manrope-Medium.ttf"))
            .expect("Bundled UI font"),
        title: load_ttf_font_from_bytes(include_bytes!("../assets/Cormorant.ttf"))
            .expect("Bundled title font"),
    };
    let fox = embedded_texture(include_bytes!("../assets/moonfox.png"));
    let art = Art {
        scenes: [
            include_bytes!("../assets/sanctuary.png").as_slice(),
            include_bytes!("../assets/jade.png").as_slice(),
            include_bytes!("../assets/twilight.png").as_slice(),
            include_bytes!("../assets/ember.png").as_slice(),
            include_bytes!("../assets/palace.png").as_slice(),
        ]
        .iter()
        .map(|bytes| Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png)))
        .collect(),
        heroine: Texture2D::from_file_with_format(
            include_bytes!("../assets/heroine.png"),
            Some(ImageFormat::Png),
        ),
        fox: fox.clone(),
        enemies: std::iter::once(fox)
            .chain(
                [
                    include_bytes!("../assets/enemy-slime.png").as_slice(),
                    include_bytes!("../assets/enemy-kodama.png").as_slice(),
                    include_bytes!("../assets/enemy-moth.png").as_slice(),
                    include_bytes!("../assets/enemy-lantern.png").as_slice(),
                    include_bytes!("../assets/enemy-golem.png").as_slice(),
                    include_bytes!("../assets/enemy-oni.png").as_slice(),
                    include_bytes!("../assets/enemy-dragon.png").as_slice(),
                ]
                .iter()
                .map(|bytes| embedded_texture(bytes)),
            )
            .collect(),
        party: [
            include_bytes!("../assets/party-ren.png").as_slice(),
            include_bytes!("../assets/party-mizuki.png").as_slice(),
            include_bytes!("../assets/party-akari.png").as_slice(),
            include_bytes!("../assets/party-yoru.png").as_slice(),
            include_bytes!("../assets/party-tsukiko.png").as_slice(),
        ]
        .iter()
        .map(|bytes| embedded_texture(bytes))
        .collect(),
        portraits: [
            include_bytes!("../assets/ren.png").as_slice(),
            include_bytes!("../assets/mizuki.png").as_slice(),
            include_bytes!("../assets/akari.png").as_slice(),
            include_bytes!("../assets/yoru.png").as_slice(),
            include_bytes!("../assets/tsukiko.png").as_slice(),
        ]
        .iter()
        .map(|bytes| Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png)))
        .collect(),
    };
    let audio = Audio {
        hit: load_sound_from_bytes(include_bytes!("../assets/strike.wav"))
            .await
            .expect("Bundled strike"),
        coin: load_sound_from_bytes(include_bytes!("../assets/chime.wav"))
            .await
            .expect("Bundled chime"),
        magic: load_sound_from_bytes(include_bytes!("../assets/magic.wav"))
            .await
            .expect("Bundled magic"),
        music: load_sound_from_bytes(include_bytes!("../assets/moonlight.wav"))
            .await
            .expect("Bundled ambience"),
    };
    let smoke = std::env::args().any(|s| s == "--smoke-test");
    let (game, offline_gold, load_warning) = if smoke {
        (Game::default(), 0.0, None)
    } else {
        game::load()
    };
    if game.music {
        play_sound(
            &audio.music,
            PlaySoundParams {
                looped: true,
                volume: 0.16,
            },
        );
    }
    let mut app = App {
        game,
        fonts,
        art,
        audio,
        tab: Tab::Adventure,
        modal: 0,
        ascend_confirm: false,
        mouse: Vec2::ZERO,
        clicked: false,
        hovered: false,
        particles: vec![],
        floats: vec![],
        toasts: vec![],
        time: 0.0,
        last_save: 0.0,
        auto_tick: 0.0,
        auto_gold: 0.0,
        hit_flash: 0.0,
        strike_anim: 0.0,
        spawn_anim: 0.0,
        combo: 0,
        combo_time: 0.0,
        cooldown: [0.0; 3],
        buff: 0.0,
        skill_flash: 0.0,
        save_error: false,
        saved_message: 0.0,
        offline_gold,
        smoke,
        frame: 0,
    };
    if let Some(warning) = load_warning {
        app.toast("A note about your journey", warning);
    }
    if app.offline_gold > 0.0 {
        app.toast(
            "Welcome back, moonchild",
            format!(
                "Your companions gathered {} spirit gold",
                number(app.offline_gold)
            ),
        );
    }
    if !app.smoke {
        app.persist();
    }
    prevent_quit();
    let mut fullscreen = false;
    loop {
        let dt = get_frame_time().min(0.10);
        if app.smoke {
            app.game.best = app.game.best.max(app.game.stage);
            app.game.run_best = app.game.run_best.max(app.game.stage);
        }
        app.tick(dt);
        let fit = display::DisplayFit::new(screen_width(), screen_height());
        set_camera(&fit.camera());
        let (mx, my) = mouse_position();
        let virtual_mouse = fit.to_game(vec2(mx, my));
        app.mouse = virtual_mouse.unwrap_or(vec2(-1000.0, -1000.0));
        app.clicked = virtual_mouse.is_some() && is_mouse_button_pressed(MouseButton::Left);
        if is_key_pressed(KeyCode::Escape) {
            if app.modal > 0 {
                app.handle(Action::Close);
            } else {
                app.handle(Action::Settings);
            }
        }
        if is_key_pressed(KeyCode::F11) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }
        if app.modal == 0 {
            if is_key_pressed(KeyCode::Space) {
                app.handle(Action::Strike);
            }
            for (i, key) in [KeyCode::Q, KeyCode::W, KeyCode::E].iter().enumerate() {
                if is_key_pressed(*key) {
                    app.handle(Action::Skill(i));
                }
            }
            if is_key_pressed(KeyCode::S) && !app.smoke {
                app.persist();
            }
        }
        if let Some(action) = app.render() {
            app.handle(action);
        }
        fit_borders(fit);
        if is_quit_requested() {
            if !app.smoke {
                app.persist();
            }
            break;
        }
        if app.smoke {
            match app.frame {
                20 => app.handle(Action::Strike),
                30 => app.handle(Action::Skill(0)),
                70 => app.toasts.clear(),
                80 => {
                    app.toasts.clear();
                    capture("adventure");
                    app.game.gold = 100000.0;
                    app.tab = Tab::Companions;
                }
                100 => {
                    app.handle(Action::Companion(1));
                    app.handle(Action::Upgrade(0));
                }
                120 => {
                    capture("party");
                    app.tab = Tab::Forge;
                }
                140 => app.handle(Action::Forge(0)),
                170 => {
                    capture("forge");
                    app.tab = Tab::Quests;
                    app.toasts.clear();
                }
                200 => {
                    capture("quests");
                    app.tab = Tab::Realms;
                }
                230 => {
                    capture("realms");
                    app.modal = 1;
                }
                260 => {
                    capture("settings");
                    app.modal = 0;
                    app.toasts.clear();
                    app.tab = Tab::Adventure;
                    app.game.stage = 21;
                    app.game.hp = app.game.max_hp();
                }
                290 => {
                    capture("jade");
                    app.game.stage = 41;
                    app.game.hp = app.game.max_hp();
                }
                320 => {
                    capture("twilight");
                    app.game.stage = 61;
                    app.game.hp = app.game.max_hp();
                }
                350 => {
                    capture("ember");
                    app.game.stage = 81;
                    app.game.hp = app.game.max_hp();
                }
                380 => {
                    capture("palace");
                    app.game.companions = [1; 6];
                    app.game.stage = 1;
                    app.game.hp = app.game.max_hp();
                }
                410 => {
                    capture("battle-party");
                    app.game.stage = 2;
                    app.game.hp = app.game.max_hp();
                }
                430 => {
                    capture("enemy-slime");
                    app.game.stage = 3;
                    app.game.hp = app.game.max_hp();
                }
                450 => {
                    capture("enemy-kodama");
                    app.game.stage = 21;
                    app.game.hp = app.game.max_hp();
                }
                470 => {
                    capture("enemy-moth");
                    app.game.stage = 41;
                    app.game.hp = app.game.max_hp();
                }
                490 => {
                    capture("enemy-lantern");
                    app.game.stage = 10;
                    app.game.hp = app.game.max_hp();
                }
                510 => {
                    capture("enemy-golem");
                    app.game.stage = 20;
                    app.game.hp = app.game.max_hp();
                }
                530 => {
                    capture("enemy-oni");
                    app.game.stage = 30;
                    app.game.hp = app.game.max_hp();
                }
                550 => {
                    capture("enemy-dragon");
                    request_new_screen_size(1200.0, 600.0);
                }
                580 => {
                    capture("scaling-wide");
                    request_new_screen_size(850.0, 850.0);
                }
                610 => {
                    capture("scaling-square");
                    request_new_screen_size(720.0, 900.0);
                }
                640 => {
                    capture("scaling-portrait");
                    set_fullscreen(true);
                }
                690 => {
                    capture("scaling-fullscreen");
                    set_fullscreen(false);
                }
                710 => {
                    println!(
                        "UI smoke test completed: six views, five realms, all eight enemies, six recruited battle companions, and wide, square, portrait and fullscreen scaling."
                    );
                    break;
                }
                _ => {}
            }
        }
        next_frame().await;
    }
}
