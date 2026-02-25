use crate::test_utils::setup_test_env;
use crate::AutoShareContractClient;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, BytesN, Vec};

#[test]
fn test_get_fundraising_progress() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let contributor = test_env.users.get(1).unwrap();
    let member1 = test_env.users.get(2).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();

    let group_id = BytesN::from_array(&test_env.env, &[1u8; 32]);
    test_env.env.mock_all_auths();

    // Setup group
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &creator, 1000);
    client.create(
        &group_id,
        &soroban_sdk::String::from_str(&test_env.env, "Test Group"),
        &creator,
        &10,
        &token,
    );

    let mut members = Vec::new(&test_env.env);
    members.push_back(crate::base::types::GroupMember {
        address: member1.clone(),
        percentage: 100,
    });
    client.update_members(&group_id, &creator, &members);

    // Start fundraising with target 1000
    let target_amount = 1000i128;
    client.start_fundraising(&group_id, &creator, &target_amount);

    // Initially 0%
    let progress = client.get_fundraising_progress(&group_id);
    assert_eq!(progress, 0);

    // Contribute 250 (25%)
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 250);
    client.contribute(&group_id, &token, &250, &contributor);

    let progress = client.get_fundraising_progress(&group_id);
    assert_eq!(progress, 25);

    // Contribute another 500 (total 75%)
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 500);
    client.contribute(&group_id, &token, &500, &contributor);

    let progress = client.get_fundraising_progress(&group_id);
    assert_eq!(progress, 75);

    // Contribute to reach 100%
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 250);
    client.contribute(&group_id, &token, &250, &contributor);

    let progress = client.get_fundraising_progress(&group_id);
    assert_eq!(progress, 100);
}

#[test]
fn test_is_fundraising_target_reached() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let contributor = test_env.users.get(1).unwrap();
    let member1 = test_env.users.get(2).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();

    let group_id = BytesN::from_array(&test_env.env, &[2u8; 32]);
    test_env.env.mock_all_auths();

    // Setup group
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &creator, 1000);
    client.create(
        &group_id,
        &soroban_sdk::String::from_str(&test_env.env, "Test Group"),
        &creator,
        &10,
        &token,
    );

    let mut members = Vec::new(&test_env.env);
    members.push_back(crate::base::types::GroupMember {
        address: member1.clone(),
        percentage: 100,
    });
    client.update_members(&group_id, &creator, &members);

    // Start fundraising
    client.start_fundraising(&group_id, &creator, &1000);

    // Not reached initially
    assert!(!client.is_fundraising_target_reached(&group_id));

    // Contribute partial amount
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 500);
    client.contribute(&group_id, &token, &500, &contributor);
    assert!(!client.is_fundraising_target_reached(&group_id));

    // Reach target
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 500);
    client.contribute(&group_id, &token, &500, &contributor);
    assert!(client.is_fundraising_target_reached(&group_id));
}

#[test]
fn test_get_user_total_contributions() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let contributor = test_env.users.get(1).unwrap();
    let member1 = test_env.users.get(2).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();

    test_env.env.mock_all_auths();

    // Create two groups
    let group_id1 = BytesN::from_array(&test_env.env, &[3u8; 32]);
    let group_id2 = BytesN::from_array(&test_env.env, &[4u8; 32]);

    for group_id in [group_id1.clone(), group_id2.clone()] {
        crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &creator, 1000);
        client.create(
            &group_id,
            &soroban_sdk::String::from_str(&test_env.env, "Test Group"),
            &creator,
            &10,
            &token,
        );

        let mut members = Vec::new(&test_env.env);
        members.push_back(crate::base::types::GroupMember {
            address: member1.clone(),
            percentage: 100,
        });
        client.update_members(&group_id, &creator, &members);
        client.start_fundraising(&group_id, &creator, &1000);
    }

    // Initially 0
    let total = client.get_user_total_contributions(&contributor);
    assert_eq!(total, 0);

    // Contribute to first group
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 300);
    client.contribute(&group_id1, &token, &300, &contributor);

    let total = client.get_user_total_contributions(&contributor);
    assert_eq!(total, 300);

    // Contribute to second group
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 500);
    client.contribute(&group_id2, &token, &500, &contributor);

    let total = client.get_user_total_contributions(&contributor);
    assert_eq!(total, 800);
}

#[test]
fn test_get_contributor_count() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let contributor1 = test_env.users.get(1).unwrap();
    let contributor2 = test_env.users.get(2).unwrap();
    let contributor3 = Address::generate(&test_env.env);
    let member1 = Address::generate(&test_env.env);
    let token = test_env.mock_tokens.get(0).unwrap();

    let group_id = BytesN::from_array(&test_env.env, &[5u8; 32]);
    test_env.env.mock_all_auths();

    // Setup group
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &creator, 1000);
    client.create(
        &group_id,
        &soroban_sdk::String::from_str(&test_env.env, "Test Group"),
        &creator,
        &10,
        &token,
    );

    let mut members = Vec::new(&test_env.env);
    members.push_back(crate::base::types::GroupMember {
        address: member1.clone(),
        percentage: 100,
    });
    client.update_members(&group_id, &creator, &members);
    client.start_fundraising(&group_id, &creator, &1000);

    // Initially 0 contributors
    assert_eq!(client.get_contributor_count(&group_id), 0);

    // First contributor
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor1, 100);
    client.contribute(&group_id, &token, &100, &contributor1);
    assert_eq!(client.get_contributor_count(&group_id), 1);

    // Same contributor again (still 1 unique)
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor1, 100);
    client.contribute(&group_id, &token, &100, &contributor1);
    assert_eq!(client.get_contributor_count(&group_id), 1);

    // Second contributor
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor2, 100);
    client.contribute(&group_id, &token, &100, &contributor2);
    assert_eq!(client.get_contributor_count(&group_id), 2);

    // Third contributor
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor3, 100);
    client.contribute(&group_id, &token, &100, &contributor3);
    assert_eq!(client.get_contributor_count(&group_id), 3);
}

#[test]
fn test_get_fundraising_remaining() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let contributor = test_env.users.get(1).unwrap();
    let member1 = test_env.users.get(2).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();

    let group_id = BytesN::from_array(&test_env.env, &[6u8; 32]);
    test_env.env.mock_all_auths();

    // Setup group
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &creator, 1000);
    client.create(
        &group_id,
        &soroban_sdk::String::from_str(&test_env.env, "Test Group"),
        &creator,
        &10,
        &token,
    );

    let mut members = Vec::new(&test_env.env);
    members.push_back(crate::base::types::GroupMember {
        address: member1.clone(),
        percentage: 100,
    });
    client.update_members(&group_id, &creator, &members);

    // Start fundraising with target 1000
    let target_amount = 1000i128;
    client.start_fundraising(&group_id, &creator, &target_amount);

    // Initially full amount remaining
    assert_eq!(client.get_fundraising_remaining(&group_id), 1000);

    // Contribute 300
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 300);
    client.contribute(&group_id, &token, &300, &contributor);
    assert_eq!(client.get_fundraising_remaining(&group_id), 700);

    // Contribute 700 to reach target
    crate::test_utils::fund_user_with_tokens(&test_env.env, &token, &contributor, 700);
    client.contribute(&group_id, &token, &700, &contributor);
    assert_eq!(client.get_fundraising_remaining(&group_id), 0);
}
