use super::{Group, User};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct Table<'c, T>
where
    T: FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'c MySqlRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

pub struct JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, MySqlRow>,
    T2: FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: (
        fn(&'c MySqlRow) -> Result<T1, sqlx::Error>,
        fn(&'c MySqlRow) -> Result<T2, sqlx::Error>,
    ),
    _marker_t1: PhantomData<&'c T1>,
    _marker_t2: PhantomData<&'c T2>,
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, MySqlRow>,
    T2: FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
        }
    }
}

pub struct Database<'c> {
    //定义一个原子引用类型，表示只印用一次
    pub groups: Arc<Table<'c, Group>>,
    pub users: Arc<Table<'c, User>>,
    pub users_to_groups: Arc<JoinTable<'c, User, Group>>,
}

impl<'a> Database<'a> {
    pub async fn new(sql_url: &String) -> Database<'a> {
        let connection = MySqlPool::connect(&sql_url).await.unwrap();
        let pool = Arc::new(connection);

        Database {
            groups: Arc::from(Table::new(pool.clone())),
            users: Arc::from(Table::new(pool.clone())),
            users_to_groups: Arc::from(JoinTable::new(pool.clone())),
        }
    }
}
