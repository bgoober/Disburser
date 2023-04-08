#[cfg(test)]
mod tests {

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr, StdError};
    use serde::{Serialize, Deserialize};

    use crate::contract::{execute, instantiate};
    use crate::msg::{ExecuteMsg, GetConfigResponse, InstantiateMsg, QueryMsg};
    use crate::queries::query;
    use crate::state::Owner;

    #[test]
    fn test_proper_instantiation() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            owners: vec![
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 45,
                },
                Owner {
                    address: Addr::unchecked("owner2"),
                    ownership: 55,
                },
            ],
        };
        let env = mock_env();
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(1, res.attributes.len());

        // it worked, let's query the state and test the response is what we expect
        let res = query(deps.as_ref(), env.clone(), QueryMsg::GetConfig {}).unwrap();
        let value: GetConfigResponse = from_binary(&res).unwrap();
        assert_eq!(
            value,
            GetConfigResponse {
                owners: vec![
                    Owner {
                        address: Addr::unchecked("owner1"),
                        ownership: 45,
                    },
                    Owner {
                        address: Addr::unchecked("owner2"),
                        ownership: 55,
                    },
                ]
            }
        );
    }

    #[test]
    fn test_disburse_messages() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            owners: vec![
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 45,
                },
                Owner {
                    address: Addr::unchecked("owner2"),
                    ownership: 55,
                },
            ],
        };

        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Owner1 disburse
        let msg = ExecuteMsg::Disburse {};
        let info = mock_info("owner1", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        // println!("ENTIRE RESPONSE: {:?}", res);
        // println!("MESSAGES: {:?}", res.messages);

        let expected_response = "Response { messages: [SubMsg { id: 0, msg: Bank(Send { to_address: \"owner1\", amount: [Coin { denom: \"earth\", amount: Uint128(450) }] }), gas_limit: None, reply_on: Never }, SubMsg { id: 0, msg: Bank(Send { to_address: \"owner2\", amount: [Coin { denom: \"earth\", amount: Uint128(550) }] }), gas_limit: None, reply_on: Never }], attributes: [Attribute { key: \"disbursed_by\", value: \"owner1\" }], events: [], data: None }";
        assert_eq!(expected_response, format!("{:?}", res));

        let expected_messages = "[SubMsg { id: 0, msg: Bank(Send { to_address: \"owner1\", amount: [Coin { denom: \"earth\", amount: Uint128(450) }] }), gas_limit: None, reply_on: Never }, SubMsg { id: 0, msg: Bank(Send { to_address: \"owner2\", amount: [Coin { denom: \"earth\", amount: Uint128(550) }] }), gas_limit: None, reply_on: Never }]";
        assert_eq!(expected_messages, format!("{:?}", res.messages));

        assert_eq!(1, res.attributes.len());
        assert_eq!(2, res.messages.len());
        assert_eq!(0, res.events.len());

        // Owner2 disburse
        let msg = ExecuteMsg::Disburse {};
        let info = mock_info("owner2", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        // println!("ENTIRE RESPONSE: {:?}", res);
        // println!("MESSAGES: {:?}", res.messages);

        let expected_response = "Response { messages: [SubMsg { id: 0, msg: Bank(Send { to_address: \"owner1\", amount: [Coin { denom: \"earth\", amount: Uint128(450) }] }), gas_limit: None, reply_on: Never }, SubMsg { id: 0, msg: Bank(Send { to_address: \"owner2\", amount: [Coin { denom: \"earth\", amount: Uint128(550) }] }), gas_limit: None, reply_on: Never }], attributes: [Attribute { key: \"disbursed_by\", value: \"owner2\" }], events: [], data: None }";
        assert_eq!(expected_response, format!("{:?}", res));

        let expected_messages = "[SubMsg { id: 0, msg: Bank(Send { to_address: \"owner1\", amount: [Coin { denom: \"earth\", amount: Uint128(450) }] }), gas_limit: None, reply_on: Never }, SubMsg { id: 0, msg: Bank(Send { to_address: \"owner2\", amount: [Coin { denom: \"earth\", amount: Uint128(550) }] }), gas_limit: None, reply_on: Never }]";
        assert_eq!(expected_messages, format!("{:?}", res.messages));

        assert_eq!(1, res.attributes.len());
        assert_eq!(2, res.messages.len());
        assert_eq!(0, res.events.len());
    }

    #[test]
    fn test_instantiate_total_ownership_error() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            owners: vec![
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 40,
                },
                Owner {
                    address: Addr::unchecked("owner2"),
                    ownership: 50,
                },
            ],
        };

        // Check that the `Total Ownership must equal 100%` error is thrown when the total ownership is greater than 100
        let res = instantiate(deps.as_mut(), env, info.clone(), msg);
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            cosmwasm_std::StdError::generic_err("Total Ownership must equal 100%."),
        );
    }

    #[test]
    fn test_instantiate_duplicate_owner_error() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            owners: vec![
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 50,
                },
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 50,
                },
            ],
        };

        // Check that the `Total Ownership must equal 100%` error is thrown when the total ownership is greater than 100
        let res = instantiate(deps.as_mut(), env, info.clone(), msg);
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            cosmwasm_std::StdError::generic_err(
                "Duplicate owner address has been input more than once"
            ),
        );
    }

    #[test]
    fn test_instantiate_0_ownership_error() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            owners: vec![
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 100,
                },
                Owner {
                    address: Addr::unchecked("owner2"),
                    ownership: 0,
                },
            ],
        };

        // Check that the `Total Ownership must equal 100%` error is thrown when the total ownership is greater than 100
        let res = instantiate(deps.as_mut(), env, info.clone(), msg);
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            cosmwasm_std::StdError::generic_err("Individual Ownership must be greater than 0."),
        );
    }

    #[test]
    fn test_unauthorized_error() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            owners: vec![
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 45,
                },
                Owner {
                    address: Addr::unchecked("owner2"),
                    ownership: 55,
                },
            ],
        };

        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
        struct ContractError(pub String);
        
        // unauthorized disburse
        let msg = ExecuteMsg::Disburse {};
        let info = mock_info("unauthorized", &coins(1000, "earth"));
        let error = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
        assert_eq!(error, StdError::generic_err("Unauthorized to disburse funds.").into());
    }
}
