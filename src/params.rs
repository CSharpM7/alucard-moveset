use smash::params::*;
use super::*;

fn params_main(params_info: &ParamsInfo<'_>) {
    
    if let Ok(fighter_param) = params_info.get::<FighterParams>() {
        let alucard_params = &mut fighter_param[*FIGHTER_KIND_RICHTER];
        alucard_params.walk_speed_max = 30.0;
        alucard_params.run_speed_max = 30.0;
        println!("Yo?");
        println!("Yo?");
    }
}
static INT_OFFSET: usize = 0x4e5380; // 12.0.0
static FLOAT_OFFSET: usize = 0x4e53C0; // 12.0.0

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn int_param_accessor_hook(
boma: u64,
param_type: u64,
param_hash: u64) -> i32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = module_accessor.kind();

    let toReturn = original!()(boma, param_type, param_hash);
    if module_accessor.is_fighter() {
        if fighter_kind == FIGHTER_KIND_RICHTER {
            if param_hash == 0 {
                if param_type == hash40("attack_combo_max") {return 3;}
                else if param_type == hash40("attack_100_rebound_count") {return 2;}
            }
        }

    }
    return toReturn;
}
#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn float_param_accessor_hook(
boma: u64,
param_type: u64,
param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = module_accessor.kind();

    let toReturn = original!()(boma, param_type, param_hash);
    if module_accessor.is_fighter() {
        if fighter_kind == FIGHTER_KIND_RICHTER {
            if param_hash == 0 {
                
                if param_type == hash40("run_speed_max") {return 30.0;}
                else if param_type == hash40("attack_100_rebound_distance") {return 50.0;}
                else if param_type == hash40("attack_100_rebound_distance") {return 50.0;}
                else if param_type == hash40("walk_accel_mul") {return 0.3;}
                else if param_type == hash40("walk_accel_add") {return 0.1;}
                else if param_type == hash40("walk_speed_max") {return 0.8;}
                else if param_type == hash40("run_speed_max") {return 1.7;}
                else if param_type == hash40("dash_speed") {return 1.9;}

                else if param_type == hash40("jump_speed_x") {return 0.8;}
                else if param_type == hash40("jump_speed_x_mul") {return 0.8;}
                else if param_type == hash40("jump_speed_x_max") {return 1.1;}
                else if param_type == hash40("jump_aerial_speed_x_mul") {return 0.8;}

                else if param_type == hash40("jump_initial_y") {return 17.0;}
                else if param_type == hash40("jump_y") {return 31.0;}
                else if param_type == hash40("mini_jump_y") {return 14.0;}
                else if param_type == hash40("jump_aerial_y") {return 34.0;}

                else if param_type == hash40("air_accel_x_mul") {return 0.05;}
                else if param_type == hash40("air_accel_x_add") {return 0.01;}
                else if param_type == hash40("air_speed_x_stable") {return 1.01;}
                else if param_type == hash40("air_brake_x") {return 0.02;}

                else if param_type == hash40("air_accel_y") {return 0.085;}
                else if param_type == hash40("air_speed_y_stable") {return 1.625;}
                else if param_type == hash40("air_brake_y") {return 0.01;}
                else if param_type == hash40("dive_speed_y") {return 2.75;}

                else if param_type == hash40("weight") {return 80.0;}
                else if param_type == hash40("landing_attack_air_frame_n") {return 7.0;}
                else if param_type == hash40("landing_attack_air_frame_f") {return 10.0;}
                else if param_type == hash40("landing_attack_air_frame_b") {return 9.0;}
                else if param_type == hash40("landing_attack_air_frame_hi") {return 12.0;}
                else if param_type == hash40("landing_attack_air_frame_lw") {return 26.0;}

                else if param_type == hash40("scale") {return 1.03;}	  
                else if param_type == hash40("shield_radius") {return 12.2;}	 

                else if param_type == hash40("height") {return 18.0;}
                else if param_type == hash40("expand_height") {return 22.0;}

                else if param_type == hash40("passive_wall_jump_y_speed") {return 2.4;}
            }
        }

    }
    return toReturn;
}

const MAX_FILE_SIZE: usize = 0xFFFF;
// in main
pub fn install() {
    //smash::params::add_hook(params_main).unwrap();
	//skyline::install_hook!(int_param_accessor_hook);
	//skyline::install_hook!(float_param_accessor_hook);
}