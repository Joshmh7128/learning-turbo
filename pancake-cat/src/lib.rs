// Define the game's configuration using the turbo::cfg! macro
turbo::cfg! {r#"
    name = "Pancake Cat Mod"
    version = "1.0.0"
    author = "Josh"
    description = "Catch falling pancakes - with Josh's mod"
    [settings]
    resolution = [256,144]
"#}

// Here's where the initialization of the game happens
// we can do game state initialization
turbo::init! {
    struct GameState {
        frame: u32, // unassigned integer 
        last_munch_at: u32,
        cat_x: f32, // unassigned float 
        cat_y: f32,
        cat_r: f32,
        // a growable array of pancakes, defined with the type paramter of Vec<t>
        pancakes: Vec<struct Pancake {
            x: f32,
            y: f32,
            vel: f32,
            radius: f32,
        }>,
        score: u32,
    } = {
        Self { // when we defint he struct we put the default values here
            frame: 0, // unassigned integer 
            last_munch_at: 0,
            cat_x: 128.0, // unassigned float 
            cat_y: 112.0,
            cat_r: 32.0,
            score: 0,
            pancakes: vec![]
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! ({
    // load our game state
    let mut state = GameState::load();

    // get our user input
    if gamepad(0).left.pressed()
    {
        // access our gamestate struct using dot notation
        state.cat_x -= 2.0;
    }

    if gamepad(0).right.pressed()
    {
        // access our gamestate struct using dot notation
        state.cat_x += 2.0;
    }

    // generate pancakes at random intervals
    if rand() % 64 == 0 {
        // construct a new pancake
        let pancake = Pancake {
            x: (rand() % 256) as f32, // make the x random between 0 and 256, and then cast as an f32
            y: 0.0,
            vel: (rand() % 1 + 1) as f32, // make the velocity between 1 and 4
            radius: (rand() % 10 + 5) as f32,
        };
        // now that we've made our pancake, use the push array function to append it to our mutable array
        state.pancakes.push(pancake);
    }

    // update the pancake positions and check for collisions with the cat
    // this is a tuple of (f32, f32) defined by our x + r, and y + r
    let cat_center = (state.cat_x + state.cat_r, state.cat_y + state.cat_r);

    // loop through our pancakes and update them
    state.pancakes.retain_mut(|pancake| {
        // move them by their velocity
        pancake.y += pancake.vel;

        // define this pancake's center point
        let pancake_center = (pancake.x + pancake.radius, pancake.y + pancake.radius);

        // check for collisions with the player by comparing the distance from the pancake to the cat
        let dx = cat_center.0 - pancake_center.0; // we are accessing the 0th element of the tuple
        let dy = cat_center.1 - pancake_center.1;

        // get the distance between our positions
        let distance = (dx * dx + dy * dy).sqrt(); // the distance between centers
        let radii_sum = state.cat_r + pancake.radius; // the sum of this pancake's radius and the cat's radius
        let radii_dif = (state.cat_r - pancake.radius).abs(); // the absolute value of the cat's radius and the pancake's radius 
        // now check if the difference in radii is less than the distance, and if the distance is less than the sum of the radii,
        // this means that the edges are overlapping
        if radii_dif <= distance && distance <= radii_sum 
        {
            // the cat caught the pancake
            state.score += 1;
            state.last_munch_at = state.frame; // when our last munch was
            false // remove the pancake from the game
        } else if pancake.y < 144.0 + (pancake.radius * 2.0)
        {
            // keep the pancake on the screen as long as it is visible by twice its radius at the bottom of the screen
            true
        } else
        {
            // remove the pancake if it is off screen
            false
        }
    });

    // set the background color
    clear(0x00ffffff);

    // draw a tiles background of moving sprites
    let frame = state.frame / 2;
    // loop through each column on the screen from 0 to 9
    for column in 0..9 {
        for row in 0..6 {
            let x = column * 32;
            let y = row * 32;
            let x = ((x + frame) % (272 + 16)) - 32;
            let y = ((y + frame) % (144 + 16)) - 24;
            // does the actual drawing of the sprite
            sprite!("heart", x = x, y = y);
        }
    }

    // draw a speech bubble whenever the cat easts a pancake
    // using saturating_sub to see if this current frame take away the last frame is less than 60 frames,
    // so that we only perform one speech bubble every second.
    // same as writing (state.frame - state.last_munch_at <= 60) if we wanted negative numbers
    if state.frame >= 64 && state.frame.saturating_sub(state.last_munch_at) <= 60 {
        // draw a rectangle
        rect!(w = 30, h = 10, x = state.cat_x + 32.0, y = state.cat_y);
        // draw a circle
        circ!(d = 10, x = state.cat_x + 28.0, y = state.cat_y + 5.0);
        rect!(w = 10, h = 5, x = state.cat_x + 28.0, y = state.cat_y + 5.0);
        circ!(d = 10, x = state.cat_x + 56.0, y = state.cat_y);
        // draw our text
        text!(
            "MUNCH!",
            x = state.cat_x + 33.0,
            y = state.cat_y + 3.0,
            font = Font::S,
            color = 0x000000ff
        );
    }

    // draw our cat in its position
    sprite!(
        "munch_cat",
        x = state.cat_x,
        y = state.cat_y,
        // now define the framerate
        fps = fps::FAST // options are FAST, SLOW, MEDIUM
    );

    // should we debug?
    let debug = false;

    // draw the falling pancakes by doing a for loop for each pancake
    for pancake in &state.pancakes 
    {
        // shadows
        circ!(
            x = pancake.x - pancake.radius/2.0,
            y = pancake.y + pancake.radius*2.0,
            d = pancake.radius,
            color = 0x000000aa
        );
        circ!(
            x = pancake.x - pancake.radius/2.0,
            y = pancake.y + pancake.radius*2.0,
            d = pancake.radius - 1.0,
            color = 0xf4d29cff
        );
        circ!(
            x = pancake.x - pancake.radius/2.0,
            y = pancake.y + pancake.radius*2.0,
            d = pancake.radius - 2.0,
            color = 0x0dba463ff
        );

        if debug {
            circ!(
                x = pancake.x,
                y = pancake.y,
                d = 1,
                color = 0xfc0303ff
            );
        }
    }

    
    if debug
    {
    // draw the cat hitbox
    circ!(
        x = state.cat_x,
        y = state.cat_y,
        d = state.cat_r,
        color = 0xfc0303ff
    );
    }

    let score = format!("Score: {}", state.score);
    // draw the score
    text!(&score, x = 10, y = 10, font = Font::L, color = 0xffffffff);

    // save the game state for the next frame
    state.frame += 1;
    state.save();
});