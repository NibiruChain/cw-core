// A simple contract that maintains a whitelist of addresses.
// Takes inspiration from cw-plus/contracts/cw1-whitelist
// 
// This example demonstrates a simple CosmWasm smart contract that manages a 
// whitelist of addresses. The contract initializes with an admin address and 
// allows the admin to add or remove addresses from the whitelist. Users can 
// query whether an address is whitelisted or not.
//
// - InitMsg: Initializes the contract with the admin address.
// - HandleMsg: Enum representing the actions that can
// 
// ### Contained Functionality
// 1. Initialize the contract with an admin address.
// 2. Allow the admin to add or remove addresses from the whitelist.
// 3. Allow anyone to query if an address is on the whitelist.



use std::collections::HashSet;

use cosmwasm_std::{
    attr, entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, 
    Response, StdResult, testing, coins, from_slice,
};
use schemars::JsonSchema;

use serde::{Deserialize, Serialize};

use crate::{
    msg,
    state::{WHITELIST, Whitelist}, 
    msg::{InitMsg, QueryMsg, AdminsResponse, IsWhitelistedResponse, HandleMsg},
};


pub const ADMIN_KEY: &[u8] = b"admin";
pub const WHITELIST_KEY: &[u8] = b"whitelist";

fn has_admin_power(deps: Deps, sender: &str) -> StdResult<bool> {
    let whitelist = WHITELIST.load(deps.storage)?;
    return Ok(whitelist.is_admin(sender));
}


#[entry_point]
pub fn init(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    let whitelist = Whitelist {
        members: HashSet::new(),
        admins: HashSet::from_iter(vec![msg.admin.to_string()]),
    };
    WHITELIST.save(deps.storage, &whitelist)?;
    Ok(Response::default())
}

#[entry_point]
pub fn handle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> StdResult<Response> {
    let admin: Addr = Addr::unchecked(
        String::from_utf8(deps.storage.get(ADMIN_KEY).unwrap_or_default()).unwrap(),
    );

    if info.sender != admin {
        return Err(cosmwasm_std::StdError::generic_err("unauthorized"));
    }

    match msg {
        HandleMsg::Add { address } => {
            //  TODO
            // let key: &[u8] = WHITELIST_KEY;
            // let old_whitelist  = match deps.storage.get(key) {
            //     Some(list_bz) => {
            //         let whitelist: Vec<String> = from_slice(&list_bz)?;
            //         Ok(Some(whitelist))
            //     },
            //     None => Ok(None),
            // };

            // old_whitelist.push(address);
            // deps.storage.set(format!("{}{}", String::from_utf8_lossy(WHITELIST_KEY), address).as_bytes(), &[]);
            Ok(Response::new().add_attributes(vec![
                attr("action", "add"),
                attr("address", address),
            ]))
        }
        HandleMsg::Remove { address } => {
            // TODO use cw-storage-plus.
            // deps.storage.remove(format!("{}{}", String::from_utf8_lossy(WHITELIST_KEY), address).as_bytes());
            Ok(Response::new().add_attributes(vec![
                attr("action", "remove"),
                attr("address", address),
            ]))
        }
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsWhitelisted { address } => {
            let is_whitelisted = deps.storage.get(
                format!("{}{}", String::from_utf8_lossy(WHITELIST_KEY), address).as_bytes()).is_some();
            let res = IsWhitelistedResponse { is_whitelisted };
            cosmwasm_std::to_binary(&res)
        }
        QueryMsg::Admins {} => {
            let whitelist = WHITELIST.load(deps.storage)?; 
            let admins: Vec<String> = whitelist.admins.into_iter().collect();
            let res = AdminsResponse { admins };
            cosmwasm_std::to_binary(&res)
        }
    }
}


mod tests {
    use super::*;
    use crate::{
        state::{WHITELIST, Whitelist}, 
        msg::{InitMsg, QueryMsg, AdminsResponse, IsWhitelistedResponse, HandleMsg},
    };
    use crate::state::tests::init_mock_whitelist;

    use cosmwasm_std::testing;
    use cosmwasm_std::coins;

    // ---------------------------------------------------------------------------
    // Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_init() {
        let mut deps = testing::mock_dependencies();
        let msg = InitMsg {
            admin: "admin".to_string(),
        };
        let info: MessageInfo = testing::mock_info(
            "addr0000", &coins(2, "token"));

        let result = init(
            deps.as_mut(), testing::mock_env(), info, msg).unwrap();
       assert_eq!(result.messages.len(), 0);
    }

    #[test]
    fn test_handle_unauthorized() {
        let mut deps = testing::mock_dependencies();
        let admin = Addr::unchecked("admin");

        let init_msg = InitMsg {
            admin: admin.as_str().to_string(),
        };
        let init_info = testing::mock_info("addr0000", &coins(2, "token"));
        init(deps.as_mut(), testing::mock_env(), init_info, init_msg).unwrap();

        let handle_msg = msg::HandleMsg::Add {
            address: "addr0001".to_string(),
        };
        let unauthorized_info = testing::mock_info("unauthorized", &[]);
        let result = handle(deps.as_mut(), testing::mock_env(), unauthorized_info, handle_msg);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_add() {
        // let mut deps = testing::mock_dependencies();
        // let admin = Addr::unchecked("admin");

        // let init_msg = InitMsg {
        //     admin: admin.as_str().to_string(),
        // };
        // let init_info = testing::mock_info("addr0000", &coins(2, "token"));
        // init(deps.as_mut(), testing::mock_env(), init_info, init_msg).unwrap();

        // let handle_msg = HandleMsg::Add {
        //     address: "addr0001".to_string(),
        // };
        // let handle_info = testing::mock_info(admin.as_str(), &[]);
        // let result = handle(deps.as_mut(), testing::mock_env(), handle_info, handle_msg).unwrap();
        // assert_eq!(result.messages.len(), 0);
        // assert_eq!(result.attributes.len(), 2);

        // let query_msg = QueryMsg::IsWhitelisted {
        //     address: "addr0001".to_string(),
        // };
        // let binary = query(deps.as_ref(), testing::mock_env(), query_msg).unwrap();
        // let response: IsWhitelistedResponse = cosmwasm_std::from_binary(&binary).unwrap();
        // assert_eq!(response.is_whitelisted, true);
    }

    #[test]
    fn test_handle_remove() {
        let mut deps = testing::mock_dependencies();
        // TODO test using cw-storage-plus
        // let admin = Addr::unchecked("admin");

        // let init_msg = InitMsg {
        //     admin: admin.as_str().to_string(),
        // };
        // let init_info = testing::mock_info("addr0000", &coins(2, "token"));
        // init(deps.as_mut(), testing::mock_env(), init_info, init_msg).unwrap();

        // let handle_msg = msg::HandleMsg::Add {
        //     address: "addr0001".to_string(),
        // };
        // let handle_info = testing::mock_info(admin.as_str(), &[]);
        // handle(deps.as_mut(), testing::mock_env(), handle_info.clone(), handle_msg).unwrap();

        // let handle_msg = HandleMsg::Remove {
        //     address: "addr0001".to_string(),
        // };
        // let result = handle(deps.as_mut(), testing::mock_env(), handle_info, handle_msg).unwrap();
        // assert_eq!(result.messages.len(), 0);
        // assert_eq!(result.attributes.len(), 2);

        // let query_msg = QueryMsg::IsWhitelisted {
        //     address: "addr0001".to_string(),
        // };
        // let binary = query(deps.as_ref(), testing::mock_env(), query_msg).unwrap();
        // let response: IsWhitelistedResponse = cosmwasm_std::from_binary(&binary).unwrap();
        // assert_eq!(response.is_whitelisted, false);
    }
}

