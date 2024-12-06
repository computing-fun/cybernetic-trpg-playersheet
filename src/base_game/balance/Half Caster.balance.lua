function health(class_level)
    class_level * 9
end

function armor_rating(class_level)
    return 5
end

function spell_level(class_level)
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