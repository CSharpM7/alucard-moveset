use super::*;
mod forwardair;
mod backair;
mod downtilt;
mod jab;
mod upthrow;
mod downsmash;


#[smashline::installer]
pub fn install() {
    forwardair::install();
    backair::install();
    downtilt::install();
    jab::install();
    upthrow::install();
    downsmash::install();
    smashline::install_agent_frames!(
        whip_update
    );
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