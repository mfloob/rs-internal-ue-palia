
use once_cell::sync::Lazy;
use retour::static_detour;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_INSERT, VK_DELETE, VK_END, VK_HOME, VK_F12};

mod utils;
mod sdk;
mod ue;

#[no_mangle]
extern "stdcall" fn DllMain(hinst: usize, reason: u32) -> i32 {
    if reason == 1 {
        std::thread::spawn(move || unsafe { main_thread(hinst) });
    }
    
    if reason == 0 {
        unsafe {
            unhook_all();
            utils::free_console();
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    }

    1
}

type FnProcessEvent2 = unsafe extern "fastcall" fn(
    a1: *const ue::UObject,
    a2: *const ue::UObject,
    params: *mut usize);
static_detour! {
    static ProcessEvent2: unsafe extern "fastcall" fn(
        *const ue::UObject,
        *const ue::UObject,
        *mut usize);
}
fn hk_process_event2(
    a1: *const ue::UObject,
    a2: *const ue::UObject,
    params: *mut usize) {

    unsafe {
        let name = (*a2).get_full_name();
        // if name.contains("FishingComponent") && !name.contains("ViewState") {
        //     println!("{name}");
        // }

        if name == "Function Palia.FishingComponent.RpcClient_StartFishingAt" {
            #[repr(C)]
            struct Params {
                fish_config_to_catch: ue::FName,
            }
            let param = params as *mut Params;
            let name = (*param).fish_config_to_catch.get_name();
            let should_sell = name.contains("Fish");

            let world = &*ue::G_WORLD.unwrap();
            let game = (*(*world)).owning_game_instance;
            let players = &(*game).local_players;
            let localplayer = players.get(0) as *const sdk::UValeriaLocalPlayer;
            let local_controller = (*localplayer).local_player_.player_.player_controller as *const sdk::BP_ValeriaPlayerController_C;
            let character = (*local_controller).valeria_player_controller_.get_valeria_character();
            let fishing_component = (*character).get_fishing_component();
            let store = (*character).villager_store_component;

            ProcessEvent2.call(a1, a2, params);
            (*fishing_component).end_fishing();
            
            match should_sell {
                true => (*store).rpc_server_sell_item(),
                false =>(*local_controller).valeria_player_controller_.discard_item(0, 0)
            }

            (*character).set_fishing_action(true);
            (*character).left_mouse_button_pressed();
            (*character).left_mouse_button_released();
            FISHING_TIMER = Lazy::new(std::time::Instant::now);
            CAN_FISH = true;

            return;
        }

        ProcessEvent2.call(a1, a2, params);
    }
}

type FnProcessEvent = unsafe extern "fastcall" fn(
    a1: *const ue::UObject,
    a2: *const ue::UObject,
    params: *mut usize);
static_detour! {
    static ProcessEvent: unsafe extern "fastcall" fn(
        *const ue::UObject,
        *const ue::UObject,
        *mut usize);
}
fn hk_process_event(
    a1: *const ue::UObject,
    a2: *const ue::UObject,
    params: *mut usize) {

    unsafe {
        let name = (*a2).get_full_name();
        if name != "Function Engine.HUD.ReceiveDrawHUD" {
            ProcessEvent.call(a1, a2, params);
            return;
        }

        on_receive_draw_hud(a1 as *const ue::UHUD);
        
        ProcessEvent.call(a1, a2, params);
    }
}

unsafe fn hook_process_event(object: &ue::UObject) -> bool {
    let vftable = object.vf_table;
    let addr = *vftable.add(0x4c);

    let fn_process_event: FnProcessEvent = std::mem::transmute(addr as *const usize);
    ProcessEvent
        .initialize(fn_process_event, hk_process_event)
        .unwrap()
        .enable()
        .unwrap();

    fn_process_event as u64 > 0
}

static mut FISHING_TIMER: Lazy<std::time::Instant> = Lazy::new(std::time::Instant::now);
static mut CAN_FISH: bool = false;
unsafe fn on_receive_draw_hud(hud: *const ue::UHUD) {
    let canvas = (*hud).canvas;
    if canvas as u64 == 0 {
        return;
    }

    let world = &*ue::G_WORLD.unwrap();
    let game = (*(*world)).owning_game_instance;
    let players = &(*game).local_players;
    let localplayer = players.get(0) as *const sdk::UValeriaLocalPlayer;
    let local_controller = (*localplayer).local_player_.player_.player_controller as *const sdk::BP_ValeriaPlayerController_C;

    let class = (*local_controller).valeria_player_controller_.aval_player_controller_base_.player_controller_.a_controller_.a_actor_.object_.get_name();
    if class.contains("BP_PregamePlayerController_C") {
        return;
    }        
    
    let character = (*local_controller).valeria_player_controller_.get_valeria_character();
    if character as u64 == 0 {
        return;
    }

    if utils::key_released(VK_DELETE.0) {
        let store = (*character).villager_store_component;
        (*store).rpc_server_sell_item();
    }

    if utils::key_released(VK_HOME.0) {
        (*local_controller).valeria_player_controller_.teleport_home();
    }

    if utils::key_released(VK_INSERT.0) {
        let movement_component = (*character).get_movement_component() as *mut sdk::UValeriaCharacterMoveComponent;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.max_walk_speed = 5000f32;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.max_acceleration = 50000f32;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.jump_z_velocity = 4000f32;
        (*movement_component).walk_speed_multiplier = 5f32;
        (*movement_component).walk_acceleration_multiplier = 50f32;
        (*movement_component).climbing_speed = 800f32;
        (*movement_component).climb_intro_speed = 800f32;
    
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.air_control = 50f32;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.air_control_boost_multiplier = 100f32;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.air_control_boost_velocity_threshold = 1000f32;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.braking_deceleration_falling = 10000f32;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.braking_deceleration_flying = 10000f32;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.braking_deceleration_swimming = 10000f32;
        (*movement_component).valeria_client_pri_movement_component_.valeria_base_movement_component_.braking_deceleration_walking = 10000f32;
    }

    static mut ACTIVATE_FISHING: bool = false;
    if utils::key_released(VK_F12.0) {
        FISHING_TIMER = Lazy::new(std::time::Instant::now);
        CAN_FISH = true;
        ACTIVATE_FISHING = !ACTIVATE_FISHING;        
    }
    
    let fishing_component = (*character).get_fishing_component();
    let mut fishing_hacks = (*fishing_component).fishing_hacks;
    if ACTIVATE_FISHING && fishing_hacks as u64 > 0 {
        if !ProcessEvent2.is_enabled() {
            let object = (*character).get_fishing_component() as *const ue::UObject;
            let vftable = (*object).vf_table;
            let addr = *vftable.add(0x4c);

            let fn_process_event2: FnProcessEvent2 = std::mem::transmute(addr as *const usize);
            ProcessEvent2
                .initialize(fn_process_event2, hk_process_event2)
                .unwrap()
                .enable()
                .unwrap();
        }

        let now = std::time::Instant::now();
        let time_since_caught = now.duration_since(*FISHING_TIMER).as_millis();

        if CAN_FISH && (*fishing_hacks).max_reel_distance == 0f32 && time_since_caught > 500u128 {
            (*character).left_mouse_button_pressed();
            (*character).set_fishing_action(false);
            CAN_FISH = false;
        }
        
        let max_dist = 0.5f32;
        if (*fishing_hacks).max_reel_distance > 0f32 && (*fishing_hacks).max_reel_distance <= max_dist {
            (*fishing_hacks).max_reel_distance = max_dist;
        }
        else if (*fishing_hacks).max_reel_distance >= max_dist + 0.15f32 {
            (*fishing_hacks).max_reel_distance = 1f32;
        }

        if (*fishing_hacks).max_reel_distance >= max_dist {
            (*character).left_mouse_button_released();
        }

        if time_since_caught > 1700u128 {
            (*character).set_fishing_action(false);
            (*character).set_fishing_action(true);
            (*character).left_mouse_button_pressed();
            (*character).left_mouse_button_released();
            (*fishing_hacks).max_reel_distance = 1f32;
            FISHING_TIMER = Lazy::new(std::time::Instant::now);
            CAN_FISH = true;
        }

        (*fishing_hacks).time_for_emote_finish = 1f32;
    }

    let my_location = (*character).valeria_moveable_pawn_.pawn_.a_actor_.k2_get_actor_location();
    let persistent_level = (*(*world)).persistent_level;
    for actor in (*persistent_level).actors.iter() {
        if actor as u64 == 0 || actor as u64 == character as u64 {
            continue;
        }
        
        let name = (*actor).object_.get_name();
        if name.contains("ValeriaCharacter") {
            let mut screen = ue::FVector2D{x:0f64, y:0f64};
            let mut location = (*actor).k2_get_actor_location();
            if (*local_controller).valeria_player_controller_.aval_player_controller_base_.player_controller_.project_world_location_to_screen(&mut location, &mut screen, true) {
                (*canvas).k2_draw_box(screen, ue::Color::Purple);
                continue;
            }
        }
        
        let mut location = (*actor).k2_get_actor_location();
        let distance = my_location.distance_to(&location) / 100f64;
        if distance > 200f64 {
            continue;
        }        

        let include_list: Vec<&str> = vec![
            "_Tree",
            "_Bug",
            "_Mining",
            "Choppable",
            "ValeriaHuntingCreature",
            "_Spiced",
            "_Loot"
        ];

        if !include_list.iter().any(|s| name.contains(s)) {
            continue;
        }

        let mut color = ue::Color::White;
        if name.contains("_Tree") {
            color = ue::Color::Green;
        }

        if name.contains("_Loot") {
            color = ue::Color::Purple;
        }

        if name.contains("_Mining") {
            color = ue::Color::Yellow;
        }

        let mut screen = ue::FVector2D{x:0f64, y:0f64};
        if !(*local_controller).valeria_player_controller_.aval_player_controller_base_.player_controller_.project_world_location_to_screen(&mut location, &mut screen, true) {
            continue;
        }

        let font = utils::get_font();
        let text = format!("{} [{:.2}m]", name, distance);
        (*canvas).k2_draw_text(font, screen, text.as_str(), color);
    }
}

type FnPostRender = unsafe extern "fastcall" fn(
    viewport: *const ue::UGameViewportClient,
    canvas: *const ue::UCanvas);
static_detour! {
    static PostRender: unsafe extern "fastcall" fn(
        *const ue::UGameViewportClient,
        *const ue::UCanvas);
}
fn hk_post_render(
    viewport: *const ue::UGameViewportClient,
    canvas: *const ue::UCanvas) {

    unsafe {
        let world = &*ue::G_WORLD.unwrap();
        let game = (*(*world)).owning_game_instance;
        let players = &(*game).local_players;
        let localplayer = players.get(0);
        let local_controller = (*localplayer).player_.player_controller;
        
        // let local_pawn = (*local_controller).a_controller_.k2_get_pawn();
        // println!("{}", local_pawn as usize);
        // if local_pawn as usize == 0 {
        //     PostRender.call(viewport, canvas);
        //     return;
        // }

        // let mut location = (*local_pawn).a_actor_.k2_get_actor_location();
        // println!("{} {} {}", location.x, location.y, location.z);
        // let mut screen = ue::FVector2D::default();
        // if (*local_controller).project_world_location_to_screen(&mut location, &mut screen, true) {
        //     if screen.x > 0f64 && screen.y > 0f64 {
        //         println!("{} {}", screen.x, screen.y);
        //         (*canvas).k2_draw_box(screen);
        //     }
        // }

        // let state = (*(*world)).game_state;
        // if state as u64 == 0 {
        //     PostRender.call(viewport, canvas);
        //     return;
        // }

        // let players = &(*state).a_game_state_base_.player_array;
        // for player in players.iter() {
        //     let pawn = &(*player).pawn_private;
        //     if *pawn as u64 == 0 { // || *pawn as u64 == local_pawn as u64
        //         continue;
        //     }
            
        //     let mut location = (*(*pawn)).a_actor_.k2_get_actor_location();
        //     let mut screen = ue::FVector2D::default();
        //     if (*local_controller).project_world_location_to_screen(&mut location, &mut screen, true) {
        //         if screen.x > 0f64 && screen.y > 0f64 {
        //             //(*canvas).k2_draw_box(screen);
        //         }
        //     }
        // }

        PostRender.call(viewport, canvas);
    }
}

unsafe fn hook_post_render(object: &ue::UObject) -> bool {
    let vftable = object.vf_table;
    let addr = *vftable.add(0x6e);

    let fn_post_render: FnPostRender = std::mem::transmute(addr as *const usize);
    PostRender
        .initialize(fn_post_render, hk_post_render)
        .unwrap()
        .enable()
        .unwrap();

    fn_post_render as u64 > 0
}


unsafe fn unhook_all() {
    ProcessEvent.disable().unwrap();
    if ProcessEvent2.is_enabled() {
        ProcessEvent2.disable().unwrap();
    }
}

unsafe fn on_loop() {
    if utils::key_released(VK_END.0) {
        utils::unload();
    }
}

static mut EXITING: bool = false;
#[allow(unused_assignments)]
unsafe fn main_thread(_hinst: usize) {
    utils::alloc_console();

    ue::G_OBJECTS = Some(utils::get_g_objects());
    ue::G_NAMES = Some(utils::get_g_names());
    ue::G_WORLD = Some(utils::get_g_world());
    
    let world = &*ue::G_WORLD.unwrap();
    let g_objects = &*ue::G_OBJECTS.unwrap();

    while *world as u64 == 0x0 || (*g_objects).len() < 1 {
        std::thread::sleep(std::time::Duration::from_millis(100));        
    }

    println!("\n[-] world: {:X}", *world as u64);
    let game = (*(*world)).owning_game_instance;
    println!("[-] game: {:X}", game as u64);
    let players = &(*game).local_players;
    println!("[-] localplayers count: {:X}", players.len());
    let localplayer = players.get(0);
    println!("[-] localplayer: {:X}", localplayer as u64);
    let mut viewport = (*localplayer).viewport_client as *mut ue::UGameViewportClient;
    println!("[-] viewport: {:X}", viewport as u64);
    let local_controller = (*localplayer).player_.player_controller;
    
    let mut console_class: *const ue::UClass = std::ptr::null_mut();
    let mut gameplay_statics: *const ue::UGameplayStatics = std::ptr::null_mut();
    loop {
        console_class = g_objects.find_object("Class Engine.Console") as *const ue::UClass;
        gameplay_statics = g_objects.find_object("Class Engine.GameplayStatics") as *const ue::UGameplayStatics;
        if !console_class.is_null() && !gameplay_statics.is_null() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let console = (*gameplay_statics).spawn_object(console_class, &(*viewport).script_viewport_client_.object_ as *const _ as *const ue::UObject);
    (*viewport).viewport_console = console as *const ue::UConsole;

    let hud = (*local_controller).my_hud;
    hook_process_event(&(*hud).actor_.object_);    

    println!("[-] successfully initiated\n");

    #[allow(clippy::empty_loop)]
    while !EXITING {
        on_loop();
    }
}