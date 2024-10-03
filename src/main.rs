use std::io::{self, Write};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use onvars_tool::SaveStateUnit;
use onvars_tool::sa2_units::{CharacterUnit, CameraUnit, TimeUnit, GravityUnit, ScoreUnit, RingUnit, LivesUnit};
use onvars_tool::process_reader::ProcessHandle;

fn main() {
    println!("OnVar's Tool (version {})", env!("CARGO_PKG_VERSION"));
    let mut process_string = "sonic2app.exe".to_string();
    let handle;
    'process_hook_loop: loop {
        match ProcessHandle::from_name_filter(|n| n.to_lowercase() == process_string.to_lowercase()).unwrap() {
            Some(h) => {
                handle = h;
                break 'process_hook_loop;
            }
            None => {
                println!();
                println!("Could not find process \"{}\".", process_string);
                println!("Please enter the name of the SA2 process.");
                print!("Process name: ");
                io::stdout().flush().unwrap();
                let stdin = io::stdin();
                process_string.clear();
                stdin.read_line(&mut process_string).unwrap();
                process_string = process_string.trim().to_string();
            }
        }
    }

    println!();
    println!("Successfully hooked into \"{}\".", process_string);
    println!();
    println!("Press D-pad Left to save a state.");
    println!("Press D-pad Right to load a state.");
    println!("Press D-pad Up/D-pad Down to increase/decrease the savestate slot respectively.");
    println!("Hold Y and press D-pad Up to set lives to 99.");
    println!();
    println!("Savestate slot: 1");

    // make separate units for each slot
    // i hate this hack with all my heart
    // i will fix this mess later, surely.
    let units1: Vec<Rc<dyn SaveStateUnit>> = vec![
        Rc::new(CharacterUnit::new()),
        Rc::new(CameraUnit::new()),
        Rc::new(TimeUnit::new()),
        Rc::new(GravityUnit::new()),
        Rc::new(ScoreUnit::new()),
        Rc::new(RingUnit::new()),
        Rc::new(LivesUnit::new()),
    ];
    let units2: Vec<Rc<dyn SaveStateUnit>> = vec![
        Rc::new(CharacterUnit::new()),
        Rc::new(CameraUnit::new()),
        Rc::new(TimeUnit::new()),
        Rc::new(GravityUnit::new()),
        Rc::new(ScoreUnit::new()),
        Rc::new(RingUnit::new()),
        Rc::new(LivesUnit::new()),
    ];
    let units3: Vec<Rc<dyn SaveStateUnit>> = vec![
        Rc::new(CharacterUnit::new()),
        Rc::new(CameraUnit::new()),
        Rc::new(TimeUnit::new()),
        Rc::new(GravityUnit::new()),
        Rc::new(ScoreUnit::new()),
        Rc::new(RingUnit::new()),
        Rc::new(LivesUnit::new()),
    ];
    let units4: Vec<Rc<dyn SaveStateUnit>> = vec![
        Rc::new(CharacterUnit::new()),
        Rc::new(CameraUnit::new()),
        Rc::new(TimeUnit::new()),
        Rc::new(GravityUnit::new()),
        Rc::new(ScoreUnit::new()),
        Rc::new(RingUnit::new()),
        Rc::new(LivesUnit::new()),
    ];
    let units5: Vec<Rc<dyn SaveStateUnit>> = vec![
        Rc::new(CharacterUnit::new()),
        Rc::new(CameraUnit::new()),
        Rc::new(TimeUnit::new()),
        Rc::new(GravityUnit::new()),
        Rc::new(ScoreUnit::new()),
        Rc::new(RingUnit::new()),
        Rc::new(LivesUnit::new()),
    ];
    let units6: Vec<Rc<dyn SaveStateUnit>> = vec![
        Rc::new(CharacterUnit::new()),
        Rc::new(CameraUnit::new()),
        Rc::new(TimeUnit::new()),
        Rc::new(GravityUnit::new()),
        Rc::new(ScoreUnit::new()),
        Rc::new(RingUnit::new()),
        Rc::new(LivesUnit::new()),
    ];
    let units7: Vec<Rc<dyn SaveStateUnit>> = vec![
        Rc::new(CharacterUnit::new()),
        Rc::new(CameraUnit::new()),
        Rc::new(TimeUnit::new()),
        Rc::new(GravityUnit::new()),
        Rc::new(ScoreUnit::new()),
        Rc::new(RingUnit::new()),
        Rc::new(LivesUnit::new()),
    ];
    let units8: Vec<Rc<dyn SaveStateUnit>> = vec![
        Rc::new(CharacterUnit::new()),
        Rc::new(CameraUnit::new()),
        Rc::new(TimeUnit::new()),
        Rc::new(GravityUnit::new()),
        Rc::new(ScoreUnit::new()),
        Rc::new(RingUnit::new()),
        Rc::new(LivesUnit::new()),
    ];

    // push clones of base_units into a vector
    let mut units_vector: Vec<Vec<Rc<dyn SaveStateUnit>>> = vec![units1, units2, units3, units4, units5, units6, units7, units8];

    // thinking in my head for the multiple states implementation:
    // savestates can only be used in one level, until you exit out.
    // you'd still need validity checks on all states because they could have leftover data
    let mut save_validity = [false, false, false, false, false, false, false, false];
    let mut save_levels = [0, 0, 0, 0, 0, 0, 0, 0];

    //let mut save_level = 0; // saved level
    //let mut save_valid = false; // flag for if the state can be loaded
    let mut save_slot = 0;
    let mut prev_buttons = 0; // prev buttons presses
    let mut frame_opt = None;
    
    let mut prev_game_state = 0; // previous game state
    loop {
        let mut score = handle.read_u32(0x0174B050).unwrap();
        score = score - (score % 10) + (save_slot as u32 + 1);
        handle.write_u32(0x0174B050, score).unwrap();
        
        let buttons = handle.read_u32(0x01A52C4C).unwrap();
        let buttons_pressed = !prev_buttons & buttons;
        prev_buttons = buttons;

        let level = handle.read_u32(0x1934B70).unwrap();

        // compare the current game state with the previous
        // if the two have changed to the menu, invalid the savestates
        let game_state = handle.read_u32(0x1934BE0).unwrap();
        if prev_game_state != 0 && game_state == 0 {
            for i in 0..8 {
                save_validity[i] = false;
            }
            println!("Exited level. Invalidating savestates.");
        }
        prev_game_state = game_state;

        // incrementing and decrementing saveslots
        if buttons_pressed & 0x8 != 0 {
            save_slot = (save_slot + 1) % 8;
            println!("Savestate slot: {}", save_slot+1);
        }

        if buttons_pressed & 0x4 != 0 {
            save_slot = ((save_slot as u32).wrapping_sub(1) % 8) as usize;
            println!("Savestate slot: {}" , save_slot+1);
        }

        // save state
        if buttons_pressed & 0x1 != 0 {
            if game_state != 0 {
                save_levels[save_slot] = level;
                save_validity[save_slot] = true;
                for unit in units_vector[save_slot].iter_mut() {
                    match Rc::get_mut(unit).unwrap().save(&handle) {
                        Ok(()) => {}
                        Err(string) => println!("Error: {}", string),
                    }
                }
                println!("Saving state");
            } else {
                println!("Not in level. Cannot save state.")
            }
        }

        // dpad up = 0x8
        // dpad up + Y = 0x808
        // activates when holding y and pressing dpad-up
        if buttons_pressed & 0x8 != 0 && prev_buttons & 0x800 != 0 {
            if game_state != 0 {
                handle.write_u8(0x0174B024, 99).unwrap();
                println!("Set lives to 99.");
            }
        }

        // load state
        if buttons_pressed & 0x2 != 0 {
            if !save_validity[save_slot] {
                println!("Error: savestate not valid")
            } else if level != save_levels[save_slot] {
                println!("Error: not the same stage as savestate");
            } else {
                println!("Loading state");
                frame_opt = Some(handle.read_u32(0x0174b03c).unwrap());
                for unit in units_vector[save_slot].iter() {
                    match unit.load(&handle) {
                        Ok(()) => {}
                        Err(string) => println!("Error: {}", string),
                    }
                }
            }
        }

        // second-frame savestate load for collision stuff
        if let Some(frame) = frame_opt {
            if frame != handle.read_u32(0x0174b03c).unwrap() {
                for unit in units_vector[save_slot].iter() {
                    match unit.load(&handle) {
                        Ok(()) => {}
                        Err(string) => println!("Error: {}", string),
                    }
                }
                frame_opt = None;
            }
        }

        thread::sleep(Duration::from_millis(10))
    }
}
