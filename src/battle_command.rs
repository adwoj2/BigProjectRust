pub enum BattleCommand {
    SelectUnit(UnitRef),
    MoveSelectedUnit(Hex),
    UseAbility {
        caster: UnitRef,
        ability_idx: usize,
        target: Hex,
    },
    CancelAction,
    EndTurn,
}
