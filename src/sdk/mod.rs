#![allow(dead_code)]
use once_cell::sync::OnceCell;

use crate::ue;

#[repr(C)]
pub struct UValeriaLocalPlayer {
    pub local_player_: ue::ULocalPlayer,
    game_login_options: ue::FString,
    on_selected_character_changed: [u8; 0x10],
    on_pawn_changed: [u8; 0x10],
    selected_character: [u8; 0x40],
    pad_308: [u8; 0x8]
}

impl UValeriaLocalPlayer {
    pub unsafe fn get_current_pawn(&self) -> *const ue::APawn {
        let g_objects = ue::G_OBJECTS.unwrap();
        static GET_CURRENT_PAWN: OnceCell<usize> = OnceCell::new();
        GET_CURRENT_PAWN.get_or_init(|| {
            (*g_objects).find_object("Function Palia.ValeriaLocalPlayer.GetCurrentPawn") as usize
        });

        #[repr(C)]
        struct Params {
            return_val: *const ue::APawn
        }

        let mut params = Params {
            return_val: std::ptr::null_mut()
        };

        let func = GET_CURRENT_PAWN.get().unwrap() as *const usize;
        self.local_player_.player_.object_.process_event(func, &mut params as *mut _ as * mut usize);
        params.return_val as *const ue::APawn
    }
}

#[repr(C)]
pub struct AVAL_PlayerControllerBase {
    pub player_controller_: ue::APlayerController,
    hud_class: *const ue::UHUD,    
    pad: [u8; 0x180]
}

#[repr(C)]
pub struct UValeriaBaseMovementComponent {
    pub pawn_movement_component_: ue::UPawnMovementComponent,
    moveable_owner: *const AValeriaMoveablePawn,
    follow_component: *const usize,
    gravity_scale: f32,
    max_step_height: f32,
    pub jump_z_velocity: f32,
    jump_off_jump_z_factor: f32,
    jump_capsule_half_height: f32,
    jump_capsule_radius: f32,
    walk_capsule_half_height: f32,
    walk_capsule_radius: f32,
    walkable_floor_angle: f32,
    walkable_floor_z: f32,
    movement_mode: [u8; 0x1],
    move_sim_flags: [u8; 0x1],
    pad_192: [u8; 0x6],
    target_facing: ue::FRotator,
    target_location1: ue::FVector,
    target_location2: ue:: FVector,
    target_location_leader: *const AValeriaMoveablePawn,
    leader_follow_distance: f32,
    leader_close_enough_range: f32,
    has_reached_target_location1: bool,
    pad_1f1: [u8; 0x3],
    teleport_direct_to_threshold: f32,
    use_facing_instead_in_sim_face_velocity_threshold: f32,
    should_use_forced_facing: bool,
    pad_1fd: [u8; 0x3],
    forced_facing: ue::FRotator,
    simple_path_target: ue::FVector2D,
    simple_path_direction: ue::FVector2D,
    simple_path_duration: f32,
    simple_path_guarantee_final_position: bool,
    pad_23d: [u8; 0x3],
    simple_path_guaranteed_final_location: ue::FVector,
    simple_path_guaranteed_final_rotation: ue::FRotator,
    custom_movement_mode: [u8; 0x1],
    network_smoothing_mode: [u8; 0x1],
    pad_272: [u8; 0x2],
    ground_friction: f32,
    pad_278: [u8; 0x40],
    speed_scalar_modifier_map: [u8; 0x50],
    speed_scalar: f32,
    pub max_walk_speed: f32,
    pub max_acceleration: f32,
    sim_speed_catch_up_scalar: f32,
    braking_friction_factor: f32,
    braking_friction: f32,
    braking_sub_step_time: f32,
    pub braking_deceleration_walking: f32,
    pub braking_deceleration_falling: f32,
    pub braking_deceleration_swimming: f32,
    pub braking_deceleration_flying: f32,
    pub air_control: f32,
    pub air_control_boost_multiplier: f32,
    pub air_control_boost_velocity_threshold: f32,
    falling_lateral_friction: f32,
    buoyancy: f32,
    perch_radius_threshold: f32,
    perch_additional_height: f32,
    rotation_rate: ue::FRotator,
    temp_override_rotation_rate: ue::FRotator,
    use_separate_braking_friction: [u8; 0x1],
    force_max_accel: [u8; 0x1],
    defer_update_move_component: [u8; 0x1],
    pad_383: [u8; 0x5],
    deferred_updated_move_component: *const usize,
    max_out_of_water_step_height: f32,
    out_of_water_z: f32,
    mass: f32,
    standing_downward_force_scale: f32,
    initial_push_force_factor: f32,
    push_force_factor: f32,
    push_force_point_z_offset_factor: f32,
    touch_force_factor: f32,
    min_touch_force: f32,
    max_touch_force: f32,
    repulsion_force: f32,
    pad_3bc: [u8; 0x4],
    acceleration: ue::FVector,
    pad_3d8: [u8; 0x8],
    last_update_rotation: [u8; 0x20],
    last_update_location: ue::FVector,
    last_update_velocity: ue::FVector,
    pending_impulse_to_apply: ue::FVector,
    pending_force_to_apply: ue::FVector,
    analog_input_modifier: f32,
    pad_464: [u8; 0xc],
    max_simulation_time_step: f32,
    max_simulation_iterations: i32,
    max_jump_apex_attempts_per_simulation: i32,
    max_depenetration_with_geometry: f32,
    max_depenetration_with_geometry_as_proxy: f32,
    max_depenetration_with_pawn: f32,
    max_depenetration_with_pawn_as_proxy: f32,
    net_proxy_shrink_radius: f32,
    net_proxy_shrink_half_height: f32,
    ledge_check_threshold: f32,
    jump_out_of_water_pitch: f32,
    pad_49c: [u8; 0x4],
    current_floor: [u8; 0xf0],
    default_land_movement_mode: [u8; 0x1],
    ground_movement_mode: [u8; 0x1],
    pad_592: [u8; 0x2],
    reduce_simulation_to_teleports_threshold: f32,
    reduce_simulation_to_one_step_threshold: f32,
    cached_distance_from_local_player_sq: f32,
    moved_by_sim_lod_tele: [u8; 0x1],
    pad_5a1: [u8; 0x3],
    quick_floor_adjust_trace_start_height: f32,
    quick_floor_adjust_trace_end_depth: f32,
    check_floor_period: f32,
    check_floor_timer: f32,
    pad_5b4: [u8; 0x4],
    cosmetic_avoidance_actors: [u8; 0x50],
    maintain_horizontal_ground_velocity: [u8; 0x1],
    notify_apex: [u8; 0x1],
    requested_move_use_acceleration: [u8; 0x1],
    pad_60b: [u8;0x5],
    requested_velocity: ue::FVector,
    pending_launch_velocity: ue::FVector,
    pad_640: [u8; 0x108],
    nav_mesh_projection_interval: f32,
    nav_mesh_projection_timer: f32,
    nav_mesh_projection_interp_speed: f32,
    nav_mesh_projection_height_scale_up: f32,
    nav_mesh_projection_height_scale_down: f32,
    nav_walking_floor_dist_tolerance: f32,
    on_teleport_to_resolve_path: [u8; 0x10],
    post_physics_tick_function: [u8; 0x30]
}

#[repr(C)]
pub struct UValeriaClientPriMovementComponent {
    pub valeria_base_movement_component_: UValeriaBaseMovementComponent,
    pad: [u8; 0x48]
}

#[repr(C)]
pub struct UValeriaCharacterMoveComponent {
    pub valeria_client_pri_movement_component_: UValeriaClientPriMovementComponent,
    on_gliding_changed: [u8; 0x10],
    on_climbing_changed: [u8; 0x10],
    on_is_climb_moving_changed: [u8; 0x10],
    on_long_falling_changed: [u8; 0x10],
    on_climb_dashing_changed: [u8; 0x10],
    on_climb_outro_started: [u8; 0x10],
    on_climb_around_edge_changed: [u8; 0x10],
    rotation_rate_with_speed: *const usize,
    sprint_acceleration_multiplier: f32,
    sprinting_stamina_drain_rate: f32,
    min_stamina_to_start_sprinting: f32,
    does_sprinting_cost_stamina: bool,
    pub walk_speed_multiplier: f32,
    pub walk_acceleration_multiplier: f32,
    sim_walk_threshold: f32,
    sim_sprint_threshold: f32,
    gliding_fall_speed: f32,
    gliding_falling_lateral_friction: f32,
    gliding_rotation_rate: ue::FRotator,
    pub gliding_max_speed: f32,
    climbing_maintain_reach: f32,
    climbing_around_surface_distance: f32,
    jump_up_ledge_max_height: f32,
    climb_intro_duration: f32,
    pub climb_intro_speed: f32,
    climbing_out_hold_duration: f32,
    pub climbing_speed: f32,
    climbing_step_duration: f32,
    climbing_sim_activation_factor: f32,
    climbing_around_edge_speed: f32,
    climbing_from_edge_speed: f32,
    climbing_dash_speed: f32,
    climbing_dash_duration: f32,
    climbing_dash_sim_activation_factor: f32,
    wall_jump_horizontal_speed: f32,
    wall_jump_vertical_speed: f32,
    pub climbing_stamina_drain_rate: f32,
    climbing_dash_stamina_cost: f32,
    pub climbing_jump_stamina_cost: f32,
    climbing_uneven_surface_budge_scalar: f32,
    climbing_surface_dot_product_range: ue::FVector2D,
    pad_90d: [u8; 0x3],
    climbing_engage_dot_product_req: f32,
    acceptable_climbing_surface_types: [u8; 0x10],
    climbability_collision_channel: [u8; 0x1],
    pad_921: [u8; 0x3],
    climbable_tag: ue::FName,
    not_climbable_tag: ue::FName,
    pad_934: [u8; 0x4],
    pub val_character: *const AValeriaCharacter,
    sprint_speed_multiplier: f32,
    pad_944: [u8; 0x4],
    gravity_scale_by_vertical_speed: *const usize,
    used_climbing_speed: f32,
    pad_954: [u8; 0x4],
    enforced_position: [u8; 0x38],
    client_driven_position: [u8; 0x38],
    should_do_falling_forward_checks_for_climbing: bool,
    climbing_client_driven_out: bool,
    pad_9ca: [u8; 0x6],
    canned_input: ue::FVector,
    canned_input_time_remaining: f32,
    jumping_is_enabled: bool,
    is_gliding: bool,
    pad_a21: [u8; 0xc],
    is_climb_dashing: bool,
    is_climbing_around_edge: bool,
    is_climb_moving: bool,
    pad_a31: [u8; 0x3],
    current_climb_moving_duration: f32,
    has_released_jump_since_climb_start: bool,
    pad_a39: [u8; 0x37],
    max_horizontal_climb_distance: f32,
    horizontal_climb_intro_distance: f32,
    climbing_surface_angle_tangent_x: f32,
    climbing_surface_angle_tangent_y: f32,
}

#[repr(C)]
pub struct FBagSlotLocation {
    bag_index: i32,
    slot_index: i32,
}

#[repr(C)]
pub struct UVillagerStoreComponent {
    actor_component_: ue::UActorComponent,
}

impl UVillagerStoreComponent {
    pub unsafe fn rpc_server_sell_item(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.VillagerStoreComponent.RpcServer_SellItem") as usize
        });
        
        #[repr(C)]
        struct Params {
            location: FBagSlotLocation,
            num_to_sell: i32,
        }

        let mut params = Params {
            location: FBagSlotLocation {
                bag_index: 0,
                slot_index: 0
            },
            num_to_sell: 1        
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct FValeriaItem {
	pub amount: i32,
    pad: [u8; 0x4a]
}

#[repr(C)]
pub struct UInventoryComponent {
    pub actor_component_: ue::UActorComponent,
    pad: [u8; 0x108]
}

impl UInventoryComponent {
    pub unsafe fn get_item_at(&self, bag_index: i32, slot_index: i32) -> FValeriaItem {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.InventoryComponent.GetItemAt") as usize
        });
        
        #[repr(C)]
        struct Params {
            location: FBagSlotLocation,
            return_val: FValeriaItem
        }

        let mut params = Params {
            location: FBagSlotLocation {
                bag_index,
                slot_index
            },
            return_val: FValeriaItem {
                amount: 0,
                pad: [0; 0x4a]
            }
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);
        params.return_val
    }
}

#[repr(C)]
pub struct UPlayerInventoryComponent {
    pub inventory_component_: UInventoryComponent,
    pad: [u8; 0x8],
    pub carried_items: ue::TArray<FValeriaItem>
}

#[repr(C)]
pub struct AValeriaCharacter {
    pub valeria_moveable_pawn_: AValeriaMoveablePawn,
    pad: [u8; 0xae0],
    pub villager_store_component: *const UVillagerStoreComponent,
    pad_1: [u8; 0x178]
}

impl AValeriaCharacter {
    pub unsafe fn get_movement_component(&self) -> *const UValeriaCharacterMoveComponent {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.ValeriaCharacter.GetValeriaCharacterMovementComponent") as usize
        });
        
        #[repr(C)]
        struct Params {
            return_val: *const UValeriaCharacterMoveComponent
        }

        let mut params = Params {
            return_val: std::ptr::null_mut()
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.valeria_moveable_pawn_.pawn_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
        params.return_val as *const UValeriaCharacterMoveComponent
    }

    pub unsafe fn get_fishing_component(&self) -> *const UFishingComponent {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.ValeriaCharacter.GetFishing") as usize
        });
        
        #[repr(C)]
        struct Params {
            return_val: *const UFishingComponent
        }

        let mut params = Params {
            return_val: std::ptr::null_mut()
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.valeria_moveable_pawn_.pawn_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
        params.return_val as *const UFishingComponent
    }

    pub unsafe fn get_inventory(&self) -> *const UPlayerInventoryComponent {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.ValeriaCharacter.GetInventory") as usize
        });
        
        #[repr(C)]
        struct Params {
            return_val: *const UPlayerInventoryComponent
        }

        let mut params = Params {
            return_val: std::ptr::null_mut()
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.valeria_moveable_pawn_.pawn_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
        params.return_val as *const UPlayerInventoryComponent
    }

    pub unsafe fn set_fishing_action(&self, is_acting: bool) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.ValeriaCharacter.SetFishingAction") as usize
        });
        
        #[repr(C)]
        struct Params {
            is_acting: bool
        }

        let mut params = Params {
            is_acting
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.valeria_moveable_pawn_.pawn_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn left_mouse_button_pressed(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.ValeriaCharacter.LeftMouseButtonPressed") as usize
        });
        
        #[repr(C)]
        struct Params {
        }

        let mut params = Params {
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.valeria_moveable_pawn_.pawn_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn left_mouse_button_released(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.ValeriaCharacter.LeftMouseButtonReleased") as usize
        });
        
        #[repr(C)]
        struct Params {
        }

        let mut params = Params {
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.valeria_moveable_pawn_.pawn_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }
}

#[repr(C)]
pub struct AValeriaMoveablePawn {
    pub pawn_: ue::APawn,
    pad: [u8; 0x142]
}

#[repr(C)]
pub struct AValeriaPlayerController {
    pub aval_player_controller_base_: AVAL_PlayerControllerBase,
    on_error_message_received: [u8; 0x10],
    on_error_message_text_received: [u8; 0x10],
    on_error_modal_received: [u8; 0x10],
    on_premium_store_entitlements_changed: [u8; 0x10],
    on_possession_changed: [u8; 0x10],
    on_character_was_spawned_event: [u8; 0x10],
    on_focus_upgraded_event: [u8; 0x10],
    on_housing_plot_visited: [u8; 0x10],
    on_simple_fade_requested: [u8; 0x10],
    on_character_teleported: [u8; 0x10],
    sync_time_interval: f32,
    player_snapshot_interval: f32,
    player_traversal_snapshot_inerval: f32,
    game_telemetry_snapshot_interval: f32,
    disconnect_with_message_timeout_hande: *const usize,
    exit_dialogue_teleport_travel_config_asset: [u8; 0x8],
    exit_dialogue_sequence: [u8; 0x30],
    unstuck_cooldown_timer_handle: [u8; 0x8],
    unstuck_last_usage_time: i64,
    unstuck_cooldown_seconds: f32,
    unstuck_use_factor_increase: f32,
    unstuck_on_cooldown_text: [u8; 0x18],
    return_home_cooldown_timer_handle: [u8; 0x8],
    return_home_last_usage_time: i64,
    return_home_cooldown_seconds: f32,
    pad_b0c: [u8; 0x4],
    return_home_destination_address: *const usize,
    pad_b18: [u8; 0x8],
    post_dialogue_chain_supplementary_info: [u8; 0x10],
    interactable_search_radius: f32,
    interactable_search_distance_ahead: f32,
    current_subgame_input_binding_handles: ue::TArray<i32>,
    on_client_leave_housing_plot: [u8; 0x10],
    on_client_arrive_on_housing_plot: [u8; 0x10],
    on_client_visitor_list_updated: [u8; 0x10],
    plot_presence: [u8; 0x10],
    on_reticle_needs_update: [u8; 0x10],
    premium_store_entitlements: [u8; 0x50],
    pub valeria_character: *const AValeriaCharacter,
    pad: [u8; 0x310]
}

impl AValeriaPlayerController {
    pub unsafe fn teleport_home(&self) {
        let g_objects = ue::G_OBJECTS.unwrap();
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            (*g_objects).find_object("Function Palia.ValeriaPlayerController.RpcServer_ForceReturnHome") as usize
        });
        
        #[repr(C)]
        struct Params {
        }
        
        let mut params = Params {
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.aval_player_controller_base_.player_controller_.a_controller_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn change_coints_cheat(&self) {
        let g_objects = ue::G_OBJECTS.unwrap();
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            (*g_objects).find_object("Function Palia.ValeriaPlayerController.ChangeCoinsCheat") as usize
        });
        
        #[repr(C)]
        struct Params {
            amount: i32,
        }
        
        let mut params = Params {
            amount: 50000
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.aval_player_controller_base_.player_controller_.a_controller_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn get_valeria_character(&self) -> *const AValeriaCharacter {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.ValeriaPlayerController.GetValeriaCharacter") as usize
        });
        
        #[repr(C)]
        struct Params {
            return_val: *const AValeriaCharacter
        }
        
        let mut params = Params {
            return_val: std::ptr::null()
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.aval_player_controller_base_.player_controller_.a_controller_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
        
        params.return_val
    }

    pub unsafe fn discard_item(&self, bag_index: i32, slot_index: i32) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.ValeriaPlayerController.DiscardItem") as usize
        });
        
        #[repr(C)]
        struct Params {
            location: FBagSlotLocation,
            amount: i32
        }
        
        let mut params = Params {
            location: FBagSlotLocation {
                bag_index,
                slot_index
            },
            amount: 1
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.aval_player_controller_base_.player_controller_.a_controller_.a_actor_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }
}

#[repr(C)]
pub struct BP_ValeriaPlayerController_C {
    pub valeria_player_controller_: AValeriaPlayerController,
    pad: [u8; 0x130]
}

#[repr(C)]
pub struct ABP_Loot_C {
    pub actor_: ue::AActor,
    uber_graph_frame: [u8; 0x8],
    ak_culled: *const usize,
    ns_idle: *const usize,
    static_mesh: *const usize,
    pub loot_interactor: *const ULootInteractorComponent,
    interactable: *const usize,
    proximity_checker: *const usize,
    interactable_collider: *const usize,
    sfx_pickup: *const usize,
    sfx_idle_loop: *const usize,
    pc: *const AValeriaCharacter,
}

#[repr(C)]
pub struct ULootInteractorBaseComponent {
    pub actor_component_: ue::UActorComponent,
    pad: [u8; 0x48]
}

impl ULootInteractorBaseComponent {
    pub unsafe fn server_trigger_gather(&self, character: *const AValeriaCharacter) -> bool {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.LootInteractorBaseComponent.Server_TriggerGather") as usize
        });
        
        #[repr(C)]
        struct Params {
            character: *const AValeriaCharacter,
            return_val: bool
        }
        
        let mut params = Params {
            character,
            return_val: false
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);

        params.return_val
    }
}

#[repr(C)]
pub struct ULootInteractorComponent {
    pub loot_interactor_base_component_: ULootInteractorBaseComponent,
    selected_loot: ue::TArray<[u8; 0x8]>,
    pad_f8: [u8; 0x8],
    players_to_grant_loot_to: ue::TArray<*const AValeriaCharacter>,
    pad_110: [u8; 0x30]
}

#[repr(C)]
pub struct UFishingComponent {
    pub actor_component_: ue::UActorComponent,
    pad_1: [u8; 0x1f0],
    pub fishing_hacks: *mut SomeFishingPointer
}

#[repr(C)]
pub struct SomeFishingPointer {
    pad: [u8; 0xa0],
    pub bobber_distance: f32,
    pad_1: [u8; 0x4],
    pub max_reel_distance: f32,
    pad_2: [u8; 0x4],
    pub time_for_emote_finish: f32,
    pad_3: [u8; 0x8],
    pub perfect_catch: f32,
    pub distance_fully_reeled: f32,
    pad_4: [u8; 0x4],
    pub wait_time_max: f32,
    pad_5: [u8; 0x14],
    pub wait_time_min: f32,
}

impl UFishingComponent {
    pub unsafe fn end_fishing(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.FishingComponent.RpcServer_EndFishing") as usize
        });
        
        #[repr(C)]
        struct Params {
            end_context: FFishingEndContext,
        }
        
        let mut params = Params {
            end_context: FFishingEndContext {
                result: EFishingMiniGameResult::Success,
                perfect: true,
                pad_2: [0; 2],
                durability_reduction: 0,
                source_water_body: std::ptr::null(),
                used_multiplayer_help: false,
                pad_11: [0; 3],
                start_rod_health: 100f32,
                end_rod_health: 100f32,
                start_fish_health: 100f32,
                end_fish_health: 100f32,
                pad_24: [0; 4]
            }
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn fish_caught(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.FishingComponent.RpcClient_FishCaught") as usize
        });
        
        #[repr(C)]
        struct Params {
            result: FFishCaughtResult,
        }
        
        let mut params = Params {
            result: FFishCaughtResult {
                was_perfect_catch: true,
                was_first_time_catch: false,
                item_category: 7,
                pad_3: [0; 5],
                fish_type: std::ptr::null(),
                fish_quality: 1i32,
                pad_14: [0; 4]
            }
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn start_fishing_at(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.FishingComponent.RpcClient_StartFishingAt") as usize
        });
        
        #[repr(C)]
        struct Params {
            fish_config_to_catch: ue::FName,
        }
        
        let mut params = Params {
            fish_config_to_catch: ue::FName {
                index: 5757421,
                number: 0
            }
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn override_next_fish(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.FishingComponent.RpcServer_CheatOverrideNextFishToCatch") as usize
        });
        
        #[repr(C)]
        struct Params {
            fish_config_to_catch: ue::FName,
        }
        
        let mut params = Params {
            fish_config_to_catch: ue::FName {
                index: 5827553,
                number: 0
            }
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn fish_started_biting(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.FishingComponent.RpcServer_FishStartedBiting") as usize
        });
        
        #[repr(C)]
        struct Params {
            water_body: *const usize,
        }
        
        let mut params = Params {
            water_body: 0x2A95F508800 as *const usize
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }

    pub unsafe fn exit_fishing_csm_state(&self) {
        static FUNC: OnceCell<usize> = OnceCell::new();
        FUNC.get_or_init(|| {
            let g_objects = ue::G_OBJECTS.unwrap();
            (*g_objects).find_object("Function Palia.FishingComponent.AnimDoneFishing") as usize
        });
        
        #[repr(C)]
        struct Params {
        }
        
        let mut params = Params {
        };

        let func = *FUNC.get().unwrap() as *const usize;
        self.actor_component_.object_.process_event(func, &mut params as *mut _ as * mut usize);
    }
}

#[repr(C)]
pub struct FFishingEndContext {
    result: EFishingMiniGameResult,
    pub perfect: bool,
    pub pad_2: [u8; 0x2],
    pub durability_reduction: i32,
    pub source_water_body: *const usize,
    pub used_multiplayer_help: bool,
    pub pad_11: [u8; 0x3],    
    pub start_rod_health: f32,
    pub end_rod_health: f32,
    pub start_fish_health: f32,
    pub end_fish_health: f32,
    pub pad_24: [u8; 0x4]
}

#[repr(C)]
pub struct FFishCaughtResult {
    was_perfect_catch: bool,
    was_first_time_catch: bool,
    item_category: u8,
    pub pad_3: [u8; 0x5],
    pub fish_type: *const usize,
    pub fish_quality: i32,
    pub pad_14: [u8; 0x4]
}

#[repr(u8)]
enum EFishingMiniGameResult {
    None,
    Success,
    Failure,
    Cancelled,
    CancelledCast,
    EmptyHanded,
    EFishingMiniGameResult_MAX
}

#[repr(C)]
struct UValeriaGameInstance {
    game_instance_: ue::UGameInstance,
    pad: [u8; 0x100],
    configs: FConfigsManager
}

#[repr(C)]
struct FConfigsManager {
    pad_1: [u8; 0x130],
    globals: FGlobalsConfig
}

#[repr(C)]
struct FGlobalsConfig {
    pad_1: [u8; 0x68],
    //fishing: *const UFishingGlobalConfig
}