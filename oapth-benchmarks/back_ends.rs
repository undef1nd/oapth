#![allow(missing_docs)]

use criterion::{
  black_box, criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup,
  BenchmarkId, Criterion,
};
use oapth::{
  Commands, Config, DieselMysql, DieselPostgres, DieselSqlite, MysqlAsync, Rusqlite, SqlxMssql,
  SqlxMysql, SqlxPostgres, SqlxSqlite, Tiberius, TokioPostgres,
};
use std::path::Path;
use tokio::runtime::Runtime;

macro_rules! add_benchmark_group {
  (
    $criterion:expr,
    $f:ident,
    $diesel_mysql:expr,
    $diesel_postgres:expr,
    $diesel_sqlite:expr,
    $mysql_async:expr,
    $rusqlite:expr,
    $sqlx_mssql:expr,
    $sqlx_mysql:expr,
    $sqlx_postgres:expr,
    $sqlx_sqlite:expr,
    $tiberius:expr,
    $tokio_postgres:expr
  ) => {
    fn $f<M>(group: &mut BenchmarkGroup<'_, M>, size: usize)
    where
      M: Measurement,
    {
      let mssql_config = Config::with_url_from_var("MSSQL").unwrap();
      let mysql_config = Config::with_url_from_var("MYSQL").unwrap();
      let postgres_config = Config::with_url_from_var("POSTGRES").unwrap();
      let sqlite_config = Config::with_url_from_var("SQLITE").unwrap();

      group.bench_with_input(BenchmarkId::new("Diesel - MySQL", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(DieselMysql::new(&mysql_config).await.unwrap());
            $diesel_mysql(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("Diesel - PostgreSQL", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(DieselPostgres::new(&postgres_config).await.unwrap());
            $diesel_postgres(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("Diesel - SQLite", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(DieselSqlite::new(&sqlite_config).await.unwrap());
            $diesel_sqlite(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("mysql_async", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(MysqlAsync::new(&mysql_config).await.unwrap());
            $mysql_async(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("rusqlite", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(Rusqlite::new(&sqlite_config).await.unwrap());
            $rusqlite(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("SQLX - MS-SQL", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(SqlxMssql::new(&mssql_config).await.unwrap());
            $sqlx_mssql(c).await;
          })
        })
      });

      group.bench_with_input(BenchmarkId::new("SQLX - MySql", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(SqlxMysql::new(&mysql_config).await.unwrap());
            $sqlx_mysql(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("SQLX - PostgreSQL", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(SqlxPostgres::new(&postgres_config).await.unwrap());
            $sqlx_postgres(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("SQLX - SQLite", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(SqlxSqlite::new(&sqlite_config).await.unwrap());
            $sqlx_sqlite(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("tiberius", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            use tokio_util::compat::Tokio02AsyncWriteCompatExt;
            let tcp =
              tokio::net::TcpStream::connect(mssql_config.full_host().unwrap()).await.unwrap();
            let c = Commands::new(Tiberius::new(&mssql_config, tcp.compat_write()).await.unwrap());
            $tiberius(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("tokio_postgres", size), &size, |b, _| {
        b.iter(|| {
          let mut rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::new(TokioPostgres::new(&postgres_config).await.unwrap());
            $tokio_postgres(c).await;
          });
        })
      });
    }

    let mut group = $criterion.benchmark_group(stringify!($f));
    $f(&mut group, 32);
    group.finish();
  };
}

fn criterion_benchmark(c: &mut Criterion) {
  add_benchmark_group!(
    c,
    migrate,
    |mut c: Commands<DieselMysql>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<DieselPostgres>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<DieselSqlite>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<MysqlAsync>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<Rusqlite>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<SqlxMssql>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<SqlxMysql>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<SqlxPostgres>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<SqlxSqlite>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<Tiberius<_>>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    },
    |mut c: Commands<TokioPostgres>| async move {
      let cfg = Path::new("../oapth-test-utils/oapth.cfg");
      c.migrate_from_cfg(cfg, black_box(128)).await.unwrap();
    }
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
