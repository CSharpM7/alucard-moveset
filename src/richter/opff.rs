use super::*;


unsafe fn metamorphosis_check_heal(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let mut swordAttack = meta_swordattack(boma);
    let mut cancelFrame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false) as f32;
 
    if (fighter.motion_frame() < 2.0){
        GetVar::set_bool(boma, &mut vars::META_HEALED,false);
        if (!fighter.is_motion(Hash40::new("attack_100")))
            {GetVar::set_bool(boma, &mut vars::META_WHIFF,false);}
    }

    if (fighter.motion_frame() < cancelFrame-5.0 
    || fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_100)
    ){
        GetVar::set_bool(boma, &mut vars::META_WHIFF,swordAttack);
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
    if (fighter.is_status(*FIGHTER_SIMON_STATUS_KIND_FINAL_END)) {return;}
    if GetVar::get_int(boma,&mut vars::META_FRAME) > 0 && GetVar::get_int(boma,&mut vars::META_EFFECT) == -1 {
        app::FighterUtil::flash_eye_info(boma);

        let handle = EffectModule::req_follow(boma, Hash40::new("sys_aura_dark"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, false, false) as u32;
        GetVar::set_int(boma,&mut vars::META_EFFECT,handle as i32);
		EffectModule::set_rgb(boma,handle, 1.0, 0.0, 0.0);

        if GetVar::get_int(boma,&mut vars::META_FRAME) > vars::META_MAX {return;}

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
            if !GetVar::get_bool(boma, &mut vars::BAT_EXIT) {

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
        let motion_factor = if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_DARK {3.125} else {2.625};
        let speedy_mult = if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_DARK {0.75} else {0.875};

        SET_SPEED_EX(fighter,stick_x*lr*motion_factor,stick_y*motion_factor*speedy_mult,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

unsafe fn fsmash_charge(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    
    let maxFrame = 30.0;
    let currentFrame = fighter.motion_frame().min(maxFrame);
    let rate = 2.0;
    let modulo = fighter.motion_frame() % rate;
    if (modulo>=1.0 && currentFrame<maxFrame && currentFrame>6.0) {return;}
    
    let canEffect = (fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD)
    || (fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4) && (currentFrame < 12.0)));
    if (!canEffect) { return; }

    let effect = Hash40::new("sys_pasaran_spark");
    let effecthi = Hash40::new("sys_fireflower_shot");
    let effectlw = Hash40::new("sys_erace_smoke");


    EFFECT_OFF_KIND(fighter,effecthi,false,false);
    EFFECT_OFF_KIND(fighter,effect,false,false);
    EFFECT_OFF_KIND(fighter,effectlw,false,false);

    if (canEffect) {
        let mut currentEffect = effect;
        let mut size = 0.5;
        let mut color = Vector3f{x: 1.0, y: 1.0, z: 1.0};
        let mut rotx=0.0;
        
        let mut stick_y: f32 = ControlModule::get_stick_y(boma);
        if (fighter.is_motion(Hash40::new("attack_s4_hi"))) {stick_y = 1.0;}
        else if (fighter.is_motion(Hash40::new("attack_s4_s"))) {stick_y = 0.0;}
        else if (fighter.is_motion(Hash40::new("attack_s4_lw"))) {stick_y = -1.0;}

        if (stick_y < -0.33)
        {
            currentEffect = effectlw;
            size = 0.5;
            color.x=0.375;
            color.y=1.0;
        }
        else if (stick_y > 0.33)
        {
            currentEffect = effecthi;
            size = 0.425;
            rotx=-90.0;
        }
        else 
        {
            currentEffect = effect;
            color.z=0.0;
            size = 0.3;
        }

        if (fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD) || (currentFrame < 11.0))
        {
            //let effecty = 3.0+(vars::LENGTH*((currentFrame/maxFrame)));
            //EFFECT_FOLLOW(fighter, currentEffect, Hash40::new("haver"), 2.0,effecty,0,0,0,0, size, true);
            for y in 0..3{
                EFFECT_FOLLOW(fighter, currentEffect, Hash40::new("haver"), 0, ((y as f32)*6.0)+2.0, 0, rotx, 0, 0, size,true);
                LAST_EFFECT_SET_RATE(fighter,0.625);
                LAST_EFFECT_SET_COLOR(fighter,color.x, color.y,color.z);
            }
        }
    }
}

unsafe fn sidespecial_refresh(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor)
{
    if (!fighter.is_situation(*SITUATION_KIND_AIR))
    {
        GetVar::set_bool(boma, &mut vars::SPECIAL_S_AERIAL, true);
    }
    else if StopModule::is_damage(boma)
    {
        GetVar::set_bool(boma, &mut vars::SPECIAL_S_AERIAL, true);
    }
}


unsafe fn final_effects(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor){
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut finalAttack = [
        //*FIGHTER_STATUS_KIND_FINAL,
        //*FIGHTER_SIMON_STATUS_KIND_FINAL_READY,
        *FIGHTER_SIMON_STATUS_KIND_FINAL_END
    ].contains(&status);
    if finalAttack && GetVar::get_int(boma,&mut vars::FINAL_EFFECT) == -1 {
        if fighter.motion_frame() > 0.0{
            let handle = EffectModule::req_follow(boma, Hash40::new("sys_blackball_set"), Hash40::new("top"), &Vector3f{x:0.0,y:27.0,z:0.0}, &Vector3f::zero(), 0.0, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rgb(boma,handle,0.25,0.0,0.0);
            EffectModule::set_rate(boma,handle,0.4);
            GetVar::set_int(boma,&mut vars::FINAL_EFFECT,handle as i32);
        }
    }
    else if finalAttack && GetVar::get_int(boma,&mut vars::FINAL_EFFECT) != -1 {
        let handle = GetVar::get_int(boma,&mut vars::FINAL_EFFECT) as u32;
        let currentScale = 0.5+fighter.motion_frame().min(120.0)/20.0;
        let scale = Vector3f{x:currentScale,y:currentScale,z:1.0};
        EffectModule::set_scale(boma, handle, &scale);
        GetVar::set_int(boma,&mut vars::META_EFFECT, -1);
    }
    else if !finalAttack && GetVar::get_int(boma,&mut vars::FINAL_EFFECT) != -1 {
        let handle = GetVar::get_int(boma,&mut vars::FINAL_EFFECT) as u32;
        EffectModule::kill(boma, handle, false, false);
        GetVar::set_int(boma,&mut vars::FINAL_EFFECT, -1);
    }
}

unsafe fn on_rebirth(fighter: &mut L2CFighterCommon)
{
    vars::reset(fighter.module_accessor);
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
                    //let screen = EffectModule::req_screen(fighter.module_accessor,Hash40::new("bg_test1"),false,false,false) as u32;
                    //EffectModule::set_rgb(boma,screen,1.0,0.0,0.0);
                    //LAST_EFFECT_SET_COLOR(fighter,1,0,0);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_exp"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.0, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_hold"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.0, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_ray"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.0, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_ray_del"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.0, true);
                }
            }
            else if fighter.is_motion(Hash40::new("appeal_s_l")) || fighter.is_motion(Hash40::new("appeal_s_r")) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_ray_long_del"), Hash40::new("top"), 0,11,0,0,0,0, 0.625, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("top"), 0,11,0,0,0,0, 0.625, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_ray_long"), Hash40::new("top"), 0, 8, 6, 5, 0, -40, 0.825, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_ray"), Hash40::new("top"), 0, 8, 6, 5, 0, -40, 0.825, true);
                }
            }
            else if fighter.is_motion(Hash40::new("appeal_lw_l")) || fighter.is_motion(Hash40::new("appeal_lw_r")) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    EffectModule::req_screen(fighter.module_accessor,Hash40::new("bg_test1"),false,false,false);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    EffectModule::req_screen(fighter.module_accessor,Hash40::new("bg_test2"),false,false,false);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    EffectModule::req_screen(fighter.module_accessor,Hash40::new("bg_test3"),false,false,false);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    EffectModule::req_screen(fighter.module_accessor,Hash40::new("bg_test4"),false,false,false);
                }
            }


        }         
    }
}

// TRAINING MODE
unsafe fn debug(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_STAKE) {return;}
    let mut agent_base = fighter.fighter_base.agent_base;
    if is_training_mode() {
    //if true {
        //ArticleModule::change_status(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_STAKE,*WEAPON_SIMON_CROSS_STATUS_KIND_TURN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));

        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            let article_boma = get_article_boma(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_STAKE);
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                let link = LinkModule::get_node_object_id(article_boma, *LINK_NO_CONSTRAINT);
                let id = boma.battle_object_id;
                LinkModule::off_model_constraint_flag(article_boma, 0);
                println!("(attack) link: {0}, self: {1}",link,id);
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                println!("(guard) joint: {0},jointPos: {1},node: {2},target: {3},nodePos: {4}",
                LinkModule::get_model_constraint_joint(article_boma),
                LinkModule::get_model_constraint_joint_global_position(article_boma,Hash40::new("trans")),
                LinkModule::get_model_constraint_no(article_boma),
                LinkModule::get_model_constraint_target_joint(article_boma),
                LinkModule::get_model_constraint_target_node_position(article_boma),
                );
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                println!("(guard) isMutual: {0}",
                LinkModule::is_model_constraint_mutual(article_boma),
                );
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                let link = LinkModule::get_node_object_id(article_boma, *LINK_NO_CONSTRAINT);
                let id = boma.battle_object_id;
                println!("(hi) link: {0}, self: {1}",link.to_string(),id.to_string());
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                let link = LinkModule::get_model_constraint_no(article_boma);
                let id = boma.battle_object_id;
                println!("(r) link: {0}, self: {1}",link.to_string(),id.to_string());
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                let link = LinkModule::unlink_node(article_boma,*LINK_NO_CONSTRAINT);
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                let link = LinkModule::unlink_all(article_boma);
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
        fsmash_charge(fighter,boma);
        sidespecial_refresh(fighter,boma);
        final_effects(fighter,boma);  

        debug(fighter,boma);      
        
        let status = StatusModule::status_kind(fighter.module_accessor);
        if (status == (*FIGHTER_STATUS_KIND_REBIRTH))
        {
            on_rebirth(fighter);
        }
        if !([
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2
        ].contains(&status)
        )
        {
            GetVar::set_int(boma, &mut vars::SPECIAL_S_TYPE,0);
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