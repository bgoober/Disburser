#[cfg(test)]
mod tests {

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr};

    use crate::contract::{execute, instantiate};
    use crate::msg::{ExecuteMsg, GetOwnersResponse, InstantiateMsg, QueryMsg};
    use crate::queries::query;
    use crate::state::Owner;
    use crate::ContractError;

    #[test]
    fn test_proper_instantiation() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            owners: vec![
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 60,
                },
                Owner {
                    address: Addr::unchecked("owner2"),
                    ownership: 40,
                },
            ],
        };

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        // assert_eq!(0, res.messages.len());
        // assert_eq!(0, res.attributes.len());
        // assert!(matches!(res, ContractError::InvalidTotalOwnership {}));
        println!("RES: {:?}", res);

        // it worked, let's query the state and test the response is what we expect
        let res = query(deps.as_ref(), env.clone(), QueryMsg::GetOwners {}).unwrap();
        let value: GetOwnersResponse = from_binary(&res).unwrap();
        assert_eq!(
            value,
            GetOwnersResponse {
                owners: vec![
                    Owner {
                        address: Addr::unchecked("owner1"),
                        ownership: 60,
                    },
                    Owner {
                        address: Addr::unchecked("owner2"),
                        ownership: 40,
                    },
                ]
            }
        );
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

        let err = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap_err();
        println!("ERR: {:?}", err);
        assert_eq!(err, ContractError::InvalidTotalOwnership {});
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

        let err = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap_err();
        println!("ERR: {:?}", err);
        assert_eq!(err, ContractError::InvalidIndividualOwnership {});
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

        let msg = ExecuteMsg::Disburse {};
        let info = mock_info("unauthorized", &[]);
        let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
        println!("ERR: {:?}", err);
        assert_eq!(err, ContractError::Unauthorized {});
    }

    // todo: test disburse function with integration tests using cw_multi_test
    // after calling disburse, check the owners balances have increased by the correct amount
}
