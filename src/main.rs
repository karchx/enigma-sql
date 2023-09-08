use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;

fn main() {
    dotenv().ok();
    let mysql_port: u16 = read_vars("MYSQL_PORT")
        .parse()
        .expect("please give me correct string number");
    let mysql_user = read_vars("MYSQL_USER");
    let mysql_pass = read_vars("MYSQL_PASS");
    let mysql_host = read_vars("MYSQL_HOST");
    let mysql_db = read_vars("MYSQL_DB");

    let opts = OptsBuilder::new()
        .user(Some(&mysql_user))
        .pass(Some(&mysql_pass))
        .ip_or_hostname(Some(&mysql_host))
        .tcp_port(mysql_port)
        .db_name(Some(&mysql_db));

    let pool = Pool::new(opts).unwrap();
    let args: Vec<String> = env::args().collect();
    let where_param = &args[1];

    // println!("Searching: {}", where_param);

    let mut conn = pool.get_conn().unwrap();

    let select_formulas = conn
        .query_iter("SELECT formula FROM seguros.plantilla_ramo_cobertura_tarifa WHERE idramo = 3");

    //let filter = select_formulas.filter(|f| f.contains(where_param));
    println!("{:?}", select_formulas);

    //    .for_each(|row| {
    //        let r: String = from_row(row.unwrap());
    //        println!("{}", r);
    //    });
}

fn read_vars(var: &str) -> String {
    let error_expect = format!("{} must be set.", var);

    std::env::var(var).expect(&error_expect)
}
