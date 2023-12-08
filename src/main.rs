use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra;

use ggez::{
    Context, GameResult
};
use gezz::input::keyboard::{
    self, KeyCode
};

use rand{
    self, thread_rng, Rng
};


const PADDING: f32 = 40.0;
const MIDDLE_LINE_WIDTH: f32 = 2.0
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 500.0;


fn clamp(
    value: &mut f32, 
    low: f32, high: f32
)
{
    if (*value < low)
    {
        *value = low;
    } else if (*value > high)
    {
        *value = high;
    }
}


fn move_racket(
    pos: &mut nalgebra::Point2<f32>,
    key_code: KeyCode, y_dir: f32,
    ctx: &mut Context
{
    let delta_time = ggez::timer::delta(ctx).as_secs_f32();
    let screen_height = graphics::drawable_size(ctx).1;

    if (keyboard::is_key_pressed(ctx, key_code))
    {
        pos.y += y_dir * PLAYER_SPEED * delta_time;
    }

    clamp(
        &mut pos.y,
        RACKET_HEIGHT_HALF,
        screen_height - RACEKT_HEIGHT_HALF,
    );
}


fn randomize_vector(
    vec: &mut nalgebra::Vector2<f32>,
    x: f32, y: f32
)
{
    let mut rng = thread_rng();

    vec.x = match rng.gen_bool(0.5)
    {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5)
    {
        true => y,
        false => -y,
    };
}


struct MainState
{
    player_1_position: nalgebra::Point2<f32>,
    player_2_position: nalgebra::Point2<f32>,
    ball_position: nalgebra::Point2<f32>,
    ball_velocity: nalgebra::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}


impl MainState
{
    pub fn new(
        ctx: &mut Context
    ) -> Self
    {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);
        let (screen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);

        let mut ball_velocity = nalgebra::Vector2::new(0.0, 0.0);
        
        randomize_vector(
            &mut ball_velocity,
            BALL_SPEED, BALL_SPEED
        );
    }


    MainState
    {
        player_1_position: nalgebra::Point2::new(RACKET_WIDTH_HALF + PADDING, screen_height_half),
        player_2_position: nalgebra::Point2::new(screen_width - RACKET_WIDTH_HALF - PADDING, screen_height_half),
        ball_position: nalgebra::Point2::new(screen_width_half, screen_height_half),
        ball_velocity,
        player_1_score: 0,
        player_2_score: 0,
    }
}


impl event::EventHandler for MainState
{
    fn update(
        &mut self,
        &mut ctx: Context
    ) -> GameResult
    {
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();

        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        move_racket(
            &mut self.player_1_position, 
            KeyCode::W, -1.0, ctx
        );
        move_racket(
            &mut self.player_1_position, 
            KeyCode::S, 1.0, ctx
        );

        move_racket(
            &mut self.player_2_position, 
            KeyCode::Up, -1.0, ctx
        );
        move_racket(
            &mut self.player_2_position, 
            KeyCode::Down, 1.0, ctx
        );

        self.ball_position += self.ball_velocity * delta_time;

        if (self.ball_position.x < 0.0)
        {
            self.ball_position.x = screen_width * 0.5;
            self.ball_position.y = screen_height * 0.5;

            randomize_vector(
                &mut self.ball_position,
                BALL_SPEED, BALL_SPEED
            );

            self.player_2_score += 1;
        }
    }
}














fn main() {
    println!("Hello, world!");
}
