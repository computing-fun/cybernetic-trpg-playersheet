function health(class_level)
    class_level * 6
end

function armor_rating(class_level)
    return 0
end

function spell_level(class_level)
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