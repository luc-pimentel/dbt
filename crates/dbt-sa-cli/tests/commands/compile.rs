use dbt_common::{FsResult, current_function_name};
use dbt_test_utils::task::{ProjectEnv, TaskSeq};

use crate::common::TaskSeqExt;

#[dbt_runtime::test]
async fn compile_inline_quiet_preserves_sql() -> FsResult<()> {
    let env = ProjectEnv::immutable_sa("tests/data/hello_world")?;

    TaskSeq::new(current_function_name!())
        .fs_sa("compile --inline select(1) -q")
        .execute_in_with_env(&env, &[("target_env_var", "duckdb_mem")])
        .await?;

    Ok(())
}

#[dbt_runtime::test]
async fn compile_inline_show_none_suppresses_sql() -> FsResult<()> {
    let env = ProjectEnv::immutable_sa("tests/data/hello_world")?;

    TaskSeq::new(current_function_name!())
        .fs_sa("compile --inline select(1) --show none")
        .execute_in_with_env(&env, &[("target_env_var", "duckdb_mem")])
        .await?;

    Ok(())
}

#[dbt_runtime::test]
async fn compile_defer_with_missing_state_manifest_errors() -> FsResult<()> {
    let env = ProjectEnv::immutable_sa("tests/data/hello_world")?;

    TaskSeq::new(current_function_name!())
        .fs_sa("compile --select hello_world --defer --state does-not-exist")
        .execute_in_with_env(&env, &[("target_env_var", "duckdb_mem")])
        .await?;

    Ok(())
}

#[dbt_runtime::test]
async fn compile_missing_state_manifest_without_defer_is_silent() -> FsResult<()> {
    let env = ProjectEnv::immutable_sa("tests/data/hello_world")?;

    TaskSeq::new(current_function_name!())
        .fs_sa("compile --select hello_world --state does-not-exist")
        .execute_in_with_env(&env, &[("target_env_var", "duckdb_mem")])
        .await?;

    Ok(())
}
