#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{AllCoursesResponse, CourseResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Course, Role, CONFIG, COURSES};

const CONTRACT_NAME: &str = "crates.io:cw-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateCourse { course_id, name } => {
            execute_create_course(deps, env, info, course_id, name)
        }
        ExecuteMsg::Enroll { course_id, role } => execute_enroll(deps, env, info, course_id, role),
        ExecuteMsg::Unenroll { course_id } => execute_unenroll(deps, env, info, course_id),
    }
}

fn execute_create_course(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    course_id: String,
    name: String,
) -> Result<Response, ContractError> {
    let admin = CONFIG.load(deps.storage)?.admin;
    if admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let participants: Vec<(Addr, Role)> = vec![];

    let course = Course { name, participants };

    COURSES.save(deps.storage, &course_id, &course)?;

    Ok(Response::new())
}

fn execute_enroll(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    course_id: String,
    role: Role,
) -> Result<Response, ContractError> {
    let course = COURSES.may_load(deps.storage, &course_id)?;

    match course {
        Some(mut course) => {
            if course
                .participants
                .iter()
                .any(|enrolement| enrolement.0 == info.sender)
            {
                return Err(ContractError::Enrolled {});
            }

            course.participants.push((info.sender, role));
            COURSES.save(deps.storage, &course_id, &course)?;
            Ok(Response::new())
        }
        None => Err(ContractError::CourseNotFound {}),
    }
}

fn execute_unenroll(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    course_id: String,
) -> Result<Response, ContractError> {
    let course = COURSES.may_load(deps.storage, &course_id)?;

    match course {
        Some(mut course) => {
            let participant = course
                .participants
                .iter()
                .position(|enrolement| enrolement.0 == info.sender);
            match participant {
                Some(index) => {
                    course.participants.swap_remove(index);
                    COURSES.save(deps.storage, &course_id, &course)?;
                    Ok(Response::new())
                }
                None => Err(ContractError::NotEnrolled {}),
            }
        }
        None => Err(ContractError::CourseNotFound {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllCourses {} => query_all_courses(deps, env),
        QueryMsg::Course { course_id } => query_course(deps, env, course_id),
    }
}

fn query_all_courses(deps: Deps, _env: Env) -> StdResult<Binary> {
    let courses = COURSES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|c| Ok(c?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllCoursesResponse { courses })
}

fn query_course(deps: Deps, _env: Env, course_id: String) -> StdResult<Binary> {
    let course = COURSES.may_load(deps.storage, &course_id)?;
    to_binary(&CourseResponse { course })
}

#[cfg(test)]
mod tests;
