Description = [[
If you choose to be a Full Martial when choosing your class, you get some huge benefits to weapons, armor, and health but you lose your access to magic. Imagine a cybernetic samurai or an ex-soldier sniper.

These characters lack magic but use abilities like Kinetics, Coordination, and Grit to their highest capacity with big focuses on their weapons and cybernetics.

- When choosing Full Martial, you gain an additional 12 Health every level, including the 1st level.
- You gain a +10 bonus to your Armor Rating.
- You gain mastery in two weapons of your choice.
- At level 5 or higher, whenever you spend action points to use the attack action twice in one round, you get to attack an additional time for free. Example, you use an action point to attack with a dagger, then you use 1 more action point to attack with a dagger. You now get to attack with a dagger for a third time without using an action point.

When choosing Full Martial, you will not be granted the ability to cast spells through your class. If there are any class features that grant the ability to cast spells, you must ignore them. However, you can gain the ability to cast spells through cybernetics.
]]

function Health(class_level)
    return class_level * 12
end

function ArmorRating(class_level)
    return 10
end

function Skills(class_level)
    local skill_table = { "Weapon Mastery", "Weapon Mastery" }
    if class_level >= 5 then
        table.insert(skill_table, "Extra Attack")
    end
    return skill_table
end
