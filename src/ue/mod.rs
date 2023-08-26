#![allow(dead_code)]
use crate::sdk;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl FVector {
    pub fn distance_to(&self, target: &FVector) -> f64 {
        let dx = target.x - self.x;
        let dy = target.y - self.y;
        let dz = target.z - self.z;
        
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct FVector2D {
    pub x: f64,
    pub y: f64
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct FRotator {
    pad: [u8; 0x18]
}

#[repr(C)]
pub struct FLinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub enum Color {
    Red,
    Green,
    Blue,
    Purple,
    Yellow,
    White
}

impl FLinearColor {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { a, b, g, r }
    }

    fn get_color(color: Color) -> FLinearColor {
        match color {
            Color::Red => FLinearColor::new(1f32, 0f32, 0f32, 1f32),
            Color::Green => FLinearColor::new(0f32, 1f32, 0f32, 1f32),
            Color::Blue => FLinearColor::new(0f32, 0f32, 1f32, 1f32),
            Color::Purple => FLinearColor::new(0.5f32, 0.07f32, 0.5f32, 1f32),
            Color::White => FLinearColor::new(1f32, 1f32, 1f32, 1f32),
            Color::Yellow => FLinearColor::new(1f32, 1f32, 0f32, 1f32),
        }
    }
}

#[repr(C)]
pub struct FNameEntryHandle {
    pub block: u32,
    pub offset: u32
}

impl FNameEntryHandle {
    fn new(block: u32, offset: u32) -> Self {
        Self {
            block,
            offset,
        }
    }

    fn index_to_handle(index: u32) -> FNameEntryHandle {
        let block = index >> 16;
        let offset = index & 65535;
        FNameEntryHandle::new(block, offset)
    }
}

#[repr(C)]
union NameUnion {
    ansi_name: [u8; 1024],
    wide_name: [u16; 1024]
}

#[repr(C)]
pub struct FNameEntry {
    flags: u16,
    name: NameUnion,
}

impl FNameEntry {
    fn is_wide(&self) -> bool {
        (self.flags & 0x1) != 0x0
    }

    fn len(&self) -> u16 {
        (self.flags >> 6) & 0x3FF
    }

    pub unsafe fn string(&self) -> String {
        if self.is_wide() {
            return String::new();
        }
        let name_bytes = &self.name.ansi_name[..self.len() as usize];
        String::from_utf8(name_bytes.to_vec()).unwrap_or(String::new())
    }
}

#[repr(C)]
pub struct FNamePool {
    pub lock: [u8; 8],
    pub current_block: u32,
    pub current_byte_cursor: u32,
    pub blocks: [*const u8; 8192],
}

impl FNamePool {
    pub unsafe fn get_entry(&self, handle: FNameEntryHandle) -> *const FNameEntry {
        let block_ptr = self.blocks[handle.block as usize];
        let offset = block_ptr as u64 + (2 * handle.offset as u64);
        let entry = offset as *const FNameEntry;
        
        entry
    }
}

#[repr(C)]
#[derive(PartialEq)]
pub struct FName {
    pub index: u32,
    pub number: u32
}

impl FName {
    pub unsafe fn get_name(&self) -> String {
        let g_names = G_NAMES.unwrap();
        let handle = FNameEntryHandle::index_to_handle(self.index); 
        let entry = (*g_names).get_entry(handle);

        let mut name = (*entry).string();
        if self.number > 0 {
            name.push_str(format!("_{}", self.number.to_string()).as_str());
        };

        if let Some(pos) = name.rfind('/') {
            name = name[(pos+1)..].to_string();
        };

        name
    }
}

#[repr(C)]
#[derive(PartialEq)]
pub struct UField {
    pub object_: UObject,
    pub pad_28: [u8; 0x8],
}

#[repr(C)]
#[derive(PartialEq)]
pub struct UStruct {
    pub field_: UField,
    pub pad_30: [u8; 0x10],
    pub super_struct: *const UStruct,
    pub pad_48: [u8; 0x68]
}

#[repr(C)]
#[derive(PartialEq)]
pub struct UClass {
    pub struct_: UStruct,
    pub pad_b0: [u8; 0x180]
}

#[repr(C)]
pub struct TUObjectArray {
    objects: *const *const u8,
    pre_allocated_objects: *const u8,
    max_elements: u32,
    pub num_elements: u32,
    max_chunks: u32,
    num_chunks: u32
}

impl TUObjectArray {
    pub unsafe fn find_object(&self, name: &str) -> *const UObject {
        for i in 0..self.num_elements {
            let object = self.get_object_by_index(i);
            if object.is_null() {
                continue;
            }

            let obj_name = (*object).get_full_name();
            if obj_name == name {
                return object as *const UObject;
            }
        }

        std::ptr::null()
    }

    pub unsafe fn get_object_by_index(&self, index: u32) -> *const UObject {
        if index >= self.num_elements {
            return std::ptr::null();
        }

        let chunk_index = index / 65536;
        if chunk_index >= self.num_chunks {
            return std::ptr::null();
        }

        let chunk = *self.objects.add(chunk_index as usize);
        if chunk.is_null() {
            return std::ptr::null()
        }

        let within_chunk_index = (index % 65536) * 24;
        let item_ptr = (chunk.add(within_chunk_index as usize)) as *const *const UObject;
        
        *item_ptr
    }

    pub fn len(&self) -> u32 {
        self.num_elements
    }
}

#[repr(C)]
pub struct TArray<T> {
    data: *const T,
    count: u32,
    max: u32
}

impl <T: Clone> TArray<T> {
    pub fn new() -> Self {
        Self {
            data: std::ptr::null(),
            count: 0u32,
            max: 0u32
        }
    }

    pub unsafe fn get(&self, index: u32) -> T {
        assert!(index < self.count, "Index out of bounds");
        let result = self.data.add(index as usize);
        (*result).clone()
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> TArrayIterator<T> {
        TArrayIterator {
            array: self,
            current_index: 0,
        }
    }
}

pub struct TArrayIterator<'a, T: Clone> {
    array: &'a TArray<T>,
    current_index: u32,
}

impl<'a, T: Clone> Iterator for TArrayIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.array.count {
            let element = unsafe { self.array.get(self.current_index) };
            self.current_index += 1;
            Some(element)
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct FString(TArray<u16>);

impl FString {
    pub fn new(string: &str) -> Self {
        let mut utf16: Vec<u16> = string.encode_utf16().collect();
        utf16.push(0); // Null-terminate the string

        let tarray = TArray {
            data: utf16.as_ptr(),
            count: utf16.len() as u32,
            max: utf16.capacity() as u32,
        };

        FString(tarray)
    }
}

pub static mut G_OBJECTS: Option<*const TUObjectArray> = None;
pub static mut G_WORLD: Option<*const *const UWorld> = None;
pub static mut G_NAMES: Option<*const FNamePool> = None;

#[repr(C)]
#[derive(PartialEq)]
pub struct UObject {
    pub vf_table: *const *const u64,
    pub object_flags: u32,
    pub internal_index: u32,
    pub class: *const UClass,
    pub name: FName,
    pub outer: *const UObject
}

impl UObject {
    pub unsafe fn get_name(&self) -> String {
        self.name.get_name()
    }

    pub unsafe fn get_full_name(&self) -> String {
        let mut name: String = String::new();        
        let mut outer = self.outer;

        while !outer.is_null() {
            let outer_name = (*outer).get_name();
            name = format!("{}.{}", outer_name, name);
            outer = (*outer).outer;
        }

        let obj_name = self.get_name();
        let class_name = (*self.class).struct_.field_.object_.get_name();
        name = format!("{} {}", class_name, name);
        name.push_str(&obj_name);

        name
    }

    pub unsafe fn is_a(&self, cmp: &UClass) -> bool {
        todo!()
    }

    pub unsafe fn process_event(&self, function: *const usize, params: *mut usize) {
        type VTableFn = extern "C" fn(*const UObject, *const usize, *const usize);
        let self_ptr = self as  *const _  as  *const *const VTableFn;
        let vtable = *self_ptr;
        let fn_call = *vtable.add(0x4c);
        fn_call(self, function, params);     
    }
}

use crate::{ue, utils};
use once_cell::sync::OnceCell;

#[repr(C)]
pub struct UConsole {
    pub object_: ue::UObject,
    pad_28: [u8; 0x10],
    console_target_player: *const ULocalPlayer,
    default_texture_black: *const u64,
    default_texture_white: *const u64,
    pad_50: [u8; 0x18],
    history_buffer: ue::TArray<ue::FString>,
    pad_78: [u8; 0xb8]
}

#[repr(C)]
pub struct UFont {
    pub object_: UObject,
    pad: [u8; 0x1a8]
}

#[repr(C)]
pub struct UCanvas {
    pub object_: ue::UObject,
    pad: [u8; 0x368]
}

impl UCanvas {
    pub unsafe fn k2_draw_text(&self, font: *const ue::UFont, screen: ue::FVector2D, text: &str, color: Color) {
        let g_objects = ue::G_OBJECTS.unwrap();
        static DRAW_TEXT: OnceCell<usize> = OnceCell::new();
        DRAW_TEXT.get_or_init(|| {
            (*g_objects).find_object("Function Engine.Canvas.K2_DrawText") as usize
        });
        let func = *DRAW_TEXT.get().unwrap() as *const usize;

        #[repr(C)]
        pub struct Params {
            render_font: *const ue::UFont,
            render_text: ue::FString,
            screen_position: ue::FVector2D,
            scale: ue::FVector2D,
            render_color: ue::FLinearColor,
            kerning: f64,
            shadow_color: ue::FLinearColor,
            shadow_offset: ue::FVector2D,
            b_centre_x: bool,
            b_centre_y: bool,
            b_outlined: bool,
            outline_color: ue::FLinearColor
        }

        let mut params = Params {
            render_font: font,
            render_text: ue::FString::new(text),
            screen_position: screen,
            scale: ue::FVector2D {
                x: 1f64,
                y: 1f64,
            },
            render_color: FLinearColor::get_color(color),
            kerning: 0f64,
            shadow_color: ue::FLinearColor {
                r: 0f32,
                g: 0f32,
                b: 0f32,
                a: 0f32
            },
            shadow_offset: ue::FVector2D {
                x: 1f64,
                y: 1f64,
            },
            b_centre_x: true,
            b_centre_y: true,
            b_outlined: true,
            outline_color: ue::FLinearColor {
                r: 0f32,
                g: 0f32,
                b: 0f32,
                a: 1f32
            },
        };

        self.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn k2_draw_line(&self, screen: ue::FVector2D) {
        let g_objects = ue::G_OBJECTS.unwrap();
        static DRAW_LINE: OnceCell<usize> = OnceCell::new();
        DRAW_LINE.get_or_init(|| {
            (*g_objects).find_object("Function Engine.Canvas.K2_DrawLine") as usize
        });

        #[repr(C)]
        pub struct Params {
            screen_position_a: ue::FVector2D,
            screen_position_b: ue::FVector2D,
            thickness: f64,
            render_color: ue::FLinearColor
        }

        let mut params = Params {
            screen_position_a: screen,
            screen_position_b: ue::FVector2D {
                x: screen.x + 20f64,
                y: screen.y + 20f64
            },
            thickness: 2f64,
            render_color: ue::FLinearColor {
                r: 1f32,
                g: 0.43f32,
                b: 0f32,
                a: 1f32
            },
        };

        let func = *DRAW_LINE.get().unwrap() as *const usize;
        self.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn k2_project(&self, world_location: ue::FVector) -> ue::FVector {
        let g_objects = ue::G_OBJECTS.unwrap();
        let func = (*g_objects).find_object("Function Engine.Canvas.K2_Project");

        #[repr(C)]
        pub struct Params {
            world_location: ue::FVector,
            return_val: ue::FVector
        }

        let mut params = Params {
            world_location,
            return_val: ue::FVector {
                x: 0f64,
                y: 0f64,
                z: 0f64
            }
        };

        self.object_.process_event(func as *const usize, &mut params as *mut _ as * mut usize);
        params.return_val
    }

    pub unsafe fn k2_draw_box(&self, screen_position: ue::FVector2D, color: Color) {
        static DRAW_BOX: OnceCell<usize> = OnceCell::new();
        DRAW_BOX.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Engine.Canvas.K2_DrawBox") as usize
        });

        #[repr(C)]
        struct Params {
            screen_position: ue::FVector2D,
            screen_size: ue::FVector2D,
            thickness: f64,
            render_color: ue::FLinearColor
        }

        let mut params = Params {
            screen_position,
            screen_size: ue::FVector2D {
                x: 15f64,
                y: 15f64
            },
            thickness: 1f64,
            render_color: FLinearColor::get_color(color),
        };

        let func = *DRAW_BOX.get().unwrap() as *const usize;
        self.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }
}

#[repr(C)]
pub struct UBlueprintFunctionLibrary {
    pub object_: ue::UObject
}

#[repr(C)]
pub struct UGameplayStatics {
    pub blueprint_function_library_: UBlueprintFunctionLibrary
}


#[repr(C)]
pub struct SpawnObjectParams {
    class: *const ue::UClass,
    outer: *const ue::UObject,
    return_val: *const ue::UObject
}

impl UGameplayStatics {
    pub unsafe fn spawn_object(&self, class: *const ue::UClass, outer: *const ue::UObject) -> *const ue::UObject {
        let g_objects = ue::G_OBJECTS.unwrap();
        let spawn_object = (*g_objects).find_object("Function Engine.GameplayStatics.SpawnObject");

        let mut params = SpawnObjectParams {
            class,
            outer,
            return_val: std::ptr::null_mut()
        };

        self.blueprint_function_library_.object_.process_event(spawn_object as *const usize, &mut params as *mut _ as * mut usize);
        params.return_val as *const ue::UObject
    }
}

#[repr(C)]
pub struct UScriptViewportClient {
    pub object_: ue::UObject,
    pad_28: [u8; 0x10]
}

#[repr(C)]
pub struct UGameViewportClient {
    pub script_viewport_client_: UScriptViewportClient,
    pad_38: [u8; 0x8],
    pub viewport_console: *const UConsole,
    pad_debug_properties: [u8; 0x10],
    pad_58: [u8; 0x10],
    max_splitscreen_players: i32,
    pad_6c: [u8; 0xc],
    world: *const UWorld,
    game_instance: *const UGameInstance,
    pad_88: [u8; 0x318]
}

#[repr(C)]
pub struct AActor {
    pub object_: ue::UObject,
    pad: [u8; 0x268]
}

static GET_ACTOR_LOCATION: OnceCell<usize> = OnceCell::new();
impl AActor {
    pub unsafe fn k2_get_actor_location(&self) -> ue::FVector {
        let g_objects = ue::G_OBJECTS.unwrap();
        GET_ACTOR_LOCATION.get_or_init(|| {
            (*g_objects).find_object("Function Engine.Actor.K2_GetActorLocation") as usize
        });

        #[repr(C)]
        struct Params {
            return_val: ue::FVector
        }

        let mut params = Params {
            return_val: ue::FVector {
                x: 0f64,
                y: 0f64,
                z: 0f64
            }
        };

        let func = *GET_ACTOR_LOCATION.get().unwrap() as *const usize;
        self.object_.process_event(func, &mut params as *mut _ as * mut usize);
        params.return_val as ue::FVector
    }
}

#[repr(C)]
pub struct AController {
    pub a_actor_: AActor,
    pad: [u8; 0x98]
}

static GET_PAWN: OnceCell<usize> = OnceCell::new();
impl AController {
    pub unsafe fn k2_get_pawn(&self) -> *const APawn {
        let g_objects = ue::G_OBJECTS.unwrap();
        GET_PAWN.get_or_init(|| {
            (*g_objects).find_object("Function Engine.Controller.K2_GetPawn") as usize
        });

        #[repr(C)]
        struct Params {
            return_val: *const APawn
        }

        let mut params = Params {
            return_val: std::ptr::null_mut()
        };

        let func = GET_PAWN.get().unwrap() as *const usize;
        self.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
        params.return_val as *const APawn
    }

    pub unsafe fn line_of_sight_to(&self, other: *const AActor) -> bool {
        let g_objects = ue::G_OBJECTS.unwrap();
        let func = (*g_objects).find_object("Function Engine.Controller.LineOfSightTo");

        #[repr(C)]
        struct Params {
            other: *const AActor,
            view_point: ue::FVector,
            alternate_checks: bool,
            return_val: bool
        }

        let mut params = Params {
            other,
            view_point: ue::FVector {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            alternate_checks: false,
            return_val: false
        };

        self.a_actor_.object_.process_event(func as *const usize, &mut params as *mut _ as * mut usize);
        params.return_val
    }
}

#[repr(C)]
pub struct APawn {
    pub a_actor_: AActor,
    pad: [u8; 0x88]    
}

#[repr(C)]
pub struct APlayerController {
    pub a_controller_: AController,
    pad_a77: [u8; 0x8],
    pub player: *const UPlayer,
    acknowledged_pawn: *const APawn,
    pub my_hud: *const UHUD,
    pub player_camera_manager: *const usize,
    pad: [u8; 0x500]
}

static GET_W2S: OnceCell<usize> = OnceCell::new();
impl APlayerController {
    pub unsafe fn project_world_location_to_screen(&self, world_location: &mut ue::FVector, screen_location: &mut ue::FVector2D, b_player_viewport_relative: bool) -> bool {
        let g_objects = ue::G_OBJECTS.unwrap();
        GET_W2S.get_or_init(|| {
            (*g_objects).find_object("Function Engine.PlayerController.ProjectWorldLocationToScreen") as usize
        });

        #[repr(C)]
        struct Params {
            world_location: ue::FVector,
            screen_location: ue::FVector2D,
            b_player_viewport_relative: bool,
            return_val: bool
        }

        let mut params = Params {
            world_location: *world_location,
            screen_location: *screen_location,
            b_player_viewport_relative,
            return_val: false,
        };

        let func = *GET_W2S.get().unwrap() as *const usize;
        self.a_controller_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
        *screen_location = params.screen_location;

        params.return_val
    }
}

#[repr(C)]
pub struct UHUD {
    pub actor_: AActor,
    pub player_owner: *const APlayerController,
    pad_298: [u8; 0x1],
    pad_299: [u8; 0x3],
    current_targetindex: i32,
    pad_2a0: [u8; 0x1],
    pad_2a1: [u8; 0x7],
    post_rendered_actors: ue::TArray<*const AActor>,
    pad_2b8: [u8; 0x8],
    debug_display: ue::TArray<ue::FName>,
    toggled_debug_categories: ue::TArray<ue::FName>,
    pub canvas: *const UCanvas,
    debug_canvas: *const UCanvas,
    debug_text_list: [u8; 0x10],
    show_debug_target_desired_class: *const AActor,
    show_debug_target_actor: *const AActor,
    pad_310: [u8; 0x70],
}

#[repr(C)]
pub struct UPlayer {
    pub object_: ue::UObject,
    pad_28: [u8; 0x8],
    pub player_controller: *const APlayerController,
    current_net_speed: i32,
    configured_internet_speed: i32,
    configured_lan_speed: i32,
    pad_44: [u8; 0x4]
}

#[repr(C)]
pub struct ULocalPlayer {
    pub player_: UPlayer,
    pad_48: [u8; 0x30],
    pub viewport_client: *const UGameViewportClient,
    pad_80: [u8; 0x38],
    aspect_ratio_axis_constraint: [u8; 0x1],
    pad_b9: [u8; 0x7],
    pending_level_player_controller_class: *const u64,
    sent_split_join: [u8; 0x1],
    pad_c9: [u8; 0x17],
    controller_id: i32,
    pad_e4: [u8; 0x1b4]
}

#[repr(C)]
pub struct UGameInstance {
    pub object_: ue::UObject,
    pad_28: [u8; 0x10],
    pub local_players: ue::TArray<*const ULocalPlayer>,
    online_session: *const u64,
    referenced_objects: ue::TArray<*const ue::UObject>,
    pad_60: [u8; 0x18],
    pad_on_pawn_controller_changed_delegates: [u8; 0x10],
    pad_88: [u8; 0x18],
    pad_on_input_device_connection_change: [u8; 0x10],
    pad_on_user_input_device_pairing_change: [u8; 0x10],
    pad_c0: [u8; 0x100]
}


#[repr(C)]
pub struct AInfo {
    pub a_actor_: AActor,
}

#[repr(C)]
pub struct APlayerState {
    pub a_info_: AInfo,
    pub score: f32,
    pub player_id: i32,
    compressed_ping: [u8; 1],
    pad_299: [u8; 1],
    pad_29a: [u8; 1],
    pad_29b: [u8; 1],
    start_time: i32,
    engine_message_class: *const usize,
    pad_2a8: [u8; 8],
    saved_network_address: ue::FString,
    pub unique_id: [u8; 0x30],
    pad_2f0: [u8; 8],
    on_pawn_set: [u8; 0x10],
    pub pawn_private: *const APawn,
    pad_310: [u8; 0x78],
    pub player_name_private: ue::FString,
    pad_398: [u8; 0x10]
}

#[repr(C)]
pub struct AGameStateBase {
    pub a_info_: AInfo,
    pad1: [u8; 8],
    pad2: [u8; 8],
    pad3: [u8; 8],
    pub player_array: ue::TArray<*const APlayerState>,
    pad: [u8; 0x22]
}

#[repr(C)]
pub struct AGameState {
    pub a_game_state_base_: AGameStateBase,    
    match_state: ue::FName,
    previous_match_state: ue::FName,
    elapsed_time: i32,
    pad_2f4: [u8; 0xc],
}

#[repr(C)]
pub struct ULevelActorContainer {
    pub object_: UObject,
    pub actors: TArray<*const AActor>
}

#[repr(C)]
pub struct ULevel {
    pub object_: UObject,
    pub pad_28: [u8; 0x70],
    pub actors: TArray<*const AActor>,
    pub garbage_actors: TArray<*const AActor>,
    pub owning_world: *const UWorld,
    pub model: *const usize,
    pub model_components: TArray<*const usize>,
    pad: [u8; 0x240]
}

#[repr(C)]
pub struct UWorld {
    pub object_: ue::UObject,
    pad_28: [u8; 0x8],
    pub persistent_level: *const ULevel,
    net_driver: *const u64,
    line_batcher: *const u64,
    persistent_line_batcher: *const u64,
    foreground_line_batcher: *const u64,
    network_manager: *const u64,
    u_physics_collision_handler: *const u64,
    extra_referenced_objects: ue::TArray<*const ue::UObject>,
    per_module_data_objects: ue::TArray<*const ue::UObject>,
    streaming_levels: ue::TArray<*const u64>,
    pad_streaming_levels_to_consider: [u8; 0x28],
    server_streaming_levels_visibility: *const u64,
    streaming_levels_prefix: ue::FString,
    pad_d8: [u8; 0x8],
    current_level_pending_visibility: *const u64,
    current_level_pending_invisibility: *const u64,
    demo_net_driver: *const u64,
    my_particle_event_manager: *const u64,
    default_physics_volume: *const u64,
    pad_108: [u8; 0x36],
    pad_13e_0: [u8; 0x1],
    pad_13f: [u8; 0x9],
    navigation_system: *const u64,
    authority_game_mode: *const u64,
    pub game_state: *const AGameState,
    ai_system: *const u64,
    avoidance_manager: *const u64,
    pub levels: ue::TArray<*const ULevel>,
    pad_level_collections: [u8; 0x10],
    pad_190: [u8; 0x28],
    pub owning_game_instance: *const UGameInstance,
    parameter_collection_instances: ue::TArray<*const u64>,
    canvas_for_rendering_to_target: *const u64,
    canvas_for_draw_material_to_render_target: *const u64,
    pad_1e0: [u8; 0x70],
    physics_field: *const u64,
    lwi_last_assigned_uid: u32,
    pad_25c: [u8; 0x4],
    pad_components_that_need_pre_end_of_frame_sync: [u8; 0x50],
    components_that_need_end_of_frame_update: ue::TArray<*const u64>,
    components_that_need_end_of_frame_update_on_game_thread: ue::TArray<*const u64>,
    pad_2d0: [u8; 0x3f8],
    world_composition: *const u64,
    content_bundle_manager: *const u64,
    pad_6d8: [u8; 0xa8],
    pad_psc_pool: [u8; 0x58],
    pad_7d8: [u8; 0xc0]
}

#[repr(C)]
pub struct UActorComponent {
    pub object_: UObject,
    pad: [u8; 0x78]
}

#[repr(C)]
pub struct UMovementComponent {
    pub actor_component_: UActorComponent,
    pad: [u8; 0x68]
}

#[repr(C)]
pub struct UNavMovementComponent {
    pub movement_component_: UMovementComponent,
    pad: [u8; 0x48]
}

#[repr(C)]
pub struct UPawnMovementComponent {
    pub nav_movement_component: UNavMovementComponent,
    pad: [u8; 0x8]
}