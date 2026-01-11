#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BoundaryCondition {
    CavityFlow = 1,
    PoiseuilleFlow = 2,
    PeriodicFlow = 3,
    LidDrivenCavity = 4,
}
