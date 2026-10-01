use bacdive::args::CommandParse;
use bacdive::args::Commands;
use bacdive::designation::bacdivedesignationsearch;
use bacdive::designationlist::designation;
use bacdive::idlist::idlist;
use bacdive::idsearch::bacdiveidsearch;
use bacdive::idwrite::id_write;
use bacdive::species::bacdivespeciessearch;
use bacdive::specieslist::species;
use bacdive::specieswrite::species_write;
use bacdive::strain::bacdivestrainsearch;
use bacdive::strainheader::strainheader;
use bacdive::strainnumber::strainnumber;
use bacdive::strainwrite::strain_write;
use bacdive::uniqueid::unique_id;
use bacdive::uniquespecies::unique_species;
use bacdive::uniquestrain::unique_strain;
use bacdive::webmine::webminer;
use clap::Parser;
use figlet_rs::FIGfont;
use std::collections::HashSet;

/*
Gaurav Sablok
gsablok@proton.me
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("bacDIVE");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let bacdiveargs = CommandParse::parse();
    match &bacdiveargs.command {
        Commands::Id {
            bacdive,
            id,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput = id_write(bacdive, id).unwrap();
                println!("The ids are: {:?}", commandoutput);
            });
        }
        Commands::Species {
            bacdive,
            species,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput = species_write(bacdive, species).unwrap();
                println!(
                    "The species and the associated information are: {:?}",
                    commandoutput
                );
            });
        }
        Commands::Strain {
            bacdive,
            strain,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput = strain_write(bacdive, strain).unwrap();
                println!(
                    "The strain specific information are as follows:{:?}",
                    commandoutput
                );
            });
        }
        Commands::IdList { bacdive, threads } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput = unique_id(bacdive).unwrap();
                println!("The category2 searches are: {:?}", commandoutput);
            });
        }
        Commands::SpeciesList { bacdive, threads } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput = unique_species(bacdive).unwrap();
                println!("The category2 searches are: {:?}", commandoutput);
            });
        }
        Commands::Strainlist { bacdive, threads } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput = unique_strain(bacdive).unwrap();
                println!("The category2 searches are: {:?}", commandoutput);
            });
        }
        Commands::IDListAnalyze {
            bacdive_analyzer,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput: HashSet<String> = idlist(bacdive_analyzer).unwrap();
                println!("The ids present in the bacdive are: {:?}", commandoutput);
            });
        }
        Commands::SpeciesListAnalyze {
            bacdive_analyzer,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput: HashSet<String> = species(bacdive_analyzer).unwrap();
                println!(
                    "The species present in the bacdive are: {:?}",
                    commandoutput
                );
            });
        }
        Commands::DesignationList {
            bacdive_analyzer,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput: HashSet<String> = designation(bacdive_analyzer).unwrap();
                println!(
                    "The designation species present in the bacdive are: {:?}",
                    commandoutput
                );
            });
        }
        Commands::StrainNumberList {
            bacdive_analyzer,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput: HashSet<String> = strainnumber(bacdive_analyzer).unwrap();
                println!(
                    "The strain number are as follows for the species in the bacdive:{:?}",
                    commandoutput
                );
            });
        }
        Commands::StrainheaderList {
            bacdive_analyzer,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let commandoutput: HashSet<String> = strainheader(bacdive_analyzer).unwrap();
                println!(
                    "The strain header are as follows for the bacdive:{:?}",
                    commandoutput
                );
            });
        }
        Commands::IDSearch {
            bacdive_analyzer,
            id,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(||{
            let commandoutput = bacdiveidsearch(bacdive_analyzer, id.clone()).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
            });
        }
        Commands::SpeciesSearch {
            bacdive_analyzer,
            species,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(||{
            let commandoutput = bacdivespeciessearch(bacdive_analyzer, species.clone()).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
            })
        }
        Commands::DesignationSearch {
            bacdive_analyzer,
            designation,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(||{
            let commandoutput =
                bacdivedesignationsearch(bacdive_analyzer, designation.clone()).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
            });
        }
        Commands::StrainSearch {
            bacdive_analyzer,
            strain,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(||{
            let commandoutput = bacdivestrainsearch(bacdive_analyzer, strain.clone()).unwrap();
            for i in commandoutput.iter() {
                println!(
                "The id of the species is:{:?}\nThe species number is {:?}\nThe designation header is: {:?}\n",i.id, i.species, i.speciesinformation
            );
            }
            });
        }
        Commands::WebMine { strainid, threads } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = webminer(strainid).unwrap();
                println!("The command has finished:{}", command);
            });
        }
    }
}
