#[derive(Debug, Clone, Copy)]
pub enum SDError {

    CliError(CliError),
    RedisError(RedisError),

}

#[derive(Debug, Clone, Copy)]
pub enum CliError {

}

#[derive(Debug, Clone, Copy)]
pub enum RedisError {

}
