use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "bacdive",
    version = "2.0",
    about = "analyses bacdive data for local analysis.
    ************************************************
    Author Gaurav Sablok
    Email: gsablok@proton.me
    ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Id {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific ID:
        id: String,
        /// threads for the analysis
        threads: String,
    },
    /// please provide the species that need to be searched.
    Species {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific species name. Use the species list with the bacdive file to see the species.
        species: String,
        /// threads for the analysis
        threads: String,
    },
    /// please provide the category2 that you want to look,
    Strain {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// specific strain
        strain: String,
        /// threads for the analysis
        threads: String,
    },
    /// this will list all the available unique ids present in the bacdive
    IdList {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// threads for the analysis
        threads: String,
    },
    /// this will list all the unique species present in the bacdive
    SpeciesList {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// threads for the analysis
        threads: String,
    },
    /// this will list all the available countries in the bacdive
    Strainlist {
        /// please provide the path to the bacdive file
        bacdive: String,
        /// threads for the analysis
        threads: String,
    },
    /// present the list of the unique ids present
    IDListAnalyze {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
        /// threads for the analysis
        threads: String,
    },
    /// provide the species present in the bacdive
    SpeciesListAnalyze {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
        /// threads for the analysis
        threads: String,
    },
    /// provide the designation header present in the bacdive
    DesignationList {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
        /// threads for the analysis
        threads: String,
    },
    /// provide the strain number present in the bacdive
    StrainNumberList {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
        /// threads for the analysis
        threads: String,
    },
    /// provide the strain header present in the bacdive
    StrainheaderList {
        /// provide the path to the bacdive file
        bacdive_analyzer: String,
        /// threads for the analysis
        threads: String,
    },
    /// search for the specific id and json output
    IDSearch {
        /// please provide the path to the bacdive file
        bacdive_analyzer: String,
        /// please provide the specific id that you want to look
        id: Option<String>,
        /// threads for the analysis
        threads: String,
    },
    /// search for the specific species and json output
    SpeciesSearch {
        /// please provide the path to the bacdive file
        bacdive_analyzer: String,
        /// please provide the species name
        species: Option<String>,
        /// threads for the analysis
        threads: String,
    },
    /// search for the specific designation and json output.
    DesignationSearch {
        /// please provide the path to the bacdive file
        bacdive_analyzer: String,
        /// please provide the designation name
        designation: Option<String>,
        /// threads for the analysis
        threads: String,
    },
    /// search for the specific strain and json output
    StrainSearch {
        /// please provide the path to the bacdive file
        bacdive_analyzer: String,
        /// please provide the strain
        strain: Option<String>,
        /// threads for the analysis
        threads: String,
    },
    /// get webmine results from the bacdive
    WebMine {
        /// please provide the id of the specific strain
        strainid: String,
        /// threads for the analysis
        threads: String,
    },
}
