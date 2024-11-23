// Define the game's configuration using the turbo::cfg! macro
turbo::cfg! {r#"
    name = "Nuclear Throne Clone"
    version = "1.0.0"
    author = "Josh"
    description = "Nuclear Throne Game Feel Clone"
    [settings]
    resolution = [512,288]
"#}

// setup the game state
turbo::init! {


    struct GameState{
        // game
        frame: u32,
        initialized: bool,
        // player
        player_x: f32,
        player_y: f32,
        player_r: f32,
        player_x_vel: f32, // player speed
        player_y_vel: f32,
        player_x_last_vel: f32,
        camera_x: f32,
        camera_y: f32,
        mouse_x: f32,
        mouse_y: f32,
        // environment
        map_size: u32,
        map_tiles: Vec<usize>,
        enemies: Vec<struct Enemy {
            x: f32,
            y: f32,
            vel: f32,
            dir: f32,
            radius: f32,
            state: u32, // 0: idle, 1: moving, 2: dead
            last_state_change: f32, // the frame on which we last had a state change
            hp: u32 // our health
        }>, 
        collision_rects: Vec<struct CollisionRect { // list of all of the wall collisions
            x: f32,
            y: f32,
            w: f32,
            h: f32
        }>,
        debug_state: bool,
    } = {
        Self {
            // game
            frame: 0,
            initialized: false,
            // player
            player_x: 0.0,
            player_y: 0.0,
            player_r: 0.0,
            player_x_vel: 0.0, // player speed
            player_y_vel: 0.0,
            player_x_last_vel: 0.0,
            camera_x: 0.0,
            camera_y: 0.0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            map_size: 64,
            map_tiles: vec![],
            enemies: vec![],
            collision_rects: vec![],
            debug_state: true,
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! ({
    let mut state = GameState::load();
    // set the background color
    clear(0xb0a14aff);

    // only run if we are not initialized
    if state.initialized == false
    {
        // generate the map tiles
        for _i in 0..state.map_size*state.map_size
        {
            let mut j = rand() % 4000;
            // then add a random amount to j
            j -= rand() % 100;
            state.map_tiles.push(j as usize);
        };
        
        state.initialized = true;
        log!("DEBUG: {}", state.initialized);
        state.save();

        log!("audio running...");
        return;
    }

    // lets draw the background
    for column in 0..state.map_size {
        for row in 0..state.map_size {
            let x = column * 32;
            let y = row * 32;
            let i = ((column * 64) + row) as usize;
            let z = state.map_tiles[i];
            if z <= 2000 { sprite!("desert-tile-1",x=x,y=y)}
            if z >= 2000 && z <= 2750 { sprite!("desert-tile-2",x=x,y=y)}
            if z >= 2750 && z <= 3500 { sprite!("desert-tile-3",x=x,y=y)}
            if z >= 3500 { sprite!("desert-tile-4",x=x,y=y)}
        }
    }

    // reset our player
    if gamepad(0).select.pressed() && state.debug_state == true
    {
        // access our gamestate struct using dot notation
        state.initialized = false;
        state.save();
    }

    // draw and move our player
    let mut input = (0.0, 0.0);

    if gamepad(0).up.pressed() { input.1 -= 1.0;}
    if gamepad(0).down.pressed() { input.1 += 1.0;}
    if gamepad(0).right.pressed() { input.0 += 1.0;}
    if gamepad(0).left.pressed() { input.0 -= 1.0;}

    // perform a collision check with our current player input
    fn collision_check(x: &f32, y: &f32) -> bool
    {
        false
    }

    // checks to see if there is a rect at a specific position
    fn collider_check(x: &f32, y: &f32, state: &mut GameState) -> bool
    {
        // start by saying there is no collision
        let col = false;
        state.collision_rects.retain_mut(|_rect|
        {
            // check if the given X is in between the _rect's x and the rx + w
            // and if the given y is in between the _rect's y and the ry + h
            if (x > &_rect.x)
            {
                
            }

            // then keep it in the rects
            true
        });

        // then return the function
        col
    }

    if (collision_check(&(input.0 as f32), &(input.1 as f32)) == false)
    {
        state.player_x += input.0;
        state.player_y += input.1;
        // set the player's velocity
        state.player_x_vel = input.0;
        state.player_y_vel = input.1;
        // get our last x velocity
        if (input.0 != 0.0) { state.player_x_last_vel = input.0; }
    }

    // then draw the player
    if (state.player_x_vel != 0.0 || state.player_y_vel != 0.0)
    {
        sprite!(
            "fish-sheet-run", 
            x = state.player_x, 
            y = state.player_y,
            fps = fps::FAST,
            flip_x = state.player_x_vel < 0.0
        );
    } else {
        sprite!(
            "fish-sheet-idle",
            x = state.player_x,
            y = state.player_y,
            fps = fps::FAST,
            flip_x = state.player_x_last_vel < 0.0
        );
    }

    // create a new collision rect
    let testRect = CollisionRect {
        x: 100 as f32,
        y: 100 as f32,
        w: 10 as f32,
        h: 10 as f32
    };

    // then add it to the collision rects
    state.collision_rects.push(testRect);

    state.frame += 1;
    state.save();

});