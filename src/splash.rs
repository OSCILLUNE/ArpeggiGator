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
            let chewing = held_pads > 0 && (now_ms / 130).is_multiple_of(2);
            draw_gator(graphics, chewing);
            if held_pads > 0 {
                let note_x = 182 - ((now_ms / 65) % 76) as i32;
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

unsafe fn draw_gator(graphics: &mut Graphics, chewing: bool) {
    // Compact pixel mascot; it appears only after ARP is enabled.
    let x = 25;
    let y = 103;
    unsafe {
        graphics.colour(INK);
        graphics.rect(x + 3, y + 11, 14, 31);
        graphics.rect(x + 14, y + 3, 49, 12);
        graphics.rect(x + 14, y + 12, 53, 34);
        graphics.rect(x + 35, y + 39, 40, 19);

        graphics.colour(PINK);
        graphics.rect(x + 4, y + 14, 11, 25);
        graphics.rect(x + 18, y + 4, 39, 5);
        graphics.rect(x + 57, y + 8, 7, 5);
        graphics.colour(PURPLE);
        graphics.rect(x + 7, y + 18, 6, 17);

        graphics.colour(GREEN);
        graphics.rect(x + 17, y + 13, 39, 27);
        graphics.rect(x + 42, y + 20, 29, 17);
        graphics.rect(x + 61, y + 25, 21, 12);
        graphics.rect(x + 28, y + 48, 31, 16);
        graphics.rect(x + 21, y + 57, 16, 8);
        graphics.rect(x + 52, y + 57, 14, 8);

        graphics.colour(LIME);
        graphics.rect(x + 20, y + 15, 24, 3);
        graphics.rect(x + 47, y + 23, 20, 3);
        graphics.rect(x + 64, y + 27, 13, 3);

        graphics.colour(INK);
        graphics.rect(x + 22, y + 20, 16, 11);
        graphics.rect(x + 41, y + 20, 16, 11);
        graphics.rect(x + 37, y + 24, 5, 3);
        graphics.rect(x + 42, y + 36, 37, 7);
        graphics.colour(VIOLET);
        graphics.rect(x + 24, y + 22, 11, 7);
        graphics.rect(x + 43, y + 22, 11, 7);
        graphics.colour(CREAM);
        graphics.rect(x + 27, y + 23, 3, 2);
        graphics.rect(x + 46, y + 23, 3, 2);

        graphics.colour(GOLD);
        if chewing {
            graphics.rect(x + 44, y + 45, 32, 9);
        } else {
            graphics.rect(x + 44, y + 42, 35, 8);
        }
        graphics.colour(INK);
        graphics.rect(x + 44, y + 37, 35, 4);
        graphics.colour(CREAM);
        graphics.rect(x + 49, y + 38, 3, 5);
        graphics.rect(x + 60, y + 38, 3, 5);
        graphics.rect(x + 71, y + 38, 3, 5);
        graphics.colour(GREEN);
        graphics.rect(x + 25, y + 61, 12, 8);
        graphics.rect(x + 52, y + 61, 13, 8);
    }
}

unsafe fn draw_note(graphics: &mut Graphics, x: i32, chewing: bool) {
    let y = 141;
    unsafe {
        graphics.colour(PINK);
        graphics.rect(x + 6, y, 3, 13);
        graphics.rect(x + 6, y, 9, 3);
        graphics.colour(GOLD);
        graphics.rect(x, y + 11, 8, 5);
        graphics.rect(x + 11, y + 7, 8, 5);
        if chewing {
            graphics.colour(CREAM);
            graphics.rect(106, y - 9, 3, 3);
        }
    }
}
