#[macro_export]
macro_rules! hex {
    ($q:expr, $r:expr) => {
        Hex { q: $q, r: $r }
    };
}

#[macro_export]
macro_rules! opt {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return None,
        }
    };
}

#[macro_export]
macro_rules! early_return {
    ($cond:expr) => {
        if !$cond {
            return;
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($opt:expr, $msg:expr) => {
        match $opt {
            Some(val) => val,
            None => {
                println!("{}", $msg);
                return;
            }
        }
    };
}
