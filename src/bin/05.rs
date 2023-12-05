advent_of_code::solution!(5);

struct AlmanacMapEntry {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

impl AlmanacMapEntry {
    fn from_map_line(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let destination_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_start = parts.next().unwrap().parse::<u64>().unwrap();
        let range = parts.next().unwrap().parse::<u64>().unwrap();

        Self {
            destination_start,
            source_start,
            range,
        }
    }
}

struct AlmanacMap {
    entries: Vec<AlmanacMapEntry>,
}

impl AlmanacMap {
    fn from_map(input: &str) -> Self {
        let entries = input.lines().map(AlmanacMapEntry::from_map_line).collect::<Vec<AlmanacMapEntry>>();

        Self {
            entries,
        }
    }

    fn get(&self, value: u64) -> u64 {
        let Some(entry) = self.entries.iter().find(|entry| entry.source_start <= value && entry.source_start + entry.range > value) else {
            return value;
        };
        let offset = value - entry.source_start;

        entry.destination_start + offset
    }

    fn get_backwards(&self, value: u64) -> u64 {
        let Some(entry) = self.entries.iter().find(|entry| entry.destination_start <= value && entry.destination_start + entry.range > value) else {
            return value;
        };
        let offset = value - entry.destination_start;

        entry.source_start + offset
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac_regex = regex::Regex::new(r"seeds: ([0-9 ]+)\n\nseed-to-soil map:\n([0-9 \n]+)\n\nsoil-to-fertilizer map:\n([0-9 \n]+)\n\nfertilizer-to-water map:\n([0-9 \n]+)\n\nwater-to-light map:\n([0-9 \n]+)\n\nlight-to-temperature map:\n([0-9 \n]+)\n\ntemperature-to-humidity map:\n([0-9 \n]+)\n\nhumidity-to-location map:\n([0-9 \n]+)").unwrap();
    let almanac = almanac_regex.captures(input).unwrap();
    let seeds = almanac.get(1).unwrap().as_str().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let seed_to_soil_map = AlmanacMap::from_map(almanac.get(2).unwrap().as_str());
    let soil_to_fertilizer_map = AlmanacMap::from_map(almanac.get(3).unwrap().as_str());
    let fertilizer_to_water_map = AlmanacMap::from_map(almanac.get(4).unwrap().as_str());
    let water_to_light_map = AlmanacMap::from_map(almanac.get(5).unwrap().as_str());
    let light_to_temperature_map = AlmanacMap::from_map(almanac.get(6).unwrap().as_str());
    let temperature_to_humidity_map = AlmanacMap::from_map(almanac.get(7).unwrap().as_str());
    let humidity_to_location_map = AlmanacMap::from_map(almanac.get(8).unwrap().as_str());

    seeds
        .iter()
        .map(|seed| {
            let soil = seed_to_soil_map.get(*seed);
            let fertilizer = soil_to_fertilizer_map.get(soil);
            let water = fertilizer_to_water_map.get(fertilizer);
            let light = water_to_light_map.get(water);
            let temperature = light_to_temperature_map.get(light);
            let humidity = temperature_to_humidity_map.get(temperature);

            humidity_to_location_map.get(humidity)
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac_regex = regex::Regex::new(r"seeds: ([0-9 ]+)\n\nseed-to-soil map:\n([0-9 \n]+)\n\nsoil-to-fertilizer map:\n([0-9 \n]+)\n\nfertilizer-to-water map:\n([0-9 \n]+)\n\nwater-to-light map:\n([0-9 \n]+)\n\nlight-to-temperature map:\n([0-9 \n]+)\n\ntemperature-to-humidity map:\n([0-9 \n]+)\n\nhumidity-to-location map:\n([0-9 \n]+)").unwrap();
    let almanac = almanac_regex.captures(input).unwrap();
    let seeds = almanac.get(1).unwrap().as_str().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut from_to_seeds = Vec::new();
    for (i, seed) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            from_to_seeds.push((*seed, seed + seeds.get(i + 1).unwrap()));
        }
    }

    let seed_to_soil_map = AlmanacMap::from_map(almanac.get(2).unwrap().as_str());
    let soil_to_fertilizer_map = AlmanacMap::from_map(almanac.get(3).unwrap().as_str());
    let fertilizer_to_water_map = AlmanacMap::from_map(almanac.get(4).unwrap().as_str());
    let water_to_light_map = AlmanacMap::from_map(almanac.get(5).unwrap().as_str());
    let light_to_temperature_map = AlmanacMap::from_map(almanac.get(6).unwrap().as_str());
    let temperature_to_humidity_map = AlmanacMap::from_map(almanac.get(7).unwrap().as_str());
    let humidity_to_location_map = AlmanacMap::from_map(almanac.get(8).unwrap().as_str());

    for potential_location in 0..u64::MAX {
        let humidity = humidity_to_location_map.get_backwards(potential_location);
        let temperature = temperature_to_humidity_map.get_backwards(humidity);
        let light = light_to_temperature_map.get_backwards(temperature);
        let water = water_to_light_map.get_backwards(light);
        let fertilizer = fertilizer_to_water_map.get_backwards(water);
        let soil = soil_to_fertilizer_map.get_backwards(fertilizer);
        let seed = seed_to_soil_map.get_backwards(soil);

        for (from, to) in &from_to_seeds {
            if seed >= *from && seed <= *to {
                return Some(potential_location);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
