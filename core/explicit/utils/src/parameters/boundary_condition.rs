#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BoundaryCondition {
    #[serde(rename = "Cavity-Flow")]
    CavityFlow = 1,
    #[serde(rename = "Poiseuille-Flow")]
    PoiseuilleFlow = 2,
    #[serde(rename = "Periodic-Flow")]
    PeriodicFlow = 3,
    #[serde(rename = "LidDrivenCavity")]
    LidDrivenCavity = 4,
}
