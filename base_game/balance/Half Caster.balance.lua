Description = [[
If you choose to be a Half Caster when choosing your class, you get about half the benefits to armors, weapons, and health and about half the access to magic as a Full Caster. You will only gain spells up to the spell level of 5 and you will have roughly half the spell slots of a Full Caster. This is a perfect balance between a Full Martial and a Full Caster character.

These characters will be tricky to balance. If you choose to be a Half Caster, take a good look at the types of Astralic that are available and the spells that come with it. Also take a look at the cybernetic the class comes with. You’re going to want to strike a strong balance between the martial ability scores and the mental ability scores.

- When choosing Half Caster, you gain an additional 9 Health every level, including the 1st level.
- You gain a +5 bonus to your Armor Rating.
- You gain mastery in one weapon of your choice.
- At level 5 or higher, whenever you spend action points to use the attack action twice in one round, you get to attack an additional time for free. Example, you use an action point to attack with a dagger, then you use 1 more action point to attack with a dagger. You now get to attack with a dagger for a third time without using an action point.

You are granted the ability to cast spells through your class but gain the ability to cast at higher levels at roughly half the pace of a Full Caster. You will be granted roughly half the spell slots as a Full Caster. When your character reaches the levels mentioned below, you unlock the spell level mentioned with it.
- Level 2 = 1st level spells
- Level 5 = 2nd level spells
- Level 9 = 3rd level spells
- Level 13 = 4th level spells
- Level 17 = 5th level spells
]]

function Health(class_level)
    return class_level * 9
end

function ArmorRating(class_level)
    return 5
end

function SpellLevel(class_level)
    if class_level >= 0 and class_level <= 1 then
        return 0 -- Level 0-1 = no spells
    elseif class_level >= 2 and class_level <= 4 then
        return 1 -- Level 2-4 = 1st level spells
    elseif class_level >= 5 and class_level <= 8 then
        return 2 -- Level 5-8 = 2nd level spells
    elseif class_level >= 9 and class_level <= 12 then
        return 3 -- Level 9-12 = 3rd level spells
    elseif class_level >= 13 and class_level <= 16 then
        return 4 -- Level 13-16 = 4th level spells
    elseif class_level >= 17 then
        return 5 -- Level 17+ = 5th level spells
    else
        return 0 -- Fallback
    end
end

function Skills(class_level)
    local skill_table = { "Weapon Mastery" }
    if class_level >= 5 then
        table.insert(skill_table, "Extra Attack")
    end
    return skill_table
end
