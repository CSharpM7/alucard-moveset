game_Throw = function ()
    if sv_animcmd.is_excute() then
        local f1_local0, f1_local1, f1_local2 = nil
        sv_animcmd.ATTACK(0, 0, 23356055229, 2, 60, 100, 44, 0, 1.600000023841858, 0, 0, 0, f1_local0, f1_local1, f1_local2, 1, 0, ATTACK_SETOFF_KIND_ON, ATTACK_LR_CHECK_F, true, 0, 0, 0, true, false, false, false, false, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, 92925133491, ATTACK_SOUND_LEVEL_S, COLLISION_SOUND_ATTR_KICK, ATTACK_REGION_OBJECT)
        AttackModule.enable_safe_pos()
    end
    return 
end

game_Born = function ()
    if sv_animcmd.is_excute() then
        local f2_local0, f2_local1, f2_local2 = nil
        sv_animcmd.ATTACK(0, 0, 23356055229, 1.399999976158142, 366, 100, 30, 0, 10, 0, 7, 0, f2_local0, f2_local1, f2_local2, 1, 1, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 0, 0, 3, true, true, false, false, false, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, 82430076406, ATTACK_SOUND_LEVEL_S, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_BOMB)
    end
    sv_animcmd.frame(3)
    if sv_animcmd.is_excute() then
        sv_animcmd.ATTACK(0, 0, 23356055229, 5.4, 86, 32, 0, 68, 7, 0, 9, 0, 0, 14, 0, 1, 1.0, ATTACK_SETOFF_KIND_ON, ATTACK_LR_CHECK_F, false, 0, 0, 0, true, true, false, false, false, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, 82430076406, ATTACK_SOUND_LEVEL_S, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_BOMB)
        local f2_local0, f2_local1, f2_local2 = nil
        sv_animcmd.ATTACK(1, 0, 23356055229, 5.4, 86, 32, 0, 68, 8, 0, 5, 0, f2_local0, f2_local1, f2_local2, 1, 1.0, ATTACK_SETOFF_KIND_ON, ATTACK_LR_CHECK_F, false, 0, 0, 0, true, true, false, false, false, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, 82430076406, ATTACK_SOUND_LEVEL_S, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_BOMB)
        ControlModule.set_rumble(72656470004, 0, false)
    end
    return 
end

return 
