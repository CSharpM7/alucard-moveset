use super::*;
mod trails;
//mod backair;
//mod downtilt;
mod jab;
mod upthrow;
mod downsmash;


#[smashline::installer]
pub fn install() {
    trails::install();
    //backair::install();
    //downtilt::install();
    jab::install();
    upthrow::install();
    downsmash::install();
    smashline::install_agent_frames!(
        whip_update
    );
}

pub unsafe fn whip_trail(weapon: &mut L2CAgentBase) {
    let backAir = weapon.is_motion(Hash40::new("attack_air_b_hi")) ||
    weapon.is_motion(Hash40::new("attack_air_b")) ||
    weapon.is_motion(Hash40::new("attack_air_b_lw"));
    let boma = get_owner_boma(weapon);
    let daggerFactor= if backAir {-0.5} else {1.0};
    
    let color = if (vars::meta_is_active(boma)) {Vector3f{x: 1.0,y:0.5,z:0.5}} else {Vector3f{x: 0.0,y:0.0,z:1.0}};
    let effect = if (vars::meta_is_active(boma)) {Hash40::new("tex_item_killsword3")} else {Hash40::new("tex_item_killsword2")};

    AFTER_IMAGE4_ON_arg29(weapon,effect, effect, 5, Hash40::new("whip"), 0.0, 0.0, 0.0, Hash40::new("whip"), 0.0, (vars::LENGTH+2.0)*daggerFactor, 0.0, true, Hash40::new("null"), Hash40::new("whip"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);

    LAST_PARTICLE_SET_COLOR(weapon, color.x,color.y,color.z);
}

struct SoulTrail {
    motion: Hash40,
    frame_min: f32,
    frame_max: f32
}
impl SoulTrail {
    unsafe fn isActive(&self, weapon: &mut L2CFighterBase, currentFrame: f32) -> bool {
        self.frame_min <= currentFrame && currentFrame <= self.frame_max &&
        weapon.is_motion(self.motion)
    }
}


#[weapon_frame( agent = WEAPON_KIND_RICHTER_WHIP )]
fn whip_update(weapon: &mut L2CFighterBase) {
    unsafe {
        let mut backAir = weapon.is_motion(Hash40::new("attack_air_b_hi")) ||
        weapon.is_motion(Hash40::new("attack_air_b")) ||
        weapon.is_motion(Hash40::new("attack_air_b_lw"));
        
        backAir = backAir && weapon.motion_frame() <= 35.0;
        ModelModule::set_mesh_visibility(weapon.module_accessor, 
            Hash40::new("stake"), 
            backAir
        );


        let upThrow = SoulTrail {
            motion: Hash40::new("throw_hi"),
            frame_min: 22.0,
            frame_max: 46.0
        };
        let downSmash = SoulTrail {
            motion: Hash40::new("attack_lw4"),
            frame_min: 14.0,
            frame_max: 31.0
        };
        let downSmashCharge = SoulTrail {
            motion: Hash40::new("attack_lw4_hold"),
            frame_min: 2.0,
            frame_max: 100.0
        };

        let trails:[SoulTrail;2] = [upThrow,downSmash];
        let mut isActive=false;
        let currentFrame = weapon.motion_frame();
        let rate = 2.0;
        let modulo = weapon.motion_frame() % rate;
        if (modulo<1.0)
        {
            for i in 0..trails.len() {
                if (trails[i].isActive(weapon, currentFrame))
                {
                    ModelModule::set_mesh_visibility(weapon.module_accessor, 
                        Hash40::new("skull"), 
                        true
                    );
                    isActive=true;
                    let mut size = 1.0;
                    if (currentFrame>=trails[i].frame_max-10.0)
                    {
                        size = 1.0 - (1.0/(trails[i].frame_max-currentFrame+1.0));
                    }
                        EFFECT_FOLLOW(weapon, Hash40::new("sys_deathscythe_trace"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, size/2.0, true);
                    break;
                }
            }
            if !isActive {
                //EffectModule::kill_kind(weapon.module_accessor,Hash40::new("sys_deathscythe_shadow"), false,true);
                EFFECT_OFF_KIND(weapon, Hash40::new("sys_deathscythe_shadow"), false,false);
                EFFECT_OFF_KIND(weapon, Hash40::new("sys_greenshell_trace"), false,false);
                ModelModule::set_mesh_visibility(weapon.module_accessor, 
                    Hash40::new("skull"), 
                    false
                );
            }
        }
        
    }
}