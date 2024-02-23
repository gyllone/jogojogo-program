// use anchor_lang::prelude::*;
//
// #[derive(Accounts)]
// #[instruction(force: [u8; 32])]
// pub struct SpinAndPullTheTrigger<'info> {
//     /// Player will be the `payer` account in the CPI call.
//     #[account(mut)]
//     player: Signer<'info>,
//
//     /// This is the player state account, it is required by Russian-Roulette to store player data
//     // (number of rounds played and info to derive the last round outcome)
//     #[account(
//         init_if_needed,
//         payer = player,
//         space = 8 + PlayerState::SIZE,
//         seeds = [
//         PLAYER_STATE_ACCOUNT_SEED,
//         player.key().as_ref()
//         ],
//         bump
//     )]
//     player_state: Account<'info, PlayerState>,
//
//     /// This account points to the last VRF request, it is necessary to validate that the player
//     /// is alive and is able to play another round.
//     /// CHECK:
//     #[account(
//         seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), player_state.force.as_ref()],
//         bump,
//         seeds::program = VRF_PROGRAM_ID
//     )]
//     prev_round: AccountInfo<'info>,
//
//     /// This account is the current VRF request account, it'll be the `request` account in the CPI call.
//     /// CHECK:
//     #[account(
//         mut,
//         seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &force],
//         bump,
//         seeds::program = VRF_PROGRAM_ID
//     )]
//     random: AccountInfo<'info>,
//
//     /// VRF treasury account, it'll be the `treasury` account in the CPI call.
//     /// CHECK:
//     #[account(mut)]
//     treasury: AccountInfo<'info>,
//     #[account(
//         mut,
//         seeds = [CONFIG_ACCOUNT_SEED.as_ref()],
//         bump,
//         seeds::program = VRF_PROGRAM_ID
//     )]
//
//     /// VRF on-chain state account, it'll be the `network_state` account in the CPI call.
//     config: Account<'info, NetworkState>,
//
//     /// VRF program address to invoke CPI
//     vrf: Program<'info, OraoVrf>,
//
//     /// System program address to create player_state and to be used in CPI call.
//     system_program: Program<'info, System>,
// }