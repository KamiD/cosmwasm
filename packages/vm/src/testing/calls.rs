//! This file has some helpers for integration tests.
//! They should be imported via full path to ensure there is no confusion
//! use cosmwasm_vm::testing::X
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt;

use cosmwasm_std::{
    to_vec, Env, HandleResult, InitResult, MigrateResult, QueryResponse, StdResult,
};

use crate::calls::{call_handle, call_init, call_migrate, call_query};
use crate::instance::Instance;
use crate::{Api, Querier, Storage};

// init mimicks the call signature of the smart contracts.
// thus it moves env and msg rather than take them as reference.
// this is inefficient here, but only used in test code
pub fn init<S, A, Q, M, U>(instance: &mut Instance<S, A, Q>, env: Env, msg: M) -> InitResult<U>
where
    S: Storage + 'static,
    A: Api + 'static,
    Q: Querier + 'static,
    M: Serialize + JsonSchema,
    U: DeserializeOwned + Clone + PartialEq + JsonSchema + fmt::Debug,
{
    let serialized_msg = to_vec(&msg)?;
    call_init(instance, &env, &serialized_msg).expect("VM error")
}

// handle mimicks the call signature of the smart contracts.
// thus it moves env and msg rather than take them as reference.
// this is inefficient here, but only used in test code
pub fn handle<S, A, Q, M, U>(instance: &mut Instance<S, A, Q>, env: Env, msg: M) -> HandleResult<U>
where
    S: Storage + 'static,
    A: Api + 'static,
    Q: Querier + 'static,
    M: Serialize + JsonSchema,
    U: DeserializeOwned + Clone + PartialEq + JsonSchema + fmt::Debug,
{
    let serialized_msg = to_vec(&msg)?;
    call_handle(instance, &env, &serialized_msg).expect("VM error")
}

// migrate mimicks the call signature of the smart contracts.
// thus it moves env and msg rather than take them as reference.
// this is inefficient here, but only used in test code
pub fn migrate<S, A, Q, M, U>(
    instance: &mut Instance<S, A, Q>,
    env: Env,
    msg: M,
) -> MigrateResult<U>
where
    S: Storage + 'static,
    A: Api + 'static,
    Q: Querier + 'static,
    M: Serialize + JsonSchema,
    U: DeserializeOwned + Clone + PartialEq + JsonSchema + fmt::Debug,
{
    let serialized_msg = to_vec(&msg)?;
    call_migrate(instance, &env, &serialized_msg).expect("VM error")
}

// query mimicks the call signature of the smart contracts.
// thus it moves env and msg rather than take them as reference.
// this is inefficient here, but only used in test code
pub fn query<S, A, Q, M>(instance: &mut Instance<S, A, Q>, msg: M) -> StdResult<QueryResponse>
where
    S: Storage + 'static,
    A: Api + 'static,
    Q: Querier + 'static,
    M: Serialize + JsonSchema,
{
    let serialized_msg = to_vec(&msg)?;
    call_query(instance, &serialized_msg).expect("VM error")
}
