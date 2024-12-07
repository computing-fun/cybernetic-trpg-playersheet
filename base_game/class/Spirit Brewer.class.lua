Description = [[
Spirit Brewers are heavy drinkers and followers of the Astralic granted by spirits. These spirits dance happily and cheerfully around the Spirit Brewer, but will turn spiteful and meticulously dastardly towards any enemy of the Spirit Brewer.

Astralic Types: Fire & Earth
Saving Throws: Grit & Astralic
Skills: You gain mastery in two skills of your choice
Weapon Mastery: Gain mastery in unarmed strikes and one weapon of your choice
]]

AstralicTypes = { "Fire", "Earth" }

SavingThrows = { "Grit", "Astralic" }

function Skills(class_level)
    local skill_table = { "Skill Mastery", "Skill Mastery", "Unarmed Mastery", "Weapon Mastery", "The Arts of War" }
    if class_level >= 2 then
        table.insert(skill_table, "Firebreath of the Spirits")
        table.insert(skill_table, "Tailwind of the Spirits")
    end
    if class_level >= 3 then
        table.insert(skill_table, "Spirit’s Interception")
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
        table.insert(skill_table, "Second Sip of Fortune")
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
    return { "Vessel of the Spirits" }
end
