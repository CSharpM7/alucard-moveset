use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

static mut META_HEALED:[bool;8] = [false; 8];
static mut META_WHIFF:[bool;8] = [false; 8];
static mut META_FRAME:[i32;8] = [0; 8];
static mut META_EFFECT:[i32;8] = [-1; 8];
const META_MAX: i32 = 1800;
const META_PUNISH: i32 = 60;

static mut BAT_INPUT_X:[f32;8] = [0.0; 8];
static mut BAT_INPUT_Y:[f32;8] = [0.0; 8];

unsafe fn metamorphosis_check_heal(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,entry: usize){
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut swordAttack = [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].contains(&status);
    if (!swordAttack)
    {
        let motion = MotionModule::motion_kind(boma);
        if (status == *FIGHTER_STATUS_KIND_ATTACK_AIR)
        {
            swordAttack = fighter.is_motion(Hash40::new("attack_air_b"))
            || fighter.is_motion(Hash40::new("attack_air_f"))
            || fighter.is_motion(Hash40::new("attack_air_f_hi"))
            || fighter.is_motion(Hash40::new("attack_air_f_lw"));
        }
        else
        {
            swordAttack = fighter.is_motion(Hash40::new("attack_13")) ||
            fighter.is_motion(Hash40::new("attack_100_end"));
        }
        
    }
    if (!swordAttack)
    {
        if (META_WHIFF[entry] && !META_HEALED[entry])
        {
            let mut frameloss = META_PUNISH;
            META_FRAME[entry]=0.max(META_FRAME[entry]-frameloss);
            EffectModule::req_follow(boma, Hash40::new("sys_hit_curse"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 1.25, true, 0, 0, 0, 0, 0, false, false);
        }
        META_HEALED[entry]=false;
    }
    META_WHIFF[entry] = swordAttack;


    let in_Hitstop = SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 ;
    if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) 
    //&& in_Hitstop) 
    ){
        if (META_HEALED[entry]) {return;}
        if (swordAttack)
        {
            let mut damageDealt = AttackModule::get_power(boma, 0, false, 1.0, false);
            println!("{}",damageDealt);
            if (damageDealt<=3.0 && status != *FIGHTER_STATUS_KIND_ATTACK_100) {return; }
            META_HEALED[entry]=true;
            DamageModule::heal(boma,damageDealt/-7.5,0);
        }
    }
}

unsafe fn metamorphosis_update(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,entry: usize){
    if META_FRAME[entry] > 0 {
        META_FRAME[entry]=(META_FRAME[entry]-1).max(0);
        metamorphosis_check_heal(fighter,boma,entry);
    }
    metamorphosis_effects(fighter,boma,entry);
}

unsafe fn metamorphosis_effects(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor,entry: usize) {
    if META_FRAME[entry] > 0 && META_EFFECT[entry] == -1 {
        app::FighterUtil::flash_eye_info(boma);
        let handle = EffectModule::req_follow(boma, Hash40::new("sys_aura_dark"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, false, false) as u32;
        META_EFFECT[entry] = handle as i32;
		EffectModule::set_rgb(boma,handle, 1.0, 0.0, 0.0);
        
        //EffectModule::req_follow(boma, Hash40::new("sys_hit_curse"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
        //EffectModule::req_follow(boma, Hash40::new("richter_final_coffin_vacuum"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 0.75, true, 0, 0, 0, 0, 0, false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_vacuum"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
        LAST_EFFECT_SET_RATE(fighter,1.75);
        //PLAY_SE(get_fighter_common_from_accessor(boma), Hash40::new("vc_pichu_final01"));
        //PLAY_SE(get_fighter_common_from_accessor(boma), Hash40::new("se_pichu_final02"));
    }
    else if META_FRAME[entry] <= 0 && META_EFFECT[entry] != -1 {
        let handle = META_EFFECT[entry] as u32;
        EffectModule::kill_kind(boma,Hash40::new("sys_aura_dark"), false,true);
        EffectModule::kill(boma, handle, false, false);
        META_EFFECT[entry] = -1;
        
        EffectModule::req_follow(boma, Hash40::new("sys_hit_curse"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
    }
}


unsafe fn bat_control(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor,entry: usize) {
    if !(fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)) {return;}
    
    let currentFrame = fighter.motion_frame();
    if (currentFrame < 5.0)
    {
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
    }
    if (currentFrame < 20.0)
    {
        BAT_INPUT_X[entry] = ControlModule::get_stick_x(boma);
        BAT_INPUT_Y[entry] = ControlModule::get_stick_y(boma);
    }
    else if (currentFrame < 30.0)
    {
        GroundModule::attach_ground(boma, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        let mut stick_x: f32 = ControlModule::get_stick_x(boma);
        let mut stick_y: f32 = ControlModule::get_stick_y(boma);
        if (stick_x.abs() < 0.1 && stick_y.abs() < 0.1)
        {
            stick_x = 0.0;
            stick_y = 1.0;
        }
        let normalized = sv_math::vec2_normalize(stick_x, stick_y);
        let lr = PostureModule::lr(boma);
        let motion_factor = 1.0;

        //ControlModule::get_stick_angle(module_accessor)
        SET_SPEED_EX(fighter,stick_x*lr*motion_factor,stick_y*motion_factor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}


unsafe fn on_rebirth(fighter: &mut L2CFighterCommon,entry: usize)
{
    META_FRAME[entry]=0;
    META_WHIFF[entry]=false;
    META_HEALED[entry]=false;
    META_EFFECT[entry] = -1;
}


// TRAINING MODE
// Full Meter Gain/Drain via shield during up/down taunt
unsafe fn training_cheat(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,entry: usize) {
    let mut agent_base = fighter.fighter_base.agent_base;
    //if is_training_mode() {
    if true {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            //sys_attack_smoke
            //sys_hit_curse
            //sys_aura_dark!!!
            //sys_damage_curse?
            //sys_damage_purple?
            //sys_deathscythe_trace_smash
            //richter_final_coffin_vacuum
            //richter_final_coffin_start

            //sys_greenshell_trace
            //sys_assist_out
            if true && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if META_FRAME[entry] == 0 { 
                    META_FRAME[entry]=META_MAX;
                    DamageModule::add_damage(boma, 50.0,0);
                }
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_shadow"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
                LAST_EFFECT_SET_COLOR(fighter,1,0,0);
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_deathscythe_trace_smash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_vacuum"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_start"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
                LAST_EFFECT_SET_RATE(fighter,1.75);
            }

            if false && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("richter_final_coffin_start"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
            }


        }         
    }
}

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
		let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        metamorphosis_update(fighter,boma,entry);
        training_cheat(fighter,boma,entry);
        bat_control(fighter,boma,entry);

        if (fighter.is_status(*FIGHTER_STATUS_KIND_REBIRTH))
        {
            on_rebirth(fighter,entry);
        }
        
    }
}

#[fighter_reset]
fn richter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
		let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if fighter.kind() == *FIGHTER_KIND_RICHTER {
            on_rebirth(fighter,entry);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        richter_update
    );
    install_agent_resets!(richter_reset);
}