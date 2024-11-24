// use std::{string, vec};

use std::{env::current_exe, io::SeekFrom, string, thread::current};

turbo::cfg! {r#"
    name = "Fast Trash Cooperative Adventure"
    version = "1.0.0"
    author = "Garlic Sale"
    description = "Trash Machine!"
    [settings]
    resolution = [512,288]
    [turbo-os]
    api-url = "https://os.turbo.computer"
"#}

turbo::init! {
    struct GameState {
        frame: u32, // unassigned integer 
        current_order: String, // the order we're making
        up: String, // the order we're making
        down: String, // the order we're making
        left: String, // the order we're making
        right: String, // the order we're making
        menu: Vec<struct Food {
            arrows: String, //UUDDLLRR <- as up down left right
            name: String
        }>,
        initialized: bool, // has the game been initialized?
        player_num: i32,
        random_numbers: Vec<u32>,
        is_playing: bool,
        current_inputs: String,
        start_frame: u32,
        current_char: u32,
        last_wrong_time: u32,
        last_right_time: u32,
        show_menu: bool,
        menu_x: u32,
        menu_y: u32,
        remaining_time: u32,
        hands_x: u32,
        hands_y: u32,
        hands_x_d: u32,
        hands_y_d: u32,
        last_success: u32,
        last_failure: u32,
        foodx: u32,
        foody: u32,
        foodx_d: u32,
        foody_d: u32,
        food_show_time:u32
    } = {
        Self { // when we define the struct we put the default values here
        frame: 0,
        current_order: String::from(""),
        menu: vec![],
        initialized: false,
        up: String::from("U"),
        down: String::from("D"),
        right: String::from("R"),
        left: String::from("L"),
        player_num: 0,
        random_numbers: vec![],
        is_playing: false,
        current_inputs: String::from(""),
        start_frame: 0,
        current_char: 0,
        last_wrong_time: 0,
        last_right_time: 0,
        show_menu: true,
        menu_x: 20,
        menu_y: 120,
        remaining_time: 60,
        hands_x: 250,
        hands_y: 210,
        hands_x_d: 250,
        hands_y_d: 210,
        last_success: 0,
        last_failure: 0,
        foodx: 300,
        foody: 120,
        foodx_d: 300,
        foody_d: 160,
        food_show_time: 120
        }
    }
}

turbo::go!({
    let program_id = "dawn-of-the-final-burger";

    // load in the game state
    let mut state = GameState::load();

    let mut num = 0;

    // our menu length
    let menu_length = 2;

    let rand_data = os::client::watch_file(program_id, "random num").data;

    if let Some(file) = rand_data {
        num = u32::try_from_slice(&file.contents).unwrap_or(0);
    }

    // all the food
    // hamburger
    let hamburger_data = os::client::watch_file(program_id, "send recipe hamburger").data;
    let mut hamburger_food_from_server = Food {arrows: "".to_string(), name: "".to_string()};
    if let Some(file) = hamburger_data {
        hamburger_food_from_server = Food::try_from_slice(&file.contents).unwrap_or(Food {arrows: "".to_string(), name: "".to_string()});
    }

    // double burger
    let double_data = os::client::watch_file(program_id, "send recipe double").data;
    let mut double_food_from_server = Food {arrows: "".to_string(), name: "".to_string()};
    if let Some(file) = double_data {
        double_food_from_server = Food::try_from_slice(&file.contents).unwrap_or(Food {arrows: "".to_string(), name: "".to_string()});
    }

    // cheeseburger
    let cheeseburger_data = os::client::watch_file(program_id, "send recipe cheeseburger").data;
    let mut cheeseburger_food_from_server = Food {arrows: "".to_string(), name: "".to_string()};
    if let Some(file) = cheeseburger_data {
        cheeseburger_food_from_server = Food::try_from_slice(&file.contents).unwrap_or(Food {arrows: "".to_string(), name: "".to_string()});
    }

    // french fries
    let fries_data = os::client::watch_file(program_id, "send recipe french_fries").data;
    let mut fries_food_from_server = Food {arrows: "".to_string(), name: "".to_string()};
    if let Some(file) = fries_data {
        fries_food_from_server = Food::try_from_slice(&file.contents).unwrap_or(Food {arrows: "".to_string(), name: "".to_string()});
    }

    // milkshakes
    let shake_data = os::client::watch_file(program_id, "send recipe shake").data;
    let mut shake_food_from_server = Food {arrows: "".to_string(), name: "".to_string()};
    if let Some(file) = shake_data {
        shake_food_from_server = Food::try_from_slice(&file.contents).unwrap_or(Food {arrows: "".to_string(), name: "".to_string()});
    }

    let active_player = os::client::watch_file(program_id, "current player").data;
    let mut active_player_from_server = 0 as u32; 


    if let Some(file) = active_player {
        active_player_from_server = u32::try_from_slice(&file.contents).unwrap_or(0);
    }


    // our time
    let current_time = os::client::watch_file(program_id, "remaining time").data;
    let mut remaining_time_from_server = 0 as u32;

    if let Some(file) = current_time {
        remaining_time_from_server = u32::try_from_slice(&file.contents).unwrap_or(0);
    }

    // our score
    let current_score = os::client::watch_file(program_id, "current score").data;
    let mut active_score_from_server = 0 as u32;

    if let Some(file) = current_score {
        active_score_from_server = u32::try_from_slice(&file.contents).unwrap_or(0 as u32);
    }

    // our current order
    let current_order = os::client::watch_file(program_id, "current order").data;
    let mut current_order_from_server = "".to_string();

    if let Some(file) = current_order {
        current_order_from_server = String::try_from_slice(&file.contents).unwrap_or("".to_string());
    }

    // now setup the current order as inputs
    let mut current_order_arrows = "".to_string();
    for food in &state.menu
    {
        let k = state.menu.clone();

        if (food.name == current_order_from_server)
        {
            current_order_arrows = food.arrows.clone();
        }
    }

    // if we're not playing yet, set our player based on key inputs, where left is 1, up is 2, right is 3, down is reset
    if (!state.is_playing)
    {

        // 1, 2, 3
        if gamepad(0).left.pressed() { state.player_num = 1;}
        if gamepad(0).up.pressed() { state.player_num = 2; }
        if gamepad(0).right.pressed() { state.player_num = 3;}
        // reset
        if gamepad(0).down.pressed() { state.player_num = 0;}

        if (state.player_num != 0)
        {
            // now make it so that we are playing
            state.is_playing = true;
            state.start_frame = state.frame;
        } 
    }

    if (num == 0)
    {
        // number!
        os::client::exec(program_id, "random", &[]);
    }

    // if we have not initialized
    if (!state.initialized && num != 0 && state.is_playing)
    {
        
        os::client::exec(program_id, "random", &[]);
        // take the number and make it a string then slice the string
        let snum: String = num.to_string(); // to bum >:D

        if (snum.len() > 0)
        {
        for _i in 0..snum.len()
        {
            let j: u32 = snum[_i.._i+1].parse::<u32>().unwrap_or(0);
            state.random_numbers.push(j);
        };
        }

        if (state.player_num == 1)
        {
            log!("Player 1");
            // create random orders
            // hamburger
            let hamburger = Food {
                // arrows
                arrows: arrow_gen(6, &mut state, &0),
                // name
                name: String::from("hamburger")
            };

            // now log the hamburger
            log!("{}", hamburger.name);
            log!("{}", hamburger.arrows);        
            let food_bytes = hamburger.try_to_vec().unwrap();
            // now that we have our recipes, send them to the server for others to read
            os::client::exec(program_id, "submit-recipe", &food_bytes);
            // save it to our menu
            state.menu.push(hamburger);
            
            // create random orders
            // double
            let double = Food {
                // arrows
                arrows: arrow_gen(6, &mut state, &2),
                // name
                name: String::from("double")
            };

            // now log the hamburger
            log!("{}", double.name);
            log!("{}", double.arrows);

            let food_bytes = double.try_to_vec().unwrap();
            // now that we have our recipes, send them to the server for others to read
            os::client::exec(program_id, "submit-recipe", &food_bytes);
            // add it to the menu
            state.menu.push(double);
           
            // cheeseburger
            let cheeseburger = Food {
                // arrows
                arrows: arrow_gen(6, &mut state, &3),
                // name
                name: String::from("cheeseburger")
            };

            // now log the hamburger
            log!("{}", cheeseburger.name);
            log!("{}", cheeseburger.arrows);

            let food_bytes = cheeseburger.try_to_vec().unwrap();

            // now that we have our recipes, send them to the server for others to read
            os::client::exec(program_id, "submit-recipe", &food_bytes);
            // add it to the menu
            state.menu.push(cheeseburger);

            // fries
            let fries = Food {
                // arrows
                arrows: arrow_gen(6, &mut state, &4),
                // name
                name: String::from("french_fries")
            };

            // now log the hamburger
            log!("{}", fries.name);
            log!("{}", fries.arrows);

            let food_bytes = fries.try_to_vec().unwrap();
            // now that we have our recipes, send them to the server for others to read
            os::client::exec(program_id, "submit-recipe", &food_bytes);
            // add it to the menu
            state.menu.push(fries);

            // milkshake
            let shake = Food {
                // arrows
                arrows: arrow_gen_shake(6, &mut state, &0),
                // name
                name: String::from("shake")
            };

            // now log the hamburger
            log!("{}", shake.name);
            log!("{}", shake.arrows);        
            let food_bytes = shake.try_to_vec().unwrap();
            // now that we have our recipes, send them to the server for others to read
            os::client::exec(program_id, "submit-recipe", &food_bytes);
            // save it to our menu
            state.menu.push(shake);    

            // if we don't have a current recipe, set one up
            if (state.menu.len() > 0 && current_order_from_server == "".to_string() && state.player_num == 1)
            {
                log!("sending current order...");
                // pick a random recipe
                let mut i = rand() % (state.menu.len()) as u32;
                log!("{}", i);
                // i -= 1;
                let current_order_bytes = state.menu[i as usize].name.try_to_vec().unwrap();
                os::client::exec(program_id, "submit-current-order", &current_order_bytes);
            }

            // if we have a current order then log one
            if (current_order_from_server != "".to_string())
            {
                log!("order found: ");
                log!("{}", current_order_from_server);
            }

            // send in our remaining time
            let i = 60 as u32;
            let t = i.try_to_vec().unwrap();
            os::client::exec(program_id, "submit-remaining-time", &t);

            // now we're initialized
            state.initialized = true;
            
        }   

        if (state.player_num == 2 || state.player_num == 3)
        {
            if (state.player_num == 2)
            {log!("Player 2")}
            if (state.player_num == 3)
            {log!("Player 3")}
            // if we are other players then get our recipes and save them locally
            if (hamburger_food_from_server.name != "")
            {
                // if it doesn't contain it, add it
                if !state.menu.contains(&hamburger_food_from_server)
                {
                    state.menu.push(hamburger_food_from_server);
                }

                // if it doesn't contain it, add it
                if !state.menu.contains(&double_food_from_server)
                {
                    state.menu.push(double_food_from_server);
                }

                // if it doesn't contain it, add it
                if !state.menu.contains(&cheeseburger_food_from_server)
                {
                    state.menu.push(cheeseburger_food_from_server);
                }

                // if it doesn't contain it, add it
                if !state.menu.contains(&fries_food_from_server)
                {
                    state.menu.push(fries_food_from_server);
                }

                // if it doesn't contain it, add it
                if !state.menu.contains(&shake_food_from_server)
                {
                    state.menu.push(shake_food_from_server);
                }

                for i in 0..state.menu.len()
                {
                    log!("{}",state.menu[i].name);
                    log!("{}",state.menu[i].arrows);
                }
                
            }

            // print the current order
            log!("order received:");
            log!("{}", current_order_from_server);

            // if the menu is the right length then we are initialized
            if (state.menu.len() >= menu_length)
            {
                state.initialized = true;
            }

        }
    }

    // make sure we have a current order baby doll
    if (state.initialized && current_order_from_server == "".to_string())
    {
        log!("sending current order...");
        log!("{}", current_order_from_server);
        // pick a random recipe
        let mut i = rand() % (state.menu.len()) as u32;
        log!("{}", i);
        // i -= 1;
        let current_order_bytes = state.menu[i as usize].name.try_to_vec().unwrap();
        os::client::exec(program_id, "submit-current-order", &current_order_bytes);
    
    }

    // gives us a random arrow string
    fn arrow_gen(count: i32, state: & mut GameState, add: &u32) -> String
    { 
        let mut s: String = "".to_owned();
        if (count > state.random_numbers.len() as i32)
        {
             return "".to_string()
        }

        for x in 0..count as i32 {
        let i = state.random_numbers[x as usize];
        if i + add == 0 {s.push_str(&state.up);}
        if i + add == 1 {s.push_str(&state.down);}
        if i + add == 2 {s.push_str(&state.left);}
        if i + add == 3 {s.push_str(&state.right);}
        if i + add == 4 {s.push_str(&state.up);}
        if i + add == 5 {s.push_str(&state.down);}
        if i + add == 6 {s.push_str(&state.left);}
        if i + add == 7 {s.push_str(&state.right);}
        if i + add == 8 {s.push_str(&state.down);}
        if i + add == 9 {s.push_str(&state.left);}
        if i + add == 10 {s.push_str(&state.right);}
        if i + add == 11 {s.push_str(&state.down);}
        if i + add == 12 {s.push_str(&state.up);}
        if i + add == 13 {s.push_str(&state.left);}
        if i + add == 14 {s.push_str(&state.right);}
        if i + add == 15 {s.push_str(&state.down);}
        if i + add == 16 {s.push_str(&state.up);}
        }
        String::from(s)
    }

    // gives us a random arrow string
    fn arrow_gen_shake(count: i32, state: & mut GameState, add: &u32) -> String
    { 
        let mut s: String = "".to_owned();
        if (count > state.random_numbers.len() as i32)
        {
                return "".to_string()
        }

        for x in 0..count as i32 {
        let i = state.random_numbers[x as usize];
        if i + add == 0 {s.push_str(&state.up);}
        if i + add == 1 {s.push_str(&state.down);}
        if i + add == 2 {s.push_str(&state.up);}
        if i + add == 3 {s.push_str(&state.down);}
        if i + add == 4 {s.push_str(&state.up);}
        if i + add == 5 {s.push_str(&state.down);}
        if i + add == 6 {s.push_str(&state.up);}
        if i + add == 7 {s.push_str(&state.down);}
        if i + add == 8 {s.push_str(&state.up);}
        if i + add == 9 {s.push_str(&state.down);}
        if i + add == 10 {s.push_str(&state.up);}
        if i + add == 11 {s.push_str(&state.down);}
        if i + add == 12 {s.push_str(&state.up);}
        if i + add == 13 {s.push_str(&state.down);}
        if i + add == 14 {s.push_str(&state.up);}
        if i + add == 15 {s.push_str(&state.down);}
        if i + add == 16 {s.push_str(&state.up);}
        }
        String::from(s)
    }

    clear!(0xe7b84eff);

    // set menu positions
    if (state.show_menu)
    {
        if (state.menu_y > 120)
        {
            state.menu_y -= 1;
        }
    } else if (state.menu_y < 300) {
        state.menu_y += 1;
    }

    
    // draw background
    sprite!("BURGER bg", x = 212, y = 0, scale_x = 1.5, scale_y = 1.5);

    // draw the menu background
    rect!(w = 180, h = 110, x = state.menu_x - 10, y = state.menu_y - 10, color = 0xcb5917ff);
    rect!(w = 170, h = 100, x = state.menu_x - 5, y = state.menu_y - 5, color = 0x993800ff);

    // draw the menu
    text!("Tasty Menu", x = state.menu_x, y = state.menu_y, font = Font::XL);
    for i in 0..state.menu.len()
    {
        let item = &state.menu[i];
        text!(&item.name, x = state.menu_x, y = state.menu_y + ((i+1)*10 + 10) as u32, font = Font::L);
        text!(&item.arrows, x = state.menu_x + 100, y = state.menu_y + ((i+1)*10 + 10) as u32, font = Font::L);
    }

   
    // draw the current order
    let co = current_order_from_server.to_string();
    text!("Current Order:", x = 20, y = 10, font = Font::XL, color = 0xff0000ff);
    text!(&co, x = 20, y = 30, font = Font::XL, color = 0xff0000ff);
    text!("Active Player:", x = 0, y = 0);
    text!(&active_player_from_server.to_string(), x = 0, y = 10);

    // set the active player if we're playing 1
    if (state.player_num == 1)
    {
        if active_player_from_server == 0
        {
            let i = 1;
            let p_bytes = i.try_to_vec().unwrap();
            os::client::exec(program_id, "submit-current-player", &p_bytes);
        }
    }

    let val: f32 = state.frame as f32;

    // if we are the active player
    if (active_player_from_server == state.player_num as u32 && remaining_time_from_server > 0)
    {
        text!("MAKE THE ORDER!", x = 20, y = (60 + (val.sin() * 2.0) as u32), color = 0xff0000ff, font = Font::XL);
        if (state.player_num == 1) { text!("FIRST TWO STEPS!", x = 20, y = (80 + (val.sin() * 2.0) as u32), color = 0xff0000ff, font = Font::XL); }
        if (state.player_num == 2) { text!("FIRST FOUR STEPS!", x = 20, y = (80 + (val.sin() * 2.0) as u32), color = 0xff0000ff, font = Font::XL); }
        if (state.player_num == 3) { text!("ALL STEPS!", x = 20, y = (80 + (val.sin() * 2.0) as u32), color = 0xff0000ff, font = Font::XL); }
    }
    
    // draw our inputs
    if (state.is_playing && state.frame != state.start_frame)
    {
        if gamepad(0).left.just_pressed() { state.current_inputs.push('L'); state.current_char += 1;}
        if gamepad(0).up.just_pressed() { state.current_inputs.push('U');  state.current_char += 1;}
        if gamepad(0).right.just_pressed() { state.current_inputs.push('R'); state.current_char += 1;}
        if gamepad(0).down.just_pressed() { state.current_inputs.push('D'); state.current_char += 1;}

        // clear!
        if gamepad(0).a.just_pressed() {
            log!("reset");
            state.current_inputs.clear();
            state.current_char = 0;

            // get a new random number
            os::client::exec(program_id, "random", &[]);
            let rand_data = os::client::watch_file(program_id, "random num").data;
            if let Some(file) = rand_data {
                num = u32::try_from_slice(&file.contents).unwrap_or(0);
            }

            // reset score
            let j = 0;
            let j_bytes = j.try_to_vec().unwrap();
            os::client::exec(program_id, "submit-current-score", &j_bytes);
            let i = 0;
            let p_bytes = i.try_to_vec().unwrap();
            os::client::exec(program_id, "submit-current-player", &p_bytes);

            let current_order_bytes = "".try_to_vec().unwrap();
            os::client::exec(program_id, "submit-current-order", &current_order_bytes);
            
            let z = 60;
            let time_bytes = z.try_to_vec().unwrap();
            os::client::exec(program_id, "submit-remaining-time", &time_bytes);
        }

        // use the b button to open and close the menu
        if gamepad(0).y.just_pressed() {
            state.show_menu = !state.show_menu;

        }
    }

    // score debug
    if (gamepad(0).select.just_pressed())
    {
        // reset score
        let j = active_score_from_server + 1;
        let j_bytes = j.try_to_vec().unwrap();
        os::client::exec(program_id, "submit-current-score", &j_bytes);
    }

    // draw our inputs on the screen
    // log!("{}", val.sin().to_string());
    text!(r"\/ MAKE THE FOOD \/", x = 200, y = (120 + (val.sin() * 2.0) as u32), color = 0xff5e00ff, font = Font::XL);
    text!(&state.current_inputs.to_string(), x = 380, y = 200, color = 0xff5e00ff, font = Font::XL);
    // text!(&state.current_char.to_string(), x = 200, y = 160, color = 0xff5e00ff, font = Font::XL);

    // draw a rect so we can read the time remaining
    rect!(w = 300, h = 20, x = 230, y = 28, color = 0xffffffff);

    // draw the score 
    text!("SCORE:", x = 300, y = 10, color = 0xff0000ff, font = Font::XL);
    text!(&active_score_from_server.to_string(), x = 390, y = 10, color = 0xff0000ff, font = Font::XL);

    // draw the score 
    text!("TIME REMAINING:", x = 235, y = 30, color = 0xfff7000ff, font = Font::XL);
    text!(&remaining_time_from_server.to_string(), x = 480, y = 30, color = 0xff7000ff, font = Font::XL);


    // check if this input is correct
    // player 1
    if state.player_num == 1 as i32 && state.current_char >= 2
    {        
        let cis = state.current_inputs.clone(); 
        let coas = current_order_arrows.clone();
        
        log!("{:?}", coas);
        let coa = &coas[..2];


        // check if coa starts with cis 
        if coa.starts_with(&cis)
        {
            log!("huh");
            if (state.current_char >= 2)
            {
                let i = 2;
                let p_bytes = i.try_to_vec().unwrap();
                os::client::exec(program_id, "submit-current-player", &p_bytes);
                state.last_success = state.frame;
            log!("WAHG");
                state.current_inputs.clear();
                state.current_char = 0;
                state.last_right_time = state.frame;
                state.foodx = state.foodx_d;
                state.foody = state.foody_d;
            }
        } else if !coa.starts_with(&cis) {
            state.current_inputs.clear();
            state.current_char = 0;
            state.last_wrong_time = state.frame;
            state.foodx = state.foodx_d;
            state.foody = state.foody_d;
        }
    }

    // player 2
    if state.player_num == 2 as i32 && state.current_char > 0
    {
        let cis = state.current_inputs.clone(); 
        let coas = current_order_arrows.clone();
        let coa = &coas[..4];

        // check if coa starts with cis 
        if coa.starts_with(&cis)
        {
            if (state.current_char >= 4)
            {
                let i = 3;
                let p_bytes = i.try_to_vec().unwrap();
                os::client::exec(program_id, "submit-current-player", &p_bytes);
                
                state.current_inputs.clear();
                state.current_char = 0;
                state.last_right_time = state.frame;
                state.foodx = state.foodx_d;
                state.foody = state.foody_d;
            }
        } else if !coa.starts_with(&cis) {
            state.current_inputs.clear();
            state.current_char = 0;
            state.last_wrong_time = state.frame;
            state.foodx = state.foodx_d;
            state.foody = state.foody_d;
            // reset the player
            let i = 1;
            let p_bytes = i.try_to_vec().unwrap();
            os::client::exec(program_id, "submit-current-player", &p_bytes);
        }
    }

    // player 3
    if state.player_num == 3 as i32 && state.current_char > 0
    {
        let cis = state.current_inputs.clone(); 
        let coas = current_order_arrows.clone();
        let coa = &coas[..6];

        // check if coa starts with cis 
        if coa.starts_with(&cis)
        {
            if (state.current_char >= 6)
            {
                let i = 1;
                let p_bytes = i.try_to_vec().unwrap();
                os::client::exec(program_id, "submit-current-player", &p_bytes);
                
                let j = active_score_from_server + 1;
                let j_bytes = j.try_to_vec().unwrap();
                os::client::exec(program_id, "submit-current-score", &j_bytes);
                state.last_right_time = state.frame;
                state.foodx = state.foodx_d;
                state.foody = state.foody_d;
                state.current_inputs.clear();
                state.current_char = 0;

                // new order!
                log!("sending current order...");
                // pick a random recipe
                let mut i = rand() % (state.menu.len()) as u32;
                log!("{}", i);
                // i -= 1;
                let current_order_bytes = state.menu[i as usize].name.try_to_vec().unwrap();
                os::client::exec(program_id, "submit-current-order", &current_order_bytes);
            }
        } else if !coa.starts_with(&cis) {
            state.current_inputs.clear();
            state.current_char = 0;
            state.last_wrong_time = state.frame;
            state.foodx = state.foodx_d;
            state.foody = state.foody_d;
            // reset the player
            let i = 1;
            let p_bytes = i.try_to_vec().unwrap();
            os::client::exec(program_id, "submit-current-player", &p_bytes);
        }
    }


    // do the seconds countdown
    if (state.player_num == 1 && remaining_time_from_server > 0)
    {
        if (state.frame % 60 == 0)
        {
            let i = remaining_time_from_server - 1;
            let time_bytes = i.try_to_vec().unwrap();
            // now that we have our recipes, send them to the server for others to read
            os::client::exec(program_id, "submit-remaining-time", &time_bytes);
        }
    }

    // draw the hands!
    sprite!("hand pixels", x = state.hands_x + (val.sin() * 1.0) as u32, y = state.hands_y + (val.sin() * 2.0) as u32, scale_x = 1.5, scale_y = 1.5);

    // make the moves
    if gamepad(0).left.pressed() { state.hands_x = 200; }
    if gamepad(0).up.pressed() { state.hands_y = 100;  }
    if gamepad(0).right.pressed() { state.hands_x = 350; }
    if gamepad(0).down.pressed() { state.hands_y = 350;}

    if (state.hands_x > state.hands_x_d) { state.hands_x -= 20}
    if (state.hands_x < state.hands_x_d) { state.hands_x += 20}
    if (state.hands_y > state.hands_y_d) { state.hands_y -= 10}
    if (state.hands_y < state.hands_y_d) { state.hands_y += 10}

    // move the food if we recently succeeded
    if (state.frame >= state.last_right_time + 30)
    {
        state.foodx += 5;
    }

    // sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
    // what are we making?
    if (current_order_from_server == "hamburger")
    {
        // if we're player 1 and we succeed with the burger
        if (state.player_num == 1 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 1 and we fail with the burger
        if (state.player_num == 1 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we succeed with the burger
        if (state.player_num == 2 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("burgerpixlettice true", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we fail with the burger
        if (state.player_num == 2 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerpixlettice false", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 3 and we succeed with the burger
        if (state.player_num == 3 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("meal success", x = state.foodx, y = state.foody - 50, scale_x = 4, scale_y = 4);
        }

        // if we're player 3 and we fail with the burger
        if (state.player_num == 3 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerpixbun top", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixlettice false", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }
    }

    if (current_order_from_server == "double")
    {
        // if we're player 1 and we succeed with the burger
        if (state.player_num == 1 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody - 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 1 and we fail with the burger
        if (state.player_num == 1 && state.frame < state.last_wrong_time + state.food_show_time)
        { 
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody - 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we succeed with the burger
        if (state.player_num == 2 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("burgerpixlettice true", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody - 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we fail with the burger
        if (state.player_num == 2 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerpixlettice false", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
           sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody - 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 3 and we succeed with the burger
        if (state.player_num == 3 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("meal success", x = state.foodx-30, y = state.foody - 50, scale_x = 4, scale_y = 4);
        }

        // if we're player 3 and we fail with the burger
        if (state.player_num == 3 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerpixbun top", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixlettice false", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixcheesefalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody + 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }
    }

    if (current_order_from_server == "cheeseburger")
    {
        // if we're player 1 and we succeed with the burger
        if (state.player_num == 1 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody - 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 1 and we fail with the burger
        if (state.player_num == 1 && state.frame < state.last_wrong_time + state.food_show_time)
        { 
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody - 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we succeed with the burger
        if (state.player_num == 2 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("burgerpixlettice true", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixcheesetrue", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody - 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgertrue", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we fail with the burger
        if (state.player_num == 2 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerpixlettice false", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixcheesefalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody - 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 3 and we succeed with the burger
        if (state.player_num == 3 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("meal success", x = state.foodx, y = state.foody - 50, scale_x = 4, scale_y = 4);
        }

        // if we're player 3 and we fail with the burger
        if (state.player_num == 3 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerpixbun top", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixlettice false", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixcheesefalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
            sprite!("burgerpixburgerfalse", x = state.foodx, y = state.foody + 10, scale_x = 2, scale_y = 2);
            sprite!("burgerpixbottom bun", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }
    }

    if (current_order_from_server == "french_fries")
    {
        // if we're player 1 and we succeed with the burger
        if (state.player_num == 1 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("burgerfries1", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 1 and we fail with the burger
        if (state.player_num == 1 && state.frame < state.last_wrong_time + state.food_show_time)
        { 
            sprite!("burgerfriesfail", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we succeed with the burger
        if (state.player_num == 2 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("burgerfries2", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
           }

        // if we're player 2 and we fail with the burger
        if (state.player_num == 2 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerfriesfail", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 3 and we succeed with the burger
        if (state.player_num == 3 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("meal success", x = state.foodx, y = state.foody - 50, scale_x = 4, scale_y = 4);
        }

        // if we're player 3 and we fail with the burger
        if (state.player_num == 3 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("burgerfriesfail", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }
    }

    if (current_order_from_server == "shake")
    {
        // if we're player 1 and we succeed with the shake
        if (state.player_num == 1 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("drinkfill1", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }
        
        // if we're player 1 and we succeed with the shake
        if (state.player_num == 1 && state.frame < state.last_right_time + state.food_show_time/2)
        {
            sprite!("drinkempty", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 1 and we fail with the shake
        if (state.player_num == 1 && state.frame < state.last_wrong_time + state.food_show_time)
        { 
            sprite!("drinkfail", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we succeed with the shake
        if (state.player_num == 2 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("drinkfill3", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 2 and we succeed with the shake
        if (state.player_num == 2 && state.frame < state.last_right_time + state.food_show_time/2)
        {
            sprite!("drinkfill2", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }
        
        // if we're player 2 and we fail with the shake
        if (state.player_num == 2 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("drinkfail", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }

        // if we're player 3 and we succeed with the shake
        if (state.player_num == 3 && state.frame < state.last_right_time + state.food_show_time)
        {
            sprite!("meal_success", x = state.foodx, y = state.foody - 50, scale_x = 4, scale_y = 4);
        }

        // if we're player 3 and we fail with the shake
        if (state.player_num == 3 && state.frame < state.last_wrong_time + state.food_show_time)
        {
            sprite!("drinkfail", x = state.foodx, y = state.foody, scale_x = 2, scale_y = 2);
        }
    }


    
    // do we render our WRONG text
    if (state.frame <= state.last_wrong_time + 80) {
        text!("!! W R O N G !!", x = 220.0 + (val.sin() * 10.0), y = 160.0 + (val.sin() * 2.0), color = 0xff0000ff, font = Font::XL);
    }

    // do we render our YAY text
    if (state.frame <= state.last_right_time + 80) {
        text!("!! YAAYYY !!", x = 220.0 + (val.sin() * 10.0), y = 160.0 + (val.sin() * 2.0), color = 0x14ff00ff, font = Font::XL);
    }


    // do we render our WRONG text
    if (remaining_time_from_server <= 0) {

        rect!(w = 500, h = 100, x = 200, y = 140, color = 0xff0000ff);
        rect!(w = 500, h = 80, x = 210, y = 150, color = 0xffffffff);
        text!("GAME OVER", x = 220.0, y = 160.0 + (val.sin() * 1.0), color = 0xff0000ff, font = Font::XL);
        text!("SCORE:", x = 220.0, y = 180.0, color = 0xff0000ff, font = Font::XL);
        text!(&active_score_from_server.to_string(), x = 350.0, y = 180.0, color = 0xff0000ff, font = Font::XL);
        text!("RATING:", x = 220.0, y = 200.0, color = 0xff0000ff, font = Font::XL);
        
        if (active_score_from_server < 5)
        {
            text!("LITERAL TRASH WTF LOL", x = 340.0, y = 205.0, color = 0xff0000ff, font = Font::L);
        }

        if (active_score_from_server >= 5 && active_score_from_server < 7)
        {
            text!("EMPLOYABLE I GUESS", x = 340.0, y = 205.0, color = 0xff9600ff, font = Font::L);
        }

        if (active_score_from_server >= 7 && active_score_from_server < 10)
        {
            text!("BURGER-MEISTER", x = 340.0, y = 205.0, color = 0xd6ff00ff, font = Font::L);
        }
        
        if (active_score_from_server >= 10)
        {
            text!("GODBURGER", x = 340.0, y = 205.0, color = 0xd39ff00ff, font = Font::L);
        }
    }

    
    

    // at the end of the frame save the state
    state.frame += 1;
    state.save();
    

});

#[export_name = "turbo/hello"]
unsafe extern "C" fn on_hello() -> usize {
    os::server::log!("Hello, world!");
    return os::server::COMMIT;
}

#[export_name = "turbo/random"]
unsafe extern "C" fn random_number() -> usize {
    let file_path = "random num";
    let mut num: u32 = os::server::random_number();
    let Ok(_) = os::server::write!(&file_path, num) else {
        return os::server::CANCEL;
    };

    let nt = format!("{}", num);

    // log the num
    os::server::log(&nt);

    return os::server::COMMIT;
}

#[export_name = "turbo/submit-recipe"]
unsafe extern "C" fn send_recipe() -> usize {
    let food = os::server::command!(Food);
    let file_path = format!("send recipe {}", food.name);

    // log the food!
    os::server::log!("{:?}", food);

    let Ok(_) = os::server::write!(&file_path, food) else {
        return os::server::CANCEL;
    };


    return os::server::COMMIT;
}

#[export_name = "turbo/submit-current-order"]
unsafe extern "C" fn send_current_order() -> usize {
    let order = os::server::command!(String);
    let file_path = format!("current order");

    let Ok(_) = os::server::write!(&file_path, order) else {
        return os::server::CANCEL;
    };

    return os::server::COMMIT;
}

#[export_name = "turbo/submit-current-player"]
unsafe extern "C" fn send_current_player() -> usize {
    let order = os::server::command!(u32);
    let file_path = format!("current player");

    let Ok(_) = os::server::write!(&file_path, order) else {
        return os::server::CANCEL;
    };

    return os::server::COMMIT;
}

#[export_name = "turbo/submit-current-score"]
unsafe extern "C" fn submit_current_score() -> usize {
    let score = os::server::command!(u32);
    let file_path = format!("current score");

    os::server::log("this");

    let Ok(_) = os::server::write!(&file_path, score) else {
        return os::server::CANCEL;
    };

    return os::server::COMMIT;
}

#[export_name = "turbo/submit-remaining-time"]
unsafe extern "C" fn submit_remaining_time() -> usize {
    let time = os::server::command!(u32);
    let file_path = format!("remaining time");

    let Ok(_) = os::server::write!(&file_path, time) else {
        return os::server::CANCEL;
    };

    return os::server::COMMIT;
}



