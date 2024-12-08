Description =
"Shadowjacks are ruthless combatants who thrive in darkness, delivering devastating, one-sided strikes that often end a fight with a single blow. Their unparalleled mastery of the C-Net and shadowy tactics makes them formidable both in the digital realm and on the battlefield. Feared as assassins and hackers, Shadowjacks are a force to be reckoned with, slipping through defenses with ease and eliminating targets with chilling precision. Their reputation for operating in the shadows has earned them the undivided attention of law enforcement, who regard them as both a threat to security and a symbol of unrelenting efficiency."

AstralicTypes = { "Dark", "Digital" }

SavingThrows = { "Ingenuity", "Influence" }

function Skills(class_level)
    local skill_table = { "Ingenuity Skill Mastery", "Skill Mastery", "Skill Mastery", "Weapon Mastery",
        "Dagger Weapon Mastery", "Spellblade Sheath’s Bond", "Spellblade’s Edge" }
    if class_level >= 2 then
        table.insert(skill_table, "Firebreath of the Spirits")
        table.insert(skill_table, "Tailwind of the Spirits")
    end
    if class_level >= 3 then
        table.insert(skill_table, "Drunken Reflexes")
    end
    if class_level >= 4 then
        table.insert(skill_table, "Tipsy Recovery")
    end
    if class_level >= 5 then
        table.insert(skill_table, "Blessings of the Brew")
    end
    if class_level >= 6 then
        table.insert(skill_table, "Fermented Fury")
        table.insert(skill_table, "Wine-Walker")
    end
    if class_level >= 7 then
        table.insert(skill_table, "A Spirit King’s Tolerance")
        table.insert(skill_table, "Wind Spirit’s Gift")
    end
    if class_level >= 10 then
        table.insert(skill_table, "Fire Spirit’s Gift")
    end
    if class_level >= 11 then
        table.insert(skill_table, "Brew-Force Blow")
    end
    if class_level >= 13 then
        table.insert(skill_table, "Ascended Skills")
    end
    if class_level >= 14 then
        table.insert(skill_table, "Powerful Silence")
    end
    if class_level >= 15 then
        table.insert(skill_table, "Spirit's Neverending Draught")
    end
    if class_level >= 17 then
        table.insert(skill_table, "Spirit’s Retaliation")
    end
    if class_level >= 20 then
        table.insert(skill_table, "Eternal Flow")
    end
    return skill_table
end

function Cybernetics(class_level)
    return { "Spellblade Sheath" }
end
