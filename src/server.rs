use crate::context::{Context, Data};
use crate::gtfs_rt::{gtfs_rt, gtfs_rt_json};
use crate::stop_monitoring::stop_monitoring;
use crate::stoppoints_discovery::stoppoints_discovery;
use actix_web::{middleware, App};
use std::sync::{Arc, Mutex};

pub fn make_context(gtfs: &str, url: &str) -> Context {
    let gtfs_rt_data = Arc::new(Mutex::new(None));
    let gtfs = if gtfs.starts_with("http") {
        gtfs_structures::Gtfs::from_url(gtfs).unwrap()
    } else {
        gtfs_structures::Gtfs::from_zip(gtfs).unwrap()
    };
    let data = Data::new(gtfs);
    let data = Arc::new(Mutex::new(data));
    Context {
        gtfs_rt: gtfs_rt_data.clone(),
        data: data.clone(),
        gtfs_rt_provider_url: url.to_owned(),
    }
}

pub fn create_server(context: Context) -> App<Context> {
    App::with_state(context)
        .middleware(middleware::Logger::default())
        .resource("/gtfs_rt", |r| r.f(gtfs_rt))
        .resource("/gtfs_rt.json", |r| r.f(gtfs_rt_json))
        .resource("/stoppoints_discovery.json", |r| {
            r.with(stoppoints_discovery)
        })
        .resource("/stop_monitoring.json", |r| r.with(stop_monitoring))
}
