use raylib::prelude::*;
const SPACESHIP_LENGTH: f32 = 10.0;
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const BULLET_SPEED:f32 = 5.0;

struct Particle{
    velocity: Vector2,
    position: Vector2
}

struct Bullet{
    velocity: Vector2,
    position: Vector2
}

impl Bullet{
    fn draw(&self, d: &mut RaylibDrawHandle){
        d.draw_circle_v(self.position, 3.0, Color::RED)
    }

    fn update(&mut self){
        self.position += self.velocity;
    }
}

struct SpaceShip{
    pos: Vector2,
    angle: f32,
    angle_velocity: f32,
    max_angle_velocity: f32,
    max_velocity: Vector2,
    velocity: Vector2,
    acceleration: f32,
    angle_acceleration: f32
}

impl SpaceShip{

    fn new() -> SpaceShip{
        SpaceShip{pos: Vector2 { x: {WINDOW_WIDTH/2} as f32, y: {WINDOW_HEIGHT/2} as f32 },
         angle: 0.0, angle_velocity: 0.0, velocity: Vector2::new(0.0, 0.0), acceleration: 0.05,
        max_angle_velocity: 0.1, max_velocity: Vector2::new(1.0, 1.0), angle_acceleration: 0.001}
    }

    fn draw(&self, d: &mut RaylibDrawHandle){

        let length_vector = Vector2::new(SPACESHIP_LENGTH, SPACESHIP_LENGTH);

        // get cords of triangle
        let transform_angle ={ 2.0 * PI}/3.0;
        let angle:f32 = self.angle;
        let top = Vector2::new(angle.cos(), angle.sin());
        let angle:f32 = angle + transform_angle as f32;
        let left = Vector2::new(angle.cos(), angle.sin());
        let angle:f32 = angle + transform_angle as f32;
        let right = Vector2::new(angle.cos(), angle.sin());

        // TODO: REMOVE

        let top = self.pos+top*length_vector;
        let left = self.pos+left*length_vector;
        let right = self.pos+right*length_vector;
        let center = self.pos;
        
        // drawing spaceship
        d.draw_line_v(top, left,Color::WHITE);
        d.draw_line_v(top, right, Color::WHITE);
        d.draw_line_v(right, center, Color::WHITE);
        d.draw_line_v(left, center, Color::WHITE);
        
    }

    fn update(&mut self, rl: &mut RaylibHandle, bullets: &mut Vec<Bullet>){


        // Add angle velocity
        if !{self.angle_velocity.abs() + self.angle_acceleration >= self.max_angle_velocity}{

            if rl.is_key_down(KeyboardKey::KEY_A){
                self.angle_velocity += -self.angle_acceleration;
            }
            if rl.is_key_down(KeyboardKey::KEY_D){
                self.angle_velocity += self.angle_acceleration;
            }
        }

        // Apply angle velocity
        self.angle += self.angle_velocity;

        // Get velocity
        if rl.is_key_down(KeyboardKey::KEY_W) && !rl.is_key_down(KeyboardKey::KEY_S){
            self.velocity.x += self.angle.cos() * self.acceleration;
            self.velocity.y += self.angle.sin() * self.acceleration;
        }
        else if rl.is_key_down(KeyboardKey::KEY_S) && !rl.is_key_down(KeyboardKey::KEY_W){
            self.velocity.x -= self.angle.cos() * self.acceleration;
            self.velocity.y -= self.angle.sin() * self.acceleration;
        }

        // Apply velocity
        self.pos += self.velocity;


        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON){
            let vector = Vector2::new(self.angle.cos()* BULLET_SPEED, self.angle.sin() * BULLET_SPEED);
            let vel = self.velocity + vector;
            bullets.push(Bullet{position: self.pos, velocity: vel});
        }



    }
    

}


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Pixels")
        .build();
    rl.set_target_fps(60);

    let mut my_spaceship = SpaceShip::new();
    let mut particles:Vec<Particle> = vec!();
    let mut bullets: Vec<Bullet> = vec!();
    let mut my_bullet = Bullet{position: Vector2 { x: 0.0, y: 0.0 }, velocity: Vector2 { x: 0.1, y: 0.1 }};

    

    while !rl.window_should_close() {
        
        my_spaceship.update(&mut rl, &mut bullets);

        for bullet in &mut bullets{
            bullet.update();
        }
    
        my_bullet.update();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(0, 0);

        
        my_spaceship.draw(&mut d);
        for bullet in &mut bullets{
            bullet.draw(&mut d);
        }

        my_bullet.draw(&mut d);

    }
}