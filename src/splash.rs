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
    held_pads: u8,
    now_ms: u64,
) {
    // SAFETY: the caller owns the live Graphics handle for this paint.
    unsafe {
        if show_shift_menu {
            draw_shift_labels(graphics, enabled, pattern, additional_notes);
        }
        if enabled {
            let phase = if held_pads > 0 { (now_ms / 150) % 8 } else { 0 };
            let chewing = held_pads > 0 && (3..=5).contains(&phase);
            draw_gator(graphics, chewing, phase, held_pads > 0);
            if held_pads > 0 {
                let note_x = 264 - ((now_ms / 65) % 76) as i32;
                draw_note(graphics, note_x, chewing);
            }
        }
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
        graphics.text("ARP", 17, 12, 80, 12, 9.0);
        graphics.colour(CREAM);
        graphics.text(pattern, 115, 12, 90, 12, 8.0);
        graphics.colour(GOLD);
        graphics.text(&format!("+{additional_notes}"), 223, 12, 80, 12, 9.0);
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

unsafe fn draw_gator(graphics: &mut Graphics, chewing: bool, phase: u64, moving: bool) {
    // The water is a small irregular silhouette, not a panel over the waveform.
    // Idle is always phase zero; animation moves only while a pad is held.
    let x = 25;
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
        // Layered ripples leave the native screen visible around their edges.
        graphics.colour(DEEP);
        scene_rect(graphics, x + 3, 161, 83, 4);
        scene_rect(graphics, x + 10, 165, 70, 4);
        scene_rect(graphics, x + 20, 169, 49, 3);
        graphics.colour(WATER);
        scene_rect(graphics, x + 5 + wave, 159, 32, 3);
        scene_rect(graphics, x + 54 - wave, 160, 32, 3);
        scene_rect(graphics, x + 11, 165, 63, 2);
        graphics.colour(FOAM);
        scene_rect(graphics, x + 3 + wave, 158, 15, 2);
        scene_rect(graphics, x + 69 - wave, 159, 16, 2);
        scene_rect(graphics, x + 20 - wave, 168, 17, 1);
        scene_rect(graphics, x + 57 + wave, 167, 15, 1);

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
    let y = 120;
    unsafe {
        graphics.colour(PINK);
        graphics.rect(x + 6, y, 3, 13);
        graphics.rect(x + 6, y, 9, 3);
        graphics.colour(GOLD);
        graphics.rect(x, y + 11, 8, 5);
        graphics.rect(x + 11, y + 7, 8, 5);
        if chewing {
            graphics.colour(CREAM);
            graphics.rect(187, y - 9, 3, 3);
        }
    }
}
