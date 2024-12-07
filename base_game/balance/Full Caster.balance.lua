Description = [[
If you choose to be a Full Caster when choosing your class, you get some benefits to casting stronger spells more often but your access to weapons, armors, and health will not be as strong as the others. You won’t be deadly with a blade but you will be a wicked spell slinger.

Full Casters focus on the mental ability scores for the lethality and potency of their Astralic. Since Full Casters focus on Astralic above all else, pay close attention to the Astralic available to the class. You will want your highest ability score to be the Astralic used in your spells, followed by a decent Coordination and/or Grit to help keep you alive.

- When choosing Full Caster, you gain an additional 6 Health every level, including the 1st level.
- You do not gain a bonus to your Armor Rating.

You are granted the ability to cast spells through your class and gain the ability to cast spells at higher levels at the fastest pace available. You will be granted all available spell slots. When your character reaches the levels mentioned below, you unlock the spell level mentioned with it.
- Level 1 = 1st level spells
- Level 3 = 2nd level spells
- Level 5 = 3rd level spells
- Level 7 = 4th level spells
- Level 9 = 5th level spells
- Level 11 = 6th level spells
- Level 13 = 7th level spells
- Level 15 = 8th level spells
- Level 17 = 9th level spells
]]

function Health(class_level)
    return class_level * 6
end

function ArmorRating(class_level)
    return 0
end

function SpellLevel(class_level)
    if class_level == 0 then
        return 0
    elseif class_level >= 1 and class_level <= 2 then
        return 1 -- Level 1 = 1st level spells
    elseif class_level >= 3 and class_level <= 4 then
        return 2 -- Level 3 = 2nd level spells
    elseif class_level >= 5 and class_level <= 6 then
        return 3 -- Level 5 = 3rd level spells
    elseif class_level >= 7 and class_level <= 8 then
        return 4 -- Level 7 = 4th level spells
    elseif class_level >= 9 and class_level <= 10 then
        return 5 -- Level 9 = 5th level spells
    elseif class_level >= 11 and class_level <= 12 then
        return 6 -- Level 11 = 6th level spells
    elseif class_level >= 13 and class_level <= 14 then
        return 7 -- Level 13 = 7th level spells
    elseif class_level >= 15 and class_level <= 16 then
        return 8 -- Level 15 = 8th level spells
    elseif class_level >= 17 then
        return 9 -- Level 17+ = 9th level spells
    else
        return 0 -- Fallback
    end
end

function Skills(class_level)
    return {}
end
