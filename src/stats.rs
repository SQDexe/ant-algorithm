use {
    serde::{
        Deserialize,
        Serialize
        },
    serde_json::{
        from_str as from_json,
        to_string_pretty as to_json
        },
    std::{
        error::Error,
        fs::File,
        io::{
            Read,
            Write
            },
        path::Path
        }
    };

/* A structure holding statistics of simulation's run, and operations needed for saving this data */
#[derive(Default, Deserialize, Serialize)]
pub struct Stats {
    pub completed: bool,
    pub pheromone_strengths: Vec<f64>,
    pub average_route_len: f64,
    pub ants_per_phase: Vec<usize>,
    pub average_returns: f64
    }

impl Stats {
    pub fn show(&self) {
        println!(
"o> --- STATISTICS --- <o
|        all reached goal: {}
|    pheromones per point: {:?}
|    average route length: {}
| satiated ants per phase: {:?}
|  average routes per ant: {}
|    pheromones per route: {:?}
o> ------------------ <o",
            self.completed,
            self.pheromone_strengths,
            self.average_route_len,
            self.ants_per_phase,
            self.average_returns,
            self.get_average_pheromone_strengths()
            );
        }

    pub fn write_to_file(self, absolute_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut data = Vec::new();
        
        if absolute_path.exists() {
            let mut file = File::open(&absolute_path)?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            for d in from_json::<Vec<Stats>>(&contents)? {
                data.push(d);
                }
            }
        
        let mut file = File::create(&absolute_path)?;

        data.push(self);

        file.write_all(to_json(&data)?.as_bytes())?;

        Ok(())
        }

    pub fn get_average_pheromone_strengths(&self) -> Vec<f64> {
        self.pheromone_strengths.iter()
            .map(|phero| phero / self.average_returns)
            .collect()
        }
    }