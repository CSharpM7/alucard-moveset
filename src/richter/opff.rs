use super::*;


unsafe fn metamorphosis_check_heal(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut swordAttack = [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_100,
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
            fighter.is_motion(Hash40::new("attack_100")) ||
            fighter.is_motion(Hash40::new("attack_100_end"));
        }
        
    }
    let mut cancelFrame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false) as f32;

    if (fighter.motion_frame() < 2.0){
        GetVar::set_bool(boma, &mut vars::META_HEALED,false);
        if (!fighter.is_motion(Hash40::new("attack_100")))
            {GetVar::set_bool(boma, &mut vars::META_WHIFF,false);}
    }
    if (!fighter.is_motion(Hash40::new("attack_100_start")) && !fighter.is_motion(Hash40::new("attack_100")))
    && (!swordAttack ||(fighter.motion_frame() > cancelFrame-5.0))
    {
        if (GetVar::is_bool(boma, &mut vars::META_WHIFF) && !GetVar::is_bool(boma, &mut vars::META_HEALED))
        {
            let mut frameloss = META_PUNISH;
            GetVar::add_int(boma,&mut vars::META_FRAME,frameloss);
            EffectModule::req_follow(boma, Hash40::new("sys_hit_curse"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 1.25, true, 0, 0, 0, 0, 0, false, false);
        }
        GetVar::set_bool(boma, &mut vars::META_WHIFF,false);
    }
    if (fighter.motion_frame() < cancelFrame-5.0 
    || status==*FIGHTER_STATUS_KIND_ATTACK_100
    ){
        GetVar::set_bool(boma, &mut vars::META_WHIFF,swordAttack);
        let in_Hitstop = SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 ;
        if (AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) 
        //&& in_Hitstop) 
        ){
            if (GetVar::is_bool(boma, &mut vars::META_HEALED)) {return;}
            if (swordAttack)
            {
                let mut damageDealt = AttackModule::get_power(boma, 0, false, 1.0, false);
                println!("{}",damageDealt);
                if (damageDealt<=3.0 && status != *FIGHTER_STATUS_KIND_ATTACK_100) {return; }
                GetVar::set_bool(boma, &mut vars::META_HEALED,true);
                DamageModule::heal(boma,damageDealt/-7.5,0);
                PLAY_SE(fighter, Hash40::new("se_richter_attackair_b02"));

                EFFECT_FOLLOW(fighter, Hash40::new("sys_badge"), Hash40::new("hip"), 0,0,0,0,0,0, 1.00, true);
                LAST_EFFECT_SET_RATE(fighter,2.5);
            }
        }
        else if fighter.is_motion(Hash40::new("attack_100"))
        {
            EFFECT_OFF_KIND(fighter,Hash40::new("sys_badge"),true,false);
        }
    }


}

unsafe fn metamorphosis_update(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    if GetVar::get_int(boma,&mut vars::META_FRAME) > 0 {
        GetVar::add_int(boma,&mut vars::META_FRAME,-1);
        metamorphosis_check_heal(fighter,boma);
    }
    metamorphosis_effects(fighter,boma);
}

unsafe fn metamorphosis_effects(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor){
    if GetVar::get_int(boma,&mut vars::META_FRAME) > 0 && GetVar::get_int(boma,&mut vars::META_EFFECT) == -1 {
        app::FighterUtil::flash_eye_info(boma);

        let handle = EffectModule::req_follow(boma, Hash40::new("sys_aura_dark"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, false, false) as u32;
        GetVar::set_int(boma,&mut vars::META_EFFECT,handle as i32);
		EffectModule::set_rgb(boma,handle, 1.0, 0.0, 0.0);
        
        EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_vacuum"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
        LAST_EFFECT_SET_RATE(fighter,1.75);
        
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_special_s"));
        //PLAY_SE(fighter, Hash40::new("vc_richter_special_s02"));
    }
    else if GetVar::get_int(boma,&mut vars::META_FRAME) <= 0 && GetVar::get_int(boma,&mut vars::META_EFFECT) != -1 {
        app::FighterUtil::flash_eye_info(boma);

        let handle = GetVar::get_int(boma,&mut vars::META_EFFECT) as u32;
        EffectModule::kill_kind(boma,Hash40::new("sys_aura_dark"), false,true);
        EffectModule::kill(boma, handle, false, false);
        GetVar::set_int(boma,&mut vars::META_EFFECT, -1);
        
        EffectModule::req_follow(boma, Hash40::new("sys_hit_curse"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);

        
        PLAY_SE(fighter, Hash40::new("se_richter_appear02"));
    }
}


unsafe fn bat_control(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor){
    
    if !(fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)) {
        
        if (fighter.is_prev_status(*FIGHTER_STATUS_KIND_SPECIAL_HI))
        {
            if !GetVar::is_bool(boma, &mut vars::BAT_EXIT) {

                GetVar::set_bool(boma, &mut vars::BAT_EXIT,true);
                GetVar::set_int(boma,&mut vars::BAT_EXIT_FRAME,10);
                EFFECT(fighter, Hash40::new("sys_damage_curse"), Hash40::new("top"), -5, 7.5, 2, 0, 0, 0, 1.5, 0,0,0,0,0,0,true);
                LAST_EFFECT_SET_COLOR(fighter,1,0,1);

                PostureModule:: set_rot(
                    fighter.module_accessor,
                    &Vector3f{x: 0.0, y: 0.0, z: 0.0},
                    0
                ); 

                let stick_x: f32 = GetVar::get_float(boma,&mut vars::BAT_INPUT_X);
                let stick_y: f32 = GetVar::get_float(boma,&mut vars::BAT_INPUT_Y);
                let lr = PostureModule::lr(boma);
                let motion_factor = 1.5;
                SET_SPEED_EX(fighter,stick_x*lr*motion_factor,stick_y*motion_factor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                
            }
        }
        else
        {
            GetVar::set_bool(boma, &mut vars::BAT_EXIT,false);
        }
        return;
    }
    
    let currentFrame = fighter.motion_frame();
    if (currentFrame >= 1.0 && currentFrame < 7.0
    && (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_motion(Hash40::new("special_hi")))
    && false)
    {
        GroundModule::set_attach_ground(boma, false);
        let mut delta = 7.0-currentFrame;
        if currentFrame < 2.0 {delta = 4.0;}
        let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)+delta/1.0, z: 0.0 };
        PostureModule::set_pos(fighter.module_accessor, &pos);
        PostureModule::init_pos(fighter.module_accessor, &pos, false, false);
    }
    if (!WorkModule::is_flag(boma,*FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE))
    {
        GroundModule::set_attach_ground(boma, false);
        let mut stick_x: f32 = ControlModule::get_stick_x(boma);
        let mut stick_y: f32 = ControlModule::get_stick_y(boma);
        let isGrounded = fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_motion(Hash40::new("special_hi"));

        //If in deadzone, go up
        if (stick_x.abs() < 0.1 && stick_y.abs() < 0.1)
        {
            stick_x = 0.0; //if isGrounded {PostureModule::lr(boma)} else {0.0};
            stick_y = 1.0;
        }
        //If on ground, and aiming the stick towards the ground, limit y to 0
        if (isGrounded && stick_y < -0.5)
        {
            stick_y = -0.5;
            if (stick_x.abs() <0.1) {stick_x = PostureModule::lr(boma);}
            stick_x = sv_math::vec2_normalize(stick_x, stick_y).x;
        }

        GetVar::set_float(boma,&mut vars::BAT_INPUT_X,stick_x);
        GetVar::set_float(boma,&mut vars::BAT_INPUT_Y,stick_y);

        let normalized = sv_math::vec2_normalize(stick_x, stick_y);
        let arctangent = normalized.y.atan2(normalized.x.abs());
        let mut degree = arctangent.to_degrees();
        let mut flip = 0.0;
        if (degree > 90.0 && degree < 270.0)
        {
            degree=(degree-180.0);
            flip = 180.0;
        }
        GetVar::set_float(boma,&mut vars::BAT_DEGREE,degree);
    }
    else
    {
        let stick_x: f32 = GetVar::get_float(boma,&mut vars::BAT_INPUT_X);
        let stick_y: f32 = GetVar::get_float(boma,&mut vars::BAT_INPUT_Y);
        if (stick_x.abs() >=0.1){
            PostureModule::set_lr(boma, stick_x.signum());
        }
        PostureModule::update_rot_y_lr(boma);
        let lr = PostureModule::lr(boma);
        let motion_factor = 2.75;
        let speedy_mult = 0.875;

        SET_SPEED_EX(fighter,stick_x*lr*motion_factor,stick_y*motion_factor*speedy_mult,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}


unsafe fn on_rebirth(fighter: &mut L2CFighterCommon)
{
    vars::reset(fighter.module_accessor);
    /* 
    GetVar::get_int(boma,&mut vars::META_FRAME)=0;
    GetVar::is_bool(boma, &mut vars::META_WHIFF)=false;
    GetVar::is_bool(boma, &mut vars::META_HEALED)=false;
    GetVar::get_int(boma,&mut vars::META_EFFECT) = -1;

    GetVar::is_bool(boma, &mut vars::BAT_EXIT)=false;
    GetVar::get_int(boma,&mut vars::BAT_EXIT_FRAME) = 0;
    */
}
// TRAINING MODE
unsafe fn training_cheat(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let mut agent_base = fighter.fighter_base.agent_base;
    if is_training_mode() {
    //if true {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if fighter.is_motion(Hash40::new("appeal_hi_l")) || fighter.is_motion(Hash40::new("appeal_hi_r")) {
                PLAY_SE(fighter, Hash40::new("se_richter_whip_hit"));
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0,0,0,0,0,0, 0.625, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0,0,0,0,0,0, 0.625, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0,0,0,0,0,0, 0.625, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0,0,0,0,0,0, 0.625, true);
                }
            }
            else if fighter.is_motion(Hash40::new("appeal_s_l")) || fighter.is_motion(Hash40::new("appeal_s_r")) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 11, 4, 0, 0, -40, 0.75, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 11, 4, 50, 0, 0, 0.75, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    EFFECT_FOLLOW(fighter, Hash40::new("richter_attack100_end"), Hash40::new("top"), 0, 8, 6, 5, 0, -40, 0.825, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_trace"), Hash40::new("top"), 0, 8, 6, 5, 0, -40, 0.825, true);
                }
            }


        }         
    }
}

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        
        metamorphosis_update(fighter,boma);
        training_cheat(fighter,boma);
        bat_control(fighter,boma);

        if (fighter.is_status(*FIGHTER_STATUS_KIND_REBIRTH))
        {
            on_rebirth(fighter);
        }
        
    }
}

#[fighter_reset]
fn richter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
		let entry = get_entry(fighter);
        if fighter.kind() == *FIGHTER_KIND_RICHTER {
            on_rebirth(fighter);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        richter_update
    );
    install_agent_resets!(richter_reset);
}