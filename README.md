# vesting-contract
Requirements:
 - Solana CLI
 - Node Environments

Installation:
 - npm install

Solana program deply
 - anchor build
 - anchor deploy

Test with mocha
 - anchor test

Goal
 - Admin needs to call initialize intruction with reward mint key
 - User needs to call stake instruction with stake amount, lockup and release months no.
 - User needs to call claim instruction for get rewards after lockup months.
 - User can claim with staked amount per every release months.
 - Admin needs to deposit vault token account for provide rewards.
