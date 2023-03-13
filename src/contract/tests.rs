use crate::contract::{execute, instantiate, query};
use crate::msg::{AllCoursesResponse, CourseResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::Role;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, from_binary};

pub const ADDR1: &str = "addr1";
pub const ADDR2: &str = "addr2";

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR1)]
    )
}

#[test]
fn test_instantiate_with_admin() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: Some(ADDR2.to_string()),
    };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR2)]
    )
}

#[test]
fn test_execute_create_course_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id".to_string(),
        name: "Salsa Beginners 1".to_string(),
    };

    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}

#[test]
fn test_execute_create_course_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: Some(ADDR2.to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id".to_string(),
        name: "Salsa Beginners 1".to_string(),
    };

    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}

#[test]
fn test_execute_enroll_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id".to_string(),
        name: "Salsa Beginners 1".to_string(),
    };

    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::Enroll {
        course_id: "some_id".to_string(),
        role: Role::Leader {},
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}

#[test]
fn test_execute_enroll_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id".to_string(),
        name: "Salsa Beginners 1".to_string(),
    };

    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::Enroll {
        course_id: "wrong_id".to_string(),
        role: Role::Leader {},
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

    let msg = ExecuteMsg::Enroll {
        course_id: "some_id".to_string(),
        role: Role::Leader {},
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}

#[test]
fn test_execute_unenroll_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id".to_string(),
        name: "Salsa Beginners 1".to_string(),
    };

    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::Enroll {
        course_id: "some_id".to_string(),
        role: Role::Leader {},
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::Unenroll {
        course_id: "some_id".to_string(),
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}

#[test]
fn test_execute_unenroll_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id".to_string(),
        name: "Salsa Beginners 1".to_string(),
    };

    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::Enroll {
        course_id: "some_id".to_string(),
        role: Role::Leader {},
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::Unenroll {
        course_id: "wrong_id".to_string(),
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

    let msg = ExecuteMsg::Unenroll {
        course_id: "some_id".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}

#[test]
fn test_query_all_courses() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id_1".to_string(),
        name: "Salsa Beginners 1".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id_2".to_string(),
        name: "Bachata Beginners 1".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = QueryMsg::AllCourses {};
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: AllCoursesResponse = from_binary(&bin).unwrap();
    assert_eq!(res.courses.len(), 2);
}

#[test]
fn test_query_course() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreateCourse {
        course_id: "some_id_1".to_string(),
        name: "Salsa Beginners 1".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = QueryMsg::Course {
        course_id: "some_id_1".to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: CourseResponse = from_binary(&bin).unwrap();
    assert!(res.course.is_some());

    let msg = QueryMsg::Course {
        course_id: "some_id_not_exist".to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: CourseResponse = from_binary(&bin).unwrap();
    assert!(res.course.is_none());
}
