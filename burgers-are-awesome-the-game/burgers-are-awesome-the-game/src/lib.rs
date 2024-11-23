use std::{fmt::format, string, vec};

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
        currentOrder: String, // the order we're making
        up: String, // the order we're making
        down: String, // the order we're making
        left: String, // the order we're making
        right: String, // the order we're making
        menu: Vec<struct Food {
            arrows: String, //UUDDLLRR <- as up down left right
            name: String
        }>,
        initialized: bool, // has the game been initialized?
        player: i32,
        random_numbers: Vec<u32>
    } = {
        Self { // when we define the struct we put the default values here
        frame: 0,
        currentOrder: String::from(""),
        menu: vec![],
        initialized: false,
        up: String::from("U"),
        down: String::from("D"),
        right: String::from("R"),
        left: String::from("L"),
        player: 0,
        random_numbers: vec![]
        }
    }
}

turbo::go!({
    // load in the game state
    let mut state = GameState::load();

    let mut num = 0;

    let rand_data = os::client::watch_file("burgers-are-awesome-the-game", "random num").data;

    if let Some(file) = rand_data {
        num = u32::try_from_slice(&file.contents).unwrap_or(0);
    }

    // log!("{}", num);

    // log!("{}", num);
    // if we have not initialized
    if (!state.initialized && num != 0)
    {

        // number!
        os::client::exec("burgers-are-awesome-the-game", "random", &[]);

        // take the number and make it a string then slice the string
        let snum: String = num.to_string(); // to bum >:D

        for _i in 0..9
        {
            let j: u32 = snum[_i.._i+1].parse::<u32>().unwrap();
            state.random_numbers.push(j);
        };
        
        // determine which player we are


        // create random orders
        // hamburger
        let hamburger = Food {
            // arrows
            arrows: arrow_gen(&6, &state, &0),
            // name
            name: String::from("hamburger")
        };

        // now log the hamburger
        log!("{}", hamburger.name);
        log!("{}", hamburger.arrows);        
        
        // create random orders
        // hamburger
        let double = Food {
            // arrows
            arrows: arrow_gen(&6, &state, &2),
            // name
            name: String::from("double")
        };

        // now log the hamburger
        log!("{}", double.name);
        log!("{}", double.arrows);

        // then initialize
        state.initialized = true;
    }

    // gives us a random arrow string
    fn arrow_gen(count: &i32, state: &GameState, add: &u32) -> String
    { 
        let mut s: String = "".to_owned();
        for x in 0..*count as i32 {
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
        }
        String::from(s)
    }

    // at the end of the frame save the state
    state.frame += 1;
    state.save();


    clear!(0xADD8E6FF);
    let (x, y, w, h) = (36, 102, 60, 20);
    let mut color = 0x00008BFF;

    let m = mouse(0);
    //check if mouse is over the button and clicked
    if (m.position[0] >= x && m.position[0] <= x + w)
        && (m.position[1] >= y && m.position[1] <= y + h)
    {
        color = 0x4169E1FF;
        if m.left.just_pressed() {
            os::client::exec("burgers-are-awesome", "hello", &[]);
        }
    }
    //draw a button
    rect!(x = x, y = y, w = w, h = h, color = color, border_radius = 8);
    text!("HELLO!!", x = 50, y = 109);
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

#[export_name = "turbo/get-player-num"]
unsafe extern "C" fn get_player_num() -> usize {
    let tp = 0;

    let file_path = "last player";

    let mut last_player = os::server::read_or!(u32, "last player", 0);
    // if the last player is 0 then become player 1
    if (last_player == 0)
    {
        
    }

    let Ok(_) = os::server::write!(&file_path, ) else {
        return os::server::CANCEL;
    };

    let nt = format!("{}", num);

    // log the num
    os::server::log(&nt);

    return os::server::COMMIT;
    
}