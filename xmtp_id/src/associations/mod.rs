mod association_log;
pub mod builder;
mod hashes;
mod member;
mod serialization;
pub mod signature;
mod state;
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
pub mod unsigned_actions;
pub mod unverified;
pub mod verified_signature;

pub use self::association_log::*;
pub use self::hashes::generate_inbox_id;
pub use self::member::{Member, MemberIdentifier, MemberKind};
pub use self::serialization::{map_vec, try_map_vec, DeserializationError};
pub use self::signature::*;
pub use self::state::{AssociationState, AssociationStateDiff};

// Apply a single IdentityUpdate to an existing AssociationState
pub fn apply_update(
    initial_state: AssociationState,
    update: IdentityUpdate,
) -> Result<AssociationState, AssociationError> {
    update.update_state(Some(initial_state), update.client_timestamp_ns)
}

// Get the current state from an array of `IdentityUpdate`s. Entire operation fails if any operation fails
pub fn get_state<Updates: AsRef<[IdentityUpdate]>>(
    updates: Updates,
) -> Result<AssociationState, AssociationError> {
    let mut state = None;
    for update in updates.as_ref().iter() {
        let res = update.update_state(state, update.client_timestamp_ns);
        state = Some(res?);
    }

    state.ok_or(AssociationError::NotCreated)
}

#[cfg(any(test, feature = "test-utils"))]
pub mod test_defaults {
    use self::{
        test_utils::{rand_string, rand_u64, rand_vec},
        unverified::{UnverifiedAction, UnverifiedIdentityUpdate},
        verified_signature::VerifiedSignature,
    };
    use super::*;

    impl IdentityUpdate {
        pub fn new_test(actions: Vec<Action>, inbox_id: String) -> Self {
            Self::new(actions, inbox_id, rand_u64())
        }
    }

    impl UnverifiedIdentityUpdate {
        pub fn new_test(actions: Vec<UnverifiedAction>, inbox_id: String) -> Self {
            Self::new(inbox_id, rand_u64(), actions)
        }
    }

    impl Default for AddAssociation {
        fn default() -> Self {
            let existing_member = rand_string();
            let new_member = rand_vec();
            Self {
                existing_member_signature: VerifiedSignature::new(
                    existing_member.into(),
                    SignatureKind::Erc191,
                    rand_vec(),
                ),
                new_member_signature: VerifiedSignature::new(
                    new_member.clone().into(),
                    SignatureKind::InstallationKey,
                    rand_vec(),
                ),
                new_member_identifier: new_member.into(),
            }
        }
    }

    // Default will create an inbox with a ERC-191 signature
    impl Default for CreateInbox {
        fn default() -> Self {
            let signer = rand_string();
            Self {
                nonce: rand_u64(),
                account_address: signer.clone(),
                initial_address_signature: VerifiedSignature::new(
                    signer.into(),
                    SignatureKind::Erc191,
                    rand_vec(),
                ),
            }
        }
    }

    impl Default for RevokeAssociation {
        fn default() -> Self {
            let signer = rand_string();
            Self {
                recovery_address_signature: VerifiedSignature::new(
                    signer.into(),
                    SignatureKind::Erc191,
                    rand_vec(),
                ),
                revoked_member: rand_string().into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use self::{
        test_utils::{rand_string, rand_vec},
        verified_signature::VerifiedSignature,
    };
    use super::*;

    pub async fn new_test_inbox() -> AssociationState {
        let create_request = CreateInbox::default();
        let inbox_id = generate_inbox_id(&create_request.account_address, &create_request.nonce);
        let identity_update =
            IdentityUpdate::new_test(vec![Action::CreateInbox(create_request)], inbox_id);

        get_state(vec![identity_update]).unwrap()
    }

    pub async fn new_test_inbox_with_installation() -> AssociationState {
        let initial_state = new_test_inbox().await;
        let inbox_id = initial_state.inbox_id().clone();
        let initial_wallet_address: MemberIdentifier =
            initial_state.recovery_address().clone().into();

        let update = Action::AddAssociation(AddAssociation {
            existing_member_signature: VerifiedSignature::new(
                initial_wallet_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            ..Default::default()
        });

        apply_update(
            initial_state,
            IdentityUpdate::new_test(vec![update], inbox_id.clone()),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_create_inbox() {
        let create_request = CreateInbox::default();
        let inbox_id = generate_inbox_id(&create_request.account_address, &create_request.nonce);
        let account_address = create_request.account_address.clone();
        let identity_update =
            IdentityUpdate::new_test(vec![Action::CreateInbox(create_request)], inbox_id.clone());
        let state = get_state(vec![identity_update]).unwrap();
        assert_eq!(state.members().len(), 1);

        let existing_entity = state.get(&account_address.clone().into()).unwrap();
        assert!(existing_entity.identifier.eq(&account_address.into()));
    }

    #[tokio::test]
    async fn create_and_add_separately() {
        let initial_state = new_test_inbox().await;
        let inbox_id = initial_state.inbox_id().clone();
        let new_installation_identifier: MemberIdentifier = rand_vec().into();
        let first_member: MemberIdentifier = initial_state.recovery_address().clone().into();

        let update = Action::AddAssociation(AddAssociation {
            new_member_identifier: new_installation_identifier.clone(),
            new_member_signature: VerifiedSignature::new(
                new_installation_identifier.clone(),
                SignatureKind::InstallationKey,
                rand_vec(),
            ),
            existing_member_signature: VerifiedSignature::new(
                first_member.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
        });

        let new_state = apply_update(
            initial_state,
            IdentityUpdate::new_test(vec![update], inbox_id.clone()),
        )
        .unwrap();
        assert_eq!(new_state.members().len(), 2);

        let new_member = new_state.get(&new_installation_identifier).unwrap();
        assert_eq!(new_member.added_by_entity, Some(first_member));
    }

    #[tokio::test]
    async fn create_and_add_together() {
        let create_action = CreateInbox::default();
        let account_address = create_action.account_address.clone();
        let inbox_id = generate_inbox_id(&account_address, &create_action.nonce);
        let new_member_identifier: MemberIdentifier = rand_vec().into();
        let add_action = AddAssociation {
            existing_member_signature: VerifiedSignature::new(
                account_address.clone().into(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            // Add an installation ID
            new_member_signature: VerifiedSignature::new(
                new_member_identifier.clone(),
                SignatureKind::InstallationKey,
                rand_vec(),
            ),
            new_member_identifier: new_member_identifier.clone(),
        };
        let identity_update = IdentityUpdate::new_test(
            vec![
                Action::CreateInbox(create_action),
                Action::AddAssociation(add_action),
            ],
            inbox_id.clone(),
        );
        let state = get_state(vec![identity_update]).unwrap();
        assert_eq!(state.members().len(), 2);
        assert_eq!(
            state.get(&new_member_identifier).unwrap().added_by_entity,
            Some(account_address.into())
        );
    }

    #[tokio::test]
    async fn create_from_legacy_key() {
        let member_identifier: MemberIdentifier = rand_string().into();
        let create_action = CreateInbox {
            nonce: 0,
            account_address: member_identifier.to_string(),
            initial_address_signature: VerifiedSignature::new(
                member_identifier.clone(),
                SignatureKind::LegacyDelegated,
                "0".as_bytes().to_vec(),
            ),
        };
        let inbox_id = generate_inbox_id(&member_identifier.to_string(), &0);
        let state = get_state(vec![IdentityUpdate::new_test(
            vec![Action::CreateInbox(create_action)],
            inbox_id.clone(),
        )])
        .unwrap();
        assert_eq!(state.members().len(), 1);

        // The legacy key can only be used once. After this, subsequent updates should fail
        let update = Action::AddAssociation(AddAssociation {
            existing_member_signature: VerifiedSignature::new(
                member_identifier,
                SignatureKind::LegacyDelegated,
                // All requests from the same legacy key will have the same signature nonce
                "0".as_bytes().to_vec(),
            ),
            ..Default::default()
        });
        let update_result = apply_update(
            state,
            IdentityUpdate::new_test(vec![update], inbox_id.clone()),
        );
        assert!(matches!(update_result, Err(AssociationError::Replay)));
    }

    #[tokio::test]
    async fn add_wallet_from_installation_key() {
        let initial_state = new_test_inbox_with_installation().await;
        let inbox_id = initial_state.inbox_id().clone();
        let installation_id = initial_state
            .members_by_kind(MemberKind::Installation)
            .first()
            .cloned()
            .unwrap()
            .identifier;

        let new_wallet_address: MemberIdentifier = rand_string().into();
        let add_association = Action::AddAssociation(AddAssociation {
            new_member_identifier: new_wallet_address.clone(),
            new_member_signature: VerifiedSignature::new(
                new_wallet_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            existing_member_signature: VerifiedSignature::new(
                installation_id.clone(),
                SignatureKind::InstallationKey,
                rand_vec(),
            ),
        });

        let new_state = apply_update(
            initial_state,
            IdentityUpdate::new_test(vec![add_association], inbox_id.clone()),
        )
        .expect("expected update to succeed");
        assert_eq!(new_state.members().len(), 3);
    }

    #[tokio::test]
    async fn reject_invalid_signature_on_create() {
        // Creates a signature with the wrong signer
        let bad_signature =
            VerifiedSignature::new(rand_string().into(), SignatureKind::Erc191, rand_vec());
        let action = CreateInbox {
            initial_address_signature: bad_signature,
            ..Default::default()
        };

        let state_result = get_state(vec![IdentityUpdate::new_test(
            vec![Action::CreateInbox(action)],
            rand_string(),
        )]);

        assert!(state_result.is_err());
        assert!(matches!(
            state_result,
            Err(AssociationError::MissingExistingMember)
        ));
    }

    #[tokio::test]
    async fn reject_invalid_signature_on_update() {
        let initial_state = new_test_inbox().await;
        let inbox_id = initial_state.inbox_id().clone();
        // Signature is from a random address
        let bad_signature =
            VerifiedSignature::new(rand_string().into(), SignatureKind::Erc191, rand_vec());

        let update_with_bad_existing_member = Action::AddAssociation(AddAssociation {
            existing_member_signature: bad_signature.clone(),
            ..Default::default()
        });

        let update_result = apply_update(
            initial_state.clone(),
            IdentityUpdate::new_test(vec![update_with_bad_existing_member], inbox_id.clone()),
        );

        assert!(matches!(
            update_result,
            Err(AssociationError::MissingExistingMember)
        ));

        let update_with_bad_new_member = Action::AddAssociation(AddAssociation {
            new_member_signature: bad_signature.clone(),
            existing_member_signature: VerifiedSignature::new(
                initial_state.recovery_address().clone().into(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            ..Default::default()
        });

        let update_result_2 = apply_update(
            initial_state,
            IdentityUpdate::new_test(vec![update_with_bad_new_member], inbox_id.clone()),
        );
        assert!(matches!(
            update_result_2,
            Err(AssociationError::NewMemberIdSignatureMismatch)
        ));
    }

    #[tokio::test]
    async fn reject_if_signer_not_existing_member() {
        let create_inbox = CreateInbox::default();
        let inbox_id = generate_inbox_id(&create_inbox.account_address, &create_inbox.nonce);
        let create_request = Action::CreateInbox(create_inbox);
        // The default here will create an AddAssociation from a random wallet
        let update = Action::AddAssociation(AddAssociation {
            // Existing member signature is coming from a random wallet
            existing_member_signature: VerifiedSignature::new(
                rand_string().into(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            ..Default::default()
        });

        let state_result = get_state(vec![IdentityUpdate::new_test(
            vec![create_request, update],
            inbox_id.clone(),
        )]);
        assert!(matches!(
            state_result,
            Err(AssociationError::MissingExistingMember)
        ));
    }

    #[tokio::test]
    async fn reject_if_installation_adding_installation() {
        let existing_state = new_test_inbox_with_installation().await;
        let inbox_id = existing_state.inbox_id().clone();
        let existing_installations = existing_state.members_by_kind(MemberKind::Installation);
        let existing_installation = existing_installations.first().unwrap();
        let new_installation_id: MemberIdentifier = rand_vec().into();

        let update = Action::AddAssociation(AddAssociation {
            existing_member_signature: VerifiedSignature::new(
                existing_installation.identifier.clone(),
                SignatureKind::InstallationKey,
                rand_vec(),
            ),
            new_member_identifier: new_installation_id.clone(),
            new_member_signature: VerifiedSignature::new(
                new_installation_id.clone(),
                SignatureKind::InstallationKey,
                rand_vec(),
            ),
        });

        let update_result = apply_update(
            existing_state,
            IdentityUpdate::new_test(vec![update], inbox_id.clone()),
        );
        assert!(matches!(
            update_result,
            Err(AssociationError::MemberNotAllowed(
                MemberKind::Installation,
                MemberKind::Installation
            ))
        ));
    }

    #[tokio::test]
    async fn revoke() {
        let initial_state = new_test_inbox_with_installation().await;
        let inbox_id = initial_state.inbox_id().clone();
        let installation_id = initial_state
            .members_by_kind(MemberKind::Installation)
            .first()
            .cloned()
            .unwrap()
            .identifier;
        let update = Action::RevokeAssociation(RevokeAssociation {
            recovery_address_signature: VerifiedSignature::new(
                initial_state.recovery_address().clone().into(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            revoked_member: installation_id.clone(),
        });

        let new_state = apply_update(
            initial_state,
            IdentityUpdate::new_test(vec![update], inbox_id.clone()),
        )
        .expect("expected update to succeed");
        assert!(new_state.get(&installation_id).is_none());
    }

    #[tokio::test]
    async fn revoke_children() {
        let initial_state = new_test_inbox_with_installation().await;
        let inbox_id = initial_state.inbox_id().clone();
        let wallet_address = initial_state
            .members_by_kind(MemberKind::Address)
            .first()
            .cloned()
            .unwrap()
            .identifier;

        let add_second_installation = Action::AddAssociation(AddAssociation {
            existing_member_signature: VerifiedSignature::new(
                wallet_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            ..Default::default()
        });

        let new_state = apply_update(
            initial_state,
            IdentityUpdate::new_test(vec![add_second_installation], inbox_id.clone()),
        )
        .expect("expected update to succeed");
        assert_eq!(new_state.members().len(), 3);

        let revocation = Action::RevokeAssociation(RevokeAssociation {
            recovery_address_signature: VerifiedSignature::new(
                wallet_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            revoked_member: wallet_address.clone(),
        });

        // With this revocation the original wallet + both installations should be gone
        let new_state = apply_update(
            new_state,
            IdentityUpdate::new_test(vec![revocation], inbox_id.clone()),
        )
        .expect("expected update to succeed");
        assert_eq!(new_state.members().len(), 0);
    }

    #[tokio::test]
    async fn revoke_and_re_add() {
        let initial_state = new_test_inbox().await;
        let wallet_address = initial_state
            .members_by_kind(MemberKind::Address)
            .first()
            .cloned()
            .unwrap()
            .identifier;

        let inbox_id = initial_state.inbox_id().clone();

        let second_wallet_address: MemberIdentifier = rand_string().into();
        let add_second_wallet = Action::AddAssociation(AddAssociation {
            new_member_identifier: second_wallet_address.clone(),
            new_member_signature: VerifiedSignature::new(
                second_wallet_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            existing_member_signature: VerifiedSignature::new(
                wallet_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
        });

        let revoke_second_wallet = Action::RevokeAssociation(RevokeAssociation {
            recovery_address_signature: VerifiedSignature::new(
                wallet_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            revoked_member: second_wallet_address.clone(),
        });

        let state_after_remove = apply_update(
            initial_state,
            IdentityUpdate::new_test(
                vec![add_second_wallet, revoke_second_wallet],
                inbox_id.clone(),
            ),
        )
        .expect("expected update to succeed");
        assert_eq!(state_after_remove.members().len(), 1);

        let add_second_wallet_again = Action::AddAssociation(AddAssociation {
            new_member_identifier: second_wallet_address.clone(),
            new_member_signature: VerifiedSignature::new(
                second_wallet_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            existing_member_signature: VerifiedSignature::new(
                wallet_address,
                SignatureKind::Erc191,
                rand_vec(),
            ),
        });

        let state_after_re_add = apply_update(
            state_after_remove,
            IdentityUpdate::new_test(vec![add_second_wallet_again], inbox_id.clone()),
        )
        .expect("expected update to succeed");
        assert_eq!(state_after_re_add.members().len(), 2);
    }

    #[tokio::test]
    async fn change_recovery_address() {
        let initial_state = new_test_inbox_with_installation().await;
        let inbox_id = initial_state.inbox_id().clone();
        let initial_recovery_address: MemberIdentifier =
            initial_state.recovery_address().clone().into();
        let new_recovery_address = rand_string();
        let update_recovery = Action::ChangeRecoveryAddress(ChangeRecoveryAddress {
            new_recovery_address: new_recovery_address.clone(),
            recovery_address_signature: VerifiedSignature::new(
                initial_state.recovery_address().clone().into(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
        });

        let new_state = apply_update(
            initial_state,
            IdentityUpdate::new_test(vec![update_recovery], inbox_id.clone()),
        )
        .expect("expected update to succeed");
        assert_eq!(new_state.recovery_address(), &new_recovery_address);

        let attempted_revoke = Action::RevokeAssociation(RevokeAssociation {
            recovery_address_signature: VerifiedSignature::new(
                initial_recovery_address.clone(),
                SignatureKind::Erc191,
                rand_vec(),
            ),
            revoked_member: initial_recovery_address.clone(),
        });

        let revoke_result = apply_update(
            new_state,
            IdentityUpdate::new_test(vec![attempted_revoke], inbox_id.clone()),
        );
        assert!(revoke_result.is_err());
        assert!(matches!(
            revoke_result,
            Err(AssociationError::MissingExistingMember)
        ));
    }
}
