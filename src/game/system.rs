use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let new_state = match simulation_state.get() {
            SimulationState::Running => SimulationState::Paused,
            SimulationState::Paused => SimulationState::Running,
        };

        simulation_state_next_state.set(new_state);
        println!("New state is {:?}", new_state);
    }
}
