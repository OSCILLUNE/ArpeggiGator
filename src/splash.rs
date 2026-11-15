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
                    draw_arp_view(graphics, held_pads, hold, rate, now_ms);
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

unsafe fn draw_arp_view(
    graphics: &mut Graphics,
    held_pads: u8,
    hold: bool,
    rate: &str,
    now_ms: u64,
) {
    unsafe {
        graphics.colour(0xffd56aff);
        graphics.text("PRESS DATA", 189, 73, 125, 15, 13.0);
        graphics.colour(if hold { LIME } else { 0xffff4053 });
        graphics.text(
            if hold { "HOLD: ON" } else { "HOLD: OFF" },
            189,
            90,
            125,
            15,
            13.0,
        );
        graphics.colour(0xff63ddec);
        graphics.text("TURN DATA: RATE", 189, 147, 125, 15, 12.0);
        graphics.colour(LIME);
        graphics.text(rate, 194, 163, 116, 22, 19.0);
        let phase = if held_pads > 0 { (now_ms / 150) % 8 } else { 0 };
        let chewing = held_pads > 0 && (3..=5).contains(&phase);
        draw_gator(graphics, chewing, phase, held_pads > 0, 0);
        if held_pads > 0 {
            let note_x = 264 - ((now_ms / 65) % 76) as i32;
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
        // Right side information column
        graphics.colour(0xff63ddec);
        graphics.text("MODE: MELODY", 189, 73, 125, 15, 13.0);
        graphics.colour(if hold { LIME } else { 0xffff4053 });
        graphics.text(
            if hold { "HOLD: ON" } else { "HOLD: OFF" },
            189,
            90,
            125,
            15,
            13.0,
        );
        graphics.colour(0xffff70d6);
        graphics.text(&format!("{bars_label}  {mel_label}"), 189, 107, 125, 15, 13.0);
        graphics.colour(0xff63ddec);
        graphics.text("TURN DATA: RATE", 189, 147, 125, 15, 12.0);
        graphics.colour(LIME);
        graphics.text(rate, 194, 163, 116, 22, 19.0);

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
        // Right side information column
        graphics.colour(0xff00e5ff); // Electric Cyan
        graphics.text("MODE: MUTATOR", 189, 73, 125, 15, 13.0);
        graphics.colour(if hold { LIME } else { 0xffff4053 });
        graphics.text(
            if hold { "HOLD: ON" } else { "HOLD: OFF" },
            189,
            90,
            125,
            15,
            13.0,
        );
        graphics.colour(0xffffaa00); // Cyber amber
        graphics.text(
            &format!("{intensity_label}  {motif_label}"),
            189,
            107,
            125,
            15,
            13.0,
        );
        graphics.colour(0xff00e5ff);
        graphics.text("TURN DATA: RATE", 189, 147, 125, 15, 12.0);
        graphics.colour(LIME);
        graphics.text(rate, 194, 163, 116, 22, 19.0);

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
        draw_dj_gator(graphics, active, now_ms);

        // Cyber visor overlay on gator
        let bob = if active {
            [0, -2, -3, -1, 0, 2, 3, 1][((now_ms / 110) % 8) as usize]
        } else {
            0
        };
        let gx = 60;
        let gy = 78 + bob;
        // Visor glow: cyan or electric lime
        graphics.colour(if active { 0xff00ffff } else { 0xff00aacc });
        graphics.rect(gx + 22, gy + 17, 18, 8);
        graphics.colour(0xffffffff);
        graphics.rect(gx + 24, gy + 19, 4, 3);
        graphics.rect(gx + 32, gy + 19, 4, 3);
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
        // Right side information column
        graphics.colour(0xffffaa00); // Warm amber
        graphics.text("MODE: ACORDATOR", 189, 73, 125, 15, 12.0);
        graphics.colour(if hold { LIME } else { 0xffff4053 });
        graphics.text(
            if hold { "HOLD: ON" } else { "HOLD: OFF" },
            189,
            90,
            125,
            15,
            13.0,
        );
        graphics.colour(0xffffdd66); // Warm gold
        graphics.text(
            &format!("{mood_label}  {style_label}"),
            189,
            107,
            125,
            15,
            13.0,
        );
        graphics.colour(0xffffaa00);
        graphics.text("TURN DATA: RATE", 189, 147, 125, 15, 12.0);
        graphics.colour(LIME);
        graphics.text(rate, 194, 163, 116, 22, 19.0);

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
        draw_dj_gator(graphics, active, now_ms);

        // Warm amber sunglasses on gator
        let bob = if active {
            [0, -2, -3, -1, 0, 2, 3, 1][((now_ms / 110) % 8) as usize]
        } else {
            0
        };
        let gx = 60;
        let gy = 78 + bob;
        graphics.colour(0xffffaa00);
        graphics.rect(gx + 22, gy + 17, 18, 8);
        graphics.colour(0xfffff0d5);
        graphics.rect(gx + 24, gy + 19, 4, 3);
        graphics.rect(gx + 32, gy + 19, 4, 3);
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
        // Right side information column
        graphics.colour(0xff9d4edd); // Cosmic violet
        graphics.text("MODE: ORBITATOR", 189, 73, 125, 15, 12.0);
        graphics.colour(if hold { LIME } else { 0xffff4053 });
        graphics.text(
            if hold { "HOLD: ON" } else { "HOLD: OFF" },
            189,
            90,
            125,
            15,
            13.0,
        );
        graphics.colour(0xff00ffff); // Electric Cyan
        graphics.text(
            &format!("{atmo_label}  {dyn_label}"),
            189,
            107,
            125,
            15,
            13.0,
        );
        graphics.colour(0xff9d4edd);
        graphics.text("TURN DATA: RATE", 189, 147, 125, 15, 12.0);
        graphics.colour(LIME);
        graphics.text(rate, 194, 163, 116, 22, 19.0);

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

        // Astronaut Gator
        draw_dj_booth(graphics, active, now_ms);
        draw_dj_gator(graphics, active, now_ms);

        // Space Helmet bubble on Gator!
        let bob = if active {
            [0, -2, -3, -1, 0, 2, 3, 1][((now_ms / 110) % 8) as usize]
        } else {
            0
        };
        let gx = 60;
        let gy = 78 + bob;
        // Translucent blue-tinted visor dome outline
        graphics.colour(0xff00e5ff);
        graphics.rect(gx + 18, gy + 12, 26, 1);
        graphics.rect(gx + 16, gy + 13, 2, 16);
        graphics.rect(gx + 44, gy + 13, 2, 16);
        graphics.rect(gx + 18, gy + 29, 26, 1);
        // Visor gold solar reflection
        graphics.colour(0xffffcf36);
        graphics.rect(gx + 22, gy + 16, 18, 9);
        graphics.colour(0xffffffff);
        graphics.rect(gx + 24, gy + 17, 4, 3);
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
