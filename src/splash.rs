//! Small ArpeggiGator controls for the 16 Levels SHIFT menu.

use mpc_sample_sdk::draw::Graphics;

const INK: u32 = 0xff16082f;
const BG: u32 = 0xff09051f;
const PURPLE: u32 = 0xff6720b5;
const VIOLET: u32 = 0xffa62be8;
const PINK: u32 = 0xffed36cf;
const GREEN: u32 = 0xff38d42e;
const LIME: u32 = 0xffa6f52a;
const GOLD: u32 = 0xffffcf36;
const CREAM: u32 = 0xfffff0d5;

// Suit and splash palette
const SUIT_DARK: u32 = 0xff130724;
const SUIT_BODY: u32 = 0xff230d3d;
const SUIT_LAPEL: u32 = 0xff381661;
const SHIRT_WHITE: u32 = 0xfffff5eb;
const SHIRT_SHADOW: u32 = 0xffd2c4de;
const TIE_RED: u32 = 0xffe6193c;
const TIE_DARK: u32 = 0xff990f28;
const TIE_LIGHT: u32 = 0xffff5975;
const SHOE_DARK: u32 = 0xff0f051c;
const SHOE_LIGHT: u32 = 0xff2b1442;
const SHADOW: u32 = 0xff075c39;
const MID: u32 = 0xff18a852;
const HIGHLIGHT: u32 = 0xffa8ed67;

/// Paint custom B1/B2/B3 captions while SHIFT is held and a small mascot only
/// while the arpeggiator is enabled.
///
/// # Safety
/// Call only from the SDK's live `on_paint` callback.
pub unsafe fn draw_arp_overlay(
    graphics: &mut Graphics,
    show_shift_menu: bool,
    enabled: bool,
    pattern: &str,
    additional_notes: u8,
    preview: (u8, bool, &str),
    now_ms: u64,
) {
    let (held_pads, hold, rate) = preview;
    // SAFETY: the caller owns the live Graphics handle for this paint.
    unsafe {
        if show_shift_menu {
            draw_shift_labels(graphics, enabled, pattern, additional_notes);
        }
        if enabled {
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
}

/// Draw the startup scene on the live app paint path.
///
/// # Safety
/// The graphics handle must belong to the active paint callback.
pub unsafe fn draw_boot_splash(graphics: &mut Graphics) {
    unsafe {
        // Sunset sky and a stepped, striped vaporwave sun.
        graphics.colour(0xff1c0737);
        graphics.rect(0, 0, 320, 240);
        graphics.colour(0xff370c5c);
        graphics.rect(0, 43, 320, 95);
        graphics.colour(0xffa52e89);
        graphics.rect(118, 50, 84, 8);
        graphics.rect(106, 58, 108, 12);
        graphics.colour(0xffff8951);
        graphics.rect(100, 70, 120, 16);
        graphics.colour(0xffffa94d);
        graphics.rect(98, 86, 124, 18);
        graphics.colour(0xfff36367);
        graphics.rect(101, 104, 118, 12);
        graphics.colour(0xffdc468e);
        graphics.rect(108, 118, 104, 7);
        graphics.rect(120, 129, 80, 5);
        // Perspective grid on the lower horizon.
        graphics.colour(0xff210734);
        graphics.rect(0, 145, 320, 95);
        graphics.colour(0xff6a258e);
        for y in [150, 158, 168, 182, 201, 226] {
            graphics.rect(0, y, 320, 1);
        }
        for row in 0..19 {
            let y = 145 + row * 5;
            for slope in [-6, -4, -2, 0, 2, 4, 6] {
                let x = 160 + slope * row;
                if (0..320).contains(&x) {
                    graphics.rect(x, y, 2, 5);
                }
            }
        }
        graphics.colour(INK);
        graphics.text("ArpeggiGator", 27, 10, 268, 30, 23.0);
        graphics.colour(0xffff99ed);
        graphics.text("ArpeggiGator", 25, 8, 268, 30, 23.0);
        draw_splash_suited_gator(graphics);

        // Bottom-right attribution
        graphics.colour(INK);
        graphics.text("OSCILLUNE", 214, 216, 100, 20, 14.0);
        graphics.colour(GOLD);
        graphics.text("OSCILLUNE", 212, 214, 100, 20, 14.0);
    }
}

/// Gator in an 80s synthwave tuxedo giving a thumbs-up on the boot screen.
unsafe fn draw_splash_suited_gator(graphics: &mut Graphics) {
    let x = 55;
    let y = 103;
    unsafe {
        // Tail silhouette and scales on the left
        graphics.colour(INK);
        scene_rect(graphics, x + 3, y + 37, 15, 6);
        scene_rect(graphics, x + 10, y + 31, 15, 16);
        graphics.colour(SHADOW);
        scene_rect(graphics, x + 13, y + 35, 15, 8);
        graphics.colour(MID);
        scene_rect(graphics, x + 20, y + 30, 15, 10);

        // Head, snout, sunglasses and headphones
        graphics.colour(INK);
        scene_rect(graphics, x + 19, y + 23, 34, 16);
        scene_rect(graphics, x + 26, y + 16, 24, 10);
        scene_rect(graphics, x + 43, y + 24, 27, 20);
        scene_rect(graphics, x + 61, y + 30, 23, 16);

        graphics.colour(SHADOW);
        scene_rect(graphics, x + 21, y + 27, 29, 12);
        scene_rect(graphics, x + 47, y + 29, 32, 14);

        graphics.colour(MID);
        scene_rect(graphics, x + 20, y + 30, 30, 10);
        scene_rect(graphics, x + 28, y + 20, 19, 10);
        scene_rect(graphics, x + 46, y + 27, 21, 13);
        scene_rect(graphics, x + 62, y + 33, 19, 7);

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

        // Headphones (dark outline + red)
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

        // Sunglasses with glint
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

        // Smile & sharp teeth
        scene_rect(graphics, x + 54, y + 40, 3, 3);
        scene_rect(graphics, x + 65, y + 40, 3, 3);
        scene_rect(graphics, x + 75, y + 40, 3, 3);
        graphics.colour(MID);
        scene_rect(graphics, x + 50, y + 43, 29, 3);

        // --- Suit, Shirt, Tie, Legs and Shoes ---

        // Main body of the suit jacket
        graphics.colour(SUIT_DARK);
        scene_rect(graphics, x + 15, y + 46, 52, 28);
        graphics.colour(SUIT_BODY);
        scene_rect(graphics, x + 17, y + 48, 48, 24);

        // Lapels (V shape cut)
        graphics.colour(SUIT_LAPEL);
        scene_rect(graphics, x + 24, y + 47, 8, 22);
        scene_rect(graphics, x + 46, y + 47, 8, 22);

        // White dress shirt in the V-neck
        graphics.colour(SHIRT_SHADOW);
        scene_rect(graphics, x + 32, y + 46, 14, 14);
        graphics.colour(SHIRT_WHITE);
        scene_rect(graphics, x + 33, y + 46, 12, 12);

        // Red Synthwave Tie
        // Tie knot
        graphics.colour(TIE_DARK);
        scene_rect(graphics, x + 37, y + 46, 6, 4);
        graphics.colour(TIE_RED);
        scene_rect(graphics, x + 38, y + 46, 4, 3);

        // Tie body descending
        graphics.colour(TIE_DARK);
        scene_rect(graphics, x + 36, y + 50, 7, 18);
        graphics.colour(TIE_RED);
        scene_rect(graphics, x + 37, y + 50, 5, 17);
        graphics.colour(TIE_LIGHT);
        scene_rect(graphics, x + 38, y + 51, 2, 14);

        // Left arm (resting along the side / pocket)
        graphics.colour(SUIT_DARK);
        scene_rect(graphics, x + 10, y + 48, 8, 20);
        graphics.colour(SUIT_BODY);
        scene_rect(graphics, x + 11, y + 49, 6, 18);
        graphics.colour(SHADOW);
        scene_rect(graphics, x + 13, y + 66, 6, 4);
        graphics.colour(GREEN);
        scene_rect(graphics, x + 14, y + 66, 4, 3);

        // Right arm giving THUMBS-UP! 👍
        // Sleeve reaching forward
        graphics.colour(SUIT_DARK);
        scene_rect(graphics, x + 54, y + 50, 18, 10);
        graphics.colour(SUIT_BODY);
        scene_rect(graphics, x + 55, y + 51, 16, 8);
        // White shirt cuff
        graphics.colour(SHIRT_WHITE);
        scene_rect(graphics, x + 69, y + 51, 3, 7);

        // Hand fist & thumb pointing up
        graphics.colour(INK);
        scene_rect(graphics, x + 72, y + 50, 9, 9);
        scene_rect(graphics, x + 74, y + 43, 6, 9);

        graphics.colour(SHADOW);
        scene_rect(graphics, x + 73, y + 51, 7, 7);
        scene_rect(graphics, x + 75, y + 44, 4, 7);

        graphics.colour(MID);
        scene_rect(graphics, x + 74, y + 52, 5, 5);
        scene_rect(graphics, x + 75, y + 45, 3, 6);

        graphics.colour(HIGHLIGHT);
        scene_rect(graphics, x + 75, y + 44, 3, 2);
        scene_rect(graphics, x + 74, y + 52, 4, 2);

        // Claws on curled fingers and thumb
        graphics.colour(CREAM);
        scene_rect(graphics, x + 78, y + 51, 2, 2);
        scene_rect(graphics, x + 78, y + 54, 2, 2);
        scene_rect(graphics, x + 76, y + 43, 2, 2);

        // Pocket square (neon pink) on chest pocket
        graphics.colour(PINK);
        scene_rect(graphics, x + 26, y + 53, 5, 2);
        graphics.colour(0xffff7ae8);
        scene_rect(graphics, x + 27, y + 52, 3, 2);

        // Trousers / Legs
        graphics.colour(SUIT_DARK);
        scene_rect(graphics, x + 22, y + 74, 14, 12);
        graphics.colour(SUIT_BODY);
        scene_rect(graphics, x + 23, y + 74, 11, 11);

        graphics.colour(SUIT_DARK);
        scene_rect(graphics, x + 42, y + 74, 14, 12);
        graphics.colour(SUIT_BODY);
        scene_rect(graphics, x + 43, y + 74, 11, 11);

        // Shiny dress shoes
        graphics.colour(SHOE_DARK);
        scene_rect(graphics, x + 19, y + 85, 17, 5);
        scene_rect(graphics, x + 42, y + 85, 17, 5);
        graphics.colour(SHOE_LIGHT);
        scene_rect(graphics, x + 20, y + 86, 14, 2);
        scene_rect(graphics, x + 43, y + 86, 14, 2);
        graphics.colour(CREAM);
        scene_rect(graphics, x + 22, y + 86, 3, 1);
        scene_rect(graphics, x + 45, y + 86, 3, 1);
    }
}

unsafe fn draw_shift_labels(
    graphics: &mut Graphics,
    enabled: bool,
    pattern: &str,
    additional_notes: u8,
) {
    // These three cells cover the MPC's B1/B2/B3 captions, leaving the rest
    // of the Sample or 16 Levels screen untouched.
    unsafe {
        label_cell(graphics, 13, 5, 88, if enabled { LIME } else { BG });
        label_cell(graphics, 111, 5, 98, BG);
        label_cell(graphics, 219, 5, 88, BG);

        graphics.colour(if enabled { INK } else { PINK });
        graphics.text("ARP", 17, 8, 80, 19, 15.0);
        graphics.colour(CREAM);
        graphics.text(pattern, 115, 8, 90, 19, 13.0);
        graphics.colour(GOLD);
        graphics.text(&format!("+{additional_notes}"), 223, 8, 80, 19, 15.0);
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
