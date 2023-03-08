use raylib::prelude::*;
const SPACESHIP_LENGTH: f32 = 10.0;
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const BULLET_SPEED:f32 = 5.0;
//const BULLET_DRAG: f32 = 0.05;
const BULLET_MAX_TRAVEL_TIME: f32 = 10.0;

#[derive(Clone, Copy)]
struct Bullet{
    velocity: Vector2,
    position: Vector2,
    time_traveled: f32
}

impl Bullet{
    fn draw(&self, d: &mut RaylibDrawHandle){
        d.draw_circle_v(self.position, 3.0, Color::RED)
    }

    fn update(&mut self) -> bool{
        self.position += self.velocity;
        self.time_traveled += 1.0;

        if self.time_traveled >= BULLET_MAX_TRAVEL_TIME{
            return true;
        }
        else{
            return false;
        }
        
    }
}

struct SpaceShip{
    pos: Vector2,
    angle: f32,
    angle_velocity: f32,
    max_angle_velocity: f32,
    velocity: Vector2,
    acceleration: f32,
    angle_acceleration: f32
}

impl SpaceShip{

    fn new() -> SpaceShip{
        SpaceShip{pos: Vector2 { x: {WINDOW_WIDTH/2} as f32, y: {WINDOW_HEIGHT/2} as f32 },
         angle: 0.0, angle_velocity: 0.0, velocity: Vector2::new(0.0, 0.0), acceleration: 0.04,
        max_angle_velocity: 0.13, angle_acceleration: 0.004}
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

        // Defining positions on triangle
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

        // Apply angle drag 
        if self.angle_velocity > 0.0{
            self.angle_velocity -= self.angle_acceleration/2.0;
        }
        if self.angle_velocity < 0.0{
            self.angle_velocity += self.angle_acceleration/2.0;
        }

        // Apply angle velocity
        self.angle += self.angle_velocity;

        // Check if player is using boost acceleration
        let mut boost = 1.0;
        if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT){
            boost = 2.5;
            println!("{}", boost);
        }


        // Get velocity from player input
        if rl.is_key_down(KeyboardKey::KEY_W) && !rl.is_key_down(KeyboardKey::KEY_S){
            self.velocity.x += self.angle.cos() * self.acceleration * boost;
            self.velocity.y += self.angle.sin() * self.acceleration * boost;
        }
        else if rl.is_key_down(KeyboardKey::KEY_S) && !rl.is_key_down(KeyboardKey::KEY_W){
            self.velocity.x -= self.angle.cos() * self.acceleration * boost;
            self.velocity.y -= self.angle.sin() * self.acceleration * boost;
        }



        // Apply velocity
        self.pos += self.velocity;


        if rl.is_key_down(KeyboardKey::KEY_SPACE){
            let vector = Vector2::new(self.angle.cos()* BULLET_SPEED, self.angle.sin() * BULLET_SPEED);
            let vel = self.velocity + vector;
            bullets.push(Bullet{position: self.pos, velocity: vel, time_traveled: 0.0});
        }

        // Apply angle veclocity
        if rl.is_key_down(KeyboardKey::KEY_D) && {self.angle_velocity + self.angle_acceleration <= self.max_angle_velocity}{
            self.angle_velocity += self.angle_acceleration;
        }
        if rl.is_key_down(KeyboardKey::KEY_A)&& {self.angle_velocity - self.angle_acceleration >= -self.max_angle_velocity}{
            self.angle_velocity += -self.angle_acceleration;
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

    let mut bullets: Vec<Bullet> = vec!();
    
    

    while !rl.window_should_close() {
        // Update

        my_spaceship.update(&mut rl, &mut bullets);

        let mut bullet_count = 0;        // TODO: Implement bullets being removed
        for bullet in &mut bullets{
            bullet_count += 1;
            if bullet.update(){
                // remove bullet
            }

        }
        
    
        let fps = rl.get_fps();

        // Draw

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        my_spaceship.draw(&mut d);
        for bullet in &mut bullets{
            bullet.draw(&mut d);

        }

        let fps = String::from("FPS: ") + &fps.to_string();
        let bullet_count = String::from("Bullet count: ") + &bullet_count.to_string();

        d.draw_text(&fps,0,0,20, Color::WHITE);
        d.draw_text(&bullet_count,0,20,20, Color::WHITE);


    }
}