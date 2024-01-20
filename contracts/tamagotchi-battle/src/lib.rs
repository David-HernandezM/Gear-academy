#![no_std]
use gstd::{msg, prelude::*, collections::BTreeMap};
use tamagotchi_battle_io::*;

static mut TAMAGOTCHI_BATTLE: Option<Battle> = None;

#[no_mangle]
extern "C" fn init() {
    let BattleInit { tmg_store_id } = msg::load()
        .expect("Unable to decode CodeId of the Escrow program");
    let tamagotchi_battle = Battle {
        tmg_store_id,
        weapons_data: BTreeMap::from([
            (SWORD_ID, SWORD_POWER),
            (KATANA_ID, KATANA_POWER),
            (AXE_ID, AXE_POWER),
            (MALLET_ID, MALLET_POWER),
            (FORK_ID, FORK_POWER),
            (BOMB_ID, BOMB_POWER),
            (KARATE_ID, KARATE_POWER),
            (BOX_ID, BOX_POWER),
            (SPOON_ID, SPOON_POWER)
        ]),
        shields_data: BTreeMap::from([
            (SHIELD_ID, SHIELD_PROTECTION),
            (JACKET_ID, JACKET_PROTECTION),
            (HELMET_ID, HELMET_PROTECTION)
        ]),
        ..Default::default()
    };
    unsafe { TAMAGOTCHI_BATTLE = Some(tamagotchi_battle) };
}   

#[gstd::async_main]
async fn main() {
    let action: BattleAction = msg::load()
        .expect("Unable to decode `FactoryAction`");
    let tmg_battle = unsafe {
        TAMAGOTCHI_BATTLE.get_or_insert(Default::default())
    };
    
    match action {
        BattleAction::Register {
            tamagotchi_id,
            attributes
        } => {
            tmg_battle.register(&tamagotchi_id, attributes).await;            
        },
        BattleAction::Move(direction) => {
            tmg_battle.make_move(direction);
        },
        BattleAction::UpdateInfo => {
            tmg_battle.update_info().await;
        },
        BattleAction::SendNewAttributesToNextRound {
            new_attributes,
        } => {
            // If the user does not send new attributes, 
            // the previous ones will be used
            tmg_battle.update_tamagotchi_attributes(new_attributes);
        },
        BattleAction::StartNewGame => {
            tmg_battle.reset_contract();
        },
        BattleAction::ReserveGas { 
            reservation_amount, 
            duration 
        } => {
            tmg_battle.make_reservation(reservation_amount, duration);
        }
    }
}

#[no_mangle]
extern fn state() {
    msg::reply(state_ref(), 0)
        .expect("Failed to share state");
}

fn state_ref() -> &'static Battle {
    let state = unsafe { TAMAGOTCHI_BATTLE.as_ref() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() }
}
