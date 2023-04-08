#[cfg(test)]
mod tests {

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr};

    use crate::contract::{execute, instantiate};
    use crate::msg::{ExecuteMsg, GetOwnersResponse, InstantiateMsg, QueryMsg};
    use crate::queries::query;
    use crate::state::Owner;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            owners: vec![
                Owner {
                    address: Addr::unchecked("owner1"),
                    ownership: 50,
                },
                Owner {
                    address: Addr::unchecked("owner2"),
                    ownership: 50,
                },
            ],
        };
        let env = mock_env();
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), env.clone(), QueryMsg::GetOwners {}).unwrap();
        let value: GetOwnersResponse = from_binary(&res).unwrap();
        assert_eq!(
            value,
            GetOwnersResponse {
                owners: vec![
                    Owner {
                        address: Addr::unchecked("owner1"),
                        ownership: 50,
                    },
                    Owner {
                        address: Addr::unchecked("owner2"),
                        ownership: 50,
                    },
                ]
            }
        );

        // Owner1 disburse
        let msg = ExecuteMsg::Disburse {};
        let env = mock_env();
        let info = mock_info("owner1", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        println!("ENTIRE RESPONSE: {:?}", res);
        println!("MESSAGES: {:?}", res.messages);
        println!("ATTRIBUTES: {:?}", res.attributes);
        println!("DATA: {:?}", res.data);
        println!("EVENTS: {:?}", res.events);

        let expected_response = "Response { messages: [SubMsg { id: 0, msg: Bank(Send { to_address: \"owner1\", amount: [Coin { denom: \"earth\", amount: Uint128(500) }] }), gas_limit: None, reply_on: Never }, SubMsg { id: 0, msg: Bank(Send { to_address: \"owner2\", amount: [Coin { denom: \"earth\", amount: Uint128(500) }] }), gas_limit: None, reply_on: Never }], attributes: [], events: [], data: None }";
        assert_eq!(expected_response, format!("{:?}", res));
  

        let expected_messages = "[SubMsg { id: 0, msg: Bank(Send { to_address: \"owner1\", amount: [Coin { denom: \"earth\", amount: Uint128(500) }] }), gas_limit: None, reply_on: Never }, SubMsg { id: 0, msg: Bank(Send { to_address: \"owner2\", amount: [Coin { denom: \"earth\", amount: Uint128(500) }] }), gas_limit: None, reply_on: Never }]";
        assert_eq!(expected_messages, format!("{:?}", res.messages));

        // Owner2 disburse
        let msg = ExecuteMsg::Disburse {};
        let env = mock_env();
        let info = mock_info("owner2", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        println!("ENTIRE RESPONSE: {:?}", res);
        println!("MESSAGES: {:?}", res.messages);
        println!("ATTRIBUTES: {:?}", res.attributes);
        println!("DATA: {:?}", res.data);
        println!("EVENTS: {:?}", res.events);

        let expected_response = "Response { messages: [SubMsg { id: 0, msg: Bank(Send { to_address: \"owner1\", amount: [Coin { denom: \"earth\", amount: Uint128(500) }] }), gas_limit: None, reply_on: Never }, SubMsg { id: 0, msg: Bank(Send { to_address: \"owner2\", amount: [Coin { denom: \"earth\", amount: Uint128(500) }] }), gas_limit: None, reply_on: Never }], attributes: [], events: [], data: None }";
        assert_eq!(expected_response, format!("{:?}", res));
  

        let expected_messages = "[SubMsg { id: 0, msg: Bank(Send { to_address: \"owner1\", amount: [Coin { denom: \"earth\", amount: Uint128(500) }] }), gas_limit: None, reply_on: Never }, SubMsg { id: 0, msg: Bank(Send { to_address: \"owner2\", amount: [Coin { denom: \"earth\", amount: Uint128(500) }] }), gas_limit: None, reply_on: Never }]";
        assert_eq!(expected_messages, format!("{:?}", res.messages));
    }
}
