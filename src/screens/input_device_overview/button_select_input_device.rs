use bevy::prelude::*;
use cpal::traits::DeviceTrait;

use crate::{components::button_primary::{handle_generic_interaction, ButtonPrimary}, resources::{configuration::Configuration, input_device::InputDevice, input_devices::InputDevices}, states::app_state::AppState};

#[derive(Component, Default)]
pub struct SelectInputDeviceButton;

pub fn button_select_input_device_interaction_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonPrimary<String>, &SelectInputDeviceButton), (Changed<Interaction>, With<Button>)>,
    input_devices: Res<InputDevices>,
    input_device: ResMut<InputDevice>,
    mut configuration: ResMut<Configuration>,
    mut next_state: ResMut<NextState<AppState>>
) {
    for (interaction, mut color, button, _marker) in &mut interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let device = input_devices.input_devices.iter().find(|&device| device.name().unwrap() == button.data).unwrap().clone();
            configuration.device = Some(device);
            next_state.set(AppState::InputDeviceDetail);
        } else {
            handle_generic_interaction(interaction, &mut color)
        }
    }
}
