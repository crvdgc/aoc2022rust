const LIMIT: usize = 32;

pub fn part0(input: &str) {
    let blueprints = parse_input(input);
    let sum_quality_level: usize = blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| (i + 1) * blueprint.best_geo())
        .sum();
    println!("{}", sum_quality_level);
}

pub fn part1(input: &str) {
    let blueprints = parse_input(input);
    // for (i, blueprint) in blueprints.iter().enumerate() {
    //     println!("Blueprint {}: {:?}", i, blueprint);
    // }
    let prod: usize = blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.best_geo())
        .product();
    println!("{}", prod);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Resource {
    ore: usize,
    clay: usize,
    obs: usize,
    geo: usize,
    ore_bot: usize,
    clay_bot: usize,
    obs_bot: usize,
    geo_bot: usize,
}

impl Resource {
    fn step(&mut self) {
        self.ore += self.ore_bot;
        self.clay += self.clay_bot;
        self.obs += self.obs_bot;
        self.geo += self.geo_bot;
    }

    fn new() -> Resource {
        Resource {
            ore: 0,
            clay: 0,
            obs: 0,
            geo: 0,
            ore_bot: 1,
            clay_bot: 0,
            obs_bot: 0,
            geo_bot: 0,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore: usize,          // ore
    clay: usize,         // ore
    obs: (usize, usize), // ore clay
    geo: (usize, usize), // ore obs
    max_ore: usize,
}

impl Blueprint {
    fn new(ore: usize, clay: usize, obs: (usize, usize), geo: (usize, usize)) -> Self {
        use std::cmp::max;
        let max_ore = max(ore, max(clay, max(obs.0, geo.0)));
        Blueprint {
            ore,
            clay,
            obs,
            geo,
            max_ore,
        }
    }
    fn new_ore_bot(&self, resource: &Resource) -> Option<Resource> {
        if self.ore > resource.ore || resource.ore_bot >= self.max_ore {
            None
        } else {
            let mut resource = Resource {
                ore: resource.ore - self.ore,
                ..*resource
            };
            resource.step();
            resource.ore_bot += 1;
            Some(resource)
        }
    }
    fn new_clay_bot(&self, resource: &Resource) -> Option<Resource> {
        if self.clay > resource.ore || resource.clay_bot >= self.obs.1 {
            None
        } else {
            let mut resource = Resource {
                ore: resource.ore - self.clay,
                ..*resource
            };
            resource.step();
            resource.clay_bot += 1;
            Some(resource)
        }
    }
    fn new_obs_bot(&self, resource: &Resource) -> Option<Resource> {
        if self.obs.0 > resource.ore || self.obs.1 > resource.clay || resource.obs_bot >= self.geo.1
        {
            None
        } else {
            let mut resource = Resource {
                ore: resource.ore - self.obs.0,
                clay: resource.clay - self.obs.1,
                ..*resource
            };
            resource.step();
            resource.obs_bot += 1;
            Some(resource)
        }
    }
    fn new_geo_bot(&self, resource: &Resource) -> Option<Resource> {
        if self.geo.0 > resource.ore || self.geo.1 > resource.obs {
            None
        } else {
            let mut resource = Resource {
                ore: resource.ore - self.geo.0,
                obs: resource.obs - self.geo.1,
                ..*resource
            };
            resource.step();
            resource.geo_bot += 1;
            Some(resource)
        }
    }
    /// upper limit of geo at time `t`
    fn max_geo(&self, t: usize, resource: &Resource) -> usize {
        let rest_time = LIMIT - t;
        // every minute a new geo bot
        resource.geo + resource.geo_bot * rest_time + rest_time * (rest_time + 1) / 2
    }

    fn best_geo(&self) -> usize {
        let mut best_geo: usize = 0;
        let resource = Resource::new();
        self.dfs(&mut best_geo, &resource, 0);
        best_geo
    }

    fn dfs(&self, best_geo: &mut usize, resource: &Resource, t: usize) {
        // dbg!(t, &resource);
        use std::cmp::max;
        if t + 1 == LIMIT {
            let cur_best = resource.geo + resource.geo_bot;
            *best_geo = max(*best_geo, cur_best);
        } else if self.max_geo(t, resource) >= *best_geo {
            if let Some(resource) = self.new_geo_bot(resource) {
                self.dfs(best_geo, &resource, t + 1);
            } else {
                if let Some(resource) = self.new_obs_bot(resource) {
                    self.dfs(best_geo, &resource, t + 1);
                }
                if let Some(resource) = self.new_clay_bot(resource) {
                    self.dfs(best_geo, &resource, t + 1);
                }
                if let Some(resource) = self.new_ore_bot(resource) {
                    self.dfs(best_geo, &resource, t + 1);
                }
                // no robot building
                let mut resource = resource.clone();
                resource.step();
                self.dfs(best_geo, &resource, t + 1);
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    fn parse_line(line: &str) -> Blueprint {
        let mut iter = line.split(' ').filter_map(|x| x.parse::<usize>().ok());
        Blueprint::new(
            iter.next().unwrap(),
            iter.next().unwrap(),
            (iter.next().unwrap(), iter.next().unwrap()),
            (iter.next().unwrap(), iter.next().unwrap()),
        )
    }
    input.lines().map(parse_line).collect()
}

pub fn example_input() -> &'static str {
    r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#
}
