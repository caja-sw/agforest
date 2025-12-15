use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
