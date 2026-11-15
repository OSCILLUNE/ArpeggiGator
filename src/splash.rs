//! Small ArpeggiGator controls for the 16 Levels SHIFT menu.

use mpc_sample_sdk::draw::Graphics;
use crate::EngineMode;

const INK: u32 = 0xff16082f;
const BG: u32 = 0xff09051f;
const PURPLE: u32 = 0xff6720b5;
const VIOLET: u32 = 0xffa62be8;
const PINK: u32 = 0xffed36cf;
const GREEN: u32 = 0xff38d42e;
const LIME: u32 = 0xffa6f52a;
const GOLD: u32 = 0xffffcf36;
const CREAM: u32 = 0xfffff0d5;

const SHADOW: u32 = 0xff075c39;
const MID: u32 = 0xff18a852;
const HIGHLIGHT: u32 = 0xffa8ed67;

/// Paint custom B1/B2/B3 captions while SHIFT is held and active overlay for
/// ARP or MELODY engine modes.
///
/// # Safety
/// Call only from the SDK's live `on_paint` callback.
pub unsafe fn draw_arp_overlay(
    graphics: &mut Graphics,
    show_shift_menu: bool,
    enabled: bool,
    mode: EngineMode,
    b2_label: &str,
    b3_label: &str,
    preview: (u8, bool, &str),
    now_ms: u64,
) {
    let (held_pads, hold, rate) = preview;
    // SAFETY: the caller owns the live Graphics handle for this paint.
    unsafe {
        if show_shift_menu {
            draw_shift_labels(graphics, mode, b2_label, b3_label);
        }
        if enabled {
            match mode {
                EngineMode::Arp => {
                    draw_arp_view(graphics, b2_label, b3_label, held_pads, hold, rate, now_ms);
                }
                EngineMode::Melody => {
                    draw_melody_view(graphics, b2_label, b3_label, held_pads, hold, rate, now_ms);
                }
                EngineMode::Mutator => {
                    draw_mutator_view(graphics, b2_label, b3_label, held_pads, hold, rate, now_ms);
                }
                EngineMode::Acordator => {
                    draw_acordator_view(graphics, b2_label, b3_label, held_pads, hold, rate, now_ms);
                }
                EngineMode::Orbitator => {
                    draw_orbitator_view(graphics, b2_label, b3_label, held_pads, hold, rate, now_ms);
                }
                _ => {}
            }
        }
    }
}

unsafe fn draw_engine_right_panel(
    graphics: &mut Graphics,
    mode_title: &str,
    param_line: &str,
    hold: bool,
    rate: &str,
    mode_color: u32,
    param_color: u32,
    card_bg: u32,
    card_border: u32,
) {
    unsafe {
        // Vertical divider separating the animation stage from control column
        graphics.colour(card_border);
        graphics.rect(180, 50, 1, 136);

        // 1. Mode title
        graphics.colour(mode_color);
        graphics.text(mode_title, 185, 54, 118, 16, 13.0);

        // 2. HOLD status
        graphics.colour(if hold { LIME } else { 0xffff4053 });
        graphics.text(
            if hold { "HOLD: ON" } else { "HOLD: OFF" },
            185,
            72,
            118,
            16,
            13.0,
        );

        // 3. Parameters
        graphics.colour(param_color);
        graphics.text(param_line, 185, 90, 118, 16, 13.0);

        // 4. Rate Card
        graphics.colour(card_bg);
        graphics.rect(184, 114, 118, 68);
        graphics.colour(card_border);
        graphics.rect(184, 114, 118, 1);
        graphics.rect(184, 181, 118, 1);
        graphics.rect(184, 114, 1, 68);
        graphics.rect(301, 114, 1, 68);

        graphics.colour(mode_color);
        graphics.text("TURN DATA: RATE", 186, 118, 114, 14, 11.0);
        graphics.colour(LIME);
        graphics.text(rate, 186, 134, 114, 42, 22.0);
    }
}

unsafe fn draw_arp_view(
    graphics: &mut Graphics,
    pattern_label: &str,
    octaves_label: &str,
    held_pads: u8,
    hold: bool,
    rate: &str,
    now_ms: u64,
) {
    unsafe {
        // Solid full-width background covering native sample waveform & playhead cursor
        graphics.colour(0xff09051f);
        graphics.rect(14, 48, 292, 140);

        // Right side info panel
        let param_str = format!("{pattern_label}  {octaves_label}");
        draw_engine_right_panel(
            graphics,
            "MODE: ARP",
            &param_str,
            hold,
            rate,
            0xff00ffff,
            0xffff70d6,
            0xff120b2e,
            0xff3d1e6d,
        );

        let phase = if held_pads > 0 { (now_ms / 150) % 8 } else { 0 };
        let chewing = held_pads > 0 && (3..=5).contains(&phase);
        draw_gator(graphics, chewing, phase, held_pads > 0, 0);
        if held_pads > 0 {
            let note_x = 168 - ((now_ms / 65) % 55) as i32;
            draw_note(graphics, note_x, chewing);
        }
    }
}

unsafe fn draw_melody_view(
    graphics: &mut Graphics,
    bars_label: &str,
    mel_label: &str,
    held_pads: u8,
    hold: bool,
    rate: &str,
    now_ms: u64,
) {
    unsafe {
        // Solid full-width background covering native sample waveform & playhead cursor
        graphics.colour(0xff070818);
        graphics.rect(14, 48, 292, 140);

        // Right side info panel
        let param_str = format!("{bars_label}  {mel_label}");
        draw_engine_right_panel(
            graphics,
            "MODE: MELODY",
            &param_str,
            hold,
            rate,
            0xff63ddec,
            0xffff70d6,
            0xff140b24,
            0xff4a154b,
        );

        // Club scene on the left
        draw_club_scene(graphics, held_pads > 0, now_ms);
    }
}

unsafe fn draw_mutator_view(
    graphics: &mut Graphics,
    intensity_label: &str,
    motif_label: &str,
    held_pads: u8,
    hold: bool,
    rate: &str,
    now_ms: u64,
) {
    unsafe {
        // Solid full-width background covering native sample waveform & playhead cursor
        graphics.colour(0xff061014);
        graphics.rect(14, 48, 292, 140);

        // Right side info panel
        let param_str = format!("{intensity_label}  {motif_label}");
        draw_engine_right_panel(
            graphics,
            "MODE: MUTATOR",
            &param_str,
            hold,
            rate,
            0xff00e5ff,
            0xffffaa00,
            0xff0a1c22,
            0xff14424a,
        );

        // Cyber Mutator Lab on the left
        draw_mutator_lab(graphics, held_pads > 0, now_ms);
    }
}

unsafe fn draw_mutator_lab(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        // DNA scanner lines / cyber grid in background
        let scan_phase = ((now_ms / 80) % 30) as i32;
        graphics.colour(0xff0c2438);
        for col in 0..7 {
            let x = 18 + col * 24;
            graphics.rect(x, 48, 1, 140);
        }
        for row in 0..6 {
            let y = 52 + row * 24;
            graphics.rect(14, y, 166, 1);
        }

        // Animated horizontal scanner laser
        graphics.colour(if active { 0xff00e5ff } else { 0xff006688 });
        let laser_y = 50 + scan_phase * 4;
        graphics.rect(14, laser_y, 166, 2);

        // Bubbling mutagen beaker on left side (x: 18..36, y: 130..175)
        graphics.colour(0xff16082f);
        graphics.rect(18, 140, 18, 32);
        graphics.rect(23, 128, 8, 12);
        // Beaker liquid
        graphics.colour(if active { 0xff39ff14 } else { 0xff188820 });
        graphics.rect(20, 148, 14, 22);
        // Bubbles in beaker
        let bubble_phase = ((now_ms / 120) % 5) as i32;
        graphics.colour(0xffffffff);
        graphics.rect(24, 160 - bubble_phase * 4, 3, 3);
        graphics.rect(28, 166 - ((bubble_phase + 2) % 5) * 4, 2, 2);

        // Draw Gator in Cyber Lab behind synth console
        draw_dj_booth(graphics, active, now_ms);
        draw_mutator_gator(graphics, active, now_ms);
    }
}

unsafe fn draw_acordator_view(
    graphics: &mut Graphics,
    mood_label: &str,
    style_label: &str,
    held_pads: u8,
    hold: bool,
    rate: &str,
    now_ms: u64,
) {
    unsafe {
        // Solid full-width background covering native sample waveform & playhead cursor
        graphics.colour(0xff160a14);
        graphics.rect(14, 48, 292, 140);

        // Right side info panel
        let param_str = format!("{mood_label}  {style_label}");
        draw_engine_right_panel(
            graphics,
            "MODE: ACORD",
            &param_str,
            hold,
            rate,
            0xffffaa00, // Warm Amber
            0xffffdd66, // Warm Gold
            0xff24101e,
            0xff52203e,
        );

        // Warm chord stage on the left
        draw_acordator_stage(graphics, held_pads > 0, now_ms);
    }
}

unsafe fn draw_acordator_stage(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        // Warm sunset synthwave glow background
        graphics.colour(0xff2a0820);
        graphics.rect(14, 48, 166, 140);

        // 4 warm chord harmony pillars behind gator
        let pulse = ((now_ms / 140) % 4) as usize;
        let pillar_colors = [0xffff4422, 0xffff8800, 0xffffbb00, 0xffffee55];
        for col in 0..4 {
            let x = 24 + (col as i32 * 40);
            let h = if active {
                35 + (((col + pulse) % 4) as i32 * 12)
            } else {
                25
            };
            graphics.colour(pillar_colors[col]);
            graphics.rect(x, 140 - h, 20, h);
            graphics.colour(0xffffffff);
            graphics.rect(x + 2, 142 - h, 16, 2);
        }

        draw_dj_booth(graphics, active, now_ms);
        draw_acordator_gator(graphics, active, now_ms);
    }
}

unsafe fn draw_orbitator_view(
    graphics: &mut Graphics,
    atmo_label: &str,
    dyn_label: &str,
    held_pads: u8,
    hold: bool,
    rate: &str,
    now_ms: u64,
) {
    unsafe {
        // Solid full-width background covering native sample waveform & playhead cursor
        graphics.colour(0xff050212);
        graphics.rect(14, 48, 292, 140);

        // Right side info panel
        let param_str = format!("{atmo_label}  {dyn_label}");
        draw_engine_right_panel(
            graphics,
            "MODE: ORBIT",
            &param_str,
            hold,
            rate,
            0xffc77dff, // Cosmic Violet
            0xff00ffff, // Nebula Cyan
            0xff110826,
            0xff431878,
        );

        // Deep Space Orbitator Scene on the left
        draw_orbitator_space(graphics, held_pads > 0, now_ms);
    }
}

unsafe fn draw_orbitator_space(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        // Deep space black/indigo canvas
        graphics.colour(0xff060312);
        graphics.rect(14, 48, 166, 140);

        // Twinkling stars
        let star_twinkle = ((now_ms / 150) % 3) as usize;
        let stars = [
            (22, 58), (40, 72), (75, 52), (130, 60), (160, 54),
            (28, 110), (145, 95), (162, 130), (35, 165), (150, 168)
        ];
        for (i, &(sx, sy)) in stars.iter().enumerate() {
            let col = if (i + star_twinkle) % 3 == 0 {
                0xffffffff
            } else {
                0xff7b68ee
            };
            graphics.colour(col);
            graphics.rect(sx, sy, 2, 2);
        }

        // Central Planet with orbiting ring behind gator
        let px = 97;
        let py = 105;
        // Planet body
        graphics.colour(0xff1a1040);
        graphics.rect(px - 16, py - 16, 32, 32);
        graphics.colour(0xff3a1f7a);
        graphics.rect(px - 14, py - 14, 28, 28);
        graphics.colour(0xff5c35b5);
        graphics.rect(px - 10, py - 10, 20, 20);

        // Orbiting celestial ring
        graphics.colour(0xff00d4ff);
        graphics.rect(px - 26, py, 52, 2);
        graphics.rect(px - 22, py - 3, 44, 1);
        graphics.rect(px - 22, py + 3, 44, 1);

        // Orbiting satellite dot
        let orbit_angle = ((now_ms / 60) % 32) as i32;
        let sat_x = px - 28 + (orbit_angle * 56 / 32);
        let sat_y = py - 4 + ((orbit_angle - 16).abs() / 3);
        graphics.colour(if active { 0xffa6f52a } else { 0xffffffff });
        graphics.rect(sat_x, sat_y, 4, 4);

        // Astronaut Gator in full NASA space suit & helmet
        draw_dj_booth(graphics, active, now_ms);
        draw_orbitator_gator(graphics, active, now_ms);
    }
}

unsafe fn draw_club_scene(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        draw_lasers(graphics, active, now_ms);
        draw_disco_ball(graphics, now_ms);
        draw_dj_booth(graphics, active, now_ms);
        draw_dj_gator(graphics, active, now_ms);
    }
}

unsafe fn draw_lasers(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        let pulse = if active { (now_ms / 100) % 4 } else { 0 };
        graphics.colour(if active && (pulse == 0 || pulse == 2) { 0xff00ffff } else { 0xff005577 });
        for i in 0..30 {
            let x = 12 + i * 5;
            let y = 50 + (i * 3);
            graphics.rect(x, y, 3, 1);
        }

        graphics.colour(if active && (pulse == 1 || pulse == 3) { 0xffff0088 } else { 0xff770044 });
        for i in 0..30 {
            let x = 160 - (i * 5);
            let y = 52 + (i * 3);
            graphics.rect(x, y, 3, 1);
        }

        if active {
            graphics.colour(0xffa6f52a);
            let sweep = ((now_ms / 150) % 6) as i32 - 3;
            for i in 0..25 {
                let x = 88 + (sweep * i / 2);
                let y = 70 + (i * 3);
                graphics.rect(x, y, 2, 1);
            }
        }
    }
}

unsafe fn draw_disco_ball(graphics: &mut Graphics, now_ms: u64) {
    unsafe {
        let cx = 88;
        let cy = 64;

        graphics.colour(0xff808898);
        graphics.rect(cx, 45, 1, 11);

        graphics.colour(0xff181c24);
        graphics.rect(cx - 7, cy - 8, 15, 17);
        graphics.rect(cx - 9, cy - 6, 19, 13);
        graphics.rect(cx - 10, cy - 4, 21, 9);

        let shimmer = ((now_ms / 120) % 4) as usize;
        let colors = [0xffffffff, 0xffa0e8ff, 0xffc0c8d8, 0xff687890];

        for row in 0..4 {
            for col in 0..4 {
                let c_idx = (row + col + shimmer) % 4;
                graphics.colour(colors[c_idx]);
                let x = cx - 8 + (col as i32 * 4);
                let y = cy - 7 + (row as i32 * 4);
                graphics.rect(x + 1, y + 1, 3, 3);
            }
        }

        let spot_phase = (now_ms / 200) % 8;
        let spots = [
            (cx - 35 + (spot_phase as i32 * 2), 170),
            (cx + 20 - (spot_phase as i32 * 2), 175),
            (cx - 15 + (spot_phase as i32 * 3), 162),
            (cx + 40 - (spot_phase as i32 * 3), 166),
        ];
        graphics.colour(0xffffffff);
        for &(sx, sy) in &spots {
            graphics.rect(sx, sy, 2, 2);
        }
    }
}

unsafe fn draw_dj_booth(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        graphics.colour(0xff120524);
        graphics.rect(26, 146, 132, 38);
        graphics.colour(0xff220a40);
        graphics.rect(28, 148, 128, 34);

        graphics.colour(0xff00f0ff);
        graphics.rect(26, 146, 132, 2);
        graphics.colour(0xffff0088);
        graphics.rect(26, 182, 132, 2);
        graphics.colour(0xff7720a8);
        graphics.rect(26, 148, 2, 34);
        graphics.rect(156, 148, 2, 34);

        // Front DJ booth marquee panel
        graphics.colour(0xff0e041d);
        graphics.rect(32, 153, 120, 24);
        // Neon borders (cyan top-left, hot pink bottom-right)
        graphics.colour(0xff00ffff);
        graphics.rect(32, 153, 120, 1);
        graphics.rect(32, 153, 1, 24);
        graphics.colour(0xffff0088);
        graphics.rect(32, 176, 120, 1);
        graphics.rect(151, 153, 1, 24);

        // Dynamic LED VU equalizers flanking the logo
        let vu_step = if active { ((now_ms / 100) % 4) as i32 } else { 1 };
        for bar in 0..3 {
            let h = match (bar, vu_step) {
                (0, 0) => 8,
                (0, 1) => 12,
                (0, 2) => 16,
                (0, _) => 10,
                (1, 0) => 14,
                (1, 1) => 8,
                (1, 2) => 12,
                (1, _) => 16,
                (2, 0) => 10,
                (2, 1) => 15,
                (2, 2) => 7,
                (2, _) => 12,
                _ => 10,
            };
            let lx = 35 + bar * 3;
            let rx = 143 + bar * 3;
            for seg in 0..(h / 4) {
                let col = match seg {
                    0 => 0xff00ff66,
                    1 => 0xffffcc00,
                    2 => 0xffff7700,
                    _ => 0xffff0066,
                };
                graphics.colour(col);
                let y = 173 - (seg * 4);
                graphics.rect(lx, y, 2, 3);
                graphics.rect(rx, y, 2, 3);
            }
        }

        // ArpeggiGator logo text: glowing drop shadow + crisp neon cyan text
        graphics.colour(0xffff0088);
        graphics.text("ArpeggiGator", 46, 157, 94, 18, 12.0);
        graphics.colour(0xffe0ffff);
        graphics.text("ArpeggiGator", 45, 156, 94, 18, 12.0);

        draw_turntable(graphics, 34, 140, active, now_ms, 0xffff2050);
        draw_turntable(graphics, 118, 140, active, now_ms, 0xff00e5ff);

        graphics.colour(0xff252835);
        graphics.rect(72, 141, 38, 12);
        graphics.colour(0xff404858);
        graphics.rect(76, 144, 30, 2);
        let fader_pos = if active { ((now_ms / 150) % 5) as i32 * 4 } else { 8 };
        graphics.colour(0xffffffff);
        graphics.rect(78 + fader_pos, 143, 4, 4);

        let vu_level = if active { ((now_ms / 100) % 4) as i32 } else { 1 };
        for led in 0..3 {
            let col = if led <= vu_level {
                match led {
                    0 => 0xff00ff66,
                    1 => 0xffffdd00,
                    _ => 0xffff2233,
                }
            } else {
                0xff1a2028
            };
            graphics.colour(col);
            graphics.rect(87, 148 + (led * 2), 2, 1);
            graphics.rect(93, 148 + (led * 2), 2, 1);
        }
    }
}

unsafe fn draw_turntable(
    graphics: &mut Graphics,
    x: i32,
    y: i32,
    active: bool,
    now_ms: u64,
    label_color: u32,
) {
    unsafe {
        graphics.colour(0xff303542);
        graphics.rect(x - 2, y - 1, 36, 14);

        graphics.colour(0xff111216);
        graphics.rect(x, y, 32, 12);
        graphics.colour(0xff1e222a);
        graphics.rect(x + 2, y + 1, 28, 10);

        graphics.colour(label_color);
        graphics.rect(x + 12, y + 4, 8, 4);

        let rot = if active { ((now_ms / 80) % 4) as i32 } else { 0 };
        graphics.colour(0xffffffff);
        let (dot_x, dot_y) = match rot {
            0 => (x + 14, y + 2),
            1 => (x + 22, y + 5),
            2 => (x + 14, y + 9),
            _ => (x + 6, y + 5),
        };
        graphics.rect(dot_x, dot_y, 2, 2);
    }
}

unsafe fn draw_dj_gator(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        let bob = if active {
            [0, -2, -3, -1, 0, 2, 3, 1][((now_ms / 110) % 8) as usize]
        } else {
            0
        };

        let gx = 60;
        let gy = 78 + bob;

        // Torso / DJ body behind booth (y: gy + 38 to 68)
        graphics.colour(INK);
        graphics.rect(gx + 6, gy + 38, 44, 28);
        graphics.colour(SHADOW);
        graphics.rect(gx + 8, gy + 40, 40, 25);
        graphics.colour(GREEN);
        graphics.rect(gx + 10, gy + 41, 36, 23);

        // Gator chest / belly scutes (yellowish green)
        graphics.colour(LIME);
        graphics.rect(gx + 22, gy + 42, 14, 21);
        graphics.colour(MID);
        for row in 0..4 {
            graphics.rect(gx + 23, gy + 46 + (row * 4), 12, 1);
        }

        // Gold chain with 'G' pendant
        graphics.colour(GOLD);
        graphics.rect(gx + 18, gy + 40, 2, 6);
        graphics.rect(gx + 38, gy + 40, 2, 6);
        graphics.rect(gx + 20, gy + 45, 18, 2);
        graphics.rect(gx + 27, gy + 47, 5, 5);
        graphics.colour(CREAM);
        graphics.rect(gx + 28, gy + 48, 3, 3);

        // Head and snout outline
        graphics.colour(INK);
        graphics.rect(gx + 10, gy + 12, 36, 28);
        graphics.rect(gx + 32, gy + 20, 32, 18);
        graphics.rect(gx + 58, gy + 18, 6, 4);

        // Green head body
        graphics.colour(SHADOW);
        graphics.rect(gx + 12, gy + 14, 32, 24);
        graphics.rect(gx + 34, gy + 22, 28, 15);
        graphics.rect(gx + 59, gy + 19, 4, 3);
        graphics.colour(GREEN);
        graphics.rect(gx + 14, gy + 16, 28, 20);
        graphics.rect(gx + 34, gy + 21, 27, 13);
        graphics.rect(gx + 59, gy + 19, 4, 3);

        // Highlights on crown and snout
        graphics.colour(HIGHLIGHT);
        graphics.rect(gx + 16, gy + 16, 22, 3);
        graphics.rect(gx + 36, gy + 21, 22, 2);
        graphics.rect(gx + 59, gy + 19, 3, 2);

        // Nostril
        graphics.colour(INK);
        graphics.rect(gx + 59, gy + 21, 3, 2);

        // Mouth & sharp teeth
        graphics.colour(INK);
        graphics.rect(gx + 32, gy + 32, 30, 2);
        graphics.colour(CREAM);
        graphics.rect(gx + 36, gy + 30, 3, 3);
        graphics.rect(gx + 42, gy + 30, 3, 4);
        graphics.rect(gx + 48, gy + 30, 3, 3);
        graphics.rect(gx + 54, gy + 30, 3, 4);
        graphics.rect(gx + 60, gy + 30, 2, 3);

        if active {
            // Mouth slightly open to the beat
            graphics.colour(INK);
            graphics.rect(gx + 34, gy + 33, 28, 4);
            graphics.colour(PINK);
            graphics.rect(gx + 38, gy + 34, 18, 2);
            graphics.colour(CREAM);
            graphics.rect(gx + 38, gy + 35, 2, 2);
            graphics.rect(gx + 44, gy + 35, 2, 2);
            graphics.rect(gx + 50, gy + 35, 2, 2);
            graphics.rect(gx + 56, gy + 35, 2, 2);
            graphics.colour(MID);
            graphics.rect(gx + 34, gy + 37, 28, 2);
        }

        // DJ Cyber Visor / Sunglasses
        graphics.colour(INK);
        graphics.rect(gx + 18, gy + 22, 26, 9);
        graphics.colour(0xff7b1fa2);
        graphics.rect(gx + 19, gy + 23, 24, 7);
        // Glint animation
        let glint_x = gx + 20 + if active { ((now_ms / 120) % 5) as i32 * 4 } else { 8 };
        graphics.colour(0xffffffff);
        graphics.rect(glint_x, gy + 24, 3, 5);
        graphics.colour(0xff00ffff);
        graphics.rect(glint_x - 1, gy + 25, 1, 3);
        graphics.rect(glint_x + 3, gy + 25, 1, 3);

        // Big Over-Ear Red DJ Headphones
        graphics.colour(INK);
        graphics.rect(gx + 10, gy + 6, 36, 8);
        graphics.colour(0xff990000);
        graphics.rect(gx + 12, gy + 8, 32, 4);
        graphics.colour(0xffff3355);
        graphics.rect(gx + 14, gy + 8, 26, 2);

        // Left headphone ear-cup
        graphics.colour(INK);
        graphics.rect(gx + 6, gy + 14, 10, 20);
        graphics.colour(0xff990000);
        graphics.rect(gx + 7, gy + 15, 8, 18);
        graphics.colour(0xffff1a35);
        graphics.rect(gx + 8, gy + 17, 6, 14);
        graphics.colour(0xffff8090);
        graphics.rect(gx + 9, gy + 18, 3, 10);

        // Right headphone ear-cup tip
        graphics.colour(INK);
        graphics.rect(gx + 44, gy + 14, 6, 12);
        graphics.colour(0xffff1a35);
        graphics.rect(gx + 45, gy + 15, 4, 10);

        // Left arm: Pumping the crowd / hyping the club in the air (far away from snout)
        let pump_dy = if active {
            [-4, -7, -9, -7, -4, -1, 1, 0][((now_ms / 100) % 8) as usize]
        } else {
            0
        };
        let hand_y = gy + 16 + pump_dy;

        // Left bicep / arm rising
        graphics.colour(INK);
        graphics.rect(gx - 6, gy + 30, 12, 16);
        graphics.colour(GREEN);
        graphics.rect(gx - 5, gy + 31, 10, 14);

        // Fist and claws in the air
        graphics.colour(INK);
        graphics.rect(gx - 14, hand_y, 14, 16);
        graphics.colour(GREEN);
        graphics.rect(gx - 13, hand_y + 1, 12, 14);
        graphics.colour(HIGHLIGHT);
        graphics.rect(gx - 12, hand_y + 2, 4, 10);
        graphics.colour(CREAM);
        graphics.rect(gx - 13, hand_y - 2, 3, 3);
        graphics.rect(gx - 9, hand_y - 3, 3, 4);
        graphics.rect(gx - 5, hand_y - 2, 3, 3);

        // Right arm: Mixing and scratching the vinyl / fader below the snout
        let scratch_dx = if active { ((now_ms / 80) % 4) as i32 - 1 } else { 0 };
        graphics.colour(INK);
        graphics.rect(gx + 44, gy + 42, 14, 12);
        graphics.colour(GREEN);
        graphics.rect(gx + 45, gy + 43, 12, 10);

        // Hand & claws on turntable platter / mixer
        graphics.colour(INK);
        graphics.rect(gx + 50 + scratch_dx, gy + 53, 16, 7);
        graphics.colour(GREEN);
        graphics.rect(gx + 51 + scratch_dx, gy + 54, 14, 5);
        graphics.colour(CREAM);
        graphics.rect(gx + 51 + scratch_dx, gy + 57, 3, 2);
        graphics.rect(gx + 55 + scratch_dx, gy + 57, 3, 2);
        graphics.rect(gx + 59 + scratch_dx, gy + 57, 3, 2);
    }
}

/// Mutator mode mascot: Two-headed mutant alligator with both hands down scratching the turntables.
unsafe fn draw_mutator_gator(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        let bob_left = if active {
            [-1, -3, -2, 0, 2, 3, 1, 0][((now_ms / 110) % 8) as usize]
        } else {
            0
        };
        let bob_right = if active {
            [1, 0, -2, -3, -1, 0, 2, 3][((now_ms / 110) % 8) as usize]
        } else {
            0
        };
        let bob_body = (bob_left + bob_right) / 2;

        let gx = 60;
        let gy = 78 + bob_body;

        // Torso / mutant DJ body behind booth (y: gy + 38 to 68)
        graphics.colour(INK);
        graphics.rect(gx + 6, gy + 38, 44, 28);
        graphics.colour(SHADOW);
        graphics.rect(gx + 8, gy + 40, 40, 25);
        graphics.colour(GREEN);
        graphics.rect(gx + 10, gy + 41, 36, 23);

        // Gator chest / belly scutes (yellowish green)
        graphics.colour(LIME);
        graphics.rect(gx + 20, gy + 42, 16, 21);
        graphics.colour(MID);
        for row in 0..4 {
            graphics.rect(gx + 21, gy + 46 + (row * 4), 14, 1);
        }

        // Gold chains for two heads
        graphics.colour(GOLD);
        graphics.rect(gx + 14, gy + 40, 2, 6);
        graphics.rect(gx + 40, gy + 40, 2, 6);
        graphics.rect(gx + 16, gy + 45, 24, 2);
        graphics.rect(gx + 26, gy + 47, 5, 5);
        graphics.colour(CREAM);
        graphics.rect(gx + 27, gy + 48, 3, 3);

        // --- LEFT HEAD (Facing LEFT with sharp teeth & cyan cyber visor) ---
        let h1x = gx + 4;
        let h1y = gy + bob_left;

        // Left neck connecting to torso
        graphics.colour(INK);
        graphics.rect(h1x - 2, h1y + 26, 18, 14);
        graphics.colour(SHADOW);
        graphics.rect(h1x, h1y + 27, 15, 12);
        graphics.colour(GREEN);
        graphics.rect(h1x + 1, h1y + 28, 13, 10);

        // Left Head Skull
        graphics.colour(INK);
        graphics.rect(h1x - 6, h1y + 10, 26, 22);
        graphics.colour(SHADOW);
        graphics.rect(h1x - 4, h1y + 12, 22, 18);
        graphics.colour(GREEN);
        graphics.rect(h1x - 3, h1y + 13, 20, 16);
        graphics.colour(HIGHLIGHT);
        graphics.rect(h1x - 2, h1y + 13, 16, 2);

        // Left Snout pointing left
        graphics.colour(INK);
        graphics.rect(h1x - 26, h1y + 18, 24, 15);
        graphics.colour(SHADOW);
        graphics.rect(h1x - 24, h1y + 19, 21, 13);
        graphics.colour(GREEN);
        graphics.rect(h1x - 23, h1y + 20, 19, 11);
        graphics.colour(HIGHLIGHT);
        graphics.rect(h1x - 22, h1y + 20, 16, 2);

        // Left nostril
        graphics.colour(INK);
        graphics.rect(h1x - 24, h1y + 21, 3, 2);

        // Left mouth & teeth
        graphics.colour(INK);
        graphics.rect(h1x - 24, h1y + 29, 24, 2);
        graphics.colour(CREAM);
        graphics.rect(h1x - 23, h1y + 27, 2, 3);
        graphics.rect(h1x - 18, h1y + 27, 3, 3);
        graphics.rect(h1x - 12, h1y + 27, 3, 3);
        graphics.rect(h1x - 6, h1y + 27, 3, 3);
        if active {
            graphics.colour(INK);
            graphics.rect(h1x - 22, h1y + 30, 20, 3);
            graphics.colour(PINK);
            graphics.rect(h1x - 18, h1y + 31, 14, 2);
        }

        // Left cyber visor (Electric Cyan)
        graphics.colour(INK);
        graphics.rect(h1x - 8, h1y + 19, 18, 7);
        graphics.colour(0xff00b4d8);
        graphics.rect(h1x - 7, h1y + 20, 16, 5);
        // Animated glint
        let glint_l = if active { ((now_ms / 100) % 4) as i32 * 3 } else { 3 };
        graphics.colour(0xffffffff);
        graphics.rect(h1x - 6 + glint_l, h1y + 21, 3, 3);

        // Left headphone cup
        graphics.colour(INK);
        graphics.rect(h1x + 18, h1y + 12, 6, 16);
        graphics.colour(0xffff1a35);
        graphics.rect(h1x + 19, h1y + 13, 4, 14);

        // --- RIGHT HEAD (Facing RIGHT with sharp teeth & cyan cyber visor) ---
        let h2x = gx + 34;
        let h2y = gy + bob_right;

        // Right neck connecting to torso
        graphics.colour(INK);
        graphics.rect(h2x - 2, h2y + 26, 18, 14);
        graphics.colour(SHADOW);
        graphics.rect(h2x, h2y + 27, 15, 12);
        graphics.colour(GREEN);
        graphics.rect(h2x + 1, h2y + 28, 13, 10);

        // Right Head Skull
        graphics.colour(INK);
        graphics.rect(h2x - 4, h2y + 10, 26, 22);
        graphics.colour(SHADOW);
        graphics.rect(h2x - 2, h2y + 12, 22, 18);
        graphics.colour(GREEN);
        graphics.rect(h2x - 1, h2y + 13, 20, 16);
        graphics.colour(HIGHLIGHT);
        graphics.rect(h2x, h2y + 13, 16, 2);

        // Right Snout pointing right
        graphics.colour(INK);
        graphics.rect(h2x + 18, h2y + 18, 24, 15);
        graphics.colour(SHADOW);
        graphics.rect(h2x + 19, h2y + 19, 21, 13);
        graphics.colour(GREEN);
        graphics.rect(h2x + 20, h2y + 20, 19, 11);
        graphics.colour(HIGHLIGHT);
        graphics.rect(h2x + 21, h2y + 20, 16, 2);

        // Right nostril
        graphics.colour(INK);
        graphics.rect(h2x + 37, h2y + 21, 3, 2);

        // Right mouth & teeth
        graphics.colour(INK);
        graphics.rect(h2x + 16, h2y + 29, 24, 2);
        graphics.colour(CREAM);
        graphics.rect(h2x + 19, h2y + 27, 3, 3);
        graphics.rect(h2x + 25, h2y + 27, 3, 3);
        graphics.rect(h2x + 31, h2y + 27, 3, 3);
        graphics.rect(h2x + 37, h2y + 27, 2, 3);
        if active {
            graphics.colour(INK);
            graphics.rect(h2x + 18, h2y + 30, 20, 3);
            graphics.colour(PINK);
            graphics.rect(h2x + 20, h2y + 31, 14, 2);
        }

        // Right cyber visor (Electric Cyan)
        graphics.colour(INK);
        graphics.rect(h2x + 6, h2y + 19, 18, 7);
        graphics.colour(0xff00b4d8);
        graphics.rect(h2x + 7, h2y + 20, 16, 5);
        // Animated glint
        let glint_r = if active { (((now_ms / 100) + 2) % 4) as i32 * 3 } else { 3 };
        graphics.colour(0xffffffff);
        graphics.rect(h2x + 8 + glint_r, h2y + 21, 3, 3);

        // Right headphone cup
        graphics.colour(INK);
        graphics.rect(h2x - 8, h2y + 12, 6, 16);
        graphics.colour(0xffff1a35);
        graphics.rect(h2x - 7, h2y + 13, 4, 14);

        // --- BOTH HANDS DOWN ON THE TURNTABLES ("obie ręce na płytach, w dole") ---
        // Left turntable is at x: 34, y: 140
        let left_scratch = if active { ((now_ms / 85) % 4) as i32 - 1 } else { 0 };

        // Left upper arm sloping down to left deck
        graphics.colour(INK);
        graphics.rect(gx - 4, gy + 40, 14, 14);
        graphics.colour(GREEN);
        graphics.rect(gx - 3, gy + 41, 12, 12);

        // Left forearm extending to platter
        graphics.colour(INK);
        graphics.rect(gx - 14, gy + 48, 14, 12);
        graphics.colour(GREEN);
        graphics.rect(gx - 13, gy + 49, 12, 10);

        // Left hand and claws scratching the left vinyl platter
        let hand_lx = 42 + left_scratch;
        let hand_ly = 138;
        graphics.colour(INK);
        graphics.rect(hand_lx, hand_ly, 16, 7);
        graphics.colour(GREEN);
        graphics.rect(hand_lx + 1, hand_ly + 1, 14, 5);
        graphics.colour(CREAM);
        graphics.rect(hand_lx + 2, hand_ly + 4, 3, 2);
        graphics.rect(hand_lx + 6, hand_ly + 4, 3, 2);
        graphics.rect(hand_lx + 10, hand_ly + 4, 3, 2);

        // Right turntable is at x: 118, y: 140
        let right_scratch = if active { (((now_ms / 80) + 2) % 4) as i32 - 1 } else { 0 };

        // Right upper arm sloping down to right deck
        graphics.colour(INK);
        graphics.rect(gx + 44, gy + 40, 14, 14);
        graphics.colour(GREEN);
        graphics.rect(gx + 45, gy + 41, 12, 12);

        // Right forearm extending to platter
        graphics.colour(INK);
        graphics.rect(gx + 54, gy + 48, 14, 12);
        graphics.colour(GREEN);
        graphics.rect(gx + 55, gy + 49, 12, 10);

        // Right hand and claws scratching the right vinyl platter
        let hand_rx = 124 + right_scratch;
        let hand_ry = 138;
        graphics.colour(INK);
        graphics.rect(hand_rx, hand_ry, 16, 7);
        graphics.colour(GREEN);
        graphics.rect(hand_rx + 1, hand_ry + 1, 14, 5);
        graphics.colour(CREAM);
        graphics.rect(hand_rx + 2, hand_ry + 4, 3, 2);
        graphics.rect(hand_rx + 6, hand_ry + 4, 3, 2);
        graphics.rect(hand_rx + 10, hand_ry + 4, 3, 2);
    }
}

/// Acordator mode mascot: Dark-green alligator with normal face as in Melody.
unsafe fn draw_acordator_gator(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        let bob = if active {
            [0, -2, -3, -1, 0, 2, 3, 1][((now_ms / 110) % 8) as usize]
        } else {
            0
        };

        let gx = 60;
        let gy = 78 + bob;

        // Dark green palette for ACORDATOR
        const ACORD_DARK_GREEN: u32 = 0xff0d5a23;
        const ACORD_DARK_SHADOW: u32 = 0xff063313;
        const ACORD_DARK_MID: u32 = 0xff1b7834;
        const ACORD_DARK_HIGHLIGHT: u32 = 0xff2ea84b;
        const ACORD_BELLY: u32 = 0xff4b8535;
        const ACORD_BELLY_STRIPE: u32 = 0xff27571b;

        // Torso / DJ body behind booth (y: gy + 38 to 68)
        graphics.colour(INK);
        graphics.rect(gx + 6, gy + 38, 44, 28);
        graphics.colour(ACORD_DARK_SHADOW);
        graphics.rect(gx + 8, gy + 40, 40, 25);
        graphics.colour(ACORD_DARK_GREEN);
        graphics.rect(gx + 10, gy + 41, 36, 23);

        // Gator chest / belly scutes (dark muted sage)
        graphics.colour(ACORD_BELLY);
        graphics.rect(gx + 22, gy + 42, 14, 21);
        graphics.colour(ACORD_BELLY_STRIPE);
        for row in 0..4 {
            graphics.rect(gx + 23, gy + 46 + (row * 4), 12, 1);
        }

        // Gold chain with 'G' pendant
        graphics.colour(GOLD);
        graphics.rect(gx + 18, gy + 40, 2, 6);
        graphics.rect(gx + 38, gy + 40, 2, 6);
        graphics.rect(gx + 20, gy + 45, 18, 2);
        graphics.rect(gx + 27, gy + 47, 5, 5);
        graphics.colour(CREAM);
        graphics.rect(gx + 28, gy + 48, 3, 3);

        // Head and snout outline
        graphics.colour(INK);
        graphics.rect(gx + 10, gy + 12, 36, 28);
        graphics.rect(gx + 32, gy + 20, 32, 18);
        graphics.rect(gx + 58, gy + 18, 6, 4);

        // Dark green head body
        graphics.colour(ACORD_DARK_SHADOW);
        graphics.rect(gx + 12, gy + 14, 32, 24);
        graphics.rect(gx + 34, gy + 22, 28, 15);
        graphics.rect(gx + 59, gy + 19, 4, 3);
        graphics.colour(ACORD_DARK_GREEN);
        graphics.rect(gx + 14, gy + 16, 28, 20);
        graphics.rect(gx + 34, gy + 21, 27, 13);
        graphics.rect(gx + 59, gy + 19, 4, 3);

        // Highlights on crown and snout
        graphics.colour(ACORD_DARK_HIGHLIGHT);
        graphics.rect(gx + 16, gy + 16, 22, 3);
        graphics.rect(gx + 36, gy + 21, 22, 2);
        graphics.rect(gx + 59, gy + 19, 3, 2);

        // Nostril
        graphics.colour(INK);
        graphics.rect(gx + 59, gy + 21, 3, 2);

        // Mouth & sharp teeth (identical to MELODY)
        graphics.colour(INK);
        graphics.rect(gx + 32, gy + 32, 30, 2);
        graphics.colour(CREAM);
        graphics.rect(gx + 36, gy + 30, 3, 3);
        graphics.rect(gx + 42, gy + 30, 3, 4);
        graphics.rect(gx + 48, gy + 30, 3, 3);
        graphics.rect(gx + 54, gy + 30, 3, 4);
        graphics.rect(gx + 60, gy + 30, 2, 3);

        if active {
            // Mouth slightly open to the beat
            graphics.colour(INK);
            graphics.rect(gx + 34, gy + 33, 28, 4);
            graphics.colour(PINK);
            graphics.rect(gx + 38, gy + 34, 18, 2);
            graphics.colour(CREAM);
            graphics.rect(gx + 38, gy + 35, 2, 2);
            graphics.rect(gx + 44, gy + 35, 2, 2);
            graphics.rect(gx + 50, gy + 35, 2, 2);
            graphics.rect(gx + 56, gy + 35, 2, 2);
            graphics.colour(ACORD_DARK_MID);
            graphics.rect(gx + 34, gy + 37, 28, 2);
        }

        // DJ Cyber Visor / Sunglasses (Normal face like in MELODY!)
        graphics.colour(INK);
        graphics.rect(gx + 18, gy + 22, 26, 9);
        graphics.colour(0xff7b1fa2);
        graphics.rect(gx + 19, gy + 23, 24, 7);
        // Glint animation
        let glint_x = gx + 20 + if active { ((now_ms / 120) % 5) as i32 * 4 } else { 8 };
        graphics.colour(0xffffffff);
        graphics.rect(glint_x, gy + 24, 3, 5);
        graphics.colour(0xff00ffff);
        graphics.rect(glint_x - 1, gy + 25, 1, 3);
        graphics.rect(glint_x + 3, gy + 25, 1, 3);

        // Big Over-Ear Red DJ Headphones
        graphics.colour(INK);
        graphics.rect(gx + 10, gy + 6, 36, 8);
        graphics.colour(0xff990000);
        graphics.rect(gx + 12, gy + 8, 32, 4);
        graphics.colour(0xffff3355);
        graphics.rect(gx + 14, gy + 8, 26, 2);

        // Left headphone ear-cup
        graphics.colour(INK);
        graphics.rect(gx + 6, gy + 14, 10, 20);
        graphics.colour(0xff990000);
        graphics.rect(gx + 7, gy + 15, 8, 18);
        graphics.colour(0xffff1a35);
        graphics.rect(gx + 8, gy + 17, 6, 14);
        graphics.colour(0xffff8090);
        graphics.rect(gx + 9, gy + 18, 3, 10);

        // Right headphone ear-cup tip
        graphics.colour(INK);
        graphics.rect(gx + 44, gy + 14, 6, 12);
        graphics.colour(0xffff1a35);
        graphics.rect(gx + 45, gy + 15, 4, 10);

        // Left arm: Pumping the crowd / hyping the club in the air
        let pump_dy = if active {
            [-4, -7, -9, -7, -4, -1, 1, 0][((now_ms / 100) % 8) as usize]
        } else {
            0
        };
        let hand_y = gy + 16 + pump_dy;

        // Left bicep / arm rising (dark green)
        graphics.colour(INK);
        graphics.rect(gx - 6, gy + 30, 12, 16);
        graphics.colour(ACORD_DARK_GREEN);
        graphics.rect(gx - 5, gy + 31, 10, 14);

        // Fist and claws in the air
        graphics.colour(INK);
        graphics.rect(gx - 14, hand_y, 14, 16);
        graphics.colour(ACORD_DARK_GREEN);
        graphics.rect(gx - 13, hand_y + 1, 12, 14);
        graphics.colour(ACORD_DARK_HIGHLIGHT);
        graphics.rect(gx - 12, hand_y + 2, 4, 10);
        graphics.colour(CREAM);
        graphics.rect(gx - 13, hand_y - 2, 3, 3);
        graphics.rect(gx - 9, hand_y - 3, 3, 4);
        graphics.rect(gx - 5, hand_y - 2, 3, 3);

        // Right arm: Mixing and scratching the vinyl / fader
        let scratch_dx = if active { ((now_ms / 80) % 4) as i32 - 1 } else { 0 };
        graphics.colour(INK);
        graphics.rect(gx + 44, gy + 42, 14, 12);
        graphics.colour(ACORD_DARK_GREEN);
        graphics.rect(gx + 45, gy + 43, 12, 10);

        // Hand & claws on turntable platter / mixer
        graphics.colour(INK);
        graphics.rect(gx + 50 + scratch_dx, gy + 53, 16, 7);
        graphics.colour(ACORD_DARK_GREEN);
        graphics.rect(gx + 51 + scratch_dx, gy + 54, 14, 5);
        graphics.colour(CREAM);
        graphics.rect(gx + 51 + scratch_dx, gy + 57, 3, 2);
        graphics.rect(gx + 55 + scratch_dx, gy + 57, 3, 2);
        graphics.rect(gx + 59 + scratch_dx, gy + 57, 3, 2);
    }
}

/// Orbitator mode mascot: White NASA space suit with astronaut helmet and gold visor.
unsafe fn draw_orbitator_gator(graphics: &mut Graphics, active: bool, now_ms: u64) {
    unsafe {
        let bob = if active {
            [0, -2, -3, -1, 0, 2, 3, 1][((now_ms / 110) % 8) as usize]
        } else {
            0
        };

        let gx = 60;
        let gy = 78 + bob;

        // --- NASA ASTRONAUT SUIT TORSO (Pure White EMU Space Suit) ---
        // Suit outline
        graphics.colour(0xff161a26);
        graphics.rect(gx + 6, gy + 36, 44, 30);
        graphics.colour(0xffb8c0d2); // Pressurized shadow
        graphics.rect(gx + 8, gy + 38, 40, 27);
        graphics.colour(0xfff6f8fc); // White space suit
        graphics.rect(gx + 10, gy + 39, 36, 25);

        // NASA Chest Pack (DCM - Display and Control Module)
        graphics.colour(0xff161a26);
        graphics.rect(gx + 20, gy + 43, 18, 18);
        graphics.colour(0xff747e94);
        graphics.rect(gx + 21, gy + 44, 16, 16);
        // Telemetry LED indicators
        graphics.colour(0xff00f0ff); // Oxygen
        graphics.rect(gx + 23, gy + 46, 3, 2);
        graphics.colour(if active { 0xff39ff14 } else { 0xffffcc00 }); // Status
        graphics.rect(gx + 28, gy + 46, 3, 2);
        graphics.colour(0xffff3355); // Warning
        graphics.rect(gx + 32, gy + 46, 3, 2);
        // Switches & pressure dial
        graphics.colour(0xff222736);
        graphics.rect(gx + 24, gy + 51, 10, 2);
        graphics.rect(gx + 24, gy + 55, 4, 3);
        graphics.rect(gx + 30, gy + 55, 4, 3);

        // NASA Meatball Patch (Blue circle + red chevron)
        graphics.colour(0xff0b3d91); // NASA blue
        graphics.rect(gx + 11, gy + 43, 7, 7);
        graphics.colour(0xffe0342b); // Red chevron
        graphics.rect(gx + 13, gy + 45, 4, 2);
        graphics.colour(0xffffffff); // Tiny white star
        graphics.rect(gx + 12, gy + 44, 1, 1);

        // USA Mission Stripe on chest/arm
        graphics.colour(0xffffffff);
        graphics.rect(gx + 40, gy + 43, 8, 5);
        graphics.colour(0xffe0342b);
        graphics.rect(gx + 40, gy + 43, 8, 1);
        graphics.rect(gx + 40, gy + 45, 8, 1);
        graphics.rect(gx + 40, gy + 47, 8, 1);
        graphics.colour(0xff0b3d91);
        graphics.rect(gx + 40, gy + 43, 4, 3);

        // --- NASA ASTRONAUT HELMET (Hełm Kosmiczny) ---
        // Neck ring / collar seal (Anodized NASA red locking ring)
        graphics.colour(0xff161a26);
        graphics.rect(gx + 12, gy + 32, 42, 6);
        graphics.colour(0xffe0342b); // Red anodized collar ring
        graphics.rect(gx + 14, gy + 33, 38, 4);
        graphics.colour(0xffffffff);
        graphics.rect(gx + 16, gy + 34, 34, 1);

        // Outer White Space Helmet Shell
        graphics.colour(0xff161a26);
        graphics.rect(gx + 8, gy + 6, 38, 28);
        graphics.rect(gx + 34, gy + 12, 34, 22);
        graphics.rect(gx + 64, gy + 14, 6, 12);

        graphics.colour(0xffb8c0d2);
        graphics.rect(gx + 10, gy + 8, 34, 25);
        graphics.rect(gx + 36, gy + 14, 30, 19);
        graphics.rect(gx + 64, gy + 15, 5, 10);

        graphics.colour(0xfff6f8fc); // White helmet dome
        graphics.rect(gx + 12, gy + 9, 31, 23);
        graphics.rect(gx + 36, gy + 15, 29, 17);
        graphics.rect(gx + 63, gy + 16, 5, 8);

        // Helmet top reflection highlight
        graphics.colour(0xffffffff);
        graphics.rect(gx + 16, gy + 9, 22, 2);
        graphics.rect(gx + 38, gy + 15, 18, 2);

        // Apollo Gold Visor (Słynna złota osłona przeciwsłoneczna)
        graphics.colour(0xff161a26); // Visor frame
        graphics.rect(gx + 20, gy + 15, 46, 17);
        graphics.colour(0xffc48200); // Gold visor shadow
        graphics.rect(gx + 22, gy + 16, 43, 15);
        graphics.colour(0xffffb703); // Rich gold visor
        graphics.rect(gx + 23, gy + 17, 41, 13);

        // Visor solar glint streak across the visor curvature
        graphics.colour(0xffffffff);
        graphics.rect(gx + 25, gy + 18, 8, 3);
        graphics.rect(gx + 36, gy + 19, 14, 2);
        graphics.rect(gx + 54, gy + 20, 6, 2);

        // Side EVA spotlight on helmet
        graphics.colour(0xff161a26);
        graphics.rect(gx + 5, gy + 14, 6, 10);
        graphics.colour(0xffffffff);
        graphics.rect(gx + 6, gy + 15, 4, 8);
        graphics.colour(0xff00f0ff);
        graphics.rect(gx + 6, gy + 17, 3, 4);

        // --- WHITE ASTRONAUT ARMS & SPACE GLOVES ---
        // Left arm: Victory pose / grooving in space with big astronaut glove
        let pump_dy = if active {
            [-4, -7, -9, -7, -4, -1, 1, 0][((now_ms / 100) % 8) as usize]
        } else {
            0
        };
        let hand_y = gy + 16 + pump_dy;

        // Left white pressurized sleeve
        graphics.colour(0xff161a26);
        graphics.rect(gx - 6, gy + 28, 14, 18);
        graphics.colour(0xfff6f8fc);
        graphics.rect(gx - 5, gy + 29, 12, 16);
        // Ribbed elbow joint
        graphics.colour(0xffb8c0d2);
        graphics.rect(gx - 5, gy + 35, 12, 2);
        graphics.rect(gx - 5, gy + 39, 12, 2);

        // Space Glove in the air
        graphics.colour(0xff161a26);
        graphics.rect(gx - 14, hand_y, 16, 18);
        graphics.colour(0xfff6f8fc);
        graphics.rect(gx - 13, hand_y + 1, 14, 16);
        graphics.colour(0xffb8c0d2);
        graphics.rect(gx - 12, hand_y + 3, 4, 12);
        // Blue anodized wrist disconnect ring (NASA standard: blue for left)
        graphics.colour(0xff0b3d91);
        graphics.rect(gx - 13, hand_y + 14, 14, 2);
        // Dark grip pads on glove fingers
        graphics.colour(0xff484e5e);
        graphics.rect(gx - 13, hand_y - 2, 3, 3);
        graphics.rect(gx - 9, hand_y - 3, 3, 4);
        graphics.rect(gx - 5, hand_y - 2, 3, 3);

        // Right arm: White space sleeve & space glove on turntable
        let scratch_dx = if active { ((now_ms / 80) % 4) as i32 - 1 } else { 0 };
        graphics.colour(0xff161a26);
        graphics.rect(gx + 44, gy + 40, 15, 14);
        graphics.colour(0xfff6f8fc);
        graphics.rect(gx + 45, gy + 41, 13, 12);
        // Ribbed elbow joint
        graphics.colour(0xffb8c0d2);
        graphics.rect(gx + 47, gy + 45, 2, 7);

        // Right Space Glove on console/turntable
        graphics.colour(0xff161a26);
        graphics.rect(gx + 50 + scratch_dx, gy + 52, 17, 8);
        graphics.colour(0xfff6f8fc);
        graphics.rect(gx + 51 + scratch_dx, gy + 53, 15, 6);
        // Red anodized wrist disconnect ring (NASA standard: red for right)
        graphics.colour(0xffe0342b);
        graphics.rect(gx + 50 + scratch_dx, gy + 53, 2, 6);
        // Grip pads
        graphics.colour(0xff484e5e);
        graphics.rect(gx + 53 + scratch_dx, gy + 56, 3, 2);
        graphics.rect(gx + 57 + scratch_dx, gy + 56, 3, 2);
        graphics.rect(gx + 61 + scratch_dx, gy + 56, 3, 2);
    }
}

/// Draw the startup scene on the live app paint path.
///
/// # Safety
/// The graphics handle must belong to the active paint callback.
pub unsafe fn draw_boot_splash(graphics: &mut Graphics) {
    unsafe {
        // Deep purple/indigo background matching the artwork
        graphics.colour(0xff16082c);
        graphics.rect(0, 0, 320, 240);

        // Club stage floor at the bottom below the sequencer pads
        graphics.colour(0xff100521);
        graphics.rect(0, 178, 320, 62);

        // Perspective grid lines on stage floor
        graphics.colour(0xff3f135b);
        for y in [184, 193, 204, 219, 239] {
            graphics.rect(0, y, 320, 1);
        }
        for row in 0..12 {
            let y = 178 + row * 5;
            for slope in [-7, -5, -3, -1, 1, 3, 5, 7] {
                let x = 160 + slope * row * 3;
                if (0..320).contains(&x) {
                    graphics.rect(x, y, 2, 5);
                }
            }
        }

        // Draw the pixel-art DJ Alligator mascot with 4 synth pads
        for group in &crate::splash_art::GATOR_GROUPS {
            graphics.colour(group.color);
            for rect in group.rects {
                graphics.rect(
                    i32::from(rect.x),
                    i32::from(rect.y),
                    i32::from(rect.w),
                    i32::from(rect.h),
                );
            }
        }

        // Top title: "ArpeggiGator" with neon synthwave glow
        graphics.colour(0xff090314);
        graphics.text("ArpeggiGator", 28, 12, 268, 30, 23.0);
        graphics.colour(0xffa62be8); // Violet glow
        graphics.text("ArpeggiGator", 27, 11, 268, 30, 23.0);
        graphics.colour(0xffed36cf); // Neon pink glow
        graphics.text("ArpeggiGator", 26, 10, 268, 30, 23.0);
        graphics.colour(0xffffffff); // Bright white core
        graphics.text("ArpeggiGator", 25, 9, 268, 30, 23.0);

        // Bottom-right attribution: "from OSCILLUNE"
        graphics.colour(0xff000000);
        graphics.text("from OSCILLUNE", 186, 219, 130, 18, 13.0);
        graphics.colour(GOLD);
        graphics.text("from OSCILLUNE", 185, 218, 130, 18, 13.0);
    }
}

unsafe fn draw_shift_labels(
    graphics: &mut Graphics,
    mode: EngineMode,
    b2_label: &str,
    b3_label: &str,
) {
    // These three cells cover the MPC's B1/B2/B3 captions, leaving the rest
    // of the Sample or 16 Levels screen untouched.
    unsafe {
        let fill_b1 = match mode {
            EngineMode::Off => BG,
            EngineMode::Arp => LIME,
            EngineMode::Melody => 0xffe6007e, // Neon club pink
            EngineMode::Mutator => 0xff00d4ff, // Electric cyan
            EngineMode::Acordator => 0xffff9900, // Warm amber
            EngineMode::Orbitator => 0xff9d4edd, // Cosmic violet
        };
        label_cell(graphics, 13, 5, 88, fill_b1);
        label_cell(graphics, 111, 5, 98, BG);
        label_cell(graphics, 219, 5, 88, BG);

        match mode {
            EngineMode::Off => {
                draw_mini_gator(graphics, 47, 10);
            }
            EngineMode::Arp => {
                graphics.colour(INK);
                graphics.text("ARP", 17, 8, 80, 19, 15.0);
            }
            EngineMode::Melody => {
                graphics.colour(0xffffffff);
                graphics.text("MELODY", 17, 8, 80, 19, 14.0);
            }
            EngineMode::Mutator => {
                graphics.colour(INK);
                graphics.text("MUTATOR", 15, 8, 84, 19, 13.0);
            }
            EngineMode::Acordator => {
                graphics.colour(INK);
                graphics.text("ACORDATOR", 13, 8, 88, 19, 12.0);
            }
            EngineMode::Orbitator => {
                graphics.colour(0xffffffff);
                graphics.text("ORBITATOR", 14, 8, 86, 19, 12.0);
            }
        }
        graphics.colour(CREAM);
        graphics.text(b2_label, 115, 8, 90, 19, 13.0);
        graphics.colour(GOLD);
        graphics.text(b3_label, 223, 8, 80, 19, 15.0);
    }
}

unsafe fn draw_mini_gator(graphics: &mut Graphics, x: i32, y: i32) {
    unsafe {
        // Dark outline
        graphics.colour(INK);
        graphics.rect(x + 2, y + 2, 17, 12);
        graphics.rect(x + 5, y + 5, 17, 8);

        // Gator green head and snout
        graphics.colour(GREEN);
        graphics.rect(x + 3, y + 3, 14, 10);
        graphics.rect(x + 8, y + 6, 13, 6);

        // Highlight scales
        graphics.colour(HIGHLIGHT);
        graphics.rect(x + 5, y + 3, 6, 2);
        graphics.rect(x + 12, y + 6, 6, 2);

        // Dark sunglasses / visor
        graphics.colour(0xffa62be8);
        graphics.rect(x + 5, y + 5, 7, 4);
        graphics.colour(0xfffff0d5);
        graphics.rect(x + 6, y + 5, 2, 2);

        // DJ headphone band on left
        graphics.colour(0xffed3043);
        graphics.rect(x + 1, y + 3, 3, 8);
        graphics.rect(x + 2, y + 1, 6, 2);

        // Cute white tooth
        graphics.colour(CREAM);
        graphics.rect(x + 16, y + 10, 2, 2);
        graphics.colour(INK);
        graphics.rect(x + 9, y + 11, 11, 1);
    }
}

unsafe fn label_cell(graphics: &mut Graphics, x: i32, y: i32, width: i32, fill: u32) {
    unsafe {
        graphics.colour(fill);
        graphics.rect(x, y, width, 25);
        graphics.colour(if fill == BG { PURPLE } else { GREEN });
        graphics.rect(x, y, width, 1);
        graphics.rect(x, y + 24, width, 1);
        graphics.rect(x, y, 1, 25);
        graphics.rect(x + width - 1, y, 1, 25);
    }
}

// Scale the mascot into the central display area while leaving the softkeys clear.
unsafe fn scene_rect(graphics: &mut Graphics, x: i32, y: i32, width: i32, height: i32) {
    unsafe { graphics.rect(25 + (x - 25) * 2, 45 + (y - 103) * 2, width * 2, height * 2) };
}

unsafe fn draw_gator(
    graphics: &mut Graphics,
    chewing: bool,
    phase: u64,
    moving: bool,
    offset: i32,
) {
    // The water is a small irregular silhouette, not a panel over the waveform.
    // Idle is always phase zero; animation moves only while a pad is held.
    let x = 25 + offset;
    let bob = if moving {
        [0, -1, -2, -1, 0, 1, 2, 1][phase as usize]
    } else {
        0
    };
    let y = 103 + bob;
    let wave = if moving { (phase % 3) as i32 - 1 } else { 0 };
    const DEEP: u32 = 0xff07536c;
    const WATER: u32 = 0xff088ba3;
    const FOAM: u32 = 0xff6be5d5;
    const SHADOW: u32 = 0xff075c39;
    const MID: u32 = 0xff18a852;
    const HIGHLIGHT: u32 = 0xffa8ed67;
    unsafe {
        // Layered ripples and expanded water pool around the mascot.
        graphics.colour(DEEP);
        scene_rect(graphics, x - 8, 160, 82, 5);
        scene_rect(graphics, x - 5, 165, 76, 4);
        scene_rect(graphics, x + 2, 169, 65, 4);
        scene_rect(graphics, x + 10, 173, 48, 3);
        // Outer water ripples
        scene_rect(graphics, x - 12, 162, 5, 2);
        scene_rect(graphics, x - 10, 166, 4, 1);
        scene_rect(graphics, x + 72, 162, 5, 2);
        scene_rect(graphics, x + 74, 165, 4, 1);

        graphics.colour(WATER);
        scene_rect(graphics, x - 6 + wave, 158, 38, 3);
        scene_rect(graphics, x + 34 - wave, 159, 40, 3);
        scene_rect(graphics, x - 2, 164, 70, 3);
        scene_rect(graphics, x + 5, 168, 55, 2);
        scene_rect(graphics, x + 14, 172, 36, 2);

        graphics.colour(FOAM);
        scene_rect(graphics, x - 7 + wave, 157, 18, 2);
        scene_rect(graphics, x + 16 - wave, 158, 20, 2);
        scene_rect(graphics, x + 55 + wave, 158, 18, 2);
        scene_rect(graphics, x - 4 - wave, 163, 16, 1);
        scene_rect(graphics, x + 28 + wave, 164, 24, 1);
        scene_rect(graphics, x + 58 - wave, 165, 14, 1);
        scene_rect(graphics, x + 8 - wave, 169, 18, 1);
        scene_rect(graphics, x + 40 + wave, 170, 18, 1);
        scene_rect(graphics, x + 18 + wave, 173, 14, 1);
        // Small splash droplets on the outer ripples
        scene_rect(graphics, x - 10, 159, 2, 2);
        scene_rect(graphics, x + 70, 160, 2, 2);
        scene_rect(graphics, x - 6, 155, 2, 1);
        scene_rect(graphics, x + 66, 156, 2, 1);

        // Tail, body and face have a dark silhouette with stepped contours.
        graphics.colour(INK);
        scene_rect(graphics, x + 3, y + 37, 15, 6);
        scene_rect(graphics, x + 10, y + 31, 15, 16);
        scene_rect(graphics, x + 19, y + 23, 34, 31);
        scene_rect(graphics, x + 26, y + 16, 24, 10);
        scene_rect(graphics, x + 43, y + 24, 27, 24);
        scene_rect(graphics, x + 61, y + 30, 23, 16);
        scene_rect(graphics, x + 29, y + 48, 33, 13);
        graphics.colour(SHADOW);
        scene_rect(graphics, x + 13, y + 35, 15, 8);
        scene_rect(graphics, x + 21, y + 27, 29, 24);
        scene_rect(graphics, x + 32, y + 47, 28, 10);
        scene_rect(graphics, x + 47, y + 29, 32, 14);
        graphics.colour(MID);
        scene_rect(graphics, x + 20, y + 30, 30, 15);
        scene_rect(graphics, x + 28, y + 20, 19, 10);
        scene_rect(graphics, x + 46, y + 27, 21, 13);
        scene_rect(graphics, x + 62, y + 33, 19, 7);
        scene_rect(graphics, x + 33, y + 45, 22, 7);
        graphics.colour(GREEN);
        scene_rect(graphics, x + 22, y + 29, 18, 7);
        scene_rect(graphics, x + 29, y + 19, 14, 6);
        scene_rect(graphics, x + 49, y + 27, 14, 5);
        scene_rect(graphics, x + 64, y + 32, 12, 3);
        graphics.colour(HIGHLIGHT);
        scene_rect(graphics, x + 24, y + 28, 13, 2);
        scene_rect(graphics, x + 29, y + 20, 10, 2);
        scene_rect(graphics, x + 52, y + 27, 8, 2);
        scene_rect(graphics, x + 69, y + 33, 6, 2);

        // Oversized over-ear headphones move with the head in every frame.
        // The stepped arch and ear cup use separate shadows and highlights.
        graphics.colour(INK);
        scene_rect(graphics, x + 20, y + 15, 7, 16);
        scene_rect(graphics, x + 23, y + 10, 8, 7);
        scene_rect(graphics, x + 29, y + 8, 18, 6);
        scene_rect(graphics, x + 45, y + 11, 7, 9);
        scene_rect(graphics, x + 49, y + 16, 6, 8);
        scene_rect(graphics, x + 13, y + 22, 14, 24);
        scene_rect(graphics, x + 16, y + 19, 9, 30);
        graphics.colour(0xffa30f26);
        scene_rect(graphics, x + 22, y + 15, 4, 13);
        scene_rect(graphics, x + 26, y + 11, 6, 4);
        scene_rect(graphics, x + 31, y + 10, 15, 3);
        scene_rect(graphics, x + 46, y + 14, 4, 7);
        scene_rect(graphics, x + 15, y + 24, 10, 20);
        scene_rect(graphics, x + 18, y + 21, 6, 26);
        graphics.colour(0xffef3043);
        scene_rect(graphics, x + 24, y + 15, 3, 9);
        scene_rect(graphics, x + 28, y + 11, 16, 2);
        scene_rect(graphics, x + 17, y + 24, 7, 19);
        graphics.colour(0xffff7a73);
        scene_rect(graphics, x + 18, y + 25, 2, 11);
        scene_rect(graphics, x + 29, y + 11, 10, 1);

        // Sunglasses, glint, nostril and a small toothy jaw.
        graphics.colour(INK);
        scene_rect(graphics, x + 26, y + 24, 12, 9);
        scene_rect(graphics, x + 40, y + 24, 12, 9);
        scene_rect(graphics, x + 37, y + 26, 4, 2);
        scene_rect(graphics, x + 67, y + 35, 3, 2);
        scene_rect(graphics, x + 49, y + 39, 32, 3);
        graphics.colour(VIOLET);
        scene_rect(graphics, x + 28, y + 26, 8, 5);
        scene_rect(graphics, x + 42, y + 26, 8, 5);
        graphics.colour(CREAM);
        scene_rect(graphics, x + 29, y + 26, 3, 1);
        scene_rect(graphics, x + 43, y + 26, 3, 1);
        scene_rect(graphics, x + 54, y + 40, 3, 3);
        scene_rect(graphics, x + 65, y + 40, 3, 3);
        scene_rect(graphics, x + 75, y + 40, 3, 3);
        if chewing {
            graphics.colour(INK);
            scene_rect(graphics, x + 50, y + 43, 28, 5);
            graphics.colour(PINK);
            scene_rect(graphics, x + 55, y + 45, 15, 2);
            graphics.colour(MID);
            scene_rect(graphics, x + 50, y + 48, 27, 3);
        } else {
            graphics.colour(MID);
            scene_rect(graphics, x + 50, y + 43, 29, 3);
        }
        // Little forearms rest on the surface.
        graphics.colour(SHADOW);
        scene_rect(graphics, x + 24, y + 51, 13, 6);
        scene_rect(graphics, x + 48, y + 51, 13, 6);
        graphics.colour(GREEN);
        scene_rect(graphics, x + 24, y + 51, 11, 3);
        scene_rect(graphics, x + 49, y + 51, 10, 3);
    }
}

unsafe fn draw_note(graphics: &mut Graphics, x: i32, chewing: bool) {
    let y = 119;
    unsafe {
        // Two beamed eighth notes: a dark outline and highlights make the
        // red shape readable against both the waveform and the mascot.
        graphics.colour(INK);
        graphics.rect(x + 8, y + 4, 6, 19);
        graphics.rect(x + 24, y + 1, 6, 19);
        graphics.rect(x + 8, y + 3, 21, 6);
        graphics.rect(x + 2, y + 17, 12, 9);
        graphics.rect(x, y + 19, 16, 5);
        graphics.rect(x + 18, y + 14, 12, 9);
        graphics.rect(x + 16, y + 16, 16, 5);

        graphics.colour(0xff990f22);
        graphics.rect(x + 4, y + 23, 8, 2);
        graphics.rect(x + 20, y + 20, 8, 2);
        graphics.rect(x + 11, y + 8, 2, 12);
        graphics.rect(x + 27, y + 5, 2, 12);

        graphics.colour(0xffed3043);
        graphics.rect(x + 10, y + 6, 3, 15);
        graphics.rect(x + 26, y + 3, 3, 15);
        graphics.rect(x + 10, y + 5, 17, 3);
        graphics.rect(x + 3, y + 19, 10, 5);
        graphics.rect(x + 2, y + 20, 12, 3);
        graphics.rect(x + 19, y + 16, 10, 5);
        graphics.rect(x + 18, y + 17, 12, 3);

        graphics.colour(0xffff8580);
        graphics.rect(x + 11, y + 6, 1, 10);
        graphics.rect(x + 27, y + 4, 1, 9);
        graphics.rect(x + 4, y + 19, 5, 2);
        graphics.rect(x + 20, y + 16, 5, 2);

        if chewing && x < 205 {
            graphics.colour(CREAM);
            graphics.rect(187, y - 3, 2, 2);
        }
    }
}
