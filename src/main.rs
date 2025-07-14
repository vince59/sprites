use image::{ImageBuffer, Rgba};
use macroquad::prelude::*;

const SIZE: u32 = 64;
const FRAME_COUNT: u32 = 4;

#[macroquad::main("Sprite Sheet Walk Animation")]
async fn main() {
    // --- Génération de la sprite sheet (à commenter si déjà générée) ---
    generate_sprite_sheet();

    // --- Chargement de la sprite sheet avec macroquad ---
    let texture = load_texture("sprite_sheet_walk.png")
        .await
        .expect("Erreur chargement sprite sheet");
    texture.set_filter(FilterMode::Nearest); // Pour un rendu pixel art net

    // Variables animation
    let mut frame: usize = 0;
    let frame_time = 0.2; // Durée d'une frame en secondes
    let mut timer = 0.0;

    loop {
        clear_background(WHITE);

        // Mise à jour timer
        timer += get_frame_time();
        if timer >= frame_time {
            timer = 0.0;
            frame = (frame + 1) % FRAME_COUNT as usize;
        }

        // Rectangle source pour la frame courante
        let src_rect = Rect::new(
            frame as f32 * SIZE as f32,
            0.0,
            SIZE as f32,
            SIZE as f32,
        );

        // Dessiner la frame au centre, échelle 4x
        draw_texture_ex(
            texture,
            screen_width() / 2.0 - (SIZE as f32 * 2.0) / 2.0,
            screen_height() / 2.0 - (SIZE as f32 * 2.0) / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(SIZE as f32 * 2.0, SIZE as f32 * 2.0)),
                source: Some(src_rect),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

fn generate_sprite_sheet() {
    let mut sheet = ImageBuffer::from_pixel(SIZE * FRAME_COUNT, SIZE, Rgba([0, 0, 0, 0]));

    let skin = Rgba([220, 180, 140, 255]);
    let hair = Rgba([60, 30, 20, 255]);
    let shirt = Rgba([50, 120, 200, 255]);
    let pants = Rgba([40, 40, 40, 255]);
    let shoes_top = Rgba([20, 20, 20, 255]);
    let shoes_bottom = Rgba([10, 10, 10, 255]);
    let eyes = Rgba([255, 255, 255, 255]);
    let pupil = Rgba([0, 0, 0, 255]);
    let mouth = Rgba([150, 0, 0, 255]);
    let nose = Rgba([180, 140, 120, 255]);

    for frame in 0..FRAME_COUNT {
        let mut img = ImageBuffer::from_pixel(SIZE, SIZE, Rgba([0, 0, 0, 0]));

        // Tête
        draw_rect(&mut img, 26, 4, 12, 12, skin);
        for dx in 0..12 {
            img.put_pixel(26 + dx, 3, hair);
            if dx % 2 == 0 {
                img.put_pixel(26 + dx, 4, hair);
            }
        }

        draw_rect(&mut img, 28, 9, 2, 2, eyes);
        draw_rect(&mut img, 34, 9, 2, 2, eyes);
        img.put_pixel(29, 10, pupil);
        img.put_pixel(35, 10, pupil);
        draw_rect(&mut img, 31, 11, 2, 2, nose);
        draw_rect(&mut img, 29, 14, 6, 1, mouth);
        draw_rect(&mut img, 30, 15, 4, 1, mouth);

        // Torse
        draw_rect(&mut img, 24, 16, 16, 18, shirt);

        // Bras avec animation simple (offset pour simuler mouvement)
        let arm_offset = match frame {
            0 => (-2, 2),  // G: arrière, D: avant
            1 => (0, 0),
            2 => (2, -2),  // G: avant, D: arrière
            3 => (0, 0),
            _ => (0, 0),
        };
        draw_rect(&mut img, (18i32 + arm_offset.0) as u32, 16, 6, 18, shirt);
        draw_rect(&mut img, (40i32 + arm_offset.1) as u32, 16, 6, 18, shirt);
        draw_rect(&mut img, (19i32 + arm_offset.0) as u32, 34, 4, 4, skin);
        draw_rect(&mut img, (41i32 + arm_offset.1) as u32, 34, 4, 4, skin);

        // Jambes avec animation simple (offset)
        let leg_offset = match frame {
            0 => (2, -2), // G: avant, D: arrière
            1 => (0, 0),
            2 => (-2, 2), // G: arrière, D: avant
            3 => (0, 0),
            _ => (0, 0),
        };
        draw_rect(&mut img, (26i32 + leg_offset.0) as u32, 34, 6, 20, pants);
        draw_rect(&mut img, (34i32 + leg_offset.1) as u32, 34, 6, 20, pants);

        // Chaussures
        draw_rect(&mut img, (26i32 + leg_offset.0) as u32, 54, 6, 3, shoes_top);
        draw_rect(&mut img, (26i32 + leg_offset.0) as u32, 57, 6, 1, shoes_bottom);
        draw_rect(&mut img, (34i32 + leg_offset.1) as u32, 54, 6, 3, shoes_top);
        draw_rect(&mut img, (34i32 + leg_offset.1) as u32, 57, 6, 1, shoes_bottom);

        // Copier frame dans la sprite sheet
        for y in 0..SIZE {
            for x in 0..SIZE {
                let px = img.get_pixel(x, y);
                sheet.put_pixel(frame * SIZE + x, y, *px);
            }
        }
    }

    sheet.save("sprite_sheet_walk.png").expect("Erreur sauvegarde sprite sheet");
    println!("Sprite sheet générée !");
}

fn draw_rect(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Rgba<u8>,
) {
    for dy in 0..height {
        for dx in 0..width {
            img.put_pixel(x + dx, y + dy, color);
        }
    }
}
