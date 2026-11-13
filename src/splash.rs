//! Small pixel-art startup mark drawn with the MPC SDK's native graphics calls.

use mpc_sample_sdk::draw::Graphics;

const PIXEL: i32 = 7;
const GATOR: [&str; 12] = [
    "....PPPPPPPP....",
    "..PPGGGGGGGGPP..",
    ".PGGGVGGVGGGGGP.",
    "PGGGGGGGGGGGGGGP",
    "PGGYYYYYYYYYGGGP",
    "GGYYYYYYYYYYYGGG",
    "GGYWBYWYBWYYGGGG",
    ".GGYYYYYYYYYGG..",
    "..GGGGGGGGGGGG..",
    "...GYYYYYYYYG...",
    "..GGGGGGGGGGGG..",
    ".GG..GG..GG..GG.",
];

/// Draw the startup card on the live Sample-page graphics context.
///
/// # Safety
/// The caller must invoke this only from the SDK's on_paint callback.
pub unsafe fn draw(graphics: &mut Graphics) {
    // SAFETY: the caller holds the live app Graphics handle on its paint path.
    unsafe {
        graphics.colour(0xff08_0520);
        graphics.rect(24, 20, 272, 200);
        graphics.colour(0xffb52aff);
        graphics.rect(38, 37, 244, 3);
        graphics.rect(38, 200, 244, 3);
        graphics.colour(0xffffd66b);
        graphics.text("ARPEGGIGATOR", 55, 49, 210, 26, 18.0);

        for (row, line) in GATOR.iter().enumerate() {
            for (column, pixel) in line.bytes().enumerate() {
                let colour = match pixel {
                    b'P' => 0xffe83bd8,
                    b'G' => 0xff39d52d,
                    b'V' => 0xff9227e8,
                    b'Y' => 0xffffcf36,
                    b'W' => 0xfffff1d7,
                    b'B' => 0xff160b2f,
                    _ => continue,
                };
                graphics.colour(colour);
                graphics.rect(
                    104 + column as i32 * PIXEL,
                    82 + row as i32 * PIXEL,
                    PIXEL,
                    PIXEL,
                );
            }
        }

        graphics.colour(0xffffd8c5);
        graphics.text("STANDALONE MPC SAMPLE MOD", 55, 181, 210, 14, 8.0);
    }
}
