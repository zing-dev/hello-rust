use rusqlite::{Connection, params, ToSql};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: u8,
    data: Option<Vec<u8>>,
}

const PATH: &str = "crates/rusqlite/quickstart.db";

struct SQL {
    conn: Connection,
}

impl SQL {
    /// 创建用户表
    fn create(&self) -> rusqlite::Result<usize> {
        return self.conn.execute(
            "CREATE TABLE IF NOT EXISTS  person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER NOT NULL,
                  data            BLOB)",
            params![],
        );
    }

    /// 初始化所有用户
    fn init(&self) -> rusqlite::Result<usize> {
        let sql = "INSERT INTO person (name, age, data) VALUES (?1, ?2, ?3)";
        for i in 1..=10 {
            let p = &Person { id: 0, name: format!("name-{}", i), age: 10 + i, data: Some(vec![1, 2, 3]) };
            self.conn.execute(sql, params![p.name,p.age, p.data])?;
        }
        Ok(10)
    }

    /// 添加一条新的数据
    fn insert(&self, p: &Person) -> rusqlite::Result<usize> {
        let sql = "INSERT INTO person (name, age, data) VALUES (?1, ?2, ?3)";
        self.conn.execute(sql, &[&p.name as &dyn ToSql, &p.age as &dyn ToSql, &p.data as &dyn ToSql])?;
        Ok(1)
    }


    /// 查询所有用户
    fn select(&self) -> rusqlite::Result<Vec<Person>> {
        let mut stmt = self.conn.prepare("SELECT id, name, age, data FROM person")?;
        let mut persons = vec![];
        let result = stmt.query_map(params![], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
                data: row.get(3)?,
            })
        })?;
        for p in result {
            persons.push(match p {
                Ok(p) => p,
                Err(e) => return Err(e),
            })
        }
        Ok(persons)
    }
}

impl Default for SQL {
    fn default() -> Self {
        SQL { conn: Connection::open(PATH).unwrap() }
    }
}

fn main() -> rusqlite::Result<()> {
    let sql = SQL::default();
    sql.create()?;
    sql.init()?;
    sql.insert(&Person { id: 0, name: String::from("zing"), age: 27, data: Some(vec![0, 0, 0, 0, 0, 0, 0]) })?;
    match sql.select() {
        Ok(persons) => {
            for p in persons {
                println!("id:{},name:{},age:{}", p.id, p.name, p.age);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}

