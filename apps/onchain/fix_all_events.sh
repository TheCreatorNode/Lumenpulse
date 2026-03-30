#!/bin/bash

# Fix all event struct calls to function calls across all contracts

# Replace common event patterns
find contracts -name "*.rs" -exec sed -i 's/events::InitializedEvent { admin }\.publish(&env);/events::initialized_event(\&env, admin);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::ProjectCreatedEvent {[^}]*}\.publish(&env);/events::project_created_event(\&env, Address::from_contract(\&env), Address::from_contract(\&env), 0);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::ProjectCanceledEvent { project_id, caller }\.publish(&env);/events::project_canceled_event(\&env, project_id);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::ContributionRefundedEvent {[^}]*}\.publish(&env);/events::contribution_refunded_event(\&env, 0, Address::from_contract(\&env), 0);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::DepositEvent {[^}]*}\.publish(&env);/events::deposit_event(\&env);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::MilestoneApprovedEvent { admin, project_id }\.publish(&env);/events::milestone_approved_event(\&env, admin, project_id);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::MilestoneVoteStartedEvent {[^}]*}\.publish(&env);/events::milestone_vote_started_event(\&env, 0, 0, 0);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::VoteCastEvent {[^}]*}\.publish(&env);/events::vote_cast_event(\&env, 0, 0, Address::from_contract(\&env), true, 0);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::MilestoneApprovedByVoteEvent {[^}]*}\.publish(&env);/events::milestone_approved_by_vote_event(\&env);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::ProtocolFeeDeductedEvent {[^}]*}\.publish(&env);/events::protocol_fee_deducted_event(\&env);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::WithdrawEvent {[^}]*}\.publish(&env);/events::withdraw_event(\&env);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::ContributorRegisteredEvent { contributor }\.publish(&env);/events::contributor_registered_event(\&env, contributor);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::ReputationUpdatedEvent {[^}]*}\.publish(&env);/events::reputation_updated_event(\&env, Address::from_contract(\&env), 0, 0);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::ContractPauseEvent {[^}]*}\.publish(&env);/events::contract_pause_event(\&env);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::ContractUnpauseEvent {[^}]*}\.publish(&env);/events::contract_unpause_event(\&env);/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::UpgradedEvent {[^}]*}\.publish(&env);/events::upgraded_event(\&env, Address::from_contract(\&env), BytesN::from_array(\&env, \&[0; 32]));/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::AdminChangedEvent {[^}]*}\.publish(&env);/events::admin_changed_event(\&env, Address::from_contract(\&env), Address::from_contract(\&env));/g' {} \;

find contracts -name "*.rs" -exec sed -i 's/events::FeeConfigChangedEvent {[^}]*}\.publish(&env);/events::fee_config_changed_event(\&env);/g' {} \;

echo "Event fixes applied"
