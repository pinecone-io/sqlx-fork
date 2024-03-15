use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;
use rand::{thread_rng, Rng};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn repro_issue() {
    let mut pool_options = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(1));
	/*.before_acquire(move |_conn, metadata| {
            Box::pin(async move {
		Ok(thread_rng().gen_range(1..10) != 1)
	    })
	});*/
    /*.after_connect(move |conn, _meta| {
	    Box::pin(async move {
		tokio::time::sleep(Duration::from_millis(500)).await;
		Ok(())
	    })
	});*/
    let pool = pool_options.connect("postgres://postgres:postgres@localhost/").await.unwrap();

    for _ in 0..1000 {
	let pool = pool.clone();
	tokio::spawn(async move {
	    loop {
		let res = sqlx::query("SELECT pg_sleep(10)")
		    .fetch_all(&pool)
		    .await;
		println!("{} max {} size {} idle {}",
			 res.is_err(),
			 pool.options().get_max_connections(),
			 pool.size(),
			 pool.num_idle(),
		);
		let sleep = thread_rng().gen_range(1..100);
		tokio::time::sleep(tokio::time::Duration::from_millis(sleep)).await;
	    }
	});
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;
}
