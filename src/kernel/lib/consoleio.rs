const CHAR_WIDTH: u64 = 15;
const CHAR_HEIGHT: u64 = 23;
const CHAR_SCALE: u64 = 3;

static mut CURSOR_X: u64 = 0;
static mut CURSOR_Y: u64 = 0;

pub fn print(msg: &[u8]) {
    let Some(framebuffer) = crate::limine::framebuffer() else {
        return;
    };
    if framebuffer.address.is_null()
        || framebuffer.pitch == 0
        || framebuffer.width < CHAR_WIDTH
        || framebuffer.height < CHAR_HEIGHT
        || framebuffer.bpp < 24
    {
        return;
    }

    let foreground = color(framebuffer, 0xff, 0xff, 0xff);
    let background = color(framebuffer, 0, 0, 0);

    for &byte in msg {
        unsafe {
            if byte == b'\n' {
                CURSOR_X = 0;
                CURSOR_Y += CHAR_HEIGHT;
                continue;
            }

            if CURSOR_X + CHAR_WIDTH > framebuffer.width {
                CURSOR_X = 0;
                CURSOR_Y += CHAR_HEIGHT * 2;
            }
            if CURSOR_Y + CHAR_HEIGHT > framebuffer.height {
                CURSOR_X = 0;
                CURSOR_Y = 0;
            }

            draw_character(framebuffer, byte, foreground, background);
            CURSOR_X += CHAR_WIDTH * 2;
        }
    }
}

fn color(framebuffer: &crate::limine::Framebuffer, red: u8, green: u8, blue: u8) -> u32 {
    channel(red, framebuffer.red_mask_size, framebuffer.red_mask_shift)
        | channel(
            green,
            framebuffer.green_mask_size,
            framebuffer.green_mask_shift,
        )
        | channel(
            blue,
            framebuffer.blue_mask_size,
            framebuffer.blue_mask_shift,
        )
}

fn channel(value: u8, size: u8, shift: u8) -> u32 {
    if size == 0 {
        return 0;
    }
    let max = (1u32 << size) - 1;
    ((value as u32 * max / 255) & max) << shift
}

unsafe fn draw_character(
    framebuffer: &crate::limine::Framebuffer,
    character: u8,
    foreground: u32,
    background: u32,
) {
    let glyph = glyph(character);

    for (row, bits) in glyph.iter().enumerate() {
        // Käydään fontin 7 riviä läpi
        for sy in 0..CHAR_SCALE {
            let pixel_y = CURSOR_Y + (row as u64 * CHAR_SCALE) + sy;
            if pixel_y >= framebuffer.height { continue; }

            // Käydään fontin 5 saraketta läpi
            for column in 0..5 {
                let is_foreground = (bits & (1 << (4 - column))) != 0;
                let color_val = if is_foreground { foreground } else { background };

                for sx in 0..CHAR_SCALE {
                    let pixel_x = CURSOR_X + (column * CHAR_SCALE) + sx;
                    if pixel_x >= framebuffer.width { continue; }

                    // Nopea ja suora suorituskykyinen muistiosoitteen laskenta
                    let offset = pixel_y * framebuffer.pitch + pixel_x * (framebuffer.bpp as u64 / 8);
                    let target_address = framebuffer.address.add(offset as usize);

                    // Kirjoitetaan suoraan 32-bittisenä arvona jos mahdollista, muuten tavuina
                    if framebuffer.bpp == 32 {
                        *(target_address as *mut u32) = color_val;
                    } else {
                        let bytes = color_val.to_le_bytes();
                        for index in 0..(framebuffer.bpp as usize / 8) {
                            target_address.add(index).write_volatile(bytes[index]);
                        }
                    }
                }
            }
        }
    }
}

unsafe fn write_pixel(framebuffer: &crate::limine::Framebuffer, x: u64, y: u64, pixel: u32) {
    let offset = y * framebuffer.pitch + x * (framebuffer.bpp as u64 / 8);
    let address = unsafe { framebuffer.address.add(offset as usize) };
    let bytes = pixel.to_le_bytes();
    for index in 0..(framebuffer.bpp as usize / 8) {
        unsafe {
            address.add(index).write_volatile(bytes[index]);
        }
    }
}

fn glyph(character: u8) -> [u8; 7] {
    match character.to_ascii_uppercase() {
        b'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        b'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        b'C' => [
            0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111,
        ],
        b'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        b'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        b'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        b'G' => [
            0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
        ],
        b'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        b'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        b'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
        ],
        b'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        b'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        b'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        b'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        b'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        b'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        b'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        b'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        b'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        b'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        b'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        b'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        b'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        b'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        b'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        b'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        b'0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        b'1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        b'2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        b'3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        b'4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        b'5' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110,
        ],
        b'6' => [
            0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        b'7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        b'8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        b'9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        b' ' => [0; 7],
        b'.' => [0, 0, 0, 0, 0, 0b00110, 0b00110],
        b'!' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0, 0b00100],
        b'?' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0, 0b00100],
        _ => [
            0b11111, 0b10001, 0b00100, 0b00100, 0b00100, 0b10001, 0b11111,
        ],
    }
}
