use rocket_db_pools::{diesel::{prelude::RunQueryDsl, QueryResult}, Connection};

use crate::{database::Db, model::rig::{Rig, RigComponent}, schema::{rigs, rig_components}};