use std::env;

pub struct Config {
    pub prod_name: Vec<String>,
    pub test_name: Vec<String>,
    pub dev_name: Vec<String>,
}

pub fn init_config_from_env() -> Config {
    let mut prod_name_list = Vec::<String>::new();
    let mut test_name_list = Vec::<String>::new();
    let mut dev_name_list = Vec::<String>::new();

    get_env(&Env::Dev, &mut dev_name_list);
    get_env(&Env::Test, &mut test_name_list);
    get_env(&Env::Prod, &mut prod_name_list);

    Config {
        prod_name: prod_name_list,
        test_name: test_name_list,
        dev_name: dev_name_list,
    }
}

enum Env {
    Prod,
    Test,
    Dev,
}

fn get_env(env: &Env, env_list: &mut Vec<String>) {
    for i in 1..51 {
        let env_string = get_env_string(env, i);
        let env_var = env::var(env_string).unwrap_or_default();

        if env_var.is_empty() {
            break;
        }

        env_list.push(env_var);
    }
}

fn get_env_string(env: &Env, num: u8) -> String {
    let env_prefix = "KCWRAP_";

    match env {
        Env::Dev => format!("{env_prefix}DEV{num}"),
        Env::Test => format!("{env_prefix}TEST{num}"),
        Env::Prod => format!("{env_prefix}PROD{num}"),
    }
}
