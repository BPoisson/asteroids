use ggez::glam::Vec2;
use ggez::{Context};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, PxScale, Text, TextLayout};
use crate::constants::SCREEN_SIZE;
use crate::save;

fn get_pause_text(score: &u64) -> Text {
    let high_score: u64 = save::get_high_score();
    let pause_string: String = format!("Game Paused!\n\nYour Score: {}\n\nHigh Score: {}\n\nPress Q To Quit", score, high_score);
    let mut pause_text: Text = Text::new(pause_string);
    pause_text.set_scale(PxScale::from(50.0));
    pause_text.set_layout(TextLayout::center());

    return pause_text;
}

fn get_game_over_text(score: &u64) -> Text {
    let high_score: u64 = save::get_high_score();
    let pause_string: String = format!("Game Over!\n\nYour Score: {}\n\nHigh Score: {}\n\nPress R To Restart\n\nPress Q To Quit", score, high_score);
    let mut pause_text: Text = Text::new(pause_string);
    pause_text.set_scale(PxScale::from(50.0));
    pause_text.set_layout(TextLayout::center());

    return pause_text;
}

fn draw_text(canvas: &mut Canvas, text: Text) -> () {
    canvas.draw(
        &text,
        DrawParam::default()
            .dest(Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y / 2.0))
    );
}

pub fn draw(ctx: &Context, canvas: &mut Canvas, paused: &bool, game_over: &bool, ship_health: &i32, score: &u64) -> () {
    if *paused {
        let pause_text: Text = get_pause_text(&score);
        draw_text(canvas, pause_text);
    }

    if *game_over {
        let game_over_text: Text = get_game_over_text(&score);
        draw_text(canvas, game_over_text);
    }

    let mut position: Vec2 = Vec2::new(SCREEN_SIZE.x - 135.0, 25.0);

    for _ in 0..*ship_health {
        let health_triangle_points: [Vec2; 3] = get_health_triangle_points(&position);
        let health_triangle_mesh: Mesh = create_health_triangle(ctx, &health_triangle_points);

        canvas.draw(
            &health_triangle_mesh,
            DrawParam::default()
        );

        position = Vec2::new(position.x + 30.0, position.y);
    }
}

fn get_health_triangle_points(position: &Vec2) -> [Vec2; 3] {
    return [
        *position + Vec2::new(0.0, -25.0),
        *position + Vec2::new(-10.0, 0.0),
        *position + Vec2::new(10.0, 0.0),
    ];
}

fn create_health_triangle(ctx: &Context, health_triangle_points: &[Vec2; 3]) -> Mesh {
    return Mesh::new_polygon(
        ctx,
        DrawMode::stroke(2.0),
        health_triangle_points,
        Color::WHITE
    ).unwrap();
}