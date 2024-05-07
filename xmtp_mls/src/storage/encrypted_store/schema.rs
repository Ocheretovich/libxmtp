// @generated automatically by Diesel CLI.

diesel::table! {
    group_intents (id) {
        id -> Integer,
        kind -> Integer,
        group_id -> Binary,
        data -> Binary,
        state -> Integer,
        payload_hash -> Nullable<Binary>,
        post_commit_data -> Nullable<Binary>,
        publish_attempts -> Integer,
    }
}

diesel::table! {
    group_messages (id) {
        id -> Binary,
        group_id -> Binary,
        decrypted_message_bytes -> Binary,
        sent_at_ns -> BigInt,
        kind -> Integer,
        sender_installation_id -> Binary,
        sender_inbox_id -> Text,
        delivery_status -> Integer,
    }
}

diesel::table! {
    groups (id) {
        id -> Binary,
        created_at_ns -> BigInt,
        membership_state -> Integer,
        installation_last_checked -> BigInt,
        purpose -> Integer,
        added_by_inbox_id -> Text,
    }
}

diesel::table! {
    identity_updates (inbox_id, sequence_id) {
        inbox_id -> Text,
        sequence_id -> BigInt,
        server_timestamp_ns -> BigInt,
        payload -> Binary,
    }
}

diesel::table! {
    openmls_key_store (key_bytes) {
        key_bytes -> Binary,
        value_bytes -> Binary,
    }
}

diesel::table! {
    refresh_state (entity_id, entity_kind) {
        entity_id -> Binary,
        entity_kind -> Integer,
        cursor -> BigInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    group_intents,
    group_messages,
    groups,
    identity_updates,
    openmls_key_store,
    refresh_state,
);
